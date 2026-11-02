//! Ownership and crime, read from the game's code (`Actor::StealAlarm`,
//! `AttackAlarm`, `MurderAlarm`, `PickpocketAlarm`, `TrespassAlarm`,
//! Xbox PDB; `docs/FACTIONS_CRIME.md`).
//!
//! Who owns a reference (`00567790`): its own `XOWN`; for a door, the
//! door on the other side's; for anything but people, furniture, doors and
//! activators, its encounter zone's owner (`ECZN` `DATA`, the reference's
//! `XEZN` else its cell's); else its cell's `XOWN` (else the cell's zone's).
//! People have only their own. The player may take what nobody owns, what
//! the player owns, and what a faction they're in owns (`005785e0`; the
//! rank `XRNK` isn't checked there).
//!
//! Stealing (`008bfa40`): −5 karma (`fKarmaModStealing`) whenever the
//! owner isn't evil (a faction flagged evil, `DATA` 0x02, or a person
//! whose record karma is evil), seen or not, unless taken from a person.
//! If the victim (the person stolen from, else the owner or a member of
//! the owning faction nearby) sees the player (detection above 0): each
//! of the victim's factions that tracks crime (`DATA` 0x100) counts a
//! minor crime and gives the player `fReputationMinorCrimeNeg` (2) infamy
//! with its reputation (`WMI1`); a second witnessed theft on the same day
//! of the month makes the victim attack; the first `iStealWarnings` (2)
//! times the victim only comes to warn (not here: the warning package).
//! Thefts and pickpockets don't count in the player's own minor crimes;
//! the trespass alarm does, and witnessed assaults and murders in their
//! major ones.
//!
//! A faction can hold the player as an enemy for their crimes (its runtime
//! flag 0x10; `SetPCEnemyofFaction`, `ClearFactionPlayerEnemyFlag`): its
//! members then react to the player as enemies (`world::factions`). An
//! assault anyone sees, and every murder, flags the victim's
//! crime-tracking factions so ([`assault_crime`], [`murder`]).
//! Trespassing (`00546da0`): in a cell with an owner, not public (`DATA`
//! 0x20 or 0x40), with no `XGLB`, that isn't the player's or a faction
//! they're in (at the cell's `XRNK` rank or above). Scripts change owners
//! and the public flag (`SetOwnership`, `SetCellOwnership`,
//! `SetCellPublicFlag`), and who notices (`IgnoreCrime`,
//! `SetIgnoreFriendlyHits`): `world::script_functions`.

use esm::{FormId, FourCC, LoadOrder, RecordRef};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::PLAYER_REF;
use crate::scripting::{base_of, game_setting, GameState};

const XOWN: FourCC = FourCC::new(b"XOWN");
const XRNK: FourCC = FourCC::new(b"XRNK");
const XGLB: FourCC = FourCC::new(b"XGLB");
const XEZN: FourCC = FourCC::new(b"XEZN");
const XTEL: FourCC = FourCC::new(b"XTEL");
const FACT: FourCC = FourCC::new(b"FACT");

fn form_in(rr: &RecordRef<'_>, record: &esm::Record, kind: FourCC) -> Option<FormId> {
    record
        .get(kind)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .filter(|f| f.0 != 0)
}

/// An encounter zone's owner (`ECZN` `DATA` form at 0).
fn zone_owner(order: &LoadOrder, zone: FormId) -> Option<FormId> {
    let rr = order.get(zone)?;
    let record = rr.record().ok()?;
    form_in(&rr, &record, esm::sig::DATA)
}

/// A cell's owner: its `XOWN`, else its encounter zone's owner.
pub fn cell_owner(order: &LoadOrder, cell: FormId) -> Option<FormId> {
    let rr = order.get(cell)?;
    let record = rr.record().ok()?;
    form_in(&rr, &record, XOWN)
        .or_else(|| form_in(&rr, &record, XEZN).and_then(|z| zone_owner(order, z)))
}

/// Who owns a reference (see the module notes). Its own owner is as
/// `SetOwnership` set it, else its `XOWN`; a cell's as `SetCellOwnership`
/// set it, else its record's (`world::script_functions`).
pub fn owner_of(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<FormId> {
    if let Some(o) = crate::script_functions::own_owner(order, state, reference) {
        return Some(o);
    }
    let rr = order.get(reference)?;
    let record = rr.record().ok()?;
    let kind = rr.entry.header.kind;
    if matches!(kind.as_bytes(), b"ACHR" | b"ACRE") {
        return None;
    }
    let base_kind = base_of(order, reference)
        .and_then(|b| order.get(b))
        .map(|b| b.entry.header.kind);
    let base = base_kind.as_ref().map(|k| *k.as_bytes());
    if base == Some(*b"DOOR") {
        let partner = form_in(&rr, &record, XTEL).and_then(|d| {
            let prr = order.get(d)?;
            let prec = prr.record().ok()?;
            form_in(&prr, &prec, XOWN)
        });
        if partner.is_some() {
            return partner;
        }
    }
    if !matches!(base, Some(b) if b == *b"FURN" || b == *b"DOOR" || b == *b"ACTI") {
        if let Some(o) = form_in(&rr, &record, XEZN).and_then(|z| zone_owner(order, z)) {
            return Some(o);
        }
    }
    let (_, cell, _, _) = state.place(order, reference)?;
    crate::script_functions::cell_owner_now(order, state, cell)
}

fn is_faction(order: &LoadOrder, id: FormId) -> bool {
    order.get(id).is_some_and(|r| r.entry.header.kind == FACT)
}

fn faction_flags(order: &LoadOrder, faction: FormId) -> u32 {
    order
        .get(faction)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
        .map_or(0, |d| {
            let mut b = [0u8; 4];
            for (i, v) in d.iter().take(4).enumerate() {
                b[i] = *v;
            }
            u32::from_le_bytes(b)
        })
}

/// Whether a faction tracks crime (`FACT` `DATA` flag 0x100).
pub fn tracks_crime(order: &LoadOrder, faction: FormId) -> bool {
    faction_flags(order, faction) & 0x100 != 0
}

/// Whether the player may take or use what `owner` owns.
pub fn may_take(order: &LoadOrder, state: &GameState, owner: Option<FormId>) -> bool {
    let Some(owner) = owner else {
        return true;
    };
    if Some(owner) == base_of(order, PLAYER_REF) || owner == crate::dialogue::PLAYER_BASE {
        return true;
    }
    is_faction(order, owner)
        && crate::factions::factions_of(order, state, PLAYER_REF).contains(&owner)
}

/// Whether an owner is evil (`00578790`): a faction flagged evil, or a
/// person whose record karma is evil or very evil.
pub fn owner_is_evil(order: &LoadOrder, owner: FormId) -> bool {
    if is_faction(order, owner) {
        return faction_flags(order, owner) & 0x02 != 0;
    }
    let karma = order
        .get(owner)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(FourCC::new(b"ACBS")).map(|s| s.data.clone()))
        .filter(|d| d.len() >= 20)
        .map_or(0.0, |d| le_f32(&d, 16));
    matches!(crate::reputation::alignment(order, karma), 2 | 4)
}

/// The karma for taking what `owner` owns: `fKarmaModStealing` (−5)
/// unless the owner is evil ([`owner_is_evil`]). The same test in the
/// theft (`008bfa40`), opening someone's terminal (`00501310`), picking up
/// their note (`005e9360`) and a note taken from their container
/// (`004c37d0`).
pub fn stealing_karma(order: &LoadOrder, state: &mut GameState, owner: FormId) {
    if !owner_is_evil(order, owner) {
        let karma = game_setting(order, "fKarmaModStealing")
            .unwrap_or(-5.0)
            .trunc() as i32;
        crate::reputation::reward_karma(order, state, karma);
    }
}

/// The person who would see a theft from `from` owned by `owner`: the
/// person stolen from, else someone in the player's cell who is the owner
/// or in the owning faction (the game asks for a loaded one).
fn victim(order: &LoadOrder, state: &GameState, from: FormId, owner: FormId) -> Option<FormId> {
    if order
        .get(from)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"ACHR")
    {
        return Some(from);
    }
    let cell = state.player_cell?;
    order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| rr.entry.header.kind.as_bytes() == b"ACHR")
        .map(|rr| rr.form_id)
        .find(|&who| {
            !state.dead.contains(&who)
                && (base_of(order, who) == Some(owner)
                    || crate::factions::factions_of(order, state, who).contains(&owner))
        })
}

/// The player takes something `owner` owns from `from` (see the module
/// notes). Whether anyone saw it.
// Translated from 008bfa40 (decompiled, FalloutNV.exe 1.4.0.525;
// `Actor::StealAlarm`, Xbox PDB)
pub fn steal(order: &LoadOrder, state: &mut GameState, from: FormId, owner: FormId) -> bool {
    let warned_before =
        state.steal_warnings < game_setting(order, "iStealWarnings").unwrap_or(2.0) as u32;
    // Taking from a person (a character) costs no karma there.
    let from_person = order
        .get(from)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"ACHR");
    if !from_person {
        stealing_karma(order, state, owner);
    }
    let Some(victim) = victim(order, state, from, owner) else {
        return false;
    };
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    // A victim who ignores crime (`IgnoreCrime`, vtable +0x320 in
    // `008bfa40`) doesn't count it.
    if crate::script_functions::ignores_crime(state, victim)
        || state.dead.contains(&victim)
        || facts
            .detection(victim, PLAYER_REF)
            .map_or(true, |v| v <= crate::detection::SEEN)
    {
        return false;
    }
    witnessed(order, state, victim, 1, 0, "fReputationMinorCrimeNeg", 2.0);
    // The record of the day kept on the player (its `+0x134`/`+0x138`):
    // the first seen crime notes the day of the month; one on that day
    // makes the victim attack (and nothing more); one on another day
    // clears the note.
    let day = state.global(order, "GameDay").unwrap_or(0.0) as u32;
    match state.last_theft_day {
        None => state.last_theft_day = Some(day),
        Some(d) if d == day => {
            state.combat.insert(victim, PLAYER_REF);
            return true;
        }
        Some(_) => state.last_theft_day = None,
    }
    // Warnings left (`iStealWarnings`, counted when the theft began): the
    // victim comes to warn (`Actor::InitiateStealWarning`, a package of
    // type 0x22 the AI follows, which counts one more warning,
    // `PlayerCharacter::ModStealWarning`, and starts `fWarningTimer`);
    // none left: the victim raises the alarm (the AI's).
    if warned_before {
        state.steal_warnings += 1;
    }
    true
}

/// A crime against `victim` counted by its factions
/// (`Actor::AddFactionMinorCrime` / `AddFactionMajorCrime`, Xbox PDB;
/// `008b7c00`, `008b7d20`): each of its factions that tracks crime counts
/// `minor` and `major` crimes, and with an infamy setting given
/// (`fReputationMinorCrimeNeg` 2, `fReputationMajorCrimeNeg` 30;
/// `TESFaction::AddMinorCrime`, `005fda00` / `005fda50` → `00616c20`)
/// gives the player that much infamy with its reputation (`WMI1`,
/// `AddReputationExact`'s notice and title box). The player's own counts
/// (`GetMinorCrimeCount` on the player) are the callers' business.
pub fn witnessed(
    order: &LoadOrder,
    state: &mut GameState,
    victim: FormId,
    minor: u32,
    major: u32,
    infamy_setting: &str,
    infamy_default: f32,
) {
    let infamy = if infamy_setting.is_empty() {
        0.0
    } else {
        game_setting(order, infamy_setting).unwrap_or(infamy_default)
    };
    for faction in crate::factions::factions_of(order, state, victim) {
        if !tracks_crime(order, faction) {
            continue;
        }
        let counts = state.faction_crimes.entry(faction).or_insert((0, 0));
        counts.0 += minor;
        counts.1 += major;
        let rep = order
            .get(faction)
            .and_then(|r| r.record().ok().map(|rec| (r, rec)))
            .and_then(|(rr, rec)| form_in(&rr, &rec, FourCC::new(b"WMI1")));
        if let Some(rep) = rep.filter(|_| infamy > 0.0) {
            crate::reputation::add(order, state, rep, crate::reputation::INFAMY, infamy);
        }
    }
}

/// Whether someone is in a faction flagged for special combat (`DATA`
/// 0x04; `Actor::IsInCombatantFaction`, Xbox PDB): a crime between two
/// such isn't one (`008c0460`, `008c09e0`).
fn in_combatant_faction(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    crate::factions::factions_of(order, state, who)
        .into_iter()
        .any(|f| faction_flags(order, f) & 0x04 != 0)
}

/// `who`'s factions that track crime and that `victim` is in hold the
/// player as an enemy (their runtime flag 0x10;
/// `Actor::SetFactionsThatCareAboutCrime`, Xbox PDB, `008b8360` with
/// `008b8e90`). Whether any did.
// Translated from 008b8360 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn set_factions_that_care(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    victim: FormId,
) -> bool {
    let mut any = false;
    for f in crate::factions::factions_of(order, state, who) {
        if tracks_crime(order, f) && crate::living::trespass::in_faction(order, state, victim, f) {
            state.crime_enemies.insert(f);
            any = true;
        }
    }
    any
}

/// Who a crime is sent to (`ProcessLists::SendCrimetoHighList`, Xbox PDB):
/// the living, enabled people in the player's cell (the game asks every
/// actor in high process) but the criminal, who detect the criminal (above
/// 0); the victim among them when alive.
fn crime_witnesses(order: &LoadOrder, state: &GameState, criminal: FormId) -> Vec<FormId> {
    let Some(cell) = state.player_cell else {
        return Vec::new();
    };
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| rr.entry.header.kind.as_bytes() == b"ACHR")
        .map(|rr| rr.form_id)
        .filter(|&w| w != criminal && !state.dead.contains(&w))
        .filter(|&w| state.disabled.get(&w) != Some(&true))
        .filter(|&w| {
            facts
                .detection(w, criminal)
                .is_some_and(|v| v > crate::detection::SEEN)
        })
        .collect()
}

/// Who sees a crime: the living people in the player's cell (the game
/// asks every loaded actor) who detect the criminal (above 0), the victim
/// left out, and those who ignore crime (`IgnoreCrime`: skipped in the
/// assault and murder witness loops, `008c0460`, `008c09e0`).
pub fn witnesses(
    order: &LoadOrder,
    state: &GameState,
    criminal: FormId,
    victim: FormId,
) -> Vec<FormId> {
    let Some(cell) = state.player_cell else {
        return Vec::new();
    };
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    order
        .references_in_cell(cell)
        .into_iter()
        .filter(|rr| rr.entry.header.kind.as_bytes() == b"ACHR")
        .map(|rr| rr.form_id)
        .filter(|&w| w != victim && w != criminal && !state.dead.contains(&w))
        .filter(|&w| !crate::script_functions::ignores_crime(state, w))
        .filter(|&w| {
            facts
                .detection(w, criminal)
                .is_some_and(|v| v > crate::detection::SEEN)
        })
        .collect()
}

/// The player hits someone who isn't fighting them (`008987f0`): a friend
/// or ally takes a few hits first (`iFriendHitCombatAllowed` 4 in
/// FalloutNV.esm, GMST `00040F09`, exe default 3 / `iFriendHitNonCombatAllowed`
/// 0 for friends, `iAllyHitCombatAllowed` 1000 / `iAllyHitNonCombatAllowed`
/// 3 for allies, the last three only the exe's defaults, `00f5a1d0` to
/// `00f5a260`; "combat" meaning the victim is fighting someone) and only
/// remarks on it; past that, or anyone
/// else, it's an assault: a major crime for the player, and if anyone saw
/// it, +1 major crime (no infamy) for each of the victim's factions that
/// tracks crime. A friend or ally who ignores friendly hits
/// (`SetIgnoreFriendlyHits`, form flag 0x100000: `008987f0` returns at
/// once) takes no notice at all; a victim who ignores crime (`IgnoreCrime`:
/// `008c0460` returns at once) counts no crime but still fights back.
/// Whether the victim fights back.
pub fn assault(order: &LoadOrder, state: &mut GameState, victim: FormId) -> bool {
    use crate::factions::Reaction;
    let reaction = crate::factions::reaction(order, state, victim, PLAYER_REF);
    let fighting = state.combat.contains_key(&victim);
    // The data's value, else the exe's default (`00f5a1d0`, `00f5a200`,
    // `00f5a230`, `00f5a260`).
    let allowed = |name: &str, default: f32| game_setting(order, name).unwrap_or(default) as u32;
    let allowance = match (reaction, fighting) {
        (Reaction::Friend, true) => Some(allowed("iFriendHitCombatAllowed", 3.0)),
        (Reaction::Friend, false) => Some(allowed("iFriendHitNonCombatAllowed", 0.0)),
        (Reaction::Ally, true) => Some(allowed("iAllyHitCombatAllowed", 1000.0)),
        (Reaction::Ally, false) => Some(allowed("iAllyHitNonCombatAllowed", 3.0)),
        _ => None,
    };
    if allowance.is_some() && crate::script_functions::ignores_friendly_hits(order, state, victim) {
        return false;
    }
    // Translated from 008987f0 (decompiled, FalloutNV.exe 1.4.0.525):
    // an allowance of 0 counts nothing; above 999 the record starts afresh
    // (`ExtraDataList::RemoveFriendHitsExtra`, Xbox PDB) before the hit.
    if let Some(limit) = allowance.filter(|&l| l > 0) {
        if limit > 999 {
            state.friendly_hits.remove(&victim);
        }
        add_friend_hit(order, state, victim);
        if friend_hit_count(order, state, victim) <= limit {
            return false;
        }
    }
    assault_crime(order, state, victim);
    true
}

/// The player's recent hits on a friend or ally (`ExtraFriendHits`, Xbox
/// PDB, extra data 0x45): the times of hits within `fFriendHitTimer` (10 s)
/// of now (`ExtraFriendHits::GetHitCount`, `00435e20` → `RemoveOldHits`
/// `00435e40`), as `GetFriendHit` reports them. The game's clock
/// (`011f1bf0`) is taken as the state's seconds.
pub fn friend_hit_count(order: &LoadOrder, state: &GameState, victim: FormId) -> u32 {
    let timer = f64::from(game_setting(order, "fFriendHitTimer").unwrap_or(10.0));
    state.friendly_hits.get(&victim).map_or(0, |hits| {
        hits.iter().filter(|&&t| state.seconds - t <= timer).count() as u32
    })
}

/// One more hit on a friend or ally (`ExtraFriendHits::AddHit`, Xbox PDB):
/// hits older than `fFriendHitTimer` are dropped, and one within
/// `fFriendMinimumLastHitTime` (0.5 s) of the last isn't counted.
// Translated from 00435d40 (decompiled, FalloutNV.exe 1.4.0.525)
fn add_friend_hit(order: &LoadOrder, state: &mut GameState, victim: FormId) {
    let timer = f64::from(game_setting(order, "fFriendHitTimer").unwrap_or(10.0));
    let gap = f64::from(game_setting(order, "fFriendMinimumLastHitTime").unwrap_or(0.5));
    let now = state.seconds;
    let hits = state.friendly_hits.entry(victim).or_default();
    hits.retain(|&t| now - t <= timer);
    if hits.last().is_some_and(|&last| now - last < gap) {
        return;
    }
    hits.push(now);
}

/// The assault crime itself (`Actor::AttackAlarm`, Xbox PDB, `008c0460`,
/// also `SendAssaultAlarm`'s): none against someone who ignores crime, nor
/// when both are in special-combat factions. The crime goes to everyone
/// who detects the player ([`crime_witnesses`], the victim too); each of
/// them who doesn't ignore crime makes their crime-tracking factions the
/// victim is in hold the player as an enemy ([`set_factions_that_care`]);
/// what they do next is the AI's (an alarm package, or attacking). With
/// anyone there: +1 major crime for the player and for each of the
/// victim's crime-tracking factions (no infamy). Nobody detecting the
/// player: nothing.
// Translated from 008c0460 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn assault_crime(order: &LoadOrder, state: &mut GameState, victim: FormId) {
    if crate::script_functions::ignores_crime(state, victim) {
        return;
    }
    if in_combatant_faction(order, state, victim) && in_combatant_faction(order, state, PLAYER_REF)
    {
        return;
    }
    let seen = crime_witnesses(order, state, PLAYER_REF);
    if seen.is_empty() {
        return;
    }
    for w in seen {
        if !crate::script_functions::ignores_crime(state, w) {
            set_factions_that_care(order, state, w, victim);
        }
    }
    // The crime's know list holds the victim, so it always counts here.
    state.player_crimes.1 += 1;
    witnessed(order, state, victim, 0, 1, "", 0.0);
}

/// The player (or a teammate) kills a person who wasn't fighting them: a
/// murder (`Actor::MurderAlarm`, Xbox PDB, `008c09e0`; none when both are
/// in special-combat factions). The victim's own crime-tracking factions
/// hold the player as an enemy whether or not anyone saw it
/// ([`set_factions_that_care`] on the victim itself); the crime goes to
/// everyone who detects the killer, and each of them who doesn't ignore
/// crime does the same for their factions the victim was in. With anyone
/// there: +1 major crime for the player and for each of the victim's
/// crime-tracking factions, with `fReputationMajorCrimeNeg` (30) infamy.
/// The player killing becomes a murderer (`IsPCAMurderer`;
/// `PlayerCharacter::SetIsAMurderer`, Xbox PDB, from `Actor::Kill`
/// `0089d900`) unless the victim is part of an evil faction (`005678a0`:
/// every faction they're in flagged evil). (Which deaths count as murder
/// hangs on two things not traced, the victim's process flag 4 and an
/// allowance for friends; here: a person who wasn't fighting the killer.
/// A creature, or someone with an owner, gets the assault alarm instead in
/// the game, `008b01c0`; nothing here.)
// Translated from 008c09e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn murder(order: &LoadOrder, state: &mut GameState, victim: FormId, killer: FormId) {
    if crime_victim_is_creature(order, victim) {
        return;
    }
    if in_combatant_faction(order, state, victim) && in_combatant_faction(order, state, killer) {
        return;
    }
    set_factions_that_care(order, state, victim, victim);
    let seen = crime_witnesses(order, state, killer);
    if !seen.is_empty() {
        for w in seen {
            if !crate::script_functions::ignores_crime(state, w) {
                set_factions_that_care(order, state, w, victim);
            }
        }
        state.player_crimes.1 += 1;
        witnessed(order, state, victim, 0, 1, "fReputationMajorCrimeNeg", 30.0);
    }
    if killer == PLAYER_REF && !crate::living::trespass::evil(order, victim) {
        state.player_murderer = true;
    }
}

fn crime_victim_is_creature(order: &LoadOrder, who: FormId) -> bool {
    crate::combat::is_creature(order, who)
}

/// Whether the player is trespassing in a cell (`00546da0`).
pub fn trespassing(order: &LoadOrder, state: &GameState, cell: FormId) -> bool {
    let Some(rr) = order.get(cell) else {
        return false;
    };
    let Ok(record) = rr.record() else {
        return false;
    };
    let Some(owner) = crate::script_functions::cell_owner_now(order, state, cell) else {
        return false;
    };
    if crate::script_functions::cell_is_public(order, state, cell) || record.get(XGLB).is_some() {
        return false;
    }
    if Some(owner) == base_of(order, PLAYER_REF) || owner == crate::dialogue::PLAYER_BASE {
        return false;
    }
    if !is_faction(order, owner) {
        return true;
    }
    let need = record
        .get(XRNK)
        .filter(|s| s.data.len() >= 4)
        .map_or(0, |s| le_u32(&s.data, 0) as i32);
    let rank = state
        .faction_changes
        .get(&(PLAYER_REF, owner))
        .map(|r| i32::from(*r))
        .or_else(|| {
            crate::factions::factions_of(order, state, PLAYER_REF)
                .contains(&owner)
                .then_some(0)
        })
        .unwrap_or(-1);
    rank < need
}
