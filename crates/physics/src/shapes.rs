//! Havok's solid shapes as triangles, for a [`crate::Collider`].

use crate::vec::*;
use crate::Vec3;

/// A convex hull's faces as triangles, from its corners and its face planes
/// (`n·p + d = 0`, `n` pointing out; Havok stores both). Each plane's
/// corners (those lying on it) are put in order around the face and fanned
/// into triangles, wound counter-clockwise seen from outside.
pub fn hull(vertices: &[Vec3], planes: &[[f32; 4]]) -> Vec<[u32; 3]> {
    let size = vertices
        .iter()
        .flat_map(|v| v.iter())
        .fold(0.0f32, |m, &c| m.max(c.abs()))
        .max(1.0);
    let tolerance = 1e-3 * size + 0.01;
    let mut out = Vec::new();
    for p in planes {
        let n = [p[0], p[1], p[2]];
        if length(n) < 1e-6 {
            continue;
        }
        let n = normalize(n);
        let on: Vec<u32> = (0..vertices.len() as u32)
            .filter(|&i| {
                let v = vertices[i as usize];
                (dot([p[0], p[1], p[2]], v) + p[3]).abs() / length([p[0], p[1], p[2]]) < tolerance
            })
            .collect();
        if on.len() < 3 {
            continue;
        }
        let center = scale(
            on.iter()
                .fold([0.0; 3], |s, &i| add(s, vertices[i as usize])),
            1.0 / on.len() as f32,
        );
        // Two axes in the face to measure angles around the center.
        let u = normalize(sub(vertices[on[0] as usize], center));
        let w = cross(n, u);
        let mut ordered: Vec<(f32, u32)> = on
            .iter()
            .map(|&i| {
                let d = sub(vertices[i as usize], center);
                (dot(d, w).atan2(dot(d, u)), i)
            })
            .collect();
        ordered.sort_by(|a, b| a.0.total_cmp(&b.0));
        for k in 1..ordered.len() - 1 {
            out.push([ordered[0].1, ordered[k].1, ordered[k + 1].1]);
        }
    }
    out
}

/// A sphere as a ring-and-segment mesh: vertices and triangles.
pub fn sphere(center: Vec3, radius: f32) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    capsule(center, center, radius)
}

/// A capsule (two end centers and a radius) as a mesh: a cylinder between
/// two half spheres.
pub fn capsule(a: Vec3, b: Vec3, radius: f32) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    const AROUND: u32 = 12;
    const HALF_RINGS: u32 = 4;
    let axis = sub(b, a);
    let up = if length(axis) > 1e-6 {
        normalize(axis)
    } else {
        [0.0, 0.0, 1.0]
    };
    let side = if up[2].abs() < 0.9 {
        normalize(cross(up, [0.0, 0.0, 1.0]))
    } else {
        normalize(cross(up, [1.0, 0.0, 0.0]))
    };
    let other = cross(up, side);
    let mut vertices = Vec::new();
    // Rings from the bottom pole to the top pole: the lower half sphere
    // around `a`, then the upper around `b`.
    let rings = 2 * HALF_RINGS + 2;
    for ring in 0..rings {
        let (end, lat) = if ring <= HALF_RINGS {
            (
                a,
                -std::f32::consts::FRAC_PI_2
                    + ring as f32 / HALF_RINGS as f32 * std::f32::consts::FRAC_PI_2,
            )
        } else {
            let k = ring - HALF_RINGS - 1;
            (
                b,
                k as f32 / HALF_RINGS as f32 * std::f32::consts::FRAC_PI_2,
            )
        };
        for step in 0..AROUND {
            let lon = step as f32 / AROUND as f32 * std::f32::consts::TAU;
            let dir = add(
                scale(up, lat.sin()),
                scale(
                    add(scale(side, lon.cos()), scale(other, lon.sin())),
                    lat.cos(),
                ),
            );
            vertices.push(add(end, scale(dir, radius)));
        }
    }
    let mut triangles = Vec::new();
    for ring in 0..rings - 1 {
        for step in 0..AROUND {
            let next = (step + 1) % AROUND;
            let i0 = ring * AROUND + step;
            let i1 = ring * AROUND + next;
            let i2 = (ring + 1) * AROUND + step;
            let i3 = (ring + 1) * AROUND + next;
            triangles.push([i0, i1, i3]);
            triangles.push([i0, i3, i2]);
        }
    }
    (vertices, triangles)
}

/// How far along a ray (`direction` of length 1) it first meets a capsule
/// (the segment `a`–`b` grown by `radius`, a sphere when they're the same
/// point): 0 when it starts inside, `None` when it misses or the capsule is
/// behind it.
pub fn ray_capsule(origin: Vec3, direction: Vec3, a: Vec3, b: Vec3, radius: f32) -> Option<f32> {
    let r2 = radius * radius;
    let axis = sub(b, a);
    let len2 = dot(axis, axis);
    // Inside: the start is within the radius of the segment.
    let along = if len2 > 1e-12 {
        (dot(sub(origin, a), axis) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    if dist2(origin, add(a, scale(axis, along))) <= r2 {
        return Some(0.0);
    }
    let mut best: Option<f32> = None;
    let mut keep = |t: f32| {
        if t >= 0.0 && best.map_or(true, |b| t < b) {
            best = Some(t);
        }
    };
    // The side: points whose distance from the axis line is the radius,
    // between the ends.
    if len2 > 1e-12 {
        let oa = sub(origin, a);
        let da = dot(direction, axis);
        let oaa = dot(oa, axis);
        let qa = len2 - da * da;
        let qb = len2 * dot(direction, oa) - oaa * da;
        let qc = len2 * dot(oa, oa) - oaa * oaa - r2 * len2;
        let h = qb * qb - qa * qc;
        if qa > 1e-12 && h >= 0.0 {
            let t = (-qb - h.sqrt()) / qa;
            let y = oaa + t * da;
            if (0.0..=len2).contains(&y) {
                keep(t);
            }
        }
    }
    // The two round ends.
    for end in [a, b] {
        let oc = sub(origin, end);
        let qb = dot(direction, oc);
        let h = qb * qb - (dot(oc, oc) - r2);
        if h >= 0.0 {
            keep(-qb - h.sqrt());
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rays_meet_capsules_on_their_sides_and_ends() {
        // Upright, from z 0 to 10, radius 2, at the origin.
        let (a, b) = ([0.0, 0.0, 0.0], [0.0, 0.0, 10.0]);
        // From the side at mid height: the surface 2 short of the axis.
        let t = ray_capsule([-10.0, 0.0, 5.0], [1.0, 0.0, 0.0], a, b, 2.0).unwrap();
        assert!((t - 8.0).abs() < 1e-4, "{t}");
        // Straight down onto the top: the round end, 2 above b.
        let t = ray_capsule([0.0, 0.0, 20.0], [0.0, 0.0, -1.0], a, b, 2.0).unwrap();
        assert!((t - 8.0).abs() < 1e-4, "{t}");
        // Past the side, and behind.
        assert!(ray_capsule([-10.0, 3.0, 5.0], [1.0, 0.0, 0.0], a, b, 2.0).is_none());
        assert!(ray_capsule([10.0, 0.0, 5.0], [1.0, 0.0, 0.0], a, b, 2.0).is_none());
        // Over the top of the side, but into the round end: it bulges out
        // 1.73 at a height of 11.
        let t = ray_capsule([-10.0, 0.0, 11.0], [1.0, 0.0, 0.0], a, b, 2.0).unwrap();
        assert!((t - (10.0 - 3.0f32.sqrt())).abs() < 1e-4, "{t}");
        // Inside, and a sphere.
        assert_eq!(
            ray_capsule([0.0, 0.5, 5.0], [1.0, 0.0, 0.0], a, b, 2.0),
            Some(0.0)
        );
        let t = ray_capsule([0.0, -5.0, 0.0], [0.0, 1.0, 0.0], a, a, 1.0).unwrap();
        assert!((t - 4.0).abs() < 1e-4);
    }

    #[test]
    fn triangulates_a_box_hull() {
        let h = [1.0f32, 2.0, 3.0];
        let corners: Vec<Vec3> = (0..8)
            .map(|k| {
                [
                    if k & 1 == 0 { -h[0] } else { h[0] },
                    if k & 2 == 0 { -h[1] } else { h[1] },
                    if k & 4 == 0 { -h[2] } else { h[2] },
                ]
            })
            .collect();
        let planes = [
            [1.0, 0.0, 0.0, -h[0]],
            [-1.0, 0.0, 0.0, -h[0]],
            [0.0, 1.0, 0.0, -h[1]],
            [0.0, -1.0, 0.0, -h[1]],
            [0.0, 0.0, 1.0, -h[2]],
            [0.0, 0.0, -1.0, -h[2]],
        ];
        let tris = hull(&corners, &planes);
        assert_eq!(tris.len(), 12);
        // Each triangle faces out: its normal points away from the center.
        for t in &tris {
            let [a, b, c] = t.map(|i| corners[i as usize]);
            let n = cross(sub(b, a), sub(c, a));
            let center = scale(add(add(a, b), c), 1.0 / 3.0);
            assert!(dot(n, center) > 0.0, "{t:?}");
        }
    }

    #[test]
    fn capsules_reach_their_radius() {
        let (v, t) = capsule([0.0, 0.0, 0.0], [0.0, 0.0, 10.0], 2.0);
        assert!(!t.is_empty());
        let top = v.iter().map(|p| p[2]).fold(f32::MIN, f32::max);
        let bottom = v.iter().map(|p| p[2]).fold(f32::MAX, f32::min);
        assert!((top - 12.0).abs() < 1e-4 && (bottom + 2.0).abs() < 1e-4);
        assert!(t.iter().flatten().all(|&i| (i as usize) < v.len()));
    }
}
