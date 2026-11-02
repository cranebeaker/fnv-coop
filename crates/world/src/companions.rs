//! Companions' wheel (`CompanionWheelMenu`, class 1075, vtable `01071d0c`
//! in FalloutNV.exe; notes in `docs/COMPANIONS.md`): what the menu reads
//! from the companion and what its buttons do to them.
//!
//! - Using a teammate (the player using them, `005fa330`, `00607990`):
//!   the wheel when they can take orders (`00754d90`: alive and up, and not
//!   about to attack the player); else nothing.
//! - Its four switches read the companion's script variables
//!   (`00754de0`, `008c1740`; -1 for one their script lacks):
//!   `FollowerSwitchAggressive` 1, `IsFollowingLong` 1, `CombatStyleRanged`
//!   1, `Waiting` not 0.
//! - A button said as a topic (`007573d0`: the companion's line for it,
//!   `0061a720`) and done (`007575b0`: the line's begin and end result
//!   scripts run on them at once, `005ac1e0`); the aggressive switch also
//!   sets `FollowerSwitchAggressive` itself (`008c16c0`).
//! - A Stimpak (`00756490`): with one in the player's things (`00576260`),
//!   a companion hurt (health below full: `00893590`, current over base
//!   Health, 1 when the base is 0) or with a crippled limb (a condition,
//!   actor values 25 to 30, at 0) gets the Stimpak's effects when hurt
//!   (`008c1f80`), the player loses one (vtable +0x17C), every condition
//!   (25 to 31, the brain's too) is restored by 1000 (`0088b740`), and they
//!   say their `Regenerating` line (`008d5910(13)`, `007573d0`).

use esm::{FormId, LoadOrder};

use crate::dialogue::{Info, Speaker, PLAYER_REF};
use script::interp::Locals;

use crate::scripting::{base_of, script_of, GameState, Runner, ScriptCache};

/// The menu's class.
pub const CLASS: i32 = 1075;

/// A companion's script variable by name (`008c1740`): -1 when their
/// script has none. A script that hasn't run yet has its variables at 0,
/// as the game's are from the start.
pub fn variable(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &GameState,
    who: FormId,
    name: &str,
) -> f32 {
    let name = name.to_ascii_lowercase();
    let value = match state.variables.get(&who) {
        Some(l) => l.get(&name),
        None => new_locals(order, scripts, who).and_then(|l| l.get(&name)),
    };
    value.map_or(-1.0, |v| v as f32)
}

/// Sets one (`008c16c0`), when their script has it.
pub fn set_variable(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    who: FormId,
    name: &str,
    value: f32,
) -> bool {
    let locals = match state.variables.entry(who) {
        std::collections::hash_map::Entry::Occupied(e) => e.into_mut(),
        std::collections::hash_map::Entry::Vacant(e) => {
            let Some(l) = new_locals(order, scripts, who) else {
                return false;
            };
            e.insert(l)
        }
    };
    locals.set(&name.to_ascii_lowercase(), f64::from(value))
}

fn new_locals(order: &LoadOrder, scripts: &ScriptCache, who: FormId) -> Option<Locals> {
    let script = scripts.script(order, script_of(order, who)?)?;
    Some(Locals::new(&script))
}

/// The wheel's four switches as the companion's variables have them
/// (`00754de0`: the menu's bytes +0x6D, +0x6F, +0x6C, +0x6E; buttons 0, 2,
/// 5 and 7 show their other picture, `user11`, when on).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Switches {
    pub aggressive: bool,
    pub waiting: bool,
    pub keep_distance: bool,
    pub ranged: bool,
}

pub fn switches(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &GameState,
    who: FormId,
) -> Switches {
    let v = |name: &str| variable(order, scripts, state, who, name);
    Switches {
        aggressive: v("FollowerSwitchAggressive") == 1.0,
        // The menu keeps "following" (`Waiting` 0); the button shows the
        // other.
        waiting: v("Waiting") != 0.0,
        keep_distance: v("IsFollowingLong") == 1.0,
        ranged: v("CombatStyleRanged") == 1.0,
    }
}

/// Whether using a teammate brings up the wheel (`00754d90`): alive and
/// up (`00884480`: not dead or unconscious), and they wouldn't attack the
/// player (`008b06d0`, `GetShouldAttack`: [`crate::factions::attacks_on_sight`]).
pub fn wheel_allowed(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    if state.dead.contains(&who) || state.unconscious.contains(&who) {
        return false;
    }
    !crate::factions::attacks_on_sight(order, state, who, PLAYER_REF)
}

/// A companion says their line for a topic (`007573d0`, as the container
/// menu's bark `0075ec60`): the topic's line for them and the player
/// (`0061a720`), whose first response is voiced and put on the menu's
/// subtitle. With `run_scripts` the line is picked **again** (`007575b0`
/// calls `0061a720` itself) and that one's result scripts run on them at
/// once: the begin script, then the end script, straight through the
/// script runner (`005ac1e0`). Neither path goes through the dialogue's
/// own result-script call (`0061f170`, which also marks a say-once line
/// said for an actor, `00935920`), so a line said here isn't marked said,
/// its topics aren't added and the companion isn't counted as talked to.
/// The line said, if they have one.
///
/// Translated from 007573d0, 007575b0 (decompiled, FalloutNV.exe
/// 1.4.0.525).
pub fn say_topic(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    who: FormId,
    topic: &str,
    run_scripts: bool,
) -> Option<Info> {
    let topic = order.form_by_editor_id(topic)?;
    let base = base_of(order, who)?;
    let speaker = Speaker::load(order, who, base)?;
    let said = crate::dialogue::pick(order, topic, &speaker, state);
    if run_scripts {
        if let Some(info) = crate::dialogue::pick(order, topic, &speaker, state) {
            for source in [info.begin_script.as_deref(), info.end_script.as_deref()]
                .into_iter()
                .flatten()
            {
                Runner::new(order, scripts, state).run_source(source, Some(who), Some(who));
            }
        }
    }
    said
}

/// The wheel's Back Up (`00756930` → `008a7760`,
/// `Actor::InitiateBackUpPackage` (Xbox PDB)): the companion's default
/// package 0x27, a `BackUpPackage` aimed at the player, put on them
/// ([`crate::scripting::Event::BackUp`]; packages are the AI's).
pub fn back_up(state: &mut GameState, who: FormId) {
    state.events.push(crate::scripting::Event::BackUp(who));
}

/// Charisma's actor value.
const CHARISMA: u16 = 8;

/// Nerve: what the player's Charisma does for their teammates' fighting.
/// For the player's teammate (actor `+0x18d`, `00566950`) 1 + 0.05 × the
/// player's Charisma as the game asks it (`0066ef50(8)` on the player:
/// the value now, kept within 1 to 10 by `0066f190` since Charisma's
/// flags are 0x8009, `0066f260`); 1 for anyone else. It multiplies:
/// - a teammate's weapon or fists damage (`00644ce0`, the last factor),
///   and their melee, unarmed and creature attacks' damage once more as
///   the hit is made (`009b5170`; a shot's, `009b5650`, isn't);
/// - a teammate's damage resistance (its share out of 100) and damage
///   threshold when they're hit (`009b5a30`, before the ammunition's
///   effects and the `fMaxArmorRating` limit).
///
/// Translated from 00644ce0, 009b5170, 009b5a30 (decompiled,
/// FalloutNV.exe 1.4.0.525).
pub fn nerve(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    if who == PLAYER_REF || !state.teammates.contains(&who) {
        return 1.0;
    }
    let charisma = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, CHARISMA)
    .unwrap_or(0.0)
    .clamp(1.0, 10.0) as f32;
    charisma * 0.05 + 1.0
}

/// Body slot 2, the upper body (`BMDT` bit 2).
const UPPER_BODY: u32 = 2;
/// The biped slots a person's armour is picked for (`006047c0`: 0 to 19).
const BIPED_SLOTS: u32 = 20;

/// An armour's body slots (`BMDT`), resistance (`DNAM` i16 at 0, kept ×
/// 100: the game's `004be080` divides it) and threshold (`DNAM` f32 at 4).
fn armour_figures(order: &LoadOrder, item: FormId) -> Option<(u32, f32, f32)> {
    let rr = order
        .get(item)
        .filter(|r| r.entry.header.kind.as_bytes() == b"ARMO")?;
    let record = rr.record().ok()?;
    let slots = crate::actor::Armor::load(order, item).map_or(0, |a| a.slots);
    let d = record
        .get(esm::FourCC::new(b"DNAM"))
        .map(|s| s.data.clone())
        .unwrap_or_default();
    let dr = if d.len() >= 2 {
        f32::from(u16::from_le_bytes([d[0], d[1]])) / 100.0
    } else {
        0.0
    };
    let dt = if d.len() >= 8 {
        f32::from_le_bytes([d[4], d[5], d[6], d[7]])
    } else {
        0.0
    };
    Some((slots, dr, dt))
}

/// What someone would wear in a body slot (`InventoryChanges::GetBestArmor`
/// (Xbox PDB), `004c8220`, asked to keep what's locked on): what they wear
/// there with `EquipItem`'s no-unequip flag ([`GameState::equip_locked`],
/// extra data 0x3e) stays; else of the armour they hold (count above 0)
/// covering the slot, the one scoring highest, the first on a tie (their
/// record's contents first, in its order, then the rest): its resistance
/// truncated (the game scales it by `00646d40`'s condition factor, but of
/// the item's health itself rather than its share of full health, which
/// is 1 for any health above 0.5) plus its threshold. A piece worn down
/// to no health isn't picked.
///
/// Translated from 004c8220 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn best_armour(order: &LoadOrder, state: &GameState, who: FormId, slot: u32) -> Option<FormId> {
    let covers =
        |item: FormId| armour_figures(order, item).is_some_and(|(s, _, _)| s & (1 << slot) != 0);
    if let Some(&locked) = state.equipped.get(&who).and_then(|worn| {
        worn.iter()
            .find(|&&i| covers(i) && state.equip_locked.contains(&(who, i)))
    }) {
        return Some(locked);
    }
    let held = state.inventory(order, who);
    let mut order_seen: Vec<FormId> = Vec::new();
    for (item, _) in crate::scripting::base_contents(order, who) {
        if !order_seen.contains(&item) {
            order_seen.push(item);
        }
    }
    for (item, _) in &held {
        if !order_seen.contains(item) {
            order_seen.push(*item);
        }
    }
    let mut best: Option<(f32, FormId)> = None;
    for item in order_seen {
        if !held.iter().any(|(i, n)| *i == item && *n > 0) {
            continue;
        }
        let Some((slots, dr, dt)) = armour_figures(order, item) else {
            continue;
        };
        if slots & (1 << slot) == 0 {
            continue;
        }
        if let (Some(c), Some(full)) = (
            state.weapon_health.get(&(who, item)),
            crate::repair::stated_health(order, item),
        ) {
            if c * full as f32 <= 0.0 {
                continue;
            }
        }
        let score = dr.trunc() + dt;
        if best.map_or(true, |(b, _)| score > b) {
            best = Some((score, item));
        }
    }
    best.map(|(_, item)| item)
}

/// A companion sorting out what they wear after trading (the container
/// menu closing in mode 3, `0075b750`; for a person `00606540` →
/// `TESNPC::InitDefaultWorn` (Xbox PDB), `006047c0`): for each body slot 0
/// to 19 in turn, [`best_armour`] for it is put on unless the upper-body
/// piece already put on covers this slot, it's worn already, or (in any
/// slot but the upper body's) it covers the upper body too; the upper
/// body's piece is the one later slots are measured against. Returns what
/// was put on.
///
/// The weapon part is [`hold_best_weapon`]; [`sort_out_gear`] does both.
///
/// Translated from 006047c0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn wear_best_armour(order: &LoadOrder, state: &mut GameState, who: FormId) -> Vec<FormId> {
    let covers = |item: FormId, slot: u32| {
        armour_figures(order, item).is_some_and(|(s, _, _)| s & (1 << slot) != 0)
    };
    let mut body: Option<FormId> = None;
    let mut put_on = Vec::new();
    for slot in 0..BIPED_SLOTS {
        let Some(item) = best_armour(order, state, who, slot) else {
            continue;
        };
        if body.is_some_and(|b| covers(b, slot)) || state.is_equipped(who, item) {
            continue;
        }
        if slot == UPPER_BODY {
            body = Some(item);
        } else if covers(item, UPPER_BODY) {
            continue;
        }
        state.equip(order, who, item);
        put_on.push(item);
    }
    put_on
}

/// The package flag "Weapons Unequipped" (`PKDT` flags 0x200000, read by
/// `00441b00`).
pub const WEAPONS_UNEQUIPPED: u32 = 0x0020_0000;

/// The weapon part of sorting out what someone holds
/// (`TESNPC::InitDefaultWorn` (Xbox PDB), `006047c0`, when asked to; a
/// creature's, `TESCreature::InitDefaultWorn`, `005f9e00`, the same):
/// what they wear has been taken off first (`004bfe50`, all but what a
/// script locked on, `004c0cf0`), so the weapon in hand goes unless it's
/// locked there; then the best weapon (`InventoryChanges::GetBestWeapon`
/// asked for any kind and to keep a locked one,
/// [`crate::dps::best_weapon`]) is put in hand (`0088c830`) unless it's
/// already there. Never the player's (`006047c0` asks the reference's
/// vtable +0x22c). With nothing to pick they hold nothing. `anims`: the
/// player's third-person animations, which the rating reads (see
/// `crate::dps`). Returns the weapon put in hand when it changed.
///
/// Translated from 006047c0, 005f9e00 (decompiled, FalloutNV.exe
/// 1.4.0.525).
pub fn hold_best_weapon(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    anims: Option<&mut dyn crate::animation::pick::Library>,
    s: crate::combat_ai::Setting,
) -> Option<FormId> {
    if who == PLAYER_REF {
        return None;
    }
    let held = crate::dps::equipped_weapon(order, state, who);
    let best = crate::dps::best_weapon(order, state, who, None, true, anims, s);
    if best == held {
        return None;
    }
    if let Some(w) = held.filter(|&w| !state.equip_locked.contains(&(who, w))) {
        state.unequip(who, w);
    }
    let b = best?;
    state.equip(order, who, b);
    Some(b).filter(|&b| state.is_equipped(who, b))
}

/// A companion sorting out what they wear and hold after trading (the
/// container menu closing in mode 3, `0075b750`): a person their armour
/// ([`wear_best_armour`]) and, unless the package they run now has
/// "Weapons Unequipped" ([`WEAPONS_UNEQUIPPED`]; `00606540` asks
/// `00441b00` of it), their weapon ([`hold_best_weapon`]); a creature
/// (`005f9e00` with the weapon asked for) only their weapon. What was put
/// on or in hand.
///
/// Translated from 0075b750, 00606540 (decompiled, FalloutNV.exe
/// 1.4.0.525).
pub fn sort_out_gear(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    anims: Option<&mut dyn crate::animation::pick::Library>,
    s: crate::combat_ai::Setting,
) -> Vec<FormId> {
    let creature = crate::combat::is_creature(order, who);
    let mut out = Vec::new();
    if !creature {
        out.extend(wear_best_armour(order, state, who));
        let unequipped = crate::ai::current_package(order, state, who)
            .is_some_and(|p| p.flags & WEAPONS_UNEQUIPPED != 0);
        if unequipped {
            return out;
        }
    }
    out.extend(hold_best_weapon(order, state, who, anims, s));
    out
}

/// How far from the player someone following them must be to be brought
/// along (`00973de0`: 350 units).
pub const FOLLOW_CATCH_UP: f32 = 350.0;

/// Whether someone is following the player (`009549a0`;
/// `PlayerCharacter::IsActorFollowingPlayer` (Xbox PDB)): not dead, not
/// disabled, and the package they run now a Follow (1) or Accompany (7)
/// one aimed at the player. (Also in the game, not looked at here: the
/// actor's vtable +0x234 and `00437bf0` tests, `008a6210`.)
///
/// Translated from 009549a0 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn follows_player(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    if who == PLAYER_REF
        || state.dead.contains(&who)
        || !crate::enabled_now(order, who, &state.disabled)
    {
        return false;
    }
    crate::ai::current_package(order, state, who)
        .and_then(|p| crate::ai::followed(&p))
        .is_some_and(|(target, _)| target == PLAYER_REF)
}

/// Who comes along when the player has been put somewhere — through a
/// load door, by a script's `MoveTo` or by fast travel (all end in the
/// player's positioning, `0093c200`, whose last step is `00973de0`; fast
/// travel, `0093cdf0`, first does the same for those following to its
/// marker, `00973ee0`): of the actors running near the player before
/// (`near`, the game's high-process list), each one alive that is the
/// player's teammate, or that follows the player ([`follows_player`]) and
/// is now more than [`FOLLOW_CATCH_UP`] away (in another place counts), is
/// moved to the player (`008ad910` → `Actor::MoveActorAndFollowers` (Xbox
/// PDB), `008ad1c0`) — except a teammate whose `Waiting` variable is 1,
/// whom `008ad1c0` passes over. Moved ones stand up from furniture, are put
/// where the player stands facing their way (the game first looks for a
/// spot on the navmesh behind the player, spaced by each one's radius,
/// `006e7e70`, and uses the player's own spot when it finds none; that
/// search is the pathing's and isn't done here) and look at their packages
/// again. Returns those moved.
///
/// Translated from 00973de0, 00973ee0, 008ad1c0 (decompiled,
/// FalloutNV.exe 1.4.0.525).
pub fn come_along(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    near: &[FormId],
) -> Vec<FormId> {
    let Some((space, cell, at, heading)) = state.place(order, PLAYER_REF) else {
        return Vec::new();
    };
    let mut moved = Vec::new();
    for &who in near {
        if who == PLAYER_REF || moved.contains(&who) || state.dead.contains(&who) {
            continue;
        }
        let teammate = state.teammates.contains(&who);
        let far = || match state.place(order, who) {
            Some((s, _, p, _)) => {
                s != space
                    || ((p[0] - at[0]).powi(2) + (p[1] - at[1]).powi(2) + (p[2] - at[2]).powi(2))
                        .sqrt()
                        > FOLLOW_CATCH_UP
            }
            None => false,
        };
        if !(teammate || (follows_player(order, state, who) && far())) {
            continue;
        }
        // `008ad1c0`: not a waiting teammate, nor someone disabled.
        if !crate::enabled_now(order, who, &state.disabled)
            || (teammate && variable(order, scripts, state, who, "Waiting") == 1.0)
        {
            continue;
        }
        state.stand(who);
        state.spaces.insert(who, (space, cell));
        state.positions.insert(who, (at, heading));
        state.evaluate.insert(who);
        moved.push(who);
    }
    moved
}

/// Whether the player is fighting: fighting someone, or someone fighting
/// them (as `IsInCombat` on the player reads here; the game's
/// `PlayerCharacter::IsPlayerCharacterInCombat` (Xbox PDB), `00953c50`,
/// keeps a flag at +0xdf0).
pub fn player_in_combat(state: &GameState) -> bool {
    state.combat.contains_key(&PLAYER_REF) || state.combat.values().any(|t| *t == PLAYER_REF)
}

/// Whether a knocked-out essential actor's time down runs this frame
/// (`00888b50`, life state 6): for the player's teammate only while the
/// player isn't in combat (so a companion knocked out in a fight gets up
/// once it's over, `fEssentialDeathTime` after); for anyone else always.
/// When it runs out they get up (`008a1800`, `008a0960`; a teammate's
/// also queues hint 11, `008d5cb0`). A teammate is essential only outside
/// Hardcore (`0087f3d0`, `more_functions::is_essential`): in Hardcore
/// they die. `combat::advance_down` holds the time with it.
///
/// Translated from 00888b50 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn down_time_runs(state: &GameState, who: FormId) -> bool {
    !(state.teammates.contains(&who) && player_in_combat(state))
}

/// What detecting a teammate hidden with the player gives (`008a0d10`:
/// −100).
pub const HIDDEN_WITH_PLAYER: i32 = -100;

/// Sneaking with the player (`008a0d10`, the detection test's start):
/// nobody detects the player's teammate while the teammate isn't fighting
/// (the actor's +0x104) and the player sneaks (`004997b0`) without
/// fighting either — the test gives [`HIDDEN_WITH_PLAYER`]. (The same
/// test also gives a teammate the player's Sneak skill when it's higher,
/// `Facts::detection_inputs`, and counts them invisible when the player
/// is, actor values 48 and 49, which the detection here doesn't model.)
///
/// Translated from 008a0d10 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn hidden_with_player(state: &GameState, who: FormId) -> bool {
    who != PLAYER_REF
        && state.teammates.contains(&who)
        && !state.combat.contains_key(&who)
        && state.player_sneaking
        && !player_in_combat(state)
}

/// The Stimpak (`DOBJ` default object 0).
pub fn stimpak(order: &LoadOrder) -> Option<FormId> {
    crate::items::default_object(order, 0)
}

/// The limbs' conditions (actor values 25 to 30), looked at for a cripple.
const LIMBS: [u16; 6] = [25, 26, 27, 28, 29, 30];
/// Those a Stimpak from the wheel restores: the brain's (31) too.
const RESTORED: [u16; 7] = [31, 29, 30, 27, 28, 26, 25];

/// The wheel's Stimpak (`00756490`): true when one was used.
pub fn heal_with_stimpak(order: &LoadOrder, state: &mut GameState, who: FormId) -> bool {
    let Some(stim) = stimpak(order) else {
        return false;
    };
    if state.item_count(order, PLAYER_REF, stim) <= 0 {
        return false;
    }
    let facts = crate::scripting::Facts {
        order,
        state,
        speaker: None,
    };
    let crippled = LIMBS
        .iter()
        .any(|&av| facts.current_actor_value(who, av).unwrap_or(100.0) == 0.0);
    let health = crate::combat::health(order, state, who).unwrap_or(0.0);
    let full = crate::combat::max_health(order, state, who).unwrap_or(0.0);
    let hurt = full > 0.0 && health / full < 1.0;
    if !hurt && !crippled {
        return false;
    }
    if hurt {
        crate::magic::apply(order, state, who, stim, PLAYER_REF, false);
    }
    state.stock(order, PLAYER_REF);
    if let Some(n) = state.items.get_mut(&(PLAYER_REF, stim)) {
        *n = (*n - 1).max(0);
    }
    for av in RESTORED {
        crate::magic::change(order, state, who, av, 1000.0, PLAYER_REF);
    }
    true
}
