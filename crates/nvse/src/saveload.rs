//! What xNVSE does around loading and starting a game, as far as it applies before the co-save
//! (I-10): translated from `Hooks_SaveLoad.cpp`, `Serialization.cpp` and `Core_Serialization.cpp`
//! (xNVSE), at the game's own load and new-game sites (ADR-0016 §10).
//!
//! - **Before a game is loaded** (the seam at `0x00847FD9` in `BGSSaveLoadGame::LoadGame`, after
//!   the call to `00852b60`, where xNVSE's `PreLoadGameHook` runs): with xNVSE every save has a
//!   co-save (one is written on every save), and reading it first runs `Core_PreLoadCallback`:
//!   the variables recorded per event list are forgotten, the delayed calls dropped, and the
//!   string and array variables reset (ids from 1 again); then the co-save's own variables are
//!   read. Until the port writes co-saves (I-10) it does the same as for a co-save holding no
//!   variables: the reset alone.
//! - **A new game** (the seam at `0x007D33CE` in `StartMenu`'s new-game confirmation, in place
//!   of the call xNVSE overwrites, and at `0x0093DB69` in `PlayerCharacter::CenterOnCell` when a
//!   `coc` from the main menu starts one: in the start menu (`0070edf0`) and not starting a new
//!   game the usual way (`0x011D8907`)): `NewGameHook`: the delayed calls dropped, then
//!   `Core_NewGameCallback`: the temporary strings and arrays cleaned (the others stay, as in
//!   xNVSE).
//!
//! xNVSE resets nothing else here: lambdas keep their scripts (their parent lists go as the game
//! destroys them, through the event list wrap of [`crate::vars`]), and the loop stack is empty
//! between script runs ([`crate::commands::scripting::unwind`]).

use crate::{tick, vars};
use engine_core::extend::{Owner, SiteSlot};
use engine_core::prelude::*;

/// `BGSSaveLoadGame::LoadGame`'s call to `00852b60` (xNVSE's `kPreLoadGamePatchAddr`).
pub const PRE_LOAD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0084_7fd9);
/// The new-game confirmation's call to the empty `0040fbe0` (`kNewGamePatchAddr`).
pub const NEW_GAME: SiteSlot = SiteSlot::new(Owner::Nvse, 0x007d_33ce);
/// `CenterOnCell`'s call to the empty `0040fbe0` (`NewGameWithNoSaveLoaded`).
pub const CENTER_ON_CELL: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0093_db69);

/// `StartMenu`'s "in the start menu" test (`cdecl`) and the "starting a new game" flag
/// (`kIsStartingNewGameAddr`).
const IS_IN_START_MENU: u32 = 0x0070_edf0;
const IS_STARTING_NEW_GAME: u32 = 0x011d_8907;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Core_Serialization.cpp:
// Core_PreLoadCallback (the part before the co-save's records).
/// Before a game is loaded.
pub fn pre_load(e: &mut Engine) {
    vars::forget_collected(e);
    tick::clear_delayed_calls(e);
    vars::reset(e);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_SaveLoad.cpp: NewGameHook, and
// Core_Serialization.cpp: Core_NewGameCallback.
/// A new game.
pub fn new_game(e: &mut Engine) {
    tick::clear_delayed_calls(e);
    vars::clean_arrays(e);
    vars::clean_strings(e);
}

fn pre_load_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
    pre_load(e);
    None
}

fn new_game_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
    new_game(e);
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_SaveLoad.cpp:
// NewGameWithNoSaveLoaded::Hook.
fn center_on_cell_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
    let in_start_menu = e.call(IS_IN_START_MENU, &[]).bool();
    if in_start_menu && e.mem.u8(IS_STARTING_NEW_GAME) == 0 {
        new_game(e);
    }
    None
}

/// Installs the three seams' handlers.
pub fn install(e: &mut Engine) {
    e.on_site(PRE_LOAD, pre_load_site);
    e.on_site(NEW_GAME, new_game_site);
    e.on_site(CENTER_ON_CELL, center_on_cell_site);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tick::{Delayed, While};
    use crate::vars::Data;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0200_0000, 0x1_0000);
        e.map(IS_STARTING_NEW_GAME & !0xfff, 0x1000);
        e.register(0x00ec_a7f4, |_, a| Ret {
            eax: u32::from((a[0] as u8).to_ascii_uppercase()),
            ..Ret::default()
        });
        install(&mut e);
        e
    }

    fn delayed(e: &mut Engine, args: Vec<Data>) {
        let d = Delayed {
            script: 0,
            time: 5.0,
            this_obj: 0,
            mode: 0,
            args,
            serial: 1,
        };
        e.ext::<crate::Nvse>().tick.after.push(d);
        let w = While {
            call: 0,
            condition: 0,
            this_obj: 0,
            flags: 0,
            args: Vec::new(),
            interval: 0.0,
            old_time: 0.0,
            serial: 2,
        };
        e.ext::<crate::Nvse>().tick.when.push(w);
    }

    /// Review item 2: before a load (the game's own seam) the delayed calls go, the string and
    /// array variables are reset (ids from 1 again) and the recorded locals forgotten; a new
    /// game drops the delayed calls and cleans only the temporaries.
    #[test]
    fn load_and_new_game_reset_as_xnvse() {
        let mut e = engine();
        let kept = vars::add_string(&mut e, 1, b"kept", false);
        let arr = vars::create_array(&mut e, vars::KEY_NUMBER, true, 1);
        let mut held = 0;
        vars::add_reference(&mut e, &mut held, arr, 1);
        vars::collect(&mut e, 0x0200_1000, 0x0200_2000, vars::VarType::String);
        delayed(&mut e, Vec::new());
        e.site(PRE_LOAD.addr(), &[]);
        let st = &e.ext::<crate::Nvse>().tick;
        let calls_left = st.after.len() + st.when.len();
        assert_eq!(calls_left, 0);
        assert!(vars::string(&e, kept).is_none());
        assert!(!vars::array_exists(&e, arr));
        assert!(vars::collected_kinds(&mut e, 0x0200_1000).is_empty());
        assert_eq!(vars::add_string(&mut e, 1, b"new", false), 1);
        // A new game: the delayed calls go, a permanent string stays, a temporary one goes.
        let temp = vars::add_string(&mut e, 1, b"temp", true);
        delayed(&mut e, Vec::new());
        e.site(NEW_GAME.addr(), &[]);
        assert!(e.ext::<crate::Nvse>().tick.after.is_empty());
        assert_eq!(vars::string(&e, 1), Some(b"new".to_vec()));
        assert!(vars::string(&e, temp).is_none());
    }

    /// `coc` from the main menu is a new game only in the start menu and when no new game is
    /// being started the usual way.
    #[test]
    fn coc_from_the_main_menu_is_a_new_game() {
        let mut e = engine();
        e.register(IS_IN_START_MENU, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        delayed(&mut e, Vec::new());
        e.mem.set_u8(IS_STARTING_NEW_GAME, 1);
        e.site(CENTER_ON_CELL.addr(), &[]);
        assert_eq!(e.ext::<crate::Nvse>().tick.after.len(), 1);
        e.mem.set_u8(IS_STARTING_NEW_GAME, 0);
        e.site(CENTER_ON_CELL.addr(), &[]);
        assert!(e.ext::<crate::Nvse>().tick.after.is_empty());
    }
}
