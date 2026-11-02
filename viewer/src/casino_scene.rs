//! The casino games' 3D on screen: the slot machine (`cellview::slots`, run
//! by `game_menus::slots`), the blackjack table (`cellview::blackjack`,
//! `game_menus::blackjack`) and the roulette table (`cellview::roulette`,
//! `game_menus::roulette`, its camera riding the table's spin). Drawn as
//! their `Draw3DElements` (slots `007c19c0`, blackjack `00733500`, roulette
//! `007bbcf0`, Caravan's body) draw them: their own camera over the scene,
//! the depth cleared, under the menus' pictures; here, as the Caravan table
//! (`caravan_table`), into the HUD's picture by a camera of its own before
//! the HUD's, which then lays its pictures over it.
//!
//! The slot machine's camera is the lockpicking menu's (`007c2180`: at
//! the node's origin looking along +x) and its node is turned and moved
//! (`cellview::slots::root_transform`); blackjack's is the table's own
//! `object1` (roulette's `object4`) with the file's frustum, its node where
//! the models are.
//! Each frame every piece is put where its model's pose has it (a culled
//! node hides it), and the shapes take the textures the menu put on them
//! (the reels' faces, the cards).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, Exposure, RenderTarget};
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::renderer::RenderDevice;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use cellview::caravan::{ModelData, Pose};
use cellview::lockpick::MenuCamera;
use cellview::{space, LightData, TextureData, ViewerScene};
use nif::math::Transform as NifTransform;

use crate::game_menus::{GameMenus, OpenMenu};
use crate::hud::HudLayer;
use crate::lighting::GameLitMaterial;
use crate::GameFiles;

/// The render layer the casino games' pieces are drawn on.
const CASINO_LAYER: usize = 27;

/// A piece: its model (its index in the scene's node), its shape, where
/// its mesh is centred.
#[derive(Component)]
struct CasinoPiece {
    model: usize,
    shape: String,
    center: Vec3,
    material: Handle<GameLitMaterial>,
}

/// Where a shape's texture comes from.
enum Texture<'a> {
    /// One of the slot machine's reel textures.
    Reel(usize),
    /// A file under `Data\` (a card).
    File(&'a str),
}

/// How the scene is seen.
enum Eye {
    /// The lockpicking menu's camera.
    Menu,
    /// A camera in the scene's space with the file's frustum.
    Placed(NifTransform, Option<nif::camera::Frustum>),
}

/// A shape's texture by its model and name (lower case).
type TextureOf<'a> = Box<dyn Fn(usize, &str) -> Option<Texture<'a>> + 'a>;

/// What the open game shows this frame.
struct View<'a> {
    /// The models' identity (a new menu, a new scene).
    id: usize,
    scene: &'a ViewerScene,
    models: &'a [ModelData],
    poses: &'a [Pose],
    root: NifTransform,
    eye: Eye,
    lights: Vec<LightData>,
    reel_textures: &'a [Option<TextureData>],
    texture: TextureOf<'a>,
    texture_changes: u64,
}

/// The open casino game's view, if any.
fn view(menus: &GameMenus) -> Option<View<'_>> {
    menus.screen.as_deref()?.open.iter().find_map(|m| match m {
        OpenMenu::Slots(s) if !s.closed => Some(View {
            id: Arc::as_ptr(&s.models) as usize,
            scene: &s.models.scene,
            models: &s.models.models,
            poses: &s.poses,
            root: cellview::slots::root_transform(),
            eye: Eye::Menu,
            lights: s.models.lights_now(),
            reel_textures: &s.models.reel_textures,
            texture: Box::new(|model, shape| {
                s.faces
                    .get(&(model, shape.to_string()))
                    .map(|&slot| Texture::Reel(slot))
            }),
            texture_changes: s.texture_changes,
        }),
        OpenMenu::Blackjack(b) if !b.closed => {
            let table = b.table.poses.first()?;
            Some(View {
                id: Arc::as_ptr(&b.models) as usize,
                scene: &b.models.scene,
                models: &b.models.models,
                poses: &b.table.poses,
                root: NifTransform::IDENTITY,
                eye: Eye::Placed(b.models.camera(table)?, b.models.frustum),
                lights: b.models.lights_now(table),
                reel_textures: &[],
                texture: Box::new(|model, shape| {
                    b.table
                        .textures
                        .get(&(model, shape.to_string()))
                        .map(|p| Texture::File(p))
                }),
                texture_changes: b.table.texture_changes,
            })
        }
        OpenMenu::Roulette(r) if !r.closed => {
            let table = r.poses.first()?;
            Some(View {
                id: Arc::as_ptr(&r.models) as usize,
                scene: &r.models.scene,
                models: &r.models.models,
                poses: &r.poses,
                root: NifTransform::IDENTITY,
                eye: Eye::Placed(r.models.camera(table)?, r.models.frustum),
                lights: r.models.lights_now(table),
                reel_textures: &[],
                texture: Box::new(|_, _| None),
                texture_changes: 0,
            })
        }
        _ => None,
    })
}

/// What's on screen.
struct Shown {
    id: usize,
    camera: Entity,
    entities: Vec<Entity>,
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<GameLitMaterial>>,
    images: Vec<Handle<Image>>,
    /// The reels' textures by slot, the files' by path.
    reel_images: Vec<Option<Handle<Image>>>,
    files: HashMap<String, Option<Handle<Image>>>,
    texture_changes: u64,
    lights: Vec<LightData>,
}

#[derive(Resource, Default)]
pub struct CasinoShown {
    shown: Option<Shown>,
    layer: Option<Handle<Image>>,
}

pub struct CasinoScenePlugin;

impl Plugin for CasinoScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CasinoShown>().add_systems(
            Update,
            show_casino
                // Kept: the table drawn for the menu as it ran this frame,
                // inside the interface idle's set (`crate::frame_order`).
                .after(crate::menus::run_menus)
                .in_set(crate::frame_order::IdleSet),
        );
    }
}

/// A piece's Bevy transform: the model's shape where its pose has it,
/// under the node.
fn piece_transform(v: &View, piece: &CasinoPiece) -> Option<Transform> {
    let model = v.models.get(piece.model)?;
    let moved = v.poses.get(piece.model)?.shape_move(model, &piece.shape)?;
    let m = v.root.then_child(&moved);
    let matrix = cellview::column_major(&m);
    Some(Transform::from_matrix(
        Mat4::from_cols_array(&space::matrix(&matrix)) * Mat4::from_translation(piece.center),
    ))
}

/// The camera's Bevy transform and projection.
fn camera_for(eye: &Eye, settings: &assets::IniSettings, size: UVec2) -> (Transform, Projection) {
    match eye {
        Eye::Menu => {
            let camera = MenuCamera::new(settings, size.x, size.y);
            (
                // Camera +x (ahead) is Bevy's +x, +y (up) Bevy's −z
                // (`space`'s conversion).
                Transform::from_translation(Vec3::ZERO).looking_to(Vec3::X, Vec3::NEG_Z),
                Projection::from(PerspectiveProjection {
                    fov: camera.vertical_fov(),
                    near: camera.near * space::METERS_PER_UNIT,
                    far: 100.0,
                    ..default()
                }),
            )
        }
        Eye::Placed(t, frustum) => {
            // The file's 45° across kept, the height following the window
            // (as `caravan_table`).
            let (fov, near, far) = frustum
                .map_or((45f32.to_radians() * 9.0 / 16.0, 1.0, 5000.0), |f| {
                    (2.0 * f.top.atan(), f.near, f.far)
                });
            (
                crate::caravan_table::camera_transform(t),
                Projection::from(PerspectiveProjection {
                    fov,
                    near: near * space::METERS_PER_UNIT,
                    far: far * space::METERS_PER_UNIT,
                    ..default()
                }),
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn show_casino(
    mut commands: Commands,
    game: Res<GameFiles>,
    settings: Res<crate::Settings>,
    menus: Res<GameMenus>,
    mut shown: ResMut<CasinoShown>,
    hud_layer: Option<Res<HudLayer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
    mut images: ResMut<Assets<Image>>,
    device: Option<Res<RenderDevice>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut pieces: Query<(&CasinoPiece, &mut Transform, &mut Visibility)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CasinoPiece>)>,
) {
    let open = view(&menus);
    let CasinoShown { shown, layer } = &mut *shown;
    let stale = match (&*shown, &open) {
        (Some(s), Some(v)) => s.id != v.id,
        (Some(_), None) => true,
        _ => false,
    };
    if stale {
        if let Some(s) = shown.take() {
            for e in s.entities {
                commands.entity(e).despawn();
            }
            for m in s.meshes {
                meshes.remove(&m);
            }
            for m in s.materials {
                materials.remove(&m);
            }
            for i in s.images {
                images.remove(&i);
            }
            for i in s.files.into_values().flatten() {
                images.remove(&i);
            }
        }
    }
    let Some(v) = open else {
        return;
    };
    let size = windows
        .single()
        .map(|w| UVec2::new(w.physical_width(), w.physical_height()))
        .unwrap_or(UVec2::new(1920, 1080))
        .max(UVec2::ONE);
    let compressed = device
        .as_ref()
        .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
    let (camera_at, projection) = camera_for(&v.eye, &game.0.settings, size);
    if shown.is_none() {
        let target = crate::lockpick::menu_layer(
            &mut commands,
            hud_layer.as_deref(),
            layer,
            &mut images,
            size,
        );
        let camera = commands
            .spawn((
                Camera3d::default(),
                Camera {
                    target: RenderTarget::from(target),
                    // Before the HUD's camera (−4), which draws the menus'
                    // pictures over it.
                    order: -5,
                    hdr: true,
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    output_mode: CameraOutputMode::Write {
                        blend_state: None,
                        clear_color: ClearColorConfig::Custom(Color::NONE),
                    },
                    ..default()
                },
                Tonemapping::None,
                DebandDither::Disabled,
                projection,
                Exposure {
                    ev100: crate::START_EV100,
                },
                camera_at,
                RenderLayers::layer(CASINO_LAYER),
            ))
            .id();
        let upload = |images: &mut Assets<Image>, t: &TextureData| {
            crate::upload_texture(images, t, compressed, settings.anisotropy)
        };
        let textures: Vec<Option<Handle<Image>>> = v
            .scene
            .textures
            .iter()
            .map(|t| upload(&mut images, t))
            .collect();
        let reel_images: Vec<Option<Handle<Image>>> = v
            .reel_textures
            .iter()
            .map(|t| t.as_ref().and_then(|t| upload(&mut images, t)))
            .collect();
        let mut s = Shown {
            id: v.id,
            camera,
            entities: vec![camera],
            meshes: Vec::new(),
            materials: Vec::new(),
            images: textures
                .iter()
                .chain(reel_images.iter())
                .flatten()
                .cloned()
                .collect(),
            reel_images,
            files: HashMap::new(),
            texture_changes: u64::MAX,
            lights: v.lights.clone(),
        };
        let lighting = crate::lockpick::menu_lighting(&v.lights, settings.brightness);
        for draw in &v.scene.draws {
            let Some(model) = (draw.reference as usize).checked_sub(1) else {
                continue;
            };
            let data = &v.scene.meshes[draw.mesh];
            let center = crate::sort_center(data).unwrap_or([0.0; 3]);
            let mesh = meshes.add(crate::game_mesh_around(data, center));
            let material = materials.add(crate::lit_material(data, &textures, lighting));
            s.meshes.push(mesh.clone());
            s.materials.push(material.clone());
            let e = commands
                .spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material.clone()),
                    Transform::IDENTITY,
                    Visibility::Hidden,
                    RenderLayers::layer(CASINO_LAYER),
                    crate::shared_light::MenuLit,
                    CasinoPiece {
                        model,
                        shape: data.shape_name.clone(),
                        center: Vec3::from(center),
                        material,
                    },
                ))
                .id();
            s.entities.push(e);
        }
        *shown = Some(s);
        // The pieces are there from the next frame (the commands run after
        // this): placed and textured then.
        return;
    }
    let Some(s) = shown.as_mut() else {
        return;
    };
    if let Ok(mut t) = cameras.get_mut(s.camera) {
        if *t != camera_at {
            *t = camera_at;
        }
    }
    if s.lights != v.lights {
        s.lights = v.lights.clone();
        let lighting = crate::lockpick::menu_lighting(&v.lights, settings.brightness);
        for m in &s.materials {
            if let Some(mat) = materials.get_mut(m) {
                mat.extension.lighting = lighting;
            }
        }
    }
    let textures_changed = s.texture_changes != v.texture_changes;
    s.texture_changes = v.texture_changes;
    for (piece, mut transform, mut visibility) in &mut pieces {
        let wanted = piece_transform(&v, piece);
        let wanted_visibility = if wanted.is_some() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted_visibility {
            *visibility = wanted_visibility;
        }
        if let Some(t) = wanted {
            if *transform != t {
                *transform = t;
            }
        }
        if !textures_changed {
            continue;
        }
        let image = match (v.texture)(piece.model, &piece.shape.to_ascii_lowercase()) {
            Some(Texture::Reel(slot)) => s.reel_images.get(slot).cloned().flatten(),
            Some(Texture::File(path)) => s
                .files
                .entry(path.to_string())
                .or_insert_with(|| {
                    let t = cellview::blackjack::card_texture(&game.0.assets, path)?;
                    crate::upload_texture(&mut images, &t, compressed, settings.anisotropy)
                })
                .clone(),
            None => None,
        };
        if let (Some(h), Some(mat)) = (image, materials.get_mut(&piece.material)) {
            if mat.base.base_color_texture.as_ref() != Some(&h) {
                mat.base.base_color_texture = Some(h);
            }
        }
    }
}
