//! Hardcore mode's needs (dehydration, hunger, sleep deprivation) and the
//! radiation stages, read from the game's code (`findings\living.md` §4).
//!
//! **Growing** (`00969c30`, each frame of play in hardcore and before each
//! waited or slept hour): nothing while scripts have the Pip-Boy turned off
//! (`DisablePlayerControls`; the clocks wait, so the time counts once it's
//! back). The time is `GameDaysPassed` × 24 × 60 game minutes; each need
//! has a clock and a step of `rate × TimeScale / 60` game minutes, i.e. one
//! point per `rate` real seconds of game time: Dehydration (actor value 73)
//! `fHCDehydrationRate` 10 (the exe's; not in the data), Sleep Deprivation
//! (75) `fHCSleepDeprivationRate` 50, Hunger (74) `fHCStarvationRate` 25 (the
//! data's), in that order. Once a step has passed, the value rises by the
//! whole steps passed and the clock jumps to now (the fraction is dropped).
//! Sleep deprivation's clock moves on the same way while the player sleeps
//! through the menu, but nothing is added. A clock of 0 (a new game, a
//! loaded one) puts all three at now first.
//!
//! **Stages** (`DEHY`, `HUNG`, `SLPD`, and `RADS` for radiation: `DATA` u32
//! threshold, u32 spell): kept highest threshold first (`004016a0` inserts
//! a record before the first with a lower threshold); a value's stage is
//! the first whose threshold it reaches (`004018b0`). New Vegas: 200, 400,
//! 600, 800, 1000; stages 1–4 diseases lowering SPECIAL, 5 an ability
//! doing 10000 damage to Health.
//!
//! **When a value changes** (the actor-value callbacks `008c5350` rads,
//! `008c5610` dehydration, `008c5890` hunger, `008c5b10` sleep; the player
//! only): if the stage differs, the old stage's spell is taken off
//! (`00824400`) and the new one given; then two notices: "…increased" when
//! it went up, else "…decreased", and "You are now sick with <spell's
//! name>" (`sRadiationSick` + `%s %s`), or the "no longer" text when no
//! stage is left. Nothing when the stage stays. Lowering goes through the
//! actor value's flag 0x200: a *Restore* lowers these, a *Damage* raises
//! them (`world::magic::change`; `WaterPurified` restores Dehydration 50).
//!
//! **Hardcore off** (`SetHardcore 0`, `00969e90`): "always hardcore" is
//! cleared and the three needs are set to 0 (in the order 73, 75, 74, so
//! their spells come off through the stages).

use esm::{FormId, FourCC, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::{controls, Facts, GameState};

pub const RADS: u16 = 54;
pub const DEHYDRATION: u16 = 73;
pub const HUNGER: u16 = 74;
pub const SLEEP_DEPRIVATION: u16 = 75;

/// The record type listing a value's stages.
pub fn stage_records(av: u16) -> Option<FourCC> {
    Some(FourCC::new(match av {
        RADS => b"RADS",
        DEHYDRATION => b"DEHY",
        HUNGER => b"HUNG",
        SLEEP_DEPRIVATION => b"SLPD",
        _ => return None,
    }))
}

/// One stage: its record, the value it starts at and its spell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stage {
    pub record: FormId,
    pub threshold: u32,
    pub spell: FormId,
}

/// A value's stages as the game keeps them: highest threshold first, a
/// later record of the same threshold after the earlier (`004016a0`).
pub fn stages(order: &LoadOrder, av: u16) -> Vec<Stage> {
    let Some(kind) = stage_records(av) else {
        return Vec::new();
    };
    let mut out: Vec<Stage> = Vec::new();
    for rr in order.records_of_type(kind) {
        let Ok(record) = rr.record() else { continue };
        let Some(data) = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 8) else {
            continue;
        };
        let stage = Stage {
            record: rr.form_id,
            threshold: crate::cell::le_u32(&data.data, 0),
            spell: rr
                .plugin
                .to_global(FormId(crate::cell::le_u32(&data.data, 4))),
        };
        let at = out
            .iter()
            .position(|s| s.threshold < stage.threshold)
            .unwrap_or(out.len());
        out.insert(at, stage);
    }
    out
}

/// The stage a value reaches: the first (highest) whose threshold it's at
/// or above; none below the lowest.
pub fn stage_at(stages: &[Stage], value: f64) -> Option<Stage> {
    stages
        .iter()
        .find(|s| f64::from(s.threshold) <= value)
        .copied()
}

/// The notices' settings for a value: increased, decreased, no longer sick
/// (with the exe's own words; `FalloutNV.esm` overrides some).
fn texts(av: u16) -> [(&'static str, &'static str); 3] {
    match av {
        RADS => [
            ("sRadiationIncrease", "Your radiation level has increased"),
            ("sRadiationDecrease", "Your radiation level has decreased"),
            ("sRadiationNotSick", "You no longer have radiation sickness"),
        ],
        DEHYDRATION => [
            (
                "sDehydrationIncrease",
                "Your dehydration level has increased",
            ),
            (
                "sDehydrationDecrease",
                "Your dehydration level has decreased",
            ),
            (
                "sDehydrationNotSick",
                "You no longer have dehydration sickness",
            ),
        ],
        HUNGER => [
            ("sHungerIncrease", "Your Hunger level has increased"),
            ("sHungerDecrease", "Your Hunger level has decreased"),
            ("sHungerNotSick", "You no longer have Hunger sickness"),
        ],
        _ => [
            (
                "sSleepDeprevationIncrease",
                "Your SleepDeprevation level has increased",
            ),
            (
                "sSleepDeprevationDecrease",
                "Your SleepDeprevation level has decreased",
            ),
            (
                "sSleepDeprevationNotSick",
                "You no longer have SleepDeprevation sickness",
            ),
        ],
    }
}

/// A need's (or radiation's) value changed for someone, from `old` to
/// `new` (the callbacks; see the module notes). Only the player's stages
/// are followed.
pub fn changed(order: &LoadOrder, state: &mut GameState, who: FormId, av: u16, old: f64, new: f64) {
    if who != PLAYER_REF || stage_records(av).is_none() {
        return;
    }
    let list = stages(order, av);
    let (before, after) = (stage_at(&list, old), stage_at(&list, new));
    if before.map(|s| s.record) == after.map(|s| s.record) {
        return;
    }
    if let Some(b) = before {
        crate::magic::remove(state, who, b.spell);
    }
    if let Some(a) = after {
        crate::magic::add_spell(order, state, who, a.spell, who, true);
    }
    let [up, down, well] = texts(av);
    let (name, exe) = if new - old > 0.0 { up } else { down };
    let icon = crate::message_icon::for_setting;
    super::notice_with(state, super::text(order, name, exe), icon(name));
    let (second, second_icon) = match after {
        Some(a) => {
            let spell = order
                .get(a.spell)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.full_name())
                .unwrap_or_default();
            (
                format!(
                    "{} {spell}",
                    super::text(order, "sRadiationSick", "You are now sick with")
                ),
                icon("sRadiationSick"),
            )
        }
        None => (super::text(order, well.0, well.1), icon(well.0)),
    };
    super::notice_with(state, second, second_icon);
}

/// A value now (rads and the needs are what they've been raised by).
pub fn value(order: &LoadOrder, state: &GameState, who: FormId, av: u16) -> f64 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, av)
    .unwrap_or(0.0)
}

/// Raises a need by whole points (a damage, through the actor value's
/// flag 0x200).
fn raise(order: &LoadOrder, state: &mut GameState, av: u16, points: i64) {
    if points > 0 {
        crate::magic::change(order, state, PLAYER_REF, av, -(points as f64), PLAYER_REF);
    }
}

/// The game's minutes now: `GameDaysPassed` × 24 × 60 (`00867ea0`),
/// worked out wider and kept as a float as the game's x87 code does.
pub fn minutes_now(order: &LoadOrder, state: &GameState) -> f32 {
    (f64::from(state.global(order, "GameDaysPassed").unwrap_or(1.0)) * 24.0 * 60.0) as f32
}

/// The needs grow (`00969c30`; see the module notes). Nothing outside
/// hardcore.
pub fn grow(order: &LoadOrder, state: &mut GameState) {
    if !state.living.hardcore || state.controls_off[controls::PIPBOY] {
        return;
    }
    let scale = f64::from(state.global(order, "TimeScale").unwrap_or(30.0));
    let step =
        |name: &str, exe: f32| (f64::from(super::setting(order, name, exe)) * scale / 60.0) as f32;
    let dehydration = step("fHCDehydrationRate", 10.0);
    let sleep = step("fHCSleepDeprivationRate", 0.26);
    let hunger = step("fHCStarvationRate", 0.39);
    let now = minutes_now(order, state);
    let clocks = &mut state.living.clocks;
    if clocks[0] == 0.0 {
        *clocks = [now; 3];
    }
    let [c_dehydration, c_hunger, c_sleep] = *clocks;
    let points = |passed: f32, step: f32| (f64::from(passed) / f64::from(step)).trunc() as i64;
    if step_passed(now - c_dehydration, dehydration) {
        let n = points(now - c_dehydration, dehydration);
        state.living.clocks[0] = now;
        raise(order, state, DEHYDRATION, n);
    }
    if step_passed(now - c_sleep, sleep) {
        let n = points(now - c_sleep, sleep);
        state.living.clocks[2] = now;
        if !state.living.sleeping {
            raise(order, state, SLEEP_DEPRIVATION, n);
        }
    }
    if step_passed(now - c_hunger, hunger) {
        let n = points(now - c_hunger, hunger);
        state.living.clocks[1] = now;
        raise(order, state, HUNGER, n);
    }
}

/// Whether a step has gone by (the game's `step <= passed`); a step of 0
/// or less never does.
fn step_passed(passed: f32, step: f32) -> bool {
    step > 0.0 && step <= passed
}

/// Hardcore turned on or off (`SetHardcore`, `00969e90`): off clears
/// "always hardcore" and sets the needs to 0 (73, 75, 74), their stages'
/// spells coming off with notices.
pub fn set_hardcore(order: &LoadOrder, state: &mut GameState, on: bool) {
    if !on {
        state.living.always_hardcore = false;
        for av in [DEHYDRATION, SLEEP_DEPRIVATION, HUNGER] {
            let v = value(order, state, PLAYER_REF, av);
            if v != 0.0 {
                crate::magic::change(order, state, PLAYER_REF, av, v, PLAYER_REF);
            }
        }
    }
    state.living.hardcore = on;
}
