//! Collision queries for the character proxy ([`crate::proxy`]): the game's
//! character shape against the walking collider's triangles and against
//! other characters, giving Havok-style contact points
//! (`hkpRootCdPoint`, Xbox PDB).
//!
//! The shape is the game's (`00c70de0`, below). The cast is the game's own
//! (`hkpGskBaseAgent::staticLinearCast` `00daf8e0`, [`gsk_linear_cast`]:
//! conservative advancement on the closest points, with the world's
//! iterative linear cast settings `hkpWorldCinfo` +0x9c/+0xa0: early-out
//! 0.01, 20 iterations), and the closest points keep the agent's outputs
//! (`staticGetClosestPoints` `00dac450`: the normal from the second
//! shape toward the first, the distance less both convex radii, the point
//! on the second shape's surface, only within the tolerance). The GJK
//! underneath (`00daad40` and its helpers) and the penetration depth for
//! overlapping cores (`00daa7e0`) are not translated (HAVOK_MAP open
//! question 7): here they are generic closest-point queries (GJK, and
//! separating axes when the shapes' cores overlap), so which of several
//! equally close points is the contact point is ours (`contact_on`).
//! Everything here is in Havok units, relative to an origin near the
//! character, so positions far out in a worldspace keep their precision.

use crate::vec::*;
use crate::{layers, Collider, Vec3, ANY_LAYER, HAVOK_UNIT};

/// The convex radius of the character's hull: 0.1 Havok units (`00cd7f50`
/// builds the convex vertices shape with the float 0.1).
pub const HULL_RADIUS: f32 = 0.1;

/// The world's iterative linear cast settings (`hkpWorldCinfo`
/// `m_iterativeLinearCastEarlyOutDistance` 0.01, `...MaxIterations` 20,
/// Havok's defaults kept by the game; HAVOK_MAP section 4).
pub const CAST_EARLY_OUT: f32 = 0.01;
pub const CAST_MAX_ITERATIONS: usize = 20;

/// The cast collector's early-out distance: the proxy's collector is an
/// all-points collector (`hkpAllCdPointCollector`, its early-out distance
/// the maximum float, `7f7fffee` as the collector constructor stores it).
const COLLECTOR_EARLY_OUT: f32 = f32::MAX;

/// The ring directions of the character's hull (`011b0154` x, `011b0174`
/// y: every 45 degrees, 0.7071 as stored).
#[allow(clippy::approx_constant)]
const RING: [[f32; 2]; 8] = [
    [1.0, 0.0],
    [0.7071, 0.7071],
    [0.0, 1.0],
    [-0.7071, 0.7071],
    [-1.0, 0.0],
    [-0.7071, -0.7071],
    [0.0, -1.0],
    [0.7071, -0.7071],
];

/// A convex shape: points around an origin, with a convex radius (Havok
/// units).
#[derive(Debug, Clone, PartialEq)]
pub struct Hull {
    pub vertices: Vec<Vec3>,
    pub radius: f32,
    /// Whether the points are the character's 18 ([`Hull::character`]):
    /// its faces and edges are known, for the separating-axis test.
    character: bool,
}

impl Hull {
    /// The character's hull (`00c70de0`, called from the shape builder
    /// `00c72410`), relative to the shape's centre, in Havok units: from
    /// the capsule's bottom and top axis points `bottom`, `top` and radius
    /// (game units) and the step height (Havok units). One point at the
    /// bottom (the bottom axis point less the radius), a ring of eight
    /// points of the radius at the step height and 0.1 above it, a ring
    /// at half the radius above the top axis point (round the axis, not
    /// offset by the top point's x and y: as the game builds it), and one
    /// point half the radius above that.
    pub fn character(bottom: Vec3, top: Vec3, radius: f32, step: f32) -> Hull {
        let k = 1.0 / HAVOK_UNIT;
        let r = k * radius;
        let low = [k * bottom[0], k * bottom[1], k * bottom[2] - r];
        let mut vertices = vec![low];
        let ring1 = low[2] + (step + 0.1);
        for d in RING {
            vertices.push([low[0] + r * d[0], low[1] + r * d[1], ring1]);
        }
        let ring2 = r * 0.5 + k * top[2];
        for d in RING {
            vertices.push([d[0] * r, r * d[1], ring2]);
        }
        vertices.push([k * top[0], top[1] * k, ring2 + r * 0.5]);
        Hull {
            vertices,
            radius: HULL_RADIUS,
            character: true,
        }
    }

    /// A triangle (no convex radius of its own: the shell is passed as
    /// the radius).
    pub fn triangle(corners: [Vec3; 3], radius: f32) -> Hull {
        Hull {
            vertices: corners.to_vec(),
            radius,
            character: false,
        }
    }

    fn support(&self, d: Vec3, at: Vec3) -> Vec3 {
        let mut best = self.vertices[0];
        let mut best_d = dot(best, d);
        for &v in &self.vertices[1..] {
            let p = dot(v, d);
            if p > best_d {
                best_d = p;
                best = v;
            }
        }
        add(best, at)
    }

    /// The lowest and highest of the points (with `at` added) along `d`.
    fn extent(&self, d: Vec3, at: Vec3) -> (f32, f32) {
        let mut lo = f32::MAX;
        let mut hi = f32::MIN;
        for &v in &self.vertices {
            let p = dot(add(v, at), d);
            lo = lo.min(p);
            hi = hi.max(p);
        }
        (lo, hi)
    }

    /// Box round the points at `at`, grown by the convex radius.
    fn bounds(&self, at: Vec3) -> (Vec3, Vec3) {
        let mut lo = [f32::MAX; 3];
        let mut hi = [f32::MIN; 3];
        for &v in &self.vertices {
            for k in 0..3 {
                lo[k] = lo[k].min(v[k] + at[k]);
                hi[k] = hi[k].max(v[k] + at[k]);
            }
        }
        let r = self.radius;
        (
            [lo[0] - r, lo[1] - r, lo[2] - r],
            [hi[0] + r, hi[1] + r, hi[2] + r],
        )
    }

    /// The face normals and edge directions to try as separating axes.
    fn axes(&self) -> (Vec<Vec3>, Vec<Vec3>) {
        let v = &self.vertices;
        if self.character {
            let mut faces = Vec::new();
            let mut edges = Vec::new();
            for i in 0..8 {
                let j = (i + 1) % 8;
                let (a1, b1) = (v[1 + i], v[1 + j]);
                let (a2, b2) = (v[9 + i], v[9 + j]);
                faces.push(cross(sub(a1, v[0]), sub(b1, v[0])));
                faces.push(cross(sub(b1, a1), sub(a2, a1)));
                faces.push(cross(sub(a2, v[17]), sub(b2, v[17])));
                edges.push(sub(a1, v[0]));
                edges.push(sub(b1, a1));
                edges.push(sub(a2, a1));
                edges.push(sub(b2, a2));
                edges.push(sub(v[17], a2));
            }
            (faces, edges)
        } else if v.len() == 3 {
            let n = cross(sub(v[1], v[0]), sub(v[2], v[0]));
            (
                vec![n],
                vec![sub(v[1], v[0]), sub(v[2], v[1]), sub(v[0], v[2])],
            )
        } else {
            (Vec::new(), Vec::new())
        }
    }
}

/// The closest points of two convex shapes' cores, or their separation
/// along the axis of least overlap when the cores overlap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Closest {
    /// Distance between the two surfaces (convex radii included); negative
    /// when they overlap.
    pub distance: f32,
    /// Unit normal from `b` toward `a`.
    pub normal: Vec3,
    /// A point on `b`'s surface.
    pub point: Vec3,
}

#[derive(Clone, Copy)]
struct SimplexPoint {
    w: Vec3,
    a: Vec3,
    b: Vec3,
}

/// Simplex points with their weights.
type Weighted = Vec<(SimplexPoint, f32)>;

/// Closest point to the origin on a simplex of 1 to 4 points: the point,
/// and the simplex reduced to the points it's made of with their weights.
fn closest_on_simplex(s: &[SimplexPoint]) -> (Vec3, Vec<(SimplexPoint, f32)>) {
    match s.len() {
        1 => (s[0].w, vec![(s[0], 1.0)]),
        2 => {
            let (a, b) = (s[0].w, s[1].w);
            let ab = sub(b, a);
            let t = -dot(a, ab);
            let den = dot(ab, ab);
            if t <= 0.0 || den <= 0.0 {
                return (a, vec![(s[0], 1.0)]);
            }
            if t >= den {
                return (b, vec![(s[1], 1.0)]);
            }
            let u = t / den;
            (add(a, scale(ab, u)), vec![(s[0], 1.0 - u), (s[1], u)])
        }
        3 => closest_on_triangle_simplex(s[0], s[1], s[2]),
        _ => {
            // A tetrahedron: the origin inside it, or the closest of its
            // faces that face the origin.
            let pts = [s[0], s[1], s[2], s[3]];
            let faces = [(0, 1, 2, 3), (0, 1, 3, 2), (0, 2, 3, 1), (1, 2, 3, 0)];
            let mut best: Option<(f32, Vec3, Weighted)> = None;
            let mut inside = true;
            for (i, j, k, o) in faces {
                let (a, b, c, d) = (pts[i].w, pts[j].w, pts[k].w, pts[o].w);
                let n = cross(sub(b, a), sub(c, a));
                let side_o = dot(scale(a, -1.0), n);
                let side_d = dot(sub(d, a), n);
                if side_o * side_d < 0.0 {
                    inside = false;
                    let (p, w) = closest_on_triangle_simplex(pts[i], pts[j], pts[k]);
                    let dd = dot(p, p);
                    if best.as_ref().map_or(true, |b| dd < b.0) {
                        best = Some((dd, p, w));
                    }
                }
            }
            if inside {
                return ([0.0; 3], pts.iter().map(|&p| (p, 0.25)).collect());
            }
            let (_, p, w) = best.expect("a face faces the origin");
            (p, w)
        }
    }
}

/// Closest point to the origin on a triangle of simplex points, with the
/// weights (Ericson, Real-Time Collision Detection, 5.1.5).
fn closest_on_triangle_simplex(
    pa: SimplexPoint,
    pb: SimplexPoint,
    pc: SimplexPoint,
) -> (Vec3, Vec<(SimplexPoint, f32)>) {
    let (a, b, c) = (pa.w, pb.w, pc.w);
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ap = scale(a, -1.0);
    let d1 = dot(ab, ap);
    let d2 = dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return (a, vec![(pa, 1.0)]);
    }
    let bp = scale(b, -1.0);
    let d3 = dot(ab, bp);
    let d4 = dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return (b, vec![(pb, 1.0)]);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (add(a, scale(ab, v)), vec![(pa, 1.0 - v), (pb, v)]);
    }
    let cp = scale(c, -1.0);
    let d5 = dot(ab, cp);
    let d6 = dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return (c, vec![(pc, 1.0)]);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (add(a, scale(ac, w)), vec![(pa, 1.0 - w), (pc, w)]);
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (add(b, scale(sub(c, b), w)), vec![(pb, 1.0 - w), (pc, w)]);
    }
    let den = va + vb + vc;
    if den.abs() <= f32::MIN_POSITIVE {
        return (a, vec![(pa, 1.0)]);
    }
    let denom = 1.0 / den;
    let v = vb * denom;
    let w = vc * denom;
    (
        add(a, add(scale(ab, v), scale(ac, w))),
        vec![(pa, 1.0 - v - w), (pb, v), (pc, w)],
    )
}

/// The closest points of `a` at `at_a` and `b` at `at_b` (their cores, no
/// convex radius), by GJK; `None` when the cores overlap.
fn gjk(a: &Hull, at_a: Vec3, b: &Hull, at_b: Vec3) -> Option<(Vec3, Vec3)> {
    let first = |d: Vec3| {
        let sa = a.support(d, at_a);
        let sb = b.support(scale(d, -1.0), at_b);
        SimplexPoint {
            w: sub(sa, sb),
            a: sa,
            b: sb,
        }
    };
    let mut d = sub(at_a, at_b);
    if dot(d, d) < 1e-12 {
        d = [1.0, 0.0, 0.0];
    }
    let mut simplex = vec![first(scale(d, -1.0))];
    let mut v = simplex[0].w;
    for _ in 0..64 {
        let vv = dot(v, v);
        if vv < 1e-12 {
            return None;
        }
        let w = first(scale(v, -1.0));
        // No point further toward the origin than the closest so far.
        if vv - dot(v, w.w) <= 1e-6 * vv.max(1e-6)
            || simplex.iter().any(|p| dist2(p.w, w.w) < 1e-14)
        {
            break;
        }
        simplex.push(w);
        let (p, kept) = closest_on_simplex(&simplex);
        if kept.len() == 4 {
            return None;
        }
        simplex = kept.iter().map(|&(s, _)| s).collect();
        if dot(p, p) >= vv {
            // No progress: rounding.
            break;
        }
        v = p;
    }
    // The closest points from the weights of the final simplex.
    let (_, kept) = closest_on_simplex(&simplex);
    let mut pa = [0.0; 3];
    let mut pb = [0.0; 3];
    for (s, wgt) in kept {
        pa = add(pa, scale(s.a, wgt));
        pb = add(pb, scale(s.b, wgt));
    }
    if dist2(pa, pb) < 1e-12 {
        return None;
    }
    Some((pa, pb))
}

/// The points of `h` (at `at`) furthest along `d`, within 0.02 (GJK's
/// normal is only so exact): the feature (vertex, edge or face) a contact
/// along `d` is made with.
fn feature(h: &Hull, at: Vec3, d: Vec3) -> Vec<Vec3> {
    let best = h
        .vertices
        .iter()
        .map(|&v| dot(v, d))
        .fold(f32::MIN, f32::max);
    h.vertices
        .iter()
        .filter(|&&v| dot(v, d) >= best - 0.02)
        .map(|&v| add(v, at))
        .collect()
}

fn middle(points: &[Vec3]) -> Vec3 {
    let sum = points.iter().fold([0.0; 3], |s, &p| add(s, p));
    scale(sum, 1.0 / points.len() as f32)
}

/// Where on `b`'s core a contact with `a` along `normal` (from `b` toward
/// `a`) is. GJK's closest points aren't unique when faces or edges lie
/// parallel (a hull's side against a wall), and its weights can then give
/// a point outside the touching region; instead, for a triangle, the point
/// of it nearest `a`'s touching feature (its vertex, its edge, or its
/// face's middle); for another hull the middle of its own touching
/// feature. Havok's choice of point isn't traced.
fn contact_on(b: &Hull, at_b: Vec3, a: &Hull, at_a: Vec3, normal: Vec3, gjk_point: Vec3) -> Vec3 {
    if b.vertices.len() == 3 {
        let f = feature(a, at_a, scale(normal, -1.0));
        let [p, q, r] = [0, 1, 2].map(|i| add(b.vertices[i], at_b));
        let on = match f.len() {
            1 => crate::closest_on_triangle(f[0], p, q, r),
            2 => crate::segment_triangle_closest(f[0], f[1], p, q, r).1,
            _ => crate::closest_on_triangle(middle(&f), p, q, r),
        };
        if on.iter().all(|v| v.is_finite()) {
            return on;
        }
        return gjk_point;
    }
    middle(&feature(b, at_b, normal))
}
/// Closest points (or least-overlap separation) of `a` at `at_a` and `b` at
/// `at_b`, radii included.
pub fn closest(a: &Hull, at_a: Vec3, b: &Hull, at_b: Vec3) -> Closest {
    if let Some((pa, pb)) = gjk(a, at_a, b, at_b) {
        let gap = sub(pa, pb);
        let d = length(gap);
        let normal = scale(gap, 1.0 / d);
        return Closest {
            distance: d - a.radius - b.radius,
            normal,
            point: add(
                contact_on(b, at_b, a, at_a, normal, pb),
                scale(normal, b.radius),
            ),
        };
    }
    // The cores overlap: the axis of least overlap among both shapes'
    // face normals and their edges' cross products.
    let (fa, ea) = a.axes();
    let (fb, eb) = b.axes();
    let mut axes: Vec<Vec3> = fa.into_iter().chain(fb).collect();
    for &x in &ea {
        for &y in &eb {
            axes.push(cross(x, y));
        }
    }
    let mut best: Option<(f32, Vec3)> = None;
    for n in axes {
        let l = length(n);
        if l < 1e-6 {
            continue;
        }
        let n = scale(n, 1.0 / l);
        let (alo, ahi) = a.extent(n, at_a);
        let (blo, bhi) = b.extent(n, at_b);
        // Moving `a` along +n by (bhi - alo), or along -n by (ahi - blo).
        for (overlap, dir) in [(bhi - alo, n), (ahi - blo, scale(n, -1.0))] {
            if best.map_or(true, |(o, _)| overlap < o) {
                best = Some((overlap, dir));
            }
        }
    }
    let (overlap, normal) = best.unwrap_or((0.0, [0.0, 0.0, 1.0]));
    let deepest = add(a.support(scale(normal, -1.0), at_a), scale(normal, overlap));
    Closest {
        distance: -overlap - a.radius - b.radius,
        normal,
        point: add(
            contact_on(b, at_b, a, at_a, normal, deepest),
            scale(normal, b.radius),
        ),
    }
}

/// What a contact point is with: a triangle of the walking collider, or
/// another character (`hkpRootCdPoint::m_rootCollidableB`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Body {
    /// A collider triangle (its index).
    Triangle(u32),
    /// Another character (index into [`Collider::people`]).
    Person(u32),
}

/// A contact point between the character and something
/// (`hkpRootCdPoint`, Xbox PDB, 0x70 bytes): position on the other's
/// surface (`+0x00`; its w holds a cast's hit fraction once the proxy has
/// converted it, `00cac890`), the separating normal toward the character
/// (`+0x10`) with the distance in its w, and the other body (`+0x48`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootCdPoint {
    pub position: Vec3,
    pub fraction: f32,
    pub normal: Vec3,
    pub distance: f32,
    pub body: Body,
}

/// What the character's collision is: its hull, where its centre is (game
/// units), and the people it runs into.
pub struct Query<'a> {
    pub collider: &'a Collider,
    pub hull: &'a Hull,
    /// Origin of the Havok-unit frame the queries work in (game units).
    pub origin: Vec3,
}

impl Query<'_> {
    /// A game-unit point in the query's Havok frame.
    pub fn to_local(&self, p: Vec3) -> Vec3 {
        scale(sub(p, self.origin), 1.0 / HAVOK_UNIT)
    }

    /// The things near the hull swept from `from` by `displacement` (Havok
    /// frame), grown by `reach`, and whether each blocks the character
    /// (the controller's layer, 30, against the triangle's).
    fn candidates(&self, from: Vec3, displacement: Vec3, reach: f32) -> Vec<Body> {
        let (lo0, hi0) = self.hull.bounds(from);
        let to = add(from, displacement);
        let (lo1, hi1) = self.hull.bounds(to);
        let g = |p: Vec3| add(scale(p, HAVOK_UNIT), self.origin);
        let pad = reach * HAVOK_UNIT + 4.0;
        let lo = g([lo0[0].min(lo1[0]), lo0[1].min(lo1[1]), lo0[2].min(lo1[2])]);
        let hi = g([hi0[0].max(hi1[0]), hi0[1].max(hi1[1]), hi0[2].max(hi1[2])]);
        let lo = [lo[0] - pad, lo[1] - pad, lo[2] - pad];
        let hi = [hi[0] + pad, hi[1] + pad, hi[2] + pad];
        let filter = layers::Filter::shared();
        let mut out: Vec<Body> = self
            .collider
            .near(lo, hi)
            .into_iter()
            .filter(|&t| {
                let [a, b, c] = self.collider.triangle(t);
                let s = self.collider.shell(t);
                let below = a[2].max(b[2]).max(c[2]) + s < lo[2];
                let above = a[2].min(b[2]).min(c[2]) - s > hi[2];
                let layer = self.collider.layer(t);
                let blocks = layer == ANY_LAYER
                    || filter.layers_touch(layers::layer::CHAR_CONTROLLER, layer);
                !below && !above && blocks
            })
            .map(Body::Triangle)
            .collect();
        for (i, p) in self.collider.people().iter().enumerate() {
            let top = p.feet[2] + p.height;
            let r = p.radius;
            if p.feet[0] + r < lo[0]
                || p.feet[0] - r > hi[0]
                || p.feet[1] + r < lo[1]
                || p.feet[1] - r > hi[1]
                || top < lo[2]
                || p.feet[2] > hi[2]
            {
                continue;
            }
            out.push(Body::Person(i as u32));
        }
        out
    }

    /// The shape of a body, and where its points are measured from, in the
    /// query's frame.
    fn body_shape(&self, body: Body) -> (Hull, Vec3) {
        match body {
            Body::Triangle(t) => {
                let corners = self.collider.triangle(t).map(|c| self.to_local(c));
                let shell = self.collider.shell(t) / HAVOK_UNIT;
                (Hull::triangle(corners, shell), [0.0; 3])
            }
            Body::Person(i) => {
                let p = self.collider.people()[i as usize];
                let (hull, centre) = person_hull(p.radius, p.height);
                (hull, self.to_local(add(p.feet, [0.0, 0.0, centre])))
            }
        }
    }

    fn closest_to(&self, at: Vec3, body: Body) -> Closest {
        let (shape, shape_at) = self.body_shape(body);
        closest(self.hull, at, &shape, shape_at)
    }

    /// The points of everything within `tolerance` of the hull at `at`
    /// (Havok frame): the start collector of a cast (an
    /// `hkpAllCdPointCollector`, one point per triangle or body).
    pub fn start_points(&self, at: Vec3, tolerance: f32) -> Vec<RootCdPoint> {
        let mut out = Vec::new();
        for body in self.candidates(at, [0.0; 3], tolerance) {
            let c = self.closest_to(at, body);
            if c.distance < tolerance {
                out.push(RootCdPoint {
                    position: c.point,
                    fraction: 0.0,
                    normal: c.normal,
                    distance: c.distance,
                    body,
                });
            }
        }
        out
    }

    /// A linear cast of the hull from `from` by `displacement` (Havok
    /// frame): every body it meets on the way, with the hit's fraction of
    /// the way in `distance` (as a cast collector holds it before the proxy
    /// converts it, `00cac890`). Per body this is the game's GSK agent
    /// cast ([`gsk_linear_cast`]); `tolerance` is the phantom's start point
    /// tolerance (the cast input's `m_tolerance`).
    pub fn linear_cast(
        &self,
        from: Vec3,
        displacement: Vec3,
        max_extra_penetration: f32,
        tolerance: f32,
    ) -> Vec<RootCdPoint> {
        let mut out = Vec::new();
        let len2 = dot(displacement, displacement);
        if len2 <= 0.0 {
            return out;
        }
        // The agent's closest points are asked within the start tolerance
        // plus the path's length (`00daf8e0`: input +0x08 and +0x64).
        let reach = tolerance + len2.sqrt();
        for body in self.candidates(from, displacement, CAST_EARLY_OUT) {
            let hit = gsk_linear_cast(
                |at| {
                    let c = self.closest_to(at, body);
                    (c.distance < reach).then_some(c)
                },
                from,
                displacement,
                max_extra_penetration,
            );
            if let Some((t, c)) = hit {
                out.push(RootCdPoint {
                    position: c.point,
                    fraction: 0.0,
                    normal: c.normal,
                    distance: t,
                    body,
                });
            }
        }
        out
    }
}

/// The cast of one convex shape against another by the game's GSK agent
/// (`hkpGskBaseAgent::staticLinearCast`, `00daf8e0`), a conservative
/// advancement on its closest points query. `closest_at(position)` is the
/// agent's closest points (`hkpGskBaseAgent::staticGetClosestPoints`,
/// `00dac450`) for the moved shape at `position`, `None` when nothing is
/// within the input's tolerance (the start point tolerance plus the path's
/// length). The hit is the points of the last query, with the fraction it
/// was made at.
///
/// The cast only counts if, with the first points' normal held, the path
/// reaches the surface (`distance + path . normal <= 0`) and goes into it
/// by more than `max_extra_penetration`. A first distance within
/// [`CAST_EARLY_OUT`] (penetrating included) is a hit at fraction 0 (the
/// collector's own early-out distance is the all-points collector's
/// maximum). Otherwise the fraction starts at `distance / -(path .
/// normal)` and each query (at most [`CAST_MAX_ITERATIONS`]) rejects the
/// cast (nothing near, moving away, or the rest of the path too short to
/// close the distance), accepts it (within the early-out) or advances the
/// fraction by `distance / -(path . normal)`; running out of iterations
/// accepts the last query.
pub fn gsk_linear_cast(
    closest_at: impl Fn(Vec3) -> Option<Closest>,
    from: Vec3,
    path: Vec3,
    max_extra_penetration: f32,
) -> Option<(f32, Closest)> {
    // Translated from 00daf8e0 (decompiled, FalloutNV.exe 1.4.0.525).
    let first = closest_at(from)?;
    let d = first.distance;
    let into = dot(path, first.normal);
    if !(d + into <= 0.0 && max_extra_penetration + into < 0.0) {
        return None;
    }
    if d <= CAST_EARLY_OUT {
        return Some((0.0, first));
    }
    let mut fraction = d / -into;
    let mut hit = None;
    for _ in 0..CAST_MAX_ITERATIONS {
        let c = closest_at(add(from, scale(path, fraction)))?;
        let into = dot(path, c.normal);
        if into >= 0.0 {
            return None;
        }
        let into = -into;
        if into < c.distance + into * fraction {
            return None;
        }
        if c.distance <= CAST_EARLY_OUT {
            return Some((fraction, c));
        }
        hit = Some((fraction, c));
        fraction += c.distance / into;
        if fraction > COLLECTOR_EARLY_OUT {
            return None;
        }
    }
    hit
}

/// Another character's hull, and its centre's height above their feet
/// (game units): the same build as the controller's own for people
/// ([`crate::CharacterShape::hull`] with that radius and height).
pub fn person_hull(radius: f32, height: f32) -> (Hull, f32) {
    let shape = crate::CharacterShape {
        radius,
        height,
        ..crate::CharacterShape::PLAYER
    };
    (shape.hull(), shape.centre())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cube(half: f32) -> Hull {
        let mut vertices = Vec::new();
        for x in [-half, half] {
            for y in [-half, half] {
                for z in [-half, half] {
                    vertices.push([x, y, z]);
                }
            }
        }
        Hull {
            vertices,
            radius: 0.0,
            character: false,
        }
    }

    #[test]
    fn the_character_hull_is_the_games_eighteen_points() {
        // People: half extents 23 x 17.5 x 64, radius 20.25, axis points at
        // +-43.75, step 31 game units (`00c6da50`: 31 x 0.142875).
        let h = Hull::character(
            [0.0, 0.0, -43.75],
            [0.0, 0.0, 43.75],
            20.25,
            31.0 / HAVOK_UNIT,
        );
        let k = 1.0 / HAVOK_UNIT;
        assert_eq!(h.vertices.len(), 18);
        assert!((h.vertices[0][2] + 64.0 * k).abs() < 1e-5);
        // The first ring 31.7 game units above the bottom point.
        let ring1 = h.vertices[1][2] - h.vertices[0][2];
        assert!(
            (ring1 * HAVOK_UNIT - 31.7).abs() < 0.01,
            "{}",
            ring1 * HAVOK_UNIT
        );
        assert!((h.vertices[1][0] - 20.25 * k).abs() < 1e-5);
        assert!((h.vertices[9][2] - 53.875 * k).abs() < 1e-5);
        assert!((h.vertices[17][2] - 64.0 * k).abs() < 1e-5);
    }

    #[test]
    fn closest_points_of_separate_and_overlapping_shapes() {
        let a = cube(1.0);
        let tri = Hull::triangle([[-5.0, -5.0, 0.0], [5.0, -5.0, 0.0], [0.0, 5.0, 0.0]], 0.0);
        let c = closest(&a, [0.0, 0.0, 3.0], &tri, [0.0; 3]);
        assert!((c.distance - 2.0).abs() < 1e-4, "{c:?}");
        assert!((c.normal[2] - 1.0).abs() < 1e-4);
        // Sunk 0.5 into the triangle: pushed back up by 0.5.
        let c = closest(&a, [0.0, 0.0, 0.5], &tri, [0.0; 3]);
        assert!((c.distance + 0.5).abs() < 1e-4, "{c:?}");
        assert!((c.normal[2] - 1.0).abs() < 1e-4, "{c:?}");
        // From below: pushed down.
        let c = closest(&a, [0.0, 0.0, -0.5], &tri, [0.0; 3]);
        assert!((c.normal[2] + 1.0).abs() < 1e-4, "{c:?}");
    }

    /// A cube of half extent 1 (core 0.9, radius 0.1) over the plane z = 0, as
    /// the cast's closest points query (a flat triangle, no radius), moved by `at`.
    fn floor_query(at: Vec3) -> Option<Closest> {
        let a = Hull {
            radius: 0.1,
            ..cube(0.9)
        };
        let tri = Hull::triangle(
            [[-50.0, -50.0, 0.0], [50.0, -50.0, 0.0], [0.0, 50.0, 0.0]],
            0.0,
        );
        Some(closest(&a, at, &tri, [0.0; 3]))
    }

    #[test]
    fn a_cast_that_cannot_reach_the_surface_is_no_hit() {
        // Gap 1 (the cube's bottom at z = 1); the path goes down 0.5 only.
        assert!(gsk_linear_cast(floor_query, [0.0, 0.0, 2.0], [0.0, 0.0, -0.5], 0.01).is_none());
        // Moving away, or along the surface.
        assert!(gsk_linear_cast(floor_query, [0.0, 0.0, 2.0], [0.0, 0.0, 3.0], 0.01).is_none());
        assert!(gsk_linear_cast(floor_query, [0.0, 0.0, 2.0], [5.0, 0.0, 0.0], 0.01).is_none());
    }

    #[test]
    fn a_cast_hits_where_the_surface_is_reached() {
        let (t, c) = gsk_linear_cast(floor_query, [0.0, 0.0, 3.0], [0.0, 0.0, -4.0], 0.01).unwrap();
        // The gap is 2 and the path 4 long: half way, within the early-out.
        assert!((t - 0.5).abs() <= CAST_EARLY_OUT / 4.0 + 1e-4, "{t}");
        assert!(c.distance <= CAST_EARLY_OUT, "{c:?}");
        assert!((c.normal[2] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn a_cast_started_within_the_early_out_hits_at_fraction_zero() {
        // 0.005 above the surface, going down 1: a hit where it starts.
        let (t, _) =
            gsk_linear_cast(floor_query, [0.0, 0.0, 1.005], [0.0, 0.0, -1.0], 0.01).unwrap();
        assert_eq!(t, 0.0);
        // Already sunk 0.5: a hit at fraction 0 as well.
        let (t, c) = gsk_linear_cast(floor_query, [0.0, 0.0, 0.5], [0.0, 0.0, -1.0], 0.01).unwrap();
        assert_eq!(t, 0.0);
        assert!(c.distance < 0.0);
    }

    #[test]
    fn a_move_into_the_surface_by_no_more_than_the_extra_penetration_is_no_hit() {
        // 0.005 above the surface moving down 0.008: it neither reaches
        // far (0.008 < 0.01 extra) nor is that counted as a hit.
        assert!(
            gsk_linear_cast(floor_query, [0.0, 0.0, 1.005], [0.0, 0.0, -0.008], 0.01).is_none()
        );
        // The same move with no extra penetration allowed is a hit.
        assert!(gsk_linear_cast(floor_query, [0.0, 0.0, 1.005], [0.0, 0.0, -0.008], 0.0).is_some());
    }

    #[test]
    fn a_cast_stops_asking_after_twenty_queries() {
        use std::cell::Cell;
        let queries = Cell::new(0);
        // A surface that is always 1 away along the path but never closer
        // by the path's direction (a normal tilted so each step gains
        // little): the iterations are bounded.
        let n = [0.1f32, 0.0, 0.995_f32];
        let hit = gsk_linear_cast(
            |_| {
                queries.set(queries.get() + 1);
                Some(Closest {
                    distance: 0.5,
                    normal: n,
                    point: [0.0; 3],
                })
            },
            [0.0; 3],
            [0.0, 0.0, -10.0],
            0.01,
        );
        // The first query plus at most twenty iterations.
        assert!(queries.get() <= 21, "{}", queries.get());
        assert!(hit.is_none() || queries.get() == 21);
    }
    #[test]
    fn closest_point_on_a_triangles_edge() {
        let a = cube(0.5);
        let tri = Hull::triangle([[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [0.0, 4.0, 0.0]], 0.25);
        let c = closest(&a, [-2.0, 1.0, 0.0], &tri, [0.0; 3]);
        assert!((c.distance - (1.5 - 0.25)).abs() < 1e-4, "{c:?}");
        assert!((c.normal[0] + 1.0).abs() < 1e-4, "{c:?}");
    }
}
