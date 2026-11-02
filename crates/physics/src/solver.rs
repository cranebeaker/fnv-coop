//! Havok's constraint solver for an island with constraints (B1 PR 7,
//! docs/PHYSICS.md "The contact solver"): `hkpConstraintSolverSetup::solve`
//! (Xbox PDB, `00d88b50`) with the velocity accumulators
//! (`hkRigidMotionUtilApplyForcesAndBuildAccumulators` `00d29830`), the
//! contacts' callbacks for new points (`hkSimpleContactConstraintData_
//! fireCallbacks` `00d92900`), the contact Jacobians
//! (`hkSimpleContactConstraintDataBuildJacobian` `00d72190`), the solver
//! (`hkSolveConstraints` `00d8d030`: substeps, micro steps, the schemas,
//! the velocities integrated between substeps), the export
//! (`hkSolverExport` `00def570`) and the accumulators applied back
//! (`hkRigidMotionUtilApplyAccumulators` `00d29bf0`). Translated from
//! FalloutNV.exe 1.4.0.525; names are the Xbox prototype's (ADR-0002),
//! addresses the PC's (ADR-0003).
//!
//! Everything here is in Havok units (the callers divide game units by
//! [`crate::HAVOK_UNIT`]); positions are given relative to an origin of the
//! caller's choosing, so far from the world's origin nothing is lost.

use crate::havok::{SolverInfo, UFLOAT8};
use crate::manifold::{Manifold, INFO_RADIUS_DIRTY, POINT_DISABLED, POINT_NEW, POINT_PAIRED};
use crate::vec::*;
use crate::Vec3;

/// The single-precision epsilon the solver adds to effective masses and
/// uses as the least allowed penetration (`1.1920929e-07`).
const EPS: f32 = 1.192_092_9e-7;

/// `hkpVelocityAccumulator::hkpAccumulatorType` (`+0x0`, a byte).
pub mod acc_kind {
    /// A moving body.
    pub const DYNAMIC: u8 = 0;
    /// A keyframed body, and the world's fixed body.
    pub const KEYFRAMED: u8 = 1;
    /// A character's.
    pub const CHARACTER: u8 = 2;
}

/// A body as the solver sees it (`hkpVelocityAccumulator`, 0x80 bytes;
/// Havok units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Accumulator {
    /// `m_type` (`+0x0`, [`acc_kind`]).
    pub kind: u8,
    /// `m_deactivationClass` (`+0x4`).
    pub deactivation_class: usize,
    /// `m_gravityFactor` (`+0x8`).
    pub gravity_factor: f32,
    /// `m_linearVel` (`+0x10`, world).
    pub linear: Vec3,
    /// `m_angularVel` (`+0x20`): in the body's own axes.
    pub angular: Vec3,
    /// `m_invMasses` (`+0x30`): the inverse inertia's diagonal in the
    /// body's axes, then the inverse mass.
    pub inv_inertia: Vec3,
    pub inv_mass: f32,
    /// `m_scratch0` (`+0x40`) before the solve: the centre of mass
    /// (relative to the caller's origin).
    pub center: Vec3,
    /// `m_scratch123` (`+0x50`) before the solve: the world to the body's
    /// axes, as columns (`local = c0 wx + c1 wy + c2 wz`; the rotation
    /// transposed, `00cd4ba0`).
    pub to_local: [Vec3; 3],
    /// `+0x40` and `+0x50` during the solve: the integrated velocity sums
    /// (after it, the velocities the positions are moved by).
    pub sum_linear: Vec3,
    pub sum_angular: Vec3,
}

impl Accumulator {
    /// The world's fixed body (the solver's first accumulator, `00d88b50`):
    /// keyframed, nothing moves it.
    pub fn fixed() -> Accumulator {
        Accumulator {
            kind: acc_kind::KEYFRAMED,
            deactivation_class: 0,
            gravity_factor: 0.0,
            linear: [0.0; 3],
            angular: [0.0; 3],
            inv_inertia: [0.0; 3],
            inv_mass: 0.0,
            center: [0.0; 3],
            to_local: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            sum_linear: [0.0; 3],
            sum_angular: [0.0; 3],
        }
    }

    /// A keyframed body moving at `velocity` (world, Havok units a second)
    /// without turning (`00d29830`, motion type 4: no inverse masses, the
    /// motion's velocities, identity axes).
    pub fn keyframed(center: Vec3, velocity: Vec3) -> Accumulator {
        Accumulator {
            center,
            linear: velocity,
            ..Accumulator::fixed()
        }
    }

    /// A dynamic body's (`00d29830` for the box motion types: the motion's
    /// damped velocities, its inverse inertia and mass, centre, and the
    /// rotation transposed; the angular velocity turned into its axes).
    /// `rotation` is the body's (columns: its axes in the world).
    #[allow(clippy::too_many_arguments)]
    pub fn dynamic(
        center: Vec3,
        rotation: [Vec3; 3],
        linear: Vec3,
        angular_world: Vec3,
        inv_inertia: Vec3,
        inv_mass: f32,
        deactivation_class: usize,
        gravity_factor: f32,
    ) -> Accumulator {
        let to_local = transpose_cols(rotation);
        Accumulator {
            kind: acc_kind::DYNAMIC,
            deactivation_class,
            gravity_factor,
            linear,
            angular: apply_cols(&to_local, angular_world),
            inv_inertia,
            inv_mass,
            center,
            to_local,
            sum_linear: [0.0; 3],
            sum_angular: [0.0; 3],
        }
    }
}

/// `c0 v.x + c1 v.y + c2 v.z`.
pub fn apply_cols(c: &[Vec3; 3], v: Vec3) -> Vec3 {
    add(add(scale(c[0], v[0]), scale(c[1], v[1])), scale(c[2], v[2]))
}

/// Columns of a matrix's transpose.
pub fn transpose_cols(c: [Vec3; 3]) -> [Vec3; 3] {
    [
        [c[0][0], c[1][0], c[2][0]],
        [c[0][1], c[1][1], c[2][1]],
        [c[0][2], c[1][2], c[2][2]],
    ]
}

/// The constraint query's step values (`hkpConstraintQueryIn`'s step
/// part, built by `00cfa6e0` from the solver info and the step).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueryIn {
    /// `m_substepDeltaTime`: the solver info's `m_deltaTime`.
    pub substep_dt: f32,
    pub micro_step_dt: f32,
    pub substep_inv_dt: f32,
    /// `m_frameDeltaTime`: the step's.
    pub frame_dt: f32,
    pub frame_inv_dt: f32,
    pub inv_num_steps: f32,
    pub inv_num_steps_times_micro_steps: f32,
    /// `m_rhsFactor`: tau ÷ damping × the substep's 1/dt.
    pub rhs_factor: f32,
    /// `m_virtMassFactor`: the damping.
    pub virt_mass_factor: f32,
    /// `m_frictionRhsFactor`: friction tau ÷ damping × the substep's 1/dt.
    pub friction_rhs_factor: f32,
}

impl QueryIn {
    // Translated from 00cfa6e0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn new(s: &SolverInfo, step: &Step) -> QueryIn {
        QueryIn {
            substep_dt: step.substep_dt,
            micro_step_dt: s.inv_num_micro_steps * step.substep_dt,
            substep_inv_dt: step.substep_inv_dt,
            frame_dt: step.dt,
            frame_inv_dt: if step.dt == 0.0 { 0.0 } else { 1.0 / step.dt },
            inv_num_steps: s.inv_num_steps,
            inv_num_steps_times_micro_steps: s.inv_num_steps * s.inv_num_micro_steps,
            rhs_factor: s.tau_div_damp * step.substep_inv_dt,
            virt_mass_factor: s.damping,
            friction_rhs_factor: s.friction_tau_div_damp * step.substep_inv_dt,
        }
    }
}

/// One step's times and gravity as `hkpSimulation::integrateInternal`
/// (`00cf8da0`) sets them in the solver info: the substep is the step ÷
/// the substeps (`+0x10c`), its inverse the substeps ÷ the step (`+0x110`),
/// gravity × the substep (`m_globalAccelerationPerSubStep`, `+0x10`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    pub dt: f32,
    pub substep_dt: f32,
    pub substep_inv_dt: f32,
    /// Havok units a second, per substep.
    pub gravity_per_substep: Vec3,
}

impl Step {
    // Translated from 00cf8da0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn new(s: &SolverInfo, dt: f32, gravity: Vec3) -> Step {
        let substep_dt = s.inv_num_steps * dt;
        Step {
            dt,
            substep_dt,
            substep_inv_dt: s.num_steps as f32 * if dt == 0.0 { 0.0 } else { 1.0 / dt },
            gravity_per_substep: scale(gravity, substep_dt),
        }
    }
}

/// A 1-D Jacobian on a linear and two angular parts (Havok's 0x30-byte
/// "1 linear, 2 angular" Jacobian): the linear part (`+0x0`), the right-hand
/// side (`+0xc`), the first body's angular part in its axes (`+0x10`), the
/// inverse of the effective mass × the virtual mass factor (`+0x1c`), the
/// second body's angular part (`+0x20`), and `+0x2c` (the effective mass,
/// or after the pair's inversion, its diagonal).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Jacobian {
    pub linear: Vec3,
    pub rhs: f32,
    pub angular_a: Vec3,
    pub inv_jac_diag: f32,
    pub angular_b: Vec3,
    pub extra: f32,
}

/// An angular-only Jacobian (the 2-D friction's third row).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AngularJacobian {
    pub angular_a: Vec3,
    pub inv_jac_diag: f32,
    pub angular_b: Vec3,
    pub rhs: f32,
}

/// The schemas the contact constraint builds (`hkpJacobianSchema` types
/// by the solver's switch in `00d8d030`; the types are numbers there).
#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    /// Type 1: the next schemas act between accumulators `a` and `b`, and
    /// export into constraint `constraint`'s results.
    Header {
        a: usize,
        b: usize,
        constraint: usize,
    },
    /// Type 0x10: one contact point, its impulse kept at or above 0;
    /// `point` is its index in the constraint.
    Contact { jac: Jacobian, point: usize },
    /// Type 0x14: two contact points solved together (the 2 × 2 inverse:
    /// the first's `extra` and the second's, and `off`).
    ContactPair {
        first: Jacobian,
        second: Jacobian,
        off: f32,
        points: [usize; 2],
    },
    /// Type 0x16: 2-D friction (two Jacobians, their 2 × 2 inverse in the
    /// `extra`s and `off`), at most `max_impulse` a micro step.
    Friction2d {
        j0: Jacobian,
        j1: Jacobian,
        off: f32,
        max_impulse: f32,
    },
    /// Type 0x17: 2-D friction and the friction turning about the normal
    /// (`radius` the contact radius).
    Friction3d {
        j0: Jacobian,
        j1: Jacobian,
        off: f32,
        angular: AngularJacobian,
        max_impulse: f32,
        radius: f32,
    },
    /// Type 5: one 1-D Jacobian, its impulse unbounded (a ball and
    /// socket's rows, `crate::constraint`).
    Linear { jac: Jacobian },
    /// Type 0xc: one angular-only row (the 2-D angular constraint's);
    /// `slot` is its place in the joint's runtime data.
    AngularRow { row: AngularJacobian, slot: usize },
    /// Type 0xd: an angular limit (twist, cone, plane, hinge): the row, the
    /// two bounds and the limit's tau, all as the builder left them.
    AngularLimit {
        row: AngularJacobian,
        lower: f32,
        upper: f32,
        tau: f32,
        slot: usize,
    },
    /// Type 0xe: angular friction about one axis, at most `max_impulse` a
    /// micro step.
    AngularFriction {
        row: AngularJacobian,
        max_impulse: f32,
        slot: usize,
    },
}

impl Schema {
    /// How many results it keeps (the table at `011b9434`).
    pub fn results(&self) -> usize {
        match self {
            Schema::Header { .. } => 0,
            Schema::Contact { .. } => 1,
            Schema::ContactPair { .. } => 2,
            Schema::Friction2d { .. } => 3,
            Schema::Friction3d { .. } => 4,
            // The table at `011b9434`: types 5, 0xc and 0xd keep one, 0xe two.
            Schema::Linear { .. } | Schema::AngularRow { .. } | Schema::AngularLimit { .. } => 1,
            Schema::AngularFriction { .. } => 2,
        }
    }
}

/// The most penetration a frame recovers: the step's time × 1 Havok unit a
/// second (`011b6924`), or 5% (`011b6928`) of what is allowed.
const RECOVERY_SPEED: f32 = 1.0;
const RECOVERY_SHARE: f32 = 0.05;
/// Restitution above this, with a point approaching faster than the
/// contact resting velocity, takes Havok's immediate response instead
/// (`00d92900`; never with Bethesda's resting velocity of FLT_MAX).
const RESTITUTION_RESPONSE_MIN: f32 = 0.3;

/// The friction axes the Jacobian builder picks from (`01268370`: the unit
/// x, y and z, set by `00fbdde0`).
const AXES: [Vec3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// `n / |n|` as Havok's `rsqrtss` and one Newton step give it (here exact),
/// zero for zero.
fn normalized(v: Vec3) -> Vec3 {
    let l = dot(v, v);
    if l == 0.0 {
        return [0.0; 3];
    }
    scale(v, 1.0 / l.sqrt())
}

/// A 1-D Jacobian along `dir` at `r_a`, `r_b` (from each centre) and its
/// effective mass (inverse masses and the angular parts squared over the
/// inverse inertias, + `EPS`).
fn jacobian(a: &Accumulator, b: &Accumulator, dir: Vec3, r_a: Vec3, r_b: Vec3) -> (Jacobian, f32) {
    let ang_a = apply_cols(&a.to_local, cross(r_a, dir));
    let ang_b = apply_cols(&b.to_local, cross(dir, r_b));
    let mut eff = a.inv_mass + b.inv_mass + EPS;
    for k in 0..3 {
        eff += ang_a[k] * ang_a[k] * a.inv_inertia[k] + ang_b[k] * ang_b[k] * b.inv_inertia[k];
    }
    (
        Jacobian {
            linear: dir,
            rhs: 0.0,
            angular_a: ang_a,
            inv_jac_diag: 0.0,
            angular_b: ang_b,
            extra: eff,
        },
        eff,
    )
}

/// `J₁ M⁻¹ J₂ᵀ` for two Jacobians between the same two bodies.
fn coupling(a: &Accumulator, b: &Accumulator, x: &Jacobian, y: &Jacobian) -> f32 {
    let mut s = (a.inv_mass + b.inv_mass) * dot(x.linear, y.linear);
    for k in 0..3 {
        s += x.angular_a[k] * y.angular_a[k] * a.inv_inertia[k]
            + x.angular_b[k] * y.angular_b[k] * b.inv_inertia[k];
    }
    s
}

/// What a new point's projected velocity is at the time of the solver's
/// callbacks (`00df2510`): the first body's velocity at the point less the
/// second's, along the normal (the motions' damped velocities, the
/// accumulators' centres).
pub struct BodyVelocity {
    pub linear: Vec3,
    pub angular: Vec3,
    pub center: Vec3,
}

fn point_velocity(b: &BodyVelocity, p: Vec3) -> Vec3 {
    add(b.linear, cross(b.angular, sub(p, b.center)))
}

/// The contact constraint's callbacks before the Jacobians are built
/// (`hkSimpleContactConstraintData_fireCallbacks` `00d92900`), for each new
/// point (Havok's `allowToSkipConfirmedCallbacks` is off in the game's
/// world, so every new point comes here): its projected velocity v
/// (`00df2510`); with restitution r (the byte ÷ 128) at most 0.3, or v not
/// under −(contact resting velocity) (always: Bethesda's is FLT_MAX), the
/// impulse's first guess (r + 1) ÷ (the inverse masses + 1e-10) × −0.2 × v,
/// the solver data r × v × the substep × −1.3, and the allowed penetration
/// that + the distance when r > 0, else 0; the point is no longer new.
/// (Over 0.3 and faster, `hkpSimpleCollisionResponse::solveSingleContact`
/// `00d9ee50` would answer at once: never reached here.)
// Translated from 00d92900 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fire_callbacks(
    m: &mut Manifold,
    q: &QueryIn,
    inv_masses: (f32, f32),
    a: &BodyVelocity,
    b: &BodyVelocity,
    contact_resting_velocity: f32,
) {
    for p in &mut m.points {
        if p.props.flags & POINT_NEW == 0 {
            continue;
        }
        let v = dot(
            sub(point_velocity(a, p.position), point_velocity(b, p.position)),
            p.normal,
        );
        if p.props.flags & POINT_DISABLED != 0 {
            // A disabled point is put far away (`0x7effffee`).
            p.distance = f32::from_bits(0x7eff_ffee);
            p.props.impulse_applied = 0.0;
            p.props.internal_solver_data = 0.0;
            p.props.internal_data_a = 0.0;
            p.props.flags &= !POINT_NEW;
            continue;
        }
        let r = f32::from(p.props.restitution) * 0.007_812_5;
        if -contact_resting_velocity <= v || r <= RESTITUTION_RESPONSE_MIN {
            p.props.impulse_applied =
                (r + 1.0) * (1.0 / (inv_masses.1 + inv_masses.0 + 1e-10)) * -0.2 * v;
            let isd = v * r * q.substep_dt * -1.3;
            p.props.internal_solver_data = isd;
            p.props.internal_data_a = if r <= 0.0 { 0.0 } else { isd + p.distance };
        } else {
            p.props.impulse_applied = 0.0;
            p.props.internal_data_a = 0.0;
        }
        p.props.flags &= !POINT_NEW;
    }
}

/// The contact constraint's Jacobians (`hkSimpleContactConstraintData
/// BuildJacobian` `00d72190`): a header, then for each point a 1-D
/// Jacobian along its normal at its position; its right-hand side from
/// its distance with the allowed penetration recovered at most the step ×
/// 1 Havok unit a second or 5% of it a step (the new allowance kept in
/// `internal_data_a`, never above −ε), and a penetration the solver didn't
/// predict (the distance under what `internal_solver_data` said by more
/// than twice the recovery and the step) added to the allowance; × the rhs
/// factor. A point flagged paired is solved with the one before as a
/// 2 × 2 block (the coupling × 0.99). Then, when the last solve's
/// impulses × the mean friction are over 0, 2-D friction at the points'
/// mean position along two axes across the mean normal (picked from x, y,
/// z by the info's index, changed to the normal's least component when
/// within 0.1² of it, which clears the friction's drifts), its right-hand
/// sides from the drifts the last export left, at most that total a micro
/// step; with two points or more, also the friction turning about the
/// normal at the mean distance of the points from their middle (worked out
/// again only when the info asks).
// Translated from 00d72190 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn build_contact_jacobians(
    m: &mut Manifold,
    constraint: usize,
    q: &QueryIn,
    (ia, ib): (usize, usize),
    a: &Accumulator,
    b: &Accumulator,
    out: &mut Vec<Schema>,
) {
    let n = m.points.len();
    if n == 0 {
        return;
    }
    let inv_n = 1.0 / n as f32;
    out.push(Schema::Header {
        a: ia,
        b: ib,
        constraint,
    });
    let mut sum_friction = 0.0f32;
    let mut sum_impulse = 0.0f32;
    let mut sum_position = [0.0f32; 3];
    let mut sum_normal = [0.0f32; 3];
    for k in 0..n {
        let p = m.points[k];
        let (mut jac, eff) = jacobian(
            a,
            b,
            p.normal,
            sub(p.position, a.center),
            sub(p.position, b.center),
        );
        jac.inv_jac_diag = q.virt_mass_factor / eff;
        sum_friction += UFLOAT8[p.props.friction as usize];
        sum_impulse += p.props.impulse_applied;
        // The right-hand side and the allowed penetration.
        let da = p.props.internal_data_a;
        let unpredicted = -p.props.internal_solver_data - (p.distance - da);
        let recovery = q.frame_dt * RECOVERY_SPEED;
        let step = recovery.min(-RECOVERY_SHARE * da);
        let mut allowed = step + da;
        let gap = (p.distance - da) - step;
        let mut seen = gap;
        if step + step + recovery < unpredicted {
            allowed -= unpredicted;
            seen = unpredicted + gap;
        }
        if -EPS <= allowed {
            allowed = -EPS;
        }
        m.points[k].props.internal_data_a = allowed;
        jac.rhs = -seen * q.rhs_factor;
        sum_position = add(sum_position, p.position);
        sum_normal = add(sum_normal, p.normal);
        let paired =
            p.props.flags & POINT_PAIRED != 0 && matches!(out.last(), Some(Schema::Contact { .. }));
        if paired {
            let Some(Schema::Contact {
                jac: first,
                point: first_point,
            }) = out.pop()
            else {
                unreachable!()
            };
            let off = coupling(a, b, &first, &jac) * 0.99;
            let (k11, k22) = (first.extra, jac.extra);
            let det = q.virt_mass_factor / (k11 * k22 - off * off);
            let mut first = first;
            first.extra = det * k22;
            jac.extra = k11 * det;
            out.push(Schema::ContactPair {
                first,
                second: jac,
                off: -off * det,
                points: [first_point, k],
            });
        } else {
            out.push(Schema::Contact { jac, point: k });
        }
    }
    // Friction.
    let mut max_impulse = inv_n * sum_friction * sum_impulse;
    if max_impulse <= 0.0 {
        return;
    }
    let first_normal = m.points[0].normal;
    let s2 = dot(sum_normal, sum_normal);
    let mut normal = if s2 * inv_n * inv_n <= 0.9999 {
        if s2 <= 0.1 {
            if dot(first_normal, first_normal) < 0.9 {
                max_impulse = 0.0;
                [0.0, 1.0, 0.0]
            } else {
                first_normal
            }
        } else {
            normalized(sum_normal)
        }
    } else {
        first_normal
    };
    normal = [normal[0], normal[1], normal[2]];
    let mut t0 = cross(AXES[m.info.index.min(2) as usize], normal);
    if dot(t0, t0) <= 0.1 {
        let (x, y, z) = (normal[0].abs(), normal[1].abs(), normal[2].abs());
        let index = if y <= x {
            if z <= y {
                2
            } else {
                1
            }
        } else if z <= x {
            2
        } else {
            0
        };
        m.info.index = index;
        m.info.data[2] = 0.0;
        m.info.data[4] = 0.0;
        t0 = cross(AXES[index as usize], normal);
    }
    let t0 = normalized(t0);
    let t1 = cross(normal, t0);
    let center = scale(sum_position, inv_n);
    let (r_a, r_b) = (sub(center, a.center), sub(center, b.center));
    let (mut j0, e0) = jacobian(a, b, t1, r_a, r_b);
    j0.inv_jac_diag = q.virt_mass_factor / e0;
    j0.rhs = m.info.data[2] * q.friction_rhs_factor;
    let (mut j1, e1) = jacobian(a, b, t0, r_a, r_b);
    j1.inv_jac_diag = q.virt_mass_factor / e1;
    j1.rhs = m.info.data[4] * q.friction_rhs_factor;
    let off = coupling(a, b, &j0, &j1);
    let det = q.virt_mass_factor / (e0 * e1 - off * off);
    j0.extra = det * e1;
    j1.extra = e0 * det;
    let off = -off * det;
    let max_impulse = q.inv_num_steps_times_micro_steps * max_impulse;
    if n < 2 {
        out.push(Schema::Friction2d {
            j0,
            j1,
            off,
            max_impulse,
        });
        return;
    }
    if m.info.flags & INFO_RADIUS_DIRTY != 0 {
        let radius = m
            .points
            .iter()
            .map(|p| length(sub(p.position, center)))
            .sum::<f32>()
            * inv_n;
        m.info.data[0] = radius;
        if radius < 1e-6 {
            out.push(Schema::Friction2d {
                j0,
                j1,
                off,
                max_impulse,
            });
            return;
        }
        m.info.flags &= !INFO_RADIUS_DIRTY;
    }
    let radius = m.info.data[0];
    let ang_a = apply_cols(&a.to_local, normal);
    let ang_b = apply_cols(&b.to_local, scale(normal, -1.0));
    let mut eff = EPS;
    for k in 0..3 {
        eff += ang_a[k] * ang_a[k] * a.inv_inertia[k] + ang_b[k] * ang_b[k] * b.inv_inertia[k];
    }
    let angular = AngularJacobian {
        angular_a: ang_a,
        inv_jac_diag: q.virt_mass_factor / eff * (1.0 / radius),
        angular_b: ang_b,
        rhs: m.info.data[6] * q.friction_rhs_factor,
    };
    out.push(Schema::Friction3d {
        j0,
        j1,
        off,
        angular,
        max_impulse,
        radius,
    });
}

/// `J (v_a − v_b)` for a Jacobian (linear on the difference, angular on
/// each body's own).
fn velocity(j: &Jacobian, a: &Accumulator, b: &Accumulator) -> f32 {
    dot(j.linear, sub(a.linear, b.linear))
        + dot(j.angular_a, a.angular)
        + dot(j.angular_b, b.angular)
}

/// The same on the integrated sums (the export's).
fn sum_velocity(j: &Jacobian, a: &Accumulator, b: &Accumulator) -> f32 {
    dot(j.linear, sub(a.sum_linear, b.sum_linear))
        + dot(j.angular_a, a.sum_angular)
        + dot(j.angular_b, b.sum_angular)
}

/// An impulse `x` along a Jacobian: the first body pushed, the second the
/// other way (each by its inverse mass and inertia).
fn push(j: &Jacobian, x: f32, a: &mut Accumulator, b: &mut Accumulator) {
    a.linear = add(a.linear, scale(j.linear, a.inv_mass * x));
    b.linear = sub(b.linear, scale(j.linear, b.inv_mass * x));
    for k in 0..3 {
        a.angular[k] += j.angular_a[k] * a.inv_inertia[k] * x;
        b.angular[k] += j.angular_b[k] * b.inv_inertia[k] * x;
    }
}

/// `J v` for an angular row (each body's own angular velocity).
fn angular_velocity(j: &AngularJacobian, a: &Accumulator, b: &Accumulator) -> f32 {
    dot(j.angular_a, a.angular) + dot(j.angular_b, b.angular)
}

fn push_angular(j: &AngularJacobian, x: f32, a: &mut Accumulator, b: &mut Accumulator) {
    for k in 0..3 {
        a.angular[k] += j.angular_a[k] * a.inv_inertia[k] * x;
        b.angular[k] += j.angular_b[k] * b.inv_inertia[k] * x;
    }
}

/// Two accumulators at once (`a` ≠ `b`; the fixed body's, index 0, is
/// never written since nothing moves it, but may be either side).
fn pair_mut(accs: &mut [Accumulator], a: usize, b: usize) -> (&mut Accumulator, &mut Accumulator) {
    assert_ne!(a, b);
    if a < b {
        let (lo, hi) = accs.split_at_mut(b);
        (&mut lo[a], &mut hi[0])
    } else {
        let (lo, hi) = accs.split_at_mut(a);
        (&mut hi[0], &mut lo[b])
    }
}

/// One pass over the schemas (`00d8d030`'s schema loop; the types as
/// above). Contacts: the impulse that brings the velocity along the normal
/// to the right-hand side, kept so the point's total stays at or above 0;
/// a pair: both as a 2 × 2 block when both stay above 0, else the second
/// alone when it does, else the first alone. Friction: the impulses that
/// stop the sliding (and the turning), the lot scaled down to at most the
/// most impulse (the scale kept in the results).
// Translated from 00d8d030 (decompiled, FalloutNV.exe 1.4.0.525)
fn solve_schemas(
    info: &SolverInfo,
    schemas: &[Schema],
    accs: &mut [Accumulator],
    results: &mut [f32],
) {
    let (mut ia, mut ib) = (0usize, 0usize);
    let mut r = 0usize;
    for s in schemas {
        match s {
            Schema::Header { a, b, .. } => {
                ia = *a;
                ib = *b;
            }
            Schema::Contact { jac, .. } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let x = ((jac.rhs - velocity(jac, a, b)) * jac.inv_jac_diag).max(0.0 - results[r]);
                push(jac, x, a, b);
                results[r] += x;
            }
            Schema::ContactPair {
                first, second, off, ..
            } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let r1 = first.rhs - velocity(first, a, b);
                let r2 = second.rhs - velocity(second, a, b);
                let x1 = r1 * first.extra + r2 * off;
                let x2 = r2 * second.extra + off * r1;
                if x1 <= -results[r] {
                    let x2 = r2 * second.inv_jac_diag;
                    if x2 <= -results[r + 1] {
                        let x = (r1 * first.inv_jac_diag).max(0.0 - results[r]);
                        push(first, x, a, b);
                        results[r] += x;
                    } else {
                        push(second, x2, a, b);
                        results[r + 1] += x2;
                    }
                } else if -results[r + 1] < x2 {
                    push(first, x1, a, b);
                    results[r] += x1;
                    push(second, x2, a, b);
                    results[r + 1] += x2;
                } else {
                    let x = (r1 * first.inv_jac_diag).max(0.0 - results[r]);
                    push(first, x, a, b);
                    results[r] += x;
                }
            }
            Schema::Friction2d {
                j0,
                j1,
                off,
                max_impulse,
            } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let r0 = j0.rhs - velocity(j0, a, b);
                let r1 = j1.rhs - velocity(j1, a, b);
                let mut x0 = r1 * off + j0.extra * r0;
                let mut x1 = r1 * j1.extra + off * r0;
                let mut factor = 1.0;
                let l2 = x0 * x0 + x1 * x1;
                if l2 > max_impulse * max_impulse {
                    factor = max_impulse / l2.sqrt();
                    x0 *= factor;
                    x1 *= factor;
                }
                results[r + 2] = factor;
                push(j0, x0, a, b);
                results[r] += x0;
                push(j1, x1, a, b);
                results[r + 1] += x1;
            }
            Schema::Friction3d {
                j0,
                j1,
                off,
                angular,
                max_impulse,
                radius,
            } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let r0 = j0.rhs - velocity(j0, a, b);
                let r1 = j1.rhs - velocity(j1, a, b);
                let r2 = angular.rhs
                    - (dot(angular.angular_a, a.angular) + dot(angular.angular_b, b.angular));
                let mut x2 = r2 * angular.inv_jac_diag;
                let mut x0 = r1 * off + j0.extra * r0;
                let mut x1 = r1 * j1.extra + off * r0;
                let mut factor = 1.0;
                let l2 = x2 * x2 + x0 * x0 + x1 * x1;
                if l2 > max_impulse * max_impulse {
                    factor = (1.0 / l2.sqrt()) * max_impulse;
                    x0 *= factor;
                    x1 *= factor;
                    x2 *= factor;
                }
                results[r + 3] = factor;
                let turn = radius * x2;
                push(j0, x0, a, b);
                results[r] += x0;
                push(j1, x1, a, b);
                results[r + 1] += x1;
                push_angular(angular, turn, a, b);
                results[r + 2] += turn;
            }
            Schema::Linear { jac } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let x = (jac.rhs - velocity(jac, a, b)) * jac.inv_jac_diag;
                push(jac, x, a, b);
                results[r] += x;
            }
            Schema::AngularRow { row, .. } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let x = (row.rhs - angular_velocity(row, a, b)) * row.inv_jac_diag;
                push_angular(row, x, a, b);
                results[r] += x;
            }
            Schema::AngularLimit {
                row,
                lower,
                upper,
                tau,
                ..
            } => {
                let (a, b) = pair_mut(accs, ia, ib);
                // The velocity the row sees: the velocities less their sums
                // (× damping) and the sums (× tau × damping ÷ tau).
                let inv_factor = 1.0 / info.integrate_velocity_factor;
                let moved = dot(row.angular_a, sub(a.angular, a.sum_angular))
                    + dot(row.angular_b, sub(b.angular, b.sum_angular));
                let summed = dot(row.angular_a, a.sum_angular) + dot(row.angular_b, b.sum_angular);
                let seen = info.damping * moved + tau * inv_factor * summed;
                let wanted = tau * row.rhs - seen;
                // Past the lower bound it pushes up (the impulse stays at
                // or above 0), past the upper it pushes down.
                let x1 = (wanted - lower) * row.inv_jac_diag;
                let x = if x1 <= -results[r] {
                    let x2 = (wanted - upper) * row.inv_jac_diag;
                    (x2 < -results[r]).then_some(x2)
                } else {
                    Some(x1)
                };
                if let Some(x) = x {
                    push_angular(row, x, a, b);
                    results[r] += x;
                }
            }
            Schema::AngularFriction {
                row, max_impulse, ..
            } => {
                let (a, b) = pair_mut(accs, ia, ib);
                let mut x = row.inv_jac_diag * (row.rhs - angular_velocity(row, a, b));
                let mut factor = 1.0;
                if x.abs() > *max_impulse {
                    factor = max_impulse / x.abs();
                    x *= factor;
                }
                results[r + 1] = factor;
                push_angular(row, x, a, b);
                results[r] += x;
            }
        }
        r += s.results();
    }
}

/// The velocities between substeps (`00d8d030` after each substep, as
/// `hkSolveIntegrateVelocitiesByTheSteps`, Xbox PDB, does): for dynamic and
/// character bodies, velocities with any component at or over 1e6 (or not
/// a number) set to the unit x (`01268370`); a body whose velocities, by
/// component, are within its deactivation class's thresholds (|ω| × the
/// angular inverse + |v| × the linear inverse ≤ 1) slowed by the slow
/// object multiplier, or stopped when within the relative sleep velocity
/// too. Then, for every body, the sums gain the integrate velocity factor ×
/// (velocity − sum); on the last substep the sums × (1 ÷ substeps) ÷ that
/// factor become the velocities the positions move by and the velocities
/// lose the old sums; otherwise the velocities are the new sums + what
/// they were less the old sums, + gravity × the gravity factor (dynamic
/// bodies only).
// Translated from 00d8d030 (decompiled, FalloutNV.exe 1.4.0.525)
fn integrate_velocities(s: &SolverInfo, step: &Step, accs: &mut [Accumulator], last: bool) {
    let f = s.integrate_velocity_factor;
    let finish = s.inv_num_steps * (1.0 / f);
    for acc in accs.iter_mut() {
        let dynamic = acc.kind == acc_kind::DYNAMIC;
        if dynamic || acc.kind == acc_kind::CHARACTER {
            // The measure is taken before the reset (as the code orders it).
            let info = &s.deactivation[acc.deactivation_class.min(5)];
            let e: Vec3 = std::array::from_fn(|k| {
                info.angular_velocity_threshold_inv * acc.angular[k].abs()
                    + info.linear_velocity_threshold_inv * acc.linear[k].abs()
            });
            let ok = |v: Vec3| v.iter().all(|c| c.abs() < 1e6);
            if !(ok(acc.linear) && ok(acc.angular)) {
                acc.linear = [1.0, 0.0, 0.0];
                acc.angular = [1.0, 0.0, 0.0];
            }
            if e.iter().all(|&x| x <= 1.0) {
                if e.iter()
                    .all(|&x| x <= info.relative_sleep_velocity_threshold)
                {
                    acc.linear = [0.0; 3];
                    acc.angular = [0.0; 3];
                } else {
                    let m = info.slow_object_velocity_multiplier;
                    acc.angular = scale(acc.angular, m);
                    acc.linear = scale(acc.linear, m);
                }
            }
        }
        let gravity = if dynamic {
            scale(step.gravity_per_substep, acc.gravity_factor)
        } else {
            [0.0; 3]
        };
        let dv = sub(acc.linear, acc.sum_linear);
        let dw = sub(acc.angular, acc.sum_angular);
        acc.sum_linear = add(acc.sum_linear, scale(dv, f));
        acc.sum_angular = add(acc.sum_angular, scale(dw, f));
        if last {
            acc.sum_linear = scale(acc.sum_linear, finish);
            acc.sum_angular = scale(acc.sum_angular, finish);
            acc.linear = dv;
            acc.angular = dw;
        } else {
            acc.linear = add(add(dv, acc.sum_linear), gravity);
            acc.angular = add(dw, acc.sum_angular);
        }
    }
}

/// `hkSolveConstraints` (`00d8d030`): gravity × the gravity factor into
/// the dynamic bodies' velocities and every sum cleared; then for each
/// substep, each micro step a pass over the schemas, and the velocities
/// integrated ([`integrate_velocities`]). Gives the results (each
/// schema's impulses).
// Translated from 00d8d030 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn solve(
    s: &SolverInfo,
    step: &Step,
    schemas: &[Schema],
    accs: &mut [Accumulator],
) -> Vec<f32> {
    let mut results = vec![0.0f32; schemas.iter().map(Schema::results).sum()];
    for acc in accs.iter_mut() {
        acc.sum_linear = [0.0; 3];
        acc.sum_angular = [0.0; 3];
        if acc.kind == acc_kind::DYNAMIC {
            acc.linear = add(
                acc.linear,
                scale(step.gravity_per_substep, acc.gravity_factor),
            );
        }
    }
    let n = s.num_steps.max(1);
    for k in 0..n {
        for _ in 0..s.num_micro_steps.max(1) {
            solve_schemas(s, schemas, accs, &mut results);
        }
        integrate_velocities(s, step, accs, k == n - 1);
    }
    results
}

/// `hkSolverExport` (`00def570`) for a contact constraint's schemas: each
/// point's impulse, and its solver data: the right-hand side × damping ÷
/// tau × the substep less the Jacobian on the integrated velocities × the
/// step (what the distance is expected to be, negated); the friction's
/// impulses and drifts (× the last pass's scale; with friction tau for
/// damping ÷ tau) into the info.
// Translated from 00def570 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn export(
    s: &SolverInfo,
    step: &Step,
    schemas: &[Schema],
    results: &[f32],
    accs: &[Accumulator],
    manifolds: &mut [&mut Manifold],
) {
    let rhs_scale = s.damp_div_tau * step.substep_dt;
    let friction_scale = s.damp_div_friction_tau * step.substep_dt;
    let frame = s.num_steps as f32 * step.substep_dt;
    let (mut ia, mut ib, mut c) = (0usize, 0usize, 0usize);
    let mut r = 0usize;
    for sch in schemas {
        match sch {
            Schema::Header { a, b, constraint } => {
                ia = *a;
                ib = *b;
                c = *constraint;
            }
            Schema::Contact { jac, point } => {
                let (a, b) = (&accs[ia], &accs[ib]);
                let p = &mut manifolds[c].points[*point].props;
                p.impulse_applied = results[r];
                p.internal_solver_data = jac.rhs * rhs_scale - sum_velocity(jac, a, b) * frame;
            }
            Schema::ContactPair {
                first,
                second,
                points,
                ..
            } => {
                let (a, b) = (&accs[ia], &accs[ib]);
                for (j, (jac, point)) in [(first, points[0]), (second, points[1])]
                    .into_iter()
                    .enumerate()
                {
                    let p = &mut manifolds[c].points[point].props;
                    p.impulse_applied = results[r + j];
                    p.internal_solver_data = jac.rhs * rhs_scale - sum_velocity(jac, a, b) * frame;
                }
            }
            Schema::Friction2d { j0, j1, .. } => {
                let (a, b) = (&accs[ia], &accs[ib]);
                let factor = results[r + 2];
                let d = &mut manifolds[c].info.data;
                d[1] = results[r];
                d[2] = factor * (j0.rhs * friction_scale - sum_velocity(j0, a, b) * frame);
                d[3] = results[r + 1];
                d[4] = factor * (j1.rhs * friction_scale - sum_velocity(j1, a, b) * frame);
            }
            Schema::Friction3d {
                j0, j1, angular, ..
            } => {
                let (a, b) = (&accs[ia], &accs[ib]);
                let factor = results[r + 3];
                let d = &mut manifolds[c].info.data;
                d[1] = results[r];
                d[2] = factor * (j0.rhs * friction_scale - sum_velocity(j0, a, b) * frame);
                d[3] = results[r + 1];
                d[4] = (j1.rhs * friction_scale - sum_velocity(j1, a, b) * frame) * factor;
                d[5] = results[r + 2];
                let ang =
                    dot(angular.angular_a, a.sum_angular) + dot(angular.angular_b, b.sum_angular);
                d[6] = factor * (angular.rhs * friction_scale - ang * frame);
            }
            // The joints' rows export through `crate::constraint::export`.
            Schema::Linear { .. }
            | Schema::AngularRow { .. }
            | Schema::AngularLimit { .. }
            | Schema::AngularFriction { .. } => {}
        }
        r += sch.results();
    }
}
