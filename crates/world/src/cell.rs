//! Cell records: finding them, and reading their flags and lighting.

use esm::{sig, FormId, FourCC, LoadOrder, Record, RecordRef};

use crate::image_space::{cell_image_space, ImageSpace};
use crate::{Error, Result};

const XCLC: FourCC = FourCC::new(b"XCLC");
const XCLL: FourCC = FourCC::new(b"XCLL");
const LTMP: FourCC = FourCC::new(b"LTMP");
const LNAM: FourCC = FourCC::new(b"LNAM");
const XCLW: FourCC = FourCC::new(b"XCLW");
const LGTM: FourCC = FourCC::new(b"LGTM");

/// Cell `DATA` flag: an interior cell.
pub const CELL_INTERIOR: u8 = 0x01;
/// Cell `DATA` flag: the cell has water.
pub const CELL_HAS_WATER: u8 = 0x02;

/// A cell's lighting (`XCLL`, or the `DATA` of a lighting template).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lighting {
    pub ambient: [u8; 3],
    pub directional: [u8; 3],
    pub fog_color: [u8; 3],
    pub fog_near: f32,
    pub fog_far: f32,
    /// Direction of the directional light, in degrees.
    pub directional_rotation_xy: i32,
    pub directional_rotation_z: i32,
    pub directional_fade: f32,
    pub fog_clip: f32,
    pub fog_power: f32,
}

impl Lighting {
    /// Reads the 40-byte lighting structure. The last field is missing in
    /// some older records and defaults to 1.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 36 {
            return None;
        }
        let rgb = |at: usize| [data[at], data[at + 1], data[at + 2]];
        let f = |at: usize| f32::from_le_bytes(data[at..at + 4].try_into().unwrap());
        let i = |at: usize| i32::from_le_bytes(data[at..at + 4].try_into().unwrap());
        Some(Self {
            ambient: rgb(0),
            directional: rgb(4),
            fog_color: rgb(8),
            fog_near: f(12),
            fog_far: f(16),
            directional_rotation_xy: i(20),
            directional_rotation_z: i(24),
            directional_fade: f(28),
            fog_clip: f(32),
            fog_power: if data.len() >= 40 { f(36) } else { 1.0 },
        })
    }

    /// Unit vector from a surface toward the directional light, in game
    /// space. The two angles give the direction the light shines along,
    /// read from apitrace recordings of the game's shader constants:
    /// Doc Mitchell's house has 270° up (35° around) and the game sends its
    /// shaders (0, 0, 1) toward the light, so it shines straight down; the
    /// Mojave Outpost barracks has 0° and 0°, and the game sends (−1, 0, 0)
    /// on an unrotated floor piece, so the around angle starts from east
    /// (+x), not north. A positive around angle turns it **clockwise** seen
    /// from above (the game's code, `00641830`: rotation Rz(around) ·
    /// Ry(up), toward the light `(−cos a cos u, sin a cos u, −sin u)`).
    pub fn toward_directional(&self) -> [f32; 3] {
        let around = (self.directional_rotation_xy as f32).to_radians();
        let up = (self.directional_rotation_z as f32).to_radians();
        let shines = [around.cos() * up.cos(), -around.sin() * up.cos(), up.sin()];
        let len = shines.iter().map(|c| c * c).sum::<f32>().sqrt();
        if len < 1e-6 {
            return [0.0, 0.0, 1.0];
        }
        shines.map(|c| -c / len)
    }

    /// Takes the fields selected by a cell's `LNAM` inheritance flags from
    /// its lighting template.
    pub fn inherit(&mut self, template: &Lighting, flags: u32) {
        let from = |bit: u32| flags & (1 << bit) != 0;
        if from(0) {
            self.ambient = template.ambient;
        }
        if from(1) {
            self.directional = template.directional;
        }
        if from(2) {
            self.fog_color = template.fog_color;
        }
        if from(3) {
            self.fog_near = template.fog_near;
        }
        if from(4) {
            self.fog_far = template.fog_far;
        }
        if from(5) {
            self.directional_rotation_xy = template.directional_rotation_xy;
            self.directional_rotation_z = template.directional_rotation_z;
        }
        if from(6) {
            self.directional_fade = template.directional_fade;
        }
        if from(7) {
            self.fog_clip = template.fog_clip;
        }
        if from(8) {
            self.fog_power = template.fog_power;
        }
    }
}

/// Everything read from a cell record.
#[derive(Debug, Clone, PartialEq)]
pub struct CellInfo {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    /// The plugin whose version of the cell wins.
    pub plugin: String,
    /// `DATA` flags.
    pub flags: u8,
    pub interior: bool,
    /// Grid position of an exterior cell.
    pub grid: Option<(i32, i32)>,
    /// The worldspace of an exterior cell.
    pub world: Option<FormId>,
    /// Lighting after applying the lighting template, if any.
    pub lighting: Option<Lighting>,
    /// Where the lighting came from, for display.
    pub lighting_source: String,
    pub water_height: Option<f32>,
    /// The image space the cell names (`XCIM`), if any.
    pub image_space: Option<ImageSpace>,
    /// Outdoors: the weather's sky colours by day, as stored: upper sky,
    /// horizon, lower sky (see `crate::weather`).
    pub sky: Option<[[u8; 3]; 3]>,
    /// Outdoors: the weather that lights it (its clouds too).
    pub weather: Option<FormId>,
}

impl CellInfo {
    /// Editor ID, else display name, else form ID.
    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .or_else(|| self.name.clone())
            .unwrap_or_else(|| self.form_id.to_string())
    }
}

/// Reads the winning version of a cell.
pub fn cell_info(order: &LoadOrder, form_id: FormId) -> Result<CellInfo> {
    let rr = order.get(form_id).ok_or(Error::NoSuchRecord(form_id))?;
    if rr.entry.header.kind != sig::CELL {
        return Err(Error::NotACell {
            form_id,
            kind: rr.entry.header.kind,
        });
    }
    let record = rr.record()?;
    let flags = record.get(sig::DATA).and_then(|d| d.data.first().copied());
    // Cells in the CELL top group are interiors even if DATA is missing.
    let interior = flags.map_or(rr.entry.top_group == sig::CELL, |f| f & CELL_INTERIOR != 0);
    let grid = record
        .get(XCLC)
        .filter(|s| s.data.len() >= 8)
        .map(|s| (le_i32(&s.data, 0), le_i32(&s.data, 4)));

    let own = record.get(XCLL).and_then(|s| Lighting::parse(&s.data));
    let template = record
        .get(LTMP)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .filter(|id| id.0 != 0);
    let inherit = record
        .get(LNAM)
        .filter(|s| s.data.len() >= 4)
        .map(|s| le_u32(&s.data, 0));
    let template_lighting = template.and_then(|id| template_lighting(order, id));

    let (lighting, lighting_source) = match (own, template_lighting) {
        (Some(mut own), Some((t, name))) if inherit.unwrap_or(0) != 0 => {
            own.inherit(&t, inherit.unwrap_or(0));
            (Some(own), format!("cell, partly from template {name}"))
        }
        (Some(own), _) => (Some(own), "cell".to_string()),
        (None, Some((t, name))) => (Some(t), format!("template {name}")),
        (None, None) => (None, "none (engine defaults)".to_string()),
    };

    Ok(CellInfo {
        form_id,
        editor_id: record.editor_id(),
        name: record.full_name(),
        plugin: rr.plugin.name.clone(),
        flags: flags.unwrap_or(0),
        interior,
        grid,
        world: order.world_of(&rr),
        lighting,
        lighting_source,
        water_height: record
            .get(XCLW)
            .filter(|s| s.data.len() >= 4)
            .map(|s| le_f32(&s.data, 0)),
        image_space: cell_image_space(order, rr.plugin, &record),
        sky: None,
        weather: None,
    })
}

fn template_lighting(order: &LoadOrder, id: FormId) -> Option<(Lighting, String)> {
    let rr = order.get(id)?;
    if rr.entry.header.kind != LGTM {
        return None;
    }
    let record = rr.record().ok()?;
    let lighting = Lighting::parse(&record.get(sig::DATA)?.data)?;
    let name = record.editor_id().unwrap_or_else(|| id.to_string());
    Some((lighting, name))
}

/// One line of a cell listing.
#[derive(Debug, Clone, PartialEq)]
pub struct CellSummary {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    pub plugin: String,
    /// Placed objects, actors included.
    pub references: usize,
}

/// Every interior cell, by form ID.
pub fn interior_cells(order: &LoadOrder) -> Result<Vec<CellSummary>> {
    let counts = order.reference_counts();
    let mut out = Vec::new();
    for rr in order.records_of_type(sig::CELL) {
        if rr.entry.top_group != sig::CELL || rr.entry.header.is_deleted() {
            continue;
        }
        let record = rr.record()?;
        out.push(CellSummary {
            form_id: rr.form_id,
            editor_id: record.editor_id(),
            name: record.full_name(),
            plugin: rr.plugin.name.clone(),
            references: counts.get(&rr.form_id).copied().unwrap_or(0),
        });
    }
    out.sort_by_key(|c| c.form_id);
    Ok(out)
}

/// Finds cells by form ID (hex), editor ID or display name, case-insensitive.
/// An exact match wins; otherwise every interior cell whose editor ID or
/// name contains the text is returned.
pub fn find_cells(order: &LoadOrder, query: &str) -> Result<Vec<FormId>> {
    let query = query.trim();
    if let Some(id) = FormId::parse_hex(query) {
        if order
            .get(id)
            .is_some_and(|r| r.entry.header.kind == sig::CELL)
        {
            return Ok(vec![id]);
        }
    }
    if let Some(rr) = order.find_by_editor_id(query, Some(sig::CELL))? {
        return Ok(vec![rr.form_id]);
    }

    let wanted = query.to_lowercase();
    let mut exact = Vec::new();
    let mut partial = Vec::new();
    for rr in order.records_of_type(sig::CELL) {
        if rr.entry.header.is_deleted() {
            continue;
        }
        let record = rr.record()?;
        let name = record.full_name().map(|n| n.to_lowercase());
        let editor_id = record.editor_id().map(|n| n.to_lowercase());
        if name.as_deref() == Some(wanted.as_str()) {
            exact.push(rr.form_id);
        } else if rr.entry.top_group == sig::CELL
            && [name, editor_id]
                .iter()
                .flatten()
                .any(|text| text.contains(&wanted))
        {
            partial.push(rr.form_id);
        }
    }
    Ok(if exact.is_empty() { partial } else { exact })
}

/// Editor ID and display name of any record, for messages.
pub fn describe_record(rr: &RecordRef<'_>) -> String {
    let record: Option<Record> = rr.record().ok();
    let editor_id = record.as_ref().and_then(Record::editor_id);
    let name = record.as_ref().and_then(Record::full_name);
    match (editor_id, name) {
        (Some(e), Some(n)) => format!("{} {e} \"{n}\"", rr.form_id),
        (Some(e), None) => format!("{} {e}", rr.form_id),
        (None, Some(n)) => format!("{} \"{n}\"", rr.form_id),
        (None, None) => rr.form_id.to_string(),
    }
}

pub(crate) fn le_u32(data: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(data[at..at + 4].try_into().unwrap())
}

pub(crate) fn le_i32(data: &[u8], at: usize) -> i32 {
    i32::from_le_bytes(data[at..at + 4].try_into().unwrap())
}

pub(crate) fn le_f32(data: &[u8], at: usize) -> f32 {
    f32::from_le_bytes(data[at..at + 4].try_into().unwrap())
}
