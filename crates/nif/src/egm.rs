//! FaceGen shape morphs (`.egm`, next to a head model): how each of the
//! face's 50 symmetric and 30 asymmetric controls moves every vertex. An
//! NPC's face (`FGGS` and `FGGA` in its record) is the model's vertices
//! plus each control's value times its morph.
//!
//! Layout (checked on `headhuman.egm`: the sizes add up to the file's
//! exactly): `FREGM002`, the vertex count, the symmetric and asymmetric
//! morph counts, a 4-byte value (a basis hash) and 40 reserved bytes; then
//! each morph (symmetric first) as a scale and, per vertex, three signed
//! 16-bit offsets that the scale multiplies.

use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::reader::Reader;

/// A model's FaceGen shape morphs.
#[derive(Debug, Clone, PartialEq)]
pub struct Egm {
    pub vertices: usize,
    /// Per morph, per vertex: the offset at a control value of 1.
    pub symmetric: Vec<Vec<Vec3>>,
    pub asymmetric: Vec<Vec<Vec3>>,
}

const MAGIC: &[u8; 8] = b"FREGM002";
const HEADER_LEN: usize = 64;

impl Egm {
    pub fn parse(bytes: &[u8]) -> Result<Egm> {
        if bytes.get(..8) != Some(&MAGIC[..]) {
            return Err(Error::Malformed {
                offset: 0,
                reason: "not a FaceGen shape file (no FREGM002 at the start)".into(),
            });
        }
        let mut r = Reader::at(bytes, 8);
        let vertices = r.u32("the vertex count")? as usize;
        let symmetric = r.u32("the symmetric morph count")? as usize;
        let asymmetric = r.u32("the asymmetric morph count")? as usize;
        let mut r = Reader::at(bytes, HEADER_LEN);
        let morph = |r: &mut Reader| -> Result<Vec<Vec3>> {
            let scale = r.f32("a morph scale")?;
            r.counted(vertices, 6, "morph offsets", |r| {
                let mut v = [0.0; 3];
                for c in &mut v {
                    *c = f32::from(r.u16("an offset")? as i16) * scale;
                }
                Ok(v)
            })
        };
        let read = |count: usize, r: &mut Reader| -> Result<Vec<Vec<Vec3>>> {
            (0..count).map(|_| morph(r)).collect()
        };
        let symmetric = read(symmetric, &mut r)?;
        let asymmetric = read(asymmetric, &mut r)?;
        Ok(Egm {
            vertices,
            symmetric,
            asymmetric,
        })
    }

    /// Moves `positions` (the model's own vertices) by the face's control
    /// values. Extra values are ignored.
    ///
    /// The morphs can have more vertices than the model: the model's own
    /// come first, in order, then one per statistical morph target of the
    /// `.tri` beside it (`headhuman.egm` has 1,449 = the head's 1,211 + its
    /// `.tri`'s 238 targets; the counts match in every head part, so this is
    /// **inferred**, see [`Egm::apply_rows`]). A model with more vertices
    /// than the morphs is left alone.
    pub fn apply(&self, positions: &mut [Vec3], symmetric: &[f32], asymmetric: &[f32]) {
        self.apply_rows(0, positions, symmetric, asymmetric);
    }

    /// As [`Egm::apply`], with the morphs' vertices taken from `first` on:
    /// the rows after a model's own vertices move its `.tri`'s statistical
    /// targets (see `world::face::FaceMorphs`). Left alone when the morphs
    /// don't reach that far.
    pub fn apply_rows(
        &self,
        first: usize,
        positions: &mut [Vec3],
        symmetric: &[f32],
        asymmetric: &[f32],
    ) {
        if first.saturating_add(positions.len()) > self.vertices {
            return;
        }
        for (morph, &value) in self
            .symmetric
            .iter()
            .zip(symmetric)
            .chain(self.asymmetric.iter().zip(asymmetric))
        {
            if value == 0.0 {
                continue;
            }
            for (p, d) in positions.iter_mut().zip(&morph[first..]) {
                for k in 0..3 {
                    p[k] += value * d[k];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn egm(vertices: u32, morphs: &[(f32, Vec<[i16; 3]>)], symmetric: u32) -> Vec<u8> {
        let mut b = MAGIC.to_vec();
        b.extend(vertices.to_le_bytes());
        b.extend(symmetric.to_le_bytes());
        b.extend((morphs.len() as u32 - symmetric).to_le_bytes());
        b.extend([0u8; 44]);
        for (scale, offsets) in morphs {
            b.extend(scale.to_le_bytes());
            for o in offsets {
                for c in o {
                    b.extend(c.to_le_bytes());
                }
            }
        }
        b
    }

    #[test]
    fn faces_move_by_each_control_times_its_morph() {
        let bytes = egm(
            2,
            &[
                (0.5, vec![[2, 0, 0], [0, 0, 0]]),
                (1.0, vec![[0, 0, -1], [0, 4, 0]]),
            ],
            1,
        );
        let e = Egm::parse(&bytes).unwrap();
        assert_eq!(e.symmetric[0][0], [1.0, 0.0, 0.0]);
        let mut p = [[0.0; 3], [10.0, 10.0, 10.0]];
        e.apply(&mut p, &[2.0], &[0.5]);
        assert_eq!(p[0], [2.0, 0.0, -0.5]);
        assert_eq!(p[1], [10.0, 12.0, 10.0]);
    }

    #[test]
    fn later_rows_move_points_past_the_models_own() {
        // Two model vertices, then one more row (a `.tri` target).
        let bytes = egm(3, &[(1.0, vec![[1, 0, 0], [2, 0, 0], [0, 0, 5]])], 1);
        let e = Egm::parse(&bytes).unwrap();
        let mut target = [[1.0, 1.0, 1.0]];
        e.apply_rows(2, &mut target, &[0.5], &[]);
        assert_eq!(target[0], [1.0, 1.0, 3.5]);
        // Past the morphs' rows: left alone.
        e.apply_rows(3, &mut target, &[0.5], &[]);
        assert_eq!(target[0], [1.0, 1.0, 3.5]);
    }

    #[test]
    fn rejects_other_files() {
        assert!(Egm::parse(b"FREGT003....").is_err());
    }
}
