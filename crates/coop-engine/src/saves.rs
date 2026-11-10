//! The game's own saves and loads, seen at the seams where the script extender keeps its co-save
//! (docs/coop/SPECS.md S15): `BGSSaveLoadGame::SaveGame` at `00847c45` and `LoadGame` at
//! `00848c3c`, each with the save's name. Co-op is the second owner of both seams, after `nvse`,
//! and takes neither call: it notes the name for the session, which keeps the party's own files
//! next to the save. The game's save file is not touched.

use std::sync::{Mutex, PoisonError};

use engine_core::extend::{Owner, SiteSlot};
use engine_core::prelude::*;

use crate::addr;

/// Co-op's handler at the save seam.
pub(crate) const SAVE: SiteSlot = SiteSlot::new(Owner::Coop, addr::SAVE_GAME_SITE);
/// Co-op's handler at the load seam.
pub(crate) const LOAD: SiteSlot = SiteSlot::new(Owner::Coop, addr::LOAD_GAME_SITE);

/// The most events kept between two takes.
const EVENTS_MAX: usize = 16;

/// A save the game wrote, or one it read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The game wrote the save with this name.
    Saved(String),
    /// The game read the save with this name.
    Loaded(String),
}

/// The game-wide state (a lock-free shared slot, as the journal's).
#[derive(Debug, Default)]
pub struct Saves {
    events: Mutex<Vec<Event>>,
}

fn note(e: &mut Engine, name: u32, make: fn(String) -> Event) {
    if name == 0 {
        return;
    }
    let text: String = e.mem.cstr(name).into_iter().map(char::from).collect();
    let s = e.ext_free::<Saves>();
    let mut q = s.events.lock().unwrap_or_else(PoisonError::into_inner);
    if q.len() < EVENTS_MAX {
        q.push(make(text));
    }
}

pub(crate) fn save_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    note(e, a.first().copied().unwrap_or(0), Event::Saved);
    None
}

pub(crate) fn load_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    note(e, a.first().copied().unwrap_or(0), Event::Loaded);
    None
}

/// The saves and loads since the last call, in order.
pub fn take_events(e: &mut Engine) -> Vec<Event> {
    let s = e.ext_free::<Saves>();
    std::mem::take(&mut *s.events.lock().unwrap_or_else(PoisonError::into_inner))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    #[test]
    fn a_save_and_a_load_are_noted_with_their_names_and_neither_call_is_taken() {
        let mut s = Story::new(&StorySpec::default());
        assert_eq!(s.save("Save 12 - Goodsprings"), None);
        assert_eq!(s.load("quicksave"), None);
        assert_eq!(
            take_events(&mut s.e),
            [
                Event::Saved("Save 12 - Goodsprings".into()),
                Event::Loaded("quicksave".into())
            ]
        );
        assert!(take_events(&mut s.e).is_empty());
    }
}
