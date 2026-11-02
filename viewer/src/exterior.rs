//! The outdoors: the squares of a worldspace around the player, loaded in
//! the background as they walk and dropped once they're far behind, as the
//! game does with `uGridsToLoad` (5 in the game's `Fallout.ini`: the
//! player's square and two on every side).

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use cellview::{space, DoorData, Game, LodChunk, ViewerScene, EYE_HEIGHT};
use physics::Collider;
use world::lod::{LodNode, LodSettings, TerrainSettings};
use world::{square_of, WorldGrid};

use crate::grade::ImageSpaceGrade;
use crate::lod::{self, choose_shown, high_detail, ChunkState, LodLandParams, SpawnedChunk};
use crate::walk::{game_point, CellCollision, Doors, Player};
use crate::{FlyCamera, GameFiles, Grading, SceneEntity, Spawner};

/// Squares loaded on every side of the player's: `uGridsToLoad` 5 → 2.
pub const LOAD_RADIUS_DEFAULT: i32 = 2;

/// Worlds too dense for the graphics to hold 25 squares at once (Dead
/// Money's Residential District lost the device on a laptop GPU within
/// seconds): they load one square on every side.
const DENSE_WORLDS: [&str; 1] = ["NVDLC01VillaDean"];

static RADIUS_NOW: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

/// A world is entered: its load radius.
fn choose_load_radius(world: &str) {
    let radius = if DENSE_WORLDS.iter().any(|w| w.eq_ignore_ascii_case(world)) {
        1
    } else {
        LOAD_RADIUS_DEFAULT
    };
    RADIUS_NOW.store(radius, std::sync::atomic::Ordering::Relaxed);
}

/// Squares loaded on every side of the player's now: the world's own
/// ([`DENSE_WORLDS`]), or `NV_LOAD_RADIUS` (0 to 4) when set.
pub fn load_radius() -> i32 {
    if let Some(r) = std::env::var("NV_LOAD_RADIUS")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
    {
        return r.clamp(0, 4);
    }
    match RADIUS_NOW.load(std::sync::atomic::Ordering::Relaxed) {
        r if r >= 0 => r,
        _ => LOAD_RADIUS_DEFAULT,
    }
}
pub fn keep_radius() -> i32 {
    load_radius() + 1
}
/// Squares loading at once.
const LOADING_AT_ONCE: usize = 3;

/// Where to go outdoors: a worldspace and a spot in it.
pub struct ExteriorStart {
    pub grid: WorldGrid,
    /// Feet position in game units; `None` for the height means "on the
    /// ground" (found once the square has loaded).
    pub feet: [f32; 2],
    pub height: Option<f32>,
    /// Radians clockwise from north.
    pub heading: f32,
}

/// An outdoor place waiting to be entered.
#[derive(Resource, Default)]
pub struct PendingExterior(pub Option<ExteriorStart>);

enum Square {
    Loading,
    /// Nothing there (no cell), or it couldn't be loaded.
    Empty,
    Loaded {
        /// What it put on screen, and its own placed lights (shared with
        /// its neighbours: `cellview::lights_reaching`).
        spawned: Box<crate::Spawned>,
        lights: Vec<cellview::LightData>,
        collision: Box<Collider>,
        object_bounds: Vec<cellview::ObjectBounds>,
        doors: Vec<DoorData>,
        swing_doors: Vec<cellview::SwingDoor>,
        talkers: Vec<crate::dialogue::Talker>,
        /// People it left out as disabled when it loaded
        /// (`world::ai::disabled_people_in_square`), who come in once a
        /// script enables them (`bring_in_people`).
        disabled_people: Vec<esm::FormId>,
    },
}

type Finished = (
    (i32, i32),
    Result<Option<ViewerScene>, String>,
    Vec<esm::FormId>,
);

/// Distant water (`water.rs`) is drawn from the level-4 chunks out to this
/// many cells from the player: the game's `uGridDistantCount` (20 in
/// `Fallout_default.ini`). The distant land itself follows the game's
/// quadtree (`world::lod`).
pub const DISTANT_CELLS: i32 = 20;
/// Distant-land chunks loading at once.
const CHUNKS_LOADING_AT_ONCE: usize = 4;

enum Chunk {
    Loading,
    Empty,
    Loaded {
        spawned: SpawnedChunk,
        /// Fading from its parent's texture: since when (seconds).
        fading_since: Option<f32>,
        shown: bool,
    },
}

type FinishedChunk = (LodNode, Option<LodChunk>);

/// A chunk's texture and normal map, lent to its quarters while they fade.
type ChunkTextures = (Option<Handle<Image>>, Option<Handle<Image>>);

/// A worldspace's distant-land quadtree and the terrain manager's settings
/// (`None` inside: no `.dlodsettings`, no distant land).
type LodTree = Option<(LodSettings, TerrainSettings)>;

/// The worldspace being walked, and its squares.
#[derive(Resource)]
pub struct Exterior {
    pub grid: Arc<WorldGrid>,
    squares: HashMap<(i32, i32), Square>,
    sender: Sender<Finished>,
    receiver: Mutex<Receiver<Finished>>,
    /// Waiting for the ground under the player: the spot, and the height if
    /// known.
    landing: Option<([f32; 2], Option<f32>)>,
    /// The first square's sky and grading have been applied.
    lit: bool,
    /// Distant land: the worldspace's quadtree (read once), its chunks by
    /// node, what was shown last frame, whether a chunk to show is still
    /// loading, the light it's drawn with (from the first square), the
    /// square it was last fitted around, and the noise texture.
    lod_tree: Option<LodTree>,
    chunks: HashMap<LodNode, Chunk>,
    lod_shown: HashSet<LodNode>,
    lod_pending: bool,
    chunk_sender: Sender<FinishedChunk>,
    chunk_receiver: Mutex<Receiver<FinishedChunk>>,
    pub(crate) lod_params: Option<LodLandParams>,
    lod_here: Option<(i32, i32)>,
    noise: Option<Option<Handle<Image>>>,
    /// The weather (from the first square), whose colours the light
    /// follows through the day (`daylight`).
    pub(crate) weather: Option<esm::FormId>,
    /// The square the terrain's blend toward the distant land was last
    /// measured from.
    blend_here: Option<(i32, i32)>,
}

impl Exterior {
    /// The squares on screen in full.
    pub fn loaded_squares(&self) -> std::collections::HashSet<(i32, i32)> {
        self.squares
            .iter()
            .filter(|(_, s)| matches!(s, Square::Loaded { .. }))
            .map(|(at, _)| *at)
            .collect()
    }

    /// The people the loaded squares left out as disabled when they loaded.
    pub fn disabled_people(&self) -> Vec<esm::FormId> {
        self.squares
            .values()
            .filter_map(|s| match s {
                Square::Loaded {
                    disabled_people, ..
                } => Some(disabled_people.iter().copied()),
                _ => None,
            })
            .flatten()
            .collect()
    }

    /// The worldspace's distant-land quadtree and the terrain manager's
    /// settings, once read (`None` until then, or with no distant land).
    pub fn lod_tree(&self) -> Option<(LodSettings, TerrainSettings)> {
        self.lod_tree.flatten()
    }

    /// The square the player is in (or is arriving in).
    pub fn here(&self, camera: &Transform) -> (i32, i32) {
        match self.landing {
            Some((spot, _)) => square_of([spot[0], spot[1], 0.0]),
            None => square_of(game_point(camera.translation)),
        }
    }
}

/// Leaving the outdoors (going into an interior) drops every square.
pub fn leave(commands: &mut Commands) {
    commands.remove_resource::<Exterior>();
}

/// Enters an outdoor place: clears what was shown, puts the camera at the
/// start, and lets [`stream_squares`] load the squares around it.
pub fn enter_exterior(
    mut commands: Commands,
    mut pending: ResMut<PendingExterior>,
    old: Query<Entity, With<SceneEntity>>,
    mut player: ResMut<Player>,
    mut cameras: Query<(&mut Transform, &mut FlyCamera)>,
    mut windows: Query<&mut Window>,
    mut start_pitch: ResMut<crate::StartPitch>,
) {
    let Some(start) = pending.0.take() else {
        return;
    };
    // The starting pitch (`--at`), the first place only.
    let pitch = std::mem::take(&mut start_pitch.0);
    for entity in &old {
        commands.entity(entity).despawn();
    }
    let name = start.grid.world.label();
    for mut window in &mut windows {
        window.title = format!("nv-rs viewer - {name}");
    }
    let [x, y] = start.feet;
    // Until the ground is found, the eye waits high above the spot.
    let z = start.height.unwrap_or(0.0);
    let eye = Vec3::from(space::point([x, y, z + EYE_HEIGHT]));
    let yaw = space::heading_to_yaw(start.heading);
    for (mut transform, mut fly) in &mut cameras {
        *transform = Transform::from_translation(eye).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            yaw,
            pitch,
            0.0,
        ));
        fly.yaw = yaw;
        fly.pitch = pitch;
        fly.start = (eye, yaw);
    }
    player.arrive([x, y, z]);
    player.ready = false;
    commands.insert_resource(CellCollision(Collider::new()));
    commands.remove_resource::<crate::scripts::ObjectBounds>();
    commands.insert_resource(Doors(Vec::new()));
    let (sender, receiver) = channel();
    let (chunk_sender, chunk_receiver) = channel();
    choose_load_radius(&name);
    println!(
        "Outdoors in {name}: loading the squares around {:.0}, {:.0} ...",
        x, y
    );
    commands.insert_resource(Exterior {
        grid: Arc::new(start.grid),
        squares: HashMap::new(),
        sender,
        receiver: Mutex::new(receiver),
        landing: Some((start.feet, start.height)),
        lit: false,
        lod_tree: None,
        chunks: HashMap::new(),
        lod_shown: HashSet::new(),
        lod_pending: false,
        chunk_sender,
        chunk_receiver: Mutex::new(chunk_receiver),
        lod_params: None,
        lod_here: None,
        noise: None,
        weather: None,
        blend_here: None,
    });
}

/// Loads the squares around the player in the background, puts finished
/// ones on screen, drops far ones, and keeps the collision and doors to
/// the loaded squares.
#[allow(clippy::too_many_arguments)]
pub fn stream_squares(
    mut commands: Commands,
    exterior: Option<ResMut<Exterior>>,
    game: Res<GameFiles>,
    mut spawner: Spawner,
    mut player: ResMut<Player>,
    mut grading: ResMut<Grading>,
    mut clear: ResMut<ClearColor>,
    mut cameras: Query<(&mut Transform, &mut FlyCamera, &mut ImageSpaceGrade)>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    mut swing_doors: ResMut<crate::doors::SwingDoors>,
    shown: Query<&crate::ai::Walker>,
    (old_talkers, brought): (Option<Res<crate::dialogue::Talkers>>, Res<crate::BroughtIn>),
) {
    let Some(mut exterior) = exterior else {
        return;
    };
    let exterior = &mut *exterior;
    let Ok((mut transform, mut fly, mut grade)) = cameras.single_mut() else {
        return;
    };
    let here = match exterior.landing {
        Some((spot, _)) => square_of([spot[0], spot[1], 0.0]),
        None => square_of(game_point(transform.translation)),
    };

    // Finished squares onto the screen.
    let mut changed = false;
    let finished: Vec<Finished> = exterior
        .receiver
        .lock()
        .map(|r| r.try_iter().collect())
        .unwrap_or_default();
    for (square, result, disabled_people) in finished {
        let far = (square.0 - here.0).abs().max((square.1 - here.1).abs()) > keep_radius();
        let state = match result {
            Ok(Some(scene)) if !far => {
                if !exterior.lit {
                    exterior.lit = true;
                    // The game's weather by now (`crate::weather`), else the
                    // square's likeliest.
                    exterior.weather = state.0.weather.current.or(scene.weather);
                    grading.grade =
                        ImageSpaceGrade::from_cell(scene.grade.as_ref(), scene.hdr.as_ref());
                    *grade = if grading.on {
                        grading.grade
                    } else {
                        grading.grade.without_cinematic()
                    };
                    if let Some(sky) = scene.sky {
                        // Behind the dome's open bottom: the fog's stored
                        // colour, as the game clears the picture (recorded
                        // at Goodsprings; the lighting writes stored values
                        // and the image space pass decodes them, so the
                        // clear colour is given the same way). `daylight`
                        // keeps it to the hour's.
                        let [r, g, b] = scene.fog.as_ref().map_or(sky[1], |f| f.color);
                        clear.0 = Color::linear_rgb(r, g, b);
                        match game.0.sky_dome(sky) {
                            Some(dome) => {
                                spawner.spawn_sky(&dome);
                            }
                            None => println!(
                                "  couldn't read the sky dome ({})",
                                cellview::game::SKY_DOME
                            ),
                        }
                        // The sun, then the weather's clouds over it.
                        let sun = exterior
                            .grid
                            .climate
                            .as_ref()
                            .zip(exterior.weather)
                            .and_then(|(c, w)| game.0.sun(c, w));
                        if let Some(stars) =
                            exterior.grid.climate.as_ref().and_then(|c| game.0.stars(c))
                        {
                            spawner.spawn_stars(&stars);
                        }
                        if let Some(sun) = &sun {
                            spawner.spawn_sun(sun);
                        }
                        let layers = exterior
                            .weather
                            .map(|w| game.0.clouds(w))
                            .unwrap_or_default();
                        println!("  {} cloud layers", layers.len());
                        for layer in &layers {
                            spawner.spawn_clouds(layer);
                        }
                    }
                    for note in &scene.notes {
                        println!("  {note}");
                    }
                    exterior.lod_params = Some(crate::lod_params(&scene, spawner.brightness()));
                }
                // The terrain blends toward the distant land, with its
                // noise (`lod_land.wgsl`'s).
                let noise = exterior
                    .noise
                    .get_or_insert_with(|| game.0.lod_noise().and_then(|t| spawner.upload(&t)))
                    .clone();
                let spawned = Box::new(spawner.spawn_square(&scene, here, noise));
                crate::local_map::capture_square(&mut spawner.commands, &scene, square);
                Square::Loaded {
                    spawned,
                    lights: scene.lights.clone(),
                    object_bounds: scene.object_bounds.clone(),
                    talkers: scene
                        .actors
                        .iter()
                        .map(crate::dialogue::Talker::from_actor)
                        .collect(),
                    collision: Box::new(scene.collision),
                    doors: scene.doors,
                    swing_doors: scene.swing_doors,
                    disabled_people,
                }
            }
            Ok(_) => Square::Empty,
            Err(e) => {
                println!("  couldn't load square {},{}: {e}", square.0, square.1);
                Square::Empty
            }
        };
        exterior.squares.insert(square, state);
        changed = true;
    }

    // The player moved into another square: every quarter's blend toward
    // the distant land is measured from its middle now.
    if exterior.blend_here != Some(here) {
        exterior.blend_here = Some(here);
        for state in exterior.squares.values() {
            if let Square::Loaded { spawned, .. } = state {
                for m in &spawned.terrain {
                    if let Some(m) = spawner.terrain_materials.get_mut(m) {
                        m.extension.land_blend.move_to(here);
                    }
                }
            }
        }
    }

    // Far squares off the screen.
    let far: Vec<(i32, i32)> = exterior
        .squares
        .iter()
        .filter(|(s, state)| {
            !matches!(state, Square::Loading)
                && ((s.0 - here.0).abs().max((s.1 - here.1).abs()) > keep_radius())
        })
        .map(|(s, _)| *s)
        .collect();
    for square in far {
        if let Some(Square::Loaded { spawned, .. }) = exterior.squares.remove(&square) {
            for entity in spawned.entities {
                commands.entity(entity).despawn();
            }
        }
        changed = true;
    }

    // Missing squares, nearest first.
    let loading = exterior
        .squares
        .values()
        .filter(|s| matches!(s, Square::Loading))
        .count();
    let mut wanted: Vec<(i32, i32)> = (-load_radius()..=load_radius())
        .flat_map(|dx| (-load_radius()..=load_radius()).map(move |dy| (here.0 + dx, here.1 + dy)))
        .filter(|s| !exterior.squares.contains_key(s))
        .collect();
    wanted.sort_by_key(|s| (s.0 - here.0).abs().max((s.1 - here.1).abs()));
    for square in wanted
        .into_iter()
        .take(LOADING_AT_ONCE.saturating_sub(loading))
    {
        exterior.squares.insert(square, Square::Loading);
        let game = Arc::clone(&game.0);
        let grid = Arc::clone(&exterior.grid);
        let sender = exterior.sender.clone();
        // What scripts have enabled and disabled, as it is now; people
        // already on screen (brought in after their square loaded,
        // `bring_in_people`) are not drawn a second time.
        let mut disabled = state.0.disabled.clone();
        disabled.extend(shown.iter().map(|w| (w.reference, true)));
        std::thread::spawn(move || {
            let result = load_square(&game, &grid, square, &disabled);
            let people =
                world::ai::disabled_people_in_square(&game.order, &grid, square, &disabled);
            let _ = sender.send((square, result, people));
        });
    }

    if changed {
        let mut collision = Collider::new();
        let mut doors = Vec::new();
        let mut talkers = Vec::new();
        let mut swing = Vec::new();
        let mut object_bounds = crate::scripts::ObjectBounds::default();
        for state in exterior.squares.values() {
            if let Square::Loaded {
                collision: c,
                object_bounds: bounds,
                doors: d,
                swing_doors: s,
                talkers: t,
                ..
            } = state
            {
                collision.extend(c);
                doors.extend(d.iter().cloned());
                swing.extend(s.iter());
                talkers.extend(t.iter().copied());
                object_bounds
                    .0
                    .extend(bounds.iter().map(|b| (b.reference, (b.lo, b.hi))));
            }
        }
        // People brought in after their place loaded (`bring_in_people`)
        // stay, as long as they're on screen; so do those moved in from
        // elsewhere: a dialogue package starting just after a square loaded
        // found Sunny Smiles (moved to the first well for VCG02) missing and
        // talked to no one.
        if let Some(old) = &old_talkers {
            let on_screen: HashSet<esm::FormId> = shown.iter().map(|w| w.reference).collect();
            talkers = with_brought_in(talkers, &old.0, &brought.people, &on_screen);
            let moved = world::ai::moved_into(&game.0.order, &state.0, exterior.grid.world.form_id);
            for t in &old.0 {
                if moved.contains(&t.reference)
                    && !talkers.iter().any(|k| k.reference == t.reference)
                {
                    talkers.push(*t);
                }
            }
        }
        commands.insert_resource(crate::dialogue::Talkers(talkers));
        // The loaded squares' doors that swing (their leaves are put where
        // their doors have them once the collider is in place).
        swing_doors.read_settings(&game.0.settings);
        swing_doors.replace(swing.into_iter(), &mut state.0);
        // Every square takes the loaded lights that reach it, its
        // neighbours' too.
        let all: Vec<cellview::LightData> = exterior
            .squares
            .values()
            .filter_map(|s| match s {
                Square::Loaded { lights, .. } => Some(lights.iter().copied()),
                _ => None,
            })
            .flatten()
            .collect();
        for (&at, state) in &exterior.squares {
            if let Square::Loaded { spawned, .. } = state {
                let lights = cellview::lights_reaching(at, &all, crate::lighting::MAX_LIGHTS);
                spawner.relight(spawned, &lights);
            }
        }
        // Once the square under the player is there, stand them on it.
        if let Some((spot, height)) = exterior.landing {
            if exterior
                .squares
                .get(&here)
                .is_some_and(|s| !matches!(s, Square::Loading))
            {
                exterior.landing = None;
                let z = height.unwrap_or_else(|| ground_under(&collision, spot).unwrap_or(0.0));
                let [x, y] = spot;
                let eye = Vec3::from(space::point([x, y, z + EYE_HEIGHT]));
                transform.translation = eye;
                fly.start = (eye, fly.yaw);
                player.arrive([x, y, z]);
                player.ready = true;
            }
        }
        commands.insert_resource(CellCollision(collision));
        commands.insert_resource(object_bounds);
        commands.insert_resource(Doors(doors));
    }
}

/// Distant land: the chunks of the game's quadtree for where the player
/// stands (`world::lod::drawn_nodes`: level 4 near, coarser ones farther,
/// as far as the worldspace goes), loaded in the background, shown as the
/// game shows them (`lod::choose_shown`), lowered under the loaded squares.
/// The levels' distances are measured from the player's position (here
/// the camera's: the viewer's eye stands over the feet).
pub fn stream_distant_land(
    exterior: Option<ResMut<Exterior>>,
    game: Res<GameFiles>,
    time: Res<Time>,
    mut spawner: Spawner,
    cameras: Query<&Transform, With<FlyCamera>>,
) {
    let Some(mut exterior) = exterior else {
        return;
    };
    let exterior = &mut *exterior;
    let Some(params) = exterior.lod_params else {
        return;
    };
    let Some(world) = exterior.grid.world.editor_id.clone() else {
        return;
    };
    let Ok(camera) = cameras.single() else {
        return;
    };
    let tree = *exterior.lod_tree.get_or_insert_with(|| {
        let tree = game
            .0
            .lod_settings(&world)
            .map(|s| (s, game.0.terrain_settings()));
        if tree.is_none() {
            println!("  no distant land for {world} (no .dlodsettings)");
        }
        tree
    });
    let Some((lod, terrain)) = tree else {
        exterior.lod_pending = false;
        exterior.lod_here.get_or_insert((0, 0));
        return;
    };
    let player = match exterior.landing {
        Some((spot, _)) => spot,
        None => {
            let p = game_point(camera.translation);
            [p[0], p[1]]
        }
    };
    let here = square_of([player[0], player[1], 0.0]);
    let noise = exterior
        .noise
        .get_or_insert_with(|| game.0.lod_noise().and_then(|t| spawner.upload(&t)))
        .clone();
    let detail = high_detail(here, load_radius());
    let now = time.elapsed_secs();

    // What the game draws from here, and the coarsest level's chunks it
    // keeps loaded besides (`bKeepLowDetailTerrain`).
    let drawn = world::lod::drawn_nodes(&lod, &terrain, player, here);
    let morphs: HashMap<LodNode, f32> = drawn.iter().map(|d| (d.node, d.morph)).collect();
    let wanted: HashSet<LodNode> = morphs.keys().copied().collect();
    let kept: HashSet<LodNode> = if terrain.keep_low_detail {
        world::lod::nodes_at(&lod, lod.max_level)
            .into_iter()
            .collect()
    } else {
        HashSet::new()
    };

    let finished: Vec<FinishedChunk> = exterior
        .chunk_receiver
        .lock()
        .map(|r| r.try_iter().collect())
        .unwrap_or_default();
    for (node, chunk) in finished {
        let state = match chunk {
            Some(chunk) => Chunk::Loaded {
                spawned: spawner.spawn_chunk(
                    &chunk,
                    LodLandParams {
                        high_detail: detail,
                        ..params
                    },
                    noise.clone(),
                ),
                fading_since: None,
                shown: false,
            },
            None => Chunk::Empty,
        };
        exterior.chunks.insert(node, state);
    }

    // The loaded squares moved: lower the land under the new ones.
    if exterior.lod_here != Some(here) {
        exterior.lod_here = Some(here);
        for chunk in exterior.chunks.values() {
            if let Chunk::Loaded { spawned, .. } = chunk {
                if let Some(m) = spawner.lod_materials.get_mut(&spawned.material) {
                    lod::set_high_detail(m, detail);
                }
            }
        }
    }

    // What shows, and the quarters that replace their parents now.
    let state_of = |chunks: &HashMap<LodNode, Chunk>, n: LodNode| match chunks.get(&n) {
        Some(Chunk::Loaded { .. }) => ChunkState::Loaded,
        Some(Chunk::Empty) => ChunkState::Empty,
        _ => ChunkState::Pending,
    };
    let showing = choose_shown(
        &lod,
        &wanted,
        |n| state_of(&exterior.chunks, n),
        &exterior.lod_shown,
    );
    exterior.lod_pending = wanted
        .iter()
        .any(|n| state_of(&exterior.chunks, *n) == ChunkState::Pending);
    let parents: HashMap<LodNode, ChunkTextures> = showing
        .fades
        .iter()
        .filter_map(|(_, p)| match exterior.chunks.get(p) {
            Some(Chunk::Loaded { spawned, .. }) => {
                Some((*p, (spawned.base.clone(), spawned.normals.clone())))
            }
            _ => None,
        })
        .collect();
    for &(node, parent) in &showing.fades {
        let (
            Some(Chunk::Loaded {
                spawned,
                fading_since,
                ..
            }),
            Some((base, normals)),
        ) = (exterior.chunks.get_mut(&node), parents.get(&parent))
        else {
            continue;
        };
        if let Some(m) = spawner.lod_materials.get_mut(&spawned.material) {
            let [u, v] = node.offset_in_parent(parent);
            lod::set_parent_textures(m, [u, v], base.clone(), normals.clone());
            *fading_since = Some(now);
        }
    }
    for (node, chunk) in exterior.chunks.iter_mut() {
        let Chunk::Loaded {
            spawned,
            fading_since,
            shown,
        } = chunk
        else {
            continue;
        };
        let show = showing.shown.contains(node);
        if show != *shown {
            *shown = show;
            // (Through commands: a chunk spawned this frame isn't in the
            // world yet.)
            spawner.commands.entity(spawned.entity).insert(if show {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
        if !show {
            continue;
        }
        // The fade from the parent's texture, then the geomorph factor.
        let fade =
            fading_since.map(|since| world::lod::texture_fade((now - since) * 1000.0, &terrain));
        let morph = morphs.get(node).copied().unwrap_or(1.0);
        let Some(m) = spawner.lod_materials.get(&spawned.material) else {
            continue;
        };
        let mut chunk_values = lod::chunk_values(m);
        chunk_values.x = morph;
        let mut done = false;
        match fade {
            Some(Some(w)) => chunk_values.w = w,
            Some(None) => {
                chunk_values.w = 1.0;
                done = true;
            }
            None => {}
        }
        if chunk_values != lod::chunk_values(m) || done {
            if let Some(m) = spawner.lod_materials.get_mut(&spawned.material) {
                lod::set_chunk_values(m, chunk_values);
                if done {
                    lod::clear_parent_textures(m);
                }
            }
        }
        if done {
            *fading_since = None;
        }
    }
    exterior.lod_shown = showing.shown;

    // Chunks not wanted, not on screen and not kept go.
    let gone: Vec<LodNode> = exterior
        .chunks
        .iter()
        .filter(|(n, c)| {
            !matches!(c, Chunk::Loading)
                && !wanted.contains(*n)
                && !kept.contains(*n)
                && !exterior.lod_shown.contains(*n)
        })
        .map(|(n, _)| *n)
        .collect();
    for node in gone {
        if let Some(Chunk::Loaded { spawned, .. }) = exterior.chunks.remove(&node) {
            spawner.commands.entity(spawned.entity).despawn();
        }
    }

    // Missing ones, those to draw first, nearest first.
    let loading = exterior
        .chunks
        .values()
        .filter(|c| matches!(c, Chunk::Loading))
        .count();
    let mut missing: Vec<LodNode> = wanted
        .iter()
        .chain(kept.iter())
        .filter(|n| !exterior.chunks.contains_key(*n))
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    missing.sort_by(|a, b| {
        let key = |n: &LodNode| (!wanted.contains(n), n.distance(player));
        key(a)
            .partial_cmp(&key(b))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.cmp(b))
    });
    for node in missing
        .into_iter()
        .take(CHUNKS_LOADING_AT_ONCE.saturating_sub(loading))
    {
        exterior.chunks.insert(node, Chunk::Loading);
        let game = Arc::clone(&game.0);
        let sender = exterior.chunk_sender.clone();
        let world = world.clone();
        std::thread::spawn(move || {
            let _ = sender.send((node, game.lod_land(&world, node)));
        });
    }
}

/// The loaded squares' people to talk to (`squares`), plus those from
/// the `old` list that were brought in after their place loaded and are
/// still on screen (they belong to no square).
pub(crate) fn with_brought_in(
    mut squares: Vec<crate::dialogue::Talker>,
    old: &[crate::dialogue::Talker],
    brought: &HashSet<esm::FormId>,
    on_screen: &HashSet<esm::FormId>,
) -> Vec<crate::dialogue::Talker> {
    let ours: HashSet<esm::FormId> = squares.iter().map(|t| t.reference).collect();
    squares.extend(old.iter().copied().filter(|t| {
        brought.contains(&t.reference)
            && on_screen.contains(&t.reference)
            && !ours.contains(&t.reference)
    }));
    squares
}

fn load_square(
    game: &Game,
    grid: &WorldGrid,
    square: (i32, i32),
    disabled: &world::Disabled,
) -> Result<Option<ViewerScene>, String> {
    game.load_square_now(grid, square, disabled)
        .map_err(|e| e.0)
}

/// The height of the highest surface under a spot.
pub fn ground_under(collision: &Collider, spot: [f32; 2]) -> Option<f32> {
    const TOP: f32 = 100_000.0;
    collision
        .raycast([spot[0], spot[1], TOP], [0.0, 0.0, -1.0], 2.0 * TOP)
        .map(|(d, _)| TOP - d)
}

/// Where a door to the outdoors leads, as a start.
pub fn door_start(game: &Game, door: &DoorData) -> Result<ExteriorStart, String> {
    let world = door
        .world
        .ok_or_else(|| "the door's far side isn't in a worldspace".to_string())?;
    let grid = WorldGrid::load(&game.order, esm::FormId(world)).map_err(|e| e.to_string())?;
    Ok(ExteriorStart {
        grid,
        feet: [door.arrive[0], door.arrive[1]],
        height: Some(door.arrive[2]),
        heading: door.arrive_heading,
    })
}

impl Exterior {
    /// Squares are still loading near the player, or the player hasn't
    /// landed yet.
    pub fn busy(&self) -> bool {
        let distant_loading = self.lod_here.is_none() || self.lod_pending;
        if self.grid.world.editor_id.is_some() && distant_loading {
            return true;
        }
        self.landing.is_some() || self.squares.values().any(|s| matches!(s, Square::Loading))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_ground_under_a_spot() {
        let mut c = Collider::new();
        c.add(
            &[
                [0.0, 0.0, 8000.0],
                [500.0, 0.0, 8000.0],
                [0.0, 500.0, 8100.0],
            ],
            &[[0, 1, 2]],
        );
        let z = ground_under(&c, [100.0, 100.0]).unwrap();
        // The triangle rises 100 units over 500 northward.
        assert!((z - 8020.0).abs() < 0.05, "{z}");
        assert_eq!(ground_under(&c, [-100.0, -100.0]), None);
    }

    #[test]
    fn people_brought_in_stay_talkers_when_squares_change() {
        // Ghost Town Gunfight: Ringo (00104C7D) is moved into the
        // worldspace by a script; after the squares change he must still
        // be someone to talk to (his after-the-fight dialogue package).
        use crate::dialogue::Talker;
        let t = |r: u32| Talker {
            reference: esm::FormId(r),
            base: esm::FormId(r + 1),
            position: [r as f32, 0.0, 0.0],
        };
        let set = |rs: &[u32]| rs.iter().map(|&r| esm::FormId(r)).collect::<HashSet<_>>();
        let squares = vec![t(10), t(20)];
        let old = [t(10), t(30), t(40), t(50)];
        // 30 brought in and on screen: kept with its last position; 40
        // brought in but gone; 50 a square's person whose square left.
        let out = with_brought_in(squares, &old, &set(&[30, 40]), &set(&[10, 20, 30, 50]));
        let refs: Vec<u32> = out.iter().map(|t| t.reference.0).collect();
        assert_eq!(refs, [10, 20, 30]);
    }
}
