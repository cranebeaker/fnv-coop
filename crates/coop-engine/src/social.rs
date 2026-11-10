//! Installs co-op's part in conversations, saves and rests (docs/coop/SPECS.md S14 to S17): the
//! wraps of [`crate::talk`] and [`crate::rest`] and the seam handlers of [`crate::saves`].
//!
//! Each wrap goes in front of a translated function. An address with nothing registered (a test
//! engine that holds only what its test needs) is left out and named by [`missing`]; in the game
//! every one of them is a translation, and the viewer prints what [`missing`] names.

use std::sync::{Mutex, PoisonError};

use engine_core::extend::WrapSlot;
use engine_core::prelude::*;

use crate::{addr, rest, saves, talk};

/// What [`install`] could not wrap.
#[derive(Debug, Default)]
pub struct Social {
    missing: Mutex<Vec<u32>>,
}

/// The wraps and seam handlers; called once by [`crate::install`], before the game's threads
/// start.
pub(crate) fn install(e: &mut Engine) {
    let wraps: [(u32, WrapSlot, AbiFn); 3] = [
        (addr::INFO_RUN_RESULT, talk::RUN_RESULT, talk::run_result),
        (addr::DIALOG_MENU_CREATE, talk::CREATE, talk::create),
        (addr::SLEEP_WAIT_DO_CLICK, rest::DO_CLICK, rest::do_click),
    ];
    let mut missing = Vec::new();
    for (at, slot, f) in wraps {
        if e.is_registered(at) {
            e.wrap(slot, f);
        } else {
            missing.push(at);
        }
    }
    e.on_site(saves::SAVE, saves::save_site);
    e.on_site(saves::LOAD, saves::load_site);
    // The slots, before the threads start.
    let _ = e.ext_free::<talk::Talk>();
    let _ = e.ext_free::<rest::Rest>();
    let _ = e.ext_free::<saves::Saves>();
    *e.ext_free::<Social>()
        .missing
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = missing;
}

/// The addresses [`install`] found nothing at, so that part of co-op is off in this game.
pub fn missing(e: &mut Engine) -> Vec<u32> {
    e.ext_free::<Social>()
        .missing
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};
    use engine_core::extend::Owner;

    #[test]
    fn the_story_kit_has_every_wrap_and_both_seams() {
        let mut s = Story::new(&StorySpec::default());
        assert!(missing(&mut s.e).is_empty());
        for at in [
            addr::INFO_RUN_RESULT,
            addr::DIALOG_MENU_CREATE,
            addr::SLEEP_WAIT_DO_CLICK,
        ] {
            assert_eq!(s.e.wraps_at(at), [Owner::Coop], "{at:08x}");
        }
        for at in [addr::SAVE_GAME_SITE, addr::LOAD_GAME_SITE] {
            assert_eq!(s.e.site_owners(at), [Owner::Coop], "{at:08x}");
        }
        assert!(s.e.unlisted_wraps().is_empty());
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn an_engine_without_the_functions_is_left_alone_and_says_so() {
        let mut e = Engine::new();
        e.register(addr::SAVE_LOAD_ADD_CHANGE, nothing);
        e.register(addr::SCRIPT_RUNNER_RUN, nothing);
        crate::install(&mut e);
        assert_eq!(
            missing(&mut e),
            [
                addr::INFO_RUN_RESULT,
                addr::DIALOG_MENU_CREATE,
                addr::SLEEP_WAIT_DO_CLICK
            ]
        );
    }
}
