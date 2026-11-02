//! A place's water, ready to draw: each outdoor square's own water (a flat
//! square at its height) and placed water (`PWAT` models), with the water
//! type's values and the noise texture its ripples are made from. The
//! rules are `world::water`'s; how the game draws it is in the viewer's
//! `water.wgsl`.

use esm::{FormId, LoadOrder};
use nif::math::Transform;
use world::water::{cell_water, placed_flags, PlaceableWater, WaterType, WorldWater};
use world::{Land, LoadedCell, RotationConvention};

use crate::TextureCache;

/// The game's water switches (`[Water]` in its INI files; all on by
/// default and in this install).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterSettings {
    pub reflections: bool,
    pub refractions: bool,
    pub depth: bool,
    /// `bUsePerWorldSpaceWaterNoise` (not in the INI files: the game's
    /// default, on): ripples from the worldspace's noise texture rather
    /// than each type's.
    pub world_noise: bool,
    /// `iWaterReflectWidth` × `iWaterReflectHeight` (1024 × 1024 in this
    /// install's `FalloutPrefs.ini`; the game's default 512).
    pub reflection_size: (u32, u32),
    /// `iWaterMultiSamples` (`[Display]`, 4 here).
    pub multisamples: u32,
    /// `bUseWaterReflectionBlur` and `iWaterBlurAmount` (on and 4 here):
    /// the reflection is blurred with radius amount + 1.
    pub blur: Option<u32>,
}

impl Default for WaterSettings {
    fn default() -> Self {
        Self {
            reflections: true,
            refractions: true,
            depth: true,
            world_noise: true,
            reflection_size: (512, 512),
            multisamples: 1,
            blur: None,
        }
    }
}

impl WaterSettings {
    pub fn from_ini(ini: &assets::IniSettings) -> Self {
        let d = Self::default();
        let flag = |section: &str, key: &str, default: bool| {
            ini.get(section, key)
                .and_then(|v| v.trim().parse::<i64>().ok())
                .map_or(default, |v| v != 0)
        };
        let number = |section: &str, key: &str, default: u32| {
            ini.get(section, key)
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(default)
        };
        let blur = flag("Water", "bUseWaterReflectionBlur", false)
            .then(|| number("Water", "iWaterBlurAmount", 1) + 1);
        Self {
            reflections: flag("Water", "bUseWaterReflections", d.reflections),
            refractions: flag("Water", "bUseWaterRefractions", d.refractions),
            depth: flag("Water", "bUseWaterDepth", d.depth),
            world_noise: flag("Water", "bUsePerWorldSpaceWaterNoise", d.world_noise),
            reflection_size: (
                number("Water", "iWaterReflectWidth", d.reflection_size.0).max(1),
                number("Water", "iWaterReflectHeight", d.reflection_size.1).max(1),
            ),
            multisamples: number("Display", "iWaterMultiSamples", d.multisamples).max(1),
            blur,
        }
    }
}

/// One water surface (game units, world space).
#[derive(Debug, Clone, PartialEq)]
pub struct WaterData {
    /// What it is, for messages.
    pub name: String,
    /// Triangles facing up (counter-clockwise seen from above).
    pub positions: Vec<[f32; 3]>,
    /// The model's texture coordinates (zero for a square's water); used
    /// for the ripples when the flags say [`placed_flags::OBJECT_UVS`].
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u16>,
    /// The surface's height.
    pub height: f32,
    /// The model's scale in the world (1 for a square's water): the
    /// shader's `t0.w`, which the ripples' offset is divided by.
    pub scale: f32,
    /// The model's own x and y axes in the world (flat, unit length): the
    /// game adds the ripples' offset in the model's own space, so it turns
    /// with the model.
    pub axes: [[f32; 2]; 2],
    /// What it reflects and refracts ([`placed_flags`]; a square's water
    /// has [`placed_flags::CELL_WATER`]), as stored.
    pub flags: u32,
    /// The passes it gets: its flags with the game's switches.
    pub reflections: bool,
    pub refractions: bool,
    pub depth: bool,
    /// Inside (the game's `INTERIORWATER` shaders: no sun).
    pub interior: bool,
    /// Its reflection shows the scene, not only the sky: outdoors, water
    /// at the worldspace's "sea level" (its distant-water height, `NAM4`;
    /// mirrored with the whole scene while `bForceHighDetailReflections`
    /// is on, `004eaa00`), other outdoor water only the sky (`004eaf80`);
    /// inside, always (the room, as its flags say: `004e9d40`).
    pub reflects_scene: bool,
    pub water_type: WaterType,
    /// Index into [`crate::ViewerScene::textures`]: the ripples' noise (a
    /// `linear` texture).
    pub noise: Option<usize>,
    /// The placed reference (a form ID; 0 for a square's water).
    pub reference: u32,
}

/// How far from the worldspace's water height water still counts as
/// standing at it: the game snaps water within `fWaterGroupHeightRange`
/// (10, not in the INI files) to a group's height (`004e4730`).
pub const GROUP_HEIGHT_RANGE: f32 = 10.0;

/// What a place's water is made from, gathered before the place's objects
/// are handed on.
pub(crate) struct WaterSource {
    interior: bool,
    world: Option<WorldWater>,
    /// The square's own water, its south-west corner and its terrain's
    /// lowest point.
    cell: Option<(world::water::CellWater, [f32; 2])>,
    /// Placed water: reference, base, transform.
    placed: Vec<(FormId, PlaceableWater, Transform)>,
    label: String,
}

impl WaterSource {
    pub(crate) fn of(
        order: &LoadOrder,
        loaded: &LoadedCell,
        land: Option<(&Land, [f32; 2])>,
    ) -> WaterSource {
        let info = &loaded.info;
        let world = info.world.and_then(|w| WorldWater::load(order, w));
        // A square's water is the land triangles under its surface (the
        // game builds "auto water" from them, `0049c930`); where all its
        // terrain stands above the water there's none. A square without
        // terrain gets the whole square (a guess).
        let cell = if info.interior {
            None
        } else {
            cell_water(order, info.form_id)
                .ok()
                .flatten()
                .zip(info.grid)
                .filter(|(w, _)| match land.and_then(|(l, _)| l.heights.as_ref()) {
                    Some(h) => h.iter().any(|&z| z < w.height),
                    None => true,
                })
                .map(|(w, (x, y))| {
                    let size = world::land::CELL_SIZE;
                    (w, [x as f32 * size, y as f32 * size])
                })
        };
        let convention = RotationConvention::DEFAULT;
        let placed = loaded
            .objects
            .iter()
            .filter(|o| world::water::is_placeable_water(o.base_type))
            .filter_map(|o| {
                let base = PlaceableWater::load(order, o.base)?;
                Some((
                    o.form_id,
                    base,
                    convention.transform(o.position, o.rotation, o.scale),
                ))
            })
            .collect();
        WaterSource {
            interior: info.interior,
            world,
            cell,
            placed,
            label: info.label(),
        }
    }

    /// The water surfaces, their noise textures loaded into `cache`.
    pub(crate) fn build(
        &self,
        order: &LoadOrder,
        assets: &assets::Assets,
        settings: &WaterSettings,
        cache: &mut TextureCache<'_>,
        notes: &mut Vec<String>,
    ) -> Vec<WaterData> {
        let mut out = Vec::new();
        // "Sea level": the worldspace's distant-water height (`NAM4`; the
        // group test reads the worldspace's +0x7c, where the loader
        // `00583560` stores `NAM4`). New Vegas's is −2300, its default
        // water height too: the Colorado below the dam.
        let sea_level = self.world.as_ref().and_then(|w| w.lod_height);
        let at_sea_level = |h: f32| sea_level.is_some_and(|w| (h - w).abs() <= GROUP_HEIGHT_RANGE);
        let noise_for = |t: &WaterType, cache: &mut TextureCache<'_>| {
            let world_noise = self
                .world
                .as_ref()
                .and_then(|w| w.noise.as_deref())
                .filter(|_| settings.world_noise);
            world_noise
                .or(t.noise.as_deref())
                .and_then(|n| cache.get_linear(&world::water::noise_path(n)))
        };
        let surface =
            |name: String, flags: u32, interior: bool, height: f32, t: WaterType| WaterData {
                name,
                positions: Vec::new(),
                uvs: Vec::new(),
                indices: Vec::new(),
                height,
                scale: 1.0,
                axes: [[1.0, 0.0], [0.0, 1.0]],
                flags,
                reflections: settings.reflections && flags & placed_flags::REFLECTS != 0,
                refractions: settings.refractions && flags & placed_flags::REFRACTS != 0,
                depth: settings.depth && flags & placed_flags::DEPTH != 0,
                interior,
                reflects_scene: interior || at_sea_level(height),
                water_type: t,
                noise: None,
                reference: 0,
            };

        if let Some((w, [x, y])) = &self.cell {
            match w.water_type.and_then(|t| WaterType::load(order, t)) {
                Some(t) => {
                    let size = world::land::CELL_SIZE;
                    let h = w.height;
                    let mut d = surface(
                        format!("{} water", self.label),
                        placed_flags::CELL_WATER,
                        false,
                        h,
                        t,
                    );
                    d.positions = vec![
                        [*x, *y, h],
                        [x + size, *y, h],
                        [x + size, y + size, h],
                        [*x, y + size, h],
                    ];
                    d.uvs = vec![[0.0, 0.0]; 4];
                    d.indices = vec![0, 1, 2, 0, 2, 3];
                    d.noise = noise_for(&d.water_type, cache);
                    out.push(d);
                }
                None => notes.push(format!(
                    "water at {:.0} has no readable water type",
                    w.height
                )),
            }
        }

        for (reference, base, placed) in &self.placed {
            let Some(t) = base.water_type.and_then(|t| WaterType::load(order, t)) else {
                notes.push(format!("placed water {reference}: no readable water type"));
                continue;
            };
            let Some(model) = base.model.as_deref() else {
                continue;
            };
            let path = assets::mesh_path(model);
            let Some(scene) = assets
                .read(&path)
                .ok()
                .flatten()
                .and_then(|b| nif::Nif::parse(b).ok())
                .and_then(|n| n.placed_scene().ok())
            else {
                notes.push(format!("placed water {reference}: can't read {path}"));
                continue;
            };
            for mesh in scene.meshes.iter().filter(|m| is_water_mesh(m)) {
                let world = placed.then_child(&mesh.transform);
                let positions: Vec<[f32; 3]> = mesh
                    .positions
                    .iter()
                    .map(|&p| world.apply_point(p))
                    .collect();
                if positions.is_empty() || mesh.triangles.is_empty() {
                    continue;
                }
                let height = positions.iter().map(|p| p[2]).sum::<f32>() / positions.len() as f32;
                let mut d = surface(
                    format!(
                        "{} ({})",
                        base.editor_id.as_deref().unwrap_or(model),
                        mesh.name
                    ),
                    base.flags,
                    self.interior,
                    height,
                    t.clone(),
                );
                d.indices = facing_up(&positions, &mesh.triangles);
                d.uvs = (0..positions.len())
                    .map(|i| mesh.uvs.get(i).copied().unwrap_or([0.0, 0.0]))
                    .collect();
                d.positions = positions;
                d.scale = world.scale;
                d.axes = flat_axes(&world);
                d.noise = noise_for(&d.water_type, cache);
                d.reference = reference.0;
                out.push(d);
            }
        }
        for d in &out {
            notes.push(format!(
                "water: {} at {:.0} ({}{}{}{}{})",
                d.name,
                d.height,
                d.water_type.label(),
                if d.reflections {
                    if d.reflects_scene {
                        ", reflects the scene"
                    } else {
                        ", reflects the sky"
                    }
                } else {
                    ""
                },
                if d.refractions { ", refracts" } else { "" },
                if d.depth { ", depth" } else { "" },
                if d.interior { ", inside" } else { "" },
            ));
        }
        out
    }
}

/// Distant water over one cell (game units, world space).
#[derive(Debug, Clone, PartialEq)]
pub struct LodWaterPiece {
    /// The cell its triangles stand over.
    pub cell: (i32, i32),
    pub positions: Vec<[f32; 3]>,
    /// Facing up (counter-clockwise seen from above) where they're flat.
    pub indices: Vec<u16>,
    /// Its highest point: the water's surface.
    pub height: f32,
}

/// A worldspace's distant water in one distant-land chunk (level 4, by its
/// south-west cell), one piece per cell. The game's chunk models
/// (`meshes\landscape\lod\<world>\<world>.level4.x<X>.y<Y>.nif`) carry,
/// besides the land, a shape with no properties: the water, moved by its
/// own node to the chunk's corner, its surface at each cell's water height
/// (Lake Mead's at 2600, the Colorado's at −2300) and its edges dropped far
/// down (to −14098). The game builds its distant water from it
/// (`004e4c80`: placed water named `PlaceableLODWater`, `00580330`); that
/// the shape's own height is used as stored is inferred, and the split by
/// cell (each triangle by its middle) is this reader's, so the pieces over
/// loaded cells can be hidden (a guess at what the game does there). Empty
/// where the chunk has none.
pub fn lod_water(assets: &assets::Assets, world: &str, cell: (i32, i32)) -> Vec<LodWaterPiece> {
    let path = format!(
        "meshes\\landscape\\lod\\{w}\\{w}.level4.x{}.y{}.nif",
        cell.0,
        cell.1,
        w = world.to_ascii_lowercase()
    );
    let Some(scene) = assets
        .read(&path)
        .ok()
        .flatten()
        .and_then(|b| nif::Nif::parse(b).ok())
        .and_then(|n| n.placed_scene().ok())
    else {
        return Vec::new();
    };
    let size = world::land::CELL_SIZE;
    // Per cell: positions and triangles.
    type Piece = (Vec<[f32; 3]>, Vec<u16>);
    let mut pieces: std::collections::BTreeMap<(i32, i32), Piece> =
        std::collections::BTreeMap::new();
    for mesh in scene.meshes.iter().filter(|m| m.property_types.is_empty()) {
        let positions: Vec<[f32; 3]> = mesh.model_positions().collect();
        for t in &mesh.triangles {
            let Some(corners) = t
                .iter()
                .map(|&i| positions.get(usize::from(i)).copied())
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            let middle = [0, 1].map(|k| corners.iter().map(|c| c[k]).sum::<f32>() / 3.0);
            let at = (
                (middle[0] / size).floor() as i32,
                (middle[1] / size).floor() as i32,
            );
            let (p, indices) = pieces.entry(at).or_default();
            if p.len() + 3 > usize::from(u16::MAX) {
                continue;
            }
            let base = p.len() as u16;
            p.extend(corners.iter().copied());
            let up = (corners[1][0] - corners[0][0]) * (corners[2][1] - corners[0][1])
                - (corners[1][1] - corners[0][1]) * (corners[2][0] - corners[0][0]);
            if up >= 0.0 {
                indices.extend([base, base + 1, base + 2]);
            } else {
                indices.extend([base, base + 2, base + 1]);
            }
        }
    }
    pieces
        .into_iter()
        .map(|(cell, (positions, indices))| LodWaterPiece {
            cell,
            height: positions.iter().map(|p| p[2]).fold(f32::MIN, f32::max),
            positions,
            indices,
        })
        .collect()
}

/// The distant water's type: the worldspace's `NAM3` (read: the code that
/// builds distant water, `004e4c80`, takes the worldspace's +0x78 through
/// `005860c0`, and the worldspace loader `00583560` stores `NAM3` there).
/// Without one the game takes a default form (`011ca53c`), taken here as
/// `DefaultWater` (0x18; a guess).
pub fn lod_water_type(order: &LoadOrder, world: FormId) -> Option<WaterType> {
    let w = WorldWater::load(order, world)?;
    WaterType::load(order, w.lod_water_type.unwrap_or(DEFAULT_WATER))
}

/// The game's `DefaultWater` water type.
pub const DEFAULT_WATER: FormId = FormId(0x18);

/// Whether a model's piece is drawn with the game's water shader.
pub fn is_water_mesh(mesh: &nif::Mesh) -> bool {
    mesh.property_types
        .iter()
        .any(|t| t == "WaterShaderProperty")
}

/// Triangles wound to face up (+z) in the world.
fn facing_up(positions: &[[f32; 3]], triangles: &[[u16; 3]]) -> Vec<u16> {
    let mut out = Vec::with_capacity(triangles.len() * 3);
    for t in triangles {
        let [a, b, c] = t.map(|i| positions.get(usize::from(i)).copied());
        let (Some(a), Some(b), Some(c)) = (a, b, c) else {
            continue;
        };
        let up = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if up >= 0.0 {
            out.extend(t);
        } else {
            out.extend([t[0], t[2], t[1]]);
        }
    }
    out
}

/// A transform's x and y axes laid flat (unit length).
fn flat_axes(t: &Transform) -> [[f32; 2]; 2] {
    let axis = |c: usize| {
        let (x, y) = (t.rotation[0][c], t.rotation[1][c]);
        let len = x.hypot(y);
        if len > 1e-6 {
            [x / len, y / len]
        } else {
            [1.0, 0.0]
        }
    };
    [axis(0), axis(1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangles_are_turned_to_face_up() {
        let p = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_eq!(facing_up(&p, &[[0, 1, 2]]), vec![0, 1, 2]);
        assert_eq!(facing_up(&p, &[[0, 2, 1]]), vec![0, 1, 2]);
    }

    #[test]
    fn settings_come_from_the_water_section() {
        let dir = std::env::temp_dir().join(format!("nv-rs-water-ini-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ini = dir.join("Fallout.ini");
        std::fs::write(
            &ini,
            "[Water]\nbUseWaterReflections=0\niWaterReflectWidth=1024\n\
             bUseWaterReflectionBlur=1\niWaterBlurAmount=4\n[Display]\niWaterMultiSamples=4\n",
        )
        .unwrap();
        let s = WaterSettings::from_ini(&assets::IniSettings::load(&[ini]));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(!s.reflections);
        assert!(s.refractions && s.depth && s.world_noise);
        assert_eq!(s.reflection_size, (1024, 512));
        assert_eq!(s.multisamples, 4);
        assert_eq!(s.blur, Some(5));
    }
}
