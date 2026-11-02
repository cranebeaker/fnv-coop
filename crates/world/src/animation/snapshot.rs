//! Persistence for an actor's in-progress animation state.
//!
//! The snapshot stores animation clocks and blend state, but identifies each
//! sequence by a caller-supplied asset key. The viewer can therefore reload
//! game-owned KF files from its current asset collection instead of putting
//! any game bytes in a save.

use std::sync::Arc;

use nif::anim::{Bone, Pose, Sequence};

use super::{bone_tracks, Active, GroupData, Player, Settings, State};

/// A bone's identity in the exact order used by the animation skeleton.
///
/// Frozen blend poses are indexed by this order, so names alone are not
/// enough when two bones are rearranged in a modded skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoneIdentity {
    pub name: String,
    pub parent: Option<usize>,
}

/// An actor's animation state, detached from loaded sequence allocations.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    /// Ordered names and parent indexes for the skeleton this state uses.
    pub bones: Vec<BoneIdentity>,
    pub settings: Settings,
    pub movement_rate: f32,
    pub weapon_rate: f32,
    pub active: Vec<ActiveSnapshot>,
}

/// A playing sequence or a frozen blend source.
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSnapshot {
    /// Caller-defined stable asset path/key for `Active::seq`. Frozen poses
    /// have no sequence and therefore no key.
    pub sequence_key: Option<String>,
    /// Structural compatibility check for the currently installed asset.
    /// Keyframe contents are intentionally not copied into saves.
    pub sequence_signature: Option<SequenceSignature>,
    /// Per-bone frozen source pose. Its length and order match `Snapshot::bones`.
    pub frozen: Vec<Option<(u8, Pose)>>,
    pub group: u8,
    pub section: u8,
    /// Saved verbatim so text-key loop boundaries and blend data survive.
    pub data: GroupData,
    pub state: State,
    pub ease: f32,
    pub ease_length: f32,
    pub time: f32,
    pub loops_left: i32,
    pub weight: f32,
}

/// The sequence structure needed to reject obviously incompatible assets
/// when restoring clocks and blend state. This is not a content hash: changes
/// to transforms or text keys that leave these fields unchanged are not
/// detected, so the same assets and load order must be available on restore.
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceSignature {
    pub name: String,
    pub start: f32,
    pub stop: f32,
    pub looping: bool,
    /// Ordered as in the source KF; track motion/key contents are omitted.
    pub tracks: Vec<TrackIdentity>,
    pub data: GroupData,
}

/// A track's topology relevant to rebuilding bone tracks and blend priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackIdentity {
    pub node: String,
    pub priority: u8,
}

/// A snapshot cannot be used safely with the requested bones/assets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    /// The supplied skeleton contains a parent index outside its bone list.
    InvalidSkeleton,
    /// The skeleton differs from the one whose bone indexes were saved.
    SkeletonMismatch,
    /// A playing sequence could not be assigned an asset key.
    UnidentifiedSequence,
    /// An asset key did not resolve to a sequence during restoration.
    MissingSequence(String),
    /// The resolved asset has a different structure from the saved one.
    SequenceMismatch(String),
    /// Saved live state violates the internal representation's structure.
    InvalidActive(usize),
    /// An animation field contains NaN or an infinite value.
    NonFinite,
}

impl std::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSkeleton => write!(f, "animation snapshot skeleton is invalid"),
            Self::SkeletonMismatch => write!(f, "animation snapshot skeleton does not match"),
            Self::UnidentifiedSequence => {
                write!(f, "playing animation has no persistent asset key")
            }
            Self::MissingSequence(key) => {
                write!(f, "animation asset {key:?} could not be loaded")
            }
            Self::SequenceMismatch(key) => {
                write!(
                    f,
                    "animation asset {key:?} no longer matches its saved structure"
                )
            }
            Self::InvalidActive(i) => write!(f, "animation snapshot entry {i} is invalid"),
            Self::NonFinite => write!(f, "animation snapshot contains a non-finite number"),
        }
    }
}

impl std::error::Error for SnapshotError {}

impl Player {
    /// Copies the current state using stable keys assigned to playing KF
    /// allocations by the caller.
    ///
    /// This does not serialize `Arc` identity or loaded game data. Every
    /// playing sequence must resolve to a key, while a frozen blend source is
    /// kept as its exact per-bone pose.
    pub fn snapshot(
        &self,
        bones: &[Bone],
        mut identify: impl FnMut(&Arc<Sequence>) -> Option<String>,
    ) -> Result<Snapshot, SnapshotError> {
        let bone_signature = signature(bones)?;
        if !finite(self.settings.default_blend)
            || !finite(self.settings.mult)
            || !finite(self.movement_rate)
            || !finite(self.weapon_rate)
        {
            return Err(SnapshotError::NonFinite);
        }
        let mut active = Vec::with_capacity(self.active.len());
        for (i, a) in self.active.iter().enumerate() {
            validate_active(a, i, bones.len())?;
            let (sequence_key, sequence_signature) = match &a.seq {
                Some(seq) => {
                    let signature = sequence_signature(seq);
                    validate_sequence_signature(&signature)?;
                    (
                        Some(
                            identify(seq)
                                .filter(|key| !key.is_empty())
                                .ok_or(SnapshotError::UnidentifiedSequence)?,
                        ),
                        Some(signature),
                    )
                }
                None => (None, None),
            };
            active.push(ActiveSnapshot {
                sequence_key,
                sequence_signature,
                frozen: a.frozen.clone(),
                group: a.group,
                section: a.section,
                data: a.data,
                state: a.state,
                ease: a.ease,
                ease_length: a.ease_length,
                time: a.time,
                loops_left: a.loops_left,
                weight: a.weight,
            });
        }
        Ok(Snapshot {
            bones: bone_signature,
            settings: self.settings,
            movement_rate: self.movement_rate,
            weapon_rate: self.weapon_rate,
            active,
        })
    }
}

impl Snapshot {
    /// Rebuilds a fresh animation player from current game assets.
    ///
    /// All keys are resolved and all entries validated before constructing
    /// the returned player. The input snapshot and any existing player are
    /// left untouched on every error.
    pub fn restore(
        &self,
        bones: &[Bone],
        mut resolve: impl FnMut(&str) -> Option<Arc<Sequence>>,
    ) -> Result<Player, SnapshotError> {
        if self.bones != signature(bones)? {
            return Err(SnapshotError::SkeletonMismatch);
        }
        if !finite(self.settings.default_blend)
            || !finite(self.settings.mult)
            || !finite(self.movement_rate)
            || !finite(self.weapon_rate)
        {
            return Err(SnapshotError::NonFinite);
        }

        let mut active = Vec::with_capacity(self.active.len());
        for (i, saved) in self.active.iter().enumerate() {
            let seq = match (&saved.sequence_key, &saved.sequence_signature) {
                (Some(key), Some(expected)) if !key.is_empty() => {
                    let seq =
                        resolve(key).ok_or_else(|| SnapshotError::MissingSequence(key.clone()))?;
                    validate_sequence_signature(expected)?;
                    if sequence_signature(&seq) != *expected {
                        return Err(SnapshotError::SequenceMismatch(key.clone()));
                    }
                    Some(seq)
                }
                (Some(_), _) | (None, Some(_)) => {
                    return Err(SnapshotError::InvalidActive(i));
                }
                (None, None) => None,
            };
            let attach = seq.as_ref().map_or(0.0, |s| GroupData::read(s).attach);
            let candidate = Active {
                bone_tracks: seq
                    .as_ref()
                    .map_or_else(Vec::new, |seq| bone_tracks(seq, bones)),
                seq,
                frozen: saved.frozen.clone(),
                group: saved.group,
                section: saved.section,
                data: GroupData {
                    attach,
                    ..saved.data
                },
                state: saved.state,
                ease: saved.ease,
                ease_length: saved.ease_length,
                time: saved.time,
                loops_left: saved.loops_left,
                weight: saved.weight,
            };
            validate_active(&candidate, i, bones.len())?;
            active.push(candidate);
        }

        Ok(Player {
            active,
            settings: self.settings,
            movement_rate: self.movement_rate,
            weapon_rate: self.weapon_rate,
            // Lasts one update at most: nothing to restore.
            skip_blend: false,
            // The camera's player has no weapon section of its own.
            weapon_drawn: false,
        })
    }
}

fn signature(bones: &[Bone]) -> Result<Vec<BoneIdentity>, SnapshotError> {
    if bones
        .iter()
        .any(|bone| bone.parent.is_some_and(|parent| parent >= bones.len()))
    {
        return Err(SnapshotError::InvalidSkeleton);
    }
    Ok(bones
        .iter()
        .map(|bone| BoneIdentity {
            name: bone.name.clone(),
            parent: bone.parent,
        })
        .collect())
}

fn sequence_signature(seq: &Sequence) -> SequenceSignature {
    SequenceSignature {
        name: seq.name.clone(),
        start: seq.start,
        stop: seq.stop,
        looping: seq.looping,
        tracks: seq
            .tracks
            .iter()
            .map(|track| TrackIdentity {
                node: track.node.clone(),
                priority: track.priority,
            })
            .collect(),
        // The attach key isn't saved (camera snapshots predate it): it is
        // read again from the file on restore.
        data: GroupData {
            attach: 0.0,
            ..GroupData::read(seq)
        },
    }
}

fn validate_sequence_signature(signature: &SequenceSignature) -> Result<(), SnapshotError> {
    if !finite(signature.start)
        || !finite(signature.stop)
        || !finite(signature.data.start)
        || !finite(signature.data.end)
        || !finite(signature.data.loop_start)
        || !finite(signature.data.loop_end)
        || signature.data.travel.iter().any(|&value| !finite(value))
    {
        return Err(SnapshotError::NonFinite);
    }
    Ok(())
}

fn validate_active(a: &Active, index: usize, bone_count: usize) -> Result<(), SnapshotError> {
    if a.section as usize >= super::section::COUNT
        || a.bone_tracks.len() != if a.seq.is_some() { bone_count } else { 0 }
        || a.frozen.len() != if a.seq.is_none() { bone_count } else { 0 }
    {
        return Err(SnapshotError::InvalidActive(index));
    }
    if !finite(a.ease)
        || !finite(a.ease_length)
        || !finite(a.time)
        || !finite(a.weight)
        || !finite(a.data.start)
        || !finite(a.data.end)
        || !finite(a.data.loop_start)
        || !finite(a.data.loop_end)
        || a.data.travel.iter().any(|&value| !finite(value))
    {
        return Err(SnapshotError::NonFinite);
    }
    if a.frozen.iter().flatten().any(|(_, pose)| {
        pose.translation
            .is_some_and(|v| v.iter().any(|&x| !finite(x)))
            || pose.rotation.is_some_and(|q| q.iter().any(|&x| !finite(x)))
            || pose.scale.is_some_and(|x| !finite(x))
    }) {
        return Err(SnapshotError::NonFinite);
    }
    Ok(())
}

fn finite(value: f32) -> bool {
    value.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::anim::{Motion, Track};
    use nif::Transform;

    fn bones() -> Vec<Bone> {
        vec![
            Bone {
                name: "Bip01".into(),
                parent: None,
                local: Transform::IDENTITY,
            },
            Bone {
                name: "Camera1st".into(),
                parent: Some(0),
                local: Transform {
                    translation: [0.0, 0.0, 10.0],
                    ..Transform::IDENTITY
                },
            },
        ]
    }

    fn camera_sequence() -> Arc<Sequence> {
        Arc::new(Sequence {
            name: "SpecialIdle".into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![
                (0.0, "Start".into()),
                (0.0, "StartLoop".into()),
                (0.7, "EndLoop".into()),
                (1.0, "End".into()),
                (0.0, "BlendIn:6".into()),
                (0.0, "BlendOut:3".into()),
            ],
            tracks: vec![Track {
                node: "Camera1st".into(),
                priority: 10,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0, 0.0, 10.0]), (1.0, [0.0, 0.0, 20.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        })
    }

    fn round_trip(player: &Player, bones: &[Bone], sequence: &Arc<Sequence>) -> Player {
        let snapshot = player
            .snapshot(bones, |found| {
                Arc::ptr_eq(found, sequence).then(|| "camera.kf".into())
            })
            .unwrap();
        snapshot
            .restore(bones, |key| {
                (key == "camera.kf").then(|| Arc::clone(sequence))
            })
            .unwrap()
    }

    #[test]
    fn frozen_blend_source_and_destination_resume_at_the_same_pose() {
        let bones = bones();
        let sequence = camera_sequence();
        let mut original = Player::default();
        assert!(original.request_special_idle(&sequence, 2, &bones));
        original.update(0.075);
        assert_eq!(original.active.len(), 2);
        assert!(original.active.iter().any(|a| a.seq.is_none()));

        let mut restored = round_trip(&original, &bones, &sequence);
        assert_eq!(
            original
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap(),
            restored
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap()
        );
        assert_eq!(original.pose(&bones), restored.pose(&bones));

        for dt in [0.05, 0.1, 0.2, 0.15] {
            original.update(dt);
            restored.update(dt);
            assert_eq!(original.pose(&bones), restored.pose(&bones));
            assert_eq!(
                original
                    .snapshot(&bones, |_| Some("camera.kf".into()))
                    .unwrap(),
                restored
                    .snapshot(&bones, |_| Some("camera.kf".into()))
                    .unwrap()
            );
        }
    }

    #[test]
    fn sequence_clock_and_finite_loop_count_continue_after_restore() {
        let bones = bones();
        let sequence = camera_sequence();
        let mut original = Player::default();
        original.request_special_idle(&sequence, 1, &bones);
        original.update(0.6);
        original.update(0.2);
        let saved = original
            .snapshot(&bones, |_| Some("camera.kf".into()))
            .unwrap();
        let destination = saved
            .active
            .iter()
            .find(|a| a.sequence_key.is_some())
            .unwrap();
        assert_eq!(destination.loops_left, 0);
        let mut restored = saved
            .restore(&bones, |_| Some(Arc::clone(&sequence)))
            .unwrap();

        original.update(0.15);
        restored.update(0.15);
        assert_eq!(original.pose(&bones), restored.pose(&bones));
        assert_eq!(
            original
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap(),
            restored
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap()
        );
    }

    #[test]
    fn rejects_asset_skeleton_and_non_finite_mismatches_without_mutation() {
        let bones = bones();
        let sequence = camera_sequence();
        let mut player = Player::default();
        player.request_special_idle(&sequence, -1, &bones);
        player.update(0.05);
        let snapshot = player
            .snapshot(&bones, |_| Some("camera.kf".into()))
            .unwrap();
        let before = snapshot.clone();

        assert_eq!(
            snapshot.restore(&bones, |_| None).unwrap_err(),
            SnapshotError::MissingSequence("camera.kf".into())
        );
        let mut changed_asset = (*sequence).clone();
        changed_asset.tracks[0].node = "Bip01".into();
        assert_eq!(
            snapshot
                .restore(&bones, |_| Some(Arc::new(changed_asset.clone())))
                .unwrap_err(),
            SnapshotError::SequenceMismatch("camera.kf".into())
        );
        let reordered = vec![
            bones[1].clone(),
            Bone {
                parent: None,
                ..bones[0].clone()
            },
        ];
        assert_eq!(
            snapshot
                .restore(&reordered, |_| Some(Arc::clone(&sequence)))
                .unwrap_err(),
            SnapshotError::SkeletonMismatch
        );

        let mut non_finite = snapshot.clone();
        non_finite.active[1].time = f32::NAN;
        assert_eq!(
            non_finite
                .restore(&bones, |_| Some(Arc::clone(&sequence)))
                .unwrap_err(),
            SnapshotError::NonFinite
        );
        assert_eq!(snapshot, before);
        assert_eq!(
            player.pose(&bones),
            snapshot
                .restore(&bones, |_| Some(Arc::clone(&sequence)))
                .unwrap()
                .pose(&bones)
        );
    }

    #[test]
    fn preserves_live_group_data_with_injected_root_travel() {
        let bones = bones();
        let sequence = camera_sequence();
        let mut player = Player::default();
        player.request_special_idle(&sequence, -1, &bones);
        player.update(0.05);
        // Group travel is supplied by the animation group at play time, not
        // necessarily encoded in the KF text keys used by its signature.
        let active = player.active.iter_mut().find(|a| a.seq.is_some()).unwrap();
        active.data.travel = [1.0, 2.0, 3.0];

        let restored = round_trip(&player, &bones, &sequence);
        assert_eq!(
            restored
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap(),
            player
                .snapshot(&bones, |_| Some("camera.kf".into()))
                .unwrap()
        );
        assert_eq!(restored.pose(&bones), player.pose(&bones));
    }
}
