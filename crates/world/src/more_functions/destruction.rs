//! Destructible objects, read from the game's code: a base's destruction
//! data (`DEST` and its stages, loaded by `004781e0`), `DamageObject`
//! (`005d4b90` → vtable +0x144 → `00579220` → `00475b20`),
//! `GetDestructionStage` (`005a4c60` → `00477430`) and `ClearDestruction`
//! (`005da260` → `00477d10`).
//!
//! The record: `DEST` (8 bytes: health i32, stage count u8, flags u8), then
//! per stage `DSTD` (20 bytes: health % u8, its number u8, the model's
//! damage stage u8, flags u8 (0x01 cap damage, 0x02 disable, 0x04
//! destroy), self damage per second i32, explosion, debris, debris count
//! i32), maybe `DMDL` (a replacement model), and `DSTF` closing it. The
//! object's health lives in its extra data (0x56, a float): none until
//! first damaged.
//!
//! What `DamageObject n` does to a placed object with destruction data
//! (n > 0; not disabled or deleted; not destroyed already): its health
//! (else the data's full health) goes down by n, at least to 0. Its health
//! as a percent of the full, rounded up, is compared before and after:
//! every stage whose percent lies below the old one and at or above the new
//! one is reached, in order; a stage with "cap damage" stops the damage at
//! its percent. The last stage reached gives the model (its own
//! replacement, else its damage stage) and the self damage; each reached
//! stage throws its explosion and debris; "disable" removes the object,
//! "destroy" (or no health left) marks it destroyed. People and creatures
//! take another path (their health is an actor value): not here.

use std::collections::BTreeMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::scripting::{Event, GameState, Runner};

const DEST: FourCC = FourCC::new(b"DEST");
const DSTD: FourCC = FourCC::new(b"DSTD");
const DMDL: FourCC = FourCC::new(b"DMDL");
const DSTF: FourCC = FourCC::new(b"DSTF");

/// Stage flags (`DSTD` byte 3).
pub const CAP_DAMAGE: u8 = 0x01;
pub const DISABLE: u8 = 0x02;
pub const DESTROY: u8 = 0x04;

/// One stage.
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub health_percent: u8,
    /// The model's damage stage (`DSTD` byte 2; at the stage's +0).
    pub damage_stage: u8,
    pub flags: u8,
    pub self_damage: i32,
    pub explosion: Option<FormId>,
    pub debris: Option<FormId>,
    pub debris_count: i32,
    pub model: Option<String>,
}

/// A base's destruction data.
#[derive(Debug, Clone, PartialEq)]
pub struct Destructible {
    pub health: i32,
    pub flags: u8,
    pub stages: Vec<Stage>,
}

impl Destructible {
    /// The base's data: its `DEST`, the stages put in the slots their
    /// `DSTD` numbers name (`004781e0`; a number past the count is
    /// dropped).
    pub fn load(order: &LoadOrder, base: FormId) -> Option<Destructible> {
        let rr = order.get(base)?;
        let record = rr.record().ok()?;
        let dest = record.get(DEST).filter(|s| s.data.len() >= 8)?;
        let count = usize::from(dest.data[4]);
        let mut stages: Vec<Option<Stage>> = vec![None; count];
        let mut current: Option<usize> = None;
        let form = |d: &[u8], at: usize| {
            Some(rr.plugin.to_global(FormId(le_u32(d, at)))).filter(|f| f.0 != 0)
        };
        for sub in &record.subrecords {
            if sub.kind == DSTD && sub.data.len() >= 20 {
                let d = &sub.data;
                let slot = usize::from(d[1]);
                current = None;
                if slot < count {
                    stages[slot] = Some(Stage {
                        health_percent: d[0],
                        damage_stage: d[2],
                        flags: d[3],
                        self_damage: le_u32(d, 4) as i32,
                        explosion: form(d, 8),
                        debris: form(d, 12),
                        debris_count: le_u32(d, 16) as i32,
                        model: None,
                    });
                    current = Some(slot);
                }
            } else if sub.kind == DMDL {
                if let Some(Some(s)) = current.map(|i| &mut stages[i]) {
                    s.model = Some(sub.zstring());
                }
            } else if sub.kind == DSTF {
                current = None;
            }
        }
        Some(Destructible {
            health: le_u32(&dest.data, 0) as i32,
            flags: dest.data[5],
            stages: stages
                .into_iter()
                .map(|s| {
                    s.unwrap_or(Stage {
                        health_percent: 0,
                        damage_stage: 0,
                        flags: 0,
                        self_damage: 0,
                        explosion: None,
                        debris: None,
                        debris_count: 0,
                        model: None,
                    })
                })
                .collect(),
        })
    }
}

/// What damaged objects keep: their health (the extra data 0x56) and the
/// self damage per second their last stage started (`00844700`; stopped by
/// `00405430`: kept, not yet applied over time).
#[derive(Debug, Clone, Default)]
pub struct Damaged {
    pub health: BTreeMap<FormId, f32>,
    pub self_damage: BTreeMap<FormId, i32>,
}

/// Percent of the full, rounded up (`00476b20`), as a byte.
fn percent(health: f32, full: f32) -> u8 {
    (health * 100.0 / full).ceil().clamp(0.0, 255.0) as u8
}

fn base_data(order: &LoadOrder, state: &GameState, r: FormId) -> Option<Destructible> {
    let base = super::placed::base_now(order, state, r)?;
    Destructible::load(order, base)
}

/// `DamageObject n` on a placed object (not a person or creature: `None`).
pub fn damage(runner: &mut Runner, what: FormId, amount: f32) -> Option<()> {
    let order = runner.order;
    if crate::script_functions::is_actor(order, what) {
        return None;
    }
    let Some(data) = base_data(order, runner.state, what) else {
        return Some(());
    };
    let state = &mut *runner.state;
    if amount.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
        || !crate::placement::enabled_now(order, what, &state.disabled)
        || state.destroyed.contains(&what)
    {
        return Some(());
    }
    let full = data.health as f32;
    let before = state
        .more
        .damaged
        .health
        .get(&what)
        .copied()
        .unwrap_or(full);
    if before == 0.0 || full <= 0.0 {
        return Some(());
    }
    let old = percent(before, full);
    let mut after = (before - amount).max(0.0);
    let mut new = percent(after, full);
    if new == 0 && after > 0.0 {
        new = 1;
    }
    // Stages reached now.
    let mut reached: Vec<usize> = Vec::new();
    for (i, s) in data.stages.iter().enumerate() {
        if s.health_percent < old && new <= s.health_percent {
            reached.push(i);
            if s.flags & CAP_DAMAGE != 0 {
                after = f32::from(s.health_percent) * full / 100.0;
                break;
            }
        }
    }
    if after <= 0.0 {
        state.destroyed.insert(what);
    }
    state.more.damaged.health.insert(what, after);
    for &i in &reached {
        let s = &data.stages[i];
        if s.flags & DISABLE != 0 {
            state.disabled.insert(what, true);
            state.events.push(Event::Enable(what, false));
        }
        if s.flags & DESTROY != 0 {
            state.destroyed.insert(what);
        }
    }
    if let Some(&last) = reached.last() {
        // The last stage reached sets the self damage, which runs while
        // health is left (none: stopped).
        let self_damage = data.stages[last].self_damage;
        if self_damage == 0 || after <= 0.0 {
            state.more.damaged.self_damage.remove(&what);
        } else {
            state.more.damaged.self_damage.insert(what, self_damage);
        }
        for &i in &reached {
            let s = &data.stages[i];
            state.events.push(Event::More(super::Shown::Destruction {
                what,
                stage: i as u8 + 1,
                model: s.model.clone(),
                damage_stage: s.damage_stage,
                explosion: s.explosion,
                debris: s.debris.map(|d| (d, s.debris_count)),
            }));
        }
    }
    Some(())
}

/// `GetDestructionStage` (`00477430`): −1 for an object without
/// destruction data or never damaged (no health kept); else how many
/// stages its health has gone through: the first stage whose percent lies
/// below the health's (rounded up), or the stage count.
pub fn stage(order: &LoadOrder, state: &GameState, what: FormId) -> i32 {
    let Some(data) = base_data(order, state, what) else {
        return -1;
    };
    let Some(&health) = state.more.damaged.health.get(&what) else {
        return -1;
    };
    // (People's and creatures' health is never kept here: see [`damage`].)
    let p = percent(health, data.health as f32);
    data.stages
        .iter()
        .position(|s| s.health_percent < p)
        .map_or(data.stages.len() as i32, |i| i as i32)
}

/// `ClearDestruction` (`00477d10`): an object with destruction data and a
/// health kept goes back to whole: the health forgotten, no longer
/// destroyed, its model back (told to the viewer), its self damage
/// stopped.
pub fn clear(runner: &mut Runner, what: FormId) {
    let order = runner.order;
    if base_data(order, runner.state, what).is_none() {
        return;
    }
    let state = &mut *runner.state;
    if state.more.damaged.health.remove(&what).is_none() {
        return;
    }
    state.destroyed.remove(&what);
    state.more.damaged.self_damage.remove(&what);
    state.events.push(Event::More(super::Shown::Destruction {
        what,
        stage: 0,
        model: None,
        damage_stage: 0,
        explosion: None,
        debris: None,
    }));
}

/// Saved lines.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    for (r, h) in &state.more.damaged.health {
        line(format!("objecthealth {:08X} {h}", r.0));
    }
    for (r, d) in &state.more.damaged.self_damage {
        line(format!("selfdamage {:08X} {d}", r.0));
    }
}

/// A saved line back.
pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    let bad = || format!("can't read '{}'", parts.join(" "));
    let form = parts
        .get(1)
        .and_then(|s| u32::from_str_radix(s, 16).ok())
        .map(FormId);
    match *parts.first()? {
        "objecthealth" => Some(
            match (form, parts.get(2).and_then(|s| s.parse::<f32>().ok())) {
                (Some(r), Some(h)) => {
                    state.more.damaged.health.insert(r, h);
                    Ok(())
                }
                _ => Err(bad()),
            },
        ),
        "selfdamage" => Some(
            match (form, parts.get(2).and_then(|s| s.parse::<i32>().ok())) {
                (Some(r), Some(d)) => {
                    state.more.damaged.self_damage.insert(r, d);
                    Ok(())
                }
                _ => Err(bad()),
            },
        ),
        _ => None,
    }
}
