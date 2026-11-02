//! FaceGen morph targets (`.tri`, next to a head part's model): the named
//! shapes a face takes while it talks, blinks and looks around. Every
//! FaceGen head part (head, mouth, teeth, tongue, eyes, eyebrows, beards)
//! has one; the game finds it by replacing the model's extension with
//! `.tri`.
//!
//! Layout (`FRTRI003`; parses all 49 `.tri` files in the game's archives to
//! their last byte): the base vertex count V, triangle count T, quad count
//! Q, two counts of labelled points (0 in every file), the UV count, flags
//! (bit 0: UVs and texture faces follow), the number of differential (D)
//! and statistical (S) morphs and the statistical morphs' total target
//! vertices M, then 16 reserved bytes; then V base vertices, the M targets,
//! T triangles, Q quads, the UVs and texture faces, the D differential
//! morphs and the S statistical ones.
//!
//! - A **differential** morph moves every vertex: a name, a scale, then per
//!   vertex three signed 16-bit offsets times the scale (game units).
//! - A **statistical** morph moves a few vertices to fixed places: a name
//!   and the vertices' indices; where they go are the next entries of the
//!   target block, in file order.
//!
//! The base vertices are the model's own, one for one (the head's model
//! was edited after its `.tri` was made, so its vertices differ slightly;
//! the offsets still apply to them).

use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::reader::{latin1, Reader};

/// A head part's morph targets.
#[derive(Debug, Clone, PartialEq)]
pub struct Tri {
    /// The base mesh's vertices (the model's, in the same order).
    pub base: Vec<Vec3>,
    pub differential: Vec<DifferentialMorph>,
    pub statistical: Vec<StatisticalMorph>,
}

/// A morph that moves every vertex by its own offset.
#[derive(Debug, Clone, PartialEq)]
pub struct DifferentialMorph {
    pub name: String,
    /// Per base vertex: how far it moves at full strength (game units).
    pub offsets: Vec<Vec3>,
}

/// A morph that moves some vertices to fixed positions.
#[derive(Debug, Clone, PartialEq)]
pub struct StatisticalMorph {
    pub name: String,
    /// The vertices it moves (indices into the base vertices).
    pub vertices: Vec<u32>,
    /// Where each of them ends up at full strength (model space).
    pub targets: Vec<Vec3>,
    /// Where its first target sits in the file's block of all targets
    /// (FaceGen's `.egm` numbers them after the base vertices).
    pub first_target: usize,
}

const MAGIC: &[u8; 8] = b"FRTRI003";
const HEADER_LEN: usize = 64;

impl Tri {
    pub fn parse(bytes: &[u8]) -> Result<Tri> {
        if bytes.get(..8) != Some(&MAGIC[..]) {
            return Err(Error::Malformed {
                offset: 0,
                reason: "not a FaceGen morph file (no FRTRI003 at the start)".into(),
            });
        }
        let mut r = Reader::at(bytes, 8);
        let mut count = |what: &str| -> Result<usize> {
            let n = r.i32(what)?;
            usize::try_from(n).map_err(|_| Error::Malformed {
                offset: 0,
                reason: format!("negative {what}: {n}"),
            })
        };
        let vertices = count("the vertex count")?;
        let triangles = count("the triangle count")?;
        let quads = count("the quad count")?;
        let _labelled_vertices = count("the labelled vertex count")?;
        let _labelled_points = count("the labelled surface point count")?;
        let uvs = count("the UV count")?;
        let flags = count("the flags")?;
        let differential = count("the differential morph count")?;
        let statistical = count("the statistical morph count")?;
        let targets = count("the statistical target count")?;

        let mut r = Reader::at(bytes, HEADER_LEN);
        let base = r.counted(vertices, 12, "base vertices", |r| r.vec3("a vertex"))?;
        let all_targets = r.counted(targets, 12, "morph targets", |r| r.vec3("a target"))?;
        r.take(triangles.saturating_mul(12), "triangles")?;
        r.take(quads.saturating_mul(16), "quads")?;
        if flags & 1 != 0 {
            r.take(uvs.saturating_mul(8), "texture coordinates")?;
            r.take(triangles.saturating_mul(12), "texture triangles")?;
            r.take(quads.saturating_mul(16), "texture quads")?;
        }
        let name = |r: &mut Reader| -> Result<String> {
            let len = r.i32("a morph name's length")?;
            let len = usize::try_from(len).map_err(|_| Error::Malformed {
                offset: r.offset(),
                reason: format!("negative morph name length {len}"),
            })?;
            Ok(latin1(r.take(len, "a morph name")?))
        };
        let differential = (0..differential)
            .map(|_| {
                let name = name(&mut r)?;
                let scale = r.f32("a morph scale")?;
                let offsets = r.counted(vertices, 6, "morph offsets", |r| {
                    let mut v = [0.0; 3];
                    for c in &mut v {
                        *c = f32::from(r.u16("an offset")? as i16) * scale;
                    }
                    Ok(v)
                })?;
                Ok(DifferentialMorph { name, offsets })
            })
            .collect::<Result<Vec<_>>>()?;
        let mut next = 0;
        let statistical = (0..statistical)
            .map(|_| {
                let name = name(&mut r)?;
                let n = r.i32("a morph's vertex count")?;
                let n = usize::try_from(n).unwrap_or(usize::MAX);
                let indices = r.counted(n, 4, "morph vertices", |r| r.u32("a vertex index"))?;
                let targets = all_targets
                    .get(next..next + n)
                    .ok_or_else(|| Error::Malformed {
                        offset: r.offset(),
                        reason: format!("{name} needs more targets than the file has"),
                    })?
                    .to_vec();
                let morph = StatisticalMorph {
                    name,
                    vertices: indices,
                    targets,
                    first_target: next,
                };
                next += n;
                Ok(morph)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Tri {
            base,
            differential,
            statistical,
        })
    }

    /// All the statistical morphs' targets together.
    pub fn target_count(&self) -> usize {
        self.statistical.iter().map(|m| m.targets.len()).sum()
    }
}

/// Where a model's morph targets are: its path with `.tri` for its
/// extension (`characters\head\headhuman.nif` → `...\headhuman.tri`).
pub fn tri_path(model: &str) -> Option<String> {
    let (stem, _) = model.rsplit_once('.')?;
    Some(format!("{stem}.tri"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `.tri` file: `base` vertices, one triangle, the differential and
    /// statistical morphs given (name, scale, offsets) and (name, indices,
    /// targets).
    pub(crate) fn tri(
        base: &[Vec3],
        differential: &[(&str, f32, Vec<[i16; 3]>)],
        statistical: &[(&str, Vec<u32>, Vec<Vec3>)],
    ) -> Vec<u8> {
        let mut b = MAGIC.to_vec();
        let targets: usize = statistical.iter().map(|s| s.2.len()).sum();
        for v in [
            base.len(),
            1,
            0,
            0,
            0,
            base.len(),
            1,
            differential.len(),
            statistical.len(),
            targets,
        ] {
            b.extend((v as i32).to_le_bytes());
        }
        b.extend([0u8; 16]);
        let floats = |b: &mut Vec<u8>, v: &[f32]| v.iter().for_each(|f| b.extend(f.to_le_bytes()));
        for v in base {
            floats(&mut b, v);
        }
        for (_, _, t) in statistical {
            for v in t {
                floats(&mut b, v);
            }
        }
        // One triangle, the UVs (one per vertex) and one texture triangle.
        for i in [0i32, 1, 2] {
            b.extend(i.to_le_bytes());
        }
        for _ in base {
            floats(&mut b, &[0.5, 0.5]);
        }
        for i in [0i32, 1, 2] {
            b.extend(i.to_le_bytes());
        }
        let name = |b: &mut Vec<u8>, n: &str| {
            b.extend((n.len() as i32 + 1).to_le_bytes());
            b.extend(n.as_bytes());
            b.push(0);
        };
        for (n, scale, offsets) in differential {
            name(&mut b, n);
            b.extend(scale.to_le_bytes());
            for o in offsets {
                for c in o {
                    b.extend(c.to_le_bytes());
                }
            }
        }
        for (n, indices, _) in statistical {
            name(&mut b, n);
            b.extend((indices.len() as i32).to_le_bytes());
            for i in indices {
                b.extend(i.to_le_bytes());
            }
        }
        b
    }

    #[test]
    fn reads_both_kinds_of_morph() {
        let base = [[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let bytes = tri(
            &base,
            &[("Aah", 0.5, vec![[2, 0, 0], [0, -4, 0], [0, 0, 32767]])],
            &[
                ("BlinkLeft", vec![2], vec![[0.0, 1.0, -0.5]]),
                ("LookUp", vec![0, 1], vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0]]),
            ],
        );
        let t = Tri::parse(&bytes).unwrap();
        assert_eq!(t.base, base);
        assert_eq!(t.differential[0].name, "Aah");
        assert_eq!(
            t.differential[0].offsets,
            [[1.0, 0.0, 0.0], [0.0, -2.0, 0.0], [0.0, 0.0, 16383.5]]
        );
        let blink = &t.statistical[0];
        assert_eq!(
            (blink.name.as_str(), &blink.vertices[..]),
            ("BlinkLeft", &[2][..])
        );
        assert_eq!(blink.targets, [[0.0, 1.0, -0.5]]);
        // The second morph's targets follow the first's in the file.
        let up = &t.statistical[1];
        assert_eq!(up.first_target, 1);
        assert_eq!(up.targets[1], [1.0, 0.0, 1.0]);
        assert_eq!(t.target_count(), 3);
    }

    #[test]
    fn rejects_other_files_and_short_ones() {
        assert!(Tri::parse(b"FREGM002....").is_err());
        let bytes = tri(&[[0.0; 3]; 3], &[("Aah", 1.0, vec![[0; 3]; 3])], &[]);
        assert!(Tri::parse(&bytes[..bytes.len() - 2]).is_err());
    }

    #[test]
    fn the_morphs_sit_beside_the_model() {
        assert_eq!(
            tri_path("Characters\\Head\\HeadHuman.NIF").as_deref(),
            Some("Characters\\Head\\HeadHuman.tri")
        );
    }
}
