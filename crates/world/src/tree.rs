//! Trees (`TREE`): SpeedTree models (`.spt`, grown by the `speedtree`
//! crate) placed like statics. Read from the game's loader (`0051bbc0`) and
//! its tree setup (`BSTreeModel`, `0066ac40`, `00666940`); the addresses
//! and the rest are in `%USERPROFILE%\nv-re\findings\trees.md`.
//!
//! A reference's tree is the base's `.spt` grown with one of its seeds
//! (`XSED` picks which), at the size `fTreeSizeConversion` makes of the
//! file's, with the record's leaf texture and `CNAM` values.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::image_space::ImageSpace;

const TREE: FourCC = FourCC::new(b"TREE");
const MODL: FourCC = FourCC::new(b"MODL");
const ICON: FourCC = FourCC::new(b"ICON");
const SNAM: FourCC = FourCC::new(b"SNAM");
const CNAM: FourCC = FourCC::new(b"CNAM");
const BNAM: FourCC = FourCC::new(b"BNAM");
const OBND: FourCC = FourCC::new(b"OBND");
const XSED: FourCC = FourCC::new(b"XSED");

/// A tree record.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeBase {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `MODL` as stored (`\WastelandShrub01.spt`); the game loads `Trees` +
    /// it ([`TreeBase::spt_path`]).
    pub model: String,
    /// `ICON`: the leaf texture, under `Textures\Trees\Leaves\`.
    pub leaves: Option<String>,
    /// `SNAM`: the seeds (zeros left out).
    pub seeds: Vec<u32>,
    /// `CNAM` (32 bytes at form +0x6c).
    pub curvature: f32,
    pub min_bud_angle: f32,
    pub max_bud_angle: f32,
    pub branch_dimming: f32,
    pub leaf_dimming: f32,
    pub shadow_radius: i32,
    /// The leaves' rocking and rustling speeds (form +0x84, +0x88).
    pub rock_speed: f32,
    pub rustle_speed: f32,
    /// `BNAM`: the billboard's width and height.
    pub billboard: [f32; 2],
    /// `OBND`.
    pub bounds: Option<([i16; 3], [i16; 3])>,
}

impl TreeBase {
    /// A tree record by form ID.
    pub fn load(order: &LoadOrder, id: FormId) -> Option<TreeBase> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == TREE)?;
        let record = rr.record().ok()?;
        let floats = |kind: FourCC, n: usize| -> Option<Vec<f32>> {
            let s = record.get(kind).filter(|s| s.data.len() >= n * 4)?;
            Some((0..n).map(|i| le_f32(&s.data, i * 4)).collect())
        };
        let seeds = record
            .get(SNAM)
            .map(|s| {
                s.data
                    .chunks_exact(4)
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                    .filter(|&v| v != 0)
                    .collect()
            })
            .unwrap_or_default();
        let c = record.get(CNAM).filter(|s| s.data.len() >= 32);
        let cf = |i: usize| c.map_or(0.0, |s| le_f32(&s.data, i * 4));
        let shadow = c.map_or(0, |s| le_u32(&s.data, 20) as i32);
        let bounds = record.get(OBND).filter(|s| s.data.len() >= 12).map(|s| {
            let v = |i: usize| i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]);
            ([v(0), v(1), v(2)], [v(3), v(4), v(5)])
        });
        Some(TreeBase {
            form_id: id,
            editor_id: record.editor_id(),
            model: record.get(MODL).map(|s| s.zstring()).unwrap_or_default(),
            leaves: record
                .get(ICON)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            seeds,
            curvature: cf(0),
            min_bud_angle: cf(1),
            max_bud_angle: cf(2),
            branch_dimming: cf(3),
            leaf_dimming: cf(4),
            // The loader: a negative radius becomes 128.
            shadow_radius: if shadow < 0 { 128 } else { shadow },
            rock_speed: cf(6),
            rustle_speed: cf(7),
            billboard: floats(BNAM, 2).map_or([0.0; 2], |b| [b[0], b[1]]),
            bounds,
        })
    }

    /// The `.spt`'s path in the archives: `Trees` + `MODL` (the record's
    /// path starts with its own backslash).
    pub fn spt_path(&self) -> String {
        let m = self.model.trim_start_matches(['\\', '/']);
        format!("trees\\{}", m.to_ascii_lowercase())
    }

    /// The leaf texture's path (`Textures\Trees\Leaves\` + `ICON`).
    pub fn leaf_texture_path(&self) -> Option<String> {
        self.leaves
            .as_ref()
            .map(|l| format!("textures\\trees\\leaves\\{}", l.to_ascii_lowercase()))
    }

    /// The seed a reference grows (`005692d0` → `0051baf0`): its `XSED`
    /// byte picks `SNAM[byte % count]`. With no `XSED` (byte 0xff) or no
    /// seeds the game takes a random one from the list (`0051c360`, its own
    /// generator): no reference in the game's files lacks `XSED`, so the
    /// first is used here.
    pub fn seed(&self, xsed: Option<u8>) -> i32 {
        if self.seeds.is_empty() {
            return 0;
        }
        let i = match xsed {
            Some(b) if b != 0xff => b as usize % self.seeds.len(),
            _ => 0,
        };
        self.seeds[i] as i32
    }
}

/// A branch texture's paths (`00666940`): `Textures\Trees\Branches\` + the
/// file's branch texture with its extension replaced by `dds`, and the
/// same with `_n` before the extension (the normal map).
pub fn branch_texture_paths(file: &str) -> (String, String) {
    let name = file.to_ascii_lowercase();
    let stem = match name.len() {
        n if n >= 3 => name[..n - 3].to_string(),
        _ => name,
    };
    let stem = stem.trim_end_matches('.').to_string();
    (
        format!("textures\\trees\\branches\\{stem}.dds"),
        format!("textures\\trees\\branches\\{stem}_n.dds"),
    )
}

/// A reference's `XSED` byte.
pub fn reference_seed(order: &LoadOrder, reference: FormId) -> Option<u8> {
    let record = order.get(reference)?.record().ok()?;
    record.get(XSED).and_then(|s| s.data.first().copied())
}

/// The game's settings the trees read: game settings with the exe's
/// defaults when no plugin sets them (none of the game's do), and the
/// INI's `[LOD] fLODMultTrees`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeSettings {
    /// `fTreeSizeConversion` (exe 10): the file's size × this.
    pub size_conversion: f32,
    /// `fTreeNearDistanceBase` × `fLODMultTrees`, `fTreeFarDistanceBase` ×
    /// `fLODMultTrees` (exe 2048 and 16384; × 0.5 here): the levels of
    /// detail's limits.
    pub near: f32,
    pub far: f32,
    /// `[SpeedTree] fTreeForceLeafDimming`, `fTreeForceBranchDimming`,
    /// `fTreeForceCS`, `fTreeForceMinBudAngle`, `fTreeForceMaxBudAngle`
    /// (−1: the record's).
    pub force_leaf_dimming: f32,
    pub force_branch_dimming: f32,
    pub force_curvature: f32,
    pub force_min_bud_angle: f32,
    pub force_max_bud_angle: f32,
    /// The wind settings (`fLeafRock…`, `fLeafRustle…`).
    pub rock_amount_sway: f32,
    pub rustle_amount_sway: f32,
    pub rock_speed_sway: f32,
    pub rustle_speed_sway: f32,
    pub rock_time_scale: f32,
    pub rustle_time_scale: f32,
}

impl TreeSettings {
    /// From the plugins' game settings and an INI lookup (`section`, `key`).
    pub fn load(order: &LoadOrder, ini: impl Fn(&str, &str) -> Option<f32>) -> TreeSettings {
        let gmst =
            |name: &str, exe: f32| crate::scripting::game_setting(order, name).unwrap_or(exe);
        // 0.5 when no INI sets it: `Fallout_default.ini`'s value (a guess:
        // the exe's own default isn't traced).
        let lod = ini("LOD", "fLODMultTrees").unwrap_or(0.5);
        let force = |key: &str| ini("SpeedTree", key).unwrap_or(-1.0);
        TreeSettings {
            size_conversion: gmst("fTreeSizeConversion", 10.0),
            near: gmst("fTreeNearDistanceBase", 2048.0) * lod,
            far: gmst("fTreeFarDistanceBase", 16384.0) * lod,
            force_leaf_dimming: force("fTreeForceLeafDimming"),
            force_branch_dimming: force("fTreeForceBranchDimming"),
            force_curvature: force("fTreeForceCS"),
            force_min_bud_angle: force("fTreeForceMinBudAngle"),
            force_max_bud_angle: force("fTreeForceMaxBudAngle"),
            rock_amount_sway: gmst("fLeafRockAmountSwayInfluence", 1.0),
            rustle_amount_sway: gmst("fLeafRustleAmountSwayInfluence", 1.0),
            rock_speed_sway: gmst("fLeafRockSpeedSwayInfluence", 1.0),
            rustle_speed_sway: gmst("fLeafRustleSpeedSwayInfluence", 1.0),
            rock_time_scale: gmst("fLeafRockTimeScale", 2.0),
            rustle_time_scale: gmst("fLeafRustleTimeScale", 0.5),
        }
    }

    /// The values `00666940` hands the library for a tree: (leaf dimming,
    /// branch dimming, curvature, bud angles). A forced value counts when
    /// it is within 0..1 (dimming), at least 0 (curvature), or both angles
    /// within 0..90 with min ≤ max; else the record's. The bud angles are
    /// only set when they then pass the same test.
    pub fn tree_values(&self, base: &TreeBase) -> TreeValues {
        let unit = |forced: f32, own: f32| {
            if (0.0..=1.0).contains(&forced) {
                forced
            } else {
                own
            }
        };
        let leaf_dimming = unit(self.force_leaf_dimming, base.leaf_dimming);
        let branch_dimming = unit(self.force_branch_dimming, base.branch_dimming);
        let curvature = if self.force_curvature >= 0.0 {
            self.force_curvature
        } else {
            base.curvature
        };
        let ok = |a: f32, b: f32| (0.0..=90.0).contains(&a) && (0.0..=90.0).contains(&b) && a <= b;
        let (a, b) = if ok(self.force_min_bud_angle, self.force_max_bud_angle) {
            (self.force_min_bud_angle, self.force_max_bud_angle)
        } else {
            (base.min_bud_angle, base.max_bud_angle)
        };
        TreeValues {
            leaf_dimming: (0.0..=1.0).contains(&leaf_dimming).then_some(leaf_dimming),
            branch_dimming: (0.0..=1.0)
                .contains(&branch_dimming)
                .then_some(branch_dimming),
            curvature: (curvature >= 0.0).then_some(curvature),
            bud_angles: ok(a, b).then_some([a, b]),
        }
    }
}

/// What a tree's record and the settings give the library (`None`: the
/// file's own stays).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeValues {
    pub leaf_dimming: Option<f32>,
    pub branch_dimming: Option<f32>,
    pub curvature: Option<f32>,
    pub bud_angles: Option<[f32; 2]>,
}

/// The image space value (and modifier track) of the tree dimmer.
pub const TREE_DIMMER: usize = 13;

/// The image space's tree dimmer (`DNAM` float 13; 1 in
/// `NVDefaultExterior`): the leaf shader's `SunDimmer.x` multiplies the
/// sun's light on leaves by it (`00bb2d10`; recorded 1 at Goodsprings).
pub fn tree_dimmer(space: &ImageSpace) -> Option<f32> {
    space
        .values
        .get(TREE_DIMMER)
        .copied()
        .filter(|v| v.is_finite() && (0.0..=100.0).contains(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_textures_swap_the_extension() {
        let (t, n) = branch_texture_paths("WastelandShrub01Bark.tga");
        assert_eq!(t, "textures\\trees\\branches\\wastelandshrub01bark.dds");
        assert_eq!(n, "textures\\trees\\branches\\wastelandshrub01bark_n.dds");
    }

    fn base() -> TreeBase {
        TreeBase {
            form_id: FormId(1),
            editor_id: None,
            model: "\\WastelandShrub01.spt".into(),
            leaves: Some("WastelandShrub01Foliage.dds".into()),
            seeds: vec![171677, 5, 9],
            curvature: 2.0,
            min_bud_angle: 5.0,
            max_bud_angle: 85.0,
            branch_dimming: 0.5,
            leaf_dimming: 0.05,
            shadow_radius: 64,
            rock_speed: 0.07,
            rustle_speed: 0.35,
            billboard: [175.0; 2],
            bounds: None,
        }
    }

    #[test]
    fn paths_and_seeds() {
        let b = base();
        assert_eq!(b.spt_path(), "trees\\wastelandshrub01.spt");
        assert_eq!(
            b.leaf_texture_path().as_deref(),
            Some("textures\\trees\\leaves\\wastelandshrub01foliage.dds")
        );
        assert_eq!(b.seed(Some(0)), 171677);
        assert_eq!(b.seed(Some(4)), 5);
        assert_eq!(b.seed(Some(0xff)), 171677);
    }

    #[test]
    fn forced_values_only_within_their_ranges() {
        let mut s = TreeSettings {
            size_conversion: 10.0,
            near: 1024.0,
            far: 8192.0,
            force_leaf_dimming: -1.0,
            force_branch_dimming: 0.25,
            force_curvature: -1.0,
            force_min_bud_angle: 50.0,
            force_max_bud_angle: 10.0,
            rock_amount_sway: 1.0,
            rustle_amount_sway: 1.0,
            rock_speed_sway: 1.0,
            rustle_speed_sway: 1.0,
            rock_time_scale: 2.0,
            rustle_time_scale: 0.5,
        };
        let v = s.tree_values(&base());
        assert_eq!(v.leaf_dimming, Some(0.05));
        assert_eq!(v.branch_dimming, Some(0.25));
        assert_eq!(v.curvature, Some(2.0));
        // Min above max: the record's angles.
        assert_eq!(v.bud_angles, Some([5.0, 85.0]));
        s.force_min_bud_angle = 0.0;
        s.force_max_bud_angle = 30.0;
        assert_eq!(s.tree_values(&base()).bud_angles, Some([0.0, 30.0]));
    }
}
