//! DATA › Local Map in the viewer (`world::local_map`, `preview::local_map`,
//! `ui::pipboy::data`): the place's geometry flagged for the local map is
//! kept as it's put on screen (an interior's, or each outdoor square's with
//! its terrain); while the Pip-Boy is up one tile's picture is made a frame
//! (`0079ffb0` from `0070ee80`), 128 × 128, and uploaded; the fog of war
//! follows the player every frame (`00555c20`); the map's line for the
//! Pip-Boy (pictures, fog, doors, quest targets, the arrow) is made here.
//!
//! Labelled guesses: outdoors the land's top for the camera's height is the
//! highest vertex of the square's geometry (`0053f440` reads the land's
//! own range); the pictures are made again when the place changes (the
//! game makes them each time the Pip-Boy comes up).

use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use cellview::ViewerScene;
use esm::FormId;
use preview::local_map::MapMesh;
use world::local_map::{self as lm, TilePicture};

use crate::dialogue::DialogueState;
use crate::GameFiles;

/// A place's meshes (by reference, 0 for the land) and their top.
type Meshes = (Vec<(u32, MapMesh)>, f32);

/// What the pictures were made for: the place, the player's tile, the
/// geometry's generation.
type MadeFor = (Option<FormId>, Option<FormId>, i32, i32, u64);

/// What the local map draws, by place.
#[derive(Resource, Default)]
pub struct LocalMapGeometry {
    /// The interior on screen: its meshes and its bound's top and bottom.
    interior: Option<Meshes>,
    /// Outdoor squares by grid place: meshes and their top.
    squares: HashMap<(i32, i32), Meshes>,
    /// Bumped whenever the geometry changes.
    generation: u64,
}

/// A place's meshes flagged for the local map, in world units, and the top
/// of what they cover.
fn meshes_of(scene: &ViewerScene, with_terrain: bool) -> Meshes {
    let mut out = Vec::new();
    let mut top = f32::MIN;
    for draw in &scene.draws {
        if draw.actor.is_some() {
            continue;
        }
        let Some(data) = scene.meshes.get(draw.mesh) else {
            continue;
        };
        if !data.local_map {
            continue;
        }
        let m = &draw.transform;
        let point = |p: [f32; 3]| {
            [
                m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
                m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
                m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
            ]
        };
        let dir = |n: [f32; 3]| {
            let v = [
                m[0] * n[0] + m[4] * n[1] + m[8] * n[2],
                m[1] * n[0] + m[5] * n[1] + m[9] * n[2],
                m[2] * n[0] + m[6] * n[1] + m[10] * n[2],
            ];
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-9);
            [v[0] / l, v[1] / l, v[2] / l]
        };
        let positions: Vec<[f32; 3]> = data.positions.iter().map(|&p| point(p)).collect();
        for p in &positions {
            top = top.max(p[2]);
        }
        out.push((
            draw.reference,
            MapMesh {
                positions,
                normals: data.normals.iter().map(|&n| dir(n)).collect(),
                triangles: triangles_of(&data.indices),
            },
        ));
    }
    if with_terrain {
        for q in &scene.terrain {
            for p in &q.positions {
                top = top.max(p[2]);
            }
            out.push((
                0,
                MapMesh {
                    positions: q.positions.clone(),
                    normals: q.normals.clone(),
                    triangles: triangles_of(&q.indices),
                },
            ));
        }
    }
    (out, if top == f32::MIN { 0.0 } else { top })
}

/// Index triples.
fn triangles_of(indices: &[u16]) -> Vec<[u16; 3]> {
    (0..indices.len() / 3)
        .map(|i| [indices[3 * i], indices[3 * i + 1], indices[3 * i + 2]])
        .collect()
}

/// An interior put on screen: its geometry for the local map.
pub fn capture_interior(commands: &mut Commands, scene: &ViewerScene) {
    let (meshes, top) = meshes_of(scene, false);
    commands.queue(move |w: &mut World| {
        let mut g = w.resource_mut::<LocalMapGeometry>();
        g.interior = Some((meshes, top));
        g.squares.clear();
        g.generation += 1;
    });
}

/// An outdoor square put on screen.
pub fn capture_square(commands: &mut Commands, scene: &ViewerScene, here: (i32, i32)) {
    let (meshes, top) = meshes_of(scene, true);
    commands.queue(move |w: &mut World| {
        let mut g = w.resource_mut::<LocalMapGeometry>();
        g.interior = None;
        // Squares long out of reach are let go [the viewer's own keeping].
        g.squares
            .retain(|s, _| (s.0 - here.0).abs().max((s.1 - here.1).abs()) <= 4);
        g.squares.insert(here, (meshes, top));
        g.generation += 1;
    });
}

/// The local map's state: the tiles' pictures made so far.
#[derive(Resource, Default)]
pub struct LocalMap {
    /// What the pictures were made for: the place, the player's tile, the
    /// geometry.
    made_for: Option<MadeFor>,
    key: u64,
    counter: i32,
    pictures: Vec<(i32, i32, String)>,
    /// The pictures by name, for the drawing.
    pub images: HashMap<String, Handle<Image>>,
    /// The line the Pip-Boy shows.
    pub line: Option<ui::pipboy::LocalMapLine>,
    /// The worldspace's grid (its squares' cells), and the doors of the
    /// map's place, worked out when the place changes.
    grid: Option<(FormId, world::WorldGrid)>,
    doors: Vec<lm::DoorMarker>,
    /// The active quest's targets on this map (`0079e0a0`), for which
    /// quest and stage.
    quests: (Option<(FormId, u16)>, Vec<[f32; 3]>),
    graph: world::quest_targets::DoorGraph,
}

pub struct LocalMapPlugin;

impl Plugin for LocalMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalMapGeometry>()
            .init_resource::<LocalMap>()
            .add_systems(
                Update,
                update_local_map
                    // Kept: the Pip-Boy shows the map made this frame; both
                    // at the interface idle (`crate::frame_order::AiSet`, see
                    // `pipboy`).
                    .before(crate::pipboy::update_pipboy)
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .after(crate::frame_order::AiSet::Call(
                        crate::frame_order::INTERFACE_IDLE,
                    )),
            );
    }
}

/// The player's place in the map's frame, the cell's north, and the place.
fn player_place(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
) -> Option<([f32; 3], f32, FormId, bool)> {
    let p = state.player_position?;
    match state.player_world {
        Some(w) => Some((p, 0.0, w, false)),
        None => {
            let cell = state.player_cell?;
            let north = lm::north_of(order, cell);
            Some((lm::rotate_by_north(north, p, true), north, cell, true))
        }
    }
}

/// Every frame: the fog of war follows the player; with the Pip-Boy up,
/// one more tile's picture; the Pip-Boy's line.
#[allow(clippy::too_many_arguments)]
pub fn update_local_map(
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    geometry: Res<LocalMapGeometry>,
    mut map: ResMut<LocalMap>,
    mut images: ResMut<Assets<Image>>,
    pipboy: Option<Res<crate::pipboy::Pipboy>>,
    cameras: Query<&Transform, With<crate::FlyCamera>>,
) {
    let order = &game.0.order;
    let Some((p, north, space, interior)) = player_place(order, &state.0) else {
        return;
    };
    state.0.seen.update(space, interior, p);
    let up = pipboy.as_ref().is_some_and(|p| p.open);
    if !up {
        return;
    }
    let n = game
        .0
        .settings
        .get("General", "uGridsToLoad")
        .and_then(|v| v.trim().parse::<i32>().ok())
        .unwrap_or(lm::GRIDS);
    let tile_px = if interior { 2048.0 } else { 1024.0 };
    let player_tile = (
        (p[0].round() as i32 - if interior { 0x800 } else { 0 }) >> 12,
        (p[1].round() as i32 - if interior { 0x800 } else { 0 }) >> 12,
    );
    let wanted = (
        state.0.player_cell.filter(|_| interior),
        state.0.player_world,
        player_tile.0,
        player_tile.1,
        geometry.generation,
    );
    if map.made_for != Some(wanted) {
        map.made_for = Some(wanted);
        map.key += 1;
        map.counter = 0;
        map.pictures.clear();
        // The doors: the interior's (`0079ffb0` on the first tile), or each
        // square's of the grid with its persistent references.
        let mut doors = Vec::new();
        if interior {
            if let Some(cell) = state.0.player_cell {
                doors = lm::door_markers(order, &state.0, cell);
            }
        } else if let Some(w) = state.0.player_world {
            if map.grid.as_ref().is_none_or(|(id, _)| *id != w) {
                map.grid = world::WorldGrid::load(order, w).ok().map(|g| (w, g));
            }
            if let Some((_, grid)) = &map.grid {
                for dx in -(n >> 1)..=(n >> 1) {
                    for dy in -(n >> 1)..=(n >> 1) {
                        let sq = (player_tile.0 + dx, player_tile.1 + dy);
                        if let Some(c) = grid.cell_at(sq) {
                            doors.extend(lm::door_markers(order, &state.0, c));
                        }
                        let persistent =
                            grid.persistent_in(sq).iter().filter_map(|&r| order.get(r));
                        doors.extend(lm::doors_among(order, &state.0, persistent));
                    }
                }
            }
        }
        map.doors = doors;
    }
    // One tile a frame (`0079ffb0` while `+0x128 < n × n`).
    if map.counter < n * n {
        let tile = lm::tile_picture(p, interior, n, map.counter);
        let picture = make_picture(order, &geometry, &tile, interior, north);
        let name = format!("nvrs:localmap:{}:{}", map.key, map.counter);
        let mut image = Image::new(
            Extent3d {
                width: lm::PICTURE_PIXELS as u32,
                height: lm::PICTURE_PIXELS as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            picture,
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = bevy::image::ImageSampler::linear();
        let handle = images.add(image);
        map.images.insert(name.clone(), handle);
        map.pictures.push((tile.gx, tile.gy, name));
        map.counter += 1;
    }
    // Pictures of earlier places let go.
    let key_prefix = format!("nvrs:localmap:{}:", map.key);
    map.images.retain(|k, _| k.starts_with(&key_prefix));
    let done = map.counter >= n * n;
    let rect = lm::map_rect(p, interior, n);
    let to_map = |w: [f32; 3]| {
        let q = if interior {
            lm::rotate_by_north(north, w, true)
        } else {
            w
        };
        lm::world_to_map(&rect, q)
    };
    // The fog at each tile's 17 × 17 corners (`00556870`).
    let seen = &state.0.seen;
    let tiles: Vec<ui::pipboy::LocalTile> = map
        .pictures
        .iter()
        .map(|(gx, gy, name)| {
            let t = lm::tile_picture(p, interior, n, gy * n + gx);
            let mut fog = Vec::with_capacity(289);
            for j in 0..17 {
                for i in 0..17 {
                    let c = seen.count(
                        space,
                        interior,
                        [
                            t.origin[0] + i as f32 * lm::SEEN_STEP,
                            t.origin[1] + j as f32 * lm::SEEN_STEP,
                        ],
                    );
                    fog.push(c as f32 / 4.0);
                }
            }
            ui::pipboy::LocalTile {
                gx: *gx,
                gy: *gy,
                picture: name.clone(),
                fog,
            }
        })
        .collect();
    let mut doors = Vec::new();
    for d in &map.doors {
        let at = to_map(d.position);
        if !(0.0..=1.0).contains(&at[0]) || !(0.0..=1.0).contains(&at[1]) {
            continue;
        }
        let q = if interior {
            lm::rotate_by_north(north, d.position, true)
        } else {
            d.position
        };
        let alpha = 255.0 * seen.count(space, interior, [q[0], q[1]]) as f32 / 4.0;
        if alpha <= 0.0 {
            continue;
        }
        doors.push(ui::pipboy::LocalDoor {
            at,
            name: d.name.clone(),
            alpha,
        });
    } // The heading: the view's (radians clockwise from north).
    let heading = cameras.single().map_or(0.0, |t| {
        let f = t.forward().as_vec3();
        f.x.atan2(-f.z)
    });
    let player = Some((lm::world_to_map(&rect, p), lm::arrow_angle(heading, north)));
    // The active quest's targets (`0079e0a0`, local map branch): one in the
    // player's place at itself; one elsewhere at the first door of the way
    // there when that door is in the player's place [guess: the game's
    // "display ref" (`005cbb70`) read as that door].
    // Translated from 0079e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
    let active = state.0.active_quest.map(|q| {
        let stage = state.0.stages.get(&q).copied().unwrap_or(0);
        (q, stage)
    });
    if map.quests.0 != active || map.counter == 1 {
        let mut points = Vec::new();
        if let Some(quest) = active.and_then(|(q, _)| world::quest::Quest::load(order, q)) {
            let here = if interior {
                state.0.player_cell
            } else {
                state.0.player_world
            };
            let player = state.0.place(order, world::dialogue::PLAYER_REF);
            for t in world::quest_targets::current_targets(order, &quest, &state.0) {
                let Some((s, _, at, _)) = state.0.place(order, t.reference) else {
                    continue;
                };
                if Some(s) == here {
                    points.push(at);
                    continue;
                }
                let Some((from_space, _, from, _)) = player else {
                    continue;
                };
                let path = world::quest_targets::door_path(
                    order,
                    &state.0,
                    &mut map.graph,
                    from_space,
                    from,
                    s,
                )
                .unwrap_or_default();
                if let Some(w) = path
                    .first()
                    .and_then(|&d| world::scripting::whereabouts(order, d))
                {
                    if (if interior { Some(w.cell) } else { w.world }) == here {
                        points.push(w.position);
                    }
                }
            }
        }
        map.quests = (active, points);
    }
    let quests: Vec<[f32; 2]> = map.quests.1.iter().map(|&q| to_map(q)).collect();
    let custom = state
        .0
        .custom_marker
        .filter(|m| {
            Some(m.space)
                == state
                    .0
                    .player_cell
                    .filter(|_| interior)
                    .or(state.0.player_world)
        })
        .map(|m| to_map(m.position));
    map.line = Some(ui::pipboy::LocalMapLine {
        key: map.key,
        grids: n,
        tile_px,
        tiles,
        done,
        player,
        doors,
        quests,
        custom,
    });
}

/// One tile's picture from the geometry kept: the place's objects except
/// those flagged hidden from the local map (reference flag 0x00800000,
/// `00477ba0`), turned into the map's frame indoors.
fn make_picture(
    order: &esm::LoadOrder,
    geometry: &LocalMapGeometry,
    tile: &TilePicture,
    interior: bool,
    north: f32,
) -> Vec<u8> {
    let shown = |r: u32| {
        r == 0
            || order
                .get(FormId(r))
                .is_none_or(|rr| rr.entry.header.flags & 0x0080_0000 == 0)
    };
    let blank = || vec![0; lm::PICTURE_PIXELS * lm::PICTURE_PIXELS * 4];
    if interior {
        let Some((meshes, top)) = &geometry.interior else {
            return blank();
        };
        let turned: Vec<MapMesh> = meshes
            .iter()
            .filter(|(r, _)| shown(*r))
            .map(|(_, m)| MapMesh {
                positions: m
                    .positions
                    .iter()
                    .map(|&q| lm::rotate_by_north(north, q, true))
                    .collect(),
                normals: m
                    .normals
                    .iter()
                    .map(|&q| lm::rotate_by_north(north, q, true))
                    .collect(),
                triangles: m.triangles.clone(),
            })
            .collect();
        preview::local_map::picture(&turned, tile, *top, lm::INTERIOR_LIFT)
    } else {
        let key = (
            (tile.origin[0] / lm::TILE).floor() as i32,
            (tile.origin[1] / lm::TILE).floor() as i32,
        );
        match geometry.squares.get(&key) {
            Some((meshes, top)) => {
                let kept: Vec<MapMesh> = meshes
                    .iter()
                    .filter(|(r, _)| shown(*r))
                    .map(|(_, m)| m.clone())
                    .collect();
                preview::local_map::picture(&kept, tile, *top, lm::EXTERIOR_LIFT)
            }
            None => blank(),
        }
    }
}
