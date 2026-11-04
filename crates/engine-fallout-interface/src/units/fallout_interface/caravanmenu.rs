//! `fallout/interface/menus/caravanmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVNiAVObject@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071AEC: u32 = 0x0107_1aec;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESCaravanCard@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071B00: u32 = 0x0107_1b00;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCaravanAIPackage@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01071B14: u32 = 0x0107_1b14;

// Translated from 007530a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVNiAVObject@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_007530a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071AEC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 007530f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVTESCaravanCard@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_007530f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B00);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 00753210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVCaravanAIPackage@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00753210(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B14);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `CaravanMenu::pMe` (Xbox PDB static): the menu singleton.
const CARAVAN_MENU: u32 = 0x011d_917c;
/// The menu class number of `CaravanMenu` (`GetClass`).
const CARAVAN_MENU_CLASS: u32 = 0x43b;
/// The vtable the constructor stores (`CaravanMenu`'s).
const CARAVAN_MENU_VTABLE: u32 = 0x0107_108c;
/// `CaravanMenu::iUT_CurrentValue` (user trait `_current_value`, read at
/// `010702d8`), `iUT_Value` (`_Value`, `0106ed18`) and `iUT_X` (`_x`,
/// `0106ec90`): the statics the constructor fills (Xbox PDB names, matched
/// by the trait names).
const UT_CURRENT_VALUE: u32 = 0x011d_9170;
const UT_VALUE: u32 = 0x011d_9174;
const UT_X: u32 = 0x011d_916c;
/// `CaravanMenu::pim` (Xbox PDB static name; the object whose float at
/// +0x2c `00747d00` reads).
const CARAVAN_INTERFACE: u32 = 0x011d_9180;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The global `0074a270` hands to `00877720` (whose result's
/// `Controls::ClearUserActions` it calls).
const CONTROLS_OWNER: u32 = 0x011d_ea0c;

/// `Error` (cdecl, format and words).
const ERROR_PRINT: u32 = 0x0040_fbe0;
/// The two formats `Print_WhatIsHeThinking` prints with (with and without
/// the card's name).
const THINKING_WITH_CARD_FORMAT: u32 = 0x0107_1040;
const THINKING_FORMAT: u32 = 0x0107_1004;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `_ftol2_sse`: the value in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// `InterfaceManager::GetScreenWidth` (no arguments; result in ST0).
const SCREEN_WIDTH: u32 = 0x0071_5d40;
/// `double` 2.0 (`01011590`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `BSSimpleArray`'s set-up (`this`, reserve, size): `006b3eb0`.
const SIMPLE_ARRAY_INITIALIZE: u32 = 0x006b_3eb0;
/// The element destroy step of the pointer arrays (`this`, element address,
/// count; does nothing per element): `0072ba80`.
const ARRAY_DESTROY_ELEMENTS: u32 = 0x0072_ba80;
/// The word at +8 of an object (a `BSSimpleArray`'s size): `0044ddc0`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// The address of element `index` of a `BSSimpleArray` (`this`, index):
/// `006a7ad0` (through `00877a30`, buffer + 4 * index).
const ARRAY_SLOT: u32 = 0x006a_7ad0;
/// `Tile::GetMenuByClass` (cdecl, one word).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `00470470` (`this` only): run on the player's two card lists when the
/// deck is saved.
const CARD_LIST_CLEAR_00470470: u32 = 0x0047_0470;
/// `005ae3d0` (`this`, address of a card slot): adds the card to a list.
const CARD_LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
/// `Interface::GetMessageMenuResult` (no arguments; `char`).
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// `00877720` (`this`): the `Controls` instance of the global at `011dea0c`.
const CONTROLS_INSTANCE: u32 = 0x0087_7720;
/// `Controls::ClearUserActions` (`this` only).
const CONTROLS_CLEAR_USER_ACTIONS: u32 = 0x00a2_53d0;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// The word at `*list + 8` lookup of a tile's child list (`0073ac90`,
/// `this` only: answers `*this + 8`).
const LIST_HEAD_DATA: u32 = 0x0073_ac90;
/// `006317a0` (`this`, node): the address of the node's data (node + 8).
const LIST_NODE_DATA: u32 = 0x0063_17a0;
/// `007b52d0` (`this`, node): the next node (`*node`, 0 for no node).
const LIST_NEXT_NODE: u32 = 0x007b_52d0;
/// The first word of an object (`00559450`, `this` only).
const FIRST_WORD: u32 = 0x0055_9450;
/// The word at +0xb4 of a card (`008d80e0`, `this` only).
const CARD_WORD_B4: u32 = 0x008d_80e0;
/// The word at +0xb8 of a card (`004036b0`, `this` only).
const CARD_WORD_B8: u32 = 0x0040_36b0;
/// Sets an object's local translation (`this`, x, y, z): `004bc1f0`.
const SET_LOCAL_TRANSLATE: u32 = 0x004b_c1f0;
/// Fills an update-data block on the stack (`this`, time, two bytes; answers
/// `this`): `0043d410`.
const UPDATE_DATA_INIT: u32 = 0x0043_d410;
/// `NiAVObject::Update`-like call (`this`, update data): `00a59c60`.
const OBJECT_UPDATE: u32 = 0x00a5_9c60;
/// The lift of a selected track's cards (`float` 3.0 at `01017718`).
const TRACK_RAISE: u32 = 0x0101_7718;
/// The height the track cursors are moved to when deselected (`float` at
/// `01024100`).
const CURSOR_HIDDEN_HEIGHT: u32 = 0x0102_4100;
/// `004b71d0` (no arguments; `bool`): when true `DoKey` does nothing.
const KEYS_BLOCKED_004B71D0: u32 = 0x004b_71d0;
/// `Interface::IsTopMenuID` (cdecl, one word).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;

/// Offsets in `CaravanMenu` (Xbox PDB names; the PC code uses the PDB's
/// offsets up to +0xe7c).
const TILES: u32 = 0x30;
const TRACK_GEOMETRY: u32 = 0x198;
const DECK_ARRAY: u32 = 0xa6c;
const CARD_TRACKS: u32 = 0xb44;
const CURSORS: u32 = 0xe6c;
/// The word at +0xe80 `DoKey` stores a key in (PC only; the Xbox PDB has
/// `bClickedScrollbar` there).
const PENDING_KEY: u32 = 0xe80;
/// The bytes at +0xe84, +0xe85 and +0xe86 (PC only; the constructor
/// clears the first two and sets the third, `DoDownClick` sets +0xe85).
const BYTE_E84: u32 = 0xe84;
const BYTE_E85: u32 = 0xe85;
const BYTE_E86: u32 = 0xe86;
/// Number of tiles (`NUM_TILES`, Xbox PDB enum).
const NUM_TILES: i32 = 0x33;

/// `CaravanMenu::enumFlags` (Xbox PDB enum).
const CAF_DISABLE_ALLCONTROLS: u32 = 1;
const CAF_MARKFORCLOSE: u32 = 0x20000;
/// `CaravanMenu::CaravanState::CA_CLOSING_MENU` (Xbox PDB enum).
const CA_CLOSING_MENU: i32 = 0x17;

layout! {
    /// `CaravanMenu` (Xbox PDB, 0xe84 bytes; 0xe88 on PC, whose code makes
    /// +0xe80 a word and adds bytes at +0xe84 to +0xe86).
    pub struct CaravanMenu: 0xe88 {
        /// `eActiveMenu` (Xbox PDB): 0 ante, 1 deck, 2 game, 3 results.
        0x28 eActiveMenu: i32,
        /// `eGameState` (Xbox PDB).
        0x2c eGameState: i32,
        /// `uiPrevIdleTick` (Xbox PDB).
        0xfc uiPrevIdleTick: u32,
        /// `iPlayerAnte` (Xbox PDB).
        0xadc iPlayerAnte: i32,
        /// `iNPCAnte` (Xbox PDB).
        0xae0 iNPCAnte: i32,
        /// `iTotalFunds` (Xbox PDB).
        0xae4 iTotalFunds: i32,
        /// `iCardsInDeck` (Xbox PDB).
        0xaec iCardsInDeck: u32,
        /// `iTotalCards` (Xbox PDB).
        0xaf0 iTotalCards: u32,
        /// `iJumpAmount` (Xbox PDB).
        0xaf4 iJumpAmount: u32,
        /// `iPrevValue` (Xbox PDB).
        0xaf8 iPrevValue: i32,
        /// `iNumSteps` (Xbox PDB).
        0xafc iNumSteps: i32,
        /// `iPrevXLocation` (Xbox PDB).
        0xb00 iPrevXLocation: i32,
        /// `iSelectedDeckCard` (Xbox PDB).
        0xe2c iSelectedDeckCard: i32,
        /// `iSelectedTrack` (Xbox PDB).
        0xe30 iSelectedTrack: i32,
        /// `iSelectedRow` (Xbox PDB).
        0xe34 iSelectedRow: i32,
        /// `iSelectedColumn` (Xbox PDB).
        0xe38 iSelectedColumn: i32,
        /// `iFlags` (Xbox PDB).
        0xe74 iFlags: u32,
        /// `bTutorial` (Xbox PDB).
        0xe78 bTutorial: u8,
    }
}

fn me(e: &Engine) -> Ptr<CaravanMenu> {
    Ptr::new(e.global::<u32>(CARAVAN_MENU))
}

fn word_at(e: &Engine, base: u32, offset: u32) -> u32 {
    e.mem.u32(base.wrapping_add(offset))
}

/// `pTiles[index]` of a menu.
fn tile(e: &Engine, menu: Ptr<CaravanMenu>, index: u32) -> u32 {
    word_at(e, menu.addr(), TILES + 4 * index)
}

/// The size of a `BSSimpleArray` (`0044ddc0`).
fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// The address of element `index` of a `BSSimpleArray` (`006a7ad0`).
fn array_slot(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(ARRAY_SLOT, &args![array, index]).u32()
}

/// Element `index` of a `BSSimpleArray` (the word at [`array_slot`]).
fn array_element(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = array_slot(e, array, index);
    e.mem.u32(slot)
}

/// `pCardTracks[track][row]` (seven rows of 0x10 bytes per track).
fn card_track(menu: u32, track: u32, row: u32) -> u32 {
    menu.wrapping_add(track.wrapping_mul(0x70))
        .wrapping_add(CARD_TRACKS)
        .wrapping_add(row << 4)
}

/// `pTrackGeometry[track][row]` (seven words per track).
fn track_geometry(e: &Engine, menu: u32, track: u32, row: u32) -> u32 {
    let base = menu
        .wrapping_add(track.wrapping_mul(0x1c))
        .wrapping_add(TRACK_GEOMETRY);
    e.mem.u32(base.wrapping_add(row.wrapping_mul(4)))
}

/// A float trait of a tile (`00a011b0`, result in ST0).
fn float_trait(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f64()
}

/// `00a012d0(tile, trait, value, 1)` with the float's bit pattern.
fn set_float_bits(e: &mut Engine, tile: u32, trait_id: u32, bits: u32) {
    e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, bits, 1u32]);
}

/// `004bc1f0(object, 0.0, 0.0, z)`: the object's local translation.
fn set_translate(e: &mut Engine, object: u32, z_bits: u32) {
    e.call(SET_LOCAL_TRANSLATE, &args![object, 0u32, 0u32, z_bits]);
}

/// `0043d410(block, 0.0, 0, 0)` on a stack block, then `00a59c60(object,
/// block)`.
fn update_object(e: &mut Engine, object: u32) {
    e.with_stack(0xc, |e, block| {
        let data = e
            .call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32])
            .u32();
        e.call(OBJECT_UPDATE, &args![object, data]);
    });
}

// Translated from 0073b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanAIPackage::Print_WhatIsHeThinking` (Xbox PDB): prints the
/// package with `Error` (track, row, deck card, goal, calc result,
/// priority); with a hand (`hand` non-zero) also the name of the hand's
/// card `iDeckCard` (the card's slot +0x130).
pub fn caravan_ai_package_print_what_is_he_thinking(e: &mut Engine, this: Ptr, hand: u32) {
    let package = this.addr();
    // CaravanAIPackage (Xbox PDB): iValueGoal +0, iCalcResults +4, iTrack
    // +0xc, iRow +0x10, iDeckCard +0x14, iPriority +0x18.
    if hand != 0 {
        let deck_card = word_at(e, package, 0x14);
        let card = array_element(e, hand, deck_card);
        let name = e.vcall(card, 0x130, &args![]).u32();
        let words = [
            THINKING_WITH_CARD_FORMAT,
            word_at(e, package, 0xc),
            word_at(e, package, 0x10),
            word_at(e, package, 0x14),
            word_at(e, package, 0),
            word_at(e, package, 4),
            word_at(e, package, 0x18),
            name,
        ];
        e.call(ERROR_PRINT, &words);
    } else {
        let words = [
            THINKING_FORMAT,
            word_at(e, package, 0xc),
            word_at(e, package, 0x10),
            word_at(e, package, 0x14),
            word_at(e, package, 0),
            word_at(e, package, 4),
            word_at(e, package, 0x18),
        ];
        e.call(ERROR_PRINT, &words);
    }
}

// Translated from 0073bab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears the bits `mask` of `iFlags` (+0xe74).
pub fn caravan_menu_set_flag(e: &mut Engine, this: Ptr<CaravanMenu>, mask: u32, set: u8) {
    let flags = e.get(this, CaravanMenu::iFlags);
    if set != 0 {
        e.set(this, CaravanMenu::iFlags, flags | mask);
    } else {
        e.set(this, CaravanMenu::iFlags, !mask & flags);
    }
}

// Translated from 0073bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `iFlags` (+0xe74).
pub fn caravan_menu_clear_flags(e: &mut Engine, this: Ptr<CaravanMenu>) {
    e.set(this, CaravanMenu::iFlags, 0);
}

// Translated from 0073bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::GetClass` (Xbox PDB): the class number `0x43b`.
pub fn caravan_menu_get_class(_e: &mut Engine, _this: Ptr<CaravanMenu>) -> u32 {
    CARAVAN_MENU_CLASS
}

// Translated from 0073bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::AttachTileByID` (Xbox PDB): stores `tile` as
/// `pTiles[id]` when `0 <= id < 0x33`.
pub fn caravan_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<CaravanMenu>, id: i32, tile: u32) {
    if (0..NUM_TILES).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(TILES + 4 * id as u32), tile);
    }
}

// Translated from 0073bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn caravan_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    flags: u32,
) -> Ptr<CaravanMenu> {
    caravan_menu_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0073ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::Close` (Xbox PDB name from the call graph): with a menu
/// open, sets `eGameState` to `CA_CLOSING_MENU`; when the menu of the class
/// is up and the deck menu is active, empties the player's two card lists
/// (`00470470`) and puts each card of `pDeckArray` back into the second
/// list (cards whose byte +0x98 is set) or the first (`005ae3d0`, given the
/// card's slot).
pub fn caravan_menu_close(e: &mut Engine) {
    if me(e).addr() == 0 {
        return;
    }
    let menu = me(e);
    e.set(menu, CaravanMenu::eGameState, CA_CLOSING_MENU);
    let class = fn_0073cbe0(e);
    let open = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if open == 0 || e.get(me(e), CaravanMenu::eActiveMenu) != 1 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let second = fn_0073cbc0(e, Ptr::new(player));
    let player = e.global::<u32>(PLAYER);
    let first = fn_0073cba0(e, Ptr::new(player));
    e.call(CARD_LIST_CLEAR_00470470, &args![second]);
    e.call(CARD_LIST_CLEAR_00470470, &args![first]);
    let mut index = 0u32;
    loop {
        let deck = me(e).addr() + DECK_ARRAY;
        if index >= array_size(e, deck) {
            break;
        }
        let card = array_element(e, deck, index);
        let list = if fn_0073cb80(e, Ptr::new(card)) != 0 {
            second
        } else {
            first
        };
        let deck = me(e).addr() + DECK_ARRAY;
        let slot = array_slot(e, deck, index);
        e.call(CARD_LIST_ADD_005AE3D0, &args![list, slot]);
        index = index.wrapping_add(1);
    }
}

// Translated from 0073cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x614` (called on the
/// player, `011dea3c`: the first of its card lists).
pub fn fn_0073cba0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x614)
}

// Translated from 0073cbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x618` (called on the
/// player: the second of its card lists).
pub fn fn_0073cbc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x618)
}

// Translated from 0073cbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x43b`, without an object (no `this`).
pub fn fn_0073cbe0(_e: &mut Engine) -> u32 {
    CARAVAN_MENU_CLASS
}

// Translated from 00740900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x61c`.
pub fn fn_00740900(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x61c)
}

// Translated from 00740920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x620`.
pub fn fn_00740920(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x620)
}

// Translated from 00740960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x628`.
pub fn fn_00740960(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x628)
}

// Translated from 00740e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x62c`.
pub fn fn_00740e50(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x62c)
}

// Translated from 00740e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores `value` at `this + 0x61c`.
pub fn fn_00740e70(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x61c, value);
}

// Translated from 00740e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores `value` at `this + 0x620`.
pub fn fn_00740e90(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x620, value);
}

// Translated from 00740eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores `value` at `this + 0x624`.
pub fn fn_00740eb0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x624, value);
}

// Translated from 00740ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores `value` at `this + 0x628`.
pub fn fn_00740ed0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x628, value);
}

// Translated from 00740ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores `value` at `this + 0x62c`.
pub fn fn_00740ef0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x62c, value);
}

// Translated from 00740f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in `iFlags` (+0xe74): 1 or 0.
pub fn caravan_menu_has_flag(e: &mut Engine, this: Ptr<CaravanMenu>, mask: u32) -> bool {
    e.get(this, CaravanMenu::iFlags) & mask != 0
}

// Translated from 007414c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all 0x33 tiles of `pTiles` are set.
pub fn caravan_menu_has_all_tiles(e: &mut Engine, this: Ptr<CaravanMenu>) -> bool {
    (0..NUM_TILES as u32).all(|index| tile(e, this, index) != 0)
}

// Translated from 00747d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: half the screen width (`InterfaceManager::GetScreenWidth`
/// over the `double` 2.0 at `01011590`) plus the float at `this + 0x2c`,
/// truncated (`_ftol2`).
pub fn fn_00747d00(e: &mut Engine, this: Ptr) -> i32 {
    let width = e.call(SCREEN_WIDTH, &args![]).f64();
    let half = x87_div(width, e.global::<f64>(TWO_DOUBLE));
    let value = x87_add(half, x87_m32(e.mem.u32(this.addr() + 0x2c)));
    e.call(FTOL, &args![value]).i32()
}

// Translated from 007492f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoDownClick` (vtable slot +8, `Menu::DoDownClick` in the
/// Xbox PDB): clears the byte at +0xe85; in the deck menu, idle, a press on
/// id 15 (`TI_DECK_METER`) stores the meter tile's `_current_value` trait
/// (truncated) as `iPrevValue`, sets the byte at +0xe85 and stores
/// `00747d00(pim)` as `iPrevXLocation`. The tile word is not read.
pub fn caravan_menu_do_down_click(e: &mut Engine, this: Ptr<CaravanMenu>, id: i32, _unused_2: u32) {
    e.mem.set_u8(this.addr() + BYTE_E85, 0);
    if e.get(this, CaravanMenu::eActiveMenu) != 1
        || id != 0xf
        || e.get(this, CaravanMenu::eGameState) != 0
    {
        return;
    }
    let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
    let meter = tile(e, this, 0xf);
    let value = float_trait(e, meter, trait_id);
    let value = e.call(FTOL, &args![value]).i32();
    e.set(this, CaravanMenu::iPrevValue, value);
    e.mem.set_u8(this.addr() + BYTE_E85, 1);
    let interface = e.global::<u32>(CARAVAN_INTERFACE);
    let location = fn_00747d00(e, Ptr::new(interface));
    e.set(this, CaravanMenu::iPrevXLocation, location);
}

// Translated from 00749360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoKey` (vtable slot +0x30, `Menu::DoKey` in the Xbox PDB):
/// clears the word at +0xe80; unless `004b71d0()` blocks keys, in the
/// results menu with the menu on top it closes the menu (handled);
/// otherwise maps keys to gamepad buttons (`this` slot +0x38, value 0.0):
/// `E`/`e` 0xe, `F`/`f` 0xc, `Q`/`q` 0xd,
/// `R`/`r` button 0xa through slot +0x3c, `S`/`s` 0x10, `W`/`w` 9, `A` and
/// `a` 0xb; keys `0x80000001` to `0x80000004` are kept at +0xe80 when the
/// game state is idle. Answers the button handler's result (0 otherwise).
pub fn caravan_menu_do_key(e: &mut Engine, this: Ptr<CaravanMenu>, key: i32) -> bool {
    e.mem.set_u32(this.addr() + PENDING_KEY, 0);
    if e.call(KEYS_BLOCKED_004B71D0, &args![]).bool() {
        return false;
    }
    if e.get(this, CaravanMenu::eActiveMenu) == 3 {
        let class = e.vcall(this.addr(), 0x34, &args![]).u32();
        if e.call(IS_TOP_MENU_ID, &args![class]).bool() {
            caravan_menu_close(e);
            return true;
        }
    }
    let zero = 0u32;
    let gamepad = |e: &mut Engine, slot: u32, button: u32| {
        e.vcall(this.addr(), slot, &args![button, zero]).bool()
    };
    match key {
        0x41 => gamepad(e, 0x38, 0xb),
        0x45 | 0x65 => gamepad(e, 0x38, 0xe),
        0x46 | 0x66 => gamepad(e, 0x38, 0xc),
        0x51 | 0x71 => gamepad(e, 0x38, 0xd),
        0x52 | 0x72 => gamepad(e, 0x3c, 0xa),
        0x53 | 0x73 => gamepad(e, 0x38, 0x10),
        0x57 | 0x77 => gamepad(e, 0x38, 9),
        0x61 => gamepad(e, 0x38, 0xb),
        -0x7fff_ffff..=-0x7fff_fffc => {
            if e.get(this, CaravanMenu::eGameState) == 0 {
                e.mem.set_i32(this.addr() + PENDING_KEY, key);
            }
            false
        }
        _ => false,
    }
}

// Translated from 0074a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets trait `0xfa9` to `value` on each child of the singleton's tile
/// `index` (the child list at the tile's +4, walked while the node and the
/// head's data are set). `this` is not read.
pub fn fn_0074a000(e: &mut Engine, _this: Ptr<CaravanMenu>, index: u32, value: u32) {
    let list = tile(e, me(e), index).wrapping_add(4);
    let mut node = e.call(FIRST_WORD, &args![list]).u32();
    loop {
        if node == 0 || list == 0 {
            break;
        }
        let head = e.call(LIST_HEAD_DATA, &args![list]).u32();
        if e.mem.u32(head) == 0 {
            break;
        }
        let data = e.call(LIST_NODE_DATA, &args![list, node]).u32();
        let child = e.mem.u32(data);
        set_float_bits(e, child, 0xfa9, x87_quiet_bits(value));
        node = e.call(LIST_NEXT_NODE, &args![list, node]).u32();
    }
}

// Translated from 0074a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::CancelConfirmFunc` (Xbox PDB name from the call graph,
/// cdecl): when the message menu answered 0 marks the menu for closing
/// (`CAF_MARKFORCLOSE`) and clears the user actions of the `Controls`
/// instance; otherwise clears `CAF_DISABLE_ALLCONTROLS`.
pub fn caravan_menu_cancel_confirm_func(e: &mut Engine) {
    if e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() == 0 {
        caravan_menu_set_flag(e, me(e), CAF_MARKFORCLOSE, 1);
        let owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
        e.call(CONTROLS_CLEAR_USER_ACTIONS, &args![controls]);
    } else {
        caravan_menu_set_flag(e, me(e), CAF_DISABLE_ALLCONTROLS, 0);
    }
}

// Translated from 0074ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::SelectWholeTrack` (Xbox PDB name from the call graph):
/// raises each of the track's seven card objects to the height at
/// `01017718` (local translation 0, 0, z) and updates it.
pub fn caravan_menu_select_whole_track(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32) {
    for row in 0..7 {
        let object = track_geometry(e, this.addr(), track, row);
        let raise = x87_quiet_bits(e.global::<u32>(TRACK_RAISE));
        set_translate(e, object, raise);
        let object = track_geometry(e, this.addr(), track, row);
        update_object(e, object);
    }
}

// Translated from 0074ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DeselectWholeTrack` (Xbox PDB name from the call graph):
/// moves each of the track's seven card objects back to 0, 0, 0 and
/// updates it.
pub fn caravan_menu_deselect_whole_track(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32) {
    for row in 0..7 {
        let object = track_geometry(e, this.addr(), track, row);
        set_translate(e, object, 0);
        let object = track_geometry(e, this.addr(), track, row);
        update_object(e, object);
    }
}

// Translated from 0074edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DeselectTrackCard` (Xbox PDB name from the call graph):
/// moves the two cursors (`pCursors`, +0xe6c and +0xe70) to 0, 0 and the
/// height at `01024100`, then updates both.
pub fn caravan_menu_deselect_track_card(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let hidden = x87_quiet_bits(e.global::<u32>(CURSOR_HIDDEN_HEIGHT));
    let cursor = word_at(e, this.addr(), CURSORS);
    set_translate(e, cursor, hidden);
    let hidden = x87_quiet_bits(e.global::<u32>(CURSOR_HIDDEN_HEIGHT));
    let cursor = word_at(e, this.addr(), CURSORS + 4);
    set_translate(e, cursor, hidden);
    let cursor = word_at(e, this.addr(), CURSORS);
    update_object(e, cursor);
    let cursor = word_at(e, this.addr(), CURSORS + 4);
    update_object(e, cursor);
}

// Translated from 0074f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of a track row: the word +0xb4 of its first card, doubled for
/// each later card whose word +0xb4 is 0xe (`C_KING`, Xbox PDB enum); 0 for
/// an empty row (the size compared signed).
pub fn fn_0074f600(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32, row: u32) -> i32 {
    let array = card_track(this.addr(), track, row);
    let size = array_size(e, array) as i32;
    if size <= 0 {
        return 0;
    }
    let first = array_element(e, array, 0);
    let mut value = e.call(CARD_WORD_B4, &args![first]).i32();
    let mut index = 1i32;
    while index < size {
        let card = array_element(e, array, index as u32);
        if e.call(CARD_WORD_B4, &args![card]).u32() == 0xe {
            value = value.wrapping_shl(1);
        }
        index += 1;
    }
    value
}

// Translated from 00752cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the rows of `track`, from the first while they have cards, whose
/// first card's word +0xb4 equals `value`, and answers the count times
/// `value`.
pub fn fn_00752cb0(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32, value: i32) -> i32 {
    let mut count = 0i32;
    let mut row = 0u32;
    while row < 7 {
        let array = card_track(this.addr(), track, row);
        if array_size(e, array) == 0 {
            break;
        }
        let card = array_element(e, array, 0);
        if e.call(CARD_WORD_B4, &args![card]).i32() == value {
            count += 1;
        }
        row += 1;
    }
    count.wrapping_mul(value)
}

// Translated from 00752d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sums the word +0xb4 of the first card of each row of `track` (from the
/// first while they have cards) whose word +0xb8 equals `suit`.
pub fn fn_00752d40(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32, suit: u32) -> u32 {
    let mut sum = 0u32;
    let mut row = 0u32;
    while row < 7 {
        let array = card_track(this.addr(), track, row);
        if array_size(e, array) == 0 {
            break;
        }
        let card = array_element(e, array, 0);
        if e.call(CARD_WORD_B8, &args![card]).u32() == suit {
            let card = array_element(e, array, 0);
            let value = e.call(CARD_WORD_B4, &args![card]).u32();
            sum = value.wrapping_add(sum);
        }
        row += 1;
    }
    sum
}

// Translated from 00752df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The side of a track total: -1 below 0x15 (21), 1 above 0x1a (26), 0 in
/// between. `this` is not read.
pub fn fn_00752df0(_e: &mut Engine, _this: Ptr<CaravanMenu>, value: i32) -> i32 {
    if value < 0x15 {
        -1
    } else if value > 0x1a {
        1
    } else {
        0
    }
}

// Translated from 00753070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance whose destructor is
/// `007530a0` (`NiAVObject` pointers): stores its vtable, then
/// `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_00753070(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071AEC);
    e.call(SIMPLE_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 007530c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance whose destructor is
/// `007530f0` (`TESCaravanCard` pointers): stores its vtable, then
/// `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_007530c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B00);
    e.call(SIMPLE_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00753110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance whose destructor is
/// `00753210` (`CaravanAIPackage` pointers): stores its vtable, then
/// `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_00753110(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01071B14);
    e.call(SIMPLE_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00753140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the last element of a pointer `BSSimpleArray`: when the size
/// (+8) is not 0, runs the element destroy step on the last slot
/// (`0072ba80(this, slot, 1)`) and lowers the size.
pub fn fn_00753140(e: &mut Engine, this: Ptr) {
    let size = e.mem.u32(this.addr() + 8);
    if size == 0 {
        return;
    }
    let buffer = e.mem.u32(this.addr() + 4);
    let last = buffer.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4);
    e.call(ARRAY_DESTROY_ELEMENTS, &args![this, last, 1u32]);
    let size = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(this.addr() + 8, size.wrapping_sub(1));
}

// Translated from 00753180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiAVObject *,1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor `007530a0`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00753180(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007530a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007531b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESCaravanCard *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `007530f0`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_007531b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007530f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007531e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CaravanAIPackage *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `00753210`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_007531e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00753210(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// `005ae380` (`this` only): the number of entries of a card list.
const CARD_LIST_COUNT: u32 = 0x005a_e380;
/// Returns `this` (`006815c0`): a list node's data is its first word.
const IDENTITY: u32 = 0x0068_15c0;
/// Stores a byte at +0x98 of a card (`this`, byte): `006e2b50`.
const CARD_SET_BYTE_98: u32 = 0x006e_2b50;
/// `BSSimpleArray::AddUninitialized`-like append (`this`, address of the
/// element): `007cb2e0`.
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// The word at +4 of an object (`00726070`): a list node's next node.
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// Sorts a `BSSimpleArray` with `qsort` (`this`, compare function):
/// `00729970`.
const ARRAY_SORT: u32 = 0x0072_9970;
/// The compare function the deck is sorted with.
const DECK_COMPARE: u32 = 0x0059_ae10;
/// Sets a tile's float trait twice (`Tile::Poke`: `this`, trait, value).
const TILE_POKE: u32 = 0x00a0_1290;
/// Sets an integer trait of a tile (`this`, trait, value).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `ActorValueOwner::GetClampedActorFloatValue` (`this`, actor value;
/// result in ST0).
const CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef50;
/// `double` 100.0 (`01017a40`).
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// The smaller of two integers (cdecl).
const MIN_OF: u32 = 0x004a_8f20;
/// The larger of two integers (cdecl).
const MAX_OF: u32 = 0x0064_7b70;
/// The data handler (`TESDataHandler *` global, `011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `0046f310` (`this` = data handler, actor): fills the barter container
/// with the actor's items.
const FILL_BARTER_CONTAINER: u32 = 0x0046_f310;
/// `InventoryChanges::GetGoldAmount` (`this` only).
const GET_GOLD_AMOUNT: u32 = 0x004c_b320;
/// The byte at +4 of a form (its type; `00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESValueForm::GetFormValue` (cdecl, one word).
const GET_FORM_VALUE: u32 = 0x0048_e8a0;
/// `NiObject::Clone_ov2` (`this` only).
const OBJECT_CLONE: u32 = 0x00a5_d680;
/// A random integer in `[low, high)` (cdecl: low, high): `00944460`.
const RANDOM_RANGE: u32 = 0x0094_4460;
/// `NiObjectNET::GetController` (`this`, RTTI).
const GET_CONTROLLER: u32 = 0x00a5_c570;
/// `NiControllerManager`'s RTTI record (`011f36ac`).
const CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
/// Makes an `NiFixedString` in a stack slot (`this` = slot, text): `00438170`.
const FIXED_STRING_INIT: u32 = 0x0043_8170;
/// Releases an `NiFixedString` stack slot (`this` = slot): `004381b0`.
const FIXED_STRING_RELEASE: u32 = 0x0043_81b0;
/// The controller manager's sequence by name (`this`, name slot): `0047a520`.
const GET_SEQUENCE: u32 = 0x0047_a520;
/// Activates a sequence (`this` = manager, sequence, 0, 0, 1.0, 0.0, 0):
/// `0047aab0`.
const ACTIVATE_SEQUENCE: u32 = 0x0047_aab0;
/// `NiTimeController::StartAnimations` (cdecl, one word).
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// The node pointer the coins and bills are attached to (`011d91e4`, read
/// through `00559450`).
const MONEY_ROOT: u32 = 0x011d_91e4;
/// The sequence names a coin or bill is animated with.
const TOP_PLACE: u32 = 0x0107_1adc;
const BOTTOM_PLACE: u32 = 0x0107_1acc;
const ALT_PLACE: u32 = 0x0107_1ac0;
/// The word at +0xb4 of a card, as an address (`006ab360`: `this + 0xb4`).
const CARD_VALUE_ADDRESS: u32 = 0x006a_b360;
/// The float trait values `UpdateTrackValue` gives a total's tile (`float`
/// 126.0 at `01071abc`, 255.0 at `01023cd8`).
const TRACK_LOW_ALPHA: u32 = 0x0107_1abc;
const TRACK_ALPHA: u32 = 0x0102_3cd8;
/// The local translation of an object (`0043c490`: `this + 0x58`).
const LOCAL_TRANSLATE: u32 = 0x0043_c490;
/// `double` 0.0 (`01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The bounded `sprintf` (cdecl: buffer, size, format, ...).
const PRINT_BOUNDED: u32 = 0x0040_6d00;
/// A child object by name (cdecl: root, name): `004aae30`.
const OBJECT_BY_NAME: u32 = 0x004a_ae30;
/// The names of the selected hand card and its shadow
/// (`Player-Deck_0%i:%i`, `Player-Deck_0%i_Shadow`).
const HAND_CARD_FORMAT: u32 = 0x0107_1a58;
const HAND_SHADOW_FORMAT: u32 = 0x0107_1a24;

/// More `CaravanMenu` offsets (Xbox PDB names).
const BET_PERCENTAGE: u32 = 0x10c;
const NPC_TOTAL_FUNDS: u32 = 0x110;
const CHALLENGER: u32 = 0x100;
const COIN_GEOMETRY: u32 = 0x11c;
const BILL_GEOMETRY: u32 = 0x128;
const LIST_OF_COINS: u32 = 0x134;
const LIST_OF_COIN_ANIMATIONS: u32 = 0x144;
const LIST_OF_BILLS: u32 = 0x14c;
const LIST_OF_BILL_ANIMATIONS: u32 = 0x15c;
const HAND_GEOMETRY: u32 = 0x190;
const PLAYER_DECK: u32 = 0xb04;
const NPC_DECK: u32 = 0xb14;
const PLAYER_HAND: u32 = 0xb24;
const NPC_HAND: u32 = 0xb34;
const TRACK_INFO: u32 = 0xde4;
const SELECTED_CARD: u32 = 0xe48;
const SELECTED_OBJECT: u32 = 0xe4c;
const SELECTED_SHADOW: u32 = 0xe50;

/// `CaravanMenu::enumFlags` (Xbox PDB enum).
const CAF_DISABLE_A: u32 = 4;
const CAF_PLAYERWIN: u32 = 0x400;
const CAF_TRACK_SETUP: u32 = 0x800;

/// `trackInfo[track]` fields (`CaravanMenu::TrackInfo`, Xbox PDB):
/// `eDirection` +0, `eCardSuit` +4, `iValue` +8.
fn track_info(menu: u32, track: u32) -> u32 {
    menu.wrapping_add(TRACK_INFO)
        .wrapping_add(track.wrapping_mul(0xc))
}

/// The word +0xb4 of element `index` of a card array (`008d80e0`).
fn card_b4(e: &mut Engine, array: u32, index: u32) -> u32 {
    let card = array_element(e, array, index);
    e.call(CARD_WORD_B4, &args![card]).u32()
}

/// The word +0xb8 of element `index` of a card array (`004036b0`).
fn card_b8(e: &mut Engine, array: u32, index: u32) -> u32 {
    let card = array_element(e, array, index);
    e.call(CARD_WORD_B8, &args![card]).u32()
}

/// `MIN_OF(MAX_OF(low, value), high)`, as the exe calls them.
fn clamp_calls(e: &mut Engine, low: i32, value: i32, high: i32) -> i32 {
    let raised = e.call(MAX_OF, &args![low, value]).i32();
    e.call(MIN_OF, &args![raised, high]).i32()
}

// Translated from 0073ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareCaravanData` (Xbox PDB name from the call graph):
/// counts the player's two card lists (`iCardsInDeck` the second's count,
/// `iTotalCards` both), appends every card of the first list (byte +0x98
/// cleared) and of the second (byte set) to `pDeckArray`, sorts it with
/// `0059ae10` and sets `iJumpAmount` to a tenth of the total cards, at
/// least 10. Answers 1.
pub fn caravan_menu_prepare_caravan_data(e: &mut Engine, this: Ptr<CaravanMenu>) -> bool {
    let player = e.global::<u32>(PLAYER);
    let first = fn_0073cba0(e, Ptr::new(player));
    let player = e.global::<u32>(PLAYER);
    let second = fn_0073cbc0(e, Ptr::new(player));
    let count = e.call(CARD_LIST_COUNT, &args![second]).u32();
    e.set(this, CaravanMenu::iCardsInDeck, count);
    let in_first = e.call(CARD_LIST_COUNT, &args![first]).u32();
    let in_second = e.call(CARD_LIST_COUNT, &args![second]).u32();
    e.set(
        this,
        CaravanMenu::iTotalCards,
        in_first.wrapping_add(in_second),
    );
    for (list, in_second_list) in [(first, 0u32), (second, 1)] {
        let mut node = list;
        while node != 0 {
            let data = e.call(IDENTITY, &args![node]).u32();
            if e.mem.u32(data) == 0 {
                break;
            }
            let data = e.call(IDENTITY, &args![node]).u32();
            let card = e.mem.u32(data);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), card);
                e.call(CARD_SET_BYTE_98, &args![card, in_second_list]);
                e.call(ARRAY_APPEND, &args![this.addr() + DECK_ARRAY, slot]);
            });
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
    }
    e.call(ARRAY_SORT, &args![this.addr() + DECK_ARRAY, DECK_COMPARE]);
    let total = e.get(this, CaravanMenu::iTotalCards);
    let jump = if total / 10 > 10 { total / 10 } else { 10 };
    e.set(this, CaravanMenu::iJumpAmount, jump);
    true
}

/// The gold of an inventory-change list plus the value times the count of
/// each entry of form type 0x74 with a positive count (one of the two
/// halves of [`caravan_menu_prepare_betting_currency`]).
fn add_money(e: &mut Engine, this: Ptr<CaravanMenu>, actor: u32, total: u32) {
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(FILL_BARTER_CONTAINER, &args![handler, actor]);
    let handler = e.global::<u32>(DATA_HANDLER);
    let changes = fn_00740960(e, Ptr::new(handler));
    let gold = e.call(GET_GOLD_AMOUNT, &args![changes]).u32();
    e.mem.set_u32(this.addr() + total, gold);
    let mut node = e.mem.u32(changes);
    while node != 0 {
        let data = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(data) == 0 {
            break;
        }
        let data = e.call(IDENTITY, &args![node]).u32();
        let entry = e.mem.u32(data);
        let form = e.call(ARRAY_SIZE, &args![entry]).u32();
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == 0x74 {
            let data = e.call(IDENTITY, &args![node]).u32();
            let entry = e.mem.u32(data);
            if e.call(WORD_AT_FOUR, &args![entry]).i32() > 0 {
                let data = e.call(IDENTITY, &args![node]).u32();
                let entry = e.mem.u32(data);
                let count = e.call(WORD_AT_FOUR, &args![entry]).u32();
                let value = e.call(GET_FORM_VALUE, &args![form]).u32();
                let sum = value
                    .wrapping_mul(count)
                    .wrapping_add(e.mem.u32(this.addr() + total));
                e.mem.set_u32(this.addr() + total, sum);
            }
        }
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }
}

// Translated from 0073d650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareBettingCurrency` (Xbox PDB name from the call
/// graph): `iTotalFunds` becomes the player's money and `iNPCTotalFunds`
/// the challenger's: each actor's items are put into the data handler's
/// barter container (`0046f310`), whose inventory changes
/// (`00740960(handler)`) give the gold plus, for each entry of form type
/// 0x74 with a positive count, the form's value times the count.
pub fn caravan_menu_prepare_betting_currency(e: &mut Engine, this: Ptr<CaravanMenu>) {
    e.set(this, CaravanMenu::iTotalFunds, 0);
    let player = e.global::<u32>(PLAYER);
    add_money(e, this, player, 0xae4);
    e.mem.set_u32(this.addr() + NPC_TOTAL_FUNDS, 0);
    let challenger = word_at(e, this.addr(), CHALLENGER);
    add_money(e, this, challenger, NPC_TOTAL_FUNDS);
}

// Translated from 00749560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoClick` (vtable slot +0xc, Xbox PDB): when idle and the
/// controls are not all disabled (`CAF_DISABLE_ALLCONTROLS`), a click on a
/// button tile acts as its gamepad button (`this` slot +0x38, value 0.0):
/// ids 7 and 14 button 0xc, 10 0x10, 12, 19 and 37 9, 13 and 36 0xb, 20
/// 0xd, 21 0xe; ids 22 and 38 go to slot +0x3c (gamepad up) with 0xa.
/// The tile word is not read.
pub fn caravan_menu_do_click(e: &mut Engine, this: Ptr<CaravanMenu>, id: i32, _unused_2: u32) {
    if e.get(this, CaravanMenu::eGameState) != 0
        || caravan_menu_has_flag(e, this, CAF_DISABLE_ALLCONTROLS)
    {
        return;
    }
    let (slot, button) = match id {
        7 | 14 => (0x38, 0xcu32),
        10 => (0x38, 0x10),
        12 | 19 | 37 => (0x38, 9),
        13 | 36 => (0x38, 0xb),
        20 => (0x38, 0xd),
        21 => (0x38, 0xe),
        22 | 38 => (0x3c, 0xa),
        _ => return,
    };
    e.vcall(this.addr(), slot, &args![button, 0u32]);
}

/// The body shared by `LoadNewBill` (`0074f9b0`) and `0074fc00`: clones
/// geometry `min(kind, 2)` of the table at `geometry`, picks a random
/// placement in `[0, choices)`, plays the clone's controller-manager
/// sequence of that placement's name (when it has a manager) and starts its
/// animations, appends the clone to the array at `list` and the sequence
/// to the list at `animations`, and attaches the clone to the money root
/// (slot +0xdc, flag 1).
///
/// When no sequence was looked up (no manager, or a placement without a
/// name) the exe hands the list the address of its uninitialised stack
/// slot; the model's slot holds 0 there.
fn load_new_money(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    kind: i32,
    geometry: u32,
    names: &[u32],
    list: u32,
    animations: u32,
) {
    let kind = e.call(MIN_OF, &args![kind, 2i32]).u32();
    let source = word_at(e, this.addr(), geometry.wrapping_add(kind.wrapping_mul(4)));
    let clone = e.call(OBJECT_CLONE, &args![source]).u32();
    let choice = e.call(RANDOM_RANGE, &args![0u32, names.len() as u32]).u32();
    let manager = e
        .call(GET_CONTROLLER, &args![clone, CONTROLLER_MANAGER_RTTI])
        .u32();
    e.with_stack(8, |e, slots| {
        let clone_slot = slots.addr();
        let sequence_slot = slots.addr() + 4;
        e.mem.set_u32(clone_slot, clone);
        if let Some(&name) = names.get(choice as usize) {
            if manager != 0 {
                let sequence = e.with_stack(4, |e, fixed| {
                    let name_slot = e.call(FIXED_STRING_INIT, &args![fixed, name]).u32();
                    let sequence = e.call(GET_SEQUENCE, &args![manager, name_slot]).u32();
                    e.call(FIXED_STRING_RELEASE, &args![fixed]);
                    sequence
                });
                e.mem.set_u32(sequence_slot, sequence);
                let one = 1.0f32.to_bits();
                e.call(
                    ACTIVATE_SEQUENCE,
                    &args![manager, sequence, 0u32, 0u32, one, 0u32, 0u32],
                );
                e.call(START_ANIMATIONS, &args![clone]);
            }
        }
        e.call(ARRAY_APPEND, &args![this.addr() + list, clone_slot]);
        let animation_list = word_at(e, this.addr(), animations);
        e.call(
            CARD_LIST_ADD_005AE3D0,
            &args![animation_list, sequence_slot],
        );
    });
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    e.vcall(root, 0xdc, &args![clone, 1u32]);
}

// Translated from 0074f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::LoadNewBill` (Xbox PDB name from the call graph): a new
/// bill from `pBillGeometry[min(kind, 2)]`, placed with one of
/// `Top_Place`, `Bottom_Place`, `Alt_Place` at random, added to
/// `pListOfBills` and `pListOfBillAnimations` (see [`load_new_money`]).
// Translation incomplete: 0074f9b0 - the C++ exception-unwinding frame (FS:[0]) that releases the name string is not translated
pub fn caravan_menu_load_new_bill(e: &mut Engine, this: Ptr<CaravanMenu>, kind: i32) {
    load_new_money(
        e,
        this,
        kind,
        BILL_GEOMETRY,
        &[TOP_PLACE, BOTTOM_PLACE, ALT_PLACE],
        LIST_OF_BILLS,
        LIST_OF_BILL_ANIMATIONS,
    );
}

// Translated from 0074fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The coin counterpart of `LoadNewBill`: a new coin from
/// `pCoinGeometry[min(kind, 2)]`, placed with `Top_Place` or
/// `Bottom_Place` at random, added to `pListOfCoins` and
/// `pListOfCoinAnimations` (see [`load_new_money`]).
// Translation incomplete: 0074fc00 - the C++ exception-unwinding frame (FS:[0]) that releases the name string is not translated
pub fn fn_0074fc00(e: &mut Engine, this: Ptr<CaravanMenu>, kind: i32) {
    load_new_money(
        e,
        this,
        kind,
        COIN_GEOMETRY,
        &[TOP_PLACE, BOTTOM_PLACE],
        LIST_OF_COINS,
        LIST_OF_COIN_ANIMATIONS,
    );
}

// Translated from 0074ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::UpdateTrackValue` (Xbox PDB name from the call graph):
/// sums the track's rows (each row the value of its first card, doubled
/// per later king, `0xe`) into `trackInfo[track].iValue` and sets the
/// track's direction and suit from its rows: none (suit 5, direction 0);
/// one (the first card's suit, or the last queen's, direction 0); more
/// (rising (1) or falling (-1) between the last two rows, the last row's
/// suit, each queen on it taking its suit and reversing the direction).
/// Then shows the total on the track's tile (`TI_GAME_PLAYER_T1` +
/// track): its value, trait `0xff4` (`0xff4` below 21, then 2 above 26, else
/// 1) and alpha trait `0xfa9` (126.0 below 21, then 255.0).
pub fn caravan_menu_update_track_value(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32) {
    let menu = this.addr();
    let mut total = 0u32;
    let mut rows = 0u32;
    while rows < 7 {
        let mut value = 0u32;
        let array = card_track(menu, track, rows);
        if array_size(e, array) == 0 {
            break;
        }
        let mut index = 0u32;
        while index < array_size(e, array) {
            let card = array_element(e, array, index);
            let address = e.call(CARD_VALUE_ADDRESS, &args![card]).u32();
            if index == 0 {
                value = e.mem.u32(address);
            } else if e.mem.u32(address) == 0xe {
                value <<= 1;
            }
            index += 1;
        }
        total = total.wrapping_add(value);
        rows += 1;
    }
    let info = track_info(menu, track);
    e.mem.set_u32(info + 8, total);
    match rows {
        0 => {
            e.mem.set_u32(info + 4, 5);
            e.mem.set_u32(info, 0);
        }
        1 => {
            let first = card_track(menu, track, 0);
            let suit = card_b8(e, first, 0);
            e.mem.set_u32(info + 4, suit);
            let last = card_track(menu, track, rows - 1);
            let mut index = 1u32;
            while index < array_size(e, last) {
                if card_b4(e, last, index) == 0xd {
                    let suit = card_b8(e, last, index);
                    e.mem.set_u32(info + 4, suit);
                }
                index += 1;
            }
            e.mem.set_u32(info, 0);
        }
        _ => {
            let last = card_track(menu, track, rows - 1);
            let before = card_track(menu, track, rows - 2);
            let last_value = card_b4(e, last, 0) as i32;
            let before_value = card_b4(e, before, 0) as i32;
            let direction = if last_value > before_value {
                1u32
            } else {
                u32::MAX
            };
            e.mem.set_u32(info, direction);
            let suit = card_b8(e, last, 0);
            e.mem.set_u32(info + 4, suit);
            let mut index = 1u32;
            while index < array_size(e, last) {
                if card_b4(e, last, index) == 0xd {
                    let suit = card_b8(e, last, index);
                    e.mem.set_u32(info + 4, suit);
                    let direction = e.mem.u32(info);
                    e.mem.set_u32(info, direction.wrapping_neg());
                }
                index += 1;
            }
        }
    }
    let value_trait = e.global::<u32>(UT_VALUE);
    let total_tile = tile(e, this, 0x17 + track);
    e.call(TILE_POKE, &args![total_tile, value_trait, 1.0f32.to_bits()]);
    let value_trait = e.global::<u32>(UT_VALUE);
    let value = e.mem.u32(info + 8);
    let total_tile = tile(e, this, 0x17 + track);
    e.call(SET_TRAIT_FROM_INT, &args![total_tile, value_trait, value]);
    if (e.mem.u32(info + 8) as i32) < 0x15 {
        let total_tile = tile(e, this, 0x17 + track);
        e.call(SET_TRAIT_FROM_INT, &args![total_tile, 0xff4u32, 0xff4u32]);
        let alpha = x87_quiet_bits(e.global::<u32>(TRACK_LOW_ALPHA));
        let total_tile = tile(e, this, 0x17 + track);
        set_float_bits(e, total_tile, 0xfa9, alpha);
    }
    let state = if (e.mem.u32(info + 8) as i32) > 0x1a {
        2u32
    } else {
        1
    };
    let total_tile = tile(e, this, 0x17 + track);
    e.call(SET_TRAIT_FROM_INT, &args![total_tile, 0xff4u32, state]);
    let alpha = x87_quiet_bits(e.global::<u32>(TRACK_ALPHA));
    let total_tile = tile(e, this, 0x17 + track);
    set_float_bits(e, total_tile, 0xfa9, alpha);
}

// Translated from 0074f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::HandleTrackSelection` (Xbox PDB name from the call graph):
/// moves the track selection by `track_step` (the player's tracks 0 to 2
/// count up, the opponent's 3 to 5 down, each clamped to its side) and,
/// when the row did not change otherwise, the row by `row_step` (the same
/// way): above the top row (-1) it crosses to the facing track (5 - the
/// old track) at row 0, past row 6 it stays at 6. Both steps 0 reset track,
/// row and column. Then the column becomes the row's card count, the card
/// is selected (`SelectTrackCard`), `CAF_DISABLE_A` is cleared when the
/// selected hand card may be placed there (set otherwise) and the tile
/// alphas are updated. The hand card's value is read (and compared with
/// 10) but not used.
#[allow(clippy::if_same_then_else)] // both sides of the exe's crossing test store 5 - track
pub fn caravan_menu_handle_track_selection(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    track_step: i32,
    row_step: i32,
) {
    let old_track = e.get(this, CaravanMenu::iSelectedTrack);
    let old_row = e.get(this, CaravanMenu::iSelectedRow);
    let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard) as u32;
    card_b4(e, this.addr() + PLAYER_HAND, deck_card);
    if track_step != 0 {
        let track = e.get(this, CaravanMenu::iSelectedTrack);
        let track = if old_track < 3 {
            let track = track.wrapping_add(track_step);
            e.set(this, CaravanMenu::iSelectedTrack, track);
            clamp_calls(e, 0, track, 2)
        } else {
            let track = track.wrapping_sub(track_step);
            e.set(this, CaravanMenu::iSelectedTrack, track);
            clamp_calls(e, 3, track, 5)
        };
        e.set(this, CaravanMenu::iSelectedTrack, track);
    }
    if row_step != 0 && old_row == e.get(this, CaravanMenu::iSelectedRow) {
        let row = e.get(this, CaravanMenu::iSelectedRow);
        let row = if e.get(this, CaravanMenu::iSelectedTrack) < 3 {
            row.wrapping_add(row_step)
        } else {
            row.wrapping_sub(row_step)
        };
        e.set(this, CaravanMenu::iSelectedRow, row);
        if row == -1 {
            if e.get(this, CaravanMenu::iSelectedTrack) < 3 {
                e.set(
                    this,
                    CaravanMenu::iSelectedTrack,
                    5i32.wrapping_sub(old_track),
                );
            } else {
                e.set(
                    this,
                    CaravanMenu::iSelectedTrack,
                    5i32.wrapping_sub(old_track),
                );
            }
            e.set(this, CaravanMenu::iSelectedRow, 0);
        } else if row == 7 {
            e.set(this, CaravanMenu::iSelectedRow, 6);
        }
    }
    if track_step == row_step && track_step == 0 {
        e.set(this, CaravanMenu::iSelectedTrack, 0);
        e.set(this, CaravanMenu::iSelectedRow, 0);
        e.set(this, CaravanMenu::iSelectedColumn, 0);
    }
    let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
    let row = e.get(this, CaravanMenu::iSelectedRow) as u32;
    let count = array_size(e, card_track(this.addr(), track, row));
    e.set(this, CaravanMenu::iSelectedColumn, count as i32);
    let track = e.get(this, CaravanMenu::iSelectedTrack);
    let row = e.get(this, CaravanMenu::iSelectedRow);
    caravan_menu_select_track_card(e, this, track, row);
    let track = e.get(this, CaravanMenu::iSelectedTrack);
    let row = e.get(this, CaravanMenu::iSelectedRow);
    let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard);
    let valid = caravan_menu_is_valid_card_placement(e, this, track, row, deck_card, 0);
    caravan_menu_set_flag(e, this, CAF_DISABLE_A, u8::from(!valid));
    caravan_menu_update_tile_alpha(e, this, -1);
}

// Translated from 0074f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::IsValidCardPlacement` (Xbox PDB name from the call graph):
/// whether hand card `card` (the opponent's hand when `npc`) may go on row
/// `row` of track `track`. Out of range (track 0..5, row 0..6, card
/// non-negative and within the hand, compared unsigned): no. While setting
/// up the tracks (`CAF_TRACK_SETUP`): only a number card (value 10 or less)
/// on an empty row. A face card (above 10) goes on a row with 1 to 3 cards,
/// but a queen (`0xd`) not when the next row (below row 6) has cards. A
/// number card goes on an empty row of the own side (the opponent's tracks
/// 3 to 5); on row 0 always, else after a non-empty row whose first card
/// has another value, and keeping the track's direction (rising: above the
/// last; falling: below it) unless the suit is the track's.
pub fn caravan_menu_is_valid_card_placement(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    track: i32,
    row: i32,
    card: i32,
    npc: u8,
) -> bool {
    if !(0..=5).contains(&track) || !(0..=6).contains(&row) || card < 0 {
        return false;
    }
    let menu = this.addr();
    let hand = menu + if npc != 0 { NPC_HAND } else { PLAYER_HAND };
    let last = array_size(e, hand).wrapping_sub(1);
    if card as u32 > last {
        return false;
    }
    let value = card_b4(e, hand, card as u32) as i32;
    let suit = card_b8(e, hand, card as u32);
    let (track, row) = (track as u32, row as u32);
    if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
        return array_size(e, card_track(menu, track, row)) == 0 && value <= 10;
    }
    if value > 10 {
        let count = array_size(e, card_track(menu, track, row)) as i32;
        if !(1..=3).contains(&count) {
            return false;
        }
        if row < 6 && array_size(e, card_track(menu, track, row + 1)) != 0 && value == 0xd {
            return false;
        }
        return true;
    }
    if array_size(e, card_track(menu, track, row)) != 0 {
        return false;
    }
    if npc != 0 && track < 3 {
        return false;
    }
    if npc == 0 && track > 2 {
        return false;
    }
    if row == 0 {
        return true;
    }
    let previous_row = card_track(menu, track, row - 1);
    if array_size(e, previous_row) == 0 {
        return false;
    }
    let previous = card_b4(e, previous_row, 0) as i32;
    if previous == value {
        return false;
    }
    let info = track_info(menu, track);
    let direction = e.mem.i32(info);
    let track_suit = e.mem.u32(info + 4);
    if direction == 1 && previous >= value && track_suit != suit {
        return false;
    }
    let direction = e.mem.i32(info);
    let track_suit = e.mem.u32(info + 4);
    !(direction == -1 && previous <= value && track_suit != suit)
}

// Translated from 00752e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::IsGameOver` (Xbox PDB name from the call graph): updates
/// the six track values, then compares each player track (0 to 2) with the
/// facing one (5 - track): a side sold (21 to 26) wins the pair when the
/// other is not, or both sold and its total is higher. With all three
/// pairs decided, more pairs won ends the game: the player's win sets
/// `CAF_PLAYERWIN`, the opponent's clears it. Otherwise the game ends when
/// a side has no cards left in deck and hand (the player out: lost, the
/// opponent out: won). Answers whether the game is over.
pub fn caravan_menu_is_game_over(e: &mut Engine, this: Ptr<CaravanMenu>) -> bool {
    let menu = this.addr();
    for track in 0..6 {
        caravan_menu_update_track_value(e, this, track);
    }
    let mut player_pairs = 0i32;
    let mut decided = 0i32;
    let mut npc_pairs = 0i32;
    let mut player_wins = false;
    let mut npc_wins = false;
    for track in 0..3u32 {
        let value = |e: &Engine, track: u32| e.mem.i32(track_info(menu, track) + 8);
        let own = value(e, track);
        if fn_00752df0(e, this, own) == 0 {
            let facing = value(e, 5 - track);
            if fn_00752df0(e, this, facing) != 0 || value(e, track) > value(e, 5 - track) {
                player_pairs += 1;
                decided += 1;
            }
        }
        let facing = value(e, 5 - track);
        if fn_00752df0(e, this, facing) == 0 {
            let own = value(e, track);
            if fn_00752df0(e, this, own) != 0 || value(e, track) < value(e, 5 - track) {
                npc_pairs += 1;
                decided += 1;
            }
        }
        if player_pairs > npc_pairs && decided == 3 {
            player_wins = true;
        }
        if player_pairs < npc_pairs && decided == 3 {
            npc_wins = true;
        }
    }
    if player_wins {
        caravan_menu_set_flag(e, this, CAF_PLAYERWIN, 1);
        return true;
    }
    if npc_wins {
        caravan_menu_set_flag(e, this, CAF_PLAYERWIN, 0);
        return true;
    }
    if array_size(e, menu + PLAYER_DECK) == 0 && array_size(e, menu + PLAYER_HAND) == 0 {
        caravan_menu_set_flag(e, this, CAF_PLAYERWIN, 0);
        return true;
    }
    if array_size(e, menu + NPC_DECK) == 0 && array_size(e, menu + NPC_HAND) == 0 {
        caravan_menu_set_flag(e, this, CAF_PLAYERWIN, 1);
        return true;
    }
    false
}

/// `004bc1f0(object, 0.0, y, z)` with `y` the float at +4 of the selected
/// object's local translation (`0043c490`), then the object's update.
fn place_at_selected_height(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    object_offset: u32,
    z_bits: u32,
) {
    let selected = word_at(e, this.addr(), SELECTED_OBJECT);
    let translate = e.call(LOCAL_TRANSLATE, &args![selected]).u32();
    let y = x87_quiet_bits(e.mem.u32(translate + 4));
    let object = word_at(e, this.addr(), object_offset);
    e.call(SET_LOCAL_TRANSLATE, &args![object, 0u32, y, z_bits]);
    let object = word_at(e, this.addr(), object_offset);
    update_object(e, object);
}

// Translated from 0074e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::SelectDeckCard` (Xbox PDB name from the call graph): puts
/// a raised selected hand card (its translation's z above 0.0) and its
/// shadow back to z 0 (y kept); then, unless `deselect_only`, selects hand
/// card `iSelectedDeckCard`: its object and shadow by name in
/// `pHandGeometry[0]` (`Player-Deck_0%i:%i` with the card's number and
/// index, `Player-Deck_0%i_Shadow`), `pSelectedCard`, and both raised to z
/// 1.0.
// Translation incomplete: 0074e480 - the /GS stack cookie check (00ec408c) is not translated
pub fn caravan_menu_select_deck_card(e: &mut Engine, this: Ptr<CaravanMenu>, deselect_only: u8) {
    let selected = word_at(e, this.addr(), SELECTED_OBJECT);
    if selected != 0 {
        let translate = e.call(LOCAL_TRANSLATE, &args![selected]).u32();
        let z = x87_load32(e.mem.u32(translate + 8));
        // FCOMP; TEST AH,0x41; JNZ: only strictly above 0.0 (NaN skips).
        if z > e.global::<f64>(ZERO_DOUBLE) {
            place_at_selected_height(e, this, SELECTED_OBJECT, 0);
            place_at_selected_height(e, this, SELECTED_SHADOW, 0);
        }
    }
    if deselect_only != 0 {
        return;
    }
    e.with_stack(0x20, |e, buffer| {
        let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard);
        e.call(
            PRINT_BOUNDED,
            &args![
                buffer,
                0x1eu32,
                HAND_CARD_FORMAT,
                deck_card.wrapping_add(1),
                deck_card
            ],
        );
        let hand = word_at(e, this.addr(), HAND_GEOMETRY);
        let object = e.call(OBJECT_BY_NAME, &args![hand, buffer]).u32();
        e.mem.set_u32(this.addr() + SELECTED_OBJECT, object);
        let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard);
        e.call(
            PRINT_BOUNDED,
            &args![
                buffer,
                0x1eu32,
                HAND_SHADOW_FORMAT,
                deck_card.wrapping_add(1)
            ],
        );
        let hand = word_at(e, this.addr(), HAND_GEOMETRY);
        let shadow = e.call(OBJECT_BY_NAME, &args![hand, buffer]).u32();
        e.mem.set_u32(this.addr() + SELECTED_SHADOW, shadow);
    });
    let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard) as u32;
    let card = array_element(e, this.addr() + PLAYER_HAND, deck_card);
    e.mem.set_u32(this.addr() + SELECTED_CARD, card);
    let one = 1.0f32.to_bits();
    place_at_selected_height(e, this, SELECTED_OBJECT, one);
    place_at_selected_height(e, this, SELECTED_SHADOW, one);
}

// Translated from 0074a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::ItemSelectCallback` (Xbox PDB name from the call graph,
/// cdecl): the quantity menu's answer `amount` is added to `iPlayerAnte`
/// (shown on `TI_ANTE_PLAYER_BET`); the opponent's limit is its funds times
/// `fBetPercentage` plus that times the challenger's actor value 0x20 over
/// 100 (truncated), at most its funds; when the player's ante is above the
/// opponent's, the opponent raises to the smaller of the limit and the
/// player's ante (shown on `TI_ANTE_NPC_BET`). Then the flags and tile
/// alphas are updated and the betting interface shown with the limit and
/// the change (`amount` less the old ante, plus how far the smaller of the
/// ante and the limit is above the opponent's ante, at least 0).
pub fn caravan_menu_item_select_callback(e: &mut Engine, amount: i32) {
    let menu = me(e);
    let mut change = amount.wrapping_sub(e.get(menu, CaravanMenu::iPlayerAnte));
    let ante = e.get(menu, CaravanMenu::iPlayerAnte).wrapping_add(amount);
    e.set(menu, CaravanMenu::iPlayerAnte, ante);
    let one = 1.0f32.to_bits();
    let value_trait = e.global::<u32>(UT_VALUE);
    let bet_tile = tile(e, me(e), 5);
    e.call(TILE_POKE, &args![bet_tile, value_trait, one]);
    let ante = e.get(me(e), CaravanMenu::iPlayerAnte);
    let value_trait = e.global::<u32>(UT_VALUE);
    let bet_tile = tile(e, me(e), 5);
    e.call(SET_TRAIT_FROM_INT, &args![bet_tile, value_trait, ante]);
    caravan_menu_set_flag(e, me(e), CAF_DISABLE_ALLCONTROLS, 0);
    let limit = opponent_limit(e);
    let funds = e.mem.i32(me(e).addr() + NPC_TOTAL_FUNDS);
    let limit = e.call(MIN_OF, &args![limit, funds]).i32();
    let menu = me(e);
    if e.get(menu, CaravanMenu::iPlayerAnte) > e.get(menu, CaravanMenu::iNPCAnte) {
        let ante = e.get(menu, CaravanMenu::iPlayerAnte);
        let raised = e.call(MIN_OF, &args![limit, ante]).i32();
        e.set(me(e), CaravanMenu::iNPCAnte, raised);
        let value_trait = e.global::<u32>(UT_VALUE);
        let npc_tile = tile(e, me(e), 4);
        e.call(TILE_POKE, &args![npc_tile, value_trait, one]);
        let ante = e.get(me(e), CaravanMenu::iNPCAnte);
        let value_trait = e.global::<u32>(UT_VALUE);
        let npc_tile = tile(e, me(e), 4);
        e.call(SET_TRAIT_FROM_INT, &args![npc_tile, value_trait, ante]);
    }
    let ante = e.get(me(e), CaravanMenu::iPlayerAnte);
    let smaller = e.call(MIN_OF, &args![ante, limit]).i32();
    let above = smaller.wrapping_sub(e.get(me(e), CaravanMenu::iNPCAnte));
    let above = e.call(MAX_OF, &args![above, 0i32]).i32();
    change = above.wrapping_add(change);
    caravan_menu_update_caravan_flags(e, me(e));
    caravan_menu_update_tile_alpha(e, me(e), -1);
    caravan_menu_update_betting_ui(e, me(e), limit, change);
}

/// The dimmed and the full alpha of a tile (`float` 128.0 at `0101e704`,
/// 255.0 at `01023cd8`).
const DIM_ALPHA: u32 = 0x0101_e704;
/// A setting's string value (`ECX` = the setting): `00403df0`.
const SETTING_STRING: u32 = 0x0040_3df0;
/// The message box shower (cdecl, 11 words): `00703e80`.
const SHOW_MESSAGE: u32 = 0x0070_3e80;
/// The three settings of the forfeit question (message, then the two
/// buttons, as pushed).
const FORFEIT_MESSAGE_SETTING: u32 = 0x011d_43b4;
const FORFEIT_YES_SETTING: u32 = 0x011d_34f8;
const FORFEIT_NO_SETTING: u32 = 0x011d_3684;
/// `CaravanMenu::CancelConfirmFunc`, the message box callback.
const CANCEL_CONFIRM_CALLBACK: u32 = 0x0074_a270;
/// The interface manager singleton getter (`004b7210`).
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// `InterfaceManager::ClearMenuButton` (cdecl, one word).
const CLEAR_MENU_BUTTON: u32 = 0x0071_7de0;
/// `NiAVObject::GetProperty` (`this`, property type).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// The word at +0x1c of an object (`00441110`): the property's type here.
const WORD_AT_0X1C: u32 = 0x0044_1110;
/// `NiPointer` set-up in a stack slot (`this` = slot, value): `00633c90`.
const POINTER_INIT: u32 = 0x0063_3c90;
/// `NiPointer` release of a stack slot (`this` = slot): `0045cec0`.
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// The address of a form's texture name (`0050d100`: `this + 0x9c`).
const TEXTURE_NAME_ADDRESS: u32 = 0x0050_d100;
/// A texture name's text (`this` = name): `00408da0`.
const TEXTURE_NAME_TEXT: u32 = 0x0040_8da0;
/// The format and prefix the texture path is built with (`%s%s`,
/// `Textures\`).
const PATH_FORMAT: u32 = 0x0101_996c;
const TEXTURES_PREFIX: u32 = 0x0101_d7fc;
/// `SwapPlatformLanguageTexturePath` (cdecl: path, out, size).
const SWAP_TEXTURE_PATH: u32 = 0x004b_7240;
/// `TES::CreateTextureImage` (`this` = `TES`, path, out slot, 0, 0).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// The `TES` global (`011dea10`).
const TES: u32 = 0x011d_ea10;
/// The name of a track card object (`Card_0%i:%i`).
const TRACK_CARD_FORMAT: u32 = 0x0107_1a6c;
/// `ModelLoader::LoadFile` (`this` = loader, path, 0, 1, 0, 0, 0).
const MODEL_LOAD_FILE: u32 = 0x0044_7080;
/// The model loader global (`011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The table model's path.
const TABLE_MODEL_PATH: u32 = 0x0107_12d4;
/// The Gamebryo allocator (cdecl, size): `00aa13e0`.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiNode` constructor (`this`, child count): `00a5ecb0`.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiPointer` assignment (`this` = pointer, value): `0066b0d0`.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The camera animation names of the table (`Bet_to_Deck`, `Deck_to_Play`,
/// `Play_to_Bet`).
const BET_TO_DECK: u32 = 0x0107_132c;
const DECK_TO_PLAY: u32 = 0x0107_131c;
const PLAY_TO_BET: u32 = 0x0107_1310;
/// `NiControllerManager::DeactivateAll` (`this`, time).
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
/// The camera object's name in the table model (`object0`).
const CAMERA_OBJECT_NAME: u32 = 0x0107_1308;
/// The camera pointer (`CaravanMenu::sp3DCamera`, Xbox PDB static name;
/// `011d91a8`).
const CAMERA_POINTER: u32 = 0x011d_91a8;
/// Makes the menu's camera (`this` = menu): `007c2180`.
const MAKE_CAMERA: u32 = 0x007c_2180;
/// The scene node array entry `n` (cdecl: `011f91c8[n]`): `00450b80`.
const SCENE_NODE: u32 = 0x0045_0b80;
/// `NiAVObject::GetWorldBound` (`this` only).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// The radius of a bound (`this`; ST0): `0084d030`.
const BOUND_RADIUS: u32 = 0x0084_d030;
/// `double` 2.5 (`01018c00`).
const LIGHT_RADIUS_SCALE: u32 = 0x0101_8c00;
/// `ShadowSceneNode::AddAllLightsFromNode_Recurse` (`this`, node, radius).
const ADD_ALL_LIGHTS: u32 = 0x00b5_ca70;

/// `CaravanMenu::enumFlags` (Xbox PDB enum).
const CAF_DISABLE_Y: u32 = 2;
const CAF_DISABLE_B: u32 = 8;
const CAF_DISABLE_X: u32 = 0x10;
const CAF_DISABLE_LB: u32 = 0x20;
const CAF_DISABLE_RB: u32 = 0x40;
const CAF_DISABLE_LT: u32 = 0x80;
const CAF_DISABLE_RT: u32 = 0x100;
const CAF_SELECT_TRACK: u32 = 0x1000;
const CAF_DESTROY_TRACK: u32 = 0x2000;

/// More `CaravanMenu` offsets (Xbox PDB names).
const TABLE_GEOMETRY: u32 = 0x184;
const CAMERA_ANIM_MANAGER: u32 = 0x240;
const CAMERA_ANIMATIONS: u32 = 0x244;
const NEXT_PLAYER_CARD: u32 = 0xe40;
const NEXT_NPC_CARD: u32 = 0xe44;

// Translated from 00749740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::UpdateCaravanFlags` (Xbox PDB name from the call graph):
/// sets the disable flags of the active menu's buttons.
/// - Ante menu: Y unless the player bet or the opponent did not; without
///   funds A and X; with a bet A when it reaches the opponent's ante (or,
///   below it, the funds) and X when it reaches the funds; without a bet
///   neither.
/// - Deck menu: the meter's card (`_current_value`) in the caravan deck
///   (byte +0x98) disables A, else X; fewer than 30 cards disable Y and dim
///   tile 8 (trait `0xfa9`); an empty deck disables X, 108 cards or more A;
///   LB always.
/// - Game menu: an empty player deck sets `iNextPlayerCard` to -1 and
///   disables LT, an empty opponent deck sets `iNextNPCCard` to -1; RT is
///   disabled unless the player's tracks have cards and the tracks are not
///   being set up.
pub fn caravan_menu_update_caravan_flags(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    match e.get(this, CaravanMenu::eActiveMenu) {
        0 => {
            let ante = e.get(this, CaravanMenu::iPlayerAnte);
            let disable_y = !(ante > 0 || e.get(this, CaravanMenu::iNPCAnte) == 0);
            caravan_menu_set_flag(e, this, CAF_DISABLE_Y, u8::from(disable_y));
            if e.get(this, CaravanMenu::iTotalFunds) == 0 {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 1);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 1);
            } else if e.get(this, CaravanMenu::iPlayerAnte) > 0 {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 0);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 0);
                let ante = e.get(this, CaravanMenu::iPlayerAnte);
                let disable_a = if ante >= e.get(this, CaravanMenu::iNPCAnte) {
                    true
                } else if ante < e.get(this, CaravanMenu::iTotalFunds) {
                    false
                } else {
                    ante < e.get(this, CaravanMenu::iNPCAnte)
                };
                if disable_a {
                    caravan_menu_set_flag(e, this, CAF_DISABLE_A, 1);
                }
                let ante = e.get(this, CaravanMenu::iPlayerAnte);
                if ante >= e.get(this, CaravanMenu::iTotalFunds) {
                    caravan_menu_set_flag(e, this, CAF_DISABLE_X, 1);
                }
            } else {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 0);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 0);
            }
        }
        1 => {
            let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
            let meter = tile(e, this, 0xf);
            let value = float_trait(e, meter, trait_id);
            let index = e.call(FTOL, &args![value]).u32();
            let card = array_element(e, menu + DECK_ARRAY, index);
            if fn_0073cb80(e, Ptr::new(card)) != 0 {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 1);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 0);
            } else {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 0);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 1);
            }
            let alpha = if e.get(this, CaravanMenu::iCardsInDeck) < 0x1e {
                caravan_menu_set_flag(e, this, CAF_DISABLE_Y, 1);
                DIM_ALPHA
            } else {
                caravan_menu_set_flag(e, this, CAF_DISABLE_Y, 0);
                TRACK_ALPHA
            };
            let bits = x87_quiet_bits(e.global::<u32>(alpha));
            let cards_tile = tile(e, this, 8);
            set_float_bits(e, cards_tile, 0xfa9, bits);
            if e.get(this, CaravanMenu::iCardsInDeck) < 1 {
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 1);
            }
            if e.get(this, CaravanMenu::iCardsInDeck) >= 0x6c {
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 1);
            }
            caravan_menu_set_flag(e, this, CAF_DISABLE_LB, 1);
        }
        2 => {
            if array_size(e, menu + PLAYER_DECK) == 0 {
                e.mem.set_i32(menu + NEXT_PLAYER_CARD, -1);
                caravan_menu_set_flag(e, this, CAF_DISABLE_LT, 1);
            }
            if array_size(e, menu + NPC_DECK) == 0 {
                e.mem.set_i32(menu + NEXT_NPC_CARD, -1);
            }
            let has_cards = array_size(e, card_track(menu, 0, 0)) != 0
                || array_size(e, card_track(menu, 1, 0)) != 0
                || array_size(e, card_track(menu, 2, 0)) != 0;
            let disable = !has_cards || caravan_menu_has_flag(e, this, CAF_TRACK_SETUP);
            caravan_menu_set_flag(e, this, CAF_DISABLE_RT, u8::from(disable));
        }
        _ => {}
    }
}

// Translated from 00749a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::UpdateTileAlpha` (Xbox PDB name from the call graph):
/// dims (128.0) or shows (255.0) a tile's children through `0074a000`.
/// `index` -1 means the active menu. Menus 0, 1 and 2 update their button
/// tiles (7, 37, 36; 8, 9, 11, 10, 12, 13, 14; 20, 21, 19, 22). Each button
/// is dimmed while its disable flag or `CAF_DISABLE_ALLCONTROLS` is set:
/// 7 Y, 12 A, 13 X, 14 Y, 19 A, 20 LT, 21 RT, 22 B, 36 X, 37 A; tile 10 by
/// RB alone; tiles 9 and 11 are always shown. Tile 8 is dimmed by Y;
/// shown, its list head's tile also gets the alpha of the deck size (dim
/// below 30 cards).
pub fn caravan_menu_update_tile_alpha(e: &mut Engine, this: Ptr<CaravanMenu>, index: i32) {
    let index = if index == -1 {
        e.get(this, CaravanMenu::eActiveMenu)
    } else {
        index
    };
    let fade = |e: &mut Engine, alpha: u32| {
        let bits = x87_quiet_bits(e.global::<u32>(alpha));
        fn_0074a000(e, this, index as u32, bits);
    };
    let by_flag = |e: &mut Engine, mask: u32| {
        let dim = caravan_menu_has_flag(e, this, mask)
            || caravan_menu_has_flag(e, this, CAF_DISABLE_ALLCONTROLS);
        fade(e, if dim { DIM_ALPHA } else { TRACK_ALPHA });
    };
    match index {
        0 => {
            for child in [7, 0x25, 0x24] {
                caravan_menu_update_tile_alpha(e, this, child);
            }
        }
        1 => {
            for child in [8, 9, 0xb, 0xa, 0xc, 0xd, 0xe] {
                caravan_menu_update_tile_alpha(e, this, child);
            }
        }
        2 => {
            for child in [0x14, 0x15, 0x13, 0x16] {
                caravan_menu_update_tile_alpha(e, this, child);
            }
        }
        7 => by_flag(e, CAF_DISABLE_Y),
        8 => {
            if caravan_menu_has_flag(e, this, CAF_DISABLE_Y) {
                fade(e, DIM_ALPHA);
            } else {
                fade(e, TRACK_ALPHA);
                let alpha = if e.get(this, CaravanMenu::iCardsInDeck) < 0x1e {
                    DIM_ALPHA
                } else {
                    TRACK_ALPHA
                };
                let bits = x87_quiet_bits(e.global::<u32>(alpha));
                let list = tile(e, this, index as u32).wrapping_add(4);
                let head = e.call(LIST_HEAD_DATA, &args![list]).u32();
                let child = e.mem.u32(head);
                set_float_bits(e, child, 0xfa9, bits);
            }
        }
        9 | 0xb => fade(e, TRACK_ALPHA),
        0xa => {
            let dim = caravan_menu_has_flag(e, this, CAF_DISABLE_RB);
            fade(e, if dim { DIM_ALPHA } else { TRACK_ALPHA });
        }
        0xc | 0x13 | 0x25 => by_flag(e, CAF_DISABLE_A),
        0xd | 0x24 => by_flag(e, CAF_DISABLE_X),
        0xe => by_flag(e, CAF_DISABLE_Y),
        0x14 => by_flag(e, CAF_DISABLE_LT),
        0x15 => by_flag(e, CAF_DISABLE_RT),
        0x16 => by_flag(e, CAF_DISABLE_B),
        _ => {}
    }
}

/// `Interface::IsTopMenuID` of the menu's class (`this` slot +0x34).
fn is_top_menu(e: &mut Engine, this: Ptr<CaravanMenu>) -> bool {
    let class = e.vcall(this.addr(), 0x34, &args![]).u32();
    e.call(IS_TOP_MENU_ID, &args![class]).bool()
}

/// `Controls::ClearUserActions` on the instance of `011dea0c`.
fn clear_user_actions(e: &mut Engine) {
    let owner = e.global::<u32>(CONTROLS_OWNER);
    let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
    e.call(CONTROLS_CLEAR_USER_ACTIONS, &args![controls]);
}

// Translated from 007490e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoGamepadUpEvent` (vtable slot +0x3c, Xbox PDB): only
/// button 10 with the controls enabled is handled. The results menu closes
/// when on top; the ante and deck menus close when on top and clear the
/// user actions; in the game a track selection is cancelled (the whole
/// track or the card deselected, `CAF_DESTROY_TRACK`, `CAF_SELECT_TRACK`
/// and `CAF_DISABLE_A` cleared, X disabled, alphas updated), else, on top,
/// the forfeit question is shown (`CancelConfirmFunc`, message type 0x17)
/// and the controls disabled. Then menu button 10 is cleared; answers 1
/// (0 when not handled).
pub fn caravan_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    if caravan_menu_has_flag(e, this, CAF_DISABLE_ALLCONTROLS) || button != 0xa {
        return false;
    }
    if e.get(this, CaravanMenu::eActiveMenu) == 3 && is_top_menu(e, this) {
        caravan_menu_close(e);
    }
    match e.get(this, CaravanMenu::eActiveMenu) {
        0 | 1 => {
            if is_top_menu(e, this) {
                caravan_menu_close(e);
            }
            clear_user_actions(e);
        }
        2 => {
            if caravan_menu_has_flag(e, this, CAF_SELECT_TRACK) {
                if caravan_menu_has_flag(e, this, CAF_DESTROY_TRACK) {
                    let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
                    caravan_menu_deselect_whole_track(e, this, track);
                } else {
                    caravan_menu_deselect_track_card(e, this);
                }
                caravan_menu_set_flag(e, this, CAF_DESTROY_TRACK, 0);
                caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 0);
                caravan_menu_set_flag(e, this, CAF_DISABLE_A, 0);
                caravan_menu_set_flag(e, this, CAF_DISABLE_X, 1);
                caravan_menu_update_tile_alpha(e, this, -1);
            } else if is_top_menu(e, this) {
                let no = e.call(SETTING_STRING, &args![FORFEIT_NO_SETTING]).u32();
                let yes = e.call(SETTING_STRING, &args![FORFEIT_YES_SETTING]).u32();
                let message = e
                    .call(SETTING_STRING, &args![FORFEIT_MESSAGE_SETTING])
                    .u32();
                e.call(
                    SHOW_MESSAGE,
                    &args![
                        message,
                        0u32,
                        0u32,
                        CANCEL_CONFIRM_CALLBACK,
                        0u32,
                        0x17u32,
                        0u32,
                        0u32,
                        yes,
                        no,
                        0u32
                    ],
                );
                caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
            }
        }
        _ => {}
    }
    e.call(INTERFACE_SINGLETON, &args![]);
    e.call(CLEAR_MENU_BUTTON, &args![0xau32]);
    true
}

/// The texture property (type 3) of `node` when its word +0x1c is 8 to 12,
/// else 0 (the exe asks `GetProperty` again for each test).
fn texture_property(e: &mut Engine, node: u32) -> u32 {
    let has_property = e.call(GET_PROPERTY, &args![node, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
        e.call(WORD_AT_0X1C, &args![property]).i32() >= 8 && {
            let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
            e.call(WORD_AT_0X1C, &args![property]).i32() <= 0xc
        }
    };
    if has_property {
        e.call(GET_PROPERTY, &args![node, 3u32]).u32()
    } else {
        0
    }
}

/// Loads the texture whose path is in `name` (`SwapPlatformLanguageTexturePath`
/// into `path`, `TES::CreateTextureImage` into the `NiPointer` slot
/// `texture`) and, when one was made, gives it to `property` (slot +0xf0
/// with 0, 0).
fn load_property_texture(e: &mut Engine, property: u32, name: u32, path: u32, texture: u32) {
    e.call(SWAP_TEXTURE_PATH, &args![name, path, 0x104u32]);
    let tes = e.global::<u32>(TES);
    e.call(CREATE_TEXTURE_IMAGE, &args![tes, path, texture, 0u32, 0u32]);
    if e.call(FIRST_WORD, &args![texture]).u32() != 0 {
        let image = e.call(FIRST_WORD, &args![texture]).u32();
        e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
    }
}

/// The text of the texture name at `field` of `card` (`0050d100`: +0x9c, or
/// `00503650`: +0xa8; then `00408da0`).
fn card_texture_text(e: &mut Engine, field: u32, card: u32) -> u32 {
    let texture_name = e.call(field, &args![card]).u32();
    e.call(TEXTURE_NAME_TEXT, &args![texture_name]).u32()
}

/// The texture part shared by `0074df90`, `0074e220` and `SelectTrackCard`:
/// when `node` has a texture property ([`texture_property`]), loads the
/// card's texture (`Textures\` + its texture name at +0x9c) into it
/// ([`load_property_texture`]); the texture goes through a stack
/// `NiPointer`. The card is read only when there is a property.
fn apply_card_texture(e: &mut Engine, node: u32, card: impl FnOnce(&mut Engine) -> u32) {
    let property = texture_property(e, node);
    e.with_stack(0x20c, |e, frame| {
        let name = frame.addr();
        let path = frame.addr() + 0x104;
        let texture = frame.addr() + 0x208;
        e.call(POINTER_INIT, &args![texture, 0u32]);
        if property != 0 {
            let card = card(e);
            let text = card_texture_text(e, TEXTURE_NAME_ADDRESS, card);
            e.call(
                PRINT_BOUNDED,
                &args![name, 0x104u32, PATH_FORMAT, TEXTURES_PREFIX, text],
            );
            load_property_texture(e, property, name, path, texture);
        }
        e.call(POINTER_RELEASE, &args![texture]);
    });
}

/// The body shared by `0074df90` and `0074e220`: finds the card object
/// `Card_0%i:%i` (number `4 - column` on the opponent's tracks 3 to 5,
/// `column + 1` on the player's; then the column) under the track row's
/// object and puts the card's texture on it ([`apply_card_texture`]).
fn swap_track_card_texture(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    track: u32,
    row: u32,
    column: u32,
    card: impl FnOnce(&mut Engine) -> u32,
) {
    let object = track_geometry(e, this.addr(), track, row);
    let number = if (track as i32) >= 3 {
        4u32.wrapping_sub(column)
    } else {
        column.wrapping_add(1)
    };
    let node = e.with_stack(0x104, |e, name| {
        e.call(
            PRINT_BOUNDED,
            &args![name, 0x104u32, TRACK_CARD_FORMAT, number, column],
        );
        e.call(OBJECT_BY_NAME, &args![object, name]).u32()
    });
    apply_card_texture(e, node, card);
}

// Translated from 0074df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the selected card (`pSelectedCard`) on the selected track slot
/// (`iSelectedTrack`, `iSelectedRow`, `iSelectedColumn`): see
/// [`swap_track_card_texture`].
// Translation incomplete: 0074df90 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_0074df90(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
    let row = e.get(this, CaravanMenu::iSelectedRow) as u32;
    let column = e.get(this, CaravanMenu::iSelectedColumn) as u32;
    swap_track_card_texture(e, this, track, row, column, |e| {
        word_at(e, this.addr(), SELECTED_CARD)
    });
}

// Translated from 0074e220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows card `column` of row `row` of track `track` on its slot: see
/// [`swap_track_card_texture`].
// Translation incomplete: 0074e220 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_0074e220(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32, row: u32, column: u32) {
    swap_track_card_texture(e, this, track, row, column, |e| {
        array_element(e, card_track(this.addr(), track, row), column)
    });
}

/// `0047a520(manager, name)` with the name as a stack `NiFixedString`
/// (made with `00438170`, released with `004381b0`).
fn sequence_by_name(e: &mut Engine, manager: u32, name: u32) -> u32 {
    e.with_stack(4, |e, fixed| {
        let slot = e.call(FIXED_STRING_INIT, &args![fixed, name]).u32();
        let sequence = e.call(GET_SEQUENCE, &args![manager, slot]).u32();
        e.call(FIXED_STRING_RELEASE, &args![fixed]);
        sequence
    })
}

// Translated from 0073cbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareShared3DElements` (Xbox PDB): loads the table model
/// (`pTableGeometry`), makes a new `NiNode` (0xac bytes) the money root
/// (`011d91e4`), attaches the table to it and updates it; with a controller
/// manager on the table keeps it (`pCameraAnimManager`) with its
/// `Bet_to_Deck`, `Deck_to_Play` and `Play_to_Bet` sequences, deactivates
/// them and starts the animations; points `sp3DCamera` at the table's
/// `object0` (or a new camera, `007c2180`, when there is none); attaches
/// the root to scene node 1 and adds its lights within 2.5 times its bound
/// radius.
// Translation incomplete: 0073cbf0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn caravan_menu_prepare_shared_3d_elements(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    let loader = e.global::<u32>(MODEL_LOADER);
    let table = e
        .call(
            MODEL_LOAD_FILE,
            &args![loader, TABLE_MODEL_PATH, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    e.mem.set_u32(menu + TABLE_GEOMETRY, table);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![MONEY_ROOT, node]);
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let table = word_at(e, menu, TABLE_GEOMETRY);
    e.vcall(root, 0xdc, &args![table, 1u32]);
    e.with_stack(0xc, |e, block| {
        e.call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32]);
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        e.call(OBJECT_UPDATE, &args![root, block]);
    });
    let table = word_at(e, menu, TABLE_GEOMETRY);
    let manager = e
        .call(GET_CONTROLLER, &args![table, CONTROLLER_MANAGER_RTTI])
        .u32();
    if manager != 0 {
        e.mem.set_u32(menu + CAMERA_ANIM_MANAGER, manager);
        for (index, name) in [BET_TO_DECK, DECK_TO_PLAY, PLAY_TO_BET]
            .into_iter()
            .enumerate()
        {
            let manager = word_at(e, menu, CAMERA_ANIM_MANAGER);
            let sequence = sequence_by_name(e, manager, name);
            e.mem
                .set_u32(menu + CAMERA_ANIMATIONS + 4 * index as u32, sequence);
        }
        let manager = word_at(e, menu, CAMERA_ANIM_MANAGER);
        e.call(DEACTIVATE_ALL, &args![manager, 0u32]);
        let table = word_at(e, menu, TABLE_GEOMETRY);
        e.call(START_ANIMATIONS, &args![table]);
    }
    let table = word_at(e, menu, TABLE_GEOMETRY);
    let camera = e
        .call(OBJECT_BY_NAME, &args![table, CAMERA_OBJECT_NAME])
        .u32();
    e.call(POINTER_ASSIGN, &args![CAMERA_POINTER, camera]);
    if e.call(FIRST_WORD, &args![CAMERA_POINTER]).u32() == 0 {
        let camera = e.call(MAKE_CAMERA, &args![menu]).u32();
        e.call(POINTER_ASSIGN, &args![CAMERA_POINTER, camera]);
    }
    let scene = e.call(SCENE_NODE, &args![1u32]).u32();
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    e.vcall(scene, 0xdc, &args![root, 1u32]);
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![root]).u32();
    let radius = e.call(BOUND_RADIUS, &args![bound]).f64();
    let radius = x87_store32(x87_mul(radius, e.global::<f64>(LIGHT_RADIUS_SCALE)));
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let scene = e.call(SCENE_NODE, &args![1u32]).u32();
    e.call(ADD_ALL_LIGHTS, &args![scene, root, x87_quiet_bits(radius)]);
}

/// `MiscStatManager::Increment` (cdecl, one word).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// `BSAudio::QInstance` (no arguments).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (`this`, out handle, name, flags; RET 0xc).
const SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `BSSoundHandle::Play` (`this`, one word; RET 4).
const SOUND_PLAY: u32 = 0x00ad_8830;
/// The sound handle's release (does nothing; `00483710`).
const SOUND_HANDLE_RELEASE: u32 = 0x0048_3710;
/// The sounds of a won and a lost game.
const WIN_SOUND: u32 = 0x0107_16a4;
const LOSE_SOUND: u32 = 0x0107_1694;
/// `Tile::GetChildByName` (`this`, name).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::AddNeedsUpdate` (`this`, flags).
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// The info rectangle names of the results and ante menus.
const RESULTS_INFO_RECT: u32 = 0x0107_1684;
const ANTE_INFO_RECT: u32 = 0x0107_1338;
/// `double` 150.0, added to an info rectangle's trait `0xfb1`.
const INFO_RECT_GROWTH: u32 = 0x0106_c948;
/// The won/lost games format (`%i/%i`).
const GAMES_FORMAT: u32 = 0x0107_14ec;
/// Sets a float trait from an unsigned integer (`this`, trait, value):
/// `00715c60`.
const SET_TRAIT_FROM_UNSIGNED: u32 = 0x0071_5c60;
/// Sets a string trait of a tile (`this`, trait, text, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// The results status settings (won, lost) and the results menu's four
/// caption settings (tiles 47 to 50).
const RESULT_WON_SETTING: u32 = 0x011d_4b58;
const RESULT_LOST_SETTING: u32 = 0x011d_1e6c;
const RESULTS_CAPTION_SETTINGS: [u32; 4] = [0x011d_309c, 0x011d_4f3c, 0x011d_2ab4, 0x011d_3360];
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this`, file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The menu's file and its missing-tile message.
const MENU_FILE: u32 = 0x0107_170c;
const MISSING_TILE_MESSAGE: u32 = 0x0107_16b8;
/// The word at +0x24 of an object (`0059bb30`): the deck parameter's list.
const WORD_AT_0X24: u32 = 0x0059_bb30;
/// Reads a float trait value: `Tile::GetMaximumDepth` (no arguments; ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// The deck meter's starting `_current_value` (`float` 12.0 at `01018f7c`).
const METER_START: u32 = 0x0101_8f7c;
/// The trait the meter is poked with (`_SetInCode`).
const SET_IN_CODE_TRAIT_NAME: u32 = 0x0107_0250;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// A byte global `Create` clears (`011d9184`).
const CREATE_BYTE: u32 = 0x011d_9184;
/// The setting whose string is the too-few-cards message (`004c69f0` reads
/// its string).
const FEW_CARDS_SETTING: u32 = 0x011d_47f8;
const SETTING_STRING_004C69F0: u32 = 0x004c_69f0;
/// The corner message shower (cdecl: text, 0, icon, 0, time, 0): `007052f0`.
const SHOW_CORNER_MESSAGE: u32 = 0x0070_52f0;
/// The message icon and its time (`float` 2.0 at `010162c0`).
const MESSAGE_ICON: u32 = 0x0104_9638;
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The general pop-up sound.
const POP_UP_SOUND: u32 = 0x0102_5cc4;
/// The tutorial manager's has-shown test and show (`this` = interface +
/// 0x4d4, index): `00718840`, `007185e0`.
const TUTORIAL_SHOWN: u32 = 0x0071_8840;
const TUTORIAL_MARK: u32 = 0x0071_85e0;
/// `TESForm::GetFormByEditorID` (cdecl, one word), the help message's
/// editor ID (`HelpCaravanBetting`), `__RTDynamicCast` (cdecl, five words)
/// and its two type records.
const FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const HELP_EDITOR_ID: u32 = 0x0101_8dec;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_6130;
/// `TutorialMenu::Create` (cdecl, message, 0).
const TUTORIAL_MENU_CREATE: u32 = 0x007e_8890;
/// The money model paths (format, folder) of `PrepareAnteMenu`.
const MONEY_FOLDER: u32 = 0x0107_12b4;
const BILL_GRID_FORMAT: u32 = 0x0107_1298;
const COIN_GRID_FORMAT: u32 = 0x0107_127c;
const BILL_FORMAT: u32 = 0x0107_1264;
const COIN_FORMAT: u32 = 0x0107_124c;
/// The ante menu's caption settings and their tiles.
const ANTE_CAPTIONS: [(u32, u32); 6] = [
    (40, 0x011d_2a60),
    (41, 0x011d_4954),
    (37, 0x011d_4168),
    (36, 0x011d_2034),
    (7, 0x011d_44ec),
    (38, 0x011d_33fc),
];
/// The challenger's full name (`0055d520`, `this` = actor) and `strlen`
/// (cdecl).
const FULL_NAME: u32 = 0x0055_d520;
const STRING_LENGTH: u32 = 0x0044_a670;
/// A tile's child by name (cdecl: tile, name): `00a08b20`.
const TILE_CHILD_NAMED: u32 = 0x00a0_8b20;
/// The opponent bet value tile's name (`CBM_NPCBetValue`).
const NPC_BET_VALUE_NAME: u32 = 0x0107_1348;

/// More `CaravanMenu` offsets (Xbox PDB names).
const DECK_PARAM: u32 = 0x104;
const DIFF_LEVEL: u32 = 0x108;
const BILLS_GRID: u32 = 0x114;
const COINS_GRID: u32 = 0x118;
const COIN_INDEX_TO_ANIM: u32 = 0x148;
const BILL_INDEX_TO_ANIM: u32 = 0x160;
const INT_LIST_BILLS: u32 = 0x164;
const INT_LIST_COINS: u32 = 0x174;
const ANTE: u32 = 0xae8;
const TUTORIAL_INDEX: u32 = 0xe7c;

/// Plays the sound `name` (`BSAudio::GetSoundHandleByName` into a stack
/// handle with flags 0x121, `BSSoundHandle::Play`, then the handle's
/// release).
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        // 00740a6b: `00ad7550` takes three words (RET 0xc); the pushed 0 is the word of
        // `00ad8830` (RET 4).
        let sound = e
            .call(SOUND_HANDLE_BY_NAME, &args![audio, handle, name, 0x121u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    });
}

/// Adds 150.0 to trait `0xfb1` of `parent`'s child `name` and marks it for
/// update (flags 0x10), when the child is there.
fn grow_info_rect(e: &mut Engine, parent: u32, name: u32) {
    let child = e.call(TILE_GET_CHILD_BY_NAME, &args![parent, name]).u32();
    if child == 0 {
        return;
    }
    let value = float_trait(e, child, 0xfb1);
    let value = x87_store32(x87_add(value, e.global::<f64>(INFO_RECT_GROWTH)));
    set_float_bits(e, child, 0xfb1, x87_quiet_bits(value));
    e.call(TILE_ADD_NEEDS_UPDATE, &args![child, 0x10u32]);
}

/// Pokes `_Value`'s trait of tile `index` with 1.0 (`Tile::Poke`).
fn poke_value(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32) {
    let value_trait = e.global::<u32>(UT_VALUE);
    let target = tile(e, this, index);
    e.call(TILE_POKE, &args![target, value_trait, 1.0f32.to_bits()]);
}

/// Sets the `_Value` trait of tile `index` from an integer.
fn set_value(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32, value: u32) {
    let value_trait = e.global::<u32>(UT_VALUE);
    let target = tile(e, this, index);
    e.call(SET_TRAIT_FROM_INT, &args![target, value_trait, value]);
}

/// Sets string trait `0xfc4` of tile `index` to setting `setting`'s text.
fn set_caption(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let target = tile(e, this, index);
    e.call(SET_STRING_TRAIT, &args![target, 0xfc4u32, text, 1u32]);
}

/// The opponent's betting limit: its funds times `fBetPercentage` plus that
/// times the player's actor value 0x20 (clamped, `0066ef50`) over 100.0,
/// truncated; all read from the singleton.
fn opponent_limit(e: &mut Engine) -> i32 {
    let menu = me(e).addr();
    let funds = f64::from(e.mem.i32(menu + NPC_TOTAL_FUNDS));
    let base = x87_mul(funds, x87_m32(e.mem.u32(menu + BET_PERCENTAGE)));
    let menu = me(e).addr();
    let funds = f64::from(e.mem.i32(menu + NPC_TOTAL_FUNDS));
    let scale = x87_mul(funds, x87_m32(e.mem.u32(menu + BET_PERCENTAGE)));
    let player = e.global::<u32>(PLAYER);
    let skill = e
        .call(
            CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0x20u32],
        )
        .f64();
    let bonus = x87_mul(x87_div(skill, e.global::<f64>(HUNDRED_DOUBLE)), scale);
    e.call(FTOL, &args![x87_add(bonus, base)]).i32()
}

// Translated from 00740980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareResultsMenu` (Xbox PDB name from the call graph):
/// sets `eGameState` 3; a won game (`CAF_PLAYERWIN`) adds `iAnte` to the
/// player's winnings, counts a won game, keeps the biggest win, increments
/// misc stat 0x25 and plays `GAMECaravanWin`; a lost one adds `iAnte` to the
/// losses, counts a lost game, increments misc stat 0x26 and plays
/// `GAMECaravanLose`. Then grows the results info rectangle, shows the
/// winnings, losses, `won/lost` games and biggest win (tiles 31 to 34),
/// the status text (tile 35), shows only the results tile set and the four
/// captions (tiles 47 to 50), and runs `0074cc70` with the win flag.
// Translation incomplete: 00740980 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_prepare_results_menu(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    e.set(this, CaravanMenu::eGameState, 3);
    if caravan_menu_has_flag(e, this, CAF_PLAYERWIN) {
        let player = Ptr::new(e.global::<u32>(PLAYER));
        let winnings = fn_00740900(e, player).wrapping_add(e.mem.u32(menu + ANTE));
        fn_00740e70(e, Ptr::new(e.global::<u32>(PLAYER)), winnings);
        let won = fn_00740940(e, Ptr::new(e.global::<u32>(PLAYER))).wrapping_add(1);
        fn_00740eb0(e, Ptr::new(e.global::<u32>(PLAYER)), won);
        let biggest = fn_00740e50(e, Ptr::new(e.global::<u32>(PLAYER)));
        if e.mem.u32(menu + ANTE) > biggest {
            let ante = e.mem.u32(menu + ANTE);
            fn_00740ef0(e, Ptr::new(e.global::<u32>(PLAYER)), ante);
        }
        e.call(MISC_STAT_INCREMENT, &args![0x25u32]);
        play_sound(e, WIN_SOUND);
    } else {
        let losses =
            fn_00740920(e, Ptr::new(e.global::<u32>(PLAYER))).wrapping_add(e.mem.u32(menu + ANTE));
        fn_00740e90(e, Ptr::new(e.global::<u32>(PLAYER)), losses);
        let lost = fn_00740960(e, Ptr::new(e.global::<u32>(PLAYER))).wrapping_add(1);
        fn_00740ed0(e, Ptr::new(e.global::<u32>(PLAYER)), lost);
        e.call(MISC_STAT_INCREMENT, &args![0x26u32]);
        play_sound(e, LOSE_SOUND);
    }
    let results = tile(e, this, 3);
    grow_info_rect(e, results, RESULTS_INFO_RECT);
    e.with_stack(0x104, |e, buffer| {
        let lost = fn_00740960(e, Ptr::new(e.global::<u32>(PLAYER)));
        let won = fn_00740940(e, Ptr::new(e.global::<u32>(PLAYER)));
        e.call(
            PRINT_BOUNDED,
            &args![buffer, 0x104u32, GAMES_FORMAT, won, lost],
        );
        poke_value(e, this, 31);
        let winnings = fn_00740900(e, Ptr::new(e.global::<u32>(PLAYER)));
        let value_trait = e.global::<u32>(UT_VALUE);
        let target = tile(e, this, 31);
        e.call(
            SET_TRAIT_FROM_UNSIGNED,
            &args![target, value_trait, winnings],
        );
        poke_value(e, this, 32);
        let losses = fn_00740920(e, Ptr::new(e.global::<u32>(PLAYER)));
        let value_trait = e.global::<u32>(UT_VALUE);
        let target = tile(e, this, 32);
        e.call(SET_TRAIT_FROM_UNSIGNED, &args![target, value_trait, losses]);
        poke_value(e, this, 33);
        let value_trait = e.global::<u32>(UT_VALUE);
        let target = tile(e, this, 33);
        e.call(SET_STRING_TRAIT, &args![target, value_trait, buffer, 1u32]);
    });
    poke_value(e, this, 34);
    let biggest = fn_00740e50(e, Ptr::new(e.global::<u32>(PLAYER)));
    let value_trait = e.global::<u32>(UT_VALUE);
    let target = tile(e, this, 34);
    e.call(
        SET_TRAIT_FROM_UNSIGNED,
        &args![target, value_trait, biggest],
    );
    let status = if caravan_menu_has_flag(e, this, CAF_PLAYERWIN) {
        RESULT_WON_SETTING
    } else {
        RESULT_LOST_SETTING
    };
    set_caption(e, this, 35, status);
    for (index, visible) in [(0u32, 0u32), (1, 0), (2, 0), (3, 1)] {
        let target = tile(e, this, index);
        e.call(SET_TRAIT_FROM_INT, &args![target, 0xfa3u32, visible]);
    }
    for (index, setting) in (47..).zip(RESULTS_CAPTION_SETTINGS) {
        set_caption(e, this, index, setting);
    }
    let won = caravan_menu_has_flag(e, this, CAF_PLAYERWIN);
    fn_0074cc70(e, this, u8::from(won));
}

// Translated from 00741060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::Create` (Xbox PDB, cdecl): opens the caravan menu against
/// `challenger` with the deck parameter's list (`0059bb30`), the AI level
/// and the bet percentage. The fifth word is overwritten with 0 before it
/// is read (and that 0 is stored in the byte at `011d9184`).
///
/// With fewer than 30 cards in the player's two lists shows the too-few
/// message (setting `011d47f8`), plays `UIPopUpMessageGeneral` and returns
/// 0. Otherwise deletes the open caravan menu, reads `caravan_menu.xml`,
/// takes its menu as `pMe` and, when it is a caravan menu: stores the
/// arguments, attaches the tile, checks the 0x33 tiles (else prints a
/// message, closes and returns 0), sets the depth for the two stacking
/// types, prepares the cards, sets the deck meter (`0x1004` the total
/// cards, `_current_value` 12.0, pokes `_SetInCode`), prepares the 3D table
/// and the ante menu, enables the controls, prepares the menu for showing
/// and shows the betting help (tutorial 0x1e) once. Returns the menu file's
/// root tile. A menu of another class is deleted (when it has a tile) and
/// the result is 0.
// Translation incomplete: 00741060 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_create(
    e: &mut Engine,
    challenger: u32,
    deck_param: u32,
    difficulty: u32,
    bet_percentage: u32,
    _overwritten_5: u32,
) -> u32 {
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![CARAVAN_MENU_CLASS])
        .u32();
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.set_global(CARAVAN_INTERFACE, interface);
    let first = fn_0073cba0(e, Ptr::new(e.global::<u32>(PLAYER)));
    let in_first = e.call(CARD_LIST_COUNT, &args![first]).u32();
    let second = fn_0073cbc0(e, Ptr::new(e.global::<u32>(PLAYER)));
    let in_second = e.call(CARD_LIST_COUNT, &args![second]).u32();
    if (in_first.wrapping_add(in_second) as i32) < 0x1e {
        e.with_stack(0x104, |e, buffer| {
            let text = e
                .call(SETTING_STRING_004C69F0, &args![FEW_CARDS_SETTING])
                .u32();
            e.call(PRINT_BOUNDED, &args![buffer, 0x104u32, text]);
            let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
            e.call(
                SHOW_CORNER_MESSAGE,
                &args![buffer, 0u32, MESSAGE_ICON, 0u32, time, 0u32],
            );
        });
        play_sound(e, POP_UP_SOUND);
        return 0;
    }
    let flag = 0u8;
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let interface = e.global::<u32>(CARAVAN_INTERFACE);
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(CARAVAN_MENU, menu);
    if me(e).addr() != 0 {
        let class = e.vcall(me(e).addr(), 0x34, &args![]).u32();
        if class == fn_0073cbe0(e) {
            let menu = me(e).addr();
            e.mem.set_u32(menu + CHALLENGER, challenger);
            let list = e.call(WORD_AT_0X24, &args![deck_param]).u32();
            e.mem.set_u32(me(e).addr() + DECK_PARAM, list);
            e.mem.set_u32(me(e).addr() + DIFF_LEVEL, difficulty);
            e.mem.set_u32(
                me(e).addr() + BET_PERCENTAGE,
                x87_quiet_bits(bet_percentage),
            );
            e.call(MENU_SET_MENU_TILE, &args![me(e), root, 0u32]);
            if !caravan_menu_has_all_tiles(e, me(e)) {
                e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE]);
                caravan_menu_close(e);
                return 0;
            }
            let kind = float_trait(e, root, 0xfa7);
            let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
                let again = float_trait(e, root, 0xfa7);
                again == e.global::<f64>(STACKING_TYPE_TWO)
            };
            if matches {
                let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
                set_float_bits(e, root, 0xfad, x87_store32(depth));
            }
            caravan_menu_prepare_caravan_data(e, me(e));
            let total = e.get(me(e), CaravanMenu::iTotalCards);
            let total = x87_store32(f64::from(total));
            let meter = tile(e, me(e), 0xf);
            e.call(SET_FLOAT_TRAIT, &args![meter, 0x1004u32, total, 0u32]);
            let start = x87_quiet_bits(e.global::<u32>(METER_START));
            let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
            let meter = tile(e, me(e), 0xf);
            e.call(SET_FLOAT_TRAIT, &args![meter, trait_id, start, 0u32]);
            let set_in_code = e
                .call(TILE_TEXT_TO_TRAIT, &args![SET_IN_CODE_TRAIT_NAME])
                .u32();
            let meter = tile(e, me(e), 0xf);
            e.call(TILE_POKE, &args![meter, set_in_code, 1.0f32.to_bits()]);
            e.mem.set_u8(CREATE_BYTE, flag);
            caravan_menu_prepare_shared_3d_elements(e, me(e));
            caravan_menu_prepare_ante_menu(e, me(e));
            caravan_menu_set_flag(e, me(e), CAF_DISABLE_ALLCONTROLS, 0);
            e.call(MENU_PREP_FOR_VISIBILITY, &args![me(e), 0u32]);
            let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
            let tutorials = interface.wrapping_add(0x4d4);
            if !e.call(TUTORIAL_SHOWN, &args![tutorials, 0x1eu32]).bool() {
                let form = e.call(FORM_BY_EDITOR_ID, &args![HELP_EDITOR_ID]).u32();
                let message = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                if message != 0 {
                    let shown = e.call(TUTORIAL_MENU_CREATE, &args![message, 0u32]).u8();
                    e.set(me(e), CaravanMenu::bTutorial, shown);
                    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
                    e.call(
                        TUTORIAL_MARK,
                        &args![interface.wrapping_add(0x4d4), 0x1eu32],
                    );
                    e.mem.set_u32(me(e).addr() + TUTORIAL_INDEX, 0x1e);
                }
            }
            return root;
        }
    }
    if me(e).addr() != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e).addr();
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

// Translated from 0073d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareAnteMenu` (Xbox PDB): loads the money models
/// (`pBillsGrid`, `pCoinsGrid`, three bills and three coins), makes the
/// ante menu active and idle, clears the bill and coin animation indexes,
/// shows only the ante tile set, sets its six captions, counts the money
/// (`PrepareBettingCurrency`), sets the opponent's ante (its funds times
/// `fBetPercentage`, truncated), widens the opponent bet value tile by 15
/// per letter of the challenger's name past 17, grows the ante info
/// rectangle, shows the challenger's name and the antes and funds (tiles 4
/// to 6), updates the flags and alphas, fills `intlistbills` with 0 to 8
/// and `intlistcoins` with 0 to 41, and shows the betting interface with
/// the opponent's limit and ante.
// Translation incomplete: 0073d020 - the /GS stack cookie check (00ec408c) is not translated
pub fn caravan_menu_prepare_ante_menu(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    let load = |e: &mut Engine, buffer: Ptr, words: &[u32], target: u32| {
        let mut call = vec![buffer.addr(), 0x104];
        call.extend_from_slice(words);
        e.call(PRINT_BOUNDED, &call);
        let loader = e.global::<u32>(MODEL_LOADER);
        let model = e
            .call(
                MODEL_LOAD_FILE,
                &args![loader, buffer, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(target, model);
    };
    e.with_stack(0x104, |e, buffer| {
        load(
            e,
            buffer,
            &[BILL_GRID_FORMAT, MONEY_FOLDER],
            menu + BILLS_GRID,
        );
        load(
            e,
            buffer,
            &[COIN_GRID_FORMAT, MONEY_FOLDER],
            menu + COINS_GRID,
        );
        for index in 0..3u32 {
            let number = index + 1;
            load(
                e,
                buffer,
                &[BILL_FORMAT, MONEY_FOLDER, number],
                menu + BILL_GEOMETRY + 4 * index,
            );
            load(
                e,
                buffer,
                &[COIN_FORMAT, MONEY_FOLDER, number],
                menu + COIN_GEOMETRY + 4 * index,
            );
        }
    });
    e.set(this, CaravanMenu::eActiveMenu, 0);
    e.set(this, CaravanMenu::eGameState, 0);
    e.mem.set_u32(menu + BILL_INDEX_TO_ANIM, 0);
    e.mem.set_u32(menu + COIN_INDEX_TO_ANIM, 0);
    for (index, visible) in [(0u32, 1u32), (1, 0), (2, 0), (3, 0)] {
        let target = tile(e, this, index);
        e.call(SET_TRAIT_FROM_INT, &args![target, 0xfa3u32, visible]);
    }
    for (index, setting) in ANTE_CAPTIONS {
        set_caption(e, this, index, setting);
    }
    caravan_menu_prepare_betting_currency(e, this);
    let funds = f64::from(e.mem.i32(menu + NPC_TOTAL_FUNDS));
    let ante = x87_mul(funds, x87_m32(e.mem.u32(menu + BET_PERCENTAGE)));
    let ante = e.call(FTOL, &args![ante]).i32();
    e.set(this, CaravanMenu::iNPCAnte, ante);
    let challenger = word_at(e, menu, CHALLENGER);
    let name = e.call(FULL_NAME, &args![challenger]).u32();
    if e.call(STRING_LENGTH, &args![name]).u32() > 0x11 {
        let challenger = word_at(e, menu, CHALLENGER);
        let name = e.call(FULL_NAME, &args![challenger]).u32();
        let length = e.call(STRING_LENGTH, &args![name]).u32();
        let widen = length.wrapping_sub(0x11).wrapping_mul(0xf) as i32;
        let bet_tile = tile(e, this, 4);
        let value_tile = e
            .call(TILE_CHILD_NAMED, &args![bet_tile, NPC_BET_VALUE_NAME])
            .u32();
        let width = float_trait(e, value_tile, 0xfa1);
        let width = x87_store32(x87_add(width, f64::from(widen)));
        set_float_bits(e, value_tile, 0xfa1, x87_quiet_bits(width));
    }
    let ante_tiles = tile(e, this, 0);
    grow_info_rect(e, ante_tiles, ANTE_INFO_RECT);
    let challenger = word_at(e, menu, CHALLENGER);
    let name = e.call(FULL_NAME, &args![challenger]).u32();
    let bet_tile = tile(e, this, 4);
    e.call(SET_STRING_TRAIT, &args![bet_tile, 0xfc4u32, name, 1u32]);
    poke_value(e, this, 4);
    let npc_ante = e.get(this, CaravanMenu::iNPCAnte) as u32;
    set_value(e, this, 4, npc_ante);
    poke_value(e, this, 5);
    let player_ante = e.get(this, CaravanMenu::iPlayerAnte) as u32;
    set_value(e, this, 5, player_ante);
    poke_value(e, this, 6);
    let funds = e.get(this, CaravanMenu::iTotalFunds) as u32;
    set_value(e, this, 6, funds);
    caravan_menu_update_caravan_flags(e, this);
    caravan_menu_update_tile_alpha(e, this, 0);
    e.with_stack(4, |e, counter| {
        for (list, count) in [(INT_LIST_BILLS, 9u32), (INT_LIST_COINS, 0x2a)] {
            for value in 0..count {
                e.mem.set_u32(counter.addr(), value);
                e.call(ARRAY_APPEND, &args![menu + list, counter]);
            }
        }
    });
    let limit = opponent_limit(e);
    let npc_ante = e.get(this, CaravanMenu::iNPCAnte);
    caravan_menu_update_betting_ui(e, this, limit, npc_ante);
}

/// The selection slot name in a track grid (`Select%i_0%i:0`).
const SELECT_SLOT_FORMAT: u32 = 0x0107_1aac;
/// The sounds of a valid and an invalid track selection.
const TOGGLE_TRACK_SOUND: u32 = 0x0107_1728;
const INVALID_STACK_SOUND: u32 = 0x0107_1a84;
/// The height the unused cursor is moved to (`float` -50.0 at `0104f7c4`).
const CURSOR_AWAY_HEIGHT: u32 = 0x0104_f7c4;
/// A quarter turn (`float` at `0101ff38`), the cursor's extra rotation.
const QUARTER_TURN: u32 = 0x0101_ff38;
/// Makes a rotation matrix about z (`this` = matrix, angle): `004a0c90`.
const MATRIX_Z_ROTATION: u32 = 0x004a_0c90;
/// The address of an object's local rotation (`006a9540`: `this + 0x34`).
const LOCAL_ROTATE: u32 = 0x006a_9540;
/// `NiMatrix3::operator*` (`this`, out, other; answers out).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// Sets an object's local rotation (`this`, matrix): `0043fa80`.
const SET_LOCAL_ROTATE: u32 = 0x0043_fa80;
/// Sets an object's local translation (`this`, vector): `00440460`.
const SET_LOCAL_TRANSLATE_VECTOR: u32 = 0x0044_0460;
/// The cursor children textured with the selected card (`Add:3`,
/// `Remove:3`).
const CURSOR_ADD_NAME: u32 = 0x0107_1aa4;
const CURSOR_REMOVE_NAME: u32 = 0x0107_1a78;

/// More `CaravanMenu` offsets (Xbox PDB names).
const TRACK_GRIDS: u32 = 0xe54;

// Translated from 0074e6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::SelectTrackCard` (Xbox PDB name from the call graph):
/// finds the selection slot `Select%i_0%i:0` (row + 1, the row's card count
/// at most 3, + 1) in the track's grid (`pTrackGrids[track]`); without one
/// does nothing. When the selected hand card may be placed there it plays
/// `GAMECaravanToggleTrack` and uses cursor 0 (cursor 1 moved away to z
/// -50.0), else `GAMECaravanInvalidStackL` and cursor 1 (cursor 0 moved
/// away). The cursor takes the slot's rotation times a quarter turn about
/// z, the slot's world bound centre as its translation and is updated; its
/// `Add:3` (valid) or `Remove:3` child then shows the selected card's
/// texture ([`apply_card_texture`]).
// Translation incomplete: 0074e6d0 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_select_track_card(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    track: i32,
    row: i32,
) {
    let menu = this.addr();
    e.with_stack(0x80, |e, frame| {
        let translate = frame.addr();
        let rotation = frame.addr() + 0x10;
        let turn = frame.addr() + 0x38;
        let product = frame.addr() + 0x5c;
        e.call(IDENTITY, &args![translate]);
        e.call(IDENTITY, &args![rotation]);
        let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard);
        let valid = caravan_menu_is_valid_card_placement(e, this, track, row, deck_card, 0);
        let mut count = array_size(e, card_track(menu, track as u32, row as u32)) as i32;
        if count > 3 {
            count = 3;
        }
        let slot = e.with_stack(0x104, |e, name| {
            e.call(
                PRINT_BOUNDED,
                &args![
                    name,
                    0x104u32,
                    SELECT_SLOT_FORMAT,
                    row.wrapping_add(1),
                    count.wrapping_add(1)
                ],
            );
            let grid = word_at(
                e,
                menu,
                TRACK_GRIDS.wrapping_add((track as u32).wrapping_mul(4)),
            );
            e.call(OBJECT_BY_NAME, &args![grid, name]).u32()
        });
        if slot == 0 {
            return;
        }
        let (sound, used, away, child_name) = if valid {
            (TOGGLE_TRACK_SOUND, CURSORS, CURSORS + 4, CURSOR_ADD_NAME)
        } else {
            (
                INVALID_STACK_SOUND,
                CURSORS + 4,
                CURSORS,
                CURSOR_REMOVE_NAME,
            )
        };
        play_sound(e, sound);
        let cursor = word_at(e, menu, used);
        let height = x87_quiet_bits(e.global::<u32>(CURSOR_AWAY_HEIGHT));
        let other = word_at(e, menu, away);
        set_translate(e, other, height);
        let other = word_at(e, menu, away);
        update_object(e, other);
        e.call(IDENTITY, &args![turn]);
        let angle = x87_quiet_bits(e.global::<u32>(QUARTER_TURN));
        e.call(MATRIX_Z_ROTATION, &args![turn, angle]);
        let source = e.call(LOCAL_ROTATE, &args![slot]).u32();
        for index in 0..9 {
            let value = e.mem.u32(source + 4 * index);
            e.mem.set_u32(rotation + 4 * index, value);
        }
        let result = e
            .call(MATRIX_MULTIPLY, &args![rotation, product, turn])
            .u32();
        for index in 0..9 {
            let value = e.mem.u32(result + 4 * index);
            e.mem.set_u32(rotation + 4 * index, value);
        }
        let bound = e.call(GET_WORLD_BOUND, &args![slot]).u32();
        let centre = e.call(IDENTITY, &args![bound]).u32();
        for index in 0..3 {
            let value = e.mem.u32(centre + 4 * index);
            e.mem.set_u32(translate + 4 * index, value);
        }
        e.call(SET_LOCAL_ROTATE, &args![cursor, rotation]);
        e.call(SET_LOCAL_TRANSLATE_VECTOR, &args![cursor, translate]);
        update_object(e, cursor);
        let child = e.call(OBJECT_BY_NAME, &args![cursor, child_name]).u32();
        apply_card_texture(e, child, |e| word_at(e, menu, SELECTED_CARD));
    });
}

/// The `Menu` constructor body (`this` only): `00a1c4a0`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body (`this` only): `00a1c520`.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// The constructor and destructor of the `BSSimpleArray<int,1024>`
/// instance (`intlistbills`, `intlistcoins`, `jokerLocations`).
const INT_ARRAY_CONSTRUCT: u32 = 0x0069_4de0;
const INT_ARRAY_DESTRUCT: u32 = 0x0069_4e10;
/// The compiler's `eh vector constructor iterator` (stdcall: array, element
/// size, count, constructor, destructor) and `eh vector destructor
/// iterator` (stdcall: array, element size, count, destructor).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `Tile::AddUserTrait` (cdecl: name, -1).
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// The names of the three user traits (`_current_value`, `_Value`, `_x`).
const CURRENT_VALUE_NAME: u32 = 0x0107_02d8;
const VALUE_NAME: u32 = 0x0106_ed18;
const X_NAME: u32 = 0x0106_ec90;
/// Sets up an empty animation list (8 bytes; `this` only, answers `this`):
/// `0096a2d0`.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `ModelLoader`'s model release by path (`this` = loader, path): `0045a5e0`.
const MODEL_RELEASE: u32 = 0x0045_a5e0;
/// Detaches a child (scene node slot +0xe8) and the scene node's light
/// refresh (`this` only): `00b5d180`.
const SCENE_REFRESH: u32 = 0x00b5_d180;
/// The number of children of a node (`00453470`, `this` only).
const CHILD_COUNT: u32 = 0x0045_3470;
/// `00572160` (`this` only), run on the money root when it has children.
const ROOT_RELEASE_00572160: u32 = 0x0057_2160;
/// The address of a pointer array's last element (`this` only): `006f2270`.
const ARRAY_LAST_SLOT: u32 = 0x006f_2270;
/// Removes a list's head (`this` only): `0063f7b0`.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// A list's scalar deleting destructor (`this`, flags): `004702f0`.
const LIST_DELETE: u32 = 0x0047_02f0;
/// `TESDataHandler::ClearBarterContainer` (`this` = data handler).
const CLEAR_BARTER_CONTAINER: u32 = 0x0046_faf0;
/// The bounded string copy (cdecl: buffer, size, text): `00406d30`.
const COPY_BOUNDED: u32 = 0x0040_6d30;
/// The deck models released when the deck menu was the last one.
const DECK_MENU_MODELS: [u32; 2] = [0x0107_121c, 0x0107_11e4];
/// The game models released by name when the game or results menu was the
/// last one, and those released as folder + name (`%s%s`).
const GAME_MENU_MODELS: [u32; 8] = [
    0x0107_11d0,
    0x0107_11bc,
    0x0107_11a8,
    0x0107_1198,
    0x0107_1188,
    0x0107_1178,
    0x0107_1168,
    0x0107_1154,
];
const GAME_MENU_FOLDER_MODELS: [u32; 4] = [0x0107_113c, 0x0107_1124, 0x0107_1114, 0x0107_1100];
/// The track models (folder, track + 1, row + 1) of the player's and the
/// opponent's tracks.
const PLAYER_TRACK_FORMAT: u32 = 0x0107_10ec;
const NPC_TRACK_FORMAT: u32 = 0x0107_10d4;

/// More `CaravanMenu` offsets (Xbox PDB names).
const INACTIVE_DECK_GEOMETRY: u32 = 0x188;
const ACTIVE_DECK_GEOMETRY: u32 = 0x18c;
const INACTIVE_DECK_ANIM_MANAGER: u32 = 0x254;
const INACTIVE_DECK_ANIMATIONS: u32 = 0x258;
const ACTIVE_DECK_ANIM_MANAGER: u32 = 0x270;
const ACTIVE_DECK_ANIMATIONS: u32 = 0x274;
const HAND_ANIM_MANAGER: u32 = 0x28c;
const TRACK_ANIM_MANAGER: u32 = 0x304;
const TRACK_ANIMATIONS: u32 = 0x3ac;
const DECK_FACE_SHADERS: u32 = 0x8ec;
const JOKER_LOCATIONS: u32 = 0xa7c;

// Translated from 0073b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CaravanMenu` constructor: the `Menu` part, the vtable, the arrays
/// (`pListOfCoins`, `pListOfBills`, `intlistbills`, `intlistcoins`,
/// `pDeckArray`, six `jokerLocations`, the four decks and hands and the 42
/// `pCardTracks`), the three user traits (`_current_value`, `_Value`, `_x`),
/// the starting values (`iTotalFunds` 100, `iCardsInDeck` 25, `iTotalCards`
/// 100, the rest 0, the byte at +0xe86 1), the flags (cleared, then
/// `CAF_DISABLE_ALLCONTROLS`) and two new animation lists
/// (`pListOfBillAnimations`, `pListOfCoinAnimations`; 0 when the
/// allocation fails). Returns `this`.
// Translation incomplete: 0073b7e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the built members is not translated
pub fn caravan_menu_constructor(e: &mut Engine, this: Ptr<CaravanMenu>) -> Ptr<CaravanMenu> {
    let menu = this.addr();
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(menu, CARAVAN_MENU_VTABLE);
    fn_00753070(e, Ptr::new(menu + LIST_OF_COINS));
    fn_00753070(e, Ptr::new(menu + LIST_OF_BILLS));
    e.call(INT_ARRAY_CONSTRUCT, &args![menu + INT_LIST_BILLS]);
    e.call(INT_ARRAY_CONSTRUCT, &args![menu + INT_LIST_COINS]);
    fn_007530c0(e, Ptr::new(menu + DECK_ARRAY));
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            menu + JOKER_LOCATIONS,
            0x10u32,
            6u32,
            INT_ARRAY_CONSTRUCT,
            INT_ARRAY_DESTRUCT
        ],
    );
    for deck in [PLAYER_DECK, NPC_DECK, PLAYER_HAND, NPC_HAND] {
        fn_007530c0(e, Ptr::new(menu + deck));
    }
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            menu + CARD_TRACKS,
            0x10u32,
            0x2au32,
            0x0075_30c0u32,
            0x0075_30f0u32
        ],
    );
    for (slot, name) in [
        (UT_CURRENT_VALUE, CURRENT_VALUE_NAME),
        (UT_VALUE, VALUE_NAME),
        (UT_X, X_NAME),
    ] {
        let id = e.call(TILE_ADD_USER_TRAIT, &args![name, -1i32]).u32();
        e.set_global(slot, id);
    }
    e.set(this, CaravanMenu::eGameState, 0);
    e.set(this, CaravanMenu::uiPrevIdleTick, 0);
    e.set(this, CaravanMenu::iPlayerAnte, 0);
    e.set(this, CaravanMenu::iNPCAnte, 0);
    e.set(this, CaravanMenu::iTotalFunds, 0x64);
    e.set(this, CaravanMenu::iCardsInDeck, 0x19);
    e.set(this, CaravanMenu::iTotalCards, 0x64);
    e.set(this, CaravanMenu::iSelectedDeckCard, 0);
    e.set(this, CaravanMenu::iSelectedTrack, 0);
    e.set(this, CaravanMenu::iSelectedRow, 0);
    e.set(this, CaravanMenu::iSelectedColumn, 0);
    e.set(this, CaravanMenu::iNumSteps, 0);
    e.mem.set_u8(menu + BYTE_E84, 0);
    e.mem.set_u8(menu + BYTE_E85, 0);
    e.set(this, CaravanMenu::bTutorial, 0);
    e.mem.set_u8(menu + BYTE_E86, 1);
    caravan_menu_clear_flags(e, this);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
    for target in [LIST_OF_BILL_ANIMATIONS, LIST_OF_COIN_ANIMATIONS] {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(menu + target, list);
    }
    this
}

/// `ModelLoader`'s release of the model at `path` (`0045a5e0`).
fn release_model(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(MODEL_RELEASE, &args![loader, path]);
}

/// Formats `words` into `buffer` (`00406d00(buffer, 0x104, ...)`) and
/// releases the model of that path.
fn release_formatted_model(e: &mut Engine, buffer: u32, words: &[u32]) {
    let mut call = vec![buffer, 0x104];
    call.extend_from_slice(words);
    e.call(PRINT_BOUNDED, &call);
    release_model(e, buffer);
}

/// Empties a pointer array from its end (`006f2270` gives the last slot, which is read;
/// `00753140` removes it) while its size is above 0.
fn empty_pointer_array(e: &mut Engine, array: u32) {
    while array_size(e, array) > 0 {
        let last = e.call(ARRAY_LAST_SLOT, &args![array]).u32();
        e.mem.u32(last);
        fn_00753140(e, Ptr::new(array));
    }
}

/// Empties the animation list at `offset` (removing heads while the list
/// and its head's data are set), deletes it (`004702f0(list, 1)`) and
/// clears the pointer.
fn delete_animation_list(e: &mut Engine, menu: u32, offset: u32) {
    loop {
        let list = word_at(e, menu, offset);
        if list == 0 {
            break;
        }
        let data = e.call(IDENTITY, &args![list]).u32();
        if e.mem.u32(data) == 0 {
            break;
        }
        let list = word_at(e, menu, offset);
        let data = e.call(IDENTITY, &args![list]).u32();
        e.mem.u32(data);
        let list = word_at(e, menu, offset);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    let list = word_at(e, menu, offset);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.mem.set_u32(menu + offset, 0);
}

/// `008454f0(array, 1)` when the array has elements.
fn release_if_filled(e: &mut Engine, array: u32) {
    if array_size(e, array) > 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![array, 1u32]);
    }
}

// Translated from 0073bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::~CaravanMenu` (Xbox PDB): stores the vtable, clears the
/// camera, releases the table model, detaches the money root from scene
/// node 1 (slot +0xe8) and refreshes it (`00b5d180`), runs `00572160` on
/// the root when `00453470` answers non-zero, clears the table, camera
/// manager and sequences, the root, `pMe`, the challenger and deck
/// parameter; releases the money models; empties the coin and bill arrays
/// and deletes their animation lists; releases `intlistcoins` and
/// `intlistbills`; clears the data handler's barter container. By the last
/// active menu: the deck menu releases its two models and clears the deck
/// objects, managers, sequences, `pDeckArray` and the 96 face shaders; the
/// game and results menus clear the hand managers and the track managers
/// and sequences, release the decks, hands and track rows that have cards,
/// the eight game models, the cursors, the four folder models, the hand
/// geometry and track grids, and each track row's player and opponent model
/// (clearing `pTrackGeometry`). Then destroys the members in reverse order
/// and runs the `Menu` destructor.
///
/// Two of the game-menu loops are kept as the exe has them: the
/// `pTrackAnimations` loop steps its row counter where the column counter
/// belongs (so only the first column of rows 0 to 3 is cleared), and the
/// opponent rows are cleared 0xc bytes past each row's start (the last
/// rows reach past `pTrackGeometry`). The exe's two empty loop nests (2 x 6
/// and 2 x 8) do nothing.
// Translation incomplete: 0073bba0 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_destructor(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    e.mem.set_u32(menu, CARAVAN_MENU_VTABLE);
    e.call(POINTER_ASSIGN, &args![CAMERA_POINTER, 0u32]);
    release_model(e, TABLE_MODEL_PATH);
    let scene = e.call(SCENE_NODE, &args![1u32]).u32();
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    e.vcall(scene, 0xe8, &args![root]);
    let scene = e.call(SCENE_NODE, &args![1u32]).u32();
    e.call(SCENE_REFRESH, &args![scene]);
    if e.call(FIRST_WORD, &args![MONEY_ROOT]).u32() != 0 {
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        if e.call(CHILD_COUNT, &args![root]).u32() != 0 {
            let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
            e.call(ROOT_RELEASE_00572160, &args![root]);
        }
    }
    e.mem.set_u32(menu + TABLE_GEOMETRY, 0);
    e.mem.set_u32(menu + CAMERA_ANIM_MANAGER, 0);
    for index in 0..4 {
        e.mem.set_u32(menu + CAMERA_ANIMATIONS + 4 * index, 0);
    }
    e.call(POINTER_ASSIGN, &args![MONEY_ROOT, 0u32]);
    e.set_global(CARAVAN_MENU, 0u32);
    e.mem.set_u32(menu + CHALLENGER, 0);
    e.mem.set_u32(menu + DECK_PARAM, 0);
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        release_formatted_model(e, buffer, &[BILL_GRID_FORMAT, MONEY_FOLDER]);
        release_formatted_model(e, buffer, &[COIN_GRID_FORMAT, MONEY_FOLDER]);
        e.mem.set_u32(menu + BILLS_GRID, 0);
        e.mem.set_u32(menu + COINS_GRID, 0);
        for index in 0..3u32 {
            release_formatted_model(e, buffer, &[BILL_FORMAT, MONEY_FOLDER, index + 1]);
            release_formatted_model(e, buffer, &[COIN_FORMAT, MONEY_FOLDER, index + 1]);
            e.mem.set_u32(menu + BILL_GEOMETRY + 4 * index, 0);
            e.mem.set_u32(menu + COIN_GEOMETRY + 4 * index, 0);
        }
    });
    empty_pointer_array(e, menu + LIST_OF_COINS);
    empty_pointer_array(e, menu + LIST_OF_BILLS);
    delete_animation_list(e, menu, LIST_OF_BILL_ANIMATIONS);
    delete_animation_list(e, menu, LIST_OF_COIN_ANIMATIONS);
    e.call(ARRAY_RELEASE_008454F0, &args![menu + INT_LIST_COINS, 1u32]);
    e.call(ARRAY_RELEASE_008454F0, &args![menu + INT_LIST_BILLS, 1u32]);
    let handler = e.global::<u32>(DATA_HANDLER);
    if handler != 0 {
        e.call(CLEAR_BARTER_CONTAINER, &args![handler]);
    }
    match e.get(this, CaravanMenu::eActiveMenu) {
        1 => {
            for path in DECK_MENU_MODELS {
                release_model(e, path);
            }
            e.mem.set_u32(menu + INACTIVE_DECK_GEOMETRY, 0);
            e.mem.set_u32(menu + ACTIVE_DECK_GEOMETRY, 0);
            e.mem.set_u32(menu + ACTIVE_DECK_ANIM_MANAGER, 0);
            e.mem.set_u32(menu + INACTIVE_DECK_ANIM_MANAGER, 0);
            for index in 0..6 {
                e.mem
                    .set_u32(menu + INACTIVE_DECK_ANIMATIONS + 4 * index, 0);
                e.mem.set_u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * index, 0);
            }
            release_if_filled(e, menu + DECK_ARRAY);
            for index in 0..0x60 {
                e.mem.set_u32(menu + DECK_FACE_SHADERS + 4 * index, 0);
            }
        }
        2 | 3 => {
            for index in 0..2 {
                e.mem.set_u32(menu + HAND_ANIM_MANAGER + 4 * index, 0);
            }
            for track in 0..6u32 {
                for row in 0..7u32 {
                    e.mem
                        .set_u32(menu + TRACK_ANIM_MANAGER + track * 0x1c + row * 4, 0);
                }
            }
            for track in 0..6u32 {
                let mut row = 0u32;
                while row < 7 {
                    let column = 0u32;
                    while row < 4 {
                        for lane in 0..2u32 {
                            let address = menu
                                + TRACK_ANIMATIONS
                                + track * 0xe0
                                + (row << 5)
                                + column * 8
                                + lane * 4;
                            e.mem.set_u32(address, 0);
                        }
                        // The exe steps the row counter here, not the column.
                        row += 1;
                    }
                    row += 1;
                }
            }
            for deck in [PLAYER_DECK, NPC_DECK, PLAYER_HAND, NPC_HAND] {
                release_if_filled(e, menu + deck);
            }
            for track in 0..6u32 {
                for row in 0..7u32 {
                    let array = card_track(menu, track, row);
                    if array_size(e, array) != 0 {
                        e.call(ARRAY_RELEASE_008454F0, &args![array, 1u32]);
                    }
                }
            }
            e.with_stack(0x104, |e, buffer| {
                let buffer = buffer.addr();
                for path in GAME_MENU_MODELS {
                    e.call(COPY_BOUNDED, &args![buffer, 0x104u32, path]);
                    release_model(e, buffer);
                }
                e.mem.set_u32(menu + CURSORS, 0);
                e.mem.set_u32(menu + CURSORS + 4, 0);
                for name in GAME_MENU_FOLDER_MODELS {
                    release_formatted_model(e, buffer, &[PATH_FORMAT, MONEY_FOLDER, name]);
                }
                e.mem.set_u32(menu + HAND_GEOMETRY, 0);
                e.mem.set_u32(menu + HAND_GEOMETRY + 4, 0);
                for index in 0..6 {
                    e.mem.set_u32(menu + TRACK_GRIDS + 4 * index, 0);
                }
                for track in 0..3u32 {
                    for row in 0..7u32 {
                        release_formatted_model(
                            e,
                            buffer,
                            &[PLAYER_TRACK_FORMAT, MONEY_FOLDER, track + 1, row + 1],
                        );
                        e.mem
                            .set_u32(menu + TRACK_GEOMETRY + track * 0x1c + row * 4, 0);
                        release_formatted_model(
                            e,
                            buffer,
                            &[NPC_TRACK_FORMAT, MONEY_FOLDER, track + 1, row + 1],
                        );
                        e.mem.set_u32(
                            menu + TRACK_GEOMETRY + (track + 3) * 0x1c + row * 4 + 0xc,
                            0,
                        );
                    }
                }
            });
        }
        _ => {}
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![menu + CARD_TRACKS, 0x10u32, 0x2au32, 0x0075_30f0u32],
    );
    for deck in [NPC_HAND, PLAYER_HAND, NPC_DECK, PLAYER_DECK] {
        fn_007530f0(e, Ptr::new(menu + deck));
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![menu + JOKER_LOCATIONS, 0x10u32, 6u32, INT_ARRAY_DESTRUCT],
    );
    fn_007530f0(e, Ptr::new(menu + DECK_ARRAY));
    e.call(INT_ARRAY_DESTRUCT, &args![menu + INT_LIST_COINS]);
    e.call(INT_ARRAY_DESTRUCT, &args![menu + INT_LIST_BILLS]);
    fn_007530a0(e, Ptr::new(menu + LIST_OF_BILLS));
    fn_007530a0(e, Ptr::new(menu + LIST_OF_COINS));
    e.call(MENU_DESTRUCTOR, &args![this]);
}

/// A form by its form ID (cdecl, one word; `0xf` the caps): `004839c0`.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// The form ID of the caps form.
const CAPS_FORM_ID: u32 = 0xf;
/// The container-entry stack object of the payout (12 bytes): set up
/// (`00481610`, `this` only), filled with a form and count (`004818e0`:
/// `this`, form, count, 0), given an actor (`004821a0`: `this`, actor, 0),
/// destroyed (`00481680`, `this` only).
const ENTRY_INIT: u32 = 0x0048_1610;
const ENTRY_FILL: u32 = 0x0048_18e0;
const ENTRY_SET_ACTOR: u32 = 0x0048_21a0;
const ENTRY_DESTROY: u32 = 0x0048_1680;
/// `ItemChange::ItemChange` (`this`, form, 0) and its destructor (`this`).
const ITEM_CHANGE_CONSTRUCT: u32 = 0x004b_c550;
const ITEM_CHANGE_DESTRUCT: u32 = 0x004b_c5f0;
/// `TESDataHandler::RemoveItemBarterContainer` (`this` = data handler,
/// entry, count, the word +0x20 of the actor): `0046f640`.
const REMOVE_BARTER_ITEM: u32 = 0x0046_f640;
/// The word at +0x20 of an actor (`007af430`).
const WORD_AT_0X20: u32 = 0x007a_f430;
/// `Actor::GetGoldAmount` (`this` only).
const ACTOR_GOLD: u32 = 0x0089_1d70;

/// The money entries of an inventory-change list (form type 0x74, positive
/// count): calls `pay(form, count, entry, value)` with each entry's count
/// lowered to `remaining / value` when it would pay more than `remaining`,
/// for counts above 0, and takes `count * value` off `remaining` after.
fn pay_from_entries(
    e: &mut Engine,
    changes: u32,
    remaining: &mut i32,
    mut pay: impl FnMut(&mut Engine, u32, i32, u32),
) {
    let mut node = e.mem.u32(changes);
    while node != 0 {
        let data = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(data) == 0 {
            break;
        }
        let data = e.call(IDENTITY, &args![node]).u32();
        let entry = e.mem.u32(data);
        let form = e.call(ARRAY_SIZE, &args![entry]).u32();
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == 0x74 {
            let data = e.call(IDENTITY, &args![node]).u32();
            let entry = e.mem.u32(data);
            if e.call(WORD_AT_FOUR, &args![entry]).i32() > 0 {
                let value = e.call(GET_FORM_VALUE, &args![form]).i32();
                let data = e.call(IDENTITY, &args![node]).u32();
                let entry = e.mem.u32(data);
                let mut count = e.call(WORD_AT_FOUR, &args![entry]).i32();
                if value.wrapping_mul(count) > *remaining {
                    // IDIV: a zero value (or i32::MIN / -1) faults as in the exe.
                    count = *remaining / value;
                }
                if count > 0 {
                    pay(e, form, count, node);
                    *remaining = remaining.wrapping_sub(count.wrapping_mul(value));
                }
            }
        }
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }
}

/// The `0x17c` slot call of an actor that hands over `count` of `form`
/// (`form, 0, count, 0, 0, 0, 0, 0, 1, 0`).
fn actor_remove_item(e: &mut Engine, actor: u32, form: u32, count: u32) {
    e.vcall(
        actor,
        0x17c,
        &args![form, 0u32, count, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 0074cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ante's payout after a game (no Xbox PDB name; `PrepareResultsMenu`
/// runs it with the win flag). Won: the challenger's items go into the
/// barter container and its money entries (form type 0x74) are moved from
/// it until `iAnte` is paid (each through a stack container entry given
/// the form, count and the player, then `RemoveItemBarterContainer`); the
/// rest is paid in caps (form 0xf): all of it when the challenger has more
/// caps, else its caps and the remainder through the challenger's slot
/// +0x17c. Lost: the same from the player's items to the challenger, each
/// payment through the player's slot +0x17c, the rest in caps (at most the
/// player's gold).
// Translation incomplete: 0074cc70 - the C++ exception-unwinding frame (FS:[0]) that destroys the stack entries is not translated
pub fn fn_0074cc70(e: &mut Engine, this: Ptr<CaravanMenu>, won: u8) {
    let menu = this.addr();
    let mut remaining = e.mem.i32(menu + ANTE);
    let handler = e.global::<u32>(DATA_HANDLER);
    if won != 0 {
        let challenger = word_at(e, menu, CHALLENGER);
        e.call(FILL_BARTER_CONTAINER, &args![handler, challenger]);
        let changes = fn_00740960(e, Ptr::new(e.global::<u32>(DATA_HANDLER)));
        pay_from_entries(e, changes, &mut remaining, |e, form, count, node| {
            e.with_stack(0xc, |e, entry| {
                e.call(ENTRY_INIT, &args![entry]);
                e.call(ENTRY_FILL, &args![entry, form, count, 0u32]);
                let player = e.global::<u32>(PLAYER);
                e.call(ENTRY_SET_ACTOR, &args![entry, player, 0u32]);
                let challenger = word_at(e, menu, CHALLENGER);
                let owner = e.call(WORD_AT_0X20, &args![challenger]).u32();
                let data = e.call(IDENTITY, &args![node]).u32();
                let item = e.mem.u32(data);
                let handler = e.global::<u32>(DATA_HANDLER);
                e.call(REMOVE_BARTER_ITEM, &args![handler, item, count, owner]);
                e.call(ENTRY_DESTROY, &args![entry]);
            });
        });
        if remaining != 0 {
            e.with_stack(0x18, |e, frame| {
                let entry = frame.addr();
                let change = frame.addr() + 0xc;
                e.call(ENTRY_INIT, &args![entry]);
                let caps = e.call(FORM_BY_ID, &args![CAPS_FORM_ID]).u32();
                e.call(ENTRY_FILL, &args![entry, caps, remaining, 0u32]);
                let player = e.global::<u32>(PLAYER);
                e.call(ENTRY_SET_ACTOR, &args![entry, player, 0u32]);
                let caps = e.call(FORM_BY_ID, &args![CAPS_FORM_ID]).u32();
                e.call(ITEM_CHANGE_CONSTRUCT, &args![change, caps, 0u32]);
                let gold = e.call(GET_GOLD_AMOUNT, &args![changes]).i32();
                if gold > remaining {
                    let challenger = word_at(e, menu, CHALLENGER);
                    let owner = e.call(WORD_AT_0X20, &args![challenger]).u32();
                    let handler = e.global::<u32>(DATA_HANDLER);
                    e.call(
                        REMOVE_BARTER_ITEM,
                        &args![handler, change, remaining, owner],
                    );
                } else {
                    let challenger = word_at(e, menu, CHALLENGER);
                    let owner = e.call(WORD_AT_0X20, &args![challenger]).u32();
                    let gold = e.call(GET_GOLD_AMOUNT, &args![changes]).u32();
                    let handler = e.global::<u32>(DATA_HANDLER);
                    e.call(REMOVE_BARTER_ITEM, &args![handler, change, gold, owner]);
                    let gold = e.call(GET_GOLD_AMOUNT, &args![changes]).i32();
                    let rest = remaining.wrapping_sub(gold) as u32;
                    let caps = e.call(FORM_BY_ID, &args![CAPS_FORM_ID]).u32();
                    let challenger = word_at(e, menu, CHALLENGER);
                    actor_remove_item(e, challenger, caps, rest);
                }
                e.call(ITEM_CHANGE_DESTRUCT, &args![change]);
                e.call(ENTRY_DESTROY, &args![entry]);
            });
        }
    } else {
        let player = e.global::<u32>(PLAYER);
        e.call(FILL_BARTER_CONTAINER, &args![handler, player]);
        let changes = fn_00740960(e, Ptr::new(e.global::<u32>(DATA_HANDLER)));
        pay_from_entries(e, changes, &mut remaining, |e, form, count, _node| {
            e.with_stack(0xc, |e, entry| {
                e.call(ENTRY_INIT, &args![entry]);
                e.call(ENTRY_FILL, &args![entry, form, count, 0u32]);
                let challenger = word_at(e, menu, CHALLENGER);
                e.call(ENTRY_SET_ACTOR, &args![entry, challenger, 0u32]);
                let player = e.global::<u32>(PLAYER);
                actor_remove_item(e, player, form, count as u32);
                e.call(ENTRY_DESTROY, &args![entry]);
            });
        });
        if remaining != 0 {
            e.with_stack(0xc, |e, entry| {
                e.call(ENTRY_INIT, &args![entry]);
                let caps = e.call(FORM_BY_ID, &args![CAPS_FORM_ID]).u32();
                e.call(ENTRY_FILL, &args![entry, caps, remaining, 0u32]);
                let challenger = word_at(e, menu, CHALLENGER);
                e.call(ENTRY_SET_ACTOR, &args![entry, challenger, 0u32]);
                let player = e.global::<u32>(PLAYER);
                let count = if e.call(ACTOR_GOLD, &args![player]).i32() > remaining {
                    remaining as u32
                } else {
                    let player = e.global::<u32>(PLAYER);
                    e.call(ACTOR_GOLD, &args![player]).u32()
                };
                let caps = e.call(FORM_BY_ID, &args![CAPS_FORM_ID]).u32();
                let player = e.global::<u32>(PLAYER);
                actor_remove_item(e, player, caps, count);
                e.call(ENTRY_DESTROY, &args![entry]);
            });
        }
    }
}

/// The address of a form's second texture name (`00503650`: `this + 0xa8`).
const BACK_TEXTURE_NAME_ADDRESS: u32 = 0x0050_3650;
/// The sixth hand slot object and the deck back object names (player,
/// opponent).
const PLAYER_SLOT_6_NAME: u32 = 0x0107_1928;
const NPC_SLOT_6_NAME: u32 = 0x0107_17a8;
const PLAYER_DECK_BACK_NAME: u32 = 0x0107_1914;
const NPC_DECK_BACK_NAME: u32 = 0x0107_1790;
/// The texture path format of the deck's back (`Textures\%s`).
const TEXTURES_FORMAT: u32 = 0x0106_57f8;
/// Sets an object's culled flag (`this`, flag): `00450f90`.
const SET_CULLED: u32 = 0x0045_0f90;

// Translated from 0074d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the textures of a side's hand and deck (no Xbox PDB name; the
/// player's with `player` set, else the opponent's), under the side's hand
/// geometry (`pHandGeometry[0]` or `[1]`): the object `Player-Deck_06:5` /
/// `Opponent-Deck_06:5` gets the texture of hand card 4 when the hand has
/// more than 4 cards (the player's card's name at
/// +0x9c, the opponent's at +0xa8); the deck's back object gets the
/// texture of the next deck card (`iNextPlayerCard` / `iNextNPCCard`, name
/// at +0xa8, `Textures\%s`) when it has a texture property and the deck
/// cards, else it is culled and updated.
// Translation incomplete: 0074d190 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_0074d190(e: &mut Engine, this: Ptr<CaravanMenu>, player: u8) {
    let menu = this.addr();
    let root = word_at(
        e,
        menu,
        if player != 0 {
            HAND_GEOMETRY
        } else {
            HAND_GEOMETRY + 4
        },
    );
    e.with_stack(0x20c, |e, frame| {
        let name = frame.addr();
        let path = frame.addr() + 0x104;
        let texture = frame.addr() + 0x208;
        let slot_name = if player != 0 {
            PLAYER_SLOT_6_NAME
        } else {
            NPC_SLOT_6_NAME
        };
        e.call(COPY_BOUNDED, &args![name, 0x104u32, slot_name]);
        let node = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
        let property = texture_property(e, node);
        e.call(POINTER_INIT, &args![texture, 0u32]);
        if property != 0 {
            let hand = menu + if player != 0 { PLAYER_HAND } else { NPC_HAND };
            if array_size(e, hand) > 4 {
                let card = array_element(e, hand, 4);
                let field = if player != 0 {
                    TEXTURE_NAME_ADDRESS
                } else {
                    BACK_TEXTURE_NAME_ADDRESS
                };
                let text = card_texture_text(e, field, card);
                e.call(
                    PRINT_BOUNDED,
                    &args![name, 0x104u32, PATH_FORMAT, TEXTURES_PREFIX, text],
                );
                load_property_texture(e, property, name, path, texture);
            }
        }
        let back = if player != 0 {
            PLAYER_DECK_BACK_NAME
        } else {
            NPC_DECK_BACK_NAME
        };
        e.call(COPY_BOUNDED, &args![name, 0x104u32, back]);
        let node = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
        let property = texture_property(e, node);
        e.call(POINTER_ASSIGN, &args![texture, 0u32]);
        let deck = menu + if player != 0 { PLAYER_DECK } else { NPC_DECK };
        if property != 0 && array_size(e, deck) != 0 {
            let next = word_at(
                e,
                menu,
                if player != 0 {
                    NEXT_PLAYER_CARD
                } else {
                    NEXT_NPC_CARD
                },
            );
            let card = array_element(e, deck, next);
            let text = card_texture_text(e, BACK_TEXTURE_NAME_ADDRESS, card);
            e.call(PRINT_BOUNDED, &args![name, 0x104u32, TEXTURES_FORMAT, text]);
            load_property_texture(e, property, name, path, texture);
        } else {
            cull_and_update(e, node);
        }
        e.call(POINTER_RELEASE, &args![texture]);
    });
}

/// Loads the texture of card `index` of `pDeckArray` (the name at `field`)
/// into the `NiPointer` slot `texture` and gives it to the face shader
/// `deckFaceShaders[group][slot]` (slot +0xf0 with 0, 0), the shader read
/// after the texture is made, as the exe does.
fn deck_face_texture(
    e: &mut Engine,
    menu: u32,
    slot: u32,
    group: u32,
    field: u32,
    index: u32,
    texture: u32,
) {
    e.with_stack(0x208, |e, frame| {
        let name = frame.addr();
        let path = frame.addr() + 0x104;
        let card = array_element(e, menu + DECK_ARRAY, index);
        let text = card_texture_text(e, field, card);
        e.call(
            PRINT_BOUNDED,
            &args![name, 0x104u32, PATH_FORMAT, TEXTURES_PREFIX, text],
        );
        e.call(SWAP_TEXTURE_PATH, &args![name, path, 0x104u32]);
        let tes = e.global::<u32>(TES);
        e.call(CREATE_TEXTURE_IMAGE, &args![tes, path, texture, 0u32, 0u32]);
        if e.call(FIRST_WORD, &args![texture]).u32() != 0 {
            let image = e.call(FIRST_WORD, &args![texture]).u32();
            let shader = word_at(e, menu, group.wrapping_add(slot.wrapping_mul(4)));
            e.vcall(shader, 0xf0, &args![0u32, 0u32, image]);
        }
    });
}

// Translated from 0074af70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the deck meter's card (`_current_value`, truncated, as the index
/// into `pDeckArray`) on deck slot `slot` (no Xbox PDB name): when the four
/// face-shader groups (`deckFaceShaders`, 24 each at +0x8ec, +0x94c,
/// +0x9ac, +0xa0c) have their entry 12, a caravan-deck card (byte +0x98)
/// gives groups +0x8ec and +0xa0c its texture name at +0x9c and +0x94c and
/// +0x9ac the one at +0xa8; another card (when its +0x9c address is not 0)
/// gives +0x9ac and +0x94c the +0x9c name and +0xa0c and +0x8ec the +0xa8
/// one.
// Translation incomplete: 0074af70 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_0074af70(e: &mut Engine, this: Ptr<CaravanMenu>, slot: u32) {
    let menu = this.addr();
    for group in [0x91c, 0x97c, 0x9dc, 0xa3c] {
        if word_at(e, menu, group) == 0 {
            return;
        }
    }
    let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
    let meter = tile(e, this, 0xf);
    let value = float_trait(e, meter, trait_id);
    let index = e.call(FTOL, &args![value]).u32();
    let card = array_element(e, menu + DECK_ARRAY, index);
    let order = if fn_0073cb80(e, Ptr::new(card)) != 0 {
        [
            (0x8ec, TEXTURE_NAME_ADDRESS),
            (0x94c, BACK_TEXTURE_NAME_ADDRESS),
            (0x9ac, BACK_TEXTURE_NAME_ADDRESS),
            (0xa0c, TEXTURE_NAME_ADDRESS),
        ]
    } else {
        let card = array_element(e, menu + DECK_ARRAY, index);
        if e.call(TEXTURE_NAME_ADDRESS, &args![card]).u32() == 0 {
            return;
        }
        [
            (0x9ac, TEXTURE_NAME_ADDRESS),
            (0xa0c, BACK_TEXTURE_NAME_ADDRESS),
            (0x8ec, BACK_TEXTURE_NAME_ADDRESS),
            (0x94c, TEXTURE_NAME_ADDRESS),
        ]
    };
    e.with_stack(4, |e, texture| {
        e.call(POINTER_INIT, &args![texture, 0u32]);
        for (group, field) in order {
            deck_face_texture(e, menu, slot, group, field, index, texture.addr());
        }
        e.call(POINTER_RELEASE, &args![texture]);
    });
}

/// Culls `node` (`00450f90(node, 1)`) and updates it with a zeroed stack
/// update-data block (`0043d410`, then `00a59c60(node, block)`).
fn cull_and_update(e: &mut Engine, node: u32) {
    e.call(SET_CULLED, &args![node, 1u32]);
    e.with_stack(0xc, |e, block| {
        e.call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32]);
        e.call(OBJECT_UPDATE, &args![node, block]);
    });
}

/// The opponent's hand card and shadow names (`Opponent-Deck_0%i:%i`,
/// `Opponent-Deck_0%i_Shadow:0`).
const NPC_HAND_CARD_FORMAT: u32 = 0x0107_1a40;
const NPC_HAND_SHADOW_FORMAT: u32 = 0x0107_1a08;

// Translated from 0074d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::UpdateGameHandTextures` (Xbox PDB): for the player's hand
/// (`player` set) or the opponent's, each of the eight card objects
/// (`Player-Deck_0%i:%i` / `Opponent-Deck_0%i:%i`) under the side's hand
/// geometry: a card in the hand gives its texture (`Textures\%s`; the
/// player's and, with the byte at `011d9184` set, the opponent's name at
/// +0x9c, else the opponent's at +0xa8); past the hand, while setting up
/// the tracks the object and its shadow are culled (except slot 5), and
/// outside the set-up they are culled when the hand has fewer than 5 cards.
/// Outside the set-up the slot-6 object then shows the next deck card's
/// back (+0xa8) when the deck has cards. The second word is not read.
// Translation incomplete: 0074d730 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_update_game_hand_textures(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    player: u8,
    _unused_2: u32,
) {
    let menu = this.addr();
    let hand = menu + if player != 0 { PLAYER_HAND } else { NPC_HAND };
    let count = array_size(e, hand) as i32;
    let root = word_at(
        e,
        menu,
        if player != 0 {
            HAND_GEOMETRY
        } else {
            HAND_GEOMETRY + 4
        },
    );
    e.with_stack(0x20c, |e, frame| {
        let name = frame.addr();
        let path = frame.addr() + 0x104;
        let texture = frame.addr() + 0x208;
        for index in 0..8i32 {
            let card_format = if player != 0 {
                HAND_CARD_FORMAT
            } else {
                NPC_HAND_CARD_FORMAT
            };
            e.call(
                PRINT_BOUNDED,
                &args![name, 0x104u32, card_format, index + 1, index],
            );
            let node = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
            if index < count {
                let property = texture_property(e, node);
                e.call(POINTER_INIT, &args![texture, 0u32]);
                if property != 0 {
                    let field = if player != 0 || e.mem.u8(CREATE_BYTE) != 0 {
                        TEXTURE_NAME_ADDRESS
                    } else {
                        BACK_TEXTURE_NAME_ADDRESS
                    };
                    let card = array_element(e, hand, index as u32);
                    let text = card_texture_text(e, field, card);
                    e.call(PRINT_BOUNDED, &args![name, 0x104u32, TEXTURES_FORMAT, text]);
                    load_property_texture(e, property, name, path, texture);
                }
                e.call(POINTER_RELEASE, &args![texture]);
                continue;
            }
            let culls = if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) && index != 5 {
                true
            } else {
                !caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) && count < 5
            };
            if culls {
                cull_and_update(e, node);
                let shadow_format = if player != 0 {
                    HAND_SHADOW_FORMAT
                } else {
                    NPC_HAND_SHADOW_FORMAT
                };
                e.call(
                    PRINT_BOUNDED,
                    &args![name, 0x104u32, shadow_format, index + 1],
                );
                let shadow = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
                cull_and_update(e, shadow);
            }
        }
        if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
            return;
        }
        let slot_name = if player != 0 {
            PLAYER_SLOT_6_NAME
        } else {
            NPC_SLOT_6_NAME
        };
        e.call(COPY_BOUNDED, &args![name, 0x104u32, slot_name]);
        let node = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
        let property = texture_property(e, node);
        e.call(POINTER_INIT, &args![texture, 0u32]);
        let deck = menu + if player != 0 { PLAYER_DECK } else { NPC_DECK };
        if property != 0 && array_size(e, deck) != 0 {
            let next = word_at(
                e,
                menu,
                if player != 0 {
                    NEXT_PLAYER_CARD
                } else {
                    NEXT_NPC_CARD
                },
            );
            let card = array_element(e, deck, next);
            let text = card_texture_text(e, BACK_TEXTURE_NAME_ADDRESS, card);
            e.call(PRINT_BOUNDED, &args![name, 0x104u32, TEXTURES_FORMAT, text]);
            load_property_texture(e, property, name, path, texture);
        }
        e.call(POINTER_RELEASE, &args![texture]);
    });
}

/// `double` 1.5 (`01016ff0`): the bill odds of the betting piles.
const BILL_ODDS: u32 = 0x0101_6ff0;
/// Removes elements from a `BSSimpleArray` (`this`, index, count):
/// `009a4320`.
const ARRAY_REMOVE: u32 = 0x009a_4320;
/// The placement names of the bills and coins in their grids.
const BILL_PLACEMENT_FORMAT: u32 = 0x0107_1998;
const COIN_PLACEMENT_FORMAT: u32 = 0x0107_1980;
const COIN_PLACEMENT_WIDE_FORMAT: u32 = 0x0107_196c;
/// `CaravanMenu::CaravanState::CA_ANTE_MONEYIN_ANIM` (Xbox PDB enum).
const CA_ANTE_MONEYIN_ANIM: i32 = 4;

/// One betting tier of [`caravan_menu_update_betting_ui`]: below which
/// limit it applies, whether its money goes to `pMe` (else `this`), the
/// lowest bill kind (`None`: always kind 2, chosen when the bill roll is
/// above the coin roll instead of by the odds), the two coin thresholds
/// and whether the coin kind is then rolled again in `[0, 2)`.
struct BettingTier {
    below: i32,
    on_singleton: bool,
    lowest_bill: Option<u32>,
    coin_thresholds: (i32, i32),
    reroll_coin: bool,
}

const BETTING_TIERS: [BettingTier; 5] = [
    BettingTier {
        below: 0x78,
        on_singleton: true,
        lowest_bill: Some(0),
        coin_thresholds: (0x5a, 0x28),
        reroll_coin: true,
    },
    BettingTier {
        below: 0x190,
        on_singleton: true,
        lowest_bill: Some(0),
        coin_thresholds: (0x4b, 0x21),
        reroll_coin: false,
    },
    BettingTier {
        below: 0x3e8,
        on_singleton: false,
        lowest_bill: Some(1),
        coin_thresholds: (0x42, 0x21),
        reroll_coin: false,
    },
    BettingTier {
        below: 0x7d0,
        on_singleton: false,
        lowest_bill: Some(1),
        coin_thresholds: (0x14, 5),
        reroll_coin: false,
    },
    BettingTier {
        below: i32::MAX,
        on_singleton: false,
        lowest_bill: None,
        coin_thresholds: (0xa, 5),
        reroll_coin: false,
    },
];

/// Places money objects `first..` of the array at `list` on random free
/// placements of the grid at `grid` (the free placement numbers are the
/// int array at `free`; one is taken out per object): each object takes the
/// placement's local rotation and its world bound centre as translation and
/// is updated. Stops when no placement is free.
fn place_money(
    e: &mut Engine,
    menu: u32,
    first: u32,
    list: u32,
    free: u32,
    grid: u32,
    name: impl Fn(u32) -> (u32, u32),
) {
    let mut index = first;
    while index < array_size(e, menu + list) {
        if array_size(e, menu + free) == 0 {
            break;
        }
        let count = array_size(e, menu + free);
        let pick = e.call(RANDOM_RANGE, &args![0u32, count]).u32();
        let number = array_element(e, menu + free, pick);
        e.call(ARRAY_REMOVE, &args![menu + free, pick, 1u32]);
        let (format, shown) = name(number);
        let node = e.with_stack(0x104, |e, buffer| {
            e.call(PRINT_BOUNDED, &args![buffer, 0x104u32, format, shown]);
            let root = word_at(e, menu, grid);
            e.call(OBJECT_BY_NAME, &args![root, buffer]).u32()
        });
        e.with_stack(0x3c, |e, frame| {
            let centre = frame.addr();
            let rotation = frame.addr() + 0xc;
            let update = frame.addr() + 0x30;
            let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
            for word in 0..3 {
                let value = e.mem.u32(bound + 4 * word);
                e.mem.set_u32(centre + 4 * word, value);
            }
            let source = e.call(LOCAL_ROTATE, &args![node]).u32();
            for word in 0..9 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(rotation + 4 * word, value);
            }
            let object = array_element(e, menu + list, index);
            e.call(SET_LOCAL_ROTATE, &args![object, rotation]);
            let x = x87_quiet_bits(e.mem.u32(centre));
            let y = x87_quiet_bits(e.mem.u32(centre + 4));
            let z = x87_quiet_bits(e.mem.u32(centre + 8));
            let object = array_element(e, menu + list, index);
            e.call(SET_LOCAL_TRANSLATE, &args![object, x, y, z]);
            let data = e
                .call(UPDATE_DATA_INIT, &args![update, 0u32, 0u32, 0u32])
                .u32();
            let object = array_element(e, menu + list, index);
            e.call(OBJECT_UPDATE, &args![object, data]);
        });
        index = index.wrapping_add(1);
    }
}

// Translated from 0074a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::UpdateBettingUI` (Xbox PDB name from the call graph):
/// builds the pile of money for `amount` with the tier of `limit` (below
/// 120, 400, 1000, 2000, or more): while money is left and fewer than 9
/// bills and 42 coins were made, a bill roll in `[0, 9 - bills)` and a coin
/// roll in `[0, 42 - coins)` choose a bill (the coin roll below 1.5 times
/// the bill roll; in the top tier the bill roll above the coin roll) of a
/// random kind (the top tier: kind 2) worth 2, 8 or 40, or else a coin of
/// the kind the roll in `[0, 100)` reaches (the lowest tier rolls the kind
/// again in `[0, 2)`) worth 1, 4 or 100 (`LoadNewBill`, `0074fc00`; the two
/// lowest tiers on `pMe`). The bills from `iBillIndexToAnim` and coins from
/// `iCoinIndexToAnim` on are placed on random free grid placements; with
/// any made the game state becomes `CA_ANTE_MONEYIN_ANIM`. Then the money
/// root is updated and its lights added (2.5 times its bound radius).
// Translation incomplete: 0074a2c0 - the /GS stack cookie check (00ec408c) is not translated
pub fn caravan_menu_update_betting_ui(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    limit: i32,
    amount: i32,
) {
    let menu = this.addr();
    let mut amount = amount;
    let mut bills = 0i32;
    let mut coins = 0i32;
    let tier = BETTING_TIERS
        .iter()
        .position(|tier| limit < tier.below)
        .unwrap_or(BETTING_TIERS.len() - 1);
    let tier = &BETTING_TIERS[tier];
    while amount > 0 && bills < 9 && coins < 0x2a {
        let bill_roll = e
            .call(RANDOM_RANGE, &args![0u32, 9i32.wrapping_sub(bills)])
            .i32();
        let coin_roll = e
            .call(RANDOM_RANGE, &args![0u32, 0x2ai32.wrapping_sub(coins)])
            .i32();
        let takes_bill = match tier.lowest_bill {
            // FCOMPP; TEST AH,5; JP: a bill when the coin roll is below.
            Some(_) => {
                f64::from(coin_roll) < x87_mul(f64::from(bill_roll), e.global::<f64>(BILL_ODDS))
            }
            None => bill_roll > coin_roll,
        };
        if takes_bill {
            let kind = match tier.lowest_bill {
                Some(lowest) => e.call(RANDOM_RANGE, &args![lowest, 3u32]).i32(),
                None => 2,
            };
            let target = if tier.on_singleton { me(e) } else { this };
            caravan_menu_load_new_bill(e, target, kind);
            let value = if kind < 2 {
                if kind != 0 {
                    8
                } else {
                    2
                }
            } else {
                0x28
            };
            amount = amount.wrapping_sub(value);
            bills += 1;
        } else {
            let roll = e.call(RANDOM_RANGE, &args![0u32, 0x64u32]).i32();
            let (high, low) = tier.coin_thresholds;
            let mut kind = if roll >= high {
                2
            } else {
                i32::from(roll >= low)
            };
            if tier.reroll_coin {
                kind = e.call(RANDOM_RANGE, &args![0u32, 2u32]).i32();
            }
            let target = if tier.on_singleton { me(e) } else { this };
            fn_0074fc00(e, target, kind);
            let value = if kind < 2 {
                if kind != 0 {
                    4
                } else {
                    1
                }
            } else {
                0x64
            };
            amount = amount.wrapping_sub(value);
            coins += 1;
        }
    }
    let first_bill = word_at(e, menu, BILL_INDEX_TO_ANIM);
    place_money(
        e,
        menu,
        first_bill,
        LIST_OF_BILLS,
        INT_LIST_BILLS,
        BILLS_GRID,
        |number| (BILL_PLACEMENT_FORMAT, number.wrapping_add(1)),
    );
    let first_coin = word_at(e, menu, COIN_INDEX_TO_ANIM);
    place_money(
        e,
        menu,
        first_coin,
        LIST_OF_COINS,
        INT_LIST_COINS,
        COINS_GRID,
        |number| {
            let shown = number.wrapping_add(1);
            if (shown as i32) < 0xa {
                (COIN_PLACEMENT_FORMAT, shown)
            } else {
                (COIN_PLACEMENT_WIDE_FORMAT, shown)
            }
        },
    );
    if e.call(MAX_OF, &args![coins, bills]).i32() > 0 {
        e.set(this, CaravanMenu::eGameState, CA_ANTE_MONEYIN_ANIM);
    }
    e.with_stack(0xc, |e, block| {
        e.call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32]);
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        e.call(OBJECT_UPDATE, &args![root, block]);
    });
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![root]).u32();
    let radius = e.call(BOUND_RADIUS, &args![bound]).f64();
    let radius = x87_store32(x87_mul(radius, e.global::<f64>(LIGHT_RADIUS_SCALE)));
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let scene = e.call(SCENE_NODE, &args![1u32]).u32();
    e.call(ADD_ALL_LIGHTS, &args![scene, root, x87_quiet_bits(radius)]);
}

/// The active and inactive deck models and their animation names.
const ACTIVE_DECK_MODEL: u32 = 0x0107_121c;
const INACTIVE_DECK_MODEL: u32 = 0x0107_11e4;
const ACTIVE_DECK_SEQUENCES: [u32; 6] = [
    0x0107_14c8,
    0x0107_14b4,
    0x0107_14a8,
    0x0107_1494,
    0x0107_1480,
    0x0107_1474,
];
const INACTIVE_DECK_SEQUENCES: [u32; 6] = [
    0x0107_145c,
    0x0107_1444,
    0x0107_1430,
    0x0107_1418,
    0x0107_1400,
    0x0107_13f0,
];
/// The deck slot object names (`Deck_0%i:%i`, `Deck_0%i_Back:0`,
/// `Available_0%i:%i`, `Available_0%i_Back:0`, and the forms without the
/// leading 0 from slot 9 on), their shadows (`Deck_0%i_Shadow:0`,
/// `Available_0%i_Shadow:0`, `Deck_%i_Shadow:0`, `Available_%i_Shadow:0`).
const DECK_FACE_FORMAT: u32 = 0x0107_13e4;
const DECK_BACK_FORMAT: u32 = 0x0107_13d4;
const AVAILABLE_FACE_FORMAT: u32 = 0x0107_13c0;
const AVAILABLE_BACK_FORMAT: u32 = 0x0107_13a8;
const DECK_FACE_WIDE_FORMAT: u32 = 0x0107_139c;
const DECK_BACK_WIDE_FORMAT: u32 = 0x0107_138c;
const AVAILABLE_FACE_WIDE_FORMAT: u32 = 0x0107_137c;
const AVAILABLE_BACK_WIDE_FORMAT: u32 = 0x0107_1368;
const DECK_SHADOW_FORMAT: u32 = 0x0107_19f4;
const AVAILABLE_SHADOW_FORMAT: u32 = 0x0107_19dc;
const DECK_SHADOW_WIDE_FORMAT: u32 = 0x0107_19c8;
const AVAILABLE_SHADOW_WIDE_FORMAT: u32 = 0x0107_19b0;
/// The deck menu's caption format (`%s %s`) and settings.
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
const DECK_TITLE_SETTING: u32 = 0x011d_2e68;
const DECK_SUBTITLE_SETTING: u32 = 0x011d_3d48;
const DECK_CAPTIONS: [(u32, u32); 7] = [
    (42, 0x011d_2e68),
    (43, 0x011d_2f70),
    (11, 0x011d_2160),
    (39, 0x011d_2ec8),
    (12, 0x011d_3ea4),
    (13, 0x011d_354c),
    (14, 0x011d_2784),
];
/// The button text trait (`_PCButtonText`) and the text it gets (`S)`).
const PC_BUTTON_TEXT_NAME: u32 = 0x0107_1358;
const PC_BUTTON_TEXT: u32 = 0x0106_33a0;
/// A float of a shader (`00526ae0`, `this` only; ST0), read and dropped.
const SHADER_VALUE_00526AE0: u32 = 0x0052_6ae0;

/// Formats `format` with `words` into a 0x19-byte stack buffer and finds
/// that object under the model at `root` (`004aae30`).
fn deck_object(e: &mut Engine, menu: u32, root: u32, format: u32, words: &[u32]) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        let mut call = vec![buffer.addr(), 0x19, format];
        call.extend_from_slice(words);
        e.call(PRINT_BOUNDED, &call);
        let model = word_at(e, menu, root);
        e.call(OBJECT_BY_NAME, &args![model, buffer]).u32()
    })
}

// Translated from 0073d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareDeckMenu` (Xbox PDB): loads the active and inactive
/// deck models, attaches them to the money root, releases `intlistbills`
/// and `intlistcoins`, updates the root, sets `iAnte` to the smaller ante,
/// keeps the two models' controller managers with their six sequences
/// each (deactivated, animations started), clears the 96 face shaders and
/// finds them again: for each of the 24 deck slots the texture property of
/// its face and back objects in the active model and in the inactive one.
/// Then shows the deck slots from 12 (`SwapDeckBuildingTextures(12, 0)`),
/// makes the deck menu active (state `CA_CAMERA_TO_DECK_ANIM`), shows the
/// card counts (tiles 9 and 8), the title (`%s %s` of two settings) with
/// its width less 150, the `_PCButtonText` unless `004b71d0()`, tile 11's
/// width less 150, only the deck tile set, the seven captions, hides tile
/// 39 and updates the flags.
// Translation incomplete: 0073d850 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_prepare_deck_menu(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    for (path, target) in [
        (ACTIVE_DECK_MODEL, ACTIVE_DECK_GEOMETRY),
        (INACTIVE_DECK_MODEL, INACTIVE_DECK_GEOMETRY),
    ] {
        let loader = e.global::<u32>(MODEL_LOADER);
        let model = e
            .call(
                MODEL_LOAD_FILE,
                &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(menu + target, model);
    }
    for model in [INACTIVE_DECK_GEOMETRY, ACTIVE_DECK_GEOMETRY] {
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        let child = word_at(e, menu, model);
        e.vcall(root, 0xdc, &args![child, 1u32]);
    }
    e.call(ARRAY_RELEASE_008454F0, &args![menu + INT_LIST_BILLS, 1u32]);
    e.call(ARRAY_RELEASE_008454F0, &args![menu + INT_LIST_COINS, 1u32]);
    e.with_stack(0xc, |e, block| {
        let data = e
            .call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32])
            .u32();
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        e.call(OBJECT_UPDATE, &args![root, data]);
    });
    let player_ante = e.get(this, CaravanMenu::iPlayerAnte);
    let npc_ante = e.get(this, CaravanMenu::iNPCAnte);
    let ante = e.call(MIN_OF, &args![player_ante, npc_ante]).u32();
    e.mem.set_u32(menu + ANTE, ante);
    for (model, manager_offset, first_sequence, names) in [
        (
            ACTIVE_DECK_GEOMETRY,
            ACTIVE_DECK_ANIM_MANAGER,
            ACTIVE_DECK_ANIMATIONS,
            ACTIVE_DECK_SEQUENCES,
        ),
        (
            INACTIVE_DECK_GEOMETRY,
            INACTIVE_DECK_ANIM_MANAGER,
            INACTIVE_DECK_ANIMATIONS,
            INACTIVE_DECK_SEQUENCES,
        ),
    ] {
        let object = word_at(e, menu, model);
        let manager = e
            .call(GET_CONTROLLER, &args![object, CONTROLLER_MANAGER_RTTI])
            .u32();
        if manager == 0 {
            continue;
        }
        e.mem.set_u32(menu + manager_offset, manager);
        for (index, name) in names.into_iter().enumerate() {
            let manager = word_at(e, menu, manager_offset);
            let sequence = sequence_by_name(e, manager, name);
            e.mem
                .set_u32(menu + first_sequence + 4 * index as u32, sequence);
        }
        let manager = word_at(e, menu, manager_offset);
        e.call(DEACTIVATE_ALL, &args![manager, 0u32]);
        let object = word_at(e, menu, model);
        e.call(START_ANIMATIONS, &args![object]);
    }
    for index in 0..0x60 {
        e.mem.set_u32(menu + DECK_FACE_SHADERS + 4 * index, 0);
    }
    for slot in 0..0x18u32 {
        let number = slot + 1;
        let lookups = if slot < 9 {
            [
                (ACTIVE_DECK_GEOMETRY, DECK_FACE_FORMAT, vec![number, slot]),
                (ACTIVE_DECK_GEOMETRY, DECK_BACK_FORMAT, vec![number]),
                (
                    INACTIVE_DECK_GEOMETRY,
                    AVAILABLE_FACE_FORMAT,
                    vec![number, slot],
                ),
                (INACTIVE_DECK_GEOMETRY, AVAILABLE_BACK_FORMAT, vec![number]),
            ]
        } else {
            [
                (
                    ACTIVE_DECK_GEOMETRY,
                    DECK_FACE_WIDE_FORMAT,
                    vec![number, slot],
                ),
                (ACTIVE_DECK_GEOMETRY, DECK_BACK_WIDE_FORMAT, vec![number]),
                (
                    INACTIVE_DECK_GEOMETRY,
                    AVAILABLE_FACE_WIDE_FORMAT,
                    vec![number, slot],
                ),
                (
                    INACTIVE_DECK_GEOMETRY,
                    AVAILABLE_BACK_WIDE_FORMAT,
                    vec![number],
                ),
            ]
        };
        // The exe formats the four names into four 0x19-byte buffers first,
        // then looks each one up.
        e.with_stack(0x64, |e, buffers| {
            for (index, (_, format, words)) in lookups.iter().enumerate() {
                let buffer = buffers.addr() + 0x19 * index as u32;
                let mut call = vec![buffer, 0x19, *format];
                call.extend_from_slice(words);
                e.call(PRINT_BOUNDED, &call);
            }
            for (index, ((root, _, _), group)) in lookups
                .iter()
                .zip([0x8ecu32, 0x94c, 0x9ac, 0xa0c])
                .enumerate()
            {
                let buffer = buffers.addr() + 0x19 * index as u32;
                let model = word_at(e, menu, *root);
                let node = e.call(OBJECT_BY_NAME, &args![model, buffer]).u32();
                let shader = texture_property(e, node);
                e.mem.set_u32(menu + group + 4 * slot, shader);
            }
        });
    }
    caravan_menu_swap_deck_building_textures(e, this, 0xc, 0);
    e.set(this, CaravanMenu::eActiveMenu, 1);
    e.set(this, CaravanMenu::eGameState, 1);
    poke_value(e, this, 9);
    let total = e.get(this, CaravanMenu::iTotalCards);
    let value_trait = e.global::<u32>(UT_VALUE);
    let total_tile = tile(e, this, 9);
    e.call(
        SET_TRAIT_FROM_UNSIGNED,
        &args![total_tile, value_trait, total],
    );
    poke_value(e, this, 8);
    let player = e.global::<u32>(PLAYER);
    let second = fn_0073cbc0(e, Ptr::new(player));
    let in_deck = e.call(CARD_LIST_COUNT, &args![second]).u32();
    let value_trait = e.global::<u32>(UT_VALUE);
    let deck_tile = tile(e, this, 8);
    e.call(
        SET_TRAIT_FROM_UNSIGNED,
        &args![deck_tile, value_trait, in_deck],
    );
    e.with_stack(0x104, |e, buffer| {
        let title = e.call(SETTING_STRING, &args![DECK_TITLE_SETTING]).u32();
        let subtitle = e.call(SETTING_STRING, &args![DECK_SUBTITLE_SETTING]).u32();
        e.call(
            PRINT_BOUNDED,
            &args![buffer, 0x104u32, TWO_STRINGS_FORMAT, subtitle, title],
        );
        let title_tile = tile(e, this, 10);
        e.call(SET_STRING_TRAIT, &args![title_tile, 0xfc4u32, buffer, 1u32]);
    });
    for (index, adds_button_text) in [(10u32, true), (11, false)] {
        let target = tile(e, this, index);
        let width = float_trait(e, target, 0xfa1);
        let width = x87_store32(x87_sub(width, e.global::<f64>(INFO_RECT_GROWTH)));
        let target = tile(e, this, index);
        set_float_bits(e, target, 0xfa1, x87_quiet_bits(width));
        let target = tile(e, this, index);
        e.call(TILE_ADD_NEEDS_UPDATE, &args![target, 1u32]);
        if adds_button_text && !e.call(KEYS_BLOCKED_004B71D0, &args![]).bool() {
            let text_trait = e
                .call(TILE_TEXT_TO_TRAIT, &args![PC_BUTTON_TEXT_NAME])
                .u32();
            let target = tile(e, this, index);
            e.call(
                SET_STRING_TRAIT,
                &args![target, text_trait, PC_BUTTON_TEXT, 1u32],
            );
        }
    }
    for (index, visible) in [(0u32, 0u32), (1, 1), (2, 0), (3, 0)] {
        let target = tile(e, this, index);
        e.call(SET_TRAIT_FROM_INT, &args![target, 0xfa3u32, visible]);
    }
    for (index, setting) in DECK_CAPTIONS {
        set_caption(e, this, index, setting);
    }
    let navigate = tile(e, this, 39);
    e.call(SET_TRAIT_FROM_INT, &args![navigate, 0xfa3u32, 0u32]);
    let navigate = tile(e, this, 39);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![navigate, 4u32]);
    caravan_menu_update_caravan_flags(e, this);
}

/// Moves a deck slot object to `x, y, z`; with `keep_xy` the object's own
/// x and y (its local translation, `0043c490`) are kept.
fn move_deck_object(e: &mut Engine, object: u32, keep_xy: bool, z_bits: u32) {
    let (x, y) = if keep_xy {
        let translate = e.call(LOCAL_TRANSLATE, &args![object]).u32();
        (
            x87_quiet_bits(e.mem.u32(translate)),
            x87_quiet_bits(e.mem.u32(translate + 4)),
        )
    } else {
        (0, 0)
    };
    e.call(SET_LOCAL_TRANSLATE, &args![object, x, y, z_bits]);
}

/// The slot objects `SwapDeckBuildingTextures` moves, in the exe's order:
/// (model, format, with the slot index as second word, the shadow pair that
/// keeps x and y on slots 11 and 12).
fn deck_slot_objects(slot: u32, hiding: bool) -> Vec<(u32, u32, bool, bool)> {
    if slot < 9 {
        let shadows_first = !hiding;
        let mut objects = vec![
            (ACTIVE_DECK_GEOMETRY, DECK_FACE_FORMAT, true, false),
            (ACTIVE_DECK_GEOMETRY, DECK_BACK_FORMAT, false, false),
        ];
        let available = [
            (INACTIVE_DECK_GEOMETRY, AVAILABLE_FACE_FORMAT, true, false),
            (INACTIVE_DECK_GEOMETRY, AVAILABLE_BACK_FORMAT, false, false),
        ];
        if shadows_first {
            objects.push((ACTIVE_DECK_GEOMETRY, DECK_SHADOW_FORMAT, false, false));
            objects.extend(available);
        } else {
            objects.extend(available);
            objects.push((ACTIVE_DECK_GEOMETRY, DECK_SHADOW_FORMAT, false, false));
        }
        objects.push((
            INACTIVE_DECK_GEOMETRY,
            AVAILABLE_SHADOW_FORMAT,
            false,
            false,
        ));
        objects
    } else {
        let keep = slot == 0xc || slot == 0xb;
        vec![
            (ACTIVE_DECK_GEOMETRY, DECK_FACE_WIDE_FORMAT, true, false),
            (ACTIVE_DECK_GEOMETRY, DECK_BACK_WIDE_FORMAT, false, false),
            (
                INACTIVE_DECK_GEOMETRY,
                AVAILABLE_FACE_WIDE_FORMAT,
                true,
                false,
            ),
            (
                INACTIVE_DECK_GEOMETRY,
                AVAILABLE_BACK_WIDE_FORMAT,
                false,
                false,
            ),
            (ACTIVE_DECK_GEOMETRY, DECK_SHADOW_WIDE_FORMAT, false, keep),
            (
                INACTIVE_DECK_GEOMETRY,
                AVAILABLE_SHADOW_WIDE_FORMAT,
                false,
                keep,
            ),
        ]
    }
}

// Translated from 0074b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::SwapDeckBuildingTextures` (Xbox PDB): for each of the 24
/// deck slots whose four face shaders are set, the card `first - (11 -
/// slot)` of `pDeckArray`: outside the deck (negative or not below
/// `iTotalCards`) the slot's six objects (faces, backs, shadows in the
/// active and inactive models) move to z 0 (`flag` set) or -50, else to z 0
/// with the card's textures on the four shaders (as `0074af70` orders
/// them; a card whose +0x9c name address is 0 gives its +0xa8 texture to
/// shaders +0x9ac and, made again from the same path, +0x8ec). On slots 11
/// and 12 the shadows keep their x and y. Both models are updated after
/// each slot; a shown slot also reads its +0x8ec shader's float
/// (`00526ae0`), which is dropped.
// Translation incomplete: 0074b700 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_swap_deck_building_textures(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    first: i32,
    flag: u8,
) {
    let menu = this.addr();
    for slot in 0..0x18u32 {
        let has_shaders = [0x8ecu32, 0x94c, 0x9ac, 0xa0c]
            .iter()
            .all(|&group| word_at(e, menu, group + 4 * slot) != 0);
        if !has_shaders {
            continue;
        }
        let card = first.wrapping_sub(0xb - slot as i32);
        let hidden = card < 0 || card as u32 >= e.get(this, CaravanMenu::iTotalCards);
        let z = if hidden {
            let height = if flag != 0 { 0 } else { -0x32i32 };
            x87_store32(f64::from(height))
        } else {
            0
        };
        for (root, format, with_index, keep_xy) in deck_slot_objects(slot, hidden) {
            let words = if with_index {
                vec![slot + 1, slot]
            } else {
                vec![slot + 1]
            };
            let object = deck_object(e, menu, root, format, &words);
            move_deck_object(e, object, keep_xy, z);
        }
        for model in [INACTIVE_DECK_GEOMETRY, ACTIVE_DECK_GEOMETRY] {
            let object = word_at(e, menu, model);
            update_object(e, object);
        }
        if hidden {
            continue;
        }
        let shader = word_at(e, menu, 0x8ec + 4 * slot);
        e.call(SHADER_VALUE_00526AE0, &args![shader]);
        let index = card as u32;
        let deck_card = array_element(e, menu + DECK_ARRAY, index);
        let order = if fn_0073cb80(e, Ptr::new(deck_card)) != 0 {
            Some([
                (0x8ec, TEXTURE_NAME_ADDRESS),
                (0x94c, BACK_TEXTURE_NAME_ADDRESS),
                (0x9ac, BACK_TEXTURE_NAME_ADDRESS),
                (0xa0c, TEXTURE_NAME_ADDRESS),
            ])
        } else {
            let deck_card = array_element(e, menu + DECK_ARRAY, index);
            if e.call(TEXTURE_NAME_ADDRESS, &args![deck_card]).u32() != 0 {
                Some([
                    (0x9ac, TEXTURE_NAME_ADDRESS),
                    (0xa0c, BACK_TEXTURE_NAME_ADDRESS),
                    (0x8ec, BACK_TEXTURE_NAME_ADDRESS),
                    (0x94c, TEXTURE_NAME_ADDRESS),
                ])
            } else {
                None
            }
        };
        e.with_stack(4, |e, texture| {
            e.call(POINTER_INIT, &args![texture, 0u32]);
            match order {
                Some(order) => {
                    for (group, field) in order {
                        deck_face_texture(e, menu, slot, group, field, index, texture.addr());
                    }
                }
                None => e.with_stack(0x208, |e, frame| {
                    let name = frame.addr();
                    let path = frame.addr() + 0x104;
                    let deck_card = array_element(e, menu + DECK_ARRAY, index);
                    let text = card_texture_text(e, BACK_TEXTURE_NAME_ADDRESS, deck_card);
                    e.call(
                        PRINT_BOUNDED,
                        &args![name, 0x104u32, PATH_FORMAT, TEXTURES_PREFIX, text],
                    );
                    e.call(SWAP_TEXTURE_PATH, &args![name, path, 0x104u32]);
                    for group in [0x9acu32, 0x8ec] {
                        let tes = e.global::<u32>(TES);
                        e.call(CREATE_TEXTURE_IMAGE, &args![tes, path, texture, 0u32, 0u32]);
                        if e.call(FIRST_WORD, &args![texture]).u32() != 0 {
                            let image = e.call(FIRST_WORD, &args![texture]).u32();
                            let shader = word_at(e, menu, group + 4 * slot);
                            e.vcall(shader, 0xf0, &args![0u32, 0u32, image]);
                        }
                    }
                }),
            }
            e.call(POINTER_RELEASE, &args![texture]);
        });
    }
}

/// `BSSimpleArray::Remove`-like removal (`this`, index, count): `006bf8f0`.
const ARRAY_REMOVE_AT: u32 = 0x006b_f8f0;
/// `QuantityMenu::Create` (cdecl: maximum, callback, limit).
const QUANTITY_MENU_CREATE: u32 = 0x007a_ba00;
/// `CaravanMenu::ItemSelectCallback`, the quantity menu's callback.
const ITEM_SELECT_CALLBACK: u32 = 0x0074_a090;
/// The sounds of `DoGamepad`.
const ADD_TO_TRACK_SOUND: u32 = 0x0107_1954;
const MENU_OK_SOUND: u32 = 0x0106_f380;
/// The objects culled when the player's deck runs out
/// (`Player-Deck_06_Back:0`, `Player-Deck_06:5` twice,
/// `Player-Deck_Back:0`, `Player-Deck_Back_Shadow:0`).
const EMPTY_DECK_OBJECTS: [u32; 5] = [
    0x0107_193c,
    0x0107_1928,
    0x0107_1928,
    0x0107_1914,
    0x0107_18f8,
];

/// `CaravanMenu::enumFlags` (Xbox PDB enum).
const CAF_AI: u32 = 0x4000;
const CAF_ISONTOP: u32 = 0x10000;
/// `CaravanMenu::CaravanState` values (Xbox PDB enum).
const CA_DECK_FLIP_ANIM: i32 = 7;
const CA_DECK_WIPE_ANIM: i32 = 0xa;
const CA_GAME_PLACECARD_ANIM: i32 = 0xd;
const CA_GAME_INTRO_DEAL_ANIM: i32 = 0xb;
const CA_GAME_DISCARDTRACK_ANIM: i32 = 0xf;

/// The results-menu test the gamepad cases share: in the results menu the
/// button counts as handled and closes the menu when it is on top.
fn results_close(e: &mut Engine, this: Ptr<CaravanMenu>, handled: &mut bool) {
    if e.get(this, CaravanMenu::eActiveMenu) == 3 {
        *handled = true;
        if is_top_menu(e, this) {
            caravan_menu_close(e);
        }
    }
}

/// Sets byte +0x98 of the deck meter's card (`_current_value`, truncated)
/// in `pDeckArray`.
fn mark_meter_card(e: &mut Engine, this: Ptr<CaravanMenu>, value: u32) {
    let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
    let meter = tile(e, this, 0xf);
    let current = float_trait(e, meter, trait_id);
    let index = e.call(FTOL, &args![current]).u32();
    let card = array_element(e, this.addr() + DECK_ARRAY, index);
    e.call(CARD_SET_BYTE_98, &args![card, value]);
}

/// Shows `iCardsInDeck` on tile 8 (`Tile::Poke` 1.0, then the value).
fn show_cards_in_deck(e: &mut Engine, this: Ptr<CaravanMenu>) {
    poke_value(e, this, 8);
    let in_deck = e.get(this, CaravanMenu::iCardsInDeck);
    let value_trait = e.global::<u32>(UT_VALUE);
    let deck_tile = tile(e, this, 8);
    e.call(
        SET_TRAIT_FROM_UNSIGNED,
        &args![deck_tile, value_trait, in_deck],
    );
}

/// Appends `value` to the array at `array` through a stack slot (the exe
/// passes the address of its local).
fn append_word(e: &mut Engine, array: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_APPEND, &args![array, slot]);
    });
}

/// The selected hand card (`pPlayerHand[iSelectedDeckCard]`), taken out of
/// the hand.
fn take_selected_hand_card(e: &mut Engine, this: Ptr<CaravanMenu>) -> u32 {
    let menu = this.addr();
    let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard) as u32;
    let card = array_element(e, menu + PLAYER_HAND, deck_card);
    let deck_card = e.get(this, CaravanMenu::iSelectedDeckCard) as u32;
    e.call(ARRAY_REMOVE_AT, &args![menu + PLAYER_HAND, deck_card, 1u32]);
    card
}

/// Replaces the selected hand card with the deck's `iNextPlayerCard`: the
/// next card is read, the hand card taken out, the next card appended to
/// the hand and removed from the deck. Answers the taken hand card.
fn draw_into_hand(e: &mut Engine, this: Ptr<CaravanMenu>) -> u32 {
    let menu = this.addr();
    let next_index = word_at(e, menu, NEXT_PLAYER_CARD);
    let next = array_element(e, menu + PLAYER_DECK, next_index);
    let card = take_selected_hand_card(e, this);
    append_word(e, menu + PLAYER_HAND, next);
    let next_index = word_at(e, menu, NEXT_PLAYER_CARD);
    e.call(
        ARRAY_REMOVE_AT,
        &args![menu + PLAYER_DECK, next_index, 1u32],
    );
    card
}

/// Picks the next deck card (-1 for an empty deck, else random in the
/// deck) and refreshes the player's hand and deck textures (`0074d190`).
fn next_player_card(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    if array_size(e, menu + PLAYER_DECK) == 0 {
        e.mem.set_i32(menu + NEXT_PLAYER_CARD, -1);
    } else {
        let size = array_size(e, menu + PLAYER_DECK);
        let next = e.call(RANDOM_RANGE, &args![0u32, size]).u32();
        e.mem.set_u32(menu + NEXT_PLAYER_CARD, next);
    }
    fn_0074d190(e, this, 1);
}

/// `pCardTracks[iSelectedTrack][iSelectedRow]`.
fn selected_track_row(e: &Engine, this: Ptr<CaravanMenu>) -> u32 {
    let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
    let row = e.get(this, CaravanMenu::iSelectedRow) as u32;
    card_track(this.addr(), track, row)
}

/// Button 9 (A) of [`caravan_menu_do_gamepad`] after its flag test. Answers
/// true when the exe answers 1 at once (a track was discarded).
fn gamepad_accept(e: &mut Engine, this: Ptr<CaravanMenu>, handled: &mut bool) -> bool {
    let menu = this.addr();
    results_close(e, this, handled);
    match e.get(this, CaravanMenu::eActiveMenu) {
        0 => {
            let old = e.get(this, CaravanMenu::iPlayerAnte);
            let npc_ante = e.get(this, CaravanMenu::iNPCAnte);
            let funds = e.get(this, CaravanMenu::iTotalFunds);
            let ante = e.call(MIN_OF, &args![npc_ante, funds]).i32();
            e.set(this, CaravanMenu::iPlayerAnte, ante);
            poke_value(e, this, 5);
            let ante = e.get(me(e), CaravanMenu::iPlayerAnte) as u32;
            set_value(e, this, 5, ante);
            *handled = true;
            let limit = opponent_limit(e);
            let change = e.get(this, CaravanMenu::iPlayerAnte).wrapping_sub(old);
            caravan_menu_update_betting_ui(e, this, limit, change);
            caravan_menu_update_caravan_flags(e, this);
            caravan_menu_update_tile_alpha(e, this, -1);
        }
        1 => {
            let in_deck = e.get(this, CaravanMenu::iCardsInDeck).wrapping_add(1);
            e.set(this, CaravanMenu::iCardsInDeck, in_deck);
            show_cards_in_deck(e, this);
            mark_meter_card(e, this, 1);
            *handled = true;
            e.set(this, CaravanMenu::eGameState, CA_DECK_FLIP_ANIM);
        }
        2 => {
            if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP)
                && !caravan_menu_has_flag(e, this, CAF_SELECT_TRACK)
            {
                e.set(this, CaravanMenu::iSelectedTrack, 0);
                e.set(this, CaravanMenu::iSelectedRow, 0);
                e.set(this, CaravanMenu::iSelectedColumn, 0);
                loop {
                    let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
                    if array_size(e, card_track(menu, track, 0)) == 0 {
                        break;
                    }
                    let track = e.get(this, CaravanMenu::iSelectedTrack);
                    e.set(this, CaravanMenu::iSelectedTrack, track.wrapping_add(1));
                }
                if e.get(this, CaravanMenu::iSelectedTrack) < 3 {
                    let track = e.get(this, CaravanMenu::iSelectedTrack);
                    let row = e.get(this, CaravanMenu::iSelectedRow);
                    caravan_menu_select_track_card(e, this, track, row);
                }
                caravan_menu_select_deck_card(e, this, 1);
                play_sound(e, ADD_TO_TRACK_SOUND);
                caravan_menu_update_tile_alpha(e, this, -1);
                caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 1);
                *handled = true;
            } else if !caravan_menu_has_flag(e, this, CAF_SELECT_TRACK) {
                play_sound(e, ADD_TO_TRACK_SOUND);
                caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 1);
                e.set(this, CaravanMenu::iSelectedTrack, 0);
                e.set(this, CaravanMenu::iSelectedRow, 0);
                e.set(this, CaravanMenu::iSelectedColumn, 0);
                caravan_menu_handle_track_selection(e, this, 0, 0);
                *handled = true;
            } else if caravan_menu_has_flag(e, this, CAF_DESTROY_TRACK) {
                let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
                caravan_menu_deselect_whole_track(e, this, track);
                let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
                e.mem.set_u32(track_info(menu, track) + 8, 0);
                e.set(this, CaravanMenu::eGameState, CA_GAME_DISCARDTRACK_ANIM);
                return true;
            } else if caravan_menu_has_flag(e, this, CAF_SELECT_TRACK) {
                if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
                    caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 0);
                    caravan_menu_deselect_track_card(e, this);
                    e.set(this, CaravanMenu::eGameState, CA_GAME_INTRO_DEAL_ANIM);
                    let card = take_selected_hand_card(e, this);
                    let row = selected_track_row(e, this);
                    append_word(e, row, card);
                } else {
                    caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 0);
                    caravan_menu_deselect_track_card(e, this);
                    e.set(this, CaravanMenu::eGameState, CA_GAME_PLACECARD_ANIM);
                    if e.mem.i32(menu + NEXT_PLAYER_CARD) == -1 {
                        for name in EMPTY_DECK_OBJECTS {
                            let hand = word_at(e, menu, HAND_GEOMETRY);
                            let node = e.call(OBJECT_BY_NAME, &args![hand, name]).u32();
                            if node != 0 {
                                cull_and_update(e, node);
                            }
                        }
                    }
                    if e.mem.i32(menu + NEXT_PLAYER_CARD) != -1 {
                        let card = draw_into_hand(e, this);
                        let row = selected_track_row(e, this);
                        append_word(e, row, card);
                        next_player_card(e, this);
                    } else {
                        let card = take_selected_hand_card(e, this);
                        let row = selected_track_row(e, this);
                        append_word(e, row, card);
                    }
                }
                *handled = true;
            }
        }
        _ => {}
    }
    false
}

// Translated from 00747d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoGamepad` (vtable slot +0x38, Xbox PDB). With the
/// controls disabled, a tutorial up or the menu not on top
/// (`CAF_ISONTOP`) it answers whether the state is
/// `CA_CAMERA_TO_RESULTS_ANIM`; during the AI's turn or any animation it
/// answers 0. Otherwise by button (each refused, answering 0, while its
/// disable flag is set; the results menu closes when on top for every
/// handled button):
/// - 9 (A): ante: bet up to the opponent's ante (at most the funds) and show
///   the pile; deck: add the meter's card to the deck; game: start the track
///   set-up selection, start selecting a track, discard the selected track
///   (answers at once), or place the selected hand card on the selected
///   track row (drawing the next deck card into the hand);
/// - 11 (X): ante: disable the controls and ask for an amount
///   (`QuantityMenu`, `ItemSelectCallback`) when there are funds left above
///   the bet; deck: take the meter's card out of the deck;
/// - 12 (Y): ante: go to the deck menu (with a bet, or when the opponent
///   bet nothing) and play `UIMenuOK`; deck: wipe with 30 cards or more;
/// - 15 (LB), 16 (RB): deck: handled; RB also deals a random caravan deck
///   (30 cards, or a random number from 30 up to all of them);
/// - 13 (LT): game, not selecting a track and with cards in the deck:
///   discard the selected hand card (state `0xc` while setting up, else
///   `0xe`), drawing the next card when there is one;
/// - 14 (RT): game, not selecting and not setting up: select the first
///   track for destroying;
/// - 5, 6, 17, 18: only the results test.
///
/// Then the tile alphas are updated. The float word is not read.
// Translation incomplete: 00747d30 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary array and sound handles is not translated
pub fn caravan_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let menu = this.addr();
    let mut handled = false;
    if caravan_menu_has_flag(e, this, CAF_DISABLE_ALLCONTROLS)
        || e.get(this, CaravanMenu::bTutorial) != 0
        || !caravan_menu_has_flag(e, this, CAF_ISONTOP)
    {
        return e.get(this, CaravanMenu::eGameState) == 3;
    }
    if caravan_menu_has_flag(e, this, CAF_AI) || e.get(this, CaravanMenu::eGameState) != 0 {
        return false;
    }
    match button {
        9 => {
            if caravan_menu_has_flag(e, this, CAF_DISABLE_A) {
                return false;
            }
            if gamepad_accept(e, this, &mut handled) {
                return true;
            }
        }
        11 => {
            if caravan_menu_has_flag(e, this, CAF_DISABLE_X) {
                return false;
            }
            results_close(e, this, &mut handled);
            match e.get(this, CaravanMenu::eActiveMenu) {
                0 => {
                    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
                    caravan_menu_update_caravan_flags(e, this);
                    caravan_menu_update_tile_alpha(e, this, -1);
                    let left = e
                        .get(this, CaravanMenu::iTotalFunds)
                        .wrapping_sub(e.get(this, CaravanMenu::iPlayerAnte));
                    if left > 0 {
                        e.call(
                            QUANTITY_MENU_CREATE,
                            &args![left, ITEM_SELECT_CALLBACK, 0x7fff_ffffu32],
                        );
                    }
                    handled = true;
                }
                1 => {
                    let in_deck = e.get(this, CaravanMenu::iCardsInDeck).wrapping_sub(1);
                    e.set(this, CaravanMenu::iCardsInDeck, in_deck);
                    show_cards_in_deck(e, this);
                    mark_meter_card(e, this, 0);
                    handled = true;
                    e.set(this, CaravanMenu::eGameState, CA_DECK_FLIP_ANIM);
                }
                _ => {}
            }
        }
        12 => {
            if caravan_menu_has_flag(e, this, CAF_DISABLE_Y) {
                return false;
            }
            results_close(e, this, &mut handled);
            match e.get(this, CaravanMenu::eActiveMenu) {
                0 => {
                    if e.get(this, CaravanMenu::iPlayerAnte) > 0
                        || e.get(this, CaravanMenu::iNPCAnte) == 0
                    {
                        caravan_menu_prepare_deck_menu(e, this);
                        play_sound(e, MENU_OK_SOUND);
                    }
                    handled = true;
                }
                1 => {
                    if e.get(this, CaravanMenu::iCardsInDeck) >= 0x1e {
                        e.set(this, CaravanMenu::eGameState, CA_DECK_WIPE_ANIM);
                    }
                    handled = true;
                }
                _ => {}
            }
        }
        15 => {
            results_close(e, this, &mut handled);
            if caravan_menu_has_flag(e, this, CAF_DISABLE_LB) {
                return false;
            }
            if e.get(this, CaravanMenu::eActiveMenu) == 1 {
                handled = true;
            }
        }
        16 => {
            results_close(e, this, &mut handled);
            if caravan_menu_has_flag(e, this, CAF_DISABLE_RB) {
                return false;
            }
            if e.get(this, CaravanMenu::eActiveMenu) == 1 {
                gamepad_random_deck(e, this);
                handled = true;
            }
        }
        13 => {
            results_close(e, this, &mut handled);
            if caravan_menu_has_flag(e, this, CAF_DISABLE_LT) {
                return false;
            }
            let active = e.get(this, CaravanMenu::eActiveMenu);
            if active == 2
                && !caravan_menu_has_flag(e, this, CAF_SELECT_TRACK)
                && array_size(e, menu + PLAYER_DECK) > 0
            {
                let state = if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
                    0xc
                } else {
                    0xe
                };
                e.set(this, CaravanMenu::eGameState, state);
                if e.mem.i32(menu + NEXT_PLAYER_CARD) != -1 {
                    draw_into_hand(e, this);
                    next_player_card(e, this);
                } else {
                    take_selected_hand_card(e, this);
                }
            }
        }
        14 => {
            results_close(e, this, &mut handled);
            if caravan_menu_has_flag(e, this, CAF_DISABLE_RT) {
                return false;
            }
            let active = e.get(this, CaravanMenu::eActiveMenu);
            if active == 2
                && !caravan_menu_has_flag(e, this, CAF_SELECT_TRACK)
                && !caravan_menu_has_flag(e, this, CAF_TRACK_SETUP)
            {
                caravan_menu_set_flag(e, this, CAF_DESTROY_TRACK, 1);
                caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 1);
                e.set(this, CaravanMenu::iSelectedTrack, 0);
                let track = e.get(this, CaravanMenu::iSelectedTrack) as u32;
                caravan_menu_select_whole_track(e, this, track);
            }
        }
        5 | 6 | 17 | 18 => results_close(e, this, &mut handled),
        _ => {}
    }
    caravan_menu_update_tile_alpha(e, this, -1);
    handled
}

/// Button 16 (RB) in the deck menu: takes every card out of the caravan
/// deck (byte +0x98 cleared, `iCardsInDeck` 0) and puts back a random
/// choice: 30 draws (or a random number from 30 up to the card count when
/// there are more than 30) from a temporary `BSSimpleArray<int>` of the
/// indexes, each drawn index taken out and its card, unless already in,
/// added (`iCardsInDeck` counted). Then the count is shown, the flags and
/// alphas updated and the deck slots shown from the meter's card.
fn gamepad_random_deck(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    e.with_stack(0x18, |e, frame| {
        let indexes = frame.addr();
        let counter = frame.addr() + 0x10;
        e.call(INT_ARRAY_CONSTRUCT, &args![indexes]);
        let count = array_size(e, menu + DECK_ARRAY) as i32;
        let mut index = 0i32;
        while index < count {
            e.mem.set_i32(counter, index);
            e.call(ARRAY_APPEND, &args![indexes, counter]);
            let card = array_element(e, menu + DECK_ARRAY, index as u32);
            e.call(CARD_SET_BYTE_98, &args![card, 0u32]);
            e.set(this, CaravanMenu::iCardsInDeck, 0);
            index += 1;
        }
        let draws = if count > 0x1e {
            e.call(RANDOM_RANGE, &args![0x1eu32, count]).i32()
        } else {
            0x1e
        };
        for _ in 0..draws.max(0) {
            let size = array_size(e, indexes);
            let pick = e.call(RANDOM_RANGE, &args![0u32, size]).u32();
            let drawn = array_element(e, indexes, pick);
            e.call(ARRAY_REMOVE_AT, &args![indexes, pick, 1u32]);
            let card = array_element(e, menu + DECK_ARRAY, drawn);
            if fn_0073cb80(e, Ptr::new(card)) == 0 {
                let in_deck = e.get(this, CaravanMenu::iCardsInDeck).wrapping_add(1);
                e.set(this, CaravanMenu::iCardsInDeck, in_deck);
                let card = array_element(e, menu + DECK_ARRAY, drawn);
                e.call(CARD_SET_BYTE_98, &args![card, 1u32]);
            }
        }
        e.call(ARRAY_RELEASE_008454F0, &args![indexes, 1u32]);
        show_cards_in_deck(e, this);
        caravan_menu_update_caravan_flags(e, this);
        caravan_menu_update_tile_alpha(e, this, -1);
        let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
        let meter = tile(e, this, 0xf);
        let current = float_trait(e, meter, trait_id);
        let first = e.call(FTOL, &args![current]).i32();
        caravan_menu_swap_deck_building_textures(e, this, first, 0);
        e.call(INT_ARRAY_DESTRUCT, &args![indexes]);
    });
}

/// The game models: the track grids (folder + number; player and
/// opponent), the cursors and the selection height of the cursors (`float`
/// at `0102caf8`).
const PLAYER_GRID_FORMAT: u32 = 0x0107_1670;
const NPC_GRID_FORMAT: u32 = 0x0107_165c;
const CURSOR_ADD_MODEL_FORMAT: u32 = 0x0107_1648;
const CURSOR_REMOVE_MODEL_FORMAT: u32 = 0x0107_1634;
const CURSOR_START_HEIGHT: u32 = 0x0102_caf8;
/// The hand animation names (six per hand) and the start animation names
/// (eight per hand).
const HAND_SEQUENCES: [u32; 6] = [
    0x0107_1624,
    0x0107_1614,
    0x0107_1604,
    0x0107_15f4,
    0x0107_15e4,
    0x0107_15d4,
];
const START_SEQUENCES: [u32; 8] = [
    0x0107_15c4,
    0x0107_15b4,
    0x0107_15a4,
    0x0107_1594,
    0x0107_1584,
    0x0107_1574,
    0x0107_1564,
    0x0107_1554,
];
/// The track animation names with their offsets in a track row's eight
/// sequences.
const TRACK_SEQUENCES: [(u32, u32); 8] = [
    (0x0107_1548, 0),
    (0x0107_153c, 8),
    (0x0107_1530, 0x10),
    (0x0107_1524, 0x18),
    (0x0107_1518, 4),
    (0x0107_150c, 0xc),
    (0x0107_1500, 0x14),
    (0x0107_14f4, 0x1c),
];
/// The game menu's info rectangle, its button setting and its captions.
const GAME_INFO_RECT: u32 = 0x0107_14dc;
const GAME_BUTTON_SETTING: u32 = 0x011d_1fec;
const GAME_CAPTIONS: [(u32, u32); 7] = [
    (44, 0x011d_2964),
    (45, 0x011d_24b4),
    (46, 0x011d_41f8),
    (19, 0x011d_4204),
    (20, 0x011d_4d80),
    (21, 0x011d_4198),
    (22, 0x011d_29f4),
];
/// `CaravanMenu::CaravanState::CA_CAMERA_TO_GAME_ANIM` (Xbox PDB enum).
const CA_CAMERA_TO_GAME_ANIM: i32 = 2;
/// More `CaravanMenu` offsets (Xbox PDB names).
const HAND_ANIMATIONS: u32 = 0x294;
const START_ANIMATIONS_OFFSET: u32 = 0x2c4;

/// Loads the model at the path formatted from `words` (`00406d00` into a
/// 0x104-byte stack buffer, then `ModelLoader::LoadFile(.., 0, 1, 0, 0,
/// 0)`).
fn load_formatted_model(e: &mut Engine, words: &[u32]) -> u32 {
    e.with_stack(0x104, |e, buffer| {
        let mut call = vec![buffer.addr(), 0x104];
        call.extend_from_slice(words);
        e.call(PRINT_BOUNDED, &call);
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOAD_FILE,
            &args![loader, buffer, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32()
    })
}

/// Attaches the object at `offset` to the money root (slot +0xdc, flag 1).
fn attach_to_root(e: &mut Engine, menu: u32, offset: u32) {
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let child = word_at(e, menu, offset);
    e.vcall(root, 0xdc, &args![child, 1u32]);
}

/// Detaches the money root's last child (slot +0xf0 with the child count
/// less one).
fn detach_last_child(e: &mut Engine) {
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let root_again = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let count = e.call(CHILD_COUNT, &args![root_again]).u32();
    e.vcall(root, 0xf0, &args![count.wrapping_sub(1)]);
}

// Translated from 0073ea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::PrepareGameMenu` (Xbox PDB): saves the deck building
/// (the player's two card lists emptied and refilled from `pDeckArray`, the
/// caravan-deck cards also appended to `pPlayerDeck`), fills `pNPCDeck`
/// from the deck parameter's list, deals eight random cards to each hand,
/// picks each side's next card, clears `pDeckArray`, the face shaders and
/// the deck managers and sequences, detaches the deck models and releases
/// them, loads and attaches the 42 track row models, the six track grids
/// (culled), the two cursors (moved to the height at `0102caf8`) and the
/// two hands, keeps the hand and track controller managers with their
/// sequences (the track rows' first four pairs set to cycle type 2 through
/// `007408d0`, their card arrays released), updates the root, resets and
/// updates the six track values, starts the track set-up
/// (`CAF_TRACK_SETUP`), shows both hands, clears the selection, sets the
/// flags (cleared, then `CAF_AI` and `CAF_DISABLE_ALLCONTROLS`), shows the
/// pot (twice `iAnte`), the net winnings, the won/lost games, the six
/// track totals (0) and the deck sizes, makes the game menu active (state
/// `CA_CAMERA_TO_GAME_ANIM`), grows the info rectangle, sets the captions,
/// shows only the game tile set and updates the flags.
// Translation incomplete: 0073ea90 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated
pub fn caravan_menu_prepare_game_menu(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    let player = e.global::<u32>(PLAYER);
    let second = fn_0073cbc0(e, Ptr::new(player));
    let player = e.global::<u32>(PLAYER);
    let first = fn_0073cba0(e, Ptr::new(player));
    e.call(CARD_LIST_CLEAR_00470470, &args![second]);
    e.call(CARD_LIST_CLEAR_00470470, &args![first]);
    let mut index = 0u32;
    while index < array_size(e, menu + DECK_ARRAY) {
        let card = array_element(e, menu + DECK_ARRAY, index);
        if fn_0073cb80(e, Ptr::new(card)) != 0 {
            let slot = array_slot(e, menu + DECK_ARRAY, index);
            e.call(CARD_LIST_ADD_005AE3D0, &args![second, slot]);
            let slot = array_slot(e, menu + DECK_ARRAY, index);
            e.call(ARRAY_APPEND, &args![menu + PLAYER_DECK, slot]);
        } else {
            let slot = array_slot(e, menu + DECK_ARRAY, index);
            e.call(CARD_LIST_ADD_005AE3D0, &args![first, slot]);
        }
        index += 1;
    }
    loop {
        let node = word_at(e, menu, DECK_PARAM);
        if node == 0 {
            break;
        }
        let data = e.call(IDENTITY, &args![node]).u32();
        if e.mem.u32(data) == 0 {
            break;
        }
        let node = word_at(e, menu, DECK_PARAM);
        let data = e.call(IDENTITY, &args![node]).u32();
        e.call(ARRAY_APPEND, &args![menu + NPC_DECK, data]);
        let node = word_at(e, menu, DECK_PARAM);
        let next = e.call(WORD_AT_FOUR, &args![node]).u32();
        e.mem.set_u32(menu + DECK_PARAM, next);
    }
    for _ in 0..8 {
        for (deck, hand) in [(PLAYER_DECK, PLAYER_HAND), (NPC_DECK, NPC_HAND)] {
            let size = array_size(e, menu + deck);
            let pick = e.call(RANDOM_RANGE, &args![0u32, size]).u32();
            let slot = array_slot(e, menu + deck, pick);
            e.call(ARRAY_APPEND, &args![menu + hand, slot]);
            e.call(ARRAY_REMOVE, &args![menu + deck, pick, 1u32]);
        }
    }
    for (deck, next) in [(PLAYER_DECK, NEXT_PLAYER_CARD), (NPC_DECK, NEXT_NPC_CARD)] {
        let size = array_size(e, menu + deck);
        let pick = e.call(RANDOM_RANGE, &args![0u32, size]).u32();
        e.mem.set_u32(menu + next, pick);
    }
    e.mem.set_u32(menu + DECK_PARAM, 0);
    release_if_filled(e, menu + DECK_ARRAY);
    for index in 0..0x60 {
        e.mem.set_u32(menu + DECK_FACE_SHADERS + 4 * index, 0);
    }
    e.call(ARRAY_RELEASE_008454F0, &args![menu + DECK_ARRAY, 1u32]);
    e.mem.set_u32(menu + INACTIVE_DECK_ANIM_MANAGER, 0);
    e.mem.set_u32(menu + ACTIVE_DECK_ANIM_MANAGER, 0);
    for index in 0..6 {
        e.mem
            .set_u32(menu + INACTIVE_DECK_ANIMATIONS + 4 * index, 0);
        e.mem.set_u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * index, 0);
    }
    for index in 0..0x60 {
        e.mem.set_u32(menu + DECK_FACE_SHADERS + 4 * index, 0);
    }
    detach_last_child(e);
    detach_last_child(e);
    release_model(e, ACTIVE_DECK_MODEL);
    release_model(e, INACTIVE_DECK_MODEL);
    e.mem.set_u32(menu + INACTIVE_DECK_GEOMETRY, 0);
    e.mem.set_u32(menu + ACTIVE_DECK_GEOMETRY, 0);
    for track in 0..3u32 {
        for row in 0..7u32 {
            for (format, side) in [(PLAYER_TRACK_FORMAT, track), (NPC_TRACK_FORMAT, track + 3)] {
                let model = load_formatted_model(e, &[format, MONEY_FOLDER, track + 1, row + 1]);
                let offset = TRACK_GEOMETRY + side * 0x1c + row * 4;
                e.mem.set_u32(menu + offset, model);
                attach_to_root(e, menu, offset);
            }
        }
    }
    for grid in 0..3u32 {
        let model = load_formatted_model(e, &[PLAYER_GRID_FORMAT, MONEY_FOLDER, grid + 1]);
        e.mem.set_u32(menu + TRACK_GRIDS + 4 * grid, model);
        let model = load_formatted_model(e, &[NPC_GRID_FORMAT, MONEY_FOLDER, grid + 1]);
        e.mem.set_u32(menu + TRACK_GRIDS + 0xc + 4 * grid, model);
    }
    for (format, cursor) in [
        (CURSOR_ADD_MODEL_FORMAT, CURSORS),
        (CURSOR_REMOVE_MODEL_FORMAT, CURSORS + 4),
    ] {
        let model = load_formatted_model(e, &[format, MONEY_FOLDER]);
        e.mem.set_u32(menu + cursor, model);
        attach_to_root(e, menu, cursor);
    }
    for (name, hand) in [
        (GAME_MENU_FOLDER_MODELS[2], HAND_GEOMETRY),
        (GAME_MENU_FOLDER_MODELS[3], HAND_GEOMETRY + 4),
    ] {
        let model = load_formatted_model(e, &[PATH_FORMAT, MONEY_FOLDER, name]);
        e.mem.set_u32(menu + hand, model);
    }
    attach_to_root(e, menu, HAND_GEOMETRY);
    attach_to_root(e, menu, HAND_GEOMETRY + 4);
    for grid in 0..6 {
        let object = word_at(e, menu, TRACK_GRIDS + 4 * grid);
        e.call(SET_CULLED, &args![object, 1u32]);
        let object = word_at(e, menu, TRACK_GRIDS + 4 * grid);
        update_object(e, object);
    }
    for cursor in [CURSORS, CURSORS + 4] {
        let object = word_at(e, menu, cursor);
        update_object(e, object);
    }
    for cursor in [CURSORS, CURSORS + 4] {
        let height = x87_quiet_bits(e.global::<u32>(CURSOR_START_HEIGHT));
        let object = word_at(e, menu, cursor);
        set_translate(e, object, height);
    }
    for cursor in [CURSORS, CURSORS + 4] {
        let object = word_at(e, menu, cursor);
        update_object(e, object);
    }
    for hand in 0..2u32 {
        let model = word_at(e, menu, HAND_GEOMETRY + 4 * hand);
        let manager = e
            .call(GET_CONTROLLER, &args![model, CONTROLLER_MANAGER_RTTI])
            .u32();
        if manager == 0 {
            continue;
        }
        let manager_offset = HAND_ANIM_MANAGER + 4 * hand;
        e.mem.set_u32(menu + manager_offset, manager);
        for (index, name) in HAND_SEQUENCES.into_iter().enumerate() {
            let manager = word_at(e, menu, manager_offset);
            let sequence = sequence_by_name(e, manager, name);
            let offset = HAND_ANIMATIONS + hand * 0x18 + 4 * index as u32;
            e.mem.set_u32(menu + offset, sequence);
        }
        for (index, name) in START_SEQUENCES.into_iter().enumerate() {
            let manager = word_at(e, menu, manager_offset);
            let sequence = sequence_by_name(e, manager, name);
            let offset = START_ANIMATIONS_OFFSET + (hand << 5) + 4 * index as u32;
            e.mem.set_u32(menu + offset, sequence);
        }
        let manager = word_at(e, menu, manager_offset);
        e.call(DEACTIVATE_ALL, &args![manager, 0u32]);
        let model = word_at(e, menu, HAND_GEOMETRY + 4 * hand);
        e.call(START_ANIMATIONS, &args![model]);
    }
    for track in 0..6u32 {
        for row in 0..7u32 {
            let model = track_geometry(e, menu, track, row);
            let manager = e
                .call(GET_CONTROLLER, &args![model, CONTROLLER_MANAGER_RTTI])
                .u32();
            if manager == 0 {
                continue;
            }
            let manager_offset = TRACK_ANIM_MANAGER + track * 0x1c + row * 4;
            e.mem.set_u32(menu + manager_offset, manager);
            let sequences = menu + TRACK_ANIMATIONS + track * 0xe0 + (row << 5);
            for (name, offset) in TRACK_SEQUENCES {
                let manager = word_at(e, menu, manager_offset);
                let sequence = sequence_by_name(e, manager, name);
                e.mem.set_u32(sequences + offset, sequence);
            }
            let manager = word_at(e, menu, manager_offset);
            e.call(DEACTIVATE_ALL, &args![manager, 0u32]);
            for pair in 0..4u32 {
                let sequence = e.mem.u32(sequences + pair * 8);
                fn_007408d0(e, Ptr::new(sequence), 2);
                let sequence = e.mem.u32(sequences + pair * 8 + 4);
                fn_007408d0(e, Ptr::new(sequence), 2);
            }
            let model = track_geometry(e, menu, track, row);
            e.call(START_ANIMATIONS, &args![model]);
            e.call(
                ARRAY_RELEASE_008454F0,
                &args![card_track(menu, track, row), 1u32],
            );
        }
    }
    e.with_stack(0xc, |e, block| {
        let data = e
            .call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32])
            .u32();
        let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
        e.call(OBJECT_UPDATE, &args![root, data]);
    });
    for track in 0..6u32 {
        let info = track_info(menu, track);
        e.mem.set_u32(info + 8, 0);
        e.mem.set_u32(info + 4, 5);
        e.mem.set_u32(info, 0);
        caravan_menu_update_track_value(e, this, track);
    }
    caravan_menu_set_flag(e, this, CAF_TRACK_SETUP, 1);
    caravan_menu_update_game_hand_textures(e, this, 1, 1);
    caravan_menu_update_game_hand_textures(e, this, 0, 1);
    e.mem.set_u32(menu + SELECTED_CARD, 0);
    e.mem.set_u32(menu + SELECTED_OBJECT, 0);
    e.mem.set_u32(menu + SELECTED_SHADOW, 0);
    caravan_menu_clear_flags(e, this);
    caravan_menu_set_flag(e, this, CAF_AI, 1);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
    e.with_stack(0x104, |e, buffer| {
        let lost = fn_00740960(e, Ptr::new(e.global::<u32>(PLAYER)));
        let won = fn_00740940(e, Ptr::new(e.global::<u32>(PLAYER)));
        e.call(
            PRINT_BOUNDED,
            &args![buffer, 0x104u32, GAMES_FORMAT, won, lost],
        );
        poke_value(e, this, 16);
        let pot = e.mem.u32(menu + ANTE).wrapping_shl(1);
        set_value(e, this, 16, pot);
        poke_value(e, this, 17);
        let winnings = fn_00740900(e, Ptr::new(e.global::<u32>(PLAYER)));
        let losses = fn_00740920(e, Ptr::new(e.global::<u32>(PLAYER)));
        set_value(e, this, 17, winnings.wrapping_sub(losses));
        poke_value(e, this, 18);
        let value_trait = e.global::<u32>(UT_VALUE);
        let target = tile(e, this, 18);
        e.call(SET_STRING_TRAIT, &args![target, value_trait, buffer, 1u32]);
    });
    for index in [26u32, 27, 28, 23, 24, 25] {
        poke_value(e, this, index);
        set_value(e, this, index, 0);
    }
    for (index, deck) in [(29u32, PLAYER_DECK), (30, NPC_DECK)] {
        poke_value(e, this, index);
        let size = array_size(e, menu + deck);
        let value_trait = e.global::<u32>(UT_VALUE);
        let target = tile(e, this, index);
        e.call(SET_TRAIT_FROM_UNSIGNED, &args![target, value_trait, size]);
    }
    e.set(this, CaravanMenu::eActiveMenu, 2);
    e.set(this, CaravanMenu::eGameState, CA_CAMERA_TO_GAME_ANIM);
    let game_tiles = tile(e, this, 2);
    let child = e
        .call(TILE_GET_CHILD_BY_NAME, &args![game_tiles, GAME_INFO_RECT])
        .u32();
    if child != 0 {
        let value = float_trait(e, child, 0xfb1);
        let value = x87_store32(x87_add(value, e.global::<f64>(INFO_RECT_GROWTH)));
        set_float_bits(e, child, 0xfb1, x87_quiet_bits(value));
        let width = float_trait(e, child, 0xfa1);
        let width = x87_store32(x87_sub(width, e.global::<f64>(INFO_RECT_GROWTH)));
        set_float_bits(e, child, 0xfa1, x87_quiet_bits(width));
        e.call(TILE_ADD_NEEDS_UPDATE, &args![child, 0x11u32]);
    }
    set_caption(e, this, 22, GAME_BUTTON_SETTING);
    for (index, visible) in [(0u32, 0u32), (1, 0), (2, 1), (3, 0)] {
        let target = tile(e, this, index);
        e.call(SET_TRAIT_FROM_INT, &args![target, 0xfa3u32, visible]);
    }
    for (index, setting) in GAME_CAPTIONS {
        set_caption(e, this, index, setting);
    }
    caravan_menu_update_caravan_flags(e, this);
}

/// The `CaravanAIPackage` setters the opponent's move code uses (`this`,
/// value): `iTrack` (+0xc), `iRow` (+0x10), `iDeckCard` (+0x14),
/// `iPriority` (+0x18) and `iActionType` (+8).
const PACKAGE_SET_TRACK: u32 = 0x0041_fd00;
const PACKAGE_SET_ROW: u32 = 0x0043_7730;
const PACKAGE_SET_DECK_CARD: u32 = 0x0060_0ab0;
const PACKAGE_SET_PRIORITY: u32 = 0x0098_4f60;
const PACKAGE_SET_ACTION: u32 = 0x0040_3550;
/// A package's `iPriority` (`009611e0`, `this` only).
const PACKAGE_PRIORITY: u32 = 0x0096_11e0;
/// The package compare function the candidates are sorted with.
const PACKAGE_COMPARE: u32 = 0x0073_b6e0;
/// `memset` (cdecl: block, value, size).
const MEMSET: u32 = 0x00ec_61c0;
/// `abs` (cdecl, one word).
const ABS: u32 = 0x00ec_7d40;
/// Whether a `BSSimpleArray` is empty (`0076b610`, `this` only).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// The address of element `index` of a `BSSimpleArray` (`00877a30`, the
/// body behind `006a7ad0`).
const ARRAY_SLOT_DIRECT: u32 = 0x0087_7a30;

/// `CaravanAIPackage::ActionType` (Xbox PDB enum).
const BUILDTRACK: u32 = 0;
const SHIFTDDIRECTION: u32 = 1;
const SAVETRACK: u32 = 2;
const USEJOKER: u32 = 3;
const SETUPTRACK: u32 = 4;
const DESTROYTRACK: u32 = 5;
const DISCARDCARD: u32 = 6;
const DESTROYPLAYER: u32 = 7;

/// The state of the opponent's move search: the candidate array and the
/// stack slot holding the package being filled.
struct MoveSearch {
    candidates: u32,
    slot: u32,
}

impl MoveSearch {
    fn package(&self, e: &Engine) -> u32 {
        e.mem.u32(self.slot)
    }

    fn set(&self, e: &mut Engine, setter: u32, value: u32) {
        let package = self.package(e);
        e.call(setter, &args![package, value]);
    }

    /// A new zeroed package (`operator new(0x1c)`, `memset`), or 0 when the
    /// allocation fails.
    fn new_package(&self, e: &mut Engine) {
        let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        let package = if block != 0 {
            e.call(MEMSET, &args![block, 0u32, 0x1cu32]);
            block
        } else {
            0
        };
        e.mem.set_u32(self.slot, package);
    }

    /// Writes the goal (`iValueGoal`) and its side (`iCalcResults`).
    fn set_goal(&self, e: &mut Engine, this: Ptr<CaravanMenu>, goal: i32) {
        let package = self.package(e);
        e.mem.set_i32(package, goal);
        let side = fn_00752df0(e, this, goal);
        let package = self.package(e);
        e.mem.set_i32(package + 4, side);
    }

    /// Appends the package (through its stack slot) to the candidates.
    fn append(&self, e: &mut Engine) {
        e.call(ARRAY_APPEND, &args![self.candidates, self.slot]);
    }

    /// Appends the package and starts a new one at the same row, hand card
    /// and track.
    fn keep(&self, e: &mut Engine, row: u32, card: u32, track: u32) {
        self.append(e);
        self.new_package(e);
        self.set(e, PACKAGE_SET_ROW, row);
        self.set(e, PACKAGE_SET_DECK_CARD, card);
        self.set(e, PACKAGE_SET_TRACK, track);
    }
}

/// The value of the opponent's hand card `index` (`pNPCHand`, word +0xb4).
fn npc_card(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32) -> i32 {
    card_b4(e, this.addr() + NPC_HAND, index) as i32
}

/// `trackInfo[track].iValue`.
fn track_total(e: &Engine, this: Ptr<CaravanMenu>, track: u32) -> i32 {
    e.mem.i32(track_info(this.addr(), track) + 8)
}

/// The first row of a track without cards (7 when all have cards).
fn first_empty_row(e: &mut Engine, this: Ptr<CaravanMenu>, track: u32) -> u32 {
    let mut row = 0u32;
    while row < 7 && array_size(e, card_track(this.addr(), track, row)) != 0 {
        row += 1;
    }
    row
}

/// The opponent's track-setup move (`CAF_TRACK_SETUP`): which hand card to
/// lay (`iDeckCard`) or discard (`DISCARDCARD`), by the AI level.
fn setup_move(e: &mut Engine, this: Ptr<CaravanMenu>, package: u32) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let level = e.mem.i32(menu + DIFF_LEVEL);
    let set = |e: &mut Engine, setter: u32, value: u32| {
        e.call(setter, &args![package, value]);
    };
    // A discard of the highest face card: a joker at once, else the first
    // face card, replaced by a later one while the kept one is a jack, or
    // a king and the later one no jack.
    let discard_face = |e: &mut Engine| {
        let mut kept = 0i32;
        let mut index = 0u32;
        while index < array_size(e, hand) {
            if npc_card(e, this, index) > 10 {
                if npc_card(e, this, index) == 0xf {
                    set(e, PACKAGE_SET_DECK_CARD, index);
                    return;
                }
                let replaces = match kept {
                    0 | 0xc => true,
                    0xe => npc_card(e, this, index) != 0xc,
                    _ => false,
                };
                if replaces {
                    set(e, PACKAGE_SET_DECK_CARD, index);
                    kept = npc_card(e, this, index);
                }
            }
            index += 1;
        }
    };
    set(e, PACKAGE_SET_ACTION, SETUPTRACK);
    if level < 1 {
        let mut numbers = 0u32;
        let mut face = 0u32;
        let mut number = 0u32;
        let mut index = 0u32;
        while index < array_size(e, hand) {
            if npc_card(e, this, index) <= 10 {
                numbers += 1;
                number = index;
            } else {
                face = index;
            }
            index += 1;
        }
        if array_size(e, hand).wrapping_sub(numbers) > 5 {
            set(e, PACKAGE_SET_ACTION, DISCARDCARD);
            set(e, PACKAGE_SET_DECK_CARD, face);
        } else {
            set(e, PACKAGE_SET_DECK_CARD, number);
        }
    } else if level < 3 {
        let mut middle = 0u32;
        let mut small = 0u32;
        let mut index = 0u32;
        while index < array_size(e, hand) {
            let value = npc_card(e, this, index);
            if value > 5 && npc_card(e, this, index) <= 10 {
                middle += 1;
            } else if npc_card(e, this, index) < 4 {
                small += 1;
            }
            index += 1;
        }
        if array_size(e, hand).wrapping_sub(small).wrapping_sub(middle) > 5 {
            set(e, PACKAGE_SET_ACTION, DISCARDCARD);
            discard_face(e);
        } else {
            let mut index = 0u32;
            while index < array_size(e, hand) {
                if middle > small {
                    if npc_card(e, this, index) > 5 && npc_card(e, this, index) <= 10 {
                        set(e, PACKAGE_SET_DECK_CARD, index);
                        break;
                    }
                } else if small != 0 && npc_card(e, this, index) < 4 {
                    set(e, PACKAGE_SET_DECK_CARD, index);
                    break;
                }
                index += 1;
            }
        }
    } else {
        let mut numbers = 0u32;
        let mut index = 0u32;
        while index < array_size(e, hand) {
            if npc_card(e, this, index) <= 10 {
                numbers += 1;
            }
            index += 1;
        }
        if array_size(e, hand).wrapping_sub(numbers) > 5 {
            set(e, PACKAGE_SET_ACTION, DISCARDCARD);
            discard_face(e);
        } else {
            let first = npc_card(e, this, 0).wrapping_sub(6);
            let mut closest = e.call(ABS, &args![first]).i32();
            set(e, PACKAGE_SET_DECK_CARD, 0);
            let mut index = 0u32;
            while index < array_size(e, hand) {
                let value = npc_card(e, this, index).wrapping_sub(6);
                if e.call(ABS, &args![value]).i32() < closest {
                    set(e, PACKAGE_SET_DECK_CARD, index);
                    let value = npc_card(e, this, index).wrapping_sub(6);
                    closest = e.call(ABS, &args![value]).i32();
                }
                index += 1;
            }
        }
    }
}

// Translated from 0074fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The opponent's next move (no Xbox PDB name; returns a `CaravanAIPackage`
/// by value through `out`, which it answers). The six track values are
/// updated first.
///
/// While setting up the tracks the package says which hand card to lay
/// (`SETUPTRACK`) or to discard (`DISCARDCARD`) by the AI level
/// ([`setup_move`]). The exe fills only those two fields of its stack
/// package; the rest is copied out as the stack held it (the model's stack
/// block is zeroed).
///
/// Otherwise it collects candidate packages (each a new zeroed 0x1c-byte
/// block appended to a new `BSSimpleArray<CaravanAIPackage *>`) for every
/// opponent track (3 to 5) by its value: below 21 (kings doubling a row,
/// number cards on the first empty row, when the result is not over 26),
/// sold with the facing player track ahead (jacks against it, jokers
/// scored from the per-track joker sums `0074f600`/`00752cb0`/`00752d40`
/// by the AI level, number cards that beat or match it), sold otherwise
/// (kings, number cards), or over 26 (jacks taking a row off); then for
/// every player track (jacks and kings against the player, queens
/// reversing a two-row run). Without candidates it discards its highest
/// card and adds a track discard (`DESTROYTRACK`) for each opponent track
/// that is not sold, prioritised by the AI level. The candidates are
/// sorted (`0073b6e0`), the first one's priority counted among the
/// leaders, the first copied to `out`, every candidate freed and the array
/// deleted.
// Translation incomplete: 0074fdc0 - the C++ exception-unwinding frame (FS:[0]) that frees the candidate array is not translated
pub fn fn_0074fdc0(e: &mut Engine, this: Ptr<CaravanMenu>, out: u32) -> u32 {
    for track in 0..6 {
        caravan_menu_update_track_value(e, this, track);
    }
    if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
        e.with_stack(0x1c, |e, package| {
            setup_move(e, this, package.addr());
            for word in 0..7 {
                let value = e.mem.u32(package.addr() + 4 * word);
                e.mem.set_u32(out + 4 * word, value);
            }
        });
        return out;
    }
    e.with_stack(0x24, |e, frame| {
        let slot = frame.addr();
        let result = frame.addr() + 4;
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let candidates = if block != 0 {
            fn_00753110(e, Ptr::new(block)).addr()
        } else {
            0
        };
        let search = MoveSearch { candidates, slot };
        search.new_package(e);
        for track in 3..6u32 {
            search.set(e, PACKAGE_SET_TRACK, track);
            npc_track_moves(e, this, &search, track);
        }
        for track in 0..3u32 {
            search.set(e, PACKAGE_SET_TRACK, track);
            player_track_moves(e, this, &search, track);
        }
        if array_size(e, candidates) == 0 {
            fallback_moves(e, this, &search);
        }
        e.call(ARRAY_SORT, &args![candidates, PACKAGE_COMPARE]);
        let first = array_element(e, candidates, 0);
        let top = e.call(PACKAGE_PRIORITY, &args![first]).u32();
        let mut index = 0u32;
        while index < array_size(e, candidates) {
            let package = array_element(e, candidates, index);
            if e.call(PACKAGE_PRIORITY, &args![package]).u32() != top {
                break;
            }
            index += 1;
        }
        let best = array_element(e, candidates, 0);
        for word in 0..7 {
            let value = e.mem.u32(best + 4 * word);
            e.mem.set_u32(result + 4 * word, value);
        }
        while !e.call(ARRAY_IS_EMPTY, &args![candidates]).bool() {
            let last = e.call(ARRAY_LAST_SLOT, &args![candidates]).u32();
            let package = e.mem.u32(last);
            fn_00753140(e, Ptr::new(candidates));
            e.call(OPERATOR_DELETE, &args![package]);
        }
        if candidates != 0 {
            e.vcall(candidates, 0, &args![1u32]);
        }
        for word in 0..7 {
            let value = e.mem.u32(result + 4 * word);
            e.mem.set_u32(out + 4 * word, value);
        }
    });
    out
}

/// The candidates of one opponent track (3 to 5) of [`fn_0074fdc0`].
fn npc_track_moves(e: &mut Engine, this: Ptr<CaravanMenu>, search: &MoveSearch, track: u32) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let level = e.mem.i32(menu + DIFF_LEVEL);
    let value = track_total(e, this, track);
    if fn_00752df0(e, this, value) < 0 {
        let mut index = 0u32;
        while index < array_size(e, hand) {
            search.set(e, PACKAGE_SET_DECK_CARD, index);
            if npc_card(e, this, index) > 10 {
                if npc_card(e, this, index) == 0xe {
                    let mut row = 0u32;
                    while row < 7 && array_size(e, card_track(menu, track, row)) != 0 {
                        search.set(e, PACKAGE_SET_ROW, row);
                        if array_size(e, card_track(menu, track, row)) > 3 {
                            row += 1;
                            continue;
                        }
                        let doubled = fn_0074f600(e, this, track, row)
                            .wrapping_add(track_total(e, this, track));
                        if fn_00752df0(e, this, doubled) < 1
                            && caravan_menu_is_valid_card_placement(
                                e,
                                this,
                                track as i32,
                                row as i32,
                                index as i32,
                                1,
                            )
                        {
                            search.set_goal(e, this, doubled);
                            let level = e.mem.i32(menu + DIFF_LEVEL);
                            let bonus: i32 = if level != 2 { 1 } else { -5 };
                            search.set(e, PACKAGE_SET_PRIORITY, bonus.wrapping_add(doubled) as u32);
                            search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
                            search.keep(e, row, index, track);
                        }
                        row += 1;
                    }
                }
            } else {
                let row = first_empty_row(e, this, track);
                search.set(e, PACKAGE_SET_ROW, row);
                if row != 7 {
                    let total = npc_card(e, this, index).wrapping_add(track_total(e, this, track));
                    if fn_00752df0(e, this, total) < 1
                        && caravan_menu_is_valid_card_placement(
                            e,
                            this,
                            track as i32,
                            row as i32,
                            index as i32,
                            1,
                        )
                    {
                        search.set_goal(e, this, total);
                        search.set(e, PACKAGE_SET_PRIORITY, total as u32);
                        search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
                        search.keep(e, row, index, track);
                    }
                }
            }
            index += 1;
        }
        return;
    }
    if fn_00752df0(e, this, value) == 0 {
        let facing = track_total(e, this, 5 - track);
        if value < facing && facing < 0x1b {
            contested_track_moves(e, this, search, track, value, level);
        } else {
            sold_track_moves(e, this, search, track, value);
        }
        return;
    }
    if fn_00752df0(e, this, value) > 0 {
        let mut index = 0u32;
        while index < array_size(e, hand) {
            search.set(e, PACKAGE_SET_DECK_CARD, index);
            let card = npc_card(e, this, index);
            if card > 10 && card == 0xc {
                for row in 0..7u32 {
                    search.set(e, PACKAGE_SET_ROW, row);
                    let row_value = fn_0074f600(e, this, track, row);
                    if value.wrapping_sub(row_value) < 0x1b
                        && caravan_menu_is_valid_card_placement(
                            e,
                            this,
                            track as i32,
                            row as i32,
                            index as i32,
                            1,
                        )
                    {
                        let lowered = value.wrapping_sub(row_value);
                        search.set_goal(e, this, lowered);
                        let priority = 0x64i32
                            .wrapping_sub(0x1ai32.wrapping_sub(value).wrapping_add(row_value));
                        search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
                        search.set(e, PACKAGE_SET_ACTION, SAVETRACK);
                        search.keep(e, row, index, track);
                    }
                }
            }
            index += 1;
        }
    }
}

/// An opponent track that is sold while the facing player track is ahead
/// and not over 26 ([`npc_track_moves`]).
fn contested_track_moves(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    search: &MoveSearch,
    track: u32,
    value: i32,
    level: i32,
) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let mut index = 0u32;
    while index < array_size(e, hand) {
        search.set(e, PACKAGE_SET_DECK_CARD, index);
        let card = npc_card(e, this, index);
        if card > 10 {
            let gap = track_total(e, this, 5 - track).wrapping_sub(value);
            if card == 0xc {
                for row in 0..7u32 {
                    search.set(e, PACKAGE_SET_ROW, row);
                    if array_size(e, card_track(menu, 5 - track, row)) == 0 {
                        break;
                    }
                    let row_value = fn_0074f600(e, this, 5 - track, row);
                    if row_value <= gap {
                        continue;
                    }
                    let remaining = track_total(e, this, 5 - track).wrapping_sub(row_value);
                    let lowered = value.wrapping_sub(remaining);
                    let priority = if track_total(e, this, 5 - track).wrapping_sub(row_value) > 0x15
                    {
                        0x64i32.wrapping_sub(lowered)
                    } else {
                        lowered.wrapping_add(0x4b)
                    };
                    search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
                    if caravan_menu_is_valid_card_placement(
                        e,
                        this,
                        track as i32,
                        row as i32,
                        index as i32,
                        1,
                    ) {
                        search.set_goal(e, this, value);
                        search.set(e, PACKAGE_SET_ACTION, DESTROYPLAYER);
                        search.keep(e, row, index, track);
                    }
                }
            } else if level >= 1 && card == 0xf {
                joker_moves(e, this, search, track, index, level);
            }
        } else {
            let row = first_empty_row(e, this, track);
            search.set(e, PACKAGE_SET_ROW, row);
            let total = card.wrapping_add(track_total(e, this, track));
            if total < 0x1b && total > track_total(e, this, 5 - track) {
                let over = total.wrapping_sub(track_total(e, this, 5 - track));
                search.set(e, PACKAGE_SET_PRIORITY, 0x64i32.wrapping_sub(over) as u32);
                search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
            } else if total < 0x1b && total == track_total(e, this, 5 - track) {
                search.set(e, PACKAGE_SET_PRIORITY, 0x32);
                search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
            } else if total < 0x1b {
                let priority = 0x19i32.wrapping_sub(0x1ai32.wrapping_sub(total));
                search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
                search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
            }
            if caravan_menu_is_valid_card_placement(
                e,
                this,
                track as i32,
                row as i32,
                index as i32,
                1,
            ) {
                search.set_goal(e, this, total);
                search.keep(e, row, index, track);
            }
        }
        index += 1;
    }
}

/// A joker in hand against a contested track ([`contested_track_moves`]):
/// for each row with cards where the joker may go, the per-track sums of
/// the cards the joker would take (`00752cb0` for an ace, else `00752d40`
/// with the row's first card's value), scored by the AI level.
fn joker_moves(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    search: &MoveSearch,
    track: u32,
    index: u32,
    level: i32,
) {
    let menu = this.addr();
    for row in 0..7u32 {
        search.set(e, PACKAGE_SET_ROW, row);
        if array_size(e, card_track(menu, track, row)) == 0 {
            break;
        }
        if !caravan_menu_is_valid_card_placement(e, this, track as i32, row as i32, index as i32, 1)
        {
            continue;
        }
        let joker_value = card_b4(e, card_track(menu, track, row), 0) as i32;
        let mut sums = [0i32; 6];
        if joker_value == 1 {
            for (other, sum) in sums.iter_mut().enumerate() {
                *sum = fn_00752cb0(e, this, other as u32, joker_value);
            }
        } else {
            for (other, sum) in sums.iter_mut().enumerate() {
                let first = card_b4(e, card_track(menu, track, row), 0);
                *sum = fn_00752d40(e, this, other as u32, first) as i32;
            }
        }
        sums[track as usize] = sums[track as usize].wrapping_add(joker_value);
        let total = |e: &Engine, track: u32| track_total(e, this, track);
        if level < 3 {
            let score = total(e, track)
                .wrapping_sub(sums[track as usize])
                .wrapping_sub(total(e, 5 - track))
                .wrapping_sub(sums[5 - track as usize]);
            if score > 0
                && caravan_menu_is_valid_card_placement(
                    e,
                    this,
                    track as i32,
                    row as i32,
                    index as i32,
                    1,
                )
            {
                search.set(e, PACKAGE_SET_PRIORITY, score.wrapping_add(0x32) as u32);
                search.set(e, PACKAGE_SET_ACTION, USEJOKER);
                search.keep(e, row, index, track);
            }
            continue;
        }
        let mut leads = [0i32; 3];
        for (pair, lead) in leads.iter_mut().enumerate() {
            let pair = pair as u32;
            *lead = total(e, pair + 3)
                .wrapping_sub(sums[pair as usize + 3])
                .wrapping_sub(total(e, pair))
                .wrapping_sub(sums[pair as usize]);
        }
        let mut score = 0i32;
        if leads[5 - track as usize] > 0 {
            score += 0x4b;
        }
        for pair in 0..3u32 {
            let own = total(e, pair + 3);
            let own_sum = sums[pair as usize + 3];
            let player_sum = sums[pair as usize];
            if own > 0x14 && own < 0x1b && own.wrapping_sub(own_sum) < 0x15 {
                score = score.wrapping_sub(
                    0x19i32.wrapping_sub(0x15i32.wrapping_sub(own.wrapping_sub(own_sum))),
                );
            }
            if fn_00752df0(e, this, own) >= 0 {
                if own < 0x1b && own.wrapping_sub(own_sum) < 0x15 {
                    score = score.wrapping_sub(0x15i32.wrapping_sub(own).wrapping_sub(own_sum));
                }
                if own < 0x1b && leads[pair as usize] < 0 {
                    let player = total(e, pair).wrapping_sub(player_sum);
                    if fn_00752df0(e, this, player) == 0 {
                        score = score.wrapping_sub(leads[pair as usize].wrapping_add(0x19));
                    }
                }
                if fn_00752df0(e, this, own) > 0 {
                    let lowered = own.wrapping_sub(own_sum);
                    if fn_00752df0(e, this, lowered) <= 0 {
                        score = score.wrapping_add(lowered);
                    }
                }
            }
            if fn_00752df0(e, this, own) > 0 && own_sum > 0 {
                let lowered = own.wrapping_sub(own_sum);
                if fn_00752df0(e, this, lowered) == 0 {
                    score = score.wrapping_add(lowered).wrapping_add(0x19);
                } else if fn_00752df0(e, this, lowered) < 0 {
                    score = score.wrapping_add(lowered);
                } else {
                    score = score.wrapping_add(own_sum);
                }
            }
            let player = total(e, pair);
            if fn_00752df0(e, this, player) == 0
                && fn_00752df0(e, this, player.wrapping_sub(player_sum)) < 0
            {
                score = 0x2ei32
                    .wrapping_sub(player)
                    .wrapping_add(player_sum)
                    .wrapping_add(score);
            }
            if score > 0
                && caravan_menu_is_valid_card_placement(
                    e,
                    this,
                    track as i32,
                    row as i32,
                    index as i32,
                    1,
                )
            {
                search.set(e, PACKAGE_SET_PRIORITY, score as u32);
                search.set(e, PACKAGE_SET_ACTION, USEJOKER);
                search.keep(e, row, index, track);
            }
        }
    }
}

/// A sold opponent track that is not behind ([`npc_track_moves`]): kings
/// doubling a row and number cards on the first empty row, when the result
/// is not over 26.
fn sold_track_moves(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    search: &MoveSearch,
    track: u32,
    value: i32,
) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let mut index = 0u32;
    while index < array_size(e, hand) {
        search.set(e, PACKAGE_SET_DECK_CARD, index);
        let card = npc_card(e, this, index);
        if card > 10 {
            if npc_card(e, this, index) == 0xe {
                for row in 0..7u32 {
                    search.set(e, PACKAGE_SET_ROW, row);
                    let row_value = fn_0074f600(e, this, track, row);
                    let doubled = value.wrapping_add(row_value.wrapping_mul(2));
                    if fn_00752df0(e, this, doubled) < 1
                        && caravan_menu_is_valid_card_placement(
                            e,
                            this,
                            track as i32,
                            row as i32,
                            index as i32,
                            1,
                        )
                    {
                        search.set_goal(e, this, doubled);
                        search.set(e, PACKAGE_SET_PRIORITY, doubled as u32);
                        search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
                        search.keep(e, row, index, track);
                    }
                }
            }
        } else {
            let row = first_empty_row(e, this, track);
            search.set(e, PACKAGE_SET_ROW, row);
            if caravan_menu_is_valid_card_placement(
                e,
                this,
                track as i32,
                row as i32,
                index as i32,
                1,
            ) {
                let total = card.wrapping_add(track_total(e, this, track));
                if fn_00752df0(e, this, total) < 1 {
                    let total = card.wrapping_add(track_total(e, this, track));
                    search.set_goal(e, this, total);
                    search.set(e, PACKAGE_SET_PRIORITY, total as u32);
                    search.set(e, PACKAGE_SET_ACTION, BUILDTRACK);
                    search.keep(e, row, index, track);
                }
            }
        }
        index += 1;
    }
}

/// The candidates against one player track (0 to 2) of [`fn_0074fdc0`]:
/// for each face card in hand and each row (from the first, while it has
/// cards, skipping rows with more than 3), a jack taking the row off a sold
/// or short track, a king pushing the track over, or a queen on a row two
/// or more down reversing a short run (sum below 8) that rises or a long
/// one (above 16) that falls (`00877a30` reads those rows' first cards).
fn player_track_moves(e: &mut Engine, this: Ptr<CaravanMenu>, search: &MoveSearch, track: u32) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let mut index = 0u32;
    while index < array_size(e, hand) {
        search.set(e, PACKAGE_SET_DECK_CARD, index);
        if npc_card(e, this, index) > 10 {
            let mut row = 0u32;
            while row < 7 && array_size(e, card_track(menu, track, row)) != 0 {
                search.set(e, PACKAGE_SET_ROW, row);
                if array_size(e, card_track(menu, track, row)) > 3 {
                    row += 1;
                    continue;
                }
                match npc_card(e, this, index) {
                    0xc => {
                        let row_value = fn_0074f600(e, this, track, row);
                        let left = track_total(e, this, track).wrapping_sub(row_value);
                        let side = fn_00752df0(e, this, track_total(e, this, track));
                        if side == 0 {
                            let priority = 0x1ai32.wrapping_sub(left).wrapping_add(0x19);
                            search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
                            search.set(e, PACKAGE_SET_ACTION, DESTROYPLAYER);
                            search.keep(e, row, index, track);
                        } else if fn_00752df0(e, this, track_total(e, this, track)) < 0 {
                            let priority = 0x1ai32.wrapping_sub(left);
                            search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
                            search.set(e, PACKAGE_SET_ACTION, DESTROYPLAYER);
                            search.keep(e, row, index, track);
                        }
                    }
                    0xe => {
                        let pushed = fn_0074f600(e, this, track, row)
                            .wrapping_add(track_total(e, this, track));
                        if fn_00752df0(e, this, pushed) == 1 {
                            search.set(e, PACKAGE_SET_PRIORITY, pushed.wrapping_sub(1) as u32);
                            search.set(e, PACKAGE_SET_ACTION, DESTROYPLAYER);
                            search.keep(e, row, index, track);
                        }
                    }
                    0xd => queen_move(e, this, search, track, row, index),
                    _ => {}
                }
                row += 1;
            }
        }
        index += 1;
    }
}

/// The queen case of [`player_track_moves`]: only for a row past 1 without
/// cards, which the row loop never reaches (it stops at the first empty
/// row); kept as the exe has it.
fn queen_move(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    search: &MoveSearch,
    track: u32,
    row: u32,
    index: u32,
) {
    let menu = this.addr();
    if row <= 1 || array_size(e, card_track(menu, track, row)) != 0 {
        return;
    }
    let first_of = |e: &mut Engine, row: u32| {
        let slot = e
            .call(
                ARRAY_SLOT_DIRECT,
                &args![card_track(menu, track, row), 0u32],
            )
            .u32();
        let card = e.mem.u32(slot);
        e.call(CARD_WORD_B4, &args![card]).i32()
    };
    let last = first_of(e, row - 1);
    let before = first_of(e, row - 2);
    let sum = last.wrapping_add(before);
    if sum < 8 {
        let last = first_of(e, row - 1);
        let before = first_of(e, row - 2);
        if last > before {
            search.set(e, PACKAGE_SET_ROW, row - 1);
            search.set(e, PACKAGE_SET_PRIORITY, 0x23i32.wrapping_sub(sum) as u32);
            search.set(e, PACKAGE_SET_ACTION, SHIFTDDIRECTION);
            search.keep(e, row, index, track);
        }
    } else if sum > 0x10 {
        let last = first_of(e, row - 1);
        let before = first_of(e, row - 2);
        if last < before {
            search.set(e, PACKAGE_SET_ROW, row - 1);
            search.set(e, PACKAGE_SET_PRIORITY, sum.wrapping_add(0x19) as u32);
            search.set(e, PACKAGE_SET_ACTION, SHIFTDDIRECTION);
            search.keep(e, row, index, track);
        }
    }
}

/// No candidates ([`fn_0074fdc0`]): discard the highest hand card (its
/// index also as the priority), then a track discard for each opponent
/// track that is not sold: priority the total less 26 (AI level 0), twice
/// that (levels 1 and 2), or less 5 per jack in the opponent's deck (level
/// 3 and above).
fn fallback_moves(e: &mut Engine, this: Ptr<CaravanMenu>, search: &MoveSearch) {
    let menu = this.addr();
    let hand = menu + NPC_HAND;
    let mut best = 0u32;
    let mut index = 0u32;
    while index < array_size(e, hand) {
        if npc_card(e, this, index) > npc_card(e, this, best) {
            best = index;
        }
        index += 1;
    }
    search.set(e, PACKAGE_SET_DECK_CARD, best);
    search.set(e, PACKAGE_SET_ACTION, DISCARDCARD);
    search.set(e, PACKAGE_SET_PRIORITY, best);
    search.append(e);
    search.new_package(e);
    for track in 3..6u32 {
        let total = track_total(e, this, track);
        if fn_00752df0(e, this, total) == 0 {
            continue;
        }
        search.set(e, PACKAGE_SET_TRACK, track);
        search.set(e, PACKAGE_SET_ACTION, DESTROYTRACK);
        let level = e.mem.i32(menu + DIFF_LEVEL);
        let total = track_total(e, this, track);
        let priority = if level < 1 {
            total.wrapping_sub(0x1a)
        } else if level < 3 {
            total.wrapping_mul(2).wrapping_sub(0x34)
        } else {
            let mut jacks = 0i32;
            let mut card = 0u32;
            while card < array_size(e, menu + NPC_DECK) {
                if card_b4(e, menu + NPC_DECK, card) == 0xc {
                    jacks += 1;
                }
                card += 1;
            }
            let total = track_total(e, this, track);
            total
                .wrapping_mul(2)
                .wrapping_sub(0x34)
                .wrapping_sub(jacks.wrapping_mul(5))
        };
        search.set(e, PACKAGE_SET_PRIORITY, priority as u32);
        search.append(e);
        search.new_package(e);
        search.set(e, PACKAGE_SET_TRACK, track);
    }
}

/// `004b7210`: the global whose rendered menu node `007fd640` returns.
const RENDER_GLOBAL_004B7210: u32 = 0x004b_7210;
/// `007fd640` (`thiscall`): the rendered menu's node.
const RENDERED_MENU_007FD640: u32 = 0x007f_d640;
/// `0043c4b0`: the render target owner `007148c0` acts on.
const RENDER_OWNER_0043C4B0: u32 = 0x0043_c4b0;
/// `007148c0` (`thiscall`, two words): clears the render target.
const RENDER_CLEAR_007148C0: u32 = 0x0071_48c0;
/// `0045c670`: the main camera holder.
const MAIN_CAMERA_0045C670: u32 = 0x0045_c670;
/// `008d80e0` (`fastcall`): the holder's culling scene.
const SCENE_008D80E0: u32 = 0x008d_80e0;
/// `00c4f270` (`thiscall`, one word): begins the culling scene.
const SCENE_BEGIN_00C4F270: u32 = 0x00c4_f270;
/// `00c4f2d0` (`thiscall`): resets the culling scene.
const SCENE_RESET_00C4F2D0: u32 = 0x00c4_f2d0;
/// `004a0fd0` (`thiscall`, one word): sets the scene's node.
const SCENE_SET_NODE_004A0FD0: u32 = 0x004a_0fd0;
/// `00709d50` (`cdecl`: camera, float): sets the camera up for rendering to a texture.
const RENDER_TO_TEXTURE_00709D50: u32 = 0x0070_9d50;
/// The float 0.15 (`0x3e19999a`) passed to `00709d50`.
const RENDER_VALUE_01030FF4: u32 = 0x0103_0ff4;
/// `006629f0` (`fastcall`): the holder's own camera.
const HOLDER_CAMERA_006629F0: u32 = 0x0066_29f0;
/// `00450b80` (`cdecl`, one word): the scene graph.
const SCENE_GRAPH_00450B80: u32 = 0x0045_0b80;
/// `BSShaderUtil::AccumulateScene` (`cdecl`: camera, graph, scene).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `00b6c0d0` (`cdecl`: camera, node, 0): renders.
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
/// `00633c90` (`thiscall`): the `NiPointer` constructor.
const NI_POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, one word): assigns the `NiPointer`.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0045cec0` (`thiscall`): the `NiPointer` destructor.
const NI_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;

// Translated from 00740f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (called from the menu renderer `00709b30`): renders the
/// caravan scene. Clears the render target (`007148c0(0, 4)` on `0043c4b0`),
/// begins the main camera holder's culling scene (`00c4f270(1)`), sets the
/// caravan camera (`011d91a8`) up with 0.15 (`00709d50`), holds the rendered
/// menu node in a stack `NiPointer` which the scene takes (`004a0fd0`), then
/// accumulates (`BSShaderUtil::AccumulateScene` with graph `00450b80(1)`) and
/// renders (`00b6c0d0` with 0) through the caravan camera, or the holder's
/// own camera (`006629f0`) when that is null. The scene drops the node,
/// the pointer is cleared (`0066b0d0(0)`), the scene reset (`00c4f2d0`) and
/// the pointer destroyed.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00740f30 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn caravan_menu_render_scene(e: &mut Engine) {
    let owner = e.call(RENDER_OWNER_0043C4B0, &args![]).u32();
    e.call(RENDER_CLEAR_007148C0, &args![owner, 0u32, 4u32]);
    let holder = e.call(MAIN_CAMERA_0045C670, &args![]).u32();
    let scene = e.call(SCENE_008D80E0, &args![holder]).u32();
    e.call(SCENE_BEGIN_00C4F270, &args![scene, 1u32]);
    let caravan_camera = e.call(FIRST_WORD, &args![CAMERA_POINTER]).u32();
    let value = x87_quiet_bits(e.mem.u32(RENDER_VALUE_01030FF4));
    e.call(RENDER_TO_TEXTURE_00709D50, &args![caravan_camera, value]);
    e.with_stack(4, |e, node| {
        let global = e.call(RENDER_GLOBAL_004B7210, &args![]).u32();
        let rendered = e.call(RENDERED_MENU_007FD640, &args![global]).u32();
        e.call(NI_POINTER_CONSTRUCTOR, &args![node, rendered]);
        let held = e.call(FIRST_WORD, &args![node]).u32();
        e.call(SCENE_SET_NODE_004A0FD0, &args![scene, held]);
        let camera = if caravan_camera == 0 {
            e.call(HOLDER_CAMERA_006629F0, &args![holder]).u32()
        } else {
            caravan_camera
        };
        let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![camera, graph, scene]);
        let held = e.call(FIRST_WORD, &args![node]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, held, 0u32]);
        e.call(SCENE_SET_NODE_004A0FD0, &args![scene, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![node, 0u32]);
        e.call(SCENE_RESET_00C4F2D0, &args![scene]);
        e.call(NI_POINTER_DESTRUCTOR, &args![node]);
    });
}

/// `CaravanMenu::DoIdle`'s timer (`float` at `011d9208`): seconds the
/// current game state has run; and the game state it was counted for
/// (`011d9204`).
const IDLE_TIME: u32 = 0x011d_9208;
const IDLE_TIME_STATE: u32 = 0x011d_9204;
/// The object whose word +0x14 (`00825c00`) `DoIdle` reads as its tick
/// (`011f6394`).
const IDLE_TICK_SOURCE: u32 = 0x011f_6394;
/// The word at +0x14 of an object (`00825c00`, `this` only).
const WORD_AT_0X14: u32 = 0x0082_5c00;
/// The word at +0xc of an object (`0084e3a0`, `this` only).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// The word at +0x10 of an object (`0044edb0`, `this` only).
const WORD_AT_0X10: u32 = 0x0044_edb0;
/// Stores a word at +0xc (`0041fd00`) and at +0x10 (`00437730`) of an
/// object (`this`, word).
const SET_WORD_AT_0XC: u32 = 0x0041_fd00;
const SET_WORD_AT_0X10: u32 = 0x0043_7730;
/// The word at +0x28 of an object (`0045cd60`, `this` only): the tile of
/// the interface manager whose trait `0xfa9` `DoIdle` sets.
const WORD_AT_0X28: u32 = 0x0045_cd60;
/// The address +0x8c of an object (`0045bb80`, `this` only).
const PLUS_0X8C: u32 = 0x0045_bb80;
/// The float at +0x30 of a controller sequence, in ST0 (`00508100`, `this`
/// only): compared with the idle timer to end an animation state.
const SEQUENCE_TIME_00508100: u32 = 0x0050_8100;
/// The float at +0x48 of the interface manager, in ST0 (`00639aa0`, `this`
/// only).
const INTERFACE_FLOAT_00639AA0: u32 = 0x0063_9aa0;
/// `00a23390` (`this` = controls, two words): a controller axis value.
const CONTROLS_AXIS_00A23390: u32 = 0x00a2_3390;
/// `00a23a50` (`this` = controls, two words): a controller button test.
const CONTROLS_BUTTON_00A23A50: u32 = 0x00a2_3a50;
/// The absolute value of a `float` (cdecl, result in ST0): `00408820`
/// (forwards to `00408840`) and `00408840`.
const FLOAT_MAGNITUDE_00408820: u32 = 0x0040_8820;
const FLOAT_MAGNITUDE_00408840: u32 = 0x0040_8840;
/// The absolute value of an integer (cdecl): `00ec7d40`.
const INTEGER_MAGNITUDE: u32 = 0x00ec_7d40;
/// `0043b4a0` (`this`, word): the node the idle animations update through
/// (`this` = the word at `011d91e4`).
const MODEL_NODE_0043B4A0: u32 = 0x0043_b4a0;
/// `0047b220` (`this` = manager, sequence, `float`): stops a sequence.
const DEACTIVATE_SEQUENCE: u32 = 0x0047_b220;
/// `Menu::StartFadeOut` and `Menu::InstantFadeOut` (`this` only).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_INSTANT_FADE_OUT: u32 = 0x00a1_d9e0;
/// A byte global `DoIdle` sets when the menu fades out on closing; when it
/// was already set the fade is instant (`011d9200`).
const CLOSE_FADED_BYTE: u32 = 0x011d_9200;
/// `NiCamera::WorldPtToScreenPt` (`this` = camera, point, x out, y out,
/// tolerance).
const WORLD_POINT_TO_SCREEN: u32 = 0x00a6_fc50;
/// The divisor of the elapsed ticks (`double` 1000.0 at `01017b70`).
const TICKS_PER_SECOND: u32 = 0x0101_7b70;
/// The stick dead zone and range (`double` 7849.0 at `010718f0`, 32767.0 at
/// `01029790`).
const STICK_DEAD_ZONE: u32 = 0x0107_18f0;
const STICK_RANGE: u32 = 0x0102_9790;
/// `double` 1.0 (`01012070`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The repeat delay of the pad and key selection (`double` 0.25 at
/// `010290b0`) and the stick threshold of the game menu (`double` 0.01 at
/// `01031148`).
const SELECTION_REPEAT: u32 = 0x0102_90b0;
const STICK_THRESHOLD: u32 = 0x0103_1148;
/// `float` -1.0 (`01012054`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// The deal intro's screen layout: the tolerance of the screen projection
/// (`float` at `010718c0`), the left edge offset (`double` 120.0 at
/// `010718b8`) and the step between the track rectangles (`double` 265.0 at
/// `010718b0`).
const PROJECTION_TOLERANCE: u32 = 0x0107_18c0;
const TRACK_RECT_LEFT: u32 = 0x0107_18b8;
const TRACK_RECT_STEP: u32 = 0x0107_18b0;
/// The node name the track rectangles are placed by (`Select1_01:0`) and
/// the two track rectangle tiles (`CGM_NPCTrackRect`,
/// `CGM_PlayerTrackRect`).
const FIRST_SELECT_NAME: u32 = 0x0107_18c4;
const NPC_TRACK_RECT_NAME: u32 = 0x0107_189c;
const PLAYER_TRACK_RECT_NAME: u32 = 0x0107_1888;
/// The opponent's deck objects hidden when it plays its last card
/// (`Opponent-Deck_06:5`, `Opponent-Deck_Back:0` and their shadows).
const NPC_DECK_OBJECTS: [u32; 4] = [0x0107_17a8, 0x0107_1790, 0x0107_1774, 0x0107_1758];
/// The sounds of the idle states (`GAMECaravanStartMatch`, `FundsDrop`,
/// `Navigate`, `ChangeDeck`, `AddRemove`, `ConfirmAddtoDeck`,
/// `RemoveCard`, `ConfirmAddToTrack`, `AddRemoveCard`, `SwitchCard`).
const START_MATCH_SOUND: u32 = 0x0107_18d4;
const FUNDS_DROP_SOUND: u32 = 0x0107_1870;
const NAVIGATE_SOUND: u32 = 0x0107_185c;
const CHANGE_DECK_SOUND: u32 = 0x0107_1844;
const ADD_REMOVE_SOUND: u32 = 0x0107_182c;
const CONFIRM_ADD_TO_DECK_SOUND: u32 = 0x0107_1810;
const REMOVE_CARD_SOUND: u32 = 0x0107_17f8;
const CONFIRM_ADD_TO_TRACK_SOUND: u32 = 0x0107_17d8;
const ADD_REMOVE_CARD_SOUND: u32 = 0x0107_17bc;
const SWITCH_CARD_SOUND: u32 = 0x0107_1740;
/// The help messages' editor IDs (`HelpCaravanDeckBuilding`,
/// `HelpCaravanStartingCaravans`, `HelpCaravanContractWar`).
const HELP_DECK_BUILDING: u32 = 0x0101_8dd4;
const HELP_STARTING_CARAVANS: u32 = 0x0101_8db8;
const HELP_CONTRACT_WAR: u32 = 0x0101_8da0;
/// The setting whose string the results menu's caption (tile 35) gets.
const RESULTS_CAPTION_SETTING: u32 = 0x011d_372c;
/// The `fSelectionDelay` field (Xbox PDB, `float` at +0xe3c).
const SELECTION_DELAY: u32 = 0xe3c;
/// The keys `DoKey` leaves in `PENDING_KEY` (up, down, right, left).
const KEY_UP: u32 = 0x8000_0003;
const KEY_DOWN: u32 = 0x8000_0004;
const KEY_RIGHT: u32 = 0x8000_0002;
const KEY_LEFT: u32 = 0x8000_0001;
/// `CaravanMenu::enumFlags` (Xbox PDB enum).
const CAF_JOKER: u32 = 0x8000;
/// `CaravanMenu`'s frame size below `EBP` (`SUB ESP,0x56c` under the three
/// words of the exception frame).
const DO_IDLE_FRAME: u32 = 0x578;

/// The idle timer as `FLD float` loads it.
fn idle_time(e: &Engine) -> f64 {
    x87_load32(e.global::<u32>(IDLE_TIME))
}

/// The idle timer's bits as an `FLD`/`FSTP` pair pushes them.
fn idle_time_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.global::<u32>(IDLE_TIME))
}

/// `FLD [idle timer]; FCOMP 0.0; TEST AH,0x44; JP`: the timer is 0.0.
fn idle_time_is_zero(e: &Engine) -> bool {
    idle_time(e) == e.global::<f64>(ZERO_DOUBLE)
}

/// `FLDZ; FSTP [idle timer]`.
fn reset_idle_time(e: &mut Engine) {
    e.set_global::<u32>(IDLE_TIME, 0);
}

/// `0047aab0(manager, sequence, 0, 0, 1.0, 0.0, 0)`.
fn idle_activate(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(
        ACTIVATE_SEQUENCE,
        &args![
            manager,
            sequence,
            0u32,
            0u32,
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0u32
        ],
    );
}

/// `0047b220(manager, sequence, 0.0)`.
fn idle_deactivate(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(DEACTIVATE_SEQUENCE, &args![manager, sequence, 0u32]);
}

/// `0043d410(block, time, 1, 0)`: an update-data block; answers the block.
fn idle_update_data(e: &mut Engine, block: u32, time_bits: u32) -> u32 {
    e.call(UPDATE_DATA_INIT, &args![block, time_bits, 1u32, 0u32])
        .u32()
}

/// `00a59c60(object, block)`.
fn idle_update(e: &mut Engine, object: u32, block: u32) {
    e.call(OBJECT_UPDATE, &args![object, block]);
}

/// `00a59c60(0043b4a0(00559450(011d91e4), 0), block)`.
fn idle_update_model(e: &mut Engine, block: u32) {
    let root = e.call(FIRST_WORD, &args![MONEY_ROOT]).u32();
    let node = e.call(MODEL_NODE_0043B4A0, &args![root, 0u32]).u32();
    e.call(OBJECT_UPDATE, &args![node, block]);
}

/// `00508100(sequence)`, then `FLD [idle timer]; FCOMPP; TEST AH,1`: the
/// timer has reached the sequence's float (ordered, not below).
fn idle_sequence_reached(e: &mut Engine, sequence: u32) -> bool {
    let end = e.call(SEQUENCE_TIME_00508100, &args![sequence]).f64();
    idle_time(e) >= end
}

/// The same with `TEST AH,0x41`: the timer is past the sequence's float.
fn idle_sequence_passed(e: &mut Engine, sequence: u32) -> bool {
    let end = e.call(SEQUENCE_TIME_00508100, &args![sequence]).f64();
    idle_time(e) > end
}

/// Shows the tutorial `index` with the message of `editor_id` unless the
/// interface manager's tutorials (+0x4d4) have shown it (`00718840`):
/// `TutorialMenu::Create` of the message (`__RTDynamicCast` of the form),
/// its result in `bTutorial`, marks it shown (`007185e0`) and keeps the
/// index in `iTutorial`.
fn idle_tutorial(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32, editor_id: u32) {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    if e.call(TUTORIAL_SHOWN, &args![interface.wrapping_add(0x4d4), index])
        .bool()
    {
        return;
    }
    let form = e.call(FORM_BY_EDITOR_ID, &args![editor_id]).u32();
    let message = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    if message == 0 {
        return;
    }
    let shown = e.call(TUTORIAL_MENU_CREATE, &args![message, 0u32]).u8();
    e.set(this, CaravanMenu::bTutorial, shown);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(TUTORIAL_MARK, &args![interface.wrapping_add(0x4d4), index]);
    e.mem.set_u32(this.addr() + TUTORIAL_INDEX, index);
}

/// `00a012d0(interface tile, 0xfa9, 255.0, 1)`: the trait of the interface
/// manager's tile (`004b7210`, its word +0x28).
fn idle_reset_interface_tile(e: &mut Engine) {
    let full = x87_quiet_bits(e.global::<u32>(TRACK_ALPHA));
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let target = e.call(WORD_AT_0X28, &args![interface]).u32();
    e.call(SET_FLOAT_TRAIT, &args![target, 0xfa9u32, full, 1u32]);
}

/// Sets the deck meter (tile 15) to `index` (`00a012d0` with flag 0) and
/// pokes its `_SetInCode` trait with 1.0.
fn idle_set_meter(e: &mut Engine, this: Ptr<CaravanMenu>, index: i32) {
    let value = x87_store32(f64::from(index));
    let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
    let meter = tile(e, this, 0xf);
    e.call(SET_FLOAT_TRAIT, &args![meter, trait_id, value, 0u32]);
    let set_in_code = e
        .call(TILE_TEXT_TO_TRAIT, &args![SET_IN_CODE_TRAIT_NAME])
        .u32();
    let meter = tile(e, this, 0xf);
    e.call(TILE_POKE, &args![meter, set_in_code, 1.0f32.to_bits()]);
}

/// The deck meter's `_current_value` (tile 15), in ST0.
fn idle_meter_value(e: &mut Engine, this: Ptr<CaravanMenu>) -> f64 {
    let trait_id = e.global::<u32>(UT_CURRENT_VALUE);
    let meter = tile(e, this, 0xf);
    float_trait(e, meter, trait_id)
}

/// `pStartAnimations[player][index]` (+0x2c4, 8 per side).
fn start_animation(menu: u32, side: u32, index: u32) -> u32 {
    menu.wrapping_add(side.wrapping_mul(0x20))
        .wrapping_add(0x2c4)
        .wrapping_add(index.wrapping_mul(4))
}

/// `pHandAnimations[player][index]` (+0x294, 6 per side).
fn hand_animation(menu: u32, side: u32, index: u32) -> u32 {
    menu.wrapping_add(side.wrapping_mul(0x18))
        .wrapping_add(0x294)
        .wrapping_add(index.wrapping_mul(4))
}

/// `pTrackAnimations[track][row][column][which]` (+0x3ac).
fn track_animation(menu: u32, track: u32, row: u32, column: u32, which: u32) -> u32 {
    menu.wrapping_add(track.wrapping_mul(0xe0))
        .wrapping_add(0x3ac)
        .wrapping_add(row << 5)
        .wrapping_add(column.wrapping_mul(8))
        .wrapping_add(which.wrapping_mul(4))
}

/// `pTrackAnimManager[track][row]` (+0x304).
fn track_manager(e: &Engine, menu: u32, track: u32, row: u32) -> u32 {
    let base = menu
        .wrapping_add(track.wrapping_mul(0x1c))
        .wrapping_add(0x304);
    e.mem.u32(base.wrapping_add(row.wrapping_mul(4)))
}

/// `pHandAnimManager[side]` (+0x28c) and `pHandGeometry[side]` (+0x190).
fn hand_manager(e: &Engine, menu: u32, side: u32) -> u32 {
    e.mem
        .u32(menu.wrapping_add(side.wrapping_mul(4)).wrapping_add(0x28c))
}

fn hand_geometry(e: &Engine, menu: u32, side: u32) -> u32 {
    e.mem.u32(
        menu.wrapping_add(side.wrapping_mul(4))
            .wrapping_add(HAND_GEOMETRY),
    )
}

/// The selected track card's sequence (`pTrackAnimations[iSelectedTrack]
/// [iSelectedRow][iSelectedColumn][0]`).
fn selected_track_animation(e: &Engine, menu: u32) -> u32 {
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let column = e.mem.u32(menu + 0xe38);
    e.mem.u32(track_animation(menu, track, row, column, 0))
}

/// `pTrackAnimManager[iSelectedTrack][iSelectedRow]`.
fn selected_track_manager(e: &Engine, menu: u32) -> u32 {
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    track_manager(e, menu, track, row)
}

/// Pokes tile 29 + `side` with `_Value` 1.0 and sets its `_Value` from the
/// size of the side's deck (`00715c60`).
fn idle_show_deck_count(e: &mut Engine, this: Ptr<CaravanMenu>, side: u32) {
    let menu = this.addr();
    let value_trait = e.global::<u32>(UT_VALUE);
    let target = e.mem.u32(menu.wrapping_add(side.wrapping_mul(4)) + 0xa4);
    e.call(TILE_POKE, &args![target, value_trait, 1.0f32.to_bits()]);
    let count = if side == 0 {
        array_size(e, menu + PLAYER_DECK)
    } else {
        array_size(e, menu + NPC_DECK)
    };
    let value_trait = e.global::<u32>(UT_VALUE);
    let target = e.mem.u32(menu.wrapping_add(side.wrapping_mul(4)) + 0xa4);
    e.call(SET_TRAIT_FROM_UNSIGNED, &args![target, value_trait, count]);
}

/// Selects the player's last hand card (`SelectDeckCard(0)` with
/// `iSelectedDeckCard` the hand's size - 1).
fn idle_select_last_hand_card(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    let size = array_size(e, menu + PLAYER_HAND);
    e.mem.set_u32(menu + 0xe2c, size.wrapping_sub(1));
    caravan_menu_select_deck_card(e, this, 0);
}

/// With `CAF_TRACK_SETUP`, sets `CAF_DISABLE_A` when the selected hand
/// card's word +0xb4 is over 10.
fn idle_check_hand_card(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
        let selected = e.mem.u32(menu + 0xe2c);
        if card_b4(e, menu + PLAYER_HAND, selected) as i32 > 10 {
            caravan_menu_set_flag(e, this, CAF_DISABLE_A, 1);
        }
    }
}

/// `UpdateCaravanFlags`, then `UpdateTileAlpha(-1)`.
fn idle_refresh(e: &mut Engine, this: Ptr<CaravanMenu>) {
    caravan_menu_update_caravan_flags(e, this);
    caravan_menu_update_tile_alpha(e, this, -1);
}

/// The stick and key input of one `DoIdle` call (`[EBP-0x14]`,
/// `[EBP-0x28]` and their floats `[EBP-0x24]`, `[EBP-0x20]`).
struct IdleInput {
    x: i32,
    y: i32,
    x_bits: u32,
    y_bits: u32,
}

/// `0043d410` on three blocks (with the idle timer, or 0.0), then the
/// model with the first, the active deck (+0x18c) with the second and the
/// inactive deck (+0x188) with the third.
fn idle_update_decks(e: &mut Engine, this: Ptr<CaravanMenu>, blocks: [u32; 3], timed: bool) {
    for block in blocks {
        let bits = if timed { idle_time_bits(e) } else { 0 };
        idle_update_data(e, block, bits);
    }
    idle_update_model(e, blocks[0]);
    let active = e.mem.u32(this.addr() + ACTIVE_DECK_GEOMETRY);
    idle_update(e, active, blocks[1]);
    let inactive = e.mem.u32(this.addr() + INACTIVE_DECK_GEOMETRY);
    idle_update(e, inactive, blocks[2]);
}

/// `pActiveDeckAnimations[slot]` or `pInactiveDeckAnimations[slot]` is set.
fn idle_decks_present(e: &Engine, menu: u32, slot: u32) -> bool {
    e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * slot) != 0
        || e.mem.u32(menu + INACTIVE_DECK_ANIMATIONS + 4 * slot) != 0
}

/// Activates the two deck sequences of `slot` (active, then inactive).
fn idle_activate_decks(e: &mut Engine, menu: u32, slot: u32) {
    let manager = e.mem.u32(menu + ACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * slot);
    idle_activate(e, manager, sequence);
    let manager = e.mem.u32(menu + INACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + INACTIVE_DECK_ANIMATIONS + 4 * slot);
    idle_activate(e, manager, sequence);
}

/// Stops the two deck sequences of `slot` (active, then inactive).
fn idle_deactivate_decks(e: &mut Engine, menu: u32, slot: u32) {
    let manager = e.mem.u32(menu + ACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * slot);
    idle_deactivate(e, manager, sequence);
    let manager = e.mem.u32(menu + INACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + INACTIVE_DECK_ANIMATIONS + 4 * slot);
    idle_deactivate(e, manager, sequence);
}

/// The active deck sequence of `slot` has run.
fn idle_decks_reached(e: &mut Engine, menu: u32, slot: u32) -> bool {
    let sequence = e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 4 * slot);
    idle_sequence_reached(e, sequence)
}

/// `SwapDeckBuildingTextures(ftol(meter value + offset), 0)`.
fn idle_swap_deck_textures(e: &mut Engine, this: Ptr<CaravanMenu>, value: f64) {
    let index = e.call(FTOL, &args![value]).i32();
    caravan_menu_swap_deck_building_textures(e, this, index, 0);
}

/// The deck building states' end: state 0, both deck sequences of `slot`
/// stopped, `CAF_DISABLE_ALLCONTROLS` cleared, flags and alpha refreshed.
fn idle_finish_decks(e: &mut Engine, this: Ptr<CaravanMenu>, slot: u32) {
    e.set(this, CaravanMenu::eGameState, 0);
    idle_deactivate_decks(e, this.addr(), slot);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    idle_refresh(e, this);
}

/// The deck building states' start: both sequences of `slot` activated,
/// `CAF_DISABLE_ALLCONTROLS` set, alpha refreshed.
fn idle_start_decks(e: &mut Engine, this: Ptr<CaravanMenu>, slot: u32) {
    idle_activate_decks(e, this.addr(), slot);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
    caravan_menu_update_tile_alpha(e, this, -1);
}

/// Decrements `iNumSteps`.
fn idle_step_down(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let steps = e.get(this, CaravanMenu::iNumSteps);
    e.set(this, CaravanMenu::iNumSteps, steps.wrapping_sub(1));
}

// Translated from 00741500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CaravanMenu::DoIdle` (vtable slot +0x2c, Xbox PDB): the menu's frame.
///
/// While a tutorial message is up (`bTutorial`) nothing runs unless the
/// menu is on top and the tutorial (`iTutorial`) has been shown, which
/// clears it. The idle timer (`011d9208`) adds the ticks since
/// `uiPrevIdleTick` (the word +0x14 of `011f6394`) over 1000.0 while the
/// game state stays the one it was counted for (`011d9204`), else restarts
/// at 0.0; off top the menu clears `CAF_ISONTOP` and stops (on top it sets
/// it). When the controls are read (`004b71d0`), the stick (controls
/// `00a23390` with 7 and 8) gives the x and y steps, zero within 7849.0, as
/// floats over 32767.0 (y negated). Off top or in the deck menu the
/// interface tile's trait `0xfa9` is set to 255.0. `CAF_MARKFORCLOSE`
/// moves an on-top menu to state 0x16 and stops; in the results menu a
/// timer past 1.0 in state 0 clears `CAF_DISABLE_ALLCONTROLS`.
///
/// Then each `eGameState` runs its step: 0 the input of the active menu
/// (deck meter steps, the hand card and track selection by keys and stick,
/// the results menu's buttons), 1 to 3 the camera intros (deck, game with
/// the track rectangles placed on screen, results), 4 the money drop, 5 to
/// 10 the deck building animations (navigate, add and remove, change deck,
/// wipe into the game), 11 to 14 dealing, discarding and placing cards,
/// 15, 17 and 18 track discards and jokers, 19 the next joker, 21 the turn
/// change (the opponent's move from `0074fdc0`), 22 the results, 23 the
/// closing fade. Each animation state starts its sequences when the timer
/// is 0.0, updates the animated objects with the timer, and ends when the
/// timer reaches the sequence's float (`00508100`).
///
/// The compiler left one block that never runs in state 0's deck meter
/// (behind `XOR EDX,EDX; JZ`); it is translated behind a constant
/// condition.
// Translation incomplete: 00741500 - the C++ exception-unwinding frame (FS:[0]) that releases the sound handles is not translated
pub fn caravan_menu_do_idle(e: &mut Engine, this: Ptr<CaravanMenu>) {
    e.with_stack(DO_IDLE_FRAME, |e, frame| {
        do_idle_frame(e, this, frame.addr() + DO_IDLE_FRAME);
    });
}

/// `DoIdle`'s body; `ebp` is the frame base its stack locals sit under.
fn do_idle_frame(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let on_top = is_top_menu(e, this);
    if e.get(this, CaravanMenu::bTutorial) != 0 {
        if !on_top {
            return;
        }
        let index = e.mem.u32(menu + TUTORIAL_INDEX);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        if !e
            .call(TUTORIAL_SHOWN, &args![interface.wrapping_add(0x4d4), index])
            .bool()
        {
            return;
        }
        e.set(this, CaravanMenu::bTutorial, 0);
    }
    if e.get(this, CaravanMenu::uiPrevIdleTick) == 0 {
        reset_idle_time(e);
        e.set_global::<u32>(IDLE_TIME_STATE, 0);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
    } else if on_top {
        caravan_menu_set_flag(e, this, CAF_ISONTOP, 1);
        let tick = e.call(WORD_AT_0X14, &args![IDLE_TICK_SOURCE]).u32();
        let elapsed = tick.wrapping_sub(e.get(this, CaravanMenu::uiPrevIdleTick)) as i32;
        if e.global::<u32>(IDLE_TIME_STATE) == e.get(this, CaravanMenu::eGameState) as u32 {
            let seconds = x87_div(f64::from(elapsed), e.global::<f64>(TICKS_PER_SECOND));
            let time = x87_add(seconds, x87_m32(e.global::<u32>(IDLE_TIME)));
            e.set_global::<u32>(IDLE_TIME, x87_store32(time));
        } else {
            reset_idle_time(e);
        }
    } else {
        caravan_menu_set_flag(e, this, CAF_ISONTOP, 0);
        return;
    }
    let mut input = IdleInput {
        x: 0,
        y: 0,
        x_bits: 0,
        y_bits: 0,
    };
    e.call(INTERFACE_SINGLETON, &args![]);
    if e.call(KEYS_BLOCKED_004B71D0, &args![]).bool() {
        let owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
        input.x = e
            .call(CONTROLS_AXIS_00A23390, &args![controls, 0u32, 7u32])
            .i32();
        let owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
        input.y = e
            .call(CONTROLS_AXIS_00A23390, &args![controls, 0u32, 8u32])
            .i32();
        let x_float = x87_store32(f64::from(input.x));
        let magnitude = e.call(FLOAT_MAGNITUDE_00408820, &args![x_float]).f64();
        if magnitude <= e.global::<f64>(STICK_DEAD_ZONE) {
            input.x = 0;
        }
        let range = e.global::<f64>(STICK_RANGE);
        input.x_bits = x87_store32(x87_div(f64::from(input.x), range));
        let y_float = x87_store32(f64::from(input.y));
        let magnitude = e.call(FLOAT_MAGNITUDE_00408820, &args![y_float]).f64();
        if magnitude <= e.global::<f64>(STICK_DEAD_ZONE) {
            input.y = 0;
        }
        let range = e.global::<f64>(STICK_RANGE);
        input.y_bits = x87_store32(x87_div(f64::from(input.y.wrapping_neg()), range));
    }
    let tick = e.call(WORD_AT_0X14, &args![IDLE_TICK_SOURCE]).u32();
    e.set(this, CaravanMenu::uiPrevIdleTick, tick);
    let state = e.get(this, CaravanMenu::eGameState);
    e.set_global::<u32>(IDLE_TIME_STATE, state as u32);
    if !on_top || e.get(this, CaravanMenu::eActiveMenu) == 1 {
        idle_reset_interface_tile(e);
    }
    if caravan_menu_has_flag(e, this, CAF_MARKFORCLOSE) {
        if on_top {
            caravan_menu_set_flag(e, this, CAF_MARKFORCLOSE, 0);
            e.set(this, CaravanMenu::eGameState, 0x16);
        }
        return;
    }
    if e.get(this, CaravanMenu::eActiveMenu) == 3
        && e.get(this, CaravanMenu::eGameState) == 0
        && idle_time(e) > e.global::<f64>(ONE_DOUBLE)
    {
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    }
    match e.get(this, CaravanMenu::eGameState) as u32 {
        0 => idle_input(e, this, &mut input),
        1 => idle_deck_intro(e, this, ebp),
        2 => idle_game_intro(e, this, ebp),
        3 => idle_results_intro(e, this, ebp),
        4 => idle_money_drop(e, this, ebp),
        5 => idle_navigate_forward(e, this, ebp),
        6 => idle_navigate_back(e, this, ebp),
        7 => idle_add_remove(e, this, ebp),
        8 => idle_change_deck_forward(e, this, ebp),
        9 => idle_change_deck_back(e, this, ebp),
        0xa => idle_deck_wipe(e, this, ebp),
        0xb => idle_intro_deal(e, this, ebp),
        0xc => idle_discard_card(e, this, ebp),
        0xd => idle_place_card(e, this, ebp),
        0xe => idle_redraw_card(e, this, ebp),
        0xf => idle_discard_track(e, this, ebp),
        0x11 => idle_remove_row(e, this, ebp),
        0x12 => idle_restore_row(e, this, ebp),
        0x13 => idle_next_joker(e, this),
        0x15 => idle_next_turn(e, this, ebp),
        0x16 => {
            caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
            caravan_menu_prepare_results_menu(e, this);
            e.set(this, CaravanMenu::eGameState, 3);
        }
        0x17 => idle_closing(e, this),
        _ => {}
    }
}

/// State 0x17 (`CA_CLOSING_MENU`): unless the fade state (`0059bb30`, the
/// word +0x24) is 2, sets the interface tile's trait `0xfa9` to 255.0, the
/// menu tile's trait `0x1772` to 1 (`00700320`), starts the fade-out,
/// clears `pMe`, fades out at once when `011d9200` was set, and sets it.
fn idle_closing(e: &mut Engine, this: Ptr<CaravanMenu>) {
    if e.call(WORD_AT_0X24, &args![this]).u32() == 2 {
        return;
    }
    idle_reset_interface_tile(e);
    let menu_tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![menu_tile, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![this]);
    e.set_global::<u32>(CARAVAN_MENU, 0);
    if e.global::<u8>(CLOSE_FADED_BYTE) != 0 {
        e.call(MENU_INSTANT_FADE_OUT, &args![this]);
    }
    e.set_global::<u8>(CLOSE_FADED_BYTE, 1);
}

/// State 1: the deck menu's camera intro (`pCameraAnimations[0]`), then
/// state 0 and the deck building help (tutorial 0x1f).
fn idle_deck_intro(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if e.mem.u32(menu + CAMERA_ANIMATIONS) == 0 {
        return;
    }
    if idle_time_is_zero(e) {
        let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
        let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS);
        idle_activate(e, manager, sequence);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        caravan_menu_update_tile_alpha(e, this, -1);
    }
    let block = ebp - 0x38;
    let bits = idle_time_bits(e);
    idle_update_data(e, block, bits);
    idle_update_model(e, block);
    let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    e.set(this, CaravanMenu::eGameState, 0);
    let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS);
    idle_deactivate(e, manager, sequence);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    caravan_menu_update_tile_alpha(e, this, -1);
    idle_tutorial(e, this, 0x1f, HELP_DECK_BUILDING);
}

/// Sets trait `0xfa1` of tile `index` to the float at `slot` (flag 1).
fn idle_place_track_rect(e: &mut Engine, this: Ptr<CaravanMenu>, index: u32, slot: u32) {
    let bits = x87_quiet_bits(e.mem.u32(slot));
    let target = tile(e, this, index);
    set_float_bits(e, target, 0xfa1, bits);
}

/// State 2: the game menu's intro (`pCameraAnimations[1]` with the two
/// hands' first sequences). At its start the flags are cleared, the
/// selection delay zeroed and the match sound played. At its end the
/// sequences stop, `CAF_TRACK_SETUP` is set and the six track value tiles
/// (23 to 28) are placed at the screen position of `Select1_01:0` of
/// `pTrackGrids[0]` (`WorldPtToScreenPt` on `sp3DCamera`) times the menu's
/// trait `0xfb1` (truncated) less 120.0, 265.0 apart; the track rectangles
/// get trait `0xfa3` 1, then state 0x15 and the starting caravans help
/// (tutorial 0x20).
fn idle_game_intro(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if e.mem.u32(menu + CAMERA_ANIMATIONS + 4) == 0
        || e.mem.u32(menu + 0x294) == 0
        || e.mem.u32(menu + 0x2ac) == 0
    {
        return;
    }
    if idle_time_is_zero(e) {
        let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
        let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 4);
        idle_activate(e, manager, sequence);
        caravan_menu_clear_flags(e, this);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        let manager = e.mem.u32(menu + HAND_ANIM_MANAGER);
        let sequence = e.mem.u32(menu + 0x294);
        idle_activate(e, manager, sequence);
        let manager = e.mem.u32(menu + HAND_ANIM_MANAGER + 4);
        let sequence = e.mem.u32(menu + 0x2ac);
        idle_activate(e, manager, sequence);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
        play_sound(e, START_MATCH_SOUND);
        caravan_menu_update_tile_alpha(e, this, -1);
    }
    let blocks = [ebp - 0x54, ebp - 0x48, ebp - 0x60];
    for block in blocks {
        let bits = idle_time_bits(e);
        idle_update_data(e, block, bits);
    }
    let hand = e.mem.u32(menu + HAND_GEOMETRY);
    idle_update(e, hand, blocks[1]);
    let hand = e.mem.u32(menu + HAND_GEOMETRY + 4);
    idle_update(e, hand, blocks[2]);
    idle_update_model(e, blocks[0]);
    let camera_sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 4);
    if !idle_sequence_reached(e, camera_sequence) {
        return;
    }
    let sequence = e.mem.u32(menu + 0x294);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let sequence = e.mem.u32(menu + 0x2ac);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 4);
    idle_deactivate(e, manager, sequence);
    let manager = e.mem.u32(menu + HAND_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + 0x294);
    idle_deactivate(e, manager, sequence);
    let manager = e.mem.u32(menu + HAND_ANIM_MANAGER + 4);
    let sequence = e.mem.u32(menu + 0x2ac);
    idle_deactivate(e, manager, sequence);
    caravan_menu_set_flag(e, this, CAF_TRACK_SETUP, 1);
    let x_slot = ebp - 0x68;
    let y_slot = ebp - 0x64;
    e.mem.set_u32(x_slot, 0);
    e.mem.set_u32(y_slot, 0);
    let root = tile(e, this, 2);
    let width = float_trait(e, root, 0xfb1);
    let width = e.call(FTOL, &args![width]).i32();
    let grids = e.mem.u32(menu + TRACK_GRIDS);
    let node = e
        .call(OBJECT_BY_NAME, &args![grids, FIRST_SELECT_NAME])
        .u32();
    let tolerance = x87_quiet_bits(e.global::<u32>(PROJECTION_TOLERANCE));
    let point = e.call(PLUS_0X8C, &args![node]).u32();
    let camera = e.call(FIRST_WORD, &args![CAMERA_POINTER]).u32();
    e.call(
        WORLD_POINT_TO_SCREEN,
        &args![camera, point, x_slot, y_slot, tolerance],
    );
    let left = x87_sub(
        x87_mul(f64::from(width), x87_m32(e.mem.u32(x_slot))),
        e.global::<f64>(TRACK_RECT_LEFT),
    );
    e.mem.set_u32(x_slot, x87_store32(left));
    idle_place_track_rect(e, this, 0x17, x_slot);
    idle_place_track_rect(e, this, 0x1c, x_slot);
    let next = x87_add(x87_m32(e.mem.u32(x_slot)), e.global::<f64>(TRACK_RECT_STEP));
    e.mem.set_u32(x_slot, x87_store32(next));
    idle_place_track_rect(e, this, 0x18, x_slot);
    idle_place_track_rect(e, this, 0x1b, x_slot);
    let next = x87_add(x87_m32(e.mem.u32(x_slot)), e.global::<f64>(TRACK_RECT_STEP));
    e.mem.set_u32(x_slot, x87_store32(next));
    idle_place_track_rect(e, this, 0x19, x_slot);
    idle_place_track_rect(e, this, 0x1a, x_slot);
    for name in [NPC_TRACK_RECT_NAME, PLAYER_TRACK_RECT_NAME] {
        let root = tile(e, this, 2);
        let rect = e.call(TILE_CHILD_NAMED, &args![root, name]).u32();
        e.call(SET_TRAIT_FROM_INT, &args![rect, 0xfa3u32, 1u32]);
    }
    idle_refresh(e, this);
    e.set(this, CaravanMenu::eGameState, 0x15);
    idle_tutorial(e, this, 0x20, HELP_STARTING_CARAVANS);
}

/// State 3: the results menu's camera intro (`pCameraAnimations[2]`),
/// then state 0 in the results menu (`eActiveMenu` 3) with `CAF_AI`
/// cleared and tile 35's caption from setting `011d372c`.
fn idle_results_intro(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if e.mem.u32(menu + CAMERA_ANIMATIONS + 8) == 0 {
        return;
    }
    if idle_time_is_zero(e) {
        let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
        let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 8);
        idle_activate(e, manager, sequence);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        caravan_menu_update_tile_alpha(e, this, -1);
    }
    let block = ebp - 0x80;
    let bits = idle_time_bits(e);
    idle_update_data(e, block, bits);
    idle_update_model(e, block);
    let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 8);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    e.set(this, CaravanMenu::eGameState, 0);
    e.set(this, CaravanMenu::eActiveMenu, 3);
    let manager = e.mem.u32(menu + CAMERA_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + CAMERA_ANIMATIONS + 8);
    idle_deactivate(e, manager, sequence);
    caravan_menu_set_flag(e, this, CAF_AI, 0);
    set_caption(e, this, 0x23, RESULTS_CAPTION_SETTING);
    idle_refresh(e, this);
}

/// State 4: the money drop. At its start the funds sound plays; the bills
/// and coins from `iBillIndexToAnim` and `iCoinIndexToAnim` on are updated
/// with the timer; past 1.0 the indexes move to the lists' ends and the
/// state returns to 0.
fn idle_money_drop(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if idle_time_is_zero(e) {
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        play_sound(e, FUNDS_DROP_SOUND);
    }
    let mut index = e.mem.u32(menu + BILL_INDEX_TO_ANIM);
    while index < array_size(e, menu + LIST_OF_BILLS) {
        let bits = idle_time_bits(e);
        let block = idle_update_data(e, ebp - 0x3c0, bits);
        let bill = array_element(e, menu + LIST_OF_BILLS, index);
        idle_update(e, bill, block);
        index = index.wrapping_add(1);
    }
    let mut index = e.mem.u32(menu + COIN_INDEX_TO_ANIM);
    while index < array_size(e, menu + LIST_OF_COINS) {
        let bits = idle_time_bits(e);
        let block = idle_update_data(e, ebp - 0x3cc, bits);
        let coin = array_element(e, menu + LIST_OF_COINS, index);
        idle_update(e, coin, block);
        index = index.wrapping_add(1);
    }
    if idle_time(e) > e.global::<f64>(ONE_DOUBLE) {
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
        let bills = array_size(e, menu + LIST_OF_BILLS);
        e.mem.set_u32(menu + BILL_INDEX_TO_ANIM, bills);
        let coins = array_size(e, menu + LIST_OF_COINS);
        e.mem.set_u32(menu + COIN_INDEX_TO_ANIM, coins);
        e.set(this, CaravanMenu::eGameState, 0);
    }
}

/// State 5: the deck navigates forward (`iNumSteps` steps, deck slot 0).
fn idle_navigate_forward(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 0) {
        return;
    }
    if idle_time_is_zero(e) {
        idle_start_decks(e, this, 0);
        idle_step_down(e, this);
        play_sound(e, NAVIGATE_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0xa0, ebp - 0x94, ebp - 0xac], true);
    if !idle_decks_reached(e, menu, 0) {
        return;
    }
    let value = idle_meter_value(e, this);
    let steps = e.get(this, CaravanMenu::iNumSteps);
    idle_swap_deck_textures(e, this, x87_sub(value, f64::from(steps)));
    idle_update_decks(e, this, [ebp - 0xc4, ebp - 0xb8, ebp - 0xd0], false);
    if e.get(this, CaravanMenu::iNumSteps) <= 0 {
        idle_finish_decks(e, this, 0);
    } else {
        reset_idle_time(e);
        idle_step_down(e, this);
    }
    e.mem.set_u32(menu + PENDING_KEY, 0);
}

/// State 6: the deck navigates back (deck slot 1).
fn idle_navigate_back(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 1) {
        return;
    }
    if idle_time_is_zero(e) {
        idle_start_decks(e, this, 1);
        idle_step_down(e, this);
        let value = idle_meter_value(e, this);
        let steps = e.get(this, CaravanMenu::iNumSteps);
        idle_swap_deck_textures(e, this, x87_add(value, f64::from(steps)));
        play_sound(e, NAVIGATE_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0xe8, ebp - 0xdc, ebp - 0xf4], true);
    if !idle_decks_reached(e, menu, 1) {
        return;
    }
    if e.get(this, CaravanMenu::iNumSteps) <= 0 {
        idle_finish_decks(e, this, 1);
    } else {
        reset_idle_time(e);
        // The exe tests iNumSteps again (unchanged, so above 0 here): the
        // first branch, which would stop the forward deck sequences,
        // cannot run.
        if e.get(this, CaravanMenu::iNumSteps) <= 0 {
            idle_finish_decks(e, this, 0);
        } else {
            reset_idle_time(e);
            idle_step_down(e, this);
        }
    }
    e.mem.set_u32(menu + PENDING_KEY, 0);
}

/// State 7: a card is added to or removed from the deck (deck slot 2);
/// at the end `0074af70(0xb)` and the objects updated at 0.0.
fn idle_add_remove(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 2) {
        return;
    }
    if idle_time_is_zero(e) {
        idle_start_decks(e, this, 2);
        play_sound(e, ADD_REMOVE_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0x154, ebp - 0x148, ebp - 0x160], true);
    if !idle_decks_reached(e, menu, 2) {
        return;
    }
    e.set(this, CaravanMenu::eGameState, 0);
    fn_0074af70(e, this, 0xb);
    idle_update_decks(e, this, [ebp - 0x178, ebp - 0x16c, ebp - 0x184], false);
    idle_deactivate_decks(e, menu, 2);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    idle_refresh(e, this);
}

/// State 8: the deck changes forward (deck slot 3; the textures swap at
/// the start).
fn idle_change_deck_forward(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 3) {
        return;
    }
    if idle_time_is_zero(e) {
        let value = idle_meter_value(e, this);
        idle_swap_deck_textures(e, this, value);
        idle_start_decks(e, this, 3);
        play_sound(e, CHANGE_DECK_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0x10c, ebp - 0x100, ebp - 0x118], true);
    if !idle_decks_reached(e, menu, 3) {
        return;
    }
    idle_finish_decks(e, this, 3);
    e.mem.set_u32(menu + PENDING_KEY, 0);
}

/// State 9: the deck changes back (deck slot 4; the textures swap at the
/// end).
fn idle_change_deck_back(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 4) {
        return;
    }
    if idle_time_is_zero(e) {
        idle_start_decks(e, this, 4);
        play_sound(e, CHANGE_DECK_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0x130, ebp - 0x124, ebp - 0x13c], true);
    if !idle_decks_reached(e, menu, 4) {
        return;
    }
    e.set(this, CaravanMenu::eGameState, 0);
    let value = idle_meter_value(e, this);
    idle_swap_deck_textures(e, this, value);
    idle_deactivate_decks(e, menu, 4);
    caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    idle_refresh(e, this);
    e.mem.set_u32(menu + PENDING_KEY, 0);
}

/// State 10: the deck wipes into the game (deck slot 5); at the end
/// `0074af70(0xb)`, the byte +0xe86 set, `CAF_AI` set and
/// `PrepareGameMenu`.
fn idle_deck_wipe(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    if !idle_decks_present(e, menu, 5) {
        return;
    }
    if idle_time_is_zero(e) {
        idle_start_decks(e, this, 5);
        play_sound(e, CHANGE_DECK_SOUND);
    }
    idle_update_decks(e, this, [ebp - 0x19c, ebp - 0x190, ebp - 0x1a8], true);
    if !idle_decks_reached(e, menu, 5) {
        return;
    }
    e.set(this, CaravanMenu::eGameState, 0);
    fn_0074af70(e, this, 0xb);
    idle_update_decks(e, this, [ebp - 0x1c0, ebp - 0x1b4, ebp - 0x1cc], true);
    let manager = e.mem.u32(menu + ACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 0x14);
    idle_deactivate(e, manager, sequence);
    let manager = e.mem.u32(menu + INACTIVE_DECK_ANIM_MANAGER);
    let sequence = e.mem.u32(menu + INACTIVE_DECK_ANIMATIONS + 0x14);
    idle_deactivate(e, manager, sequence);
    e.mem.set_u8(menu + BYTE_E86, 1);
    caravan_menu_set_flag(e, this, CAF_AI, 1);
    idle_refresh(e, this);
    caravan_menu_prepare_game_menu(e, this);
}

/// `CAF_AI` as the side index (0 the player, 1 the opponent).
fn idle_side(e: &mut Engine, this: Ptr<CaravanMenu>) -> u32 {
    u32::from(caravan_menu_has_flag(e, this, CAF_AI))
}

/// The hand cards' redraw after a deal, discard or placement: when the
/// side's track 2 (player) or track 3 (opponent) list is filled
/// (`(size(+0xc24) != 0 && !CAF_AI) || (size(+0xc94) != 0 && CAF_AI)`)
/// the hand textures are updated with `CAF_TRACK_SETUP` and the side's
/// hand sequence 1 is run to 0.0 (and, from the deal, with both lists
/// filled on the player's side `CAF_TRACK_SETUP` is cleared and the
/// contract war help shown, tutorial 0x21); otherwise the textures are
/// updated with 1 and the side's start sequence 0 is run to 0.0. Then all
/// the side's hand sequences stop.
fn idle_redraw_hand(
    e: &mut Engine,
    this: Ptr<CaravanMenu>,
    side: u32,
    blocks: [u32; 2],
    from_deal: bool,
) {
    let menu = this.addr();
    let filled = if array_size(e, menu + 0xc24) != 0 && !caravan_menu_has_flag(e, this, CAF_AI) {
        true
    } else {
        array_size(e, menu + 0xc94) != 0 && caravan_menu_has_flag(e, this, CAF_AI)
    };
    if filled {
        let setup = u32::from(caravan_menu_has_flag(e, this, CAF_TRACK_SETUP));
        let player = u8::from(!caravan_menu_has_flag(e, this, CAF_AI));
        caravan_menu_update_game_hand_textures(e, this, player, setup);
        let manager = hand_manager(e, menu, side);
        let sequence = e.mem.u32(hand_animation(menu, side, 1));
        idle_activate(e, manager, sequence);
        idle_update_data(e, blocks[0], 0);
        let hand = hand_geometry(e, menu, side);
        idle_update(e, hand, blocks[0]);
        let manager = hand_manager(e, menu, side);
        let sequence = e.mem.u32(hand_animation(menu, side, 1));
        idle_deactivate(e, manager, sequence);
        if from_deal
            && !caravan_menu_has_flag(e, this, CAF_AI)
            && array_size(e, menu + 0xc24) != 0
            && array_size(e, menu + 0xc94) != 0
        {
            caravan_menu_set_flag(e, this, CAF_TRACK_SETUP, 0);
            idle_tutorial(e, this, 0x21, HELP_CONTRACT_WAR);
        }
    } else {
        let player = u8::from(!caravan_menu_has_flag(e, this, CAF_AI));
        caravan_menu_update_game_hand_textures(e, this, player, 1);
        let manager = hand_manager(e, menu, side);
        let sequence = e.mem.u32(start_animation(menu, side, 0));
        idle_activate(e, manager, sequence);
        idle_update_data(e, blocks[1], 0);
        let hand = hand_geometry(e, menu, side);
        idle_update(e, hand, blocks[1]);
        let manager = hand_manager(e, menu, side);
        let sequence = e.mem.u32(start_animation(menu, side, 0));
        idle_deactivate(e, manager, sequence);
    }
    let manager = hand_manager(e, menu, side);
    e.call(DEACTIVATE_ALL, &args![manager, 0u32]);
}

/// State 11 (`CA_GAME_INTRO_DEAL_ANIM`): the selected start card is dealt
/// onto the selected track card (`0074df90` first, the player's selection
/// cleared, the confirm sound). At the end the hand is redrawn, the
/// opponent's last hand card selected, the deck count shown, state 0x15.
fn idle_intro_deal(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let side = idle_side(e, this);
    let selected = e.mem.u32(menu + 0xe2c);
    if e.mem.u32(start_animation(menu, side, selected)) == 0
        || selected_track_animation(e, menu) == 0
    {
        return;
    }
    if idle_time_is_zero(e) {
        fn_0074df90(e, this);
        let manager = hand_manager(e, menu, side);
        let selected = e.mem.u32(menu + 0xe2c);
        let sequence = e.mem.u32(start_animation(menu, side, selected));
        idle_activate(e, manager, sequence);
        let manager = selected_track_manager(e, menu);
        let sequence = selected_track_animation(e, menu);
        idle_activate(e, manager, sequence);
        if side == 0 {
            caravan_menu_select_deck_card(e, this, 1);
        }
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
        play_sound(e, CONFIRM_ADD_TO_DECK_SOUND);
    }
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x1e8, bits);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x1e8);
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x1dc, bits);
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let geometry = track_geometry(e, menu, track, row);
    idle_update(e, geometry, ebp - 0x1dc);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(start_animation(menu, side, selected));
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let sequence = selected_track_animation(e, menu);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let manager = hand_manager(e, menu, side);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(start_animation(menu, side, selected));
    idle_deactivate(e, manager, sequence);
    let manager = selected_track_manager(e, menu);
    let sequence = selected_track_animation(e, menu);
    idle_deactivate(e, manager, sequence);
    idle_redraw_hand(e, this, side, [ebp - 0x1f4, ebp - 0x204], true);
    if side == 1 {
        idle_select_last_hand_card(e, this);
        idle_check_hand_card(e, this);
    }
    idle_show_deck_count(e, this, side);
    idle_refresh(e, this);
    e.set(this, CaravanMenu::eGameState, 0x15);
}

/// State 12: the selected start card is discarded (the remove sound). At
/// the end the hand is redrawn, the player's last hand card selected
/// (player side), the deck count shown; on the opponent's side state 0x15,
/// else state 0 with `CAF_DISABLE_ALLCONTROLS` cleared.
fn idle_discard_card(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let side = idle_side(e, this);
    let selected = e.mem.u32(menu + 0xe2c);
    if e.mem.u32(start_animation(menu, side, selected)) == 0 {
        return;
    }
    if idle_time_is_zero(e) {
        let manager = hand_manager(e, menu, side);
        let selected = e.mem.u32(menu + 0xe2c);
        let sequence = e.mem.u32(start_animation(menu, side, selected));
        idle_activate(e, manager, sequence);
        if side == 0 {
            caravan_menu_select_deck_card(e, this, 1);
        }
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
        play_sound(e, REMOVE_CARD_SOUND);
    }
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x214, bits);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x214);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(start_animation(menu, side, selected));
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let manager = hand_manager(e, menu, side);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(start_animation(menu, side, selected));
    idle_deactivate(e, manager, sequence);
    idle_redraw_hand(e, this, side, [ebp - 0x220, ebp - 0x22c], false);
    if side == 0 {
        idle_select_last_hand_card(e, this);
        idle_check_hand_card(e, this);
    }
    idle_show_deck_count(e, this, side);
    idle_refresh(e, this);
    if caravan_menu_has_flag(e, this, CAF_AI) {
        e.set(this, CaravanMenu::eGameState, 0x15);
    } else {
        e.set(this, CaravanMenu::eGameState, 0);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
    }
}

/// The word +0xb4 of `pSelectedCard` (`008d80e0`).
fn selected_card_b4(e: &mut Engine, menu: u32) -> u32 {
    let card = e.mem.u32(menu + SELECTED_CARD);
    e.call(CARD_WORD_B4, &args![card]).u32()
}

/// State 13 (`CA_GAME_PLACECARD_ANIM`): the selected hand card (or, with
/// no next card for the side, the start card) is placed onto the selected
/// track card. At the end the hand textures are updated and the side's
/// hand sequence 1 run to 0.0; on the opponent's side, unless the placed
/// card's word +0xb4 is 12 or 15, the player's last hand card is selected.
/// A placed 12 or 15 shows the deck count and moves to state 0x11 (12) or
/// 0x13 (15): for 0x13 every other track card (rows until an empty one)
/// whose word +0xb4 matches the track's first card (or, when that is 1,
/// whose word +0xb8 matches) has its row appended to the track's
/// `jokerLocations`, and `iSelectedTrack` is cleared. Otherwise the deck
/// count is shown and state 0x15 follows.
fn idle_place_card(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let side = idle_side(e, this);
    let selected = e.mem.u32(menu + 0xe2c);
    if e.mem
        .u32(hand_animation(menu, side, selected).wrapping_add(4))
        == 0
        || selected_track_animation(e, menu) == 0
    {
        return;
    }
    if idle_time_is_zero(e) {
        fn_0074df90(e, this);
        let next = if side == 0 {
            e.mem.u32(menu + NEXT_PLAYER_CARD)
        } else {
            e.mem.u32(menu + NEXT_NPC_CARD)
        };
        let manager = hand_manager(e, menu, side);
        let selected = e.mem.u32(menu + 0xe2c);
        let sequence = if next == u32::MAX {
            e.mem.u32(start_animation(menu, side, selected))
        } else {
            e.mem
                .u32(hand_animation(menu, side, selected).wrapping_add(4))
        };
        idle_activate(e, manager, sequence);
        let manager = selected_track_manager(e, menu);
        let sequence = selected_track_animation(e, menu);
        idle_activate(e, manager, sequence);
        if side == 0 {
            caravan_menu_select_deck_card(e, this, 1);
        }
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
        play_sound(e, CONFIRM_ADD_TO_TRACK_SOUND);
    }
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x248, bits);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x248);
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x23c, bits);
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let geometry = track_geometry(e, menu, track, row);
    idle_update(e, geometry, ebp - 0x23c);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(hand_animation(menu, side, selected));
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let sequence = selected_track_animation(e, menu);
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    let manager = hand_manager(e, menu, side);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e
        .mem
        .u32(hand_animation(menu, side, selected).wrapping_add(4));
    idle_deactivate(e, manager, sequence);
    let manager = selected_track_manager(e, menu);
    let sequence = selected_track_animation(e, menu);
    idle_deactivate(e, manager, sequence);
    let setup = u32::from(caravan_menu_has_flag(e, this, CAF_TRACK_SETUP));
    let player = u8::from(!caravan_menu_has_flag(e, this, CAF_AI));
    caravan_menu_update_game_hand_textures(e, this, player, setup);
    let manager = hand_manager(e, menu, side);
    let sequence = e.mem.u32(hand_animation(menu, side, 1));
    idle_activate(e, manager, sequence);
    idle_update_data(e, ebp - 0x254, 0);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x254);
    let manager = hand_manager(e, menu, side);
    let sequence = e.mem.u32(hand_animation(menu, side, 1));
    idle_deactivate(e, manager, sequence);
    if side == 1 && selected_card_b4(e, menu) != 0xc && selected_card_b4(e, menu) != 0xf {
        idle_select_last_hand_card(e, this);
        idle_check_hand_card(e, this);
    } else if selected_card_b4(e, menu) == 0xc || selected_card_b4(e, menu) == 0xf {
        idle_show_deck_count(e, this, side);
        let next = if selected_card_b4(e, menu) != 0xc {
            0x13
        } else {
            0x11
        };
        e.set(this, CaravanMenu::eGameState, next);
        if e.get(this, CaravanMenu::eGameState) == 0x13 {
            idle_collect_jokers(e, this, ebp);
        }
        return;
    }
    idle_show_deck_count(e, this, side);
    idle_refresh(e, this);
    e.set(this, CaravanMenu::eGameState, 0x15);
}

/// State 13's joker search: the first card of the selected track row
/// gives a suit (its word +0xb4) and a value (+0xb8); for each track and
/// row up to the first empty row, a card other than the selected one whose
/// +0xb4 matches (or, for suit 1, whose +0xb8 matches) has its row (the
/// stack local at `EBP-0x264`) appended to the track's `jokerLocations`.
/// Clears `iSelectedTrack`.
fn idle_collect_jokers(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let suit = card_b4(e, card_track(menu, track, row), 0);
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let value = card_b8(e, card_track(menu, track, row), 0);
    let row_slot = ebp - 0x264;
    for track in 0..6u32 {
        e.mem.set_u32(row_slot, 0);
        while e.mem.i32(row_slot) < 7 {
            let row = e.mem.u32(row_slot);
            let cards = card_track(menu, track, row);
            if array_size(e, cards) == 0 {
                break;
            }
            let matches = if suit != 1 {
                card_b4(e, cards, 0) == suit
            } else {
                card_b8(e, cards, 0) == value
            };
            if matches && (track != e.mem.u32(menu + 0xe30) || row != e.mem.u32(menu + 0xe34)) {
                let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
                e.call(ARRAY_APPEND, &args![locations, row_slot]);
            }
            let row = e.mem.u32(row_slot);
            e.mem.set_u32(row_slot, row.wrapping_add(1));
        }
    }
    e.mem.set_u32(menu + 0xe30, 0);
}

/// State 14: the selected hand card is redrawn (with no next card for the
/// side the start card, with the remove sound; else the hand card, with
/// the add-remove sound). At the end the hand textures are updated (player
/// side for side 0, flag 0), the side's hand sequence 1 run to 0.0, the
/// deck count shown, state 0x15, the player's last hand card made the
/// selection (selected for the opponent without `CAF_TRACK_SETUP` or the
/// player with it).
fn idle_redraw_card(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let side = idle_side(e, this);
    let selected = e.mem.u32(menu + 0xe2c);
    if e.mem
        .u32(hand_animation(menu, side, selected).wrapping_add(4))
        == 0
    {
        return;
    }
    if idle_time_is_zero(e) {
        let next = if side == 0 {
            e.mem.u32(menu + NEXT_PLAYER_CARD)
        } else {
            e.mem.u32(menu + NEXT_NPC_CARD)
        };
        let manager = hand_manager(e, menu, side);
        let selected = e.mem.u32(menu + 0xe2c);
        if next == u32::MAX {
            let sequence = e.mem.u32(start_animation(menu, side, selected));
            idle_activate(e, manager, sequence);
            play_sound(e, REMOVE_CARD_SOUND);
        } else {
            let sequence = e
                .mem
                .u32(hand_animation(menu, side, selected).wrapping_add(4));
            idle_activate(e, manager, sequence);
            play_sound(e, ADD_REMOVE_CARD_SOUND);
        }
        if side == 0 {
            caravan_menu_select_deck_card(e, this, 1);
        }
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
    }
    let bits = idle_time_bits(e);
    idle_update_data(e, ebp - 0x274, bits);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x274);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e.mem.u32(hand_animation(menu, side, selected));
    if !idle_sequence_reached(e, sequence) {
        return;
    }
    caravan_menu_update_game_hand_textures(e, this, u8::from(side == 0), 0);
    let manager = hand_manager(e, menu, side);
    let selected = e.mem.u32(menu + 0xe2c);
    let sequence = e
        .mem
        .u32(hand_animation(menu, side, selected).wrapping_add(4));
    idle_deactivate(e, manager, sequence);
    let manager = hand_manager(e, menu, side);
    let sequence = e.mem.u32(hand_animation(menu, side, 1));
    idle_activate(e, manager, sequence);
    idle_update_data(e, ebp - 0x280, 0);
    let hand = hand_geometry(e, menu, side);
    idle_update(e, hand, ebp - 0x280);
    let manager = hand_manager(e, menu, side);
    let sequence = e.mem.u32(hand_animation(menu, side, 1));
    idle_deactivate(e, manager, sequence);
    idle_refresh(e, this);
    idle_show_deck_count(e, this, side);
    e.set(this, CaravanMenu::eGameState, 0x15);
    let size = array_size(e, menu + PLAYER_HAND);
    e.mem.set_u32(menu + 0xe2c, size.wrapping_sub(1));
    idle_check_hand_card(e, this);
    let opponent_side = side == 1 && !caravan_menu_has_flag(e, this, CAF_TRACK_SETUP);
    if opponent_side || (side == 0 && caravan_menu_has_flag(e, this, CAF_TRACK_SETUP)) {
        caravan_menu_select_deck_card(e, this, 0);
    }
}

/// Activates sequence `which` of column `iSelectedColumn` on every row
/// from `first` of the selected track whose card list holds more cards than
/// the column; answers how many.
fn idle_activate_column(e: &mut Engine, menu: u32, first: u32, which: u32) -> i32 {
    let mut count = 0;
    let mut row = first as i32;
    while row < 7 {
        let track = e.mem.u32(menu + 0xe30);
        let cards = array_size(e, card_track(menu, track, row as u32)) as i32;
        if cards > e.mem.i32(menu + 0xe38) {
            let track = e.mem.u32(menu + 0xe30);
            let column = e.mem.u32(menu + 0xe38);
            let sequence = e
                .mem
                .u32(track_animation(menu, track, row as u32, column, which));
            let manager = track_manager(e, menu, track, row as u32);
            idle_activate(e, manager, sequence);
            count += 1;
        }
        row += 1;
    }
    count
}

/// Stops sequence `which` of column `iSelectedColumn` on every row from
/// `first` of the selected track.
fn idle_deactivate_column(e: &mut Engine, menu: u32, first: u32, which: u32) {
    let mut row = first as i32;
    while row < 7 {
        let track = e.mem.u32(menu + 0xe30);
        let column = e.mem.u32(menu + 0xe38);
        let sequence = e
            .mem
            .u32(track_animation(menu, track, row as u32, column, which));
        let manager = track_manager(e, menu, track, row as u32);
        idle_deactivate(e, manager, sequence);
        row += 1;
    }
}

/// Updates the selected track's geometry with the timer from row `first`
/// until an empty row (`block` the stack block).
fn idle_update_track_rows(e: &mut Engine, menu: u32, first: u32, block: u32) {
    let mut row = first as i32;
    while row < 7 {
        let track = e.mem.u32(menu + 0xe30);
        if array_size(e, card_track(menu, track, row as u32)) == 0 {
            break;
        }
        let bits = idle_time_bits(e);
        let data = idle_update_data(e, block, bits);
        let track = e.mem.u32(menu + 0xe30);
        let geometry = track_geometry(e, menu, track, row as u32);
        idle_update(e, geometry, data);
        row += 1;
    }
}

/// While nothing was activated and `iSelectedColumn` is above 0: one column
/// down, its sequence 1 activated from row `first`.
fn idle_next_column_down(e: &mut Engine, menu: u32, first_row: u32, count: &mut i32) {
    while *count == 0 && e.mem.i32(menu + 0xe38) > 0 {
        let column = e.mem.i32(menu + 0xe38);
        e.mem.set_u32(menu + 0xe38, column.wrapping_sub(1) as u32);
        let first = if first_row == u32::MAX {
            e.mem.u32(menu + 0xe34)
        } else {
            first_row
        };
        *count += idle_activate_column(e, menu, first, 1);
    }
}

/// State 15 (`CA_GAME_DISCARDTRACK_ANIM`): the selected track's cards go,
/// column by column from the top (sequence 1). When none is left the
/// track's seven card lists are released (`008454f0`), state 0x15, the
/// player's last hand card selected on the opponent's turn, the track's
/// value updated and `CAF_DESTROY_TRACK` and `CAF_SELECT_TRACK` cleared.
fn idle_discard_track(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let track = e.mem.u32(menu + 0xe30);
    if e.mem.u32(track_animation(menu, track, 0, 0, 1)) == 0 {
        return;
    }
    let mut count = 0;
    if idle_time_is_zero(e) {
        e.mem.set_u32(menu + 0xe38, 4);
        idle_next_column_down(e, menu, 0, &mut count);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
        e.mem.set_u32(menu + SELECTION_DELAY, 0);
        play_sound(e, REMOVE_CARD_SOUND);
    }
    idle_update_track_rows(e, menu, 0, ebp - 0x468);
    let track = e.mem.u32(menu + 0xe30);
    let column = e.mem.u32(menu + 0xe38);
    let sequence = e.mem.u32(track_animation(menu, track, 0, column, 1));
    if !idle_sequence_passed(e, sequence) {
        return;
    }
    idle_deactivate_column(e, menu, 0, 1);
    idle_next_column_down(e, menu, 0, &mut count);
    if count != 0 {
        reset_idle_time(e);
        return;
    }
    for row in 0..7 {
        let track = e.mem.u32(menu + 0xe30);
        e.call(
            ARRAY_RELEASE_008454F0,
            &args![card_track(menu, track, row), 1u32],
        );
    }
    e.set(this, CaravanMenu::eGameState, 0x15);
    if caravan_menu_has_flag(e, this, CAF_AI) {
        idle_select_last_hand_card(e, this);
    }
    let track = e.mem.u32(menu + 0xe30);
    caravan_menu_update_track_value(e, this, track);
    caravan_menu_set_flag(e, this, CAF_DESTROY_TRACK, 0);
    caravan_menu_set_flag(e, this, CAF_SELECT_TRACK, 0);
}

/// Moves the selected track's rows from `first` up by one (rows 0 to 5):
/// each row's list is released and refilled from the next row's cards,
/// `0074e220(track, row, index)` for each.
fn idle_shift_rows(e: &mut Engine, this: Ptr<CaravanMenu>, first: i32) {
    let menu = this.addr();
    let mut row = first;
    while row < 6 {
        let track = e.mem.u32(menu + 0xe30);
        e.call(
            ARRAY_RELEASE_008454F0,
            &args![card_track(menu, track, row as u32), 1u32],
        );
        let track = e.mem.u32(menu + 0xe30);
        let source = card_track(menu, track, row.wrapping_add(1) as u32);
        let mut index = 0u32;
        while index < array_size(e, source) {
            let slot = array_slot(e, source, index);
            let track = e.mem.u32(menu + 0xe30);
            e.call(
                ARRAY_APPEND,
                &args![card_track(menu, track, row as u32), slot],
            );
            let track = e.mem.u32(menu + 0xe30);
            fn_0074e220(e, this, track, row as u32, index);
            index = index.wrapping_add(1);
        }
        row += 1;
    }
}

/// State 17: the selected row and the rows after it go (column by column,
/// sequence 1). When none is left the rows move up: without `CAF_JOKER`
/// once from the selected row (the last row released); with it from each
/// joker location in turn (`jokerLocations` of the track, its first entry
/// removed each round, one lower when past the previous start). Then
/// state 0x12.
fn idle_remove_row(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    if e.mem.u32(track_animation(menu, track, row, 0, 1)) == 0 {
        return;
    }
    let mut count = 0;
    if idle_time_is_zero(e) {
        e.mem.set_u32(menu + 0xe38, 4);
        idle_next_column_down(e, menu, u32::MAX, &mut count);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
    }
    let first = e.mem.u32(menu + 0xe34);
    idle_update_track_rows(e, menu, first, ebp - 0x474);
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let column = e.mem.u32(menu + 0xe38);
    let sequence = e.mem.u32(track_animation(menu, track, row, column, 1));
    if !idle_sequence_passed(e, sequence) {
        return;
    }
    let first = e.mem.u32(menu + 0xe34);
    idle_deactivate_column(e, menu, first, 1);
    idle_next_column_down(e, menu, u32::MAX, &mut count);
    if count != 0 {
        reset_idle_time(e);
        return;
    }
    if !caravan_menu_has_flag(e, this, CAF_JOKER) {
        let first = e.mem.i32(menu + 0xe34);
        idle_shift_rows(e, this, first);
        let track = e.mem.u32(menu + 0xe30);
        e.call(
            ARRAY_RELEASE_008454F0,
            &args![card_track(menu, track, 6), 1u32],
        );
    } else {
        let mut more = true;
        let mut start = e.mem.i32(menu + 0xe34);
        while more {
            idle_shift_rows(e, this, start);
            let track = e.mem.u32(menu + 0xe30);
            let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
            e.call(ARRAY_REMOVE_AT, &args![locations, 0u32, 1u32]);
            let track = e.mem.u32(menu + 0xe30);
            let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
            if array_size(e, locations) != 0 {
                let previous = start;
                let track = e.mem.u32(menu + 0xe30);
                let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
                start = array_element(e, locations, 0) as i32;
                if start > previous {
                    start = start.wrapping_sub(1);
                }
            } else {
                more = false;
            }
        }
    }
    e.set(this, CaravanMenu::eGameState, 0x12);
}

/// While nothing was activated and `iSelectedColumn` is below 4: one column
/// up, its sequence 0 activated from the selected row.
fn idle_next_column_up(e: &mut Engine, menu: u32, count: &mut i32) {
    while *count == 0 && e.mem.i32(menu + 0xe38) < 4 {
        let column = e.mem.i32(menu + 0xe38);
        e.mem.set_u32(menu + 0xe38, column.wrapping_add(1) as u32);
        let first = e.mem.u32(menu + 0xe34);
        *count += idle_activate_column(e, menu, first, 0);
    }
}

/// State 18: the moved rows come back, column by column from the bottom
/// (sequence 0; nothing to show skips the animation). When done the
/// track's value is updated and the state is 0x13 with `CAF_JOKER`, else
/// 0x15. The caravan flags are updated each frame.
fn idle_restore_row(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    if e.mem.u32(track_animation(menu, track, row, 0, 0)) == 0 {
        return;
    }
    let mut count = 0;
    let mut skip = false;
    if idle_time_is_zero(e) {
        e.mem.set_u32(menu + 0xe38, u32::MAX);
        idle_next_column_up(e, menu, &mut count);
        if count == 0 {
            skip = true;
        }
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 1);
    }
    if !skip {
        let first = e.mem.u32(menu + 0xe34);
        idle_update_track_rows(e, menu, first, ebp - 0x480);
    }
    let done = skip || {
        let track = e.mem.u32(menu + 0xe30);
        let row = e.mem.u32(menu + 0xe34);
        let column = e.mem.u32(menu + 0xe38);
        let sequence = e.mem.u32(track_animation(menu, track, row, column, 0));
        idle_sequence_passed(e, sequence)
    };
    if done {
        if !skip {
            let first = e.mem.u32(menu + 0xe34);
            idle_deactivate_column(e, menu, first, 0);
            idle_next_column_up(e, menu, &mut count);
        }
        if count != 0 {
            reset_idle_time(e);
        } else {
            let track = e.mem.u32(menu + 0xe30);
            caravan_menu_update_track_value(e, this, track);
            if caravan_menu_has_flag(e, this, CAF_JOKER) {
                e.set(this, CaravanMenu::eGameState, 0x13);
            } else {
                e.set(this, CaravanMenu::eGameState, 0x15);
            }
        }
    }
    caravan_menu_update_caravan_flags(e, this);
}

/// State 19: the next track with joker locations (from `iSelectedTrack`)
/// becomes the selection (its first location the row) and state 0x11;
/// without one state 0x15 and `CAF_JOKER` cleared.
fn idle_next_joker(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let menu = this.addr();
    caravan_menu_set_flag(e, this, CAF_JOKER, 1);
    while e.mem.i32(menu + 0xe30) < 6 {
        let track = e.mem.u32(menu + 0xe30);
        let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
        if array_size(e, locations) != 0 {
            break;
        }
        let track = e.mem.u32(menu + 0xe30);
        e.mem.set_u32(menu + 0xe30, track.wrapping_add(1));
    }
    if e.mem.i32(menu + 0xe30) >= 6 {
        e.set(this, CaravanMenu::eGameState, 0x15);
        caravan_menu_set_flag(e, this, CAF_JOKER, 0);
    } else {
        let track = e.mem.u32(menu + 0xe30);
        let locations = menu.wrapping_add(track << 4) + JOKER_LOCATIONS;
        let row = array_element(e, locations, 0);
        e.mem.set_u32(menu + 0xe34, row);
        e.set(this, CaravanMenu::eGameState, 0x11);
    }
}

/// The pending key `DoKey` left (`PENDING_KEY`) as a step: up and down
/// set y to -1 and 1, right and left x to 1 and -1 (and, with `floats`,
/// the float steps to the same; -1.0 from `01012054`), clearing the key.
fn idle_pending_key(e: &mut Engine, menu: u32, input: &mut IdleInput, floats: bool) {
    let key = e.mem.u32(menu + PENDING_KEY);
    if key == KEY_UP {
        e.mem.set_u32(menu + PENDING_KEY, 0);
        input.y = -1;
        if floats {
            input.y_bits = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
        }
    } else if key == KEY_DOWN {
        e.mem.set_u32(menu + PENDING_KEY, 0);
        input.y = 1;
        if floats {
            input.y_bits = 1.0f32.to_bits();
        }
    } else if key == KEY_RIGHT {
        e.mem.set_u32(menu + PENDING_KEY, 0);
        input.x = 1;
        if floats {
            input.x_bits = 1.0f32.to_bits();
        }
    } else if key == KEY_LEFT {
        e.mem.set_u32(menu + PENDING_KEY, 0);
        input.x = -1;
        if floats {
            input.x_bits = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
        }
    }
}

/// State 0 (`DoIdle`'s input step) of the active menu, then
/// `UpdateCaravanFlags` and `UpdateTileAlpha(-1)`.
fn idle_input(e: &mut Engine, this: Ptr<CaravanMenu>, input: &mut IdleInput) {
    match e.get(this, CaravanMenu::eActiveMenu) {
        1 => idle_deck_input(e, this, input),
        2 => idle_game_input(e, this, input),
        3 => {
            let owner = e.global::<u32>(CONTROLS_OWNER);
            let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
            let mut pressed = e
                .call(CONTROLS_BUTTON_00A23A50, &args![controls, 0u32, 1u32])
                .u32()
                != 0;
            if !pressed {
                let owner = e.global::<u32>(CONTROLS_OWNER);
                let controls = e.call(CONTROLS_INSTANCE, &args![owner]).u32();
                pressed = e
                    .call(CONTROLS_BUTTON_00A23A50, &args![controls, 1u32, 1u32])
                    .u32()
                    != 0;
            }
            if pressed {
                e.vcall(this.addr(), 0x38, &args![9u32, 0u32]);
            }
        }
        _ => {}
    }
    idle_refresh(e, this);
}

/// The deck menu's input: the stick's x moves the deck meter by its sign
/// (states 5 and 6 with one step); with the controls not read
/// (`004b71d0`), a release of the meter (`pim`'s float +0x48 back to 0.0
/// after a press, `iPrevValue` the value at the press) moves by the
/// difference (states 5 or 6 for 1 to 4 steps, 8 or 9 for more), and a
/// press records the value and `00747d00`'s position. In state 0 the
/// pending key or stick steps the meter by x every 0.25 s.
fn idle_deck_input(e: &mut Engine, this: Ptr<CaravanMenu>, input: &mut IdleInput) {
    let menu = this.addr();
    if input.x != 0 {
        let value = idle_meter_value(e, this);
        let magnitude = e
            .call(
                FLOAT_MAGNITUDE_00408840,
                &args![x87_quiet_bits(input.x_bits)],
            )
            .f64();
        let sum = x87_add(x87_div(x87_m32(input.x_bits), magnitude), value);
        let index = e.call(FTOL, &args![sum]).i32();
        if index >= 0 && (index as u32) < e.get(this, CaravanMenu::iTotalCards) {
            e.set(this, CaravanMenu::iNumSteps, 1);
            idle_set_meter(e, this, index);
            let next = if x87_m32(input.x_bits) > e.global::<f64>(ZERO_DOUBLE) {
                5
            } else {
                6
            };
            e.set(this, CaravanMenu::eGameState, next);
        }
    }
    if !e.call(KEYS_BLOCKED_004B71D0, &args![]).bool() {
        let interface = e.global::<u32>(CARAVAN_INTERFACE);
        let level = e.call(INTERFACE_FLOAT_00639AA0, &args![interface]).f64();
        if level == e.global::<f64>(ZERO_DOUBLE)
            && e.mem.u8(menu + BYTE_E85) != 0
            && e.get(this, CaravanMenu::eActiveMenu) == 1
        {
            e.set(this, CaravanMenu::iNumSteps, 0);
            e.mem.set_u8(menu + BYTE_E85, 0);
            let value = idle_meter_value(e, this);
            let current = e.call(FTOL, &args![value]).i32();
            let moved = current.wrapping_sub(e.get(this, CaravanMenu::iPrevValue));
            // `XOR EDX,EDX; JZ`: the compiler's jump past this block is
            // always taken, so the block never runs.
            let folded = 0u32;
            if moved == 0 && folded != 0 {
                idle_meter_drag(e, this);
            } else if moved <= -1 {
                if moved > -5 {
                    e.set(this, CaravanMenu::iNumSteps, moved.wrapping_neg());
                    e.set(this, CaravanMenu::eGameState, 6);
                } else {
                    e.set(this, CaravanMenu::eGameState, 9);
                }
            } else if moved >= 1 {
                if moved < 5 {
                    e.set(this, CaravanMenu::iNumSteps, moved);
                    e.set(this, CaravanMenu::eGameState, 5);
                } else {
                    e.set(this, CaravanMenu::eGameState, 8);
                }
            }
            e.set(this, CaravanMenu::iPrevValue, current);
        } else if e.mem.u8(menu + BYTE_E85) == 0 {
            let interface = e.global::<u32>(CARAVAN_INTERFACE);
            let level = e.call(INTERFACE_FLOAT_00639AA0, &args![interface]).f64();
            if level > e.global::<f64>(ZERO_DOUBLE) {
                let value = idle_meter_value(e, this);
                let current = e.call(FTOL, &args![value]).i32();
                e.set(this, CaravanMenu::iPrevValue, current);
                e.mem.set_u8(menu + BYTE_E85, 1);
                let interface = e.global::<u32>(CARAVAN_INTERFACE);
                let position = fn_00747d00(e, Ptr::new(interface));
                e.set(this, CaravanMenu::iPrevXLocation, position);
            }
        }
    }
    if e.get(this, CaravanMenu::eGameState) != 0 {
        return;
    }
    idle_pending_key(e, menu, input, false);
    if input.x == 0 && input.y == 0 {
        return;
    }
    let tick = f64::from(e.get(this, CaravanMenu::uiPrevIdleTick));
    let since = x87_sub(tick, x87_m32(e.mem.u32(menu + SELECTION_DELAY)));
    if since <= e.global::<f64>(SELECTION_REPEAT) || since.is_nan() {
        return;
    }
    let tick = f64::from(e.get(this, CaravanMenu::uiPrevIdleTick));
    e.mem.set_u32(menu + SELECTION_DELAY, x87_store32(tick));
    if e.get(this, CaravanMenu::eActiveMenu) != 1 || input.x == 0 {
        return;
    }
    let value = idle_meter_value(e, this);
    let index = e
        .call(FTOL, &args![x87_add(value, f64::from(input.x))])
        .i32();
    if index >= 0 && (index as u32) < e.get(this, CaravanMenu::iTotalCards) {
        idle_set_meter(e, this, index);
        let next = i32::from(input.x <= 0) + 5;
        e.set(this, CaravanMenu::eGameState, next);
        e.set(this, CaravanMenu::iNumSteps, 1);
    }
}

/// The block of the deck meter's release that the compiler made
/// unreachable (`00747335` to `00747497`): the step is the screen width
/// over `iTotalCards` (truncated); the drag since the press
/// (`00747d00` less `iPrevXLocation`, the first call's result unused)
/// over the negated step moves the meter, clamped to 0 and `iTotalCards`
/// (states 5 or 6 with the steps for under 5, 8 or 9 otherwise).
fn idle_meter_drag(e: &mut Engine, this: Ptr<CaravanMenu>) {
    let width = e.call(SCREEN_WIDTH, &args![]).f64();
    let total = f64::from(e.get(this, CaravanMenu::iTotalCards));
    let step = e.call(FTOL, &args![x87_div(width, total)]).i32();
    let interface = e.global::<u32>(CARAVAN_INTERFACE);
    fn_00747d00(e, Ptr::new(interface));
    let interface = e.global::<u32>(CARAVAN_INTERFACE);
    let position = fn_00747d00(e, Ptr::new(interface));
    let mut moved = position.wrapping_sub(e.get(this, CaravanMenu::iPrevXLocation));
    moved = moved.wrapping_div(step.wrapping_neg());
    if moved != 0 {
        let total = e.get(this, CaravanMenu::iTotalCards);
        let target = e.get(this, CaravanMenu::iPrevValue).wrapping_add(moved);
        let raised = e.call(MAX_OF, &args![target, 0u32]).i32();
        let clamped = e.call(MIN_OF, &args![raised, total]).i32();
        idle_set_meter(e, this, clamped);
        if e.call(INTEGER_MAGNITUDE, &args![moved]).i32() < 5 {
            let steps = e.call(INTEGER_MAGNITUDE, &args![moved]).i32();
            e.set(this, CaravanMenu::iNumSteps, steps);
            e.set(this, CaravanMenu::eGameState, i32::from(moved <= 0) + 5);
        } else {
            e.set(this, CaravanMenu::eGameState, i32::from(moved <= 0) + 8);
        }
    }
}

/// The step of a stick float: 0.0 within 0.01 (`01031148`), else the
/// float over its magnitude; truncated.
fn idle_stick_step(e: &mut Engine, bits: u32) -> i32 {
    let magnitude = e
        .call(FLOAT_MAGNITUDE_00408840, &args![x87_quiet_bits(bits)])
        .f64();
    let step = if magnitude < e.global::<f64>(STICK_THRESHOLD) {
        0.0
    } else {
        let magnitude = e
            .call(FLOAT_MAGNITUDE_00408840, &args![x87_quiet_bits(bits)])
            .f64();
        x87_div(x87_m32(bits), magnitude)
    };
    e.call(FTOL, &args![step]).i32()
}

/// The game menu's input: in state 0 the pending key gives the steps; any
/// step past the selection delay (0.25 s of the idle timer) moves, without
/// `CAF_SELECT_TRACK`, the hand card selection by x (wrapping), with the
/// switch sound, `SelectDeckCard(0)` and `CAF_DISABLE_A` from the card's
/// word +0xb4 over 10 under `CAF_TRACK_SETUP`; with it and without
/// `CAF_TRACK_SETUP`, the whole track selection by the stick's x (clamped
/// to 0..2, with the toggle sound) under `CAF_DESTROY_TRACK`, else
/// `HandleTrackSelection`.
fn idle_game_input(e: &mut Engine, this: Ptr<CaravanMenu>, input: &mut IdleInput) {
    let menu = this.addr();
    if e.get(this, CaravanMenu::eGameState) == 0 {
        idle_pending_key(e, menu, input, true);
    }
    if input.x == 0 && input.y == 0 {
        return;
    }
    let since = x87_sub(idle_time(e), x87_m32(e.mem.u32(menu + SELECTION_DELAY)));
    if since <= e.global::<f64>(SELECTION_REPEAT) || since.is_nan() {
        return;
    }
    e.mem.set_u32(menu + SELECTION_DELAY, idle_time_bits(e));
    if !caravan_menu_has_flag(e, this, CAF_SELECT_TRACK) {
        let magnitude = e
            .call(
                FLOAT_MAGNITUDE_00408840,
                &args![x87_quiet_bits(input.x_bits)],
            )
            .f64();
        let selected = e.mem.i32(menu + 0xe2c);
        let step = x87_add(
            x87_div(x87_m32(input.x_bits), magnitude),
            f64::from(selected),
        );
        let selected = e.call(FTOL, &args![step]).i32();
        e.mem.set_u32(menu + 0xe2c, selected as u32);
        let size = array_size(e, menu + PLAYER_HAND) as i32;
        if e.mem.i32(menu + 0xe2c) > size.wrapping_sub(1) {
            e.mem.set_u32(menu + 0xe2c, 0);
        } else if e.mem.i32(menu + 0xe2c) < 0 {
            e.mem.set_u32(menu + 0xe2c, size.wrapping_sub(1) as u32);
        }
        play_sound(e, SWITCH_CARD_SOUND);
        caravan_menu_select_deck_card(e, this, 0);
        let high = caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) && {
            let selected = e.mem.u32(menu + 0xe2c);
            card_b4(e, menu + PLAYER_HAND, selected) as i32 > 10
        };
        caravan_menu_set_flag(e, this, CAF_DISABLE_A, u8::from(high));
        caravan_menu_update_tile_alpha(e, this, -1);
        return;
    }
    if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
        return;
    }
    let x_step = idle_stick_step(e, input.x_bits);
    let y_step = idle_stick_step(e, input.y_bits);
    if caravan_menu_has_flag(e, this, CAF_DESTROY_TRACK) {
        let track = e.mem.u32(menu + 0xe30);
        caravan_menu_deselect_whole_track(e, this, track);
        let track = e.mem.i32(menu + 0xe30).wrapping_add(x_step);
        e.mem.set_u32(menu + 0xe30, track as u32);
        let track = e.mem.i32(menu + 0xe30);
        let clamped = clamp_calls(e, 0, track, 2);
        e.mem.set_u32(menu + 0xe30, clamped as u32);
        play_sound(e, TOGGLE_TRACK_SOUND);
        let track = e.mem.u32(menu + 0xe30);
        caravan_menu_select_whole_track(e, this, track);
    } else {
        caravan_menu_handle_track_selection(e, this, x_step, y_step);
    }
}

/// State 0x15: the turn change. `CAF_AI` is set; a finished game moves to
/// state 0x16. After the opponent's turn (the byte +0xe86 clear) the
/// player's last hand card is selected (track and selected card and object
/// cleared), the byte toggled and state 0 with the controls enabled and
/// `CAF_AI` cleared. Otherwise the byte is cleared and the opponent's move
/// (`0074fdc0` into the stack package at `EBP-0x31c`: count +8, hand card
/// +0x14, track +0xc, row +0x10) is made: while setting up the tracks
/// (`CAF_TRACK_SETUP`) a discard (count 6: the hand card swapped with the
/// next deck card, a new random next card, state 0xc with the byte set) or
/// the deal onto the highest free opponent track (5 down to 3, state 0xb);
/// otherwise a discard (6: state 0xe, the hand card replaced by the next
/// deck card when there is one), a track discard (5: state 0xf, the
/// track's direction cleared) or a placement (state 0xd; without a next
/// deck card the opponent's deck objects are hidden first). With the debug
/// byte `011d9184` the package is printed.
fn idle_next_turn(e: &mut Engine, this: Ptr<CaravanMenu>, ebp: u32) {
    let menu = this.addr();
    caravan_menu_set_flag(e, this, CAF_AI, 1);
    if caravan_menu_is_game_over(e, this) {
        e.set(this, CaravanMenu::eGameState, 0x16);
        return;
    }
    if e.mem.u8(menu + BYTE_E86) == 0 {
        let size = array_size(e, menu + PLAYER_HAND);
        e.mem.set_u32(menu + 0xe2c, size.wrapping_sub(1));
        e.mem.set_u32(menu + 0xe30, 0);
        e.mem.set_u32(menu + SELECTED_CARD, 0);
        e.mem.set_u32(menu + SELECTED_OBJECT, 0);
        caravan_menu_select_deck_card(e, this, 0);
        let toggled = u8::from(e.mem.u8(menu + BYTE_E86) == 0);
        e.mem.set_u8(menu + BYTE_E86, toggled);
        e.set(this, CaravanMenu::eGameState, 0);
        caravan_menu_set_flag(e, this, CAF_DISABLE_ALLCONTROLS, 0);
        idle_refresh(e, this);
        caravan_menu_set_flag(e, this, CAF_AI, 0);
        return;
    }
    e.mem.set_u8(menu + BYTE_E86, 0);
    let package = ebp - 0x31c;
    fn_0074fdc0(e, this, package);
    if caravan_menu_has_flag(e, this, CAF_TRACK_SETUP) {
        if array_size(e, package) == 6 {
            let card = e.call(WORD_AT_0X14, &args![package]).u32();
            e.mem.set_u32(menu + 0xe2c, card);
            let selected = e.mem.u32(menu + 0xe2c);
            let chosen = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(menu + SELECTED_CARD, chosen);
            let next = e.mem.u32(menu + NEXT_NPC_CARD);
            let drawn = array_element(e, menu + NPC_DECK, next);
            e.mem.set_u32(ebp - 0x324, drawn);
            let selected = e.mem.u32(menu + 0xe2c);
            let discarded = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(ebp - 0x320, discarded);
            let selected = e.mem.u32(menu + 0xe2c);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
            e.call(ARRAY_APPEND, &args![menu + NPC_HAND, ebp - 0x324]);
            let next = e.mem.u32(menu + NEXT_NPC_CARD);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_DECK, next, 1u32]);
            let remaining = array_size(e, menu + NPC_DECK);
            let next = e.call(RANDOM_RANGE, &args![0u32, remaining]).u32();
            e.mem.set_u32(menu + NEXT_NPC_CARD, next);
            fn_0074d190(e, this, 0);
            e.mem.set_u8(menu + BYTE_E86, 1);
            e.set(this, CaravanMenu::eGameState, 0xc);
        } else {
            let mut track = 5i32;
            while track > 2 {
                if array_size(e, card_track(menu, track as u32, 0)) == 0 {
                    break;
                }
                track -= 1;
            }
            e.mem.set_u32(menu + 0xe30, track as u32);
            let card = e.call(WORD_AT_0X14, &args![package]).u32();
            e.mem.set_u32(menu + 0xe2c, card);
            e.mem.set_u32(menu + 0xe34, 0);
            let row = e.mem.u32(menu + 0xe34);
            let count = array_size(e, card_track(menu, track as u32, row));
            e.mem.set_u32(menu + 0xe38, count);
            let selected = e.mem.u32(menu + 0xe2c);
            let chosen = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(menu + SELECTED_CARD, chosen);
            e.call(SET_WORD_AT_0XC, &args![package, track as u32]);
            e.call(SET_WORD_AT_0X10, &args![package, 0u32]);
            if e.global::<u8>(CREATE_BYTE) != 0 {
                caravan_ai_package_print_what_is_he_thinking(e, Ptr::new(package), menu + NPC_HAND);
            }
            let selected = e.mem.u32(menu + 0xe2c);
            let placed = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(ebp - 0x328, placed);
            let selected = e.mem.u32(menu + 0xe2c);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
            let row = e.mem.u32(menu + 0xe34);
            e.call(
                ARRAY_APPEND,
                &args![card_track(menu, track as u32, row), ebp - 0x328],
            );
            e.set(this, CaravanMenu::eGameState, 0xb);
        }
        return;
    }
    if array_size(e, package) == 6 {
        if e.mem.u32(menu + NEXT_NPC_CARD) != u32::MAX {
            let card = e.call(WORD_AT_0X14, &args![package]).u32();
            e.mem.set_u32(menu + 0xe2c, card);
            let next = e.mem.u32(menu + NEXT_NPC_CARD);
            let drawn = array_element(e, menu + NPC_DECK, next);
            e.mem.set_u32(ebp - 0x334, drawn);
            let selected = e.mem.u32(menu + 0xe2c);
            let discarded = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(ebp - 0x330, discarded);
            let selected = e.mem.u32(menu + 0xe2c);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
            e.call(ARRAY_APPEND, &args![menu + NPC_HAND, ebp - 0x334]);
            let next = e.mem.u32(menu + NEXT_NPC_CARD);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_DECK, next, 1u32]);
            idle_draw_next_npc_card(e, menu);
            fn_0074d190(e, this, 0);
        } else {
            let card = e.call(WORD_AT_0X14, &args![package]).u32();
            e.mem.set_u32(menu + 0xe2c, card);
            let selected = e.mem.u32(menu + 0xe2c);
            let discarded = array_element(e, menu + NPC_HAND, selected);
            e.mem.set_u32(ebp - 0x338, discarded);
            let selected = e.mem.u32(menu + 0xe2c);
            e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
            fn_0074d190(e, this, 0);
        }
        e.set(this, CaravanMenu::eGameState, 0xe);
        return;
    }
    if array_size(e, package) == 5 {
        let track = e.call(WORD_AT_0XC, &args![package]).u32();
        e.mem.set_u32(menu + 0xe30, track);
        let track = e.mem.u32(menu + 0xe30);
        e.mem.set_u32(track_info(menu, track) + 8, 0);
        e.set(this, CaravanMenu::eGameState, 0xf);
        return;
    }
    if e.mem.u32(menu + NEXT_NPC_CARD) != u32::MAX {
        idle_select_package_target(e, this, package);
        let next = e.mem.u32(menu + NEXT_NPC_CARD);
        let drawn = array_element(e, menu + NPC_DECK, next);
        e.mem.set_u32(ebp - 0x340, drawn);
        let selected = e.mem.u32(menu + 0xe2c);
        let placed = array_element(e, menu + NPC_HAND, selected);
        e.mem.set_u32(ebp - 0x33c, placed);
        let selected = e.mem.u32(menu + 0xe2c);
        e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
        e.call(ARRAY_APPEND, &args![menu + NPC_HAND, ebp - 0x340]);
        if e.global::<u8>(CREATE_BYTE) != 0 {
            caravan_ai_package_print_what_is_he_thinking(e, Ptr::new(package), menu + NPC_HAND);
        }
        let next = e.mem.u32(menu + NEXT_NPC_CARD);
        e.call(ARRAY_REMOVE_AT, &args![menu + NPC_DECK, next, 1u32]);
        idle_draw_next_npc_card(e, menu);
        let track = e.mem.u32(menu + 0xe30);
        let row = e.mem.u32(menu + 0xe34);
        e.call(
            ARRAY_APPEND,
            &args![card_track(menu, track, row), ebp - 0x33c],
        );
        fn_0074d190(e, this, 0);
    } else {
        let blocks = [ebp - 0x354, ebp - 0x360, ebp - 0x36c, ebp - 0x378];
        for (name, block) in NPC_DECK_OBJECTS.into_iter().zip(blocks) {
            let root = e.mem.u32(menu + HAND_GEOMETRY + 4);
            let object = e.call(OBJECT_BY_NAME, &args![root, name]).u32();
            if object != 0 {
                e.call(SET_CULLED, &args![object, 1u32]);
                e.call(UPDATE_DATA_INIT, &args![block, 0u32, 0u32, 0u32]);
                idle_update(e, object, block);
            }
        }
        idle_select_package_target(e, this, package);
        let selected = e.mem.u32(menu + 0xe2c);
        let placed = array_element(e, menu + NPC_HAND, selected);
        e.mem.set_u32(ebp - 0x348, placed);
        let selected = e.mem.u32(menu + 0xe2c);
        e.call(ARRAY_REMOVE_AT, &args![menu + NPC_HAND, selected, 1u32]);
        if e.global::<u8>(CREATE_BYTE) != 0 {
            caravan_ai_package_print_what_is_he_thinking(e, Ptr::new(package), menu + NPC_HAND);
        }
        let track = e.mem.u32(menu + 0xe30);
        let row = e.mem.u32(menu + 0xe34);
        e.call(
            ARRAY_APPEND,
            &args![card_track(menu, track, row), ebp - 0x348],
        );
    }
    e.set(this, CaravanMenu::eGameState, 0xd);
}

/// `iNextNPCCard`: -1 for an empty opponent deck, else a random index
/// (`00944460(0, size)`).
fn idle_draw_next_npc_card(e: &mut Engine, menu: u32) {
    if array_size(e, menu + NPC_DECK) == 0 {
        e.mem.set_u32(menu + NEXT_NPC_CARD, u32::MAX);
    } else {
        let remaining = array_size(e, menu + NPC_DECK);
        let next = e.call(RANDOM_RANGE, &args![0u32, remaining]).u32();
        e.mem.set_u32(menu + NEXT_NPC_CARD, next);
    }
}

/// The package's placement as the selection: track (+0xc), row (+0x10),
/// column the row's card count, hand card (+0x14) and `pSelectedCard`.
fn idle_select_package_target(e: &mut Engine, this: Ptr<CaravanMenu>, package: u32) {
    let menu = this.addr();
    let track = e.call(WORD_AT_0XC, &args![package]).u32();
    e.mem.set_u32(menu + 0xe30, track);
    let row = e.call(WORD_AT_0X10, &args![package]).u32();
    e.mem.set_u32(menu + 0xe34, row);
    let track = e.mem.u32(menu + 0xe30);
    let row = e.mem.u32(menu + 0xe34);
    let count = array_size(e, card_track(menu, track, row));
    e.mem.set_u32(menu + 0xe38, count);
    let card = e.call(WORD_AT_0X14, &args![package]).u32();
    e.mem.set_u32(menu + 0xe2c, card);
    let selected = e.mem.u32(menu + 0xe2c);
    let chosen = array_element(e, menu + NPC_HAND, selected);
    e.mem.set_u32(menu + SELECTED_CARD, chosen);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_30a0, fn_007530a0(Ptr)),
        entry!(0x0075_30f0, fn_007530f0(Ptr)),
        entry!(0x0075_3210, fn_00753210(Ptr)),
        entry!(0x0073_cb80, fn_0073cb80(Ptr) -> u8),
        entry!(0x0074_08d0, fn_007408d0(Ptr, i32) -> bool),
        entry!(0x0074_0940, fn_00740940(Ptr) -> u32),
        entry!(
            0x0073_b730,
            caravan_ai_package_print_what_is_he_thinking(Ptr, u32)
        ),
        entry!(
            0x0073_bab0,
            caravan_menu_set_flag(Ptr<CaravanMenu>, u32, u8)
        ),
        entry!(0x0073_bb10, caravan_menu_clear_flags(Ptr<CaravanMenu>)),
        entry!(0x0073_bb30, caravan_menu_get_class(Ptr<CaravanMenu>) -> u32),
        entry!(
            0x0073_bb40,
            caravan_menu_attach_tile_by_id(Ptr<CaravanMenu>, i32, u32)
        ),
        entry!(
            0x0073_bb70,
            caravan_menu_scalar_deleting_destructor(Ptr<CaravanMenu>, u32) -> Ptr<CaravanMenu>
        ),
        entry!(0x0073_ca50, caravan_menu_close()),
        entry!(0x0073_cba0, fn_0073cba0(Ptr) -> u32),
        entry!(0x0073_cbc0, fn_0073cbc0(Ptr) -> u32),
        entry!(0x0073_cbe0, fn_0073cbe0() -> u32),
        entry!(
            0x0073_cbf0,
            caravan_menu_prepare_shared_3d_elements(Ptr<CaravanMenu>)
        ),
        entry!(
            0x0073_ced0,
            caravan_menu_prepare_caravan_data(Ptr<CaravanMenu>) -> bool
        ),
        entry!(
            0x0073_d020,
            caravan_menu_prepare_ante_menu(Ptr<CaravanMenu>)
        ),
        entry!(
            0x0073_d650,
            caravan_menu_prepare_betting_currency(Ptr<CaravanMenu>)
        ),
        entry!(0x0074_0900, fn_00740900(Ptr) -> u32),
        entry!(0x0074_0920, fn_00740920(Ptr) -> u32),
        entry!(0x0074_0960, fn_00740960(Ptr) -> u32),
        entry!(
            0x0074_0980,
            caravan_menu_prepare_results_menu(Ptr<CaravanMenu>)
        ),
        entry!(0x0074_0e50, fn_00740e50(Ptr) -> u32),
        entry!(0x0074_0e70, fn_00740e70(Ptr, u32)),
        entry!(0x0074_0e90, fn_00740e90(Ptr, u32)),
        entry!(0x0074_0eb0, fn_00740eb0(Ptr, u32)),
        entry!(0x0074_0ed0, fn_00740ed0(Ptr, u32)),
        entry!(0x0074_0ef0, fn_00740ef0(Ptr, u32)),
        entry!(
            0x0074_0f10,
            caravan_menu_has_flag(Ptr<CaravanMenu>, u32) -> bool
        ),
        entry!(0x0074_0f30, caravan_menu_render_scene()),
        entry!(0x0074_1060, caravan_menu_create(u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x0074_14c0,
            caravan_menu_has_all_tiles(Ptr<CaravanMenu>) -> bool
        ),
        entry!(0x0074_7d00, fn_00747d00(Ptr) -> i32),
        entry!(
            0x0074_90e0,
            caravan_menu_do_gamepad_up_event(Ptr<CaravanMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0074_92f0,
            caravan_menu_do_down_click(Ptr<CaravanMenu>, i32, u32)
        ),
        entry!(
            0x0074_9360,
            caravan_menu_do_key(Ptr<CaravanMenu>, i32) -> bool
        ),
        entry!(
            0x0074_9560,
            caravan_menu_do_click(Ptr<CaravanMenu>, i32, u32)
        ),
        entry!(
            0x0074_9740,
            caravan_menu_update_caravan_flags(Ptr<CaravanMenu>)
        ),
        entry!(
            0x0074_9a40,
            caravan_menu_update_tile_alpha(Ptr<CaravanMenu>, i32)
        ),
        entry!(0x0074_a000, fn_0074a000(Ptr<CaravanMenu>, u32, u32)),
        entry!(0x0074_a090, caravan_menu_item_select_callback(i32)),
        entry!(0x0074_a270, caravan_menu_cancel_confirm_func()),
        entry!(0x0074_df90, fn_0074df90(Ptr<CaravanMenu>)),
        entry!(0x0074_e220, fn_0074e220(Ptr<CaravanMenu>, u32, u32, u32)),
        entry!(
            0x0074_e480,
            caravan_menu_select_deck_card(Ptr<CaravanMenu>, u8)
        ),
        entry!(
            0x0074_e6d0,
            caravan_menu_select_track_card(Ptr<CaravanMenu>, i32, i32)
        ),
        entry!(
            0x0074_ecb0,
            caravan_menu_select_whole_track(Ptr<CaravanMenu>, u32)
        ),
        entry!(
            0x0074_ed40,
            caravan_menu_deselect_whole_track(Ptr<CaravanMenu>, u32)
        ),
        entry!(
            0x0074_edd0,
            caravan_menu_deselect_track_card(Ptr<CaravanMenu>)
        ),
        entry!(
            0x0074_ee70,
            caravan_menu_update_track_value(Ptr<CaravanMenu>, u32)
        ),
        entry!(
            0x0074_f360,
            caravan_menu_handle_track_selection(Ptr<CaravanMenu>, i32, i32)
        ),
        entry!(0x0074_f600, fn_0074f600(Ptr<CaravanMenu>, u32, u32) -> i32),
        entry!(
            0x0074_f6c0,
            caravan_menu_is_valid_card_placement(Ptr<CaravanMenu>, i32, i32, i32, u8) -> bool
        ),
        entry!(
            0x0074_f9b0,
            caravan_menu_load_new_bill(Ptr<CaravanMenu>, i32)
        ),
        entry!(0x0074_fc00, fn_0074fc00(Ptr<CaravanMenu>, i32)),
        entry!(0x0075_2cb0, fn_00752cb0(Ptr<CaravanMenu>, u32, i32) -> i32),
        entry!(0x0075_2d40, fn_00752d40(Ptr<CaravanMenu>, u32, u32) -> u32),
        entry!(0x0075_2df0, fn_00752df0(Ptr<CaravanMenu>, i32) -> i32),
        entry!(
            0x0075_2e20,
            caravan_menu_is_game_over(Ptr<CaravanMenu>) -> bool
        ),
        entry!(0x0075_3070, fn_00753070(Ptr) -> Ptr),
        entry!(0x0075_30c0, fn_007530c0(Ptr) -> Ptr),
        entry!(0x0075_3110, fn_00753110(Ptr) -> Ptr),
        entry!(0x0075_3140, fn_00753140(Ptr)),
        entry!(0x0075_3180, fn_00753180(Ptr, u32) -> Ptr),
        entry!(0x0075_31b0, fn_007531b0(Ptr, u32) -> Ptr),
        entry!(0x0075_31e0, fn_007531e0(Ptr, u32) -> Ptr),
        entry!(
            0x0073_b7e0,
            caravan_menu_constructor(Ptr<CaravanMenu>) -> Ptr<CaravanMenu>
        ),
        entry!(0x0073_bba0, caravan_menu_destructor(Ptr<CaravanMenu>)),
        entry!(0x0074_af70, fn_0074af70(Ptr<CaravanMenu>, u32)),
        entry!(0x0074_cc70, fn_0074cc70(Ptr<CaravanMenu>, u8)),
        entry!(0x0074_d190, fn_0074d190(Ptr<CaravanMenu>, u8)),
        entry!(
            0x0074_d730,
            caravan_menu_update_game_hand_textures(Ptr<CaravanMenu>, u8, u32)
        ),
        entry!(
            0x0074_a2c0,
            caravan_menu_update_betting_ui(Ptr<CaravanMenu>, i32, i32)
        ),
        entry!(
            0x0073_d850,
            caravan_menu_prepare_deck_menu(Ptr<CaravanMenu>)
        ),
        entry!(
            0x0074_b700,
            caravan_menu_swap_deck_building_textures(Ptr<CaravanMenu>, i32, u8)
        ),
        entry!(
            0x0074_7d30,
            caravan_menu_do_gamepad(Ptr<CaravanMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0073_ea90,
            caravan_menu_prepare_game_menu(Ptr<CaravanMenu>)
        ),
        entry!(0x0074_fdc0, fn_0074fdc0(Ptr<CaravanMenu>, u32) -> u32),
        entry!(0x0074_1500, caravan_menu_do_idle(Ptr<CaravanMenu>)),
    ]
}

// Translated from 0073cb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the byte at `this + 0x98` (in `AL`).
pub fn fn_0073cb80(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x98)
}

// Translated from 007408d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: when `value` is 2 or 0 stores it at `this + 0x24` and returns
/// `true`; any other value is refused (`false`, nothing stored). `RET 4`.
pub fn fn_007408d0(e: &mut Engine, this: Ptr, value: i32) -> bool {
    if value == 2 || value == 0 {
        e.mem.set_i32(this.addr() + 0x24, value);
        true
    } else {
        false
    }
}

// Translated from 00740940 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: returns the word at `this + 0x624`.
pub fn fn_00740940(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x624)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0073cb80_reads_the_byte_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x98, 0xaabb_cc07);
        assert_eq!(e.call(0x0073_cb80, &args![this]).u32(), 7);
    }

    #[test]
    fn fn_007408d0_accepts_only_two_and_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        for (value, accepted) in [(2i32, true), (0, true), (1, false), (-1, false), (3, false)] {
            e.mem.set_u32(this + 0x24, 0x55);
            let result = e.call(0x0074_08d0, &args![this, value]).bool();
            assert_eq!(result, accepted, "value {value}");
            let stored = if accepted { value as u32 } else { 0x55 };
            assert_eq!(e.mem.u32(this + 0x24), stored);
        }
    }

    #[test]
    fn fn_00740940_reads_the_word_at_0x624() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x628);
        e.mem.set_u32(this + 0x624, 0x1234_5678);
        assert_eq!(e.call(0x0074_0940, &args![this]).u32(), 0x1234_5678);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007530a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_30a0, ARRAY_VTABLE_01071AEC, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_007530f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_30f0, ARRAY_VTABLE_01071B00, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_00753210_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0075_3210, ARRAY_VTABLE_01071B14, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod n052_tests {
    use super::*;

    /// Every callee outside this file the n052 translations call, doubled
    /// with a function that answers 0 (some get working doubles below).
    const STUBS: &[u32] = &[
        ACTIVATE_SEQUENCE,
        ACTOR_GOLD,
        ADD_ALL_LIGHTS,
        ARRAY_APPEND,
        ARRAY_DESTROY_ELEMENTS,
        ARRAY_LAST_SLOT,
        ARRAY_RELEASE_008454F0,
        ARRAY_REMOVE,
        ARRAY_REMOVE_AT,
        ARRAY_SORT,
        AUDIO_INSTANCE,
        BACK_TEXTURE_NAME_ADDRESS,
        BOUND_RADIUS,
        CARD_LIST_ADD_005AE3D0,
        CARD_LIST_CLEAR_00470470,
        CARD_LIST_COUNT,
        CARD_SET_BYTE_98,
        CHILD_COUNT,
        CLAMPED_ACTOR_VALUE,
        CLEAR_BARTER_CONTAINER,
        CLEAR_MENU_BUTTON,
        CONTROLS_CLEAR_USER_ACTIONS,
        CONTROLS_INSTANCE,
        COPY_BOUNDED,
        CREATE_TEXTURE_IMAGE,
        DEACTIVATE_ALL,
        DYNAMIC_CAST,
        ENTRY_DESTROY,
        ENTRY_FILL,
        ENTRY_INIT,
        ENTRY_SET_ACTOR,
        ERROR_PRINT,
        FILL_BARTER_CONTAINER,
        FIXED_STRING_RELEASE,
        FORM_BY_EDITOR_ID,
        FORM_BY_ID,
        FORM_TYPE,
        FULL_NAME,
        GET_CONTROLLER,
        GET_FLOAT_TRAIT,
        GET_FORM_VALUE,
        GET_GOLD_AMOUNT,
        GET_MESSAGE_MENU_RESULT,
        GET_PROPERTY,
        GET_SEQUENCE,
        GET_WORLD_BOUND,
        INTERFACE_SINGLETON,
        INT_ARRAY_CONSTRUCT,
        INT_ARRAY_DESTRUCT,
        IS_TOP_MENU_ID,
        ITEM_CHANGE_CONSTRUCT,
        ITEM_CHANGE_DESTRUCT,
        KEYS_BLOCKED_004B71D0,
        LIST_CONSTRUCT,
        LIST_DELETE,
        LIST_HEAD_DATA,
        LIST_NEXT_NODE,
        LIST_NODE_DATA,
        LIST_REMOVE_HEAD,
        MAKE_CAMERA,
        MATRIX_MULTIPLY,
        MATRIX_Z_ROTATION,
        MENU_CONSTRUCTOR,
        MENU_DESTRUCTOR,
        MENU_PREP_FOR_VISIBILITY,
        MENU_SET_MENU_TILE,
        MISC_STAT_INCREMENT,
        MODEL_LOAD_FILE,
        MODEL_RELEASE,
        NI_ALLOCATE,
        NODE_CONSTRUCT,
        OBJECT_BY_NAME,
        OBJECT_CLONE,
        OBJECT_UPDATE,
        OPERATOR_DELETE,
        OPERATOR_NEW,
        POINTER_ASSIGN,
        POINTER_INIT,
        POINTER_RELEASE,
        PRINT_BOUNDED,
        PRINT_MESSAGE,
        QUANTITY_MENU_CREATE,
        RANDOM_RANGE,
        REMOVE_BARTER_ITEM,
        ROOT_RELEASE_00572160,
        SCENE_NODE,
        SCENE_REFRESH,
        SCREEN_WIDTH,
        SETTING_STRING,
        SETTING_STRING_004C69F0,
        SET_CULLED,
        SET_FLOAT_TRAIT,
        SET_LOCAL_ROTATE,
        SET_LOCAL_TRANSLATE,
        SET_LOCAL_TRANSLATE_VECTOR,
        SET_STRING_TRAIT,
        SET_TRAIT_FROM_INT,
        SET_TRAIT_FROM_UNSIGNED,
        SHADER_VALUE_00526AE0,
        SHOW_CORNER_MESSAGE,
        SHOW_MESSAGE,
        SIMPLE_ARRAY_INITIALIZE,
        SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_RELEASE,
        SOUND_PLAY,
        START_ANIMATIONS,
        STRING_LENGTH,
        SWAP_TEXTURE_PATH,
        TEXTURE_NAME_ADDRESS,
        TEXTURE_NAME_TEXT,
        TILE_ADD_NEEDS_UPDATE,
        TILE_ADD_USER_TRAIT,
        TILE_CHILD_NAMED,
        TILE_GET_CHILD_BY_NAME,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        TILE_POKE,
        TILE_READER,
        TILE_READ_FILE,
        TILE_TEXT_TO_TRAIT,
        TUTORIAL_MARK,
        TUTORIAL_MENU_CREATE,
        TUTORIAL_SHOWN,
        VECTOR_CONSTRUCT,
        VECTOR_DESTRUCT,
        WORD_AT_0X1C,
        WORD_AT_0X20,
        WORD_AT_0X24,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn word_f64(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
    }

    /// An engine with the globals' pages mapped, every outside callee
    /// doubled, and working doubles for the small accessors.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_8000,
            0x0101_e000,
            0x0101_f000,
            0x0102_3000,
            0x0102_4000,
            0x0102_c000,
            0x0104_f000,
            0x0106_c000,
            0x0107_0000,
            0x0107_1000,
            0x011c_3000,
            0x011d_9000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        for &address in STUBS {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_SLOT, |e, a| {
            ret(e.mem.u32(a[0] + 4).wrapping_add(a[1].wrapping_mul(4)))
        });
        e.register(IDENTITY, |_, a| ret(a[0]));
        e.register(WORD_AT_FOUR, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(CARD_WORD_B4, |e, a| ret(e.mem.u32(a[0] + 0xb4)));
        e.register(CARD_WORD_B8, |e, a| ret(e.mem.u32(a[0] + 0xb8)));
        e.register(CARD_VALUE_ADDRESS, |_, a| ret(a[0] + 0xb4));
        e.register(LOCAL_TRANSLATE, |_, a| ret(a[0] + 0x58));
        e.register(LOCAL_ROTATE, |_, a| ret(a[0] + 0x34));
        e.register(MIN_OF, |_, a| ret((a[0] as i32).min(a[1] as i32) as u32));
        e.register(MAX_OF, |_, a| ret((a[0] as i32).max(a[1] as i32) as u32));
        e.register(FTOL, |_, a| ret(word_f64(a) as i32 as u32));
        e.register(UPDATE_DATA_INIT, |_, a| ret(a[0]));
        e.register(FIXED_STRING_INIT, |_, a| ret(a[0]));
        e.register(LIST_HEAD_DATA, |e, a| ret(e.mem.u32(a[0]) + 8));
        e.register(LIST_NODE_DATA, |_, a| ret(a[1] + 8));
        e.register(LIST_NEXT_NODE, |e, a| ret(e.mem.u32(a[1])));
        e
    }

    /// A menu as the singleton with its 0x33 tiles (0x10-byte blocks whose
    /// child list has an empty head), and a player.
    fn setup(e: &mut Engine) -> (Ptr<CaravanMenu>, u32) {
        let this = e.new_object::<CaravanMenu>();
        e.set_global(CARAVAN_MENU, this.addr());
        let tiles = e.mem.alloc(0x340);
        for index in 0..0x33 {
            let tile = tiles + 0x10 * index;
            e.mem.set_u32(this.addr() + TILES + 4 * index, tile);
            // A child list whose head node holds no child.
            let head = e.mem.alloc(0xc);
            e.mem.set_u32(tile + 4, head);
        }
        let player = e.mem.alloc(0x700);
        e.set_global(PLAYER, player);
        (this, player)
    }

    /// A `BSSimpleArray` at `array` holding `items`.
    fn fill_array(e: &mut Engine, array: u32, items: &[u32]) {
        let buffer = e.mem.alloc(4 * items.len() as u32 + 4);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *item);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, items.len() as u32);
    }

    /// A card with words +0xb4 (value) and +0xb8 (suit), and byte +0x98.
    fn card(e: &mut Engine, value: u32, suit: u32, in_deck: u8) -> u32 {
        let card = e.mem.alloc(0xc0);
        e.mem.set_u32(card + 0xb4, value);
        e.mem.set_u32(card + 0xb8, suit);
        e.mem.set_u8(card + 0x98, in_deck);
        card
    }

    /// Tile `index` of a menu.
    fn tile_of(e: &Engine, this: Ptr<CaravanMenu>, index: u32) -> u32 {
        e.mem.u32(this.addr() + TILES + 4 * index)
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn count(e: &Engine, callee: u32) -> usize {
        calls_to(e, callee).len()
    }

    /// An object with a vtable whose slots are `0x00d0_0000 + slot`.
    fn object_with_vtable(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x400);
        for slot in 0..0x100 {
            e.mem.set_u32(vtable + 4 * slot, 0x00d0_0000 + 4 * slot);
            e.register(0x00d0_0000 + 4 * slot, |_, _| Ret::default());
        }
        e.mem.set_u32(object, vtable);
        object
    }

    const ONE: u32 = 0x3f80_0000;

    #[test]
    fn print_what_is_he_thinking_prints_with_and_without_the_card() {
        let mut e = engine();
        let package = e.mem.alloc(0x1c);
        for (index, value) in [10u32, 11, 12, 13, 14, 1, 16].iter().enumerate() {
            e.mem.set_u32(package + 4 * index as u32, *value);
        }
        e.call_log = Some(vec![]);
        e.call(0x0073_b730, &args![package, 0u32]);
        assert_eq!(
            calls_to(&e, ERROR_PRINT),
            vec![vec![THINKING_FORMAT, 13, 14, 1, 10, 11, 16]]
        );
        let card = object_with_vtable(&mut e);
        e.register(0x00d0_0130, |_, _| ret(0x4444));
        let hand = e.mem.alloc(0x10);
        fill_array(&mut e, hand, &[0, card]);
        e.call_log = Some(vec![]);
        e.call(0x0073_b730, &args![package, hand]);
        assert_eq!(
            calls_to(&e, ERROR_PRINT),
            vec![vec![
                THINKING_WITH_CARD_FORMAT,
                13,
                14,
                1,
                10,
                11,
                16,
                0x4444
            ]]
        );
    }

    #[test]
    fn flags_set_clear_test_and_tiles() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        e.call(0x0073_bab0, &args![this, 0x11u32, 1u8]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), 0x11);
        assert!(e.call(0x0074_0f10, &args![this, 0x10u32]).bool());
        assert_eq!(e.call(0x0074_0f10, &args![this, 0x10u32]).u32(), 1);
        e.call(0x0073_bab0, &args![this, 0x1u32, 0u8]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), 0x10);
        assert!(!e.call(0x0074_0f10, &args![this, 0x1u32]).bool());
        e.call(0x0073_bb10, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), 0);
        assert_eq!(e.call(0x0073_bb30, &args![this]).u32(), 0x43b);
        assert_eq!(e.call(0x0073_cbe0, &args![]).u32(), 0x43b);
        e.call(0x0073_bb40, &args![this, 0x32i32, 0x99u32]);
        e.call(0x0073_bb40, &args![this, 0x33i32, 0x98u32]);
        e.call(0x0073_bb40, &args![this, -1i32, 0x97u32]);
        assert_eq!(e.mem.u32(this.addr() + TILES + 0x32 * 4), 0x99);
        assert_eq!(e.mem.u32(this.addr() + TILES + 0x33 * 4), 0);
        assert!(e.call(0x0074_14c0, &args![this]).bool());
        e.mem.set_u32(this.addr() + TILES + 0x20 * 4, 0);
        assert!(!e.call(0x0074_14c0, &args![this]).bool());
    }

    #[test]
    fn player_caravan_words_get_and_set() {
        let mut e = engine();
        let object = e.mem.alloc(0x700);
        for (setter, offset) in [
            (0x0074_0e70u32, 0x61cu32),
            (0x0074_0e90, 0x620),
            (0x0074_0eb0, 0x624),
            (0x0074_0ed0, 0x628),
            (0x0074_0ef0, 0x62c),
        ] {
            e.call(setter, &args![object, offset + 1]);
            assert_eq!(e.mem.u32(object + offset), offset + 1);
        }
        e.mem.set_u32(object + 0x614, 0x55);
        e.mem.set_u32(object + 0x618, 0x66);
        assert_eq!(e.call(0x0073_cba0, &args![object]).u32(), 0x55);
        assert_eq!(e.call(0x0073_cbc0, &args![object]).u32(), 0x66);
        assert_eq!(e.call(0x0074_0900, &args![object]).u32(), 0x61d);
        assert_eq!(e.call(0x0074_0920, &args![object]).u32(), 0x621);
        assert_eq!(e.call(0x0074_0960, &args![object]).u32(), 0x629);
        assert_eq!(e.call(0x0074_0e50, &args![object]).u32(), 0x62d);
    }

    #[test]
    fn scalar_deleting_destructors_free_on_bit_0() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        for (address, vtable) in [
            (0x0075_3180u32, ARRAY_VTABLE_01071AEC),
            (0x0075_31b0, ARRAY_VTABLE_01071B00),
            (0x0075_31e0, ARRAY_VTABLE_01071B14),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![array, 1u32]).u32(), array);
            assert_eq!(e.mem.u32(array), vtable);
            assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![array, 1]]);
            assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![array]]);
            e.call_log = Some(vec![]);
            e.call(address, &args![array, 2u32]);
            assert_eq!(count(&e, OPERATOR_DELETE), 0);
        }
    }

    #[test]
    fn array_constructors_store_the_vtable_and_set_up() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        for (address, vtable) in [
            (0x0075_3070u32, ARRAY_VTABLE_01071AEC),
            (0x0075_30c0, ARRAY_VTABLE_01071B00),
            (0x0075_3110, ARRAY_VTABLE_01071B14),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![array]).u32(), array);
            assert_eq!(e.mem.u32(array), vtable);
            assert_eq!(
                calls_to(&e, SIMPLE_ARRAY_INITIALIZE),
                vec![vec![array, 0, 0]]
            );
        }
    }

    #[test]
    fn remove_last_lowers_the_size() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        fill_array(&mut e, array, &[1, 2, 3]);
        let buffer = e.mem.u32(array + 4);
        e.call_log = Some(vec![]);
        e.call(0x0075_3140, &args![array]);
        assert_eq!(e.mem.u32(array + 8), 2);
        assert_eq!(
            calls_to(&e, ARRAY_DESTROY_ELEMENTS),
            vec![vec![array, buffer + 8, 1]]
        );
        e.mem.set_u32(array + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0075_3140, &args![array]);
        assert_eq!(count(&e, ARRAY_DESTROY_ELEMENTS), 0);
        assert_eq!(e.mem.u32(array + 8), 0);
    }

    #[test]
    fn close_puts_the_deck_cards_back_into_the_lists() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        e.mem.set_u32(player + 0x614, 0x111);
        e.mem.set_u32(player + 0x618, 0x222);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| ret(1));
        e.set(this, CaravanMenu::eActiveMenu, 1);
        let in_deck = card(&mut e, 3, 1, 1);
        let out = card(&mut e, 4, 1, 0);
        fill_array(&mut e, this.addr() + DECK_ARRAY, &[in_deck, out]);
        let buffer = e.mem.u32(this.addr() + DECK_ARRAY + 4);
        e.call_log = Some(vec![]);
        e.call(0x0073_ca50, &args![]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x17);
        assert_eq!(calls_to(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x43b]]);
        assert_eq!(
            calls_to(&e, CARD_LIST_CLEAR_00470470),
            vec![vec![0x222], vec![0x111]]
        );
        assert_eq!(
            calls_to(&e, CARD_LIST_ADD_005AE3D0),
            vec![vec![0x222, buffer], vec![0x111, buffer + 4]]
        );
        // Another menu active: only the state.
        e.set(this, CaravanMenu::eActiveMenu, 2);
        e.call_log = Some(vec![]);
        e.call(0x0073_ca50, &args![]);
        assert_eq!(count(&e, CARD_LIST_CLEAR_00470470), 0);
        // No menu: nothing at all.
        e.set_global(CARAVAN_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0073_ca50, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn half_screen_width_plus_offset_is_truncated() {
        let mut e = engine();
        let object = e.mem.alloc(0x30);
        e.mem.set_f32(object + 0x2c, 10.5);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.register(SCREEN_WIDTH, |_, _| ret_f(1280.0));
        assert_eq!(e.call(0x0074_7d00, &args![object]).i32(), 650);
    }

    #[test]
    fn do_down_click_starts_dragging_the_meter() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let interface = e.mem.alloc(0x30);
        e.mem.set_f32(interface + 0x2c, 4.0);
        e.set_global(CARAVAN_INTERFACE, interface);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(UT_CURRENT_VALUE, 0x77u32);
        e.register(SCREEN_WIDTH, |_, _| ret_f(100.0));
        e.register(GET_FLOAT_TRAIT, |_, _| ret_f(17.9));
        e.set(this, CaravanMenu::eActiveMenu, 1);
        e.mem.set_u8(this.addr() + BYTE_E85, 7);
        e.call_log = Some(vec![]);
        e.call(0x0074_92f0, &args![this, 0xfi32, 0u32]);
        assert_eq!(
            calls_to(&e, GET_FLOAT_TRAIT),
            vec![vec![tile_of(&e, this, 0xf), 0x77]]
        );
        assert_eq!(e.get(this, CaravanMenu::iPrevValue), 17);
        assert_eq!(e.mem.u8(this.addr() + BYTE_E85), 1);
        assert_eq!(e.get(this, CaravanMenu::iPrevXLocation), 54);
        // Another id: only the byte is cleared.
        e.call(0x0074_92f0, &args![this, 0xei32, 0u32]);
        assert_eq!(e.mem.u8(this.addr() + BYTE_E85), 0);
    }

    #[test]
    fn do_key_maps_keys_to_buttons() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let object = object_with_vtable(&mut e);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this.addr(), vtable);
        for (key, slot, button) in [
            (0x41i32, 0x38u32, 0xbu32),
            (0x61, 0x38, 0xb),
            (0x45, 0x38, 0xe),
            (0x66, 0x38, 0xc),
            (0x71, 0x38, 0xd),
            (0x52, 0x3c, 0xa),
            (0x73, 0x38, 0x10),
            (0x57, 0x38, 9),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x0074_9360, &args![this, key]);
            assert_eq!(
                calls_to(&e, 0x00d0_0000 + slot),
                vec![vec![this.addr(), button, 0]],
                "key {key:#x}"
            );
        }
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0074_9360, &args![this, 0x42i32]).bool());
        assert_eq!(count(&e, 0x00d0_0038), 0);
        e.call(0x0074_9360, &args![this, -0x7fff_fffei32]);
        assert_eq!(e.mem.i32(this.addr() + PENDING_KEY), -0x7fff_fffe);
        e.set(this, CaravanMenu::eGameState, 1);
        e.call(0x0074_9360, &args![this, -0x7fff_fffei32]);
        assert_eq!(e.mem.u32(this.addr() + PENDING_KEY), 0);
        // Keys blocked.
        e.register(KEYS_BLOCKED_004B71D0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0074_9360, &args![this, 0x41i32]).bool());
        assert_eq!(count(&e, 0x00d0_0038), 0);
        // Results menu on top: closes.
        e.register(KEYS_BLOCKED_004B71D0, |_, _| ret(0));
        e.register(IS_TOP_MENU_ID, |_, _| ret(1));
        e.register(0x00d0_0034, |_, _| ret(0x43b));
        e.set(this, CaravanMenu::eActiveMenu, 3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0074_9360, &args![this, 0x42i32]).bool());
        assert_eq!(calls_to(&e, IS_TOP_MENU_ID), vec![vec![0x43b]]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x17);
    }

    #[test]
    fn set_alpha_on_each_child_of_a_tile() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let parent = e.mem.alloc(0x10);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(parent + 4, node);
        e.mem.set_u32(this.addr() + TILES + 4 * 3, parent);
        e.register(LIST_HEAD_DATA, |e, a| ret(e.mem.u32(a[0]) + 8));
        e.register(LIST_NODE_DATA, |_, a| ret(a[1] + 8));
        e.register(LIST_NEXT_NODE, |e, a| ret(e.mem.u32(a[1])));
        e.mem.set_u32(node + 8, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0074_a000, &args![this, 3u32, 0x4300_0000u32]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x5555, 0xfa9, 0x4300_0000, 1]]
        );
        // An empty head: nothing.
        e.mem.set_u32(node + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_a000, &args![this, 3u32, 0x4300_0000u32]);
        assert_eq!(count(&e, SET_FLOAT_TRAIT), 0);
    }

    #[test]
    fn cancel_confirm_marks_for_close_or_enables() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        e.set(this, CaravanMenu::iFlags, CAF_DISABLE_ALLCONTROLS);
        e.set_global(CONTROLS_OWNER, 0x99u32);
        e.register(CONTROLS_INSTANCE, |_, _| ret(0x55));
        e.call_log = Some(vec![]);
        e.call(0x0074_a270, &args![]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_ALLCONTROLS | CAF_MARKFORCLOSE
        );
        assert_eq!(calls_to(&e, CONTROLS_INSTANCE), vec![vec![0x99]]);
        assert_eq!(calls_to(&e, CONTROLS_CLEAR_USER_ACTIONS), vec![vec![0x55]]);
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| ret(1));
        e.call(0x0074_a270, &args![]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_MARKFORCLOSE);
    }

    #[test]
    fn whole_track_and_cursor_moves() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        e.set_global(TRACK_RAISE, 3.0f32.to_bits());
        e.set_global(CURSOR_HIDDEN_HEIGHT, (-100.0f32).to_bits());
        for row in 0..7 {
            e.mem.set_u32(
                this.addr() + TRACK_GEOMETRY + 2 * 0x1c + 4 * row,
                0x900 + row,
            );
        }
        e.call_log = Some(vec![]);
        e.call(0x0074_ecb0, &args![this, 2u32]);
        let moves = calls_to(&e, SET_LOCAL_TRANSLATE);
        assert_eq!(moves.len(), 7);
        assert_eq!(moves[6], vec![0x906, 0, 0, 3.0f32.to_bits()]);
        assert_eq!(calls_to(&e, OBJECT_UPDATE).len(), 7);
        e.call_log = Some(vec![]);
        e.call(0x0074_ed40, &args![this, 2u32]);
        assert_eq!(calls_to(&e, SET_LOCAL_TRANSLATE)[0], vec![0x900, 0, 0, 0]);
        e.mem.set_u32(this.addr() + CURSORS, 0x31);
        e.mem.set_u32(this.addr() + CURSORS + 4, 0x32);
        e.call_log = Some(vec![]);
        e.call(0x0074_edd0, &args![this]);
        let height = (-100.0f32).to_bits();
        assert_eq!(
            calls_to(&e, SET_LOCAL_TRANSLATE),
            vec![vec![0x31, 0, 0, height], vec![0x32, 0, 0, height]]
        );
        assert_eq!(calls_to(&e, OBJECT_UPDATE).len(), 2);
    }

    #[test]
    fn track_row_values_and_counts() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let five = card(&mut e, 5, 2, 0);
        let king = card(&mut e, 0xe, 2, 0);
        let seven = card(&mut e, 7, 3, 0);
        fill_array(&mut e, card_track(menu, 1, 0), &[five, king, king]);
        fill_array(&mut e, card_track(menu, 1, 1), &[seven]);
        fill_array(&mut e, card_track(menu, 1, 2), &[five]);
        assert_eq!(e.call(0x0074_f600, &args![this, 1u32, 0u32]).i32(), 20);
        assert_eq!(e.call(0x0074_f600, &args![this, 1u32, 3u32]).i32(), 0);
        // Rows from the first with cards: 5, 7, 5.
        assert_eq!(e.call(0x0075_2cb0, &args![this, 1u32, 5i32]).i32(), 10);
        assert_eq!(e.call(0x0075_2d40, &args![this, 1u32, 2u32]).u32(), 10);
        assert_eq!(e.call(0x0075_2d40, &args![this, 1u32, 3u32]).u32(), 7);
        for (value, side) in [(20, -1), (21, 0), (26, 0), (27, 1)] {
            assert_eq!(e.call(0x0075_2df0, &args![this, value]).i32(), side);
        }
    }

    #[test]
    fn prepare_caravan_data_fills_and_sorts_the_deck() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        // Two lists of nodes (data word, next word).
        let first = e.mem.alloc(8);
        let first_two = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0xa1);
        e.mem.set_u32(first + 4, first_two);
        e.mem.set_u32(first_two, 0xa2);
        e.mem.set_u32(second, 0xb1);
        e.mem.set_u32(player + 0x614, first);
        e.mem.set_u32(player + 0x618, second);
        e.register_double(CARD_LIST_COUNT, move |_, a| {
            ret(if a[0] == second { 1 } else { 2 })
        });
        e.register(ARRAY_APPEND, |e, a| {
            let value = e.mem.u32(a[1]);
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 0xc + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0073_ced0, &args![this]).bool());
        assert_eq!(e.get(this, CaravanMenu::iCardsInDeck), 1);
        assert_eq!(e.get(this, CaravanMenu::iTotalCards), 3);
        assert_eq!(
            calls_to(&e, CARD_SET_BYTE_98),
            vec![vec![0xa1, 0], vec![0xa2, 0], vec![0xb1, 1]]
        );
        assert_eq!(e.mem.u32(this.addr() + DECK_ARRAY + 8), 3);
        assert_eq!(
            calls_to(&e, ARRAY_SORT),
            vec![vec![this.addr() + DECK_ARRAY, DECK_COMPARE]]
        );
        assert_eq!(e.get(this, CaravanMenu::iJumpAmount), 10);
        e.register(CARD_LIST_COUNT, |_, _| ret(70));
        e.call(0x0073_ced0, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iJumpAmount), 14);
    }

    #[test]
    fn prepare_betting_currency_counts_gold_and_money_items() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        e.mem.set_u32(this.addr() + CHALLENGER, 0x4000);
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        // The changes list: head word -> node (entry, next).
        let changes = e.mem.alloc(4);
        e.mem.set_u32(handler + 0x628, changes);
        let node = e.mem.alloc(8);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(changes, node);
        e.mem.set_u32(node, entry);
        e.mem.set_u32(entry + 4, 3);
        e.mem.set_u32(entry + 8, 0x600);
        e.register(GET_GOLD_AMOUNT, |_, _| ret(50));
        e.register(FORM_TYPE, |_, a| ret(if a[0] == 0x600 { 0x74 } else { 0 }));
        e.register(GET_FORM_VALUE, |_, _| ret(20));
        e.call_log = Some(vec![]);
        e.call(0x0073_d650, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iTotalFunds), 110);
        assert_eq!(e.mem.u32(this.addr() + NPC_TOTAL_FUNDS), 110);
        assert_eq!(
            calls_to(&e, FILL_BARTER_CONTAINER),
            vec![vec![handler, player], vec![handler, 0x4000]]
        );
        // Not money: only the gold.
        e.register(FORM_TYPE, |_, _| ret(0x73));
        e.call(0x0073_d650, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iTotalFunds), 50);
    }

    #[test]
    fn do_click_maps_buttons_when_idle() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let object = object_with_vtable(&mut e);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this.addr(), vtable);
        for (id, slot, button) in [
            (7i32, 0x38u32, 0xcu32),
            (10, 0x38, 0x10),
            (19, 0x38, 9),
            (36, 0x38, 0xb),
            (20, 0x38, 0xd),
            (21, 0x38, 0xe),
            (38, 0x3c, 0xa),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x0074_9560, &args![this, id, 0u32]);
            assert_eq!(
                calls_to(&e, 0x00d0_0000 + slot),
                vec![vec![this.addr(), button, 0]],
                "id {id}"
            );
        }
        e.call_log = Some(vec![]);
        e.call(0x0074_9560, &args![this, 8i32, 0u32]);
        e.set(this, CaravanMenu::eGameState, 1);
        e.call(0x0074_9560, &args![this, 7i32, 0u32]);
        e.set(this, CaravanMenu::eGameState, 0);
        e.set(this, CaravanMenu::iFlags, CAF_DISABLE_ALLCONTROLS);
        e.call(0x0074_9560, &args![this, 7i32, 0u32]);
        assert_eq!(count(&e, 0x00d0_0038), 0);
    }

    #[test]
    fn new_bill_and_coin_are_cloned_animated_and_attached() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        e.mem.set_u32(menu + BILL_GEOMETRY + 8, 0x620);
        e.mem.set_u32(menu + COIN_GEOMETRY + 4, 0x610);
        e.mem.set_u32(menu + LIST_OF_BILL_ANIMATIONS, 0x5b);
        e.mem.set_u32(menu + LIST_OF_COIN_ANIMATIONS, 0x5c);
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.register(OBJECT_CLONE, |_, a| ret(a[0] + 1));
        e.register(RANDOM_RANGE, |_, a| ret(a[1] - 1));
        e.register(GET_CONTROLLER, |_, _| ret(0x88));
        e.register(GET_SEQUENCE, |_, _| ret(0x99));
        e.register(ARRAY_APPEND, |e, a| ret(e.mem.u32(a[1])));
        e.register(CARD_LIST_ADD_005AE3D0, |e, a| ret(e.mem.u32(a[1])));
        e.call_log = Some(vec![]);
        e.call(0x0074_f9b0, &args![this, 5i32]);
        assert_eq!(calls_to(&e, MIN_OF), vec![vec![5, 2]]);
        assert_eq!(calls_to(&e, OBJECT_CLONE), vec![vec![0x620]]);
        assert_eq!(calls_to(&e, RANDOM_RANGE), vec![vec![0, 3]]);
        assert_eq!(calls_to(&e, FIXED_STRING_INIT)[0][1], ALT_PLACE);
        assert_eq!(
            calls_to(&e, ACTIVATE_SEQUENCE),
            vec![vec![0x88, 0x99, 0, 0, ONE, 0, 0]]
        );
        assert_eq!(calls_to(&e, START_ANIMATIONS), vec![vec![0x621]]);
        assert_eq!(calls_to(&e, ARRAY_APPEND)[0][0], menu + LIST_OF_BILLS);
        assert_eq!(calls_to(&e, CARD_LIST_ADD_005AE3D0)[0][0], 0x5b);
        assert_eq!(calls_to(&e, 0x00d0_00dc), vec![vec![root, 0x621, 1]]);
        // A coin without a controller manager: no animation.
        e.register(GET_CONTROLLER, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0074_fc00, &args![this, 1i32]);
        assert_eq!(calls_to(&e, OBJECT_CLONE), vec![vec![0x610]]);
        assert_eq!(calls_to(&e, RANDOM_RANGE), vec![vec![0, 2]]);
        assert_eq!(count(&e, ACTIVATE_SEQUENCE), 0);
        assert_eq!(calls_to(&e, CARD_LIST_ADD_005AE3D0)[0][0], 0x5c);
    }

    #[test]
    fn update_track_value_sums_rows_and_sets_direction_and_suit() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        e.set_global(UT_VALUE, 0x41u32);
        e.set_global(TRACK_LOW_ALPHA, 126.0f32.to_bits());
        e.set_global(TRACK_ALPHA, 255.0f32.to_bits());
        let four = card(&mut e, 4, 1, 0);
        let nine = card(&mut e, 9, 2, 0);
        let king = card(&mut e, 0xe, 3, 0);
        let queen = card(&mut e, 0xd, 4, 0);
        fill_array(&mut e, card_track(menu, 2, 0), &[four]);
        fill_array(&mut e, card_track(menu, 2, 1), &[nine, king, queen]);
        e.call_log = Some(vec![]);
        e.call(0x0074_ee70, &args![this, 2u32]);
        let info = track_info(menu, 2);
        assert_eq!(e.mem.u32(info + 8), 22);
        // Rising, then the queen reverses it and gives its suit.
        assert_eq!(e.mem.i32(info), -1);
        assert_eq!(e.mem.u32(info + 4), 4);
        let total_tile = tile_of(&e, this, 0x19);
        assert_eq!(calls_to(&e, TILE_POKE), vec![vec![total_tile, 0x41, ONE]]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![total_tile, 0x41, 22], vec![total_tile, 0xff4, 1]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![total_tile, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        // One row: its suit, direction 0; below 21 dims first.
        fill_array(&mut e, card_track(menu, 2, 1), &[]);
        e.call_log = Some(vec![]);
        e.call(0x0074_ee70, &args![this, 2u32]);
        assert_eq!(e.mem.u32(info + 8), 4);
        assert_eq!(e.mem.i32(info), 0);
        assert_eq!(e.mem.u32(info + 4), 1);
        assert_eq!(calls_to(&e, SET_FLOAT_TRAIT).len(), 2);
        // No rows: suit 5.
        fill_array(&mut e, card_track(menu, 2, 0), &[]);
        e.call(0x0074_ee70, &args![this, 2u32]);
        assert_eq!(e.mem.u32(info + 4), 5);
    }

    #[test]
    fn is_valid_card_placement_rules() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let five = card(&mut e, 5, 1, 0);
        let seven = card(&mut e, 7, 2, 0);
        let queen = card(&mut e, 0xd, 2, 0);
        let three = card(&mut e, 3, 2, 0);
        fill_array(&mut e, menu + PLAYER_HAND, &[five, seven, queen, three]);
        let valid = |e: &mut Engine, track: i32, row: i32, card: i32| {
            e.call(0x0074_f6c0, &args![this, track, row, card, 0u8])
                .bool()
        };
        assert!(!valid(&mut e, 6, 0, 0));
        assert!(!valid(&mut e, 0, 7, 0));
        assert!(!valid(&mut e, 0, 0, 4));
        // Setting up: a number card on an empty row only.
        e.set(this, CaravanMenu::iFlags, CAF_TRACK_SETUP);
        assert!(valid(&mut e, 0, 0, 0));
        assert!(!valid(&mut e, 0, 0, 2));
        e.set(this, CaravanMenu::iFlags, 0);
        // Number cards: own side, after a different value, keeping the direction.
        assert!(valid(&mut e, 0, 0, 0));
        assert!(!valid(&mut e, 3, 0, 0));
        assert!(!valid(&mut e, 0, 1, 0));
        fill_array(&mut e, card_track(menu, 0, 0), &[five]);
        assert!(!valid(&mut e, 0, 0, 1));
        assert!(!valid(&mut e, 0, 1, 0));
        let info = track_info(menu, 0);
        e.mem.set_i32(info, 1);
        e.mem.set_u32(info + 4, 1);
        assert!(valid(&mut e, 0, 1, 1));
        assert!(!valid(&mut e, 0, 1, 3));
        e.mem.set_u32(info + 4, 2);
        assert!(valid(&mut e, 0, 1, 3));
        e.mem.set_i32(info, -1);
        e.mem.set_u32(info + 4, 1);
        assert!(!valid(&mut e, 0, 1, 1));
        // Face cards: on a row with 1 to 3 cards; a queen not before cards.
        assert!(valid(&mut e, 0, 0, 2));
        fill_array(&mut e, card_track(menu, 0, 1), &[seven]);
        assert!(!valid(&mut e, 0, 0, 2));
        assert!(!valid(&mut e, 0, 2, 2));
        // The opponent's hand: its tracks only.
        fill_array(&mut e, menu + NPC_HAND, &[five]);
        assert!(!e
            .call(0x0074_f6c0, &args![this, 1i32, 3i32, 0i32, 1u8])
            .bool());
        assert!(e
            .call(0x0074_f6c0, &args![this, 4i32, 0i32, 0i32, 1u8])
            .bool());
    }

    #[test]
    fn handle_track_selection_moves_and_wraps() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, menu + PLAYER_HAND, &[five]);
        fill_array(&mut e, card_track(menu, 2, 0), &[five, five]);
        e.call_log = Some(vec![]);
        e.call(0x0074_f360, &args![this, 3i32, 0i32]);
        assert_eq!(e.get(this, CaravanMenu::iSelectedTrack), 2);
        assert_eq!(e.get(this, CaravanMenu::iSelectedColumn), 2);
        assert_eq!(calls_to(&e, MAX_OF), vec![vec![0, 3]]);
        assert_eq!(calls_to(&e, MIN_OF), vec![vec![3, 2]]);
        // A row step past the top crosses to the facing track.
        e.call(0x0074_f360, &args![this, 0i32, -1i32]);
        assert_eq!(e.get(this, CaravanMenu::iSelectedTrack), 3);
        assert_eq!(e.get(this, CaravanMenu::iSelectedRow), 0);
        // On the opponent's side the row step counts down.
        e.call(0x0074_f360, &args![this, 0i32, -1i32]);
        assert_eq!(e.get(this, CaravanMenu::iSelectedRow), 1);
        e.set(this, CaravanMenu::iSelectedRow, 6);
        e.call(0x0074_f360, &args![this, 0i32, -1i32]);
        assert_eq!(e.get(this, CaravanMenu::iSelectedRow), 6);
        // Both steps 0 reset the selection; an invalid place disables A.
        e.call(0x0074_f360, &args![this, 0i32, 0i32]);
        assert_eq!(e.get(this, CaravanMenu::iSelectedTrack), 0);
        assert_eq!(e.get(this, CaravanMenu::iSelectedRow), 0);
        assert_eq!(e.get(this, CaravanMenu::iFlags) & CAF_DISABLE_A, 0);
        e.call(0x0074_f360, &args![this, 2i32, 0i32]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags) & CAF_DISABLE_A,
            CAF_DISABLE_A
        );
    }

    #[test]
    fn is_game_over_by_tracks_and_by_empty_hands() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let ten = card(&mut e, 10, 1, 0);
        let seven = card(&mut e, 7, 1, 0);
        let four = card(&mut e, 4, 1, 0);
        for track in 0..3 {
            fill_array(&mut e, card_track(menu, track, 0), &[ten]);
            fill_array(&mut e, card_track(menu, track, 1), &[seven]);
            fill_array(&mut e, card_track(menu, track, 2), &[four]);
        }
        fill_array(&mut e, menu + PLAYER_HAND, &[ten]);
        fill_array(&mut e, menu + NPC_HAND, &[ten]);
        assert!(e.call(0x0075_2e20, &args![this]).bool());
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_PLAYERWIN);
        // The opponent sells all three.
        for track in 0..3 {
            fill_array(&mut e, card_track(menu, track, 2), &[]);
            fill_array(&mut e, card_track(menu, 5 - track, 0), &[ten]);
            fill_array(&mut e, card_track(menu, 5 - track, 1), &[seven]);
            fill_array(&mut e, card_track(menu, 5 - track, 2), &[four]);
        }
        assert!(e.call(0x0075_2e20, &args![this]).bool());
        assert_eq!(e.get(this, CaravanMenu::iFlags), 0);
        // Nobody sold: over only when a side is out of cards.
        for track in 3..6 {
            fill_array(&mut e, card_track(menu, track, 2), &[]);
        }
        assert!(!e.call(0x0075_2e20, &args![this]).bool());
        fill_array(&mut e, menu + NPC_HAND, &[]);
        assert!(e.call(0x0075_2e20, &args![this]).bool());
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_PLAYERWIN);
    }

    #[test]
    fn select_deck_card_lowers_the_old_and_raises_the_new() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        e.map(0x0101_2000, 0x1000);
        let old = e.mem.alloc(0x70);
        let shadow = e.mem.alloc(0x70);
        e.mem.set_f32(old + 0x58 + 4, 6.0);
        e.mem.set_f32(old + 0x58 + 8, 1.0);
        e.mem.set_u32(menu + SELECTED_OBJECT, old);
        e.mem.set_u32(menu + SELECTED_SHADOW, shadow);
        e.mem.set_u32(menu + HAND_GEOMETRY, 0x190);
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, menu + PLAYER_HAND, &[0, five]);
        e.set(this, CaravanMenu::iSelectedDeckCard, 1);
        let new = e.mem.alloc(0x70);
        e.mem.set_f32(new + 0x58 + 4, 2.0);
        e.register_double(OBJECT_BY_NAME, move |_, _| ret(new));
        e.call_log = Some(vec![]);
        e.call(0x0074_e480, &args![this, 0u8]);
        let six = 6.0f32.to_bits();
        let two = 2.0f32.to_bits();
        assert_eq!(
            calls_to(&e, SET_LOCAL_TRANSLATE),
            vec![
                vec![old, 0, six, 0],
                vec![shadow, 0, six, 0],
                vec![new, 0, two, ONE],
                vec![new, 0, two, ONE]
            ]
        );
        let names = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&names[0][1..], &[0x1e, HAND_CARD_FORMAT, 2, 1]);
        assert_eq!(&names[1][1..], &[0x1e, HAND_SHADOW_FORMAT, 2]);
        assert_eq!(e.mem.u32(menu + SELECTED_CARD), five);
        // Deselect only, with the card not raised: nothing moves.
        e.mem.set_f32(new + 0x58 + 8, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x0074_e480, &args![this, 1u8]);
        assert_eq!(count(&e, SET_LOCAL_TRANSLATE), 0);
    }

    #[test]
    fn item_select_callback_raises_the_opponent() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        let menu = this.addr();
        e.set_global(UT_VALUE, 0x41u32);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set(this, CaravanMenu::iPlayerAnte, 10);
        e.set(this, CaravanMenu::iNPCAnte, 5);
        e.mem.set_i32(menu + NPC_TOTAL_FUNDS, 200);
        e.mem.set_f32(menu + BET_PERCENTAGE, 0.25);
        e.register(CLAMPED_ACTOR_VALUE, |_, _| ret_f(50.0));
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.call_log = Some(vec![]);
        e.call(0x0074_a090, &args![30i32]);
        assert_eq!(e.get(this, CaravanMenu::iPlayerAnte), 40);
        // Limit 50 + 50 * 0.5 = 75; the opponent raises to the player's 40.
        assert_eq!(e.get(this, CaravanMenu::iNPCAnte), 40);
        assert_eq!(
            calls_to(&e, CLAMPED_ACTOR_VALUE),
            vec![vec![player + 0xa4, 0x20]]
        );
        let npc_tile = tile_of(&e, this, 4);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).contains(&vec![npc_tile, 0x41, 40]));
        // The pile: limit 75, change 30 - 10 + max(40 - 40, 0) = 20.
        assert_eq!(calls_to(&e, RANDOM_RANGE)[0], vec![0, 9]);
    }

    #[test]
    fn update_caravan_flags_by_menu() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        // Ante: no funds disables A and X; a bet below the opponent's ante
        // and the funds leaves A.
        e.set(this, CaravanMenu::iNPCAnte, 20);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_Y | CAF_DISABLE_A | CAF_DISABLE_X
        );
        e.set(this, CaravanMenu::iTotalFunds, 100);
        e.set(this, CaravanMenu::iPlayerAnte, 10);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), 0);
        e.set(this, CaravanMenu::iPlayerAnte, 20);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_DISABLE_A);
        e.set(this, CaravanMenu::iPlayerAnte, 100);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_A | CAF_DISABLE_X
        );
        // Deck: the meter's card in the deck disables A; few cards dim tile 8.
        e.set(this, CaravanMenu::iFlags, 0);
        e.set(this, CaravanMenu::eActiveMenu, 1);
        e.set_global(DIM_ALPHA, 128.0f32.to_bits());
        let in_deck = card(&mut e, 3, 1, 1);
        fill_array(&mut e, menu + DECK_ARRAY, &[in_deck]);
        e.set(this, CaravanMenu::iCardsInDeck, 5);
        e.call_log = Some(vec![]);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_A | CAF_DISABLE_Y | CAF_DISABLE_LB
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![tile_of(&e, this, 8), 0xfa9, 128.0f32.to_bits(), 1]]
        );
        e.set(this, CaravanMenu::iCardsInDeck, 0);
        e.mem.set_u8(in_deck + 0x98, 0);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_X | CAF_DISABLE_Y | CAF_DISABLE_LB
        );
        // Game: empty decks and tracks.
        e.set(this, CaravanMenu::iFlags, 0);
        e.set(this, CaravanMenu::eActiveMenu, 2);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(e.mem.i32(menu + NEXT_PLAYER_CARD), -1);
        assert_eq!(e.mem.i32(menu + NEXT_NPC_CARD), -1);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_DISABLE_LT | CAF_DISABLE_RT
        );
        fill_array(&mut e, card_track(menu, 1, 0), &[in_deck]);
        e.call(0x0074_9740, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_DISABLE_LT);
    }

    #[test]
    fn update_tile_alpha_dims_disabled_buttons() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        e.set_global(DIM_ALPHA, 128.0f32.to_bits());
        e.set_global(TRACK_ALPHA, 255.0f32.to_bits());
        // Give tile 12 one child.
        let parent = tile_of(&e, this, 12);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(parent + 4, node);
        e.mem.set_u32(node + 8, 0xc12);
        e.register(LIST_HEAD_DATA, |e, a| ret(e.mem.u32(a[0]) + 8));
        e.register(LIST_NODE_DATA, |_, a| ret(a[1] + 8));
        e.register(LIST_NEXT_NODE, |e, a| ret(e.mem.u32(a[1])));
        // Tile 8 has a head child too (its list head is always read).
        let parent = tile_of(&e, this, 8);
        let head = e.mem.alloc(0xc);
        e.mem.set_u32(parent + 4, head);
        e.mem.set_u32(head + 8, 0x888);
        e.set(this, CaravanMenu::eActiveMenu, 1);
        e.set(this, CaravanMenu::iFlags, CAF_DISABLE_A);
        e.call_log = Some(vec![]);
        e.call(0x0074_9a40, &args![this, -1i32]);
        let dim = 128.0f32.to_bits();
        let full = 255.0f32.to_bits();
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![
                vec![0x888, 0xfa9, full, 1],
                vec![0x888, 0xfa9, dim, 1],
                vec![0xc12, 0xfa9, dim, 1]
            ]
        );
        e.set(this, CaravanMenu::iFlags, 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_9a40, &args![this, 12i32]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0xc12, 0xfa9, 255.0f32.to_bits(), 1]]
        );
        // Tile 8 shown also sets its list head.s tile by the deck size.
        e.set(this, CaravanMenu::iCardsInDeck, 0x1e);
        e.call_log = Some(vec![]);
        e.call(0x0074_9a40, &args![this, 8i32]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x888, 0xfa9, full, 1], vec![0x888, 0xfa9, full, 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0074_9a40, &args![this, 3i32]);
        assert_eq!(count(&e, SET_FLOAT_TRAIT), 0);
    }

    #[test]
    fn do_gamepad_up_event_cancels_or_asks_to_forfeit() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let object = object_with_vtable(&mut e);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this.addr(), vtable);
        e.register(IS_TOP_MENU_ID, |_, _| ret(1));
        e.register(SETTING_STRING, |_, a| ret(a[0] + 1));
        assert!(!e.call(0x0074_90e0, &args![this, 9i32, 0u32]).bool());
        e.set(this, CaravanMenu::eActiveMenu, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0074_90e0, &args![this, 10i32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE),
            vec![vec![
                FORFEIT_MESSAGE_SETTING + 1,
                0,
                0,
                CANCEL_CONFIRM_CALLBACK,
                0,
                0x17,
                0,
                0,
                FORFEIT_YES_SETTING + 1,
                FORFEIT_NO_SETTING + 1,
                0
            ]]
        );
        assert_eq!(calls_to(&e, CLEAR_MENU_BUTTON), vec![vec![0xa]]);
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_DISABLE_ALLCONTROLS);
        assert!(!e.call(0x0074_90e0, &args![this, 10i32, 0u32]).bool());
        // Selecting a track: cancelled.
        e.set(
            this,
            CaravanMenu::iFlags,
            CAF_SELECT_TRACK | CAF_DESTROY_TRACK | CAF_DISABLE_A,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0074_90e0, &args![this, 10i32, 0u32]).bool());
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_DISABLE_X);
        assert_eq!(count(&e, SHOW_MESSAGE), 0);
        assert_eq!(count(&e, OBJECT_UPDATE), 7);
        // Deck menu: closes and clears the user actions.
        e.set(this, CaravanMenu::iFlags, 0);
        e.set(this, CaravanMenu::eActiveMenu, 1);
        e.call_log = Some(vec![]);
        e.call(0x0074_90e0, &args![this, 10i32, 0u32]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x17);
        assert_eq!(count(&e, CONTROLS_CLEAR_USER_ACTIONS), 1);
    }

    /// A node with a texture property of type `kind` whose slot +0xf0 is
    /// logged.
    fn textured_node(e: &mut Engine, kind: i32) -> (u32, u32) {
        let property = object_with_vtable(e);
        e.mem.set_i32(property + 0x1c, kind);
        e.register_double(GET_PROPERTY, move |_, _| ret(property));
        e.register(WORD_AT_0X1C, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(CREATE_TEXTURE_IMAGE, |e, a| {
            e.mem.set_u32(a[2], 0x7e7);
            Ret::default()
        });
        e.register(TEXTURE_NAME_ADDRESS, |_, a| ret(a[0] + 0x9c));
        e.register(BACK_TEXTURE_NAME_ADDRESS, |_, a| ret(a[0] + 0xa8));
        e.register(TEXTURE_NAME_TEXT, |_, a| ret(a[0] + 1));
        (0x0123, property)
    }

    #[test]
    fn track_card_textures_by_selection_and_by_slot() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        e.mem
            .set_u32(menu + TRACK_GEOMETRY + 4 * 0x1c + 4 * 2, 0x4442);
        let (_, property) = textured_node(&mut e, 9);
        e.register(OBJECT_BY_NAME, |_, _| ret(0x0123));
        e.set(this, CaravanMenu::iSelectedTrack, 4);
        e.set(this, CaravanMenu::iSelectedRow, 2);
        e.set(this, CaravanMenu::iSelectedColumn, 1);
        e.mem.set_u32(menu + SELECTED_CARD, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0074_df90, &args![this]);
        let names = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&names[0][1..], &[0x104, TRACK_CARD_FORMAT, 3, 1]);
        assert_eq!(
            &names[1][1..],
            &[0x104, PATH_FORMAT, TEXTURES_PREFIX, 0x509d]
        );
        assert_eq!(calls_to(&e, OBJECT_BY_NAME)[0][0], 0x4442);
        assert_eq!(calls_to(&e, 0x00d0_00f0), vec![vec![property, 0, 0, 0x7e7]]);
        assert_eq!(count(&e, POINTER_RELEASE), 1);
        // By slot, on a player track: number column + 1.
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, card_track(menu, 1, 0), &[0, five]);
        e.call_log = Some(vec![]);
        e.call(0x0074_e220, &args![this, 1u32, 0u32, 1u32]);
        let names = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&names[0][1..], &[0x104, TRACK_CARD_FORMAT, 2, 1]);
        assert_eq!(names[1][4], five + 0x9c + 1);
        // A property of another kind: no texture.
        e.mem.set_i32(property + 0x1c, 7);
        e.call_log = Some(vec![]);
        e.call(0x0074_e220, &args![this, 1u32, 0u32, 1u32]);
        assert_eq!(count(&e, CREATE_TEXTURE_IMAGE), 0);
        assert_eq!(count(&e, POINTER_RELEASE), 1);
    }

    #[test]
    fn prepare_shared_3d_elements_builds_the_table() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let root = object_with_vtable(&mut e);
        let scene = object_with_vtable(&mut e);
        e.set_global(MODEL_LOADER, 0x10ad_u32);
        e.set_global(LIGHT_RADIUS_SCALE, 2.5f64);
        e.register(MODEL_LOAD_FILE, |_, _| ret(0x7ab1));
        e.register(NI_ALLOCATE, |_, _| ret(0x5000));
        e.register(NODE_CONSTRUCT, |_, a| ret(a[0]));
        e.register_double(POINTER_ASSIGN, move |e, a| {
            let value = if a[0] == MONEY_ROOT && a[1] != 0 {
                root
            } else {
                a[1]
            };
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(GET_CONTROLLER, |_, _| ret(0x3a3));
        e.register(GET_SEQUENCE, |_, a| ret(e_seq(a[1])));
        e.register_double(SCENE_NODE, move |_, _| ret(scene));
        e.register(BOUND_RADIUS, |_, _| ret_f(10.0));
        e.register(MAKE_CAMERA, |_, _| ret(0xca));
        e.call_log = Some(vec![]);
        e.call(0x0073_cbf0, &args![this]);
        assert_eq!(
            calls_to(&e, MODEL_LOAD_FILE),
            vec![vec![0x10ad, TABLE_MODEL_PATH, 0, 1, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(menu + TABLE_GEOMETRY), 0x7ab1);
        assert_eq!(calls_to(&e, NODE_CONSTRUCT), vec![vec![0x5000, 0]]);
        assert_eq!(calls_to(&e, 0x00d0_00dc)[0], vec![root, 0x7ab1, 1]);
        assert_eq!(e.mem.u32(menu + CAMERA_ANIM_MANAGER), 0x3a3);
        assert_eq!(calls_to(&e, DEACTIVATE_ALL), vec![vec![0x3a3, 0]]);
        // object0 not found: a new camera.
        assert_eq!(e.mem.u32(CAMERA_POINTER), 0xca);
        assert_eq!(calls_to(&e, 0x00d0_00dc)[1], vec![scene, root, 1]);
        assert_eq!(
            calls_to(&e, ADD_ALL_LIGHTS),
            vec![vec![scene, root, 25.0f32.to_bits()]]
        );
    }

    /// The sequence a test's `GET_SEQUENCE` double answers for a name slot.
    fn e_seq(slot: u32) -> u32 {
        slot & 0xff
    }

    #[test]
    fn prepare_results_menu_records_a_win() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        let menu = this.addr();
        e.set_global(UT_VALUE, 0x41u32);
        e.mem.set_u32(menu + ANTE, 30);
        e.mem.set_u32(player + 0x61c, 100);
        e.mem.set_u32(player + 0x624, 2);
        e.mem.set_u32(player + 0x62c, 25);
        e.set(this, CaravanMenu::iFlags, CAF_PLAYERWIN);
        let challenger = object_with_vtable(&mut e);
        e.mem.set_u32(menu + CHALLENGER, challenger);
        let player_vtable = e.mem.u32(challenger);
        e.mem.set_u32(player, player_vtable);
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        let changes = e.mem.alloc(4);
        e.mem.set_u32(handler + 0x628, changes);
        e.call_log = Some(vec![]);
        e.call(0x0074_0980, &args![this]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 3);
        assert_eq!(e.mem.u32(player + 0x61c), 130);
        assert_eq!(e.mem.u32(player + 0x624), 3);
        assert_eq!(e.mem.u32(player + 0x62c), 30);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x25]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], WIN_SOUND);
        assert_eq!(
            &calls_to(&e, PRINT_BOUNDED)[0][1..],
            &[0x104, GAMES_FORMAT, 3, 0]
        );
        assert!(calls_to(&e, SET_TRAIT_FROM_UNSIGNED).contains(&vec![
            tile_of(&e, this, 31),
            0x41,
            130
        ]));
        assert!(calls_to(&e, SETTING_STRING).contains(&vec![RESULT_WON_SETTING]));
        // The payout of the win.
        assert_eq!(
            calls_to(&e, FILL_BARTER_CONTAINER),
            vec![vec![handler, challenger]]
        );
        // A loss.
        e.set(this, CaravanMenu::iFlags, 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_0980, &args![this]);
        assert_eq!(e.mem.u32(player + 0x620), 30);
        assert_eq!(e.mem.u32(player + 0x628), 1);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x26]]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], LOSE_SOUND);
    }

    #[test]
    fn create_refuses_small_decks_and_other_menus() {
        let mut e = engine();
        let (_, _) = setup(&mut e);
        e.set_global(CARAVAN_MENU, 0u32);
        e.set_global(MESSAGE_TIME, 2.0f32.to_bits());
        e.register(CARD_LIST_COUNT, |_, _| ret(14));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0074_1060, &args![1u32, 2u32, 3u32, 0u32, 9u32])
                .u32(),
            0
        );
        assert_eq!(
            calls_to(&e, SHOW_CORNER_MESSAGE)[0][1..],
            [0, MESSAGE_ICON, 0, 2.0f32.to_bits(), 0]
        );
        assert_eq!(count(&e, TILE_READ_FILE), 0);
        // Enough cards, but the file's menu is another class: deleted.
        e.register(CARD_LIST_COUNT, |_, _| ret(15));
        let other = object_with_vtable(&mut e);
        e.register(0x00d0_0034, |_, _| ret(0x400));
        e.register_double(TILE_GET_MENU, move |_, _| ret(other));
        e.register(WORD_AT_FOUR, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0074_1060, &args![1u32, 2u32, 3u32, 0u32, 9u32])
                .u32(),
            0
        );
        assert_eq!(calls_to(&e, TILE_READ_FILE)[0][1], MENU_FILE);
        assert_eq!(calls_to(&e, 0x00d0_0000), vec![vec![other, 1]]);
    }

    #[test]
    fn create_opens_a_caravan_menu() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x34, 0x0073_bb30);
        e.mem.set_u32(this.addr(), vtable);
        e.register(CARD_LIST_COUNT, |_, _| ret(20));
        e.register_double(TILE_GET_MENU, move |_, _| ret(this.addr()));
        e.register(TILE_READ_FILE, |_, _| ret(0x2007));
        e.register(WORD_AT_0X24, |_, _| ret(0x24));
        e.register(TUTORIAL_SHOWN, |_, _| ret(1));
        let scene = object_with_vtable(&mut e);
        e.register_double(SCENE_NODE, move |_, _| ret(scene));
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        let changes = e.mem.alloc(4);
        e.mem.set_u32(handler + 0x628, changes);
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.register_double(POINTER_ASSIGN, move |e, a| {
            if a[0] != MONEY_ROOT {
                e.mem.set_u32(a[0], a[1]);
            }
            Ret::default()
        });
        let player_bits = 0.25f32.to_bits();
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x0074_1060,
                &args![0x4000u32, 0x3000u32, 2u32, player_bits, 9u32],
            )
            .u32();
        assert_eq!(result, 0x2007);
        assert_eq!(e.mem.u32(this.addr() + CHALLENGER), 0x4000);
        assert_eq!(e.mem.u32(this.addr() + DECK_PARAM), 0x24);
        assert_eq!(e.mem.u32(this.addr() + DIFF_LEVEL), 2);
        assert_eq!(e.mem.u32(this.addr() + BET_PERCENTAGE), player_bits);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this.addr(), 0x2007, 0]]
        );
        assert_eq!(e.get(this, CaravanMenu::eActiveMenu), 0);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags) & CAF_DISABLE_ALLCONTROLS,
            0
        );
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
        assert_eq!(e.mem.u8(CREATE_BYTE), 0);
        // A missing tile: printed, closed, 0.
        e.mem.set_u32(this.addr() + TILES + 4 * 9, 0);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x0074_1060,
                &args![0x4000u32, 0x3000u32, 2u32, player_bits, 9u32],
            )
            .u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn prepare_ante_menu_loads_money_and_shows_the_bets() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        e.set_global(UT_VALUE, 0x41u32);
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        let changes = e.mem.alloc(4);
        e.mem.set_u32(handler + 0x628, changes);
        e.register(GET_GOLD_AMOUNT, |_, _| ret(40));
        e.mem.set_f32(menu + BET_PERCENTAGE, 0.5);
        e.register(MODEL_LOAD_FILE, |_, _| ret(0x3333));
        e.register(STRING_LENGTH, |_, _| ret(19));
        e.register(TILE_CHILD_NAMED, |_, _| ret(0x2020));
        e.register(GET_FLOAT_TRAIT, |_, _| ret_f(100.0));
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.call_log = Some(vec![]);
        e.call(0x0073_d020, &args![this]);
        assert_eq!(count(&e, MODEL_LOAD_FILE), 8);
        assert_eq!(e.mem.u32(menu + BILLS_GRID), 0x3333);
        assert_eq!(e.mem.u32(menu + COIN_GEOMETRY + 8), 0x3333);
        assert_eq!(e.get(this, CaravanMenu::iTotalFunds), 40);
        assert_eq!(e.get(this, CaravanMenu::iNPCAnte), 20);
        // 19 letters: 2 * 15 wider.
        assert!(calls_to(&e, SET_FLOAT_TRAIT).contains(&vec![
            0x2020,
            0xfa1,
            130.0f32.to_bits(),
            1
        ]));
        let appends = calls_to(&e, ARRAY_APPEND);
        let into = |list: u32| appends.iter().filter(|a| a[0] == menu + list).count();
        assert_eq!(into(INT_LIST_BILLS), 9);
        assert_eq!(into(INT_LIST_COINS), 0x2a);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).contains(&vec![tile_of(&e, this, 6), 0x41, 40]));
    }

    #[test]
    fn select_track_card_moves_the_cursor_and_textures_it() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, menu + PLAYER_HAND, &[five]);
        e.mem.set_u32(menu + TRACK_GRIDS + 4, 0x6611);
        e.mem.set_u32(menu + CURSORS, 0xc0);
        e.mem.set_u32(menu + CURSORS + 4, 0xc1);
        e.set_global(CURSOR_AWAY_HEIGHT, (-50.0f32).to_bits());
        let slot = e.mem.alloc(0x60);
        for index in 0..9 {
            e.mem.set_u32(slot + 0x34 + 4 * index, 0x100 + index);
        }
        let product = e.mem.alloc(0x24);
        e.mem.set_u32(product, 0x77);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 8, 0x99);
        e.register_double(OBJECT_BY_NAME, move |_, a| {
            ret(match a[0] {
                0x6611 => slot,
                0 => 0,
                _ => 0x0123,
            })
        });
        e.register_double(MATRIX_MULTIPLY, move |_, _| ret(product));
        e.register_double(GET_WORLD_BOUND, move |_, _| ret(bound));
        let (_, property) = textured_node(&mut e, 8);
        e.call_log = Some(vec![]);
        e.call(0x0074_e6d0, &args![this, 1i32, 0i32]);
        let names = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&names[0][1..], &[0x104, SELECT_SLOT_FORMAT, 1, 1]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], TOGGLE_TRACK_SOUND);
        assert_eq!(
            calls_to(&e, SET_LOCAL_TRANSLATE),
            vec![vec![0xc1, 0, 0, (-50.0f32).to_bits()]]
        );
        let rotate = calls_to(&e, SET_LOCAL_ROTATE);
        assert_eq!(rotate[0][0], 0xc0);
        assert_eq!(e.mem.u32(rotate[0][1]), 0x77);
        let translate = calls_to(&e, SET_LOCAL_TRANSLATE_VECTOR);
        assert_eq!(e.mem.u32(translate[0][1] + 8), 0x99);
        assert_eq!(calls_to(&e, OBJECT_BY_NAME)[1], vec![0xc0, CURSOR_ADD_NAME]);
        assert_eq!(calls_to(&e, 0x00d0_00f0), vec![vec![property, 0, 0, 0x7e7]]);
        // Invalid: the other cursor and sound.
        e.call_log = Some(vec![]);
        e.call(0x0074_e6d0, &args![this, 4i32, 0i32]);
        assert_eq!(count(&e, SET_LOCAL_ROTATE), 0);
    }

    #[test]
    fn constructor_sets_up_members_and_defaults() {
        let mut e = engine();
        let this = e.new_object::<CaravanMenu>();
        let menu = this.addr();
        e.register(TILE_ADD_USER_TRAIT, |_, a| ret(a[0] & 0xff));
        e.register(OPERATOR_NEW, |_, _| ret(0x5150));
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0] + 1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_b7e0, &args![this]).u32(), menu);
        assert_eq!(e.mem.u32(menu), CARAVAN_MENU_VTABLE);
        assert_eq!(e.mem.u32(menu + LIST_OF_COINS), ARRAY_VTABLE_01071AEC);
        assert_eq!(e.mem.u32(menu + PLAYER_HAND), ARRAY_VTABLE_01071B00);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![
                vec![
                    menu + JOKER_LOCATIONS,
                    0x10,
                    6,
                    INT_ARRAY_CONSTRUCT,
                    INT_ARRAY_DESTRUCT
                ],
                vec![menu + CARD_TRACKS, 0x10, 0x2a, 0x0075_30c0, 0x0075_30f0]
            ]
        );
        assert_eq!(e.global::<u32>(UT_CURRENT_VALUE), 0xd8);
        assert_eq!(e.global::<u32>(UT_VALUE), 0x18);
        assert_eq!(e.global::<u32>(UT_X), 0x90);
        assert_eq!(e.get(this, CaravanMenu::iTotalFunds), 0x64);
        assert_eq!(e.get(this, CaravanMenu::iCardsInDeck), 0x19);
        assert_eq!(e.mem.u8(menu + BYTE_E86), 1);
        assert_eq!(e.get(this, CaravanMenu::iFlags), CAF_DISABLE_ALLCONTROLS);
        assert_eq!(e.mem.u32(menu + LIST_OF_BILL_ANIMATIONS), 0x5151);
        assert_eq!(e.mem.u32(menu + LIST_OF_COIN_ANIMATIONS), 0x5151);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        e.call(0x0073_b7e0, &args![this]);
        assert_eq!(e.mem.u32(menu + LIST_OF_COIN_ANIMATIONS), 0);
    }

    #[test]
    fn destructor_releases_by_the_last_menu() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let scene = object_with_vtable(&mut e);
        e.register_double(SCENE_NODE, move |_, _| ret(scene));
        e.mem.set_u32(MONEY_ROOT, 0x4040);
        e.register(CHILD_COUNT, |_, _| ret(1));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        fill_array(&mut e, menu + LIST_OF_COINS, &[1, 2]);
        e.register(ARRAY_LAST_SLOT, |e, a| ret(e.mem.u32(a[0] + 4)));
        let anims = e.mem.alloc(8);
        e.mem.set_u32(menu + LIST_OF_BILL_ANIMATIONS, anims);
        e.mem.set_u32(anims, 0x77);
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.set(this, CaravanMenu::eActiveMenu, 2);
        let ten = card(&mut e, 10, 1, 0);
        fill_array(&mut e, card_track(menu, 4, 3), &[ten]);
        e.call_log = Some(vec![]);
        e.call(0x0073_bba0, &args![this]);
        assert_eq!(e.mem.u32(menu), CARAVAN_MENU_VTABLE);
        assert_eq!(e.global::<u32>(CARAVAN_MENU), 0);
        assert_eq!(e.mem.u32(MONEY_ROOT), 0);
        assert_eq!(calls_to(&e, 0x00d0_00e8), vec![vec![scene, 0x4040]]);
        assert_eq!(calls_to(&e, ROOT_RELEASE_00572160), vec![vec![0x4040]]);
        assert_eq!(e.mem.u32(menu + LIST_OF_COINS + 8), 0);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![anims, 1]]);
        assert_eq!(e.mem.u32(menu + LIST_OF_BILL_ANIMATIONS), 0);
        // Game menu: the track row with a card is released, models by
        // name and by path.
        assert!(calls_to(&e, ARRAY_RELEASE_008454F0).contains(&vec![card_track(menu, 4, 3), 1]));
        let releases = calls_to(&e, MODEL_RELEASE);
        // Table, 8 money, 8 game, 4 folder, 42 track rows.
        assert_eq!(releases.len(), 1 + 8 + 8 + 4 + 42);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![
                vec![menu + CARD_TRACKS, 0x10, 0x2a, 0x0075_30f0],
                vec![menu + JOKER_LOCATIONS, 0x10, 6, INT_ARRAY_DESTRUCT]
            ]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![menu]]);
        // Deck menu: the deck models, and the deck array released.
        e.set(this, CaravanMenu::eActiveMenu, 1);
        fill_array(&mut e, menu + DECK_ARRAY, &[ten]);
        e.call_log = Some(vec![]);
        e.call(0x0073_bba0, &args![this]);
        assert_eq!(calls_to(&e, MODEL_RELEASE).len(), 1 + 8 + 2);
        assert!(calls_to(&e, ARRAY_RELEASE_008454F0).contains(&vec![menu + DECK_ARRAY, 1]));
    }

    #[test]
    fn payout_moves_money_entries_then_caps() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        let menu = this.addr();
        let challenger = object_with_vtable(&mut e);
        e.mem.set_u32(challenger + 0x20, 0x2020);
        e.mem.set_u32(menu + CHALLENGER, challenger);
        let vtable = e.mem.u32(challenger);
        e.mem.set_u32(player, vtable);
        e.mem.set_u32(menu + ANTE, 50);
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        let changes = e.mem.alloc(4);
        e.mem.set_u32(handler + 0x628, changes);
        let node = e.mem.alloc(8);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(changes, node);
        e.mem.set_u32(node, entry);
        e.mem.set_u32(entry + 4, 3);
        e.mem.set_u32(entry + 8, 0x600);
        e.register(FORM_TYPE, |_, _| ret(0x74));
        e.register(GET_FORM_VALUE, |_, _| ret(20));
        e.register(FORM_BY_ID, |_, _| ret(0xf0f));
        e.register(GET_GOLD_AMOUNT, |_, _| ret(5));
        e.register(WORD_AT_0X20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.call_log = Some(vec![]);
        e.call(0x0074_cc70, &args![this, 1u8]);
        // 3 * 20 > 50: two items (40), then 10 in caps: 5 from the barter
        // container, 5 from the challenger.
        let removed = calls_to(&e, REMOVE_BARTER_ITEM);
        assert_eq!(removed[0], vec![handler, entry, 2, 0x2020]);
        assert_eq!(removed[1][2], 5);
        assert_eq!(
            calls_to(&e, 0x00d0_017c),
            vec![vec![challenger, 0xf0f, 0, 5, 0, 0, 0, 0, 0, 1, 0]]
        );
        // Lost: the player pays, the caps limited by the player's gold.
        e.register(ACTOR_GOLD, |_, _| ret(100));
        e.call_log = Some(vec![]);
        e.call(0x0074_cc70, &args![this, 0u8]);
        assert_eq!(
            calls_to(&e, 0x00d0_017c),
            vec![
                vec![player, 0x600, 0, 2, 0, 0, 0, 0, 0, 1, 0],
                vec![player, 0xf0f, 0, 10, 0, 0, 0, 0, 0, 1, 0]
            ]
        );
    }

    #[test]
    fn hand_and_deck_back_textures() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let (_, property) = textured_node(&mut e, 0xc);
        e.register(OBJECT_BY_NAME, |_, a| ret(a[0] + 1));
        e.mem.set_u32(menu + HAND_GEOMETRY + 4, 0x194);
        let cards: Vec<u32> = (0..5).map(|value| card(&mut e, value, 1, 0)).collect();
        fill_array(&mut e, menu + NPC_HAND, &cards);
        fill_array(&mut e, menu + NPC_DECK, &[0, cards[2]]);
        e.mem.set_u32(menu + NEXT_NPC_CARD, 1);
        e.call_log = Some(vec![]);
        e.call(0x0074_d190, &args![this, 0u8]);
        assert_eq!(
            calls_to(&e, COPY_BOUNDED)
                .iter()
                .map(|a| a[2])
                .collect::<Vec<_>>(),
            vec![NPC_SLOT_6_NAME, NPC_DECK_BACK_NAME]
        );
        let paths = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(paths[0][4], cards[4] + 0xa8 + 1);
        assert_eq!(&paths[1][2..], &[TEXTURES_FORMAT, cards[2] + 0xa8 + 1]);
        assert_eq!(count(&e, 0x00d0_00f0), 2);
        assert_eq!(calls_to(&e, 0x00d0_00f0)[0][0], property);
        // An empty deck culls the back.
        fill_array(&mut e, menu + NPC_DECK, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0074_d190, &args![this, 0u8]);
        assert_eq!(calls_to(&e, SET_CULLED), vec![vec![0x195, 1]]);
    }

    #[test]
    fn deck_meter_card_textures_its_face_shaders() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        textured_node(&mut e, 0xc);
        let mut shaders = vec![];
        for group in [0x8ecu32, 0x94c, 0x9ac, 0xa0c] {
            let shader = object_with_vtable(&mut e);
            e.mem.set_u32(menu + group + 0x30, shader);
            e.mem.set_u32(menu + group + 4 * 2, shader);
            shaders.push(shader);
        }
        let in_deck = card(&mut e, 3, 1, 1);
        fill_array(&mut e, menu + DECK_ARRAY, &[in_deck]);
        e.call_log = Some(vec![]);
        e.call(0x0074_af70, &args![this, 2u32]);
        assert_eq!(
            calls_to(&e, 0x00d0_00f0)
                .iter()
                .map(|a| a[0])
                .collect::<Vec<_>>(),
            shaders
        );
        let texts: Vec<u32> = calls_to(&e, PRINT_BOUNDED).iter().map(|a| a[4]).collect();
        let face = in_deck + 0x9c + 1;
        let back = in_deck + 0xa8 + 1;
        assert_eq!(texts, vec![face, back, back, face]);
        // Group entries 12 missing: nothing.
        e.mem.set_u32(menu + 0x9dc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_af70, &args![this, 2u32]);
        assert_eq!(count(&e, PRINT_BOUNDED), 0);
    }

    #[test]
    fn update_game_hand_textures_shows_cards_and_culls_empty_slots() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        textured_node(&mut e, 0xc);
        e.register(OBJECT_BY_NAME, |_, _| ret(0x0b0b));
        e.mem.set_u32(menu + HAND_GEOMETRY, 0x190);
        let cards: Vec<u32> = (0..3).map(|value| card(&mut e, value, 1, 0)).collect();
        fill_array(&mut e, menu + PLAYER_HAND, &cards);
        e.call_log = Some(vec![]);
        e.call(0x0074_d730, &args![this, 1u8, 1u32]);
        // Three cards textured; five empty slots culled with their shadows
        // (fewer than 5 cards, not setting up); then the slot-6 back (no deck).
        assert_eq!(count(&e, CREATE_TEXTURE_IMAGE), 3);
        assert_eq!(count(&e, SET_CULLED), 10);
        assert_eq!(calls_to(&e, COPY_BOUNDED)[0][2], PLAYER_SLOT_6_NAME);
        // Setting up: slot 5 stays, the rest culled; no slot-6 back.
        e.set(this, CaravanMenu::iFlags, CAF_TRACK_SETUP);
        e.call_log = Some(vec![]);
        e.call(0x0074_d730, &args![this, 1u8, 1u32]);
        assert_eq!(count(&e, SET_CULLED), 8);
        assert_eq!(count(&e, COPY_BOUNDED), 0);
    }

    #[test]
    fn update_betting_ui_builds_a_pile_and_places_it() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.set_global(BILL_ODDS, 1.5f64);
        // Rolls: bill 8, coin 3 (a bill); then the kind 2 (40).
        let rolls = std::rc::Rc::new(std::cell::RefCell::new(vec![8u32, 3, 2]));
        let queue = rolls.clone();
        e.register_double(RANDOM_RANGE, move |_, _| {
            let mut queue = queue.borrow_mut();
            ret(if queue.is_empty() { 0 } else { queue.remove(0) })
        });
        e.call_log = Some(vec![]);
        e.call(0x0074_a2c0, &args![this, 0x100i32, 40i32]);
        assert_eq!(calls_to(&e, RANDOM_RANGE)[2], vec![0, 3]);
        assert_eq!(calls_to(&e, OBJECT_CLONE).len(), 1);
        assert_eq!(e.get(this, CaravanMenu::eGameState), CA_ANTE_MONEYIN_ANIM);
        assert_eq!(count(&e, ADD_ALL_LIGHTS), 1);
        // Placing: one bill on a free placement.
        let bill = e.mem.alloc(0x70);
        fill_array(&mut e, menu + LIST_OF_BILLS, &[bill]);
        fill_array(&mut e, menu + INT_LIST_BILLS, &[4, 6]);
        let placement = e.mem.alloc(0x60);
        e.register_double(OBJECT_BY_NAME, move |_, _| ret(placement));
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound, 1.0);
        e.register_double(GET_WORLD_BOUND, move |_, _| ret(bound));
        rolls.borrow_mut().extend([1u32]);
        e.call_log = Some(vec![]);
        e.call(0x0074_a2c0, &args![this, 0x100i32, 0i32]);
        assert_eq!(
            calls_to(&e, ARRAY_REMOVE),
            vec![vec![menu + INT_LIST_BILLS, 1, 1]]
        );
        assert_eq!(
            &calls_to(&e, PRINT_BOUNDED)[0][1..],
            &[0x104, BILL_PLACEMENT_FORMAT, 7]
        );
        assert_eq!(
            calls_to(&e, SET_LOCAL_TRANSLATE),
            vec![vec![bill, ONE, 0, 0]]
        );
        assert_eq!(calls_to(&e, SET_LOCAL_ROTATE)[0][0], bill);
    }

    #[test]
    fn prepare_deck_menu_loads_the_decks_and_finds_the_shaders() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        let menu = this.addr();
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.set_global(UT_VALUE, 0x41u32);
        e.register(MODEL_LOAD_FILE, |_, a| ret(a[1] & 0xffff));
        e.register(GET_CONTROLLER, |_, a| ret(a[0] + 1));
        e.register(GET_SEQUENCE, |_, _| ret(0x5e));
        let object = e.mem.alloc(0x100);
        e.register_double(OBJECT_BY_NAME, move |_, _| ret(object));
        textured_node(&mut e, 0xc);
        e.set(this, CaravanMenu::iPlayerAnte, 20);
        e.set(this, CaravanMenu::iNPCAnte, 15);
        e.mem.set_u32(player + 0x618, 0x618);
        e.register(CARD_LIST_COUNT, |_, _| ret(33));
        let deck_card = card(&mut e, 2, 1, 0);
        fill_array(&mut e, menu + DECK_ARRAY, &[deck_card]);
        e.call_log = Some(vec![]);
        e.call(0x0073_d850, &args![this]);
        assert_eq!(e.mem.u32(menu + ACTIVE_DECK_GEOMETRY), 0x121c);
        assert_eq!(e.mem.u32(menu + INACTIVE_DECK_GEOMETRY), 0x11e4);
        assert_eq!(e.mem.u32(menu + ANTE), 15);
        assert_eq!(e.mem.u32(menu + ACTIVE_DECK_ANIM_MANAGER), 0x121d);
        assert_eq!(e.mem.u32(menu + ACTIVE_DECK_ANIMATIONS + 0x14), 0x5e);
        assert_eq!(e.mem.u32(menu + INACTIVE_DECK_ANIM_MANAGER), 0x11e5);
        assert_ne!(e.mem.u32(menu + 0xa0c + 4 * 0x17), 0);
        assert_eq!(e.get(this, CaravanMenu::eActiveMenu), 1);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 1);
        assert!(calls_to(&e, SET_TRAIT_FROM_UNSIGNED).contains(&vec![
            tile_of(&e, this, 8),
            0x41,
            33
        ]));
        let formats: Vec<u32> = calls_to(&e, PRINT_BOUNDED)
            .iter()
            .filter(|a| a[1] == 0x19)
            .map(|a| a[2])
            .collect();
        assert_eq!(
            &formats[..4],
            &[
                DECK_FACE_FORMAT,
                DECK_BACK_FORMAT,
                AVAILABLE_FACE_FORMAT,
                AVAILABLE_BACK_FORMAT
            ]
        );
    }

    #[test]
    fn swap_deck_building_textures_hides_and_shows_slots() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        textured_node(&mut e, 0xc);
        let object = e.mem.alloc(0x100);
        e.register_double(OBJECT_BY_NAME, move |_, _| ret(object));
        let shader = object_with_vtable(&mut e);
        for group in [0x8ecu32, 0x94c, 0x9ac, 0xa0c] {
            e.mem.set_u32(menu + group + 4 * 11, shader);
        }
        let in_deck = card(&mut e, 3, 1, 1);
        fill_array(&mut e, menu + DECK_ARRAY, &[in_deck]);
        e.set(this, CaravanMenu::iTotalCards, 1);
        e.call_log = Some(vec![]);
        // Slot 11 shows card 0.
        e.call(0x0074_b700, &args![this, 0i32, 0u8]);
        assert_eq!(count(&e, SET_LOCAL_TRANSLATE), 6);
        assert_eq!(count(&e, SHADER_VALUE_00526AE0), 1);
        assert_eq!(count(&e, 0x00d0_00f0), 4);
        // Card 1 does not exist: hidden at -50.
        e.call_log = Some(vec![]);
        e.call(0x0074_b700, &args![this, 1i32, 0u8]);
        let low = (-50.0f32).to_bits();
        assert!(calls_to(&e, SET_LOCAL_TRANSLATE)
            .iter()
            .all(|a| a[3] == low));
        assert_eq!(count(&e, SHADER_VALUE_00526AE0), 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_b700, &args![this, 1i32, 1u8]);
        assert!(calls_to(&e, SET_LOCAL_TRANSLATE).iter().all(|a| a[3] == 0));
    }

    #[test]
    fn do_gamepad_refuses_and_handles_buttons() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        let object = object_with_vtable(&mut e);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(menu, vtable);
        e.set_global(UT_VALUE, 0x41u32);
        // Not on top: answers whether the state is 3.
        assert!(!e.call(0x0074_7d30, &args![this, 9i32, 0u32]).bool());
        e.set(this, CaravanMenu::eGameState, 3);
        assert!(e.call(0x0074_7d30, &args![this, 9i32, 0u32]).bool());
        e.set(this, CaravanMenu::eGameState, 0);
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP);
        // Deck menu: A adds the meter's card.
        e.set(this, CaravanMenu::eActiveMenu, 1);
        let out = card(&mut e, 3, 1, 0);
        fill_array(&mut e, menu + DECK_ARRAY, &[out]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0074_7d30, &args![this, 9i32, 0u32]).bool());
        assert_eq!(e.get(this, CaravanMenu::iCardsInDeck), 1);
        assert_eq!(calls_to(&e, CARD_SET_BYTE_98), vec![vec![out, 1]]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), CA_DECK_FLIP_ANIM);
        // A refused while disabled.
        e.set(this, CaravanMenu::eGameState, 0);
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP | CAF_DISABLE_A);
        assert!(!e.call(0x0074_7d30, &args![this, 9i32, 0u32]).bool());
        // Ante: X asks for an amount.
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP);
        e.set(this, CaravanMenu::eActiveMenu, 0);
        e.set(this, CaravanMenu::iTotalFunds, 100);
        e.set(this, CaravanMenu::iPlayerAnte, 30);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0074_7d30, &args![this, 11i32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, QUANTITY_MENU_CREATE),
            vec![vec![70, ITEM_SELECT_CALLBACK, 0x7fff_ffff]]
        );
        // Game: RT selects the first track for destroying; A then discards it.
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP);
        e.set(this, CaravanMenu::eActiveMenu, 2);
        e.set_global(TRACK_RAISE, 3.0f32.to_bits());
        assert!(!e.call(0x0074_7d30, &args![this, 14i32, 0u32]).bool());
        assert_eq!(
            e.get(this, CaravanMenu::iFlags),
            CAF_ISONTOP | CAF_DESTROY_TRACK | CAF_SELECT_TRACK
        );
        e.mem.set_u32(track_info(menu, 0) + 8, 9);
        assert!(e.call(0x0074_7d30, &args![this, 9i32, 0u32]).bool());
        assert_eq!(e.mem.u32(track_info(menu, 0) + 8), 0);
        assert_eq!(
            e.get(this, CaravanMenu::eGameState),
            CA_GAME_DISCARDTRACK_ANIM
        );
        // LT discards the selected hand card and draws the next one.
        e.set(this, CaravanMenu::eGameState, 0);
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP);
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, menu + PLAYER_DECK, &[five]);
        fill_array(&mut e, menu + PLAYER_HAND, &[five]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0074_7d30, &args![this, 13i32, 0u32]).bool());
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0xe);
        assert_eq!(
            calls_to(&e, ARRAY_REMOVE_AT),
            vec![
                vec![menu + PLAYER_HAND, 0, 1],
                vec![menu + PLAYER_DECK, 0, 1]
            ]
        );
    }

    #[test]
    fn prepare_game_menu_deals_and_loads_the_table() {
        let mut e = engine();
        let (this, player) = setup(&mut e);
        let menu = this.addr();
        let root = object_with_vtable(&mut e);
        e.mem.set_u32(MONEY_ROOT, root);
        e.set_global(UT_VALUE, 0x41u32);
        e.mem.set_u32(player + 0x614, 0x614);
        e.mem.set_u32(player + 0x618, 0x618);
        let in_deck = card(&mut e, 3, 1, 1);
        let out = card(&mut e, 4, 1, 0);
        fill_array(&mut e, menu + DECK_ARRAY, &[in_deck, out]);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0xdead);
        e.mem.set_u32(menu + DECK_PARAM, node);
        e.register(MODEL_LOAD_FILE, |_, _| ret(0x0b0b));
        e.register(GET_CONTROLLER, |_, _| ret(0x6c));
        let sequence = e.mem.alloc(0x40);
        e.register_double(GET_SEQUENCE, move |_, _| ret(sequence));
        e.call_log = Some(vec![]);
        e.call(0x0073_ea90, &args![this]);
        assert_eq!(
            calls_to(&e, CARD_LIST_ADD_005AE3D0)
                .iter()
                .map(|a| a[0])
                .collect::<Vec<_>>(),
            vec![0x618, 0x614]
        );
        let appends = calls_to(&e, ARRAY_APPEND);
        assert_eq!(appends[0][0], menu + PLAYER_DECK);
        assert_eq!(appends[1], vec![menu + NPC_DECK, node]);
        assert_eq!(e.mem.u32(menu + DECK_PARAM), 0);
        // 8 deals per side.
        assert_eq!(
            appends
                .iter()
                .filter(|a| a[0] == menu + PLAYER_HAND)
                .count(),
            8
        );
        assert_eq!(count(&e, MODEL_LOAD_FILE), 42 + 6 + 2 + 2);
        assert_eq!(e.mem.u32(menu + HAND_ANIM_MANAGER + 4), 0x6c);
        assert_eq!(
            e.mem.u32(menu + TRACK_ANIM_MANAGER + 5 * 0x1c + 6 * 4),
            0x6c
        );
        assert_eq!(e.get(this, CaravanMenu::eActiveMenu), 2);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 2);
        let set = CAF_AI | CAF_DISABLE_ALLCONTROLS;
        assert_eq!(e.get(this, CaravanMenu::iFlags) & set, set);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).contains(&vec![tile_of(&e, this, 16), 0x41, 0]));
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_destructor_and_frees() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let scene = object_with_vtable(&mut e);
        e.register_double(SCENE_NODE, move |_, _| ret(scene));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_bb70, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0073_bb70, &args![this, 0u32]);
        assert_eq!(count(&e, OPERATOR_DELETE), 0);
    }

    /// Working doubles for the opponent's move search: allocations, the
    /// candidate array and the package setters.
    fn move_search_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(ARRAY_APPEND, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x200);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(ARRAY_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) == 0))
        });
        e.register(ARRAY_LAST_SLOT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 4 * (e.mem.u32(a[0] + 8) - 1))
        });
        for (setter, offset) in [
            (PACKAGE_SET_TRACK, 0xcu32),
            (PACKAGE_SET_ROW, 0x10),
            (PACKAGE_SET_DECK_CARD, 0x14),
            (PACKAGE_SET_PRIORITY, 0x18),
            (PACKAGE_SET_ACTION, 8),
        ] {
            e.register_double(setter, move |e, a| {
                e.mem.set_u32(a[0] + offset, a[1]);
                Ret::default()
            });
        }
        e.register(PACKAGE_PRIORITY, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(ABS, |_, a| ret((a[0] as i32).unsigned_abs()));
        // The candidate array's vtable: slot 0 deletes.
        e.mem.set_u32(ARRAY_VTABLE_01071B14, 0x00d0_0f00);
        e.register(0x00d0_0f00, |_, _| Ret::default());
    }

    fn out_words(e: &Engine, out: u32) -> Vec<u32> {
        (0..7).map(|word| e.mem.u32(out + 4 * word)).collect()
    }

    #[test]
    fn opponent_setup_move_by_level() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        move_search_doubles(&mut e);
        e.set(this, CaravanMenu::iFlags, CAF_TRACK_SETUP);
        let cards: Vec<u32> = [5u32, 12, 7]
            .iter()
            .map(|v| card(&mut e, *v, 1, 0))
            .collect();
        fill_array(&mut e, menu + NPC_HAND, &cards);
        let out = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x0074_fdc0, &args![this, out]).u32(), out);
        let words = out_words(&e, out);
        assert_eq!(words[2], SETUPTRACK);
        assert_eq!(words[5], 2);
        // Level 3: the number card closest to 6.
        let cards: Vec<u32> = [9u32, 6, 2]
            .iter()
            .map(|v| card(&mut e, *v, 1, 0))
            .collect();
        fill_array(&mut e, menu + NPC_HAND, &cards);
        e.mem.set_u32(menu + DIFF_LEVEL, 3);
        e.call(0x0074_fdc0, &args![this, out]);
        assert_eq!(out_words(&e, out)[5], 1);
        // Level 1 with six face cards: discard, the joker first.
        let cards: Vec<u32> = [12u32, 13, 14, 15, 12, 13]
            .iter()
            .map(|v| card(&mut e, *v, 1, 0))
            .collect();
        fill_array(&mut e, menu + NPC_HAND, &cards);
        e.mem.set_u32(menu + DIFF_LEVEL, 1);
        e.call(0x0074_fdc0, &args![this, out]);
        let words = out_words(&e, out);
        assert_eq!(words[2], DISCARDCARD);
        assert_eq!(words[5], 3);
    }

    #[test]
    fn opponent_builds_its_empty_tracks() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        move_search_doubles(&mut e);
        let five = card(&mut e, 5, 1, 0);
        fill_array(&mut e, menu + NPC_HAND, &[five]);
        let out = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0074_fdc0, &args![this, out]);
        // Goal 5 (below 21), build track 3 row 0 with card 0, priority 5.
        assert_eq!(
            out_words(&e, out),
            vec![5, u32::MAX, BUILDTRACK, 3, 0, 0, 5]
        );
        // Three candidates were freed, then the array.
        assert_eq!(count(&e, OPERATOR_DELETE), 3);
        assert_eq!(count(&e, 0x00d0_0f00), 1);
        assert_eq!(calls_to(&e, ARRAY_SORT)[0][1], PACKAGE_COMPARE);
    }

    #[test]
    fn opponent_without_moves_discards_its_highest_card() {
        let mut e = engine();
        let (this, _) = setup(&mut e);
        let menu = this.addr();
        move_search_doubles(&mut e);
        // A queen with nothing to act on: no candidates.
        let queen = card(&mut e, 13, 1, 0);
        let two = card(&mut e, 2, 1, 0);
        fill_array(&mut e, menu + NPC_HAND, &[two, queen]);
        // All opponent tracks full: no number card fits.
        for track in 3..6 {
            for row in 0..7 {
                fill_array(&mut e, card_track(menu, track, row), &[two, two, two, two]);
            }
        }
        let out = e.mem.alloc(0x1c);
        e.call(0x0074_fdc0, &args![this, out]);
        let words = out_words(&e, out);
        assert_eq!(words[2], DISCARDCARD);
        assert_eq!(words[5], 1);
        assert_eq!(words[6], 1);
    }

    #[test]
    fn render_scene_uses_the_caravan_camera_or_the_holders() {
        let mut e = engine();
        e.map(0x0103_0000, 0x1000);
        e.mem.set_u32(RENDER_VALUE_01030FF4, 0x3e19_999a);
        let holder = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 0xb4, 0x5ce);
        for callee in [
            RENDER_CLEAR_007148C0,
            SCENE_BEGIN_00C4F270,
            SCENE_RESET_00C4F2D0,
            SCENE_SET_NODE_004A0FD0,
            RENDER_TO_TEXTURE_00709D50,
            ACCUMULATE_SCENE,
            RENDER_00B6C0D0,
            NI_POINTER_DESTRUCTOR,
        ] {
            e.register_double(callee, |_, _| Ret::default());
        }
        e.register_double(RENDER_OWNER_0043C4B0, |_, _| ret(0x0e1));
        e.register_double(MAIN_CAMERA_0045C670, move |_, _| ret(holder));
        e.register_double(RENDER_GLOBAL_004B7210, |_, _| ret(0x91));
        e.register_double(RENDERED_MENU_007FD640, |_, _| ret(0x70d));
        e.register_double(HOLDER_CAMERA_006629F0, |_, _| ret(0xcafe));
        e.register_double(SCENE_GRAPH_00450B80, |_, _| ret(0x6a));
        e.register_double(NI_POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.set_global(CAMERA_POINTER, 0xca);
        e.call_log = Some(vec![]);
        e.call(0x0074_0f30, &args![]);
        assert_eq!(calls_to(&e, RENDER_CLEAR_007148C0), vec![vec![0x0e1, 0, 4]]);
        assert_eq!(calls_to(&e, SCENE_BEGIN_00C4F270), vec![vec![0x5ce, 1]]);
        assert_eq!(
            calls_to(&e, RENDER_TO_TEXTURE_00709D50),
            vec![vec![0xca, 0x3e19_999a]]
        );
        assert_eq!(
            calls_to(&e, SCENE_SET_NODE_004A0FD0),
            vec![vec![0x5ce, 0x70d], vec![0x5ce, 0]]
        );
        assert_eq!(
            calls_to(&e, ACCUMULATE_SCENE),
            vec![vec![0xca, 0x6a, 0x5ce]]
        );
        assert_eq!(calls_to(&e, RENDER_00B6C0D0), vec![vec![0xca, 0x70d, 0]]);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN)[0][1], 0);
        assert_eq!(count(&e, SCENE_RESET_00C4F2D0), 1);
        assert_eq!(count(&e, NI_POINTER_DESTRUCTOR), 1);
        assert_eq!(count(&e, HOLDER_CAMERA_006629F0), 0);
        // Without the caravan camera the holder's own renders.
        e.set_global(CAMERA_POINTER, 0);
        e.call_log = Some(vec![]);
        e.call(0x0074_0f30, &args![]);
        assert_eq!(calls_to(&e, HOLDER_CAMERA_006629F0), vec![vec![holder]]);
        assert_eq!(
            calls_to(&e, ACCUMULATE_SCENE),
            vec![vec![0xcafe, 0x6a, 0x5ce]]
        );
        assert_eq!(calls_to(&e, RENDER_TO_TEXTURE_00709D50)[0][0], 0);
    }

    /// An engine for `DoIdle`: the n052 doubles plus its own outside
    /// callees answering 0, the constants it reads, and the singleton on
    /// top of the menus (`GetClass` through a vtable).
    fn idle_engine() -> (Engine, Ptr<CaravanMenu>) {
        let mut e = engine();
        e.map(0x0102_9000, 0x1000);
        e.map(0x0103_1000, 0x1000);
        for address in [
            WORD_AT_0X14,
            CONTROLS_AXIS_00A23390,
            CONTROLS_BUTTON_00A23A50,
            WORD_AT_0X28,
            MODEL_NODE_0043B4A0,
            DEACTIVATE_SEQUENCE,
            SEQUENCE_TIME_00508100,
            PLUS_0X8C,
            WORLD_POINT_TO_SCREEN,
            WORD_AT_0XC,
            WORD_AT_0X10,
            SET_WORD_AT_0XC,
            SET_WORD_AT_0X10,
            INTERFACE_FLOAT_00639AA0,
            INTEGER_MAGNITUDE,
            MENU_START_FADE_OUT,
            MENU_INSTANT_FADE_OUT,
            FLOAT_MAGNITUDE_00408820,
            FLOAT_MAGNITUDE_00408840,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        e.set_global::<f64>(TICKS_PER_SECOND, 1000.0);
        e.set_global::<f64>(STICK_DEAD_ZONE, 7849.0);
        e.set_global::<f64>(STICK_RANGE, 32767.0);
        e.set_global::<f64>(SELECTION_REPEAT, 0.25);
        e.set_global::<f64>(STICK_THRESHOLD, 0.01);
        e.set_global::<f32>(MINUS_ONE_FLOAT, -1.0);
        e.set_global::<f32>(TRACK_ALPHA, 255.0);
        let (this, _) = setup(&mut e);
        let object = object_with_vtable(&mut e);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(this.addr(), vtable);
        e.register(0x00d0_0034, |_, _| ret(0x43b));
        e.register(IS_TOP_MENU_ID, |_, _| ret(1));
        (e, this)
    }

    /// The idle timer at `time` for `state`, with no ticks elapsing (the
    /// tick source answers the menu's previous tick, 1000).
    fn idle_at(e: &mut Engine, this: Ptr<CaravanMenu>, state: i32, time: f32) {
        e.set(this, CaravanMenu::eGameState, state);
        e.set(this, CaravanMenu::uiPrevIdleTick, 1000);
        e.register(WORD_AT_0X14, |_, _| ret(1000));
        e.set_global::<u32>(IDLE_TIME_STATE, state as u32);
        e.set_global::<f32>(IDLE_TIME, time);
    }

    fn do_idle(e: &mut Engine, this: Ptr<CaravanMenu>) {
        e.call_log = Some(vec![]);
        e.call(0x0074_1500, &args![this]);
    }

    #[test]
    fn do_idle_waits_for_the_tutorial_and_off_top_clears_ison_top() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        e.set(this, CaravanMenu::uiPrevIdleTick, 5);
        e.set(this, CaravanMenu::iFlags, CAF_ISONTOP);
        // A tutorial up and not yet shown: nothing runs.
        e.set(this, CaravanMenu::bTutorial, 1);
        e.mem.set_u32(menu + TUTORIAL_INDEX, 0x1f);
        do_idle(&mut e, this);
        assert_eq!(calls_to(&e, TUTORIAL_SHOWN)[0][1], 0x1f);
        assert_eq!(count(&e, WORD_AT_0X14), 0);
        assert_eq!(e.get(this, CaravanMenu::bTutorial), 1);
        // Shown: the tutorial byte clears; off top the menu stops after
        // clearing `CAF_ISONTOP`.
        e.register(TUTORIAL_SHOWN, |_, _| ret(1));
        do_idle(&mut e, this);
        assert_eq!(e.get(this, CaravanMenu::bTutorial), 0);
        e.register(IS_TOP_MENU_ID, |_, _| ret(0));
        e.set(this, CaravanMenu::bTutorial, 0);
        e.set(this, CaravanMenu::uiPrevIdleTick, 5);
        do_idle(&mut e, this);
        assert_eq!(e.get(this, CaravanMenu::iFlags) & CAF_ISONTOP, 0);
        assert_eq!(count(&e, WORD_AT_0X14), 0);
        assert_eq!(e.get(this, CaravanMenu::uiPrevIdleTick), 5);
    }

    #[test]
    fn do_idle_counts_the_timer_and_ends_the_deck_intro() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        e.set(this, CaravanMenu::eActiveMenu, 1);
        idle_at(&mut e, this, 1, 0.5);
        e.register(WORD_AT_0X14, |_, _| ret(1250));
        e.mem.set_u32(menu + CAMERA_ANIM_MANAGER, 0x600);
        e.mem.set_u32(menu + CAMERA_ANIMATIONS, 0x500);
        e.register(SEQUENCE_TIME_00508100, |_, _| ret_f(0.75));
        e.register(DYNAMIC_CAST, |_, _| ret(0x777));
        e.register(TUTORIAL_MENU_CREATE, |_, _| ret(1));
        do_idle(&mut e, this);
        // 0.5 + 250 / 1000.
        assert_eq!(e.global::<f32>(IDLE_TIME), 0.75);
        assert_eq!(e.get(this, CaravanMenu::uiPrevIdleTick), 1250);
        assert_eq!(e.global::<u32>(IDLE_TIME_STATE), 1);
        assert_ne!(e.get(this, CaravanMenu::iFlags) & CAF_ISONTOP, 0);
        // The deck menu resets the interface tile's trait 0xfa9.
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT)[0][1..],
            [0xfa9, 0x437f_0000, 1]
        );
        // Not at 0.0: no activation; the timer reached the sequence's end.
        assert_eq!(count(&e, ACTIVATE_SEQUENCE), 0);
        assert_eq!(
            calls_to(&e, DEACTIVATE_SEQUENCE),
            vec![vec![0x600, 0x500, 0]]
        );
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0);
        assert_eq!(
            calls_to(&e, FORM_BY_EDITOR_ID),
            vec![vec![HELP_DECK_BUILDING]]
        );
        assert_eq!(calls_to(&e, TUTORIAL_MARK)[0][1], 0x1f);
        assert_eq!(e.get(this, CaravanMenu::bTutorial), 1);
        assert_eq!(e.mem.u32(menu + TUTORIAL_INDEX), 0x1f);
        // Before the end: the sequence starts at 0.0 and keeps running.
        e.set(this, CaravanMenu::bTutorial, 0);
        idle_at(&mut e, this, 1, 0.0);
        e.register(SEQUENCE_TIME_00508100, |_, _| ret_f(0.75));
        do_idle(&mut e, this);
        assert_eq!(
            calls_to(&e, ACTIVATE_SEQUENCE),
            vec![vec![0x600, 0x500, 0, 0, 0x3f80_0000, 0, 0]]
        );
        assert_eq!(count(&e, DEACTIVATE_SEQUENCE), 0);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 1);
    }

    #[test]
    fn do_idle_money_drop_updates_the_new_money_and_ends_past_a_second() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        idle_at(&mut e, this, 4, 1.5);
        fill_array(&mut e, menu + LIST_OF_BILLS, &[0xb0, 0xb1, 0xb2]);
        fill_array(&mut e, menu + LIST_OF_COINS, &[0xc0, 0xc1]);
        e.mem.set_u32(menu + BILL_INDEX_TO_ANIM, 1);
        e.mem.set_u32(menu + COIN_INDEX_TO_ANIM, 2);
        do_idle(&mut e, this);
        let updated: Vec<u32> = calls_to(&e, OBJECT_UPDATE).iter().map(|c| c[0]).collect();
        assert_eq!(updated, vec![0xb1, 0xb2]);
        assert_eq!(calls_to(&e, UPDATE_DATA_INIT)[0][1..], [0x3fc0_0000, 1, 0]);
        assert_eq!(e.mem.u32(menu + BILL_INDEX_TO_ANIM), 3);
        assert_eq!(e.mem.u32(menu + COIN_INDEX_TO_ANIM), 2);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0);
    }

    #[test]
    fn do_idle_deck_meter_release_moves_by_the_difference() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        e.set(this, CaravanMenu::eActiveMenu, 1);
        idle_at(&mut e, this, 0, 0.0);
        e.mem.set_u8(menu + BYTE_E85, 1);
        e.set(this, CaravanMenu::iPrevValue, 4);
        e.register(GET_FLOAT_TRAIT, |_, _| ret_f(7.0));
        let deck: Vec<u32> = (0..13).map(|_| card(&mut e, 1, 1, 0)).collect();
        fill_array(&mut e, menu + DECK_ARRAY, &deck);
        do_idle(&mut e, this);
        assert_eq!(e.get(this, CaravanMenu::iNumSteps), 3);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 5);
        assert_eq!(e.get(this, CaravanMenu::iPrevValue), 7);
        assert_eq!(e.mem.u8(menu + BYTE_E85), 0);
        // Five or more steps back: state 9.
        idle_at(&mut e, this, 0, 0.0);
        e.mem.set_u8(menu + BYTE_E85, 1);
        e.set(this, CaravanMenu::iPrevValue, 12);
        do_idle(&mut e, this);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 9);
        assert_eq!(e.get(this, CaravanMenu::iPrevValue), 7);
    }

    #[test]
    fn do_idle_game_keys_move_the_hand_card_selection() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        e.set(this, CaravanMenu::eActiveMenu, 2);
        idle_at(&mut e, this, 0, 1.0);
        e.mem.set_u32(menu + SELECTION_DELAY, 0.5f32.to_bits());
        e.mem.set_u32(menu + PENDING_KEY, KEY_RIGHT);
        e.register(FLOAT_MAGNITUDE_00408840, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).abs()))
        });
        let cards: Vec<u32> = (0..3).map(|_| card(&mut e, 2, 1, 0)).collect();
        fill_array(&mut e, menu + PLAYER_HAND, &cards);
        e.mem.set_u32(menu + 0xe2c, 2);
        let object = e.mem.alloc(0x100);
        e.register_double(OBJECT_BY_NAME, move |_, _| ret(object));
        e.set(this, CaravanMenu::iFlags, CAF_DISABLE_A);
        do_idle(&mut e, this);
        assert_eq!(e.mem.u32(menu + PENDING_KEY), 0);
        assert_eq!(e.mem.u32(menu + SELECTION_DELAY), 1.0f32.to_bits());
        // 2 + 1 is past the last card: back to the first.
        assert_eq!(e.mem.u32(menu + 0xe2c), 0);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], SWITCH_CARD_SOUND);
        assert_eq!(e.get(this, CaravanMenu::iFlags) & CAF_DISABLE_A, 0);
        // Within the delay nothing moves.
        idle_at(&mut e, this, 0, 1.1);
        e.mem.set_u32(menu + PENDING_KEY, KEY_LEFT);
        do_idle(&mut e, this);
        assert_eq!(e.mem.u32(menu + 0xe2c), 0);
        assert_eq!(count(&e, SOUND_HANDLE_BY_NAME), 0);
    }

    #[test]
    fn do_idle_next_joker_selects_the_next_track_with_jokers() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        idle_at(&mut e, this, 0x13, 0.0);
        e.mem.set_u32(menu + 0xe30, 1);
        fill_array(&mut e, menu + JOKER_LOCATIONS + 0x20, &[3]);
        do_idle(&mut e, this);
        assert_eq!(e.mem.u32(menu + 0xe30), 2);
        assert_eq!(e.mem.u32(menu + 0xe34), 3);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x11);
        assert_ne!(e.get(this, CaravanMenu::iFlags) & CAF_JOKER, 0);
        // No track left with jokers: the turn ends.
        idle_at(&mut e, this, 0x13, 0.0);
        e.mem.set_u32(menu + 0xe30, 3);
        do_idle(&mut e, this);
        assert_eq!(e.mem.u32(menu + 0xe30), 6);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x15);
        assert_eq!(e.get(this, CaravanMenu::iFlags) & CAF_JOKER, 0);
    }

    #[test]
    fn do_idle_navigates_the_deck_step_by_step() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        idle_at(&mut e, this, 5, 0.0);
        e.mem.set_u32(menu + ACTIVE_DECK_ANIM_MANAGER, 0x610);
        e.mem.set_u32(menu + ACTIVE_DECK_ANIMATIONS, 0x510);
        e.mem.set_u32(menu + INACTIVE_DECK_ANIM_MANAGER, 0x620);
        e.mem.set_u32(menu + INACTIVE_DECK_ANIMATIONS, 0x520);
        e.mem.set_u32(menu + PENDING_KEY, KEY_UP);
        e.set(this, CaravanMenu::iNumSteps, 3);
        e.register(GET_FLOAT_TRAIT, |_, _| ret_f(9.0));
        e.register(SEQUENCE_TIME_00508100, |_, _| ret_f(0.0));
        do_idle(&mut e, this);
        // Started at 0.0 and already at the end: one step taken, the
        // timer restarts for the next.
        let activated: Vec<u32> = calls_to(&e, ACTIVATE_SEQUENCE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(activated, vec![0x510, 0x520]);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], NAVIGATE_SOUND);
        assert_eq!(e.get(this, CaravanMenu::iNumSteps), 1);
        assert_eq!(e.global::<u32>(IDLE_TIME), 0);
        assert_eq!(e.mem.u32(menu + PENDING_KEY), 0);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 5);
        assert_eq!(count(&e, DEACTIVATE_SEQUENCE), 0);
        // The last step stops the sequences.
        idle_at(&mut e, this, 5, 0.0);
        do_idle(&mut e, this);
        assert_eq!(e.get(this, CaravanMenu::iNumSteps), 0);
        let stopped: Vec<u32> = calls_to(&e, DEACTIVATE_SEQUENCE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(stopped, vec![0x510, 0x520]);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0);
    }

    #[test]
    fn do_idle_turn_change_and_closing() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        // Both sides out of cards: the game is over.
        idle_at(&mut e, this, 0x15, 0.0);
        do_idle(&mut e, this);
        assert_ne!(e.get(this, CaravanMenu::iFlags) & CAF_AI, 0);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x16);
        // Closing: the fade starts, instantly when it was started before.
        idle_at(&mut e, this, 0x17, 0.0);
        e.set_global::<u8>(CLOSE_FADED_BYTE, 1);
        do_idle(&mut e, this);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        assert_eq!(calls_to(&e, MENU_INSTANT_FADE_OUT), vec![vec![menu]]);
        assert_eq!(e.global::<u32>(CARAVAN_MENU), 0);
        assert_eq!(e.global::<u8>(CLOSE_FADED_BYTE), 1);
        // A fade state of 2 does nothing.
        e.register(WORD_AT_0X24, |_, _| ret(2));
        do_idle(&mut e, this);
        assert_eq!(count(&e, MENU_START_FADE_OUT), 0);
    }

    #[test]
    fn do_idle_discards_a_track_column_by_column() {
        let (mut e, this) = idle_engine();
        let menu = this.addr();
        idle_at(&mut e, this, 0xf, 0.0);
        e.mem.set_u32(menu + 0xe30, 1);
        for row in 0..2 {
            let cards = [card(&mut e, 3, 1, 0), card(&mut e, 4, 1, 0)];
            fill_array(&mut e, card_track(menu, 1, row), &cards);
            e.mem
                .set_u32(menu + TRACK_GEOMETRY + 0x1c + 4 * row, 0x900 + row);
            e.mem
                .set_u32(menu + TRACK_ANIM_MANAGER + 0x1c + 4 * row, 0x800 + row);
            for column in 0..4 {
                for which in 0..2 {
                    let sequence = 0x1000 + 0x100 * row + 0x10 * column + which;
                    e.mem
                        .set_u32(track_animation(menu, 1, row, column, which), sequence);
                }
            }
        }
        e.register(SEQUENCE_TIME_00508100, |_, _| ret_f(-1.0));
        do_idle(&mut e, this);
        // From column 4 down, the first column with cards (1) starts on both
        // rows; past its end it stops and the timer restarts.
        let activated: Vec<u32> = calls_to(&e, ACTIVATE_SEQUENCE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(activated, vec![0x1011, 0x1111]);
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE)[0][0], 0x800);
        let updated: Vec<u32> = calls_to(&e, OBJECT_UPDATE).iter().map(|c| c[0]).collect();
        assert_eq!(updated, vec![0x900, 0x901]);
        assert_eq!(count(&e, DEACTIVATE_SEQUENCE), 7);
        assert_eq!(calls_to(&e, DEACTIVATE_SEQUENCE)[0][1], 0x1011);
        assert_eq!(e.mem.u32(menu + 0xe38), 1);
        assert_eq!(e.global::<u32>(IDLE_TIME), 0);
        assert_eq!(calls_to(&e, SOUND_HANDLE_BY_NAME)[0][2], REMOVE_CARD_SOUND);
        // A later frame past the column's end: the next column (0) starts.
        idle_at(&mut e, this, 0xf, 0.5);
        do_idle(&mut e, this);
        let activated: Vec<u32> = calls_to(&e, ACTIVATE_SEQUENCE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(activated, vec![0x1001, 0x1101]);
        assert_eq!(e.mem.u32(menu + 0xe38), 0);
        assert_eq!(count(&e, ARRAY_RELEASE_008454F0), 0);
        // The last column ends: the lists are released, the turn ends.
        idle_at(&mut e, this, 0xf, 0.5);
        e.set(
            this,
            CaravanMenu::iFlags,
            CAF_DESTROY_TRACK | CAF_SELECT_TRACK,
        );
        do_idle(&mut e, this);
        assert_eq!(count(&e, ACTIVATE_SEQUENCE), 0);
        let released: Vec<u32> = calls_to(&e, ARRAY_RELEASE_008454F0)
            .iter()
            .map(|c| c[0])
            .collect();
        let expected: Vec<u32> = (0..7).map(|row| card_track(menu, 1, row)).collect();
        assert_eq!(released, expected);
        assert_eq!(e.get(this, CaravanMenu::eGameState), 0x15);
        assert_eq!(
            e.get(this, CaravanMenu::iFlags) & (CAF_DESTROY_TRACK | CAF_SELECT_TRACK),
            0
        );
    }
}
