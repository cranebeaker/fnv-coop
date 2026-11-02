//! LoveTesterMenu's original XML, model scene and live SPECIAL values.
//! Executable evidence and unimplemented branches: docs/VIGOR.md.

use super::{OpenMenu, Screen};
use bevy::prelude::*;
use cellview::vigor::{DisplayState, VigorScene};
use std::sync::Arc;
use ui::menus::vigor::{InputIntent, Request, VigorMenu, FILE};
use world::chargen::CharacterMenu;
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState};

pub struct VigorScreen {
    pub menu: VigorMenu,
    pub models: Arc<VigorScene>,
    pub size: UVec2,
    pub sequence: Option<usize>,
    pub time: f32,
    pub pointer: Option<Vec2>,
}

pub fn takes(request: &crate::menus::Menu) -> bool {
    matches!(
        request,
        crate::menus::Menu::Character(CharacterMenu::Special { .. })
    )
}

fn values(game: &cellview::Game, state: &GameState) -> [i32; 7] {
    let facts = Facts {
        order: &game.order,
        state,
        speaker: None,
    };
    std::array::from_fn(|i| {
        facts
            .current_actor_value(PLAYER_REF, 5 + i as u16)
            .unwrap_or(0.0) as i32
    })
}

impl VigorScreen {
    pub fn display(&self, game: &cellview::Game, state: &GameState) -> DisplayState {
        DisplayState {
            page: self.menu.model.page,
            selection: self.menu.model.selection,
            controller: false,
            values: values(game, state),
            budget: self.menu.model.budget,
        }
    }

    pub fn layer(&self) -> Option<(&nif::Sequence, f32)> {
        self.sequence.map(|i| {
            let s = &self.models.sequences[i];
            (s, s.start + self.time.clamp(0.0, s.stop - s.start))
        })
    }

    fn apply(
        &mut self,
        requests: Vec<Request>,
        game: &cellview::Game,
        state: &mut GameState,
        sounds: &mut Vec<esm::FormId>,
    ) {
        for request in requests {
            match request {
                Request::Sound(name) => {
                    if let Some(id) = game.order.form_by_editor_id(name) {
                        sounds.push(id);
                    }
                }
                Request::SetActorValue { av, value } => {
                    state
                        .actor_values
                        .insert((PLAYER_REF, av), f64::from(value));
                }
                Request::Page { page, forward } => {
                    self.sequence = cellview::vigor::page_sequence(page, forward)
                        .and_then(|name| self.models.sequences.iter().position(|s| s.name == name));
                    self.time = 0.0;
                }
                Request::Close => self.menu.mark_closed(),
            }
        }
    }

    /// Carry out every click before another key uses its value snapshot.
    pub fn inputs(&mut self, game: &cellview::Game, state: &mut GameState) -> Vec<esm::FormId> {
        let mut sounds = Vec::new();
        for input in self.menu.take_inputs() {
            if self.menu.closed {
                break;
            }
            match input {
                InputIntent::TileClick(id) => {
                    let before = values(game, state);
                    let av = self.menu.model.actor_value(false);
                    let requests = self.menu.model.tile_click(id, &before, false);
                    self.apply(requests, game, state, &mut sounds);
                    if matches!(id, 2 | 3) {
                        if let Some(av) = av {
                            let after = values(game, state)[usize::from(av - 5)];
                            let old = before[usize::from(av - 5)];
                            let name = if after < old {
                                Some("OBJBookSpecialNumberDown")
                            } else if after > old {
                                Some("OBJBookSpecialNumber")
                            } else {
                                None
                            };
                            if let Some(name) = name {
                                self.apply(vec![Request::Sound(name)], game, state, &mut sounds);
                            }
                        }
                    }
                }
                InputIntent::ModelPick => {
                    if let Some(pointer) = self.pointer {
                        let camera =
                            cellview::vigor::camera(&game.settings, self.size.x, self.size.y);
                        let ray = Vec3::new(
                            (2.0 * pointer.x / self.size.x as f32 - 1.0) * camera.tan_half_width,
                            (1.0 - 2.0 * pointer.y / self.size.y as f32) * camera.tan_half_height,
                            -1.0,
                        )
                        .normalize();
                        for page_specific in [true, false] {
                            let display = self.display(game, state);
                            let picked = self
                                .models
                                .pick_button(
                                    page_specific,
                                    &display,
                                    self.size.x,
                                    [0.0, 0.0, 1.0],
                                    ray.to_array(),
                                    self.layer(),
                                )
                                .map(str::to_string);
                            if let Some(name) = picked {
                                let requests = self.menu.model.model_button(
                                    &name,
                                    &values(game, state),
                                    false,
                                );
                                self.apply(requests, game, state, &mut sounds);
                            }
                            if self.menu.closed {
                                break;
                            }
                        }
                    }
                }
            }
        }
        sounds
    }

    pub fn key(
        &mut self,
        code: u32,
        game: &cellview::Game,
        state: &mut GameState,
    ) -> (bool, Vec<esm::FormId>) {
        let used = self.menu.keyboard(code, &values(game, state), false);
        (used, self.inputs(game, state))
    }
}

pub fn open(
    screen: &mut Screen,
    game: &cellview::Game,
    state: &mut GameState,
    request: crate::menus::Menu,
) -> Vec<esm::FormId> {
    let crate::menus::Menu::Character(CharacterMenu::Special { points }) = request else {
        return Vec::new();
    };
    let models = match VigorScene::load(&game.assets) {
        Ok(models) => Arc::new(models),
        Err(e) => {
            println!("Vit-o-matic menu can't be shown: {e}");
            return Vec::new();
        }
    };
    let mut menu = VigorMenu::new(0, points as i32);
    let tile = match screen.load(game, FILE, &mut menu) {
        Ok(tile) => tile,
        Err(e) => {
            println!("Vit-o-matic menu can't be shown: {e}");
            return Vec::new();
        }
    };
    menu.menu = tile;
    if let Some(missing) = menu.tiles.iter().position(Option::is_none) {
        println!("Vit-o-matic XML is missing tile id{missing}");
        screen.ui.detach(tile);
        return Vec::new();
    }
    screen.ui.set_number(tile, ui::names::t::VISIBLE, 1.0);
    let mut opened = VigorScreen {
        menu,
        models,
        size: screen.size,
        sequence: None,
        time: 0.0,
        pointer: None,
    };
    let requests = opened.menu.model.navigate(true, &values(game, state));
    let mut sounds = Vec::new();
    opened.apply(requests, game, state, &mut sounds);
    screen.open.push(OpenMenu::Vigor(Box::new(opened)));
    println!("Vit-o-matic: original models and LoveTesterMenu opened.");
    sounds
}

pub fn update(screen: &mut Screen, dt: f32) {
    for opened in &mut screen.open {
        let OpenMenu::Vigor(vigor) = opened else {
            continue;
        };
        vigor.time += dt;
        let sequence = vigor.layer().map(|(s, at)| (at, s.stop));
        vigor.menu.model.update_ready(sequence);
    }
}

use crate::lighting::GameLitMaterial;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::render::camera::{CameraOutputMode, Exposure, RenderTarget};
use bevy::render::render_resource::{BlendState, WgpuFeatures};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::RenderLayers;
use cellview::space;

const SCENE_LAYER: usize = 27;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_output_preserves_transparency_independent_of_other_cameras() {
        let camera = scene_camera(RenderTarget::default());
        let CameraOutputMode::Write {
            blend_state,
            clear_color,
        } = camera.output_mode
        else {
            panic!("the scene must reach the HUD picture");
        };
        // None lets Bevy select ALPHA_BLENDING from unrelated camera query
        // order. Over an opaque clear, even transparent input becomes opaque.
        assert_eq!(blend_state, Some(BlendState::REPLACE));
        assert!(matches!(clear_color, ClearColorConfig::Custom(c) if c == Color::NONE));
        assert!(matches!(camera.clear_color, ClearColorConfig::Custom(c) if c == Color::NONE));
    }

    /// The tester's menu open on a generated room's data (its models and
    /// pictures stand-in files of the right names), with the game.
    fn opened_tester(tag: &str) -> (testdata::TempData, cellview::Game, VigorScreen) {
        let data = testdata::room(tag);
        let model = std::fs::read(data.path().join("meshes/test/floor.nif")).unwrap();
        let texture = std::fs::read(data.path().join("textures/test/floor.dds")).unwrap();
        for path in [
            cellview::vigor::ACTIVATE_MODEL,
            cellview::vigor::CABINET_MODEL,
        ] {
            data.write(path, &model);
        }
        for picture in (0..=10).map(cellview::vigor::Picture::Number).chain([
            cellview::vigor::Picture::Right(false),
            cellview::vigor::Picture::Right(true),
            cellview::vigor::Picture::Left,
        ]) {
            data.write(&picture.path(), &texture);
        }
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let models = Arc::new(VigorScene::load(&game.assets).unwrap());
        let mut menu = VigorMenu::new(0, 40);
        menu.model.page = 1;
        let opened = VigorScreen {
            menu,
            models,
            size: UVec2::new(1920, 1080),
            sequence: None,
            time: 0.0,
            pointer: None,
        };
        (data, game, opened)
    }

    /// With the tester open the HUD's camera blends its picture over the
    /// machine, decided in one place for every menu with a 3D scene. (The
    /// Caravan table's own reset once undid it every frame: the tester's
    /// menu worked but was drawn over by the HUD's empty picture.)
    #[test]
    fn the_hud_blends_over_the_tester_and_writes_plainly_without_it() {
        let (_data, _game, opened) = opened_tester("vigor-hud");
        let open = vec![super::super::OpenMenu::Vigor(Box::new(opened))];
        assert!(super::super::blends(&super::super::hud_output(
            &open, false
        )));
        assert!(!super::super::blends(&super::super::hud_output(&[], false)));
        // The lockpicking menu (not one of the game's menus here) draws
        // into the same picture, so the HUD blends over it too.
        assert!(super::super::blends(&super::super::hud_output(&[], true)));
    }

    #[test]
    fn tester_applies_each_click_to_live_values_and_only_closes_at_budget() {
        let (_data, game, mut opened) = opened_tester("vigor-menu");
        let mut state = GameState::default();
        world::chargen::set_special(&mut state, [5; 7]);
        opened.menu.intents.push(InputIntent::TileClick(4));
        opened.inputs(&game, &mut state);
        assert!(!opened.menu.closed);
        for expected in 6..=10 {
            assert!(opened.key(ui::menu::key::UP, &game, &mut state).0);
            assert_eq!(values(&game, &state)[0], expected);
        }
        assert!(!opened.key(ui::menu::key::UP, &game, &mut state).0);
        assert_eq!(values(&game, &state), [10, 5, 5, 5, 5, 5, 5]);
        opened.key(ui::menu::key::DOWN, &game, &mut state);
        assert_eq!(values(&game, &state)[0], 9);
        opened.key(ui::menu::key::UP, &game, &mut state);
        opened.menu.intents.push(InputIntent::TileClick(4));
        opened.inputs(&game, &mut state);
        assert!(opened.menu.closed);
        assert_eq!(values(&game, &state), [10, 5, 5, 5, 5, 5, 5]);
    }
}

#[derive(Component)]
pub struct VigorPiece {
    mesh: usize,
    reference: u32,
    center: Vec3,
}

#[derive(Default)]
pub struct Shown {
    entities: Vec<Entity>,
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<GameLitMaterial>>,
    textures: Vec<Option<Handle<Image>>>,
    layer: Option<Handle<Image>>,
    active: bool,
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct SceneAssets<'w> {
    images: ResMut<'w, Assets<Image>>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<GameLitMaterial>>,
    device: Option<Res<'w, RenderDevice>>,
}

fn scene_camera(target: RenderTarget) -> Camera {
    Camera {
        target,
        order: -5,
        hdr: true,
        clear_color: ClearColorConfig::Custom(Color::NONE),
        output_mode: CameraOutputMode::Write {
            blend_state: Some(BlendState::REPLACE),
            clear_color: ClearColorConfig::Custom(Color::NONE),
        },
        ..default()
    }
}

/// Render before the HUD/cursor into the same transparent menu picture.
/// The final image-space pass composites this picture after grading.
#[allow(clippy::too_many_arguments)]
pub fn draw(
    mut commands: Commands,
    menus: Res<super::GameMenus>,
    game: Res<crate::GameFiles>,
    state: Res<crate::dialogue::DialogueState>,
    settings: Res<crate::Settings>,
    hud: Option<Res<crate::hud::HudLayer>>,
    mut assets: SceneAssets,
    mut shown: Local<Shown>,
    mut pieces: Query<(
        &VigorPiece,
        &mut Transform,
        &mut Visibility,
        &MeshMaterial3d<GameLitMaterial>,
    )>,
    world_camera: Query<&Projection, With<crate::FlyCamera>>,
) {
    let open = menus.screen.as_ref().and_then(|s| {
        s.open.iter().find_map(|m| match m {
            OpenMenu::Vigor(v) => Some(&**v),
            _ => None,
        })
    });
    // Each camera clears its intermediate picture to transparent. The -5
    // camera replaces the output; the HUD's camera (-4) blends cursor/XML
    // over it while this menu is open (`super::compose_hud_over_scene`).
    // Bevy 0.16 chooses an implicit output blend from query iteration order
    // when None is used, so the first camera must explicitly replace.
    let Some(open) = open else {
        if shown.active {
            for entity in shown.entities.drain(..) {
                commands.entity(entity).despawn();
            }
            for mesh in shown.meshes.drain(..) {
                assets.meshes.remove(&mesh);
            }
            for material in shown.materials.drain(..) {
                assets.materials.remove(&material);
            }
            for image in shown.textures.drain(..).flatten() {
                assets.images.remove(&image);
            }
            shown.active = false;
        }
        return;
    };
    if !shown.active {
        let target = crate::lockpick::menu_layer(
            &mut commands,
            hud.as_deref(),
            &mut shown.layer,
            &mut assets.images,
            open.size,
        );
        let compressed = assets
            .device
            .as_ref()
            .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
        shown.textures = open
            .models
            .scene
            .textures
            .iter()
            .map(|t| crate::upload_texture(&mut assets.images, t, compressed, settings.anisotropy))
            .collect();
        let frustum = cellview::vigor::camera(&game.0.settings, open.size.x, open.size.y);
        // 007945f0 asks the world scene graph for its far plane. Bevy's
        // projection uses infinite reversed depth; far only bounds culling.
        let far = world_camera.iter().find_map(|p| match p {
            Projection::Perspective(p) => Some(p.far),
            _ => None,
        });
        let Some(far) = far else { return };
        shown.entities.push(
            commands
                .spawn((
                    Camera3d::default(),
                    scene_camera(RenderTarget::from(target)),
                    Projection::from(PerspectiveProjection {
                        fov: frustum.vertical_fov(),
                        near: frustum.near * space::METERS_PER_UNIT,
                        far,
                        ..default()
                    }),
                    Transform::from_translation(Vec3::from(space::point([0.0, 0.0, 1.0])))
                        .looking_to(Vec3::NEG_Y, Vec3::NEG_Z),
                    Tonemapping::None,
                    DebandDither::Disabled,
                    // No light clusters: nothing here is lit by Bevy's lights.
                    bevy::pbr::ClusterConfig::None,
                    Exposure {
                        ev100: crate::START_EV100,
                    },
                    RenderLayers::layer(SCENE_LAYER),
                ))
                .id(),
        );
        let lighting = crate::lockpick::menu_lighting(&open.models.lights, settings.brightness);
        for draw in &open.models.scene.draws {
            let data = &open.models.scene.meshes[draw.mesh];
            let center = crate::sort_center(data).unwrap_or([0.0; 3]);
            let mesh = assets.meshes.add(crate::game_mesh_around(data, center));
            let material =
                assets
                    .materials
                    .add(crate::lit_material(data, &shown.textures, lighting));
            shown.meshes.push(mesh.clone());
            shown.materials.push(material.clone());
            shown.entities.push(
                commands
                    .spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::IDENTITY,
                        RenderLayers::layer(SCENE_LAYER),
                        // Lit by the menu's own lights, not the hour's
                        // outdoors (as the lockpicking and Caravan pieces).
                        crate::shared_light::MenuLit,
                        VigorPiece {
                            mesh: draw.mesh,
                            reference: draw.reference,
                            center: Vec3::from(center),
                        },
                    ))
                    .id(),
            );
        }
        shown.active = true;
    }
    let display = open.display(&game.0, &state.0);
    for (piece, mut transform, mut visible, handle) in &mut pieces {
        let data = &open.models.scene.meshes[piece.mesh];
        let matrix = VigorScene::piece_matrix(open.size.x, data.motion.as_deref(), open.layer());
        *transform = Transform::from_matrix(
            Mat4::from_cols_array(&space::matrix(&matrix)) * Mat4::from_translation(piece.center),
        );
        let (show, picture) = display.appearance(piece.reference, &data.shape_name);
        *visible = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if let Some(picture) = picture {
            if let Some((_, index)) = open.models.pictures.iter().find(|(p, _)| *p == picture) {
                if let Some(material) = assets.materials.get_mut(handle.id()) {
                    let texture = shown.textures[*index].clone();
                    if material.base.base_color_texture != texture {
                        material.base.base_color_texture = texture;
                    }
                }
            }
        }
    }
}
