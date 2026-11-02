//! The game's collision filter (`bhkCollisionFilter`): which Havok layers
//! touch which, as FalloutNV.exe builds its table at startup and asks it for
//! every pair of bodies, phantoms and ray casts.
//!
//! A body's filter word (`hkpCollidable` `+0x1c`, a ray's `+0x24`) is its
//! layer in the low 7 bits, a "no collision" flag (0x4000), a "linked
//! group" flag (0x8000) and its ragdoll part number in bits 8–12, and a
//! system group in the high 16 bits (the NIF's `HavokFilter` without the
//! group). Bodies of different groups touch when the table says their
//! layers do; parts of one ragdoll (one group) by the part table.

/// Number of layers (`nif::collision::layers::NAMES`).
pub const LAYERS: usize = 43;

/// Layers named in the rules here (the game's names, `011b0810`).
pub mod layer {
    pub const UNIDENTIFIED: u8 = 0;
    pub const STATIC: u8 = 1;
    pub const ANIM_STATIC: u8 = 2;
    pub const TRANSPARENT: u8 = 3;
    pub const CLUTTER: u8 = 4;
    pub const WEAPON: u8 = 5;
    pub const PROJECTILE: u8 = 6;
    pub const BIPED: u8 = 8;
    pub const PROPS: u8 = 10;
    pub const TERRAIN: u8 = 13;
    pub const CHAR_CONTROLLER: u8 = 30;
    pub const CUSTOMPICK2: u8 = 40;
}

/// The layer table and the ragdoll part table (`01267f20`, 43 × 64 bits:
/// bit `b` of row `a` set means layer `a` touches layer `b`; `01268078`,
/// 32 × 32 bits by part number).
#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub layers: [u64; LAYERS],
    pub parts: [u32; 32],
}

/// `0x01267f20 + 8 × layer (+ 4 for the high half)`.
fn word(rows: &mut [u64; LAYERS], addr: u32) -> (&mut u64, u32) {
    let off = addr - 0x0126_7f20;
    (&mut rows[(off / 8) as usize], (off % 8) / 4 * 32)
}

fn and(rows: &mut [u64; LAYERS], addr: u32, mask: u32) {
    let (row, shift) = word(rows, addr);
    let keep = !(u64::from(!mask) << shift);
    *row &= keep;
}

fn or(rows: &mut [u64; LAYERS], addr: u32, bits: u32) {
    let (row, shift) = word(rows, addr);
    *row |= u64::from(bits) << shift;
}

/// Layers `a` and `b` stop touching each other (`00c827f0` with `false`).
// Translated from 00c827f0 (decompiled, FalloutNV.exe 1.4.0.525)
fn apart(rows: &mut [u64; LAYERS], a: usize, b: usize) {
    rows[a] &= !(1u64 << b);
    rows[b] &= !(1u64 << a);
}

/// Layer `row` touches exactly the layers set in `mask`, both ways.
// Translated from 00c82870 (decompiled, FalloutNV.exe 1.4.0.525)
fn set_row(rows: &mut [u64; LAYERS], row: usize, mask: u64) {
    rows[row] = mask;
    for (r, other) in rows.iter_mut().enumerate() {
        if mask >> r & 1 == 1 {
            *other |= 1u64 << row;
        } else {
            *other &= !(1u64 << row);
        }
    }
}

impl Filter {
    /// [`Filter::game`], built once.
    pub fn shared() -> &'static Filter {
        static FILTER: std::sync::OnceLock<Filter> = std::sync::OnceLock::new();
        FILTER.get_or_init(Filter::game)
    }

    /// The tables as the game fills them at startup.
    // Translated from 00c828f0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn game() -> Filter {
        let mut parts = [0u32; 32];
        for (i, bits) in [
            (1, 0x4230c0),
            (2, 0x14030c0),
            (3, 0x14030c0),
            (4, 0x14230c0),
            (5, 0x403000),
            (6, 0x42791e),
            (7, 0x41ff1e),
            (8, 0x145f0c0),
            (9, 0x145e080),
            (10, 0x144e080),
            (11, 0x4000c0),
            (12, 0x4241fe),
            (13, 0x45c7fe),
            (14, 0x14437c0),
            (15, 0x1442780),
            (16, 0x1442380),
            (17, 0x401052),
            (18, 0x41e700),
            (20, 0x400000),
            (21, 0x400000),
            (22, 0x37fffe),
            (24, 0x1c71c),
        ] {
            parts[i] |= bits;
        }
        let mut r = [u64::MAX; LAYERS];
        let rows = &mut r;
        and(rows, 0x0126_8028, 0xfff7_ffff);
        and(rows, 0x0126_8040, 0xfff7_ffff);
        and(rows, 0x0126_8070, 0xfff7_ffff);
        and(rows, 0x0126_7fb8, 0x2bfa_6637);
        and(rows, 0x0126_7fc4, 0xffff_ffdf);
        and(rows, 0x0126_7fc0, 0xfffa_76ff);
        and(rows, 0x0126_7fbc, 0xffff_fb85);
        and(rows, 0x0126_7fc8, 0x43dc_0000);
        or(rows, 0x0126_7fc8, 0x4000_0000);
        or(rows, 0x0126_8028, 0x0020_0000);
        and(rows, 0x0126_7f20, 0xffdf_ffff);
        and(rows, 0x0126_7ff8, 0xffdf_ffff);
        and(rows, 0x0126_7fcc, 0xffff_fc17);
        or(rows, 0x0126_7fcc, 2);
        and(rows, 0x0126_8010, 0xdff3_feff);
        or(rows, 0x0126_8010, 0x0020_0800);
        and(rows, 0x0126_7fb0, 0x0be0_0001);
        and(rows, 0x0126_8018, 0xffd3_ffff);
        and(rows, 0x0126_8038, 0xffd3_ffff);
        and(rows, 0x0126_8048, 0xffc3_ffff);
        and(rows, 0x0126_8050, 0xffd3_ffff);
        and(rows, 0x0126_8058, 0xffdb_ffff);
        and(rows, 0x0126_8060, 0xffdb_ffff);
        and(rows, 0x0126_8068, 0xffdb_ffff);
        and(rows, 0x0126_7fb4, 0xffff_fc17);
        and(rows, 0x0126_7f28, 0xffdb_7fff);
        and(rows, 0x0126_7f40, 0xffdb_7eff);
        and(rows, 0x0126_7f30, 0xffdb_7fff);
        and(rows, 0x0126_7f38, 0xffd3_7fff);
        and(rows, 0x0126_7ff0, 0xffd3_7fff);
        and(rows, 0x0126_8000, 0xffd3_7fff);
        and(rows, 0x0126_7f48, 0xffdb_7eff);
        and(rows, 0x0126_7f50, 0xffd3_7fff);
        and(rows, 0x0126_7f58, 0xffd3_7fff);
        and(rows, 0x0126_7f68, 0xffdb_7fff);
        and(rows, 0x0126_7f70, 0xffdb_7eff);
        and(rows, 0x0126_7f80, 0xffd3_7fff);
        and(rows, 0x0126_7f88, 0xffdb_7fff);
        and(rows, 0x0126_7f90, 0xffdb_7fff);
        and(rows, 0x0126_7f60, 0x9fc3_7bcf);
        and(rows, 0x0126_8008, 0xbfdb_7eff);
        and(rows, 0x0126_7f78, 0xffc3_7fff);
        or(rows, 0x0126_7f78, 0x4000_0000);
        and(rows, 0x0126_7fa8, 0xffdb_7fff);
        and(rows, 0x0126_7fa0, 0xffc3_7fff);
        and(rows, 0x0126_7f98, 0xcbc0_0001);
        for b in [0x1e, 0x1f, 0x23, 0x25, 0x26, 0x27, 0x28, 0x29] {
            apart(rows, 0xf, b);
        }
        for b in [1, 2, 3, 0x1a, 0x1c, 4, 5, 6, 7, 8, 0x1d, 9, 10] {
            apart(rows, 0x21, b);
        }
        or(rows, 0x0126_8028, 0x800);
        or(rows, 0x0126_7f7c, 2);
        let pairs: &[(usize, &[usize])] = &[
            (
                0x21,
                &[
                    0xc, 0xd, 0xe, 0x21, 0x11, 0xf, 0x10, 0x1e, 0x1f, 0x23, 0x25, 0x26, 0x27, 0x28,
                    0x29,
                ],
            ),
            (0x23, &[5, 7, 0xc, 0x23, 0xb, 6]),
            (0x24, &[7, 0xc, 0xb]),
            (0x25, &[3, 0x1a, 0x1c, 7, 0xb, 0xc, 0x1e]),
            (
                0x26,
                &[
                    0x1f, 4, 5, 6, 7, 8, 0x1d, 0x1e, 10, 0x23, 0xc, 0xe, 0x10, 0x29, 0x24, 0x25,
                ],
            ),
            (
                0x1f,
                &[
                    0x1f, 1, 4, 5, 6, 7, 8, 0x1d, 9, 10, 0xb, 0xd, 0xf, 0x11, 0x23, 0x29, 0x24,
                    0x25,
                ],
            ),
            (
                0x29,
                &[
                    3, 0x1a, 0x1c, 7, 0xb, 0x23, 0x24, 0x25, 0x27, 0x28, 0x26, 0x29,
                ],
            ),
            (6, &[7, 0x10, 3]),
            (
                0xc,
                &[
                    1, 2, 3, 0x1a, 0x1c, 9, 10, 0xb, 0xc, 0xd, 0xf, 0x10, 0x11, 0x16, 0x20, 0x26,
                    0x2a, 0x25, 0x24, 0x29,
                ],
            ),
            (
                0x16,
                &[
                    1, 2, 3, 0x1a, 4, 5, 6, 7, 8, 0x1d, 9, 10, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11,
                    0x12, 0x13, 0x14, 0x16, 0x17, 0x19, 0x1f, 0x20, 0x21, 0x23, 0x24, 0x25, 0x26,
                    0x29, 0x2a, 0x1b, 0x22, 0,
                ],
            ),
            (
                0x17,
                &[
                    1, 2, 3, 0x1a, 0x1c, 4, 5, 7, 8, 0x1d, 9, 10, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10,
                    0x11, 0x12, 0x13, 0x14, 0x16, 0x17, 0x19, 0x1e, 0x1f, 0x20, 0x21, 0x23, 0x24,
                    0x25, 0x26, 0x29, 0x2a,
                ],
            ),
            (
                0x18,
                &[
                    1, 2, 3, 0x1a, 0x1c, 4, 5, 7, 8, 0x1d, 9, 10, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10,
                    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1f, 0x20, 0x21, 0x23,
                    0x24, 0x25, 0x26, 0x2a,
                ],
            ),
            (
                0x19,
                &[
                    3, 0x1a, 0x1c, 6, 7, 8, 0xc, 0xf, 0x10, 0x12, 0x1e, 0x1f, 0x23, 0x24, 0x25,
                    0x26, 0x2a, 0xb,
                ],
            ),
            (7, &[7, 0x10, 3, 0x1a, 0x1c]),
            (0x2a, &[4, 6, 5, 0xb, 0x1d, 0x1f, 0x26, 0x23, 0x29, 0x25]),
            (0x1b, &[0x13, 0x19, 0x1f, 0x20, 0x21, 0x23, 0x25, 0x2a, 5]),
            (3, &[0, 0xd]),
        ];
        for &(a, bs) in pairs {
            for &b in bs {
                apart(rows, a, b);
            }
        }
        set_row(rows, 0x22, 0x1000_0004);
        // Bits past the last layer mean nothing.
        for row in r.iter_mut() {
            *row &= (1u64 << LAYERS) - 1;
        }
        Filter { layers: r, parts }
    }

    /// Whether layer `a`'s row has layer `b` (`a` is the asking side: the
    /// first body, or the ray).
    pub fn layers_touch(&self, a: u8, b: u8) -> bool {
        usize::from(a) < LAYERS && b < 64 && self.layers[usize::from(a)] >> b & 1 == 1
    }

    /// Whether two filter words collide (`a` first: a ray's word, or the
    /// first body's). `linked_parts` stands for `00624070` on both words:
    /// each body's layer is the biped (8) or dead-biped (29) one.
    // Translated from 00c84740 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn collides(&self, a: u32, b: u32, linked_parts: bool) -> bool {
        let la = (a & 0x7f) as u8;
        if la != layer::CUSTOMPICK2 && (a >> 14 & 1 == 1 || b >> 14 & 1 == 1) {
            return false;
        }
        if a & 0xffff_0000 == 0 || b & 0xffff_0000 == 0 {
            return true;
        }
        let groups_differ = (a ^ b) & 0xffff_0000 != 0;
        let lb = (b & 0x7f) as u8;
        let part = |w: u32| (w >> 8 & 0x1f) as usize;
        let by_parts = || self.parts[part(a)] >> part(b) & 1 == 1;
        if la == layer::BIPED && lb == layer::BIPED {
            return !groups_differ && by_parts();
        }
        if groups_differ {
            return self.layers_touch(la, lb);
        }
        if a & b & 0x8000 != 0 {
            return self.layers_touch(la, lb) && part(a).abs_diff(part(b)) != 1;
        }
        linked_parts && by_parts()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_character_row_matches_the_walking_layers() {
        let f = Filter::game();
        // `nif::collision::layers::CHARACTER_COLLIDES_WITH`, worked out
        // by hand from the same function earlier.
        assert_eq!(f.layers[30], 0x0799_DD73_7EFF);
        // The table is symmetric where the game clears pairs both ways.
        for a in 0..LAYERS {
            for b in 0..LAYERS {
                if f.layers[a] >> b & 1 != f.layers[b] >> a & 1 {
                    // Single rows masked one way only are allowed; just
                    // make sure the character row agrees both ways.
                    assert!(a != 30 && b != 30, "{a} {b}");
                }
            }
        }
    }

    #[test]
    fn projectiles_meet_statics_and_props_but_pass_transparent_layers() {
        let f = Filter::game();
        use layer::*;
        assert!(f.layers_touch(PROJECTILE, STATIC));
        assert!(f.layers_touch(PROJECTILE, PROPS));
        assert!(f.layers_touch(PROJECTILE, TERRAIN));
        assert!(!f.layers_touch(PROJECTILE, TRANSPARENT));
        assert!(!f.layers_touch(PROJECTILE, 7)); // SPELL
        assert!(f.layers_touch(PROJECTILE, BIPED));
        assert!(f.layers_touch(PROJECTILE, 27)); // INVISIBLE_WALL
        assert!(!f.layers_touch(CHAR_CONTROLLER, BIPED));
        assert!(f.layers_touch(CHAR_CONTROLLER, TRANSPARENT));
        assert_eq!(f.layers[PROJECTILE as usize], 0x3b1_7d92_7f77);
    }

    #[test]
    fn filter_words_follow_groups_and_flags() {
        let f = Filter::game();
        let w = |layer: u32, group: u32| layer | group << 16;
        // Different groups: the layer table.
        assert!(f.collides(w(6, 1), w(1, 2), false));
        assert!(!f.collides(w(6, 1), w(3, 2), false));
        // "No collision" flag.
        assert!(!f.collides(w(6, 1), w(1, 2) | 0x4000, false));
        // A group of 0 touches everything.
        assert!(f.collides(w(6, 1), 3, false));
        // One ragdoll's bones: by the part table.
        let bone = |part: u32| w(8, 7) | part << 8;
        assert_eq!(
            f.collides(bone(1), bone(5), false),
            f.parts[1] >> 5 & 1 == 1
        );
    }
}
