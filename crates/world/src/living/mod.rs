//! Living in the Mojave, read from the game's code
//! (`%USERPROFILE%\nv-re\findings\living.md`, plus the functions named in
//! each module): sleeping and waiting ([`sleep`]), hardcore mode's needs and
//! the radiation stages ([`needs`]), pickpocketing ([`pickpocket`]) and
//! being warned off someone's property ([`trespass`]).
//!
//! What these keep is [`Living`] (`GameState::living`); the script
//! functions about them are answered here ([`value`], [`change`]).
//!
//! For the game's menus and the HUD, which draw them:
//! - sleeping and waiting: [`sleep::is_bed`]; [`sleep::may_sleep_in`] and
//!   [`sleep::may_wait`] (the game's refusal, or `Ok` to open the menu);
//!   [`sleep::begin`] when the hours are accepted; then each frame
//!   [`sleep::menu_mode`] (the scripts' `MenuMode` blocks) and
//!   [`sleep::menu_frame`] (`Some(true)` once the last hour has passed);
//!   [`sleep::cancel`]; [`sleep::MOST_HOURS`];
//! - picking pockets: [`pickpocket::use_person`] (what using a person
//!   does); [`pickpocket::Visit::new`] on opening; per stack
//!   [`pickpocket::stack_value`] and [`pickpocket::chance`] (for the
//!   menu's figures) and [`pickpocket::attempt`] with the game's roll
//!   (`GameState::roll() % 100`); a caught visit closes the menu;
//! - the crosshair: [`crosshair_red`].

pub mod needs;
pub mod pickpocket;
pub mod sleep;
pub mod trespass;

use std::collections::{BTreeMap, HashSet};

use esm::FormId;

use crate::dialogue::PLAYER_REF;
use crate::scripting::{Facts, GameState, Runner, Value};

/// What sleeping, the needs, pickpocketing and trespassing keep.
#[derive(Debug, Clone, Default)]
pub struct Living {
    /// Hardcore mode (the player's +0x7bc; `IsHardcore` is it == 1).
    pub hardcore: bool,
    /// "Always hardcore" (+0xe38, the achievement's tracking): set for a new
    /// player (`00938180`, `0095c9c0`), cleared for good by turning hardcore
    /// off or `DisableHardcoreTracking` (`005de9b0`).
    pub always_hardcore: bool,
    /// The needs' clocks, game minutes (dehydration, hunger, sleep
    /// deprivation: the game's `[011e0d7c]`, `[011e0d78]`, `[011e0d74]`).
    /// Not saved: the game resets them on loading (`0095a3b0`) and for a
    /// new game (`00958fc0`), which a first clock of 0 does here.
    pub clocks: [f32; 3],
    /// The sleep/wait menu at work: hours still to pass (the player's
    /// +0x654, `GetPCSleepHours`) and whether it's sleep (+0x658,
    /// `IsPCSleeping`). Not saved (the menu is never open in a save).
    pub hours_left: i32,
    pub sleeping: bool,
    /// People who have caught the player pickpocketing (their AI process's
    /// flag, process+0x188, set by `008d86e0`; nothing was found that clears
    /// it). They refuse further attempts. Saved here (whether the game's
    /// save keeps it isn't traced).
    pub caught_by: HashSet<FormId>,
    /// What the player has slipped into people's pockets, (person, item) →
    /// count: their stacks keep the player as owner, so taking them back
    /// needs no roll (`0075dc80`).
    pub planted: BTreeMap<(FormId, FormId), i32>,
    /// Someone coming to warn the player off their property (the trespass
    /// package, [`trespass::Warning`]), and the cell whose alarm has gone
    /// off: what follows an alarm isn't traced, so nothing more is done
    /// there while the player stays. Not saved.
    pub trespass: Option<trespass::Warning>,
    pub trespass_alarm: Option<FormId>,
}

impl Living {
    /// A new player (`00938180`): "always hardcore" until hardcore is
    /// turned off.
    pub fn new_game() -> Living {
        Living {
            always_hardcore: true,
            ..Living::default()
        }
    }
}

/// The script functions answered or carried out here, by the game's own
/// names (the table's).
pub const HANDLED: &[&str] = &[
    "IsHardcore",
    "SetHardcore",
    "IsAlwaysHardcore",
    "DisableHardcoreTracking",
    "IsPCSleeping",
    "GetPCSleepHours",
    "SetPCSleepHours",
    "ShowSleepWaitMenu",
    "GetTrespassWarningLevel",
    "SendTrespassAlarm",
];

const READS: &[&str] = &[
    "IsHardcore",
    "IsAlwaysHardcore",
    "IsPCSleeping",
    "GetPCSleepHours",
    "GetTrespassWarningLevel",
];

fn flag(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

/// The functions that read, as their handlers answer: `IsHardcore`
/// (`005a5f90`: the player's +0x7bc == 1, whoever it's asked of),
/// `IsAlwaysHardcore` (`005a5ff0`: +0xe38), `IsPCSleeping` (`005a1de0`:
/// +0x658), `GetPCSleepHours` (`005cde00`: +0x654), `GetTrespassWarningLevel`
/// (`005a1770`: −1 unless the subject's package is a trespass warning, then
/// the warnings it has given).
pub(crate) fn value(
    facts: &Facts,
    name: &str,
    on: Option<FormId>,
    _args: &[Value],
) -> Option<Option<f64>> {
    if !READS.contains(&name) {
        return None;
    }
    let s = &facts.state.living;
    Some(Some(match name {
        "IsHardcore" => flag(s.hardcore),
        "IsAlwaysHardcore" => flag(s.always_hardcore),
        "IsPCSleeping" => flag(s.sleeping),
        "GetPCSleepHours" => f64::from(s.hours_left),
        "GetTrespassWarningLevel" => {
            let who = on?;
            match &s.trespass {
                Some(w) if w.warner == who => f64::from(w.given),
                _ => -1.0,
            }
        }
        _ => return None,
    }))
}

/// The functions that change things.
///
/// - `SetHardcore n` (`005de920`): only 0 or 1 (anything else is an error
///   and changes nothing); see [`needs::set_hardcore`].
/// - `DisableHardcoreTracking` (`005de9d0`): "always hardcore" off.
/// - `SetPCSleepHours n` (`005cde40`): hours left = n and the sleeping
///   flag on (`005c1a00(n, 1)`), as the timer scripts that cut a sleep
///   short use it (`VMQ03aTimerSCRIPT`).
/// - `ShowSleepWaitMenu sleep [checks]` (`005e0090`): with `checks` 1 the
///   wait refusals (`00969fa0`) and then the menu in sleep mode (its
///   `007054f0(1)`, whatever `sleep` says); else the menu in sleep (1) or
///   wait (0) mode: [`crate::scripting::Event::SleepWaitMenu`].
/// - `SendTrespassAlarm criminal` (`005d47e0`): with a criminal and an
///   actor to call it, the trespass alarm ([`trespass::alarm`]) raised by
///   the caller about its own base.
pub(crate) fn change(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<Option<f64>> {
    if !HANDLED.contains(&name) || READS.contains(&name) {
        return None;
    }
    let order = runner.order;
    let arg = |i: usize| args.get(i).map_or(0.0, Value::number);
    Some((|| {
        match name {
            "SetHardcore" => {
                let n = arg(0);
                if n != 0.0 && n != 1.0 {
                    return Some(0.0);
                }
                needs::set_hardcore(order, runner.state, n == 1.0);
            }
            "DisableHardcoreTracking" => runner.state.living.always_hardcore = false,
            "SetPCSleepHours" => {
                runner.state.living.hours_left = arg(0) as i32;
                runner.state.living.sleeping = true;
            }
            "ShowSleepWaitMenu" => {
                let sleep = arg(0) != 0.0;
                if arg(1) == 1.0 {
                    if let Err(why) = sleep::may_wait(order, runner.state) {
                        // Every refusal has the sad Vault Boy (`00969fa0`).
                        runner.state.events.push(crate::scripting::Event::Message {
                            title: None,
                            text: why,
                            buttons: Vec::new(),
                            icon: Some(crate::message_icon::SAD.to_string()),
                        });
                        return Some(1.0);
                    }
                    runner
                        .state
                        .events
                        .push(crate::scripting::Event::SleepWaitMenu { sleep: true });
                } else {
                    runner
                        .state
                        .events
                        .push(crate::scripting::Event::SleepWaitMenu { sleep });
                }
            }
            "SendTrespassAlarm" => {
                let alarmer = target?;
                let criminal = args.first().map(Value::form).filter(|f| f.0 != 0)?;
                if criminal == PLAYER_REF && crate::script_functions::is_actor(order, alarmer) {
                    let owner = crate::scripting::base_of(order, alarmer);
                    trespass::alarm(order, runner.state, alarmer, owner);
                }
            }
            _ => return None,
        }
        Some(0.0)
    })())
}

/// Saved lines (one fact a line, as `world::save`).
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let id = |f: FormId| format!("{:08X}", f.0);
    let s = &state.living;
    if s.hardcore {
        line("hardcore".to_string());
    }
    if s.always_hardcore {
        line("alwayshardcore".to_string());
    }
    let mut caught: Vec<FormId> = s.caught_by.iter().copied().collect();
    caught.sort();
    for c in caught {
        line(format!("caughtby {}", id(c)));
    }
    for ((who, item), n) in &s.planted {
        line(format!("planted {} {} {n}", id(*who), id(*item)));
    }
}

/// A saved line back, if it's one of these.
pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let word = *parts.first()?;
    let bad = || format!("can't read '{raw}'");
    let form = |i: usize| -> Result<FormId, String> {
        parts
            .get(i)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(FormId)
            .ok_or_else(bad)
    };
    let num = |i: usize| -> Result<f64, String> {
        parts
            .get(i)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(bad)
    };
    let s = &mut state.living;
    let result = (|| -> Result<bool, String> {
        match word {
            "hardcore" => s.hardcore = true,
            "alwayshardcore" => s.always_hardcore = true,
            "caughtby" => {
                s.caught_by.insert(form(1)?);
            }
            "planted" => {
                s.planted.insert((form(1)?, form(2)?), num(3)? as i32);
            }
            _ => return Ok(false),
        }
        Ok(true)
    })();
    match result {
        Ok(true) => Some(Ok(())),
        Ok(false) => None,
        Err(e) => Some(Err(e)),
    }
}

/// Whether the crosshair's text for something is red (`00579690`):
/// never while the player is dead; a living person while the player
/// sneaks (the "Pickpocket" prompt, `004997b0`); else anything but a
/// creature whose owner (`world::crime::owner_of`) the player may not take
/// from: items, containers, furniture (a bed by its own test `00509420`,
/// the rest the same way), owned people. `None` for doors: their rule
/// (locked and owned, or leading into someone's private interior) asks
/// whether the player can open the door (`00518f00`), which isn't traced.
/// Creatures while sneaking: red when their record allows talking to the
/// player (base +0x30 vtable +0x18, `GetAllowPCDialogue` (Xbox PDB),
/// `0047c790`: `ACBS` flags 0x200000).
pub fn crosshair_red(order: &esm::LoadOrder, state: &GameState, reference: FormId) -> Option<bool> {
    use crate::crime::{may_take, owner_of};
    if state.dead.contains(&PLAYER_REF) {
        return Some(false);
    }
    let base_kind = crate::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .map(|r| *r.entry.header.kind.as_bytes());
    let alive = !state.dead.contains(&reference);
    match base_kind {
        Some(b) if &b == b"DOOR" => return None,
        Some(b) if &b == b"NPC_" && state.player_sneaking && alive => return Some(true),
        Some(b) if &b == b"CREA" => {
            let talks = crate::scripting::base_of(order, reference).is_some_and(|base| {
                crate::activation::actor_flags(order, base) & crate::activation::ALLOW_PC_DIALOGUE
                    != 0
            });
            return Some(state.player_sneaking && alive && talks);
        }
        _ => {}
    }
    let owner = owner_of(order, state, reference);
    Some(owner.is_some() && !may_take(order, state, owner))
}

/// A person's or thing's name as the game shows it (`0055d520`): its base
/// record's `FULL`, as a script renamed it if one did.
pub(crate) fn display_name(order: &esm::LoadOrder, state: &GameState, who: FormId) -> String {
    let base = crate::scripting::base_of(order, who);
    if let Some(n) = base.and_then(|b| state.set_by_scripts.names.get(&b)) {
        return n.clone();
    }
    base.and_then(|b| order.get(b))
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_else(|| who.to_string())
}

/// A text game setting, else the exe's own words.
pub(crate) fn text(order: &esm::LoadOrder, name: &str, exe: &str) -> String {
    crate::scripting::game_setting_text(order, name).unwrap_or_else(|| exe.to_string())
}

/// A number game setting, else the exe's own value.
pub(crate) fn setting(order: &esm::LoadOrder, name: &str, exe: f32) -> f32 {
    crate::scripting::game_setting(order, name).unwrap_or(exe)
}

/// A notice on the screen (the game's HUD message, `007052f0`) with its
/// picture (`world::message_icon`).
pub(crate) fn notice_with(state: &mut GameState, text: String, icon: Option<&str>) {
    state.events.push(crate::scripting::Event::Message {
        title: None,
        text,
        buttons: Vec::new(),
        icon: icon.map(str::to_string),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions_go_by_the_games_own_names_and_once() {
        for name in HANDLED {
            assert!(
                script::functions::FUNCTIONS.iter().any(|f| f.name == *name),
                "{name} isn't a function's name in the game's table"
            );
            assert!(
                !crate::scripting::HANDLED.contains(name),
                "{name} is carried out twice"
            );
        }
        assert!(READS.iter().all(|r| HANDLED.contains(r)));
    }
}
