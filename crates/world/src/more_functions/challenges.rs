//! Challenges (`CHAL`), read from the game's code: how they count up, are
//! completed and recur (`005f5950`, `005f6110`), and the script functions
//! about them.
//!
//! The record: `FULL` name, `SCRI` the script run when one is completed,
//! `DESC` its description, `DATA` (24 bytes, kept as is at the form's
//! +0x54): kind u32 (0 kill from a form list, 1 kill a form, 2 kill any of
//! a kind, 3 hit an enemy, 4 discover a map marker, 5 use an item, 6
//! acquire an item, 7 use a skill, 8 do damage, 9 use an item from a list,
//! 10 acquire an item from a list, 11 a miscellaneous statistic, 12 craft,
//! 13 scripted), threshold u32, flags (0x01 starts disabled, 0x02
//! recurring), interval u32, three values u16 at 16, 18, 20; `SNAM` and
//! `XNAM` two forms (+0x74, +0x78). At run time: the count (+0x6c) and
//! flags (+0x70: 0x01 unlocked, 0x02 completed, 0x04 has recurred, 0x08 no
//! longer recurs), saved.
//!
//! A challenge counts when it isn't completed (or recurs) and doesn't
//! start disabled (or has been unlocked). Each time it counts below its
//! threshold, the count's notice ("Name   3\5" and the description) shows
//! whenever the count is a multiple of the interval (100 when 0). On
//! reaching the threshold: its script runs on the player (its
//! `ScriptEffectStart` blocks: `ChallengeHighXPSCRIPT` gives 100 XP), the
//! statistic "Challenges Completed" goes up (not for a challenge counting
//! that statistic), the completion notice shows and the `UILevelUp` sound
//! plays; then it's completed, or if it recurs (and still may) it starts
//! over with what went past the threshold.

use std::collections::BTreeMap;

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, GameState, Runner};

const CHAL: FourCC = FourCC::new(b"CHAL");
const DATA: FourCC = FourCC::new(b"DATA");
const DESC: FourCC = FourCC::new(b"DESC");
const SCRI: FourCC = FourCC::new(b"SCRI");
const SNAM: FourCC = FourCC::new(b"SNAM");
const XNAM: FourCC = FourCC::new(b"XNAM");

/// The kinds a challenge counts (`DATA` u32 at 0).
pub mod kinds {
    pub const KILL_FROM_LIST: u32 = 0;
    pub const KILL_FORM: u32 = 1;
    pub const KILL_KIND: u32 = 2;
    pub const HIT_ENEMY: u32 = 3;
    pub const MAP_MARKER: u32 = 4;
    pub const USE_ITEM: u32 = 5;
    pub const ACQUIRE_ITEM: u32 = 6;
    pub const USE_SKILL: u32 = 7;
    pub const DO_DAMAGE: u32 = 8;
    pub const USE_FROM_LIST: u32 = 9;
    pub const ACQUIRE_FROM_LIST: u32 = 10;
    pub const MISC_STAT: u32 = 11;
    pub const CRAFT: u32 = 12;
    pub const SCRIPTED: u32 = 13;
}

/// Run-time flags (+0x70).
pub mod flags {
    pub const UNLOCKED: u32 = 0x01;
    pub const COMPLETED: u32 = 0x02;
    pub const RECURRED: u32 = 0x04;
    pub const NO_RECUR: u32 = 0x08;
}

/// "Challenges Completed", the statistic completing one bumps (`004d5c60(
/// 0x1b)`).
pub const CHALLENGES_COMPLETED: u8 = 27;

/// A challenge's record.
#[derive(Debug, Clone, PartialEq)]
pub struct Challenge {
    pub form_id: FormId,
    pub name: String,
    pub description: String,
    pub script: Option<FormId>,
    pub kind: u32,
    pub threshold: i32,
    /// 0x01 starts disabled, 0x02 recurring.
    pub flags: u32,
    pub interval: i32,
    pub values: [u16; 3],
    pub forms: [Option<FormId>; 2],
    /// Its picture (`ICON`), if it has one.
    pub icon: Option<String>,
}

impl Challenge {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Challenge> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == CHAL)?;
        let record = rr.record().ok()?;
        let form = |kind: FourCC| {
            record
                .get(kind)
                .filter(|s| s.data.len() >= 4)
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                .filter(|f| f.0 != 0)
        };
        let data = record.get(DATA).map(|s| s.data.clone()).unwrap_or_default();
        let u = |at: usize| {
            if data.len() >= at + 4 {
                le_u32(&data, at)
            } else {
                0
            }
        };
        let h = |at: usize| {
            if data.len() >= at + 2 {
                u16::from_le_bytes([data[at], data[at + 1]])
            } else {
                0
            }
        };
        Some(Challenge {
            form_id: id,
            name: record.full_name().unwrap_or_default(),
            description: record.get(DESC).map(|s| s.zstring()).unwrap_or_default(),
            script: form(SCRI),
            kind: u(0),
            threshold: u(4) as i32,
            flags: u(8),
            interval: u(12) as i32,
            values: [h(16), h(18), h(20)],
            forms: [form(SNAM), form(XNAM)],
            icon: record
                .get(FourCC::new(b"ICON"))
                .map(|s| s.zstring())
                .filter(|p| !p.trim().is_empty()),
        })
    }

    fn recurring(&self) -> bool {
        self.flags & 0x02 != 0
    }

    fn starts_disabled(&self) -> bool {
        self.flags & 0x01 != 0
    }
}

/// Every challenge of a kind, in load order (the game keeps one list per
/// kind, `011cb3e0` + kind × 8).
pub fn of_kind(order: &LoadOrder, kind: u32) -> Vec<Challenge> {
    order
        .records_of_type(CHAL)
        .filter_map(|rr| Challenge::load(order, rr.form_id))
        .filter(|c| c.kind == kind)
        .collect()
}

/// What challenges keep while playing: each one's count and flags
/// (+0x6c, +0x70), and statistics bumped since challenges last looked
/// (`world::stats::bump` has no load order to look them up with).
#[derive(Debug, Clone, Default)]
pub struct Challenges {
    pub progress: BTreeMap<FormId, (i32, u32)>,
    pub stat_bumps: Vec<(u8, i64)>,
}

/// A challenge's run-time flags: as kept, with `UnlockChallenge`'s unlock
/// (`world::script_functions`, which keeps it in its own set).
pub fn flags_of(state: &GameState, challenge: FormId) -> u32 {
    let unlocked = if state.set_by_scripts.challenges.contains(&challenge) {
        flags::UNLOCKED
    } else {
        0
    };
    state
        .more
        .challenges
        .progress
        .get(&challenge)
        .map_or(0, |p| p.1)
        | unlocked
}

fn set_flag(state: &mut GameState, challenge: FormId, flag: u32, on: bool) {
    let p = state.more.challenges.progress.entry(challenge).or_default();
    if on {
        p.1 |= flag;
    } else {
        p.1 &= !flag;
    }
}

/// Whether a challenge counts now (`005f6110`, `005f5950`): not completed
/// (or recurring), and not starting disabled (or unlocked).
fn counts(state: &GameState, c: &Challenge) -> bool {
    let f = flags_of(state, c.form_id);
    (f & flags::COMPLETED == 0 || c.recurring())
        && (!c.starts_disabled() || f & flags::UNLOCKED != 0)
}

/// The count's notice: "Name   count\threshold" and the description, on
/// two lines (`"%s   %d\\%d\n%s"`); only for a challenge with a name and a
/// description.
///
/// Its picture is the challenge's own icon (`0048e730`: its `ICON`; none
/// in vanilla's, so type 0's neutral Vault Boy).
fn notice(state: &mut GameState, c: &Challenge, count: i32) {
    if c.name.is_empty() || c.description.is_empty() {
        return;
    }
    state.events.push(Event::Message {
        title: None,
        text: format!("{}   {}\\{}\n{}", c.name, count, c.threshold, c.description),
        buttons: Vec::new(),
        icon: c.icon.clone(),
    });
}

/// `amount` more of a challenge that counts (the common part of `005f5950`
/// and `005f6110`).
fn advance(runner: &mut Runner, c: &Challenge, amount: i32) {
    let id = c.form_id;
    let count = {
        let p = runner.state.more.challenges.progress.entry(id).or_default();
        p.0 += amount;
        p.0
    };
    if count < c.threshold {
        let interval = if c.interval == 0 { 100 } else { c.interval };
        // The game counts down from the count by the interval and shows the
        // notice on reaching exactly 0.
        if interval > 0 && count >= 0 && count % interval == 0 {
            notice(runner.state, c, count);
        }
        return;
    }
    // Completed: the script on the player.
    if let Some(script) = c.script {
        run_completion_script(runner, script);
    }
    if !(c.kind == kinds::MISC_STAT && c.values[0] == u16::from(CHALLENGES_COMPLETED)) {
        crate::stats::bump(runner.state, CHALLENGES_COMPLETED, 1);
    }
    notice(runner.state, c, c.threshold);
    if let Some(sound) = runner.order.form_by_editor_id("UILevelUp") {
        runner.state.events.push(Event::Sound(sound));
    }
    let f = flags_of(runner.state, id);
    if !c.recurring() || f & flags::NO_RECUR != 0 {
        set_flag(runner.state, id, flags::COMPLETED, true);
    } else {
        let p = runner.state.more.challenges.progress.entry(id).or_default();
        p.0 = count - c.threshold;
        p.1 |= flags::RECURRED;
    }
}

/// The challenge's script, run on the player as a script effect would run
/// it (`005ac340`): its `ScriptEffectStart` blocks.
fn run_completion_script(runner: &mut Runner, script: FormId) {
    let Some(s) = runner.scripts.script(runner.order, script) else {
        return;
    };
    let mut locals = script::interp::Locals::new(&s);
    let saved = (runner.this, runner.owner);
    runner.this = Some(PLAYER_REF);
    runner.owner = None;
    script::interp::run_blocks(&s, "scripteffectstart", |_| true, &mut locals, runner);
    (runner.this, runner.owner) = saved;
}

/// `IncrementScriptedChallenge challenge` (`005dfe30` → `005f6110(c, 1)`):
/// the scripted challenge counts one more if it counts now.
pub fn increment_scripted(runner: &mut Runner, challenge: FormId, amount: i32) {
    let Some(c) = Challenge::load(runner.order, challenge) else {
        return;
    };
    if c.kind != kinds::SCRIPTED || !counts(runner.state, &c) {
        return;
    }
    advance(runner, &c, amount);
}

/// What happened, as `005f5950` hears it: a kind of thing, how much, the
/// forms involved and up to three values; every challenge of that kind
/// whose forms and values match counts it. Values: a challenge's value 1
/// must match when set (for statistics even when 0), value 2 likewise (an
/// enemy hit, kind 3, only matches 0 or 1 when the challenge leaves it 0),
/// value 3 when set; its first form must be (or, for the list kinds 0, 9,
/// 10, list) the first form given, and its second form list the second
/// form for kinds 0–4 and 8.
#[allow(clippy::too_many_arguments)]
pub fn happened(
    runner: &mut Runner,
    kind: u32,
    amount: i32,
    form: Option<FormId>,
    second: Option<FormId>,
    values: [u16; 3],
) {
    if kind >= 14 {
        return;
    }
    let order = runner.order;
    for c in of_kind(order, kind) {
        if !counts(runner.state, &c) {
            continue;
        }
        if let Some(own) = c.forms[0] {
            let Some(given) = form else { continue };
            let matches = match kind {
                kinds::KILL_FROM_LIST | kinds::USE_FROM_LIST | kinds::ACQUIRE_FROM_LIST => {
                    crate::script_functions::form_list(order, runner.state, own).contains(&given)
                }
                _ => given == own,
            };
            if !matches {
                continue;
            }
        }
        if let Some(own) = c.forms[1] {
            let Some(given) = second else { continue };
            if (kind < 5 || kind == kinds::DO_DAMAGE)
                && !crate::script_functions::form_list(order, runner.state, own).contains(&given)
            {
                continue;
            }
        }
        let [v1, v2, v3] = c.values;
        if v1 == 0 {
            if kind == kinds::MISC_STAT && values[0] != v1 {
                continue;
            }
        } else if values[0] == 0 || values[0] != v1 {
            continue;
        }
        if v2 == 0 {
            if kind == kinds::HIT_ENEMY && values[1] != 0 && values[1] != 1 {
                continue;
            }
        } else if values[1] == 0 || values[1] != v2 {
            continue;
        }
        if v3 != 0 && (values[2] == 0 || values[2] != v3) {
            continue;
        }
        advance(runner, &c, amount);
    }
}

/// Statistics bumped since challenges last looked count for the
/// "miscellaneous statistic" challenges (`004d5e10` calls `005f5950(11,
/// amount, 0, 0, statistic)` with every change).
pub fn catch_up(runner: &mut Runner) {
    let bumps = std::mem::take(&mut runner.state.more.challenges.stat_bumps);
    for (stat, by) in bumps {
        if by != 0 {
            happened(
                runner,
                kinds::MISC_STAT,
                by as i32,
                None,
                None,
                [u16::from(stat), 0, 0],
            );
        }
    }
}

/// `RemoveRecurringFromChallenge challenge` (`005def90` → `005defe0(1)`):
/// it no longer recurs (+0x70 flag 0x08).
pub fn stop_recurring(state: &mut GameState, challenge: FormId) {
    set_flag(state, challenge, flags::NO_RECUR, true);
}

/// `GetChallengeCompleted` as a script calls it (`005deef0`): completed, or
/// recurred at least once (flags 0x02, 0x04).
pub fn completed_for_scripts(state: &GameState, challenge: FormId) -> bool {
    flags_of(state, challenge) & (flags::COMPLETED | flags::RECURRED) != 0
}

/// `GetChallengeCompleted` as a condition asks it (`005a60f0`): the
/// completed flag itself, 0 or 2.
pub fn completed_for_conditions(state: &GameState, challenge: FormId) -> f64 {
    f64::from(flags_of(state, challenge) & flags::COMPLETED)
}
