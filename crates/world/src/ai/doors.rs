//! Closed doors on the navmesh, as the game's navmesh obstacle manager
//! marks them (`bCutDoors:Pathfinding` 1, `00f88de0`).
//!
//! A door's body is an obstacle (`NavMeshObstacleManager` (Xbox PDB),
//! `006c02c0`..): a box in the body's own axes. When a door is closed (its
//! `Close` event or `SetOpenState`, `0047ac70`/`0047aec0` →
//! `NavMeshObstacleManager::OnDoorClose` (Xbox PDB) `006c0f10`; and as the
//! door's 3D loads, `ProcessNewDoors` `006c5190`), a task (operation 0x30,
//! `006c8170`) adds it to each navmesh it overlaps (`NavMesh::AddClosedDoor`
//! (Xbox PDB), `006997e0`): the box is thinned along its thinnest side to a
//! sheet 2 % as thick (each face moved in by 0.49 of that side, `0106b168`),
//! and every triangle it overlaps (`006987d0`) that isn't already a portal
//! of this door gets flag 0x1000 and a portal entry for the door. Opening
//! (`OnDoorOpen` `006c0dc0`, operation 0x40) takes them away again
//! (`NavMesh::RemoveClosedDoor` (Xbox PDB), `00699a30`). Sliding doors
//! (`DOOR` flag 0x10, `00518080`) are never obstacles.
//!
//! A triangle overlaps when (`006987d0`): its box overlaps the obstacle's
//! (its top raised by 128, `0102e430`), it isn't flagged 0x20, and its
//! middle is in the sheet (within 128 of its height range, `0101e704`), or
//! the sheet's middle is in the triangle, or one of its edges crosses one of
//! the sheet's four sides, seen from above in the box's axes.
//!
//! Not followed (labelled): the obstacle box is the door model's collision
//! part's box in the door's placed axes (the game's is the Havok body's own,
//! a `bhkRigidBodyT`'s rotation included; their being the same for door
//! leaves is assumed); the tasks run at once, not in the background
//! (`bBackgroundNavmeshUpdate`).

use esm::FormId;

use super::{height_in, offmesh::NO_LOCATION, NavMesh};
use crate::ai::navsearch::DOOR;

/// A door's obstacle box: its centre, axes (unit, the rows of a rotation:
/// local x, y, z in world space) and half sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DoorBox {
    pub center: [f32; 3],
    pub axes: [[f32; 3]; 3],
    pub half: [f32; 3],
}

/// How much of the thinnest side each face moves in (`0106b168`, 0.49).
const THIN: f32 = 0.49;
/// The triangle's top is raised by this for the box test (`0102e430`).
const RAISE: f32 = 128.0;
/// The height tolerance of the middle tests (`0101e704`).
const HEIGHT_TOLERANCE: f32 = 128.0;

impl DoorBox {
    fn local(&self, p: [f32; 3]) -> [f32; 3] {
        let d = [
            p[0] - self.center[0],
            p[1] - self.center[1],
            p[2] - self.center[2],
        ];
        self.axes.map(|a| a[0] * d[0] + a[1] * d[1] + a[2] * d[2])
    }

    /// The world box around it.
    fn world_box(&self) -> ([f32; 3], [f32; 3]) {
        let mut lo = self.center;
        let mut hi = self.center;
        for k in 0..3 {
            let r: f32 = (0..3).map(|i| (self.axes[i][k] * self.half[i]).abs()).sum();
            lo[k] -= r;
            hi[k] += r;
        }
        (lo, hi)
    }
}

impl NavMesh {
    /// A closed door's triangles (`006997e0`).
    // Translated from 006997e0 and 006987d0 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn add_closed_door(&mut self, door: FormId, boxes: &[DoorBox]) {
        self.doors_closed.insert(door);
        for b in boxes {
            // The thinnest side, thinned (x, y; a thinnest z doesn't change
            // the sheet seen from above).
            let size = b.half.map(|h| 2.0 * h);
            let mut grow = [0.0f32; 3];
            let k = if size[0] <= size[1] {
                if size[0] <= size[2] {
                    0
                } else {
                    2
                }
            } else if size[1] <= size[2] {
                1
            } else {
                2
            };
            grow[k] = -size[k] * THIN;
            let rect = [
                (-b.half[0] - grow[0], -b.half[1] - grow[1]),
                (-b.half[0] - grow[0], b.half[1] + grow[1]),
                (b.half[0] + grow[0], b.half[1] + grow[1]),
                (b.half[0] + grow[0], -b.half[1] - grow[1]),
            ];
            let (lo, hi) = b.world_box();
            let height = hi[2] - lo[2];
            for t in 0..self.triangles.len() {
                if self.triangles[t].flags & NO_LOCATION != 0 {
                    continue;
                }
                if self.door_portals.get(&t) == Some(&door)
                    || self.door_triangles.get(&t) == Some(&door)
                {
                    continue;
                }
                let corners = [0, 1, 2].map(|i| self.corner(t, i));
                let tlo =
                    [0, 1, 2].map(|k| corners.iter().map(|c| c[k]).fold(f32::INFINITY, f32::min));
                let thi = [0, 1, 2].map(|k| {
                    corners
                        .iter()
                        .map(|c| c[k])
                        .fold(f32::NEG_INFINITY, f32::max)
                });
                if thi[0] < lo[0]
                    || tlo[0] > hi[0]
                    || thi[1] < lo[1]
                    || tlo[1] > hi[1]
                    || thi[2] + RAISE < lo[2]
                    || tlo[2] > hi[2]
                {
                    continue;
                }
                let local = corners.map(|c| b.local(c));
                let middle = b.local(self.centroid(t));
                let inside = |p: [f32; 3]| {
                    p[0] >= rect[0].0
                        && p[0] <= rect[2].0
                        && p[1] >= rect[0].1
                        && p[1] <= rect[2].1
                        && p[2].abs() <= b.half[2] + height.max(HEIGHT_TOLERANCE)
                };
                let mut hit = inside(middle);
                if !hit {
                    // The sheet's middle in the triangle.
                    hit = height_in(local[0], local[1], local[2], [0.0, 0.0, 0.0])
                        .is_some_and(|z| z.abs() <= height.max(HEIGHT_TOLERANCE));
                }
                if !hit {
                    'edges: for i in 0..3 {
                        let (p, q) = (local[i], local[(i + 1) % 3]);
                        for j in 0..4 {
                            let (r, s) = (rect[j], rect[(j + 1) % 4]);
                            if segments_cross((p[0], p[1]), (q[0], q[1]), r, s) {
                                hit = true;
                                break 'edges;
                            }
                        }
                    }
                }
                if hit {
                    self.triangles[t].flags |= DOOR;
                    self.door_triangles.insert(t, door);
                }
            }
        }
    }

    /// An opened door's triangles freed (`00699a30`).
    // Translated from 00699a30 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn remove_closed_door(&mut self, door: FormId) {
        self.doors_closed.remove(&door);
        let mine: Vec<usize> = self
            .door_triangles
            .iter()
            .filter(|(_, d)| **d == door)
            .map(|(t, _)| *t)
            .collect();
        for t in mine {
            self.door_triangles.remove(&t);
            if t < self.triangles.len() {
                self.triangles[t].flags &= !DOOR;
            }
        }
    }

    /// The doors marked closed now (sorted).
    pub fn closed_doors(&self) -> Vec<FormId> {
        self.doors_closed.iter().copied().collect()
    }
}

fn segments_cross(p: (f32, f32), q: (f32, f32), r: (f32, f32), s: (f32, f32)) -> bool {
    let cross = |a: (f32, f32), b: (f32, f32), c: (f32, f32)| {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    };
    let d1 = cross(r, s, p);
    let d2 = cross(r, s, q);
    let d3 = cross(p, q, r);
    let d4 = cross(p, q, s);
    ((d1 > 0.0) != (d2 > 0.0)) && ((d3 > 0.0) != (d4 > 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::navsearch::{DoorWay, PathRequest};
    use crate::ai::NavTriangle;

    /// A strip 400 long (x) and 100 wide, four squares of two triangles.
    fn strip() -> NavMesh {
        let mut vertices = Vec::new();
        for i in 0..=4 {
            vertices.push([i as f32 * 100.0, 0.0, 0.0]);
            vertices.push([i as f32 * 100.0, 100.0, 0.0]);
        }
        let mut triangles = Vec::new();
        for i in 0..4 {
            let (sw, nw, se, ne) = (2 * i, 2 * i + 1, 2 * i + 2, 2 * i + 3);
            let east = (i + 1 < 4).then_some(2 * (i + 1) + 1);
            triangles.push(NavTriangle {
                vertices: [sw, se, ne],
                neighbors: [None, east, Some(2 * i + 1)],
                ..Default::default()
            });
            let west = (i > 0).then(|| 2 * (i - 1));
            triangles.push(NavTriangle {
                vertices: [sw, ne, nw],
                neighbors: [Some(2 * i), None, west],
                ..Default::default()
            });
        }
        NavMesh {
            vertices,
            triangles,
            ..Default::default()
        }
    }

    #[test]
    fn a_closed_door_marks_the_triangles_under_its_leaf_and_opening_frees_them() {
        let mut mesh = strip();
        let door = FormId(0x500);
        // A leaf 4 thick (x) across the strip at x = 210, 100 tall.
        let leaf = DoorBox {
            center: [210.0, 50.0, 50.0],
            axes: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            half: [2.0, 50.0, 50.0],
        };
        mesh.add_closed_door(door, &[leaf]);
        let marked: Vec<usize> = {
            let mut v: Vec<usize> = mesh.door_triangles.keys().copied().collect();
            v.sort();
            v
        };
        // Square 2 (x 200..300): its two triangles; nothing else.
        assert_eq!(marked, vec![4, 5]);
        assert!(mesh.triangles[4].flags & DOOR != 0);
        // Locked: crossing costs a hundred times as much.
        mesh.door_rules.insert(door, DoorWay::Locked);
        let at = mesh.centroid(2);
        let plain = PathRequest::default();
        let locked = mesh.crossing_cost_for_test(2, 5, at, &plain);
        mesh.door_rules.insert(door, DoorWay::Open);
        let open = mesh.crossing_cost_for_test(2, 5, at, &plain);
        assert!((locked - open * 100.0).abs() < 1e-2, "{locked} {open}");
        // A path through it names the door.
        let (_, doors) = mesh
            .path_with_doors([20.0, 50.0, 0.0], [380.0, 50.0, 0.0])
            .unwrap();
        assert!(doors.iter().any(|d| d.0 == door), "{doors:?}");
        // Opened: freed.
        mesh.remove_closed_door(door);
        assert!(mesh.door_triangles.is_empty());
        assert!(mesh.triangles[4].flags & DOOR == 0);
    }

    #[test]
    fn a_closed_door_over_no_triangle_still_counts_as_marked() {
        let mut mesh = strip();
        let gate = FormId(0x501);
        // Far off the strip: covers nothing.
        let leaf = DoorBox {
            center: [5000.0, 5000.0, 50.0],
            axes: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            half: [2.0, 50.0, 50.0],
        };
        mesh.add_closed_door(gate, &[leaf]);
        assert!(mesh.door_triangles.is_empty());
        assert_eq!(mesh.closed_doors(), vec![gate]);
        mesh.remove_closed_door(gate);
        assert!(mesh.closed_doors().is_empty());
    }
}
