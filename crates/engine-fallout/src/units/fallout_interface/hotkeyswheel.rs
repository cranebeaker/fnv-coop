//! `fallout/interface/hotkeyswheel.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The hot keys wheel is the HUD wheel of eight hotkey tiles (`HK_Item_0` to
//! `HK_Item_7`) under one parent tile. Its object is 0x34 bytes: the parent
//! tile, the eight tile pointers, a flag that a hotkey is being selected,
//! the last highlighted hotkey and the two user trait ids the highlight
//! writes (`_SelectedHotkey` and `_SelectedText`). The player's hotkeys are
//! entries of the `InventoryChanges` the player holds (reached through
//! `004bf220` of the player at `011dea3c`); the wheel itself only shows
//! them and assigns, uses or highlights one.
//!
//! The functions with a C++ exception-unwinding frame for a temporary
//! string (`00701610`, `007017b0`) carry a `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// The player character object (the word at `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The object whose word at +0x14 is a tick count (`00825c00` reads it).
const TICK_OBJECT: u32 = 0x011f_6394;
/// The hotkey `MoveHighlight` last asked to highlight (a static `int`,
/// initially -1) and the tick count when that changed.
const LAST_ASKED: u32 = 0x0119_f33c;
const LAST_ASKED_TICK: u32 = 0x011d_88f4;
/// The `float` 50.0: ticks the highlight waits before following the pad.
const HIGHLIGHT_DELAY: u32 = 0x0101_b268;
/// A `float` constant (2.0) passed to the message call.
const MESSAGE_SECONDS: u32 = 0x0101_62c0;
/// The object (word at `011c3b3c`) that model paths are handed to.
const MODEL_OBJECT: u32 = 0x011c_3b3c;

/// Returns the word at +4 of its argument (a list node's next pointer, an
/// inventory entry's count).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// Returns its `this` (the address of the first member).
const THIS_ITEM: u32 = 0x0068_15c0;
/// Returns the first word of its object (the first list of an inventory
/// entry, and a `BSStringT`'s characters, share this getter).
const FIRST_WORD: u32 = 0x0055_9450;
/// The object of an inventory entry: the word at +8 (`0044ddc0`).
const ENTRY_OBJECT: u32 = 0x0044_ddc0;
/// The form type of an object: the byte at +4 (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// Sets a float trait from an integer (tile, trait id, integer).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// Sets a string trait (tile, trait id, string, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// The float of a trait, in `ST0` (tile, trait id).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `Tile::GetString` (engine map): the string of a trait (tile, trait id).
const GET_STRING_TRAIT: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map): whether a trait is set (tile, trait id).
const IS_TRAIT_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetChildByName` (engine map): tile, name.
const GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::AddUserTrait` (engine map), cdecl (name, id).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::TextToTrait` (engine map), cdecl, one name.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `InterfaceManager::CheckMenuButton` (engine map), cdecl (button, flag).
const CHECK_MENU_BUTTON: u32 = 0x0071_7a40;
/// Whether the gamepad interface is in use (`004b71d0`; `00709d40` calls it).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
const GAMEPAD_IN_USE_WRAPPER: u32 = 0x0070_9d40;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in `ST0` to an integer.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// The tick count of the object in `ECX` (the word at +0x14).
const TICK_COUNT: u32 = 0x0082_5c00;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Prints a message (one string), cdecl.
const PRINT_MESSAGE: u32 = 0x005b_5e40;

/// `BSStringT` constructor / destructor / `sprintf` / character getter /
/// setter (from characters) / append (characters).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_C_STR: u32 = 0x0055_9450;
const STRING_SET: u32 = 0x0043_8390;
const STRING_APPEND: u32 = 0x0040_4820;

/// `InventoryChanges::GetInventoryChanges` (engine map), cdecl (actor).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// The inventory changes' hotkey entry for a hotkey number (`004bf4b0`).
const HOTKEY_ENTRY: u32 = 0x004b_f4b0;
/// An array of the eight hotkey entries (`004bf660`, taking 0 and 7); it is
/// freed with `00401030`.
const HOTKEY_ENTRIES: u32 = 0x004b_f660;
const FREE_ARRAY: u32 = 0x0040_1030;
/// `InventoryChanges::RemoveHotKeyItem` (engine map; entry, hotkey) and
/// `InventoryChanges::SetHotKeyItem` (engine map; entry, extra data,
/// hotkey).
const REMOVE_HOTKEY_ITEM: u32 = 0x004b_f330;
const SET_HOTKEY_ITEM: u32 = 0x004b_f800;
/// `ItemChange::GetFullName` (engine map) and `ItemChange::GetWorn` (engine
/// map; takes a flag).
const ITEM_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
/// The entry's destructor (`004459e0`; takes the delete flag).
const ENTRY_DESTROY: u32 = 0x0044_59e0;
/// `BGSEquipType::GetEquipType` (engine map; cdecl) and
/// `BGSEquipType::GetEquipTypeIconFilename` (engine map; cdecl).
const GET_EQUIP_TYPE: u32 = 0x0047_9430;
const GET_EQUIP_TYPE_ICON_FILENAME: u32 = 0x0047_9540;
/// `TESBipedModelForm::GetBipedTESModel` (engine map; cdecl, form and
/// actor).
const GET_BIPED_MODEL: u32 = 0x0048_1090;
/// Takes a model path and hands it on (`0045a5e0`, on the object in
/// `011c3b3c`).
const MODEL_RELEASE: u32 = 0x0045_a5e0;
/// `ModelLoader::LoadFile` (engine map; path and five flags).
const MODEL_LOAD_FILE: u32 = 0x0044_7080;
/// Whether the form is the item the hands are empty for / whether the form
/// is usable (`004c0bf0`).
const FORM_IS_USABLE: u32 = 0x004c_0bf0;
/// `Actor::GetAnimAction` (engine map).
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `BGSDefaultObjectManager::GetDefaultObject` (engine map; cdecl).
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// `Actor::QueueEquipObject` and `Actor::QueueUnEquipObject` (engine map;
/// object, count, extra data, 0, 0, 1).
const QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
const QUEUE_UNEQUIP_OBJECT: u32 = 0x0088_c790;
/// `PlayerCharacter::UpdatePlayer3d` (engine map).
const UPDATE_PLAYER_3D: u32 = 0x0095_0030;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map).
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// The player call after an assignment (`008aded0`; form, 1, 0).
const AFTER_ASSIGN: u32 = 0x008a_ded0;
/// The setting object (`011d2af0`) whose text is the "can't now" message,
/// its getter (`004c69f0`) and the message call (`007052f0`).
const MESSAGE_SETTING: u32 = 0x011d_2af0;
const MESSAGE_TEXT: u32 = 0x004c_69f0;
const SHOW_MESSAGE: u32 = 0x0070_52f0;

/// `TESBoundObject`'s and `TESModelTextureSwap`'s RTTI type descriptors.
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_MODEL_TEXTURE_SWAP: u32 = 0x0118_31c4;

/// `"HotKeyWheel: Hot key wheel being initialized with a NULL parent tile.
/// This...should not...be!"`.
const NULL_PARENT_MESSAGE: u32 = 0x0106_eaa0;
/// `"HK_Item_%d"`, `"_SelectedHotkey"`, `"_SelectedText"`.
const ITEM_NAME_FORMAT: u32 = 0x0106_ea94;
const SELECTED_HOTKEY_NAME: u32 = 0x0106_ea84;
const SELECTED_TEXT_NAME: u32 = 0x0106_ea74;
/// `"Interface\Icons\"`, `"_HotKeyIcon"`, `"_HotKeyAssigned"` and `"%s (%d)"`.
const ICON_DIRECTORY: u32 = 0x0106_eb24;
const HOTKEY_ICON_NAME: u32 = 0x0106_eb18;
const HOTKEY_ASSIGNED_NAME: u32 = 0x0106_eb08;
const NAME_AND_COUNT_FORMAT: u32 = 0x0106_eb00;
/// The empty string.
const EMPTY_STRING: u32 = 0x0101_1584;

/// Trait ids (the `Tile::enumTrait` numbers the PC build uses).
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STRING: i32 = 0xfc4;

/// The number of hotkey tiles.
const HOTKEY_COUNT: u32 = 8;
/// The hotkey (1) that is never shown or used.
const SKIPPED_HOTKEY: i32 = 1;
/// Offset of the eight tile pointers.
const TILES: u32 = 4;

layout! {
    /// `HotKeysWheel` (Xbox PDB), 0x34 bytes; the eight tile pointers
    /// (`pHotkeyTilesA`) are at +4.
    pub struct HotKeysWheel: 0x34 {
        /// `pWheel` (Xbox PDB): the parent tile.
        0x00 pWheel: Ptr,
        /// `bSelectingHotkey` (Xbox PDB).
        0x24 bSelectingHotkey: bool,
        /// `iLastHighlightedHotkey` (Xbox PDB).
        0x28 iLastHighlightedHotkey: i32,
        /// `iUT_HighlightedHotkey` (Xbox PDB): a user trait id.
        0x2C iUT_HighlightedHotkey: i32,
        /// `iUT_HighlightedText` (Xbox PDB): a user trait id.
        0x30 iUT_HighlightedText: i32,
    }
}

/// A `float` copied through `FLD` / `FSTP`: a signaling NaN comes out quiet.
fn through_x87(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// The tile of hotkey `index` (the pointer array has no bounds check).
fn hotkey_tile(e: &Engine, this: Ptr<HotKeysWheel>, index: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(TILES)
            .wrapping_add(index.wrapping_mul(4)),
    )
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

fn set_string(e: &mut Engine, tile: u32, trait_id: i32, text: u32) {
    e.call(SET_STRING_TRAIT, &args![tile, trait_id, text, true]);
}

fn check_menu_button(e: &mut Engine, button: u32) -> i32 {
    e.call(CHECK_MENU_BUTTON, &args![button, 0u32]).i32()
}

/// The first extra data list of an inventory entry: the first item of the
/// list its first word points at, 0 for none.
fn first_extra_data(e: &mut Engine, entry: u32) -> u32 {
    if e.call(FIRST_WORD, &args![entry]).u32() == 0 {
        0
    } else {
        let list = e.call(FIRST_WORD, &args![entry]).u32();
        let node = e.call(THIS_ITEM, &args![list]).u32();
        e.mem.u32(node)
    }
}

/// The biped model of an object: `GetBipedTESModel` of the object and the
/// player, or the object cast to `TESModelTextureSwap` when that is null.
fn biped_model(e: &mut Engine, entry: u32) -> u32 {
    let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
    let player: u32 = e.global(PLAYER);
    let model = e.call(GET_BIPED_MODEL, &args![form, player]).u32();
    if model != 0 {
        return model;
    }
    let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0i32, TYPE_BOUND_OBJECT, TYPE_MODEL_TEXTURE_SWAP, 0i32],
    )
    .u32()
}

// Translated from 00701610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::Initialize` (Xbox PDB): with a parent tile, stores it,
/// finds its eight children `HK_Item_0` to `HK_Item_7` (a null pointer for
/// a missing one), registers the user traits `_SelectedHotkey` and
/// `_SelectedText`, clears the selecting flag, and sets the first trait to
/// -1 and the second to the empty string on the parent. Without a parent it
/// prints a warning and leaves the object untouched.
// Translation incomplete: 00701610 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn hotkeys_wheel_initialize(e: &mut Engine, this: Ptr<HotKeysWheel>, parent: Ptr) {
    // The exe's frame (`SUB ESP,0x10` under the exception record, then the record link):
    // the text at `EBP-0x14` (+0xc).
    e.with_stack(0x20, |e, frame| {
        let text = frame.addr() + 0xc;
        e.call(STRING_CONSTRUCT, &args![text]);
        if parent.is_null() {
            e.call(PRINT_MESSAGE, &args![NULL_PARENT_MESSAGE]);
            e.call(STRING_DESTROY, &args![text]);
            return;
        }
        e.set(this, HotKeysWheel::pWheel, parent);
        for index in 0..HOTKEY_COUNT {
            e.call(STRING_FORMAT, &args![text, ITEM_NAME_FORMAT, index]);
            let name = e.call(STRING_C_STR, &args![text]).u32();
            let wheel = e.get(this, HotKeysWheel::pWheel);
            let tile = e.call(GET_CHILD_BY_NAME, &args![wheel, name]).u32();
            e.mem.set_u32(this.addr() + TILES + 4 * index, tile);
        }
        let id = e
            .call(ADD_USER_TRAIT, &args![SELECTED_HOTKEY_NAME, -1i32])
            .i32();
        e.set(this, HotKeysWheel::iUT_HighlightedHotkey, id);
        let id = e
            .call(ADD_USER_TRAIT, &args![SELECTED_TEXT_NAME, -1i32])
            .i32();
        e.set(this, HotKeysWheel::iUT_HighlightedText, id);
        e.set(this, HotKeysWheel::bSelectingHotkey, false);
        let wheel = e.get(this, HotKeysWheel::pWheel).addr();
        let hotkey_trait = e.get(this, HotKeysWheel::iUT_HighlightedHotkey);
        set_trait_from_int(e, wheel, hotkey_trait, -1);
        let wheel = e.get(this, HotKeysWheel::pWheel).addr();
        let text_trait = e.get(this, HotKeysWheel::iUT_HighlightedText);
        set_string(e, wheel, text_trait, EMPTY_STRING);
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 00701740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::GetVisible` (Xbox PDB): whether the parent tile's
/// visible trait (`0xfa3`) is set.
pub fn hotkeys_wheel_get_visible(e: &mut Engine, this: Ptr<HotKeysWheel>) -> bool {
    let wheel = e.get(this, HotKeysWheel::pWheel).addr();
    e.call(IS_TRAIT_TRUE, &args![wheel, TRAIT_VISIBLE]).bool()
}

// Translated from 00701760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::SetVisible` (Xbox PDB): showing first refreshes the
/// hotkey list; then sets the parent tile's visible trait to 1 or 0.
pub fn hotkeys_wheel_set_visible(e: &mut Engine, this: Ptr<HotKeysWheel>, visible: bool) {
    if visible {
        hotkeys_wheel_update_hotkey_list(e, this);
        let wheel = e.get(this, HotKeysWheel::pWheel).addr();
        set_trait_from_int(e, wheel, TRAIT_VISIBLE, 1);
    } else {
        let wheel = e.get(this, HotKeysWheel::pWheel).addr();
        set_trait_from_int(e, wheel, TRAIT_VISIBLE, 0);
    }
}

// Translated from 007017b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::UpdateHotkeyList` (Xbox PDB): with inventory changes for
/// the player, fills each hotkey tile except number 1 from the player's
/// hotkey entries. An assigned tile gets the icon
/// (`Interface\Icons\<equip type icon>`, user trait `_HotKeyIcon`), the
/// user trait `_HotKeyAssigned` set to 1 and its string trait set to the
/// item's name (`"<name> (<count>)"` for a count above 1). An empty tile
/// gets `_HotKeyAssigned` 0 and an empty string. Every entry is destroyed
/// at the end (also number 1) and the array freed.
// Translation incomplete: 007017b0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn hotkeys_wheel_update_hotkey_list(e: &mut Engine, this: Ptr<HotKeysWheel>) {
    let player: u32 = e.global(PLAYER);
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    // The exe's frame (`SUB ESP,0x30` under the exception record, then the record link):
    // the text at `EBP-0x14` (+0x2c).
    e.with_stack(0x40, |e, frame| {
        let text = frame.addr() + 0x2c;
        e.call(STRING_CONSTRUCT, &args![text]);
        if inventory != 0 {
            let entries = e.call(HOTKEY_ENTRIES, &args![inventory, 0u32, 7u32]).u32();
            for index in 0..HOTKEY_COUNT {
                if index as i32 == SKIPPED_HOTKEY {
                    continue;
                }
                let entry = e.mem.u32(entries + 4 * index);
                let tile = hotkey_tile(e, this, index);
                if entry != 0 {
                    e.call(STRING_SET, &args![text, ICON_DIRECTORY]);
                    let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
                    let equip_type = e.call(GET_EQUIP_TYPE, &args![form]).u32();
                    let icon = e
                        .call(GET_EQUIP_TYPE_ICON_FILENAME, &args![equip_type])
                        .u32();
                    e.call(STRING_APPEND, &args![text, icon]);
                    let path = e.call(STRING_C_STR, &args![text]).u32();
                    let icon_trait = e.call(TEXT_TO_TRAIT, &args![HOTKEY_ICON_NAME]).i32();
                    set_string(e, tile, icon_trait, path);
                    let assigned = e.call(TEXT_TO_TRAIT, &args![HOTKEY_ASSIGNED_NAME]).i32();
                    set_trait_from_int(e, tile, assigned, 1);
                    let count = e.call(WORD_AT_FOUR, &args![entry]).i32();
                    if count > 1 {
                        let count = e.call(WORD_AT_FOUR, &args![entry]).u32();
                        let name = e.call(ITEM_FULL_NAME, &args![entry]).u32();
                        e.call(
                            STRING_FORMAT,
                            &args![text, NAME_AND_COUNT_FORMAT, name, count],
                        );
                    } else {
                        let name = e.call(ITEM_FULL_NAME, &args![entry]).u32();
                        e.call(STRING_SET, &args![text, name]);
                    }
                    let shown = e.call(STRING_C_STR, &args![text]).u32();
                    set_string(e, tile, TRAIT_STRING, shown);
                } else {
                    let assigned = e.call(TEXT_TO_TRAIT, &args![HOTKEY_ASSIGNED_NAME]).i32();
                    set_trait_from_int(e, tile, assigned, 0);
                    set_string(e, tile, TRAIT_STRING, EMPTY_STRING);
                }
            }
            for index in 0..HOTKEY_COUNT {
                let entry = e.mem.u32(entries + 4 * index);
                if entry != 0 {
                    e.call(ENTRY_DESTROY, &args![entry, 1u32]);
                }
            }
            e.call(FREE_ARRAY, &args![entries]);
        }
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007019e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns an inventory entry to the hotkey being selected (a virtual-slot
/// handler of the wheel's menu, called with the clicked entry). Needs a
/// non-null entry, the selecting flag and inventory changes for the player.
///
/// The entry currently on that hotkey (`004bf4b0`) is removed first
/// (`RemoveHotKeyItem`), and the model path of its object (biped model, or
/// the object cast to `TESModelTextureSwap`; virtual slot `+0x14`) goes to
/// the model object `0045a5e0`. The new entry and its first extra data list
/// are set on the hotkey (`SetHotKeyItem`) and its model path is loaded
/// (`ModelLoader::LoadFile`, path then 0, 1, 0, 0, 0). The wheel's list is
/// refreshed, the player is told (`008aded0` with the object, 1, 0), the
/// selecting flag is cleared and the removed entry destroyed.
pub fn fn_007019e0(e: &mut Engine, this: Ptr<HotKeysWheel>, entry: u32) {
    let player: u32 = e.global(PLAYER);
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    if entry == 0 || !e.get(this, HotKeysWheel::bSelectingHotkey) || inventory == 0 {
        return;
    }
    let hotkey = e.get(this, HotKeysWheel::iLastHighlightedHotkey);
    let old = e.call(HOTKEY_ENTRY, &args![inventory, hotkey]).u32();
    let extra = first_extra_data(e, entry);
    if old != 0 {
        let hotkey = e.get(this, HotKeysWheel::iLastHighlightedHotkey);
        e.call(REMOVE_HOTKEY_ITEM, &args![inventory, old, hotkey]);
        if e.call(ENTRY_OBJECT, &args![old]).u32() != 0 {
            let model = biped_model(e, old);
            if model != 0 {
                let path = e.vcall(model, 0x14, &args![]).u32();
                let loader: u32 = e.global(MODEL_OBJECT);
                e.call(MODEL_RELEASE, &args![loader, path]);
            }
        }
    }
    let hotkey = e.get(this, HotKeysWheel::iLastHighlightedHotkey);
    e.call(SET_HOTKEY_ITEM, &args![inventory, entry, extra, hotkey]);
    if e.call(ENTRY_OBJECT, &args![entry]).u32() != 0 {
        let model = biped_model(e, entry);
        if model != 0 {
            let path = e.vcall(model, 0x14, &args![]).u32();
            let loader: u32 = e.global(MODEL_OBJECT);
            e.call(
                MODEL_LOAD_FILE,
                &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
            );
        }
    }
    hotkeys_wheel_update_hotkey_list(e, this);
    let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
    e.call(AFTER_ASSIGN, &args![player, form, 1u32, 0u32]);
    e.set(this, HotKeysWheel::bSelectingHotkey, false);
    if old != 0 {
        e.call(ENTRY_DESTROY, &args![old, 1u32]);
    }
}

// Translated from 00701bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::MoveHighlight` (Xbox PDB): returns the hotkey now
/// highlighted (-1 for none) and highlights it. Hotkey 1 does nothing and
/// answers -1.
///
/// Without the gamepad the hotkey is the argument (-1 for a negative one).
/// With it, a negative argument is read from the four d-pad buttons
/// (`CheckMenuButton` of 2, 1, 4 and 3, a button counting as held when its
/// answer is 0 or 2): 2 with 4 gives 5, 2 with 3 gives 7, 2 alone 6; 1 with
/// 4 gives 0, 1 with 3 gives 2 (1 alone none); 4 gives 3; 3 gives 4. The
/// highlight then moves when the hotkey differs from the current one and
/// either more than 50 ticks have passed since the pad last asked for a
/// different hotkey or one of the two is -1.
pub fn hotkeys_wheel_move_highlight(e: &mut Engine, this: Ptr<HotKeysWheel>, hotkey: i32) -> i32 {
    let mut result = -1;
    if hotkey == 1 {
        return -1;
    }
    if !e.call(GAMEPAD_IN_USE, &args![]).bool() {
        if hotkey >= 0 {
            result = hotkey;
        }
        hotkeys_wheel_highlight_hotkey(e, this, result);
        return result;
    }
    if hotkey < 0 {
        let held = |button: i32| button == 2 || button == 0;
        let up = check_menu_button(e, 2);
        let left = check_menu_button(e, 1);
        let right = check_menu_button(e, 4);
        let down = check_menu_button(e, 3);
        let (up, left, right, down) = (held(up), held(left), held(right), held(down));
        if up {
            result = if right {
                5
            } else if down {
                7
            } else {
                6
            };
        } else if left {
            if right {
                result = 0;
            } else if down {
                result = 2;
            }
        } else if right {
            result = 3;
        } else if down {
            result = 4;
        }
    }
    let highlighted_trait = e.get(this, HotKeysWheel::iUT_HighlightedHotkey);
    let wheel = e.get(this, HotKeysWheel::pWheel).addr();
    let highlighted = e
        .call(GET_FLOAT_TRAIT, &args![wheel, highlighted_trait])
        .f32();
    let current = e.call(FLOAT_TO_INT, &args![highlighted as f64]).i32();
    if result != current {
        if result != e.global::<i32>(LAST_ASKED) {
            e.set_global(LAST_ASKED, result);
            let ticks = e.call(TICK_COUNT, &args![TICK_OBJECT]).u32();
            e.set_global(LAST_ASKED_TICK, ticks);
        }
        let ticks = e.call(TICK_COUNT, &args![TICK_OBJECT]).u32();
        let elapsed = ticks.wrapping_sub(e.global::<u32>(LAST_ASKED_TICK));
        let delay: f32 = e.global(HIGHLIGHT_DELAY);
        if (delay as f64) < elapsed as f64 || result == -1 || current == -1 {
            hotkeys_wheel_highlight_hotkey(e, this, result);
        }
    }
    result
}

// Translated from 00701e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::HighlightHotkey` (Xbox PDB): sets the highlighted-hotkey
/// user trait of the parent tile to `hotkey`, and the highlighted-text
/// trait to the hotkey tile's string (the empty string for a negative
/// hotkey or hotkey 1). A non-negative hotkey also becomes the last
/// highlighted one and sets the selecting flag.
pub fn hotkeys_wheel_highlight_hotkey(e: &mut Engine, this: Ptr<HotKeysWheel>, hotkey: i32) {
    let wheel = e.get(this, HotKeysWheel::pWheel).addr();
    let highlighted_trait = e.get(this, HotKeysWheel::iUT_HighlightedHotkey);
    set_trait_from_int(e, wheel, highlighted_trait, hotkey);
    let text = if hotkey >= 0 && hotkey != SKIPPED_HOTKEY {
        let tile = hotkey_tile(e, this, hotkey as u32);
        e.call(GET_STRING_TRAIT, &args![tile, TRAIT_STRING]).u32()
    } else {
        EMPTY_STRING
    };
    let wheel = e.get(this, HotKeysWheel::pWheel).addr();
    let text_trait = e.get(this, HotKeysWheel::iUT_HighlightedText);
    set_string(e, wheel, text_trait, text);
    if hotkey >= 0 {
        e.set(this, HotKeysWheel::iLastHighlightedHotkey, hotkey);
        e.set(this, HotKeysWheel::bSelectingHotkey, true);
    }
}

// Translated from 00701e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Uses the highlighted hotkey (a handler of the wheel's menu): equips the
/// entry on the hotkey, or unequips it when it is worn.
///
/// Needs the selecting flag, inventory changes for the player and an entry
/// on the hotkey. It stops when the player is mid-animation
/// (`Actor::GetAnimAction` is not -1) and the object is of form type 0x18 or
/// 0x28. For the default object 0 (`GetDefaultObject(0)`) it shows the
/// message of the setting at `011d2af0` for 2.0 seconds and stops when the
/// player's slot 8 and slot `0x1c` of the object at +0xa4 agree for 0x10.
/// Otherwise it queues the equip, or the unequip for a worn entry, with the
/// entry's count (only used for form type 0x28 usable objects; else 1) and
/// first extra data list, updates the player's 3D, calls the saved acquire
/// object's slot `+0x580` with (1, 0, 0) and the player's slot `+0x3c4`,
/// and destroys the entry.
pub fn fn_00701e80(e: &mut Engine, this: Ptr<HotKeysWheel>) {
    let player: u32 = e.global(PLAYER);
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    let entry = if inventory != 0 {
        let hotkey = e.get(this, HotKeysWheel::iLastHighlightedHotkey);
        e.call(HOTKEY_ENTRY, &args![inventory, hotkey]).u32()
    } else {
        0
    };
    let extra = if entry != 0 {
        first_extra_data(e, entry)
    } else {
        0
    };
    let mut counted = false;
    if entry != 0 {
        let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
        if e.call(FORM_TYPE, &args![form]).u32() == 0x28 {
            let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
            if e.call(FORM_IS_USABLE, &args![form]).bool() {
                counted = true;
            }
        }
    }
    if !e.get(this, HotKeysWheel::bSelectingHotkey) || inventory == 0 || entry == 0 {
        return;
    }
    for form_type in [0x18u32, 0x28] {
        if e.call(ACTOR_GET_ANIM_ACTION, &args![player]).i32() != -1 {
            let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
            if e.call(FORM_TYPE, &args![form]).u32() == form_type {
                return;
            }
        }
    }
    let form = e.call(ENTRY_OBJECT, &args![entry]).u32();
    let default_object = e.call(GET_DEFAULT_OBJECT, &args![0u32]).u32();
    if form == default_object {
        let slots = player + 0xa4;
        let first = e.vcall(slots, 8, &args![0x10u32]).u32();
        let second = e.vcall(slots, 0x1c, &args![0x10u32]).u32();
        if first == second {
            let seconds = through_x87(e.global(MESSAGE_SECONDS));
            let text = e.call(MESSAGE_TEXT, &args![MESSAGE_SETTING]).u32();
            e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, seconds, 0u32]);
            return;
        }
    }
    let worn = e.call(ITEM_GET_WORN, &args![entry, 0u32]).bool();
    let count = if counted {
        e.call(WORD_AT_FOUR, &args![entry]).u32()
    } else {
        1
    };
    let object = e.call(ENTRY_OBJECT, &args![entry]).u32();
    let queue = if worn {
        QUEUE_UNEQUIP_OBJECT
    } else {
        QUEUE_EQUIP_OBJECT
    };
    e.call(
        queue,
        &args![player, object, count, extra, 0u32, 0u32, 1u32],
    );
    e.call(UPDATE_PLAYER_3D, &args![player]);
    let acquire = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
    e.vcall(acquire, 0x580, &args![1u32, 0u32, 0u32]);
    let player: u32 = e.global(PLAYER);
    e.vcall(player, 0x3c4, &args![]);
    e.call(ENTRY_DESTROY, &args![entry, 1u32]);
}

// Translated from 00702140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HotKeysWheel::IsDPadPressed` (Xbox PDB): with the gamepad in use
/// (`00709d40`), whether any of the buttons 2, 1, 4 and 3 answers other
/// than -1 (tested in that order, stopping at the first). Static: no word and ECX not read
/// (its caller 00782200 pushes nothing).
pub fn hotkeys_wheel_is_d_pad_pressed(e: &mut Engine) -> bool {
    if !e.call(GAMEPAD_IN_USE_WRAPPER, &args![]).bool() {
        return false;
    }
    [2u32, 1, 4, 3]
        .into_iter()
        .any(|button| check_menu_button(e, button) != -1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00701610, hotkeys_wheel_initialize(Ptr<HotKeysWheel>, Ptr)),
        entry!(
            0x00701740,
            hotkeys_wheel_get_visible(Ptr<HotKeysWheel>) -> bool
        ),
        entry!(
            0x00701760,
            hotkeys_wheel_set_visible(Ptr<HotKeysWheel>, bool)
        ),
        entry!(
            0x007017b0,
            hotkeys_wheel_update_hotkey_list(Ptr<HotKeysWheel>)
        ),
        entry!(0x007019e0, fn_007019e0(Ptr<HotKeysWheel>, u32)),
        entry!(
            0x00701bd0,
            hotkeys_wheel_move_highlight(Ptr<HotKeysWheel>, i32) -> i32
        ),
        entry!(
            0x00701e00,
            hotkeys_wheel_highlight_hotkey(Ptr<HotKeysWheel>, i32)
        ),
        entry!(0x00701e80, fn_00701e80(Ptr<HotKeysWheel>)),
        entry!(0x00702140, hotkeys_wheel_is_d_pad_pressed() -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The configuration page the doubles read.
    const CFG: u32 = 0x0200_0000;
    const CFG_ANIM_ACTION: u32 = CFG;
    const CFG_WORN: u32 = CFG + 0x04;
    const CFG_DEFAULT_OBJECT: u32 = CFG + 0x08;
    const CFG_GAMEPAD: u32 = CFG + 0x0c;
    const CFG_TICKS: u32 = CFG + 0x40;
    const CFG_USABLE: u32 = CFG + 0x44;
    const CFG_INVENTORY: u32 = CFG + 0x48;
    const CFG_ENTRIES: u32 = CFG + 0x4c;
    const CFG_SLOT_A: u32 = CFG + 0x50;
    const CFG_SLOT_B: u32 = CFG + 0x54;
    const CFG_HIGHLIGHT_TRAIT: u32 = CFG + 0x70;
    const CFG_VISIBLE: u32 = CFG + 0x74;
    const CFG_ACQUIRE: u32 = CFG + 0x78;
    /// A string the tiles' string traits read as.
    const CFG_TILE_TEXT: u32 = CFG + 0x200;
    /// Answers of `CheckMenuButton`, by button number.
    const CFG_BUTTONS: u32 = CFG + 0x20;
    /// The hotkey entries `004bf4b0` answers, by hotkey.
    const CFG_HOTKEYS: u32 = CFG + 0x100;

    /// Vtables of the test objects: the player (slot `+0x3c4`), its
    /// embedded object at +0xa4 (slots 8 and `0x1c`), the acquire object
    /// (slot `+0x580`) and the biped models (slot `+0x14`).
    const PLAYER_TABLE: u32 = 0x0f30_0000;
    const PLAYER_SLOT: u32 = 0x0f30_5000;
    const SUB_TABLE: u32 = 0x0f31_0000;
    const SUB_SLOT_A: u32 = 0x0f31_5000;
    const SUB_SLOT_B: u32 = 0x0f31_5001;
    const ACQUIRE_TABLE: u32 = 0x0f32_0000;
    const ACQUIRE_SLOT: u32 = 0x0f32_5000;
    const MODEL_TABLE: u32 = 0x0f33_0000;
    const MODEL_SLOT: u32 = 0x0f33_5000;

    type Strings = Rc<RefCell<Vec<(u32, u32, Vec<u8>)>>>;

    struct World {
        e: Engine,
        strings: Strings,
        player: u32,
        acquire: u32,
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn world() -> World {
        let mut e = Engine::new();
        for page in [
            0x0200_0000u32,
            0x011d_e000,
            0x011d_8000,
            0x0119_f000,
            0x0101_b000,
            0x0101_6000,
            0x0101_1000,
            0x011c_3000,
            0x0106_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HIGHLIGHT_DELAY, 50.0f32);
        e.set_global(MESSAGE_SECONDS, 2.0f32);
        e.set_global(LAST_ASKED, -1i32);
        e.set_global(MODEL_OBJECT, 0x7c00u32);
        e.mem.set_cstr(ICON_DIRECTORY, b"Interface\\Icons\\");
        for button in 1..=4 {
            e.mem.set_i32(CFG_BUTTONS + 4 * button, -1);
        }
        e.mem.set_i32(CFG_ANIM_ACTION, -1);

        let player = e.mem.alloc(0x200);
        e.mem.set_u32(player, PLAYER_TABLE);
        e.mem.set_u32(player + 0xa4, SUB_TABLE);
        e.set_global(PLAYER, player);
        e.put_vtable(PLAYER_TABLE, &[0u32; 0xf2]);
        e.mem.set_u32(PLAYER_TABLE + 0x3c4, PLAYER_SLOT);
        e.put_vtable(SUB_TABLE, &[0u32; 8]);
        e.mem.set_u32(SUB_TABLE + 8, SUB_SLOT_A);
        e.mem.set_u32(SUB_TABLE + 0x1c, SUB_SLOT_B);
        let acquire = e.mem.alloc(0x40);
        e.mem.set_u32(acquire, ACQUIRE_TABLE);
        e.put_vtable(ACQUIRE_TABLE, &[0u32; 0x162]);
        e.mem.set_u32(ACQUIRE_TABLE + 0x580, ACQUIRE_SLOT);
        e.put_vtable(MODEL_TABLE, &[0u32; 8]);
        e.mem.set_u32(MODEL_TABLE + 0x14, MODEL_SLOT);

        for addr in [
            PLAYER_SLOT,
            ACQUIRE_SLOT,
            SET_TRAIT_FROM_INT,
            STRING_DESTROY,
            PRINT_MESSAGE,
            ENTRY_DESTROY,
            REMOVE_HOTKEY_ITEM,
            SET_HOTKEY_ITEM,
            MODEL_RELEASE,
            MODEL_LOAD_FILE,
            AFTER_ASSIGN,
            QUEUE_EQUIP_OBJECT,
            QUEUE_UNEQUIP_OBJECT,
            UPDATE_PLAYER_3D,
            SHOW_MESSAGE,
            FREE_ARRAY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // Accessors.
        e.register(WORD_AT_FOUR, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(THIS_ITEM, |_, a| word(a[0]));
        e.register(STRING_C_STR, |e, a| word(e.mem.u32(a[0])));
        e.register(ENTRY_OBJECT, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(FORM_TYPE, |e, a| word(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_BIPED_MODEL, |e, a| word(e.mem.u32(a[0] + 0x20)));
        e.register(RT_DYNAMIC_CAST, |e, a| word(e.mem.u32(a[0] + 0x24)));
        e.register(MODEL_SLOT, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(ITEM_FULL_NAME, |e, a| word(e.mem.u32(a[0] + 0x10)));
        // Configured answers.
        e.register(GET_INVENTORY_CHANGES, |e, _| word(e.mem.u32(CFG_INVENTORY)));
        e.register(HOTKEY_ENTRIES, |e, _| word(e.mem.u32(CFG_ENTRIES)));
        e.register(HOTKEY_ENTRY, |e, a| word(e.mem.u32(CFG_HOTKEYS + 4 * a[1])));
        e.register(ACTOR_GET_ANIM_ACTION, |e, _| {
            word(e.mem.u32(CFG_ANIM_ACTION))
        });
        e.register(ITEM_GET_WORN, |e, _| word(e.mem.u32(CFG_WORN)));
        e.register(GET_DEFAULT_OBJECT, |e, _| {
            word(e.mem.u32(CFG_DEFAULT_OBJECT))
        });
        e.register(FORM_IS_USABLE, |e, _| word(e.mem.u32(CFG_USABLE)));
        e.register(GAMEPAD_IN_USE, |e, _| word(e.mem.u32(CFG_GAMEPAD)));
        e.register(GAMEPAD_IN_USE_WRAPPER, |e, _| word(e.mem.u32(CFG_GAMEPAD)));
        e.register(CHECK_MENU_BUTTON, |e, a| {
            word(e.mem.u32(CFG_BUTTONS + 4 * a[0]))
        });
        e.register(TICK_COUNT, |e, _| word(e.mem.u32(CFG_TICKS)));
        e.mem.set_u32(CFG_ACQUIRE, acquire);
        e.register(GET_SAVED_ACQUIRE_OBJECT, |e, _| {
            word(e.mem.u32(CFG_ACQUIRE))
        });
        e.mem.set_cstr(CFG_TILE_TEXT, b"TILE TEXT");
        e.register(IS_TRAIT_TRUE, |e, _| word(e.mem.u32(CFG_VISIBLE)));
        e.register(GET_FLOAT_TRAIT, |e, _| Ret {
            st0: e.mem.f32(CFG_HIGHLIGHT_TRAIT) as f64,
            ..Ret::default()
        });
        e.register(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            word(value as i32 as u32)
        });
        e.register(GET_STRING_TRAIT, |_, _| word(CFG_TILE_TEXT));
        e.register(MESSAGE_TEXT, |_, _| word(0x7e11));
        e.register(SUB_SLOT_A, |e, _| word(e.mem.u32(CFG_SLOT_A)));
        e.register(SUB_SLOT_B, |e, _| word(e.mem.u32(CFG_SLOT_B)));
        e.register(GET_EQUIP_TYPE, |_, _| word(7));
        e.register(GET_EQUIP_TYPE_ICON_FILENAME, |e, _| {
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, b"Weapon.dds");
            word(name)
        });
        // Strings: the first word of a string is its characters.
        e.register(STRING_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(160);
            e.mem.set_u32(a[0], buffer);
            word(a[0])
        });
        e.register(STRING_SET, |e, a| {
            let text = e.mem.cstr(a[1]);
            let buffer = e.mem.u32(a[0]);
            e.mem.set_cstr(buffer, &text);
            word(a[0])
        });
        e.register(STRING_APPEND, |e, a| {
            let buffer = e.mem.u32(a[0]);
            let mut text = e.mem.cstr(buffer);
            text.extend(e.mem.cstr(a[1]));
            e.mem.set_cstr(buffer, &text);
            word(a[0])
        });
        e.register(STRING_FORMAT, |e, a| {
            let text = if a[1] == ITEM_NAME_FORMAT {
                format!("HK_Item_{}", a[2]).into_bytes()
            } else {
                let mut text = e.mem.cstr(a[2]);
                text.extend(format!(" ({})", a[3] as i32).into_bytes());
                text
            };
            let buffer = e.mem.u32(a[0]);
            e.mem.set_cstr(buffer, &text);
            word(a[0])
        });
        e.register(TEXT_TO_TRAIT, |_, a| {
            word(if a[0] == HOTKEY_ICON_NAME { 500 } else { 501 })
        });
        let mut next_trait = 300u32;
        e.register_double(ADD_USER_TRAIT, move |_, _| {
            next_trait += 1;
            word(next_trait)
        });
        e.register(GET_CHILD_BY_NAME, |e, a| {
            let name = e.mem.cstr(a[1]);
            let index = (name.last().unwrap() - b'0') as u32;
            word(if index == 5 { 0 } else { 0x6100 + 0x10 * index })
        });
        let strings: Strings = Rc::new(RefCell::new(vec![]));
        let log = strings.clone();
        e.register_double(SET_STRING_TRAIT, move |e, a| {
            log.borrow_mut().push((a[0], a[1], e.mem.cstr(a[2])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        World {
            e,
            strings,
            player,
            acquire,
        }
    }

    const WHEEL_TILE: u32 = 0x6000;

    fn tile(index: u32) -> u32 {
        0x6100 + 0x10 * index
    }

    /// A wheel with the parent tile, eight tiles and the two trait ids 301
    /// and 302.
    fn wheel(e: &mut Engine) -> Ptr<HotKeysWheel> {
        let this: Ptr<HotKeysWheel> = e.new_object();
        e.set(this, HotKeysWheel::pWheel, Ptr::new(WHEEL_TILE));
        for index in 0..8 {
            e.mem.set_u32(this.addr() + TILES + 4 * index, tile(index));
        }
        e.set(this, HotKeysWheel::iUT_HighlightedHotkey, 301);
        e.set(this, HotKeysWheel::iUT_HighlightedText, 302);
        this
    }

    /// An inventory entry: extra list head (+0), count (+4), object (+8)
    /// and name (+0x10).
    fn entry(e: &mut Engine, name: &[u8], count: u32, object: u32, extra: u32) -> u32 {
        let entry = e.mem.alloc(0x20);
        if extra != 0 {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, extra);
            e.mem.set_u32(entry, node);
        }
        e.mem.set_u32(entry + 4, count);
        e.mem.set_u32(entry + 8, object);
        let text = e.mem.alloc(32);
        e.mem.set_cstr(text, name);
        e.mem.set_u32(entry + 0x10, text);
        entry
    }

    /// A form: type byte (+4), biped model (+0x20) and the cast result
    /// (+0x24).
    fn form(e: &mut Engine, kind: u8, model: u32, cast: u32) -> u32 {
        let form = e.mem.alloc(0x40);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(form + 0x20, model);
        e.mem.set_u32(form + 0x24, cast);
        form
    }

    /// A model object whose path is `path`.
    fn model(e: &mut Engine, path: u32) -> u32 {
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model, MODEL_TABLE);
        e.mem.set_u32(model + 8, path);
        model
    }

    fn set_inventory(e: &mut Engine, inventory: u32) {
        e.mem.set_u32(CFG_INVENTORY, inventory);
    }

    // ----- 00701610 -----

    #[test]
    fn initialize_without_a_parent_only_warns() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1610, &args![this, 0u32]);
        assert_eq!(calls(&w.e, PRINT_MESSAGE), vec![vec![NULL_PARENT_MESSAGE]]);
        assert_eq!(w.e.get(this, HotKeysWheel::pWheel).addr(), WHEEL_TILE);
        assert!(calls(&w.e, ADD_USER_TRAIT).is_empty());
        assert_eq!(calls(&w.e, STRING_DESTROY).len(), 1);
    }

    #[test]
    fn initialize_finds_the_children_and_registers_the_traits() {
        let mut w = world();
        let this: Ptr<HotKeysWheel> = w.e.new_object();
        w.e.set(this, HotKeysWheel::bSelectingHotkey, true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1610, &args![this, 0x6000u32]);
        assert_eq!(w.e.get(this, HotKeysWheel::pWheel).addr(), 0x6000);
        // The children HK_Item_0 .. HK_Item_7; number 5 is missing.
        let tiles: Vec<u32> = (0..8).map(|i| hotkey_tile(&w.e, this, i)).collect();
        assert_eq!(
            tiles,
            [
                tile(0),
                tile(1),
                tile(2),
                tile(3),
                tile(4),
                0,
                tile(6),
                tile(7)
            ]
        );
        let queried: Vec<Vec<u32>> = calls(&w.e, GET_CHILD_BY_NAME)
            .iter()
            .map(|c| vec![c[0]])
            .collect();
        assert_eq!(queried, vec![vec![0x6000]; 8]);
        // The two user traits, the selecting flag, and the parent's traits.
        assert_eq!(
            calls(&w.e, ADD_USER_TRAIT),
            vec![
                vec![SELECTED_HOTKEY_NAME, u32::MAX],
                vec![SELECTED_TEXT_NAME, u32::MAX]
            ]
        );
        assert_eq!(w.e.get(this, HotKeysWheel::iUT_HighlightedHotkey), 301);
        assert_eq!(w.e.get(this, HotKeysWheel::iUT_HighlightedText), 302);
        assert!(!w.e.get(this, HotKeysWheel::bSelectingHotkey));
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![0x6000, 301, u32::MAX]]
        );
        assert_eq!(*w.strings.borrow(), vec![(0x6000, 302, vec![])]);
        assert_eq!(calls(&w.e, STRING_DESTROY).len(), 1);
    }

    // ----- 00701740, 00701760 -----

    #[test]
    fn visibility_is_the_parent_tiles_visible_trait() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_VISIBLE, 1);
        w.e.call_log = Some(vec![]);
        assert!(w.e.call(0x0070_1740, &args![this]).bool());
        assert_eq!(calls(&w.e, IS_TRAIT_TRUE), vec![vec![WHEEL_TILE, 0xfa3]]);
        w.e.mem.set_u32(CFG_VISIBLE, 0);
        assert!(!w.e.call(0x0070_1740, &args![this]).bool());
    }

    #[test]
    fn showing_refreshes_the_list_first() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1760, &args![this, 1u32]);
        assert_eq!(calls(&w.e, GET_INVENTORY_CHANGES).len(), 1);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 0xfa3, 1]]
        );
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1760, &args![this, 0u32]);
        assert!(calls(&w.e, GET_INVENTORY_CHANGES).is_empty());
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 0xfa3, 0]]
        );
    }

    // ----- 007017b0 -----

    #[test]
    fn the_list_does_nothing_without_inventory_changes() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_17b0, &args![this]);
        assert!(calls(&w.e, HOTKEY_ENTRIES).is_empty());
        assert!(w.strings.borrow().is_empty());
        assert_eq!(calls(&w.e, STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&w.e, STRING_DESTROY).len(), 1);
    }

    #[test]
    fn the_list_fills_the_tiles_from_the_hotkey_entries() {
        let mut w = world();
        let this = wheel(&mut w.e);
        set_inventory(&mut w.e, 0x8000);
        let object = form(&mut w.e, 5, 0, 0);
        let stimpak = entry(&mut w.e, b"Stimpak", 1, object, 0);
        let ignored = entry(&mut w.e, b"Ignored", 1, object, 0);
        let jet = entry(&mut w.e, b"Jet", 3, object, 0);
        let array = w.e.mem.alloc(32);
        w.e.mem.set_u32(array, stimpak);
        w.e.mem.set_u32(array + 4, ignored);
        w.e.mem.set_u32(array + 8, jet);
        w.e.mem.set_u32(CFG_ENTRIES, array);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_17b0, &args![this]);
        assert_eq!(calls(&w.e, HOTKEY_ENTRIES), vec![vec![0x8000, 0, 7]]);
        let strings = w.strings.borrow().clone();
        let text = |s: &str| s.as_bytes().to_vec();
        assert_eq!(
            strings,
            vec![
                (tile(0), 500, text("Interface\\Icons\\Weapon.dds")),
                (tile(0), 0xfc4, text("Stimpak")),
                (tile(2), 500, text("Interface\\Icons\\Weapon.dds")),
                (tile(2), 0xfc4, text("Jet (3)")),
                (tile(3), 0xfc4, vec![]),
                (tile(4), 0xfc4, vec![]),
                (tile(5), 0xfc4, vec![]),
                (tile(6), 0xfc4, vec![]),
                (tile(7), 0xfc4, vec![]),
            ]
        );
        // Assigned flags: 1 for the entries, 0 for the empty hotkeys; the
        // skipped hotkey 1 is not touched.
        let assigned: Vec<(u32, u32)> = calls(&w.e, SET_TRAIT_FROM_INT)
            .iter()
            .map(|c| (c[0], c[2]))
            .collect();
        assert_eq!(
            assigned,
            [
                (tile(0), 1),
                (tile(2), 1),
                (tile(3), 0),
                (tile(4), 0),
                (tile(5), 0),
                (tile(6), 0),
                (tile(7), 0)
            ]
        );
        // Every entry (also the skipped one) is destroyed; the array freed.
        assert_eq!(
            calls(&w.e, ENTRY_DESTROY),
            vec![vec![stimpak, 1], vec![ignored, 1], vec![jet, 1]]
        );
        assert_eq!(calls(&w.e, FREE_ARRAY), vec![vec![array]]);
        assert_eq!(calls(&w.e, STRING_DESTROY).len(), 1);
    }

    // ----- 007019e0 -----

    /// Everything the assignment needs: an inventory, a hotkey 3 being
    /// selected, the entry to assign and the entry on hotkey 3.
    struct Assign {
        w: World,
        this: Ptr<HotKeysWheel>,
        entry: u32,
        old: u32,
        new_form: u32,
        extra: u32,
    }

    fn assign_world() -> Assign {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.set(this, HotKeysWheel::bSelectingHotkey, true);
        w.e.set(this, HotKeysWheel::iLastHighlightedHotkey, 3);
        set_inventory(&mut w.e, 0x8000);
        let zeros = w.e.mem.alloc(32);
        w.e.mem.set_u32(CFG_ENTRIES, zeros);
        let new_model = model(&mut w.e, 0xa001);
        let new_form = form(&mut w.e, 5, new_model, 0);
        let extra = 0xe001;
        let entry = entry(&mut w.e, b"New", 1, new_form, extra);
        let old_model = model(&mut w.e, 0xa002);
        let old_form = form(&mut w.e, 5, old_model, 0);
        let old = entry_for(&mut w.e, old_form);
        w.e.mem.set_u32(CFG_HOTKEYS + 4 * 3, old);
        w.e.call_log = Some(vec![]);
        Assign {
            w,
            this,
            entry,
            old,
            new_form,
            extra,
        }
    }

    fn entry_for(e: &mut Engine, object: u32) -> u32 {
        entry(e, b"Old", 1, object, 0)
    }

    #[test]
    fn assigning_replaces_the_entry_on_the_hotkey() {
        let mut a = assign_world();
        let e = &mut a.w.e;
        e.call(0x0070_19e0, &args![a.this, a.entry]);
        let player = a.w.player;
        assert_eq!(calls(e, HOTKEY_ENTRY)[0], vec![0x8000, 3]);
        assert_eq!(calls(e, REMOVE_HOTKEY_ITEM), vec![vec![0x8000, a.old, 3]]);
        // The old model path is released, the new one loaded.
        assert_eq!(calls(e, MODEL_RELEASE), vec![vec![0x7c00, 0xa002]]);
        assert_eq!(
            calls(e, SET_HOTKEY_ITEM),
            vec![vec![0x8000, a.entry, a.extra, 3]]
        );
        assert_eq!(
            calls(e, MODEL_LOAD_FILE),
            vec![vec![0x7c00, 0xa001, 0, 1, 0, 0, 0]]
        );
        assert_eq!(calls(e, AFTER_ASSIGN), vec![vec![player, a.new_form, 1, 0]]);
        assert_eq!(calls(e, ENTRY_DESTROY), vec![vec![a.old, 1]]);
        assert!(!e.get(a.this, HotKeysWheel::bSelectingHotkey));
        // The list is refreshed after the assignment.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [
                    SET_HOTKEY_ITEM,
                    MODEL_LOAD_FILE,
                    HOTKEY_ENTRIES,
                    AFTER_ASSIGN,
                ]
                .contains(address)
            })
            .collect();
        assert_eq!(
            order,
            [
                SET_HOTKEY_ITEM,
                MODEL_LOAD_FILE,
                HOTKEY_ENTRIES,
                AFTER_ASSIGN
            ]
        );
    }

    #[test]
    fn assigning_to_an_empty_hotkey_removes_nothing() {
        let mut a = assign_world();
        let e = &mut a.w.e;
        e.mem.set_u32(CFG_HOTKEYS + 4 * 3, 0);
        e.call(0x0070_19e0, &args![a.this, a.entry]);
        assert!(calls(e, REMOVE_HOTKEY_ITEM).is_empty());
        assert!(calls(e, MODEL_RELEASE).is_empty());
        assert_eq!(calls(e, SET_HOTKEY_ITEM).len(), 1);
        assert!(calls(e, ENTRY_DESTROY).is_empty());
    }

    #[test]
    fn a_form_without_a_biped_model_is_cast_to_the_texture_swap_type() {
        let mut a = assign_world();
        let e = &mut a.w.e;
        let cast_model = model(e, 0xa003);
        e.mem.set_u32(a.new_form + 0x20, 0);
        e.mem.set_u32(a.new_form + 0x24, cast_model);
        e.call(0x0070_19e0, &args![a.this, a.entry]);
        assert_eq!(
            calls(e, RT_DYNAMIC_CAST),
            vec![vec![
                a.new_form,
                0,
                TYPE_BOUND_OBJECT,
                TYPE_MODEL_TEXTURE_SWAP,
                0
            ]]
        );
        assert_eq!(
            calls(e, MODEL_LOAD_FILE),
            vec![vec![0x7c00, 0xa003, 0, 1, 0, 0, 0]]
        );
    }

    #[test]
    fn a_form_without_any_model_loads_nothing() {
        let mut a = assign_world();
        let e = &mut a.w.e;
        e.mem.set_u32(a.new_form + 0x20, 0);
        e.call(0x0070_19e0, &args![a.this, a.entry]);
        assert!(calls(e, MODEL_LOAD_FILE).is_empty());
        assert_eq!(calls(e, AFTER_ASSIGN).len(), 1);
    }

    #[test]
    fn assigning_needs_an_entry_the_flag_and_inventory_changes() {
        let mut a = assign_world();
        a.w.e.call(0x0070_19e0, &args![a.this, 0u32]);
        assert!(calls(&a.w.e, SET_HOTKEY_ITEM).is_empty());
        let mut a = assign_world();
        a.w.e.set(a.this, HotKeysWheel::bSelectingHotkey, false);
        a.w.e.call(0x0070_19e0, &args![a.this, a.entry]);
        assert!(calls(&a.w.e, SET_HOTKEY_ITEM).is_empty());
        let mut a = assign_world();
        set_inventory(&mut a.w.e, 0);
        a.w.e.call(0x0070_19e0, &args![a.this, a.entry]);
        assert!(calls(&a.w.e, SET_HOTKEY_ITEM).is_empty());
        assert!(calls(&a.w.e, HOTKEY_ENTRY).is_empty());
    }

    // ----- 00701bd0 -----

    fn press(e: &mut Engine, up: bool, left: bool, right: bool, down: bool) {
        for (button, held) in [(2u32, up), (1, left), (4, right), (3, down)] {
            e.mem
                .set_i32(CFG_BUTTONS + 4 * button, if held { 2 } else { -1 });
        }
    }

    #[test]
    fn move_highlight_without_the_gamepad_follows_the_argument() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, 3i32]).i32(), 3);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 301, 3]]
        );
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, -5i32]).i32(), -1);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT)[1],
            vec![WHEEL_TILE, 301, u32::MAX]
        );
    }

    #[test]
    fn move_highlight_ignores_hotkey_one() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, 1i32]).i32(), -1);
        assert!(calls(&w.e, SET_TRAIT_FROM_INT).is_empty());
        assert!(calls(&w.e, GAMEPAD_IN_USE).is_empty());
    }

    #[test]
    fn the_d_pad_picks_the_hotkey() {
        // (up, left, right, down) -> hotkey.
        let expected = |up: bool, left: bool, right: bool, down: bool| -> i32 {
            if up {
                if right {
                    5
                } else if down {
                    7
                } else {
                    6
                }
            } else if left {
                if right {
                    0
                } else if down {
                    2
                } else {
                    -1
                }
            } else if right {
                3
            } else if down {
                4
            } else {
                -1
            }
        };
        for bits in 0..16u32 {
            let (up, left, right, down) =
                (bits & 1 != 0, bits & 2 != 0, bits & 4 != 0, bits & 8 != 0);
            let mut w = world();
            let this = wheel(&mut w.e);
            w.e.mem.set_u32(CFG_GAMEPAD, 1);
            press(&mut w.e, up, left, right, down);
            // The trait holds a different value, so the highlight moves.
            w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 99.0);
            w.e.mem.set_u32(CFG_TICKS, 1000);
            w.e.set_global(LAST_ASKED, 99i32);
            w.e.set_global(LAST_ASKED_TICK, 0u32);
            w.e.call_log = Some(vec![]);
            let want = expected(up, left, right, down);
            let got = w.e.call(0x0070_1bd0, &args![this, -1i32]).i32();
            assert_eq!(got, want, "{up} {left} {right} {down}");
            // The four buttons are read in the order 2, 1, 4, 3.
            let buttons: Vec<u32> = calls(&w.e, CHECK_MENU_BUTTON)
                .iter()
                .map(|c| c[0])
                .collect();
            assert_eq!(buttons, [2, 1, 4, 3]);
        }
    }

    #[test]
    fn a_button_answer_of_zero_counts_as_held() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_GAMEPAD, 1);
        w.e.mem.set_i32(CFG_BUTTONS + 4 * 2, 0);
        w.e.mem.set_i32(CFG_BUTTONS + 4 * 3, 1);
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 99.0);
        // Up held (0), down not held (1 is neither 0 nor 2): hotkey 6.
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, -1i32]).i32(), 6);
    }

    #[test]
    fn the_pad_waits_before_moving_the_highlight() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_GAMEPAD, 1);
        press(&mut w.e, true, false, true, false);
        // Current highlight 2, asked 5: new request at tick 1000.
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 2.0);
        w.e.mem.set_u32(CFG_TICKS, 1000);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, -1i32]).i32(), 5);
        assert_eq!(w.e.global::<i32>(LAST_ASKED), 5);
        assert_eq!(w.e.global::<u32>(LAST_ASKED_TICK), 1000);
        // No time has passed: the highlight stays.
        assert!(calls(&w.e, SET_TRAIT_FROM_INT).is_empty());
        // 50 ticks later is not enough (strictly more is needed) ...
        w.e.mem.set_u32(CFG_TICKS, 1050);
        w.e.call(0x0070_1bd0, &args![this, -1i32]);
        assert!(calls(&w.e, SET_TRAIT_FROM_INT).is_empty());
        // ... 51 is.
        w.e.mem.set_u32(CFG_TICKS, 1051);
        w.e.call(0x0070_1bd0, &args![this, -1i32]);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 301, 5]]
        );
        // The wait is counted from the change, not reset by repeats.
        assert_eq!(w.e.global::<u32>(LAST_ASKED_TICK), 1000);
    }

    #[test]
    fn the_pad_moves_at_once_to_or_from_no_hotkey() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_GAMEPAD, 1);
        w.e.mem.set_u32(CFG_TICKS, 1000);
        // From no hotkey (-1) to hotkey 5.
        press(&mut w.e, true, false, true, false);
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, -1.0);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1bd0, &args![this, -1i32]);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 301, 5]]
        );
        // To no hotkey (-1) from hotkey 5.
        press(&mut w.e, false, false, false, false);
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 5.0);
        w.e.call(0x0070_1bd0, &args![this, -1i32]);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT)[1],
            vec![WHEEL_TILE, 301, u32::MAX]
        );
        // The same hotkey as the current one changes nothing.
        press(&mut w.e, true, false, true, false);
        w.e.call_log = Some(vec![]);
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 5.0);
        w.e.call(0x0070_1bd0, &args![this, -1i32]);
        assert!(calls(&w.e, SET_TRAIT_FROM_INT).is_empty());
    }

    #[test]
    fn a_pad_move_with_a_hotkey_argument_reads_no_buttons() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_GAMEPAD, 1);
        w.e.mem.set_f32(CFG_HIGHLIGHT_TRAIT, 4.0);
        w.e.call_log = Some(vec![]);
        // The result stays -1, which differs from 4 and highlights at once.
        assert_eq!(w.e.call(0x0070_1bd0, &args![this, 4i32]).i32(), -1);
        assert!(calls(&w.e, CHECK_MENU_BUTTON).is_empty());
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 301, u32::MAX]]
        );
    }

    // ----- 00701e00 -----

    #[test]
    fn highlighting_a_hotkey_shows_its_text() {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1e00, &args![this, 4i32]);
        assert_eq!(
            calls(&w.e, SET_TRAIT_FROM_INT),
            vec![vec![WHEEL_TILE, 301, 4]]
        );
        // The text is the hotkey tile's own string trait.
        assert_eq!(calls(&w.e, GET_STRING_TRAIT), vec![vec![tile(4), 0xfc4]]);
        assert_eq!(
            *w.strings.borrow(),
            vec![(WHEEL_TILE, 302, b"TILE TEXT".to_vec())]
        );
        assert_eq!(w.e.get(this, HotKeysWheel::iLastHighlightedHotkey), 4);
        assert!(w.e.get(this, HotKeysWheel::bSelectingHotkey));
    }

    #[test]
    fn highlighting_none_or_hotkey_one_shows_the_empty_text() {
        for hotkey in [-1i32, 1] {
            let mut w = world();
            let this = wheel(&mut w.e);
            w.e.set(this, HotKeysWheel::iLastHighlightedHotkey, 6);
            w.e.call_log = Some(vec![]);
            w.e.call(0x0070_1e00, &args![this, hotkey]);
            assert!(calls(&w.e, GET_STRING_TRAIT).is_empty());
            let shown = calls(&w.e, SET_STRING_TRAIT);
            assert_eq!(shown[0][2], EMPTY_STRING);
            if hotkey < 0 {
                assert_eq!(w.e.get(this, HotKeysWheel::iLastHighlightedHotkey), 6);
                assert!(!w.e.get(this, HotKeysWheel::bSelectingHotkey));
            } else {
                // Hotkey 1 is a valid index for the "remember" part.
                assert_eq!(w.e.get(this, HotKeysWheel::iLastHighlightedHotkey), 1);
                assert!(w.e.get(this, HotKeysWheel::bSelectingHotkey));
            }
        }
    }

    // ----- 00701e80 -----

    struct Use {
        w: World,
        this: Ptr<HotKeysWheel>,
        entry: u32,
    }

    fn use_world(kind: u8) -> Use {
        let mut w = world();
        let this = wheel(&mut w.e);
        w.e.set(this, HotKeysWheel::bSelectingHotkey, true);
        w.e.set(this, HotKeysWheel::iLastHighlightedHotkey, 4);
        set_inventory(&mut w.e, 0x8000);
        w.e.mem.set_u32(CFG_DEFAULT_OBJECT, 0x1234);
        let object = form(&mut w.e, kind, 0, 0);
        let entry = entry(&mut w.e, b"Item", 5, object, 0xe002);
        w.e.mem.set_u32(CFG_HOTKEYS + 4 * 4, entry);
        // The slots of the player's embedded object differ.
        w.e.mem.set_u32(CFG_SLOT_A, 1);
        w.e.mem.set_u32(CFG_SLOT_B, 2);
        w.e.call_log = Some(vec![]);
        Use { w, this, entry }
    }

    #[test]
    fn using_a_hotkey_equips_the_entry() {
        let mut u = use_world(5);
        let player = u.w.player;
        let acquire = u.w.acquire;
        u.w.e.call(0x0070_1e80, &args![u.this]);
        let e = &mut u.w.e;
        let object = e.mem.u32(u.entry + 8);
        // Count 1 for an object that is not a usable 0x28; the extra data.
        assert_eq!(
            calls(e, QUEUE_EQUIP_OBJECT),
            vec![vec![player, object, 1, 0xe002, 0, 0, 1]]
        );
        assert!(calls(e, QUEUE_UNEQUIP_OBJECT).is_empty());
        assert_eq!(calls(e, UPDATE_PLAYER_3D), vec![vec![player]]);
        assert_eq!(calls(e, ACQUIRE_SLOT), vec![vec![acquire, 1, 0, 0]]);
        assert_eq!(calls(e, PLAYER_SLOT), vec![vec![player]]);
        assert_eq!(calls(e, ENTRY_DESTROY), vec![vec![u.entry, 1]]);
        // The hotkey entry was asked for the highlighted hotkey.
        assert_eq!(calls(e, HOTKEY_ENTRY), vec![vec![0x8000, 4]]);
    }

    #[test]
    fn using_a_worn_entry_unequips_it() {
        let mut u = use_world(5);
        u.w.e.mem.set_u32(CFG_WORN, 1);
        let player = u.w.player;
        u.w.e.call(0x0070_1e80, &args![u.this]);
        let e = &mut u.w.e;
        let object = e.mem.u32(u.entry + 8);
        assert_eq!(calls(e, ITEM_GET_WORN), vec![vec![u.entry, 0]]);
        assert_eq!(
            calls(e, QUEUE_UNEQUIP_OBJECT),
            vec![vec![player, object, 1, 0xe002, 0, 0, 1]]
        );
        assert!(calls(e, QUEUE_EQUIP_OBJECT).is_empty());
    }

    #[test]
    fn a_usable_weapon_form_passes_its_count() {
        let mut u = use_world(0x28);
        u.w.e.mem.set_u32(CFG_USABLE, 1);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert_eq!(calls(&u.w.e, QUEUE_EQUIP_OBJECT)[0][2], 5);
        // Not usable: the count is 1.
        let mut u = use_world(0x28);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert_eq!(calls(&u.w.e, QUEUE_EQUIP_OBJECT)[0][2], 1);
    }

    #[test]
    fn a_busy_player_cannot_use_form_types_0x18_and_0x28() {
        for (kind, stops) in [(0x18u8, true), (0x28, true), (5, false)] {
            let mut u = use_world(kind);
            u.w.e.mem.set_i32(CFG_ANIM_ACTION, 7);
            u.w.e.call(0x0070_1e80, &args![u.this]);
            assert_eq!(
                calls(&u.w.e, QUEUE_EQUIP_OBJECT).is_empty(),
                stops,
                "type {kind:#x}"
            );
            // Not busy: any type is used.
            let mut u = use_world(kind);
            u.w.e.call(0x0070_1e80, &args![u.this]);
            assert_eq!(calls(&u.w.e, QUEUE_EQUIP_OBJECT).len(), 1);
        }
    }

    #[test]
    fn the_default_object_shows_a_message_when_the_slots_agree() {
        let mut u = use_world(5);
        let object = u.w.e.mem.u32(u.entry + 8);
        u.w.e.mem.set_u32(CFG_DEFAULT_OBJECT, object);
        u.w.e.mem.set_u32(CFG_SLOT_B, 1);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        let e = &u.w.e;
        assert_eq!(
            calls(e, SHOW_MESSAGE),
            vec![vec![0x7e11, 0, 0, 0, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(e, MESSAGE_TEXT), vec![vec![MESSAGE_SETTING]]);
        assert_eq!(calls(e, SUB_SLOT_A).len(), 1);
        assert!(calls(e, QUEUE_EQUIP_OBJECT).is_empty());
        assert!(calls(e, ENTRY_DESTROY).is_empty());
        // The slots take 0x10 each.
        assert_eq!(calls(e, SUB_SLOT_A)[0][1], 0x10);
        // With different slot answers it equips as usual.
        let mut u = use_world(5);
        let object = u.w.e.mem.u32(u.entry + 8);
        u.w.e.mem.set_u32(CFG_DEFAULT_OBJECT, object);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert!(calls(&u.w.e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls(&u.w.e, QUEUE_EQUIP_OBJECT).len(), 1);
    }

    #[test]
    fn using_needs_the_flag_the_inventory_and_an_entry() {
        let mut u = use_world(5);
        u.w.e.set(u.this, HotKeysWheel::bSelectingHotkey, false);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert!(calls(&u.w.e, QUEUE_EQUIP_OBJECT).is_empty());
        let mut u = use_world(5);
        set_inventory(&mut u.w.e, 0);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert!(calls(&u.w.e, HOTKEY_ENTRY).is_empty());
        assert!(calls(&u.w.e, QUEUE_EQUIP_OBJECT).is_empty());
        let mut u = use_world(5);
        u.w.e.mem.set_u32(CFG_HOTKEYS + 4 * 4, 0);
        u.w.e.call(0x0070_1e80, &args![u.this]);
        assert!(calls(&u.w.e, QUEUE_EQUIP_OBJECT).is_empty());
        assert!(calls(&u.w.e, ENTRY_DESTROY).is_empty());
    }

    // ----- 00702140 -----

    #[test]
    fn the_d_pad_test_needs_the_gamepad() {
        let mut w = world();
        let _this = wheel(&mut w.e);
        press(&mut w.e, true, true, true, true);
        w.e.call_log = Some(vec![]);
        assert!(!w.e.call(0x0070_2140, &args![]).bool());
        assert!(calls(&w.e, CHECK_MENU_BUTTON).is_empty());
    }

    #[test]
    fn the_d_pad_test_stops_at_the_first_pressed_button() {
        let mut w = world();
        let _this = wheel(&mut w.e);
        w.e.mem.set_u32(CFG_GAMEPAD, 1);
        w.e.call_log = Some(vec![]);
        assert!(!w.e.call(0x0070_2140, &args![]).bool());
        let order: Vec<u32> = calls(&w.e, CHECK_MENU_BUTTON)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(order, [2, 1, 4, 3]);
        // Button 4 pressed: 3 is not asked.
        w.e.mem.set_i32(CFG_BUTTONS + 4 * 4, 0);
        w.e.call_log = Some(vec![]);
        assert!(w.e.call(0x0070_2140, &args![]).bool());
        let order: Vec<u32> = calls(&w.e, CHECK_MENU_BUTTON)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(order, [2, 1, 4]);
    }

    #[test]
    fn the_wheel_text_is_at_the_exe_frame_offset() {
        // 00701610 (0x20 bytes): the text at +0xc.
        let mut w = world();
        let this = wheel(&mut w.e);
        let frame = w.e.next_stack_local(0x20);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_1610, &args![this, 0u32]);
        assert_eq!(calls(&w.e, STRING_CONSTRUCT), vec![vec![frame + 0xc]]);
        assert_eq!(calls(&w.e, STRING_DESTROY), vec![vec![frame + 0xc]]);
        // 007017b0 (0x40 bytes): the text at +0x2c.
        let frame = w.e.next_stack_local(0x40);
        w.e.call_log = Some(vec![]);
        w.e.call(0x0070_17b0, &args![this]);
        assert_eq!(calls(&w.e, STRING_CONSTRUCT), vec![vec![frame + 0x2c]]);
    }
}
