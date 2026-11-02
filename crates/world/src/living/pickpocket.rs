//! Pickpocketing, read from the game's code (`findings\living.md` §3:
//! `005fa330`, `0075dc80`, `0075e240`, `0075e0b0`, `00643400`, the crime
//! `008c00e0`, live grenades `0075d510`).
//!
//! **Using a person** (`005fa330`): someone unconscious (`SetUnconscious`),
//! knocked down or essential and down (`world::fatigue::knocked`)
//! answers "<name> is unconscious." (`sNoTalkUnConscious`), someone
//! fleeing "<name> is fleeing." (`sNoTalkFleeing`); the dead are searched
//! (their inventory opens as a container); a living person who isn't the
//! player's teammate, used by the player **while sneaking**, has their
//! pockets picked, unless they have caught the player before: "<name> has
//! already caught you." (`sNoPickPocketAgain`; names and texts joined as
//! `%s %s`). A teammate brings up their wheel, or nothing when they can't
//! take orders (`00754d90`, [`crate::companions::wheel_allowed`]). Anyone
//! else is talked to. Facing, light and being seen play
//! no part. (Creatures' own "allow pickpocket" and a few process flags in
//! the test aren't read: only people are pickpocketed here.)
//!
//! **Each stack moved** (`0075dc80`): taking, the value V is the item's
//! value × the count (`0075e240`); stacks the player put there themselves
//! are taken back without a roll. Placing, V is 1 whatever the item. The
//! chance (`00643400`) is the sum of `fPickPocketActorSkillBase`,
//! `…ActorSkillMult` × your Sneak, `…TargetSkillBase`, `…TargetSkillMult`
//! × their Sneak, `…AmountBase` and `…AmountMult` × V, cut to a whole
//! number and clamped to `fPickPocketMinChance` … `MaxChance`: with the
//! data's settings clamp(40 + 0.6 × yours − 0.6 × theirs − 0.5 × V, 5, 85).
//! No perk takes part. The roll (`0075e0b0`): none when V ≤ 0; otherwise a
//! random 0–99 succeeds below the chance, and either way −5 karma
//! (`fKarmaModStealing`) unless the person's own karma is evil. Success
//! moves the stack; the first theft of a visit counts in "Pockets Picked"
//! (misc stat 13). Failure moves nothing and closes the screen: "You've
//! been caught pickpocketing." (`sPickpocketFail`), the person won't let
//! the player try again, and the crime (`008c00e0`) unless they ignore
//! crime: +1 minor crime and `fReputationMinorCrimeNeg` infamy for their
//! crime-tracking factions (`008b7c00`), the same record of the day as
//! thefts kept on the player: the first crime notes the day of the month;
//! a later one on that day makes them attack, one on another day clears
//! the note.
//!
//! Items here are whole and unmodded, so a stack's value is its record's
//! (the game's value of worn or modded items, `004bd400`, isn't traced).
//!
//! Not traced, and so not done: the warning that follows a crime
//! (`008bf500` gives the person a PickpocketWarning package, type 0x23,
//! the first `iPickPocketWarnings` times: how it walks and what ends it);
//! live grenades (`0075d510`: placing a thrown or placed weapon with a
//! lobber projectile, [`is_live_grenade`], on someone living and not
//! essential while sneaking arms a projectile on them with a fuse of
//! `fProjectileInventoryGrenadeTimer` 2 s instead of moving it: the
//! projectile, its explosion and the process test at +0x388 aren't here,
//! so the item is placed like any other); a person "trespassing" themselves
//! (no crime then).

use esm::{FormId, FourCC, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::{base_of, Facts, GameState};

/// What using a person does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Use {
    /// Nothing, and a notice why, with its picture (`005fa330`: the
    /// surprised Vault Boy for the unconscious, the sad one otherwise).
    Refused(String, &'static str),
    /// Searching the dead: their inventory as a container.
    Search,
    /// Picking their pockets.
    Pickpocket,
    /// Talking.
    Talk,
    /// A teammate's wheel (`CompanionWheelMenu`, [`crate::companions`]).
    Wheel,
    /// A teammate who can't take orders now: nothing happens.
    Nothing,
}

/// The player uses a person (`005fa330`; see the module notes). `fleeing`:
/// whether they're running away (the viewer knows).
pub fn use_person(order: &LoadOrder, state: &GameState, who: FormId, fleeing: bool) -> Use {
    let name = || super::display_name(order, state, who);
    // `%s %s`: the name, a space, the setting as it is (the data's texts
    // have no leading space; the exe's own do).
    let joined = |setting: &str, exe: &str| {
        let icon = crate::message_icon::for_setting(setting).unwrap_or(crate::message_icon::SAD);
        Use::Refused(
            format!("{} {}", name(), super::text(order, setting, exe)),
            icon,
        )
    };
    // Unconscious (life state 3), knocked down (vtable +0x230: any knock
    // state, `world::fatigue`) or essential and down (life state 6).
    if state.unconscious.contains(&who) || crate::fatigue::knocked(state, who) {
        return joined("sNoTalkUnConscious", " is unconscious.");
    }
    // Fleeing: the viewer's running away, or the engine's flee package
    // (`ForceFlee`, `Actor::IsFleeing` (Xbox PDB)).
    let fleeing = fleeing || crate::ai::flee::forced(state, who);
    if fleeing && !state.dead.contains(&who) {
        return joined("sNoTalkFleeing", " is fleeing for their life.");
    }
    if state.dead.contains(&who) {
        return Use::Search;
    }
    let person = order
        .get(who)
        .is_some_and(|r| r.entry.header.kind.as_bytes() == b"ACHR");
    if person && state.player_sneaking && !state.teammates.contains(&who) {
        if state.living.caught_by.contains(&who) {
            return joined("sNoPickPocketAgain", " has already caught you.");
        }
        return Use::Pickpocket;
    }
    // The player using a teammate: their wheel, or nothing (`005fa330`
    // after the pickpocketing, `00754d90`).
    if state.teammates.contains(&who) {
        return if crate::companions::wheel_allowed(order, state, who) {
            Use::Wheel
        } else {
            Use::Nothing
        };
    }
    Use::Talk
}

/// The chance of a move, in percent (`00643400`; see the module notes):
/// `value` is V.
pub fn chance(order: &LoadOrder, state: &GameState, target: FormId, value: f64) -> i32 {
    let s = |name: &str, exe: f32| f64::from(super::setting(order, name, exe));
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let sneak = |who: FormId| facts.current_actor_value(who, 42).unwrap_or(0.0);
    let raw = s("fPickPocketActorSkillBase", 0.0)
        + s("fPickPocketActorSkillMult", 1.0) * sneak(PLAYER_REF)
        + s("fPickPocketTargetSkillBase", 0.0)
        + s("fPickPocketTargetSkillMult", -1.0) * f64::from(sneak(target) as i32)
        + s("fPickPocketAmountBase", 0.0)
        + s("fPickPocketAmountMult", -3.0) * value;
    let (min, max) = (
        s("fPickPocketMinChance", 5.0),
        s("fPickPocketMaxChance", 75.0),
    );
    let mut c = raw.trunc() as i32;
    if f64::from(c) < min {
        c = min.trunc() as i32;
    }
    if f64::from(c) > max {
        c = max.trunc() as i32;
    }
    c
}

/// V when taking (`0075e240`): the item's value × how many.
pub fn stack_value(order: &LoadOrder, state: &GameState, item: FormId, count: i32) -> f64 {
    let unit = crate::items::value(order, state, item);
    f64::from(unit) * f64::from(count.max(0))
}

/// Whether placing `item` arms it (`0075d510`): a weapon of animation type
/// 10–13 whose projectile (`DNAM` at 36) is a lobber (`PROJ` `DATA` u16 type
/// at 2 is 2).
pub fn is_live_grenade(order: &LoadOrder, item: FormId) -> bool {
    let Some(rr) = order
        .get(item)
        .filter(|r| r.entry.header.kind.as_bytes() == b"WEAP")
    else {
        return false;
    };
    let Ok(record) = rr.record() else {
        return false;
    };
    let Some(dnam) = record
        .get(FourCC::new(b"DNAM"))
        .filter(|s| s.data.len() >= 40)
    else {
        return false;
    };
    let kind = crate::cell::le_u32(&dnam.data, 0);
    if !(10..=13).contains(&kind) {
        return false;
    }
    let projectile = rr
        .plugin
        .to_global(FormId(crate::cell::le_u32(&dnam.data, 36)));
    order
        .get(projectile)
        .filter(|p| p.entry.header.kind.as_bytes() == b"PROJ")
        .and_then(|p| p.record().ok())
        .and_then(|p| p.get(esm::sig::DATA).map(|s| s.data.clone()))
        .is_some_and(|d| d.len() >= 4 && u16::from_le_bytes([d[2], d[3]]) == 2)
}

/// A visit to someone's pockets (the container screen in its pickpocket
/// mode).
#[derive(Debug, Clone, PartialEq)]
pub struct Visit {
    pub target: FormId,
    /// The running total of what's been taken (+V) and given (−V), as the
    /// menu keeps it (menu+0x84; its use isn't traced).
    pub total: f64,
    /// A theft has been counted in "Pockets Picked" this visit.
    pub counted: bool,
    /// Caught: no more tries, and the screen closes.
    pub caught: bool,
}

impl Visit {
    pub fn new(target: FormId) -> Visit {
        Visit {
            target,
            total: 0.0,
            counted: false,
            caught: false,
        }
    }
}

/// What a move did.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Moved {
    /// How many moved.
    pub count: i32,
    /// Caught (the screen closes; the notice is queued).
    pub caught: bool,
}

/// The player tries to move `count` of `item`: from the person's pockets
/// (`taking`) or into them. `roll` is the game's random 0–99.
pub fn attempt(
    order: &LoadOrder,
    state: &mut GameState,
    visit: &mut Visit,
    item: FormId,
    count: i32,
    taking: bool,
    roll: u32,
) -> Moved {
    let target = visit.target;
    if visit.caught || count <= 0 {
        return Moved::default();
    }
    let (from, to) = if taking {
        (target, PLAYER_REF)
    } else {
        (PLAYER_REF, target)
    };
    // The player's own things come back without a roll.
    if taking {
        let planted = state
            .living
            .planted
            .get(&(target, item))
            .copied()
            .unwrap_or(0);
        if planted > 0 {
            let n = state.move_item(order, from, to, item, count.min(planted));
            let left = planted - n;
            if left > 0 {
                state.living.planted.insert((target, item), left);
            } else {
                state.living.planted.remove(&(target, item));
            }
            return Moved {
                count: n,
                ..Moved::default()
            };
        }
    }
    let value = if taking {
        stack_value(order, state, item, count)
    } else {
        1.0
    };
    if taking {
        visit.total += value;
    } else {
        visit.total -= value;
    }
    if value > 0.0 {
        let chance = chance(order, state, target, value);
        let evil = base_of(order, target).is_some_and(|b| crate::crime::owner_is_evil(order, b));
        if !evil {
            let karma = super::setting(order, "fKarmaModStealing", -5.0).trunc() as i32;
            crate::reputation::reward_karma(order, state, karma);
        }
        if roll >= chance.max(0) as u32 {
            visit.caught = true;
            caught(order, state, target);
            return Moved {
                caught: true,
                ..Moved::default()
            };
        }
        if taking && !visit.counted {
            visit.counted = true;
            crate::stats::bump(state, crate::stats::POCKETS_PICKED, 1);
        }
    }
    let n = state.move_item(order, from, to, item, count);
    if !taking && n > 0 {
        *state.living.planted.entry((target, item)).or_insert(0) += n;
    }
    Moved {
        count: n,
        ..Moved::default()
    }
}

/// Caught pickpocketing `victim` (`0075e0b0`'s failure, then the crime
/// `008c00e0`; see the module notes).
pub fn caught(order: &LoadOrder, state: &mut GameState, victim: FormId) {
    // Type 2: the sad Vault Boy.
    super::notice_with(
        state,
        super::text(
            order,
            "sPickpocketFail",
            "You've been caught pickpocketing.",
        ),
        Some(crate::message_icon::SAD),
    );
    state.living.caught_by.insert(victim);
    if crate::script_functions::ignores_crime(state, victim) || state.dead.contains(&victim) {
        return;
    }
    crate::crime::witnessed(order, state, victim, 1, 0, "fReputationMinorCrimeNeg", 2.0);
    // The same record as thefts: a second crime the same day of the month
    // and they attack; another day clears it.
    let day = state.global(order, "GameDay").unwrap_or(0.0) as u32;
    match state.last_theft_day {
        None => state.last_theft_day = Some(day),
        Some(d) if d == day => {
            state.combat.insert(victim, PLAYER_REF);
        }
        Some(_) => state.last_theft_day = None,
    }
}
