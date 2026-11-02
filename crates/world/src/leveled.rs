//! Leveled lists (`LVLI` items, `LVLN` people, `LVLC` creatures): what one
//! gives for the player's level.
//!
//! The record: `LVLD` (chance of nothing, percent; or `LVLG`, a global
//! holding it), `LVLF` flags (0x01 "calculate from all levels <= player's
//! level", 0x02 "calculate for each item in count", 0x04 "use all"), then
//! `LVLO` entries (level u16, 2 unused bytes, form u32, count u16, 2 unused
//! bytes), each maybe followed by `COED` (owner, condition).
//!
//! The pick, as the game's editor documents it (not traced in the game's
//! code): roll the chance of nothing; keep the entries at or below the
//! player's level, and unless flag 0x01 is set only those at the highest
//! such level; flag 0x04 gives every one, otherwise one is picked at
//! random. Lists inside lists are picked the same way. A count of more than
//! one picks once and multiplies, or with flag 0x02 picks for each.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;

const LVLD: FourCC = FourCC::new(b"LVLD");
const LVLG: FourCC = FourCC::new(b"LVLG");
const LVLF: FourCC = FourCC::new(b"LVLF");
const LVLO: FourCC = FourCC::new(b"LVLO");
const GLOB: FourCC = FourCC::new(b"GLOB");
const FLTV: FourCC = FourCC::new(b"FLTV");

pub const FROM_ALL_LEVELS: u8 = 0x01;
pub const EACH_IN_COUNT: u8 = 0x02;
pub const USE_ALL: u8 = 0x04;

/// The record types that are leveled lists.
pub fn is_leveled(kind: FourCC) -> bool {
    [b"LVLI", b"LVLN", b"LVLC"]
        .iter()
        .any(|k| kind.as_bytes() == *k)
}

/// One entry of a list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry {
    pub level: u16,
    pub form: FormId,
    pub count: u16,
}

/// A leveled list's record.
#[derive(Debug, Clone, PartialEq)]
pub struct Leveled {
    pub chance_none: f32,
    pub flags: u8,
    pub entries: Vec<Entry>,
}

impl Leveled {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Leveled> {
        let rr = order.get(id).filter(|r| is_leveled(r.entry.header.kind))?;
        let record = rr.record_shared().ok()?;
        let global = record
            .get(LVLG)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            .filter(|g| g.0 != 0)
            .and_then(|g| {
                let gr = order.get(g).filter(|r| r.entry.header.kind == GLOB)?;
                let rec = gr.record_shared().ok()?;
                let v = rec.get(FLTV).filter(|s| s.data.len() >= 4)?;
                Some(f32::from_le_bytes(v.data[..4].try_into().ok()?))
            });
        Some(Leveled {
            chance_none: global.unwrap_or_else(|| {
                record
                    .get(LVLD)
                    .and_then(|s| s.data.first().copied())
                    .map_or(0.0, f32::from)
            }),
            flags: record
                .get(LVLF)
                .and_then(|s| s.data.first().copied())
                .unwrap_or(0),
            entries: record
                .get_all(LVLO)
                .filter(|s| s.data.len() >= 10)
                .map(|s| Entry {
                    level: u16::from_le_bytes([s.data[0], s.data[1]]),
                    form: rr.plugin.to_global(FormId(le_u32(&s.data, 4))),
                    count: u16::from_le_bytes([s.data[8], s.data[9]]).max(1),
                })
                .collect(),
        })
    }

    /// The entries that can be picked at a level.
    fn candidates(&self, level: u16) -> Vec<Entry> {
        let mut under: Vec<Entry> = self
            .entries
            .iter()
            .copied()
            .filter(|e| e.level <= level)
            .collect();
        if self.flags & FROM_ALL_LEVELS == 0 {
            let top = under.iter().map(|e| e.level).max();
            under.retain(|e| Some(e.level) == top);
        }
        under
    }
}

/// Everything a form can give, at any level: itself when it isn't a
/// leveled list, else every entry's, lists inside lists followed (each
/// once, in the lists' order).
pub fn outcomes(order: &LoadOrder, form: FormId) -> Vec<FormId> {
    let mut out = Vec::new();
    let mut seen = Vec::new();
    outcomes_into(order, form, 0, &mut seen, &mut out);
    out
}

fn outcomes_into(
    order: &LoadOrder,
    form: FormId,
    depth: u8,
    seen: &mut Vec<FormId>,
    out: &mut Vec<FormId>,
) {
    if depth > 16 || seen.contains(&form) {
        return;
    }
    seen.push(form);
    match Leveled::load(order, form) {
        Some(list) => {
            for e in &list.entries {
                outcomes_into(order, e.form, depth + 1, seen, out);
            }
        }
        None => out.push(form),
    }
}

/// What a form gives `count` of at the player's level: itself when it
/// isn't a leveled list, else what the list picks, as (form, count).
/// `roll` gives random numbers.
pub fn resolve(
    order: &LoadOrder,
    form: FormId,
    count: i32,
    level: u16,
    roll: &mut dyn FnMut() -> u64,
) -> Vec<(FormId, i32)> {
    let mut out = Vec::new();
    resolve_into(order, form, count, level, roll, 0, &mut out);
    out
}

fn resolve_into(
    order: &LoadOrder,
    form: FormId,
    count: i32,
    level: u16,
    roll: &mut dyn FnMut() -> u64,
    depth: u8,
    out: &mut Vec<(FormId, i32)>,
) {
    if count <= 0 {
        return;
    }
    let Some(list) = Leveled::load(order, form) else {
        add(out, form, count);
        return;
    };
    if depth > 16 {
        return;
    }
    // Picked once and multiplied, or once for each.
    let (picks, times) = if list.flags & EACH_IN_COUNT != 0 {
        (count, 1)
    } else {
        (1, count)
    };
    for _ in 0..picks {
        if ((roll() % 100) as f32) < list.chance_none {
            continue;
        }
        let candidates = list.candidates(level);
        if candidates.is_empty() {
            continue;
        }
        let chosen: Vec<Entry> = if list.flags & USE_ALL != 0 {
            candidates
        } else {
            vec![candidates[(roll() % candidates.len() as u64) as usize]]
        };
        for e in chosen {
            let n = i32::from(e.count) * times;
            resolve_into(order, e.form, n, level, roll, depth + 1, out);
        }
    }
}

/// What a form gives without chance: like [`resolve`] with nothing left
/// to chance (no "chance of nothing", the first entry where one is picked
/// at random), for what a person is drawn with before anything is rolled.
pub fn resolve_first(
    order: &LoadOrder,
    form: FormId,
    count: i32,
    level: u16,
) -> Vec<(FormId, i32)> {
    let mut out = Vec::new();
    first_into(order, form, count, level, 0, &mut out);
    out
}

fn first_into(
    order: &LoadOrder,
    form: FormId,
    count: i32,
    level: u16,
    depth: u8,
    out: &mut Vec<(FormId, i32)>,
) {
    if count <= 0 || depth > 16 {
        return;
    }
    let Some(list) = Leveled::load(order, form) else {
        add(out, form, count);
        return;
    };
    let candidates = list.candidates(level);
    let chosen = if list.flags & USE_ALL != 0 {
        candidates
    } else {
        candidates.into_iter().take(1).collect()
    };
    for e in chosen {
        first_into(
            order,
            e.form,
            i32::from(e.count) * count,
            level,
            depth + 1,
            out,
        );
    }
}

fn add(out: &mut Vec<(FormId, i32)>, form: FormId, n: i32) {
    match out.iter_mut().find(|(f, _)| *f == form) {
        Some((_, c)) => *c += n,
        None => out.push((form, n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(flags: u8, entries: &[(u16, u32)]) -> Leveled {
        Leveled {
            chance_none: 0.0,
            flags,
            entries: entries
                .iter()
                .map(|&(level, form)| Entry {
                    level,
                    form: FormId(form),
                    count: 1,
                })
                .collect(),
        }
    }

    #[test]
    fn only_the_highest_level_reached_unless_all_levels() {
        let l = list(0, &[(1, 1), (5, 2), (5, 3), (10, 4)]);
        let forms = |c: Vec<Entry>| c.iter().map(|e| e.form.0).collect::<Vec<_>>();
        assert_eq!(forms(l.candidates(7)), vec![2, 3]);
        assert_eq!(forms(l.candidates(1)), vec![1]);
        let all = list(FROM_ALL_LEVELS, &[(1, 1), (5, 2), (10, 4)]);
        assert_eq!(forms(all.candidates(7)), vec![1, 2]);
    }
}
