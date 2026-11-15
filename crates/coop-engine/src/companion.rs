//! Companions in a party (docs/coop/SPECS.md S33): a companion follows the player who hired it.
//!
//! Hiring and firing are the game's own: a line's result or a script calls `SetPlayerTeammate`
//! (`Actor::SetPlayerTeammate`, `008bca90`). Co-op notes each change of an actor's teammate flag
//! ([`take_hires`]) with the player whose line the host's game was running then
//! ([`with_hirer`]), so the session can tell who hired it.
//!
//! A follow package's target is "the player", which in each game is that game's own player. For a
//! companion another player keeps, this game's copy is to follow that player's avatar instead.
//! The package is left as it is (other actors may run it); what changes is the process's target
//! (`pTarget`, +0x40), the reference every `ProcessFollow` follows, for that one actor: the three
//! `ProcessFollow` functions (`HighProcess`, `MiddleHighProcess`, `LowProcess`) are wrapped, and
//! for a companion with a keeper here whose target is this game's player the target is set to the
//! keeper's avatar first (the process's own `SetTarget`). The game puts the player back on the
//! next `ResetTarget`; the next follow step aims it again. With no session ([`set_on`]) nothing
//! here changes anything.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, commands, fight, talk};

pub(crate) const TEAMMATE: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_SET_PLAYER_TEAMMATE);
pub(crate) const HIGH_FOLLOW: WrapSlot = WrapSlot::new(Owner::Coop, addr::HIGH_PROCESS_FOLLOW);
pub(crate) const MIDDLE_FOLLOW: WrapSlot =
    WrapSlot::new(Owner::Coop, addr::MIDDLE_HIGH_PROCESS_FOLLOW);
pub(crate) const LOW_FOLLOW: WrapSlot = WrapSlot::new(Owner::Coop, addr::LOW_PROCESS_FOLLOW);

/// A change of an actor's teammate flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hire {
    /// The actor's reference id.
    pub actor: u32,
    /// Hired (true) or let go.
    pub on: bool,
    /// The player slot whose line the host's game was running ([`with_hirer`]), if any.
    pub by: Option<u8>,
    /// This game's own conversation with the actor was open (its dialogue menu's speaker).
    pub own_menu: bool,
}

/// The game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Companions {
    on: AtomicBool,
    /// Companions this game's copy follows another player's avatar for: actor id to the avatar's
    /// reference id.
    keepers: RwLock<BTreeMap<u32, u32>>,
    /// Every companion a player of the session keeps: actor id to the keeper's slot.
    slots: RwLock<BTreeMap<u32, u8>>,
    /// Companions this game's player keeps, with their switches as last read (party saves).
    mine: RwLock<BTreeSet<u32>>,
    /// Companions a party save gave back to this player, with their switches, for the session to
    /// claim.
    restored: Mutex<Vec<(u32, [f32; 4])>>,
    /// The player slot whose line runs now, plus one; 0 for none.
    hirer: AtomicU32,
    hires: Mutex<Vec<Hire>>,
    /// How deep co-op is in its own teammate calls (not noted).
    own: AtomicU32,
    aimed: AtomicU64,
}

fn state(e: &mut Engine) -> &Companions {
    e.ext_free::<Companions>()
}

/// Installs the wraps whose functions this program has.
pub(crate) fn install(e: &mut Engine) {
    // The slot is made here, before the threads start.
    let _ = e.ext_free::<Companions>();
    if e.is_registered(addr::ACTOR_SET_PLAYER_TEAMMATE) {
        e.wrap(TEAMMATE, teammate_wrap);
    }
    if e.is_registered(addr::HIGH_PROCESS_FOLLOW) {
        e.wrap(HIGH_FOLLOW, |e, a| {
            follow_wrap(e, a, addr::HIGH_PROCESS_FOLLOW)
        });
    }
    if e.is_registered(addr::LOW_PROCESS_FOLLOW) {
        e.wrap(LOW_FOLLOW, |e, a| {
            follow_wrap(e, a, addr::LOW_PROCESS_FOLLOW)
        });
    }
    if e.is_registered(addr::MIDDLE_HIGH_PROCESS_FOLLOW) {
        e.wrap(MIDDLE_FOLLOW, |e, a| {
            follow_wrap(e, a, addr::MIDDLE_HIGH_PROCESS_FOLLOW)
        });
    }
}

/// Turns the companions' part on or off (on while a session runs; off, the wraps pass every call
/// on unchanged).
pub fn set_on(e: &mut Engine, on: bool) {
    state(e).on.store(on, Ordering::SeqCst);
}

/// Whether the part is on.
pub fn on(e: &mut Engine) -> bool {
    state(e).on.load(Ordering::SeqCst)
}

/// Has this game's copy of companion `actor` follow the avatar with reference id `avatar`
/// (`None`: this game's own player, as the game has it).
pub fn set_keeper(e: &mut Engine, actor: u32, avatar: Option<u32>) {
    let mut k = state(e)
        .keepers
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    match avatar {
        Some(a) if a != 0 => {
            k.insert(actor, a);
        }
        _ => {
            k.remove(&actor);
        }
    }
}

/// The avatar this game's copy of `actor` follows, if another player keeps it.
pub fn keeper(e: &mut Engine, actor: u32) -> Option<u32> {
    state(e)
        .keepers
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&actor)
        .copied()
}

/// Says which player (slot) keeps companion `actor` (`None`: nobody).
pub fn set_kept_by(e: &mut Engine, actor: u32, slot: Option<u8>) {
    let mut s = state(e)
        .slots
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    match slot {
        Some(p) => {
            s.insert(actor, p);
        }
        None => {
            s.remove(&actor);
        }
    }
}

/// The slot of the player who keeps companion `actor`, if any.
pub fn kept_by(e: &mut Engine, actor: u32) -> Option<u8> {
    state(e)
        .slots
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&actor)
        .copied()
}

/// Names the companions this game's player keeps.
pub fn set_mine(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    *state(e)
        .mine
        .write()
        .unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

/// The companions this game's player keeps.
pub fn mine(e: &mut Engine) -> Vec<u32> {
    state(e)
        .mine
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
        .copied()
        .collect()
}

/// Whether any player keeps `actor` as a companion in this session.
pub fn kept(e: &mut Engine, actor: u32) -> bool {
    keeper(e, actor).is_some()
        || kept_by(e, actor).is_some()
        || state(e)
            .mine
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .contains(&actor)
}

/// A party save gave these companions back to this player (with their switches).
pub fn restore_kept(e: &mut Engine, kept: &[(u32, [f32; 4])]) {
    *state(e)
        .restored
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = kept.to_vec();
}

/// Takes what [`restore_kept`] left.
pub fn take_restored(e: &mut Engine) -> Option<Vec<(u32, [f32; 4])>> {
    let mut r = state(e)
        .restored
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    (!r.is_empty()).then(|| std::mem::take(&mut *r))
}

/// Runs `f` with the host's game running a line of player `slot` (a guest's line enacted here):
/// a teammate change in it was that player's doing.
pub fn with_hirer<R>(e: &mut Engine, slot: u8, f: impl FnOnce(&mut Engine) -> R) -> R {
    let before = state(e).hirer.swap(u32::from(slot) + 1, Ordering::SeqCst);
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e)));
    state(e).hirer.store(before, Ordering::SeqCst);
    match out {
        Ok(r) => r,
        Err(p) => std::panic::resume_unwind(p),
    }
}

/// The teammate changes since the last call, in order.
pub fn take_hires(e: &mut Engine) -> Vec<Hire> {
    std::mem::take(
        &mut *state(e)
            .hires
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    )
}

/// How many follow steps were aimed at another player's avatar.
pub fn aimed(e: &mut Engine) -> u64 {
    state(e).aimed.load(Ordering::SeqCst)
}

/// Whether actor `id` is a teammate of this game's player; `None` when it is not here.
pub fn teammate(e: &mut Engine, id: u32) -> Option<bool> {
    let a = actors::actor(e, id)?;
    Some(e.mem.u8(a.wrapping_add(addr::ACTOR_TEAMMATE_OFFSET)) != 0)
}

/// Makes actor `id` this game's player's teammate or not, through the game's own
/// `SetPlayerTeammate` (not noted as a hire). False when it is not here or the program lacks it.
pub fn set_teammate(e: &mut Engine, id: u32, on: bool) -> bool {
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    if !e.is_registered(addr::ACTOR_SET_PLAYER_TEAMMATE) {
        return false;
    }
    state(e).own.fetch_add(1, Ordering::SeqCst);
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.call(addr::ACTOR_SET_PLAYER_TEAMMATE, &args![a, u32::from(on)]);
    }));
    state(e).own.fetch_sub(1, Ordering::SeqCst);
    if let Err(p) = out {
        std::panic::resume_unwind(p);
    }
    true
}

/// The companion wheel's four switches of actor `id` (aggressive, keep distance, ranged,
/// waiting), as the wheel reads them; `None` when it is not here or the program lacks the getter.
pub fn switches(e: &mut Engine, id: u32) -> Option<[f32; 4]> {
    let a = actors::actor(e, id)?;
    if !e.is_registered(addr::ACTOR_GET_VARIABLE) {
        return None;
    }
    Some(
        addr::WHEEL_SWITCH_TEXTS
            .map(|name| e.call(addr::ACTOR_GET_VARIABLE, &args![a, name]).f64() as f32),
    )
}

/// Sets actor `id`'s four switches as the wheel does, those that differ. False when it is not
/// here or the program lacks the setter.
pub fn set_switches(e: &mut Engine, id: u32, want: [f32; 4]) -> bool {
    let Some(have) = switches(e, id) else {
        return false;
    };
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    if !e.is_registered(addr::ACTOR_SET_VARIABLE) {
        return false;
    }
    for i in 0..4 {
        if have[i] != want[i] {
            e.call(
                addr::ACTOR_SET_VARIABLE,
                &args![a, addr::WHEEL_SWITCH_TEXTS[i], f64::from(want[i])],
            );
        }
    }
    true
}

/// Whether companion `id` follows in this game now: its package keeps company with "the player"
/// and its wheel's "Waiting" is off. False when it is not here.
pub fn following(e: &mut Engine, id: u32) -> bool {
    let waits = switches(e, id).is_some_and(|s| s[addr::WHEEL_WAITING] != 0.0);
    let keeps =
        commands::current_package(e, id).is_some_and(|p| p != 0 && commands::keeps_company(e, p));
    keeps && !waits
}

/// The wrap at `008bca90`: the game's call runs as it is; with the part on, a change of a placed
/// actor's flag that co-op did not make itself is noted.
fn teammate_wrap(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a.first().copied().unwrap_or(0);
    let c = state(e);
    if actor == 0 || !c.on.load(Ordering::SeqCst) || c.own.load(Ordering::SeqCst) > 0 {
        return e.call_next(addr::ACTOR_SET_PLAYER_TEAMMATE, a);
    }
    let flag = a.get(1).copied().unwrap_or(0) & 0xff != 0;
    let at = actor.wrapping_add(addr::ACTOR_TEAMMATE_OFFSET);
    let before = e.mem.u8(at) != 0;
    let ret = e.call_next(addr::ACTOR_SET_PLAYER_TEAMMATE, a);
    let after = e.mem.u8(at) != 0;
    let id = e.mem.u32(actor.wrapping_add(addr::FORM_ID_OFFSET));
    if before != after && after == flag && actors::shareable(id) {
        let hirer = state(e).hirer.load(Ordering::SeqCst);
        let own_menu = talk::menu_npc(e) == Some(id);
        let hire = Hire {
            actor: id,
            on: after,
            by: hirer.checked_sub(1).map(|s| s as u8),
            own_menu,
        };
        let mut h = state(e)
            .hires
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if h.len() < 64 {
            h.push(hire);
        }
    }
    ret
}

/// The wraps of the three `ProcessFollow`s. Their words: the process, the actor, then the
/// function's own.
fn follow_wrap(e: &mut Engine, a: &[u32], at: u32) -> Ret {
    let aimed = aim(e, a);
    let ret = e.call_next(at, a);
    if aimed {
        // The high process's step makes a teammate a follower of this game's player (so it goes
        // with that player through doors); one another player keeps is not.
        let actor = a.get(1).copied().unwrap_or(0);
        let player = e.mem.u32(addr::PLAYER_SINGLETON);
        if player != 0
            && e.is_registered(addr::REF_EXTRA_LIST)
            && e.is_registered(addr::EXTRA_REMOVE_FOLLOWER)
        {
            let list = e.call(addr::REF_EXTRA_LIST, &args![player]).u32();
            if list != 0 {
                e.call(addr::EXTRA_REMOVE_FOLLOWER, &args![list, actor]);
            }
        }
    }
    ret
}

/// Sets the process's target to the keeper's avatar when the actor has one here, the avatar is
/// in this game in the actor's space, and the target is this game's player (or none). True when
/// it did.
fn aim(e: &mut Engine, a: &[u32]) -> bool {
    let (process, actor) = (
        a.first().copied().unwrap_or(0),
        a.get(1).copied().unwrap_or(0),
    );
    if process == 0 || actor == 0 || !on(e) {
        return false;
    }
    let id = e.mem.u32(actor.wrapping_add(addr::FORM_ID_OFFSET));
    let Some(avatar_id) = keeper(e, id) else {
        return false;
    };
    let Some(avatar) = crate::forms::form(e, avatar_id) else {
        return false;
    };
    if e.mem.u32(avatar.wrapping_add(addr::REF_LOADED_OFFSET)) == 0 {
        return false;
    }
    let space = fight::space_of(e, actor);
    if space == 0 || fight::space_of(e, avatar) != space {
        return false;
    }
    let player = e.mem.u32(addr::PLAYER_SINGLETON);
    let target = e.mem.u32(process.wrapping_add(addr::PROCESS_TARGET_OFFSET));
    if target == avatar {
        return true;
    }
    if target != 0 && target != player {
        return false;
    }
    e.vcall(process, addr::PROCESS_SET_TARGET_SLOT, &args![avatar]);
    state(e).aimed.fetch_add(1, Ordering::SeqCst);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const CELL: u32 = 0x0003_0000;
    const CASS: u32 = 0x0004_0001;
    const BOONE: u32 = 0x0004_0002;
    const FOLLOW: u32 = 0x0005_0001;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            actors: vec![CASS, BOONE],
            packages: vec![FOLLOW],
            ..StorySpec::default()
        });
        s.move_player(CELL, [0.0, 0.0, 0.0], 0.0);
        for id in [CASS, BOONE] {
            s.place_actor(id, CELL, [100.0, 0.0, 0.0], 0.0);
        }
        s
    }

    #[test]
    fn hires_are_noted_only_while_on_and_with_the_hirer() {
        let mut s = story();
        s.script_set_teammate(CASS, true);
        assert!(take_hires(&mut s.e).is_empty(), "off: nothing noted");
        assert_eq!(
            teammate(&mut s.e, CASS),
            Some(true),
            "the game's own call ran"
        );
        set_on(&mut s.e, true);
        s.script_set_teammate(CASS, false);
        with_hirer(&mut s.e, 2, |e| {
            let a = actors::actor(e, BOONE).unwrap();
            e.call(addr::ACTOR_SET_PLAYER_TEAMMATE, &args![a, 1u32]);
        });
        // The same flag again changes nothing and is not noted.
        s.script_set_teammate(BOONE, true);
        assert_eq!(
            take_hires(&mut s.e),
            vec![
                Hire {
                    actor: CASS,
                    on: false,
                    by: None,
                    own_menu: false
                },
                Hire {
                    actor: BOONE,
                    on: true,
                    by: Some(2),
                    own_menu: false
                },
            ]
        );
        // Co-op's own call is not a hire.
        assert!(set_teammate(&mut s.e, CASS, true));
        assert_eq!(teammate(&mut s.e, CASS), Some(true));
        assert!(take_hires(&mut s.e).is_empty());
    }

    #[test]
    fn a_kept_companion_follows_the_keepers_avatar_and_no_other_does() {
        let mut s = story();
        let avatar = s.place_avatar_at(CELL, [500.0, 0.0, 0.0]);
        set_on(&mut s.e, true);
        set_keeper(&mut s.e, CASS, Some(avatar));
        let player = s.e.mem.u32(addr::PLAYER_SINGLETON);
        for id in [CASS, BOONE] {
            s.set_follow_target(id, player);
        }
        for f in [
            addr::HIGH_PROCESS_FOLLOW,
            addr::MIDDLE_HIGH_PROCESS_FOLLOW,
            addr::LOW_PROCESS_FOLLOW,
        ] {
            for id in [CASS, BOONE] {
                s.set_follow_target(id, player);
                s.process_follow(f, id);
            }
            let av = crate::forms::form(&mut s.e, avatar).unwrap();
            assert_eq!(s.followed(CASS), av, "the keeper's avatar ({f:08x})");
            assert_eq!(s.followed(BOONE), player, "this game's player ({f:08x})");
        }
        assert_eq!(aimed(&mut s.e), 3);
        // Off: as the game has it.
        set_on(&mut s.e, false);
        s.set_follow_target(CASS, player);
        s.process_follow(addr::HIGH_PROCESS_FOLLOW, CASS);
        assert_eq!(s.followed(CASS), player);
    }

    #[test]
    fn an_avatar_in_another_space_or_another_target_is_not_aimed_at() {
        let mut s = story();
        let avatar = s.place_avatar_at(CELL, [500.0, 0.0, 0.0]);
        set_on(&mut s.e, true);
        set_keeper(&mut s.e, CASS, Some(avatar));
        // A script has her follow someone else: left alone.
        let boone = actors::actor(&mut s.e, BOONE).unwrap();
        s.set_follow_target(CASS, boone);
        s.process_follow(addr::HIGH_PROCESS_FOLLOW, CASS);
        assert_eq!(s.followed(CASS), boone);
        // The avatar not loaded: the player, as the game has it.
        let player = s.e.mem.u32(addr::PLAYER_SINGLETON);
        let av = crate::forms::form(&mut s.e, avatar).unwrap();
        s.e.mem.set_u32(av + addr::REF_LOADED_OFFSET, 0);
        s.set_follow_target(CASS, player);
        s.process_follow(addr::HIGH_PROCESS_FOLLOW, CASS);
        assert_eq!(s.followed(CASS), player);
        assert_eq!(aimed(&mut s.e), 0);
    }

    #[test]
    fn switches_are_read_and_set_as_the_wheel_does() {
        let mut s = story();
        assert_eq!(switches(&mut s.e, CASS), Some([0.0; 4]));
        assert!(set_switches(&mut s.e, CASS, [1.0, 0.0, 1.0, 1.0]));
        assert_eq!(switches(&mut s.e, CASS), Some([1.0, 0.0, 1.0, 1.0]));
        assert_eq!(switches(&mut s.e, BOONE), Some([0.0; 4]));
        // Waiting: not following, whatever its package.
        let player = s.e.mem.u32(addr::PLAYER_SINGLETON);
        s.aim_package_at(FOLLOW, player);
        s.set_package_type(FOLLOW, 1);
        s.set_own_package(CASS, FOLLOW);
        s.set_own_package(BOONE, FOLLOW);
        assert!(!following(&mut s.e, CASS));
        assert!(following(&mut s.e, BOONE));
    }
}
