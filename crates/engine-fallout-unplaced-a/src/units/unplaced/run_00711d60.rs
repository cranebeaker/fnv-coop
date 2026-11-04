//! `(unplaced)/run_00711d60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` value accessor (`00403e20`, `this` = the setting): the
/// address of its value (+4), or of a zeroed static when `this` is null.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `fForegroundMouseMult:Controls`, `fForegroundMouseBase:Controls`,
/// `fForegroundMouseAccelTop:Controls` and `fForegroundMouseAccelBase:Controls`
/// (the names their start-up initializers `00fb4570`, `00fb4610`, `00fb46b0`
/// and the next one give them).
const MOUSE_MULT: u32 = 0x011f_35f4;
const MOUSE_BASE: u32 = 0x011f_3600;
const MOUSE_ACCEL_TOP: u32 = 0x011f_360c;
const MOUSE_ACCEL_BASE: u32 = 0x011f_3618;
/// `InterfaceManager::GetScreenWidth` (`00715d40`, engine map) and the
/// matching height function next to it (`00715da0`); neither reads `ECX`,
/// both answer in ST0.
const SCREEN_WIDTH: u32 = 0x0071_5d40;
const SCREEN_HEIGHT: u32 = 0x0071_5da0;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;

/// A setting's float as `FLD dword [value]` leaves it.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    f32::from_bits(x87_quiet_bits(e.mem.u32(value)))
}

/// `value / 2.0` (`FDIV qword`), stored as a float and loaded back.
fn half(e: &mut Engine, value: f64) -> f32 {
    let two = f64::from_bits(e.mem.u64(TWO));
    f32::from_bits(x87_quiet_bits(x87_store32(x87_div(value, two))))
}

// Translated from 00711d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the byte at
/// +0x1B50 of `this` (a field, unnamed here).
pub fn fn_00711d60(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1b50)
}

// Translated from 00711d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the value of
/// `fForegroundMouseMult:Controls`.
pub fn fn_00711d80(e: &mut Engine, _this: Ptr) -> f32 {
    setting_float(e, MOUSE_MULT)
}

// Translated from 00711da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the value of
/// `fForegroundMouseBase:Controls`.
pub fn fn_00711da0(e: &mut Engine, _this: Ptr) -> f32 {
    setting_float(e, MOUSE_BASE)
}

// Translated from 00711dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the value of
/// `fForegroundMouseAccelTop:Controls`.
pub fn fn_00711dc0(e: &mut Engine, _this: Ptr) -> f32 {
    setting_float(e, MOUSE_ACCEL_TOP)
}

// Translated from 00711de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the value of
/// `fForegroundMouseAccelBase:Controls`.
pub fn fn_00711de0(e: &mut Engine, _this: Ptr) -> f32 {
    setting_float(e, MOUSE_ACCEL_BASE)
}

// Translated from 00711e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): 1 when bit 3 of
/// the word at +4 of `this` is set, else 0 (the whole of `EAX`).
pub fn fn_00711e00(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u32(this.addr() + 4) & 8 != 0)
}

// Translated from 00711e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): minus half the screen width
/// (`00715d40`), as a float.
pub fn fn_00711e20(e: &mut Engine) -> f32 {
    let width = e.call(SCREEN_WIDTH, &[]).f64();
    half(e, -width)
}

// Translated from 00711e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): half the screen width
/// (`00715d40`), as a float.
pub fn fn_00711e40(e: &mut Engine) -> f32 {
    let width = e.call(SCREEN_WIDTH, &[]).f64();
    half(e, width)
}

// Translated from 00711e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): minus half of `00715da0`'s
/// answer (the screen height function next to the width), as a float.
pub fn fn_00711e60(e: &mut Engine) -> f32 {
    let height = e.call(SCREEN_HEIGHT, &[]).f64();
    half(e, -height)
}

// Translated from 00711e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): half of `00715da0`'s answer,
/// as a float.
pub fn fn_00711e80(e: &mut Engine) -> f32 {
    let height = e.call(SCREEN_HEIGHT, &[]).f64();
    half(e, height)
}

/// The scope marker constructor (`00404eb0`, `thiscall`: kind, flag, source
/// file, line) and its destructor (`00404ee0`); the source file is
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\Interface\InterfaceManager.cpp"`
/// (`0106ef50`).
const SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_DESTROY: u32 = 0x0040_4ee0;
const INTERFACE_MANAGER_SOURCE: u32 = 0x0106_ef50;
/// The object at `011dea0c` and `00877720` on it (`main.cpp`): the controls
/// object `Controls::ClearKeystrokes` (`00a237b0`) later runs on.
const MAIN_OBJECT: u32 = 0x011d_ea0c;
const MAIN_00877720: u32 = 0x0087_7720;
const CLEAR_KEYSTROKES: u32 = 0x00a2_37b0;
/// The menu list: `00586150` (`thiscall`, the list holder; the list is at
/// +4 of its answer), its first node (`00559450`) and the step that answers
/// the node's item and advances (`0057cbe0`, `thiscall`, the iterator).
const MENU_LIST_HOLDER: u32 = 0x0058_6150;
const LIST_HEAD: u32 = 0x0055_9450;
const LIST_NEXT: u32 = 0x0057_cbe0;
/// `Tile::GetMenu` (`00a03c90`), the menu's tile (`00726070`, `thiscall`),
/// `00712380` (`thiscall` on the tile; skip when true), the menu's fade value
/// (`00716660`, `cdecl`, ST0), `00408840` (`cdecl`, one float, ST0), the
/// menu's fade kind (`0059bb30`, `thiscall`: 2 fading in, 8 fading out by
/// the branches).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_TILE: u32 = 0x0072_6070;
const TILE_00712380: u32 = 0x0071_2380;
const MENU_FADE_VALUE: u32 = 0x0071_6660;
const FADE_CURVE_00408840: u32 = 0x0040_8840;
const MENU_FADE_KIND: u32 = 0x0059_bb30;
/// `InterfaceManager::GetEnterStackTop` (`00714f00`) and `GetEnterStack`
/// (`00714f70`, one word), engine map names.
const ENTER_STACK_TOP: u32 = 0x0071_4f00;
const ENTER_STACK: u32 = 0x0071_4f70;
/// `BGSMenuPacker::RecomputePacking` (`007037c0`, one word),
/// `Tile::IsTrue` (`00a01230`, one word), `00700320` (`thiscall` on a tile:
/// a trait and a value), `0056c7f0` (`thiscall`: the tile's node),
/// `InterfaceManager::RecursiveFade` (`00712450`: node, two floats).
const RECOMPUTE_PACKING: u32 = 0x0070_37c0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_SET_VALUE: u32 = 0x0070_0320;
const TILE_NODE: u32 = 0x0056_c7f0;
const RECURSIVE_FADE: u32 = 0x0071_2450;
/// `004e3270` (no arguments; `teswater.cpp`) and its two setters
/// (`007123f0`, a float; `007123a0`, a word).
const WATER_OBJECT: u32 = 0x004e_3270;
const WATER_SET_FADE: u32 = 0x0071_23f0;
const WATER_SET_WORD: u32 = 0x0071_23a0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// The `TES` pointer and `00451530` on it; `004b71d0` (no arguments);
/// `00450f90` (`thiscall` on a node, one word); `Tile::UpdateTile`
/// (`00a04640`, one word); the global word set to 99 (`011d8a90`).
const TES: u32 = 0x011d_ea10;
const TES_00451530: u32 = 0x0045_1530;
const GLOBAL_004B71D0: u32 = 0x004b_71d0;
const NODE_00450F90: u32 = 0x0045_0f90;
const TILE_UPDATE: u32 = 0x00a0_4640;
const STATE_011D8A90: u32 = 0x011d_8a90;

/// `x87_sub(1.0, v)` as `FLD v; FLD1; FSUBRP; FSTP; FLD` leaves it.
fn one_minus(value: u32) -> u32 {
    x87_quiet_bits(x87_store32(x87_sub(1.0, x87_load32(value))))
}

/// `RecursiveFade` on the node of `menu`'s tile with `first` and `second`
/// (float bits), after setting the tile's trait 0xFA3 to `flag`.
fn fade_menu(e: &mut Engine, this: u32, menu: u32, flag: u32, first: u32, second: u32) {
    let tile = e.call(MENU_TILE, &args![menu]).u32();
    e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, flag]);
    let tile = e.call(MENU_TILE, &args![menu]).u32();
    let node = e.call(TILE_NODE, &args![tile]).u32();
    e.call(RECURSIVE_FADE, &args![this, node, first, second]);
}

// Translation incomplete: 00711ea0 - the FS:[0] exception frame (destroying the scope marker when a call throws) is not translated
// Translated from 00711ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `InterfaceManager`, by the
/// source file its scope marker names): the menu fade update. Inside a scope
/// marker (kind 0xD, line 0xCA1) it clears the fading byte +0x11 and walks
/// the menu list: for each open menu with a tile not skipped by `00712380`,
/// the fade value (`00716660`) and whether `00408840` of it is exactly 1.0
/// decide, by the fade kind (`0059bb30`): kind 2 finished (value 1) repacks
/// (4) and, when the tile's trait 0x1772 is true, deletes the tile (slot 0
/// with 1) and stops the walk, else hides it (trait 0xFA3 = 0, fade 0, 0);
/// kind 2 unfinished sets the fading byte and fades to `1 - value`; kind 8
/// finished shows it (0xFA3 = 1, fade 1, 0) and repacks (1); kind 8
/// unfinished sets the fading byte and fades to the value. When the enter
/// stack says so (`GetEnterStackTop`, `GetEnterStack(0)`, `(1)`, menu
/// 0x3E9), the water object (`004e3270`) gets the same fade and, when
/// finished, its word set to 0. After the walk, with the state +0xC at 4 and
/// either nothing fading or `00451530` true on `TES`: clears the keystrokes,
/// sets the state to 1, shows the cursor tile (+0x28) unless `004b71d0`
/// answers true with +0x7D clear, updates it and clears +0x14C. Finally the
/// word at `011d8a90` becomes 99.
pub fn fn_00711ea0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.with_stack(8, |e, locals| {
        let scope = locals.addr();
        let iterator = locals.addr() + 4;
        e.call(
            SCOPE_CONSTRUCT,
            &args![scope, 0xdu32, 1u32, INTERFACE_MANAGER_SOURCE, 0xca1u32],
        );
        let main = e.mem.u32(MAIN_OBJECT);
        let controls = e.call(MAIN_00877720, &args![main]).u32();
        e.mem.set_u8(at + 0x11, 0);
        let holder = e.call(MENU_LIST_HOLDER, &args![at]).u32();
        let head = e.call(LIST_HEAD, &args![holder.wrapping_add(4)]).u32();
        e.mem.set_u32(iterator, head);
        while e.mem.u32(iterator) != 0 {
            let holder = e.call(MENU_LIST_HOLDER, &args![at]).u32();
            let item = e
                .call(LIST_NEXT, &args![holder.wrapping_add(4), iterator])
                .u32();
            let tile = e.mem.u32(item);
            let mut menu = 0u32;
            if tile != 0 {
                menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            }
            if menu == 0 || e.call(MENU_TILE, &args![menu]).u32() == 0 {
                continue;
            }
            let menu_tile = e.call(MENU_TILE, &args![menu]).u32();
            if e.call(TILE_00712380, &args![menu_tile]).bool() {
                continue;
            }
            let value = e.call(MENU_FADE_VALUE, &args![menu]).f64();
            let value = x87_quiet_bits(x87_store32(value));
            let curve = e.call(FADE_CURVE_00408840, &args![value]).f64();
            let finished = curve == f64::from_bits(e.mem.u64(ONE));
            let kind = e.call(MENU_FADE_KIND, &args![menu]).u32();
            let top = e.call(ENTER_STACK_TOP, &args![at]).u32();
            let first = e.call(ENTER_STACK, &args![at, 0u32]).u32();
            let second = e.call(ENTER_STACK, &args![at, 1u32]).u32();
            if kind == 2 {
                let water =
                    (top == 0 && first == 0) || (first != 0 && second == 0 && first == 0x3e9);
                if menu != 0 && finished {
                    e.call(RECOMPUTE_PACKING, &args![menu, 4u32]);
                    let menu_tile = e.call(MENU_TILE, &args![menu]).u32();
                    if e.call(TILE_IS_TRUE, &args![menu_tile, 0x1772u32]).bool() {
                        if e.call(MENU_TILE, &args![menu]).u32() != 0 {
                            let doomed = e.call(MENU_TILE, &args![menu]).u32();
                            if doomed != 0 {
                                e.vcall(doomed, 0, &args![1u32]);
                            }
                        }
                        e.mem.set_u32(iterator, 0);
                    } else {
                        let zero = 0.0f32.to_bits();
                        fade_menu(e, at, menu, 0, zero, zero);
                        if water {
                            let object = e.call(WATER_OBJECT, &[]).u32();
                            e.call(WATER_SET_FADE, &args![object, zero]);
                            let object = e.call(WATER_OBJECT, &[]).u32();
                            e.call(WATER_SET_WORD, &args![object, 0u32]);
                        }
                    }
                } else {
                    e.mem.set_u8(at + 0x11, 1);
                    let fade = one_minus(value);
                    fade_menu(e, at, menu, 1, fade, 0.0f32.to_bits());
                    if water {
                        let fade = one_minus(value);
                        let object = e.call(WATER_OBJECT, &[]).u32();
                        e.call(WATER_SET_FADE, &args![object, fade]);
                    }
                }
            } else if kind == 8 {
                let water = (top != 0 && first == top && top != 0x3e9)
                    || (second != 0 && second == top && first == 0x3e9);
                if finished {
                    let one = 1.0f32.to_bits();
                    fade_menu(e, at, menu, 1, one, 0.0f32.to_bits());
                    e.call(RECOMPUTE_PACKING, &args![menu, 1u32]);
                    if water {
                        let object = e.call(WATER_OBJECT, &[]).u32();
                        e.call(WATER_SET_FADE, &args![object, one]);
                        let object = e.call(WATER_OBJECT, &[]).u32();
                        e.call(WATER_SET_WORD, &args![object, 0u32]);
                    }
                } else {
                    e.mem.set_u8(at + 0x11, 1);
                    fade_menu(e, at, menu, 1, value, 0.0f32.to_bits());
                    if water {
                        let object = e.call(WATER_OBJECT, &[]).u32();
                        e.call(WATER_SET_FADE, &args![object, value]);
                    }
                }
            }
        }
        if e.mem.u32(at + 0xc) == 4 {
            let settled = e.mem.u8(at + 0x11) == 0 || {
                let tes = e.mem.u32(TES);
                e.call(TES_00451530, &args![tes]).bool()
            };
            if settled {
                e.call(CLEAR_KEYSTROKES, &args![controls]);
                e.mem.set_u32(at + 0xc, 1);
                if !e.call(GLOBAL_004B71D0, &[]).bool() || e.mem.u8(at + 0x7d) != 0 {
                    let cursor = e.mem.u32(at + 0x28);
                    let node = e.call(TILE_NODE, &args![cursor]).u32();
                    e.call(NODE_00450F90, &args![node, 1u32]);
                    let cursor = e.mem.u32(at + 0x28);
                    e.call(TILE_SET_VALUE, &args![cursor, 0xfa3u32, 0u32]);
                }
                let cursor = e.mem.u32(at + 0x28);
                e.call(TILE_UPDATE, &args![cursor, 1u32]);
                e.mem.set_u32(at + 0x14c, 0);
            }
        }
        e.mem.set_u32(STATE_011D8A90, 99);
        e.call(SCOPE_DESTROY, &args![scope]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00711ea0, fn_00711ea0(Ptr)),
        entry!(0x00711d60, fn_00711d60(Ptr) -> u8),
        entry!(0x00711d80, fn_00711d80(Ptr) -> f32),
        entry!(0x00711da0, fn_00711da0(Ptr) -> f32),
        entry!(0x00711dc0, fn_00711dc0(Ptr) -> f32),
        entry!(0x00711de0, fn_00711de0(Ptr) -> f32),
        entry!(0x00711e00, fn_00711e00(Ptr) -> u32),
        entry!(0x00711e20, fn_00711e20() -> f32),
        entry!(0x00711e40, fn_00711e40() -> f32),
        entry!(0x00711e60, fn_00711e60() -> f32),
        entry!(0x00711e80, fn_00711e80() -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_and_flag_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c00);
        e.mem.set_u8(this + 0x1b50, 3);
        assert_eq!(e.call(0x0071_1d60, &args![this]).u8(), 3);
        e.mem.set_u32(this + 4, 0x8);
        assert_eq!(e.call(0x0071_1e00, &args![this]).u32(), 1);
        e.mem.set_u32(this + 4, 0xf7);
        assert_eq!(e.call(0x0071_1e00, &args![this]).u32(), 0);
    }

    #[test]
    fn mouse_settings_are_read_through_the_accessor() {
        let mut e = Engine::new();
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.map(0x011f_3000, 0x1000);
        for (i, setting) in [MOUSE_MULT, MOUSE_BASE, MOUSE_ACCEL_TOP, MOUSE_ACCEL_BASE]
            .into_iter()
            .enumerate()
        {
            e.set_global(setting + 4, i as f32 + 0.5);
        }
        assert_eq!(e.call(0x0071_1d80, &args![0u32]).f32(), 0.5);
        assert_eq!(e.call(0x0071_1da0, &args![0u32]).f32(), 1.5);
        assert_eq!(e.call(0x0071_1dc0, &args![0u32]).f32(), 2.5);
        assert_eq!(e.call(0x0071_1de0, &args![0u32]).f32(), 3.5);
    }

    #[test]
    fn screen_half_extents() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u64(TWO, 2.0f64.to_bits());
        e.register(SCREEN_WIDTH, |_, _| 1280.0f64.into_ret());
        e.register(SCREEN_HEIGHT, |_, _| 720.0f64.into_ret());
        assert_eq!(e.call(0x0071_1e20, &[]).f32(), -640.0);
        assert_eq!(e.call(0x0071_1e40, &[]).f32(), 640.0);
        assert_eq!(e.call(0x0071_1e60, &[]).f32(), -360.0);
        assert_eq!(e.call(0x0071_1e80, &[]).f32(), 360.0);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An interface manager whose menu list holds menus with the given
    /// (fade value, fade kind); each menu's tile is the menu + 0x200 (its
    /// vtable slot 0 the no-op `0x0019_9000`), its node the tile + 1, the
    /// list's nodes `{next, tile}`. Answers (engine, manager, menus).
    fn fade_rig(menus: &[(f32, u32)]) -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        for address in [
            SCOPE_CONSTRUCT,
            SCOPE_DESTROY,
            TILE_00712380,
            RECOMPUTE_PACKING,
            TILE_IS_TRUE,
            TILE_SET_VALUE,
            RECURSIVE_FADE,
            WATER_SET_FADE,
            WATER_SET_WORD,
            CLEAR_KEYSTROKES,
            TES_00451530,
            GLOBAL_004B71D0,
            NODE_00450F90,
            TILE_UPDATE,
            ENTER_STACK_TOP,
            ENTER_STACK,
            0x0019_9000,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MAIN_00877720, |_, _| 0xc0c0u32.into_ret());
        e.register(WATER_OBJECT, |_, _| 0x3a3au32.into_ret());
        e.register(TILE_GET_MENU, |_, a| (a[0] - 0x200).into_ret());
        e.register(MENU_TILE, |_, a| (a[0] + 0x200).into_ret());
        e.register(TILE_NODE, |_, a| (a[0] + 1).into_ret());
        e.register(MENU_FADE_VALUE, |e, a| {
            f64::from(e.mem.f32(a[0])).into_ret()
        });
        e.register(MENU_FADE_KIND, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FADE_CURVE_00408840, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        e.put_vtable(0x0019_8000, &[0x0019_9000]);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_u64(ONE, 1.0f64.to_bits());
        e.map(0x011d_8000, 0x7000);
        let mut made = Vec::new();
        let mut next = 0u32;
        let mut first = 0u32;
        for (value, kind) in menus.iter().rev() {
            let menu = e.mem.alloc(0x300);
            e.mem.set_f32(menu, *value);
            e.mem.set_u32(menu + 4, *kind);
            e.mem.set_u32(menu + 0x200, 0x0019_8000);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, menu + 0x200);
            next = node;
            first = node;
            made.insert(0, menu);
        }
        let holder = e.mem.alloc(8);
        e.register_double(MENU_LIST_HOLDER, move |_, _| holder.into_ret());
        e.register_double(LIST_HEAD, move |_, _| first.into_ret());
        e.register(LIST_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let following = e.mem.u32(node);
            e.mem.set_u32(a[1], following);
            (node + 4).into_ret()
        });
        let manager = e.mem.alloc(0x150);
        e.mem.set_u32(manager + 0x28, 0x5550);
        (e, manager, made)
    }

    #[test]
    fn fading_in_menu_fades_and_marks_fading() {
        let (mut e, manager, menus) = fade_rig(&[(0.25, 2)]);
        e.mem.set_u32(manager + 0xc, 4);
        e.call_log = Some(vec![]);
        e.call(0x0071_1ea0, &args![manager]);
        let tile = menus[0] + 0x200;
        assert_eq!(e.mem.u8(manager + 0x11), 1);
        assert_eq!(calls_to(&e, TILE_SET_VALUE), vec![vec![tile, 0xfa3, 1]]);
        assert_eq!(
            calls_to(&e, RECURSIVE_FADE),
            vec![vec![manager, tile + 1, 0.75f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, WATER_SET_FADE),
            vec![vec![0x3a3a, 0.75f32.to_bits()]]
        );
        // Still fading and TES says no: the state stays.
        assert_eq!(e.mem.u32(manager + 0xc), 4);
        assert!(calls_to(&e, CLEAR_KEYSTROKES).is_empty());
        assert_eq!(e.mem.u32(STATE_011D8A90), 99);
    }

    #[test]
    fn finished_fade_out_settles_the_state() {
        let (mut e, manager, menus) = fade_rig(&[(1.0, 8)]);
        e.mem.set_u32(manager + 0xc, 4);
        e.mem.set_u32(manager + 0x14c, 7);
        e.call_log = Some(vec![]);
        e.call(0x0071_1ea0, &args![manager]);
        let tile = menus[0] + 0x200;
        assert_eq!(e.mem.u8(manager + 0x11), 0);
        assert_eq!(
            calls_to(&e, RECURSIVE_FADE),
            vec![vec![manager, tile + 1, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(calls_to(&e, RECOMPUTE_PACKING), vec![vec![menus[0], 1]]);
        assert!(calls_to(&e, WATER_SET_FADE).is_empty());
        assert_eq!(calls_to(&e, CLEAR_KEYSTROKES), vec![vec![0xc0c0]]);
        assert_eq!(e.mem.u32(manager + 0xc), 1);
        assert_eq!(calls_to(&e, NODE_00450F90), vec![vec![0x5551, 1]]);
        assert_eq!(calls_to(&e, TILE_UPDATE), vec![vec![0x5550, 1]]);
        assert_eq!(e.mem.u32(manager + 0x14c), 0);
    }

    #[test]
    fn finished_fade_in_with_the_trait_deletes_the_tile_and_stops() {
        let (mut e, manager, menus) = fade_rig(&[(1.0, 2), (0.5, 8)]);
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0071_1ea0, &args![manager]);
        let tile = menus[0] + 0x200;
        assert_eq!(calls_to(&e, RECOMPUTE_PACKING), vec![vec![menus[0], 4]]);
        assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![tile, 0x1772]]);
        assert_eq!(calls_to(&e, 0x0019_9000), vec![vec![tile, 1]]);
        // The walk stopped: the second menu was not faded.
        assert!(calls_to(&e, RECURSIVE_FADE).is_empty());
    }
}
