//! Native head and eye tracking on placed people (`world::look_ik`): the
//! head bone turned toward whom they look at after the animation is
//! sampled, before skinning, as the game's `bhkRagdollController` does
//! in `DoRagdollAnim` (00c7d630); the eye angles it leaves go to the face
//! (`faces`, 00888970).
//!
//! Set up as 0087e130 does: the body part with IK data and the head
//! tracking flag (`DefaultBodyPartData`'s head) names the head bone
//! (`BPNI`) and its maximum angle; people (not creatures) also track with
//! their eyes, within the limits their FaceGen head sets (00607420).
//!
//! Whom they look at is `ai`'s head-track target (`world::head_track`).
//! Where (the target's look anchor, virtual +0x194): the player in first
//! person at the camera (`Camera1st`, 00952ff0, +0x64a clear); everyone
//! else, and the player in third person, at their `Bip01 Head` (008a2fa0),
//! for people with a head pose at the height of their own eye point
//! (00c757b0). The tracking distance is between the two positions.
//!
//! Inferences, see docs/OPENING_LOOK_IK.md: anchors are kept from the
//! last pose (an actor looking at one posed later in the frame sees the
//! last frame's), the player's third-person anchor keeps the head's own
//! height (whether the player's controller has a head pose isn't traced),
//! and the model frame is the skeleton root's.

use std::collections::HashMap;

use bevy::prelude::*;
use esm::FormId;
use world::dialogue::PLAYER_REF;
use world::look_ik::{havok_scale, LookIk, Pose, Qs, Settings, Target, TrackSettings};

use crate::actors::ActorRig;

/// The look-IK state of one placed person.
#[derive(Component)]
pub struct HeadTrack {
    pub ik: LookIk,
    /// The tracked head bone (skeleton index).
    pub head: Option<usize>,
    /// Their `Bip01 Head` (the look anchor's node, skeleton index).
    pub anchor: Option<usize>,
}

/// The LookIK settings: the executable's defaults and any INI values (the
/// shipped INI files set none).
#[derive(Resource)]
pub struct LookSettings(pub Settings);

/// Where each actor is looked at this frame ([`Target`], game units), by
/// reference, the player included.
#[derive(Resource, Default)]
pub struct LookAnchors(pub HashMap<FormId, Target>);

/// Gives each placed person their head tracking once their skeleton is
/// up (0087e130), with their head's eye limits (00607420).
pub fn set_up(
    mut commands: Commands,
    game: Res<crate::GameFiles>,
    rigs: Query<(Entity, &ActorRig, &crate::ai::Walker), Without<HeadTrack>>,
) {
    let order = &game.0.order;
    let mut track_settings = None;
    for (entity, rig, walker) in &rigs {
        let bones = &rig.skeleton.bones;
        let mut track = HeadTrack {
            ik: LookIk::default(),
            head: None,
            anchor: bones
                .iter()
                .position(|b| b.name.eq_ignore_ascii_case(world::look_ik::ANCHOR_BONE)),
        };
        let ordered = bones
            .iter()
            .enumerate()
            .all(|(i, b)| b.parent.is_none_or(|p| p < i));
        let part = world::body_parts::BodyPartData::of(order, walker.reference)
            .and_then(|d| d.head_tracking_part().cloned());
        if let (Some(part), true) = (part, ordered) {
            let head = part
                .ik_start
                .as_deref()
                .and_then(|n| bones.iter().position(|b| b.name.eq_ignore_ascii_case(n)));
            let mut pose = bind_pose(bones);
            let creature = world::body_parts::is_creature(order, walker.reference);
            track
                .ik
                .init(&mut pose, head, creature, part.tracking_max_angle);
            track.head = head.filter(|_| track.ik.init);
            // 00607420: an NPC's FaceGen head sets the eye's limits.
            if !creature && track.ik.init {
                let s = track_settings.get_or_insert_with(|| {
                    TrackSettings::read(|name| world::scripting::game_setting(order, name))
                });
                track.ik.head_attached(s);
            }
        }
        commands.entity(entity).insert(track);
    }
}

/// The skeleton's own pose (the scene graph before animation), from
/// which 00c79340 takes the bones' forward directions.
fn bind_pose(bones: &[nif::Bone]) -> Pose {
    let parents = bones
        .iter()
        .map(|b| b.parent.map_or(-1, |p| p as i16))
        .collect();
    let locals = bones.iter().map(|b| to_qs(&b.local)).collect();
    Pose::from_locals(parents, locals)
}

/// A game transform as Havok's (translation in Havok units).
fn to_qs(t: &nif::Transform) -> Qs {
    let q = world::animation::quat_from_matrix(&t.rotation);
    let h = havok_scale();
    Qs {
        t: [
            t.translation[0] * h,
            t.translation[1] * h,
            t.translation[2] * h,
            0.0,
        ],
        q: [q[1], q[2], q[3], q[0]],
        s: [t.scale; 4],
    }
}

/// Havok's transform back as the game's.
fn from_qs(t: &Qs) -> nif::Transform {
    let [x, y, z, w] = t.q;
    let h = 1.0 / havok_scale();
    nif::Transform {
        rotation: [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - z * w),
                2.0 * (x * z + y * w),
            ],
            [
                2.0 * (x * y + z * w),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - x * w),
            ],
            [
                2.0 * (x * z - y * w),
                2.0 * (y * z + x * w),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ],
        translation: [t.t[0] * h, t.t[1] * h, t.t[2] * h],
        scale: t.s[0],
    }
}

/// Runs one update of head tracking on a posed skeleton: `pose` holds the
/// bones' transforms in the skeleton root's space (as `ActorRig::pose_now`
/// gives them) and is changed for the head and everything under it.
/// `position` is the actor's own (the distance's other end).
pub fn track(
    track: &mut HeadTrack,
    settings: &Settings,
    bones: &[nif::Bone],
    pose: &mut [nif::Transform],
    placement: &nif::Transform,
    position: [f32; 3],
    target: Option<Target>,
) {
    let Some(head) = track.head else {
        return;
    };
    track.ik.update_target(settings, position, target);
    if !(track.ik.active && settings.look_ik && settings.ragdoll_anim) {
        return;
    }
    // The bones' local transforms from the posed ones.
    let locals: Vec<nif::Transform> = bones
        .iter()
        .enumerate()
        .map(|(i, b)| match b.parent {
            Some(p) => pose[p].inverse().then_child(&pose[i]),
            None => pose[i],
        })
        .collect();
    let parents = bones
        .iter()
        .map(|b| b.parent.map_or(-1, |p| p as i16))
        .collect();
    let mut sg = Pose::from_locals(parents, locals.iter().map(to_qs).collect());
    let world_from_model = to_qs(placement);
    track.ik.do_look_at_ik(settings, &mut sg, &world_from_model);
    // The head's new local transform; everything under it keeps its own.
    let mut locals = locals;
    locals[head] = from_qs(&sg.local(head));
    let mut below = vec![false; bones.len()];
    below[head] = true;
    for i in head..bones.len() {
        if i != head && !bones[i].parent.is_some_and(|p| below[p]) {
            continue;
        }
        below[i] = true;
        pose[i] = match bones[i].parent {
            Some(p) => pose[p].then_child(&locals[i]),
            None => locals[i],
        };
    }
}

/// Where this person is looked at by others from now on (008a2fa0), from
/// their final pose: the `Bip01 Head`'s world position, at the height of
/// their own eye point when their controller has a head pose (00c757b0:
/// the head pose's eye bone under the head and the world transform).
pub fn anchor_of(
    track: Option<&HeadTrack>,
    bones: &[nif::Bone],
    pose: &[nif::Transform],
    placement: &nif::Transform,
    position: [f32; 3],
) -> Option<Target> {
    let node = track.and_then(|t| t.anchor).or_else(|| {
        bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case(world::look_ik::ANCHOR_BONE))
    })?;
    let head = placement.apply_point(pose.get(node)?.translation);
    let eye_z = track.and_then(|t| {
        let eye = t.ik.eye_offset()?;
        let h = pose.get(t.head?)?;
        Some(placement.apply_point(h.apply_point(eye))[2])
    });
    // Without the node the game looks 0.9 of the actor's height up; the
    // viewer has no actor bounds, so such an actor isn't looked at.
    Some(Target {
        anchor: world::look_ik::actor_anchor(Some(head), eye_z, position, 0.0),
        position,
    })
}

/// Keeps the player's look anchor (00952ff0): in first person the camera
/// (`Camera1st`); in third person (+0x64a set) as an actor, their body's
/// `Bip01 Head` as last drawn. The distance is measured from their feet.
pub fn follow_player(
    state: Res<crate::dialogue::DialogueState>,
    view: Option<Res<crate::player_camera::PlayerView>>,
    body: Option<Res<crate::player_body::PlayerBody>>,
    camera: Query<&GlobalTransform, With<crate::FlyCamera>>,
    rigs: Query<&ActorRig>,
    joints: Query<&GlobalTransform>,
    mut anchors: ResMut<LookAnchors>,
) {
    let Some(feet) = state.0.player_position else {
        anchors.0.remove(&PLAYER_REF);
        return;
    };
    let third = view.as_ref().is_some_and(|v| v.camera.actually_third);
    let body_head = || {
        let rig = rigs.get(body.as_ref()?.root()?).ok()?;
        let i = rig
            .skeleton
            .bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case(world::look_ik::ANCHOR_BONE))?;
        let g = joints.get(*rig.joints.get(i)?).ok()?;
        Some(crate::walk::game_point(g.translation()))
    };
    let anchor = if third {
        body_head()
    } else {
        camera.iter().next().map(player_look_point)
    };
    match anchor {
        Some(anchor) => {
            anchors.0.insert(
                PLAYER_REF,
                Target {
                    anchor,
                    position: feet,
                },
            );
        }
        None => {
            anchors.0.remove(&PLAYER_REF);
        }
    }
}

/// Where someone looks at the player: the camera, in first person
/// (`PlayerCharacter::GetLookingAtLocation`, 00952ff0).
pub fn player_look_point(camera: &GlobalTransform) -> [f32; 3] {
    crate::walk::game_point(camera.translation())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bone(name: &str, parent: Option<usize>, z: f32) -> nif::Bone {
        nif::Bone {
            name: name.into(),
            parent,
            local: nif::Transform {
                translation: [0.0, 0.0, z],
                ..nif::Transform::IDENTITY
            },
        }
    }

    #[test]
    fn another_actor_is_looked_at_by_its_head_at_its_eye_height() {
        let bones = vec![
            bone("Bip01", None, 60.0),
            bone("Bip01 Neck1", Some(0), 50.0),
            bone("Bip01 Head", Some(1), 10.0),
        ];
        let pose = nif::posed(&bones, None, 0.0);
        let placement = nif::Transform {
            translation: [100.0, 0.0, 10.0],
            ..nif::Transform::IDENTITY
        };
        // Without a controller: the head node itself.
        let t = anchor_of(None, &bones, &pose, &placement, [100.0, 0.0, 10.0]).unwrap();
        assert!((t.anchor[2] - 130.0).abs() < 1e-3, "{t:?}");
        assert_eq!(t.position, [100.0, 0.0, 10.0]);
        // A person's controller: z from the eye point 9 up the head's +X
        // (here the model's +X, so the height is the head's), 6 along +Y.
        let mut bind = bind_pose(&bones);
        let mut ik = LookIk::default();
        assert!(ik.init(&mut bind, Some(2), false, 60.0));
        let tr = HeadTrack {
            ik,
            head: Some(2),
            anchor: Some(2),
        };
        let t = anchor_of(Some(&tr), &bones, &pose, &placement, [100.0, 0.0, 10.0]).unwrap();
        assert!((t.anchor[0] - 100.0).abs() < 1e-3, "{t:?}");
        assert!((t.anchor[2] - 130.0).abs() < 1e-3, "{t:?}");
        // Headless: nobody to look at.
        let headless = vec![bone("Bip01", None, 60.0)];
        let p = nif::posed(&headless, None, 0.0);
        assert!(anchor_of(None, &headless, &p, &placement, [0.0; 3]).is_none());
    }
}
