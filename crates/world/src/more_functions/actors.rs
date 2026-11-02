//! Companions and actors: `PushActorAway`, `SetDisposition`, `DispelAllSpells` and `GetCauseofDeath` (read in [`super::value`], kept
//! here by [`record_cause`]). Notes: `docs/DEAD_MONEY.md` "Companions and
//! actors".

use std::collections::BTreeMap;

use esm::{FormId, LoadOrder};

use super::{is_actor, Shown};
use crate::combat::Weapon;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, Event, Facts, GameState, Runner, Value};

/// The functions here that change things, by the game's own names.
pub const FUNCTIONS: &[&str] = &["PushActorAway", "SetDisposition", "DispelAllSpells"];

/// Agility's actor value number.
const AGILITY: u16 = 10;

/// What people's dispositions toward the player scripts changed, by actor:
/// the list the actor keeps (actor +0xfc, a change flag 0x80000) of
/// (amount, toward whom), which `ModDisposition` (`0087fb40`) only ever
/// writes for the player.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dispositions(pub BTreeMap<FormId, i32>);

/// Carries out one of [`FUNCTIONS`]; `on` is the reference the script runs
/// on (or names).
pub(super) fn carry_out(
    runner: &mut Runner,
    name: &str,
    on: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let order = runner.order;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    match name {
        // `005d6b60`: the caller pushes the actor given away from itself.
        "PushActorAway" => {
            let from = on?;
            let who = arg(0).form();
            if !is_actor(order, runner.state, who) {
                println!(
                    "SCRIPTS: PushActorAway in script on '{from}' is attempting to push a \
                     non-actor reference."
                );
                return Some(1.0);
            }
            let agility = Facts {
                order,
                state: runner.state,
                speaker: None,
            }
            .current_actor_value(who, AGILITY)
            .unwrap_or(0.0);
            let force = push_force(order, agility, arg(1).number() as i32);
            // Only someone whose AI process is the high one (process
            // +0x28 is 0): here, someone with 3D loaded.
            if runner.state.more.loaded.contains(&who) {
                runner
                    .state
                    .events
                    .push(Event::More(Shown::PushedAway { who, from, force }));
            }
            Some(1.0)
        }
        // `005d54a0`: on a person or creature, their disposition toward the
        // actor given moves to the number (the disposition now, vtable
        // +0x344, taken from it and the difference added, +0x460).
        "SetDisposition" => {
            let who = on?;
            let toward = arg(0).form();
            if toward.0 != 0 && is_actor(order, runner.state, who) {
                let now = disposition(runner.state, who, toward);
                modify_disposition(runner.state, who, toward, arg(1).number() as i32 - now);
            }
            Some(1.0)
        }
        // `005c2190` → `008249d0`: on a person or creature, every effect
        // whose source [`dispelled_by_all`] says goes ends (`00804210`;
        // script effects run their `ScriptEffectFinish`).
        "DispelAllSpells" => {
            let who = on?;
            if is_actor(order, runner.state, who) {
                let (gone, kept): (Vec<_>, Vec<_>) =
                    std::mem::take(&mut runner.state.active_effects)
                        .into_iter()
                        .partition(|e| e.target == who && dispelled_by_all(order, e.source));
                runner.state.active_effects = kept;
                for mut e in gone {
                    if let Some(script) = e.script.filter(|_| e.started) {
                        runner.run_effect_script(script, &mut e, "scripteffectfinish", 0.0);
                    }
                }
            }
            Some(1.0)
        }
        _ => None,
    }
}

/// Whether `DispelAllSpells` ends an effect from this source (`008249d0`,
/// by the magic item's type, vtable +0x18): a spell (`SPEL` type 0), a
/// power (2) or lesser power (3), an ingestible (`ALCH`, type 7: chems,
/// food, drink) or an ingredient (`INGR`, 8); an enchantment (`ENCH`, 6)
/// only when its type (`ENIT` u32 at 0, the item's +0x34) is 0. Diseases
/// (1), abilities (4), poisons (5) and addictions (10) stay.
pub fn dispelled_by_all(order: &LoadOrder, source: FormId) -> bool {
    let Some(rr) = order.get(source) else {
        return false;
    };
    let first_u32 = |sig: &[u8; 4]| {
        rr.record()
            .ok()
            .and_then(|r| r.get(esm::FourCC::new(sig)).map(|s| s.data.clone()))
            .filter(|d| d.len() >= 4)
            .map(|d| u32::from_le_bytes([d[0], d[1], d[2], d[3]]))
    };
    match rr.entry.header.kind.as_bytes() {
        b"SPEL" => matches!(first_u32(b"SPIT"), Some(0 | 2 | 3)),
        b"ALCH" | b"INGR" => true,
        b"ENCH" => first_u32(b"ENIT") == Some(0),
        _ => false,
    }
}

/// How hard `PushActorAway` pushes (`00646580`), from the pushed actor's
/// Agility and the script's number: (`fKnockbackAgilBase` 1 +
/// `fKnockbackAgilMult` −0.008 × Agility × 10) × (number ×
/// `fKnockbackDamageMult` 10 + `fKnockbackDamageBase` 50); the defaults
/// are the exe's (`00f61a40`…`00f61ad0`), `FalloutNV.esm` has none of
/// them. Agility 5 and 5 give 60; Agility 10 and 5, 20.
pub fn push_force(order: &LoadOrder, agility: f64, number: i32) -> f32 {
    let setting = |n: &str, default: f32| game_setting(order, n).unwrap_or(default);
    let agility_part = setting("fKnockbackAgilMult", -0.008) * (agility * 10.0) as f32
        + setting("fKnockbackAgilBase", 1.0);
    let number_part = number as f32 * setting("fKnockbackDamageMult", 10.0)
        + setting("fKnockbackDamageBase", 50.0);
    agility_part * number_part
}

/// Someone's disposition toward another, as far as it's kept here: what
/// scripts added toward the player. The rest of the game's reckoning
/// (`0087fd90`: factions, Charisma, …) isn't carried out, so a person
/// starts at 0.
pub fn disposition(state: &GameState, who: FormId, toward: FormId) -> i32 {
    if toward != PLAYER_REF {
        return 0;
    }
    state.more.dispositions.0.get(&who).copied().unwrap_or(0)
}

/// `0087fb40`: toward the player only, `amount` is added to what the actor
/// keeps, cut so the disposition stays between 0 and 100.
fn modify_disposition(state: &mut GameState, who: FormId, toward: FormId, amount: i32) {
    if toward != PLAYER_REF {
        return;
    }
    let now = disposition(state, who, toward);
    let amount = if now + amount < 0 {
        -now
    } else if now + amount > 100 {
        100 - now
    } else {
        amount
    };
    *state.more.dispositions.0.entry(who).or_insert(0) += amount;
}

/// The causes of death `GetCauseofDeath` gives (the dismembered limbs
/// extra data 0x5f, +0x10); −1 when none was kept.
pub mod cause {
    pub const EXPLOSION: i32 = 0;
    pub const GUN: i32 = 1;
    pub const BLUNT_WEAPON: i32 = 2;
    pub const HAND_TO_HAND: i32 = 3;
    pub const OBJECT_IMPACT: i32 = 4;
    pub const POISON: i32 = 5;
}

/// The cause of death a killing hit leaves (`0089a760`), by the form type
/// of what struck: a weapon itself (a melee blow) blunt, a missile, beam,
/// flame or continuous beam projectile a gun, a grenade or an explosion an
/// explosion, an ingestible poison, debris an object impact; anything else,
/// fists or a creature's attack, hand to hand.
pub fn cause_of(order: &LoadOrder, weapon: Option<&Weapon>) -> i32 {
    let Some(w) = weapon else {
        return cause::HAND_TO_HAND;
    };
    if w.is_melee() {
        return cause::BLUNT_WEAPON;
    }
    // A lobber's projectile (`PROJ` `DATA` type 2) is placed as a grenade.
    let lobber = w
        .projectile
        .and_then(|p| order.get(p))
        .and_then(|p| p.record().ok())
        .and_then(|p| p.get(esm::sig::DATA).map(|s| s.data.clone()))
        .is_some_and(|d| d.len() >= 4 && u16::from_le_bytes([d[2], d[3]]) == 2);
    if lobber {
        cause::EXPLOSION
    } else {
        cause::GUN
    }
}

/// Keeps the cause of `who`'s death (`00572fc0`, through `008b43a0` →
/// `008b4d10`).
pub fn record_cause(state: &mut GameState, who: FormId, cause: i32) {
    state.more.cause_of_death.insert(who, cause);
}

/// Saved lines.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    for (who, amount) in &state.more.dispositions.0 {
        line(format!("disposition {:08X} {amount}", who.0));
    }
    let mut causes: Vec<_> = state.more.cause_of_death.iter().collect();
    causes.sort();
    for (who, cause) in causes {
        line(format!("causeofdeath {:08X} {cause}", who.0));
    }
}

/// A saved line back.
pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    let word = *parts.first()?;
    if word != "disposition" && word != "causeofdeath" {
        return None;
    }
    let who = parts.get(1).and_then(|s| u32::from_str_radix(s, 16).ok());
    let n = parts.get(2).and_then(|s| s.parse::<i32>().ok());
    Some(match (who, n) {
        (Some(who), Some(n)) => {
            if word == "disposition" {
                state.more.dispositions.0.insert(FormId(who), n);
            } else {
                state.more.cause_of_death.insert(FormId(who), n);
            }
            Ok(())
        }
        _ => Err(format!("can't read '{}'", parts.join(" "))),
    })
}
