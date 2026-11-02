//! Faces that move (`world::face`): everyone blinks every 1.5–4 s, and a
//! line with a lip sync file (`.lip`, beside its voice) moves the
//! speaker's mouth, tongue, teeth, brows and lids frame by frame, the voice
//! held back by the lead-in and `fSpeechDelay` so the two line up, as in
//! the game. A line without one leaves the mouth still (the game has no
//! fallback driven by the sound).
//!
//! The head parts' vertices are moved here, on the CPU, before the
//! graphics card skins them: a face piece gets its own copy of its mesh the
//! first time it moves, and is rewritten only when what it shows changes,
//! within the game's distances (`world::face::Reach`). Only positions move;
//! the normals stay as at rest (whether the game moves them isn't traced).
//!
//! While someone has a head-track target (`ai::Walker::looking_at`) their
//! eyes move after the keys have played, as the game's face update does
//! (`00663510` → `0064be40`): toward their look controller's eye heading
//! and pitch (`look::HeadTrack`, `00888970`) plus a dart offset, on the
//! LookLeft/Right/Up/Down morphs (`world::face::FaceAnimation::track_eyes`).
//!
//! Not done: the head turns a `.lip` carries (pitch, roll, yaw; which bone
//! they turn, and in what order, isn't traced) and expressions (moods, a
//! line's emotion), so the eyes always dart as for no expression.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::audio::{AudioSink, AudioSinkPlayback, PlaybackSettings};
use bevy::prelude::*;
use cellview::MeshData;
use esm::FormId;
use world::face::{FaceAnimation, FaceMorphs, FaceSettings, Reach, Weights};
use world::lip::{lip_path, Lip};

use crate::actors::ActorRig;
use crate::scripts::PlacedRef;
use crate::walk::game_point;
use crate::FlyCamera;

/// The settings faces follow (game settings and the INI's).
#[derive(Resource)]
pub struct Faces(pub FaceSettings);

impl Faces {
    pub fn new(game: &cellview::Game) -> Faces {
        Faces(FaceSettings::read(&game.order, |section, key| {
            game.settings.float(section, key)
        }))
    }
}

/// A head part's piece (head, mouth, teeth, tongue, eyes, brows, beard):
/// its morphs and its vertices at rest.
#[derive(Component)]
pub struct FacePiece {
    morphs: Arc<FaceMorphs>,
    rest: Vec<[f32; 3]>,
    /// It has its own copy of the mesh (the placements of a model share
    /// one until their faces move).
    own_mesh: bool,
}

impl FacePiece {
    /// For a piece whose part has morphs.
    pub fn of(data: &MeshData) -> Option<FacePiece> {
        let morphs = data.rig.as_ref()?.face.clone()?;
        Some(FacePiece {
            morphs,
            rest: data.positions.clone(),
            own_mesh: false,
        })
    }
}

/// An actor's face, on its root: what it says and when it blinks, and the
/// weights its pieces show.
#[derive(Component)]
pub struct ActorFace {
    animation: FaceAnimation,
    shown: Weights,
    /// The head bone's joint, for the distance to the camera.
    head: Option<Entity>,
}

/// A spoken line's sound: who says it and its lip sync.
#[derive(Component)]
pub(crate) struct Voice {
    pub(crate) speaker: FormId,
    pub(crate) lip: Option<Arc<Lip>>,
}

/// Stop voice entities and their face tracks when a custom save replaces
/// dialogue state. Other audio entities are deliberately left alone.
pub(crate) fn discard_voices(world: &mut World) {
    let voices = {
        let mut query = world.query::<(Entity, &Voice)>();
        query
            .iter(world)
            .map(|(entity, voice)| (entity, voice.speaker))
            .collect::<Vec<_>>()
    };
    let speakers: std::collections::HashSet<u32> =
        voices.iter().map(|(_, speaker)| speaker.0).collect();
    for (entity, _) in voices {
        let _ = world.despawn(entity);
    }
    let mut faces = world.query::<(&PlacedRef, &mut ActorFace)>();
    for (placed, mut face) in faces.iter_mut(world) {
        // A saved world has no matching continuation for any old lip track.
        // Re-seed the face's ordinary animation state for every stale voice
        // owner, including tracks whose audio entity already ended.
        if speakers.contains(&placed.0) || face.animation.speaking() {
            face.animation = FaceAnimation::new(u64::from(placed.0));
        }
    }
}

/// A voice held back until its line's lead-in has gone by: seconds left.
#[derive(Component)]
pub(crate) struct VoiceDelay(pub(crate) f32);

/// How a line's voice file plays, and its [`Voice`]: with a lip sync file
/// beside it, paused until [`release_voices`] lets it go; without one, at
/// once (`008a20d0`).
pub fn voice_playback(
    game: &cellview::Game,
    path: &str,
    speaker: FormId,
) -> (PlaybackSettings, Voice) {
    let lip = lip_path(path)
        .and_then(|p| game.assets.read(&p).ok().flatten())
        .and_then(|bytes| {
            Lip::parse(&bytes)
                .map_err(|e| println!("  couldn't read the lip sync beside {path}: {e}"))
                .ok()
        })
        .map(Arc::new);
    let settings = if lip.is_some() {
        PlaybackSettings::DESPAWN.paused()
    } else {
        PlaybackSettings::DESPAWN
    };
    (settings, Voice { speaker, lip })
}

/// A line was cut short (skipped, or the talk ended while it played): the
/// speaker's face drops the rest of it and eases back to rest (a guess,
/// see `FaceAnimation::hush`).
pub fn cut_short(commands: &mut Commands, speaker: FormId) {
    commands.queue(move |world: &mut World| {
        let mut faces = world.query::<(&PlacedRef, &mut ActorFace)>();
        for (placed, mut face) in faces.iter_mut(world) {
            if placed.0 == speaker.0 && face.animation.speaking() {
                face.animation.hush();
            }
        }
    });
}

/// Gives each actor with face pieces its face (blinking on its own
/// schedule, picked by its reference's form ID), and starts the lines
/// whose voices were just queued: their lip sync onto the speaker's face,
/// and the voice held back by the lead-in and `fSpeechDelay` (whether or
/// not the speaker is drawn).
pub fn start_lines(
    mut commands: Commands,
    faces: Res<Faces>,
    new_pieces: Query<&ChildOf, Added<FacePiece>>,
    roots: Query<(&PlacedRef, &ActorRig), Without<ActorFace>>,
    voices: Query<(Entity, &Voice), Added<Voice>>,
    mut speakers: Query<(&PlacedRef, &mut ActorFace)>,
) {
    for child_of in &new_pieces {
        let root = child_of.parent();
        let Ok((placed, rig)) = roots.get(root) else {
            continue;
        };
        let head = rig
            .skeleton
            .bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case("Bip01 Head"))
            .and_then(|i| rig.joints.get(i).copied());
        commands.entity(root).insert_if_new(ActorFace {
            animation: FaceAnimation::new(u64::from(placed.0)),
            shown: Weights::NEUTRAL,
            head,
        });
    }
    for (entity, voice) in &voices {
        let Some(lip) = &voice.lip else {
            continue;
        };
        let face = speakers
            .iter_mut()
            .find(|(placed, _)| placed.0 == voice.speaker.0);
        let timing = match face {
            Some((_, mut face)) => face.animation.speak(lip, &faces.0),
            None => world::face::line_timing(lip, &faces.0),
        };
        commands
            .entity(entity)
            .insert(VoiceDelay(timing.voice_delay));
    }
}

/// Lets held-back voices play once their delay is over (their sound is set
/// up paused, a frame after it's queued).
pub fn release_voices(
    mut commands: Commands,
    time: Res<Time>,
    mut waiting: Query<(Entity, &mut VoiceDelay, Option<&AudioSink>)>,
) {
    for (entity, mut delay, sink) in &mut waiting {
        delay.0 -= time.delta_secs();
        if delay.0 > 0.0 {
            continue;
        }
        if let Some(sink) = sink {
            println!("  voice playing: {entity}");
            sink.play();
            commands.entity(entity).remove::<VoiceDelay>();
        }
    }
}

/// What `animate_faces` reads of an actor: the face, the rig (still or
/// fallen), whom they look at and their look controller's eye angles.
type FaceOf = (
    Entity,
    &'static mut ActorFace,
    Option<&'static ActorRig>,
    Option<&'static crate::ai::Walker>,
    Option<&'static crate::look::HeadTrack>,
);

/// Every frame: each face moves on (its blinks, its line's keys, and its
/// eyes while it has someone to look at), and its pieces are redrawn when
/// the weights they show change. The dead don't blink (the game has an
/// "eyes closed" face state, presumably for them; who sets it isn't
/// traced, so their eyes stay as they were).
pub fn animate_faces(
    time: Res<Time>,
    faces: Res<Faces>,
    mut actors: Query<FaceOf>,
    joints: Query<&GlobalTransform>,
    cameras: Query<&GlobalTransform, With<FlyCamera>>,
    mut pieces: Query<(&mut FacePiece, &mut Mesh3d, &ChildOf)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let eye = cameras.single().ok().map(|c| game_point(c.translation()));
    let dt = time.delta_secs();
    let mut redraw: HashMap<Entity, Weights> = HashMap::new();
    for (root, mut face, rig, walker, look) in &mut actors {
        if rig.is_some_and(|r| r.still || r.ragdoll.is_some()) {
            continue;
        }
        face.animation.update(dt, &faces.0);
        // The eyes follow while there is someone to look at (`00663510`;
        // its three untraced conditions are not modelled).
        if walker.is_some_and(|w| w.looking_at().is_some()) {
            let desired = look.map_or((0.0, 0.0), |l| l.ik.desired_eyes());
            face.animation.track_eyes(dt, desired, &faces.0);
        }
        // Measured from the camera to the head, as the game does.
        let head = joints
            .get(face.head.unwrap_or(root))
            .ok()
            .map(|g| game_point(g.translation()));
        let reach = match (eye, head) {
            (Some(e), Some(h)) => {
                let d =
                    ((e[0] - h[0]).powi(2) + (e[1] - h[1]).powi(2) + (e[2] - h[2]).powi(2)).sqrt();
                Reach::at(d, &faces.0)
            }
            _ => Reach::ALL,
        };
        let now = face.animation.weights().within(reach);
        if now != face.shown {
            face.shown = now;
            redraw.insert(root, now);
        }
    }
    if redraw.is_empty() {
        return;
    }
    for (mut piece, mut mesh, child_of) in &mut pieces {
        let Some(weights) = redraw.get(&child_of.parent()) else {
            continue;
        };
        let mut positions = piece.rest.clone();
        piece.morphs.apply(weights, &mut positions);
        if !piece.own_mesh {
            let Some(copy) = meshes.get(&mesh.0).cloned() else {
                continue;
            };
            mesh.0 = meshes.add(copy);
            piece.own_mesh = true;
        }
        if let Some(m) = meshes.get_mut(&mesh.0) {
            m.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::RenderAssetUsages;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
    use world::face::{channel, Morph};
    use world::lip::LipFrame;

    fn positions(meshes: &Assets<Mesh>, handle: &Handle<Mesh>) -> Vec<[f32; 3]> {
        match meshes
            .get(handle)
            .and_then(|m| m.attribute(Mesh::ATTRIBUTE_POSITION))
        {
            Some(VertexAttributeValues::Float32x3(p)) => p.clone(),
            _ => Vec::new(),
        }
    }

    #[test]
    fn a_speaking_face_moves_its_own_copy_of_the_mesh() {
        let settings = FaceSettings {
            blink_down: 0.0,
            ..FaceSettings::DEFAULT
        };
        let mut world = World::new();
        world.insert_resource(Faces(settings));
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_millis(50));
        world.insert_resource(time);
        let rest = vec![[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, rest.clone());
        let mut meshes = Assets::<Mesh>::default();
        let shared = meshes.add(mesh);
        world.insert_resource(meshes);
        // A line saying "Aah" throughout; the morph lowers vertex 0.
        let mut frame = LipFrame {
            phonemes: [0.0; 16],
            modifiers: [0.0; 17],
        };
        frame.phonemes[0] = 1.0;
        let mut animation = FaceAnimation::new(1);
        animation.speak(
            &Lip {
                offset: 0,
                frames: vec![frame; 30],
            },
            &settings,
        );
        let root = world
            .spawn(ActorFace {
                animation,
                shown: Weights::NEUTRAL,
                head: None,
            })
            .id();
        let morphs = FaceMorphs {
            morphs: vec![Morph {
                channel: channel("Aah").unwrap(),
                moves: vec![(0, [0.0, 0.0, -2.0])],
            }],
        };
        let piece = world
            .spawn((
                FacePiece {
                    morphs: Arc::new(morphs),
                    rest: rest.clone(),
                    own_mesh: false,
                },
                Mesh3d(shared.clone()),
                ChildOf(root),
            ))
            .id();
        world.run_system_once(animate_faces).unwrap();
        let own = world.get::<Mesh3d>(piece).unwrap().0.clone();
        assert_ne!(own, shared, "the piece has its own mesh now");
        let meshes = world.resource::<Assets<Mesh>>();
        assert_eq!(positions(meshes, &own)[0], [0.0, 0.0, -2.0]);
        // Other placements sharing the model's mesh keep its rest shape.
        assert_eq!(positions(meshes, &shared), rest);
    }

    #[test]
    fn reload_clears_a_lip_track_after_its_voice_entity_has_ended() {
        let mut animation = FaceAnimation::new(42);
        animation.speak(
            &Lip {
                offset: 0,
                frames: vec![LipFrame {
                    phonemes: [0.5; 16],
                    modifiers: [0.0; 17],
                }],
            },
            &FaceSettings::DEFAULT,
        );
        assert!(animation.speaking());
        let mut world = World::new();
        let actor = world
            .spawn((
                PlacedRef(42),
                ActorFace {
                    animation,
                    shown: Weights::NEUTRAL,
                    head: None,
                },
            ))
            .id();
        discard_voices(&mut world);
        assert!(!world.get::<ActorFace>(actor).unwrap().animation.speaking());
    }
}
