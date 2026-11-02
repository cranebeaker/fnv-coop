//! The player's arms and weapon in first person, as the game draws them:
//! the first-person skeleton (`world::actor::first_person_look`) holding
//! the weapon in hand (or fists) in the hold pose for its kind, the clothes
//! worn (else the race's upper body) and the first-person hands, lit by the
//! place the player is in.
//!
//! Drawn as the game draws its first-person pass (the actors recording,
//! `FalloutNVActors.trace`, frame `166830323`): after the world, the depth
//! buffer is cleared (`Clear(D3DCLEAR_ZBUFFER)` at call 166828991) and the
//! pass is drawn with its own projection, the view at the eye (the view
//! matrix is the axis swap alone, the skeleton placed in camera space),
//! near plane 5 units, the first-person field of view
//! (`fDefault1stPersonFOV` 55 in this install's `Fallout.ini`, a 4:3 width
//! like `fDefaultFOV`: the recorded projection's cotangents 1.440737 /
//! 2.56131 at 16:9), far plane about 6,600 units; the image space passes
//! come after it. Here that pass is a second camera ([`spawn_camera`]): a
//! child of the main one, drawing only [`FIRST_PERSON_LAYER`] over a
//! cleared depth buffer (so the hands never go into walls), the image space
//! grade copied from the main camera and run after it ([`copy_grade`],
//! `grade::GradeDeferred`).
//!
//! Shown only while the weapon is out (`combat::PlayerAttack::out`) or the
//! drawing / putting-away animation plays: in the five recorded frames with
//! the weapon holstered the pass after the depth clear drew nothing, in the
//! one with the machete out it drew the arms, hands and weapon. Placed every
//! frame so its `Camera1st` node sits at the eye: the skeleton stands facing
//! where the player looks, and looking up and down turns everything under
//! `Bip01 Looking` (the pivot at eye height) by the pitch. Hidden while
//! flying, while the dialogue menu is in being (also under the barter menu
//! it opens; the game skips its first-person pass then, `00870bd0`), while
//! the Pip-Boy's arm is up or going up or down (`pipboy` draws it with
//! the hands and weapon), when dead, and in screenshots. Other menus
//! don't hide it: it stays in the world behind them.
//!
//! Not yet: the Pip-Boy glove, V.A.T.S. drawing the weapon by itself.

use std::sync::Arc;

use bevy::core_pipeline::core_3d::Camera3dDepthLoadOp;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{ClearColorConfig, Exposure};
use bevy::render::view::RenderLayers;
use cellview::space;
use esm::FormId;
use preview::cell::ActorSkeleton;
use world::dialogue::PLAYER_REF;

use crate::dialogue::{Conversation, DialogueState};
use crate::grade::ImageSpaceGrade;
use crate::lighting::GameLighting;
use crate::menus::Menus;
use crate::walk::{game_point, Player};
use crate::{FlyCamera, GameFiles, ScreenshotRequest, Spawner};

/// The game's first-person field of view setting (`fDefault1stPersonFOV`
/// in this install's `Fallout.ini`).
pub const FIRST_PERSON_FOV_DEGREES: f32 = 55.0;

/// The first-person pass's near plane, game units: from the recorded
/// projection (z row 1.000758, −5.003791: −5.003791 / 1.000758 = 5.000).
pub const FIRST_PERSON_NEAR: f32 = 5.0;

/// Its far plane from the same projection (1.000758 = far / (far − near):
/// about 6,600 units; Bevy's projection has no far plane, so this only
/// bounds culling, and the pieces aren't culled anyway).
pub const FIRST_PERSON_FAR: f32 = 6600.0;

/// The render layer the first-person view is drawn on: only its own camera
/// draws it (the main camera, the HUD's and the water's don't).
pub const FIRST_PERSON_LAYER: usize = 20;

/// The camera that draws the first-person pass (see the module notes).
#[derive(Component)]
pub struct FirstPersonCamera;

/// Spawns the first-person camera under the main camera `parent`, with
/// the same exposure.
pub fn spawn_camera(commands: &mut Commands, parent: Entity, exposure: Exposure) -> Entity {
    commands
        .spawn((
            Camera3d {
                // The game's `Clear(D3DCLEAR_ZBUFFER)` before the pass
                // (0 is Bevy's far, its depth being reversed).
                depth_load_op: Camera3dDepthLoadOp::Clear(0.0),
                ..default()
            },
            Camera {
                // After the main camera, onto the same picture.
                order: 1,
                hdr: true,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            Tonemapping::None,
            // No light clusters: nothing here is lit by Bevy's lights.
            bevy::pbr::ClusterConfig::None,
            Projection::from(PerspectiveProjection {
                fov: cellview::vertical_fov(FIRST_PERSON_FOV_DEGREES),
                near: FIRST_PERSON_NEAR * space::METERS_PER_UNIT,
                far: FIRST_PERSON_FAR * space::METERS_PER_UNIT,
                ..default()
            }),
            exposure,
            Transform::IDENTITY,
            ChildOf(parent),
            RenderLayers::layer(FIRST_PERSON_LAYER),
            // The text and menus stay on this camera when a movie's camera
            // (`movie`, a higher order) is on screen.
            IsDefaultUiCamera,
            // Replaced by the main camera's every frame (`copy_grade`).
            ImageSpaceGrade::NEUTRAL,
            FirstPersonCamera,
        ))
        .id()
}

/// Gives the first-person camera the main camera's image space grade (the
/// game runs its image space passes after the first-person pass, so they
/// run on this camera, the last on the window). After everything that sets
/// the grade, including `grade::adapt_eyes`.
pub fn copy_grade(
    main: Query<&ImageSpaceGrade, (With<FlyCamera>, Without<FirstPersonCamera>)>,
    mut own: Query<&mut ImageSpaceGrade, With<FirstPersonCamera>>,
) {
    let Ok(grade) = main.single() else {
        return;
    };
    for mut g in &mut own {
        if *g != *grade {
            *g = *grade;
        }
    }
}

/// The lighting of the place the player is in, for what's drawn apart from
/// it; and a count of changes, so the first-person view is relit.
#[derive(Resource, Default)]
pub struct PlaceLighting {
    lighting: Option<GameLighting>,
    changes: u64,
}

impl PlaceLighting {
    pub fn set(&mut self, lighting: GameLighting) {
        self.lighting = Some(lighting);
        self.changes += 1;
    }

    pub fn get(&self) -> Option<GameLighting> {
        self.lighting
    }

    /// How many times the place has changed (to know when to relight).
    pub fn changes(&self) -> u64 {
        self.changes
    }

    /// Changes the light as it is (the hour moving on outdoors), without
    /// counting it as a new place.
    pub fn relight(&mut self, change: impl FnOnce(&mut GameLighting)) {
        if let Some(l) = self.lighting.as_mut() {
            change(l);
        }
    }
}

/// `--weapon`: a weapon to start with (given and equipped once the place
/// is up, with 50 rounds of its first kind of ammunition, and drawn). With
/// one, the first-person view shows in screenshots too.
#[derive(Resource, Default)]
pub struct StartWeapon(pub Option<String>);

/// Gives the `--weapon` once the place is ready, and draws it (the game
/// starts with it holstered; `--weapon` is for pictures of it in hand).
pub fn give_start_weapon(
    game: Res<GameFiles>,
    player: Res<Player>,
    mut start: ResMut<StartWeapon>,
    mut state: ResMut<DialogueState>,
    mut attack: ResMut<crate::combat::PlayerAttack>,
) {
    if !player.ready {
        return;
    }
    let Some(name) = start.0.take() else {
        return;
    };
    // `--weapon none` (or any name that isn't a weapon): fists, drawn.
    attack.draw_at_once(None);
    let order = &game.0.order;
    let id = FormId::parse_hex(&name)
        .filter(|id| order.get(*id).is_some())
        .or_else(|| order.form_by_editor_id(&name));
    let Some(weapon) = id.and_then(|id| world::combat::Weapon::load(order, id)) else {
        println!("--weapon: no weapon '{name}'.");
        return;
    };
    let state = &mut state.0;
    state.stock(order, PLAYER_REF);
    *state.items.entry((PLAYER_REF, weapon.form_id)).or_insert(0) += 1;
    if let Some(&ammo) = weapon.ammo.first() {
        *state.items.entry((PLAYER_REF, ammo)).or_insert(0) += 50;
    }
    let worn = state.equipped.entry(PLAYER_REF).or_default();
    worn.retain(|&f| {
        order
            .get(f)
            .is_none_or(|r| r.entry.header.kind.as_bytes() != b"WEAP")
    });
    worn.push(weapon.form_id);
    attack.draw_at_once(Some(weapon.form_id));
    println!("Holding the {}.", weapon.name);
}

/// Whether screenshots show the first-person view (with `--weapon`).
#[derive(Resource, Default)]
pub struct ShowInPictures(pub bool);

/// What the first-person view is built from, to know when to rebuild it.
#[derive(Clone, PartialEq)]
struct Built {
    weapon: Option<FormId>,
    /// Its fitted mods (the model changes with them).
    mods: u8,
    worn: Vec<FormId>,
    female: bool,
}

/// The first-person view on screen.
#[derive(Resource, Default)]
pub struct ViewModel {
    built: Option<Built>,
    /// The place's lighting it's lit with (`PlaceLighting::changes`), and
    /// its pieces, to light them again when that changes.
    lit_at: u64,
    lit: crate::LitPieces,
    /// The entity under the first-person camera holding it, the skeleton's
    /// root, its joints.
    holder: Option<Entity>,
    root: Option<Entity>,
    joints: Vec<Entity>,
    skeleton: Option<Arc<ActorSkeleton>>,
    /// Bones: `Bip01 Looking` (and everything under it), `Camera1st`.
    turned: Vec<usize>,
    looking: usize,
    camera: usize,
    /// The weapon's attack and reload, and its drawing (`<kind>equip.kf`)
    /// and putting away (`<kind>unequip.kf`), played over the hold pose.
    attack: Option<nif::Sequence>,
    reload: Option<nif::Sequence>,
    equip: Option<nif::Sequence>,
    unequip: Option<nif::Sequence>,
    /// The iron-sights variants of the hold pose and the attack
    /// (`world::iron_sights::first_person_is`: `<kind>aimis.kf`,
    /// `<kind>attack…is.kf`), and how far the view has gone over to them
    /// (0 to 1, over `fAnimationDefaultBlend`).
    aim_is: Option<nif::Sequence>,
    attack_is: Option<nif::Sequence>,
    is_blend: f32,
    /// The weapon model's `##SightingNode`, where it sits in the model
    /// (`008bb650` looks for it with `bTrueIronSights`; the HUD hides the
    /// crosshair while it is kept, [`SightingNode`], and the first-person
    /// camera sits at it, `00874c10`), and the skeleton's `Weapon` bone.
    sighting_node: Option<nif::Transform>,
    weapon_bone: Option<usize>,
    /// Blocking (`BlockIdle`, looping) and a blocked hit (`BlockHit`):
    /// `<kind>blockidle.kf`, `<kind>blockhit.kf`, else the plain
    /// `mtblock…` files (fists have no `h2hblock…`; the fall back to the
    /// group without the weapon kind is assumed, as the third-person body's
    /// is).
    block_idle: Option<nif::Sequence>,
    block_hit: Option<nif::Sequence>,
    /// Power attacks and the counter by group (`<kind><group>.kf`, the
    /// sneaking variant first while sneaking), read when first played.
    group_attacks: std::collections::HashMap<(u8, bool), Option<nif::Sequence>>,
    /// The gun's sway (`world::gun_wobble`): the kind's wobble model, the
    /// skeleton's bone of its name and every bone under it (the bone
    /// first).
    wobble: Option<(nif::camera::KeyedNode, Vec<usize>)>,
}

impl ViewModel {
    /// The first-person skeleton's `Weapon` joint, where the weapon's
    /// model hangs (`weapon_fx` places the muzzle flash under it).
    pub(crate) fn weapon_joint(&self) -> Option<Entity> {
        self.weapon_bone.and_then(|b| self.joints.get(b).copied())
    }
}

/// The gun sway's amount kept between frames (`011a3b2c`) and what the
/// scope's sway needs: the wobble (`008b0dd0(0)`) for the scope.
#[derive(Resource)]
pub struct GunWobble {
    pub amount: f32,
    pub settings: Option<world::gun_wobble::Settings>,
    pub vats: Option<std::sync::Arc<world::vats::Settings>>,
}

impl Default for GunWobble {
    fn default() -> Self {
        GunWobble {
            amount: 1.0,
            settings: None,
            vats: None,
        }
    }
}

impl GunWobble {
    /// The player's gun wobble now (`008b0dd0` → `00646910`,
    /// `world::vats::wobble`), from how they stand: sneaking, walking or
    /// running by the movement keys, aiming down the sights. (Which of
    /// `008b0dd0`'s modes passes which stance isn't fully traced: the
    /// player's own stance is used for both.)
    pub fn wobble(
        &mut self,
        order: &esm::LoadOrder,
        state: &world::scripting::GameState,
        weapon: &world::combat::Weapon,
        player: &Player,
        aiming: bool,
    ) -> f32 {
        let vs = self
            .vats
            .get_or_insert_with(|| std::sync::Arc::new(world::vats::Settings::load(order)))
            .clone();
        let m = player.moving;
        let moving = m.forward || m.backward || m.left || m.right;
        let stance = world::vats::Stance {
            sneaking: player.sneaking,
            swimming: false,
            walking: moving && !state.player_running,
            running: moving && state.player_running,
            aiming,
        };
        world::vats::wobble(order, state, &vs, PLAYER_REF, Some(weapon), stance)
    }
}

/// Whether the sights are up through a scope now (`008bb650`: the weapon's
/// scope model, not switching view), for the scope's overlay and sway
/// (`scope`) and the hidden first-person model.
#[derive(Resource, Default)]
pub struct Scoped(pub Option<esm::FormId>);

/// The weapon's sighting node kept while looking down the sights in first
/// person (`PlayerCharacter` +0xe34, `m_pWeaponSightingNode` (Xbox PDB),
/// set by `008bbbf0` from `008bb650` when `bTrueIronSights:GamePlay` is
/// on): the HUD hides the crosshair then (`00771700`).
#[derive(Resource, Default)]
pub struct SightingNode(pub bool);

/// The player's fields of view as `0095de30` eases them for the sights
/// (`world::iron_sights::step_fov`): the world's (the main camera's in
/// first person, `player_camera::place_view`) and the first-person
/// pass's.
#[derive(Resource)]
pub struct IronSightsFov {
    pub fov: world::iron_sights::Fov,
    settings: Option<world::iron_sights::FovSettings>,
}

impl Default for IronSightsFov {
    fn default() -> Self {
        IronSightsFov {
            fov: world::iron_sights::Fov {
                world: cellview::GAME_FOV_DEGREES,
                first_person: FIRST_PERSON_FOV_DEGREES,
            },
            settings: None,
        }
    }
}

/// The easing's settings: the viewer's world and first-person fields of
/// view (`fDefaultWorldFOV`, `fDefault1stPersonFOV`), `[Combat]
/// fIronSightsZoomDefault` and `bIronSightsZoomEnable` from the INI files,
/// `fIronSightsFOVTimeChange` from the game settings.
fn fov_settings(game: &cellview::Game) -> world::iron_sights::FovSettings {
    let base = world::iron_sights::FovSettings::default();
    world::iron_sights::FovSettings {
        world: cellview::GAME_FOV_DEGREES,
        first_person: FIRST_PERSON_FOV_DEGREES,
        zoom_default: game
            .settings
            .float("Combat", "fIronSightsZoomDefault")
            .unwrap_or(base.zoom_default),
        time_change: world::scripting::game_setting(&game.order, "fIronSightsFOVTimeChange")
            .filter(|t| *t > 0.0)
            .unwrap_or(base.time_change),
        enabled: game
            .settings
            .get("Combat", "bIronSightsZoomEnable")
            .map_or(base.enabled, |v| v.trim() != "0"),
    }
}

/// Where a model's node of that name sits in the model: its parents'
/// transforms over its own, the top node's left out (the model is hung on
/// a bone, which takes the top node's place).
pub(crate) fn node_in_model(
    game: &cellview::Game,
    model: &str,
    name: &str,
) -> Option<nif::Transform> {
    let bytes = game.assets.read(&assets::mesh_path(model)).ok()??;
    let bones = nif::Nif::parse(bytes).ok()?.skeleton().ok()?;
    let i = bones
        .iter()
        .position(|b| b.name.eq_ignore_ascii_case(name))?;
    let mut t = bones[i].local;
    let mut parent = bones[i].parent;
    while let Some(j) = parent {
        if bones[j].parent.is_none() {
            break;
        }
        t = bones[j].local.then_child(&t);
        parent = bones[j].parent;
    }
    Some(t)
}

/// One bone's two poses mixed, `w` of the way from `a` to `b`
/// (translation and scale straight, rotation along the arc). The game
/// blends its sequences' local transforms; these are the bones' model
/// space ones, which is this viewer's simplification.
fn mix(a: &nif::Transform, b: &nif::Transform, w: f32) -> nif::Transform {
    let quat = |r: &[[f32; 3]; 3]| {
        Quat::from_mat3(&Mat3::from_cols(
            Vec3::new(r[0][0], r[1][0], r[2][0]),
            Vec3::new(r[0][1], r[1][1], r[2][1]),
            Vec3::new(r[0][2], r[1][2], r[2][2]),
        ))
        .normalize()
    };
    let m = Mat3::from_quat(quat(&a.rotation).slerp(quat(&b.rotation), w));
    let rotation = [0, 1, 2].map(|i| [m.x_axis[i], m.y_axis[i], m.z_axis[i]]);
    let ta = Vec3::from(a.translation);
    let tb = Vec3::from(b.translation);
    nif::Transform {
        rotation,
        translation: ta.lerp(tb, w).to_array(),
        scale: a.scale + (b.scale - a.scale) * w,
    }
}

/// An animation file's first sequence, if the game has the file.
pub(crate) fn sequence(game: &cellview::Game, path: &str) -> Option<nif::Sequence> {
    let bytes = game.assets.read(&assets::mesh_path(path)).ok()??;
    nif::Nif::parse(bytes)
        .ok()?
        .sequences()
        .ok()?
        .into_iter()
        .next()
}

/// A layer at `since` seconds ago, while it's still playing.
fn playing(
    sequence: Option<&nif::Sequence>,
    since: Option<f32>,
    now: f32,
) -> Option<(&nif::Sequence, f32)> {
    let s = sequence?;
    let t = now - since?;
    (t >= 0.0 && t <= s.stop - s.start).then_some((s, s.start + t))
}

/// What can hide the first-person view: the dialogue menu, the Pip-Boy's
/// arm, a picture being taken, V.A.T.S.'s camera; and the menus, which
/// tell a menu from the ground still loading.
type ViewGates<'w> = (
    Res<'w, Conversation>,
    (Res<'w, Menus>, Res<'w, crate::pipboy::Pipboy>),
    Res<'w, ScreenshotRequest>,
    Res<'w, ShowInPictures>,
    Res<'w, crate::vats::Vats>,
);

/// The sights' field of view and node, the animation settings, the scope
/// and the gun's sway.
type ViewState<'w> = (
    ResMut<'w, IronSightsFov>,
    ResMut<'w, SightingNode>,
    Option<Res<'w, crate::actors::AnimSettings>>,
    Res<'w, Scoped>,
    ResMut<'w, GunWobble>,
);

/// Keeps the first-person view built for what the player holds and wears,
/// posed and placed at the eye.
#[allow(clippy::too_many_arguments)]
pub fn update_view_model(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    player: Res<Player>,
    (conversation, (menus, pipboy), screenshot, in_pictures, vats): ViewGates,
    mut view: ResMut<ViewModel>,
    mut attack: ResMut<crate::combat::PlayerAttack>,
    mut spawner: Spawner,
    third_person: Res<crate::player_camera::PlayerView>,
    cameras: Query<&Transform, With<FlyCamera>>,
    first_person: Query<Entity, With<FirstPersonCamera>>,
    mut first_person_projection: Query<&mut Projection, With<FirstPersonCamera>>,
    mut transforms: Query<&mut Transform, (Without<FlyCamera>, Without<FirstPersonCamera>)>,
    mut visibility: Query<&mut Visibility>,
    (mut iron, mut sighting, anim, scoped, mut gun): ViewState,
) {
    let order = &game.0.order;
    let state = &state.0;
    // The first-person camera sits on the main one, whose transform is
    // the eye in the world.
    let (Ok(camera), Ok(camera_transform)) = (first_person.single(), cameras.single()) else {
        return;
    };
    // The place's lighting, kept by the spawner as it puts places on screen.
    let Some(lighting) = spawner.place_lighting.lighting else {
        return;
    };
    let lighting_changes = spawner.place_lighting.changes;
    // What it should show.
    let weapon = world::combat::weapon_in_hand(order, state, PLAYER_REF);
    let worn: Vec<FormId> = state
        .equipped
        .get(&PLAYER_REF)
        .into_iter()
        .flatten()
        .copied()
        .filter(|&i| {
            order
                .get(i)
                .is_some_and(|r| r.entry.header.kind.as_bytes() == b"ARMO")
        })
        .collect();
    let female = state.player_female.unwrap_or(false);
    let wanted = Built {
        weapon: weapon.as_ref().map(|w| w.form_id),
        mods: weapon.as_ref().map_or(0, |w| {
            world::weapon_mods::flags(state, PLAYER_REF, w.form_id)
        }),
        worn: worn.clone(),
        female,
    };
    if view.built.as_ref() == Some(&wanted)
        && view.lit_at != lighting_changes
        && view.root.is_some()
    {
        // The place's lighting changed (another place, or outdoors another
        // square loaded): the same materials made with it, as rebuilding
        // the view would make them, without rebuilding it.
        view.lit_at = lighting_changes;
        spawner.relight_lone_actor(&view.lit, lighting);
    }
    if view.built.as_ref() != Some(&wanted) || view.lit_at != lighting_changes {
        if let Some(old) = view.holder.take() {
            if let Ok(mut e) = commands.get_entity(old) {
                e.despawn();
            }
        }
        view.built = Some(wanted);
        view.lit_at = lighting_changes;
        view.lit = crate::LitPieces::default();
        view.root = None;
        // The player's model with its mods on
        // (`world::weapon_mods::player_model`).
        let held = weapon.as_ref().and_then(|w| {
            let flags = world::weapon_mods::flags(state, PLAYER_REF, w.form_id);
            let model = world::weapon_mods::player_model(order, w.form_id, flags)?;
            Some((model, w.animation))
        });
        view.sighting_node = held
            .as_ref()
            .and_then(|(model, _)| node_in_model(&game.0, model, "##SightingNode"));
        let Some(look) = world::actor::first_person_look(order, female, &worn, held) else {
            return;
        };
        let scene = game.0.actor_scene(&look);
        let holder = commands
            .spawn((Transform::IDENTITY, Visibility::Hidden, ChildOf(camera)))
            .id();
        view.holder = Some(holder);
        let Some((root, joints, _, lit)) =
            spawner.spawn_lone_actor_lit(&scene, lighting, holder, FIRST_PERSON_LAYER)
        else {
            return;
        };
        view.lit = lit;
        let skeleton = scene.actors[0].skeleton.clone();
        let find = |name: &str| {
            skeleton
                .bones
                .iter()
                .position(|b| b.name.eq_ignore_ascii_case(name))
        };
        let (Some(looking), Some(camera_bone)) = (find("Bip01 Looking"), find("Camera1st")) else {
            return;
        };
        view.weapon_bone = find("Weapon");
        let turned = (0..skeleton.bones.len())
            .filter(|&i| {
                let mut b = Some(i);
                while let Some(j) = b {
                    if j == looking {
                        return true;
                    }
                    b = skeleton.bones[j].parent;
                }
                false
            })
            .collect();
        view.root = Some(root);
        view.joints = joints;
        view.skeleton = Some(skeleton);
        view.turned = turned;
        view.looking = looking;
        view.camera = camera_bone;
        // The weapon's attack (melee and fists ship `_a` variants), reload,
        // drawing and putting away.
        let animation = weapon.as_ref().map(|w| w.animation);
        let attack_file = world::actor::first_person_attack(
            animation,
            weapon.as_ref().map_or(255, |w| w.attack_animation),
        );
        view.attack = sequence(&game.0, &attack_file)
            .or_else(|| sequence(&game.0, &attack_file.replace(".kf", "_a.kf")));
        view.reload = weapon.as_ref().and_then(|w| {
            sequence(
                &game.0,
                &world::actor::first_person_reload(animation, w.reload_animation),
            )
        });
        view.equip = sequence(&game.0, &world::actor::first_person_ready(animation, true));
        view.unequip = sequence(&game.0, &world::actor::first_person_ready(animation, false));
        // Looking down the sights: the weapon groups' `IS` variants.
        let is = world::iron_sights::first_person_is;
        view.aim_is = sequence(&game.0, &is(&world::actor::first_person_pose(animation)));
        view.attack_is = sequence(&game.0, &is(&attack_file));
        // Blocking (melee weapons and fists).
        let kind = world::actor::first_person_kind(animation);
        let block = |name: &str| {
            sequence(&game.0, &format!("Characters\\_1stPerson\\{kind}{name}.kf"))
                .or_else(|| sequence(&game.0, &format!("Characters\\_1stPerson\\mt{name}.kf")))
        };
        view.block_idle = block("blockidle");
        view.block_hit = block("blockhit");
        view.group_attacks.clear();
        // The gun's sway model and the bone it turns.
        let skeleton = view.skeleton.clone();
        view.wobble = weapon.as_ref().filter(|w| !w.is_melee()).and_then(|w| {
            let skeleton = skeleton.as_ref()?;
            let path = world::gun_wobble::model_path(world::gun_wobble::wobble_kind(w.animation))?;
            let bytes = game.0.assets.read(&path).ok()??;
            let node = nif::Nif::parse(bytes).ok()?.keyed_root().ok()??;
            let bone = skeleton
                .bones
                .iter()
                .position(|b| b.name.eq_ignore_ascii_case(&node.name))?;
            let mut under = vec![bone];
            for i in 0..skeleton.bones.len() {
                let mut b = skeleton.bones[i].parent;
                while let Some(j) = b {
                    if j == bone {
                        under.push(i);
                        break;
                    }
                    b = skeleton.bones[j].parent;
                }
            }
            Some((node, under))
        });
    }
    let view = &mut *view;
    // The sights' field of view, eased every frame (`0095de30`).
    let dt = time.delta_secs();
    // The gun sway's amount (`00962de0`, `world::gun_wobble::chase`): with
    // a gun out the wobble, × `fNonAttackGunWobbleMult` when not attacking;
    // 1 otherwise.
    let gs = *gun
        .settings
        .get_or_insert_with(|| world::gun_wobble::Settings::read(order));
    let attacking = attack
        .fired_at
        .is_some_and(|t| time.elapsed_secs() < t + attack.attack_length);
    let gun_out = weapon.as_ref().filter(|w| attack.out && !w.is_melee());
    let mut target = match gun_out {
        Some(w) => gun.wobble(order, state, w, &player, attack.iron_sights),
        None => 1.0,
    };
    if !attacking {
        target *= gs.non_attack_mult;
    }
    gun.amount = world::gun_wobble::chase(gun.amount, target, attacking, dt, gs.chase_drift);
    let settings = *iron.settings.get_or_insert_with(|| fov_settings(&game.0));
    let sight = weapon
        .as_ref()
        .filter(|_| attack.out)
        .and_then(|w| world::iron_sights::sight_fov(order, w.form_id));
    let mut fov = iron.fov;
    world::iron_sights::step_fov(&mut fov, &settings, attack.iron_sights, sight, false, dt);
    iron.fov = fov;
    // `bTrueIronSights:GamePlay` (on unless the INI turns it off).
    let true_iron_sights = game
        .0
        .settings
        .get("GamePlay", "bTrueIronSights")
        .is_none_or(|v| v.trim() != "0");
    let kept = attack.iron_sights
        && true_iron_sights
        && view.sighting_node.is_some()
        && !third_person.camera.actually_third;
    if sighting.0 != kept {
        sighting.0 = kept;
    }
    // Over to the sights' poses, or back, over the default blend.
    let blend = anim.map_or(0.2, |a| a.0.default_blend).max(1e-3);
    let target = if attack.iron_sights { 1.0 } else { 0.0 };
    view.is_blend = if view.is_blend < target {
        (view.is_blend + dt / blend).min(target)
    } else {
        (view.is_blend - dt / blend).max(target)
    };
    let (Some(holder), Some(root), Some(skeleton)) =
        (view.holder, view.root, view.skeleton.clone())
    else {
        return;
    };
    let now = time.elapsed_secs();
    // Sped up in V.A.T.S. (`vats`), and at the attack's and reload's own
    // rates (`combat`: the weapon's speed, Agility, the perks): the
    // layers' time runs that much faster from when they started.
    let speed = attack.sped_up.unwrap_or(1.0);
    let faster = |since: Option<f32>, rate: f32| {
        let rate = if rate > 0.0 { rate } else { 1.0 };
        since.map(|s| now - (now - s) * speed * rate)
    };
    // Drawing or putting away: the equip or unequip animation while it
    // plays (the Ready Item key and attacks wait for it: `combat`).
    let readying = attack.readied_at.and_then(|(since, drawing)| {
        let s = if drawing { &view.equip } else { &view.unequip };
        if let Some(s) = s.as_ref() {
            attack.readying_until(since, s.stop - s.start);
        }
        playing(s.as_ref(), Some(since), now)
    });
    // Menus don't hide it: the frame's first-person pass (`00870bd0`:
    // `00874c10`, `00875110`) is skipped only with the first-person node
    // culled, the Pip-Boy drawn on its own (`bUsePipboyMode` off, its node
    // at `InterfaceManager+0x1dc`+8) or the dialogue menu in being
    // (`011d9514`, set by its constructor `007617a0`, cleared by its
    // closing `00762160` and destructor `00761960`; checked at
    // `00870c9b`), which it is under the barter and recipe menus it opens
    // too. A line said with `SayTo` opens no dialogue menu.
    let dialog_menu = conversation.0.as_ref().is_some_and(|t| !t.is_line_only());
    // Not ready with no menu up: the ground is still loading.
    let loading = !player.ready && !menus.is_open() && !dialog_menu;
    let shown = (player.walking || in_pictures.0)
        && !loading
        && !dialog_menu
        // The Pip-Boy's arm holds the hands and the weapon while it's up,
        // raised or put away (`pipboy`: one model in the game).
        && !pipboy.arm_shown(time.elapsed_secs())
        && (screenshot.path.is_none() || in_pictures.0)
        && !state.dead.contains(&PLAYER_REF)
        // A V.A.T.S. camera shot has the view.
        && !vats.shot_view()
        // The third-person body shows instead (`00951a10` hides one of
        // the two).
        && !third_person.camera.actually_third
        // Holstered, nothing shows (`009466d0`): only drawn, or while
        // drawing or putting away.
        && (attack.out || readying.is_some())
        // Through a scope the first-person model is hidden (`008bb650`:
        // `SetAppCulled(1)` on the first-person root).
        && scoped.0.is_none();
    if let Ok(mut v) = visibility.get_mut(holder) {
        let want = if shown {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if *v != want {
            *v = want;
        }
    }
    if !shown {
        return;
    }
    // V.A.T.S.'s menu zooms the first-person view too (`0095de30`): the
    // first-person camera's own field of view.
    let fov = cellview::vertical_fov(vats.first_person_fov.unwrap_or(iron.fov.first_person));
    if let Ok(mut projection) = first_person_projection.single_mut() {
        if let Projection::Perspective(p) = projection.as_mut() {
            if p.fov != fov {
                p.fov = fov;
            }
        }
    }
    // The pose: the hold pose, looping, with a reload, the last attack or
    // the drawing / putting away over it while they play, turned by the
    // pitch about the pivot.
    let f = camera_transform.forward().as_vec3();
    let dir = [f.x, -f.z, f.y];
    let heading = dir[0].atan2(dir[1]);
    let pitch = dir[2].clamp(-1.0, 1.0).asin();
    let looping = |s: &nif::Sequence| {
        let span = (s.stop - s.start).max(1e-3);
        s.start + now.rem_euclid(span)
    };
    let mut layers: Vec<(&nif::Sequence, f32)> = Vec::new();
    if let Some(s) = skeleton.idle.as_deref() {
        layers.push((s, looping(s)));
    }
    let reloading = playing(
        view.reload.as_ref(),
        faster(attack.reload_started, attack.reload_rate),
        now,
    );
    let fired = faster(attack.fired_at, attack.attack_rate);
    // A power attack or the counter plays its own group's file
    // (`world::melee`); the sneaking variant first while sneaking (the
    // move-type prefix, `005f38d0`).
    let group = attack.attack_group;
    let special = world::animation::kind_of(group) == 6;
    let sneaking = state.player_sneaking;
    if special && !view.group_attacks.contains_key(&(group, sneaking)) {
        let kind = world::actor::first_person_kind(weapon.as_ref().map(|w| w.animation));
        let stem = world::melee::group_file_stem(group).unwrap_or_default();
        let file = |prefix: &str| format!("Characters\\_1stPerson\\{prefix}{kind}{stem}.kf");
        let s = sneaking
            .then(|| sequence(&game.0, &file("sneak")))
            .flatten()
            .or_else(|| sequence(&game.0, &file("")));
        view.group_attacks.insert((group, sneaking), s);
    }
    let attack_sequence = if special {
        view.group_attacks
            .get(&(group, sneaking))
            .and_then(|s| s.as_ref())
            .or(view.attack.as_ref())
    } else {
        view.attack.as_ref()
    };
    // How long it plays (a power attack waits for it: `combat`).
    let rate = speed
        * if attack.attack_rate > 0.0 {
            attack.attack_rate
        } else {
            1.0
        };
    attack.attack_length = attack_sequence.map_or(0.0, |s| (s.stop - s.start) / rate.max(1e-3));
    let firing = playing(attack_sequence, fired, now);
    // Blocking: the block idle looping, a blocked hit's `BlockHit` over it.
    let block_hit = playing(view.block_hit.as_ref(), attack.block_hit_at, now);
    let blocking = view
        .block_idle
        .as_ref()
        .filter(|_| attack.blocking)
        .map(|s| (s, looping(s)));
    layers.extend(readying.or(reloading).or(firing).or(block_hit).or(blocking));
    let mut pose = nif::posed_layers(&skeleton.bones, &layers);
    // Down the sights: the `IS` hold pose and attack (the reload and the
    // drawing have none), mixed in as far as the blend has gone.
    if view.is_blend > 0.0 {
        if let Some(aim_is) = view.aim_is.as_ref() {
            let mut sights: Vec<(&nif::Sequence, f32)> = vec![(aim_is, looping(aim_is))];
            let firing_is = playing(view.attack_is.as_ref(), fired, now)
                .or_else(|| playing(view.attack.as_ref(), fired, now));
            sights.extend(readying.or(reloading).or(firing_is));
            let posed = nif::posed_layers(&skeleton.bones, &sights);
            for (bone, is) in pose.iter_mut().zip(&posed) {
                *bone = mix(bone, is, view.is_blend);
            }
        }
    }
    // The gun's sway (`world::gun_wobble`): the wobble model's turn since
    // its start × the amount, after the bone's own rotation (the
    // `AdditionalRotation` controller), carrying everything under it. The
    // model is sampled at the viewer's clock (the game's first-person
    // animation time, AnimData +0xd0, isn't kept here).
    if let Some((node, bones)) = view.wobble.as_ref() {
        if let (Some(a), Some(base)) = (node.angles_at(now), node.angles_at(node.start())) {
            let r = world::gun_wobble::sway_rotation(a, base, gun.amount);
            let b = bones[0];
            let m = pose[b];
            let turn = nif::Transform {
                rotation: r,
                translation: [0.0; 3],
                scale: 1.0,
            };
            let delta = m.then_child(&turn).then_child(&m.inverse());
            for &i in bones {
                pose[i] = delta.then_child(&pose[i]);
            }
        }
    }
    let pivot = pose[view.looking].translation;
    let (s, c) = pitch.sin_cos();
    let rotation = [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]];
    let turn = nif::Transform {
        rotation,
        // A turn about the pivot (its x stays).
        translation: [
            0.0,
            pivot[1] - (c * pivot[1] - s * pivot[2]),
            pivot[2] - (s * pivot[1] + c * pivot[2]),
        ],
        scale: 1.0,
    };
    for &i in &view.turned {
        pose[i] = turn.then_child(&pose[i]);
    }
    for (joint, bone) in view.joints.iter().zip(&pose) {
        if let Ok(mut t) = transforms.get_mut(*joint) {
            *t = crate::actors::bevy_transform(bone);
        }
    }
    // The root: facing the heading, with `Camera1st` at the eye. The
    // first-person camera sits on the main one, so the eye is its place.
    let eye = game_point(camera_transform.translation);
    let (sh, ch) = heading.sin_cos();
    let cam = pose[view.camera].translation;
    let feet = [
        eye[0] - (ch * cam[0] + sh * cam[1]),
        eye[1] - (-sh * cam[0] + ch * cam[1]),
        eye[2] - cam[2],
    ];
    let game = [
        ch, -sh, 0.0, 0.0, sh, ch, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, feet[0], feet[1], feet[2], 1.0,
    ];
    let world_root = Mat4::from_cols_array(&space::matrix(&game));
    let local = camera_transform.compute_matrix().inverse() * world_root;
    if let Ok(mut t) = transforms.get_mut(root) {
        *t = Transform::from_matrix(local);
    }
    // True iron sights: the first-person camera at the weapon's sighting
    // node (`00874c10` sets the camera's translation from player +0xe34's
    // world position while it's kept): the view moves so the node is at
    // its origin.
    let at_sights = match (sighting.0, view.sighting_node, view.weapon_bone) {
        (true, Some(node), Some(bone)) => {
            let node = pose[bone].then_child(&node).translation;
            -local.transform_point3(Vec3::from(node))
        }
        _ => Vec3::ZERO,
    };
    if let Ok(mut t) = transforms.get_mut(holder) {
        if t.translation != at_sights {
            t.translation = at_sights;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::camera::CameraProjection;

    #[test]
    fn the_first_person_projection_is_the_recorded_one() {
        // The actors recording's first-person pass (call 166828994):
        // cotangents 1.440737 across and 2.56131 up at 1920 × 1080, near 5
        // units (z row 1.000758, −5.003791).
        let projection = PerspectiveProjection {
            fov: cellview::vertical_fov(FIRST_PERSON_FOV_DEGREES),
            aspect_ratio: 1920.0 / 1080.0,
            near: FIRST_PERSON_NEAR * space::METERS_PER_UNIT,
            far: FIRST_PERSON_FAR * space::METERS_PER_UNIT,
        };
        let m = projection.get_clip_from_view();
        assert!((m.x_axis.x - 1.440737).abs() < 1e-4, "{}", m.x_axis.x);
        assert!((m.y_axis.y - 2.56131).abs() < 1e-4, "{}", m.y_axis.y);
        // Bevy's reversed depth: 1 at the near plane.
        let near = m.project_point3(Vec3::new(0.0, 0.0, -projection.near));
        assert!((near.z - 1.0).abs() < 1e-5, "{}", near.z);
        // The game's z row (A, −B): near = B / A, far from A = far / (far − near).
        assert!((5.003791f32 / 1.000758 - FIRST_PERSON_NEAR).abs() < 1e-3);
        assert!(
            (1.000758 / (1.000758 - 1.0) * FIRST_PERSON_NEAR / FIRST_PERSON_FAR - 1.0).abs() < 0.01
        );
    }
}
