//! Trees, ready to draw: a `TREE` record's `.spt` grown with a reference's
//! seed as the game's SpeedTreeRT does (`speedtree`), turned into the
//! branch and leaf meshes of each level of detail the game uploads
//! (`speedtree::mesh`), with the textures `BSTreeModel` gives them
//! (`00666940`): the branches `Textures\Trees\Branches\<the file's bark>.dds`
//! and its `_n` normal map, the leaves the record's `ICON` under
//! `Textures\Trees\Leaves\`.

use esm::FormId;
use speedtree::lod::LeafFade;
use speedtree::mesh::{branch_meshes, leaf_base, leaf_meshes, BranchMesh, LeafMesh, MeshSettings};
use speedtree::tree::{grow, Settings};
use world::tree::{branch_texture_paths, TreeBase, TreeSettings};

use crate::{Game, TextureData};

/// The `LeafBase` constants the leaf shader has room for (`c34`–`c81`).
pub const LEAF_BASE_SIZE: usize = 48;

/// One tree base grown with one seed.
pub struct TreeModel {
    pub base: TreeBase,
    pub seed: i32,
    /// The `.spt`'s path, for messages.
    pub path: String,
    /// The size it grew to (the file's × `fTreeSizeConversion`, ± the
    /// variance).
    pub size: f32,
    /// Per level of detail; a level with no branches is empty.
    pub branches: Vec<BranchMesh>,
    pub leaves: Vec<LeafMesh>,
    /// `LeafBase` (level 0's card corners, padded to [`LEAF_BASE_SIZE`]).
    pub leaf_base: Vec<[f32; 4]>,
    pub branch_texture: Option<TextureData>,
    pub branch_normal_map: Option<TextureData>,
    pub leaf_texture: Option<TextureData>,
    /// `LeafLighting.y`: the curvature the leaves' normals bend by.
    pub curvature: f32,
    /// `RockParams.z`, `RustleParams.z`: the file's 21001 and 21000.
    pub rock_amount: f32,
    pub rustle_amount: f32,
    /// The record's rocking and rustling speeds (× the wind's clocks for
    /// `RockParams.y`, `RustleParams.y`).
    pub rock_speed: f32,
    pub rustle_speed: f32,
    /// How the leaves change level (the file's 9002–9004, 16014).
    pub fade: LeafFade,
    /// How far any vertex can reach from the tree's origin (cards, wind
    /// turns about the origin included), unscaled.
    pub radius: f32,
    /// The trunk's base diameter and its first branch's height (the
    /// library's branch geometry +0x18, +0x1c).
    pub trunk: (f32, f32),
}

impl TreeModel {
    pub fn branch_levels(&self) -> u16 {
        self.branches.len() as u16
    }

    pub fn leaf_levels(&self) -> u16 {
        self.leaves.len() as u16
    }
}

/// A placed tree: where, and which grown model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedTree {
    pub reference: FormId,
    pub base: FormId,
    /// Game units.
    pub position: [f32; 3],
    /// The reference's angles (radians, `DATA`).
    pub rotation: [f32; 3],
    pub scale: f32,
    /// The reference's `XSED` byte.
    pub xsed: Option<u8>,
}

impl PlacedTree {
    /// The tree's model matrix in game space (column-major), placed like
    /// any reference (`world::RotationConvention::DEFAULT`).
    pub fn matrix(&self) -> [f32; 16] {
        crate::column_major(&world::RotationConvention::DEFAULT.transform(
            self.position,
            self.rotation,
            self.scale,
        ))
    }
}

/// The trees among a cell's placed objects.
pub fn placed_trees(order: &esm::LoadOrder, cell: &world::LoadedCell) -> Vec<PlacedTree> {
    cell.objects
        .iter()
        .filter(|o| o.base_type == esm::FourCC::new(b"TREE"))
        .map(|o| PlacedTree {
            reference: o.form_id,
            base: o.base,
            position: o.position,
            rotation: o.rotation,
            scale: o.scale,
            xsed: world::tree::reference_seed(order, o.form_id),
        })
        .collect()
}

impl Game {
    /// The trees placed on a worldspace's square (with what scripts have
    /// enabled and disabled).
    pub fn square_trees(
        &self,
        grid: &world::WorldGrid,
        square: (i32, i32),
        disabled: &world::Disabled,
    ) -> Result<Vec<PlacedTree>, crate::Error> {
        Ok(match grid.load_square_now(&self.order, square, disabled)? {
            Some(cell) => placed_trees(&self.order, &cell),
            None => Vec::new(),
        })
    }

    /// The tree settings (game settings and the INI's).
    pub fn tree_settings(&self) -> TreeSettings {
        TreeSettings::load(&self.order, |section, key| {
            self.settings.float(section, key)
        })
    }

    /// A tree base grown with `seed` (see [`TreeBase::seed`]); `None` when
    /// the record or its `.spt` can't be read.
    pub fn tree_model(
        &self,
        base: FormId,
        seed: i32,
        settings: &TreeSettings,
    ) -> Option<TreeModel> {
        let base = TreeBase::load(&self.order, base)?;
        let path = base.spt_path();
        let bytes = self.assets.read(&path).ok()??;
        let spt = speedtree::SptFile::parse(&bytes).ok()?;
        Some(grow_model(base, seed, &path, &spt, settings, |p| {
            let bytes = self.assets.read(p).ok()??;
            TextureData::from_dds(p.to_string(), bytes).ok()
        }))
    }
}

/// [`Game::tree_model`] once the file is read: the setup `0066ac40` and
/// `00666940` make before `Compute(NULL, seed, 1)`.
pub fn grow_model(
    base: TreeBase,
    seed: i32,
    path: &str,
    spt: &speedtree::SptFile,
    settings: &TreeSettings,
    texture: impl Fn(&str) -> Option<TextureData>,
) -> TreeModel {
    let values = settings.tree_values(&base);
    let mut s = Settings::from_file(spt);
    s.seed = seed;
    // `SetTreeSize(size × conversion, variance × conversion)`.
    s.size = spt.size * settings.size_conversion;
    s.size_variance = spt.size_variance * settings.size_conversion;
    if let Some(v) = values.leaf_dimming {
        s.leaf_dimming = v;
    }
    if let Some(v) = values.branch_dimming {
        s.branch_dimming = v;
    }
    if let Some(a) = values.bud_angles {
        s.bud_angles = a;
    }
    // Fewer than four leaf maps rock in two groups, else one.
    s.rocking_groups = if spt.leaves.textures.len() < 4 { 2 } else { 1 };
    s.wind_matrices = (0, 4);
    let tree = grow(spt, &s);
    let mesh_settings = MeshSettings {
        rocking_groups: s.rocking_groups,
        wind_matrices: s.wind_matrices,
        leaf_lod_step: spt.lod.value_24,
    };
    let branches = branch_meshes(&tree);
    let leaves = leaf_meshes(spt, &tree, &mesh_settings);
    let mut base_table = leaf_base(&tree, s.rocking_groups);
    base_table.resize(LEAF_BASE_SIZE.max(base_table.len()), [0.0; 4]);
    base_table.truncate(LEAF_BASE_SIZE);

    // The farthest a vertex reaches: positions, plus the largest card at
    // the largest level's scale for leaves.
    let card = base_table
        .iter()
        .map(|c| (c[1] * c[1] + c[2] * c[2]).sqrt())
        .fold(0.0f32, f32::max);
    let mut radius = 0.0f32;
    let len = |p: &[f32; 3]| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    for b in &branches {
        for p in &b.positions {
            radius = radius.max(len(p));
        }
    }
    for l in &leaves {
        for (p, blend) in l.positions.iter().zip(&l.blend) {
            radius = radius.max(len(p) + card * blend[3]);
        }
    }

    let (bark, bark_normal) = branch_texture_paths(&spt.branch_texture);
    TreeModel {
        seed,
        path: path.to_string(),
        size: tree.size,
        branches,
        leaves,
        leaf_base: base_table,
        branch_texture: texture(&bark),
        branch_normal_map: texture(&bark_normal),
        leaf_texture: base.leaf_texture_path().and_then(|p| texture(&p)),
        // A negative CNAM curvature leaves the model's own, 0 (none of the
        // game's trees has one).
        curvature: values.curvature.unwrap_or(0.0),
        // Every shipped file has 21000 and 21001; without them the
        // library's defaults aren't traced (0 here).
        rock_amount: spt.wind_40.unwrap_or(0.0),
        rustle_amount: spt.wind_3c.unwrap_or(0.0),
        rock_speed: base.rock_speed,
        rustle_speed: base.rustle_speed,
        fade: LeafFade {
            mode: spt.lod.mode,
            width: spt.lod.value_1c,
            // Every shipped file has 16014 (0.25); its default isn't traced.
            start: spt.value_16014.unwrap_or(0.25),
            exponent: spt.lod.value_20,
        },
        radius,
        trunk: (
            tree.geometry.trunk_diameter,
            tree.geometry.trunk_first_branch,
        ),
        base,
    }
}
