//! FaceGen's control file (`FACEGEN\SI.CTL`): the named face sliders and
//! the age/gender statistics that the face menu and Randomize work with.
//! Each is a linear function of a face's FaceGen coordinates (the `FGGS`,
//! `FGGA`, `FGTS` values of an NPC); `world::chargen::facegen` evaluates
//! them.
//!
//! Layout, read from the game's loader (`FalloutNV.exe` 1.4.0.525:
//! `00aaa7d0` header, `00aaac10` controls, `00aaaf10` statistics; all
//! little-endian 32-bit values):
//!
//! * `FRCTL001`, two 32-bit values the game keeps but this reader does not
//!   interpret (**inferred**: identifiers of the shape and texture bases,
//!   like the value in an `.egm` header), then the basis sizes: shape
//!   symmetric, shape asymmetric, texture symmetric, texture asymmetric.
//! * Controls: for shape then texture, symmetric then asymmetric, a count
//!   and per control as many coefficients as that basis has, a label length
//!   and the label.
//! * Statistics, for five groups (the game only uses the first): for age
//!   then gender, for shape then texture, a symmetric-basis coefficient
//!   vector and an offset.
//! * Group differences: for each ordered pair of different groups, a shape
//!   and a texture vector, then one offset.
//! * Distributions, per group: the mean shape and texture, their joint
//!   covariance (square, both sizes added) and each one's own covariance.
//!
//! The game reads no further. The group differences and distributions are
//! loaded but nothing in the game uses them (no caller outside the loader's
//! source file reaches them); they are kept for completeness.

use crate::error::{Error, Result};
use crate::reader::{latin1, Reader};

const MAGIC: &[u8; 8] = b"FRCTL001";

/// FaceGen statistic groups in the file (`00aaaf10` loops over five).
pub const GROUPS: usize = 5;

/// Which part of the face a control or statistic moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Shape = 0,
    Texture = 1,
}

/// Symmetric coordinates move both sides of the face alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symmetry {
    Symmetric = 0,
    Asymmetric = 1,
}

/// The two statistics (`00aabdb0`'s second index).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    Age = 0,
    Gender = 1,
}

/// A named slider: its value for a face is the coefficients' dot product
/// with the face's coordinates of its kind and symmetry.
#[derive(Debug, Clone, PartialEq)]
pub struct Control {
    pub label: String,
    pub coefficients: Vec<f32>,
}

/// A statistic as a linear function: coefficients · coordinates + offset.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub coefficients: Vec<f32>,
    pub offset: f32,
}

/// The difference between two groups (unused by the game).
#[derive(Debug, Clone, PartialEq)]
pub struct GroupDifference {
    pub from: usize,
    pub to: usize,
    /// Shape, texture.
    pub coefficients: [Vec<f32>; 2],
    pub offset: f32,
}

/// A group's face distribution (unused by the game). Matrices are stored
/// row by row.
#[derive(Debug, Clone, PartialEq)]
pub struct Distribution {
    /// Shape, texture.
    pub mean: [Vec<f32>; 2],
    pub joint_covariance: Vec<f32>,
    /// Shape, texture.
    pub covariance: [Vec<f32>; 2],
}

/// A parsed control file.
#[derive(Debug, Clone, PartialEq)]
pub struct Ctl {
    /// The two header values after the magic (see the module notes).
    pub basis_ids: [u32; 2],
    /// `[kind][symmetry]` basis sizes.
    pub sizes: [[usize; 2]; 2],
    /// `[kind][symmetry]` controls, in file order (the order the face menu
    /// numbers them).
    pub controls: [[Vec<Control>; 2]; 2],
    /// `[group][stat][kind]`, over the symmetric coordinates.
    pub stats: Vec<[[Linear; 2]; 2]>,
    pub differences: Vec<GroupDifference>,
    pub distributions: Vec<Distribution>,
}

impl Ctl {
    pub fn parse(bytes: &[u8]) -> Result<Ctl> {
        if bytes.get(..8) != Some(&MAGIC[..]) {
            return Err(Error::Malformed {
                offset: 0,
                reason: "not a FaceGen control file (no FRCTL001 at the start)".into(),
            });
        }
        let mut r = Reader::at(bytes, 8);
        let basis_ids = [r.u32("a basis id")?, r.u32("a basis id")?];
        let mut sizes = [[0; 2]; 2];
        for kind in &mut sizes {
            for size in kind.iter_mut() {
                *size = r.u32("a basis size")? as usize;
            }
        }
        let floats = |r: &mut Reader, n: usize, what: &str| r.counted(n, 4, what, |r| r.f32(what));

        let mut controls: [[Vec<Control>; 2]; 2] = Default::default();
        for (kind, lists) in controls.iter_mut().enumerate() {
            for (symmetry, list) in lists.iter_mut().enumerate() {
                let n = sizes[kind][symmetry];
                let count = r.u32("a control count")? as usize;
                // Every control holds at least its coefficients and a length.
                *list = r.counted(count, n * 4 + 4, "controls", |r| {
                    let coefficients = floats(r, n, "control coefficients")?;
                    let len = r.u32("a control label length")? as usize;
                    let label = latin1(r.take(len, "a control label")?);
                    Ok(Control {
                        label,
                        coefficients,
                    })
                })?;
            }
        }

        let symmetric = [sizes[0][0], sizes[1][0]];
        let mut stats = Vec::with_capacity(GROUPS);
        for _ in 0..GROUPS {
            let linear = |r: &mut Reader, kind: usize| -> Result<Linear> {
                Ok(Linear {
                    coefficients: floats(r, symmetric[kind], "statistic coefficients")?,
                    offset: r.f32("a statistic offset")?,
                })
            };
            let age = [linear(&mut r, 0)?, linear(&mut r, 1)?];
            let gender = [linear(&mut r, 0)?, linear(&mut r, 1)?];
            stats.push([age, gender]);
        }

        let mut differences = Vec::new();
        for from in 0..GROUPS {
            for to in (0..GROUPS).filter(|&to| to != from) {
                let shape = floats(&mut r, symmetric[0], "group difference coefficients")?;
                let texture = floats(&mut r, symmetric[1], "group difference coefficients")?;
                differences.push(GroupDifference {
                    from,
                    to,
                    coefficients: [shape, texture],
                    offset: r.f32("a group difference offset")?,
                });
            }
        }

        let joint = symmetric[0] + symmetric[1];
        let mut distributions = Vec::with_capacity(GROUPS);
        for _ in 0..GROUPS {
            let mean = [
                floats(&mut r, symmetric[0], "a mean")?,
                floats(&mut r, symmetric[1], "a mean")?,
            ];
            let joint_covariance = floats(&mut r, joint * joint, "a covariance")?;
            let covariance = [
                floats(&mut r, symmetric[0] * symmetric[0], "a covariance")?,
                floats(&mut r, symmetric[1] * symmetric[1], "a covariance")?,
            ];
            distributions.push(Distribution {
                mean,
                joint_covariance,
                covariance,
            });
        }

        Ok(Ctl {
            basis_ids,
            sizes,
            controls,
            stats,
            differences,
            distributions,
        })
    }

    /// The controls of one kind and symmetry.
    pub fn controls(&self, kind: Kind, symmetry: Symmetry) -> &[Control] {
        &self.controls[kind as usize][symmetry as usize]
    }

    /// A statistic of a group.
    pub fn stat(&self, group: usize, stat: Stat, kind: Kind) -> Option<&Linear> {
        self.stats
            .get(group)
            .map(|s| &s[stat as usize][kind as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `[kind][symmetry]` lists of (label, coefficients).
    type Lists<'a> = [[Vec<(&'a str, Vec<f32>)>; 2]; 2];

    /// Writes a control file in the game's layout; `stat(group,
    /// stat, kind)` gives each statistic. Group differences and
    /// distributions are filled with a running counter.
    fn ctl(
        sizes: [[usize; 2]; 2],
        controls: &Lists,
        stat: impl Fn(usize, usize, usize) -> (Vec<f32>, f32),
    ) -> Vec<u8> {
        let mut b = MAGIC.to_vec();
        b.extend(7u32.to_le_bytes());
        b.extend(9u32.to_le_bytes());
        for kind in sizes {
            for n in kind {
                b.extend((n as u32).to_le_bytes());
            }
        }
        let f = |b: &mut Vec<u8>, v: f32| b.extend(v.to_le_bytes());
        for lists in controls {
            for list in lists {
                b.extend((list.len() as u32).to_le_bytes());
                for (label, c) in list {
                    c.iter().for_each(|&v| f(&mut b, v));
                    b.extend((label.len() as u32).to_le_bytes());
                    b.extend(label.as_bytes());
                }
            }
        }
        for g in 0..GROUPS {
            for s in 0..2 {
                for k in 0..2 {
                    let (c, o) = stat(g, s, k);
                    c.iter().for_each(|&v| f(&mut b, v));
                    f(&mut b, o);
                }
            }
        }
        let (gs, ts) = (sizes[0][0], sizes[1][0]);
        let mut counter = 0.0;
        let mut run = |b: &mut Vec<u8>, n: usize| {
            for _ in 0..n {
                counter += 1.0;
                f(b, counter);
            }
        };
        for _ in 0..GROUPS * (GROUPS - 1) {
            run(&mut b, gs + ts + 1);
        }
        for _ in 0..GROUPS {
            run(&mut b, gs + ts + (gs + ts) * (gs + ts) + gs * gs + ts * ts);
        }
        b
    }

    #[test]
    fn a_control_file_reads_in_the_games_order() {
        let controls = [
            [
                vec![("Brow ridge", vec![1.0, 2.0]), ("Cheeks", vec![0.0, -1.0])],
                vec![("Tilt", vec![0.5])],
            ],
            [vec![("Tone", vec![3.0, 4.0, 5.0])], vec![]],
        ];
        let bytes = ctl([[2, 1], [3, 0]], &controls, |g, s, k| {
            let n = [2, 3][k];
            (vec![(g * 10 + s) as f32; n], k as f32)
        });
        let c = Ctl::parse(&bytes).unwrap();
        assert_eq!(c.basis_ids, [7, 9]);
        assert_eq!(c.sizes, [[2, 1], [3, 0]]);
        let shape = c.controls(Kind::Shape, Symmetry::Symmetric);
        assert_eq!(shape.len(), 2);
        assert_eq!(shape[1].label, "Cheeks");
        assert_eq!(shape[1].coefficients, [0.0, -1.0]);
        assert_eq!(
            c.controls(Kind::Shape, Symmetry::Asymmetric)[0].label,
            "Tilt"
        );
        assert_eq!(
            c.controls(Kind::Texture, Symmetry::Symmetric)[0].coefficients,
            [3.0, 4.0, 5.0]
        );
        assert!(c.controls(Kind::Texture, Symmetry::Asymmetric).is_empty());

        let gender = c.stat(3, Stat::Gender, Kind::Texture).unwrap();
        assert_eq!(gender.coefficients, [31.0; 3]);
        assert_eq!(gender.offset, 1.0);
        assert_eq!(
            c.stat(0, Stat::Age, Kind::Shape).unwrap().coefficients,
            [0.0; 2]
        );

        assert_eq!(c.differences.len(), 20);
        assert_eq!((c.differences[4].from, c.differences[4].to), (1, 0));
        assert_eq!(c.differences[0].coefficients[0], [1.0, 2.0]);
        assert_eq!(c.differences[0].offset, 6.0);
        let d = &c.distributions[0];
        assert_eq!(d.mean[0], [121.0, 122.0]);
        assert_eq!(d.joint_covariance.len(), 25);
        assert_eq!(d.covariance[1].len(), 9);
        assert_eq!(
            c.distributions[4].covariance[1].last(),
            Some(&(20.0 * 6.0 + 5.0 * 43.0))
        );
    }

    #[test]
    fn a_short_or_foreign_file_is_an_error() {
        assert!(Ctl::parse(b"FREGM002").is_err());
        let bytes = ctl([[1, 0], [1, 0]], &Default::default(), |_, _, _| {
            (vec![0.0], 0.0)
        });
        assert!(Ctl::parse(&bytes[..bytes.len() - 1]).is_err());
        assert!(Ctl::parse(&bytes).is_ok());
    }
}
