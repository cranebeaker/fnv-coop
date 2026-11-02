//! The Caravan menu's 3D table, put together as the game's code does it
//! (`world::caravan::menu` runs the menu; notes in `docs/CARAVAN.md`):
//!
//! - a new node holds everything, as authored (no transform is set):
//!   the table (`NV_Caravan-Table.NIF`, `PrepareShared3DElements`
//!   `0073cbf0`); the deck screen's `NV_Caravan-Available.NIF` and
//!   `NV_Caravan-Deck.NIF` (`PrepareDeckMenu`); the game's 42 rows
//!   (`Player_{t+1}_0{r+1}.NIF`, `Opponent_{t+1}_0{r+1}.NIF`), the two
//!   markers (`Select_Add.NIF`, `Select_Remove.NIF`) and hands
//!   (`Player_Deck.NIF`, `Opponent_Deck.NIF`, `PrepareGameMenu` `0073ea90`);
//!   the ante's bills and coins, copies of `Currency-Bill_{1..3}.NIF` and
//!   `Currency-Coin_{1..3}.NIF` (`LoadNewBill`, `LoadNewCoin`). The six grids
//!   (`PlayerGrid_{1..3}`, `OppGrid_{1..3}`) and the money's two are read
//!   for their shapes' places, never drawn.
//! - the camera is the table's `object0` (an `NiCamera` under `Camera01`,
//!   under `Dummy05`, which the table's `Bet_to_Deck`, `Deck_to_Play` and
//!   `Play_to_Bet` move) with the file's frustum (45° across, 16:9, near 1,
//!   far 5000);
//! - the lights are the table's three point lights (also under `Dummy05`,
//!   so they move with the camera), their radius 2.5 × the node's bound
//!   radius with only the table in it, their attenuation zeroed
//!   (`00b5ca70`, as the lockpicking menu's).
//! - poses stay where a model's last sequence left them: a sequence moves
//!   the nodes it has tracks for, from its first update after it starts
//!   (Gamebryo's `NiControllerSequence`).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use nif::math::{Transform, Vec3};

use crate::{LightData, TextureData, ViewerScene};

/// Where the models are.
pub const FOLDER: &str = "Meshes\\Terminals\\NV_Caravan\\";
/// The lights' radius: this × the node's bound radius (`01018c00`).
pub const LIGHT_RADIUS_MULT: f32 = 2.5;

/// The models drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    Table,
    Deck,
    Available,
    Hand {
        npc: bool,
    },
    Row {
        track: usize,
        row: usize,
    },
    /// `Select_Add` (`valid`) or `Select_Remove`.
    Cursor {
        valid: bool,
    },
    Bill(usize),
    Coin(usize),
}

impl Part {
    /// The model's file.
    pub fn file(self) -> String {
        let name = match self {
            Part::Table => "NV_Caravan-Table.NIF".to_string(),
            Part::Deck => "NV_Caravan-Deck.NIF".to_string(),
            Part::Available => "NV_Caravan-Available.NIF".to_string(),
            Part::Hand { npc: false } => "Player_Deck.NIF".to_string(),
            Part::Hand { npc: true } => "Opponent_Deck.NIF".to_string(),
            Part::Row { track, row } if track < 3 => {
                format!("Player_{}_0{}.NIF", track + 1, row + 1)
            }
            Part::Row { track, row } => format!("Opponent_{}_0{}.NIF", track - 2, row + 1),
            Part::Cursor { valid: true } => "Select_Add.NIF".to_string(),
            Part::Cursor { valid: false } => "Select_Remove.NIF".to_string(),
            Part::Bill(k) => format!("Currency-Bill_{}.NIF", k + 1),
            Part::Coin(k) => format!("Currency-Coin_{}.NIF", k + 1),
        };
        format!("{FOLDER}{name}")
    }

    /// Every model the menu loads, in the order the game attaches them.
    pub fn all() -> Vec<Part> {
        let mut v = vec![Part::Table, Part::Available, Part::Deck];
        for t in 0..3 {
            for r in 0..7 {
                v.push(Part::Row { track: t, row: r });
                v.push(Part::Row {
                    track: t + 3,
                    row: r,
                });
            }
        }
        v.extend([
            Part::Cursor { valid: true },
            Part::Cursor { valid: false },
            Part::Hand { npc: false },
            Part::Hand { npc: true },
        ]);
        v.extend((0..3).map(Part::Bill));
        v.extend((0..3).map(Part::Coin));
        v
    }
}

/// A chain of nodes from a model's top down to one of its objects, each
/// with its own transform (the object's last).
pub type Chain = Vec<(String, Transform)>;

/// What a model's file gives.
#[derive(Debug, Clone, Default)]
pub struct ModelData {
    /// Each shape's chain, by its name (lower case).
    pub chains: HashMap<String, Chain>,
    /// Every node's own transform, by name (lower case).
    pub nodes: HashMap<String, Transform>,
    /// Its top node's name (lower case).
    pub top: String,
    pub sequences: Arc<Vec<nif::Sequence>>,
    /// Each shape's bounding sphere's centre in the model's space (lower
    /// case names).
    pub centers: HashMap<String, Vec3>,
}

impl ModelData {
    /// Reads a model's chains, transforms, sequences and centres.
    pub fn read(nif: &nif::Nif) -> ModelData {
        let mut m = ModelData {
            sequences: Arc::new(nif.sequences().unwrap_or_default()),
            ..ModelData::default()
        };
        fn walk(nif: &nif::Nif, index: i32, chain: &mut Chain, m: &mut ModelData) {
            let Ok(index) = usize::try_from(index) else {
                return;
            };
            match nif.block(index) {
                Ok(nif::Block::Node(node)) => {
                    let name = node.av.net.name.to_ascii_lowercase();
                    m.nodes.insert(name.clone(), node.av.transform);
                    if chain.is_empty() {
                        m.top = name.clone();
                    }
                    chain.push((name.clone(), node.av.transform));
                    m.chains.insert(name, chain.clone());
                    for &c in &node.children {
                        walk(nif, c, chain, m);
                    }
                    chain.pop();
                }
                Ok(nif::Block::Geometry(g)) => {
                    let name = g.av.net.name.to_ascii_lowercase();
                    m.nodes.insert(name.clone(), g.av.transform);
                    chain.push((name.clone(), g.av.transform));
                    let data = usize::try_from(g.data).ok().and_then(|d| nif.block(d).ok());
                    if let Some(nif::Block::GeometryData(d)) = data {
                        let world = chain_transform(chain, &HashMap::new());
                        m.centers.insert(name.clone(), world.apply_point(d.center));
                    }
                    m.chains.insert(name, chain.clone());
                    chain.pop();
                }
                _ => {
                    // Lights, cameras: their chains too.
                    if let Some(name) = nif.block_name(index) {
                        let name = name.to_ascii_lowercase();
                        let own = nif.av_transform(index).unwrap_or(Transform::IDENTITY);
                        chain.push((name.clone(), own));
                        m.chains.insert(name, chain.clone());
                        chain.pop();
                    }
                }
            }
        }
        for &root in nif.roots() {
            walk(nif, root, &mut Vec::new(), &mut m);
        }
        m
    }

    /// A sequence by name.
    pub fn sequence(&self, name: &str) -> Option<&nif::Sequence> {
        self.sequences
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }
}

/// A chain's transform with some nodes' own transforms replaced.
pub fn chain_transform(
    chain: &[(String, Transform)],
    pose: &HashMap<String, Transform>,
) -> Transform {
    chain
        .iter()
        .fold(Transform::IDENTITY, |world, (name, own)| {
            world.then_child(pose.get(name).unwrap_or(own))
        })
}

/// A model's pose now: nodes moved by its sequences (or by the menu's
/// code), the sequences running with the time each started at, the nodes
/// hidden.
#[derive(Debug, Clone, Default)]
pub struct Pose {
    pub nodes: HashMap<String, Transform>,
    active: Vec<(String, Option<f32>)>,
    pub culled: HashSet<String>,
}

impl Pose {
    pub fn activate(&mut self, sequence: &str) {
        if !self
            .active
            .iter()
            .any(|(s, _)| s.eq_ignore_ascii_case(sequence))
        {
            self.active.push((sequence.to_string(), None));
        }
    }

    pub fn deactivate(&mut self, sequence: &str) {
        self.active
            .retain(|(s, _)| !s.eq_ignore_ascii_case(sequence));
    }

    pub fn deactivate_all(&mut self) {
        self.active.clear();
    }

    /// The running sequences pose the model at `time` (each from its first
    /// update; clamped at its end, or round again when it loops).
    pub fn update(&mut self, model: &ModelData, time: f32) {
        for (name, offset) in &mut self.active {
            let Some(seq) = model.sequence(name) else {
                continue;
            };
            let start = *offset.get_or_insert(time);
            let length = (seq.stop - seq.start).max(0.0);
            let local = (time - start).max(0.0);
            let local = if seq.looping && length > 0.0 {
                local % length
            } else {
                local.min(length)
            };
            for track in &seq.tracks {
                let node = track.node.to_ascii_lowercase();
                let Some(base) = self.nodes.get(&node).or_else(|| model.nodes.get(&node)) else {
                    continue;
                };
                let posed = track.sample(seq.start + local).apply(base);
                self.nodes.insert(node, posed);
            }
        }
    }

    /// Posed at a sequence's first frame (activate, update at 0,
    /// deactivate).
    pub fn reset(&mut self, model: &ModelData, sequence: &str) {
        let running = std::mem::take(&mut self.active);
        self.active.push((sequence.to_string(), None));
        self.update(model, 0.0);
        self.active = running;
    }

    /// A node's own translation set (the menu's code moving markers, a
    /// track or a hand card).
    pub fn set_translation(&mut self, model: &ModelData, node: &str, t: Vec3) {
        let node = node.to_ascii_lowercase();
        let mut own = self
            .nodes
            .get(&node)
            .or_else(|| model.nodes.get(&node))
            .copied()
            .unwrap_or(Transform::IDENTITY);
        own.translation = t;
        self.nodes.insert(node, own);
    }

    /// A node's own translation now.
    pub fn translation(&self, model: &ModelData, node: &str) -> Vec3 {
        let node = node.to_ascii_lowercase();
        self.nodes
            .get(&node)
            .or_else(|| model.nodes.get(&node))
            .map_or([0.0; 3], |t| t.translation)
    }

    /// Where a shape is now against where it was loaded (its vertices are
    /// in the model's space as loaded); `None` for one hidden or unknown.
    pub fn shape_move(&self, model: &ModelData, shape: &str) -> Option<Transform> {
        let chain = model.chains.get(&shape.to_ascii_lowercase())?;
        if chain.iter().any(|(n, _)| self.culled.contains(n)) {
            return None;
        }
        let rest = chain_transform(chain, &HashMap::new());
        Some(chain_transform(chain, &self.nodes).then_child(&rest.inverse()))
    }

    /// An object's place in the model's space now.
    pub fn world(&self, model: &ModelData, object: &str) -> Option<Transform> {
        let chain = model.chains.get(&object.to_ascii_lowercase())?;
        Some(chain_transform(chain, &self.nodes))
    }
}

/// A place on a grid: the shape's own turn and its bound's centre (a
/// marker there is turned that × [`quarter_turn`], `0074e6d0`; a bill or
/// coin just that, `0074a2c0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spot {
    pub rotation: [[f32; 3]; 3],
    pub center: Vec3,
}

/// Z(π/2) as the game builds it (`004a0c90`, single-precision π/2 at
/// `0101ff38`): rows (cos, sin, 0), (−sin, cos, 0), (0, 0, 1).
pub fn quarter_turn() -> [[f32; 3]; 3] {
    let (s, c) = std::f32::consts::FRAC_PI_2.sin_cos();
    [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]]
}

/// Everything the menu shows, ready to draw.
pub struct CaravanModels {
    /// Every drawn model's pieces (each draw's `reference` is its index in
    /// [`CaravanModels::parts`] + 1).
    pub scene: ViewerScene,
    pub parts: Vec<Part>,
    pub models: Vec<ModelData>,
    /// The camera's frustum as the file has it.
    pub frustum: Option<nif::camera::Frustum>,
    /// The table's point lights: their names (lower case) and what they
    /// give; the radius they all get.
    pub lights: Vec<(String, nif::Light)>,
    pub light_radius: f32,
    /// The grids' slots, by track, then by `Select{r}_0{c}:0` (row and
    /// column from 0).
    pub grids: Vec<HashMap<(usize, usize), Spot>>,
    /// The money's spots (`Bill-Placement_0{n}:0`, `Coin-Placement_…`).
    pub bill_spots: Vec<Spot>,
    pub coin_spots: Vec<Spot>,
}

impl CaravanModels {
    pub fn model(&self, part: Part) -> Option<&ModelData> {
        self.parts
            .iter()
            .position(|&p| p == part)
            .and_then(|i| self.models.get(i))
    }

    /// The camera's place (the table's `object0`) in the table's pose.
    pub fn camera(&self, table: &Pose) -> Option<Transform> {
        table.world(self.model(Part::Table)?, "object0")
    }

    /// The lights where the table's pose has them.
    pub fn lights_now(&self, table: &Pose) -> Vec<LightData> {
        let Some(model) = self.model(Part::Table) else {
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

/// A card's texture (`Textures\` and its `TX00` or `TX01`).
pub fn card_texture(assets: &assets::Assets, path: &str) -> Option<TextureData> {
    let path = format!("textures\\{}", path.trim_start_matches(['\\', '/']));
    let (real, bytes) = crate::read_texture(assets, &path)?;
    TextureData::from_dds(real, bytes).ok()
}

/// The slots of a grid model.
fn spots(
    model: &ModelData,
    name: impl Fn(&str) -> Option<(usize, usize)>,
) -> HashMap<(usize, usize), Spot> {
    let mut out = HashMap::new();
    for (shape, center) in &model.centers {
        let Some(key) = name(shape) else {
            continue;
        };
        let own = model
            .nodes
            .get(shape)
            .copied()
            .unwrap_or(Transform::IDENTITY);
        out.insert(
            key,
            Spot {
                rotation: own.rotation,
                center: *center,
            },
        );
    }
    out
}

/// Loads every model the menu uses; `None` when the table can't be read.
pub fn load(assets: &assets::Assets) -> Option<CaravanModels> {
    let read = |path: &str| -> Option<nif::Nif> { nif::Nif::parse(assets.read(path).ok()??).ok() };
    let table = read(&Part::Table.file())?;
    // The node's bound with only the table in it, as loaded.
    let bound = table.roots().iter().fold(None, |acc, &r| {
        crate::lockpick::merge(
            acc,
            crate::lockpick::bound(&table, r, &Transform::IDENTITY, None),
        )
    });
    let light_radius = LIGHT_RADIUS_MULT * bound.map_or(0.0, |(_, r)| r);
    let lights: Vec<(String, nif::Light)> = table
        .lights()
        .unwrap_or_default()
        .into_iter()
        .filter(|l| l.kind == nif::LightKind::Point)
        .map(|l| (l.name.to_ascii_lowercase(), l))
        .collect();
    let frustum = (0..table.blocks().len())
        .find(|&i| {
            table.block_type(i) == "NiCamera"
                && table
                    .block_name(i)
                    .is_some_and(|n| n.eq_ignore_ascii_case("object0"))
        })
        .and_then(|i| table.camera_frustum(i).ok().flatten())
        .map(|(f, _)| f);
    let parts = Part::all();
    let mut models = Vec::new();
    let mut cell = world::LoadedCell::actors_only(Vec::new());
    for (i, part) in parts.iter().enumerate() {
        let file = part.file();
        let data = read(&file).map(|n| ModelData::read(&n)).unwrap_or_default();
        models.push(data);
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
            model: Some(file),
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
    // The grids: `Select{r}_0{c}:0`.
    let grids = (0..6)
        .map(|t| {
            let file = if t < 3 {
                format!("{FOLDER}PlayerGrid_{}.NIF", t + 1)
            } else {
                format!("{FOLDER}OppGrid_{}.NIF", t - 2)
            };
            read(&file).map_or_else(HashMap::new, |n| {
                spots(&ModelData::read(&n), |shape| {
                    let rest = shape.strip_prefix("select")?.strip_suffix(":0")?;
                    let (r, c) = rest.split_once("_0")?;
                    Some((r.parse::<usize>().ok()? - 1, c.parse::<usize>().ok()? - 1))
                })
            })
        })
        .collect();
    let money_spots = |file: &str, prefix: &str| -> Vec<Spot> {
        let Some(n) = read(&format!("{FOLDER}{file}")) else {
            return Vec::new();
        };
        let found = spots(&ModelData::read(&n), |shape| {
            let rest = shape.strip_prefix(prefix)?.strip_suffix(":0")?;
            Some((rest.trim_start_matches('0').parse::<usize>().ok()? - 1, 0))
        });
        let mut v: Vec<((usize, usize), Spot)> = found.into_iter().collect();
        v.sort_by_key(|(k, _)| *k);
        v.into_iter().map(|(_, s)| s).collect()
    };
    let bill_spots = money_spots("Currency-Bill_Grid.NIF", "bill-placement_");
    let coin_spots = money_spots("Currency-Coin_Grid.NIF", "coin-placement_");
    Some(CaravanModels {
        scene,
        parts,
        models,
        frustum,
        lights,
        light_radius,
        grids,
        bill_spots,
        coin_spots,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files() {
        assert_eq!(
            Part::Row { track: 0, row: 0 }.file(),
            "Meshes\\Terminals\\NV_Caravan\\Player_1_01.NIF"
        );
        assert_eq!(
            Part::Row { track: 5, row: 6 }.file(),
            "Meshes\\Terminals\\NV_Caravan\\Opponent_3_07.NIF"
        );
        assert_eq!(Part::all().len(), 3 + 42 + 4 + 6);
    }

    #[test]
    fn poses_stay() {
        let mut m = ModelData::default();
        m.nodes.insert("a".into(), Transform::IDENTITY);
        let mut pose = Pose::default();
        pose.set_translation(&m, "A", [0.0, 0.0, 3.0]);
        assert_eq!(pose.translation(&m, "a"), [0.0, 0.0, 3.0]);
        // A sequence that doesn't exist moves nothing.
        pose.activate("Missing");
        pose.update(&m, 1.0);
        assert_eq!(pose.translation(&m, "a"), [0.0, 0.0, 3.0]);
    }
}
