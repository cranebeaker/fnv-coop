//! Being warned off someone's property, read from the game's code
//! (`findings\reputation.md` §3.6; the detection update `008f5480`, the
//! trespass package `009f9250` and its procedures `00918120` and
//! `008db240`, the alarm `008c0ec0` with `008c1010`, `008b8e90`,
//! `0047d740`, `008b7c00`).
//!
//! **Who comes** (`008f5480`): someone who detects the trespassing player
//! (above 0; trespassing as `world::crime::trespassing` has it) gives the
//! player a trespass package (type 0x17, one at a time) and takes it,
//! unless the cell's owner is evil and they aren't. Evil (`0047d740`,
//! `0047d7c0`): a faction flagged evil (`DATA` 0x02); a person whose
//! factions are all flagged evil (and who has at least one). The package
//! allows **3 warnings, or none in an "off limits" cell** (record flag
//! 0x20000, `00544490`).
//!
//! **The package** (`00918120`, `008db240`): they walk up to the player
//! until within 200 units (`010179e0`); there, with no warnings allowed
//! the alarm goes off at once; otherwise each time their timer has run out
//! they say the `GuardTrespass` topic (combat topic 20, `000000F2`), the
//! timer is set to `fAITrespassWarningTimer` (10 s, the exe's) and the
//! count goes up; once more warnings have been given than allowed and the
//! timer has run out, the alarm. So with 3 allowed: warnings at 0, 10, 20
//! and 30 s, the alarm at 40 s. The player leaving the property (no longer
//! trespassing, or another cell) ends it. `GetTrespassWarningLevel` on the
//! warner gives the warnings given.
//!
//! **The alarm** (`008c0ec0`, also `SendTrespassAlarm`): everyone loaded
//! who doesn't ignore crime, detects the player (above 0) and cares turns
//! on the player; the first of them has +1 minor crime and
//! `fReputationMinorCrimeNeg` infamy counted for their crime-tracking
//! factions (`008b7c00(1, 1)`), and the player +1 minor crime
//! (`008bff50`), once. Who cares (`008c1010`): someone in one of the
//! alarmer's factions; with an owner given (`SendTrespassAlarm` gives the
//! caller's own base), also someone in one of that person's factions, or
//! in the owning faction. Being in a faction there (`008b8e90`): the base
//! record lists it (whatever rank) and no script took them out, or a
//! script put them in.
//!
//! Not traced, and so not done: which of several people noticing at once
//! comes (the first whose detection update runs; here the first in the
//! cell's reference order); what follows an alarm (the package stays with
//! the player and others may take it over; here nothing more happens in
//! that cell while the player stays); a warner who dies or goes; "loaded"
//! people beyond the player's cell.

use esm::{FormId, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::{base_of, Event, Facts, GameState};

/// The game's hard-coded `GuardTrespass` topic.
pub const GUARD_TRESPASS: FormId = FormId(0xF2);

/// The trespass package's type (`009f9250`).
pub const PACKAGE_TYPE: u8 = 0x17;

/// How near the warner comes before warning (`00918120`: 200.0).
pub const WARNING_DISTANCE: f32 = 200.0;

/// The warnings a trespass package allows (`009f9250`'s last argument: 3,
/// or 0 in an off-limits cell).
pub const WARNINGS: u32 = 3;

/// The record flag of an "off limits" cell (`00544490`): no warnings.
const OFF_LIMITS: u32 = 0x2_0000;

/// A warning under way.
#[derive(Debug, Clone, PartialEq)]
pub struct Warning {
    /// Who comes to warn.
    pub warner: FormId,
    /// The cell trespassed in.
    pub cell: FormId,
    /// Seconds until the next warning (package+0x80).
    pub timer: f32,
    /// Warnings given (+0x84, `GetTrespassWarningLevel`) and allowed (+0x94).
    pub given: u32,
    pub allowed: u32,
}

fn detects_player(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    Facts {
        order,
        state,
        speaker: None,
    }
    .detection(who, PLAYER_REF)
    .is_some_and(|v| v > crate::detection::SEEN)
}

/// The living, enabled people in the player's cell, in its reference
/// order (the "loaded" ones).
fn people_here(order: &LoadOrder, state: &GameState) -> Vec<FormId> {
    let Some(cell) = state.player_cell else {
        return Vec::new();
    };
    order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| rr.entry.header.kind.as_bytes() == b"ACHR")
        .map(|rr| rr.form_id)
        .filter(|w| !state.dead.contains(w) && state.disabled.get(w) != Some(&true))
        .collect()
}

/// The factions a person's base record lists (`SNAM`, whatever rank; from
/// the template when it gives the factions).
fn listed_factions(order: &LoadOrder, person: FormId) -> Vec<FormId> {
    let base = if order
        .get(person)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"NPC_")
    {
        Some(person)
    } else {
        base_of(order, person)
    };
    let Some((rr, record)) =
        base.and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_FACTIONS))
    else {
        return Vec::new();
    };
    record
        .get_all(esm::FourCC::new(b"SNAM"))
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(crate::cell::le_u32(&s.data, 0))))
        .collect()
}

/// Whether a faction is flagged evil (`0047d7c0`: `DATA` 0x02).
fn faction_evil(order: &LoadOrder, faction: FormId) -> bool {
    order
        .get(faction)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.first().copied()))
        .is_some_and(|f| f & 0x02 != 0)
}

/// Whether an owner (a faction, or a person or their base) is evil
/// (`0047d7c0`; `0047d740`: a person with factions, all flagged evil).
pub fn evil(order: &LoadOrder, who: FormId) -> bool {
    if order
        .get(who)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"FACT")
    {
        return faction_evil(order, who);
    }
    let listed = listed_factions(order, who);
    !listed.is_empty() && listed.iter().all(|&f| faction_evil(order, f))
}

/// Whether `who` is in `faction` (`008b8e90`): their base lists it (any
/// rank) and no script took them out, or a script put them in.
pub(crate) fn in_faction(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    faction: FormId,
) -> bool {
    match state.faction_changes.get(&(who, faction)) {
        Some(&rank) if rank < 0 => false,
        Some(_) => true,
        None => listed_factions(order, who).contains(&faction),
    }
}

/// Whether `who` cares about an alarm raised by `alarmer` about `owner`
/// (`008c1010`; see the module notes).
fn cares(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    alarmer: FormId,
    owner: Option<FormId>,
) -> bool {
    let about_owner =
        owner.is_some_and(
            |o| match order.get(o).map(|r| *r.entry.header.kind.as_bytes()) {
                Some(k) if &k == b"FACT" => in_faction(order, state, who, o),
                Some(k) if &k == b"NPC_" => listed_factions(order, o)
                    .into_iter()
                    .any(|f| in_faction(order, state, who, f)),
                _ => false,
            },
        );
    if about_owner {
        return true;
    }
    crate::factions::factions_of(order, state, alarmer)
        .into_iter()
        .any(|f| in_faction(order, state, who, f))
}

fn distance(order: &LoadOrder, state: &GameState, a: FormId, b: FormId) -> Option<f32> {
    let (sa, _, pa, _) = state.place(order, a)?;
    let (sb, _, pb, _) = state.place(order, b)?;
    (sa == sb).then(|| (0..3).map(|i| (pa[i] - pb[i]).powi(2)).sum::<f32>().sqrt())
}

/// The trespass package as the AI follows it: near the player, within
/// [`WARNING_DISTANCE`] (`world::ai::current_package` gives it to the
/// warner).
pub fn package(state: &GameState, who: FormId) -> Option<crate::ai::Package> {
    state.living.trespass.as_ref().filter(|w| w.warner == who)?;
    Some(crate::ai::Package {
        actions: Default::default(),
        data: Default::default(),
        form_id: FormId(0),
        editor_id: Some("TrespassPackage".into()),
        kind: PACKAGE_TYPE,
        flags: 0,
        location: Some(crate::ai::Location {
            kind: 0,
            form: PLAYER_REF,
            radius: WARNING_DISTANCE as i32,
        }),
        schedule: crate::ai::Schedule {
            month: -1,
            day_of_week: -1,
            date: 0,
            hour: -1,
            duration: 0,
        },
        conditions: Vec::new(),
        target: None,
        topic: None,
    })
}

/// Each frame (`seconds` of it): someone noticing the trespassing player
/// comes over, warns and at last raises the alarm (see the module notes).
pub fn update(order: &LoadOrder, state: &mut GameState, seconds: f32) {
    let here = state.player_cell;
    let trespassing = here.is_some_and(|c| crate::crime::trespassing(order, state, c));
    if let Some(w) = &state.living.trespass {
        if !trespassing || here != Some(w.cell) {
            state.living.trespass = None;
        }
    }
    if !trespassing || state.living.trespass_alarm != here {
        state.living.trespass_alarm = None;
    }
    let Some(cell) = here.filter(|_| trespassing) else {
        return;
    };
    if state.living.trespass_alarm == Some(cell) {
        return;
    }
    if state.living.trespass.is_none() {
        let owner = crate::script_functions::cell_owner_now(order, state, cell);
        let owner_evil = owner.is_some_and(|o| evil(order, o));
        let warner = people_here(order, state)
            .into_iter()
            .find(|&p| (!owner_evil || evil(order, p)) && detects_player(order, state, p));
        let Some(warner) = warner else {
            return;
        };
        let off_limits = order
            .get(cell)
            .is_some_and(|r| r.entry.header.flags & OFF_LIMITS != 0);
        state.living.trespass = Some(Warning {
            warner,
            cell,
            timer: 0.0,
            given: 0,
            allowed: if off_limits { 0 } else { WARNINGS },
        });
    }
    let Some(mut w) = state.living.trespass.clone() else {
        return;
    };
    // Walking over first.
    if distance(order, state, w.warner, PLAYER_REF).map_or(true, |d| d > WARNING_DISTANCE) {
        return;
    }
    if w.allowed == 0 || (w.allowed < w.given && w.timer <= 0.0) {
        state.living.trespass = None;
        state.living.trespass_alarm = Some(cell);
        alarm(order, state, w.warner, None);
        return;
    }
    let limit = super::setting(order, "fAITrespassWarningTimer", 10.0);
    if w.timer <= 0.0 {
        state.events.push(Event::Talk {
            speaker: w.warner,
            to: PLAYER_REF,
            topic: Some(GUARD_TRESPASS),
            conversation: false,
        });
        w.timer += limit;
        w.given += 1;
    } else {
        w.timer -= seconds;
        // More than a whole wait left (the setting changed): none
        // (`00918120` takes the timer off itself).
        if w.timer > limit {
            w.timer = 0.0;
        }
    }
    state.living.trespass = Some(w);
}

/// The trespass alarm (`008c0ec0`; see the module notes): who turned on
/// the player.
pub fn alarm(
    order: &LoadOrder,
    state: &mut GameState,
    alarmer: FormId,
    owner: Option<FormId>,
) -> Vec<FormId> {
    let turned: Vec<FormId> = people_here(order, state)
        .into_iter()
        .filter(|&p| !crate::script_functions::ignores_crime(state, p))
        .filter(|&p| detects_player(order, state, p))
        .filter(|&p| cares(order, state, p, alarmer, owner))
        .collect();
    for &p in &turned {
        state.combat.insert(p, PLAYER_REF);
    }
    if let Some(&first) = turned.first() {
        crate::crime::witnessed(order, state, first, 1, 0, "fReputationMinorCrimeNeg", 2.0);
        // The player's own minor crimes (`GetMinorCrimeCount`; the Xbox
        // `Actor::TrespassAlarm` counts it once at the end).
        state.player_crimes.0 += 1;
    }
    turned
}
