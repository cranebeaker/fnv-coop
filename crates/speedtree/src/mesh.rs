//! The meshes the game makes from a grown tree (`BSTreeModel`): one leaf
//! mesh and one branch mesh per level of detail, laid out as the vertex
//! buffers the game uploads. Checked against the Goodsprings recording
//! (`trace_goodsprings`, calls 96398298–96398318): the shrub's leaf buffers
//! of both levels and its first branch level, value for value.
//!
//! Leaves (`00668500`, from `00b0bc00` and the leaf tables `00b29380`): four
//! vertices per leaf, all at the leaf's position with its normal; the
//! vertex shader (`STLEAF001.vso`) moves each corner out along the camera's
//! axes by the corner offsets in `LeafBase` ([`leaf_base`]). Per vertex
//! `BLENDINDICES` = (wind weight, wind matrix × 4, the `LeafBase` entry +
//! the leaf's brightness as a fraction, the level's card scale).
//!
//! Branches (`00667470`): each level keeps only the vertices its strips
//! use, in their original order, and its strips joined into one
//! (`00b2ed70` with 0: the last index of one strip and the first of the
//! next between them).

use crate::spt::SptFile;
use crate::tree::{Leaf, Tree};

/// One level of detail's leaf cards.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LeafMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// `BLENDINDICES`: wind weight, wind matrix × 4, `LeafBase` entry +
    /// brightness, card scale.
    pub blend: Vec<[f32; 4]>,
    /// Two triangles per leaf: (3, 1, 2), (0, 1, 3).
    pub indices: Vec<u16>,
}

/// One level of detail's branches.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BranchMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// `TANGENT` (the shader's first axis): around the branch.
    pub tangents: Vec<[f32; 3]>,
    /// `BINORMAL` (the second axis).
    pub binormals: Vec<[f32; 3]>,
    /// `BLENDINDICES`: wind weight, wind matrix × 4, brightness, 0.
    pub blend: Vec<[f32; 4]>,
    /// One triangle strip.
    pub strip: Vec<u16>,
}

impl BranchMesh {
    /// The strip as triangles, every other one turned round and the
    /// joins' empty ones dropped (as the game's `.nif` strips are read).
    pub fn triangles(&self) -> Vec<[u16; 3]> {
        strip_triangles(&self.strip)
    }
}

/// A triangle strip as a triangle list (the winding `nif` uses for the
/// game's own strips).
pub fn strip_triangles(strip: &[u16]) -> Vec<[u16; 3]> {
    let mut out = Vec::new();
    for i in 2..strip.len() {
        let (a, b, c) = (strip[i - 2], strip[i - 1], strip[i]);
        if a == b || b == c || a == c {
            continue;
        }
        out.push(if i % 2 == 0 { [a, b, c] } else { [a, c, b] });
    }
    out
}

/// How the game sets up the library (`BSTreeManager`'s constructor,
/// `00664440`: `00b0d7b0(1)`): leaf texture coordinates' v turned
/// round (`00b28720` multiplies them by −1).
pub const FLIP_V: bool = true;

/// The leaf texture coordinates of map `m` (texture `m / 2`, the odd maps
/// mirrored): the file's 10002 entry for the texture (`00b28720`): corners
/// (u, v) 0–3; the mirrored map swaps u between corners 0 and 1, 2 and 3;
/// v × −1 with [`FLIP_V`].
pub fn leaf_coords(spt: &SptFile, map: usize) -> [[f32; 2]; 4] {
    let tex = map / 2;
    let e = spt
        .billboards
        .as_ref()
        .and_then(|b| b.a.get(tex).copied())
        .unwrap_or([0.0; 8]);
    let s = if FLIP_V { -1.0 } else { 1.0 };
    if map % 2 == 0 {
        [
            [e[0], s * e[1]],
            [e[2], s * e[3]],
            [e[4], s * e[5]],
            [e[6], s * e[7]],
        ]
    } else {
        [
            [e[2], s * e[1]],
            [e[0], s * e[3]],
            [e[6], s * e[5]],
            [e[4], s * e[7]],
        ]
    }
}

/// Whether the leaf textures name more than one file (`00668500`'s first
/// loop compares every pair): then a corner takes its u from its own
/// coordinates and its v from the opposite corner's.
pub fn mixed_leaf_textures(spt: &SptFile) -> bool {
    let t = &spt.leaves.textures;
    (0..t.len()).any(|i| (i + 1..t.len()).any(|j| !t[i].file.eq_ignore_ascii_case(&t[j].file)))
}

/// The card corner offsets the leaf shader reads (`LeafBase`, `00b288f0`):
/// for each map m (two per leaf texture) and rocking group, four corners
/// (0, y, z, 0): with x0 = the texture's origin x (1 − it for even maps),
/// y0 its origin y and the card's width w and height h,
/// (0, (1 − x0) w, (1 − y0) h), (0, −x0 w, (1 − y0) h), (0, −x0 w, −y0 h),
/// (0, (1 − x0) w, −y0 h); entry `(m × groups + group) × 4 + corner`. The
/// game uploads level 0's (the shader scales by the vertex's card scale).
pub fn leaf_base(tree: &Tree, rocking_groups: i32) -> Vec<[f32; 4]> {
    let groups = rocking_groups.max(1) as usize;
    let mut out = Vec::new();
    for map in 0..tree.leaf_maps.len() * 2 {
        let m = &tree.leaf_maps[map / 2];
        let mut x0 = m.origin[0];
        if map % 2 == 0 {
            x0 = 1.0 - x0;
        }
        let y0 = m.origin[1];
        let right = (1.0 - x0) * m.width;
        let top = (1.0 - y0) * m.height;
        let left = -(x0 * m.width);
        let bottom = -(y0 * m.height);
        for _ in 0..groups {
            out.push([0.0, right, top, 0.0]);
            out.push([0.0, left, top, 0.0]);
            out.push([0.0, left, bottom, 0.0]);
            out.push([0.0, right, bottom, 0.0]);
        }
    }
    out
}

/// What the leaf and branch meshes need besides the grown tree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeshSettings {
    /// `SetNumLeafRockingGroups` (2 with fewer than four leaf textures,
    /// else 1: `0066ac40`).
    pub rocking_groups: i32,
    /// The first wind matrix and how many (`SetLocalMatrices(0, 4)`).
    pub wind_matrices: (i32, i32),
    /// How much bigger each leaf level's cards are (the file's 9009).
    pub leaf_lod_step: f32,
}

/// The leaf meshes, one per level of detail (`00668500`).
pub fn leaf_meshes(spt: &SptFile, tree: &Tree, settings: &MeshSettings) -> Vec<LeafMesh> {
    let mixed = mixed_leaf_textures(spt);
    let groups = settings.rocking_groups.max(1);
    tree.leaf_lods
        .iter()
        .enumerate()
        .map(|(lod, leaves)| {
            // `00b0cb10`: level l's cards × (1 + l × 9009).
            let scale = if lod == 0 {
                1.0
            } else {
                lod as f32 * settings.leaf_lod_step + 1.0
            };
            leaf_mesh(spt, leaves, mixed, groups, settings.wind_matrices, scale)
        })
        .collect()
}

/// The wind matrix a leaf or branch vertex follows (`00b29380`, `00b30560`):
/// the first + the group's low byte modulo the count.
pub fn wind_matrix(group: i32, matrices: (i32, i32)) -> u8 {
    let count = (matrices.1 & 0xff).max(1) as u32;
    (matrices.0 as u8).wrapping_add(((group as u8) as u32 % count) as u8)
}

fn leaf_mesh(
    spt: &SptFile,
    leaves: &[Leaf],
    mixed: bool,
    groups: i32,
    matrices: (i32, i32),
    scale: f32,
) -> LeafMesh {
    let mut m = LeafMesh::default();
    for (i, leaf) in leaves.iter().enumerate() {
        let first = (i * 4) as u16;
        // The brightness: the colour's green / 255, below 1.
        let mut bright = (((leaf.color & 0xff00) >> 8) as f64 / 255.0) as f32;
        if 1.0 <= bright {
            bright = 0.99;
        }
        let tc = leaf_coords(spt, leaf.map as usize);
        let cluster = (groups as u8)
            .wrapping_mul(leaf.map)
            .wrapping_add(leaf.rock_group);
        let weight = (1.0 - leaf.wind as f64) as f32;
        let matrix = wind_matrix(leaf.wind_group, matrices);
        for k in 0..4usize {
            m.positions.push(leaf.position);
            m.normals.push(leaf.normal);
            m.uvs.push(if mixed {
                [tc[k][0], tc[3 - k][1]]
            } else {
                tc[k]
            });
            let corner = (((k as u16 + 2) & 3) + cluster as u16 * 4) as f32;
            m.blend
                .push([weight, (matrix as u32 * 4) as f32, corner + bright, scale]);
        }
        m.indices.extend_from_slice(&[
            first + 3,
            first + 1,
            first + 2,
            first,
            first + 1,
            first + 3,
        ]);
    }
    m
}

/// The branch meshes, one per level of detail (`00667470`).
pub fn branch_meshes(tree: &Tree) -> Vec<BranchMesh> {
    let g = &tree.geometry;
    g.lods
        .iter()
        .map(|strips| {
            let joined = join_strips(strips);
            if joined.is_empty() {
                return BranchMesh::default();
            }
            // The vertices used, in their original order.
            let mut used = vec![false; g.positions.len()];
            for &i in &joined {
                if let Some(u) = used.get_mut(i as usize) {
                    *u = true;
                }
            }
            let mut remap = vec![u16::MAX; g.positions.len()];
            let mut m = BranchMesh::default();
            for (i, _) in used.iter().enumerate().filter(|(_, u)| **u) {
                remap[i] = m.positions.len() as u16;
                m.positions.push(g.positions[i]);
                m.normals.push(g.normals[i]);
                m.uvs.push(g.uvs[i]);
                m.tangents.push(g.binormals[i]);
                m.binormals.push(g.tangents[i]);
                let bright = (((g.colors[i] & 0xff00) >> 8) as f64 / 255.0) as f32;
                m.blend.push([
                    g.wind_weights[i],
                    (g.wind_groups[i] as u32 * 4) as f32,
                    bright,
                    0.0,
                ]);
            }
            m.strip = joined.iter().map(|&i| remap[i as usize]).collect();
            m
        })
        .collect()
}

/// `00b2ed70` with 0: the strips one after another, each join adding the
/// last index of the one before and the first of the next.
pub fn join_strips(strips: &[Vec<u16>]) -> Vec<u16> {
    let strips: Vec<&Vec<u16>> = strips.iter().collect();
    let mut out = Vec::new();
    for (k, s) in strips.iter().enumerate() {
        out.extend_from_slice(s);
        if k + 1 < strips.len() {
            if let (Some(&last), Some(&next)) = (s.last(), strips[k + 1].first()) {
                out.push(last);
                out.push(next);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_join_with_two_repeats() {
        let joined = join_strips(&[vec![0, 1, 2, 3], vec![4, 5, 6, 7]]);
        assert_eq!(joined, vec![0, 1, 2, 3, 3, 4, 4, 5, 6, 7]);
        // The joins give empty triangles, which are dropped; the winding
        // alternates.
        let tris = strip_triangles(&joined);
        assert_eq!(tris[0], [0, 1, 2]);
        assert_eq!(tris[1], [1, 3, 2]);
        assert_eq!(tris.len(), 4);
        assert_eq!(tris[2], [4, 5, 6]);
    }

    #[test]
    fn wind_matrix_takes_the_low_byte() {
        assert_eq!(wind_matrix(0x1_0003, (0, 4)), 3);
        assert_eq!(wind_matrix(6, (0, 4)), 2);
        assert_eq!(wind_matrix(6, (1, 4)), 3);
    }
}
