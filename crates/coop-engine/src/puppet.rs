//! A game's copies of actors another game runs show what that game's actor does (docs/coop/
//! DECISIONS.md D6, stage B): they fight whom it fights and attack when it attacks.
//!
//! Each actor is run by one game (its owner: the host's, or the nearest guest's after a
//! hand-off); the other games have a copy of it, with its AI on so that it walks and is animated
//! by its own game. Left to itself, a copy chose its own combat target (often its own game's
//! player) and attacked on its own: in one game the actor faced one player while in the other it
//! shot the other one. Two wraps take that away from the copies, and from nothing else:
//!
//! - its target: a wrap of the combat controller's change of target (`00980830`, the one place
//!   every change goes through) puts the target the owner's actor has, as the session names it
//!   ([`set_copies`]), in place of the copy's own choice, when the copy's combat group knows that
//!   target. [`aim`] turns a copy to it at once; one whose group does not know it yet is started
//!   on a combat with it by the session (`fight::start_combat`).
//! - its attacks: a wrap of `Actor::QueueAttack` (`008935f0`, the combat procedures' only way to
//!   attack) refuses a copy's own attacks; the owner's are replayed on it ([`replay`]). On every
//!   other actor the wrap notes each attack its game queues ([`attack_of`]), for the session to
//!   publish with the actor's state.
//!
//! What a copy's attack does to a player is decided as before (`fight::hit_me`): a replayed
//! attack is still a hit in the copy's game, and the hit wrap keeps it there when the damage is
//! another game's.
//!
//! An actor its own game runs is never touched: the wraps do as the game does for any actor not
//! named a copy, and a copy left to its own AI (one in a conversation or a scene) is not named.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, forms};

/// Co-op's place in front of `Actor::QueueAttack`.
pub(crate) const QUEUE_ATTACK: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_QUEUE_ATTACK);

/// Co-op's place in front of the combat controller's change of target.
pub(crate) const SET_TARGET: WrapSlot = WrapSlot::new(Owner::Coop, addr::CONTROLLER_SET_TARGET);

/// Most actors whose attacks are noted (an actor no longer near anyone keeps its entry; past
/// this many the notes start afresh).
const NOTED_MAX: usize = 4096;

/// The last attack an actor's own game queued: a count that goes up by one with each attack
/// (wrapping), and the attack's animation group. Two states with the same count carry the same
/// attack.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attack {
    /// How many attacks were queued (wrapping); 0 before the first.
    pub seq: u8,
    /// The animation group of the last one.
    pub group: u8,
}

/// The copies' game-wide state (a lock-free shared slot, as the fights'): the wraps run on
/// whatever thread the game runs its AI on, the session writes between frames.
#[derive(Debug, Default)]
pub struct Puppets {
    /// The copies in this game: their form ids, each with the form id of the target its owner's
    /// actor fights (0: none, or not one this game has).
    copies: RwLock<BTreeMap<u32, u32>>,
    /// The actor whose attack is being replayed now (its pointer), 0 for none.
    replaying: AtomicU32,
    /// The attacks queued by actors this game runs, by form id.
    noted: Mutex<BTreeMap<u32, Attack>>,
    /// A copy's own attacks refused, ever.
    refused: AtomicU64,
    /// A copy's own choice of target replaced by its owner's, ever.
    turned: AtomicU64,
}

/// Installs the wraps where the game has the functions (the translation, or a test's
/// stand-ins); called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    if e.is_registered(addr::ACTOR_QUEUE_ATTACK) {
        e.wrap(QUEUE_ATTACK, queue_attack);
    }
    if e.is_registered(addr::CONTROLLER_SET_TARGET) {
        e.wrap(SET_TARGET, set_target);
    }
    let _ = e.ext_free::<Puppets>();
}

/// Names the copies in this game, each with its owner's target (a form id of this game: its
/// player `0x14`, an avatar, or another actor; 0 for none). Every other actor is its game's own.
pub fn set_copies(e: &mut Engine, copies: impl IntoIterator<Item = (u32, u32)>) {
    *e.ext_free::<Puppets>()
        .copies
        .write()
        .unwrap_or_else(PoisonError::into_inner) = copies.into_iter().collect();
}

/// Names one copy, or gives it its owner's new target, leaving the others as they are.
pub fn set_copy(e: &mut Engine, id: u32, target: u32) {
    e.ext_free::<Puppets>()
        .copies
        .write()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(id, target);
}

/// Whether actor `id` is a copy here (another game runs it).
pub fn is_copy(e: &mut Engine, id: u32) -> bool {
    locked(e, id).is_some()
}

fn locked(e: &mut Engine, id: u32) -> Option<u32> {
    e.ext_free::<Puppets>()
        .copies
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&id)
        .copied()
}

fn id_of(e: &mut Engine, r: u32) -> u32 {
    match r {
        0 => 0,
        r => e.mem.u32(r.wrapping_add(addr::FORM_ID_OFFSET)),
    }
}

/// The last attack this game queued for actor `id` (one it runs); `None` before its first.
pub fn attack_of(e: &mut Engine, id: u32) -> Option<Attack> {
    e.ext_free::<Puppets>()
        .noted
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&id)
        .copied()
}

/// How many of the copies' own attacks were refused, and how many of their own choices of
/// target were replaced by the owner's, ever.
pub fn counts(e: &mut Engine) -> (u64, u64) {
    let p = e.ext_free::<Puppets>();
    (
        p.refused.load(Ordering::SeqCst),
        p.turned.load(Ordering::SeqCst),
    )
}

/// The wrap at `008935f0`. Its words are the call's: the actor and the attack's animation group.
/// A copy's own attack is refused as the game refuses one it cannot make (false: the procedure
/// tries again later); a replayed one goes through. Any other actor's attack goes to the game,
/// and one it queues is noted.
pub(crate) fn queue_attack(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a.first().copied().unwrap_or(0);
    let group = a.get(1).copied().unwrap_or(0);
    let id = id_of(e, actor);
    let replaying = e.ext_free::<Puppets>().replaying.load(Ordering::SeqCst);
    if id != 0 && actor != replaying && locked(e, id).is_some() {
        e.ext_free::<Puppets>()
            .refused
            .fetch_add(1, Ordering::SeqCst);
        return Ret::default();
    }
    let before = e
        .mem
        .u32(actor.wrapping_add(addr::ACTOR_QUEUED_ATTACK_OFFSET));
    let r = e.call_next(addr::ACTOR_QUEUE_ATTACK, a);
    // A new attack: none was queued before and the game took this one.
    if r.eax & 0xff != 0 && before & 0xff == addr::NO_QUEUED_ATTACK {
        let mut noted = e
            .ext_free::<Puppets>()
            .noted
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if noted.len() >= NOTED_MAX && !noted.contains_key(&id) {
            noted.clear();
        }
        let n = noted.entry(id).or_default();
        n.seq = n.seq.wrapping_add(1);
        n.group = group as u8;
    }
    r
}

/// Whether the combat group of `controller` keeps `target` (a pointer) as one of its targets.
fn group_knows(e: &mut Engine, controller: u32, target: u32) -> bool {
    let group = e
        .mem
        .u32(controller.wrapping_add(addr::CONTROLLER_GROUP_OFFSET));
    group != 0 && e.call(addr::GROUP_TARGET_OF, &args![group, target]).u32() != 0
}

/// The wrap at `00980830`. Its words are the call's: the controller and the new target. A copy
/// whose owner fights a target its group knows is given that one, whatever its own choice was;
/// any other change goes to the game as it is.
pub(crate) fn set_target(e: &mut Engine, a: &[u32]) -> Ret {
    let controller = a.first().copied().unwrap_or(0);
    let target = a.get(1).copied().unwrap_or(0);
    if controller != 0 {
        let attacker = e
            .mem
            .u32(controller.wrapping_add(addr::CONTROLLER_ATTACKER_OFFSET));
        let id = id_of(e, attacker);
        // (a copy whose owner fights nobody has 0, which is no form)
        if let Some(lock) = locked(e, id) {
            if let Some(want) = forms::form(e, lock) {
                if want != target && group_knows(e, controller, want) {
                    e.ext_free::<Puppets>()
                        .turned
                        .fetch_add(1, Ordering::SeqCst);
                    return e.call_next(addr::CONTROLLER_SET_TARGET, &args![controller, want]);
                }
            }
        }
    }
    e.call_next(addr::CONTROLLER_SET_TARGET, a)
}

/// What [`aim`] found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aim {
    /// The copy or the target is not in this game now.
    Missing,
    /// The copy is not fighting here (it has no combat controller).
    NotFighting,
    /// It fights the target already.
    Already,
    /// It was turned to the target.
    Turned,
    /// It fights, but its combat group does not know the target yet.
    Unknown,
}

/// Turns copy `id` to fight `target` (form ids) now, through the game's own change of target,
/// when it fights and its group knows the target.
pub fn aim(e: &mut Engine, id: u32, target: u32) -> Aim {
    let (Some(a), Some(t)) = (actors::actor(e, id), forms::form(e, target)) else {
        return Aim::Missing;
    };
    let controller = e
        .vcall(a, addr::ACTOR_COMBAT_CONTROLLER_SLOT, &args![])
        .u32();
    if controller == 0 {
        return Aim::NotFighting;
    }
    if e.mem
        .u32(controller.wrapping_add(addr::CONTROLLER_TARGET_OFFSET))
        == t
    {
        return Aim::Already;
    }
    if !group_knows(e, controller, t) {
        return Aim::Unknown;
    }
    e.call(addr::CONTROLLER_SET_TARGET, &args![controller, t]);
    Aim::Turned
}

/// Replays the owner's attack (animation group `group`) on copy `id`: the game's own
/// `QueueAttack`, let through the wrap, so that the copy attacks as any actor does when its
/// weapon, its ammunition and its animation allow. False when the copy is not in this game now
/// or the game did not queue the attack.
pub fn replay(e: &mut Engine, id: u32, group: u8) -> bool {
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    let p = e.ext_free::<Puppets>();
    p.replaying.store(a, Ordering::SeqCst);
    let queued = e
        .call(addr::ACTOR_QUEUE_ATTACK, &args![a, u32::from(group)])
        .bool();
    e.ext_free::<Puppets>().replaying.store(0, Ordering::SeqCst);
    queued
}

/// Has copy `id` walk or run as its owner's actor does (`flags`: the owner's movement flags):
/// the game's own `ForceMoveMode`, so its animation is that gait while the session puts it on
/// the owner's path; 0 has it stand. False when it is not in this game now or has no mover.
pub fn force_moves(e: &mut Engine, id: u32, flags: u16) -> bool {
    match actors::actor(e, id) {
        Some(a) if e.mem.u32(a.wrapping_add(addr::ACTOR_MOVER_OFFSET)) != 0 => {
            e.call(addr::ACTOR_FORCE_MOVE_MODE, &args![a, u32::from(flags)]);
            true
        }
        _ => false,
    }
}

/// Gives copy `id` its own movement back (`ClearForcedMoveMode`). False when it is not in this
/// game now or has no mover.
pub fn free_moves(e: &mut Engine, id: u32) -> bool {
    match actors::actor(e, id) {
        Some(a) if e.mem.u32(a.wrapping_add(addr::ACTOR_MOVER_OFFSET)) != 0 => {
            e.call(addr::ACTOR_CLEAR_FORCED_MOVE_MODE, &args![a]);
            true
        }
        _ => false,
    }
}

/// Ends copy `id`'s combat here (`Actor::StopCombat`), for a copy whose owner no longer fights.
/// False when it is not in this game now.
pub fn stop(e: &mut Engine, id: u32) -> bool {
    match actors::actor(e, id) {
        Some(a) => {
            e.vcall(a, addr::ACTOR_STOP_COMBAT_SLOT, &args![0u32]);
            true
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const HOUSE: u32 = 0x0010_5a00;
    const RAIDER: u32 = 0x0010_4f25;
    const GECKO: u32 = 0x0010_4f26;
    const AVATAR: u32 = 0x0010_4f27;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![HOUSE],
            actors: vec![RAIDER, GECKO, AVATAR],
            ..StorySpec::default()
        });
        for id in [RAIDER, GECKO, AVATAR] {
            s.place_actor(id, HOUSE, [0.0; 3], 0.0);
        }
        s
    }

    #[test]
    fn an_actor_its_own_game_runs_attacks_as_the_game_has_it_and_the_attack_is_noted() {
        let mut s = story();
        s.set_combat(RAIDER, actors::PLAYER_REF_ID);
        assert_eq!(attack_of(&mut s.e, RAIDER), None);
        assert!(s.queue_attack(RAIDER, 0x20));
        assert_eq!(s.attacks_queued(RAIDER), 1);
        assert_eq!(
            attack_of(&mut s.e, RAIDER),
            Some(Attack {
                seq: 1,
                group: 0x20
            })
        );
        // Asked again while the attack is still queued: the same attack, not a new one.
        assert!(s.queue_attack(RAIDER, 0x20));
        assert_eq!(attack_of(&mut s.e, RAIDER).map(|a| a.seq), Some(1));
        // Once it is over, the next is a new one.
        s.finish_attack(RAIDER);
        assert!(s.queue_attack(RAIDER, 0x22));
        assert_eq!(
            attack_of(&mut s.e, RAIDER),
            Some(Attack {
                seq: 2,
                group: 0x22
            })
        );
        // Another actor of its own game is not a copy either.
        assert!(s.queue_attack(GECKO, 0x20));
        assert_eq!(counts(&mut s.e), (0, 0));
    }

    #[test]
    fn an_attack_the_game_refuses_is_not_noted() {
        let mut s = story();
        s.set_attack_refused(RAIDER, true);
        assert!(!s.queue_attack(RAIDER, 0x20));
        assert_eq!(attack_of(&mut s.e, RAIDER), None);
    }

    #[test]
    fn a_copys_own_attacks_are_refused_and_its_owners_replayed() {
        let mut s = story();
        set_copies(&mut s.e, [(RAIDER, actors::PLAYER_REF_ID)]);
        s.set_combat(RAIDER, AVATAR);
        assert!(!s.queue_attack(RAIDER, 0x20));
        assert_eq!(s.attacks_queued(RAIDER), 0);
        assert_eq!(counts(&mut s.e).0, 1);
        // A copy's attacks are not this game's to publish.
        assert_eq!(attack_of(&mut s.e, RAIDER), None);
        assert!(replay(&mut s.e, RAIDER, 0x21));
        assert_eq!(s.attacks_queued(RAIDER), 1);
        assert_eq!(s.queued_group(RAIDER), 0x21);
        // After the replay its own are refused again.
        s.finish_attack(RAIDER);
        assert!(!s.queue_attack(RAIDER, 0x20));
        assert_eq!(s.attacks_queued(RAIDER), 1);
        // Another actor is not touched.
        assert!(s.queue_attack(GECKO, 0x20));
        // A replay the game cannot make says so.
        s.set_attack_refused(RAIDER, true);
        s.finish_attack(RAIDER);
        assert!(!replay(&mut s.e, RAIDER, 0x21));
        // Given back to its own game, it attacks as before.
        set_copies(&mut s.e, []);
        s.set_attack_refused(RAIDER, false);
        assert!(s.queue_attack(RAIDER, 0x20));
    }

    #[test]
    fn a_copys_own_choice_of_target_is_replaced_by_its_owners() {
        let mut s = story();
        s.set_combat(RAIDER, AVATAR);
        s.know_target(RAIDER, actors::PLAYER_REF_ID);
        set_copies(&mut s.e, [(RAIDER, actors::PLAYER_REF_ID)]);
        // Its own update turns it to the avatar: it is given the player instead.
        s.choose_target(RAIDER, AVATAR);
        assert_eq!(s.actor_view(RAIDER).target, actors::PLAYER_REF_ID);
        assert_eq!(counts(&mut s.e).1, 1);
        // The owner's target is taken as it is.
        s.choose_target(RAIDER, actors::PLAYER_REF_ID);
        assert_eq!(s.actor_view(RAIDER).target, actors::PLAYER_REF_ID);
        assert_eq!(counts(&mut s.e).1, 1);
        // Its own losing of every target is the owner's target too.
        s.choose_target(RAIDER, 0);
        assert_eq!(s.actor_view(RAIDER).target, actors::PLAYER_REF_ID);
    }

    #[test]
    fn a_target_the_copys_group_does_not_know_is_not_forced_on_it() {
        let mut s = story();
        s.set_combat(RAIDER, AVATAR);
        set_copies(&mut s.e, [(RAIDER, GECKO)]);
        s.choose_target(RAIDER, AVATAR);
        assert_eq!(s.actor_view(RAIDER).target, AVATAR);
        assert_eq!(aim(&mut s.e, RAIDER, GECKO), Aim::Unknown);
        assert_eq!(s.actor_view(RAIDER).target, AVATAR);
        // Once it knows it (a combat started with it), it is turned.
        s.know_target(RAIDER, GECKO);
        assert_eq!(aim(&mut s.e, RAIDER, GECKO), Aim::Turned);
        assert_eq!(s.actor_view(RAIDER).target, GECKO);
        assert_eq!(aim(&mut s.e, RAIDER, GECKO), Aim::Already);
    }

    #[test]
    fn a_copy_whose_owner_fights_nobody_and_an_actor_its_game_runs_choose_their_own() {
        let mut s = story();
        s.set_combat(RAIDER, AVATAR);
        s.set_combat(GECKO, AVATAR);
        s.know_target(RAIDER, actors::PLAYER_REF_ID);
        s.know_target(GECKO, actors::PLAYER_REF_ID);
        set_copies(&mut s.e, [(RAIDER, 0)]);
        s.choose_target(RAIDER, actors::PLAYER_REF_ID);
        assert_eq!(s.actor_view(RAIDER).target, actors::PLAYER_REF_ID);
        set_copies(&mut s.e, [(RAIDER, AVATAR)]);
        s.choose_target(GECKO, actors::PLAYER_REF_ID);
        assert_eq!(s.actor_view(GECKO).target, actors::PLAYER_REF_ID);
        assert_eq!(counts(&mut s.e).1, 0);
    }

    #[test]
    fn a_controller_without_a_group_knows_no_target() {
        let mut s = story();
        s.set_combat(RAIDER, AVATAR);
        s.drop_group(RAIDER);
        set_copies(&mut s.e, [(RAIDER, AVATAR)]);
        assert_eq!(aim(&mut s.e, RAIDER, GECKO), Aim::Unknown);
        s.choose_target(RAIDER, GECKO);
        assert_eq!(s.actor_view(RAIDER).target, GECKO);
    }

    #[test]
    fn aiming_needs_a_fighting_copy_in_this_game() {
        let mut s = story();
        assert_eq!(
            aim(&mut s.e, RAIDER, actors::PLAYER_REF_ID),
            Aim::NotFighting
        );
        assert_eq!(aim(&mut s.e, 0x0010_9999, RAIDER), Aim::Missing);
        assert_eq!(aim(&mut s.e, RAIDER, 0x0010_9999), Aim::Missing);
    }

    #[test]
    fn a_copy_is_given_its_owners_gait_and_its_own_back() {
        let mut s = story();
        s.set_moves(RAIDER, 0x201);
        assert_eq!(
            crate::fight::snap_of(&mut s.e, RAIDER).unwrap().moves,
            0x201
        );
        assert!(force_moves(&mut s.e, GECKO, 0x101));
        assert_eq!(s.forced_moves(GECKO), Some(0x101));
        assert!(force_moves(&mut s.e, GECKO, 0));
        assert_eq!(s.forced_moves(GECKO), Some(0));
        assert!(free_moves(&mut s.e, GECKO));
        assert_eq!(s.forced_moves(GECKO), None);
        // No mover (or not here): nothing asked of the game.
        s.drop_mover(GECKO);
        assert!(!force_moves(&mut s.e, GECKO, 0x101));
        assert!(!free_moves(&mut s.e, GECKO));
        assert_eq!(s.forced_moves(GECKO), None);
        assert!(!force_moves(&mut s.e, 0x0010_9999, 0x101));
    }

    #[test]
    fn stopping_a_copy_ends_its_combat() {
        let mut s = story();
        s.set_combat(RAIDER, AVATAR);
        assert!(stop(&mut s.e, RAIDER));
        assert!(!s.actor_view(RAIDER).in_combat);
        assert!(!stop(&mut s.e, 0x0010_9999));
    }
}
