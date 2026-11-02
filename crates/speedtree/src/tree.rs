//! Growing a tree from its `.spt` and a seed: the game's
//! `CTreeEngine::Compute` (`00b2af80`) and the recursive branch step
//! (`00b11050`), with the leaves the last branch level carries (`00b13100`,
//! `00b13c10`) and the branch rings' vertices (`00b14450`, `00b14ed0`).
//! Each step keeps the game's order of random draws (`random`), so the
//! same file and seed give the same tree, branch for branch.

use crate::math::{self, d, DEG, HALF_PI, IDENTITY, M3, TWO_PI, V3};
use crate::random::Random;
use crate::spt::{BranchLevel, SptFile};

/// What the game sets before growing a tree (`0066ac40`, `00666940`).
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// The seed (`Compute`'s, `00b2c0c0`: 0 a random one, 1 keeps the
    /// file's): the TREE record's `SNAM` entry the reference picks.
    pub seed: i32,
    /// The tree size and its variance (`SetTreeSize`: the file's ×
    /// `fTreeSizeConversion`).
    pub size: f32,
    pub size_variance: f32,
    /// `CNAM` branch dimming (engine leaf info +8).
    pub branch_dimming: f32,
    /// `CNAM` leaf dimming (engine leaf info +4).
    pub leaf_dimming: f32,
    /// `CNAM` least and most leaf (bud) angles.
    pub bud_angles: [f32; 2],
    /// Rocking groups (`SetNumLeafRockingGroups`).
    pub rocking_groups: i32,
    /// The first wind matrix and how many (`SetLocalMatrices(0, 4)`).
    pub wind_matrices: (i32, i32),
    /// How much bigger each leaf level of detail's cards get (CSpeedTreeRT
    /// +0x24, the file's 9009, default 0.5; `Compute` hands it to
    /// `00b2c890`).
    pub leaf_lod_step: f32,
}

impl Settings {
    /// The file's own values, the way SpeedTreeRT would grow it untouched.
    pub fn from_file(spt: &SptFile) -> Settings {
        Settings {
            seed: spt.seed.unwrap_or(1),
            size: spt.size,
            size_variance: spt.size_variance,
            branch_dimming: spt.leaves.branch_dimming,
            leaf_dimming: spt.leaves.dimming_amount,
            bud_angles: spt.leaves.bud_angles,
            rocking_groups: spt.leaves.rocking_groups,
            wind_matrices: (0, 4),
            leaf_lod_step: spt.lod.value_24,
        }
    }
}

/// One cross section along a branch (0x48 bytes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Section {
    /// +0x00: the direction the branch grows (the frame's first row).
    pub direction: V3,
    /// +0x0c.
    pub position: V3,
    /// +0x18.
    pub radius: f32,
    /// +0x1c: the frame, rows: along, and the two across.
    pub frame: M3,
    /// +0x40: distance along the branch.
    pub distance: f32,
    /// +0x44: the wind weight the branch's children inherit.
    pub wind: f32,
}

impl Default for Section {
    /// `00b130a0`.
    fn default() -> Self {
        Section {
            direction: [0.0; 3],
            position: [0.0; 3],
            radius: 0.0,
            frame: IDENTITY,
            distance: 0.0,
            wind: 0.0,
        }
    }
}

/// A flare on a trunk (`00b14ca0`, 24 bytes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flare {
    pub angle: f32,
    pub radial: f32,
    pub radial_exponent: f32,
    pub length: f32,
    pub length_exponent: f32,
    pub distance: f32,
}

impl Flare {
    /// `00b14980`: how far the flare pushes the ring out at `angle`
    /// (radians) and `t` along the branch.
    fn push(&self, angle: f32, t: f32) -> f32 {
        let mut a = self.angle;
        let mut angle = angle;
        if (angle - a).abs() > std::f32::consts::PI {
            if a <= angle {
                a += TWO_PI;
            } else {
                angle += TWO_PI;
            }
        }
        let gap = (angle - a).abs();
        let left = self.length - t;
        if gap < self.radial && left > 0.0 {
            let p1 = (d(1.0 - gap / self.radial)).powf(d(self.radial_exponent)) as f32;
            let p2 = (d(left / self.length)).powf(d(self.length_exponent)) as f32;
            self.distance * p1 * p2
        } else {
            0.0
        }
    }
}

/// A branch (`CBranch`, 0x50 bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    pub parent: Option<usize>,
    pub level: u32,
    /// +0x04: where along its parent it starts (0..1 of the parent's
    /// first..last branch range).
    pub t: f32,
    /// +0x08: children: (section, fraction to the next, branch).
    pub children: Vec<(usize, f32, usize)>,
    /// +0x20.
    pub sections: Vec<Section>,
    /// +0x28.
    pub cross_sections: u16,
    /// +0x2c: its first vertex in the branch geometry.
    pub first_vertex: u32,
    /// +0x30 (`00b14b40`): Σ (rᵢ + rᵢ₊₁) × |pᵢ₊₁ − pᵢ|, for levels of
    /// detail.
    pub weight: f32,
    /// +0x34: the levels of detail's sort key (`00b2c130`).
    pub lod_key: f32,
    pub flares: Vec<Flare>,
}

/// A leaf (`CLeaf`-like, 0x4c bytes, `00b40070`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Leaf {
    /// +0x04.
    pub position: V3,
    /// +0x10: rocking group.
    pub rock_group: u8,
    /// +0x14: colour, `0xff` alpha, then blue, green, red from the top.
    pub color: u32,
    /// +0x18: how dim (255 not at all).
    pub dimming: u8,
    /// +0x1c: the normal.
    pub normal: V3,
    /// +0x40: the leaf map (texture × 2 + mirrored).
    pub map: u8,
    /// +0x44: wind weight (as the branch gives it).
    pub wind: f32,
    /// +0x48: the wind group (the seed-level branch's number).
    pub wind_group: i32,
}

/// The branches' vertices (`CIndexedGeometry`, the CSpeedTreeRT's +4).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BranchGeometry {
    pub positions: Vec<V3>,
    pub normals: Vec<V3>,
    /// The ring's tangent around the branch (`00b305f0`).
    pub binormals: Vec<V3>,
    /// `binormal × out` (`00b30660`).
    pub tangents: Vec<V3>,
    pub uvs: Vec<[f32; 2]>,
    /// `0xff` alpha, blue, green, red (the dimming in each).
    pub colors: Vec<u32>,
    /// `1 − weight` (`00b30560`).
    pub wind_weights: Vec<f32>,
    /// `group % matrices + first`.
    pub wind_groups: Vec<u8>,
    /// Per level of detail, its triangle strips (`00b2fdc0`).
    pub lods: Vec<Vec<Vec<u16>>>,
    /// +0x18, +0x1c: the trunk's base diameter and its first branch's
    /// height.
    pub trunk_diameter: f32,
    pub trunk_first_branch: f32,
}

/// One leaf texture with its size worked out (engine leaf info's 0x54
/// entries).
#[derive(Debug, Clone, PartialEq)]
pub struct LeafMap {
    pub blossom: bool,
    pub color: [f32; 3],
    pub variance: f32,
    pub origin: [f32; 3],
    pub size: [f32; 3],
    /// +0x48: width and height (tree size × size).
    pub width: f32,
    pub height: f32,
}

/// A grown tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// The size drawn for this tree (`size ± variance`).
    pub size: f32,
    pub branches: Vec<Branch>,
    pub leaves: Vec<Leaf>,
    /// The leaves of each level of detail: the first is `leaves`, each next
    /// one pairs up the one before (`00b2c890`).
    pub leaf_lods: Vec<Vec<Leaf>>,
    pub geometry: BranchGeometry,
    pub leaf_maps: Vec<LeafMap>,
    /// The random generator as the growing left it (the leaves'
    /// rocking phases come from it next, `00b3cf40`).
    pub random: Random,
}

/// The C library's `rand`, seeded by `srand` (`00ecada6`): the trunk's
/// flares draw from it.
struct CRand(u32);

impl CRand {
    fn next(&mut self) -> i32 {
        self.0 = self.0.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.0 >> 16) & 0x7fff) as i32
    }

    /// `00b14e90`.
    fn uniform(&mut self, lo: f32, hi: f32) -> f32 {
        let r = self.next() as f32;
        ((d(r) / 32767.0) * (d(hi) - d(lo)) + d(lo)) as f32
    }
}

struct Grower<'a> {
    spt: &'a SptFile,
    settings: &'a Settings,
    random: Random,
    crand: CRand,
    /// `011f8bfc`.
    seed_counter: i32,
    /// `011ad14c`.
    seed_level: u32,
    /// `011f8c04`.
    wind_per_level: bool,
    /// `011ad148`: leaves are grown (no stored leaf levels).
    grow_leaves: bool,
    /// `011f8c18`, `011f8c68`: leaf maps for ordinary and blossom leaves.
    normal_maps: Vec<i32>,
    blossom_maps: Vec<i32>,
    /// `011f8c80`: the leaves of the branch being finished.
    local_leaves: Vec<usize>,
    branches: Vec<Branch>,
    leaves: Vec<Leaf>,
    geometry: BranchGeometry,
    maps: Vec<LeafMap>,
    /// The leaves of each level of detail after the first (`00b2c890`).
    leaf_lods: Vec<Vec<Leaf>>,
}

/// Grows the tree (`00b2af80`).
pub fn grow(spt: &SptFile, settings: &Settings) -> Tree {
    let tree_size = settings.size;
    let maps: Vec<LeafMap> = spt
        .leaves
        .textures
        .iter()
        .map(|t| {
            let (width, height) = if tree_size > 0.0 {
                (tree_size * t.size[0], tree_size * t.size[1])
            } else {
                (t.absolute_size[0], t.absolute_size[1])
            };
            LeafMap {
                blossom: t.blossom,
                color: t.color,
                variance: t.color_variance,
                origin: t.origin,
                size: t.size,
                width,
                height,
            }
        })
        .collect();
    let mut random = Random::new(settings.seed);
    let size = random.uniform(
        settings.size - settings.size_variance,
        settings.size + settings.size_variance,
    );
    let mut g = Grower {
        leaf_lods: Vec::new(),
        spt,
        settings,
        random: Random::new(settings.seed),
        crand: CRand(spt.srand as u32),
        seed_counter: settings.seed,
        seed_level: spt.seed_level.max(0) as u32,
        wind_per_level: spt.wind_per_level,
        grow_leaves: spt.stored_leaves.is_none(),
        normal_maps: Vec::new(),
        blossom_maps: Vec::new(),
        local_leaves: Vec::new(),
        branches: Vec::new(),
        leaves: Vec::new(),
        geometry: BranchGeometry::default(),
        maps,
    };
    g.geometry.lods = vec![Vec::new(); spt.lod.branch_lods.max(0) as usize];
    if !spt.levels.is_empty() {
        g.branches.push(new_branch(None));
        let up = [0.0, 0.0, 1.0];
        g.branch(
            0,
            settings.seed,
            size,
            0,
            [0.0; 3],
            0.0,
            0.0,
            IDENTITY,
            up,
            1.0,
            settings.seed,
            -1.0,
        );
    }
    g.branch_lods();
    if g.grow_leaves {
        g.reduce_leaves(settings.leaf_lod_step);
    }
    let mut leaf_lods = vec![g.leaves.clone()];
    leaf_lods.append(&mut g.leaf_lods);
    Tree {
        size,
        branches: g.branches,
        leaves: g.leaves,
        leaf_lods,
        geometry: g.geometry,
        leaf_maps: g.maps,
        random: g.random,
    }
}

/// `00b10dc0`.
fn new_branch(parent: Option<usize>) -> Branch {
    Branch {
        parent,
        level: 0,
        t: 0.0,
        children: Vec::new(),
        sections: Vec::new(),
        cross_sections: 0,
        first_vertex: u32::MAX,
        weight: 0.0,
        lod_key: 0.0,
        flares: Vec::new(),
    }
}

/// `00b404c0`'s packing (each channel × 255, truncated; alpha 255).
fn pack(c: [f32; 3]) -> u32 {
    let to = |c: f32| (d(c) * 255.0) as i32 as u32;
    to(c[0])
        .wrapping_add(to(c[1]) << 8)
        .wrapping_add(to(c[2]) << 16)
        .wrapping_add(0xff00_0000)
}

/// `00b40430`.
fn unpack(c: u32) -> [f32; 3] {
    [
        (c & 0xff) as f32 / 255.0,
        (d(((c & 0xff00) >> 8) as f32) / 255.0) as f32,
        (d(((c & 0xff_0000) >> 16) as f32) / 255.0) as f32,
    ]
}

/// `00b14ae0`.
fn seed_level_wind(level: u32, seed_level: u32, t: f32, flex: f32) -> f32 {
    if level == seed_level {
        (1.0 - d(t) * d(flex)) as f32
    } else {
        1.0
    }
}

/// `00b14b10`.
fn inherited_wind(w: f32, flex: f32) -> f32 {
    ((0.0 - d(w)) * d(flex) + d(w)) as f32
}

impl Grower<'_> {
    fn level(&self, i: u32) -> &BranchLevel {
        &self.spt.levels[i as usize]
    }

    /// `00b13a40`: which leaf maps ordinary and blossom leaves pick from.
    fn leaf_lists(&mut self) {
        self.normal_maps.clear();
        self.blossom_maps.clear();
        for (i, t) in self.spt.leaves.textures.iter().enumerate() {
            let i = i as i32;
            if t.blossom {
                self.blossom_maps.push(i * 2);
                self.blossom_maps.push(i * 2 + 1);
            } else {
                self.normal_maps.push(i * 2);
                self.normal_maps.push(i * 2 + 1);
            }
        }
    }

    /// `00b11050`: one branch, then its children.
    #[allow(clippy::too_many_arguments)]
    fn branch(
        &mut self,
        me: usize,
        mut seed: i32,
        size: f32,
        level: u32,
        start: V3,
        t: f32,
        dim_from: f32,
        parent_frame: M3,
        parent_direction: V3,
        wind: f32,
        group: i32,
        radius_cap: f32,
    ) {
        let group = if level == self.seed_level {
            let g = self.seed_counter;
            self.seed_counter += 1;
            g
        } else {
            group
        };
        if level == 0 {
            self.leaf_lists();
        }
        let lv = self.level(level).clone();
        // Fronds aren't drawn by any of the game's trees (13007 is off in
        // every file), so every level is a branch.
        self.branches[me].first_vertex = self.geometry.positions.len() as u32;
        self.branches[me].t = t;
        self.branches[me].level = level;
        self.flares(me, &lv);
        let length = (d(lv.length.value(t, &mut self.random)) * d(size)) as f32;
        let twist = self.random.uniform(-180.0, 180.0);
        let start_angle = lv.start_angle.value(t, &mut self.random);
        let gravity = lv.gravity.value(t, &mut self.random);
        let mut radius = (d(lv.radius.value(t, &mut self.random)) * d(size)) as f32;
        let flexibility = lv.flexibility.value(t, &mut self.random);
        let cross_sections = lv.cross_sections as u16;
        self.branches[me].cross_sections = cross_sections;
        let twist_offset = if lv.twist_texture {
            (d(twist) + 0.5 + d(t)) as f32
        } else {
            0.0
        };
        let u_offset = if (seed as u32) % 2 == 0 {
            lv.u_offset
        } else {
            -lv.u_offset
        };
        let v_tile = if lv.v_tile_absolute {
            lv.v_tile
        } else {
            (d(length / size) * d(lv.v_tile)) as f32
        };
        let u_tile = if lv.u_tile_absolute {
            lv.u_tile
        } else {
            (d(lv.u_tile) * d(radius) * d(TWO_PI)) as f32
        };
        let r0 = (d(lv.radius_scale.value(0.0, &mut self.random)) * d(radius)) as f32;
        if radius_cap > 0.0 && (d(radius_cap) * 0.85) < d(r0) {
            radius = (d(radius_cap) * 0.85) as f32;
        }
        let count = lv.segments + 1;
        if count < 2 {
            return;
        }
        let count = count as usize;
        let mut sections = vec![Section::default(); count];
        // The branch's strip (each ring's vertices with the next ring's,
        // two repeats between rows).
        let base = self.geometry.positions.len() as u16;
        let cs = cross_sections;
        let mut strip = Vec::with_capacity((count - 1) * (cs as usize * 2 + 4));
        let mut c: u16 = 0;
        for _ in 0..count - 1 {
            let row = c;
            for _ in 0..=cs {
                strip.push(cs + 1 + base + c);
                strip.push(base + c);
                c += 1;
            }
            strip.push(cs + 1 + base + row);
            strip.push(cs + 1 + base + row);
        }
        if let Some(lod) = self.geometry.lods.first_mut() {
            lod.push(strip);
        }
        sections[0].position = start;
        let d1 = lv.disturbance.variance_at(0.0, &mut self.random);
        let d2 = lv.disturbance.variance_at(0.0, &mut self.random);
        let mut frame = parent_frame;
        let axis = math::mat_vec(&parent_frame, parent_direction);
        math::rotate_axis(&mut frame, twist, axis);
        math::rotate_two(&mut frame, start_angle + d2, d1);
        sections[0].frame = frame;
        sections[0].direction = math::vec_mat([1.0, 0.0, 0.0], &frame);
        sections[0].radius = (d(lv.radius_scale.value(0.0, &mut self.random)) * d(radius)) as f32;
        let flex0 = (d(lv.flexibility_scale.value(0.0, &mut self.random)) * d(flexibility)) as f32;
        self.bend(&mut sections[0], &lv, 0.0, gravity);
        if level == 0 {
            self.geometry.trunk_diameter = sections[0].radius + sections[0].radius;
            self.geometry.trunk_first_branch = length * lv.first_branch;
        }
        let mut ring_wind = wind;
        if self.seed_level < level {
            if self.wind_per_level {
                sections[0].wind = inherited_wind(wind, flex0);
                ring_wind = sections[0].wind;
            }
        } else {
            sections[0].wind = seed_level_wind(level, self.seed_level, 0.0, flex0);
            ring_wind = sections[0].wind;
        }
        let tex = [u_tile, v_tile, u_offset];
        let first_dim = if level != 0 { dim_from } else { 0.0 };
        self.ring(
            me,
            &sections[0],
            0.0,
            first_dim,
            cs,
            tex,
            ring_wind,
            group,
            twist_offset,
            level,
        );
        let mut along = 0.0f32;
        sections[0].distance = 0.0;
        for i in 1..count {
            let u = (i as f32) / ((count - 1) as f32);
            let p = (d(u)).powf(d(lv.segment_exponent)) as f32;
            let step = (d(p) * d(length) - d(along)) as f32;
            sections[i].radius = (d(lv.radius_scale.value(p, &mut self.random)) * d(radius)) as f32;
            let flex = (d(lv.flexibility_scale.value(p, &mut self.random)) * d(flexibility)) as f32;
            sections[i].frame = sections[i - 1].frame;
            sections[i].direction = math::vec_mat([1.0, 0.0, 0.0], &sections[i].frame);
            let mut s = sections[i];
            self.bend(&mut s, &lv, p, gravity);
            sections[i] = s;
            let a = lv.disturbance.variance_at(p, &mut self.random);
            let b = lv.disturbance.variance_at(p, &mut self.random);
            math::rotate_two(&mut sections[i].frame, b, a);
            sections[i].direction = math::vec_mat([1.0, 0.0, 0.0], &sections[i].frame);
            let prev = sections[i - 1];
            sections[i].position = math::add(math::scale(prev.direction, step), prev.position);
            let mut ring_wind = wind;
            if self.seed_level < level {
                if self.wind_per_level {
                    sections[i].wind = inherited_wind(wind, flex);
                    ring_wind = sections[i].wind;
                }
            } else {
                sections[i].wind = seed_level_wind(level, self.seed_level, p, flex);
                ring_wind = sections[i].wind;
            }
            let dim = if level == 0 {
                if p < lv.first_branch {
                    1.0
                } else {
                    (-d(lv.first_branch) / (1.0 - d(lv.first_branch))) as f32
                }
            } else {
                dim_from
            };
            let s = sections[i];
            self.ring(
                me,
                &s,
                p,
                dim,
                cs,
                tex,
                ring_wind,
                group,
                twist_offset,
                level,
            );
            along = (d(along) + d(step)) as f32;
            sections[i].distance = along;
        }
        self.branches[me].sections = sections;
        self.normals(me);
        let children = ((d(lv.frequency) / d(size)) * d(length)) as i32;
        let next = level + 1;
        let last = (self.spt.levels.len() as i32 - 1) <= next as i32;
        if !last || self.grow_leaves {
            for k in 0..children.max(0) {
                if !last {
                    seed = seed.wrapping_add(3);
                    self.random.reseed(seed);
                }
                let range = d(lv.last_branch) - d(lv.first_branch);
                let pos = if !last && k == 0 {
                    let lo = (range * 0.85 + d(lv.first_branch)) as f32;
                    let hi = (range * 0.95 + d(lv.first_branch)) as f32;
                    self.random.uniform(lo, hi)
                } else {
                    self.random.uniform(lv.first_branch, lv.last_branch)
                };
                let (seg, frac) = self.find_section(me, (d(pos) * d(length)) as f32);
                let secs = &self.branches[me].sections;
                let mut child_wind = wind;
                if level == self.seed_level || self.wind_per_level {
                    let a = secs[seg].wind;
                    let b = secs[seg + 1].wind;
                    child_wind = ((d(b) - d(a)) * d(frac) + d(a)) as f32;
                }
                if !last {
                    self.random.reseed(seed);
                    self.random.uniform(0.0, 100.0);
                }
                let secs = &self.branches[me].sections;
                let at = math::lerp(secs[seg].position, secs[seg + 1].position, frac);
                let rel = if lv.first_branch != lv.last_branch {
                    ((d(pos) - d(lv.first_branch)) / (d(lv.last_branch) - d(lv.first_branch)))
                        as f32
                } else {
                    1.0
                };
                let frame = secs[seg].frame;
                let direction = secs[seg].direction;
                if !last {
                    let cap = ((d(secs[seg + 1].radius) - d(secs[seg].radius)) * d(frac)
                        + d(secs[seg].radius)) as f32;
                    let child = self.branches.len();
                    self.branches.push(new_branch(Some(me)));
                    let mut child_dim = ((1.0 - d(dim_from)) * d(rel) + d(dim_from)) as f32;
                    if (next as i32) < 2 {
                        child_dim = child_dim * child_dim;
                    }
                    self.branch(
                        child, seed, size, next, at, rel, child_dim, frame, direction, child_wind,
                        group, cap,
                    );
                    self.branches[me].children.push((seg, frac, child));
                } else {
                    self.leaf_branch(me, size, next, at, rel, frame, direction, child_wind, group);
                }
            }
        }
        if last {
            self.local_leaves.clear();
        }
        self.weigh(me);
    }

    /// The gravity step of `00b11050` for one section (`frame ← frame ·
    /// R(axis, g × gravity × angle × horizontal)`).
    fn bend(&mut self, s: &mut Section, lv: &BranchLevel, p: f32, gravity: f32) {
        let down: V3 = [0.0, 0.0, -1.0];
        let angle = (d(math::angle(s.direction, down)) * d(DEG)) as f32;
        let horizontal =
            (1.0 - (d(90.0 - angle)).abs() as f32 as f64 * 0.011_111_111f32 as f64) as f32;
        let mut axis = math::cross(s.direction, down);
        math::normalize(&mut axis);
        let gs = lv.gravity_scale.value(p, &mut self.random);
        let g = (-((d(gs) - 0.5) + (d(gs) - 0.5))) as f32;
        let r = math::rotation(
            (d(g) * d(gravity) * d(angle) * d(horizontal)) as f32,
            axis[0],
            axis[1],
            axis[2],
        );
        s.frame = math::mul(&s.frame, &r);
        s.direction = math::vec_mat([1.0, 0.0, 0.0], &s.frame);
    }

    /// `00b14ca0`: a branch's flares, from the C library's `rand`.
    fn flares(&mut self, me: usize, lv: &BranchLevel) {
        if lv.flares <= 0 {
            return;
        }
        let v = lv.flare_values;
        let base = self.crand.uniform(0.0, TWO_PI);
        let step = TWO_PI / lv.flares as f32;
        for i in 0..lv.flares {
            let r = self.crand.uniform((d(v[0]) * d(step)) as f32, step);
            let mut angle = (d(r) * d(i as f32) + d(base)) as f32;
            if TWO_PI < angle {
                angle -= TWO_PI;
            }
            let radial = (d(self.crand.uniform(v[1] - v[2], v[2] + v[1])) / d(DEG)) as f32;
            let length = self.crand.uniform(v[6] - v[7], v[6] + v[7]);
            let distance = self.crand.uniform(v[4] - v[5], v[4] + v[5]);
            self.branches[me].flares.push(Flare {
                angle,
                radial,
                radial_exponent: v[3],
                length,
                length_exponent: v[8],
                distance,
            });
        }
    }

    /// `00b14450`: one ring of vertices around a section.
    #[allow(clippy::too_many_arguments)]
    fn ring(
        &mut self,
        me: usize,
        s: &Section,
        t: f32,
        dim_from: f32,
        cs: u16,
        tex: [f32; 3],
        wind: f32,
        group: i32,
        twist_offset: f32,
        level: u32,
    ) {
        let step = 1.0 / cs as f32;
        let mut dim = ((1.0 - d(dim_from)) * d(t) + d(dim_from)) as f32;
        if level == 0 {
            dim = dim * dim;
        }
        let bd = self.settings.branch_dimming;
        dim = (d(dim) * d(bd) + (1.0 - d(bd))) as f32;
        let mut acc = 0.0f32;
        let (first, count) = self.settings.wind_matrices;
        for _ in 0..=cs {
            let angle = TWO_PI * acc;
            let u = ((d(tex[2]) * d(t)) + d(acc * tex[0])) as f32;
            let v = ((d(t) + d(twist_offset)) * d(tex[1])) as f32;
            // The texture's v is turned around for Direct3D (`011f8b79`).
            self.geometry.uvs.push([u, -v]);
            let row1 = [s.frame[3], s.frame[4], s.frame[5]];
            let row2 = [s.frame[6], s.frame[7], s.frame[8]];
            let mix = |c: f32, sn: f32| -> V3 {
                [
                    (d(c) * d(row1[0]) + d(sn) * d(row2[0])) as f32,
                    (d(c) * d(row1[1]) + d(sn) * d(row2[1])) as f32,
                    (d(c) * d(row1[2]) + d(sn) * d(row2[2])) as f32,
                ]
            };
            let a = d(angle);
            let mut out = mix(a.cos() as f32, a.sin() as f32);
            let a2 = d((d(HALF_PI) + d(angle)) as f32);
            let around = mix(a2.cos() as f32, a2.sin() as f32);
            self.geometry.binormals.push(around);
            self.geometry.tangents.push(math::cross(around, out));
            let mut flare = 0.0f32;
            for f in &self.branches[me].flares {
                flare = (d(f.push(angle, t)) + d(flare)) as f32;
            }
            let push = 1.0 + flare;
            let r = s.radius;
            let mut p = [
                (d(out[0]) * d(r) + d(s.position[0])) as f32,
                (d(out[1]) * d(r) + d(s.position[1])) as f32,
                (d(out[2]) * d(r) + d(s.position[2])) as f32,
            ];
            if push != 1.0 {
                let q = [
                    (d(out[0]) * d(r) * d(push) + d(s.position[0])) as f32,
                    (d(out[1]) * d(r) * d(push) + d(s.position[1])) as f32,
                    (d(out[2]) * d(r) * d(push) + d(s.position[2])) as f32,
                ];
                let mut n = math::sub(q, p);
                math::normalize(&mut n);
                out = n;
                p = q;
            }
            let _ = out;
            self.geometry.positions.push(p);
            let c = (d(dim) * 255.0) as i32 as u32;
            self.geometry
                .colors
                .push(c + (c << 8) + (c << 16) + 0xff00_0000);
            self.geometry.wind_weights.push(1.0 - wind);
            let g = ((group as u32 as u8) as u32 % count.max(1) as u32) as u8;
            self.geometry.wind_groups.push(g.wrapping_add(first as u8));
            acc = (d(acc) + d(step)) as f32;
        }
    }

    /// `00b14ed0`: the normals of a branch's vertices, each from its ring
    /// neighbours and the rings before and after.
    fn normals(&mut self, me: usize) {
        let b = &self.branches[me];
        let cs = b.cross_sections as usize;
        let first = b.first_vertex as usize;
        let rows = b.sections.len();
        let pos = &self.geometry.positions;
        let mut normals = Vec::with_capacity(rows * (cs + 1));
        for i in 0..rows {
            let row = (cs + 1) * i;
            for k in 0..=cs {
                let before = if k == 0 { row + cs - 1 } else { row + k - 1 };
                let after = if k == cs { row + 1 } else { row + 1 + k };
                let mut around = math::sub(pos[first + after], pos[first + before]);
                math::normalize(&mut around);
                let below = if i != 0 { (i - 1) * (cs + 1) } else { row } + k;
                let above = if i != rows - 1 {
                    (i + 1) * (cs + 1)
                } else {
                    row
                } + k;
                let mut along = math::sub(pos[first + above], pos[first + below]);
                math::normalize(&mut along);
                normals.push(math::cross(around, along));
            }
        }
        self.geometry.normals.extend(normals);
    }

    /// `00b14be0`: the section a distance along the branch falls in, and
    /// how far to the next.
    fn find_section(&self, me: usize, distance: f32) -> (usize, f32) {
        let secs = &self.branches[me].sections;
        if secs.len() < 2 {
            return (0, 0.0);
        }
        let mut seg = 0;
        for (i, s) in secs.iter().enumerate().skip(1) {
            if distance < s.distance {
                seg = i - 1;
                break;
            }
        }
        let a = secs[seg].distance;
        let b = secs[seg + 1].distance;
        (seg, ((d(distance) - d(a)) / (d(b) - d(a))) as f32)
    }

    /// `00b13100`: a last-level "branch": a short twig from `at`, its end
    /// gets the leaf.
    #[allow(clippy::too_many_arguments)]
    fn leaf_branch(
        &mut self,
        parent: usize,
        size: f32,
        level: u32,
        at: V3,
        t: f32,
        parent_frame: M3,
        parent_direction: V3,
        wind: f32,
        group: i32,
    ) {
        let lv = self.level(level).clone();
        let length = (d(lv.length.value(t, &mut self.random)) * d(size)) as f32;
        let step = length / lv.segments as f32;
        let step = if 0.01 <= step { step } else { 0.01 };
        let twist = self.random.uniform(-180.0, 180.0);
        let axis = math::mat_vec(&parent_frame, parent_direction);
        let mut frame = parent_frame;
        math::rotate_axis(&mut frame, twist, axis);
        math::pitch(&mut frame, 60.0);
        let direction = math::vec_mat([1.0, 0.0, 0.0], &frame);
        let end = math::add(at, math::scale(direction, step));
        let mut leaf_direction = math::sub(end, at);
        math::normalize(&mut leaf_direction);
        // From an ancestor's base (`normal_depth` levels up), else the
        // parent's own.
        let b = &self.branches[parent];
        let mut root_direction = leaf_direction;
        if !b.sections.is_empty() {
            let mut base = b.sections[0].position;
            let depth = self.spt.leaves.normal_depth;
            if depth != 0 {
                let mut up = Some(parent);
                let mut k = 0;
                while let Some(u) = up {
                    if k >= depth {
                        break;
                    }
                    up = self.branches[u].parent;
                    k += 1;
                }
                if let Some(u) = up {
                    if let Some(s) = self.branches[u].sections.first() {
                        base = s.position;
                    }
                }
            }
            root_direction = math::sub(end, base);
        }
        math::normalize(&mut root_direction);
        self.leaf(end, t, parent, leaf_direction, root_direction, wind, group);
    }

    /// `00b13c10`: one leaf, unless it lands too near another.
    #[allow(clippy::too_many_arguments)]
    fn leaf(
        &mut self,
        position: V3,
        t: f32,
        parent: usize,
        leaf_direction: V3,
        root_direction: V3,
        wind: f32,
        group: i32,
    ) {
        if self.maps.is_empty() {
            return;
        }
        let blossom = !self.blossom_maps.is_empty() && self.blossom(parent, t);
        let map = if blossom {
            let n = self.blossom_maps.len();
            let mut i = 0;
            if n > 1 {
                let r = self.random.uniform(0.0, 100_000.0);
                i = ((r as f64).trunc() as i64 as u32 % n as u32) as usize;
            }
            self.blossom_maps[i]
        } else {
            let n = self.normal_maps.len();
            if n == 0 {
                return;
            }
            let mut i = 0;
            if n > 1 {
                let r = self.random.uniform(0.0, 1_000_000.0);
                i = ((r as f64).trunc() as i64 as u32 % n as u32) as usize;
            }
            self.normal_maps[i]
        };
        let tex = (map / 2) as usize;
        let free = match self.spt.leaves.collision {
            1 => self.room(position, tex, &self.local_leaves),
            2 => {
                let all: Vec<usize> = (0..self.leaves.len()).collect();
                self.room(position, tex, &all)
            }
            _ => true,
        };
        if !free {
            return;
        }
        let mut dim = 1.0f32;
        if self.spt.leaves.dimming {
            let mut l = self.depth_share(Some(parent));
            l = ((1.0 - d(l)) * d(t) + d(l)) as f32;
            l = (d(l) * d(l) * d(l)) as f32;
            let keep = 1.0 - self.settings.leaf_dimming;
            dim = (d(keep) * (1.0 - d(l)) + d(l)) as f32;
        }
        let rock = self.random.uniform(0.0, 10_000.0) as i32;
        let groups = self.settings.rocking_groups.max(1);
        let rock_group = (rock % groups) as u8;
        let dimming = ((d(dim) * 255.0) as i32) as u16 as u8;
        let m = &self.maps[tex];
        let mut normal = if math::fast_length(leaf_direction) >= 0.0001 {
            let blend = math::scale(math::sub(leaf_direction, root_direction), m.variance);
            math::add(root_direction, blend)
        } else {
            root_direction
        };
        math::normalize(&mut normal);
        let r = self.random.uniform(-m.variance, m.variance);
        let mut color = [r + m.color[0], r + m.color[1], r + m.color[2]];
        let k = dimming as f32 / 255.0;
        for c in color.iter_mut() {
            *c *= k;
            *c = c.clamp(0.0, 1.0);
        }
        let packed = pack(color);
        let index = self.leaves.len();
        self.leaves.push(Leaf {
            position,
            rock_group,
            color: packed,
            dimming,
            normal,
            map: map as u8,
            wind,
            wind_group: group,
        });
        if self.spt.leaves.collision == 1 {
            self.local_leaves.push(index);
        }
    }

    /// `00b14280`: no leaf within the box `max(width, height) × collision
    /// scale` around `position`.
    fn room(&self, position: V3, tex: usize, others: &[usize]) -> bool {
        let m = &self.maps[tex];
        let half = (if m.width <= m.height {
            m.height
        } else {
            m.width
        }) * self.spt.leaves.collision_scale;
        for &o in others {
            let p = self.leaves[o].position;
            if position[0] < p[0] + half
                && p[0] - half < position[0]
                && position[1] < p[1] + half
                && p[1] - half < position[1]
                && position[2] < p[2] + half
                && p[2] - half < position[2]
            {
                return false;
            }
        }
        true
    }

    /// `00b15110`: how deep along its ancestors a branch starts (0 for the
    /// trunk and its children's measure from it).
    fn depth_share(&self, b: Option<usize>) -> f32 {
        let Some(b) = b else { return 0.0 };
        let Some(parent) = self.branches[b].parent else {
            return 0.0;
        };
        let up = self.depth_share(Some(parent));
        ((1.0 - d(up)) * d(self.branches[b].t) + d(up)) as f32
    }

    /// `00b13b50`: whether a leaf at `t` on `branch` is a blossom.
    fn blossom(&mut self, branch: usize, t: f32) -> bool {
        let info = &self.spt.leaves;
        let mut at = t;
        if info.normal_depth == 0 {
            let mut b = Some(branch);
            let mut k = 0;
            while let Some(x) = b {
                let up = self.branches[x].parent;
                if up.is_none() || info.normal_depth <= k {
                    break;
                }
                b = up;
                k += 1;
            }
            if let Some(x) = b {
                at = self.branches[x].t;
            }
        }
        if info.blossom_distance < at {
            let r = self.random.uniform(0.0, 1.0);
            if math::not_less(info.blossom_level, r) {
                return true;
            }
        }
        false
    }

    /// `00b2c130`: which branches each level of detail draws. The branches
    /// in order (`00b13820`: each before its children), the heaviest
    /// (more than `1 − keep` of the heaviest's weight) set apart; each other
    /// one draws a random number for its sort key (which stays its weight:
    /// the blend the code makes with it is only checked for being below
    /// 0); both lists sorted heaviest first (`std::sort`), the set-apart
    /// ones put in front one at a time (so in reverse). Level `l` of `n`
    /// takes branches from the front while the weight taken so far is below
    /// `(min − max) × l/(n − 1) + max` of the total, the one crossing it
    /// included.
    fn branch_lods(&mut self) {
        let lods = self.geometry.lods.len();
        if lods == 0 || self.branches.is_empty() {
            return;
        }
        let mut order: Vec<usize> = (0..self.branches.len()).collect();
        let mut total = 0.0f32;
        let mut heaviest = 0.0f32;
        for &b in &order {
            let w = self.branches[b].weight;
            total += w;
            if heaviest < w {
                heaviest = w;
            }
        }
        let keep = 1.0 - self.spt.lod.branch_keep;
        let mut apart = Vec::new();
        let mut i = 0;
        while i < order.len() {
            let w = self.branches[order[i]].weight;
            if d(w) <= d(heaviest) * d(keep) {
                let r = self.random.uniform(0.0, self.spt.lod.branch_random);
                let blend = d(r) * d(heaviest) + (1.0 - d(r)) * d(w);
                self.branches[order[i]].lod_key = if 0.0 <= blend { w } else { 0.0 };
                i += 1;
            } else {
                apart.push(order.remove(i));
            }
        }
        let key = |b: &Branch| b.lod_key;
        order.sort_by(|a, b| key(&self.branches[*b]).total_cmp(&key(&self.branches[*a])));
        apart.sort_by(|a, b| key(&self.branches[*b]).total_cmp(&key(&self.branches[*a])));
        for b in apart {
            order.insert(0, b);
        }
        for l in 0..lods {
            let share = if lods < 2 {
                1.0
            } else {
                ((d(self.spt.lod.branch_min) - d(self.spt.lod.branch_max))
                    * d((l as f32) / ((lods - 1) as f32))
                    + d(self.spt.lod.branch_max)) as f32
            };
            let target = share * total;
            let mut taken = 0.0f32;
            let mut count = 0;
            while count < order.len() && taken < target {
                taken += self.branches[order[count]].weight;
                count += 1;
            }
            let mut strips = Vec::new();
            for &b in order.iter().take(count) {
                if let Some(s) = self.strip(b) {
                    strips.push(s);
                }
            }
            if l == 0 && count == 0 {
                strips.push(Vec::new());
            }
            self.geometry.lods[l] = strips;
        }
    }

    /// `00b135a0`: a branch's strip (every ring, the step from 1 cross
    /// section to the next being 1 in this version).
    fn strip(&self, b: usize) -> Option<Vec<u16>> {
        let br = &self.branches[b];
        let cs = br.cross_sections;
        if cs < 2 {
            return None;
        }
        let rows = br.sections.len();
        let base = br.first_vertex as u16;
        let mut s = Vec::with_capacity(rows.saturating_sub(1) * (cs as usize * 2 + 4));
        let mut pos = 0u16;
        for _ in 0..rows.saturating_sub(1) {
            let row = pos;
            for _ in 0..cs {
                s.push(base + 1 + cs + pos);
                s.push(pos + base);
                pos += 1;
            }
            s.push(base + 1 + cs + row + cs);
            s.push(base + row + cs);
            pos = row + cs + 1;
            s.push(base + cs + 1 + row);
            s.push(base + cs + 1 + row);
        }
        Some(s)
    }

    /// `00b2c890`: the leaves of each level of detail after the first: the
    /// previous level's leaves paired (`00b41110`: each leaf with the
    /// nearest later one not yet taken that is nearer than ten times the
    /// largest card × collision scale, searching onward until a taken one),
    /// then each pair kept as one leaf when a random 0..1 is above the
    /// file's 9010 (`00b40c80`): halfway between, raised by `scale ×
    /// height × 0.5 × (origin y − 0.5)`, the two colours averaged.
    fn reduce_leaves(&mut self, step: f32) {
        let mut reach = -1.0f32;
        for m in &self.maps {
            let big = if m.width <= m.height {
                m.height
            } else {
                m.width
            };
            let r = big * self.spt.leaves.collision_scale;
            if reach < r {
                reach = r;
            }
        }
        if reach == -1.0 {
            reach = 10.0;
        }
        let reach = reach * 10.0;
        let lods = self.spt.lod.leaf_lods.unwrap_or(0);
        let mut previous = self.leaves.clone();
        for l in 1..lods.max(1) {
            let scale = (l as f32) * step + 1.0;
            let n = previous.len();
            let mut taken = vec![false; n];
            let mut pairs = Vec::new();
            for i in 0..n {
                let mut best = f32::MAX;
                let mut partner = None;
                for j in i + 1..n {
                    if taken[j] {
                        break;
                    }
                    let dist = math::fast_distance(previous[i].position, previous[j].position);
                    if dist < reach && dist < best {
                        best = dist;
                        partner = Some(j);
                    }
                }
                if let Some(j) = partner {
                    taken[j] = true;
                    pairs.push((i, j));
                }
            }
            let mut next = Vec::new();
            for (a, b) in pairs {
                let r = self.random.uniform(0.0, 1.0);
                if math::not_less(self.spt.lod.value_fc, r) {
                    continue;
                }
                let (la, lb) = (previous[a], previous[b]);
                let mut leaf = la;
                let sum = [
                    lb.position[0] + la.position[0],
                    lb.position[1] + la.position[1],
                    lb.position[2] + la.position[2],
                ];
                let mut p = [0.5 * sum[0], 0.5 * sum[1], 0.5 * sum[2]];
                let m = &self.maps[(la.map / 2) as usize];
                let lift = (d(scale) * d(m.height) * 0.5 * (d(m.origin[1]) - 0.5)) as f32;
                p[2] += lift;
                leaf.position = p;
                let ca = unpack(la.color);
                let cb = unpack(lb.color);
                let avg = [
                    ((cb[0] + ca[0]) * 0.5).clamp(0.0, 1.0),
                    ((cb[1] + ca[1]) * 0.5).clamp(0.0, 1.0),
                    ((cb[2] + ca[2]) * 0.5).clamp(0.0, 1.0),
                ];
                leaf.color = pack(avg);
                next.push(leaf);
            }
            self.leaf_lods.push(next.clone());
            previous = next;
        }
    }

    /// `00b14b40`.
    fn weigh(&mut self, me: usize) {
        let b = &mut self.branches[me];
        if b.sections.len() < 2 {
            return;
        }
        let mut w = 0.0f32;
        for i in 0..b.sections.len() - 1 {
            let s0 = b.sections[i];
            let s1 = b.sections[i + 1];
            let dist = math::fast_distance(s0.position, s1.position);
            w = ((d(s0.radius) + d(s1.radius)) * d(dist) + d(w)) as f32;
        }
        b.weight = w;
    }
}
