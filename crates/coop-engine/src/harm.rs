//! Damage that does not pass `Actor::HitMe`, in the game (docs/coop/SPECS.md S21): a fall, a
//! trap, drowning and dangerous water (the damage applier, called straight) and an effect's
//! damage over time (the value change, called straight).
//!
//! Every loss of health, whatever made it, ends in one function: the one behind `Actor`
//! virtual slot `0x3ac` (`00881130`), with the health value and a negative amount. Co-op wraps
//! it as [`Owner::Coop`] and asks the fights the same question the hit wrap asks
//! ([`fight::verdict`]): a loss on an actor the host runs is not a guest's game's to apply, a
//! loss on a player's avatar belongs to that player's own game, and what the host's actors and
//! the other players do to a guest's player comes from the host. A loss it keeps from the game
//! and that this game's player (or, on the host, anyone) caused is added up per victim and
//! source for the session ([`take`]), which sends it the way a kept hit goes.
//!
//! A hit that `Actor::HitMe` applies also ends here, but only after the hit wrap let it pass,
//! and the wrap decides the same for both. What co-op itself writes (a held actor's health, the
//! damage the host decided) is made inside [`own`] and passes. With no session
//! ([`fight::Role::Off`]) every call goes to the game as before.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{addr, fight};

/// Co-op's place in front of the actor's value change.
pub(crate) const MODIFY_VALUE: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_MODIFY_VALUE);

/// Most pairs of victim and source whose losses are added up between two [`take`]s.
const PAIRS_MAX: usize = 256;

/// Health an actor or an avatar lost outside `Actor::HitMe`, kept from the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Harm {
    /// The form id of the actor that lost health.
    pub target: u32,
    /// The form id of the actor the game names as the source, 0 for none (a fall).
    pub source: u32,
    /// Points of health, whole and above 0.
    pub amount: f32,
}

/// The wrap's game-wide state (a lock-free shared slot, as the fights').
#[derive(Debug, Default)]
pub struct Harms {
    /// The value change is wrapped on this game.
    wrapped: AtomicU8,
    /// Above 0 while co-op itself changes a value: the wrap passes the call on.
    own: AtomicU32,
    /// Losses kept from the game and not yet taken, by (victim, source).
    pending: Mutex<BTreeMap<(u32, u32), f32>>,
    /// Losses kept from the game, ever.
    kept: AtomicU64,
}

/// Installs the wrap when the game has the function (the translation, or a test's stand-in);
/// called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    let wrapped = e.is_registered(addr::ACTOR_MODIFY_VALUE);
    if wrapped {
        e.wrap(MODIFY_VALUE, modify_value);
    }
    e.ext_free::<Harms>()
        .wrapped
        .store(u8::from(wrapped), Ordering::SeqCst);
}

/// Whether the actor's value change is wrapped on this game.
pub fn wrapped(e: &mut Engine) -> bool {
    e.ext_free::<Harms>().wrapped.load(Ordering::SeqCst) != 0
}

/// Runs `f` as co-op's own change of an actor's values: the wrap passes it on.
pub(crate) fn own<R>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> R {
    e.ext_free::<Harms>().own.fetch_add(1, Ordering::SeqCst);
    let r = f(e);
    e.ext_free::<Harms>().own.fetch_sub(1, Ordering::SeqCst);
    r
}

/// How many losses of health the wrap kept from the game, ever.
pub fn kept(e: &mut Engine) -> u64 {
    e.ext_free::<Harms>().kept.load(Ordering::SeqCst)
}

/// The losses kept for the session since the last call, as whole points per victim and source;
/// what is less than a point stays for the next call (an effect takes a little each frame).
pub fn take(e: &mut Engine) -> Vec<Harm> {
    let h = e.ext_free::<Harms>();
    let mut pending = h.pending.lock().unwrap_or_else(PoisonError::into_inner);
    let mut out = Vec::new();
    for ((target, source), sum) in pending.iter_mut() {
        let whole = sum.floor();
        if whole >= 1.0 {
            *sum -= whole;
            out.push(Harm {
                target: *target,
                source: *source,
                amount: whole,
            });
        }
    }
    pending.retain(|_, sum| *sum > 0.0);
    out
}

/// The wrap at `00881130`. Its words are the call's: the actor, the value's number, the amount
/// and the source actor. A loss of health it keeps from the game answers as the game's own call
/// does for a value that cannot be changed: nothing happened.
pub(crate) fn modify_value(e: &mut Engine, a: &[u32]) -> Ret {
    let word = |i: usize| a.get(i).copied().unwrap_or(0);
    let (actor, amount) = (word(0), f32::from_bits(word(2)));
    let ours = actor != 0
        && word(1) == addr::ACTOR_VALUE_HEALTH
        && amount < 0.0
        && e.ext_free::<Harms>().own.load(Ordering::SeqCst) == 0;
    if !ours {
        return e.call_next(addr::ACTOR_MODIFY_VALUE, a);
    }
    let v = fight::verdict(e, actor, word(3));
    if !v.keep {
        return e.call_next(addr::ACTOR_MODIFY_VALUE, a);
    }
    let h = e.ext_free::<Harms>();
    h.kept.fetch_add(1, Ordering::SeqCst);
    if v.tell {
        let mut pending = h.pending.lock().unwrap_or_else(PoisonError::into_inner);
        let key = (v.target, v.aggressor);
        if pending.len() < PAIRS_MAX || pending.contains_key(&key) {
            *pending.entry(key).or_insert(0.0) -= amount;
        }
    }
    Ret::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fight::{set_avatars, set_held, set_role, Role};
    use crate::testkit::{Story, StorySpec};
    use crate::{actors, live};

    const HOUSE: u32 = 0x0010_5a00;
    const RAIDER: u32 = 0x0010_4f25;
    const GECKO: u32 = 0x0010_4f26;
    const PLAYER: u32 = actors::PLAYER_REF_ID;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![HOUSE],
            actors: vec![RAIDER, GECKO],
            ..StorySpec::default()
        });
        s.move_player(HOUSE, [0.0; 3], 0.0);
        s.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5);
        s.place_actor(GECKO, HOUSE, [4.0, 5.0, 6.0], 0.5);
        s
    }

    fn harm(target: u32, source: u32, amount: f32) -> Harm {
        Harm {
            target,
            source,
            amount,
        }
    }

    #[test]
    fn with_no_session_every_loss_is_the_games() {
        let mut s = story();
        assert!(wrapped(&mut s.e));
        set_held(&mut s.e, [RAIDER]);
        s.fall(RAIDER, 30.0);
        s.burn(RAIDER, PLAYER, 5.0);
        s.fall(PLAYER, 10.0);
        assert_eq!(s.actor_view(RAIDER).health, 65.0);
        assert_eq!(s.actor_view(PLAYER).health, 90.0);
        assert_eq!((kept(&mut s.e), take(&mut s.e)), (0, vec![]));
    }

    #[test]
    fn a_guest_keeps_losses_of_held_actors_and_sends_its_players() {
        let mut s = story();
        let avatar = live::avatar_create(&mut s.e, 7).unwrap().id;
        set_role(&mut s.e, Role::Guest);
        set_held(&mut s.e, [RAIDER]);
        set_avatars(&mut s.e, [avatar]);
        // A held actor falls, or burns from another actor's fire, in this game only: kept, and
        // nobody's claim (the host's game decides what its actors lose).
        s.fall(RAIDER, 30.0);
        s.burn(RAIDER, GECKO, 20.0);
        assert_eq!(s.actor_view(RAIDER).health, 100.0);
        assert_eq!((kept(&mut s.e), take(&mut s.e)), (2, vec![]));
        // This player's effect on it, a little each frame: kept, and added up to whole points.
        for _ in 0..7 {
            s.burn(RAIDER, PLAYER, 0.4);
        }
        assert_eq!(s.actor_view(RAIDER).health, 100.0);
        assert_eq!(take(&mut s.e), vec![harm(RAIDER, PLAYER, 2.0)]);
        assert_eq!(take(&mut s.e), vec![]);
        s.burn(RAIDER, PLAYER, 0.4);
        assert_eq!(take(&mut s.e), vec![harm(RAIDER, PLAYER, 1.0)]);
        // What a held actor's or an avatar's effect does to this player comes from the host;
        // this player's effect on an avatar is the host's to judge.
        s.burn(PLAYER, RAIDER, 10.0);
        s.burn(PLAYER, avatar, 10.0);
        s.burn(avatar, PLAYER, 15.0);
        assert_eq!(s.actor_view(PLAYER).health, 100.0);
        assert_eq!(take(&mut s.e), vec![harm(avatar, PLAYER, 15.0)]);
        // The player's own fall, an actor this game runs, and a gain of health: the game's.
        s.fall(PLAYER, 12.0);
        s.fall(GECKO, 40.0);
        s.burn(GECKO, PLAYER, 5.0);
        s.burn(RAIDER, 0, -8.0);
        assert_eq!(s.actor_view(PLAYER).health, 88.0);
        assert_eq!(s.actor_view(GECKO).health, 55.0);
        assert_eq!(s.actor_view(RAIDER).health, 108.0);
        assert_eq!(take(&mut s.e), vec![]);
    }

    #[test]
    fn a_host_keeps_only_losses_of_avatars() {
        let mut s = story();
        let avatar = live::avatar_create(&mut s.e, 7).unwrap().id;
        set_role(&mut s.e, Role::Host);
        set_avatars(&mut s.e, [avatar]);
        s.burn(avatar, RAIDER, 25.0);
        s.fall(avatar, 3.0);
        assert_eq!(
            take(&mut s.e),
            vec![harm(avatar, 0, 3.0), harm(avatar, RAIDER, 25.0)]
        );
        s.fall(RAIDER, 30.0);
        s.burn(GECKO, PLAYER, 10.0);
        s.fall(PLAYER, 10.0);
        assert_eq!(s.actor_view(RAIDER).health, 70.0);
        assert_eq!(s.actor_view(GECKO).health, 90.0);
        assert_eq!(s.actor_view(PLAYER).health, 90.0);
        assert_eq!(take(&mut s.e), vec![]);
    }

    #[test]
    fn what_co_op_itself_writes_passes_the_wrap() {
        let mut s = story();
        set_role(&mut s.e, Role::Guest);
        set_held(&mut s.e, [RAIDER]);
        // A held actor's health goes down to the host's, and the host's damage reaches this
        // player though a held actor is named as the attacker.
        fight::hold(&mut s.e, RAIDER, [1.0, 2.0, 3.0], 0.5, 62.0).unwrap();
        assert_eq!(s.actor_view(RAIDER).health, 62.0);
        assert_eq!(fight::hurt_player(&mut s.e, 12.0, RAIDER), Some(88.0));
        assert_eq!((kept(&mut s.e), take(&mut s.e)), (0, vec![]));
        // On the host a guest's damage reaches the actor.
        set_role(&mut s.e, Role::Host);
        assert_eq!(fight::hurt_actor(&mut s.e, GECKO, 30.0, 0), Some(70.0));
    }
}
