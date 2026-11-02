//! The Pip-Boy 3000 (`ui::pipboy`): the game's three Pip-Boy menus (STATS,
//! ITEMS, DATA) filled from the game's state, drawn as the game draws them
//! with `[Pipboy] bUsePipboyMode` on (this install): into a picture of
//! their own (1280 × 960 menu units, `007fba00`), which the screen effect
//! (`ISIFSCANBLEND`, `pipboy.wgsl`: the glow, scanlines, the rolling
//! picture and the bright band, timed by `ui::pipboy::screen`) turns into
//! the texture of the arm model's `pipboyscreen:0` (`PipBoy3000\
//! PipBoyArm.NIF`, the `PipBoy` apparel's model, held on the first-person
//! skeleton's `Bip01 L ForeTwist` as its `Prn` says), raised in front of
//! the eye by the first-person `pipboy.kf` and drawn with
//! `fPipboy1stPersonFOV` (47 in this install's `Fallout.ini`).
//!
//! Keys (read from the code): Tab (control 14 "Pip-Boy", its default key
//! in `00a24b70`) puts it up when let go before `fPlayerPipBoyLightTimer`
//! (0.8 s); held that long it switches the Pip-Boy light instead
//! (`009673d0`; the `PipBoyLight` ability, whose `PipLight` effect's light
//! is `PipboyLight640`: radius 768, colour 194, 245, 209). Up, Tab again
//! puts it away. Inside, the PC keyboard as the game reads it in menus
//! (`007154b0`, `0070c4a0`): the arrows (up and down the lists, left and
//! right the pages and tabs; with Shift the previous or next menu, round
//! the three), Enter the A button (equip, use, make a quest active),
//! Shift + Enter X and Alt + Enter Y (the Status page's aid), and letters
//! the buttons the menus' `_PCButton_` traits name (S Stimpak, A RadAway,
//! X Rad-X, E Doctor's Bag, R the General page's reputations).
//!
//! Sounds: `UIPipBoyAccessUp` / `Down` putting it up and away, the hum
//! `UIPipBoyHumLP` while it's up, `UIPipBoyTab` (the knob) between menus and
//! pages, `UIPipBoyScroll`, `UIPipBoySelect`, `UIPipBoyLightOn` / `Off`.
//! The lamps over STATS, ITEMS and DATA: only the shown menu's lit
//! (`007fa010`); the light's cone shown with the light on (`007fa310`).
//!
//! F1, F2 and F3 put it up on STATS, ITEMS or DATA, or turn to that menu;
//! the shown menu's own key puts it away (`0070c4a0`). Tab let go again
//! puts it away (control 14 come up).
//!
//! The mouse (`007f8720`, `0070c4a0`, `007126c0`): the pointer's ray from
//! the first-person camera that draws the arm meets `pipboyscreen:0`
//! (skinned as drawn); its texture coordinates × 1280 × 960 are the place
//! on the menus' picture, where the interface picks tiles, moves the
//! mouse-over, clicks, drags and turns the wheel
//! (`ui::pipboy::Pipboy::pointer`). Off the screen nothing is picked.
//! Pressing and letting go over the same one of the model's
//! `PipBoyButton01` .. `03` shows STATS, ITEMS or DATA with `UIMenuMode`.
//! The game's cursor is drawn over it (`game_menus`), hidden over DATA's
//! map where the highlight box follows the pointer.
//!
//! The right button drops the chosen item on ITEMS and asks for the
//! player's own marker on DATA's world map; the wheel and Page Up / Page
//! Down zoom the map. Questions (fast travel, the marker, "how many?") are
//! the game's own menus over the Pip-Boy (`game_menus::asks`), answered
//! back here.
//!
//! Guesses: the picture's size in pixels (one a menu unit); the arm held at
//! the raising animation's `Hit` key while up (where it's highest) and
//! lowered by playing on from there.
//! Not done: the keys held repeating, the world paused while it's up, the
//! buttons moving, the `xbox` button labels swapped for the PC's, Page Up
//! / Page Down on ITEMS (Mod).
// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    AsBindGroup, Extent3d, ShaderRef, ShaderType, TextureDimension, TextureFormat, TextureUsages,
    WgpuFeatures,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::RenderLayers;
use bevy::sprite::{AlphaMode2d, Material2d, Material2dPlugin};
use cellview::space;
use esm::FormId;
use preview::cell::ActorSkeleton;
use ui::draw::{DrawItem, DrawKind};
use ui::pipboy::screen::{ScreenEffects, ScreenSettings};
use ui::pipboy::{Action, Key, PipboyInput, Section};
use world::dialogue::PLAYER_REF;

use crate::dialogue::DialogueState;
use crate::game_menus::asks::{self, Ask};
use crate::hud::{Files, Quad, TileMaterial};
use crate::lighting::{GameLight, GameLitMaterial};
use crate::menus::Menus;
use crate::sounds::{PcmSound, SoundRequests};
use crate::walk::{game_point, Player};
use crate::{FlyCamera, GameFiles, Spawner};

const SCREEN_SHADER: Handle<Shader> = weak_handle!("6f1d0b9e-3c52-4a8e-b7f4-2e9a5c0d81b7");

/// The render layers only the Pip-Boy's two cameras see.
const MENU_LAYER: usize = 24;
const SCREEN_LAYER: usize = 25;

/// The menus' picture in pixels: the game's 1280 × 960 menu units, one
/// pixel each (the size of the game's render target isn't traced).
const PICTURE: UVec2 = UVec2::new(1280, 960);

/// `fPipboy1stPersonFOV` in this install's `Fallout.ini` (the exe's
/// reader isn't traced; a 4:3 width like the other fields of view).
const PIPBOY_FOV_DEGREES: f32 = 47.0;

/// `fPlayerPipBoyLightTimer` (`011cd098`): how long Tab is held to switch
/// the light.
const LIGHT_HOLD_SECONDS: f32 = 0.8;

/// The first-person raising animation's files (`Characters\_1stPerson\
/// Locomotion\Male\Pipboy.kf`, 0.73 s, and the female one).
const RAISE_MALE: &str = "Characters\\_1stPerson\\Locomotion\\Male\\Pipboy.kf";
const RAISE_FEMALE: &str = "Characters\\_1stPerson\\Locomotion\\Female\\PipboyFemale.kf";

/// The Pip-Boy (`ARMO` `PipBoy`) and its glove (`PipBoyGlove`): the
/// player record's two items, worn.
const PIPBOY_ITEM: &str = "PipBoy";
const GLOVE_ITEM: &str = "PipBoyGlove";

pub struct PipboyPlugin;

impl Plugin for PipboyPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SCREEN_SHADER, "pipboy.wgsl", Shader::from_wgsl);
        app.add_plugins(Material2dPlugin::<ScreenMaterial>::default())
            .init_resource::<Pipboy>()
            .add_systems(Startup, setup_pipboy)
            .add_systems(
                Update,
                // Kept, inside the interface idle's set (`crate::frame_order`):
                // after the game's own menus, so one open over the Pip-Boy
                // (a question, "how many?") has the input first.
                pipboy_keys
                    .after(crate::game_menus::run_open_menus)
                    .before(crate::menus::run_menus)
                    .in_set(crate::frame_order::IdleSet),
            )
            // The Pip-Boy's screen is the interface idle's
            // (`Main::OnIdle_DoInterfaceIdle`), the AI threads' first call
            // with threads > 1 (`crate::frame_order::AiSet`): after the scripts
            // (stage 4) and the first-person model (stage 2) by the stages'
            // order. At the call but outside its gate: without AI work (menu
            // mode, which the Pip-Boy is) the main thread makes the call
            // instead (frame step 105).
            .add_systems(
                Update,
                (
                    // Kept: the light follows the screen's state.
                    pipboy_light.after(update_pipboy),
                    update_pipboy,
                )
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .after(crate::frame_order::AiSet::Call(
                        crate::frame_order::INTERFACE_IDLE,
                    ))
                    // Kept: after the menus, as when they ran ahead of the frame
                    // (the idle's own order among them isn't traced).
                    .after(crate::frame_order::IdleSet)
                    .before(crate::frame_order::AiSet::next(
                        crate::frame_order::INTERFACE_IDLE,
                    )),
            );
    }
}

/// The Pip-Boy light as the lighting shader takes it: the `PipBoyLight`
/// ability's `PipLight` effect is a Light effect (archetype 13), whose
/// light `LightEffect::AttachLight` (Xbox PDB; `0080e970`) makes: the
/// effect's light record's colour (`PipboyLight640`: 194, 245, 209), a
/// radius of `fMagicUnitsPerFoot` (22) × (the effect's magnitude, 15 +
/// `fMagicLightRadiusBase` 0), on the actor's node at `fMagicLightSide
/// Offset` (0) across, (the bound's far y + `fMagicLightForwardOffset`
/// 22) × scale forward and the actor's height (`008853a0`: the bound's
/// height × scale) + `fMagicLightHeightOffset` (10) × scale up. The
/// player's bound is read as its `OBND` (y to 17, height 132). The
/// light's brightness at 1 (what `0080ed20(255)` sets isn't traced).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipboyLightData {
    pub color: [f32; 3],
    pub radius: f32,
    /// Across, forward and up from the player's feet, in game units.
    pub offset: [f32; 3],
}

impl PipboyLightData {
    fn load(order: &esm::LoadOrder) -> Option<PipboyLightData> {
        let setting = |n: &str, d: f32| world::scripting::game_setting(order, n).unwrap_or(d);
        let spell = order.form_by_editor_id("PipBoyLight")?;
        let effects = world::items::effects(order, spell);
        let effect = effects.first()?;
        let mgef = order.get(effect.effect)?.record().ok()?;
        let data = mgef.get(esm::sig::DATA)?.data.clone();
        let light = u32::from_le_bytes(data.get(24..28)?.try_into().ok()?);
        let rr = order.get(effect.effect)?;
        let light = rr.plugin.to_global(FormId(light));
        let lrec = order.get(light)?.record().ok()?;
        let l = lrec.get(esm::sig::DATA)?.data.clone();
        let color = [l.get(8)?, l.get(9)?, l.get(10)?].map(|&c| f32::from(c) / 255.0);
        let radius = setting("fMagicUnitsPerFoot", 22.0)
            * (effect.magnitude as f32 + setting("fMagicLightRadiusBase", 0.0));
        let player = order.get(PLAYER_BASE)?.record().ok()?;
        let obnd = player.get(esm::FourCC::new(b"OBND"))?.data.clone();
        let i16_at = |i: usize| -> Option<f32> {
            Some(f32::from(i16::from_le_bytes([
                *obnd.get(i)?,
                *obnd.get(i + 1)?,
            ])))
        };
        let (min_z, max_y, max_z) = (i16_at(4)?, i16_at(8)?, i16_at(10)?);
        Some(PipboyLightData {
            color,
            radius,
            offset: [
                setting("fMagicLightSideOffset", 0.0),
                max_y + setting("fMagicLightForwardOffset", 22.0),
                (max_z - min_z) + setting("fMagicLightHeightOffset", 10.0),
            ],
        })
    }
}

/// The player's base (`NPC_` 00000007).
const PLAYER_BASE: FormId = FormId(7);

/// Where the Pip-Boy light is in the world: the player's feet turned by
/// the heading (radians clockwise from north), plus the offset.
fn light_point(feet: [f32; 3], heading: f32, offset: [f32; 3]) -> [f32; 3] {
    let (s, c) = heading.sin_cos();
    // Forward is (sin, cos) on the ground; across, to the right, (cos, -sin).
    [
        feet[0] + offset[0] * c + offset[1] * s,
        feet[1] - offset[0] * s + offset[1] * c,
        feet[2] + offset[2],
    ]
}

/// The lit surfaces' lights with the Pip-Boy light added after the
/// place's (and taken off again): kept per material, with the count the
/// place gave it.
#[derive(Default)]
pub struct LightOnSurfaces {
    data: Option<Option<PipboyLightData>>,
    last: Option<GameLight>,
    lit: HashMap<AssetId<GameLitMaterial>, usize>,
    terrain: HashMap<AssetId<crate::terrain::TerrainMaterial>, usize>,
    counts: (usize, usize, u64),
}

/// Adds the light to (or takes it from) one surface's lights; true when
/// something changed.
fn put_light(
    lighting: &mut crate::lighting::GameLighting,
    ours: Option<usize>,
    last: Option<GameLight>,
    light: Option<GameLight>,
) -> Option<usize> {
    let count = lighting.scale.y as usize;
    // The place's own count: ours added on top, unless the place's lights
    // were given again since.
    let base = match ours {
        Some(b) if count == b + 1 && last.is_some_and(|l| lighting.lights[b] == l) => b,
        Some(b) if count == b => b,
        _ => count,
    };
    match light {
        Some(l) if base < crate::lighting::MAX_LIGHTS => {
            lighting.lights[base] = l;
            lighting.scale.y = (base + 1) as f32;
            Some(base)
        }
        _ => {
            lighting.scale.y = base as f32;
            None
        }
    }
}

/// The Pip-Boy light lighting the place while it's on.
#[allow(clippy::too_many_arguments)]
fn pipboy_light(
    pipboy: Res<Pipboy>,
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    settings: Res<crate::Settings>,
    views: Query<&Transform, With<FlyCamera>>,
    pieces: Query<(&MeshMaterial3d<GameLitMaterial>, &RenderLayers)>,
    mut lit: ResMut<Assets<GameLitMaterial>>,
    mut terrain: ResMut<Assets<crate::terrain::TerrainMaterial>>,
    remade: Res<crate::daylight::RemadeLit>,
    mut on: Local<LightOnSurfaces>,
) {
    let data = *on
        .data
        .get_or_insert_with(|| PipboyLightData::load(&game.0.order));
    let light = (|| {
        let data = data?;
        if !pipboy.light {
            return None;
        }
        let feet = state.0.player_position?;
        let f = views.single().ok()?.forward().as_vec3();
        let heading = f.x.atan2(-f.z);
        let at = light_point(feet, heading, data.offset);
        let [x, y, z] = space::point(at);
        let [r, g, b] = data.color.map(|c| c * settings.brightness);
        Some(GameLight {
            position_radius: Vec4::new(x, y, z, data.radius * space::METERS_PER_UNIT),
            color: Vec4::new(r, g, b, 0.0),
        })
    })();
    // Moved little and nothing new to light (no surface added, none made
    // again): left as it is.
    let counts = (lit.len(), terrain.len(), remade.times);
    let near = match (light, on.last) {
        (Some(a), Some(b)) => a.position_radius.distance(b.position_radius) < 0.05,
        (None, None) => true,
        _ => false,
    };
    if near && counts == on.counts {
        return;
    }
    let last = on.last;
    // The first-person view (the arm, the weapon in hand) is drawn apart
    // with the place's own light: the place's light is what this adds to.
    let first_person: std::collections::HashSet<_> = pieces
        .iter()
        .filter(|(_, layers)| {
            layers.intersects(&RenderLayers::layer(crate::viewmodel::FIRST_PERSON_LAYER))
        })
        .map(|(m, _)| m.0.id())
        .collect();
    let ids: Vec<_> = lit
        .ids()
        .filter(|id| !first_person.contains(id) || on.lit.contains_key(id))
        .collect();
    for id in ids {
        let light = if first_person.contains(&id) {
            None
        } else {
            light
        };
        let ours = on.lit.get(&id).copied();
        if ours.is_none() && light.is_none() {
            continue;
        }
        if let Some(m) = lit.get_mut(id) {
            match put_light(&mut m.extension.lighting, ours, last, light) {
                Some(b) => on.lit.insert(id, b),
                None => on.lit.remove(&id),
            };
        }
    }
    let ids: Vec<_> = terrain.ids().collect();
    for id in ids {
        let ours = on.terrain.get(&id).copied();
        if ours.is_none() && light.is_none() {
            continue;
        }
        if let Some(m) = terrain.get_mut(id) {
            match put_light(&mut m.extension.lighting, ours, last, light) {
                Some(b) => on.terrain.insert(id, b),
                None => on.terrain.remove(&id),
            };
        }
    }
    on.last = light;
    on.counts = counts;
}

/// `--pipboy SECTION[:PAGE]`: open it on this once the place is up.
#[derive(Resource, Default)]
pub struct StartPipboy(pub Option<String>);

/// `--pipboy-keys`: keys pressed in it, one a frame, once it's up.
#[derive(Resource, Default)]
pub struct StartPipboyKeys(pub Vec<String>);

/// `--pad`: the menus as with a 360 pad connected (for testing what only
/// shows with one).
#[derive(Resource, Default)]
pub struct PretendPad(pub bool);

/// A `--pipboy-keys` name as the key the menus get.
fn named_key(name: &str) -> Option<Key> {
    Some(match name {
        "up" => Key::Up,
        "down" => Key::Down,
        "left" => Key::Left,
        "right" => Key::Right,
        "enter" => Key::Activate,
        "padx" => Key::ButtonX,
        "pady" => Key::ButtonY,
        k if k.len() == 1 => Key::Letter(k.chars().next()?),
        _ => return None,
    })
}

/// The screen effect's constants (`ISIFSCANBLEND`'s `Params`,
/// `DistortParams`, `Tint`, `Offsets`).
#[derive(Clone, Copy, Debug, ShaderType)]
pub struct ScreenUniform {
    pub(crate) params: Vec4,
    pub(crate) distort: Vec4,
    pub(crate) tint: Vec4,
    pub(crate) offsets: Vec4,
}

/// The screen effect: the menus' picture, the scanlines and the band.
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct ScreenMaterial {
    #[uniform(0)]
    pub(crate) u: ScreenUniform,
    #[texture(1)]
    #[sampler(2)]
    pub(crate) picture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub(crate) scanlines: Handle<Image>,
    #[texture(5)]
    #[sampler(6)]
    pub(crate) band: Handle<Image>,
}

impl Material2d for ScreenMaterial {
    fn fragment_shader() -> ShaderRef {
        SCREEN_SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

/// The menus as built, and the pieces of their picture on screen.
struct Built {
    ui: ui::Ui,
    pipboy: ui::pipboy::Pipboy,
    sizes: HashMap<String, Option<(u32, u32)>>,
    atlases: HashMap<String, Option<ui::Atlas>>,
    images: HashMap<(String, bool, bool), Option<Handle<Image>>>,
    font_images: HashMap<(usize, u32), Option<Handle<Image>>>,
    last: Vec<DrawItem>,
    drawn: Vec<(Entity, Handle<Mesh>, Handle<TileMaterial>)>,
    /// Whether its menus show a pad's buttons (`ui::game::set_pad`).
    pad: bool,
}

/// The arm on screen.
struct Arm {
    worn: Vec<FormId>,
    /// The weapon held and its model (a mod fitted changes it).
    weapon: Option<(FormId, String)>,
    female: bool,
    lighting: u64,
    /// Frames since it was built: it is shown once its screen has its
    /// picture (or a few frames have gone), the arm it replaces kept
    /// until then, so changing what's worn doesn't blink the arm out.
    waited: u8,
    holder: Entity,
    root: Entity,
    joints: Vec<Entity>,
    skeleton: Arc<ActorSkeleton>,
    turned: Vec<usize>,
    looking: usize,
    camera: usize,
    raise: Option<Raise>,
    /// The screen piece, until its texture has been swapped.
    screen: Option<Entity>,
    light_effect: Vec<Entity>,
    /// The lamps over the STATS, ITEMS and DATA buttons.
    glows: Vec<Entity>,
    /// What the mouse picks (`007f8720`): `pipboyscreen:0`, and the
    /// shapes of `PipBoyButton01` .. `03` (STATS, ITEMS, DATA).
    screen_pick: Option<PickMesh>,
    button_picks: [Vec<PickMesh>; 3],
    /// The knobs' and needle's pieces, and the angles they're drawn at.
    knob_pieces: Vec<KnobPiece>,
    knobs_drawn: [f32; 3],
}

/// A piece's triangles as the GPU skins them, kept for picking with the
/// mouse: its bind-pose vertices and texture coordinates, and per joint
/// the joint's entity and the piece-to-joint (inverse bind) transform.
struct PickMesh {
    positions: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u16>,
    joints: Vec<(Entity, Mat4)>,
    joint_indices: Vec<[u16; 4]>,
    weights: Vec<[f32; 4]>,
}

impl PickMesh {
    fn new(data: &cellview::MeshData, joints: &[Entity]) -> Option<PickMesh> {
        let rig = data.rig.as_ref()?;
        let skin = crate::actors::skin_joints(data, joints);
        Some(PickMesh {
            positions: data
                .positions
                .iter()
                .map(|&p| Vec3::from_array(p))
                .collect(),
            uvs: data.uvs.iter().map(|&u| Vec2::from_array(u)).collect(),
            indices: data.indices.clone(),
            joints: skin
                .into_iter()
                .zip(&rig.joints)
                .map(|(e, (_, bind))| (e, Mat4::from_cols_array(bind)))
                .collect(),
            joint_indices: rig.joint_indices.clone(),
            weights: rig.joint_weights.clone(),
        })
    }

    /// The nearest place the ray meets the piece as posed now (skinned as
    /// the GPU does: the weighted joints' world matrices × their inverse
    /// binds), its distance along the ray and its texture coordinates.
    fn hit(&self, ray: Ray3d, globals: &Query<&GlobalTransform>) -> Option<(f32, Vec2)> {
        let mats: Vec<Mat4> = self
            .joints
            .iter()
            .map(|(e, bind)| {
                globals
                    .get(*e)
                    .map_or(Mat4::ZERO, |g| g.compute_matrix() * *bind)
            })
            .collect();
        let world: Vec<Vec3> = self
            .positions
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let (ji, w) = (self.joint_indices[i], self.weights[i]);
                (0..4)
                    .filter(|&k| w[k] > 0.0)
                    .map(|k| w[k] * mats[ji[k] as usize].transform_point3(p))
                    .sum()
            })
            .collect();
        let mut best: Option<(f32, Vec2)> = None;
        for tri in self.indices.as_chunks::<3>().0 {
            let [a, b, c] = tri.map(usize::from);
            let Some((t, u, v)) =
                ray_triangle(ray.origin, *ray.direction, world[a], world[b], world[c])
            else {
                continue;
            };
            if best.is_none_or(|(bt, _)| t < bt) {
                let uv = self.uvs[a] * (1.0 - u - v) + self.uvs[b] * u + self.uvs[c] * v;
                best = Some((t, uv));
            }
        }
        best
    }
}

/// Where a ray meets a triangle, either side (Möller–Trumbore): the
/// distance along the ray and the barycentric weights of `b` and `c`.
fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let (e1, e2) = (b - a, c - a);
    let p = dir.cross(e2);
    let det = e1.dot(p);
    if det.abs() < 1e-12 {
        return None;
    }
    let inv = 1.0 / det;
    let s = origin - a;
    let u = s.dot(p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(e1);
    let v = dir.dot(q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = e2.dot(q) * inv;
    (t > 0.0).then_some((t, u, v))
}

/// The point on the menus' picture a place on the screen's texture is
/// (`007f8720`: x = u × the screen's height × 4/3 (`01078128`
/// 1.333333), y = v × the height, in pixels of the 4:3 rectangle the
/// menus' camera covers with 1280 × 960 units, `007fba00`): u × 1280,
/// v × 960 menu units.
fn menu_point(uv: Vec2) -> [f32; 2] {
    [
        uv.x * ui::pipboy::PICTURE_SIZE[0],
        uv.y * ui::pipboy::PICTURE_SIZE[1],
    ]
}

/// The Pip-Boy's state.
#[derive(Resource, Default)]
pub struct Pipboy {
    built: Option<Box<Built>>,
    failed: bool,
    /// Up (or going up).
    pub open: bool,
    /// When it last went up or down (seconds), and whether that was at
    /// once (screenshots).
    since: Option<f32>,
    at_once: bool,
    /// Tab held since, and whether this hold switched the light.
    tab_down: Option<f32>,
    held_for_light: bool,
    pub light: bool,
    input: Option<PipboyInput>,
    effects: Option<ScreenEffects>,
    picture: Option<Handle<Image>>,
    screen: Option<Handle<Image>>,
    white: Option<Handle<Image>>,
    material: Option<Handle<ScreenMaterial>>,
    hum: Option<Entity>,
    arm: Option<Arm>,
    /// Arms replaced, kept until the new one is ready.
    retired: Vec<Entity>,
    /// A menu and page to show once filled.
    pending: Option<(Section, Option<usize>)>,
    /// The model's button the mouse button went down on (`011a0ba0`,
    /// 0 .. 2), kept until it comes up over the same one.
    button_down: Option<usize>,
    /// The left and right mouse buttons held, as their events said.
    mouse_held: [bool; 2],
    /// The marker "Do you want to travel to ...?" asks about (the map
    /// menu's `+0x118`), and the one to travel to once the Pip-Boy is
    /// down (`00798710` → `0070f690`, which runs `00798a00` when it has
    /// closed).
    travel_asked: Option<u32>,
    travel_after_close: Option<u32>,
    /// Where the player's own marker was asked for (the menu's `+0xf8` ..
    /// `+0x104`): the worldspace and the place.
    marker_asked: Option<(FormId, [f32; 3])>,
    /// The item waiting for "how many?" to drop.
    drop_asked: Option<u32>,
    /// The number keys (hot keys 1 to 8) held, as their events said.
    hotkeys_held: [bool; 8],
    /// ITEMS was up last frame (its `InventoryMenu` made: `0077fc10`
    /// asks for the weapons help as it's made).
    items_up: bool,
    /// A note's audio playing.
    note: Option<NotePlayback>,
    /// The world map's quest markers.
    quest_points: QuestPoints,
    /// The knobs and needle, their settings, and whether they're to be set
    /// up (on coming up, `007f8ba0` → `007f99d0`).
    knobs: Knobs,
    knob_settings: Option<KnobSettings>,
    knobs_fresh: bool,
}

impl Pipboy {
    /// Where the arm is in its raising animation at `now`: up to `Hit`
    /// while opening, held there, on from `Hit` to the end while being
    /// put away; `None` once it's down and away.
    fn raise_at(&self, now: f32) -> Option<f32> {
        let (start, hit, stop) = self
            .arm
            .as_ref()
            .and_then(|a| a.raise.as_ref())
            .map_or((0.0, 0.33, 0.73), |r| {
                (r.sequence.start, r.hit, r.sequence.stop)
            });
        let since = self
            .since
            .filter(|_| !self.at_once)
            .map(|s| (now - s).max(0.0));
        raise_time(start, hit, stop, self.open, since)
    }

    /// Whether the first-person model with the Pip-Boy (the arm, the
    /// hands, the weapon in hand) is on screen: up, or still being raised
    /// or put away. In the game this is the one first-person model
    /// (`007f8ba0` finds `pipboyscreen` under the first-person node,
    /// `00950bb0(1)`), playing `Pipboy.kf` over the hold pose; here
    /// it's drawn apart, so the ordinary first-person view
    /// (`viewmodel`) waits until it's down.
    pub fn arm_shown(&self, now: f32) -> bool {
        self.open || self.raise_at(now).is_some()
    }

    /// The shown menu's class number while it's up (STATS 1003, ITEMS
    /// 1002, DATA 1023).
    pub fn menu_class(&self) -> Option<i32> {
        let b = self.built.as_ref().filter(|_| self.open)?;
        Some(b.pipboy.class())
    }

    /// The class of its menu on top while it's up (STATS, ITEMS, DATA,
    /// or the repair or mod screen over ITEMS), for the tutorial manager.
    pub fn top_class(&self) -> Option<i32> {
        let b = self.built.as_ref().filter(|_| self.open)?;
        Some(b.pipboy.top_class())
    }

    /// Whether the game's cursor is hidden over the Pip-Boy (DATA's map
    /// under it: the highlight box stands in for it, `0079a130`).
    pub fn cursor_hidden(&self) -> bool {
        self.open
            && self
                .built
                .as_ref()
                .is_some_and(|b| b.pipboy.cursor_hidden())
    }
}

/// A picture to render into, read as stored values.
pub(crate) fn target_image(images: &mut Assets<Image>, size: UVec2) -> Handle<Image> {
    let mut image = Image::new_uninit(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        TextureFormat::Rgba16Float,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = crate::hud::sampler(false, false);
    images.add(image)
}

/// The two pictures and their cameras: the menus drawn into one, the
/// screen effect drawing that into the other.
fn setup_pipboy(
    mut commands: Commands,
    game: Res<GameFiles>,
    mut pipboy: ResMut<Pipboy>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ScreenMaterial>>,
    device: Option<Res<RenderDevice>>,
) {
    let game = &game.0;
    let picture = target_image(&mut images, PICTURE);
    let screen = target_image(&mut images, PICTURE);
    let camera = |target: &Handle<Image>, order: isize, layer: usize| {
        (
            Camera2d,
            Camera {
                target: RenderTarget::from(target.clone()),
                order,
                hdr: true,
                clear_color: ClearColorConfig::Custom(Color::NONE),
                is_active: false,
                ..default()
            },
            Tonemapping::None,
            DebandDither::Disabled,
            Msaa::Off,
            RenderLayers::layer(layer),
            PipboyCamera,
        )
    };
    commands.spawn(camera(&picture, -5, MENU_LAYER));
    commands.spawn(camera(&screen, -4, SCREEN_LAYER));
    let compressed = device
        .as_ref()
        .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
    let mut white = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    white.sampler = crate::hud::sampler(false, false);
    let white = images.add(white);
    // `sScanlineTexture:InterfaceFX` (`Data\Textures\Pipboy3000\
    // PipboyScanlines.dds`), repeated; the band's map, clamped.
    let scanlines = crate::hud::upload_picture(
        &mut images,
        game,
        "textures\\pipboy3000\\pipboyscanlines.dds",
        (true, true),
        compressed,
    )
    .unwrap_or_else(|| white.clone());
    let band = crate::hud::upload_picture(
        &mut images,
        game,
        "textures\\pipboy3000\\pipboydistorteffectmap.dds",
        (false, false),
        compressed,
    )
    .unwrap_or_else(|| white.clone());
    let material = materials.add(ScreenMaterial {
        u: ScreenUniform {
            params: Vec4::new(0.0, 0.0, 1.0, 0.0),
            distort: Vec4::ZERO,
            tint: Vec4::ONE,
            offsets: Vec4::ZERO,
        },
        picture: picture.clone(),
        scanlines,
        band,
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PICTURE.x as f32, PICTURE.y as f32))),
        MeshMaterial2d(material.clone()),
        Transform::IDENTITY,
        RenderLayers::layer(SCREEN_LAYER),
    ));
    let ini = |s: &str, k: &str| game.settings.get(s, k).map(str::to_string);
    pipboy.effects = Some(ScreenEffects::new(ScreenSettings::from_ini(&ini), 0.0));
    pipboy.picture = Some(picture);
    pipboy.screen = Some(screen);
    pipboy.white = Some(white);
    pipboy.material = Some(material);
}

/// The Pip-Boy's two cameras (on only while it's up).
#[derive(Component)]
pub struct PipboyCamera;

/// Reads the menus from the game's files.
fn build(game: &cellview::Game) -> Result<Built, String> {
    let mut read = |p: &str| game.assets.read(p).ok().flatten();
    let ini = |s: &str, k: &str| game.settings.get(s, k).map(str::to_string);
    let (w, h) = (
        game.settings
            .get("Display", "iSize W")
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(1920),
        game.settings
            .get("Display", "iSize H")
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(1080),
    );
    let mut ui = ui::game::new_ui(
        &mut read,
        &ini,
        crate::hud::text_settings(&game.order),
        w,
        h,
    );
    let pipboy = ui::pipboy::Pipboy::load(&mut ui, &mut read)?;
    for w in &ui.warnings {
        println!("  Pip-Boy: {w}");
    }
    Ok(Built {
        ui,
        pipboy,
        sizes: HashMap::new(),
        atlases: HashMap::new(),
        images: HashMap::new(),
        font_images: HashMap::new(),
        last: Vec::new(),
        drawn: Vec::new(),
        pad: false,
    })
}

/// A sound record by editor ID, queued.
fn sound(order: &esm::LoadOrder, requests: &mut SoundRequests, name: &str) {
    if let Some(id) = order.form_by_editor_id(name) {
        requests.0.push(id);
    }
}

/// The menu keys pressed this frame, as the game reads a PC keyboard in
/// its menus (`007154b0`, `0070c4a0`): the arrows (Shift with left or
/// right: the previous or next menu), Enter (Shift: the X button, Alt: the
/// Y button), and the letters for the menus' `_PCButton_` traits.
fn menu_keys(keys: &ButtonInput<KeyCode>) -> Vec<Key> {
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let alt = keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
    let mut out = Vec::new();
    let pressed = |code: KeyCode| keys.just_pressed(code);
    if pressed(KeyCode::ArrowUp) {
        out.push(Key::Up);
    }
    if pressed(KeyCode::ArrowDown) {
        out.push(Key::Down);
    }
    if pressed(KeyCode::ArrowLeft) {
        out.push(if shift { Key::PrevSection } else { Key::Left });
    }
    if pressed(KeyCode::ArrowRight) {
        out.push(if shift { Key::NextSection } else { Key::Right });
    }
    if pressed(KeyCode::PageUp) {
        out.push(Key::PageUp);
    }
    if pressed(KeyCode::PageDown) {
        out.push(Key::PageDown);
    }
    if pressed(KeyCode::Enter) || pressed(KeyCode::NumpadEnter) {
        out.push(if shift {
            Key::ButtonX
        } else if alt {
            Key::ButtonY
        } else {
            Key::Activate
        });
    }
    for (i, code) in [
        KeyCode::KeyA,
        KeyCode::KeyB,
        KeyCode::KeyC,
        KeyCode::KeyD,
        KeyCode::KeyE,
        KeyCode::KeyF,
        KeyCode::KeyG,
        KeyCode::KeyH,
        KeyCode::KeyI,
        KeyCode::KeyJ,
        KeyCode::KeyK,
        KeyCode::KeyL,
        KeyCode::KeyM,
        KeyCode::KeyN,
        KeyCode::KeyO,
        KeyCode::KeyP,
        KeyCode::KeyQ,
        KeyCode::KeyR,
        KeyCode::KeyS,
        KeyCode::KeyT,
        KeyCode::KeyU,
        KeyCode::KeyV,
        KeyCode::KeyW,
        KeyCode::KeyX,
        KeyCode::KeyY,
        KeyCode::KeyZ,
    ]
    .into_iter()
    .enumerate()
    {
        if pressed(code) {
            out.push(Key::Letter((b'a' + i as u8) as char));
        }
    }
    out
}
/// What opening, closing and the light need besides the Pip-Boy itself.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Around<'w> {
    time: Res<'w, Time>,
    game: Res<'w, GameFiles>,
    state: ResMut<'w, DialogueState>,
    menus: ResMut<'w, Menus>,
    player: ResMut<'w, Player>,
    conversation: Res<'w, crate::dialogue::Conversation>,
    requests: ResMut<'w, SoundRequests>,
    messages: ResMut<'w, crate::hud::HudMessages>,
    wavs: ResMut<'w, Assets<PcmSound>>,
    markers: Res<'w, crate::map::MapMarkers>,
    asks: ResMut<'w, crate::game_menus::asks::PipboyAsks>,
    collision: Res<'w, crate::walk::CellCollision>,
    controls: Option<Res<'w, crate::controls::Controls>>,
    real: Res<'w, Time<bevy::time::Real>>,
    radio_out: ResMut<'w, crate::radio::RadioOut>,
}

/// The mouse as the Pip-Boy reads it: the window's pointer, the first-
/// person camera that draws the arm (its ray through the pointer), the
/// joints' places, the left button and the wheel.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Mouse<'w, 's> {
    windows: Query<'w, 's, &'static Window, With<bevy::window::PrimaryWindow>>,
    camera: Query<
        'w,
        's,
        (&'static Camera, &'static GlobalTransform),
        With<crate::viewmodel::FirstPersonCamera>,
    >,
    globals: Query<'w, 's, &'static GlobalTransform>,
    buttons: ResMut<'w, ButtonInput<MouseButton>>,
    /// The buttons' own events: the Pip-Boy keeps the mouse from the rest
    /// of the viewer by clearing `buttons` every frame, which also drops
    /// what `buttons` would say about a button let go later, so the
    /// Pip-Boy reads the presses and releases themselves.
    events: EventReader<'w, 's, bevy::input::mouse::MouseButtonInput>,
    scroll: ResMut<'w, bevy::input::mouse::AccumulatedMouseScroll>,
    /// A connected pad, for its sticks on DATA's maps.
    pads: Query<'w, 's, &'static Gamepad>,
    /// The keys' own events, for the number keys (hot keys), which the
    /// Pip-Boy clears from `ButtonInput` while it's up.
    keyboard: EventReader<'w, 's, bevy::input::keyboard::KeyboardInput>,
    /// The view, for the player's facing (drops land in front of it).
    view: Query<'w, 's, &'static Transform, With<FlyCamera>>,
    /// `--menu-pointer` and `--menu-click` (for testing: screenshots have
    /// no mouse), which the Pip-Boy takes while it's up and no game menu
    /// is over it.
    fixed: Res<'w, crate::game_menus::FixedPointer>,
    clicks: ResMut<'w, crate::game_menus::FixedClicks>,
}

/// The view's heading on the ground (radians clockwise from north, game
/// space), 0 without a view.
fn view_heading(view: &Query<&Transform, With<FlyCamera>>) -> f32 {
    view.single().map_or(0.0, |t| {
        let f = t.forward().as_vec3();
        f.x.atan2(-f.z)
    })
}

/// The hot key wheel's controls' keys (0x11 .. 0x18: Hotkey1, Ammo Swap,
/// Hotkey3 .. Hotkey8), from the INI or the exe's defaults
/// (`controls::Controls::hotkeys`, `00a24b70`: the digits 1 to 8).
fn hotkey_keys(controls: Option<&crate::controls::Controls>) -> [Option<KeyCode>; 8] {
    let c = controls.copied().unwrap_or_default();
    c.hotkeys.map(|b| b.key)
}

/// The hot keys' keys from their events: held now (carried over in
/// `held`), and those that came up this frame. Each control is looked up
/// on its own (`00a24660`), so a key bound to two of them holds both.
fn hotkey_events(
    keys: &[Option<KeyCode>; 8],
    held: &mut [bool; 8],
    events: impl Iterator<Item = (KeyCode, bevy::input::ButtonState)>,
) -> [bool; 8] {
    let mut released = [false; 8];
    for (code, state) in events {
        for n in (0..8).filter(|&n| keys[n] == Some(code)) {
            match state {
                bevy::input::ButtonState::Pressed => held[n] = true,
                bevy::input::ButtonState::Released => {
                    if held[n] {
                        released[n] = true;
                    }
                    held[n] = false;
                }
            }
        }
    }
    released
}

/// The mouse buttons this frame from their events, `held` carried over
/// from earlier frames: (left, right). A button that went down and came
/// up within one frame counts as both pressed and released.
fn mouse_buttons(
    held: &mut [bool; 2],
    events: impl Iterator<Item = (MouseButton, bevy::input::ButtonState)>,
) -> [ui::pipboy::Button; 2] {
    let mut out = [ui::pipboy::Button::default(); 2];
    for (button, state) in events {
        let i = match button {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            _ => continue,
        };
        match state {
            bevy::input::ButtonState::Pressed => {
                if !held[i] {
                    out[i].pressed = true;
                }
                held[i] = true;
            }
            bevy::input::ButtonState::Released => {
                if held[i] {
                    out[i].released = true;
                }
                held[i] = false;
            }
        }
    }
    for i in 0..2 {
        out[i].down = held[i];
    }
    out
}

/// The function keys that open or turn to a menu (`0070c4a0` reads the
/// keys themselves, `00a24180` DIK 0x3B .. 0x3D): F1 STATS, F2 ITEMS, F3
/// DATA.
fn section_key(keys: &ButtonInput<KeyCode>) -> Option<Section> {
    if keys.just_pressed(KeyCode::F1) {
        Some(Section::Stats)
    } else if keys.just_pressed(KeyCode::F2) {
        Some(Section::Items)
    } else if keys.just_pressed(KeyCode::F3) {
        Some(Section::Data)
    } else {
        None
    }
}

/// Tab, the light, the function keys, and the mouse and keys while it's
/// up.
fn pipboy_keys(
    mut commands: Commands,
    mut pipboy: ResMut<Pipboy>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut start_keys: ResMut<StartPipboyKeys>,
    around: Around,
    mut mouse: Mouse,
) {
    let Around {
        time,
        game,
        mut state,
        mut menus,
        mut player,
        conversation,
        mut requests,
        mut messages,
        markers,
        mut asks,
        mut wavs,
        collision,
        controls,
        real,
        mut radio_out,
        ..
    } = around;
    let order = &game.0.order;
    let now = time.elapsed_secs();
    let pipboy = &mut *pipboy;
    // The mouse buttons, read from their events every frame (up or not,
    // so what's held is known on opening): `buttons` is cleared below
    // while it's up, so it would never see a button come up.
    let [button, right] = mouse_buttons(
        &mut pipboy.mouse_held,
        mouse.events.read().map(|e| (e.button, e.state)),
    );
    let hotkeys_released = hotkey_events(
        &hotkey_keys(controls.as_deref()),
        &mut pipboy.hotkeys_held,
        mouse.keyboard.read().map(|e| (e.key_code, e.state)),
    );
    // Scripts can take the Pip-Boy away (`DisablePlayerControls`).
    let allowed = !state.0.controls_off[world::scripting::controls::PIPBOY];
    let free = !menus.others_open() && conversation.0.is_none();
    // A hot key let go in the game (`HUDMainMenu::UpdateHotkeysWheel`
    // (Xbox PDB), `0077da60`, PC: control 0x11 + n comes up →
    // `HotKeysWheel::UseHotkeyItem`, `00701e80`): its item taken off when
    // it's equipped, else equipped or used. Every n, the 2 key's (Ammo
    // Swap's control, n 1) too: going down it only selects slot 1
    // (`0061cc40(1)` on the wheel's +0x28, no name shown, `00701bd0(1)`
    // not called), held it never shows the wheel (`n != 1`), and coming up
    // it uses slot 1, which nothing can be put on (the Pip-Boy's wheel
    // skips it, `00781ba0`, `007017b0`): nothing happens. The ammunition
    // swap itself is the player's controls' (`0093e860`, control 0x12
    // going down; `combat`).
    if !pipboy.open && free && player.ready {
        for (n, _) in hotkeys_released.iter().enumerate().filter(|&(_, &r)| r) {
            use_hotkey(order, &mut state.0, n);
        }
    }

    // The answers to the Pip-Boy's questions (their callbacks).
    for answer in std::mem::take(&mut asks.answers) {
        let state = &mut state.0;
        match answer.owner {
            // `00798710`: Yes (0) puts the Pip-Boy away and travels once
            // it's down (`0070f690` with `00798a00`); No forgets the
            // marker.
            asks::TRAVEL => {
                let marker = pipboy.travel_asked.take();
                if answer.value == 0 && pipboy.open {
                    close(
                        &mut commands,
                        pipboy,
                        &mut menus,
                        &mut player,
                        &conversation,
                        now,
                    );
                    sound(order, &mut requests, "UIPipBoyAccessDown");
                    pipboy.travel_after_close = marker;
                }
            }
            // `00798840`: Move It (0) or Yes (3, the set box's first
            // number) sets the marker at the place asked (`00952e60`);
            // Remove It (1) removes it (`00952f90`); Leave It and No
            // leave it.
            asks::SET_MARKER => {
                let asked = pipboy.marker_asked.take();
                match (answer.value, asked) {
                    (0 | 3, Some((space, at))) => {
                        world::map::set_custom_marker(state, space, at);
                        println!("Map marker set at {:.0}, {:.0}.", at[0], at[1]);
                    }
                    (1, _) => {
                        world::map::remove_custom_marker(state);
                        println!("Map marker removed.");
                    }
                    _ => {}
                }
            }
            // "How many?" answered (`00780c50`): that many dropped, none
            // for Cancel (0).
            asks::DROP => {
                if let Some(form) = pipboy.drop_asked.take() {
                    if answer.value > 0 {
                        drop_items(
                            order,
                            state,
                            &mut requests,
                            &collision.0,
                            FormId(form),
                            answer.value,
                            view_heading(&mouse.view),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    // ITEMS made (`InventoryMenu::Create`, `0077fc10`): it asks for the
    // weapons help over itself (`ShowMessage(0x22, 1002, 512)`).
    let items_up = pipboy.open
        && pipboy
            .built
            .as_ref()
            .is_some_and(|b| b.pipboy.section == Section::Items);
    if items_up && !pipboy.items_up {
        use world::tutorial::{ask, id, menu};
        ask(
            order,
            &mut state.0.tutorials,
            id::WEAPONS,
            menu::INVENTORY,
            menu::DELAY,
        );
    }
    pipboy.items_up = items_up;

    // One of the game's own menus over the Pip-Boy (the fast-travel
    // question, "how many?", a help message) takes the keys and the
    // mouse, as the game's interface hands them to the menu on top
    // (`0070f6e0`; how it orders the Pip-Boy's menus under a box isn't
    // traced further).
    if pipboy.open && menus.game_open {
        return;
    }
    let section_key = section_key(&keys);
    if pipboy.open {
        // The Pip-Boy control let go again puts it away (`0070c4a0`:
        // control 14 come up, `00a24660(0xe, 2)`, → `0070f690`); so does
        // the function key of the menu shown, while another's turns to
        // its menu (`00704c10` / `007048f0` / `00704170`).
        let shown = pipboy.built.as_ref().map(|b| b.pipboy.section);
        let close_now =
            keys.just_released(KeyCode::Tab) || (section_key.is_some() && section_key == shown);
        if close_now {
            close(
                &mut commands,
                pipboy,
                &mut menus,
                &mut player,
                &conversation,
                now,
            );
            sound(order, &mut requests, "UIPipBoyAccessDown");
            keys.reset(KeyCode::Tab);
            return;
        }
        if let (Some(section), Some(b)) = (section_key, pipboy.built.as_mut()) {
            b.pipboy.show(&mut b.ui, section);
            b.ui.refresh();
            if let Some(fx) = pipboy.effects.as_mut() {
                fx.tab_changed(now * 1000.0);
            }
        }
    } else {
        // F1 .. F3 put it up on their menu (`0070f4e0(0, class)`).
        if let Some(section) = section_key {
            if free && allowed && open(pipboy, &mut menus, &mut player, now, false) {
                pipboy.pending = Some((section, None));
                sound(order, &mut requests, "UIPipBoyAccessUp");
            }
            return;
        }
        // Tab (control 14, `00a24b70`'s default): held past
        // `fPlayerPipBoyLightTimer`, the light; let go sooner, the Pip-Boy
        // (`009673d0`).
        if keys.just_pressed(KeyCode::Tab) && free && allowed {
            pipboy.tab_down = Some(now);
            pipboy.held_for_light = false;
        }
        if let Some(down) = pipboy.tab_down {
            if !pipboy.held_for_light && now - down >= LIGHT_HOLD_SECONDS {
                pipboy.held_for_light = true;
                pipboy.light = !pipboy.light;
                sound(
                    order,
                    &mut requests,
                    if pipboy.light {
                        "UIPipBoyLightOn"
                    } else {
                        "UIPipBoyLightOff"
                    },
                );
                println!(
                    "The Pip-Boy light is {}.",
                    if pipboy.light { "on" } else { "off" }
                );
            }
            if keys.just_released(KeyCode::Tab) || !keys.pressed(KeyCode::Tab) {
                pipboy.tab_down = None;
                if !pipboy.held_for_light
                    && free
                    && allowed
                    && open(pipboy, &mut menus, &mut player, now, false)
                {
                    sound(order, &mut requests, "UIPipBoyAccessUp");
                }
            }
        }
        if !pipboy.open {
            return;
        }
    }
    // `--pipboy-keys`' "close": put away as the Pip-Boy control does.
    if pipboy.input.is_some() && start_keys.0.first().is_some_and(|k| k == "close") {
        start_keys.0.remove(0);
        println!("--pipboy-keys: close");
        close(
            &mut commands,
            pipboy,
            &mut menus,
            &mut player,
            &conversation,
            now,
        );
        sound(order, &mut requests, "UIPipBoyAccessDown");
        return;
    }
    let mut pressed = menu_keys(&keys);
    // `--pipboy-keys`: one a frame once its menus are filled.
    if pipboy.input.is_some() && !start_keys.0.is_empty() {
        let name = start_keys.0.remove(0);
        println!("--pipboy-keys: {name}");
        pressed.extend(named_key(&name));
    }
    // The keyboard is the Pip-Boy's while it's up (Escape left to the
    // viewer).
    let held: Vec<KeyCode> = keys
        .get_pressed()
        .copied()
        // Tab stays, so that letting it go puts the Pip-Boy away.
        .filter(|&k| k != KeyCode::Escape && k != KeyCode::Tab)
        .collect();
    for k in held {
        keys.reset(k);
    }
    // The mouse is the Pip-Boy's too (no looking, attacking or the
    // flying camera's speed while it's up).
    let notches = match mouse.scroll.unit {
        bevy::input::mouse::MouseScrollUnit::Line => mouse.scroll.delta.y,
        bevy::input::mouse::MouseScrollUnit::Pixel => mouse.scroll.delta.y / 40.0,
    }
    .round() as i32;
    mouse.buttons.reset_all();
    mouse.scroll.delta = Vec2::ZERO;
    // `--menu-pointer` / `--menu-click`: the pointer at that pixel of the
    // 1920 × 1080 picture, the left button pressed one frame and let go
    // the next.
    let mut button = button;
    if mouse.fixed.0.is_some() {
        let release = std::mem::take(&mut mouse.clicks.release);
        let due = mouse.clicks.at.iter().position(|&t| t <= f64::from(now));
        let press = due.map(|i| mouse.clicks.at.remove(i)).is_some();
        mouse.clicks.release = press;
        button.pressed |= press;
        button.down |= press;
        button.released |= release;
    }
    let fixed = mouse.fixed.0;
    // Where the pointer is on the screen and on the model's buttons
    // (`007f8720`), while the arm is up and posed.
    let ray = mouse
        .windows
        .single()
        .ok()
        .and_then(|w| {
            fixed
                .map(|(x, y)| Vec2::new(x, y) / w.scale_factor())
                .or_else(|| w.cursor_position())
        })
        .zip(mouse.camera.single().ok())
        .and_then(|(at, (camera, global))| camera.viewport_to_world(global, at).ok());
    let mut at = None;
    let mut model_button = None;
    if let (Some(ray), Some(arm)) = (ray, pipboy.arm.as_ref()) {
        at = arm
            .screen_pick
            .as_ref()
            .and_then(|m| m.hit(ray, &mouse.globals))
            .map(|(_, uv)| menu_point(uv));
        if button.pressed || button.released {
            let over = arm
                .button_picks
                .iter()
                .position(|picks| picks.iter().any(|m| m.hit(ray, &mouse.globals).is_some()));
            if button.pressed {
                // Kept from an earlier press when this one isn't on a
                // button, as the game keeps `011a0ba0`.
                if over.is_some() {
                    pipboy.button_down = over;
                }
            } else if over.is_some() && over == pipboy.button_down {
                model_button = over;
                pipboy.button_down = None;
            }
        }
    }
    let (Some(b), Some(input)) = (pipboy.built.as_mut(), pipboy.input.clone()) else {
        return;
    };
    let before_pointer = (
        b.pipboy.section,
        b.pipboy.stats.page,
        b.pipboy.items.tab,
        b.pipboy.data.tab,
    );
    let mut actions = b.pipboy.pointer_with_right(
        &mut b.ui,
        at,
        button,
        right,
        keys.pressed(KeyCode::Tab),
        f64::from(now),
        &input,
    );
    if notches != 0 {
        actions.extend(b.pipboy.wheel(&mut b.ui, notches, &input));
    }
    // A pad's sticks zoom and pan DATA's maps (`00799790`: the left stick's
    // up and down, the right stick), as the 360 pad's -32767 .. 32767.
    if let Some(pad) = mouse.pads.iter().next() {
        let axis = |a: GamepadAxis| (pad.get(a).unwrap_or(0.0) * 32767.0).round() as i32;
        actions.extend(b.pipboy.sticks(
            &mut b.ui,
            axis(GamepadAxis::LeftStickY),
            [
                axis(GamepadAxis::RightStickX),
                axis(GamepadAxis::RightStickY),
            ],
        ));
    }
    b.pipboy.hotkey_keys(&mut b.ui, pipboy.hotkeys_held, &input);
    if let Some(i) = model_button {
        actions.extend(b.pipboy.press_section(&mut b.ui, i + 1));
    }
    let after_pointer = (
        b.pipboy.section,
        b.pipboy.stats.page,
        b.pipboy.items.tab,
        b.pipboy.data.tab,
    );
    if before_pointer != after_pointer {
        if let Some(fx) = pipboy.effects.as_mut() {
            fx.tab_changed(now * 1000.0);
        }
    }
    for key in pressed {
        let before = (
            b.pipboy.section,
            b.pipboy.stats.page,
            b.pipboy.items.tab,
            b.pipboy.data.tab,
        );
        actions.extend(b.pipboy.key(&mut b.ui, key, &input));
        let after = (
            b.pipboy.section,
            b.pipboy.stats.page,
            b.pipboy.items.tab,
            b.pipboy.data.tab,
        );
        if before != after {
            if let Some(fx) = pipboy.effects.as_mut() {
                fx.tab_changed(now * 1000.0);
            }
        }
    }
    // The limb STATS' healing mode aims at, for the effects the Stimpak
    // adds now (`00823210` asks the stats menu, `007e06b0`).
    let aim = b
        .pipboy
        .stats
        .healing_part()
        .filter(|_| b.pipboy.section == Section::Stats);
    let state = &mut state.0;
    state.healing_part = aim;
    // A corner message with its picture (`world::message_icon`).
    let mut say = |text: String, icon: Option<&str>| {
        if text.is_empty() {
            return;
        }
        println!("{text}");
        if messages.on {
            messages
                .queue
                .push(crate::hud::HudMessage::with_icon(text, icon));
        }
    };
    // A queue: what the repair and mod screens answer is carried out too.
    let mut actions: std::collections::VecDeque<Action> = actions.into();
    while let Some(action) = actions.pop_front() {
        match action {
            Action::Sound(name) => sound(order, &mut requests, &name),
            Action::Equip(form) => {
                let item = FormId(form);
                // Translated from 00780d60 (decompiled, FalloutNV.exe
                // 1.4.0.525): `0088c790` takes it off, `0088c650` puts it
                // on, both with their sound flag set; the player's sound
                // is `008aded0` -> `008adcf0` (down on taking off, up on
                // putting on: the item's own `ZNAM` / `YNAM`, else its
                // type's `UIItem...`). A refused equip (a broken item)
                // returns before it.
                let was_worn = state.is_equipped(PLAYER_REF, item);
                if was_worn {
                    state.unequip_item(order, PLAYER_REF, item);
                } else {
                    state.equip(order, PLAYER_REF, item);
                }
                if state.is_equipped(PLAYER_REF, item) != was_worn {
                    println!(
                        "Pip-Boy: {item} {}.",
                        if was_worn { "taken off" } else { "equipped" }
                    );
                    if let Some(s) = world::sound::item_sound(order, item, !was_worn) {
                        println!(
                            "Pip-Boy: sound {s} ({:?}).",
                            order
                                .get(s)
                                .and_then(|r| r.record().ok())
                                .and_then(|r| r.editor_id())
                        );
                        requests.0.push(s);
                    }
                }
            }
            Action::Use(form) => {
                let item = FormId(form);
                let kind = order.get(item).map(|r| *r.entry.header.kind.as_bytes());
                match kind {
                    // A book's own notice (its skill raised) is the
                    // game's.
                    // (`sSkillIncreasedNum`, type 1: very happy.)
                    // Read: `UIItemGenericUp` (`0088c830` case 0x19:
                    // `008aded0(book, 1, 1)`; the sound flag set, so not
                    // the book's own).
                    Some(k) if &k == b"BOOK" => {
                        say(
                            world::items::read_book(order, state, item).unwrap_or_default(),
                            world::message_icon::for_setting("sSkillIncreasedNum"),
                        );
                        sound(order, &mut requests, "UIItemGenericUp");
                    }
                    // Aid: no notice (the viewer's own line on the
                    // console only).
                    _ => {
                        if let Some(done) = world::items::use_item(order, state, PLAYER_REF, item) {
                            println!("{done}");
                        }
                    }
                }
            }
            // `00796fd0` case 0x1a: when the player can fast travel from
            // here (`0093d660`, which says why not) and to the marker
            // (`00438ef0`), "%s %s?" with `sTravelQuestion` and the
            // marker's name, Yes and No (`00703e80`, callback `00798710`).
            Action::Travel(reference) => {
                let Some(m) = markers
                    .list
                    .iter()
                    .find(|m| m.reference.0 == reference)
                    .cloned()
                else {
                    continue;
                };
                if let Some((why, icon)) = world::map::travel_refusal(order, state) {
                    say(why, Some(icon));
                    continue;
                }
                if !world::map::can_travel(state, &m) {
                    continue;
                }
                pipboy.travel_asked = Some(reference);
                let text = |n: &str| setting_text(pipboy, n);
                asks.asks.push(Ask::Box {
                    owner: asks::TRAVEL,
                    text: format!("{} {}?", text("sTravelQuestion"), m.name),
                    buttons: vec![text("sYes"), text("sNo")],
                    first_number: 0,
                });
            }
            // `00796fd0` case 0x0c on the world map: the place under the
            // pointer (`0079c450`), `UIPopUpMapMarkerAdded ` (the exe's
            // name ends with a space; looked up as written), then
            // `sMoveMarkerQuestion` with Move It, Remove It and Leave It
            // when the player has a marker (`00798400`), else
            // `sSetMarkerQuestion` with Yes and No answering from 3
            // (callback `00798840`).
            Action::PlaceMarker(at_map) => {
                let (Some(space), Some(map)) = (markers.world, input.world_map.as_ref()) else {
                    continue;
                };
                let at = world::map::map_to_world(map.corners[0], map.corners[1], at_map);
                pipboy.marker_asked = Some((space, at));
                sound(order, &mut requests, "UIPopUpMapMarkerAdded ");
                let text = |n: &str| setting_text(pipboy, n);
                let ask = if state.custom_marker.is_some() {
                    Ask::Box {
                        owner: asks::SET_MARKER,
                        text: text("sMoveMarkerQuestion"),
                        buttons: vec![
                            text("sMoveMarker"),
                            text("sRemoveMarker"),
                            text("sLeaveMarker"),
                        ],
                        first_number: 0,
                    }
                } else {
                    Ask::Box {
                        owner: asks::SET_MARKER,
                        text: text("sSetMarkerQuestion"),
                        buttons: vec![text("sYes"), text("sNo")],
                        first_number: 3,
                    }
                };
                asks.asks.push(ask);
            }
            // `00780140` case 7 (the mouse's right button or Drop, the
            // pad's X): the game's refusals in turn as a corner message
            // (`007052f0`, `world::items::drop_refusal`: a quest item, an
            // equipped one during an action, in the air); more than
            // `iInventoryAskQuantityAt` asks "how many?" (`007aba00`, from
            // all of them), else one is dropped (`00780c50(1)`). (The
            // player's current action, `008a7570`, isn't tracked here; a
            // worn item that can't come off and no room aren't checked.)
            Action::Drop(form) => {
                let item = FormId(form);
                let in_air = player.walking && !player.character.on_ground;
                if let Some(setting) = world::items::drop_refusal(order, state, item, false, in_air)
                {
                    say(
                        setting_text(pipboy, setting),
                        world::message_icon::for_setting(setting),
                    );
                    continue;
                }
                let count = state.item_count(order, PLAYER_REF, item);
                if world::items::drop_asks(order, count) {
                    pipboy.drop_asked = Some(form);
                    asks.asks.push(Ask::HowMany {
                        owner: asks::DROP,
                        most: count,
                    });
                } else {
                    let heading = view_heading(&mouse.view);
                    drop_items(order, state, &mut requests, &collision.0, item, 1, heading);
                }
            }
            Action::ActiveQuest(form) => state.active_quest = Some(FormId(form)),
            Action::Radio(station) => {
                let now = crate::music::audio_clock(&real);
                crate::radio::click(state, &mut radio_out, station, now);
            }
            // `007019e0` → `004bf800`: the item onto that hot key, off any
            // other.
            Action::SetHotkey { slot, item } => {
                let item = FormId(item);
                for h in state.hotkeys.iter_mut() {
                    if *h == Some(item) {
                        *h = None;
                    }
                }
                if let Some(h) = state.hotkeys.get_mut(slot) {
                    *h = Some(item);
                    println!("Hot key {}: {item}.", slot + 1);
                }
            }
            Action::Notice(text) => say(text, None),
            // `007f8610(0, ±fScrollKnobIncrement, fScrollKnobRate)`.
            Action::ScrollKnob { down } => {
                if let Some(s) = pipboy.knob_settings {
                    let by = if down {
                        s.scroll_increment
                    } else {
                        -s.scroll_increment
                    };
                    pipboy.knobs.turn(0, by, s.scroll_rate);
                }
            }
            // `00798ad0`: the note's audio stops.
            Action::StopNote => {
                if let Some(p) = pipboy.note.take() {
                    if let Some(e) = p.entity {
                        if let Ok(mut e) = commands.get_entity(e) {
                            e.despawn();
                        }
                    }
                }
            }
            // `00796fd0` case 0x18 → `0079a660`: the pieces queued, the
            // whole length their lengths and 500 ms between each two, the
            // first started.
            Action::PlayNote(form) => {
                let pieces = note_pieces(&game.0, state, FormId(form));
                if pieces.is_empty() {
                    println!("Note {}: no audio found.", FormId(form));
                    continue;
                }
                let total_ms =
                    pieces.iter().map(|p| p.2).sum::<f32>() + (pieces.len() as f32 - 1.0) * 500.0;
                let entity = play_piece(&mut commands, &mut wavs, &pieces[0].0, pieces[0].1);
                println!(
                    "Note {}: {} piece(s), {:.1} s.",
                    FormId(form),
                    pieces.len(),
                    total_ms / 1000.0
                );
                pipboy.note = Some(NotePlayback {
                    note: FormId(form),
                    pieces,
                    index: 0,
                    entity,
                    started: now,
                    piece_started: now,
                    gap_from: None,
                    total_ms,
                    done: false,
                });
            }
            // ITEMS' Repair: the screen on that item (`007b7020`).
            Action::OpenRepair(form) => {
                let input = ui::pipboy::gather::repair_input(order, state, FormId(form));
                if let Some(b) = pipboy.built.as_mut() {
                    actions.extend(b.pipboy.open_repair(&mut b.ui, input));
                }
            }
            // Mending with a part (`007b5d80`), then the screen again or
            // ITEMS (`007b5b40`).
            Action::Repair { chosen, part } => {
                let to = world::repair::repair_with(order, state, FormId(chosen), FormId(part));
                println!("Repaired {chosen:08X} with {part:08X}: {:.0}%.", to * 100.0);
                let input = ui::pipboy::gather::repair_input(order, state, FormId(chosen));
                if let Some(b) = pipboy.built.as_mut() {
                    actions.extend(b.pipboy.repaired(&mut b.ui, input));
                }
            }
            // The weapon mod screen (ITEMS' Mod, `00784710`).
            Action::OpenItemMod(form) => {
                let input = ui::pipboy::gather::item_mod_input(order, state, FormId(form));
                if let Some(b) = pipboy.built.as_mut() {
                    actions.extend(b.pipboy.open_item_mod(&mut b.ui, input));
                }
            }
            // `ShowMessage` (`00718630`): ITEMS' tab buttons.
            Action::Tutorial { id, menu, delay } => {
                world::tutorial::ask(order, &mut state.tutorials, id, menu, delay);
            }
            // A mod fitted (`007838a0` → `00783af0`): one used up, the
            // sound, the list again.
            Action::FitMod { weapon, item } => {
                let (weapon, item) = (FormId(weapon), FormId(item));
                if world::weapon_mods::attach(order, state, PLAYER_REF, weapon, item) {
                    println!("Fitted {item} to {weapon}.");
                    sound(order, &mut requests, ui::pipboy::item_mod::FIT_SOUND);
                }
                let input = ui::pipboy::gather::item_mod_input(order, state, weapon);
                if let Some(b) = pipboy.built.as_mut() {
                    actions.extend(b.pipboy.item_modded(&mut b.ui, input));
                }
            }
        }
    }
}
/// A note's audio playing (the map menu's sound list `+0x98`, `0079a660`):
/// its pieces in order (a sound note's one sound; a voice note's line's
/// responses, each a voice file), their lengths, the one playing.
struct NotePlayback {
    note: FormId,
    pieces: Vec<(Vec<u8>, bool, f32)>,
    index: usize,
    entity: Option<Entity>,
    /// When it began, when the piece playing began, and when the last
    /// ended (the next starts 500 ms later, `0079a660`).
    started: f32,
    piece_started: f32,
    gap_from: Option<f32>,
    total_ms: f32,
    done: bool,
}

/// How long a sound file plays, in milliseconds: a WAV's samples over its
/// rate; an OGG's last page's granule position over the rate its
/// identification header gives.
pub(crate) fn audio_ms(path: &str, bytes: &[u8]) -> Option<f32> {
    if path.to_ascii_lowercase().ends_with(".wav") {
        let pcm = cellview::sound::read_wav(bytes).ok()?;
        let frames = pcm.samples.len() as f32 / f32::from(pcm.channels.max(1));
        return Some(frames / pcm.rate.max(1) as f32 * 1000.0);
    }
    let id = bytes.windows(7).position(|w| w == b"\x01vorbis")?;
    let rate = u32::from_le_bytes(bytes.get(id + 12..id + 16)?.try_into().ok()?);
    let last = bytes.windows(4).rposition(|w| w == b"OggS")?;
    let granule = u64::from_le_bytes(bytes.get(last + 6..last + 14)?.try_into().ok()?);
    (rate > 0).then(|| granule as f32 / rate as f32 * 1000.0)
}

/// The audio a note plays (`00796fd0` case 0x18): a sound note (kind 0)
/// its sound (`SNAM`, `005e9100`); a voice note (3) the line its speaker
/// (`SNAM`, an NPC) says in its topic (`TNAM`, `005e9240` / `005e92a0`:
/// the first the speaker can say, `world::dialogue::pick`), each response's
/// voice file of the speaker's voice type.
fn note_pieces(
    game: &cellview::Game,
    state: &world::scripting::GameState,
    note: FormId,
) -> Vec<(Vec<u8>, bool, f32)> {
    let order = &game.order;
    let Some((rr, record)) = order
        .get(note)
        .and_then(|r| r.record().ok().map(|rec| (r, rec)))
    else {
        return Vec::new();
    };
    let form = |sig: &[u8; 4]| {
        record
            .get(esm::FourCC::new(sig))
            .filter(|s| s.data.len() >= 4)
            .map(|s| {
                rr.plugin.to_global(FormId(u32::from_le_bytes([
                    s.data[0], s.data[1], s.data[2], s.data[3],
                ])))
            })
    };
    let kind = record
        .get(esm::sig::DATA)
        .and_then(|s| s.data.first().copied())
        .unwrap_or(1);
    let mut out = Vec::new();
    let mut push = |path: String, bytes: Vec<u8>| {
        if let Some(ms) = audio_ms(&path, &bytes) {
            let ogg = path.to_ascii_lowercase().ends_with(".ogg");
            out.push((bytes, ogg, ms));
        }
    };
    match kind {
        0 => {
            if let Some(s) = form(b"SNAM").and_then(|s| world::sound::Sound::load(order, s)) {
                if let Some((path, bytes)) = game.sound_file(&s, state.dice) {
                    push(path, bytes);
                }
            }
        }
        3 => {
            let (Some(topic), Some(speaker)) = (form(b"TNAM"), form(b"SNAM")) else {
                return out;
            };
            let Some(who) = world::dialogue::Speaker::load(order, speaker, speaker) else {
                return out;
            };
            let Some(voice) = who.voice else {
                return out;
            };
            if let Some(info) = world::dialogue::pick(order, topic, &who, state) {
                for response in &info.responses {
                    let Some(path) = world::dialogue::voice_path(order, &info, response, voice)
                    else {
                        continue;
                    };
                    if let Some(bytes) = game.assets.read(&path).ok().flatten() {
                        push(path, bytes);
                    }
                }
            }
        }
        _ => {}
    }
    out
}

/// Starts a piece of a note's audio playing.
fn play_piece(
    commands: &mut Commands,
    wavs: &mut Assets<PcmSound>,
    bytes: &[u8],
    ogg: bool,
) -> Option<Entity> {
    // Ogg pieces decode through the viewer's own sample sources, as every
    // other sound does (`crate::sounds::read_sound`).
    let pcm = if ogg {
        crate::sounds::decode_ogg(bytes).ok()?
    } else {
        cellview::sound::read_wav(bytes).ok()?
    };
    let handle = wavs.add(crate::sounds::pcm_sound(pcm));
    Some(
        commands
            .spawn((AudioPlayer(handle), PlaybackSettings::DESPAWN))
            .id(),
    )
}

/// A note's audio, every frame the Pip-Boy is up (`0079a660` runs with the
/// map menu): when the piece playing has ended the next starts 500 ms
/// later; after the last it's done. What the menu shows of it (reported
/// done once, then let go).
fn advance_note(
    commands: &mut Commands,
    pipboy: &mut Pipboy,
    wavs: &mut Assets<PcmSound>,
    now: f32,
) -> Option<ui::pipboy::NoteAudio> {
    let p = pipboy.note.as_mut()?;
    if p.done {
        pipboy.note = None;
        return None;
    }
    let length = p.pieces.get(p.index).map_or(0.0, |x| x.2);
    if p.gap_from.is_none() && (now - p.piece_started) * 1000.0 >= length {
        p.entity = None;
        if p.index + 1 >= p.pieces.len() {
            p.done = true;
        } else {
            p.gap_from = Some(now);
        }
    }
    if let Some(from) = p.gap_from {
        if (now - from) * 1000.0 > 500.0 {
            p.index += 1;
            let (bytes, ogg, _) = &p.pieces[p.index];
            p.entity = play_piece(commands, wavs, bytes, *ogg);
            p.piece_started = now;
            p.gap_from = None;
        }
    }
    Some(ui::pipboy::NoteAudio {
        note: p.note.0,
        elapsed_ms: (now - p.started) * 1000.0,
        total_ms: p.total_ms,
        done: p.done,
    })
}

/// Uses hot key `n` (`00701e80`, `HotKeysWheel::UseHotkeyItem` (Xbox
/// PDB)): the item on it, while carried, is taken off when equipped
/// (`0088c790`), else equipped (`0088c650`; aid and the like are used). (The
/// game's refusals during an attack and its check against the hand's
/// weapon, `0058db10`, aren't made here.)
fn use_hotkey(order: &esm::LoadOrder, state: &mut world::scripting::GameState, n: usize) {
    let Some(item) = state.hotkeys.get(n).copied().flatten() else {
        return;
    };
    if state.item_count(order, PLAYER_REF, item) <= 0 {
        return;
    }
    let kind = order.get(item).map(|r| *r.entry.header.kind.as_bytes());
    if state.is_equipped(PLAYER_REF, item) {
        state.unequip(PLAYER_REF, item);
        println!("Hot key {}: took off {item}.", n + 1);
    } else if matches!(kind, Some(k) if &k == b"WEAP" || &k == b"ARMO") {
        state.equip(order, PLAYER_REF, item);
        println!("Hot key {}: equipped {item}.", n + 1);
    } else if let Some(done) = world::items::use_item(order, state, PLAYER_REF, item) {
        println!("Hot key {}: {done}", n + 1);
    }
}

/// A text game setting as the Pip-Boy's menus read it (the plugins' `GMST`
/// or the exe's own default); empty when there's none.
fn setting_text(pipboy: &Pipboy, name: &str) -> String {
    pipboy
        .built
        .as_ref()
        .and_then(|b| b.ui.setting_text(name))
        .unwrap_or_default()
}

/// Drops `count` of an item (`00780c50` → the player's slot 0x3cc,
/// `world::more_functions::placed::drop_item`), with the item's put-down
/// sound (a guess at what that slot plays).
fn drop_items(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    requests: &mut SoundRequests,
    collision: &physics::Collider,
    item: FormId,
    count: i32,
    heading: f32,
) {
    if let Some(id) = world::more_functions::placed::drop_item(order, state, item, count, heading) {
        rest_on_ground(order, state, collision, id);
        println!("Dropped {count} of {item}.");
        if let Some(s) = world::sound::item_sound(order, item, false) {
            requests.0.push(s);
        }
    }
}

/// How far above the drop point the ground is looked for, and how far
/// below (game units; the game's own drop casts the item's shape,
/// `009614b0`, not traced into this).
const GROUND_ABOVE: f32 = 64.0;
const GROUND_BELOW: f32 = 1024.0;

/// A dropped item set down on what's below it: the place's static
/// collision under the drop point (a ray straight down), the item's lowest
/// point (its `OBND`) on it. Physics takes it on from there where the
/// physics is (another branch's).
fn rest_on_ground(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    collision: &physics::Collider,
    id: FormId,
) {
    let Some(m) = state.more.placed.refs.get(&id).copied() else {
        return;
    };
    let lowest = order
        .get(m.base)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            let s = r.get(esm::FourCC::new(b"OBND"))?;
            (s.data.len() >= 6).then(|| f32::from(i16::from_le_bytes([s.data[4], s.data[5]])))
        })
        .unwrap_or(0.0);
    let from = [m.position[0], m.position[1], m.position[2] + GROUND_ABOVE];
    if let Some((d, _)) = collision.raycast(from, [0.0, 0.0, -1.0], GROUND_ABOVE + GROUND_BELOW) {
        if let Some(r) = state.more.placed.refs.get_mut(&id) {
            r.position[2] = from[2] - d - lowest;
        }
    }
}

/// Puts it up; false when the menus can't be read.
fn open(
    pipboy: &mut Pipboy,
    menus: &mut Menus,
    player: &mut Player,
    now: f32,
    at_once: bool,
) -> bool {
    if pipboy.failed {
        return false;
    }
    pipboy.open = true;
    pipboy.since = Some(now);
    pipboy.at_once = at_once;
    pipboy.knobs_fresh = true;
    menus.pipboy = true;
    player.ready = false;
    if let Some(fx) = pipboy.effects.as_mut() {
        fx.open(if at_once { -1.0e6 } else { now * 1000.0 });
    }
    true
}

/// Puts it away (the arm goes down, then hides).
fn close(
    commands: &mut Commands,
    pipboy: &mut Pipboy,
    menus: &mut Menus,
    player: &mut Player,
    conversation: &crate::dialogue::Conversation,
    now: f32,
) {
    pipboy.open = false;
    pipboy.since = Some(now);
    pipboy.at_once = false;
    menus.pipboy = false;
    // The stats menu closes with it, and its healing mode.
    if let Some(b) = pipboy.built.as_mut() {
        b.pipboy.stats.healing = false;
    }
    if !menus.others_open() && conversation.0.is_none() {
        player.ready = true;
    }
    if let Some(e) = pipboy.hum.take() {
        if let Ok(mut e) = commands.get_entity(e) {
            e.despawn();
        }
    }
}

/// Where the player is, for the DATA menu: the place's name (`00578870`:
/// the cell's own name; outdoors in a cell without one the game names the
/// place from the worldspace by the player's position, `TESWorldSpace`
/// slot 0x138, not traced: left empty), the world map of the worldspace
/// the player last walked in (`MapMarkers`; none before going outdoors:
/// which map the game shows then isn't traced) with the player on it when
/// outdoors there.
fn whereabouts(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    markers: &crate::map::MapMarkers,
    heading: f32,
    quests: &mut QuestPoints,
) -> ui::pipboy::gather::Whereabouts {
    // The quest markers' places, worked out again when the active quest,
    // its objectives, the player's cell or what's enabled change.
    let key = (
        state.active_quest,
        state.player_cell,
        markers.world,
        state.objectives.len(),
        state.objectives.values().filter(|&&d| d).count(),
        state.disabled.len(),
    );
    if quests.key != Some(key) {
        quests.points = markers.world.map_or(Vec::new(), |w| {
            ui::pipboy::gather::quest_points(order, state, &mut quests.graph, w)
        });
        quests.key = Some(key);
    }
    let location = state
        .player_cell
        .and_then(|id| order.get(id))
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default();
    let world = markers.world;
    let outdoors = state.player_world.is_some() && state.player_world == world;
    ui::pipboy::gather::Whereabouts {
        location,
        world,
        markers: markers.list.clone(),
        player: state
            .player_position
            .filter(|_| outdoors)
            .map(|p| (p, heading)),
        quest: quests.points.clone(),
    }
}

/// What the quest markers' places hang on: the active quest, the player's
/// cell, the map's worldspace, the objectives shown and done, what's
/// enabled.
type QuestPointsKey = (
    Option<FormId>,
    Option<FormId>,
    Option<FormId>,
    usize,
    usize,
    usize,
);

/// The world map's quest markers' places, kept until what they hang on
/// changes; and the door search's doors.
#[derive(Default)]
struct QuestPoints {
    key: Option<QuestPointsKey>,
    points: Vec<[f32; 3]>,
    graph: world::quest_targets::DoorGraph,
}
/// What drawing the picture needs.
/// (Pictures, meshes and the arm's materials come through the `Spawner`.)
#[derive(bevy::ecs::system::SystemParam)]
pub struct Drawing<'w> {
    tiles: ResMut<'w, Assets<TileMaterial>>,
    screens: ResMut<'w, Assets<ScreenMaterial>>,
    /// DATA › Local Map's line and pictures (`local_map`).
    local_map: Res<'w, crate::local_map::LocalMap>,
    /// The player's third-person animations, which the ITEMS card's
    /// damage a second reads the rate of fire from
    /// (`world::dps::shots_per_second`).
    library: Option<ResMut<'w, crate::anim_library::AnimLibrary>>,
}

/// Every frame while it's up: the menus filled from the game's state, the
/// picture's pieces, the screen effect's values, the arm.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update_pipboy(
    mut commands: Commands,
    mut pipboy: ResMut<Pipboy>,
    start: Option<ResMut<StartPipboy>>,
    around: Around,
    drawing: Drawing,
    mut spawner: Spawner,
    mut cameras: Query<&mut Camera, With<PipboyCamera>>,
    views: Query<(Entity, &Transform, &Projection), With<FlyCamera>>,
    mut transforms: Query<&mut Transform, (Without<FlyCamera>, Without<PipboyCamera>)>,
    mut visibility: Query<&mut Visibility>,
    piece_materials: Query<&MeshMaterial3d<GameLitMaterial>>,
    mesh_handles: Query<&Mesh3d>,
    first_person: Query<
        &Projection,
        (
            With<crate::viewmodel::FirstPersonCamera>,
            Without<FlyCamera>,
        ),
    >,
    pads: Query<(), With<Gamepad>>,
    pretend_pad: Res<PretendPad>,
) {
    let Around {
        time,
        game,
        mut state,
        mut menus,
        mut player,
        mut wavs,
        markers,
        mut messages,
        ..
    } = around;
    let Drawing {
        mut tiles,
        mut screens,
        local_map,
        mut library,
    } = drawing;
    let order = &game.0.order;
    let now = time.elapsed_secs();
    let pipboy = &mut *pipboy;

    // `--pipboy`: up at once once the place is.
    if let Some(mut start) = start {
        if player.ready && !pipboy.open {
            if let Some(which) = start.0.take() {
                if open(pipboy, &mut menus, &mut player, now, true) {
                    pipboy.pending = Some(parse_start(&which));
                }
            }
        }
    }

    // Where the arm is in its raising animation (none: down and away).
    let raise_at = pipboy.raise_at(now);
    let shown = pipboy.arm_shown(now);
    for mut camera in &mut cameras {
        if camera.is_active != shown {
            camera.is_active = shown;
        }
    }
    if let Some(arm) = pipboy.arm.as_ref() {
        let ready = arm.screen.is_none() || arm.waited >= 3;
        if ready {
            for old in pipboy.retired.drain(..) {
                if let Ok(mut e) = commands.get_entity(old) {
                    e.despawn();
                }
            }
        }
        if let Ok(mut v) = visibility.get_mut(arm.holder) {
            let want = if shown && ready {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            if *v != want {
                *v = want;
            }
        }
        // The light's cone (`PipboyLightEffect`) shown with the light on
        // (`007fa310`); of the three lamps over STATS, ITEMS and DATA only
        // the shown menu's lit (`007f9070` finds them, `007fa010` hides
        // all three and shows one).
        let section = pipboy.built.as_ref().map(|b| b.pipboy.section);
        let lamps = arm.glows.iter().enumerate().map(|(i, &e)| {
            let on = section.is_some_and(|s| s as usize == i);
            (e, on)
        });
        let cones = arm.light_effect.iter().map(|&e| (e, pipboy.light));
        for (e, on) in cones.chain(lamps) {
            if let Ok(mut v) = visibility.get_mut(e) {
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
    }
    if !shown {
        state.0.healing_part = None;
        // Down after "Do you want to travel to ...?" said Yes: the trip
        // (`00798a00` hands the marker to the player's travel, `0093be30`).
        if let Some(reference) = pipboy.travel_after_close.take() {
            if let Some(m) = markers.list.iter().find(|m| m.reference.0 == reference) {
                // A refusal's picture (`0093d660`), looked up before the
                // trip as `travel` itself refuses.
                let icon = world::map::travel_refusal(order, &state.0).map(|(_, icon)| icon);
                match world::map::travel(order, &mut state.0, m) {
                    Ok(_) => println!("Fast travel to {}.", m.name),
                    Err(why) => {
                        println!("{why}");
                        if messages.on {
                            messages
                                .queue
                                .push(crate::hud::HudMessage::with_icon(why, icon));
                        }
                    }
                }
            }
        }
        return;
    }

    // The menus, read once.
    if pipboy.built.is_none() && !pipboy.failed {
        match build(&game.0) {
            Ok(b) => pipboy.built = Some(Box::new(b)),
            Err(e) => {
                println!("The Pip-Boy's menus can't be shown: {e}");
                pipboy.failed = true;
                pipboy.open = false;
                menus.pipboy = false;
                player.ready = true;
                return;
            }
        }
    }
    // A note's audio going on.
    let note_audio = advance_note(&mut commands, pipboy, &mut wavs, now);

    let Some(b) = pipboy.built.as_mut() else {
        return;
    };
    // A pad connected shows its buttons (`00719630`; Bevy's gamepads stand
    // for XInput's pad 0, `bDisable360Controller` isn't read).
    let pad = pretend_pad.0 || !pads.is_empty();
    if b.pad != pad {
        ui::game::set_pad(&mut b.ui, pad);
        b.pad = pad;
    }

    // The hum while it's up.
    if pipboy.open && pipboy.hum.is_none() {
        if let Some(s) = order
            .form_by_editor_id("UIPipBoyHumLP")
            .and_then(|id| world::sound::Sound::load(order, id))
        {
            pipboy.hum =
                crate::sounds::play(&mut commands, &game.0, &mut wavs, &s, state.0.dice, true);
        }
    }

    // Filled from the game's state.
    let view = views.single().ok();
    let heading = view.map_or(0.0, |(_, t, _)| {
        let f = t.forward().as_vec3();
        f.x.atan2(-f.z).to_degrees()
    });
    let at = whereabouts(order, &state.0, &markers, heading, &mut pipboy.quest_points);
    let mut input = match library
        .as_deref_mut()
        .and_then(|l| l.player_library(&game.0))
    {
        Some(mut lib) => ui::pipboy::gather::gather_with(order, &state.0, &at, Some(&mut lib)),
        None => ui::pipboy::gather::gather(order, &state.0, &at),
    };
    input.local_map = local_map.line.clone();
    input.note_audio = note_audio;
    b.pipboy.fill(&mut b.ui, &input);
    b.pipboy.frame(&mut b.ui, f64::from(now));
    // The knobs: set up on coming up; the tab knob to the shown menu's
    // page or tab (`007dfc90`, `00782470`, `0079c340` → `007fa0f0`); all
    // moved on (`007f8320`).
    let knob_settings = *pipboy
        .knob_settings
        .get_or_insert_with(|| KnobSettings::load(&game.0));
    let now_ms = now * 1000.0;
    if std::mem::take(&mut pipboy.knobs_fresh) {
        pipboy.knobs.init(&knob_settings, input.rads, now_ms);
    }
    let tab = match b.pipboy.section {
        Section::Stats => b.pipboy.stats.page,
        Section::Items => b.pipboy.items.tab,
        Section::Data => b.pipboy.data.tab,
    };
    pipboy.knobs.tab(tab, &knob_settings);
    pipboy.knobs.update(now_ms);
    b.pipboy.update(&mut b.ui, f64::from(now));
    if let Some((section, page)) = pipboy.pending.take() {
        b.pipboy.show(&mut b.ui, section);
        if let Some(p) = page {
            match section {
                Section::Stats => b.pipboy.stats.show_page(&mut b.ui, p, &input),
                Section::Items => b.pipboy.items.show_tab(&mut b.ui, p, &input),
                Section::Data => b.pipboy.data.show_tab(&mut b.ui, p, &input),
            }
        }
        b.ui.refresh();
    }
    pipboy.input = Some(input);

    // The picture's pieces, when something changed.
    let compressed = spawner
        .device
        .as_ref()
        .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
    let menu = b.pipboy.menu();
    let items = {
        let mut files = Files {
            game: &game.0,
            sizes: &mut b.sizes,
            atlases: &mut b.atlases,
        };
        ui::draw::update_file_sizes(&mut b.ui, menu, &mut files);
        let mut items = ui::draw_list(&mut b.ui, menu, &mut files, &|_| None);
        // DATA › Local Map's pictures under its markers (`local_map`).
        if let Some(input) = pipboy.input.as_ref() {
            let extra = b.pipboy.data.local_map_draws(&mut b.ui, input);
            if let Some(first) = extra.first() {
                // The map's tile is a hot rectangle (a solid picture, its
                // `alpha` 255 in `map_menu.xml`); its pictures stand in for
                // that square, so unexplored ground is see-through to the
                // Pip-Boy's background [guess: what happens to the tile's
                // own square once `0079ffb0` adds the pictures isn't
                // traced].
                let map_tile = first.tile;
                items
                    .retain(|it| it.tile != map_tile || !matches!(it.kind, DrawKind::Image { .. }));
                let at = items
                    .iter()
                    .position(|it| it.depth > first.depth)
                    .unwrap_or(items.len());
                items.splice(at..at, extra);
            }
        }
        items
    };
    if items != b.last {
        for (e, mesh, material) in b.drawn.drain(..) {
            commands.entity(e).despawn();
            spawner.meshes.remove(&mesh);
            tiles.remove(&material);
        }
        let Some(white) = pipboy.white.clone() else {
            return;
        };
        for (i, item) in items.iter().enumerate() {
            let tint = Vec4::from_array(item.color);
            let mut pieces: Vec<(Handle<Image>, Vec<Quad>)> = Vec::new();
            match &item.kind {
                DrawKind::Image {
                    texture,
                    rect,
                    uv,
                    repeat_u,
                    ..
                } => {
                    let repeat = (*repeat_u, false);
                    let key = (texture.clone(), repeat.0, repeat.1);
                    let handle = b
                        .images
                        .entry(key)
                        .or_insert_with(|| {
                            crate::hud::upload_picture(
                                &mut spawner.images,
                                &game.0,
                                texture,
                                repeat,
                                compressed,
                            )
                        })
                        .clone();
                    if let Some(handle) = handle {
                        let corners = [
                            [uv[0], uv[1]],
                            [uv[2], uv[1]],
                            [uv[0], uv[3]],
                            [uv[2], uv[3]],
                        ];
                        pieces.push((handle, vec![(*rect, corners)]));
                    }
                }
                DrawKind::Model {
                    texture,
                    triangles,
                    alpha,
                    ..
                } => {
                    // A local map tile's picture (made by `local_map`), or a
                    // file's.
                    let handle = match texture {
                        Some(name) if name.starts_with("nvrs:localmap:") => {
                            local_map.images.get(name).cloned()
                        }
                        Some(path) => b
                            .images
                            .entry((path.clone(), false, false))
                            .or_insert_with(|| {
                                crate::hud::upload_picture(
                                    &mut spawner.images,
                                    &game.0,
                                    path,
                                    (false, false),
                                    compressed,
                                )
                            })
                            .clone(),
                        None => Some(white.clone()),
                    };
                    let Some(texture) = handle else {
                        continue;
                    };
                    let mesh = spawner
                        .meshes
                        .add(crate::hud::triangles_mesh(triangles, alpha, 1.0, PICTURE));
                    let material = tiles.add(TileMaterial::plain(tint, texture, white.clone()));
                    let entity = commands
                        .spawn((
                            Mesh2d(mesh.clone()),
                            MeshMaterial2d(material.clone()),
                            Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                            RenderLayers::layer(MENU_LAYER),
                        ))
                        .id();
                    b.drawn.push((entity, mesh, material));
                    continue;
                }
                DrawKind::Text { font, glyphs } => {
                    let Some(f) = b.ui.fonts.get(font - 1).cloned().flatten() else {
                        continue;
                    };
                    let paths = ui::draw::font_textures(&f);
                    let mut by_picture: HashMap<u32, Vec<Quad>> = HashMap::new();
                    for (rect, uv, picture) in glyphs {
                        by_picture.entry(*picture).or_default().push((*rect, *uv));
                    }
                    let mut pictures: Vec<_> = by_picture.into_iter().collect();
                    pictures.sort_by_key(|(p, _)| *p);
                    for (picture, quads) in pictures {
                        let Some(path) = paths.get(picture as usize) else {
                            continue;
                        };
                        let handle = b
                            .font_images
                            .entry((*font, picture))
                            .or_insert_with(|| {
                                crate::hud::upload_font_picture(&mut spawner.images, &game.0, path)
                            })
                            .clone();
                        if let Some(texture) = handle {
                            pieces.push((texture, quads));
                        }
                    }
                }
            }
            for (texture, quads) in pieces {
                // One menu unit a pixel.
                let mesh = spawner
                    .meshes
                    .add(crate::hud::quads_mesh(&quads, 1.0, PICTURE));
                let material = tiles.add(TileMaterial::plain(tint, texture, white.clone()));
                let entity = commands
                    .spawn((
                        Mesh2d(mesh.clone()),
                        MeshMaterial2d(material.clone()),
                        Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                        RenderLayers::layer(MENU_LAYER),
                    ))
                    .id();
                b.drawn.push((entity, mesh, material));
            }
        }
        b.last = items;
    }

    // The screen effect's values this frame.
    if let (Some(fx), Some(handle)) = (pipboy.effects.as_mut(), pipboy.material.as_ref()) {
        let p = fx.params(now * 1000.0);
        let tint = pipboy_colour(&game.0);
        if let Some(m) = screens.get_mut(handle) {
            m.u = ScreenUniform {
                params: Vec4::new(p.blur_intensity, p.scroll, 1.0, p.scanline_frequency),
                distort: p.distort.map_or(Vec4::ZERO, |(v, progress, h)| {
                    Vec4::new(v, progress, h, 0.0)
                }),
                tint,
                offsets: Vec4::new(
                    p.blur_radius / PICTURE.x as f32,
                    p.blur_radius / PICTURE.y as f32,
                    0.0,
                    0.0,
                ),
            };
        }
    }

    // The arm.
    let Some((camera, camera_transform, world_projection)) = view else {
        return;
    };
    // The arm's pieces are drawn by the first-person camera
    // (`viewmodel::FIRST_PERSON_LAYER`), so its field of view is the one
    // the arm's scale makes up for.
    let projection = first_person.single().unwrap_or(world_projection);
    let Some(lighting) = spawner.place_lighting.get() else {
        return;
    };
    let lighting_changes = spawner.place_lighting.changes();
    let female = state.0.player_female.unwrap_or(false);
    let worn: Vec<FormId> = state
        .0
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
    // The weapon in hand, as the ordinary first-person view holds it (with
    // its mods, `world::weapon_mods::player_model`).
    let weapon = world::combat::weapon_in_hand(order, &state.0, PLAYER_REF).and_then(|w| {
        let flags = world::weapon_mods::flags(&state.0, PLAYER_REF, w.form_id);
        let model = world::weapon_mods::player_model(order, w.form_id, flags)?;
        Some((w.form_id, model, w.animation))
    });
    let weapon_id = weapon.as_ref().map(|(id, model, _)| (*id, model.clone()));
    let rebuild = pipboy.arm.as_ref().is_none_or(|a| {
        a.worn != worn
            || a.female != female
            || a.lighting != lighting_changes
            || a.weapon != weapon_id
    });
    if rebuild {
        if let Some(old) = pipboy.arm.take() {
            pipboy.retired.push(old.holder);
        }
        pipboy.arm = build_arm(
            &mut commands,
            &game.0,
            &mut spawner,
            camera,
            lighting,
            lighting_changes,
            &worn,
            female,
            weapon,
        );
    }
    let Some(arm) = pipboy.arm.as_mut() else {
        return;
    };
    // The screen piece takes the screen's picture (once its material is
    // there).
    if let (Some(piece), Some(screen)) = (arm.screen, pipboy.screen.as_ref()) {
        if let Ok(handle) = piece_materials.get(piece) {
            if let Some(m) = spawner.lit_materials.get_mut(&handle.0) {
                m.base.base_color_texture = Some(screen.clone());
            }
            arm.screen = None;
        }
    }
    arm.waited = arm.waited.saturating_add(1);
    pose_arm(arm, raise_at, camera_transform, projection, &mut transforms);
    draw_knobs(arm, &pipboy.knobs, &mut spawner.meshes, &mesh_handles);
}

/// `--pipboy`'s value: `stats`, `items` or `data`, with `:PAGE` (the stats
/// page or the tab, from 0).
fn parse_start(which: &str) -> (Section, Option<usize>) {
    let (section, page) = match which.split_once(':') {
        Some((s, p)) => (s, p.trim().parse::<usize>().ok()),
        None => (which, None),
    };
    let section = match section.trim().to_ascii_lowercase().as_str() {
        "items" => Section::Items,
        "data" => Section::Data,
        _ => Section::Stats,
    };
    (section, page)
}

/// `uPipboyColor` (`[Interface]`; 4290134783 in this install's
/// `FalloutPrefs.ini`), 0 to 1.
fn pipboy_colour(game: &cellview::Game) -> Vec4 {
    colour_of(
        game.settings
            .get("Interface", "uPipboyColor")
            .and_then(|v| v.trim().parse::<u32>().ok())
            .unwrap_or(0xFFB6_42FF),
    )
}

/// A colour setting's value as red, green, blue, alpha bytes from the top
/// (0xFFB642FF: 255, 182, 66); alpha taken as 1.
fn colour_of(v: u32) -> Vec4 {
    let byte = |shift: u32| ((v >> shift) & 0xFF) as f32 / 255.0;
    Vec4::new(byte(24), byte(16), byte(8), 1.0)
}

/// The first-person model with the Pip-Boy: the first-person look (the
/// clothes worn, the hands, the weapon in hand, world::actor::
/// first_person_look) with the Pip-Boy and its glove (first-person version)
/// worn, as the player always wears them.
#[allow(clippy::too_many_arguments)]
fn build_arm(
    commands: &mut Commands,
    game: &cellview::Game,
    spawner: &mut Spawner,
    camera: Entity,
    lighting: crate::lighting::GameLighting,
    lighting_changes: u64,
    worn: &[FormId],
    female: bool,
    weapon: Option<(FormId, String, u32)>,
) -> Option<Arm> {
    let order = &game.order;
    let mut wearing: Vec<FormId> = worn.to_vec();
    let glove = order.form_by_editor_id(GLOVE_ITEM);
    for item in [glove, order.form_by_editor_id(PIPBOY_ITEM)]
        .into_iter()
        .flatten()
    {
        if !wearing.contains(&item) {
            wearing.push(item);
        }
    }
    let held = weapon
        .as_ref()
        .map(|(_, model, animation)| (model.clone(), *animation));
    let mut look = world::actor::first_person_look(order, female, &wearing, held)?;
    // The Pip-Boy's model is made in its bone's own axes (the forearm
    // along x from 4 to 17 units), the bone its top node's `Prn` names
    // (`Bip01 L ForeTwist`): held there as a weapon is at `Weapon`.
    if let Some(pipboy) = order
        .form_by_editor_id(PIPBOY_ITEM)
        .and_then(|p| world::actor::Armor::load(order, p))
    {
        let models = [pipboy.male.clone(), pipboy.female.clone()];
        for part in &mut look.parts {
            if models.iter().flatten().any(|m| *m == part.model) {
                part.bone = attach_bone(game, &part.model);
            }
        }
    }
    // The glove's first-person version (`LeftHandPipboyGlove1st.nif`).
    if let Some(glove) = glove.and_then(|g| world::actor::Armor::load(order, g)) {
        let models = [glove.male.clone(), glove.female.clone()];
        for part in &mut look.parts {
            if models.iter().flatten().any(|m| *m == part.model)
                && !part.model.to_ascii_lowercase().contains("1st.")
            {
                part.model = world::actor::first_person_hand(&part.model);
            }
        }
    }
    let scene = game.actor_scene(&look);
    let holder = commands
        .spawn((Transform::IDENTITY, Visibility::Hidden, ChildOf(camera)))
        .id();
    let (root, joints, pieces) = spawner.spawn_lone_actor(&scene, lighting, holder)?;
    // The pieces in the order spawned (the actor's draws with a rig), told
    // apart by their texture: `pipboyscreen:0` is the unlit one showing
    // `Pipboy3000\Screen.dds` (the lit `ScreenLit:8` lies just behind
    // it), `PipboyLightEffect:0` the unlit `effects\FXWHITE.dds` cone.
    let kinds: Vec<(String, bool)> = scene
        .draws
        .iter()
        .map(|d| &scene.meshes[d.mesh])
        .filter(|m| m.rig.is_some())
        .map(|m| {
            let texture = m
                .material
                .texture
                .and_then(|i| scene.textures.get(i))
                .map(|t| t.path.to_ascii_lowercase())
                .unwrap_or_default();
            (texture, m.material.unlit)
        })
        .collect();
    // What the mouse picks, by the shapes' names (`007f8ba0` finds
    // `pipboyscreen:0`, `007f9070` `PipBoyButton01` .. `03`, whose shapes
    // are `PipBoyButton0N:0` and `:1`).
    let mut screen_pick = None;
    let mut button_picks: [Vec<PickMesh>; 3] = Default::default();
    for data in scene
        .draws
        .iter()
        .map(|d| &scene.meshes[d.mesh])
        .filter(|m| m.rig.is_some())
    {
        let name = data.shape_name.to_ascii_lowercase();
        if name == "pipboyscreen" || name.starts_with("pipboyscreen:") {
            screen_pick = PickMesh::new(data, &joints);
        }
        for (i, picks) in button_picks.iter_mut().enumerate() {
            let node = format!("pipboybutton0{}", i + 1);
            if name == node || name.starts_with(&format!("{node}:")) {
                picks.extend(PickMesh::new(data, &joints));
            }
        }
    }
    // The knobs' pieces (`007f99d0`), kept with their vertices to be turned.
    let mut shapes = Vec::new();
    if let Some(pipboy) = order
        .form_by_editor_id(PIPBOY_ITEM)
        .and_then(|p| world::actor::Armor::load(order, p))
    {
        for model in [pipboy.male.clone(), pipboy.female.clone()]
            .into_iter()
            .flatten()
        {
            if look.parts.iter().any(|p| p.model == model) {
                shapes.extend(knob_shapes(game, &model));
            }
        }
    }
    let mut knob_pieces = Vec::new();
    for (data, &entity) in scene
        .draws
        .iter()
        .map(|d| &scene.meshes[d.mesh])
        .filter(|m| m.rig.is_some())
        .zip(&pieces)
    {
        let name = data.shape_name.to_ascii_lowercase();
        if let Some((_, knob, node, local)) = shapes.iter().find(|(n, ..)| *n == name) {
            knob_pieces.push(KnobPiece {
                knob: *knob,
                node: *node,
                local: *local,
                entity,
                positions: data.positions.clone(),
                normals: data.normals.clone(),
                tangents: data.tangents.clone(),
            });
        }
    }
    let mut screen = None;
    let mut light_effect = Vec::new();
    let mut glows = Vec::new();
    for ((texture, unlit), &piece) in kinds.iter().zip(&pieces) {
        if texture.ends_with("pipboy3000\\screen.dds") && *unlit {
            screen = Some(piece);
        } else if texture.ends_with("effects\\fxwhite.dds") && *unlit {
            light_effect.push(piece);
        } else if texture.ends_with("pipboybtnglow01.dds") {
            // `StatsGlow`, `ItemsGlow`, `DataGlow`, in the file's order.
            glows.push(piece);
        }
    }
    let skeleton = scene.actors.first()?.skeleton.clone();
    let find = |name: &str| {
        skeleton
            .bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case(name))
    };
    let looking = find("Bip01 Looking")?;
    let camera_bone = find("Camera1st")?;
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
    let raise = raise_animation(game, if female { RAISE_FEMALE } else { RAISE_MALE })
        .or_else(|| raise_animation(game, RAISE_MALE));
    // Said once (outdoors the arm is built again as squares load).
    static SAID: std::sync::Once = std::sync::Once::new();
    SAID.call_once(|| {
        println!(
            "  Pip-Boy: {}; {}",
            look.parts
                .iter()
                .map(|p| p.model.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            match (&raise, screen.is_some()) {
                (Some(r), true) => format!(
                    "raised by {} (held at {:.2} s of {:.2})",
                    r.sequence.name,
                    r.hit - r.sequence.start,
                    r.sequence.stop - r.sequence.start
                ),
                (None, _) => "no raising animation found".into(),
                (_, false) => "no screen found on the model".into(),
            }
        )
    });
    Some(Arm {
        worn: worn.to_vec(),
        weapon: weapon.map(|(id, model, _)| (id, model)),
        female,
        lighting: lighting_changes,
        waited: 0,
        holder,
        root,
        joints,
        skeleton,
        turned,
        looking,
        camera: camera_bone,
        raise,
        screen,
        light_effect,
        glows,
        screen_pick,
        button_picks,
        knob_pieces,
        knobs_drawn: [f32::NAN; 3],
    })
}

/// `NiMatrix3::MakeRotation` (`004168a0`): a rotation by `angle` about the
/// unit `axis`, as Gamebryo writes it (rows; the transpose of the usual
/// right-handed matrix).
// Translated from 004168a0 (decompiled, FalloutNV.exe 1.4.0.525)
fn make_rotation(angle: f32, [x, y, z]: [f32; 3]) -> [[f32; 3]; 3] {
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [
        [x * x * t + c, x * y * t + z * s, x * z * t - y * s],
        [x * y * t - z * s, y * y * t + c, y * z * t + x * s],
        [x * z * t + y * s, y * z * t - x * s, z * z * t + c],
    ]
}

/// The Pip-Boy's knobs and needle (`FOPipboyManager` (Xbox PDB), `+0x10c`
/// on: the angles now, the targets, the rates in radians a millisecond;
/// 0 the scroll knob, 1 the radiation needle, 2 the tab knob), set up as
/// `FOPipboyManager::InitKnobs` (`007f99d0`) does when the Pip-Boy comes
/// up and moved as `UpdateKnobs` (`007f8320`) moves them.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Knobs {
    now: [f32; 3],
    target: [f32; 3],
    rate: [f32; 3],
    last_ms: f32,
    /// The tab knob's five places (`+0x130` .. `+0x140`).
    tabs: [f32; 5],
}

/// The knobs' settings (`[Pipboy]` in the INI files; the exe's defaults).
#[derive(Debug, Clone, Copy, PartialEq)]
struct KnobSettings {
    tab_min: f32,
    tab_max: f32,
    tab_rate: f32,
    scroll_increment: f32,
    scroll_rate: f32,
}

impl KnobSettings {
    fn load(game: &cellview::Game) -> KnobSettings {
        let get = |k: &str, d: f32| {
            game.settings
                .get("Pipboy", k)
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(d)
        };
        KnobSettings {
            tab_min: get("fTabKnobMinPosition", -0.5),
            tab_max: get("fTabKnobMaxPosition", 1.5),
            tab_rate: get("fTabKnobMoveRate", 0.0075),
            scroll_increment: get("fScrollKnobIncrement", -0.1),
            scroll_rate: get("fScrollKnobRate", 0.0015),
        }
    }
}

impl Knobs {
    /// `007f99d0`: the tab knob's places from `fTabKnobMinPosition` to
    /// `MaxPosition` in four steps, the knob at the first; the needle at
    /// π/2 − rads ÷ 1000 × π (`007ddef0`: 1000), still.
    // Translated from 007f99d0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn init(&mut self, s: &KnobSettings, rads: f32, now_ms: f32) {
        let step = (s.tab_max - s.tab_min) / 4.0;
        for (i, p) in self.tabs.iter_mut().enumerate() {
            *p = s.tab_min + step * i as f32;
        }
        self.now[2] = self.tabs[0];
        self.target[2] = self.tabs[0];
        self.rate[2] = 0.0;
        let needle = std::f32::consts::FRAC_PI_2 - rads / 1000.0 * std::f32::consts::PI;
        self.now[1] = needle;
        self.target[1] = needle;
        self.rate[1] = 0.0;
        self.last_ms = now_ms;
    }

    /// `007f8610`: knob `i` turned by `by` at `rate`.
    fn turn(&mut self, i: usize, by: f32, rate: f32) {
        self.target[i] += by;
        self.rate[i] = rate;
    }

    /// `007fa0f0`: the tab knob to a tab's place, at `fTabKnobMoveRate`.
    fn tab(&mut self, tab: usize, s: &KnobSettings) {
        let to = self.tabs[tab.min(4)];
        if to != self.target[2] {
            self.target[2] = to;
            self.rate[2] = s.tab_rate;
        }
    }

    /// `007f8320`: each knob toward its target by its rate × the time
    /// since the last frame, not past it. Which knobs moved.
    // Translated from 007f8320 (decompiled, FalloutNV.exe 1.4.0.525)
    fn update(&mut self, now_ms: f32) -> [bool; 3] {
        let dt = now_ms - self.last_ms;
        let mut moved = [false; 3];
        for (i, m) in moved.iter_mut().enumerate() {
            if self.now[i] < self.target[i] {
                self.now[i] = (self.now[i] + self.rate[i] * dt).min(self.target[i]);
                *m = true;
            }
            if self.target[i] < self.now[i] {
                self.now[i] = (self.now[i] - self.rate[i] * dt).max(self.target[i]);
                *m = true;
            }
        }
        self.last_ms = now_ms;
        moved
    }
}

/// A knob's piece of the arm's model, kept to be turned: its knob (0
/// scroll, 1 needle, 2 tab), its node's own transform and the piece's
/// under it (from the model file), and the piece's vertices as built.
struct KnobPiece {
    knob: usize,
    node: nif::Transform,
    local: nif::Transform,
    entity: Entity,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    tangents: Option<Vec<[f32; 4]>>,
}

/// Turns the knobs' pieces to the knobs' angles where they changed: each
/// piece's vertices moved by [`knob_delta`].
fn draw_knobs(arm: &mut Arm, knobs: &Knobs, meshes: &mut Assets<Mesh>, handles: &Query<&Mesh3d>) {
    for k in 0..3 {
        if arm.knobs_drawn[k] == knobs.now[k] {
            continue;
        }
        let mut all = true;
        for p in arm.knob_pieces.iter().filter(|p| p.knob == k) {
            let Some(mesh) = handles
                .get(p.entity)
                .ok()
                .and_then(|h| meshes.get_mut(&h.0))
            else {
                all = false;
                continue;
            };
            let d = knob_delta(k, &p.node, &p.local, knobs.now[k]);
            let positions: Vec<[f32; 3]> = p.positions.iter().map(|&v| d.apply_point(v)).collect();
            let normals: Vec<[f32; 3]> = p.normals.iter().map(|&n| d.apply_direction(n)).collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            if let Some(tangents) = &p.tangents {
                let turned: Vec<[f32; 4]> = tangents
                    .iter()
                    .map(|t| {
                        let [x, y, z] = d.apply_direction([t[0], t[1], t[2]]);
                        [x, y, z, t[3]]
                    })
                    .collect();
                mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, turned);
            }
        }
        if all {
            arm.knobs_drawn[k] = knobs.now[k];
        }
    }
}

/// The knobs' nodes (`007f99d0` finds `ScrollKnob`, `RadNeedle` and
/// `TabKnob` in the first-person model) and the axis each turns about (the
/// scroll knob x, the others z).
const KNOB_NODES: [(&str, [f32; 3]); 3] = [
    ("scrollknob", [1.0, 0.0, 0.0]),
    ("radneedle", [0.0, 0.0, 1.0]),
    ("tabknob", [0.0, 0.0, 1.0]),
];

/// A model's knob pieces: for each shape under a knob's node, (shape
/// name, knob, the node's transform, the shape's).
fn knob_shapes(
    game: &cellview::Game,
    model: &str,
) -> Vec<(String, usize, nif::Transform, nif::Transform)> {
    let Some(nif) = game
        .assets
        .read(&assets::mesh_path(model))
        .ok()
        .flatten()
        .and_then(|b| nif::Nif::parse(b).ok())
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for i in 0..nif.blocks().len() {
        let Ok(nif::Block::Node(node)) = nif.block(i) else {
            continue;
        };
        let name = node.av.net.name.to_ascii_lowercase();
        let Some(knob) = KNOB_NODES.iter().position(|(n, _)| *n == name) else {
            continue;
        };
        for &child in &node.children {
            let Some(c) = usize::try_from(child)
                .ok()
                .filter(|&c| c < nif.blocks().len())
            else {
                continue;
            };
            if let Ok(nif::Block::Geometry(g)) = nif.block(c) {
                out.push((
                    g.av.net.name.to_ascii_lowercase(),
                    knob,
                    node.av.transform,
                    g.av.transform,
                ));
            }
        }
    }
    out
}

/// How a knob piece's vertices move with the knob at `angle`, in the
/// piece's own space: the node's rotation set to `make_rotation(angle,
/// axis)` (`006404b0`, the node's own rotation replaced); for the scroll
/// knob the node's first rotation kept under it, as `007f99d0` folds it
/// into the piece. (Whether the needle's and tab knob's turns start from
/// their files' poses that way isn't checked against the game.)
fn knob_delta(
    knob: usize,
    node: &nif::Transform,
    local: &nif::Transform,
    angle: f32,
) -> nif::Transform {
    let axis = KNOB_NODES[knob].1;
    let turned = nif::Transform {
        rotation: make_rotation(angle, axis),
        translation: node.translation,
        scale: node.scale,
    };
    let under = if knob == 0 {
        nif::Transform {
            rotation: node.rotation,
            translation: [0.0; 3],
            scale: 1.0,
        }
        .then_child(local)
    } else {
        *local
    };
    node.then_child(local)
        .inverse()
        .then_child(&turned.then_child(&under))
}

/// The bone a model's top node names in its `Prn` (`nif::Nif::attach_bone`).
fn attach_bone(game: &cellview::Game, model: &str) -> Option<String> {
    let bytes = game.assets.read(&assets::mesh_path(model)).ok()??;
    nif::Nif::parse(bytes).ok()?.attach_bone()
}

/// The raising animation and the moment it holds at while the Pip-Boy is
/// up: its `Hit` text key (0.33 s of 0.73; the arm is highest there),
/// else halfway (a guess at how the game uses the key: it plays to it on
/// opening and on from it to the end on closing).
fn raise_animation(game: &cellview::Game, path: &str) -> Option<Raise> {
    let bytes = game.assets.read(&assets::mesh_path(path)).ok()??;
    let nif = nif::Nif::parse(bytes).ok()?;
    let sequence = nif.sequences().ok()?.into_iter().next()?;
    let hit = nif
        .text_keys()
        .ok()
        .and_then(|keys| {
            keys.into_iter()
                .find(|(_, k)| k.eq_ignore_ascii_case("hit"))
                .map(|(t, _)| t)
        })
        .unwrap_or((sequence.start + sequence.stop) / 2.0)
        .clamp(sequence.start, sequence.stop);
    Some(Raise { sequence, hit })
}

/// The raising animation (see [`raise_animation`]).
struct Raise {
    sequence: nif::Sequence,
    hit: f32,
}

/// Where in the raising animation the arm is: going up from the start to
/// `Hit` over that many seconds after opening, held there, and on from
/// `Hit` to the end after closing. `since` is how long ago it was opened
/// or closed (`None`: never).
fn raise_time(start: f32, hit: f32, stop: f32, open: bool, since: Option<f32>) -> Option<f32> {
    match (open, since) {
        (true, Some(t)) => Some((start + t).min(hit)),
        (true, None) => Some(hit),
        (false, Some(t)) if hit + t < stop => Some(hit + t),
        _ => None,
    }
}

/// Poses the arm: the hold pose with the raising animation at `raise_at`
/// over it, turned by the view's pitch about `Bip01 Looking`, with
/// `Camera1st` at the eye; drawn with the Pip-Boy's field of view.
fn pose_arm(
    arm: &Arm,
    raise_at: Option<f32>,
    camera_transform: &Transform,
    projection: &Projection,
    transforms: &mut Query<&mut Transform, (Without<FlyCamera>, Without<PipboyCamera>)>,
) {
    // `projection` is the camera that draws the arm (the first-person
    // one): scaled across by k, the arm looks as it would drawn with the
    // Pip-Boy's own field of view.
    let drawn_fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => cellview::vertical_fov(cellview::GAME_FOV_DEGREES),
    };
    let own_fov = cellview::vertical_fov(PIPBOY_FOV_DEGREES);
    let k = (drawn_fov * 0.5).tan() / (own_fov * 0.5).tan();
    if let Ok(mut t) = transforms.get_mut(arm.holder) {
        *t = Transform::from_scale(Vec3::new(k, k, 1.0));
    }
    let f = camera_transform.forward().as_vec3();
    let dir = [f.x, -f.z, f.y];
    let heading = dir[0].atan2(dir[1]);
    let pitch = dir[2].clamp(-1.0, 1.0).asin();
    let mut layers: Vec<(&nif::Sequence, f32)> = Vec::new();
    if let Some(s) = arm.skeleton.idle.as_ref() {
        layers.push((s, s.start));
    }
    if let (Some(r), Some(at)) = (arm.raise.as_ref(), raise_at) {
        layers.push((&r.sequence, at));
    }
    let mut pose = nif::posed_layers(&arm.skeleton.bones, &layers);
    let pivot = pose[arm.looking].translation;
    let (s, c) = pitch.sin_cos();
    let turn = nif::Transform {
        rotation: [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        translation: [
            0.0,
            pivot[1] - (c * pivot[1] - s * pivot[2]),
            pivot[2] - (s * pivot[1] + c * pivot[2]),
        ],
        scale: 1.0,
    };
    for &i in &arm.turned {
        pose[i] = turn.then_child(&pose[i]);
    }
    for (joint, bone) in arm.joints.iter().zip(&pose) {
        if let Ok(mut t) = transforms.get_mut(*joint) {
            *t = crate::actors::bevy_transform(bone);
        }
    }
    let eye = game_point(camera_transform.translation);
    let (sh, ch) = heading.sin_cos();
    let cam = pose[arm.camera].translation;
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
    if let Ok(mut t) = transforms.get_mut(arm.root) {
        *t = Transform::from_matrix(local);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pipboy_colour_is_read_as_red_green_blue_alpha() {
        // This install's 4290134783 = 0xFFB642FF: 255, 182, 66.
        let c = colour_of(4_290_134_783);
        assert_eq!(
            [c.x, c.y, c.z].map(|c| (c * 255.0).round() as u8),
            [255, 182, 66]
        );
        assert_eq!(c.w, 1.0);
    }

    /// The pointer's ray meets a screen triangle and its texture
    /// coordinates map onto the menus' picture (`007f8720`).
    #[test]
    fn the_pointer_lands_on_the_screens_picture() {
        let (a, b, c) = (
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(2.0, 0.0, -2.0),
            Vec3::new(0.0, 2.0, -2.0),
        );
        let (t, u, v) = ray_triangle(Vec3::new(0.5, 0.5, 0.0), Vec3::NEG_Z, a, b, c).unwrap();
        assert_eq!((t, u, v), (2.0, 0.25, 0.25));
        // Either side; nothing behind the ray or beside the triangle.
        assert!(ray_triangle(Vec3::new(0.5, 0.5, -4.0), Vec3::Z, a, b, c).is_some());
        assert!(ray_triangle(Vec3::new(0.5, 0.5, -4.0), Vec3::NEG_Z, a, b, c).is_none());
        assert!(ray_triangle(Vec3::new(1.5, 1.5, 0.0), Vec3::NEG_Z, a, b, c).is_none());
        // The screen shows 0 .. 0.753 × 0 .. 0.7625 of the picture.
        assert_eq!(menu_point(Vec2::new(0.5, 0.25)), [640.0, 240.0]);
    }

    /// The mouse failure found live: the Pip-Boy clears `ButtonInput`
    /// every frame to keep the mouse from the rest of the viewer, and a
    /// cleared button never reports coming up, so no click ever finished.
    /// The buttons' events still say it.
    #[test]
    fn a_click_finishes_although_the_buttons_are_cleared() {
        let mut input = ButtonInput::<MouseButton>::default();
        input.press(MouseButton::Left);
        input.reset_all();
        input.clear();
        input.release(MouseButton::Left);
        assert!(!input.just_released(MouseButton::Left));

        use bevy::input::ButtonState::{Pressed, Released};
        let mut held = [false; 2];
        let [l, r] = mouse_buttons(&mut held, [(MouseButton::Left, Pressed)].into_iter());
        assert!(l.pressed && l.down && !l.released);
        assert_eq!(r, ui::pipboy::Button::default());
        let [l, _] = mouse_buttons(&mut held, std::iter::empty());
        assert!(l.down && !l.pressed);
        let [l, _] = mouse_buttons(&mut held, [(MouseButton::Left, Released)].into_iter());
        assert!(l.released && !l.down);
        // Down and up within one frame: both.
        let [_, r] = mouse_buttons(
            &mut held,
            [
                (MouseButton::Right, Pressed),
                (MouseButton::Right, Released),
            ]
            .into_iter(),
        );
        assert!(r.pressed && r.released && !r.down);
    }

    /// A note's pieces' lengths: an OGG's last granule over the rate in its
    /// identification header, a WAV's frames over its rate.
    #[test]
    fn sound_files_know_how_long_they_play() {
        let mut ogg = b"OggS\0\x02".to_vec();
        ogg.extend([0u8; 22]);
        ogg.extend(b"\x01vorbis");
        ogg.extend(0u32.to_le_bytes());
        ogg.push(1);
        ogg.extend(44_100u32.to_le_bytes());
        ogg.extend([0u8; 16]);
        ogg.extend(b"OggS\0\x04");
        ogg.extend(88_200u64.to_le_bytes());
        ogg.extend([0u8; 12]);
        assert_eq!(audio_ms("a.ogg", &ogg), Some(2000.0));
        let samples = 11_025u32;
        let mut wav = b"RIFF".to_vec();
        wav.extend((36 + samples * 2).to_le_bytes());
        wav.extend(b"WAVEfmt ");
        wav.extend(16u32.to_le_bytes());
        wav.extend(1u16.to_le_bytes());
        wav.extend(1u16.to_le_bytes());
        wav.extend(22_050u32.to_le_bytes());
        wav.extend(44_100u32.to_le_bytes());
        wav.extend(2u16.to_le_bytes());
        wav.extend(16u16.to_le_bytes());
        wav.extend(b"data");
        wav.extend((samples * 2).to_le_bytes());
        wav.extend(vec![0u8; samples as usize * 2]);
        assert_eq!(audio_ms("b.wav", &wav), Some(500.0));
    }

    /// `007f99d0`, `007f8320`, `007fa0f0`, `007f8610`: the tab knob's five
    /// places from -0.5 to 1.5; the needle at π/2 − rads/1000 × π; knobs move
    /// toward their targets at their rates, not past them.
    #[test]
    fn knobs_turn_as_the_manager_turns_them() {
        let s = KnobSettings {
            tab_min: -0.5,
            tab_max: 1.5,
            tab_rate: 0.0075,
            scroll_increment: -0.1,
            scroll_rate: 0.0015,
        };
        let mut k = Knobs::default();
        k.init(&s, 250.0, 1000.0);
        assert_eq!(k.tabs, [-0.5, 0.0, 0.5, 1.0, 1.5]);
        assert_eq!(k.now[2], -0.5);
        assert!(
            (k.now[1] - (std::f32::consts::FRAC_PI_2 - 0.25 * std::f32::consts::PI)).abs() < 1e-6
        );
        k.tab(2, &s);
        // 100 ms at 0.0075 a ms: 0.75 on, to 0.25; 100 more would pass the
        // target 0.5: held there.
        assert_eq!(k.update(1100.0), [false, false, true]);
        assert!((k.now[2] - 0.25).abs() < 1e-6);
        k.update(1200.0);
        assert_eq!(k.now[2], 0.5);
        k.turn(0, s.scroll_increment, s.scroll_rate);
        k.update(1250.0);
        assert!((k.now[0] - (-0.075)).abs() < 1e-6);
        k.update(1300.0);
        assert!((k.now[0] - (-0.1)).abs() < 1e-6);
        assert_eq!(k.update(1400.0), [false; 3]);
    }

    /// `004168a0`: Gamebryo's rotation (the transpose of the usual one); a
    /// scroll knob's piece doesn't move at angle 0 (its node's turn is
    /// kept under it).
    #[test]
    fn knob_pieces_turn_about_their_nodes() {
        let r = make_rotation(std::f32::consts::FRAC_PI_2, [0.0, 0.0, 1.0]);
        // Rows applied to x: (0, -1, 0).
        let x = nif::Transform {
            rotation: r,
            translation: [0.0; 3],
            scale: 1.0,
        }
        .apply_direction([1.0, 0.0, 0.0]);
        assert!((x[0]).abs() < 1e-6 && (x[1] + 1.0).abs() < 1e-6);
        let node = nif::Transform {
            rotation: make_rotation(0.3, [0.0, 0.6, 0.8]),
            translation: [5.0, 2.0, 1.0],
            scale: 1.0,
        };
        let local = nif::Transform::IDENTITY;
        let d = knob_delta(0, &node, &local, 0.0);
        let p = d.apply_point([1.0, 2.0, 3.0]);
        assert!(
            (p[0] - 1.0).abs() < 1e-4 && (p[1] - 2.0).abs() < 1e-4 && (p[2] - 3.0).abs() < 1e-4
        );
        // The tab knob at an angle: a point on its axis stays put.
        let flat = nif::Transform::IDENTITY;
        let d = knob_delta(2, &flat, &local, 1.0);
        let p = d.apply_point([0.0, 0.0, 4.0]);
        assert!((p[2] - 4.0).abs() < 1e-5 && p[0].abs() < 1e-5);
    }

    /// The light's place: forward is the heading's way, across to the
    /// right.
    #[test]
    fn the_light_hangs_in_front_of_the_player() {
        let at = light_point([100.0, 200.0, 0.0], 0.0, [0.0, 39.0, 142.0]);
        assert_eq!(at, [100.0, 239.0, 142.0]);
        let east = light_point(
            [0.0, 0.0, 0.0],
            std::f32::consts::FRAC_PI_2,
            [0.0, 39.0, 0.0],
        );
        assert!((east[0] - 39.0).abs() < 1e-4 && east[1].abs() < 1e-4);
    }

    /// The number keys from their events: held carried over, a key's
    /// coming up reported once; other keys ignored.
    #[test]
    fn hot_keys_from_their_events() {
        use bevy::input::ButtonState::{Pressed, Released};
        let keys = hotkey_keys(None);
        let mut held = [false; 8];
        let up = hotkey_events(&keys, &mut held, [(KeyCode::Digit3, Pressed)].into_iter());
        assert!(held[2] && up == [false; 8]);
        let up = hotkey_events(
            &keys,
            &mut held,
            [(KeyCode::Digit3, Released), (KeyCode::KeyA, Released)].into_iter(),
        );
        assert!(!held[2] && up[2]);
        let up = hotkey_events(&keys, &mut held, [(KeyCode::Digit9, Released)].into_iter());
        assert_eq!(up, [false; 8]);
        // The 2 key is Ammo Swap's (slot 1, which nothing is ever put on,
        // so using it does nothing); bound by the INI to R, the 2 key
        // holds nothing and R holds slot 1.
        let up = hotkey_events(&keys, &mut held, [(KeyCode::Digit2, Pressed)].into_iter());
        assert!(held[1] && up == [false; 8]);
        held = [false; 8];
        let mut ini = assets::IniSettings::default();
        ini.add("[Controls]\nAmmo Swap=0013FF01\nHotkey3=0003FFFF\n");
        let c = crate::controls::Controls::read(&ini);
        let keys = hotkey_keys(Some(&c));
        hotkey_events(&keys, &mut held, [(KeyCode::Digit2, Pressed)].into_iter());
        assert!(!held[1] && held[2]);
        hotkey_events(&keys, &mut held, [(KeyCode::KeyR, Pressed)].into_iter());
        assert!(held[1]);
    }

    #[test]
    fn the_start_option_names_a_menu_and_a_page() {
        assert_eq!(parse_start("items:1"), (Section::Items, Some(1)));
        assert_eq!(parse_start("DATA"), (Section::Data, None));
        assert_eq!(parse_start("stats:4"), (Section::Stats, Some(4)));
    }
}
