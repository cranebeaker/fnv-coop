//! `BSMenu/tilemenu.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TileMenu` is the root tile of a menu: a `TileRect` that owns the
//! `Menu` object its class trait creates.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::tile::{
    Tile, FLAG_RELEASED, TRAIT_CLASS, TRAIT_DEPTH, TRAIT_LOCUS, TRAIT_STACKING_TYPE,
    TRAIT_SYSTEM_COLOR, TRAIT_VISIBLE, TRAIT_X, TRAIT_Y,
};

layout! {
    /// `TileMenu` (Xbox PDB), 0x40 bytes: a `TileRect` and its menu.
    pub struct TileMenu: 0x40 {
        /// `pMenu` (Xbox PDB): `Menu*`.
        0x3C pMenu: Ptr,
    }
}

/// `Tile::Init` (`009ff600`, Xbox PDB name).
const TILE_INIT: u32 = 0x009f_f600;
/// Sets a float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::SetNeedsUpdate` (`00a074d0`, Xbox PDB name).
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
/// The float value of a trait (`00a011b0`, thiscall: trait; `ST0`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::Release` (`009ff690`, Xbox PDB name).
const TILE_RELEASE: u32 = 0x009f_f690;
/// `Tile::~Tile` (`009ff340`, Xbox PDB name).
const TILE_DESTRUCTOR: u32 = 0x009f_f340;
/// `TileRect::MakeNode` (`00a1f3b0`, Xbox PDB name).
const TILE_RECT_MAKE_NODE: u32 = 0x00a1_f3b0;
/// `Menu::SetMenuTile` (`00a1dc70`, Xbox PDB name; thiscall: tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::GetMaxDepth` (`00a1dfb0`, Xbox PDB name; no arguments, `ST0`).
const MENU_GET_MAX_DEPTH: u32 = 0x00a1_dfb0;
/// Records the tile of a menu id (`00a1e130`, BSMenu/menu.cpp, cdecl: id,
/// tile).
const MENU_SET_TILE_FOR_ID: u32 = 0x00a1_e130;
/// `Interface::CreateMenuByClass` (`00707990`, Xbox PDB name, cdecl).
const INTERFACE_CREATE_MENU_BY_CLASS: u32 = 0x0070_7990;
/// The interface's error print (`00703c00`, fallout/interface/interface.cpp,
/// cdecl with a format string).
const INTERFACE_PRINT_ERROR: u32 = 0x0070_3c00;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// The `Menu` virtual slot that answers the menu's id (`+0x34`).
const MENU_ID_SLOT: u32 = 0x34;
/// The `Menu` virtual slot `PostParse` hands user traits to (`+0x40`).
const MENU_USER_TRAIT_SLOT: u32 = 0x40;
/// `TileMenu`'s and `TileRect`'s vtables (`0106ed44`, `0106ed70`; the RTTI
/// names `.?AVTileMenu@@` and `.?AVTileRect@@`).
const TILE_MENU_VTABLE: u32 = 0x0106_ed44;
const TILE_RECT_VTABLE: u32 = 0x0106_ed70;
/// `0.25f` (`0101622c`): what `Init` gives trait `0xFC1`.
const INITIAL_TRAIT_FC1: u32 = 0x0101_622c;
/// Trait `0xFC1` (no name in `crate::units::bsmenu::tile`).
const TRAIT_FC1: i32 = 0xfc1;
/// The two stacking-type values that give the menu its depth: `102.0` and
/// `103.0` (`double`, `010704e8` and `01094bb8`).
const STACKING_TYPE_A: u32 = 0x0107_04e8;
const STACKING_TYPE_B: u32 = 0x0109_4bb8;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// `"ERROR: Cant find menu class!"` (`0109569c`).
const NO_MENU_CLASS: u32 = 0x0109_569c;
/// The first user trait id (`0x2710`).
const FIRST_USER_TRAIT: i32 = 0x2710;
/// The menu ids that have a byte in the table at `011f3478` (`0x3E9` to
/// `0x43C`); the code indexes it as `011f308f + id`.
const FIRST_MENU_ID: i32 = 0x3e9;
const LAST_MENU_ID: i32 = 0x43c;
const MENU_ID_BYTES: u32 = 0x011f_308f;

/// The menu's id (virtual slot `+0x34`).
fn menu_id(e: &mut Engine, menu: Ptr) -> i32 {
    e.vcall(menu.addr(), MENU_ID_SLOT, &args![]).i32()
}

/// `_ftol2_sse` on a `float` loaded by `FLD`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![x87_load32(value.to_bits())]).i32()
}

// Translated from 00a1ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::Init` (Xbox PDB): `Tile::Init` with the same three words,
/// then x and y 0, locus 1, trait `0xFC1` the float at `0101622c`, system
/// colour 1 and visible 0 (each with flag word 1), the update bits 3
/// queued, and no menu.
pub fn tile_menu_init(
    e: &mut Engine,
    this: Ptr<TileMenu>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.call(TILE_INIT, &args![this, parent, name, set_parent_arg]);
    let set = |e: &mut Engine, trait_id: i32, bits: u32| {
        e.call(TILE_SET_FLOAT, &args![this, trait_id, bits, 1u32]);
    };
    set(e, TRAIT_X, 0.0f32.to_bits());
    set(e, TRAIT_Y, 0.0f32.to_bits());
    set(e, TRAIT_LOCUS, 1.0f32.to_bits());
    let initial = x87_quiet_bits(e.global::<u32>(INITIAL_TRAIT_FC1));
    set(e, TRAIT_FC1, initial);
    set(e, TRAIT_SYSTEM_COLOR, 1.0f32.to_bits());
    set(e, TRAIT_VISIBLE, 0.0f32.to_bits());
    e.call(TILE_SET_NEEDS_UPDATE, &args![this, 3u32]);
    e.set(this, TileMenu::pMenu, Ptr::NULL);
}

// Translated from 00a1eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TileMenu` destructor (no PDB name; BSMenu/tilemenu.cpp): with a
/// menu, clears the menu id's byte (ids `0x3E9` to `0x43C`), forgets the
/// id's tile (`00a1e130(id, 0)`), detaches the menu's tile
/// (`SetMenuTile(0, 0)`) and deletes the menu (virtual slot 0, flag 1);
/// then, for the `TileMenu` and again for the `TileRect` part (vtables
/// `0106ed44` and `0106ed70`), runs `Tile::Release` unless the tile is
/// already released (flag `0x2000`), and finally `Tile::~Tile`.
// Translation incomplete: 00a1eff0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_00a1eff0(e: &mut Engine, this: Ptr<TileMenu>) {
    e.mem.set_u32(this.addr(), TILE_MENU_VTABLE);
    if !e.get(this, TileMenu::pMenu).is_null() {
        let menu = e.get(this, TileMenu::pMenu);
        let id = menu_id(e, menu);
        if (FIRST_MENU_ID..=LAST_MENU_ID).contains(&id) {
            e.mem.set_u8(MENU_ID_BYTES.wrapping_add(id as u32), 0);
        }
        let menu = e.get(this, TileMenu::pMenu);
        let id = menu_id(e, menu);
        e.call(MENU_SET_TILE_FOR_ID, &args![id, 0u32]);
        let menu = e.get(this, TileMenu::pMenu);
        e.call(MENU_SET_MENU_TILE, &args![menu, 0u32, 0u32]);
        let menu = e.get(this, TileMenu::pMenu);
        if !menu.is_null() {
            e.vcall(menu.addr(), 0, &args![1u32]);
        }
    }
    let tile = this.cast::<Tile>();
    if e.get(tile, Tile::uiFlags) & FLAG_RELEASED == 0 {
        e.call(TILE_RELEASE, &args![this]);
    }
    e.mem.set_u32(this.addr(), TILE_RECT_VTABLE);
    if e.get(tile, Tile::uiFlags) & FLAG_RELEASED == 0 {
        e.call(TILE_RELEASE, &args![this]);
    }
    e.call(TILE_DESTRUCTOR, &args![this]);
}

// Translated from 00a1f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::MakeNode` (Xbox PDB): `TileRect::MakeNode`'s model.
pub fn tile_menu_make_node(e: &mut Engine, this: Ptr<TileMenu>) -> Ptr {
    e.call(TILE_RECT_MAKE_NODE, &args![this]).ptr()
}

// Translated from 00a1f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::PostParse` (Xbox PDB): after trait `trait_id` changed to
/// `value`:
///
/// * class: without a menu, creates one of class `(int)value`; when there
///   is still none, prints `"ERROR: Cant find menu class!"`. Otherwise
///   gives the menu this tile (`SetMenuTile(this, 0)`), stores the menu's
///   id at menu +0x20, and when the stacking type is `102` or `103` also at
///   menu +0x14 and sets the depth to `Menu::GetMaxDepth()`; records this
///   tile for id `(int)value` and returns `this`.
/// * a user trait (`>= 0x2710`) with a menu: handed to the menu's slot
///   `+0x40`.
///
/// Then (not after a class with a menu), for the visible trait with a menu
/// whose id is `0x3E9` to `0x43C`, stores in the id's byte whether `value`
/// is not `0.0` (NaN counts as not equal). Returns 0. The third stack word
/// is not read.
pub fn tile_menu_post_parse(
    e: &mut Engine,
    this: Ptr<TileMenu>,
    trait_id: i32,
    value: f32,
    _unused_3: u32,
) -> Ptr<TileMenu> {
    if trait_id == TRAIT_CLASS {
        if e.get(this, TileMenu::pMenu).is_null() {
            let class = float_to_int(e, value);
            let menu = e.call(INTERFACE_CREATE_MENU_BY_CLASS, &args![class]).ptr();
            e.set(this, TileMenu::pMenu, menu);
        }
        if e.get(this, TileMenu::pMenu).is_null() {
            e.call(INTERFACE_PRINT_ERROR, &args![NO_MENU_CLASS]);
        } else {
            let menu = e.get(this, TileMenu::pMenu);
            e.call(MENU_SET_MENU_TILE, &args![menu, this, 0u32]);
            let menu = e.get(this, TileMenu::pMenu);
            let id = menu_id(e, menu);
            e.mem.set_i32(menu.addr() + 0x20, id);
            let stacking = e
                .call(TILE_GET_FLOAT, &args![this, TRAIT_STACKING_TYPE])
                .f64();
            let gives_depth = if stacking == e.global::<f64>(STACKING_TYPE_A) {
                true
            } else {
                let stacking = e
                    .call(TILE_GET_FLOAT, &args![this, TRAIT_STACKING_TYPE])
                    .f64();
                stacking == e.global::<f64>(STACKING_TYPE_B)
            };
            if gives_depth {
                let menu = e.get(this, TileMenu::pMenu);
                let id = menu_id(e, menu);
                e.mem.set_i32(menu.addr() + 0x14, id);
                let depth = e.call(MENU_GET_MAX_DEPTH, &args![]).f32();
                e.call(TILE_SET_FLOAT, &args![this, TRAIT_DEPTH, depth, 1u32]);
            }
            let id = float_to_int(e, value);
            e.call(MENU_SET_TILE_FOR_ID, &args![id, this]);
            return this;
        }
    } else if !e.get(this, TileMenu::pMenu).is_null() && trait_id >= FIRST_USER_TRAIT {
        let menu = e.get(this, TileMenu::pMenu);
        e.vcall(menu.addr(), MENU_USER_TRAIT_SLOT, &args![trait_id]);
    }
    if !e.get(this, TileMenu::pMenu).is_null() && trait_id == TRAIT_VISIBLE {
        let shown = if f64::from(value) == e.global::<f64>(ZERO) {
            0u8
        } else {
            1u8
        };
        let menu = e.get(this, TileMenu::pMenu);
        let id = menu_id(e, menu);
        if (FIRST_MENU_ID..=LAST_MENU_ID).contains(&id) {
            e.mem.set_u8(MENU_ID_BYTES.wrapping_add(id as u32), shown);
        }
    }
    Ptr::NULL
}

/// `Tile::ForceTextureRelease` (engine map, `00a0b020`; thiscall, no stack words).
const TILE_FORCE_TEXTURE_RELEASE: u32 = 0x00a0_b020;

// Translated from 00a1f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileMenu::ForceTextureRelease` (engine map; thiscall, no stack words): runs the base
/// `Tile::ForceTextureRelease` (`00a0b020`) unless the tile has a menu whose id (virtual
/// slot `+0x34`, asked once for each test) is 0x3EC or 0x3EF.
pub fn tile_menu_force_texture_release(e: &mut Engine, this: Ptr<TileMenu>) {
    let menu = e.get(this, TileMenu::pMenu);
    if !menu.is_null() {
        let id = e.vcall(menu.addr(), MENU_ID_SLOT, &args![]).u32();
        if id == 0x3ec {
            return;
        }
        let menu = e.get(this, TileMenu::pMenu);
        let id = e.vcall(menu.addr(), MENU_ID_SLOT, &args![]).u32();
        if id == 0x3ef {
            return;
        }
    }
    e.call(TILE_FORCE_TEXTURE_RELEASE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1_f310, tile_menu_force_texture_release(Ptr<TileMenu>)),
        entry!(
            0x00a1_ef30,
            tile_menu_init(Ptr<TileMenu>, Ptr<Tile>, Ptr, u32)
        ),
        entry!(0x00a1_eff0, fn_00a1eff0(Ptr<TileMenu>)),
        entry!(0x00a1_f140, tile_menu_make_node(Ptr<TileMenu>) -> Ptr),
        entry!(
            0x00a1_f160,
            tile_menu_post_parse(Ptr<TileMenu>, i32, f32, u32) -> Ptr<TileMenu>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test vtable of the menu: id slot and user-trait slot and the deleting
    /// destructor.
    const MENU_VTABLE: u32 = 0x0003_0000;
    const MENU_GET_ID: u32 = 0x0000_1340;
    const MENU_USER_TRAIT: u32 = 0x0000_1400;
    const MENU_DELETE: u32 = 0x0000_1000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// A menu object with id `id` and the id byte table mapped.
    fn menu(e: &mut Engine, id: i32) -> Ptr {
        let mut slots = vec![0u32; 0x11];
        slots[0] = MENU_DELETE;
        slots[(MENU_ID_SLOT / 4) as usize] = MENU_GET_ID;
        slots[(MENU_USER_TRAIT_SLOT / 4) as usize] = MENU_USER_TRAIT;
        e.put_vtable(MENU_VTABLE, &slots);
        let menu = e.mem.alloc(0x40);
        e.mem.set_u32(menu, MENU_VTABLE);
        e.mem.set_i32(menu + 0x30, id);
        e.register(MENU_GET_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        quiet(e, &[MENU_DELETE, MENU_USER_TRAIT]);
        e.map(0x011f_3478, 0x54);
        Ptr::new(menu)
    }

    #[test]
    fn tile_menu_init_sets_the_default_traits() {
        let mut e = Engine::new();
        quiet(&mut e, &[TILE_INIT, TILE_SET_FLOAT, TILE_SET_NEEDS_UPDATE]);
        e.map(INITIAL_TRAIT_FC1, 4);
        e.set_global(INITIAL_TRAIT_FC1, 0.25f32);
        let this = e.new_object::<TileMenu>();
        e.set(this, TileMenu::pMenu, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x00a1_ef30, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, TILE_INIT), vec![vec![this.addr(), 1, 2, 3]]);
        let t = this.addr();
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![
                vec![t, 0xfa1, 0, 1],
                vec![t, 0xfa2, 0, 1],
                vec![t, 0xfa8, 1.0f32.to_bits(), 1],
                vec![t, 0xfc1, 0.25f32.to_bits(), 1],
                vec![t, 0xff4, 1.0f32.to_bits(), 1],
                vec![t, 0xfa3, 0, 1],
            ]
        );
        assert_eq!(calls_to(&e, TILE_SET_NEEDS_UPDATE), vec![vec![t, 3]]);
        assert!(e.get(this, TileMenu::pMenu).is_null());
    }

    #[test]
    fn fn_00a1eff0_deletes_the_menu_and_releases_twice() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MENU_SET_TILE_FOR_ID,
                MENU_SET_MENU_TILE,
                TILE_RELEASE,
                TILE_DESTRUCTOR,
            ],
        );
        let menu = menu(&mut e, 0x3ea);
        e.mem.set_u8(0x011f_3479, 1);
        let this = e.new_object::<TileMenu>();
        e.set(this, TileMenu::pMenu, menu);
        e.call_log = Some(vec![]);
        e.call(0x00a1_eff0, &args![this]);
        assert_eq!(e.mem.u8(0x011f_3479), 0);
        assert_eq!(calls_to(&e, MENU_SET_TILE_FOR_ID), vec![vec![0x3ea, 0]]);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![menu.addr(), 0, 0]]
        );
        assert_eq!(calls_to(&e, MENU_DELETE), vec![vec![menu.addr(), 1]]);
        assert_eq!(calls_to(&e, TILE_RELEASE).len(), 2);
        assert_eq!(calls_to(&e, TILE_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(e.mem.u32(this.addr()), TILE_RECT_VTABLE);
    }

    #[test]
    fn fn_00a1eff0_without_a_menu_skips_release_of_a_released_tile() {
        let mut e = Engine::new();
        quiet(&mut e, &[TILE_RELEASE, TILE_DESTRUCTOR]);
        let this = e.new_object::<TileMenu>();
        e.set(this.cast::<Tile>(), Tile::uiFlags, FLAG_RELEASED);
        e.call_log = Some(vec![]);
        e.call(0x00a1_eff0, &args![this]);
        assert!(calls_to(&e, TILE_RELEASE).is_empty());
        assert_eq!(calls_to(&e, TILE_DESTRUCTOR), vec![vec![this.addr()]]);
    }

    #[test]
    fn tile_menu_make_node_forwards_to_the_rect() {
        let mut e = Engine::new();
        e.register(TILE_RECT_MAKE_NODE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00a1_f140, &args![0x5000u32]).u32(), 0x5001);
    }

    fn post_parse_engine(stacking: f64) -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TILE_SET_FLOAT,
                MENU_SET_MENU_TILE,
                MENU_SET_TILE_FOR_ID,
                INTERFACE_PRINT_ERROR,
            ],
        );
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register_double(TILE_GET_FLOAT, move |_, _| Ret {
            st0: stacking,
            ..Ret::default()
        });
        e.register(MENU_GET_MAX_DEPTH, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        for (address, value) in [
            (STACKING_TYPE_A, 102.0f64),
            (STACKING_TYPE_B, 103.0),
            (ZERO, 0.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e
    }

    #[test]
    fn tile_menu_post_parse_class_creates_the_menu_with_depth() {
        for stacking in [102.0, 103.0] {
            let mut e = post_parse_engine(stacking);
            let menu = menu(&mut e, 0x3f0);
            e.register_double(INTERFACE_CREATE_MENU_BY_CLASS, move |_, _| Ret {
                eax: menu.addr(),
                ..Ret::default()
            });
            let this = e.new_object::<TileMenu>();
            e.call_log = Some(vec![]);
            let result = e.call(0x00a1_f160, &args![this, 0xfa4u32, 1012.0f32, 0u32]);
            assert_eq!(result.u32(), this.addr());
            assert_eq!(
                calls_to(&e, INTERFACE_CREATE_MENU_BY_CLASS),
                vec![vec![1012]]
            );
            assert_eq!(e.get(this, TileMenu::pMenu), menu);
            assert_eq!(
                calls_to(&e, MENU_SET_MENU_TILE),
                vec![vec![menu.addr(), this.addr(), 0]]
            );
            assert_eq!(e.mem.u32(menu.addr() + 0x20), 0x3f0);
            assert_eq!(e.mem.u32(menu.addr() + 0x14), 0x3f0);
            assert_eq!(
                calls_to(&e, TILE_SET_FLOAT),
                vec![vec![this.addr(), 0xfad, 7.5f32.to_bits(), 1]]
            );
            assert_eq!(
                calls_to(&e, MENU_SET_TILE_FOR_ID),
                vec![vec![1012, this.addr()]]
            );
        }
    }

    #[test]
    fn tile_menu_post_parse_class_other_stacking_and_missing_class() {
        let mut e = post_parse_engine(f64::NAN);
        let menu = menu(&mut e, 0x3f0);
        let this = e.new_object::<TileMenu>();
        e.set(this, TileMenu::pMenu, menu);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f160, &args![this, 0xfa4u32, 5.0f32, 0u32]);
        assert!(calls_to(&e, INTERFACE_CREATE_MENU_BY_CLASS).is_empty());
        assert_eq!(calls_to(&e, TILE_GET_FLOAT).len(), 2);
        assert!(calls_to(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(e.mem.u32(menu.addr() + 0x14), 0);

        let mut e = post_parse_engine(0.0);
        e.register(INTERFACE_CREATE_MENU_BY_CLASS, |_, _| Ret::default());
        let this = e.new_object::<TileMenu>();
        e.call_log = Some(vec![]);
        let result = e.call(0x00a1_f160, &args![this, 0xfa4u32, 5.0f32, 0u32]);
        assert_eq!(result.u32(), 0);
        assert_eq!(
            calls_to(&e, INTERFACE_PRINT_ERROR),
            vec![vec![NO_MENU_CLASS]]
        );
        assert!(calls_to(&e, MENU_SET_TILE_FOR_ID).is_empty());
    }

    #[test]
    fn tile_menu_post_parse_user_traits_and_visibility() {
        let mut e = post_parse_engine(0.0);
        let menu = menu(&mut e, 0x43c);
        let this = e.new_object::<TileMenu>();
        e.set(this, TileMenu::pMenu, menu);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a1_f160, &args![this, 0x2710u32, 1.0f32, 0u32])
                .u32(),
            0
        );
        assert_eq!(
            calls_to(&e, MENU_USER_TRAIT),
            vec![vec![menu.addr(), 0x2710]]
        );
        let byte = 0x011f_308f + 0x43c;
        for (value, shown) in [(1.0f32, 1u8), (0.0, 0), (f32::NAN, 1), (-0.0, 0)] {
            e.call(0x00a1_f160, &args![this, 0xfa3u32, value, 0u32]);
            assert_eq!(e.mem.u8(byte), shown);
        }
        // An id outside the table: nothing written; no menu: nothing asked.
        e.mem.set_i32(menu.addr() + 0x30, 0x43d);
        e.mem.set_u8(byte, 9);
        e.call(0x00a1_f160, &args![this, 0xfa3u32, 1.0f32, 0u32]);
        assert_eq!(e.mem.u8(byte), 9);
        e.set(this, TileMenu::pMenu, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f160, &args![this, 0x2711u32, 1.0f32, 0u32]);
        e.call(0x00a1_f160, &args![this, 0xfa3u32, 1.0f32, 0u32]);
        assert!(calls_to(&e, MENU_GET_ID).is_empty());
        assert!(calls_to(&e, MENU_USER_TRAIT).is_empty());
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn run(e: &mut Engine, this: Ptr<TileMenu>) -> (usize, usize) {
        e.call_log = Some(vec![]);
        e.call(0x00a1_f310, &args![this]);
        let log = e.call_log.take().unwrap();
        let asked = log.iter().filter(|c| c.0 == 0x0003_2000).count();
        let released = log
            .iter()
            .filter(|c| c.0 == TILE_FORCE_TEXTURE_RELEASE)
            .count();
        (asked, released)
    }

    #[test]
    fn force_texture_release_skips_two_menu_ids() {
        let mut e = Engine::new();
        e.register(TILE_FORCE_TEXTURE_RELEASE, |_, _| Ret::default());
        let menu = e.mem.alloc(0x40);
        e.put_vtable(0x0003_1000, &[0; 0x10]);
        e.mem.set_u32(0x0003_1000 + MENU_ID_SLOT, 0x0003_2000);
        e.mem.set_u32(menu, 0x0003_1000);
        let this = e.new_object::<TileMenu>();
        // No menu: released.
        assert_eq!(run(&mut e, this), (0, 1));
        e.set(this, TileMenu::pMenu, Ptr::new(menu));
        for (id, expect) in [(0x3ecu32, (1, 0)), (0x3ef, (2, 0)), (0x3ed, (2, 1))] {
            e.register_double(0x0003_2000, move |_, _| Ret {
                eax: id,
                ..Ret::default()
            });
            assert_eq!(run(&mut e, this), expect, "id {id:#x}");
        }
    }
}
