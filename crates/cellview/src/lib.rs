//! Loads a cell into plain data a real-time renderer can upload directly:
//! meshes, textures with all their mip levels, materials, where each mesh
//! is drawn, lights, and where the player starts.
//!
//! Everything game-specific happens here, so the renderer on top (the Bevy
//! viewer) stays a thin layer: placement follows the game's rules (see the
//! `world` and `preview` crates), triangles are wound the way GPUs cull,
//! and missing normals and texture coordinates are filled in. Positions
//! stay in game space; [`space`] converts to a Y-up, meters renderer.
//!
//! ```no_run
//! let data = cellview::find_data_folder("C:/Games/Fallout New Vegas".as_ref())?;
//! let scene = cellview::load(&data, "GSDocMitchellHouse", &cellview::Options::default())?;
//! println!("{} meshes, {} draws", scene.meshes.len(), scene.draws.len());
//! # Ok::<(), cellview::Error>(())
//! ```

pub mod blackjack;
pub mod caravan;
pub mod game;
pub mod grass;
pub mod impacts;
pub mod lockpick;
pub mod music;
pub mod particles;
pub mod rendered_terminal;
pub mod roulette;
pub mod slots;
pub mod sound;
pub mod space;
pub mod texture;
pub mod tree;
pub mod vigor;
pub mod water;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use assets::Assets;
use esm::load_order::{default_plugins_txt, parse_plugins_txt, MAIN_MASTER};
use esm::{ActivePlugins, FormId};
/// How decals are pulled toward the camera (see `preview::cell`).
pub use preview::cell::{decal_depth_bias, EXTERIOR_DEPTH_SCALE, INTERIOR_DEPTH_SCALE};
/// The eye height and field of view the game uses (see `preview::cell`).
pub use preview::cell::{vertical_fov, Specular, EYE_HEIGHT, GAME_FOV_DEGREES};
/// Pieces moved by their model's own animation, and pieces that turn to
/// face the camera.
pub use preview::cell::{Billboard, BillboardKind, MeshMotion, SwingDoor};
use preview::cell::{CellScene, ModelMesh};
use preview::raster::{AlphaMode, BlendFactor};
pub use preview::raster::{AlphaTest, Falloff};
use world::RotationConvention;

pub use game::{
    Game, LodChunk, SkyDome, TerrainData, TerrainLodBlend, LAND_BLEND_FULL, LAND_BLEND_WIDTH,
    LOD_CHUNK_CELLS, MAX_TERRAIN_TEXTURES,
};
pub use texture::{GpuFormat, TextureData};
pub use water::{WaterData, WaterSettings};
/// The cell's final color adjustment, from its image space.
pub use world::Cinematic as Grade;
pub use world::Hdr;

/// Something went wrong loading; the message says what, for the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

macro_rules! from_error {
    ($($t:ty),*) => {$(
        impl From<$t> for Error {
            fn from(e: $t) -> Self {
                Error(e.to_string())
            }
        }
    )*};
}
from_error!(esm::Error, assets::Error, world::Error, std::io::Error);

/// How to load the game's files.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Read the active plugin list from this file instead of
    /// `%LOCALAPPDATA%\FalloutNV\plugins.txt`.
    pub plugins_txt: Option<PathBuf>,
    /// Load only FalloutNV.esm and the official DLC.
    pub official: bool,
    /// Read the archive list from this ini file.
    pub ini: Option<PathBuf>,
    /// Apply each model's top-node transform, as model viewers do (the game
    /// doesn't).
    pub keep_root_transforms: bool,
}

/// How a surface's transparency works.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blend {
    Opaque,
    /// Cut out where alpha is below this (0..1).
    Mask(f32),
    /// Ordinary transparency.
    Blend,
    /// Cut out below this alpha (0..1), then blend what's left, as the game
    /// does for a mesh that both tests and blends alpha (glass, fabric,
    /// leaves): its draws have the alpha test and blending on together
    /// (`D3DRS_ALPHATESTENABLE` and `D3DRS_ALPHABLENDENABLE`, the gourd in
    /// Doc Mitchell's house). The exact test is in
    /// [`MaterialData::alpha_test`].
    MaskedBlend(f32),
    /// Added on top (glows, light beams).
    Add,
    /// Multiplied with what's behind (dirt, shadows drawn that way).
    Multiply,
    /// Color already multiplied by alpha.
    Premultiplied,
}

/// Everything about a surface except its geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialData {
    /// Index into [`ViewerScene::textures`].
    pub texture: Option<usize>,
    /// Multiplies the texture (or is the color, without one); alpha is the
    /// material's opacity.
    pub color: [f32; 4],
    pub blend: Blend,
    /// The alpha test as the game sets it (`D3DRS_ALPHAFUNC`,
    /// `D3DRS_ALPHAREF`): a pixel is kept when its alpha (0..1, after
    /// every fade) compared with the threshold (0..1) passes. `None`: no
    /// test.
    pub alpha_test: Option<(AlphaTest, f32)>,
    /// Hidden behind nearer surfaces; and hiding what's drawn after it
    /// (see `preview::cell::depth_of`). Blended surfaces that write depth
    /// (the ceiling fan, windows) hide what's behind them that's drawn
    /// later, as in the game.
    pub depth_test: bool,
    pub depth_write: bool,
    /// Where the game sorts the surface among the blended ones: the centre
    /// of the bounding sphere the file stores with the mesh, in the model's
    /// space. Blended meshes are drawn after everything opaque, farthest
    /// first by this point's depth, mesh by mesh (not object by object):
    /// in the recording of Doc Mitchell's house the window panes and light
    /// beams come in the order of their meshes' centres, not their models'
    /// origins. Decals come before them all.
    pub sort_center: [f32; 3],
    /// Drawn at full brightness, ignoring lights.
    pub unlit: bool,
    pub double_sided: bool,
    /// Self-lit color, in the game's lighting units: it is added to the
    /// light falling on the surface (see [`ViewerScene::ambient`]), so 1
    /// shows the texture at full brightness. It can go above 1.
    pub emissive: [f32; 3],
    /// Index into [`ViewerScene::textures`]: the glow map, which masks and
    /// tints the self-lit color. `None` lights the whole surface. Only set
    /// for meshes that light themselves.
    pub glow: Option<usize>,
    /// Lies on another surface and is drawn just in front of it: the game
    /// pulls decals toward the camera by `preview::cell::decal_depth_bias`
    /// in its depth buffer, and they don't write depth.
    pub decal: bool,
    /// Index into [`ViewerScene::textures`]: the normal map (a `linear`
    /// texture; alpha is the specular mask). Lit meshes only.
    pub normal_map: Option<usize>,
    /// The specular highlight, when the shader asks for one.
    pub specular: Option<Specular>,
    /// No-lighting surfaces: the color the game multiplies them by (its
    /// `MaterialColor` constant), as stored values that can go above 1.
    /// White, except for meshes marked for external emittance whose placed
    /// object's Emittance gives a color: that color times the mesh's glow
    /// multiplier (read from a recording of the game's shader constants:
    /// the light beams got (1, 0.89, 0.667), a window panel ten times
    /// that, and lamp shades with a black glow color white).
    pub unlit_color: [f32; 3],
    /// Fades the surface by viewing angle (no-lighting effects such as
    /// light beams); see `preview::raster::Falloff::opacity`.
    pub falloff: Option<Falloff>,
    /// The reflection added on top (lit meshes with the environment map
    /// flag; see `preview::cell::Environment`).
    pub environment: Option<EnvironmentData>,
    /// Where the glow's color comes from when it changes with the time of
    /// day: `emissive` (lit) or `unlit_color` (no-lighting) is then that
    /// color × the link's scale, worked out again as the hour moves.
    pub emittance: Option<EmittanceLink>,
    /// Which of the game's lighting a lit surface gets: plain, skin or hair
    /// (see `preview::cell::Shading`).
    pub shading: preview::cell::Shading,
    /// Hair: the NPC's hair colour, the hair shader's `HairTint` (0..1);
    /// the tint itself is already in the vertex colours.
    pub hair_tint: Option<[f32; 3]>,
}

/// A surface marked for external emittance whose color comes from a
/// region's weather (`world::weather::EmittanceNow`): a region its placed
/// object's Emittance names, or with none the player's weather region.
/// Light colors don't change, so surfaces taking a light's color have none.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmittanceLink {
    /// The region; `None`: the player's weather region.
    pub region: Option<FormId>,
    /// The color times this is the glow: the mesh's glow multiplier (× the
    /// image space's emissive multiplier on lit surfaces).
    pub scale: f32,
    /// The glow when the source gives no color: the mesh's own glow color
    /// (lit, × the image space's multiplier) or white (no-lighting).
    pub fallback: [f32; 3],
}

impl EmittanceLink {
    /// The glow for a color from [`world::weather::EmittanceNow`].
    pub fn glow(&self, now: &world::weather::EmittanceNow) -> [f32; 3] {
        let color = match self.region {
            Some(r) => now.regions.get(&r).copied(),
            None => now.player_region,
        };
        color.map_or(self.fallback, |c| c.map(|v| v * self.scale))
    }
}

/// A surface's reflection, ready to draw.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvironmentData {
    /// Index into [`ViewerScene::textures`]: a cube map (`layers` 6), sampled
    /// as stored. Its directions are the game's world axes (x east, y
    /// north, z up).
    pub cube: usize,
    /// Index into [`ViewerScene::textures`]: the mask (red channel, a
    /// `linear` texture). `None`: the normal map's alpha is the mask.
    pub mask: Option<usize>,
    /// The shader's environment map scale.
    pub strength: f32,
    /// Window reflections: the view direction turned around.
    pub window: bool,
}

/// One drawable piece of a model, in the model's space (game units). Every
/// array has one entry per vertex; triangles are counter-clockwise seen
/// from their front.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshData {
    /// Model path and piece name, for messages.
    pub name: String,
    /// Exact geometry name from the NIF, used by model-driven menus.
    /// Kept separately from the diagnostic model path and piece index.
    pub shape_name: String,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// For the normal map: the direction the texture's U runs (xyz, unit
    /// length) and in `w` which side its V runs: V = w × (normal × U).
    /// Present when the mesh stores its tangent space.
    pub tangents: Option<Vec<[f32; 4]>>,
    pub uvs: Vec<[f32; 2]>,
    /// Linear RGBA multipliers (converted from the file's gamma-encoded
    /// colors), when the game uses the mesh's vertex colors.
    pub colors: Option<Vec<[f32; 4]>>,
    pub indices: Vec<u16>,
    pub material: MaterialData,
    /// A light effect (glow, beam, haze) rather than a solid surface.
    pub effect: bool,
    /// Actor pieces: the bones that move it. The arrays above then hold
    /// the piece in its bind pose (its own space), for skinning.
    pub rig: Option<RigData>,
    /// Pieces the model's own animation moves or changes (a ceiling fan's
    /// blades, a window's glow; `preview::cell::placed_sequences`):
    /// [`piece_now`] gives the move and the material at each moment.
    pub motion: Option<std::sync::Arc<PieceMotion>>,
    /// Pieces that turn to face the camera ([`billboard_matrix`]).
    pub billboard: Option<Billboard>,
    /// Drawn into the local map's pictures (`preview::cell::ModelMesh::
    /// local_map`).
    pub local_map: bool,
    /// Actor pieces: which part of the actor's look it comes from
    /// (`preview::cell::ModelMesh::actor_part`).
    pub actor_part: Option<u16>,
    /// Stored as triangle strips: what takes the game's world decals
    /// (`preview::cell::ModelMesh::strips`).
    pub strips: bool,
}

/// How a billboard piece is turned for a camera at `eye` (game units)
/// whose right, up and backward directions are `axes` (game axes): a
/// column-major 4x4 matrix in the model's space, applied before the draw's
/// own transform (`world = draw × this × vertex`), as for
/// [`motion_matrix`].
pub fn billboard_matrix(
    billboard: &Billboard,
    draw: &[f32; 16],
    eye: [f32; 3],
    axes: [[f32; 3]; 3],
) -> [f32; 16] {
    column_major(&billboard.facing(&from_column_major(draw), eye, axes))
}

/// A column-major model matrix (rotation and uniform scale, then a move)
/// back as a transform.
fn from_column_major(m: &[f32; 16]) -> nif::math::Transform {
    let scale = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
    let s = if scale > 0.0 { 1.0 / scale } else { 0.0 };
    nif::math::Transform {
        rotation: [
            [m[0] * s, m[4] * s, m[8] * s],
            [m[1] * s, m[5] * s, m[9] * s],
            [m[2] * s, m[6] * s, m[10] * s],
        ],
        translation: [m[12], m[13], m[14]],
        scale,
    }
}

/// A piece its model's animation changes, with what its material is made
/// of when the animation doesn't say: so a changed glow colour or
/// multiplier, or opacity, can be put together the way [`mesh_data`] puts
/// the material's own together.
#[derive(Debug, Clone, PartialEq)]
pub struct PieceMotion {
    pub motion: MeshMotion,
    /// The material's own glow colour and multiplier.
    pub emissive_color: [f32; 3],
    pub emissive_mult: f32,
    /// What multiplies the glow on top (the image space's emissive
    /// multiplier for lit pieces); 0 when the glow isn't the material's
    /// (no-lighting pieces, and glows taken from the placed object's
    /// Emittance), which then isn't changed.
    pub emissive_scale: f32,
    /// The material's own opacity.
    pub opacity: f32,
}

impl PieceMotion {
    /// Whether any of the model's sequences changes this piece's material
    /// (so the piece needs a material of its own).
    pub fn changes_material(&self) -> bool {
        let Some((shape, _)) = self.motion.nodes.last() else {
            return false;
        };
        self.motion.all.iter().any(|s| {
            s.materials
                .iter()
                .any(|m| m.node.eq_ignore_ascii_case(shape))
        })
    }
}

/// What a changing piece shows at a moment ([`piece_now`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceNow {
    /// Its move: a column-major 4x4 matrix in the model's space (game
    /// units), applied before the draw's own transform
    /// (`world = draw × this × vertex`).
    pub matrix: [f32; 16],
    /// The material's opacity, when the animation sets it.
    pub opacity: Option<f32>,
    /// The glow ([`MaterialData::emissive`]), when the animation sets its
    /// colour or multiplier.
    pub emissive: Option<[f32; 3]>,
}

/// A changing piece `seconds` after its object appeared; with `group`, the
/// sequence a script started (`PlayGroup`) and the seconds since.
pub fn piece_now(piece: &PieceMotion, seconds: f32, group: Option<(&str, f32)>) -> PieceNow {
    piece_shown(piece, &piece.motion.layers(seconds, group))
}

/// A changing piece with one of its model's sequences by name at a moment
/// of that sequence's own clock (a door's `Open` or `Close` as it swings,
/// `world::doors`); `None` when the model has no such sequence.
pub fn piece_in_sequence(piece: &PieceMotion, name: &str, seconds: f32) -> Option<PieceNow> {
    let layer = piece.motion.sequence_at(name, seconds)?;
    Some(piece_shown(piece, &[layer]))
}

fn piece_shown(piece: &PieceMotion, layers: &[(&nif::Sequence, f32)]) -> PieceNow {
    let layers = layers.to_vec();
    let material = piece.motion.material(&layers);
    let emissive = (piece.emissive_scale > 0.0
        && (material.emissive.is_some() || material.emissive_mult.is_some()))
    .then(|| {
        let color = material.emissive.unwrap_or(piece.emissive_color);
        let mult = material.emissive_mult.unwrap_or(piece.emissive_mult);
        color.map(|c| c * mult * piece.emissive_scale)
    });
    PieceNow {
        matrix: column_major(&piece.motion.transform(&layers)),
        opacity: material.alpha.map(|a| a.clamp(0.0, 1.0)),
        emissive,
    }
}

/// Where a moving piece is `seconds` after its object appeared, with the
/// sequences shown from the start: [`PieceNow::matrix`].
pub fn motion_matrix(motion: &MeshMotion, seconds: f32) -> [f32; 16] {
    column_major(&motion.at(seconds))
}

/// How an actor piece is skinned: `world = joint bone × inverse bind ×
/// vertex`, blended by up to four weights per vertex (see
/// `preview::cell::Rig`).
#[derive(Debug, Clone, PartialEq)]
pub struct RigData {
    /// Per joint: the bone (an index into its actor's skeleton) and the
    /// column-major transform from the piece's space to that bone's.
    pub joints: Vec<(usize, [f32; 16])>,
    /// Per vertex: four joints and their weights.
    pub joint_indices: Vec<[u16; 4]>,
    pub joint_weights: Vec<[f32; 4]>,
    /// FaceGen head parts: the morphs that move `positions` as the face
    /// talks and blinks (`world::face`).
    pub face: Option<std::sync::Arc<world::face::FaceMorphs>>,
}

/// A placed person or creature: its skeleton and idle, and where it
/// stands (column-major, game space).
#[derive(Debug, Clone)]
pub struct ActorData {
    pub skeleton: std::sync::Arc<preview::cell::ActorSkeleton>,
    pub transform: [f32; 16],
    /// The placed reference and its base record (form IDs), for talking.
    pub reference: u32,
    pub base: u32,
    /// Where it stands (game units).
    pub position: [f32; 3],
    /// A woman (the idle tree has women's sitting idles).
    pub female: bool,
    /// What it was built from (its pieces' `MeshData::actor_part` index its
    /// parts), for redrawing what changes.
    pub look: Option<std::sync::Arc<world::ActorLook>>,
}

/// A mesh drawn somewhere: `transform` (column-major 4x4) takes the mesh's
/// model space to game space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Draw {
    /// Index into [`ViewerScene::meshes`].
    pub mesh: usize,
    pub transform: [f32; 16],
    /// For an actor's pieces: an index into [`ViewerScene::actors`].
    pub actor: Option<usize>,
    /// The placed reference it belongs to (a form ID; 0 for none), so
    /// scripts' `Enable` / `Disable` can show and hide it.
    pub reference: u32,
}

/// The cell's directional light.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Directional {
    /// Unit vector toward the light, in game space.
    pub direction: [f32; 3],
    /// 0..1 per channel.
    pub color: [f32; 3],
}

/// The cell's fog. The game's lit shaders (`SLS2001.vso` and the rest)
/// blend each surface toward `color` by
/// `(1 - saturate((FogParam.x - d) / FogParam.y)) ^ FogParam.z`, where `d`
/// is the length of the position after the camera's projection (x, y and
/// depth, so height on screen counts more than distance sideways). Taking
/// `FogParam` as (far, far - near, power) gives
/// `saturate((d - near) / (far - near)) ^ power`; that mapping is inferred
/// (the engine sets it), the formula is the shader's.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fog {
    /// 0..1 per channel, as stored.
    pub color: [f32; 3],
    /// Game units.
    pub near: f32,
    pub far: f32,
    pub power: f32,
}

/// The game's near clip plane, in game units: its projection matrices put
/// depth as `Q × (z − 5)` (recorded in Doc Mitchell's house and at
/// Goodsprings: `z × 1.000014 − 5.000071` outdoors), and the fog's
/// distance (the length of the projected position) takes that depth.
pub const GAME_NEAR_CLIP: f32 = 5.0;

/// A place's light as the renderer takes it: the ambient colour (0..1),
/// the directional light (if it has any colour) and the fog (if it has
/// any distance).
pub fn scene_light(l: &world::Lighting) -> ([f32; 3], Option<Directional>, Option<Fog>) {
    let ambient = l.ambient.map(|c| f32::from(c) / 255.0);
    let color = l.directional.map(|c| f32::from(c) / 255.0);
    let directional = color.iter().any(|&c| c > 0.0).then(|| Directional {
        direction: l.toward_directional(),
        color,
    });
    let fog = (l.fog_far > l.fog_near).then(|| Fog {
        color: l.fog_color.map(|c| f32::from(c) / 255.0),
        near: l.fog_near,
        far: l.fog_far,
        power: if l.fog_power > 0.0 { l.fog_power } else { 1.0 },
    });
    (ambient, directional, fog)
}

/// The outdoors' light at an hour (`world::weather::SkyLight`, unrounded)
/// as [`scene_light`] gives a cell's: the ambient, the sun (its colour as
/// the weather gives it, before any dimmer) and the fog.
pub fn exterior_light(
    l: &world::weather::SkyLight,
) -> ([f32; 3], Option<Directional>, Option<Fog>) {
    let directional = l.sunlight.iter().any(|&c| c > 0.0).then_some(Directional {
        direction: l.toward_sun,
        color: l.sunlight,
    });
    let fog = (l.fog_far > l.fog_near).then_some(Fog {
        color: l.fog_color,
        near: l.fog_near,
        far: l.fog_far,
        power: if l.fog_power > 0.0 { l.fog_power } else { 1.0 },
    });
    (l.ambient, directional, fog)
}

/// A light source, in game units. The game lights a surface with
/// `color * (1 - saturate((distance / radius)^2)) * saturate(N·L)`, adding
/// every light, the ambient and the directional light together, then
/// multiplies the surface's stored (gamma-encoded) color by the sum; read
/// from its lit-surface shaders (`SLS2029` in `shaderpackage013.sdp`). The
/// color, radius and fade are the placed light's (`world::PlacedLight`):
/// the game sends `color * fade` and this radius, confirmed by recording
/// its shader constants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightData {
    pub position: [f32; 3],
    /// 0..1 per channel.
    pub color: [f32; 3],
    /// Reach in game units.
    pub radius: f32,
    /// Brightness multiplier from the light's record.
    pub fade: f32,
}

/// The lights that reach an outdoor square (`square`: its grid x, y),
/// out of all the loaded squares' lights: each whose radius reaches the
/// square's ground area (measured flat), nearest first (a light standing
/// in the square comes first), at most `max`. A light near a square's
/// edge lights its neighbour too, as it does in the game, where lights
/// aren't bound to squares. Which lights the game picks when more than
/// the shaders take reach a surface isn't traced (the nearest are kept).
pub fn lights_reaching(square: (i32, i32), lights: &[LightData], max: usize) -> Vec<LightData> {
    let size = world::land::CELL_SIZE;
    let (x0, y0) = (square.0 as f32 * size, square.1 as f32 * size);
    let (x1, y1) = (x0 + size, y0 + size);
    let mut near: Vec<(f32, LightData)> = lights
        .iter()
        .filter_map(|l| {
            let [x, y, _] = l.position;
            let dx = (x0 - x).max(0.0).max(x - x1);
            let dy = (y0 - y).max(0.0).max(y - y1);
            let d = dx.hypot(dy);
            (d < l.radius).then_some((d, *l))
        })
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0));
    near.into_iter().take(max).map(|(_, l)| l).collect()
}

/// Where the camera starts: eye position in game units, and the compass
/// heading (radians clockwise from north).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Start {
    pub eye: [f32; 3],
    pub heading: f32,
    /// Where the player arrives this way, for display.
    pub via: &'static str,
}

/// A cell, ready to draw.
pub struct ViewerScene {
    /// Editor ID or name of the cell.
    pub cell: String,
    /// The cell's form ID.
    pub cell_id: u32,
    pub meshes: Vec<MeshData>,
    pub textures: Vec<TextureData>,
    pub draws: Vec<Draw>,
    /// World-space visible mesh bounds for each placed reference that has
    /// drawable geometry. Used for interaction when the record's OBND is
    /// absent or empty.
    pub object_bounds: Vec<ObjectBounds>,
    pub lights: Vec<LightData>,
    /// The cell's ambient light, 0..1 per channel.
    pub ambient: [f32; 3],
    pub directional: Option<Directional>,
    pub fog: Option<Fog>,
    pub start: Start,
    /// The color adjustment the game applies to the finished picture in
    /// this cell, when its image space names one.
    pub grade: Option<Grade>,
    /// The image space's HDR values (bloom threshold, strength, blur
    /// radius, brightness limits).
    pub hdr: Option<Hdr>,
    /// Every solid surface the player walks into, in game units: the
    /// placed models' Havok collision on the layers that stop a character.
    pub collision: physics::Collider,
    /// Load doors: where they are and where they lead.
    pub doors: Vec<DoorData>,
    /// Placed objects Havok moves (clutter): their bodies, for the viewer's
    /// simulation (`physics::rigid`); their collision isn't in [`Self::collision`].
    pub bodies: Vec<preview::cell::DynamicBody>,
    /// Doors that open where they stand (`world::doors`): their models'
    /// sequences and the collision their leaves swing with.
    pub swing_doors: Vec<SwingDoor>,
    /// An exterior cell's terrain, a mesh per quarter.
    pub terrain: Vec<TerrainData>,
    /// Outdoors: the sky's colours by day as stored (0..1): upper sky,
    /// horizon, lower sky. There's no sky dome yet; the viewer clears to
    /// the horizon colour.
    pub sky: Option<[[f32; 3]; 3]>,
    /// Outdoors: the weather (its clouds: `Game::clouds`).
    pub weather: Option<esm::FormId>,
    /// People and creatures, for animating their pieces.
    pub actors: Vec<ActorData>,
    /// The water: an outdoor square's own and placed water (see [`water`]).
    pub water: Vec<WaterData>,
    /// Placed objects' particle systems (see [`particles`]).
    pub particles: Vec<particles::ParticleData>,
    /// What went into the scene and what couldn't, one line each.
    pub notes: Vec<String>,
}

/// The world-space bounds of one placed object's rendered geometry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectBounds {
    pub reference: u32,
    pub lo: [f32; 3],
    pub hi: [f32; 3],
}

/// A door that takes the player to another place.
#[derive(Debug, Clone, PartialEq)]
pub struct DoorData {
    /// The door's base object, for display.
    pub name: String,
    /// The box around the door's model, in game units.
    pub lo: [f32; 3],
    pub hi: [f32; 3],
    /// Where the player arrives: feet position, and the heading (radians
    /// clockwise from north) from the destination's `XTEL`.
    pub arrive: [f32; 3],
    pub arrive_heading: f32,
    /// The cell holding the destination door (a form ID), when known.
    pub cell: Option<u32>,
    /// That cell, for display (and whether it's an interior one).
    pub cell_label: String,
    pub interior: bool,
    /// For a door to the outdoors: the worldspace (a form ID).
    pub world: Option<u32>,
    /// The placed door (a form ID), for its script and whether a script
    /// has made it stop working (`SetDestroyed`).
    pub reference: u32,
}

/// How far the player reaches to open a door or pick something up: the
/// game's `iActivatePickLength` (150 units; built in, New Vegas doesn't
/// change it).
pub const ACTIVATE_REACH: f32 = 150.0;

/// The game's `Data` folder, given it or the install folder containing it.
pub fn find_data_folder(path: &Path) -> Result<PathBuf, Error> {
    for folder in [path.to_path_buf(), path.join("Data")] {
        let has_master = std::fs::read_dir(&folder).is_ok_and(|entries| {
            entries.flatten().any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .eq_ignore_ascii_case(MAIN_MASTER)
            })
        });
        if has_master {
            return Ok(folder);
        }
    }
    Err(Error(format!(
        "neither {} nor a Data folder inside it contains {MAIN_MASTER}",
        path.display()
    )))
}

pub(crate) fn active_plugins(options: &Options) -> Result<ActivePlugins, Error> {
    if options.official {
        return Ok(ActivePlugins::OfficialOnly);
    }
    let Some(path) = options.plugins_txt.clone().or_else(default_plugins_txt) else {
        return Ok(ActivePlugins::OfficialOnly);
    };
    let bytes = std::fs::read(&path)
        .map_err(|e| Error(format!("could not read {}: {e}", path.display())))?;
    Ok(ActivePlugins::List(parse_plugins_txt(&bytes)))
}

/// Loads a cell (by editor ID, form ID or name) from a `Data` folder. To
/// load several places, open a [`Game`] once instead.
pub fn load(data_dir: &Path, cell: &str, options: &Options) -> Result<ViewerScene, Error> {
    let game = Game::open(data_dir, options)?;
    let id = game.find_cell(cell)?;
    game.load_cell(id)
}

/// Turns a loaded cell scene into renderer-ready data, loading the full
/// textures from the game's files.
pub fn convert_cell(scene: &CellScene, assets: &Assets) -> ViewerScene {
    let mut cache = TextureCache::new(assets);
    let mut viewer = convert(scene, &mut cache);
    let (textures, unreadable) = cache.finish();
    viewer.textures = textures;
    for line in unreadable {
        viewer.notes.push(format!("unreadable texture: {line}"));
    }
    viewer
}

/// [`convert_cell`] with the textures gathered in `cache` (the scene's
/// `textures` stay empty until the caller takes them from the cache).
pub(crate) fn convert(scene: &CellScene, cache: &mut TextureCache<'_>) -> ViewerScene {
    let mut notes = Vec::new();

    // One mesh per piece of each model, shared by every placement, except
    // that placements whose glow differs (it can come from the placed
    // object) get their own copy.
    let convention = RotationConvention::DEFAULT;
    let object_bounds = scene
        .object_bounds(convention)
        .into_iter()
        .enumerate()
        .filter_map(|(index, bounds)| {
            let (lo, hi, _) = bounds?;
            let object = scene.cell.objects.get(index)?;
            Some(ObjectBounds {
                reference: object.form_id.0,
                lo,
                hi,
            })
        })
        .collect();
    // The image space's emissive multiplier, on lit glows only (see
    // `world::Hdr::emissive_mult`).
    let emissive_mult = scene
        .cell
        .info
        .image_space
        .as_ref()
        .and_then(|s| s.hdr())
        .map_or(1.0, |h| h.emissive_mult);
    let mut meshes = Vec::new();
    let mut draws = Vec::new();
    type Source = Option<Option<FormId>>;
    // The sequences moving a piece, by name.
    type Moving = Option<Vec<String>>;
    let mut variants: HashMap<(usize, usize, [u32; 3], Source, Moving), usize> = HashMap::new();
    let mut actors = Vec::new();
    let mut particle_data = Vec::new();
    for instance in &scene.instances {
        let transform = column_major(&scene.transform(instance, convention));
        let model = &scene.models[instance.model];
        if let Some(pm) = &model.particles {
            let placed = &scene.cell.objects[instance.object];
            let link = match placed.emittance {
                None => Some(None),
                Some(world::Emittance::Region(r)) => Some(Some(r)),
                Some(_) => None,
            };
            let openable = preview::cell::opens_and_closes(placed.base_type);
            let playing = particles::playing(pm, openable);
            particle_data.push(particles::ParticleData {
                reference: placed.form_id.0,
                placed: scene.transform(instance, convention),
                model: pm.clone(),
                looks: particles::looks(
                    pm,
                    |path| cache.get(path),
                    scene.cell.emittance_color(placed),
                    link,
                    &playing,
                ),
                motion: particles::motion(pm, openable),
                playing,
            });
        }
        let actor = model.skeleton.as_ref().map(|skeleton| {
            let placed = &scene.cell.objects[instance.object];
            actors.push(ActorData {
                skeleton: skeleton.clone(),
                transform,
                reference: placed.form_id.0,
                base: placed.base.0,
                position: placed.position,
                female: placed.actor.as_ref().is_some_and(|a| a.female),
                look: placed.actor.clone().map(std::sync::Arc::new),
            });
            actors.len() - 1
        });
        for (k, mesh) in model.meshes.iter().enumerate() {
            if mesh.triangles.is_empty() || mesh.positions.is_empty() {
                continue;
            }
            let mut emissive = scene.emissive_for(instance, mesh);
            if mesh.lit {
                emissive = emissive.map(|c| c * emissive_mult);
            }
            // A glow whose color follows a region's weather.
            let link = mesh
                .external_emittance
                .then(|| match scene.cell.objects[instance.object].emittance {
                    None => Some(None),
                    Some(world::Emittance::Region(r)) => Some(Some(r)),
                    Some(_) => None,
                })
                .flatten()
                .map(|region| {
                    if mesh.lit {
                        EmittanceLink {
                            region,
                            scale: mesh.emissive_mult * emissive_mult,
                            fallback: mesh.emissive.map(|c| c * emissive_mult),
                        }
                    } else {
                        EmittanceLink {
                            region,
                            scale: mesh.emissive_mult,
                            fallback: [1.0; 3],
                        }
                    }
                });
            // The model's own animation, as this placement plays it (see
            // `preview::cell::placed_sequences`). Billboards turn instead
            // (both on one piece isn't done).
            let motion = if mesh.nodes.is_empty() || mesh.billboard.is_some() {
                None
            } else {
                let base_type = scene.cell.objects[instance.object].base_type;
                let playing = preview::cell::placed_sequences(
                    &mesh.sequences,
                    preview::cell::opens_and_closes(base_type),
                );
                MeshMotion::of(&mesh.nodes, &playing, &mesh.sequences).map(|motion| {
                    // The glow is the material's own only for lit pieces
                    // not taking it from the placed object.
                    let own_glow = mesh.lit && !mesh.external_emittance;
                    let mult = mesh.emissive_mult;
                    std::sync::Arc::new(PieceMotion {
                        motion,
                        emissive_color: mesh
                            .emissive
                            .map(|c| if mult > 0.0 { c / mult } else { c }),
                        emissive_mult: mult,
                        emissive_scale: if own_glow { emissive_mult } else { 0.0 },
                        opacity: mesh.opacity,
                    })
                })
            };
            let key = (
                instance.model,
                k,
                emissive.map(f32::to_bits),
                link.map(|l| l.region),
                motion.as_ref().map(|m| {
                    m.motion
                        .sequences
                        .iter()
                        .map(|p| format!("{} {} {}", p.sequence.name, p.runs, p.at_end))
                        .collect()
                }),
            );
            let index = match variants.get(&key) {
                Some(&index) => index,
                None => {
                    let texture = match (&mesh.texture_path, mesh.texture) {
                        (Some(path), Some(_)) => cache.get(path),
                        _ => None,
                    };
                    let self_lit = mesh.lit && emissive.iter().any(|&c| c > 0.0);
                    let glow = match (&mesh.glow_path, mesh.glow) {
                        (Some(glow), Some(_)) if self_lit => cache.get(glow),
                        _ => None,
                    };
                    let normal_map = mesh
                        .normal_path
                        .as_deref()
                        .and_then(|path| cache.get_linear(path));
                    let mut data = mesh_data(
                        &format!("{} #{k}", model.path),
                        mesh,
                        emissive,
                        texture,
                        glow,
                    );
                    data.material.normal_map = normal_map;
                    if !specular_allowed(normal_map.map(|i| cache.textures[i].has_alpha_channel()))
                    {
                        data.material.specular = None;
                    }
                    if let Some(rig) = &mesh.rig {
                        rig_into(&mut data, rig, mesh.lit);
                    }
                    data.material.environment = mesh.environment.as_ref().and_then(|e| {
                        Some(EnvironmentData {
                            cube: cache.get_cube(&e.cube_path)?,
                            mask: e.mask_path.as_deref().and_then(|p| cache.get_linear(p)),
                            strength: e.strength,
                            window: e.window,
                        })
                    });
                    data.material.emittance = link;
                    data.motion = motion;
                    if !mesh.lit {
                        data.material.unlit_color = unlit_color(
                            mesh.external_emittance,
                            mesh.emissive_mult,
                            scene
                                .cell
                                .emittance_color(&scene.cell.objects[instance.object]),
                        );
                    }
                    meshes.push(data);
                    variants.insert(key, meshes.len() - 1);
                    meshes.len() - 1
                }
            };
            draws.push(Draw {
                mesh: index,
                transform,
                actor,
                reference: scene.cell.objects[instance.object].form_id.0,
            });
        }
    }
    let collision = scene.collider(convention);
    let doors = cell_doors(scene, convention);
    let swing_doors = scene.swing_doors(convention);
    let bodies = scene.dynamic_bodies(convention);

    let cell = &scene.cell;
    let lights: Vec<LightData> = cell
        .lights()
        .filter(|(_, l)| !l.is_off_by_default() && !l.is_negative())
        .filter_map(|(p, _)| cell.placed_light(p).map(|l| (p, l)))
        .filter(|(_, l)| l.radius > 0.0)
        .map(|(p, l)| LightData {
            position: p.position,
            color: l.color,
            radius: l.radius,
            fade: l.fade,
        })
        .collect();
    let (ambient, directional, fog) = match &cell.info.lighting {
        Some(l) => scene_light(l),
        None => ([0.3; 3], None, None),
    };

    let start = start_point(scene);
    let report = &scene.report;
    notes.push(format!(
        "{} placed objects, {} meshes ({} draws), {} textures, {} lights",
        cell.objects.len(),
        meshes.len(),
        draws.len(),
        cache.textures.len(),
        lights.len()
    ));
    if !particle_data.is_empty() {
        notes.push(format!(
            "particles: {} placed objects with {} particle systems",
            particle_data.len(),
            particle_data
                .iter()
                .map(|p| p.model.systems.len())
                .sum::<usize>()
        ));
        // Each system's colour as placed (`MaterialColor`), counted.
        let mut colours: Vec<(String, usize)> = Vec::new();
        for p in &particle_data {
            for (s, look) in p.model.systems.iter().zip(&p.looks) {
                let [r, g, b, a] = look.color;
                let line = format!(
                    "{} {r:.3},{g:.3},{b:.3} alpha {a:.2}{}",
                    s.name,
                    look.left_out
                        .map(|why| format!(" (not drawn: {why})"))
                        .unwrap_or_default()
                );
                match colours.iter_mut().find(|(l, _)| *l == line) {
                    Some((_, n)) => *n += 1,
                    None => colours.push((line, 1)),
                }
            }
        }
        for (line, n) in colours.iter().take(8) {
            notes.push(format!("  particle colour: {line} (x{n})"));
        }
    }
    if let Some(l) = &cell.info.lighting {
        let rgb = |c: [u8; 3]| format!("{},{},{}", c[0], c[1], c[2]);
        notes.push(format!(
            "lighting: ambient {}, directional {} from {}° around, {}° up ({})",
            rgb(l.ambient),
            rgb(l.directional),
            l.directional_rotation_xy,
            l.directional_rotation_z,
            cell.info.lighting_source
        ));
    }
    if let Some(f) = &fog {
        notes.push(format!(
            "fog: {:.0},{:.0},{:.0} from {:.0} to {:.0} units, power {:.2}",
            f.color[0] * 255.0,
            f.color[1] * 255.0,
            f.color[2] * 255.0,
            f.near,
            f.far,
            f.power
        ));
    }
    let self_lit = meshes
        .iter()
        .filter(|m| !m.material.unlit && m.material.emissive.iter().any(|&c| c > 0.0))
        .count();
    let glow_mapped = meshes.iter().filter(|m| m.material.glow.is_some()).count();
    if self_lit > 0 {
        notes.push(format!(
            "{self_lit} self-lit pieces, {glow_mapped} of them through a glow map"
        ));
    }
    let grade = cell.info.image_space.as_ref().and_then(|s| s.cinematic());
    let hdr = cell.info.image_space.as_ref().and_then(|s| s.hdr());
    notes.push(image_space_note(cell.info.image_space.as_ref()));
    for (path, n) in &report.missing_models {
        notes.push(format!("missing model: {path} (x{n})"));
    }
    for path in report.missing_textures.keys() {
        notes.push(format!("missing texture: {path}"));
    }
    for (path, e) in report.unreadable_models.iter() {
        notes.push(format!("unreadable model: {path}: {e}"));
    }
    let opening: Vec<&world::Placement> = cell
        .objects
        .iter()
        .filter(|o| preview::cell::is_opening_door(o))
        .collect();
    let markers = cell
        .markers
        .iter()
        .filter(|m| m.base == world::COLLISION_MARKER && m.primitive.is_some())
        .count();
    notes.push(format!(
        "collision: {} triangles; {} load doors; {} doors that open (E), {} of them placed open; {} collision markers",
        collision.triangle_count(),
        doors.len(),
        opening.len(),
        opening.iter().filter(|d| d.open_by_default).count(),
        markers
    ));
    for (path, e) in report.unreadable_collision.iter() {
        notes.push(format!("unreadable collision: {path}: {e}"));
    }
    notes.push(format!(
        "{} placed objects Havok moves (clutter bodies)",
        bodies.len()
    ));
    let held: Vec<String> = scene
        .unsimulated_bodies()
        .into_iter()
        .map(|(r, name)| format!("{r} ({name})"))
        .collect();
    if !held.is_empty() {
        notes.push(format!(
            "{} placed objects with joined or several moving bodies, kept solid (constraints aren't simulated): {}",
            held.len(),
            held.join(", ")
        ));
    }

    ViewerScene {
        cell: cell.info.label(),
        cell_id: cell.info.form_id.0,
        meshes,
        textures: Vec::new(),
        draws,
        object_bounds,
        lights,
        ambient,
        directional,
        fog,
        start,
        grade,
        hdr,
        collision,
        doors,
        swing_doors,
        bodies,
        terrain: Vec::new(),
        actors,
        sky: cell
            .info
            .sky
            .map(|s| s.map(|c| c.map(|v| f32::from(v) / 255.0))),
        weather: cell.info.weather,
        water: Vec::new(),
        particles: particle_data,
        notes,
    }
}

/// Load doors (references with a destination, `XTEL`): the box around
/// each one's model and where it leads. The destination cell is filled in
/// by [`load`], which has the load order.
fn cell_doors(scene: &CellScene, convention: RotationConvention) -> Vec<DoorData> {
    let bounds = scene.object_bounds(convention);
    scene
        .cell
        .objects
        .iter()
        .enumerate()
        .filter_map(|(i, object)| {
            let teleport = object.teleport?;
            let (lo, hi, _) = bounds.get(i).copied().flatten()?;
            Some(DoorData {
                name: object
                    .base_editor_id
                    .clone()
                    .unwrap_or_else(|| object.base.to_string()),
                lo,
                hi,
                arrive: teleport.position,
                arrive_heading: teleport.rotation[2],
                cell: None,
                cell_label: format!("the far side of {}", teleport.door),
                interior: false,
                world: None,
                reference: object.form_id.0,
            })
        })
        .collect()
}

/// One line about the cell's image space, with the raw values around the
/// cinematic ones so the layout can be checked against the game.
fn image_space_note(space: Option<&world::ImageSpace>) -> String {
    let Some(space) = space else {
        return "image space: the cell names none".into();
    };
    let name = space
        .editor_id
        .clone()
        .unwrap_or_else(|| space.form_id.to_string());
    let raw: Vec<String> = space
        .values
        .iter()
        .skip(21)
        .take(13)
        .map(|v| format!("{v:.3}"))
        .collect();
    let raw = format!("({} bytes; values 21-33: {})", space.size, raw.join(" "));
    match space.cinematic() {
        Some(c) => format!(
            "image space {name}: saturation {:.2}, tint {:.2},{:.2},{:.2} at {:.2}, \
             brightness {:.2}, contrast {:.2} around {:.2} {raw}",
            c.saturation,
            c.tint[0],
            c.tint[1],
            c.tint[2],
            c.tint_amount,
            c.brightness,
            c.contrast,
            c.contrast_average
        ),
        None => format!("image space {name}: values not understood, not applied {raw}"),
    }
}

/// Textures loaded so far, each once.
pub(crate) struct TextureCache<'a> {
    assets: &'a Assets,
    textures: Vec<TextureData>,
    /// Path (or "diffuse * glow" pair) to index; `None` when it couldn't be
    /// loaded.
    index: HashMap<String, Option<usize>>,
    unreadable: Vec<String>,
}

impl<'a> TextureCache<'a> {
    pub(crate) fn new(assets: &'a Assets) -> Self {
        Self {
            assets,
            textures: Vec::new(),
            index: HashMap::new(),
            unreadable: Vec::new(),
        }
    }

    /// The textures, and a line for each that couldn't be read.
    pub(crate) fn finish(self) -> (Vec<TextureData>, Vec<String>) {
        (self.textures, self.unreadable)
    }
}

impl TextureCache<'_> {
    /// A skin tint: a file, or a body tint the game makes when it has no
    /// file for it (`world::actor::MadeBodyTint`: its file if there, else
    /// made from the race's texture morphs, `nif::Egt::tint`).
    fn tint(&self, reference: &str) -> Option<TextureData> {
        let read = |path: &str| {
            self.assets
                .read(path)
                .ok()
                .flatten()
                .and_then(|bytes| TextureData::from_dds(path, bytes).ok())
        };
        let Some(made) = world::actor::MadeBodyTint::parse(reference) else {
            return read(reference);
        };
        if let Some(file) = read(&made.file) {
            return Some(file);
        }
        let bytes = self.assets.read(&assets::mesh_path(&made.egt)).ok()??;
        let egt = nif::Egt::parse(&bytes).ok()?;
        let rgb = egt.tint(&made.values);
        let pixels = rgb
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect();
        Some(TextureData::from_rgba(
            format!("{} made with {} values", made.egt, made.values.len()),
            egt.width as u32,
            egt.height as u32,
            pixels,
        ))
    }

    /// A texture from the game's files (or a head's with its NPC's face
    /// tint laid on: see `preview::actor::FACE_TINT`; without a tint file,
    /// just the base).
    pub(crate) fn get(&mut self, path: &str) -> Option<usize> {
        if let Some(&known) = self.index.get(path) {
            return known;
        }
        if let Some((tint, base)) = preview::actor::face_tint_parts(path) {
            let found = self.read(base).map(|b| {
                let tinted = self.tint(tint).and_then(|t| b.plus_face_tint(&t).ok());
                self.push(tinted.unwrap_or(b))
            });
            self.index.insert(path.to_string(), found);
            return found;
        }
        if let Some((layer, base)) = preview::actor::hair_layer_parts(path) {
            let found = self.read(base).map(|b| {
                let layered = self
                    .assets
                    .read(layer)
                    .ok()
                    .flatten()
                    .and_then(|bytes| TextureData::from_dds(layer, bytes).ok())
                    .and_then(|l| b.with_layer(&l).ok());
                self.push(layered.unwrap_or(b))
            });
            self.index.insert(path.to_string(), found);
            return found;
        }
        let found = self.read(path).map(|t| self.push(t));
        self.index.insert(path.to_string(), found);
        found
    }

    /// A texture holding data rather than colors (a normal map), kept apart
    /// from any color use of the same file.
    pub(crate) fn get_linear(&mut self, path: &str) -> Option<usize> {
        let key = format!("linear:{path}");
        if let Some(&known) = self.index.get(&key) {
            return known;
        }
        let found = self.read(path).map(|mut t| {
            t.linear = true;
            self.push(t)
        });
        self.index.insert(key, found);
        found
    }

    /// A cube map for reflections (sampled as stored): the file's six
    /// faces, or its one picture on every face.
    fn get_cube(&mut self, path: &str) -> Option<usize> {
        let key = format!("cube:{path}");
        if let Some(&known) = self.index.get(&key) {
            return known;
        }
        let found = self.read(path).and_then(|mut t| {
            t.linear = true;
            match t.into_cube() {
                Some(cube) => Some(self.push(cube)),
                None => {
                    self.unreadable
                        .push(format!("{path}: not square, so not usable as a cube map"));
                    None
                }
            }
        });
        self.index.insert(key, found);
        found
    }

    fn read(&mut self, path: &str) -> Option<TextureData> {
        let (real, bytes) = read_texture(self.assets, path)?;
        TextureData::from_dds(real, bytes)
            .map_err(|e| self.unreadable.push(format!("{path}: {e}")))
            .ok()
    }

    fn push(&mut self, texture: TextureData) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }
}

/// Reads a texture, correcting a wrong file extension as the game's own
/// files sometimes need (`chromedull_e.nif` for `.dds`).
pub(crate) fn read_texture(assets: &Assets, path: &str) -> Option<(String, Vec<u8>)> {
    let mut candidates = vec![path.to_string()];
    if !path.ends_with(".dds") {
        if let Some((stem, _)) = path.rsplit_once('.') {
            candidates.push(format!("{stem}.dds"));
        }
    }
    candidates
        .into_iter()
        .find_map(|p| assets.read(&p).ok().flatten().map(|bytes| (p, bytes)))
}

/// Where the camera starts: the first arrival point (the `coc` marker, else
/// a door), or the middle of the cell facing north.
fn start_point(scene: &CellScene) -> Start {
    if let Some(a) = scene.cell.arrivals.first() {
        let [x, y, z] = a.position;
        return Start {
            eye: [x, y, z + EYE_HEIGHT],
            heading: a.rotation[2],
            via: "the arrival point",
        };
    }
    let convention = RotationConvention::DEFAULT;
    let floor = scene.floor_levels(convention).first().map_or(0.0, |f| f.0);
    let center = scene
        .plan_bounds(convention, f32::INFINITY)
        .map_or([0.0; 2], |(lo, hi)| {
            [(lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5]
        });
    Start {
        eye: [center[0], center[1], floor + EYE_HEIGHT],
        heading: 0.0,
        via: "the middle of the cell",
    }
}

pub fn column_major(t: &nif::math::Transform) -> [f32; 16] {
    let r = &t.rotation;
    let s = t.scale;
    [
        r[0][0] * s,
        r[1][0] * s,
        r[2][0] * s,
        0.0,
        r[0][1] * s,
        r[1][1] * s,
        r[2][1] * s,
        0.0,
        r[0][2] * s,
        r[1][2] * s,
        r[2][2] * s,
        0.0,
        t.translation[0],
        t.translation[1],
        t.translation[2],
        1.0,
    ]
}

/// The blending a mesh's alpha settings amount to.
pub fn blend_of(alpha: &AlphaMode) -> Blend {
    let blend = match alpha.blend {
        None => Blend::Opaque,
        Some((BlendFactor::SrcAlpha | BlendFactor::One, BlendFactor::One)) => Blend::Add,
        Some((BlendFactor::DstColor, BlendFactor::Zero))
        | Some((BlendFactor::Zero, BlendFactor::SrcColor)) => Blend::Multiply,
        Some((BlendFactor::One, BlendFactor::InvSrcAlpha)) => Blend::Premultiplied,
        Some(_) => Blend::Blend,
    };
    let Some((test, threshold)) = alpha.test else {
        return blend;
    };
    let cutoff = f32::from(threshold) / 255.0;
    match (test, blend) {
        (AlphaTest::Always, _) => blend,
        (AlphaTest::Never, _) => Blend::Mask(2.0),
        (_, Blend::Opaque) => Blend::Mask(cutoff),
        (_, Blend::Blend) => Blend::MaskedBlend(cutoff),
        // Added or multiplied surfaces: the test only trims faint pixels,
        // which add or take away next to nothing anyway.
        (_, other) => other,
    }
}

fn mesh_data(
    name: &str,
    mesh: &ModelMesh,
    emissive: [f32; 3],
    texture: Option<usize>,
    glow: Option<usize>,
) -> MeshData {
    let n = mesh.positions.len();
    let mut triangles: Vec<[u16; 3]> = mesh
        .triangles
        .iter()
        .copied()
        .filter(|t| t.iter().all(|&i| usize::from(i) < n))
        .collect();
    let has_normals = mesh.normals.len() == n;
    if has_normals && !front_is_ccw(&mesh.positions, &mesh.normals, &triangles) {
        for t in &mut triangles {
            t.swap(1, 2);
        }
    }
    let normals = if has_normals {
        mesh.normals.iter().map(|&v| normalized(v)).collect()
    } else {
        smooth_normals(&mesh.positions, &triangles)
    };
    let uvs = if mesh.uvs.len() == n {
        mesh.uvs.clone()
    } else {
        vec![[0.0; 2]; n]
    };
    let colors = (mesh.colors.len() == n).then(|| {
        mesh.colors
            .iter()
            .map(|&[r, g, b, a]| [linear(r), linear(g), linear(b), a])
            .collect()
    });
    let base = if texture.is_some() || !mesh.lit {
        1.0
    } else {
        0.6
    };
    let tangents = (mesh.along_u.len() == n && mesh.along_v.len() == n && mesh.lit).then(|| {
        normals
            .iter()
            .zip(mesh.along_u.iter().zip(&mesh.along_v))
            .map(|(&normal, (&u, &v))| tangent_frame(normal, u, v))
            .collect()
    });
    MeshData {
        name: name.to_string(),
        shape_name: mesh.name.clone(),
        positions: mesh.positions.clone(),
        normals,
        tangents,
        uvs,
        colors,
        indices: triangles.iter().flatten().copied().collect(),
        material: MaterialData {
            texture,
            color: [base, base, base, mesh.opacity],
            blend: blend_of(&mesh.alpha),
            alpha_test: mesh
                .alpha
                .test
                .map(|(test, threshold)| (test, f32::from(threshold) / 255.0)),
            depth_test: mesh.depth_test,
            depth_write: mesh.depth_write,
            sort_center: mesh.bound_center,
            unlit: !mesh.lit,
            double_sided: mesh.double_sided,
            emissive,
            glow,
            decal: mesh.decal,
            normal_map: None,
            specular: mesh.specular,
            unlit_color: [1.0; 3],
            falloff: mesh.falloff,
            environment: None,
            emittance: None,
            shading: mesh.shading,
            hair_tint: mesh.hair_tint,
        },
        effect: mesh.effect,
        rig: None,
        motion: None,
        billboard: mesh.billboard,
        local_map: mesh.local_map,
        actor_part: mesh.actor_part,
        strips: mesh.strips,
    }
}

/// Whether a mesh flagged for specular gets its highlights, given whether
/// its normal map (if it has one) carries an alpha channel: the game draws
/// specular passes only for normal maps with alpha (the alpha is the
/// highlight mask). Read from the actors recording: every one of 438
/// specular passes in three frames samples a DXT5 normal map, and none of
/// the 35 lit draws with a DXT1 or X8R8G8B8 normal map got one (the
/// machete, `machete_n.dds` DXT1, specular flag on, is drawn in one pass
/// without highlights). Without a normal map the mask is taken as full (a
/// guess).
fn specular_allowed(normal_map_alpha: Option<bool>) -> bool {
    normal_map_alpha.unwrap_or(true)
}

/// Puts an actor piece's bind pose and bones into its mesh data, for
/// skinning: the vertices in the piece's own space instead of posed.
fn rig_into(data: &mut MeshData, rig: &preview::cell::Rig, lit: bool) {
    let n = rig.positions.len();
    if n != data.positions.len() {
        return;
    }
    data.positions = rig.positions.clone();
    let triangles: Vec<[u16; 3]> = data
        .indices
        .chunks_exact(3)
        .map(|t| [t[0], t[1], t[2]])
        .collect();
    data.normals = if rig.normals.len() == n {
        rig.normals.iter().map(|&v| normalized(v)).collect()
    } else {
        smooth_normals(&rig.positions, &triangles)
    };
    data.tangents = (rig.along_u.len() == n && rig.along_v.len() == n && lit).then(|| {
        data.normals
            .iter()
            .zip(rig.along_u.iter().zip(&rig.along_v))
            .map(|(&normal, (&u, &v))| tangent_frame(normal, u, v))
            .collect()
    });
    data.rig = Some(RigData {
        joints: rig
            .joints
            .iter()
            .map(|(bone, t)| (*bone, column_major(t)))
            .collect(),
        joint_indices: rig.weights.iter().map(|w| w.map(|x| x.0)).collect(),
        joint_weights: rig.weights.iter().map(|w| w.map(|x| x.1)).collect(),
        face: rig.face.clone(),
    });
}

/// A no-lighting surface's color (see [`MaterialData::unlit_color`]): the
/// placed object's Emittance color (0..1, `world::LoadedCell::
/// emittance_color`) times the glow multiplier for meshes marked for
/// external emittance, white otherwise.
fn unlit_color(
    external_emittance: bool,
    emissive_mult: f32,
    emittance: Option<[f32; 3]>,
) -> [f32; 3] {
    match emittance.filter(|_| external_emittance) {
        Some(c) => c.map(|v| v * emissive_mult),
        None => [1.0; 3],
    }
}

/// A vertex's normal-map frame as renderers take it: U's direction made
/// perpendicular to the normal, and in `w` the side V lies on
/// (V = w × (normal × U)).
fn tangent_frame(normal: [f32; 3], along_u: [f32; 3], along_v: [f32; 3]) -> [f32; 4] {
    let d = dot(along_u, normal);
    let u = normalized([0, 1, 2].map(|k| along_u[k] - normal[k] * d));
    let side = if dot(cross(normal, u), along_v) < 0.0 {
        -1.0
    } else {
        1.0
    };
    [u[0], u[1], u[2], side]
}

/// A gamma-encoded (sRGB) channel value as linear light. Values past 1
/// (hair tints, which brighten) follow the same curve.
pub fn linear(c: f32) -> f32 {
    let c = c.max(0.0);
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalized(v: [f32; 3]) -> [f32; 3] {
    let len = dot(v, v).sqrt();
    if len > 1e-12 {
        v.map(|c| c / len)
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// Whether the triangles are counter-clockwise seen from the side their
/// vertex normals point to (a vote, so a few odd triangles don't matter).
pub fn front_is_ccw(positions: &[[f32; 3]], normals: &[[f32; 3]], triangles: &[[u16; 3]]) -> bool {
    let mut votes = 0i64;
    for t in triangles {
        let [a, b, c] = t.map(usize::from);
        let face = cross(
            sub(positions[b], positions[a]),
            sub(positions[c], positions[a]),
        );
        let n = [0, 1, 2].map(|k| normals[a][k] + normals[b][k] + normals[c][k]);
        votes += if dot(face, n) >= 0.0 { 1 } else { -1 };
    }
    votes >= 0
}

/// Vertex normals averaged from the (counter-clockwise) faces around them,
/// weighted by area.
pub fn smooth_normals(positions: &[[f32; 3]], triangles: &[[u16; 3]]) -> Vec<[f32; 3]> {
    let mut sums = vec![[0.0f32; 3]; positions.len()];
    for t in triangles {
        let [a, b, c] = t.map(usize::from);
        let face = cross(
            sub(positions[b], positions[a]),
            sub(positions[c], positions[a]),
        );
        for i in [a, b, c] {
            for k in 0..3 {
                sums[i][k] += face[k];
            }
        }
    }
    sums.into_iter().map(normalized).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lights_near_a_square_edge_light_the_next_square_too() {
        let light = |x: f32, y: f32, radius: f32| LightData {
            position: [x, y, 0.0],
            color: [1.0; 3],
            radius,
            fade: 1.0,
        };
        // Square 0,0 spans 0..4096 both ways. One light inside it, one
        // 100 east of its east edge reaching 300, one 500 away reaching
        // 300, one far.
        let lights = [
            light(5000.0, 9000.0, 1000.0),
            light(4196.0, 2000.0, 300.0),
            light(2000.0, 2000.0, 300.0),
            light(4596.0, 2000.0, 300.0),
        ];
        let got = lights_reaching((0, 0), &lights, 64);
        let xs: Vec<f32> = got.iter().map(|l| l.position[0]).collect();
        assert_eq!(xs, [2000.0, 4196.0]);
        // At most `max`, nearest first.
        assert_eq!(lights_reaching((0, 0), &lights, 1)[0].position[0], 2000.0);
        // The east neighbour gets the edge light and the far one.
        let east: Vec<f32> = lights_reaching((1, 0), &lights, 64)
            .iter()
            .map(|l| l.position[0])
            .collect();
        assert_eq!(east, [4196.0, 4596.0]);
    }

    #[test]
    fn converts_vertex_colors_to_linear_light() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(1.0), 1.0);
        assert!((linear(0.5) - 0.214).abs() < 1e-3);
    }

    #[test]
    fn highlights_need_a_normal_map_with_alpha() {
        // The machete: a DXT1 normal map, no highlights.
        assert!(!specular_allowed(Some(false)));
        assert!(specular_allowed(Some(true)));
        assert!(specular_allowed(None));
    }

    #[test]
    fn colors_unlit_surfaces_as_the_game_does() {
        let sunlight = Some([255, 227, 170].map(|c: u8| f32::from(c) / 255.0));
        // The light beams: external emittance, glow multiplier 1.
        let beam = unlit_color(true, 1.0, sunlight);
        assert!((beam[1] - 0.890).abs() < 1e-3 && (beam[2] - 0.667).abs() < 1e-3);
        // A window panel with multiplier 10: (10, 8.9, 6.67).
        assert!((unlit_color(true, 10.0, sunlight)[0] - 10.0).abs() < 1e-5);
        // Not marked, or nothing to take the color from: white.
        assert_eq!(unlit_color(false, 10.0, sunlight), [1.0; 3]);
        assert_eq!(unlit_color(true, 10.0, None), [1.0; 3]);
    }

    #[test]
    fn a_linked_glow_takes_its_regions_color_now() {
        let region = FormId(0x9A0);
        let mut now = world::weather::EmittanceNow::default();
        now.regions.insert(region, [1.0, 0.5, 0.25]);
        let lamp = EmittanceLink {
            region: None,
            scale: 5.0,
            fallback: [3.785; 3],
        };
        // No weather region for the player: the lamp's own glow.
        assert_eq!(lamp.glow(&now), [3.785; 3]);
        now.player_region = Some([0.5, 0.25, 0.0]);
        assert_eq!(lamp.glow(&now), [2.5, 1.25, 0.0]);
        let beam = EmittanceLink {
            region: Some(region),
            scale: 1.0,
            fallback: [1.0; 3],
        };
        assert_eq!(beam.glow(&now), [1.0, 0.5, 0.25]);
    }

    #[test]
    fn builds_the_normal_map_frame_from_the_stored_directions() {
        let up = [0.0, 0.0, 1.0];
        // U along x, V along y: V = normal × U, so w is +1.
        assert_eq!(
            tangent_frame(up, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            [1.0, 0.0, 0.0, 1.0]
        );
        // V the other way (a mirrored texture): w is -1.
        assert_eq!(
            tangent_frame(up, [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
            [1.0, 0.0, 0.0, -1.0]
        );
        // U tilted out of the surface is laid back into it.
        let t = tangent_frame(up, [2.0, 0.0, 1.0], [0.0, 1.0, 0.0]);
        assert!((t[0] - 1.0).abs() < 1e-6 && t[2].abs() < 1e-6, "{t:?}");
    }

    #[test]
    fn flips_triangles_wound_the_other_way() {
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let up = [[0.0, 0.0, 1.0]; 3];
        let down = [[0.0, 0.0, -1.0]; 3];
        assert!(front_is_ccw(&positions, &up, &[[0, 1, 2]]));
        assert!(!front_is_ccw(&positions, &down, &[[0, 1, 2]]));
        let n = smooth_normals(&positions, &[[0, 1, 2]]);
        assert_eq!(n[0], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn a_scripts_group_turns_a_window_glow_off() {
        // The house window's glow card: "Right" fades its alpha 1 → 0,
        // "Left" 0 → 1 and doubles the glow; neither plays by itself.
        let fade = |name: &str, from: f32, to: f32, mult: f32| nif::Sequence {
            name: name.into(),
            start: 0.0,
            stop: 0.333,
            looping: false,
            tracks: Vec::new(),
            accum_root: None,
            text_keys: Vec::new(),
            materials: vec![
                nif::MaterialTrack {
                    node: "Object01:0".into(),
                    target: nif::MaterialTarget::Alpha,
                    keys: nif::MaterialKeys::Float(
                        [(0.0, from), (0.25, to)]
                            .map(|(time, value)| nif::FloatKey {
                                time,
                                value,
                                tangents: None,
                                hold: false,
                            })
                            .to_vec(),
                    ),
                },
                nif::MaterialTrack {
                    node: "Object01:0".into(),
                    target: nif::MaterialTarget::EmissiveMult,
                    keys: nif::MaterialKeys::Float(vec![nif::FloatKey {
                        time: 0.0,
                        value: mult,
                        tangents: None,
                        hold: false,
                    }]),
                },
            ],
        };
        let all = std::sync::Arc::new(vec![
            fade("Right", 1.0, 0.0, 2.0),
            fade("Left", 0.0, 1.0, 4.0),
        ]);
        let nodes = vec![("Object01:0".to_string(), nif::math::Transform::IDENTITY)];
        // An activator: no Open/Close, Idle or SpecialIdle: "Right"'s first
        // frame is held.
        let playing = preview::cell::placed_sequences(&all, true);
        let piece = PieceMotion {
            motion: MeshMotion::of(&nodes, &playing, &all).unwrap(),
            emissive_color: [0.25, 0.5, 1.0],
            emissive_mult: 2.0,
            emissive_scale: 1.5,
            opacity: 1.0,
        };
        let held = piece_now(&piece, 10.0, None);
        assert_eq!(held.opacity, Some(1.0));
        assert_eq!(held.emissive, Some([0.75, 1.5, 3.0]));
        // `PlayGroup Right` by day: gone once it has played.
        let off = piece_now(&piece, 10.0, Some(("right", 1.0)));
        assert_eq!(off.opacity, Some(0.0));
        let halfway = piece_now(&piece, 10.0, Some(("Right", 0.125)));
        assert_eq!(halfway.opacity, Some(0.5));
        // `PlayGroup Left` at night: lit, glowing twice as bright.
        let on = piece_now(&piece, 10.0, Some(("Left", 1.0)));
        assert_eq!(on.opacity, Some(1.0));
        assert_eq!(on.emissive, Some([1.5, 3.0, 6.0]));
        // A glow from the placed object's Emittance isn't changed.
        let external = PieceMotion {
            emissive_scale: 0.0,
            ..piece
        };
        assert_eq!(
            piece_now(&external, 0.0, Some(("Left", 1.0))).emissive,
            None
        );
    }

    #[test]
    fn maps_nif_alpha_to_blend_modes() {
        let mode = |flags: u16, threshold: u8| blend_of(&AlphaMode::from_nif(flags, threshold));
        assert_eq!(mode(0x00EC, 0), Blend::Opaque);
        assert_eq!(mode(0x00ED, 0), Blend::Blend);
        assert_eq!(mode(0x000D, 0), Blend::Add);
        assert_eq!(mode(0x12EC, 128), Blend::Mask(128.0 / 255.0));
        // Test and blend together: cut out, then blend the rest.
        assert_eq!(mode(0x12ED, 128), Blend::MaskedBlend(128.0 / 255.0));
        assert_eq!(mode(0x12ED, 8), Blend::MaskedBlend(8.0 / 255.0));
        // Tested additive glows stay additive.
        assert_eq!(mode(0x120D, 8), Blend::Add);
        // DstColor x source + Zero x destination: multiply.
        assert_eq!(mode(1 | (4 << 1) | (1 << 5), 0), Blend::Multiply);
    }

    #[test]
    fn column_major_matrices_place_points_like_the_transform() {
        let t = RotationConvention::DEFAULT.transform(
            [100.0, 0.0, 0.0],
            [0.0, 0.0, std::f32::consts::FRAC_PI_2],
            2.0,
        );
        let m = column_major(&t);
        let p = [0.0, 10.0, 5.0];
        let a = space::transform_point(&m, p);
        let b = t.apply_point(p);
        assert!(
            a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4),
            "{a:?} {b:?}"
        );
    }
}
