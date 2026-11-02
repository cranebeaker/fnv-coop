//! Terrain: an exterior cell's `LAND` record (heights, normals, vertex
//! colours and the textures painted on each quarter of the cell) and the
//! land textures (`LTEX`) it names.
//!
//! A cell is 4096 units square, sampled on a 33 × 33 grid of points 128
//! units apart (the edges are shared with the neighbouring cells). Rows run
//! from the cell's south edge northward, each from west to east. The cell is
//! textured in four quarters of 17 × 17 points, each with a base texture
//! and layers painted over it.

use esm::{FormId, FourCC, LoadOrder, Record, RecordRef};

use crate::cell::{le_f32, le_u32};

/// Size of an exterior cell in game units.
pub const CELL_SIZE: f32 = 4096.0;
/// Points per side of a cell's terrain grid.
pub const GRID: usize = 33;
/// Distance between neighbouring grid points.
pub const SPACING: f32 = CELL_SIZE / (GRID - 1) as f32;
/// Points per side of one quarter's texture grid.
pub const QUARTER_GRID: usize = 17;

const VHGT: FourCC = FourCC::new(b"VHGT");
const VNML: FourCC = FourCC::new(b"VNML");
const VCLR: FourCC = FourCC::new(b"VCLR");
const BTXT: FourCC = FourCC::new(b"BTXT");
const ATXT: FourCC = FourCC::new(b"ATXT");
const VTXT: FourCC = FourCC::new(b"VTXT");
const LAND: FourCC = FourCC::new(b"LAND");
const LTEX: FourCC = FourCC::new(b"LTEX");
const TXST: FourCC = FourCC::new(b"TXST");
const TNAM: FourCC = FourCC::new(b"TNAM");
const TX00: FourCC = FourCC::new(b"TX00");
const TX01: FourCC = FourCC::new(b"TX01");
const SNAM: FourCC = FourCC::new(b"SNAM");

/// The texture the game uses where a quarter names none: the INI settings
/// `sDefaultLandDiffuseTexture` and `sDefaultLandNormalTexture` under
/// `[Landscape]` (this install's `Fallout.ini`: `DirtWasteland01.dds` and
/// `DirtWasteland01_N.dds`), looked up in `textures\landscape\` (the game's
/// format string `Landscape\%s`).
pub const DEFAULT_DIFFUSE: &str = "landscape\\dirtwasteland01.dds";
pub const DEFAULT_NORMAL: &str = "landscape\\dirtwasteland01_n.dds";

/// One exterior cell's terrain.
#[derive(Debug, Clone, PartialEq)]
pub struct Land {
    pub form_id: FormId,
    /// `DATA` flags.
    pub flags: u32,
    /// 33 × 33 heights in game units, absolute (not relative to the cell).
    pub heights: Option<Vec<f32>>,
    /// 33 × 33 unit normals.
    pub normals: Option<Vec<[f32; 3]>>,
    /// 33 × 33 vertex colours.
    pub colors: Option<Vec<[u8; 3]>>,
    /// South-west, south-east, north-west, north-east.
    pub quarters: [Quarter; 4],
}

/// The textures painted on one quarter of a cell.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Quarter {
    /// The base texture (`BTXT`); `None` means the default land texture.
    pub base: Option<FormId>,
    /// Layers painted over it (`ATXT` + `VTXT`), in layer order.
    pub layers: Vec<Layer>,
}

/// A texture painted over a quarter with an opacity per point.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub texture: FormId,
    /// The layer number the record gives (layers are kept in this order).
    pub layer: u16,
    /// 17 × 17 opacities, 0 where the record lists none.
    pub opacity: Vec<f32>,
}

/// A land texture (`LTEX`) with its texture set's files.
#[derive(Debug, Clone, PartialEq)]
pub struct LandTexture {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// Diffuse and normal map, relative to `textures\` (the texture set's
    /// `TX00` and `TX01`).
    pub diffuse: Option<String>,
    pub normal: Option<String>,
    /// `SNAM`: the specular exponent.
    pub specular_exponent: Option<u8>,
}

impl Land {
    /// The terrain record of a cell, if it has one.
    pub fn of_cell(order: &LoadOrder, cell: FormId) -> crate::Result<Option<Land>> {
        let Some(rr) = order
            .in_cell(cell)
            .into_iter()
            .find(|r| r.entry.header.kind == LAND && !r.entry.header.is_deleted())
        else {
            return Ok(None);
        };
        let record = rr.record()?;
        Ok(Some(Land::parse(&rr, &record)))
    }

    /// Reads a `LAND` record. Texture form IDs are turned into load-order
    /// form IDs with the record's plugin.
    pub fn parse(rr: &RecordRef<'_>, record: &Record) -> Land {
        // `DATA` flags (the game keeps `& 7`): 0x1 heights and normals,
        // 0x2 colours, 0x4 textures; what a flag doesn't cover isn't read
        // (`00535d00`).
        let flags = record
            .get(esm::sig::DATA)
            .filter(|s| s.data.len() >= 4)
            .map_or(0, |s| le_u32(&s.data, 0));
        let heights = record
            .get(VHGT)
            .filter(|_| flags & 0x1 != 0)
            .filter(|s| s.data.len() >= 4 + GRID * GRID)
            .map(|s| heights(le_f32(&s.data, 0), &s.data[4..4 + GRID * GRID]));
        let normals = record
            .get(VNML)
            .filter(|_| flags & 0x1 != 0)
            .filter(|s| s.data.len() >= GRID * GRID * 3)
            .map(|s| {
                s.data[..GRID * GRID * 3]
                    .chunks_exact(3)
                    .map(|c| normal([c[0] as i8, c[1] as i8, c[2] as i8]))
                    .collect()
            });
        let colors = record
            .get(VCLR)
            .filter(|_| flags & 0x2 != 0)
            .filter(|s| s.data.len() >= GRID * GRID * 3)
            .map(|s| {
                s.data[..GRID * GRID * 3]
                    .chunks_exact(3)
                    .map(|c| [c[0], c[1], c[2]])
                    .collect()
            });
        let textured = flags & 0x4 != 0;

        // Textures (`00535d00`, `0053a8a0`): an `ATXT`'s layer number is its
        // slot (0–5; higher numbers are clamped to 5, the game logs it), a
        // later `ATXT` in the same slot replaces its texture and writes
        // over its opacities; `VTXT` opacities at or below 0 are 0, with no
        // cap at 1.
        let mut quarters: [Quarter; 4] = Default::default();
        let global = |data: &[u8]| rr.plugin.to_global(FormId(le_u32(data, 0)));
        let mut current: Option<(usize, u16)> = None;
        for sub in record.subrecords.iter().filter(|_| textured) {
            if sub.kind == BTXT && sub.data.len() >= 8 {
                current = None;
                let q = usize::from(sub.data[4] & 3);
                let id = global(&sub.data);
                quarters[q].base = (id.0 != 0).then_some(id);
            } else if sub.kind == ATXT && sub.data.len() >= 8 {
                let q = usize::from(sub.data[4] & 3);
                let slot =
                    u16::from_le_bytes([sub.data[6], sub.data[7]]).min(MAX_LAYERS as u16 - 1);
                let texture = global(&sub.data);
                let layers = &mut quarters[q].layers;
                match layers.iter_mut().find(|l| l.layer == slot) {
                    Some(l) => l.texture = texture,
                    None => layers.push(Layer {
                        texture,
                        layer: slot,
                        opacity: vec![0.0; QUARTER_GRID * QUARTER_GRID],
                    }),
                }
                current = Some((q, slot));
            } else if sub.kind == VTXT {
                let Some((q, slot)) = current else { continue };
                let Some(layer) = quarters[q].layers.iter_mut().find(|l| l.layer == slot) else {
                    continue;
                };
                for entry in sub.data.chunks_exact(8) {
                    let at = usize::from(u16::from_le_bytes([entry[0], entry[1]]));
                    if let Some(o) = layer.opacity.get_mut(at) {
                        *o = le_f32(entry, 4).max(0.0);
                    }
                }
            }
        }
        for q in &mut quarters {
            q.layers.sort_by_key(|l| l.layer);
        }

        Land {
            form_id: rr.form_id,
            flags,
            heights,
            normals,
            colors,
            quarters,
        }
    }

    /// The height at grid point (`x` east, `y` north), if the record has
    /// heights.
    pub fn height(&self, x: usize, y: usize) -> Option<f32> {
        self.heights.as_ref().map(|h| h[y * GRID + x])
    }

    /// One quarter of the terrain as a mesh, with the cell's south-west
    /// corner at `origin` (game units). `None` without heights.
    pub fn quarter_mesh(&self, quarter: usize, origin: [f32; 2]) -> Option<TerrainMesh> {
        self.quarter_mesh_tiled(quarter, origin, DEFAULT_TILING)
    }

    /// [`Self::quarter_mesh`] with the INI's `fLandTextureTilingMult`.
    pub fn quarter_mesh_tiled(
        &self,
        quarter: usize,
        origin: [f32; 2],
        tiling: f32,
    ) -> Option<TerrainMesh> {
        let uv_per_point = tiling / 4.0;
        let heights = self.heights.as_ref()?;
        let q = &self.quarters[quarter];
        let (qx, qy) = (
            (quarter & 1) * (QUARTER_GRID - 1),
            (quarter >> 1) * (QUARTER_GRID - 1),
        );
        let n = QUARTER_GRID;
        let mut mesh = TerrainMesh {
            positions: Vec::with_capacity(n * n),
            normals: Vec::with_capacity(n * n),
            uvs: Vec::with_capacity(n * n),
            colors: Vec::with_capacity(n * n),
            weights: Vec::with_capacity(n * n),
            tangents: Vec::new(),
            binormals: Vec::new(),
            indices: Vec::with_capacity((n - 1) * (n - 1) * 6),
            textures: std::iter::once(q.base)
                .chain(q.layers.iter().map(|l| Some(l.texture)))
                .collect(),
        };
        for ly in 0..n {
            for lx in 0..n {
                let (gx, gy) = (qx + lx, qy + ly);
                let at = gy * GRID + gx;
                mesh.positions.push([
                    origin[0] + gx as f32 * SPACING,
                    origin[1] + gy as f32 * SPACING,
                    heights[at],
                ]);
                mesh.normals.push(
                    self.normals
                        .as_ref()
                        .map_or([0.0, 0.0, 1.0], |normals| normals[at]),
                );
                mesh.uvs
                    .push([lx as f32 * uv_per_point, ly as f32 * uv_per_point]);
                mesh.colors.push(
                    self.colors
                        .as_ref()
                        .map_or([1.0; 3], |c| c[at].map(|v| f32::from(v) / 255.0)),
                );
                let opacities: Vec<f32> = q.layers.iter().map(|l| l.opacity[ly * n + lx]).collect();
                mesh.weights.push(blend_weights(&opacities));
            }
        }
        for ly in 0..n - 1 {
            for lx in 0..n - 1 {
                let i = |x: usize, y: usize| (y * n + x) as u16;
                let (sw, se, nw, ne) = (i(lx, ly), i(lx + 1, ly), i(lx, ly + 1), i(lx + 1, ly + 1));
                // Counter-clockwise seen from above, the diagonal in a
                // checkerboard (the game's strip at `0118ae90`): south-west
                // to north-east where column + row is even, south-east to
                // north-west where it's odd.
                if (lx + ly) % 2 == 0 {
                    mesh.indices.extend_from_slice(&[ne, nw, sw, sw, se, ne]);
                } else {
                    mesh.indices.extend_from_slice(&[nw, sw, se, se, ne, nw]);
                }
            }
        }
        (mesh.tangents, mesh.binormals) =
            tangent_frames(&mesh.positions, &mesh.normals, &mesh.uvs, &mesh.indices);
        Some(mesh)
    }
}

/// The normal maps' frame at each vertex, as the game builds it for a
/// terrain quarter (`CreateTangentSpaceSimple`, `00b54b60`, read in
/// `findings\terrain.md`): every triangle adds its position change per
/// unit of the texture's U (`dP/du`) to its three corners; then
/// `T = normalize(S − (N·S) N)` with `S` that sum and `N` the vertex normal,
/// and `B = normalize(N × T)`. Confirmed value for value by the Goodsprings
/// recording's terrain vertex buffers (the corner of a slope with normal
/// (0.1026, −0.3708, 0.923) got T (0.9924, −0.0258, −0.1206) and B (0.0685,
/// 0.9284, 0.3653); laying +x into the surface would give (0.9947, 0.0382,
/// −0.0952) there). Where `S` lies along the normal (or there's none), T is
/// +x laid into the surface.
fn tangent_frames(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    uvs: &[[f32; 2]],
    indices: &[u16],
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let sub = |a: [f32; 3], b: [f32; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let cross = |a: [f32; 3], b: [f32; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let normalize = |v: [f32; 3]| {
        let len = dot(v, v).sqrt();
        (len > 1e-12).then(|| v.map(|c| c / len))
    };
    let mut sums = vec![[0.0f32; 3]; positions.len()];
    for t in indices.chunks_exact(3) {
        let [a, b, c] = [t[0], t[1], t[2]].map(usize::from);
        let (e1, e2) = (
            sub(positions[b], positions[a]),
            sub(positions[c], positions[a]),
        );
        let (du1, dv1) = (uvs[b][0] - uvs[a][0], uvs[b][1] - uvs[a][1]);
        let (du2, dv2) = (uvs[c][0] - uvs[a][0], uvs[c][1] - uvs[a][1]);
        let det = du1 * dv2 - du2 * dv1;
        if det.abs() < 1e-12 {
            continue;
        }
        let dpdu: [f32; 3] = std::array::from_fn(|k| (e1[k] * dv2 - e2[k] * dv1) / det);
        for v in [a, b, c] {
            for k in 0..3 {
                sums[v][k] += dpdu[k];
            }
        }
    }
    let mut tangents = Vec::with_capacity(positions.len());
    let mut binormals = Vec::with_capacity(positions.len());
    for (s, &n) in sums.into_iter().zip(normals) {
        let along = |v: [f32; 3]| {
            let d = dot(n, v);
            normalize([v[0] - d * n[0], v[1] - d * n[1], v[2] - d * n[2]])
        };
        let t = along(s)
            .or_else(|| along([1.0, 0.0, 0.0]))
            .unwrap_or([1.0, 0.0, 0.0]);
        let b = normalize(cross(n, t)).unwrap_or([0.0, 1.0, 0.0]);
        tangents.push(t);
        binormals.push(b);
    }
    (tangents, binormals)
}

/// `fLandTextureTilingMult`'s built-in default (`00f4afb0`; also this
/// install's INI value). Texture coordinates per grid step are it / 4
/// (`00533420`): 0.5, so every texture repeats every two points (256
/// units), eight times across a quarter. Every quarter starts at (0, 0) in
/// its south-west corner, v northward; all textures and normal maps share
/// them.
pub const DEFAULT_TILING: f32 = 2.0;

/// A quarter of a cell's terrain, ready to draw: one entry per grid point
/// (17 × 17, rows from the south), triangles counter-clockwise from above.
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainMesh {
    /// Game units, world space.
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// The vertex colours as stored, 0..1.
    pub colors: Vec<[f32; 3]>,
    /// How much of each texture shows, in the order of `textures`.
    pub weights: Vec<Vec<f32>>,
    /// The normal maps' frame (see `tangent_frames`): T along the texture's
    /// U (east), B along V (north), both unit length.
    pub tangents: Vec<[f32; 3]>,
    pub binormals: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
    /// The base texture (`None`: the default land texture), then each
    /// layer's.
    pub textures: Vec<Option<FormId>>,
}

/// Layer slots per quarter: with the base, the seven textures the terrain
/// shaders take.
pub const MAX_LAYERS: usize = 6;

/// How much of the base texture and of each layer shows at a point, from
/// the layers' opacities (in slot order): the first weight is the base's.
///
/// Read from the game (`0053aeb0`): the layers keep their opacities as
/// stored; the base gets what's left, `clamp(1 − Σ, 0, 1)`; where the
/// layers add up past 1 (Goodsprings has 1.34) each is divided by their
/// total and the base gets nothing. So the weights always add up to 1 and
/// no layer covers another. (Layers whose texture can't be found are
/// dropped by the game after counting their opacity in that total; not done
/// here.)
pub fn blend_weights(opacities: &[f32]) -> Vec<f32> {
    let total: f32 = opacities.iter().map(|o| o.max(0.0)).sum();
    let share = if total > 1.0 { total } else { 1.0 };
    std::iter::once((1.0 - total).clamp(0.0, 1.0))
        .chain(opacities.iter().map(|o| o.max(0.0) / share))
        .collect()
}

/// Heights from `VHGT`: a starting value, then one signed step per point.
/// The first step of each row is from the first point of the row below;
/// the rest are from the point to the west. Each running total is rounded
/// to a whole number (the game's `FISTP`, to nearest) before × 8 game units.
fn heights(offset: f32, steps: &[u8]) -> Vec<f32> {
    let mut out = vec![0.0; GRID * GRID];
    let mut row_start = offset;
    for y in 0..GRID {
        row_start += f32::from(steps[y * GRID] as i8);
        let mut h = row_start;
        out[y * GRID] = round_to_even(h) * 8.0;
        for x in 1..GRID {
            h += f32::from(steps[y * GRID + x] as i8);
            out[y * GRID + x] = round_to_even(h) * 8.0;
        }
    }
    out
}

/// Rounded to the nearest whole number, halves to even (as `FISTP` does by
/// default).
fn round_to_even(h: f32) -> f32 {
    let r = h.round();
    if (h - h.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r - h.signum()
    } else {
        r
    }
}

/// A `VNML` normal: signed bytes ÷ 127, normalized; a zero one stays zero
/// (`004a0c10`).
fn normal(n: [i8; 3]) -> [f32; 3] {
    let v = n.map(|c| f32::from(c) / 127.0);
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-6 {
        v.map(|c| c / len)
    } else {
        [0.0; 3]
    }
}

impl LandTexture {
    /// Reads a land texture and the texture set it names.
    pub fn load(order: &LoadOrder, id: FormId) -> Option<LandTexture> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == LTEX)?;
        let record = rr.record().ok()?;
        let set = record
            .get(TNAM)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            .and_then(|set| order.get(set))
            .filter(|r| r.entry.header.kind == TXST)
            .and_then(|r| r.record().ok());
        let path = |kind: FourCC| {
            set.as_ref()
                .and_then(|s| s.get(kind))
                .map(|s| s.zstring())
                .filter(|p| !p.is_empty())
        };
        Some(LandTexture {
            form_id: id,
            editor_id: record.editor_id(),
            diffuse: path(TX00),
            normal: path(TX01),
            specular_exponent: record.get(SNAM).and_then(|s| s.data.first().copied()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_share_with_the_base_as_the_game_works_it_out() {
        // Nothing painted: all base.
        assert_eq!(blend_weights(&[0.0, 0.0]), vec![1.0, 0.0, 0.0]);
        // Up to 1 the layers keep their opacities and the base fills the
        // rest; no layer covers another.
        assert_eq!(blend_weights(&[0.25, 0.5]), vec![0.25, 0.25, 0.5]);
        // Past 1 they're scaled to share 1 and the base gets nothing.
        assert_eq!(blend_weights(&[1.0, 1.0]), vec![0.0, 0.5, 0.5]);
        let w = blend_weights(&[0.8, 0.3, 0.6]);
        assert_eq!(w[0], 0.0);
        assert!((w[1] - 0.8 / 1.7).abs() < 1e-6, "{w:?}");
        assert!((w.iter().sum::<f32>() - 1.0).abs() < 1e-6, "{w:?}");
        // Negative opacities count as none.
        assert_eq!(blend_weights(&[-0.5]), vec![1.0, 0.0]);
    }

    #[test]
    fn squares_split_in_a_checkerboard() {
        let mut land = Land {
            form_id: FormId(1),
            flags: 7,
            heights: Some(vec![0.0; GRID * GRID]),
            normals: None,
            colors: None,
            quarters: Default::default(),
        };
        land.quarters[0].base = None;
        let mesh = land.quarter_mesh(0, [0.0, 0.0]).unwrap();
        // The first square (0, 0) runs south-west to north-east: both its
        // triangles hold points 0 (SW) and 18 (NE); the next (1, 0) runs
        // south-east to north-west: points 2 (SE) and 18 (NW).
        let first = &mesh.indices[..6];
        assert!(first[..3].contains(&0) && first[..3].contains(&18));
        assert!(first[3..].contains(&0) && first[3..].contains(&18));
        let next = &mesh.indices[6..12];
        assert!(next[..3].contains(&2) && next[..3].contains(&18));
        assert!(next[3..].contains(&2) && next[3..].contains(&18));
        // Texture coordinates: half a repeat per point.
        assert_eq!(mesh.uvs[1], [0.5, 0.0]);
        assert_eq!(mesh.uvs[16 * 17 + 16], [8.0, 8.0]);
    }

    #[test]
    fn the_normal_map_frame_is_the_games() {
        // The south-west corner of a quarter in the Goodsprings recording
        // (cell -18,0, its south-west quarter, relative to the cell's
        // middle): the corner square's four points, uvs, the corner's
        // normal, and the T and B the game's vertex buffer held there.
        let positions = [
            [-2048.0, -2048.0, -100.0],
            [-1920.0, -2048.0, -124.0],
            [-2048.0, -1920.0, -36.0],
            [-1920.0, -1920.0, -60.0],
        ];
        let normals = [
            [0.1026, -0.3708, 0.923],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ];
        let uvs = [[0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]];
        // South-west to north-east, counter-clockwise: (NE, NW, SW), (SW,
        // SE, NE).
        let indices = [3, 2, 0, 0, 1, 3];
        let (t, b) = tangent_frames(&positions, &normals, &uvs, &indices);
        let close = |got: [f32; 3], want: [f32; 3]| {
            assert!(
                got.iter().zip(want).all(|(g, w)| (g - w).abs() < 2e-4),
                "{got:?} vs {want:?}"
            );
        };
        close(t[0], [0.9924, -0.0258, -0.1206]);
        close(b[0], [0.0685, 0.9284, 0.3653]);
        // On flat ground: east and north.
        close(t[3], [1.0, 0.0, 0.0]);
        close(b[3], [0.0, 1.0, 0.0]);
    }

    #[test]
    fn heights_build_rows_from_the_first_point_of_the_row_below() {
        let mut steps = vec![0u8; GRID * GRID];
        // Row 0: starts 1 above the offset, then rises 1 per point.
        steps[..GRID].fill(1);
        // Row 1 starts 2 below row 0's first point and is flat.
        steps[GRID] = (-2i8) as u8;
        let h = heights(10.0, &steps);
        assert_eq!(h[0], 11.0 * 8.0);
        assert_eq!(h[GRID - 1], (11.0 + 32.0) * 8.0);
        assert_eq!(h[GRID], 9.0 * 8.0);
        assert_eq!(h[2 * GRID - 1], 9.0 * 8.0);
    }
}
