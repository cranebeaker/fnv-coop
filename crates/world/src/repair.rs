//! Repairing weapons and armour, read from FalloutNV.exe 1.4.0.525 (notes
//! in `docs/REPAIR.md`):
//!
//! - The Pip-Boy's repair (`RepairMenu`): the chosen item mended with
//!   another of the player's (`00648090`): with a = the chosen item's
//!   condition / 10 and b = the other's (percent / 10), hi and lo the
//!   larger and smaller, the result in percent is 10 × (hi +
//!   `fRepairMin` + (`fRepairMax` − `fRepairMin`) × Repair / 100 + lo ×
//!   `fRepairScavengeMult`), the new condition its hundredth, at most 1
//!   (`007b57f0`, `007b5d80`). Nothing caps it by skill: the function
//!   works out `fRepairSkillBase` + (`fRepairSkillMax` − base) × Repair /
//!   100 and never uses it, and its "skill needed" (only when the result
//!   comes out no better than hi, which `fRepairMin` 0.5 rules out) is
//!   never shown.
//! - What it can be mended with (`0047bb50`): the same item, or one in the
//!   chosen item's repair list (`REPL`, a form list); with Jury Rigging
//!   (perk entry point 48 "Has Jury Rigging" above 0) also anything of the
//!   same kind that isn't a quest item: weapons playable (`DNAM` flags
//!   0x80 clear) with the same skill (`DNAM` u32 at 104) and animation type
//!   (`DNAM` byte 0), armour playable (`BMDT` 0x40 clear) with the same
//!   weight class (heavy 0x80, medium 0x08) and equip type (`ETYP`).
//! - Merchants (`ShowRepairMenu`, `RepairServicesMenu`): a vendor with
//!   Repair s mends to (`fRepairSkillBase` + (`fRepairSkillMax` −
//!   `fRepairSkillBase`) × s / 100) / 10, at most 1, of an item's health
//!   (`007b7f70`); for a cost (`00648230`) of `fItemRepairCostMult` ×
//!   value × (that target × 10 − condition / 10) / 10, rounded half up
//!   and at least 1 when above 0.5, else nothing (and it can't be
//!   repaired). Their list (`007b7b40`) holds the player's weapons and
//!   armour with health, not caps, quest items or armour marked not
//!   playable, below `fRepairSkillMax` × 10 percent, armour only with a
//!   damage threshold or resistance; ordered (`007b7c40`) what can be paid
//!   for first, then the most health first (equipped first on a tie), the
//!   rest the other way round. The player pays the vendor (`008924e0`).
//!
//! Repair is read as the menus read it (`0066ef20`): the current value,
//! within 0..100 (`0066f190`), truncated.
//!
//! Here an item's condition is kept per holder and kind (weapons and
//! armour, `GameState::weapon_health`), so a repair mends every one of that
//! kind the holder has, where the game mends one.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, Facts, GameState};

/// The Repair actor value.
pub const REPAIR: u16 = 39;
/// The misc statistic "Items Repaired" (`004d5c60(0x22)` in `007b5d80`).
pub const ITEMS_REPAIRED: u8 = 34;
/// "Has Jury Rigging" (`005e58f0(0x30, …)` in `0047bb50`).
pub const HAS_JURY_RIGGING: u8 = 48;
/// The sound of a repair (`007b5b40`, `007b7d80`).
pub const REPAIR_SOUND: &str = "UIRepairWeapon";
/// The sound of a merchant's line that can't be repaired (`007b7d80`).
pub const CANT_SOUND: &str = "UIVATSInsufficientAP";

const WEAP: FourCC = FourCC::new(b"WEAP");
const ARMO: FourCC = FourCC::new(b"ARMO");
const DNAM: FourCC = FourCC::new(b"DNAM");
const BMDT: FourCC = FourCC::new(b"BMDT");
const ETYP: FourCC = FourCC::new(b"ETYP");
const REPL: FourCC = FourCC::new(b"REPL");

fn setting(order: &LoadOrder, name: &str, default: f32) -> f32 {
    game_setting(order, name).unwrap_or(default)
}

/// Someone's Repair as the repair menus read it (`0066ef20`): the current
/// value within 0..100, truncated.
pub fn skill(order: &LoadOrder, state: &GameState, who: FormId) -> i32 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, REPAIR)
    .unwrap_or(0.0)
    .clamp(0.0, 100.0) as i32
}

/// The Pip-Boy's repair (`00648090`): what an item at `a` percent mended
/// with one at `b` comes to, in percent (not capped).
pub fn mended(order: &LoadOrder, skill: i32, a: f32, b: f32) -> f32 {
    let (a, b) = (a / 10.0, b / 10.0);
    let max = setting(order, "fRepairMax", 2.0);
    let min = setting(order, "fRepairMin", 0.5);
    let scavenge = setting(order, "fRepairScavengeMult", 0.05);
    let by_skill = (max - min) * skill as f32 / 100.0;
    let from_lower = a.min(b) * scavenge;
    (a.max(b) + min + by_skill + from_lower) * 10.0
}

/// The condition (0..1) the Pip-Boy's repair leaves (`007b57f0`,
/// `007b5d80`): [`mended`] / 100, at most 1.
pub fn mended_condition(order: &LoadOrder, skill: i32, a: f32, b: f32) -> f32 {
    (mended(order, skill, a, b) / 100.0).min(1.0)
}

/// What a vendor with Repair `skill` mends to, 0..1 (`007b7f70`).
pub fn service_target(order: &LoadOrder, skill: i32) -> f32 {
    (service_points(order, skill) / 10.0).min(1.0)
}

/// `fRepairSkillBase` + (`fRepairSkillMax` − base) × skill / 100.
fn service_points(order: &LoadOrder, skill: i32) -> f32 {
    let base = setting(order, "fRepairSkillBase", 4.0);
    let max = setting(order, "fRepairSkillMax", 9.0);
    (max - base) * skill as f32 / 100.0 + base
}

/// A merchant's price for mending an item at `condition` percent worth
/// `value` (`00648230`); 0: they can't.
pub fn service_cost(order: &LoadOrder, skill: i32, condition: f32, value: f32) -> i32 {
    let target = service_points(order, skill);
    let mult = setting(order, "fItemRepairCostMult", 1.0);
    let cost = mult * value * ((target - condition / 10.0) / 10.0);
    if cost > 0.5 {
        (crate::barter::round_to(cost, 1.0) as i32).max(1)
    } else {
        0
    }
}

/// Whether an item has a condition: weapons and armour.
pub fn has_condition(order: &LoadOrder, item: FormId) -> bool {
    order
        .get(item)
        .is_some_and(|r| r.entry.header.kind == WEAP || r.entry.header.kind == ARMO)
}

/// An item's full health (`004873d0`; weapons `004bcf00`, whose mods
/// aren't kept here): `DATA` i32 at 4 for weapons and armour.
pub fn max_health(order: &LoadOrder, item: FormId) -> i32 {
    stated_health(order, item).unwrap_or(0)
}

/// [`max_health`] when the record has a `DATA` to give it.
pub fn stated_health(order: &LoadOrder, item: FormId) -> Option<i32> {
    order
        .get(item)
        .filter(|r| r.entry.header.kind == WEAP || r.entry.header.kind == ARMO)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            r.get(esm::sig::DATA)
                .filter(|s| s.data.len() >= 8)
                .map(|s| le_u32(&s.data, 4) as i32)
        })
}

/// An item's condition in percent (`004bcdb0(…, 1)`), as kept (full when
/// nothing wore it).
pub fn condition(state: &GameState, holder: FormId, item: FormId) -> f32 {
    crate::combat::weapon_condition(state, holder, item) * 100.0
}

/// Armour's damage resistance and threshold (`DNAM`: i16 at 0, f32 at 4).
pub fn armour_stats(order: &LoadOrder, item: FormId) -> Option<(f32, f32)> {
    let rr = order.get(item).filter(|r| r.entry.header.kind == ARMO)?;
    let record = rr.record().ok()?;
    let d = record.get(DNAM).filter(|s| s.data.len() >= 8)?;
    let d = &d.data;
    Some((
        f32::from(i16::from_le_bytes([d[0], d[1]])),
        f32::from_le_bytes([d[4], d[5], d[6], d[7]]),
    ))
}

/// Armour's resistance or threshold at a condition 0..1 (`00646360`,
/// `00646d40`): whole above half, × (0.5 + condition) at or below.
pub fn armour_at(value: f32, condition: f32) -> f32 {
    let f = if condition <= 0.5 {
        1.0 - (0.5 - condition)
    } else {
        1.0
    };
    value * f
}

/// The figure the repair menus show beside an item's condition
/// (`007b82f0`, `007b7020`): its title setting and value ("--" for none).
#[derive(Debug, Clone, PartialEq)]
pub struct Stat {
    pub title: &'static str,
    pub value: Option<i32>,
}

/// [`Stat`] for an item at a condition (0..1): a weapon's damage
/// (`006450f0`, rounded; here [`crate::combat::weapon_damage_at`] × a
/// split beam's 1.3, without the ammunition's effects and perk entry 0
/// that `006450f0` adds),
/// `sInventoryDamage`; armour's damage threshold there when above 0
/// (`sInventoryDamageThreshold`), else its resistance
/// (`sInventoryDamageResistance`, "--" when neither), truncated.
pub fn shown_stat(
    order: &LoadOrder,
    state: &GameState,
    item: FormId,
    condition: f32,
) -> Option<Stat> {
    let kind = order.get(item)?.entry.header.kind;
    if kind == WEAP {
        let w = crate::combat::Weapon::load(order, item)?;
        let d =
            crate::combat::weapon_damage_at(order, state, PLAYER_REF, Some(&w), false, condition)
                * crate::weapon_mods::shown_damage_mult(order, state, PLAYER_REF, item);
        return Some(Stat {
            title: "sInventoryDamage",
            value: Some(crate::barter::round_to(d, 1.0) as i32),
        });
    }
    if kind != ARMO {
        return None;
    }
    let (dr, dt) = armour_stats(order, item).unwrap_or((0.0, 0.0));
    let dt = armour_at(dt.trunc(), condition);
    let dr = armour_at(dr.trunc(), condition);
    Some(if dt > 0.0 {
        Stat {
            title: "sInventoryDamageThreshold",
            value: Some(dt as i32),
        }
    } else if dr > 0.0 {
        Stat {
            title: "sInventoryDamageResistance",
            value: Some(dr as i32),
        }
    } else {
        Stat {
            title: "sInventoryDamageResistance",
            value: None,
        }
    })
}

/// One of the player's things on a merchant's repair list.
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceLine {
    pub item: FormId,
    pub name: String,
    pub count: i32,
    /// The game's form type number (0x18 armour, 0x28 weapons).
    pub form_type: u8,
    pub equipped: bool,
    /// Its condition in percent.
    pub condition: f32,
    /// Its health (condition × full health).
    pub health: f32,
    /// What mending it costs (0: it can't be).
    pub cost: i32,
    /// It costs something the player can pay (`_CanRepair`).
    pub can_repair: bool,
    pub icon: String,
    /// [`shown_stat`] now and at the vendor's target.
    pub stat: Option<Stat>,
    pub stat_after: Option<Stat>,
}

/// Whether a merchant's list holds a thing (`007b7b40`).
pub fn service_lists(
    order: &LoadOrder,
    state: &GameState,
    line: &crate::items::InventoryLine,
) -> bool {
    let kind = order.get(line.item).map(|r| r.entry.header.kind);
    let armour = kind == Some(ARMO);
    if line.item == crate::items::CAPS
        || (armour && !line.playable)
        || line.quest_item
        || !(armour || kind == Some(WEAP))
        || max_health(order, line.item) <= 0
    {
        return false;
    }
    let top = setting(order, "fRepairSkillMax", 9.0) * 10.0;
    if top <= condition(state, PLAYER_REF, line.item) {
        return false;
    }
    if armour {
        let (dr, dt) = armour_stats(order, line.item).unwrap_or((0.0, 0.0));
        return dr > 0.0 || dt > 0.0;
    }
    true
}

/// The merchant's list (`007b8b30`): the player's things they can mend,
/// each with its cost at the vendor's Repair `skill`, and the total (`Repair
/// All`, `+0x98`: every line's cost), in the menu's order (`007b7c40`).
pub fn service_lines(order: &LoadOrder, state: &GameState, skill: i32) -> (Vec<ServiceLine>, i32) {
    let caps = state.item_count(order, PLAYER_REF, crate::barter::caps(order));
    let target = service_target(order, skill);
    let mut lines: Vec<ServiceLine> = crate::items::inventory_lines(order, state, PLAYER_REF)
        .into_iter()
        .filter(|l| service_lists(order, state, l))
        .map(|l| {
            let condition = condition(state, PLAYER_REF, l.item);
            let value = crate::barter::value_now(order, state, l.item);
            let cost = service_cost(order, skill, condition, value);
            ServiceLine {
                item: l.item,
                name: l.name,
                count: l.count,
                form_type: l.form_type,
                equipped: l.equipped,
                condition,
                health: condition / 100.0 * max_health(order, l.item) as f32,
                cost,
                can_repair: cost > 0 && cost <= caps,
                icon: l.icon,
                stat: shown_stat(order, state, l.item, condition / 100.0),
                stat_after: shown_stat(order, state, l.item, target),
            }
        })
        .collect();
    sort_service_lines(&mut lines);
    let total = lines.iter().map(|l| l.cost).sum();
    (lines, total)
}

/// `007b7c40`: < 0 puts `a` first.
pub fn compare_service_lines(a: &ServiceLine, b: &ServiceLine) -> i32 {
    use std::cmp::Ordering;
    let mut r = match (a.can_repair, b.can_repair) {
        (true, false) => -1,
        (false, true) => 1,
        _ => match b.health.partial_cmp(&a.health) {
            Some(Ordering::Greater) => 1,
            Some(Ordering::Less) => -1,
            _ if a.equipped => -1,
            _ if b.equipped => 1,
            _ => 0,
        },
    };
    if !a.can_repair && !b.can_repair {
        r = -r;
    }
    r
}

/// The game's list sort (`007653f0`, a Shell sort; `ui::list::ListBox::
/// sort`) with [`compare_service_lines`].
pub fn sort_service_lines(lines: &mut [ServiceLine]) {
    let n = lines.len();
    let mut gap = 1usize;
    while n > 0 && gap <= (n - 1) / 9 {
        gap = gap * 3 + 1;
    }
    while gap > 0 {
        for i in gap..n {
            let it = lines[i].clone();
            let mut j = i;
            while j >= gap && compare_service_lines(&it, &lines[j - gap]) < 0 {
                lines[j] = lines[j - gap].clone();
                j -= gap;
            }
            lines[j] = it;
        }
        gap /= 3;
    }
}

/// A merchant mends one of the player's things (`007b7f70`) for `cost`
/// caps, paid to them (`008924e0`).
pub fn repair_by(
    order: &LoadOrder,
    state: &mut GameState,
    vendor: FormId,
    item: FormId,
    cost: i32,
) {
    let s = skill(order, state, vendor);
    let target = service_target(order, s);
    if has_condition(order, item) {
        state.weapon_health.insert((PLAYER_REF, item), target);
    }
    pay(order, state, vendor, cost);
}

/// The player pays a vendor (`008924e0` → `004cb4b0`: their caps to the
/// vendor).
pub fn pay(order: &LoadOrder, state: &mut GameState, vendor: FormId, cost: i32) {
    if cost > 0 {
        let caps = crate::barter::caps(order);
        state.stock(order, PLAYER_REF);
        state.move_item(order, PLAYER_REF, vendor, caps, cost);
    }
}

/// Whether `candidate` can mend `chosen` (`0047bb50`).
pub fn mends(order: &LoadOrder, state: &GameState, chosen: FormId, candidate: FormId) -> bool {
    if chosen == candidate {
        return true;
    }
    let Some(list) = repair_list(order, chosen) else {
        return false;
    };
    if crate::script_functions::form_list(order, state, list).contains(&candidate) {
        return true;
    }
    if crate::perks::apply(order, state, HAS_JURY_RIGGING, 0.0) <= 0.0 {
        return false;
    }
    let kind = |f: FormId| order.get(f).map(|r| r.entry.header.kind);
    if kind(chosen) != kind(candidate)
        || crate::script_functions::is_quest_item(order, state, candidate)
    {
        return false;
    }
    let sub = |f: FormId, sig: FourCC| -> Vec<u8> {
        order
            .get(f)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(sig).map(|s| s.data.clone()))
            .unwrap_or_default()
    };
    match kind(candidate) {
        Some(k) if k == WEAP => {
            let (c, w) = (sub(candidate, DNAM), sub(chosen, DNAM));
            let byte = |d: &[u8], at: usize| d.get(at).copied().unwrap_or(0);
            let skill = |d: &[u8]| (d.len() >= 108).then(|| le_u32(d, 104));
            byte(&c, 12) & 0x80 == 0 && skill(&c) == skill(&w) && byte(&c, 0) == byte(&w, 0)
        }
        Some(k) if k == ARMO => {
            let flags = |f: FormId| sub(f, BMDT).get(4).copied().unwrap_or(0);
            let class = |f: FormId| {
                let b = flags(f);
                if b & 0x80 != 0 {
                    1
                } else if b & 0x08 != 0 {
                    2
                } else {
                    0
                }
            };
            let etyp = |f: FormId| {
                let d = sub(f, ETYP);
                (d.len() >= 4).then(|| le_u32(&d, 0))
            };
            flags(candidate) & 0x40 == 0
                && class(candidate) == class(chosen)
                && etyp(candidate) == etyp(chosen)
        }
        _ => false,
    }
}

/// An item's repair list (`REPL`, `0047bac0`).
pub fn repair_list(order: &LoadOrder, item: FormId) -> Option<FormId> {
    let rr = order
        .get(item)
        .filter(|r| r.entry.header.kind == WEAP || r.entry.header.kind == ARMO)?;
    let record = rr.record().ok()?;
    let s = record.get(REPL).filter(|s| s.data.len() >= 4)?;
    Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0)))).filter(|f| f.0 != 0)
}

/// One of the player's things the Pip-Boy's repair can use.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub item: FormId,
    pub name: String,
    /// How many of it can be used (`007b6aa0` lists each one).
    pub count: i32,
    /// Its condition in percent.
    pub condition: f32,
    /// The chosen item's condition after mending with it (0..1).
    pub mends_to: f32,
    /// One of it is worn (the first line's mark).
    pub equipped: bool,
    /// The chosen item's own kind: its first one is the chosen one (not
    /// counted in `count`), listed where its kind comes.
    pub chosen: bool,
}

/// Whether the Pip-Boy offers to repair an item (`00781860`): a weapon or
/// armour below full condition (rounded up), with another of it, or
/// something else [`mends`] accepts that isn't worn.
pub fn can_repair(order: &LoadOrder, state: &GameState, item: FormId) -> bool {
    let kind = order.get(item).map(|r| r.entry.header.kind);
    if !(kind == Some(WEAP) || kind == Some(ARMO)) {
        return false;
    }
    if condition(state, PLAYER_REF, item).ceil() >= 100.0 {
        return false;
    }
    if state.item_count(order, PLAYER_REF, item) >= 2 {
        return true;
    }
    state
        .inventory(order, PLAYER_REF)
        .iter()
        .any(|&(other, _)| {
            other != item
                && !state.is_equipped(PLAYER_REF, other)
                && mends(order, state, item, other)
        })
}

/// What the Pip-Boy can mend `chosen` with (`007b6aa0`), in the player's
/// things' order: every one of each thing [`mends`] accepts (each its own
/// line) but the chosen one itself, the first of its kind (its kind's
/// entry has `chosen` set, with what's left of it to use); of the chosen
/// kind one worn isn't offered (the chosen one is that one here), of other
/// kinds it is.
pub fn parts(order: &LoadOrder, state: &GameState, chosen: FormId) -> Vec<Part> {
    let s = skill(order, state, PLAYER_REF);
    let at = condition(state, PLAYER_REF, chosen);
    let mut out = Vec::new();
    for line in crate::items::inventory_lines(order, state, PLAYER_REF) {
        if !mends(order, state, chosen, line.item) {
            continue;
        }
        let c = condition(state, PLAYER_REF, line.item);
        // `007b6aa0`: the chosen kind's own at full condition aren't
        // listed (none here: they share the chosen one's).
        if line.item == chosen && c.ceil() >= 100.0 {
            continue;
        }
        let count = if line.item == chosen {
            line.count - 1
        } else {
            line.count
        };
        if count <= 0 && line.item != chosen {
            continue;
        }
        out.push(Part {
            item: line.item,
            name: line.name,
            count: count.max(0),
            condition: c,
            mends_to: mended_condition(order, s, at, c),
            equipped: line.equipped && line.item != chosen,
            chosen: line.item == chosen,
        });
    }
    out
}

/// The Pip-Boy mends `chosen` with one `part` (`007b5d80`): its new
/// condition, one of the part gone, "Items Repaired" counted. The new
/// condition (0..1).
pub fn repair_with(order: &LoadOrder, state: &mut GameState, chosen: FormId, part: FormId) -> f32 {
    let s = skill(order, state, PLAYER_REF);
    let to = mended_condition(
        order,
        s,
        condition(state, PLAYER_REF, chosen),
        condition(state, PLAYER_REF, part),
    );
    state.stock(order, PLAYER_REF);
    let key = (PLAYER_REF, part);
    if let Some(n) = state.items.get_mut(&key) {
        *n -= 1;
        if *n <= 0 {
            state.items.remove(&key);
            if state.is_equipped(PLAYER_REF, part) {
                state.unequip_item(order, PLAYER_REF, part);
            }
        }
    }
    if has_condition(order, chosen) {
        state.weapon_health.insert((PLAYER_REF, chosen), to);
    }
    crate::stats::bump(state, ITEMS_REPAIRED, 1);
    to
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue::Speaker;
    use testdata::{group, record, sub};

    /// Two vendors as the game's records give them: Repair 15 with an
    /// offset of 60 (`DNAM` byte 21, as Mick's), stats set by hand and by
    /// the game (`ACBS` flag 0x10).
    fn vendors(tag: &str) -> (std::path::PathBuf, LoadOrder) {
        let dir = std::env::temp_dir().join(format!("nv-rs-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        let npc = |id: u32, flags: u32| {
            let mut acbs = flags.to_le_bytes().to_vec();
            acbs.extend([0; 20]);
            let mut dnam = vec![10u8; 28];
            dnam[7] = 15;
            dnam[14..].fill(0);
            dnam[21] = 60;
            let mut d = sub(b"ACBS", &acbs);
            let mut data = 50i32.to_le_bytes().to_vec();
            data.extend([5; 7]);
            d.extend(sub(b"DATA", &data));
            d.extend(sub(b"DNAM", &dnam));
            record(b"NPC_", id, &d)
        };
        let mut npcs = npc(0x800, 0x200);
        npcs.extend(npc(0x801, 0x210));
        plugin.extend(group(*b"NPC_", 0, &npcs));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        (dir, order)
    }

    /// `00607850`: a skill is `cSkill` + `cOffset` (Xbox PDB `NPC_DATA`)
    /// unless the game works the stats out (`005f0d00`: `ACBS` 0x10).
    #[test]
    fn a_vendors_repair_adds_its_offset() {
        let (dir, order) = vendors("repair-offsets");
        let state = GameState::default();
        let ask = |base: u32| {
            let speaker = Speaker {
                reference: FormId(0x900 + base),
                base: FormId(base),
                name: None,
                voice: None,
                race: None,
                female: false,
                factions: Vec::new(),
            };
            Facts {
                order: &order,
                state: &state,
                speaker: Some(&speaker),
            }
            .current_actor_value(FormId(0x900 + base), REPAIR)
        };
        assert_eq!(ask(0x800), Some(75.0));
        assert_eq!(ask(0x801), Some(15.0));
        let _ = std::fs::remove_dir_all(dir);
    }
}
