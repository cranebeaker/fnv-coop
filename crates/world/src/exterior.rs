//! Outdoor worldspaces (`WRLD`): their settings, the grid of exterior cells,
//! and loading one square of the grid with everything placed in it.
//!
//! A worldspace's cells sit in its child groups, one `CELL` per grid square
//! (`XCLC` gives the square). Objects the game keeps loaded everywhere
//! (doors, map markers, many people) are "persistent": they sit together in
//! one extra cell of the worldspace, flagged persistent, rather than in the
//! square they stand in. Loading a square brings in those standing in it.

use std::collections::HashMap;

use esm::{flags, sig, FormId, FourCC, LoadOrder, RecordRef};

use crate::cell::{le_f32, le_i32, le_u32};
use crate::image_space::ImageSpace;
use crate::land::{Land, CELL_SIZE};
use crate::placement::{load_cell_with, Disabled, LoadedCell};
use crate::weather::{light_exterior, Climate};
use crate::{Error, Result};

const XCLC: FourCC = FourCC::new(b"XCLC");
const CNAM: FourCC = FourCC::new(b"CNAM");
const NAM2: FourCC = FourCC::new(b"NAM2");
const DNAM: FourCC = FourCC::new(b"DNAM");
const INAM: FourCC = FourCC::new(b"INAM");
const WNAM: FourCC = FourCC::new(b"WNAM");
const PNAM: FourCC = FourCC::new(b"PNAM");
const WRLD: FourCC = FourCC::new(b"WRLD");

/// A worldspace's settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Worldspace {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    /// The worldspace this one borrows from (`WNAM`), and what it borrows
    /// (`PNAM` flags).
    pub parent: Option<(FormId, u16)>,
    /// `CNAM`: the climate (weathers, sunrise and sunset).
    pub climate: Option<FormId>,
    /// `NAM2`: the water.
    pub water: Option<FormId>,
    /// `INAM`: the image space.
    pub image_space: Option<FormId>,
    /// `DNAM`: the height of land and water where a square has none.
    pub default_land_height: f32,
    pub default_water_height: f32,
    /// `DATA` flags.
    pub flags: u8,
}

/// Where a worldspace's cells are.
#[derive(Debug, Clone)]
pub struct WorldGrid {
    pub world: Worldspace,
    /// Exterior cells by grid square (x east, y north).
    pub cells: HashMap<(i32, i32), FormId>,
    /// The cell holding the worldspace's persistent objects.
    pub persistent: Option<FormId>,
    /// Persistent objects by the grid square they stand in.
    persistent_by_square: HashMap<(i32, i32), Vec<FormId>>,
    /// The worldspace's climate.
    pub climate: Option<Climate>,
    /// The worldspace's image space.
    pub image_space: Option<ImageSpace>,
}

impl Worldspace {
    pub fn load(order: &LoadOrder, id: FormId) -> Result<Worldspace> {
        let rr = order.get(id).ok_or(Error::NoSuchRecord(id))?;
        if rr.entry.header.kind != WRLD {
            return Err(Error::NotA {
                form_id: id,
                kind: rr.entry.header.kind,
                wanted: "worldspace",
            });
        }
        let record = rr.record()?;
        let form = |kind: FourCC| {
            record
                .get(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                .filter(|id| id.0 != 0)
        };
        let dnam = record.get(DNAM).filter(|s| s.data.len() >= 8);
        Ok(Worldspace {
            form_id: id,
            editor_id: record.editor_id(),
            name: record.full_name(),
            parent: form(WNAM).map(|p| {
                let flags = record
                    .get(PNAM)
                    .filter(|s| s.data.len() >= 2)
                    .map_or(0, |s| u16::from_le_bytes([s.data[0], s.data[1]]));
                (p, flags)
            }),
            climate: form(CNAM),
            water: form(NAM2),
            image_space: form(INAM),
            default_land_height: dnam.map_or(0.0, |s| le_f32(&s.data, 0)),
            default_water_height: dnam.map_or(0.0, |s| le_f32(&s.data, 4)),
            flags: record
                .get(sig::DATA)
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
        })
    }

    /// Editor ID, else name, else form ID.
    pub fn label(&self) -> String {
        self.editor_id
            .clone()
            .or_else(|| self.name.clone())
            .unwrap_or_else(|| self.form_id.to_string())
    }
}

/// Every worldspace, by form ID.
pub fn worldspaces(order: &LoadOrder) -> Vec<Worldspace> {
    let mut out: Vec<Worldspace> = order
        .records_of_type(WRLD)
        .filter_map(|rr| Worldspace::load(order, rr.form_id).ok())
        .collect();
    out.sort_by_key(|w| w.form_id);
    out
}

/// A worldspace by form ID (hex), editor ID or name (case-insensitive).
pub fn find_worldspace(order: &LoadOrder, query: &str) -> Result<Option<FormId>> {
    let query = query.trim();
    if let Some(id) = FormId::parse_hex(query) {
        if order.get(id).is_some_and(|r| r.entry.header.kind == WRLD) {
            return Ok(Some(id));
        }
    }
    if let Some(rr) = order.find_by_editor_id(query, Some(WRLD))? {
        return Ok(Some(rr.form_id));
    }
    let wanted = query.to_lowercase();
    for rr in order.records_of_type(WRLD) {
        if rr.record()?.full_name().map(|n| n.to_lowercase()) == Some(wanted.clone()) {
            return Ok(Some(rr.form_id));
        }
    }
    Ok(None)
}

/// The grid square a position falls in.
pub fn square_of(position: [f32; 3]) -> (i32, i32) {
    (
        (position[0] / CELL_SIZE).floor() as i32,
        (position[1] / CELL_SIZE).floor() as i32,
    )
}

impl WorldGrid {
    /// Finds a worldspace's cells and sorts its persistent objects into
    /// grid squares.
    pub fn load(order: &LoadOrder, world: FormId) -> Result<WorldGrid> {
        let world = Worldspace::load(order, world)?;
        let mut cells = HashMap::new();
        let mut persistent = None;
        for rr in order.records_of_type(sig::CELL) {
            if order.world_of(&rr) != Some(world.form_id) || rr.entry.header.is_deleted() {
                continue;
            }
            if rr.entry.header.flags & flags::PERSISTENT != 0 {
                persistent = Some(rr.form_id);
                continue;
            }
            let record = rr.record()?;
            if let Some(s) = record.get(XCLC).filter(|s| s.data.len() >= 8) {
                cells.insert((le_i32(&s.data, 0), le_i32(&s.data, 4)), rr.form_id);
            }
        }
        let mut persistent_by_square: HashMap<(i32, i32), Vec<FormId>> = HashMap::new();
        if let Some(p) = persistent {
            for rr in order.references_in_cell(p) {
                if let Some(position) = position_of(&rr) {
                    persistent_by_square
                        .entry(square_of(position))
                        .or_default()
                        .push(rr.form_id);
                }
            }
        }
        let climate = world.climate.and_then(|c| Climate::load(order, c));
        let image_space = world.image_space.and_then(|i| ImageSpace::load(order, i));
        Ok(WorldGrid {
            world,
            cells,
            persistent,
            persistent_by_square,
            climate,
            image_space,
        })
    }

    /// The persistent objects standing in a grid square.
    pub fn persistent_in(&self, square: (i32, i32)) -> &[FormId] {
        self.persistent_by_square
            .get(&square)
            .map_or(&[], |v| v.as_slice())
    }

    /// The cell at a grid square, if the worldspace has one there.
    pub fn cell_at(&self, square: (i32, i32)) -> Option<FormId> {
        self.cells.get(&square).copied()
    }

    /// Loads one grid square: its cell's objects plus the persistent ones
    /// standing in it. `None` when the worldspace has no cell there.
    pub fn load_square(&self, order: &LoadOrder, square: (i32, i32)) -> Result<Option<LoadedCell>> {
        self.load_square_now(order, square, &Disabled::new())
    }

    /// [`Self::load_square`] with what scripts have enabled and disabled.
    pub fn load_square_now(
        &self,
        order: &LoadOrder,
        square: (i32, i32),
        disabled: &Disabled,
    ) -> Result<Option<LoadedCell>> {
        let Some(cell) = self.cell_at(square) else {
            return Ok(None);
        };
        let extra: Vec<RecordRef<'_>> = self
            .persistent_by_square
            .get(&square)
            .into_iter()
            .flatten()
            .filter_map(|&id| order.get(id))
            .collect();
        let mut loaded = load_cell_with(order, cell, extra, disabled)?;
        light_exterior(order, &mut loaded.info, self.climate.as_ref());
        if loaded.info.image_space.is_none() {
            loaded.info.image_space = self.image_space.clone();
        }
        Ok(Some(loaded))
    }

    /// A grid square's terrain.
    pub fn land(&self, order: &LoadOrder, square: (i32, i32)) -> Result<Option<Land>> {
        match self.cell_at(square) {
            Some(cell) => Land::of_cell(order, cell),
            None => Ok(None),
        }
    }
}

/// A placed object's position, read from its `DATA`.
fn position_of(rr: &RecordRef<'_>) -> Option<[f32; 3]> {
    let record = rr.record().ok()?;
    let data = record.get(sig::DATA).filter(|s| s.data.len() >= 12)?;
    Some([
        le_f32(&data.data, 0),
        le_f32(&data.data, 4),
        le_f32(&data.data, 8),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_round_down_toward_the_south_west() {
        assert_eq!(square_of([0.0, 0.0, 0.0]), (0, 0));
        assert_eq!(square_of([4095.0, 4096.0, 0.0]), (0, 1));
        assert_eq!(square_of([-73349.5, 1296.9, 8741.4]), (-18, 0));
        assert_eq!(square_of([-0.5, -4096.0, 0.0]), (-1, -1));
    }
}
