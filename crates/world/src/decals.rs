//! World decals: the marks a shot leaves on what it strikes, and blood
//! spattered on a wall behind someone hit. Read from `FalloutNV.exe`
//! 1.4.0.525 (`BSTempEffectSimpleDecal`, `BGSDecalManager`).
//!
//! **Where** (`009c20e0` for shots, `0088e8d0` for spatter): the impact's
//! decal data (`IPCT` `DODT`, the record's `+0x54`) gives a size U(min
//! width, max width) used for both sides (`004a40c0`, `004a40a0`,
//! `00476b70`; chosen once per projectile), its depth (`008aff10`, `DODT`
//! +0x10; spatter uses 48), its colour (`004a4220`: bytes 0–2 ÷ 255, the
//! fourth unused), the impact's angle threshold (`009a9350`, `IPCT`
//! +0x38), a turn U(0, 1) taken as radians (`004a4240`) and one of the
//! texture's four pictures (`trunc(U(0, 4))`, `00476b70`, `00ec62c0`). The
//! decal goes on each `NiTriStrips` piece of what's there (`004a1a70`, see
//! `preview::cell::ModelMesh::strips`): for shots, the objects the
//! `DecalCaster` query finds at the point whose struck shape has the
//! impact's material (`009c20e0`), skipping pieces whose shader is itself a
//! decal (`004a2020(0x1a)`) and bases that take none (`004a1060`: placeable
//! water, people, creatures, projectiles).
//!
//! **Shape** ([`DecalBox`], `0068b4c0`): the box around the point along
//! the surface's normal D, its sides U and V the effect frame's X and Y
//! (`00c4b600`, as [`crate::impacts::effect_rotation`]) turned about D by
//! the turn (`004a0c90`, `0043f8d0`: U = cos·X + sin·Y, V = −sin·X +
//! cos·Y); half the size across U and V, the depth either way along D (six
//! planes, `0068d660`). Each triangle of the piece whose face normal is
//! within the threshold of D (`0068d230`: n·D > |n| × max(0.01,
//! cos(threshold)), the global `0119c29c` set in `0068ad20`) is clipped to
//! the box (`0068d820`, Sutherland–Hodgman, the cut's `t` kept to 0..1)
//! and the polygon added as a fan (`0068cb60`), at most 512 vertices a
//! decal (the piece's first vertices are shared, new ones within 0.01 of
//! an earlier new one too). Texture coordinates (`0068b4c0`): `u =
//! (U·(p − o) / width + 0.5) / 2`, `v` likewise along V, moved by half for
//! pictures 1 and 3 (u) and 2 and 3 (v). Vertex colour: the decal's
//! colour, alpha `max(0, (n̂·D − c) / (1 − c))` with the vertex's normal
//! (`0068cb60`), so it fades toward the threshold.
//!
//! **How long** ([`fade`], `0068c8e0`): `fDecalLifetime` (`[Display]`, exe
//! 10 s) at full strength, then its alpha falls from 1 to 0 over a second
//! and it goes. At most `uMaxDecals` (`[Decals]`, exe 100) at once, the
//! oldest going first (`0068be90`), and `iMaxDecalsPerFrame` (`[Display]`,
//! exe 10) made each frame (`004a1a70`); one decal is one piece's.

use crate::impacts::{effect_rotation, Decal};

/// `fDecalLifetime` (`[Display]`), the exe's default (`00f3d570`).
pub const LIFETIME: f32 = 10.0;
/// `uMaxDecals` (`[Decals]`), the exe's default (`00f3d600`).
pub const MAX_DECALS: usize = 100;
/// `iMaxDecalsPerFrame` (`[Display]`), the exe's default (`00f3d5a0`).
pub const MAX_PER_FRAME: usize = 10;
/// A decal's vertex budget (`0068cb60`: 0x200).
pub const MAX_VERTICES: usize = 512;
/// The depth a wall spatter's box reaches either way (`0088e8d0`).
pub const SPATTER_DEPTH: f32 = 48.0;
/// The decal query's size: 32 for decals up to 32 wide, 256 above
/// (`00622a70`, `00622ba0`, `00622bb0`).
pub fn query_size(size: f32) -> f32 {
    if size <= 32.0 {
        32.0
    } else {
        256.0
    }
}

type V3 = [f32; 3];

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn length(a: V3) -> f32 {
    dot(a, a).sqrt()
}

fn lerp(a: V3, b: V3, t: f32) -> V3 {
    [0, 1, 2].map(|k| a[k] + (b[k] - a[k]) * t)
}

/// The box a decal fills, and how it maps what it covers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecalBox {
    pub origin: V3,
    /// D: the struck surface's normal (unit).
    pub normal: V3,
    /// The sides across (unit).
    pub u: V3,
    pub v: V3,
    pub width: f32,
    pub height: f32,
    /// How far it reaches either way along D.
    pub depth: f32,
    /// `max(0.01, cos(angle threshold))`.
    pub min_cos: f32,
    /// The vertex colour (0..1).
    pub color: [f32; 3],
    /// Which of the texture's four pictures (0–3; anything else the whole
    /// texture).
    pub picture: u8,
}

/// What a decal is made from: its data, the turn (0..1, radians), the
/// picture's roll (0..1) and the size's (0..1), and the depth (the decal
/// data's for shots, [`SPATTER_DEPTH`] for spatter).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecalRolls {
    pub size: f32,
    pub turn: f32,
    pub picture: f32,
}

impl DecalBox {
    /// A decal at `origin` on a surface facing `normal` (`0068b4c0`).
    pub fn new(
        origin: V3,
        normal: V3,
        decal: &Decal,
        angle_threshold: f32,
        depth: f32,
        rolls: DecalRolls,
    ) -> DecalBox {
        let size = decal.size(rolls.size, false);
        DecalBox::sized(origin, normal, decal, angle_threshold, (size, depth), rolls)
    }

    /// [`Self::new`] with the size given (×1.5 for spatter under Bloody
    /// Mess, `0088e8d0`).
    pub fn sized(
        origin: V3,
        normal: V3,
        decal: &Decal,
        angle_threshold: f32,
        (size, depth): (f32, f32),
        rolls: DecalRolls,
    ) -> DecalBox {
        let frame = effect_rotation(normal, 0.0);
        let col = |j: usize| [frame[0][j], frame[1][j], frame[2][j]];
        let (x, y, d) = (col(0), col(1), col(2));
        let (s, c) = rolls.turn.sin_cos();
        let u = [0, 1, 2].map(|k| c * x[k] + s * y[k]);
        let v = [0, 1, 2].map(|k| -s * x[k] + c * y[k]);
        let picture = (rolls.picture * 4.0).trunc().clamp(0.0, 4.0) as u8;
        DecalBox {
            origin,
            normal: d,
            u,
            v,
            width: size,
            height: size,
            depth,
            min_cos: angle_threshold.to_radians().cos().max(0.01),
            color: [0, 1, 2].map(|k| f32::from(decal.color[k]) / 255.0),
            picture,
        }
    }

    /// The six planes as (normal, constant): a point is kept when
    /// `normal·p − constant ≥ 0`, in the game's order (`0068d660`: `+0x64`,
    /// `+0x74`, `+0x84`, `+0x94`, `+0xb4`, `+0xa4`).
    fn planes(&self) -> [(V3, f32); 6] {
        let neg = |a: V3| a.map(|c| -c);
        let (u, v, d, o) = (self.u, self.v, self.normal, self.origin);
        [
            (u, dot(u, o) - self.width * 0.5),
            (neg(u), -(self.width * 0.5 + dot(u, o))),
            (v, dot(v, o) - self.height * 0.5),
            (neg(v), -(self.height * 0.5 + dot(v, o))),
            (d, dot(d, o) - self.depth),
            (neg(d), -(self.depth + dot(d, o))),
        ]
    }

    /// Whether a triangle faces the decal's way closely enough to take it
    /// (`0068d230`).
    pub fn accepts(&self, [a, b, c]: [V3; 3]) -> bool {
        let n = cross(sub(b, a), sub(c, a));
        dot(n, self.normal) > length(n) * self.min_cos
    }

    /// A triangle clipped to the box (`0068d660`, `0068d820`): its corners
    /// (position, normal, the piece's vertex it is if it's one of them),
    /// empty when nothing is left.
    pub fn clip(&self, corners: [(V3, V3, Option<u32>); 3]) -> Vec<(V3, V3, Option<u32>)> {
        let mut poly: Vec<(V3, V3, Option<u32>)> = corners.to_vec();
        for (n, k) in self.planes() {
            if poly.is_empty() {
                break;
            }
            let side = |p: V3| dot(n, p) - k;
            // A point on the negative side is out (`0049da80` == 2).
            let out: Vec<bool> = poly.iter().map(|q| side(q.0) < 0.0).collect();
            if out.iter().all(|&o| o) {
                return Vec::new();
            }
            let mut next = Vec::with_capacity(poly.len() + 1);
            for i in 0..poly.len() {
                let prev = if i == 0 { poly.len() - 1 } else { i - 1 };
                let cut = |from: usize, to: usize| {
                    let (a, b) = (side(poly[from].0), side(poly[to].0));
                    let t = if a != b {
                        (a / (a - b)).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    (
                        lerp(poly[from].0, poly[to].0, t),
                        lerp(poly[from].1, poly[to].1, t),
                        None,
                    )
                };
                if !out[i] {
                    if out[prev] {
                        next.push(cut(prev, i));
                    }
                    next.push(poly[i]);
                } else if !out[prev] {
                    next.push(cut(prev, i));
                }
            }
            poly = next;
        }
        poly
    }

    /// A point's texture coordinates (`0068b4c0`).
    pub fn uv(&self, p: V3) -> [f32; 2] {
        let o = sub(p, self.origin);
        let mut u = (dot(o, self.u) / self.width + 0.5) / 2.0;
        let mut v = (dot(o, self.v) / self.height + 0.5) / 2.0;
        match self.picture {
            0 => {}
            1 => u += 0.5,
            2 => v += 0.5,
            3 => {
                u += 0.5;
                v += 0.5;
            }
            _ => {
                u *= 2.0;
                v *= 2.0;
            }
        }
        [u, v]
    }

    /// A vertex's alpha by its normal (`0068cb60`).
    pub fn alpha(&self, normal: V3) -> f32 {
        let l = length(normal);
        if l <= 0.0 {
            return 0.0;
        }
        ((dot(normal, self.normal) / l - self.min_cos) / (1.0 - self.min_cos)).max(0.0)
    }
}

/// A decal's geometry, in the space the triangles were given in.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecalMesh {
    pub positions: Vec<V3>,
    pub normals: Vec<V3>,
    pub uvs: Vec<[f32; 2]>,
    /// Linear RGB is left to the drawer: the decal colour (0..1) and the
    /// angle alpha.
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u16>,
    /// The piece's own vertices already in it, by their index.
    shared: std::collections::HashMap<u32, u16>,
    /// Where the vertices made by clipping start in each list (they're
    /// shared within 0.01).
    made: Vec<u16>,
    /// The vertex budget ran out: nothing more is added (`0068d230`
    /// returns).
    pub full: bool,
}

impl DecalMesh {
    /// Adds one of the piece's triangles (corners with their normals and
    /// vertex indices) if it faces the decal's way, clipped to the box.
    pub fn add_triangle(&mut self, b: &DecalBox, corners: [(V3, V3, u32); 3]) {
        if self.full || !b.accepts(corners.map(|c| c.0)) {
            return;
        }
        let poly = b.clip(corners.map(|(p, n, i)| (p, n, Some(i))));
        if poly.len() < 3 {
            return;
        }
        if self.positions.len() + poly.len() >= MAX_VERTICES {
            self.full = true;
            return;
        }
        let mut at = Vec::with_capacity(poly.len());
        for (p, n, source) in poly {
            let found = match source {
                Some(i) => self.shared.get(&i).copied(),
                None => self
                    .made
                    .iter()
                    .copied()
                    .find(|&j| length(sub(self.positions[usize::from(j)], p)) <= 0.01),
            };
            let index = match found {
                Some(j) => j,
                None => {
                    let j = self.positions.len() as u16;
                    self.positions.push(p);
                    self.normals.push(n);
                    self.uvs.push(b.uv(p));
                    let [r, g, bl] = b.color;
                    self.colors.push([r, g, bl, b.alpha(n)]);
                    match source {
                        Some(i) => {
                            self.shared.insert(i, j);
                        }
                        None => self.made.push(j),
                    }
                    j
                }
            };
            at.push(index);
        }
        for k in 2..at.len() {
            self.indices.extend([at[0], at[k - 1], at[k]]);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

/// A decal's strength `age` seconds after it was made, if it's still there
/// (`0068c8e0`): 1 for `lifetime`, then down to 0 over a second.
pub fn fade(age: f32, lifetime: f32) -> Option<f32> {
    if age <= lifetime {
        return Some(1.0);
    }
    let over = age - lifetime;
    (over < 1.0).then_some(1.0 - over)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decal() -> Decal {
        Decal {
            min_width: 10.0,
            max_width: 30.0,
            min_height: 8.0,
            max_height: 32.0,
            depth: 16.0,
            shininess: 4.0,
            parallax_scale: 0.04,
            parallax_passes: 1,
            flags: Decal::ALPHA_BLENDING,
            color: [255, 128, 0, 255],
        }
    }

    fn rolls(size: f32, turn: f32, picture: f32) -> DecalRolls {
        DecalRolls {
            size,
            turn,
            picture,
        }
    }

    /// A wall facing −y (toward a shooter in front), the decal on it.
    fn on_wall(turn: f32, picture: f32) -> DecalBox {
        DecalBox::new(
            [0.0, 0.0, 100.0],
            [0.0, -1.0, 0.0],
            &decal(),
            15.0,
            16.0,
            rolls(0.5, turn, picture),
        )
    }

    #[test]
    fn a_decal_box_lies_on_the_surface_with_its_sides_turned() {
        let b = on_wall(0.0, 0.0);
        assert_eq!(b.width, 20.0);
        assert_eq!(b.height, 20.0);
        assert!((dot(b.u, b.normal)).abs() < 1e-6 && (dot(b.v, b.normal)).abs() < 1e-6);
        assert!((dot(b.u, b.v)).abs() < 1e-6);
        assert!((b.min_cos - 15f32.to_radians().cos()).abs() < 1e-6);
        assert_eq!(b.color, [1.0, 128.0 / 255.0, 0.0]);
        // A turn of 0.5 radians turns U about D by that much.
        let t = on_wall(0.5, 0.0);
        assert!((dot(t.u, b.u) - 0.5f32.cos()).abs() < 1e-5);
        // 90° and more: the threshold's floor.
        let wide = DecalBox::new(
            [0.0; 3],
            [0.0, 0.0, 1.0],
            &decal(),
            180.0,
            4.0,
            rolls(0.0, 0.0, 0.0),
        );
        assert_eq!(wide.min_cos, 0.01);
        assert_eq!(wide.width, 10.0);
    }

    #[test]
    fn only_triangles_facing_the_decals_way_take_it() {
        let b = on_wall(0.0, 0.0);
        // Facing −y (wound so), and the same triangle wound the other way.
        let facing = [[-50.0, 0.0, 50.0], [50.0, 0.0, 50.0], [0.0, 0.0, 150.0]];
        assert!(b.accepts(facing));
        assert!(!b.accepts([facing[0], facing[2], facing[1]]));
        // Tilted 30° away: past the 15° threshold.
        let a = 30f32.to_radians();
        let tilted = [
            [-50.0, 0.0, 50.0],
            [50.0, 0.0, 50.0],
            [0.0, 100.0 * a.sin(), 50.0 + 100.0 * a.cos()],
        ];
        assert!(!b.accepts(tilted));
    }

    #[test]
    fn a_big_triangle_is_cut_to_the_box() {
        let b = on_wall(0.0, 0.0);
        let n = [0.0, -1.0, 0.0];
        let big = [
            ([-500.0, 0.0, -400.0], n, 0),
            ([500.0, 0.0, -400.0], n, 1),
            ([0.0, 0.0, 600.0], n, 2),
        ];
        let mut mesh = DecalMesh::default();
        mesh.add_triangle(&b, big);
        // The 20 × 20 square inside the triangle: four corners, two
        // triangles, every corner on the box's edge.
        assert_eq!(mesh.positions.len(), 4, "{:?}", mesh.positions);
        assert_eq!(mesh.indices.len(), 6);
        for p in &mesh.positions {
            let o = sub(*p, b.origin);
            assert!((dot(o, b.u).abs() - 10.0).abs() < 1e-3);
            assert!((dot(o, b.v).abs() - 10.0).abs() < 1e-3);
        }
        // Picture 0: the first quarter of the texture, its corners.
        for uv in &mesh.uvs {
            assert!(uv
                .iter()
                .all(|&c| (c - 0.0).abs() < 1e-4 || (c - 0.5).abs() < 1e-4));
        }
        // Facing straight along D: full alpha, the decal's colour.
        assert!(mesh
            .colors
            .iter()
            .all(|c| (c[3] - 1.0).abs() < 1e-5 && c[0] == 1.0));
        // Too far behind the wall (beyond the depth): nothing.
        let behind = big.map(|(p, n, i)| ([p[0], 40.0, p[2]], n, i + 3));
        let mut none = DecalMesh::default();
        none.add_triangle(&b, behind);
        assert!(none.is_empty());
    }

    #[test]
    fn pictures_pick_a_quarter_and_alpha_fades_toward_the_threshold() {
        let centre = [0.0, 0.0, 100.0];
        assert_eq!(on_wall(0.0, 0.0).uv(centre), [0.25, 0.25]);
        assert_eq!(on_wall(0.0, 0.3).uv(centre), [0.75, 0.25]);
        assert_eq!(on_wall(0.0, 0.6).uv(centre), [0.25, 0.75]);
        assert_eq!(on_wall(0.0, 0.9).uv(centre), [0.75, 0.75]);
        assert_eq!(on_wall(0.0, 1.0).uv(centre), [0.5, 0.5]);
        let b = on_wall(0.0, 0.0);
        let half = (15f32.to_radians().cos() + 1.0) / 2.0;
        let between = [0.0, -half, (1.0 - half * half).sqrt()];
        assert!((b.alpha(between) - 0.5).abs() < 1e-3);
        assert_eq!(b.alpha([0.0, 0.0, 1.0]), 0.0);
    }

    #[test]
    fn shared_corners_are_kept_once_and_the_budget_stops_a_decal() {
        let b = DecalBox::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            &decal(),
            15.0,
            16.0,
            rolls(1.0, 0.0, 0.0),
        );
        let n = [0.0, 0.0, 1.0];
        // A 2 × 2 unit square of two triangles well inside the 30-wide box.
        let p = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.0, 2.0, 0.0],
        ];
        let mut mesh = DecalMesh::default();
        mesh.add_triangle(&b, [(p[0], n, 0), (p[1], n, 1), (p[2], n, 2)]);
        mesh.add_triangle(&b, [(p[0], n, 0), (p[2], n, 2), (p[3], n, 3)]);
        assert_eq!(mesh.positions.len(), 4);
        assert_eq!(mesh.indices, vec![0, 1, 2, 0, 2, 3]);
        // Filling past 512 vertices stops it.
        let mut full = DecalMesh::default();
        for i in 0..200u32 {
            let base = i * 3;
            full.add_triangle(
                &b,
                [(p[0], n, base), (p[1], n, base + 1), (p[2], n, base + 2)],
            );
        }
        assert!(full.full);
        assert!(full.positions.len() < MAX_VERTICES);
    }

    #[test]
    fn decals_last_their_lifetime_then_fade_for_a_second() {
        assert_eq!(fade(0.0, LIFETIME), Some(1.0));
        assert_eq!(fade(10.0, LIFETIME), Some(1.0));
        assert!((fade(10.25, LIFETIME).unwrap() - 0.75).abs() < 1e-6);
        assert_eq!(fade(11.0, LIFETIME), None);
        assert_eq!(query_size(32.0), 32.0);
        assert_eq!(query_size(33.0), 256.0);
    }
}
