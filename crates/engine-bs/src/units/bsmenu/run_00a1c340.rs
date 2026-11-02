//! `BSMenu/run_00a1c340` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::BSSimpleArray;
use crate::units::bsmenu::menu::Menu;
use crate::units::platform::MEMORY_MANAGER;

/// `Menu`'s vtable (`01095484`; its RTTI locator names `.?AVMenu@@`).
const MENU_VTABLE: u32 = 0x0109_5484;
/// One byte per menu id from 1001 to 1084: the byte for id `n` is at
/// `011f308f + n` (cleared by the destructor).
const MENU_OPEN_BYTES: u32 = 0x011f_308f;
/// `Interface::PopFromEnterStack` (`00706fd0`, `cdecl`, two words).
const INTERFACE_POP_FROM_ENTER_STACK: u32 = 0x0070_6fd0;
/// `00a1e130` (`BSMenu/menu.cpp`, `cdecl`, two words: menu id and 0).
const MENU_ID_CLOSED_00A1E130: u32 = 0x00a1_e130;
/// `Tile::TileTemplate::~TileTemplate` (Xbox PDB, `00a0c190`, `thiscall`).
const TILE_TEMPLATE_DESTRUCTOR: u32 = 0x00a0_c190;
/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The template list's clean-up (`00ae83e0`, `thiscall` on the list at `+8`).
const TEMPLATE_LIST_CLEAR_00AE83E0: u32 = 0x00ae_83e0;
/// `Interface::GetTopMenuID` (`007023c0`, no arguments).
const INTERFACE_GET_TOP_MENU_ID: u32 = 0x0070_23c0;
/// The global object at `011dea0c` that `00877720` is called on.
const MAIN_GLOBAL_011DEA0C: u32 = 0x011d_ea0c;
/// `00877720` (`fallout/misc/main.cpp`, `thiscall`, no stack words).
const MAIN_GETTER_00877720: u32 = 0x0087_7720;
/// `00a24660` (`thiscall`, two stack words).
const QUERY_00A24660: u32 = 0x00a2_4660;

// Translated from 00a1c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::Menu` (the constructor; unnamed in the engine map): sets the `Menu` vtable, clears
/// the embedded template list node, the menu tile, the last tile, `iIsModal`,
/// `iMenuThickness` and `bRenderedMenu`, sets `xFadeState` to 4 (`FADE_HIDDEN`) and
/// `bDeleteTemplates` to 1. `iClass` (`+0x20`) is left as it is. Returns `this`.
pub fn menu_menu(e: &mut Engine, this: Ptr<Menu>) -> Ptr<Menu> {
    e.mem.set_u32(this.addr(), MENU_VTABLE);
    // xTemplates: item at +8, next at +0xC.
    e.mem.set_u32(this.addr().wrapping_add(8), 0);
    e.mem.set_u32(this.addr().wrapping_add(0xc), 0);
    e.set(this, Menu::pMenu, Ptr::NULL);
    e.set(this, Menu::pLastTile, Ptr::NULL);
    e.set(this, Menu::iIsModal, 0);
    e.set(this, Menu::xFadeState, 4);
    e.set(this, Menu::bDeleteTemplates, true);
    e.set(this, Menu::iMenuThickness, 0);
    e.set(this, Menu::bRenderedMenu, false);
    this
}

// Translated from 00a1c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Menu::~Menu` (the destructor; unnamed in the engine map). Sets the `Menu` vtable; for a
/// menu id from 1001 to 1084 clears its byte at `011f308f + id`; with a non-zero `iIsModal`
/// pops the interface's enter stack (`00706fd0(iIsModal, 1)`); calls `00a1e130(id, 0)`;
/// with a menu tile clears the tile's word at `+0x3c` and deletes the tile (virtual slot 0
/// with 1); clears `pMenu`; with `bDeleteTemplates` destroys (`00a0c190`) and frees
/// (`00aa4060`) every non-null template of the list at `+8`; clears the list (`00ae83e0`);
/// when no menu is on top (`Interface::GetTopMenuID` answers 0) and `00a24660(0xe, 0)` on
/// what `00877720` answers for the object at `011dea0c` is non-zero, sets bit 3 of the word
/// at `+0x1b92` of what `00877720` answers; then clears the list once more (the member's
/// own destructor, `00ae83e0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00a1c520 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn menu_destructor(e: &mut Engine, this: Ptr<Menu>) {
    e.mem.set_u32(this.addr(), MENU_VTABLE);
    let id = e.get(this, Menu::iClass);
    if (1001..=0x43c).contains(&id) {
        e.mem.set_u8(MENU_OPEN_BYTES.wrapping_add(id as u32), 0);
    }
    let modal = e.get(this, Menu::iIsModal);
    if modal != 0 {
        e.call(INTERFACE_POP_FROM_ENTER_STACK, &args![modal, 1u32]);
    }
    e.call(MENU_ID_CLOSED_00A1E130, &args![id, 0u32]);
    let tile = e.get(this, Menu::pMenu);
    if !tile.is_null() {
        // Tile +0x3c.
        e.mem.set_u32(tile.addr().wrapping_add(0x3c), 0);
        let tile = e.get(this, Menu::pMenu);
        if !tile.is_null() {
            e.vcall(tile.addr(), 0, &args![1u32]);
        }
    }
    e.set(this, Menu::pMenu, Ptr::NULL);
    if e.get(this, Menu::bDeleteTemplates) {
        let mut node = this.addr().wrapping_add(8);
        while node != 0 {
            let template = e.mem.u32(node);
            if template != 0 {
                e.call(TILE_TEMPLATE_DESTRUCTOR, &args![template]);
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, template]);
            }
            node = e.mem.u32(node.wrapping_add(4));
        }
    }
    let list = this.addr().wrapping_add(8);
    e.call(TEMPLATE_LIST_CLEAR_00AE83E0, &args![list]);
    if e.call(INTERFACE_GET_TOP_MENU_ID, &args![]).u32() == 0 {
        let global = e.mem.u32(MAIN_GLOBAL_011DEA0C);
        let object = e.call(MAIN_GETTER_00877720, &args![global]).u32();
        if e.call(QUERY_00A24660, &args![object, 0xeu32, 0u32]).u32() != 0 {
            let global = e.mem.u32(MAIN_GLOBAL_011DEA0C);
            let object = e.call(MAIN_GETTER_00877720, &args![global]).u32();
            let flags = e.mem.u16(object.wrapping_add(0x1b92));
            e.mem.set_u16(object.wrapping_add(0x1b92), flags | 8);
        }
    }
    e.call(TEMPLATE_LIST_CLEAR_00AE83E0, &args![list]);
}

/// `memmove` (`00ec7230`, `cdecl`, three words).
const MEMMOVE: u32 = 0x00ec_7230;
/// The size of the items `00a1c400` moves.
const ITEM_SIZE: u32 = 0x20;

// Translated from 00a1c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` item move for 0x20-byte items (no map name; `thiscall`, `this` not
/// read): moves `count` items from `source` to `dest` one at a time with `memmove`, first
/// to last when `dest` is below `source`, last to first when it is above, and nothing when
/// they are equal or `count` is 0.
pub fn fn_00a1c400(e: &mut Engine, _this: Ptr, dest: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if dest < source {
        for i in 0..count {
            let offset = i.wrapping_mul(ITEM_SIZE);
            e.call(
                MEMMOVE,
                &args![
                    dest.wrapping_add(offset),
                    source.wrapping_add(offset),
                    ITEM_SIZE
                ],
            );
        }
    } else if dest > source {
        // The index is compared signed (`JL`).
        let mut i = count.wrapping_sub(1) as i32;
        while i >= 0 {
            let offset = (i as u32).wrapping_mul(ITEM_SIZE);
            e.call(
                MEMMOVE,
                &args![
                    dest.wrapping_add(offset),
                    source.wrapping_add(offset),
                    ITEM_SIZE
                ],
            );
            i -= 1;
        }
    }
}

// Translated from 00a1c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` buffer change for 0x20-byte items (no map name; `thiscall`, two
/// words): without a buffer, allocates `reserve` items (vtable +0x04) and records
/// `iReservedSize`; when `count` equals `iReservedSize`, reallocates the buffer in place
/// (vtable +0x0C with the buffer and `reserve`); otherwise allocates a new buffer, moves the
/// first `count` items into it ([`fn_00a1c400`]), frees the old one (vtable +0x08) and
/// keeps the new one. Only the first case records the reserve.
pub fn fn_00a1c340(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, count: u32) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        let buffer = e.vcall(this.addr(), 0x04, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let old = e.get(this, BSSimpleArray::pBuffer);
        let buffer = e.vcall(this.addr(), 0x0c, &args![old, reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
    } else {
        let buffer = e.vcall(this.addr(), 0x04, &args![reserve]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        fn_00a1c400(e, this.cast(), buffer, old, count);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.vcall(this.addr(), 0x08, &args![old]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, buffer);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1_c400, fn_00a1c400(Ptr, u32, u32, u32)),
        entry!(0x00a1_c340, fn_00a1c340(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00a1_c4a0, menu_menu(Ptr<Menu>) -> Ptr<Menu>),
        entry!(0x00a1_c520, menu_destructor(Ptr<Menu>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn menu_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        for off in (0..0x28).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        assert_eq!(e.call(0x00a1_c4a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), MENU_VTABLE);
        for off in [4, 8, 0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x1c), 0);
        assert_eq!(e.mem.u8(this + 0x1d), 1);
        assert_eq!(e.mem.u32(this + 0x20), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x24), 4);
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        for addr in [
            INTERFACE_POP_FROM_ENTER_STACK,
            MENU_ID_CLOSED_00A1E130,
            TILE_TEMPLATE_DESTRUCTOR,
            MEMORY_DEALLOCATE,
            TEMPLATE_LIST_CLEAR_00AE83E0,
            0x0f00_0000,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(QUERY_00A24660, |e, _| e.mem.u32(0x011d_e100).into_ret());
        e.register(INTERFACE_GET_TOP_MENU_ID, |e, _| {
            e.mem.u32(0x011d_e104).into_ret()
        });
        e.register(MAIN_GETTER_00877720, |e, _| {
            e.mem.u32(0x011d_e108).into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0x0f00_0000]);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn menu_destructor_releases_everything() {
        let mut e = rig();
        let object = e.mem.alloc(0x2000);
        e.mem.set_u16(object + 0x1b92, 0x0101);
        e.mem.set_u32(0x011d_e108, object);
        e.mem.set_u32(MAIN_GLOBAL_011DEA0C, 0x5555);
        e.mem.set_u32(0x011d_e100, 1);
        let this = e.mem.alloc(0x28);
        let tile = e.mem.alloc(0x40);
        e.mem.set_u32(tile, 0x0f00_1000);
        e.mem.set_u32(tile + 0x3c, 9);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x7002);
        e.mem.set_u32(this + 4, tile);
        e.mem.set_u32(this + 8, 0x7001);
        e.mem.set_u32(this + 0xc, second);
        e.mem.set_u32(this + 0x14, 3);
        e.mem.set_u8(this + 0x1d, 1);
        e.mem.set_u32(this + 0x20, 1001);
        e.mem.set_u8(MENU_OPEN_BYTES + 1001, 1);
        e.call(0x00a1_c520, &args![this]);
        assert_eq!(e.mem.u32(this), MENU_VTABLE);
        assert_eq!(e.mem.u8(MENU_OPEN_BYTES + 1001), 0);
        assert_eq!(calls(&e, INTERFACE_POP_FROM_ENTER_STACK), vec![vec![3, 1]]);
        assert_eq!(calls(&e, MENU_ID_CLOSED_00A1E130), vec![vec![1001, 0]]);
        assert_eq!(e.mem.u32(tile + 0x3c), 0);
        assert_eq!(calls(&e, 0x0f00_0000), vec![vec![tile, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls(&e, TILE_TEMPLATE_DESTRUCTOR),
            vec![vec![0x7001], vec![0x7002]]
        );
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7001], vec![MEMORY_MANAGER, 0x7002]]
        );
        assert_eq!(
            calls(&e, TEMPLATE_LIST_CLEAR_00AE83E0),
            vec![vec![this + 8], vec![this + 8]]
        );
        assert_eq!(calls(&e, QUERY_00A24660), vec![vec![object, 0xe, 0]]);
        assert_eq!(
            calls(&e, MAIN_GETTER_00877720),
            vec![vec![0x5555], vec![0x5555]]
        );
        assert_eq!(e.mem.u16(object + 0x1b92), 0x0109);
    }

    #[test]
    fn menu_destructor_skips_what_is_not_there() {
        let mut e = rig();
        // A menu on top: the flag is not touched.
        e.mem.set_u32(0x011d_e104, 7);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, 0x7001);
        e.mem.set_u32(this + 0x20, 1000);
        e.mem.set_u8(MENU_OPEN_BYTES + 1000, 1);
        e.call(0x00a1_c520, &args![this]);
        assert_eq!(e.mem.u8(MENU_OPEN_BYTES + 1000), 1);
        assert!(calls(&e, INTERFACE_POP_FROM_ENTER_STACK).is_empty());
        assert!(calls(&e, 0x0f00_0000).is_empty());
        // bDeleteTemplates is 0: the templates stay.
        assert!(calls(&e, TILE_TEMPLATE_DESTRUCTOR).is_empty());
        assert!(calls(&e, QUERY_00A24660).is_empty());
        assert_eq!(calls(&e, MENU_ID_CLOSED_00A1E130), vec![vec![1000, 0]]);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn item_move_picks_the_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a1_c400, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            [vec![0x1000, 0x2000, 0x20], vec![0x1020, 0x2020, 0x20]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a1_c400, &args![0u32, 0x3000u32, 0x2000u32, 2u32]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            [vec![0x3020, 0x2020, 0x20], vec![0x3000, 0x2000, 0x20]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a1_c400, &args![0u32, 0x2000u32, 0x2000u32, 2u32]);
        e.call(0x00a1_c400, &args![0u32, 0x1000u32, 0x2000u32, 0u32]);
        assert!(calls_to(&e, MEMMOVE).is_empty());
    }

    fn array(e: &mut Engine) -> Ptr<BSSimpleArray> {
        let a = e.new_object::<BSSimpleArray>();
        e.put_vtable(0x0300_0000, &[0, 0x0300_1004, 0x0300_1008, 0x0300_100c]);
        e.mem.set_u32(a.addr(), 0x0300_0000);
        e.register(0x0300_1004, |_, _| Ret {
            eax: 0x0500_0000,
            ..Ret::default()
        });
        e.register(0x0300_1008, |_, _| Ret::default());
        e.register(0x0300_100c, |_, _| Ret {
            eax: 0x0600_0000,
            ..Ret::default()
        });
        e.register(MEMMOVE, |_, _| Ret::default());
        a
    }

    #[test]
    fn buffer_change_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        let a = array(&mut e);
        // No buffer: allocate and record the reserve.
        e.call_log = Some(vec![]);
        e.call(0x00a1_c340, &args![a, 8u32, 0u32]);
        assert_eq!(calls_to(&e, 0x0300_1004), [vec![a.addr(), 8]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0x0500_0000);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 8);
        // Count equals the reserve: reallocate, the reserve is not recorded.
        e.call_log = Some(vec![]);
        e.call(0x00a1_c340, &args![a, 16u32, 8u32]);
        assert_eq!(calls_to(&e, 0x0300_100c), [vec![a.addr(), 0x0500_0000, 16]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0x0600_0000);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 8);
        // Otherwise: a new buffer, the items moved, the old one freed.
        e.call_log = Some(vec![]);
        e.call(0x00a1_c340, &args![a, 4u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0300_1004), [vec![a.addr(), 4]]);
        assert_eq!(
            calls_to(&e, MEMMOVE),
            [vec![0x0500_0000, 0x0600_0000, 0x20]]
        );
        assert_eq!(calls_to(&e, 0x0300_1008), [vec![a.addr(), 0x0600_0000]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), 0x0500_0000);
    }
}
