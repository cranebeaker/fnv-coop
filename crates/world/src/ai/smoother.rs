//! The path smoother the game runs on the navmesh search's triangles for
//! people and creatures up to 150 wide (`PathSmootherPOVSearch` (Xbox
//! PDB), vtable `0106b9cc`; `PathBuilder::RunPathSmoother` (Xbox PDB)
//! `006cd8a0` takes it while `bUseOldPathSmoothing` is 0 and the walker's
//! radius isn't above 150 with `bUseAlternateSmoothingForPrime`): a search
//! over "points of visibility" that keeps the walker's radius off the
//! corridor's corners.
//!
//! The points (`CreatePathingNodes` (Xbox PDB), `006adcb0`):
//! - the start and the goal (radius 0, `006ae190`, `006ae280`);
//! - the corridor's corners: the shared edges' ends, on the left and on the
//!   right of the way (`006ae370`), each a circle of 1.2 × the walker's
//!   radius (`010290f0`) where its side bends into the corridor (the turn
//!   at it, the cross product of the unit ways in and out, beyond −0.05 or
//!   0.05 by side, `0106b9d8`, `0103c7c8`), and always the first and last
//!   of a side (`006aeba0`); a left corner is passed with it on the left
//!   (counterclockwise round it), a right one the other way;
//! - obstacles: the request's avoid nodes (`006aefc0`) and the obstacles
//!   marked on the navmeshes the corridor crosses (`006af2b0`), circles of
//!   their radius + 1.2 × the walker's, passed either way.
//!
//! The search (the A* template, keyed by point and the way round it,
//! `006b3420`): from a corner it tries the goal, then the corners after it
//! on its own side and those of the other side from the first one at its
//! place, each run until four of them have failed, then every obstacle; from
//! the start both sides from their first corners; from an obstacle every
//! corner (`006b0ad0`). A step between two circles (`006b0f40`) runs along
//! one of their common tangents (`006bb570`): the one that leaves the first
//! circle the way it is being gone round and reaches the second the way
//! that one must be gone round. It is taken when it climbs no steeper than
//! 1.2 (height² ≤ 1.44 × flat²), the arc round a corner before it is at
//! most 3.4657 radians (`0106ba00`), it is clear on the navmesh with room
//! on both sides (`006b25f0`: the line itself and lines 0.9 × the radius to
//! either side, each walked triangle to triangle, `006b2060`), and the
//! navmesh height where it reaches the second circle is within −64..180 of
//! it (`00697980`; `0106b9f0`, `0104ed58`). It costs (1 − 0.9 × the share
//! of preferred triangles the line crosses) × (the arc on the first circle,
//! plus the line's length, plus extra through obstacles: (share inside ×
//! `fPOVSmootherAvoidNodeCost` 7, plus 1) × the chord, `006b2c00`) × the
//! second point's cost; the estimate is 0.1 × the distance to the goal
//! (`006b1df0`), the start's 0.75 × it (`006ad770`).
//!
//! The path (`006afcc0`): for each step its two tangent points, the one
//! leaving a corner left out when the arc round it is under 50 long
//! (`0101e2c0`) (the game also asks `006a0660` there, not read). The
//! follower walks straight between them.
//!
//! Not followed (labelled): the start or goal standing within a circle (the
//! game cuts the circle with a chord there; here the point itself is the
//! tangent point), the start and goal's own obstacle points near the
//! navmesh's edge (`006af500`), the turn-angle points of a
//! request with +0xa2 set, avoid nodes of kind 2 (a second, costlier point,
//! cost 10), and what the game does when the search fails (`006b1970`
//! tries the nearest point reached straight to the goal; here: the path to
//! that point, then the goal when the straight step to it is clear).

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use super::navsearch::{PathRequest, LARGE_ACTOR_RADIUS, NO_LARGE_CREATURES, PREFERRED};
use super::{height_in, NavMesh};

/// Corners and obstacles are circles this many times the walker's radius
/// (`010290f0`, 1.2).
pub const CORNER_SCALE: f32 = 1.2;
/// How far a side may bend the wrong way and still get a corner (`0106b9d8`
/// −0.05, `0103c7c8` 0.05).
const TURN_SLACK: f32 = 0.05;
/// The side lines run this share of the radius to either side (`010177e8`,
/// `0106b9e8`, 0.9).
const SIDE_SHARE: f32 = 0.9;
/// The longest arc round a corner (`0106ba00`, 3.4657359 radians).
const ARC_LIMIT: f32 = 3.465_736;
/// Arcs shorter than this leave out their second tangent point
/// (`0101e2c0`, 50).
const SHORT_ARC: f32 = 50.0;
/// Navmesh height at a reached tangent point, relative to it: at most 180
/// above (`0104ed58`), more than 64 below (`0106b9f0`).
const HEIGHT_ABOVE: f32 = 180.0;
const HEIGHT_BELOW: f32 = -64.0;
/// `[Pathfinding] fPOVSmootherAvoidNodeCost` (7).
const AVOID_NODE_COST: f32 = 7.0;
/// A run of points to try ends after this many failures (`006b0ad0`).
const FAILURES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// A corner on the left of the way: gone round counterclockwise.
    Left,
    /// On the right: clockwise.
    Right,
    Start,
    Goal,
    Obstacle,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    kind: Kind,
    at: [f32; 3],
    radius: f32,
    cost: f32,
    /// A corner's place in its side's list, and the first corner of the
    /// other side at or after its place in the corridor (+0x3c, +0x40).
    own: usize,
    other: usize,
    /// A corner's shared edge (its place in the corridor) where it first
    /// comes.
    crossing: usize,
}

/// The way round a circle: counterclockwise (seen from above) or not.
type Ccw = bool;

/// A common tangent of two circles: the point leaving the first and the
/// point reaching the second, for going round the first `from_ccw` and
/// the second `to_ccw` (`006bb570`). A point at distance `r` to the right
/// of the way goes round counterclockwise.
fn tangent(a: &Point, from_ccw: Ccw, b: &Point, to_ccw: Ccw) -> Option<([f32; 3], [f32; 3])> {
    let sign = |ccw: Ccw| if ccw { 1.0f32 } else { -1.0 };
    let (cx, cy) = (b.at[0] - a.at[0], b.at[1] - a.at[1]);
    let d = cx.hypot(cy);
    if d < 1e-4 {
        return None;
    }
    let k = sign(to_ccw) * b.radius - sign(from_ccw) * a.radius;
    if k.abs() >= d {
        return None;
    }
    let theta = cy.atan2(cx) + (-k / d).asin();
    let (dx, dy) = (theta.cos(), theta.sin());
    // Right of the way.
    let (rx, ry) = (dy, -dx);
    let pa = [
        a.at[0] + sign(from_ccw) * a.radius * rx,
        a.at[1] + sign(from_ccw) * a.radius * ry,
        a.at[2],
    ];
    let pb = [
        b.at[0] + sign(to_ccw) * b.radius * rx,
        b.at[1] + sign(to_ccw) * b.radius * ry,
        b.at[2],
    ];
    Some((pa, pb))
}

/// The angle gone round a circle centred at `c` from `p` to `q`, going
/// round it `ccw` (0..2π, `006a0840`/`006a07a0`).
fn arc_angle(c: [f32; 3], p: [f32; 3], q: [f32; 3], ccw: Ccw) -> f32 {
    let (ux, uy) = (p[0] - c[0], p[1] - c[1]);
    let (vx, vy) = (q[0] - c[0], q[1] - c[1]);
    let mut a = (ux * vy - uy * vx).atan2(ux * vx + uy * vy);
    if !ccw {
        a = -a;
    }
    if a < 0.0 {
        a += std::f32::consts::TAU;
    }
    a
}

fn length(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|k| (b[k] - a[k]).powi(2)).sum::<f32>().sqrt()
}

impl NavMesh {
    /// The triangle whose outline (seen from above) holds `p`, nearest in
    /// height, within 200 (a path location resolved to its triangle).
    pub(crate) fn triangle_under(&self, p: [f32; 3]) -> Option<(usize, f32)> {
        let mut best: Option<(f32, usize, f32)> = None;
        // Only those over the point's grid square can hold it, each
        // square's in the mesh's order (so the first of the nearest wins,
        // as looking at every triangle did): the smoother asks this for
        // every line it walks.
        let under = self.index().get(&super::grid_square(p[0], p[1]));
        for &t in under.into_iter().flatten() {
            let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
            if let Some(z) = height_in(a, b, c, p) {
                let dz = (z - p[2]).abs();
                if best.map_or(true, |(d, ..)| dz < d) {
                    best = Some((dz, t, z));
                }
            }
        }
        best.filter(|(dz, ..)| *dz < 200.0).map(|(_, t, z)| (t, z))
    }

    /// Whether a point is on the navmesh and not on (within a unit of) an
    /// edge of its triangle that leads nowhere.
    fn well_inside(&self, p: [f32; 3]) -> bool {
        let Some((t, _)) = self.triangle_under(p) else {
            return false;
        };
        (0..3).all(|e| {
            if self.triangles[t].neighbors[e].is_some() {
                return true;
            }
            let (u, v) = self.edge_ends(t, e);
            let (ex, ey) = (v[0] - u[0], v[1] - u[1]);
            let l2 = ex * ex + ey * ey;
            if l2 < 1e-9 {
                return true;
            }
            let s = (((p[0] - u[0]) * ex + (p[1] - u[1]) * ey) / l2).clamp(0.0, 1.0);
            (p[0] - u[0] - ex * s).hypot(p[1] - u[1] - ey * s) >= 1.0
        })
    }

    /// Walks the line from `p` to `q` over the navmesh, triangle to
    /// triangle through the edges it leaves by (`006b2060`): the triangle
    /// it ends on and the share of preferred triangles it went through, or
    /// `None` when it leaves the navmesh, crosses a link of kind 1 or, for a
    /// walker wider than `fPathingLargeActorRadius`, enters a triangle
    /// flagged 0x10. At most 1000 triangles.
    // Translated from 006b2060 (decompiled, FalloutNV.exe 1.4.0.525).
    pub(crate) fn walk_line(&self, p: [f32; 3], q: [f32; 3], radius: f32) -> Option<(usize, f32)> {
        let (mut t, _) = self.triangle_under(p)?;
        let (mut steps, mut preferred) = (0u32, 0u32);
        let mut entered = -1.0f32;
        while steps < 1000 {
            steps += 1;
            if self.triangles[t].flags & PREFERRED != 0 {
                preferred += 1;
            }
            let [a, b, c] = [0, 1, 2].map(|i| self.corner(t, i));
            if height_in(a, b, c, q).is_some() {
                return Some((t, preferred as f32 / steps as f32));
            }
            let mut exit: Option<(f32, usize)> = None;
            for e in 0..3 {
                let (u, v) = (self.corner(t, e), self.corner(t, e + 1));
                if let Some(s) = super::crossing(p, q, u, v) {
                    if s > entered + 1e-5 && exit.map_or(true, |(x, _)| s > x) {
                        exit = Some((s, e));
                    }
                }
            }
            let (s, e) = exit?;
            if self.triangles[t].closed & (1 << e) != 0 {
                return None;
            }
            let (n, _) = self.across(t, e)?;
            if radius > LARGE_ACTOR_RADIUS && self.triangles[n].flags & NO_LARGE_CREATURES != 0 {
                return None;
            }
            entered = s;
            t = n;
        }
        None
    }

    /// Whether the step from `a` to `b` is clear with room for the walker
    /// (`006b25f0`): the line itself, and lines 0.9 × the radius to either
    /// side (from `a` out to its side point, then on to `b`'s), the width
    /// 0.9 × the radius, or of the step's length when shorter; at the start
    /// the side points are half the width across and half on along the
    /// way, at the goal half across and half back. The share of preferred
    /// triangles on the line itself.
    // Translated from 006b25f0 (decompiled, FalloutNV.exe 1.4.0.525).
    fn clear(
        &self,
        a: [f32; 3],
        b: [f32; 3],
        a_end: bool,
        b_end: bool,
        radius: f32,
    ) -> Option<f32> {
        let (_, share) = self.walk_line(a, b, radius)?;
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let d = dx.hypot(dy);
        if d < 1e-4 {
            return Some(share);
        }
        // The start or goal off the navmesh or on its edge (someone standing
        // just off it, a marker beside it): the game first builds a way from
        // there onto the navmesh with ray casts (`PathBuilder::
        // BuildPathToNavMeshSearchStart`/`BuildPathFromNavMeshSearchGoal`
        // (Xbox PDB), not traced); here such an end needs only the line
        // itself, as its side points would always leave the navmesh.
        if (a_end && !self.well_inside(a)) || (b_end && !self.well_inside(b)) {
            return Some(share);
        }
        let (ux, uy) = (dx / d, dy / d);
        // Across the way (the way × up).
        let (px, py) = (uy, -ux);
        let width = SIDE_SHARE * if radius * radius <= d * d { radius } else { d };
        let full = SIDE_SHARE * radius;
        // A side point: across by `side`, and along the way by `along`.
        let at = |p: [f32; 3], side: f32, along: f32| {
            [
                p[0] + px * side + ux * along,
                p[1] + py * side + uy * along,
                p[2],
            ]
        };
        for sign in [1.0f32, -1.0] {
            // The start's side points are half the width across and half
            // on along the way; the goal's half across and half back;
            // others the full 0.9 × the radius across.
            let sa = if a_end {
                at(a, sign * width * 0.5, width * 0.5)
            } else {
                at(a, sign * full, 0.0)
            };
            let sb = if b_end {
                at(b, sign * width * 0.5, -width * 0.5)
            } else {
                at(b, sign * full, 0.0)
            };
            self.walk_line(a, sa, radius)?;
            self.walk_line(sa, sb, radius)?;
        }
        Some(share)
    }
}

/// The smoothed path from `from` to `to` through the corridor's triangles
/// (`006ad770`): whether the search reached the goal, the points to walk
/// through (from `from`, ending at `to` when it did), and when it didn't
/// the corridor's edges to keep off on the next try.
// Translated from 006ad770, 006adcb0, 006ae370, 006aeba0, 006aefc0, 006af2b0, 006b0ad0, 006b0f40, 006bb570, 006b2c00 and 006afcc0 (decompiled, FalloutNV.exe 1.4.0.525).
pub(super) fn smooth(
    mesh: &NavMesh,
    from: [f32; 3],
    to: [f32; 3],
    corridor: &[usize],
    request: &PathRequest,
) -> (bool, Vec<[f32; 3]>, Vec<(usize, usize)>) {
    let r = request.radius;
    let corner_radius = r * CORNER_SCALE;
    // The sides: each shared edge's ends, left and right of the way, with
    // the edge (place in the corridor) where each first comes.
    let mut left: Vec<([f32; 3], usize)> = Vec::new();
    let mut right: Vec<([f32; 3], usize)> = Vec::new();
    for (i, w) in corridor.windows(2).enumerate() {
        let Some((l, rt)) = mesh.portal(w[0], w[1]) else {
            continue;
        };
        if left.last().map_or(true, |(p, _)| *p != l) {
            left.push((l, i));
        }
        if right.last().map_or(true, |(p, _)| *p != rt) {
            right.push((rt, i));
        }
    }
    let mut points = vec![
        Point {
            kind: Kind::Start,
            at: from,
            radius: 0.0,
            cost: 1.0,
            own: 0,
            other: 0,
            crossing: 0,
        },
        Point {
            kind: Kind::Goal,
            at: to,
            radius: 0.0,
            cost: 1.0,
            own: 0,
            other: 0,
            crossing: 0,
        },
    ];
    // Corners where a side bends into the corridor (and each side's first
    // and last).
    let corners = |side: &[([f32; 3], usize)], is_left: bool| -> Vec<(usize, Point)> {
        let mut out = Vec::new();
        for i in 0..side.len() {
            let v = side[i].0;
            let keep = if i == 0 || i + 1 == side.len() {
                true
            } else {
                let unit = |p: [f32; 3], q: [f32; 3]| {
                    let (x, y) = (q[0] - p[0], q[1] - p[1]);
                    let l = x.hypot(y);
                    if l > 1e-6 {
                        (x / l, y / l, true)
                    } else {
                        (0.0, 0.0, false)
                    }
                };
                let (ix, iy, ok) = unit(side[i - 1].0, v);
                let (ox, oy, _) = unit(v, side[i + 1].0);
                // Turning left (counterclockwise) is positive.
                let turn = ix * oy - iy * ox;
                let bend = if is_left { -turn } else { turn };
                if ok {
                    bend < TURN_SLACK
                } else {
                    bend < -TURN_SLACK
                }
            };
            if keep {
                out.push((
                    side[i].1,
                    Point {
                        kind: if is_left { Kind::Left } else { Kind::Right },
                        at: v,
                        radius: corner_radius,
                        cost: 1.0,
                        own: 0,
                        other: 0,
                        crossing: side[i].1,
                    },
                ));
            }
        }
        out
    };
    let left_corners = corners(&left, true);
    let right_corners = corners(&right, false);
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for (k, (_, p)) in left_corners.iter().enumerate() {
        left_list.push(points.len());
        points.push(Point { own: k, ..*p });
    }
    for (k, (_, p)) in right_corners.iter().enumerate() {
        right_list.push(points.len());
        points.push(Point { own: k, ..*p });
    }
    // The first corner of the other side at or after each one's place.
    for (k, (edge, _)) in left_corners.iter().enumerate() {
        let other = right_corners
            .iter()
            .position(|(e, _)| e >= edge)
            .unwrap_or(right_corners.len());
        points[left_list[k]].other = other;
    }
    for (k, (edge, _)) in right_corners.iter().enumerate() {
        let other = left_corners
            .iter()
            .position(|(e, _)| e >= edge)
            .unwrap_or(left_corners.len());
        points[right_list[k]].other = other;
    }
    // Obstacles: avoid nodes and the navmeshes' marked obstacles.
    let mut obstacles = Vec::new();
    for n in request.avoid {
        obstacles.push(points.len());
        points.push(Point {
            kind: Kind::Obstacle,
            at: n.position,
            radius: n.radius + corner_radius,
            cost: 1.0,
            own: 0,
            other: 0,
            crossing: 0,
        });
    }
    let navmeshes: Vec<_> = corridor.iter().filter_map(|&t| mesh.owner_of(t)).collect();
    for o in &mesh.obstacles {
        if navmeshes.is_empty()
            || mesh
                .owner_of(o.triangle)
                .is_some_and(|n| navmeshes.contains(&n))
        {
            obstacles.push(points.len());
            points.push(Point {
                kind: Kind::Obstacle,
                at: o.position,
                radius: o.radius + corner_radius,
                cost: 1.0,
                own: 0,
                other: 0,
                crossing: 0,
            });
        }
    }
    let (found, path, last) = search(mesh, &points, &left_list, &right_list, &obstacles, r);
    // Failed: the edges to keep off next time (`006ad770` after
    // `006b1970`): the corridor's shared edges narrower than twice the
    // radius (`006b2fc0`: length² < 4 r², `0101db80`), and the shared edge
    // of the corner the path got to, when it got to one.
    let mut keep_off = Vec::new();
    if !found {
        let edge_of = |i: usize| -> Option<(usize, usize)> {
            let (t, n) = (*corridor.get(i)?, *corridor.get(i + 1)?);
            let e = mesh.triangles[t]
                .neighbors
                .iter()
                .position(|&m| m == Some(n))?;
            Some((t, e))
        };
        for i in 0..corridor.len().saturating_sub(1) {
            if let Some((t, e)) = edge_of(i) {
                let (a, b) = mesh.edge_ends(t, e);
                let l2: f32 = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum();
                if l2 < r * r * 4.0 {
                    keep_off.push((t, e));
                }
            }
        }
        let p = points[last];
        if matches!(p.kind, Kind::Left | Kind::Right) {
            if let Some(edge) = edge_of(p.crossing) {
                if !keep_off.contains(&edge) {
                    keep_off.push(edge);
                }
            }
        }
    }
    (found, path, keep_off)
}

/// A search node: a point and the way round it, how it was reached.
#[derive(Clone, Copy)]
struct Node {
    point: usize,
    ccw: Ccw,
    g: f32,
    h: f32,
    parent: Option<usize>,
    /// Where the step to it reached its circle, and where that step left
    /// the parent's.
    arrived: [f32; 3],
    left_parent: [f32; 3],
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
        // Least f first; among equals the newest.
        o.f.total_cmp(&self.f).then(self.seq.cmp(&o.seq))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

/// A step found: the way round the point reached, the cost so far, and the
/// tangent points leaving and reaching.
type Found = (Ccw, f32, [f32; 3], [f32; 3]);

/// The search over the points (`006b3380` with `006b0ad0`/`006b0f40`).
fn search(
    mesh: &NavMesh,
    points: &[Point],
    left: &[usize],
    right: &[usize],
    obstacles: &[usize],
    r: f32,
) -> (bool, Vec<[f32; 3]>, usize) {
    let goal_at = points[1].at;
    let mut nodes = vec![Node {
        point: 0,
        ccw: false,
        g: 0.0,
        h: length(points[0].at, goal_at) * 0.75,
        parent: None,
        arrived: points[0].at,
        left_parent: points[0].at,
        version: 0,
        open: false,
    }];
    let mut node_of: HashMap<(usize, Ccw), usize> = HashMap::from([((0, false), 0)]);
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
    let mut current = pop(&mut open, &mut nodes).expect("pushed");
    let mut best = current;
    let found = loop {
        let a = points[nodes[current].point];
        if a.kind == Kind::Goal {
            best = current;
            break true;
        }
        // What to try from here, run by run (`006b0ad0`): the goal; from a
        // corner its own side after it and the other side from its place,
        // from the start both sides, each until four have failed; from an
        // obstacle both sides whole; then every obstacle.
        let mut runs: Vec<(Vec<usize>, bool)> = vec![(vec![1], false)];
        match a.kind {
            Kind::Left | Kind::Right => {
                let (own, other) = if a.kind == Kind::Left {
                    (left, right)
                } else {
                    (right, left)
                };
                runs.push((own.iter().skip(a.own + 1).copied().collect(), true));
                runs.push((other.iter().skip(a.other).copied().collect(), true));
            }
            Kind::Start => {
                runs.push((left.to_vec(), true));
                runs.push((right.to_vec(), true));
            }
            _ => {
                runs.push((left.to_vec(), false));
                runs.push((right.to_vec(), false));
            }
        }
        runs.push((obstacles.to_vec(), false));
        let mut improved = Vec::new();
        for (run, limited) in runs {
            let mut failures = 0usize;
            for b_index in run {
                let (ok, found) = step(mesh, points, &nodes, current, b_index, r);
                for (b_ccw, cost, pa, pb) in found {
                    let key = (b_index, b_ccw);
                    match node_of.get(&key) {
                        None => {
                            let k = nodes.len();
                            nodes.push(Node {
                                point: b_index,
                                ccw: b_ccw,
                                g: cost,
                                h: length(points[b_index].at, goal_at) * 0.1,
                                parent: None,
                                arrived: pb,
                                left_parent: pa,
                                version: 0,
                                open: false,
                            });
                            node_of.insert(key, k);
                            improved.push((k, pa, pb));
                        }
                        Some(&k) if cost < nodes[k].g => {
                            nodes[k].g = cost;
                            improved.push((k, pa, pb));
                        }
                        Some(_) => {}
                    }
                }
                if !ok {
                    failures += 1;
                    if limited && failures == FAILURES {
                        break;
                    }
                }
            }
        }
        let mut failed = false;
        for (k, pa, pb) in improved {
            nodes[k].open = false;
            if nodes[current].parent == Some(k) {
                failed = true;
                break;
            }
            nodes[k].parent = Some(current);
            nodes[k].arrived = pb;
            nodes[k].left_parent = pa;
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
    // The way back, as tangent points (`006afcc0`).
    let mut chain = Vec::new();
    let mut at = Some(best);
    while let Some(k) = at {
        chain.push(k);
        at = nodes[k].parent;
        if chain.len() > nodes.len() {
            break;
        }
    }
    chain.reverse();
    let mut path = vec![points[0].at];
    for w in chain.windows(2) {
        let (prev, next) = (nodes[w[0]], nodes[w[1]]);
        let p = points[prev.point];
        // Leaving the previous point: left out after a short arc round it.
        let short = prev.parent.is_some()
            && p.radius * arc_angle(p.at, prev.arrived, next.left_parent, prev.ccw) < SHORT_ARC;
        if !short {
            push_new(&mut path, next.left_parent);
        }
        push_new(&mut path, next.arrived);
    }
    if !found {
        // `006b1970`: from the nearest point reached straight on to the
        // goal, when that's clear.
        let last = *path.last().expect("start");
        if mesh.clear(last, goal_at, false, true, r).is_some() {
            push_new(&mut path, goal_at);
        }
    }
    (found, path, nodes[best].point)
}

fn push_new(path: &mut Vec<[f32; 3]>, p: [f32; 3]) {
    if path.last().map_or(true, |q| length(*q, p) > 1e-3) {
        path.push(p);
    }
}

/// One step from the search node `current` to point `b` (`006b0f40`): the
/// steps found (one for each tangent taken), and whether it didn't fail
/// (no tangent was tried, or not all that were tried failed).
fn step(
    mesh: &NavMesh,
    points: &[Point],
    nodes: &[Node],
    current: usize,
    b_index: usize,
    r: f32,
) -> (bool, Vec<Found>) {
    let here = nodes[current];
    let a = points[here.point];
    let b = points[b_index];
    let mut out = Vec::new();
    if b_index == here.point || here.parent.is_some_and(|p| nodes[p].point == b_index) {
        return (true, out);
    }
    let dz = b.at[2] - a.at[2];
    let flat2 = (b.at[0] - a.at[0]).powi(2) + (b.at[1] - a.at[1]).powi(2);
    if dz * dz > flat2 * CORNER_SCALE * CORNER_SCALE {
        return (true, out);
    }
    let ways: &[Ccw] = match b.kind {
        Kind::Left => &[true],
        Kind::Right => &[false],
        Kind::Start | Kind::Goal => {
            if here.ccw {
                &[true]
            } else {
                &[false]
            }
        }
        Kind::Obstacle => &[true, false],
    };
    let (mut tried, mut failed) = (0usize, 0usize);
    for &b_ccw in ways {
        // The start within the circle: the point itself (the game cuts the
        // circle with a chord there, not followed).
        let (pa, pb) = if a.kind == Kind::Start && flat2.sqrt() <= b.radius {
            (a.at, a.at)
        } else {
            match tangent(&a, here.ccw, &b, b_ccw) {
                Some(t) => t,
                None => continue,
            }
        };
        // The arc round a corner before the step.
        let arc = if here.parent.is_some() && a.radius > 0.0 {
            let angle = arc_angle(a.at, here.arrived, pa, here.ccw);
            if matches!(a.kind, Kind::Left | Kind::Right) && angle > ARC_LIMIT {
                continue;
            }
            angle * a.radius
        } else {
            0.0
        };
        tried += 1;
        let Some(share) = mesh.clear(pa, pb, a.kind == Kind::Start, b.kind == Kind::Goal, r) else {
            failed += 1;
            continue;
        };
        let height_ok = mesh
            .triangle_under(pb)
            .is_some_and(|(_, z)| (HEIGHT_BELOW..HEIGHT_ABOVE).contains(&(z - pb[2])));
        if !height_ok {
            failed += 1;
            continue;
        }
        // The line's length and its extra through obstacles (`006b2c00`).
        let mut along = length(pa, pb);
        for p in points.iter().filter(|p| p.kind == Kind::Obstacle) {
            if let Some((chord, inside)) = chord_through(p, pa, pb) {
                along += (inside * AVOID_NODE_COST + 1.0) * chord;
            }
        }
        let preferred = (share * SIDE_SHARE).clamp(0.0, 1.0);
        let cost = here.g + (1.0 - preferred) * (arc + along) * b.cost;
        out.push((b_ccw, cost, pa, pb));
    }
    (tried == 0 || failed < tried, out)
}

/// The part of the segment `p`–`q` inside an obstacle's circle (seen from
/// above): its length, and how deep the segment passes, (r − its distance
/// from the centre) ÷ r.
fn chord_through(o: &Point, p: [f32; 3], q: [f32; 3]) -> Option<(f32, f32)> {
    let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
    let len2 = dx * dx + dy * dy;
    if len2 < 1e-9 || o.radius <= 0.0 {
        return None;
    }
    let (fx, fy) = (p[0] - o.at[0], p[1] - o.at[1]);
    let b = fx * dx + fy * dy;
    let c = fx * fx + fy * fy - o.radius * o.radius;
    let disc = b * b - len2 * c;
    if disc <= 0.0 {
        return None;
    }
    let s = disc.sqrt();
    let t0 = ((-b - s) / len2).max(0.0);
    let t1 = ((-b + s) / len2).min(1.0);
    if t1 <= t0 {
        return None;
    }
    let chord = (t1 - t0) * len2.sqrt();
    let t = (-b / len2).clamp(0.0, 1.0);
    let closest = ((fx + dx * t).powi(2) + (fy + dy * t).powi(2)).sqrt();
    Some((chord, ((o.radius - closest) / o.radius).max(0.0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::NavTriangle;
    use crate::movement::AvoidNode;

    /// A grid of `nx` × `ny` squares of `size`, each two triangles, joined
    /// to their neighbours.
    fn grid(nx: usize, ny: usize, size: f32) -> NavMesh {
        let v = |x: usize, y: usize| y * (nx + 1) + x;
        let mut vertices = Vec::new();
        for y in 0..=ny {
            for x in 0..=nx {
                vertices.push([x as f32 * size, y as f32 * size, 0.0]);
            }
        }
        // Square (x, y): triangle 2i = (sw, se, ne), 2i + 1 = (sw, ne, nw).
        let sq = |x: usize, y: usize| 2 * (y * nx + x);
        let mut triangles = Vec::new();
        for y in 0..ny {
            for x in 0..nx {
                let (sw, se, ne, nw) = (v(x, y), v(x + 1, y), v(x + 1, y + 1), v(x, y + 1));
                let south = (y > 0).then(|| sq(x, y - 1) + 1);
                let east = (x + 1 < nx).then(|| sq(x + 1, y) + 1);
                triangles.push(NavTriangle {
                    vertices: [sw, se, ne],
                    neighbors: [south, east, Some(sq(x, y) + 1)],
                    ..Default::default()
                });
                let north = (y + 1 < ny).then(|| sq(x, y + 1));
                let west = (x > 0).then(|| sq(x - 1, y));
                triangles.push(NavTriangle {
                    vertices: [sw, ne, nw],
                    neighbors: [Some(sq(x, y)), north, west],
                    ..Default::default()
                });
            }
        }
        NavMesh {
            vertices,
            triangles,
            ..Default::default()
        }
    }

    #[test]
    fn tangents_touch_both_circles_and_go_round_them_the_asked_way() {
        let a = Point {
            kind: Kind::Start,
            at: [0.0, 0.0, 0.0],
            radius: 0.0,
            cost: 1.0,
            own: 0,
            other: 0,
            crossing: 0,
        };
        let b = Point {
            kind: Kind::Left,
            at: [100.0, 0.0, 0.0],
            radius: 42.0,
            cost: 1.0,
            own: 0,
            other: 0,
            crossing: 0,
        };
        // Round b counterclockwise: it stays on the left, the tangent point
        // below its centre.
        let (pa, pb) = tangent(&a, false, &b, true).unwrap();
        assert_eq!(pa, a.at);
        let to_centre = (pb[0] - 100.0).hypot(pb[1]);
        assert!((to_centre - 42.0).abs() < 1e-3);
        assert!(pb[1] < 0.0, "{pb:?}");
        // The line is at right angles to the radius there.
        let dot = (pb[0] - pa[0]) * (pb[0] - 100.0) + (pb[1] - pa[1]) * pb[1];
        assert!(dot.abs() < 1e-2, "{dot}");
        // Clockwise: above.
        let (_, pb) = tangent(&a, false, &b, false).unwrap();
        assert!(pb[1] > 0.0);
        // Between two circles the inner tangents need them apart.
        let c = Point {
            at: [70.0, 0.0, 0.0],
            kind: Kind::Right,
            ..b
        };
        let near = Point {
            at: [0.0, 0.0, 0.0],
            ..b
        };
        assert!(tangent(&near, true, &c, false).is_none());
        assert!(tangent(&near, true, &c, true).is_some());
        // Arcs: a quarter round counterclockwise, three quarters the other.
        let q = arc_angle([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], true);
        assert!((q - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        let q = arc_angle([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], false);
        assert!((q - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn an_avoid_node_in_the_way_is_gone_round_at_its_radius_and_the_corner_radius() {
        let mesh = grid(10, 4, 100.0);
        let node = AvoidNode {
            position: [500.0, 200.0, 0.0],
            radius: 50.0,
            cost: 2.0,
        };
        let request = PathRequest {
            avoid: &[node],
            ..Default::default()
        };
        let (path, _) = mesh
            .plan([100.0, 200.0, 0.0], [900.0, 200.0, 0.0], &request)
            .unwrap();
        assert_eq!(path.first(), Some(&[100.0, 200.0, 0.0]));
        assert_eq!(path.last(), Some(&[900.0, 200.0, 0.0]));
        // Every point keeps 50 + 1.2 × 35 from the node.
        let keep = 50.0 + 1.2 * 35.0;
        for p in &path[1..path.len() - 1] {
            let d = (p[0] - 500.0).hypot(p[1] - 200.0);
            assert!((d - keep).abs() < 1e-2, "{path:?}");
        }
        assert!(path.len() >= 3, "{path:?}");
        // Without it: straight.
        let straight = mesh.path([100.0, 200.0, 0.0], [900.0, 200.0, 0.0]).unwrap();
        assert_eq!(straight.len(), 2);
    }

    #[test]
    fn a_corridor_without_room_for_the_walker_is_not_smoothed_through() {
        // A strip 100 wide has room for the side lines; one 25 wide hasn't
        // even for the start and goal's (half of 0.9 × 35 either side).
        for (width, room) in [(100.0, true), (25.0, false)] {
            let mesh = {
                let mut m = grid(6, 1, width);
                // Stretch the strip: squares `width` across, 100 long.
                for v in &mut m.vertices {
                    v[0] = v[0] / width * 100.0;
                }
                m
            };
            let from = [50.0, width / 2.0, 0.0];
            let to = [550.0, width / 2.0, 0.0];
            let corridor: Vec<usize> = {
                let (found, route) = mesh.search(
                    mesh.triangle_at(from).unwrap(),
                    mesh.triangle_at(to).unwrap(),
                    &PathRequest::default(),
                );
                assert!(found);
                route
            };
            let (found, ..) = smooth(&mesh, from, to, &corridor, &PathRequest::default());
            assert_eq!(found, room, "{width}");
        }
    }
}
