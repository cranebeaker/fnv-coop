//! The Caravan menu's table on screen (`cellview::caravan`, run by
//! `game_menus::caravan`). Drawn as the game's `Draw3DElements`
//! (`00740f30`) draws it: the depth cleared, every model in one pass seen by
//! the table's camera, after the image space pass and under the menus'
//! pictures; here into the HUD's picture by a camera of its own before the
//! HUD's (which then blends over it, `game_menus::compose_hud_over_scene`).
//!
//! Each frame every piece is put where its model's pose has it (a hidden
//! node hides it; the deck screen's models only while that screen's in, the
//! game's only once the game is), the money on its spots, the card
//! textures the menu set on their shapes, the table's lights where its
//! camera rig has them.
//!
//! The camera keeps the file's frustum (45° across, 16:9) on any shape of
//! window, stretched as the game's renderer stretches it: Gamebryo maps a
//! camera's frustum onto its viewport (the whole screen) as it is, and
//! nothing in the menu (`PrepareShared3DElements` `0073cbf0`,
//! `Draw3DElements` `00740f30`) fits it to the screen ([`FileFrustum`]).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, Exposure, RenderTarget};
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::renderer::RenderDevice;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use cellview::caravan::{CaravanModels, Part};
use cellview::space;
use nif::math::Transform as NifTransform;

use crate::game_menus::caravan::CaravanScreen;
use crate::game_menus::{GameMenus, OpenMenu};
use crate::hud::HudLayer;
use crate::lighting::GameLitMaterial;
use crate::GameFiles;

/// The render layer the table's pieces are drawn on.
const TABLE_LAYER: usize = 28;

/// A piece of the table: its model (index into the models' parts), the
/// money piece it belongs to, its mesh and shape, where its mesh is
/// centred.
#[derive(Component)]
struct TablePiece {
    part: usize,
    money: Option<usize>,
    shape: String,
    center: Vec3,
    material: Handle<GameLitMaterial>,
}

/// What's on screen.
struct Shown {
    models: Arc<CaravanModels>,
    camera: Entity,
    entities: Vec<Entity>,
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<GameLitMaterial>>,
    images: Vec<Handle<Image>>,
    textures: Vec<Option<Handle<Image>>>,
    /// Card textures by path.
    cards: HashMap<String, Option<Handle<Image>>>,
    /// The money pieces with pieces made.
    money: usize,
    texture_changes: u64,
    lights: Vec<cellview::LightData>,
}

#[derive(Resource, Default)]
pub struct TableShown {
    shown: Option<Shown>,
    layer: Option<Handle<Image>>,
}

pub struct CaravanTablePlugin;

impl Plugin for CaravanTablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TableShown>().add_systems(
            Update,
            show_table
                // Kept: the table drawn for the menu as it ran this frame,
                // inside the interface idle's set (`crate::frame_order`).
                .after(crate::menus::run_menus)
                .in_set(crate::frame_order::IdleSet),
        );
    }
}

/// The open Caravan menu, if any.
fn caravan(menus: &GameMenus) -> Option<&CaravanScreen> {
    menus.screen.as_deref()?.open.iter().find_map(|m| match m {
        OpenMenu::Caravan(c) => Some(&**c),
        _ => None,
    })
}

/// Where the camera puts the player's first grid's `Select1_01:0` across
/// the screen (0 to 1; `NiCamera::WorldPtToScreenPt` `00a6fc50`), once
/// the hands are dealt (state 2's end).
pub fn track_x(c: &CaravanScreen) -> Option<f32> {
    use world::caravan::menu::{state, Screen};
    if c.game.screen != Screen::Game || c.game.state == state::CAMERA_TO_GAME {
        return None;
    }
    let spot = c.models.grids.first()?.get(&(0, 0))?;
    let table = c.table.poses.get(&Part::Table).cloned().unwrap_or_default();
    let camera = c.models.camera(&table)?;
    let f = c.models.frustum?;
    // Into the camera's space: x ahead, y up, z right.
    let local = camera.inverse().apply_point(spot.center);
    if local[0] <= 1e-5 {
        return None;
    }
    let across = local[2] / local[0];
    Some((across - f.left) / (f.right - f.left))
}

/// The table camera's projection: the file's frustum as it is, whatever
/// the window's shape (a Gamebryo camera's frustum fills its viewport).
/// `left`..`right` and `bottom`..`top` are the frustum's sides at a
/// distance of 1; `near` and `far` in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileFrustum {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl FileFrustum {
    /// From the table's camera; without one, 45° across at 16:9, near 1,
    /// far 5000 (what the file has).
    pub fn new(f: Option<nif::camera::Frustum>) -> FileFrustum {
        let (left, right, top, bottom, near, far) = match f {
            Some(f) => (f.left, f.right, f.top, f.bottom, f.near, f.far),
            None => {
                let r = 22.5f32.to_radians().tan();
                (-r, r, r * 9.0 / 16.0, -r * 9.0 / 16.0, 1.0, 5000.0)
            }
        };
        FileFrustum {
            left,
            right,
            top,
            bottom,
            near: near * space::METERS_PER_UNIT,
            far: far * space::METERS_PER_UNIT,
        }
    }

    /// The sides at a distance (`left`, `right`, `bottom`, `top`).
    fn at(&self, z: f32) -> [f32; 4] {
        [self.left * z, self.right * z, self.bottom * z, self.top * z]
    }
}

impl bevy::render::camera::CameraProjection for FileFrustum {
    /// Bevy's reversed, infinite-far depth (as its own perspective), off
    /// centre as the file's sides say.
    fn get_clip_from_view(&self) -> Mat4 {
        let (w, h) = (self.right - self.left, self.top - self.bottom);
        Mat4::from_cols(
            Vec4::new(2.0 / w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 / h, 0.0, 0.0),
            Vec4::new(
                (self.right + self.left) / w,
                (self.top + self.bottom) / h,
                0.0,
                -1.0,
            ),
            Vec4::new(0.0, 0.0, self.near, 0.0),
        )
    }

    fn get_clip_from_view_for_sub(&self, _sub: &bevy::render::camera::SubCameraView) -> Mat4 {
        self.get_clip_from_view()
    }

    /// The window's shape doesn't change it.
    fn update(&mut self, _width: f32, _height: f32) {}

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [bevy::math::Vec3A; 8] {
        let corners = |z: f32| {
            let z = z.abs();
            let [l, r, b, t] = self.at(z);
            [
                bevy::math::Vec3A::new(r, b, -z),
                bevy::math::Vec3A::new(r, t, -z),
                bevy::math::Vec3A::new(l, t, -z),
                bevy::math::Vec3A::new(l, b, -z),
            ]
        };
        let (n, f) = (corners(z_near), corners(z_far));
        [n[0], n[1], n[2], n[3], f[0], f[1], f[2], f[3]]
    }
}

/// The camera's Bevy transform from its place in the game's space
/// (Gamebryo cameras look along their +x with +y up).
pub(crate) fn camera_transform(t: &NifTransform) -> Transform {
    let origin = t.apply_point([0.0; 3]);
    let along = |v: [f32; 3]| {
        let p = t.apply_point(v);
        Vec3::from(space::direction([
            p[0] - origin[0],
            p[1] - origin[1],
            p[2] - origin[2],
        ]))
        .normalize_or_zero()
    };
    let forward = along([1.0, 0.0, 0.0]);
    let up = along([0.0, 1.0, 0.0]);
    Transform::from_translation(Vec3::from(space::point(origin))).looking_to(forward, up)
}

#[allow(clippy::too_many_arguments)]
fn show_table(
    mut commands: Commands,
    game: Res<GameFiles>,
    settings: Res<crate::Settings>,
    menus: Res<GameMenus>,
    mut shown: ResMut<TableShown>,
    hud_layer: Option<Res<HudLayer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
    mut images: ResMut<Assets<Image>>,
    device: Option<Res<RenderDevice>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut pieces: Query<(&TablePiece, &mut Transform, &mut Visibility)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<TablePiece>)>,
) {
    let open = caravan(&menus);
    // While the table's drawn the HUD's camera lays its pictures over it
    // (blended): `game_menus::compose_hud_over_scene`, for every menu with
    // a 3D scene.
    let TableShown { shown, layer } = &mut *shown;
    let stale = match (&*shown, open) {
        (Some(s), Some(c)) => !Arc::ptr_eq(&s.models, &c.models),
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
        }
    }
    let Some(c) = open else {
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
    let models = c.models.clone();
    let table_pose = c.table.poses.get(&Part::Table).cloned().unwrap_or_default();
    let lights = models.lights_now(&table_pose);
    if shown.is_none() {
        let target = crate::lockpick::menu_layer(
            &mut commands,
            hud_layer.as_deref(),
            layer,
            &mut images,
            size,
        );
        let projection = FileFrustum::new(models.frustum);
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
                    // The picture cleared, the table written into it.
                    output_mode: CameraOutputMode::Write {
                        blend_state: None,
                        clear_color: ClearColorConfig::Custom(Color::NONE),
                    },
                    ..default()
                },
                Tonemapping::None,
                DebandDither::Disabled,
                Projection::custom(projection),
                Exposure {
                    ev100: crate::START_EV100,
                },
                Transform::IDENTITY,
                RenderLayers::layer(TABLE_LAYER),
            ))
            .id();
        let textures: Vec<Option<Handle<Image>>> = models
            .scene
            .textures
            .iter()
            .map(|t| crate::upload_texture(&mut images, t, compressed, settings.anisotropy))
            .collect();
        let mut s = Shown {
            models: models.clone(),
            camera,
            entities: vec![camera],
            meshes: Vec::new(),
            materials: Vec::new(),
            images: textures.iter().flatten().cloned().collect(),
            textures,
            cards: HashMap::new(),
            money: 0,
            texture_changes: u64::MAX,
            lights: lights.clone(),
        };
        // Every model's pieces but the money's (made per piece below).
        let draws: Vec<usize> = (0..models.scene.draws.len()).collect();
        spawn_pieces(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut s,
            &draws,
            None,
            &lights,
            settings.brightness,
        );
        *shown = Some(s);
        // The pieces are there from the next frame (the commands run after
        // this): placed and textured then, so a texture set as the menu
        // opens isn't lost (as `casino_scene`).
        return;
    }
    let Some(s) = shown.as_mut() else {
        return;
    };
    // New money: its kind's pieces.
    while s.money < c.table.money.len() {
        let part = c.table.money[s.money].part;
        let index = models.parts.iter().position(|&p| p == part);
        let draws: Vec<usize> = models
            .scene
            .draws
            .iter()
            .enumerate()
            .filter(|(_, d)| Some(d.reference as usize) == index.map(|i| i + 1))
            .map(|(i, _)| i)
            .collect();
        let money = s.money;
        spawn_pieces(
            &mut commands,
            &mut meshes,
            &mut materials,
            s,
            &draws,
            Some(money),
            &lights,
            settings.brightness,
        );
        s.money += 1;
    }
    // The camera.
    if let Some(t) = models.camera(&table_pose) {
        if let Ok(mut transform) = cameras.get_mut(s.camera) {
            let wanted = camera_transform(&t);
            if *transform != wanted {
                *transform = wanted;
            }
        }
    }
    // The lights move with the camera rig.
    if s.lights != lights {
        s.lights = lights.clone();
        let lighting = crate::lockpick::menu_lighting(&lights, settings.brightness);
        for m in &s.materials {
            if let Some(mat) = materials.get_mut(m) {
                mat.extension.lighting = lighting;
            }
        }
    }
    // Card textures.
    let textures_changed = s.texture_changes != c.table.texture_changes;
    s.texture_changes = c.table.texture_changes;
    // The pieces.
    for (piece, mut transform, mut visibility) in &mut pieces {
        let part = models.parts[piece.part];
        let attached = match part {
            Part::Table => true,
            Part::Deck | Part::Available => c.table.deck_models,
            Part::Bill(_) | Part::Coin(_) => piece.money.is_some(),
            _ => c.table.game_models,
        };
        let pose = match piece.money {
            Some(i) => c.table.money.get(i).map(|m| &m.pose),
            None => c.table.poses.get(&part),
        };
        let default = cellview::caravan::Pose::default();
        let pose = pose.unwrap_or(&default);
        let model = &models.models[piece.part];
        let moved = attached
            .then(|| pose.shape_move(model, &piece.shape))
            .flatten();
        let wanted_visibility = if moved.is_some() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != wanted_visibility {
            *visibility = wanted_visibility;
        }
        if let Some(m) = moved {
            let matrix = cellview::column_major(&m);
            let wanted = Transform::from_matrix(
                Mat4::from_cols_array(&space::matrix(&matrix))
                    * Mat4::from_translation(piece.center),
            );
            if *transform != wanted {
                *transform = wanted;
            }
        }
        if textures_changed {
            let key = (part, piece.shape.to_ascii_lowercase());
            if let Some(path) = c.table.textures.get(&key) {
                let handle = s
                    .cards
                    .entry(path.clone())
                    .or_insert_with(|| {
                        cellview::caravan::card_texture(&game.0.assets, path).and_then(|t| {
                            crate::upload_texture(&mut images, &t, compressed, settings.anisotropy)
                        })
                    })
                    .clone();
                if let (Some(h), Some(mat)) = (handle, materials.get_mut(&piece.material)) {
                    if mat.base.base_color_texture.as_ref() != Some(&h) {
                        mat.base.base_color_texture = Some(h);
                    }
                }
            }
        }
    }
}

/// Pieces for draws (of the models, or of one money piece).
#[allow(clippy::too_many_arguments)]
fn spawn_pieces(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<GameLitMaterial>,
    s: &mut Shown,
    draws: &[usize],
    money: Option<usize>,
    lights: &[cellview::LightData],
    brightness: f32,
) {
    let models = s.models.clone();
    let lighting = crate::lockpick::menu_lighting(lights, brightness);
    for &d in draws {
        let draw = &models.scene.draws[d];
        let Some(part) = (draw.reference as usize).checked_sub(1) else {
            continue;
        };
        // The money's templates are drawn only as money.
        if money.is_none() && matches!(models.parts.get(part), Some(Part::Bill(_) | Part::Coin(_)))
        {
            continue;
        }
        let data = &models.scene.meshes[draw.mesh];
        let center = crate::sort_center(data).unwrap_or([0.0; 3]);
        let mesh = meshes.add(crate::game_mesh_around(data, center));
        let material = materials.add(crate::lit_material(data, &s.textures, lighting));
        s.meshes.push(mesh.clone());
        s.materials.push(material.clone());
        let e = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
                Visibility::Hidden,
                RenderLayers::layer(TABLE_LAYER),
                crate::shared_light::MenuLit,
                TablePiece {
                    part,
                    money,
                    shape: data.shape_name.clone(),
                    center: Vec3::from(center),
                    material,
                },
            ))
            .id();
        s.entities.push(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::camera::CameraProjection;

    /// The file's frustum is Bevy's own perspective at 16:9, and stays so
    /// whatever the window: on a 4:3 or 21:9 window it's stretched.
    #[test]
    fn the_file_frustum_ignores_the_window() {
        let r = 22.5f32.to_radians().tan();
        let f = FileFrustum::new(Some(nif::camera::Frustum {
            left: -r,
            right: r,
            top: r * 9.0 / 16.0,
            bottom: -r * 9.0 / 16.0,
            near: 1.0,
            far: 5000.0,
            ortho: false,
        }));
        let mut bevy = PerspectiveProjection {
            fov: 2.0 * (r * 9.0 / 16.0).atan(),
            near: f.near,
            ..default()
        };
        bevy.update(1920.0, 1080.0);
        let mut fixed = f;
        fixed.update(1024.0, 768.0);
        let (a, b) = (fixed.get_clip_from_view(), bevy.get_clip_from_view());
        assert!(a.abs_diff_eq(b, 1e-5), "{a} {b}");
        // A point at the frustum's right edge lands on the screen's edge.
        let p = a * Vec4::new(r * 10.0, 0.0, -10.0, 1.0);
        assert!((p.x / p.w - 1.0).abs() < 1e-5);
        // The corners at distance 2: the sides × 2.
        let c = fixed.get_frustum_corners(-2.0, -4.0);
        assert!((c[0].x - 2.0 * r).abs() < 1e-6 && (c[2].y - 2.0 * r * 9.0 / 16.0).abs() < 1e-6);
        assert_eq!(fixed.far(), 5000.0 * space::METERS_PER_UNIT);
    }
}
