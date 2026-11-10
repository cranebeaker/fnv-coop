//! Conformance rows of the world's state (SPEC-15 steps 1 to 4): the host's game finds a
//! change in the game's own map of changed forms, and a guest's game takes it as the game's
//! own change record.
//!
//! The rows need a backend with the game's own save code: the engine backend
//! ([`crate::engine_peer`]). On another backend each row is PENDING. Each row has a mutation
//! twin (`mutate`): the row breaks its own scenario, and then it must fail.
//!
//! Facts that the rows use (measured on the game program, 2026-11-10): `00104C13` is a
//! reference of Doc Mitchell's house (the Vigor Tester) with a 3D object while the player is
//! there. Its flags word (form +8) has bit 0x800 when it is disabled. Its 3D word (form
//! +0x64) is 0 with no 3D object. `Disable` marks it in the change map with flag bit 1.
//! It is a persistent reference: it stays in memory in each cell. `00104C16` is a temporary
//! reference of the same cell: it leaves memory when the player is in another cell and the
//! game's cell buffer is purged (the console's `pcb`).

use crate::backend::{
    bad, need, BackendPeer, ChangeRecord, CoopBackend, GuestRules, Test, Verdict,
};
use crate::data::Data;

/// A reference of the start cell, with a 3D object while the player is there.
const MACHINE: u32 = 0x0010_4c13;
/// A temporary reference of the start cell.
const LOOSE: u32 = 0x0010_4c16;
/// The start cell of each peer, and another interior cell.
const HOME: &str = "GSDocMitchellHouse";
const AWAY: &str = "GSProspectorSaloonInterior";
/// A form's flags word and its "disabled" bit. A reference's 3D word.
const FLAGS: u32 = 8;
const DISABLED: u32 = 0x800;
const NODE: u32 = 0x64;
/// The change flag of "enabled or disabled".
const CHANGE_ENABLE: u32 = 1;

fn disabled(p: &dyn BackendPeer, form: u32, when: &str) -> Result<bool, Verdict> {
    p.form_word(form, FLAGS)
        .map(|w| w & DISABLED != 0)
        .ok_or_else(|| bad(format!("{when}: the game has no form {form:08X}")))
}

/// The host disables a reference with the game's own command, and gives its record and the
/// index pairs that the record can name.
fn host_disables(
    host: &mut dyn BackendPeer,
    form: u32,
) -> Result<(ChangeRecord, Vec<(u32, u32)>), Verdict> {
    if !host.adapter_line(&format!("\"{form:08X}\".Disable")) || !host.frames(2) {
        return Err(bad("the host's game did not run the command"));
    }
    if !disabled(host, form, "on the host")? {
        return Err(bad("the host's command did not disable the reference"));
    }
    let record = host
        .record(form)
        .ok_or_else(|| bad("the host's game saved no record"))?;
    Ok((record, host.index_pairs(1)))
}

/// The host's own command marks the reference in the game's change map, with the flag of
/// its kind, and the map names nothing of it before. Twin: a baseline that is taken after
/// the change sees no change.
fn change_is_found(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let count = host
        .changes_baseline()
        .ok_or_else(|| Verdict::Pending("PENDING (this backend has no change map)".into()))?;
    if count == 0 {
        return Err(bad("the game's change map is empty at the start"));
    }
    let before = host.changes().unwrap_or_default();
    if before.iter().any(|(form, _)| *form == MACHINE) {
        return Err(bad("the map named the reference before its change"));
    }
    if !host.adapter_line(&format!("\"{MACHINE:08X}\".Disable")) || !host.frames(2) {
        return Err(bad("the host's game did not run the command"));
    }
    if mutate {
        host.changes_baseline();
    }
    let found = host.changes().unwrap_or_default();
    match found.iter().find(|(form, _)| *form == MACHINE) {
        Some((_, new)) if new & CHANGE_ENABLE != 0 => Ok(()),
        other => Err(bad(format!(
            "the change map does not name the disabled reference: {other:?} in {found:X?}"
        ))),
    }
}

/// The host's record of a disabled reference crosses to a guest in the same cell: the
/// guest's own load disables it and takes its 3D object away, and it stays so. Twin: a guest
/// that takes no record keeps the reference.
fn record_disables_on_guest(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    if guest
        .form_word(MACHINE, NODE)
        .ok_or_else(|| bad("no reference on the guest"))?
        == 0
    {
        return Err(bad(
            "the guest's reference has no 3D object at the start: the row proves nothing",
        ));
    }
    let (record, pairs) = host_disables(&mut *host, MACHINE)?;
    if record.flags & CHANGE_ENABLE == 0 || record.bytes.len() < 2 {
        return Err(bad(format!("the host's record is empty: {record:?}")));
    }
    if !mutate {
        if guest.take_pairs(&pairs).is_none() {
            return Err(bad("the guest's game took no index pair"));
        }
        match need(guest.take_record(&record))? {
            true => {}
            false => return Err(bad("the record waits, but the form is in memory")),
        }
    }
    if !guest.frames(120) {
        return Err(bad("the guest's game ran no frame"));
    }
    if !disabled(&*guest, MACHINE, "on the guest")? {
        return Err(bad("the reference is not disabled on the guest"));
    }
    if guest.form_word(MACHINE, NODE) != Some(0) {
        return Err(bad("the guest's reference still has its 3D object"));
    }
    Ok(())
}

/// A guest in another cell takes the host's record of a reference that its game does not
/// have in memory. The record waits, and the game's own load applies it when the guest goes
/// into the reference's cell. Twin: with no record the reference is not disabled there.
fn record_waits_for_its_cell(b: &dyn CoopBackend, d: &Data, mutate: bool) -> Result<(), Verdict> {
    let mut host = need(b.peer(d, true))?;
    let mut guest = need(b.guest(d, GuestRules::Policy))?;
    let (record, pairs) = host_disables(&mut *host, LOOSE)?;
    // The guest leaves, and its game forgets the cell.
    if !guest.enter(AWAY) || !guest.adapter_line("pcb") || !guest.frames(5) {
        return Err(bad(format!("the guest did not get into {AWAY}")));
    }
    if guest.has_form(LOOSE) {
        return Err(bad(
            "the guest's game still has the reference in memory: the row proves nothing",
        ));
    }
    if !mutate {
        if guest.take_pairs(&pairs).is_none() {
            return Err(bad("the guest's game took no index pair"));
        }
        if need(guest.take_record(&record))? {
            return Err(bad("the record was loaded, but the form is not in memory"));
        }
    }
    if !guest.enter(HOME) || !guest.frames(60) {
        return Err(bad(format!("the guest did not get back into {HOME}")));
    }
    if !disabled(&*guest, LOOSE, "on the guest, back in the cell")? {
        return Err(bad(
            "the reference is not disabled after the guest came into its cell",
        ));
    }
    Ok(())
}

/// The world's rows: id and function.
pub const TESTS: &[(&str, Test)] = &[
    ("change_is_found", change_is_found),
    ("record_disables_on_guest", record_disables_on_guest),
    ("record_waits_for_its_cell", record_waits_for_its_cell),
];
