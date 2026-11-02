//! The slot machine's 3D scene, put together as the game's code does it
//! (`Prepare3DElements` `007c1af0`; `world::casino::slots` runs the menu,
//! notes in `docs/CASINO.md`):
//!
//! - a new node holds the casino's machine (its model 7) and the three
//!   reels (`NV_SlotMachine-Reel01..03.NIF`), children 0 to 3, as authored;
//!   the node is turned by X(π/2) · Z(π/2) and moved (175, −18, 0), the
//!   lockpicking menu's turn ([`crate::lockpick::root_rotation`]);
//! - the camera is the lockpicking menu's (`007c2180`, the shared builder:
//!   at the origin looking along +x, `fDefaultFOV` × 0.15 across,
//!   [`crate::lockpick::MenuCamera`]);
//! - the lights are the machine's point lights with their radius 3 × the
//!   node's bound radius (`00b5ca70`, `01021928`), their attenuation zeroed;
//! - the reels' faces take the casino's seven symbol textures and the
//!   blank's (`NV_SlotMachine-SymbolS.dds`) as the menu swaps them.

use nif::math::{Transform, Vec3};

use crate::caravan::ModelData;
use crate::{LightData, TextureData, ViewerScene};

/// Where the node sits in front of the camera (`0105478c` 175,
/// `0107678c` −18).
pub const ROOT_TRANSLATION: Vec3 = [175.0, -18.0, 0.0];
/// The lights' radius: this × the node's bound radius.
pub const LIGHT_RADIUS_MULT: f32 = 3.0;

/// The node's transform.
pub fn root_transform() -> Transform {
    Transform {
        rotation: crate::lockpick::root_rotation(),
        translation: ROOT_TRANSLATION,
        scale: 1.0,
    }
}

/// Everything the menu shows, ready to draw.
pub struct SlotModels {
    /// The four models' pieces (each draw's `reference` is its model's
    /// index + 1: 1 the machine, 2 to 4 the reels).
    pub scene: ViewerScene,
    pub models: Vec<ModelData>,
    /// The machine's point lights (names lower case) and the radius they
    /// all get.
    pub lights: Vec<(String, nif::Light)>,
    pub light_radius: f32,
    /// The reels' textures by slot: the casino's seven, then the blank.
    pub reel_textures: Vec<Option<TextureData>>,
}

impl SlotModels {
    /// The lights where the machine has them, in the camera's space.
    pub fn lights_now(&self) -> Vec<LightData> {
        let Some(machine) = self.models.first() else {
            return Vec::new();
        };
        let root = root_transform();
        let rest = crate::caravan::Pose::default();
        self.lights
            .iter()
            .filter_map(|(name, l)| {
                let at = root.then_child(&rest.world(machine, name)?);
                Some(LightData {
                    position: at.apply_point([0.0; 3]),
                    color: l.diffuse,
                    radius: self.light_radius,
                    fade: l.dimmer,
                })
            })
            .collect()
    }
}

/// Loads the machine (`machine`: the casino's model 7, relative to
/// `Meshes\`), the reels and the reels' textures (`symbols`: the casino's
/// textures 0 to 6, relative to `Textures\`); `None` when the machine
/// can't be read.
pub fn load(assets: &assets::Assets, machine: &str, symbols: &[String]) -> Option<SlotModels> {
    let read = |path: &str| -> Option<nif::Nif> { nif::Nif::parse(assets.read(path).ok()??).ok() };
    let mut files = vec![assets::mesh_path(machine)];
    files.extend(
        world::casino::slots::REEL_MODELS
            .iter()
            .map(|s| s.to_string()),
    );
    let nifs: Vec<Option<nif::Nif>> = files.iter().map(|f| read(f)).collect();
    let machine_nif = nifs.first()?.as_ref()?;
    // The node's bound with all four in it.
    let bound = nifs.iter().flatten().fold(None, |acc, n| {
        n.roots().iter().fold(acc, |acc, &r| {
            crate::lockpick::merge(
                acc,
                crate::lockpick::bound(n, r, &Transform::IDENTITY, None),
            )
        })
    });
    let light_radius = LIGHT_RADIUS_MULT * bound.map_or(0.0, |(_, r)| r);
    let lights: Vec<(String, nif::Light)> = machine_nif
        .lights()
        .unwrap_or_default()
        .into_iter()
        .filter(|l| l.kind == nif::LightKind::Point)
        .map(|l| (l.name.to_ascii_lowercase(), l))
        .collect();
    let models: Vec<ModelData> = nifs
        .iter()
        .map(|n| n.as_ref().map(ModelData::read).unwrap_or_default())
        .collect();
    let mut cell = world::LoadedCell::actors_only(Vec::new());
    for (i, file) in files.iter().enumerate() {
        cell.objects.push(world::Placement {
            form_id: esm::FormId(i as u32 + 1),
            record_type: esm::FourCC::new(b"REFR"),
            editor_id: None,
            base: esm::FormId(0),
            base_type: esm::FourCC::new(b"STAT"),
            base_editor_id: None,
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0,
            model: Some(file.clone()),
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
    let texture = |path: &str| {
        let (real, bytes) = crate::read_texture(assets, path)?;
        TextureData::from_dds(real, bytes).ok()
    };
    let mut reel_textures: Vec<Option<TextureData>> = symbols
        .iter()
        .take(7)
        .map(|s| texture(&assets::texture_path(s)))
        .collect();
    reel_textures.resize_with(7, || None);
    reel_textures.push(texture(world::casino::slots::BLANK_TEXTURE_PATH));
    Some(SlotModels {
        scene,
        models,
        lights,
        light_radius,
        reel_textures,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The node's turn takes the machine's +x to the screen's right (+z of
    /// the camera), +y away from the camera (+x), +z up (+y).
    #[test]
    fn the_node_faces_the_camera() {
        let r = root_transform();
        let o = r.apply_point([0.0; 3]);
        let along = |v: Vec3| {
            let p = r.apply_point(v);
            [p[0] - o[0], p[1] - o[1], p[2] - o[2]]
        };
        let close = |a: Vec3, b: Vec3| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5);
        assert!(close(along([1.0, 0.0, 0.0]), [0.0, 0.0, 1.0]));
        assert!(close(along([0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]));
        assert!(close(along([0.0, 0.0, 1.0]), [0.0, 1.0, 0.0]));
        assert_eq!(o, ROOT_TRANSLATION);
    }
}
