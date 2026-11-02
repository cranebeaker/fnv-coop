//! Distant objects: the game's merged blocks of far-off buildings, rocks
//! and other things (`cellview::game::LodBlock`, 4 × 4 cells each), the
//! ones the game keeps for where the player stands
//! (`world::lod::object_blocks`: the ordinary block within
//! `fBlockLoadDistanceLow`, 50000 units, the "high" one of the tallest
//! landmarks out to `fBlockLoadDistance`, 125000), loaded in the
//! background. Each block's part over a cell that's loaded in full is
//! hidden, as the game hides those segments, so the real objects stand
//! there instead.

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use cellview::game::LodBlock;
use world::lod::ObjectBlock;

use crate::exterior::Exterior;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles, Spawner};

/// Blocks loading at once.
const LOADING_AT_ONCE: usize = 3;

enum Block {
    Loading,
    Empty,
    /// Its entities, each with the cell its part covers (`None`: not
    /// tied to a cell).
    Loaded(Vec<(Entity, Option<(i32, i32)>)>),
}

type Finished = (ObjectBlock, Option<LodBlock>);

/// The blocks around the player.
#[derive(Resource)]
pub struct DistantObjects {
    blocks: HashMap<ObjectBlock, Block>,
    sender: Sender<Finished>,
    receiver: Mutex<Receiver<Finished>>,
    /// The loaded squares the blocks were last fitted around.
    hidden_for: HashSet<(i32, i32)>,
}

impl Default for DistantObjects {
    fn default() -> Self {
        let (sender, receiver) = channel();
        DistantObjects {
            blocks: HashMap::new(),
            sender,
            receiver: Mutex::new(receiver),
            hidden_for: HashSet::new(),
        }
    }
}

pub fn stream_distant_objects(
    exterior: Option<Res<Exterior>>,
    game: Res<GameFiles>,
    mut spawner: Spawner,
    mut objects: ResMut<DistantObjects>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut visibility: Query<&mut Visibility>,
) {
    let objects = &mut *objects;
    let Some(exterior) = exterior else {
        // Indoors: whatever was on screen went with the place.
        objects.blocks.clear();
        objects.hidden_for.clear();
        return;
    };
    if exterior.is_added() {
        objects.blocks.clear();
        objects.hidden_for.clear();
    }
    let Some(world) = exterior.grid.world.editor_id.clone() else {
        return;
    };
    // The worldspace's quadtree, once the distant land has read it.
    let Some((lod, terrain)) = exterior.lod_tree() else {
        return;
    };
    let Ok(camera) = cameras.single() else {
        return;
    };
    let player = {
        let p = game_point(camera.translation);
        [p[0], p[1]]
    };
    let wanted: HashSet<ObjectBlock> = world::lod::object_blocks(&lod, &terrain, player)
        .into_iter()
        .collect();

    let finished: Vec<Finished> = objects
        .receiver
        .lock()
        .map(|r| r.try_iter().collect())
        .unwrap_or_default();
    let loaded = exterior.loaded_squares();
    for (key, block) in finished {
        if !objects.blocks.contains_key(&key) {
            continue;
        }
        let state = match block {
            Some(block) if wanted.contains(&key) => {
                let entities = spawner.spawn(&block.scene);
                let tied = block
                    .scene
                    .draws
                    .iter()
                    .map(|d| block.cells.get(d.reference as usize).copied())
                    .collect::<Vec<_>>();
                let parts: Vec<_> = entities.into_iter().zip(tied).collect();
                // Hidden from the start over cells already loaded.
                for (entity, cell) in &parts {
                    if cell.is_some_and(|c| objects.hidden_for.contains(&c)) {
                        spawner.commands.entity(*entity).insert(Visibility::Hidden);
                    }
                }
                Block::Loaded(parts)
            }
            _ => Block::Empty,
        };
        objects.blocks.insert(key, state);
    }

    // Parts over loaded cells hidden, the rest shown.
    if loaded != objects.hidden_for {
        for block in objects.blocks.values() {
            if let Block::Loaded(parts) = block {
                for (entity, cell) in parts {
                    if let Ok(mut v) = visibility.get_mut(*entity) {
                        let hide = cell.is_some_and(|c| loaded.contains(&c));
                        let want = if hide {
                            Visibility::Hidden
                        } else {
                            Visibility::Inherited
                        };
                        if *v != want {
                            *v = want;
                        }
                    }
                }
            }
        }
        objects.hidden_for = loaded;
    }

    // Blocks no longer kept (or switching between their ordinary and high
    // models) dropped.
    let gone: Vec<ObjectBlock> = objects
        .blocks
        .iter()
        .filter(|(b, s)| !matches!(s, Block::Loading) && !wanted.contains(*b))
        .map(|(b, _)| *b)
        .collect();
    for key in gone {
        if let Some(Block::Loaded(parts)) = objects.blocks.remove(&key) {
            for (entity, _) in parts {
                spawner.commands.entity(entity).despawn();
            }
        }
    }

    // Missing ones, nearest first.
    let loading = objects
        .blocks
        .values()
        .filter(|b| matches!(b, Block::Loading))
        .count();
    let mut missing: Vec<ObjectBlock> = wanted
        .iter()
        .filter(|b| !objects.blocks.contains_key(*b))
        .copied()
        .collect();
    missing.sort_by(|a, b| {
        a.node
            .distance(player)
            .partial_cmp(&b.node.distance(player))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.node.cmp(&b.node))
    });
    for key in missing
        .into_iter()
        .take(LOADING_AT_ONCE.saturating_sub(loading))
    {
        objects.blocks.insert(key, Block::Loading);
        let game = Arc::clone(&game.0);
        let sender = objects.sender.clone();
        let world = world.clone();
        std::thread::spawn(move || {
            let _ = sender.send((key, game.lod_object_block(&world, key)));
        });
    }
}
