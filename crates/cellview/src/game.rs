//! The game's files opened once ([`Game`]), for loading one place after
//! another: interiors by name, and the squares of an outdoor worldspace
//! with their terrain.

use std::collections::HashMap;
use std::path::Path;

use assets::{archive_list_from, default_ini_candidates, Assets};
use esm::{FormId, LoadOrder};
use preview::cell::build_scene_with;
use world::land::{DEFAULT_DIFFUSE, DEFAULT_NORMAL};
use world::{Land, LandTexture, LoadedCell, WorldGrid};

use crate::{active_plugins, convert, Error, Options, TextureCache, TextureData, ViewerScene};

/// The game's plugins and archives, opened once. Shareable between threads,
/// so squares of the world can load in the background.
pub struct Game {
    pub order: LoadOrder,
    pub assets: Assets,
    keep_root_transforms: bool,
    /// The game's INI settings (`assets::default_settings_files`).
    pub settings: assets::IniSettings,
    /// Every folder's sound files ([`Game::sound_path`]), listed in one
    /// pass the first time it's needed ([`Game::warm_sound_folders`]):
    /// looking through every archived path for one folder took ~10 ms, once
    /// per gunshot.
    sound_folders: std::sync::OnceLock<HashMap<String, std::sync::Arc<[String]>>>,
}

/// One quarter of a cell's terrain, ready to draw (see
/// `world::land::TerrainMesh`): game units, world space.
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainData {
    pub name: String,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// For the normal maps, the game's frame (`world::land::TerrainMesh`):
    /// T along U (east on flat ground, `w` 1) and B along V (north).
    pub tangents: Vec<[f32; 4]>,
    pub binormals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// Vertex colours as stored, 0..1.
    pub colors: Vec<[f32; 3]>,
    /// How much of each texture shows: up to [`MAX_TERRAIN_TEXTURES`],
    /// zero past the last.
    pub weights: Vec<[f32; 8]>,
    pub indices: Vec<u16>,
    /// Each texture's diffuse and normal map (indices into
    /// [`ViewerScene::textures`]), base first.
    pub textures: Vec<(Option<usize>, Option<usize>)>,
    /// What the quarter fades into toward the loaded area's edge (see
    /// [`TerrainLodBlend`]); `None` where the worldspace has no distant
    /// land there.
    pub lod_blend: Option<TerrainLodBlend>,
}

/// The game's second pass over every loaded terrain quarter (recorded at
/// Goodsprings: a vertex and pixel shader not in package 13, alpha
/// blended, depth equal): the quarter drawn again with its distant-land
/// chunk's own texture and normal map, so the loaded terrain fades into
/// the distant land toward the edge of the loaded area. Its alpha is
/// `1 − saturate((LAND_BLEND_FULL − d) / LAND_BLEND_WIDTH)`, `d` the flat
/// distance from the middle of the player's cell (`LandBlendParams.zw`);
/// its texture coordinates are the quarter's own / 64 (a level-4 chunk is
/// eight quarters across) plus the quarter's corner in the chunk
/// (`LandBlendParams.xy`: `((cell − chunk) × 2 + quarter column or row) /
/// 8`), then `u` turned around (`1 − u`: the chunks' textures run east to
/// west, as their own meshes' coordinates do: their south-west corner is
/// at (1, 0)) and inset a texel (× 0.9921875 + 0.00390625). The colour is
/// the distant land's (`texture × (0.55 + 0.8 × noise at 1.75 × uv) ×
/// (ambient + sun × saturate(n · L))`), fogged.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainLodBlend {
    /// The chunk's diffuse texture and normal map (indices into
    /// [`ViewerScene::textures`], sampled as stored).
    pub diffuse: Option<usize>,
    pub normals: Option<usize>,
    /// The quarter's south-west corner in the chunk's texture coordinates
    /// before `u` is turned around (`LandBlendParams.xy`).
    pub offset: [f32; 2],
}

/// Where the blend toward the distant land is full, and how far it ramps
/// up before that, in game units from the middle of the player's cell
/// (constants of the recorded blend shader: 9625.6 and 1 / 0.0003756 =
/// 2662.4, i.e. 2.35 and 0.65 cells; with `uGridsToLoad` 5).
pub const LAND_BLEND_FULL: f32 = 9625.6;
pub const LAND_BLEND_WIDTH: f32 = 1.0 / 0.000_375_600_97;

/// A quarter's corner in its level-4 distant-land chunk's texture
/// (`LandBlendParams.xy`, recorded: cell -20,1's northern quarters at (0,
/// 0.375) and (0.125, 0.375) in chunk -20,0).
pub fn land_blend_offset(cell: (i32, i32), quarter: usize) -> [f32; 2] {
    let chunk = (
        cell.0.div_euclid(LOD_CHUNK_CELLS) * LOD_CHUNK_CELLS,
        cell.1.div_euclid(LOD_CHUNK_CELLS) * LOD_CHUNK_CELLS,
    );
    let per_side = (LOD_CHUNK_CELLS * 2) as f32;
    [
        ((cell.0 - chunk.0) * 2 + (quarter & 1) as i32) as f32 / per_side,
        ((cell.1 - chunk.1) * 2 + (quarter >> 1) as i32) as f32 / per_side,
    ]
}

/// The most textures the game's terrain shaders blend in one pass (the
/// seven groups `SLS2092`–`SLS2147`: base plus six layers).
pub const MAX_TERRAIN_TEXTURES: usize = 7;

/// The sky dome the game draws outdoors (`Meshes\Sky\Atmosphere.nif`, a
/// path written in its code), coloured for a weather. In game units around
/// the eye (about 500 across); triangles counter-clockwise seen from inside.
#[derive(Debug, Clone, PartialEq)]
pub struct SkyDome {
    pub positions: Vec<[f32; 3]>,
    /// Stored colours (0..1, a little over where the weights add past 1)
    /// and opacity.
    pub colors: Vec<[f32; 4]>,
    /// The model's own vertex colours (the weights [`sky_color`] blends
    /// the three sky colours by), for colouring it again as the hour
    /// changes.
    pub weights: Vec<[f32; 4]>,
    pub indices: Vec<u16>,
}

/// The dome model's path.
pub const SKY_DOME: &str = "meshes\\sky\\atmosphere.nif";

/// A block of distant objects (see [`Game::lod_objects`]).
pub struct LodBlock {
    /// Its south-west cell.
    pub cell: (i32, i32),
    /// One draw per segment, its reference the segment number.
    pub scene: ViewerScene,
    /// The cell each segment covers.
    pub cells: Vec<(i32, i32)>,
}

/// The night sky: the climate's star model (`MODL`, `Sky\Stars.nif`: a
/// sphere 32 units across textured `SkyStars.dds`, added onto the sky,
/// One + One). The game draws it with `SKYSTARS.vso` +
/// `SKYSHORIZFADE.pso`: colour = `BlendColor[0..2]` weighted by the vertex
/// colour (white here) × the texture, alpha = vertex alpha ×
/// `BlendColor[0].w` × `saturate((z − eye z) / 17)` (gone below the
/// horizon).
pub struct StarDome {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub texture: TextureData,
}

/// One of a weather's cloud layers, to draw as the game's cloud shaders
/// do (the cloud vertex shader with `SKYTEX.pso`): the layer's own shape
/// of `Meshes\Sky\Clouds.nif` (see `cloud_layer_mesh`; its vertices pure
/// red, their alpha fading toward the horizon) with the layer's texture;
/// colour `BlendColor[0]` (the red weight is 1) times the texture, alpha
/// the vertex alpha × `BlendColor[0].w` × the texture's; the texture
/// scrolled along V (`TexCoordYOff`); × `Params.y`. All read from the
/// Goodsprings recording: `BlendColor[0]` is the layer's `PNAM` colour
/// blended by the hour as the sky's are (`.w` 1), `Params.y` the sky's
/// brightness (`world::weather::sky_brightness`, 0.88 there), and
/// `Params.x` (the cross-fade to the next weather's texture) 0 without a
/// weather change.
pub struct CloudLayer {
    /// Game units around the eye, like the sky dome.
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub alphas: Vec<f32>,
    pub indices: Vec<u16>,
    pub texture: TextureData,
    /// Stored colour (0..1).
    pub color: [f32; 3],
    /// Texture lengths a second along V.
    pub scroll: f32,
    /// Which of the weather's four layers (its colours by time of day).
    pub layer: usize,
}

/// The sun: the climate's texture (`FNAM`, `Sky\Sun.dds`: a wide soft
/// halo around a small disk) and its glare (`GNAM`), each on a square
/// facing the eye, drawn with the sky's textured shaders (`SKYT.vso` +
/// `SKYTEX.pso`). From the game's code (`00641830`, `00640810`): both stand
/// where `world::weather::sun_at` puts the disc, squares of half-size
/// `[Weather] fSunBaseSize` and `fSunGlareSize` (750 and 800 in this
/// install's INI; engine defaults 250, 350), coloured by the weather's sun
/// colour (recorded at Goodsprings: `BlendColor[0]` (0.99928, 0.98995,
/// 0.96843), the day's and high noon's sun blended by the hour) × the
/// sky's brightness (`Params.y`); the disc's alpha is the sun's
/// visibility, the glare's the weather's glare byte / 255 × that. Not
/// done: the glare shrinking as the sun is hidden (an occlusion query; the
/// recording drew no glare with the sun overhead behind the camera).
pub struct SunSprite {
    pub texture: TextureData,
    pub glare: Option<TextureData>,
    /// Stored colour by day (0..1).
    pub color: [f32; 3],
    /// Half-sizes in sky units.
    pub half_size: f32,
    pub glare_half_size: f32,
    /// The weather's glare strength (`DATA` byte 4 / 255).
    pub glare_strength: f32,
}

/// How far the sun is drawn, in the sky dome's units (its radius): the
/// game's disc stands about 800 units out (`sun_at`), so its squares are
/// scaled to keep their size as seen.
pub const SUN_DISTANCE: f32 = 500.0;

/// The clouds model's path (named in the game's code, as the dome's is).
pub const CLOUDS: &str = "meshes\\sky\\clouds.nif";

/// The weather's empty cloud layer texture.
const NO_CLOUDS: &str = "sky\\alpha.dds";

/// One chunk of a worldspace's distant land: a pre-built model covering
/// `level` × `level` cells (a node of the game's quadtree, see
/// `world::lod`), with its own diffuse texture and normal map
/// (`meshes\landscape\lod\<world>\<world>.level<L>.x<X>.y<Y>.nif`, the
/// game's own name pattern).
pub struct LodChunk {
    /// The chunk's south-west cell.
    pub cell: (i32, i32),
    /// Cells per side (4, 8, 16 or 32 in the Mojave).
    pub level: u32,
    /// World space, game units.
    pub positions: Vec<[f32; 3]>,
    /// The height each vertex geomorphs toward (its texcoord1, from the
    /// model's `NiAdditionalGeometryData`), in the world like `positions`;
    /// the vertex's own height where the model has none. In every chunk the
    /// game ships these equal the vertices' own heights.
    pub morph_heights: Vec<f32>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    pub diffuse: Option<TextureData>,
    /// World-space normals (x east, y north, z up), stored as `0.5 + n / 2`.
    pub normals: Option<TextureData>,
}

/// Cells per side of a distant-land chunk at the level drawn (level 4).
pub const LOD_CHUNK_CELLS: i32 = 4;

/// The noise the distant-land shaders (and the terrain's blend toward
/// them) multiply the colour by: `0.55 + 0.8 × noise`, sampled at 1.75 ×
/// the chunk's texture coordinates. Confirmed by the Goodsprings
/// recording: the texture bound as `LODLandNoise` was uploaded at startup
/// as a 512² DXT1 with 10 levels whose first level is byte for byte this
/// file's.
pub const LOD_NOISE: &str = "textures\\effects\\noisemap.dds";

/// The dome's colour at a vertex. The game's sky shader (`SKY.vso`,
/// `SKY.pso`) gives `BlendColor[0] × red + BlendColor[1] × green +
/// BlendColor[2] × blue` of the vertex colour, alpha `vertex alpha ×
/// BlendColor[0].w`, then `× Params.y`. On the dome red covers the bottom
/// and the horizon, green a band just above it, and blue the top.
///
/// The three colours, from the game's code (`006335d0`, `00b89d80`):
/// `BlendColor[0]` = the weather's horizon, `[1]` = its lower sky, `[2]` =
/// its upper sky, alpha 1 (`sky` holds upper, horizon, lower); recorded
/// at Goodsprings exactly so. `Params.y` is the sky's brightness
/// (`world::weather::sky_brightness`: the image space's "LUM ramp no tex"
/// after its modifiers, 0.88 there), which the caller multiplies in.
pub fn sky_color(vertex: [f32; 4], sky: [[f32; 3]; 3]) -> [f32; 4] {
    let [upper, horizon, lower] = sky;
    let c = [0, 1, 2].map(|k| horizon[k] * vertex[0] + lower[k] * vertex[1] + upper[k] * vertex[2]);
    [c[0], c[1], c[2], vertex[3]]
}

/// Every `.wav` and `.ogg` path by its folder (up to and with its last
/// backslash), each folder's sorted.
fn sound_files_by_folder<'a>(
    paths: impl Iterator<Item = &'a str>,
) -> HashMap<String, std::sync::Arc<[String]>> {
    let mut by: HashMap<String, Vec<String>> = HashMap::new();
    for p in paths {
        if !(p.ends_with(".wav") || p.ends_with(".ogg")) {
            continue;
        }
        let Some(i) = p.rfind('\\') else {
            continue;
        };
        by.entry(p[..=i].to_string())
            .or_default()
            .push(p.to_string());
    }
    by.into_iter()
        .map(|(folder, mut files)| {
            files.sort();
            (folder, files.into())
        })
        .collect()
}

impl Game {
    pub fn open(data_dir: &Path, options: &Options) -> Result<Game, Error> {
        let order = LoadOrder::from_data_dir(data_dir, &active_plugins(options)?)?;
        let list = match &options.ini {
            Some(ini) => archive_list_from(std::slice::from_ref(ini)),
            None => archive_list_from(&default_ini_candidates(data_dir)),
        };
        let plugins: Vec<String> = order.plugins().iter().map(|p| p.name.clone()).collect();
        let mut files = assets::default_settings_files(data_dir);
        files.extend(options.ini.iter().cloned());
        let settings = assets::IniSettings::load(&files);
        let archive_settings = assets::ArchiveSettings::from_ini(&settings, list.names);
        let assets = Assets::open_with_settings(data_dir, &plugins, &archive_settings)?;
        Ok(Game {
            order,
            assets,
            keep_root_transforms: options.keep_root_transforms,
            settings,
            sound_folders: Default::default(),
        })
    }

    /// `[Landscape] fLandTextureTilingMult` (0 refused, as the game does;
    /// its built-in default 2).
    pub fn land_tiling(&self) -> f32 {
        self.settings
            .float("Landscape", "fLandTextureTilingMult")
            .filter(|&m| m != 0.0)
            .unwrap_or(world::land::DEFAULT_TILING)
    }

    /// A cell by editor ID, form ID or name.
    pub fn find_cell(&self, cell: &str) -> Result<FormId, Error> {
        let matches = world::find_cells(&self.order, cell)?;
        match matches.as_slice() {
            [] => Err(Error(format!("no cell matches '{cell}'"))),
            [one] => Ok(*one),
            many => {
                let names: Vec<String> = many
                    .iter()
                    .take(10)
                    .filter_map(|&id| self.order.get(id).map(|r| world::describe_record(&r)))
                    .collect();
                Err(Error(format!(
                    "'{cell}' matches {} cells; use an editor ID or form ID: {}",
                    many.len(),
                    names.join(", ")
                )))
            }
        }
    }

    /// An interior cell (or one exterior cell on its own, without terrain),
    /// as when it first loads.
    pub fn load_cell(&self, cell: FormId) -> Result<ViewerScene, Error> {
        self.load_cell_now(cell, &world::Disabled::new())
    }

    /// [`Self::load_cell`] with what scripts have enabled and disabled.
    pub fn load_cell_now(
        &self,
        cell: FormId,
        disabled: &world::Disabled,
    ) -> Result<ViewerScene, Error> {
        let loaded = world::load_cell_now(&self.order, cell, disabled)?;
        Ok(self.scene_of(loaded, None))
    }

    /// One actor on its own at the origin facing north (the first-person
    /// view), ready to draw; no lights of its own.
    pub fn actor_scene(&self, look: &world::ActorLook) -> ViewerScene {
        crate::convert_cell(
            &preview::cell::actor_scene(&self.assets, look),
            &self.assets,
        )
    }

    /// People on their own, where they're placed, ready to draw: those
    /// who come into a place after it loaded (through a door, or moved by
    /// a script). No lights of their own.
    pub fn actors_scene(&self, references: &[FormId]) -> ViewerScene {
        let actors = references
            .iter()
            .filter_map(|&r| world::placement_of(&self.order, r))
            .filter(|p| p.actor.is_some())
            .collect();
        self.scene_of(world::LoadedCell::actors_only(actors), None)
    }

    /// References made while playing (`PlaceAtMe`), ready to draw: people
    /// and creatures, and objects with a model or a light (markers left
    /// out). No lights of the place's own.
    pub fn made_scene(&self, placements: Vec<world::Placement>) -> ViewerScene {
        let (actors, objects): (Vec<_>, Vec<_>) =
            placements.into_iter().partition(|p| p.actor.is_some());
        let mut cell = world::LoadedCell::actors_only(actors);
        cell.objects = objects
            .into_iter()
            .filter(|p| p.model.is_some() || p.light.is_some() || !p.parts.is_empty())
            .filter(|p| !world::is_marker(p.base, p.base_type, p.model.as_deref()))
            .collect();
        self.scene_of(cell, None)
    }

    /// Placed references that start disabled and that scripts have enabled
    /// since the place loaded (the gas jets), ready to draw: as when a place
    /// loads, people, and objects with a model or a light, markers left out.
    pub fn placed_scene(&self, references: &[FormId]) -> ViewerScene {
        let (actors, objects): (Vec<_>, Vec<_>) = references
            .iter()
            .filter_map(|&r| world::placement_of(&self.order, r))
            .partition(|p| p.actor.is_some());
        let mut cell = world::LoadedCell::actors_only(actors);
        cell.objects = objects
            .into_iter()
            .filter(|p| p.model.is_some() || p.light.is_some() || !p.parts.is_empty())
            .filter(|p| !world::is_marker(p.base, p.base_type, p.model.as_deref()))
            .collect();
        self.scene_of(cell, None)
    }

    /// The sun of a worldspace's climate in a weather (see [`SunSprite`]).
    pub fn sun(&self, climate: &world::weather::Climate, weather: FormId) -> Option<SunSprite> {
        let path = assets::texture_path(climate.sun.as_deref()?);
        let bytes = self.assets.read(&path).ok()??;
        let texture = TextureData::from_dds(path, bytes).ok()?;
        let glare = climate.sun_glare.as_deref().and_then(|g| {
            let path = assets::texture_path(g);
            let bytes = self.assets.read(&path).ok()??;
            TextureData::from_dds(path, bytes).ok()
        });
        let w = world::weather::Weather::load(&self.order, weather)?;
        let color = w
            .color(
                world::weather::SkyColor::Sun,
                world::weather::TimeOfDay::Day,
            )
            .map_or([1.0; 3], |c| c.map(|v| f32::from(v) / 255.0));
        let setting =
            |key: &str, default: f32| self.settings.float("Weather", key).unwrap_or(default);
        Some(SunSprite {
            texture,
            glare,
            color,
            half_size: setting("fSunBaseSize", 250.0),
            glare_half_size: setting("fSunGlareSize", 350.0),
            glare_strength: f32::from(w.sun_glare) / 255.0,
        })
    }

    /// A climate's stars (see [`StarDome`]), or `None` when it names none or
    /// they can't be read.
    pub fn stars(&self, climate: &world::weather::Climate) -> Option<StarDome> {
        let path = assets::mesh_path(climate.stars.as_deref()?);
        let bytes = self.assets.read(&path).ok()??;
        let mesh = nif::Nif::parse(bytes)
            .ok()?
            .scene()
            .ok()?
            .meshes
            .into_iter()
            .next()?;
        let texture_path = assets::texture_path(mesh.diffuse_texture()?);
        let texture =
            TextureData::from_dds(texture_path.clone(), self.assets.read(&texture_path).ok()??)
                .ok()?;
        let positions: Vec<[f32; 3]> = mesh.model_positions().collect();
        let uvs = (0..positions.len())
            .map(|i| mesh.uvs.get(i).copied().unwrap_or([0.0, 0.0]))
            .collect();
        Some(StarDome {
            positions,
            uvs,
            indices: mesh.triangles.iter().flatten().copied().collect(),
            texture,
        })
    }

    /// A weather's cloud layers (see [`CloudLayer`]), lowest number first;
    /// empty layers (`sky\alpha.dds`) and unreadable ones left out.
    pub fn clouds(&self, weather: FormId) -> Vec<CloudLayer> {
        let Some(w) = world::weather::Weather::load(&self.order, weather) else {
            return Vec::new();
        };
        let meshes = self
            .assets
            .read(CLOUDS)
            .ok()
            .flatten()
            .and_then(|b| nif::Nif::parse(b).ok())
            .and_then(|n| n.scene().ok())
            .map(|s| s.meshes)
            .unwrap_or_default();
        let mut out = Vec::new();
        for (layer, texture) in w.cloud_textures.iter().enumerate() {
            let Some(texture) = texture.as_deref() else {
                continue;
            };
            if texture.eq_ignore_ascii_case(NO_CLOUDS) {
                continue;
            }
            let Some(dome) = cloud_layer_mesh(&meshes, layer) else {
                continue;
            };
            let positions: Vec<[f32; 3]> = dome.model_positions().collect();
            let alphas: Vec<f32> = (0..positions.len())
                .map(|i| dome.colors.get(i).map_or(1.0, |c| c[3]))
                .collect();
            let uvs = (0..positions.len())
                .map(|i| dome.uvs.get(i).copied().unwrap_or([0.0, 0.0]))
                .collect::<Vec<_>>();
            let indices: Vec<u16> = dome.triangles.iter().flatten().copied().collect();
            let path = assets::texture_path(texture);
            let Some(data) = self
                .assets
                .read(&path)
                .ok()
                .flatten()
                .and_then(|b| TextureData::from_dds(path.clone(), b).ok())
            else {
                continue;
            };
            let color = w
                .cloud_color(layer, world::weather::TimeOfDay::Day)
                .map_or([1.0; 3], |c| c.map(|v| f32::from(v) / 255.0));
            out.push(CloudLayer {
                positions,
                uvs,
                alphas,
                indices,
                texture: data,
                color,
                // `00634110`: wind × `fWeatherCloudSpeedMax` × the layer's
                // speed / 255 texture lengths a second. (Recorded: layers 0
                // and 3 of `NVWastelandGS`, speeds 52 and 65, stood at
                // offsets 0.232 and 0.29, in the ratio of their speeds.)
                scroll: f32::from(w.wind) / 255.0
                    * world::weather::SkySettings::load(&self.order).cloud_speed_max
                    * f32::from(w.cloud_speeds[layer])
                    / 255.0,
                layer,
            });
        }
        out
    }

    /// A worldspace by editor ID, form ID or name, with its grid of cells.
    pub fn world(&self, query: &str) -> Result<WorldGrid, Error> {
        let id = world::find_worldspace(&self.order, query)?
            .ok_or_else(|| Error(format!("no worldspace matches '{query}'")))?;
        Ok(WorldGrid::load(&self.order, id)?)
    }

    /// The sky dome coloured with a weather's sky colours by day (upper,
    /// horizon, lower; stored values 0..1), or `None` when the model can't
    /// be read.
    pub fn sky_dome(&self, sky: [[f32; 3]; 3]) -> Option<SkyDome> {
        let bytes = self.assets.read(SKY_DOME).ok()??;
        let scene = nif::Nif::parse(bytes).ok()?.scene().ok()?;
        let mesh = scene
            .meshes
            .into_iter()
            .find(|m| m.colors.len() == m.positions.len())?;
        let positions: Vec<[f32; 3]> = mesh.model_positions().collect();
        let colors = mesh.colors.iter().map(|&c| sky_color(c, sky)).collect();
        Some(SkyDome {
            positions,
            colors,
            weights: mesh.colors.clone(),
            indices: mesh.triangles.iter().flatten().copied().collect(),
        })
    }

    /// A block of a worldspace's distant objects (level 4: buildings and
    /// other things marked to be seen from afar, merged by the game's tools
    /// into one mesh per 4 × 4 cells with a shared texture atlas), by its
    /// south-west cell. Each draw's reference is its segment number
    /// (`preview::cell::lod_block_scene`), and `cells[i]` the cell segment
    /// `i` covers: the segments go north first, then east (`x + i / 4,
    /// y + i % 4`; checked on `wastelandnv.level4.x-20.y0.nif`, each
    /// segment's middle in that cell). `None` where the game has none.
    pub fn lod_objects(&self, world: &str, cell: (i32, i32)) -> Option<LodBlock> {
        self.lod_object_block(
            world,
            world::lod::ObjectBlock {
                node: world::lod::LodNode {
                    level: LOD_CHUNK_CELLS as u32,
                    x: cell.0,
                    y: cell.1,
                },
                high: false,
            },
        )
    }

    /// A block of distant objects as `world::lod::object_blocks` picks it:
    /// its ordinary model, or past `fBlockLoadDistanceLow` its "high" one
    /// (`...\blocks\<world>.level4.high.x<X>.y<Y>.nif`: only the tallest
    /// landmarks). `None` where the game has no such model.
    pub fn lod_object_block(
        &self,
        world: &str,
        block: world::lod::ObjectBlock,
    ) -> Option<LodBlock> {
        let path = block.node.object_model(world, block.high);
        let cell = (block.node.x, block.node.y);
        let scene = preview::cell::lod_block_scene(&self.assets, &path)?;
        let scene = crate::convert_cell(&scene, &self.assets);
        let cells = (0..16).map(|i| (cell.0 + i / 4, cell.1 + i % 4)).collect();
        Some(LodBlock { cell, scene, cells })
    }

    /// A chunk of a worldspace's distant land (level 4), by its south-west
    /// cell (a multiple of 4). `None` where the game has none.
    pub fn lod_chunk(&self, world: &str, cell: (i32, i32)) -> Option<LodChunk> {
        self.lod_land(
            world,
            world::lod::LodNode {
                level: LOD_CHUNK_CELLS as u32,
                x: cell.0,
                y: cell.1,
            },
        )
    }

    /// A worldspace's distant-land quadtree (`lodsettings\<world>
    /// .dlodsettings`); `None` where it has none (no distant land).
    pub fn lod_settings(&self, world: &str) -> Option<world::lod::LodSettings> {
        let bytes = self.assets.read(&world::lod::settings_path(world)).ok()??;
        world::lod::LodSettings::parse(&bytes)
    }

    /// The terrain manager's INI settings (`[TerrainManager]`).
    pub fn terrain_settings(&self) -> world::lod::TerrainSettings {
        world::lod::TerrainSettings::from_ini(|section, key| self.settings.float(section, key))
    }

    /// One node's distant-land chunk, any level. `None` where the game has
    /// none.
    pub fn lod_land(&self, world: &str, node: world::lod::LodNode) -> Option<LodChunk> {
        let nif = nif::Nif::parse(self.assets.read(&node.land_model(world)).ok()??).ok()?;
        // Not placed by a reference: the top node's transform (kept by
        // `scene`, unlike `placed_scene`) puts the chunk in the world.
        let scene = nif.scene().ok()?;
        let mesh = scene.meshes.into_iter().next()?;
        let positions: Vec<[f32; 3]> = mesh.model_positions().collect();
        let morph = nif
            .additional_geometry()
            .into_iter()
            .find_map(|(_, g)| g.channels.first().and_then(|c| c.floats()))
            .filter(|m| m.len() == mesh.positions.len());
        let morph_heights = match morph {
            Some(m) => mesh
                .positions
                .iter()
                .zip(m)
                .map(|(p, h)| mesh.transform.apply_point([p[0], p[1], h])[2])
                .collect(),
            None => positions.iter().map(|p| p[2]).collect(),
        };
        let texture = |slot: usize| {
            let path = lod_texture_path(mesh.textures.get(slot)?)?;
            let bytes = self.assets.read(&path).ok()??;
            TextureData::from_dds(path, bytes).ok()
        };
        let mut normals = texture(1);
        if let Some(n) = normals.as_mut() {
            n.linear = true;
        }
        Some(LodChunk {
            cell: (node.x, node.y),
            level: node.level,
            positions,
            morph_heights,
            uvs: mesh.uvs.clone(),
            indices: mesh.triangles.iter().flatten().copied().collect(),
            diffuse: texture(0),
            normals,
        })
    }

    /// The diffuse texture and normal map of the level-4 distant-land
    /// chunk holding `cell` (the paths its model names), if there is one.
    pub fn lod_chunk_textures(&self, world: &str, cell: (i32, i32)) -> Option<[Option<String>; 2]> {
        let chunk = (
            cell.0.div_euclid(LOD_CHUNK_CELLS) * LOD_CHUNK_CELLS,
            cell.1.div_euclid(LOD_CHUNK_CELLS) * LOD_CHUNK_CELLS,
        );
        let bytes = self.assets.read(&lod_chunk_path(world, chunk)).ok()??;
        let scene = nif::Nif::parse(bytes).ok()?.scene().ok()?;
        let mesh = scene.meshes.into_iter().next()?;
        Some([0, 1].map(|slot| mesh.textures.get(slot).and_then(|p| lod_texture_path(p))))
    }

    /// The noise texture distant land is drawn with (see [`LOD_NOISE`]).
    pub fn lod_noise(&self) -> Option<TextureData> {
        let bytes = self.assets.read(LOD_NOISE).ok()??;
        let mut t = TextureData::from_dds(LOD_NOISE.to_string(), bytes).ok()?;
        t.linear = true;
        Some(t)
    }

    /// A sound record's file: its path and bytes. A sound naming a folder
    /// plays one of the files in it; `pick` chooses which (the game picks at
    /// random).
    pub fn sound_file(&self, sound: &world::sound::Sound, pick: u64) -> Option<(String, Vec<u8>)> {
        let path = self.sound_path(sound, pick)?;
        let bytes = self.assets.read(&path).ok()??;
        Some((path, bytes))
    }

    /// The file [`Game::sound_file`] plays for a sound record: one of a
    /// folder's files, picked by `pick`, else the file named, else the
    /// `.ogg` of the same name (many records name a `.wav` that the game
    /// ships as an `.ogg`: Goodsprings' interior loop,
    /// `amb_gsinteriorloop.wav`, and the game plays that).
    pub fn sound_path(&self, sound: &world::sound::Sound, pick: u64) -> Option<String> {
        let path = if sound.is_folder() {
            let folder = format!("{}\\", sound.file.trim_end_matches('\\'));
            let files = self.sound_folder(&folder);
            if files.is_empty() {
                return None;
            }
            files[(pick % files.len() as u64) as usize].clone()
        } else {
            sound.file.clone()
        };
        if self.assets.contains(&path) {
            return Some(path);
        }
        let ogg = format!("{}.ogg", path.strip_suffix(".wav")?);
        self.assets.contains(&ogg).then_some(ogg)
    }

    /// The `.wav` and `.ogg` files directly in a sound folder (`folder`
    /// ends with its backslash), sorted.
    fn sound_folder(&self, folder: &str) -> std::sync::Arc<[String]> {
        self.sound_folders
            .get_or_init(|| sound_files_by_folder(self.assets.paths()))
            .get(folder)
            .cloned()
            .unwrap_or_else(|| std::sync::Arc::from(Vec::new()))
    }

    /// Lists every folder's sound files now (a background thread can do it
    /// before the first sound plays).
    pub fn warm_sound_folders(&self) {
        self.sound_folder("");
    }

    /// The worldspace a cell belongs to, if it's an exterior one.
    pub fn world_of_cell(&self, cell: FormId) -> Option<FormId> {
        let rr = self.order.get(cell)?;
        self.order.world_of(&rr)
    }

    /// One square of a worldspace: its objects, terrain and collision.
    /// `None` where the worldspace has no cell.
    pub fn load_square(
        &self,
        grid: &WorldGrid,
        square: (i32, i32),
    ) -> Result<Option<ViewerScene>, Error> {
        self.load_square_now(grid, square, &world::Disabled::new())
    }

    /// [`Self::load_square`] with what scripts have enabled and disabled.
    pub fn load_square_now(
        &self,
        grid: &WorldGrid,
        square: (i32, i32),
        disabled: &world::Disabled,
    ) -> Result<Option<ViewerScene>, Error> {
        let Some(loaded) = grid.load_square_now(&self.order, square, disabled)? else {
            return Ok(None);
        };
        let land = grid.land(&self.order, square)?;
        let origin = [
            square.0 as f32 * world::land::CELL_SIZE,
            square.1 as f32 * world::land::CELL_SIZE,
        ];
        let world = grid.world.editor_id.as_deref();
        Ok(Some(self.scene_of(
            loaded,
            land.map(|l| (l, origin, world.map(|w| (w, square)))),
        )))
    }

    /// The game's water switches (see [`crate::WaterSettings`]).
    pub fn water_settings(&self) -> crate::WaterSettings {
        crate::WaterSettings::from_ini(&self.settings)
    }

    /// `land`: a square's terrain, its south-west corner, and the
    /// worldspace's editor ID with the square (for the blend toward the
    /// distant land).
    fn scene_of(&self, loaded: LoadedCell, land: Option<SquareLand<'_>>) -> ViewerScene {
        let teleports: Vec<world::Teleport> =
            loaded.objects.iter().filter_map(|o| o.teleport).collect();
        let water = crate::water::WaterSource::of(
            &self.order,
            &loaded,
            land.as_ref().map(|(l, o, _)| (l, *o)),
        );
        let scene = build_scene_with(&self.assets, loaded, self.keep_root_transforms);
        let mut cache = TextureCache::new(&self.assets);
        let mut viewer = convert(&scene, &mut cache);
        viewer.water = water.build(
            &self.order,
            &self.assets,
            &self.water_settings(),
            &mut cache,
            &mut viewer.notes,
        );
        if let Some((land, origin, lod)) = land {
            let terrain = self.terrain(&land, origin, lod, &mut cache);
            for t in &terrain {
                // The ground stands out from the terrain's triangles by the
                // game's height-field radius (physics::TERRAIN_SHELL), and
                // rubs as the land body does (physics::LAND_SURFACE).
                viewer.collision.add_solid_surface(
                    &t.positions,
                    &triangles(&t.indices),
                    (physics::TERRAIN_SHELL, 0, physics::NO_MATERIAL),
                    Some(physics::LAND_SURFACE),
                );
            }
            viewer.notes.push(format!(
                "terrain: {} quarters, heights {}",
                terrain.len(),
                land.heights
                    .as_ref()
                    .map(|h| {
                        let lo = h.iter().copied().fold(f32::MAX, f32::min);
                        let hi = h.iter().copied().fold(f32::MIN, f32::max);
                        format!("{lo:.0} to {hi:.0}")
                    })
                    .unwrap_or_else(|| "missing".into())
            ));
            viewer.terrain = terrain;
        }
        let (textures, unreadable) = cache.finish();
        viewer.textures = textures;
        for line in unreadable {
            viewer.notes.push(format!("unreadable texture: {line}"));
        }
        // Where each load door leads: the cell holding its destination door.
        for (door, teleport) in viewer.doors.iter_mut().zip(teleports) {
            let Some(record) = self.order.get(teleport.door) else {
                continue;
            };
            if let Some(cell) = self.order.cell_of(&record) {
                door.cell = Some(cell.0);
                door.world = self.order.world_of(&record).map(|w| w.0);
                if let Ok(info) = world::cell_info(&self.order, cell) {
                    door.interior = info.flags & world::CELL_INTERIOR != 0;
                    door.cell_label = match (door.interior, door.world) {
                        (false, Some(w)) => world::Worldspace::load(&self.order, FormId(w))
                            .map(|w| w.label())
                            .unwrap_or_else(|_| info.label()),
                        _ => info.label(),
                    };
                }
            }
        }
        viewer
    }

    /// A cell's terrain, one mesh per quarter, with its textures loaded.
    fn terrain(
        &self,
        land: &Land,
        origin: [f32; 2],
        lod: Option<(&str, (i32, i32))>,
        cache: &mut TextureCache<'_>,
    ) -> Vec<TerrainData> {
        let mut land_textures: HashMap<FormId, Option<LandTexture>> = HashMap::new();
        // The distant-land chunk's textures, for the blend at the loaded
        // area's edge.
        let lod_textures = lod.and_then(|(world, square)| {
            let [diffuse, normals] = self.lod_chunk_textures(world, square)?;
            Some((
                square,
                diffuse.and_then(|p| cache.get(&p)),
                normals.and_then(|p| cache.get_linear(&p)),
            ))
        });
        let mut out = Vec::new();
        for quarter in 0..4 {
            let Some(mesh) = land.quarter_mesh_tiled(quarter, origin, self.land_tiling()) else {
                continue;
            };
            let mut textures = Vec::new();
            for id in mesh.textures.iter().take(MAX_TERRAIN_TEXTURES) {
                let files = id.and_then(|id| {
                    land_textures
                        .entry(id)
                        .or_insert_with(|| LandTexture::load(&self.order, id))
                        .clone()
                });
                let diffuse = files
                    .as_ref()
                    .and_then(|t| t.diffuse.clone())
                    .unwrap_or_else(|| DEFAULT_DIFFUSE.into());
                let normal = files
                    .as_ref()
                    .and_then(|t| t.normal.clone())
                    .unwrap_or_else(|| DEFAULT_NORMAL.into());
                textures.push((
                    cache.get(&texture_path(&diffuse)),
                    cache.get_linear(&texture_path(&normal)),
                ));
            }
            let weights = mesh
                .weights
                .iter()
                .map(|w| {
                    let mut out = [0.0; 8];
                    for (o, v) in out.iter_mut().zip(w.iter().take(MAX_TERRAIN_TEXTURES)) {
                        *o = *v;
                    }
                    out
                })
                .collect();
            let tangents = mesh
                .tangents
                .iter()
                .map(|t| [t[0], t[1], t[2], 1.0])
                .collect();
            out.push(TerrainData {
                name: format!("terrain {} quarter {quarter}", land.form_id),
                positions: mesh.positions,
                normals: mesh.normals,
                tangents,
                binormals: mesh.binormals,
                uvs: mesh.uvs,
                colors: mesh.colors,
                weights,
                indices: mesh.indices,
                textures,
                lod_blend: lod_textures.map(|(square, diffuse, normals)| TerrainLodBlend {
                    diffuse,
                    normals,
                    offset: land_blend_offset(square, quarter),
                }),
            });
        }
        out
    }
}

/// A square's terrain, its south-west corner (game units), and the
/// worldspace's editor ID with the square, when there's distant land to
/// blend toward.
type SquareLand<'a> = (Land, [f32; 2], Option<(&'a str, (i32, i32))>);

/// A level-4 distant-land chunk's model, by its south-west cell (the
/// game's own name pattern).
fn lod_chunk_path(world: &str, chunk: (i32, i32)) -> String {
    format!(
        "meshes\\landscape\\lod\\{w}\\{w}.level4.x{}.y{}.nif",
        chunk.0,
        chunk.1,
        w = world.to_ascii_lowercase()
    )
}

/// A distant-land model's texture path as the asset lookup takes it (the
/// models write `Data\Textures\...`).
fn lod_texture_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return None;
    }
    let lower = path.replace('/', "\\").to_ascii_lowercase();
    Some(lower.strip_prefix("data\\").unwrap_or(&lower).to_string())
}

/// The shape a weather's cloud layer is drawn on: `Clouds.nif`'s shapes in
/// file order, one per layer (`CloudDome` for layer 0, `HorizonLayerClear`
/// 1, `HorizonLayerOvercast` 2, `LowerLayer` 3), the first when the model
/// has fewer. Recorded at Goodsprings: the game drew the four layers on
/// shapes of 289, 84, 96 and 341 vertices, in that order, each with its
/// own layer's texture and `PNAM` colour (`NVWastelandGS`'s only real
/// layer, 3, `sky\NVCloudlight.dds`, on `LowerLayer`).
fn cloud_layer_mesh<T>(meshes: &[T], layer: usize) -> Option<&T> {
    meshes.get(layer).or_else(|| meshes.first())
}

/// A texture set's path (relative to `textures\`) as the asset lookup
/// takes it.
fn texture_path(path: &str) -> String {
    let lower = path.replace('/', "\\").to_ascii_lowercase();
    if lower.starts_with("textures\\") {
        lower
    } else {
        format!("textures\\{lower}")
    }
}

fn triangles(indices: &[u16]) -> Vec<[u32; 3]> {
    indices
        .chunks_exact(3)
        .map(|t| [u32::from(t[0]), u32::from(t[1]), u32::from(t[2])])
        .collect()
}

#[cfg(test)]
mod tests {
    /// The folder index lists what scanning every path for one folder
    /// listed: the folder's own sound files, sorted, none from subfolders
    /// or of other kinds.
    #[test]
    fn sound_folders_list_what_a_scan_finds() {
        let paths = [
            "sound\\fx\\wpn\\9mm\\fire\\b.wav",
            "sound\\fx\\wpn\\9mm\\fire\\a.wav",
            "sound\\fx\\wpn\\9mm\\fire\\c.ogg",
            "sound\\fx\\wpn\\9mm\\fire\\old\\d.wav",
            "sound\\fx\\wpn\\9mm\\fire\\notes.txt",
            "sound\\fx\\wpn\\9mm\\e.wav",
            "readme.wav",
        ];
        let index = super::sound_files_by_folder(paths.iter().copied());
        let scan = |folder: &str| -> Vec<String> {
            let mut v: Vec<String> = paths
                .iter()
                .filter(|p| {
                    p.starts_with(folder)
                        && !p[folder.len()..].contains('\\')
                        && (p.ends_with(".wav") || p.ends_with(".ogg"))
                })
                .map(|p| p.to_string())
                .collect();
            v.sort();
            v
        };
        for folder in [
            "sound\\fx\\wpn\\9mm\\fire\\",
            "sound\\fx\\wpn\\9mm\\",
            "sound\\fx\\wpn\\9mm\\fire\\old\\",
            "sound\\nothing\\",
        ] {
            let got: Vec<String> = index.get(folder).map(|f| f.to_vec()).unwrap_or_default();
            assert_eq!(got, scan(folder), "{folder}");
        }
    }

    use super::*;

    #[test]
    fn the_sky_blends_horizon_lower_and_upper_by_the_vertex_colour() {
        // Upper, horizon, lower.
        let sky = [[0.2, 0.3, 0.5], [0.9, 0.9, 0.9], [0.4, 0.5, 0.6]];
        // All red: the horizon (`BlendColor[0]`); green: the lower sky;
        // blue: the upper.
        assert_eq!(sky_color([1.0, 0.0, 0.0, 0.5], sky), [0.9, 0.9, 0.9, 0.5]);
        assert_eq!(sky_color([0.0, 1.0, 0.0, 1.0], sky), [0.4, 0.5, 0.6, 1.0]);
        assert_eq!(sky_color([0.0, 0.0, 1.0, 1.0], sky), [0.2, 0.3, 0.5, 1.0]);
        let c = sky_color([0.5, 0.5, 0.0, 1.0], sky);
        assert!((c[0] - 0.65).abs() < 1e-6, "{c:?}");
    }

    #[test]
    fn quarters_find_their_place_in_the_distant_land_texture() {
        // Recorded `LandBlendParams.xy`: cell -20,1's north-west and
        // north-east quarters in chunk -20,0; cell -18,2's south-west one;
        // cell -16,1's north-west one in chunk -16,0.
        assert_eq!(land_blend_offset((-20, 1), 2), [0.0, 0.375]);
        assert_eq!(land_blend_offset((-20, 1), 3), [0.125, 0.375]);
        assert_eq!(land_blend_offset((-18, 2), 0), [0.5, 0.5]);
        assert_eq!(land_blend_offset((-16, 1), 2), [0.0, 0.375]);
        assert_eq!(land_blend_offset((-17, 2), 3), [0.875, 0.625]);
        // The blend's distances: full at 2.35 cells, over 0.65 of one.
        assert!((LAND_BLEND_FULL / 4096.0 - 2.35).abs() < 1e-4);
        assert!((LAND_BLEND_WIDTH / 4096.0 - 0.65).abs() < 1e-4);
    }

    #[test]
    fn each_cloud_layer_has_its_own_shape() {
        let shapes = [
            "CloudDome:0",
            "HorizonLayerClear:1",
            "HorizonLayerOvercast:1",
            "LowerLayer:0",
        ];
        assert_eq!(cloud_layer_mesh(&shapes, 0), Some(&"CloudDome:0"));
        assert_eq!(cloud_layer_mesh(&shapes, 3), Some(&"LowerLayer:0"));
        assert_eq!(cloud_layer_mesh(&shapes[..1], 3), Some(&"CloudDome:0"));
        assert_eq!(cloud_layer_mesh::<&str>(&[], 0), None);
    }

    #[test]
    fn terrain_textures_are_found_under_textures() {
        assert_eq!(
            texture_path("Landscape\\DirtWastes01.dds"),
            "textures\\landscape\\dirtwastes01.dds"
        );
        assert_eq!(
            texture_path("textures/landscape/a.dds"),
            "textures\\landscape\\a.dds"
        );
    }
}
