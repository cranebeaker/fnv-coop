//! Ragdolls: the physics bodies a skeleton carries for its bones and the
//! joints between them, which the game hands to Havok when someone dies.
//!
//! In `skeleton.nif` 18 bones (pelvis, spine, neck, head, the limbs) have a
//! collision object (`bhkBlendCollisionObject`: target, flags, body, two
//! gains) naming a `bhkRigidBody` with a `bhkCapsuleShape`. The body's
//! translation and rotation are its frame in the skeleton's space (Havok
//! units, seven game units each); a `bhkRigidBodyT`'s (the dog's pelvis
//! and spine, with spheres) place it relative to its bone instead (the
//! joints' pivots meet only so); its mass, centre of mass, inertia,
//! damping, friction and restitution follow (offsets as in
//! [`Nif::collision`]'s notes: inertia at 116 as three rows of four floats,
//! centre at 164, mass at 180, linear and angular damping, friction,
//! restitution, the most linear and angular speed, at 184–204), then its
//! constraints (count at 228, then references).
//!
//! A constraint: the body count (2), the two bodies (A the child, B the
//! parent), a priority, then the joint's own data, every vector four floats
//! in its body's frame:
//! - `bhkRagdollConstraint` (169 bytes): twist, plane, motor and pivot axes
//!   for A, then for B; the cone's widest angle, the plane's least and
//!   most, the twist's least and most, the most friction, a motor type.
//!   The spine's: a cone of 18°, plane and twist ±10°.
//! - `bhkLimitedHingeConstraint`: axle, two perpendicular axes and pivot for
//!   A, then for B; the least and most angle, the most friction, a motor
//!   type. A knee's: −50° to 87°.
//! - `bhkMalleableConstraint` wraps one of those: its type (2 limited
//!   hinge, 7 ragdoll) after the common part, the wrapped constraint's own
//!   common part, its data, then a strength (`tau`, 0.9).
//!
//! Layouts read byte for byte from the game's `characters\_male\
//! skeleton.nif` (every field accounted for in each block's size).

use crate::blocks::Block;
use crate::collision::HAVOK_SCALE;
use crate::error::Result;
use crate::file::Nif;
use crate::math::{Mat3, Transform, Vec3};
use crate::reader::Reader;

/// One body of a ragdoll.
#[derive(Debug, Clone, PartialEq)]
pub struct RagdollBody {
    /// The bone it moves: its index in [`Nif::skeleton`] and its name.
    pub bone: usize,
    pub bone_name: String,
    /// The body's frame in the skeleton's space, in game units, as the
    /// file's own pose has it.
    pub frame: Transform,
    pub mass: f32,
    /// The centre of mass, in the body's frame (game units).
    pub center: Vec3,
    /// The inertia tensor's diagonal, in the body's frame (mass × game
    /// units²).
    pub inertia: Vec3,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub friction: f32,
    pub restitution: f32,
    /// The fastest it moves (game units a second) and turns (radians a
    /// second).
    pub max_linear_speed: f32,
    pub max_angular_speed: f32,
    /// Its capsule in the body's frame: the two ends and the radius, game
    /// units (a sphere has both ends at its centre).
    pub capsule: Option<(Vec3, Vec3, f32)>,
    /// The Havok layer (8 for people's bodies).
    pub layer: u8,
    /// The body part number (the collision filter's next byte, low five
    /// bits): 1 head, 2 body, 3–4 spine, 5–7 left arm, 8–10 left leg, 11–13
    /// right arm, 14–16 right leg, as the game's death push reads it.
    pub part: u8,
}

/// How far a joint lets its bodies turn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JointLimit {
    /// A ragdoll joint (hips, spine, neck, shoulders): each body's twist
    /// and plane axis in its own frame (A's, then B's), and the angle
    /// limits in radians.
    Ragdoll {
        twist: [Vec3; 2],
        plane: [Vec3; 2],
        cone: f32,
        plane_range: (f32, f32),
        twist_range: (f32, f32),
    },
    /// A hinge (knees, elbows): each body's axle and first perpendicular
    /// axis, and the angle range.
    Hinge {
        axle: [Vec3; 2],
        perpendicular: [Vec3; 2],
        range: (f32, f32),
    },
    /// Turns freely (a ball and socket).
    Free,
}

/// A joint between two bodies (indices into [`Ragdoll::bodies`]: the child,
/// then the parent), at a point given in each body's frame (game units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RagdollJoint {
    pub bodies: [usize; 2],
    pub pivots: [Vec3; 2],
    pub limit: JointLimit,
    /// Each body's third axis of the joint's frame (the ragdoll's motor
    /// axis, the hinge's second perpendicular axis), in the body's frame;
    /// zero for a ball and socket. With the twist (or axle) and plane (or
    /// perpendicular) axes the frame `hkpSetLocalTransformsConstraintAtom`
    /// gives the solver.
    pub third: [Vec3; 2],
    /// The most friction torque (`hkpAngFrictionConstraintAtom`'s, set by
    /// `00cde9f0` from the file's value); 0 for a ball and socket.
    pub max_friction: f32,
}

/// A skeleton's ragdoll.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ragdoll {
    pub bodies: Vec<RagdollBody>,
    pub joints: Vec<RagdollJoint>,
}

fn scaled(v: Vec3) -> Vec3 {
    v.map(|x| x * HAVOK_SCALE)
}

/// Four floats, the first three kept.
fn vec4(r: &mut Reader, what: &str) -> Result<Vec3> {
    let v = r.vec3(what)?;
    r.f32(what)?;
    Ok(v)
}

impl Nif {
    /// The skeleton's ragdoll, if its bones carry bodies.
    pub fn ragdoll(&self) -> Result<Option<Ragdoll>> {
        let bones = self.skeleton()?;
        // Each bone in the skeleton's space, in the file's pose (for
        // `bhkRigidBodyT` bodies, which sit relative to their bone).
        let mut pose: Vec<Transform> = Vec::with_capacity(bones.len());
        for b in &bones {
            let w = match b.parent.and_then(|p| pose.get(p)) {
                Some(parent) => parent.then_child(&b.local),
                None => b.local,
            };
            pose.push(w);
        }
        let mut ragdoll = Ragdoll::default();
        // Rigid body block → body index.
        let mut body_of = std::collections::HashMap::new();
        let mut constraint_blocks = Vec::new();
        for index in 0..self.blocks().len() {
            let Ok(Block::Node(node)) = self.block(index) else {
                continue;
            };
            let Some(object) = self.reference(node.av.collision) else {
                continue;
            };
            if !matches!(
                self.block_type(object),
                "bhkBlendCollisionObject" | "bhkCollisionObject"
            ) {
                continue;
            }
            let mut r = self.reader(object);
            r.i32("the collision target")?;
            r.u16("the collision flags")?;
            let Some(body_block) = self.reference(r.i32("the rigid body")?) else {
                continue;
            };
            let relative = match self.block_type(body_block) {
                "bhkRigidBody" => false,
                "bhkRigidBodyT" => true,
                _ => continue,
            };
            let Some(bone) = bones.iter().position(|b| b.name == node.av.net.name) else {
                continue;
            };
            let (mut body, constraints) = self.ragdoll_body(body_block, bone, &bones[bone].name)?;
            // A `bhkRigidBodyT`'s translation and rotation place it relative
            // to its bone (the game scales that offset with the body,
            // `00cb30d0`); a plain body's are its frame in the skeleton's
            // space. The dog's pelvis and spine bodies are the first kind.
            if relative {
                body.frame = pose[bone].then_child(&Transform {
                    scale: 1.0,
                    ..body.frame
                });
                body.frame.scale = 1.0;
            }
            body_of.insert(body_block, ragdoll.bodies.len());
            ragdoll.bodies.push(body);
            constraint_blocks.extend(constraints);
        }
        if ragdoll.bodies.is_empty() {
            return Ok(None);
        }
        constraint_blocks.sort_unstable();
        constraint_blocks.dedup();
        for index in constraint_blocks {
            if let Some(joint) = self.ragdoll_joint(index, &body_of)? {
                ragdoll.joints.push(joint);
            }
        }
        Ok(Some(ragdoll))
    }

    /// A `bhkRigidBody` as a ragdoll body, and the constraint blocks it
    /// lists.
    fn ragdoll_body(
        &self,
        index: usize,
        bone: usize,
        name: &str,
    ) -> Result<(RagdollBody, Vec<usize>)> {
        let mut r = self.reader(index);
        let shape = r.i32("the body's shape")?;
        let layer = r.u8("the collision layer")?;
        let part = r.u8("the collision flags and part number")? & 0x1F;
        let mut r = self.reader(index);
        r.take(52, "the rigid body's leading fields")?;
        let translation = scaled(vec4(&mut r, "the body's translation")?);
        let q = [
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
            r.f32("the body's rotation")?,
        ];
        r.take(32, "the body's velocities")?;
        // The tensor's rows; their diagonal entries (the bodies' tensors
        // are diagonal in their frames).
        let mut inertia = [0.0; 3];
        for (k, value) in inertia.iter_mut().enumerate() {
            let row = vec4(&mut r, "the inertia tensor")?;
            *value = row[k] * HAVOK_SCALE * HAVOK_SCALE;
        }
        let center = scaled(vec4(&mut r, "the centre of mass")?);
        let mass = r.f32("the mass")?;
        let linear_damping = r.f32("the linear damping")?;
        let angular_damping = r.f32("the angular damping")?;
        let friction = r.f32("the friction")?;
        let restitution = r.f32("the restitution")?;
        let max_linear_speed = r.f32("the most linear speed")? * HAVOK_SCALE;
        let max_angular_speed = r.f32("the most angular speed")?;
        r.take(228 - 208, "the body's other values")?;
        let constraints = r.ref_list("the body's constraints")?;
        let body = RagdollBody {
            bone,
            bone_name: name.to_string(),
            frame: Transform {
                rotation: quaternion(q),
                translation,
                scale: 1.0,
            },
            mass,
            center,
            inertia,
            linear_damping,
            angular_damping,
            friction,
            restitution,
            max_linear_speed,
            max_angular_speed,
            capsule: self.ragdoll_capsule(shape)?,
            layer,
            part,
        };
        let constraints = constraints
            .into_iter()
            .filter_map(|c| self.reference(c))
            .collect();
        Ok((body, constraints))
    }

    /// A capsule (or sphere) shape in game units.
    fn ragdoll_capsule(&self, reference: i32) -> Result<Option<(Vec3, Vec3, f32)>> {
        let Some(index) = self.reference(reference) else {
            return Ok(None);
        };
        let mut r = self.reader(index);
        Ok(match self.block_type(index) {
            "bhkCapsuleShape" => {
                r.take(4, "the capsule's material")?;
                let radius = r.f32("the capsule's radius")?;
                r.take(8, "unused")?;
                let a = scaled(vec4(&mut r, "the capsule's first point")?);
                let b = scaled(r.vec3("the capsule's second point")?);
                Some((a, b, radius * HAVOK_SCALE))
            }
            "bhkSphereShape" => {
                r.take(4, "the sphere's material")?;
                let radius = r.f32("the sphere's radius")?;
                Some(([0.0; 3], [0.0; 3], radius * HAVOK_SCALE))
            }
            _ => None,
        })
    }

    fn ragdoll_joint(
        &self,
        index: usize,
        body_of: &std::collections::HashMap<usize, usize>,
    ) -> Result<Option<RagdollJoint>> {
        let mut r = self.reader(index);
        let kind = self.block_type(index);
        let (a, b) = {
            r.u32("the constraint's body count")?;
            let a = r.i32("the constraint's first body")?;
            let b = r.i32("the constraint's second body")?;
            r.u32("the constraint's priority")?;
            (a, b)
        };
        let body = |reference: i32| {
            self.reference(reference)
                .and_then(|i| body_of.get(&i).copied())
        };
        let (Some(a), Some(b)) = (body(a), body(b)) else {
            return Ok(None);
        };
        // A malleable constraint names the kind it wraps, then repeats the
        // common part.
        let wrapped = match kind {
            "bhkMalleableConstraint" => {
                let inner = r.u32("the wrapped constraint's type")?;
                r.take(16, "the wrapped constraint's common part")?;
                match inner {
                    2 => "bhkLimitedHingeConstraint",
                    7 => "bhkRagdollConstraint",
                    1 => "bhkBallAndSocketConstraint",
                    _ => return Ok(None),
                }
            }
            other => other,
        };
        let (pivots, limit, third, max_friction) = match wrapped {
            "bhkRagdollConstraint" => {
                type Side = (Vec3, Vec3, Vec3, Vec3);
                let mut side = || -> Result<Side> {
                    let twist = vec4(&mut r, "a twist axis")?;
                    let plane = vec4(&mut r, "a plane axis")?;
                    let motor = vec4(&mut r, "a motor axis")?;
                    let pivot = scaled(vec4(&mut r, "a pivot")?);
                    Ok((twist, plane, motor, pivot))
                };
                let (twist_a, plane_a, motor_a, pivot_a) = side()?;
                let (twist_b, plane_b, motor_b, pivot_b) = side()?;
                let cone = r.f32("the cone's angle")?;
                let plane_range = (r.f32("the plane's least")?, r.f32("the plane's most")?);
                let twist_range = (r.f32("the twist's least")?, r.f32("the twist's most")?);
                let max_friction = r.f32("the most friction")?;
                (
                    [pivot_a, pivot_b],
                    JointLimit::Ragdoll {
                        twist: [twist_a, twist_b],
                        plane: [plane_a, plane_b],
                        cone,
                        plane_range,
                        twist_range,
                    },
                    [motor_a, motor_b],
                    max_friction,
                )
            }
            "bhkLimitedHingeConstraint" => {
                type Side = (Vec3, Vec3, Vec3, Vec3);
                let mut side = || -> Result<Side> {
                    let axle = vec4(&mut r, "an axle")?;
                    let perpendicular = vec4(&mut r, "a perpendicular axis")?;
                    let second = vec4(&mut r, "a second perpendicular axis")?;
                    let pivot = scaled(vec4(&mut r, "a pivot")?);
                    Ok((axle, perpendicular, second, pivot))
                };
                let (axle_a, perp_a, second_a, pivot_a) = side()?;
                let (axle_b, perp_b, second_b, pivot_b) = side()?;
                let range = (r.f32("the least angle")?, r.f32("the most angle")?);
                let max_friction = r.f32("the most friction")?;
                (
                    [pivot_a, pivot_b],
                    JointLimit::Hinge {
                        axle: [axle_a, axle_b],
                        perpendicular: [perp_a, perp_b],
                        range,
                    },
                    [second_a, second_b],
                    max_friction,
                )
            }
            "bhkBallAndSocketConstraint" => {
                let pivot_a = scaled(vec4(&mut r, "a pivot")?);
                let pivot_b = scaled(vec4(&mut r, "a pivot")?);
                ([pivot_a, pivot_b], JointLimit::Free, [[0.0; 3]; 2], 0.0)
            }
            _ => return Ok(None),
        };
        Ok(Some(RagdollJoint {
            bodies: [a, b],
            pivots,
            limit,
            third,
            max_friction,
        }))
    }
}

/// A unit quaternion (x, y, z, w), as Havok stores it, as a rotation.
fn quaternion(q: [f32; 4]) -> Mat3 {
    let [x, y, z, w] = q;
    crate::anim::quat_matrix([w, x, y, z])
}
