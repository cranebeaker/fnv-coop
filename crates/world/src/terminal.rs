//! Computer terminals (`TERM`) and the notes they show (`NOTE`).
//!
//! A terminal: `DESC` the header, `DNAM` (hacking difficulty u8: 0 very
//! easy … 4 very hard; flags u8: 0x02 unlocked; server type u8), then its
//! menu items, each `ITXT` the item's text, `RNAM` what it prints when
//! picked, `ANAM` flags, `INAM` a note to show, `TNAM` a sub-menu (another
//! terminal), a result script (`SCHR` … `SCTX`) and `CTDA` conditions
//! deciding whether it's listed. Read from the game's terminals:
//! `CampSearchlightFireChief` shows the note "Chief Fire Officer Report";
//! `P04CompanionFireTerminal` (flags 0x02) leads to a sub-menu. A note:
//! `FULL` its title, `DATA` its kind (0 sound, 1 text, 2 image, 3 voice),
//! `TNAM` the text.
//!
//! Getting in (`00966c60`, read from `FalloutNV.exe` 1.4.0.525): open
//! when the player holds the terminal's password note (`PNAM`), or it was
//! hacked, or it's unlocked (its `DNAM` flag 0x02, or a script's
//! `Unlock`); locked out when it locked the player out before (once only,
//! with a perk's "Ignore Locked Terminal": Computer Whiz) or its level is
//! 5 ("requires key"); otherwise the hacking game (`world::hacking`) when
//! the player's Science reaches `fHackingMinSkill<difficulty>` (0, 25, 50,
//! 75, 100), else "A Science skill of N is required to hack this
//! terminal." (`sHackIneligible`). What each placed terminal remembers is
//! the game's `ExtraTerminalState` ([`TerminalState`]); its lock level is
//! kept with every other lock in `GameState::locks`.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::{Condition, PLAYER_REF};
use crate::scripting::GameState;

const TERM: FourCC = FourCC::new(b"TERM");
const NOTE: FourCC = FourCC::new(b"NOTE");

/// `DNAM` flag: no hacking needed (`005015d0`).
pub const UNLOCKED: u8 = 0x02;
/// `DNAM` flag: the difficulty rises with the level (`00501250`).
pub const LEVELED: u8 = 0x01;
/// The lock level that only a key opens (the hacking menu shows the
/// terminal locked).
pub const REQUIRES_KEY: u8 = 5;

/// The terminal menu's number (`ComputersMenu`, `&ComputersMenu;`).
pub const TERMINAL_MENU: u16 = 1057;

/// One menu item.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TerminalItem {
    pub text: String,
    /// Printed when picked.
    pub result: Option<String>,
    pub flags: u8,
    pub note: Option<FormId>,
    pub submenu: Option<FormId>,
    pub script: Option<String>,
    pub conditions: Vec<Condition>,
}

/// A terminal's screen.
#[derive(Debug, Clone, PartialEq)]
pub struct Terminal {
    pub form_id: FormId,
    pub name: String,
    pub header: String,
    pub difficulty: u8,
    pub flags: u8,
    /// Its server type (`DNAM` byte 2, `+0xA6`): which
    /// `sTerminalServerText` the screen shows.
    pub server: u8,
    /// The note whose holder gets in without hacking (`PNAM`, the
    /// terminal's `+0xA0`).
    pub password_note: Option<FormId>,
    pub items: Vec<TerminalItem>,
}

impl Terminal {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Terminal> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == TERM)?;
        let record = rr.record().ok()?;
        let text = |s: &esm::Subrecord| {
            esm::text::decode_cp1252(s.data.strip_suffix(&[0]).unwrap_or(&s.data))
        };
        let form = |s: &esm::Subrecord| {
            (s.data.len() >= 4)
                .then(|| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                .filter(|f| f.0 != 0)
        };
        let dnam = record
            .get(FourCC::new(b"DNAM"))
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let mut items: Vec<TerminalItem> = Vec::new();
        for sub in &record.subrecords {
            match sub.kind.as_bytes() {
                b"ITXT" => items.push(TerminalItem {
                    text: text(sub),
                    ..TerminalItem::default()
                }),
                b"RNAM" => {
                    if let Some(i) = items.last_mut() {
                        i.result = Some(text(sub)).filter(|t| !t.trim().is_empty());
                    }
                }
                b"ANAM" => {
                    if let Some(i) = items.last_mut() {
                        i.flags = sub.data.first().copied().unwrap_or(0);
                    }
                }
                b"INAM" => {
                    if let Some(i) = items.last_mut() {
                        i.note = form(sub);
                    }
                }
                b"TNAM" => {
                    if let Some(i) = items.last_mut() {
                        i.submenu = form(sub);
                    }
                }
                b"SCTX" => {
                    if let Some(i) = items.last_mut() {
                        i.script = Some(text(sub)).filter(|t| !t.trim().is_empty());
                    }
                }
                b"CTDA" => {
                    if let Some(i) = items.last_mut() {
                        i.conditions
                            .extend(crate::dialogue::read_condition(&rr, &sub.data));
                    }
                }
                _ => {}
            }
        }
        Some(Terminal {
            form_id: id,
            name: record.full_name().unwrap_or_default(),
            header: record
                .get(FourCC::new(b"DESC"))
                .map(text)
                .unwrap_or_default(),
            difficulty: dnam.first().copied().unwrap_or(0),
            flags: dnam.get(1).copied().unwrap_or(0),
            server: dnam.get(2).copied().unwrap_or(0),
            password_note: record.get(FourCC::new(b"PNAM")).and_then(form),
            items,
        })
    }

    /// Whether its record says it opens without hacking.
    pub fn unlocked(&self) -> bool {
        self.flags & UNLOCKED != 0
    }

    /// Whether its difficulty rises with the level.
    pub fn leveled(&self) -> bool {
        self.flags & LEVELED != 0
    }
}

/// What a placed terminal remembers (the game's `ExtraTerminalState`,
/// extra data 0x50: a flags byte, 0x80 hacked and the rest the lockouts,
/// and the lock level, which lives in `GameState::locks` here).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerminalState {
    /// Hacked (`005018b0`).
    pub hacked: bool,
    /// Times it locked the player out (`00501930`).
    pub lockouts: u8,
}

/// How the player gets into a terminal (`00966c60`'s answers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// Straight in: the password note held (0), or hacked or unlocked (1).
    Open,
    /// Locked out (2): the hacking menu opens on "TERMINAL LOCKED".
    LockedOut,
    /// The hacking game (3).
    Hack,
    /// Not enough Science (4): the Science it takes.
    NeedsScience(u16),
}

/// The terminal's own level unless a script locked it at another
/// (`00501a30`: the reference's level when 0 or more).
pub fn lock_level(state: &GameState, terminal: &Terminal, reference: FormId) -> u8 {
    match state.locks.get(&reference) {
        Some(Some(level)) => *level,
        _ => terminal.difficulty,
    }
}

/// Unlocked (`00501ae0`): a script's `Unlock` (level −2), its record's
/// flag (level −1, the record's own), never at a level a script set.
pub fn unlocked(state: &GameState, terminal: &Terminal, reference: FormId) -> bool {
    match state.locks.get(&reference) {
        Some(None) => true,
        Some(Some(_)) => false,
        None => terminal.unlocked(),
    }
}

/// The difficulty the hacking game is set at (`005011a0`): the lock level;
/// for a leveled terminal plus ⌊level × `fHackLevelMult`⌋ (0.25, exe
/// default), at most 4. The level is the reference's encounter zone's,
/// else the player's (`00567e10`); zone levels aren't here, so always the
/// player's.
pub fn difficulty(
    order: &LoadOrder,
    state: &GameState,
    terminal: &Terminal,
    reference: FormId,
) -> u8 {
    let level = lock_level(state, terminal, reference);
    if !terminal.leveled() {
        return level;
    }
    let mult = crate::scripting::game_setting(order, "fHackLevelMult").unwrap_or(0.25);
    let extra = (f64::from(state.player_level.max(1)) * f64::from(mult)).trunc() as i64;
    (i64::from(level) + extra).clamp(0, 4) as u8
}

/// "Ignore Locked Terminal".
const IGNORE_LOCKED_TERMINAL: u8 = 20;

/// Locked out (`00501990`): once, unless a perk's "Ignore Locked
/// Terminal" (entry point 20, from 0) is other than 0; twice, for good.
pub fn locked_out(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    match state
        .terminal_states
        .get(&reference)
        .map_or(0, |t| t.lockouts & 0x7F)
    {
        0 => false,
        1 => {
            crate::perks::apply_for(order, state, PLAYER_REF, IGNORE_LOCKED_TERMINAL, 0.0, &[])
                == 0.0
        }
        _ => true,
    }
}

/// The player's Science as the game asks it here (`0066ef50`: the current
/// value, clamped to 0..100).
pub fn science(order: &LoadOrder, state: &GameState) -> f32 {
    crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, SCIENCE)
    .unwrap_or(0.0)
    .clamp(0.0, 100.0) as f32
}

/// How the player gets in (`00966c60`).
pub fn access(
    order: &LoadOrder,
    state: &GameState,
    terminal: &Terminal,
    reference: FormId,
) -> Access {
    if terminal
        .password_note
        .is_some_and(|n| state.notes.contains(&n))
    {
        return Access::Open;
    }
    let hacked = state
        .terminal_states
        .get(&reference)
        .is_some_and(|t| t.hacked);
    if hacked || unlocked(state, terminal, reference) {
        return Access::Open;
    }
    if locked_out(order, state, reference) || lock_level(state, terminal, reference) == REQUIRES_KEY
    {
        return Access::LockedOut;
    }
    let needed = crate::hacking::min_skill(difficulty(order, state, terminal, reference));
    if science(order, state) >= needed {
        Access::Hack
    } else {
        // `00501310`: rounded (`004bd510(needed, 1)`) for the message.
        Access::NeedsScience((needed + 0.5).floor().min(f32::from(u16::MAX)) as u16)
    }
}

/// `iXPRewardHackComputer…`'s exe defaults (the official data sets them).
const HACK_XP_DEFAULTS: [f32; 5] = [5.0, 10.0, 20.0, 30.0, 50.0];

/// The hacking game won (`00766b80`): the terminal stays hacked, the
/// experience for its difficulty (`006705b0(2, …)`: the reward beside the
/// first of `iXPLevelHackComputer…` (0–4) the difficulty doesn't pass,
/// `iXPRewardHackComputer…`), and "Computers Hacked" + 1; then, for a
/// terminal with an owner, the trespass alarm with the terminal and its
/// owner (`008c0ec0(terminal, owner, -1)`, `world::living::trespass::
/// alarm`: whoever sees the player and is the owner's, or in the owning
/// faction, turns on them; a minor crime). The game doesn't ask whether
/// the player may use the owner's things there.
pub fn hacked(order: &LoadOrder, state: &mut GameState, terminal: &Terminal, reference: FormId) {
    let d = f32::from(difficulty(order, state, terminal, reference));
    state.terminal_states.entry(reference).or_default().hacked = true;
    let mut reward = None;
    let mut last = 0.0;
    for (i, name) in crate::experience::DIFFICULTIES.iter().enumerate() {
        let level = crate::scripting::game_setting(order, &format!("iXPLevelHackComputer{name}"))
            .unwrap_or(i as f32);
        let xp = crate::scripting::game_setting(order, &format!("iXPRewardHackComputer{name}"))
            .unwrap_or(HACK_XP_DEFAULTS[i]);
        last = xp;
        if d <= level {
            reward = Some(xp);
            break;
        }
    }
    crate::experience::reward(order, state, f64::from(reward.unwrap_or(last)));
    crate::stats::bump(state, crate::stats::COMPUTERS_HACKED, 1);
    // Translated from 00766b80 (decompiled, FalloutNV.exe 1.4.0.525)
    if let Some(owner) = crate::crime::owner_of(order, state, reference) {
        crate::living::trespass::alarm(order, state, reference, Some(owner));
    }
}

/// The player's use of a terminal goes through to its menu or the hacking
/// game (`00501310`, `BGSTerminal::Activate`, past its refusals): someone
/// else's terminal (its owner, `world::crime::owner_of`, one whose things
/// the player may not use) costs the stealing karma
/// ([`crate::crime::stealing_karma`]) unless the record is flagged
/// unlocked (`005015d0`; the record's flag only, not a script's `Unlock`).
// Translated from 00501310 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn opened(order: &LoadOrder, state: &mut GameState, base: FormId, reference: FormId) {
    if Terminal::load(order, base).map_or(true, |t| t.unlocked()) {
        return;
    }
    let Some(owner) = crate::crime::owner_of(order, state, reference) else {
        return;
    };
    if !crate::crime::may_take(order, state, Some(owner)) {
        crate::crime::stealing_karma(order, state, owner);
    }
}

/// The hacking game lost (`00501930(ref, 1)`): one more lockout.
pub fn lock_out(state: &mut GameState, reference: FormId) {
    let t = state.terminal_states.entry(reference).or_default();
    t.lockouts = t.lockouts.wrapping_add(1) & 0x7F;
}

/// A script's `Lock` on a terminal (`005cbf80`): the level 0..5 (0 when
/// not given), and no longer hacked.
pub fn script_lock(state: &mut GameState, reference: FormId, level: i32) {
    state.locks.insert(reference, Some(level.clamp(0, 5) as u8));
    if let Some(t) = state.terminal_states.get_mut(&reference) {
        t.hacked = false;
    }
}

/// A script's `Unlock` on a terminal (`005cc120`): unlocked, and its
/// flags (hacked, lockouts) cleared.
pub fn script_unlock(state: &mut GameState, reference: FormId) {
    state.locks.insert(reference, None);
    state.terminal_states.remove(&reference);
}

/// `GetLocked` on a terminal (`0059d010`): 2 locked out, 1 not unlocked
/// (even when hacked), 0 unlocked.
pub fn get_locked(
    order: &LoadOrder,
    state: &GameState,
    terminal: &Terminal,
    reference: FormId,
) -> f64 {
    if access(order, state, terminal, reference) == Access::LockedOut {
        2.0
    } else if !unlocked(state, terminal, reference) {
        1.0
    } else {
        0.0
    }
}

/// `GetLockLevel` on a terminal (`0059d100`): −1 unlocked, else its
/// difficulty.
pub fn get_lock_level(
    order: &LoadOrder,
    state: &GameState,
    terminal: &Terminal,
    reference: FormId,
) -> f64 {
    if unlocked(state, terminal, reference) {
        -1.0
    } else {
        f64::from(difficulty(order, state, terminal, reference))
    }
}

/// The terminal a reference places, if it places one.
pub fn placed(order: &LoadOrder, reference: FormId) -> Option<Terminal> {
    let base = crate::scripting::base_of(order, reference)?;
    Terminal::load(order, base)
}

/// The Science skill's actor value.
pub const SCIENCE: u16 = 40;

/// An item's flags (`ANAM`, the item's `+0x74` in FalloutNV.exe): picking
/// it gives the player its note (`00758350`, 18 items in the official
/// data).
pub const ADD_NOTE: u8 = 0x01;
/// Picking it fills the list again, its conditions asked anew
/// (`00758390`; 137 items).
pub const FORCE_REDRAW: u8 = 0x02;

/// A note as a terminal shows it (`NOTE`: `DATA` its kind, `FULL`, and for
/// text `TNAM`, for an image `XNAM` the picture, for a sound `SNAM`).
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub kind: u8,
    pub title: String,
    pub text: Option<String>,
    pub image: Option<String>,
    pub sound: Option<FormId>,
}

/// Note kinds (`DATA`).
pub mod note_kind {
    pub const SOUND: u8 = 0;
    pub const TEXT: u8 = 1;
    pub const IMAGE: u8 = 2;
    pub const VOICE: u8 = 3;
}

/// A note record.
pub fn note(order: &LoadOrder, id: FormId) -> Option<Note> {
    let rr = order.get(id).filter(|r| r.entry.header.kind == NOTE)?;
    let record = rr.record().ok()?;
    let kind = record
        .get(esm::sig::DATA)
        .and_then(|s| s.data.first().copied())
        .unwrap_or(note_kind::TEXT);
    let text = |sig: &[u8; 4]| {
        record
            .get(FourCC::new(sig))
            .map(|s| esm::text::decode_cp1252(s.data.strip_suffix(&[0]).unwrap_or(&s.data)))
    };
    let form = |sig: &[u8; 4]| {
        record
            .get(FourCC::new(sig))
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            .filter(|f| f.0 != 0)
    };
    Some(Note {
        kind,
        title: record.full_name().unwrap_or_default(),
        text: (kind == note_kind::TEXT).then(|| text(b"TNAM")).flatten(),
        image: (kind == note_kind::IMAGE).then(|| text(b"XNAM")).flatten(),
        sound: (kind == note_kind::SOUND).then(|| form(b"SNAM")).flatten(),
    })
}

/// The items a terminal's screen lists now, with their places in the
/// record (`007586e0`: those whose conditions pass on the placed
/// terminal, `00680c30`).
pub fn shown_items(
    order: &LoadOrder,
    state: &GameState,
    terminal: FormId,
    reference: FormId,
) -> Vec<(usize, TerminalItem)> {
    let Some(t) = Terminal::load(order, terminal) else {
        return Vec::new();
    };
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    t.items
        .into_iter()
        .enumerate()
        .filter(|(_, i)| facts.conditions_pass(&i.conditions, reference, PLAYER_REF))
        .collect()
}

/// A note's title and text (`NOTE` `FULL`, `TNAM`); text notes only.
pub fn note_text(order: &LoadOrder, id: FormId) -> Option<(String, String)> {
    let rr = order.get(id).filter(|r| r.entry.header.kind == NOTE)?;
    let record = rr.record().ok()?;
    let text = record
        .get(FourCC::new(b"TNAM"))
        .map(|s| esm::text::decode_cp1252(s.data.strip_suffix(&[0]).unwrap_or(&s.data)))?;
    Some((record.full_name().unwrap_or_default(), text))
}
