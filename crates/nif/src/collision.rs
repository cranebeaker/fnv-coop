//! Collision: the Havok shapes a model carries for physics, turned into
//! plain geometry in the model's space (game units).
//!
//! A node's collision link leads to a `bhkCollisionObject`, which names a
//! rigid body (`bhkRigidBody`, or `bhkRigidBodyT` with its own offset and
//! rotation), which names a shape tree. Shape coordinates are in Havok's
//! units: seven game units each. Layouts were checked byte for byte against
//! the game's files (`nvinspect <file> block <N>`): every field accounted
//! for in each block's recorded size.

use std::collections::BTreeMap;

use crate::blocks::Block;
use crate::error::Result;
use crate::file::Nif;
use crate::math::{mat_vec, Mat3, Transform, Vec3};
use crate::reader::Reader;

/// Game units per Havok unit: the exe's 69.99125671 game units a metre
/// over 10 (`00f38070`; a Havok unit is a decimetre).
pub const HAVOK_SCALE: f32 = 6.999_125_7;

/// Deeper shape trees than this only happen in corrupt files.
const MAX_SHAPE_DEPTH: usize = 16;

/// Havok collision layers: the low seven bits of a body's collision filter
/// (the first byte of a NIF's `HavokFilter`), and which of them a walking
/// character runs into.
pub mod layers {
    pub const UNIDENTIFIED: u8 = 0;
    pub const STATIC: u8 = 1;
    pub const ANIM_STATIC: u8 = 2;
    pub const TRANSPARENT: u8 = 3;
    pub const CLUTTER: u8 = 4;
    pub const WEAPON: u8 = 5;
    pub const BIPED: u8 = 8;
    pub const TREES: u8 = 9;
    pub const PROPS: u8 = 10;
    pub const WATER: u8 = 11;
    pub const TRIGGER: u8 = 12;
    pub const TERRAIN: u8 = 13;
    pub const NON_COLLIDABLE: u8 = 15;
    pub const GROUND: u8 = 17;
    pub const DEBRIS_SMALL: u8 = 19;
    pub const DEBRIS_LARGE: u8 = 20;
    pub const TRANSPARENT_SMALL: u8 = 26;
    pub const INVISIBLE_WALL: u8 = 27;
    pub const TRANSPARENT_SMALL_ANIM: u8 = 28;
    pub const DEAD_BIPED: u8 = 29;
    pub const CHAR_CONTROLLER: u8 = 30;

    /// The game's names for its 43 layers, by number: its own table at
    /// `011b0810`, which the collision filter's description (`00c849b0`,
    /// "-LAYER") prints from.
    pub const NAMES: [&str; 43] = [
        "UNIDENTIFIED",
        "STATIC",
        "ANIMSTATIC",
        "TRANSPARENT",
        "CLUTTER",
        "WEAPON",
        "PROJECTILE",
        "SPELL",
        "BIPED",
        "TREES",
        "PROPS",
        "WATER",
        "TRIGGER",
        "TERRAIN",
        "TRAP",
        "NONCOLLIDABLE",
        "CLOUDTRAP",
        "GROUND",
        "PORTAL",
        "DEBRIS SMALL",
        "DEBRIS LARGE",
        "ACOUSTIC SPACE",
        "ACTORZONE",
        "PROJECTILEZONE",
        "GASTRAP",
        "SHELLCASING",
        "TRANSPARENT SMALL",
        "INVISIBLE WALL",
        "TRANSPARENT SMALL ANIM",
        "DEADBIP",
        "CHARCONTROLLER",
        "AVOIDBOX",
        "COLLISIONBOX",
        "CAMERASPHERE",
        "DOORDETECTION",
        "CAMERAPICK",
        "ITEMPICK",
        "LINEOFSIGHT",
        "PATHPICK",
        "CUSTOMPICK1",
        "CUSTOMPICK2",
        "SPELLEXPLOSION",
        "DROPPINGPICK",
    ];

    /// A layer's name as the game prints it, or its number.
    pub fn name(layer: u8) -> String {
        NAMES
            .get(usize::from(layer))
            .map_or_else(|| format!("layer {layer}"), |n| n.to_string())
    }

    /// Bit `n` set: a character controller (layer 30, the player's and
    /// every person's) collides with layer `n`. Row 30 of the game's layer
    /// matrix, as `00c828f0` builds it at startup (every bit set, then
    /// pairs cleared both ways and single rows masked; worked out from all
    /// of its statements). The filter (`00c84740`) asks it for bodies in
    /// different groups once neither carries the "no collision" flag; the
    /// row and the column agree for layer 30. Not with: biped (living
    /// people's bone bodies), non-collidable, portal, small debris,
    /// projectile zones, shell casings, the dead (`DEADBIP`), camera
    /// spheres, door detection, line of sight and path picks.
    pub const CHARACTER_COLLIDES_WITH: u64 = 0x0799_DD73_7EFF;

    /// Whether a solid body on this layer stops a walking character: the
    /// game's layer matrix for the character controller
    /// ([`CHARACTER_COLLIDES_WITH`]). Phantoms (triggers, acoustic spaces)
    /// on layers it collides with are never solid; see
    /// [`super::Collision`].
    pub fn blocks_walking(layer: u8) -> bool {
        layer < 43 && CHARACTER_COLLIDES_WITH >> layer & 1 == 1
    }

    /// The filter's "no collision" bit (`0x4000` of the filter, `0x40` of
    /// the NIF's flags byte): such a body collides with nothing
    /// (`00c84740`), whatever its layer.
    pub const NO_COLLISION_FLAG: u8 = 0x40;
}

/// One solid piece of a model's collision, in the model's space.
#[derive(Debug, Clone, PartialEq)]
pub enum CollisionShape {
    /// A triangle mesh (architecture, mostly).
    Triangles {
        vertices: Vec<Vec3>,
        triangles: Vec<[u32; 3]>,
    },
    /// A convex hull: its corners, and its faces as planes `n·p + d = 0`
    /// with `n` pointing out (boxes become these too).
    Convex {
        vertices: Vec<Vec3>,
        planes: Vec<[f32; 4]>,
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
}

/// A shape with the Havok layer its body (or sub-part) is on.
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionPart {
    pub layer: u8,
    /// The body moves under physics (clutter you can knock over) rather
    /// than staying put.
    pub dynamic: bool,
    /// The body is keyframed: moved by the model's own animation, such as a
    /// door's leaf swinging open (Havok motion system 6).
    pub keyframed: bool,
    /// The node (block index) the collision object hangs on.
    pub node: usize,
    /// The nodes from the top of the file down to that node, each with its
    /// local transform (the top node's left out for placed collision, as
    /// `Mesh::nodes` leaves it out of placed scenes): what a sequence moves
    /// when it animates the body (a door's leaf), see `scene::posed_chain`.
    pub nodes: Vec<(String, Transform)>,
    /// The `bhkCollisionObject`'s flags as stored (`0x1` active, `0x4`
    /// notify, `0x8` set local, `0x40` reset, `0x80` synced to its node on
    /// every update).
    pub flags: u16,
    /// How far the solid surface stands out from the shape (Havok's convex
    /// radius, game units): boxes, hulls and triangle meshes carry one (0.1
    /// Havok units, 0.7 game units, in every game model); a sphere's or
    /// capsule's radius is its size, so theirs is 0.
    pub shell: f32,
    pub shape: CollisionShape,
    /// The Havok material (what a shot striking it sounds and looks like:
    /// `world::impacts::Material::from_havok`): the shape's own `material`
    /// field, or a packed triangle sub-part's. 0 (stone) for shapes that
    /// don't name one (the list and tree shapes pass their children's on).
    pub material: u32,
    /// The rigid body the shape belongs to, as the file stores it: what
    /// moves it when it's simulated (mass, inertia, damping, friction...).
    pub body: RigidBodyInfo,
}

/// A `bhkRigidBody`'s physical values (the `hkpRigidBodyCinfo` the game
/// hands Havok), moved into the model's space and game units. Offsets in
/// the block (236 bytes, checked byte for byte against
/// `clutter\junk\ssbottle02.nif`: mass 1, damping 0.1/0.05, friction 0.5,
/// restitution 0.4, most speeds 1068/31.57, penetration depth 0.15, motion
/// 4, deactivator 2, solver deactivation 2, quality 3): the inertia
/// tensor's three rows (four floats each) at 116, the centre of mass at
/// 164, mass 180, linear and angular damping 184/188, friction 192,
/// restitution 196, most linear and angular speed 200/204, penetration
/// depth 208, then four bytes: motion system 212, deactivator type 213,
/// solver deactivation 214, quality type 215.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RigidBodyInfo {
    /// The `bhkRigidBody(T)` block: parts with the same block are one body.
    pub block: usize,
    /// Havok's mass units (the game calls them kilograms).
    pub mass: f32,
    /// The centre of mass, in the model's space (game units).
    pub center: Vec3,
    /// The inertia tensor about the centre of mass, turned into the
    /// model's space (mass × game units²), row-major.
    pub inertia: Mat3,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub friction: f32,
    pub restitution: f32,
    /// The fastest it moves (game units a second) and turns (radians a
    /// second).
    pub max_linear_speed: f32,
    pub max_angular_speed: f32,
    /// Havok's allowed penetration (game units).
    pub penetration_depth: f32,
    /// Havok's motion system (`hkpMotion::MotionType` as stored: 1
    /// dynamic, 2 sphere inertia, 3 sphere stabilized, 4 box inertia, 5
    /// box stabilized, 6 keyframed, 7 fixed, 8 thin box, 9 character).
    pub motion: u8,
    pub deactivator: u8,
    pub solver_deactivation: u8,
    pub quality: u8,
    /// How many constraints (`bhk…Constraint` blocks: hinges, ragdoll
    /// joints, springs) join it to other bodies (228).
    pub constraints: u32,
    /// The body's flags (`bhkWorldObject::FLAGS`, Xbox PDB): bit 0
    /// [`BODY_WIND`], the rest runtime state. The block's last word.
    pub body_flags: u32,
}

/// `bhkWorldObject::FLAGS::WIND` (Xbox PDB): the body is pushed by the
/// wind (`bhkWindListener`, `physics::wind`).
pub const BODY_WIND: u32 = 1;

/// `BSXFlags` bit 1, "Havok": the game's `00c6a350` only changes the
/// motion of a model's bodies without being forced when it's set.
pub const BSX_HAVOK: u32 = 2;

impl Nif {
    /// The model's `BSXFlags` value (the block's name index, then the
    /// value), if it has the block.
    pub fn bsx_flags(&self) -> Option<u32> {
        (0..self.blocks().len())
            .find(|&i| self.block_type(i) == "BSXFlags")
            .and_then(|i| self.block_bytes(i).get(4..8))
            .map(|v| u32::from_le_bytes([v[0], v[1], v[2], v[3]]))
    }
}

/// A model's collision.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Collision {
    pub parts: Vec<CollisionPart>,
    /// Block types met in collision that aren't read, by type.
    pub unhandled: BTreeMap<String, usize>,
    /// Bodies left out because their filter carries the "no collision"
    /// flag ([`layers::NO_COLLISION_FLAG`]).
    pub no_collision: usize,
}

impl Collision {
    pub fn triangle_count(&self) -> usize {
        self.parts
            .iter()
            .map(|p| match &p.shape {
                CollisionShape::Triangles { triangles, .. } => triangles.len(),
                _ => 0,
            })
            .sum()
    }
}

/// The rigid body a collision object names.
struct Body {
    shape: i32,
    layer: u8,
    /// The filter's second byte: flags (0x40 no collision) and part.
    flags: u8,
    /// The Havok motion system (7 fixed, 6 keyframed, others move).
    motion: u8,
    /// Offset and rotation of a `bhkRigidBodyT`, in Havok units.
    transform: Option<Transform>,
    /// Its physical values in its own frame, Havok units (moved into the
    /// model's space by [`Nif::collision_object`]).
    info: RigidBodyInfo,
}

/// Havok motion systems as the NIF stores them.
const MOTION_KEYFRAMED: u8 = 6;
const MOTION_FIXED: u8 = 7;

impl Nif {
    /// The model's collision as the game places it: the top node's own
    /// transform is left out, as in [`Nif::placed_scene`].
    pub fn placed_collision(&self) -> Result<Collision> {
        self.build_collision(false)
    }

    /// The model's collision with every node's transform applied, as
    /// [`Nif::scene`] gives the meshes.
    pub fn collision(&self) -> Result<Collision> {
        self.build_collision(true)
    }

    fn build_collision(&self, apply_root: bool) -> Result<Collision> {
        let mut out = Collision::default();
        let mut visited = vec![false; self.blocks().len()];
        for &root in self.roots() {
            self.collision_visit(
                root,
                &Transform::IDENTITY,
                &[],
                0,
                apply_root,
                &mut visited,
                &mut out,
            )?;
        }
        Ok(out)
    }

    fn block_index(&self, reference: i32) -> Option<usize> {
        usize::try_from(reference)
            .ok()
            .filter(|&i| i < self.blocks().len())
    }

    #[allow(clippy::too_many_arguments)]
    fn collision_visit(
        &self,
        reference: i32,
        parent: &Transform,
        chain: &[(String, Transform)],
        depth: usize,
        apply_root: bool,
        visited: &mut [bool],
        out: &mut Collision,
    ) -> Result<()> {
        let Some(index) = self.block_index(reference) else {
            return Ok(());
        };
        if visited[index] || depth > 64 {
            return Ok(());
        }
        visited[index] = true;
        let (av, children) = match self.block(index)? {
            Block::Node(node) => {
                let children = match node.active_child {
                    Some(k) => node.children.get(k).copied().into_iter().collect(),
                    None => node.children,
                };
                (node.av, children)
            }
            Block::Geometry(geometry) => (geometry.av, Vec::new()),
            _ => return Ok(()),
        };
        let local = if depth == 0 && !apply_root {
            Transform::IDENTITY
        } else {
            av.transform
        };
        let world = parent.then_child(&local);
        let mut chain = chain.to_vec();
        chain.push((av.net.name.clone(), local));
        if let Some(object) = self.block_index(av.collision) {
            self.collision_object(object, index, &world, &chain, out)?;
        }
        for child in children {
            self.collision_visit(child, &world, &chain, depth + 1, apply_root, visited, out)?;
        }
        Ok(())
    }

    /// A `bhkCollisionObject`: target node (4), flags (2), body (4).
    /// Phantoms (`bhkSPCollisionObject`, triggers) and ragdoll blends aren't
    /// solid and are left out.
    ///
    /// The body sits where its node is, turned as the node is, but not
    /// scaled by the nodes' own scales: a Havok body can't be scaled, and
    /// the shapes carry any scale themselves. The game's static collections
    /// show it: every piece placed at a scale other than 1 (99 models, all
    /// in `meshes\scol\`) has its node scaled for drawing and its packed
    /// triangles scaled by the same factor (`nvinspect <Data> collision`),
    /// so applying both would shrink them twice. A placed reference's own
    /// scale is different: the game clones the model's bodies scaled by it
    /// (`00c8f2a0` scales the body and marks its filter "scaled";
    /// `00cb30d0` scales a `bhkRigidBodyT`'s offset), which the placement's
    /// transform does here.
    fn collision_object(
        &self,
        index: usize,
        node_index: usize,
        node: &Transform,
        chain: &[(String, Transform)],
        out: &mut Collision,
    ) -> Result<()> {
        let type_name = self.block_type(index);
        if type_name != "bhkCollisionObject" {
            *out.unhandled.entry(type_name.to_string()).or_default() += 1;
            return Ok(());
        }
        let mut r = Reader::new(self.block_bytes(index), self.blocks()[index].offset);
        let _target = r.i32("the collision target")?;
        let flags = r.u16("the collision flags")?;
        let body_ref = r.i32("the rigid body")?;
        let Some(body_index) = self.block_index(body_ref) else {
            return Ok(());
        };
        let Some(body) = self.rigid_body(body_index, out)? else {
            return Ok(());
        };
        if body.flags & layers::NO_COLLISION_FLAG != 0 {
            out.no_collision += 1;
            return Ok(());
        }
        let unscaled = Transform {
            scale: 1.0,
            ..*node
        };
        let scale = Transform {
            scale: HAVOK_SCALE,
            ..Transform::IDENTITY
        };
        let mut to_model = unscaled.then_child(&scale);
        if let Some(t) = &body.transform {
            to_model = to_model.then_child(t);
        }
        // The body's values into the model's space: the centre placed like
        // the shapes, the tensor turned (R I Rᵀ) and scaled to game units².
        let r = to_model.rotation;
        let i = body.info.inertia;
        let mut turned = [[0.0f32; 3]; 3];
        for (a, row) in turned.iter_mut().enumerate() {
            for (b, value) in row.iter_mut().enumerate() {
                let mut sum = 0.0;
                for k in 0..3 {
                    for l in 0..3 {
                        sum += r[a][k] * i[k][l] * r[b][l];
                    }
                }
                *value = sum * HAVOK_SCALE * HAVOK_SCALE;
            }
        }
        let info = RigidBodyInfo {
            center: to_model.apply_point(body.info.center),
            inertia: turned,
            max_linear_speed: body.info.max_linear_speed * HAVOK_SCALE,
            penetration_depth: body.info.penetration_depth * HAVOK_SCALE,
            ..body.info
        };
        let template = CollisionPart {
            layer: body.layer,
            dynamic: !matches!(body.motion, MOTION_KEYFRAMED | MOTION_FIXED),
            keyframed: body.motion == MOTION_KEYFRAMED,
            node: node_index,
            nodes: chain.to_vec(),
            flags,
            shell: 0.0,
            shape: CollisionShape::Sphere {
                center: [0.0; 3],
                radius: 0.0,
            },
            material: 0,
            body: info,
        };
        self.shape(body.shape, &to_model, &template, 0, out)
    }

    /// `bhkRigidBody` / `bhkRigidBodyT` (236 bytes without constraints):
    /// shape (4) at 0, Havok filter (layer, flags and part, group) at 4,
    /// world object info (20), entity info (4), then the rigid body info:
    /// the translation (Vector4) at 52, rotation quaternion (x, y, z, w) at
    /// 68, ... motion system at 212.
    fn rigid_body(&self, index: usize, out: &mut Collision) -> Result<Option<Body>> {
        let type_name = self.block_type(index);
        let with_transform = match type_name {
            "bhkRigidBody" => false,
            "bhkRigidBodyT" => true,
            other => {
                *out.unhandled.entry(other.to_string()).or_default() += 1;
                return Ok(None);
            }
        };
        let bytes = self.block_bytes(index);
        let mut r = Reader::new(bytes, self.blocks()[index].offset);
        let shape = r.i32("the body's shape")?;
        let layer = r.u8("the collision layer")?;
        let flags = r.u8("the collision flags")?;
        let mut r = Reader::new(bytes, self.blocks()[index].offset);
        r.take(52, "the rigid body's leading fields")?;
        let translation = r.vec3("the body's translation")?;
        r.f32("the translation's w")?;
        let q = [
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
        ];
        // Velocities (two Vector4s), then the values at 116.
        r.take(32, "the body's velocities")?;
        let mut inertia = [[0.0f32; 3]; 3];
        for row in &mut inertia {
            *row = r.vec3("the inertia tensor")?;
            r.f32("the inertia tensor")?;
        }
        let center = r.vec3("the centre of mass")?;
        r.f32("the centre's w")?;
        let mass = r.f32("the mass")?;
        let linear_damping = r.f32("the linear damping")?;
        let angular_damping = r.f32("the angular damping")?;
        let friction = r.f32("the friction")?;
        let restitution = r.f32("the restitution")?;
        let max_linear_speed = r.f32("the most linear speed")?;
        let max_angular_speed = r.f32("the most angular speed")?;
        let penetration_depth = r.f32("the penetration depth")?;
        let motion = r.u8("the motion system")?;
        let deactivator = r.u8("the deactivator type")?;
        let solver_deactivation = r.u8("the solver deactivation")?;
        let quality = r.u8("the quality type")?;
        // The number of constraints (joints to other bodies or the world)
        // at 228, before their references.
        let constraints = {
            let mut c = Reader::new(bytes, self.blocks()[index].offset);
            c.take(228, "the rigid body's fields before its constraints")
                .and_then(|_| c.u32("the constraint count"))
                .unwrap_or(0)
        };
        // The block's last word, after the constraint references: the
        // `bhkWorldObject` flags the game reads into `+0x10` (`iFlags`,
        // Xbox PDB) as it loads the body.
        // Translated from 00c8ea30 (decompiled, FalloutNV.exe 1.4.0.525)
        let body_flags = {
            let mut c = Reader::new(bytes, self.blocks()[index].offset);
            let at = 232 + 4 * constraints as usize;
            c.take(at, "the rigid body's fields before its flags")
                .and_then(|_| c.u32("the body flags"))
                .unwrap_or(0)
        };
        let info = RigidBodyInfo {
            block: index,
            constraints,
            body_flags,
            mass,
            center,
            inertia,
            linear_damping,
            angular_damping,
            friction,
            restitution,
            max_linear_speed,
            max_angular_speed,
            penetration_depth,
            motion,
            deactivator,
            solver_deactivation,
            quality,
        };
        Ok(Some(Body {
            shape,
            layer,
            flags,
            motion,
            info,
            transform: with_transform.then(|| Transform {
                rotation: quaternion(q),
                translation,
                scale: 1.0,
            }),
        }))
    }

    fn shape(
        &self,
        reference: i32,
        to_model: &Transform,
        template: &CollisionPart,
        depth: usize,
        out: &mut Collision,
    ) -> Result<()> {
        let Some(index) = self.block_index(reference) else {
            return Ok(());
        };
        if depth > MAX_SHAPE_DEPTH {
            return Ok(());
        }
        let bytes = self.block_bytes(index);
        let mut r = Reader::new(bytes, self.blocks()[index].offset);
        // Shapes with a convex radius keep it at 4 (after the material);
        // the packed triangles at 8.
        let radius_at = |at: usize| {
            bytes
                .get(at..at + 4)
                .map_or(0.0, |b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .max(0.0)
                * HAVOK_SCALE
        };
        let push = |out: &mut Collision, shape: CollisionShape, shell: f32, material: u32| {
            out.parts.push(CollisionPart {
                shape,
                shell,
                material,
                ..template.clone()
            });
        };
        match self.block_type(index) {
            // Child shape (4), then the MOPP search tree, not needed here
            // (its "shape scale" at 16 is 1 in every game file).
            "bhkMoppBvTreeShape" => {
                let child = r.i32("the tree's shape")?;
                self.shape(child, to_model, template, depth + 1, out)?;
            }
            // User data, unused, radius, unused, scale (Vector4), radius
            // copy, scale copy, then the data (4) at 52.
            "bhkPackedNiTriStripsShape" => {
                r.take(16, "the packed shape's leading fields")?;
                let s = r.vec3("the packed shape's scale")?;
                r.take(52 - 28, "the packed shape's copies")?;
                let data = r.i32("the packed data")?;
                let scaled = Transform {
                    // Uniform in every game file; the first component is
                    // used.
                    scale: if s[0] > 0.0 { s[0] } else { 1.0 },
                    ..Transform::IDENTITY
                };
                if let Some(data_index) = self.block_index(data) {
                    let template = CollisionPart {
                        shell: radius_at(8),
                        ..template.clone()
                    };
                    self.packed_triangles(
                        data_index,
                        &to_model.then_child(&scaled),
                        &template,
                        out,
                    )?;
                }
            }
            // Material (4), radius (4), unused (20), grow-by (4), scale
            // (Vector4), the `NiTriStripsData` blocks (counted references),
            // then one filter per data block (counted, 4 bytes each: layer,
            // flags, group). Checked on the game's one such model
            // (`traps\terminaldesktrap01.nif`, 72 bytes: two data blocks,
            // two filters on the static layer).
            "bhkNiTriStripsShape" => {
                r.take(32, "the strips shape's leading fields")?;
                let s = r.vec3("the strips shape's scale")?;
                r.f32("the scale's w")?;
                let data = r.ref_list("the strips data")?;
                let n = r.u32("the strips filters")? as usize;
                let filters = r.counted(n, 4, "the strips filters", |r| {
                    Ok([
                        r.u8("a filter")?,
                        r.u8("a filter")?,
                        r.u8("a filter")?,
                        r.u8("a filter")?,
                    ])
                })?;
                let scaled = to_model.then_child(&Transform {
                    scale: if s[0] > 0.0 { s[0] } else { 1.0 },
                    ..Transform::IDENTITY
                });
                for (k, &data_ref) in data.iter().enumerate() {
                    let filter = filters.get(k).copied().unwrap_or([template.layer, 0, 0, 0]);
                    if filter[1] & layers::NO_COLLISION_FLAG != 0 {
                        out.no_collision += 1;
                        continue;
                    }
                    let Some(data_index) = self.block_index(data_ref) else {
                        continue;
                    };
                    let Block::GeometryData(g) = self.block(data_index)? else {
                        *out.unhandled
                            .entry(self.block_type(data_index).to_string())
                            .or_default() += 1;
                        continue;
                    };
                    out.parts.push(CollisionPart {
                        layer: filter[0],
                        shell: radius_at(4),
                        shape: CollisionShape::Triangles {
                            vertices: g.positions.iter().map(|&v| scaled.apply_point(v)).collect(),
                            triangles: g.triangles.iter().map(|t| t.map(u32::from)).collect(),
                        },
                        ..template.clone()
                    });
                }
            }
            // Material (4), radius, unused (8), half extents (Vector3),
            // unused (4).
            "bhkBoxShape" => {
                let material = r.u32("the box's material")?;
                r.take(12, "the box's leading fields")?;
                let h = r.vec3("the box's half size")?;
                let corners: Vec<Vec3> = (0..8)
                    .map(|k| {
                        [
                            if k & 1 == 0 { -h[0] } else { h[0] },
                            if k & 2 == 0 { -h[1] } else { h[1] },
                            if k & 4 == 0 { -h[2] } else { h[2] },
                        ]
                    })
                    .collect();
                push(
                    out,
                    convex(&corners, &box_planes(h), to_model),
                    radius_at(4),
                    material,
                );
            }
            // Material (4), radius (4).
            "bhkSphereShape" => {
                let material = r.u32("the sphere's material")?;
                let radius = r.f32("the sphere's radius")?;
                push(
                    out,
                    CollisionShape::Sphere {
                        center: to_model.apply_point([0.0; 3]),
                        radius: radius * to_model.scale,
                    },
                    0.0,
                    material,
                );
            }
            // Material, radius, unused (8), first point, its radius,
            // second point, its radius.
            "bhkCapsuleShape" => {
                let material = r.u32("the capsule's material")?;
                let radius = r.f32("the capsule's radius")?;
                r.take(8, "unused")?;
                let a = r.vec3("the capsule's first point")?;
                r.f32("the first point's radius")?;
                let b = r.vec3("the capsule's second point")?;
                push(
                    out,
                    CollisionShape::Capsule {
                        a: to_model.apply_point(a),
                        b: to_model.apply_point(b),
                        radius: radius * to_model.scale,
                    },
                    0.0,
                    material,
                );
            }
            // Material, radius, two (12-byte) properties, then the
            // vertices (Vector4) and the face planes (Vector4: normal and
            // distance), each counted.
            "bhkConvexVerticesShape" => {
                let material = r.u32("the hull's material")?;
                r.take(28, "the hull's leading fields")?;
                let n = r.u32("the hull's vertex count")? as usize;
                let vertices = r.counted(n, 16, "the hull's vertices", |r| {
                    let v = r.vec3("a hull vertex")?;
                    r.f32("its w")?;
                    Ok(v)
                })?;
                let n = r.u32("the hull's plane count")? as usize;
                let planes = r.counted(n, 16, "the hull's planes", |r| {
                    Ok([
                        r.f32("a plane")?,
                        r.f32("a plane")?,
                        r.f32("a plane")?,
                        r.f32("a plane")?,
                    ])
                })?;
                push(
                    out,
                    convex(&vertices, &planes, to_model),
                    radius_at(4),
                    material,
                );
            }
            // Child shape (4), material, radius, unused (8), then a 4x4
            // matrix as Havok keeps it: three rotation columns and the
            // translation, each four floats.
            "bhkConvexTransformShape" | "bhkTransformShape" => {
                let child = r.i32("the transformed shape")?;
                r.take(16, "the transform shape's fields")?;
                let mut columns = [[0.0f32; 4]; 4];
                for column in &mut columns {
                    for v in column.iter_mut() {
                        *v = r.f32("the transform")?;
                    }
                }
                let rotation: Mat3 = [
                    [columns[0][0], columns[1][0], columns[2][0]],
                    [columns[0][1], columns[1][1], columns[2][1]],
                    [columns[0][2], columns[1][2], columns[2][2]],
                ];
                let local = Transform {
                    rotation,
                    translation: [columns[3][0], columns[3][1], columns[3][2]],
                    scale: 1.0,
                };
                self.shape(
                    child,
                    &to_model.then_child(&local),
                    template,
                    depth + 1,
                    out,
                )?;
            }
            // Counted child shapes, then material and filter fields.
            "bhkListShape" => {
                let children = r.ref_list("the list's shapes")?;
                for child in children {
                    self.shape(child, to_model, template, depth + 1, out)?;
                }
            }
            other => {
                *out.unhandled.entry(other.to_string()).or_default() += 1;
            }
        }
        Ok(())
    }

    /// `hkPackedNiTriStripsData`: triangle count, triangles (three vertex
    /// indices and welding info, 8 bytes each), vertex count, a
    /// "compressed" byte, the vertices (three floats each), then sub-parts
    /// (filter, vertex count, material: 12 bytes each) that split the
    /// vertices into runs on their own layers.
    fn packed_triangles(
        &self,
        index: usize,
        to_model: &Transform,
        template: &CollisionPart,
        out: &mut Collision,
    ) -> Result<()> {
        if self.block_type(index) != "hkPackedNiTriStripsData" {
            *out.unhandled
                .entry(self.block_type(index).to_string())
                .or_default() += 1;
            return Ok(());
        }
        let mut r = Reader::new(self.block_bytes(index), self.blocks()[index].offset);
        let n = r.u32("the triangle count")? as usize;
        let triangles = r.counted(n, 8, "the triangles", |r| {
            let t = [
                u32::from(r.u16("a triangle")?),
                u32::from(r.u16("a triangle")?),
                u32::from(r.u16("a triangle")?),
            ];
            r.u16("welding info")?;
            Ok(t)
        })?;
        let n = r.u32("the vertex count")? as usize;
        let compressed = r.u8("the compressed flag")? != 0;
        let vertices = if compressed {
            r.counted(n, 6, "the vertices", |r| {
                Ok([
                    half(r.u16("a vertex")?),
                    half(r.u16("a vertex")?),
                    half(r.u16("a vertex")?),
                ])
            })?
        } else {
            r.counted(n, 12, "the vertices", |r| r.vec3("a vertex"))?
        };
        let parts = r.u16("the sub-part count")? as usize;
        let runs = r.counted(parts, 12, "the sub-parts", |r| {
            let layer = r.u8("a sub-part's layer")?;
            let flags = r.u8("a sub-part's flags")?;
            r.take(2, "the rest of its filter")?;
            let count = r.u32("a sub-part's vertex count")?;
            let material = r.u32("a sub-part's material")?;
            Ok((layer, flags, count, material))
        })?;
        let vertices: Vec<Vec3> = vertices.iter().map(|&v| to_model.apply_point(v)).collect();
        let valid = |t: &[u32; 3]| t.iter().all(|&i| (i as usize) < vertices.len());
        if runs.is_empty() {
            out.parts.push(CollisionPart {
                shape: CollisionShape::Triangles {
                    triangles: triangles.into_iter().filter(valid).collect(),
                    vertices,
                },
                ..template.clone()
            });
            return Ok(());
        }
        // Each sub-part owns a run of vertices; a triangle belongs to the
        // run its first vertex is in.
        let mut start = 0u32;
        for (layer, flags, count, material) in runs {
            let end = start.saturating_add(count);
            let mine: Vec<[u32; 3]> = triangles
                .iter()
                .filter(|t| t[0] >= start && t[0] < end && valid(t))
                .copied()
                .collect();
            start = end;
            if mine.is_empty() {
                continue;
            }
            if flags & layers::NO_COLLISION_FLAG != 0 {
                out.no_collision += 1;
                continue;
            }
            out.parts.push(CollisionPart {
                layer,
                material,
                shape: CollisionShape::Triangles {
                    vertices: vertices.clone(),
                    triangles: mine,
                },
                ..template.clone()
            });
        }
        Ok(())
    }
}

/// A box's six faces (half sizes `h`), as planes pointing out.
fn box_planes(h: Vec3) -> Vec<[f32; 4]> {
    vec![
        [1.0, 0.0, 0.0, -h[0]],
        [-1.0, 0.0, 0.0, -h[0]],
        [0.0, 1.0, 0.0, -h[1]],
        [0.0, -1.0, 0.0, -h[1]],
        [0.0, 0.0, 1.0, -h[2]],
        [0.0, 0.0, -1.0, -h[2]],
    ]
}

/// A box in game units (half sizes `half`), centred on `to_world`'s
/// origin and turned with it: the shape the game builds for a placed
/// collision primitive (`world::Primitive`).
pub fn box_shape(half: Vec3, to_world: &Transform) -> CollisionShape {
    let corners: Vec<Vec3> = (0..8)
        .map(|k| {
            [
                if k & 1 == 0 { -half[0] } else { half[0] },
                if k & 2 == 0 { -half[1] } else { half[1] },
                if k & 4 == 0 { -half[2] } else { half[2] },
            ]
        })
        .collect();
    convex(&corners, &box_planes(half), to_world)
}

/// A convex hull moved into the model's space: corners transformed, each
/// plane `n·p + d = 0` turned and moved with them.
fn convex(vertices: &[Vec3], planes: &[[f32; 4]], to_model: &Transform) -> CollisionShape {
    let vertices = vertices.iter().map(|&v| to_model.apply_point(v)).collect();
    let planes = planes
        .iter()
        .map(|p| {
            let n = mat_vec(&to_model.rotation, [p[0], p[1], p[2]]);
            let t = to_model.translation;
            let d = p[3] * to_model.scale - (n[0] * t[0] + n[1] * t[1] + n[2] * t[2]);
            [n[0], n[1], n[2], d]
        })
        .collect();
    CollisionShape::Convex { vertices, planes }
}

/// A unit quaternion (x, y, z, w) as a row-major rotation matrix.
fn quaternion(q: [f32; 4]) -> Mat3 {
    let [x, y, z, w] = q;
    let len = (x * x + y * y + z * z + w * w).sqrt();
    if len < 1e-6 {
        return crate::math::IDENTITY3;
    }
    let (x, y, z, w) = (x / len, y / len, z / len, w / len);
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

/// A 16-bit float, as compressed vertices store them.
fn half(bits: u16) -> f32 {
    let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exponent = i32::from((bits >> 10) & 0x1f);
    let mantissa = f32::from(bits & 0x3ff);
    sign * match exponent {
        0 => mantissa * 2f32.powi(-24),
        31 => f32::INFINITY,
        e => (1.0 + mantissa / 1024.0) * 2f32.powi(e - 15),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_half_floats() {
        assert_eq!(half(0x3c00), 1.0);
        assert_eq!(half(0xc000), -2.0);
        assert_eq!(half(0x3800), 0.5);
        assert_eq!(half(0), 0.0);
    }

    #[test]
    fn turns_quaternions_into_rotations() {
        // 90° about z.
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = quaternion([0.0, 0.0, s, s]);
        let v = mat_vec(&m, [1.0, 0.0, 0.0]);
        assert!((v[0]).abs() < 1e-6 && (v[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn moves_hull_planes_with_the_hull() {
        // A plane x = 1 (n = +x, d = -1), moved by +10 in x and scaled 7.
        let t = Transform {
            translation: [10.0, 0.0, 0.0],
            scale: 7.0,
            ..Transform::IDENTITY
        };
        let CollisionShape::Convex { vertices, planes } =
            convex(&[[1.0, 0.0, 0.0]], &[[1.0, 0.0, 0.0, -1.0]], &t)
        else {
            unreachable!()
        };
        // The vertex lands at x = 17, on the moved plane.
        let p = vertices[0];
        let [nx, ny, nz, d] = planes[0];
        assert!((p[0] - 17.0).abs() < 1e-5);
        assert!((nx * p[0] + ny * p[1] + nz * p[2] + d).abs() < 1e-5);
    }

    #[test]
    fn the_character_runs_into_the_game_layers_only() {
        use layers::*;
        // Solid for people: the world, clutter, doors, other people.
        for layer in [
            UNIDENTIFIED,
            STATIC,
            ANIM_STATIC,
            TRANSPARENT,
            CLUTTER,
            WEAPON,
            TREES,
            PROPS,
            TERRAIN,
            GROUND,
            DEBRIS_LARGE,
            TRANSPARENT_SMALL,
            INVISIBLE_WALL,
            TRANSPARENT_SMALL_ANIM,
            CHAR_CONTROLLER,
        ] {
            assert!(blocks_walking(layer), "{}", name(layer));
        }
        // Not: living people's bones, the dead, non-collidable, small debris.
        for layer in [
            BIPED,
            DEAD_BIPED,
            NON_COLLIDABLE,
            DEBRIS_SMALL,
            18,
            25,
            33,
            34,
            37,
            38,
        ] {
            assert!(!blocks_walking(layer), "{}", name(layer));
        }
        assert!(!blocks_walking(43) && !blocks_walking(200));
        assert_eq!(name(30), "CHARCONTROLLER");
        assert_eq!(name(99), "layer 99");
    }
}
