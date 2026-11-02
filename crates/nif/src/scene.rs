//! Turning a NIF's node tree into a flat list of drawable meshes.

use std::collections::BTreeMap;

use crate::blocks::{
    AlphaProperty, AvObject, Block, Geometry, MaterialProperty, ShaderProperty, ZBufferProperty,
};
use crate::error::{Error, Result};
use crate::file::Nif;
use crate::math::{Transform, Vec3};

/// Deeper nesting than this only happens in corrupt files.
const MAX_DEPTH: usize = 64;

/// One drawable piece of a model.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub name: String,
    /// Index of the shape block this came from.
    pub block: usize,
    /// From the mesh's own space to the model's space (all parent nodes
    /// applied).
    pub transform: Transform,
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    /// The stored tangent space (see `GeometryData::tangents`); empty when
    /// the file has none.
    pub tangents: Vec<Vec3>,
    pub bitangents: Vec<Vec3>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub triangles: Vec<[u16; 3]>,
    /// The bounding sphere the file stores with the vertices (centre and
    /// radius, in the mesh's own space, before `transform`).
    pub bound: (Vec3, f32),
    /// The nodes from the top of the file down to the shape itself, each
    /// with its local transform as `transform` composes them (the top
    /// node's left out for placed scenes): what an animation moves.
    pub nodes: Vec<(String, Transform)>,
    /// Under an `NiBillboardNode`: its place in [`Self::nodes`] (the
    /// deepest one) and its billboard mode.
    pub billboard: Option<(usize, u16)>,
    /// Texture paths from the shader: diffuse first, then normal map, glow,
    /// height, environment map and environment mask where present.
    pub textures: Vec<String>,
    pub shader: Option<ShaderProperty>,
    pub material: Option<MaterialProperty>,
    pub alpha: Option<AlphaProperty>,
    /// The `NiZBufferProperty`, when the mesh or a node above it has one.
    pub zbuffer: Option<ZBufferProperty>,
    pub double_sided: bool,
    /// Bound to a skeleton; positions are in the bind pose.
    pub skinned: bool,
    /// The skin (bones, weights, partitions) when it could be read.
    pub skin: Option<crate::skin::Skin>,
    /// Block types of the properties that apply to this mesh, in order.
    pub property_types: Vec<String>,
}

impl Mesh {
    pub fn diffuse_texture(&self) -> Option<&str> {
        self.textures
            .first()
            .map(String::as_str)
            .filter(|t| !t.is_empty())
    }

    /// The normal map (second slot of a lit shader's texture set). Its
    /// alpha is the specular mask.
    pub fn normal_texture(&self) -> Option<&str> {
        self.lit_slot(1)
    }

    /// The glow map (third slot of a lit shader's texture set): where and
    /// in what color the surface lights itself.
    pub fn glow_texture(&self) -> Option<&str> {
        self.lit_slot(2)
    }

    /// The environment (reflection) cube map: fifth slot of a lit shader's
    /// texture set.
    pub fn environment_texture(&self) -> Option<&str> {
        self.lit_slot(4)
    }

    /// The environment mask (sixth slot): where reflections show, in its
    /// red channel.
    pub fn environment_mask_texture(&self) -> Option<&str> {
        self.lit_slot(5)
    }

    fn lit_slot(&self, slot: usize) -> Option<&str> {
        let lit = self.shader.as_ref().is_some_and(|s| s.lit);
        self.textures
            .get(slot)
            .filter(|t| lit && !t.is_empty())
            .map(String::as_str)
    }

    /// Whether any of the sequences moves a node above (or at) this mesh.
    pub fn moved_by(&self, sequences: &[crate::anim::Sequence]) -> bool {
        sequences.iter().any(|s| {
            s.tracks.iter().any(|t| {
                self.nodes
                    .iter()
                    .any(|(n, _)| n.eq_ignore_ascii_case(&t.node))
            })
        })
    }

    /// From the mesh's own space to the model's with sequences' poses laid
    /// over its nodes (each at its own time, later ones over earlier ones,
    /// as `anim::posed_layers` does for skeletons): [`Self::transform`]
    /// when nothing moves them.
    pub fn posed_transform(&self, layers: &[(&crate::anim::Sequence, f32)]) -> Transform {
        posed_chain(&self.nodes, layers)
    }

    /// The stored bounding sphere in the model's space: its centre and
    /// radius.
    pub fn model_bound(&self) -> (Vec3, f32) {
        (
            self.transform.apply_point(self.bound.0),
            self.bound.1 * self.transform.scale,
        )
    }

    /// Vertex positions in the model's space.
    pub fn model_positions(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.positions
            .iter()
            .map(|&p| self.transform.apply_point(p))
    }
}

/// A chain of nodes (top first, each with its own local transform)
/// composed, with sequences' poses laid over the nodes they move (each at
/// its own time; a later layer over an earlier one).
pub fn posed_chain(
    nodes: &[(String, Transform)],
    layers: &[(&crate::anim::Sequence, f32)],
) -> Transform {
    let mut world = Transform::IDENTITY;
    for (name, own) in nodes {
        let mut local = *own;
        for (sequence, time) in layers {
            if let Some(track) = sequence
                .tracks
                .iter()
                .find(|t| t.node.eq_ignore_ascii_case(name))
            {
                local = track.sample(*time).apply(&local);
            }
        }
        world = world.then_child(&local);
    }
    world
}

/// Everything drawable in a NIF, plus what was left out.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    /// Scene objects of types not drawn yet (particles, lights...), by type.
    pub unhandled: BTreeMap<String, usize>,
    /// References to blocks that don't exist.
    pub invalid_references: usize,
    /// Triangles dropped because they pointed past the vertex list.
    pub dropped_triangles: usize,
    /// Property blocks that failed to decode, by type. The meshes using
    /// them are kept, just without what that property would have added.
    pub unreadable_properties: BTreeMap<String, usize>,
    /// The top node's own transform, when it isn't the identity. The game
    /// replaces it with the placement's transform when a model is placed
    /// in the world, so [`Nif::placed_scene`] leaves it out.
    pub root_transform: Option<Transform>,
}

impl Scene {
    pub fn triangle_count(&self) -> usize {
        self.meshes.iter().map(|m| m.triangles.len()).sum()
    }
}

/// How a scene is being collected.
struct Walk {
    /// Apply the top node's own transform (as viewers do) or not (as the
    /// game does for placed models).
    apply_root: bool,
}

/// The nodes from the top node down to a shape (each with its own
/// transform), and the deepest billboard node among them (its place in that
/// list and its mode).
type NodePath = (Vec<(String, Transform)>, Option<(usize, u16)>);

fn is_identity(t: &Transform) -> bool {
    let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
    close(t.scale, 1.0)
        && t.translation.iter().all(|&v| v.abs() < 1e-3)
        && (0..3).all(|i| (0..3).all(|j| close(t.rotation[i][j], if i == j { 1.0 } else { 0.0 })))
}

/// Hidden objects and editor-only markers aren't drawn in game.
fn is_invisible(av: &AvObject) -> bool {
    av.is_hidden() || av.net.name.to_ascii_lowercase().starts_with("editormarker")
}

impl Nif {
    /// Walks the node tree from the root blocks and collects every visible
    /// shape. Collision geometry, hidden objects and editor markers are
    /// skipped; switch and LOD nodes contribute only their active child.
    ///
    /// Every node's transform is applied, the top node's included, as
    /// viewers like NifSkope show the file.
    pub fn scene(&self) -> Result<Scene> {
        self.build_scene(true)
    }

    /// The scene as the game shows the model when it's placed in the world:
    /// the engine overwrites the top node's transform with the placement's
    /// position, rotation and scale, so whatever the file stores there is
    /// left out. [`Scene::root_transform`] still reports it.
    pub fn placed_scene(&self) -> Result<Scene> {
        self.build_scene(false)
    }

    fn build_scene(&self, apply_root: bool) -> Result<Scene> {
        let mut scene = Scene::default();
        let mut visited = vec![false; self.blocks().len()];
        let walk = Walk { apply_root };
        for &root in self.roots() {
            self.visit(
                &walk,
                root,
                &Transform::IDENTITY,
                &[],
                None,
                &[],
                0,
                &mut visited,
                &mut scene,
            )?;
        }
        Ok(scene)
    }

    /// A node's local transform, with the top node's handled as asked.
    /// A node's transform in the placed model's space (the top node's own
    /// left out, as [`Self::placed_scene`] does), by name, ignoring case:
    /// the first met walking down from the roots, hidden or not (the game
    /// looks nodes up by name, e.g. a weapon's `ProjectileNode`).
    pub fn placed_node(&self, name: &str) -> Option<Transform> {
        let mut visited = vec![false; self.blocks().len()];
        self.roots()
            .iter()
            .find_map(|&root| self.find_node(root, name, &Transform::IDENTITY, 0, &mut visited))
    }

    fn find_node(
        &self,
        reference: i32,
        name: &str,
        parent: &Transform,
        depth: usize,
        visited: &mut [bool],
    ) -> Option<Transform> {
        let index = self.valid_index(reference)?;
        if visited[index] || depth > MAX_DEPTH {
            return None;
        }
        visited[index] = true;
        let Ok(Block::Node(node)) = self.block(index) else {
            return None;
        };
        let local = if depth == 0 {
            Transform::IDENTITY
        } else {
            node.av.transform
        };
        let world = parent.then_child(&local);
        if node.av.net.name.eq_ignore_ascii_case(name) {
            return Some(world);
        }
        node.children
            .iter()
            .find_map(|&c| self.find_node(c, name, &world, depth + 1, visited))
    }

    fn local_transform(
        &self,
        walk: &Walk,
        depth: usize,
        own: &Transform,
        scene: &mut Scene,
    ) -> Transform {
        if depth > 0 {
            return *own;
        }
        if !is_identity(own) && scene.root_transform.is_none() {
            scene.root_transform = Some(*own);
        }
        if walk.apply_root {
            *own
        } else {
            Transform::IDENTITY
        }
    }

    fn valid_index(&self, reference: i32) -> Option<usize> {
        usize::try_from(reference)
            .ok()
            .filter(|&i| i < self.blocks().len())
    }

    #[allow(clippy::too_many_arguments)]
    fn visit(
        &self,
        walk: &Walk,
        reference: i32,
        parent: &Transform,
        path: &[(String, Transform)],
        billboard: Option<(usize, u16)>,
        inherited: &[i32],
        depth: usize,
        visited: &mut [bool],
        scene: &mut Scene,
    ) -> Result<()> {
        if reference < 0 {
            return Ok(());
        }
        let Some(index) = self.valid_index(reference) else {
            scene.invalid_references += 1;
            return Ok(());
        };
        if visited[index] {
            return Ok(());
        }
        if depth > MAX_DEPTH {
            return Err(Error::Malformed {
                offset: self.blocks()[index].offset,
                reason: format!("nodes are nested more than {MAX_DEPTH} levels deep"),
            });
        }
        visited[index] = true;
        if self.block_type(index) == "RootCollisionNode" {
            return Ok(());
        }

        match self.block(index)? {
            Block::Node(node) => {
                if is_invisible(&node.av) {
                    return Ok(());
                }
                let local = self.local_transform(walk, depth, &node.av.transform, scene);
                let world = parent.then_child(&local);
                let mut path = path.to_vec();
                path.push((node.av.net.name.clone(), local));
                // The deepest billboard node above a shape is the one it turns
                // with.
                let billboard = node
                    .billboard
                    .map(|mode| (path.len() - 1, mode))
                    .or(billboard);
                let properties = [inherited, &node.av.properties].concat();
                let children: Vec<i32> = match node.active_child {
                    Some(k) => node.children.get(k).copied().into_iter().collect(),
                    None => node.children,
                };
                for child in children {
                    self.visit(
                        walk,
                        child,
                        &world,
                        &path,
                        billboard,
                        &properties,
                        depth + 1,
                        visited,
                        scene,
                    )?;
                }
            }
            Block::Geometry(geometry) => {
                if !is_invisible(&geometry.av) {
                    let local = self.local_transform(walk, depth, &geometry.av.transform, scene);
                    let world = parent.then_child(&local);
                    let mut path = path.to_vec();
                    path.push((geometry.av.net.name.clone(), local));
                    let properties = [inherited, &geometry.av.properties].concat();
                    self.add_mesh(
                        index,
                        geometry,
                        world,
                        (path, billboard),
                        &properties,
                        scene,
                    )?;
                }
            }
            _ => {
                *scene
                    .unhandled
                    .entry(self.block_type(index).to_string())
                    .or_default() += 1;
            }
        }
        Ok(())
    }

    fn add_mesh(
        &self,
        index: usize,
        geometry: Geometry,
        transform: Transform,
        (nodes, billboard): NodePath,
        properties: &[i32],
        scene: &mut Scene,
    ) -> Result<()> {
        let Some(data_index) = self.valid_index(geometry.data) else {
            scene.invalid_references += 1;
            return Ok(());
        };
        let Block::GeometryData(mut data) = self.block(data_index)? else {
            let label = format!("{} (as shape data)", self.block_type(data_index));
            *scene.unhandled.entry(label).or_default() += 1;
            return Ok(());
        };
        let vertex_count = data.positions.len();
        let before = data.triangles.len();
        data.triangles
            .retain(|t| t.iter().all(|&v| usize::from(v) < vertex_count));
        scene.dropped_triangles += before - data.triangles.len();

        let mut mesh = Mesh {
            name: geometry.av.net.name,
            block: index,
            transform,
            positions: data.positions,
            normals: data.normals,
            tangents: data.tangents,
            bitangents: data.bitangents,
            uvs: data.uvs,
            colors: data.colors,
            triangles: data.triangles,
            bound: (data.center, data.radius),
            nodes,
            billboard,
            textures: Vec::new(),
            shader: None,
            material: None,
            alpha: None,
            zbuffer: None,
            double_sided: false,
            skinned: geometry.skin >= 0,
            skin: None,
            property_types: Vec::new(),
        };
        if geometry.skin >= 0 {
            match self.skin(geometry.skin, vertex_count) {
                Ok(skin) => mesh.skin = skin,
                Err(_) => *scene.unhandled.entry("unreadable skin".into()).or_default() += 1,
            }
        }

        // Properties on parent nodes apply to their children; a shape's own
        // properties come last and override them.
        let mut texturing_base: Option<String> = None;
        for &reference in properties {
            let Some(p) = self.valid_index(reference) else {
                continue;
            };
            let type_name = self.block_type(p);
            mesh.property_types.push(type_name.to_string());
            let Ok(block) = self.block(p) else {
                *scene
                    .unreadable_properties
                    .entry(type_name.to_string())
                    .or_default() += 1;
                continue;
            };
            match block {
                Block::Shader(shader) => {
                    mesh.textures = if shader.lit {
                        match self.valid_index(shader.texture_set).map(|t| self.block(t)) {
                            Some(Ok(Block::TextureSet(set))) => set.textures,
                            _ => Vec::new(),
                        }
                    } else {
                        shader
                            .file_name
                            .clone()
                            .into_iter()
                            .filter(|f| !f.is_empty())
                            .collect()
                    };
                    mesh.shader = Some(shader);
                }
                Block::Texturing(texturing) => {
                    texturing_base = match self
                        .valid_index(texturing.base_texture)
                        .map(|t| self.block(t))
                    {
                        Some(Ok(Block::SourceTexture(source))) => {
                            Some(source.file_name).filter(|f| !f.is_empty())
                        }
                        _ => None,
                    };
                }
                Block::Material(material) => mesh.material = Some(material),
                Block::Alpha(alpha) => mesh.alpha = Some(alpha),
                Block::ZBuffer(zbuffer) => mesh.zbuffer = Some(zbuffer),
                Block::Stencil(stencil) => mesh.double_sided = stencil.double_sided(),
                _ => {}
            }
        }
        // Meshes using the older material style (hair, some ported assets)
        // name their texture through NiTexturingProperty instead.
        if mesh.textures.iter().all(String::is_empty) {
            if let Some(base) = texturing_base {
                mesh.textures = vec![base];
            }
        }
        scene.meshes.push(mesh);
        Ok(())
    }
}
