//! Distant land and distant objects: which of the game's pre-built pieces
//! are drawn around the player. Read from the game's code (its terrain
//! manager: the asserts name `BGSTerrainManager.cpp`, `BGSTerrainNode.cpp`
//! and `BGSDistantObjectBlock.cpp`) and checked against the Goodsprings
//! recording, where every distant-land chunk and object block the game drew
//! is what [`drawn_nodes`] and [`object_blocks`] give for the player's spot
//! (see the tests).
//!
//! **The land is a quadtree.** A worldspace's `lodsettings\<editor
//! id>.dlodsettings` ([`LodSettings`]) names its root (WastelandNV: 128
//! cells across from cell −64, −64), the finest level (4) and the coarsest
//! level with models (32). Every node is split into its four quarters down
//! to the finest level; each frame (`006fdaa0`) a node is drawn whole unless
//!
//! - it's coarser than the coarsest level with models (always split), or
//! - it touches the loaded cells (`006fede0`: the `uGridsToLoad` square
//!   around the player's cell, edges included), or
//! - the player is nearer it than its split distance (`006fe550`): the
//!   distance from the player's position to the node's square on the map
//!   (`006fe830`, flat: no heights), against the node's radius (half its
//!   diagonal, `006fd210`) × `[TerrainManager] fSplitDistanceMult` (exe
//!   default 0.75; `FalloutPrefs.ini` here says 1.5, and the recording
//!   matches 1.5 only).
//!
//! Drawn nodes are the models `meshes\landscape\lod\<world>\<world>
//! .level<L>.x<X>.y<Y>.nif` ([`LodNode::land_model`]).
//!
//! **Geomorphing** (`006feb20`): each drawn node gets a factor from its
//! flat distance `d`: 1 nearer than the parent's radius ×
//! `fMorphEndDistanceMult` (0.65), 0 past the parent's radius ×
//! `fMorphStartDistanceMult` (0.7), linear between; the root always 1. The
//! distant-land vertex shader (`SLS2002.vso`) draws each vertex at
//! `lerp(texcoord1, z, factor)`. In all 2,308 distant-land models the game
//! ships (every worldspace, levels 4 to 32) and in the recorded vertex
//! buffers, texcoord1 is the vertex's own height, so the factor changes
//! nothing with the game's own files. With the
//! split distance at 1.5 radii, only level-4 nodes can be near enough to
//! morph (a coarser drawn node is at least 0.75 of its parent's radius
//! away): recorded 1 on the nearest level-4 chunks, 0.41288 on one, 0 on
//! all others.
//!
//! **The parent's texture** (`006ff2c0`, `006ff3f0`): when a node's four
//! quarters replace it, each quarter starts out drawn with its quarter of
//! the parent's texture and normal map and fades to its own over
//! `uTerrainTextureFadeTime` milliseconds (1000), linearly
//! ([`texture_fade`]); the shader's `LODTexParams` = (the quarter's corner
//! in the parent's texture, `fDetailTextureScale` 3 × level / 4 (not used
//! by the shader), the fade). The quarters only appear once all four are
//! loaded. Recorded: every chunk at 1 (no fade running), `.z` 3, 6, 12, 24.
//!
//! **Distant objects** (`006fdfc0`): the merged blocks are one per node of
//! the objects level (4), whatever the land does: within
//! `fBlockLoadDistance` (125000) of the player a block is kept, its
//! ordinary model `...\blocks\<world>.level4.x<X>.y<Y>.nif` within
//! `fBlockLoadDistanceLow` (50000; exe default `fDefaultBlockLoadDistanceLow`
//! 50000) and its "high" model `...\blocks\<world>.level4.high.x<X>.y<Y>
//! .nif` farther out: only the tallest landmarks, and only six blocks of
//! the Mojave have one. Recorded: exactly the ordinary blocks within 50000
//! and the high ones of (−4, 12), (−8, 20), (−8, 24).

use crate::land::CELL_SIZE;

/// Where a worldspace's quadtree is described (the game's format string is
/// `Data\LODSettings\%s.DLODSettings` with the worldspace's editor ID).
pub fn settings_path(world: &str) -> String {
    format!("lodsettings\\{}.dlodsettings", world.to_ascii_lowercase())
}

/// A worldspace's distant-land quadtree (`<world>.dlodsettings`, 24 bytes,
/// read in this order by the terrain manager's constructor, `006fc490`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LodSettings {
    /// The finest level, in cells per side: nodes this size aren't split.
    pub min_level: u32,
    /// The coarsest level with models: larger nodes are always split.
    pub max_level: u32,
    /// The root node's size in cells.
    pub root_level: u32,
    /// The root's south-west cell.
    pub root: (i32, i32),
    /// The north-east corner the file gives (cell 63, 63 in the Mojave);
    /// the code traced here doesn't use it.
    pub last_cell: (i32, i32),
    /// The level of the distant-object blocks.
    pub object_level: u32,
}

impl LodSettings {
    /// Reads a `.dlodsettings` file: u32 finest level, u32 coarsest level
    /// with models, u32 root size, i16 × 2 root cell, i16 × 2 last cell, u32
    /// objects level (WastelandNV: 4, 32, 128, −64 −64, 63 63, 4).
    pub fn parse(bytes: &[u8]) -> Option<LodSettings> {
        let u32_at = |at: usize| {
            bytes
                .get(at..at + 4)
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        let i16_at = |at: usize| {
            bytes
                .get(at..at + 2)
                .map(|b| i32::from(i16::from_le_bytes([b[0], b[1]])))
        };
        let settings = LodSettings {
            min_level: u32_at(0)?,
            max_level: u32_at(4)?,
            root_level: u32_at(8)?,
            root: (i16_at(12)?, i16_at(14)?),
            last_cell: (i16_at(16)?, i16_at(18)?),
            object_level: u32_at(20)?,
        };
        // Levels halve down the tree: anything else isn't a quadtree.
        let power = |l: u32| l > 0 && l.is_power_of_two();
        (power(settings.min_level)
            && power(settings.root_level)
            && settings.min_level <= settings.root_level)
            .then_some(settings)
    }

    /// The root node.
    pub fn root_node(&self) -> LodNode {
        LodNode {
            level: self.root_level,
            x: self.root.0,
            y: self.root.1,
        }
    }

    /// The node `node` is a quarter of (`None` for the root).
    pub fn parent(&self, node: LodNode) -> Option<LodNode> {
        if node.level >= self.root_level {
            return None;
        }
        let level = node.level * 2;
        let snap = |c: i32, root: i32| root + (c - root).div_euclid(level as i32) * level as i32;
        Some(LodNode {
            level,
            x: snap(node.x, self.root.0),
            y: snap(node.y, self.root.1),
        })
    }
}

/// The terrain manager's INI settings (`[TerrainManager]`, and
/// `[General] uGridsToLoad`), with the exe's defaults (read from the
/// settings' constructors at `00f8bb60` … `00f8be20`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainSettings {
    /// `fSplitDistanceMult` (0.75): a node is split when the player is
    /// nearer than its radius × this.
    pub split_mult: f32,
    /// `fMorphStartDistanceMult` (0.7) and `fMorphEndDistanceMult` (0.65):
    /// where a node's geomorph factor starts falling from 1 (end, nearer)
    /// and reaches 0 (start), as multiples of its parent's radius. Not in
    /// this install's INI files.
    pub morph_start_mult: f32,
    pub morph_end_mult: f32,
    /// `uTerrainTextureFadeTime` (1000): how long new quarters take to fade
    /// from their parent's texture to their own, in milliseconds.
    pub texture_fade_ms: f32,
    /// `bKeepLowDetailTerrain` (1): nodes of the coarsest level keep their
    /// models loaded (hidden) while split.
    pub keep_low_detail: bool,
    /// `fBlockLoadDistance` (125000): distant-object blocks nearer than this
    /// are kept.
    pub block_load_distance: f32,
    /// `fBlockLoadDistanceLow` (exe default `fDefaultBlockLoadDistanceLow`
    /// 50000): nearer than this a block draws its ordinary model, farther its
    /// "high" one.
    pub block_load_distance_low: f32,
    /// `uGridsToLoad` (5): the loaded cells, whose nodes are always split.
    pub grids_to_load: i32,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        TerrainSettings {
            split_mult: 0.75,
            morph_start_mult: 0.7,
            morph_end_mult: 0.65,
            texture_fade_ms: 1000.0,
            keep_low_detail: true,
            block_load_distance: 125_000.0,
            block_load_distance_low: 50_000.0,
            grids_to_load: 5,
        }
    }
}

impl TerrainSettings {
    /// The settings from an INI lookup (`section`, `key` → value).
    pub fn from_ini(get: impl Fn(&str, &str) -> Option<f32>) -> Self {
        let d = TerrainSettings::default();
        let tm = |key: &str, default: f32| get("TerrainManager", key).unwrap_or(default);
        let low_default = tm("fDefaultBlockLoadDistanceLow", d.block_load_distance_low);
        TerrainSettings {
            split_mult: tm("fSplitDistanceMult", d.split_mult),
            morph_start_mult: tm("fMorphStartDistanceMult", d.morph_start_mult),
            morph_end_mult: tm("fMorphEndDistanceMult", d.morph_end_mult),
            texture_fade_ms: tm("uTerrainTextureFadeTime", d.texture_fade_ms),
            keep_low_detail: tm("bKeepLowDetailTerrain", 1.0) != 0.0,
            block_load_distance: tm("fBlockLoadDistance", d.block_load_distance),
            block_load_distance_low: tm("fBlockLoadDistanceLow", low_default),
            grids_to_load: get("General", "uGridsToLoad").unwrap_or(5.0) as i32,
        }
    }
}

/// One node of the quadtree: `level` cells across from cell `x`, `y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LodNode {
    pub level: u32,
    pub x: i32,
    pub y: i32,
}

impl LodNode {
    /// Its width in game units (`level << 12`, as the game computes it).
    pub fn size(self) -> f32 {
        (self.level << 12) as f32
    }

    /// Its middle, in game units (`006fd210`: computed in doubles, kept as
    /// floats).
    pub fn center(self) -> [f32; 2] {
        let half = f64::from(self.size()) * 0.5;
        [
            (half + f64::from(self.x) * f64::from(CELL_SIZE)) as f32,
            (half + f64::from(self.y) * f64::from(CELL_SIZE)) as f32,
        ]
    }

    /// Its radius: from its middle to its south-west corner (half the
    /// diagonal), as the game measures it.
    pub fn radius(self) -> f32 {
        let c = self.center();
        let dx = c[0] - (f64::from(self.x) * f64::from(CELL_SIZE)) as f32;
        let dy = c[1] - (f64::from(self.y) * f64::from(CELL_SIZE)) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// The flat distance from a point to the node's square (0 inside it):
    /// the game's measure for splitting, geomorphing and object blocks
    /// (`006fe830`).
    pub fn distance(self, point: [f32; 2]) -> f32 {
        let size = self.size();
        let c = self.center();
        let half = size * 0.5;
        let gap = |p: f32, c: f32| {
            let lo = c - half;
            if p <= lo {
                lo - p
            } else if lo + size < p {
                p - (lo + size)
            } else {
                0.0
            }
        };
        let (dx, dy) = (gap(point[0], c[0]), gap(point[1], c[1]));
        (dy * dy + dx * dx).sqrt()
    }

    /// Its four quarters, in the game's order: south-west, north-west,
    /// south-east, north-east (`006fd210`).
    pub fn children(self) -> [LodNode; 4] {
        let level = self.level / 2;
        let h = level as i32;
        let at = |x: i32, y: i32| LodNode { level, x, y };
        [
            at(self.x, self.y),
            at(self.x, self.y + h),
            at(self.x + h, self.y),
            at(self.x + h, self.y + h),
        ]
    }

    /// Where this quarter's part of its parent's textures starts
    /// (`LodTexParams.xy`, `006ff210`): the chunks' textures run east to
    /// west, so the western quarters start half way across; the northern
    /// ones half way up.
    pub fn offset_in_parent(self, parent: LodNode) -> [f32; 2] {
        let west = self.x == parent.x;
        let north = self.y != parent.y;
        [if west { 0.5 } else { 0.0 }, if north { 0.5 } else { 0.0 }]
    }

    /// Whether the node touches the loaded cells: the `grids` × `grids`
    /// square around `cell`, its edges included (`006fede0`).
    pub fn touches_loaded(self, cell: (i32, i32), grids: i32) -> bool {
        let lo = (cell.0 - (grids >> 1), cell.1 - (grids >> 1));
        let size = self.level as i32;
        self.x <= lo.0 + grids
            && lo.0 <= self.x + size
            && self.y <= lo.1 + grids
            && lo.1 <= self.y + size
    }

    /// The node's distant-land model (the game's format string,
    /// `Data\Meshes\Landscape\LOD\%s\%s.Level%i.X%i.Y%i.NIF`).
    pub fn land_model(self, world: &str) -> String {
        let w = world.to_ascii_lowercase();
        format!(
            "meshes\\landscape\\lod\\{w}\\{w}.level{}.x{}.y{}.nif",
            self.level, self.x, self.y
        )
    }

    /// The node's distant-object block: the ordinary one, or the "high" one
    /// drawn past `fBlockLoadDistanceLow` (`006f55f0`, `006f5640`).
    pub fn object_model(self, world: &str, high: bool) -> String {
        let w = world.to_ascii_lowercase();
        let high = if high { ".high" } else { "" };
        format!(
            "meshes\\landscape\\lod\\{w}\\blocks\\{w}.level{}{high}.x{}.y{}.nif",
            self.level, self.x, self.y
        )
    }
}

/// A node to draw, and its geomorph factor (1: its own heights, 0: its
/// texcoord1 heights).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawnNode {
    pub node: LodNode,
    pub morph: f32,
}

/// Whether the game splits `node` with the player at `player` (game
/// units) in cell `cell` (`006fe550`).
pub fn split(
    node: LodNode,
    lod: &LodSettings,
    terrain: &TerrainSettings,
    player: [f32; 2],
    cell: (i32, i32),
) -> bool {
    if node.level > lod.max_level {
        return true;
    }
    if node.level <= lod.min_level {
        return false;
    }
    node.touches_loaded(cell, terrain.grids_to_load)
        || node.distance(player) < node.radius() * terrain.split_mult
}

/// A drawn node's geomorph factor (`006feb20`), its distances from its
/// parent's radius (the root: always 1).
pub fn morph(node: LodNode, lod: &LodSettings, terrain: &TerrainSettings, player: [f32; 2]) -> f32 {
    let Some(parent) = lod.parent(node) else {
        return 1.0;
    };
    let start = parent.radius() * terrain.morph_start_mult;
    let end = parent.radius() * terrain.morph_end_mult;
    let d = node.distance(player);
    if d < end {
        1.0
    } else if d <= start {
        1.0 - (d - end) / (start - end)
    } else {
        0.0
    }
}

/// The distant-land nodes the game draws with the player at `player` (game
/// units: the player's own position, not the camera's; in first person the
/// game's camera stands 3.7 units behind it) in cell `cell`, coarsest first.
pub fn drawn_nodes(
    lod: &LodSettings,
    terrain: &TerrainSettings,
    player: [f32; 2],
    cell: (i32, i32),
) -> Vec<DrawnNode> {
    let mut out = Vec::new();
    let mut stack = vec![lod.root_node()];
    while let Some(node) = stack.pop() {
        if split(node, lod, terrain, player, cell) {
            if node.level > lod.min_level {
                // Pushed in reverse so they come out in the game's order.
                stack.extend(node.children().into_iter().rev());
            }
        } else if node.level <= lod.max_level {
            out.push(DrawnNode {
                node,
                morph: morph(node, lod, terrain, player),
            });
        }
    }
    out
}

/// Every node of the quadtree at `level`.
pub fn nodes_at(lod: &LodSettings, level: u32) -> Vec<LodNode> {
    let mut out = Vec::new();
    let mut stack = vec![lod.root_node()];
    while let Some(node) = stack.pop() {
        if node.level == level {
            out.push(node);
        } else if node.level > level && node.level > 1 {
            stack.extend(node.children());
        }
    }
    out
}

/// A distant-object block to draw: its node, and whether it's the "high"
/// model (past `fBlockLoadDistanceLow`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectBlock {
    pub node: LodNode,
    pub high: bool,
}

/// The distant-object blocks the game keeps with the player at `player`
/// (`006fdfc0`): every node of the objects level within
/// `fBlockLoadDistance`, the high model past `fBlockLoadDistanceLow`.
/// Where a model doesn't exist nothing is drawn.
pub fn object_blocks(
    lod: &LodSettings,
    terrain: &TerrainSettings,
    player: [f32; 2],
) -> Vec<ObjectBlock> {
    nodes_at(lod, lod.object_level)
        .into_iter()
        .filter_map(|node| {
            let d = node.distance(player);
            (d < terrain.block_load_distance).then_some(ObjectBlock {
                node,
                high: d >= terrain.block_load_distance_low,
            })
        })
        .collect()
}

/// How far a quarter has faded from its parent's texture to its own,
/// `elapsed_ms` after it appeared (`006ff3f0`): `None` once the fade is
/// over (drawn with its own texture only).
pub fn texture_fade(elapsed_ms: f32, terrain: &TerrainSettings) -> Option<f32> {
    if elapsed_ms > terrain.texture_fade_ms || terrain.texture_fade_ms <= 0.0 {
        None
    } else {
        Some((elapsed_ms / terrain.texture_fade_ms).max(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `WastelandNV.dlodsettings`'s values.
    fn mojave() -> LodSettings {
        let mut bytes = Vec::new();
        for v in [4u32, 32, 128] {
            bytes.extend(v.to_le_bytes());
        }
        for v in [-64i16, -64, 63, 63] {
            bytes.extend(v.to_le_bytes());
        }
        bytes.extend(4u32.to_le_bytes());
        LodSettings::parse(&bytes).unwrap()
    }

    /// The recording's settings: `FalloutPrefs.ini`'s split distance.
    fn prefs() -> TerrainSettings {
        TerrainSettings::from_ini(|section, key| match (section, key) {
            ("TerrainManager", "fSplitDistanceMult") => Some(1.5),
            ("TerrainManager", "fBlockLoadDistance") => Some(125_000.0),
            ("TerrainManager", "fBlockLoadDistanceLow") => Some(50_000.0),
            ("General", "uGridsToLoad") => Some(5.0),
            _ => None,
        })
    }

    /// Where the Goodsprings recording's player stood: the camera's x, and
    /// its y + 3.74 (the first-person camera stands that far behind the
    /// player; the geomorph factor below gives it).
    const PLAYER: [f32; 2] = [-72151.5, 643.0];
    const CELL: (i32, i32) = (-18, 0);

    #[test]
    fn reads_the_settings_file() {
        let s = mojave();
        assert_eq!((s.min_level, s.max_level, s.root_level), (4, 32, 128));
        assert_eq!(s.root, (-64, -64));
        assert_eq!(s.last_cell, (63, 63));
        assert_eq!(s.object_level, 4);
        assert!(LodSettings::parse(&[0; 10]).is_none());
        assert_eq!(
            settings_path("WastelandNV"),
            "lodsettings\\wastelandnv.dlodsettings"
        );
    }

    #[test]
    fn nodes_measure_as_the_game_does() {
        let n = LodNode {
            level: 8,
            x: -16,
            y: 8,
        };
        assert_eq!(n.center(), [-49152.0, 49152.0]);
        assert!((n.radius() - 23170.475).abs() < 0.01);
        // Inside: 0; beside: the gap; off a corner: the corner's distance.
        assert_eq!(n.distance([-50000.0, 40000.0]), 0.0);
        assert_eq!(n.distance([-70000.0, 40000.0]), 4464.0);
        assert!((n.distance([-69536.0, 28768.0]) - 4000.0 * 2f32.sqrt()).abs() < 0.01);
        assert_eq!(
            n.children().map(|c| (c.level, c.x, c.y)),
            [(4, -16, 8), (4, -16, 12), (4, -12, 8), (4, -12, 12)]
        );
        let s = mojave();
        assert_eq!(
            s.parent(LodNode {
                level: 4,
                x: -12,
                y: 12
            }),
            Some(n)
        );
        assert_eq!(
            s.parent(LodNode {
                level: 32,
                x: 32,
                y: -64
            }),
            Some(LodNode {
                level: 64,
                x: 0,
                y: -64
            })
        );
        assert_eq!(s.parent(s.root_node()), None);
        // The textures run east to west: the western quarters start half
        // way across.
        let [sw, nw, se, ne] = n.children();
        assert_eq!(sw.offset_in_parent(n), [0.5, 0.0]);
        assert_eq!(nw.offset_in_parent(n), [0.5, 0.5]);
        assert_eq!(se.offset_in_parent(n), [0.0, 0.0]);
        assert_eq!(ne.offset_in_parent(n), [0.0, 0.5]);
        assert_eq!(
            n.land_model("WastelandNV"),
            "meshes\\landscape\\lod\\wastelandnv\\wastelandnv.level8.x-16.y8.nif"
        );
        assert_eq!(
            LodNode {
                level: 4,
                x: -8,
                y: 24
            }
            .object_model("WastelandNV", true),
            "meshes\\landscape\\lod\\wastelandnv\\blocks\\wastelandnv.level4.high.x-8.y24.nif"
        );
    }

    #[test]
    fn the_loaded_square_counts_its_edges() {
        // Cell -18,0 with 5 loaded: cells -20..=-16 and -2..=2, i.e. x from
        // -20 to -15 in cell units, edges included.
        let at = |level, x, y| LodNode { level, x, y };
        assert!(at(4, -16, 0).touches_loaded(CELL, 5));
        assert!(at(4, -24, 0).touches_loaded(CELL, 5)); // ends at -20
        assert!(at(4, -16, 3).touches_loaded(CELL, 5)); // starts at the edge
        assert!(!at(4, -16, 4).touches_loaded(CELL, 5));
        assert!(!at(4, -28, 0).touches_loaded(CELL, 5));
    }

    /// Every distant-land chunk the game drew in the Goodsprings recording
    /// (the main view, looking north): level, south-west cell.
    const RECORDED: [(u32, i32, i32); 37] = [
        (32, 32, 32),
        (32, 0, 32),
        (16, 0, 16),
        (16, -16, 48),
        (16, -16, 32),
        (16, -32, 48),
        (16, -32, 32),
        (8, -8, 24),
        (8, -8, 16),
        (8, -16, 24),
        (8, -16, 16),
        (8, -8, 8),
        (4, -12, 12),
        (4, -12, 8),
        (4, -16, 12),
        (4, -16, 8),
        (4, -12, 4),
        (4, -16, 4),
        (4, -16, 0),
        (8, -24, 24),
        (8, -24, 16),
        (8, -32, 24),
        (8, -32, 16),
        (4, -20, 12),
        (4, -20, 8),
        (4, -24, 12),
        (4, -24, 8),
        (4, -20, 4),
        (4, -20, 0),
        (4, -24, 4),
        (4, -24, 0),
        (8, -32, 8),
        (4, -28, 4),
        (32, -64, 32),
        (16, -48, 16),
        (8, -40, 8),
        (16, -64, 16),
    ];

    /// Whether a node's square reaches into the recorded camera's view seen
    /// from above: north ± 45.65° from the eye (the projection's x scale
    /// 0.977419), past the near plane. The square is clipped by the view's
    /// three sides; anything left is in view.
    fn in_view(n: LodNode) -> bool {
        let eye = [-72151.5f32, 639.2589];
        let x0 = n.x as f32 * CELL_SIZE - eye[0];
        let y0 = n.y as f32 * CELL_SIZE - eye[1];
        let s = n.size();
        let mut poly = vec![[x0, y0], [x0 + s, y0], [x0 + s, y0 + s], [x0, y0 + s]];
        let k = 1.0 / 0.977_419f32;
        let sides: [fn([f32; 2], f32) -> f32; 3] = [
            |p, k| p[0] + k * p[1],
            |p, k| k * p[1] - p[0],
            |p, _| p[1] - 5.0,
        ];
        for side in sides {
            let mut out = Vec::new();
            for i in 0..poly.len() {
                let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                let (da, db) = (side(a, k), side(b, k));
                if da >= 0.0 {
                    out.push(a);
                }
                if (da >= 0.0) != (db >= 0.0) {
                    let t = da / (da - db);
                    out.push([a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]);
                }
            }
            poly = out;
            if poly.is_empty() {
                return false;
            }
        }
        // Some area left (not just a touching edge).
        let area: f32 = (0..poly.len())
            .map(|i| {
                let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                a[0] * b[1] - b[0] * a[1]
            })
            .sum();
        area.abs() > 1.0
    }

    #[test]
    fn the_recorded_chunks_are_the_ones_drawn_in_view() {
        let drawn = drawn_nodes(&mojave(), &prefs(), PLAYER, CELL);
        let recorded: Vec<LodNode> = RECORDED
            .iter()
            .map(|&(level, x, y)| LodNode { level, x, y })
            .collect();
        for r in &recorded {
            assert!(
                drawn.iter().any(|d| d.node == *r),
                "{r:?} was drawn in the game"
            );
        }
        // Everything else drawn is out of that view.
        for d in &drawn {
            if !recorded.contains(&d.node) {
                assert!(
                    !in_view(d.node),
                    "{:?} would be in view here but the game didn't draw it",
                    d.node
                );
            }
        }
        // The whole map is covered once: the areas add up.
        let area: u64 = drawn.iter().map(|d| u64::from(d.node.level).pow(2)).sum();
        assert_eq!(area, 128 * 128);
    }

    #[test]
    fn with_the_exe_default_split_the_recorded_chunks_dont_come_out() {
        // The recording needs FalloutPrefs.ini's 1.5: at 0.75 the level-8
        // chunk south-west of the recorded level-4 ones isn't split.
        let drawn = drawn_nodes(&mojave(), &TerrainSettings::default(), PLAYER, CELL);
        assert!(!drawn.iter().any(|d| d.node
            == LodNode {
                level: 4,
                x: -16,
                y: 12
            }));
    }

    #[test]
    fn the_geomorph_factor_is_the_recorded_one() {
        let drawn = drawn_nodes(&mojave(), &prefs(), PLAYER, CELL);
        let factor = |level, x, y| {
            drawn
                .iter()
                .find(|d| d.node == LodNode { level, x, y })
                .map(|d| d.morph)
                .unwrap()
        };
        // Recorded `GeomorphParams.x`: 1, 1, 1, 0.41288, then 0.
        assert_eq!(factor(4, -16, 0), 1.0);
        assert_eq!(factor(4, -20, 0), 1.0);
        assert_eq!(factor(4, -24, 0), 1.0);
        assert!(
            (factor(4, -20, 4) - 0.41288).abs() < 2e-4,
            "{}",
            factor(4, -20, 4)
        );
        assert_eq!(factor(4, -16, 4), 0.0);
        assert_eq!(factor(8, -32, 8), 0.0);
        assert_eq!(factor(32, 0, 32), 0.0);
        // Morphing runs over the parent's radius × 0.65 to × 0.7.
        assert_eq!(
            morph(mojave().root_node(), &mojave(), &prefs(), PLAYER),
            1.0
        );
    }

    #[test]
    fn the_recorded_object_blocks_are_the_near_ones_and_the_high_ones() {
        let blocks = object_blocks(&mojave(), &prefs(), PLAYER);
        let block = |x, y| {
            blocks
                .iter()
                .find(|b| b.node == LodNode { level: 4, x, y })
                .copied()
        };
        // Drawn in the game: ordinary blocks to 49483 units, "high" ones of
        // (-4, 12) at 73913 and (-8, 24) at 105303.
        assert_eq!(block(-24, 12).map(|b| b.high), Some(false));
        assert_eq!(block(-4, 12).map(|b| b.high), Some(true));
        assert_eq!(block(-8, 24).map(|b| b.high), Some(true));
        // (-8, 8) at 50824 has only an ordinary model: too far for it.
        assert_eq!(block(-8, 8).map(|b| b.high), Some(true));
        // Past fBlockLoadDistance nothing.
        assert_eq!(block(-4, 28), None);
    }

    #[test]
    fn new_quarters_fade_from_the_parents_texture_over_a_second() {
        let t = TerrainSettings::default();
        assert_eq!(texture_fade(0.0, &t), Some(0.0));
        assert_eq!(texture_fade(250.0, &t), Some(0.25));
        assert_eq!(texture_fade(1000.0, &t), Some(1.0));
        assert_eq!(texture_fade(1000.5, &t), None);
    }
}
