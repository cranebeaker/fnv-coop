//! Water: the water types (`WATR`), where a cell's water stands, and
//! placed water (`PWAT`).
//!
//! Read from the game's code (`FalloutNV.exe`; the notes are in
//! `nv-re\findings\water.md`) and checked against `FalloutNV.esm`:
//!
//! - A water type's look is its 196-byte `DNAM` (loader `0057ef80`); which
//!   field feeds which shader constant is read from the code that fills the
//!   water shader's constants (`004e3590`, `00b864d0`), the ripples
//!   (`004e3d60`) and the depth pass (`004ec800`), and the console's `mws`
//!   command (`005d2a80`) prints the fields under the names used here.
//! - An outdoor cell's water (`005471e0`): none indoors or when the cell's
//!   `DATA` lacks [`crate::CELL_HAS_WATER`]; else at `XCLW`, or at the
//!   worldspace's default water height when `XCLW` holds the largest float
//!   (as Goodsprings' does). Its type is `XCWT`, else the worldspace's
//!   (`NAM2`).
//! - Placed water (`PWAT`): a model (a flat quad carrying a
//!   `WaterShaderProperty`) and `DNAM` = flags, water type. The flags say
//!   what it reflects and refracts ([`placed_flags`]).

use esm::{sig, FormId, FourCC, LoadOrder, Record};

use crate::cell::{le_f32, le_u32, CELL_HAS_WATER, CELL_INTERIOR};
use crate::exterior::Worldspace;
use crate::{Error, Result};

const WATR: FourCC = FourCC::new(b"WATR");
const PWAT: FourCC = FourCC::new(b"PWAT");
const WRLD: FourCC = FourCC::new(b"WRLD");
const NNAM: FourCC = FourCC::new(b"NNAM");
const ANAM: FourCC = FourCC::new(b"ANAM");
const FNAM: FourCC = FourCC::new(b"FNAM");
const SNAM: FourCC = FourCC::new(b"SNAM");
const DNAM: FourCC = FourCC::new(b"DNAM");
const XCLW: FourCC = FourCC::new(b"XCLW");
const XCWT: FourCC = FourCC::new(b"XCWT");
const XNAM: FourCC = FourCC::new(b"XNAM");
const NAM3: FourCC = FourCC::new(b"NAM3");
const NAM4: FourCC = FourCC::new(b"NAM4");

/// The size of a water type's `DNAM` in New Vegas.
pub const VISUAL_SIZE: usize = 196;

/// `ANAM` when a water type has none: the loader's default (75).
pub const DEFAULT_OPACITY: u8 = 75;

/// What a cell's `XCLW` holds when the cell takes the worldspace's water
/// height: the largest float (`7F7FFFFF`).
pub const USE_DEFAULT_HEIGHT: f32 = f32::MAX;

/// A water type's look (`DNAM`). Byte offsets in brackets; the shader
/// constant each feeds is named where it was read.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterVisual {
    /// [16] The power of the sun's highlight (`VarAmounts.x`).
    pub sun_power: f32,
    /// [20] How much of the reflection shows over the reflection colour
    /// (`VarAmounts.y`, "reflectamt").
    pub reflectivity: f32,
    /// [24] The fresnel term's value looking straight down (`FresnelRI.x`).
    pub fresnel: f32,
    /// [32], [36] Fog above water, near and far (`FogParam.w` = far − near,
    /// `FogParam.z` = far); the far distance also scales the depth map.
    pub fog_near: f32,
    pub fog_far: f32,
    /// [40], [44], [48] Shallow, deep and reflection colours, as stored
    /// (R, G, B, unused).
    pub shallow: [u8; 4],
    pub deep: [u8; 4],
    pub reflection: [u8; 4],
    /// [88] The strength of the wading displacement's normal
    /// (`BlendRadius.w`).
    pub displacement_strength: f32,
    /// [96] How steep the ripple normals are (`fNoiseScale`).
    pub noise_scale: f32,
    /// [100..108] The three ripple layers' wind directions, degrees.
    pub wind_directions: [f32; 3],
    /// [112..120] Their speeds, in noise texture lengths a second.
    pub wind_speeds: [f32; 3],
    /// [124], [128] Depth falloff start and end (`DepthFalloff`).
    pub depth_falloff: [f32; 2],
    /// [132] How much the above-water fog hides (`FogColor.w`).
    pub fog_amount: f32,
    /// [136] World units one noise texture spans (`TexScale`).
    pub noise_tile_size: f32,
    /// [140], [144], [148] Underwater fog amount, near and far.
    pub underwater_fog_amount: f32,
    pub underwater_fog_near: f32,
    pub underwater_fog_far: f32,
    /// [152] How far ripples bend the reflection and refraction far away
    /// (`VarAmounts.w`).
    pub distortion: f32,
    /// [156] Shininess of point-light highlights (`FresnelRI.z`; the lit
    /// water pass only).
    pub shininess: f32,
    /// [160] Reflection multiplier as stored (see
    /// [`WaterVisual::reflection_multiplier`]).
    pub reflection_multiplier_raw: f32,
    /// [164], [168] Point-light radius and brightness (lit water pass).
    pub light_radius: f32,
    pub light_brightness: f32,
    /// [172..180] The three ripple layers' texture coordinate scales as
    /// stored (see [`WaterVisual::noise_uv_scale`]).
    pub uv_scales: [f32; 3],
    /// [184..192] The three ripple layers' amplitudes.
    pub amplitudes: [f32; 3],
}

impl WaterVisual {
    /// Reads the 196-byte `DNAM`.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < VISUAL_SIZE {
            return None;
        }
        let f = |at: usize| le_f32(data, at);
        let rgba = |at: usize| [data[at], data[at + 1], data[at + 2], data[at + 3]];
        Some(Self {
            sun_power: f(16),
            reflectivity: f(20),
            fresnel: f(24),
            fog_near: f(32),
            fog_far: f(36),
            shallow: rgba(40),
            deep: rgba(44),
            reflection: rgba(48),
            displacement_strength: f(88),
            noise_scale: f(96),
            wind_directions: [f(100), f(104), f(108)],
            wind_speeds: [f(112), f(116), f(120)],
            depth_falloff: [f(124), f(128)],
            fog_amount: f(132),
            noise_tile_size: f(136),
            underwater_fog_amount: f(140),
            underwater_fog_near: f(144),
            underwater_fog_far: f(148),
            distortion: f(152),
            shininess: f(156),
            reflection_multiplier_raw: f(160),
            light_radius: f(164),
            light_brightness: f(168),
            uv_scales: [f(172), f(176), f(180)],
            amplitudes: [f(184), f(188), f(192)],
        })
    }

    /// `FresnelRI.w`: the stored reflection multiplier ÷ 10, at least 1
    /// (`00b864d0`).
    pub fn reflection_multiplier(&self) -> f32 {
        (self.reflection_multiplier_raw / 10.0).max(1.0)
    }

    /// `DepthFalloff` as the shader gets it: start and end, or (−1, 0) when
    /// they're equal (`00b864d0`), which makes the falloff 1 everywhere.
    pub fn falloff_constant(&self) -> [f32; 2] {
        let [start, end] = self.depth_falloff;
        if start == end {
            [-1.0, 0.0]
        } else {
            [start, end]
        }
    }

    /// Ripple layer `i`'s texture coordinate scale (`fTexScale`, the noise
    /// pass's `c4`): the stored value ÷ 100 rounded up, at least 1
    /// (`004e3d60`). New Vegas's water stores 0: scale 1.
    pub fn noise_uv_scale(&self, i: usize) -> f32 {
        (self.uv_scales[i] / 100.0).ceil().max(1.0)
    }

    /// Where ripple layer `i` has scrolled after `seconds`, in noise
    /// texture lengths (both 0..1). Each frame the game adds speed × frame
    /// time × (sin, cos) of the wind direction (`004e3d60`), wrapping to
    /// 0..1; summed, that's this. (That the frame time is in seconds is
    /// inferred.)
    pub fn layer_offset(&self, i: usize, seconds: f32) -> [f32; 2] {
        let (sin, cos) = self.wind_directions[i].to_radians().sin_cos();
        let travel = self.wind_speeds[i] * seconds;
        [
            (travel * sin).rem_euclid(1.0),
            (travel * cos).rem_euclid(1.0),
        ]
    }
}

/// A water type (`WATR`).
#[derive(Debug, Clone, PartialEq)]
pub struct WaterType {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `NNAM`: the noise texture, as written (`Data\Textures\Water\...`).
    pub noise: Option<String>,
    /// `ANAM`: opacity, 0–100 (`VarAmounts.z` = this ÷ 100).
    pub opacity: u8,
    /// `FNAM` flags (what they do isn't traced).
    pub flags: u8,
    /// `SNAM`: the sound.
    pub sound: Option<FormId>,
    /// `DNAM`, when the record has the full 196 bytes.
    pub visual: Option<WaterVisual>,
}

impl WaterType {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<WaterType> {
        let rr = order.get(id)?;
        if rr.entry.header.kind != WATR {
            return None;
        }
        let record = rr.record().ok()?;
        Some(WaterType {
            form_id: id,
            editor_id: record.editor_id(),
            noise: record
                .get(NNAM)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            opacity: record
                .get(ANAM)
                .and_then(|s| s.data.first().copied())
                .unwrap_or(DEFAULT_OPACITY),
            flags: record
                .get(FNAM)
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            sound: form(&rr, &record, SNAM),
            visual: record.get(DNAM).and_then(|s| WaterVisual::parse(&s.data)),
        })
    }

    /// Editor ID, else form ID.
    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .unwrap_or_else(|| self.form_id.to_string())
    }
}

/// The noise texture's path as the asset lookup takes it (under
/// `textures\`, lower case, without the `Data\` the records write).
pub fn noise_path(written: &str) -> String {
    let lower = written.replace('/', "\\").to_ascii_lowercase();
    let lower = lower.strip_prefix("data\\").unwrap_or(&lower);
    if lower.starts_with("textures\\") {
        lower.to_string()
    } else {
        format!("textures\\{lower}")
    }
}

/// The water of an outdoor cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellWater {
    /// The surface's height, game units.
    pub height: f32,
    /// The water type.
    pub water_type: Option<FormId>,
    /// The height is the worldspace's default (the cell's `XCLW` holds
    /// [`USE_DEFAULT_HEIGHT`], or it has none).
    pub default_height: bool,
}

/// Worldspace flag (`PNAM`): the worldspace takes its water from its parent
/// (`WNAM`). The flag's meaning is xEdit's name for it, not traced.
pub const USE_PARENT_WATER: u16 = 0x08;

/// A worldspace's water: its default height (`DNAM`), its water type
/// (`NAM2`) and its noise texture (`XNAM`), from its parent when it uses
/// the parent's water ([`USE_PARENT_WATER`]).
#[derive(Debug, Clone, PartialEq)]
pub struct WorldWater {
    pub default_height: f32,
    pub water_type: Option<FormId>,
    /// `NAM3`: the distant (LOD) water's type (worldspace +0x78, read by
    /// the code building distant water, `004e4c80`).
    pub lod_water_type: Option<FormId>,
    /// `NAM4` (worldspace +0x7c): the distant water's mirror plane, and the
    /// "sea level": water groups at this height outdoors reflect the whole
    /// scene (`004e4730`, `004eaa00`). Read from the loader (`00583560`).
    pub lod_height: Option<f32>,
    /// `XNAM`: the ripples' noise texture for every water in the
    /// worldspace. The game uses it rather than each type's own while
    /// `bUsePerWorldSpaceWaterNoise` is on (its default; `004e3d60`).
    pub noise: Option<String>,
}

impl WorldWater {
    pub fn load(order: &LoadOrder, world: FormId) -> Option<WorldWater> {
        let mut w = Worldspace::load(order, world).ok()?;
        // Follow the parent while the worldspace uses its water (at most a
        // few steps; a loop of parents would be broken data).
        for _ in 0..4 {
            match w.parent {
                Some((parent, flags)) if flags & USE_PARENT_WATER != 0 => {
                    match Worldspace::load(order, parent) {
                        Ok(p) => w = p,
                        Err(_) => break,
                    }
                }
                _ => break,
            }
        }
        let rr = order
            .get(w.form_id)
            .filter(|r| r.entry.header.kind == WRLD)?;
        let record = rr.record().ok()?;
        let noise = record
            .get(XNAM)
            .map(|s| s.zstring())
            .filter(|s| !s.is_empty());
        Some(WorldWater {
            default_height: w.default_water_height,
            water_type: w.water,
            lod_water_type: form(&rr, &record, NAM3),
            lod_height: record
                .get(NAM4)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_f32(&s.data, 0)),
            noise,
        })
    }
}

/// A cell's water (see the module notes): `None` indoors and where the
/// cell has no water.
pub fn cell_water(order: &LoadOrder, cell: FormId) -> Result<Option<CellWater>> {
    let rr = order.get(cell).ok_or(Error::NoSuchRecord(cell))?;
    if rr.entry.header.kind != sig::CELL {
        return Err(Error::NotACell {
            form_id: cell,
            kind: rr.entry.header.kind,
        });
    }
    let record = rr.record()?;
    let flags = record
        .get(sig::DATA)
        .and_then(|d| d.data.first().copied())
        .unwrap_or(0);
    if flags & CELL_INTERIOR != 0 || flags & CELL_HAS_WATER == 0 {
        return Ok(None);
    }
    let world = order.world_of(&rr).and_then(|w| WorldWater::load(order, w));
    let own = record
        .get(XCLW)
        .filter(|s| s.data.len() >= 4)
        .map(|s| le_f32(&s.data, 0))
        .filter(|&h| h != USE_DEFAULT_HEIGHT);
    let (height, default_height) = match own {
        Some(h) => (h, false),
        // A cell without `XCLW` is taken as one holding the largest float
        // (the field's value before it's read; inferred).
        None => (world.as_ref().map_or(0.0, |w| w.default_height), true),
    };
    let water_type = form(&rr, &record, XCWT).or(world.and_then(|w| w.water_type));
    Ok(Some(CellWater {
        height,
        water_type,
        default_height,
    }))
}

/// What placed water reflects and refracts: its `PWAT` `DNAM` flags. The
/// game reads these bits (`004e3590`); the rest of the names are xEdit's
/// and match the defaults the code gives a cell's own water.
pub mod placed_flags {
    pub const REFLECTS: u32 = 0x1;
    pub const REFLECTS_ACTORS: u32 = 0x2;
    pub const REFLECTS_LAND: u32 = 0x4;
    pub const REFLECTS_LOD_LAND: u32 = 0x8;
    pub const REFLECTS_LOD_BUILDINGS: u32 = 0x10;
    pub const REFLECTS_TREES: u32 = 0x20;
    pub const REFLECTS_SKY: u32 = 0x40;
    pub const REFLECTS_DYNAMIC_OBJECTS: u32 = 0x80;
    pub const REFLECTS_DEAD_BODIES: u32 = 0x100;
    pub const REFRACTS: u32 = 0x200;
    pub const REFRACTS_ACTORS: u32 = 0x400;
    pub const REFRACTS_LAND: u32 = 0x800;
    pub const REFRACTS_DYNAMIC_OBJECTS: u32 = 0x1_0000;
    pub const REFRACTS_DEAD_BODIES: u32 = 0x2_0000;
    pub const SILHOUETTE_REFLECTIONS: u32 = 0x4_0000;
    pub const DEPTH: u32 = 0x1000_0000;
    /// The ripples follow the model's texture coordinates rather than the
    /// world position.
    pub const OBJECT_UVS: u32 = 0x2000_0000;
    pub const NO_UNDERWATER_FOG: u32 = 0x8000_0000;

    /// A cell's own water ("auto water", `00580130` with `004fc2e0`):
    /// reflects (land, distant land and buildings, trees, the sky),
    /// refracts (actors, land), depth, and a bit (0x40000000) whose
    /// meaning isn't traced.
    pub const CELL_WATER: u32 = REFLECTS
        | REFLECTS_LAND
        | REFLECTS_LOD_LAND
        | REFLECTS_LOD_BUILDINGS
        | REFLECTS_TREES
        | REFLECTS_SKY
        | REFRACTS
        | REFRACTS_ACTORS
        | REFRACTS_LAND
        | DEPTH
        | 0x4000_0000;
}

/// Placed water's base object (`PWAT`).
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceableWater {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `MODL`, relative to `meshes\`.
    pub model: Option<String>,
    /// `DNAM` flags ([`placed_flags`]).
    pub flags: u32,
    /// `DNAM`'s water type.
    pub water_type: Option<FormId>,
}

impl PlaceableWater {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<PlaceableWater> {
        let rr = order.get(id)?;
        if rr.entry.header.kind != PWAT {
            return None;
        }
        let record = rr.record().ok()?;
        let dnam = record.get(DNAM).filter(|s| s.data.len() >= 8);
        Some(PlaceableWater {
            form_id: id,
            editor_id: record.editor_id(),
            model: record
                .get(sig::MODL)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            flags: dnam.map_or(0, |s| le_u32(&s.data, 0)),
            water_type: dnam
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 4))))
                .filter(|id| id.0 != 0),
        })
    }
}

/// Whether a record is placed water's base object.
pub fn is_placeable_water(kind: FourCC) -> bool {
    kind == PWAT
}

/// A form ID subrecord, made global.
fn form(rr: &esm::RecordRef<'_>, record: &Record, kind: FourCC) -> Option<FormId> {
    record
        .get(kind)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .filter(|id| id.0 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `NVCleanWaterGS`'s `DNAM`, as in `FalloutNV.esm` (only the fields
    /// read here filled in).
    fn goodsprings() -> Vec<u8> {
        let mut d = vec![0u8; VISUAL_SIZE];
        let mut put = |at: usize, v: f32| d[at..at + 4].copy_from_slice(&v.to_le_bytes());
        put(16, 801.0);
        put(20, 0.75);
        put(24, 0.75);
        put(32, 7.0);
        put(36, 58.0);
        put(96, 13.41);
        put(100, 180.0);
        put(104, 10.0);
        put(108, 67.0);
        put(112, 0.065);
        put(116, 0.033);
        put(120, 0.029);
        put(128, 0.0109);
        put(132, 0.72);
        put(136, 1000.0);
        put(152, 623.2);
        put(160, 1.0);
        put(184, 0.3);
        d[40..44].copy_from_slice(&[91, 102, 145, 0]);
        d
    }

    #[test]
    fn reads_the_visual_data_where_the_game_does() {
        assert!(WaterVisual::parse(&[0; 100]).is_none());
        let v = WaterVisual::parse(&goodsprings()).unwrap();
        assert_eq!(v.sun_power, 801.0);
        assert_eq!(v.fog_far, 58.0);
        assert_eq!(v.shallow, [91, 102, 145, 0]);
        assert_eq!(v.noise_tile_size, 1000.0);
        assert_eq!(v.distortion, 623.2);
        assert_eq!(v.depth_falloff, [0.0, 0.0109]);
        assert_eq!(v.amplitudes[0], 0.3);
        // max(1 / 10, 1).
        assert_eq!(v.reflection_multiplier(), 1.0);
        // Stored 0: scale 1.
        assert_eq!(v.noise_uv_scale(0), 1.0);
        assert_eq!(v.falloff_constant(), [0.0, 0.0109]);
    }

    #[test]
    fn equal_falloff_ends_turn_the_falloff_off() {
        let mut d = goodsprings();
        d[128..132].copy_from_slice(&0f32.to_le_bytes());
        assert_eq!(
            WaterVisual::parse(&d).unwrap().falloff_constant(),
            [-1.0, 0.0]
        );
    }

    #[test]
    fn layers_scroll_along_their_wind() {
        let v = WaterVisual::parse(&goodsprings()).unwrap();
        // Layer 1 blows toward 180°: v goes down by 0.065 a second.
        let [u, w] = v.layer_offset(0, 1.0);
        assert!(u.abs() < 1e-6 || (u - 1.0).abs() < 1e-6, "{u}");
        assert!((w - (1.0 - 0.065)).abs() < 1e-5, "{w}");
        let [u, _] = v.layer_offset(2, 2.0);
        assert!((u - 0.058 * 67f32.to_radians().sin()).abs() < 1e-5);
    }

    #[test]
    fn noise_textures_are_found_under_textures() {
        assert_eq!(
            noise_path("Data\\Textures\\Water\\WastelandWaterPotomac.dds"),
            "textures\\water\\wastelandwaterpotomac.dds"
        );
        assert_eq!(noise_path("water/a.dds"), "textures\\water\\a.dds");
    }
}
