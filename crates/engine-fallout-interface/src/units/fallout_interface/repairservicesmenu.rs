//! `fallout/interface/menus/repairservicesmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `RepairServicesMenu::pMe` and `pRepairVendor` (Xbox PDB statics): the open menu and the
/// vendor who repairs.
const SERVICES_MENU: u32 = 0x011d_a7f0;
const REPAIR_VENDOR: u32 = 0x011d_a7f4;
/// The trait ids the constructor looks up with `"_Title"`, `"_Value"`, `"_RepairCost"` and
/// `"_CanRepair"`.
const TITLE_TRAIT_011DA7E0: u32 = 0x011d_a7e0;
const VALUE_TRAIT_011DA7E8: u32 = 0x011d_a7e8;
const REPAIR_COST_TRAIT_011DA7E4: u32 = 0x011d_a7e4;
const CAN_REPAIR_TRAIT_011DA7DC: u32 = 0x011d_a7dc;
/// The menu's vtable and class number.
const SERVICES_MENU_VTABLE: u32 = 0x0107_5db4;
const SERVICES_MENU_CLASS: u32 = 0x422;
/// `RepairServicesMenu` fields (Xbox PDB names, the PC code's offsets): `lbRepairList`
/// (+0x68), `iRepairAllCost` (+0x98), `iVendorRepairSkill` (+0x9c).
const REPAIR_LIST: u32 = 0x68;
const REPAIR_ALL_COST: u32 = 0x98;
const VENDOR_REPAIR_SKILL: u32 = 0x9c;
/// The player.
const PLAYER: u32 = 0x011d_ea3c;

/// `Menu::Menu` (`00a1c4a0`) and the `Menu` destructor body (`00a1c520`),
/// `ListBox<ItemChange_P>`'s constructor (`00730810`) and destructor (`007308e0`), `operator
/// delete` (`00401030`).
const MENU_CONSTRUCTOR: u32 = 0x00a1_c4a0;
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
const LIST_BOX_CONSTRUCTOR: u32 = 0x0073_0810;
const LIST_BOX_DESTRUCTOR: u32 = 0x0073_08e0;
const FREE: u32 = 0x0040_1030;
/// Interface functions: the interface manager (`004b7210`), its tile reader (`00586150`),
/// `Tile::ReadFile` (`RET 4`), `Tile::GetMenu`, `Tile::GetMenuByClass` (`cdecl`),
/// `Menu::SetMenuTile` (`RET 8`), `Tile::GetMaximumDepth` (`ST0`),
/// `Menu::PrepForVisibility` (`RET 4`), `Menu::StartFadeOut`, the word at +4 (`00726070`),
/// `Tile::GetChildByName` (`RET 4`), `Tile::GetChildByID` (`RET 4`), `Tile::TextToTrait`
/// (`cdecl`), `Tile::Poke` (`RET 8`), `Tile::IsTrue` (`RET 4`), the float getter
/// (`00a011b0`, `ST0`) and setter (`00a012d0`, `RET 0xc`), the integer setter (`00700320`,
/// `RET 8`), the string setter (`00a01350`, `RET 0xc`), `ListBox<unsigned int>::Initialize`
/// (`RET 8`), `DialogMenu::ServiceStart` and `ServiceEnd` (`cdecl`), `TESTopic::GetTopic`
/// (`cdecl`, two words), the error report `0040fbe0` and the print function `005b5e40`
/// (`cdecl`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const TILE_READER_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const WORD_AT_FOUR: u32 = 0x0072_6070;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
const SERVICE_START: u32 = 0x0076_3ff0;
const SERVICE_END: u32 = 0x0076_40a0;
const GET_TOPIC: u32 = 0x0061_a2d0;
const REPORT_ERROR_0040FBE0: u32 = 0x0040_fbe0;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The list box (`ListBox<ItemChange *>`) helpers: the current item (`007a1910`), the walk
/// (`007314c0`, `RET 0xc`), the fill from an inventory (`00719ef0`, `RET 0x18`).
const LIST_BOX_CURRENT_ITEM: u32 = 0x007a_1910;
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
const LIST_BOX_FILL_00719EF0: u32 = 0x0071_9ef0;
/// `ItemChange` methods (`thiscall`): `GetItemHealth` (`RET 4`, `ST0`), `GetWorn` (`RET 4`),
/// `HasModEffectActive` (`RET 4`), `GetIconOne` (`RET 4`), `GetScript` (`004bde40`), the
/// form (`0044ddc0`), the extra data lists (`00559450`), the list node's item slot
/// (`006815c0`), `006317a0` (`RET 4`: a tile list's element), `005ae3d0` (`RET 4`: an
/// append), `00905330` (`RET 4`: a removal).
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
const GET_WORN: u32 = 0x004b_ddd0;
const HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
const GET_ICON_ONE: u32 = 0x004b_e200;
const GET_SCRIPT: u32 = 0x004b_de40;
const ITEM_FORM: u32 = 0x0044_ddc0;
const WORD_AT_ZERO: u32 = 0x0055_9450;
const NODE_SELF: u32 = 0x0068_15c0;
const LIST_ELEMENT_006317A0: u32 = 0x0063_17a0;
const LIST_APPEND_005AE3D0: u32 = 0x005a_e3d0;
const LIST_REMOVE_00905330: u32 = 0x0090_5330;
/// `ExtraDataList` methods (see `repairmenu.cpp`'s `RepairItem`).
const EXTRA_LIST_CONSTRUCT: u32 = 0x0041_0360;
const EXTRA_DUPLICATE_FOR_CONTAINER: u32 = 0x0041_2380;
const EXTRA_SET_COUNT: u32 = 0x0041_9ad0;
const EXTRA_SET_HEALTH: u32 = 0x0041_9970;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_00419F80: u32 = 0x0041_9f80;
const EXTRA_COMPARE_FOR_CONTAINER: u32 = 0x0041_26c0;
const ALLOCATE: u32 = 0x0040_1000;
/// Form and actor functions: the type (`00401170`), `TESValueForm::GetFormValue` (`cdecl`),
/// `GamePlayFormulas::GetRepairCost` (`cdecl`, three words), `Actor::GetGoldAmount`
/// (`00891d70`), `Actor::PayGoldToActor` (`RET 8`), `ActorValueOwner::GetClampedActorValue`
/// (`RET 4`), `TESBipedModelForm::GetFormAsBipedModel` (`cdecl`),
/// `TESBipedModelForm::GetPlayable`, `TESContainer::IsGold` (`cdecl`),
/// `TESHealthForm::GetFormAsHealthForm` (`cdecl`), `TESHealthForm::GetFormHealth` (`cdecl`),
/// `TESObjectWEAP::GetFormHealth` (`RET 4`), the armour values `004be060` and `004be180`
/// (`ST0`), `CombatFormulas::CalcArmorRating` (`cdecl`: a word and a `float`, `ST0`),
/// `CombatFormulas::CalcWeaponDamageForDisplay` (`cdecl`, four words, `ST0`), `004bd510`,
/// `0040ebd0`, `00404010` (`cdecl`, two `float`s, `ST0`), `_ftol2`, `__RTDynamicCast`,
/// `InventoryChanges::GetInventoryChanges` (`cdecl`), `InventoryChanges::GetObjectInList`
/// (`RET 0xc`), `0043d4d0` (`thiscall` on a setting: its value's address).
const FORM_TYPE: u32 = 0x0040_1170;
const GET_FORM_VALUE: u32 = 0x0048_e8a0;
const GET_REPAIR_COST: u32 = 0x0064_8230;
const GET_GOLD_AMOUNT: u32 = 0x0089_1d70;
const PAY_GOLD_TO_ACTOR: u32 = 0x0089_24e0;
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const GET_FORM_AS_BIPED_MODEL: u32 = 0x0048_0db0;
const GET_PLAYABLE: u32 = 0x0048_0d90;
const IS_GOLD: u32 = 0x0048_1f10;
const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
const HEALTH_FORM_GET_FORM_HEALTH: u32 = 0x0048_73d0;
const WEAPON_GET_FORM_HEALTH: u32 = 0x004b_cf00;
const ARMOR_004BE060: u32 = 0x004b_e060;
const ARMOR_004BE180: u32 = 0x004b_e180;
const CALC_ARMOR_RATING: u32 = 0x0064_6360;
const CALC_WEAPON_DAMAGE_FOR_DISPLAY: u32 = 0x0064_50f0;
const ROUND_004BD510: u32 = 0x004b_d510;
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
const FTOL2: u32 = 0x00ec_62c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const GET_OBJECT_IN_LIST: u32 = 0x004b_fba0;
const SETTING_VALUE_0043D4D0: u32 = 0x0043_d4d0;
/// `BSAudio::QInstance`, `BSAudio::GetSoundHandleByName` (`RET 0xc`),
/// `BSSoundHandle::Play` (`RET 4`), the sound handle's destructor.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// The game settings' string (`00403df0`) and value address (`00403e20`); the string
/// functions (`cdecl`): format (`00406d00`), copy (`00406d30`), append (`00406d50`),
/// `00408b20` (two strings compared).
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
const FORMAT: u32 = 0x0040_6d00;
const COPY: u32 = 0x0040_6d30;
const APPEND: u32 = 0x0040_6d50;
const STRINGS_00408B20: u32 = 0x0040_8b20;

/// `"_Title"`, `"_Value"`, `"_RepairCost"`, `"_CanRepair"`, the menu file, the null vendor
/// error, the creation failure message, the list template, two child names,
/// `"UIRepairWeapon"`, the failure sound `01073a94`, `"%"`, the percent format
/// `010734b0`, `"ENGLISH"`, `"--"`, the gold formats `01075f34` and `01070854`.
const TITLE_TEXT: u32 = 0x0106_ed20;
const VALUE_TEXT: u32 = 0x0106_ed18;
const REPAIR_COST_TEXT: u32 = 0x0107_5da0;
const CAN_REPAIR_TEXT: u32 = 0x0107_5d94;
const MENU_FILE_TEXT: u32 = 0x0107_5e9c;
const NULL_VENDOR_TEXT: u32 = 0x0107_5ec0;
const CREATION_FAILED_TEXT: u32 = 0x0107_5e40;
const LIST_TEMPLATE_TEXT: u32 = 0x0107_5e28;
const CHILD_01075E14: u32 = 0x0107_5e14;
const CHILD_01075DFC: u32 = 0x0107_5dfc;
const REPAIR_SOUND_TEXT: u32 = 0x0107_5d38;
const FAIL_SOUND_01073A94: u32 = 0x0107_3a94;
const PERCENT_TEXT: u32 = 0x0101_2320;
const PERCENT_FORMAT_010734B0: u32 = 0x0107_34b0;
const ENGLISH_TEXT: u32 = 0x0101_8348;
const DASHES_TEXT: u32 = 0x0104_7008;
const GOLD_CAP_FORMAT_01075F34: u32 = 0x0107_5f34;
const GOLD_FORMAT_01070854: u32 = 0x0107_0854;
/// The RTTI descriptors of the weapon cast (`01183108` to `01183998`).
const RTTI_01183108: u32 = 0x0118_3108;
const RTTI_01183998: u32 = 0x0118_3998;
/// `double`s: `0.0`, `0.5`, `0.6`, `10.0`, `100.0`, `255.0`, `6006.0`, `102.0`; `float`s:
/// `128.0`, `255.0`, the rounding step `01017d00`.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HALF_DOUBLE: u32 = 0x0101_1588;
const SIX_TENTHS_DOUBLE: u32 = 0x0102_90b8;
const TEN_DOUBLE: u32 = 0x0102_0758;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const TWO_FIFTY_FIVE_DOUBLE: u32 = 0x0101_e568;
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
const HALF_ALPHA_FLOAT: u32 = 0x0101_e704;
const FULL_ALPHA_FLOAT: u32 = 0x0102_3cd8;
const ROUND_STEP_01017D00: u32 = 0x0101_7d00;
/// The game settings the menu shows or reads.
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
const SETTING_011D32B8: u32 = 0x011d_32b8;
const SETTING_011D37D4: u32 = 0x011d_37d4;
const SETTING_011D4DA4: u32 = 0x011d_4da4;
const SETTING_011D1F2C: u32 = 0x011d_1f2c;
const SETTING_011D35C4: u32 = 0x011d_35c4;
const SETTING_011D2C7C: u32 = 0x011d_2c7c;
const SETTING_011D396C: u32 = 0x011d_396c;
const SETTING_011D20C4: u32 = 0x011d_20c4;
const SETTING_011D0FF0: u32 = 0x011d_0ff0;
const SETTING_011D018C: u32 = 0x011d_018c;
const SETTING_011D4EE8: u32 = 0x011d_4ee8;
const SETTING_011D3CF4: u32 = 0x011d_3cf4;
const SETTING_011D3A08: u32 = 0x011d_3a08;
const SETTING_011D25B0: u32 = 0x011d_25b0;
const SETTING_011D4030: u32 = 0x011d_4030;
const SETTING_011D4780: u32 = 0x011d_4780;

/// `InitFunc`'s, `FilterFunc`'s, the compare function's and `RepairAllFunc`'s addresses,
/// as the code passes them.
const INIT_FUNC_007B7920: u32 = 0x007b_7920;
const FILTER_FUNC_007B7B40: u32 = 0x007b_7b40;
const COMPARE_007B7C40: u32 = 0x007b_7c40;
const REPAIR_ALL_FUNC_007B82C0: u32 = 0x007b_82c0;

/// The open menu.
fn services_me(e: &Engine) -> u32 {
    e.global::<u32>(SERVICES_MENU)
}

/// The tile word at `offset` of the open menu.
fn services_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(services_me(e).wrapping_add(offset))
}

/// The item word of a list node, reached through `006815c0`.
fn services_node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// Plays the named sound (`BSAudio::GetSoundHandleByName(QInstance, handle, name, 0x121)`,
/// `Play(0)`, the handle destroyed).
fn services_play_sound(e: &mut Engine, name: u32) {
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

/// The setting's string as trait `trait_id` of `tile`.
fn services_setting_text(e: &mut Engine, tile: u32, trait_id: u32, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    e.call(TILE_SET_STRING, &args![tile, trait_id, text, 1u32]);
}

/// `FCOMP double 0.0; TEST AH,0x41; JNZ`: on only when strictly above (ordered).
fn services_above(value: f64, other: f64) -> bool {
    value.partial_cmp(&other) == Some(std::cmp::Ordering::Greater)
}

// Translated from 00754c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::AttachTileByID` (Xbox PDB; `RET 8`): stores `tile` as `pTiles[id]`
/// when `0 <= id < 16`.
pub fn repair_services_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0x10).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(0x28 + 4 * id as u32), tile);
    }
}

// Translated from 007b7400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b7400 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the members' construction) is not translated.
/// `RepairServicesMenu::RepairServicesMenu` (Xbox PDB): the `Menu` base, the vtable
/// `01075db4`, the repair list (+0x68), the trait ids of `"_Title"`, `"_Value"`,
/// `"_RepairCost"` and `"_CanRepair"`, `iRepairAllCost` cleared. Returns `this`.
pub fn repair_services_menu_repair_services_menu(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), SERVICES_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCTOR, &args![this.addr() + REPAIR_LIST]);
    for (name, slot) in [
        (TITLE_TEXT, TITLE_TRAIT_011DA7E0),
        (VALUE_TEXT, VALUE_TRAIT_011DA7E8),
        (REPAIR_COST_TEXT, REPAIR_COST_TRAIT_011DA7E4),
        (CAN_REPAIR_TEXT, CAN_REPAIR_TRAIT_011DA7DC),
    ] {
        let id = e.call(TILE_TEXT_TO_TRAIT, &args![name]).u32();
        e.set_global(slot, id);
    }
    e.mem.set_u32(this.addr() + REPAIR_ALL_COST, 0);
    this
}

// Translated from 007b74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::GetClass` (Xbox PDB): the class number `0x422`.
pub fn repair_services_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    SERVICES_MENU_CLASS
}

// Translated from 007b74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::_scalar_deleting_destructor_` (Xbox PDB; `RET 4`): the destructor,
/// then the object freed when bit 0 of `flags` is set. Returns `this`.
pub fn repair_services_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_007b7500(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007b7500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b7500 - the C++ exception frame (FS:[0] chain, unwind state 0 around the list's destruction) is not translated.
/// The `RepairServicesMenu` destructor (unnamed in the engine map): the vtable, `pMe`
/// cleared, the repair list destroyed and the `Menu` destructor.
pub fn fn_007b7500(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SERVICES_MENU_VTABLE);
    e.set_global(SERVICES_MENU, 0u32);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.addr() + REPAIR_LIST]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 007b7570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::Create` (Xbox PDB, `cdecl`: the vendor). Without a vendor reports
/// an error (`0040fbe0`) and answers 0. Otherwise deletes an open menu of the class, reads
/// the menu file and takes its menu as `pMe`. For a menu of this class: attaches the tile;
/// with a tile missing prints the failure message and answers 0; otherwise sets the depth
/// for the two stacking types, pokes `"_Value"` 1.0 into the tiles at +0x40 and +0x54, keeps
/// the vendor and its repair skill (`0x27`), sets up the repair list (tile +0x30), runs
/// `Update`, starts the service and, with the `"ENGLISH"` language setting, sets six titles;
/// prepares the menu for showing and returns the root tile. A menu of another class is
/// deleted (when it has a tile) and the result is 0.
pub fn repair_services_menu_create(e: &mut Engine, vendor: u32) -> u32 {
    if vendor == 0 {
        e.call(REPORT_ERROR_0040FBE0, &args![NULL_VENDOR_TEXT]);
        return 0;
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let class = fn_007b7890(e);
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER_00586150, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(SERVICES_MENU, menu);
    if services_me(e) != 0 {
        let class = e.vcall(services_me(e), 0x34, &args![]).u32();
        if class == fn_007b7890(e) {
            return finish_services_create(e, root, vendor);
        }
    }
    if services_me(e) != 0
        && e.call(WORD_AT_FOUR, &args![services_me(e)]).u32() != 0
        && services_me(e) != 0
    {
        let menu = services_me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`repair_services_menu_create`] after the menu of the class was read.
fn finish_services_create(e: &mut Engine, root: u32, vendor: u32) -> u32 {
    let menu = services_me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    if !fn_007b78a0(e, Ptr::new(services_me(e))) {
        e.call(PRINT_MESSAGE, &args![CREATION_FAILED_TEXT]);
        return 0;
    }
    // `FCOMP double; TEST AH,0x44`: equal (and ordered) to either stacking type.
    let kind = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = e.call(TILE_GET_FLOAT, &args![root, 0xfa7u32]).f64();
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![root, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
    for offset in [0x40u32, 0x54] {
        let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
        let tile = services_tile(e, offset);
        e.call(TILE_POKE, &args![tile, value_trait, 1.0f32.to_bits()]);
    }
    e.set_global(REPAIR_VENDOR, vendor);
    let skill = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![vendor + 0xa4, 0x27u32])
        .u32();
    let menu = services_me(e);
    e.mem.set_u32(menu + VENDOR_REPAIR_SKILL, skill);
    let container = services_tile(e, 0x30);
    let list = services_me(e) + REPAIR_LIST;
    e.call(
        LIST_BOX_INITIALIZE,
        &args![list, container, LIST_TEMPLATE_TEXT],
    );
    let menu = services_me(e);
    repair_services_menu_update(e, Ptr::new(menu));
    e.call(SERVICE_START, &args![]);
    let language = e.call(SETTING_STRING, &args![SETTING_011C3CB4]).u32();
    if e.call(STRINGS_00408B20, &args![language, ENGLISH_TEXT])
        .u32()
        != 0
    {
        for (child, setting) in [
            (CHILD_01075E14, SETTING_011D32B8),
            (CHILD_01075DFC, SETTING_011D37D4),
        ] {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let title = e.global::<u32>(TITLE_TRAIT_011DA7E0);
            let tile = e.call(TILE_GET_CHILD_BY_NAME, &args![root, child]).u32();
            e.call(TILE_SET_STRING, &args![tile, title, text, 1u32]);
        }
        for offset in [0x3cu32, 0x50] {
            let text = e.call(SETTING_STRING, &args![SETTING_011D4DA4]).u32();
            let title = e.global::<u32>(TITLE_TRAIT_011DA7E0);
            let tile = services_tile(e, offset);
            e.call(TILE_SET_STRING, &args![tile, title, text, 1u32]);
        }
        for (offset, setting) in [(0x5cu32, SETTING_011D1F2C), (0x64, SETTING_011D35C4)] {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let tile = services_tile(e, offset);
            e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        }
    }
    let menu = services_me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    root
}

// Translated from 007b7890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x422`, without an object (no `this`).
pub fn fn_007b7890(_e: &mut Engine) -> u32 {
    SERVICES_MENU_CLASS
}

// Translated from 007b78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all sixteen tiles of `pTiles` are set.
pub fn fn_007b78a0(e: &mut Engine, this: Ptr) -> bool {
    (0..0x10).all(|i| e.mem.u32(this.addr().wrapping_add(0x28 + 4 * i)) != 0)
}

// Translated from 007b78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::Close` (Xbox PDB, no arguments): ends the service with topic
/// `GetTopic(5, 5)`, gives the open menu's tile trait `0x1772` the value 1 and starts its
/// fade-out (the menu is not checked for null).
pub fn repair_services_menu_close(e: &mut Engine) {
    let topic = e.call(GET_TOPIC, &args![5u32, 5u32]).u32();
    e.call(SERVICE_END, &args![topic]);
    let tile = services_tile(e, 4);
    e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
    let menu = services_me(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

/// The tile of a tile list's element: `*006317a0(list, index)`.
fn list_element_tile(e: &mut Engine, list: u32, index: u32) -> u32 {
    let slot = e.call(LIST_ELEMENT_006317A0, &args![list, index]).u32();
    e.mem.u32(slot)
}

// Translated from 007b7920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::InitFunc` (Xbox PDB, `cdecl`: a tile and its `ItemChange`): with
/// both, computes the repair cost (`GetRepairCost(iVendorRepairSkill, health, value)`), the
/// alpha (255 when the cost is above 0 and the player can pay it, else 128), gives the tile
/// trait `0xfaa` 0x11 and the cost, adds it to `iRepairAllCost`, sets the alpha of three
/// tiles, shows the tile when its index is not below `0.0`, sets the health bar (`0xfb1` of
/// child 0x12) to the health times 0.6, `"_CanRepair"` to whether the alpha is 255, and
/// marks a worn item.
pub fn repair_services_menu_init_func(e: &mut Engine, tile: u32, item: u32) {
    if tile == 0 || item == 0 {
        return;
    }
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let price = e.call(GET_FORM_VALUE, &args![form]).i32();
    let price = x87_store32(f64::from(price));
    let health = x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64());
    let skill = services_tile(e, VENDOR_REPAIR_SKILL);
    let cost = e.call(GET_REPAIR_COST, &args![skill, health, price]).i32();
    let player = e.global::<u32>(PLAYER);
    let affordable = cost > 0 && cost <= e.call(GET_GOLD_AMOUNT, &args![player]).i32();
    let alpha = x87_quiet_bits(e.global::<u32>(if affordable {
        FULL_ALPHA_FLOAT
    } else {
        HALF_ALPHA_FLOAT
    }));
    e.call(TILE_SET_INT, &args![tile, 0xfaau32, 0x11u32]);
    let cost_trait = e.global::<u32>(REPAIR_COST_TRAIT_011DA7E4);
    e.call(TILE_SET_INT, &args![tile, cost_trait, cost]);
    let menu = services_me(e);
    let total = e.mem.u32(menu + REPAIR_ALL_COST).wrapping_add(cost as u32);
    let menu = services_me(e);
    e.mem.set_u32(menu + REPAIR_ALL_COST, total);
    let index = e.call(WORD_AT_FOUR, &args![tile + 4]).u32();
    let target = list_element_tile(e, tile + 4, index);
    e.call(TILE_SET_FLOAT, &args![target, 0xfa9u32, alpha, 1u32]);
    let child = e.call(TILE_GET_CHILD_BY_ID, &args![tile, 0x12u32]).u32();
    e.call(TILE_SET_FLOAT, &args![child, 0xfa9u32, alpha, 1u32]);
    let index = e.call(WORD_AT_ZERO, &args![tile + 4]).u32();
    let target = list_element_tile(e, tile + 4, index);
    e.call(TILE_SET_FLOAT, &args![target, 0xfa9u32, alpha, 1u32]);
    let position = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP double 0.0; TEST AH,1; JNZ`: hidden when below `0.0` or NaN.
    let shown = !matches!(
        position.partial_cmp(&zero),
        Some(std::cmp::Ordering::Less) | None
    );
    e.call(TILE_SET_INT, &args![tile, 0xfa3u32, u32::from(shown)]);
    let health = e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64();
    let factor: f64 = e.global(SIX_TENTHS_DOUBLE);
    let bar = x87_store32(x87_mul(health, factor));
    let child = e.call(TILE_GET_CHILD_BY_ID, &args![tile, 0x12u32]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![child, 0xfb1u32, x87_quiet_bits(bar), 1u32],
    );
    // `FCOMP double 255.0; TEST AH,0x44; JP`: equal (and ordered).
    let full = x87_load32(alpha) == e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE);
    let can_repair = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    e.call(TILE_SET_INT, &args![tile, can_repair, u32::from(full)]);
    if e.call(GET_WORN, &args![item, 0u32]).bool() {
        let index = e.call(WORD_AT_ZERO, &args![tile + 4]).u32();
        let target = list_element_tile(e, tile + 4, index);
        e.call(TILE_SET_INT, &args![target, 0xfa3u32, 1u32]);
    }
}

// Translated from 007b7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::FilterFunc` (Xbox PDB, `cdecl`, an `ItemChange`): whether the item
/// is left out of the list: without a form, an unplayable biped model, gold, or a form whose
/// vtable +0x94 says so; then a form without health; a health at or above the setting
/// `011d018c` times 10; otherwise kept, except armour (type 0x18) whose `004be060` value is
/// at or below `0.0` and `004be180` value at or below `0.0`.
pub fn repair_services_menu_filter_func(e: &mut Engine, item: u32) -> bool {
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let biped = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
    if form == 0 {
        return true;
    }
    if biped != 0 && !e.call(GET_PLAYABLE, &args![biped]).bool() {
        return true;
    }
    if e.call(IS_GOLD, &args![form]).bool() {
        return true;
    }
    if e.vcall(form, 0x94, &args![]).bool() {
        return true;
    }
    if e.call(GET_FORM_AS_HEALTH_FORM, &args![form]).u32() == 0 {
        return true;
    }
    let health = e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64();
    let value = e.call(SETTING_VALUE, &args![SETTING_011D018C]).u32();
    let ten: f64 = e.global(TEN_DOUBLE);
    let limit = x87_mul(x87_m32(e.mem.u32(value)), ten);
    // `FCOMP double; TEST AH,0x41; JNP`: left out when the limit is at or below the health
    // (ordered).
    if limit <= health {
        return true;
    }
    if e.call(FORM_TYPE, &args![form]).u32() != 0x18 {
        return false;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let base = e.call(ARMOR_004BE060, &args![form]).f64();
    if !(base <= zero) {
        return false;
    }
    let damaged = e.call(ARMOR_004BE180, &args![form]).f64();
    damaged <= zero
}

// Translated from 007b7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`: two list items; the repair list's sort order): items that can be
/// repaired (`"_CanRepair"`) come first; then higher health first, then worn items first;
/// the order is reversed when neither can be repaired.
// The exe tests the lower health and the first item's worn flag in separate branches that
// give the same order.
#[allow(clippy::if_same_then_else)]
pub fn fn_007b7c40(e: &mut Engine, first: u32, second: u32) -> i32 {
    let trait_id = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    let tile = e.mem.u32(first);
    let first_blocked = !e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool();
    let trait_id = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    let tile = e.mem.u32(second);
    let second_blocked = !e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool();
    let mut order = 0i32;
    if first_blocked && !second_blocked {
        order = 1;
    } else if !first_blocked && second_blocked {
        order = -1;
    } else {
        let first_item = e.mem.u32(first + 4);
        let second_item = e.mem.u32(second + 4);
        let a = e.call(GET_ITEM_HEALTH, &args![first_item, 0u32]).f64();
        let b = e.call(GET_ITEM_HEALTH, &args![second_item, 0u32]).f64();
        if b > a {
            order = 1;
        } else {
            let a = e.call(GET_ITEM_HEALTH, &args![first_item, 0u32]).f64();
            let b = e.call(GET_ITEM_HEALTH, &args![second_item, 0u32]).f64();
            if b < a {
                order = -1;
            } else if e.call(GET_WORN, &args![first_item, 0u32]).bool() {
                order = -1;
            } else if e.call(GET_WORN, &args![second_item, 0u32]).bool() {
                order = 1;
            }
        }
    }
    if first_blocked && second_blocked {
        order = order.wrapping_neg();
    }
    order
}

// Translated from 007b7d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b7d80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the sound handles) is not translated.
/// `RepairServicesMenu::DoClick` (Xbox PDB; `RET 8`: id, tile; without a tile an id below 16
/// takes `pTiles[id]`). Id 0x11: when the tile can be repaired, repairs the list's current
/// item, pays its cost to the vendor, plays `"UIRepairWeapon"` and updates; else plays the
/// failure sound. Id 0xe repairs every item (`RepairAllFunc` over the list), pays
/// `iRepairAllCost`, plays `"UIRepairWeapon"` and updates. Id 0xf closes the menu.
pub fn repair_services_menu_do_click(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    let tile = if tile == 0 && (0..0x10).contains(&id) {
        e.mem.u32(this.addr().wrapping_add(0x28 + 4 * id as u32))
    } else {
        tile
    };
    let base = this.addr();
    match id {
        0x11 => {
            let trait_id = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
            if e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool() {
                let item = e
                    .call(LIST_BOX_CURRENT_ITEM, &args![base + REPAIR_LIST])
                    .u32();
                repair_services_menu_repair_item(e, this, item);
                let cost_trait = e.global::<u32>(REPAIR_COST_TRAIT_011DA7E4);
                let cost = e.call(TILE_GET_FLOAT, &args![tile, cost_trait]).f64();
                let cost = e.call(FTOL2, &args![cost]).u32();
                let player = e.global::<u32>(PLAYER);
                let vendor = e.global::<u32>(REPAIR_VENDOR);
                e.call(PAY_GOLD_TO_ACTOR, &args![player, vendor, cost]);
                services_play_sound(e, REPAIR_SOUND_TEXT);
                repair_services_menu_update(e, this);
            } else {
                services_play_sound(e, FAIL_SOUND_01073A94);
            }
        }
        0xe => {
            e.call(
                LIST_BOX_FOR_EACH,
                &args![
                    base + REPAIR_LIST,
                    REPAIR_ALL_FUNC_007B82C0,
                    0u32,
                    0x7fff_ffffu32
                ],
            );
            let cost = e.mem.u32(base + REPAIR_ALL_COST);
            let player = e.global::<u32>(PLAYER);
            let vendor = e.global::<u32>(REPAIR_VENDOR);
            e.call(PAY_GOLD_TO_ACTOR, &args![player, vendor, cost]);
            services_play_sound(e, REPAIR_SOUND_TEXT);
            repair_services_menu_update(e, this);
        }
        0xf => repair_services_menu_close(e),
        _ => {}
    }
}

/// The repaired condition the vendor reaches: `0040ebd0((setting 011d0ff0 + (setting
/// 011d018c - setting 011d0ff0) * iVendorRepairSkill / 100) / 10, 1.0)`, as a `float`.
fn vendor_fraction(e: &mut Engine, this: Ptr) -> u32 {
    let low = e.call(SETTING_VALUE, &args![SETTING_011D0FF0]).u32();
    let high = e.call(SETTING_VALUE, &args![SETTING_011D018C]).u32();
    let again = e.call(SETTING_VALUE, &args![SETTING_011D0FF0]).u32();
    let range = x87_sub(x87_m32(e.mem.u32(high)), x87_m32(e.mem.u32(again)));
    let skill = e.mem.u32(this.addr() + VENDOR_REPAIR_SKILL) as i32;
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let share = x87_div(f64::from(skill), hundred);
    let level = x87_store32(x87_add(x87_mul(range, share), x87_m32(e.mem.u32(low))));
    let ten: f64 = e.global(TEN_DOUBLE);
    let scaled = x87_store32(x87_div(x87_load32(level), ten));
    let limited = e
        .call(
            FLOAT_PAIR_0040EBD0,
            &args![x87_quiet_bits(scaled), 1.0f32.to_bits()],
        )
        .f64();
    x87_store32(limited)
}

// Translated from 007b7f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b7f70 - the C++ exception frame (FS:[0] chain, unwind state 0 around the extra data list allocation) is not translated.
/// `RepairServicesMenu::RepairItem` (Xbox PDB; `RET 4`: the `ItemChange`): the vendor's
/// repaired condition fraction times the item's full health (a weapon's `GetFormHealth`
/// with its mod effect 10, else `TESHealthForm::GetFormHealth`); a new extra data list (a
/// copy of the item's first, count 1, that health), the old list's count lowered. A matching
/// stack in the player's inventory entry gets one more (the walk stops at a scripted
/// armour); otherwise the new list goes into the entry's lists, or (a single item) the old
/// list takes the new health. The new list is deleted when a stack took it or for a single
/// item; a single item's old list is removed from the entry when a stack took it.
pub fn repair_services_menu_repair_item(e: &mut Engine, this: Ptr, item: u32) {
    let mut fraction = vendor_fraction(e, this);
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, RTTI_01183108, RTTI_01183998, 0u32],
        )
        .u32();
    if weapon != 0 {
        let effect = e.call(HAS_MOD_EFFECT_ACTIVE, &args![item, 0xau32]).u8();
        let health = e
            .call(WEAPON_GET_FORM_HEALTH, &args![weapon, u32::from(effect)])
            .i32();
        fraction = x87_store32(x87_mul(f64::from(health), x87_m32(fraction)));
    } else {
        let form = e.call(ITEM_FORM, &args![item]).u32();
        let health = e.call(HEALTH_FORM_GET_FORM_HEALTH, &args![form]).u32();
        fraction = x87_store32(x87_mul(f64::from(health), x87_m32(fraction)));
    }
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let player = e.global::<u32>(PLAYER);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    let entry = e
        .call(GET_OBJECT_IN_LIST, &args![changes, form, 1u32, 0u32])
        .u32();
    let old = if e.call(WORD_AT_ZERO, &args![item]).u32() != 0 {
        let lists = e.call(WORD_AT_ZERO, &args![item]).u32();
        services_node_item(e, lists)
    } else {
        0
    };
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
        if count > 1 {
            e.call(EXTRA_SET_COUNT, &args![old, count - 1]);
        }
        let mut stacked = false;
        let mut node = e.call(WORD_AT_ZERO, &args![entry]).u32();
        while node != 0 && services_node_item(e, node) != 0 {
            let form = e.call(ITEM_FORM, &args![item]).u32();
            if e.call(FORM_TYPE, &args![form]).u32() == 0x18
                && e.call(GET_SCRIPT, &args![item]).u32() != 0
            {
                break;
            }
            let other = services_node_item(e, node);
            if !e
                .call(EXTRA_COMPARE_FOR_CONTAINER, &args![other, list, 1u32, 0u32])
                .bool()
            {
                let other = services_node_item(e, node);
                if !e
                    .call(EXTRA_COMPARE_FOR_CONTAINER, &args![list, other, 1u32, 0u32])
                    .bool()
                {
                    let other = services_node_item(e, node);
                    let other_count =
                        i32::from(e.call(EXTRA_GET_COUNT, &args![other]).u32() as u16 as i16);
                    let other = services_node_item(e, node);
                    e.call(EXTRA_SET_COUNT, &args![other, other_count + 1]);
                    stacked = true;
                }
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        if !stacked {
            if count > 1 {
                let lists = e.call(WORD_AT_ZERO, &args![entry]).u32();
                e.call(LIST_APPEND_005AE3D0, &args![lists, new_slot]);
            } else {
                let old = e.mem.u32(old_slot);
                e.call(EXTRA_SET_HEALTH, &args![old, x87_quiet_bits(fraction)]);
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

// Translated from 007b82c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::RepairAllFunc` (Xbox PDB, `cdecl`: a tile and its `ItemChange`):
/// repairs the item (`RepairItem` on `pMe`) when the tile can be repaired.
pub fn repair_services_menu_repair_all_func(e: &mut Engine, tile: u32, item: u32) {
    let trait_id = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    if e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool() {
        let menu = services_me(e);
        repair_services_menu_repair_item(e, Ptr::new(menu), item);
    }
}

/// `DoEnter`'s weapon or armour line for the condition `fraction` in the tile at `offset`
/// (of `this` for a weapon, of `pMe` for armour; `buffer` takes `"--"`).
fn show_item_line(e: &mut Engine, this: Ptr, item: u32, fraction: u32, offset: u32, buffer: Ptr) {
    let form = e.call(ITEM_FORM, &args![item]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == 0x28 {
        let title = e.global::<u32>(TITLE_TRAIT_011DA7E0);
        let tile = e.mem.u32(this.addr() + offset);
        services_setting_text(e, tile, title, SETTING_011D2C7C);
        let form = e.call(ITEM_FORM, &args![item]).u32();
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
        let whole = e.call(FTOL2, &args![rounded]).u32();
        let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
        let tile = e.mem.u32(this.addr() + offset);
        e.call(TILE_SET_INT, &args![tile, value_trait, whole]);
        return;
    }
    let form = e.call(ITEM_FORM, &args![item]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() != 0x18 {
        return;
    }
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let base = e.call(ARMOR_004BE060, &args![form]).f64();
    let base = e.call(FTOL2, &args![base]).u32();
    let base = x87_store32(
        e.call(CALC_ARMOR_RATING, &args![base, x87_quiet_bits(fraction)])
            .f64(),
    );
    let form = e.call(ITEM_FORM, &args![item]).u32();
    let damaged = e.call(ARMOR_004BE180, &args![form]).f64();
    let damaged = e.call(FTOL2, &args![damaged]).u32();
    let damaged = x87_store32(
        e.call(CALC_ARMOR_RATING, &args![damaged, x87_quiet_bits(fraction)])
            .f64(),
    );
    let zero: f64 = e.global(ZERO_DOUBLE);
    let chosen = if services_above(x87_load32(damaged), zero) {
        Some((SETTING_011D396C, damaged))
    } else if services_above(x87_load32(base), zero) {
        Some((SETTING_011D20C4, base))
    } else {
        None
    };
    match chosen {
        Some((setting, bits)) => {
            let title = e.global::<u32>(TITLE_TRAIT_011DA7E0);
            let tile = services_tile(e, offset);
            services_setting_text(e, tile, title, setting);
            let whole = e.call(FTOL2, &args![x87_load32(bits)]).u32();
            let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
            let tile = services_tile(e, offset);
            e.call(TILE_SET_INT, &args![tile, value_trait, whole]);
        }
        None => {
            e.call(COPY, &args![buffer, 0x80u32, DASHES_TEXT]);
            let title = e.global::<u32>(TITLE_TRAIT_011DA7E0);
            let tile = services_tile(e, offset);
            services_setting_text(e, tile, title, SETTING_011D20C4);
            let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
            let tile = services_tile(e, offset);
            e.call(TILE_SET_STRING, &args![tile, value_trait, buffer, 1u32]);
        }
    }
}

// Translated from 007b82f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b82f0 - the /GS security cookie check (00ec408c) is not translated
/// `RepairServicesMenu::DoEnter` (Xbox PDB; `RET 8`: id, tile). Only id 0x11: for the list's
/// current item, enables the repair button (`0xfaf` of +0x5c) as `"_CanRepair"` says, shows
/// the item's icon and its condition with the weapon's damage or the armour's ratings in
/// the tiles from +0x3c. When the vendor's condition (rounded with `01017d00`) is not above
/// the item's, shows the setting `011d4ee8` message with the vendor's limit and hides +0x50.
/// Otherwise shows the vendor's condition and the line at it from +0x50, the gains in
/// percent (+0x44) and value (+0x48), the cost (setting `011d3cf4`, +0x58) and its alpha.
pub fn repair_services_menu_do_enter(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if id != 0x11 {
        return;
    }
    let base = this.addr();
    let item = e
        .call(LIST_BOX_CURRENT_ITEM, &args![base + REPAIR_LIST])
        .u32();
    let can_repair = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    let enabled = e.call(TILE_IS_TRUE, &args![tile, can_repair]).u8();
    let button = e.mem.u32(base + 0x5c);
    e.call(TILE_SET_INT, &args![button, 0xfafu32, u32::from(enabled)]);
    let tile38 = e.mem.u32(base + 0x38);
    e.call(TILE_SET_INT, &args![tile38, 0xfa3u32, 1u32]);
    let icon = e.call(GET_ICON_ONE, &args![item, 0u32]).u32();
    let tile34 = e.mem.u32(base + 0x34);
    e.call(TILE_SET_STRING, &args![tile34, 0xfccu32, icon, 1u32]);
    let tile34 = e.mem.u32(base + 0x34);
    e.call(TILE_SET_INT, &args![tile34, 0xfa3u32, 1u32]);
    let health = e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let condition = x87_store32(x87_div(health, hundred));
    let tile3c = e.mem.u32(base + 0x3c);
    e.call(
        TILE_SET_FLOAT,
        &args![tile3c, 0x1004u32, x87_quiet_bits(condition), 1u32],
    );
    e.with_stack(0x80, |e, buffer| {
        show_item_line(e, this, item, condition, 0x40, buffer);
    });
    let fraction = vendor_fraction(e, this);
    let step = x87_quiet_bits(e.global::<u32>(ROUND_STEP_01017D00));
    let vendor_level = e
        .call(ROUND_004BD510, &args![x87_quiet_bits(fraction), step])
        .f64();
    let step = x87_quiet_bits(e.global::<u32>(ROUND_STEP_01017D00));
    let item_level = e
        .call(ROUND_004BD510, &args![x87_quiet_bits(condition), step])
        .f64();
    // `FCOMP double; TEST AH,1; JNZ`: repairs only when the item's rounded condition is
    // below the vendor's (or unordered).
    if !matches!(
        item_level.partial_cmp(&vendor_level),
        Some(std::cmp::Ordering::Less) | None
    ) {
        e.with_stack(0x40, |e, message| {
            let hundred: f64 = e.global(HUNDRED_DOUBLE);
            let limit = x87_mul(x87_load32(fraction), hundred).to_bits();
            let format = e.call(SETTING_STRING, &args![SETTING_011D4EE8]).u32();
            e.call(
                FORMAT,
                &args![
                    message,
                    0x40u32,
                    format,
                    limit as u32,
                    (limit >> 32) as u32,
                    PERCENT_TEXT
                ],
            );
            let tile = e.mem.u32(base + 0x4c);
            e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, message, 1u32]);
        });
        let tile50 = e.mem.u32(base + 0x50);
        e.call(TILE_SET_INT, &args![tile50, 0xfa3u32, 0u32]);
        return;
    }
    let tile50 = e.mem.u32(base + 0x50);
    e.call(
        TILE_SET_FLOAT,
        &args![tile50, 0x1004u32, x87_quiet_bits(fraction), 1u32],
    );
    let tile50 = e.mem.u32(base + 0x50);
    e.call(TILE_SET_INT, &args![tile50, 0xfa3u32, 1u32]);
    e.with_stack(0x80, |e, buffer| {
        show_item_line(e, this, item, fraction, 0x54, buffer);
    });
    let tile = e.mem.u32(base + 0x50);
    let after = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    let tile = e.mem.u32(base + 0x3c);
    let before = e.call(TILE_GET_FLOAT, &args![tile, 0x1004u32]).f64();
    let condition_gain = x87_store32(x87_mul(x87_sub(after, before), hundred));
    let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
    let tile = e.mem.u32(base + 0x54);
    let after = e.call(TILE_GET_FLOAT, &args![tile, value_trait]).f64();
    let value_trait = e.global::<u32>(VALUE_TRAIT_011DA7E8);
    let tile = e.mem.u32(base + 0x40);
    let before = e.call(TILE_GET_FLOAT, &args![tile, value_trait]).f64();
    let value_gain = x87_store32(x87_sub(after, before));
    let condition_gain = x87_store32(
        e.call(
            FLOAT_PAIR_00404010,
            &args![0.0f32.to_bits(), x87_quiet_bits(condition_gain)],
        )
        .f64(),
    );
    let value_gain = x87_store32(
        e.call(
            FLOAT_PAIR_00404010,
            &args![0.0f32.to_bits(), x87_quiet_bits(value_gain)],
        )
        .f64(),
    );
    let half: f64 = e.global(HALF_DOUBLE);
    e.with_stack(0x40, |e, text| {
        let whole = e
            .call(FTOL2, &args![x87_add(x87_load32(condition_gain), half)])
            .u32();
        e.call(
            FORMAT,
            &args![text, 0x40u32, PERCENT_FORMAT_010734B0, whole],
        );
        e.call(APPEND, &args![text, 0x40u32, PERCENT_TEXT]);
        let tile = e.mem.u32(base + 0x44);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        let whole = e
            .call(FTOL2, &args![x87_add(x87_load32(value_gain), half)])
            .u32();
        e.call(
            FORMAT,
            &args![text, 0x40u32, PERCENT_FORMAT_010734B0, whole],
        );
        let tile = e.mem.u32(base + 0x48);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    });
    let cost_trait = e.global::<u32>(REPAIR_COST_TRAIT_011DA7E4);
    let cost = e.call(TILE_GET_FLOAT, &args![tile, cost_trait]).f64();
    let cost = e.call(FTOL2, &args![cost]).u32();
    e.with_stack(0x40, |e, text| {
        let format = e.call(SETTING_STRING, &args![SETTING_011D3CF4]).u32();
        e.call(FORMAT, &args![text, 0x40u32, format, cost]);
        let tile = e.mem.u32(base + 0x58);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    });
    let can_repair = e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC);
    let alpha = if e.call(TILE_IS_TRUE, &args![tile, can_repair]).bool() {
        FULL_ALPHA_FLOAT
    } else {
        HALF_ALPHA_FLOAT
    };
    let alpha = x87_quiet_bits(e.global::<u32>(alpha));
    let tile58 = e.mem.u32(base + 0x58);
    e.call(TILE_SET_FLOAT, &args![tile58, 0xfa9u32, alpha, 1u32]);
}

// Translated from 007b8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RepairServicesMenu::DoLeave` (Xbox PDB; `RET 8`: id, tile): id 0x11 disables the repair
/// button (`0xfaf` 1 on +0x5c) and hides the tiles at +0x34 and +0x38.
pub fn repair_services_menu_do_leave(e: &mut Engine, this: Ptr, id: i32, _tile: u32) {
    if id != 0x11 {
        return;
    }
    let base = this.addr();
    let tile = e.mem.u32(base + 0x5c);
    e.call(TILE_SET_INT, &args![tile, 0xfafu32, 1u32]);
    for offset in [0x34u32, 0x38] {
        let tile = e.mem.u32(base + offset);
        e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
    }
}

// Translated from 007b8b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b8b30 - the /GS security cookie check (00ec408c) is not translated
/// `RepairServicesMenu::Update` (Xbox PDB): with an open menu, clears `iRepairAllCost`,
/// focuses tile 0x11 (vtable +0x14), fills the list from the player's inventory with
/// `FilterFunc` (`00719ef0`), runs `InitFunc` over it, sorts it with `fn_007b7c40`, shows the
/// player's gold (with the setting `011d3a08` cap when above it), the vendor's skill, the
/// repair-all cost, and enables repair-all when the cost is above 0 and affordable.
pub fn repair_services_menu_update(e: &mut Engine, this: Ptr) {
    if services_me(e) == 0 {
        return;
    }
    let menu = services_me(e);
    e.mem.set_u32(menu + REPAIR_ALL_COST, 0);
    let menu = services_me(e);
    e.vcall(menu, 0x14, &args![0x11u32, 0u32]);
    let player = e.global::<u32>(PLAYER);
    let list = services_me(e) + REPAIR_LIST;
    e.call(
        LIST_BOX_FILL_00719EF0,
        &args![list, player, FILTER_FUNC_007B7B40, 0u32, 0u32, 1u32, 0u32],
    );
    let list = services_me(e) + REPAIR_LIST;
    e.call(
        LIST_BOX_FOR_EACH,
        &args![list, INIT_FUNC_007B7920, 0xffff_ffffu32, 0x7fff_ffffu32],
    );
    let list = services_me(e) + REPAIR_LIST;
    e.vcall(list, 0x20, &args![COMPARE_007B7C40]);
    e.with_stack(0x40, |e, text| {
        let player = e.global::<u32>(PLAYER);
        let gold = e.call(GET_GOLD_AMOUNT, &args![player]).i32();
        let cap = e
            .call(SETTING_VALUE_0043D4D0, &args![SETTING_011D3A08])
            .u32();
        if gold > e.mem.u32(cap) as i32 {
            let cap = e
                .call(SETTING_VALUE_0043D4D0, &args![SETTING_011D3A08])
                .u32();
            let cap = e.mem.u32(cap);
            let label = e.call(SETTING_STRING, &args![SETTING_011D25B0]).u32();
            e.call(
                FORMAT,
                &args![text, 0x40u32, GOLD_CAP_FORMAT_01075F34, label, cap],
            );
        } else {
            let player = e.global::<u32>(PLAYER);
            let gold = e.call(GET_GOLD_AMOUNT, &args![player]).u32();
            let label = e.call(SETTING_STRING, &args![SETTING_011D25B0]).u32();
            e.call(
                FORMAT,
                &args![text, 0x40u32, GOLD_FORMAT_01070854, label, gold],
            );
        }
        let tile = e.mem.u32(this.addr() + 0x2c);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        let skill = e.mem.u32(this.addr() + VENDOR_REPAIR_SKILL);
        let label = e.call(SETTING_STRING, &args![SETTING_011D4030]).u32();
        e.call(
            FORMAT,
            &args![text, 0x40u32, GOLD_FORMAT_01070854, label, skill],
        );
        let tile = e.mem.u32(this.addr() + 0x28);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
        let total = services_tile(e, REPAIR_ALL_COST);
        let format = e.call(SETTING_STRING, &args![SETTING_011D4780]).u32();
        e.call(FORMAT, &args![text, 0x40u32, format, total]);
        let tile = services_tile(e, 0x60);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    });
    let total = services_tile(e, REPAIR_ALL_COST) as i32;
    let affordable = total > 0 && {
        let player = e.global::<u32>(PLAYER);
        let gold = e.call(GET_GOLD_AMOUNT, &args![player]).i32();
        services_tile(e, REPAIR_ALL_COST) as i32 <= gold
    };
    let tile = services_tile(e, 0x60);
    e.call(TILE_SET_INT, &args![tile, 0xfafu32, u32::from(affordable)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0075_4c00,
            repair_services_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(0x007b_7400, repair_services_menu_repair_services_menu(Ptr) -> Ptr),
        entry!(0x007b_74c0, repair_services_menu_get_class(Ptr) -> u32),
        entry!(0x007b_74d0, repair_services_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x007b_7500, fn_007b7500(Ptr)),
        entry!(0x007b_7570, repair_services_menu_create(u32) -> u32),
        entry!(0x007b_7890, fn_007b7890() -> u32),
        entry!(0x007b_78a0, fn_007b78a0(Ptr) -> bool),
        entry!(0x007b_78e0, repair_services_menu_close()),
        entry!(0x007b_7920, repair_services_menu_init_func(u32, u32)),
        entry!(0x007b_7b40, repair_services_menu_filter_func(u32) -> bool),
        entry!(0x007b_7c40, fn_007b7c40(u32, u32) -> i32),
        entry!(0x007b_7d80, repair_services_menu_do_click(Ptr, i32, u32)),
        entry!(0x007b_7f70, repair_services_menu_repair_item(Ptr, u32)),
        entry!(0x007b_82c0, repair_services_menu_repair_all_func(u32, u32)),
        entry!(0x007b_82f0, repair_services_menu_do_enter(Ptr, i32, u32)),
        entry!(0x007b_8ae0, repair_services_menu_do_leave(Ptr, i32, u32)),
        entry!(0x007b_8b30, repair_services_menu_update(Ptr)),
    ]
}

#[cfg(test)]
mod n057_tests {
    use super::*;

    /// Every callee outside this file the translations above reach.
    const EXTERNAL: &[u32] = &[
        0x00401000, 0x00401030, 0x00401170, 0x00403df0, 0x00403e20, 0x00404010, 0x00406d00,
        0x00406d30, 0x00406d50, 0x00408b20, 0x0040ebd0, 0x0040fbe0, 0x00410360, 0x00412380,
        0x004126c0, 0x00418770, 0x00419970, 0x00419ad0, 0x00419f80, 0x0043d4d0, 0x0044ddc0,
        0x00453a70, 0x00480d90, 0x00480db0, 0x00481f10, 0x00483710, 0x004872e0, 0x004873d0,
        0x0048e8a0, 0x004b7210, 0x004bcdb0, 0x004bcf00, 0x004bd510, 0x004bda70, 0x004bddd0,
        0x004bde40, 0x004be060, 0x004be180, 0x004be200, 0x004bf220, 0x004bfba0, 0x00559450,
        0x00586150, 0x005ae3d0, 0x005b5e40, 0x0061a2d0, 0x006317a0, 0x006450f0, 0x00646360,
        0x00648230, 0x0066ef20, 0x006815c0, 0x00700320, 0x00719ef0, 0x00726070, 0x00730810,
        0x007308e0, 0x007314c0, 0x0075a0d0, 0x00763ff0, 0x007640a0, 0x007a1910, 0x00891d70,
        0x008924e0, 0x00905330, 0x00a011b0, 0x00a01230, 0x00a01290, 0x00a012d0, 0x00a01350,
        0x00a01530, 0x00a01860, 0x00a01b00, 0x00a03c90, 0x00a03da0, 0x00a03eb0, 0x00a09030,
        0x00a1c4a0, 0x00a1c520, 0x00a1d910, 0x00a1dc20, 0x00a1dc70, 0x00ad7550, 0x00ad8830,
        0x00ec43fb, 0x00ec62c0,
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
        let cap = e.mem.alloc(4);
        e.register_double(SETTING_VALUE_0043D4D0, move |_, _| cap.into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        for page in [
            0x011d_a000,
            0x011d_e000,
            0x0101_1000,
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0102_0000,
            0x0102_3000,
            0x0102_9000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(TWO_FIFTY_FIVE_DOUBLE, 255.0f64);
        e.set_global(FULL_ALPHA_FLOAT, 255.0f32);
        e.set_global(HALF_ALPHA_FLOAT, 128.0f32);
        e.set_global(SIX_TENTHS_DOUBLE, 0.6f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(TEN_DOUBLE, 10.0f64);
        e
    }

    /// A services menu (sixteen tiles, a vtable of doubles, the list's too) as `pMe`.
    fn menu(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0xa0);
        e.put_vtable(0x0e00_0000, &[0x0000_8000; 0x100]);
        e.register(0x0000_8000, |_, _| Ret::default());
        e.mem.set_u32(menu, 0x0e00_0000);
        e.mem.set_u32(menu + REPAIR_LIST, 0x0e00_0000);
        for i in 0..16 {
            e.mem.set_u32(menu + 0x28 + 4 * i, 0x2000 + 4 * i);
        }
        e.set_global(SERVICES_MENU, menu);
        menu
    }

    #[test]
    fn construction_class_and_destruction() {
        let mut e = engine();
        e.register(TILE_TEXT_TO_TRAIT, |_, a| (a[0] & 0xff).into_ret());
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + REPAIR_ALL_COST, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_7400, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SERVICES_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + REPAIR_ALL_COST), 0);
        assert_eq!(calls_to(&e, LIST_BOX_CONSTRUCTOR), vec![vec![this + 0x68]]);
        assert_eq!(e.global::<u32>(REPAIR_COST_TRAIT_011DA7E4), 0xa0);
        assert_eq!(e.global::<u32>(CAN_REPAIR_TRAIT_011DA7DC), 0x94);
        assert_eq!(e.call(0x007b_74c0, &args![this]).u32(), 0x422);
        assert_eq!(e.call(0x007b_7890, &args![]).u32(), 0x422);
        e.call(0x0075_4c00, &args![this, 15i32, 0x99u32]);
        e.call(0x0075_4c00, &args![this, 16i32, 0x98u32]);
        assert_eq!(e.mem.u32(this + 0x28 + 60), 0x99);
        assert!(!e.call(0x007b_78a0, &args![this]).bool());
        for i in 0..16 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x007b_78a0, &args![this]).bool());
        e.set_global(SERVICES_MENU, this);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_74d0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.global::<u32>(SERVICES_MENU), 0);
        assert_eq!(calls_to(&e, LIST_BOX_DESTRUCTOR), vec![vec![this + 0x68]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x007b_7500, &args![this]);
        assert!(calls_to(&e, FREE).is_empty());
    }

    #[test]
    fn create_needs_a_vendor_and_sets_the_menu_up() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_7570, &args![0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, REPORT_ERROR_0040FBE0),
            vec![vec![NULL_VENDOR_TEXT]]
        );
        let menu = menu(&mut e);
        e.mem.set_u32(0x0e00_0000 + 0x34, 0x0000_8034);
        answer(&mut e, 0x0000_8034, 0x422);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, menu);
        answer(&mut e, GET_CLAMPED_ACTOR_VALUE, 45);
        answer(&mut e, STRINGS_00408B20, 1);
        e.set_global(VALUE_TRAIT_011DA7E8, 0x62u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_7570, &args![0x7700u32]).u32(), 0x4444);
        assert_eq!(e.global::<u32>(REPAIR_VENDOR), 0x7700);
        assert_eq!(e.mem.u32(menu + VENDOR_REPAIR_SKILL), 45);
        assert_eq!(
            calls_to(&e, TILE_POKE),
            vec![
                vec![0x2018, 0x62, 1.0f32.to_bits()],
                vec![0x202c, 0x62, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![menu + 0x68, 0x2008, LIST_TEMPLATE_TEXT]]
        );
        assert_eq!(calls_to(&e, SERVICE_START), vec![Vec::<u32>::new()]);
        assert_eq!(calls_to(&e, TILE_SET_STRING).len(), 6 + 3);
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
    }

    #[test]
    fn close_ends_the_service() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + 4, 0x2300);
        answer(&mut e, GET_TOPIC, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x007b_78e0, &args![]);
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![5, 5]]);
        assert_eq!(calls_to(&e, SERVICE_END), vec![vec![0x5150]]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x2300, 0x1772, 1]]);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
    }

    #[test]
    fn init_func_prices_the_item() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + VENDOR_REPAIR_SKILL, 40);
        e.mem.set_u32(menu + REPAIR_ALL_COST, 5);
        answer(&mut e, GET_FORM_VALUE, 300);
        st0(&mut e, GET_ITEM_HEALTH, 50.0);
        answer(&mut e, GET_REPAIR_COST, 20);
        answer(&mut e, GET_GOLD_AMOUNT, 100);
        e.set_global(REPAIR_COST_TRAIT_011DA7E4, 0x63u32);
        e.set_global(CAN_REPAIR_TRAIT_011DA7DC, 0x64u32);
        answer(&mut e, TILE_GET_CHILD_BY_ID, 0x2400);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x2500);
        answer(&mut e, LIST_ELEMENT_006317A0, slot);
        let tile = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x007b_7920, &args![tile, 0x7000u32]);
        assert_eq!(
            calls_to(&e, GET_REPAIR_COST),
            vec![vec![40, 50.0f32.to_bits(), 300.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(menu + REPAIR_ALL_COST), 25);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![tile, 0xfaa, 0x11],
                vec![tile, 0x63, 20],
                vec![tile, 0xfa3, 1],
                vec![tile, 0x64, 1]
            ]
        );
        let alphas: Vec<_> = calls_to(&e, TILE_SET_FLOAT)
            .into_iter()
            .filter(|c| c[1] == 0xfa9)
            .collect();
        assert_eq!(alphas.len(), 3);
        assert!(alphas.iter().all(|c| c[2] == 255.0f32.to_bits()));
        answer(&mut e, GET_GOLD_AMOUNT, 10);
        e.call_log = Some(vec![]);
        e.call(0x007b_7920, &args![tile, 0x7000u32]);
        assert!(calls_to(&e, TILE_SET_INT).contains(&vec![tile, 0x64, 0]));
    }

    #[test]
    fn filter_func_leaves_out_unrepairable_items() {
        let mut e = engine();
        assert!(e.call(0x007b_7b40, &args![0x7000u32]).bool());
        let form = e.mem.alloc(4);
        e.put_vtable(0x0d00_0000, &[0x0000_9000; 0x40]);
        e.register(0x0000_9000, |_, _| Ret::default());
        e.mem.set_u32(form, 0x0d00_0000);
        answer(&mut e, ITEM_FORM, form);
        assert!(e.call(0x007b_7b40, &args![0x7000u32]).bool());
        answer(&mut e, GET_FORM_AS_HEALTH_FORM, 1);
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 9.0f32.to_bits());
        answer(&mut e, SETTING_VALUE, limit);
        st0(&mut e, GET_ITEM_HEALTH, 90.0);
        assert!(e.call(0x007b_7b40, &args![0x7000u32]).bool());
        st0(&mut e, GET_ITEM_HEALTH, 50.0);
        assert!(!e.call(0x007b_7b40, &args![0x7000u32]).bool());
        answer(&mut e, FORM_TYPE, 0x18);
        assert!(e.call(0x007b_7b40, &args![0x7000u32]).bool());
        st0(&mut e, ARMOR_004BE180, 3.0);
        assert!(!e.call(0x007b_7b40, &args![0x7000u32]).bool());
        answer(&mut e, IS_GOLD, 1);
        assert!(e.call(0x007b_7b40, &args![0x7000u32]).bool());
    }

    #[test]
    fn repair_list_order() {
        let mut e = engine();
        e.register(TILE_IS_TRUE, |_, a| u32::from(a[0] == 0x2100).into_ret());
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
        assert_eq!(e.call(0x007b_7c40, &args![first, second]).i32(), -1);
        assert_eq!(e.call(0x007b_7c40, &args![second, first]).i32(), 1);
        e.mem.set_u32(first, 0x2300);
        assert_eq!(e.call(0x007b_7c40, &args![first, second]).i32(), -1);
        e.mem.set_u32(second + 4, 0x7010);
        answer(&mut e, GET_WORN, 1);
        assert_eq!(e.call(0x007b_7c40, &args![first, second]).i32(), 1);
    }

    #[test]
    fn do_click_repairs_and_pays() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.set_global(REPAIR_VENDOR, 0x7700u32);
        e.set_global(PLAYER, 0x7800u32);
        e.call_log = Some(vec![]);
        e.call(0x007b_7d80, &args![menu, 0x11i32, 0x2200u32]);
        assert_eq!(
            calls_to(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2],
            FAIL_SOUND_01073A94
        );
        e.mem.set_u32(menu + REPAIR_ALL_COST, 70);
        e.call_log = Some(vec![]);
        e.call(0x007b_7d80, &args![menu, 0xei32, 0u32]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH)[0],
            vec![menu + 0x68, REPAIR_ALL_FUNC_007B82C0, 0, 0x7fff_ffff]
        );
        assert_eq!(
            calls_to(&e, PAY_GOLD_TO_ACTOR),
            vec![vec![0x7800, 0x7700, 70]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007b_7d80, &args![menu, 0xfi32, 0u32]);
        assert_eq!(calls_to(&e, SERVICE_END).len(), 1);
    }

    #[test]
    fn repair_item_and_repair_all() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + VENDOR_REPAIR_SKILL, 50);
        let low = e.mem.alloc(4);
        e.mem.set_u32(low, 20.0f32.to_bits());
        let high = e.mem.alloc(4);
        e.mem.set_u32(high, 60.0f32.to_bits());
        e.register_double(SETTING_VALUE, move |_, a| {
            if a[0] == SETTING_011D0FF0 {
                low.into_ret()
            } else {
                high.into_ret()
            }
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        answer(&mut e, HEALTH_FORM_GET_FORM_HEALTH, 100);
        let lists = e.mem.alloc(8);
        e.mem.set_u32(lists, 0x4400);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, lists);
        let entry_lists = e.mem.alloc(8);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, entry_lists);
        answer(&mut e, GET_OBJECT_IN_LIST, entry);
        answer(&mut e, EXTRA_GET_COUNT, 1);
        e.call_log = Some(vec![]);
        e.call(0x007b_7f70, &args![menu, item]);
        // (20 + (60 - 20) * 0.5) / 10 = 4.0, limited to 1.0, times 100.
        assert_eq!(
            calls_to(&e, EXTRA_SET_HEALTH),
            vec![
                vec![0, 100.0f32.to_bits()],
                vec![0x4400, 100.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&e, EXTRA_00419F80), vec![vec![0, 0]]);
        answer(&mut e, TILE_IS_TRUE, 1);
        e.call_log = Some(vec![]);
        e.call(0x007b_82c0, &args![0x2100u32, item]);
        assert_eq!(calls_to(&e, GET_OBJECT_IN_LIST).len(), 1);
    }

    #[test]
    fn do_enter_shows_the_repair() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.mem.set_u32(menu + VENDOR_REPAIR_SKILL, 100);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0.9f32.to_bits());
        answer(&mut e, SETTING_VALUE, value);
        e.map(0x0101_7000, 0x1000);
        e.set_global(ROUND_STEP_01017D00, 0.001f32);
        e.register(ROUND_004BD510, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        st0(&mut e, GET_ITEM_HEALTH, 50.0);
        answer(&mut e, LIST_BOX_CURRENT_ITEM, 0x7000);
        e.call(0x007b_82f0, &args![menu, 0x10i32, 0u32]);
        e.call_log = Some(vec![]);
        e.call(0x007b_82f0, &args![menu, 0x11i32, 0x2200u32]);
        // The vendor reaches 0.09 (0.9 / 10), not above the item's 0.5: the message.
        assert_eq!(
            calls_to(&e, TILE_SET_INT).last().unwrap(),
            &vec![0x2028, 0xfa3, 0]
        );
        e.mem.set_u32(value, 9.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x007b_82f0, &args![menu, 0x11i32, 0x2200u32]);
        assert!(calls_to(&e, TILE_SET_FLOAT).contains(&vec![0x2028, 0x1004, 0.9f32.to_bits(), 1]));
        assert_eq!(calls_to(&e, APPEND).len(), 1);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT).last().unwrap(),
            &vec![0x2030, 0xfa9, 128.0f32.to_bits(), 1]
        );
    }

    #[test]
    fn do_leave_and_update() {
        let mut e = engine();
        let menu = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007b_8ae0, &args![menu, 0x11i32, 0u32]);
        e.call(0x007b_8ae0, &args![menu, 0x10i32, 0u32]);
        assert_eq!(
            calls_to(&e, TILE_SET_INT),
            vec![
                vec![0x2034, 0xfaf, 1],
                vec![0x2000 + 0x0c, 0xfa3, 0],
                vec![0x2010, 0xfa3, 0]
            ]
        );
        e.set_global(PLAYER, 0x7800u32);
        answer(&mut e, GET_GOLD_AMOUNT, 500);
        let cap = e.mem.alloc(4);
        e.mem.set_u32(cap, 300);
        answer(&mut e, SETTING_VALUE_0043D4D0, cap);
        e.mem.set_u32(menu + REPAIR_ALL_COST, 7);
        e.mem.set_u32(menu + VENDOR_REPAIR_SKILL, 33);
        e.call_log = Some(vec![]);
        e.call(0x007b_8b30, &args![menu]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FILL_00719EF0),
            vec![vec![menu + 0x68, 0x7800, FILTER_FUNC_007B7B40, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                menu + 0x68,
                INIT_FUNC_007B7920,
                0xffff_ffff,
                0x7fff_ffff
            ]]
        );
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats[0][2..], [GOLD_CAP_FORMAT_01075F34, 0, 300]);
        assert_eq!(formats[1][2..], [GOLD_FORMAT_01070854, 0, 33]);
        assert_eq!(formats[2][2..], [0, 0]);
        assert_eq!(calls_to(&e, TILE_SET_INT), vec![vec![0x2038, 0xfaf, 0]]);
    }
}
