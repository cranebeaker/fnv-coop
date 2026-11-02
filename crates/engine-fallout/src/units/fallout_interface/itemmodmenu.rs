//! `fallout/interface/menus/itemmodmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The item modification menu (`ItemModMenu`, class `0x425`): a `Menu` with
//! fourteen tiles from +0x28 and the item list (`lbItemModList`,
//! `ListBox<ItemChange *>`) at +0x60. The singleton `ItemModMenu::pMe` is
//! `011d9f54`. The constructor and the
//! destructor keep a C++ exception-unwinding frame (`FS:[0]`) that is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ItemModMenu::pMe` (Xbox PDB static): the menu singleton.
const ITEM_MOD_MENU: u32 = 0x011d_9f54;
/// The menu class number.
const ITEM_MOD_MENU_CLASS: u32 = 0x425;
/// `ItemModMenu`'s vtable.
const ITEM_MOD_MENU_VTABLE: u32 = 0x0107_3b7c;
/// The four trait ids the constructor looks up (the Xbox PDB lists the
/// statics `iUT_Title`, `iUT_Value`, `iUT_Enabled` and `iUT_IsItemModItem`;
/// which address is which is not confirmed, so they are named by the
/// address of the trait name each is looked up with).
const TRAIT_FROM_NAME_0106ED20: u32 = 0x011d_9f3c;
const TRAIT_FROM_NAME_0106ED18: u32 = 0x011d_9f44;
const TRAIT_FROM_NAME_01070244: u32 = 0x011d_9f48;
const TRAIT_FROM_NAME_01073B64: u32 = 0x011d_9f4c;
/// A byte static the constructor clears (the Xbox PDB has one `bool`
/// static, `bIsEquipped`; that this is it is not confirmed).
const FLAG_011D9F40: u32 = 0x011d_9f40;
/// The check that all fourteen tiles from +0x28 are set (`007273f0`, in
/// `recipemenu.cpp`).
const ALL_TILES_SET: u32 = 0x0072_73f0;

/// `Menu::Menu`.
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// `ListBox<ItemChange *>::ListBox<ItemChange *>`.
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::RemoveFadeControl` (`this`, trait).
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The interface manager singleton getter.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// `Tile::GetMenuByClass` (cdecl, one word).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this` + file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// Sets the byte at +0x1c of `this` to 1 (the code is shared with
/// `Calendar`; on a menu it is `bRenderedMenu`).
const SET_BYTE_AT_0X1C: u32 = 0x0086_7a20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// A list box call with one word (`007a1950(0, word)` behind it).
const LIST_BOX_WITH_FLAG: u32 = 0x007a_1930;
/// `Tile::Poke` (`this`, trait, float).
const TILE_POKE: u32 = 0x00a0_1290;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;

/// The menu file name, the missing-tile message and the list template.
const MENU_FILE: u32 = 0x0107_3c34;
const MISSING_TILE_MESSAGE: u32 = 0x0107_3be0;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_3bc4;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// Tile traits.
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_0XFA9: i32 = 0xfa9;

/// Offset of `lbItemModList` (Xbox PDB).
const MOD_LIST: u32 = 0x60;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(ITEM_MOD_MENU)
}

// Translated from 00783310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::ItemModMenu` (Xbox PDB): constructs the `Menu` base, stores
/// the class's vtable, constructs the item list at +0x60, looks up the four
/// trait ids the menu keeps in statics and clears the byte static
/// `011d9f40`. Returns `this`.
// Translation incomplete: 00783310 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn item_mod_menu_item_mod_menu(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), ITEM_MOD_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(MOD_LIST)]);
    for (name, slot) in [
        (0x0106_ed20u32, TRAIT_FROM_NAME_0106ED20),
        (0x0106_ed18, TRAIT_FROM_NAME_0106ED18),
        (0x0107_0244, TRAIT_FROM_NAME_01070244),
        (0x0107_3b64, TRAIT_FROM_NAME_01073B64),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    e.set_global(FLAG_011D9F40, 0u8);
    this
}

// Translated from 007833d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::GetClass` (Xbox PDB): the class number `0x425`.
pub fn item_mod_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    ITEM_MOD_MENU_CLASS
}

// Translated from 007833e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 14`.
pub fn item_mod_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0xe).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(0x28 + 4 * id as u32), tile);
    }
}

// Translated from 00783410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn item_mod_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00783650(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00783440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::Create` (Xbox PDB, cdecl, no arguments): deletes an open
/// menu of the class, reads the menu file (`01073c34`) and takes its menu
/// as `pMe`. For a menu of this class: attaches the tile, marks it rendered
/// (`00867a20`), checks all fourteen tiles (`007273f0`; else prints a
/// message and answers 0), sets the depth for the two stacking types, sets
/// up the item list on the tile at +0x2c, calls `007a1930(list, 0)`, pokes
/// 1.0 into the trait `011d9f44` of the tiles at +0x3c and +0x50 and prepares the menu
/// for showing (flag 1). Returns the root tile. A menu of another class is
/// deleted (when it has a tile) and the result is 0.
pub fn item_mod_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let class = fn_00783640(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(ITEM_MOD_MENU, menu);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_00783640(e) {
            return finish_create(e, root);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`item_mod_menu_create`] after the menu of the class was read.
fn finish_create(e: &mut Engine, root: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    let menu = me(e);
    if !e.call(ALL_TILES_SET, &args![menu]).bool() {
        e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE]);
        return 0;
    }
    let kind = e
        .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
        .f64();
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = e
            .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
            .f64();
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            SET_FLOAT_TRAIT,
            &args![root, TRAIT_DEPTH, x87_store32(depth), true],
        );
    }
    let container = e.mem.u32(me(e).wrapping_add(0x2c));
    let list = me(e).wrapping_add(MOD_LIST);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, ITEM_TEMPLATE_NAME],
    );
    let list = me(e).wrapping_add(MOD_LIST);
    e.call(LIST_BOX_WITH_FLAG, &args![list, 0u32]);
    for offset in [0x3cu32, 0x50] {
        let trait_id = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        let tile = e.mem.u32(me(e).wrapping_add(offset));
        e.call(TILE_POKE, &args![tile, trait_id, 1.0f32]);
    }
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 1u32]);
    root
}

// Translated from 00783640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x425`, without an object (no `this`).
pub fn fn_00783640(_e: &mut Engine) -> u32 {
    ITEM_MOD_MENU_CLASS
}

// Translated from 00783650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ItemModMenu` destructor: stores the class's vtable, removes the
/// fade control `0xfa9` of the tile at +0x48, clears `pMe`, destroys the
/// item list and runs the `Menu` destructor.
// Translation incomplete: 00783650 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_00783650(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ITEM_MOD_MENU_VTABLE);
    let tile = e.mem.u32(this.addr().wrapping_add(0x48));
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, TRAIT_0XFA9]);
    e.set_global(ITEM_MOD_MENU, 0u32);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(MOD_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

/// The item being modified, an `ItemChange *` static (the Xbox PDB's one
/// `ItemChange *` static of the class is `pBrokenItem`; that this is it is
/// not confirmed).
const MOD_ITEM: u32 = 0x011d_9f58;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The words at +0 and +4 of an object (`00559450`, `00726070`), the
/// identity `006815c0` and the word at +4 used on a tile + 4 (`007b5370`).
const WORD_AT_ZERO: u32 = 0x0055_9450;
const IDENTITY: u32 = 0x0068_15c0;
const WORD_AT_FOUR_007B5370: u32 = 0x007b_5370;
/// A tile list lookup (`006317a0`, `thiscall`: word; answers the address
/// of the element).
const TILE_LIST_LOOKUP: u32 = 0x0063_17a0;
/// Tile traits (`this` = the tile): set integer, set string, is visible.
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
/// `ItemChange`'s object (`0044ddc0`), `GetItemHealth`, `GetModSlots`,
/// `GetWorn`, `GetFullName`, `GetIconOne`, `SetIsModding`,
/// `RemoveIsModding` (Xbox PDB).
const ITEM_OBJECT: u32 = 0x0044_ddc0;
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
const ITEM_GET_MOD_SLOTS: u32 = 0x004b_d820;
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const ITEM_GET_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_GET_ICON_ONE: u32 = 0x004b_e200;
const ITEM_SET_IS_MODDING: u32 = 0x004b_db40;
const ITEM_REMOVE_IS_MODDING: u32 = 0x004b_dc70;
/// `ItemChange::ItemChange` (Xbox PDB; `this`, object, count).
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// The weapon's mod of a slot (`004bd570`, `thiscall`: slot bit) and the
/// effect of a slot's mod (`004bd880`, slot).
const WEAPON_MOD: u32 = 0x004b_d570;
const WEAPON_MOD_EFFECT: u32 = 0x004b_d880;
/// `TESObjectWEAP::GetModEffectValue` (Xbox PDB; `this`, effect, flag; ST0).
const WEAPON_MOD_EFFECT_VALUE: u32 = 0x004b_cf60;
/// The dimmed alpha of a fitted mod (`float`).
const FITTED_ALPHA: u32 = 0x0107_3c54;
/// `0.0` (`double`) and the alpha below which an entry cannot be chosen.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ENABLED_ALPHA: u32 = 0x0101_e568;
/// `100.0` (`double`).
const FULL_HEALTH: u32 = 0x0101_7a40;
/// `ListBox<ItemChange *>`'s `NumUnfiltered`, `AddItem`, `RemoveItem` (Xbox
/// PDB), its highlighted item (`007a1910`) and its sort (`007314c0`).
const LIST_NUM_UNFILTERED: u32 = 0x0071_ae60;
const LIST_ADD_ITEM: u32 = 0x0071_a320;
const LIST_REMOVE_ITEM: u32 = 0x0071_a5c0;
const LIST_SELECTED_ITEM: u32 = 0x007a_1910;
const LIST_SORT: u32 = 0x0073_14c0;
/// `Interface::PlayMenuSound` and `InterfaceManager::PlayMenuSound` (Xbox
/// PDB, cdecl).
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
/// `Actor::QueueEquipObject` and `Actor::QueueUnEquipObject` (Xbox PDB).
const QUEUE_EQUIP: u32 = 0x0088_c650;
const QUEUE_UNEQUIP: u32 = 0x0088_c790;
/// `Interface::SetInventoryMenuVisible` (Xbox PDB, cdecl) and the interface
/// call `007046f0` (cdecl, three words) after opening the menu.
const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
const INTERFACE_007046F0: u32 = 0x0070_46f0;
/// `BSAudio::QInstance`, `BSAudio::GetSoundHandleByName`,
/// `BSSoundHandle::Play` (Xbox PDB) and the handle's destructor body.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// The sound of a fitted mod.
const MOD_SOUND: u32 = 0x0107_3c58;
/// `InventoryMenu::UpdateInventoryMenu` (Xbox PDB, no arguments).
const UPDATE_INVENTORY_MENU: u32 = 0x0078_2a90;
/// The inventory helpers of `ItemModItem`: `GetInventoryChanges` (cdecl),
/// `GetObjectInList` (`this`, object, flag, flag), `GetItemModGroup`
/// (`this`, object, list) (Xbox PDB).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_OBJECT_IN_LIST: u32 = 0x004b_fba0;
const GET_ITEM_MOD_GROUP: u32 = 0x004d_4ca0;
/// `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ExtraDataList`'s constructor, `GetCount`, `SetCount`,
/// `DuplicateExtraListForContainer`, `GetWeaponModFlags`,
/// `SetWeaponModSlot`, `CompareListForContainer`, `GetHealth`, `SetHealth`
/// (Xbox PDB) and its clear `00419f80` (flag).
const EXTRA_CONSTRUCTOR: u32 = 0x0041_0360;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_SET_COUNT: u32 = 0x0041_9ad0;
const EXTRA_DUPLICATE: u32 = 0x0041_2380;
const EXTRA_GET_MOD_FLAGS: u32 = 0x0042_e560;
const EXTRA_SET_MOD_SLOT: u32 = 0x0042_e380;
const EXTRA_COMPARE: u32 = 0x0041_26c0;
const EXTRA_GET_HEALTH: u32 = 0x0041_86f0;
const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
const EXTRA_CLEAR: u32 = 0x0041_9f80;
/// The simple list's constructor, head removal, add, removal (`00905330`)
/// and destructor.
const SIMPLE_LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const SIMPLE_LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_ADD_OBJECT: u32 = 0x005a_e3d0;
const SIMPLE_LIST_REMOVE: u32 = 0x0090_5330;
const SIMPLE_LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// `MiscStatManager::Increment` (Xbox PDB, cdecl).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// `_ftol2_sse`, the float rounding `004bd510` (value, step) and the health
/// rounding `00476b20` (cdecl float).
const FTOL: u32 = 0x00ec_62c0;
const ROUND_TO_MULTIPLE: u32 = 0x004b_d510;
const HEALTH_ROUND: u32 = 0x0047_6b20;
/// `CombatFormulas::CalcWeaponDamageForDisplay` and
/// `CombatFormulas::CalcArmorRating` (Xbox PDB, cdecl; ST0), and the armour
/// value `004be060` of an object (`thiscall`; ST0).
const CALC_WEAPON_DAMAGE: u32 = 0x0064_50f0;
const CALC_ARMOR_RATING: u32 = 0x0064_6360;
const ARMOR_VALUE: u32 = 0x004b_e060;
/// The name getter `00408da0` (used on the object + 0x30).
const GET_NAME: u32 = 0x0040_8da0;
/// A setting's string value (`00403df0`, `ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// The interface's gamepad test and its focus target
/// (`InterfaceManager::SetCurrentFocusTarget`, Xbox PDB).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
const SET_FOCUS_TARGET: u32 = 0x0071_5860;
/// The tutorial message shower.
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// A setting's float address (`00403e20`, `ECX` = the setting) and the
/// Pip-Boy knob (`FOPipboyManager::MoveKnob`, Xbox PDB, through `00704370`).
const SETTING_FLOAT: u32 = 0x0040_3e20;
const PIPBOY_MANAGER: u32 = 0x0070_4370;
const MOVE_KNOB: u32 = 0x007f_8610;
const KNOB_STEP_SETTING: u32 = 0x011d_b2f0;
const KNOB_TIME_SETTING: u32 = 0x011d_b358;
/// `DoEnter`'s static: the last highlighted row.
const LAST_ROW: u32 = 0x011d_9fd0;
/// The empty text, the description signature and the trait name and float
/// of the description tile.
const EMPTY_TEXT: u32 = 0x0101_1584;
const DESCRIPTION_SIGNATURE: u32 = 0x4353_4544;
const DESCRIPTION_TRAIT_NAME: u32 = 0x0107_27cc;
const DESCRIPTION_FLOAT: u32 = 0x0102_3cd8;
/// The language setting and the string it is compared with, the label
/// child's name.
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const LANGUAGE_DEFAULT: u32 = 0x0101_8348;
const LABEL_CHILD_NAME: u32 = 0x0107_3c6c;
/// `Tile::GetChildByName` (Xbox PDB) and the string comparison (cdecl).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `_ftol2` on an `f64`.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// The element of a list node.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(IDENTITY, &args![node]).u32();
    e.mem.u32(slot)
}

/// The first extra data list of an item, or 0 without one.
fn first_extra_list(e: &mut Engine, item: u32) -> u32 {
    if e.call(WORD_AT_ZERO, &args![item]).u32() == 0 {
        return 0;
    }
    let list = e.call(WORD_AT_ZERO, &args![item]).u32();
    node_item(e, list)
}

fn mod_item(e: &Engine) -> u32 {
    e.global::<u32>(MOD_ITEM)
}

/// Sets the trait of an entry's tile through `006317a0` (the tile's word at
/// +4).
fn set_entry_trait(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    let word = e.call(WORD_AT_ZERO, &args![tile.wrapping_add(4)]).u32();
    let slot = e
        .call(TILE_LIST_LOOKUP, &args![tile.wrapping_add(4), word])
        .u32();
    let target = e.mem.u32(slot);
    e.call(SET_TRAIT_FROM_INT, &args![target, trait_id, value]);
}

// Translated from 007836d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::InitFunc` (Xbox PDB, cdecl: tile, item): sets the entry's
/// trait `0xfaa` to `0xf`, shows it when its row (`0xfac`) is not negative,
/// sets `0xfa3` to 1.0, and dims it (`0xfa9` to the float `01073c54`) when
/// the item is a mod already fitted in one of the weapon's mod slots.
pub fn item_mod_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    if tile == 0 || item == 0 {
        return;
    }
    let health = e.call(ITEM_GET_HEALTH, &args![item, 1u32]).f64();
    let _ = x87_store32(health);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfaau32, 0xfu32]);
    let row = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
    // FCOMP; TEST AH,1; JNZ: hidden when below 0.0 or unordered.
    let visible = !(row < e.global::<f64>(ZERO_DOUBLE) || row.is_nan());
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, 0xfa3u32, u32::from(visible)],
    );
    e.call(
        SET_FLOAT_TRAIT,
        &args![tile, 0xfa3u32, 1.0f32.to_bits(), 1u32],
    );
    let weapon = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
    if e.call(ITEM_GET_MOD_SLOTS, &args![mod_item(e)]).u8() == 0 {
        return;
    }
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let fitted = [1u32, 2, 4].into_iter().any(|bit| {
        u32::from(e.call(ITEM_GET_MOD_SLOTS, &args![mod_item(e)]).u8()) & bit != 0
            && object == e.call(WEAPON_MOD, &args![weapon, bit]).u32()
    });
    if fitted {
        let alpha = x87_quiet_bits(e.global::<u32>(FITTED_ALPHA));
        let slot = e
            .call(WORD_AT_FOUR_007B5370, &args![tile.wrapping_add(4)])
            .u32();
        let target = e.mem.u32(slot);
        e.call(SET_FLOAT_TRAIT, &args![target, 0xfa9u32, alpha, 1u32]);
    }
}

/// Whether an entry's tile (through `006317a0`) is visible.
fn entry_visible(e: &mut Engine, entry: u32) -> bool {
    let tile = e.mem.u32(entry);
    let word = e.call(WORD_AT_ZERO, &args![tile.wrapping_add(4)]).u32();
    let slot = e
        .call(TILE_LIST_LOOKUP, &args![tile.wrapping_add(4), word])
        .u32();
    let target = e.mem.u32(slot);
    e.call(TILE_IS_VISIBLE, &args![target]).bool()
}

// Translated from 00783810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, the list's sort): visible entries
/// first (-1 when only `a` is visible, 1 when only `b` is, else 0).
pub fn fn_00783810(e: &mut Engine, a: u32, b: u32) -> i32 {
    let first = entry_visible(e, a);
    let second = entry_visible(e, b);
    if first && !second {
        -1
    } else if !first && second {
        1
    } else {
        0
    }
}

// Translated from 007838a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::DoClick` (Xbox PDB). The close button (`0xc`) re-equips a
/// weapon that was unequipped for modding and returns to the inventory.
/// The list (`0xf`): an enabled entry other than the item being modified is
/// fitted (`ItemModItem`): the mod is removed from the player (vtable
/// `+0x17c`), the mod sound plays, the entry leaves the list, the menu is
/// rebuilt, and a worn weapon updates the player and the inventory.
// Translation incomplete: 007838a0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn item_mod_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let base = this.addr();
    let tile = if tile == 0 && (0..0xe).contains(&id) {
        e.mem.u32(base.wrapping_add(0x28 + 4 * id as u32))
    } else {
        tile
    };
    if id == 0xc {
        e.call(PLAY_MENU_SOUND, &args![0x24u32]);
        if e.global::<u8>(FLAG_011D9F40) == 1 {
            let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
            let extra = node_item(e, list);
            let count = e.call(WORD_AT_FOUR, &args![mod_item(e)]).u32();
            let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
            let player = e.global::<u32>(PLAYER);
            e.call(
                QUEUE_EQUIP,
                &args![player, object, count, extra, 1u32, 0u32, 1u32],
            );
            e.call(ITEM_REMOVE_IS_MODDING, &args![mod_item(e)]);
            e.set_global(FLAG_011D9F40, 0u8);
        }
        e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 0u32]);
        return;
    }
    if id != 0xf {
        return;
    }
    let slot = e
        .call(WORD_AT_FOUR_007B5370, &args![tile.wrapping_add(4)])
        .u32();
    let target = e.mem.u32(slot);
    let alpha = e.call(GET_FLOAT_TRAIT, &args![target, 0xfa9u32]).f64();
    // FCOMP; TEST AH,5; JP: a dimmed entry (below, ordered) is refused.
    if alpha < e.global::<f64>(ENABLED_ALPHA) {
        return;
    }
    let selected = e.call(LIST_SELECTED_ITEM, &args![base + MOD_LIST]).u32();
    if selected == mod_item(e) {
        return;
    }
    let item = e.call(LIST_SELECTED_ITEM, &args![base + MOD_LIST]).u32();
    let extra = first_extra_list(e, item);
    if !item_mod_menu_item_mod_item(e, this, item) {
        return;
    }
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let player = e.global::<u32>(PLAYER);
    e.vcall(
        player,
        0x17c,
        &args![object, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_GET_SOUND, &args![audio, handle, MOD_SOUND, 0x121u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
    let highlighted = e.vcall(base + MOD_LIST, 4, &args![]).u32();
    e.call(LIST_REMOVE_ITEM, &args![base + MOD_LIST, highlighted]);
    item_mod_menu_init_item_mod_menu(e, mod_item(e), 0);
    if e.call(ITEM_GET_WORN, &args![item, 0u32]).bool() {
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3c4, &args![]);
        e.call(UPDATE_INVENTORY_MENU, &args![]);
    }
}

// Translated from 00783af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::ItemModItem` (Xbox PDB): fits the mod `item` to one free
/// mod slot of the weapon being modified: splits one weapon off its extra
/// list (a new list with count 1, the old count reduced), marks the slot
/// whose mod is `item`'s object, merges the new list into an equal one of
/// the player's stack (or adds it, or for a single weapon marks the old
/// list itself), adds the mod's health effect (`0xa`) to the list's health
/// and counts stat `0x21`. Answers whether a slot was fitted.
// Translation incomplete: 00783af0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn item_mod_menu_item_mod_item(e: &mut Engine, _this: Ptr, item: u32) -> bool {
    let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
    let player = e.global::<u32>(PLAYER);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    let entry = e
        .call(GET_OBJECT_IN_LIST, &args![changes, object, 1u32, 0u32])
        .u32();
    let old_extra = first_extra_list(e, mod_item(e));
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let new_extra = if block != 0 {
        e.call(EXTRA_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(8, |e, lists| {
        e.mem.set_u32(lists.addr(), new_extra);
        e.mem.set_u32(lists.addr() + 4, old_extra);
        let new_slot = lists.addr();
        let old_slot = lists.addr() + 4;
        let mut merged = false;
        let mut fitted = false;
        let weapon = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        let mod_object = e.call(ITEM_OBJECT, &args![item]).u32();
        let count = i32::from(e.call(EXTRA_GET_COUNT, &args![old_extra]).u16() as i16);
        e.call(EXTRA_DUPLICATE, &args![new_extra, old_extra]);
        e.call(EXTRA_SET_COUNT, &args![new_extra, 1u32]);
        let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
        e.call(SIMPLE_LIST_REMOVE_HEAD, &args![list]);
        if count > 1 {
            e.call(EXTRA_SET_COUNT, &args![old_extra, count - 1]);
        }
        let mut slot = 0u8;
        for bit in [1u8, 2, 4] {
            let flags = e.call(EXTRA_GET_MOD_FLAGS, &args![new_extra]).u8();
            if flags & bit == 0
                && e.call(WEAPON_MOD, &args![weapon, u32::from(bit)]).u32() == mod_object
            {
                e.call(EXTRA_SET_MOD_SLOT, &args![new_extra, u32::from(bit)]);
                slot = bit;
                fitted = true;
                break;
            }
        }
        let mut node = e.call(WORD_AT_ZERO, &args![entry]).u32();
        while node != 0 {
            if node_item(e, node) == 0 {
                break;
            }
            let other = node_item(e, node);
            let same = !e
                .call(EXTRA_COMPARE, &args![other, new_extra, 1u32, 0u32])
                .bool()
                && {
                    let other = node_item(e, node);
                    !e.call(EXTRA_COMPARE, &args![new_extra, other, 1u32, 0u32])
                        .bool()
                };
            if same {
                let other = node_item(e, node);
                let other_count = e.call(EXTRA_GET_COUNT, &args![other]).u16() as i16;
                let other = node_item(e, node);
                e.call(
                    EXTRA_SET_COUNT,
                    &args![other, i32::from(other_count).wrapping_add(1)],
                );
                let element = e.call(IDENTITY, &args![node]).u32();
                let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
                e.call(LIST_ADD_OBJECT, &args![list, element]);
                merged = true;
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let mut result = 0u32;
        if !merged {
            if count > 1 {
                let list = e.call(WORD_AT_ZERO, &args![entry]).u32();
                e.call(LIST_ADD_OBJECT, &args![list, new_slot]);
                let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
                e.call(LIST_ADD_OBJECT, &args![list, new_slot]);
                result = e.mem.u32(new_slot);
            } else {
                e.call(EXTRA_SET_MOD_SLOT, &args![old_extra, u32::from(slot)]);
                let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
                e.call(LIST_ADD_OBJECT, &args![list, old_slot]);
                result = e.mem.u32(old_slot);
            }
        } else if count == 1 {
            let list = e.call(WORD_AT_ZERO, &args![entry]).u32();
            e.call(SIMPLE_LIST_REMOVE, &args![list, old_slot]);
        }
        if merged || count == 1 {
            let new_extra = e.mem.u32(new_slot);
            e.call(EXTRA_CLEAR, &args![new_extra, 0u32]);
            let new_extra = e.mem.u32(new_slot);
            if new_extra != 0 {
                e.vcall(new_extra, 0, &args![1u32]);
            }
            if result == e.mem.u32(new_slot) {
                result = 0;
            }
        }
        if e.call(WEAPON_MOD_EFFECT, &args![weapon, u32::from(slot)])
            .u32()
            == 0xa
            && result != 0
        {
            let health = e.call(EXTRA_GET_HEALTH, &args![result]).f64();
            let value = e
                .call(WEAPON_MOD_EFFECT_VALUE, &args![weapon, 0xau32, 0u32])
                .f64();
            let health = x87_store32(x87_add(value, health));
            e.call(EXTRA_SET_HEALTH, &args![result, health]);
        }
        if fitted {
            e.call(MISC_STAT_INCREMENT, &args![0x21u32]);
        }
        fitted
    })
}

// Translated from 00783ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::DoEnter` (Xbox PDB): entering the list (`0xf`) moves the
/// Pip-Boy knob when the highlighted row changed, clears the description
/// and, for a highlighted item with a weapon being modified, shows the
/// item's description (`DESC`), its icon and the description trait
/// (`010727cc`) float.
pub fn item_mod_menu_do_enter(e: &mut Engine, this: Ptr, id: i32, _unused_2: u32) {
    let base = this.addr();
    if id != 0xf {
        return;
    }
    let item = e.call(LIST_SELECTED_ITEM, &args![base + MOD_LIST]).u32();
    let row = if item != 0 {
        let tile = e.vcall(base + MOD_LIST, 4, &args![]).u32();
        let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
        ftol(e, value)
    } else {
        0
    };
    if e.global::<i32>(LAST_ROW) != row {
        let step = e.call(SETTING_FLOAT, &args![KNOB_STEP_SETTING]).u32();
        let step = if e.global::<i32>(LAST_ROW) < row {
            x87_quiet_bits(e.mem.u32(step))
        } else {
            x87_store32(-x87_load32(e.mem.u32(step)))
        };
        let time = e.call(SETTING_FLOAT, &args![KNOB_TIME_SETTING]).u32();
        let time = x87_quiet_bits(e.mem.u32(time));
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let pipboy = e.call(PIPBOY_MANAGER, &args![manager]).u32();
        e.call(MOVE_KNOB, &args![pipboy, 0u32, step, time, 1u32]);
        e.set_global(LAST_ROW, row);
    }
    let description = e.mem.u32(base + 0x5c);
    e.call(
        SET_STRING_TRAIT,
        &args![description, 0xfc4u32, EMPTY_TEXT, 1u32],
    );
    if item == 0 || mod_item(e) == 0 {
        return;
    }
    let object = e.call(ITEM_OBJECT, &args![item]).u32();
    let text = e
        .vcall(
            object.wrapping_add(0x74),
            0x10,
            &args![0u32, DESCRIPTION_SIGNATURE],
        )
        .u32();
    let description = e.mem.u32(base + 0x5c);
    e.call(SET_STRING_TRAIT, &args![description, 0xfc4u32, text, 1u32]);
    let icon = e.call(ITEM_GET_ICON_ONE, &args![item, 0u32]).u32();
    let image = e.mem.u32(me(e).wrapping_add(0x34));
    e.call(SET_STRING_TRAIT, &args![image, 0xfccu32, icon, 1u32]);
    let value = x87_quiet_bits(e.global::<u32>(DESCRIPTION_FLOAT));
    let trait_id = e
        .call(TILE_TEXT_TO_TRAIT, &args![DESCRIPTION_TRAIT_NAME])
        .u32();
    let tile = e.mem.u32(base + 0x54);
    e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, value, 1u32]);
}

// Translated from 00784050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::DoLeave` (Xbox PDB): leaving the list (`0xf`) hides the
/// tile at +0x4c and, with more than one unfiltered entry, the tile at
/// +0x48.
pub fn item_mod_menu_do_leave(e: &mut Engine, this: Ptr, id: i32, _unused_2: u32) {
    let base = this.addr();
    if id != 0xf {
        return;
    }
    let tile = e.mem.u32(base + 0x4c);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfa3u32, 0u32]);
    if e.call(LIST_NUM_UNFILTERED, &args![base + MOD_LIST]).u32() > 1 {
        let tile = e.mem.u32(base + 0x48);
        e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfa3u32, 0u32]);
    }
}

// Translated from 007840a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::DoGamepadUpEvent` (Xbox PDB): buttons 6 and `0xa` play menu
/// sound 3 and click `0xc` (close); answers whether it handled the button.
pub fn item_mod_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    button: i32,
    _unused_2: u32,
) -> bool {
    if button != 6 && button != 0xa {
        return false;
    }
    e.call(INTERFACE_PLAY_MENU_SOUND, &args![3u32]);
    e.vcall(this.addr(), 0xc, &args![0xcu32, 0u32]);
    true
}

/// Adds `item` to the mod list with its full name and sets the entry's
/// trait `011d9f4c` to 1; answers the entry.
fn add_mod_entry(e: &mut Engine, item: u32) -> u32 {
    let name = e.call(ITEM_GET_FULL_NAME, &args![item]).u32();
    let list = me(e) + MOD_LIST;
    let entry = e
        .call(LIST_ADD_ITEM, &args![list, item, name, 0u32, 0u32])
        .u32();
    let trait_id = e.global::<u32>(TRAIT_FROM_NAME_01073B64);
    e.call(SET_TRAIT_FROM_INT, &args![entry, trait_id, 1u32]);
    entry
}

/// A new `ItemChange` of `count` of `object` (0x0c bytes; 0 when the
/// allocation fails).
fn new_item_change(e: &mut Engine, object: impl FnOnce(&mut Engine) -> u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block == 0 {
        return 0;
    }
    let object = object(e);
    e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, object, 1u32])
        .u32()
}

// Translated from 007840f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::Update` (Xbox PDB, no arguments): fills the list for the
/// weapon being modified: an entry for the mod of each of its mod slots,
/// then the player's mods of its group (`GetItemModGroup`) other than
/// those, one entry per item of each stack (copies carry the stack's extra
/// lists); relabels for another language.
// Translation incomplete: 007840f0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn item_mod_menu_update(e: &mut Engine) {
    if mod_item(e) == 0 {
        return;
    }
    let weapon = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
    let mut mods = [0u32; 3];
    let slots = e.call(ITEM_GET_MOD_SLOTS, &args![mod_item(e)]).u8() as i8;
    if slots != 0 {
        for (i, bit) in [1u32, 2, 4].into_iter().enumerate() {
            if (slots as i32 as u32) & bit == 0 {
                continue;
            }
            mods[i] = e.call(WEAPON_MOD, &args![weapon, bit]).u32();
            let mod_object = mods[i];
            let item = new_item_change(e, |_| mod_object);
            let entry = add_mod_entry(e, item);
            set_entry_trait(e, entry, 0xfa3, 1);
        }
    }
    e.with_stack(8, |e, group| {
        e.call(SIMPLE_LIST_CONSTRUCTOR, &args![group]);
        let player = e.global::<u32>(PLAYER);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        e.call(GET_ITEM_MOD_GROUP, &args![changes, object, group]);
        let mut node = group.addr();
        while node != 0 && node_item(e, node) != 0 {
            let mut k = 0i32;
            loop {
                let stack = node_item(e, node);
                if k >= e.call(WORD_AT_FOUR, &args![stack]).i32() {
                    break;
                }
                let stack = node_item(e, node);
                let is_fitted = |e: &mut Engine| {
                    mods.iter().any(|&fitted| {
                        let object = e.call(ITEM_OBJECT, &args![stack]).u32();
                        object == fitted
                    })
                };
                if !is_fitted(e) {
                    if k == 0 {
                        add_mod_entry(e, stack);
                    } else {
                        let copy = new_item_change(e, |e| {
                            let stack = node_item(e, node);
                            e.call(ITEM_OBJECT, &args![stack]).u32()
                        });
                        let stack = node_item(e, node);
                        let mut extra = e.call(WORD_AT_ZERO, &args![stack]).u32();
                        while extra != 0 && node_item(e, extra) != 0 {
                            let element = e.call(IDENTITY, &args![extra]).u32();
                            let list = e.call(WORD_AT_ZERO, &args![copy]).u32();
                            e.call(LIST_ADD_OBJECT, &args![list, element]);
                            extra = e.call(WORD_AT_FOUR, &args![extra]).u32();
                        }
                        add_mod_entry(e, copy);
                    }
                }
                k += 1;
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let language = e.call(SETTING_STRING, &args![LANGUAGE_SETTING]).u32();
        if e.call(STRING_COMPARE, &args![language, LANGUAGE_DEFAULT])
            .i32()
            != 0
        {
            relabel(e);
        }
        e.call(SIMPLE_LIST_DESTRUCTOR, &args![group]);
    });
}

/// The labels [`item_mod_menu_update`] sets for another language.
fn relabel(e: &mut Engine) {
    let text = e.call(SETTING_STRING, &args![0x011d_4378u32]).u32();
    let menu_tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    let child = e
        .call(TILE_GET_CHILD_BY_NAME, &args![menu_tile, LABEL_CHILD_NAME])
        .u32();
    e.call(SET_STRING_TRAIT, &args![child, 0xfc4u32, text, 1u32]);
    let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
    for offset in [0x38u32, 0x4c] {
        let text = e.call(SETTING_STRING, &args![0x011d_4da4u32]).u32();
        let tile = e.mem.u32(me(e).wrapping_add(offset));
        e.call(SET_STRING_TRAIT, &args![tile, title, text, 1u32]);
    }
    for (offset, setting) in [(0x54u32, 0x011d_1f2cu32), (0x58, 0x011d_29f4)] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let tile = e.mem.u32(me(e).wrapping_add(offset));
        e.call(SET_STRING_TRAIT, &args![tile, 0xfc4u32, text, 1u32]);
    }
}

// Translated from 007846a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::ResetMenu` (Xbox PDB, no arguments): clears the list
/// (vtable `+0x1c`), the item being modified, leaves the list, removes the
/// tile at +0x48's fade control `0xfa9` and hides it.
pub fn item_mod_menu_reset_menu(e: &mut Engine) {
    e.vcall(me(e) + MOD_LIST, 0x1c, &args![]);
    e.set_global(MOD_ITEM, 0u32);
    e.vcall(me(e), 0x14, &args![0xfu32, 0u32]);
    let tile = e.mem.u32(me(e) + 0x48);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, TRAIT_0XFA9]);
    let tile = e.mem.u32(me(e) + 0x48);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfa3u32, 0u32]);
}

// Translated from 00784710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ItemModMenu::InitItemModMenu` (Xbox PDB, cdecl: item, flag): with a
/// menu, takes `item` as the item being modified (when given); with the
/// flag, a worn weapon is unequipped for modding (the byte static set). Sets
/// the health bar, the damage (weapons, `CalcWeaponDamageForDisplay`) or
/// armour rating line, the name; for a given item rebuilds the list
/// (`Update`) with the icon; sorts the list, focuses its first entry with a
/// gamepad, enters it, and for a given item opens the menu and shows
/// tutorial 6.
pub fn item_mod_menu_init_item_mod_menu(e: &mut Engine, item: u32, flag: u8) {
    if flag != 0 {
        e.set_global(FLAG_011D9F40, 0u8);
    }
    if me(e) == 0 {
        return;
    }
    if item != 0 {
        e.set_global(MOD_ITEM, item);
    }
    if mod_item(e) != 0 && flag != 0 && e.call(ITEM_GET_WORN, &args![mod_item(e), 0u32]).bool() {
        e.call(ITEM_SET_IS_MODDING, &args![mod_item(e)]);
        let list = e.call(WORD_AT_ZERO, &args![mod_item(e)]).u32();
        let extra = node_item(e, list);
        let count = e.call(WORD_AT_FOUR, &args![mod_item(e)]).u32();
        let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        let player = e.global::<u32>(PLAYER);
        e.call(
            QUEUE_UNEQUIP,
            &args![player, object, count, extra, 0u32, 0u32, 1u32],
        );
        e.set_global(FLAG_011D9F40, 1u8);
    }
    let health = e.call(ITEM_GET_HEALTH, &args![mod_item(e), 1u32]).f64();
    let ratio = x87_store32(x87_div(health, e.global::<f64>(FULL_HEALTH)));
    let bar = e.mem.u32(me(e) + 0x38);
    e.call(SET_FLOAT_TRAIT, &args![bar, 0x1004u32, ratio, 1u32]);
    let bar = e.mem.u32(me(e) + 0x38);
    e.call(SET_TRAIT_FROM_INT, &args![bar, 0xfa3u32, 1u32]);
    let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
    let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
    let value_trait = TRAIT_FROM_NAME_0106ED18;
    if e.call(FORM_TYPE, &args![object]).u32() == 0x28 {
        let text = e.call(SETTING_STRING, &args![0x011d_2c7cu32]).u32();
        let tile = e.mem.u32(me(e) + 0x3c);
        e.call(SET_STRING_TRAIT, &args![tile, title, text, 1u32]);
        let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        // 00784856: the cdecl `006450f0` reads four words (its [EBP+0x14] too); the fourth is
        // the 0 pushed at 0078483d (ADD ESP,0xc pops only three, the slot is reused for
        // `004bd510`'s first word).
        let damage = e
            .call(CALC_WEAPON_DAMAGE, &args![object, ratio, item, 0u32])
            .f64();
        let rounded = e
            .call(
                ROUND_TO_MULTIPLE,
                &args![x87_store32(damage), 1.0f32.to_bits()],
            )
            .f64();
        let damage = ftol(e, rounded);
        let trait_id = e.global::<u32>(value_trait);
        let tile = e.mem.u32(me(e) + 0x3c);
        e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, damage]);
        let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        let name = e.call(GET_NAME, &args![object.wrapping_add(0x30)]).u32();
        let tile = e.mem.u32(me(e) + 0x28);
        e.call(SET_STRING_TRAIT, &args![tile, 0xfc4u32, name, 1u32]);
    } else {
        let text = e.call(SETTING_STRING, &args![0x011d_20c4u32]).u32();
        let tile = e.mem.u32(me(e) + 0x3c);
        e.call(SET_STRING_TRAIT, &args![tile, title, text, 1u32]);
        let object = e.call(ITEM_OBJECT, &args![mod_item(e)]).u32();
        let armor = e.call(ARMOR_VALUE, &args![object]).f64();
        let armor = ftol(e, armor);
        // 007848f1: the cdecl `00646360` takes (armour, ratio); its second word is the
        // ratio pushed at 007848d2 (ADD ESP,4 pops only the first, the slot is reused
        // for `00476b20`).
        let rating = e.call(CALC_ARMOR_RATING, &args![armor, ratio]).f64();
        let rounded = e.call(HEALTH_ROUND, &args![x87_store32(rating)]).f64();
        let rating = ftol(e, rounded);
        let trait_id = e.global::<u32>(value_trait);
        let tile = e.mem.u32(me(e) + 0x3c);
        e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, rating]);
    }
    if item != 0 {
        e.vcall(me(e) + MOD_LIST, 0x1c, &args![]);
        let icon = e.call(ITEM_GET_ICON_ONE, &args![mod_item(e), 0u32]).u32();
        let image = e.mem.u32(me(e) + 0x34);
        e.call(SET_STRING_TRAIT, &args![image, 0xfccu32, icon, 1u32]);
        item_mod_menu_update(e);
    }
    let list = me(e) + MOD_LIST;
    e.call(
        LIST_SORT,
        &args![list, 0x0078_36d0u32, 0u32, 0x7fff_ffffu32],
    );
    e.vcall(me(e) + MOD_LIST, 0x20, &args![0x0078_3810u32]);
    if item != 0 && e.call(GAMEPAD_IN_USE, &args![]).bool() {
        // 007849ca: the list's +0x14 (`007a30a0`, RET 8) takes the 1 and 0 pushed last;
        // the 0xfc3 and 1 pushed first are the last two words of `00715860` (RET 0xc).
        let first = e.vcall(me(e) + MOD_LIST, 0x14, &args![1u32, 0u32]).u32();
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(SET_FOCUS_TARGET, &args![manager, first, 0xfc3u32, 1u32]);
    }
    let highlighted = e.vcall(me(e) + MOD_LIST, 4, &args![]).u32();
    e.vcall(me(e), 0x10, &args![0xfu32, highlighted]);
    if item != 0 {
        e.call(INTERFACE_007046F0, &args![1u32, 0u32, 0u32]);
        let class = fn_00783640(e);
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SHOW_TUTORIAL,
            &args![manager.wrapping_add(0x4d4), 6u32, class, 0x1f4u32],
        );
    }
}

/// A form's type byte (`this` only).
const FORM_TYPE: u32 = 0x0040_1170;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0078_3310, item_mod_menu_item_mod_menu(Ptr) -> Ptr),
        entry!(0x0078_33d0, item_mod_menu_get_class(Ptr) -> u32),
        entry!(0x0078_33e0, item_mod_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x0078_3410, item_mod_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0078_3440, item_mod_menu_create() -> u32),
        entry!(0x0078_3640, fn_00783640() -> u32),
        entry!(0x0078_3650, fn_00783650(Ptr)),
        entry!(0x0078_36d0, item_mod_menu_init_func(u32, u32)),
        entry!(0x0078_3810, fn_00783810(u32, u32) -> i32),
        entry!(0x0078_38a0, item_mod_menu_do_click(Ptr, i32, u32)),
        entry!(0x0078_3af0, item_mod_menu_item_mod_item(Ptr, u32) -> bool),
        entry!(0x0078_3ed0, item_mod_menu_do_enter(Ptr, i32, u32)),
        entry!(0x0078_4050, item_mod_menu_do_leave(Ptr, i32, u32)),
        entry!(
            0x0078_40a0,
            item_mod_menu_do_gamepad_up_event(Ptr, i32, u32) -> bool
        ),
        entry!(0x0078_40f0, item_mod_menu_update()),
        entry!(0x0078_46a0, item_mod_menu_reset_menu()),
        entry!(0x0078_4710, item_mod_menu_init_item_mod_menu(u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_builds_the_menu_and_looks_up_four_traits() {
        let mut e = Engine::new();
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        stub(&mut e, &[MENU_CONSTRUCTOR, LIST_BOX_CONSTRUCTOR]);
        e.set_global(FLAG_011D9F40, 1u8);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_3310, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ITEM_MOD_MENU_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_CONSTRUCTOR), vec![vec![this + 0x60]]);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED20), 0xed20);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED18), 0xed18);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01070244), 0x0244);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01073B64), 0x3b64);
        assert_eq!(e.global::<u8>(FLAG_011D9F40), 0);
    }

    #[test]
    fn class_number_and_tiles() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0078_33d0, &args![0u32]).u32(), 0x425);
        assert_eq!(e.call(0x0078_3640, &args![]).u32(), 0x425);
        let this = e.mem.alloc(0x90);
        e.call(0x0078_33e0, &args![this, 13i32, 0x99u32]);
        e.call(0x0078_33e0, &args![this, 14i32, 0x98u32]);
        assert_eq!(e.mem.u32(this + 0x28 + 52), 0x99);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }

    #[test]
    fn deleting_destructor_tears_down_and_frees() {
        let mut e = Engine::new();
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x8c);
        e.set_global(ITEM_MOD_MENU, this);
        e.mem.set_u32(this + 0x48, 0x480);
        stub(
            &mut e,
            &[
                TILE_REMOVE_FADE_CONTROL,
                LIST_BOX_DESTRUCTOR,
                MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_3410, &args![this, 1u32]).u32(), this);
        assert_eq!(e.global::<u32>(ITEM_MOD_MENU), 0);
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x480, 0xfa9]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_DESTRUCTOR), vec![vec![this + 0x60]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0078_3650, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_sets_up_a_complete_menu() {
        let mut e = Engine::new();
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.set_global(STACKING_TYPE_ONE, 102.0f64);
        e.set_global(TRAIT_FROM_NAME_0106ED18, 0x77u32);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x0078_33d0);
        for i in 0..13 {
            e.mem.set_u32(this + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this);
        answer(&mut e, ALL_TILES_SET, 1);
        e.register(GET_FLOAT_TRAIT, |_, _| Ret {
            st0: 102.0,
            ..Ret::default()
        });
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| Ret {
            st0: 9.0,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                SET_FLOAT_TRAIT,
                LIST_BOX_INITIALIZE,
                LIST_BOX_WITH_FLAG,
                TILE_POKE,
                MENU_PREP_FOR_VISIBILITY,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_3440, &args![]).u32(), 0x4444);
        assert_eq!(e.global::<u32>(ITEM_MOD_MENU), this);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![this, 0x4444, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x4444, 0xfad, 9.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![this + 0x60, 0x104, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_WITH_FLAG), vec![vec![this + 0x60, 0]]);
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![
                vec![0x114, 0x77, 1.0f32.to_bits()],
                vec![0x128, 0x77, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![this, 1]]);
    }

    #[test]
    fn create_deletes_the_old_menu_and_reports_missing_tiles() {
        let mut e = Engine::new();
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        let old = e.mem.alloc(8);
        let old_vtable = e.mem.alloc(8);
        e.mem.set_u32(old, old_vtable);
        e.mem.set_u32(old_vtable, 0x00c0_0000);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x0078_33d0);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, old);
        answer(&mut e, TILE_GET_MENU, this);
        answer(&mut e, ALL_TILES_SET, 0);
        stub(
            &mut e,
            &[
                0x00c0_0000,
                INTERFACE_SINGLETON,
                TILE_READER,
                TILE_READ_FILE,
                MENU_SET_MENU_TILE,
                SET_BYTE_AT_0X1C,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_3440, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![old, 1]]);
        assert_eq!(calls_to(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x425]]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut e = Engine::new();
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable, 0x00c0_0000);
        e.mem.set_u32(vtable + 0x34, 0x00c0_0034);
        answer(&mut e, 0x00c0_0034, 0x3f5);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_GET_MENU, this);
        answer(&mut e, WORD_AT_FOUR, 1);
        stub(
            &mut e,
            &[
                0x00c0_0000,
                INTERFACE_SINGLETON,
                TILE_READER,
                TILE_READ_FILE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_3440, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![this, 1]]);
    }
}

#[cfg(test)]
mod n054_tests {
    use super::*;

    type Log = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, callee: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_st0(e: &mut Engine, address: u32, st0: f64) {
        e.register_double(address, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn real_ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32).into_ret()
        });
    }

    fn pages(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            if !e.mem.is_mapped(address) {
                e.map(address & !0xfff, 0x1000);
            }
        }
    }

    /// A menu with its 14 tiles (`0x100 + 4 * i`) as `pMe`, its vtable
    /// slots `0x00c0_0000 + offset` and the list's `0x00c1_0000 + offset`.
    fn menu(e: &mut Engine) -> Ptr {
        pages(e, &[ITEM_MOD_MENU, PLAYER, ZERO_DOUBLE, FULL_HEALTH]);
        let this = e.mem.alloc(0x90);
        for i in 0..14 {
            e.mem.set_u32(this + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        for slot in 0..0x10 {
            e.mem.set_u32(vtable + 4 * slot, 0x00c0_0000 + 4 * slot);
        }
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x60, list_vtable);
        for slot in 0..0x10 {
            e.mem
                .set_u32(list_vtable + 4 * slot, 0x00c1_0000 + 4 * slot);
        }
        e.set_global(ITEM_MOD_MENU, this);
        Ptr::new(this)
    }

    #[test]
    fn init_func_dims_a_fitted_mod() {
        let mut e = Engine::new();
        menu(&mut e);
        pages(&mut e, &[FITTED_ALPHA]);
        e.set_global(FITTED_ALPHA, 0.5f32.to_bits());
        e.set_global(MOD_ITEM, 0x7000u32);
        answer_st0(&mut e, ITEM_GET_HEALTH, 50.0);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 2.0);
        e.register(ITEM_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        answer(&mut e, ITEM_GET_MOD_SLOTS, 6);
        e.register(WEAPON_MOD, |_, a| {
            (if a[1] == 4 { 0x8100u32 } else { 0x8200 }).into_ret()
        });
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5600);
        answer(&mut e, WORD_AT_FOUR_007B5370, slot);
        stub(&mut e, &[SET_TRAIT_FROM_INT, SET_FLOAT_TRAIT]);
        e.call_log = Some(vec![]);
        e.call(0x0078_36d0, &args![0x700u32, 0x8000u32]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x700, 0xfaa, 0xf], vec![0x700, 0xfa3, 1]]
        );
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![
                vec![0x700, 0xfa3, 1.0f32.to_bits(), 1],
                vec![0x5600, 0xfa9, 0.5f32.to_bits(), 1]
            ]
        );
        assert_eq!(
            calls_to(&e, WEAPON_MOD),
            vec![vec![0x7100, 2], vec![0x7100, 4]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_36d0, &args![0u32, 0x8000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn visible_entries_sort_first() {
        let mut e = Engine::new();
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, 0x700);
        e.mem.set_u32(b, 0x800);
        answer(&mut e, WORD_AT_ZERO, 0);
        e.register(TILE_LIST_LOOKUP, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, a[0] - 4);
            slot.into_ret()
        });
        e.register(TILE_IS_VISIBLE, |_, a| u32::from(a[0] == 0x700).into_ret());
        assert_eq!(e.call(0x0078_3810, &args![a, b]).i32(), -1);
        assert_eq!(e.call(0x0078_3810, &args![b, a]).i32(), 1);
        assert_eq!(e.call(0x0078_3810, &args![a, a]).i32(), 0);
    }

    #[test]
    fn close_re_equips_the_weapon() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[FLAG_011D9F40]);
        e.set_global(FLAG_011D9F40, 1u8);
        e.set_global(MOD_ITEM, 0x7000u32);
        answer(&mut e, WORD_AT_ZERO, 0x7200);
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.mem.map(0x7000, 0x1000);
        e.mem.set_u32(0x7200, 0x7300);
        answer(&mut e, WORD_AT_FOUR, 1);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        stub(
            &mut e,
            &[
                PLAY_MENU_SOUND,
                QUEUE_EQUIP,
                ITEM_REMOVE_IS_MODDING,
                SET_INVENTORY_MENU_VISIBLE,
            ],
        );
        let player = e.global::<u32>(PLAYER);
        e.call_log = Some(vec![]);
        e.call(0x0078_38a0, &args![this, 0xci32, 0u32]);
        assert_eq!(
            calls_to(&e, QUEUE_EQUIP),
            vec![vec![player, 0x7100, 1, 0x7300, 1, 0, 1]]
        );
        assert_eq!(e.global::<u8>(FLAG_011D9F40), 0);
        assert_eq!(
            calls_to(&e, SET_INVENTORY_MENU_VISIBLE),
            vec![vec![1, 0, 0]]
        );
    }

    #[test]
    fn click_refuses_a_dimmed_entry() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[ENABLED_ALPHA]);
        e.mem.set_f64(ENABLED_ALPHA, 255.0);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5600);
        answer(&mut e, WORD_AT_FOUR_007B5370, slot);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 127.0);
        e.call_log = Some(vec![]);
        e.call(0x0078_38a0, &args![this, 0xfi32, 0u32]);
        // The tile of id 0xf is out of the 14 tiles: the null tile + 4.
        assert_eq!(calls_to(&e, WORD_AT_FOUR_007B5370), vec![vec![4]]);
        assert!(calls_to(&e, LIST_SELECTED_ITEM).is_empty());
    }

    #[test]
    fn fitting_a_mod_to_a_single_weapon() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(MOD_ITEM, 0x7000u32);
        e.register(ITEM_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        answer(&mut e, GET_INVENTORY_CHANGES, 0x9000);
        answer(&mut e, GET_OBJECT_IN_LIST, 0x9100);
        answer(&mut e, WORD_AT_ZERO, 0);
        answer(&mut e, OPERATOR_NEW, 0x9200);
        answer(&mut e, EXTRA_CONSTRUCTOR, 0x9200);
        answer(&mut e, EXTRA_GET_COUNT, 1);
        answer(&mut e, EXTRA_GET_MOD_FLAGS, 1);
        e.register(WEAPON_MOD, |_, a| {
            (if a[1] == 2 { 0x8100u32 } else { 0 }).into_ret()
        });
        answer(&mut e, WEAPON_MOD_EFFECT, 0);
        let extra = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(extra, vtable);
        e.mem.set_u32(vtable, 0x00c5_0000);
        answer(&mut e, EXTRA_CONSTRUCTOR, extra);
        stub(
            &mut e,
            &[
                EXTRA_DUPLICATE,
                EXTRA_SET_COUNT,
                SIMPLE_LIST_REMOVE_HEAD,
                EXTRA_SET_MOD_SLOT,
                LIST_ADD_OBJECT,
                SIMPLE_LIST_REMOVE,
                EXTRA_CLEAR,
                0x00c5_0000,
                MISC_STAT_INCREMENT,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_3af0, &args![this, 0x8000u32]).bool());
        assert_eq!(calls_to(&e, EXTRA_SET_MOD_SLOT)[0], vec![extra, 2]);
        // A single weapon: the old list is marked and the new one deleted.
        assert_eq!(calls_to(&e, EXTRA_SET_MOD_SLOT)[1], vec![0, 2]);
        assert_eq!(calls_to(&e, 0x00c5_0000), vec![vec![extra, 1]]);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x21]]);
        // No free slot holds the mod: nothing is fitted.
        answer(&mut e, EXTRA_GET_MOD_FLAGS, 7);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0078_3af0, &args![this, 0x8000u32]).bool());
        assert!(calls_to(&e, MISC_STAT_INCREMENT).is_empty());
    }

    #[test]
    fn entering_clears_the_description() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[LAST_ROW]);
        answer(&mut e, LIST_SELECTED_ITEM, 0);
        stub(&mut e, &[SET_STRING_TRAIT]);
        e.call_log = Some(vec![]);
        e.call(0x0078_3ed0, &args![this, 0xfi32, 0u32]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![0x134, 0xfc4, EMPTY_TEXT, 1]]
        );
        // The row changed (-1 -> 0): the knob moves down.
        e.set_global(LAST_ROW, 1i32);
        let step = e.mem.alloc(4);
        e.mem.set_f32(step, 2.0);
        answer(&mut e, SETTING_FLOAT, step);
        answer(&mut e, INTERFACE_SINGLETON, 0x6000);
        answer(&mut e, PIPBOY_MANAGER, 0x6100);
        stub(&mut e, &[MOVE_KNOB]);
        e.call_log = Some(vec![]);
        e.call(0x0078_3ed0, &args![this, 0xfi32, 0u32]);
        assert_eq!(
            calls_to(&e, MOVE_KNOB),
            vec![vec![0x6100, 0, (-2.0f32).to_bits(), 2.0f32.to_bits(), 1]]
        );
        assert_eq!(e.global::<i32>(LAST_ROW), 0);
    }

    #[test]
    fn leave_and_gamepad_up() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(&mut e, &[SET_TRAIT_FROM_INT]);
        answer(&mut e, LIST_NUM_UNFILTERED, 2);
        e.call_log = Some(vec![]);
        e.call(0x0078_4050, &args![this, 0xfi32, 0u32]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x124, 0xfa3, 0], vec![0x120, 0xfa3, 0]]
        );
        stub(&mut e, &[INTERFACE_PLAY_MENU_SOUND, 0x00c0_000c]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0078_40a0, &args![this, 6i32, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![this.addr(), 0xc, 0]]);
        assert!(!e.call(0x0078_40a0, &args![this, 7i32, 0u32]).bool());
    }

    #[test]
    fn update_lists_the_slot_mods() {
        let mut e = Engine::new();
        menu(&mut e);
        e.set_global(MOD_ITEM, 0x7000u32);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, ITEM_GET_MOD_SLOTS, 1);
        answer(&mut e, WEAPON_MOD, 0x8100);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, ITEM_CHANGE_CONSTRUCTOR, 0x9000);
        answer(&mut e, ITEM_GET_FULL_NAME, 0x5500);
        answer(&mut e, LIST_ADD_ITEM, 0x9100);
        answer(&mut e, WORD_AT_ZERO, 0);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5600);
        answer(&mut e, TILE_LIST_LOOKUP, slot);
        e.register(IDENTITY, |_, a| a[0].into_ret());
        answer(&mut e, SETTING_STRING, 0x5700);
        answer(&mut e, STRING_COMPARE, 0);
        stub(
            &mut e,
            &[
                SET_TRAIT_FROM_INT,
                SIMPLE_LIST_CONSTRUCTOR,
                GET_INVENTORY_CHANGES,
                GET_ITEM_MOD_GROUP,
                SIMPLE_LIST_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_40f0, &args![]);
        assert_eq!(
            calls_to(&e, ITEM_CHANGE_CONSTRUCTOR),
            vec![vec![0x9000, 0x8100, 1]]
        );
        let trait_id = e.global::<u32>(TRAIT_FROM_NAME_01073B64);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x9100, trait_id, 1], vec![0x5600, 0xfa3, 1]]
        );
        e.set_global(MOD_ITEM, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_40f0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn reset_clears_the_menu() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set_global(MOD_ITEM, 0x7000u32);
        stub(
            &mut e,
            &[
                0x00c1_001c,
                0x00c0_0014,
                TILE_REMOVE_FADE_CONTROL,
                SET_TRAIT_FROM_INT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_46a0, &args![]);
        assert_eq!(e.global::<u32>(MOD_ITEM), 0);
        assert_eq!(calls_to(&e, 0x00c0_0014), vec![vec![this.addr(), 0xf, 0]]);
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x120, 0xfa9]]
        );
    }

    #[test]
    fn init_for_armour_without_a_new_item() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[FLAG_011D9F40]);
        e.mem.set_f64(FULL_HEALTH, 100.0);
        e.set_global(MOD_ITEM, 0x7000u32);
        answer_st0(&mut e, ITEM_GET_HEALTH, 50.0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x18);
        answer(&mut e, SETTING_STRING, 0x5700);
        answer_st0(&mut e, ARMOR_VALUE, 12.0);
        answer_st0(&mut e, CALC_ARMOR_RATING, 30.0);
        answer_st0(&mut e, HEALTH_ROUND, 31.0);
        real_ftol(&mut e);
        answer(&mut e, 0x00c1_0004, 0x900);
        stub(
            &mut e,
            &[
                SET_FLOAT_TRAIT,
                SET_TRAIT_FROM_INT,
                SET_STRING_TRAIT,
                LIST_SORT,
                0x00c1_0020,
                0x00c0_0010,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_4710, &args![0u32, 0u32]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x110, 0x1004, 0.5f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, CALC_ARMOR_RATING),
            vec![vec![12, 0.5f32.to_bits()]]
        );
        let value = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x110, 0xfa3, 1], vec![0x114, value, 31]]
        );
        assert_eq!(
            calls_to(&e, 0x00c0_0010),
            vec![vec![this.addr(), 0xf, 0x900]]
        );
        assert_eq!(
            calls_to(&e, LIST_SORT),
            vec![vec![this.addr() + 0x60, 0x0078_36d0, 0, 0x7fff_ffff]]
        );
        // Without a menu only the flag is cleared.
        e.set_global(FLAG_011D9F40, 1u8);
        e.set_global(ITEM_MOD_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_4710, &args![0u32, 1u32]);
        assert_eq!(e.global::<u8>(FLAG_011D9F40), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A weapon (form type 0x28): `CalcWeaponDamageForDisplay` gets four words, the last
    /// the 0 the exe pushes at 0078483d, and its answer is rounded to a multiple of 1.0.
    #[test]
    fn init_for_a_weapon_gives_the_damage_display_four_words() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        pages(&mut e, &[FLAG_011D9F40]);
        e.mem.set_f64(FULL_HEALTH, 100.0);
        e.set_global(MOD_ITEM, 0x7000u32);
        answer_st0(&mut e, ITEM_GET_HEALTH, 50.0);
        answer(&mut e, ITEM_OBJECT, 0x7100);
        answer(&mut e, FORM_TYPE, 0x28);
        answer(&mut e, SETTING_STRING, 0x5700);
        answer_st0(&mut e, CALC_WEAPON_DAMAGE, 20.0);
        answer_st0(&mut e, ROUND_TO_MULTIPLE, 21.0);
        answer(&mut e, GET_NAME, 0x5800);
        real_ftol(&mut e);
        answer(&mut e, 0x00c1_0004, 0x900);
        stub(
            &mut e,
            &[
                SET_FLOAT_TRAIT,
                SET_TRAIT_FROM_INT,
                SET_STRING_TRAIT,
                LIST_SORT,
                0x00c1_0020,
                0x00c0_0010,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0078_4710, &args![0u32, 0u32]);
        assert_eq!(
            calls_to(&e, CALC_WEAPON_DAMAGE),
            vec![vec![0x7100, 0.5f32.to_bits(), 0, 0]]
        );
        assert_eq!(
            calls_to(&e, ROUND_TO_MULTIPLE),
            vec![vec![20.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        let value = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x110, 0xfa3, 1], vec![0x114, value, 21]]
        );
        assert_eq!(
            calls_to(&e, 0x00c0_0010),
            vec![vec![this.addr(), 0xf, 0x900]]
        );
    }
}
