//! Trees: the SpeedTree trees placed on the loaded squares, grown as the
//! game grows them (`cellview::tree`, `speedtree`), a model per tree base
//! and seed, loaded in the background with their squares. Each tree shows,
//! every frame, the branch and leaf levels of detail the game's rules pick
//! for its distance from the camera (`speedtree::lod`), swayed by the
//! weather's wind as the game moves it (`speedtree::wind`), and lit like
//! the rest of the outdoors (the leaves with the image space's tree
//! dimmer). Shaders: `tree_leaf.wgsl`, `tree_branch.wgsl`.

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology,
};
use bevy::render::primitives::Aabb;
use bevy::render::render_resource::{
    AsBindGroup, Face, RenderPipelineDescriptor, ShaderRef, ShaderType,
    SpecializedMeshPipelineError, VertexFormat,
};
use cellview::space;
use cellview::tree::{PlacedTree, TreeModel};
use esm::FormId;
use speedtree::lod::{lod_level, visible, LodLimits};
use speedtree::mesh::{BranchMesh, LeafMesh};
use speedtree::wind::{FastTrig, Wind, WindFrame, WindSettings};
use world::tree::{TreeSettings, TREE_DIMMER};

use crate::dialogue::DialogueState;
use crate::exterior::Exterior;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles, SceneEntity, Spawner};

const LEAF_SHADER: Handle<Shader> = weak_handle!("3c1f6a52-8b2e-4d71-9e4a-5f0d2b7c8e19");
const BRANCH_SHADER: Handle<Shader> = weak_handle!("8e4d2a17-6c3b-4f95-a1d8-2b7e9c0f4a63");

/// `BLENDINDICES`: wind weight, wind matrix × 4, and the leaf's corner and
/// brightness (or the branch's brightness), card scale.
pub const ATTRIBUTE_BLEND: MeshVertexAttribute =
    MeshVertexAttribute::new("TreeBlend", 2_917_403_551, VertexFormat::Float32x4);
/// A branch vertex's `TANGENT` (around the branch) and `BINORMAL`.
pub const ATTRIBUTE_TANGENT: MeshVertexAttribute =
    MeshVertexAttribute::new("TreeTangent", 2_917_403_552, VertexFormat::Float32x3);
pub const ATTRIBUTE_BINORMAL: MeshVertexAttribute =
    MeshVertexAttribute::new("TreeBinormal", 2_917_403_553, VertexFormat::Float32x3);

pub type LeafMaterial = ExtendedMaterial<StandardMaterial, LeafShader>;
pub type BranchMaterial = ExtendedMaterial<StandardMaterial, BranchShader>;

/// What the leaf shader reads (`tree_leaf.wgsl`), in the game's units and
/// axes.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct LeafParams {
    pub ambient: Vec4,
    pub diffuse: Vec4,
    pub sun_direction: Vec4,
    pub dimmer: Vec4,
    pub billboard_right: Vec4,
    pub billboard_up: Vec4,
    pub rock: Vec4,
    pub rustle: Vec4,
    pub lighting: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
    pub wind: [Vec4; 16],
    pub leaf_base: [Vec4; 48],
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct LeafShader {
    #[uniform(100)]
    pub params: LeafParams,
    #[texture(101)]
    #[sampler(102)]
    pub texture: Option<Handle<Image>>,
}

/// What the branch shader reads (`tree_branch.wgsl`).
#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct BranchParams {
    pub ambient: Vec4,
    pub sun_color: Vec4,
    pub sun_direction: Vec4,
    pub flags: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
    pub wind: [Vec4; 16],
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct BranchShader {
    #[uniform(100)]
    pub params: BranchParams,
    #[texture(101)]
    #[sampler(102)]
    pub texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    pub normal_map: Option<Handle<Image>>,
}

impl MaterialExtension for LeafShader {
    fn vertex_shader() -> ShaderRef {
        LEAF_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        LEAF_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor.vertex.shader != LEAF_SHADER {
            return Ok(());
        }
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_BLEND.at_shader_location(8),
        ])?];
        Ok(())
    }
}

impl MaterialExtension for BranchShader {
    fn vertex_shader() -> ShaderRef {
        BRANCH_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        BRANCH_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor.vertex.shader != BRANCH_SHADER {
            return Ok(());
        }
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_BLEND.at_shader_location(8),
            ATTRIBUTE_TANGENT.at_shader_location(9),
            ATTRIBUTE_BINORMAL.at_shader_location(10),
        ])?];
        Ok(())
    }
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, LEAF_SHADER, "tree_leaf.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, BRANCH_SHADER, "tree_branch.wgsl", Shader::from_wgsl);
        app.add_plugins((
            MaterialPlugin::<LeafMaterial>::default(),
            MaterialPlugin::<BranchMaterial>::default(),
        ))
        .init_resource::<TreeField>()
        // The trees put on screen with the world (stage 4), then swayed:
        // `BSTreeManager::Update` (step 77, `world::frame::world_time`,
        // docs/FRAME_SKELETON.md "PR 5 result"). The wind moves at its wind
        // update (`006654dd`), under its gate (the world runs); the rest of
        // the sway right after it, outside the gate: the camera's axes
        // (`CSpeedTreeRT::SetCamera`, called in menus too), the light and
        // the levels of detail (their places in the exe aren't traced).
        .add_systems(
            Update,
            (
                // Placed for order only: in the exe the trees are the cells'
                // references, attached with the grid (step 78); kept ahead of
                // the tree manager so that a tree put on screen gets this
                // frame's wind and light.
                stream_trees
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::WorldAndTime,
                    ))
                    .before(crate::frame_order::FrameSet::step(
                        crate::frame_order::TREE_MANAGER_UPDATE,
                    )),
                blow_wind.in_set(crate::frame_order::WorldSet::at(
                    crate::frame_order::TREE_MANAGER_UPDATE,
                    crate::frame_order::TREE_WIND_AT,
                )),
                sway_trees
                    .in_set(crate::frame_order::FrameSet::step(
                        crate::frame_order::TREE_MANAGER_UPDATE,
                    ))
                    .after(crate::frame_order::WorldSet::at(
                        crate::frame_order::TREE_MANAGER_UPDATE,
                        crate::frame_order::TREE_WIND_AT,
                    ))
                    .before(crate::frame_order::WorldSet::after(
                        crate::frame_order::TREE_MANAGER_UPDATE,
                        crate::frame_order::TREE_WIND_AT,
                    )),
            )
                // Kept: the trees on screen before they sway.
                .chain(),
        );
    }
}

/// A tree model on the graphics card: its meshes per level and materials.
struct GpuTree {
    model: Arc<TreeModel>,
    leaf_material: Handle<LeafMaterial>,
    branch_material: Handle<BranchMaterial>,
    branches: Vec<Option<Handle<Mesh>>>,
    leaves: Vec<Option<Handle<Mesh>>>,
}

/// One placed tree on screen.
struct ShownTree {
    root: Entity,
    position: [f32; 3],
    model: (FormId, i32),
    /// Per level: its entity, if the level has anything.
    branches: Vec<Option<Entity>>,
    leaves: Vec<Option<Entity>>,
}

enum TreeSquare {
    Loading,
    Loaded(Vec<ShownTree>),
}

type Models = Arc<Mutex<HashMap<(FormId, i32), Option<Arc<TreeModel>>>>>;
/// A placed tree with its model's key and the model grown.
type LoadedTree = (PlacedTree, (FormId, i32), Option<Arc<TreeModel>>);
type Finished = ((i32, i32), Vec<LoadedTree>);

/// The trees of the loaded squares.
#[derive(Resource)]
pub struct TreeField {
    squares: HashMap<(i32, i32), TreeSquare>,
    sender: Sender<Finished>,
    receiver: Mutex<Receiver<Finished>>,
    models: Models,
    gpu: HashMap<(FormId, i32), GpuTree>,
    settings: Option<TreeSettings>,
    wind: Wind,
    /// The wind's last frame ([`blow_wind`]); kept while the wind update
    /// doesn't run (menu mode), so the trees hold still.
    frame: Option<WindFrame>,
    trig: FastTrig,
    /// The light last worked out (ambient, sun colour with the sunlight
    /// dimmer, toward the sun, tree dimmer, fog colour, fog range), the
    /// weathers it followed and the hour.
    light: Option<TreeLight>,
    shown: Option<(Option<FormId>, Option<FormId>)>,
    applied: Option<f32>,
}

#[derive(Clone, Copy)]
struct TreeLight {
    ambient: Vec4,
    sun_color: Vec4,
    sun_direction: Vec4,
    tree_dimmer: f32,
    fog_color: Vec4,
    fog_range: Vec4,
}

impl Default for TreeField {
    fn default() -> Self {
        let (sender, receiver) = channel();
        TreeField {
            squares: HashMap::new(),
            sender,
            receiver: Mutex::new(receiver),
            models: Arc::default(),
            gpu: HashMap::new(),
            settings: None,
            wind: Wind::default(),
            frame: None,
            trig: FastTrig::new(),
            light: None,
            shown: None,
            applied: None,
        }
    }
}

/// Squares of trees loading at once.
const LOADING_AT_ONCE: usize = 3;

/// Loads the trees of the loaded squares in the background, puts them on
/// screen, and drops those of squares no longer loaded.
pub fn stream_trees(
    exterior: Option<Res<Exterior>>,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    mut spawner: Spawner,
    mut field: ResMut<TreeField>,
    mut leaf_materials: ResMut<Assets<LeafMaterial>>,
    mut branch_materials: ResMut<Assets<BranchMaterial>>,
) {
    let field = &mut *field;
    let Some(exterior) = exterior else {
        // Indoors: whatever was on screen went with the place.
        field.squares.clear();
        return;
    };
    if exterior.is_added() {
        field.squares.clear();
        field.applied = None;
    }
    let settings = *field.settings.get_or_insert_with(|| game.0.tree_settings());
    let wanted = exterior.loaded_squares();

    let finished: Vec<Finished> = field
        .receiver
        .lock()
        .map(|r| r.try_iter().collect())
        .unwrap_or_default();
    for (square, trees) in finished {
        if !matches!(field.squares.get(&square), Some(TreeSquare::Loading)) {
            continue;
        }
        let mut shown = Vec::new();
        for (placed, key, model) in trees {
            let Some(model) = model else { continue };
            let gpu = field.gpu.entry(key).or_insert_with(|| {
                field.applied = None;
                upload(
                    &model,
                    &mut spawner,
                    &mut leaf_materials,
                    &mut branch_materials,
                )
            });
            shown.push(spawn_tree(&placed, key, gpu, &mut spawner));
        }
        field.squares.insert(square, TreeSquare::Loaded(shown));
    }

    // Squares no longer loaded lose their trees.
    let gone: Vec<(i32, i32)> = field
        .squares
        .iter()
        .filter(|(s, state)| !matches!(state, TreeSquare::Loading) && !wanted.contains(s))
        .map(|(s, _)| *s)
        .collect();
    for square in gone {
        if let Some(TreeSquare::Loaded(trees)) = field.squares.remove(&square) {
            for t in trees {
                spawner.commands.entity(t.root).despawn();
            }
        }
    }

    let loading = field
        .squares
        .values()
        .filter(|s| matches!(s, TreeSquare::Loading))
        .count();
    let mut missing: Vec<(i32, i32)> = wanted
        .into_iter()
        .filter(|s| !field.squares.contains_key(s))
        .collect();
    missing.sort();
    for square in missing
        .into_iter()
        .take(LOADING_AT_ONCE.saturating_sub(loading))
    {
        field.squares.insert(square, TreeSquare::Loading);
        let game = Arc::clone(&game.0);
        let grid = Arc::clone(&exterior.grid);
        let sender = field.sender.clone();
        let models = Arc::clone(&field.models);
        let disabled = state.0.disabled.clone();
        std::thread::spawn(move || {
            let trees = load_square(&game, &grid, square, &disabled, &models, &settings);
            let _ = sender.send((square, trees));
        });
    }
}

/// One square's trees, each with its grown model (grown once per base and
/// seed).
fn load_square(
    game: &cellview::Game,
    grid: &world::WorldGrid,
    square: (i32, i32),
    disabled: &world::Disabled,
    models: &Models,
    settings: &TreeSettings,
) -> Vec<LoadedTree> {
    let placed = match game.square_trees(grid, square, disabled) {
        Ok(p) => p,
        Err(e) => {
            println!(
                "  couldn't read the trees of {},{}: {e}",
                square.0, square.1
            );
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for tree in placed {
        let Some(base) = world::tree::TreeBase::load(&game.order, tree.base) else {
            continue;
        };
        let key = (tree.base, base.seed(tree.xsed));
        // Held while growing, so two squares never grow the same model.
        let Ok(mut known) = models.lock() else {
            continue;
        };
        let model = known
            .entry(key)
            .or_insert_with(|| {
                let model = game.tree_model(key.0, key.1, settings).map(Arc::new);
                match &model {
                    Some(m) => println!(
                        "  tree {} (seed {}): {} branch and {} leaf levels",
                        m.path,
                        key.1,
                        m.branches.len(),
                        m.leaves.len()
                    ),
                    None => println!("  couldn't grow the tree {}", base.spt_path()),
                }
                model
            })
            .clone();
        drop(known);
        out.push((tree, key, model));
    }
    out
}

/// A tree model's meshes and materials.
fn upload(
    model: &Arc<TreeModel>,
    spawner: &mut Spawner,
    leaf_materials: &mut Assets<LeafMaterial>,
    branch_materials: &mut Assets<BranchMaterial>,
) -> GpuTree {
    let leaf_texture = model.leaf_texture.as_ref().and_then(|t| spawner.upload(t));
    let bark = model
        .branch_texture
        .as_ref()
        .and_then(|t| spawner.upload(t));
    let bark_normal = model
        .branch_normal_map
        .as_ref()
        .and_then(|t| spawner.upload(t));
    let mut leaf_base = [Vec4::ZERO; 48];
    for (i, c) in model.leaf_base.iter().take(48).enumerate() {
        leaf_base[i] = Vec4::from(*c);
    }
    let leaf_material = leaf_materials.add(LeafMaterial {
        base: StandardMaterial {
            // Transparency multisampling (the game's `ATOC`): the alpha is
            // the share of samples covered.
            alpha_mode: AlphaMode::AlphaToCoverage,
            double_sided: true,
            cull_mode: None,
            ..default()
        },
        extension: LeafShader {
            params: LeafParams {
                ambient: Vec4::ZERO,
                diffuse: Vec4::ZERO,
                sun_direction: Vec4::Z,
                dimmer: Vec4::new(1.0, crate::full_brightness_nits(), 0.0, 0.0),
                billboard_right: Vec4::X,
                billboard_up: Vec4::Z,
                rock: Vec4::ZERO,
                rustle: Vec4::ZERO,
                lighting: Vec4::new(0.0, model.curvature, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                fog_range: Vec4::ZERO,
                wind: identity_wind(),
                leaf_base,
            },
            texture: leaf_texture,
        },
    });
    let branch_material = branch_materials.add(BranchMaterial {
        base: StandardMaterial {
            cull_mode: Some(Face::Back),
            ..default()
        },
        extension: BranchShader {
            params: BranchParams {
                ambient: Vec4::ZERO,
                sun_color: Vec4::ZERO,
                sun_direction: Vec4::Z,
                flags: Vec4::new(crate::full_brightness_nits(), 0.0, 0.0, 0.0),
                fog_color: Vec4::ZERO,
                fog_range: Vec4::ZERO,
                wind: identity_wind(),
            },
            texture: bark,
            normal_map: bark_normal,
        },
    });
    GpuTree {
        branches: model
            .branches
            .iter()
            .map(|b| (!b.strip.is_empty()).then(|| spawner.meshes.add(branch_mesh(b))))
            .collect(),
        leaves: model
            .leaves
            .iter()
            .map(|l| (!l.indices.is_empty()).then(|| spawner.meshes.add(leaf_mesh(l))))
            .collect(),
        model: Arc::clone(model),
        leaf_material,
        branch_material,
    }
}

fn identity_wind() -> [Vec4; 16] {
    let mut w = [Vec4::ZERO; 16];
    for m in 0..4 {
        w[m * 4] = Vec4::X;
        w[m * 4 + 1] = Vec4::Y;
        w[m * 4 + 2] = Vec4::Z;
        w[m * 4 + 3] = Vec4::W;
    }
    w
}

/// A placed tree: a root at its place (game space converted like every
/// placed model) with an entity per level of detail, hidden until the
/// first frame picks the levels.
fn spawn_tree(
    placed: &PlacedTree,
    key: (FormId, i32),
    gpu: &GpuTree,
    spawner: &mut Spawner,
) -> ShownTree {
    let transform = Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&placed.matrix())));
    let root = spawner
        .commands
        .spawn((transform, Visibility::default(), SceneEntity))
        .id();
    // Cards and the wind's turns stay within the model's reach of its
    // origin.
    let r = gpu.model.radius.max(1.0);
    let aabb = Aabb::from_min_max(Vec3::splat(-r), Vec3::splat(r));
    let mut child = |mesh: &Handle<Mesh>, branch: bool| {
        let mut e = spawner.commands.spawn((
            Mesh3d(mesh.clone()),
            Transform::IDENTITY,
            Visibility::Hidden,
            aabb,
            ChildOf(root),
        ));
        if branch {
            e.insert(MeshMaterial3d(gpu.branch_material.clone()));
        } else {
            e.insert(MeshMaterial3d(gpu.leaf_material.clone()));
        }
        e.id()
    };
    let branches = gpu
        .branches
        .iter()
        .map(|m| m.as_ref().map(|m| child(m, true)))
        .collect();
    let leaves = gpu
        .leaves
        .iter()
        .map(|m| m.as_ref().map(|m| child(m, false)))
        .collect();
    ShownTree {
        root,
        position: placed.position,
        model: key,
        branches,
        leaves,
    }
}

/// A leaf level's cards for Bevy (the vertices as the game uploads them).
fn leaf_mesh(l: &LeafMesh) -> Mesh {
    let mut out = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    out.insert_attribute(Mesh::ATTRIBUTE_POSITION, l.positions.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, l.normals.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_UV_0, l.uvs.clone());
    out.insert_attribute(ATTRIBUTE_BLEND, l.blend.clone());
    out.insert_indices(Indices::U16(l.indices.clone()));
    out
}

/// A branch level for Bevy: its strip as triangles.
fn branch_mesh(b: &BranchMesh) -> Mesh {
    let mut out = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    out.insert_attribute(Mesh::ATTRIBUTE_POSITION, b.positions.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, b.normals.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_UV_0, b.uvs.clone());
    out.insert_attribute(ATTRIBUTE_BLEND, b.blend.clone());
    out.insert_attribute(ATTRIBUTE_TANGENT, b.tangents.clone());
    out.insert_attribute(ATTRIBUTE_BINORMAL, b.binormals.clone());
    let indices: Vec<u16> = b.triangles().into_iter().flatten().collect();
    out.insert_indices(Indices::U16(indices));
    out
}

/// Outdoors, while the world runs: the wind moves on (`speedtree::wind`,
/// the wind update `006658b0`, which `BSTreeManager::Update` calls only when
/// it isn't told the world is stopped, `006653a9`; this system sits in that
/// sub-step's set, `crate::frame_order::WorldSet`). The frame is kept for
/// [`sway_trees`]; in menu mode the trees hold the pose they had.
pub fn blow_wind(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    exterior: Option<Res<Exterior>>,
    time: Res<Time>,
    mut field: ResMut<TreeField>,
    mut weathers: ResMut<crate::weather::Weathers>,
) {
    let field = &mut *field;
    let Some(exterior) = exterior else {
        return;
    };
    if field.gpu.is_empty() {
        return;
    }
    let order = &game.0.order;
    let Some(weather) = weathers.mix(order, &state.0, exterior.weather) else {
        return;
    };
    // The wind this frame: the weather's (0 to 1).
    let tree_settings = field.settings.unwrap_or_else(|| game.0.tree_settings());
    let wind_settings = WindSettings {
        rock_amount_sway: tree_settings.rock_amount_sway,
        rustle_amount_sway: tree_settings.rustle_amount_sway,
        rock_speed_sway: tree_settings.rock_speed_sway,
        rustle_speed_sway: tree_settings.rustle_speed_sway,
        rock_time_scale: tree_settings.rock_time_scale,
        rustle_time_scale: tree_settings.rustle_time_scale,
    };
    field.frame = Some(field.wind.update(
        weather.wind(),
        time.delta_secs(),
        &wind_settings,
        &field.trig,
    ));
}

/// Every frame outdoors: the wind's last frame ([`blow_wind`]) and the
/// camera's axes for the leaf cards (`CSpeedTreeRT::SetCamera`, which
/// `BSTreeManager::Update` calls in menus too), the light once the clock has
/// moved a game minute, and each tree's levels of detail for its distance
/// from the camera.
#[allow(clippy::too_many_arguments)]
pub fn sway_trees(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    exterior: Option<Res<Exterior>>,
    settings: Res<crate::Settings>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut field: ResMut<TreeField>,
    mut leaf_materials: ResMut<Assets<LeafMaterial>>,
    mut branch_materials: ResMut<Assets<BranchMaterial>>,
    mut weathers: ResMut<crate::weather::Weathers>,
    mut visibilities: Query<&mut Visibility>,
) {
    let field = &mut *field;
    let Some(exterior) = exterior else {
        field.applied = None;
        return;
    };
    if field.gpu.is_empty() {
        return;
    }
    let Ok(camera) = cameras.single() else {
        return;
    };
    let order = &game.0.order;
    let shown = (
        state.0.weather.current.or(exterior.weather),
        state.0.weather.previous,
    );
    if field.shown != Some(shown) {
        field.shown = Some(shown);
        field.applied = None;
    }
    let Some(weather) = weathers.mix(order, &state.0, exterior.weather) else {
        return;
    };
    let hour = state
        .0
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);
    if field.applied.is_none_or(|h| (h - hour).abs() >= 1.0 / 60.0) {
        field.applied = Some(hour);
        let clock = world::weather::SkyClock::new(
            exterior.grid.climate.as_ref(),
            world::weather::SkySettings::load(order),
        );
        let light = weather.light_at(&clock, hour);
        let (ambient, mut directional, fog) = cellview::exterior_light(&light);
        let modifier = weather.modifier_at(order, &clock, hour);
        let image_space = exterior.grid.image_space.as_ref();
        // The sun's light with the sunlight dimmer, as for lit statics (the
        // leaves' recorded `DiffColor` was the sunlight × 1.21).
        let dimmer = world::weather::sunlight_dimmer(image_space, modifier.as_ref());
        if let Some(d) = directional.as_mut() {
            d.color = d.color.map(|c| c * dimmer);
        }
        // The leaves' `SunDimmer.x`: the image space's tree dimmer after the
        // weather's modifiers (track 13), as the grass dimmer is taken.
        let tree_dimmer = image_space
            .and_then(world::tree::tree_dimmer)
            .map(|d| {
                modifier
                    .as_ref()
                    .map_or(d, |m| d * m.multiply[TREE_DIMMER] + m.add[TREE_DIMMER])
            })
            .unwrap_or(1.0);
        let fields = crate::light_fields(
            ambient,
            directional.as_ref(),
            fog.as_ref(),
            settings.brightness,
        );
        let sun_direction = match &directional {
            Some(d) => Vec4::new(d.direction[0], d.direction[1], d.direction[2], 0.0),
            None => Vec4::Z,
        };
        field.light = Some(TreeLight {
            ambient: fields.ambient.truncate().extend(1.0),
            sun_color: fields.directional_color,
            sun_direction,
            tree_dimmer,
            fog_color: fields.fog_color,
            fog_range: fields.fog_range,
        });
    }

    // The wind's last frame (none before the first: the materials keep the
    // still wind they were made with).
    let tree_settings = field.settings.unwrap_or_else(|| game.0.tree_settings());
    let frame = field.frame;

    // The camera's axes in the game's (`00bb1a90`).
    let game_axis = |v: Vec3| Vec4::new(v.x, -v.z, v.y, 0.0);
    let right = game_axis(camera.rotation * Vec3::X);
    let up = game_axis(camera.rotation * Vec3::Y);

    let light = field.light;
    for gpu in field.gpu.values() {
        if let Some(m) = leaf_materials.get_mut(&gpu.leaf_material) {
            leaf_params(
                &mut m.extension.params,
                gpu,
                frame.as_ref(),
                light.as_ref(),
                right,
                up,
            );
        }
        if let Some(m) = branch_materials.get_mut(&gpu.branch_material) {
            let p = &mut m.extension.params;
            if let Some(frame) = &frame {
                p.wind = wind_rows(frame);
            }
            if let Some(l) = &light {
                p.ambient = l.ambient;
                p.sun_color = l.sun_color;
                p.sun_direction = l.sun_direction;
                p.fog_color = l.fog_color;
                p.fog_range = l.fog_range;
            }
        }
    }

    // Levels of detail by distance from the camera (`00669a10`).
    let eye = game_point(camera.translation);
    let limits = LodLimits {
        near: tree_settings.near,
        far: tree_settings.far,
    };
    for square in field.squares.values() {
        let TreeSquare::Loaded(trees) = square else {
            continue;
        };
        for t in trees {
            let Some(gpu) = field.gpu.get(&t.model) else {
                continue;
            };
            let m = &gpu.model;
            let lod = lod_level(eye, t.position, limits);
            let v = visible(lod, m.branch_levels(), m.leaf_levels(), m.fade);
            for (i, e) in t.branches.iter().enumerate() {
                if let Some(e) = e {
                    let on = v.branches.is_some_and(|b| b.0 as usize == i);
                    set_visible(&mut visibilities, *e, on);
                }
            }
            for (i, e) in t.leaves.iter().enumerate() {
                if let Some(e) = e {
                    let on = v.leaves.iter().any(|l| l.0 as usize == i);
                    set_visible(&mut visibilities, *e, on);
                }
            }
        }
    }
}

fn set_visible(visibilities: &mut Query<&mut Visibility>, entity: Entity, on: bool) {
    if let Ok(mut v) = visibilities.get_mut(entity) {
        let want = if on {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *v != want {
            *v = want;
        }
    }
}

fn wind_rows(frame: &WindFrame) -> [Vec4; 16] {
    let mut w = [Vec4::ZERO; 16];
    for (m, matrix) in frame.matrices.iter().enumerate() {
        for (r, row) in matrix.iter().enumerate() {
            w[m * 4 + r] = Vec4::from(*row);
        }
    }
    w
}

/// The leaf shader's values for this frame (`00bb1f60`, `00bb2d10`):
/// `RockParams` = (rock amount, the tree's rock speed × the rock clock, the
/// file's 21001, 0), `RustleParams` the same with rustle and 21000.
fn leaf_params(
    p: &mut LeafParams,
    gpu: &GpuTree,
    frame: Option<&WindFrame>,
    light: Option<&TreeLight>,
    right: Vec4,
    up: Vec4,
) {
    let m = &gpu.model;
    if let Some(frame) = frame {
        p.wind = wind_rows(frame);
        p.rock = Vec4::new(
            frame.rock_amount,
            m.rock_speed * frame.rock_time,
            m.rock_amount,
            0.0,
        );
        p.rustle = Vec4::new(
            frame.rustle_amount,
            m.rustle_speed * frame.rustle_time,
            m.rustle_amount,
            0.0,
        );
    }
    p.billboard_right = right;
    p.billboard_up = up;
    if let Some(l) = light {
        p.ambient = l.ambient;
        p.diffuse = l.sun_color;
        p.sun_direction = l.sun_direction;
        p.dimmer.x = l.tree_dimmer;
        p.fog_color = l.fog_color;
        p.fog_range = l.fog_range;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_meshes_carry_every_attribute() {
        let b = BranchMesh {
            positions: vec![[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 4],
            uvs: vec![[0.0; 2]; 4],
            tangents: vec![[1.0, 0.0, 0.0]; 4],
            binormals: vec![[0.0, 1.0, 0.0]; 4],
            blend: vec![[1.0, 4.0, 0.5, 0.0]; 4],
            strip: vec![0, 1, 2, 3],
        };
        let mesh = branch_mesh(&b);
        assert_eq!(mesh.count_vertices(), 4);
        assert_eq!(mesh.indices().map(|i| i.len()), Some(6));
        assert!(mesh.attribute(ATTRIBUTE_TANGENT).is_some());
        assert!(mesh.attribute(ATTRIBUTE_BINORMAL).is_some());
    }

    #[test]
    fn wind_rows_follow_the_matrices() {
        let mut frame = WindFrame {
            matrices: [[[0.0; 4]; 4]; 4],
            rock_amount: 0.0,
            rustle_amount: 0.0,
            rock_time: 0.0,
            rustle_time: 0.0,
        };
        frame.matrices[2][1] = [0.0, 0.9, -0.1, 0.0];
        let rows = wind_rows(&frame);
        assert_eq!(rows[9], Vec4::new(0.0, 0.9, -0.1, 0.0));
    }
}
