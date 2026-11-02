//! Blackjack's 3D scene, put together as the game's code does it
//! (`Prepare3DElements` `00732540`, `InitCasinoData` `00731d70`;
//! `world::casino::blackjack` runs the menu, notes in `docs/CASINO.md`):
//!
//! - a new node holds, in order, the casino's table (its model 8, `MOD3`),
//!   `NV_Blackjack-Hand1.NIF`, `-Hand2.NIF`, `-Dealer.NIF`, then each chip
//!   stack (the casino's models 0 to 5) followed by its shadow (the stack's
//!   path up to its first `.`, then `_Shadow.NIF`; most stacks have none),
//!   all as authored at the node's origin;
//! - the camera is the table's `object1` (`GetObjectByName`, the
//!   `NiCamera` under `object0`) with the file's frustum (no animation
//!   moves it);
//! - the lights are the table's point lights with their radius 3 × the
//!   node's bound radius (`00b5ca70`, `01021928`), their attenuation zeroed.

use nif::math::Transform;

use crate::caravan::{ModelData, Pose};
use crate::{LightData, ViewerScene};
use world::casino::blackjack::{Model, HAND_MODELS};

/// The lights' radius: this × the node's bound radius.
pub const LIGHT_RADIUS_MULT: f32 = 3.0;

/// The scene's models in the node's order (`Prepare3DElements`).
pub fn model_order() -> Vec<Model> {
    let mut v = vec![Model::Table, Model::Player, Model::Split, Model::Dealer];
    for i in 0..6 {
        v.push(Model::Chip(i));
        v.push(Model::Shadow(i));
    }
    v
}

/// A model's place in [`model_order`].
pub fn index(model: Model) -> usize {
    match model {
        Model::Table => 0,
        Model::Player => 1,
        Model::Split => 2,
        Model::Dealer => 3,
        Model::Chip(i) => 4 + 2 * i.min(5),
        Model::Shadow(i) => 5 + 2 * i.min(5),
    }
}

/// A chip stack's shadow model (`00731d70`): the path up to its first `.`
/// and `_Shadow.NIF`.
pub fn shadow_path(chip: &str) -> String {
    let stem = chip.split('.').next().unwrap_or(chip);
    format!("{stem}_Shadow.NIF")
}

/// Everything the menu shows, ready to draw.
pub struct BlackjackModels {
    /// Every model's pieces (each draw's `reference` is its index in
    /// [`model_order`] + 1).
    pub scene: ViewerScene,
    pub models: Vec<ModelData>,
    /// The camera's frustum as the table's file has it.
    pub frustum: Option<nif::camera::Frustum>,
    /// The table's point lights (names lower case) and the radius they all
    /// get.
    pub lights: Vec<(String, nif::Light)>,
    pub light_radius: f32,
}

impl BlackjackModels {
    /// The camera's place (the table's `object1`) in the table's pose.
    pub fn camera(&self, table: &Pose) -> Option<Transform> {
        table.world(self.models.first()?, "object1")
    }

    /// The table's lights where its pose has them.
    pub fn lights_now(&self, table: &Pose) -> Vec<LightData> {
        let Some(model) = self.models.first() else {
            return Vec::new();
        };
        self.lights
            .iter()
            .filter_map(|(name, l)| {
                let at = table.world(model, name)?;
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

/// A card's texture by its path under `Data\` (`deck_textures`' names).
pub fn card_texture(assets: &assets::Assets, path: &str) -> Option<crate::TextureData> {
    let (real, bytes) = crate::read_texture(assets, path)?;
    crate::TextureData::from_dds(real, bytes).ok()
}

/// Loads the table (`table`: the casino's model 8), the hands and the chip
/// stacks (`chips`: the casino's models 0 to 5), relative to `Meshes\`;
/// `None` when the table can't be read.
pub fn load(assets: &assets::Assets, table: &str, chips: &[String]) -> Option<BlackjackModels> {
    let read = |path: &str| -> Option<nif::Nif> { nif::Nif::parse(assets.read(path).ok()??).ok() };
    let mut files = vec![assets::mesh_path(table)];
    files.extend(HAND_MODELS.iter().map(|s| s.to_string()));
    for i in 0..6 {
        let chip = chips.get(i).cloned().unwrap_or_default();
        files.push(assets::mesh_path(&chip));
        files.push(assets::mesh_path(&shadow_path(&chip)));
    }
    let nifs: Vec<Option<nif::Nif>> = files.iter().map(|f| read(f)).collect();
    let table_nif = nifs.first()?.as_ref()?;
    let bound = nifs.iter().flatten().fold(None, |acc, n| {
        n.roots().iter().fold(acc, |acc, &r| {
            crate::lockpick::merge(
                acc,
                crate::lockpick::bound(n, r, &Transform::IDENTITY, None),
            )
        })
    });
    let light_radius = LIGHT_RADIUS_MULT * bound.map_or(0.0, |(_, r)| r);
    let lights: Vec<(String, nif::Light)> = table_nif
        .lights()
        .unwrap_or_default()
        .into_iter()
        .filter(|l| l.kind == nif::LightKind::Point)
        .map(|l| (l.name.to_ascii_lowercase(), l))
        .collect();
    let frustum = (0..table_nif.blocks().len())
        .find(|&i| {
            table_nif.block_type(i) == "NiCamera"
                && table_nif
                    .block_name(i)
                    .is_some_and(|n| n.eq_ignore_ascii_case("object1"))
        })
        .and_then(|i| table_nif.camera_frustum(i).ok().flatten())
        .map(|(f, _)| f);
    let models: Vec<ModelData> = nifs
        .iter()
        .map(|n| n.as_ref().map(ModelData::read).unwrap_or_default())
        .collect();
    let mut cell = world::LoadedCell::actors_only(Vec::new());
    for (i, (file, nif)) in files.iter().zip(&nifs).enumerate() {
        if nif.is_none() {
            continue;
        }
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
    Some(BlackjackModels {
        scene,
        models,
        frustum,
        lights,
        light_radius,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_and_shadows() {
        let order = model_order();
        assert_eq!(order.len(), 16);
        for (i, m) in order.iter().enumerate() {
            assert_eq!(index(*m), i);
        }
        assert_eq!(
            shadow_path("terminals\\NV_Blackjack\\NV_Blackjack-Chip_001.NIF"),
            "terminals\\NV_Blackjack\\NV_Blackjack-Chip_001_Shadow.NIF"
        );
    }
}
