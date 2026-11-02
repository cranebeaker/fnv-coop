//! FaceGen texture morphs (`.egt`, next to a head or body model): how each
//! of the face's 50 symmetric texture controls (`FGTS` in an NPC's record)
//! changes the skin's colour, as a small picture per control. The game
//! makes an NPC's skin tint from them when it has no tint file for it (the
//! player's body always: `006149b0`, "Creating from scratch"); see
//! [`Egt::tint`].
//!
//! Layout (checked on `upperbodyhumanmale.egt`: 64 + 50 × (4 + 3 × 32 ×
//! 32) bytes, the file's size exactly): `FREGT003`, the rows, the columns,
//! the symmetric and asymmetric morph counts, a 4-byte basis value and
//! reserved bytes to 64; then each morph (symmetric first) as a scale and
//! its red, green and blue pictures, one signed byte a pixel, row by row.

use crate::error::{Error, Result};
use crate::reader::Reader;

/// A model's FaceGen texture morphs.
#[derive(Debug, Clone, PartialEq)]
pub struct Egt {
    pub width: usize,
    pub height: usize,
    /// Symmetric morphs, then asymmetric ones.
    pub symmetric: Vec<EgtMorph>,
    pub asymmetric: Vec<EgtMorph>,
}

/// One control's change: its scale and, per channel (red, green, blue),
/// a signed byte per pixel, row by row.
#[derive(Debug, Clone, PartialEq)]
pub struct EgtMorph {
    pub scale: f32,
    pub channels: [Vec<i8>; 3],
}

const MAGIC: &[u8; 8] = b"FREGT003";
const HEADER_LEN: usize = 64;

impl Egt {
    pub fn parse(bytes: &[u8]) -> Result<Egt> {
        if bytes.get(..8) != Some(&MAGIC[..]) {
            return Err(Error::Malformed {
                offset: 0,
                reason: "not a FaceGen texture file (no FREGT003 at the start)".into(),
            });
        }
        let mut r = Reader::at(bytes, 8);
        let height = r.u32("the row count")? as usize;
        let width = r.u32("the column count")? as usize;
        let symmetric = r.u32("the symmetric morph count")? as usize;
        let asymmetric = r.u32("the asymmetric morph count")? as usize;
        let pixels = width * height;
        let morph_len = 4 + 3 * pixels;
        let needed = HEADER_LEN + (symmetric + asymmetric) * morph_len;
        if bytes.len() < needed {
            return Err(Error::Malformed {
                offset: bytes.len(),
                reason: format!(
                    "a {width} x {height} texture morph file with {} morphs needs {needed} bytes",
                    symmetric + asymmetric
                ),
            });
        }
        let morph = |i: usize| -> EgtMorph {
            let at = HEADER_LEN + i * morph_len;
            let scale =
                f32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
            let channel = |c: usize| {
                let from = at + 4 + c * pixels;
                bytes[from..from + pixels]
                    .iter()
                    .map(|&b| b as i8)
                    .collect()
            };
            EgtMorph {
                scale,
                channels: [channel(0), channel(1), channel(2)],
            }
        };
        Ok(Egt {
            width,
            height,
            symmetric: (0..symmetric).map(morph).collect(),
            asymmetric: (symmetric..symmetric + asymmetric).map(morph).collect(),
        })
    }

    /// The skin tint for these control values (`FGTS`), as the game makes
    /// it (`0065b410`): each morph's pixels times its scale and the
    /// control's value, both in 1/256ths (`× 256.0`, `010231d8`, cut to
    /// whole numbers by `_ftol`), added up as whole numbers and divided by
    /// 65,536; then made a picture (`0064ceb0` with −255, 255 and 0.5):
    /// each sum rounded down, kept within ±255, and `(v + 255) × 0.5`
    /// stored with the fraction cut off, so no change is 127. RGB, row by
    /// row, `width × height × 3` bytes.
    pub fn tint(&self, values: &[f32]) -> Vec<u8> {
        let pixels = self.width * self.height;
        let mut sums = vec![0i32; pixels * 3];
        for (morph, &value) in self.symmetric.iter().zip(values) {
            let weight = (value * 256.0) as i32;
            if weight == 0 {
                continue;
            }
            let k = (morph.scale * 256.0) as i32 * weight;
            for (c, channel) in morph.channels.iter().enumerate() {
                for (p, &d) in channel.iter().enumerate() {
                    let s = &mut sums[p * 3 + c];
                    *s = s.wrapping_add(i32::from(d).wrapping_mul(k));
                }
            }
        }
        sums.iter()
            .map(|&s| {
                let v = (s as f32 * (1.0 / 65536.0)).floor().clamp(-255.0, 255.0);
                ((v + 255.0) * 0.5) as u8
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(width: u32, height: u32, morphs: &[(f32, [i8; 3])]) -> Vec<u8> {
        let mut b = MAGIC.to_vec();
        for v in [height, width, morphs.len() as u32, 0, 81] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.resize(HEADER_LEN, 0);
        for (scale, rgb) in morphs {
            b.extend_from_slice(&scale.to_le_bytes());
            for c in rgb {
                b.extend(std::iter::repeat_n(*c as u8, (width * height) as usize));
            }
        }
        b
    }

    #[test]
    fn reads_the_layout() {
        let egt = Egt::parse(&file(2, 3, &[(0.5, [1, -2, 3]), (2.0, [0, 0, 0])])).unwrap();
        assert_eq!((egt.width, egt.height), (2, 3));
        assert_eq!(egt.symmetric.len(), 2);
        assert_eq!(egt.symmetric[0].scale, 0.5);
        assert_eq!(egt.symmetric[0].channels[1], vec![-2; 6]);
        assert!(Egt::parse(b"FREGM002....").is_err());
        assert!(Egt::parse(&file(2, 3, &[(1.0, [0; 3])])[..80]).is_err());
    }

    #[test]
    fn no_change_is_127_and_values_add_in_256ths() {
        let egt = Egt::parse(&file(1, 1, &[(1.0, [100, -100, 0]), (0.5, [10, 0, 0])])).unwrap();
        assert_eq!(egt.tint(&[0.0, 0.0]), vec![127, 127, 127]);
        // 1.0 × 1.0 × 100 + 0.5 × 2.0 × 10 = 110 → (110 + 255) / 2 = 182.5.
        assert_eq!(egt.tint(&[1.0, 2.0]), vec![182, 77, 127]);
        // Values are cut to 1/256ths: 0.001 is nothing.
        assert_eq!(egt.tint(&[0.001, 0.0]), vec![127, 127, 127]);
        // Kept within ±255.
        assert_eq!(egt.tint(&[9.0, 0.0]), vec![255, 0, 127]);
    }
}
