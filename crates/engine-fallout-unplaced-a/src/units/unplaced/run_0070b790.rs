//! `(unplaced)/run_0070b790` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Tile::GetMenuByClass` (`00a09030`, engine map; `cdecl`, the menu
/// class) and `Tile::GetMenu` (`00a03c90`, engine map; `thiscall`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `StatsMenu::SetCurrentPage` (`007dff40`, engine map; `thiscall`, one
/// word).
const STATS_MENU_SET_CURRENT_PAGE: u32 = 0x007d_ff40;
/// `00706a40` (`cdecl`, one word) and `007069f0` (`thiscall`, one word),
/// both `fallout/interface/interface.cpp` by the engine map.
const INTERFACE_00706A40: u32 = 0x0070_6a40;
const INTERFACE_007069F0: u32 = 0x0070_69f0;

/// The open menu of `class`: `Tile::GetMenuByClass`, then `Tile::GetMenu`
/// on its tile (null when there is no tile).
fn open_menu(e: &mut Engine, class: u32) -> u32 {
    let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if tile == 0 {
        return 0;
    }
    e.call(TILE_GET_MENU, &args![tile]).u32()
}

// Translated from 0070b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`): stores `value`
/// in the byte at +0x12 and, when the menu of class `0x3EB` is open, hands it
/// (zero-extended) to that menu through `0070b7f0`.
pub fn fn_0070b790(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x12, value);
    let menu = open_menu(e, 0x3eb);
    if menu != 0 {
        fn_0070b7f0(e, Ptr::new(menu), value);
    }
}

// Translated from 0070b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`):
/// `StatsMenu::SetCurrentPage` on `this` with `page` sign-extended.
pub fn fn_0070b7f0(e: &mut Engine, this: Ptr, page: u8) {
    e.call(
        STATS_MENU_SET_CURRENT_PAGE,
        &args![this, i32::from(page as i8)],
    );
}

// Translated from 0070b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`): stores `value`
/// in the byte at +0x13 and, when the menu of class `0x3EA` is open, calls
/// `00706a40` with `value` sign-extended.
pub fn fn_0070b810(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x13, value);
    if open_menu(e, 0x3ea) != 0 {
        e.call(INTERFACE_00706A40, &args![i32::from(value as i8)]);
    }
}

// Translated from 0070b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte, `RET 4`): stores `value`
/// in the byte at +0x15 and, when the menu of class `0x3FF` is open, calls
/// `007069f0` on that menu with `value` sign-extended.
pub fn fn_0070b870(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x15, value);
    let menu = open_menu(e, 0x3ff);
    if menu != 0 {
        e.call(INTERFACE_007069F0, &args![menu, i32::from(value as i8)]);
    }
}

// Translated from 0070b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): only replaces
/// its own argument slot with `0x3EB` when it is 0; nothing outside the
/// function changes.
pub fn fn_0070b8d0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070b790, fn_0070b790(Ptr, u8)),
        entry!(0x0070b7f0, fn_0070b7f0(Ptr, u8)),
        entry!(0x0070b810, fn_0070b810(Ptr, u8)),
        entry!(0x0070b870, fn_0070b870(Ptr, u8)),
        entry!(0x0070b8d0, fn_0070b8d0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Menus open for the classes 0x3EB and 0x3FF only (tile = class,
    /// menu = class + 0x1000).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(TILE_GET_MENU_BY_CLASS, |_, a| {
            if a[0] == 0x3eb || a[0] == 0x3ff {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(TILE_GET_MENU, |_, a| (a[0] + 0x1000).into_ret());
        for address in [
            STATS_MENU_SET_CURRENT_PAGE,
            INTERFACE_00706A40,
            INTERFACE_007069F0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn stats_page_goes_to_the_open_menu() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0070_b790, &args![this, 0xffu32]);
        assert_eq!(e.mem.u8(this + 0x12), 0xff);
        assert_eq!(
            calls_to(&e, STATS_MENU_SET_CURRENT_PAGE),
            vec![vec![0x13eb, 0xffff_ffff]]
        );
    }

    #[test]
    fn closed_menu_gets_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0070_b810, &args![this, 2u32]);
        assert_eq!(e.mem.u8(this + 0x13), 2);
        assert!(calls_to(&e, INTERFACE_00706A40).is_empty());
        e.call(0x0070_b870, &args![this, 0x80u32]);
        assert_eq!(e.mem.u8(this + 0x15), 0x80);
        assert_eq!(
            calls_to(&e, INTERFACE_007069F0),
            vec![vec![0x13ff, 0xffff_ff80]]
        );
        e.register(TILE_GET_MENU_BY_CLASS, |_, a| a[0].into_ret());
        e.call(0x0070_b810, &args![this, 3u32]);
        assert_eq!(calls_to(&e, INTERFACE_00706A40), vec![vec![3]]);
    }

    #[test]
    fn argument_only_function_changes_nothing() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0070_b8d0, &args![0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
