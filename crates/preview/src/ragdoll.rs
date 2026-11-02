//! The dead as ragdolls: a skeleton's bodies and joints (`nif::ragdoll`)
//! set moving by `physics::ragdoll` from the pose the actor died in, and
//! the bones read back from the bodies.
//!
//! Each body keeps the place it has on its bone in the skeleton file's own
//! pose (the body's frame relative to the bone's), so a body's frame is its
//! bone's times that offset, and the other way round. Bones without a body
//! (fingers, the weapon node) keep the turn they had relative to their
//! parent when the actor died.

use nif::math::{mat_vec, Mat3, Transform, Vec3};
use nif::{Bone, JointLimit, Ragdoll};
use physics::ragdoll::{self as sim, BodySetup, JointSetup, Limit};

/// A skeleton's ragdoll, ready to start.
#[derive(Debug, Clone, PartialEq)]
pub struct RagdollRig {
    pub ragdoll: Ragdoll,
    /// Each body's frame relative to its bone's, in the file's pose.
    pub offsets: Vec<Transform>,
}

/// The inverse of a transform.
pub fn inverse(t: &Transform) -> Transform {
    let r = transpose(&t.rotation);
    let s = if t.scale != 0.0 { 1.0 / t.scale } else { 1.0 };
    let moved = mat_vec(&r, t.translation);
    Transform {
        rotation: r,
        translation: [-moved[0] * s, -moved[1] * s, -moved[2] * s],
        scale: s,
    }
}

fn transpose(m: &Mat3) -> Mat3 {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

/// Every bone's transform in the skeleton's space, in the file's own pose.
pub fn file_pose(bones: &[Bone]) -> Vec<Transform> {
    let mut world: Vec<Transform> = Vec::with_capacity(bones.len());
    for b in bones {
        let w = match b.parent.and_then(|p| world.get(p)) {
            Some(parent) => parent.then_child(&b.local),
            None => b.local,
        };
        world.push(w);
    }
    world
}

impl RagdollRig {
    pub fn new(bones: &[Bone], ragdoll: Ragdoll) -> Self {
        let pose = file_pose(bones);
        let offsets = ragdoll
            .bodies
            .iter()
            .map(|b| inverse(&pose[b.bone]).then_child(&b.frame))
            .collect();
        RagdollRig { ragdoll, offsets }
    }

    /// The bodies and joints for the simulation, at `scale` (the actor's
    /// size).
    pub fn setups(&self, scale: f32) -> (Vec<BodySetup>, Vec<JointSetup>) {
        let s = scale;
        let sv = |v: Vec3| v.map(|x| x * s);
        let bodies = self
            .ragdoll
            .bodies
            .iter()
            .map(|b| BodySetup {
                mass: b.mass,
                center: sv(b.center),
                inertia: b.inertia.map(|i| i * s * s),
                capsule: b.capsule.map(|(a, c, r)| (sv(a), sv(c), r * s)),
                friction: b.friction,
                restitution: b.restitution,
                linear_damping: b.linear_damping,
                angular_damping: b.angular_damping,
                max_linear_speed: b.max_linear_speed * s,
                max_angular_speed: b.max_angular_speed,
                layer: b.layer,
                part: b.part,
            })
            .collect();
        let joints = self
            .ragdoll
            .joints
            .iter()
            .map(|j| JointSetup {
                bodies: j.bodies,
                pivots: [sv(j.pivots[0]), sv(j.pivots[1])],
                limit: limit(&j.limit),
                third: j.third,
                max_friction: j.max_friction,
            })
            .collect();
        (bodies, joints)
    }

    /// A ragdoll starting from `pose` (each bone in the skeleton's space),
    /// with the skeleton placed in the world by `placement`.
    pub fn start(&self, pose: &[Transform], placement: &Transform) -> sim::Ragdoll {
        let (bodies, joints) = self.setups(placement.scale);
        let frames: Vec<(Mat3, Vec3)> = self
            .ragdoll
            .bodies
            .iter()
            .zip(&self.offsets)
            .map(|(b, offset)| {
                let world = placement.then_child(&pose[b.bone]).then_child(offset);
                (world.rotation, world.translation)
            })
            .collect();
        sim::Ragdoll::new(bodies, joints, &frames)
    }

    /// The bones in the skeleton's space as the ragdoll has them now:
    /// bones with a body from it, the rest following their parents as they
    /// were in `died` (the pose at death).
    pub fn bones(
        &self,
        ragdoll: &sim::Ragdoll,
        bones: &[Bone],
        died: &[Transform],
        placement: &Transform,
    ) -> Vec<Transform> {
        let to_skeleton = inverse(placement);
        let mut from_body: Vec<Option<Transform>> = vec![None; bones.len()];
        for (i, (b, offset)) in self.ragdoll.bodies.iter().zip(&self.offsets).enumerate() {
            let (rotation, translation) = ragdoll.frame(i);
            let frame = Transform {
                rotation,
                translation,
                scale: placement.scale,
            };
            let world = frame.then_child(&inverse(offset));
            let mut bone = to_skeleton.then_child(&world);
            bone.scale = died.get(b.bone).map_or(1.0, |t| t.scale);
            from_body[b.bone] = Some(bone);
        }
        let mut out: Vec<Transform> = Vec::with_capacity(bones.len());
        for (i, b) in bones.iter().enumerate() {
            let t = match (from_body[i], b.parent) {
                (Some(t), _) => t,
                (None, Some(p)) if p < out.len() => {
                    // Its place under its parent at death, kept.
                    let local = inverse(&died[p]).then_child(&died[i]);
                    out[p].then_child(&local)
                }
                _ => died.get(i).copied().unwrap_or(b.local),
            };
            out.push(t);
        }
        out
    }
}

impl RagdollRig {
    /// Where a ray (`direction` of length 1, game space) first meets the
    /// bodies of a living actor posed as `pose` (each bone in the
    /// skeleton's space) and placed by `placement`: how far along it, and
    /// the bone of the body it meets. Alive, the game keeps these same
    /// bodies on the bones (`bhkBlendCollisionObject`s) and shots hit them.
    pub fn ray_hit(
        &self,
        pose: &[Transform],
        placement: &Transform,
        origin: Vec3,
        direction: Vec3,
    ) -> Option<(f32, usize)> {
        let mut best: Option<(f32, usize)> = None;
        for (b, offset) in self.ragdoll.bodies.iter().zip(&self.offsets) {
            let Some((a, c, r)) = b.capsule else {
                continue;
            };
            let Some(bone) = pose.get(b.bone) else {
                continue;
            };
            let frame = placement.then_child(bone).then_child(offset);
            let t = physics::shapes::ray_capsule(
                origin,
                direction,
                frame.apply_point(a),
                frame.apply_point(c),
                r * frame.scale,
            );
            if let Some(t) = t {
                if best.map_or(true, |(d, _)| t < d) {
                    best = Some((t, b.bone));
                }
            }
        }
        best
    }
}

/// The bone nearest a ray (`direction` of length 1) among those `keep`
/// accepts, for an actor posed as `pose` and placed by `placement`: by the
/// distance from each bone's origin to the ray's nearest point ahead. For
/// skeletons without bodies (a guess: the game needs the bodies).
pub fn nearest_bone(
    pose: &[Transform],
    placement: &Transform,
    origin: Vec3,
    direction: Vec3,
    keep: impl Fn(usize) -> bool,
) -> Option<usize> {
    let mut best: Option<(f32, usize)> = None;
    for (i, bone) in pose.iter().enumerate() {
        if !keep(i) {
            continue;
        }
        let p = placement.apply_point(bone.translation);
        let to = [0, 1, 2].map(|k| p[k] - origin[k]);
        let t = (0..3).map(|k| to[k] * direction[k]).sum::<f32>().max(0.0);
        let d2: f32 = (0..3)
            .map(|k| (origin[k] + direction[k] * t - p[k]).powi(2))
            .sum();
        if best.map_or(true, |(d, _)| d2 < d) {
            best = Some((d2, i));
        }
    }
    best.map(|(_, i)| i)
}

fn limit(l: &JointLimit) -> Limit {
    match *l {
        JointLimit::Ragdoll {
            twist,
            plane,
            cone,
            plane_range,
            twist_range,
        } => Limit::Cone {
            twist,
            plane,
            cone,
            plane_range,
            twist_range,
        },
        JointLimit::Hinge {
            axle,
            perpendicular,
            range,
        } => Limit::Hinge {
            axle,
            perpendicular,
            range,
        },
        JointLimit::Free => Limit::Free,
    }
}

/// A joint's angles in the skeleton file's own pose, against its limits
/// (for checking how the limits are read: the file's pose should fall
/// inside every one).
pub fn file_pose_angles(ragdoll: &Ragdoll) -> Vec<sim::JointAngles> {
    ragdoll
        .joints
        .iter()
        .map(|j| {
            let r = |k: usize| ragdoll.bodies[j.bodies[k]].frame.rotation;
            sim::joint_angles(&limit(&j.limit), &r(0), &r(1))
        })
        .collect()
}

/// How far apart each joint's two pivots are in the file's own pose (they
/// should meet).
pub fn file_pose_gaps(ragdoll: &Ragdoll) -> Vec<f32> {
    ragdoll
        .joints
        .iter()
        .map(|j| {
            let p = |k: usize| {
                let f = &ragdoll.bodies[j.bodies[k]].frame;
                f.apply_point(j.pivots[k])
            };
            let (a, b) = (p(0), p(1));
            ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::RagdollBody;

    fn close(a: Vec3, b: Vec3) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3)
    }

    #[test]
    fn rays_find_the_body_they_meet_and_its_bone() {
        // Bip01 at the origin; the spine 68 up, turned 90° about z (its x
        // along the world's y), with a body whose capsule runs 5 along it
        // from 2 out, radius 3.
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let bones = vec![
            Bone {
                name: "Bip01".into(),
                parent: None,
                local: Transform::IDENTITY,
            },
            Bone {
                name: "Bip01 Spine".into(),
                parent: Some(0),
                local: Transform {
                    rotation: turn,
                    translation: [0.0, 0.0, 68.0],
                    scale: 1.0,
                },
            },
        ];
        let body = RagdollBody {
            bone: 1,
            bone_name: "Bip01 Spine".into(),
            frame: Transform {
                rotation: turn,
                translation: [0.0, 2.0, 68.0],
                scale: 1.0,
            },
            mass: 9.0,
            center: [0.0; 3],
            inertia: [10.0; 3],
            linear_damping: 0.1,
            angular_damping: 0.05,
            friction: 0.3,
            restitution: 0.8,
            max_linear_speed: 7000.0,
            max_angular_speed: 30.0,
            capsule: Some(([0.0; 3], [5.0, 0.0, 0.0], 3.0)),
            layer: 8,
            part: 3,
        };
        let rig = RagdollRig::new(
            &bones,
            Ragdoll {
                bodies: vec![body],
                joints: Vec::new(),
            },
        );
        let pose = file_pose(&bones);
        // Placed 100 east: the capsule runs from (100, 2, 68) to (100, 7,
        // 68). A ray along x at y 4 meets its side 3 before the axis.
        let placement = Transform {
            translation: [100.0, 0.0, 0.0],
            ..Transform::IDENTITY
        };
        let (t, bone) = rig
            .ray_hit(&pose, &placement, [50.0, 4.0, 68.0], [1.0, 0.0, 0.0])
            .unwrap();
        assert!((t - 47.0).abs() < 1e-3, "{t}");
        assert_eq!(bone, 1);
        // Above it: nothing (the game's shots pass between the bodies).
        assert!(rig
            .ray_hit(&pose, &placement, [50.0, 4.0, 80.0], [1.0, 0.0, 0.0])
            .is_none());
        // The nearest bone to a ray, among those allowed.
        let ray = ([50.0, 0.0, 60.0], [1.0, 0.0, 0.0]);
        assert_eq!(
            nearest_bone(&pose, &placement, ray.0, ray.1, |_| true),
            Some(1)
        );
        assert_eq!(
            nearest_bone(&pose, &placement, ray.0, ray.1, |i| i == 0),
            Some(0)
        );
    }

    #[test]
    fn bodies_carry_their_bones_and_give_them_back() {
        // Bip01 at the origin, the spine 68 up and turned 90° about z; its
        // body sits 2 along the spine's x.
        let turn = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let bones = vec![
            Bone {
                name: "Bip01".into(),
                parent: None,
                local: Transform::IDENTITY,
            },
            Bone {
                name: "Bip01 Spine".into(),
                parent: Some(0),
                local: Transform {
                    rotation: turn,
                    translation: [0.0, 0.0, 68.0],
                    scale: 1.0,
                },
            },
        ];
        let body = RagdollBody {
            bone: 1,
            bone_name: "Bip01 Spine".into(),
            frame: Transform {
                rotation: turn,
                // 2 along the spine's x, which points along y.
                translation: [0.0, 2.0, 68.0],
                scale: 1.0,
            },
            mass: 9.0,
            center: [0.0; 3],
            inertia: [10.0; 3],
            linear_damping: 0.1,
            angular_damping: 0.05,
            friction: 0.3,
            restitution: 0.8,
            max_linear_speed: 7000.0,
            max_angular_speed: 30.0,
            capsule: Some(([0.0; 3], [5.0, 0.0, 0.0], 3.0)),
            layer: 8,
            part: 3,
        };
        let rig = RagdollRig::new(
            &bones,
            Ragdoll {
                bodies: vec![body],
                joints: Vec::new(),
            },
        );
        assert!(close(rig.offsets[0].translation, [2.0, 0.0, 0.0]));
        // Placed facing east at (100, 0, 0): the body starts where its
        // bone is, and reading it back gives the bones as they were.
        let placement = Transform {
            rotation: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [100.0, 0.0, 0.0],
            scale: 1.0,
        };
        let pose = file_pose(&bones);
        let sim = rig.start(&pose, &placement);
        let (_, origin) = sim.frame(0);
        let expected = placement.then_child(&pose[1]).apply_point([2.0, 0.0, 0.0]);
        assert!(close(origin, expected), "{origin:?} {expected:?}");
        let back = rig.bones(&sim, &bones, &pose, &placement);
        for (b, p) in back.iter().zip(&pose) {
            assert!(close(b.translation, p.translation), "{b:?} {p:?}");
            for k in 0..3 {
                assert!(close(b.rotation[k], p.rotation[k]));
            }
        }
    }
}
