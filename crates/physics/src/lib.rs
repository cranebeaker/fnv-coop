//! Collision and walking, in game units (z up).
//!
//! [`Collider`] holds a cell's solid surfaces as one triangle set, bucketed
//! on a grid for quick lookups; [`shapes`] turns the models' Havok shapes
//! (hulls, spheres, capsules) into triangles for it. [`Character`] is the
//! game's character controller ([`controller`]) around Havok's character
//! proxy ([`proxy`]): it falls, lands, slides along walls, rides up steps
//! and stops at slopes steeper than 47 degrees.
//!
//! ```
//! use physics::{Character, CharacterShape, Collider};
//!
//! // A floor: two triangles at z = 0.
//! let mut floor = Collider::new();
//! floor.add(&[[-500.0, -500.0, 0.0], [500.0, -500.0, 0.0], [500.0, 500.0, 0.0], [-500.0, 500.0, 0.0]],
//!           &[[0, 1, 2], [0, 2, 3]]);
//! let shape = CharacterShape::PLAYER;
//! let mut player = Character::new([0.0, 0.0, 50.0]);
//! for _ in 0..120 {
//!     player.update(&floor, &shape, [0.0, 0.0], 1.0 / 60.0);
//! }
//! assert!(player.on_ground && player.feet[2].abs() < 1.0);
//! ```

pub mod character_cd;
pub mod constraint;
pub mod contacts;
pub mod continuous;
pub mod controller;
pub mod grab;
pub mod havok;
pub mod impulses;
pub mod islands;
pub mod layers;
pub mod manifold;
pub mod proxy;
pub mod ragdoll;
pub mod rigid;
pub mod shapes;
pub mod simplex;
pub mod solver;
mod vec;
pub mod view_caster;
pub mod wind;

use std::collections::{HashMap, HashSet};

use vec::*;

pub type Vec3 = [f32; 3];

/// Size of the grid's buckets, in game units.
const BUCKET: f32 = 128.0;

/// Solid surfaces as triangles, for collision and ray casts.
///
/// Each triangle can stand out from its plane by a shell (Havok's convex
/// radius: every shape in the game's models has 0.1 Havok units, 0.7 game
/// units, around it) and can belong to an object whose collision can be
/// switched off and on (a door that opens). Other characters, which move,
/// are kept apart from the triangles ([`Collider::set_people`]).
#[derive(Debug, Clone, Default)]
pub struct Collider {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
    /// Bucket (x, y) to the triangles overlapping it.
    grid: HashMap<(i32, i32), Vec<u32>>,
    /// What changes while walking (kept aside, so a collider stays small).
    live: Box<Live>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle {
    corners: [u32; 3],
    /// How far its surface stands out (game units).
    shell: f32,
    /// The object it belongs to, 0 for none.
    owner: u32,
    /// The Havok material of the shape it came from ([NO_MATERIAL] when
    /// none was given): what a shot striking it sounds and looks like.
    material: u32,
    /// Its rigid body's friction and restitution: 1 + an index into
    /// `Live::surfaces`, 0 for none given.
    surface: u16,
    /// Its body's Havok layer ([`ANY_LAYER`]: not given), which decides
    /// what casts meet it ([`Collider::raycast_layer`]).
    layer: u8,
    /// The placed reference it comes from, 0 for none
    /// ([`Collider::add_placed`]).
    reference: u32,
}

/// A triangle added without a Havok layer: every cast meets it.
pub const ANY_LAYER: u8 = u8::MAX;

/// How a surface rubs and bounces: its rigid body's friction and
/// restitution (`nif::RigidBodyInfo`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Surface {
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Debug, Clone, Default)]
struct Live {
    /// The surfaces triangles were added with (see `Triangle::surface`).
    surfaces: Vec<Surface>,
    /// Objects whose triangles are switched off.
    hidden: HashSet<u32>,
    /// Other characters, as upright cylinders.
    people: Vec<Person>,
    /// Owned triangles' vertices where they were added (an object that
    /// moves, such as a door's leaf, is put back from these), and the
    /// owned triangles themselves, by owner.
    rest: HashMap<u32, Vec<(u32, Vec3)>>,
    owned: HashMap<u32, Vec<u32>>,
    /// The triangles of each placed reference ([`Collider::add_placed`]).
    placed: HashMap<u32, Vec<u32>>,
}

/// Another character a walking one runs into: the game's character
/// controllers collide with each other (layer 30 with itself), and the
/// contact callback (`00c711d0`) flattens each such contact to a vertical
/// wall (its normal's z set to 0), so nobody stands on anybody.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Person {
    pub feet: Vec3,
    pub radius: f32,
    pub height: f32,
}

/// A triangle added without a Havok material.
pub const NO_MATERIAL: u32 = u32::MAX;

fn bucket(v: f32) -> i32 {
    (v / BUCKET).floor() as i32
}

/// [`bucket_keys`] without making a list: each key in the same order.
fn for_bucket(a: Vec3, b: Vec3, c: Vec3, mut f: impl FnMut((i32, i32))) {
    let (x0, x1) = (a[0].min(b[0]).min(c[0]), a[0].max(b[0]).max(c[0]));
    let (y0, y1) = (a[1].min(b[1]).min(c[1]), a[1].max(b[1]).max(c[1]));
    for bx in bucket(x0)..=bucket(x1) {
        for by in bucket(y0)..=bucket(y1) {
            f((bx, by));
        }
    }
}

/// The buckets a triangle's x-y box overlaps.
fn bucket_keys(a: Vec3, b: Vec3, c: Vec3) -> Vec<(i32, i32)> {
    let (x0, x1) = (a[0].min(b[0]).min(c[0]), a[0].max(b[0]).max(c[0]));
    let (y0, y1) = (a[1].min(b[1]).min(c[1]), a[1].max(b[1]).max(c[1]));
    let mut keys = Vec::new();
    for bx in bucket(x0)..=bucket(x1) {
        for by in bucket(y0)..=bucket(y1) {
            keys.push((bx, by));
        }
    }
    keys
}

impl Collider {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds triangles (indices into `vertices`); out-of-range ones and ones
    /// with a corner that isn't a finite number are skipped.
    pub fn add(&mut self, vertices: &[Vec3], triangles: &[[u32; 3]]) {
        self.add_solid(vertices, triangles, 0.0, 0);
    }

    /// [`Collider::add`] with a shell around each triangle (game units) and
    /// the object they belong to (0 for none; see
    /// [`Collider::set_hidden`]).
    pub fn add_solid(&mut self, vertices: &[Vec3], triangles: &[[u32; 3]], shell: f32, owner: u32) {
        self.add_solid_material(vertices, triangles, shell, owner, NO_MATERIAL);
    }

    /// [`Self::add`], the triangles being of a Havok material (what a shot
    /// striking them looks and sounds like).
    pub fn add_with_material(&mut self, vertices: &[Vec3], triangles: &[[u32; 3]], material: u32) {
        self.add_solid_material(vertices, triangles, 0.0, 0, material);
    }

    /// [`Self::add_solid`] with the triangles' Havok material.
    pub fn add_solid_material(
        &mut self,
        vertices: &[Vec3],
        triangles: &[[u32; 3]],
        shell: f32,
        owner: u32,
        material: u32,
    ) {
        self.add_solid_surface(vertices, triangles, (shell, owner, material), None);
    }

    /// [`Self::add_solid_material`] (shell, owner and material) with the
    /// friction and restitution of the body they belong to, which rigid
    /// bodies resting on them rub and bounce against
    /// ([`crate::rigid`]).
    pub fn add_solid_surface(
        &mut self,
        vertices: &[Vec3],
        triangles: &[[u32; 3]],
        parts: (f32, u32, u32),
        surface: Option<Surface>,
    ) {
        self.add_layered(vertices, triangles, parts, surface, ANY_LAYER);
    }

    /// [`Self::add_solid_surface`] with the Havok layer of the body the
    /// triangles belong to (`nif::CollisionPart::layer`). An owner is also
    /// the placed reference they come from ([`Self::add_placed`]).
    pub fn add_layered(
        &mut self,
        vertices: &[Vec3],
        triangles: &[[u32; 3]],
        parts: (f32, u32, u32),
        surface: Option<Surface>,
        layer: u8,
    ) {
        self.add_placed(vertices, triangles, parts, surface, layer, parts.1);
    }

    /// [`Self::add_layered`] for the collision of a placed reference
    /// (`reference`, 0 for none): what the crosshair's pick
    /// ([`view_caster`]) takes a triangle for, whether or not the triangles
    /// have an owner that moves or switches them off.
    pub fn add_placed(
        &mut self,
        vertices: &[Vec3],
        triangles: &[[u32; 3]],
        (shell, owner, material): (f32, u32, u32),
        surface: Option<Surface>,
        layer: u8,
        reference: u32,
    ) {
        let surface = surface.map_or(0, |s| self.surface_index(s));
        let base = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        if owner != 0 {
            let rest = self.live.rest.entry(owner).or_default();
            rest.extend(
                vertices
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (base + i as u32, v)),
            );
        }
        for t in triangles {
            if t.iter().any(|&i| i as usize >= vertices.len()) {
                continue;
            }
            self.push_triangle(
                [t[0] + base, t[1] + base, t[2] + base],
                shell,
                owner,
                (material, surface, layer, reference),
            );
        }
    }

    /// 1 + the index of a surface in the table (added if new; the table is
    /// small: a few values per place). 0 once it's full.
    fn surface_index(&mut self, s: Surface) -> u16 {
        let table = &mut self.live.surfaces;
        if let Some(i) = table.iter().position(|&t| t == s) {
            return i as u16 + 1;
        }
        if table.len() >= usize::from(u16::MAX - 1) {
            return 0;
        }
        table.push(s);
        table.len() as u16
    }

    /// The friction and restitution a triangle was added with, if any.
    pub fn surface(&self, index: u32) -> Option<Surface> {
        let s = self.triangles.get(index as usize)?.surface;
        (s > 0)
            .then(|| self.live.surfaces.get(usize::from(s) - 1).copied())
            .flatten()
    }

    /// Moves an object's triangles (`owner`, see [`Collider::add_solid`])
    /// to where a rigid move puts their added positions: `v = rotation ×
    /// rest + translation` (a 3 × 3 row-major rotation). A door's leaf
    /// follows its animation this way.
    pub fn move_owner(&mut self, owner: u32, rotation: &[[f32; 3]; 3], translation: Vec3) {
        if owner == 0 {
            return;
        }
        let Some(rest) = self.live.rest.get(&owner) else {
            return;
        };
        let Some(owned) = self.live.owned.get(&owner) else {
            return;
        };
        // Out of their old buckets first: each bucket they're in looked
        // through once (a triangle is only in its own buckets, so the lists
        // come out as taking them out one at a time leaves them).
        let owned: Vec<u32> = owned.clone();
        let lo = owned.iter().copied().min().unwrap_or(0);
        let hi = owned.iter().copied().max().unwrap_or(0);
        // An owner's triangles are added together: when they're one run
        // of numbers, being in the run is being one of them.
        let run = (hi - lo) as usize + 1 == owned.len();
        let leaving: HashSet<u32> = if run {
            HashSet::new()
        } else {
            owned.iter().copied().collect()
        };
        let mut buckets: Vec<(i32, i32)> = Vec::new();
        for &t in &owned {
            let [a, b, c] = self.triangle(t);
            for_bucket(a, b, c, |key| {
                if !buckets.contains(&key) {
                    buckets.push(key);
                }
            });
        }
        for key in &buckets {
            if let Some(list) = self.grid.get_mut(key) {
                list.retain(|&i| i < lo || i > hi || (!run && !leaving.contains(&i)));
            }
        }
        for &(i, p) in rest {
            let r = [
                rotation[0][0] * p[0] + rotation[0][1] * p[1] + rotation[0][2] * p[2],
                rotation[1][0] * p[0] + rotation[1][1] * p[1] + rotation[1][2] * p[2],
                rotation[2][0] * p[0] + rotation[2][1] * p[1] + rotation[2][2] * p[2],
            ];
            self.vertices[i as usize] = add(r, translation);
        }
        // Into their new ones: each bucket's in the owner's order, as
        // putting them in one at a time does.
        let mut arriving: Vec<((i32, i32), Vec<u32>)> = Vec::new();
        for &t in &owned {
            let [a, b, c] = self.triangle(t);
            for_bucket(a, b, c, |key| {
                match arriving.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, list)) => list.push(t),
                    None => arriving.push((key, vec![t])),
                }
            });
        }
        for (key, list) in arriving {
            self.grid.entry(key).or_default().extend(list);
        }
    }

    /// One triangle over vertices already added. Triangles without an area
    /// (their corners in a line: the game's meshes have a few) have no
    /// surface to touch, and the closest-point tests divide by their area,
    /// so they're left out.
    fn push_triangle(
        &mut self,
        tri: [u32; 3],
        shell: f32,
        owner: u32,
        (material, surface, layer, reference): (u32, u16, u8, u32),
    ) {
        let [a, b, c] = tri.map(|i| self.vertices[i as usize]);
        if [a, b, c].iter().flatten().any(|v| !v.is_finite()) {
            return;
        }
        let area = length(cross(sub(b, a), sub(c, a)));
        // Also leaves out a NaN area.
        if area.is_nan() || area <= 1e-6 {
            return;
        }
        let index = self.triangles.len() as u32;
        self.triangles.push(Triangle {
            corners: tri,
            shell,
            owner,
            material,
            surface,
            layer,
            reference,
        });
        if owner != 0 {
            self.live.owned.entry(owner).or_default().push(index);
        }
        if reference != 0 {
            self.live.placed.entry(reference).or_default().push(index);
        }
        for key in bucket_keys(a, b, c) {
            self.grid.entry(key).or_default().push(index);
        }
    }

    /// Adds every triangle of another collider (the squares of an outdoor
    /// area, gathered into one), with its shells, owners and switched-off
    /// objects.
    ///
    /// Its triangles were checked as they went into it and its buckets are
    /// those of its triangles where they are now, so they're taken over as
    /// they are, renumbered (each bucket and owner's list in triangle
    /// order, as pushing them in one at a time puts them), rather than
    /// measured again one by one: outdoors this is redone for every loaded
    /// square whenever one comes or goes.
    pub fn extend(&mut self, other: &Collider) {
        let base = self.vertices.len() as u32;
        let first = self.triangles.len() as u32;
        self.vertices.extend_from_slice(&other.vertices);
        let surfaces: Vec<u16> = other
            .live
            .surfaces
            .iter()
            .map(|&s| self.surface_index(s))
            .collect();
        self.triangles
            .extend(other.triangles.iter().map(|tri| Triangle {
                corners: tri.corners.map(|i| i + base),
                surface: match tri.surface {
                    0 => 0,
                    s => surfaces.get(usize::from(s) - 1).copied().unwrap_or(0),
                },
                ..*tri
            }));
        let renumbered = |list: &[u32]| {
            let mut list: Vec<u32> = list.iter().map(|&t| t + first).collect();
            list.sort_unstable();
            list
        };
        for (&key, list) in &other.grid {
            if !list.is_empty() {
                self.grid.entry(key).or_default().extend(renumbered(list));
            }
        }
        for (&owner, list) in &other.live.owned {
            if !list.is_empty() {
                self.live
                    .owned
                    .entry(owner)
                    .or_default()
                    .extend(renumbered(list));
            }
        }
        // The placed objects' lists (the crosshair's view caster) the same
        // way.
        for (&reference, list) in &other.live.placed {
            if !list.is_empty() {
                self.live
                    .placed
                    .entry(reference)
                    .or_default()
                    .extend(renumbered(list));
            }
        }
        self.live.hidden.extend(other.live.hidden.iter().copied());
        for (&owner, rest) in &other.live.rest {
            self.live
                .rest
                .entry(owner)
                .or_default()
                .extend(rest.iter().map(|&(i, v)| (i + base, v)));
        }
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// The Havok material a triangle was added with, if any.
    pub fn material(&self, index: u32) -> Option<u32> {
        self.triangles
            .get(index as usize)
            .map(|t| t.material)
            .filter(|&m| m != NO_MATERIAL)
    }

    /// The corners of a triangle.
    pub fn triangle(&self, index: u32) -> [Vec3; 3] {
        self.triangles[index as usize]
            .corners
            .map(|i| self.vertices[i as usize])
    }

    /// How far a triangle's surface stands out from it.
    pub fn shell(&self, index: u32) -> f32 {
        self.triangles[index as usize].shell
    }

    /// The object a triangle belongs to (0 for none).
    pub fn owner(&self, index: u32) -> u32 {
        self.triangles[index as usize].owner
    }

    fn switched_off(&self, index: u32) -> bool {
        !self.live.hidden.is_empty() && self.live.hidden.contains(&self.owner(index))
    }

    /// Switches an object's triangles off (`true`: an open door) or back on.
    pub fn set_hidden(&mut self, owner: u32, hidden: bool) {
        if hidden && owner != 0 {
            self.live.hidden.insert(owner);
        } else {
            self.live.hidden.remove(&owner);
        }
    }

    pub fn is_hidden(&self, owner: u32) -> bool {
        self.live.hidden.contains(&owner)
    }

    /// Whether any triangle belongs to this object.
    pub fn owns(&self, owner: u32) -> bool {
        owner != 0 && self.live.owned.get(&owner).is_some_and(|t| !t.is_empty())
    }

    /// The other characters walkers run into, replacing the last list.
    pub fn set_people(&mut self, people: Vec<Person>) {
        self.live.people = people;
    }

    pub fn people(&self) -> &[Person] {
        &self.live.people
    }

    /// Triangles whose buckets overlap the box from `lo` to `hi` (x and y;
    /// z isn't bucketed), each once; switched-off objects' are left out.
    pub fn near(&self, lo: Vec3, hi: Vec3) -> Vec<u32> {
        let mut out = Vec::new();
        for bx in bucket(lo[0])..=bucket(hi[0]) {
            for by in bucket(lo[1])..=bucket(hi[1]) {
                if let Some(list) = self.grid.get(&(bx, by)) {
                    out.extend_from_slice(list);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        if !self.live.hidden.is_empty() {
            out.retain(|&t| !self.switched_off(t));
        }
        out
    }

    /// The nearest surface along a ray from `origin` in `direction` (unit
    /// length), within `max` units: its distance and triangle.
    pub fn raycast(&self, origin: Vec3, direction: Vec3, max: f32) -> Option<(f32, u32)> {
        self.cast(origin, direction, max, false, None)
    }

    /// [`Collider::raycast`] for a cast on Havok layer `layer`: triangles
    /// whose body's layer that layer doesn't touch (the game's collision
    /// filter, [`layers::Filter::layers_touch`] with the cast first, as the
    /// ray filter `00c84930` asks it) are passed through. A shot (layer 6,
    /// `PROJECTILE`) goes through a `TRANSPARENT` (3) chain-link fence that
    /// stops a walker.
    pub fn raycast_layer(
        &self,
        origin: Vec3,
        direction: Vec3,
        max: f32,
        layer: u8,
    ) -> Option<(f32, u32)> {
        self.cast(origin, direction, max, false, Some(layer))
    }

    /// The Havok layer a triangle's body is on ([`ANY_LAYER`]: not given).
    pub fn layer(&self, index: u32) -> u8 {
        self.triangles[index as usize].layer
    }

    /// [`Collider::raycast`], also meeting switched-off objects (an open
    /// door, to close it again).
    pub fn raycast_including_hidden(
        &self,
        origin: Vec3,
        direction: Vec3,
        max: f32,
    ) -> Option<(f32, u32)> {
        self.cast(origin, direction, max, true, None)
    }

    fn cast(
        &self,
        origin: Vec3,
        direction: Vec3,
        max: f32,
        hidden: bool,
        layer: Option<u8>,
    ) -> Option<(f32, u32)> {
        let filter = layer.map(|l| (l, layers::Filter::shared()));
        let passes = |t: u32| {
            filter.is_some_and(|(l, f)| {
                let tl = self.triangles[t as usize].layer;
                tl != ANY_LAYER && !f.layers_touch(l, tl)
            })
        };
        // Walk the buckets the ray crosses, a step at a time.
        let steps = (max / (BUCKET * 0.5)).ceil().max(1.0) as usize;
        let mut best: Option<(f32, u32)> = None;
        let mut seen = std::collections::HashSet::new();
        for s in 0..=steps {
            let along = (s as f32 * BUCKET * 0.5).min(max);
            let p = add(origin, scale(direction, along));
            if best.is_some_and(|(t, _)| t < along - BUCKET) {
                break;
            }
            for bx in bucket(p[0]) - 1..=bucket(p[0]) + 1 {
                for by in bucket(p[1]) - 1..=bucket(p[1]) + 1 {
                    let Some(list) = self.grid.get(&(bx, by)) else {
                        continue;
                    };
                    for &t in list {
                        if !seen.insert(t) || (!hidden && self.switched_off(t)) || passes(t) {
                            continue;
                        }
                        let [a, b, c] = self.triangle(t);
                        if let Some(d) = ray_triangle(origin, direction, a, b, c) {
                            if d <= max && best.map_or(true, |(bd, _)| d < bd) {
                                best = Some((d, t));
                            }
                        }
                    }
                }
            }
        }
        best
    }

    /// A sphere of `radius` whose centre moves from `origin` along
    /// `direction` (unit length) for up to `max` units: where it first
    /// touches a surface, with the point it touches. Each triangle's shell
    /// counts as part of it, and switched-off objects (open doors) are
    /// passed through, as for [`Collider::raycast`]. A sphere already
    /// touching something gives distance 0 and the nearest point of what
    /// it touches. (The game's cameras are kept out of walls with such a
    /// cast, `fCameraCasterSize` wide: `0094a0c0`.)
    pub fn spherecast(
        &self,
        origin: Vec3,
        direction: Vec3,
        max: f32,
        radius: f32,
    ) -> Option<SphereHit> {
        let steps = (max / (BUCKET * 0.5)).ceil().max(1.0) as usize;
        let mut best: Option<SphereHit> = None;
        let mut seen = std::collections::HashSet::new();
        for s in 0..=steps {
            let along = (s as f32 * BUCKET * 0.5).min(max);
            let p = add(origin, scale(direction, along));
            if best.is_some_and(|h| h.distance < along - BUCKET) {
                break;
            }
            for bx in bucket(p[0]) - 1..=bucket(p[0]) + 1 {
                for by in bucket(p[1]) - 1..=bucket(p[1]) + 1 {
                    let Some(list) = self.grid.get(&(bx, by)) else {
                        continue;
                    };
                    for &t in list {
                        if !seen.insert(t) || self.switched_off(t) {
                            continue;
                        }
                        let [a, b, c] = self.triangle(t);
                        let shell = self.shell(t);
                        let r = radius + shell;
                        if let Some((d, on_triangle)) =
                            sphere_sweep_triangle(origin, direction, max, r, a, b, c)
                        {
                            if best.map_or(true, |h| d < h.distance) {
                                // The touching point is on the shell's
                                // surface, toward the sphere's centre.
                                let centre = add(origin, scale(direction, d));
                                let point = add(
                                    on_triangle,
                                    scale(normalize(sub(centre, on_triangle)), shell),
                                );
                                best = Some(SphereHit {
                                    distance: d,
                                    point,
                                    triangle: t,
                                });
                            }
                        }
                    }
                }
            }
        }
        best
    }

    /// The placed reference a triangle comes from (0 for none).
    pub fn reference(&self, index: u32) -> u32 {
        self.triangles[index as usize].reference
    }

    /// Every triangle a sphere of `radius` cast from `origin` along
    /// `direction` (unit length) for up to `max` units touches, as an
    /// all-hits collector gathers a Havok linear cast's: each with where it
    /// is touched, nearest first. Only triangles whose body's layer the
    /// cast's `layer` touches ([`layers::Filter::layers_touch`], the cast
    /// first; those added without a layer always count), not switched off.
    pub fn spherecast_all(
        &self,
        origin: Vec3,
        direction: Vec3,
        max: f32,
        radius: f32,
        layer: u8,
    ) -> Vec<SphereHit> {
        let filter = layers::Filter::shared();
        let lo = [0, 1, 2].map(|i| origin[i].min(origin[i] + direction[i] * max) - radius);
        let hi = [0, 1, 2].map(|i| origin[i].max(origin[i] + direction[i] * max) + radius);
        let mut out = Vec::new();
        for t in self.near(lo, hi) {
            let tl = self.layer(t);
            if tl != ANY_LAYER && !filter.layers_touch(layer, tl) {
                continue;
            }
            let [a, b, c] = self.triangle(t);
            let shell = self.shell(t);
            if let Some((d, on_triangle)) =
                sphere_sweep_triangle(origin, direction, max, radius + shell, a, b, c)
            {
                let centre = add(origin, scale(direction, d));
                let point = add(
                    on_triangle,
                    scale(normalize(sub(centre, on_triangle)), shell),
                );
                out.push(SphereHit {
                    distance: d,
                    point,
                    triangle: t,
                });
            }
        }
        out.sort_by(|a, b| a.distance.total_cmp(&b.distance));
        out
    }

    /// Where a ray from `origin` along `direction` (unit length) first meets
    /// one placed reference's triangles ([`Self::add_placed`]), however far,
    /// passing everything else; `None` when it misses them or they're
    /// switched off.
    pub fn raycast_reference(&self, origin: Vec3, direction: Vec3, reference: u32) -> Option<f32> {
        let tris = self.live.placed.get(&reference)?;
        tris.iter()
            .filter(|&&t| !self.switched_off(t))
            .filter_map(|&t| {
                let [a, b, c] = self.triangle(t);
                ray_triangle(origin, direction, a, b, c)
            })
            .min_by(f32::total_cmp)
    }
}

/// Where a ray from `o` along `d` (unit length) first meets a capsule (the
/// segment `a b` grown by `r`); 0 when it starts inside.
pub fn ray_capsule(o: Vec3, d: Vec3, a: Vec3, b: Vec3, r: f32) -> Option<f32> {
    let inside = {
        let p = closest_on_segment(o, a, b);
        dist2(o, p) <= r * r
    };
    if inside {
        return Some(0.0);
    }
    let mut best: Option<f32> = None;
    let mut consider = |t: f32| {
        if t >= 0.0 && best.map_or(true, |b| t < b) {
            best = Some(t);
        }
    };
    if let Some((t, _)) = sphere_sweep_segment(o, d, r, a, b) {
        consider(t);
    }
    for c in [a, b] {
        if let Some(t) = ray_sphere(o, d, c, r) {
            consider(t);
        }
    }
    best
}

/// A sphere of radius `r` moving from `o` along `d` (unit length) up to
/// `max`: when it first touches a capsule (segment `a b`, radius `cr`) and
/// the point of the capsule's surface it touches; already touching, 0 and
/// the capsule's surface point nearest the start.
pub fn sphere_sweep_capsule(
    o: Vec3,
    d: Vec3,
    max: f32,
    r: f32,
    (a, b, cr): (Vec3, Vec3, f32),
) -> Option<(f32, Vec3)> {
    let t = ray_capsule(o, d, a, b, r + cr)?;
    if t > max {
        return None;
    }
    let centre = add(o, scale(d, t));
    let axis = closest_on_segment(centre, a, b);
    let toward = sub(centre, axis);
    let n = if dot(toward, toward) > 1e-12 {
        normalize(toward)
    } else {
        scale(d, -1.0)
    };
    Some((t, add(axis, scale(n, cr))))
}

/// The point of segment `a b` nearest `p`.
fn closest_on_segment(p: Vec3, a: Vec3, b: Vec3) -> Vec3 {
    let e = sub(b, a);
    let ee = dot(e, e);
    if ee < 1e-12 {
        return a;
    }
    let u = (dot(sub(p, a), e) / ee).clamp(0.0, 1.0);
    add(a, scale(e, u))
}

/// Where a swept sphere first touches something ([`Collider::spherecast`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphereHit {
    /// How far the sphere's centre travelled from the origin.
    pub distance: f32,
    /// The point on the surface it touches.
    pub point: Vec3,
    pub triangle: u32,
}

/// A sphere of radius `r` moving from `o` along `d` (unit length) up to
/// `max`: the distance at which it first touches triangle `a b c` and the
/// point of the triangle it touches, if it does. Already touching: 0 and
/// the nearest point.
fn sphere_sweep_triangle(
    o: Vec3,
    d: Vec3,
    max: f32,
    r: f32,
    a: Vec3,
    b: Vec3,
    c: Vec3,
) -> Option<(f32, Vec3)> {
    let nearest = closest_on_triangle(o, a, b, c);
    if dist2(o, nearest) <= r * r {
        return Some((0.0, nearest));
    }
    let mut best: Option<(f32, Vec3)> = None;
    let mut consider = |t: f32, p: Vec3| {
        if t >= 0.0 && t <= max && best.map_or(true, |(bt, _)| t < bt) {
            best = Some((t, p));
        }
    };
    // The face: the sphere's near side reaches the plane while approaching
    // it, at a point inside the triangle. (Overlapping the plane already
    // and sliding in from outside the triangle is an edge or corner
    // touch, below.)
    let n = normalize(cross(sub(b, a), sub(c, a)));
    let s0 = dot(n, sub(o, a));
    let nd = dot(n, d);
    if nd.abs() > 1e-9 && s0 * nd < 0.0 {
        let side = if s0 >= 0.0 { 1.0 } else { -1.0 };
        let t = (side * r - s0) / nd;
        if t >= 0.0 {
            let centre = add(o, scale(d, t));
            let p = sub(centre, scale(n, side * r));
            if point_in_triangle(p, a, b, c, n) {
                consider(t, p);
            }
        }
    }
    for (p, q) in [(a, b), (b, c), (c, a)] {
        if let Some((t, point)) = sphere_sweep_segment(o, d, r, p, q) {
            consider(t, point);
        }
    }
    for v in [a, b, c] {
        if let Some(t) = ray_sphere(o, d, v, r) {
            consider(t, v);
        }
    }
    best
}

/// Whether `p`, on the plane of triangle `a b c` with normal `n`, lies
/// inside it (edges included).
fn point_in_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3, n: Vec3) -> bool {
    dot(cross(sub(b, a), sub(p, a)), n) >= -1e-6
        && dot(cross(sub(c, b), sub(p, b)), n) >= -1e-6
        && dot(cross(sub(a, c), sub(p, c)), n) >= -1e-6
}

/// A sphere of radius `r` moving from `o` along `d`: when it first touches
/// the segment `p q` (its side, not its ends), and the point on the segment
/// it touches.
fn sphere_sweep_segment(o: Vec3, d: Vec3, r: f32, p: Vec3, q: Vec3) -> Option<(f32, Vec3)> {
    let e = sub(q, p);
    let ee = dot(e, e);
    if ee < 1e-12 {
        return None;
    }
    let m = sub(o, p);
    let me = dot(m, e);
    let de = dot(d, e);
    // The parts of the offset and the motion across the segment's line.
    let w = sub(m, scale(e, me / ee));
    let v = sub(d, scale(e, de / ee));
    let qa = dot(v, v);
    if qa < 1e-12 {
        return None;
    }
    let qb = 2.0 * dot(v, w);
    let qc = dot(w, w) - r * r;
    let disc = qb * qb - 4.0 * qa * qc;
    if disc < 0.0 {
        return None;
    }
    let t = (-qb - disc.sqrt()) / (2.0 * qa);
    if t < 0.0 {
        return None;
    }
    let u = (me + de * t) / ee;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    Some((t, add(p, scale(e, u))))
}

/// When a ray from `o` along `d` (unit length) first reaches the surface of
/// a sphere of radius `r` at `centre`.
fn ray_sphere(o: Vec3, d: Vec3, centre: Vec3, r: f32) -> Option<f32> {
    let m = sub(o, centre);
    let b = dot(m, d);
    let c = dot(m, m) - r * r;
    let disc = b * b - c;
    if disc < 0.0 {
        return None;
    }
    let t = -b - disc.sqrt();
    (t >= 0.0).then_some(t)
}

/// Möller–Trumbore, both faces: the distance along the ray, if it hits.
fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<f32> {
    let e1 = sub(b, a);
    let e2 = sub(c, a);
    let p = cross(dir, e2);
    let det = dot(e1, p);
    if det.abs() < 1e-9 {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(origin, a);
    let u = dot(s, p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross(s, e1);
    let v = dot(dir, q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = dot(e2, q) * inv;
    (t >= 0.0).then_some(t)
}

/// Game units per Havok unit (the exe's 69.99125671 game units a metre over
/// 10).
pub const HAVOK_UNIT: f32 = 6.999_125_7;

/// How far the ground's surface stands out from the terrain's triangles: the
/// game builds each square's terrain as a sampled height field wrapped in a
/// triangle collection with a radius of 0.5 Havok units (`00cb0820`, the
/// float at `01016248`), 3.5 game units; models' shapes have 0.1.
pub const TERRAIN_SHELL: f32 = 0.5 * HAVOK_UNIT;

/// How the ground rubs and bounces: each square's land body is built
/// (`00621f60`) from Havok's default body info (`00c8f510`, restitution
/// 0.4) as a fixed body (mass 0, motion 5) with its friction set to
/// `[Landscape] fLandFriction` (2.5 in the exe and `Fallout.ini`).
pub const LAND_SURFACE: Surface = Surface {
    friction: 2.5,
    restitution: 0.4,
};

/// The Havok world's gravity, (0, 0, −98.1) Havok units a second squared
/// (`00f4b550`: 10 × −9.81, put on z by both world builders), in game
/// units: 686.61.
pub const GRAVITY: f32 = 98.1 * HAVOK_UNIT;

/// A walking character's size: a vertical capsule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterShape {
    pub radius: f32,
    /// From the feet to the top of the head.
    pub height: f32,
    /// The highest ledge it walks up without jumping.
    pub step: f32,
    /// Units per second squared, downward.
    pub gravity: f32,
    /// Steepest ground it stands on: the cosine of the slope.
    pub max_slope_cos: f32,
    /// How far above the feet the shape's bottom floats.
    pub lift: f32,
}

impl CharacterShape {
    /// The player, and every person (the game gives people one fixed size,
    /// not scaled by height; read from its code, `00c72410`): half extents
    /// 23 × 17.5 × 64, so radius (23 + 17.5) / 2 = 20.25 and height 128;
    /// steps up to 31 (`00c6eb00`); ground no steeper than 47°
    /// (`011b0148`, cosine 0.682); the world's gravity, multiplied by one.
    /// The hull's bottom point floats 0.1 Havok units (`lift`, 0.7) above
    /// the feet (twice the 0.05 keep distance, `00c72410`) and its convex
    /// radius is 0.1, so its surface is at the feet; the proxy keeps 0.05
    /// Havok units (0.35) off what it stands on. The shape is the game's
    /// eight-sided hull with a cone underneath 31.7 units tall
    /// ([`CharacterShape::hull`]).
    pub const PLAYER: CharacterShape = CharacterShape {
        radius: 20.25,
        height: 128.0,
        step: 31.0,
        gravity: GRAVITY,
        // cos 47°.
        max_slope_cos: 0.681_998_4,
        lift: 0.1 * HAVOK_UNIT,
    };

    /// The controller's step height in Havok units (`fStepHeight` +0x470):
    /// the cinfo's (31 game units × 0.142875, `00c6da50`), raised to at
    /// least 0.75 × the radius (`00c72410`).
    pub fn step_havok(&self) -> f32 {
        (self.step / HAVOK_UNIT).max(self.radius / HAVOK_UNIT * 0.75)
    }

    /// How far the shape's centre is above the feet (game units): half the
    /// height and twice the proxy's keep distance (`fCenter` +0x538,
    /// `00c72410`: 2 × 0.05 Havok units + (half height − radius + radius)).
    pub fn centre(&self) -> f32 {
        2.0 * proxy::ProxySettings::GAME.keep_distance * HAVOK_UNIT + self.height * 0.5
    }

    /// The controller's hull ([`character_cd::Hull::character`]) for an
    /// upright shape: its capsule's axis points at ±(half height − radius)
    /// round the centre (`00c72410`).
    pub fn hull(&self) -> character_cd::Hull {
        let half = (self.height * 0.5 - self.radius).max(0.0);
        character_cd::Hull::character(
            [0.0, 0.0, -half],
            [0.0, 0.0, half],
            self.radius,
            self.step_havok(),
        )
    }
}

/// A walking character: Bethesda's character controller around Havok's
/// character proxy ([`controller`], [`proxy`]), as every actor in the game
/// walks, with the shape of [`CharacterShape::hull`].
#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    /// Where the feet are, game units (the controller's position less its
    /// centre's height).
    pub feet: Vec3,
    /// The controller's vertical speed, game units a second (up positive).
    pub vertical_speed: f32,
    /// In the on-ground state.
    pub on_ground: bool,
    /// The feet's height when last on the ground.
    pub ground: f32,
    /// The height the feet were at when it last left the ground (jumping
    /// or stepping off, `fFallStartHeight`), and on landing how far it fell
    /// from there (the game measures falls this way, not from the top of a
    /// jump: `00c70550`); taken by whoever applies fall damage.
    pub left_ground_at: f32,
    pub fell: Option<f32>,
    /// The controller's horizontal velocity, game units a second.
    pub horizontal: [f32; 2],
    /// The bodies the last update's proxy touched, for the rigid world to
    /// push (`applySurfaceInteractions`, `00cacf80`; see
    /// [`proxy::SurfaceContact`] and
    /// [`rigid::RigidWorld::apply_surface_interactions`]).
    pub surface: Vec<proxy::SurfaceContact>,
    pub controller: controller::Controller,
}

impl Character {
    pub fn new(feet: Vec3) -> Self {
        let shape = CharacterShape::PLAYER;
        Self {
            feet,
            vertical_speed: 0.0,
            on_ground: true,
            ground: feet[2],
            left_ground_at: feet[2],
            fell: None,
            horizontal: [0.0; 2],
            surface: Vec::new(),
            controller: controller::Controller::new(
                shape.step_havok(),
                controller::gravity_multiplier(shape.gravity),
            ),
        }
    }

    /// One controller update of `dt` seconds wanting to go at `desired`
    /// (x, y; game units a second), as `bhkCharacterController::Move`
    /// (`00c73170`) is handed one move a frame (the frame's displacement
    /// and speed): the support check, the state (on the ground, jumping,
    /// in the air) setting the velocity, and the proxy moving the shape.
    /// `jump_height` (game units) asks for a jump this update (taken on
    /// walkable ground; it leaves the ground the update after, as the
    /// jumping state runs then). `air_gain` is the in-air state's steering
    /// (`fAcrobatics` × 0.3, `world::locomotion::air_gain`).
    pub fn update_controlled(
        &mut self,
        collider: &Collider,
        shape: &CharacterShape,
        desired: [f32; 2],
        jump_height: Option<f32>,
        air_gain: f32,
        dt: f32,
    ) {
        self.surface.clear();
        if dt <= 0.0 {
            return;
        }
        let c = &mut self.controller;
        let step = shape.step_havok();
        if c.listener.step_height != step {
            c.listener = controller::ListenerState {
                flags: c.listener.flags,
                ..controller::ListenerState::new(step)
            };
        }
        c.gravity = controller::gravity_multiplier(shape.gravity);
        if let Some(h) = jump_height.filter(|&h| h > 0.0) {
            c.jump(h);
        }
        let was_on_ground = c.state == controller::State::OnGround;
        let input = controller::MoveInput {
            dt,
            displacement: [desired[0] * dt, desired[1] * dt, 0.0],
            speed: (desired[0] * desired[0] + desired[1] * desired[1]).sqrt(),
            air_gain,
        };
        c.move_by(
            collider,
            &shape.hull(),
            shape.centre(),
            &mut self.feet,
            input,
        );
        let v = c.proxy.velocity;
        self.vertical_speed = v[2] * HAVOK_UNIT;
        self.horizontal = [v[0] * HAVOK_UNIT, v[1] * HAVOK_UNIT];
        self.surface = std::mem::take(&mut c.proxy.surface);
        self.on_ground = c.state == controller::State::OnGround;
        if was_on_ground && !self.on_ground {
            self.left_ground_at = c.fall_start;
        }
        if c.listener.flags & controller::flags::LANDED != 0 {
            self.fell = Some(c.fall_start - self.feet[2]);
        }
        if self.on_ground {
            self.ground = self.feet[2];
        }
    }

    /// [`Character::update_controlled`] steering fully in the air, without
    /// jumping.
    pub fn update(
        &mut self,
        collider: &Collider,
        shape: &CharacterShape,
        velocity: [f32; 2],
        dt: f32,
    ) {
        self.update_controlled(collider, shape, velocity, None, 1.0, dt);
    }
}
/// The closest points between segment `p`–`q` and triangle `a b c`: one on
/// the segment, one on the triangle.
pub(crate) fn segment_triangle_closest(
    p: Vec3,
    q: Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
) -> (Vec3, Vec3) {
    // The segment crossing the triangle: distance zero there.
    let dir = sub(q, p);
    let len = length(dir);
    if len > 1e-9 {
        let unit = scale(dir, 1.0 / len);
        if let Some(t) = ray_triangle(p, unit, a, b, c) {
            if t <= len {
                let hit = add(p, scale(unit, t));
                return (hit, hit);
            }
        }
    }
    let mut best = (p, closest_on_triangle(p, a, b, c));
    let mut best_d = dist2(best.0, best.1);
    let mut consider = |s: Vec3, t: Vec3| {
        let d = dist2(s, t);
        if d < best_d {
            best_d = d;
            best = (s, t);
        }
    };
    consider(q, closest_on_triangle(q, a, b, c));
    for (e0, e1) in [(a, b), (b, c), (c, a)] {
        let (s, t) = segment_segment_closest(p, q, e0, e1);
        consider(s, t);
    }
    best
}

/// The point of triangle `a b c` nearest `p` (Ericson, Real-Time Collision
/// Detection, 5.1.5).
pub(crate) fn closest_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ap = sub(p, a);
    let d1 = dot(ab, ap);
    let d2 = dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = sub(p, b);
    let d3 = dot(ab, bp);
    let d4 = dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return add(a, scale(ab, d1 / (d1 - d3)));
    }
    let cp = sub(p, c);
    let d5 = dot(ab, cp);
    let d6 = dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return add(a, scale(ac, d2 / (d2 - d6)));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        return add(b, scale(sub(c, b), (d4 - d3) / ((d4 - d3) + (d5 - d6))));
    }
    let denom = 1.0 / (va + vb + vc);
    add(a, add(scale(ab, vb * denom), scale(ac, vc * denom)))
}

/// The closest points between segments `p1`–`q1` and `p2`–`q2` (Ericson,
/// 5.1.9).
pub(crate) fn segment_segment_closest(p1: Vec3, q1: Vec3, p2: Vec3, q2: Vec3) -> (Vec3, Vec3) {
    let d1 = sub(q1, p1);
    let d2 = sub(q2, p2);
    let r = sub(p1, p2);
    let a = dot(d1, d1);
    let e = dot(d2, d2);
    let f = dot(d2, r);
    let (s, t);
    if a <= 1e-12 && e <= 1e-12 {
        return (p1, p2);
    }
    if a <= 1e-12 {
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = dot(d1, r);
        if e <= 1e-12 {
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            let b = dot(d1, d2);
            let denom = a * e - b * b;
            let mut s0 = if denom != 0.0 {
                ((b * f - c * e) / denom).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let mut t0 = (b * s0 + f) / e;
            if t0 < 0.0 {
                t0 = 0.0;
                s0 = (-c / a).clamp(0.0, 1.0);
            } else if t0 > 1.0 {
                t0 = 1.0;
                s0 = ((b - c) / a).clamp(0.0, 1.0);
            }
            s = s0;
            t = t0;
        }
    }
    (add(p1, scale(d1, s)), add(p2, scale(d2, t)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`Collider::extend`] as it was: every triangle pushed in again, its
    /// buckets measured again.
    fn extend_one_at_a_time(c: &mut Collider, other: &Collider) {
        let base = c.vertices.len() as u32;
        c.vertices.extend_from_slice(&other.vertices);
        let surfaces: Vec<u16> = other
            .live
            .surfaces
            .iter()
            .map(|&s| c.surface_index(s))
            .collect();
        for tri in &other.triangles {
            let surface = match tri.surface {
                0 => 0,
                s => surfaces.get(usize::from(s) - 1).copied().unwrap_or(0),
            };
            c.push_triangle(
                tri.corners.map(|i| i + base),
                tri.shell,
                tri.owner,
                (tri.material, surface, tri.layer, tri.reference),
            );
        }
        c.live.hidden.extend(other.live.hidden.iter().copied());
        for (&owner, rest) in &other.live.rest {
            c.live
                .rest
                .entry(owner)
                .or_default()
                .extend(rest.iter().map(|&(i, v)| (i + base, v)));
        }
    }

    /// Squares of ground and objects, some owned, some with surfaces, some
    /// triangles without area or with a corner that isn't a number, one
    /// owner moved after.
    fn square(seed: u32, at: [f32; 2]) -> Collider {
        let mut rng = seed.wrapping_mul(2654435761) | 1;
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 17;
            rng ^= rng << 5;
            (rng % 10_000) as f32 / 10_000.0
        };
        let mut c = Collider::new();
        for part in 0..12u32 {
            let mut vertices = Vec::new();
            let mut tris = Vec::new();
            for t in 0..20u32 {
                let x = at[0] + next() * 4096.0;
                let y = at[1] + next() * 4096.0;
                let size = 10.0 + next() * 900.0;
                vertices.push([x, y, next() * 100.0]);
                vertices.push([x + size, y + next() * size, next() * 100.0]);
                vertices.push([x + next() * size, y + size, next() * 100.0]);
                let b = 3 * t;
                tris.push(match t % 9 {
                    // Without area, then out of range.
                    4 => [b, b, b + 1],
                    8 => [b, b + 1, 9999],
                    _ => [b, b + 1, b + 2],
                });
            }
            if part == 5 {
                vertices[0] = [f32::NAN, 0.0, 0.0];
            }
            let owner = if part % 3 == 0 { 0 } else { 0x100 + part % 4 };
            let surface = (part % 2 == 0).then_some(Surface {
                friction: 0.1 * (part % 3) as f32,
                restitution: 0.5,
            });
            c.add_layered(
                &vertices,
                &tris,
                (part as f32, owner, part),
                surface,
                (part % 5) as u8,
            );
        }
        c.set_hidden(0x101, true);
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        c.move_owner(0x102, &turn, [500.0, -300.0, 0.0]);
        c
    }

    /// Gathering the outdoor squares into one collider takes their
    /// triangles and buckets over as they are: the same collider as putting
    /// every triangle in again.
    #[test]
    fn gathered_squares_are_as_if_put_in_one_at_a_time() {
        let squares: Vec<Collider> = (0..5)
            .map(|i| square(i + 1, [4096.0 * (i % 3) as f32, 4096.0 * (i / 3) as f32]))
            .collect();
        let mut fast = Collider::new();
        let mut slow = Collider::new();
        for s in &squares {
            fast.extend(s);
            extend_one_at_a_time(&mut slow, s);
        }
        assert!(slow.triangles.len() > 500);
        assert_eq!(fast.vertices.len(), slow.vertices.len());
        for (a, b) in fast.vertices.iter().zip(&slow.vertices) {
            assert!(a == b || (a[0].is_nan() && b[0].is_nan()));
        }
        assert_eq!(fast.triangles, slow.triangles);
        assert_eq!(fast.grid, slow.grid);
        assert_eq!(fast.live.owned, slow.live.owned);
        assert_eq!(fast.live.placed, slow.live.placed);
        assert_eq!(fast.live.surfaces, slow.live.surfaces);
        assert_eq!(fast.live.hidden, slow.live.hidden);
        assert_eq!(fast.live.rest.len(), slow.live.rest.len());
        // And gathered again from the gathered one (moved owners' buckets
        // aren't in triangle order there).
        let mut twice = Collider::new();
        let mut twice_slow = Collider::new();
        let turn = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        fast.move_owner(0x103, &turn, [10.0, 20.0, 0.0]);
        twice.extend(&fast);
        extend_one_at_a_time(&mut twice_slow, &fast);
        assert_eq!(twice.triangles, twice_slow.triangles);
        assert_eq!(twice.grid, twice_slow.grid);
        assert_eq!(twice.live.owned, twice_slow.live.owned);
        assert_eq!(twice.live.placed, twice_slow.live.placed);
    }

    /// The shell the game's models have around their collision.
    const SHELL: f32 = 0.1 * HAVOK_UNIT;

    /// A room: floor at z = 0 from -500 to 500, a wall at x = 200, a 16-unit
    /// step at x from -200 to -100, and a 64-unit block at y = 200..300;
    /// every surface with a model's shell.
    fn room() -> Collider {
        let mut c = Collider::new();
        let quad =
            |c: &mut Collider, v: [Vec3; 4]| c.add_solid(&v, &[[0, 1, 2], [0, 2, 3]], SHELL, 0);
        quad(
            &mut c,
            [
                [-500.0, -500.0, 0.0],
                [500.0, -500.0, 0.0],
                [500.0, 500.0, 0.0],
                [-500.0, 500.0, 0.0],
            ],
        );
        quad(
            &mut c,
            [
                [200.0, -500.0, 0.0],
                [200.0, 500.0, 0.0],
                [200.0, 500.0, 300.0],
                [200.0, -500.0, 300.0],
            ],
        );
        // The step: top at z = 16 and its front face at x = -100.
        quad(
            &mut c,
            [
                [-200.0, -500.0, 16.0],
                [-100.0, -500.0, 16.0],
                [-100.0, 500.0, 16.0],
                [-200.0, 500.0, 16.0],
            ],
        );
        quad(
            &mut c,
            [
                [-100.0, -500.0, 0.0],
                [-100.0, 500.0, 0.0],
                [-100.0, 500.0, 16.0],
                [-100.0, -500.0, 16.0],
            ],
        );
        // The block's front face at y = 200, 64 high.
        quad(
            &mut c,
            [
                [-50.0, 200.0, 0.0],
                [50.0, 200.0, 0.0],
                [50.0, 200.0, 64.0],
                [-50.0, 200.0, 64.0],
            ],
        );
        quad(
            &mut c,
            [
                [-50.0, 200.0, 64.0],
                [50.0, 200.0, 64.0],
                [50.0, 300.0, 64.0],
                [-50.0, 300.0, 64.0],
            ],
        );
        // A bench: 32 high (a third over a step), front face at y = -200.
        quad(
            &mut c,
            [
                [-50.0, -200.0, 0.0],
                [50.0, -200.0, 0.0],
                [50.0, -200.0, 32.0],
                [-50.0, -200.0, 32.0],
            ],
        );
        quad(
            &mut c,
            [
                [-50.0, -200.0, 32.0],
                [50.0, -200.0, 32.0],
                [50.0, -300.0, 32.0],
                [-50.0, -300.0, 32.0],
            ],
        );
        c
    }

    /// How far the feet stay above a surface: the proxy's keep distance
    /// (0.05 Havok units), the hull's surface being at the feet.
    const KEEP: f32 = 0.05 * HAVOK_UNIT;

    fn run(c: &Collider, player: &mut Character, velocity: [f32; 2], seconds: f32) {
        let steps = (seconds * 60.0).round() as usize;
        for _ in 0..steps {
            player.update(c, &CharacterShape::PLAYER, velocity, 1.0 / 60.0);
        }
    }

    /// Feet standing on a model's surface at height `z`: its shell and the
    /// keep distance above (the cast stops within its early-out, 0.01 Havok
    /// units, of that).
    fn stands_at(feet: f32, z: f32) -> bool {
        let want = z + SHELL + KEEP;
        feet >= want - 0.05 && feet <= want + 0.1
    }

    #[test]
    fn a_flat_sliver_triangle_is_ignored() {
        // Game collision has triangles with no area: two corners the same
        // (the Prospector Saloon's, about 40, −581, 3516), or three in a
        // line.
        let mut c = room();
        c.add(
            &[[0.0, 0.0, 10.0], [0.0, 0.0, 10.0], [1.9, -3.2, 10.7]],
            &[[0, 1, 2]],
        );
        c.add(
            &[[-10.0, 0.0, 10.0], [0.0, 0.0, 10.0], [10.0, 0.0, 10.0]],
            &[[0, 1, 2]],
        );
        let mut p = Character::new([0.0, 0.0, 2.0]);
        run(&c, &mut p, [0.0, 0.0], 0.5);
        assert!(p.feet.iter().all(|v| v.is_finite()), "{:?}", p.feet);
        run(&c, &mut p, [50.0, 0.0], 0.5);
        assert!(p.feet.iter().all(|v| v.is_finite()), "{:?}", p.feet);
    }

    #[test]
    fn falls_onto_the_floor_lands_and_stands_there() {
        let c = room();
        let mut p = Character::new([0.0, 0.0, 100.0]);
        run(&c, &mut p, [0.0, 0.0], 0.1);
        assert!(!p.on_ground, "falling");
        run(&c, &mut p, [0.0, 0.0], 2.0);
        assert!(p.on_ground);
        assert!(stands_at(p.feet[2], 0.0), "{:?}", p.feet);
        // Measured from where it left the ground: the start.
        let fell = p.fell.expect("landed");
        assert!((fell - 100.0).abs() < 2.0, "{fell}");
        // Standing still: no velocity, no drift.
        let at = p.feet;
        run(&c, &mut p, [0.0, 0.0], 1.0);
        assert_eq!(p.feet, at);
        assert_eq!(p.controller.proxy.velocity, [0.0; 3]);
    }

    #[test]
    fn stops_at_a_wall_its_radius_the_shells_and_the_keep_distance_away() {
        let c = room();
        let mut p = Character::new([0.0, 0.0, 1.05]);
        run(&c, &mut p, [300.0, 0.0], 2.0);
        // The hull's corner points along x: radius + its convex radius.
        let r = CharacterShape::PLAYER.radius + 0.1 * HAVOK_UNIT;
        let x = 200.0 - SHELL - KEEP - r;
        assert!((p.feet[0] - x).abs() < 0.1, "{:?} {x}", p.feet);
        assert!(p.feet[1].abs() < 0.01, "{:?}", p.feet);
        assert!(stands_at(p.feet[2], 0.0), "{:?}", p.feet);
    }

    #[test]
    fn triangles_without_area_are_left_out() {
        let mut c = room();
        let before = c.triangle_count();
        c.add(
            &[[-50.0, 0.0, 10.0], [0.0, 0.0, 10.0], [50.0, 0.0, 10.0]],
            &[[0, 1, 2]],
        );
        assert_eq!(c.triangle_count(), before);
        let mut p = Character::new([0.0, -60.0, 1.05]);
        run(&c, &mut p, [0.0, 100.0], 1.0);
        assert!(p.feet.iter().all(|v| v.is_finite()), "{:?}", p.feet);
        assert!((p.feet[1] - 40.0).abs() < 1.0, "{:?}", p.feet);
    }

    #[test]
    fn switched_off_objects_dont_block() {
        // A door's leaf across the way, owned by the door (form 0x123).
        let mut c = room();
        c.add_solid(
            &[
                [-100.0, 100.0, 0.0],
                [100.0, 100.0, 0.0],
                [100.0, 100.0, 200.0],
                [-100.0, 100.0, 200.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
            SHELL,
            0x123,
        );
        assert!(c.owns(0x123) && !c.owns(0x124));
        let mut p = Character::new([0.0, 0.0, 1.05]);
        run(&c, &mut p, [0.0, 100.0], 1.0);
        assert!(p.feet[1] < 100.0 - 20.0, "closed: {:?}", p.feet);
        // Open: walked through, and rays pass.
        c.set_hidden(0x123, true);
        assert!(c
            .raycast([0.0, 0.0, 50.0], [0.0, 1.0, 0.0], 150.0)
            .is_none());
        let (_, t) = c
            .raycast_including_hidden([0.0, 0.0, 50.0], [0.0, 1.0, 0.0], 150.0)
            .unwrap();
        assert_eq!(c.owner(t), 0x123);
        let mut p = Character::new([0.0, 0.0, 1.05]);
        run(&c, &mut p, [0.0, 100.0], 1.5);
        assert!(p.feet[1] > 120.0, "open: {:?}", p.feet);
        let mut merged = Collider::new();
        merged.extend(&c);
        assert!(merged.is_hidden(0x123) && merged.owns(0x123));
    }

    #[test]
    fn people_are_walls_nobody_stands_on() {
        let mut c = room();
        let other = [0.0, 100.0, 1.05];
        c.set_people(vec![Person {
            feet: other,
            radius: 20.25,
            height: 128.0,
        }]);
        // Walking into them: never inside their hull (their contacts are
        // vertical walls), sliding round it, on the floor all the way.
        let mut p = Character::new([5.0, 0.0, 1.05]);
        for _ in 0..90 {
            p.update(&c, &CharacterShape::PLAYER, [0.0, 150.0], 1.0 / 60.0);
            let apart = (p.feet[0] - other[0]).hypot(p.feet[1] - other[1]);
            // Two octagons of radius 20.25 (their flats 18.7 out).
            assert!(apart > 2.0 * 18.7, "{apart} {:?}", p.feet);
            assert!(stands_at(p.feet[2], 0.0), "{:?}", p.feet);
        }
        // Dropped on their head: their top is a wall too, so it slides off
        // to the floor.
        let mut p = Character::new([5.0, 100.0, 140.0]);
        run(&c, &mut p, [0.0, 0.0], 3.0);
        assert!(p.on_ground && stands_at(p.feet[2], 0.0), "{:?}", p.feet);
    }
    #[test]
    fn walks_up_a_step_but_not_a_bench_or_a_block() {
        // The 16-unit step: under the 31-unit step height.
        let c = room();
        let mut p = Character::new([0.0, 0.0, 1.05]);
        run(&c, &mut p, [-200.0, 0.0], 0.65);
        run(&c, &mut p, [0.0, 0.0], 1.0);
        assert!(p.feet[0] < -110.0, "went on: {:?}", p.feet);
        assert!(stands_at(p.feet[2], 16.0), "onto the step: {:?}", p.feet);
        assert!(p.feet[1].abs() < 0.5, "straight on: {:?}", p.feet);
        // The 32-unit bench: over a step.
        let mut p = Character::new([0.0, -100.0, 1.05]);
        run(&c, &mut p, [0.0, -200.0], 2.0);
        assert!(p.feet[1] > -200.0, "went through: {:?}", p.feet);
        assert!(stands_at(p.feet[2], 0.0), "climbed: {:?}", p.feet);
        // The 64-unit block.
        let mut p = Character::new([0.0, 100.0, 1.05]);
        run(&c, &mut p, [0.0, 200.0], 2.0);
        assert!(
            p.feet[1] < 200.0 && stands_at(p.feet[2], 0.0),
            "{:?}",
            p.feet
        );
    }

    /// A ramp rising along +x from x = 0 at `degrees`, with a floor before
    /// it.
    fn ramp(degrees: f32) -> Collider {
        let mut c = Collider::new();
        let rise = 1000.0 * degrees.to_radians().tan();
        c.add_solid(
            &[
                [-500.0, -500.0, 0.0],
                [0.0, -500.0, 0.0],
                [0.0, 500.0, 0.0],
                [-500.0, 500.0, 0.0],
                [1000.0, -500.0, rise],
                [1000.0, 500.0, rise],
            ],
            &[[0, 1, 2], [0, 2, 3], [1, 4, 5], [1, 5, 2]],
            SHELL,
            0,
        );
        c
    }

    #[test]
    fn walks_up_slopes_to_47_degrees_and_not_steeper() {
        for (degrees, climbs) in [(30.0, true), (45.0, true), (50.0, false), (60.0, false)] {
            let c = ramp(degrees);
            let mut p = Character::new([-60.0, 0.0, 1.05]);
            run(&c, &mut p, [0.0, 0.0], 0.2);
            run(&c, &mut p, [200.0, 0.0], 3.0);
            if climbs {
                assert!(p.feet[2] > 100.0, "{degrees}: {:?}", p.feet);
                assert!(p.on_ground, "{degrees}: {:?}", p.feet);
            } else {
                assert!(p.feet[2] < 32.0, "{degrees}: climbed {:?}", p.feet);
            }
        }
    }

    #[test]
    fn stands_on_a_box_and_walks_off_its_edge() {
        let c = room();
        // On top of the 64-unit block (y 200 to 300).
        let mut p = Character::new([0.0, 250.0, 70.0]);
        run(&c, &mut p, [0.0, 0.0], 0.5);
        assert!(p.on_ground && stands_at(p.feet[2], 64.0), "{:?}", p.feet);
        p.fell = None;
        // South off its front edge at y = 200.
        run(&c, &mut p, [0.0, -150.0], 1.5);
        assert!(p.on_ground && stands_at(p.feet[2], 0.0), "{:?}", p.feet);
        let fell = p.fell.expect("landed");
        assert!((fell - 64.0).abs() < 2.0, "{fell}");
    }

    #[test]
    fn a_jump_rises_its_height_keeps_its_run_up_and_lands() {
        let c = room();
        let shape = CharacterShape::PLAYER;
        let dt = 1.0 / 60.0;
        let mut p = Character::new([120.0, 250.0, 1.05]);
        run(&c, &mut p, [0.0, 0.0], 0.2);
        let start = p.feet[2];
        // Running south, then the jump (`fJumpHeightMin` 64).
        for _ in 0..10 {
            p.update_controlled(&c, &shape, [0.0, -308.0], None, 0.3, dt);
        }
        assert!((p.horizontal[1] + 308.0).abs() < 1.0, "{:?}", p.horizontal);
        p.update_controlled(&c, &shape, [0.0, -308.0], Some(64.0), 0.3, dt);
        // Asked on the ground; it leaves the update after.
        p.update_controlled(&c, &shape, [0.0, -308.0], None, 0.3, dt);
        assert!(!p.on_ground);
        // The jumping state adds √(2 g h) up to the proxy's velocity: the
        // run-up is kept (`00cd4280`).
        assert!((p.horizontal[1] + 308.0).abs() < 1.0, "{:?}", p.horizontal);
        let mut highest = p.feet[2];
        for _ in 0..120 {
            p.update_controlled(&c, &shape, [0.0, -308.0], None, 0.3, dt);
            highest = highest.max(p.feet[2]);
            if p.on_ground {
                break;
            }
        }
        assert!((highest - start - 64.0).abs() < 4.0, "{highest} {start}");
        assert!(p.on_ground && stands_at(p.feet[2], 0.0), "{:?}", p.feet);
        assert!(p.fell.is_some_and(|f| f.abs() < 1.0), "{:?}", p.fell);
    }

    #[test]
    fn sunk_waist_deep_in_the_land_comes_up_at_the_recovery_speed() {
        // Terrain: a big flat square with the land's 0.5 Havok-unit shell.
        let mut c = Collider::new();
        c.add_solid(
            &[
                [-2000.0, -2000.0, 0.0],
                [2000.0, -2000.0, 0.0],
                [2000.0, 2000.0, 0.0],
                [-2000.0, 2000.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
            TERRAIN_SHELL,
            0,
        );
        let surface = TERRAIN_SHELL + KEEP;
        // Away from the square's diagonal (an edge both its triangles share).
        let mut p = Character::new([-500.0, -300.0, -40.0]);
        run(&c, &mut p, [0.0, 0.0], 0.5);
        // Standing still in an unchanged manifold the proxy isn't
        // integrated at all (`00c73170`): it stays sunk (but for the first
        // update, which falls before the manifold is there).
        let sunk = p.feet[2];
        assert!(sunk < -38.0, "{sunk}");
        run(&c, &mut p, [0.0, 0.0], 1.0);
        assert_eq!(p.feet[2], sunk);
        // Walking, the crossing becomes a velocity out of the land at the
        // penetration recovery speed (1 a second for each Havok unit sunk,
        // `00cacd60`): about 1 − e^-0.5 of the way up in half a second.
        run(&c, &mut p, [150.0, 0.0], 0.5);
        let depth = surface - sunk;
        let risen = p.feet[2] - sunk;
        assert!(
            risen > 0.25 * depth && risen < 0.6 * depth,
            "{risen} of {depth}"
        );
        run(&c, &mut p, [150.0, 0.0], 6.0);
        assert!((p.feet[2] - surface).abs() < 1.0, "{:?}", p.feet);
    }
    #[test]
    fn walks_straight_across_a_triangulated_floor() {
        // The floor quad's diagonal runs through the start: its edge
        // mustn't push the shape sideways.
        let c = room();
        let mut p = Character::new([0.0, 0.0, 1.05]);
        run(&c, &mut p, [-60.0, 0.0], 1.0);
        assert!(
            p.feet[1].abs() < 0.01 && (p.feet[0] + 60.0).abs() < 1.0,
            "{:?}",
            p.feet
        );
    }
    #[test]
    fn merged_colliders_keep_every_surface() {
        let mut merged = Collider::new();
        merged.extend(&room());
        let mut far = Collider::new();
        far.add(
            &[[5000.0, 0.0, 0.0], [5100.0, 0.0, 0.0], [5000.0, 100.0, 0.0]],
            &[[0, 1, 2]],
        );
        merged.extend(&far);
        assert_eq!(merged.triangle_count(), room().triangle_count() + 1);
        let down = [0.0, 0.0, -1.0];
        assert!(merged.raycast([5010.0, 10.0, 50.0], down, 100.0).is_some());
        assert!(merged.raycast([0.0, 0.0, 50.0], down, 100.0).is_some());
    }

    #[test]
    fn rays_hit_the_nearest_surface() {
        let c = room();
        let (d, _) = c
            .raycast([0.0, 0.0, 50.0], [1.0, 0.0, 0.0], 1000.0)
            .unwrap();
        assert!((d - 200.0).abs() < 1e-3);
        assert!(c
            .raycast([0.0, 0.0, 50.0], [1.0, 0.0, 0.0], 100.0)
            .is_none());
    }

    #[test]
    fn an_owned_object_moves_from_its_added_place() {
        // A door's leaf: a wall across y = 100 belonging to 0x904, and a
        // wall of nobody's at x = 300.
        let mut c = Collider::new();
        let quad = |c: &mut Collider, v: [[f32; 3]; 4], owner: u32| {
            c.add_solid(&v, &[[0, 1, 2], [0, 2, 3]], 0.0, owner)
        };
        quad(
            &mut c,
            [
                [-40.0, 100.0, 0.0],
                [40.0, 100.0, 0.0],
                [40.0, 100.0, 200.0],
                [-40.0, 100.0, 200.0],
            ],
            0x904,
        );
        quad(
            &mut c,
            [
                [300.0, -40.0, 0.0],
                [300.0, 40.0, 0.0],
                [300.0, 40.0, 200.0],
                [300.0, -40.0, 200.0],
            ],
            0,
        );
        let north = |c: &Collider| c.raycast([0.0, 0.0, 100.0], [0.0, 1.0, 0.0], 500.0);
        let east = |c: &Collider| c.raycast([0.0, 0.0, 100.0], [1.0, 0.0, 0.0], 500.0);
        assert!(north(&c).is_some() && east(&c).is_some());
        // Turned a quarter turn about the origin (x → y) and moved 1000
        // north: it's off the ray and bucketed where it now is.
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        c.move_owner(0x904, &turn, [0.0, 1000.0, 0.0]);
        assert!(north(&c).is_none());
        let there = c.raycast([-200.0, 1000.0, 100.0], [1.0, 0.0, 0.0], 500.0);
        assert!(
            there.is_some_and(|(d, _)| (d - 100.0).abs() < 1e-3),
            "{there:?}"
        );
        // From its added place, not from where it was: back to the start.
        c.move_owner(
            0x904,
            &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [0.0; 3],
        );
        assert!(north(&c).is_some_and(|(d, _)| (d - 100.0).abs() < 1e-3));
        // Nobody's wall never moved; an unknown owner does nothing.
        assert!(east(&c).is_some_and(|(d, _)| (d - 300.0).abs() < 1e-3));
        c.move_owner(0x999, &turn, [0.0; 3]);
        c.move_owner(0, &turn, [0.0; 3]);
        assert!(east(&c).is_some());
        // Gathered into another collider, the object still moves there.
        let mut all = Collider::new();
        all.extend(&c);
        all.move_owner(0x904, &turn, [0.0, 1000.0, 0.0]);
        assert!(north(&all).is_none());
    }

    /// Moving an owner leaves every bucket's list as taking its triangles
    /// out one at a time and putting them back does (the order queries see
    /// them in).
    #[test]
    fn moving_an_owner_leaves_the_buckets_as_one_at_a_time() {
        let mut c = Collider::new();
        // Ground under everything, and an object of many triangles on it.
        let mut ground = Vec::new();
        let mut tris = Vec::new();
        for i in 0..20 {
            let x = i as f32 * 50.0 - 500.0;
            let b = ground.len() as u32;
            ground.extend([[x, -500.0, 0.0], [x + 60.0, -500.0, 0.0], [x, 500.0, 0.0]]);
            tris.push([b, b + 1, b + 2]);
        }
        c.add(&ground, &tris);
        let mut body = Vec::new();
        let mut body_tris = Vec::new();
        for i in 0..12 {
            let a = i as f32 * 0.5;
            let b = body.len() as u32;
            body.extend([
                [a.cos() * 40.0, a.sin() * 40.0, 10.0],
                [a.cos() * 90.0, a.sin() * 90.0, 30.0],
                [0.0, 0.0, 60.0],
            ]);
            body_tris.push([b, b + 1, b + 2]);
        }
        c.add_solid_surface(&body, &body_tris, (0.0, 0x77, NO_MATERIAL), None);
        let one_at_a_time = |c: &mut Collider, r: &[[f32; 3]; 3], t: Vec3| {
            let owned = c.live.owned[&0x77].clone();
            for &tri in &owned {
                let [a, b, cc] = c.triangle(tri);
                for key in bucket_keys(a, b, cc) {
                    if let Some(list) = c.grid.get_mut(&key) {
                        list.retain(|&i| i != tri);
                    }
                }
            }
            for &(i, p) in &c.live.rest[&0x77].clone() {
                let q = [0, 1, 2].map(|k| r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2]);
                c.vertices[i as usize] = add(q, t);
            }
            for &tri in &owned {
                let [a, b, cc] = c.triangle(tri);
                for key in bucket_keys(a, b, cc) {
                    c.grid.entry(key).or_default().push(tri);
                }
            }
        };
        let mut expected = c.clone();
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        for (r, t) in [(turn, [300.0, 20.0, 0.0]), (turn, [-280.0, 500.0, 5.0])] {
            c.move_owner(0x77, &r, t);
            one_at_a_time(&mut expected, &r, t);
            assert_eq!(c.grid, expected.grid);
            assert_eq!(c.vertices, expected.vertices);
        }
    }

    #[test]
    fn a_swept_sphere_stops_its_radius_and_the_shell_short_of_a_wall() {
        let c = room();
        let hit = c
            .spherecast([0.0, 0.0, 50.0], [1.0, 0.0, 0.0], 1000.0, 10.0)
            .unwrap();
        // The wall at x = 200 stands SHELL out of its triangles: the
        // sphere's centre stops 10 + SHELL before it, touching the wall
        // straight ahead.
        assert!(
            (hit.distance - (200.0 - 10.0 - SHELL)).abs() < 1e-3,
            "{hit:?}"
        );
        assert!((hit.point[0] - (200.0 - SHELL)).abs() < 1e-3, "{hit:?}");
        assert!(hit.point[1].abs() < 1e-3 && (hit.point[2] - 50.0).abs() < 1e-3);
        // Out of reach: nothing.
        assert!(c
            .spherecast([0.0, 0.0, 50.0], [1.0, 0.0, 0.0], 150.0, 10.0)
            .is_none());
        // Already touching: distance 0 and the nearest point.
        let touching = c
            .spherecast([195.0, 0.0, 50.0], [1.0, 0.0, 0.0], 100.0, 10.0)
            .unwrap();
        assert_eq!(touching.distance, 0.0);
        assert!((touching.point[0] - (200.0 - SHELL)).abs() < 1e-3);
    }

    #[test]
    fn a_swept_sphere_catches_a_walls_edge_a_ray_would_miss() {
        let c = room();
        // Past the wall's end (y = 500): a ray misses, but a sphere 5 units
        // beyond the edge touches it.
        assert!(c
            .raycast([0.0, 505.0, 50.0], [1.0, 0.0, 0.0], 1000.0)
            .is_none());
        let hit = c
            .spherecast([0.0, 505.0, 50.0], [1.0, 0.0, 0.0], 1000.0, 10.0)
            .unwrap();
        let r = 10.0 + SHELL;
        let expected = 200.0 - (r * r - 5.0f32 * 5.0).sqrt();
        assert!((hit.distance - expected).abs() < 1e-2, "{hit:?}");
        // Touched on the edge's shell, SHELL out from the edge toward the
        // sphere's centre.
        let edge = [200.0, 500.0, 50.0];
        let off = sub(hit.point, edge);
        assert!((length(off) - SHELL).abs() < 1e-3, "{hit:?}");
        assert!(off[0] < 0.0 && off[1] > 0.0, "{hit:?}");
        // Far enough past it: nothing.
        assert!(c
            .spherecast([0.0, 512.0, 50.0], [1.0, 0.0, 0.0], 1000.0, 10.0)
            .is_none());
    }

    #[test]
    fn a_swept_sphere_passes_switched_off_objects() {
        let mut c = room();
        c.add_solid(
            &[
                [-100.0, 100.0, 0.0],
                [100.0, 100.0, 0.0],
                [100.0, 100.0, 200.0],
                [-100.0, 100.0, 200.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
            SHELL,
            0x123,
        );
        let hit = c
            .spherecast([0.0, 0.0, 50.0], [0.0, 1.0, 0.0], 400.0, 10.0)
            .unwrap();
        assert_eq!(c.owner(hit.triangle), 0x123);
        c.set_hidden(0x123, true);
        assert!(c
            .spherecast([0.0, 0.0, 50.0], [0.0, 1.0, 0.0], 150.0, 10.0)
            .is_none());
    }

    #[test]
    fn shots_pass_a_transparent_fence_that_stops_walkers() {
        // A chain-link fence on layer 3 (TRANSPARENT) 100 units north, a
        // wall on layer 1 (STATIC) behind it.
        let mut c = Collider::new();
        let plane = |y: f32| {
            [
                [-100.0, y, 0.0],
                [100.0, y, 0.0],
                [100.0, y, 200.0],
                [-100.0, y, 200.0],
            ]
        };
        let quad = [[0, 1, 2], [0, 2, 3]];
        c.add_layered(&plane(100.0), &quad, (0.0, 0x31, NO_MATERIAL), None, 3);
        c.add_layered(&plane(200.0), &quad, (0.0, 0x32, NO_MATERIAL), None, 1);
        let north = [0.0, 1.0, 0.0];
        // Unfiltered (and for the character, layer 30) the fence is met.
        let (d, t) = c.raycast([0.0, 0.0, 50.0], north, 500.0).unwrap();
        assert!((d - 100.0).abs() < 1e-3 && c.owner(t) == 0x31);
        let (_, t) = c.raycast_layer([0.0, 0.0, 50.0], north, 500.0, 30).unwrap();
        assert_eq!(c.owner(t), 0x31);
        // A projectile's cast (layer 6) goes through to the wall.
        let (d, t) = c.raycast_layer([0.0, 0.0, 50.0], north, 500.0, 6).unwrap();
        assert!((d - 200.0).abs() < 1e-3 && c.owner(t) == 0x32, "{d}");
        assert_eq!(c.layer(t), 1);
        // Triangles added without a layer stop every cast.
        let mut plain = Collider::new();
        plain.add(&plane(100.0), &quad);
        assert!(plain
            .raycast_layer([0.0, 0.0, 50.0], north, 500.0, 6)
            .is_some());
    }

    #[test]
    fn the_reference_speed_is_the_default_until_set_and_zero_is_ignored() {
        // 77 × 4 with SpeedMult 100 and sound legs (`00647f00`); `0055e230`
        // keeps nothing that isn't above zero.
        assert_eq!(controller::reference_speed(), 77.0 * 4.0);
        controller::set_reference_speed(0.0);
        controller::set_reference_speed(-5.0);
        assert_eq!(controller::reference_speed(), controller::REFERENCE_SPEED);
    }

    #[test]
    fn a_touching_character_hands_the_body_to_the_world_each_update() {
        // A crate beside the character: its triangles carry the reference
        // the world's body has.
        let (h, y) = (10.0f32, 20.25 + 10.0 + 5.0);
        let corners: Vec<Vec3> = (0..8)
            .map(|k| {
                [
                    if k & 1 == 0 { -h } else { h },
                    y + if k & 2 == 0 { -h } else { h },
                    if k & 4 == 0 { 0.0 } else { 2.0 * h },
                ]
            })
            .collect();
        let faces: [[u32; 3]; 12] = [
            [0, 2, 1],
            [1, 2, 3],
            [4, 5, 6],
            [5, 7, 6],
            [0, 1, 4],
            [1, 5, 4],
            [2, 6, 3],
            [3, 6, 7],
            [0, 4, 2],
            [2, 4, 6],
            [1, 3, 5],
            [3, 7, 5],
        ];
        let mut c = Collider::new();
        c.add(
            &[
                [-500.0, -500.0, 0.0],
                [500.0, -500.0, 0.0],
                [500.0, 500.0, 0.0],
                [-500.0, 500.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c.add_layered(&corners, &faces, (0.0, 55, NO_MATERIAL), None, 4);
        let shape = CharacterShape::PLAYER;
        let mut who = Character::new([0.0, 0.0, 0.0]);
        let mut seen = Vec::new();
        for _ in 0..30 {
            who.update_controlled(&c, &shape, [0.0, 200.0], None, 1.0, 1.0 / 60.0);
            seen.extend(who.surface.iter().copied());
        }
        let touch = seen.first().expect("the crate was touched");
        assert_eq!(touch.reference, 55);
        // Its face looks back at the character (south, tilted by the hull's
        // rounded front), the proxy moves
        // north at the move's speed.
        assert!(touch.normal[1] < -0.5, "{:?}", touch.normal);
        assert!(touch.velocity[1] > 0.0, "{:?}", touch.velocity);
        assert!((touch.dt - 1.0 / 60.0).abs() < 1e-6);
        // Not touching (an update with no move): nothing is kept.
        who.update_controlled(&c, &shape, [0.0, 0.0], None, 1.0, 0.0);
        assert!(who.surface.is_empty());
    }
}
