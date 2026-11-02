//! Skinning: how a shape bends with a skeleton. A skinned shape names a
//! skin instance (`NiSkinInstance`, or `BSDismemberSkinInstance` with body
//! parts), which lists its bones by node and points at the bind data
//! (`NiSkinData`: each bone's transform from the skin to the bone, and the
//! weight each bone has on each vertex) and at the partitions
//! (`NiSkinPartition`: the triangles split by the bones they use).
//! Layouts follow the community NIF format description for 20.2.0.7.

use crate::blocks::Block;
use crate::error::Result;
use crate::file::Nif;
use crate::math::{Transform, Vec3};
use crate::reader::Reader;

/// Body parts in this range are the gore caps shown where a limb has been
/// shot off (`BSDismemberSkinInstance`'s section caps, 100s, and torso
/// caps, 200s); the game hides them on an intact body. The body itself
/// uses 0–13 and the thousands (`upperbody.nif`: 0, 3, 5, 7, 10, 3000,
/// 5000, 7000, 10000; its caps 101–110 and 201–210).
pub const CAP_BODY_PARTS: std::ops::Range<u16> = 100..300;

/// One place to use a piece of furniture from (see
/// [`Nif::furniture_markers`]): in the model's space, its heading in
/// radians, and the marker's number (chairs 11 left, 12 right, 13 back,
/// 14 front, as the idle tree's chair exits name them).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FurnitureMarker {
    pub offset: Vec3,
    pub heading: f32,
    pub marker: u8,
}

/// The box a skeleton carries for its actor's collision (see
/// [`Nif::bound`]): its centre and half extents, in the model's space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound {
    pub center: Vec3,
    pub half_extents: Vec3,
}

/// A shape's skin.
#[derive(Debug, Clone, PartialEq)]
pub struct Skin {
    /// The bones by node name, in the skin's order.
    pub bones: Vec<String>,
    /// For each bone, from the skin's space to the bone's (the inverse of
    /// the bone's bind pose, with the skin's own offset).
    pub bone_transforms: Vec<Transform>,
    /// `NiSkinData`'s overall skin transform.
    pub skin_transform: Transform,
    /// For each vertex, the bones that move it and how much.
    pub weights: Vec<Vec<(u16, f32)>>,
    /// The dismemberment partitions, when the skin has them.
    pub partitions: Vec<Partition>,
}

/// One partition of a skinned shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Partition {
    /// `BSDismemberSkinInstance`: the body part (see [`CAP_BODY_PARTS`])
    /// and its flags; 0 for plain skins.
    pub body_part: u16,
    pub flags: u16,
    /// Its triangles, in the shape's vertex numbering.
    pub triangles: Vec<[u16; 3]>,
}

impl Partition {
    pub fn is_cap(&self) -> bool {
        CAP_BODY_PARTS.contains(&self.body_part)
    }
}

fn read_transform(r: &mut Reader) -> Result<Transform> {
    let rotation = r.mat3("a skin rotation")?;
    let translation = r.vec3("a skin translation")?;
    let scale = r.f32("a skin scale")?;
    Ok(Transform {
        rotation,
        translation,
        scale,
    })
}

/// `NiSkinInstance` / `BSDismemberSkinInstance`.
struct Instance {
    data: i32,
    partition: i32,
    bones: Vec<i32>,
    /// (flags, body part) per partition.
    parts: Vec<(u16, u16)>,
}

fn read_instance(r: &mut Reader, dismember: bool) -> Result<Instance> {
    let data = r.i32("the skin data reference")?;
    let partition = r.i32("the skin partition reference")?;
    r.i32("the skeleton root")?;
    let bones = r.ref_list("the bones")?;
    let parts = if dismember {
        let n = r.u32("the partition count")? as usize;
        r.counted(n, 4, "partitions", |r| {
            Ok((r.u16("a part flag")?, r.u16("a body part")?))
        })?
    } else {
        Vec::new()
    };
    Ok(Instance {
        data,
        partition,
        bones,
        parts,
    })
}

struct Data {
    skin_transform: Transform,
    bones: Vec<(Transform, Vec<(u16, f32)>)>,
}

fn read_data(r: &mut Reader) -> Result<Data> {
    let skin_transform = read_transform(r)?;
    let n = r.u32("the bone count")? as usize;
    let has_weights = r.bool("the has-weights flag")?;
    let mut bones = Vec::with_capacity(n.min(1024));
    for _ in 0..n {
        let transform = read_transform(r)?;
        r.vec3("a bounding sphere")?;
        r.f32("a bounding sphere")?;
        let count = usize::from(r.u16("the weighted vertex count")?);
        let weights = if has_weights {
            r.counted(count, 6, "vertex weights", |r| {
                Ok((r.u16("a vertex index")?, r.f32("a weight")?))
            })?
        } else {
            Vec::new()
        };
        bones.push((transform, weights));
    }
    Ok(Data {
        skin_transform,
        bones,
    })
}

/// One `NiSkinPartition` block: per partition its triangles (in the
/// shape's numbering) and, when the skin data has none, its weights.
struct PartitionData {
    triangles: Vec<[u16; 3]>,
    /// (shape vertex, [(skin bone, weight)]).
    weights: Vec<(u16, Vec<(u16, f32)>)>,
}

fn read_partitions(r: &mut Reader) -> Result<Vec<PartitionData>> {
    let n = r.u32("the partition count")? as usize;
    let mut out = Vec::with_capacity(n.min(256));
    for _ in 0..n {
        let vertices = usize::from(r.u16("the vertex count")?);
        let triangles = usize::from(r.u16("the triangle count")?);
        let bones = usize::from(r.u16("the bone count")?);
        let strips = usize::from(r.u16("the strip count")?);
        let per_vertex = usize::from(r.u16("the weights per vertex")?);
        let bone_list = r.counted(bones, 2, "partition bones", |r| r.u16("a bone"))?;
        let map = if r.bool("the has-vertex-map flag")? {
            r.counted(vertices, 2, "the vertex map", |r| r.u16("a vertex"))?
        } else {
            (0..vertices as u16).collect()
        };
        let weights = if r.bool("the has-weights flag")? {
            r.counted(vertices * per_vertex, 4, "partition weights", |r| {
                r.f32("a weight")
            })?
        } else {
            Vec::new()
        };
        let strip_lengths = r.counted(strips, 2, "strip lengths", |r| r.u16("a strip length"))?;
        let mut local: Vec<[u16; 3]> = Vec::new();
        if r.bool("the has-faces flag")? {
            if strips == 0 {
                local = r.counted(triangles, 6, "partition triangles", |r| {
                    Ok([r.u16("a corner")?, r.u16("a corner")?, r.u16("a corner")?])
                })?;
            } else {
                for len in strip_lengths {
                    let points =
                        r.counted(usize::from(len), 2, "a strip", |r| r.u16("a strip point"))?;
                    for k in 2..points.len() {
                        let t = if k % 2 == 0 {
                            [points[k - 2], points[k - 1], points[k]]
                        } else {
                            [points[k - 2], points[k], points[k - 1]]
                        };
                        if t[0] != t[1] && t[1] != t[2] && t[0] != t[2] {
                            local.push(t);
                        }
                    }
                }
            }
        }
        let bone_indices = if r.bool("the has-bone-indices flag")? {
            r.counted(vertices * per_vertex, 1, "bone indices", |r| {
                r.u8("a bone index")
            })?
        } else {
            Vec::new()
        };
        let to_shape = |i: u16| map.get(usize::from(i)).copied().unwrap_or(i);
        let mut part = PartitionData {
            triangles: local.iter().map(|t| t.map(to_shape)).collect(),
            weights: Vec::new(),
        };
        if weights.len() == vertices * per_vertex && bone_indices.len() == vertices * per_vertex {
            for (v, &shape_vertex) in map.iter().enumerate() {
                let list = (0..per_vertex)
                    .filter_map(|k| {
                        let w = weights[v * per_vertex + k];
                        let b = bone_list.get(usize::from(bone_indices[v * per_vertex + k]))?;
                        (w > 0.0).then_some((*b, w))
                    })
                    .collect();
                part.weights.push((shape_vertex, list));
            }
        }
        out.push(part);
    }
    Ok(out)
}

impl Nif {
    /// The skin a shape's skin reference points at, if it's one this reader
    /// understands. `vertices` is the shape's vertex count.
    pub fn skin(&self, reference: i32, vertices: usize) -> Result<Option<Skin>> {
        let Some(index) = self.reference(reference) else {
            return Ok(None);
        };
        let dismember = match self.block_type(index) {
            "NiSkinInstance" => false,
            "BSDismemberSkinInstance" => true,
            _ => return Ok(None),
        };
        let instance = read_instance(&mut self.reader(index), dismember)?;
        let Some(data_index) = self
            .reference(instance.data)
            .filter(|&i| self.block_type(i) == "NiSkinData")
        else {
            return Ok(None);
        };
        let data = read_data(&mut self.reader(data_index))?;
        let partitions = match self
            .reference(instance.partition)
            .filter(|&i| self.block_type(i) == "NiSkinPartition")
        {
            Some(i) => read_partitions(&mut self.reader(i))?,
            None => Vec::new(),
        };

        let bones = instance
            .bones
            .iter()
            .map(|&b| match self.reference(b).map(|i| self.block(i)) {
                Some(Ok(Block::Node(node))) => node.av.net.name,
                _ => String::new(),
            })
            .collect();
        let mut weights: Vec<Vec<(u16, f32)>> = vec![Vec::new(); vertices];
        let has_data_weights = data.bones.iter().any(|(_, w)| !w.is_empty());
        if has_data_weights {
            for (bone, (_, list)) in data.bones.iter().enumerate() {
                for &(v, w) in list {
                    if let Some(slot) = weights.get_mut(usize::from(v)) {
                        slot.push((bone as u16, w));
                    }
                }
            }
        } else {
            for part in &partitions {
                for (v, list) in &part.weights {
                    if let Some(slot) = weights.get_mut(usize::from(*v)) {
                        slot.clone_from(list);
                    }
                }
            }
        }
        let partitions = partitions
            .into_iter()
            .enumerate()
            .map(|(k, p)| {
                let (flags, body_part) = instance.parts.get(k).copied().unwrap_or((0, 0));
                Partition {
                    body_part,
                    flags,
                    triangles: p.triangles,
                }
            })
            .collect();
        Ok(Some(Skin {
            bones,
            bone_transforms: data.bones.into_iter().map(|(t, _)| t).collect(),
            skin_transform: data.skin_transform,
            weights,
            partitions,
        }))
    }

    /// The bone a model hangs from when worn (eyes, hats, glasses): the
    /// top node's `NiStringExtraData` named `Prn` (the eye models say
    /// `Bip01 Head`). Its pieces that aren't skinned follow that bone.
    pub fn attach_bone(&self) -> Option<String> {
        let root = self.roots().first().and_then(|&r| self.reference(r))?;
        let Block::Node(node) = self.block(root).ok()? else {
            return None;
        };
        node.av.net.extra_data.iter().find_map(|&e| {
            let index = self.reference(e)?;
            if self.block_type(index) != "NiStringExtraData" {
                return None;
            }
            let mut r = self.reader(index);
            let string = |i: i32| {
                usize::try_from(i)
                    .ok()
                    .and_then(|i| self.header().strings.get(i))
                    .cloned()
            };
            let name = string(r.i32("a name").ok()?)?;
            let value = string(r.i32("a string").ok()?)?;
            (name == "Prn").then_some(value)
        })
    }

    /// Where people use a piece of furniture: the top node's
    /// `BSFurnitureMarker` (a name, a count, then per position an offset
    /// (3 × f32), a heading u16 in thousandths of a radian and two marker
    /// numbers u8). Read from `SubChairDirty01.nif` (Doc Mitchell's chair):
    /// left 11 at (-53.7, 7.5, -35.6) heading 1.570, right 12 at (49.0,
    /// 7.6, -35.6) heading 4.712, front 14 at (-2.0, 62.8, -37.2) heading
    /// 3.141; the idle tree's chair branches are for markers 11 to 14.
    pub fn furniture_markers(&self) -> Vec<FurnitureMarker> {
        let Some(root) = self.roots().first().and_then(|&r| self.reference(r)) else {
            return Vec::new();
        };
        let Ok(Block::Node(node)) = self.block(root) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for &e in &node.av.net.extra_data {
            let Some(index) = self.reference(e) else {
                continue;
            };
            if self.block_type(index) != "BSFurnitureMarker" {
                continue;
            }
            let mut r = self.reader(index);
            let read = |r: &mut crate::reader::Reader<'_>| -> Result<Vec<FurnitureMarker>> {
                r.i32("a name")?;
                let n = r.u32("a position count")?;
                let mut list = Vec::new();
                for _ in 0..n.min(64) {
                    let offset = r.vec3("an offset")?;
                    let heading = f32::from(r.u16("a heading")?) / 1000.0;
                    let marker = r.u8("a marker")?;
                    r.u8("a second marker")?;
                    list.push(FurnitureMarker {
                        offset,
                        heading,
                        marker,
                    });
                }
                Ok(list)
            };
            out.extend(read(&mut r).unwrap_or_default());
        }
        out
    }

    /// The top node's `BSBound` (a name, then the centre and the half
    /// extents, 3 × f32 each): the box the game sizes a creature's
    /// character controller from (`00c55170`, `findings\physics.md`).
    pub fn bound(&self) -> Option<Bound> {
        let root = self.roots().first().and_then(|&r| self.reference(r))?;
        let Block::Node(node) = self.block(root).ok()? else {
            return None;
        };
        node.av.net.extra_data.iter().find_map(|&e| {
            let index = self.reference(e)?;
            if self.block_type(index) != "BSBound" {
                return None;
            }
            let mut r = self.reader(index);
            r.i32("a name").ok()?;
            let center = r.vec3("a centre").ok()?;
            let half_extents = r.vec3("half extents").ok()?;
            Some(Bound {
                center,
                half_extents,
            })
        })
    }

    pub(crate) fn reference(&self, reference: i32) -> Option<usize> {
        usize::try_from(reference)
            .ok()
            .filter(|&i| i < self.blocks().len())
    }

    pub(crate) fn reader(&self, index: usize) -> Reader<'_> {
        Reader::new(self.block_bytes(index), self.blocks()[index].offset)
    }
}

impl Skin {
    /// Where a vertex goes with each bone at `bone_world` (the bone's
    /// transform in the space the result should be in): the weighted sum
    /// of bone × (skin → bone) applied to the vertex. With the bones in
    /// their bind pose this gives back the vertex as the skin's offset
    /// places it.
    pub fn deform(&self, vertex: usize, position: Vec3, bone_world: &[Transform]) -> Vec3 {
        self.blend(vertex, bone_world, |t| t.apply_point(position))
            .unwrap_or(position)
    }

    /// A direction (normal, tangent) turned with the bones, normalized.
    pub fn deform_direction(
        &self,
        vertex: usize,
        direction: Vec3,
        bone_world: &[Transform],
    ) -> Vec3 {
        let d = self
            .blend(vertex, bone_world, |t| t.apply_direction(direction))
            .unwrap_or(direction);
        crate::math::normalize(d)
    }

    fn blend(
        &self,
        vertex: usize,
        bone_world: &[Transform],
        apply: impl Fn(&Transform) -> Vec3,
    ) -> Option<Vec3> {
        let list = self.weights.get(vertex).filter(|l| !l.is_empty())?;
        let mut sum = [0.0f32; 3];
        let mut total = 0.0;
        for &(bone, w) in list {
            let (Some(world), Some(skin_to_bone)) = (
                bone_world.get(usize::from(bone)),
                self.bone_transforms.get(usize::from(bone)),
            ) else {
                continue;
            };
            let p = apply(&world.then_child(skin_to_bone));
            for k in 0..3 {
                sum[k] += w * p[k];
            }
            total += w;
        }
        (total > 0.0).then(|| sum.map(|c| c / total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f32s(v: &[f32]) -> Vec<u8> {
        v.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    fn transform_bytes(translation: [f32; 3]) -> Vec<u8> {
        let mut b = f32s(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        b.extend(f32s(&translation));
        b.extend(f32s(&[1.0]));
        b
    }

    #[test]
    fn reads_skin_data_with_its_weights() {
        let mut b = transform_bytes([0.0; 3]);
        b.extend(2u32.to_le_bytes());
        b.push(1);
        for (offset, verts) in [
            ([0.0, 0.0, -10.0], vec![(0u16, 1.0f32), (1, 0.25)]),
            ([5.0, 0.0, 0.0], vec![(1, 0.75)]),
        ] {
            b.extend(transform_bytes(offset));
            b.extend(f32s(&[0.0, 0.0, 0.0, 1.0]));
            b.extend((verts.len() as u16).to_le_bytes());
            for (v, w) in verts {
                b.extend(v.to_le_bytes());
                b.extend(w.to_le_bytes());
            }
        }
        let data = read_data(&mut Reader::new(&b, 0)).unwrap();
        assert_eq!(data.bones.len(), 2);
        assert_eq!(data.bones[0].0.translation, [0.0, 0.0, -10.0]);
        assert_eq!(data.bones[1].1, vec![(1, 0.75)]);
    }

    #[test]
    fn reads_partitions_into_the_shapes_numbering() {
        let mut b = 1u32.to_le_bytes().to_vec();
        // 3 vertices, 1 triangle, 1 bone, no strips, 1 weight per vertex.
        for v in [3u16, 1, 1, 0, 1] {
            b.extend(v.to_le_bytes());
        }
        b.extend(7u16.to_le_bytes()); // the partition's bone is skin bone 7
        b.push(1); // vertex map
        for v in [10u16, 11, 12] {
            b.extend(v.to_le_bytes());
        }
        b.push(1); // weights
        b.extend(f32s(&[1.0, 1.0, 0.5]));
        b.push(1); // faces
        for v in [0u16, 1, 2] {
            b.extend(v.to_le_bytes());
        }
        b.push(1); // bone indices
        b.extend([0u8, 0, 0]);
        let parts = read_partitions(&mut Reader::new(&b, 0)).unwrap();
        assert_eq!(parts[0].triangles, vec![[10, 11, 12]]);
        assert_eq!(parts[0].weights[2], (12, vec![(7, 0.5)]));
    }

    #[test]
    fn only_section_and_torso_caps_count_as_caps() {
        let part = |body_part| Partition {
            body_part,
            flags: 0,
            triangles: Vec::new(),
        };
        for cap in [101, 110, 201, 210] {
            assert!(part(cap).is_cap(), "{cap}");
        }
        // The body's own parts, as in upperbody.nif.
        for body in [0, 3, 10, 3000, 5000, 10000] {
            assert!(!part(body).is_cap(), "{body}");
        }
    }

    #[test]
    fn bones_in_their_bind_pose_leave_the_vertex_where_it_was() {
        // A bone 10 up: skin-to-bone moves down 10; the bone moves back up.
        let skin = Skin {
            bones: vec!["Bip01".into()],
            bone_transforms: vec![Transform {
                translation: [0.0, 0.0, -10.0],
                ..Transform::IDENTITY
            }],
            skin_transform: Transform::IDENTITY,
            weights: vec![vec![(0, 1.0)]],
            partitions: Vec::new(),
        };
        let bind = [Transform {
            translation: [0.0, 0.0, 10.0],
            ..Transform::IDENTITY
        }];
        assert_eq!(skin.deform(0, [1.0, 2.0, 3.0], &bind), [1.0, 2.0, 3.0]);
        // Raise the bone by 5: the vertex follows.
        let raised = [Transform {
            translation: [0.0, 0.0, 15.0],
            ..Transform::IDENTITY
        }];
        assert_eq!(skin.deform(0, [1.0, 2.0, 3.0], &raised), [1.0, 2.0, 8.0]);
    }
}
