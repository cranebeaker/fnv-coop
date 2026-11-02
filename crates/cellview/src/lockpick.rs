//! The lockpicking menu's 3D scene, put together as the game's code does
//! it (`0078e1c0`, the camera `0078e900`, drawn by `0078e7c0` after the
//! image space pass and before the menus' pictures, `00872940`):
//!
//! - a new node holds the lock (`meshes\terminals\LockInterface01.NIF`,
//!   its top node as the file has it) and the bobby pin
//!   (`BobbyPin01.NIF`, its top node set to no turn and (0, 0, −1.6),
//!   `010744c0`); the node is turned by X(π/2) · Z(π/2) (`00524ac0`,
//!   `004a0c90`, `0043f8d0`: rows (0, 1, 0), (0, 0, 1), (1, 0, 0)) and
//!   moved (125, 0, 0) (`0103128c`);
//! - the camera is a new `NiCamera` left at the origin with no turn (its
//!   `LookAtWorldPoint((0, 0, 1), (0, 0, 1))` is degenerate and changes
//!   nothing), so it looks along +x with +y up and +z to the right; its
//!   frustum is ±0.75 · tan(`fDefaultFOV` × 0.15°) across and that × the
//!   screen's height / width up and down (`01016264`, `01070c80`), near
//!   `fNearDistance` (both `[Display]` INI settings);
//! - the lights are the lock model's point lights, given to the menus'
//!   scene by `00b5ca70` (→ `00b5c940`, which takes point and directional
//!   lights only, not its ambient light) with their radius set to three
//!   times the new node's bounding radius and their attenuation zeroed
//!   (light +0xe0, +0xe4, +0xe8). The scene's own light (`00b5e0f0`) has a
//!   black ambient and colour.
//!
//! The rules of the game are in `world::lockpick`.

use std::sync::Arc;

use nif::math::{Transform, Vec3};

use crate::{Game, LightData, ViewerScene};

/// Where the scene's node sits in front of the camera (`0103128c`).
pub const ROOT_TRANSLATION: Vec3 = [125.0, 0.0, 0.0];
/// The lights' radius: this × the scene node's bounding radius (`01021928`
/// × `0084d030` on the node's world bound, `0078e1c0`).
pub const LIGHT_RADIUS_MULT: f32 = 3.0;
/// The frustum's width factor and the field of view's share
/// (`01016264` 0.75, `01070c80` 0.15).
pub const FRUSTUM_SCALE: f32 = 0.75;
pub const FOV_SHARE: f32 = 0.15;

/// The scene node's turn: X(π/2) · Z(π/2), with the sines and cosines of a
/// single-precision π/2 (`0101ff38`) as the game works them out.
pub fn root_rotation() -> [[f32; 3]; 3] {
    let a = std::f32::consts::FRAC_PI_2;
    let (s, c) = a.sin_cos();
    // `00524ac0` and `004a0c90` (rows).
    let x = [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]];
    let z = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
    nif::math::mat_mul(&x, &z)
}

/// The scene node's transform.
pub fn root_transform() -> Transform {
    Transform {
        rotation: root_rotation(),
        translation: ROOT_TRANSLATION,
        scale: 1.0,
    }
}

/// The menu camera's frustum (`0078e900`): the tangent of half its width
/// and of half its height, and its near plane (game units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuCamera {
    pub tan_half_width: f32,
    pub tan_half_height: f32,
    pub near: f32,
}

impl MenuCamera {
    /// For a screen of `width` × `height` pixels with the INI's
    /// `fDefaultFOV` (75 when unset, `0102f0f8`) and `fNearDistance` (5,
    /// `0101712c`).
    pub fn new(settings: &assets::IniSettings, width: u32, height: u32) -> MenuCamera {
        MenuCamera::with_share(settings, width, height, FOV_SHARE)
    }

    /// [`MenuCamera::new`] with another share of the field of view (the
    /// rendered terminal's `fRenderedTerminalFOV`, `007feeb0`).
    pub fn with_share(
        settings: &assets::IniSettings,
        width: u32,
        height: u32,
        share: f32,
    ) -> MenuCamera {
        let fov = settings.float("Display", "fDefaultFOV").unwrap_or(75.0);
        let near = settings.float("Display", "fNearDistance").unwrap_or(5.0);
        let angle = (f64::from(fov) * f64::from(1.0f32.to_radians()) * f64::from(share)) as f32;
        let tan_half_width = angle.tan() * FRUSTUM_SCALE;
        let aspect = height as f32 / width.max(1) as f32;
        MenuCamera {
            tan_half_width,
            tan_half_height: angle.tan() * aspect * FRUSTUM_SCALE,
            near,
        }
    }

    /// The vertical field of view (radians) a symmetric camera needs.
    pub fn vertical_fov(&self) -> f32 {
        2.0 * self.tan_half_height.atan()
    }
}

/// The two models, ready to draw.
pub struct LockpickScene {
    /// Both models' pieces at the scene node (the lock's draws carry
    /// reference 1, the pin's 2), with their textures.
    pub scene: ViewerScene,
    pub lock_sequences: Arc<Vec<nif::Sequence>>,
    pub pin_sequences: Arc<Vec<nif::Sequence>>,
    /// The lock model's point lights in the scene's space (the camera's:
    /// game units, x forward, y up, z right), colour the diffuse colour and
    /// fade the dimmer.
    pub lights: Vec<LightData>,
    /// The scene node's bounding radius at setup.
    pub bound_radius: f32,
    /// The pin file's own top-node transform, which the game replaces
    /// (`0078e1c0`, `00790df0`); the pin's pieces are built with it, so it's
    /// taken back out.
    pub pin_root: Transform,
}

/// The lock's draws (reference 1) and the pin's (reference 2).
pub const LOCK_REFERENCE: u32 = 1;
pub const PIN_REFERENCE: u32 = 2;

/// Gamebryo's sphere merge (`NiBound::Merge`): a sphere of no size is
/// empty; one that holds the other is kept; else the smallest sphere
/// round both.
pub(crate) fn merge(a: Option<(Vec3, f32)>, b: Option<(Vec3, f32)>) -> Option<(Vec3, f32)> {
    let (Some((ca, ra)), Some((cb, rb))) = (a, b) else {
        return a.or(b);
    };
    let d = [cb[0] - ca[0], cb[1] - ca[1], cb[2] - ca[2]];
    let len2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let dr = rb - ra;
    if dr * dr >= len2 {
        return Some(if dr >= 0.0 { (cb, rb) } else { (ca, ra) });
    }
    let len = len2.sqrt();
    let radius = 0.5 * (len + ra + rb);
    let k = (radius - ra) / len;
    Some((
        [ca[0] + k * d[0], ca[1] + k * d[1], ca[2] + k * d[2]],
        radius,
    ))
}

/// A block's bounding sphere in its parent's space with `parent` above:
/// nodes merge their children in order; shapes give their stored sphere;
/// anything else (lights) gives none.
pub(crate) fn bound(
    nif: &nif::Nif,
    index: i32,
    parent: &Transform,
    own: Option<&Transform>,
) -> Option<(Vec3, f32)> {
    let index = usize::try_from(index).ok()?;
    match nif.block(index).ok()? {
        nif::Block::Node(node) => {
            let world = parent.then_child(own.unwrap_or(&node.av.transform));
            node.children
                .iter()
                .fold(None, |acc, &c| merge(acc, bound(nif, c, &world, None)))
        }
        nif::Block::Geometry(g) => {
            let world = parent.then_child(&g.av.transform);
            let data = usize::try_from(g.data).ok()?;
            let nif::Block::GeometryData(d) = nif.block(data).ok()? else {
                return None;
            };
            (d.radius > 0.0).then(|| (world.apply_point(d.center), d.radius * world.scale))
        }
        _ => None,
    }
}

impl Game {
    /// The lockpicking menu's models and lights ([`lockpick_scene`]).
    pub fn lockpick_scene(&self) -> Option<LockpickScene> {
        lockpick_scene(&self.assets)
    }
}

/// The lockpicking menu's models and lights (`0078e1c0`); `None` when
/// either model can't be read.
pub fn lockpick_scene(assets: &assets::Assets) -> Option<LockpickScene> {
    {
        let read =
            |path: &str| -> Option<nif::Nif> { nif::Nif::parse(assets.read(path).ok()??).ok() };
        let lock = read(world::lockpick::LOCK_MODEL)?;
        let pin = read(world::lockpick::PIN_MODEL)?;
        let pin_root = Transform {
            rotation: nif::math::IDENTITY3,
            translation: world::lockpick::PIN_OFFSET,
            scale: 1.0,
        };
        // The node's bound at setup: the lock as the file has it, the pin
        // at its offset; the node's own turn doesn't change the radius.
        let lock_bound = lock.roots().iter().fold(None, |acc, &r| {
            merge(acc, bound(&lock, r, &Transform::IDENTITY, None))
        });
        let pin_bound = pin
            .roots()
            .first()
            .and_then(|&r| bound(&pin, r, &Transform::IDENTITY, Some(&pin_root)));
        let pin_file_root = pin
            .roots()
            .first()
            .and_then(|&r| usize::try_from(r).ok())
            .and_then(|r| match pin.block(r).ok()? {
                nif::Block::Node(node) => Some(node.av.transform),
                _ => None,
            })
            .unwrap_or(Transform::IDENTITY);
        let bound_radius = merge(lock_bound, pin_bound).map_or(0.0, |(_, r)| r);
        let root = root_transform();
        let lights = lock
            .lights()
            .ok()?
            .into_iter()
            // The lock model's lights are two point lights and an ambient
            // one; the ambient one isn't taken (`00b5c940`).
            .filter(|l| l.kind == nif::LightKind::Point)
            .map(|l| LightData {
                position: root.apply_point(l.position()),
                color: l.diffuse,
                radius: LIGHT_RADIUS_MULT * bound_radius,
                fade: l.dimmer,
            })
            .collect();
        let sequences = |n: &nif::Nif| Arc::new(n.sequences().unwrap_or_default());
        let (lock_sequences, pin_sequences) = (sequences(&lock), sequences(&pin));

        // Both models as placed objects at the node, keeping their top
        // nodes' transforms (the pin's is set in the viewer each frame).
        let mut cell = world::LoadedCell::actors_only(Vec::new());
        for (reference, model) in [
            (LOCK_REFERENCE, world::lockpick::LOCK_MODEL),
            (PIN_REFERENCE, world::lockpick::PIN_MODEL),
        ] {
            cell.objects.push(world::Placement {
                form_id: esm::FormId(reference),
                record_type: esm::FourCC::new(b"REFR"),
                editor_id: None,
                base: esm::FormId(0),
                base_type: esm::FourCC::new(b"STAT"),
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
            });
        }
        let built = preview::cell::build_scene_with(assets, cell, true);
        let scene = crate::convert_cell(&built, assets);
        Some(LockpickScene {
            scene,
            lock_sequences,
            pin_sequences,
            lights,
            bound_radius,
            pin_root: pin_file_root,
        })
    }
}

/// A sequence by name.
pub fn sequence<'a>(all: &'a [nif::Sequence], name: &str) -> Option<&'a nif::Sequence> {
    all.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

/// The spans `world::lockpick` plays (`0078e1c0`: the lock's `Forward` and
/// `Backward`, the pin's `Forward`, `Backward` and `Left`).
pub fn spans(lock: &[nif::Sequence], pin: &[nif::Sequence]) -> world::lockpick::Sequences {
    let span = |s: Option<&nif::Sequence>| {
        s.map(|s| world::lockpick::Span {
            begin: s.start,
            end: s.stop,
        })
    };
    world::lockpick::Sequences {
        lock_forward: span(sequence(lock, "Forward")),
        lock_backward: span(sequence(lock, "Backward")),
        pin_forward: span(sequence(pin, "Forward")),
        pin_backward: span(sequence(pin, "Backward")),
        pin_left: span(sequence(pin, "Left")),
    }
}

impl LockpickScene {
    /// Where a piece is drawn (column-major, the scene's space): the scene
    /// node; for the pin's pieces (`reference` [`PIN_REFERENCE`]) its top
    /// node as the game sets it for the pick at `x` on a meter `width` wide
    /// (`00790df0`) in place of the file's own; then the piece's move in its
    /// model's sequence at a time (`layer`; `None`: as loaded).
    pub fn piece_matrix(
        &self,
        reference: u32,
        motion: Option<&crate::PieceMotion>,
        layer: Option<(&nif::Sequence, f32)>,
        pick: (f32, f32),
    ) -> [f32; 16] {
        let mut t = node_transform(reference, &self.pin_root, pick);
        if let (Some(m), Some(layer)) = (motion, layer) {
            t = t.then_child(&m.motion.transform(&[layer]));
        }
        crate::column_major(&t)
    }
}

/// The scene node, and for the pin's pieces its top node for the pick at
/// `x` on a meter `width` wide with the file's own top node (`file_root`)
/// taken back out.
fn node_transform(reference: u32, file_root: &Transform, (x, width): (f32, f32)) -> Transform {
    let t = root_transform();
    if reference != PIN_REFERENCE {
        return t;
    }
    let (rotation, translation) = world::lockpick::pin_transform(x, width);
    t.then_child(&Transform {
        rotation,
        translation,
        scale: 1.0,
    })
    .then_child(&file_root.inverse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scene_node_faces_the_camera() {
        // Model +y (the lock's face looks down −y) runs away from the
        // camera (+x), model +z is up (+y), model +x is right (+z).
        let r = root_rotation();
        let apply = |v: Vec3| nif::math::mat_vec(&r, v);
        let close = |a: Vec3, b: Vec3| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-6);
        assert!(close(apply([0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]));
        assert!(close(apply([0.0, 0.0, 1.0]), [0.0, 1.0, 0.0]));
        assert!(close(apply([1.0, 0.0, 0.0]), [0.0, 0.0, 1.0]));
    }

    #[test]
    fn spheres_merge_as_gamebryo_merges_them() {
        let a = Some(([0.0, 0.0, 0.0], 1.0));
        let b = Some(([4.0, 0.0, 0.0], 1.0));
        assert_eq!(merge(a, b), Some(([2.0, 0.0, 0.0], 3.0)));
        // One inside the other: the bigger.
        let big = Some(([0.0, 0.0, 0.0], 10.0));
        assert_eq!(merge(b, big), big);
        assert_eq!(merge(big, b), big);
        assert_eq!(merge(None, b), b);
    }

    #[test]
    fn the_pin_hangs_from_its_own_node() {
        let close = |a: Vec3, b: Vec3| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4);
        let root = root_transform();
        // The lock's pieces: the scene node only.
        let lock = node_transform(LOCK_REFERENCE, &Transform::IDENTITY, (0.0, 100.0));
        assert!(close(
            lock.apply_point([1.0, 2.0, 3.0]),
            root.apply_point([1.0, 2.0, 3.0])
        ));
        // The pin in the meter's middle: its node (0, 0, −1.6) with no
        // turn, and a file top node moved by (5, 0, 0) taken back out.
        let file = Transform {
            rotation: nif::math::IDENTITY3,
            translation: [5.0, 0.0, 0.0],
            scale: 1.0,
        };
        let mid = node_transform(PIN_REFERENCE, &file, (50.0, 100.0));
        assert!(close(
            mid.apply_point([5.0, 0.0, 0.0]),
            root.apply_point([0.0, 0.0, -1.6])
        ));
        // At the left end it has turned 90° about the pivot (0, 0, 0.6):
        // the pivot stays put.
        let left = node_transform(PIN_REFERENCE, &Transform::IDENTITY, (0.0, 100.0));
        assert!(close(
            left.apply_point([0.0, 0.0, 2.2]),
            root.apply_point([0.0, 0.0, 0.6])
        ));
    }

    #[test]
    fn the_menu_camera() {
        // 75 × 0.15 = 11.25°: 0.75 × tan 11.25° across; × 9/16 up and down.
        let ini = assets::IniSettings::default();
        let c = MenuCamera::new(&ini, 1920, 1080);
        assert!((c.tan_half_width - 0.149_184).abs() < 1e-5);
        assert!((c.tan_half_height - 0.149_184 * 0.5625).abs() < 1e-5);
        assert_eq!(c.near, 5.0);
    }
}
