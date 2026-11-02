//! Effects on people and creatures: what aid items, spells and abilities do
//! to their actor values, and for how long.
//!
//! An item or spell lists its effects (`EFID` + `EFIT`, read by
//! [`crate::items::effects`]); each names a magic effect (`MGEF`) whose
//! `DATA` says how it works. Read from the records and checked against
//! what the game's items are known to do:
//!
//! - **Recover** (flag 0x02): the value comes back when the effect ends,
//!   so it's a modifier while it lasts. Buffout's "Increase Strength"
//!   (0x72) has it: +2 Strength for 240 s, then back.
//! - Without it the change stays: at once with no duration, else the
//!   magnitude every second for the duration ("Restore Health" 0x70: a
//!   Nuka-Cola's 2 for 25 s heals 50 over 25 s).
//! - **Detrimental** (flag 0x04) lowers the value; the resisting actor
//!   value (at 16) takes its percentage off ("Damage Rads" is resisted by
//!   Rad Resistance, 20).
//! - **Script** effects (archetype 1) run their script (the associated
//!   item, at 8) on whoever they affect: `ScriptEffectStart` once,
//!   `ScriptEffectUpdate` while they last, `ScriptEffectFinish` at the end.
//!   Nuka-Cola's cap effect is `additem caps001 1` at the start.
//!
//! Rads, dehydration, hunger and sleep deprivation count up as they're
//! "damaged": the game's own scripts read `player.getav RadiationRads >=
//! 200` (`CGTutorialSCRIPT`) and cure with `player.RestoreAV
//! RadiationRads 1000` (the doctors' dialogue).
//!
//! Guesses: changes over time are applied smoothly rather than once a
//! second; resistance scales a detrimental change by (1 − resistance /
//! 100); a script effect with no duration runs start, update and finish
//! in one go; abilities and diseases last while held, other spells given
//! with `AddSpell` are cast once; nothing ends an effect early on death
//! but the death itself.

use esm::{FormId, LoadOrder};
use script::interp::Locals;

use crate::combat::av;
use crate::scripting::{Facts, GameState, Runner};

/// Magic effect archetypes (`MGEF` `DATA` u32 at 64) this module tells
/// apart (the game's list: 0 value modifier, 1 script, 2 dispel, 3 cure
/// disease, 24 paralysis, 33 concussion, 34 value and parts, 35 limb
/// condition, 36 turbo, …).
pub mod archetype {
    pub const VALUE_MODIFIER: u32 = 0;
    pub const SCRIPT: u32 = 1;
    pub const VALUE_AND_PARTS: u32 = 34;
    pub const LIMB_CONDITION: u32 = 35;
}

/// Actor values that count up as they're damaged: radiation (54), and
/// hardcore mode's dehydration, hunger and sleep deprivation (73–75).
pub const COUNTERS: [u16; 4] = [54, 73, 74, 75];

/// Spell types (`SPIT` u32 at 0) that last as long as they're held:
/// disease 1, ability 4, addiction 10.
const HELD_SPELL_TYPES: [u32; 3] = [1, 4, 10];

/// An effect working on someone.
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveEffect {
    pub target: FormId,
    /// The item or spell it came from.
    pub source: FormId,
    /// The magic effect (`MGEF`).
    pub effect: FormId,
    pub actor_value: i32,
    pub magnitude: f32,
    pub detrimental: bool,
    pub recover: bool,
    pub archetype: u32,
    pub resist: i32,
    pub script: Option<FormId>,
    /// Seconds left; infinite while an ability is held.
    pub remaining: f32,
    /// Script effects: whether `ScriptEffectStart` has run, and the
    /// script's variables.
    pub started: bool,
    pub locals: Locals,
    /// The body part condition (actor value 25 ..) a "value and parts" or
    /// "limb condition" effect is aimed at (the effect's `+0x4c`), set as
    /// it's added while the Pip-Boy's STATS healing mode aims at a limb
    /// (`00823210` → `00589f50`, [`GameState::healing_part`]); -1 none.
    pub part: i32,
    /// Who cast it (`pCaster`, Xbox PDB, `+0x28`), when known.
    pub caster: Option<FormId>,
    /// Which of its source's effects it is (`pEffect`, Xbox PDB, `+0xc`):
    /// the index among the source's `EFID`s, when known.
    pub item: Option<usize>,
}

impl ActiveEffect {
    fn changes_value(&self) -> bool {
        matches!(
            self.archetype,
            archetype::VALUE_MODIFIER | archetype::VALUE_AND_PARTS
        ) && self.actor_value >= 0
    }

    fn sign(&self) -> f64 {
        if self.detrimental {
            -1.0
        } else {
            1.0
        }
    }
}

/// What an effect's recover modifiers add to an actor value now (Buffout's
/// +2 Strength while it lasts). `held_only`: only those that last while a
/// spell is held (abilities), for the "permanent" value.
pub fn modifier(state: &GameState, who: FormId, value: u16, held_only: bool) -> f64 {
    state
        .active_effects
        .iter()
        .filter(|e| {
            e.target == who
                && e.recover
                && e.changes_value()
                && e.actor_value == i32::from(value)
                && (!held_only || e.remaining.is_infinite())
        })
        .map(|e| e.sign() * f64::from(e.magnitude))
        .sum()
}

/// What's working on someone, for the player's screen: one line per effect
/// on a value, with its item or spell and the time left ("Buffout: +2
/// Strength, 230 s left").
pub fn summary(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<String> {
    let name = |f: FormId| {
        order
            .get(f)
            .and_then(|r| r.record().ok())
            .and_then(|r| {
                r.full_name()
                    .filter(|n| !n.is_empty())
                    .or_else(|| r.editor_id())
            })
            .unwrap_or_else(|| f.to_string())
    };
    state
        .active_effects
        .iter()
        .filter(|e| e.target == who && e.changes_value())
        .map(|e| {
            let v = e.actor_value as u16;
            let up = !e.detrimental != COUNTERS.contains(&v);
            let per = if e.recover { "" } else { " a second" };
            let left = if e.remaining.is_finite() {
                format!(", {:.0} s left", e.remaining.ceil())
            } else {
                String::new()
            };
            format!(
                "{}: {}{} {}{per}{left}",
                name(e.source),
                if up { "+" } else { "-" },
                e.magnitude,
                crate::chargen::actor_value_name(order, v)
            )
        })
        .collect()
}

/// Whether someone is under a magic effect (`HasMagicEffect`).
pub fn has_effect(state: &GameState, who: FormId, effect: FormId) -> bool {
    state
        .active_effects
        .iter()
        .any(|e| e.target == who && e.effect == effect)
}

/// Whether someone is under an item's or spell's effects (`IsSpellTarget`).
pub fn is_target_of(state: &GameState, who: FormId, source: FormId) -> bool {
    state
        .active_effects
        .iter()
        .any(|e| e.target == who && e.source == source)
}

/// A value changes for good: positive restores, negative damages. Health
/// goes through [`crate::combat`] (damage can kill); the counters
/// ([`COUNTERS`]) go up when damaged; other values are lowered by damage
/// and restored up to what they were. A counter's change then goes through
/// its stages (radiation sickness, hardcore's needs:
/// `world::living::needs::changed`, the game's actor-value callbacks).
/// Whether it killed them.
pub fn change(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    value: u16,
    amount: f64,
    by: FormId,
) -> bool {
    if value == av::HEALTH {
        if amount < 0.0 {
            let died = crate::combat::hurt(order, state, who, -amount, by);
            // Effects and scripts' damage aren't hits (no `008ae000` push).
            crate::combat::not_a_hit(state, who);
            return died;
        }
        let lost = state.damage.entry(who).or_insert(0.0);
        *lost = (*lost - amount).max(0.0);
        return false;
    }
    let counter = COUNTERS.contains(&value);
    let before = counter.then(|| crate::living::needs::value(order, state, who, value));
    let damage = state.value_damage.entry((who, value)).or_insert(0.0);
    *damage = (*damage - amount).max(0.0);
    if *damage == 0.0 {
        state.value_damage.remove(&(who, value));
    }
    if let Some(old) = before {
        let new = crate::living::needs::value(order, state, who, value);
        if new != old {
            crate::living::needs::changed(order, state, who, value, old, new);
        }
    }
    false
}

/// An effect changes its value by `amount`. A "value and parts" one
/// (archetype 34, the Stimpak's "Restore Health & Conditions"): aimed at
/// no part, its value by `amount` and every body part condition (actor
/// values 25–31) by `amount` × `fMagicVACNoPartTargetedMult`; aimed at
/// one (the Pip-Boy's healing mode), that part by `amount` and its value
/// by `amount` × `fMagicVACPartTargetedMult` (both 0.1, the exe's
/// defaults). Translated from 0082b970 (decompiled, FalloutNV.exe
/// 1.4.0.525). Whether it killed them.
fn change_with_parts(
    order: &LoadOrder,
    state: &mut GameState,
    effect: &ActiveEffect,
    amount: f64,
    by: FormId,
) -> bool {
    let setting =
        |name: &str| f64::from(crate::scripting::game_setting(order, name).unwrap_or(0.1));
    let own = effect.actor_value as u16;
    if effect.archetype != archetype::VALUE_AND_PARTS {
        return change(order, state, effect.target, own, amount, by);
    }
    if effect.part >= 0 {
        let share = setting("fMagicVACPartTargetedMult");
        change(order, state, effect.target, effect.part as u16, amount, by);
        return change(order, state, effect.target, own, amount * share, by);
    }
    let died = change(order, state, effect.target, own, amount, by);
    let share = setting("fMagicVACNoPartTargetedMult");
    for part in crate::body_parts::av::FIRST_CONDITION..=crate::body_parts::av::LAST_CONDITION {
        change(order, state, effect.target, part, amount * share, by);
    }
    died
}

/// Applies an item's or spell's effects to `target` (cast by `caster`):
/// each effect whose conditions pass. `held`: an ability's, lasting as long
/// as it's held. Says what each did, for messages ("+2 Strength for
/// 240 s").
pub fn apply(
    order: &LoadOrder,
    state: &mut GameState,
    target: FormId,
    source: FormId,
    caster: FormId,
    held: bool,
) -> Vec<String> {
    let mut done = Vec::new();
    let ingestible = crate::items::ingestible_flags(order, source);
    let addiction = spell_type(order, source) == Some(ADDICTION_SPELL_TYPE);
    for (item, e) in crate::items::effects(order, source).into_iter().enumerate() {
        let applies = Facts {
            order,
            state,
            speaker: None,
        }
        .conditions_pass(&e.conditions, target, caster);
        if !applies {
            continue;
        }
        let magnitude = ingestible_magnitude(order, state, target, ingestible, &e);
        let mut duration = e.duration.max(0) as f32;
        if addiction {
            // An addiction's withdrawal lasts as the target's perks'
            // "Modify Addiction Duration" (entry point 24) say
            // (`00823210`; no perk in the game's data uses it).
            duration = crate::perks::apply_for(
                order,
                state,
                target,
                crate::perks::entry::MODIFY_ADDICTION_DURATION,
                duration,
                &[],
            );
        } else if ingestible.is_some() && !e.hostile {
            // A food's or chem's benefit lasts as the target's perks'
            // "Modify Positive Chem Duration" (25: Chemist × 2, Day
            // Tripper × 1.33) say; hostile effects (`MGEF` flag 0x01) as
            // long as ever (`00823210`).
            duration = crate::perks::apply_for(
                order,
                state,
                target,
                crate::perks::entry::MODIFY_POSITIVE_CHEM_DURATION,
                duration,
                &[],
            );
        }
        let mut active = ActiveEffect {
            target,
            source,
            effect: e.effect,
            actor_value: e.actor_value,
            magnitude,
            detrimental: e.harmful,
            recover: e.recover,
            archetype: e.archetype,
            resist: e.resist,
            script: e.script,
            remaining: if held { f32::INFINITY } else { duration },
            started: false,
            locals: Locals::default(),
            // Aimed at the limb the Pip-Boy's healing mode targets
            // (`00823210`: archetypes 0x22 and 0x23).
            part: match e.archetype {
                archetype::VALUE_AND_PARTS | archetype::LIMB_CONDITION => {
                    state.healing_part.map_or(-1, i32::from)
                }
                _ => -1,
            },
            caster: Some(caster),
            item: Some(item),
        };
        if active.detrimental && active.resist >= 0 {
            let resisted = Facts {
                order,
                state,
                speaker: None,
            }
            .current_actor_value(target, active.resist as u16)
            .unwrap_or(0.0);
            active.magnitude *= (1.0 - (resisted as f32 / 100.0)).clamp(0.0, 1.0);
        }
        let value_name = (active.actor_value >= 0)
            .then(|| crate::chargen::actor_value_name(order, active.actor_value as u16));
        // Rads go up when damaged.
        let counter = active.actor_value >= 0 && COUNTERS.contains(&(active.actor_value as u16));
        let amount = format!(
            "{}{}",
            if active.detrimental != counter {
                "-"
            } else {
                "+"
            },
            active.magnitude
        );
        match (active.changes_value(), value_name) {
            // At once.
            (true, Some(name)) if active.remaining == 0.0 => {
                if !active.recover {
                    let amount_now = active.sign() * f64::from(active.magnitude);
                    change_with_parts(order, state, &active, amount_now, caster);
                    done.push(format!("{amount} {name}"));
                }
                continue;
            }
            (true, Some(name)) => {
                let per = if active.recover { "" } else { " a second" };
                if active.remaining.is_finite() {
                    done.push(format!("{amount} {name}{per} for {} s", active.remaining));
                } else {
                    done.push(format!("{amount} {name}{per}"));
                }
            }
            _ => {}
        }
        state.active_effects.push(active);
    }
    done
}

/// The spell type an addiction is (`SPIT` u32 at 0).
const ADDICTION_SPELL_TYPE: u32 = 10;

/// A spell's type (`SPIT` u32 at 0: 0 actor effect, 1 disease, 4 ability,
/// 10 addiction); `None` for anything but a spell.
fn spell_type(order: &LoadOrder, spell: FormId) -> Option<u32> {
    let rr = order
        .get(spell)
        .filter(|r| r.entry.header.kind.as_bytes() == b"SPEL")?;
    let d = rr
        .record()
        .ok()?
        .get(esm::FourCC::new(b"SPIT"))
        .filter(|s| s.data.len() >= 4)?
        .data
        .clone();
    Some(crate::cell::le_u32(&d, 0))
}

/// An aid item's effect magnitude as the game sets it up (`00815d00`,
/// read from the code): a **medicine** (`ENIT` flag 0x04, the Stimpak)
/// restores its magnitude × (`fMagicMedicineSkillBase` + the player's
/// Medicine ÷ 100 × `fMagicMedicineSkillMult` 2) on every effect; a
/// **food** (0x02) its magnitude × (`fMagicSurvivalSkillBase` + Survival
/// ÷ 100 × `fMagicSurvivalSkillMult` 2) on every effect but radiation
/// (actor value 54), whose magnitude goes through the eater's perks'
/// "Modify Radiation Consumed" (entry point 44: Lead Belly × 0.5) instead;
/// either kind's Health effects (16) also through the perks' "Modify
/// Recovered Health" (12: Better Healing × 1.2) when the one healed is the
/// player. Neither flag: the magnitude as written. The skill is the
/// player's whoever eats (`0066ef50`).
fn ingestible_magnitude(
    order: &LoadOrder,
    state: &GameState,
    target: FormId,
    flags: Option<u8>,
    e: &crate::items::ItemEffect,
) -> f32 {
    use crate::items::ingestible::{FOOD, MEDICINE};
    let base = e.magnitude as f32;
    let Some(flags) = flags else {
        return base;
    };
    let setting = |n: &str, d: f32| crate::scripting::game_setting(order, n).unwrap_or(d);
    let player_skill = |skill: u16| {
        Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(crate::dialogue::PLAYER_REF, skill)
        .unwrap_or(0.0) as f32
    };
    let recovered_health = |m: f32| {
        if target == crate::dialogue::PLAYER_REF && e.actor_value == i32::from(av::HEALTH) {
            crate::perks::apply_for(
                order,
                state,
                target,
                crate::perks::entry::MODIFY_RECOVERED_HEALTH,
                m,
                &[],
            )
        } else {
            m
        }
    };
    let mut magnitude = base;
    if flags & MEDICINE != 0 {
        let medicine = setting("fMagicMedicineSkillBase", MEDICINE_SKILL_BASE)
            + player_skill(37) / 100.0 * setting("fMagicMedicineSkillMult", 2.0);
        magnitude = recovered_health(base * medicine);
    }
    if flags & FOOD != 0 {
        if e.actor_value == i32::from(RADIATION) {
            magnitude = base
                * crate::perks::apply_for(
                    order,
                    state,
                    target,
                    crate::perks::entry::MODIFY_RADIATION_CONSUMED,
                    1.0,
                    &[],
                );
        } else {
            let survival = setting("fMagicSurvivalSkillBase", SURVIVAL_SKILL_BASE)
                + player_skill(44) / 100.0 * setting("fMagicSurvivalSkillMult", 2.0);
            magnitude = recovered_health(base * survival);
        }
    }
    magnitude
}

/// The radiation actor value (`RadiationRads`).
const RADIATION: u16 = 54;

/// The exe's defaults for `fMagicMedicineSkillBase` and
/// `fMagicSurvivalSkillBase` (the data doesn't set them).
const MEDICINE_SKILL_BASE: f32 = 1.0;
const SURVIVAL_SKILL_BASE: f32 = 1.0;

/// A spell is cast on someone (`CastImmediateOnSelf`, `Cast`) or given to
/// them (`AddSpell`): abilities, diseases and addictions given last while
/// held; anything else takes effect as cast.
pub fn add_spell(
    order: &LoadOrder,
    state: &mut GameState,
    target: FormId,
    spell: FormId,
    caster: FormId,
    given: bool,
) -> Vec<String> {
    let held = given
        && order
            .get(spell)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.get(esm::FourCC::new(b"SPIT")).map(|s| s.data.clone()))
            .is_some_and(|d| {
                d.len() >= 4 && HELD_SPELL_TYPES.contains(&crate::cell::le_u32(&d, 0))
            });
    if held && is_target_of(state, target, spell) {
        return Vec::new();
    }
    apply(order, state, target, spell, caster, held)
}

/// The largest area (`EFIT` u32 at 4) of a spell's touch-range effects
/// (range, u32 at 12, 1), as `00818ce0` picks it; 0 when none has one.
fn touch_area(order: &LoadOrder, spell: FormId) -> u32 {
    let Some(record) = order.get(spell).and_then(|r| r.record().ok()) else {
        return 0;
    };
    record
        .get_all(esm::FourCC::new(b"EFIT"))
        .filter(|s| s.data.len() >= 16)
        .filter(|s| crate::cell::le_u32(&s.data, 12) == 1)
        .map(|s| crate::cell::le_u32(&s.data, 4))
        .max()
        .unwrap_or(0)
}

/// Whom a cast at `target` reaches besides it (`MagicCaster::FindTargets`
/// `00815d00`, its gathering `00818ce0`, Xbox PDB): with an area on the
/// spell's touch-range effects, the actors loaded around (here: placed in
/// the player's cell, and the player) other than the caster, with 3D (not
/// disabled), not ghosts (`008ace90`), within `fMagicUnitsPerFoot` (22) ×
/// that area of the target, at the effectiveness 1 a script's cast has;
/// unless the spell's flag 0x10 (`SPIT` flags byte, "area effect ignores
/// LOS", `0040e210(0x10)`) says otherwise, each needs a line of sight
/// (`008190d0`: a ray on layer 0x27 from the target's position to theirs
/// raised by half their height, `008853a0`, that leaves out their own
/// collision and meets nothing). Without the viewer's [`Sight`] (headless)
/// the line of sight isn't tested. The target itself is cast on by the
/// caller and left out here. The disguise pulse's area 25 reaches 550
/// units.
///
/// [`Sight`]: crate::sight::Sight
// Translated from 00818ce0 and 008190d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn area_targets(
    order: &LoadOrder,
    state: &GameState,
    spell: FormId,
    caster: FormId,
    target: FormId,
    sight: Option<&dyn crate::sight::Sight>,
) -> Vec<FormId> {
    let area = touch_area(order, spell);
    if area == 0 {
        return Vec::new();
    }
    let feet = crate::scripting::game_setting(order, "fMagicUnitsPerFoot").unwrap_or(22.0);
    let radius = (feet * 1.0).trunc() * area as f32;
    let (Some(cell), Some((space, _, at, _))) = (state.player_cell, state.place(order, target))
    else {
        return Vec::new();
    };
    let ignores_los = spell_flags(order, spell) & AREA_IGNORES_LOS != 0;
    let mut out: Vec<FormId> = order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| matches!(rr.entry.header.kind.as_bytes(), b"ACHR" | b"ACRE"))
        .map(|rr| rr.form_id)
        .collect();
    // The player last (`00818ce0` asks them after the process lists).
    out.push(crate::dialogue::PLAYER_REF);
    out.retain(|&w| {
        if w == caster || w == target || state.disabled.get(&w) == Some(&true) {
            return false;
        }
        if crate::more_functions::is_ghost(state, w) {
            return false;
        }
        let Some((s, _, p, _)) = state.place(order, w) else {
            return false;
        };
        if s != space || (0..3).map(|i| (p[i] - at[i]).powi(2)).sum::<f32>().sqrt() > radius {
            return false;
        }
        if ignores_los {
            return true;
        }
        let Some(sight) = sight else {
            return true;
        };
        let base = crate::scripting::base_of(order, w).unwrap_or(w);
        let scale = order
            .get(w)
            .and_then(|rr| rr.record().ok())
            .and_then(|r| r.get(esm::FourCC::new(b"XSCL")).map(|s| s.data.clone()))
            .filter(|d| d.len() >= 4)
            .map_or(1.0, |d| crate::cell::le_f32(&d, 0));
        let height = crate::npc_aim::actor_height(order, base, scale).unwrap_or(0.0);
        sight.ray(at, [p[0], p[1], p[2] + height * 0.5]).is_none()
    });
    out
}

/// `SPIT` flags (byte 12; the spell's `+0x40`): 0x10, area effects ignore
/// the line of sight (`00818ce0`).
const AREA_IGNORES_LOS: u8 = 0x10;

fn spell_flags(order: &LoadOrder, spell: FormId) -> u8 {
    order
        .get(spell)
        .filter(|r| r.entry.header.kind.as_bytes() == b"SPEL")
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::FourCC::new(b"SPIT")).map(|s| s.data.clone()))
        .filter(|d| d.len() >= 13)
        .map_or(0, |d| d[12])
}
/// Takes an item's or spell's effects off someone (`RemoveSpell`,
/// `Dispel`): what was taken off (script effects still to finish).
pub fn remove(state: &mut GameState, target: FormId, source: FormId) -> Vec<ActiveEffect> {
    let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut state.active_effects)
        .into_iter()
        .partition(|e| e.target == target && e.source == source);
    state.active_effects = kept;
    gone
}

/// A source's effect items as `004042a0` compares them: each `EFID`, its
/// `EFIT` (magnitude, area, duration, range, actor value) and its
/// conditions (`CTDA`s).
fn effect_items(order: &LoadOrder, source: FormId) -> Vec<(u32, Vec<u8>, Vec<Vec<u8>>)> {
    let Some(record) = order.get(source).and_then(|r| r.record().ok()) else {
        return Vec::new();
    };
    let mut out: Vec<(u32, Vec<u8>, Vec<Vec<u8>>)> = Vec::new();
    for sub in &record.subrecords {
        match sub.kind.as_bytes() {
            b"EFID" if sub.data.len() >= 4 => {
                out.push((crate::cell::le_u32(&sub.data, 0), Vec::new(), Vec::new()))
            }
            b"EFIT" => {
                if let Some(last) = out.last_mut() {
                    last.1 = sub.data.clone();
                }
            }
            b"CTDA" => {
                if let Some(last) = out.last_mut() {
                    last.2.push(sub.data.clone());
                }
            }
            _ => {}
        }
    }
    out
}

/// A script's cast of a spell on `target` (`MagicCaster::CastSpellImmediate`
/// → `MagicTarget::CheckAddEffect` (Xbox PDB), `00823210`, for each effect
/// it adds): what was dispelled to make way (their `ScriptEffectFinish`
/// is the caller's to run). By the spell's type (`MagicSystem::SpellType`,
/// Xbox PDB; `SPIT`):
/// - a poison (5) with a duration: an identical effect already working
///   from the same spell and caster (`00824c00`) gets the new duration
///   added to its own and the new one isn't added;
/// - wortcraft (8): added as it is;
/// - anything else (actor effect, disease, power, lesser power, ability,
///   leveled, addiction), unless the effect comes from a worn enchantment
///   (flag 0x100, never for a cast): `MagicTarget::Dispel` (`00824400`)
///   with the spell, the caster and the effect item ends the first effect
///   working from the same spell and caster on the same magic effect with
///   an identical effect item (`004042a0`: its `EFIT` data and
///   conditions), then the new one is added.
///
/// Not translated here: potions' and enchantments' own branches (an
/// `ALCH` or `ENCH` source is added as before), and the "Usage Monitor
/// Effect" (`0x14F`, `00408f60`) branch for chems' addictions.
// Translated from 00823210 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn cast(
    order: &LoadOrder,
    state: &mut GameState,
    target: FormId,
    spell: FormId,
    caster: FormId,
) -> Vec<ActiveEffect> {
    let before = state.active_effects.len();
    add_spell(order, state, target, spell, caster, false);
    let Some(kind) = spell_type(order, spell) else {
        return Vec::new();
    };
    let items = effect_items(order, spell);
    let same_item = |a: Option<usize>, b: Option<usize>| match (a, b) {
        (Some(a), Some(b)) => items.get(a).is_some() && items.get(a) == items.get(b),
        _ => false,
    };
    let added: Vec<ActiveEffect> = state.active_effects.drain(before..).collect();
    let mut dispelled = Vec::new();
    for new in added {
        let matches = |e: &ActiveEffect| {
            e.target == new.target
                && e.source == spell
                && e.caster == new.caster
                && e.effect == new.effect
                && same_item(e.item, new.item)
        };
        match kind {
            POISON_SPELL_TYPE if new.remaining > 0.0 => {
                if let Some(e) = state.active_effects.iter_mut().find(|e| matches(e)) {
                    e.remaining += new.remaining;
                    continue;
                }
            }
            POISON_SPELL_TYPE | WORTCRAFT_SPELL_TYPE => {}
            _ => {
                if let Some(i) = state.active_effects.iter().position(matches) {
                    dispelled.push(state.active_effects.remove(i));
                }
            }
        }
        state.active_effects.push(new);
    }
    dispelled
}

/// `MagicSystem::SpellType` (Xbox PDB) 5, poison, and 8, wortcraft.
const POISON_SPELL_TYPE: u32 = 5;
const WORTCRAFT_SPELL_TYPE: u32 = 8;
/// `MGEF` `DATA` flag 0x10000000 ("No Death Dispel", xEdit's name): the
/// effect keeps working on someone dead (`00804560`).
pub const NO_DEATH_DISPEL: u32 = 0x1000_0000;

/// Whether a magic effect keeps working after its target dies (its `MGEF`
/// flags have [`NO_DEATH_DISPEL`]: the energy weapons' critical effects,
/// `LaserDisintegrationEffect` and `GooificationEffect`, 0x10000475).
pub fn survives_death(order: &LoadOrder, effect: FormId) -> bool {
    order
        .get(effect)
        .filter(|r| r.entry.header.kind.as_bytes() == b"MGEF")
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
        .filter(|d| d.len() >= 4)
        .is_some_and(|d| crate::cell::le_u32(&d, 0) & NO_DEATH_DISPEL != 0)
}

/// Time passes for every effect: changes over time are applied, script
/// effects run, and what has run its course ends. The dead lose theirs
/// (a script effect's `ScriptEffectFinish` runs) unless the magic effect
/// survives death ([`survives_death`]): those go on as on the living.
// Translated from 00804560 (decompiled, FalloutNV.exe 1.4.0.525;
// `ActiveEffect`'s update): after the frame's update, a target actor that
// `IsDead` with the effect's flag 0x10000000 clear ends the effect (+0x13),
// and an effect that started then finishes (vtable +0x58).
pub fn tick(runner: &mut Runner, seconds: f32) {
    let order = runner.order;
    let active = std::mem::take(&mut runner.state.active_effects);
    let mut kept = Vec::with_capacity(active.len());
    for mut e in active {
        if runner.state.dead.contains(&e.target) && !survives_death(order, e.effect) {
            if let (true, Some(script)) = (e.started, e.script) {
                runner.run_effect_script(script, &mut e, "scripteffectfinish", seconds);
            }
            continue;
        }
        let step = seconds.min(e.remaining.max(0.0));
        if e.changes_value() && !e.recover && step > 0.0 {
            let amount = e.sign() * f64::from(e.magnitude) * f64::from(step);
            if change_with_parts(order, runner.state, &e, amount, e.target) {
                runner.run_event(e.target, "ondeath", e.target);
                continue;
            }
        }
        if let Some(script) = e.script {
            if !e.started {
                e.started = true;
                runner.run_effect_script(script, &mut e, "scripteffectstart", seconds);
            }
            runner.run_effect_script(script, &mut e, "scripteffectupdate", seconds);
        }
        e.remaining -= seconds;
        if e.remaining > 0.0 {
            kept.push(e);
        } else if let Some(script) = e.script {
            runner.run_effect_script(script, &mut e, "scripteffectfinish", seconds);
        }
    }
    // Effects scripts added meanwhile come after.
    kept.append(&mut runner.state.active_effects);
    runner.state.active_effects = kept;
}
