//! Ends of a path off the navmesh, as the game's path builder handles them
//! (`PathBuilder::BuildPathToNavMeshSearchStart` and
//! `BuildPathFromNavMeshSearchGoal` (Xbox PDB), `006caa40` and `006cac90`).
//!
//! A path location is on the navmesh when its navmesh has a triangle for
//! it (`PathingLocation::ResolveToClosestNavmeshAndTriangle` (Xbox PDB),
//! `006dd6f0` → `NavMesh::FindTriangleForLocation` (Xbox PDB), `00696a50`):
//! a triangle that holds the point seen from above and isn't flagged 0x20,
//! whose height there is at most 64 above the point and at most 180 below
//! it (no lower limit for a point less than 50 above the land); of those,
//! the one nearest in height (one within 40 is taken at once). A location
//! that resolves to no triangle keeps only its navmesh (`006ddc00`).
//!
//! Such an end is joined to the navmesh by ray casts
//! (`bUseRayCasts` 1): the navmesh's open edges within
//! `fFindClosestEdgesRadius` (512) of the point give candidate spots
//! (`006d4120`: the point of the edge nearest it, moved a tenth of the way
//! toward its triangle's middle), those whose height lies between 64 below
//! and 180 above the point first, then by distance, the nearest 10 kept.
//! The way to each candidate in turn is walked by ray casts
//! (`PathingTaskData::GetRayCastPathToNavmesh` (Xbox PDB), `006e6320` →
//! `006e6e40`): steps of the walker's radius (at least 32) along the line,
//! each a ray 32 above the ground to the next step (blocked: no way) and a
//! ray down from there by `fJumpFallHeightMin` + 32 to find the ground (none:
//! no way); the last step to the spot itself, whose height must come within
//! 64 of the ground found. The first candidate with a way is taken: the
//! path runs from the point to it, and the navmesh search starts there. A
//! goal off the navmesh takes, before any ray cast, a candidate within the
//! request's target radius whose height is in the window (`006cac90`);
//! else the way is walked from the spot to the goal.
//!
//! Not followed (labelled): which layers the `PATHPICK` ray meets (the
//! world collision here is the character's, [`super::PathPick`]); the
//! picks are done where the request is made, as the game hands them to the
//! main thread (`PathManager::GetRayCastPathToNavMesh` (Xbox PDB),
//! `006ebbf0`); `bUseRayCasts` 0's other way (`006cdb40`) isn't followed.

use super::{closest_in_triangle, height_in, NavMesh};

/// `fFindClosestEdgesRadius` (`[Pathing]`, exe default 512, `010231a0`).
pub const FIND_CLOSEST_EDGES_RADIUS: f32 = 512.0;
/// The most candidate edges tried (`006caa40` asks `006d4120` for 10).
const CANDIDATES: usize = 10;
/// A candidate's height window, from the point (`006d4020`, `006cac90`:
/// `0106b9f0` −64 and `0104ed58` 180).
const WINDOW_BELOW: f32 = -64.0;
const WINDOW_ABOVE: f32 = 180.0;
/// `00696a50`: a triangle at most 64 above the point (`010240c0`) and
/// 180 below it (`0106b160`), unless the point is within 50 of the land
/// (`0101e2c0`); one within 40 (`01035810`) is taken at once.
const TRIANGLE_ABOVE: f32 = 64.0;
const TRIANGLE_BELOW: f32 = 180.0;
const NEAR_LAND: f32 = 50.0;
const CLOSE_ENOUGH: f32 = 40.0;
/// Triangle flag 0x20: such triangles hold no location (`00696a50`,
/// `00691140(0x20)`; what sets it isn't traced).
pub const NO_LOCATION: u32 = 0x20;
/// The ray-cast way's step (at least, `0101e340`) and how high above the
/// ground its rays run (`006e61a0`: +0x90 = 32).
const STEP_MIN: f32 = 32.0;
const LIFT: f32 = 32.0;
/// How near the ground found must come to the spot's height (`006e6e40`:
/// `010240c0`, 64).
const END_TOLERANCE: f32 = 64.0;

/// A spot on an open edge of the navmesh near a point (`006d4120`'s
/// `NavMeshEdgeLocation` (Xbox PDB)).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeSpot {
    pub triangle: usize,
    pub edge: usize,
    /// The edge's nearest point, a tenth of the way to the triangle's
    /// middle.
    pub point: [f32; 3],
    /// Squared distance from the point to the edge's nearest point.
    pub distance_sq: f32,
    /// The point's height above the edge's nearest point.
    pub dz: f32,
}

/// One end of a path on the navmesh: its triangle and point, and the
/// ray-cast way between it and the location (empty when the location is on
/// the navmesh).
#[derive(Debug, Clone, PartialEq)]
pub struct End {
    pub triangle: usize,
    pub point: [f32; 3],
    pub way: Vec<[f32; 3]>,
}

impl NavMesh {
    /// The triangle a location resolves to (`00696a50`), and the
    /// triangle's height there above the point.
    // Translated from 00696a50 and 006b9cf0 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn find_triangle(&self, p: [f32; 3]) -> Option<(usize, f32)> {
        // Only the triangles over the point's grid square can hold it; the
        // index lists them in the mesh's order, so the first close enough is
        // the same one as going through them all.
        let under = self.index().get(&super::grid_square(p[0], p[1]));
        self.find_triangle_among(p, under.into_iter().flatten().copied())
    }

    fn find_triangle_among(
        &self,
        p: [f32; 3],
        triangles: impl Iterator<Item = usize>,
    ) -> Option<(usize, f32)> {
        let near_land = self.land_height(p).is_some_and(|h| p[2] < h + NEAR_LAND);
        let mut best: Option<(usize, f32)> = None;
        for t in triangles {
            if self.triangles[t].flags & NO_LOCATION != 0 {
                continue;
            }
            let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
            let Some(z) = height_in(a, b, c, p) else {
                continue;
            };
            let dz = z - p[2];
            if dz > TRIANGLE_ABOVE || (!near_land && dz < -TRIANGLE_BELOW) {
                continue;
            }
            if best.map_or(true, |(_, d)| dz.abs() < d.abs()) {
                best = Some((t, dz));
                if dz.abs() < CLOSE_ENOUGH {
                    return best;
                }
            }
        }
        best
    }

    /// The open-edge spots near a point (`006d4120`), best first: those
    /// whose height lies in the window first, then the nearest; at most
    /// [`CANDIDATES`].
    // Translated from 006d4120 and 006d4020 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn edge_spots(&self, p: [f32; 3], radius: f32) -> Vec<EdgeSpot> {
        let r2 = radius * radius;
        let mut out = Vec::new();
        for t in 0..self.triangles.len() {
            let tri = &self.triangles[t];
            for e in 0..3 {
                if tri.neighbors[e].is_some() || tri.linked & (1 << e) != 0 {
                    continue;
                }
                let (a, b) = self.edge_ends(t, e);
                let q = nearest_on_segment(a, b, p);
                let d2: f32 = (0..3).map(|k| (p[k] - q[k]).powi(2)).sum();
                if d2 >= r2 {
                    continue;
                }
                let middle = self.centroid(t);
                let point = [0, 1, 2].map(|k| q[k] + (middle[k] - q[k]) * 0.1);
                out.push(EdgeSpot {
                    triangle: t,
                    edge: e,
                    point,
                    distance_sq: d2,
                    dz: p[2] - q[2],
                });
            }
        }
        let in_window = |s: &EdgeSpot| WINDOW_BELOW < s.dz && s.dz < WINDOW_ABOVE;
        out.sort_by(|x, y| {
            in_window(y)
                .cmp(&in_window(x))
                .then(x.distance_sq.total_cmp(&y.distance_sq))
        });
        out.truncate(CANDIDATES);
        out
    }

    /// Walks the ray-cast way from `from` to `to` (`006e6e40`, each step
    /// `006e6f90`): whether there is one.
    // Translated from 006e6e40 and 006e6f90 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn ray_way(&self, from: [f32; 3], to: [f32; 3], radius: f32) -> bool {
        let Some(pick) = self.pick.0.as_ref() else {
            return false;
        };
        let fall = if self.fall_height > 0.0 {
            self.fall_height
        } else {
            256.0
        };
        let step = radius.max(STEP_MIN);
        let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        // One step: a ray `LIFT` above the ground to the next point (at the
        // current height), then down to the ground there.
        let hop = |at: [f32; 3], next: [f32; 3]| -> Option<[f32; 3]> {
            let a = [at[0], at[1], at[2] + LIFT];
            let b = [next[0], next[1], at[2] + LIFT];
            if pick.pick(a, b).is_some() {
                return None;
            }
            let down = [b[0], b[1], b[2] - (fall + LIFT)];
            let f = pick.pick(b, down)?;
            Some([b[0], b[1], b[2] - (fall + LIFT) * f])
        };
        let mut at = from;
        if length > 1e-3 {
            let n = (length / step) as usize;
            let dir = d.map(|v| v / length);
            for _ in 0..n {
                let next = [at[0] + dir[0] * step, at[1] + dir[1] * step, at[2]];
                match hop(at, next) {
                    Some(g) => at = g,
                    None => return false,
                }
            }
        }
        match hop(at, to) {
            Some(g) => (g[2] - to[2]).abs() < END_TOLERANCE,
            None => false,
        }
    }

    /// Where a path's start is on the navmesh (`006caa40`): its own
    /// triangle, else the first open-edge spot the location can reach by
    /// ray casts. `None`: no way onto the navmesh.
    pub fn start_end(&self, from: [f32; 3], radius: f32) -> Option<End> {
        if let Some((t, _)) = self.find_triangle(from) {
            return Some(End {
                triangle: t,
                point: from,
                way: Vec::new(),
            });
        }
        self.edge_spots(from, FIND_CLOSEST_EDGES_RADIUS)
            .into_iter()
            .find(|s| self.ray_way(from, s.point, radius))
            .map(|s| End {
                triangle: s.triangle,
                point: s.point,
                way: vec![from, s.point],
            })
    }

    /// Where a path's goal is on the navmesh (`006cac90`): its own
    /// triangle; else an open-edge spot within `target_radius` in the
    /// height window (no ray cast); else the first spot from which the
    /// goal can be reached by ray casts.
    pub fn goal_end(&self, to: [f32; 3], radius: f32, target_radius: f32) -> Option<End> {
        if let Some((t, _)) = self.find_triangle(to) {
            return Some(End {
                triangle: t,
                point: to,
                way: Vec::new(),
            });
        }
        let spots = self.edge_spots(to, FIND_CLOSEST_EDGES_RADIUS);
        let near = spots.iter().find(|s| {
            s.distance_sq < target_radius * target_radius
                && WINDOW_BELOW < s.dz
                && s.dz < WINDOW_ABOVE
        });
        if let Some(s) = near {
            return Some(End {
                triangle: s.triangle,
                point: s.point,
                way: Vec::new(),
            });
        }
        spots
            .into_iter()
            .find(|s| self.ray_way(s.point, to, radius))
            .map(|s| End {
                triangle: s.triangle,
                point: s.point,
                way: vec![s.point, to],
            })
    }

    /// Why a location has no way onto the navmesh, for messages: the
    /// triangle under it from above and how high that is above it, the
    /// land under it, and how many edge spots there were.
    pub fn explain_off(&self, p: [f32; 3]) -> String {
        let under: Vec<String> = (0..self.triangles.len())
            .filter_map(|t| {
                let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
                height_in(a, b, c, p).map(|z| format!("{t} {:+.0}", z - p[2]))
            })
            .take(4)
            .collect();
        let spots = self.edge_spots(p, FIND_CLOSEST_EDGES_RADIUS);
        format!(
            "triangles under it (height above): [{}], land {:?}, {} open-edge spots{}, picks {}",
            under.join(", "),
            self.land_height(p).map(|h| (h - p[2]).round()),
            spots.len(),
            spots
                .first()
                .map(|s| format!(
                    " (nearest {:.0} away, {:+.0} below)",
                    s.distance_sq.sqrt(),
                    s.dz
                ))
                .unwrap_or_default(),
            if self.pick.0.is_some() { "on" } else { "off" }
        )
    }

    /// The point of triangle `t` nearest `p` (from above).
    pub fn nearest_in(&self, t: usize, p: [f32; 3]) -> [f32; 3] {
        let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
        closest_in_triangle(a, b, c, p)
    }
}

/// The point of segment `a`–`b` nearest `p` (`006b9c20`).
fn nearest_on_segment(a: [f32; 3], b: [f32; 3], p: [f32; 3]) -> [f32; 3] {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    if len2 <= 1e-9 {
        return a;
    }
    let t = (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1] + (p[2] - a[2]) * d[2]) / len2)
        .clamp(0.0, 1.0);
    [a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{NavTriangle, PathPick, Picker};
    use std::sync::Arc;

    /// A floor at z = 0 everywhere, and a wall across x = 150 from y = 0
    /// to y = 300 standing 200 high.
    struct Floor;
    impl PathPick for Floor {
        fn pick(&self, from: [f32; 3], to: [f32; 3]) -> Option<f32> {
            // The wall.
            if (from[0] - 150.0) * (to[0] - 150.0) < 0.0 {
                let f = (150.0 - from[0]) / (to[0] - from[0]);
                let y = from[1] + (to[1] - from[1]) * f;
                let z = from[2] + (to[2] - from[2]) * f;
                if (0.0..=300.0).contains(&y) && z < 200.0 {
                    return Some(f);
                }
            }
            // The floor.
            if from[2] >= 0.0 && to[2] < 0.0 {
                return Some(from[2] / (from[2] - to[2]));
            }
            None
        }
    }

    /// One square of navmesh, (0,0)–(100,100) at z = 0, every edge open.
    fn square() -> NavMesh {
        NavMesh {
            vertices: vec![
                [0.0, 0.0, 0.0],
                [100.0, 0.0, 0.0],
                [100.0, 100.0, 0.0],
                [0.0, 100.0, 0.0],
            ],
            triangles: vec![
                NavTriangle {
                    vertices: [0, 1, 2],
                    neighbors: [None, None, Some(1)],
                    linked: 0b100,
                    ..Default::default()
                },
                NavTriangle {
                    vertices: [0, 2, 3],
                    neighbors: [Some(0), None, None],
                    linked: 0b001,
                    ..Default::default()
                },
            ],
            pick: Picker(Some(Arc::new(Floor))),
            ..Default::default()
        }
    }

    #[test]
    fn a_location_resolves_to_a_triangle_within_its_height_window() {
        let mesh = square();
        assert_eq!(mesh.find_triangle([60.0, 20.0, 10.0]).map(|t| t.0), Some(0));
        // The triangle 64 or less above the point: on it; more: not.
        assert!(mesh.find_triangle([60.0, 20.0, -60.0]).is_some());
        assert!(mesh.find_triangle([60.0, 20.0, -70.0]).is_none());
        // 180 or less below: on it; more (no land here): not.
        assert!(mesh.find_triangle([60.0, 20.0, 170.0]).is_some());
        assert!(mesh.find_triangle([60.0, 20.0, 190.0]).is_none());
        // Outside it from above: not.
        assert!(mesh.find_triangle([160.0, 20.0, 0.0]).is_none());
    }

    /// Looking only at the triangles indexed over the point's square finds
    /// what going through every triangle finds: a generated stack of
    /// overlapping layers of random triangles (bridges over ground) at
    /// Goodsprings' coordinates, and points over and between them.
    #[test]
    fn the_indexed_lookup_finds_what_the_full_scan_finds() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 40) as f32 / (1u64 << 24) as f32
        };
        let (ox, oy) = (-68000.0, 5000.0);
        let mut mesh = NavMesh::default();
        for _ in 0..400 {
            let (cx, cy, cz) = (
                ox + next() * 3000.0,
                oy + next() * 3000.0,
                8300.0 + next() * 400.0,
            );
            let base = mesh.vertices.len();
            for _ in 0..3 {
                mesh.vertices.push([
                    cx + (next() - 0.5) * 600.0,
                    cy + (next() - 0.5) * 600.0,
                    cz + (next() - 0.5) * 60.0,
                ]);
            }
            mesh.triangles.push(NavTriangle {
                vertices: [base, base + 1, base + 2],
                ..Default::default()
            });
        }
        let mut found = 0;
        for _ in 0..4000 {
            let p = [
                ox + next() * 3000.0,
                oy + next() * 3000.0,
                8200.0 + next() * 600.0,
            ];
            let indexed = mesh.find_triangle(p);
            let all = mesh.find_triangle_among(p, 0..mesh.triangles.len());
            assert_eq!(indexed, all, "{p:?}");
            found += usize::from(all.is_some());
        }
        assert!(found > 500, "{found}");
    }

    #[test]
    fn an_end_off_the_navmesh_is_joined_by_ray_casts_to_an_open_edge() {
        let mesh = square();
        // 60 east of the square: the nearest open edge spot is on its east
        // edge, a tenth of the way in.
        let spots = mesh.edge_spots([160.0, 50.0, 0.0], FIND_CLOSEST_EDGES_RADIUS);
        assert!(!spots.is_empty());
        assert!(
            (spots[0].distance_sq - 60.0 * 60.0).abs() < 1e-2,
            "{spots:?}"
        );
        // But the wall at x = 150 is in the way: from the south, past the
        // wall's end, the way is clear.
        let start = mesh.start_end([200.0, 50.0, 0.0], 35.0);
        assert!(start.is_none(), "{start:?}");
        let start = mesh.start_end([120.0, 330.0, 0.0], 35.0).unwrap();
        assert_eq!(start.way.len(), 2);
        assert_eq!(start.way[0], [120.0, 330.0, 0.0]);
        // A goal within the target radius of an edge needs no ray cast.
        let goal = mesh.goal_end([110.0, 50.0, 0.0], 35.0, 20.0).unwrap();
        assert!(goal.way.is_empty() && goal.point[0] < 100.0);
        // Farther: the way from the spot to the goal is walked.
        let goal = mesh.goal_end([140.0, 50.0, 0.0], 35.0, 20.0).unwrap();
        assert_eq!(goal.way.last(), Some(&[140.0, 50.0, 0.0]));
    }
}
