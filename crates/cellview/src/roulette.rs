//! Roulette's 3D scene, put together as the game's code does it
//! (`Prepare3DElements` `007b94c0`, `PrepareBetTiles` `007b9c40`;
//! `world::casino::roulette` runs the menu, notes in `docs/CASINO.md`):
//!
//! - a new node holds, in order, the casino's table (its model 9, `MOD4`),
//!   `NV_Roulette-Points.NIF` (the spots) and `NV_Roulette-Bet.NIF` (the
//!   cursor's rings), each 0.1 above the table, then ten of the casino's
//!   chip (its model 6) and ten of its shadow (the chip's path up to its
//!   first `.`, then `_Shadow.NIF`): children 3 to 12 and 13 to 22;
//! - the camera is the table's `object4` (the `NiCamera` riding the
//!   `Camera01` rig the spins move) with the file's frustum;
//! - the lights are the table's point lights (on the rig too) with their
//!   radius 4 × the node's bound radius (`00b5ca70`);
//! - each spot is at its Points mesh's centre (`A01:0` … `F24:0`, `S01:0` …
//!   `S15:0`), 0.1 up.

use nif::math::Transform;

use crate::caravan::{ModelData, Pose};
use crate::{LightData, ViewerScene};

/// The lights' radius: this × the node's bound radius.
pub const LIGHT_RADIUS_MULT: f32 = 4.0;
/// The spots' and the cursor's models.
pub const POINTS_MODEL: &str = "meshes\\terminals\\nv_roulette\\nv_roulette-points.nif";
pub const BET_MODEL: &str = "meshes\\terminals\\nv_roulette\\nv_roulette-bet.nif";
/// How far above the table the spots and the cursor sit.
pub const LIFT: f32 = 0.1;
/// The cursor's rings (`Bet_Selector:0` on a spot, `Bet_Invalid:0` off).
pub const VALID_RING: &str = "bet_selector:0";
pub const INVALID_RING: &str = "bet_invalid:0";
/// The ring the spins' textures turn (`ShiftWheelTexture`).
pub const WHEEL_RING: &str = "roulette_wheel:2";

/// Everything the menu shows, ready to draw.
pub struct RouletteModels {
    /// Every model's pieces (each draw's `reference` is its child index +
    /// 1: the table 0, the spots 1, the cursor 2, chips 3 to 12, shadows 13
    /// to 22).
    pub scene: ViewerScene,
    pub models: Vec<ModelData>,
    pub frustum: Option<nif::camera::Frustum>,
    pub lights: Vec<(String, nif::Light)>,
    pub light_radius: f32,
    /// Each spot's place in the node's space (`world::casino::roulette::
    /// spots`' order).
    pub places: Vec<[f32; 3]>,
}

impl RouletteModels {
    /// The camera's place (the table's `object4`) in the table's pose.
    pub fn camera(&self, table: &Pose) -> Option<Transform> {
        table.world(self.models.first()?, "object4")
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

/// Loads the table (`table`: the casino's model 9) and the chip (`chip`:
/// its model 6), relative to `Meshes\`, with the spots and the cursor;
/// `None` when the table can't be read.
pub fn load(assets: &assets::Assets, table: &str, chip: &str) -> Option<RouletteModels> {
    let read = |path: &str| -> Option<nif::Nif> { nif::Nif::parse(assets.read(path).ok()??).ok() };
    let mut files = vec![
        assets::mesh_path(table),
        POINTS_MODEL.to_string(),
        BET_MODEL.to_string(),
    ];
    files.extend(std::iter::repeat(assets::mesh_path(chip)).take(10));
    let shadow = crate::blackjack::shadow_path(chip);
    files.extend(std::iter::repeat(assets::mesh_path(&shadow)).take(10));
    let nifs: Vec<Option<nif::Nif>> = files.iter().map(|f| read(f)).collect();
    let table_nif = nifs.first()?.as_ref()?;
    let bound = nifs.iter().take(3).flatten().fold(None, |acc, n| {
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
                    .is_some_and(|n| n.eq_ignore_ascii_case("object4"))
        })
        .and_then(|i| table_nif.camera_frustum(i).ok().flatten())
        .map(|(f, _)| f);
    let models: Vec<ModelData> = nifs
        .iter()
        .map(|n| n.as_ref().map(ModelData::read).unwrap_or_default())
        .collect();
    // The spots: each Points mesh's centre, lifted with its model.
    let points = &models[1];
    let mut places = Vec::with_capacity(159);
    for i in 0..159usize {
        let name = if i < 144 {
            format!("{}{:02}:0", (b'a' + (i / 24) as u8) as char, i % 24 + 1)
        } else {
            format!("s{:02}:0", i - 143)
        };
        let c = points.centers.get(&name).copied().unwrap_or([f32::MAX; 3]);
        places.push([c[0], c[1], c[2] + LIFT]);
    }
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
    Some(RouletteModels {
        scene,
        models,
        frustum,
        lights,
        light_radius,
        places,
    })
}
