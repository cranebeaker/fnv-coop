//! Doors and gates that open where they stand, as the game's own code does
//! them (`%USERPROFILE%\nv-re\findings\doors.md`). Load doors (with a
//! destination, `XTEL`) are different: the game teleports through those
//! without playing anything (`TESObjectDOOR::Activate`, `005180b0`).
//!
//! The rules, each with the function it was read from:
//!
//! - A door's state is its reference's action flag 4 ("open", kept in
//!   `ExtraAction`, `00572d30`), set and cleared as it opens and closes.
//!   `GetOpenState` (`0047b250`) answers 0 for anything that doesn't open
//!   (only activators, terminals, containers and doors do, `0047a490`), 1
//!   open, 3 closed, and while the model's `Open` or `Close` sequence is
//!   playing 2 (opening) or 4 (closing).
//! - It starts open by default (`00561d90`: action flag 8, which the
//!   editor writes as the reference's `XACT` and an empty `ONAM` marker)
//!   or closed; when its 3D loads (`005659f0` → `0047aec0`) the matching
//!   sequence is set to its end at once, with no sound.
//! - Activating it (`0047a560`, the open/close form's part of
//!   `TESObjectDOOR::Activate`): nothing happens while either sequence is
//!   still playing; otherwise the flag flips, the base's opening sound
//!   (`SNAM`) or closing sound (`ANAM`) plays at the door, and the new
//!   state's sequence plays from its start over its own length (`Open` 1 s
//!   on Goodsprings' picket gate). Whoever activated it is remembered
//!   (`ExtraOpenCloseActivateRef`). A model without both sequences just
//!   flips.
//! - When the sequence reaches its end (`00567050`, every frame: a
//!   non-looping sequence is finished once its clock passes its last key)
//!   the door's script runs its `OnOpen` or `OnClose` block (`0047ac70`,
//!   event flags 0x10000 / 0x20000), and the door's collision is put at the
//!   leaf's new place: with the INI's `bAnimateDoorPhysics` off (the
//!   shipped default) the Havok bodies are only re-placed then
//!   (`0047ab40`); on, they follow the leaf as it swings.
//! - `SetOpenState` (`005ced30`) activates the door with no activator
//!   when it isn't already as asked (open or opening for 1; closed or
//!   closing for 0): the sound still plays (it's the base's), and a lock is
//!   not in the way (the lock is only checked for an actor, `005180b0`).
//!   `Lock` (`005cbf80`) shuts an open door at once, with no animation or
//!   sound. `SetDefaultOpen` (`005cebf0`) changes the default and shows the
//!   door as it is now.
//! - People walking a path through a closed door (`009e20c0`, from the
//!   actor's path following, `009e0a00`) activate it when they reach it and
//!   wait while it opens (state 2), then go on.
//! - `DOOR` `FNAM` flags (byte at the record's +0x84, `00517c30`): 0x02
//!   automatic (`00517fa0`: the door gets a Havok controller that activates
//!   it for actors within `fAutoDoorActivateDistance` 300, `00519040`; no
//!   New Vegas door has it), 0x04 hidden (left off the local map), 0x08
//!   minimal use (the AI's paths avoid it), 0x10 sliding (`00518080`: not a
//!   navmesh obstacle, and no Havok change when a person opens it).

use std::collections::HashMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::script_functions::kind_of;
use crate::scripting::{base_of, GameState};

const DOOR: FourCC = FourCC::new(b"DOOR");
const FNAM: FourCC = FourCC::new(b"FNAM");
const XTEL: FourCC = FourCC::new(b"XTEL");
const ONAM: FourCC = FourCC::new(b"ONAM");
const XACT: FourCC = FourCC::new(b"XACT");

/// `DOOR` `FNAM` flags.
pub const AUTOMATIC: u8 = 0x02;
pub const HIDDEN: u8 = 0x04;
pub const MINIMAL_USE: u8 = 0x08;
pub const SLIDING: u8 = 0x10;

/// What `GetOpenState` answers (`0047b250`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenState {
    /// Not something that opens.
    None = 0,
    Open = 1,
    Opening = 2,
    Closed = 3,
    Closing = 4,
}

impl OpenState {
    pub fn is_open(self) -> bool {
        matches!(self, OpenState::Open | OpenState::Opening)
    }

    pub fn is_moving(self) -> bool {
        matches!(self, OpenState::Opening | OpenState::Closing)
    }
}

/// How long a door model's `Open` and `Close` sequences play, seconds.
/// Known once the model has been looked at (`GameState::door_lengths`,
/// filled by whoever loads models); a door whose model has no such
/// sequences, or isn't loaded, just flips.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lengths {
    pub open: f32,
    pub close: f32,
}

/// A door's sequence playing: which way, when it started (the state's
/// clock, [`GameState::seconds`]) and for how long. Not saved: a loaded
/// door is shown at its end.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub opening: bool,
    pub started: f64,
    pub length: f32,
    /// Who activated it (`ExtraOpenCloseActivateRef`), for the `OnOpen` /
    /// `OnClose` blocks' action reference; none for a script's
    /// `SetOpenState`.
    pub by: Option<FormId>,
}

impl Motion {
    pub fn ends(&self) -> f64 {
        self.started + f64::from(self.length)
    }
}

/// Base object types the game opens and closes (`0047a490`: activators,
/// terminals, containers and doors).
pub fn opens_and_closes(base_type: FourCC) -> bool {
    [b"ACTI", b"TERM", b"CONT", b"DOOR"]
        .iter()
        .any(|t| base_type == FourCC::new(t))
}

/// A door's `FNAM` flags (0 when it has none).
pub fn flags(order: &LoadOrder, door_base: FormId) -> u8 {
    order
        .get(door_base)
        .and_then(|r| r.record_shared().ok())
        .and_then(|r| r.get(FNAM).and_then(|s| s.data.first().copied()))
        .unwrap_or(0)
}

/// Whether a reference is a door that opens where it stands: a `DOOR`
/// without a destination (`XTEL`).
pub fn is_swing_door(order: &LoadOrder, reference: FormId) -> bool {
    if base_of(order, reference).and_then(|b| kind_of(order, b)) != Some(DOOR) {
        return false;
    }
    order
        .get(reference)
        .and_then(|r| r.record().ok())
        .is_some_and(|r| r.get(XTEL).is_none())
}

/// Whether a door reference is placed open: action flag 8 (`00561d90`),
/// which the editor writes as `XACT` and as an empty `ONAM` marker (both on
/// the same 255 references in `FalloutNV.esm`).
pub fn open_by_default(order: &LoadOrder, reference: FormId) -> bool {
    order
        .get(reference)
        .and_then(|r| r.record_shared().ok())
        .is_some_and(|r| {
            r.get(ONAM).is_some()
                || r.get(XACT)
                    .filter(|s| s.data.len() >= 4)
                    .is_some_and(|s| le_u32(&s.data, 0) & crate::OPEN_BY_DEFAULT != 0)
        })
}

/// Whether a door's "open" flag (action flag 4) is set now: as the game has
/// left it, else as placed.
pub fn is_open(order: &LoadOrder, state: &GameState, door: FormId) -> bool {
    match state.set_by_scripts.open.get(&door) {
        Some(&open) => open,
        None => open_by_default(order, door),
    }
}

/// The lengths of a door's sequences, when its model has been looked at.
pub fn lengths(order: &LoadOrder, state: &GameState, door: FormId) -> Option<Lengths> {
    let base = base_of(order, door)?;
    state.door_lengths.get(&base).copied()
}

/// What `GetOpenState` answers for a reference now (`0047b250`).
pub fn open_state(order: &LoadOrder, state: &GameState, reference: FormId) -> OpenState {
    let kind = base_of(order, reference).and_then(|b| kind_of(order, b));
    if !kind.is_some_and(opens_and_closes) {
        return OpenState::None;
    }
    let open = is_open(order, state, reference);
    match state.door_motion.get(&reference) {
        Some(m) if state.seconds < m.ends() => {
            if m.opening {
                OpenState::Opening
            } else {
                OpenState::Closing
            }
        }
        _ if open => OpenState::Open,
        _ => OpenState::Closed,
    }
}

/// What activating a door did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activated {
    /// It starts opening (or is open at once, without a model's
    /// sequences); the sound to play is the base's `SNAM`.
    Opening,
    /// It starts closing; the sound is the base's `ANAM`.
    Closing,
    /// Its sequence is still playing: nothing happens (`0047a560`).
    Busy,
}

/// Activates a door (`0047a560`): flips it unless a sequence is still
/// playing, starting the new state's sequence (over its length, when the
/// model has one) and remembering who did it.
pub fn activate(
    order: &LoadOrder,
    state: &mut GameState,
    door: FormId,
    by: Option<FormId>,
) -> Activated {
    let now = state.seconds;
    if state.door_motion.get(&door).is_some_and(|m| now < m.ends()) {
        return Activated::Busy;
    }
    let opening = !is_open(order, state, door);
    state.set_by_scripts.open.insert(door, opening);
    let length = lengths(order, state, door)
        .map(|l| if opening { l.open } else { l.close })
        .unwrap_or(0.0)
        .max(0.0);
    state.door_motion.insert(
        door,
        Motion {
            opening,
            started: now,
            length,
            by,
        },
    );
    if opening {
        Activated::Opening
    } else {
        Activated::Closing
    }
}

/// The sound an activation plays: the base's opening (`SNAM`) or closing
/// (`ANAM`) sound (`0047a560`, `00407820` / `00407840`).
pub fn sound(order: &LoadOrder, door: FormId, activated: Activated) -> Option<FormId> {
    let base = base_of(order, door)?;
    let (open, close) = crate::sound::door_sounds(order, base);
    match activated {
        Activated::Opening => open,
        Activated::Closing => close,
        Activated::Busy => None,
    }
}

/// Puts a door open or shut at once, with no sequence or sound (`0047aec0`
/// with its "set" argument: a lock closing it, `SetDefaultOpen`, a loaded
/// state).
pub fn set_at_once(state: &mut GameState, door: FormId, open: bool) {
    state.set_by_scripts.open.insert(door, open);
    state.door_motion.remove(&door);
}

/// `SetOpenState` (`005ced30`): activates the door, with nobody as the
/// activator, when it isn't already as asked. A door activated with no
/// actor is unlocked first (`005180b0`: the lock is only put to an actor;
/// without one it's just cleared). Whether anything happened.
pub fn set_open_state(
    order: &LoadOrder,
    state: &mut GameState,
    door: FormId,
    open: bool,
) -> Option<Activated> {
    let now = open_state(order, state, door);
    if now == OpenState::None {
        return None;
    }
    if now.is_open() == open {
        return None;
    }
    if crate::locks::lock_now(order, state, door).is_some() {
        state.locks.insert(door, None);
    }
    Some(activate(order, state, door, None))
}

/// The doors whose sequence has just finished (`00567050`): their motions
/// are dropped and each is given back with whether it opened and who did
/// it, for the `OnOpen` / `OnClose` blocks and the collision's move.
pub fn finished(state: &mut GameState) -> Vec<(FormId, Motion)> {
    let now = state.seconds;
    let done: Vec<FormId> = state
        .door_motion
        .iter()
        .filter(|(_, m)| now >= m.ends())
        .map(|(&d, _)| d)
        .collect();
    let mut out: Vec<(FormId, Motion)> = done
        .into_iter()
        .filter_map(|d| state.door_motion.remove(&d).map(|m| (d, m)))
        .collect();
    out.sort_by_key(|(d, _)| d.0);
    out
}

/// Where a door's leaf is along its swing at `seconds` on the state's
/// clock: the sequence playing (`Open` or `Close`) and how far into it,
/// clamped to its end; with no motion, the end of the sequence of its
/// state (what the game shows after `0047aec0` snaps it).
pub fn pose(order: &LoadOrder, state: &GameState, door: FormId, seconds: f64) -> (bool, f32) {
    match state.door_motion.get(&door) {
        Some(m) => (
            m.opening,
            ((seconds - m.started) as f32).clamp(0.0, m.length),
        ),
        None => {
            let open = is_open(order, state, door);
            let length = lengths(order, state, door)
                .map(|l| if open { l.open } else { l.close })
                .unwrap_or(0.0);
            (open, length)
        }
    }
}

/// The sequence lengths of door models, by base record, for
/// [`GameState::door_lengths`].
pub type DoorLengths = HashMap<FormId, Lengths>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_state_numbers_match_the_games() {
        assert_eq!(OpenState::None as i32, 0);
        assert_eq!(OpenState::Open as i32, 1);
        assert_eq!(OpenState::Opening as i32, 2);
        assert_eq!(OpenState::Closed as i32, 3);
        assert_eq!(OpenState::Closing as i32, 4);
        assert!(OpenState::Opening.is_open() && OpenState::Opening.is_moving());
        assert!(!OpenState::Closing.is_open() && OpenState::Closing.is_moving());
    }
}
