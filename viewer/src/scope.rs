//! Looking through a scope, as `FalloutNV.exe` 1.4.0.525 does it.
//!
//! - When: the sights up (`combat::PlayerAttack::iron_sights`) with a
//!   weapon that has a scope model (`world::iron_sights::scoped`: `MOD3`,
//!   and for "scope from mod" weapons a zoom mod, which this viewer never
//!   has), the weapon out, the view not switching (`008bb650`: want ==
//!   shown third person). The first-person model is hidden then
//!   (`viewmodel`), the HUD shows only the enemy's health (mode 0x17,
//!   `00771700`: mask 8), the zoom is the sights' field of view
//!   (`viewmodel::IronSightsFov`, `0095de30`).
//! - The overlay (`0077f2f0` loads the model, `0077f3c0` shows it,
//!   `0077f0d0` draws it): the scope model is hung under the HUD's scope
//!   node, which the HUD set up (`0076bfe0` at `0076fd02`…`0076fdb4`)
//!   turned by X(π/2)·Z(π/2) (`00524ac0`, `004a0c90`: the model's x to the
//!   camera's right, z up, y away), scaled 0.1 and 380 units in front of
//!   its own camera (`0077ee50`), the model's top node's own transform
//!   replaced. Every frame its frustum (`00709d50(camera, 0.1)`): angle a
//!   = min(0.1 × `fDefaultFOV:Display` × π/180 × 1.1, 1.53938), top
//!   0.75·tan a, bottom −0.75·tan a, right/left ± that × the screen's
//!   width ÷ height, near 1, far 5000. Drawn after the image space, over
//!   the picture (here into the HUD's picture, before its pieces), each
//!   mesh blended as its `NiAlphaProperty` says, unlit.
//! - The sway (`world::gun_wobble`): `ScopeWobble.nif`'s change since the
//!   last frame × the wobble × `fGunWobbleMultScope`, its X angle added to
//!   the pitch and its Z angle to the heading.
//!
//! Not here: the world drawn inside a scissor while scoped (`00870bd0`,
//! `fScopeScissorAmount:Display`; the overlay's frame covers the rest),
//! the shaders' view-angle falloff on the overlay's meshes (they face the
//! camera), forcing first person from third (`00950460(1)`), weapon mods'
//! zoom (mod effect 0xe).

use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::{ClearColorConfig, RenderTarget};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use esm::FormId;
use world::dialogue::PLAYER_REF;

use crate::dialogue::DialogueState;
use crate::walk::Player;
use crate::{FlyCamera, GameFiles};

/// The layer the scope's overlay is drawn on (its own camera only).
pub const SCOPE_LAYER: usize = 28;

/// The overlay's camera.
#[derive(Component)]
pub struct ScopeCamera;

/// What's on screen for the scope: the weapon whose model is built and
/// its root; the scope sway's model and last sampled angles.
#[derive(Resource, Default)]
pub struct ScopeOverlay {
    built: Option<FormId>,
    root: Option<Entity>,
    sway: Option<Option<nif::camera::KeyedNode>>,
    last: Option<[f32; 3]>,
}

/// The overlay's camera onto the HUD's picture, before the HUD's pieces:
/// on only while a scope is up (a 3D view costs a frame its work even
/// with nothing in it), clearing the picture then, the HUD's camera
/// clearing it otherwise ([`update_scope`]).
pub fn spawn_camera(commands: &mut Commands, layer: Handle<Image>) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: false,
            target: RenderTarget::from(layer),
            order: -5,
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: frustum_fov(75.0),
            near: 1.0,
            far: 5000.0,
            ..default()
        }),
        Tonemapping::None,
        DebandDither::Disabled,
        // No light clusters: nothing here is lit by Bevy's lights.
        bevy::pbr::ClusterConfig::None,
        Msaa::Off,
        Transform::IDENTITY,
        RenderLayers::layer(SCOPE_LAYER),
        ScopeCamera,
    ));
}

/// The overlay camera's vertical field of view (radians) for
/// `fDefaultFOV` (`00709d50`): top = 0.75 × tan a at the near plane 1.
// Translated from 00709d50 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn frustum_fov(default_fov: f32) -> f32 {
    let a = (0.1 * default_fov * 0.017_453_3 * 1.1).min(1.539_38);
    2.0 * (0.75 * a.tan()).atan()
}

/// The scope node's transform in its camera's space (Bevy's axes: −z
/// ahead, y up): 380 ahead, scale 0.1, the model's y away and z up.
pub fn node_transform() -> Transform {
    Transform {
        translation: Vec3::new(0.0, 0.0, -380.0),
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        scale: Vec3::splat(0.1),
    }
}

/// How a mesh's `NiAlphaProperty` blends (flags: bit 0 on, source factor
/// bits 1–4, destination 5–8: 6 source alpha, 7 its inverse, 0 one, 1
/// zero).
fn alpha_mode(alpha: Option<&nif::AlphaProperty>) -> (AlphaMode, bool) {
    let Some(a) = alpha.filter(|a| a.flags & 1 != 0) else {
        return (AlphaMode::Opaque, false);
    };
    let src = (a.flags >> 1) & 0xf;
    let dst = (a.flags >> 5) & 0xf;
    match (src, dst) {
        // Source alpha, one: added.
        (6, 0) => (AlphaMode::Add, false),
        // Zero, inverse source alpha: the picture darkened by the alpha,
        // the same as blending black.
        (1, 7) => (AlphaMode::Blend, true),
        _ => (AlphaMode::Blend, false),
    }
}

/// Builds the overlay for a scope model under a new root.
fn build(
    commands: &mut Commands,
    game: &cellview::Game,
    model: &str,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> Option<Entity> {
    let bytes = game.assets.read(&assets::mesh_path(model)).ok()??;
    let scene = nif::Nif::parse(bytes).ok()?.placed_scene().ok()?;
    let root = commands
        .spawn((
            node_transform(),
            Visibility::Hidden,
            RenderLayers::layer(SCOPE_LAYER),
        ))
        .id();
    // The model's top node keeps its turn (`scope01.nif`'s is tilted so its
    // pieces lie across the view); its translation is set to 0 when it's
    // hung (`0077f2f0`: `0043d410(0, 0, 0)`). (Its rotation's set from an
    // unrecovered value there too; the model only faces the camera with its
    // own, so that is kept: an assumption.)
    let root_turn = scene.root_transform.unwrap_or(nif::Transform::IDENTITY);
    for m in &scene.meshes {
        let positions: Vec<[f32; 3]> = m
            .model_positions()
            .map(|p| {
                let q = root_turn.apply_point(p);
                [
                    q[0] - root_turn.translation[0],
                    q[1] - root_turn.translation[1],
                    q[2] - root_turn.translation[2],
                ]
            })
            .collect();
        let n = positions.len();
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; n]);
        let uvs = if m.uvs.len() == n {
            m.uvs.clone()
        } else {
            vec![[0.0, 0.0]; n]
        };
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        let (mode, black) = alpha_mode(m.alpha.as_ref());
        let colors: Vec<[f32; 4]> = if m.colors.len() == n {
            m.colors
                .iter()
                .map(|c| if black { [0.0, 0.0, 0.0, c[3]] } else { *c })
                .collect()
        } else {
            vec![[1.0; 4]; n]
        };
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U16(
            m.triangles.iter().flatten().copied().collect(),
        ));
        let texture = m.diffuse_texture().and_then(|t| {
            crate::hud::upload_picture(images, game, &assets::texture_path(t), (true, true), false)
        });
        let material = StandardMaterial {
            base_color: if black { Color::BLACK } else { Color::WHITE },
            base_color_texture: texture,
            unlit: true,
            alpha_mode: mode,
            double_sided: true,
            cull_mode: None,
            ..default()
        };
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
            Transform::IDENTITY,
            RenderLayers::layer(SCOPE_LAYER),
            ChildOf(root),
        ));
    }
    Some(root)
}

/// Where the overlay's meshes, materials and pictures go.
type OverlayAssets<'w> = (
    ResMut<'w, Assets<Mesh>>,
    ResMut<'w, Assets<StandardMaterial>>,
    ResMut<'w, Assets<Image>>,
);

/// Each frame after the player's attack: whether a scope is up
/// (`viewmodel::Scoped`), the overlay built for the weapon and shown or
/// hidden, its frustum.
/// The overlay's camera and the HUD's.
type ScopeCameras<'w, 's> = (
    Query<'w, 's, (&'static mut Projection, &'static mut Camera), With<ScopeCamera>>,
    Query<'w, 's, &'static mut Camera, (With<crate::hud::HudCamera>, Without<ScopeCamera>)>,
);

#[allow(clippy::too_many_arguments)]
pub fn update_scope(
    mut commands: Commands,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    attack: Res<crate::combat::PlayerAttack>,
    view: Res<crate::player_camera::PlayerView>,
    mut scoped: ResMut<crate::viewmodel::Scoped>,
    mut overlay: ResMut<ScopeOverlay>,
    (mut meshes, mut materials, mut images): OverlayAssets,
    mut visibility: Query<&mut Visibility>,
    (mut cameras, mut hud): ScopeCameras,
) {
    let order = &game.0.order;
    let weapon = world::combat::weapon_in_hand(order, &state.0, PLAYER_REF);
    let switching = view.camera.want_third != view.camera.actually_third;
    let now = weapon
        .as_ref()
        .filter(|w| {
            attack.iron_sights
                && attack.out
                && !switching
                && !view.camera.actually_third
                && world::iron_sights::scoped(order, w.form_id, w.flags2, None)
        })
        .map(|w| w.form_id);
    if scoped.0 != now {
        scoped.0 = now;
    }
    // The overlay's camera on while a scope is up (this frame: the cameras
    // that draw are decided after `Update`), the HUD's clearing the
    // picture when it's off.
    let on = now.is_some();
    for (_, mut camera) in &mut cameras {
        if camera.is_active != on {
            camera.is_active = on;
        }
    }
    for mut camera in &mut hud {
        let clearing = !matches!(camera.clear_color, ClearColorConfig::None);
        if clearing == on {
            camera.clear_color = if on {
                ClearColorConfig::None
            } else {
                ClearColorConfig::Custom(Color::NONE)
            };
        }
    }
    // The overlay's model follows the weapon in hand (`0077f2f0`).
    let wanted = weapon
        .as_ref()
        .filter(|w| world::iron_sights::scope_model(order, w.form_id).is_some())
        .map(|w| w.form_id);
    if overlay.built != wanted {
        if let Some(old) = overlay.root.take() {
            if let Ok(mut e) = commands.get_entity(old) {
                e.despawn();
            }
        }
        overlay.built = wanted;
        overlay.root = wanted
            .and_then(|w| world::iron_sights::scope_model(order, w))
            .and_then(|model| {
                build(
                    &mut commands,
                    &game.0,
                    &model,
                    &mut meshes,
                    &mut materials,
                    &mut images,
                )
            });
    }
    if let Some(root) = overlay.root {
        if let Ok(mut v) = visibility.get_mut(root) {
            let want = if now.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            if *v != want {
                *v = want;
            }
        }
    }
    let default_fov = game
        .0
        .settings
        .float("Display", "fDefaultFOV")
        .unwrap_or(cellview::GAME_FOV_DEGREES);
    let fov = frustum_fov(default_fov);
    for (mut p, _) in &mut cameras {
        if let Projection::Perspective(p) = p.as_mut() {
            if p.fov != fov {
                p.fov = fov;
            }
        }
    }
}

/// The scope's sway on the view (before the mouse look writes the
/// camera's turn): `ScopeWobble.nif`'s change since the last frame × the
/// wobble × `fGunWobbleMultScope`; X to the pitch, Z to the heading.
#[allow(clippy::too_many_arguments)]
pub fn scope_sway(
    time: Res<Time>,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    player: Res<Player>,
    attack: Res<crate::combat::PlayerAttack>,
    scoped: Res<crate::viewmodel::Scoped>,
    mut gun: ResMut<crate::viewmodel::GunWobble>,
    mut overlay: ResMut<ScopeOverlay>,
    mut cameras: Query<&mut FlyCamera>,
) {
    let order = &game.0.order;
    let sway = overlay
        .sway
        .get_or_insert_with(|| {
            let path = world::gun_wobble::model_path(0)?;
            let bytes = game.0.assets.read(&path).ok()??;
            nif::Nif::parse(bytes).ok()?.keyed_root().ok()?
        })
        .clone();
    let Some(id) = scoped.0 else {
        overlay.last = None;
        return;
    };
    let (Some(node), Some(w)) = (
        sway,
        world::combat::weapon_in_hand(order, &state.0, PLAYER_REF),
    ) else {
        return;
    };
    if w.form_id != id {
        return;
    }
    let Some(angles) = node.angles_at(time.elapsed_secs()) else {
        return;
    };
    let gs = *gun
        .settings
        .get_or_insert_with(|| world::gun_wobble::Settings::read(order));
    let amount = gun.wobble(order, &state.0, &w, &player, attack.iron_sights) * gs.scope_mult;
    let last = overlay.last.replace(angles).unwrap_or(angles);
    let [x, _, z] = world::gun_wobble::sway_angles(angles, last, amount);
    if let Ok(mut fly) = cameras.single_mut() {
        // Game pitch is the camera's −pitch, the heading its −yaw.
        fly.pitch = (fly.pitch - x).clamp(-1.54, 1.54);
        fly.yaw -= z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_overlay_fills_the_screen_as_the_game_frames_it() {
        // fDefaultFOV 75: a = 8.25 degrees; at 380 units the half height is
        // 380 × 0.75 × tan a = 41.3, the scope's black frame (±460 × 0.1)
        // covers it, and the lens (radius 360 × 0.1) nearly fills it.
        let fov = frustum_fov(75.0);
        let half = 380.0 * (fov / 2.0).tan();
        assert!((half - 41.3).abs() < 0.1, "{half}");
        assert!(46.0 > half && 36.0 < half);
        // The model's y goes away from the camera, z up, x right.
        let t = node_transform();
        let away = t.rotation * Vec3::Y;
        let up = t.rotation * Vec3::Z;
        assert!((away - Vec3::NEG_Z).length() < 1e-5);
        assert!((up - Vec3::Y).length() < 1e-5);
        assert!((t.rotation * Vec3::X - Vec3::X).length() < 1e-5);
    }
}
