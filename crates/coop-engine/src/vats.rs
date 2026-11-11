//! V.A.T.S. with several players, in the game (docs/coop/SPECS.md S20).
//!
//! Alone, V.A.T.S. stops the game while a target is picked and slows it while the queued shots
//! play. With other players in the session neither may happen: every game runs actors the
//! others see, so the world's clock is everyone's (coop-core's `combat::vats_scales`).
//!
//! - What stops the game while a target is picked is menu mode: the V.A.T.S. menu is a menu
//!   like any other, and the idle update does not move the process lists, the scene or the
//!   calendar while `Interface::IsInMenuMode` (`00702360`) says a menu is up. Co-op wraps it as
//!   [`Owner::Coop`]: with a shared world it answers "no menu" while the V.A.T.S. menu is the
//!   top menu, so the game runs on behind the menu.
//! - What slows the game while the shots play is `BSTimer::SetGlobalTimeMultiplier`
//!   (`00aa4db0`), which the V.A.T.S. code calls with less than 1. Co-op wraps it: with a shared
//!   world a multiplier asked for while V.A.T.S. is on is raised to the world's speed.
//!
//! The queued shots themselves need nothing here: the game plays them as attacks, each lands
//! through `Actor::HitMe`, and the hit wrap ([`crate::fight`]) sends a guest's to the host.
//! With the world's speed at 0 ([`set_world`], the state of a game with no session or no other
//! player) both wraps pass every call on unchanged.

use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::addr;

/// Co-op's place in front of `Interface::IsInMenuMode`.
pub(crate) const IN_MENU_MODE: WrapSlot =
    WrapSlot::new(Owner::Coop, addr::INTERFACE_IS_IN_MENU_MODE);

/// Co-op's place in front of `BSTimer::SetGlobalTimeMultiplier`.
pub(crate) const TIME_MULTIPLIER: WrapSlot =
    WrapSlot::new(Owner::Coop, addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER);

/// The wraps' game-wide state (a lock-free shared slot, as the fights').
#[derive(Debug, Default)]
pub struct Vats {
    /// The bits of the speed the shared world runs at whatever V.A.T.S. asks for; 0.0 leaves
    /// V.A.T.S. as the game has it.
    world: AtomicU32,
    /// Whether the V.A.T.S. menu no longer stops the game (1) while the world is shared.
    unpause: AtomicU8,
    /// Bit 0: the menu-mode wrap is installed; bit 1: the multiplier's.
    wrapped: AtomicU8,
    /// Menu-mode questions answered "no menu" because the V.A.T.S. menu was up.
    unpaused: AtomicU64,
    /// Multipliers raised to the world's speed.
    unslowed: AtomicU64,
}

/// Installs the two wraps where the game has the functions (the translations, or a test's
/// stand-ins); called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    let mut wrapped = 0;
    if e.is_registered(addr::INTERFACE_IS_IN_MENU_MODE) {
        e.wrap(IN_MENU_MODE, in_menu_mode);
        wrapped |= 1;
    }
    if e.is_registered(addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER) {
        e.wrap(TIME_MULTIPLIER, time_multiplier);
        wrapped |= 2;
    }
    e.ext_free::<Vats>()
        .wrapped
        .store(wrapped, Ordering::SeqCst);
}

/// Whether this game's V.A.T.S. can be kept from stopping the game (the menu-mode wrap) and
/// from slowing it (the multiplier's).
pub fn wrapped(e: &mut Engine) -> (bool, bool) {
    let w = e.ext_free::<Vats>().wrapped.load(Ordering::SeqCst);
    (w & 1 != 0, w & 2 != 0)
}

/// Sets the speed the world runs at while this game's player is in V.A.T.S.: coop-core's
/// `vats_scales(shared, wanted).world`, which is 1.0 in a session with another player. 0.0 (or
/// anything not above 0) gives V.A.T.S. back to the game. `unpause` also keeps the V.A.T.S.
/// menu from stopping the game.
pub fn set_world(e: &mut Engine, speed: f32, unpause: bool) {
    let speed = if speed > 0.0 { speed } else { 0.0 };
    let v = e.ext_free::<Vats>();
    v.world.store(speed.to_bits(), Ordering::SeqCst);
    v.unpause
        .store(u8::from(unpause && speed > 0.0), Ordering::SeqCst);
}

/// The V.A.T.S. mode of this game: 0 none, 1 target select, 2 menu, 3 scan, 4 playback.
pub fn mode(e: &mut Engine) -> u32 {
    e.mem.u32(addr::VATS + addr::VATS_MODE_OFFSET)
}

/// How often the V.A.T.S. menu was kept from stopping the game, and how often a multiplier was
/// raised.
pub fn counts(e: &mut Engine) -> (u64, u64) {
    let v = e.ext_free::<Vats>();
    (
        v.unpaused.load(Ordering::SeqCst),
        v.unslowed.load(Ordering::SeqCst),
    )
}

/// The wrap at `00702360`. With a shared world, while V.A.T.S. is picking targets and its menu
/// is the top menu, the answer is "no menu": the game's own answer when none is up.
pub(crate) fn in_menu_mode(e: &mut Engine, a: &[u32]) -> Ret {
    if e.ext_free::<Vats>().unpause.load(Ordering::Relaxed) != 0
        && !matches!(mode(e), 0 | addr::VATS_MODE_PLAYBACK)
        && e.is_registered(addr::INTERFACE_IS_TOP_MENU_ID)
        && e.call(
            addr::INTERFACE_IS_TOP_MENU_ID,
            &args![addr::VATS_MENU_CLASS],
        )
        .bool()
    {
        e.ext_free::<Vats>()
            .unpaused
            .fetch_add(1, Ordering::Relaxed);
        return Ret::default();
    }
    e.call_next(addr::INTERFACE_IS_IN_MENU_MODE, a)
}

/// The wrap at `00aa4db0`. Its words are the call's: the timer, the multiplier's bits and the
/// "immediate" byte. With a shared world a multiplier below the world's speed that is asked for
/// while V.A.T.S. is on goes to the game as the world's speed.
pub(crate) fn time_multiplier(e: &mut Engine, a: &[u32]) -> Ret {
    let world = f32::from_bits(e.ext_free::<Vats>().world.load(Ordering::Relaxed));
    let asked = a.get(1).map(|b| f32::from_bits(*b));
    if world > 0.0 && asked.is_some_and(|m| m < world) && mode(e) != 0 {
        e.ext_free::<Vats>()
            .unslowed
            .fetch_add(1, Ordering::Relaxed);
        let mut words = a.to_vec();
        words[1] = world.to_bits();
        return e.call_next(addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER, &words);
    }
    e.call_next(addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    fn story() -> Story {
        Story::new(&StorySpec::default())
    }

    #[test]
    fn with_no_shared_world_vats_stops_and_slows_the_game_as_before() {
        let mut s = story();
        assert_eq!(wrapped(&mut s.e), (true, true));
        s.open_menu(addr::VATS_MENU_CLASS);
        s.set_vats_mode(2);
        assert!(s.in_menu_mode());
        s.close_menu();
        s.set_vats_mode(4);
        s.ask_time_multiplier(0.25);
        assert_eq!(s.time_multiplier(), 0.25);
        s.set_vats_mode(0);
        s.ask_time_multiplier(1.0);
        assert_eq!((mode(&mut s.e), s.time_multiplier()), (0, 1.0));
        assert_eq!(counts(&mut s.e), (0, 0));
    }

    #[test]
    fn with_a_shared_world_the_vats_menu_does_not_stop_the_game() {
        let mut s = story();
        set_world(&mut s.e, 1.0, true);
        // No menu, and any other menu: the game's own answer.
        assert!(!s.in_menu_mode());
        s.open_menu(0x3ea);
        assert!(s.in_menu_mode());
        // The V.A.T.S. menu while a target is picked: the game runs on.
        s.open_menu(addr::VATS_MENU_CLASS);
        for mode in [1, 2, 3] {
            s.set_vats_mode(mode);
            assert!(!s.in_menu_mode(), "mode {mode}");
        }
        // Another menu over it (the pause menu) stops the game as always; so does the V.A.T.S.
        // menu when V.A.T.S. is not picking.
        s.open_menu(0x3f5);
        assert!(s.in_menu_mode());
        s.open_menu(addr::VATS_MENU_CLASS);
        s.set_vats_mode(0);
        assert!(s.in_menu_mode());
        assert_eq!(counts(&mut s.e).0, 3);
        // The world given back, or only its speed shared: a menu again.
        s.set_vats_mode(2);
        set_world(&mut s.e, 1.0, false);
        assert!(s.in_menu_mode());
        set_world(&mut s.e, 0.0, true);
        assert!(s.in_menu_mode());
    }

    #[test]
    fn with_a_shared_world_vats_does_not_slow_the_game() {
        let mut s = story();
        set_world(&mut s.e, 1.0, true);
        s.set_vats_mode(4);
        s.ask_time_multiplier(0.25);
        assert_eq!(s.time_multiplier(), 1.0);
        s.ask_time_multiplier(0.125);
        // A multiplier at or above the world's speed is the game's.
        s.ask_time_multiplier(2.0);
        assert_eq!(s.time_multiplier(), 2.0);
        // Outside V.A.T.S. the game's own slow motion is not co-op's to change.
        s.set_vats_mode(0);
        s.ask_time_multiplier(0.5);
        assert_eq!(s.time_multiplier(), 0.5);
        assert_eq!(counts(&mut s.e), (0, 2));
        assert_eq!(s.time_multiplier_calls(), 4);
    }
}
