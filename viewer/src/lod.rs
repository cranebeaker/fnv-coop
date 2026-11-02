//! Distant land: the game's pre-built terrain chunks drawn past the loaded
//! cells, with its distant-land shader (`lod_land.wgsl`), at the levels of
//! the game's quadtree (`world::lod`): which chunks show, the parent's
//! texture fading out of newly split quarters, and the geomorph factor.

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use std::collections::HashSet;

use bevy::asset::{load_internal_asset, weak_handle, RenderAssetUsages};
#[cfg(not(target_os = "macos"))]
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
#[cfg(target_os = "macos")]
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology,
};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError,
    VertexFormat,
};
use cellview::{space, LodChunk};
use world::lod::{LodNode, LodSettings};

use crate::{SceneEntity, Spawner};

const SHADER: Handle<Shader> = weak_handle!("5c7e9a13-2b4d-4f86-9e01-3a8d6c2b7f45");

/// How far distant land inside the loaded cells is lowered, in game units:
/// the game's `fLODLandDropAmount` (230 in `Fallout_default.ini`), which
/// the Goodsprings recording showed arriving as `GeomorphParams.y` (230).
pub const LOD_DROP: f32 = 230.0;

/// The game's far clip plane outdoors, in game units: from the Goodsprings
/// recording's projection (`1.000014`, `−5.000071` with the near plane at
/// 5: 352,000 give or take 2,500; how the game picks it isn't traced).
/// Distant land past it isn't drawn (the sky shows there), as in the game.
pub const GAME_FAR_CLIP: f32 = 352_000.0;

/// Each vertex's geomorph height (the chunk's texcoord1), in meters up.
pub const ATTRIBUTE_MORPH_HEIGHT: MeshVertexAttribute =
    MeshVertexAttribute::new("LodMorphHeight", 990_412_771, VertexFormat::Float32);

#[cfg(target_os = "macos")]
pub type LodLandMaterial = LodLand;
#[cfg(not(target_os = "macos"))]
pub type LodLandMaterial = ExtendedMaterial<StandardMaterial, LodLand>;

#[derive(Clone, Copy, Debug, PartialEq, ShaderType, Reflect)]
pub struct LodLandParams {
    pub ambient: Vec4,
    pub sun_color: Vec4,
    pub sun_direction: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
    pub scale: Vec4,
    /// The loaded cells' area (center x, center z, half width in meters)
    /// and the drop in meters.
    pub high_detail: Vec4,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct LodLand {
    #[uniform(100)]
    pub params: LodLandParams,
    #[texture(101)]
    #[sampler(102)]
    pub base: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    pub normals: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    pub noise: Option<Handle<Image>>,
    /// This chunk's own values: x the geomorph factor (`GeomorphParams.x`),
    /// y and z its corner in its parent's texture and w how far it has
    /// faded from the parent's texture to its own (`LODTexParams.xyw`; 1:
    /// its own only).
    #[uniform(107)]
    pub chunk: Vec4,
    /// The parent's texture and normal map while fading from them.
    #[texture(108)]
    #[sampler(109)]
    pub parent_base: Option<Handle<Image>>,
    #[texture(110)]
    #[sampler(111)]
    pub parent_normals: Option<Handle<Image>>,
    /// x: the game's far clip plane in meters ([`GAME_FAR_CLIP`]).
    #[uniform(112)]
    pub clip: Vec4,
    /// The hour's light (`crate::shared_light::BUFFER`): the ambient, the
    /// sun and the fog (the params' own copies aren't read).
    #[storage(113, read_only)]
    pub shared: Handle<bevy::render::storage::ShaderStorageBuffer>,
}

#[cfg(target_os = "macos")]
impl Material for LodLand {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        specialize_lod_pipeline(descriptor, layout)
    }
}

#[cfg(not(target_os = "macos"))]
impl MaterialExtension for LodLand {
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
        specialize_lod_pipeline(descriptor, layout)
    }
}

fn specialize_lod_pipeline(
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayoutRef,
) -> Result<(), SpecializedMeshPipelineError> {
    if descriptor.vertex.shader != SHADER {
        return Ok(());
    }
    descriptor.vertex.buffers = vec![layout.0.get_layout(&[
        Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ATTRIBUTE_MORPH_HEIGHT.at_shader_location(3),
    ])?];
    Ok(())
}

fn lod_land(material: &LodLandMaterial) -> &LodLand {
    #[cfg(target_os = "macos")]
    {
        material
    }
    #[cfg(not(target_os = "macos"))]
    {
        &material.extension
    }
}

fn lod_land_mut(material: &mut LodLandMaterial) -> &mut LodLand {
    #[cfg(target_os = "macos")]
    {
        material
    }
    #[cfg(not(target_os = "macos"))]
    {
        &mut material.extension
    }
}

pub fn set_high_detail(material: &mut LodLandMaterial, detail: Vec4) {
    lod_land_mut(material).params.high_detail = detail;
}

pub fn set_parent_textures(
    material: &mut LodLandMaterial,
    offset: [f32; 2],
    base: Option<Handle<Image>>,
    normals: Option<Handle<Image>>,
) {
    let land = lod_land_mut(material);
    land.chunk = Vec4::new(land.chunk.x, offset[0], offset[1], 0.0);
    land.parent_base = base;
    land.parent_normals = normals;
}

pub fn chunk_values(material: &LodLandMaterial) -> Vec4 {
    lod_land(material).chunk
}

pub fn set_chunk_values(material: &mut LodLandMaterial, chunk: Vec4) {
    lod_land_mut(material).chunk = chunk;
}

pub fn clear_parent_textures(material: &mut LodLandMaterial) {
    let land = lod_land_mut(material);
    land.parent_base = None;
    land.parent_normals = None;
}

/// A chunk as a Bevy mesh (positions in Bevy's space).
pub fn chunk_mesh(chunk: &LodChunk) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let positions: Vec<[f32; 3]> = chunk.positions.iter().map(|&p| space::point(p)).collect();
    let count = positions.len();
    let uvs = if chunk.uvs.len() == count {
        chunk.uvs.clone()
    } else {
        vec![[0.0, 0.0]; count]
    };
    // Bevy's up is the game's z.
    let morph: Vec<f32> = if chunk.morph_heights.len() == count {
        chunk
            .morph_heights
            .iter()
            .map(|h| h * space::METERS_PER_UNIT)
            .collect()
    } else {
        positions.iter().map(|p| p[1]).collect()
    };
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(ATTRIBUTE_MORPH_HEIGHT, morph);
    // Bevy's standard pipeline expects normals for some passes; the
    // shader takes them from the normal map.
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; count]);
    mesh.insert_indices(Indices::U16(chunk.indices.clone()));
    mesh
}

/// How far inside the loaded area's edge the lowering stops, in game
/// units: the Goodsprings recording's `HighDetailRange` was the middle of
/// the player's cell and 10225 = 2.5 cells − 15 each way, so distant-land
/// points on the area's very edge stay up (the shader tests `<`). The 15
/// is read from that one recording, not traced in the code.
const HIGH_DETAIL_INSET: f32 = 15.0;

/// The loaded cells' area for the shader (`HighDetailRange`): centred on
/// square `here`, `radius` squares out on every side less
/// [`HIGH_DETAIL_INSET`], in Bevy's meters, with the drop.
pub fn high_detail(here: (i32, i32), radius: i32) -> Vec4 {
    let size = world::land::CELL_SIZE;
    let center = space::point([
        (here.0 as f32 + 0.5) * size,
        (here.1 as f32 + 0.5) * size,
        0.0,
    ]);
    let half = ((radius as f32 + 0.5) * size - HIGH_DETAIL_INSET) * space::METERS_PER_UNIT;
    Vec4::new(
        center[0],
        center[2],
        half,
        LOD_DROP * space::METERS_PER_UNIT,
    )
}

/// A chunk on screen (or kept hidden): its entity, material and textures
/// (lent to its quarters while they fade from them).
pub struct SpawnedChunk {
    pub entity: Entity,
    pub material: Handle<LodLandMaterial>,
    pub base: Option<Handle<Image>>,
    pub normals: Option<Handle<Image>>,
}

impl Spawner<'_, '_> {
    /// A chunk of distant land, hidden until shown.
    pub fn spawn_chunk(
        &mut self,
        chunk: &LodChunk,
        params: LodLandParams,
        noise: Option<Handle<Image>>,
    ) -> SpawnedChunk {
        let base = chunk.diffuse.as_ref().and_then(|t| self.upload(t));
        let normals = chunk.normals.as_ref().and_then(|t| self.upload(t));
        let land = LodLand {
            params,
            base: base.clone(),
            normals: normals.clone(),
            noise,
            chunk: Vec4::new(1.0, 0.0, 0.0, 1.0),
            parent_base: None,
            parent_normals: None,
            clip: Vec4::new(GAME_FAR_CLIP * space::METERS_PER_UNIT, 0.0, 0.0, 0.0),
            shared: crate::shared_light::BUFFER,
        };
        #[cfg(target_os = "macos")]
        let material = self.lod_materials.add(land);
        #[cfg(not(target_os = "macos"))]
        let material = self.lod_materials.add(ExtendedMaterial {
            base: StandardMaterial::default(),
            extension: land,
        });
        let mesh = self.meshes.add(chunk_mesh(chunk));
        let entity = self
            .commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
                Visibility::Hidden,
                SceneEntity,
            ))
            .id();
        SpawnedChunk {
            entity,
            material,
            base,
            normals,
        }
    }
}

/// Where a chunk's model stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    /// Not loaded yet.
    Pending,
    /// The game has no model there.
    Empty,
    Loaded,
}

/// What the distant land shows this frame.
#[derive(Debug, Default, PartialEq)]
pub struct Showing {
    pub shown: HashSet<LodNode>,
    /// Quarters that replace their parent this frame, with that parent:
    /// they start out in its texture.
    pub fades: Vec<(LodNode, LodNode)>,
}

/// Which chunks to show, as the game's terrain manager does it
/// (`006fdaa0`): the nodes `wanted` (`world::lod::drawn_nodes`), except
/// that a split node stays on screen until all four of its quarters are
/// loaded (they then appear together, each starting in its quarter of the
/// parent's texture), and a node whose quarters are merged back keeps them
/// on screen until it's loaded itself. `before`: what was shown last frame.
pub fn choose_shown(
    lod: &LodSettings,
    wanted: &HashSet<LodNode>,
    state: impl Fn(LodNode) -> ChunkState,
    before: &HashSet<LodNode>,
) -> Showing {
    let mut shown = HashSet::new();
    visit(lod.root_node(), lod, wanted, &state, before, &mut shown);
    let mut fades = Vec::new();
    for &node in &shown {
        if before.contains(&node) {
            continue;
        }
        if let Some(parent) = lod.parent(node) {
            if before.contains(&parent) && !shown.contains(&parent) {
                fades.push((node, parent));
            }
        }
    }
    fades.sort();
    Showing { shown, fades }
}

fn ready(
    node: LodNode,
    lod: &LodSettings,
    wanted: &HashSet<LodNode>,
    state: &impl Fn(LodNode) -> ChunkState,
) -> bool {
    if wanted.contains(&node) {
        return state(node) != ChunkState::Pending;
    }
    if node.level <= lod.min_level {
        return true;
    }
    node.children()
        .into_iter()
        .all(|c| ready(c, lod, wanted, state))
}

fn visit(
    node: LodNode,
    lod: &LodSettings,
    wanted: &HashSet<LodNode>,
    state: &impl Fn(LodNode) -> ChunkState,
    before: &HashSet<LodNode>,
    out: &mut HashSet<LodNode>,
) {
    if wanted.contains(&node) {
        match state(node) {
            ChunkState::Loaded => {
                out.insert(node);
            }
            ChunkState::Empty => {}
            // Merging: what was shown inside it stays until it's loaded.
            ChunkState::Pending => {
                for &b in before {
                    if inside(b, node) && state(b) == ChunkState::Loaded {
                        out.insert(b);
                    }
                }
            }
        }
        return;
    }
    if node.level <= lod.min_level {
        return;
    }
    let children = node.children();
    if children.iter().all(|&c| ready(c, lod, wanted, state)) {
        for c in children {
            visit(c, lod, wanted, state, before, out);
        }
    } else if state(node) == ChunkState::Loaded {
        // Split, but not all its quarters are loaded yet: it stays.
        out.insert(node);
    } else {
        for c in children {
            visit(c, lod, wanted, state, before, out);
        }
    }
}

/// Whether `inner` is a smaller node inside `outer`.
fn inside(inner: LodNode, outer: LodNode) -> bool {
    let size = outer.level as i32;
    inner.level < outer.level
        && (outer.x..outer.x + size).contains(&inner.x)
        && (outer.y..outer.y + size).contains(&inner.y)
}

pub struct LodPlugin;

impl Plugin for LodPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER, "lod_land.wgsl", Shader::from_wgsl);
        #[cfg(target_os = "macos")]
        app.add_plugins(MaterialPlugin::<LodLandMaterial> {
            prepass_enabled: false,
            shadows_enabled: false,
            ..default()
        });
        #[cfg(not(target_os = "macos"))]
        app.add_plugins(MaterialPlugin::<LodLandMaterial>::default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_loaded_area_is_centred_on_the_players_square() {
        let h = high_detail((0, 0), 2);
        let m = space::METERS_PER_UNIT;
        assert!((h.x - 2048.0 * m).abs() < 1e-4);
        // Game north is Bevy's -z.
        assert!((h.y + 2048.0 * m).abs() < 1e-4);
        // Recorded: 10225 units each way with five squares loaded.
        assert!((h.z - 10225.0 * m).abs() < 1e-3);
    }

    #[test]
    fn the_sky_stands_behind_all_the_distant_land_the_game_draws() {
        // The game draws the sky at its far plane and the land over it; here
        // the dome must be past the game's far clip plane, or land beyond
        // the dome would be covered by sky.
        const { assert!(crate::SKY_RADIUS_METERS > GAME_FAR_CLIP * space::METERS_PER_UNIT) };
    }

    #[test]
    fn chunks_carry_their_geomorph_heights() {
        let chunk = LodChunk {
            cell: (0, 0),
            level: 8,
            positions: vec![[0.0, 0.0, 900.0], [100.0, 0.0, 900.0], [0.0, 100.0, 900.0]],
            morph_heights: vec![800.0, 850.0, 900.0],
            uvs: vec![[1.0, 0.0], [0.0, 0.0], [1.0, 1.0]],
            indices: vec![0, 1, 2],
            diffuse: None,
            normals: None,
        };
        let mesh = chunk_mesh(&chunk);
        let Some(bevy::render::mesh::VertexAttributeValues::Float32(morph)) =
            mesh.attribute(ATTRIBUTE_MORPH_HEIGHT)
        else {
            panic!("no geomorph heights");
        };
        let m = space::METERS_PER_UNIT;
        assert_eq!(morph, &vec![800.0 * m, 850.0 * m, 900.0 * m]);
    }

    /// A tree of one level-16 root over level 8 and 4.
    fn small() -> LodSettings {
        LodSettings {
            min_level: 4,
            max_level: 16,
            root_level: 16,
            root: (0, 0),
            last_cell: (15, 15),
            object_level: 4,
        }
    }

    fn node(level: u32, x: i32, y: i32) -> LodNode {
        LodNode { level, x, y }
    }

    #[test]
    fn a_split_node_stays_until_all_its_quarters_are_loaded() {
        let lod = small();
        let root = node(16, 0, 0);
        let quarters: HashSet<LodNode> = root.children().into_iter().collect();
        let before: HashSet<LodNode> = [root].into_iter().collect();
        // Three of four loaded: the root stays, alone.
        let missing = node(8, 8, 8);
        let state = |n: LodNode| {
            if n == missing {
                ChunkState::Pending
            } else {
                ChunkState::Loaded
            }
        };
        let s = choose_shown(&lod, &quarters, state, &before);
        assert_eq!(s.shown, before);
        assert!(s.fades.is_empty());
        // All four: they replace it together, each fading from it.
        let s = choose_shown(&lod, &quarters, |_| ChunkState::Loaded, &before);
        assert_eq!(s.shown, quarters);
        assert_eq!(s.fades.len(), 4);
        assert!(s.fades.iter().all(|&(_, p)| p == root));
        // Next frame nothing new fades.
        let s2 = choose_shown(&lod, &quarters, |_| ChunkState::Loaded, &s.shown);
        assert!(s2.fades.is_empty());
        // A quarter the game has no model for counts as ready.
        let s = choose_shown(
            &lod,
            &quarters,
            |n| {
                if n == missing {
                    ChunkState::Empty
                } else {
                    ChunkState::Loaded
                }
            },
            &before,
        );
        assert_eq!(s.shown.len(), 3);
    }

    #[test]
    fn merged_quarters_stay_until_their_whole_is_loaded() {
        let lod = small();
        let root = node(16, 0, 0);
        let quarters: HashSet<LodNode> = root.children().into_iter().collect();
        let wanted: HashSet<LodNode> = [root].into_iter().collect();
        let state = |n: LodNode| {
            if n == root {
                ChunkState::Pending
            } else {
                ChunkState::Loaded
            }
        };
        let s = choose_shown(&lod, &wanted, state, &quarters);
        assert_eq!(s.shown, quarters);
        let s = choose_shown(&lod, &wanted, |_| ChunkState::Loaded, &quarters);
        assert_eq!(s.shown, wanted);
        assert!(s.fades.is_empty());
    }
}
