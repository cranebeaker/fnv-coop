//! A small software rasterizer: textured, lit triangles into a color and
//! depth buffer. It knows nothing about the game; [`crate::cell`] feeds it.
//!
//! Coordinates are the game's: Z up, right-handed. Lighting is per pixel and
//! done on the textures' stored (gamma-encoded) values, as the game's
//! shaders do: they never convert colors to linear light.

pub type Vec3 = [f32; 3];

pub fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn scale(a: Vec3, s: f32) -> Vec3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn length(a: Vec3) -> f32 {
    dot(a, a).sqrt()
}

pub fn normalize(a: Vec3) -> Vec3 {
    let len = length(a);
    if len > 1e-12 {
        scale(a, 1.0 / len)
    } else {
        a
    }
}

// ---------------------------------------------------------------------------
// Textures and materials
// ---------------------------------------------------------------------------

/// An RGBA8 image, sampled with bilinear filtering and wrapping.
#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; 4]>,
}

impl Texture {
    /// From tightly packed RGBA bytes.
    pub fn from_rgba(width: usize, height: usize, rgba: &[u8]) -> Self {
        assert_eq!(rgba.len(), width * height * 4, "texture size mismatch");
        Self {
            width,
            height,
            pixels: rgba
                .chunks_exact(4)
                .map(|p| [p[0], p[1], p[2], p[3]])
                .collect(),
        }
    }

    pub fn solid(color: [u8; 4]) -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![color],
        }
    }

    /// Bilinear sample at texture coordinates (0..1 wraps), as 0..1 floats.
    pub fn sample(&self, u: f32, v: f32) -> [f32; 4] {
        let (u, v) = if u.is_finite() && v.is_finite() {
            (u, v)
        } else {
            (0.0, 0.0)
        };
        let x = u * self.width as f32 - 0.5;
        let y = v * self.height as f32 - 0.5;
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let wrap = |i: f32, n: usize| (i as i64).rem_euclid(n as i64) as usize;
        let (xa, xb) = (wrap(x0, self.width), wrap(x0 + 1.0, self.width));
        let (ya, yb) = (wrap(y0, self.height), wrap(y0 + 1.0, self.height));
        let p = |x: usize, y: usize| self.pixels[y * self.width + x];
        let (a, b, c, d) = (p(xa, ya), p(xb, ya), p(xa, yb), p(xb, yb));
        let mut out = [0.0; 4];
        for (i, o) in out.iter_mut().enumerate() {
            let top = f32::from(a[i]) * (1.0 - fx) + f32::from(b[i]) * fx;
            let bottom = f32::from(c[i]) * (1.0 - fx) + f32::from(d[i]) * fx;
            *o = (top * (1.0 - fy) + bottom * fy) / 255.0;
        }
        out
    }
}

/// Source and destination factors for alpha blending, numbered as in
/// Gamebryo's `NiAlphaProperty`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    One,
    Zero,
    SrcColor,
    InvSrcColor,
    DstColor,
    InvDstColor,
    SrcAlpha,
    InvSrcAlpha,
    DstAlpha,
    InvDstAlpha,
    SrcAlphaSaturate,
}

impl BlendFactor {
    fn from_bits(bits: u16) -> Self {
        use BlendFactor::*;
        [
            One,
            Zero,
            SrcColor,
            InvSrcColor,
            DstColor,
            InvDstColor,
            SrcAlpha,
            InvSrcAlpha,
            DstAlpha,
            InvDstAlpha,
            SrcAlphaSaturate,
        ]
        .get(usize::from(bits))
        .copied()
        .unwrap_or(One)
    }

    /// There's no destination alpha channel; it reads as opaque.
    fn weight(self, src: [f32; 4], dst: Vec3) -> Vec3 {
        let a = src[3];
        match self {
            BlendFactor::One | BlendFactor::DstAlpha => [1.0; 3],
            BlendFactor::Zero | BlendFactor::InvDstAlpha => [0.0; 3],
            BlendFactor::SrcColor => [src[0], src[1], src[2]],
            BlendFactor::InvSrcColor => [1.0 - src[0], 1.0 - src[1], 1.0 - src[2]],
            BlendFactor::DstColor => dst,
            BlendFactor::InvDstColor => [1.0 - dst[0], 1.0 - dst[1], 1.0 - dst[2]],
            BlendFactor::SrcAlpha => [a; 3],
            BlendFactor::InvSrcAlpha => [1.0 - a; 3],
            // min(source alpha, 1 - destination alpha), with the destination opaque.
            BlendFactor::SrcAlphaSaturate => [0.0; 3],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaTest {
    Always,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Never,
}

impl AlphaTest {
    fn passes(self, alpha: u8, threshold: u8) -> bool {
        match self {
            AlphaTest::Always => true,
            AlphaTest::Less => alpha < threshold,
            AlphaTest::Equal => alpha == threshold,
            AlphaTest::LessEqual => alpha <= threshold,
            AlphaTest::Greater => alpha > threshold,
            AlphaTest::NotEqual => alpha != threshold,
            AlphaTest::GreaterEqual => alpha >= threshold,
            AlphaTest::Never => false,
        }
    }
}

/// How a surface's alpha is used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AlphaMode {
    pub blend: Option<(BlendFactor, BlendFactor)>,
    pub test: Option<(AlphaTest, u8)>,
}

impl AlphaMode {
    pub const OPAQUE: Self = Self {
        blend: None,
        test: None,
    };

    /// From `NiAlphaProperty` flags and threshold.
    pub fn from_nif(flags: u16, threshold: u8) -> Self {
        let blend = (flags & 1 != 0).then(|| {
            (
                BlendFactor::from_bits((flags >> 1) & 0xF),
                BlendFactor::from_bits((flags >> 5) & 0xF),
            )
        });
        let test = (flags & 0x200 != 0).then(|| {
            let func = [
                AlphaTest::Always,
                AlphaTest::Less,
                AlphaTest::Equal,
                AlphaTest::LessEqual,
                AlphaTest::Greater,
                AlphaTest::NotEqual,
                AlphaTest::GreaterEqual,
                AlphaTest::Never,
            ][usize::from((flags >> 10) & 7)];
            (func, threshold)
        });
        Self { blend, test }
    }

    pub fn uses_alpha(&self) -> bool {
        self.blend.is_some() || self.test.is_some()
    }
}

/// Opacity by viewing angle, used by effect surfaces (light beams, haze).
/// Angles are cosines between the view direction and the surface normal:
/// 1 is face-on, 0 edge-on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Falloff {
    pub start_cos: f32,
    pub stop_cos: f32,
    pub start_opacity: f32,
    pub stop_opacity: f32,
}

impl Falloff {
    /// The opacity at a viewing angle, as the game's no-lighting vertex
    /// shaders work it out (`NOLIGHT006.vso`): a smooth S-curve
    /// (`3t² − 2t³`) from the start opacity at the start angle to the stop
    /// opacity at the stop angle.
    pub fn opacity(&self, cos: f32) -> f32 {
        let (a, b) = (self.start_cos, self.stop_cos);
        if (a - b).abs() < 1e-6 {
            return if cos >= a {
                self.start_opacity
            } else {
                self.stop_opacity
            };
        }
        let t = ((cos - a) / (b - a)).clamp(0.0, 1.0);
        let s = t * t * (3.0 - 2.0 * t);
        self.start_opacity + (self.stop_opacity - self.start_opacity) * s
    }
}

/// Everything about a surface except its geometry.
#[derive(Debug, Clone, Copy)]
pub struct Material<'a> {
    pub texture: Option<&'a Texture>,
    /// Multiplies the texture (or is the color, without one).
    pub color: [f32; 4],
    pub alpha: AlphaMode,
    pub double_sided: bool,
    /// False draws at full brightness regardless of lights.
    pub lit: bool,
    /// Self-lit color, added to the light reaching the surface (so it
    /// multiplies the texture, as in the game).
    pub emissive: Vec3,
    /// A glow map, which masks and tints the self-lit color.
    pub glow: Option<&'a Texture>,
    /// Drawn slightly in front of what it lies on, to avoid flicker.
    pub decal: bool,
    /// Blends the final color toward this, for highlighting.
    pub tint: Option<Vec3>,
    /// Fades the surface by viewing angle.
    pub falloff: Option<Falloff>,
    /// Hidden behind what's nearer (tested against the depth buffer).
    pub depth_test: bool,
    /// Hides what's drawn later behind it (writes the depth buffer), blended
    /// or not.
    pub depth_write: bool,
}

impl Default for Material<'_> {
    fn default() -> Self {
        Self {
            texture: None,
            color: [1.0; 4],
            alpha: AlphaMode::OPAQUE,
            double_sided: false,
            lit: true,
            emissive: [0.0; 3],
            glow: None,
            decal: false,
            tint: None,
            falloff: None,
            depth_test: true,
            depth_write: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Lights, cameras, planes
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Vec3,
    /// 0..1 per channel, already scaled by brightness.
    pub color: Vec3,
    pub radius: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lighting {
    pub ambient: Vec3,
    /// Direction toward the light, and its color.
    pub directional: Option<(Vec3, Vec3)>,
    pub points: Vec<PointLight>,
}

impl Lighting {
    /// Light reaching a point with the given normal, as the game's lit
    /// shaders add it up (`SLS2029` in `shaderpackage013.sdp`, which lights
    /// with the directional light and five point lights in one pass):
    /// ambient, plus each light's color times `saturate(N·L)`, point lights
    /// also times `1 - saturate(d² / r²)`, and the sum kept at 0 or above.
    /// The normal is never flipped, so the back of a two-sided surface takes
    /// only light that is on its front side (those shaders have no way to
    /// tell which side is drawn).
    pub fn at(&self, p: Vec3, n: Vec3, points: &[usize]) -> Vec3 {
        let facing = |d: f32| d.clamp(0.0, 1.0);
        let mut out = self.ambient;
        if let Some((dir, color)) = self.directional {
            out = add(out, scale(color, facing(dot(n, dir))));
        }
        for &i in points {
            let light = &self.points[i];
            let to = sub(light.position, p);
            let d2 = dot(to, to);
            let r2 = light.radius * light.radius;
            if d2 >= r2 || r2 <= 0.0 {
                continue;
            }
            let attenuation = 1.0 - d2 / r2;
            let lambert = if d2 > 1e-6 {
                facing(dot(n, scale(to, 1.0 / d2.sqrt())))
            } else {
                1.0
            };
            out = add(out, scale(light.color, attenuation * lambert));
        }
        out.map(|c| c.max(0.0))
    }

    /// Indices of the point lights that can reach a sphere.
    pub fn reaching(&self, center: Vec3, radius: f32) -> Vec<usize> {
        self.points
            .iter()
            .enumerate()
            .filter(|(_, l)| length(sub(l.position, center)) < l.radius + radius)
            .map(|(i, _)| i)
            .collect()
    }
}

/// Keeps points where `dot(normal, p) + offset >= 0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
    pub offset: f32,
}

impl Plane {
    /// Keeps everything at or below a height.
    pub fn below(z: f32) -> Self {
        Self {
            normal: [0.0, 0.0, -1.0],
            offset: z,
        }
    }

    pub fn distance(&self, p: Vec3) -> f32 {
        dot(self.normal, p) + self.offset
    }
}

/// Where a triangle crosses a plane, if it does.
pub fn plane_segment(tri: [Vec3; 3], plane: Plane) -> Option<(Vec3, Vec3)> {
    let d = tri.map(|p| plane.distance(p));
    let mut points = Vec::with_capacity(2);
    for i in 0..3 {
        let j = (i + 1) % 3;
        if (d[i] < 0.0) != (d[j] < 0.0) {
            let t = d[i] / (d[i] - d[j]);
            points.push(add(tri[i], scale(sub(tri[j], tri[i]), t)));
        }
    }
    (points.len() == 2).then(|| (points[0], points[1]))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    /// Horizontal field of view in radians.
    Perspective { fov_x: f32 },
    /// Half the width of the view, in game units.
    Orthographic { half_width: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub projection: Projection,
    pub near: f32,
}

impl Camera {
    /// Looking along `forward`, with `up_hint` roughly up.
    pub fn new(eye: Vec3, forward: Vec3, up_hint: Vec3, projection: Projection) -> Self {
        let forward = normalize(forward);
        let mut right = cross(forward, up_hint);
        if length(right) < 1e-6 {
            right = cross(forward, [0.0, 1.0, 0.0]);
        }
        let right = normalize(right);
        let up = cross(right, forward);
        Self {
            eye,
            forward,
            right,
            up,
            projection,
            near: 1.0,
        }
    }

    /// Standing at `eye`, facing a compass heading (radians clockwise from
    /// north, +Y) and pitched up by `pitch` radians.
    pub fn first_person(eye: Vec3, heading: f32, pitch: f32, fov_x: f32) -> Self {
        let forward = [
            heading.sin() * pitch.cos(),
            heading.cos() * pitch.cos(),
            pitch.sin(),
        ];
        let mut camera = Self::new(
            eye,
            forward,
            [0.0, 0.0, 1.0],
            Projection::Perspective { fov_x },
        );
        camera.near = 4.0;
        camera
    }

    /// Looking straight down from height `z`, north up.
    pub fn top_down(center: [f32; 2], z: f32, half_width: f32) -> Self {
        let mut camera = Self::new(
            [center[0], center[1], z],
            [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0],
            Projection::Orthographic { half_width },
        );
        camera.up = [0.0, 1.0, 0.0];
        camera.right = [1.0, 0.0, 0.0];
        camera.near = 0.0;
        camera
    }

    /// World to camera space: x right, y up, z forward (depth).
    pub fn to_view(&self, p: Vec3) -> Vec3 {
        let d = sub(p, self.eye);
        [dot(d, self.right), dot(d, self.up), dot(d, self.forward)]
    }

    fn is_perspective(&self) -> bool {
        matches!(self.projection, Projection::Perspective { .. })
    }

    /// Camera space to pixel coordinates on a `width` × `height` image.
    fn view_to_screen(&self, v: Vec3, width: usize, height: usize) -> (f32, f32) {
        let aspect = width as f32 / height as f32;
        let (nx, ny) = match self.projection {
            Projection::Perspective { fov_x } => {
                let f = 1.0 / (fov_x * 0.5).tan();
                (v[0] * f / v[2], v[1] * f * aspect / v[2])
            }
            Projection::Orthographic { half_width } => {
                (v[0] / half_width, v[1] * aspect / half_width)
            }
        };
        (
            (nx + 1.0) * 0.5 * width as f32,
            (1.0 - ny) * 0.5 * height as f32,
        )
    }

    /// Where a world point lands on a `width` × `height` image, if it's in
    /// front of the camera.
    pub fn project(&self, p: Vec3, width: usize, height: usize) -> Option<(f32, f32)> {
        let v = self.to_view(p);
        (v[2] > self.near || !self.is_perspective()).then(|| self.view_to_screen(v, width, height))
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

/// Geometry in world space. Missing normals, UVs or colors may be empty.
pub struct MeshInput<'a> {
    pub positions: &'a [Vec3],
    pub normals: &'a [Vec3],
    pub uvs: &'a [[f32; 2]],
    pub colors: &'a [[f32; 4]],
    pub triangles: &'a [[u16; 3]],
}

/// Counters for a render.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stats {
    pub triangles_in: usize,
    pub culled: usize,
    pub clipped_away: usize,
    pub drawn: usize,
    pub blended: usize,
}

#[derive(Clone, Copy)]
struct Vert {
    /// World position, then camera-space position after projection starts.
    p: Vec3,
    world: Vec3,
    normal: Vec3,
    uv: [f32; 2],
    color: [f32; 4],
}

impl Vert {
    fn lerp(&self, o: &Vert, t: f32) -> Vert {
        let l = |a: f32, b: f32| a + (b - a) * t;
        let l3 = |a: Vec3, b: Vec3| [l(a[0], b[0]), l(a[1], b[1]), l(a[2], b[2])];
        Vert {
            p: l3(self.p, o.p),
            world: l3(self.world, o.world),
            normal: l3(self.normal, o.normal),
            uv: [l(self.uv[0], o.uv[0]), l(self.uv[1], o.uv[1])],
            color: [
                l(self.color[0], o.color[0]),
                l(self.color[1], o.color[1]),
                l(self.color[2], o.color[2]),
                l(self.color[3], o.color[3]),
            ],
        }
    }
}

/// A vertex on screen: pixel position, depth, 1/depth for perspective
/// correction, and attributes.
#[derive(Clone, Copy)]
struct ScreenVert {
    x: f32,
    y: f32,
    z: f32,
    q: f32,
    /// uv, color, world position, normal.
    attrs: [f32; ATTRS],
}

const ATTRS: usize = 12;

struct Deferred<'a> {
    depth: f32,
    tri: [ScreenVert; 3],
    material: Material<'a>,
    lights: usize,
}

/// Keeps the part of a polygon on the positive side of `distance`.
fn clip_polygon(poly: &[Vert], distance: impl Fn(&Vert) -> f32) -> Vec<Vert> {
    let mut out = Vec::with_capacity(poly.len() + 2);
    for i in 0..poly.len() {
        let a = &poly[i];
        let b = &poly[(i + 1) % poly.len()];
        let (da, db) = (distance(a), distance(b));
        if da >= 0.0 {
            out.push(*a);
        }
        if (da >= 0.0) != (db >= 0.0) {
            out.push(a.lerp(b, da / (da - db)));
        }
    }
    out
}

/// A color and depth buffer with a camera, drawing meshes into it.
pub struct Renderer<'a> {
    pub width: usize,
    pub height: usize,
    pub color: Vec<Vec3>,
    pub depth: Vec<f32>,
    pub camera: Camera,
    pub clip_planes: Vec<Plane>,
    /// Skip triangles facing away from the camera (unless double-sided).
    pub cull: bool,
    pub stats: Stats,
    /// Lights for [`Renderer::draw`].
    pub lighting: Lighting,
    deferred: Vec<Deferred<'a>>,
    /// Point lights reaching each mesh drawn so far.
    light_sets: Vec<Vec<usize>>,
}

impl<'a> Renderer<'a> {
    pub fn new(width: usize, height: usize, background: Vec3, camera: Camera) -> Self {
        Self {
            width,
            height,
            color: vec![background; width * height],
            depth: vec![f32::INFINITY; width * height],
            camera,
            clip_planes: Vec::new(),
            cull: true,
            stats: Stats::default(),
            lighting: Lighting::default(),
            deferred: Vec::new(),
            light_sets: Vec::new(),
        }
    }

    /// Draws a mesh, lit per pixel by [`Renderer::lighting`]. Blended
    /// surfaces are held back and drawn by [`Renderer::finish`], farthest
    /// first.
    pub fn draw(&mut self, mesh: &MeshInput, material: Material<'a>) {
        let n = mesh.positions.len();
        if n == 0 || mesh.triangles.is_empty() {
            return;
        }
        let has_normals = mesh.normals.len() == n;
        let (center, radius) = bounding_sphere(mesh.positions);
        let lights = if material.lit {
            self.lighting.reaching(center, radius)
        } else {
            Vec::new()
        };
        self.light_sets.push(lights);
        let light_set = self.light_sets.len() - 1;

        // Which winding faces outward: vote with the vertex normals.
        let front_is_ccw = !has_normals || {
            let mut votes = 0i64;
            for t in mesh.triangles {
                let [a, b, c] = t.map(usize::from);
                if a.max(b).max(c) >= n {
                    continue;
                }
                let face = face_normal(mesh.positions[a], mesh.positions[b], mesh.positions[c]);
                let avg = add(add(mesh.normals[a], mesh.normals[b]), mesh.normals[c]);
                votes += if dot(face, avg) >= 0.0 { 1 } else { -1 };
            }
            votes >= 0
        };

        for t in mesh.triangles {
            self.stats.triangles_in += 1;
            let [a, b, c] = t.map(usize::from);
            if a.max(b).max(c) >= n {
                continue;
            }
            let ps = [mesh.positions[a], mesh.positions[b], mesh.positions[c]];
            let mut face = face_normal(ps[0], ps[1], ps[2]);
            if !front_is_ccw {
                face = scale(face, -1.0);
            }
            if self.cull && !material.double_sided {
                let toward_camera = if self.camera.is_perspective() {
                    sub(self.camera.eye, ps[0])
                } else {
                    scale(self.camera.forward, -1.0)
                };
                if dot(face, toward_camera) <= 0.0 {
                    self.stats.culled += 1;
                    continue;
                }
            }
            let verts: Vec<Vert> = [a, b, c]
                .iter()
                .enumerate()
                .map(|(k, &i)| {
                    let vc = mesh.colors.get(i).copied().unwrap_or([1.0; 4]);
                    Vert {
                        p: ps[k],
                        world: ps[k],
                        normal: if has_normals { mesh.normals[i] } else { face },
                        uv: mesh.uvs.get(i).copied().unwrap_or([0.0; 2]),
                        color: [
                            material.color[0] * vc[0],
                            material.color[1] * vc[1],
                            material.color[2] * vc[2],
                            material.color[3] * vc[3],
                        ],
                    }
                })
                .collect();
            self.draw_polygon(verts, material, light_set);
        }
    }

    fn draw_polygon(&mut self, mut poly: Vec<Vert>, material: Material<'a>, lights: usize) {
        for plane in &self.clip_planes {
            poly = clip_polygon(&poly, |v| plane.distance(v.p));
            if poly.len() < 3 {
                self.stats.clipped_away += 1;
                return;
            }
        }
        // Into camera space, then clip against the near plane.
        for v in &mut poly {
            v.p = self.camera.to_view(v.p);
        }
        let near = self.camera.near;
        poly = clip_polygon(&poly, |v| v.p[2] - near);
        if poly.len() < 3 {
            self.stats.clipped_away += 1;
            return;
        }
        let perspective = self.camera.is_perspective();
        let bias = if material.decal {
            0.4 + 0.0015 * poly[0].p[2].abs()
        } else {
            0.0
        };
        let screen: Vec<ScreenVert> = poly
            .iter()
            .map(|v| {
                let (x, y) = self.camera.view_to_screen(v.p, self.width, self.height);
                let z = v.p[2] - bias;
                let q = if perspective { 1.0 / z.max(1e-4) } else { 1.0 };
                // Premultiplied by q for perspective-correct interpolation.
                let a = [
                    v.uv[0],
                    v.uv[1],
                    v.color[0],
                    v.color[1],
                    v.color[2],
                    v.color[3],
                    v.world[0],
                    v.world[1],
                    v.world[2],
                    v.normal[0],
                    v.normal[1],
                    v.normal[2],
                ];
                ScreenVert {
                    x,
                    y,
                    z,
                    q,
                    attrs: a.map(|value| value * q),
                }
            })
            .collect();
        self.stats.drawn += 1;
        for i in 1..screen.len() - 1 {
            let tri = [screen[0], screen[i], screen[i + 1]];
            if material.alpha.blend.is_some() {
                self.stats.blended += 1;
                self.deferred.push(Deferred {
                    depth: (tri[0].z + tri[1].z + tri[2].z) / 3.0,
                    tri,
                    material,
                    lights,
                });
            } else {
                self.raster(&tri, &material, false, lights);
            }
        }
    }

    /// Draws the held-back blended surfaces, farthest first.
    pub fn finish(&mut self) {
        let mut deferred = std::mem::take(&mut self.deferred);
        deferred.sort_by(|a, b| b.depth.total_cmp(&a.depth));
        for d in &deferred {
            self.raster(&d.tri, &d.material, true, d.lights);
        }
    }

    fn raster(&mut self, tri: &[ScreenVert; 3], material: &Material, blend: bool, lights: usize) {
        let [v0, v1, v2] = tri;
        let area = (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x);
        if !area.is_finite() || area.abs() < 1e-8 {
            return;
        }
        let min_x = v0.x.min(v1.x).min(v2.x).floor().max(0.0) as usize;
        let min_y = v0.y.min(v1.y).min(v2.y).floor().max(0.0) as usize;
        let max_x = (v0.x.max(v1.x).max(v2.x).ceil() as isize).min(self.width as isize - 1);
        let max_y = (v0.y.max(v1.y).max(v2.y).ceil() as isize).min(self.height as isize - 1);
        if max_x < 0 || max_y < 0 {
            return;
        }
        let (max_x, max_y) = (max_x as usize, max_y as usize);
        let inv_area = 1.0 / area;
        let perspective = self.camera.is_perspective();
        let (a0, a1, a2) = (&v0.attrs, &v1.attrs, &v2.attrs);
        let lighting = &self.lighting;
        let light_set = &self.light_sets[lights];
        let camera = self.camera;
        let edge = |a: &ScreenVert, b: &ScreenVert, x: f32, y: f32| {
            (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
        };

        for py in min_y..=max_y {
            let y = py as f32 + 0.5;
            for px in min_x..=max_x {
                let x = px as f32 + 0.5;
                let w0 = edge(v1, v2, x, y) * inv_area;
                let w1 = edge(v2, v0, x, y) * inv_area;
                let w2 = edge(v0, v1, x, y) * inv_area;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let q = w0 * v0.q + w1 * v1.q + w2 * v2.q;
                let depth = if perspective {
                    1.0 / q
                } else {
                    w0 * v0.z + w1 * v1.z + w2 * v2.z
                };
                let index = py * self.width + px;
                if material.depth_test && depth >= self.depth[index] {
                    continue;
                }
                let inv_q = 1.0 / q;
                let at = |k: usize| (w0 * a0[k] + w1 * a1[k] + w2 * a2[k]) * inv_q;
                let (u, v) = (at(0), at(1));
                let mut shade = [at(2), at(3), at(4), at(5)];
                if material.lit {
                    let world = [at(6), at(7), at(8)];
                    let normal = normalize([at(9), at(10), at(11)]);
                    let e = material.emissive;
                    let emissive = match material.glow {
                        Some(glow) => {
                            let g = glow.sample(u, v);
                            [e[0] * g[0], e[1] * g[1], e[2] * g[2]]
                        }
                        None => e,
                    };
                    let light = add(lighting.at(world, normal, light_set), emissive);
                    shade = [
                        shade[0] * light[0],
                        shade[1] * light[1],
                        shade[2] * light[2],
                        shade[3],
                    ];
                }
                let texel = material.texture.map_or([1.0; 4], |t| t.sample(u, v));
                let alpha = if material.alpha.uses_alpha() {
                    let fade = material.falloff.map_or(1.0, |f| {
                        let world = [at(6), at(7), at(8)];
                        let normal = normalize([at(9), at(10), at(11)]);
                        let view = if perspective {
                            normalize(sub(camera.eye, world))
                        } else {
                            scale(camera.forward, -1.0)
                        };
                        f.opacity(dot(normal, view).abs())
                    });
                    (texel[3] * shade[3] * fade).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                if let Some((test, threshold)) = material.alpha.test {
                    if !test.passes((alpha * 255.0).round() as u8, threshold) {
                        continue;
                    }
                }
                let mut rgb = [
                    texel[0] * shade[0],
                    texel[1] * shade[1],
                    texel[2] * shade[2],
                ];
                if let Some(tint) = material.tint {
                    rgb = add(scale(rgb, 0.35), scale(tint, 0.65));
                }
                if blend {
                    let (src_f, dst_f) = material
                        .alpha
                        .blend
                        .unwrap_or((BlendFactor::SrcAlpha, BlendFactor::InvSrcAlpha));
                    let src = [rgb[0], rgb[1], rgb[2], alpha];
                    let dst = self.color[index];
                    let sw = src_f.weight(src, dst);
                    let dw = dst_f.weight(src, dst);
                    self.color[index] = [
                        rgb[0] * sw[0] + dst[0] * dw[0],
                        rgb[1] * sw[1] + dst[1] * dw[1],
                        rgb[2] * sw[2] + dst[2] * dw[2],
                    ];
                } else {
                    self.color[index] = rgb;
                }
                if material.depth_write {
                    self.depth[index] = depth;
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Flat overlays, in pixel coordinates of this renderer
    // -----------------------------------------------------------------------

    /// A filled circle.
    pub fn disc(&mut self, cx: f32, cy: f32, radius: f32, color: Vec3) {
        self.segment(cx, cy, cx, cy, radius * 2.0, color);
    }

    /// A line `width` pixels wide with round ends.
    pub fn segment(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: Vec3) {
        let r = width * 0.5;
        let min_x = (x0.min(x1) - r - 1.0).floor().max(0.0) as usize;
        let min_y = (y0.min(y1) - r - 1.0).floor().max(0.0) as usize;
        let max_x = ((x0.max(x1) + r + 1.0).ceil() as isize).min(self.width as isize - 1);
        let max_y = ((y0.max(y1) + r + 1.0).ceil() as isize).min(self.height as isize - 1);
        if max_x < 0 || max_y < 0 {
            return;
        }
        let (dx, dy) = (x1 - x0, y1 - y0);
        let len2 = dx * dx + dy * dy;
        for py in min_y..=max_y as usize {
            for px in min_x..=max_x as usize {
                let (x, y) = (px as f32 + 0.5, py as f32 + 0.5);
                let t = if len2 > 0.0 {
                    (((x - x0) * dx + (y - y0) * dy) / len2).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let (cx, cy) = (x0 + dx * t, y0 + dy * t);
                let d = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                let coverage = (r + 0.5 - d).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    let i = py * self.width + px;
                    let old = self.color[i];
                    self.color[i] = add(scale(old, 1.0 - coverage), scale(color, coverage));
                }
            }
        }
    }

    /// A line between two world points, if both are in front of the camera.
    pub fn world_segment(&mut self, a: Vec3, b: Vec3, width: f32, color: Vec3) {
        let (w, h) = (self.width, self.height);
        if let (Some(p), Some(q)) = (self.camera.project(a, w, h), self.camera.project(b, w, h)) {
            self.segment(p.0, p.1, q.0, q.1, width, color);
        }
    }

    /// The image as RGBA8, shrunk by averaging `factor` × `factor` blocks
    /// (supersampling). Colors are clamped to 0..1.
    pub fn to_rgba8(&self, factor: usize) -> (usize, usize, Vec<u8>) {
        let factor = factor.max(1);
        let (w, h) = (self.width / factor, self.height / factor);
        let mut out = Vec::with_capacity(w * h * 4);
        let norm = 1.0 / (factor * factor) as f32;
        for y in 0..h {
            for x in 0..w {
                let mut sum = [0.0f32; 3];
                for sy in 0..factor {
                    for sx in 0..factor {
                        let c = self.color[(y * factor + sy) * self.width + x * factor + sx];
                        for k in 0..3 {
                            sum[k] += c[k].clamp(0.0, 1.0);
                        }
                    }
                }
                for s in sum {
                    out.push((s * norm * 255.0).round() as u8);
                }
                out.push(255);
            }
        }
        (w, h, out)
    }
}

/// The normal of a counter-clockwise triangle (unnormalized length is
/// twice the area).
fn face_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    normalize(cross(sub(b, a), sub(c, a)))
}

fn bounding_sphere(points: &[Vec3]) -> (Vec3, f32) {
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in points {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let center = scale(add(lo, hi), 0.5);
    (center, length(sub(hi, center)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad(z: f32, size: f32) -> (Vec<Vec3>, Vec<[u16; 3]>) {
        (
            vec![
                [-size, -size, z],
                [size, -size, z],
                [size, size, z],
                [-size, size, z],
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        )
    }

    fn flat(color: [f32; 4]) -> Material<'static> {
        Material {
            color,
            lit: false,
            ..Material::default()
        }
    }

    /// A pixel, rounded to hide float noise from interpolation.
    fn pixel(r: &Renderer, x: usize, y: usize) -> Vec3 {
        r.color[y * r.width + x].map(|c| (c * 1000.0).round() / 1000.0)
    }

    #[test]
    fn nearer_surfaces_hide_farther_ones_whatever_the_order() {
        let camera = Camera::top_down([0.0, 0.0], 100.0, 10.0);
        let (low, tris) = quad(0.0, 5.0);
        let (high, _) = quad(10.0, 2.0);
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        for (positions, color) in [(&high, [1.0, 0.0, 0.0, 1.0]), (&low, [0.0, 1.0, 0.0, 1.0])] {
            r.draw(
                &MeshInput {
                    positions,
                    normals: &[],
                    uvs: &[],
                    colors: &[],
                    triangles: &tris,
                },
                flat(color),
            );
        }
        assert_eq!(pixel(&r, 10, 10), [1.0, 0.0, 0.0]); // the high quad
        assert_eq!(pixel(&r, 5, 10), [0.0, 1.0, 0.0]); // only the low one
        assert_eq!(pixel(&r, 0, 0), [0.0; 3]); // background
        assert_eq!(r.stats.drawn, 4);
    }

    #[test]
    fn culls_back_faces_and_clips_above_the_cut() {
        let camera = Camera::top_down([0.0, 0.0], 100.0, 10.0);
        let (positions, tris) = quad(0.0, 5.0);
        let reversed: Vec<[u16; 3]> = tris.iter().map(|t| [t[0], t[2], t[1]]).collect();
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &[],
                uvs: &[],
                colors: &[],
                triangles: &reversed,
            },
            flat([1.0; 4]),
        );
        assert_eq!(r.stats.culled, 2);
        assert_eq!(pixel(&r, 10, 10), [0.0; 3]);

        // With normals pointing down, the reversed winding is the front...
        let down = vec![[0.0, 0.0, -1.0]; 4];
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &down,
                uvs: &[],
                colors: &[],
                triangles: &tris,
            },
            flat([1.0; 4]),
        );
        assert_eq!(r.stats.culled, 4);

        // ...and a cut below the quad removes it entirely.
        r.clip_planes.push(Plane::below(-1.0));
        r.cull = false;
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &[],
                uvs: &[],
                colors: &[],
                triangles: &tris,
            },
            flat([1.0; 4]),
        );
        assert_eq!(r.stats.clipped_away, 2);
        assert_eq!(pixel(&r, 10, 10), [0.0; 3]);
    }

    #[test]
    fn perspective_projection_centers_the_view_direction() {
        let camera = Camera::first_person([0.0, 0.0, 0.0], 0.0, 0.0, 90f32.to_radians());
        let (x, y) = camera.project([0.0, 100.0, 0.0], 200, 100).unwrap();
        assert!((x - 100.0).abs() < 1e-3 && (y - 50.0).abs() < 1e-3);
        // 45 degrees to the right lands on the right edge.
        let (x, _) = camera.project([100.0, 100.0, 0.0], 200, 100).unwrap();
        assert!((x - 200.0).abs() < 1e-3);
        // Heading east.
        let east = Camera::first_person([0.0; 3], std::f32::consts::FRAC_PI_2, 0.0, 1.0);
        assert!((east.forward[0] - 1.0).abs() < 1e-6);
        assert!(camera.project([0.0, -100.0, 0.0], 200, 100).is_none());
    }

    #[test]
    fn textures_sample_bilinearly_and_wrap() {
        let t = Texture::from_rgba(2, 1, &[0, 0, 0, 255, 255, 255, 255, 255]);
        assert_eq!(t.sample(0.25, 0.5)[0], 0.0);
        assert_eq!(t.sample(0.75, 0.5)[0], 1.0);
        assert!((t.sample(0.5, 0.5)[0] - 0.5).abs() < 1e-6);
        assert!((t.sample(1.0, 0.5)[0] - 0.5).abs() < 1e-6); // wraps
        assert_eq!(t.sample(f32::NAN, 0.0)[3], 1.0);
    }

    #[test]
    fn reads_nif_alpha_flags() {
        // 0x00ED: blend, src = SrcAlpha, dst = InvSrcAlpha. 0x12EC: test Greater.
        let blend = AlphaMode::from_nif(0x00ED, 0);
        assert_eq!(
            blend.blend,
            Some((BlendFactor::SrcAlpha, BlendFactor::InvSrcAlpha))
        );
        assert_eq!(blend.test, None);
        let test = AlphaMode::from_nif(0x12EC, 128);
        assert_eq!(test.blend, None);
        assert_eq!(test.test, Some((AlphaTest::Greater, 128)));
        assert!(!AlphaMode::from_nif(0x00EC, 0).uses_alpha());
    }

    #[test]
    fn blended_and_cut_out_surfaces() {
        let camera = Camera::top_down([0.0, 0.0], 100.0, 10.0);
        let (positions, tris) = quad(0.0, 5.0);
        let (above, _) = quad(5.0, 5.0);
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        let draw = |r: &mut Renderer<'static>, p: &[Vec3], m: Material<'static>| {
            r.draw(
                &MeshInput {
                    positions: p,
                    normals: &[],
                    uvs: &[],
                    colors: &[],
                    triangles: &tris,
                },
                m,
            );
        };
        draw(&mut r, &positions, flat([0.0, 0.0, 1.0, 1.0]));
        // Half-transparent red above: blended over the blue.
        let mut glass = flat([1.0, 0.0, 0.0, 0.5]);
        glass.alpha = AlphaMode::from_nif(0x00ED, 0);
        draw(&mut r, &above, glass);
        assert_eq!(pixel(&r, 10, 10), [0.0, 0.0, 1.0]); // not until finish()
        r.finish();
        assert_eq!(pixel(&r, 10, 10), [0.5, 0.0, 0.5]);

        // A cut-out surface below the threshold vanishes.
        let mut leaf = flat([0.0, 1.0, 0.0, 0.2]);
        leaf.alpha = AlphaMode::from_nif(0x12EC, 128);
        draw(&mut r, &above, leaf);
        assert_eq!(pixel(&r, 10, 10), [0.5, 0.0, 0.5]);
    }

    #[test]
    fn point_lights_fade_with_distance() {
        let lighting = Lighting {
            ambient: [0.1; 3],
            directional: None,
            points: vec![PointLight {
                position: [0.0, 0.0, 100.0],
                color: [1.0, 0.5, 0.0],
                radius: 200.0,
            }],
        };
        let under = lighting.at([0.0; 3], [0.0, 0.0, 1.0], &[0]);
        assert!((under[0] - (0.1 + 0.75)).abs() < 1e-5, "{under:?}");
        let facing_away = lighting.at([0.0; 3], [0.0, 0.0, -1.0], &[0]);
        assert_eq!(facing_away, [0.1; 3]);
        let far = lighting.at([0.0, 0.0, -150.0], [0.0, 0.0, 1.0], &[0]);
        assert_eq!(far, [0.1; 3]);
        assert_eq!(
            lighting.reaching([0.0, 0.0, 500.0], 10.0),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn lights_each_pixel_so_big_quads_get_pools_of_light() {
        let camera = Camera::top_down([0.0, 0.0], 100.0, 100.0);
        // Every corner is outside the light's reach; only the middle is lit.
        let (positions, tris) = quad(0.0, 100.0);
        let up = vec![[0.0, 0.0, 1.0]; 4];
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        r.lighting = Lighting {
            ambient: [0.1; 3],
            directional: None,
            points: vec![PointLight {
                position: [0.0, 0.0, 30.0],
                color: [1.0; 3],
                radius: 100.0,
            }],
        };
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &up,
                uvs: &[],
                colors: &[],
                triangles: &tris,
            },
            Material::default(),
        );
        let middle = pixel(&r, 10, 10)[0];
        let corner = pixel(&r, 0, 0)[0];
        assert!(middle > 0.9, "{middle}");
        assert!((corner - 0.1).abs() < 1e-3, "{corner}");
    }

    #[test]
    fn two_sided_surfaces_take_light_only_on_their_front() {
        // The game's lit shaders never turn the normal around, so a light
        // under a two-sided floor leaves its top at the ambient level.
        let camera = Camera::top_down([0.0, 0.0], 100.0, 100.0);
        let (positions, tris) = quad(0.0, 100.0);
        let up = vec![[0.0, 0.0, 1.0]; 4];
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        r.lighting = Lighting {
            ambient: [0.1; 3],
            directional: Some(([0.0, 0.0, -1.0], [0.5; 3])),
            points: vec![PointLight {
                position: [0.0, 0.0, -30.0],
                color: [1.0; 3],
                radius: 100.0,
            }],
        };
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &up,
                uvs: &[],
                colors: &[],
                triangles: &tris,
            },
            Material {
                double_sided: true,
                ..Material::default()
            },
        );
        let middle = pixel(&r, 10, 10)[0];
        assert!((middle - 0.1).abs() < 1e-3, "{middle}");
    }

    #[test]
    fn effect_surfaces_fade_by_viewing_angle() {
        let f = Falloff {
            start_cos: 1.0,
            stop_cos: 0.0,
            start_opacity: 0.8,
            stop_opacity: 0.0,
        };
        assert!((f.opacity(1.0) - 0.8).abs() < 1e-6);
        assert!((f.opacity(0.5) - 0.4).abs() < 1e-6);
        assert_eq!(f.opacity(0.0), 0.0);
        // The game's S-curve, not a straight line: a quarter of the way
        // from the start angle it has faded by 3t² − 2t³ = 0.15625.
        assert!(
            (f.opacity(0.75) - 0.675).abs() < 1e-6,
            "{}",
            f.opacity(0.75)
        );

        // A glow card seen face-on from above, added onto a dark floor.
        let camera = Camera::top_down([0.0, 0.0], 100.0, 10.0);
        let (floor, tris) = quad(0.0, 5.0);
        let (card, _) = quad(5.0, 5.0);
        let up = vec![[0.0, 0.0, 1.0]; 4];
        let mut r = Renderer::new(20, 20, [0.0; 3], camera);
        let draw = |r: &mut Renderer<'static>, p: &[Vec3], m: Material<'static>| {
            r.draw(
                &MeshInput {
                    positions: p,
                    normals: &up,
                    uvs: &[],
                    colors: &[],
                    triangles: &tris,
                },
                m,
            );
        };
        draw(&mut r, &floor, flat([0.1, 0.1, 0.1, 1.0]));
        let mut glow = flat([1.0, 1.0, 1.0, 0.5]); // material alpha 0.5
        glow.alpha = AlphaMode::from_nif(0x000D, 0); // SrcAlpha, One: additive
        glow.falloff = Some(Falloff {
            start_opacity: 0.4,
            ..f
        });
        draw(&mut r, &card, glow);
        r.finish();
        // 0.1 + 1.0 * (0.5 * 0.4)
        assert!(
            (pixel(&r, 10, 10)[0] - 0.3).abs() < 1e-3,
            "{:?}",
            pixel(&r, 10, 10)
        );
    }

    #[test]
    fn finds_where_triangles_cross_a_plane() {
        let tri = [[0.0, 0.0, 0.0], [10.0, 0.0, 20.0], [0.0, 10.0, 20.0]];
        let (a, b) = plane_segment(tri, Plane::below(10.0)).unwrap();
        assert_eq!(a, [5.0, 0.0, 10.0]);
        assert_eq!(b, [0.0, 5.0, 10.0]);
        assert!(plane_segment(tri, Plane::below(30.0)).is_none());
    }

    #[test]
    fn downsamples_by_averaging() {
        let camera = Camera::top_down([0.0, 0.0], 10.0, 1.0);
        let mut r = Renderer::new(4, 2, [0.0; 3], camera);
        r.color[0] = [1.0; 3];
        r.color[1] = [2.0; 3]; // clamped
        let (w, h, px) = r.to_rgba8(2);
        assert_eq!((w, h), (2, 1));
        assert_eq!(&px[..4], &[128, 128, 128, 255]);
    }
}
