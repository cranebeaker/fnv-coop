//! Every animation the other player plays, on its avatar (docs/coop/SPECS.md S39), through the
//! game's own animation code.
//!
//! An actor's body is driven by its process and its `Animation` (the eight sections of
//! `Actor::PickAnimations`, `actor_p3.rs`): the movement flags pick walking, running, sneaking and
//! turning on the spot (section 1); the character controller's state picks the jump, the fall and
//! the landing; the sit and sleep state and the furniture pick sitting, sleeping and using a
//! workbench; the knock state (driven by fatigue) knocks it down and gets it up; the iron sights
//! pick aiming; anything else (a hit reaction, a stagger, a block, an equip, a death) is a group
//! started in one of the other sections, and an idle (a gesture, a fidget, a furniture idle) is
//! an `AnimIdle` the process plays.
//!
//! - [`body`] reads all of that of an actor (this game's player's third-person animation for the
//!   player); [`replayable`] says whether a group the owner's game started is one the avatar
//!   would not start by itself (not a movement, draw, attack or aim, which have their own ways).
//! - On an avatar: [`play_group`] plays a group as the `PlayGroup` command does, [`play_idle`] an
//!   idle as `PlayIdle` does, [`jump`] is the mobile object's own jump, [`set_sights`] is
//!   `Actor::SetIronSights`, [`set_speeds`] stores the owner's walking and running speeds in its
//!   process (so its legs move at the owner's speed), [`knock`] lowers or restores its fatigue
//!   (the knock state's own trigger), [`sit`] gives it the sit package the player's furniture
//!   activation gives, [`get_up`] the package the player's getting up gives; [`turn`] adds the
//!   turn-on-the-spot flags for a heading that changes while standing.
//!
//! Nothing here runs unless a session asks for it, and nothing is wrapped.

use engine_core::prelude::*;

use crate::{addr, forms, live};

/// The fatigue an avatar is knocked down with, and given back to get up (co-op's own number: the
/// game knocks an actor down below zero, `middlehighprocess.rs`,
/// `middle_high_process_update_knock_state`, and the restore is capped at its maximum).
pub const KNOCK_FATIGUE: f32 = 1000.0;
/// A heading that changes by more than this between two frames while standing is a turn on the
/// spot (radians; co-op's own number).
pub const TURN_MIN: f32 = 0.01;

/// What an actor's body is doing, as [`body`] reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    /// The animation group id playing in each section ([`addr::ANIM_GROUP_NONE`] for none).
    pub groups: [u16; addr::ANIMATION_SECTIONS as usize],
    /// The form id of the idle playing, 0 for none.
    pub idle: u32,
    /// The character controller's state ([`addr::CONTROLLER_STATE`]), 0 without one.
    pub controller: u32,
    /// The sit and sleep state, 0 for none.
    pub sit: i32,
    /// The knock state, 0 for none.
    pub knock: i32,
    /// The reference id of the furniture it uses, 0 for none.
    pub furniture: u32,
    /// It looks down its sights.
    pub sights: bool,
    /// Its walking and running speeds.
    pub walk: f32,
    /// See [`Body::walk`].
    pub run: f32,
}

impl Default for Body {
    fn default() -> Body {
        Body {
            groups: [addr::ANIM_GROUP_NONE; addr::ANIMATION_SECTIONS as usize],
            idle: 0,
            controller: 0,
            sit: 0,
            knock: 0,
            furniture: 0,
            sights: false,
            walk: 0.0,
            run: 0.0,
        }
    }
}

/// Whether the program has the animation functions this module reads (the translation, or a
/// test's stand-ins).
pub fn available(e: &Engine) -> bool {
    e.is_registered(addr::ANIMATION_GROUP_OF_SLOT)
}

/// The function behind virtual `slot` of the object at `this`, 0 when there is none (an object
/// whose table lacks it, which the game's own tables never do).
fn slot_fn(e: &mut Engine, this: u32, slot: u32) -> u32 {
    if this == 0 || !e.mem.is_mapped(this) {
        return 0;
    }
    let table = e.mem.u32(this);
    if table == 0 || !e.mem.is_mapped(table.wrapping_add(slot)) {
        return 0;
    }
    e.mem.u32(table.wrapping_add(slot))
}

/// Virtual `slot` of `this` with `args`; `None` when the object has no such function.
fn vcall_if(e: &mut Engine, this: u32, slot: u32, args: &[u32]) -> Option<Ret> {
    (slot_fn(e, this, slot) != 0).then(|| e.vcall(this, slot, args))
}

fn process(e: &mut Engine, actor: u32) -> u32 {
    e.mem.u32(actor.wrapping_add(addr::ACTOR_PROCESS_OFFSET))
}

fn id_of(e: &mut Engine, form: u32) -> u32 {
    match form {
        0 => 0,
        f => e.mem.u32(f.wrapping_add(addr::FORM_ID_OFFSET)),
    }
}

/// An actor's animation: for this game's player its third-person one
/// ([`addr::PLAYER_GET_ANIMATION`] with 0), for any other actor its virtual `0x1e4`.
fn animation(e: &mut Engine, actor: u32) -> u32 {
    if live::player(e) == Some(actor) && e.is_registered(addr::PLAYER_GET_ANIMATION) {
        return e
            .call(addr::PLAYER_GET_ANIMATION, &args![actor, 0u32])
            .u32();
    }
    vcall_if(e, actor, addr::ACTOR_GET_ANIMATION_SLOT, &args![]).map_or(0, |r| r.u32())
}

/// What the actor at `actor` (a pointer) is doing. A part the program has no function for reads
/// as nothing.
pub fn body(e: &mut Engine, actor: u32) -> Body {
    let mut out = Body::default();
    if !available(e) {
        return out;
    }
    let anim = animation(e, actor);
    if anim != 0 {
        for (s, g) in out.groups.iter_mut().enumerate() {
            *g = e
                .call(addr::ANIMATION_GROUP_OF_SLOT, &args![anim, s as u32])
                .u32() as u16;
        }
        let idle = e.mem.u32(anim + addr::ANIMATION_IDLE_OFFSET);
        if idle != 0 {
            let form = e.mem.u32(idle + addr::ANIM_IDLE_FORM_OFFSET);
            out.idle = id_of(e, form);
        }
    }
    if e.is_registered(addr::MOBILE_GET_CHAR_CONTROLLER) && e.is_registered(addr::CONTROLLER_STATE)
    {
        let c = e
            .call(addr::MOBILE_GET_CHAR_CONTROLLER, &args![actor])
            .u32();
        if c != 0 {
            out.controller = e.call(addr::CONTROLLER_STATE, &args![c]).u32();
        }
    }
    let p = process(e, actor);
    if p != 0 {
        out.sit =
            vcall_if(e, p, addr::PROCESS_SIT_SLEEP_STATE_SLOT, &args![]).map_or(0, |r| r.i32());
        out.knock = vcall_if(e, p, addr::PROCESS_KNOCK_STATE_SLOT, &args![])
            .map_or(0, |r| i32::from(r.u32() as u8 as i8));
        let f =
            vcall_if(e, p, addr::PROCESS_CURRENT_FURNITURE_SLOT, &args![]).map_or(0, |r| r.u32());
        out.furniture = id_of(e, f);
    }
    if e.is_registered(addr::ACTOR_GET_IRON_SIGHTS) {
        out.sights = e.call(addr::ACTOR_GET_IRON_SIGHTS, &args![actor]).u32() & 0xff != 0;
    }
    if e.is_registered(addr::ACTOR_GET_WALK_SPEED) && e.is_registered(addr::ACTOR_GET_RUN_SPEED) {
        out.walk = e.call(addr::ACTOR_GET_WALK_SPEED, &args![actor]).f32();
        out.run = e.call(addr::ACTOR_GET_RUN_SPEED, &args![actor]).f32();
    }
    out
}

/// This game's player's [`Body`]; `None` before the game has a player.
pub fn player_body(e: &mut Engine) -> Option<Body> {
    let p = live::player(e)?;
    Some(body(e, p))
}

/// The [`Body`] of avatar (or any actor) `id` in this game; `None` when it is not here.
pub fn body_of_id(e: &mut Engine, id: u32) -> Option<Body> {
    let a = forms::form(e, id)?;
    Some(body(e, a))
}

/// A group id's type: its low byte ([`addr::ANIM_GROUP_TYPE_DRAW`]).
pub fn group_type(group: u16) -> u8 {
    group as u8
}

/// Whether group `group`, started in section `section` of this game's player, is one its avatar
/// is to be given as a group: not in the movement section (the avatar's own movement flags and
/// controller pick those), not drawing or putting away (the draw call), not an attack (the
/// attack wrap and `QueueAttack`) or an aim (the iron sights), as the game's own `TESAnimGroup`
/// tests say.
pub fn replayable(e: &mut Engine, section: u32, group: u16) -> bool {
    if group == addr::ANIM_GROUP_NONE || section == addr::ANIMATION_MOVEMENT_SECTION {
        return false;
    }
    let kind = group_type(group);
    if kind == addr::ANIM_GROUP_TYPE_DRAW || kind == addr::ANIM_GROUP_TYPE_HOLSTER {
        return false;
    }
    for test in [addr::ANIM_GROUP_IS_ATTACK, addr::ANIM_GROUP_IS_AIM] {
        if e.is_registered(test) && e.call(test, &args![u32::from(group)]).u32() & 0xff != 0 {
            return false;
        }
    }
    true
}

/// Has avatar `id` play the group of type `kind` as the `PlayGroup` command plays one: the
/// actor's own group for the type (`Actor::GetAnimGroup`), started now in the section of its
/// type (`Animation::PlayGroup(group, 1, -1, -1)`), and the command's animation action with
/// section 1's sequence. False when it is not here or has no animation.
pub fn play_group(e: &mut Engine, id: u32, kind: u8) -> bool {
    let Some(a) = forms::form(e, id) else {
        return false;
    };
    if !available(e) || !e.is_registered(addr::ANIMATION_PLAY_GROUP) {
        return false;
    }
    let anim = animation(e, a);
    if anim == 0 {
        return false;
    }
    let group = e
        .call(
            addr::ACTOR_GET_ANIM_GROUP,
            &args![a, u32::from(kind), 0u32, 0u32, 0u32],
        )
        .u32()
        & 0xffff;
    e.call(
        addr::ANIMATION_PLAY_GROUP,
        &args![anim, group, 1u32, 0xffff_ffffu32, 0xffff_ffffu32],
    );
    if kind != 0 && e.is_registered(addr::ACTOR_SET_ANIM_ACTION) {
        let sequence = e
            .call(addr::ANIMATION_SECTION_SEQUENCE, &args![anim, 1u32])
            .u32();
        e.call(
            addr::ACTOR_SET_ANIM_ACTION,
            &args![a, addr::ANIM_ACTION_SCRIPT, sequence],
        );
    }
    true
}

/// Has avatar `id` play idle `idle` (a form id) as `PlayIdle` does: its process is given the
/// idle and told to play it. Nothing when it plays that idle already; false when it is not here,
/// has no process or the idle is not in this game.
pub fn play_idle(e: &mut Engine, id: u32, idle: u32) -> bool {
    let (Some(a), Some(form)) = (forms::form(e, id), forms::form(e, idle)) else {
        return false;
    };
    if !available(e) {
        return false;
    }
    let anim = animation(e, a);
    if anim != 0 {
        let held = e.mem.u32(anim + addr::ANIMATION_IDLE_OFFSET);
        if held != 0 && e.mem.u32(held + addr::ANIM_IDLE_FORM_OFFSET) == form {
            return false;
        }
    }
    let p = process(e, a);
    if p == 0 || vcall_if(e, p, addr::PROCESS_SET_IDLE_SLOT, &args![form]).is_none() {
        return false;
    }
    vcall_if(
        e,
        p,
        addr::PROCESS_QUEUE_FLAGS_SLOT,
        &args![addr::PROCESS_FLAG_PLAY_IDLE],
    );
    true
}

/// Has avatar `id` jump: the mobile object's own jump, which its controller carries out (its
/// own `PickAnimations` then plays the jump, the fall and the landing). False when it is not
/// here or has no controller.
pub fn jump(e: &mut Engine, id: u32) -> bool {
    let Some(a) = forms::form(e, id) else {
        return false;
    };
    if !e.is_registered(addr::MOBILE_JUMP) || !e.is_registered(addr::MOBILE_GET_CHAR_CONTROLLER) {
        return false;
    }
    if e.call(addr::MOBILE_GET_CHAR_CONTROLLER, &args![a]).u32() == 0 {
        return false;
    }
    e.call(addr::MOBILE_JUMP, &args![a]);
    true
}

/// Has avatar `id` look down its sights (`true`) or not: `Actor::SetIronSights(on, 0, 0)`.
pub fn set_sights(e: &mut Engine, id: u32, on: bool) -> bool {
    match forms::form(e, id) {
        Some(a) if e.is_registered(addr::ACTOR_SET_IRON_SIGHTS) => {
            e.call(
                addr::ACTOR_SET_IRON_SIGHTS,
                &args![a, u32::from(on), 0u32, 0u32],
            );
            true
        }
        _ => false,
    }
}

/// Stores `walk` and `run` as avatar `id`'s walking and running speeds in its process, so its
/// walking and running groups play at those speeds. False when it is not here or has no process.
pub fn set_speeds(e: &mut Engine, id: u32, walk: f32, run: f32) -> bool {
    let Some(a) = forms::form(e, id) else {
        return false;
    };
    let p = process(e, a);
    if p == 0
        || !e.is_registered(addr::PROCESS_SET_WALK_SPEED)
        || !e.is_registered(addr::PROCESS_SET_RUN_SPEED)
    {
        return false;
    }
    e.call(addr::PROCESS_SET_WALK_SPEED, &args![p, walk.to_bits()]);
    e.call(addr::PROCESS_SET_RUN_SPEED, &args![p, run.to_bits()]);
    true
}

/// Knocks avatar `id` down (`true`: its fatigue lowered by [`KNOCK_FATIGUE`], below zero) or has
/// it get up (its fatigue given back), through the actor's change of a value; the game's knock
/// state does the rest (falling, lying, the get-up idle). False when it is not here.
pub fn knock(e: &mut Engine, id: u32, down: bool) -> bool {
    let Some(a) = forms::form(e, id) else {
        return false;
    };
    if slot_fn(e, a, addr::ACTOR_MODIFY_VALUE_SLOT) == 0 {
        return false;
    }
    let by = if down { -KNOCK_FATIGUE } else { KNOCK_FATIGUE };
    e.vcall(
        a,
        addr::ACTOR_MODIFY_VALUE_SLOT,
        &args![addr::ACTOR_VALUE_FATIGUE, by.to_bits(), 0u32],
    );
    true
}

/// Gives avatar `id` the package the player's furniture activation gives it
/// (`PlayerCharacter::InitiateSitSleepPackage`): a sit and sleep package at furniture `furniture`
/// (a reference id), so the game has it sit, lie or use it. False when either is not here or the
/// program lacks the package functions.
pub fn sit(e: &mut Engine, id: u32, furniture: u32) -> bool {
    let (Some(a), Some(f)) = (forms::form(e, id), forms::form(e, furniture)) else {
        return false;
    };
    if !e.is_registered(addr::PACKAGE_CREATE) || slot_fn(e, a, addr::ACTOR_PUT_PACKAGE_SLOT) == 0 {
        return false;
    }
    let kind = addr::PACKAGE_TYPE_SIT_SLEEP;
    let package = e.call(addr::PACKAGE_CREATE, &args![kind]).u32();
    if package == 0 {
        return false;
    }
    e.call(addr::PACKAGE_SET_TYPE, &args![package, kind]);
    e.call(addr::PACKAGE_SET_826B40, &args![package, 0u32]);
    e.call(addr::PACKAGE_SET_826B90, &args![package, 1u32]);
    let block = e
        .call(addr::OPERATOR_NEW, &args![addr::PACKAGE_LOCATION_SIZE])
        .u32();
    let location = match block {
        0 => 0,
        b => e.call(addr::PACKAGE_LOCATION_CONSTRUCT, &args![b]).u32(),
    };
    e.call(addr::PACKAGE_LOCATION_SET_REFERENCE, &args![location, f]);
    e.call(addr::PACKAGE_SET_LOCATION, &args![package, location]);
    if location != 0 {
        e.call(addr::LOCATION_DELETE, &args![location, 1u32]);
    }
    e.call(addr::PACKAGE_SET_984F60, &args![package, 0u32]);
    e.vcall(a, addr::ACTOR_PUT_PACKAGE_SLOT, &args![package, 0u32, 1u32]);
    true
}

/// Has avatar `id` get up from its furniture: the package `PlayerCharacter::InitiateGetUpPackage`
/// gives (`008a75a0`). False when it is not here.
pub fn get_up(e: &mut Engine, id: u32) -> bool {
    match forms::form(e, id) {
        Some(a) if e.is_registered(addr::ACTOR_GET_UP_PACKAGE) => {
            e.call(addr::ACTOR_GET_UP_PACKAGE, &args![a]);
            true
        }
        _ => false,
    }
}

/// The movement flags to force on an avatar whose player has `moves` and whose heading goes from
/// `from` to `to` this frame: a player standing (no direction or turn flags) whose heading
/// changed by more than [`TURN_MIN`] turns on the spot, left for a rising heading and right for
/// a falling one, as the game's own turning in place sets the flags.
pub fn turn(moves: u16, from: f32, to: f32) -> u16 {
    if moves & (addr::MOVE_DIRECTIONS | addr::MOVE_TURN_LEFT | addr::MOVE_TURN_RIGHT) != 0 {
        return moves;
    }
    let tau = std::f32::consts::TAU;
    let mut d = (to - from).rem_euclid(tau);
    if d > tau / 2.0 {
        d -= tau;
    }
    if d > TURN_MIN {
        moves | addr::MOVE_TURN_LEFT
    } else if d < -TURN_MIN {
        moves | addr::MOVE_TURN_RIGHT
    } else {
        moves
    }
}

#[cfg(test)]
#[path = "anim_tests.rs"]
mod tests;
