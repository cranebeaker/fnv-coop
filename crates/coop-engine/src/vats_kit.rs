//! The testkit's V.A.T.S. (docs/coop/SPECS.md S20): the game's V.A.T.S. mode, a menu stack of
//! one, the game's time multiplier, and stand-ins for the three game functions co-op's V.A.T.S.
//! code wraps or calls. Each does what co-op relies on the real one to do, and no more:
//!
//! - `Interface::IsInMenuMode` answers whether a menu is up;
//! - `Interface::IsTopMenuID` answers whether the top menu is the one asked for;
//! - `BSTimer::SetGlobalTimeMultiplier` stores the multiplier.

use engine_core::prelude::*;

use super::{bump, eax, Story};
use crate::addr;

/// Where the kit keeps the top menu's class (0: none), the multiplier's bits and the count of
/// multiplier calls.
const STATE: u32 = 0x0171_0000;
const TOP_MENU: u32 = STATE;
const MULTIPLIER: u32 = STATE + 4;
const MULTIPLIER_CALLS: u32 = STATE + 8;
/// The timer the kit's calls are made on (the address `vats.rs` passes).
const TIMER: u32 = 0x011f_6394;

fn in_menu_mode(e: &mut Engine, _: &[u32]) -> Ret {
    eax(u32::from(e.mem.u32(TOP_MENU) != 0))
}

fn is_top_menu(e: &mut Engine, a: &[u32]) -> Ret {
    eax(u32::from(a[0] != 0 && e.mem.u32(TOP_MENU) == a[0]))
}

fn set_multiplier(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(MULTIPLIER, a[1]);
    bump(e, MULTIPLIER_CALLS);
    Ret::default()
}

/// Called by `Story::new` before co-op is installed.
pub(super) fn register(e: &mut Engine) {
    e.map(STATE, 0x100);
    e.map(addr::VATS & !0xfff, 0x1000);
    e.mem.set_f32(MULTIPLIER, 1.0);
    e.register(addr::INTERFACE_IS_IN_MENU_MODE, in_menu_mode);
    e.register(addr::INTERFACE_IS_TOP_MENU_ID, is_top_menu);
    e.register(addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER, set_multiplier);
}

impl Story {
    /// Puts the game's V.A.T.S. in `mode` (0 none, 2 the menu, 4 playback), as `VATS::SetMode`
    /// stores it.
    pub fn set_vats_mode(&mut self, mode: u32) {
        self.e
            .mem
            .set_u32(addr::VATS + addr::VATS_MODE_OFFSET, mode);
    }

    /// Opens a menu of class `class` on top of whatever is open.
    pub fn open_menu(&mut self, class: u32) {
        self.e.mem.set_u32(TOP_MENU, class);
    }

    /// Closes every menu.
    pub fn close_menu(&mut self) {
        self.e.mem.set_u32(TOP_MENU, 0);
    }

    /// What the game's idle update is told when it asks whether a menu is up (and so whether
    /// the world stands still).
    pub fn in_menu_mode(&mut self) -> bool {
        self.e
            .call(addr::INTERFACE_IS_IN_MENU_MODE, &args![])
            .bool()
    }

    /// The game asks for a time multiplier, as the V.A.T.S. code does.
    pub fn ask_time_multiplier(&mut self, multiplier: f32) {
        self.e.call(
            addr::TIMER_SET_GLOBAL_TIME_MULTIPLIER,
            &args![TIMER, multiplier.to_bits(), 0u32],
        );
    }

    /// The time multiplier the game runs at.
    pub fn time_multiplier(&mut self) -> f32 {
        self.e.mem.f32(MULTIPLIER)
    }

    /// How often the game's multiplier was set.
    pub fn time_multiplier_calls(&mut self) -> u32 {
        self.e.mem.u32(MULTIPLIER_CALLS)
    }
}
