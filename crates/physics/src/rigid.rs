//! Free rigid bodies: the clutter, weapons and props whose models carry a
//! moving Havok body (`nif::RigidBodyInfo`, motion systems other than
//! keyframed and fixed), falling, sliding, tumbling and coming to rest on a
//! [`Collider`] and on each other, pushed by shots, blasts
//! ([`crate::impulses`]) and walkers, as the game hands them to Havok.
//!
//! Read from the game: each body's mass, inertia tensor, centre of mass,
//! linear and angular damping, friction, restitution, most linear and
//! angular speed and its shapes (hulls with their convex radius, boxes,
//! spheres, capsules) from its model; gravity ([`crate::GRAVITY`], the Havok
//! world's); the step: the game steps Havok in fixed steps of
//! `[HAVOK] fMaxTime` (0.016 in `Fallout_default.ini`, exe default 1/60),
//! up to three a frame, carrying the rest over ([`Clock`], `00c66760` with
//! `iUpdateType` 0).
//!
//! This solver's own (labelled where used): contacts are found between a body's corners (and
//! sphere and capsule centres) and the other side's faces, and between the
//! other side's corners and a body's hull, not edge against edge; a
//! surface added without a body's values rubs and bounces as Havok's
//! default body does (friction 0.5, restitution 0.4, `hkpRigidBodyCinfo`'s
//! defaults); walkers push bodies lighter than
//! `fMoveLimitMass` as unstoppable capsules (the character proxy's own
//! impulse, `hkpCharacterProxy`, isn't translated; the game's proxy has
//! infinite strength).
//!
//! Traced here: friction and restitution combine as the square root of
//! the two sides' product, the restitution kept as a byte × 128
//! ([`combined_friction`], [`combined_restitution`], `00cfd800`); the
//! mouse spring ([`Spring`], `00cbb1e0`); the saved velocities
//! ([`RigidWorld::set_velocity`], `00563380`); Havok's step driver,
//! single-body integrator, solver-path forces and velocity checks, and
//! deactivation ([`crate::havok`], B1 PRs 1–4); the broadphase's boxes
//! and pairs and the simulation islands they merge and split
//! ([`crate::islands`], B1 PR 5); the contact manager (`crate::manifold`,
//! PR 6); the constraint solver for contacts (`crate::solver`, PR 7).

use std::collections::{BTreeSet, HashSet};

use crate::ragdoll::{mat_quat, mat_t_vec, mat_vec, quat_mat, Mat3, Quat};
use crate::vec::*;
use crate::{
    closest_on_triangle, segment_segment_closest, segment_triangle_closest, Collider, Surface,
    Vec3, HAVOK_UNIT,
};

pub use crate::havok::{Clock, MAX_STEPS};

/// `[HAVOK] fMaxTime` as `Fallout_default.ini` sets it (the executable's
/// default is 1/60).
pub const STEP: f32 = crate::havok::MAX_TIME;
/// What a surface added without its body's values rubs and bounces like:
/// Havok's default body (`hkpRigidBodyCinfo`: friction 0.5, restitution
/// 0.4; not traced in the executable). The game's clutter models carry
/// these same values (`clutter\junk\ssbottle02.nif`).
pub const DEFAULT_SURFACE: Surface = Surface {
    friction: 0.5,
    restitution: 0.4,
};

/// One solid piece of a body, in its model's space (game units).
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// A convex hull: corners, face planes `n·p + d = 0` (`n` out, unit
    /// length) and Havok's convex radius around it.
    Hull {
        vertices: Vec<Vec3>,
        planes: Vec<[f32; 4]>,
        shell: f32,
    },
    Sphere {
        center: Vec3,
        radius: f32,
    },
    Capsule {
        a: Vec3,
        b: Vec3,
        radius: f32,
    },
    /// Triangles on a moving body: only their corners touch here.
    Mesh {
        vertices: Vec<Vec3>,
        triangles: Vec<[u32; 3]>,
        shell: f32,
    },
}

impl Shape {
    /// A hull with its planes made unit length.
    pub fn hull(vertices: Vec<Vec3>, planes: &[[f32; 4]], shell: f32) -> Shape {
        let planes = planes
            .iter()
            .filter_map(|p| {
                let l = length([p[0], p[1], p[2]]);
                (l > 1e-9).then(|| [p[0] / l, p[1] / l, p[2] / l, p[3] / l])
            })
            .collect();
        Shape::Hull {
            vertices,
            planes,
            shell,
        }
    }

    /// The shape as triangles in its model's space, and how far its
    /// surface stands out from them: for the collider, which walkers and
    /// shots meet.
    pub fn triangles(&self) -> (Vec<Vec3>, Vec<[u32; 3]>, f32) {
        match self {
            Shape::Hull {
                vertices,
                planes,
                shell,
            } => (
                vertices.clone(),
                crate::shapes::hull(vertices, planes),
                *shell,
            ),
            Shape::Sphere { center, radius } => {
                let (v, t) = crate::shapes::sphere(*center, *radius);
                (v, t, 0.0)
            }
            Shape::Capsule { a, b, radius } => {
                let (v, t) = crate::shapes::capsule(*a, *b, *radius);
                (v, t, 0.0)
            }
            Shape::Mesh {
                vertices,
                triangles,
                shell,
            } => (vertices.clone(), triangles.clone(), *shell),
        }
    }

    /// The farthest the shape's surface reaches from `from`.
    fn reach(&self, from: Vec3) -> f32 {
        let far = |v: &[Vec3], extra: f32| {
            v.iter()
                .map(|&p| length(sub(p, from)))
                .fold(0.0f32, f32::max)
                + extra
        };
        match self {
            Shape::Hull {
                vertices, shell, ..
            }
            | Shape::Mesh {
                vertices, shell, ..
            } => far(vertices, *shell),
            Shape::Sphere { center, radius } => far(&[*center], *radius),
            Shape::Capsule { a, b, radius } => far(&[*a, *b], *radius),
        }
    }
}

/// A body as its model gives it, in the model's space (game units; a
/// placed reference's scale already applied to the shapes and the centre).
#[derive(Debug, Clone, PartialEq)]
pub struct RigidSetup {
    /// The placed reference it belongs to: its triangles in the collider
    /// carry this owner.
    pub reference: u32,
    /// Its Havok layer (`nif::collision::layers`).
    pub layer: u8,
    pub mass: f32,
    /// The centre of mass.
    pub center: Vec3,
    /// The inertia tensor about the centre (mass × game units²).
    pub inertia: Mat3,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub friction: f32,
    pub restitution: f32,
    /// Game units a second, radians a second (0: no limit).
    pub max_linear_speed: f32,
    pub max_angular_speed: f32,
    /// Havok's motion system (`nif::RigidBodyInfo::motion`).
    pub motion: u8,
    /// Havok's collision quality (`hkpCollidableQualityType`, as the model
    /// stores it in the cinfo's last byte: `00c8d4a0` names 0 fixed, 1
    /// keyframed, 2 debris, 4 moving, 5 critical, 6 bullet, 7 user; 3 is
    /// debris with simplified TOI, Xbox PDB): [`crate::continuous`].
    pub quality: u8,
    /// The wind pushes it (`nif::collision::BODY_WIND`, [`crate::wind`]).
    pub wind: bool,
    pub shapes: Vec<Shape>,
}

/// A rotation and a translation: model space to the world.
pub type Pose = (Mat3, Vec3);

/// A body in the world.
#[derive(Debug, Clone)]
pub struct Rigid {
    pub setup: RigidSetup,
    /// The inverse inertia in the model's axes, as Havok's box motion keeps
    /// it: 1 ÷ the tensor's diagonal only (`hkpBoxMotion::setInertiaLocal`,
    /// Xbox PDB, `00d1e750`; zero: it doesn't turn).
    inverse_inertia: Vec3,
    inverse_mass: f32,
    /// The centre of mass in the world, the turn, the velocities.
    x: Vec3,
    q: Quat,
    v: Vec3,
    w: Vec3,
    /// Where it was put: the pose its triangles were added to the collider
    /// at ([`RigidWorld::delta`]).
    rest: Pose,
    /// How far its surface reaches from the centre of mass.
    reach: f32,
    /// Its count of passing deactivation checks after its last step
    /// (`crate::havok::Motion::update_deactivation`).
    frames: u32,
    /// At rest: not stepped until something pushes it.
    pub asleep: bool,
    /// Havok's motion: its speed limits, gravity factor and deactivation
    /// state (the position and velocities are the fields above).
    pub motion: crate::havok::Motion,
    /// Moved since it was put (the game's "Havok moved" reference change,
    /// `CHANGE_REFR_HAVOK_MOVE`, named by `0083fef0`).
    pub moved: bool,
    /// Its box in the broadphase (`hkpCollidable::m_boundingVolumeData`,
    /// [`RigidWorld::recalc_aabb`]), kept while it sleeps.
    aabb: (Vec3, Vec3),
    /// Where its step began (centre of mass and rotation): the swept
    /// transform's start (`hkpMotion` `+0x50`, `+0x70`).
    start: (Vec3, Quat),
}

/// The hkUFloat8 index of a model's most speed (Havok units a second, or
/// radians a second): [`crate::havok::ufloat8`], taking the entry below
/// when the value is that entry give or take the rounding of nv-rs's
/// game-unit scaling (1e-5 of it).
pub(crate) fn speed_index(v: f32) -> u8 {
    let i = crate::havok::ufloat8(v);
    if i > 0 {
        let below = crate::havok::UFLOAT8[i as usize - 1];
        if (below - v).abs() <= below * 1e-5 {
            return i - 1;
        }
    }
    i
}

fn mat_mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut m = [[0.0; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    m
}

fn transpose(a: &Mat3) -> Mat3 {
    let mut m = [[0.0; 3]; 3];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = a[j][i];
        }
    }
    m
}

/// The inverse of a 3 × 3 matrix (zero when it has none).
fn invert(m: &Mat3) -> Mat3 {
    let c =
        |r1: usize, c1: usize, r2: usize, c2: usize| m[r1][c1] * m[r2][c2] - m[r1][c2] * m[r2][c1];
    let cof = [
        [c(1, 1, 2, 2), -c(1, 0, 2, 2), c(1, 0, 2, 1)],
        [-c(0, 1, 2, 2), c(0, 0, 2, 2), -c(0, 0, 2, 1)],
        [c(0, 1, 1, 2), -c(0, 0, 1, 2), c(0, 0, 1, 1)],
    ];
    let det = m[0][0] * cof[0][0] + m[0][1] * cof[0][1] + m[0][2] * cof[0][2];
    if det.abs() < 1e-12 {
        return [[0.0; 3]; 3];
    }
    let mut out = [[0.0; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = cof[j][i] / det;
        }
    }
    out
}

impl Rigid {
    fn new(setup: RigidSetup, pose: Pose) -> Rigid {
        let dynamic = crate::impulses::moves(setup.motion) && setup.mass > 0.0;
        let inverse_mass = if dynamic { 1.0 / setup.mass } else { 0.0 };
        // `00d1e750`: 1 ÷ each diagonal element.
        let inverse_inertia = if dynamic {
            let d = |k: usize| {
                let i = setup.inertia[k][k];
                if i > 0.0 {
                    1.0 / i
                } else {
                    0.0
                }
            };
            [d(0), d(1), d(2)]
        } else {
            [0.0; 3]
        };
        let reach = setup
            .shapes
            .iter()
            .map(|s| s.reach(setup.center))
            .fold(0.0f32, f32::max);
        let (r, t) = pose;
        let mut motion = crate::havok::Motion::new(
            crate::havok::motion_type::DYNAMIC,
            add(mat_vec(&r, setup.center), t),
            mat_quat(&r),
        );
        motion.linear_damping = setup.linear_damping;
        motion.angular_damping = setup.angular_damping;
        motion.max_linear_velocity = speed_index(setup.max_linear_speed / HAVOK_UNIT);
        motion.max_angular_velocity = speed_index(setup.max_angular_speed);
        motion.object_radius = reach / HAVOK_UNIT;
        let x = add(mat_vec(&r, setup.center), t);
        let q = mat_quat(&r);
        let mut b = Rigid {
            x,
            q,
            v: [0.0; 3],
            w: [0.0; 3],
            rest: pose,
            reach,
            frames: 0,
            asleep: true,
            moved: false,
            motion,
            inverse_inertia,
            inverse_mass,
            setup,
            aabb: ([0.0; 3], [0.0; 3]),
            start: (x, q),
        };
        b.aabb = b.shape_aabb();
        b
    }

    /// Its shapes' box where it is now, grown by half the collision
    /// tolerance (`hkpShape::getAabb` with the tolerance the box update
    /// passes, `00d1a330`: the collision input's tolerance × 0.5).
    // Translated from 00d1a330 (decompiled, FalloutNV.exe 1.4.0.525)
    fn shape_aabb(&self) -> (Vec3, Vec3) {
        let grow = crate::havok::COLLISION_TOLERANCE * 0.5 * HAVOK_UNIT;
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        let mut points: Vec<(Vec3, f32)> = Vec::new();
        let mut take = |p: Vec3, r: f32| points.push((p, r));
        for shape in &self.setup.shapes {
            match shape {
                Shape::Hull {
                    vertices, shell, ..
                }
                | Shape::Mesh {
                    vertices, shell, ..
                } => {
                    for &v in vertices {
                        take(self.to_world(v), *shell);
                    }
                }
                Shape::Sphere { center, radius } => take(self.to_world(*center), *radius),
                Shape::Capsule { a, b, radius } => {
                    take(self.to_world(*a), *radius);
                    take(self.to_world(*b), *radius);
                }
            }
        }
        if points.is_empty() {
            points.push((self.x, 0.0));
        }
        for (p, r) in points {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k] - r);
                hi[k] = hi[k].max(p[k] + r);
            }
        }
        (sub(lo, [grow; 3]), add(hi, [grow; 3]))
    }

    /// Model space to the world now.
    pub fn pose(&self) -> Pose {
        let r = quat_mat(self.q);
        (r, sub(self.x, mat_vec(&r, self.setup.center)))
    }

    /// Where it was put (its triangles' pose in the collider).
    pub fn rest(&self) -> Pose {
        self.rest
    }

    /// The centre of mass in the world.
    pub fn center(&self) -> Vec3 {
        self.x
    }

    pub fn velocity(&self) -> Vec3 {
        self.v
    }

    pub fn spin(&self) -> Vec3 {
        self.w
    }

    pub fn dynamic(&self) -> bool {
        self.inverse_mass > 0.0
    }

    /// Its Havok motion with the position and velocities as they are now
    /// (game units).
    fn motion_now(&self) -> crate::havok::Motion {
        let mut m = self.motion;
        m.center = self.x;
        m.rotation = self.q;
        m.linear_velocity = self.v;
        m.angular_velocity = self.w;
        m
    }

    fn set_motion(&mut self, m: &crate::havok::Motion) {
        self.motion = *m;
        self.x = m.center;
        self.q = m.rotation;
        self.v = m.linear_velocity;
        self.w = m.angular_velocity;
    }

    /// The inverse inertia in the world applied to `a`.
    fn inverse_inertia_world(&self, a: Vec3) -> Vec3 {
        let r = quat_mat(self.q);
        let l = mat_t_vec(&r, a);
        let ii = self.inverse_inertia;
        mat_vec(&r, [l[0] * ii[0], l[1] * ii[1], l[2] * ii[2]])
    }

    /// Changes its velocities as an impulse `p` (game units) at `arm` would.
    fn kick(&mut self, p: Vec3, arm: Vec3) {
        if self.inverse_mass == 0.0 {
            return;
        }
        let turn = self.inverse_inertia_world(cross(arm, p));
        self.v = add(self.v, scale(p, self.inverse_mass));
        self.w = add(self.w, turn);
    }

    fn point_velocity(&self, arm: Vec3) -> Vec3 {
        add(self.v, cross(self.w, arm))
    }

    /// Model-space point to the world.
    fn to_world(&self, p: Vec3) -> Vec3 {
        let r = quat_mat(self.q);
        add(self.x, mat_vec(&r, sub(p, self.setup.center)))
    }

    /// World point to the model's space.
    fn to_model(&self, p: Vec3) -> Vec3 {
        let r = quat_mat(self.q);
        add(self.setup.center, mat_t_vec(&r, sub(p, self.x)))
    }
}

/// The other side of a contact.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Other {
    /// The collider's triangle (still).
    World(u32),
    Body(usize),
}

/// A touching point before it's solved: on `a` (a model-space point of
/// it), on the other side (a model-space point of the other body, or a
/// world point), the normal pushing `a` away, and the gap they keep.
#[derive(Debug, Clone, Copy)]
struct Touch {
    on_a: Vec3,
    on_b: Vec3,
    n: Vec3,
    margin: f32,
    surface: Surface,
    /// The two shapes' features it came from (this generator's point id,
    /// [`feature_key`]).
    key: u64,
    /// The other side's share of the margin (its shell or radius): its
    /// surface is `on_b` + `n` × this.
    radius_b: f32,
}

/// A contact point's identity: what kind of touch, which shapes, which
/// feature (corner, edge end) and which triangle.
fn feature_key(kind: u64, shapes: usize, feature: usize, triangle: u32) -> u64 {
    kind << 56
        | (shapes as u64 & 0xff) << 48
        | (feature as u64 & 0xffff) << 32
        | u64::from(triangle)
}

/// The other side of a manifold: another body, or the world's fixed body
/// a collider triangle belongs to (its placed reference, 0 for the
/// place's landscape and unowned statics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Partner {
    Body(usize),
    Fixed(u32),
}

/// A contact point the narrowphase found this step, before the contact
/// manager takes it.
#[derive(Debug, Clone, Copy)]
struct Candidate {
    point: crate::manifold::NewPoint,
    surface: Surface,
    triangle: Option<u32>,
}

/// Havok's mouse spring (`hkpMouseSpringAction`, the player's Z-key grab,
/// `crate::grab`): pulls a point of a body toward a point in the world.
/// The fields are the action's (Havok's names; `+0x20`…`+0x4c` in the
/// object `00cbb1e0` reads).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub body: usize,
    /// `m_positionInRbLocal`: the held point, in the body's model space.
    pub local: Vec3,
    /// `m_mousePositionInWorld` (game units).
    pub target: Vec3,
    /// `m_springDamping`, `m_springElasticity`.
    pub damping: f32,
    pub elasticity: f32,
    /// `m_maxRelativeForce`: the most force per unit mass (Havok units a
    /// second²).
    pub max_relative_force: f32,
    /// `m_objectDamping`: the body's velocities are multiplied by this
    /// every step.
    pub object_damping: f32,
}

/// A body coming into contact with something (Havok's "contact point
/// added", which the game's collision listener `00623cb0` hears for its
/// impact sounds and physics damage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContactEvent {
    pub body: usize,
    /// The other body, or the collider's triangle it met.
    pub other_body: Option<usize>,
    pub triangle: Option<u32>,
    /// Where (game units).
    pub point: Vec3,
    /// How fast they closed along the contact's normal (game units a
    /// second; Havok's projected velocity × 7).
    pub speed: f32,
}

/// Free rigid bodies.
#[derive(Debug, Clone, Default)]
pub struct RigidWorld {
    pub bodies: Vec<Rigid>,
    pub clock: Clock,
    /// The world's solver settings (`hkpSolverInfo`), rescaled each frame.
    pub solver: crate::havok::SolverInfo,
    /// The player's grab, if they hold something.
    pub spring: Option<Spring>,
    /// Each agent's contact points (its contact manager's atom,
    /// [crate::manifold]): the first body, the other side.
    pub manifolds: std::collections::BTreeMap<(usize, Partner), crate::manifold::Manifold>,
    /// Contact points added since [`RigidWorld::take_contacts`].
    began: Vec<ContactEvent>,
    /// The simulation islands (`crate::islands`).
    pub islands: crate::islands::Islands,
    /// The broadphase's pairs of bodies with an agent (lower index first):
    /// their boxes overlap and the collision filter lets them meet.
    pairs: BTreeSet<(usize, usize)>,
}

impl RigidWorld {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a body at `pose`, asleep, and gives its index: in an inactive
    /// island of its own (a fixed body in none), its box put in the
    /// broadphase, which pairs it with the bodies it overlaps (merging
    /// their islands: Bethesda adds a place's bodies in a batch with
    /// activation 0, `00c674d0` → `hkpWorld::addEntityBatch` `00c94bd0`).
    pub fn add(&mut self, setup: RigidSetup, pose: Pose) -> usize {
        let body = Rigid::new(setup, pose);
        let fixed = !body.dynamic();
        self.bodies.push(body);
        let i = self.islands.add_body(fixed, false);
        debug_assert_eq!(i, self.bodies.len() - 1);
        self.update_pairs(&[i]);
        i
    }

    /// Whether the broadphase pairs bodies `a` and `b`: their boxes
    /// overlap, they aren't both fixed (the quality table gives fixed
    /// against fixed no agent, `00d10800`) and the collision filter
    /// (`00c84740`, `crate::layers`) lets their layers meet.
    fn pairs_with(&self, a: usize, b: usize) -> bool {
        let (x, y) = (&self.bodies[a], &self.bodies[b]);
        if !x.dynamic() && !y.dynamic() {
            return false;
        }
        let ((alo, ahi), (blo, bhi)) = (x.aabb, y.aabb);
        let overlap = (0..3).all(|k| alo[k] <= bhi[k] && blo[k] <= ahi[k]);
        overlap && crate::layers::Filter::shared().layers_touch(x.setup.layer, y.setup.layer)
    }

    /// The broadphase's update for bodies `moved` (their boxes recalculated):
    /// pairs that begin get an agent, merging the two bodies' islands
    /// (`00cc0f40` → `00cb5570`; an active island wakes the other); pairs
    /// that end lose theirs, an island holding both asking for a split
    /// check (`00cc10b0`).
    // Translated from 00cf9c60, 00d10800, 00d10860, 00cc0f40 and 00cc10b0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn update_pairs(&mut self, moved: &[usize]) {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut seen = HashSet::new();
        for &a in moved {
            for b in 0..self.bodies.len() {
                if b == a {
                    continue;
                }
                let key = (a.min(b), a.max(b));
                if !seen.insert(key) {
                    continue;
                }
                let now = self.pairs_with(a, b);
                let was = self.pairs.contains(&key);
                if now && !was {
                    added.push(key);
                } else if was && !now {
                    removed.push(key);
                }
            }
        }
        for key in removed {
            self.pairs.remove(&key);
            self.islands.agent_removed(key.0, key.1);
        }
        for key in added {
            self.pairs.insert(key);
            let woken = self.islands.merge(key.0, key.1);
            self.woken(&woken);
        }
    }

    /// Bodies whose island was just woken (`00cb5100`): stepped again,
    /// their deactivation counts started again.
    fn woken(&mut self, bodies: &[usize]) {
        let solver = self.solver;
        for &i in bodies {
            let b = &mut self.bodies[i];
            b.asleep = false;
            b.motion.activate(&solver);
        }
    }

    /// The box of body `i` for the broadphase after a step
    /// (`hkpEntityAabbUtil::entityBatchRecalcAabb` `00d1a330`): its
    /// shapes' box ([`Rigid::shape_aabb`]) grown toward where the step
    /// began, by the turn's sweep (the step's angle × the object radius,
    /// held within the bounding sphere grown by half the tolerance) and by
    /// the centre's travel back to the step's start.
    // Translated from 00d1a330 (decompiled, FalloutNV.exe 1.4.0.525)
    fn recalc_aabb(&mut self, i: usize) {
        let b = &self.bodies[i];
        let (mut lo, mut hi) = b.shape_aabb();
        let (c0, q0) = b.start;
        let c1 = b.x;
        let cos = (q0[0] * b.q[0] + q0[1] * b.q[1] + q0[2] * b.q[2] + q0[3] * b.q[3]).abs();
        let angle = 2.0 * cos.min(1.0).acos();
        let radius = b.motion.object_radius * HAVOK_UNIT;
        let sweep = angle * radius;
        let sphere = radius + crate::havok::COLLISION_TOLERANCE * 0.5 * HAVOK_UNIT;
        let back = sub(c0, c1);
        for k in 0..3 {
            hi[k] = hi[k].max((hi[k] + sweep).min(c1[k] + sphere));
            lo[k] = lo[k].min((lo[k] - sweep).max(c1[k] - sphere));
            hi[k] += back[k].max(0.0);
            lo[k] += back[k].min(0.0);
        }
        self.bodies[i].aabb = (lo, hi);
    }

    /// The bodies of the active islands.
    fn active_bodies(&self) -> Vec<usize> {
        (0..self.bodies.len())
            .filter(|&i| self.islands.is_active(i))
            .collect()
    }

    /// The body of a reference.
    pub fn find(&self, reference: u32) -> Option<usize> {
        self.bodies
            .iter()
            .position(|b| b.setup.reference == reference)
    }

    /// Takes a reference's body out, giving it back.
    pub fn remove(&mut self, reference: u32) -> Option<Rigid> {
        let i = self.find(reference)?;
        Some(self.remove_at(i))
    }

    /// Sets a body's velocities (game units and radians a second) and
    /// wakes it: a moving body coming back from a save (`00563380` sets
    /// both, then activates the body).
    pub fn set_velocity(&mut self, body: usize, linear: Vec3, angular: Vec3) {
        let b = &mut self.bodies[body];
        if !b.dynamic() {
            return;
        }
        b.v = linear;
        b.w = angular;
        self.wake(body);
    }

    /// The contacts begun since the last call (each pair once when it
    /// starts touching; resting pairs don't repeat).
    pub fn take_contacts(&mut self) -> Vec<ContactEvent> {
        std::mem::take(&mut self.began)
    }

    /// The heaviest other body body `i` touches now (0: none; the world's
    /// fixed bodies have no mass here, as Havok's fixed bodies report 0).
    pub fn heaviest_contact(&self, i: usize) -> f32 {
        let mut heaviest = 0.0f32;
        for j in 0..self.bodies.len() {
            if j == i || !self.bodies[j].dynamic() {
                continue;
            }
            let gap = length(sub(self.bodies[i].x, self.bodies[j].x));
            if gap > self.bodies[i].reach + self.bodies[j].reach + 2.0 {
                continue;
            }
            if !self.body_touches(i, j, 0.0).is_empty() {
                heaviest = heaviest.max(self.bodies[j].setup.mass);
            }
        }
        heaviest
    }

    /// Takes body `i` out (a grab on it ends).
    pub fn remove_at(&mut self, i: usize) -> Rigid {
        // Pairs are kept by index: start them afresh.
        self.began.clear();
        self.manifolds = std::mem::take(&mut self.manifolds)
            .into_iter()
            .filter(|((a, p), _)| *a != i && *p != Partner::Body(i))
            .map(|((a, p), m)| {
                let a = a - usize::from(a > i);
                let p = match p {
                    Partner::Body(b) => Partner::Body(b - usize::from(b > i)),
                    f => f,
                };
                ((a, p), m)
            })
            .collect();
        // Its agents go with it (`hkpWorld::removeEntity`), the rest keep
        // theirs, renumbered; its island asks for a split check.
        self.pairs = std::mem::take(&mut self.pairs)
            .into_iter()
            .filter(|&(a, b)| a != i && b != i)
            .map(|(a, b)| (a - usize::from(a > i), b - usize::from(b > i)))
            .collect();
        self.islands.remove_body(i);
        if let Some(s) = &mut self.spring {
            if s.body == i {
                self.spring = None;
            } else if s.body > i {
                s.body -= 1;
            }
        }
        self.bodies.remove(i)
    }

    /// Where a body's model-space point is now.
    pub fn point(&self, body: usize, local: Vec3) -> Vec3 {
        self.bodies[body].to_world(local)
    }

    /// A world point as a point of a body's model space.
    pub fn local_point(&self, body: usize, world: Vec3) -> Vec3 {
        self.bodies[body].to_model(world)
    }

    /// `hkpMouseSpringAction::applyAction` (Havok's, named by the Xbox
    /// PDB): the body's velocities × the object damping; then the impulse
    /// at the held point that would take away the spring damping × the
    /// point's velocity plus the elasticity × the distance to the target ÷
    /// the step, through the body's inverse mass matrix at that point;
    /// at most the step × the mass × the most relative force; applied at
    /// the point. Nothing when that matrix has no inverse.
    // Translated from 00cbb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn apply_spring(&mut self, s: &Spring, dt: f32) {
        let b = &mut self.bodies[s.body];
        if !b.dynamic() || dt <= 0.0 {
            return;
        }
        let p = b.to_world(s.local);
        let err = sub(p, s.target);
        let r = sub(p, b.x);
        // K: an impulse j at the point changes its velocity by K j.
        let k_of = |b: &Rigid, j: Vec3| {
            add(
                scale(j, b.inverse_mass),
                cross(b.inverse_inertia_world(cross(r, j)), r),
            )
        };
        let cols = [
            k_of(b, [1.0, 0.0, 0.0]),
            k_of(b, [0.0, 1.0, 0.0]),
            k_of(b, [0.0, 0.0, 1.0]),
        ];
        let k: Mat3 = [
            [cols[0][0], cols[1][0], cols[2][0]],
            [cols[0][1], cols[1][1], cols[2][1]],
            [cols[0][2], cols[1][2], cols[2][2]],
        ];
        let k_inv = invert(&k);
        if k_inv == [[0.0; 3]; 3] {
            return;
        }
        b.v = scale(b.v, s.object_damping);
        b.w = scale(b.w, s.object_damping);
        let v_point = b.point_velocity(r);
        let want = add(scale(v_point, s.damping), scale(err, s.elasticity / dt));
        let mut j = scale(mat_vec(&k_inv, want), -1.0);
        // The limit in game units: Havok's force per mass is in Havok
        // units a second².
        let limit = dt * s.max_relative_force * HAVOK_UNIT / b.inverse_mass;
        let l = length(j);
        if l > limit {
            j = scale(j, limit / l);
        }
        b.kick(j, r);
        b.moved = true;
    }

    /// Puts a body at a pose, still and asleep (a moved reference coming
    /// back where it was left).
    pub fn place(&mut self, body: usize, pose: Pose, moved: bool) {
        let b = &mut self.bodies[body];
        let (r, t) = pose;
        b.x = add(mat_vec(&r, b.setup.center), t);
        b.q = mat_quat(&r);
        b.v = [0.0; 3];
        b.w = [0.0; 3];
        b.moved = moved;
        b.start = (b.x, b.q);
        // Moving a body doesn't activate it (Bethesda's world cinfo turns
        // `m_shouldActivateOnRigidBodyTransformChange` off, `00c681c0`);
        // its box moves in the broadphase.
        b.asleep = !self.islands.is_active(body);
        self.recalc_aabb(body);
        self.update_pairs(&[body]);
    }

    /// How a body has moved since it was put: the rotation and translation
    /// that take its rest pose to where it is (for
    /// [`Collider::move_owner`] and the drawing).
    pub fn delta(&self, body: usize) -> Pose {
        let b = &self.bodies[body];
        let (r, t) = b.pose();
        let (r0, t0) = b.rest;
        let rd = mat_mul(&r, &transpose(&r0));
        (rd, sub(t, mat_vec(&rd, t0)))
    }

    /// The character proxy's push on the dynamic bodies it touches
    /// (`applySurfaceInteractions`, `00cacf80`, `hkpCharacterProxy`; the
    /// manifold's touches as the proxy recorded them,
    /// [`crate::proxy::SurfaceContact`]), applied in order. A touch on a
    /// body that moves (not fixed or keyframed, motion type 4 and 5 skipped)
    /// gets: with `v` the body's velocity at the point minus the proxy's,
    /// along the normal `n` (toward the character), `f = −0.9 v`, plus
    /// `0.4 ÷ dt` × the distance when that is negative (a penetration); when
    /// `f < 0`, the impulse `f ÷ k` along `n`, `k` the body's inverse mass
    /// plus `(r × n)·I⁻¹(r × n)` at the arm `r` from its centre of mass, not
    /// below `−strength × dt`; then the character's weight, `mass × (dt × g·n
    /// − v` when `v < 0`) along `n` when below −1.19e-7. The body is woken
    /// (`00c9c1d0`) and the impulse applied at the point whatever its size.
    /// Not translated: the listener callback for bodies with property
    /// 0x1300 (`00c6ca30`, moving platforms; the character listener's
    /// other, `+0x14`, does nothing).
    // Translated from 00cacf80 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn apply_surface_interactions(
        &mut self,
        contacts: &[crate::proxy::SurfaceContact],
        gravity: Vec3,
    ) {
        let u = HAVOK_UNIT;
        for c in contacts {
            let Some(i) = self.find(c.reference) else {
                continue;
            };
            let b = &self.bodies[i];
            if !b.dynamic() {
                continue;
            }
            let n = c.normal;
            // The point's arm and the body's velocity there, Havok units.
            let arm = scale(sub(c.position, b.x), 1.0 / u);
            let at = scale(b.point_velocity(sub(c.position, b.x)), 1.0 / u);
            let v = dot(sub(at, c.velocity), n);
            let mut f = -v * 0.9;
            if c.distance < 0.0 {
                f += c.distance * (1.0 / c.dt) * 0.4;
            }
            let mut impulse = [0.0; 3];
            if f < 0.0 {
                let rn = cross(arm, n);
                // The stored inverse inertia is per game unit squared:
                // times the unit squared it is Havok's.
                let k = b.inverse_mass + u * u * dot(rn, b.inverse_inertia_world(rn));
                let j = (f / k).max(-c.strength * c.dt);
                impulse = scale(n, j);
            }
            let mut w = c.dt * dot(gravity, n);
            if v < 0.0 {
                w -= v;
            }
            if w < -1.192_092_9e-7 {
                impulse = add(impulse, scale(n, w * c.mass));
            }
            self.apply_point_impulse(i, impulse, c.position);
        }
    }

    /// Wakes a body (`hkpEntity::activate`, Xbox PDB): a sleeping one's
    /// island is activated with all its bodies (`00cb5100`: their counts
    /// start again; Havok marks it active and wakes it at the next step's
    /// start, here at once); an awake one's island marked inactive is
    /// marked active again, so it stays awake.
    pub fn wake(&mut self, body: usize) {
        let Some(id) = self.islands.island_of(body) else {
            return;
        };
        if self.islands.is_active(body) {
            self.islands.mark_active(id);
        } else {
            let woken = self.islands.activate(id);
            self.woken(&woken);
        }
    }

    /// `hkpRigidBody::applyPointImpulse`: an impulse in Havok units (mass ×
    /// Havok units a second) at a world point.
    pub fn apply_point_impulse(&mut self, body: usize, impulse: Vec3, point: Vec3) {
        self.wake(body);
        let b = &mut self.bodies[body];
        let arm = sub(point, b.x);
        b.kick(scale(impulse, HAVOK_UNIT), arm);
        b.moved = true;
    }

    /// `hkpMotion::applyForce(deltaTime, force)` (Havok's, Xbox PDB slot
    /// `+0x5c`) after waking the body (`00c9c1d0`): its velocity gains
    /// force ÷ mass × `dt` (a force in Havok units).
    pub fn apply_force(&mut self, body: usize, force: Vec3, dt: f32) {
        if !self.bodies[body].dynamic() {
            return;
        }
        self.wake(body);
        let b = &mut self.bodies[body];
        b.kick(scale(force, HAVOK_UNIT * dt), [0.0; 3]);
        b.moved = true;
    }

    /// `hkpRigidBody::applyLinearImpulse`: through the centre of mass.
    pub fn apply_linear_impulse(&mut self, body: usize, impulse: Vec3) {
        self.wake(body);
        let b = &mut self.bodies[body];
        b.kick(scale(impulse, HAVOK_UNIT), [0.0; 3]);
        b.moved = true;
    }

    /// `hkpRigidBody::applyAngularImpulse`: Havok units (mass × Havok
    /// units² a second).
    pub fn apply_angular_impulse(&mut self, body: usize, impulse: Vec3) {
        self.wake(body);
        let b = &mut self.bodies[body];
        if b.inverse_mass == 0.0 {
            return;
        }
        let turn = b.inverse_inertia_world(scale(impulse, HAVOK_UNIT * HAVOK_UNIT));
        b.w = add(b.w, turn);
        b.moved = true;
    }

    /// Whether any body is awake.
    pub fn awake(&self) -> bool {
        self.bodies.iter().any(|b| !b.asleep)
    }

    /// Moves on by a frame of `frame` seconds, in the game's steps
    /// ([`Clock`]); the steps taken.
    ///
    /// As `bhkWorld::Update` (`00c6ae70`) runs a frame with a frame time
    /// over 0: the solver settings scaled to the step (`00c66a00`), then
    /// `hkpWorld::stepDeltaTime` for each whole step to the frame marker
    /// (see [`Clock`]). The wind listener (`00c66e20`) runs after, with the
    /// frame's time (the viewer's `clutter`).
    // Translated from 00c6ae70 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(&mut self, collider: &Collider, frame: f32) -> u32 {
        let (n, _) = self.clock.advance(frame);
        if frame <= 0.0 {
            return 0;
        }
        let step = self.clock.step();
        self.solver.scale(step, crate::havok::TAU_RATIO);
        for _ in 0..n {
            self.step(collider, step);
        }
        n
    }

    /// One step of `dt` seconds (`hkpSimulation::stepDeltaTime` `00cf8730`:
    /// integrate, collide, advance time; here the integration and this
    /// solver's contacts together).
    pub fn step(&mut self, collider: &Collider, dt: f32) {
        // `hkpSimulation::integrate` (`00cf9340`): the world's maintenance
        // first (`00d0b280`): the islands that asked are split into their
        // connected parts, bodies joined by an agent (a broadphase pair; no
        // constraints or many-body actions here).
        let pairs = &self.pairs;
        self.islands
            .split(|a, b| pairs.contains(&(a.min(b), a.max(b))));
        // `hkpSimulation::integrateInternal` (`00cf8da0`) then cleans up
        // the dirty islands (`00cb55d0`): one marked inactive at the last
        // step goes to sleep when every body passes the last test
        // (`00cb5310`), its velocities zeroed; otherwise it stays awake.
        let bodies = &self.bodies;
        let cleanup = self.islands.cleanup(|members| {
            members
                .iter()
                .all(|&i| bodies[i].motion_now().can_deactivate(HAVOK_UNIT))
        });
        for &i in &cleanup.deactivated {
            let b = &mut self.bodies[i];
            let mut m = b.motion_now();
            m.deactivate();
            b.set_motion(&m);
            b.asleep = true;
        }
        self.woken(&cleanup.activated);
        // The grab's spring acts first (an action, before Havok solves).
        if let Some(s) = self.spring {
            if s.body < self.bodies.len() && self.bodies[s.body].dynamic() {
                self.wake(s.body);
                self.apply_spring(&s, dt);
            }
        }
        // Actions (the spring, above), then the deactivation flags, then
        // the islands.
        self.solver.increment_deactivation_flags();
        if !self.awake() || dt <= 0.0 {
            return;
        }
        // Where each moving body's step begins (its swept transform's
        // start), for its box afterwards.
        for b in &mut self.bodies {
            if !b.asleep {
                b.start = (b.x, b.q);
            }
        }
        // Each active island, last to first (`00cf8da0`): without contact
        // points (no constraint in it) each body is stepped alone by
        // `hkRigidMotionUtilApplyForcesAndStep` (`00d28a30`); with some,
        // the solver steps the island ([`RigidWorld::solve_island`]).
        let gravity_step = scale(crate::havok::GRAVITY, dt * HAVOK_UNIT);
        let solver = self.solver;
        for id in self.islands.active().into_iter().rev() {
            let Some(island) = self.islands.island(id) else {
                continue;
            };
            let members = island.bodies.clone();
            let constrained = self.manifolds.iter().any(|(&(a, p), m)| {
                !m.is_empty()
                    && (members.contains(&a)
                        || matches!(p, Partner::Body(b) if members.contains(&b)))
            });
            if constrained {
                self.solve_island(&members, dt);
            } else {
                for &i in &members {
                    let b = &mut self.bodies[i];
                    let mut m = b.motion_now();
                    b.frames = m.step(dt, gravity_step, HAVOK_UNIT, &solver).unwrap_or(0);
                    b.set_motion(&m);
                }
            }
        } // Each active island's fewest passing checks (each body's count
          // was updated with its step): more than 5 and still marked active,
          // it is marked inactive (`00cf8da0` → `00cb5420`) and goes to sleep
          // at the next step's start, above.
        for b in &mut self.bodies {
            if !b.asleep && b.dynamic() {
                b.moved |= b.v != [0.0; 3] || b.w != [0.0; 3];
            }
        }
        for id in self.islands.active() {
            let Some(island) = self.islands.island(id) else {
                continue;
            };
            let fewest = island
                .bodies
                .iter()
                .map(|&i| self.bodies[i].frames)
                .min()
                .unwrap_or(0);
            if fewest > crate::havok::INACTIVE_FRAMES_TO_DEACTIVATE
                && island.active_mark
                && crate::havok::WANT_DEACTIVATION
            {
                self.islands.mark_inactive(id);
            }
        }
        self.continuous(collider);
        self.collide_broadphase();
        self.collide_narrowphase(collider);
    }

    /// `hkpConstraintSolverSetup::solve` (`00d88b50`) for an island with
    /// contact points (`crate::solver`, in Havok units about the first
    /// body's centre): the accumulators, the world's fixed body first
    /// (`00d29830`, which damps the motions' velocities); for each contact
    /// constraint the callbacks for its new points (`00d92900`) and its
    /// Jacobians (`00d72190`); the solver (`00d8d030`); the export
    /// (`00def570`) into the points' properties and the friction's data;
    /// the accumulators applied to the motions (`00d29bf0`), each body's
    /// count of passing checks kept. Walkers pushing a body are this
    /// solver's: keyframed bodies at their velocity, their touches new
    /// points every step.
    // Translated from 00d88b50 (decompiled, FalloutNV.exe 1.4.0.525)
    fn solve_island(&mut self, members: &[usize], dt: f32) {
        use crate::manifold::Manifold;
        use crate::solver::*;
        let u = HAVOK_UNIT;
        let solver = self.solver;
        let origin = self.bodies[members[0]].x;
        let hv = |p: Vec3| scale(sub(p, origin), 1.0 / u);
        let cols = |q: Quat| {
            let r = quat_mat(q);
            [
                [r[0][0], r[1][0], r[2][0]],
                [r[0][1], r[1][1], r[2][1]],
                [r[0][2], r[1][2], r[2][2]],
            ]
        };
        let mut accs = vec![Accumulator::fixed()];
        let mut spin = vec![[0.0f32; 3]];
        let mut acc_of = std::collections::HashMap::new();
        for &i in members {
            let b = &mut self.bodies[i];
            let mut m = b.motion_now();
            m.apply_damping(dt);
            b.set_motion(&m);
            acc_of.insert(i, accs.len());
            accs.push(Accumulator::dynamic(
                hv(b.x),
                cols(b.q),
                scale(b.v, 1.0 / u),
                b.w,
                scale(b.inverse_inertia, u * u),
                b.inverse_mass,
                usize::from(b.motion.deactivation_class),
                crate::havok::from_half(b.motion.gravity_factor),
            ));
            spin.push(b.w);
        }
        // The island's contact constraints, Havok units.
        let mut keys = Vec::new();
        let mut copies: Vec<Manifold> = Vec::new();
        let mut sides = Vec::new();
        for (key, m) in &self.manifolds {
            let Some(&ia) = acc_of.get(&key.0) else {
                continue;
            };
            let ib = match key.1 {
                Partner::Body(j) => match acc_of.get(&j) {
                    Some(&ib) => ib,
                    None => continue,
                },
                Partner::Fixed(_) => 0,
            };
            if m.is_empty() {
                continue;
            }
            let mut c = m.clone();
            for p in &mut c.points {
                p.position = hv(p.position);
                p.distance /= u;
            }
            keys.push(*key);
            copies.push(c);
            sides.push((ia, ib));
        }
        let held = copies.len();
        let step = Step::new(&solver, dt, crate::havok::GRAVITY);
        let q = QueryIn::new(&solver, &step);
        let mut schemas = Vec::new();
        for (ci, (c, &(ia, ib))) in copies.iter_mut().zip(&sides).enumerate() {
            let velocity = |k: usize| BodyVelocity {
                linear: accs[k].linear,
                angular: spin[k],
                center: accs[k].center,
            };
            fire_callbacks(
                c,
                &q,
                (accs[ia].inv_mass, accs[ib].inv_mass),
                &velocity(ia),
                &velocity(ib),
                solver.contact_resting_velocity,
            );
            build_contact_jacobians(c, ci, &q, (ia, ib), &accs[ia], &accs[ib], &mut schemas);
        }
        let results = solve(&solver, &step, &schemas, &mut accs);
        {
            let mut refs: Vec<&mut Manifold> = copies.iter_mut().collect();
            export(&solver, &step, &schemas, &results, &accs, &mut refs);
        }
        for (key, c) in keys.iter().zip(&copies[..held]) {
            if let Some(m) = self.manifolds.get_mut(key) {
                for (p, cp) in m.points.iter_mut().zip(&c.points) {
                    p.props = cp.props;
                }
                m.info = c.info;
            }
        }
        for &i in members {
            let acc = accs[acc_of[&i]];
            let b = &mut self.bodies[i];
            let c = cols(b.q);
            let mut m = b.motion_now();
            b.frames = m
                .apply_accumulator(
                    scale(acc.linear, u),
                    apply_cols(&c, acc.angular),
                    scale(acc.sum_linear, u),
                    apply_cols(&c, acc.sum_angular),
                    dt,
                    u,
                    &solver,
                )
                .unwrap_or(0);
            b.set_motion(&m);
        }
    }
    /// The continuous part of `hkpContinuousSimulation::collideInternal`
    /// (`00d0ef40`) and `handleSimpleToi` (`00d0e210`) for the active
    /// bodies whose quality gets a simplified time of impact against the
    /// world ([`crate::continuous`]): a body the step carried through a
    /// triangle is set to its pose at the time it touched it (centre,
    /// turn: `00cf1d50`), its velocities as they were; the contacts there
    /// are the collide's and the next step's. At most the world's queue
    /// of events ([`crate::continuous::TOI_QUEUE`]).
    // Translated from 00d0e210 and 00cf1d50 (decompiled, FalloutNV.exe 1.4.0.525)
    fn continuous(&mut self, collider: &Collider) {
        let owners: HashSet<u32> = self.bodies.iter().map(|b| b.setup.reference).collect();
        let mut events = Vec::new();
        for i in self.active_bodies() {
            let b = &self.bodies[i];
            if !b.dynamic() || !crate::continuous::simple_toi_against_world(b.setup.quality) {
                continue;
            }
            // Every moving step is looked at, not only one that carried the
            // centre past the shape's reach: a thin shape that turns as it
            // falls can end a step with a corner a few units behind the
            // floor, which the PSI finds but accepts as allowed penetration
            // (a new point's distance), so it falls through (the
            // ophthalmoscope by Doc Mitchell's bed, 2 units thick). The
            // game's agents decide by a predictive cast against fractions
            // of the body's allowed penetration depth (not traced);
            // `first_touch` keeps to points whose centres ended behind a
            // triangle, so a body resting on one isn't stopped.
            if b.x == b.start.0 && b.q == b.start.1 {
                continue;
            }
            let margin = b.reach + crate::havok::COLLISION_TOLERANCE * HAVOK_UNIT;
            let lo = sub(
                [
                    b.start.0[0].min(b.x[0]),
                    b.start.0[1].min(b.x[1]),
                    b.start.0[2].min(b.x[2]),
                ],
                [margin; 3],
            );
            let hi = add(
                [
                    b.start.0[0].max(b.x[0]),
                    b.start.0[1].max(b.x[1]),
                    b.start.0[2].max(b.x[2]),
                ],
                [margin; 3],
            );
            let tris: Vec<u32> = collider
                .near(lo, hi)
                .into_iter()
                .filter(|&t| !owners.contains(&collider.owner(t)))
                .collect();
            let points: Vec<(Vec3, f32)> = b
                .setup
                .shapes
                .iter()
                .flat_map(feature_points)
                .map(|(p, r)| (sub(p, b.setup.center), r))
                .collect();
            if let Some(t) =
                crate::continuous::first_touch(collider, &tris, b.start, (b.x, b.q), &points)
            {
                events.push((t, i));
            }
        }
        // In time order (`00d0e9c0`: the earliest first; ties, the lower
        // body); the queue holds so many.
        events.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        events.truncate(crate::continuous::TOI_QUEUE);
        for (t, i) in events {
            let b = &mut self.bodies[i];
            let (x, q) = crate::continuous::swept_pose(b.start, (b.x, b.q), t);
            b.x = x;
            b.q = q;
            b.moved = true;
        }
    }

    /// The broadphase part of `hkpSimulation::collide` (`00cf8bc0`; the
    /// continuous simulation's `00d0d3f0` per island): the moving bodies'
    /// boxes recalculated (`00d1a330`), then the pairs updated
    /// ([`RigidWorld::update_pairs`]).
    fn collide_broadphase(&mut self) {
        let moved = self.active_bodies();
        for &i in &moved {
            self.recalc_aabb(i);
        }
        self.update_pairs(&moved);
    }

    /// The narrowphase part of `hkpSimulation::collide` (`00cf8bc0`, per
    /// active island's agents): for each agent with a body in an active
    /// island, the contact points its bodies have now (this generator's,
    /// within the collision tolerance: Havok's agents create and keep
    /// points up to 0.1 Havok units apart, `00cfb570`), handed to the
    /// contact manager ([`RigidWorld::update_manifold`]). The world's
    /// triangles pair with every body: one agent per fixed body (here per
    /// placed reference). Sleeping islands' points stay as they were.
    // Translated from 00cf8bc0 and 00cfb570 (decompiled, FalloutNV.exe 1.4.0.525)
    fn collide_narrowphase(&mut self, collider: &Collider) {
        let tol = crate::havok::COLLISION_TOLERANCE * HAVOK_UNIT;
        let owners: HashSet<u32> = self.bodies.iter().map(|b| b.setup.reference).collect();
        let active = self.active_bodies();
        for &i in &active {
            if !self.bodies[i].dynamic() {
                continue;
            }
            // The world's fixed bodies.
            let (lo, hi) = self.bodies[i].aabb;
            let (lo, hi) = (sub(lo, [tol; 3]), add(hi, [tol; 3]));
            let tris: Vec<u32> = collider
                .near(lo, hi)
                .into_iter()
                .filter(|&t| !owners.contains(&collider.owner(t)))
                .filter(|&t| {
                    let [p, q, s] = collider.triangle(t);
                    p[2].max(q[2]).max(s[2]) >= lo[2] && p[2].min(q[2]).min(s[2]) <= hi[2]
                })
                .collect();
            let mut groups: std::collections::BTreeMap<u32, Vec<Candidate>> = Default::default();
            for (t, touch) in self.world_touches(collider, i, &tris, tol) {
                let fixed = collider.reference(t);
                let point = self.new_point(i, Partner::Fixed(fixed), &touch);
                groups.entry(fixed).or_default().push(Candidate {
                    point,
                    surface: touch.surface,
                    triangle: Some(t),
                });
            }
            let old: Vec<u32> = self
                .manifolds
                .keys()
                .filter_map(|&(a, p)| match p {
                    Partner::Fixed(r) if a == i && !groups.contains_key(&r) => Some(r),
                    _ => None,
                })
                .collect();
            for r in old {
                self.manifolds.remove(&(i, Partner::Fixed(r)));
            }
            for (r, found) in groups {
                self.update_manifold(i, Partner::Fixed(r), found);
            }
        }
        // Agents between bodies: each pair once, the lower index first.
        let pairs: Vec<(usize, usize)> = self
            .pairs
            .iter()
            .copied()
            .filter(|&(a, b)| self.islands.is_active(a) || self.islands.is_active(b))
            .collect();
        for (a, b) in pairs {
            let found: Vec<Candidate> = self
                .body_touches(a, b, tol)
                .into_iter()
                .map(|touch| Candidate {
                    point: self.new_point(a, Partner::Body(b), &touch),
                    surface: touch.surface,
                    triangle: None,
                })
                .collect();
            self.update_manifold(a, Partner::Body(b), found);
        }
        // Agents gone with their pairs (`00cc10b0` removes their points).
        let pairs = &self.pairs;
        self.manifolds.retain(|&(a, p), _| match p {
            Partner::Body(b) => pairs.contains(&(a.min(b), a.max(b))),
            Partner::Fixed(_) => true,
        });
    }

    /// A touch as a contact point: on the other side's surface, the normal
    /// from it toward body `a`, the distance between the surfaces.
    fn new_point(&self, a: usize, other: Partner, t: &Touch) -> crate::manifold::NewPoint {
        let o = match other {
            Partner::Body(j) => Other::Body(j),
            Partner::Fixed(_) => Other::World(0),
        };
        let (pa, pb, _, _) = self.ends(a, o, t);
        crate::manifold::NewPoint {
            key: t.key,
            position: add(pb, scale(t.n, t.radius_b)),
            normal: t.n,
            distance: dot(sub(pa, pb), t.n) - t.margin,
        }
    }

    /// The contact manager's update for one agent (`hkpSimpleConstraint
    /// ContactMgr`, vtable `010cc984`): points still found keep their
    /// properties with the new position, normal and distance; points no
    /// longer found are removed (`removeContactPointImpl` `00cfd320`); new
    /// ones are added (`addContactPointImpl` `00cfcf80`) with the contact
    /// properties of the two sides (`00cfd800`), each heard by the world's
    /// contact listeners as "contact point added" (`fireContactPointAdded`
    /// `00d01850`, the game's `FOCollisionListener::contactPointAdded
    /// Callback` `00623cb0`: impact sounds, physics damage) with its
    /// projected velocity: the first body's velocity at the point less the
    /// other's, along the normal (`00ca0c40`).
    // Translated from 00cfcf80, 00cfd320 and 00d01850 (decompiled, FalloutNV.exe 1.4.0.525)
    fn update_manifold(&mut self, a: usize, other: Partner, found: Vec<Candidate>) {
        let m = self.manifolds.entry((a, other)).or_default();
        let mut k = 0;
        while k < m.points.len() {
            let key = m.points[k].key;
            if let Some(c) = found.iter().find(|c| c.point.key == key) {
                let p = &mut m.points[k];
                p.position = c.point.position;
                p.normal = c.point.normal;
                p.distance = c.point.distance;
                k += 1;
            } else {
                m.remove(k);
            }
        }
        let mut added = Vec::new();
        for c in found {
            if m.points.iter().any(|p| p.key == c.point.key) {
                continue;
            }
            let me = &self.bodies[a].setup;
            let props = crate::manifold::point_properties(
                (me.friction, c.surface.friction),
                (me.restitution, c.surface.restitution),
            );
            m.add(c.point, props);
            added.push(c);
        }
        if m.is_empty() {
            self.manifolds.remove(&(a, other));
        }
        for c in added {
            let p = c.point.position;
            let va = self.bodies[a].point_velocity(sub(p, self.bodies[a].x));
            let vb = match other {
                Partner::Body(j) => self.bodies[j].point_velocity(sub(p, self.bodies[j].x)),
                Partner::Fixed(_) => [0.0; 3],
            };
            let projected = dot(sub(va, vb), c.point.normal);
            self.began.push(ContactEvent {
                body: a,
                other_body: match other {
                    Partner::Body(j) => Some(j),
                    Partner::Fixed(_) => None,
                },
                triangle: c.triangle,
                point: p,
                speed: projected.abs(),
            });
        }
    }

    /// Both sides' points of a touch in the world, and the arms from their
    /// centres.
    fn ends(&self, a: usize, other: Other, t: &Touch) -> (Vec3, Vec3, Vec3, Vec3) {
        let pa = self.bodies[a].to_world(t.on_a);
        let pb = match other {
            Other::Body(j) => self.bodies[j].to_world(t.on_b),
            _ => t.on_b,
        };
        let arm_b = match other {
            Other::Body(j) => sub(pb, self.bodies[j].x),
            _ => [0.0; 3],
        };
        (pa, pb, sub(pa, self.bodies[a].x), arm_b)
    }

    /// Where body `i` touches the collider's triangles `tris`, or comes
    /// within `tol` of touching.
    fn world_touches(
        &self,
        collider: &Collider,
        i: usize,
        tris: &[u32],
        tol: f32,
    ) -> Vec<(u32, Touch)> {
        let b = &self.bodies[i];
        let mut out = Vec::new();
        let mut tags = Vec::new();
        for &t in tris {
            let start = out.len();
            self.triangle_touches(collider, b, t, tol, &mut out);
            tags.extend(std::iter::repeat(t).take(out.len() - start));
        }
        tags.into_iter().zip(out).collect()
    }

    /// Where body `b` touches the collider's triangle `t` (or comes within
    /// `tol`).
    fn triangle_touches(
        &self,
        collider: &Collider,
        b: &Rigid,
        t: u32,
        tol: f32,
        out: &mut Vec<Touch>,
    ) {
        let [ta, tb, tc] = collider.triangle(t);
        let shell = collider.shell(t);
        let surface = collider.surface(t).unwrap_or(DEFAULT_SURFACE);
        let mut face = normalize(cross(sub(tb, ta), sub(tc, ta)));
        if dot(face, sub(b.x, ta)) < 0.0 {
            face = scale(face, -1.0);
        }
        for (si, shape) in b.setup.shapes.iter().enumerate() {
            let mut point = |p_model: Vec3, radius: f32, feature: usize| {
                let p = b.to_world(p_model);
                if let Some((on_b, n, margin)) =
                    point_triangle(p, radius + shell, tol, [ta, tb, tc], face, b.reach)
                {
                    out.push(Touch {
                        on_a: p_model,
                        on_b,
                        n,
                        margin,
                        surface,
                        key: feature_key(1, si, feature, t),
                        radius_b: shell,
                    });
                }
            };
            match shape {
                Shape::Hull {
                    vertices, shell: s, ..
                }
                | Shape::Mesh {
                    vertices, shell: s, ..
                } => {
                    for (k, &v) in vertices.iter().enumerate() {
                        point(v, *s, k);
                    }
                }
                Shape::Sphere { center, radius } => point(*center, *radius, 0),
                Shape::Capsule { a, b: e, radius } => {
                    point(*a, *radius, 0);
                    point(*e, *radius, 1);
                    let (pa, pe) = (b.to_world(*a), b.to_world(*e));
                    let (on_axis, on_tri) = segment_triangle_closest(pa, pe, ta, tb, tc);
                    let mid_model = b.to_model(on_axis);
                    if dist2(on_axis, pa) > 1e-4 && dist2(on_axis, pe) > 1e-4 {
                        let d = length(sub(on_axis, on_tri));
                        let n = if d > 1e-5 {
                            scale(sub(on_axis, on_tri), 1.0 / d)
                        } else {
                            face
                        };
                        if d < radius + shell + tol {
                            out.push(Touch {
                                on_a: mid_model,
                                on_b: on_tri,
                                n,
                                margin: radius + shell,
                                surface,
                                key: feature_key(2, si, 0, t),
                                radius_b: shell,
                            });
                        }
                    }
                }
            }
            // The triangle's edges reaching into a hull (a rail's long
            // edge under a bottle's base, a ridge of the ground): where
            // each enters and leaves the hull grown by the shells, the
            // hull is pushed off along its own face that faces the
            // triangle the most squarely, under that point.
            if let Shape::Hull {
                planes, shell: s, ..
            } = shape
            {
                let margin = s + shell;
                let r = quat_mat(b.q);
                for (ei, (e0, e1)) in [(ta, tb), (tb, tc), (tc, ta)].into_iter().enumerate() {
                    let (l0, l1) = (b.to_model(e0), b.to_model(e1));
                    let Some((t0, t1)) = clip_segment(l0, l1, planes, margin + tol) else {
                        continue;
                    };
                    for (end, t_) in [t0, t1].into_iter().enumerate() {
                        let local = add(l0, scale(sub(l1, l0), t_));
                        // Only a face the body is in front of (a rail's
                        // top under a bottle, not its side).
                        let to_body = normalize(sub(b.x, b.to_world(local)));
                        if dot(face, to_body) < 0.5 {
                            continue;
                        }
                        if let Some(mut touch) = edge_point_touch(
                            &r,
                            local,
                            b.to_world(local),
                            planes,
                            face,
                            (margin, tol),
                            surface,
                        ) {
                            touch.key = feature_key(3, si, ei * 2 + end, t);
                            touch.radius_b = shell;
                            out.push(touch);
                        }
                    }
                }
            }
        }
    }

    /// Where body `i` touches body `j` (`j`'s side as model-space points
    /// of `j`), or comes within `tol`.
    fn body_touches(&self, i: usize, j: usize, tol: f32) -> Vec<Touch> {
        let (a, b) = (&self.bodies[i], &self.bodies[j]);
        let surface = Surface {
            friction: b.setup.friction,
            restitution: b.setup.restitution,
        };
        let mut out = Vec::new();
        for (ia, sa) in a.setup.shapes.iter().enumerate() {
            for (ib, sb) in b.setup.shapes.iter().enumerate() {
                let shapes = ia * 16 + ib;
                // a's corners and centres against b's hulls, b's against
                // a's (turned around).
                for (k, (p, r)) in feature_points(sa).into_iter().enumerate() {
                    let world = a.to_world(p);
                    if let Some(t) = point_in_shape(b, world, sb, r, tol, surface) {
                        out.push(Touch {
                            on_a: p,
                            on_b: b.to_model(t.on_b),
                            key: feature_key(4, shapes, k, 0),
                            radius_b: t.margin - r,
                            ..t
                        });
                    }
                }
                for (k, (p, r)) in feature_points(sb).into_iter().enumerate() {
                    let world = b.to_world(p);
                    if let Some(t) = point_in_shape(a, world, sa, r, tol, surface) {
                        // t: on_b is the point of b (world), n pushes b away
                        // from a; turn it around for a.
                        out.push(Touch {
                            on_a: a.to_model(t.on_b_surface()),
                            on_b: p,
                            n: scale(t.n, -1.0),
                            margin: t.margin,
                            surface,
                            key: feature_key(5, shapes, k, 0),
                            radius_b: r,
                        });
                    }
                }
                // Round shapes against each other.
                if let (Some((a0, a1, ra)), Some((b0, b1, rb))) = (round(sa), round(sb)) {
                    let (wa0, wa1) = (a.to_world(a0), a.to_world(a1));
                    let (wb0, wb1) = (b.to_world(b0), b.to_world(b1));
                    let (pa, pb) = segment_segment_closest(wa0, wa1, wb0, wb1);
                    let d = length(sub(pa, pb));
                    if d < ra + rb + tol && d > 1e-5 {
                        out.push(Touch {
                            on_a: a.to_model(pa),
                            on_b: b.to_model(pb),
                            n: scale(sub(pa, pb), 1.0 / d),
                            margin: ra + rb,
                            surface,
                            key: feature_key(6, shapes, 0, 0),
                            radius_b: rb,
                        });
                    }
                }
            }
        }
        out
    }
}

impl Touch {
    /// For a touch found by [`point_in_shape`]: the point on the shape's
    /// surface (world) where the outside point is pushed out to.
    fn on_b_surface(&self) -> Vec3 {
        // `on_a` holds it there (see `point_in_shape`).
        self.on_a
    }
}

/// A contact's friction: the square root of the two bodies' frictions
/// multiplied (the contact manager's new point, `00cfd800`, from each
/// entity's material at `+0x90`).
// Translated from 00cfd800 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn combined_friction(a: f32, b: f32) -> f32 {
    (a * b).max(0.0).sqrt()
}

/// A contact's restitution: the square root of the two restitutions
/// multiplied (`+0x94`), kept in a byte as × 128 rounded (`00cfd800`).
// Translated from 00cfd800 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn combined_restitution(a: f32, b: f32) -> f32 {
    let r = (a * b).max(0.0).sqrt();
    // FISTP to an integer, its low byte kept.
    f32::from((r * 128.0).round() as i32 as u8) / 128.0
}

/// The points of a shape that touch others here, with their radius:
/// corners (with the shell), sphere and capsule centres.
fn feature_points(shape: &Shape) -> Vec<(Vec3, f32)> {
    match shape {
        Shape::Hull {
            vertices, shell, ..
        }
        | Shape::Mesh {
            vertices, shell, ..
        } => vertices.iter().map(|&v| (v, *shell)).collect(),
        Shape::Sphere { center, radius } => vec![(*center, *radius)],
        Shape::Capsule { a, b, radius } => vec![(*a, *radius), (*b, *radius)],
    }
}

/// A sphere or capsule as a segment and a radius.
fn round(shape: &Shape) -> Option<(Vec3, Vec3, f32)> {
    match shape {
        Shape::Sphere { center, radius } => Some((*center, *center, *radius)),
        Shape::Capsule { a, b, radius } => Some((*a, *b, *radius)),
        _ => None,
    }
}

fn closest_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let ab = sub(b, a);
    let l = dot(ab, ab);
    if l < 1e-12 {
        return a;
    }
    add(a, scale(ab, (dot(sub(p, a), ab) / l).clamp(0.0, 1.0)))
}

/// A point `p` of a body (with `margin`: its radius and the triangle's
/// shell) against a triangle whose face normal `face` points toward the
/// body: the point on the triangle side (world), the normal pushing the
/// body away and the margin, when they overlap. A point that has gone
/// through the face (up to `deep` behind it) still counts while it's over
/// the triangle.
fn point_triangle(
    p: Vec3,
    margin: f32,
    tol: f32,
    [a, b, c]: [Vec3; 3],
    face: Vec3,
    deep: f32,
) -> Option<(Vec3, Vec3, f32)> {
    let s = dot(face, sub(p, a));
    let on_plane = sub(p, scale(face, s));
    let inside = {
        let n = face;
        dot(cross(sub(b, a), sub(on_plane, a)), n) * dot(cross(sub(b, a), sub(c, a)), n) >= 0.0
            && dot(cross(sub(c, b), sub(on_plane, b)), n) * dot(cross(sub(c, b), sub(a, b)), n)
                >= 0.0
            && dot(cross(sub(a, c), sub(on_plane, c)), n) * dot(cross(sub(a, c), sub(b, c)), n)
                >= 0.0
    };
    if inside {
        return (s < margin + tol && s > -deep).then_some((on_plane, face, margin));
    }
    let q = closest_on_triangle(p, a, b, c);
    let d = length(sub(p, q));
    (d < margin + tol && d > 1e-5).then(|| (q, scale(sub(p, q), 1.0 / d), margin))
}

/// The part of segment `a`–`b` (model space) inside a hull's planes grown
/// by `margin`, as the range of the segment's parameter (Cyrus–Beck).
fn clip_segment(a: Vec3, b: Vec3, planes: &[[f32; 4]], margin: f32) -> Option<(f32, f32)> {
    if planes.is_empty() {
        return None;
    }
    let d = sub(b, a);
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    for p in planes {
        let n = [p[0], p[1], p[2]];
        let start = dot(n, a) + p[3] - margin;
        let along = dot(n, d);
        if along.abs() < 1e-9 {
            if start > 0.0 {
                return None;
            }
            continue;
        }
        let t = -start / along;
        if along > 0.0 {
            t1 = t1.min(t);
        } else {
            t0 = t0.max(t);
        }
        if t0 > t1 {
            return None;
        }
    }
    Some((t0, t1))
}

/// A point of the world (`local` in the body's model space, `world` in the
/// world) within a hull's `margin`: the touch pushing the hull off it along
/// the hull's own face that faces the world's triangle (normal `face`,
/// pointing toward the body) the most squarely among those it's least
/// deep behind; none when no face of the hull faces the triangle.
fn edge_point_touch(
    r: &Mat3,
    local: Vec3,
    world: Vec3,
    planes: &[[f32; 4]],
    face: Vec3,
    (margin, tol): (f32, f32),
    surface: Surface,
) -> Option<Touch> {
    let mut best: Option<(f32, Vec3, Vec3)> = None;
    for p in planes {
        let n_local = [p[0], p[1], p[2]];
        let n = mat_vec(r, n_local);
        if dot(n, face) > -0.5 {
            continue;
        }
        let sd = dot(n_local, local) + p[3];
        if best.map_or(true, |(b, _, _)| sd > b) {
            best = Some((sd, n_local, n));
        }
    }
    let (sd, n_local, n) = best?;
    (sd < margin + tol).then(|| Touch {
        on_a: sub(local, scale(n_local, sd)),
        on_b: world,
        n: scale(n, -1.0),
        margin,
        surface,
        key: 0,
        radius_b: 0.0,
    })
}

/// A world point `p` inside body `b`'s hull (its planes, in the model's
/// space) grown by `margin`: pushes the body away from it, along the face
/// it's least deep behind.
fn corner_in_hull(
    b: &Rigid,
    p: Vec3,
    planes: &[[f32; 4]],
    margin: f32,
    tol: f32,
    surface: Surface,
) -> Option<Touch> {
    if planes.is_empty() {
        return None;
    }
    let local = b.to_model(p);
    let mut best = (f32::NEG_INFINITY, [0.0; 3]);
    for pl in planes {
        let n = [pl[0], pl[1], pl[2]];
        let sd = dot(n, local) + pl[3];
        if sd > best.0 {
            best = (sd, n);
        }
    }
    let (sd, n_local) = best;
    if sd >= margin + tol {
        return None;
    }
    let r = quat_mat(b.q);
    let n = mat_vec(&r, n_local);
    // The hull's surface point under `p`, as a model-space point of the
    // body.
    let on_hull = sub(local, scale(n_local, sd));
    Some(Touch {
        on_a: on_hull,
        on_b: p,
        n: scale(n, -1.0),
        margin,
        surface,
        key: 0,
        radius_b: 0.0,
    })
}

/// A world point `p` with radius `r` against shape `shape` of body `b`:
/// the touch pushing `p`'s side away from `b` (`n`), with `on_a` holding
/// the shape's surface point (world) and `on_b` the shape's surface point
/// too (filled by the caller).
fn point_in_shape(
    b: &Rigid,
    p: Vec3,
    shape: &Shape,
    r: f32,
    tol: f32,
    surface: Surface,
) -> Option<Touch> {
    match shape {
        Shape::Hull { planes, shell, .. } => {
            let t = corner_in_hull(b, p, planes, shell + r, tol, surface)?;
            // corner_in_hull pushes b away from p; for p's side, the other
            // way, and the surface point in the world.
            let on_surface = b.to_world(t.on_a);
            Some(Touch {
                on_a: on_surface,
                on_b: on_surface,
                n: scale(t.n, -1.0),
                margin: t.margin,
                surface,
                key: 0,
                radius_b: 0.0,
            })
        }
        Shape::Sphere { .. } | Shape::Capsule { .. } => {
            let (s0, s1, radius) = round(shape)?;
            let (w0, w1) = (b.to_world(s0), b.to_world(s1));
            let on = closest_on_segment(p, w0, w1);
            let d = length(sub(p, on));
            if d >= radius + r + tol || d < 1e-5 {
                return None;
            }
            Some(Touch {
                on_a: on,
                on_b: on,
                n: scale(sub(p, on), 1.0 / d),
                margin: radius + r,
                surface,
                key: 0,
                radius_b: 0.0,
            })
        }
        Shape::Mesh { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I3: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    fn floor(at: f32) -> Collider {
        let mut c = Collider::new();
        c.add(
            &[
                [-1000.0, -1000.0, at],
                [1000.0, -1000.0, at],
                [1000.0, 1000.0, at],
                [-1000.0, 1000.0, at],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c
    }

    /// A box of half sizes `h` centred on its origin, of `mass`.
    fn crate_(reference: u32, h: Vec3, mass: f32) -> RigidSetup {
        let corners: Vec<Vec3> = (0..8)
            .map(|k| {
                [
                    if k & 1 == 0 { -h[0] } else { h[0] },
                    if k & 2 == 0 { -h[1] } else { h[1] },
                    if k & 4 == 0 { -h[2] } else { h[2] },
                ]
            })
            .collect();
        let planes = [
            [1.0, 0.0, 0.0, -h[0]],
            [-1.0, 0.0, 0.0, -h[0]],
            [0.0, 1.0, 0.0, -h[1]],
            [0.0, -1.0, 0.0, -h[1]],
            [0.0, 0.0, 1.0, -h[2]],
            [0.0, 0.0, -1.0, -h[2]],
        ];
        let k = mass / 3.0;
        RigidSetup {
            reference,
            layer: 4,
            mass,
            center: [0.0; 3],
            inertia: [
                [k * (h[1] * h[1] + h[2] * h[2]), 0.0, 0.0],
                [0.0, k * (h[0] * h[0] + h[2] * h[2]), 0.0],
                [0.0, 0.0, k * (h[0] * h[0] + h[1] * h[1])],
            ],
            linear_damping: 0.1,
            angular_damping: 0.05,
            friction: 0.5,
            restitution: 0.4,
            max_linear_speed: 1068.0 * HAVOK_UNIT,
            max_angular_speed: 31.57,
            motion: 4,
            quality: 3,
            wind: false,
            shapes: vec![Shape::hull(corners, &planes, 0.7)],
        }
    }

    #[test]
    fn the_clock_steps_as_the_game_does() {
        let mut c = Clock::new(0.016);
        // A 60 Hz frame: one step, 0.000667 s over.
        assert_eq!(c.advance(1.0 / 60.0), (1, 0.016));
        // A long frame: three steps at most, the rest held to one step.
        let (n, dt) = c.advance(0.2);
        assert_eq!((n, dt), (3, 0.016));
        assert!((c.left() - 0.016).abs() < 1e-6, "{}", c.left());
        // Less than half a step waits; half or more takes one of it all.
        let mut c = Clock::new(0.016);
        assert_eq!(c.advance(0.005), (0, 0.0));
        assert_eq!(c.advance(0.002), (0, 0.0));
        let (n, dt) = c.advance(0.0015);
        assert_eq!(n, 1);
        assert!((dt - 0.016).abs() < 1e-6, "{dt}");
        // Rounding up runs ahead: the rest is negative.
        let mut c = Clock::new(0.016);
        assert_eq!(c.advance(0.013).0, 1);
        assert!(c.left() < 0.0);
    }

    #[test]
    fn a_body_in_the_open_is_stepped_by_havoks_integrator() {
        // Far above the floor: nothing near it, so each step is exactly
        // `hkRigidMotionUtilApplyForcesAndStep` in game units.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 5000.0]));
        w.wake(i);
        w.bodies[i].w = [0.0, 0.0, 3.0];
        let mut m = w.bodies[i].motion_now();
        let g = scale(crate::havok::GRAVITY, STEP * HAVOK_UNIT);
        for _ in 0..40 {
            assert_eq!(w.update(&c, STEP), 1);
            let solver = w.solver;
            m.step(STEP, g, HAVOK_UNIT, &solver);
            assert_eq!(w.bodies[i].center(), m.center);
            assert_eq!(w.bodies[i].velocity(), m.linear_velocity);
            assert_eq!(w.bodies[i].spin(), m.angular_velocity);
        }
    }

    #[test]
    fn a_resting_body_sleeps_after_the_traced_number_of_steps() {
        // Woken resting on the floor: its first check (step 4) sets the
        // references (a new motion's are zero); slot 0 then passes at
        // steps 8, 12, 20, 24, 28 and 36, the sixth pass marks its island
        // inactive, and the next step's start puts it to sleep.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 10.7]));
        w.wake(i);
        for k in 1..=37 {
            assert!(!w.bodies[i].asleep, "asleep before step {k}");
            assert_eq!(w.update(&c, STEP), 1);
        }
        assert!(w.bodies[i].asleep);
        assert_eq!(w.bodies[i].velocity(), [0.0; 3]);
    }

    #[test]
    fn a_stack_sleeps_together_and_a_nudge_wakes_it() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let lower = w.add(crate_(3, [10.0; 3], 10.0), (I3, [0.0, 0.0, 10.7]));
        let upper = w.add(crate_(4, [10.0; 3], 10.0), (I3, [0.0, 0.0, 32.1]));
        w.wake(lower);
        w.wake(upper);
        let mut slept = None;
        for k in 1..=200 {
            w.update(&c, STEP);
            let (a, b) = (w.bodies[lower].asleep, w.bodies[upper].asleep);
            if a || b {
                // One island: both at once.
                assert!(a && b, "step {k}: {a} {b}");
                slept = Some(k);
                break;
            }
        }
        assert!(slept.is_some());
        // A small push on the lower box: it wakes, and the box on it with
        // it (their islands merge), and both settle again.
        w.apply_linear_impulse(lower, [0.5, 0.0, 0.0]);
        w.update(&c, STEP);
        assert!(!w.bodies[lower].asleep && !w.bodies[upper].asleep);
        for _ in 0..300 {
            w.update(&c, STEP);
        }
        assert!(w.bodies[lower].asleep && w.bodies[upper].asleep);
    }

    #[test]
    fn bodies_placed_together_share_an_island_and_wake_together() {
        // Two boxes put side by side, 0.2 apart (inside the broadphase's
        // half tolerance on each box): one island at once, asleep; a third
        // further off has its own.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let a = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 10.7]));
        let b = w.add(crate_(2, [10.0; 3], 5.0), (I3, [21.6, 0.0, 10.7]));
        let far = w.add(crate_(3, [10.0; 3], 5.0), (I3, [100.0, 0.0, 10.7]));
        assert_eq!(w.islands.island_of(a), w.islands.island_of(b));
        assert_ne!(w.islands.island_of(a), w.islands.island_of(far));
        assert!(w.bodies.iter().all(|b| b.asleep));
        // A push on one wakes its neighbour, not the far one.
        w.apply_linear_impulse(a, [0.0, 0.0, 0.1]);
        assert!(!w.bodies[b].asleep && w.bodies[far].asleep);
        w.update(&c, STEP);
        assert!(!w.bodies[a].asleep && !w.bodies[b].asleep && w.bodies[far].asleep);
    }

    #[test]
    fn a_moving_body_wakes_a_sleeper_when_their_boxes_meet() {
        // A box sliding along the floor toward a sleeping one: the sleeper
        // wakes when the broadphase pairs them, before they touch.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let mover = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 10.7]));
        let sleeper = w.add(crate_(2, [10.0; 3], 5.0), (I3, [40.0, 0.0, 10.7]));
        w.wake(mover);
        w.set_velocity(mover, [200.0, 0.0, 0.0], [0.0; 3]);
        let mut woke_at = None;
        for k in 0..60 {
            w.update(&c, STEP);
            if !w.bodies[sleeper].asleep {
                woke_at = Some((k, w.bodies[mover].center()[0]));
                break;
            }
        }
        woke_at.expect("the sleeper wakes");
        // Woken by the broadphase pair, before any contact moved it.
        assert_eq!(w.bodies[sleeper].center(), [40.0, 0.0, 10.7]);
        assert_eq!(w.islands.island_of(mover), w.islands.island_of(sleeper));
    }

    #[test]
    fn bodies_that_part_split_and_sleep_apart() {
        // Two boxes side by side, one shoved away hard along the floor:
        // their pair ends, the island splits at the next step's start, and
        // the one left still sleeps on its own while the other slides.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let a = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 10.7]));
        let b = w.add(crate_(2, [10.0; 3], 5.0), (I3, [21.6, 0.0, 10.7]));
        w.wake(a);
        w.set_velocity(b, [300.0, 0.0, 0.0], [0.0; 3]);
        let mut apart = false;
        for _ in 0..400 {
            w.update(&c, STEP);
            if w.islands.island_of(a) != w.islands.island_of(b) {
                apart = true;
            }
            if apart && w.bodies[a].asleep && !w.bodies[b].asleep {
                break;
            }
        }
        assert!(apart);
        assert!(w.bodies[a].asleep);
    }

    #[test]
    fn layers_that_dont_meet_dont_pair() {
        // Two overlapping bodies on layers the collision filter keeps
        // apart get no agent, so their islands stay apart.
        let f = crate::layers::Filter::shared();
        let mut w = RigidWorld::new();
        let mut x = crate_(1, [10.0; 3], 5.0);
        let mut y = crate_(2, [10.0; 3], 5.0);
        // Find two layers the filter keeps apart.
        let (la, lb) = (0..crate::layers::LAYERS as u8)
            .flat_map(|a| (0..crate::layers::LAYERS as u8).map(move |b| (a, b)))
            .find(|&(a, b)| !f.layers_touch(a, b) && !f.layers_touch(b, a))
            .expect("two layers apart");
        x.layer = la;
        y.layer = lb;
        let a = w.add(x, (I3, [0.0, 0.0, 10.7]));
        let b = w.add(y, (I3, [5.0, 0.0, 10.7]));
        assert_ne!(w.islands.island_of(a), w.islands.island_of(b));
    }

    #[test]
    fn a_box_falls_and_rests_on_the_floor() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(1, [10.0, 10.0, 10.0], 5.0), (I3, [0.0, 0.0, 100.0]));
        w.wake(i);
        let c = floor(0.0);
        for _ in 0..240 {
            w.update(&c, 1.0 / 60.0);
        }
        let (_, t) = w.bodies[i].pose();
        // Half its size plus its shell above the floor.
        assert!((t[2] - 10.7).abs() < 0.5, "{t:?}");
        assert!(t[0].abs() < 1.0 && t[1].abs() < 1.0, "{t:?}");
        assert!(w.bodies[i].asleep && w.bodies[i].moved);
    }

    #[test]
    fn a_shot_knocks_a_bottle_off_a_rail() {
        // A rail 4 units wide at z 100; a tall thin body standing on it.
        let mut c = Collider::new();
        c.add(
            &[
                [-200.0, -2.0, 100.0],
                [200.0, -2.0, 100.0],
                [200.0, 2.0, 100.0],
                [-200.0, 2.0, 100.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c.extend(&floor(0.0));
        let mut w = RigidWorld::new();
        let i = w.add(crate_(2, [1.5, 1.5, 12.0], 1.0), (I3, [0.0, 0.0, 112.7]));
        // Asleep where it was put, until pushed.
        for _ in 0..30 {
            w.update(&c, 1.0 / 60.0);
        }
        assert!(w.bodies[i].asleep);
        assert_eq!(w.bodies[i].pose().1, [0.0, 0.0, 112.7]);
        // A shot along +y high up.
        let j = crate::impulses::projectile_impulse(
            10.0,
            [0.0, 1.0, 0.0],
            10,
            1.0,
            &Default::default(),
        )
        .unwrap();
        w.apply_point_impulse(i, j, [0.0, 0.0, 120.0]);
        assert!(!w.bodies[i].asleep && w.bodies[i].moved);
        for _ in 0..300 {
            w.update(&c, 1.0 / 60.0);
        }
        let (_, t) = w.bodies[i].pose();
        // Off the rail, on the ground beyond it.
        assert!(t[2] < 20.0, "{t:?}");
        assert!(t[1] > 2.0, "{t:?}");
    }

    #[test]
    fn bodies_stack_and_the_proxy_pushes_them() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let lower = w.add(crate_(3, [10.0; 3], 10.0), (I3, [0.0, 0.0, 10.7]));
        let upper = w.add(crate_(4, [10.0; 3], 10.0), (I3, [0.0, 0.0, 40.0]));
        w.wake(upper);
        for _ in 0..240 {
            w.update(&c, 1.0 / 60.0);
        }
        let top = w.bodies[upper].pose().1;
        assert!((top[2] - 32.1).abs() < 1.5, "{top:?}");
        // A proxy coming from the west at 150 units a second touches the
        // lower box's west face every frame and pushes it east.
        let start = w.bodies[lower].pose().1;
        for _ in 0..60 {
            let touch = touch_west(&w, lower, 150.0 / HAVOK_UNIT);
            w.apply_surface_interactions(&[touch], crate::havok::GRAVITY);
            w.update(&c, 1.0 / 60.0);
        }
        let now = w.bodies[lower].pose().1;
        assert!(now[0] > start[0] + 20.0, "{start:?} → {now:?}");
    }

    /// A six-sided bottle as `clutter\junk\ssbottle02.nif`'s body has it
    /// (rounded): 4.5 across its corners, 17.8 tall to the shoulder, its
    /// neck to 29.1 narrowing to 2; mass 1, centre 12.3 up, inertia 59.1,
    /// 59.1, 7.7; damping, friction, restitution and limits as stored.
    fn bottle(reference: u32) -> RigidSetup {
        let ring = |r: f32, z: f32| -> Vec<Vec3> {
            (0..6)
                .map(|k| {
                    let a = k as f32 * std::f32::consts::FRAC_PI_3;
                    [r * a.cos(), r * a.sin(), z]
                })
                .collect()
        };
        let mut vertices = ring(4.5, 0.0);
        vertices.extend(ring(4.5, 17.8));
        vertices.extend(ring(2.0, 29.1));
        // Faces: bottom, top, the six sides of the body, the six of the
        // shoulder; planes through their corners, pointing out.
        let mut planes = vec![[0.0, 0.0, -1.0, 0.0], [0.0, 0.0, 1.0, -29.1]];
        for k in 0..6 {
            for (lo, hi) in [(0, 6), (6, 12)] {
                let (a, b, c) = (
                    vertices[lo + k],
                    vertices[lo + (k + 1) % 6],
                    vertices[hi + k],
                );
                let mut n = normalize(cross(sub(b, a), sub(c, a)));
                if dot(n, [a[0], a[1], 0.0]) < 0.0 {
                    n = scale(n, -1.0);
                }
                planes.push([n[0], n[1], n[2], -dot(n, a)]);
            }
        }
        RigidSetup {
            reference,
            layer: 10,
            mass: 1.0,
            center: [0.0, 0.0, 12.31],
            inertia: [[59.13, 0.0, 0.0], [0.0, 59.13, 0.0], [0.0, 0.0, 7.69]],
            linear_damping: 0.1,
            angular_damping: 0.05,
            friction: 0.5,
            restitution: 0.4,
            max_linear_speed: 1068.0 * HAVOK_UNIT,
            max_angular_speed: 31.57,
            motion: 4,
            quality: 3,
            wind: false,
            shapes: vec![Shape::hull(vertices, &planes, 0.7)],
        }
    }

    /// A rail 3 wide with its top at z 100 (two boxes meeting at x 0.5),
    /// over the ground at z 20 (the terrain's shell).
    fn fence() -> Collider {
        let mut c = Collider::new();
        for (x0, x1) in [(-200.0f32, 0.5f32), (0.5, 200.0)] {
            let v = [
                [x0, -1.5, 100.0],
                [x1, -1.5, 100.0],
                [x1, 1.5, 100.0],
                [x0, 1.5, 100.0],
                [x0, -1.5, 96.0],
                [x1, -1.5, 96.0],
                [x1, 1.5, 96.0],
                [x0, 1.5, 96.0],
            ];
            let t = [
                [0, 1, 2],
                [0, 2, 3],
                [4, 6, 5],
                [4, 7, 6],
                [0, 4, 5],
                [0, 5, 1],
                [3, 2, 6],
                [3, 6, 7],
            ];
            c.add_solid(&v, &t, 0.7, 0);
        }
        c.add_solid(
            &[
                [-1000.0, -1000.0, 20.0],
                [1000.0, -1000.0, 20.0],
                [1000.0, 1000.0, 20.0],
                [-1000.0, 1000.0, 20.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
            crate::TERRAIN_SHELL,
            0,
        );
        c
    }

    #[test]
    fn a_bottle_stands_on_a_narrow_rail_until_a_shot_tips_it_off() {
        bottle_shot_off_a_rail([0.0; 3]);
    }

    #[test]
    fn far_from_the_origin_a_shot_bottle_falls_the_same_way() {
        // Goodsprings' fence is at (-68233, 5050, 8438): there a
        // position's rounding is worth several units a second of speed
        // when velocities come from positions.
        bottle_shot_off_a_rail([-68233.0, 5050.0, 8338.0]);
    }

    /// Everything moved by `at`.
    fn shifted(c: &Collider, at: Vec3) -> Collider {
        let mut out = Collider::new();
        for k in 0..c.triangle_count() as u32 {
            let tri = c.triangle(k).map(|p| add(p, at));
            out.add_solid(&tri, &[[0, 1, 2]], c.shell(k), 0);
        }
        out
    }

    fn bottle_shot_off_a_rail(at: Vec3) {
        let c = shifted(&fence(), at);
        let mut w = RigidWorld::new();
        let i = w.add(bottle(9), (I3, add(at, [0.0, 0.0, 101.4])));
        // Woken, it stays standing on the rail (its base wider than the
        // rail: held by the rail's long edges) and falls asleep.
        w.apply_linear_impulse(i, [0.0, 0.0, 1e-4]);
        for _ in 0..120 {
            w.update(&c, 1.0 / 60.0);
        }
        let t = sub(w.bodies[i].pose().1, at);
        assert!(length(sub(t, [0.0, 0.0, 101.4])) < 0.1, "{t:?}");
        assert!(w.bodies[i].asleep);
        // A varmint rifle round (impact force 3: 9 Havok units on this
        // prop) high on it, heading north: it tips over and falls off
        // behind the rail, coming to rest on the ground.
        let j = crate::impulses::projectile_impulse(
            3.0,
            [0.0, 1.0, -0.14],
            10,
            1.0,
            &Default::default(),
        )
        .unwrap();
        w.apply_point_impulse(i, j, add(at, [0.0, 0.0, 120.0]));
        for _ in 0..400 {
            w.update(&c, 1.0 / 60.0);
        }
        let t = sub(w.bodies[i].pose().1, at);
        // Off northward, more than sideways (lying on its side it rolls a
        // little either way).
        assert!(t[1] > 10.0 && t[2] < 30.0 && t[0].abs() < t[1], "{t:?}");
        assert!(w.bodies[i].asleep && w.bodies[i].moved);
        // Lying on its side on the ground: its axis level.
        let (r, _) = w.bodies[i].pose();
        assert!(r[2][2].abs() < 0.2, "{r:?}");
    }

    #[test]
    fn a_rails_end_under_a_body_doesnt_push_it_sideways() {
        // A rail ending under a standing body (its end corners within the
        // body's footprint, inside its shell): the body stays put.
        let mut c = Collider::new();
        c.add(
            &[
                [-200.0, -2.0, 100.0],
                [1.0, -2.0, 100.0],
                [1.0, 2.0, 100.0],
                [-200.0, 2.0, 100.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        let mut w = RigidWorld::new();
        let i = w.add(crate_(8, [2.0, 2.0, 12.0], 1.0), (I3, [0.0, 0.0, 112.7]));
        w.apply_linear_impulse(i, [0.0, 0.0, 0.001]);
        for _ in 0..120 {
            w.update(&c, 1.0 / 60.0);
        }
        let t = w.bodies[i].pose().1;
        assert!(t[0].abs() < 0.5 && t[1].abs() < 0.5, "{t:?}");
        assert!((t[2] - 112.7).abs() < 0.5, "{t:?}");
    }

    #[test]
    fn surfaces_keep_their_bodies_values_through_extend() {
        let ice = Surface {
            friction: 0.1,
            restitution: 0.2,
        };
        let mut a = Collider::new();
        a.add_solid_surface(
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[[0, 1, 2]],
            (0.7, 0, 3),
            Some(ice),
        );
        let mut all = floor(0.0);
        all.extend(&a);
        let last = all.triangle_count() as u32 - 1;
        assert_eq!(all.surface(last), Some(ice));
        assert_eq!(all.surface(0), None);
        assert_eq!(all.material(last), Some(3));
    }

    #[test]
    fn deltas_take_the_rest_pose_to_the_pose_now() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(5, [5.0; 3], 2.0), (I3, [10.0, 20.0, 30.0]));
        let turned = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        w.place(i, (turned, [0.0, 0.0, 5.0]), true);
        let (r, t) = w.delta(i);
        // The rest pose's origin goes to the new one.
        let moved = add(mat_vec(&r, [10.0, 20.0, 30.0]), t);
        assert!(length(sub(moved, [0.0, 0.0, 5.0])) < 1e-4, "{moved:?}");
        assert!(w.bodies[i].moved && w.bodies[i].asleep);
    }

    #[test]
    fn impulses_change_velocity_by_mass_in_havok_units() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(6, [5.0; 3], 2.0), (I3, [0.0; 3]));
        w.apply_linear_impulse(i, [4.0, 0.0, 0.0]);
        // 4 / 2 Havok units a second, in game units.
        assert!((w.bodies[i].velocity()[0] - 2.0 * HAVOK_UNIT).abs() < 1e-4);
        w.apply_angular_impulse(i, [0.0, 0.0, 1.0]);
        assert!(w.bodies[i].spin()[2] > 0.0);
        // Fixed bodies don't move.
        let mut fixed = crate_(7, [5.0; 3], 2.0);
        fixed.motion = 7;
        let j = w.add(fixed, (I3, [50.0, 0.0, 0.0]));
        w.apply_linear_impulse(j, [4.0, 0.0, 0.0]);
        assert_eq!(w.bodies[j].velocity(), [0.0; 3]);
        assert!(w.bodies[j].asleep);
    }

    /// A touch on the west face of a crate at `h` high, the proxy moving
    /// east at `speed` Havok units a second (`applySurfaceInteractions`).
    fn touch_west(w: &RigidWorld, i: usize, speed: f32) -> crate::proxy::SurfaceContact {
        let b = w.bodies[i].center();
        crate::proxy::SurfaceContact {
            reference: w.bodies[i].setup.reference,
            position: [b[0] - 10.0, b[1], b[2]],
            normal: [-1.0, 0.0, 0.0],
            distance: 0.0,
            velocity: [speed, 0.0, 0.0],
            dt: 1.0 / 60.0,
            strength: 3.402_822e38,
            mass: 0.0,
        }
    }

    #[test]
    fn a_touch_gives_a_body_nine_tenths_of_the_proxys_speed_whatever_its_mass() {
        // Through the centre of mass the impulse is f ÷ inverse mass, so
        // the body gains 0.9 × the closing speed (the game's strength is
        // FLT_MAX: nothing limits it).
        for mass in [5.0, 20.0, 100.0, 400.0] {
            let mut w = RigidWorld::new();
            let i = w.add(crate_(11, [10.0; 3], mass), (I3, [0.0, 0.0, 10.7]));
            let touch = touch_west(&w, i, 2.0);
            w.apply_surface_interactions(&[touch], crate::havok::GRAVITY);
            let v = w.bodies[i].velocity();
            assert!(
                (v[0] - 0.9 * 2.0 * HAVOK_UNIT).abs() < 1e-2,
                "{mass}: {v:?}"
            );
            assert!(v[1].abs() < 1e-3 && v[2].abs() < 1e-3, "{mass}: {v:?}");
        }
    }

    #[test]
    fn a_touch_pushes_a_second_time_by_what_is_left() {
        // The second pass sees the first push's velocity: 0.9 of the rest.
        let mut w = RigidWorld::new();
        let i = w.add(crate_(11, [10.0; 3], 20.0), (I3, [0.0, 0.0, 10.7]));
        let touch = touch_west(&w, i, 2.0);
        w.apply_surface_interactions(&[touch, touch], crate::havok::GRAVITY);
        let v = w.bodies[i].velocity()[0] / HAVOK_UNIT;
        assert!((v - (2.0 - 0.1 * 0.1 * 2.0)).abs() < 1e-3, "{v}");
    }

    #[test]
    fn a_touch_from_a_body_moving_away_or_a_stranger_does_nothing() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(11, [10.0; 3], 20.0), (I3, [0.0, 0.0, 10.7]));
        // The proxy moving away from the body (west).
        let away = touch_west(&w, i, -2.0);
        // A reference the world doesn't hold.
        let mut stranger = touch_west(&w, i, 2.0);
        stranger.reference = 999;
        w.apply_surface_interactions(&[away, stranger], crate::havok::GRAVITY);
        assert_eq!(w.bodies[i].velocity(), [0.0; 3]);
    }

    #[test]
    fn a_penetrating_touch_is_pushed_out_at_four_tenths_per_step() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(11, [10.0; 3], 20.0), (I3, [0.0, 0.0, 10.7]));
        let mut touch = touch_west(&w, i, 0.0);
        touch.distance = -0.05;
        w.apply_surface_interactions(&[touch], crate::havok::GRAVITY);
        // f = −0.05 × 60 × 0.4 along the normal: the body goes east.
        let v = w.bodies[i].velocity()[0] / HAVOK_UNIT;
        assert!((v - 0.05 * 60.0 * 0.4).abs() < 1e-3, "{v}");
    }

    #[test]
    fn the_strength_limits_the_impulse_and_the_mass_adds_weight() {
        let mut w = RigidWorld::new();
        let i = w.add(crate_(11, [10.0; 3], 20.0), (I3, [0.0, 0.0, 10.7]));
        let mut touch = touch_west(&w, i, 2.0);
        // At most strength × dt: 30 × (1/60) = 0.5 of impulse, 0.025
        // Havok units a second on 20 kg.
        touch.strength = 30.0;
        w.apply_surface_interactions(&[touch], crate::havok::GRAVITY);
        let v = w.bodies[i].velocity()[0] / HAVOK_UNIT;
        assert!((v - 0.5 / 20.0).abs() < 1e-4, "{v}");
        // Standing on a body (the normal up) with a mass: dt × g·n × mass
        // down on it.
        let mut w = RigidWorld::new();
        let i = w.add(crate_(12, [10.0; 3], 20.0), (I3, [0.0, 0.0, 10.7]));
        let b = w.bodies[i].center();
        let on = crate::proxy::SurfaceContact {
            reference: 12,
            position: [b[0], b[1], b[2] + 10.0],
            normal: [0.0, 0.0, 1.0],
            distance: 0.0,
            velocity: [0.0; 3],
            dt: 1.0 / 60.0,
            strength: 3.402_822e38,
            mass: 80.0,
        };
        w.apply_surface_interactions(&[on], crate::havok::GRAVITY);
        let vz = w.bodies[i].velocity()[2] / HAVOK_UNIT;
        assert!((vz - 80.0 / 20.0 * (-98.1 / 60.0)).abs() < 1e-3, "{vz}");
    }

    /// As the viewer has it: the body's triangles in the collider (the
    /// walking character walks through them) and the proxy's touches
    /// handed to the world.
    fn walk_into(h: Vec3, mass: f32) -> (Vec3, Vec3) {
        let mut c = floor(0.0);
        let setup = crate_(13, h, mass);
        let mut w = RigidWorld::new();
        let i = w.add(setup.clone(), (I3, [0.0, 0.0, h[2] + 0.7]));
        for shape in &setup.shapes {
            let (v, t, shell) = shape.triangles();
            let placed: Vec<Vec3> = v.iter().map(|p| add(*p, [0.0, 0.0, h[2] + 0.7])).collect();
            c.add_layered(&placed, &t, (shell, 13, crate::NO_MATERIAL), None, 4);
        }
        let shape = crate::CharacterShape::PLAYER;
        let mut walker = crate::Character::new([-100.0, 0.0, 0.0]);
        for _ in 0..90 {
            walker.update_controlled(&c, &shape, [280.0, 0.0], None, 1.0, 1.0 / 60.0);
            w.apply_surface_interactions(&walker.surface, crate::havok::GRAVITY);
            w.update(&c, 1.0 / 60.0);
            let (r, t) = w.delta(i);
            c.move_owner(13, &r, t);
        }
        (w.bodies[i].center(), walker.feet)
    }

    #[test]
    fn the_player_walking_into_clutter_pushes_it() {
        // A low, light box (a tumbleweed's size): pushed ahead.
        let (at, feet) = walk_into([15.0, 15.0, 15.0], 5.0);
        assert!(at[0] > 30.0, "{at:?} (walker at {feet:?})");
    }

    #[test]
    fn materials_combine_as_havok_does() {
        // A bottle (0.5, 0.4) on the ground (2.5, 0.4).
        assert!((combined_friction(0.5, 2.5) - 1.118_034).abs() < 1e-5);
        // sqrt(0.16) = 0.4 → 51.2 → 51 / 128.
        assert_eq!(combined_restitution(0.4, 0.4), 51.0 / 128.0);
        assert_eq!(combined_restitution(1.0, 1.0), 1.0);
    }

    #[test]
    fn a_falling_body_reports_one_contact_when_it_lands() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(10, [3.0; 3], 1.0), (I3, [0.0, 0.0, 60.0]));
        w.wake(i);
        let mut events = Vec::new();
        for _ in 0..120 {
            w.update(&c, 1.0 / 60.0);
            events.extend(w.take_contacts());
        }
        // Landing from about 56 units: sqrt(2 g h) ≈ 280 units a second.
        let first = events.first().copied().expect("a landing");
        assert!(first.speed > 200.0 && first.speed < 320.0, "{first:?}");
        assert_eq!((first.body, first.other_body), (i, None));
        assert!(first.triangle.is_some() && first.point[2] < 5.0);
        // Resting on the floor doesn't keep reporting.
        let late: Vec<_> = (0..60)
            .flat_map(|_| {
                w.update(&c, 1.0 / 60.0);
                w.take_contacts()
            })
            .collect();
        assert!(late.is_empty(), "{late:?}");
    }

    #[test]
    fn each_new_contact_point_is_heard_once() {
        // A box dropped on the floor: one event per contact point the
        // manager adds (its bottom corners), none while it rests on them.
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(10, [3.0; 3], 1.0), (I3, [0.0, 0.0, 30.0]));
        w.wake(i);
        let mut events = Vec::new();
        for _ in 0..90 {
            w.update(&c, STEP);
            events.extend(w.take_contacts());
        }
        let m = &w.manifolds[&(i, Partner::Fixed(0))];
        assert!(m.points.len() >= 4, "{:?}", m.points.len());
        // Every point now held was heard once when it came.
        for p in &m.points {
            let heard = events
                .iter()
                .filter(|e| length(sub(e.point, p.position)) < 3.0)
                .count();
            assert!(heard >= 1, "{p:?}");
        }
        let keys: Vec<u64> = m.points.iter().map(|p| p.key).collect();
        // Resting: the same points, no more events.
        for _ in 0..120 {
            w.update(&c, STEP);
            assert!(w.take_contacts().is_empty());
        }
        if let Some(m) = w.manifolds.get(&(i, Partner::Fixed(0))) {
            let now: Vec<u64> = m.points.iter().map(|p| p.key).collect();
            assert_eq!(now, keys);
        }
    }

    #[test]
    fn contact_points_carry_the_managers_properties() {
        // A bottle-like box (0.5, 0.4) on a surface of 2.5 and 0.4.
        let mut c = Collider::new();
        c.add_solid_surface(
            &[
                [-100.0, -100.0, 0.0],
                [100.0, -100.0, 0.0],
                [100.0, 100.0, 0.0],
                [-100.0, 100.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
            (0.0, 0, crate::NO_MATERIAL),
            Some(Surface {
                friction: 2.5,
                restitution: 0.4,
            }),
        );
        let mut w = RigidWorld::new();
        let i = w.add(crate_(1, [3.0; 3], 1.0), (I3, [0.0, 0.0, 3.8]));
        w.wake(i);
        w.update(&c, STEP);
        let m = w.manifolds.values().next().expect("points");
        let p = m.points[0];
        assert_eq!(crate::havok::UFLOAT8[p.props.friction as usize], 1.14);
        assert_eq!(p.props.restitution, 51);
        // Paired as they came: 1, 3, 1, 3.
        assert_eq!(m.points[1].props.flags & crate::manifold::POINT_PAIRED, 2);
        assert!(p.normal[2] > 0.99 && p.distance.abs() < 1.0, "{p:?}");
    }
    /// A ball of `radius` and `mass` (solid sphere inertia).
    fn ball(reference: u32, radius: f32, mass: f32) -> RigidSetup {
        let k = 0.4 * mass * radius * radius;
        RigidSetup {
            shapes: vec![Shape::Sphere {
                center: [0.0; 3],
                radius,
            }],
            inertia: [[k, 0.0, 0.0], [0.0, k, 0.0], [0.0, 0.0, k]],
            linear_damping: 0.0,
            angular_damping: 0.0,
            ..crate_(reference, [radius; 3], mass)
        }
    }

    #[test]
    fn a_box_at_rest_doesnt_drift() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(1, [10.0; 3], 5.0), (I3, [0.0, 0.0, 10.7]));
        let start = w.bodies[i].center();
        for _ in 0..1000 {
            w.wake(i);
            w.update(&c, STEP);
        }
        let now = w.bodies[i].center();
        assert!(length(sub(now, start)) < 0.1, "{start:?} -> {now:?}");
        let (r, _) = w.bodies[i].pose();
        assert!(r[2][2] > 0.9999, "{r:?}");
    }

    #[test]
    fn a_ball_rolls_down_a_slope() {
        // A 20° slope rising toward +x: the ball rolls down toward -x, its
        // spin matching its speed (no slipping at friction 0.5).
        let a = 20f32.to_radians();
        let (s, co) = (a.sin(), a.cos());
        let mut c = Collider::new();
        let p = |x: f32, y: f32| [x * co, y, x * s];
        c.add(
            &[
                p(-2000.0, -500.0),
                p(2000.0, -500.0),
                p(2000.0, 500.0),
                p(-2000.0, 500.0),
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        let mut w = RigidWorld::new();
        let n = [-s, 0.0, co];
        let i = w.add(ball(1, 10.0, 5.0), (I3, scale(n, 10.0)));
        w.wake(i);
        for _ in 0..60 {
            w.update(&c, STEP);
        }
        let b = &w.bodies[i];
        let v = b.velocity();
        let spin = length(b.spin()) * 10.0;
        assert!(v[0] < -50.0, "{v:?}");
        assert!(
            (spin - length(v)).abs() < 0.25 * length(v),
            "spin {spin} speed {}",
            length(v)
        );
        // Still on the slope.
        assert!(dot(sub(b.center(), [0.0; 3]), n) < 11.5);
    }

    #[test]
    fn a_dropped_ball_comes_back_up_a_little() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(ball(1, 5.0, 1.0), (I3, [0.0, 0.0, 200.0]));
        w.wake(i);
        let mut fastest_down = 0.0f32;
        let mut fastest_up = 0.0f32;
        for _ in 0..120 {
            w.update(&c, STEP);
            let vz = w.bodies[i].velocity()[2];
            if fastest_up == 0.0 {
                fastest_down = fastest_down.min(vz);
            }
            fastest_up = fastest_up.max(vz);
        }
        // It comes back up, much slower than it came down: Havok's PSI
        // restitution is a one-step distance offset on the new point
        // (`00d92900`), which the solver's integrated velocities mostly
        // spend on moving the ball out (about 2% here).
        assert!(
            fastest_up > 0.01 * -fastest_down,
            "{fastest_up} {fastest_down}"
        );
        assert!(fastest_up < -fastest_down, "{fastest_up} {fastest_down}");
    }

    /// A body shot at a floor of no thickness, its step longer than the
    /// body (`first_touch`): with the simplified time of impact (quality 3,
    /// the game's clutter) it is set where it touched, and rests on the
    /// floor; as debris (quality 2, none against the world) it is carried
    /// through.
    fn shot_at_the_floor(quality: u8, setup: RigidSetup, velocity: Vec3) -> f32 {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(RigidSetup { quality, ..setup }, (I3, [0.0, 0.0, 60.0]));
        w.set_velocity(i, velocity, [0.0; 3]);
        w.wake(i);
        let mut lowest = f32::MAX;
        for _ in 0..90 {
            w.update(&c, STEP);
            lowest = lowest.min(w.bodies[i].center()[2]);
        }
        lowest
    }

    #[test]
    fn a_ball_shot_fast_at_the_floor_doesnt_pass_through() {
        for speed in [1500.0, 4000.0, 9000.0] {
            let v = [0.0, 0.0, -speed];
            let lowest =
                shot_at_the_floor(crate::continuous::DEBRIS_SIMPLE_TOI, ball(1, 5.0, 1.0), v);
            assert!(lowest > -1.0, "{speed}: {lowest}");
        }
        // The same without the time of impact goes through.
        let v = [0.0, 0.0, -9000.0];
        let lowest = shot_at_the_floor(crate::continuous::DEBRIS, ball(1, 5.0, 1.0), v);
        assert!(lowest < -10.0, "{lowest}");
    }

    #[test]
    fn a_thin_rod_dropped_from_carrying_height_rests_on_the_floor() {
        // The ophthalmoscope by Doc Mitchell's bed (`Opthamoscope01.nif`:
        // one hull 6 × 2 × 14, radius 0.7, quality 3) let go of 70 units
        // up, upright, lying and tilted, still or turning.
        let rod = crate_(1, [3.0, 1.0, 7.0], 1.0);
        let c = floor(0.0);
        let s = 0.5f32.sqrt();
        let poses: [Mat3; 3] = [
            I3,
            [[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]],
            [[1.0, 0.0, 0.0], [0.0, s, -s], [0.0, s, s]],
        ];
        for (k, r) in poses.iter().enumerate() {
            for spin in [[0.0; 3], [3.0, 1.0, 0.5]] {
                let mut w = RigidWorld::new();
                let i = w.add(rod.clone(), (*r, [0.0, 0.0, 70.0]));
                w.set_velocity(i, [0.0; 3], spin);
                w.wake(i);
                let mut lowest = f32::MAX;
                for _ in 0..240 {
                    w.update(&c, STEP);
                    lowest = lowest.min(w.bodies[i].center()[2]);
                }
                assert!(lowest > 0.0, "pose {k}, spin {spin:?}: lowest {lowest}");
            }
        }
    }

    #[test]
    fn a_crate_shot_fast_and_slantwise_at_the_floor_doesnt_pass_through() {
        for speed in [2000.0, 6000.0] {
            let v = [speed * 0.3, speed * 0.1, -speed];
            let lowest = shot_at_the_floor(
                crate::continuous::DEBRIS_SIMPLE_TOI,
                crate_(1, [4.0, 6.0, 3.0], 2.0),
                v,
            );
            assert!(lowest > -1.0, "{speed}: {lowest}");
        }
    }

    #[test]
    fn a_sliding_box_stops_by_friction() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let mut setup = crate_(1, [10.0; 3], 5.0);
        setup.linear_damping = 0.0;
        let i = w.add(setup, (I3, [0.0, 0.0, 10.7]));
        w.wake(i);
        w.update(&c, STEP);
        w.set_velocity(i, [300.0, 0.0, 0.0], [0.0; 3]);
        for _ in 0..240 {
            w.update(&c, STEP);
        }
        // About v² ÷ (2 μ g) with μ = √(0.5 × 0.5) (the table's 0.514).
        let x = w.bodies[i].center()[0];
        let want = 300.0 * 300.0 / (2.0 * 0.514 * crate::GRAVITY);
        assert!(x > 0.5 * want && x < 1.5 * want, "{x} vs {want}");
        assert!(length(w.bodies[i].velocity()) < 1.0);
    }
    #[test]
    fn the_grab_spring_carries_a_body_to_its_target() {
        let c = floor(0.0);
        let mut w = RigidWorld::new();
        let i = w.add(crate_(8, [3.0; 3], 1.0), (I3, [0.0, 0.0, 3.7]));
        let s = crate::grab::GrabSettings::default();
        let (damping, elasticity, object_damping, max) = crate::grab::spring_values(10, &s);
        w.spring = Some(Spring {
            body: i,
            local: [0.0; 3],
            target: [20.0, 0.0, 60.0],
            damping,
            elasticity,
            max_relative_force: max,
            object_damping,
        });
        for _ in 0..180 {
            w.update(&c, 1.0 / 60.0);
        }
        // Held under the target, sagging by gravity × step² ÷ elasticity
        // (about 0.9 units).
        let at = w.bodies[i].center();
        assert!(length(sub(at, [20.0, 0.0, 60.0])) < 2.0, "{at:?}");
        assert!(!w.bodies[i].asleep && w.bodies[i].moved);
        // Let go: it falls back to the floor.
        w.spring = None;
        for _ in 0..240 {
            w.update(&c, 1.0 / 60.0);
        }
        assert!(w.bodies[i].center()[2] < 5.0, "{:?}", w.bodies[i].center());
        // Removing the held body ends the grab; others shift down.
        let j = w.add(crate_(9, [3.0; 3], 1.0), (I3, [50.0, 0.0, 3.7]));
        w.spring = Some(Spring {
            body: j,
            local: [0.0; 3],
            target: [0.0; 3],
            damping,
            elasticity,
            max_relative_force: max,
            object_damping,
        });
        w.remove(8);
        assert_eq!(w.spring.map(|s| s.body), Some(0));
        w.remove(9);
        assert!(w.spring.is_none());
    }
}
