//! The long way: the navmesh info map (`NAVI`) and the high-level route
//! over it, as the game plans a path whose goal lies on a navmesh that isn't
//! loaded (`Pathfind.cpp`, `NavMeshInfoMap.cpp`; FalloutNV.exe 1.4.0.525).
//!
//! The game keeps one `NavMeshInfo` (Xbox PDB) per navmesh in memory at all
//! times, read from the `NAVI` record (`NavMeshInfoMap::Load` (Xbox PDB),
//! `006b5d80`): `NVMI` (`006b4b50`) gives the navmesh, its place (a
//! worldspace and grid square, or an interior cell), a rough position and a
//! "preferred" factor; `NVCI` (`006b51a0`) the navmeshes it joins (two
//! lists) and its doors. A path request between two navmeshes first
//! searches this map (`NavMeshInfoSearch`, an `AStarSearch` (Xbox PDB):
//! `006b8c50`, `006b9180`, edge costs `006b8490`) and turns the route into
//! "virtual" path nodes: the start, each navmesh passed (at its rough
//! position), the goal (`006c94c0`). Only the run of nodes from the
//! actor's on whose cells are attached gets a detailed navmesh path
//! (`006c9fc0`, `006ca0e0`); someone out of sight (not the high process)
//! walks the virtual nodes point to point (`VirtualActorPathHandler`
//! (Xbox PDB), `009ea8a0`).
//!
//! Not done (labelled where it matters): the search's door edges (third
//! list, `006b8490`: used for an actor's requests, 409600 extra for a
//! locked door; places are changed through `super::door_toward` instead),
//! the "island" data of flag 0x20 entries, and what a failed search falls
//! back to (`006c8f50` → `006c9b20`): here a failed search gives no path.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use esm::{FormId, FourCC, LoadOrder};

use super::NavMesh;
use crate::cell::{le_f32, le_u32};
use crate::movement::PolyWalk;

const NAVI: FourCC = FourCC::new(b"NAVI");
const NVER: FourCC = FourCC::new(b"NVER");
const NVMI: FourCC = FourCC::new(b"NVMI");
const NVCI: FourCC = FourCC::new(b"NVCI");

/// One navmesh's entry in the map (`NavMeshInfo` (Xbox PDB), 0x5c bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct NavInfo {
    /// The navmesh (`NAVM`), +0.
    pub navmesh: FormId,
    /// Its place (`NVMI` +8): a worldspace, or an interior cell. +4.
    pub space: FormId,
    /// `NVMI` flags, +8 (0x20: island data follows the position).
    pub flags: u32,
    /// Its grid square (two i16), +0xc; outdoors only.
    pub square: Option<(i32, i32)>,
    /// Its rough position, +0x10: the virtual path node for it.
    pub position: [f32; 3],
    /// The preferred factor (`NVMI`'s last float, NVER > 9), +0x20: edges
    /// leaving it cost × clamp(1 − 100 × this, 0.1, 1) (`006b8490`).
    pub preferred: f32,
    /// `NVCI`'s first list, +0x24: navmeshes it joins; crossing to one
    /// costs three times the distance (`006b8490`).
    pub links: Vec<FormId>,
    /// `NVCI`'s second list (NVER > 10), +0x34: crossing costs the
    /// distance.
    pub cheap_links: Vec<FormId>,
    /// `NVCI`'s third list, +0x44: its doors (not searched here).
    pub doors: Vec<FormId>,
}

/// Every navmesh's entry, by navmesh and by place.
#[derive(Debug, Clone, Default)]
pub struct NavInfoMap {
    pub infos: Vec<NavInfo>,
    by_navmesh: HashMap<FormId, usize>,
    /// By interior cell (`None`) or worldspace and square.
    by_place: HashMap<Place, Vec<usize>>,
}

/// An interior cell (no square), or a worldspace and grid square.
type Place = (FormId, Option<(i32, i32)>);

/// An `NVMI` entry (`006b4b50`): flags, navmesh, place, grid (i16, i16),
/// position; then, with flag 0x20, island data; then (NVER > 9) the
/// preferred factor.
// Translated from 006b4b50 (decompiled, FalloutNV.exe 1.4.0.525).
fn read_nvmi(
    data: &[u8],
    version: u32,
    global: &dyn Fn(u32) -> FormId,
    exterior: &dyn Fn(FormId) -> bool,
) -> Option<NavInfo> {
    if data.len() < 28 {
        return None;
    }
    let flags = le_u32(data, 0);
    let navmesh = global(le_u32(data, 4));
    let space = global(le_u32(data, 8));
    // The grid word holds y, then x: WastelandNV's navmesh 001639F7 (cell
    // 000DEBB6, `XCLC` 1, −17; vertices near x 4096..8192, y −69632)
    // reads −17, 1.
    let gy = i16::from_le_bytes([data[12], data[13]]);
    let gx = i16::from_le_bytes([data[14], data[15]]);
    let position = [le_f32(data, 16), le_f32(data, 20), le_f32(data, 24)];
    // The island data's length is the loader's (`0069c8f0`); the factor is
    // the entry's last float.
    let preferred = if version > 9 && data.len() >= 32 {
        le_f32(data, data.len() - 4)
    } else {
        0.0
    };
    Some(NavInfo {
        navmesh,
        space,
        flags,
        square: exterior(space).then_some((i32::from(gx), i32::from(gy))),
        position,
        preferred,
        links: Vec::new(),
        cheap_links: Vec::new(),
        doors: Vec::new(),
    })
}

/// An `NVCI` entry's three lists (`006b51a0`), after its navmesh: each a
/// count and form IDs; the second only for NVER > 10.
// Translated from 006b51a0 (decompiled, FalloutNV.exe 1.4.0.525).
#[allow(clippy::type_complexity)]
fn read_nvci(
    data: &[u8],
    version: u32,
    global: &dyn Fn(u32) -> FormId,
) -> Option<(FormId, Vec<FormId>, Vec<FormId>, Vec<FormId>)> {
    let mut at = 0usize;
    let word = |at: &mut usize| -> Option<u32> {
        let v = data.get(*at..*at + 4)?;
        *at += 4;
        Some(u32::from_le_bytes([v[0], v[1], v[2], v[3]]))
    };
    let navmesh = global(word(&mut at)?);
    let list = |at: &mut usize| -> Option<Vec<FormId>> {
        let n = word(at)?;
        (0..n).map(|_| word(at).map(global)).collect()
    };
    let links = list(&mut at)?;
    let cheap = if version > 10 {
        list(&mut at)?
    } else {
        Vec::new()
    };
    let doors = list(&mut at)?;
    Some((navmesh, links, cheap, doors))
}

impl NavInfoMap {
    /// Every `NAVI` record's entries, every plugin's version of it in load
    /// order: a plugin's version holds only its own navmeshes' entries
    /// (the DLCs override `FalloutNV.esm`'s 00014B92 with a handful), and
    /// the loader adds each version's entries to the one map (`006b5d80`
    /// runs per record read); a later entry for a navmesh replaces an
    /// earlier one.
    pub fn load(order: &LoadOrder) -> NavInfoMap {
        let mut map = NavInfoMap::default();
        let ids: Vec<FormId> = order.records_of_type(NAVI).map(|rr| rr.form_id).collect();
        for rr in ids.into_iter().flat_map(|id| order.versions(id)) {
            if rr.entry.header.is_deleted() {
                continue;
            }
            let Ok(record) = rr.record() else { continue };
            let global = |raw: u32| rr.plugin.to_global(FormId(raw));
            let exterior = |space: FormId| {
                order
                    .get(space)
                    .is_some_and(|r| r.entry.header.kind == esm::sig::WRLD)
            };
            // `NVER` (the loader's default 1 when absent).
            let version = record.get(NVER).map_or(1, |s| le_u32(&s.data, 0));
            for sub in &record.subrecords {
                if sub.kind == NVMI {
                    if let Some(info) = read_nvmi(&sub.data, version, &global, &exterior) {
                        map.insert(info);
                    }
                } else if sub.kind == NVCI && version > 4 {
                    if let Some((navmesh, links, cheap, doors)) =
                        read_nvci(&sub.data, version, &global)
                    {
                        if let Some(&i) = map.by_navmesh.get(&navmesh) {
                            let info = &mut map.infos[i];
                            info.links = links;
                            info.cheap_links = cheap;
                            info.doors = doors;
                        }
                    }
                }
            }
        }
        map
    }

    fn insert(&mut self, info: NavInfo) {
        let key = (info.space, info.square);
        if let Some(&i) = self.by_navmesh.get(&info.navmesh) {
            let old = (self.infos[i].space, self.infos[i].square);
            if let Some(list) = self.by_place.get_mut(&old) {
                list.retain(|&j| j != i);
            }
            self.infos[i] = info;
            self.by_place.entry(key).or_default().push(i);
            return;
        }
        let i = self.infos.len();
        self.by_navmesh.insert(info.navmesh, i);
        self.infos.push(info);
        self.by_place.entry(key).or_default().push(i);
    }

    /// The entry of a navmesh.
    pub fn of_navmesh(&self, navmesh: FormId) -> Option<usize> {
        self.by_navmesh.get(&navmesh).copied()
    }

    /// The entries of an interior cell (`square` `None`) or a worldspace's
    /// grid square.
    pub fn in_place(&self, space: FormId, square: Option<(i32, i32)>) -> &[usize] {
        self.by_place
            .get(&(space, square))
            .map_or(&[], Vec::as_slice)
    }

    /// Whether two entries are outdoors in the same worldspace
    /// (`00690800` gives an entry's worldspace, none indoors).
    fn same_world(&self, a: usize, b: usize) -> bool {
        let (a, b) = (&self.infos[a], &self.infos[b]);
        a.square.is_some() && b.square.is_some() && a.space == b.space
    }

    /// The route from navmesh `start` (the walker at `from`) to navmesh
    /// `goal` (the goal at `to`): the entries passed, start and goal
    /// included, and whether the goal was reached. A search that fails
    /// gives the route to the entry it got nearest to (least estimate;
    /// `006b8f40` called with "closest" set by `006b8c50`).
    ///
    /// A*: a node's cost so far `g`, its estimate `h` = 0.1 × the distance
    /// from its rough position to `to` when both ends are outdoors in one
    /// worldspace, else 1; the open list ordered by g + h (`006b9710`; its
    /// buckets, `006f4790`, put every node with h ≥ 0 in one sorted list;
    /// a new node goes before those with the same g + h). Leaving entry `n`
    /// (at `from` for the start) for a joined entry `m` (at `to` for the
    /// goal) costs the distance × k (× 3 for `links`), k = clamp(1 − 100 ×
    /// n's preferred factor, 0.1, 1); an entry reached more cheaply is
    /// taken up again. A neighbour improved through the node it was reached
    /// from ends the search as failed (`006b9180`'s parent test).
    // Translated from 006b8c50, 006b9180 and 006b8490 (decompiled, FalloutNV.exe 1.4.0.525).
    pub fn search(
        &self,
        start: usize,
        from: [f32; 3],
        goal: usize,
        to: [f32; 3],
    ) -> (bool, Vec<usize>) {
        #[derive(Clone, Copy)]
        struct Node {
            info: usize,
            g: f32,
            h: f32,
            parent: Option<usize>,
            /// Open-list entry version (stale heap entries are skipped).
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
        let same_world = self.same_world(start, goal);
        let estimate = |info: usize| {
            if same_world {
                distance(self.infos[info].position, to) * 0.1
            } else {
                1.0
            }
        };
        let place_of = |info: usize, is_from: bool| {
            if is_from && info == start {
                from
            } else if !is_from && info == goal {
                to
            } else {
                self.infos[info].position
            }
        };
        let mut nodes: Vec<Node> = Vec::new();
        let mut node_of: HashMap<usize, usize> = HashMap::new();
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
        // The start node: cost 0, its estimate as above (`006b8ec0`).
        let h0 = if same_world {
            distance(self.infos[start].position, to) * 0.1
        } else {
            1.0
        };
        nodes.push(Node {
            info: start,
            g: 0.0,
            h: h0,
            parent: None,
            version: 0,
            open: false,
        });
        node_of.insert(start, 0);
        push(&mut open, &mut nodes, 0, &mut seq);
        let pop = |open: &mut BinaryHeap<Entry>, nodes: &mut [Node]| loop {
            let e = open.pop()?;
            if nodes[e.node].open && nodes[e.node].version == e.version {
                nodes[e.node].open = false;
                return Some(e.node);
            }
        };
        let Some(mut current) = pop(&mut open, &mut nodes) else {
            return (false, Vec::new());
        };
        let mut best = current;
        let found = loop {
            if nodes[current].info == goal {
                best = current;
                break true;
            }
            // The connections (`006b8490`): new nodes and cheaper ways.
            let here = nodes[current].info;
            let g = nodes[current].g;
            let k = (1.0 - self.infos[here].preferred * 100.0).clamp(0.1, 1.0);
            let a = place_of(here, true);
            let mut improved = Vec::new();
            let lists = [
                (&self.infos[here].links, 3.0f32),
                (&self.infos[here].cheap_links, 1.0),
            ];
            for (list, mult) in lists {
                for &navmesh in list.iter() {
                    let Some(m) = self.of_navmesh(navmesh) else {
                        continue;
                    };
                    let b = place_of(m, false);
                    let cost = distance(a, b) * k * mult + g;
                    match node_of.get(&m) {
                        None => {
                            let n = nodes.len();
                            nodes.push(Node {
                                info: m,
                                g: cost,
                                h: estimate(m),
                                parent: None,
                                version: 0,
                                open: false,
                            });
                            node_of.insert(m, n);
                            improved.push(n);
                        }
                        Some(&n) if cost < nodes[n].g => {
                            nodes[n].g = cost;
                            improved.push(n);
                        }
                        Some(_) => {}
                    }
                }
            }
            let mut failed = false;
            for n in improved {
                nodes[n].open = false;
                if nodes[current].parent == Some(n) {
                    failed = true;
                    break;
                }
                nodes[n].parent = Some(current);
                push(&mut open, &mut nodes, n, &mut seq);
                if nodes[n].h < nodes[best].h {
                    best = n;
                }
            }
            if failed {
                break false;
            }
            match pop(&mut open, &mut nodes) {
                Some(n) => current = n,
                None => break false,
            }
        };
        // The route back from the goal (or the nearest node) (`00996ae0`).
        let mut route = Vec::new();
        let mut at = Some(best);
        while let Some(n) = at {
            route.push(nodes[n].info);
            at = nodes[n].parent;
            if route.len() > nodes.len() {
                break;
            }
        }
        route.reverse();
        (found, route)
    }
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

/// A node of a planned path (`VirtualPathingNode` (Xbox PDB)): where it
/// is, the grid square of its cell outdoors (a navmesh's own square, else
/// the point's) and the navmesh it's on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VirtualNode {
    pub position: [f32; 3],
    pub square: Option<(i32, i32)>,
    pub navmesh: FormId,
}

/// The navmesh info map, loaded on first use, and the navmeshes read to
/// tell which one a point is on.
#[derive(Default)]
pub struct NavInfos {
    map: Option<NavInfoMap>,
    meshes: HashMap<Place, NavMesh>,
}

impl NavInfos {
    pub fn map(&mut self, order: &LoadOrder) -> &NavInfoMap {
        self.map.get_or_insert_with(|| NavInfoMap::load(order))
    }

    fn interior(order: &LoadOrder, space: FormId) -> bool {
        order
            .get(space)
            .is_some_and(|r| r.entry.header.kind == esm::sig::CELL)
    }

    /// The entry of the navmesh a point in `space` (an interior cell or a
    /// worldspace) is on: the only one of its cell (or grid square), else
    /// the one whose triangles hold it (`PathingLocation`'s resolution,
    /// `006dd6f0`, looks through the cell's navmeshes for the triangle),
    /// else the one whose rough position is nearest (unresolved: the
    /// game's handling of a point on none isn't traced).
    pub fn info_at(&mut self, order: &LoadOrder, space: FormId, p: [f32; 3]) -> Option<usize> {
        let square = (!Self::interior(order, space)).then(|| crate::square_of(p));
        let candidates: Vec<usize> = self.map(order).in_place(space, square).to_vec();
        match candidates.len() {
            0 => return None,
            1 => return Some(candidates[0]),
            _ => {}
        }
        let map = self.map.as_ref().expect("loaded above");
        let navmeshes: Vec<FormId> = candidates.iter().map(|&i| map.infos[i].navmesh).collect();
        let mesh = self
            .meshes
            .entry((space, square))
            .or_insert_with(|| NavMesh::load_navmeshes(order, &navmeshes));
        if let Some(i) = mesh.navmesh_at(p).and_then(|n| map.of_navmesh(n)) {
            return Some(i);
        }
        candidates.into_iter().min_by(|&a, &b| {
            distance(map.infos[a].position, p).total_cmp(&distance(map.infos[b].position, p))
        })
    }

    /// The planned path from `from` to `to` in `space` (`006c94c0`): the
    /// start, the rough position of each navmesh the route passes, the
    /// goal; just the two ends when both are on one navmesh. `None` when
    /// either end is on no known navmesh or the search fails.
    pub fn virtual_path(
        &mut self,
        order: &LoadOrder,
        space: FormId,
        from: [f32; 3],
        to: [f32; 3],
    ) -> Option<Vec<VirtualNode>> {
        let start = self.info_at(order, space, from)?;
        let goal = self.info_at(order, space, to)?;
        let outdoors = !Self::interior(order, space);
        let map = self.map(order);
        let end = |p: [f32; 3], info: usize| VirtualNode {
            position: p,
            square: outdoors.then(|| crate::square_of(p)),
            navmesh: map.infos[info].navmesh,
        };
        if start == goal {
            return Some(vec![end(from, start), end(to, goal)]);
        }
        let (found, route) = map.search(start, from, goal, to);
        if !found || route.len() < 2 {
            return None;
        }
        let mut nodes = vec![end(from, start)];
        nodes.extend(route[1..route.len() - 1].iter().map(|&i| VirtualNode {
            position: map.infos[i].position,
            square: map.infos[i].square,
            navmesh: map.infos[i].navmesh,
        }));
        nodes.push(end(to, goal));
        Some(nodes)
    }
}

/// The run of nodes from `first` whose cells are attached (`006c9fc0`):
/// the last node's index, or `None` when `first`'s isn't. (A teleport
/// door node would end the run; routes here have none.) Only this run gets
/// a detailed navmesh path (`006ca0e0`).
// Translated from 006c9fc0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn attached_run(
    nodes: &[VirtualNode],
    first: usize,
    attached: impl Fn(&VirtualNode) -> bool,
) -> Option<usize> {
    if !attached(nodes.get(first)?) {
        return None;
    }
    let mut last = first;
    while last + 1 < nodes.len() && attached(&nodes[last + 1]) {
        last += 1;
    }
    Some(last)
}

/// Someone out of sight walks `distance` along the nodes still ahead
/// (`009ea8a0`): point to point, facing each leg, the nodes reached taken
/// off; partway along the last one, done when within `radius` of the end.
// Translated from 009ea8a0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn walk_nodes(
    nodes: &mut Vec<[f32; 3]>,
    from: [f32; 3],
    heading: f32,
    distance_left: f32,
    radius: f32,
) -> PolyWalk {
    let mut at = from;
    let mut facing = heading;
    let mut left = distance_left;
    while let Some(&next) = nodes.first() {
        let d = distance(at, next);
        if d > 1e-3 {
            facing = crate::movement::heading_to(at, next);
        }
        let last = nodes.len() == 1;
        if d <= left {
            left -= d;
            at = next;
            nodes.remove(0);
            continue;
        }
        let f = if d > 0.0 { left / d } else { 0.0 };
        at = [0, 1, 2].map(|k| at[k] + (next[k] - at[k]) * f);
        return PolyWalk {
            at,
            heading: facing,
            left: 0.0,
            done: last && d - left < radius,
        };
    }
    PolyWalk {
        at,
        heading: facing,
        left,
        done: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(navmesh: u32, x: f32, links: &[u32], cheap: &[u32]) -> NavInfo {
        NavInfo {
            navmesh: FormId(navmesh),
            space: FormId(0x3C),
            flags: 0,
            square: Some(((x / 4096.0).floor() as i32, 0)),
            position: [x, 0.0, 0.0],
            preferred: 0.0,
            links: links.iter().map(|&l| FormId(l)).collect(),
            cheap_links: cheap.iter().map(|&l| FormId(l)).collect(),
            doors: Vec::new(),
        }
    }

    fn map(infos: Vec<NavInfo>) -> NavInfoMap {
        let mut m = NavInfoMap::default();
        for i in infos {
            m.insert(i);
        }
        m
    }

    #[test]
    fn the_first_list_costs_three_times_the_second() {
        // A at 0, C at 1000; B at 500 off to the side (y 400). A–C joined
        // directly by the first list (3 × 1000), A–B–C by the second (2 ×
        // about 640): the search goes round by B.
        let mut b = info(2, 500.0, &[], &[1, 3]);
        b.position[1] = 400.0;
        let m = map(vec![
            info(1, 0.0, &[3], &[2]),
            b,
            info(3, 1000.0, &[1], &[2]),
        ]);
        let (found, route) = m.search(0, [0.0; 3], 2, [1000.0, 0.0, 0.0]);
        assert!(found);
        assert_eq!(route, [0, 1, 2]);
        // With A–C in the second list too, straight there.
        let m = map(vec![
            info(1, 0.0, &[], &[2, 3]),
            info(2, 500.0, &[], &[1, 3]),
            info(3, 1000.0, &[], &[1, 2]),
        ]);
        let (_, route) = m.search(0, [0.0; 3], 2, [1000.0, 0.0, 0.0]);
        assert_eq!(route, [0, 2]);
    }

    #[test]
    fn a_preferred_navmesh_makes_leaving_it_cheaper() {
        // Two ways from A to D, by B or by C, equally long; B is preferred
        // (factor 0.005: k = 1 − 0.5), so its leg costs half.
        let mut b = info(2, 500.0, &[], &[1, 4]);
        b.position[1] = 300.0;
        b.preferred = 0.005;
        let mut c = info(3, 500.0, &[], &[1, 4]);
        c.position[1] = -300.0;
        let m = map(vec![
            info(1, 0.0, &[], &[3, 2]),
            b,
            c,
            info(4, 1000.0, &[], &[2, 3]),
        ]);
        let (found, route) = m.search(0, [0.0; 3], 3, [1000.0, 0.0, 0.0]);
        assert!(found);
        assert_eq!(route, [0, 1, 3]);
    }

    #[test]
    fn a_search_that_fails_gives_the_way_to_the_nearest_entry() {
        // D can't be reached: the route ends at C, nearest to it.
        let m = map(vec![
            info(1, 0.0, &[], &[2]),
            info(2, 400.0, &[], &[1, 3]),
            info(3, 800.0, &[], &[2]),
            info(4, 2000.0, &[], &[]),
        ]);
        let (found, route) = m.search(0, [0.0; 3], 3, [2000.0, 0.0, 0.0]);
        assert!(!found);
        assert_eq!(route, [0, 1, 2]);
    }

    #[test]
    fn entries_read_as_the_loader_reads_them() {
        let global = |raw: u32| FormId(raw);
        let exterior = |space: FormId| space == FormId(0xDA726);
        let mut nvmi = 0x20u32.to_le_bytes().to_vec();
        nvmi.extend(0x1639F7u32.to_le_bytes());
        nvmi.extend(0xDA726u32.to_le_bytes());
        nvmi.extend((-17i16).to_le_bytes());
        nvmi.extend(1i16.to_le_bytes());
        for v in [6070.5f32, -67421.0, 9407.0] {
            nvmi.extend(v.to_le_bytes());
        }
        // Island data (bounds, no vertices or triangles), then the factor.
        nvmi.extend([0u8; 24]);
        nvmi.extend([0u8; 4]);
        nvmi.extend(0.25f32.to_le_bytes());
        let i = read_nvmi(&nvmi, 11, &global, &exterior).unwrap();
        assert_eq!(i.square, Some((1, -17)));
        assert_eq!(i.position, [6070.5, -67421.0, 9407.0]);
        assert_eq!(i.preferred, 0.25);
        assert_eq!(
            read_nvmi(&nvmi, 9, &global, &exterior).unwrap().preferred,
            0.0
        );
        // Three lists from NVER 11; before that no second list.
        let words = |w: &[u32]| w.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
        let (n, a, b, d) = read_nvci(&words(&[9, 2, 10, 11, 1, 12, 1, 13]), 11, &global).unwrap();
        assert_eq!(
            (n, a.len(), b, d),
            (FormId(9), 2, vec![FormId(12)], vec![FormId(13)])
        );
        let (_, a, b, d) = read_nvci(&words(&[9, 1, 10, 1, 13]), 10, &global).unwrap();
        assert_eq!((a, b, d), (vec![FormId(10)], vec![], vec![FormId(13)]));
    }

    #[test]
    fn walking_the_nodes_takes_off_those_reached() {
        let mut nodes = vec![[100.0, 0.0, 0.0], [100.0, 100.0, 0.0]];
        let w = walk_nodes(&mut nodes, [0.0; 3], 0.0, 150.0, 20.0);
        assert_eq!(nodes, [[100.0, 100.0, 0.0]]);
        assert_eq!((w.at, w.done), ([100.0, 50.0, 0.0], false));
        // Within the radius of the end: done.
        let w = walk_nodes(&mut nodes, w.at, w.heading, 40.0, 20.0);
        assert!(w.done && (w.at[1] - 90.0).abs() < 1e-4, "{w:?}");
    }
}
