//! Which level of detail of a tree the game draws each frame (`00669a10`
//! for every tree node, with the library's `ComputeLodLevel` `00b2bd40`,
//! the branch level `00b08530`, and `00b0ad00` / `00b0c8d0` for the
//! leaves' cross-fade).
//!
//! Checked against the Goodsprings recording: two shrubs at 6,216 and
//! 6,087 units (camera to tree) drew only their second leaf level, with
//! alpha references 239 and 217 — exactly these rules with the library's
//! fast square root for the distance (the true distance gives 188 and 163).

use crate::math::{fast_distance, V3};

/// The library's alpha reference for a fully shown level (CSpeedTreeRT
/// +0x44, a byte: 84).
pub const ALPHA_SHOWN: u8 = 84;

/// What the library is told (`0066ac40`): `SetLodLimits(fTreeNearDistanceBase
/// × fLODMultTrees, fTreeFarDistanceBase × fLODMultTrees)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LodLimits {
    pub near: f32,
    pub far: f32,
}

/// The level of detail at a distance (`00b2bd40`): `1 − (d − near) / (far −
/// near)`, clamped to 0..1, `d` the fast distance from the camera to the
/// tree's position.
pub fn lod_level(camera: V3, tree: V3, limits: LodLimits) -> f32 {
    let d = fast_distance(camera, tree);
    let v = 1.0 - (d - limits.near) / (limits.far - limits.near);
    if v <= 1.0 {
        if v < 0.0 {
            0.0
        } else {
            v
        }
    } else {
        1.0
    }
}

/// The branch level drawn at `lod` (`00b08530`): `trunc((1 − lod) × n)`,
/// `n` itself becoming `n − 1`.
pub fn branch_level(lod: f32, levels: u16) -> u16 {
    let i = ((1.0 - lod) * levels as f32) as i32 as u16;
    if i == levels {
        i.wrapping_sub(1)
    } else {
        i
    }
}

/// One level shown, with its alpha reference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shown {
    pub level: u16,
    pub alpha: f32,
}

/// `00b0c8d0`: the two levels a cross-fade shows at `lod` among `levels`
/// (the leaf levels + 1 for the billboard), `width` the file's 9003,
/// `start` CSpeedTreeRT +0x28, `exponent` 9004, `base` [`ALPHA_SHOWN`]:
/// step = 1/n; i = (1 − lod)/step rounded half up; f = (1 − lod) − i·step.
/// At the ends (i = 0 or n) or |f| > width: one level, min(trunc((1 − lod)
/// n), n − 1), at `base`, the second none at 255. Otherwise t = 1 − (width
/// − f)/(2 width): level i − 1 at 171 × (1 − a)^exponent + base with a =
/// min(1 − (t − start)/(1 − start), 1), level i at 171 × (1 − b)^exponent
/// + base with b = min(t/(1 − start), 1).
pub fn cross_fade(
    lod: f32,
    levels: u16,
    width: f32,
    start: f32,
    exponent: f32,
    base: f32,
) -> [Option<Shown>; 2] {
    // The game's main thread runs the x87 unit at single precision (the
    // Direct3D default), so these are plain float sums.
    let n = levels as i32;
    let step = 1.0 / n as f32;
    let x = (1.0 - lod) / step;
    let mut i = x as i32;
    if 0.5 <= x - i as f32 {
        i += 1;
    }
    let i = i as u16;
    let f = (1.0 - lod) - i as f32 * step;
    if i == 0 || i == levels || f.abs() > width {
        let mut a = ((1.0 - lod) * levels as f32) as i32;
        if n - 1 <= a {
            a = n - 1;
        }
        return [
            Some(Shown {
                level: a as u16,
                alpha: base,
            }),
            None,
        ];
    }
    let t = 1.0 - (width - f) / (width * 2.0);
    let mut a = 1.0 - (t - start) / (1.0 - start);
    if crate::math::not_less(a, 1.0) {
        a = 1.0;
    }
    let mut b = t / (1.0 - start);
    if crate::math::not_less(b, 1.0) {
        b = 1.0;
    }
    let fade = |v: f32| ((1.0 - v) as f64).powf(exponent as f64) as f32 * (255.0 - base) + base;
    [
        Some(Shown {
            level: i - 1,
            alpha: fade(a),
        }),
        Some(Shown {
            level: i,
            alpha: fade(b),
        }),
    ]
}

/// How a tree's leaves change level (the file's 9002, CSpeedTreeRT +0x18).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafFade {
    pub mode: i32,
    pub width: f32,
    pub start: f32,
    pub exponent: f32,
}

/// What a tree node shows this frame (`00669a10` with `00b0ad00`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Visible {
    /// The branch level and its alpha reference (a byte; 255 never shows).
    pub branches: Option<(u16, u8)>,
    /// The leaf levels shown, each with its alpha reference.
    pub leaves: Vec<(u16, u8)>,
}

/// The game's `ROUND` (x87 round to nearest, ties to even) to a byte.
fn byte(v: f32) -> u8 {
    let r = v.round();
    let r = if (v - v.trunc()).abs() == 0.5 {
        2.0 * (v / 2.0).round()
    } else {
        r
    };
    (r as i64 & 0xff) as u8
}

/// What's drawn at `lod`, with the manager's billboard flag on
/// (`00664440` sets `011f8b78`: the billboard counts as one more leaf level;
/// the game draws no billboard for New Vegas's trees: the recording's two
/// shrubs, whose second shown level was the billboard, drew none).
///
/// At `lod` 0 or less nothing. Branches: level [`branch_level`], its
/// alpha the first leaf level's when that is the last leaf level (n − 2),
/// 255 (hidden) when the first is the billboard, else [`ALPHA_SHOWN`].
/// Leaves by `fade.mode`: 1 the cross-fade's levels below the leaf count,
/// 3 always level 0, else level `min(trunc((1 − lod)(n + 1)), n)` if it is a
/// leaf level (`00b08610`). A level whose alpha rounds to 255 isn't shown
/// (`0066bdb0`).
pub fn visible(lod: f32, branch_levels: u16, leaf_levels: u16, fade: LeafFade) -> Visible {
    let mut out = Visible::default();
    if lod <= 0.0 {
        return out;
    }
    let n = leaf_levels + 1;
    let base = ALPHA_SHOWN as f32;
    let [a, b] = cross_fade(lod, n, fade.width, fade.start, fade.exponent, base);
    let branch_alpha = match a {
        Some(s) if s.level == n - 2 => s.alpha,
        Some(s) if s.level == n - 1 => 255.0,
        _ => base,
    };
    if branch_levels > 0 {
        let level = branch_level(lod, branch_levels);
        let alpha = byte(branch_alpha);
        if alpha != 0xff {
            out.branches = Some((level, alpha));
        }
    }
    let mut push = |level: u16, alpha: f32| {
        let a = byte(alpha);
        if level < leaf_levels && a != 0xff {
            out.leaves.push((level, a));
        }
    };
    match fade.mode {
        1 => {
            for s in [a, b].into_iter().flatten() {
                push(s.level, s.alpha);
            }
        }
        3 => push(0, base),
        _ => {
            let mut i = ((1.0 - lod) * n as f32) as i32 as u16;
            if i == n {
                i -= 1;
            }
            push(i, base);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Goodsprings recording's shrubs (`WastelandShrub01`: two leaf
    /// levels, 9003 = 0.1, 9004 = 1.5, start 0.25; limits 1024 and 8192
    /// with `fLODMultTrees` 0.5): camera-relative positions from their
    /// `ModelViewProj`, alpha references 239 and 217, only leaf level 1.
    #[test]
    fn the_recorded_shrubs() {
        let limits = LodLimits {
            near: 1024.0,
            far: 8192.0,
        };
        let fade = LeafFade {
            mode: 1,
            width: 0.1,
            start: 0.25,
            exponent: 1.5,
        };
        for (p, alpha) in [
            ([-3484.1, 5112.7, 599.9], 239u8),
            ([-3329.4, 5072.4, 489.0], 217),
        ] {
            let lod = lod_level([0.0; 3], p, limits);
            let v = visible(lod, 2, 2, fade);
            assert_eq!(v.leaves.len(), 1, "{v:?}");
            assert_eq!(v.leaves[0].0, 1);
            assert!(
                (v.leaves[0].1 as i32 - alpha as i32).abs() <= 1,
                "{v:?} vs {alpha}"
            );
            // Branch level 1 of the shrub is empty; the alpha is the fade's.
            assert_eq!(v.branches.map(|b| b.0), Some(1));
        }
    }

    #[test]
    fn near_trees_show_everything_at_full() {
        let limits = LodLimits {
            near: 1024.0,
            far: 8192.0,
        };
        let fade = LeafFade {
            mode: 1,
            width: 0.1,
            start: 0.25,
            exponent: 1.5,
        };
        let v = visible(lod_level([0.0; 3], [500.0, 0.0, 0.0], limits), 2, 2, fade);
        assert_eq!(v.branches, Some((0, ALPHA_SHOWN)));
        assert_eq!(v.leaves, vec![(0, ALPHA_SHOWN)]);
        // Beyond the far limit nothing.
        let v = visible(lod_level([0.0; 3], [9000.0, 0.0, 0.0], limits), 2, 2, fade);
        assert_eq!(v, Visible::default());
    }

    #[test]
    fn cross_fade_shows_two_levels_in_the_band() {
        // (1 − lod) just past a third of the way: levels 0 and 1 together.
        let [a, b] = cross_fade(1.0 - 0.36, 3, 0.1, 0.25, 1.5, 84.0);
        assert_eq!(a.map(|s| s.level), Some(0));
        assert_eq!(b.map(|s| s.level), Some(1));
        assert!(a.unwrap().alpha > 84.0 && b.unwrap().alpha < 255.0);
    }
}
