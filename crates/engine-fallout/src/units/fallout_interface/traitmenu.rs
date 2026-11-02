//! `fallout/interface/menus/traitmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TraitMenu` (Xbox PDB, 0x8C bytes) is the menu that picks the player's
//! traits: nine tiles (`pTiles`, +0x28), the points picked and allowed
//! (`iCurrPoints` +0x4C, `iMaxPoints` +0x50), the perk list box
//! (`lbPerksList`, `ListBox<BGSPerk *>`, +0x54) and the perks offered
//! (`AvailablePerks`, `BSSimpleList<BGSPerk *>`, +0x84). `pMe` (`011daf74`)
//! is the open menu. The `Menu` base fields (`pMenu` +4, `xFadeState`
//! +0x24) are read at their offsets.
//!
//! Notes that apply to every function:
//! - The user trait ids the menu reads (`011daf5c`, `011daf64`, `011daf68`,
//!   `011daf70`, `011daf78`) are the class's `iUT_*` statics (Xbox PDB lists
//!   eight); which id is which name is not confirmed, so they are named by
//!   address.
//! - The player is the `PlayerCharacter` at `011dea3c`; its vtable slots
//!   +0x4A0 (a perk's rank, `(perk, 0)`) and +0x498 (`(perk, rank, 0)`) are
//!   called by offset.
//! - `BSSimpleList` nodes are walked as the code walks them: `006815c0`
//!   answers the node (its item is the word at +0), `00726070` the next node
//!   (the word at +4).
//! - The compiler's stack cookie check (`00ec408c`) and exception frames
//!   (`FS:[0]`) are not translated; the functions that have them carry a
//!   `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TraitMenu`'s class id (`GetClass`, `007e6dc0`).
const TRAIT_MENU_CLASS: u32 = 0x43c;
/// `TraitMenu::pMe` (Xbox PDB static): the open menu.
const TRAIT_MENU_ME: u32 = 0x011d_af74;
/// `TraitMenu`'s vtable (RTTI `.?AVTraitMenu@@`).
const TRAIT_MENU_VTABLE: u32 = 0x0107_79bc;
/// The player (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;
/// User trait ids (`iUT_*` statics of the class; which is which is not confirmed).
const USER_TRAIT_011DAF5C: u32 = 0x011d_af5c;
const USER_TRAIT_011DAF64: u32 = 0x011d_af64;
const USER_TRAIT_011DAF68: u32 = 0x011d_af68;
const USER_TRAIT_011DAF70: u32 = 0x011d_af70;
const USER_TRAIT_011DAF78: u32 = 0x011d_af78;

/// The player's vtable slot that answers a perk's rank (`(perk, 0)`, a byte).
const PLAYER_SLOT_PERK_RANK: u32 = 0x4a0;
/// The player's vtable slot called with `(perk, rank, 0)`.
const PLAYER_SLOT_SET_PERK_RANK: u32 = 0x498;

// Offsets in `TraitMenu` (Xbox PDB names, PC offsets as the code uses them).
/// `Menu::xFadeState` (Xbox PDB).
const FADE_STATE: u32 = 0x24;
/// `pTiles` (Xbox PDB): nine tiles.
const TILES: u32 = 0x28;
/// `iCurrPoints` (Xbox PDB).
const CURR_POINTS: u32 = 0x4c;
/// `iMaxPoints` (Xbox PDB).
const MAX_POINTS: u32 = 0x50;
/// `lbPerksList` (Xbox PDB): `ListBox<BGSPerk *>`.
const PERKS_LIST: u32 = 0x54;
/// `AvailablePerks` (Xbox PDB): `BSSimpleList<BGSPerk *>` (item at +0, next at +4).
const AVAILABLE_PERKS: u32 = 0x84;

// Callees outside this unit.
/// the allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// thiscall on a game setting: its string value.
const SETTING_STRING: u32 = 0x0040_3df0;
/// cdecl (buffer, size, format, ...): formatted print into the buffer.
const PRINT_FORMATTED: u32 = 0x0040_6d00;
/// cdecl (buffer, size, text): string copy into the buffer.
const COPY_STRING: u32 = 0x0040_6d30;
/// thiscall on a setting object: the address of its value.
const SETTING_VALUE: u32 = 0x0040_8d60;
/// cdecl (left, right): string compare.
const COMPARE_STRINGS: u32 = 0x0040_8b20;
/// thiscall (no words): the text of a string member (`MapMarkerData::GetLocationName` in the engine map; folded code).
const STRING_C_STR: u32 = 0x0040_8da0;
/// `EffectItemData` (0x14 bytes) constructor, thiscall.
const EFFECT_ITEM_DATA_CONSTRUCT: u32 = 0x0040_3ba0;
/// thiscall on an `EffectItem`: its effect code (-1 for none).
const EFFECT_ITEM_CODE: u32 = 0x0040_3ea0;
/// `EffectItem::GetMagnitude` (engine map), thiscall.
const EFFECT_ITEM_GET_MAGNITUDE: u32 = 0x0040_4390;
/// thiscall on a `BSSimpleList` node (no words): answers the node itself.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// thiscall on a `BSSimpleList` node: the next node (word at +4); on a
/// menu it reads `pMenu` (+4) the same way.
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall (setting object): a pointer to the trait-point count.
const TRAIT_POINTS_VALUE: u32 = 0x0043_d4d0;
/// The setting object `0043d4d0` reads.
const TRAIT_POINTS_OBJECT: u32 = 0x011d_2f94;
/// cdecl (text): the string's length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// thiscall on a perk entry: the object whose effect list (+0x28) is applied.
const PERK_ENTRY_SPELL: u32 = 0x0044_ddc0;
/// thiscall on the player: answers the object `008d51f0` reads.
const PLAYER_PROGRESSION: u32 = 0x0046_4e10;
/// `BSSimpleList` destructor (thiscall).
const SIMPLE_LIST_DESTROY: u32 = 0x0046_ffb0;
/// thiscall on the object at `011c3f2c`: its list at +0x178.
const LIST_AT_178: u32 = 0x0046_0fb0;
/// The global whose word `00460fb0` is called on.
const LIST_OWNER_011C3F2C: u32 = 0x011c_3f2c;
/// cdecl (), the object at `011d8a80`.
const INTERFACE_OBJECT: u32 = 0x004b_7210;
/// thiscall (no words): the tile reader at +0x9c of that object.
const TILE_READER: u32 = 0x0058_6150;
/// cdecl (text): prints a message.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// thiscall on a perk: its number of ranks (byte).
const PERK_NUM_RANKS: u32 = 0x005d_4ee0;
/// thiscall on a perk: a check of the list filter.
const PERK_CHECK_5EB5E0: u32 = 0x005e_b5e0;
/// thiscall on a perk: a check of the list filter.
const PERK_CHECK_7A9280: u32 = 0x007a_9280;
/// thiscall (perk, rank): the perk's entry for that rank.
const PERK_ENTRY_FOR_RANK: u32 = 0x005e_b900;
/// `BGSPerk::IsPerkAttainable` (engine map), thiscall (perk, player).
const PERK_IS_ATTAINABLE: u32 = 0x005e_ba00;
/// `BGSPerk::BuildDescriptionString` (engine map), thiscall (perk, buffer, size).
const PERK_BUILD_DESCRIPTION: u32 = 0x005e_bac0;
/// thiscall on a perk: its level (signed byte).
const PERK_LEVEL: u32 = 0x006a_abf0;
/// thiscall (tile, trait, value): sets an integer trait value.
const TILE_SET_VALUE: u32 = 0x0070_0320;
/// `Interface::PlayMenuSound` (engine map), cdecl (sound).
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// thiscall (list box, callback, 0, 0x7fffffff): runs the callback over the items.
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// `ListBox<unsigned int>::Initialize` (engine map, folded), thiscall (list box, tile, name).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// `HUDMainMenu::ClearXPMeterFinishedMoving` (engine map), cdecl.
const HUD_CLEAR_XP_METER: u32 = 0x0077_da30;
/// thiscall (perk, player): whether the perk's requirements are met.
const PERK_REQUIREMENTS_MET: u32 = 0x0078_5150;
/// `ListBox<BGSPerk *>` destructor (thiscall).
const LIST_BOX_DESTROY: u32 = 0x0078_6c80;
/// `ListBox<BGSPerk *>::AddItem` (engine map), thiscall (list box, perk, text, 0, 0): the item's tile.
const LIST_BOX_ADD_ITEM: u32 = 0x0078_6cf0;
/// thiscall (owner, code).
const EFFECT_CODE_CHECK: u32 = 0x0078_69e0;
/// `ListBox<BGSPerk *>::RestoreListState` (engine map), thiscall (list box, 1).
const LIST_BOX_RESTORE_STATE: u32 = 0x0078_7600;
/// thiscall (list box, tile).
const LIST_BOX_SELECT: u32 = 0x0078_7760;
/// thiscall (list box, tile).
const LIST_BOX_DESELECT: u32 = 0x0078_7780;
/// thiscall (list box, tile): whether the tile is selected.
const LIST_BOX_IS_SELECTED: u32 = 0x0078_77a0;
/// thiscall (list box): the number of selected items.
const LIST_BOX_SELECTED_COUNT: u32 = 0x0078_78d0;
/// thiscall (array, item): appends an `EffectItemData` copy.
const EFFECT_ARRAY_ADD: u32 = 0x0078_7950;
/// `StatsMenu::UpdatePlayerName` (engine map), cdecl.
const STATS_UPDATE_PLAYER_NAME: u32 = 0x007d_f4e0;
/// `StatsMenu::UpdatePlayerTitle` (engine map), cdecl.
const STATS_UPDATE_PLAYER_TITLE: u32 = 0x007d_f5d0;
/// thiscall (list box, tile): the item (perk) of a tile.
const LIST_BOX_ITEM_OF_TILE: u32 = 0x007e_1110;
/// thiscall: the word at +0x14 of an effect item.
const EFFECT_ITEM_FIELD_14: u32 = 0x0082_5c00;
/// thiscall (array, flag): empties the effect array.
const EFFECT_ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `TraitMenu::PerkEffectList` (Xbox PDB static, `BSSimpleArray<EffectItemData,1024>`).
const PERK_EFFECT_LIST: u32 = 0x011d_aff0;
/// thiscall on the player: its level (16 bits).
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// `Actor::UpdateMagic` (engine map), thiscall (actor, 1, 0).
const ACTOR_UPDATE_MAGIC: u32 = 0x008c_3c40;
/// `CharacterProgression::QReadyToLevelUp` (engine map), thiscall.
const READY_TO_LEVEL_UP: u32 = 0x008d_51f0;
/// thiscall (list, &item): adds the item to a `BSSimpleList`.
const SIMPLE_LIST_ADD: u32 = 0x0090_5820;
/// thiscall (tile, trait): the trait's float value in ST0.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::GetString` (engine map), thiscall (tile, trait).
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map), thiscall (tile, trait).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::Poke` (engine map), thiscall (tile, trait, float).
const TILE_POKE: u32 = 0x00a0_1290;
/// thiscall (tile, trait, float, flag): sets a float trait value.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// thiscall (tile, trait, text, flag): sets a string trait value.
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::GetMaximumDepth` (engine map), cdecl: a float in ST0.
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Tile::TextToTrait` (engine map), cdecl (name).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::ReadFile` (engine map), thiscall (reader, path): the root tile.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu` (engine map), thiscall.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuByClass` (engine map), cdecl (class).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Menu` base destructor, thiscall.
const MENU_DESTROY: u32 = 0x00a1_c520;
/// `Menu::StartFadeOut` (engine map), thiscall.
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `Menu::PrepForVisibility` (engine map), thiscall (menu, 0).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `Menu::SetMenuTile` (engine map), thiscall (menu, tile, 0).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// thiscall on the object at `011f2250`: a flag `DoIdle` checks.
const IDLE_FLAG_CHECK: u32 = 0x0052_5430;
const IDLE_FLAG_OBJECT: u32 = 0x011f_2250;

// Data.
/// `"Data\Menus\trait_menu.xml"`.
const MENU_FILE: u32 = 0x0107_7a5c;
/// `"MENUS: Trait Menu Creation Failed... Are your menu and art resources up to date?"`.
const CREATION_FAILED_MESSAGE: u32 = 0x0107_7a08;
/// The two `double`s the root's trait 0xfa7 is compared with (6006.0 and 102.0 in the C).
const ROOT_VALUE_A: u32 = 0x0107_04f0;
const ROOT_VALUE_B: u32 = 0x0107_04e8;
/// The settings whose strings format the points line (one point / several).
const POINTS_FORMAT_ONE: u32 = 0x011d_351c;
const POINTS_FORMAT_MANY: u32 = 0x011d_2310;
/// The settings whose strings go into the tiles at +0x3c and +0x40.
const TEXT_FOR_TILE_3C: u32 = 0x011d_33d8;
const TEXT_FOR_TILE_40: u32 = 0x011d_35c4;
/// `"_Title"`.
const TITLE_TRAIT_NAME: u32 = 0x0106_ed20;
/// The setting `00408d60` reads in the list filter.
const FILTER_SETTING: u32 = 0x011d_a034;
/// The list box name `0075a0d0` gets.
const LIST_BOX_NAME: u32 = 0x0107_3db8;
/// The setting used when a perk has no name.
const NO_NAME_TEXT: u32 = 0x011d_35b8;
/// The format for a perk with several ranks (read from the exe).
const RANKED_NAME_FORMAT: u32 = 0x0106_eb00;
/// The settings formatting the point counter (one point left / several).
const COUNTER_FORMAT_ONE: u32 = 0x011d_4558;
const COUNTER_FORMAT_MANY: u32 = 0x011d_2c10;
/// The `double` `DoClick` compares a tile's trait value with.
const CLICK_VALUE: u32 = 0x0101_e568;
/// The floats `007e7670` sets: available, unavailable.
const AVAILABLE_VALUE: u32 = 0x0102_3cd8;
const UNAVAILABLE_VALUE: u32 = 0x0101_e704;
/// Trait names `DoEnter` sets and pokes.
const ENTER_TRAIT_A: u32 = 0x0107_02d8;
const ENTER_TRAIT_B: u32 = 0x0107_0250;

/// The callbacks the list box gets (functions of this unit, passed by address).
const CALLBACK_SET_VALUE: u32 = 0x007e_7670;
const CALLBACK_APPLY_SELECTED: u32 = 0x007e_76e0;
const CALLBACK_COMPARE: u32 = 0x007e_7740;
/// A callback `DoClick` passes (`007e7630`, not a function of this list).
const CALLBACK_RESET: u32 = 0x007e_7630;

fn tile_at(e: &Engine, this: u32, index: u32) -> u32 {
    e.mem.u32(this + TILES + 4 * index)
}

// Translated from 007e68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::GetClass` (Xbox PDB): the menu class id, `0x43c`.
pub fn trait_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    TRAIT_MENU_CLASS
}

// Translated from 007e68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::AttachTileByID` (Xbox PDB): stores the tile in `pTiles`
/// (+0x28) for ids 0 to 8 (signed compare); other ids do nothing.
pub fn trait_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..9).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + 4 * id as u32, tile);
    }
}

// Translated from 007e68e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`007e6910`) and, when bit 0 of the flags is set, frees the
/// object. Answers `this`.
pub fn trait_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007e6910(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007e6910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e6910 - the C++ exception-unwinding frame (FS:[0]) that destroys the members when a destructor throws is not translated
/// `TraitMenu::~TraitMenu` (unnamed in the engine map; the body is the
/// destructor): sets the vtable, clears `pMe`, destroys `AvailablePerks`
/// (+0x84), the list box (+0x54) and the `Menu` base.
pub fn fn_007e6910(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, TRAIT_MENU_VTABLE);
    e.set_global(TRAIT_MENU_ME, 0u32);
    e.call(SIMPLE_LIST_DESTROY, &args![base + AVAILABLE_PERKS]);
    e.call(LIST_BOX_DESTROY, &args![base + PERKS_LIST]);
    e.call(MENU_DESTROY, &args![base]);
}

// Translated from 007e6990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e6990 - the /GS stack cookie check (00ec408c) is not translated
/// `TraitMenu::Create` (Xbox PDB), `cdecl`: deletes an open trait menu
/// (vtable slot 0 with 1), reads `Data\Menus\trait_menu.xml` and makes the
/// menu `pMe`. When the new menu's class is the trait menu's, it takes the
/// tile, checks all nine tiles are attached (otherwise prints the creation
/// failure and answers 0), resizes the root when its trait 0xfa7 is one of
/// the two values at `010704f0`/`010704e8`, sets the points (0 of the
/// count at `011d2f94`) and the captions, fills `AvailablePerks` with the
/// perks of the list at +0x178 of the object at `011c3f2c` that pass the
/// filter, fills the list box, shows the counters, hooks the callbacks and
/// restores the list state; answers the root tile. Otherwise deletes a
/// menu that has a tile and answers 0.
pub fn trait_menu_create(e: &mut Engine) -> u32 {
    let existing = e
        .call(TILE_GET_MENU_BY_CLASS, &args![TRAIT_MENU_CLASS])
        .u32();
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(TRAIT_MENU_ME, menu);
    if menu != 0 {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        if class == fn_007e6dc0(e) {
            return create_contents(e, root);
        }
    }
    let menu: u32 = e.global(TRAIT_MENU_ME);
    if menu != 0 && e.call(LIST_NODE_NEXT, &args![menu]).u32() != 0 {
        let menu: u32 = e.global(TRAIT_MENU_ME);
        if menu != 0 {
            e.vcall(menu, 0, &args![1u32]);
        }
    }
    0
}

/// The part of [`trait_menu_create`] once the menu is the trait menu.
fn create_contents(e: &mut Engine, root: u32) -> u32 {
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    if !fn_007e6dd0(e, Ptr::new(menu)) {
        e.call(PRINT_MESSAGE, &args![CREATION_FAILED_MESSAGE]);
        return 0;
    }
    let first = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
    let resize = first == e.global::<f64>(ROOT_VALUE_A) || {
        let second = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
        second == e.global::<f64>(ROOT_VALUE_B)
    };
    if resize {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![root, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    let points_at = e
        .call(TRAIT_POINTS_VALUE, &args![TRAIT_POINTS_OBJECT])
        .u32();
    let points = e.mem.u32(points_at);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.mem.set_u32(menu + CURR_POINTS, 0);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.mem.set_u32(menu + MAX_POINTS, points);
    e.with_stack(0x40, |e, buffer| {
        let format_setting = if points == 1 {
            POINTS_FORMAT_ONE
        } else {
            POINTS_FORMAT_MANY
        };
        let format = e.call(SETTING_STRING, &args![format_setting]).u32();
        e.call(PRINT_FORMATTED, &args![buffer, 0x40u32, format, points]);
        let menu: u32 = e.global(TRAIT_MENU_ME);
        let tile = tile_at(e, menu, 0);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
    });
    let text = e.call(SETTING_STRING, &args![TEXT_FOR_TILE_3C]).u32();
    let menu: u32 = e.global(TRAIT_MENU_ME);
    let tile = tile_at(e, menu, 5);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    let text = e.call(SETTING_STRING, &args![TEXT_FOR_TILE_40]).u32();
    let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![TITLE_TRAIT_NAME]).u32();
    let menu: u32 = e.global(TRAIT_MENU_ME);
    let tile = tile_at(e, menu, 6);
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);

    let owner: u32 = e.global(LIST_OWNER_011C3F2C);
    let mut node = e.call(LIST_AT_178, &args![owner]).u32();
    e.with_stack(4, |e, perk_slot| {
        while node != 0 {
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let perk = e.mem.u32(item);
            e.mem.set_u32(perk_slot.addr(), perk);
            if offer_perk(e, perk) {
                let menu: u32 = e.global(TRAIT_MENU_ME);
                e.call(SIMPLE_LIST_ADD, &args![menu + AVAILABLE_PERKS, perk_slot]);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    });

    let menu: u32 = e.global(TRAIT_MENU_ME);
    fn_007e7030(e, Ptr::new(menu));
    for (field, trait_global) in [
        (CURR_POINTS, USER_TRAIT_011DAF64),
        (MAX_POINTS, USER_TRAIT_011DAF68),
    ] {
        let menu: u32 = e.global(TRAIT_MENU_ME);
        let value = e.mem.u32(menu + field);
        let trait_id: u32 = e.global(trait_global);
        let tile = e.call(LIST_NODE_NEXT, &args![menu]).u32();
        e.call(TILE_SET_VALUE, &args![tile, trait_id, value]);
    }
    let menu: u32 = e.global(TRAIT_MENU_ME);
    trait_menu_update_point_counter(e, Ptr::new(menu));
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.call(
        LIST_BOX_FOR_EACH,
        &args![menu + PERKS_LIST, CALLBACK_SET_VALUE, 0u32, 0x7fff_ffffu32],
    );
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.vcall(menu + PERKS_LIST, 0x20, &args![CALLBACK_COMPARE]);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.call(LIST_BOX_RESTORE_STATE, &args![menu + PERKS_LIST, 1u32]);
    root
}

/// The filter of [`trait_menu_create`]'s perk loop: the perk's level (a
/// signed byte) is above 0, the player's rank in it is below its number of
/// ranks, the three checks pass, and, when the setting at `011da034` is
/// on, its requirements are met and its level is at most the player's.
fn offer_perk(e: &mut Engine, perk: u32) -> bool {
    if e.call(PERK_LEVEL, &args![perk]).u8() as i8 <= 0 {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    let rank = e
        .vcall(player, PLAYER_SLOT_PERK_RANK, &args![perk, 0u32])
        .u8();
    let ranks = e.call(PERK_NUM_RANKS, &args![perk]).u8();
    if rank >= ranks {
        return false;
    }
    if !e.call(PERK_CHECK_7A9280, &args![perk]).bool() {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    if !e.call(PERK_IS_ATTAINABLE, &args![perk, player]).bool() {
        return false;
    }
    if !e.call(PERK_CHECK_5EB5E0, &args![perk]).bool() {
        return false;
    }
    let setting = e.call(SETTING_VALUE, &args![FILTER_SETTING]).u32();
    if e.mem.u8(setting) == 0 {
        return true;
    }
    let player: u32 = e.global(PLAYER);
    if !e.call(PERK_REQUIREMENTS_MET, &args![perk, player]).bool() {
        return false;
    }
    let level = i32::from(e.call(PERK_LEVEL, &args![perk]).u8() as i8);
    let player: u32 = e.global(PLAYER);
    let player_level = i32::from(e.call(PLAYER_LEVEL, &args![player]).u16());
    level <= player_level
}

// Translated from 007e6dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trait menu's class id, `0x43c`, as a function without `this` (the
/// static counterpart of `GetClass`, unnamed in the engine map).
pub fn fn_007e6dc0(_e: &mut Engine) -> u32 {
    TRAIT_MENU_CLASS
}

// Translated from 007e6dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all nine tiles of `pTiles` (+0x28) are attached (unnamed in the
/// engine map); stops at the first missing one.
pub fn fn_007e6dd0(e: &mut Engine, this: Ptr) -> bool {
    (0..9).all(|index| tile_at(e, this.addr(), index) != 0)
}

// Translated from 007e6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::Close` (Xbox PDB), static: clears the XP meter's state
/// (`0077da30`) and, when the menu is open, applies the selected perks
/// (the list box callback `007e76e0`), updates the player's magic when the
/// player is ready to level up, refreshes the stats menu's name and title,
/// sets trait 0x1772 of the menu tile to 1, starts the fade-out and clears
/// `pMe`.
pub fn trait_menu_close(e: &mut Engine) {
    e.call(HUD_CLEAR_XP_METER, &args![]);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    if menu == 0 {
        return;
    }
    e.call(
        LIST_BOX_FOR_EACH,
        &args![
            menu + PERKS_LIST,
            CALLBACK_APPLY_SELECTED,
            0u32,
            0x7fff_ffffu32
        ],
    );
    let player: u32 = e.global(PLAYER);
    let progression = e.call(PLAYER_PROGRESSION, &args![player]).u32();
    if e.call(READY_TO_LEVEL_UP, &args![progression]).bool() {
        let player: u32 = e.global(PLAYER);
        e.call(ACTOR_UPDATE_MAGIC, &args![player, 1u32, 0u32]);
    }
    e.call(STATS_UPDATE_PLAYER_NAME, &args![]);
    e.call(STATS_UPDATE_PLAYER_TITLE, &args![]);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    let tile = e.call(LIST_NODE_NEXT, &args![menu]).u32();
    e.call(TILE_SET_VALUE, &args![tile, 0x1772u32, 1u32]);
    let menu: u32 = e.global(TRAIT_MENU_ME);
    e.call(MENU_START_FADE_OUT, &args![menu]);
    e.set_global(TRAIT_MENU_ME, 0u32);
}

// Translated from 007e6ea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e6ea0 - the /GS stack cookie check (00ec408c) is not translated
/// `TraitMenu::DoEnter` (Xbox PDB): for id 10 (a list item), shows the
/// item's perk: its name (the string at +0x2c of the perk) in tile +0x30's
/// trait 0xfcc, its description (built into a 0x200-byte buffer) in tile
/// +0x34's trait 0xfc4, makes tile +0x30 visible (trait 0xfa3) when the
/// name is longer than one character, shows tile +0x34, hides tile +0x44,
/// plays menu sound 4 and resets two traits of tile +0x48 (set to 0.0,
/// poked with 1.0). Other ids do nothing.
pub fn trait_menu_do_enter(e: &mut Engine, this: Ptr, id: u32, tile: u32) {
    if id != 0xa {
        return;
    }
    let base = this.addr();
    let perk = e
        .call(LIST_BOX_ITEM_OF_TILE, &args![base + PERKS_LIST, tile])
        .u32();
    let name = e.call(STRING_C_STR, &args![perk.wrapping_add(0x2c)]).u32();
    let name_tile = tile_at(e, base, 2);
    e.call(TILE_SET_STRING, &args![name_tile, 0xfccu32, name, 1u32]);
    e.with_stack(0x200, |e, buffer| {
        let perk = e
            .call(LIST_BOX_ITEM_OF_TILE, &args![base + PERKS_LIST, tile])
            .u32();
        e.call(PERK_BUILD_DESCRIPTION, &args![perk, buffer, 0x200u32]);
        let text_tile = tile_at(e, base, 3);
        e.call(TILE_SET_STRING, &args![text_tile, 0xfc4u32, buffer, 1u32]);
    });
    let name_tile = tile_at(e, base, 2);
    let text = e.call(TILE_GET_STRING, &args![name_tile, 0xfccu32]).u32();
    let length = e.call(STRING_LENGTH, &args![text]).u32();
    let name_tile = tile_at(e, base, 2);
    let visible = u32::from(length > 1);
    e.call(TILE_SET_VALUE, &args![name_tile, 0xfa3u32, visible]);
    let text_tile = tile_at(e, base, 3);
    e.call(TILE_SET_VALUE, &args![text_tile, 0xfa3u32, 1u32]);
    let hidden_tile = tile_at(e, base, 7);
    e.call(TILE_SET_VALUE, &args![hidden_tile, 0xfa3u32, 0u32]);
    e.call(PLAY_MENU_SOUND, &args![4u32]);
    let target = tile_at(e, base, 8);
    let first = e.call(TILE_TEXT_TO_TRAIT, &args![ENTER_TRAIT_A]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![target, first, 0.0f32.to_bits(), 0u32],
    );
    let second = e.call(TILE_TEXT_TO_TRAIT, &args![ENTER_TRAIT_B]).u32();
    e.call(TILE_POKE, &args![target, second, 1.0f32.to_bits()]);
}

// Translated from 007e7030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e7030 - the /GS stack cookie check (00ec408c) is not translated
/// Fills the list box (unnamed in the engine map): initializes it on tile
/// +0x2c with the name at `01073db8`, then adds one item per perk of
/// `AvailablePerks`: its name (or the setting at `011d35b8` when empty),
/// with the player's next rank formatted in when the perk has more than one
/// rank and the player has a rank in it; sets trait 0xfaa of the item's
/// tile to 10 and the trait at `011daf78` to the next rank.
pub fn fn_007e7030(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let list_tile = tile_at(e, base, 1);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![base + PERKS_LIST, list_tile, LIST_BOX_NAME],
    );
    let mut node = base + AVAILABLE_PERKS;
    e.with_stack(0x40, |e, buffer| {
        while node != 0 {
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let perk = e.mem.u32(item);
            let player: u32 = e.global(PLAYER);
            let rank = u32::from(
                e.vcall(player, PLAYER_SLOT_PERK_RANK, &args![perk, 0u32])
                    .u8(),
            );
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let perk = e.mem.u32(item);
            let name = e.call(STRING_C_STR, &args![perk.wrapping_add(0x18)]).u32();
            e.call(COPY_STRING, &args![buffer, 0x40u32, name]);
            if e.call(STRING_LENGTH, &args![buffer]).u32() == 0 {
                let text = e.call(SETTING_STRING, &args![NO_NAME_TEXT]).u32();
                e.call(COPY_STRING, &args![buffer, 0x40u32, text]);
            }
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let perk = e.mem.u32(item);
            let ranks = e.call(PERK_NUM_RANKS, &args![perk]).u8();
            if ranks > 1 && rank > 0 {
                e.call(
                    PRINT_FORMATTED,
                    &args![buffer, 0x40u32, RANKED_NAME_FORMAT, buffer, rank + 1],
                );
            }
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let perk = e.mem.u32(item);
            let item_tile = e
                .call(
                    LIST_BOX_ADD_ITEM,
                    &args![base + PERKS_LIST, perk, buffer, 0u32, 0u32],
                )
                .u32();
            e.call(TILE_SET_VALUE, &args![item_tile, 0xfaau32, 0xau32]);
            let trait_id: u32 = e.global(USER_TRAIT_011DAF78);
            e.call(TILE_SET_VALUE, &args![item_tile, trait_id, rank + 1]);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 007e71b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e71b0 - the /GS stack cookie check (00ec408c) is not translated
/// `TraitMenu::UpdatePointCounter` (Xbox PDB): formats the points left
/// (`iMaxPoints - iCurrPoints`) with the setting at `011d4558` when it is
/// exactly 1, else `011d2c10`, into tile +0x38's trait 0xfc4, and shows
/// that tile (trait 0xfa3 = 1).
pub fn trait_menu_update_point_counter(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let left = e
        .mem
        .u32(base + MAX_POINTS)
        .wrapping_sub(e.mem.u32(base + CURR_POINTS));
    let format_setting = if left == 1 {
        COUNTER_FORMAT_ONE
    } else {
        COUNTER_FORMAT_MANY
    };
    e.with_stack(0x40, |e, buffer| {
        let format = e.call(SETTING_STRING, &args![format_setting]).u32();
        e.call(PRINT_FORMATTED, &args![buffer, 0x40u32, format, left]);
        let tile = tile_at(e, base, 4);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
        let tile = tile_at(e, base, 4);
        e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, 1u32]);
    });
}

// Translated from 007e7280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::DoClick` (Xbox PDB): id 5 plays menu sound 1, clears the
/// points, shows them on the menu tile, runs the list box callback
/// `007e7630` and updates the counter; id 6 closes the menu; id 10 (a list
/// item) deselects a selected item, or selects it when its trait at
/// `011daf70` equals the `double` at `0101e568` and points are left,
/// applying the perk's effects either way, then counts the selection into
/// `iCurrPoints`, shows it and updates the counter. Other ids do nothing.
pub fn trait_menu_do_click(e: &mut Engine, this: Ptr, id: u32, tile: u32) {
    let base = this.addr();
    match id {
        5 => {
            e.call(PLAY_MENU_SOUND, &args![1u32]);
            e.mem.set_u32(base + CURR_POINTS, 0);
            show_points(e, base);
            e.call(
                LIST_BOX_FOR_EACH,
                &args![base + PERKS_LIST, CALLBACK_RESET, 0u32, 0x7fff_ffffu32],
            );
            trait_menu_update_point_counter(e, this);
        }
        6 => trait_menu_close(e),
        0xa => {
            if e.call(LIST_BOX_IS_SELECTED, &args![base + PERKS_LIST, tile])
                .bool()
            {
                e.call(LIST_BOX_DESELECT, &args![base + PERKS_LIST, tile]);
                trait_menu_apply_perk_effects(e, this, tile);
            } else {
                let trait_id: u32 = e.global(USER_TRAIT_011DAF70);
                let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
                if value == e.global::<f64>(CLICK_VALUE)
                    && e.mem.i32(base + CURR_POINTS) < e.mem.i32(base + MAX_POINTS)
                {
                    e.call(LIST_BOX_SELECT, &args![base + PERKS_LIST, tile]);
                    trait_menu_apply_perk_effects(e, this, tile);
                }
            }
            let count = e
                .call(LIST_BOX_SELECTED_COUNT, &args![base + PERKS_LIST])
                .u32();
            e.mem.set_u32(base + CURR_POINTS, count);
            show_points(e, base);
            trait_menu_update_point_counter(e, this);
        }
        _ => {}
    }
}

/// `iCurrPoints` into the menu tile's trait at `011daf64`.
fn show_points(e: &mut Engine, base: u32) {
    let value = e.mem.u32(base + CURR_POINTS);
    let trait_id: u32 = e.global(USER_TRAIT_011DAF64);
    let tile = e.call(LIST_NODE_NEXT, &args![base]).u32();
    e.call(TILE_SET_VALUE, &args![tile, trait_id, value]);
}

// Translated from 007e73c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::DoIdle` (Xbox PDB): when the menu is hidden (`xFadeState`
/// 4) and the object at `011f2250` answers false (`00525430`), prepares
/// the menu for visibility (`Menu::PrepForVisibility(0)`).
pub fn trait_menu_do_idle(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + FADE_STATE) == 4
        && !e.call(IDLE_FLAG_CHECK, &args![IDLE_FLAG_OBJECT]).bool()
    {
        e.call(MENU_PREP_FOR_VISIBILITY, &args![this, 0u32]);
    }
}

// Translated from 007e7670 (decompiled, FalloutNV.exe 1.4.0.525)
/// List box callback, `cdecl (tile, perk)` (unnamed in the engine map):
/// sets the tile's trait at `011daf70` (flag 1) to the float at
/// `01023cd8` when the perk's requirements are met and its level (a signed
/// byte) is at most the player's, else to the float at `0101e704`
/// (`FLD`/`FSTP` copies: a signalling NaN comes out quiet).
pub fn fn_007e7670(e: &mut Engine, tile: u32, perk: u32) {
    let player: u32 = e.global(PLAYER);
    let available = e.call(PERK_REQUIREMENTS_MET, &args![perk, player]).bool() && {
        let level = i32::from(e.call(PERK_LEVEL, &args![perk]).u8() as i8);
        let player: u32 = e.global(PLAYER);
        level <= i32::from(e.call(PLAYER_LEVEL, &args![player]).u16())
    };
    let source = if available {
        AVAILABLE_VALUE
    } else {
        UNAVAILABLE_VALUE
    };
    let value = x87_quiet_bits(e.global::<u32>(source));
    let trait_id: u32 = e.global(USER_TRAIT_011DAF70);
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, 1u32]);
}

// Translated from 007e76e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// List box callback, `cdecl (tile, perk)` (unnamed in the engine map):
/// when the tile's trait at `011daf5c` is true, gives the player the next
/// rank of the perk (player slot +0x498 with `(perk, rank + 1, 0)`, the
/// rank from slot +0x4A0).
pub fn fn_007e76e0(e: &mut Engine, tile: u32, perk: u32) {
    let trait_id: u32 = e.global(USER_TRAIT_011DAF5C);
    if !e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool() {
        return;
    }
    let player: u32 = e.global(PLAYER);
    let rank = u32::from(
        e.vcall(player, PLAYER_SLOT_PERK_RANK, &args![perk, 0u32])
            .u8(),
    );
    let player: u32 = e.global(PLAYER);
    e.vcall(
        player,
        PLAYER_SLOT_SET_PERK_RANK,
        &args![perk, rank + 1, 0u32],
    );
}

// Translated from 007e7740 (decompiled, FalloutNV.exe 1.4.0.525)
/// List box sort callback, `cdecl (left, right)` (unnamed in the engine
/// map); each item is a tile (+0) and a perk (+4). Answers 1 when the right
/// tile's trait at `011daf70` is above the left's, -1 when it is below,
/// then -1 or 1 by the perks' levels (signed bytes, lower first), and
/// otherwise the string compare of the tiles' trait 0xfc4 (left, right).
/// NaN values compare neither above nor below.
pub fn fn_007e7740(e: &mut Engine, left: u32, right: u32) -> i32 {
    let (left_value, right_value) = trait_values(e, left, right);
    if right_value > left_value {
        return 1;
    }
    let (left_value, right_value) = trait_values(e, left, right);
    if right_value < left_value {
        return -1;
    }
    let left_level = item_perk_level(e, left);
    let right_level = item_perk_level(e, right);
    if left_level < right_level {
        return -1;
    }
    let left_level = item_perk_level(e, left);
    let right_level = item_perk_level(e, right);
    if left_level > right_level {
        return 1;
    }
    let right_tile = e.mem.u32(right);
    let right_text = e.call(TILE_GET_STRING, &args![right_tile, 0xfc4u32]).u32();
    let left_tile = e.mem.u32(left);
    let left_text = e.call(TILE_GET_STRING, &args![left_tile, 0xfc4u32]).u32();
    e.call(COMPARE_STRINGS, &args![left_text, right_text]).i32()
}

/// The two items' tiles' trait at `011daf70` (left read first), as the
/// `double`s the comparison sees.
fn trait_values(e: &mut Engine, left: u32, right: u32) -> (f64, f64) {
    let trait_id: u32 = e.global(USER_TRAIT_011DAF70);
    let left_tile = e.mem.u32(left);
    let left_value = e.call(TILE_GET_FLOAT, &args![left_tile, trait_id]).f64();
    let trait_id: u32 = e.global(USER_TRAIT_011DAF70);
    let right_tile = e.mem.u32(right);
    let right_value = e.call(TILE_GET_FLOAT, &args![right_tile, trait_id]).f64();
    (left_value, right_value)
}

/// The level (signed byte) of the perk of a sort item (+4).
fn item_perk_level(e: &mut Engine, item: u32) -> i32 {
    let perk = e.mem.u32(item + 4);
    i32::from(e.call(PERK_LEVEL, &args![perk]).u8() as i8)
}

// Translated from 007e7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::ApplyPerkEffects` (Xbox PDB): for the perk of the clicked
/// tile, at the rank of the tile's trait at `011daf78` (truncated, low
/// byte), takes the rank's entry and, when its type (vtable +0x10) is 1,
/// rebuilds `PerkEffectList` (`011daff0`): for each effect item of the
/// entry's object (list at +0x28) with a code other than -1, an
/// `EffectItemData` (0x14 bytes, magnitude +0, code +0x10) holding its
/// magnitude minus the magnitudes of the previous rank's items with the
/// same code, negated when the tile's trait at `011daf5c` is false and
/// again when `007869e0` answers true for the item's +0x14 word and the
/// code.
pub fn trait_menu_apply_perk_effects(e: &mut Engine, this: Ptr, tile: u32) {
    let perk = e
        .call(
            LIST_BOX_ITEM_OF_TILE,
            &args![this.addr() + PERKS_LIST, tile],
        )
        .u32();
    let trait_id: u32 = e.global(USER_TRAIT_011DAF78);
    let value = e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64();
    let rank = crate::x87::fistp_chop(value) as u8;
    let entry = e
        .call(PERK_ENTRY_FOR_RANK, &args![perk, u32::from(rank)])
        .u32();
    let previous = if rank > 1 {
        e.call(PERK_ENTRY_FOR_RANK, &args![perk, u32::from(rank) - 1])
            .u32()
    } else {
        0
    };
    if entry == 0 || e.vcall(entry, 0x10, &args![]).u32() != 1 {
        return;
    }
    let spell = e.call(PERK_ENTRY_SPELL, &args![entry]).u32();
    let previous_spell = if previous != 0 {
        e.call(PERK_ENTRY_SPELL, &args![previous]).u32()
    } else {
        0
    };
    e.call(EFFECT_ARRAY_CLEAR, &args![PERK_EFFECT_LIST, 1u32]);
    let mut node = if spell != 0 { spell + 0x28 } else { 0 };
    e.with_stack(0x14, |e, data| {
        let data = data.addr();
        while node != 0 {
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) == 0 {
                break;
            }
            e.call(EFFECT_ITEM_DATA_CONSTRUCT, &args![data]);
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let effect = e.mem.u32(item);
            let code = e.call(EFFECT_ITEM_CODE, &args![effect]).u32();
            e.mem.set_u32(data + 0x10, code);
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let effect = e.mem.u32(item);
            let magnitude = e.call(EFFECT_ITEM_GET_MAGNITUDE, &args![effect]).u32();
            e.mem.set_u32(data, magnitude);
            if code != 0xffff_ffff {
                subtract_previous(e, previous_spell, data);
                let trait_id: u32 = e.global(USER_TRAIT_011DAF5C);
                if !e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool() {
                    let magnitude = e.mem.i32(data).wrapping_mul(-1);
                    e.mem.set_i32(data, magnitude);
                }
                let code = e.mem.u32(data + 0x10);
                let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let effect = e.mem.u32(item);
                let owner = e.call(EFFECT_ITEM_FIELD_14, &args![effect]).u32();
                if e.call(EFFECT_CODE_CHECK, &args![owner, code]).bool() {
                    let magnitude = e.mem.i32(data).wrapping_mul(-1);
                    e.mem.set_i32(data, magnitude);
                }
                e.call(EFFECT_ARRAY_ADD, &args![PERK_EFFECT_LIST, data]);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    });
}

/// The inner loop of [`trait_menu_apply_perk_effects`]: subtracts from the
/// data's magnitude (+0) the magnitude of every item of the previous rank's
/// list (+0x28 of `previous_spell`) whose code equals the data's (+0x10).
fn subtract_previous(e: &mut Engine, previous_spell: u32, data: u32) {
    let mut other = if previous_spell != 0 {
        previous_spell + 0x28
    } else {
        0
    };
    while other != 0 {
        let item = e.call(LIST_NODE_ITEM, &args![other]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_NODE_ITEM, &args![other]).u32();
        let effect = e.mem.u32(item);
        if e.call(EFFECT_ITEM_CODE, &args![effect]).u32() == e.mem.u32(data + 0x10) {
            let item = e.call(LIST_NODE_ITEM, &args![other]).u32();
            let effect = e.mem.u32(item);
            let other_magnitude = e.call(EFFECT_ITEM_GET_MAGNITUDE, &args![effect]).u32();
            let magnitude = e.mem.u32(data).wrapping_sub(other_magnitude);
            e.mem.set_u32(data, magnitude);
        }
        other = e.call(LIST_NODE_NEXT, &args![other]).u32();
    }
}

/// `Tile::PlayTileSound` (`00a0b110`, `RET 4`), `00709d40` (no words; whether the
/// gamepad is in use, by its use here), the input object (`011dea0c`) with its device
/// (`00877720`) and that device's axis (`00a23390`, `RET 8`: 0 and 0xa), the magnitude
/// (`00408820`, cdecl, a `float`, `ST0`) and its dead zone (`010745f8`, a `double`),
/// the clock (`00457fe0`) and the next repeat time (`011db004`), `_ftol2` (`00ec62c0`).
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
const GAMEPAD_IN_USE_00709D40: u32 = 0x0070_9d40;
const INPUT_011DEA0C: u32 = 0x011d_ea0c;
const INPUT_DEVICE_00877720: u32 = 0x0087_7720;
const DEVICE_AXIS_00A23390: u32 = 0x00a2_3390;
const MAGNITUDE_00408820: u32 = 0x0040_8820;
const AXIS_DEAD_ZONE: u32 = 0x0107_45f8;
const CLOCK_00457FE0: u32 = 0x0045_7fe0;
const NEXT_REPEAT_011DB004: u32 = 0x011d_b004;
const FTOL2: u32 = 0x00ec_62c0;

// Translated from 007e73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::DoGamepad` (Xbox PDB; `thiscall`, `RET 8`: the button and a word not
/// read). Button 9 with a list box selection (`lbPerksList` slot +4): its tile plays
/// sound 0xfcb and the menu clicks it (slot +0xc with 10 and the tile); handled. Then,
/// with the gamepad in use and tile +0x48 shown (`Tile::IsTrue` 0xfa3): the axis
/// outside the dead zone steps tile +0x48's `"_current_value"` by one (up for a
/// negative axis), at most every 500 ms at first and 50 ms while held (`011db004`),
/// poking `"_SetInCode"` when the value read back changed; a centred axis clears the repeat
/// time. Answers whether the button was handled.
pub fn trait_menu_do_gamepad(e: &mut Engine, this: Ptr, button: u32, _unused_2: u32) -> bool {
    let base = this.addr();
    let mut handled = false;
    let list_box = base + PERKS_LIST;
    if button == 9 && e.vcall(list_box, 4, &args![]).u32() != 0 {
        let tile = e.vcall(list_box, 4, &args![]).u32();
        e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
        let tile = e.vcall(list_box, 4, &args![]).u32();
        e.vcall(base, 0xc, &args![0xau32, tile]);
        handled = true;
    }
    if !e.call(GAMEPAD_IN_USE_00709D40, &args![]).bool() {
        return handled;
    }
    let slider = e.mem.u32(base + 0x48);
    if !e.call(TILE_IS_TRUE, &args![slider, 0xfa3u32]).bool() {
        return handled;
    }
    let input = e.global::<u32>(INPUT_011DEA0C);
    let device = e.call(INPUT_DEVICE_00877720, &args![input]).u32();
    let mut axis = e
        .call(DEVICE_AXIS_00A23390, &args![device, 0u32, 0xau32])
        .i32();
    let magnitude = e
        .call(MAGNITUDE_00408820, &args![x87_store32(f64::from(axis))])
        .f64();
    // FCOMP with the dead zone; TEST AH,0x41: inside (or on) it the axis is 0.
    if magnitude <= e.global::<f64>(AXIS_DEAD_ZONE) {
        axis = 0;
    }
    let step = if axis != 0 {
        if axis < 0 {
            1
        } else {
            -1
        }
    } else {
        0
    };
    if step != 0 {
        let now = e.call(CLOCK_00457FE0, &args![]).u32();
        if now > e.global::<u32>(NEXT_REPEAT_011DB004) {
            let slider = e.mem.u32(base + 0x48);
            let current = e.call(TILE_TEXT_TO_TRAIT, &args![ENTER_TRAIT_A]).u32();
            let value = e.call(TILE_GET_FLOAT, &args![slider, current]).f64();
            let value = e.call(FTOL2, &args![value]).i32();
            let stepped = x87_store32(f64::from(value.wrapping_add(step)));
            e.call(TILE_SET_FLOAT, &args![slider, current, stepped, 0u32]);
            let before = f64::from(value);
            let now_value = e.call(TILE_GET_FLOAT, &args![slider, current]).f64();
            // FUCOMPP; TEST AH,0x44; JNP: poked when the value read back differs.
            if before != now_value {
                let set_in_code = e.call(TILE_TEXT_TO_TRAIT, &args![ENTER_TRAIT_B]).u32();
                e.call(TILE_POKE, &args![slider, set_in_code, x87_store32(1.0)]);
            }
            let now = e.call(CLOCK_00457FE0, &args![]).u32();
            let delay = if e.global::<u32>(NEXT_REPEAT_011DB004) != 0 {
                0x32
            } else {
                0x1f4
            };
            e.set_global(NEXT_REPEAT_011DB004, now.wrapping_add(delay));
        }
    } else {
        e.set_global(NEXT_REPEAT_011DB004, 0u32);
    }
    handled
}

// Translated from 007e75d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::DoGamepadUpEvent` (Xbox PDB; `thiscall`, `RET 8`: the button and a word
/// not read): button 11 with tile +0x40 enabled (`Tile::IsTrue` 0xfaf) plays menu sound
/// 1 and clicks id 6 (slot +0xc with 6 and 0); handled. Answers whether it was handled.
pub fn trait_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    button: u32,
    _unused_2: u32,
) -> bool {
    if button != 0xb {
        return false;
    }
    let tile = e.mem.u32(this.addr() + 0x40);
    if !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
        return false;
    }
    e.call(PLAY_MENU_SOUND, &args![1u32]);
    e.vcall(this.addr(), 0xc, &args![6u32, 0u32]);
    true
}

// Translated from 007e7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::ResetPerksFunc` (Xbox PDB; cdecl, a tile; the list box callback): when
/// the tile's user trait `011daf5c` is true, sets it to 0 and applies the perk effects of
/// the open menu for the tile (`007e7850`).
pub fn trait_menu_reset_perks_func(e: &mut Engine, tile: u32) {
    let which = e.global::<u32>(USER_TRAIT_011DAF5C);
    if !e.call(TILE_IS_TRUE, &args![tile, which]).bool() {
        return;
    }
    let which = e.global::<u32>(USER_TRAIT_011DAF5C);
    e.call(TILE_SET_VALUE, &args![tile, which, 0u32]);
    let me = e.global::<u32>(TRAIT_MENU_ME);
    trait_menu_apply_perk_effects(e, Ptr::new(me), tile);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007e68a0, trait_menu_get_class(Ptr) -> u32),
        entry!(0x007e68b0, trait_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(
            0x007e68e0,
            trait_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007e6910, fn_007e6910(Ptr)),
        entry!(0x007e6990, trait_menu_create() -> u32),
        entry!(0x007e6dc0, fn_007e6dc0() -> u32),
        entry!(0x007e6dd0, fn_007e6dd0(Ptr) -> bool),
        entry!(0x007e6e10, trait_menu_close()),
        entry!(0x007e6ea0, trait_menu_do_enter(Ptr, u32, u32)),
        entry!(0x007e7030, fn_007e7030(Ptr)),
        entry!(0x007e71b0, trait_menu_update_point_counter(Ptr)),
        entry!(0x007e7280, trait_menu_do_click(Ptr, u32, u32)),
        entry!(0x007e73c0, trait_menu_do_idle(Ptr)),
        entry!(0x007e7670, fn_007e7670(u32, u32)),
        entry!(0x007e76e0, fn_007e76e0(u32, u32)),
        entry!(0x007e7740, fn_007e7740(u32, u32) -> i32),
        entry!(0x007e7850, trait_menu_apply_perk_effects(Ptr, u32)),
        entry!(
            0x007e_73f0,
            trait_menu_do_gamepad(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x007e_75d0,
            trait_menu_do_gamepad_up_event(Ptr, u32, u32) -> bool
        ),
        entry!(0x007e_7630, trait_menu_reset_perks_func(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANK_SLOT_DOUBLE: u32 = 0xdead_0000;
    const SET_RANK_SLOT_DOUBLE: u32 = 0xdead_0010;
    const CLASS_SLOT_DOUBLE: u32 = 0xdead_0020;
    const DELETE_SLOT_DOUBLE: u32 = 0xdead_0030;
    const SORT_SLOT_DOUBLE: u32 = 0xdead_0040;
    const TYPE_SLOT_DOUBLE: u32 = 0xdead_0050;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_0000, 0x4_0000);
        e.map(0x0101_0000, 0x7_0000);
        e
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// A player whose rank slot answers 2 and whose other slot is logged.
    fn player(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x500);
        e.mem
            .set_u32(vtable + PLAYER_SLOT_PERK_RANK, RANK_SLOT_DOUBLE);
        e.mem
            .set_u32(vtable + PLAYER_SLOT_SET_PERK_RANK, SET_RANK_SLOT_DOUBLE);
        let player = e.mem.alloc(0x10);
        e.mem.set_u32(player, vtable);
        e.set_global(PLAYER, player);
        e.register(RANK_SLOT_DOUBLE, |_, _| ret(2));
        quiet(e, &[SET_RANK_SLOT_DOUBLE]);
        player
    }

    /// A menu object with every tile attached (tile `i` at 0x9000 + i).
    fn menu(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, DELETE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x34, CLASS_SLOT_DOUBLE);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(list_vtable + 0x20, SORT_SLOT_DOUBLE);
        let menu = e.mem.alloc(0x8c);
        e.mem.set_u32(menu, vtable);
        e.mem.set_u32(menu + 4, 0x7777);
        e.mem.set_u32(menu + PERKS_LIST, list_vtable);
        for index in 0..9 {
            e.mem.set_u32(menu + TILES + 4 * index, 0x9000 + index);
        }
        quiet(e, &[DELETE_SLOT_DOUBLE, SORT_SLOT_DOUBLE]);
        e.register(CLASS_SLOT_DOUBLE, |_, _| ret(TRAIT_MENU_CLASS));
        menu
    }

    #[test]
    fn get_class_and_the_static_class_answer_0x43c() {
        let mut e = engine();
        assert_eq!(e.call(0x007e_68a0, &args![0x1000u32]).u32(), 0x43c);
        assert_eq!(e.call(0x007e_6dc0, &args![]).u32(), 0x43c);
    }

    #[test]
    fn attach_tile_by_id_keeps_ids_0_to_8() {
        let mut e = engine();
        let menu = e.mem.alloc(0x8c);
        e.call(0x007e_68b0, &args![menu, 8u32, 0x55u32]);
        e.call(0x007e_68b0, &args![menu, 9u32, 0x66u32]);
        e.call(0x007e_68b0, &args![menu, -1i32, 0x77u32]);
        assert_eq!(e.mem.u32(menu + 0x48), 0x55);
        assert_eq!(e.mem.u32(menu + 0x4c), 0);
        assert_eq!(e.mem.u32(menu + 0x24), 0);
    }

    #[test]
    fn deleting_destructor_destroys_the_members_and_frees_on_bit_0() {
        let mut e = engine();
        quiet(
            &mut e,
            &[SIMPLE_LIST_DESTROY, LIST_BOX_DESTROY, MENU_DESTROY, FREE],
        );
        let menu = e.mem.alloc(0x8c);
        e.set_global(TRAIT_MENU_ME, menu);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_68e0, &args![menu, 1u32]).u32(), menu);
        assert_eq!(e.mem.u32(menu), TRAIT_MENU_VTABLE);
        assert_eq!(e.global::<u32>(TRAIT_MENU_ME), 0);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTROY), vec![vec![menu + 0x84]]);
        assert_eq!(calls(&e, LIST_BOX_DESTROY), vec![vec![menu + 0x54]]);
        assert_eq!(calls(&e, MENU_DESTROY), vec![vec![menu]]);
        assert_eq!(calls(&e, FREE), vec![vec![menu]]);
        e.call_log = Some(vec![]);
        e.call(0x007e_68e0, &args![menu, 2u32]);
        assert!(calls(&e, FREE).is_empty());
    }

    #[test]
    fn all_tiles_check_fails_on_a_missing_tile() {
        let mut e = engine();
        let menu = menu(&mut e);
        assert!(e.call(0x007e_6dd0, &args![menu]).bool());
        e.mem.set_u32(menu + TILES + 4 * 4, 0);
        assert!(!e.call(0x007e_6dd0, &args![menu]).bool());
    }

    fn create_doubles(e: &mut Engine, root: u32, menu: u32) {
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| Ret::default());
        e.register(INTERFACE_OBJECT, |_, _| ret(0x5000));
        e.register(TILE_READER, |_, _| ret(0x6000));
        e.register_double(TILE_READ_FILE, move |_, _| ret(root));
        e.register_double(TILE_GET_MENU, move |_, _| ret(menu));
        quiet(
            e,
            &[
                MENU_SET_MENU_TILE,
                PRINT_MESSAGE,
                TILE_SET_FLOAT,
                SETTING_STRING,
                PRINT_FORMATTED,
                TILE_SET_STRING,
                TILE_TEXT_TO_TRAIT,
                SIMPLE_LIST_ADD,
                LIST_BOX_INITIALIZE,
                TILE_SET_VALUE,
                SETTING_VALUE,
                LIST_BOX_FOR_EACH,
                LIST_BOX_RESTORE_STATE,
                LIST_AT_178,
            ],
        );
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 6006.0,
            ..Ret::default()
        });
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 2);
        e.register_double(TRAIT_POINTS_VALUE, move |_, _| ret(count));
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(PERK_LEVEL, |_, _| ret(1));
        e.register(PERK_NUM_RANKS, |_, _| ret(3));
        for check in [PERK_CHECK_7A9280, PERK_IS_ATTAINABLE, PERK_CHECK_5EB5E0] {
            e.register(check, |_, _| ret(1));
        }
    }

    #[test]
    fn create_builds_the_menu_and_offers_the_perks_that_pass() {
        let mut e = engine();
        player(&mut e);
        let menu = menu(&mut e);
        let root = 0x7000;
        create_doubles(&mut e, root, menu);
        e.set_global(ROOT_VALUE_A, 6006.0f64);
        let setting = e.mem.alloc(4);
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x4444);
        e.register_double(LIST_AT_178, move |_, _| ret(node));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_6990, &args![]).u32(), root);
        assert_eq!(e.global::<u32>(TRAIT_MENU_ME), menu);
        assert_eq!(calls(&e, MENU_SET_MENU_TILE), vec![vec![menu, root, 0]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![root, 0xfad, 2.5f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.u32(menu + MAX_POINTS), 2);
        assert_eq!(e.mem.u32(menu + CURR_POINTS), 0);
        assert_eq!(calls(&e, SETTING_STRING)[0], vec![POINTS_FORMAT_MANY]);
        let added = calls(&e, SIMPLE_LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], menu + AVAILABLE_PERKS);
        assert_eq!(
            calls(&e, LIST_BOX_INITIALIZE),
            vec![vec![menu + PERKS_LIST, 0x9001, LIST_BOX_NAME]]
        );
        assert_eq!(
            calls(&e, SORT_SLOT_DOUBLE),
            vec![vec![menu + PERKS_LIST, CALLBACK_COMPARE]]
        );
        assert_eq!(
            calls(&e, LIST_BOX_RESTORE_STATE),
            vec![vec![menu + PERKS_LIST, 1]]
        );
        assert!(calls(&e, PRINT_MESSAGE).is_empty());
    }

    #[test]
    fn create_rejects_a_menu_with_a_missing_tile_or_another_class() {
        let mut e = engine();
        player(&mut e);
        let menu = menu(&mut e);
        create_doubles(&mut e, 0x7000, menu);
        e.mem.set_u32(menu + TILES, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_6990, &args![]).u32(), 0);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![CREATION_FAILED_MESSAGE]]
        );
        e.register(CLASS_SLOT_DOUBLE, |_, _| ret(0x400));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_6990, &args![]).u32(), 0);
        assert_eq!(calls(&e, DELETE_SLOT_DOUBLE), vec![vec![menu, 1]]);
        assert!(calls(&e, MENU_SET_MENU_TILE).is_empty());
    }

    #[test]
    fn close_applies_the_selection_and_fades_out() {
        let mut e = engine();
        let player = player(&mut e);
        let menu = menu(&mut e);
        e.set_global(TRAIT_MENU_ME, menu);
        quiet(
            &mut e,
            &[
                HUD_CLEAR_XP_METER,
                LIST_BOX_FOR_EACH,
                ACTOR_UPDATE_MAGIC,
                STATS_UPDATE_PLAYER_NAME,
                STATS_UPDATE_PLAYER_TITLE,
                TILE_SET_VALUE,
                MENU_START_FADE_OUT,
            ],
        );
        e.register(PLAYER_PROGRESSION, |_, _| ret(0x3333));
        e.register(READY_TO_LEVEL_UP, |_, _| ret(1));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.call_log = Some(vec![]);
        e.call(0x007e_6e10, &args![]);
        assert_eq!(
            calls(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                menu + PERKS_LIST,
                CALLBACK_APPLY_SELECTED,
                0,
                0x7fff_ffff
            ]]
        );
        assert_eq!(calls(&e, ACTOR_UPDATE_MAGIC), vec![vec![player, 1, 0]]);
        assert_eq!(calls(&e, TILE_SET_VALUE), vec![vec![0x7777, 0x1772, 1]]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        assert_eq!(e.global::<u32>(TRAIT_MENU_ME), 0);
        e.call_log = Some(vec![]);
        e.call(0x007e_6e10, &args![]);
        assert!(calls(&e, LIST_BOX_FOR_EACH).is_empty());
    }

    #[test]
    fn do_enter_shows_the_perk_of_a_list_item() {
        let mut e = engine();
        let menu = menu(&mut e);
        quiet(
            &mut e,
            &[
                PERK_BUILD_DESCRIPTION,
                TILE_SET_STRING,
                TILE_SET_VALUE,
                PLAY_MENU_SOUND,
                TILE_SET_FLOAT,
                TILE_POKE,
            ],
        );
        e.register(LIST_BOX_ITEM_OF_TILE, |_, _| ret(0x4000));
        e.register(STRING_C_STR, |_, a| ret(a[0] + 0x100));
        e.register(TILE_GET_STRING, |_, _| ret(0x5555));
        e.register(STRING_LENGTH, |_, _| ret(2));
        e.register(TILE_TEXT_TO_TRAIT, |_, a| ret(a[0] & 0xff));
        e.call_log = Some(vec![]);
        e.call(0x007e_6ea0, &args![menu, 0xau32, 0x8888u32]);
        assert_eq!(calls(&e, STRING_C_STR), vec![vec![0x402c]]);
        let strings = calls(&e, TILE_SET_STRING);
        assert_eq!(strings[0], vec![0x9002, 0xfcc, 0x412c, 1]);
        assert_eq!(strings[1][..2], [0x9003, 0xfc4]);
        assert_eq!(
            calls(&e, TILE_SET_VALUE),
            vec![
                vec![0x9002, 0xfa3, 1],
                vec![0x9003, 0xfa3, 1],
                vec![0x9007, 0xfa3, 0]
            ]
        );
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![4]]);
        assert_eq!(calls(&e, TILE_SET_FLOAT), vec![vec![0x9008, 0xd8, 0, 0]]);
        assert_eq!(
            calls(&e, TILE_POKE),
            vec![vec![0x9008, 0x50, 1.0f32.to_bits()]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007e_6ea0, &args![menu, 0x9u32, 0x8888u32]);
        assert!(calls(&e, TILE_SET_STRING).is_empty());
    }

    #[test]
    fn fill_list_adds_each_available_perk_with_its_next_rank() {
        let mut e = engine();
        player(&mut e);
        let menu = menu(&mut e);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x4100);
        e.mem.set_u32(menu + AVAILABLE_PERKS, 0x4000);
        e.mem.set_u32(menu + AVAILABLE_PERKS + 4, second);
        quiet(
            &mut e,
            &[
                LIST_BOX_INITIALIZE,
                COPY_STRING,
                PRINT_FORMATTED,
                TILE_SET_VALUE,
            ],
        );
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(STRING_C_STR, |_, a| ret(a[0]));
        e.register(STRING_LENGTH, |_, _| ret(0));
        e.register(SETTING_STRING, |_, _| ret(0x6666));
        e.register(PERK_NUM_RANKS, |_, a| {
            ret(if a[0] == 0x4000 { 3 } else { 1 })
        });
        e.register(LIST_BOX_ADD_ITEM, |_, a| ret(a[1] + 1));
        e.call_log = Some(vec![]);
        e.call(0x007e_7030, &args![menu]);
        let copies = calls(&e, COPY_STRING);
        assert_eq!(copies.len(), 4);
        assert_eq!(copies[0][2], 0x4018);
        assert_eq!(copies[1][2], 0x6666);
        let formatted = calls(&e, PRINT_FORMATTED);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0][2], RANKED_NAME_FORMAT);
        assert_eq!(formatted[0][4], 3);
        let items = calls(&e, LIST_BOX_ADD_ITEM);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1][1], 0x4100);
        let values = calls(&e, TILE_SET_VALUE);
        assert_eq!(values[0], vec![0x4001, 0xfaa, 0xa]);
        assert_eq!(values[1], vec![0x4001, 0, 3]);
    }

    #[test]
    fn point_counter_picks_the_singular_format_for_one_point() {
        let mut e = engine();
        let menu = menu(&mut e);
        quiet(&mut e, &[PRINT_FORMATTED, TILE_SET_STRING, TILE_SET_VALUE]);
        e.register(SETTING_STRING, |_, a| ret(a[0] + 1));
        e.mem.set_u32(menu + MAX_POINTS, 2);
        e.mem.set_u32(menu + CURR_POINTS, 1);
        e.call_log = Some(vec![]);
        e.call(0x007e_71b0, &args![menu]);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![COUNTER_FORMAT_ONE]]);
        assert_eq!(
            calls(&e, PRINT_FORMATTED)[0][2..],
            [COUNTER_FORMAT_ONE + 1, 1]
        );
        assert_eq!(calls(&e, TILE_SET_VALUE), vec![vec![0x9004, 0xfa3, 1]]);
        e.mem.set_u32(menu + CURR_POINTS, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_71b0, &args![menu]);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![COUNTER_FORMAT_MANY]]);
    }

    #[test]
    fn do_click_resets_selects_and_closes() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.set_global(CLICK_VALUE, 1.0f64);
        quiet(
            &mut e,
            &[
                PLAY_MENU_SOUND,
                TILE_SET_VALUE,
                LIST_BOX_FOR_EACH,
                SETTING_STRING,
                PRINT_FORMATTED,
                TILE_SET_STRING,
                LIST_BOX_SELECT,
                LIST_BOX_DESELECT,
                HUD_CLEAR_XP_METER,
                LIST_BOX_ITEM_OF_TILE,
                PERK_ENTRY_FOR_RANK,
                TILE_GET_FLOAT,
            ],
        );
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_BOX_SELECTED_COUNT, |_, _| ret(1));
        e.register(LIST_BOX_IS_SELECTED, |_, _| ret(0));
        e.mem.set_u32(menu + CURR_POINTS, 3);
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 5u32, 0u32]);
        assert_eq!(e.mem.u32(menu + CURR_POINTS), 0);
        assert_eq!(
            calls(&e, LIST_BOX_FOR_EACH),
            vec![vec![menu + PERKS_LIST, CALLBACK_RESET, 0, 0x7fff_ffff]]
        );
        // A value that is not the click value selects nothing.
        e.mem.set_u32(menu + MAX_POINTS, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 0xau32, 0x8888u32]);
        assert!(calls(&e, LIST_BOX_SELECT).is_empty());
        assert_eq!(e.mem.u32(menu + CURR_POINTS), 1);
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 0xau32, 0x8888u32]);
        assert_eq!(
            calls(&e, LIST_BOX_SELECT),
            vec![vec![menu + PERKS_LIST, 0x8888]]
        );
        e.register(LIST_BOX_IS_SELECTED, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 0xau32, 0x8888u32]);
        assert_eq!(calls(&e, LIST_BOX_DESELECT).len(), 1);
        e.set_global(TRAIT_MENU_ME, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 6u32, 0u32]);
        assert_eq!(calls(&e, HUD_CLEAR_XP_METER).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x007e_7280, &args![menu, 7u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_idle_prepares_a_hidden_menu() {
        let mut e = engine();
        let menu = menu(&mut e);
        quiet(&mut e, &[MENU_PREP_FOR_VISIBILITY, IDLE_FLAG_CHECK]);
        e.mem.set_u32(menu + FADE_STATE, 4);
        e.call_log = Some(vec![]);
        e.call(0x007e_73c0, &args![menu]);
        assert_eq!(calls(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
        e.register(IDLE_FLAG_CHECK, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x007e_73c0, &args![menu]);
        assert!(calls(&e, MENU_PREP_FOR_VISIBILITY).is_empty());
        e.mem.set_u32(menu + FADE_STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x007e_73c0, &args![menu]);
        assert!(calls(&e, IDLE_FLAG_CHECK).is_empty());
    }

    #[test]
    fn set_value_callback_picks_the_value_by_requirements_and_level() {
        let mut e = engine();
        player(&mut e);
        e.set_global(AVAILABLE_VALUE, 1.0f32);
        e.set_global(UNAVAILABLE_VALUE, 0x7f80_0001u32);
        e.set_global(USER_TRAIT_011DAF70, 0x33u32);
        quiet(&mut e, &[TILE_SET_FLOAT]);
        e.register(PERK_REQUIREMENTS_MET, |_, _| ret(1));
        e.register(PERK_LEVEL, |_, _| ret(0xfe));
        e.register(PLAYER_LEVEL, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x007e_7670, &args![0x9000u32, 0x4000u32]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x9000, 0x33, 1.0f32.to_bits(), 1]]
        );
        e.register(PERK_LEVEL, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x007e_7670, &args![0x9000u32, 0x4000u32]);
        assert_eq!(calls(&e, TILE_SET_FLOAT)[0][2], 0x7fc0_0001);
    }

    #[test]
    fn apply_selected_callback_gives_the_next_rank() {
        let mut e = engine();
        let player = player(&mut e);
        e.register(TILE_IS_TRUE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x007e_76e0, &args![0x9000u32, 0x4000u32]);
        assert_eq!(
            calls(&e, SET_RANK_SLOT_DOUBLE),
            vec![vec![player, 0x4000, 3, 0]]
        );
        e.register(TILE_IS_TRUE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x007e_76e0, &args![0x9000u32, 0x4000u32]);
        assert!(calls(&e, SET_RANK_SLOT_DOUBLE).is_empty());
    }

    #[test]
    fn sort_callback_orders_by_value_then_level_then_text() {
        let mut e = engine();
        let left = e.mem.alloc(8);
        let right = e.mem.alloc(8);
        e.mem.set_u32(left, 0x10);
        e.mem.set_u32(right, 0x20);
        e.mem.set_u32(left + 4, 1);
        e.mem.set_u32(right + 4, 2);
        let values = e.mem.alloc(8);
        e.register_double(TILE_GET_FLOAT, move |e, a| Ret {
            st0: f64::from(e.mem.f32(values + if a[0] == 0x10 { 0 } else { 4 })),
            ..Ret::default()
        });
        let levels = e.mem.alloc(8);
        e.register_double(PERK_LEVEL, move |e, a| {
            ret(e.mem.u32(levels + 4 * (a[0] - 1)))
        });
        e.register(TILE_GET_STRING, |_, a| ret(a[0] + 0x100));
        e.register(COMPARE_STRINGS, |_, a| ret(a[0].wrapping_sub(a[1])));
        e.mem.set_f32(values, 1.0);
        e.mem.set_f32(values + 4, 2.0);
        assert_eq!(e.call(0x007e_7740, &args![left, right]).i32(), 1);
        e.mem.set_f32(values + 4, 0.5);
        assert_eq!(e.call(0x007e_7740, &args![left, right]).i32(), -1);
        e.mem.set_f32(values + 4, f32::NAN);
        e.mem.set_u32(levels, 0xff);
        e.mem.set_u32(levels + 4, 1);
        assert_eq!(e.call(0x007e_7740, &args![left, right]).i32(), -1);
        e.mem.set_u32(levels, 2);
        assert_eq!(e.call(0x007e_7740, &args![left, right]).i32(), 1);
        e.mem.set_u32(levels, 1);
        assert_eq!(e.call(0x007e_7740, &args![left, right]).i32(), -0x10);
    }

    #[test]
    fn apply_perk_effects_rebuilds_the_effect_list() {
        let mut e = engine();
        let menu = menu(&mut e);
        let entry_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(entry_vtable + 0x10, TYPE_SLOT_DOUBLE);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, entry_vtable);
        let previous = e.mem.alloc(8);
        e.mem.set_u32(previous, entry_vtable);
        e.register(TYPE_SLOT_DOUBLE, |_, _| ret(1));
        e.register(LIST_BOX_ITEM_OF_TILE, |_, _| ret(0x4000));
        e.register(TILE_GET_FLOAT, |_, _| Ret {
            st0: 2.9,
            ..Ret::default()
        });
        e.register_double(PERK_ENTRY_FOR_RANK, move |_, a| {
            ret(if a[1] == 2 { entry } else { previous })
        });
        // The rank's list holds effects 0x100 (code 5, magnitude 10) and
        // 0x200 (code -1); the previous rank's holds 0x300 (code 5, 3).
        let spell = e.mem.alloc(0x30);
        let spell_second = e.mem.alloc(8);
        e.mem.set_u32(spell + 0x28, 0x100);
        e.mem.set_u32(spell + 0x2c, spell_second);
        e.mem.set_u32(spell_second, 0x200);
        let previous_spell = e.mem.alloc(0x30);
        e.mem.set_u32(previous_spell + 0x28, 0x300);
        e.register_double(PERK_ENTRY_SPELL, move |_, a| {
            ret(if a[0] == entry { spell } else { previous_spell })
        });
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(EFFECT_ITEM_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 0xffff_ffff);
            Ret::default()
        });
        e.register(EFFECT_ITEM_CODE, |_, a| {
            ret(if a[0] == 0x200 { 0xffff_ffff } else { 5 })
        });
        e.register(EFFECT_ITEM_GET_MAGNITUDE, |_, a| {
            ret(if a[0] == 0x300 { 3 } else { 10 })
        });
        e.register(TILE_IS_TRUE, |_, _| ret(0));
        e.register(EFFECT_ITEM_FIELD_14, |_, _| ret(0x7000));
        e.register(EFFECT_CODE_CHECK, |_, _| ret(0));
        quiet(&mut e, &[EFFECT_ARRAY_CLEAR]);
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = added.clone();
        e.register_double(EFFECT_ARRAY_ADD, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.i32(a[1]), e.mem.u32(a[1] + 0x10)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007e_7850, &args![menu, 0x8888u32]);
        assert_eq!(
            calls(&e, PERK_ENTRY_FOR_RANK),
            vec![vec![0x4000, 2], vec![0x4000, 1]]
        );
        assert_eq!(
            calls(&e, EFFECT_ARRAY_CLEAR),
            vec![vec![PERK_EFFECT_LIST, 1]]
        );
        assert_eq!(*added.borrow(), vec![(-7, 5)]);
        e.register(EFFECT_CODE_CHECK, |_, _| ret(1));
        added.borrow_mut().clear();
        e.call(0x007e_7850, &args![menu, 0x8888u32]);
        assert_eq!(*added.borrow(), vec![(7, 5)]);
        e.register(TYPE_SLOT_DOUBLE, |_, _| ret(2));
        e.call_log = Some(vec![]);
        e.call(0x007e_7850, &args![menu, 0x8888u32]);
        assert!(calls(&e, EFFECT_ARRAY_CLEAR).is_empty());
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x011d_b000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0107_4000, 0x1000);
        e.set_global(AXIS_DEAD_ZONE, 8689.0f64);
        for address in [
            TILE_PLAY_TILE_SOUND,
            TILE_SET_FLOAT,
            TILE_POKE,
            PLAY_MENU_SOUND,
            TILE_SET_VALUE,
            INPUT_DEVICE_00877720,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..8).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e
    }

    fn menu(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x8c);
        e.mem.set_u32(this, VTABLE);
        e.mem.set_u32(this + PERKS_LIST, VTABLE);
        e.mem.set_u32(this + 0x48, 0x4800);
        e.mem.set_u32(this + 0x40, 0x4000);
        this
    }

    #[test]
    fn gamepad_clicks_the_selection_and_steps_the_slider() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(slot(4), |_, _| 0x7700u32.into_ret());
        e.register(GAMEPAD_IN_USE_00709D40, |_, _| 1u32.into_ret());
        e.register(TILE_IS_TRUE, |_, _| 1u32.into_ret());
        e.register(DEVICE_AXIS_00A23390, |_, _| (-20000i32).into_ret());
        e.register(MAGNITUDE_00408820, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).abs(),
            ..Ret::default()
        });
        e.register(CLOCK_00457FE0, |_, _| 1000u32.into_ret());
        // The value reads 3 before the set and 4 after it.
        let mut reads = 0;
        e.register_double(TILE_GET_FLOAT, move |_, _| {
            reads += 1;
            Ret {
                st0: if reads % 2 == 1 { 3.0 } else { 4.0 },
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_73f0, &args![this, 9u32, 0u32]).bool());
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x7700, 0xfcb]]);
        assert_eq!(calls(&e, slot(0xc)), vec![vec![this, 0xa, 0x7700]]);
        // The axis is negative: the value 3 goes to 4, which reads back changed: poked.
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x4800, 0x02d8, 4.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&e, TILE_POKE),
            vec![vec![0x4800, 0x0250, 1.0f32.to_bits()]]
        );
        assert_eq!(e.global::<u32>(NEXT_REPEAT_011DB004), 1500);
        // Held: not before the repeat time, then every 50 ms.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007e_73f0, &args![this, 1u32, 0u32]).bool());
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        e.register(CLOCK_00457FE0, |_, _| 1600u32.into_ret());
        e.call(0x007e_73f0, &args![this, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(NEXT_REPEAT_011DB004), 1650);
        // Inside the dead zone the repeat time clears.
        e.register(DEVICE_AXIS_00A23390, |_, _| 100u32.into_ret());
        e.call(0x007e_73f0, &args![this, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(NEXT_REPEAT_011DB004), 0);
    }

    #[test]
    fn gamepad_up_event_and_the_reset_callback() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] == 0x4000).into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007e_75d0, &args![this, 0xau32, 0u32]).bool());
        assert!(e.call(0x007e_75d0, &args![this, 0xbu32, 0u32]).bool());
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![1]]);
        assert_eq!(calls(&e, slot(0xc)), vec![vec![this, 6, 0]]);
        e.mem.set_u32(this + 0x40, 0x4100);
        assert!(!e.call(0x007e_75d0, &args![this, 0xbu32, 0u32]).bool());
        // The reset callback: a tile whose trait is false is left alone.
        e.set_global(USER_TRAIT_011DAF5C, 0x55u32);
        e.call_log = Some(vec![]);
        e.call(0x007e_7630, &args![0x4100u32]);
        assert!(calls(&e, TILE_SET_VALUE).is_empty());
    }
}
