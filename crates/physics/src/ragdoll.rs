//! A ragdoll: rigid bodies joined at pivots, falling under gravity and
//! coming to rest on a [`Collider`], as the game's dead do.
//!
//! The bodies, their joints and the joints' limits come from the skeleton
//! (`nif::ragdoll`). The game hands them to Havok, and so does this: the
//! joints' constraint atoms ([`crate::constraint`]: the ball socket, twist,
//! cone and plane limits, angular friction, the hinge's limit and 2-D
//! constraint) are built into the solver's Jacobians by the game's builder
//! (`00d6f460`) and solved with the contacts of the ragdoll's capsules
//! (against the world's triangles and against each other) in one island
//! solve ([`crate::solver`], `hkpConstraintSolverSetup::solve` `00d88b50`),
//! once a world step (`fMaxTime` 0.016 s in `Fallout.ini`), in the world's
//! solver substeps.
//!
//! Read from the game: masses, inertia, centres of mass, damping, friction,
//! restitution, the most speed, every capsule, pivot, axis and angle limit,
//! the most friction torque, the step, the solver's settings and the
//! atoms' constants (see [`crate::constraint`]). A ragdoll's own bodies
//! meet as the game's collision filter has them: one system group, so the
//! part table decides ([`parts_meet`], `00c84740`). Not Havok's: the
//! contact points themselves (Havok's collision agents aren't translated;
//! capsule ends and the axis' closest point against the triangles, the
//! closest points of two capsules, this generator's), the ragdoll's
//! motors (the game's death leaves them off), and gravity (real gravity,
//! as for walking).

use crate::constraint::{self, Atom, Frames, JointRuntime, Pose};
use crate::manifold::{point_properties, Manifold, NewPoint, PointProperties};
use crate::solver::{self, Accumulator, BodyVelocity, QueryIn, Step};
use crate::vec::*;
use crate::{closest_on_triangle, segment_triangle_closest, Collider, Vec3, HAVOK_UNIT};

/// A rotation as a row-major matrix (column vectors), as `nif` stores it.
pub type Mat3 = [[f32; 3]; 3];
/// A rotation as a unit quaternion (x, y, z, w).
pub(crate) type Quat = [f32; 4];

/// The physics step: `[HAVOK] fMaxTime` (0.016 s).
pub const STEP: f32 = 0.016;

/// What a body is made of, in its own frame (game units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodySetup {
    pub mass: f32,
    pub center: Vec3,
    /// The inertia tensor's diagonal about the centre.
    pub inertia: Vec3,
    /// Two ends and a radius.
    pub capsule: Option<(Vec3, Vec3, f32)>,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub max_linear_speed: f32,
    pub max_angular_speed: f32,
    /// Its Havok layer and body part number (the filter word's low byte
    /// and bits 8–12): which other bodies of the same ragdoll it meets
    /// ([`crate::layers::Filter::collides`]).
    pub layer: u8,
    pub part: u8,
}

/// Whether two bodies of one ragdoll meet: one ragdoll's bodies share a
/// system group, so the collision filter (`00c84740`) takes the part
/// table for them when both are on the biped or dead-biped layer
/// (`00624070`: layer 8 or 29); the order it's asked in isn't known, so
/// either way round counts.
pub fn parts_meet(a: &BodySetup, b: &BodySetup) -> bool {
    let f = crate::layers::Filter::shared();
    // Any non-zero system group, the same for both.
    let word = |s: &BodySetup| u32::from(s.layer & 0x7f) | u32::from(s.part & 0x1f) << 8 | 1 << 16;
    let linked = |s: &BodySetup| matches!(s.layer, 8 | 29);
    let both = linked(a) && linked(b);
    f.collides(word(a), word(b), both) || f.collides(word(b), word(a), both)
}

/// How far a joint lets its bodies turn: each vector in its own body's
/// frame (the first body's, then the second's).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Limit {
    /// The first body's twist axis stays within `cone` of the second's;
    /// its angle out of the plane across the second's plane axis within
    /// `plane_range`; its turn about the twist axis (measured between the
    /// plane axes) within `twist_range`.
    Cone {
        twist: [Vec3; 2],
        plane: [Vec3; 2],
        cone: f32,
        plane_range: (f32, f32),
        twist_range: (f32, f32),
    },
    /// The axles stay lined up; the turn about them (between the
    /// perpendicular axes) stays within `range`.
    Hinge {
        axle: [Vec3; 2],
        perpendicular: [Vec3; 2],
        range: (f32, f32),
    },
    Free,
}

/// A joint: two bodies, and the point each holds (in its frame).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointSetup {
    pub bodies: [usize; 2],
    pub pivots: [Vec3; 2],
    pub limit: Limit,
    /// Each body's third axis of the joint's frame (the twist and plane
    /// axes, or the axle and perpendicular, come first; `nif::RagdollJoint
    /// ::third`).
    pub third: [Vec3; 2],
    /// The most angular friction torque (Havok units).
    pub max_friction: f32,
}

impl JointSetup {
    /// The joint's constraint atoms, for the solver's builder.
    fn atoms(&self) -> Vec<Atom> {
        match self.limit {
            Limit::Cone {
                cone,
                plane_range,
                twist_range,
                ..
            } => constraint::ragdoll_atoms(cone, plane_range, twist_range, self.max_friction),
            Limit::Hinge { range, .. } => constraint::hinge_atoms(range, self.max_friction),
            Limit::Free => constraint::ball_atoms(),
        }
    }

    /// The frames: each body's three axes and the pivot (Havok units).
    fn frames(&self) -> Frames {
        let axes = |k: usize| match self.limit {
            Limit::Cone { twist, plane, .. } => [twist[k], plane[k], self.third[k]],
            Limit::Hinge {
                axle,
                perpendicular,
                ..
            } => [axle[k], perpendicular[k], self.third[k]],
            Limit::Free => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
        Frames {
            axes: [axes(0), axes(1)],
            pivots: self.pivots.map(|p| scale(p, 1.0 / HAVOK_UNIT)),
        }
    }
}

/// A joint's angles now, against its limits (radians).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JointAngles {
    Cone { cone: f32, plane: f32, twist: f32 },
    Hinge { misaligned: f32, angle: f32 },
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    /// The centre of mass, in the world.
    x: Vec3,
    q: Quat,
    v: Vec3,
    w: Vec3,
}

/// A ragdoll in motion.
#[derive(Debug, Clone)]
pub struct Ragdoll {
    pub bodies: Vec<BodySetup>,
    pub joints: Vec<JointSetup>,
    /// Each joint's atoms and what they keep from step to step.
    atoms: Vec<Vec<Atom>>,
    runtimes: Vec<JointRuntime>,
    /// Pairs of its own bodies that meet ([`parts_meet`]) and the contact
    /// points between each pair's capsules.
    pairs: Vec<(usize, usize)>,
    pair_points: Vec<Manifold>,
    /// Each body's contact points with the world's triangles.
    world_points: Vec<Manifold>,
    state: Vec<State>,
    /// Each body's Havok motion: speed limits and deactivation state
    /// (`crate::havok::Motion`; position and velocities are in `state`).
    motions: Vec<crate::havok::Motion>,
    /// The world's step clock (`crate::havok::Clock`).
    pub clock: crate::havok::Clock,
    /// Marked inactive at the last step (put to sleep at the next one's
    /// start).
    inactive: bool,
    /// Its solver settings (`crate::havok::SolverInfo`).
    solver: crate::havok::SolverInfo,
    /// At rest: no longer stepped.
    pub asleep: bool,
}

/// Downward acceleration, units per second squared: the Havok world's
/// (`crate::GRAVITY`); nothing in the game's death path changes a body's
/// gravity factor.
pub const GRAVITY: f32 = crate::GRAVITY;

/// A rotation's columns (its axes in the world).
fn columns(q: Quat) -> [Vec3; 3] {
    let r = quat_mat(q);
    [
        [r[0][0], r[1][0], r[2][0]],
        [r[0][1], r[1][1], r[2][1]],
        [r[0][2], r[1][2], r[2][2]],
    ]
}

/// A contact point's identity: what kind of touch, which feature (capsule
/// end or axis) and which triangle.
fn point_key(kind: u64, feature: u64, triangle: u32) -> u64 {
    kind << 56 | feature << 32 | u64::from(triangle)
}

/// The contact manager's update for one agent (`hkpSimpleConstraintContactMgr`
/// `00cfcf80`, `00cfd320`, as [`crate::rigid`] does): points still found
/// keep their properties with the new position, normal and distance, the
/// ones no longer found go, new ones are added with their properties.
fn update_points(m: &mut Manifold, found: &[(NewPoint, PointProperties)]) {
    let mut k = 0;
    while k < m.points.len() {
        if let Some((n, _)) = found.iter().find(|(n, _)| n.key == m.points[k].key) {
            let p = &mut m.points[k];
            p.position = n.position;
            p.normal = n.normal;
            p.distance = n.distance;
            k += 1;
        } else {
            m.remove(k);
        }
    }
    for (n, props) in found {
        if !m.points.iter().any(|p| p.key == n.key) {
            m.add(*n, *props);
        }
    }
}

impl Ragdoll {
    /// Bodies placed at `frames` (each body frame's rotation and origin in
    /// the world), at rest.
    pub fn new(bodies: Vec<BodySetup>, joints: Vec<JointSetup>, frames: &[(Mat3, Vec3)]) -> Self {
        let state = bodies
            .iter()
            .zip(frames)
            .map(|(b, (r, origin))| State {
                x: add(*origin, mat_vec(r, b.center)),
                q: mat_quat(r),
                v: [0.0; 3],
                w: [0.0; 3],
            })
            .collect();
        let motions = bodies
            .iter()
            .zip(&state)
            .map(|(b, s): (&BodySetup, &State)| {
                let mut m = crate::havok::Motion::new(crate::havok::motion_type::DYNAMIC, s.x, s.q);
                m.max_linear_velocity =
                    crate::rigid::speed_index(b.max_linear_speed / crate::HAVOK_UNIT);
                m.max_angular_velocity = crate::rigid::speed_index(b.max_angular_speed);
                m.object_radius = b.capsule.map_or(0.0, |(a, e, r)| {
                    length(sub(a, b.center)).max(length(sub(e, b.center))) + r
                }) / crate::HAVOK_UNIT;
                m
            })
            .collect();
        let mut pairs = Vec::new();
        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let (a, b) = (&bodies[i], &bodies[j]);
                if a.capsule.is_some() && b.capsule.is_some() && parts_meet(a, b) {
                    pairs.push((i, j));
                }
            }
        }
        let atoms: Vec<Vec<Atom>> = joints.iter().map(JointSetup::atoms).collect();
        let runtimes = atoms.iter().map(|a| JointRuntime::new(a)).collect();
        Ragdoll {
            pair_points: vec![Manifold::default(); pairs.len()],
            world_points: vec![Manifold::default(); bodies.len()],
            bodies,
            joints,
            atoms,
            runtimes,
            pairs,
            state,
            motions,
            clock: crate::havok::Clock::default(),
            inactive: false,
            solver: crate::havok::SolverInfo::new(),
            asleep: false,
        }
    }

    /// A body's frame now: its rotation and origin in the world.
    pub fn frame(&self, body: usize) -> (Mat3, Vec3) {
        let s = &self.state[body];
        let r = quat_mat(s.q);
        (r, sub(s.x, mat_vec(&r, self.bodies[body].center)))
    }

    /// Each body's capsule where it is now, in the world: body index, ends
    /// and radius (bodies of other shapes are left out).
    pub fn world_capsules(&self) -> Vec<(usize, Vec3, Vec3, f32)> {
        (0..self.bodies.len())
            .filter_map(|i| {
                let (a, b, r) = self.bodies[i].capsule?;
                let (m, origin) = self.frame(i);
                Some((
                    i,
                    add(origin, mat_vec(&m, a)),
                    add(origin, mat_vec(&m, b)),
                    r,
                ))
            })
            .collect()
    }

    /// Sets every body moving at `velocity`.
    pub fn set_velocity(&mut self, velocity: Vec3) {
        for s in &mut self.state {
            s.v = velocity;
        }
        self.wake();
    }

    /// Every body's speed changes by `velocity`, whatever its mass (the
    /// game adds a velocity to each moving body of a tree: `0062b930`).
    pub fn add_velocity(&mut self, velocity: Vec3) {
        for s in &mut self.state {
            s.v = add(s.v, velocity);
        }
        self.wake();
    }

    /// The game's death push (`0062b660`): every body's speed changes by
    /// `speed` × its multiplier, away from `origin` (each body along the
    /// line from `origin` to its centre), whatever its mass.
    pub fn throw_from(&mut self, origin: Vec3, speed: f32, multipliers: &[f32]) {
        for (i, s) in self.state.iter_mut().enumerate() {
            let m = multipliers.get(i).copied().unwrap_or(1.0);
            let away = sub(s.x, origin);
            let d = length(away);
            if d > 1e-6 && m != 0.0 {
                s.v = add(s.v, scale(away, speed * m / d));
            }
        }
        self.wake();
    }

    /// A blow (mass × units a second) to one body, through its centre.
    pub fn push(&mut self, body: usize, impulse: Vec3) {
        let m = self.bodies[body].mass;
        if m > 0.0 {
            let s = &mut self.state[body];
            s.v = add(s.v, scale(impulse, 1.0 / m));
            self.wake();
        }
    }

    /// The angles of a joint now.
    pub fn joint_angles(&self, joint: usize) -> JointAngles {
        let j = &self.joints[joint];
        let r1 = quat_mat(self.state[j.bodies[0]].q);
        let r2 = quat_mat(self.state[j.bodies[1]].q);
        joint_angles(&j.limit, &r1, &r2)
    }

    /// The fastest any body moves (units a second) and turns (radians a
    /// second) now, and which.
    pub fn fastest(&self) -> ((f32, usize), (f32, usize)) {
        let mut out = ((0.0, 0), (0.0, 0));
        for (i, s) in self.state.iter().enumerate() {
            if length(s.v) > out.0 .0 {
                out.0 = (length(s.v), i);
            }
            if length(s.w) > out.1 .0 {
                out.1 = (length(s.w), i);
            }
        }
        out
    }

    /// How far apart a joint's two pivots are now (0 when it holds).
    pub fn joint_gap(&self, joint: usize) -> f32 {
        let j = &self.joints[joint];
        let p = |k: usize| {
            let (r, origin) = self.frame(j.bodies[k]);
            add(origin, mat_vec(&r, j.pivots[k]))
        };
        length(sub(p(0), p(1)))
    }

    /// Moves on by a frame of `dt` seconds, in the world's whole steps
    /// ([`crate::havok::Clock`], `00c66760`/`00c6ae70`).
    pub fn update(&mut self, collider: &Collider, dt: f32) {
        let (n, step) = self.clock.advance(dt);
        for _ in 0..n {
            if self.asleep {
                return;
            }
            self.step_by(collider, step);
        }
    }

    /// One physics step of the game's length.
    pub fn step(&mut self, collider: &Collider) {
        self.step_by(collider, STEP);
    }

    /// One physics step of `dt` seconds: the contact points for where the
    /// bodies are, the joints' and contacts' Jacobians, the island's solve
    /// (`hkpConstraintSolverSetup::solve` `00d88b50`) and the bodies moved
    /// (`hkRigidMotionUtilApplyAccumulators` `00d29bf0`), with the
    /// deactivation bookkeeping.
    // Translated from 00d88b50 and 00d29bf0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn step_by(&mut self, collider: &Collider, dt: f32) {
        self.begin_step();
        if self.asleep {
            return;
        }
        let n = self.bodies.len();
        let u = HAVOK_UNIT;
        // Havok's forces for a constrained island (`00d29830`): damping
        // once a step.
        for i in 0..n {
            if self.bodies[i].mass > 0.0 {
                let mut m = self.motion(i);
                m.apply_damping(dt);
                self.set_motion(i, &m);
            }
        }
        self.find_points(collider);
        let origin = self.state[0].x;
        let hv = |p: Vec3| scale(sub(p, origin), 1.0 / u);
        // The accumulators: the world's fixed body, then each body.
        let mut accs = vec![Accumulator::fixed()];
        let mut spin = vec![[0.0f32; 3]];
        let mut poses = Vec::with_capacity(n);
        for i in 0..n {
            let (s, b) = (&self.state[i], &self.bodies[i]);
            let rotation = columns(s.q);
            if b.mass > 0.0 {
                let inertia = b.inertia.map(|k| u * u / k.max(1e-6));
                accs.push(Accumulator::dynamic(
                    hv(s.x),
                    rotation,
                    scale(s.v, 1.0 / u),
                    s.w,
                    inertia,
                    1.0 / b.mass,
                    usize::from(self.motions[i].deactivation_class),
                    crate::havok::from_half(self.motions[i].gravity_factor),
                ));
            } else {
                accs.push(Accumulator::keyframed(hv(s.x), scale(s.v, 1.0 / u)));
            }
            spin.push(s.w);
            let (_, frame_origin) = self.frame(i);
            poses.push(Pose {
                origin: hv(frame_origin),
                rotation,
            });
        }
        // The contact constraints (Havok units): the bodies' points with the
        // world (against the fixed body), then the pairs'.
        let mut copies: Vec<Manifold> = Vec::new();
        let mut sides = Vec::new();
        let havok = |m: &Manifold| {
            let mut c = m.clone();
            for p in &mut c.points {
                p.position = hv(p.position);
                p.distance /= u;
            }
            c
        };
        let mut keys = Vec::new();
        for (i, m) in self.world_points.iter().enumerate() {
            if !m.is_empty() {
                copies.push(havok(m));
                sides.push((1 + i, 0));
                keys.push((false, i));
            }
        }
        for (k, m) in self.pair_points.iter().enumerate() {
            if !m.is_empty() {
                let (a, b) = self.pairs[k];
                copies.push(havok(m));
                sides.push((1 + a, 1 + b));
                keys.push((true, k));
            }
        }
        let step = Step::new(&self.solver, dt, crate::havok::GRAVITY);
        let q = QueryIn::new(&self.solver, &step);
        let cx = constraint::Context {
            q: &q,
            tau: self.solver.tau,
        };
        let mut schemas = Vec::new();
        // The joints first (their bodies' constraints are older than the
        // contacts), their headers numbered after the contacts'.
        for (j, joint) in self.joints.iter().enumerate() {
            let [a, b] = joint.bodies;
            let (ia, ib) = (1 + a, 1 + b);
            constraint::build(
                &self.atoms[j],
                &joint.frames(),
                [&poses[a], &poses[b]],
                copies.len() + j,
                &cx,
                &constraint::Sides {
                    ids: (ia, ib),
                    a: &accs[ia],
                    b: &accs[ib],
                },
                &mut self.runtimes[j],
                &mut schemas,
            );
        }
        for (ci, (c, &(ia, ib))) in copies.iter_mut().zip(&sides).enumerate() {
            let velocity = |k: usize| BodyVelocity {
                linear: accs[k].linear,
                angular: spin[k],
                center: accs[k].center,
            };
            solver::fire_callbacks(
                c,
                &q,
                (accs[ia].inv_mass, accs[ib].inv_mass),
                &velocity(ia),
                &velocity(ib),
                self.solver.contact_resting_velocity,
            );
            solver::build_contact_jacobians(
                c,
                ci,
                &q,
                (ia, ib),
                &accs[ia],
                &accs[ib],
                &mut schemas,
            );
        }
        let results = solver::solve(&self.solver, &step, &schemas, &mut accs);
        {
            let mut refs: Vec<&mut Manifold> = copies.iter_mut().collect();
            solver::export(&self.solver, &step, &schemas, &results, &accs, &mut refs);
        }
        constraint::export(
            &self.solver,
            &step,
            &schemas,
            &results,
            &accs,
            copies.len(),
            &mut self.runtimes,
        );
        for (c, &(pair, k)) in copies.iter().zip(&keys) {
            let m = if pair {
                &mut self.pair_points[k]
            } else {
                &mut self.world_points[k]
            };
            for (p, cp) in m.points.iter_mut().zip(&c.points) {
                p.props = cp.props;
            }
            m.info = c.info;
        }
        // The bodies moved (`00d29bf0`) and the deactivation bookkeeping per
        // body; the ragdoll's bodies are one island (its constraints join
        // them): marked inactive when the fewest passing checks is more
        // than 5.
        let mut fewest = u32::MAX;
        for i in 0..n {
            if self.bodies[i].mass <= 0.0 {
                continue;
            }
            let acc = accs[1 + i];
            let cols = columns(self.state[i].q);
            let mut m = self.motion(i);
            if let Some(f) = m.apply_accumulator(
                scale(acc.linear, u),
                solver::apply_cols(&cols, acc.angular),
                scale(acc.sum_linear, u),
                solver::apply_cols(&cols, acc.sum_angular),
                dt,
                u,
                &self.solver,
            ) {
                fewest = fewest.min(f);
            }
            self.set_motion(i, &m);
        }
        self.inactive = fewest != u32::MAX
            && fewest > crate::havok::INACTIVE_FRAMES_TO_DEACTIVATE
            && crate::havok::WANT_DEACTIVATION;
    }

    /// The contact points for where the bodies are (the end of Havok's
    /// previous step, `hkpSimulation::collide` `00cf8bc0`): each capsule
    /// against the world's triangles near it, and each pair of the
    /// ragdoll's own capsules that meet. This generator's (Havok's agents
    /// aren't translated): a capsule's two end spheres and the closest point
    /// of its axis (when it isn't an end) against a triangle, the closest
    /// points of two axes; each within the collision tolerance
    /// (`00cfb570`), on the other side's surface.
    fn find_points(&mut self, collider: &Collider) {
        let tol = crate::havok::COLLISION_TOLERANCE * HAVOK_UNIT;
        for i in 0..self.bodies.len() {
            let Some((ea, eb, radius)) = self.bodies[i].capsule else {
                continue;
            };
            let (r, origin) = self.frame(i);
            let (p, q) = (add(origin, mat_vec(&r, ea)), add(origin, mat_vec(&r, eb)));
            // The collision tolerance and the distance it can close in the step: a
            // point this near is kept, and the solver lets a body approach it no
            // further than the gap (without continuous collision, PR 9, this is
            // what stops a fast body at a surface).
            let tol = tol + length(self.state[i].v) * STEP;
            let reach = radius + 4.0 + tol;
            let lo = [
                p[0].min(q[0]) - reach,
                p[1].min(q[1]) - reach,
                p[2].min(q[2]) - reach,
            ];
            let hi = [
                p[0].max(q[0]) + reach,
                p[1].max(q[1]) + reach,
                p[2].max(q[2]) + reach,
            ];
            let mut found: Vec<(NewPoint, PointProperties)> = Vec::new();
            let b = &self.bodies[i];
            // How far behind a face a point may be and still count.
            let deep = 2.0 * radius + tol + 1.0;
            for t in collider.near(lo, hi) {
                let [ta, tb, tc] = collider.triangle(t);
                if ta[2].max(tb[2]).max(tc[2]) < lo[2] || ta[2].min(tb[2]).min(tc[2]) > hi[2] {
                    continue;
                }
                let shell = collider.shell(t);
                let surface = collider.surface(t).unwrap_or(crate::rigid::DEFAULT_SURFACE);
                let props = point_properties(
                    (b.friction, surface.friction),
                    (b.restitution, surface.restitution),
                );
                let face = {
                    let f = normalize(cross(sub(tb, ta), sub(tc, ta)));
                    if dot(f, sub(self.state[i].x, ta)) >= 0.0 {
                        f
                    } else {
                        scale(f, -1.0)
                    }
                };
                // A point of the capsule (an end, or the axis' closest
                // point) against the triangle: over the face, the face's
                // normal (a point gone through still counts, up to `deep`
                // behind it, as the rigid bodies' do); beside it, from the
                // closest point.
                let margin = radius + shell;
                let mut touch = |c: Vec3, feature: u64| {
                    let s = dot(face, sub(c, ta));
                    let on_plane = sub(c, scale(face, s));
                    let over = {
                        let side = |p: Vec3, q: Vec3, r: Vec3| {
                            dot(cross(sub(q, p), sub(on_plane, p)), face)
                                * dot(cross(sub(q, p), sub(r, p)), face)
                                >= 0.0
                        };
                        side(ta, tb, tc) && side(tb, tc, ta) && side(tc, ta, tb)
                    };
                    let (on, n, d) = if over {
                        if !(s < margin + tol && s > -deep) {
                            return;
                        }
                        (on_plane, face, s)
                    } else {
                        let on = closest_on_triangle(c, ta, tb, tc);
                        let d = length(sub(c, on));
                        if d >= margin + tol || d <= 1e-5 {
                            return;
                        }
                        (on, scale(sub(c, on), 1.0 / d), d)
                    };
                    found.push((
                        NewPoint {
                            key: point_key(1, feature, t),
                            position: add(on, scale(n, shell)),
                            normal: n,
                            distance: d - margin,
                        },
                        props,
                    ));
                };
                touch(p, 0);
                touch(q, 1);
                let (on_axis, _) = segment_triangle_closest(p, q, ta, tb, tc);
                if dist2(on_axis, p) > 1e-4 && dist2(on_axis, q) > 1e-4 {
                    touch(on_axis, 2);
                }
            }
            update_points(&mut self.world_points[i], &found);
        }
        for k in 0..self.pairs.len() {
            let (a, b) = self.pairs[k];
            let (Some((a0, a1, ra)), Some((b0, b1, rb))) =
                (self.bodies[a].capsule, self.bodies[b].capsule)
            else {
                continue;
            };
            let ends = |me: &Self, body: usize, p: Vec3, q: Vec3| {
                let (r, origin) = me.frame(body);
                (add(origin, mat_vec(&r, p)), add(origin, mat_vec(&r, q)))
            };
            let (pa, qa) = ends(self, a, a0, a1);
            let (pb, qb) = ends(self, b, b0, b1);
            let mut found = Vec::new();
            let tol = tol + (length(self.state[a].v) + length(self.state[b].v)) * STEP;
            let (fa, fb) = (self.bodies[a], self.bodies[b]);
            let props =
                point_properties((fa.friction, fb.friction), (fa.restitution, fb.restitution));
            // The ends of each axis against the other, and the axes' closest
            // points when neither is an end; the normal from the second
            // body toward the first.
            let mut touch = |on_a: Vec3, on_b: Vec3, feature: u64| {
                let gap = sub(on_a, on_b);
                let d = length(gap);
                if d >= ra + rb + tol {
                    return;
                }
                let n = if d > 1e-5 {
                    scale(gap, 1.0 / d)
                } else {
                    let between = sub(self.state[a].x, self.state[b].x);
                    if length(between) > 1e-5 {
                        normalize(between)
                    } else {
                        [0.0, 0.0, 1.0]
                    }
                };
                found.push((
                    NewPoint {
                        key: point_key(2, feature, 0),
                        position: add(on_b, scale(n, rb)),
                        normal: n,
                        distance: d - (ra + rb),
                    },
                    props,
                ));
            };
            touch(pa, closest_on_segment(pa, pb, qb), 0);
            touch(qa, closest_on_segment(qa, pb, qb), 1);
            touch(closest_on_segment(pb, pa, qa), pb, 2);
            touch(closest_on_segment(qb, pa, qa), qb, 3);
            let (on_a, on_b) = segments_closest(pa, qa, pb, qb);
            let end = |p: Vec3| [pa, qa, pb, qb].iter().any(|e| dist2(*e, p) < 1e-4);
            if !end(on_a) && !end(on_b) {
                touch(on_a, on_b, 4);
            }
            update_points(&mut self.pair_points[k], &found);
        }
    }

    /// The start of a step (`00cf8da0`): an island marked inactive goes to
    /// sleep when every body passes the last test (`00d28560`), velocities
    /// zeroed (`00cb5310`); then the deactivation flags.
    fn begin_step(&mut self) {
        if std::mem::take(&mut self.inactive) {
            let all = (0..self.bodies.len())
                .filter(|&i| self.bodies[i].mass > 0.0)
                .all(|i| self.motion(i).can_deactivate(crate::HAVOK_UNIT));
            if all {
                self.asleep = true;
                for s in &mut self.state {
                    s.v = [0.0; 3];
                    s.w = [0.0; 3];
                }
                return;
            }
        }
        self.solver.increment_deactivation_flags();
    }

    /// Woken (`00cb5100`): every body's counts start again.
    fn wake(&mut self) {
        if self.asleep {
            for m in &mut self.motions {
                m.activate(&self.solver);
            }
        }
        self.asleep = false;
        self.inactive = false;
    }

    /// Body `i` as a Havok motion (game units), its velocities and limits.
    fn motion(&self, i: usize) -> crate::havok::Motion {
        let (b, s) = (&self.bodies[i], &self.state[i]);
        let mut m = self.motions[i];
        m.center = s.x;
        m.rotation = s.q;
        m.linear_velocity = s.v;
        m.angular_velocity = s.w;
        m.linear_damping = b.linear_damping;
        m.angular_damping = b.angular_damping;
        m
    }

    fn set_motion(&mut self, i: usize, m: &crate::havok::Motion) {
        self.motions[i] = *m;
        let s = &mut self.state[i];
        s.x = m.center;
        s.q = m.rotation;
        s.v = m.linear_velocity;
        s.w = m.angular_velocity;
    }
}

/// A joint's angles for bodies turned by `r1` and `r2` (see [`Limit`]).
pub fn joint_angles(limit: &Limit, r1: &Mat3, r2: &Mat3) -> JointAngles {
    match *limit {
        Limit::Cone { twist, plane, .. } => {
            let (a1, a2) = (mat_vec(r1, twist[0]), mat_vec(r2, twist[1]));
            let c2 = mat_vec(r2, plane[1]);
            JointAngles::Cone {
                cone: angle_between(a1, a2),
                plane: dot(normalize(a1), normalize(c2)).clamp(-1.0, 1.0).asin(),
                twist: twist_angle(a1, a2, mat_vec(r1, plane[0]), c2).map_or(0.0, |(_, t)| t),
            }
        }
        Limit::Hinge {
            axle,
            perpendicular,
            ..
        } => {
            let (a1, a2) = (mat_vec(r1, axle[0]), mat_vec(r2, axle[1]));
            JointAngles::Hinge {
                misaligned: angle_between(a1, a2),
                angle: turn_about(
                    normalize(a1),
                    mat_vec(r2, perpendicular[1]),
                    mat_vec(r1, perpendicular[0]),
                )
                .unwrap_or(0.0),
            }
        }
        Limit::Free => JointAngles::Free,
    }
}

/// The point of segment `a`–`b` closest to `p`.
fn closest_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ab = sub(b, a);
    let l2 = dot(ab, ab);
    if l2 <= 1e-12 {
        return a;
    }
    add(a, scale(ab, (dot(sub(p, a), ab) / l2).clamp(0.0, 1.0)))
}

/// The closest points of two segments (`p1`–`q1`, `p2`–`q2`): on the
/// first, then on the second (Ericson, "Real-Time Collision Detection",
/// 5.1.9).
fn segments_closest(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> (Vec3, Vec3) {
    let d1 = sub(q1, p1);
    let d2 = sub(q2, p2);
    let r = sub(p1, p2);
    let a = dot(d1, d1);
    let e = dot(d2, d2);
    let f = dot(d2, r);
    let eps = 1e-9;
    let (s, t) = if a <= eps && e <= eps {
        (0.0, 0.0)
    } else if a <= eps {
        (0.0, (f / e).clamp(0.0, 1.0))
    } else {
        let c = dot(d1, r);
        if e <= eps {
            ((-c / a).clamp(0.0, 1.0), 0.0)
        } else {
            let b = dot(d1, d2);
            let denom = a * e - b * b;
            let mut s = if denom > eps {
                ((b * f - c * e) / denom).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let mut t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            }
            (s, t)
        }
    };
    (add(p1, scale(d1, s)), add(p2, scale(d2, t)))
}

fn angle_between(a: Vec3, b: Vec3) -> f32 {
    dot(normalize(a), normalize(b)).clamp(-1.0, 1.0).acos()
}

/// The turn about `axis` (unit) that takes `from` to `to`, both flattened
/// across the axis; `None` when either lies along it.
fn turn_about(axis: Vec3, from: Vec3, to: Vec3) -> Option<f32> {
    let flat = |v: Vec3| {
        let f = sub(v, scale(axis, dot(v, axis)));
        (length(f) > 1e-4).then(|| normalize(f))
    };
    let (f, t) = (flat(from)?, flat(to)?);
    Some(dot(axis, cross(f, t)).atan2(dot(f, t)))
}

/// The first body's turn about the shared twist axis relative to the
/// second (between their plane axes), and that axis.
fn twist_angle(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> Option<(Vec3, f32)> {
    let mid = add(normalize(a1), normalize(a2));
    let t = if length(mid) > 1e-3 {
        normalize(mid)
    } else {
        normalize(a1)
    };
    Some((t, turn_about(t, b2, b1)?))
}

pub(crate) fn mat_vec(m: &Mat3, v: Vec3) -> Vec3 {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// The transpose (inverse) of a rotation applied to `v`.
pub(crate) fn mat_t_vec(m: &Mat3, v: Vec3) -> Vec3 {
    [
        m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2],
        m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2],
        m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2],
    ]
}

pub(crate) fn quat_normalize(q: Quat) -> Quat {
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if l > 0.0 {
        q.map(|x| x / l)
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

pub(crate) fn quat_mat(q: Quat) -> Mat3 {
    let [x, y, z, w] = q;
    [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ]
}

/// A rotation matrix as a quaternion (Shepperd's method).
pub(crate) fn mat_quat(m: &Mat3) -> Quat {
    let trace = m[0][0] + m[1][1] + m[2][2];
    let q = if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [
            (m[2][1] - m[1][2]) / s,
            (m[0][2] - m[2][0]) / s,
            (m[1][0] - m[0][1]) / s,
            0.25 * s,
        ]
    } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
        let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
        [
            0.25 * s,
            (m[0][1] + m[1][0]) / s,
            (m[0][2] + m[2][0]) / s,
            (m[2][1] - m[1][2]) / s,
        ]
    } else if m[1][1] > m[2][2] {
        let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
        [
            (m[0][1] + m[1][0]) / s,
            0.25 * s,
            (m[1][2] + m[2][1]) / s,
            (m[0][2] - m[2][0]) / s,
        ]
    } else {
        let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
        [
            (m[0][2] + m[2][0]) / s,
            (m[1][2] + m[2][1]) / s,
            0.25 * s,
            (m[1][0] - m[0][1]) / s,
        ]
    };
    quat_normalize(q)
}

#[cfg(test)]
mod tests {
    use super::*;

    const I3: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    fn floor() -> Collider {
        let mut c = Collider::new();
        c.add(
            &[
                [-1000.0, -1000.0, 0.0],
                [1000.0, -1000.0, 0.0],
                [1000.0, 1000.0, 0.0],
                [-1000.0, 1000.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c
    }

    fn rod(mass: f32) -> BodySetup {
        // A capsule 20 long along x, radius 3.
        BodySetup {
            mass,
            center: [10.0, 0.0, 0.0],
            inertia: [mass * 9.0, mass * 40.0, mass * 40.0],
            capsule: Some(([0.0; 3], [20.0, 0.0, 0.0], 3.0)),
            friction: 0.3,
            restitution: 0.0,
            linear_damping: 0.1,
            angular_damping: 0.05,
            max_linear_speed: 7000.0,
            max_angular_speed: 30.0,
            layer: 8,
            part: 0,
        }
    }

    /// A ball and socket from a rod's origin (the first body) to the end of
    /// the second.
    fn ball(a: usize, b: usize) -> JointSetup {
        JointSetup {
            bodies: [a, b],
            pivots: [[0.0; 3], [20.0, 0.0, 0.0]],
            limit: Limit::Free,
            third: [[0.0; 3]; 2],
            max_friction: 0.0,
        }
    }

    #[test]
    fn rotations_survive_the_round_trip() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let q = mat_quat(&m);
        assert!((q[2] - s).abs() < 1e-5 && (q[3] - s).abs() < 1e-5, "{q:?}");
        let back = quat_mat(q);
        for i in 0..3 {
            for j in 0..3 {
                assert!((back[i][j] - m[i][j]).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn an_added_velocity_moves_every_body_alike_and_wakes_them() {
        // A light and a heavy rod, both asleep: each gains the same
        // velocity (`0062b930` adds it whatever the mass).
        let mut r = Ragdoll::new(
            vec![rod(1.0), rod(50.0)],
            Vec::new(),
            &[(I3, [0.0, 0.0, 100.0]), (I3, [0.0, 50.0, 100.0])],
        );
        r.asleep = true;
        r.add_velocity([20.0, 0.0, 0.0]);
        assert!(!r.asleep);
        r.add_velocity([0.0, -5.0, 0.0]);
        for s in &r.state {
            assert_eq!(s.v, [20.0, -5.0, 0.0]);
        }
    }

    #[test]
    fn a_body_falls_and_rests_on_the_floor() {
        let mut r = Ragdoll::new(vec![rod(5.0)], Vec::new(), &[(I3, [0.0, 0.0, 100.0])]);
        let c = floor();
        for _ in 0..300 {
            r.update(&c, 1.0 / 60.0);
        }
        let (_, origin) = r.frame(0);
        // Lying on its side: its axis the radius above the floor.
        assert!((origin[2] - 3.0).abs() < 0.5, "{origin:?}");
        assert!(r.asleep);
    }

    #[test]
    fn own_bodies_meet_only_as_the_part_table_says() {
        // Two rods side by side at the same spot, the first held up (no
        // mass). A head (part 1) and a left forearm (6) meet in the game's
        // table (`01268078` row 1 has bit 6); a head and the body (2)
        // don't.
        let side_by_side = |part_a: u8, part_b: u8| {
            let mut a = rod(0.0);
            a.part = part_a;
            let mut b = rod(4.0);
            b.part = part_b;
            let mut r = Ragdoll::new(
                vec![a, b],
                Vec::new(),
                &[(I3, [0.0, 0.0, 500.0]), (I3, [0.0, 0.0, 508.0])],
            );
            for _ in 0..60 {
                r.step(&Collider::new());
            }
            let (_, low) = r.frame(0);
            let (_, high) = r.frame(1);
            high[2] - low[2]
        };
        assert!(parts_meet(
            &BodySetup {
                part: 1,
                ..rod(1.0)
            },
            &BodySetup {
                part: 6,
                ..rod(1.0)
            }
        ));
        assert!(!parts_meet(
            &BodySetup {
                part: 1,
                ..rod(1.0)
            },
            &BodySetup {
                part: 2,
                ..rod(1.0)
            }
        ));
        // Meeting: the second lands on the first, the two radii apart.
        let apart = side_by_side(1, 6);
        assert!((5.0..7.0).contains(&apart), "{apart}");
        // Not meeting: the second falls through the first.
        let through = side_by_side(1, 2);
        assert!(through < 0.0, "{through}");
    }

    #[test]
    fn joints_hold_and_hinges_keep_their_range() {
        // Two rods end to end along x, joined by a hinge about y that lets
        // the second bend from 0 to 90°, held 200 up. The first is fixed
        // (no mass); the second hangs down under gravity, stopped by the
        // hinge at 90°.
        let joint = JointSetup {
            bodies: [1, 0],
            pivots: [[0.0; 3], [20.0, 0.0, 0.0]],
            limit: Limit::Hinge {
                axle: [[0.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                perpendicular: [[1.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
                range: (0.0, std::f32::consts::FRAC_PI_2),
            },
            // axle × perpendicular
            third: [[0.0, 0.0, -1.0]; 2],
            max_friction: 0.0,
        };
        let mut r = Ragdoll::new(
            vec![rod(0.0), rod(4.0)],
            vec![joint],
            &[(I3, [0.0, 0.0, 200.0]), (I3, [20.0, 0.0, 200.0])],
        );
        let (mut lowest, mut highest) = (0.0f32, 0.0f32);
        let c = floor();
        for _ in 0..240 {
            r.update(&c, 1.0 / 60.0);
            if let JointAngles::Hinge { angle, .. } = r.joint_angles(0) {
                lowest = lowest.min(angle);
                highest = highest.max(angle);
            }
        }
        assert!(r.joint_gap(0) < 0.5, "gap {}", r.joint_gap(0));
        let JointAngles::Hinge { misaligned, angle } = r.joint_angles(0) else {
            unreachable!()
        };
        assert!(misaligned < 0.02, "{misaligned}");
        // It fell from level, swinging to the limit and no further (positive
        // about the axle: the first body turning from the second's plane axis
        // toward its third), and hangs near straight down.
        assert!(
            lowest > -0.1 && highest < std::f32::consts::FRAC_PI_2 + 0.1,
            "{lowest} {highest}"
        );
        assert!(
            (angle - std::f32::consts::FRAC_PI_2).abs() < 0.25,
            "{angle}"
        );
        // Hanging straight down from the first rod's end.
        let (rot, origin) = r.frame(1);
        let tip = add(origin, mat_vec(&rot, [20.0, 0.0, 0.0]));
        assert!(
            (tip[0] - 20.0).abs() < 6.0 && (tip[2] - 180.0).abs() < 3.0,
            "{tip:?}"
        );
    }

    #[test]
    fn a_hanging_chain_keeps_its_joints_together() {
        // Five rods laid out along x, held at the first's origin, ball and
        // socket joints between them: they swing down like a whip.
        let n = 5;
        let bodies: Vec<BodySetup> = (0..n)
            .map(|i| rod(if i == 0 { 0.0 } else { 3.0 }))
            .collect();
        let joints: Vec<JointSetup> = (1..n).map(|i| ball(i, i - 1)).collect();
        let frames: Vec<(Mat3, Vec3)> = (0..n)
            .map(|i| (I3, [20.0 * i as f32, 0.0, 300.0]))
            .collect();
        let mut r = Ragdoll::new(bodies, joints, &frames);
        let c = floor();
        let mut worst = 0.0f32;
        for _ in 0..360 {
            r.update(&c, 1.0 / 60.0);
            for j in 0..n - 1 {
                worst = worst.max(r.joint_gap(j));
            }
        }
        // The joints stay together (Havok's tau 0.6 closes a gap over a few
        // steps; a whip's swing opens one a little).
        assert!(worst < 2.0, "joint gap {worst}");
    }

    #[test]
    fn a_cone_limit_holds() {
        // A rod hung off the end of a fixed one by a ragdoll joint with a
        // 30° cone: it falls to the cone's edge and no further. Frames:
        // twist x, plane z, third twist × plane = −y.
        let cone = 30.0f32.to_radians();
        let joint = JointSetup {
            bodies: [1, 0],
            pivots: [[0.0; 3], [20.0, 0.0, 0.0]],
            limit: Limit::Cone {
                twist: [[1.0, 0.0, 0.0]; 2],
                plane: [[0.0, 0.0, 1.0]; 2],
                cone,
                plane_range: (-1.0, 1.0),
                twist_range: (-1.0, 1.0),
            },
            third: [[0.0, -1.0, 0.0]; 2],
            max_friction: 0.0,
        };
        let mut r = Ragdoll::new(
            vec![rod(0.0), rod(4.0)],
            vec![joint],
            &[(I3, [0.0, 0.0, 200.0]), (I3, [20.0, 0.0, 200.0])],
        );
        let c = floor();
        let mut widest = 0.0f32;
        for _ in 0..240 {
            r.update(&c, 1.0 / 60.0);
            let JointAngles::Cone { cone: now, .. } = r.joint_angles(0) else {
                unreachable!()
            };
            widest = widest.max(now);
        }
        assert!(r.joint_gap(0) < 0.5, "gap {}", r.joint_gap(0));
        // Not past the edge by more than the solver's give.
        assert!(widest < cone + 0.12, "{widest} against {cone}");
        let JointAngles::Cone { cone: now, .. } = r.joint_angles(0) else {
            unreachable!()
        };
        assert!(now > cone - 0.12, "{now}: it should hang at the edge");
    }

    #[test]
    fn a_jointed_body_settles_and_sleeps() {
        // Three rods in a chain dropped on the floor lie still and go to
        // sleep, their joints together.
        let bodies: Vec<BodySetup> = (0..3).map(|_| rod(3.0)).collect();
        let joints = vec![ball(1, 0), ball(2, 1)];
        let frames = [
            (I3, [0.0, 0.0, 40.0]),
            (I3, [20.0, 0.0, 40.0]),
            (I3, [40.0, 0.0, 40.0]),
        ];
        let mut r = Ragdoll::new(bodies, joints, &frames);
        let c = floor();
        for _ in 0..1200 {
            r.update(&c, 1.0 / 60.0);
            if r.asleep {
                break;
            }
        }
        assert!(r.asleep, "still moving: {:?}", r.fastest());
        assert!(r.joint_gap(0) < 1.0 && r.joint_gap(1) < 1.0);
        for i in 0..3 {
            let (_, origin) = r.frame(i);
            assert!(origin[2] < 8.0, "body {i} at {origin:?}");
        }
    }
}
