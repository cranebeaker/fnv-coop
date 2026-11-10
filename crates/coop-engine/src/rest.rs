//! Sleeping and waiting (docs/coop/SPECS.md S17): in a party, time passes for everyone or for no
//! one.
//!
//! The rest button of the game's sleep and wait menu goes through `SleepWaitMenu::DoClick`
//! (`007c0220`, button 4). Co-op wraps it as [`Owner::Coop`]. While a session shares the rest
//! ([`set_sharing`]) the button does not start the rest: the hours asked for and whether the
//! player is in a bed are noted ([`take_asked`]) and the click goes on as the game's own cancel
//! (button 5), which closes the menu. Once the party agreed, each game passes the hours itself
//! with [`pass`], hour by hour through the game's own hour step, as the menu does.
//!
//! With sharing off the wrap passes every click on untouched.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{addr, live};

/// Co-op's place in front of `SleepWaitMenu::DoClick`.
pub(crate) const DO_CLICK: WrapSlot = WrapSlot::new(Owner::Coop, addr::SLEEP_WAIT_DO_CLICK);

/// The most hours one rest passes (the menu's slider ends at 24).
pub const MAX_HOURS: u32 = 24;

/// A rest the player asked for in the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asked {
    /// The hours, 1 to [`MAX_HOURS`].
    pub hours: u32,
    /// Sleeping in a bed (else waiting).
    pub in_bed: bool,
}

/// The game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Rest {
    sharing: AtomicBool,
    asked: Mutex<Vec<Asked>>,
}

/// Starts or stops putting rests to the party: with it off the wrap changes nothing.
pub fn set_sharing(e: &mut Engine, on: bool) {
    e.ext_free::<Rest>().sharing.store(on, Ordering::SeqCst);
}

/// The rests asked for since the last call, in order.
pub fn take_asked(e: &mut Engine) -> Vec<Asked> {
    let r = e.ext_free::<Rest>();
    std::mem::take(&mut *r.asked.lock().unwrap_or_else(PoisonError::into_inner))
}

/// Passes `hours` hours in this game as the menu's rest does: the player's wait is set, then for
/// each hour the scripts run once and the game's own hour step moves the clock and the calendar
/// and restores the player; then the wait is cleared. False with no player.
pub fn pass(e: &mut Engine, hours: u32, in_bed: bool) -> bool {
    let Some(player) = live::player(e) else {
        return false;
    };
    let hours = hours.clamp(1, MAX_HOURS);
    e.call(
        addr::PLAYER_SET_WAIT,
        &args![player, hours, u32::from(in_bed)],
    );
    for _ in 0..hours {
        let tes = e.mem.u32(addr::TES_SINGLETON);
        if tes != 0 {
            e.call(addr::TES_RUN_SCRIPTS, &args![tes]);
        }
        e.call(addr::PLAYER_WAIT_HOUR, &args![player]);
    }
    e.call(addr::PLAYER_SET_WAIT, &args![player, 0u32, 0u32]);
    true
}

/// The wrap at `007c0220`. Its words: the menu, the button id, the tile.
pub(crate) fn do_click(e: &mut Engine, a: &[u32]) -> Ret {
    let (menu, id) = (
        a.first().copied().unwrap_or(0),
        a.get(1).copied().unwrap_or(0),
    );
    let r = e.ext_free::<Rest>();
    if !r.sharing.load(Ordering::SeqCst) || id != addr::SLEEP_WAIT_REST_BUTTON || menu == 0 {
        return e.call_next(addr::SLEEP_WAIT_DO_CLICK, a);
    }
    // The hours as the button reads them: the slider's value, truncated, plus one.
    let tile = e.mem.u32(menu + addr::SLEEP_WAIT_HOURS_TILE_OFFSET);
    let id_of_value = e.mem.u32(addr::SLEEP_WAIT_VALUE_TRAIT);
    let value = e
        .call(addr::TILE_GET_FLOAT, &args![tile, id_of_value])
        .f64();
    let hours = if value.is_finite() && value > 0.0 {
        (value as u32).saturating_add(1)
    } else {
        1
    };
    let asked = Asked {
        hours: hours.clamp(1, MAX_HOURS),
        in_bed: e.mem.u8(menu + addr::SLEEP_WAIT_IN_BED_OFFSET) != 0,
    };
    let r = e.ext_free::<Rest>();
    r.asked
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(asked);
    let mut cancel = a.to_vec();
    cancel[1] = addr::SLEEP_WAIT_CANCEL_BUTTON;
    e.call_next(addr::SLEEP_WAIT_DO_CLICK, &cancel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forms;
    use crate::testkit::{Story, StorySpec};

    const HOUR: u32 = 0x38;
    const CELL: u32 = 0x0010_5a00;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            globals: vec![(HOUR, 8.0)],
            cells: vec![CELL],
            ..StorySpec::default()
        });
        s.set_hour_global(HOUR);
        s
    }

    #[test]
    fn with_sharing_off_the_rest_button_is_the_games_own() {
        let mut s = story();
        s.click_rest(3, true);
        assert_eq!(s.rest_clicks(), (1, 0));
        assert_eq!(s.wait_left(), (3, true));
        assert!(take_asked(&mut s.e).is_empty());
    }

    #[test]
    fn a_shared_rest_is_asked_for_and_the_click_becomes_the_cancel() {
        let mut s = story();
        set_sharing(&mut s.e, true);
        s.click_rest(3, true);
        s.click_rest(40, false);
        assert_eq!(
            s.rest_clicks(),
            (0, 2),
            "the rest never started; the menu closed"
        );
        assert_eq!(s.wait_left(), (0, false));
        assert_eq!(
            take_asked(&mut s.e),
            [
                Asked {
                    hours: 3,
                    in_bed: true
                },
                Asked {
                    hours: MAX_HOURS,
                    in_bed: false
                }
            ]
        );
        // The cancel button itself is passed on as it is.
        s.click_cancel();
        assert_eq!(s.rest_clicks(), (0, 3));
        assert!(take_asked(&mut s.e).is_empty());
    }

    #[test]
    fn the_hours_pass_through_the_games_own_hour_step() {
        let mut s = story();
        assert!(pass(&mut s.e, 5, false));
        assert_eq!(forms::global_value(&mut s.e, HOUR), Some(13.0));
        assert_eq!(
            s.hours_passed(),
            (5, 5),
            "five hour steps, the scripts run before each"
        );
        assert_eq!(s.wait_left(), (0, false));
        assert!(pass(&mut s.e, 0, true), "at least an hour");
        assert_eq!(forms::global_value(&mut s.e, HOUR), Some(14.0));
        assert!(pass(&mut s.e, 99, true), "at most a day");
        assert_eq!(forms::global_value(&mut s.e, HOUR), Some(14.0));
        assert_eq!(s.hours_passed().0, 30);
    }
}
