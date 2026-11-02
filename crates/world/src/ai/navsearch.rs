//! The detailed path search over the attached navmeshes, as the game runs
//! it for a path request (`PathingRequest` (Xbox PDB)): `PathBuilder::
//! RunNavMeshSearch` (Xbox PDB, `006cd670`) resolves both ends to their
//! triangles and runs a `NavMeshSearch` (Xbox PDB, vtable `0106b63c`,
//! `006a6b70`), the same A* template as the navmesh info search
//! ([`super::navinfo`]): nodes are triangles, each at its middle (the
//! mean of its corners, `00558220`); the open list is ordered by cost +
//! estimate, a node reached more cheaply is taken up again, and a
//! neighbour improved through the node's own parent ends the search as
//! failed (`006a8010`, `006a7e00`).
//!
//! Crossing an edge (`006a6fa0`, the search's `GetNodeConnections`) costs
//! the distance between the two middles, times:
//! - 0.01 into a preferred triangle (flag 0x40, `00693b70`; the double at
//!   `01016408`);
//! - (the avoid nodes' summed cost at the new middle + 1) when it is above
//!   0 (`PathingAvoidNodeArray::GetCostForPoint` (Xbox PDB), `006dc990`);
//! - the summed cost of the obstacles marked on the new triangle (flag
//!   0x80000000, `00691660`; someone stuck there, [`NavMesh::mark_obstacle`]);
//! - 0.5 + 0.5 × clamp(1 − the crossed edge's length ÷ 512, 0, 1): wide
//!   edges are cheaper (`010231a0` = 512, `01011588` = 0.5);
//! - 4 from water into water, 5 into or out of water (flag 0x200; requests
//!   may enter water by default, request +0x9d = 1 in `006e2420`);
//! - 100 across an edge the request keeps off (`006cc5e0`'s retries);
//! - 100 through a locked door (flag 0x1000, the door found for the
//!   triangle in the navmesh's door portal list, `00699af0`; `00518000` on
//!   the door's lock), when the walker may open it at all.
//!
//! An edge isn't crossed when it leads to the triangle itself, when the
//! triangle across doesn't link back (`0068f460`), when its link to another
//! navmesh is of kind 1 (`0068f230`), into a triangle flagged 0x10 for an
//! actor wider than `fPathingLargeActorRadius` (80), into water for a
//! request that may not swim, or through a door the walker may not use.
//!
//! The estimate is 0.01 × the distance from a triangle's middle to the goal
//! triangle's (`006a6ef0`); the start node's is the whole distance
//! (`006a6b70`). The goal is reached on the goal triangle (`006a79c0`).
//!
//! Not followed (labelled): the request's search radius (+0x78, 0 by
//! default), the turn-angle term (request +0xa2, 0 by default), the
//! goal test's "near enough" case for wide actors with avoid nodes, the
//! navmesh list of a long path's attached run (+0x2088; the viewer's
//! navmesh is already only the attached cells') and the door test
//! `0057b460` (a door whose extra data 0x1c has flag 0x100 is never
//! crossed; what that data is wasn't traced). Flag 0x1000 is never in the
//! data (`NVDP` portal triangles carry 0x400): the game sets it at run
//! time on the triangles under a closed door that aren't already its
//! portals, adding them to the portal list (`006997e0`, from `006c8170`;
//! cleared by `00699a30` when it opens): [`super::doors`]. Without a rule
//! in the request, the navmesh's own door rules
//! ([`NavMesh::door_rules`]) are asked.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use esm::FormId;

use super::NavMesh;
use crate::movement::AvoidNode;

/// Triangle flags (`NVTR` flags; [`super::NavTriangle::flags`]).
pub const NO_LARGE_CREATURES: u32 = 0x10;
pub const PREFERRED: u32 = 0x40;
pub const WATER: u32 = 0x200;
pub const DOOR: u32 = 0x1000;
/// Set at run time on a triangle someone got stuck on (`009e4cf0` →
/// `00691510` → `00691570`).
pub const OBSTACLE: u32 = 0x8000_0000;

/// `[Pathfinding] fPathingLargeActorRadius` (exe default 80; this install's
/// INI sets none): wider actors keep off triangles flagged 0x10 (`006a6fa0`)
/// and get the wide straight-line test (`006cd1f0`).
pub const LARGE_ACTOR_RADIUS: f32 = 80.0;

/// Whether a walker may go through a door on its path (`006a6fa0`'s door
/// test: the request's +0xa4, `00502450` and the lock `00518000`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorWay {
    /// Not through it.
    Shut,
    /// Through it.
    Open,
    /// Through it, but locked: crossing its triangle costs 100 times as much.
    Locked,
}

/// What a path is asked for with (`PathingRequest` (Xbox PDB), defaults from
/// its constructor `006e2420`).
#[derive(Clone, Copy)]
pub struct PathRequest<'a> {
    /// The walker's radius (+0x68; 35 by default).
    pub radius: f32,
    /// Places to keep away from (+0x60's array, `009e5ae0`).
    pub avoid: &'a [AvoidNode],
    /// No doors at all (+0x98; 0 by default).
    pub no_doors: bool,
    /// May go into water (+0x9d; 1 by default).
    pub can_swim: bool,
    /// What the walker may do with a door; `None`: every door is used and
    /// none counts as locked (callers that don't know the walker; not the
    /// game's: it always asks).
    pub door: Option<&'a dyn Fn(FormId) -> DoorWay>,
    /// Edges to keep off: crossing one costs 100 times as much (+0x2084,
    /// `006a6fa0`; filled by the smoother's failures between tries,
    /// `006cc5e0`): (triangle, edge).
    pub avoid_edges: &'a [(usize, usize)],
    /// How near the goal counts as there (+0x74, "Target Radius"): a goal
    /// off the navmesh within it of an open edge needs no ray-cast way
    /// (`006cac90`, [`super::offmesh`]).
    pub target_radius: f32,
}

impl Default for PathRequest<'_> {
    fn default() -> Self {
        PathRequest {
            target_radius: 0.0,
            radius: crate::movement::REQUEST_RADIUS,
            avoid: &[],
            no_doors: false,
            can_swim: true,
            door: None,
            avoid_edges: &[],
        }
    }
}

/// An obstacle marked on a triangle (`00691510`: a 0x28-byte record on the
/// navmesh, `006915d0`): where, how wide (the walker's request radius), and
/// what it adds to the triangle's cost multiplier (1.0 from the stuck test).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavObstacle {
    pub triangle: usize,
    pub position: [f32; 3],
    pub radius: f32,
    pub cost: f32,
}

/// The avoid nodes' cost at a point (`006dc990`): each node's (1 − d² ÷ r²)
/// × its cost while the point is within its radius (`006dc780`, a node
/// about a point: kinds 0 and 2), summed.
// Translated from 006dc990 and 006dc780 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn avoid_cost(avoid: &[AvoidNode], p: [f32; 3]) -> f32 {
    avoid
        .iter()
        .map(|n| {
            let r2 = n.radius * n.radius;
            let d2: f32 = (0..3).map(|k| (p[k] - n.position[k]).powi(2)).sum();
            if r2 <= d2 {
                0.0
            } else {
                (1.0 - d2 / r2) * n.cost
            }
        })
        .sum()
}

impl NavMesh {
    /// The triangle across edge `e` of `t`, and the edge of that triangle
    /// that leads back (`0068f460`: a link only counts when the triangle
    /// across links back).
    pub(crate) fn across(&self, t: usize, e: usize) -> Option<(usize, usize)> {
        let n = self.triangles[t].neighbors[e]?;
        let back = self.triangles[n]
            .neighbors
            .iter()
            .position(|&m| m == Some(t))?;
        Some((n, back))
    }

    /// The two ends of edge `e` of triangle `t` (`0068f0c0`).
    pub(crate) fn edge_ends(&self, t: usize, e: usize) -> ([f32; 3], [f32; 3]) {
        (self.corner(t, e), self.corner(t, e + 1))
    }

    /// Marks an obstacle on a triangle (`00691510`): the record, and the
    /// triangle's flag 0x80000000.
    pub fn mark_obstacle(&mut self, triangle: usize, position: [f32; 3], radius: f32, cost: f32) {
        if triangle >= self.triangles.len() {
            return;
        }
        self.triangles[triangle].flags |= OBSTACLE;
        self.obstacles.push(NavObstacle {
            triangle,
            position,
            radius,
            cost,
        });
    }

    /// The search from triangle `start` to triangle `goal`: whether the goal
    /// was reached, and the triangles from the start to it (or, when not, to
    /// the node whose estimate was least, `006a7d60` called with "closest").
    // Translated from 006a6b70, 006a8010, 006a6fa0, 006a6ef0 and 006a79c0 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn search(&self, start: usize, goal: usize, request: &PathRequest) -> (bool, Vec<usize>) {
        #[derive(Clone, Copy)]
        struct Node {
            triangle: usize,
            g: f32,
            h: f32,
            parent: Option<usize>,
            version: u32,
            open: bool,
        }
        #[derive(PartialEq)]
        struct Entry {
            f: f32,
            seq: u64,
            node: usize,
            version: u32,
        }
        impl Eq for Entry {}
        impl Ord for Entry {
            fn cmp(&self, o: &Self) -> Ordering {
                // Least f first; among equals the newest (inserted before
                // them in the game's list).
                o.f.total_cmp(&self.f).then(self.seq.cmp(&o.seq))
            }
        }
        impl PartialOrd for Entry {
            fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
                Some(self.cmp(o))
            }
        }
        if start >= self.triangles.len() || goal >= self.triangles.len() {
            return (false, Vec::new());
        }
        let goal_at = self.centroid(goal);
        let distance = |a: [f32; 3], b: [f32; 3]| -> f32 {
            (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt()
        };
        let mut nodes = vec![Node {
            triangle: start,
            g: 0.0,
            h: distance(self.centroid(start), goal_at),
            parent: None,
            version: 0,
            open: false,
        }];
        let mut node_of: HashMap<usize, usize> = HashMap::from([(start, 0)]);
        let mut open = BinaryHeap::new();
        let mut seq = 0u64;
        let push = |open: &mut BinaryHeap<Entry>, nodes: &mut [Node], n: usize, seq: &mut u64| {
            nodes[n].version += 1;
            nodes[n].open = true;
            *seq += 1;
            open.push(Entry {
                f: nodes[n].g + nodes[n].h,
                seq: *seq,
                node: n,
                version: nodes[n].version,
            });
        };
        let pop = |open: &mut BinaryHeap<Entry>, nodes: &mut [Node]| loop {
            let e = open.pop()?;
            if nodes[e.node].open && nodes[e.node].version == e.version {
                nodes[e.node].open = false;
                return Some(e.node);
            }
        };
        push(&mut open, &mut nodes, 0, &mut seq);
        let Some(mut current) = pop(&mut open, &mut nodes) else {
            return (false, Vec::new());
        };
        let mut best = current;
        let found = loop {
            if nodes[current].triangle == goal {
                best = current;
                break true;
            }
            let here = nodes[current].triangle;
            let at = self.centroid(here);
            let g = nodes[current].g;
            let mut improved = Vec::new();
            for e in 0..3 {
                let Some(cost) = self.crossing_cost(here, e, at, request) else {
                    continue;
                };
                let (n, _) = self.across(here, e).expect("costed above");
                let cost = g + cost;
                match node_of.get(&n) {
                    None => {
                        let k = nodes.len();
                        nodes.push(Node {
                            triangle: n,
                            g: cost,
                            h: distance(self.centroid(n), goal_at) * 0.01,
                            parent: None,
                            version: 0,
                            open: false,
                        });
                        node_of.insert(n, k);
                        improved.push(k);
                    }
                    Some(&k) if cost < nodes[k].g => {
                        nodes[k].g = cost;
                        improved.push(k);
                    }
                    Some(_) => {}
                }
            }
            let mut failed = false;
            for k in improved {
                nodes[k].open = false;
                if nodes[current].parent == Some(k) {
                    failed = true;
                    break;
                }
                nodes[k].parent = Some(current);
                push(&mut open, &mut nodes, k, &mut seq);
                if nodes[k].h < nodes[best].h {
                    best = k;
                }
            }
            if failed {
                break false;
            }
            match pop(&mut open, &mut nodes) {
                Some(k) => current = k,
                None => break false,
            }
        };
        let mut route = Vec::new();
        let mut at = Some(best);
        while let Some(k) = at {
            route.push(nodes[k].triangle);
            at = nodes[k].parent;
            if route.len() > nodes.len() {
                break;
            }
        }
        route.reverse();
        (found, route)
    }

    /// What crossing from triangle `from` into its neighbour `to` costs.
    #[cfg(test)]
    pub(crate) fn crossing_cost_for_test(
        &self,
        from: usize,
        to: usize,
        at: [f32; 3],
        request: &PathRequest,
    ) -> f32 {
        let e = (0..3)
            .find(|&e| self.triangles[from].neighbors[e] == Some(to))
            .expect("neighbours");
        self.crossing_cost(from, e, at, request).expect("crossable")
    }

    /// What crossing edge `e` of triangle `t` (its node at `at`) costs, if
    /// it may be crossed (`006a6fa0`'s loop body).
    fn crossing_cost(
        &self,
        t: usize,
        e: usize,
        at: [f32; 3],
        request: &PathRequest,
    ) -> Option<f32> {
        let (n, back) = self.across(t, e)?;
        if n == t || self.triangles[t].closed & (1 << e) != 0 {
            return None;
        }
        let flags = self.triangles[n].flags;
        // Doors.
        let mut door_mult = 1.0;
        if flags & DOOR != 0 {
            if request.no_doors {
                return None;
            }
            // The triangle's door (`00699af0`: the data's portals first,
            // then those added for closed doors at run time).
            let door = self
                .door_portals
                .get(&n)
                .or_else(|| self.door_triangles.get(&n))
                .copied();
            if let Some(door) = door {
                let way = match request.door {
                    Some(rule) => rule(door),
                    None => self.door_rules.get(&door).copied().unwrap_or(DoorWay::Open),
                };
                match way {
                    DoorWay::Shut => return None,
                    DoorWay::Open => {}
                    DoorWay::Locked => door_mult = 100.0,
                }
            }
        }
        // Wide actors.
        if request.radius > LARGE_ACTOR_RADIUS && flags & NO_LARGE_CREATURES != 0 {
            return None;
        }
        // Water.
        let into = flags & WATER != 0;
        let from = self.triangles[t].flags & WATER != 0;
        let water_mult = if !request.can_swim {
            if into {
                return None;
            }
            1.0
        } else if into == from {
            if into {
                4.0
            } else {
                1.0
            }
        } else {
            5.0
        };
        let middle = self.centroid(n);
        let mut cost = (0..3)
            .map(|k| (middle[k] - at[k]).powi(2))
            .sum::<f32>()
            .sqrt();
        if flags & PREFERRED != 0 {
            cost *= 0.01;
        }
        let avoid = avoid_cost(request.avoid, middle);
        if avoid > 0.0 {
            cost *= avoid + 1.0;
        }
        if flags & OBSTACLE != 0 {
            let marked: Vec<f32> = self
                .obstacles
                .iter()
                .filter(|o| o.triangle == n)
                .map(|o| o.cost)
                .collect();
            if !marked.is_empty() {
                cost *= marked.iter().sum::<f32>();
            }
        }
        // An edge to keep off: × 100.
        let keep_off = if request.avoid_edges.contains(&(t, e)) {
            100.0
        } else {
            1.0
        };
        let (a, b) = self.edge_ends(n, back);
        let length = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt();
        let narrow = (1.0 - length / 512.0).clamp(0.0, 1.0);
        cost *= (narrow * 0.5 + 0.5) * water_mult * door_mult * keep_off;
        Some(cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::NavTriangle;

    /// A strip of squares of `w` × 100 going east, each two triangles,
    /// numbered from the west: square `i` has triangles `2i` (south-east
    /// half) and `2i + 1` (north-west half).
    fn strip(squares: usize, w: f32) -> NavMesh {
        let mut vertices = Vec::new();
        for i in 0..=squares {
            vertices.push([i as f32 * w, 0.0, 0.0]);
            vertices.push([i as f32 * w, 100.0, 0.0]);
        }
        let mut triangles = Vec::new();
        for i in 0..squares {
            let (sw, nw, se, ne) = (2 * i, 2 * i + 1, 2 * i + 2, 2 * i + 3);
            // South-east half: sw, se, ne; edge 2 (ne → sw) to the other
            // half; edge 1 (se → ne) east.
            let east = (i + 1 < squares).then_some(2 * (i + 1) + 1);
            triangles.push(NavTriangle {
                vertices: [sw, se, ne],
                neighbors: [None, east, Some(2 * i + 1)],
                ..Default::default()
            });
            // North-west half: sw, ne, nw; edge 0 back; edge 2 (nw → sw)
            // west.
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
    fn the_search_walks_triangle_to_triangle_and_costs_follow_the_flags() {
        let mut mesh = strip(4, 100.0);
        let request = PathRequest::default();
        let (found, route) = mesh.search(0, 7, &request);
        assert!(found);
        assert_eq!(route.first(), Some(&0));
        assert_eq!(route.last(), Some(&7));
        // Each step crosses a shared edge.
        for w in route.windows(2) {
            assert!(
                mesh.across(w[0], 0).map(|x| x.0) == Some(w[1])
                    || mesh.across(w[0], 1).map(|x| x.0) == Some(w[1])
                    || mesh.across(w[0], 2).map(|x| x.0) == Some(w[1])
            );
        }
        // From triangle 0 into its other half: the middles' distance, ×
        // (0.5 + 0.5 × (1 − the diagonal's length ÷ 512)).
        let at = mesh.centroid(0);
        let d = {
            let m = mesh.centroid(1);
            ((m[0] - at[0]).powi(2) + (m[1] - at[1]).powi(2)).sqrt()
        };
        let diagonal = 100.0f32 * 2f32.sqrt();
        let expected = d * ((1.0 - diagonal / 512.0) * 0.5 + 0.5);
        let cost = mesh.crossing_cost(0, 2, at, &request).unwrap();
        assert!((cost - expected).abs() < 1e-3, "{cost} {expected}");
        // Preferred: a hundredth.
        mesh.triangles[1].flags |= PREFERRED;
        let preferred = mesh.crossing_cost(0, 2, at, &request).unwrap();
        assert!((preferred - expected * 0.01).abs() < 1e-4);
        mesh.triangles[1].flags &= !PREFERRED;
        // Water from dry land: × 5; may not swim: not at all.
        mesh.triangles[1].flags |= WATER;
        let wet = mesh.crossing_cost(0, 2, at, &request).unwrap();
        assert!((wet - expected * 5.0).abs() < 1e-3);
        let dry = PathRequest {
            can_swim: false,
            ..request
        };
        assert!(mesh.crossing_cost(0, 2, at, &dry).is_none());
        mesh.triangles[1].flags &= !WATER;
        // A locked door: × 100; a shut one: not crossed.
        mesh.triangles[1].flags |= DOOR;
        mesh.door_portals.insert(1, FormId(0x77));
        let locked = |_: FormId| DoorWay::Locked;
        let shut = |_: FormId| DoorWay::Shut;
        let through_locked = PathRequest {
            door: Some(&locked),
            ..request
        };
        let c = mesh.crossing_cost(0, 2, at, &through_locked).unwrap();
        assert!((c - expected * 100.0).abs() < 1e-2);
        let through_shut = PathRequest {
            door: Some(&shut),
            ..request
        };
        assert!(mesh.crossing_cost(0, 2, at, &through_shut).is_none());
        let no_doors = PathRequest {
            no_doors: true,
            ..request
        };
        assert!(mesh.crossing_cost(0, 2, at, &no_doors).is_none());
        // Wide actors keep off "no large creatures" triangles.
        mesh.triangles[1].flags = NO_LARGE_CREATURES;
        assert!(mesh.crossing_cost(0, 2, at, &request).is_some());
        let wide = PathRequest {
            radius: 100.0,
            ..request
        };
        assert!(mesh.crossing_cost(0, 2, at, &wide).is_none());
    }

    #[test]
    fn avoid_nodes_and_obstacles_multiply_the_cost() {
        let mut mesh = strip(2, 100.0);
        let request = PathRequest::default();
        let at = mesh.centroid(0);
        let plain = mesh.crossing_cost(0, 2, at, &request).unwrap();
        let middle = mesh.centroid(1);
        // A node right on the middle, cost 2: × (2 + 1).
        let node = AvoidNode {
            position: middle,
            radius: 50.0,
            cost: 2.0,
        };
        assert!((avoid_cost(&[node], middle) - 2.0).abs() < 1e-6);
        // Halfway out: (1 − 1/4) × 2.
        let off = [middle[0] + 25.0, middle[1], middle[2]];
        assert!((avoid_cost(&[node], off) - 1.5).abs() < 1e-6);
        let avoiding = PathRequest {
            avoid: &[node],
            ..request
        };
        let c = mesh.crossing_cost(0, 2, at, &avoiding).unwrap();
        assert!((c - plain * 3.0).abs() < 1e-3);
        // Two obstacles of cost 1 marked: × 2.
        mesh.mark_obstacle(1, middle, 35.0, 1.0);
        mesh.mark_obstacle(1, middle, 35.0, 1.0);
        let c = mesh.crossing_cost(0, 2, at, &request).unwrap();
        assert!((c - plain * 2.0).abs() < 1e-3);
    }

    #[test]
    fn a_link_that_does_not_lead_back_is_not_crossed() {
        let mut mesh = strip(2, 100.0);
        // Triangle 1 forgets triangle 0.
        mesh.triangles[1].neighbors[0] = None;
        assert!(mesh.across(0, 2).is_none());
        let (found, _) = mesh.search(0, 1, &PathRequest::default());
        assert!(!found);
    }
}
