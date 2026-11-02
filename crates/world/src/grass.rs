//! Grass (`GRAS`): the small plants the game scatters over the terrain
//! wherever a land texture that lists them (`LTEX` `GNAM`) is painted,
//! worked out the way the game's code does it (`FalloutNV.exe`; each rule
//! names the function it was read from).
//!
//! When a square's terrain loads, the game divides each quarter into
//! 512-unit spots and notes, for every grass of every texture painted
//! there, a 3 × 3 grid of densities ([`land_spots`], `0053bc10`). Around the
//! player it then fills the spots: a 6 × 6 grid of candidates per spot, each
//! kept with the density at its place, jittered, dropped onto the terrain
//! and checked against the grass's water and slope limits ([`fill_spot`],
//! `0057de00`). Each kept one gets a brightness and a size from a generator
//! seeded by its own position ([`instance_data`], `00b62de0`), and the
//! grass shader (`GRASS2002.vso`, ported in the viewer's `grass.wgsl`)
//! does the rest: it turns the model to the slope, sways it in the wind,
//! lights it and fades it out with distance.
//!
//! **A guess**: the game draws the candidates' random numbers from one
//! generator seeded from the clock at start-up (`00aa5230`, a Mersenne
//! twister), so where each blade lands differs every time the game runs.
//! Here the same kind of generator is seeded per spot and grass
//! ([`spot_seed`]), so a spot always comes out the same; the counts and
//! the spread match the game's on average, not blade for blade. The
//! brightness and size are exact: the game seeds those from each blade's
//! position.

use std::collections::{HashMap, HashSet};
use std::f32::consts::TAU;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{cell_info, le_f32, le_u32, CELL_HAS_WATER, CELL_INTERIOR};
use crate::exterior::WorldGrid;
use crate::image_space::ImageSpace;
use crate::land::{Land, Quarter, CELL_SIZE, GRID, QUARTER_GRID, SPACING};

const GRAS: FourCC = FourCC::new(b"GRAS");
const LTEX: FourCC = FourCC::new(b"LTEX");
const GNAM: FourCC = FourCC::new(b"GNAM");
const MODL: FourCC = FourCC::new(b"MODL");

/// A grass record.
#[derive(Debug, Clone, PartialEq)]
pub struct Grass {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `MODL`, as stored (relative to `meshes\`).
    pub model: Option<String>,
    /// The chance in percent that each candidate place gets a blade.
    pub density: u8,
    /// Slope limits in degrees from flat.
    pub min_slope: u8,
    pub max_slope: u8,
    /// How far from the water level the water rule measures.
    pub units_from_water: u16,
    /// 0 above (at least), 1 above (at most), 2 below (at least), 3 below
    /// (at most), 4 either (at least), 5 either (at most); anything else
    /// isn't tested (see [`Grass::water_allows`]).
    pub water_rule: u32,
    /// How far each blade is moved from its place at random, in units.
    pub position_range: f32,
    /// How much a blade's size varies (a fraction either way).
    pub height_range: f32,
    /// How much a blade's brightness varies (see [`instance_data`]).
    pub color_range: f32,
    /// How many times the wind sways it back and forth per game hour.
    pub wave_period: f32,
    /// 0x01 lit by its own normals, 0x02 scaled the same on every axis,
    /// 0x04 turned to the slope.
    pub flags: u8,
}

/// `DATA` is 32 bytes: density u8, min and max slope u8, unused u8, units
/// from water u16, unused u16, water rule u32, position range f32, height
/// range f32, colour range f32, wave period f32, flags u8, three unused.
/// The game copies it over the form byte for byte (`00509f80`), so a short
/// one leaves the defaults after it (`00509ee0`: 30, 0, 90, 0, 0, 32, 0.2,
/// 0.5, 10, no flags).
const DATA_SIZE: usize = 32;

fn default_data() -> [u8; DATA_SIZE] {
    let mut d = [0u8; DATA_SIZE];
    d[0] = 30;
    d[2] = 90;
    d[12..16].copy_from_slice(&32f32.to_le_bytes());
    d[16..20].copy_from_slice(&0.2f32.to_le_bytes());
    d[20..24].copy_from_slice(&0.5f32.to_le_bytes());
    d[24..28].copy_from_slice(&10f32.to_le_bytes());
    d
}

impl Grass {
    /// A grass record by form ID.
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Grass> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == GRAS)?;
        let record = rr.record().ok()?;
        Some(Grass::from_data(
            id,
            record.editor_id(),
            record
                .get(MODL)
                .map(|s| s.zstring())
                .filter(|m| !m.is_empty()),
            record.get(esm::sig::DATA).map(|s| s.data.as_slice()),
        ))
    }

    /// A grass from its `DATA` (see [`DATA_SIZE`]).
    pub fn from_data(
        form_id: FormId,
        editor_id: Option<String>,
        model: Option<String>,
        data: Option<&[u8]>,
    ) -> Grass {
        let mut d = default_data();
        if let Some(data) = data {
            let n = data.len().min(DATA_SIZE);
            d[..n].copy_from_slice(&data[..n]);
        }
        Grass {
            form_id,
            editor_id,
            model,
            density: d[0],
            min_slope: d[1],
            max_slope: d[2],
            units_from_water: u16::from_le_bytes([d[4], d[5]]),
            water_rule: le_u32(&d, 8),
            position_range: le_f32(&d, 12),
            height_range: le_f32(&d, 16),
            color_range: le_f32(&d, 20),
            wave_period: le_f32(&d, 24),
            flags: d[28],
        }
    }

    /// Lit by the model's own normals (the shader's `VERTLIT` variants).
    pub fn vertex_lighting(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Scaled the same on every axis (else only in height).
    pub fn uniform_scaling(&self) -> bool {
        self.flags & 0x02 != 0
    }

    /// Turned to lie along the slope (the shader's `FIT_TO_SLOPE`).
    pub fn fit_to_slope(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Editor ID, else form ID.
    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .unwrap_or_else(|| self.form_id.to_string())
    }

    /// Whether a blade may grow at `height` with the water at `water`
    /// ([`NO_WATER`] where the square has none), by the grass's water rule
    /// (`0057de00`). Rule 0 ("above, at least") lets a blade stand as low
    /// as `units` *below* the water, as the game's code compares it.
    pub fn water_allows(&self, height: f32, water: f32) -> bool {
        let u = f32::from(self.units_from_water);
        match self.water_rule {
            0 => height >= water - u,
            1 => water <= height && height <= water + u,
            2 => height <= water - u,
            3 => water - u <= height && height <= water,
            4 => height >= water + u || height <= water - u,
            5 => water - u <= height && height <= water + u,
            _ => true,
        }
    }
}

/// The grasses a land texture lists (`GNAM`, in order).
pub fn texture_grasses(order: &LoadOrder, land_texture: FormId) -> Vec<FormId> {
    let Some(rr) = order
        .get(land_texture)
        .filter(|r| r.entry.header.kind == LTEX)
    else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .subrecords
        .iter()
        .filter(|s| s.kind == GNAM && s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

/// The grass records met so far and each land texture's grasses, read
/// once each.
#[derive(Debug, Default)]
pub struct GrassCatalog {
    grasses: HashMap<FormId, Option<Grass>>,
    textures: HashMap<FormId, Vec<FormId>>,
}

impl GrassCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// A catalog of these grasses, for each texture its list (for building
    /// from records not in a load order).
    pub fn with(grasses: Vec<Grass>, textures: Vec<(FormId, Vec<FormId>)>) -> Self {
        GrassCatalog {
            grasses: grasses.into_iter().map(|g| (g.form_id, Some(g))).collect(),
            textures: textures.into_iter().collect(),
        }
    }

    /// A grass, if the record exists.
    pub fn grass(&self, id: FormId) -> Option<&Grass> {
        self.grasses.get(&id).and_then(Option::as_ref)
    }

    /// The grasses a land texture lists that exist, in order (the game
    /// skips list entries that don't name a grass).
    fn texture(&mut self, order: Option<&LoadOrder>, texture: FormId) -> Vec<FormId> {
        if let Some(order) = order {
            if !self.textures.contains_key(&texture) {
                let list = texture_grasses(order, texture);
                for &id in &list {
                    self.grasses
                        .entry(id)
                        .or_insert_with(|| Grass::load(order, id));
                }
                self.textures.insert(texture, list);
            }
        }
        self.textures
            .get(&texture)
            .map(|list| {
                list.iter()
                    .copied()
                    .filter(|&id| self.grass(id).is_some())
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// The game's grass settings, from the INI files (`[Grass]`), with the
/// engine's built-in defaults where they're missing (`011ca524` and
/// neighbours).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrassSettings {
    /// `bDrawShaderGrass`.
    pub draw: bool,
    /// `iGrassDensityEvalSize`: spots are 2 × this many grid points across
    /// (only 1, 2, 4 and 8 are taken; anything else is 2).
    pub eval_size: i32,
    /// `iMaxGrassTypesPerTexure` (sic): one more than this many grasses per
    /// texture are used (the game's count is off by one).
    pub max_types_per_texture: i32,
    /// `fTexturePctThreshold`: how opaque a texture must be at a point for
    /// its grass to grow there (below 0 counts as 0, above 1 as 0.9).
    pub texture_pct_threshold: f32,
    /// `iMinGrassSize`: the smallest spacing of candidates, in units.
    pub min_grass_size: i32,
    /// `fGrassStartFadeDistance` (the in-game "Grass Fade" slider, which
    /// `FalloutPrefs.ini` keeps), else `fGrassDefaultStartFadeDistance`.
    pub start_fade: f32,
    /// `fGrassFadeRange`: the fade's length.
    pub fade_range: f32,
    /// `fGrassMinStartFadeDistance`: with the slider at its minimum (less
    /// than half a unit above it) no grass is made.
    pub min_start_fade: f32,
    /// `fGrassWindMagnitudeMin`/`Max`: the sway's size in calm and at full
    /// wind.
    pub wind_min: f32,
    pub wind_max: f32,
    /// `[BlurShaderHDR] fGrassDimmer`: only the value the sun's light on
    /// grass starts with; the image space's grass dimmer replaces it every
    /// frame ([`grass_dimmer`]).
    pub dimmer: f32,
}

impl Default for GrassSettings {
    fn default() -> Self {
        GrassSettings {
            draw: true,
            eval_size: 2,
            max_types_per_texture: 2,
            texture_pct_threshold: 0.3,
            min_grass_size: 80,
            start_fade: 3500.0,
            fade_range: 1000.0,
            min_start_fade: 400.0,
            wind_min: 5.0,
            wind_max: 125.0,
            dimmer: 1.3,
        }
    }
}

impl GrassSettings {
    /// The settings from an INI lookup (`section`, `key` → value).
    pub fn from_ini(get: impl Fn(&str, &str) -> Option<f32>) -> Self {
        let d = GrassSettings::default();
        let grass = |key: &str, default: f32| get("Grass", key).unwrap_or(default);
        let default_start = grass("fGrassDefaultStartFadeDistance", d.start_fade);
        GrassSettings {
            draw: grass("bDrawShaderGrass", 1.0) != 0.0,
            eval_size: grass("iGrassDensityEvalSize", d.eval_size as f32) as i32,
            max_types_per_texture: grass("iMaxGrassTypesPerTexure", 2.0) as i32,
            texture_pct_threshold: grass("fTexturePctThreshold", d.texture_pct_threshold),
            min_grass_size: grass("iMinGrassSize", d.min_grass_size as f32) as i32,
            start_fade: grass("fGrassStartFadeDistance", default_start),
            fade_range: grass("fGrassFadeRange", d.fade_range),
            min_start_fade: grass("fGrassMinStartFadeDistance", d.min_start_fade),
            wind_min: grass("fGrassWindMagnitudeMin", d.wind_min),
            wind_max: grass("fGrassWindMagnitudeMax", d.wind_max),
            dimmer: get("BlurShaderHDR", "fGrassDimmer").unwrap_or(d.dimmer),
        }
    }

    /// The evaluation size as the game takes it (`0053bc10`).
    pub fn eval_size(&self) -> usize {
        match self.eval_size {
            1 | 2 | 4 | 8 => self.eval_size as usize,
            _ => 2,
        }
    }

    /// The opacity threshold as the game clamps it.
    pub fn threshold(&self) -> f32 {
        let t = self.texture_pct_threshold.max(0.0);
        if t > 1.0 {
            0.9
        } else {
            t
        }
    }

    /// Where grass has faded out completely.
    pub fn fade_end(&self) -> f32 {
        self.start_fade + self.fade_range
    }

    /// Whether the game makes grass at all (`0057d0a0`).
    pub fn makes_grass(&self) -> bool {
        self.draw && self.start_fade >= self.min_start_fade + 0.5
    }

    /// The sway's size for a weather's wind (`DATA` byte 0 / 255): the
    /// shader's `WindData.z` (`00b5e870`).
    pub fn wind_magnitude(&self, wind: u8) -> f32 {
        self.wind_min + f32::from(wind) / 255.0 * (self.wind_max - self.wind_min)
    }
}

/// The image space's grass dimmer (`DNAM` float 12): the sun's light on
/// grass is multiplied by it (the shader's `AddlParams.x`). The game copies
/// it from the image space every frame (`00b8b440`), after the weather's
/// modifiers apply (track 12, as for the sunlight dimmer); 1.5 in
/// `NVDefaultExterior`.
pub fn grass_dimmer(space: &ImageSpace) -> Option<f32> {
    space
        .values
        .get(GRASS_DIMMER)
        .copied()
        .filter(|v| v.is_finite() && (0.0..=100.0).contains(v))
}

/// The image space value (and modifier track) of the grass dimmer.
pub const GRASS_DIMMER: usize = 12;

/// The wind's phase for the shader (`WindData.w`, `00bac9c0`): the game's
/// clock in seconds into the day (`GameHour` × 3600) × 2π × the wave
/// period / 3600, wrapped to one turn. So a grass sways `wave_period`
/// times per game hour.
pub fn wind_phase(wave_period: f32, game_hour: f32) -> f32 {
    let turns = f64::from(wave_period) * f64::from(game_hour);
    (turns.rem_euclid(1.0) * std::f64::consts::TAU) as f32
}

/// The 3 × 3 sample points around a spot's point, as offsets in the
/// quarter's 17-point rows: rows below to above, each west to east.
const SAMPLES: [isize; 9] = [-18, -17, -16, -1, 0, 1, 16, 17, 18];

/// At most this many grasses per spot.
pub const MAX_SPOT_ENTRIES: usize = 16;

/// A layer only adds its grasses where one of a spot's samples is more than
/// this opaque (a constant in the game's code).
const LAYER_PRESENT: f32 = 0.1;

/// One spot of a quarter: where it is and what grows there.
#[derive(Debug, Clone, PartialEq)]
pub struct Spot {
    /// 0 south-west, 1 south-east, 2 north-west, 3 north-east.
    pub quarter: usize,
    /// The point it's centred on, in the quarter's 17 × 17 grid.
    pub point: usize,
    /// Its south-west corner in world units: the point less 256 each way.
    pub corner: [f32; 2],
    pub entries: Vec<SpotGrass>,
}

/// A grass in a spot and its densities at the 3 × 3 samples
/// ([`SAMPLES`]'s order): the grass's density / 100 where its texture is
/// more than the threshold opaque, else 0 (`TESGrassAreaParam`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotGrass {
    pub grass: FormId,
    pub density: [f32; 9],
}

impl Spot {
    /// The game's key for a spot (`0057ddb0`): its corner's whole x in the
    /// high 16 bits, y in the low.
    pub fn key(&self) -> u32 {
        position_key(self.corner[0], self.corner[1])
    }
}

/// `(floor(x) << 16) | (floor(y) & 0xFFFF)`, as the game keys spots and
/// seeds each blade's brightness and size.
pub fn position_key(x: f32, y: f32) -> u32 {
    ((x.floor() as i32 as u32) << 16) | (y.floor() as i32 as u32 & 0xFFFF)
}

/// How opaque the base texture is at a point: 1 less all six layers'
/// opacities (`0053a7a0`; it can go below 0).
fn base_opacity(quarter: &Quarter, point: usize) -> f32 {
    1.0 - quarter
        .layers
        .iter()
        .map(|l| l.opacity.get(point).copied().unwrap_or(0.0))
        .sum::<f32>()
}

/// A quarter's spots and what grows in each, as the game notes them when a
/// square's terrain loads (`0053bc10`), with the square's south-west corner
/// at `origin`. Spots are centred on every `2E`-th point from `E` (`E` the
/// evaluation size: points 2, 6, 10, 14), columns in the outer loop; each
/// covers the `512`-unit square around its point. First the base texture's
/// grasses, then each layer's in slot order (a layer only if one of the
/// nine samples is more than 0.1 opaque), at most one more than
/// `max_types_per_texture` per texture and [`MAX_SPOT_ENTRIES`] in all. The
/// density at a sample is all or nothing (not × the opacity), and an entry
/// whose densities average below the threshold is cleared. Spots with no
/// grass are left out.
pub fn land_spots(
    land: &Land,
    origin: [f32; 2],
    order: Option<&LoadOrder>,
    catalog: &mut GrassCatalog,
    settings: &GrassSettings,
) -> Vec<Spot> {
    let e = settings.eval_size();
    let threshold = settings.threshold();
    let per_texture = usize::try_from(settings.max_types_per_texture + 1).unwrap_or(0);
    let mut spots = Vec::new();
    for (q, quarter) in land.quarters.iter().enumerate() {
        let (qx, qy) = ((q & 1) * (QUARTER_GRID - 1), (q >> 1) * (QUARTER_GRID - 1));
        for column in (e..QUARTER_GRID - 1).step_by(2 * e) {
            for row in (e..QUARTER_GRID - 1).step_by(2 * e) {
                let point = row * QUARTER_GRID + column;
                let sample = |k: usize| (point as isize + SAMPLES[k]) as usize;
                let mut entries: Vec<SpotGrass> = Vec::new();
                let mut add = |texture: FormId,
                               opacity: &dyn Fn(usize) -> f32,
                               entries: &mut Vec<SpotGrass>| {
                    for id in catalog
                        .texture(order, texture)
                        .into_iter()
                        .take(per_texture)
                    {
                        if entries.len() >= MAX_SPOT_ENTRIES {
                            break;
                        }
                        let Some(grass) = catalog.grass(id) else {
                            continue;
                        };
                        let full = f32::from(grass.density) / 100.0;
                        let mut density = [0.0; 9];
                        for (k, d) in density.iter_mut().enumerate() {
                            if opacity(sample(k)) > threshold {
                                *d = full;
                            }
                        }
                        if density.iter().sum::<f32>() / 9.0 < threshold {
                            density = [0.0; 9];
                        }
                        entries.push(SpotGrass { grass: id, density });
                    }
                };
                if let Some(base) = quarter.base {
                    add(base, &|p| base_opacity(quarter, p), &mut entries);
                }
                for slot in 0..crate::land::MAX_LAYERS as u16 {
                    let Some(layer) = quarter.layers.iter().find(|l| l.layer == slot) else {
                        continue;
                    };
                    let at = |p: usize| layer.opacity.get(p).copied().unwrap_or(0.0);
                    if !(0..9).any(|k| at(sample(k)) > LAYER_PRESENT) {
                        continue;
                    }
                    add(layer.texture, &at, &mut entries);
                }
                if entries.is_empty() {
                    continue;
                }
                spots.push(Spot {
                    quarter: q,
                    point,
                    corner: [
                        origin[0] + (qx + column) as f32 * SPACING - 256.0,
                        origin[1] + (qy + row) as f32 * SPACING - 256.0,
                    ],
                    entries,
                });
            }
        }
    }
    spots
}

/// The height the game gives where there's no terrain.
pub const NO_LAND_HEIGHT: f32 = -2048.0;

/// The water level of a square with no water (the game's `-FLT_MAX`).
pub const NO_WATER: f32 = f32::MIN;

/// The terrain of the squares around, for finding the ground under a
/// blade (one jittered past a square's edge lands on its neighbour's).
#[derive(Debug, Default)]
pub struct Ground<'a> {
    lands: HashMap<(i32, i32), &'a Land>,
}

/// What the terrain is like at one point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundPoint {
    pub height: f32,
    /// The terrain's stored normals as the game blends them (see
    /// [`Ground::point`]).
    pub smooth_normal: [f32; 3],
    /// The triangle's own upward normal.
    pub face_normal: [f32; 3],
    /// The terrain's vertex colour there (stored, 0..1; see
    /// [`Ground::point`]).
    pub color: [f32; 3],
}

/// Which half of its 128-unit square a point lies in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Half {
    /// Squares split south-west to north-east.
    SouthEast,
    NorthWest,
    /// Squares split south-east to north-west.
    SouthWest,
    NorthEast,
}

impl<'a> Ground<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The terrain of a square.
    pub fn insert(&mut self, square: (i32, i32), land: &'a Land) {
        self.lands.insert(square, land);
    }

    /// The ground at a point (`0053b550`, `0053f1e0`, `0053caf0`,
    /// `005345c0`). The point's 128-unit square is split as the terrain is
    /// drawn (`world::land`: south-west to north-east where column + row is
    /// even, else south-east to north-west), and the triangle holding the
    /// point found (on the diagonal: the north-west / south-west half).
    ///
    /// - Height: the triangle's plane.
    /// - The blended normal: the game blends the corners' stored normals
    ///   by how far the *nearest corner* lies into its 128-unit square,
    ///   which is always 0, so it comes out as one corner's normal: the
    ///   square's south-west corner, or the south-east one in the
    ///   north-east half of a south-east to north-west square. (Where that
    ///   nearest corner stands at height exactly 0, both normals are its
    ///   own: a comparison in the game's code.)
    /// - Colour, in the `home` square (the one whose spot is being
    ///   filled): of the triangle's corner nearest the point, the first of
    ///   (the corner off the diagonal, the diagonal's south end, its north
    ///   end) on a tie (`0053fa10`). In a neighbouring square the game
    ///   takes another path (`0053f570`): the corners' colours blended as
    ///   the normals are meant to be (along the row holding two corners,
    ///   then toward the third), by the point's own position `fmod` 128,
    ///   which is negative west and south of the world's origin (so the
    ///   weights run out of 0..1 there, as in the game). White where the
    ///   terrain has no colours.
    ///
    /// Where there's no terrain: height −2048, both normals straight up,
    /// and black (the game leaves the colour at zero).
    pub fn point(&self, x: f32, y: f32, home: (i32, i32)) -> GroundPoint {
        let square = (
            (x / CELL_SIZE).floor() as i32,
            (y / CELL_SIZE).floor() as i32,
        );
        let land = self.lands.get(&square).copied();
        let Some((land, heights)) = land.and_then(|l| l.heights.as_ref().map(|h| (l, h))) else {
            return GroundPoint {
                height: NO_LAND_HEIGHT,
                smooth_normal: [0.0, 0.0, 1.0],
                face_normal: [0.0, 0.0, 1.0],
                color: [0.0; 3],
            };
        };
        let lx = x - square.0 as f32 * CELL_SIZE;
        let ly = y - square.1 as f32 * CELL_SIZE;
        let column = ((lx / SPACING).floor().max(0.0) as usize).min(GRID - 2);
        let row = ((ly / SPACING).floor().max(0.0) as usize).min(GRID - 2);
        let fx = lx - column as f32 * SPACING;
        let fy = ly - row as f32 * SPACING;
        let at = |c: usize, r: usize| r * GRID + c;
        let (sw, se, nw, ne) = (
            at(column, row),
            at(column + 1, row),
            at(column, row + 1),
            at(column + 1, row + 1),
        );
        // The corner off the diagonal, then the diagonal's two ends.
        let (half, lone, a, b) = if (column + row) % 2 == 0 {
            if fx <= fy {
                (Half::NorthWest, nw, sw, ne)
            } else {
                (Half::SouthEast, se, sw, ne)
            }
        } else if fx + fy <= SPACING {
            (Half::SouthWest, sw, se, nw)
        } else {
            (Half::NorthEast, ne, se, nw)
        };
        let h = |i: usize| heights[i];
        let height = match half {
            Half::SouthEast => {
                (h(ne) - h(se)) / SPACING * fy + (h(se) - h(sw)) / SPACING * fx + h(sw)
            }
            Half::NorthWest => {
                (h(ne) - h(nw)) / SPACING * fx + (h(nw) - h(sw)) / SPACING * fy + h(sw)
            }
            Half::SouthWest => {
                (h(nw) - h(sw)) / SPACING * fy + (h(se) - h(sw)) / SPACING * fx + h(sw)
            }
            Half::NorthEast => {
                h(ne)
                    - ((SPACING - fy) * ((h(ne) - h(se)) / SPACING)
                        + (SPACING - fx) * ((h(ne) - h(nw)) / SPACING))
            }
        };

        let place = |i: usize| {
            [
                (i % GRID) as f32 * SPACING,
                (i / GRID) as f32 * SPACING,
                heights[i],
            ]
        };
        let face_normal = upward_normal(place(lone), place(a), place(b));
        let stored = |i: usize| {
            land.normals
                .as_ref()
                .map_or([0.0, 0.0, 1.0], |n| n.get(i).copied().unwrap_or([0.0; 3]))
        };
        let distance = |i: usize| {
            let p = place(i);
            (p[0] - lx).hypot(p[1] - ly)
        };
        let mut nearest = lone;
        for corner in [a, b] {
            if distance(corner) < distance(nearest) {
                nearest = corner;
            }
        }
        let (smooth_normal, face_normal) = if heights[nearest] == 0.0 {
            (stored(nearest), stored(nearest))
        } else {
            let corner = if half == Half::NorthEast { se } else { sw };
            (normalized(stored(corner)), face_normal)
        };
        let colour = |i: usize| {
            land.colors.as_ref().map_or([1.0; 3], |c| {
                c.get(i)
                    .map_or([1.0; 3], |c| c.map(|v| f32::from(v) / 255.0))
            })
        };
        let color = if square == home {
            colour(nearest)
        } else {
            let (wx, wy) = ((x % SPACING) / SPACING, (y % SPACING) / SPACING);
            let mix =
                |p: [f32; 3], q: [f32; 3], t: f32| [0, 1, 2].map(|k| p[k] * (1.0 - t) + q[k] * t);
            // Along the row holding two corners, then toward the third.
            match half {
                Half::SouthEast => mix(mix(colour(sw), colour(se), wx), colour(ne), wy),
                Half::NorthWest => mix(colour(sw), mix(colour(nw), colour(ne), wx), wy),
                Half::SouthWest => mix(mix(colour(sw), colour(se), wx), colour(nw), wy),
                Half::NorthEast => mix(colour(se), mix(colour(nw), colour(ne), wx), wy),
            }
        };
        GroundPoint {
            height,
            smooth_normal,
            face_normal,
            color,
        }
    }
}

fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Unit length; zero (shorter than the game's 1e-6) stays zero.
fn normalized(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-6 {
        v.map(|c| c / len)
    } else {
        [0.0; 3]
    }
}

/// A triangle's unit normal, facing up (`0053d1a0`).
fn upward_normal(lone: [f32; 3], a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    let n = cross(sub3(a, lone), sub3(b, lone));
    let n = if n[2] < 0.0 { n.map(|c| -c) } else { n };
    normalized(n)
}

/// The cosine the game compares slopes with: its 512-step table
/// (`0057e960`, filled by `00a813c0`), `cos(2π k / 512)` with `k` the angle
/// in 512ths of a turn, cut to a whole number. So a 40° limit is taken as
/// 39.4°.
pub fn table_cos(radians: f32) -> f32 {
    let k = ((radians * 512.0 / TAU) as i32) & 511;
    (TAU * k as f32 / 512.0).cos()
}

/// One blade the game keeps (`0057de00`): where it stands, with the
/// terrain's normal packed into the fractions of its position, and the
/// brightness of the ground there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blade {
    /// Whole x, y and floor(height), each plus 0.5 + normal / 2 (at most
    /// 0.97): the shader reads the normal back as `2 × frac − 1`.
    pub position: [f32; 3],
    /// The ground colour's luminance (`0.31 R + 0.37 G + 0.32 B`).
    pub luminance: f32,
}

/// The game's main random number generator (`00aa5230`): a Mersenne
/// twister whose words come out without the usual tempering, seeded by
/// multiplying by 69069 (`00aa51a0`).
#[derive(Clone)]
pub struct Twister {
    state: [u32; 624],
    index: usize,
}

impl Twister {
    pub fn seeded(seed: u32) -> Twister {
        let mut state = [0u32; 624];
        state[0] = seed;
        for i in 1..624 {
            state[i] = state[i - 1].wrapping_mul(69069);
        }
        Twister { state, index: 624 }
    }

    pub fn next_u32(&mut self) -> u32 {
        const MAG: [u32; 2] = [0, 0x9908_B0DF];
        if self.index >= 624 {
            let s = &mut self.state;
            for k in 0..623 {
                let y = (s[k] & 0x8000_0000) | (s[k + 1] & 0x7FFF_FFFF);
                s[k] = s[(k + 397) % 624] ^ (y >> 1) ^ MAG[(s[k + 1] & 1) as usize];
            }
            let y = (s[623] & 0x8000_0000) | (s[0] & 0x7FFF_FFFF);
            s[623] = s[396] ^ (y >> 1) ^ MAG[(s[0] & 1) as usize];
            self.index = 0;
        }
        let v = self.state[self.index];
        self.index += 1;
        v
    }

    /// A word for a range of `n` (`00aa5230(n)`, through `005a00a0`): 0
    /// without drawing for 0; the word masked by `n` for −1 (`u32::MAX`)
    /// and 0x7FFF; otherwise the word modulo `n`.
    pub fn below(&mut self, n: u32) -> u32 {
        match n {
            0 => 0,
            u32::MAX | 0x7FFF => self.next_u32() & n,
            n => self.next_u32() % n,
        }
    }

    /// 0..32767, as the game's `rand(0, 0x7FFF)` gives it (`00944460`).
    pub fn below_32768(&mut self) -> i32 {
        (self.next_u32() & 0x7FFF) as i32
    }

    /// −1..1 (`004dfec0`: the word × 2⁻³¹ − 1).
    pub fn signed_unit(&mut self) -> f32 {
        (f64::from(self.next_u32()) * 2f64.powi(-31) - 1.0) as f32
    }
}

/// The seed for one grass in one spot. **A guess** (see the module notes):
/// the game's generator runs on from a clock seed.
pub fn spot_seed(key: u32, grass: FormId) -> u32 {
    let mut z = (u64::from(key) << 32 | u64::from(grass.0)).wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    ((z ^ (z >> 31)) as u32) | 1
}

/// Rounded to the nearest whole number, halves to even (`FISTP`).
fn round_even(v: f32) -> i32 {
    let r = v.round();
    if (v - v.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        (r - v.signum()) as i32
    } else {
        r as i32
    }
}

/// Fills one spot with one grass (`0057de00`). Candidates stand on an
/// `n × n` grid from the spot's corner, `n` = the smaller of
/// spot size / position range and spot size / `iMinGrassSize` (6 for every
/// New Vegas grass), `spot / n` apart, x in the outer loop. Each takes the
/// density at its place, the 3 × 3 samples stretched across the spot:
/// `d = 3i / n` split into a whole part rounded to the nearest (halves to
/// even) and the rest (which can be −0.5), blending toward the next sample
/// only where the rest is above 0.01. It's kept if a random 0..32767 is
/// below `trunc(density × 32768)`, then moved by a random −1..1 × the
/// position range in x, then y, cut to whole units, and dropped on the
/// ground: kept if the water rule passes and the normal's height lies
/// between the cosines of the max and min slopes ([`table_cos`]). The
/// normal: the triangle's with lit-by-normals fit-to-slope grass, the
/// average of the blended and the triangle's with fit-to-slope grass, else
/// the blended one ([`Ground::point`]).
pub fn fill_spot(
    corner: [f32; 2],
    density: &[f32; 9],
    grass: &Grass,
    ground: &Ground,
    water: f32,
    settings: &GrassSettings,
    random: &mut Twister,
) -> Vec<Blade> {
    let spot = 256 * settings.eval_size() as i32;
    // Both counts are kept as 16-bit numbers; a position range of 0 gives
    // the processor's "no number" result, which comes out as 0 candidates.
    let quotient = spot as f32 / grass.position_range;
    let by_range = if quotient.is_finite() && quotient.abs() < 2_147_483_648.0 {
        i32::from(quotient as i32 as u16)
    } else {
        0
    };
    let n = by_range.min(i32::from((spot / settings.min_grass_size.max(1)) as u16));
    if n == 0 {
        return Vec::new();
    }
    let step = spot as f32 / n as f32;
    // The square whose spot this is: the spot's point lies 256 units in.
    let home = (
        ((corner[0] + 256.0) / CELL_SIZE).floor() as i32,
        ((corner[1] + 256.0) / CELL_SIZE).floor() as i32,
    );
    let cos_min = table_cos(f32::from(grass.min_slope) * std::f32::consts::PI / 180.0);
    let cos_max = table_cos(f32::from(grass.max_slope) * std::f32::consts::PI / 180.0);
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let mut p = [corner[0] + step * i as f32, corner[1] + step * j as f32];
            let dx = 3.0 / n as f32 * i as f32;
            let dy = 3.0 / n as f32 * j as f32;
            let (bx, by) = (round_even(dx), round_even(dy));
            let (fx, fy) = (dx - bx as f32, dy - by as f32);
            let d = |c: i32, r: i32| density[(r.clamp(0, 2) * 3 + c.clamp(0, 2)) as usize];
            let c00 = d(bx, by);
            let (mut c10, mut c11, mut c01) = (0.0, 0.0, 0.0);
            if fx > 0.01 {
                c10 = d(bx + 1, by);
                if fy > 0.01 {
                    c11 = d(bx + 1, by + 1);
                }
            }
            if fy > 0.01 {
                c01 = d(bx, by + 1);
            }
            let here =
                (c11 * fx + (1.0 - fx) * c01) * fy + (1.0 - fy) * (c10 * fx + (1.0 - fx) * c00);
            let t = (f64::from(here) * 32768.0) as i64 as i32;
            if t == 0 || random.below_32768() >= t {
                continue;
            }
            p[0] += random.signed_unit() * grass.position_range;
            p[1] += random.signed_unit() * grass.position_range;
            let p = [p[0].floor(), p[1].floor()];
            let g = ground.point(p[0], p[1], home);
            if !grass.water_allows(g.height, water) {
                continue;
            }
            let n = match (grass.fit_to_slope(), grass.vertex_lighting()) {
                (true, true) => g.face_normal,
                (true, false) => normalized([
                    g.smooth_normal[0] + g.face_normal[0],
                    g.smooth_normal[1] + g.face_normal[1],
                    g.smooth_normal[2] + g.face_normal[2],
                ]),
                (false, _) => g.smooth_normal,
            };
            if !(cos_max <= n[2] && n[2] <= cos_min) {
                continue;
            }
            let packed = n.map(|c| {
                let v = c * 0.5 + 0.5;
                if v >= 0.97 {
                    0.97
                } else {
                    v
                }
            });
            let [r, gr, b] = g.color;
            out.push(Blade {
                position: [
                    p[0] + packed[0],
                    p[1] + packed[1],
                    g.height.floor() + packed[2],
                ],
                luminance: b * 0.32 + gr * 0.37 + r * 0.31,
            });
        }
    }
    out
}

/// A blade as the shader gets it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrassInstance {
    /// The shader's `InstanceData`: the blade's packed position (see
    /// [`Blade::position`]) and `w` = a whole-number size step in
    /// hundredths plus the brightness.
    pub data: [f32; 4],
    /// 0..0.99: how much of the light reaches it (the shader takes
    /// `0.25 + 0.75 ×` this).
    pub brightness: f32,
    /// Its size: `1 + w / 100` on the scaled axes.
    pub scale: f32,
}

/// The Microsoft C library's `rand`, seeded as `srand(seed)`.
struct CRand(u32);

impl CRand {
    fn next(&mut self) -> i32 {
        self.0 = self.0.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.0 >> 16) & 0x7FFF) as i32
    }
}

/// A blade's brightness and size (`00b62de0`): `srand` with the blade's
/// [`position_key`], then `b = (colour range × 0.5 × (rand / 65534 − 1) +
/// 1 − colour range) × luminance` (so between `1 − 1.5 cr` and
/// `1 − 1.25 cr` times the luminance: the game divides by 65534 where
/// 32767 was surely meant), at least 0 and 0.99 for anything from 1 up;
/// then `w = floor(height range × (2 rand / 32767 − 1) × 100) + b`.
pub fn instance_data(blade: &Blade, grass: &Grass) -> GrassInstance {
    let [x, y, z] = blade.position;
    let mut random = CRand(position_key(x, y));
    let r1 = random.next() as f32;
    let cr = grass.color_range;
    let mut b = (cr * 0.5 * (r1 / 65534.0 - 1.0) + (1.0 - cr)) * blade.luminance;
    if b >= 1.0 {
        b = 0.99;
    } else if b < 0.0 {
        b = 0.0;
    }
    let r2 = random.next() as f32;
    let s = 2.0 * r2 / 32767.0 - 1.0;
    let step = (grass.height_range * s * 100.0).floor();
    let w = step + b;
    GrassInstance {
        data: [x, y, z, w],
        brightness: b,
        scale: 1.0 + 0.01 * w,
    }
}

/// One grass's blades on a square.
#[derive(Debug, Clone, PartialEq)]
pub struct GrassBatch {
    pub grass: Grass,
    pub instances: Vec<GrassInstance>,
}

/// Fills a square's spots ([`land_spots`]) on this ground: for each spot,
/// each entry in order (`0057d4f0`), stopping at a grass without a model;
/// a grass already given blades in the spot (under another texture) isn't
/// filled again (the game keys what it made by grass and spot).
pub fn fill_spots(
    spots: &[Spot],
    catalog: &GrassCatalog,
    ground: &Ground,
    water: f32,
    settings: &GrassSettings,
) -> Vec<GrassBatch> {
    let mut batches: Vec<GrassBatch> = Vec::new();
    for spot in spots {
        let key = spot.key();
        let mut filled: HashSet<FormId> = HashSet::new();
        for entry in &spot.entries {
            let Some(grass) = catalog.grass(entry.grass) else {
                continue;
            };
            if grass.model.is_none() {
                break;
            }
            if filled.contains(&grass.form_id) {
                continue;
            }
            let mut random = Twister::seeded(spot_seed(key, grass.form_id));
            let blades = fill_spot(
                spot.corner,
                &entry.density,
                grass,
                ground,
                water,
                settings,
                &mut random,
            );
            if blades.is_empty() {
                continue;
            }
            filled.insert(grass.form_id);
            let at = match batches
                .iter()
                .position(|b| b.grass.form_id == grass.form_id)
            {
                Some(at) => at,
                None => {
                    batches.push(GrassBatch {
                        grass: grass.clone(),
                        instances: Vec::new(),
                    });
                    batches.len() - 1
                }
            };
            batches[at]
                .instances
                .extend(blades.iter().map(|b| instance_data(b, grass)));
        }
    }
    batches
}

/// A square's water level for the water rules (`005471e0`): none
/// ([`NO_WATER`]) unless the cell is flagged as having water (and isn't an
/// interior); then its `XCLW`, or the worldspace's default where it has
/// none (the game's "use the default" value is `FLT_MAX`).
pub fn water_level(order: &LoadOrder, grid: &WorldGrid, cell: FormId) -> f32 {
    let Ok(info) = cell_info(order, cell) else {
        return NO_WATER;
    };
    if info.flags & CELL_HAS_WATER == 0 || info.flags & CELL_INTERIOR != 0 {
        return NO_WATER;
    }
    match info.water_height {
        Some(h) if h != f32::MAX => h,
        _ => grid.world.default_water_height,
    }
}

/// Everything one square grows, as the game makes it around the player:
/// the square's spots filled on its terrain and its neighbours' (for
/// blades moved over the edge). Empty without terrain.
pub fn square_grass(
    order: &LoadOrder,
    grid: &WorldGrid,
    square: (i32, i32),
    settings: &GrassSettings,
) -> crate::Result<Vec<GrassBatch>> {
    let Some(land) = grid.land(order, square)? else {
        return Ok(Vec::new());
    };
    let mut around = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let at = (square.0 + dx, square.1 + dy);
            if at != square {
                if let Some(l) = grid.land(order, at)? {
                    around.push((at, l));
                }
            }
        }
    }
    let mut ground = Ground::new();
    ground.insert(square, &land);
    for (at, l) in &around {
        ground.insert(*at, l);
    }
    let water = grid
        .cell_at(square)
        .map_or(NO_WATER, |c| water_level(order, grid, c));
    let mut catalog = GrassCatalog::new();
    let origin = [square.0 as f32 * CELL_SIZE, square.1 as f32 * CELL_SIZE];
    let spots = land_spots(&land, origin, Some(order), &mut catalog, settings);
    Ok(fill_spots(&spots, &catalog, &ground, water, settings))
}

/// The squares the game fills with grass around a point (`0057d0a0`): of
/// the 3 × 3 around the one it's in, those whose ground area is closer
/// than where the grass fades out completely.
pub fn squares_near(x: f32, y: f32, settings: &GrassSettings) -> Vec<(i32, i32)> {
    let here = (
        (x / CELL_SIZE).floor() as i32,
        (y / CELL_SIZE).floor() as i32,
    );
    let mut out = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let s = (here.0 + dx, here.1 + dy);
            let (x0, y0) = (s.0 as f32 * CELL_SIZE, s.1 as f32 * CELL_SIZE);
            let gx = (x0 - x).max(0.0).max(x - (x0 + CELL_SIZE));
            let gy = (y0 - y).max(0.0).max(y - (y0 + CELL_SIZE));
            if gx.hypot(gy) < settings.fade_end() {
                out.push(s);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::land::Layer;

    fn grass(density: u8) -> Grass {
        let mut data = vec![density, 0, 40, 0xCD, 0, 0, 0xCD, 0xCD];
        data.extend(0u32.to_le_bytes());
        for v in [22.0f32, 0.35, 0.1, 15.0] {
            data.extend(v.to_le_bytes());
        }
        data.extend([6, 0, 0, 0]);
        Grass::from_data(
            FormId(0x17753),
            Some("GrassWasteland01".into()),
            Some("Landscape\\Grass\\GrassWasteland01.NIF".into()),
            Some(&data),
        )
    }

    fn flat_land(height: f32) -> Land {
        Land {
            form_id: FormId(1),
            flags: 7,
            heights: Some(vec![height; GRID * GRID]),
            normals: Some(vec![[0.0, 0.0, 1.0]; GRID * GRID]),
            colors: None,
            quarters: Default::default(),
        }
    }

    #[test]
    fn reads_the_grass_data_as_the_game_lays_it_out() {
        let g = grass(30);
        assert_eq!((g.density, g.min_slope, g.max_slope), (30, 0, 40));
        assert_eq!(g.water_rule, 0);
        assert_eq!(g.position_range, 22.0);
        assert_eq!(g.height_range, 0.35);
        assert_eq!(g.color_range, 0.1);
        assert_eq!(g.wave_period, 15.0);
        assert!(g.uniform_scaling() && g.fit_to_slope() && !g.vertex_lighting());
        // Without `DATA`: the engine's defaults.
        let d = Grass::from_data(FormId(2), None, None, None);
        assert_eq!((d.density, d.max_slope), (30, 90));
        assert_eq!((d.position_range, d.height_range), (32.0, 0.2));
        assert_eq!((d.color_range, d.wave_period), (0.5, 10.0));
    }

    #[test]
    fn water_rules_compare_as_the_game_does() {
        let mut g = grass(30);
        g.units_from_water = 10;
        // Rule 0 lets blades stand down to `units` below the water.
        assert!(g.water_allows(95.0, 100.0));
        assert!(!g.water_allows(85.0, 100.0));
        g.water_rule = 2;
        assert!(g.water_allows(85.0, 100.0) && !g.water_allows(95.0, 100.0));
        g.water_rule = 4;
        assert!(g.water_allows(111.0, 100.0) && !g.water_allows(100.0, 100.0));
        g.water_rule = 7;
        assert!(g.water_allows(-1e9, 100.0));
        // No water: everything is above it, nothing below.
        g.water_rule = 0;
        assert!(g.water_allows(-5000.0, NO_WATER));
        g.water_rule = 2;
        assert!(!g.water_allows(-5000.0, NO_WATER));
    }

    #[test]
    fn settings_come_from_the_ini_with_the_engines_defaults() {
        let s = GrassSettings::from_ini(|section, key| match (section, key) {
            ("Grass", "fTexturePctThreshold") => Some(0.0),
            ("Grass", "fGrassDefaultStartFadeDistance") => Some(3500.0),
            ("Grass", "fGrassStartFadeDistance") => Some(7000.0),
            ("Grass", "iGrassDensityEvalSize") => Some(3.0),
            _ => None,
        });
        assert_eq!(s.threshold(), 0.0);
        assert_eq!(s.fade_end(), 8000.0);
        // Only 1, 2, 4 and 8 are taken.
        assert_eq!(s.eval_size(), 2);
        assert!(s.makes_grass());
        let high = GrassSettings {
            texture_pct_threshold: 1.5,
            ..GrassSettings::default()
        };
        assert_eq!(high.threshold(), 0.9);
        // Calm to full wind: NVWastelandGS's 50 gives 28.5.
        assert!((s.wind_magnitude(50) - 28.53).abs() < 0.01);
    }

    #[test]
    fn spots_note_each_textures_grass_where_it_is_painted() {
        let mut land = flat_land(0.0);
        let base = FormId(0xA00);
        let road = FormId(0xA01);
        land.quarters[0].base = Some(base);
        // Road painted fully over the north half of the south-west quarter.
        let mut opacity = vec![0.0; QUARTER_GRID * QUARTER_GRID];
        for (p, o) in opacity.iter_mut().enumerate() {
            if p / QUARTER_GRID >= 8 {
                *o = 1.0;
            }
        }
        land.quarters[0].layers.push(Layer {
            texture: road,
            layer: 0,
            opacity,
        });
        let g = grass(30);
        let mut other = grass(50);
        other.form_id = FormId(0x99);
        let mut catalog = GrassCatalog::with(
            vec![g.clone(), other.clone()],
            vec![(base, vec![g.form_id]), (road, vec![other.form_id])],
        );
        let settings = GrassSettings {
            texture_pct_threshold: 0.0,
            ..GrassSettings::default()
        };
        let spots = land_spots(&land, [4096.0, 0.0], None, &mut catalog, &settings);
        // Four by four spots in the quarter, centred on points 2, 6, 10, 14.
        assert_eq!(spots.len(), 16);
        let first = &spots[0];
        assert_eq!(first.point, 2 * 17 + 2);
        assert_eq!(first.corner, [4096.0 + 256.0 - 256.0, 0.0]);
        // Columns in the outer loop: the second spot is north of the first.
        assert_eq!(spots[1].point, 6 * 17 + 2);
        // Below the road only the base's grass.
        assert_eq!(first.entries.len(), 1);
        assert_eq!(first.entries[0].density, [0.3; 9]);
        // Row 6's spot samples rows 5–7: no road yet. Row 10's (9–11)
        // lies under it: the base is gone there, the road's grass grows.
        let under = &spots[2];
        assert_eq!(under.entries.len(), 2);
        assert_eq!(under.entries[0].density, [0.0; 9]);
        assert_eq!(under.entries[1].grass, other.form_id);
        assert_eq!(under.entries[1].density, [0.5; 9]);
        // Row 6 spot touches row 7 only: the road's samples there are 0.
        assert_eq!(spots[1].entries.len(), 1);
    }

    #[test]
    fn at_most_one_more_grass_than_the_setting_per_texture() {
        let mut land = flat_land(0.0);
        let base = FormId(0xA00);
        land.quarters[0].base = Some(base);
        let grasses: Vec<Grass> = (0..5)
            .map(|i| {
                let mut g = grass(30);
                g.form_id = FormId(0x100 + i);
                g
            })
            .collect();
        let ids = grasses.iter().map(|g| g.form_id).collect();
        let mut catalog = GrassCatalog::with(grasses, vec![(base, ids)]);
        let spots = land_spots(
            &land,
            [0.0, 0.0],
            None,
            &mut catalog,
            &GrassSettings::default(),
        );
        assert_eq!(spots[0].entries.len(), 3);
    }

    #[test]
    fn the_ground_is_the_triangles_plane() {
        let mut land = flat_land(0.0);
        let heights = land.heights.as_mut().unwrap();
        // The square at column 0, row 0 (even: split south-west to
        // north-east): corners SW 0, SE 128, NW 256, NE 512.
        heights[0] = 0.0;
        heights[1] = 128.0;
        heights[GRID] = 256.0;
        heights[GRID + 1] = 512.0;
        let mut ground = Ground::new();
        ground.insert((0, 0), &land);
        // South-east half: SW + (SE − SW) x + (NE − SE) y.
        let p = ground.point(64.0, 32.0, (0, 0));
        assert!((p.height - (64.0 + (512.0 - 128.0) * 0.25)).abs() < 1e-3);
        // North-west half: SW + (NE − NW) x + (NW − SW) y.
        let p = ground.point(32.0, 64.0, (0, 0));
        assert!((p.height - ((512.0 - 256.0) * 0.25 + 128.0)).abs() < 1e-3);
        // Both triangles face up.
        assert!(p.face_normal[2] > 0.0);
        // Off the terrain: the game's −2048.
        assert_eq!(ground.point(-10.0, 5.0, (0, 0)).height, NO_LAND_HEIGHT);
    }

    #[test]
    fn the_blended_normal_is_one_corners_as_the_game_works_it_out() {
        let mut land = flat_land(100.0);
        let normals = land.normals.as_mut().unwrap();
        let tilted = |x: f32| normalized([x, 0.0, 1.0]);
        // The square at column 1, row 0 is odd: south-east to north-west.
        normals[1] = tilted(0.1); // its SW corner
        normals[2] = tilted(0.2); // SE
        normals[GRID + 1] = tilted(0.3); // NW
        normals[GRID + 2] = tilted(0.4); // NE
        let mut ground = Ground::new();
        ground.insert((0, 0), &land);
        // The south-west half takes the south-west corner's, wherever the
        // point is in it; the north-east half the south-east corner's.
        let sw = ground.point(128.0 + 10.0, 20.0, (0, 0)).smooth_normal;
        assert!((sw[0] - tilted(0.1)[0]).abs() < 1e-6, "{sw:?}");
        let ne = ground.point(128.0 + 120.0, 100.0, (0, 0)).smooth_normal;
        assert!((ne[0] - tilted(0.2)[0]).abs() < 1e-6, "{ne:?}");
        // The face of flat ground is straight up.
        assert_eq!(
            ground.point(140.0, 20.0, (0, 0)).face_normal,
            [0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn the_colour_is_the_nearest_corners() {
        let mut land = flat_land(10.0);
        let mut colors = vec![[255u8, 255, 255]; GRID * GRID];
        colors[GRID + 1] = [255, 0, 0]; // NE corner of the first square
        land.colors = Some(colors);
        let mut ground = Ground::new();
        ground.insert((0, 0), &land);
        assert_eq!(ground.point(100.0, 110.0, (0, 0)).color, [1.0, 0.0, 0.0]);
        assert_eq!(ground.point(10.0, 20.0, (0, 0)).color, [1.0; 3]);
        // Seen from the neighbour west of it, the same point blends the
        // corners: the north-west half's row (NW white, NE red) at x
        // 100/128, then from SW (white) toward it by 110/128.
        let c = ground.point(100.0, 110.0, (-1, 0)).color;
        let g = 1.0 - (100.0 / 128.0) * (110.0 / 128.0);
        assert!(
            (c[0] - 1.0).abs() < 1e-5 && (c[1] - g).abs() < 1e-5,
            "{c:?}"
        );
        // Off the terrain the colour stays black.
        assert_eq!(ground.point(-5.0, 10.0, (0, 0)).color, [0.0; 3]);
    }

    #[test]
    fn west_of_the_origin_the_neighbours_blend_runs_past_its_corners() {
        // The game blends by the position `fmod` 128, negative here.
        let mut land = flat_land(10.0);
        let mut colors = vec![[0u8, 0, 0]; GRID * GRID];
        colors[1] = [255, 255, 255]; // the square's south-east corner
        land.colors = Some(colors);
        let mut ground = Ground::new();
        ground.insert((-1, 0), &land);
        // x = −4096 + 100: 100 units into the square, but −28 / 128 by
        // the game's measure (column 0 + row 0 is even; y 10 < x 100: the
        // south-east half, `(1 − w) SW + w SE` along the row).
        let c = ground.point(-4096.0 + 100.0, 10.0, (0, 0)).color;
        assert!(
            (c[0] - (-28.0 / 128.0) * (1.0 - 10.0 / 128.0)).abs() < 1e-5,
            "{c:?}"
        );
    }

    #[test]
    fn slopes_are_compared_through_the_games_table() {
        assert_eq!(table_cos(0.0), 1.0);
        // 40° is 56.9 steps of 512: taken as 56.
        let c = table_cos(40f32.to_radians());
        assert!((c - (TAU * 56.0 / 512.0).cos()).abs() < 1e-6);
        assert!(c > 40f32.to_radians().cos());
    }

    #[test]
    fn the_twister_matches_the_reference_without_tempering() {
        // Standard MT19937 seeded with the game's multiplier: untempered
        // output is the raw state word after the first twist.
        let mut t = Twister::seeded(1);
        let first = t.next_u32();
        let mut again = Twister::seeded(1);
        assert_eq!(again.next_u32(), first);
        assert!((0..32768).contains(&Twister::seeded(5).below_32768()));
        let u = Twister::seeded(9).signed_unit();
        assert!((-1.0..1.0).contains(&u));
    }

    #[test]
    fn a_full_density_spot_keeps_every_candidate() {
        let land = flat_land(1000.4);
        let mut ground = Ground::new();
        ground.insert((0, 0), &land);
        let mut g = grass(100);
        g.position_range = 22.0;
        let settings = GrassSettings::default();
        let mut random = Twister::seeded(7);
        let blades = fill_spot(
            [512.0, 512.0],
            &[1.0; 9],
            &g,
            &ground,
            NO_WATER,
            &settings,
            &mut random,
        );
        // 512 / 80 = 6 candidates per side.
        assert_eq!(blades.len(), 36);
        for b in &blades {
            let [x, y, z] = b.position;
            // Flat ground packs (0, 0, 1) as 0.5, 0.5, 0.97.
            assert!((x - x.floor() - 0.5).abs() < 1e-3, "{x}");
            assert!((y - y.floor() - 0.5).abs() < 1e-3, "{y}");
            assert!((z - 1000.97).abs() < 1e-3, "{z}");
            assert!((512.0 - 23.0..1024.0 + 23.0).contains(&x), "{x}");
            // No colours: white.
            assert!((b.luminance - 1.0).abs() < 1e-6);
        }
        // Sloped ground past the grass's limit keeps nothing.
        let mut steep = flat_land(0.0);
        for (i, h) in steep.heights.as_mut().unwrap().iter_mut().enumerate() {
            *h = (i % GRID) as f32 * 256.0;
        }
        steep.normals = Some(vec![normalized([-2.0, 0.0, 1.0]); GRID * GRID]);
        let mut ground = Ground::new();
        ground.insert((0, 0), &steep);
        let mut random = Twister::seeded(7);
        let none = fill_spot(
            [512.0, 512.0],
            &[1.0; 9],
            &g,
            &ground,
            NO_WATER,
            &settings,
            &mut random,
        );
        assert!(none.is_empty());
    }

    #[test]
    fn densities_stretch_across_the_spot_with_the_games_rounding() {
        // Only the east column of samples has grass: with 6 candidates,
        // i = 3 rounds 1.5 to 2 and reads the east column at 1.5 × its
        // density, i = 4 and 5 read it in full, i = 2 blends nothing in.
        let land = flat_land(0.0);
        let mut ground = Ground::new();
        ground.insert((0, 0), &land);
        let g = grass(100);
        let density = [0.0, 0.0, 0.7, 0.0, 0.0, 0.7, 0.0, 0.0, 0.7];
        let mut random = Twister::seeded(3);
        let blades = fill_spot(
            [0.0, 0.0],
            &density,
            &g,
            &ground,
            NO_WATER,
            &GrassSettings::default(),
            &mut random,
        );
        // Every blade comes from columns 3 to 5 (x from 256 less the jitter).
        assert!(!blades.is_empty());
        assert!(blades.iter().all(|b| b.position[0] >= 256.0 - 23.0));
        // Column 3 (1.05 → certain) is always kept: 6 blades there.
        let near_256 = blades
            .iter()
            .filter(|b| b.position[0] < 256.0 + 23.0)
            .count();
        assert_eq!(near_256, 6);
    }

    #[test]
    fn brightness_and_size_come_from_the_blades_position() {
        let g = grass(30);
        let blade = Blade {
            position: [-73700.5, 1200.5, 8300.97],
            luminance: 0.8,
        };
        let a = instance_data(&blade, &g);
        assert_eq!(a, instance_data(&blade, &g));
        // Colour range 0.1: 0.85 to 0.875 of the luminance.
        assert!(a.brightness >= 0.8 * 0.85 - 1e-4 && a.brightness <= 0.8 * 0.875 + 1e-4);
        // The size step is whole hundredths within the height range.
        let step = a.data[3] - a.brightness;
        assert!((step - step.round()).abs() < 1e-3);
        assert!(step.abs() <= 35.0);
        assert!((a.scale - (1.0 + 0.01 * a.data[3])).abs() < 1e-6);
        // `srand(1)`: the C library's first numbers are 41 and 18467.
        let mut r = CRand(1);
        assert_eq!((r.next(), r.next()), (41, 18467));
    }

    #[test]
    fn the_wind_sways_wave_period_times_an_hour() {
        assert!(wind_phase(15.0, 10.0).abs() < 1e-4);
        assert!((wind_phase(1.0, 10.25) - std::f32::consts::FRAC_PI_2).abs() < 1e-4);
    }

    #[test]
    fn grass_grows_in_the_players_square_and_its_neighbours() {
        let s = GrassSettings {
            start_fade: 7000.0,
            fade_range: 1000.0,
            ..GrassSettings::default()
        };
        assert_eq!(squares_near(-73700.0, 1000.0, &s).len(), 9);
        let short = GrassSettings {
            start_fade: 100.0,
            fade_range: 100.0,
            ..GrassSettings::default()
        };
        // Within 200 units of the square's south-west corner: it, and the
        // three squares meeting there.
        let near = squares_near(4096.0 + 50.0, 4096.0 + 50.0, &short);
        assert_eq!(near.len(), 4);
        assert!(near.contains(&(0, 0)) && near.contains(&(1, 1)));
    }
}
