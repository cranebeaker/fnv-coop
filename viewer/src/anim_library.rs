//! The animations an actor's 3D loads, by group id (`world::animation::
//! groups::AnimSet`): every `.kf` in the skeleton's folder, its
//! `Locomotion` folder (`00447330`, `ModelLoader.cpp`) and, for people, the
//! `Locomotion\Male` or `Locomotion\Female` folder their walk comes from
//! (`008b73f0`), each file's group read off its sequence's name
//! (`005f3a20`) and its kinds off its name (`005f38d0`). The sequences are
//! read when first played and kept.
//!
//! (The game can instead load a precached list per weapon kind,
//! `00600700`, when one is present; how those lists are made isn't
//! traced. Loading every file of the folders gives the lookup the same
//! files for the kinds an actor uses.)

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::Resource;
use preview::cell::ActorSkeleton;
use world::animation::groups::AnimSet;
use world::animation::pick::Library;

/// The sets read so far (by their folders) and the sequences.
#[derive(Resource, Default)]
pub struct AnimLibrary {
    sets: HashMap<String, Arc<AnimSet>>,
    sequences: HashMap<String, Option<Arc<nif::Sequence>>>,
    /// The player's third-person set, once their body has animated
    /// (`PlayerCharacter::GetAnimation(0)`: the damage-a-second figures
    /// read the player's attack animations, `world::dps`).
    pub player: Option<Arc<AnimSet>>,
}

/// The folders (paths under `meshes\`, lower case) an actor's animations
/// come from.
pub fn folders(skeleton: &ActorSkeleton) -> Vec<String> {
    let parent = |p: &str| {
        let p = assets::mesh_path(p);
        p.rfind('\\').map(|i| p[..i].to_string())
    };
    let mut out = Vec::new();
    if let Some(base) = parent(&skeleton.skeleton_path) {
        out.push(format!("{base}\\locomotion"));
        out.push(base);
    }
    if let Some(walk) = parent(&skeleton.walk_path) {
        if !out.contains(&walk) {
            out.push(walk);
        }
    }
    out
}

impl AnimLibrary {
    /// The set for an actor's skeleton, read once per set of folders. The
    /// skeleton's own idle and walk are kept as the sequences of their
    /// files (so the idle's phase spread carries on).
    pub fn set_for(&mut self, game: &cellview::Game, skeleton: &ActorSkeleton) -> Arc<AnimSet> {
        for (path, seq) in [
            (&skeleton.idle_path, &skeleton.idle),
            (&skeleton.walk_path, &skeleton.walk),
        ] {
            if let Some(seq) = seq {
                self.sequences
                    .entry(assets::mesh_path(path))
                    .or_insert_with(|| Some(seq.clone()));
            }
        }
        let folders = folders(skeleton);
        let key = folders.join("|");
        if let Some(set) = self.sets.get(&key) {
            return set.clone();
        }
        let started = std::time::Instant::now();
        let mut files: Vec<String> = game
            .assets
            .paths()
            .filter(|p| p.ends_with(".kf"))
            .filter(|p| {
                folders.iter().any(|f| {
                    p.len() > f.len() + 1
                        && p.starts_with(f.as_str())
                        && p.as_bytes()[f.len()] == b'\\'
                        && !p[f.len() + 1..].contains('\\')
                })
            })
            .map(str::to_string)
            .collect();
        files.sort();
        let mut set = AnimSet::default();
        for path in &files {
            let name = self
                .sequence_at(game, path)
                .map(|s| s.name.clone())
                .unwrap_or_default();
            set.add(path, &name);
        }
        println!(
            "Animations for {key}: {} files, {} groups ({:.2} s)",
            files.len(),
            set.len(),
            started.elapsed().as_secs_f32()
        );
        let set = Arc::new(set);
        self.sets.insert(key, set.clone());
        set
    }

    /// The player's animations as `world::dps` asks for them (the first
    /// file of a group with several), when their body has animated.
    pub fn player_library<'a>(&'a mut self, game: &'a cellview::Game) -> Option<PlayerLibrary<'a>> {
        let set = self.player.clone()?;
        Some(PlayerLibrary {
            set,
            library: self,
            game,
        })
    }

    /// A file's sequence (a path under `meshes\`), read once.
    pub fn sequence_at(&mut self, game: &cellview::Game, path: &str) -> Option<Arc<nif::Sequence>> {
        self.sequences
            .entry(assets::mesh_path(path))
            .or_insert_with(|| crate::viewmodel::sequence(game, path).map(Arc::new))
            .clone()
    }
}

/// The player's set for `world::dps` (their attack animations' `a:` keys).
pub struct PlayerLibrary<'a> {
    set: Arc<AnimSet>,
    library: &'a mut AnimLibrary,
    game: &'a cellview::Game,
}

impl Library for PlayerLibrary<'_> {
    fn set(&self) -> &AnimSet {
        &self.set
    }

    fn sequence(&mut self, id: u16) -> Option<Arc<nif::Sequence>> {
        let path = self.set.file(id, 0)?.to_string();
        self.library.sequence_at(self.game, &path)
    }
}

/// One actor's view of the library for `world::animation::pick`: its set,
/// the sequences, and the random draw for groups with several files.
pub struct ActorLibrary<'a> {
    pub set: &'a AnimSet,
    pub library: &'a mut AnimLibrary,
    pub game: &'a cellview::Game,
    pub draw: u32,
}

impl Library for ActorLibrary<'_> {
    fn set(&self) -> &AnimSet {
        self.set
    }

    fn sequence(&mut self, id: u16) -> Option<Arc<nif::Sequence>> {
        let path = self.set.file(id, self.draw)?.to_string();
        self.library.sequence_at(self.game, &path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn people_load_the_skeleton_locomotion_and_gender_folders() {
        let sk = ActorSkeleton {
            skeleton_path: "Characters\\_Male\\Skeleton.NIF".into(),
            walk_path: "Characters\\_Male\\locomotion\\female\\mtforward.kf".into(),
            ..Default::default()
        };
        assert_eq!(
            folders(&sk),
            vec![
                "meshes\\characters\\_male\\locomotion".to_string(),
                "meshes\\characters\\_male".to_string(),
                "meshes\\characters\\_male\\locomotion\\female".to_string(),
            ]
        );
        // A creature's walk is in its locomotion folder: nothing more.
        let gecko = ActorSkeleton {
            skeleton_path: "Creatures\\NVGecko\\Skeleton.NIF".into(),
            walk_path: "Creatures\\NVGecko\\locomotion\\mtforward.kf".into(),
            ..Default::default()
        };
        assert_eq!(folders(&gecko).len(), 2);
    }
}
