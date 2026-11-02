//! Regions (`REGN`): parts of a worldspace drawn as outlines, which
//! weathers, sounds and scripts (`IsPlayerInRegion`) go by.
//!
//! A region: `WNAM` its worldspace, then its areas, each `RPLI` (the edge
//! fall-off, u32) and `RPLD` (the outline's points, x and y f32 pairs, in
//! the worldspace's units). Checked on `vMapVictorInNovacRegion`: one area
//! of four points around Novac (x 32,347 to 50,711, y -44,569 to -24,964).

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};

const REGN: FourCC = FourCC::new(b"REGN");
const WNAM: FourCC = FourCC::new(b"WNAM");
const RPLD: FourCC = FourCC::new(b"RPLD");
const RDAT: FourCC = FourCC::new(b"RDAT");
const RDMP: FourCC = FourCC::new(b"RDMP");
const XCLR: FourCC = FourCC::new(b"XCLR");

/// A region's map name (its region data of type 4, `RDAT` then `RDMP`)
/// and that data's priority (`RDAT` byte 5).
pub fn map_name(order: &LoadOrder, region: FormId) -> Option<(String, u8)> {
    let record = order.get(region)?.record().ok()?;
    let mut current: Option<(u32, u8)> = None;
    for s in &record.subrecords {
        if s.kind == RDAT && s.data.len() >= 6 {
            current = Some((le_u32(&s.data, 0), s.data[5]));
        } else if s.kind == RDMP {
            if let Some((4, priority)) = current {
                let name = s.zstring();
                if !name.is_empty() {
                    return Some((name, priority));
                }
            }
        }
    }
    None
}

/// The name of the place at (x, y) in a worldspace, as the worldspace
/// gives it (`TESWorldSpace` vtable +0x138, `00586500`): among the regions
/// of the cell there (its `XCLR`; with no cell, every region of the
/// worldspace) the map regions whose outline holds the point, the one of
/// highest priority. `None` when none does (the game then falls back to
/// other names, `00408da0` / `00586980`, not followed).
// Translated from 00586500 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn location_name(
    order: &LoadOrder,
    world: FormId,
    cell: Option<FormId>,
    x: f32,
    y: f32,
) -> Option<String> {
    let listed: Option<Vec<FormId>> = cell.and_then(|c| {
        let rr = order.get(c)?;
        let record = rr.record().ok()?;
        let xclr = record.get(XCLR)?;
        Some(
            xclr.data
                .chunks_exact(4)
                .map(|b| rr.plugin.to_global(FormId(le_u32(b, 0))))
                .collect(),
        )
    });
    let candidates: Vec<FormId> = match listed {
        Some(list) => list,
        None => order.records_of_type(REGN).map(|rr| rr.form_id).collect(),
    };
    candidates
        .into_iter()
        .filter_map(|id| {
            let region = Region::load(order, id)?;
            if region.world != Some(world) || !region.contains(x, y) {
                return None;
            }
            map_name(order, id)
        })
        .max_by_key(|(_, priority)| *priority)
        .map(|(name, _)| name)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub form_id: FormId,
    pub world: Option<FormId>,
    /// Each area's outline.
    pub areas: Vec<Vec<[f32; 2]>>,
}

impl Region {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Region> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == REGN)?;
        let record = rr.record().ok()?;
        Some(Region {
            form_id: id,
            world: record
                .get(WNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0)))),
            areas: record
                .get_all(RPLD)
                .map(|s| {
                    s.data
                        .chunks_exact(8)
                        .map(|c| [le_f32(c, 0), le_f32(c, 4)])
                        .collect()
                })
                .collect(),
        })
    }

    /// Whether a point (x, y) is inside one of its areas (even-odd rule).
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.areas.iter().any(|area| {
            let mut inside = false;
            for (i, a) in area.iter().enumerate() {
                let b = area[(i + area.len() - 1) % area.len()];
                if (a[1] > y) != (b[1] > y) && x < (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0]
                {
                    inside = !inside;
                }
            }
            inside
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_inside_an_outline() {
        let r = Region {
            form_id: FormId(1),
            world: None,
            // An L: the square 0..20 without its top-right quarter.
            areas: vec![vec![
                [0.0, 0.0],
                [20.0, 0.0],
                [20.0, 10.0],
                [10.0, 10.0],
                [10.0, 20.0],
                [0.0, 20.0],
            ]],
        };
        assert!(r.contains(5.0, 5.0));
        assert!(r.contains(15.0, 5.0));
        assert!(r.contains(5.0, 15.0));
        assert!(!r.contains(15.0, 15.0));
        assert!(!r.contains(-1.0, 5.0));
    }
}
