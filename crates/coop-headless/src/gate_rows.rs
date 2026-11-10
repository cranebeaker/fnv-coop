//! Conformance rows of a guest's script gate (SPEC-04): what a guest's game runs itself,
//! what it leaves to the host, and what happens to a write that the gate did not stop.
//!
//! The rows need a backend that runs the game's own scripts frame by frame: the engine
//! backend ([`crate::engine_peer`]). On a backend with no script gate each row is PENDING.
//! Each row has a mutation twin (`mutate`): the row breaks its own scenario, and then it
//! must fail.
//!
//! Facts of the game's data that the rows use (measured on the game program, 2026-11-10):
//! - `VCG01` is a quest of each player in the policy. `SetStage VCG01 0` starts it, and its
//!   own script moves it to stage 1 in some hundred frames.
//! - The result of `VCG01` stage 200 starts the shared quest `VMQ01`, sets its stage 10, and
//!   sets the global `GameHour` to 8 with a `set` line (no function to guard).
//! - A game with no gate moves shared quest variables by its own quest scripts in 100
//!   frames of play.

use coop_core::ids::FormId as CForm;
use coop_core::schema::Field;

use crate::backend::{bad, need, snapshot_of, BackendPeer, CoopBackend, GuestRules, Test, Verdict};
use crate::data::Data;

/// A quest that each player runs alone (the policy's `per_player` scope).
const OWN_QUEST: &str = "VCG01";
/// A shared quest that the tests drive.
const SHARED_QUEST: &str = "VCG03";
/// The shared quest that the own quest's stage 200 moves.
const MAIN_QUEST: &str = "VMQ01";
/// The clock global that the own quest's stage 200 sets with a `set` line.
const HOUR: &str = "GameHour";
const STAGE: u16 = 10;
/// How far a guest's hour can be from the host's and be no leak, in these rows.
const HOUR_LIMIT: f32 = 0.5;
/// The field kind of a stage and of a global (`Field::kind`).
const KIND_STAGE: u8 = 0;
const KIND_GLOBAL: u8 = 3;

fn form(d: &Data, editor_id: &str) -> Result<u32, Verdict> {
    d.order
        .form_by_editor_id(editor_id)
        .map(|f| f.0)
        .ok_or_else(|| bad(format!("no form {editor_id} in this data")))
}

/// The console line that sets a quest's stage. The game's console takes a form id.
fn set_stage_line(d: &Data, quest: &str, stage: u16) -> Result<String, Verdict> {
    Ok(format!("SetStage {:08X} {stage}", form(d, quest)?))
}

/// The shared state without the globals: the game's clock moves them on each machine.
fn story(p: &dyn BackendPeer) -> Vec<(CForm, Field)> {
    p.export()
        .into_iter()
        .filter(|(_, f)| !matches!(f, Field::Global { .. }))
        .collect()
}

fn done(ok: bool, what: &str) -> Result<(), Verdict> {
    if ok {
        Ok(())
    } else {
        Err(bad(format!("{what}: the game did not do it")))
    }
}

/// The story slots of `now` that are not in `then`.
fn moved<'f>(now: &'f [(CForm, Field)], then: &[(CForm, Field)]) -> Vec<&'f (CForm, Field)> {
    now.iter().filter(|slot| !then.contains(slot)).collect()
}

/// The first slots of a list, for a failure's text.
fn names(slots: &[&(CForm, Field)]) -> String {
    let first: Vec<String> = slots
        .iter()
        .take(12)
        .map(|(form, field)| format!("{:08X} {field:?}", form.0))
        .collect();
    first.join(", ")
}

fn without_globals(fields: Vec<(CForm, Field)>) -> Vec<(CForm, Field)> {
    fields
        .into_iter()
        .filter(|(_, f)| !matches!(f, Field::Global { .. }))
        .collect()
}

/// From the main menu through the cell's load and 200 frames of play, the host's own quest
/// scripts move the shared state. A guest's game runs the same frames and moves no shared
/// slot, and its object scripts run (the gate is selective). The host's snapshot then makes
/// the two equal. Twin: a guest with no gate moves shared slots alone.
fn guest_runs_no_shared_quest_script(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    const FRAMES: u32 = 200;
    let mut host = need(b.peer(d, true))?;
    let rules = if mutate {
        GuestRules::Open
    } else {
        GuestRules::Policy
    };
    let mut guest = need(b.guest(d, rules))?;
    let start = |p: &dyn BackendPeer| {
        p.start_state()
            .map(without_globals)
            .ok_or_else(|| bad("the backend does not keep the state at the start"))
    };
    let (host_start, guest_start) = (start(&*host)?, start(&*guest)?);
    done(host.frames(FRAMES), "the host's frames")?;
    done(guest.frames(FRAMES), "the guest's frames")?;
    if moved(&story(&*host), &host_start).is_empty() {
        return Err(bad(
            "the host's own scripts moved no shared slot: the row proves nothing",
        ));
    }
    let guest_now = story(&*guest);
    let alone = moved(&guest_now, &guest_start);
    if !alone.is_empty() {
        return Err(bad(format!(
            "the guest's game moved {} shared slots alone: {}",
            alone.len(),
            names(&alone)
        )));
    }
    let stats = guest
        .gate_stats()
        .ok_or_else(|| bad("the guest has no gate"))?;
    if stats.stopped == 0 {
        return Err(bad("no script of a shared quest reached the gate"));
    }
    if stats.runs <= stats.stopped {
        return Err(bad(
            "the gate stopped each script: an object's script must run on a guest",
        ));
    }
    if stats.blind {
        return Err(bad("the gate says that it is blind"));
    }
    guest.apply_snapshot(&snapshot_of(&*host));
    if guest.shard_hashes() != host.shard_hashes() {
        return Err(bad("the guest does not equal the host after the snapshot"));
    }
    Ok(())
}

/// A quest of each player runs on the guest: its own script moves its stage, and no shared
/// slot moves. Twin: a gate with no local quest holds it at stage 0.
fn guest_runs_local_quest(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let rules = if mutate {
        GuestRules::Closed
    } else {
        GuestRules::Policy
    };
    let mut guest = need(b.guest(d, rules))?;
    let before = story(&*guest);
    done(
        guest.script_line(&set_stage_line(d, OWN_QUEST, 0)?),
        "the script line",
    )?;
    // The quest's own script moves it to stage 1 in some hundred frames.
    let mut stage = guest.local_stage(OWN_QUEST);
    for _ in 0..15 {
        if stage.is_some_and(|s| s >= 1) {
            break;
        }
        done(guest.frames(100), "the guest's frames")?;
        stage = guest.local_stage(OWN_QUEST);
    }
    if !stage.is_some_and(|s| s >= 1) {
        return Err(bad(format!(
            "the guest's own quest {OWN_QUEST} is at stage {stage:?}, want 1 or more"
        )));
    }
    if story(&*guest) != before {
        return Err(bad("the guest's own quest moved a shared slot"));
    }
    Ok(())
}

/// A local stage result that sets a shared quest's stage runs on the guest, the shared
/// quest does not move there, and the host gets the change as one intent. Twin: with the
/// shared quest in the local rules, it moves on the guest.
fn local_script_cannot_move_shared_quest(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    let rules = if mutate {
        GuestRules::PolicyAnd(&[MAIN_QUEST])
    } else {
        GuestRules::Policy
    };
    let mut guest = need(b.guest(d, rules))?;
    guest.intents();
    done(
        guest.script_line(&set_stage_line(d, OWN_QUEST, 200)?),
        "the script line",
    )?;
    done(guest.frames(3), "the guest's frames")?;
    if guest.local_stage(OWN_QUEST) != Some(200) {
        return Err(bad(format!(
            "the guest's own quest did not take its stage: {:?}",
            guest.local_stage(OWN_QUEST)
        )));
    }
    if let Some(stage) = guest.stage(MAIN_QUEST) {
        return Err(bad(format!(
            "a local script moved the shared quest {MAIN_QUEST} to stage {stage} on the guest"
        )));
    }
    let want = format!("stage:{:08X}:10", form(d, MAIN_QUEST)?);
    let intents = guest.intents();
    if intents.iter().filter(|i| **i == want).count() != 1 {
        return Err(bad(format!("want one intent {want}, got {intents:?}")));
    }
    Ok(())
}

/// A script's `set` line writes a shared global on the guest: no function can be guarded.
/// The audit names the global, and the host's value comes back. Twin: an audit that only
/// reports leaves the guest's value different.
fn inline_global_is_repaired(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let host = need(b.peer(d, true))?;
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    guest.apply_snapshot(&snapshot_of(&*host));
    done(
        guest.set_moving(&[(HOUR, HOUR_LIMIT), ("GameDaysPassed", 0.05)]),
        "the clock's limits",
    )?;
    let hour = |p: &dyn BackendPeer| p.global(HOUR).ok_or_else(|| bad("no GameHour"));
    let host_hour = hour(&*host)?;
    done(
        guest.script_line(&set_stage_line(d, OWN_QUEST, 200)?),
        "the script line",
    )?;
    done(guest.frames(1), "the guest's frame")?;
    if (hour(&*guest)? - host_hour).abs() <= HOUR_LIMIT {
        return Err(bad(
            "the script's `set` line did not move the guest's hour: the row proves nothing",
        ));
    }
    let report = guest
        .audit(!mutate)
        .ok_or_else(|| bad("the guest has no audit"))?;
    if !report.named.contains(&(form(d, HOUR)?, KIND_GLOBAL)) {
        return Err(bad(format!("the audit did not name the hour: {report:?}")));
    }
    let now = hour(&*guest)?;
    if (now - host_hour).abs() > HOUR_LIMIT {
        return Err(bad(format!(
            "the guest's hour is {now}, the host's is {host_hour}"
        )));
    }
    Ok(())
}

/// A guest's script asks for a shared stage (two times: a stopped script asks again). The
/// guest's stage does not move, the host gets one intent and runs it, and the snapshot
/// brings the stage to the guest. Twin: with the quest in the local rules, the guest moves
/// it alone.
///
/// The script line is the stand-in for an object's script. A real activator of a shared
/// quest is a row of the activation slice.
fn guest_stage_ask_moves_host_once(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let rules = if mutate {
        GuestRules::PolicyAnd(&[SHARED_QUEST])
    } else {
        GuestRules::Policy
    };
    let mut guest = need(b.guest(d, rules))?;
    guest.apply_snapshot(&snapshot_of(&*host));
    guest.intents();
    let line = set_stage_line(d, SHARED_QUEST, STAGE)?;
    done(guest.script_line(&line), "the script line")?;
    done(guest.script_line(&line), "the script line again")?;
    if guest.stage(SHARED_QUEST).is_some() {
        return Err(bad("the guest's game set the shared stage itself"));
    }
    let want = format!("stage:{:08X}:{STAGE}", form(d, SHARED_QUEST)?);
    let intents = guest.intents();
    if intents != [want.clone()] {
        return Err(bad(format!("want the one intent {want}, got {intents:?}")));
    }
    // The host decides, and its game runs the change once.
    if !host.set_stage(SHARED_QUEST, STAGE) {
        return Err(bad("the host refused the guest's intent"));
    }
    guest.apply_snapshot(&snapshot_of(&*host));
    if guest.stage(SHARED_QUEST) != Some(STAGE) || guest.shard_hashes() != host.shard_hashes() {
        return Err(bad("the guest does not equal the host after the snapshot"));
    }
    if !guest.intents().is_empty() {
        return Err(bad("an intent came again after the host's answer"));
    }
    Ok(())
}

/// With no gate (each override off), a script on the guest sets a shared stage. The audit
/// names the quest, and the guest equals the host again. Twin: with no audit the guest
/// stays different.
fn audit_names_leak_when_gate_is_blind(
    b: &dyn CoopBackend,
    d: &Data,
    mutate: bool,
) -> Result<(), Verdict> {
    let host = need(b.peer(d, true))?;
    let mut guest = need(b.guest(d, GuestRules::Open))?;
    guest.apply_snapshot(&snapshot_of(&*host));
    done(
        guest.script_line(&set_stage_line(d, SHARED_QUEST, STAGE)?),
        "the script line",
    )?;
    if guest.stage(SHARED_QUEST) != Some(STAGE) {
        return Err(bad(
            "with no gate the stage did not move: the row proves nothing",
        ));
    }
    if !mutate {
        let report = guest
            .audit(true)
            .ok_or_else(|| bad("the guest has no audit"))?;
        if !report.named.contains(&(form(d, SHARED_QUEST)?, KIND_STAGE)) {
            return Err(bad(format!("the audit did not name the quest: {report:?}")));
        }
        if report.restored == 0 {
            return Err(bad("the audit gave no value back"));
        }
    }
    if guest.shard_hashes() != host.shard_hashes() {
        return Err(bad("the guest does not equal the host after the audit"));
    }
    Ok(())
}

/// The adapter's own console line changes a gated guest's game, and it is no intent. Twin:
/// the same line as a script's is held.
fn adapter_line_passes_gate(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    guest.intents();
    let line = set_stage_line(d, SHARED_QUEST, STAGE)?;
    let ran = if mutate {
        guest.script_line(&line)
    } else {
        guest.adapter_line(&line)
    };
    done(ran, "the console line")?;
    if guest.stage(SHARED_QUEST) != Some(STAGE) {
        return Err(bad(
            "the adapter's own line did not change the guest's game",
        ));
    }
    if !guest.intents().is_empty() {
        return Err(bad("the adapter's own line became an intent"));
    }
    Ok(())
}

/// The gate's rows: id and function.
pub const TESTS: &[(&str, Test)] = &[
    (
        "guest_runs_no_shared_quest_script",
        guest_runs_no_shared_quest_script,
    ),
    ("guest_runs_local_quest", guest_runs_local_quest),
    (
        "local_script_cannot_move_shared_quest",
        local_script_cannot_move_shared_quest,
    ),
    ("inline_global_is_repaired", inline_global_is_repaired),
    (
        "guest_stage_ask_moves_host_once",
        guest_stage_ask_moves_host_once,
    ),
    (
        "audit_names_leak_when_gate_is_blind",
        audit_names_leak_when_gate_is_blind,
    ),
    ("adapter_line_passes_gate", adapter_line_passes_gate),
];
