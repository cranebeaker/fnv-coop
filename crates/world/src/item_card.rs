//! What the item cards work out beyond the record's own numbers (the
//! Pip-Boy's ITEMS card, the container's and barter's: `00707e30`,
//! `Interface::PopulateItemStatsDisplay` (Xbox PDB)): a weapon's damage a
//! second and an item's effects text. Read from `FalloutNV.exe` 1.4.0.525;
//! `docs/COMPANIONS.md` ("The numbers on the item cards") has the rules.

use esm::{FormId, FourCC, LoadOrder};

use crate::animation::pick::Library;
use crate::combat::Weapon;
use crate::combat_ai::Setting;
use crate::dialogue::PLAYER_REF;
use crate::dps::{self, DpsCall};
use crate::items::{self, ItemEffect};
use crate::perks;
use crate::scripting::{game_setting, game_setting_text, Facts, GameState};

/// The magic effect archetypes whose effects have an actor value
/// (`00403c80`: the table at `01183328`, flag 2): value modifier (0), 4,
/// 6, 7, 8, 11, 12, 15, 24, value and limb conditions (34), 36. The
/// others' effects count as no actor value (`EffectItem::
/// GetActorValueIndex`, `00403ea0`, gives -1).
const ARCHETYPES_WITH_VALUE: [u32; 11] = [0, 4, 6, 7, 8, 11, 12, 15, 24, 34, 36];

/// The magic effect the engine makes for itself and hides from the
/// cards, "Usage Monitor Effect" (form 0x14F, `00408f60`; `00404730`
/// compares with it).
const USAGE_MONITOR: FormId = FormId(0x14F);

/// The actor values whose amounts the card writes with their sign turned
/// (actor value flag 0x200, set by `0066f260`): RadiationRads (54),
/// Dehydration (73), Hunger (74), SleepDeprivation (75).
const SIGN_TURNED: [i32; 4] = [54, 73, 74, 75];

/// An actor value's abbreviation (`0066eb00`: the value's information
/// +0x3c, its `AVIF` record's `ANAM`: "HP", "PER", "Rads"); empty without.
pub fn actor_value_abbreviation(order: &LoadOrder, av: i32) -> String {
    let Some(name) = usize::try_from(av)
        .ok()
        .and_then(|i| script::ACTOR_VALUES.get(i))
    else {
        return String::new();
    };
    order
        .form_by_editor_id(&format!("AV{name}"))
        .and_then(|id| order.get(id))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.get(FourCC::new(b"ANAM")).map(|s| s.zstring()))
        .unwrap_or_default()
}

/// An effect list's text for the cards (`EffectItemList::BuildMenuString`
/// (Xbox PDB)), as the player sees it: of the effects whose conditions
/// pass on the player (and not the engine's usage monitor), those whose
/// magic effect shows its name alone (flag 0x2000) are written first, by
/// name, one a line; the rest are summed by actor value (the first one's
/// duration kept, its magnitude turned negative when the effect is
/// detrimental — flag 4 — and none when it recovers — flag 2; a "no
/// magnitude" or "no duration" effect, flags 0x100 and 0x80, gives 0) and
/// written latest-added first, ", " between, each as `ABBR %+i`, with a
/// duration `(%d%c)` or `(%.1f%c)` in seconds, minutes from 60 or hours
/// from 3600. A value's amount: turned for the values counted the other
/// way (rads, thirst, hunger, sleep); × the player's medicine
/// effectiveness (`fMagicMedicineSkillBase` + `fMagicMedicineSkillMult` ×
/// Medicine ÷ 100) for a medicine, × their survival effectiveness (the
/// same with Survival) for a food except on rads, Health's through the
/// perks' "Modify Recovered Health" (entry point 12) in both; positive
/// rads × (1 − the player's Rad Resistance, at most
/// `fPlayerMaxResistance`, ÷ 100) rounded down, and for a food through
/// "Modify Radiation Consumed" (44); each truncated, 0 shown as 1.
/// `medicine` and `food`: the list's kind (an aid item's `ENIT` flags 4,
/// and 2 when not every effect is hostile or poison; never for an
/// enchantment).
///
/// Translated from 00406620, 007e0b20 (decompiled, FalloutNV.exe
/// 1.4.0.525).
pub fn effects_text(
    order: &LoadOrder,
    state: &GameState,
    effects: &[ItemEffect],
    medicine: bool,
    food: bool,
) -> String {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let setting = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
    let player_av = |a: u16| facts.current_actor_value(PLAYER_REF, a).unwrap_or(0.0) as f32;
    let mut out = String::new();
    let mut named = false;
    // The status list (`StatsMenu::StatusDataList` (Xbox PDB)): actor
    // value, magnitude, duration; new ones go to its head (`005ae3d0`).
    let mut status: Vec<(i32, i32, i32)> = Vec::new();
    for e in effects {
        if e.effect == USAGE_MONITOR
            || !(e.conditions.is_empty()
                || facts.conditions_pass(&e.conditions, PLAYER_REF, PLAYER_REF))
        {
            continue;
        }
        if e.flags & 0x2000 != 0 {
            if named {
                out.push('\n');
            }
            out.push_str(&e.name);
            named = true;
            continue;
        }
        let av = if ARCHETYPES_WITH_VALUE.contains(&e.archetype) {
            e.actor_value
        } else {
            -1
        };
        let magnitude = if e.flags & 0x100 != 0 { 0 } else { e.magnitude };
        match status.iter_mut().find(|s| s.0 == av) {
            Some(s) => s.1 += magnitude,
            None => {
                let m = if e.flags & 0x04 != 0 {
                    -magnitude
                } else {
                    magnitude
                };
                let duration = if e.flags & 0x02 != 0 || e.flags & 0x80 != 0 {
                    0
                } else {
                    e.duration
                };
                status.insert(0, (av, m, duration));
            }
        }
    }
    let medicine_mult = setting("fMagicMedicineSkillBase", 1.0)
        + player_av(37) / 100.0 * setting("fMagicMedicineSkillMult", 0.5);
    let survival_mult = setting("fMagicSurvivalSkillBase", 1.0)
        + player_av(44) / 100.0 * setting("fMagicSurvivalSkillMult", 0.5);
    let recovered = |v: f32, av: i32| {
        if av == 16 {
            perks::apply(order, state, perks::entry::MODIFY_RECOVERED_HEALTH, v)
        } else {
            v
        }
    };
    for &(av, magnitude, duration) in &status {
        if av == -1 {
            continue;
        }
        let mut amount = magnitude;
        if SIGN_TURNED.contains(&av) {
            amount = -amount;
        }
        if medicine {
            amount = recovered(medicine_mult * amount as f32, av) as i32;
        }
        if food && av != 54 {
            amount = recovered(survival_mult * amount as f32, av) as i32;
        }
        if av == 54 && amount > 0 {
            // `008c4330`: the player's resistance, at most
            // `fPlayerMaxResistance`, as a share left.
            let resist = player_av(20)
                .min(setting("fPlayerMaxResistance", 85.0))
                .max(0.0);
            amount = ((1.0 - resist * 0.01) * amount as f32).floor() as i32;
        }
        if food && av == 54 {
            let mult = perks::apply(order, state, perks::entry::MODIFY_RADIATION_CONSUMED, 1.0);
            amount = (amount as f32 * mult) as i32;
        }
        if amount == 0 {
            amount = 1;
        }
        if !out.is_empty() {
            out.push_str(", ");
        }
        let abbreviation = actor_value_abbreviation(order, av);
        if duration == 0 {
            out.push_str(&format!("{abbreviation} {amount:+}"));
        } else {
            let (mut t, mut unit) = (duration as f32, 's');
            if t >= 3600.0 {
                t /= 3600.0;
                unit = 'h';
            } else if t >= 60.0 {
                t /= 60.0;
                unit = 'm';
            }
            if t == t.floor() {
                out.push_str(&format!("{abbreviation} {amount:+}({}{unit})", t as i32));
            } else {
                out.push_str(&format!("{abbreviation} {amount:+}({t:.1}{unit})"));
            }
        }
    }
    out
}

/// An ammunition's effects text (`00503a70`): each of its effects
/// (`RCIL` → `AMEF`) as "%s %s %.2f" — what it changes (`sAmmoEffectDAM`
/// "DAM", `…DR` "Target DR", `…DT` "Target DT", `…Spread` "Gun Spread",
/// `…Condition` "Gun CND", `…Fatigue` "Target Fatigue", by its type 0 to
/// 5), how ("+" add, "x" multiply, "-" subtract) and by how much — one a
/// line.
///
/// Translated from 00503a70, 0059a1e0 (decompiled, FalloutNV.exe
/// 1.4.0.525).
pub fn ammo_effects_text(order: &LoadOrder, ammo: FormId) -> String {
    const NAMES: [(&str, &str); 6] = [
        ("sAmmoEffectDAM", "DAM"),
        ("sAmmoEffectDR", "Target DR"),
        ("sAmmoEffectDT", "Target DT"),
        ("sAmmoEffectSpread", "Gun Spread"),
        ("sAmmoEffectCondition", "Gun CND"),
        ("sAmmoEffectFatigue", "Target Fatigue"),
    ];
    let mut lines = Vec::new();
    for (kind, op, value) in crate::combat::ammo_effects(order, ammo) {
        let Some(&(setting, default)) = NAMES.get(kind as usize) else {
            continue;
        };
        let name = game_setting_text(order, setting).unwrap_or_else(|| default.to_string());
        let sign = match op {
            0 => "+",
            1 => "x",
            2 => "-",
            _ => "",
        };
        lines.push(format!("{name} {sign} {value:.2}"));
    }
    lines.join("\n")
}

/// Whether an aid item's effects can poison (`EffectItemList::CanBePoison`
/// (Xbox PDB), `00405da0`): some effects, every one hostile (`MGEF` flag
/// 1) or flagged 0x400000.
fn can_be_poison(effects: &[ItemEffect]) -> bool {
    !effects.is_empty()
        && effects
            .iter()
            .all(|e| e.flags & 0x01 != 0 || e.flags & 0x0040_0000 != 0)
}

/// The effects text the item card shows for `item` of the player's
/// (`00707e30`, mask bit 0x40), or `None` when the card hides it: an aid
/// item's or ingredient's effects ([`effects_text`], medicine and food by
/// its `ENIT` flags: `AlchemyItem::IsMedicine` / `IsFood` (Xbox PDB)); a
/// weapon's or apparel's enchantment's (`EITM` → `ENCH`; not one flagged
/// "hide effect", `ENIT` flags 0x04); ammunition's ([`ammo_effects_text`]);
/// a weapon mod's description (`IMOD` `DESC`). Empty text hides it, and so
/// does a weapon with a mod fitted (its card shows the mods instead).
pub fn card_effects(order: &LoadOrder, state: &GameState, item: FormId) -> Option<String> {
    let rr = order.get(item)?;
    let kind = rr.entry.header.kind;
    let record = rr.record().ok()?;
    let text = match kind.as_bytes() {
        b"ALCH" | b"INGR" => {
            let effects = items::effects(order, item);
            let flags = items::ingestible_flags(order, item).unwrap_or(0);
            let medicine = flags & items::ingestible::MEDICINE != 0;
            let food = flags & items::ingestible::FOOD != 0 && !can_be_poison(&effects);
            effects_text(order, state, &effects, medicine, food)
        }
        b"AMMO" => ammo_effects_text(order, item),
        b"IMOD" => record
            .get(FourCC::new(b"DESC"))
            .map(|s| s.zstring())
            .unwrap_or_default(),
        b"WEAP" | b"ARMO" => {
            if kind.as_bytes() == b"WEAP" && crate::weapon_mods::flags(state, PLAYER_REF, item) != 0
            {
                return None;
            }
            let ench = record
                .get(FourCC::new(b"EITM"))
                .filter(|s| s.data.len() >= 4)
                .map(|s| rr.plugin.to_global(FormId(crate::cell::le_u32(&s.data, 0))))?;
            let hidden = order
                .get(ench)
                .and_then(|r| r.record().ok())
                .and_then(|r| {
                    r.get(FourCC::new(b"ENIT"))
                        .and_then(|s| s.data.get(12).copied())
                })
                .is_some_and(|f| f & 0x04 != 0);
            if hidden {
                return None;
            }
            effects_text(order, state, &items::effects(order, ench), false, false)
        }
        _ => return None,
    };
    (!text.is_empty()).then_some(text)
}

/// The damage a second the item card shows for a weapon of the player's
/// (`00707e30`: `00645380` for the player, at the weapon's condition, with
/// their perks, its inventory entry — so their mods count and the shots a
/// second come off their animations — and, for the weapon in their hands,
/// the ammunition it's loaded with), "%d" rounded half up.
pub fn card_dps(
    order: &LoadOrder,
    state: &GameState,
    weapon: &Weapon,
    anims: Option<&mut dyn Library>,
    s: Setting,
) -> f32 {
    let item = weapon.form_id;
    let ammo = state
        .is_equipped(PLAYER_REF, item)
        .then(|| weapon.ammo_in_use(order, state, PLAYER_REF))
        .flatten();
    let call = DpsCall {
        who: PLAYER_REF,
        weapon: Some(weapon),
        condition: crate::combat::weapon_condition(state, PLAYER_REF, item),
        perks: true,
        entry: true,
        ammo,
    };
    dps::weapon_dps(order, state, &call, anims, s)
}
