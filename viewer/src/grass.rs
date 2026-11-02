//! Grass: the blades the game grows around the player (`world::grass`),
//! loaded in the background for the player's square and the eight around
//! it (the game fills no farther, whatever `uGridsToLoad` is), one mesh per
//! grass and square (`cellview::grass`), and drawn with the game's grass
//! shader (`grass.wgsl`). The light follows the clock as the rest of the
//! outdoors does (`daylight`), and the wind sways each grass at its own
//! pace every frame.

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
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError,
    VertexFormat,
};
use cellview::grass::{grass_mesh, GrassMesh, GrassModel};
use cellview::space;
use esm::FormId;
use world::grass::{squares_near, wind_phase, GrassSettings, GRASS_DIMMER};

use crate::dialogue::DialogueState;
use crate::exterior::Exterior;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles, SceneEntity, Spawner};

const SHADER: Handle<Shader> = weak_handle!("6a2f8c41-9d3e-4b57-8e12-c4b7a9d05e63");

/// Each vertex's blade (`InstanceData`, see `world::grass::GrassInstance`).
pub const ATTRIBUTE_INSTANCE: MeshVertexAttribute =
    MeshVertexAttribute::new("GrassInstance", 3_814_202_931, VertexFormat::Float32x4);

pub type GrassMaterial = ExtendedMaterial<StandardMaterial, GrassShader>;

/// What the grass shader reads (`grass.wgsl`), in the game's units and
/// axes.
#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct GrassParams {
    /// Toward the sun.
    pub sun_direction: Vec4,
    /// The sun's colour: the weather's sunlight, without the sunlight
    /// dimmer the lit surfaces get (the game hands the grass shader the sun
    /// light's own colour, `00baac10`, and its own dimmer instead).
    pub sun_color: Vec4,
    pub ambient: Vec4,
    pub scale_mask: Vec4,
    /// Direction x and y (north: the game's `(0, 1)`), size, phase.
    pub wind: Vec4,
    /// Fade start and length (game units), the grass dimmer, the alpha
    /// test reference.
    pub fade: Vec4,
    /// Fit to slope, lit by the model's normals, full-brightness scale.
    pub flags: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct GrassShader {
    #[uniform(100)]
    pub params: GrassParams,
    #[texture(101)]
    #[sampler(102)]
    pub texture: Option<Handle<Image>>,
}

impl MaterialExtension for GrassShader {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor.vertex.shader != SHADER {
            return Ok(());
        }
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
            ATTRIBUTE_INSTANCE.at_shader_location(8),
        ])?];
        Ok(())
    }
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "grass.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<GrassMaterial>::default())
            .init_resource::<GrassField>()
            .add_systems(
                Update,
                // Not in the frame's stages: the GPU's copy of the grass around
                // the camera (`crate::frame_order::ViewerSet::AfterFrame`).
                (stream_grass, light_grass)
                    .chain()
                    .in_set(crate::frame_order::ViewerSet::AfterFrame),
            );
    }
}

/// A grass's material, shared by every square.
struct GrassType {
    material: Handle<GrassMaterial>,
    wave_period: f32,
}

enum GrassSquare {
    Loading,
    Loaded(Vec<Entity>),
}

type Finished = ((i32, i32), Vec<(GrassMesh, Arc<GrassModel>)>);

/// Models read so far, by path (shared with the loading threads).
type Models = Arc<Mutex<HashMap<String, Option<Arc<GrassModel>>>>>;

/// The grass around the player.
#[derive(Resource)]
pub struct GrassField {
    squares: HashMap<(i32, i32), GrassSquare>,
    sender: Sender<Finished>,
    receiver: Mutex<Receiver<Finished>>,
    types: HashMap<FormId, GrassType>,
    models: Models,
    settings: Option<GrassSettings>,
    /// The weathers the light last followed (current, fading out), and
    /// the hour last applied.
    shown: Option<(Option<FormId>, Option<FormId>)>,
    applied: Option<f32>,
}

impl Default for GrassField {
    fn default() -> Self {
        let (sender, receiver) = channel();
        GrassField {
            squares: HashMap::new(),
            sender,
            receiver: Mutex::new(receiver),
            types: HashMap::new(),
            models: Arc::default(),
            settings: None,
            shown: None,
            applied: None,
        }
    }
}

/// Squares of grass loading at once.
const LOADING_AT_ONCE: usize = 3;

/// Loads the grass of the squares around the player in the background,
/// puts it on screen, and drops what's no longer near (the game removes
/// the grass of every other square, `0057d0a0`).
pub fn stream_grass(
    exterior: Option<Res<Exterior>>,
    game: Res<GameFiles>,
    mut spawner: Spawner,
    mut field: ResMut<GrassField>,
    mut materials: ResMut<Assets<GrassMaterial>>,
    cameras: Query<&Transform, With<FlyCamera>>,
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
    let settings = *field
        .settings
        .get_or_insert_with(|| game.0.grass_settings());
    let Ok(camera) = cameras.single() else {
        return;
    };
    let [x, y, _] = game_point(camera.translation);
    let wanted = if settings.makes_grass() {
        squares_near(x, y, &settings)
    } else {
        Vec::new()
    };

    let finished: Vec<Finished> = field
        .receiver
        .lock()
        .map(|r| r.try_iter().collect())
        .unwrap_or_default();
    for (square, meshes) in finished {
        if !matches!(field.squares.get(&square), Some(GrassSquare::Loading)) {
            continue;
        }
        let mut entities = Vec::new();
        for (mesh, model) in &meshes {
            let material = match field.types.entry(mesh.grass.form_id) {
                std::collections::hash_map::Entry::Occupied(kind) => kind.get().material.clone(),
                std::collections::hash_map::Entry::Vacant(slot) => {
                    let texture = model.texture.as_ref().and_then(|t| spawner.upload(t));
                    let material = materials.add(GrassMaterial {
                        base: StandardMaterial {
                            // Transparency multisampling: the shader's alpha
                            // is the share of samples covered.
                            alpha_mode: AlphaMode::AlphaToCoverage,
                            double_sided: model.double_sided,
                            cull_mode: if model.double_sided {
                                None
                            } else {
                                Some(bevy::render::render_resource::Face::Back)
                            },
                            ..default()
                        },
                        extension: GrassShader {
                            params: params(&mesh.grass, model, &settings),
                            texture,
                        },
                    });
                    slot.insert(GrassType {
                        material: material.clone(),
                        wave_period: mesh.grass.wave_period,
                    });
                    // New materials take the light of the hour.
                    field.applied = None;
                    material
                }
            };
            let (lo, hi) = mesh.bounds;
            let [a, b] = [space::point(lo), space::point(hi)];
            let aabb = Aabb::from_min_max(Vec3::from(a).min(b.into()), Vec3::from(a).max(b.into()));
            let handle = spawner.meshes.add(bevy_mesh(mesh));
            entities.push(
                spawner
                    .commands
                    .spawn((
                        Mesh3d(handle),
                        MeshMaterial3d(material),
                        Transform::IDENTITY,
                        aabb,
                        SceneEntity,
                    ))
                    .id(),
            );
        }
        field.squares.insert(square, GrassSquare::Loaded(entities));
    }

    // Squares no longer near lose their grass.
    let gone: Vec<(i32, i32)> = field
        .squares
        .iter()
        .filter(|(s, state)| !matches!(state, GrassSquare::Loading) && !wanted.contains(s))
        .map(|(s, _)| *s)
        .collect();
    for square in gone {
        if let Some(GrassSquare::Loaded(entities)) = field.squares.remove(&square) {
            for entity in entities {
                spawner.commands.entity(entity).despawn();
            }
        }
    }

    // Missing ones, the player's first.
    let loading = field
        .squares
        .values()
        .filter(|s| matches!(s, GrassSquare::Loading))
        .count();
    let here = world::square_of([x, y, 0.0]);
    let mut missing: Vec<(i32, i32)> = wanted
        .into_iter()
        .filter(|s| !field.squares.contains_key(s))
        .collect();
    missing.sort_by_key(|s| (s.0 - here.0).abs().max((s.1 - here.1).abs()));
    for square in missing
        .into_iter()
        .take(LOADING_AT_ONCE.saturating_sub(loading))
    {
        field.squares.insert(square, GrassSquare::Loading);
        let game = Arc::clone(&game.0);
        let grid = Arc::clone(&exterior.grid);
        let sender = field.sender.clone();
        let models = Arc::clone(&field.models);
        std::thread::spawn(move || {
            let meshes = load_square(&game, &grid, square, &models, settings.wind_max);
            let _ = sender.send((square, meshes));
        });
    }
}

/// One square's grass, as meshes with their models (read once each).
fn load_square(
    game: &cellview::Game,
    grid: &world::WorldGrid,
    square: (i32, i32),
    models: &Models,
    wind_max: f32,
) -> Vec<(GrassMesh, Arc<GrassModel>)> {
    let batches = match game.square_grass(grid, square) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "  couldn't make the grass of {},{}: {e}",
                square.0, square.1
            );
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for batch in &batches {
        let Some(path) = batch.grass.model.clone() else {
            continue;
        };
        let known = models.lock().ok().and_then(|m| m.get(&path).cloned());
        let model = match known {
            Some(model) => model,
            None => {
                let model = game.grass_model(&path).map(Arc::new);
                if model.is_none() {
                    println!("  couldn't read the grass model {path}");
                }
                if let Ok(mut m) = models.lock() {
                    m.insert(path, model.clone());
                }
                model
            }
        };
        if let Some(model) = model {
            out.push((grass_mesh(batch, &model, wind_max), model));
        }
    }
    out
}

/// A grass mesh for Bevy: the model's vertices as stored (game units; the
/// shader places them) and each one's blade.
fn bevy_mesh(mesh: &GrassMesh) -> Mesh {
    let mut out = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    out.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh.uvs.clone());
    out.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh.colors.clone());
    out.insert_attribute(ATTRIBUTE_INSTANCE, mesh.instances.clone());
    out.insert_indices(Indices::U32(mesh.indices.clone()));
    out
}

/// A grass's shader values that don't change with the hour.
fn params(
    grass: &world::grass::Grass,
    model: &GrassModel,
    settings: &GrassSettings,
) -> GrassParams {
    let flag = |on: bool| if on { 1.0 } else { 0.0 };
    GrassParams {
        sun_direction: Vec4::new(0.0, 0.0, 1.0, 1.0),
        sun_color: Vec4::ZERO,
        ambient: Vec4::ZERO,
        // `ScaleMask` (`00bac9c0`): every axis with uniform scaling, else
        // only the height.
        scale_mask: if grass.uniform_scaling() {
            Vec4::new(1.0, 1.0, 1.0, 0.0)
        } else {
            Vec4::new(0.0, 0.0, 1.0, 0.0)
        },
        wind: Vec4::new(0.0, 1.0, 0.0, 0.0),
        fade: Vec4::new(
            settings.start_fade,
            settings.fade_range.max(1e-3),
            settings.dimmer,
            model.alpha_test,
        ),
        flags: Vec4::new(
            flag(grass.fit_to_slope()),
            flag(grass.vertex_lighting()),
            crate::full_brightness_nits(),
            0.0,
        ),
        fog_color: Vec4::ZERO,
        fog_range: Vec4::ZERO,
    }
}

/// Every frame outdoors: the wind's phase for each grass, and once the
/// clock has moved a game minute (or new grass appeared) the light of the
/// hour: the weather's ambient and sunlight, the sun's direction, the
/// fog, the image space's grass dimmer (after the weather's modifiers) and
/// the wind's strength.
pub fn light_grass(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    exterior: Option<Res<Exterior>>,
    settings: Res<crate::Settings>,
    mut field: ResMut<GrassField>,
    mut materials: ResMut<Assets<GrassMaterial>>,
    mut weathers: ResMut<crate::weather::Weathers>,
) {
    let field = &mut *field;
    let Some(exterior) = exterior else {
        field.applied = None;
        return;
    };
    if field.types.is_empty() {
        return;
    }
    let order = &game.0.order;
    // The game state's weather, mid-fade or not (`crate::weather`).
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
    let grass_settings = field.settings.unwrap_or_else(|| game.0.grass_settings());
    let hour = state
        .0
        .global(order, "GameHour")
        .unwrap_or(world::weather::DEFAULT_HOUR);

    let relight = field.applied.is_none_or(|h| (h - hour).abs() >= 1.0 / 60.0);
    let light = relight.then(|| {
        let clock = world::weather::SkyClock::new(
            exterior.grid.climate.as_ref(),
            world::weather::SkySettings::load(order),
        );
        let light = weather.light_at(&clock, hour);
        let (ambient, directional, fog) = cellview::exterior_light(&light);
        let modifier = weather.modifier_at(order, &clock, hour);
        let dimmer = exterior
            .grid
            .image_space
            .as_ref()
            .and_then(world::grass::grass_dimmer)
            .map(|d| {
                modifier
                    .as_ref()
                    .map_or(d, |m| d * m.multiply[GRASS_DIMMER] + m.add[GRASS_DIMMER])
            })
            .unwrap_or(grass_settings.dimmer);
        let fields = crate::light_fields(
            ambient,
            directional.as_ref(),
            fog.as_ref(),
            settings.brightness,
        );
        let (sun_direction, sun_color) = match &directional {
            Some(d) => (
                Vec4::new(d.direction[0], d.direction[1], d.direction[2], 1.0),
                Vec4::new(
                    d.color[0] * settings.brightness,
                    d.color[1] * settings.brightness,
                    d.color[2] * settings.brightness,
                    1.0,
                ),
            ),
            None => (Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::ZERO),
        };
        (
            fields,
            sun_direction,
            sun_color,
            dimmer,
            grass_settings.wind_magnitude((weather.wind() * 255.0).round() as u8),
        )
    });
    if relight {
        field.applied = Some(hour);
    }

    for kind in field.types.values() {
        let Some(m) = materials.get_mut(&kind.material) else {
            continue;
        };
        let p = &mut m.extension.params;
        if let Some((fields, sun_direction, sun_color, dimmer, wind)) = &light {
            p.ambient = fields.ambient;
            p.sun_direction = *sun_direction;
            p.sun_color = *sun_color;
            p.fade.z = *dimmer;
            p.fog_color = fields.fog_color;
            p.fog_range = fields.fog_range;
            p.wind.z = *wind;
        }
        p.wind.w = wind_phase(kind.wave_period, hour);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> GrassModel {
        GrassModel {
            path: "meshes\\test\\grass.nif".into(),
            positions: vec![[0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            normals: vec![[0.0, -1.0, 0.0]; 3],
            uvs: vec![[0.0; 2]; 3],
            colors: vec![[1.0; 4]; 3],
            indices: vec![0, 1, 2],
            texture: None,
            alpha_test: 128.0 / 255.0,
            double_sided: true,
            radius: 1.0,
            sway: 1.0,
        }
    }

    #[test]
    fn grass_scales_every_axis_only_when_uniform() {
        let settings = GrassSettings {
            start_fade: 7000.0,
            fade_range: 1000.0,
            ..GrassSettings::default()
        };
        let mut grass =
            world::grass::Grass::from_data(FormId(1), None, Some("a.nif".into()), Some(&[30]));
        grass.flags = 0x06;
        let p = params(&grass, &model(), &settings);
        assert_eq!(p.scale_mask.truncate(), Vec3::ONE);
        assert_eq!((p.fade.x, p.fade.y), (7000.0, 1000.0));
        assert_eq!((p.flags.x, p.flags.y), (1.0, 0.0));
        // The wind blows north (game +y).
        assert_eq!((p.wind.x, p.wind.y), (0.0, 1.0));
        grass.flags = 0x01;
        let p = params(&grass, &model(), &settings);
        assert_eq!(p.scale_mask.truncate(), Vec3::Z);
        assert_eq!((p.flags.x, p.flags.y), (0.0, 1.0));
    }

    #[test]
    fn every_vertex_carries_its_blade() {
        let grass = world::grass::Grass::from_data(FormId(1), None, Some("a.nif".into()), None);
        let blade = |x: f32| world::grass::GrassInstance {
            data: [x, 0.5, 100.97, 3.5],
            brightness: 0.5,
            scale: 1.035,
        };
        let batch = world::grass::GrassBatch {
            grass,
            instances: vec![blade(10.5), blade(500.5)],
        };
        let mesh = bevy_mesh(&grass_mesh(&batch, &model(), 125.0));
        assert_eq!(mesh.count_vertices(), 6);
        let Some(bevy::render::mesh::VertexAttributeValues::Float32x4(instances)) =
            mesh.attribute(ATTRIBUTE_INSTANCE)
        else {
            panic!("no blade attribute");
        };
        assert_eq!(instances[3][0], 500.5);
        assert_eq!(mesh.indices().map(|i| i.len()), Some(6));
    }
}
