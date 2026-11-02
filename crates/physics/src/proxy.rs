//! Havok's character proxy (`hkpCharacterProxy`, Xbox PDB), as compiled
//! into FalloutNV.exe 1.4.0.525 (Havok 7.1.0-r1): the shape phantom that
//! every walking actor is moved by. Each update it casts its shape along
//! the move, keeps a manifold of contact points, turns them into surface
//! constraints, lets Bethesda's listener change them
//! ([`crate::controller`]), and moves as far as the simplex solver
//! ([`crate::simplex`]) lets it.
//!
//! Translated from the PC executable (addresses per function); layouts and
//! names from the Xbox prototype's PDB (ADR-0002). Units: Havok units
//! (game units ÷ [`crate::HAVOK_UNIT`]) and seconds; positions handed in
//! and out are game units. The collision queries the phantom makes are in
//! [`crate::character_cd`] (not traced: Havok's agents).
//!
//! `applySurfaceInteractions` (`00cacf80`, the push on the dynamic bodies
//! the manifold touches) is split: each pass records its touches here
//! ([`SurfaceContact`]) and
//! [`crate::rigid::RigidWorld::apply_surface_interactions`] applies them
//! to the bodies. Not translated: the listener callback it fires for
//! bodies with property 0x1300 (moving platforms); other characters'
//! proxies are phantoms, which it doesn't push.

use crate::character_cd::{Body, Hull, Query, RootCdPoint};
use crate::simplex::{self, SimplexSolverInput, SimplexSolverOutput, SurfaceConstraintInfo};
use crate::vec::*;
use crate::{Collider, Vec3, HAVOK_UNIT};

/// The proxy's settings (`hkpCharacterProxyCinfo`, Xbox PDB), as the game
/// fills them for every actor: Havok's defaults (`00c6cde0`) with the
/// solver's speed limit raised to 100 (`00c6da50`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProxySettings {
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub keep_contact_tolerance: f32,
    pub up: Vec3,
    pub extra_up_static_friction: f32,
    pub extra_down_static_friction: f32,
    pub keep_distance: f32,
    pub contact_angle_sensitivity: f32,
    pub user_planes: u32,
    pub max_character_speed_for_solver: f32,
    pub character_strength: f32,
    pub character_mass: f32,
    /// The cosine of the cinfo's max slope (pi/2, `0x3fc90fdb`):
    /// `hkpCharacterProxy::updateFromCinfo` keeps the cosine.
    pub max_slope_cosine: f32,
    pub penetration_recovery_speed: f32,
    pub max_cast_iterations: u32,
}

impl ProxySettings {
    /// `00c6cde0` and `00c6da50`: dynamic friction 1, static 0, keep
    /// contact tolerance 0.1, up (0, 1, 0) in the cinfo (the controller's
    /// up is z, `UpVec` +0x4c0, which it hands the proxy: here z),
    /// keep distance 0.05, contact angle sensitivity 10, 4 user planes,
    /// solver speed 100 (Bethesda's; Havok's 10), strength FLT_MAX
    /// (`0x7f7fffee`), mass 0, max slope pi/2, penetration recovery speed
    /// 1, 10 cast iterations (the controller's move sets 4 each update,
    /// `00c73170`).
    pub const GAME: ProxySettings = ProxySettings {
        dynamic_friction: 1.0,
        static_friction: 0.0,
        keep_contact_tolerance: 0.1,
        up: [0.0, 0.0, 1.0],
        extra_up_static_friction: 0.0,
        extra_down_static_friction: 0.0,
        keep_distance: 0.05,
        contact_angle_sensitivity: 10.0,
        user_planes: 4,
        max_character_speed_for_solver: 100.0,
        character_strength: 3.402_822e38,
        character_mass: 0.0,
        // cos(1.5707964) as f32.
        max_slope_cosine: -4.371_139e-8,
        penetration_recovery_speed: 1.0,
        max_cast_iterations: 10,
    };
}

/// One manifold point's touch on a body the proxy may push
/// (`applySurfaceInteractions`, `00cacf80`: the manifold entry and the
/// values the function reads from the proxy and the step), recorded after
/// each pass's solve for
/// [`crate::rigid::RigidWorld::apply_surface_interactions`]. Whether the
/// collidable is a dynamic body is for the body's side to say (the
/// function skips fixed and keyframed ones).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceContact {
    /// The placed reference the touched triangle belongs to.
    pub reference: u32,
    /// The contact point on the body's surface (game units, world).
    pub position: Vec3,
    /// The separating normal toward the character, and its distance
    /// (Havok units; negative: penetrating).
    pub normal: Vec3,
    pub distance: f32,
    /// The proxy's velocity (+0x10, Havok units a second).
    pub velocity: Vec3,
    /// The step's time (`stepInfo` +0x8, seconds).
    pub dt: f32,
    /// `m_characterStrength` (+0x6c) and `m_characterMass` (+0x70).
    pub strength: f32,
    pub mass: f32,
}

/// The proxy's state between updates (`hkpCharacterProxy` +0x10
/// `m_velocity`, +0x20 `m_oldDisplacement`, +0x74 `m_manifold`).
#[derive(Debug, Clone, PartialEq)]
pub struct Proxy {
    /// Havok units a second.
    pub velocity: Vec3,
    /// The last move the solver asked for (Havok units): the next update's
    /// first cast goes this far.
    pub old_displacement: Vec3,
    pub manifold: Vec<RootCdPoint>,
    pub settings: ProxySettings,
    /// The touches `applySurfaceInteractions` found since taken, in pass
    /// order (see [`SurfaceContact`]).
    pub surface: Vec<SurfaceContact>,
}

/// What changes the surface constraints once they're built
/// (`hkpCharacterProxyListener::processConstraintsCallback`, fired by
/// `00cad440`): Bethesda's controller ([`crate::controller`]).
pub trait Listener {
    fn process_constraints(
        &mut self,
        manifold: &[RootCdPoint],
        constraints: &mut Vec<SurfaceConstraintInfo>,
    );
}

/// No listener.
pub struct NoListener;

impl Listener for NoListener {
    fn process_constraints(&mut self, _: &[RootCdPoint], _: &mut Vec<SurfaceConstraintInfo>) {}
}

/// How alike two contact points are (`00cafd90`, Xbox PDB
/// `hkpCharacterProxy::surfaceDistance`): the angle between their normals
/// (× the contact angle sensitivity squared, × 10), their bodies' velocity
/// difference squared × 0.1, and their distance difference squared. The
/// bodies here never move ([`point_velocity`]).
pub fn surface_distance(settings: &ProxySettings, a: &RootCdPoint, b: &RootCdPoint) -> f32 {
    let s = settings.contact_angle_sensitivity;
    let va = point_velocity(a);
    let vb = point_velocity(b);
    let dv = sub(va, vb);
    let dd = a.distance - b.distance;
    s * s * (1.0 - dot(a.normal, b.normal)) * 10.0 + dot(dv, dv) * 0.1 + dd * dd
}

/// A contact point's body's velocity there (`00ca0c40` for rigid bodies;
/// phantoms have none). The walking collider's triangles don't move (a
/// door's leaf is moved between updates, not given a velocity).
fn point_velocity(_p: &RootCdPoint) -> Vec3 {
    [0.0; 3]
}

impl Proxy {
    pub fn new(settings: ProxySettings) -> Self {
        Self {
            velocity: [0.0; 3],
            old_displacement: [0.0; 3],
            manifold: Vec::new(),
            settings,
            surface: Vec::new(),
        }
    }

    /// The manifold point most like `p` within 0.1 ([`surface_distance`]),
    /// if any (`00cacf10`, Xbox PDB `hkpCharacterProxy::findSurface`).
    pub fn find_surface(&self, p: &RootCdPoint) -> Option<usize> {
        let mut best = 0.1f32;
        let mut found = None;
        for (i, m) in self.manifold.iter().enumerate() {
            let d = surface_distance(&self.settings, p, m);
            if d < best {
                best = d;
                found = Some(i);
            }
        }
        found
    }

    /// `00caf4d0` (vtable +0x0c, Xbox PDB `hkpCharacterProxy::updateManifold`):
    /// each manifold point is replaced by the start point most like it
    /// (within 1.1) or dropped; of the start points left, those as close as
    /// the closest start point join (or replace the manifold point they
    /// match); the cast's first hit joins unless it matches one; then
    /// points within 0.1 of an earlier one are dropped.
    pub fn update_manifold(&mut self, start: &[RootCdPoint], cast: &[RootCdPoint]) {
        let mut min_distance = 3.40282e38f32;
        let mut left: Vec<RootCdPoint> = start.to_vec();
        for p in &left {
            if p.distance < min_distance {
                min_distance = p.distance;
            }
        }
        let mut i = self.manifold.len();
        while i > 0 {
            i -= 1;
            let m = self.manifold[i];
            let mut best = 1.1f32;
            let mut found: Option<usize> = None;
            for (j, s) in left.iter().enumerate() {
                let d = surface_distance(&self.settings, s, &m);
                if d < best {
                    found = Some(j);
                    best = d;
                }
            }
            match found {
                None => {
                    // Removed: the last point takes its place.
                    self.manifold.swap_remove(i);
                }
                Some(j) => {
                    self.manifold[i] = left[j];
                    left.swap_remove(j);
                }
            }
        }
        for s in &left {
            if s.distance == min_distance {
                match self.find_surface(s) {
                    None => self.manifold.push(*s),
                    Some(k) => self.manifold[k] = *s,
                }
            }
        }
        if let Some(first) = cast.first() {
            if self.find_surface(first).is_none() {
                self.manifold.push(*first);
            }
        }
        let mut i = self.manifold.len().saturating_sub(1);
        while i > 0 {
            for j in (0..i).rev() {
                if surface_distance(&self.settings, &self.manifold[i], &self.manifold[j]) < 0.1 {
                    self.manifold.swap_remove(i);
                    break;
                }
            }
            i -= 1;
        }
    }

    /// A manifold point's surface constraint (`00cacd60`, vtable +0x10,
    /// Xbox PDB `hkpCharacterProxy::extractSurfaceConstraintInfo`): its
    /// plane kept `keep_distance` back, the proxy's frictions, the body's
    /// velocity (none here) and priority (2 for a fixed body, 1 keyframed,
    /// 0 for anything else: the walking collider's triangles are the
    /// world's fixed bodies, other characters are phantoms); a plane
    /// already crossed is put back to zero distance and the crossing turned
    /// into a velocity out of it at `penetration_recovery_speed`.
    /// `time_travelled` is how much of the update has gone.
    pub fn extract_surface_constraint_info(
        &self,
        p: &RootCdPoint,
        time_travelled: f32,
    ) -> SurfaceConstraintInfo {
        let s = &self.settings;
        let mut c = SurfaceConstraintInfo {
            plane: [
                p.normal[0],
                p.normal[1],
                p.normal[2],
                p.distance - s.keep_distance,
            ],
            velocity: [0.0; 4],
            static_friction: s.static_friction,
            extra_up_static_friction: s.extra_up_static_friction,
            extra_down_static_friction: s.extra_down_static_friction,
            dynamic_friction: s.dynamic_friction,
            priority: 0,
        };
        if let Body::Triangle(_) = p.body {
            let v = point_velocity(p);
            c.velocity = [v[0], v[1], v[2], 0.0];
            c.plane[3] -= (c.velocity[2] * c.plane[2]
                + c.velocity[1] * c.plane[1]
                + c.velocity[0] * c.plane[0])
                * time_travelled;
            // Fixed (motion type 5).
            c.priority = 2;
        }
        if c.plane[3] < -1.192_092_9e-7 {
            let push = -c.plane[3] * s.penetration_recovery_speed;
            c.velocity[0] += push * p.normal[0];
            c.velocity[1] += push * p.normal[1];
            c.velocity[2] += push * p.normal[2];
            c.plane[3] = 0.0;
        }
        c
    }

    /// A vertical copy of a constraint steeper than the proxy's max slope
    /// (`00cadca0`): added when the plane faces up (more than 0.01) but
    /// less than the max slope's cosine. With the game's max slope of
    /// pi/2 this never happens (Bethesda's listener does it instead).
    fn add_max_slope_plane(&self, constraints: &mut Vec<SurfaceConstraintInfo>, i: usize) {
        let up = self.settings.up;
        let c = constraints[i];
        let d = c.plane[2] * up[2] + c.plane[1] * up[1] + c.plane[0] * up[0];
        if !(d > 0.01 && self.settings.max_slope_cosine > d) {
            return;
        }
        let mut v = c;
        let nd = -d;
        let x = nd * up[0] + v.plane[0];
        let y = nd * up[1] + v.plane[1];
        let z = nd * up[2] + v.plane[2];
        let l2 = z * z + y * y + x * x;
        let inv = if l2 == 0.0 { 0.0 } else { 1.0 / l2.sqrt() };
        // Havok's up vector has w = 0.
        v.plane = [inv * x, inv * y, inv * z, inv * (nd * 0.0 + v.plane[3])];
        constraints.push(v);
    }

    /// Converts a cast's hits for the manifold (`00cac890`): the fraction
    /// moves to the position's w and the distance becomes how far the
    /// surface was along its normal from the start.
    fn convert_hits(hits: &mut [RootCdPoint], displacement: Vec3) {
        for h in hits.iter_mut().rev() {
            h.fraction = h.distance;
            h.distance *= -(h.normal[2] * displacement[2]
                + h.normal[1] * displacement[1]
                + h.normal[0] * displacement[0]);
        }
    }

    /// `00cacc50` (Xbox PDB `hkpCharacterProxy::moveToLinearCastHitPosition`):
    /// moves `position` toward `to` up to the first hit, kept
    /// `keep_distance` short along its normal; returns the time that took
    /// (the solver's time × the fraction).
    fn move_to_linear_cast_hit_position(
        &self,
        output: &SimplexSolverOutput,
        first: &RootCdPoint,
        to: Vec3,
        position: &mut Vec3,
    ) -> f32 {
        let d = [output.position[0], output.position[1], output.position[2]];
        let len = (d[2] * d[2] + d[1] * d[1] + d[0] * d[0]).sqrt();
        let along =
            -((first.normal[2] * d[2] + first.normal[1] * d[1] + first.normal[0] * d[0]) / len);
        let f = (first.fraction - (self.settings.keep_distance / along) / len).clamp(0.0, 1.0);
        for k in 0..3 {
            position[k] = (1.0 - f) * position[k] + to[k] * f;
        }
        output.delta_time * f
    }

    /// One update of `dt` seconds (`00cade20`, Xbox PDB
    /// `hkpCharacterProxy::integrateImplementation`, through
    /// `hkpCharacterProxy::integrate` `00cafbd0`), moving the shape's
    /// centre `centre` (game units). `max_cast_iterations` is what the
    /// controller sets for the update (4, `00c73170`).
    ///
    /// Each pass, until the time is used up or the passes run out: cast
    /// from where it is by the last solved move, collecting what's within
    /// keep-contact tolerance + keep distance of the start; update the
    /// manifold; build the constraints (and give the listener them); solve;
    /// if the solved move differs from the cast one by more than 0.001,
    /// cast it and stop at the first new surface met; otherwise move all
    /// the way. The solved velocity is kept at the end.
    pub fn integrate(
        &mut self,
        collider: &Collider,
        hull: &Hull,
        centre: &mut Vec3,
        dt: f32,
        listener: &mut dyn Listener,
    ) {
        let s = self.settings;
        let query = Query {
            collider,
            hull,
            origin: *centre,
        };
        // The phantom's position in the query's frame.
        let mut position = [0.0f32; 3];
        let start_tolerance = s.keep_contact_tolerance + s.keep_distance;
        let mut remaining = dt;
        let mut pass = 0u32;
        let mut solved_velocity: Option<Vec3> = None;
        while remaining > 1.192_092_9e-7 && pass < s.max_cast_iterations {
            // StInitialCast.
            let start = query.start_points(position, start_tolerance);
            let start: Vec<RootCdPoint> = start.into_iter().map(|p| to_game(&query, p)).collect();
            let mut hits =
                query.linear_cast(position, self.old_displacement, 0.01, start_tolerance);
            if !hits.is_empty() {
                sort_hits(&mut hits);
                Self::convert_hits(&mut hits, self.old_displacement);
            }
            let hits: Vec<RootCdPoint> = hits.into_iter().map(|p| to_game(&query, p)).collect();
            // StUpdateManifold.
            self.update_manifold(&start, &hits);
            let mut constraints: Vec<SurfaceConstraintInfo> =
                Vec::with_capacity(self.manifold.len() + 10 + s.user_planes as usize);
            for i in 0..self.manifold.len() {
                let c = self.extract_surface_constraint_info(&self.manifold[i], dt - remaining);
                constraints.push(c);
                let last = constraints.len() - 1;
                self.add_max_slope_plane(&mut constraints, last);
            }
            // StSlexMove.
            let v2 = self.velocity[2] * self.velocity[2]
                + self.velocity[1] * self.velocity[1]
                + self.velocity[0] * self.velocity[0];
            let min_delta_time = if v2 != 0.0 {
                (1.0 / v2.sqrt()) * s.keep_distance * 0.5
            } else {
                0.0
            };
            listener.process_constraints(&self.manifold, &mut constraints);
            let m = s.max_character_speed_for_solver;
            let output = simplex::solve(&SimplexSolverInput {
                position: [0.0; 4],
                velocity: [self.velocity[0], self.velocity[1], self.velocity[2], 0.0],
                max_surface_velocity: [m, m, m, m],
                up_vector: [s.up[0], s.up[1], s.up[2], 0.0],
                delta_time: remaining,
                min_delta_time,
                constraints: &constraints,
            });
            // StApplySurf (`00cacf80`): the manifold's touches on bodies,
            // before the cast move adds its hit.
            for p in &self.manifold {
                if let Body::Triangle(t) = p.body {
                    let reference = collider.owner(t);
                    if reference != 0 {
                        self.surface.push(SurfaceContact {
                            reference,
                            position: p.position,
                            normal: p.normal,
                            distance: p.distance,
                            velocity: self.velocity,
                            dt,
                            strength: s.character_strength,
                            mass: s.character_mass,
                        });
                    }
                }
            }
            // StCastMove.
            let solved = [output.position[0], output.position[1], output.position[2]];
            let same = (0..3).all(|k| (self.old_displacement[k] - solved[k]).abs() <= 0.001);
            let mut used = None;
            if !same {
                let to = add(position, solved);
                let mut hits = query.linear_cast(position, solved, 0.01, start_tolerance);
                if !hits.is_empty() {
                    sort_hits(&mut hits);
                    Self::convert_hits(&mut hits, solved);
                    let mut hits: Vec<RootCdPoint> =
                        hits.into_iter().map(|p| to_game(&query, p)).collect();
                    // Hits on surfaces already in the manifold don't stop
                    // the move.
                    while !hits.is_empty() && self.find_surface(&hits[0]).is_some() {
                        hits.remove(0);
                    }
                    if let Some(&first) = hits.first() {
                        self.manifold.push(first);
                        used = Some(self.move_to_linear_cast_hit_position(
                            &output,
                            &first,
                            to,
                            &mut position,
                        ));
                    }
                }
            }
            match used {
                Some(t) => remaining -= t,
                None => {
                    remaining -= output.delta_time;
                    position = add(position, solved);
                }
            }
            self.old_displacement = solved;
            solved_velocity = Some([output.velocity[0], output.velocity[1], output.velocity[2]]);
            pass += 1;
        }
        if let Some(v) = solved_velocity {
            self.velocity = v;
        }
        *centre = add(*centre, scale(position, HAVOK_UNIT));
    }
}

/// Sorts a cast's hits nearest first (`00cabad0` → `009b2ed0`, by the
/// fraction held in the distance).
fn sort_hits(hits: &mut [RootCdPoint]) {
    hits.sort_by(|a, b| a.distance.total_cmp(&b.distance));
}

/// A point from the query's frame: the position in game units.
fn to_game(q: &Query, mut p: RootCdPoint) -> RootCdPoint {
    p.position = add(q.origin, scale(p.position, HAVOK_UNIT));
    p
}
