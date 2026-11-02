//! Impact effect models (`world::impacts`): how long the game plays one.
//!
//! **How long** ([`animation_time`], the game's `00689840`, used because
//! the game makes these effects with flag 4, `00689310`): the longest
//! (stop − start) of the time controllers on the model's nodes and shapes,
//! looked for through every node's children; none gives `None` (the game's
//! −1), and the effect then lasts what it was given
//! (`world::impacts::effect_lifetime`).
//!
//! **Drawn** ([`EffectModel`], [`effect_piece_at`]): the model on its own,
//! its pieces moved by the controllers it carries itself
//! (`nif::Nif::own_controllers`) from when the effect appears, under a
//! billboard node turned toward the camera, faded by its material's alpha
//! controller. Particle systems in the model aren't drawn with it.

fn le_i32(b: &[u8], at: usize) -> Option<i32> {
    Some(i32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn le_f32(b: &[u8], at: usize) -> Option<f32> {
    Some(f32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// The controllers an object has: an `NiObjectNET`'s controller (after its
/// name and extra data list), then each controller's next (a
/// `NiTimeController` starts with it: next (4), flags (2), frequency,
/// phase, start time, stop time).
fn controller_times(nif: &nif::Nif, object: usize, longest: &mut Option<f32>) {
    let bytes = nif.block_bytes(object);
    let Some(count) = le_i32(bytes, 4) else {
        return;
    };
    let Ok(count) = usize::try_from(count) else {
        return;
    };
    let mut next = le_i32(bytes, 8 + 4 * count).unwrap_or(-1);
    let mut steps = 0;
    while let Ok(c) = usize::try_from(next) {
        if c >= nif.blocks().len() || steps > 64 {
            break;
        }
        steps += 1;
        let b = nif.block_bytes(c);
        let (Some(start), Some(stop)) = (le_f32(b, 14), le_f32(b, 18)) else {
            break;
        };
        let t = stop - start;
        if t.is_finite() {
            *longest = Some(longest.map_or(t, |l| l.max(t)));
        }
        next = le_i32(b, 0).unwrap_or(-1);
    }
}

/// How long a model animates (see the module notes).
pub fn animation_time(nif: &nif::Nif) -> Option<f32> {
    fn visit(nif: &nif::Nif, index: usize, depth: usize, longest: &mut Option<f32>) {
        if depth > 64 || index >= nif.blocks().len() {
            return;
        }
        controller_times(nif, index, longest);
        if let Ok(nif::Block::Node(node)) = nif.block(index) {
            for child in node.children {
                if let Ok(c) = usize::try_from(child) {
                    visit(nif, c, depth + 1, longest);
                }
            }
        }
    }
    let mut longest = None;
    for &root in nif.roots() {
        if let Ok(r) = usize::try_from(root) {
            visit(nif, r, 0, &mut longest);
        }
    }
    longest
}

/// An impact's effect model, ready to be put at a hit (`006890b0`): what
/// to draw (a scene of the model on its own at the origin, as
/// [`crate::Game::made_scene`] makes it), per drawn piece the nodes above
/// it and its billboard, the model's own controllers
/// ([`nif::Nif::own_controllers`]: the game's impact models move their
/// billboard node and fade their material this way) and how long it
/// animates ([`animation_time`]).
pub struct EffectModel {
    pub scene: crate::ViewerScene,
    /// One per non-actor draw of [`Self::scene`], in order.
    pub pieces: Vec<EffectPiece>,
    pub controllers: Option<nif::Sequence>,
    pub animation: Option<f32>,
    /// The model has particle systems (not drawn with it).
    pub particles: bool,
}

/// A piece of an effect model: the nodes from the model's top down to it
/// (the top's own transform left out, as for placed models) and, under an
/// `NiBillboardNode`, which node that is and how it turns.
#[derive(Debug, Clone)]
pub struct EffectPiece {
    pub nodes: Vec<(String, nif::Transform)>,
    pub billboard: Option<(usize, preview::cell::Billboard)>,
    /// Under an `NiBillboardNode` in mode 1 (`ROTATE_ABOUT_UP`, which the
    /// game's ballistic impact models use): which node.
    pub turns_about_up: Option<usize>,
}

impl crate::Game {
    /// An impact's effect model (`IPCT` `MODL`, relative to `meshes\`),
    /// `None` when it can't be read or draws nothing.
    pub fn effect_model(&self, model: &str) -> Option<EffectModel> {
        let path = assets::mesh_path(model);
        let nif = nif::Nif::parse(self.assets.read(&path).ok()??).ok()?;
        let placed = nif.placed_scene().ok()?;
        let controllers = nif.own_controllers().ok().flatten();
        let animation = animation_time(&nif);
        let particles = nif
            .particle_systems(false)
            .is_ok_and(|systems| !systems.is_empty());
        let scene = self.made_scene(vec![world::Placement {
            form_id: esm::FormId(0),
            record_type: esm::FourCC::new(b"REFR"),
            editor_id: None,
            base: esm::FormId(0),
            base_type: esm::FourCC::new(b"IPCT"),
            base_editor_id: None,
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0,
            model: Some(model.to_string()),
            parts: Vec::new(),
            light: None,
            radius: None,
            teleport: None,
            emittance: None,
            flags: 0,
            enable_parent: None,
            plugin: String::new(),
            actor: None,
            primitive: None,
            open_by_default: false,
        }]);
        let mut used = vec![false; placed.meshes.len()];
        let pieces = scene
            .draws
            .iter()
            .filter(|d| d.actor.is_none())
            .map(|d| {
                let name = &scene.meshes[d.mesh].shape_name;
                let found = placed
                    .meshes
                    .iter()
                    .enumerate()
                    .position(|(i, m)| !used[i] && m.name == *name);
                match found {
                    Some(i) => {
                        used[i] = true;
                        let m = &placed.meshes[i];
                        EffectPiece {
                            nodes: m.nodes.clone(),
                            billboard: m
                                .billboard
                                .map(|(at, _)| at)
                                .zip(preview::cell::Billboard::of(m)),
                            turns_about_up: m.billboard.filter(|b| b.1 == 1).map(|b| b.0),
                        }
                    }
                    None => EffectPiece {
                        nodes: Vec::new(),
                        billboard: None,
                        turns_about_up: None,
                    },
                }
            })
            .collect();
        (!scene.draws.is_empty()).then_some(EffectModel {
            scene,
            pieces,
            controllers,
            animation,
            particles,
        })
    }
}

/// Where an effect's piece is `time` seconds after the effect appeared and
/// how opaque its material is then: the controllers played from their
/// start (taken to begin when the effect appears: the game keeps the
/// effect exactly as long as their longest span, `00689840`), clamped at
/// their stop; a billboard turned toward a camera at `eye` with right, up
/// and back `axes` (game axes) for the effect placed by `placement` (model
/// to world). The move is a column-major matrix in the model's space from
/// the loaded vertices (`world = placement × move × vertex`); the opacity
/// is `None` when no controller sets it.
pub fn effect_piece_at(
    piece: &EffectPiece,
    controllers: Option<&nif::Sequence>,
    time: f32,
    placement: &nif::Transform,
    eye: [f32; 3],
    axes: [[f32; 3]; 3],
) -> ([f32; 16], Option<f32>) {
    let layers: Vec<(&nif::Sequence, f32)> = controllers
        .map(|s| (s, (s.start + time.max(0.0)).min(s.stop)))
        .into_iter()
        .collect();
    let nodes = &piece.nodes;
    let rest = nif::posed_chain(nodes, &[]);
    let moved = match &piece.billboard {
        Some((at, billboard)) if *at < nodes.len() => {
            let above = nif::posed_chain(&nodes[..*at], &layers);
            let node = nif::posed_chain(&nodes[*at..=*at], &layers);
            let below = nif::posed_chain(&nodes[*at + 1..], &layers);
            billboard.facing_posed((&above, &node, &below), placement, eye, axes)
        }
        // [G] Mode 1, `ROTATE_ABOUT_UP`: turned about the node's own y
        // until its z faces the camera, as Gamebryo describes the mode
        // (the exe's billboard update isn't traced); with `NV_GUESSES=1`
        // only, else drawn as modelled.
        _ => match piece.turns_about_up {
            Some(at) if at < nodes.len() && world::guesses::enabled() => {
                let above = nif::posed_chain(&nodes[..at], &layers);
                let node = nif::posed_chain(&nodes[at..=at], &layers);
                let below = nif::posed_chain(&nodes[at + 1..], &layers);
                let world_node = placement.then_child(&above).then_child(&node);
                let col = |j: usize| {
                    let r = &world_node.rotation;
                    [r[0][j], r[1][j], r[2][j]]
                };
                let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
                let to_eye = [0, 1, 2].map(|k| eye[k] - world_node.translation[k]);
                let theta = dot(to_eye, col(0)).atan2(dot(to_eye, col(2)));
                let (s, c) = theta.sin_cos();
                let turn = nif::Transform {
                    rotation: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
                    translation: [0.0; 3],
                    scale: 1.0,
                };
                above
                    .then_child(&node)
                    .then_child(&turn)
                    .then_child(&below)
                    .then_child(&rest.inverse())
            }
            _ => nif::posed_chain(nodes, &layers).then_child(&rest.inverse()),
        },
    };
    let opacity = nodes.last().and_then(|(shape, _)| {
        layers.iter().find_map(|(s, t)| {
            s.materials
                .iter()
                .filter(|m| m.target == nif::MaterialTarget::Alpha)
                .find(|m| m.node.eq_ignore_ascii_case(shape))
                .and_then(|m| m.float_at(*t))
                .map(|a| a.clamp(0.0, 1.0))
        })
    });
    (crate::column_major(&moved), opacity)
}

/// An effect's placement (`00c4b8a0`): at `point`, its model's axes taken
/// to the world's by `rotation` (`world::impacts::effect_rotation`).
pub fn effect_placement(point: [f32; 3], rotation: [[f32; 3]; 3]) -> nif::Transform {
    nif::Transform {
        rotation,
        translation: point,
        scale: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_effects_pieces_follow_its_controllers_from_when_it_appears() {
        let nif = nif::Nif::parse(testdata::impacts::controlled_effect_nif()).unwrap();
        let controllers = nif.own_controllers().unwrap().unwrap();
        // The shape under the root (whose own transform a placed model
        // leaves out, but whose controller still grows it).
        let piece = EffectPiece {
            nodes: vec![
                ("Root".to_string(), nif::Transform::IDENTITY),
                ("Quad".to_string(), nif::Transform::IDENTITY),
            ],
            billboard: None,
            turns_about_up: None,
        };
        let place = effect_placement(
            [0.0; 3],
            world::impacts::effect_rotation([0.0, 0.0, 1.0], 0.0),
        );
        let axes = [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]];
        let (m, alpha) = effect_piece_at(&piece, Some(&controllers), 0.15, &place, [0.0; 3], axes);
        // Scale 2 halfway, alpha 0.5 at 0.15 s.
        assert!(
            (m[0] - 2.0).abs() < 1e-5 && (m[10] - 2.0).abs() < 1e-5,
            "{m:?}"
        );
        assert!((alpha.unwrap() - 0.5).abs() < 1e-5);
        // Held at the stop afterwards; at the start before.
        let (late, gone) = effect_piece_at(&piece, Some(&controllers), 5.0, &place, [0.0; 3], axes);
        assert!((late[0] - 3.0).abs() < 1e-5);
        assert_eq!(gone, Some(0.0));
        let (first, full) =
            effect_piece_at(&piece, Some(&controllers), 0.0, &place, [0.0; 3], axes);
        assert!((first[0] - 1.0).abs() < 1e-5);
        assert_eq!(full, Some(1.0));
        // No controllers: where it was loaded, its own opacity.
        let (still, none) = effect_piece_at(&piece, None, 0.1, &place, [0.0; 3], axes);
        assert_eq!(still, crate::column_major(&nif::Transform::IDENTITY));
        assert_eq!(none, None);
    }

    /// A model whose root node carries the controllers playing `times`
    /// but the last, which is on its child shape.
    fn animated_nif(times: &[(f32, f32)]) -> nif::Nif {
        nif::Nif::parse(testdata::impacts::animated_effect_nif(times)).unwrap()
    }

    #[test]
    fn an_effect_lasts_its_longest_controller() {
        // Root 0.5–2.0 (1.5 s), the shape 0–3.25: the shape's, found
        // through the root's children.
        let nif = animated_nif(&[(0.5, 2.0), (0.0, 3.25)]);
        assert_eq!(animation_time(&nif), Some(3.25));
        // A chain on the root: 1, then 4 (the longest), then the shape 2.
        let chain = animated_nif(&[(0.0, 1.0), (1.0, 5.0), (0.0, 2.0)]);
        assert_eq!(animation_time(&chain), Some(4.0));
        let still = animated_nif(&[]);
        assert_eq!(animation_time(&still), None);
    }
}
