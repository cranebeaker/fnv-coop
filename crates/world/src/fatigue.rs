//! Fatigue (actor value 22) and knock-outs, read from the game's code
//! (FalloutNV.exe 1.4.0.525; Xbox 360 prototype names where marked).
//!
//! **Full fatigue** ([`base_fatigue`], `Actor::GetBaseActorValue`
//! `008803a0` with the value's flags 0x1841): a creature's is its record's
//! (`ACBS` u16 at 4, × its level when the record's level is a multiple of
//! the player's, `TESCreature::GetFatigue` (Xbox PDB)); a person's is the
//! record's plus a derived part (`00643870`, `AiFormulas::
//! CalculateDerivedFatigue` (Xbox PDB)): the player `fAVDFatigueBase` 90 +
//! `fAVDFatigueEnduranceMult` 20 × Endurance + `fAVDFatigueLevelMult` 10 ×
//! (level − 1) (record 200: 390 with Endurance 5 at level 1), other people
//! `fAVDNPCFatigueBase` 90 + `fAVDNPCFatigueEnduranceMult` 20 × Endurance
//! (record 50: 240 with Endurance 5), and people whose stats are worked out
//! by the game (`ACBS` flag 0x10) the record's + 20 × the record's
//! Endurance + `fAVDNPCFatigueLevelMult` 10 × level, cut to a whole number
//! (`006040d0`, `TESNPC::GetFormFatigueLeveled` (Xbox PDB); 0 when the
//! record's is 0), added to the record's again.
//!
//! **Damage.** Bare fists (no weapon, someone not a creature striking
//! someone not knocked down) do fatigue damage of half their health
//! damage (`00646310`, from `009b5170`); a weapon's ammunition effect of
//! kind 5 adds to it (`009b5a30`: the bean bag's +250). It goes through the
//! armour in the same share as the health damage, is scaled with the rest
//! of the hit when the player is struck in V.A.T.S., and takes the hit's
//! last multiplier when it was an unarmed blow (`009b73d0`). A hit doing
//! fatigue damage wears no armour (`009b5a30`). The hit takes it only while
//! fatigue is above `fMinimumFatigue` (−25; `0089d6f0`,
//! `DamageHealthAndFatigue`), after the health. Scripts damage it as any
//! value (`DamageAV Fatigue 50`: the boxing gloves' and cattle prod's hit
//! scripts, the gas grenade's `10000`). (`fHandFatigueDamageBase`/`Mult`
//! are read by no code.)
//!
//! **Coming back** ([`regenerate`], `0088b5a0`, `Actor::RestoreFatigue`
//! (Xbox PDB), from the actors' magic update `008c3c40` while the value is
//! damaged (the health/fatigue callback `008b9960` flags it) or below 0):
//! `fFatigueReturnBase` 1 + `fFatigueReturnMult` 0 × 10 × a cached value a
//! second, only while damaged (`0088b740`); the moment it climbs from below
//! 0 to 0 or more, all the damage goes at once (+1).
//!
//! **Knock-outs** ([`advance`], `MiddleHighProcess::UpdateKnockState`
//! (Xbox PDB), `00920150`; the process's knock state +0x13c, named by
//! `0118c6d4`: 0 normal, 1 explode, 2 explode lead-in, 3 knocked out, 4
//! knock-out lead-in, 5 queued, 6 getting up): someone who can be knocked
//! down (`008845a0`: alive, the record without `ACBS` flag 0x4000000) goes
//! down when their fatigue is damaged below 0, or paralysed (actor value
//! 47 above 0: straight to knocked out, stiff), or essential and down
//! (life state 6, `world::combat::hurt`); the lead-in (the fall, until the
//! ragdoll stops) becomes knocked out; they lie there while any of the
//! three holds, then the get-up animation is queued and played (5, 6) and
//! they're back to normal. Lying there blocks using them (the "is
//! unconscious." message, `005fa330`, `world::living::pickpocket`), their
//! AI (the viewer) and fists' fatigue damage; `GetKnockedState` is 1 for
//! knocked out and the lead-in (`005a08c0`). Here the lead-in ends at the
//! next update and the get-up at the one after: the fall and the get-up
//! animations are the viewer's.

use std::collections::HashMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, Event, Facts, GameState};

const ACBS: FourCC = FourCC::new(b"ACBS");
const CREA: FourCC = FourCC::new(b"CREA");

/// Fatigue's actor value.
pub const FATIGUE: u16 = 22;
/// Paralysis's actor value.
pub const PARALYSIS: u16 = 47;
/// `ACBS` flag: the record's stats are worked out by the game.
const AUTO_CALC: u32 = 0x10;
/// `ACBS` flag: the record's level is a multiple of the player's.
const PC_LEVEL_MULT: u32 = 0x80;
/// `ACBS` flag tested by `008845a0` (`00884690`): never knocked down.
const NO_KNOCKDOWN: u32 = 0x0400_0000;

/// The process's knock states (`0118c6d4`).
pub mod knock {
    pub const NORMAL: u8 = 0;
    pub const EXPLODE: u8 = 1;
    pub const EXPLODE_LEAD_IN: u8 = 2;
    pub const KNOCKED_OUT: u8 = 3;
    pub const KNOCK_OUT_LEAD_IN: u8 = 4;
    pub const QUEUED: u8 = 5;
    pub const GETTING_UP: u8 = 6;
}

/// The knock states of those not in state 0, kept with the game state.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Knocks {
    pub state: HashMap<FormId, u8>,
}

fn setting(order: &LoadOrder, name: &str, default: f32) -> f64 {
    f64::from(game_setting(order, name).unwrap_or(default))
}

/// The record's `ACBS`: flags, fatigue.
fn acbs(order: &LoadOrder, state: &GameState, who: FormId) -> Option<(bool, u32, u16)> {
    let base = crate::more_functions::placed::base_now(order, state, who)?;
    let (rr, record) = crate::actor::data_record(order, base, crate::actor::USE_STATS)?;
    let a = record.get(ACBS).filter(|s| s.data.len() >= 6)?;
    let creature = rr.entry.header.kind == CREA;
    Some((
        creature,
        le_u32(&a.data, 0),
        u16::from_le_bytes([a.data[4], a.data[5]]),
    ))
}

/// Someone's full fatigue before effects and damage (see the module
/// notes): the record's, and for people the derived part.
// Translated from 008803a0, 00643870 and 006040d0 (decompiled,
// FalloutNV.exe 1.4.0.525).
pub fn base_fatigue(facts: &Facts, who: FormId) -> Option<f64> {
    let order = facts.order;
    let (creature, flags, record) = acbs(order, facts.state, who)?;
    let record = f64::from(record);
    let level = || f64::from(facts.level_of(who).unwrap_or(1).max(1));
    if creature {
        return Some(if flags & PC_LEVEL_MULT != 0 {
            record * level()
        } else {
            record
        });
    }
    let derived = if who != PLAYER_REF && flags & AUTO_CALC != 0 {
        if record == 0.0 {
            0.0
        } else {
            let endurance = facts
                .base_actor_value(who, crate::combat::av::ENDURANCE)
                .unwrap_or(0.0);
            (record
                + endurance * setting(order, "fAVDNPCFatigueEnduranceMult", 20.0)
                + level() * setting(order, "fAVDNPCFatigueLevelMult", 10.0))
            .trunc()
            .max(0.0)
        }
    } else {
        let endurance = facts
            .permanent_actor_value(who, crate::combat::av::ENDURANCE)
            .unwrap_or(0.0);
        if who == PLAYER_REF {
            setting(order, "fAVDFatigueBase", 90.0)
                + endurance * setting(order, "fAVDFatigueEnduranceMult", 20.0)
                + setting(order, "fAVDFatigueLevelMult", 10.0)
                    * (f64::from(facts.state.player_level.max(1)) - 1.0)
        } else {
            setting(order, "fAVDNPCFatigueBase", 90.0)
                + endurance * setting(order, "fAVDNPCFatigueEnduranceMult", 20.0)
        }
    };
    Some(record + derived)
}

fn facts<'a>(order: &'a LoadOrder, state: &'a GameState) -> Facts<'a> {
    Facts {
        order,
        state,
        speaker: None,
    }
}

/// Someone's fatigue now.
pub fn fatigue(order: &LoadOrder, state: &GameState, who: FormId) -> Option<f64> {
    facts(order, state).current_actor_value(who, FATIGUE)
}

/// What fatigue damage someone has taken.
pub fn damage_taken(state: &GameState, who: FormId) -> f64 {
    state
        .value_damage
        .get(&(who, FATIGUE))
        .copied()
        .unwrap_or(0.0)
}

/// `GetFatiguePercentage` (`00893530`): fatigue now ÷ the full value as
/// a whole number; 1 when that's 0.
pub fn percentage(order: &LoadOrder, state: &GameState, who: FormId) -> Option<f64> {
    let facts = facts(order, state);
    let full = facts.permanent_actor_value(who, FATIGUE)?.trunc();
    if full == 0.0 {
        return Some(1.0);
    }
    Some(facts.current_actor_value(who, FATIGUE)? / full)
}

/// Whether someone is in a knock state other than normal (actor vtable
/// +0x230, `00884560`), or essential and down (life state 6, whose actors
/// the game knocks out too).
pub fn knocked(state: &GameState, who: FormId) -> bool {
    knock_state(state, who) != knock::NORMAL
}

/// Someone's knock state: essential people who are down lie knocked out.
pub fn knock_state(state: &GameState, who: FormId) -> u8 {
    match state.knocks.state.get(&who) {
        Some(&s) => s,
        None if state.more.down.contains_key(&who) => knock::KNOCKED_OUT,
        None => knock::NORMAL,
    }
}

/// Whether someone lies on the ground now (falling or knocked out), for
/// the viewer's limp body.
pub fn lies_down(state: &GameState, who: FormId) -> bool {
    matches!(
        knock_state(state, who),
        knock::KNOCKED_OUT | knock::KNOCK_OUT_LEAD_IN
    )
}

/// `GetKnockedState` (`005a08c0`): 1 knocked out or falling to it, else 0.
pub fn knocked_state_value(state: &GameState, who: FormId) -> f64 {
    match knock_state(state, who) {
        knock::KNOCKED_OUT | knock::KNOCK_OUT_LEAD_IN => 1.0,
        _ => 0.0,
    }
}

/// The fatigue damage of a bare-fisted blow before the armour (`00646310`):
/// half the fists' health damage, none when the attacker is a creature or
/// the one struck is knocked down (actor vtable +0x230, `009b5170`).
pub fn fists_fatigue(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    target: FormId,
    fists_damage: f32,
) -> f32 {
    if crate::combat::is_creature(order, attacker) || knocked(state, target) {
        return 0.0;
    }
    fists_damage * 0.5
}

/// A hit's fatigue damage after the armour (`009b5a30`): with a weapon,
/// the ammunition's fatigue effects (kind 5) applied to it; then, when
/// above 0 and the hit had damage before the armour, × the share of the
/// damage the armour let through.
pub fn through_armour(
    order: &LoadOrder,
    fatigue: f32,
    ammo: Option<FormId>,
    with_weapon: bool,
    before: f32,
    after: f32,
) -> f32 {
    let mut fatigue = fatigue;
    if with_weapon {
        let effects = ammo
            .map(|a| crate::combat::ammo_effects(order, a))
            .unwrap_or_default();
        fatigue = crate::combat::with_ammo(&effects, 5, fatigue);
    }
    if fatigue > 0.0 && before > 0.0 {
        fatigue *= after / before;
    }
    fatigue
}

/// A hit's fatigue damage is taken (`0089d6f0`): only while fatigue is
/// above `fMinimumFatigue` (−25). Whether it was.
// Translated from 0089d6f0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn take_hit(order: &LoadOrder, state: &mut GameState, who: FormId, amount: f32) -> bool {
    if amount <= 0.0 || state.dead.contains(&who) {
        return false;
    }
    let least = setting(order, "fMinimumFatigue", -25.0);
    match fatigue(order, state, who) {
        Some(now) if now > least => {
            *state.value_damage.entry((who, FATIGUE)).or_insert(0.0) += f64::from(amount);
            true
        }
        _ => false,
    }
}

/// Fatigue comes back for `seconds` (`0088b5a0`; see the module notes).
// Translated from 0088b5a0, 00648050 and 0088b740 (decompiled,
// FalloutNV.exe 1.4.0.525).
pub fn regenerate(order: &LoadOrder, state: &mut GameState, who: FormId, seconds: f32) {
    // The cached value the rate multiplies (+0x24 of the actor's cached
    // values) isn't traced; `fFatigueReturnMult` is 0, so it adds nothing.
    let rate = setting(order, "fFatigueReturnBase", 1.0);
    let back = rate * f64::from(seconds);
    let Some(before) = fatigue(order, state, who) else {
        return;
    };
    let key = (who, FATIGUE);
    let Some(damage) = state.value_damage.get_mut(&key) else {
        return;
    };
    if back > 0.0 {
        *damage = (*damage - back).max(0.0);
    }
    if before < 0.0 && before + back >= 0.0 {
        *damage = 0.0;
    }
    if *damage == 0.0 {
        state.value_damage.remove(&key);
    }
}

/// Whether someone can be knocked down (`008845a0`): alive and their
/// record allows it.
fn can_knock_down(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    if state.dead.contains(&who) {
        return false;
    }
    !acbs(order, state, who).is_some_and(|(_, flags, _)| flags & NO_KNOCKDOWN != 0)
}

fn paralysed(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    facts(order, state)
        .current_actor_value(who, PARALYSIS)
        .is_some_and(|v| v > 0.0)
}

/// What keeps someone down: fatigue damaged below 0, paralysis, or being
/// essential and down.
fn kept_down(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    (damage_taken(state, who) > 0.0 && fatigue(order, state, who).is_some_and(|f| f < 0.0))
        || paralysed(order, state, who)
        || state.more.down.contains_key(&who)
}

/// One update of someone's knock state (`00920150`; see the module
/// notes). Knock-outs by fatigue or paralysis say so (`Event::KnockedOut`,
/// `Event::GotUp`); essential people's going down and getting up are
/// told by `world::combat`.
// Translated from 00920150 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn update_knock(order: &LoadOrder, state: &mut GameState, who: FormId) {
    if !can_knock_down(order, state, who) {
        state.knocks.state.remove(&who);
        return;
    }
    let essential_down = state.more.down.contains_key(&who);
    let now = state
        .knocks
        .state
        .get(&who)
        .copied()
        .unwrap_or(knock::NORMAL);
    let next = match now {
        knock::NORMAL if kept_down(order, state, who) => {
            if !essential_down {
                state.events.push(Event::KnockedOut { who });
            }
            if paralysed(order, state, who) {
                knock::KNOCKED_OUT
            } else {
                knock::KNOCK_OUT_LEAD_IN
            }
        }
        knock::KNOCK_OUT_LEAD_IN => knock::KNOCKED_OUT,
        knock::KNOCKED_OUT if !kept_down(order, state, who) => {
            state.events.push(Event::GotUp { who });
            knock::GETTING_UP
        }
        knock::QUEUED => knock::GETTING_UP,
        knock::GETTING_UP => knock::NORMAL,
        other => other,
    };
    // Essential people are told apart by `world::combat`'s timer; their
    // knock state stays implied ([`knock_state`]).
    if next == knock::NORMAL || (essential_down && now == knock::NORMAL) {
        state.knocks.state.remove(&who);
    } else {
        state.knocks.state.insert(who, next);
    }
}

/// A frame of `seconds`: fatigue comes back for everyone damaged and
/// alive, and the knock states move on.
pub fn advance(order: &LoadOrder, state: &mut GameState, seconds: f32) {
    let mut damaged: Vec<FormId> = state
        .value_damage
        .keys()
        .filter(|k| k.1 == FATIGUE)
        .map(|k| k.0)
        .collect();
    damaged.sort();
    for &who in &damaged {
        if !state.dead.contains(&who) {
            regenerate(order, state, who, seconds);
        }
    }
    let mut who_all: Vec<FormId> = damaged;
    who_all.extend(state.knocks.state.keys().copied());
    who_all.extend(
        state
            .active_effects
            .iter()
            .filter(|e| e.actor_value == i32::from(PARALYSIS))
            .map(|e| e.target),
    );
    // Actors set below 0 by scripts (`SetAV Fatigue -1`) without damage
    // don't count: the game's test needs the value flagged as damaged.
    who_all.sort();
    who_all.dedup();
    for who in who_all {
        update_knock(order, state, who);
    }
}
