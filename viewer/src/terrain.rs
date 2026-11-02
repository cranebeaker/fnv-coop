//! The terrain's material: the game's landscape shader (`terrain.wgsl`),
//! blending up to seven textures by per-vertex weights, as an extension of
//! the standard material (which only supplies Bevy's pipeline here).

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology,
};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError,
    VertexFormat,
};
use cellview::{space, TerrainData};

use crate::lighting::GameLighting;

const SHADER: Handle<Shader> = weak_handle!("8d1f0c52-3a6e-4b97-a1d4-6e2b9c0f7a31");

/// The weights of the first four textures, and of the next three (`w`
/// unused).
pub const ATTRIBUTE_WEIGHTS_A: MeshVertexAttribute =
    MeshVertexAttribute::new("TerrainWeightsA", 3_814_202_911, VertexFormat::Float32x4);
pub const ATTRIBUTE_WEIGHTS_B: MeshVertexAttribute =
    MeshVertexAttribute::new("TerrainWeightsB", 3_814_202_912, VertexFormat::Float32x4);
/// The normal maps' second axis (along V), as the game stores it beside the
/// tangent (`world::land::TerrainMesh::binormals`).
pub const ATTRIBUTE_BINORMAL: MeshVertexAttribute =
    MeshVertexAttribute::new("TerrainBinormal", 3_814_202_913, VertexFormat::Float32x3);

pub type TerrainMaterial = ExtendedMaterial<StandardMaterial, Terrain>;

/// A texture's diffuse and normal map.
pub type TexturePair = (Option<Handle<Image>>, Option<Handle<Image>>);

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct Terrain {
    #[uniform(100)]
    pub lighting: GameLighting,
    #[texture(101)]
    #[sampler(115)]
    pub base_0: Option<Handle<Image>>,
    #[texture(102)]
    pub base_1: Option<Handle<Image>>,
    #[texture(103)]
    pub base_2: Option<Handle<Image>>,
    #[texture(104)]
    pub base_3: Option<Handle<Image>>,
    #[texture(105)]
    pub base_4: Option<Handle<Image>>,
    #[texture(106)]
    pub base_5: Option<Handle<Image>>,
    #[texture(107)]
    pub base_6: Option<Handle<Image>>,
    /// Normal maps, sampled as stored.
    #[texture(108)]
    pub normal_0: Option<Handle<Image>>,
    #[texture(109)]
    pub normal_1: Option<Handle<Image>>,
    #[texture(110)]
    pub normal_2: Option<Handle<Image>>,
    #[texture(111)]
    pub normal_3: Option<Handle<Image>>,
    #[texture(112)]
    pub normal_4: Option<Handle<Image>>,
    #[texture(113)]
    pub normal_5: Option<Handle<Image>>,
    #[texture(114)]
    pub normal_6: Option<Handle<Image>>,
    /// The blend toward the distant land at the loaded area's edge
    /// (`cellview::TerrainLodBlend`): where, and the chunk's textures and
    /// the noise, sampled as stored.
    #[uniform(116)]
    pub land_blend: LandBlend,
    #[texture(117)]
    #[sampler(120)]
    pub lod_base: Option<Handle<Image>>,
    #[texture(118)]
    pub lod_normal: Option<Handle<Image>>,
    #[texture(119)]
    pub lod_noise: Option<Handle<Image>>,
    /// The hour's light outdoors (`crate::shared_light::BUFFER`): the
    /// ambient, the sun and the fog.
    #[storage(121, read_only)]
    pub shared: Handle<bevy::render::storage::ShaderStorageBuffer>,
}

/// `terrain.wgsl`'s `LandBlend`: the middle of the player's cell (Bevy x
/// and z), where the blend is full and how far it ramps (meters); the
/// quarter's corner in the chunk's texture, and 1 when it blends at all.
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType, Reflect)]
pub struct LandBlend {
    pub centre_full_width: Vec4,
    pub offset: Vec4,
}

impl LandBlend {
    /// For a quarter whose corner in its chunk's texture is `offset`
    /// (`None`: no distant land to blend to), with the player in `here`.
    pub fn new(offset: Option<[f32; 2]>, here: (i32, i32)) -> Self {
        let mut out = LandBlend {
            centre_full_width: Vec4::ZERO,
            offset: offset.map_or(Vec4::ZERO, |[u, v]| Vec4::new(u, v, 0.0, 1.0)),
        };
        out.move_to(here);
        out
    }

    /// The player is now in square `here`: the blend is measured from its
    /// middle (`LandBlendParams.zw`).
    pub fn move_to(&mut self, here: (i32, i32)) {
        let size = world::land::CELL_SIZE;
        let centre = space::point([
            (here.0 as f32 + 0.5) * size,
            (here.1 as f32 + 0.5) * size,
            0.0,
        ]);
        let m = space::METERS_PER_UNIT;
        self.centre_full_width = Vec4::new(
            centre[0],
            centre[2],
            cellview::LAND_BLEND_FULL * m,
            cellview::LAND_BLEND_WIDTH * m,
        );
    }
}

impl Terrain {
    /// The material for one quarter: its textures (diffuse, normal map),
    /// base first, the lighting, and what it blends into at the loaded
    /// area's edge (the distant land's texture and normal map, the noise).
    pub fn new(
        lighting: GameLighting,
        textures: &[TexturePair],
        land_blend: LandBlend,
        lod: TexturePair,
        noise: Option<Handle<Image>>,
    ) -> Self {
        let get = |i: usize| textures.get(i).cloned().unwrap_or((None, None));
        let [(b0, n0), (b1, n1), (b2, n2), (b3, n3), (b4, n4), (b5, n5), (b6, n6)] =
            [0, 1, 2, 3, 4, 5, 6].map(get);
        // Without the distant land's textures there's nothing to blend to.
        let land_blend = if lod.0.is_some() && lod.1.is_some() && noise.is_some() {
            land_blend
        } else {
            LandBlend {
                offset: Vec4::ZERO,
                ..land_blend
            }
        };
        Terrain {
            shared: crate::shared_light::BUFFER,
            lighting,
            land_blend,
            lod_base: lod.0,
            lod_normal: lod.1,
            lod_noise: noise,
            base_0: b0,
            base_1: b1,
            base_2: b2,
            base_3: b3,
            base_4: b4,
            base_5: b5,
            base_6: b6,
            normal_0: n0,
            normal_1: n1,
            normal_2: n2,
            normal_3: n3,
            normal_4: n4,
            normal_5: n5,
            normal_6: n6,
        }
    }
}

impl MaterialExtension for Terrain {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor.vertex.shader != SHADER {
            return Ok(());
        }
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_TANGENT.at_shader_location(4),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
            ATTRIBUTE_WEIGHTS_A.at_shader_location(8),
            ATTRIBUTE_WEIGHTS_B.at_shader_location(9),
            ATTRIBUTE_BINORMAL.at_shader_location(10),
        ])?];
        Ok(())
    }
}

/// A quarter of terrain as a Bevy mesh, in Bevy's space.
pub fn terrain_mesh(data: &TerrainData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let positions: Vec<[f32; 3]> = data.positions.iter().map(|&p| space::point(p)).collect();
    let normals: Vec<[f32; 3]> = data.normals.iter().map(|&n| space::direction(n)).collect();
    let tangents: Vec<[f32; 4]> = data
        .tangents
        .iter()
        .map(|t| {
            let [x, y, z] = space::direction([t[0], t[1], t[2]]);
            [x, y, z, t[3]]
        })
        .collect();
    let binormals: Vec<[f32; 3]> = data
        .binormals
        .iter()
        .map(|&b| space::direction(b))
        .collect();
    let colors: Vec<[f32; 4]> = data
        .colors
        .iter()
        .map(|c| [c[0], c[1], c[2], 1.0])
        .collect();
    let weights_a: Vec<[f32; 4]> = data
        .weights
        .iter()
        .map(|w| [w[0], w[1], w[2], w[3]])
        .collect();
    let weights_b: Vec<[f32; 4]> = data
        .weights
        .iter()
        .map(|w| [w[4], w[5], w[6], w[7]])
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(ATTRIBUTE_WEIGHTS_A, weights_a);
    mesh.insert_attribute(ATTRIBUTE_WEIGHTS_B, weights_b);
    mesh.insert_attribute(ATTRIBUTE_BINORMAL, binormals);
    mesh.insert_indices(Indices::U16(data.indices.clone()));
    mesh
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "terrain.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<TerrainMaterial>::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_blend_toward_distant_land_is_measured_from_the_players_cell() {
        let m = space::METERS_PER_UNIT;
        // The Goodsprings recording: `LandBlendParams.zw` pointed every
        // quarter at the middle of cell -18,0, (-71680, 2048).
        let blend = LandBlend::new(Some([0.0, 0.375]), (-18, 0));
        let c = blend.centre_full_width;
        assert!((c.x + 71680.0 * m).abs() < 1e-3);
        // Game north is Bevy's -z.
        assert!((c.y + 2048.0 * m).abs() < 1e-3);
        assert!((c.z - 9625.6 * m).abs() < 1e-3);
        assert!((c.w - 2662.4 * m).abs() < 1e-2);
        assert_eq!(blend.offset, Vec4::new(0.0, 0.375, 0.0, 1.0));
        // Moving on: measured from the new square's middle.
        let mut moved = blend;
        moved.move_to((-17, 0));
        assert!((moved.centre_full_width.x + 67584.0 * m).abs() < 1e-3);
        // Without distant land there's nothing to blend toward.
        assert_eq!(LandBlend::new(None, (0, 0)).offset.w, 0.0);
    }
}
