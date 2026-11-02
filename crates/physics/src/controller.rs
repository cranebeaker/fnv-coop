//! Bethesda's character controller around Havok's proxy
//! (`bhkCharacterController`, Xbox PDB, 0x650 bytes): its listener, which
//! turns the proxy's surface constraints into walkable ground, steps and
//! walls (`processConstraintsCallback` `00c711d0`), the support check
//! (`hkpCharacterProxy::checkSupportWithCollector` `00cae900` through
//! `00c6cef0`), the character states (on the ground `00cd4800`, jumping
//! `00cd4280`, in the air `00cd3fb0`, sharing Havok's movement util
//! `00d6aef0`) and the move (`bhkCharacterController::Move` `00c73170`,
//! vtable +0xc8).
//!
//! Translated from FalloutNV.exe 1.4.0.525 (addresses per function); names
//! and layouts from the Xbox prototype's PDB (ADR-0002). Units: Havok units
//! and seconds inside; game units for the feet and fall heights.

use crate::character_cd::{Body, Hull, RootCdPoint};
use crate::proxy::{Listener, Proxy, ProxySettings};
use crate::simplex::{self, SimplexSolverInput, SurfaceConstraintInfo};
use crate::vec::*;
use crate::{layers, Collider, Vec3, ANY_LAYER, GRAVITY, HAVOK_UNIT};

/// `bhkCharacterListener::iFlags` (+0x414) bits used here.
pub mod flags {
    /// The controller's shape lies down (a creature longer than tall).
    pub const LYING: u32 = 0x1;
    /// Set by the listener while a step turned into a ramp is met.
    pub const ON_STEP: u32 = 0x4;
    /// The move wants to go somewhere (not standing still).
    pub const MOVING: u32 = 0x8;
    /// Landed this update (set by the in-air state, read by the fall
    /// damage `008a62b0`).
    pub const LANDED: u32 = 0x80;
    /// Supported, from the support check.
    pub const SUPPORTED: u32 = 0x100;
    /// Something counted as support was met.
    pub const HAS_SUPPORT: u32 = 0x200;
    /// Walkable ground (or a step) was met.
    pub const ON_WALKABLE: u32 = 0x400;
    /// Swimming or flying: steep surfaces aren't walled off, the move's
    /// height is kept.
    pub const NO_WALLS: u32 = 0x800;
    /// Set by the jumping state until the jump is over.
    pub const JUMPING: u32 = 0x2000;
    /// Inside the support check.
    pub const CHECKING_SUPPORT: u32 = 0x10000;
    /// Resizing: one cast pass an update.
    pub const RESIZING: u32 = 0x10_0000;
    /// An `ANIM_STATIC` body was met.
    pub const ON_ANIM_STATIC: u32 = 0x2000_0000;
}

/// The character states (`hkpCharacterStateType`, with Bethesda's
/// registrations in `00c6da50`: 0 on ground, 1 jumping, 2 in air; 3
/// climbing, 4 flying, 5 swimming and 6 projectile aren't used here).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    OnGround = 0,
    Jumping = 1,
    InAir = 2,
}

/// `hkpSurfaceInfo::SupportedState` (Xbox PDB).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Supported {
    Unsupported = 0,
    Sliding = 1,
    Supported = 2,
}

/// What the support check found (`hkpSurfaceInfo`, controller +0x480).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceInfo {
    pub supported: Supported,
    pub normal: Vec3,
    pub velocity: Vec3,
}

/// Steepest walkable ground: 47 degrees (the float 47.0 at `011b0148`,
/// × 0.017453292 in `00c741e0`, its cosine kept at +0x430 by `00c6c7a0`).
pub const MAX_SLOPE_DEGREES: f32 = 47.0;

/// The extra down static friction given to walkable ground (`0x40b8a3d8`,
/// 5.77) and to fixed or keyframed `ANIM_STATIC` bodies (`0x3f13b646`,
/// 0.577) by `00c711d0`.
pub const WALKABLE_EXTRA_DOWN_FRICTION: f32 = 5.77;
pub const ANIM_STATIC_EXTRA_DOWN_FRICTION: f32 = 0.577;

/// The on-ground state's largest velocity change an update (500 Havok
/// units a second, `01013d84`) and the in-air state's
/// (`bhkCharacterStateInAir::fMaxVelocityDelta`, 2000, `00cd3f90`).
pub const GROUND_MAX_VELOCITY_DELTA: f32 = 500.0;
pub const AIR_MAX_VELOCITY_DELTA: f32 = 2000.0;

/// The player's running speed the game keeps (`01267bc4`; zero in the
/// executable until a game loads): the move's speed is divided by it into
/// `fSpeedPct` (`00c73170`, the only reader). `0055d760` (a game loading
/// or starting) sets it for the player (`0055e230`, only when above
/// zero) to `00647f00(player's actor values, 0, 0, 0, 1, 0, 0)`:
/// SpeedMult (actor value 21) ÷ 100 × `fMoveBaseSpeed` (`011d0448`; 77 in
/// `FalloutNV.esm`) × the legs' condition (`fMoveOneCrippledLegSpeedMult`
/// 0.85, `fMoveTwoCrippledLegsSpeedMult` 0.75 for one or two crippled legs,
/// `00647d10`) × `fMoveRunMult` (`011d0898`; 4). See
/// [`set_reference_speed`] (`world::animation::run_speed` works it out).
/// This default is that value for SpeedMult 100 and sound legs.
pub const REFERENCE_SPEED: f32 = 308.0;

static REFERENCE_SPEED_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x439a_0000);

/// The running speed `fSpeedPct` is measured against now (`01267bc4`).
pub fn reference_speed() -> f32 {
    f32::from_bits(REFERENCE_SPEED_BITS.load(std::sync::atomic::Ordering::Relaxed))
}

/// `0055e230`: keeps the player's running speed, unless it isn't above
/// zero.
// Translated from 0055e230 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn set_reference_speed(speed: f32) {
    if speed > 0.0 {
        REFERENCE_SPEED_BITS.store(speed.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }
}

/// The listener's state (`bhkCharacterListener`, controller +0x410).
#[derive(Debug, Clone, PartialEq)]
pub struct ListenerState {
    pub flags: u32,
    /// Cosine of the steepest walkable ground (+0x430).
    pub max_slope_cos: f32,
    /// Havok layer of the last supporting body (+0x438).
    pub support_layer: u8,
    /// The supporting surfaces' summed normal, normalized (+0x450); the
    /// up vector when none.
    pub support_normal: Vec3,
    /// Step height, Havok units (+0x470).
    pub step_height: f32,
    /// How far ahead a step is looked for (+0x474): the step height ÷
    /// tan(max slope) (`00c6c7a0`).
    pub cast_depth: f32,
    /// Whether every support was walled off (+0x478).
    pub all_surfaces_stopped: bool,
    /// How many constraints counted as support (+0x608).
    pub support_count: u32,
}

/// What [`ListenerState::process_constraints`] reads besides the
/// constraints.
#[derive(Clone, Copy)]
pub struct ListenerInput<'a> {
    pub collider: &'a Collider,
    pub state: State,
    /// `fSpeedPct` (+0x568).
    pub speed_pct: f32,
    /// Feet, game units: the controller's base position (`00c6e300`, the
    /// phantom less its rotation centre).
    pub feet: Vec3,
}

/// The Havok layer of a contact's body (the filter's low 7 bits):
/// triangles carry their body's, unknown ones are taken as `STATIC`; other
/// characters are on the controller layer (30).
pub fn body_layer(collider: &Collider, body: Body) -> u8 {
    match body {
        Body::Triangle(t) => match collider.layer(t) {
            ANY_LAYER => layers::layer::STATIC,
            l => l & 0x7f,
        },
        Body::Person(_) => layers::layer::CHAR_CONTROLLER,
    }
}

/// A 4-vector scaled to unit length in x, y, z (w scaled with them), as
/// the game's `rsqrtss` and Newton step do; zero stays zero.
fn normalize4(v: [f32; 4]) -> [f32; 4] {
    let l2 = v[2] * v[2] + v[1] * v[1] + v[0] * v[0];
    if l2 == 0.0 {
        return [0.0; 4];
    }
    let inv = 1.0 / l2.sqrt();
    [v[0] * inv, v[1] * inv, v[2] * inv, v[3] * inv]
}

impl ListenerState {
    /// `00c6c7a0`: max slope 47 degrees, step height (Havok units), the
    /// cast depth from them.
    pub fn new(step_height: f32) -> Self {
        let slope = MAX_SLOPE_DEGREES * 0.017_453_292;
        let cos = slope.cos();
        Self {
            flags: 0,
            max_slope_cos: cos,
            support_layer: 0,
            support_normal: [0.0, 0.0, 1.0],
            step_height,
            cast_depth: (step_height / slope.sin()) * cos,
            all_surfaces_stopped: false,
            support_count: 0,
        }
    }

    /// `00c711d0` (`bhkCharacterController::processConstraintsCallback`,
    /// Xbox PDB): for each manifold point's constraint —
    ///
    /// - another controller (a phantom on layer 30): a vertical wall (its
    ///   normal's z cleared and renormalized); nothing else;
    /// - an `ANIM_STATIC` fixed or keyframed body: extra down friction
    ///   0.577; `PROPS`: support straight up, no dynamic friction;
    /// - facing up at least the max slope's cosine (0.682): walkable, extra
    ///   down friction 5.77 (unless flag 0x8);
    /// - steeper, a step ([`ListenerState::is_step`]): a ramp, its normal
    ///   (x, y, 1) normalized, no dynamic friction;
    /// - steeper, not a step: under 0.2 it doesn't count as support, and a
    ///   vertical copy of it is added (at most four an update);
    ///
    /// on the ground each constraint's dynamic friction is scaled by the
    /// move's speed fraction; whatever counts as support adds its (maybe
    /// changed) normal to the support normal. Afterwards: all surfaces
    /// stopped when every support got a wall.
    ///
    /// Not translated (labelled): the hurtful-body damage branch (flags
    /// 0x10000 and 0x200000 with a hurtful body), the keyframed-platform
    /// cast (a normal above 0.985 on a keyframed body, `01267bb4`), the
    /// collision box's rotation-centre test (flag 0x20), the support
    /// material (`00c6e980`: material and support velocity, kept for
    /// footsteps and moving platforms), and clutter, weapons and
    /// projectiles (layers 4–6: the walking collider holds no dynamic
    /// bodies).
    pub fn process_constraints(
        &mut self,
        input: &ListenerInput,
        manifold: &[RootCdPoint],
        constraints: &mut Vec<SurfaceConstraintInfo>,
    ) {
        self.flags &= !flags::ON_STEP;
        self.support_normal = [0.0, 0.0, 1.0];
        self.support_count = 0;
        let mut walls_added = 0;
        let mut walled_supports = 0;
        let mut first_support = true;
        let mut anim_static = false;
        let count = manifold.len().min(constraints.len());
        for i in 0..count {
            let point = &manifold[i];
            let layer = body_layer(input.collider, point.body);
            let mut nz = point.normal[2];
            anim_static |= layer == layers::layer::ANIM_STATIC;
            let mut not_floating = self.flags & flags::NO_WALLS == 0;
            let mut support = constraints[i].plane;
            if let Body::Person(_) = point.body {
                // A phantom: only the layer switch (passed-through
                // controllers aren't in the collider's people at all).
                let c = &mut constraints[i];
                c.plane[2] = 0.0;
                c.plane = normalize4(c.plane);
                continue;
            }
            match layer {
                layers::layer::ANIM_STATIC => {
                    // The walking collider's bodies are fixed (motion type
                    // 5) or keyframed (4).
                    constraints[i].extra_down_static_friction = ANIM_STATIC_EXTRA_DOWN_FRICTION;
                }
                layers::layer::CLUTTER | layers::layer::WEAPON | layers::layer::PROJECTILE => {
                    // Dynamic bodies: counted as support straight up and
                    // never walled off or stepped onto; one lighter than
                    // `fMoveLimitMass` (95) also has its constraint's
                    // velocity cleared (the walking collider's clutter
                    // triangles carry no velocity, so that changes
                    // nothing here). The proxy pushes them
                    // (`applySurfaceInteractions`, `crate::proxy` and
                    // `crate::rigid`).
                    if self.flags & flags::CHECKING_SUPPORT == 0 {
                        nz = 1.0;
                        support = [0.0, 0.0, 1.0, 0.0];
                    }
                    not_floating = false;
                }
                layers::layer::PROPS => {
                    if self.flags & flags::CHECKING_SUPPORT == 0 {
                        nz = 1.0;
                        constraints[i].dynamic_friction = 0.0;
                        support = [0.0, 0.0, 1.0, 0.0];
                    }
                    not_floating = false;
                }
                _ => {}
            }
            if nz < 0.0 {
                continue;
            }
            let mut counts = true;
            if self.max_slope_cos <= nz {
                self.flags |= flags::ON_WALKABLE;
                if self.flags & flags::MOVING == 0 {
                    constraints[i].extra_down_static_friction = WALKABLE_EXTRA_DOWN_FRICTION;
                }
            } else if self.is_step(input, point) {
                if self.flags & 0x1800 == 0 {
                    constraints[i].dynamic_friction = 0.0;
                }
                self.flags |= flags::ON_WALKABLE | flags::ON_STEP;
                support = normalize4([support[0], support[1], 1.0, support[3]]);
                constraints[i].plane = support;
            } else {
                if (not_floating || self.flags & flags::NO_WALLS != 0) && nz < 0.2 {
                    counts = false;
                }
                if walls_added < 4 && not_floating {
                    let mut wall = constraints[i];
                    wall.velocity = [0.0; 4];
                    wall.plane[2] = 0.0;
                    wall.plane = normalize4(wall.plane);
                    constraints.push(wall);
                    walls_added += 1;
                    if counts {
                        walled_supports += 1;
                    }
                }
            }
            if input.state == State::OnGround {
                constraints[i].dynamic_friction *= input.speed_pct;
            }
            if counts {
                self.flags |= flags::HAS_SUPPORT;
                self.support_count += 1;
                self.support_layer = layer;
                let n = [support[0], support[1], support[2]];
                if first_support {
                    first_support = false;
                    self.support_normal = n;
                } else {
                    self.support_normal = add(self.support_normal, n);
                }
            }
        }
        if anim_static {
            self.flags |= flags::ON_ANIM_STATIC;
        } else {
            self.flags &= !flags::ON_ANIM_STATIC;
        }
        self.all_surfaces_stopped = self.support_count == walled_supports;
        let n = self.support_normal;
        let l2 = n[2] * n[2] + n[1] * n[1] + n[0] * n[0];
        if l2 != 0.0 {
            self.support_normal = scale(n, 1.0 / l2.sqrt());
        }
    }

    /// Whether a contact steeper than the max slope is a step to ride up
    /// (`00c6eb00`, `bhkCharacterController::IsStep`, Xbox PDB), for an
    /// upright controller: the contact no higher than a third of the step
    /// above the step's top (feet + step height), and a segment at the
    /// step's top from over the contact `cast_depth` further on (away from
    /// the character) clear of the contacted triangle (capsule radius
    /// 0.05 against the triangle's own convex radius: `00cd6520`, both end
    /// points' distances not negative).
    ///
    /// The game takes that path for mesh shapes (a contact with a shape
    /// key); for convex shapes it casts a ray from the feet's x and y at
    /// the step's top toward the contact, radius + cast depth long, and a
    /// hit means no step. The walking collider holds every shape as
    /// triangles, so the contacted triangle is tested for both (a
    /// stand-in for convex shapes). The stair-material callback
    /// (`01267bc8`) is never set on PC.
    pub fn is_step(&self, input: &ListenerInput, point: &RootCdPoint) -> bool {
        let Body::Triangle(t) = point.body else {
            return false;
        };
        let k = 1.0 / HAVOK_UNIT;
        let c = scale(input.feet, k);
        let p = scale(point.position, k);
        let (dx, dy) = (c[0] - p[0], c[1] - p[1]);
        let l2 = dy * dy + dx * dx;
        let inv = if l2 == 0.0 { 0.0 } else { 1.0 / l2.sqrt() };
        let dir = [dx * inv, dy * inv];
        let top = c[2] + self.step_height;
        let h = p[2] - top;
        if self.step_height / 3.0 < h {
            return false;
        }
        let a = [p[0], p[1], top];
        let b = [
            p[0] - dir[0] * self.cast_depth,
            p[1] - dir[1] * self.cast_depth,
            top,
        ];
        let corners = input.collider.triangle(t).map(|v| scale(v, k));
        let radius = input.collider.shell(t) * k;
        let clear = |q: Vec3| {
            let on = crate::closest_on_triangle(q, corners[0], corners[1], corners[2]);
            length(sub(q, on)) - 0.05 - radius >= 0.0
        };
        // A segment through the triangle has a negative distance.
        let (on_segment, on_triangle) =
            crate::segment_triangle_closest(a, b, corners[0], corners[1], corners[2]);
        let pierced = length(sub(on_segment, on_triangle)) == 0.0;
        !pierced && clear(a) && clear(b)
    }
}

/// The proxy's listener for one update.
struct Hook<'a, 'b> {
    state: &'a mut ListenerState,
    input: ListenerInput<'b>,
}

impl Listener for Hook<'_, '_> {
    fn process_constraints(
        &mut self,
        manifold: &[RootCdPoint],
        constraints: &mut Vec<SurfaceConstraintInfo>,
    ) {
        self.state
            .process_constraints(&self.input, manifold, constraints);
    }
}

/// Havok's movement util (`00d6aef0`, `hkpCharacterMovementUtil`): the
/// velocity moved toward `desired` (in the surface's frame: x along the
/// surface away from `forward`, y across it) by `gain` of the gap, the gap
/// cut to `max_delta`, relative to the surface's velocity.
#[allow(clippy::too_many_arguments)]
pub fn calculate_movement(
    gain: f32,
    forward: Vec3,
    up: Vec3,
    surface_normal: Vec3,
    current: Vec3,
    desired: Vec3,
    max_delta: f32,
    surface_velocity: Vec3,
) -> Vec3 {
    let n = cross(forward, up);
    let l2 = n[2] * n[2] + n[1] * n[1] + n[0] * n[0];
    if l2 < 1.192_092_9e-7 {
        return current;
    }
    let n = scale(n, 1.0 / l2.sqrt());
    let f = normalize(cross(n, surface_normal));
    let r = normalize(cross(f, surface_normal));
    let rel = sub(current, surface_velocity);
    // Into the frame (rows f, r, surface normal).
    let local = [dot(f, rel), dot(r, rel), dot(surface_normal, rel)];
    let d = sub(desired, local);
    let d2 = d[2] * d[2] + d[1] * d[1] + d[0] * d[0];
    let step = if gain * d2 * gain <= max_delta * max_delta {
        scale(d, gain)
    } else {
        scale(d, (1.0 / d2.sqrt()) * max_delta)
    };
    let local = add(step, local);
    let world = add(
        add(scale(f, local[0]), scale(r, local[1])),
        scale(surface_normal, local[2]),
    );
    add(surface_velocity, world)
}

/// A controller (the parts of `bhkCharacterController` this uses).
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub proxy: Proxy,
    pub listener: ListenerState,
    /// `m_currentState` (+0x3f0).
    pub state: State,
    /// `eWantState` (+0x520; `None` is 0xb, no change wanted).
    pub want: Option<State>,
    pub surface: SurfaceInfo,
    /// `OutVelocity` (+0x4f0), Havok units a second.
    pub out_velocity: Vec3,
    /// `fJumpHeight` (+0x540), Havok units.
    pub jump_height: f32,
    /// `fFallStartHeight` (+0x544): where the feet were on leaving the
    /// ground, game units.
    pub fall_start: f32,
    /// `fFallTime` (+0x548).
    pub fall_time: f32,
    /// `fGravity` (+0x54c): the world gravity's multiplier.
    pub gravity: f32,
    /// `bFakeSupport` (+0x610) and `FakeSupportStart` (+0x620, feet).
    pub fake_support: Option<Vec3>,
    /// `LastManifold` (+0x63c): up to five points of the last update's
    /// manifold (never shrunk).
    pub last_manifold: Vec<RootCdPoint>,
}

/// The world gravity, Havok units a second squared (`00f4b550`).
pub const WORLD_GRAVITY: Vec3 = [0.0, 0.0, -98.1];

/// The up vector (`UpVec` +0x4c0) and the forward vector a controller
/// without pitch or roll uses (`ForwardVec` +0x4d0: (0, −1, 0) turned by
/// the phantom's rotation, which is none without flags 0x1, 0x2000000,
/// 0x4000000; `00c73170`).
const UP: Vec3 = [0.0, 0.0, 1.0];
const FORWARD: Vec3 = [0.0, -1.0, 0.0];

/// One move's input (`00c73170`'s second argument): the frame's time, the
/// wanted displacement (game units) and the wanted speed (game units a
/// second, for `fSpeedPct`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveInput {
    pub dt: f32,
    pub displacement: Vec3,
    pub speed: f32,
    /// The in-air state's gain (`fAcrobatics` × 0.3 + `01267bbc`).
    pub air_gain: f32,
}

impl Controller {
    /// A controller standing still, on the ground (the cinfo's start state
    /// is 0, `00c6da50`), with the game's proxy settings.
    pub fn new(step_height: f32, gravity: f32) -> Self {
        Self {
            proxy: Proxy::new(ProxySettings::GAME),
            listener: ListenerState::new(step_height),
            state: State::OnGround,
            want: None,
            surface: SurfaceInfo {
                supported: Supported::Supported,
                normal: [0.0, 0.0, 1.0],
                velocity: [0.0; 3],
            },
            out_velocity: [0.0; 3],
            jump_height: 0.0,
            fall_start: 0.0,
            fall_time: 0.0,
            gravity,
            fake_support: None,
            last_manifold: Vec::new(),
        }
    }

    /// Asks for a jump of `height` game units (`bhkCharacterController::Jump`
    /// through `00c6d4b0`: the wanted state jumping and `fJumpHeight`).
    pub fn jump(&mut self, height: f32) {
        self.want = Some(State::Jumping);
        self.jump_height = height / HAVOK_UNIT;
    }

    /// The support check (`00c6cef0` → `00cafd00` → `00cae900`,
    /// `hkpCharacterProxy::checkSupportWithCollector`): the manifold's
    /// constraints (as the listener changes them; their velocities kept
    /// aside and cleared) solved for a move straight down at 1 unit a
    /// second for 1/60 s. Unsupported if nothing changed that move;
    /// supported if it stopped or turned no further from straight down
    /// than the proxy's max slope (never, at pi/2); sliding otherwise. The
    /// surface's normal and velocity are the touched constraints' facing
    /// up (more than 0.08 against the move), summed and averaged; none of
    /// them, unsupported. The manifold isn't refreshed
    /// (`m_refreshManifoldInCheckSupport` false).
    fn check_support(&mut self, input: ListenerInput) -> SurfaceInfo {
        let dir = scale(UP, -1.0);
        let mut constraints: Vec<SurfaceConstraintInfo> = self
            .proxy
            .manifold
            .iter()
            .map(|p| self.proxy.extract_surface_constraint_info(p, 0.0))
            .collect();
        self.listener
            .process_constraints(&input, &self.proxy.manifold, &mut constraints);
        let saved: Vec<[f32; 4]> = constraints.iter().map(|c| c.velocity).collect();
        for c in &mut constraints {
            c.velocity = [0.0; 4];
        }
        let s = self.proxy.settings;
        let m = s.max_character_speed_for_solver;
        let out = simplex::solve(&SimplexSolverInput {
            position: [0.0; 4],
            velocity: [dir[0], dir[1], dir[2], 0.0],
            max_surface_velocity: [m; 4],
            up_vector: [UP[0], UP[1], UP[2], 0.0],
            delta_time: 1.0 / 60.0,
            min_delta_time: 1.0 / 60.0,
            constraints: &constraints,
        });
        let v = [out.velocity[0], out.velocity[1], out.velocity[2]];
        let unchanged = (0..3).all(|k| (v[k] - dir[k]).abs() <= 0.001);
        let unsupported = SurfaceInfo {
            supported: Supported::Unsupported,
            normal: [0.0; 3],
            velocity: [0.0; 3],
        };
        if unchanged {
            return unsupported;
        }
        let v2 = v[2] * v[2] + v[1] * v[1] + v[0] * v[0];
        let supported = if v2 < 0.001 {
            Supported::Supported
        } else {
            let vn = scale(v, 1.0 / v2.sqrt());
            let a = dir[2] * vn[2] + dir[1] * vn[1] + dir[0] * vn[0];
            if s.max_slope_cosine * s.max_slope_cosine <= 1.0 - a * a {
                Supported::Supported
            } else {
                Supported::Sliding
            }
        };
        let mut count = 0;
        let mut normal = [0.0f32; 3];
        let mut velocity = [0.0f32; 3];
        for (i, c) in constraints.iter().enumerate() {
            let touched = out.plane_interactions.get(i).is_some_and(|p| p.touched);
            if !touched {
                continue;
            }
            let n = [c.plane[0], c.plane[1], c.plane[2]];
            if dir[2] * n[2] + dir[1] * n[1] + dir[0] * n[0] < -0.08 {
                count += 1;
                normal = add(normal, n);
                velocity = add(velocity, [saved[i][0], saved[i][1], saved[i][2]]);
            }
        }
        if count == 0 {
            return unsupported;
        }
        SurfaceInfo {
            supported,
            normal: normalize(normal),
            velocity: scale(velocity, 1.0 / count as f32),
        }
    }

    /// Changes to the wanted state, if any (`00c6cba0`).
    fn transition(&mut self) {
        if let Some(w) = self.want.take() {
            self.state = w;
        }
    }

    /// Runs the current state's update (state vtable +0x20).
    fn update_state(&mut self, ctx: &StateInput) {
        match self.state {
            State::OnGround => self.on_ground(ctx),
            State::Jumping => self.jumping(ctx),
            State::InAir => self.in_air(ctx),
        }
    }

    /// `00cd4800`, the on-ground state. Not supported: the vertical
    /// velocity cleared (`bKillVelocityOnLaunch`), the fall measured from
    /// here, into the air at once. Otherwise the velocity moves to the
    /// wanted one in the support's plane (gain 1, at most 500 a update),
    /// keeping its vertical part, which gets gravity × 0.5 × the speed
    /// fraction × dt; a wanted jump starts only on walkable ground.
    fn on_ground(&mut self, ctx: &StateInput) {
        if self.listener.flags & flags::SUPPORTED == 0 {
            if self.fake_support.is_none() {
                self.out_velocity[2] = 0.0;
                self.fall_start = ctx.feet[2];
                self.want = Some(State::InAir);
                self.transition();
                self.update_state(ctx);
                return;
            }
        } else if let Some(start) = self.fake_support {
            let d = length(sub(
                scale(ctx.feet, 1.0 / HAVOK_UNIT),
                scale(start, 1.0 / HAVOK_UNIT),
            ));
            if 64.0 * (1.0 / HAVOK_UNIT) < d || self.want.is_some() {
                self.fake_support = None;
            }
        }
        let old_z = self.out_velocity[2];
        let desired = [ctx.direction[1], ctx.direction[0], 0.0];
        self.out_velocity = calculate_movement(
            1.0,
            FORWARD,
            UP,
            self.listener.support_normal,
            self.out_velocity,
            desired,
            GROUND_MAX_VELOCITY_DELTA,
            self.surface.velocity,
        );
        self.out_velocity[2] = old_z;
        let g = self.gravity * 0.5 * ctx.speed_pct;
        self.out_velocity = add(self.out_velocity, scale(WORLD_GRAVITY, g * ctx.dt));
        match self.want {
            None => {}
            Some(State::Jumping) if self.listener.flags & flags::ON_WALKABLE == 0 => {}
            Some(_) => self.transition(),
        }
    }

    /// `00cd4280`, the jumping state: up at √(2 |g| h) added to the proxy's
    /// own velocity (its horizontal part kept), the fall measured from
    /// here, then into the air (whose update runs at once).
    fn jumping(&mut self, ctx: &StateInput) {
        if matches!(self.want, Some(State::OnGround)) {
            self.transition();
            return;
        }
        self.listener.flags |= flags::JUMPING;
        let g = scale(WORLD_GRAVITY, self.gravity);
        let g2 = g[2] * g[2] + g[1] * g[1] + g[0] * g[0];
        let inv = if g2 == 0.0 { 0.0 } else { 1.0 / g2.sqrt() };
        let speed = -((g2 * inv) * 2.0 * self.jump_height.abs()).sqrt();
        let jump = scale(g, speed * inv);
        let v = self.proxy.velocity;
        self.out_velocity = [v[0] + jump[0], v[1] + jump[1], jump[2]];
        self.fall_start = ctx.feet[2];
        self.want = Some(State::InAir);
        self.transition();
        self.listener.flags &= !0x20100;
        self.update_state(ctx);
    }

    /// `00cd3fb0`, the in-air state: supported, landed (flag 0x80) and to
    /// the ground at once; otherwise steered toward the wanted velocity
    /// across the up axis (the air gain, at most 2000 an update; the
    /// vertical part kept), and gravity added.
    fn in_air(&mut self, ctx: &StateInput) {
        if self.listener.flags & flags::SUPPORTED != 0 {
            self.want = Some(State::OnGround);
            self.listener.flags |= flags::LANDED;
        }
        if self.want.is_none() && 1.0 < self.fall_time && (self.fall_start - ctx.feet[2]) < 0.1 {
            self.want = Some(State::OnGround);
            self.fake_support = Some(ctx.feet);
        }
        match self.want {
            None => {}
            Some(State::Jumping) => self.want = None,
            Some(_) => {
                self.fall_time = 0.0;
                self.transition();
                return;
            }
        }
        if ctx.air_gain > 0.0 {
            let before = self.out_velocity;
            let desired = [ctx.direction[1], ctx.direction[0], 0.0];
            let mut v = calculate_movement(
                ctx.air_gain,
                FORWARD,
                UP,
                UP,
                self.out_velocity,
                desired,
                AIR_MAX_VELOCITY_DELTA,
                self.surface.velocity,
            );
            let off = 0.0 - dot(UP, v);
            v = add(v, scale(UP, off));
            let keep = dot(UP, before);
            v = add(v, scale(UP, keep));
            self.out_velocity = v;
        }
        self.out_velocity = add(
            self.out_velocity,
            scale(WORLD_GRAVITY, self.gravity * ctx.dt),
        );
        self.fall_time += ctx.dt;
    }

    /// One move (`00c73170`): the support check (when moving, not on the
    /// ground, or without a support normal), the state's update, the
    /// proxy's integration (skipped standing still in an unchanged
    /// manifold), then the manifold kept for the next move. `feet` (game
    /// units) is moved; `hull` and `centre` are the controller's shape and
    /// its centre's height above the feet.
    pub fn move_by(
        &mut self,
        collider: &Collider,
        hull: &Hull,
        centre: f32,
        feet: &mut Vec3,
        input: MoveInput,
    ) {
        let k = 1.0 / HAVOK_UNIT;
        self.listener.flags &= !0xc0;
        let d = input.displacement;
        let still = d.iter().all(|c| c.abs() <= 0.05);
        if still {
            self.listener.flags &= !flags::MOVING;
        } else {
            self.listener.flags |= flags::MOVING;
        }
        let speed_pct = input.speed / reference_speed();
        let dt = input.dt;
        let inv_dt = 1.0 / dt;
        let mut direction = scale(d, inv_dt);
        if self.listener.flags & flags::NO_WALLS == 0 {
            direction[2] = 0.0;
        }
        let direction = scale(direction, k);
        self.out_velocity = self.proxy.velocity;
        if still && self.state != State::InAir {
            self.out_velocity[2] = 0.0;
        }
        let input_l = ListenerInput {
            collider,
            state: self.state,
            speed_pct,
            feet: *feet,
        };
        let n = self.listener.support_normal;
        let no_normal = n.iter().all(|c| c.abs() <= 0.001);
        let moving = d != [0.0; 3];
        if self.state != State::OnGround || moving || no_normal {
            self.listener.flags &= !0x600;
            self.listener.flags |= flags::CHECKING_SUPPORT;
            self.surface = self.check_support(input_l);
            self.listener.flags &= !flags::CHECKING_SUPPORT;
            let f = self.listener.flags;
            let mut sup = self.surface.supported == Supported::Supported
                && !self.listener.all_surfaces_stopped;
            if f & flags::JUMPING == 0 {
                sup |= f & flags::HAS_SUPPORT != 0;
            } else if (!sup && f & flags::HAS_SUPPORT == 0) || self.out_velocity[2] <= 0.0 {
                self.listener.flags &= !flags::JUMPING;
                sup = false;
            } else {
                let n = self.listener.support_normal;
                let v = self.out_velocity;
                let l = length(v);
                let away = l > 0.0 && dot(scale(v, 1.0 / l), n) > 0.05;
                if l > 0.0
                    && sup
                    && f & flags::HAS_SUPPORT != 0
                    && self.listener.max_slope_cos <= n[2]
                    && !away
                {
                    self.listener.flags &= !flags::JUMPING;
                }
                sup = false;
            }
            if sup {
                self.listener.flags |= flags::SUPPORTED;
            } else {
                self.listener.flags &= !flags::SUPPORTED;
            }
        }
        let ctx = StateInput {
            dt,
            direction,
            speed_pct,
            air_gain: input.air_gain,
            feet: *feet,
        };
        self.update_state(&ctx);
        self.proxy.velocity = self.out_velocity;
        let unchanged = still
            && self.state != State::InAir
            && self.proxy.manifold.len() <= self.last_manifold.len()
            && self
                .proxy
                .manifold
                .iter()
                .zip(&self.last_manifold)
                .all(|(a, b)| same_point(a, b));
        if unchanged {
            self.out_velocity = [0.0; 3];
            self.proxy.velocity = [0.0; 3];
        } else {
            let iterations = if self.listener.flags & flags::RESIZING != 0 {
                1
            } else {
                4
            };
            self.proxy.settings.max_cast_iterations = iterations;
            let mut c = add(*feet, [0.0, 0.0, centre]);
            let mut hook = Hook {
                state: &mut self.listener,
                input: ListenerInput {
                    collider,
                    state: self.state,
                    speed_pct,
                    feet: *feet,
                },
            };
            self.proxy.integrate(collider, hull, &mut c, dt, &mut hook);
            self.proxy.settings.max_cast_iterations = 4;
            *feet = sub(c, [0.0, 0.0, centre]);
        }
        self.want = None;
        for (i, p) in self.proxy.manifold.iter().take(5).enumerate() {
            if i < self.last_manifold.len() {
                self.last_manifold[i] = *p;
            } else {
                self.last_manifold.push(*p);
            }
        }
    }
}

/// What a state's update reads besides the controller.
struct StateInput {
    dt: f32,
    /// `Direction` (+0x510): the wanted velocity, Havok units a second.
    direction: Vec3,
    speed_pct: f32,
    air_gain: f32,
    feet: Vec3,
}

/// Whether two contact points are the same within 0.001 (positions with
/// the fraction, normals with the distance) on the same body (`00c73170`'s
/// comparison with `LastManifold`).
fn same_point(a: &RootCdPoint, b: &RootCdPoint) -> bool {
    let close = |x: f32, y: f32| (x - y).abs() <= 0.001;
    (0..3).all(|k| close(a.position[k], b.position[k]) && close(a.normal[k], b.normal[k]))
        && close(a.fraction, b.fraction)
        && close(a.distance, b.distance)
        && a.body == b.body
}

/// The world gravity's multiplier for a shape's gravity (game units a
/// second squared): `fGravity` (+0x54c, 1 for everyone, `00c6da50`).
pub fn gravity_multiplier(gravity: f32) -> f32 {
    gravity / GRAVITY
}
