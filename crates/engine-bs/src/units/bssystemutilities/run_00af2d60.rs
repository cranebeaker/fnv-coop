//! `BSSystemUtilities/run_00af2d60` (the engine map's run of unnamed functions from
//! `00af2d60`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00af2d60 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00af2d60(_e: &mut Engine, _this: u32) {}

/// The `steam_api.dll` import slots `SteamAPI_IsSteamRunning` (`00fdf498`, no arguments, a
/// `bool` in `AL`) and `SteamUser` (`00fdf484`, no arguments), and the `ISteamUser` slot +4
/// (`thiscall`, no words, a `bool` in `AL`) the exe calls.
const STEAM_IS_RUNNING: u32 = 0x00fd_f498;
const STEAM_USER: u32 = 0x00fd_f484;
const USER_LOGGED_ON: u32 = 4;

// Translated from 00af2db0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The network utility's idle step (no label; `thiscall`, plain `RET`). The byte at +0
/// becomes 1 when `SteamAPI_IsSteamRunning()` is true, `SteamUser()` is not null and a second
/// `SteamUser()`'s slot +4 answers true, else 0 (the calls stop at the first that fails). When
/// the byte changed and the word at +4 (a function) is not null, that function is called
/// (`cdecl`, two words: 0 and whether the byte is now set).
pub fn fn_00af2db0(e: &mut Engine, this: u32) {
    let before = e.mem.u8(this);
    let mut connected = 0u8;
    if e.call(STEAM_IS_RUNNING, &args![]).u8() != 0 && e.call(STEAM_USER, &args![]).u32() != 0 {
        let user = e.call(STEAM_USER, &args![]).u32();
        if e.vcall(user, USER_LOGGED_ON, &args![]).u8() != 0 {
            connected = 1;
        }
    }
    e.mem.set_u8(this, connected);
    let now = e.mem.u8(this);
    let callback = e.mem.u32(this.wrapping_add(4));
    if now != before && callback != 0 {
        e.call(callback, &args![0u32, u32::from(now != 0)]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_2d60, fn_00af2d60(u32)),
        entry!(0x00af_2db0, fn_00af2db0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x55);
        e.call(0x00af_2d60, &args![this]);
        assert_eq!(e.mem.u32(this), 0x55);
    }

    const CALLBACK: u32 = 0x0e00_0100;
    const LOGGED_ON: u32 = 0x0e00_0200;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A utility object (state byte `state`, the callback set) and Steam answering `running`,
    /// a user object when `user`, and `logged_on` from its slot +4.
    fn engine(state: u8, running: u8, user: bool, logged_on: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u8(this, state);
        e.mem.set_u32(this + 4, CALLBACK);
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + USER_LOGGED_ON, LOGGED_ON);
        e.register_double(STEAM_IS_RUNNING, move |_, _| u32::from(running).into_ret());
        let answer = if user { object } else { 0 };
        e.register_double(STEAM_USER, move |_, _| answer.into_ret());
        e.register_double(LOGGED_ON, move |_, _| u32::from(logged_on).into_ret());
        e.register(CALLBACK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn without_steam_running_the_state_clears_and_the_callback_hears_of_it() {
        let (mut e, this) = engine(1, 0, true, 1);
        e.call(0x00af_2db0, &args![this]);
        assert_eq!(e.mem.u8(this), 0);
        assert!(calls(&e, STEAM_USER).is_empty());
        assert_eq!(calls(&e, CALLBACK), vec![vec![0, 0]]);
        // Unchanged: no callback.
        e.call(0x00af_2db0, &args![this]);
        assert_eq!(calls(&e, CALLBACK).len(), 1);
    }

    #[test]
    fn a_logged_on_user_sets_the_state() {
        let (mut e, this) = engine(0, 1, true, 1);
        e.call(0x00af_2db0, &args![this]);
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(calls(&e, STEAM_USER).len(), 2);
        assert_eq!(calls(&e, CALLBACK), vec![vec![0, 1]]);
        // A null user or one not logged on: 0.
        let (mut e, this) = engine(1, 1, false, 1);
        e.call(0x00af_2db0, &args![this]);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(calls(&e, STEAM_USER).len(), 1);
        let (mut e, this) = engine(0, 1, true, 0);
        e.mem.set_u32(this + 4, 0);
        e.call(0x00af_2db0, &args![this]);
        assert_eq!(e.mem.u8(this), 0);
        assert!(calls(&e, CALLBACK).is_empty());
    }
}
