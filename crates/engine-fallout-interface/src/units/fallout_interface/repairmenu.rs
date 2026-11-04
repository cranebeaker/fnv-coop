//! `fallout/interface/menus/repairmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The repair menu (`RepairMenu`, class `0x40b`): a `Menu` with thirteen
//! tiles from +0x28 and the repair list (`ListBox<ItemChange *>`) at +0x5c.
//! The singleton `RepairMenu::pMe` is `011da75c`. The constructor and the
//! destructor keep a C++ exception-unwinding frame (`FS:[0]`) that is not
//! translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `RepairMenu::pMe` (Xbox PDB static): the menu singleton.
const REPAIR_MENU: u32 = 0x011d_a75c;
/// The menu class number.
const REPAIR_MENU_CLASS: u32 = 0x40b;
/// `RepairMenu`'s vtable.
const REPAIR_MENU_VTABLE: u32 = 0x0107_5c5c;
/// The four trait ids the constructor looks up (the Xbox PDB lists the
/// statics `iUT_Title`, `iUT_Value`, `iUT_Enabled` and `iUT_IsRepairItem`;
/// which address is which is not confirmed, so they are named by the
/// address of the trait name each is looked up with).
const TRAIT_FROM_NAME_0106ED20: u32 = 0x011d_a74c;
const TRAIT_FROM_NAME_0106ED18: u32 = 0x011d_a754;
const TRAIT_FROM_NAME_01070244: u32 = 0x011d_a750;
const TRAIT_FROM_NAME_01075C44: u32 = 0x011d_a748;

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
const MENU_FILE: u32 = 0x0107_5d14;
const MISSING_TILE_MESSAGE: u32 = 0x0107_5cc0;
const ITEM_TEMPLATE_NAME: u32 = 0x0107_5ca4;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// Tile traits.
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_0XFA9: i32 = 0xfa9;

/// Offset of `lbRepairList` (Xbox PDB).
const REPAIR_LIST: u32 = 0x5c;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(REPAIR_MENU)
}

// Translated from 007b5400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::RepairMenu` (Xbox PDB): constructs the `Menu` base, stores
/// the class's vtable, constructs the repair list at +0x5c and looks up the
/// four trait ids the menu keeps in statics. Returns `this`.
// Translation incomplete: 007b5400 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn repair_menu_repair_menu(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), REPAIR_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.byte_add(REPAIR_LIST)]);
    for (name, slot) in [
        (0x0106_ed20u32, TRAIT_FROM_NAME_0106ED20),
        (0x0106_ed18, TRAIT_FROM_NAME_0106ED18),
        (0x0107_0244, TRAIT_FROM_NAME_01070244),
        (0x0107_5c44, TRAIT_FROM_NAME_01075C44),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    this
}

// Translated from 007b54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::GetClass` (Xbox PDB): the class number `0x40b`.
pub fn repair_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    REPAIR_MENU_CLASS
}

// Translated from 007b54c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 13`.
pub fn repair_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0xd).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(0x28 + 4 * id as u32), tile);
    }
}

// Translated from 007b54f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn repair_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007b5770(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007b5520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::Create` (Xbox PDB, cdecl, no arguments): deletes an open
/// menu of the class, reads the menu file (`01075d14`) and takes its menu
/// as `pMe`. For a menu of this class: attaches the tile, marks it rendered
/// (`00867a20`), checks all thirteen tiles (else prints a message and
/// answers 0), sets the depth for the two stacking types, sets up the repair
/// list on the tile at +0x2c, calls `007a1930(list, 0)`, pokes 1.0 into the
/// trait `011da754` of the tiles at +0x3c and +0x50 and prepares the menu
/// for showing (flag 1). Returns the root tile. A menu of another class is
/// deleted (when it has a tile) and the result is 0.
pub fn repair_menu_create(e: &mut Engine) -> u32 {
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let class = fn_007b5720(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(REPAIR_MENU, menu);
    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_007b5720(e) {
            return finish_create(e, root);
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`repair_menu_create`] after the menu of the class was read.
fn finish_create(e: &mut Engine, root: u32) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    e.call(SET_BYTE_AT_0X1C, &args![menu]);
    if !fn_007b5730(e, Ptr::new(me(e))) {
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
    let list = me(e).wrapping_add(REPAIR_LIST);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, ITEM_TEMPLATE_NAME],
    );
    let list = me(e).wrapping_add(REPAIR_LIST);
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

// Translated from 007b5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x40b`, without an object (no `this`).
pub fn fn_007b5720(_e: &mut Engine) -> u32 {
    REPAIR_MENU_CLASS
}

// Translated from 007b5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all thirteen tiles of `pTiles` are set.
pub fn fn_007b5730(e: &mut Engine, this: Ptr) -> bool {
    (0..0xd).all(|i| e.mem.u32(this.addr().wrapping_add(0x28 + 4 * i)) != 0)
}

// Translated from 007b5770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `RepairMenu` destructor: stores the class's vtable, removes the
/// fade control `0xfa9` of the tile at +0x48, clears `pMe`, destroys the
/// repair list and runs the `Menu` destructor.
// Translation incomplete: 007b5770 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007b5770(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REPAIR_MENU_VTABLE);
    let tile = e.mem.u32(this.addr().wrapping_add(0x48));
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, TRAIT_0XFA9]);
    e.set_global(REPAIR_MENU, 0u32);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(REPAIR_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

/// `RepairMenu::pBrokenItem` (Xbox PDB static): the `ItemChange` being repaired.
const BROKEN_ITEM: u32 = 0x011d_a760;
/// `DoEnter`'s last highlighted index (the knob position `MoveKnob` was given).
const KNOB_INDEX_011DA7D8: u32 = 0x011d_a7d8;
/// The player.
const PLAYER: u32 = 0x011d_ea3c;

/// Tile functions: the integer setter (`00700320`, `RET 8`), the string setter
/// (`00a01350`, `RET 0xc`), `Tile::IsTrue` (`RET 4`), `Tile::GetChildByID` (`RET 4`),
/// `Tile::GetValueQ` (`RET 4`), `Tile::HasFadeControl` (`RET 4`), `Tile::AddFadeControl`
/// (`RET 0x14`), `Tile::GetChildByName` (`RET 4`).
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const TILE_HAS_FADE_CONTROL: u32 = 0x00a0_7ed0;
const TILE_ADD_FADE_CONTROL: u32 = 0x00a0_7c60;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// The tile value's actions (`00a09200`, `00a09130`, `00a09080`, `00a09410`).
const VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
const VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
const VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `ItemChange` methods (`thiscall`): `GetItemHealth` (`004bcdb0`, `RET 4`, `ST0`),
/// `GetWorn` (`004bddd0`, `RET 4`), `HasModEffectActive` (`004bda70`, `RET 4`),
/// `GetIconOne` (`004be200`, `RET 4`), `GetFullName` (`004be2d0`), the form (`0044ddc0`),
/// the extra data lists (`00559450`, the word at +0), the constructor (`004bc550`, `RET 8`).
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
const GET_WORN: u32 = 0x004b_ddd0;
const HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
const GET_ICON_ONE: u32 = 0x004b_e200;
const GET_FULL_NAME: u32 = 0x004b_e2d0;
const ITEM_FORM: u32 = 0x0044_ddc0;
const WORD_AT_ZERO: u32 = 0x0055_9450;
const ITEM_CHANGE_CONSTRUCT: u32 = 0x004b_c550;
/// The list node's item slot (`006815c0`), `006317a0` (`RET 4`: a tile list's element),
/// `005ae3d0` (`RET 4`: a list append), `00905330` (`RET 4`: a list removal).
const NODE_SELF: u32 = 0x0068_15c0;
const LIST_ELEMENT_006317A0: u32 = 0x0063_17a0;
const LIST_APPEND_005AE3D0: u32 = 0x005a_e3d0;
const LIST_REMOVE_00905330: u32 = 0x0090_5330;
/// `ExtraDataList` methods: the constructor (`00410360`), `DuplicateExtraListForContainer`
/// (`RET 4`), `SetCount` (`RET 4`), `SetHealth` (`RET 4`, a `float`), `GetCount` (`AX`),
/// `HasWeaponMods`, `00419f80` (`RET 4`), `CompareListForContainer` (`RET 0xc`).
const EXTRA_LIST_CONSTRUCT: u32 = 0x0041_0360;
const EXTRA_DUPLICATE_FOR_CONTAINER: u32 = 0x0041_2380;
const EXTRA_SET_COUNT: u32 = 0x0041_9ad0;
const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_HAS_WEAPON_MODS: u32 = 0x0041_8ba0;
const EXTRA_00419F80: u32 = 0x0041_9f80;
const EXTRA_COMPARE_FOR_CONTAINER: u32 = 0x0041_26c0;
/// `MemoryManager` allocate (`00401000`, `cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
/// `ActorValueOwner::GetClampedActorValue` (`RET 4`, on +0xa4 of an actor),
/// `GamePlayFormulas::GetRepairedCondition` (`cdecl`, four words, `ST0`), `0040ebd0`
/// (`cdecl`, two `float`s, `ST0`), `00404010` (`cdecl`, two `float`s, `ST0`),
/// `__RTDynamicCast` (`cdecl`, five words), `TESObjectWEAP::GetFormHealth` (`004bcf00`,
/// `RET 4`), `TESHealthForm::GetFormHealth` (`004873d0`, `cdecl`),
/// `InventoryChanges::GetInventoryChanges` (`cdecl`), `InventoryChanges::GetObjectInList`
/// (`RET 0xc`), `InventoryChanges::GetRepairItemGroup` (`004d4bd0`, `RET 8`),
/// `MiscStatManager::Increment` (`cdecl`), `0063f7b0` (`thiscall`).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const GET_REPAIRED_CONDITION: u32 = 0x0064_8090;
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const WEAPON_GET_FORM_HEALTH: u32 = 0x004b_cf00;
const HEALTH_FORM_GET_FORM_HEALTH: u32 = 0x0048_73d0;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_OBJECT_IN_LIST: u32 = 0x004b_fba0;
const GET_REPAIR_ITEM_GROUP: u32 = 0x004d_4bd0;
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const ITEM_0063F7B0: u32 = 0x0063_f7b0;
/// `InterfaceManager::PlayMenuSound` (`cdecl`), `Interface::PlayMenuSound` (`00706f30`,
/// `cdecl`), `Interface::SetInventoryMenuVisible` (`007048f0`, `cdecl`, three words),
/// `007044c0` (`cdecl`, three words), `InventoryMenu::UpdateInventoryMenu` (`00782a90`, no
/// arguments), the tutorial message (`00718630`, `RET 0xc`, on +0x4d4 of the interface
/// manager), `00715860` (`RET 0xc`: the current focus target), `004b71d0` (no arguments, a
/// `bool`), `00704370` (`thiscall` on the interface manager), `FOPipboyManager::MoveKnob`
/// (`RET 0x10`).
const INTERFACE_MANAGER_PLAY_MENU_SOUND: u32 = 0x0071_7280;
const INTERFACE_PLAY_MENU_SOUND: u32 = 0x0070_6f30;
const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
const INTERFACE_007044C0: u32 = 0x0070_44c0;
const UPDATE_INVENTORY_MENU: u32 = 0x0078_2a90;
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const FLAG_004B71D0: u32 = 0x004b_71d0;
const PIPBOY_MANAGER_00704370: u32 = 0x0070_4370;
const PIPBOY_MOVE_KNOB: u32 = 0x007f_8610;
/// The list box (`ListBox<ItemChange *>`) methods of `bartermenu.cpp`/`listbox.cpp`: the
/// current item (`007a1910`), `RemoveItem` (`0071a5c0`, `RET 4`), `NumUnfiltered`
/// (`0071ae60`), `AddItem` (`0071a320`, `RET 0x10`), the walk (`007314c0`, `RET 0xc`).
const LIST_BOX_CURRENT_ITEM: u32 = 0x007a_1910;
const LIST_BOX_REMOVE_ITEM: u32 = 0x0071_a5c0;
const LIST_BOX_NUM_UNFILTERED: u32 = 0x0071_ae60;
const LIST_BOX_ADD_ITEM: u32 = 0x0071_a320;
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// The embedded list's constructor (`0096a2d0`) and destructor (`0046ffb0`), the next node
/// (the word at +4).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `BSAudio::QInstance`, `BSAudio::GetSoundHandleByName` (`RET 0xc`),
/// `BSSoundHandle::Play` (`RET 4`), the sound handle's destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// The game settings' string (`00403df0`) and value address (`00403e20`).
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The string functions (`cdecl`): format (`00406d00`), copy (`00406d30`), append
/// (`00406d50`), `00408b20` (two strings compared).
const FORMAT: u32 = 0x0040_6d00;
const COPY: u32 = 0x0040_6d30;
const APPEND: u32 = 0x0040_6d50;
const STRINGS_00408B20: u32 = 0x0040_8b20;
/// Form functions: the type (`00401170`), `CombatFormulas::CalcWeaponDamageForDisplay`
/// (`cdecl`, four words, `ST0`), `004bd510` (`cdecl`, two `float`s, `ST0`), `_ftol2`,
/// the armour values `004be060` and `004be180` (`thiscall`, `ST0`),
/// `CombatFormulas::CalcArmorRating` (`cdecl`: a word and a `float`, `ST0`),
/// `00476b20` (`cdecl`, a `float`, `ST0`).
const FORM_TYPE: u32 = 0x0040_1170;
const CALC_WEAPON_DAMAGE_FOR_DISPLAY: u32 = 0x0064_50f0;
const ROUND_004BD510: u32 = 0x004b_d510;
const FTOL2: u32 = 0x00ec_62c0;
const ARMOR_004BE060: u32 = 0x004b_e060;
const ARMOR_004BE180: u32 = 0x004b_e180;
const CALC_ARMOR_RATING: u32 = 0x0064_6360;
const ROUND_00476B20: u32 = 0x0047_6b20;

/// The RTTI descriptors of the weapon cast (`01183108` to `01183998`).
const RTTI_01183108: u32 = 0x0118_3108;
const RTTI_01183998: u32 = 0x0118_3998;
/// `"UIRepairWeapon"`, `"_line_alpha"`, `"%"`, the percent format `010734b0`, `"ENGLISH"`,
/// `"RM_Headline_Title"`, `"--"`.
const REPAIR_SOUND_TEXT: u32 = 0x0107_5d38;
const LINE_ALPHA_TEXT: u32 = 0x0107_27cc;
const PERCENT_TEXT: u32 = 0x0101_2320;
const PERCENT_FORMAT_010734B0: u32 = 0x0107_34b0;
const ENGLISH_TEXT: u32 = 0x0101_8348;
const HEADLINE_TITLE_TEXT: u32 = 0x0107_5d48;
const DASHES_TEXT: u32 = 0x0104_7008;
/// `double`s: `0.0`, `0.5`, `0.6`, `10.0`, `99.0`, `100.0`, `255.0`; `float`s: `2.0`,
/// `50.0`, `64.0`, `75.0`, `128.0`, `255.0`.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HALF_DOUBLE: u32 = 0x0101_1588;
const SIX_TENTHS_DOUBLE: u32 = 0x0102_90b8;
const TEN_DOUBLE: u32 = 0x0102_0758;
const NINETY_NINE_DOUBLE: u32 = 0x0107_5d30;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const TWO_FIFTY_FIVE_DOUBLE: u32 = 0x0101_e568;
const TWO_FLOAT: u32 = 0x0101_62c0;
const FIFTY_FLOAT: u32 = 0x0101_b268;
const SIXTY_FOUR_FLOAT: u32 = 0x0101_e6ec;
const SEVENTY_FIVE_FLOAT: u32 = 0x0102_f0f8;
const HALF_ALPHA_FLOAT: u32 = 0x0101_e704;
const FULL_ALPHA_FLOAT: u32 = 0x0102_3cd8;

/// The game settings the menu shows (`00403df0`) or reads (`00403e20`).
const SETTING_011D2C7C: u32 = 0x011d_2c7c;
const SETTING_011D396C: u32 = 0x011d_396c;
const SETTING_011D20C4: u32 = 0x011d_20c4;
const SETTING_011D3DCC: u32 = 0x011d_3dcc;
const SETTING_011D1F2C: u32 = 0x011d_1f2c;
const SETTING_011D270C: u32 = 0x011d_270c;
const SETTING_011D4EE8: u32 = 0x011d_4ee8;
const SETTING_011D018C: u32 = 0x011d_018c;
const SETTING_011D2B20: u32 = 0x011d_2b20;
const SETTING_011DB2F0: u32 = 0x011d_b2f0;
const SETTING_011DB358: u32 = 0x011d_b358;
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
const SETTING_011D4378: u32 = 0x011d_4378;
const SETTING_011D4030: u32 = 0x011d_4030;
const SETTING_011D4DA4: u32 = 0x011d_4da4;
const SETTING_011D29F4: u32 = 0x011d_29f4;

/// The broken item.
fn broken_item(e: &Engine) -> u32 {
    e.global::<u32>(BROKEN_ITEM)
}

/// The tile word at `offset` of the open menu.
fn menu_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(me(e).wrapping_add(offset))
}

/// An `ItemChange`'s first extra data list: 0 without a list, else the first node's item.
fn first_extra_list(e: &mut Engine, item: u32) -> u32 {
    if e.call(WORD_AT_ZERO, &args![item]).u32() == 0 {
        return 0;
    }
    let list = e.call(WORD_AT_ZERO, &args![item]).u32();
    let slot = e.call(NODE_SELF, &args![list]).u32();
    e.mem.u32(slot)
}

/// `first_extra_list` of the item of `node`, the item read again through `006815c0` each
/// time as the code does.
fn first_extra_list_of_node(e: &mut Engine, node: u32) -> u32 {
    let item = node_item(e, node);
    if e.call(WORD_AT_ZERO, &args![item]).u32() == 0 {
        return 0;
    }
    let item = node_item(e, node);
    let list = e.call(WORD_AT_ZERO, &args![item]).u32();
    let slot = e.call(NODE_SELF, &args![list]).u32();
    e.mem.u32(slot)
}

/// The item word of a list node, reached through `006815c0`.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The setting's string set as trait `trait_id` of `tile` (flag 1).
fn set_setting_text(e: &mut Engine, tile_offset: u32, trait_id: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let tile = menu_tile(e, tile_offset);
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);
}

/// Plays the named sound: `BSAudio::GetSoundHandleByName(QInstance, handle, name, 0x121)`,
/// `Play(0)`, the handle destroyed.
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// `__RTDynamicCast(object, 0, 01183108, 01183998, 0)`: the weapon of an item's form.
fn weapon_of(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_01183108, RTTI_01183998, 0u32],
    )
    .u32()
}

/// The repaired condition fraction: `0040ebd0(GetRepairedCondition(player's skill 0x27,
/// broken item's health, item's health, extra) / 100, 1.0)`, as a `float`.
fn repaired_fraction(e: &mut Engine, item: u32, extra: u32) -> u32 {
    let item_health = x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64());
    let broken = broken_item(e);
    let broken_health = x87_store32(e.call(GET_ITEM_HEALTH, &args![broken, 1u32]).f64());
    let player = e.global::<u32>(PLAYER);
    let skill = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![player + 0xa4, 0x27u32])
        .u32();
    let condition = e
        .call(
            GET_REPAIRED_CONDITION,
            &args![skill, broken_health, item_health, extra],
        )
        .f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let fraction = x87_store32(x87_div(condition, hundred));
    let limited = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(fraction), 1.0f32.to_bits()],
        )
        .f64();
    x87_store32(limited)
}

// Translated from 007b57f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::InitFunc` (Xbox PDB, `cdecl`: a tile and its `ItemChange`): with both, gives
/// the tile trait `0xfaa` 0xe, shows it (`0xfa3`) when its index (`0xfac`) is not below
/// `0.0`, sets the health bar (`0xfb1` of child 0xf) to the item's health times 0.6, marks a
/// worn item (`0xfa3` of the tile list's element), and sets `0x1004` to the condition the
/// broken item would reach repaired with it (`GetRepairedCondition / 100`, at most 1).
pub fn repair_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    if tile == 0 || item == 0 {
        return;
    }
    let health = x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64());
    e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0xeu32]);
    let index = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfacu32]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP double 0.0; TEST AH,1; JNZ`: hidden when below `0.0` or NaN.
    let shown = !matches!(
        index.partial_cmp(&zero),
        Some(std::cmp::Ordering::Less) | None
    );
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, u32::from(shown)]);
    let factor: f64 = e.global(SIX_TENTHS_DOUBLE);
    let bar = x87_store32(x87_mul(x87_m32(health), factor));
    let child = e.call(TILE_GET_CHILD_BY_ID, &args![tile, 0xfu32]).u32();
    e.call(
        SET_FLOAT_TRAIT,
        &args![child, 0xfb1u32, x87_quiet_bits(bar), 1u32],
    );
    if e.call(GET_WORN, &args![item, 0u32]).bool() {
        let index = e.call(WORD_AT_ZERO, &args![tile + 4]).u32();
        let slot = e.call(LIST_ELEMENT_006317A0, &args![tile + 4, index]).u32();
        let worn = e.mem.u32(slot);
        e.call(TILE_SET_INT, &args![worn, 0xfa3u32, 1u32]);
    }
    let broken = broken_item(e);
    let broken_health = x87_store32(e.call(GET_ITEM_HEALTH, &args![broken, 1u32]).f64());
    let fraction = e.with_stack(4, |e, extra| {
        e.mem.set_u32(extra.addr(), 0xffff_ffff);
        let player = e.global::<u32>(PLAYER);
        let skill = e
            .call(GET_CLAMPED_ACTOR_VALUE, &args![player + 0xa4, 0x27u32])
            .u32();
        let condition = e
            .call(
                GET_REPAIRED_CONDITION,
                &args![skill, broken_health, health, extra],
            )
            .f64();
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        x87_store32(x87_div(condition, hundred))
    });
    let limited = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(fraction), 1.0f32.to_bits()],
        )
        .f64();
    e.call(
        SET_FLOAT_TRAIT,
        &args![tile, 0x1004u32, x87_store32(limited), 1u32],
    );
}

/// `fn_007b5950`'s "below full alpha": the alpha (`0xfa9`) of the item's tile (`007b5370` on
/// the list item's tile + 4) below `255.0` (`FCOMP; TEST AH,5; JP`: ordered).
fn below_full_alpha(e: &mut Engine, tile: u32) -> bool {
    let slot = e.call(ITEM_TILE_007B5370, &args![tile + 4]).u32();
    let item_tile = e.mem.u32(slot);
    let alpha = e.call(GET_FLOAT_TRAIT, &args![item_tile, 0xfa9u32]).f64();
    alpha < e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE)
}

/// `007b5370` (`thiscall`): the item's tile slot.
const ITEM_TILE_007B5370: u32 = 0x007b_5370;

/// The health order of two `ItemChange`s as the compare functions read it: 1 when the
/// second's health is above the first's, -1 when below, 0 otherwise (or unordered).
fn health_order(e: &mut Engine, first: u32, second: u32, flag: u32) -> i32 {
    let a = e.call(GET_ITEM_HEALTH, &args![first, flag]).f64();
    let b = e.call(GET_ITEM_HEALTH, &args![second, flag]).f64();
    if b > a {
        return 1;
    }
    let a = e.call(GET_ITEM_HEALTH, &args![first, flag]).f64();
    let b = e.call(GET_ITEM_HEALTH, &args![second, flag]).f64();
    if b < a {
        -1
    } else {
        0
    }
}

// Translated from 007b5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`: two list items; the repair list's sort order): items whose tile has
/// `iUT_IsRepairItem`-like trait `011da748` set come first, then items at full alpha; then
/// by health (higher first, the order reversed when both are below full alpha); then worn
/// items first.
pub fn fn_007b5950(e: &mut Engine, first: u32, second: u32) -> i32 {
    let trait_id = e.global::<u32>(TRAIT_FROM_NAME_01075C44);
    let first_tile = e.mem.u32(first);
    let first_set = e.call(TILE_IS_TRUE, &args![first_tile, trait_id]).bool();
    let trait_id = e.global::<u32>(TRAIT_FROM_NAME_01075C44);
    let second_tile = e.mem.u32(second);
    let second_set = e.call(TILE_IS_TRUE, &args![second_tile, trait_id]).bool();
    let first_tile = e.mem.u32(first);
    let first_dim = below_full_alpha(e, first_tile);
    let second_tile = e.mem.u32(second);
    let second_dim = below_full_alpha(e, second_tile);
    if first_set && !second_set {
        return -1;
    }
    if !first_set && second_set {
        return 1;
    }
    if first_dim && !second_dim {
        return 1;
    }
    if !first_dim && second_dim {
        return -1;
    }
    let first_item = e.mem.u32(first + 4);
    let second_item = e.mem.u32(second + 4);
    let mut order = health_order(e, first_item, second_item, 1);
    if first_dim && second_dim {
        order = order.wrapping_neg();
    }
    if order == 0 {
        if e.call(GET_WORN, &args![first_item, 0u32]).bool() {
            order = -1;
        } else if e.call(GET_WORN, &args![second_item, 0u32]).bool() {
            order = 1;
        }
    }
    order
}

// Translated from 007b5b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b5b40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `RepairMenu::DoClick` (Xbox PDB; `RET 8`: id, tile; without a tile an id below 13 takes
/// `pTiles[id]`). Id 0xc plays sound `0x24` and shows the inventory menu again
/// (`SetInventoryMenuVisible(1, 0, 0)`). Id 0xe, when the tile's item is at full alpha (not
/// below `255.0`), is not the broken item and the tile at +0x4c is shown: repairs the
/// broken item with the list's current item, removes that item from the player (vtable
/// +0x17c), plays `"UIRepairWeapon"`, removes it from the list, then clicks 0xc when the
/// list has one item left or the broken item is at or above `99.0`, else sets the menu up
/// again; a worn item updates the player (vtable +0x3c4) and the inventory menu.
pub fn repair_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let tile = if tile == 0 && (0..0xd).contains(&id) {
        e.mem.u32(this.addr().wrapping_add(0x28 + 4 * id as u32))
    } else {
        tile
    };
    let base = this.addr();
    if id == 0xc {
        e.call(INTERFACE_MANAGER_PLAY_MENU_SOUND, &args![0x24u32]);
        e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 0u32]);
        return;
    }
    if id != 0xe {
        return;
    }
    let slot = e.call(ITEM_TILE_007B5370, &args![tile + 4]).u32();
    let item_tile = e.mem.u32(slot);
    let alpha = e.call(GET_FLOAT_TRAIT, &args![item_tile, 0xfa9u32]).f64();
    if alpha < e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE) {
        return;
    }
    if e.call(LIST_BOX_CURRENT_ITEM, &args![base + REPAIR_LIST])
        .u32()
        == broken_item(e)
    {
        return;
    }
    let shown_tile = e.mem.u32(base + 0x4c);
    if !e.call(TILE_IS_TRUE, &args![shown_tile, 0xfa3u32]).bool() {
        return;
    }
    let item = e
        .call(LIST_BOX_CURRENT_ITEM, &args![base + REPAIR_LIST])
        .u32();
    let extra = first_extra_list(e, item);
    repair_menu_repair_item(e, this, item);
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let player = e.global::<u32>(PLAYER);
    e.vcall(
        player,
        0x17c,
        &args![form, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    play_sound(e, REPAIR_SOUND_TEXT);
    let selected = e.vcall(base + REPAIR_LIST, 4, &args![]).u32();
    e.call(LIST_BOX_REMOVE_ITEM, &args![base + REPAIR_LIST, selected]);
    // `FCOMP double 99.0; TEST AH,1; JNZ`: set up again when below `99.0` (or NaN).
    let last = e
        .call(LIST_BOX_NUM_UNFILTERED, &args![base + REPAIR_LIST])
        .u32()
        == 1;
    let done = last || {
        let broken = broken_item(e);
        let health = e.call(GET_ITEM_HEALTH, &args![broken, 1u32]).f64();
        !matches!(
            health.partial_cmp(&e.global::<f64>(NINETY_NINE_DOUBLE)),
            Some(std::cmp::Ordering::Less) | None
        )
    };
    if done {
        e.vcall(base, 0xc, &args![0xcu32, 0u32]);
    } else {
        repair_menu_init_repair_menu(e, 0);
    }
    if e.call(GET_WORN, &args![item, 0u32]).bool() {
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3c4, &args![]);
        e.call(UPDATE_INVENTORY_MENU, &args![]);
    }
}

// Translated from 007b5d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b5d80 - the C++ exception frame (FS:[0] chain, unwind state 0 around the extra data list allocation) is not translated.
/// `RepairMenu::RepairItem` (Xbox PDB; `RET 4`: the `ItemChange` used up). Computes the
/// repaired condition fraction (`GetRepairedCondition / 100`, at most 1) times the broken
/// item's full health (a weapon's `GetFormHealth` with its mod effect 10, else
/// `TESHealthForm::GetFormHealth`), builds a new extra data list (a copy of the broken
/// item's first, count 1, that health), counts the stat `0x22`, calls `0063f7b0` on the
/// broken item's lists and lowers the old list's count. A matching stack in the player's
/// inventory entry gets one more (and the new list goes into the broken item's lists);
/// otherwise the new list is added to both lists, or (for a single item) the old list takes
/// the new health and goes into the broken item's lists. The new list is deleted when a
/// stack took it or for a single item; a single item's old list is removed from the
/// entry's lists when a stack took it.
pub fn repair_menu_repair_item(e: &mut Engine, _this: Ptr, item: u32) {
    let mut fraction = repaired_fraction(e, item, 0);
    let broken = broken_item(e);
    let form = e.call(ITEM_FORM, &args![broken]).u32();
    let weapon = weapon_of(e, form);
    if weapon != 0 {
        let broken = broken_item(e);
        let effect = e.call(HAS_MOD_EFFECT_ACTIVE, &args![broken, 0xau32]).u8();
        let health = e
            .call(WEAPON_GET_FORM_HEALTH, &args![weapon, u32::from(effect)])
            .i32();
        fraction = x87_store32(x87_mul(f64::from(health), x87_m32(fraction)));
    } else {
        let broken = broken_item(e);
        let form = e.call(ITEM_FORM, &args![broken]).u32();
        let health = e.call(HEALTH_FORM_GET_FORM_HEALTH, &args![form]).u32();
        fraction = x87_store32(x87_mul(f64::from(health), x87_m32(fraction)));
    }
    let broken = broken_item(e);
    let form = e.call(ITEM_FORM, &args![broken]).u32();
    let player = e.global::<u32>(PLAYER);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    let entry = e
        .call(GET_OBJECT_IN_LIST, &args![changes, form, 1u32, 0u32])
        .u32();
    let broken = broken_item(e);
    let old = first_extra_list(e, broken);
    e.with_stack(8, |e, locals| {
        let new_slot = locals.addr();
        let old_slot = locals.addr() + 4;
        e.mem.set_u32(old_slot, old);
        let block = e.call(ALLOCATE, &args![0x20u32]).u32();
        let list = if block != 0 {
            e.call(EXTRA_LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(new_slot, list);
        let count = i32::from(e.call(EXTRA_GET_COUNT, &args![old]).u32() as u16 as i16);
        e.call(EXTRA_DUPLICATE_FOR_CONTAINER, &args![list, old]);
        e.call(EXTRA_SET_COUNT, &args![list, 1u32]);
        e.call(EXTRA_SET_HEALTH, &args![list, x87_quiet_bits(fraction)]);
        e.call(MISC_STAT_INCREMENT, &args![0x22u32]);
        let broken = broken_item(e);
        let lists = e.call(WORD_AT_ZERO, &args![broken]).u32();
        e.call(ITEM_0063F7B0, &args![lists]);
        if count > 1 {
            e.call(EXTRA_SET_COUNT, &args![old, count - 1]);
        }
        let mut stacked = false;
        let mut node = e.call(WORD_AT_ZERO, &args![entry]).u32();
        while node != 0 && node_item(e, node) != 0 {
            let other = node_item(e, node);
            if !e
                .call(EXTRA_COMPARE_FOR_CONTAINER, &args![other, list, 1u32, 0u32])
                .bool()
            {
                let other = node_item(e, node);
                if !e
                    .call(EXTRA_COMPARE_FOR_CONTAINER, &args![list, other, 1u32, 0u32])
                    .bool()
                {
                    let other = node_item(e, node);
                    let other_count =
                        i32::from(e.call(EXTRA_GET_COUNT, &args![other]).u32() as u16 as i16);
                    let other = node_item(e, node);
                    e.call(EXTRA_SET_COUNT, &args![other, other_count + 1]);
                    let slot = e.call(NODE_SELF, &args![node]).u32();
                    let broken = broken_item(e);
                    let lists = e.call(WORD_AT_ZERO, &args![broken]).u32();
                    e.call(LIST_APPEND_005AE3D0, &args![lists, slot]);
                    stacked = true;
                }
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        if !stacked {
            if count > 1 {
                let lists = e.call(WORD_AT_ZERO, &args![entry]).u32();
                e.call(LIST_APPEND_005AE3D0, &args![lists, new_slot]);
                let broken = broken_item(e);
                let lists = e.call(WORD_AT_ZERO, &args![broken]).u32();
                e.call(LIST_APPEND_005AE3D0, &args![lists, new_slot]);
            } else {
                let old = e.mem.u32(old_slot);
                e.call(EXTRA_SET_HEALTH, &args![old, x87_quiet_bits(fraction)]);
                let broken = broken_item(e);
                let lists = e.call(WORD_AT_ZERO, &args![broken]).u32();
                e.call(LIST_APPEND_005AE3D0, &args![lists, old_slot]);
            }
        } else if count == 1 {
            let lists = e.call(WORD_AT_ZERO, &args![entry]).u32();
            e.call(LIST_REMOVE_00905330, &args![lists, old_slot]);
        }
        if stacked || count == 1 {
            let list = e.mem.u32(new_slot);
            e.call(EXTRA_00419F80, &args![list, 0u32]);
            let list = e.mem.u32(new_slot);
            if list != 0 {
                e.vcall(list, 0, &args![1u32]);
            }
        }
    });
}

/// The setting's string as the title (`TRAIT_FROM_NAME_0106ED20`) of the menu's tile.
fn set_title(e: &mut Engine, tile_offset: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
    let tile = menu_tile(e, tile_offset);
    e.call(TILE_SET_STRING, &args![tile, title, text, 1u32]);
}

/// A `float` truncated by `_ftol2` as the value (`TRAIT_FROM_NAME_0106ED18`) of the menu's
/// tile.
fn set_value_int(e: &mut Engine, tile_offset: u32, bits: u32) {
    let whole = e.call(FTOL2, &args![x87_load32(bits)]).u32();
    let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = menu_tile(e, tile_offset);
    e.call(TILE_SET_INT, &args![tile, value_trait, whole]);
}

/// A text as the value (`TRAIT_FROM_NAME_0106ED18`) of the menu's tile.
fn set_value_text(e: &mut Engine, tile_offset: u32, text: Ptr) {
    let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = menu_tile(e, tile_offset);
    e.call(TILE_SET_STRING, &args![tile, value_trait, text, 1u32]);
}

/// `InitRepairMenu`'s armour line in the tile at +0x3c: the rating `damaged` (the
/// `004be180` one) titled by setting `011d396c` when above `0.0`, else `base` titled by
/// `011d20c4` when above `0.0`, else `"--"` (copied into `buffer`) titled by `011d20c4`.
fn show_armor_ratings(e: &mut Engine, base: u32, damaged: u32, buffer: Ptr) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if above_zero(x87_load32(damaged), zero) {
        set_title(e, 0x3c, SETTING_011D396C);
        set_value_int(e, 0x3c, damaged);
    } else if above_zero(x87_load32(base), zero) {
        set_title(e, 0x3c, SETTING_011D20C4);
        set_value_int(e, 0x3c, base);
    } else {
        e.call(COPY, &args![buffer, 0x80u32, DASHES_TEXT]);
        set_title(e, 0x3c, SETTING_011D20C4);
        set_value_text(e, 0x3c, buffer);
    }
}

/// `DoEnter`'s armour lines (the broken item now in +0x3c, repaired in +0x50; `buffer`
/// holds `"--"`): the `004be180` ratings titled by setting `011d396c` when the repaired one
/// is above `0.0`, else the `004be060` ratings titled by `011d20c4` when the repaired one is
/// above `0.0`, else `"--"` in both. The current rating shows `"--"` unless above `0.0`.
fn show_armor_change(e: &mut Engine, buffer: Ptr, ratings: [u32; 4]) {
    let [now_base, now_damaged, then_base, then_damaged] = ratings;
    let zero: f64 = e.global(ZERO_DOUBLE);
    let chosen = if above_zero(x87_load32(then_damaged), zero) {
        Some((SETTING_011D396C, now_damaged, then_damaged))
    } else if above_zero(x87_load32(then_base), zero) {
        Some((SETTING_011D20C4, now_base, then_base))
    } else {
        None
    };
    match chosen {
        Some((setting, now, then)) => {
            set_title(e, 0x3c, setting);
            if above_zero(x87_load32(now), zero) {
                set_value_int(e, 0x3c, now);
            } else {
                set_value_text(e, 0x3c, buffer);
            }
            set_title(e, 0x50, setting);
            set_value_int(e, 0x50, then);
        }
        None => {
            set_title(e, 0x3c, SETTING_011D20C4);
            set_value_text(e, 0x3c, buffer);
            set_title(e, 0x50, SETTING_011D20C4);
            set_value_text(e, 0x50, buffer);
        }
    }
}

/// `FCOMP double 0.0; TEST AH,0x41; JNZ`: on only when strictly above (ordered).
fn above_zero(value: f64, zero: f64) -> bool {
    value.partial_cmp(&zero) == Some(std::cmp::Ordering::Greater)
}

/// An armour form's two ratings at `fraction` (`CalcArmorRating(_ftol2(004be060(form)),
/// fraction)` and the same with `004be180`), as `float`s.
fn armor_ratings(e: &mut Engine, form: u32, fraction: u32) -> (u32, u32) {
    let base = e.call(ARMOR_004BE060, &args![form]).f64();
    let base = e.call(FTOL2, &args![base]).u32();
    let first = e
        .call(CALC_ARMOR_RATING, &args![base, x87_quiet_bits(fraction)])
        .f64();
    let first = x87_store32(first);
    let damaged = e.call(ARMOR_004BE180, &args![form]).f64();
    let damaged = e.call(FTOL2, &args![damaged]).u32();
    let second = e
        .call(CALC_ARMOR_RATING, &args![damaged, x87_quiet_bits(fraction)])
        .f64();
    (first, x87_store32(second))
}

/// A weapon's displayed damage at `fraction`: `_ftol2(004bd510(
/// CalcWeaponDamageForDisplay(form, fraction, item, 0), 1.0))`.
fn weapon_damage(e: &mut Engine, form: u32, fraction: u32, item: u32) -> u32 {
    let damage = e
        .call(
            CALC_WEAPON_DAMAGE_FOR_DISPLAY,
            &args![form, x87_quiet_bits(fraction), item, 0u32],
        )
        .f64();
    let rounded = e
        .call(
            ROUND_004BD510,
            &args![x87_store32(damage), 1.0f32.to_bits()],
        )
        .f64();
    e.call(FTOL2, &args![rounded]).u32()
}

/// Sets `_line_alpha` of the tile at +0x54 of `this` to the `float` at `alpha`.
fn set_line_alpha(e: &mut Engine, this: Ptr, alpha: u32) {
    let value = x87_quiet_bits(e.global::<u32>(alpha));
    let line_alpha = e.call(TILE_TEXT_TO_TRAIT, &args![LINE_ALPHA_TEXT]).u32();
    let tile = e.mem.u32(this.addr() + 0x54);
    e.call(SET_FLOAT_TRAIT, &args![tile, line_alpha, value, 1u32]);
}

// Translated from 007b6120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b6120 - the /GS security cookie check (00ec408c) is not translated
/// `RepairMenu::DoEnter` (Xbox PDB; `RET 8`: id, tile). Only id 0xe: moves the Pip-Boy knob
/// (`MoveKnob(0, ±setting 011db2f0, setting 011db358, 1)`) when the highlighted index
/// (`0xfac` of the list's highlighted tile) changed. With a current item and a broken item:
/// the tile at +0x54 says whether the item is in good shape (setting `011d3dcc` at or above
/// 75, or 50 for armour, else `011d1f2c`). For the broken item itself: an error (setting
/// `011d270c`) when the list has more items, and `_line_alpha` 128. For another item whose
/// repair cost (`0x1005` of the tile) is above 0: an error (`011d4ee8` with the condition
/// limit times 10 above 100, else `011d2b20` with the cost) and `_line_alpha` 128.
/// Otherwise `_line_alpha` 255, the repaired condition on the tile at +0x4c, the weapon's
/// damage or the armour's ratings at it, and the condition and value gains (in percent,
/// rounded, not below 0) on the tiles at +0x40 and +0x44.
pub fn repair_menu_do_enter(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if id != 0xe {
        return;
    }
    let base = this.addr();
    let current = e
        .call(LIST_BOX_CURRENT_ITEM, &args![base + REPAIR_LIST])
        .u32();
    let index = if current != 0 {
        let highlighted = e.vcall(base + REPAIR_LIST, 4, &args![]).u32();
        let value = e.call(GET_FLOAT_TRAIT, &args![highlighted, 0xfacu32]).f64();
        e.call(FTOL2, &args![value]).i32()
    } else {
        0
    };
    if e.global::<i32>(KNOB_INDEX_011DA7D8) != index {
        let step = if e.global::<i32>(KNOB_INDEX_011DA7D8) < index {
            let value = e.call(SETTING_VALUE, &args![SETTING_011DB2F0]).u32();
            x87_quiet_bits(e.mem.u32(value))
        } else {
            let value = e.call(SETTING_VALUE, &args![SETTING_011DB2F0]).u32();
            x87_store32(-x87_load32(e.mem.u32(value)))
        };
        let value = e.call(SETTING_VALUE, &args![SETTING_011DB358]).u32();
        let speed = x87_quiet_bits(e.mem.u32(value));
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let pipboy = e.call(PIPBOY_MANAGER_00704370, &args![interface]).u32();
        e.call(PIPBOY_MOVE_KNOB, &args![pipboy, 0u32, step, speed, 1u32]);
        e.set_global(KNOB_INDEX_011DA7D8, index);
    }
    if current == 0 || broken_item(e) == 0 {
        return;
    }
    let mut threshold = x87_quiet_bits(e.global::<u32>(SEVENTY_FIVE_FLOAT));
    let form = e.call(ITEM_FORM, &args![current]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == 0x18 {
        threshold = x87_quiet_bits(e.global::<u32>(FIFTY_FLOAT));
    }
    let health = e.call(GET_ITEM_HEALTH, &args![current, 1u32]).f64();
    // `FLD threshold; FCOMPP; TEST AH,0x41; JP`: good when the threshold is at or below the
    // health (ordered).
    let setting = if x87_load32(threshold) <= health {
        SETTING_011D3DCC
    } else {
        SETTING_011D1F2C
    };
    set_setting_text(e, 0x54, 0xfc4, setting);
    if current == broken_item(e) {
        if e.call(LIST_BOX_NUM_UNFILTERED, &args![base + REPAIR_LIST])
            .u32()
            > 1
        {
            let text = e.call(SETTING_STRING, &args![SETTING_011D270C]).u32();
            repair_menu_show_error_message(e, this, text);
        }
        set_line_alpha(e, this, HALF_ALPHA_FLOAT);
        return;
    }
    let condition = x87_store32(e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f64());
    let cost_value = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1005u32]).f64();
    let cost = e.call(FTOL2, &args![cost_value]).i32();
    if cost > 100 {
        e.with_stack(0x40, |e, buffer| {
            let value = e.call(SETTING_VALUE, &args![SETTING_011D018C]).u32();
            let ten: f64 = e.global(TEN_DOUBLE);
            let limit = x87_mul(x87_m32(e.mem.u32(value)), ten).to_bits();
            let format = e.call(SETTING_STRING, &args![SETTING_011D4EE8]).u32();
            e.call(
                FORMAT,
                &args![
                    buffer,
                    0x40u32,
                    format,
                    limit as u32,
                    (limit >> 32) as u32,
                    PERCENT_TEXT
                ],
            );
            repair_menu_show_error_message(e, this, buffer.addr());
        });
        set_line_alpha(e, this, HALF_ALPHA_FLOAT);
        return;
    }
    if cost > 0 {
        e.with_stack(0x40, |e, buffer| {
            let format = e.call(SETTING_STRING, &args![SETTING_011D2B20]).u32();
            e.call(FORMAT, &args![buffer, 0x40u32, format, cost, PERCENT_TEXT]);
            repair_menu_show_error_message(e, this, buffer.addr());
        });
        set_line_alpha(e, this, HALF_ALPHA_FLOAT);
        return;
    }
    set_line_alpha(e, this, FULL_ALPHA_FLOAT);
    let tile4c = e.mem.u32(base + 0x4c);
    e.call(
        SET_FLOAT_TRAIT,
        &args![tile4c, 0x1004u32, x87_quiet_bits(condition), 1u32],
    );
    let tile4c = e.mem.u32(base + 0x4c);
    e.call(TILE_SET_INT, &args![tile4c, 0xfa3u32, 1u32]);
    let tile48 = e.mem.u32(base + 0x48);
    e.call(TILE_SET_INT, &args![tile48, 0xfa3u32, 0u32]);
    let broken = broken_item(e);
    let broken_form = e.call(ITEM_FORM, &args![broken]).u32();
    if e.call(FORM_TYPE, &args![broken_form]).u32() == 0x28 {
        let text = e.call(SETTING_STRING, &args![SETTING_011D2C7C]).u32();
        let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
        let tile50 = e.mem.u32(base + 0x50);
        e.call(TILE_SET_STRING, &args![tile50, title, text, 1u32]);
        let broken = broken_item(e);
        let form = e.call(ITEM_FORM, &args![broken]).u32();
        let damage = weapon_damage(e, form, condition, broken);
        let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        let tile50 = e.mem.u32(base + 0x50);
        e.call(TILE_SET_INT, &args![tile50, value_trait, damage]);
    } else {
        let form = e.call(ITEM_FORM, &args![current]).u32();
        if e.call(FORM_TYPE, &args![form]).u32() == 0x18 {
            let broken = broken_item(e);
            let form = e.call(ITEM_FORM, &args![broken]).u32();
            let broken = broken_item(e);
            let health = e.call(GET_ITEM_HEALTH, &args![broken, 1u32]).f64();
            let hundred: f64 = e.global(HUNDRED_DOUBLE);
            let now = x87_store32(x87_div(health, hundred));
            let (now_base, now_damaged) = armor_ratings(e, form, now);
            let (then_base, then_damaged) = armor_ratings(e, form, condition);
            e.with_stack(0x80, |e, buffer| {
                e.call(COPY, &args![buffer, 0x80u32, DASHES_TEXT]);
                show_armor_change(e, buffer, [now_base, now_damaged, then_base, then_damaged]);
            });
        }
    }
    let tile = e.mem.u32(base + 0x4c);
    let after = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f64();
    let tile = e.mem.u32(base + 0x38);
    let before = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let condition_gain = x87_store32(x87_mul(x87_sub(after, before), hundred));
    let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = e.mem.u32(base + 0x50);
    let after = e.call(GET_FLOAT_TRAIT, &args![tile, value_trait]).f64();
    let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = e.mem.u32(base + 0x3c);
    let before = e.call(GET_FLOAT_TRAIT, &args![tile, value_trait]).f64();
    let value_gain = x87_store32(x87_sub(after, before));
    show_gains(e, this, condition_gain, value_gain, [0x40, 0x44]);
}

/// The condition and value gains (each not below `0.0`, `00404010`), rounded (`+ 0.5`,
/// `_ftol2`) and formatted with `010734b0` (the first with `"%"` appended), shown in the
/// two tiles of `this`.
fn show_gains(e: &mut Engine, this: Ptr, condition_gain: u32, value_gain: u32, tiles: [u32; 2]) {
    let condition_gain = e
        .call(
            FLOAT_PAIR_00404010,
            &args![0.0f32.to_bits(), x87_quiet_bits(condition_gain)],
        )
        .f64();
    let condition_gain = x87_store32(condition_gain);
    let value_gain = e
        .call(
            FLOAT_PAIR_00404010,
            &args![0.0f32.to_bits(), x87_quiet_bits(value_gain)],
        )
        .f64();
    let value_gain = x87_store32(value_gain);
    let half: f64 = e.global(HALF_DOUBLE);
    e.with_stack(0x40, |e, buffer| {
        let rounded = x87_add(x87_load32(condition_gain), half);
        let whole = e.call(FTOL2, &args![rounded]).u32();
        e.call(
            FORMAT,
            &args![buffer, 0x40u32, PERCENT_FORMAT_010734B0, whole],
        );
        e.call(APPEND, &args![buffer, 0x40u32, PERCENT_TEXT]);
        let tile = e.mem.u32(this.addr() + tiles[0]);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
        let rounded = x87_add(x87_load32(value_gain), half);
        let whole = e.call(FTOL2, &args![rounded]).u32();
        e.call(
            FORMAT,
            &args![buffer, 0x40u32, PERCENT_FORMAT_010734B0, whole],
        );
        let tile = e.mem.u32(this.addr() + tiles[1]);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, buffer, 1u32]);
    });
}

// Translated from 007b6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::ShowErrorMessage` (Xbox PDB; `RET 4`: the text): shows it in the tile at
/// +0x48 (and that tile, hiding the one at +0x4c) and, unless it has one, gives it a fade
/// control on its alpha (`0xfa9`: 255, 64, 2.0, 1).
pub fn repair_menu_show_error_message(e: &mut Engine, this: Ptr, text: u32) {
    let base = this.addr();
    let tile = e.mem.u32(base + 0x48);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    let tile = e.mem.u32(base + 0x4c);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    let tile = e.mem.u32(base + 0x48);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    let tile = e.mem.u32(base + 0x48);
    if !e.call(TILE_HAS_FADE_CONTROL, &args![tile, 0xfa9u32]).bool() {
        let full = x87_quiet_bits(e.global::<u32>(FULL_ALPHA_FLOAT));
        let low = x87_quiet_bits(e.global::<u32>(SIXTY_FOUR_FLOAT));
        let period = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
        let tile = e.mem.u32(base + 0x48);
        e.call(
            TILE_ADD_FADE_CONTROL,
            &args![tile, 0xfa9u32, full, low, period, 1u32],
        );
    }
}

// Translated from 007b6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::DoLeave` (Xbox PDB; `RET 8`: id, tile): id 0xe hides the tile at +0x4c and,
/// when the list has more than one item, the one at +0x48.
pub fn repair_menu_do_leave(e: &mut Engine, this: Ptr, id: i32, _tile: u32) {
    if id != 0xe {
        return;
    }
    let base = this.addr();
    let tile = e.mem.u32(base + 0x4c);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    if e.call(LIST_BOX_NUM_UNFILTERED, &args![base + REPAIR_LIST])
        .u32()
        > 1
    {
        let tile = e.mem.u32(base + 0x48);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    }
}

// Translated from 007b6a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::DoGamepadUpEvent` (Xbox PDB; `RET 8`: a button code and an unused word):
/// codes 6 and 10 play menu sound 3 and click (vtable +0xc) id 0xc. Returns false (the code
/// never sets its result).
pub fn repair_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr,
    code: u32,
    _unused_2: u32,
) -> bool {
    if code == 6 || code == 0xa {
        e.call(INTERFACE_PLAY_MENU_SOUND, &args![3u32]);
        e.vcall(this.addr(), 0xc, &args![0xcu32, 0u32]);
    }
    false
}

// Translated from 007b6aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b6aa0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the local list and the item allocation) is not translated.
/// `RepairMenu::Update` (Xbox PDB, no arguments): with a broken item, takes the player's
/// items that can repair it (`GetRepairItemGroup` into a local list) and, for each item and
/// each of its count (the word at +4), skips undamaged copies (rounded health at or above
/// 100) of the broken item's own form; the first copy goes into the list (`AddItem` with
/// its name) and, when it is the broken item itself (same form and first extra list),
/// becomes `pBrokenItem`, gets trait `011da748` and makes the tile at +0x30's height and
/// position values follow it; a copy of the broken item's form that has weapon mods or is
/// worn is removed again. Further copies get an `ItemChange` of their own (the form, count
/// 1, the extra lists) in the list. With the `"ENGLISH"` language setting, also sets the
/// menu's titles.
pub fn repair_menu_update(e: &mut Engine) {
    if broken_item(e) == 0 {
        return;
    }
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCT, &args![list]);
        let player = e.global::<u32>(PLAYER);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
        let broken = broken_item(e);
        let form = e.call(ITEM_FORM, &args![broken]).u32();
        e.call(GET_REPAIR_ITEM_GROUP, &args![changes, form, list]);
        let mut node = list.addr();
        while node != 0 && node_item(e, node) != 0 {
            update_entry(e, node);
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        let language = e.call(SETTING_STRING, &args![SETTING_011C3CB4]).u32();
        if e.call(STRINGS_00408B20, &args![language, ENGLISH_TEXT])
            .u32()
            != 0
        {
            let text = e.call(SETTING_STRING, &args![SETTING_011D4378]).u32();
            let root = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
            let headline = e
                .call(TILE_GET_CHILD_BY_NAME, &args![root, HEADLINE_TITLE_TEXT])
                .u32();
            e.call(TILE_SET_STRING, &args![headline, 0xfc4u32, text, 1u32]);
            for (offset, setting) in [
                (0x28u32, SETTING_011D4030),
                (0x38, SETTING_011D4DA4),
                (0x4c, SETTING_011D4DA4),
            ] {
                let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
                set_setting_text(e, offset, title, setting);
            }
            set_setting_text(e, 0x54, 0xfc4, SETTING_011D1F2C);
            set_setting_text(e, 0x58, 0xfc4, SETTING_011D29F4);
        }
        e.call(LIST_DESTRUCT, &args![list]);
    });
}

/// `RepairMenu::Update` for one entry of the repair item group.
fn update_entry(e: &mut Engine, node: u32) {
    let mut index = 0i32;
    loop {
        let entry = node_item(e, node);
        let count = e.call(WORD_AT_FOUR, &args![entry]).i32();
        if index >= count {
            return;
        }
        let entry = node_item(e, node);
        let health = x87_store32(e.call(GET_ITEM_HEALTH, &args![entry, 1u32]).f64());
        let rounded = e.call(ROUND_00476B20, &args![x87_quiet_bits(health)]).f64();
        let rounded = x87_store32(rounded);
        let extra = first_extra_list_of_node(e, node);
        let has_mods = extra != 0 && e.call(EXTRA_HAS_WEAPON_MODS, &args![extra]).bool();
        // `FCOMP double 100.0; TEST AH,5; JNP`: on when below `100.0`; at or above (or NaN)
        // the broken item's own form is skipped.
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        let skip = !(x87_load32(rounded) < hundred) && {
            let entry = node_item(e, node);
            let form = e.call(ITEM_FORM, &args![entry]).u32();
            let broken = broken_item(e);
            form == e.call(ITEM_FORM, &args![broken]).u32()
        };
        if !skip {
            if index == 0 {
                let item = node_item(e, node);
                let name = e.call(GET_FULL_NAME, &args![item]).u32();
                let list = me(e) + REPAIR_LIST;
                let tile = e
                    .call(LIST_BOX_ADD_ITEM, &args![list, item, name, 0u32, 0u32])
                    .u32();
                if tile != 0 {
                    let form = e.call(ITEM_FORM, &args![item]).u32();
                    let broken = broken_item(e);
                    if form == e.call(ITEM_FORM, &args![broken]).u32() {
                        let own = first_extra_list(e, item);
                        let broken = broken_item(e);
                        let broken_extra = first_extra_list(e, broken);
                        if own == broken_extra {
                            e.set_global(BROKEN_ITEM, item);
                            let trait_id = e.global::<u32>(TRAIT_FROM_NAME_01075C44);
                            e.call(TILE_SET_INT, &args![tile, trait_id, 1u32]);
                            let holder = menu_tile(e, 0x30);
                            let height = e.call(TILE_GET_VALUE_Q, &args![holder, 0xfb0u32]).u32();
                            let holder = menu_tile(e, 0x30);
                            let position = e.call(TILE_GET_VALUE_Q, &args![holder, 0xfa2u32]).u32();
                            e.call(VALUE_CLEAR_ACTIONS, &args![height]);
                            e.call(
                                VALUE_ADD_ACTION_TILE,
                                &args![height, 0x7d0u32, tile, 0xfb0u32],
                            );
                            e.call(
                                VALUE_ADD_ACTION_FLOAT,
                                &args![height, 0x7d2u32, 1.0f32.to_bits()],
                            );
                            e.call(VALUE_CALCULATE, &args![height, 0u32]);
                            e.call(VALUE_CLEAR_ACTIONS, &args![position]);
                            e.call(
                                VALUE_ADD_ACTION_TILE,
                                &args![position, 0x7d0u32, tile, 0xfa2u32],
                            );
                            e.call(VALUE_CALCULATE, &args![position, 0u32]);
                        } else if has_mods || e.call(GET_WORN, &args![item, 0u32]).bool() {
                            let list = me(e) + REPAIR_LIST;
                            e.call(LIST_BOX_REMOVE_ITEM, &args![list, tile]);
                        }
                    }
                }
            } else {
                let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                let item = if block != 0 {
                    let entry = node_item(e, node);
                    let form = e.call(ITEM_FORM, &args![entry]).u32();
                    e.call(ITEM_CHANGE_CONSTRUCT, &args![block, form, 1u32])
                        .u32()
                } else {
                    0
                };
                let entry = node_item(e, node);
                let mut extra_node = e.call(WORD_AT_ZERO, &args![entry]).u32();
                while extra_node != 0 && node_item(e, extra_node) != 0 {
                    let slot = e.call(NODE_SELF, &args![extra_node]).u32();
                    let lists = e.call(WORD_AT_ZERO, &args![item]).u32();
                    e.call(LIST_APPEND_005AE3D0, &args![lists, slot]);
                    extra_node = e.call(WORD_AT_FOUR, &args![extra_node]).u32();
                }
                let name = e.call(GET_FULL_NAME, &args![item]).u32();
                let list = me(e) + REPAIR_LIST;
                e.call(LIST_BOX_ADD_ITEM, &args![list, item, name, 0u32, 0u32]);
            }
        }
        index += 1;
    }
}

// Translated from 007b6fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairMenu::ResetMenu` (Xbox PDB, no arguments): empties the list (vtable +0x1c),
/// clears `pBrokenItem`, moves the focus to tile 0xe (vtable +0x14), removes the fade
/// control of the tile at +0x48 and hides it.
pub fn repair_menu_reset_menu(e: &mut Engine) {
    let list = me(e) + REPAIR_LIST;
    e.vcall(list, 0x1c, &args![]);
    e.set_global(BROKEN_ITEM, 0u32);
    let menu = me(e);
    e.vcall(menu, 0x14, &args![0xeu32, 0u32]);
    let tile = menu_tile(e, 0x48);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![tile, TRAIT_0XFA9]);
    let tile = menu_tile(e, 0x48);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
}

/// `fn_007b5720`'s, `RepairMenu::InitFunc`'s and `fn_007b5950`'s addresses, as the code
/// passes them.
const INIT_FUNC_007B57F0: u32 = 0x007b_57f0;
const COMPARE_007B5950: u32 = 0x007b_5950;

// Translated from 007b7020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b7020 - the /GS security cookie check (00ec408c) is not translated
/// `RepairMenu::InitRepairMenu` (Xbox PDB, `cdecl`: an item, may be 0). With an open menu:
/// takes the item as `pBrokenItem` (when given), shows its condition (`0x1004` of the tile
/// at +0x38), the weapon's damage or the armour's ratings (else `"--"`) in the tile at
/// +0x3c, and with an item its icon (`0xfcc` of +0x34) and `Update`; runs `InitFunc` over
/// the list, sorts it with `fn_007b5950`, shows the player's repair skill (+0x28), focuses
/// the list's second tile (with an item and `004b71d0`), enters (vtable +0x10, id 0xe) the
/// highlighted tile and, with an item, `007044c0(1, 0, 0)` and the tutorial message 6.
pub fn repair_menu_init_repair_menu(e: &mut Engine, item: u32) {
    if me(e) == 0 {
        return;
    }
    if item != 0 {
        e.set_global(BROKEN_ITEM, item);
    }
    let broken = broken_item(e);
    let health = e.call(GET_ITEM_HEALTH, &args![broken, 1u32]).f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let fraction = x87_store32(x87_div(health, hundred));
    let tile = menu_tile(e, 0x38);
    e.call(
        SET_FLOAT_TRAIT,
        &args![tile, 0x1004u32, x87_quiet_bits(fraction), 1u32],
    );
    let tile = menu_tile(e, 0x38);
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
    let broken = broken_item(e);
    let form = e.call(ITEM_FORM, &args![broken]).u32();
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    if kind == 0x28 {
        let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
        set_setting_text(e, 0x3c, title, SETTING_011D2C7C);
        let damage = weapon_damage(e, form, fraction, item);
        let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
        let tile = menu_tile(e, 0x3c);
        e.call(TILE_SET_INT, &args![tile, value_trait, damage]);
    } else if kind == 0x18 {
        let (base, damaged) = armor_ratings(e, form, fraction);
        e.with_stack(0x80, |e, buffer| {
            show_armor_ratings(e, base, damaged, buffer);
        });
    } else {
        e.with_stack(0x80, |e, buffer| {
            e.call(COPY, &args![buffer, 0x80u32, DASHES_TEXT]);
            let title = e.global::<u32>(TRAIT_FROM_NAME_0106ED20);
            set_setting_text(e, 0x3c, title, SETTING_011D20C4);
            let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
            let tile = menu_tile(e, 0x3c);
            e.call(TILE_SET_STRING, &args![tile, value_trait, buffer, 1u32]);
        });
    }
    if item != 0 {
        let broken = broken_item(e);
        let icon = e.call(GET_ICON_ONE, &args![broken, 0u32]).u32();
        let tile = menu_tile(e, 0x34);
        e.call(TILE_SET_STRING, &args![tile, 0xfccu32, icon, 1u32]);
        repair_menu_update(e);
    }
    let list = me(e) + REPAIR_LIST;
    e.call(
        LIST_BOX_FOR_EACH,
        &args![list, INIT_FUNC_007B57F0, 0u32, 0x7fff_ffffu32],
    );
    let list = me(e) + REPAIR_LIST;
    e.vcall(list, 0x20, &args![COMPARE_007B5950]);
    let player = e.global::<u32>(PLAYER);
    let skill = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![player + 0xa4, 0x27u32])
        .u32();
    let value_trait = e.global::<u32>(TRAIT_FROM_NAME_0106ED18);
    let tile = menu_tile(e, 0x28);
    e.call(TILE_SET_INT, &args![tile, value_trait, skill]);
    if item != 0 && e.call(FLAG_004B71D0, &args![]).bool() {
        let list = me(e) + REPAIR_LIST;
        let target = e.vcall(list, 0x14, &args![1u32, 0u32]).u32();
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SET_CURRENT_FOCUS_TARGET,
            &args![interface, target, 0xfc3u32, 1u32],
        );
    }
    let list = me(e) + REPAIR_LIST;
    let highlighted = e.vcall(list, 4, &args![]).u32();
    let menu = me(e);
    e.vcall(menu, 0x10, &args![0xeu32, highlighted]);
    if item != 0 {
        e.call(INTERFACE_007044C0, &args![1u32, 0u32, 0u32]);
        let class = fn_007b5720(e);
        let manager = e.call(INTERFACE_SINGLETON, &args![]).u32() + 0x4d4;
        e.call(
            TUTORIAL_SHOW_MESSAGE,
            &args![manager, 6u32, class, 0x1f4u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_5400, repair_menu_repair_menu(Ptr) -> Ptr),
        entry!(0x007b_54b0, repair_menu_get_class(Ptr) -> u32),
        entry!(0x007b_54c0, repair_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x007b_54f0, repair_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x007b_5520, repair_menu_create() -> u32),
        entry!(0x007b_5720, fn_007b5720() -> u32),
        entry!(0x007b_5730, fn_007b5730(Ptr) -> bool),
        entry!(0x007b_5770, fn_007b5770(Ptr)),
        entry!(0x007b_57f0, repair_menu_init_func(u32, u32)),
        entry!(0x007b_5950, fn_007b5950(u32, u32) -> i32),
        entry!(0x007b_5b40, repair_menu_do_click(Ptr, i32, u32)),
        entry!(0x007b_5d80, repair_menu_repair_item(Ptr, u32)),
        entry!(0x007b_6120, repair_menu_do_enter(Ptr, i32, u32)),
        entry!(0x007b_6970, repair_menu_show_error_message(Ptr, u32)),
        entry!(0x007b_6a00, repair_menu_do_leave(Ptr, i32, u32)),
        entry!(0x007b_6a50, repair_menu_do_gamepad_up_event(Ptr, u32, u32) -> bool),
        entry!(0x007b_6aa0, repair_menu_update()),
        entry!(0x007b_6fb0, repair_menu_reset_menu()),
        entry!(0x007b_7020, repair_menu_init_repair_menu(u32)),
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
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        stub(&mut e, &[MENU_CONSTRUCTOR, LIST_BOX_CONSTRUCTOR]);
        e.register(TILE_TEXT_TO_TRAIT, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_5400, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), REPAIR_MENU_VTABLE);
        assert_eq!(calls_to(&e, LIST_BOX_CONSTRUCTOR), vec![vec![this + 0x5c]]);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED20), 0xed20);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_0106ED18), 0xed18);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01070244), 0x0244);
        assert_eq!(e.global::<u32>(TRAIT_FROM_NAME_01075C44), 0x5c44);
    }

    #[test]
    fn class_number_and_tiles() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x007b_54b0, &args![0u32]).u32(), 0x40b);
        assert_eq!(e.call(0x007b_5720, &args![]).u32(), 0x40b);
        let this = e.mem.alloc(0x8c);
        e.call(0x007b_54c0, &args![this, 12i32, 0x99u32]);
        e.call(0x007b_54c0, &args![this, 13i32, 0x98u32]);
        assert_eq!(e.mem.u32(this + 0x28 + 48), 0x99);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert!(!e.call(0x007b_5730, &args![this]).bool());
        for i in 0..13 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x007b_5730, &args![this]).bool());
    }

    #[test]
    fn deleting_destructor_tears_down_and_frees() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x8c);
        e.set_global(REPAIR_MENU, this);
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
        assert_eq!(e.call(0x007b_54f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.global::<u32>(REPAIR_MENU), 0);
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x480, 0xfa9]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_DESTRUCTOR), vec![vec![this + 0x5c]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x007b_5770, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_sets_up_a_complete_menu() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.set_global(STACKING_TYPE_ONE, 102.0f64);
        e.set_global(TRAIT_FROM_NAME_0106ED18, 0x77u32);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x007b_54b0);
        for i in 0..13 {
            e.mem.set_u32(this + 0x28 + 4 * i, 0x100 + 4 * i);
        }
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this);
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
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0x4444);
        assert_eq!(e.global::<u32>(REPAIR_MENU), this);
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
            vec![vec![this + 0x5c, 0x104, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(calls_to(&e, LIST_BOX_WITH_FLAG), vec![vec![this + 0x5c, 0]]);
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
        e.map(REPAIR_MENU & !0xfff, 0x1000);
        let old = e.mem.alloc(8);
        let old_vtable = e.mem.alloc(8);
        e.mem.set_u32(old, old_vtable);
        e.mem.set_u32(old_vtable, 0x00c0_0000);
        let this = e.mem.alloc(0x8c);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x34, 0x007b_54b0);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, old);
        answer(&mut e, TILE_GET_MENU, this);
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
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![old, 1]]);
        assert_eq!(calls_to(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x40b]]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut e = Engine::new();
        e.map(REPAIR_MENU & !0xfff, 0x1000);
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
        assert_eq!(e.call(0x007b_5520, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![this, 1]]);
    }
}

#[cfg(test)]
mod n057_tests {
    use super::*;

    /// Every callee outside this file the translations above reach.
    const EXTERNAL: &[u32] = &[
        0x00401000, 0x00401170, 0x00403df0, 0x00403e20, 0x00404010, 0x00406d00, 0x00406d30,
        0x00406d50, 0x00408b20, 0x0040ebd0, 0x00410360, 0x00412380, 0x004126c0, 0x00418770,
        0x00418ba0, 0x00419970, 0x00419ad0, 0x00419f80, 0x0044ddc0, 0x00453a70, 0x0046ffb0,
        0x00476b20, 0x00483710, 0x004873d0, 0x004b71d0, 0x004b7210, 0x004bc550, 0x004bcdb0,
        0x004bcf00, 0x004bd510, 0x004bda70, 0x004bddd0, 0x004be060, 0x004be180, 0x004be200,
        0x004be2d0, 0x004bf220, 0x004bfba0, 0x004d4bd0, 0x004d5c60, 0x00559450, 0x005ae3d0,
        0x006317a0, 0x0063f7b0, 0x006450f0, 0x00646360, 0x00648090, 0x0066ef20, 0x006815c0,
        0x00700320, 0x00704370, 0x007044c0, 0x007048f0, 0x00706f30, 0x00715860, 0x00717280,
        0x00718630, 0x0071a320, 0x0071a5c0, 0x0071ae60, 0x00726070, 0x007314c0, 0x00782a90,
        0x007a1910, 0x007b5370, 0x007f8610, 0x00905330, 0x0096a2d0, 0x00a00e90, 0x00a011b0,
        0x00a01230, 0x00a012d0, 0x00a01350, 0x00a01860, 0x00a03da0, 0x00a03eb0, 0x00a07c60,
        0x00a07dc0, 0x00a07ed0, 0x00a09080, 0x00a09130, 0x00a09200, 0x00a09410, 0x00ad7550,
        0x00ad8830, 0x00ec43fb, 0x00ec62c0,
    ];

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| eax.into_ret());
    }

    fn st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_FOUR, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        for page in [
            0x011d_a000,
            0x011d_e000,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_b000,
            0x0101_e000,
            0x0102_0000,
            0x0102_3000,
            0x0102_9000,
            0x0102_f000,
            0x0107_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(TWO_FIFTY_FIVE_DOUBLE, 255.0f64);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        e.set_global(NINETY_NINE_DOUBLE, 99.0f64);
        e.set_global(SIX_TENTHS_DOUBLE, 0.6f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(TEN_DOUBLE, 10.0f64);
        e.set_global(SEVENTY_FIVE_FLOAT, 75.0f32);
        e.set_global(FIFTY_FLOAT, 50.0f32);
        e.set_global(SIXTY_FOUR_FLOAT, 64.0f32);
        e.set_global(TWO_FLOAT, 2.0f32);
        e
    }

    /// A repair menu (thirteen tiles, a vtable of doubles, the list's vtable too) as `pMe`.
    fn menu(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0x8c);
        e.put_vtable(0x0e00_0000, &[0x0000_8000; 0x100]);
        e.register(0x0000_8000, |_, _| Ret::default());
        e.mem.set_u32(menu, 0x0e00_0000);
        e.mem.set_u32(menu + REPAIR_LIST, 0x0e00_0000);
        for i in 0..13 {
            e.mem.set_u32(menu + 0x28 + 4 * i, 0x2000 + 4 * i);
        }
        e.set_global(REPAIR_MENU, menu);
        menu
    }

    /// An `ItemChange` (extra lists, count, form) with one extra data list.
    fn item_change(e: &mut Engine, form: u32, extra: u32) -> u32 {
        let lists = e.mem.alloc(8);
        e.mem.set_u32(lists, extra);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, lists);
        e.mem.set_u32(item + 4, 1);
        e.mem.set_u32(item + 8, form);
        item
    }

    #[test]
    fn init_func_shows_the_repaired_condition() {
        let mut e = engine();
        e.call(0x007b_57f0, &args![0u32, 1u32]);
        st0(&mut e, GET_ITEM_HEALTH, 50.0);
        st0(&mut e, GET_FLOAT_TRAIT, 2.0);
        answer(&mut e, TILE_GET_CHILD_BY_ID, 0x2400);
        st0(&mut e, GET_REPAIRED_CONDITION, 80.0);
        e.register(FLOAT_PAIR_0040EBD0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        answer(&mut e, GET_WORN, 1);
        let tile = e.mem.alloc(0x10);
        e.mem.set_u32(tile + 4, 3);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2500);
        answer(&mut e, LIST_ELEMENT_006317A0, slot);
        e.call_log = Some(vec![]);
        e.call(0x007b_57f0, &args![tile, 0x7000u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![tile, 0xfaa, 0xe],
                vec![tile, 0xfa3, 1],
                vec![0x2500, 0xfa3, 1]
            ]
        );
        assert_eq!(calls_to(&e, LIST_ELEMENT_006317A0), vec![vec![tile + 4, 3]]);
        let repaired = calls_to(&e, GET_REPAIRED_CONDITION);
        assert_eq!(repaired[0][1..3], [50.0f32.to_bits(), 50.0f32.to_bits()]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![
                vec![0x2400, 0xfb1, 30.0f32.to_bits(), 1],
                vec![tile, 0x1004, 0.8f32.to_bits(), 1]
            ]
        );
    }

    #[test]
    fn repair_list_order() {
        let mut e = engine();
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] == 0x2100).into_ret());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2900);
        answer(&mut e, ITEM_TILE_007B5370, slot);
        st0(&mut e, GET_FLOAT_TRAIT, 255.0);
        e.register(GET_ITEM_HEALTH, |_, a| Ret {
            st0: f64::from(a[0] & 0xff),
            ..Ret::default()
        });
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x2100);
        e.mem.set_u32(second, 0x2200);
        e.mem.set_u32(first + 4, 0x7010);
        e.mem.set_u32(second + 4, 0x7020);
        assert_eq!(e.call(0x007b_5950, &args![first, second]).i32(), -1);
        assert_eq!(e.call(0x007b_5950, &args![second, first]).i32(), 1);
        e.mem.set_u32(first, 0x2300);
        assert_eq!(e.call(0x007b_5950, &args![first, second]).i32(), 1);
        st0(&mut e, GET_FLOAT_TRAIT, 100.0);
        assert_eq!(e.call(0x007b_5950, &args![first, second]).i32(), -1);
        e.mem.set_u32(second + 4, 0x7010);
        answer(&mut e, GET_WORN, 1);
        assert_eq!(e.call(0x007b_5950, &args![first, second]).i32(), -1);
    }

    #[test]
    fn do_click_closes_or_repairs() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007b_5b40, &args![menu, 0xci32, 0u32]);
        assert_eq!(
            calls_to(&e, INTERFACE_MANAGER_PLAY_MENU_SOUND),
            vec![vec![0x24]]
        );
        assert_eq!(
            calls_to(&e, SET_INVENTORY_MENU_VISIBLE),
            vec![vec![1, 0, 0]]
        );
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2900);
        answer(&mut e, ITEM_TILE_007B5370, slot);
        st0(&mut e, GET_FLOAT_TRAIT, 128.0);
        e.call_log = Some(vec![]);
        e.call(0x007b_5b40, &args![menu, 0xei32, 0u32]);
        assert_eq!(calls_to(&e, ITEM_TILE_007B5370), vec![vec![4]]);
        assert!(calls_to(&e, LIST_BOX_CURRENT_ITEM).is_empty());
        st0(&mut e, GET_FLOAT_TRAIT, 255.0);
        answer(&mut e, TILE_IS_TRUE, 1);
        let broken = item_change(&mut e, 0x3300, 0);
        e.set_global(BROKEN_ITEM, broken);
        let used = item_change(&mut e, 0x3300, 0x4400);
        let entry = item_change(&mut e, 0x3300, 0);
        answer(&mut e, GET_OBJECT_IN_LIST, entry);
        answer(&mut e, LIST_BOX_CURRENT_ITEM, used);
        answer(&mut e, ITEM_FORM, 0x3300);
        answer(&mut e, LIST_BOX_NUM_UNFILTERED, 1);
        let player = e.mem.alloc(4);
        e.mem.set_u32(player, 0x0e00_0000);
        e.set_global(PLAYER, player);
        e.call_log = Some(vec![]);
        e.call(0x007b_5b40, &args![menu, 0xei32, 0x2200u32]);
        let player_calls: Vec<_> = calls_to(&e, 0x0000_8000)
            .into_iter()
            .filter(|c| c[0] == player)
            .collect();
        assert_eq!(
            player_calls,
            vec![vec![player, 0x3300, 0x4400, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2],
            REPAIR_SOUND_TEXT
        );
        assert!(calls_to(&e, 0x0000_8000).contains(&vec![menu, 0xc, 0]));
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x22]]);
    }

    #[test]
    fn repair_item_stacks_or_keeps_a_new_list() {
        let mut e = engine();
        menu(&mut e);
        st0(&mut e, GET_ITEM_HEALTH, 40.0);
        st0(&mut e, GET_REPAIRED_CONDITION, 50.0);
        e.register(FLOAT_PAIR_0040EBD0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        answer(&mut e, HEALTH_FORM_GET_FORM_HEALTH, 200);
        let broken = item_change(&mut e, 0x3300, 0x4400);
        e.set_global(BROKEN_ITEM, broken);
        let entry = item_change(&mut e, 0x3300, 0x4500);
        answer(&mut e, GET_OBJECT_IN_LIST, entry);
        answer(&mut e, ALLOCATE, 0x4600);
        answer(&mut e, EXTRA_LIST_CONSTRUCT, 0x4700);
        answer(&mut e, EXTRA_GET_COUNT, 3);
        e.map(0x4000, 0x1000);
        e.mem.set_u32(0x4700, 0x0e00_0000);
        e.call_log = Some(vec![]);
        e.call(0x007b_5d80, &args![0u32, 0x7000u32]);
        assert_eq!(
            calls_to(&e, EXTRA_SET_HEALTH),
            vec![vec![0x4700, 100.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, EXTRA_SET_COUNT),
            vec![vec![0x4700, 1], vec![0x4400, 2], vec![0x4500, 4]]
        );
        assert_eq!(calls_to(&e, LIST_APPEND_005AE3D0).len(), 1);
        assert_eq!(calls_to(&e, EXTRA_00419F80), vec![vec![0x4700, 0]]);
    }

    #[test]
    fn do_enter_moves_the_knob_and_reports_costs() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.call(0x007b_6120, &args![menu, 3i32, 0u32]);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 0.5f32.to_bits());
        answer(&mut e, SETTING_VALUE, setting);
        answer(&mut e, LIST_BOX_CURRENT_ITEM, 0x7100);
        answer(&mut e, 0x0000_8000, 0x2600);
        st0(&mut e, GET_FLOAT_TRAIT, 4.0);
        answer(&mut e, PIPBOY_MANAGER_00704370, 0x9900);
        e.call_log = Some(vec![]);
        e.call(0x007b_6120, &args![menu, 0xei32, 0x2200u32]);
        assert_eq!(
            calls_to(&e, PIPBOY_MOVE_KNOB),
            vec![vec![0x9900, 0, 0.5f32.to_bits(), 0.5f32.to_bits(), 1]]
        );
        assert_eq!(e.global::<u32>(KNOB_INDEX_011DA7D8), 4);
        let broken = item_change(&mut e, 0x3300, 0);
        e.set_global(BROKEN_ITEM, broken);
        st0(&mut e, GET_ITEM_HEALTH, 80.0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x61);
        e.register(GET_FLOAT_TRAIT, |_, a| Ret {
            st0: if a[1] == 0x1005 { 30.0 } else { 4.0 },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007b_6120, &args![menu, 0xei32, 0x2200u32]);
        assert!(calls_to(&e, PIPBOY_MOVE_KNOB).is_empty());
        assert_eq!(calls_to(&e, FORMAT)[0][3..], [30, PERCENT_TEXT]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT).last().unwrap(),
            &vec![0x2000 + 0x2c, 0x61, 128.0f32.to_bits(), 1]
        );
        assert!(calls_to(&e, TILE_SET_STRING)
            .iter()
            .any(|c| c[0] == 0x2000 + 0x2c && c[1] == 0xfc4));
    }

    #[test]
    fn do_enter_shows_the_gains() {
        let mut e = engine();
        let menu = menu(&mut e);
        let broken = item_change(&mut e, 0x3300, 0);
        e.set_global(BROKEN_ITEM, broken);
        e.set_global(KNOB_INDEX_011DA7D8, 0u32);
        answer(&mut e, LIST_BOX_CURRENT_ITEM, 0x7100);
        st0(&mut e, GET_ITEM_HEALTH, 80.0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x61);
        answer(&mut e, FORM_TYPE, 0x1f);
        e.register(GET_FLOAT_TRAIT, |_, a| Ret {
            st0: match (a[0], a[1]) {
                (_, 0x1005) => 0.0,
                (0x2024, _) => 0.75,
                (0x2010, _) => 0.5,
                _ => 0.0,
            },
            ..Ret::default()
        });
        e.register(FLOAT_PAIR_00404010, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007b_6120, &args![menu, 0xei32, 0x2200u32]);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT)[0],
            vec![0x2000 + 0x2c, 0x61, 255.0f32.to_bits(), 1]
        );
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats[0][2..], [PERCENT_FORMAT_010734B0, 25]);
        assert_eq!(formats[1][2..], [PERCENT_FORMAT_010734B0, 0]);
        assert_eq!(calls_to(&e, APPEND).len(), 1);
    }

    #[test]
    fn error_message_leave_and_gamepad() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007b_6970, &args![menu, 0x6100u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_STRING),
            vec![vec![0x2020, 0xfc4, 0x6100, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_ADD_FADE_CONTROL),
            vec![vec![
                0x2020,
                0xfa9,
                255.0f32.to_bits(),
                64.0f32.to_bits(),
                2.0f32.to_bits(),
                1
            ]]
        );
        answer(&mut e, LIST_BOX_NUM_UNFILTERED, 2);
        e.call_log = Some(vec![]);
        e.call(0x007b_6a00, &args![menu, 0xei32, 0u32]);
        e.call(0x007b_6a00, &args![menu, 0xdi32, 0u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![vec![0x2024, 0xfa3, 0], vec![0x2020, 0xfa3, 0]]
        );
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007b_6a50, &args![menu, 6u32, 0u32]).bool());
        assert!(!e.call(0x007b_6a50, &args![menu, 7u32, 0u32]).bool());
        assert_eq!(calls_to(&e, INTERFACE_PLAY_MENU_SOUND), vec![vec![3]]);
        assert_eq!(calls_to(&e, 0x0000_8000), vec![vec![menu, 0xc, 0]]);
    }

    #[test]
    fn update_lists_the_repair_items() {
        let mut e = engine();
        e.call(0x007b_6aa0, &args![]);
        let menu = menu(&mut e);
        let broken = item_change(&mut e, 0x3300, 0x4400);
        e.set_global(BROKEN_ITEM, broken);
        let entry = item_change(&mut e, 0x3300, 0x4400);
        e.mem.set_u32(entry + 4, 2);
        // The group list gets the entry.
        e.register_double(GET_REPAIR_ITEM_GROUP, move |e, a| {
            e.mem.set_u32(a[2], entry);
            Ret::default()
        });
        e.register(ITEM_FORM, |e, a| e.mem.u32(a[0] + 8).into_ret());
        st0(&mut e, ROUND_00476B20, 50.0);
        answer(&mut e, LIST_BOX_ADD_ITEM, 0x2700);
        answer(&mut e, ALLOCATE, 0x4800);
        e.register(ITEM_CHANGE_CONSTRUCT, |e, a| {
            let lists = e.mem.alloc(8);
            e.mem.set_u32(a[0], lists);
            a[0].into_ret()
        });
        e.map(0x4000, 0x1000);
        e.set_global(TRAIT_FROM_NAME_01075C44, 0x5au32);
        answer(&mut e, TILE_GET_VALUE_Q, 0x9100);
        e.call_log = Some(vec![]);
        e.call(0x007b_6aa0, &args![]);
        assert_eq!(e.global::<u32>(BROKEN_ITEM), entry);
        let added = calls_to(&e, LIST_BOX_ADD_ITEM);
        assert_eq!(added.len(), 2);
        assert_eq!(added[0][..2], [menu + REPAIR_LIST, entry]);
        assert_eq!(added[1][..2], [menu + REPAIR_LIST, 0x4800]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x2700, 0x5a, 1]]);
        assert_eq!(
            calls_to(&e, VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x9100, 0x7d2, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn reset_and_init_repair_menu() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.set_global(BROKEN_ITEM, 0x7100u32);
        e.call_log = Some(vec![]);
        e.call(0x007b_6fb0, &args![]);
        assert_eq!(e.global::<u32>(BROKEN_ITEM), 0);
        assert_eq!(
            calls_to(&e, 0x0000_8000),
            vec![vec![menu + REPAIR_LIST], vec![menu, 0xe, 0]]
        );
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![0x2020, 0xfa9]]
        );
        let broken = item_change(&mut e, 0x3300, 0);
        st0(&mut e, GET_ITEM_HEALTH, 50.0);
        answer(&mut e, FORM_TYPE, 0x18);
        st0(&mut e, CALC_ARMOR_RATING, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x007b_7020, &args![broken]);
        assert_eq!(e.global::<u32>(BROKEN_ITEM), broken);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT)[0],
            vec![0x2010, 0x1004, 0.5f32.to_bits(), 1]
        );
        assert_eq!(
            calls_to(&e, COPY),
            vec![vec![calls_to(&e, COPY)[0][0], 0x80, DASHES_TEXT]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![menu + REPAIR_LIST, INIT_FUNC_007B57F0, 0, 0x7fff_ffff]]
        );
        assert!(calls_to(&e, 0x0000_8000).contains(&vec![menu + REPAIR_LIST, COMPARE_007B5950]));
        assert!(calls_to(&e, 0x0000_8000).contains(&vec![menu, 0xe, 0]));
        assert_eq!(
            calls_to(&e, TUTORIAL_SHOW_MESSAGE)[0][1..],
            [6, 0x40b, 0x1f4]
        );
        assert_eq!(calls_to(&e, INTERFACE_007044C0), vec![vec![1, 0, 0]]);
    }
}
