//! Who's hostile to whom: factions (`FACT`) and aggression.
//!
//! A faction's `XNAM`s are its relations: the other faction, a modifier
//! (i32), and the combat reaction (u32: 0 neutral, 1 enemy, 2 ally, 3
//! friend). Checked: `GeneralHostileToPlayerFaction` ("Hostile to the
//! player only.") names `PlayerFaction` (`0001B2A4`) with 1; the NCR's
//! relation to the player is 0. People list their factions in `SNAM`s
//! (faction, rank; from their template when its flags say so); the
//! player is in `PlayerFaction`.
//!
//! Scripts change both, and the game's hostilities come from them: the
//! Powder Gangers' records make them neutral to the player, and
//! `VFreeformNCRCFScript` or `VES04FactReactPGDeathScript` run `SetEnemy
//! PowderGangerFactionNV PlayerFaction`. `SetEnemy F1 F2 [a b]` makes the
//! two enemies (each flag 1 instead leaves that side neutral), `SetAlly F1
//! F2 [a b]` allies (a flag 1 makes that side a friend), as the editor
//! documents them; the changes are kept in the game state.
//!
//! Whether someone attacks the player on sight, by their aggression
//! (`AIDT` byte 0) and the reaction of their factions, read from the
//! game's code ([`attacks_on_sight`], [`reaction`]). Reputations (`REPU`)
//! and crimes aren't counted yet.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::scripting::{base_of, GameState};

const FACT: FourCC = FourCC::new(b"FACT");
const XNAM: FourCC = FourCC::new(b"XNAM");
const SNAM: FourCC = FourCC::new(b"SNAM");
const AIDT: FourCC = FourCC::new(b"AIDT");

/// The player's faction, a fixed form in every game.
pub const PLAYER_FACTION: FormId = FormId(0x1B2A4);

/// How one faction reacts to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reaction {
    Neutral,
    Enemy,
    Ally,
    Friend,
}

impl Reaction {
    /// As stored (`XNAM`, `GetFactionRelation`): 0 neutral, 1 enemy, 2 ally,
    /// 3 friend.
    pub fn from_code(code: u32) -> Reaction {
        match code {
            1 => Reaction::Enemy,
            2 => Reaction::Ally,
            3 => Reaction::Friend,
            _ => Reaction::Neutral,
        }
    }

    pub fn code(self) -> u8 {
        match self {
            Reaction::Neutral => 0,
            Reaction::Enemy => 1,
            Reaction::Ally => 2,
            Reaction::Friend => 3,
        }
    }
}

/// A faction's relations (`XNAM`), as the records have them.
pub fn relations(order: &LoadOrder, faction: FormId) -> Vec<(FormId, Reaction)> {
    let Some(rr) = order.get(faction).filter(|r| r.entry.header.kind == FACT) else {
        return Vec::new();
    };
    let Ok(record) = rr.record_shared() else {
        return Vec::new();
    };
    record
        .get_all(XNAM)
        .filter(|s| s.data.len() >= 12)
        .map(|s| {
            let other = rr.plugin.to_global(FormId(le_u32(&s.data, 0)));
            (other, Reaction::from_code(le_u32(&s.data, 8)))
        })
        .collect()
}

/// How one faction reacts to another now: as a script set it, else the
/// record's relation, else neutral.
pub fn faction_reaction(
    order: &LoadOrder,
    state: &GameState,
    faction: FormId,
    other: FormId,
) -> Reaction {
    if let Some(&code) = state.faction_relations.get(&(faction, other)) {
        return Reaction::from_code(u32::from(code));
    }
    relations(order, faction)
        .into_iter()
        .find(|(f, _)| *f == other)
        .map_or(Reaction::Neutral, |(_, r)| r)
}

/// A placed person's base record, or the record itself when it's a person
/// or creature (`NPC_`, `CREA`) rather than a placement.
fn person(order: &LoadOrder, who: FormId) -> Option<FormId> {
    let kind = order.get(who)?.entry.header.kind;
    if kind.as_bytes() == b"NPC_" || kind.as_bytes() == b"CREA" {
        return Some(who);
    }
    base_of(order, who)
}

/// Someone's factions now: the player's `PlayerFaction`; others their
/// base record's `SNAM`s (or their template's, when it gives the
/// factions) whose rank (i8 at 4) isn't negative, as the game counts
/// membership (`008b8290`); with what scripts added and took away.
pub fn factions_of(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<FormId> {
    let mut out = Vec::new();
    if who == crate::dialogue::PLAYER_REF {
        out.push(PLAYER_FACTION);
    }
    if let Some((rr, record)) = person(order, who)
        .and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_FACTIONS))
    {
        out.extend(
            record
                .get_all(SNAM)
                .filter(|s| s.data.len() >= 4)
                .filter(|s| s.data.get(4).map_or(true, |&r| (r as i8) >= 0))
                .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
                .filter(|f| order.get(*f).is_some_and(|r| r.entry.header.kind == FACT)),
        );
    }
    let mut changes: Vec<(FormId, i8)> = state
        .faction_changes
        .iter()
        .filter(|((w, _), _)| *w == who)
        .map(|((_, f), r)| (*f, *r))
        .collect();
    changes.sort();
    for (faction, rank) in changes {
        out.retain(|f| *f != faction);
        if rank >= 0 {
            out.push(faction);
        }
    }
    out
}

/// How `who` reacts to `other`, from their factions' relations now
/// (`008b87a0`, combining every pair of factions as `008b8740` does): an
/// ally relation wins at once; a friend one beats enemy and neutral; an
/// enemy one beats neutral. (Read from the game's code, `findings\
/// combat_ai.md` §2.1: friend and ally beat enemy.)
pub fn reaction(order: &LoadOrder, state: &GameState, who: FormId, other: FormId) -> Reaction {
    let mine = factions_of(order, state, who);
    let theirs = factions_of(order, state, other);
    let mut found = Reaction::Neutral;
    for faction in mine {
        // A faction holding the player as an enemy for their crimes
        // (`005a2270`, `world::crime`) makes the reaction so far "enemy"
        // in place of its relations; the factions after it still count, so
        // a friend or an ally there wins (`008b87a0`).
        if other == crate::dialogue::PLAYER_REF && state.crime_enemies.contains(&faction) {
            found = Reaction::Enemy;
            continue;
        }
        for &other_faction in &theirs {
            match faction_reaction(order, state, faction, other_faction) {
                Reaction::Ally => return Reaction::Ally,
                Reaction::Friend => found = Reaction::Friend,
                Reaction::Enemy if found == Reaction::Neutral => found = Reaction::Enemy,
                _ => {}
            }
        }
    }
    found
}

/// Someone's aggression: actor value 0 as scripts left it (`008bea10`),
/// else `AIDT` byte 0 (their template's when it gives the AI data).
pub fn aggression(order: &LoadOrder, who: FormId) -> u8 {
    person(order, who)
        .and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_AI_DATA))
        .and_then(|(_, r)| r.get(AIDT).and_then(|s| s.data.first().copied()))
        .unwrap_or(0)
}

/// Whether `who` would attack `other` (`008b06d0`, `GetShouldAttack`):
/// frenzied (aggression 3) always; aggressive (1) enemies; very aggressive
/// (2) enemies and neutrals; unaggressive (0) never. Fighting them
/// already, anyone but an ally or friend. (Followers' `FollowerSwitch
/// Aggressive` and the perk entry point that can overrule it aren't here.)
pub fn attacks_on_sight(order: &LoadOrder, state: &GameState, who: FormId, other: FormId) -> bool {
    let reaction = reaction(order, state, who, other);
    let aggression = state
        .actor_values
        .get(&(who, 0))
        .map_or_else(|| aggression(order, who), |&v| v.clamp(0.0, 3.0) as u8);
    if aggression == 3 {
        return true;
    }
    if state.combat.get(&who) == Some(&other) {
        return !matches!(reaction, Reaction::Ally | Reaction::Friend);
    }
    match aggression {
        0 => false,
        1 => reaction == Reaction::Enemy,
        _ => matches!(reaction, Reaction::Enemy | Reaction::Neutral),
    }
}

/// Whether `who`, hit by `other`, would fight them: `GetShouldAttackActor`
/// (Xbox PDB) with its "attacked" flag set, as `Actor::AttackedBy` and the
/// combat controller ask it (`CanAttackActor` `008b0670`, `0097f580`).
/// Someone frenzied (aggression 3) always; else only when the attacker is
/// frenzied or isn't a friend or an ally (`008b87a0` reaction not 3/2):
/// friends and allies tolerate each other's hits. A teammate attacker is
/// taken as the player (`00566950` = actor `+0x18d`, then the call again
/// with the player). A teammate hit by someone else answers by the
/// attacker's reaction to the player (the `FollowerSwitchAggressive` ≠ 0
/// branch: the follower's script variable isn't read, so its 0 branch,
/// fighting only those already fighting the player or the follower,
/// isn't done). The player as attacker (`008bffc0`, untraced, can make it
/// 1) and the perk entry point 0x0F (`005e58f0`) aren't applied.
// Translated from 008b06d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn fights_when_hit(order: &LoadOrder, state: &GameState, who: FormId, other: FormId) -> bool {
    let player = crate::dialogue::PLAYER_REF;
    let teammate = |x: FormId| state.teammates.contains(&x);
    if !teammate(who) && teammate(other) && other != player {
        return fights_when_hit(order, state, who, player);
    }
    // A teammate never fights a child (vtable +0x1a0, `IsChild`); else
    // takes their attacker's view of the player, at aggression 1.
    if teammate(who) && other != player && crate::script_functions::is_child(order, state, other) {
        return false;
    }
    let (aggression, reacting) = if teammate(who) && other != player {
        (1, reaction(order, state, other, player))
    } else {
        (
            crate::combat_ai::aggression(order, state, who),
            reaction(order, state, who, other),
        )
    };
    if aggression == 3 {
        return true;
    }
    crate::combat_ai::aggression(order, state, other) == 3
        || !matches!(reacting, Reaction::Ally | Reaction::Friend)
}
