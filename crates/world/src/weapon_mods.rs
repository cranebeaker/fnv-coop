//! Weapon mods (`TESObjectIMOD` items fitted to a weapon's three mod slots;
//! read from FalloutNV.exe with the Xbox 360 build's names, notes in
//! `docs/WEAPON_MODS.md`).
//!
//! - A weapon's slots: its `WMI1`–`WMI3` mod items, and in `DNAM` (the
//!   `OBJ_WEAP` data) each slot's effect (`eModActionOne`–`Three` at 140,
//!   144, 148, `WEAPON_MOD_EFFECT`) and two values (152.., 184..). The
//!   fitted slots are a byte of flags on the weapon (extra data 0x8D,
//!   `ExtraWeaponModFlags`: 1, 2, 4).
//! - `HasModEffect` (`004bda70`): a fitted slot (in the order 1, 2, 3) has
//!   the effect. Its value (`004bd8d0`): that slot's; `GetModEffectValue`
//!   (`004bcf60`): the first slot with the effect, fitted or not.
//! - What the effects change: damage + the value (`00644ce0`, with the
//!   melee/unarmed bonus), clip + the value rounded (`004fe160`), spread and
//!   minimum spread − the value, at least 0 (`00524b80`, `00524be0`),
//!   weight − the value (`004be380`), attack speed + the value
//!   (`00646020`, `DNAM` 60), projectiles + the value rounded
//!   (`00525b20`), the V.A.T.S. to-hit chance + the value rounded
//!   (`00647730`), most condition: (health + the value) truncated
//!   (`004bcf00`); an item's worth + each fitted mod's value (`004bd400`).
//! - Fitting one (`00783af0`, from the mod menu `007838a0`): the first slot
//!   (1, 2, 3) not fitted whose item is the mod; a condition mod adds its
//!   value to the weapon's health too; misc stat 0x21 "Weapon
//!   Modifications" + 1; the mod item used up.
//!
//! The game splits a modded weapon from its stack (its own extra data); the
//! mods here are kept per holder and weapon, as its condition is.

use esm::{FormId, FourCC, LoadOrder};

use crate::combat::Weapon;
use crate::scripting::GameState;

/// `WEAPON_MOD_EFFECT`.
pub mod effect {
    pub const DAMAGE: i32 = 1;
    pub const CLIP_SIZE: i32 = 2;
    pub const SPREAD: i32 = 3;
    pub const WEIGHT: i32 = 4;
    pub const AMMO_REGEN_SHOT: i32 = 5;
    pub const AMMO_REGEN_SECONDS: i32 = 6;
    pub const EQUIP_SPEED: i32 = 7;
    pub const FIRE_SPEED: i32 = 8;
    pub const PROJECTILE_SPEED: i32 = 9;
    pub const MAX_HEALTH: i32 = 10;
    pub const SILENCE: i32 = 11;
    pub const SPLIT_BEAM: i32 = 12;
    pub const VATS_BONUS: i32 = 13;
    pub const IRON_SIGHTS: i32 = 14;
    pub const VATS_SPECIAL_ATTACK: i32 = 15;
}

/// The misc stat fitting a mod counts in.
pub const WEAPON_MODIFICATIONS: u8 = 0x21;

const WEAP: FourCC = FourCC::new(b"WEAP");
const DNAM: FourCC = FourCC::new(b"DNAM");
const WMI: [FourCC; 3] = [
    FourCC::new(b"WMI1"),
    FourCC::new(b"WMI2"),
    FourCC::new(b"WMI3"),
];

/// One of a weapon's mod slots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slot {
    pub item: Option<FormId>,
    pub effect: i32,
    pub value: f32,
    pub value_two: f32,
}

/// A weapon's three slots.
pub fn slots(order: &LoadOrder, weapon: FormId) -> Option<[Slot; 3]> {
    let rr = order.get(weapon).filter(|r| r.entry.header.kind == WEAP)?;
    let record = rr.record().ok()?;
    let dnam = record.get(DNAM).map(|s| s.data.as_slice()).unwrap_or(&[]);
    let i32_at = |at: usize| {
        dnam.get(at..at + 4)
            .map_or(0, |b| i32::from_le_bytes(b.try_into().unwrap()))
    };
    let f32_at = |at: usize| {
        dnam.get(at..at + 4)
            .map_or(0.0, |b| f32::from_le_bytes(b.try_into().unwrap()))
    };
    let mut out = [Slot {
        item: None,
        effect: 0,
        value: 0.0,
        value_two: 0.0,
    }; 3];
    for (i, slot) in out.iter_mut().enumerate() {
        slot.item = record
            .get(WMI[i])
            .filter(|s| s.data.len() >= 4)
            .map(|s| {
                rr.plugin
                    .to_global(FormId(u32::from_le_bytes(s.data[0..4].try_into().unwrap())))
            })
            .filter(|f| f.0 != 0);
        slot.effect = i32_at(140 + 4 * i);
        slot.value = f32_at(152 + 4 * i);
        slot.value_two = f32_at(184 + 4 * i);
    }
    Some(out)
}

/// The fitted slots of a holder's weapon (`ExtraWeaponModFlags`).
pub fn flags(state: &GameState, holder: FormId, weapon: FormId) -> u8 {
    state
        .weapon_mods
        .get(&(holder, weapon))
        .copied()
        .unwrap_or(0)
}

/// The fitted slot with an effect (`004bda70` / `004bd8d0`), slots in order.
fn fitted(order: &LoadOrder, flags: u8, weapon: FormId, effect: i32) -> Option<Slot> {
    if flags == 0 {
        return None;
    }
    let slots = slots(order, weapon)?;
    (0..3)
        .find(|&i| flags & (1 << i) != 0 && slots[i].effect == effect)
        .map(|i| slots[i])
}

/// Whether a fitted mod has the effect (`004bda70`).
pub fn has_effect(order: &LoadOrder, flags: u8, weapon: FormId, effect: i32) -> bool {
    fitted(order, flags, weapon, effect).is_some()
}

/// The fitted mod's value for an effect (`004bd8d0`), if one has it.
pub fn bonus(order: &LoadOrder, flags: u8, weapon: FormId, effect: i32) -> Option<f32> {
    fitted(order, flags, weapon, effect).map(|s| s.value)
}

/// `GetModEffectValue` (`004bcf60`): the first slot with the effect's value
/// (or second value), fitted or not; 0 with none.
pub fn value_of(order: &LoadOrder, weapon: FormId, effect: i32, two: bool) -> f32 {
    slots(order, weapon)
        .and_then(|s| s.into_iter().find(|s| s.effect == effect))
        .map_or(0.0, |s| if two { s.value_two } else { s.value })
}

/// FPU rounding as `(int)ROUND(x)` does it (to nearest, ties to even).
pub fn round(x: f32) -> i32 {
    let f = x.floor();
    let rest = x - f;
    let up = rest > 0.5 || (rest == 0.5 && (f as i64) % 2 != 0);
    (if up { f + 1.0 } else { f }) as i32
}

/// A weapon as its holder's mods change it: clip (`004fe160`), spread
/// (`00524b80`, `00524be0`), attack speed (`00646020`), projectiles
/// (`00525b20`) and split beam's cone (`00523150`), most condition
/// (`004bcf00`).
pub fn modded(order: &LoadOrder, state: &GameState, holder: FormId, mut weapon: Weapon) -> Weapon {
    let f = flags(state, holder, weapon.form_id);
    if f == 0 {
        return weapon;
    }
    let id = weapon.form_id;
    let has = |e: i32| has_effect(order, f, id, e);
    let value = |e: i32| value_of(order, id, e, false);
    if has(effect::CLIP_SIZE) {
        weapon.clip = (weapon.clip as i32 + (round(value(effect::CLIP_SIZE)) & 0xff)) as u32;
    }
    if has(effect::SPREAD) {
        weapon.min_spread = (weapon.min_spread - value(effect::SPREAD)).max(0.0);
        weapon.spread = (weapon.spread - value(effect::SPREAD)).max(0.0);
    }
    if has(effect::FIRE_SPEED) {
        weapon.attack_mult += value(effect::FIRE_SPEED);
    }
    if has(effect::SPLIT_BEAM) {
        weapon.projectiles = round(value(effect::SPLIT_BEAM) + f32::from(weapon.projectiles)) as u8;
        // The cone (`00523150`): × the second value (0 on both laser
        // rifles, whose split beams fly straight).
        weapon.cone_mult = value_of(order, id, effect::SPLIT_BEAM, true);
    }
    if has(effect::MAX_HEALTH) {
        weapon.health = (weapon.health as f64 + f64::from(value(effect::MAX_HEALTH))) as i32;
    }
    weapon
}

/// What the damage the menus show is multiplied by (`006450f0`, the
/// figure the Pip-Boy, the mod screen and the repair menus show): 1.3 with
/// a split beam fitted, else 1.
pub fn shown_damage_mult(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: FormId,
) -> f32 {
    if has_effect(
        order,
        flags(state, holder, weapon),
        weapon,
        effect::SPLIT_BEAM,
    ) {
        1.3
    } else {
        1.0
    }
}

/// The one weapon the exe never gives its mod models (`00522df0`,
/// `004ab400`; not in FalloutNV.esm).
const NO_MOD_MODELS: FormId = FormId(0x1735D4);

/// The weapon's model with these mods fitted, as someone holds it
/// (`00522df0`, Xbox PDB `TESObjectWEAP::GetModTESModel`): `MWD1`–`MWD7` by
/// the fitted flags' value (the load puts `MWD{n}` where the flags `n` find
/// it: 1 the first mod, 2 the second, 3 both, 4 the third, … 7 all three;
/// the 9mm's `MWD3` is `9mmExtClipScp.NIF`), the plain `MODL` without mods
/// or when that one is empty.
pub fn model(order: &LoadOrder, weapon: FormId, flags: u8) -> Option<String> {
    let rr = order.get(weapon).filter(|r| r.entry.header.kind == WEAP)?;
    let record = rr.record().ok()?;
    let path = |sig: &[u8; 4]| {
        record
            .get(FourCC::new(sig))
            .map(|s| s.zstring())
            .filter(|p| !p.is_empty())
    };
    let modded = match flags & 7 {
        _ if weapon == NO_MOD_MODELS => None,
        0 => None,
        n => path(&[b'M', b'W', b'D', b'0' + n]),
    };
    modded.or_else(|| path(b"MODL"))
}

/// The model the player holds, in first and third person (`004ab400`, for
/// either of the player's bodies): the first-person object for the fitted
/// mods (`WNM{flags}`, `004ab500`, Xbox PDB
/// `TESObjectWEAP::Get1stPersonModObject`, filled in the same order as
/// `MWD`), else the plain one (`WNAM`, `008d8b00`): a `STAT` whose `MODL`
/// it is. Without either, [`model`]. (15 of the game's weapons differ:
/// Lily's carbine is the plain carbine, the fire axe `1stFireAxe.NIF`.)
pub fn player_model(order: &LoadOrder, weapon: FormId, flags: u8) -> Option<String> {
    let rr = order.get(weapon).filter(|r| r.entry.header.kind == WEAP)?;
    let record = rr.record().ok()?;
    let object = |sig: &[u8; 4]| {
        record
            .get(FourCC::new(sig))
            .filter(|s| s.data.len() >= 4)
            .map(|s| {
                rr.plugin
                    .to_global(FormId(u32::from_le_bytes(s.data[0..4].try_into().unwrap())))
            })
            .filter(|id| id.0 != 0 && order.get(*id).is_some())
    };
    let modded = match flags & 7 {
        _ if weapon == NO_MOD_MODELS => None,
        0 => None,
        n => object(&[b'W', b'N', b'M', b'0' + n]),
    };
    let Some(stat) = modded.or_else(|| object(b"WNAM")) else {
        return model(order, weapon, flags);
    };
    let stat = order.get(stat)?.record().ok()?;
    Some(
        stat.get(FourCC::new(b"MODL"))
            .map_or(String::new(), |s| s.zstring()),
    )
}

/// The weight a holder's mods take off a weapon (`004be380`).
pub fn weight_off(order: &LoadOrder, state: &GameState, holder: FormId, weapon: FormId) -> f32 {
    let f = flags(state, holder, weapon);
    if has_effect(order, f, weapon, effect::WEIGHT) {
        value_of(order, weapon, effect::WEIGHT, false)
    } else {
        0.0
    }
}

/// What the fitted mods add to a weapon's worth (`004bd400`: each one's
/// value).
pub fn worth_added(order: &LoadOrder, state: &GameState, holder: FormId, weapon: FormId) -> f32 {
    let f = flags(state, holder, weapon);
    let Some(slots) = (f != 0).then(|| slots(order, weapon)).flatten() else {
        return 0.0;
    };
    (0..3)
        .filter(|&i| f & (1 << i) != 0)
        .filter_map(|i| slots[i].item)
        .map(|m| crate::items::item_info(order, m).map_or(0, |i| i.value) as f32)
        .sum()
}

/// The slot a mod would go in (`00783af0`): the first not fitted whose item
/// it is.
pub fn slot_for(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: FormId,
    item: FormId,
) -> Option<usize> {
    let f = flags(state, holder, weapon);
    let slots = slots(order, weapon)?;
    (0..3).find(|&i| f & (1 << i) == 0 && slots[i].item == Some(item))
}

/// Fits a mod from the holder's things to their weapon (`00783af0`, the mod
/// menu's `007838a0`): false when it doesn't fit (or they have none).
pub fn attach(
    order: &LoadOrder,
    state: &mut GameState,
    holder: FormId,
    weapon: FormId,
    item: FormId,
) -> bool {
    state.stock(order, holder);
    if state.item_count(order, holder, item) < 1 {
        return false;
    }
    let Some(slot) = slot_for(order, state, holder, weapon, item) else {
        return false;
    };
    let slots = slots(order, weapon).expect("slot_for read them");
    // A condition mod: the weapon's health goes up by the value too, so its
    // share of the new most is (health + value) / (most + value).
    if slots[slot].effect == effect::MAX_HEALTH {
        let most = crate::repair::max_health(order, weapon) as f32;
        let v = slots[slot].value;
        let now = crate::combat::weapon_condition(state, holder, weapon) * most;
        if most + v > 0.0 {
            state
                .weapon_health
                .insert((holder, weapon), ((now + v) / (most + v)).min(1.0));
        }
    }
    *state.weapon_mods.entry((holder, weapon)).or_insert(0) |= 1 << slot;
    *state.misc_stats.entry(WEAPON_MODIFICATIONS).or_insert(0) += 1;
    if let Some(n) = state.items.get_mut(&(holder, item)) {
        *n -= 1;
    }
    true
}

/// The mods in the holder's things that would fit a weapon (the mod menu's
/// list).
pub fn fitting(
    order: &LoadOrder,
    state: &GameState,
    holder: FormId,
    weapon: FormId,
) -> Vec<FormId> {
    let Some(slots) = slots(order, weapon) else {
        return Vec::new();
    };
    let f = flags(state, holder, weapon);
    (0..3)
        .filter(|&i| f & (1 << i) == 0)
        .filter_map(|i| slots[i].item)
        .filter(|&m| state.item_count(order, holder, m) > 0)
        .collect()
}

/// Mods for the save: `weaponmods <holder> <weapon> <flags>`.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let mut v: Vec<_> = state.weapon_mods.iter().filter(|(_, f)| **f != 0).collect();
    v.sort();
    for ((holder, weapon), f) in v {
        line(format!("weaponmods {:08X} {:08X} {f}", holder.0, weapon.0));
    }
}

pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if *parts.first()? != "weaponmods" {
        return None;
    }
    let form = |i: usize| parts.get(i).and_then(|s| u32::from_str_radix(s, 16).ok());
    match (
        form(1),
        form(2),
        parts.get(3).and_then(|s| s.parse::<u8>().ok()),
    ) {
        (Some(h), Some(w), Some(f)) => {
            state.weapon_mods.insert((FormId(h), FormId(w)), f);
            Some(Ok(()))
        }
        _ => Some(Err(format!("can't read '{raw}'"))),
    }
}

#[cfg(test)]
mod tests {
    /// The FPU's default rounding: to nearest, ties to even.
    #[test]
    fn rounding() {
        let r = super::round;
        assert_eq!((r(0.5), r(1.5), r(2.5), r(-0.5)), (0, 2, 2, 0));
        assert_eq!((r(2.4), r(2.6), r(7.0)), (2, 3, 7));
    }
}
