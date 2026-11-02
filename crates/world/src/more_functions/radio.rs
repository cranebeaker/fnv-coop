//! The radio's script functions (handlers from the command table at
//! `01190910`), acting on the one radio the Pip-Boy shows and plays
//! (`FalloutRadio` (Xbox PDB), [`crate::radio::Radio`], kept in
//! `GameState::radio`): switching and tuning the Pip-Boy radio, a station's
//! conversation, people playing a station (`SetNPCRadio`). Notes:
//! `docs/DEAD_MONEY.md` "Radio", `docs/PIPBOY.md`.
//!
//! The radio's "disabled" flag (`011dd436`, which makes every one of these
//! do nothing) is taken as clear: what sets it isn't traced. Stations made
//! from an activator's or an actor's radio template (`004fd3c0` →
//! `008356e0`) aren't carried out: such a reference can't be tuned (the
//! radio goes off, as when no station can be made).

use esm::{FormId, FourCC};

use super::{is_actor, kind_of, placed, st};
use crate::scripting::{GameState, Runner, Value};

const TACT: FourCC = FourCC::new(b"TACT");

/// The radio's functions, by the game's own names.
pub const CHANGES: &[&str] = &[
    "PipboyRadio",
    "PipBoyRadioOff",
    "StartRadioConversation",
    "SetNPCRadio",
    "ForceRadioStationUpdate",
    "ResetPipboyManager",
];

/// Whether a reference can be a station by itself: its base is a talking
/// activator (form type 0x16; `00832cb0`).
fn is_station(runner: &Runner, r: FormId) -> bool {
    placed::base_now(runner.order, runner.state, r).and_then(|b| kind_of(runner.order, b))
        == Some(TACT)
}

/// Tunes the Pip-Boy radio (`00832240(station, 1)`): only while it's on.
/// A reference that can't be a station switches the radio off (the
/// station object isn't made, `00832cb0` gives none). No station given:
/// the first station in range of the player (none: off).
// Translated from 00832240 (decompiled, FalloutNV.exe 1.4.0.525)
fn tune(runner: &mut Runner, station: FormId) {
    if !runner.state.radio.on {
        return;
    }
    let station = if station.0 == 0 {
        let mut radio = std::mem::take(&mut runner.state.radio);
        let first = radio.first_in_range(runner.order, runner.state);
        runner.state.radio = radio;
        match first {
            Some(s) => s,
            None => return runner.state.radio.script_enable(false),
        }
    } else {
        station
    };
    if is_station(runner, station) {
        runner.state.radio.script_tune(station);
    } else {
        runner.state.radio.script_enable(false);
    }
}

/// Carries out one of [`CHANGES`]; `None` when it isn't one.
pub(super) fn carry_out(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    match name {
        // `005d7fb0`: a word, then an optional station. A word starting
        // with `1`, or `enable` or `on`: on, and tuned to the station;
        // starting with `0`, or `disable` or `off`: off; `tune`: tuned
        // (while on). Dead Money writes `Tune`, so the words are compared
        // without case (`00404dc0`).
        "PipboyRadio" => {
            let word = match arg(0) {
                Value::Text(t) => t.to_ascii_lowercase(),
                Value::Number(n) => format!("{n}"),
                Value::Form(f) => format!("{}", f.0),
            };
            let station = arg(1).form();
            if word.starts_with('1') || word == "enable" || word == "on" {
                runner.state.radio.script_enable(true);
                tune(runner, station);
            } else if word.starts_with('0') || word == "disable" || word == "off" {
                runner.state.radio.script_enable(false);
            } else if word == "tune" {
                tune(runner, station);
            }
        }
        // `005dc580` → `008324e0(0)`.
        "PipBoyRadioOff" => runner.state.radio.script_enable(false),
        // `005d82a0` → `00835be0`: on a station, its conversation starts
        // now, replacing whatever it was playing; no topic given, the
        // default one.
        "StartRadioConversation" => {
            let station = target?;
            if is_station(runner, station) {
                let topic = Some(arg(0).form()).filter(|f| f.0 != 0);
                let (order, scripts) = (runner.order, runner.scripts);
                let mut radio = std::mem::take(&mut runner.state.radio);
                radio.start_conversation(order, scripts, runner.state, station, topic);
                runner.state.radio = radio;
            }
        }
        // `005d8100`: on a person (vtable +0x100) with a station: 1 plays
        // the station through them (`00835810`), 0 stops it (`00835980`);
        // other numbers do nothing.
        "SetNPCRadio" => {
            let who = target.filter(|&t| is_actor(runner.order, runner.state, t))?;
            let station = arg(1).form();
            if station.0 == 0 {
                return Some(0.0);
            }
            match arg(0).number() as i32 {
                1 => {
                    let base = placed::base_now(runner.order, runner.state, station)?;
                    let order = runner.order;
                    let mut radio = std::mem::take(&mut runner.state.radio);
                    radio.enable_npc_radio(order, runner.state, who, base);
                    runner.state.radio = radio;
                }
                0 => runner.state.radio.disable_npc_radio(who),
                _ => {}
            }
        }
        // `005d8280` → `00832ad0(1)`: the stations update now rather than
        // at their next interval (here at the radio's next frame).
        "ForceRadioStationUpdate" => runner.state.radio.force_update = true,
        // `005db490`: the player's Pip-Boy manager (`00705990`), when
        // there is one, gets its reset flag (`005db4c0(1)`, +0x16c; what
        // reads it isn't traced).
        "ResetPipboyManager" => st(runner).pipboy_reset = true,
        _ => return None,
    }
    Some(1.0)
}

/// Saved lines: a station's conversation a script started, while it
/// plays; the people playing stations; the Pip-Boy manager's reset. (The
/// Pip-Boy radio itself is the `radio` line, `crate::radio::save_lines`.)
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let id = |f: FormId| format!("{:08X}", f.0);
    for s in &state.radio.stations {
        if let Some(topic) = s.started {
            line(format!(
                "radioconversation {} {}",
                id(s.reference),
                topic.map_or("00000000".into(), id)
            ));
        }
    }
    for s in &state.radio.stations {
        for u in s.users.iter().filter(|u| u.playing) {
            line(format!("npcradio {} {}", id(u.reference), id(s.reference)));
        }
    }
    if state.more.pipboy_reset {
        line("pipboyreset".into());
    }
}

/// A saved line back. `scriptradio on <station>` (saves from before the
/// script functions shared the Pip-Boy's radio) is the Pip-Boy radio on
/// and tuned to it; a conversation starts again from its first line.
pub(crate) fn load_line(state: &mut GameState, parts: &[&str]) -> Option<Result<(), String>> {
    let form = |i: usize| {
        parts
            .get(i)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(FormId)
    };
    let nonzero = |f: FormId| Some(f).filter(|f| f.0 != 0);
    let bad = || Err(format!("can't read '{}'", parts.join(" ")));
    let r = &mut state.radio;
    Some(match *parts.first()? {
        "scriptradio" if parts.get(1) == Some(&"on") => match form(2) {
            Some(t) => {
                r.on = true;
                r.active = nonzero(t);
                if let Some(t) = r.active {
                    r.restore_station(t);
                }
                Ok(())
            }
            None => bad(),
        },
        "radioconversation" => match (form(1), form(2)) {
            (Some(s), Some(t)) => {
                let i = r.restore_station(s);
                r.stations[i].started = Some(nonzero(t));
                r.stations[i].pending_start = true;
                Ok(())
            }
            _ => bad(),
        },
        "npcradio" => match (form(1), form(2)) {
            (Some(w), Some(s)) => {
                r.restore_receiver(w, s);
                Ok(())
            }
            _ => bad(),
        },
        "pipboyreset" => {
            state.more.pipboy_reset = true;
            Ok(())
        }
        _ => return None,
    })
}
