//! Particle systems in placed models, ready to run and draw: each placed
//! object whose model has particle systems (`preview::cell::ParticleModel`)
//! with its place, the sequences it plays, and how each system is drawn.
//! The simulation itself is `world::particles`; the viewer runs it and
//! builds the quads every frame.
//!
//! How the game draws them (read from `FalloutNV.exe` and the barracks and
//! Goodsprings recordings, `nv-re\findings\particles.md`): camera-facing
//! quads built on the CPU (`00e6ab40`), drawn with the no-lighting shaders,
//! `NOLIGHT017.vso` for systems with an atlas (`SubTexOffsets`, pass 'W')
//! and `NOLIGHT016.vso` without (pass 'V', `00b6f65a` picking by the data's
//! sub-texture count); neither fades by viewing angle. The pixel shader is
//! `NOLIGHTTEXVC.pso` (texture × vertex colour × `MaterialColor`, fogged as
//! the blending says), or `NOLIGHTTEXVCPMA.pso` with `One`/`One` blending
//! for systems whose alpha property adds (`SrcAlpha`/`One`, swapped by
//! `00b700d0`): the colour × alpha × (1 − fog) added. `MaterialColor` is
//! the placed object's Emittance colour × the material's glow multiplier for
//! shaders flagged for external emittance, white otherwise, its alpha the
//! material's; a blended system whose material alpha is 0 isn't drawn
//! (`00e70140`). Depth tested (less or equal), written only with the
//! shader's depth-write flag; the alpha test as the alpha property sets it.

use std::sync::Arc;

use nif::particles::{EmitterShape, ModifierKind, ParticleSystem};
use preview::cell::ParticleModel;
use preview::raster::{AlphaMode, BlendFactor};

use crate::{blend_of, AlphaTest, Blend, EmittanceLink};

/// Shader flag: the colour comes from the placed object's Emittance.
const EXTERNAL_EMITTANCE: u32 = 0x2000_0000;
/// Second shader flag set: the surface writes its depth.
const DEPTH_WRITE: u32 = 0x0000_0001;

/// A placed object's particle systems.
#[derive(Debug, Clone)]
pub struct ParticleData {
    /// The placed object (a form ID).
    pub reference: u32,
    /// Model to world, game units (the model's top node left out).
    pub placed: nif::Transform,
    pub model: Arc<ParticleModel>,
    /// Which of the model's particle sequences play (index into
    /// `model.sequences`), and whether each runs or holds its start (see
    /// `preview::cell::placed_sequences`).
    pub playing: Vec<(usize, bool)>,
    /// The sequences moving the model's nodes that it plays (index into
    /// `model.all_sequences`), and whether each runs.
    pub motion: Vec<(usize, bool)>,
    /// How each system (`model.systems`, same order) is drawn.
    pub looks: Vec<ParticleLook>,
}

/// How one system is drawn.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleLook {
    /// Index into [`crate::ViewerScene::textures`].
    pub texture: Option<usize>,
    /// `Blend::Add` for systems drawn added, else `Blend::Blend` /
    /// `MaskedBlend` (alpha-blended, drawn back to front) or opaque.
    pub blend: Blend,
    /// Added by source alpha (`SrcAlpha`/`One`): drawn with the `PMA`
    /// shader, the colour × alpha added.
    pub adds_alpha: bool,
    /// The alpha test and its threshold (0..1).
    pub alpha_test: Option<(AlphaTest, f32)>,
    pub depth_test: bool,
    pub depth_write: bool,
    /// `MaterialColor`: RGB and the material's alpha.
    pub color: [f32; 4],
    /// For colours that follow a region's weather (see
    /// [`EmittanceLink`]).
    pub emittance: Option<EmittanceLink>,
    /// Pieces of an atlas: (u, width, v, height) each.
    pub atlas: Vec<[f32; 4]>,
    /// Drawn at all: see `left_out`.
    pub drawn: bool,
    /// Why not: blended with the material's alpha 0 (the game skips it), or
    /// moved by a sequence through an interpolator not read here (left out
    /// rather than drawn in the wrong place).
    pub left_out: Option<&'static str>,
}

/// Whether a sequence playing (`playing`, indices into the model's particle
/// sequences) moves one of the system's nodes, or of the objects it names,
/// through an interpolator not read here.
fn moved_unread(model: &ParticleModel, playing: &[(usize, bool)], s: &ParticleSystem) -> bool {
    let movers: Vec<&String> = playing
        .iter()
        .filter_map(|&(i, _)| model.sequences.get(i))
        .flat_map(|q| &q.unread_movers)
        .collect();
    if movers.is_empty() {
        return false;
    }
    let mut chains: Vec<&[(String, nif::Transform)]> = vec![&s.nodes];
    for m in &s.modifiers {
        let objects: Vec<&nif::particles::ObjectRef> = match &m.kind {
            ModifierKind::Emitter(e) => match &e.shape {
                EmitterShape::Box { object, .. }
                | EmitterShape::Cylinder { object, .. }
                | EmitterShape::Sphere { object, .. }
                | EmitterShape::Array { object } => object.iter().collect(),
                EmitterShape::Mesh { meshes, .. } => meshes.iter().collect(),
            },
            ModifierKind::Gravity(g) => g.object.iter().collect(),
            ModifierKind::Drag(d) => d.object.iter().collect(),
            ModifierKind::Bomb(b) => b.object.iter().collect(),
            ModifierKind::Colliders(c) => c.iter().filter_map(|c| c.object.as_ref()).collect(),
            _ => Vec::new(),
        };
        chains.extend(objects.into_iter().map(|o| o.nodes.as_slice()));
    }
    chains.iter().any(|chain| {
        chain
            .iter()
            .any(|(name, _)| movers.iter().any(|m| m.eq_ignore_ascii_case(name)))
    })
}

/// The look of each of a model's systems as placed: `texture` turns a
/// texture path into an index, `emittance` is the placed object's Emittance
/// colour (if any) and `link` how it follows a region (if it does);
/// `playing` the particle sequences it plays ([`playing`]).
pub fn looks(
    model: &ParticleModel,
    mut texture: impl FnMut(&str) -> Option<usize>,
    emittance: Option<[f32; 3]>,
    link: Option<Option<esm::FormId>>,
    playing: &[(usize, bool)],
) -> Vec<ParticleLook> {
    model
        .systems
        .iter()
        .map(|s| {
            let alpha = match &s.alpha {
                Some(a) => AlphaMode::from_nif(a.flags, a.threshold),
                None => AlphaMode::OPAQUE,
            };
            let blend = blend_of(&alpha);
            let shader_flags = s.shader.as_ref().map_or(0, |sh| sh.shader_flags);
            let shader_flags2 = s.shader.as_ref().map_or(0, |sh| sh.shader_flags2);
            let (depth_test, depth_write) = match (&s.shader, &s.zbuffer) {
                (Some(_), _) => (true, shader_flags2 & DEPTH_WRITE != 0),
                (None, Some(z)) => (z.test(), z.write()),
                (None, None) => (true, true),
            };
            let external = shader_flags & EXTERNAL_EMITTANCE != 0;
            let mult = s.material.as_ref().map_or(1.0, |m| m.emissive_mult);
            let material_alpha = s.material.as_ref().map_or(1.0, |m| m.alpha);
            let rgb = match emittance.filter(|_| external) {
                Some(c) => c.map(|v| v * mult),
                None => [1.0; 3],
            };
            let blended = alpha.blend.is_some();
            // `00e70140`: a blended draw with the material's alpha at or
            // below 0 is skipped.
            let left_out = if blended && material_alpha <= 0.0 {
                Some("blended with the material's alpha 0")
            } else if moved_unread(model, playing, s) {
                Some("moved along a path (NiPathInterpolator) not followed here")
            } else {
                None
            };
            ParticleLook {
                texture: s.texture.as_deref().and_then(&mut texture),
                blend,
                adds_alpha: matches!(alpha.blend, Some((BlendFactor::SrcAlpha, BlendFactor::One))),
                alpha_test: alpha
                    .test
                    .map(|(test, threshold)| (test, f32::from(threshold) / 255.0)),
                depth_test,
                depth_write,
                color: [rgb[0], rgb[1], rgb[2], material_alpha],
                emittance: link.filter(|_| external).map(|region| EmittanceLink {
                    region,
                    scale: mult,
                    fallback: [1.0; 3],
                }),
                atlas: s.data.subtexture_offsets.clone(),
                drawn: left_out.is_none(),
                left_out,
            }
        })
        .collect()
}

/// The sequences a placed object plays, as indices into the model's
/// particle sequences.
pub fn playing(model: &ParticleModel, openable: bool) -> Vec<(usize, bool)> {
    preview::cell::placed_sequences(&model.all_sequences, openable)
        .iter()
        .filter_map(|p| {
            model
                .sequences
                .iter()
                .position(|s| s.name.eq_ignore_ascii_case(&p.sequence.name))
                .map(|i| (i, p.runs))
        })
        .collect()
}

/// The sequences moving the model's nodes that a placed object plays, as
/// indices into the model's sequences (`all_sequences`).
pub fn motion(model: &ParticleModel, openable: bool) -> Vec<(usize, bool)> {
    preview::cell::placed_sequences(&model.all_sequences, openable)
        .iter()
        .filter(|p| !p.sequence.tracks.is_empty())
        .filter_map(|p| {
            model
                .all_sequences
                .iter()
                .position(|s| s.name.eq_ignore_ascii_case(&p.sequence.name))
                .map(|i| (i, p.runs))
        })
        .collect()
}

/// Corners for drawing: positions, colours, texture coordinates.
pub type Corners = (Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<[f32; 2]>);

/// A quad's corners for drawing: game-unit world positions, the colour as
/// 0..1 (from the game's packed bytes), and the texture coordinates
/// already moved into the system's atlas piece (as `NOLIGHT017.vso` does:
/// `uv × (width, height) + (u, v)`).
pub fn corners_for_drawing(corners: &[world::particles::Corner], atlas: &[[f32; 4]]) -> Corners {
    let mut positions = Vec::with_capacity(corners.len());
    let mut colors = Vec::with_capacity(corners.len());
    let mut uvs = Vec::with_capacity(corners.len());
    for c in corners {
        positions.push(c.position);
        let [b, g, r, a] = c.color;
        colors.push([r, g, b, a].map(|v| f32::from(v) / 255.0));
        let uv = match atlas.get(c.sub_texture as usize) {
            Some(&[u0, w, v0, h]) if !atlas.is_empty() => [c.uv[0] * w + u0, c.uv[1] * h + v0],
            _ => c.uv,
        };
        uvs.push(uv);
    }
    (positions, colors, uvs)
}

/// Triangles for `n` quads: (0, 1, 2) and (0, 2, 3) of each (`00e6ab40`).
pub fn quad_indices(n: usize) -> Vec<u32> {
    (0..n as u32)
        .flat_map(|k| {
            let b = 4 * k;
            [b, b + 1, b + 2, b, b + 2, b + 3]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::particles::{dust_nif, Dust};

    fn model(d: &Dust) -> ParticleModel {
        let nif = nif::Nif::parse(dust_nif(d)).unwrap();
        ParticleModel {
            systems: nif
                .particle_systems(false)
                .unwrap()
                .into_iter()
                .map(Arc::new)
                .collect(),
            sequences: nif.particle_sequences().unwrap(),
            all_sequences: Arc::new(nif.sequences().unwrap()),
        }
    }

    #[test]
    fn the_dust_is_drawn_as_the_game_draws_it() {
        let m = model(&Dust::default());
        let looks = looks(&m, |_| Some(7), Some([0.5, 0.25, 1.0]), None, &[]);
        let l = &looks[0];
        assert_eq!(l.texture, Some(7));
        // SrcAlpha / InvSrcAlpha, test greater than 1.
        assert_eq!(l.blend, Blend::MaskedBlend(1.0 / 255.0));
        assert!(!l.adds_alpha);
        assert_eq!(l.alpha_test, Some((AlphaTest::Greater, 1.0 / 255.0)));
        // Tested, not written (second flag set 0).
        assert!(l.depth_test && !l.depth_write);
        // External emittance: the placed object's colour × the glow mult.
        assert_eq!(l.color, [0.5, 0.25, 1.0, 1.0]);
        assert!(l.drawn);
        let added = model(&Dust {
            additive: true,
            ..Dust::default()
        });
        let l = &looks_of(&added)[0];
        assert_eq!(l.blend, Blend::Add);
        assert!(l.adds_alpha);
    }

    fn looks_of(m: &ParticleModel) -> Vec<ParticleLook> {
        looks(m, |_| None, None, None, &[])
    }

    #[test]
    fn invisible_or_unfollowed_systems_are_left_out() {
        let clear = model(&Dust {
            material_alpha: 0.0,
            ..Dust::default()
        });
        assert!(!looks_of(&clear)[0].drawn);
        // Moved along a path by the playing Idle: left out; not playing it,
        // drawn.
        let moved = model(&Dust {
            sequence: true,
            path_mover: true,
            ..Dust::default()
        });
        let playing = playing(&moved, false);
        assert_eq!(playing, vec![(0, true)]);
        let l = &looks(&moved, |_| None, None, None, &playing)[0];
        assert!(!l.drawn && l.left_out.is_some());
        assert!(looks_of(&moved)[0].drawn);
    }

    #[test]
    fn quads_are_two_triangles_from_the_first_corner() {
        assert_eq!(quad_indices(2), vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }

    #[test]
    fn atlas_pieces_move_the_texture_coordinates() {
        let corner = world::particles::Corner {
            position: [1.0, 2.0, 3.0],
            color: [171, 166, 158, 115],
            uv: [1.0, 0.0],
            sub_texture: 5.0,
        };
        let atlas: Vec<[f32; 4]> = (0..16)
            .map(|i| {
                let (x, y) = ((i % 4) as f32, (i / 4) as f32);
                [x * 0.25, 0.25, y * 0.25, 0.25]
            })
            .collect();
        let (p, c, uv) = corners_for_drawing(&[corner], &atlas);
        assert_eq!(p[0], [1.0, 2.0, 3.0]);
        // Piece 5 is the second across, second down.
        assert_eq!(uv[0], [0.5, 0.25]);
        // The packed bytes are B, G, R, A.
        assert_eq!(c[0][0], 158.0 / 255.0);
        assert_eq!(c[0][3], 115.0 / 255.0);
        // Without an atlas the corners keep their own.
        assert_eq!(corners_for_drawing(&[corner], &[]).2[0], [1.0, 0.0]);
    }
}
