//! Grass, ready to draw: the blades the game grows on a square
//! (`world::grass`) and each grass's model, copied once per blade into one
//! mesh per grass and square, as the game batches them (`00b61980`,
//! `00b62de0`): every copy's vertices carry their blade's `InstanceData`,
//! and the grass shader (`GRASS2002.vso`) places, turns, sizes and sways
//! each copy from it.

use world::grass::{GrassBatch, GrassSettings};
use world::WorldGrid;

use crate::{Error, Game, TextureData};

/// A grass's model as the game copies it for each blade: its one mesh's
/// vertices as the file stores them (the copies are built from the mesh
/// data, `00b61980`, so no node's transform is applied; all 17 grass
/// models the game's records name have one mesh and no top-node
/// transform).
pub struct GrassModel {
    /// The model's path, for messages.
    pub path: String,
    /// Game units, in the model's own axes (x across, y the other way
    /// across, z up).
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// Vertex colours as stored (0..1). Red, green and blue scale the
    /// sunlight on the blade; alpha is how much the vertex sways (its
    /// square). White and fully swaying where the model has none.
    pub colors: Vec<[f32; 4]>,
    /// Triangles.
    pub indices: Vec<u32>,
    pub texture: Option<TextureData>,
    /// The model's alpha threshold / 255 (the shader's `AlphaTestRef`), 0
    /// without an alpha property.
    pub alpha_test: f32,
    pub double_sided: bool,
    /// How far the farthest vertex is from the model's origin.
    pub radius: f32,
    /// The largest vertex alpha, squared: the most any vertex sways, per
    /// unit of wind.
    pub sway: f32,
}

/// One grass's blades on one square as a single mesh: the model copied
/// once per blade, each copy's vertices holding the blade's
/// `InstanceData`.
#[derive(Debug, Clone)]
pub struct GrassMesh {
    pub grass: world::grass::Grass,
    /// Per vertex: the model's vertex as stored (game units).
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    /// Per vertex: its blade's `InstanceData` (see
    /// `world::grass::GrassInstance::data`).
    pub instances: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    /// A box (low and high corners, world game units) holding every blade
    /// at any size and sway.
    pub bounds: ([f32; 3], [f32; 3]),
}

impl Game {
    /// The game's grass settings, from its INI files.
    pub fn grass_settings(&self) -> GrassSettings {
        GrassSettings::from_ini(|section, key| self.settings.float(section, key))
    }

    /// The blades one square of a worldspace grows, by grass
    /// (`world::grass::square_grass`); empty where the game makes no grass
    /// (`bDrawShaderGrass` off, or the fade slider at its minimum).
    pub fn square_grass(
        &self,
        grid: &WorldGrid,
        square: (i32, i32),
    ) -> Result<Vec<GrassBatch>, Error> {
        let settings = self.grass_settings();
        if !settings.makes_grass() {
            return Ok(Vec::new());
        }
        Ok(world::grass::square_grass(
            &self.order,
            grid,
            square,
            &settings,
        )?)
    }

    /// A grass's model (`MODL`, under `meshes\`), or `None` when it can't
    /// be read or has no mesh.
    pub fn grass_model(&self, model: &str) -> Option<GrassModel> {
        let path = assets::mesh_path(model);
        let bytes = self.assets.read(&path).ok()??;
        let mesh = nif::Nif::parse(bytes)
            .ok()?
            .placed_scene()
            .ok()?
            .meshes
            .into_iter()
            .next()?;
        let count = mesh.positions.len();
        let texture = mesh.diffuse_texture().and_then(|t| {
            let path = assets::texture_path(t);
            let bytes = self.assets.read(&path).ok()??;
            TextureData::from_dds(path, bytes).ok()
        });
        let colors: Vec<[f32; 4]> = (0..count)
            .map(|i| mesh.colors.get(i).copied().unwrap_or([1.0; 4]))
            .collect();
        let radius = mesh
            .positions
            .iter()
            .map(|p| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
            .fold(0.0, f32::max);
        let sway = colors.iter().map(|c| c[3] * c[3]).fold(0.0, f32::max);
        Some(GrassModel {
            path,
            normals: (0..count)
                .map(|i| mesh.normals.get(i).copied().unwrap_or([0.0, 0.0, 1.0]))
                .collect(),
            uvs: (0..count)
                .map(|i| mesh.uvs.get(i).copied().unwrap_or([0.0, 0.0]))
                .collect(),
            positions: mesh.positions,
            colors,
            indices: mesh
                .triangles
                .iter()
                .flatten()
                .map(|&i| u32::from(i))
                .collect(),
            texture,
            alpha_test: mesh
                .alpha
                .as_ref()
                .map_or(0.0, |a| f32::from(a.threshold) / 255.0),
            double_sided: mesh.double_sided,
            radius,
            sway,
        })
    }
}

/// A grass's blades on a square as one mesh (see [`GrassMesh`]);
/// `wind_max` is the strongest sway (`fGrassWindMagnitudeMax`), for the
/// bounds.
pub fn grass_mesh(batch: &GrassBatch, model: &GrassModel, wind_max: f32) -> GrassMesh {
    let n = model.positions.len();
    let blades = batch.instances.len();
    let mut mesh = GrassMesh {
        grass: batch.grass.clone(),
        positions: Vec::with_capacity(n * blades),
        normals: Vec::with_capacity(n * blades),
        uvs: Vec::with_capacity(n * blades),
        colors: Vec::with_capacity(n * blades),
        instances: Vec::with_capacity(n * blades),
        indices: Vec::with_capacity(model.indices.len() * blades),
        bounds: ([f32::MAX; 3], [f32::MIN; 3]),
    };
    for (k, blade) in batch.instances.iter().enumerate() {
        let first = (k * n) as u32;
        mesh.positions.extend_from_slice(&model.positions);
        mesh.normals.extend_from_slice(&model.normals);
        mesh.uvs.extend_from_slice(&model.uvs);
        mesh.colors.extend_from_slice(&model.colors);
        mesh.instances.extend(std::iter::repeat(blade.data).take(n));
        mesh.indices
            .extend(model.indices.iter().map(|&i| first + i));
        // The shader turns, scales (by `1 + w / 100`) and sways the copy
        // around the blade's position.
        let reach = model.radius * blade.scale.abs().max(1.0) + wind_max.abs() * model.sway;
        for axis in 0..3 {
            mesh.bounds.0[axis] = mesh.bounds.0[axis].min(blade.data[axis] - reach);
            mesh.bounds.1[axis] = mesh.bounds.1[axis].max(blade.data[axis] + reach);
        }
    }
    mesh
}
