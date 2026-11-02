//! `fallout/interface/menus/chargenmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The character-creation menu (`CharGenMenu`, class `0x418`) lists either
//! the S.P.E.C.I.A.L. attributes (`eItemType` 0) or the skills (`eItemType`
//! 1) in a `ListBox<ActorValue::Index>` at +0x58 and counts the points spent.
//! The menu object is the singleton `CharGenMenu::pMe` (`011d920c`).
//!
//! The compiler's exception-unwinding frames (`FS:[0]`) and the `/GS`
//! cookie checks are not translated; the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `CharGenMenu::pMe` (Xbox PDB static): the menu singleton.
const CHAR_GEN_MENU: u32 = 0x011d_920c;
/// The menu class number `GetClass` answers.
const CHAR_GEN_MENU_CLASS: u32 = 0x418;
/// The vtable the destructor stores (`CharGenMenu`'s).
const CHAR_GEN_MENU_VTABLE: u32 = 0x0107_1bb4;
/// The user-trait ids the menu keeps in statics (the Xbox PDB lists
/// `iUT_BaseValue`, `iUT_ExtraValue`, `iUT_AddedValue`, `iUT_CurrPoints`,
/// `iUT_MaxPoints`, `iUT_Selected`). Which address holds which is not
/// confirmed; they are named by how the code uses them. This one is the
/// value an item starts from.
const TRAIT_BASE_VALUE: u32 = 0x011d_9210;
/// Added to the base value when the display string is built.
const TRAIT_SECOND_VALUE: u32 = 0x011d_9214;
/// Set together with the current points on the menu tile.
const TRAIT_CURRENT_POINTS: u32 = 0x011d_9218;
/// Set together with the maximum points on the menu tile.
const TRAIT_MAXIMUM_POINTS: u32 = 0x011d_921c;
/// Set to 1 on a tagged skill and read by the tag-skill walk.
const TRAIT_TAGGED: u32 = 0x011d_9220;
/// The points added to an entry.
const TRAIT_ADDED_VALUE: u32 = 0x011d_9224;
/// `CharGenMenu::TagSkillsFunc`'s running tag-skill index (a static).
const TAG_SKILL_INDEX: u32 = 0x011d_92a0;
/// `ListBox<ActorValue::Index>::IsSelected`'s function-local static trait
/// id and its guard word (bit 0: initialized).
const IS_SELECTED_TRAIT: u32 = 0x011d_92a4;
const IS_SELECTED_GUARD: u32 = 0x011d_92a8;
/// The same kind of static trait id and guard in `00754a00`.
const TOGGLE_TRAIT: u32 = 0x011d_92ac;
const TOGGLE_GUARD: u32 = 0x011d_92b0;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;

/// `Tile::GetMenuByClass` (cdecl, one word).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// The interface manager singleton getter.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// The tile reader of the interface manager (`this` only).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (`this` + file name).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Menu::SetMenuTile` (`this`, tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (`this`, flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `Menu::StartFadeOut`.
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// The `Menu` destructor body.
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// The word at +4 of an object (a menu's `pMenu` tile).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// Sets an integer trait of a tile (`this`, trait, value).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// Sets a float trait of a tile (`this`, trait, value, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// Sets a string trait of a tile (`this`, trait, text, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// Reads a float trait of a tile (`this`, trait; result in ST0).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `Tile::GetValue` (`this`, trait).
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::GetValueQ` (`this`, trait).
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
/// `Tile::IsTrue` (`this`, trait).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetMaximumDepth` (no arguments; result in ST0).
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Tile::TextToTrait` (cdecl, one word).
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::AddUserTrait` (cdecl: name, -1).
const TILE_ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `Tile::Value::ClearActions`.
const TILE_VALUE_CLEAR_ACTIONS: u32 = 0x00a0_9200;
/// `Tile::Value::AddAction_ov2` (`this`, action, tile, trait).
const TILE_VALUE_ADD_ACTION_TILE: u32 = 0x00a0_9130;
/// `Tile::Value::AddAction` (`this`, action, float).
const TILE_VALUE_ADD_ACTION_FLOAT: u32 = 0x00a0_9080;
/// `Tile::Value::CalculateValue` (`this`, flag).
const TILE_VALUE_CALCULATE: u32 = 0x00a0_9410;
/// `Menu::RenderTemplate` (`this`, parent, template name, flag).
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// A setting's string value (`ECX` = the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// The print function the menu reports a missing tile with (cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `ActorValue::GetActorValueIcon` (cdecl, one word).
const ACTOR_VALUE_ICON: u32 = 0x0066_e9f0;
/// `ActorValue::GetActorValueDesc` (cdecl, one word).
const ACTOR_VALUE_DESCRIPTION: u32 = 0x0066_e9b0;
/// `ActorValue::GetActorValueName` (cdecl, one word).
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
/// `ActorValue::GetSectionCount` (cdecl, one word).
const ACTOR_VALUE_SECTION_COUNT: u32 = 0x0066_ecf0;
/// `ActorValue::ToActorValue` (cdecl, two words).
const ACTOR_VALUE_FROM_SECTION: u32 = 0x0066_ec10;
/// Tests flags of an actor value (cdecl: value, mask; `bool`).
const ACTOR_VALUE_HAS_FLAGS: u32 = 0x0040_6d70;
/// `Actor::GetClass`.
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
/// Whether the class tags the actor value (`this`, value; `bool`).
const CLASS_TAGS_SKILL: u32 = 0x005a_5f40;
/// `TESClass::SetTagSkill` (`this`, index, value).
const CLASS_SET_TAG_SKILL: u32 = 0x005f_6e80;
/// The number of tag skills (no arguments; answers 4).
const TAG_SKILL_COUNT: u32 = 0x005d_9660;
/// `Interface::PlayMenuSound` (cdecl, one word).
const PLAY_MENU_SOUND: u32 = 0x0070_6f30;
/// The list box's walk over its items with a callback (`this`, callback,
/// first index, last index).
const LIST_BOX_FOR_EACH: u32 = 0x0073_14c0;
/// The value of a list box item's tile (`this`, tile).
const LIST_BOX_VALUE_OF: u32 = 0x007e_1110;
/// The list box's highlighted tile (`this` only).
const LIST_BOX_HIGHLIGHTED: u32 = 0x007d_6e90;
/// `ListBox<unsigned int>::Initialize` (`this`, container tile, template).
const LIST_BOX_INITIALIZE: u32 = 0x0075_a0d0;
/// The list box's destructor body.
const LIST_BOX_DESTRUCTOR: u32 = 0x0072_1150;
/// `ListBox::IsEnabled`.
const LIST_BOX_IS_ENABLED: u32 = 0x0072_4430;
/// Whether a tile belongs to the list box (`this`, tile).
const LIST_BOX_CONTAINS: u32 = 0x0072_42f0;
/// A list box flag test (`this`, word).
const LIST_BOX_FLAG: u32 = 0x0072_42c0;
/// `ListBox<ActorValue::Index>::Synchronize`.
const LIST_BOX_SYNCHRONIZE: u32 = 0x0072_4360;
/// `ListBox::InitTilePosition` (`this`, tile).
const LIST_BOX_INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// `ListBox::SetTotalListHeight`.
const LIST_BOX_SET_TOTAL_LIST_HEIGHT: u32 = 0x0072_4800;
/// The list box's item count (`this` only).
const LIST_BOX_COUNT: u32 = 0x0072_6c40;
/// The list item constructor (`this`, tile, address of the value).
const LIST_ITEM_CONSTRUCT: u32 = 0x007a_62e0;
/// The list's sorted insert (`this`, item, compare function).
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;
/// The list's add-at-head (`this`, address of the item).
const LIST_ADD_HEAD: u32 = 0x0090_5820;
/// The word at +0x28 of a tile (its parent, as the code walks it).
const WORD_AT_0X28: u32 = 0x0045_cd60;
/// Sets a tile's text from a C string (`this`, text).
const TILE_SET_TEXT: u32 = 0x0070_7e10;
/// The tutorial message shower (`this` = interface + 0x4d4; id, class, time).
const SHOW_TUTORIAL: u32 = 0x0071_8630;
/// `CharGenMenu::InitAttributeList` (not translated here).
const INIT_ATTRIBUTE_LIST: u32 = 0x0075_39a0;
/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The bounded `sprintf` (cdecl: buffer, size, format, ...).
const PRINT_BOUNDED: u32 = 0x0040_6d00;
/// The bounded string append (cdecl: buffer, size, text).
const APPEND_BOUNDED: u32 = 0x0040_6d50;
/// `_ftol2_sse`: the value in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The larger of two integers (cdecl).
const MAX_OF: u32 = 0x0064_7b70;
/// The smaller of two integers (cdecl).
const MIN_OF: u32 = 0x004a_8f20;

/// The menu's file name.
const MENU_FILE: u32 = 0x0107_1c5c;
/// The message printed when a tile is missing.
const MISSING_TILE_MESSAGE: u32 = 0x0107_1c08;
/// The trait name of the only-add-values flag.
const ONLY_ADD_TRAIT_NAME: u32 = 0x0107_1bfc;
/// The list item template name.
const ITEM_TEMPLATE_NAME: u32 = 0x0107_1ca0;
/// The user-trait name of the display string.
const DISPLAY_TRAIT_NAME: u32 = 0x0107_1cb8;
/// The user-trait name of the selection flag.
const SELECTED_TRAIT_NAME: u32 = 0x0107_1b64;
/// The user-trait name a list box reads on its container to scroll.
const LIST_SCROLL_TRAIT_NAME: u32 = 0x0106_f860;
/// The format of the display string.
const DISPLAY_FORMAT: u32 = 0x0102_0764;
/// The text appended to a plural points count.
const PLURAL_SUFFIX: u32 = 0x0101_2018;
/// The `double` stacking types the depth is set for.
const STACKING_TYPE_ONE: u32 = 0x0107_04f0;
const STACKING_TYPE_TWO: u32 = 0x0107_04e8;
/// `double` 0.0, the trait values are compared with.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The float written to the list container's trait `0xfa2`.
const LIST_TRAIT_VALUE: u32 = 0x0102_243c;
/// The setting whose value is the tag-skill bonus.
const TAG_BONUS_SETTING: u32 = 0x011c_cf5c;
/// The four setting strings `UpdatePointCounter` formats with, and the two
/// the create function puts on the tiles at +0x40 and +0x44.
const POINTS_FORMAT_ATTRIBUTES: u32 = 0x011d_390c;
const POINTS_FORMAT_SKILLS: u32 = 0x011d_4138;
const REMAINING_FORMAT_ATTRIBUTES: u32 = 0x011d_4de0;
const REMAINING_FORMAT_SKILLS: u32 = 0x011d_3888;
const TITLE_SETTING_ONE: u32 = 0x011d_33d8;
const TITLE_SETTING_TWO: u32 = 0x011d_35c4;
/// The callbacks the list walks are run with.
const TAG_SKILLS_CALLBACK: u32 = 0x0075_4530;
const APPLY_VALUES_CALLBACK: u32 = 0x0075_4460;
const RESET_ATTRIBUTES_CALLBACK: u32 = 0x0075_4330;
const RESET_SKILLS_CALLBACK: u32 = 0x0075_43c0;
const COMPARE_CALLBACK: u32 = 0x007d_a270;

/// Tile traits.
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STACKING_TYPE: u32 = 0xfa7;
const TRAIT_DEPTH: u32 = 0xfad;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_IMAGE: i32 = 0xfcc;
const TRAIT_ID: i32 = 0xfaa;
const TRAIT_LIST_INDEX: i32 = 0xfac;
const TRAIT_0XFA2: u32 = 0xfa2;
const TRAIT_0XFB0: u32 = 0xfb0;
const TRAIT_0X1772: i32 = 0x1772;

layout! {
    /// `CharGenMenu` (Xbox PDB), 0x88 bytes; the PC code uses the PDB's
    /// offsets.
    pub struct CharGenMenu: 0x88 {
        /// `eItemType` (Xbox PDB): 0 attributes, 1 skills.
        0x28 eItemType: i32,
        /// `iCurrPoints` (Xbox PDB).
        0x48 iCurrPoints: i32,
        /// `iMaxPoints` (Xbox PDB).
        0x4C iMaxPoints: i32,
        /// `fPrevHighlightIndex` (Xbox PDB), as its bit pattern.
        0x50 fPrevHighlightIndex: u32,
        /// `bOnlyAddValues` (Xbox PDB).
        0x54 bOnlyAddValues: u8,
        /// `bShowTaggedSkills` (Xbox PDB).
        0x55 bShowTaggedSkills: u8,
    }
}

/// Offset of `pTiles` (Xbox PDB), seven tile pointers.
const TILES: u32 = 0x2c;
/// Offset of `lbItemList` (Xbox PDB), the `ListBox<ActorValue::Index>`.
const ITEM_LIST: u32 = 0x58;

fn me(e: &Engine) -> u32 {
    e.global::<u32>(CHAR_GEN_MENU)
}

fn tile_at(e: &Engine, menu: u32, offset: u32) -> u32 {
    e.mem.u32(menu.wrapping_add(offset))
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

fn set_string(e: &mut Engine, tile: u32, trait_id: i32, text: u32) {
    e.call(SET_STRING_TRAIT, &args![tile, trait_id, text, true]);
}

fn set_float_bits(e: &mut Engine, tile: u32, trait_id: u32, bits: u32) {
    e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, bits, true]);
}

fn float_trait(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f64()
}

fn player_class(e: &mut Engine) -> u32 {
    let player = e.global::<u32>(PLAYER);
    e.call(ACTOR_GET_CLASS, &args![player]).u32()
}

fn for_each(e: &mut Engine, list: u32, callback: u32) {
    e.call(
        LIST_BOX_FOR_EACH,
        &args![list, callback, 0u32, 0x7fff_ffffu32],
    );
}

/// The static user-trait id in `slot` behind `guard` (bit 0 set and the id
/// added by `Tile::AddUserTrait(name, -1)` on the first use).
fn static_trait(e: &mut Engine, guard: u32, slot: u32) -> u32 {
    let flags = e.global::<u32>(guard);
    if flags & 1 == 0 {
        e.set_global(guard, flags | 1);
        let id = e
            .call(TILE_ADD_USER_TRAIT, &args![SELECTED_TRAIT_NAME, -1i32])
            .u32();
        e.set_global(slot, id);
    }
    e.global::<u32>(slot)
}

// Translated from 00753340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::GetClass` (Xbox PDB): the class number `0x418`.
pub fn char_gen_menu_get_class(_e: &mut Engine, _this: Ptr<CharGenMenu>) -> u32 {
    CHAR_GEN_MENU_CLASS
}

// Translated from 00753350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::AttachTileByID` (Xbox PDB): stores `tile` as `pTiles[id]`
/// when `0 <= id < 7`.
pub fn char_gen_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<CharGenMenu>, id: i32, tile: u32) {
    if (0..7).contains(&id) {
        e.mem
            .set_u32(this.addr().wrapping_add(TILES + 4 * id as u32), tile);
    }
}

// Translated from 00753380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn char_gen_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CharGenMenu>,
    flags: u32,
) -> Ptr<CharGenMenu> {
    fn_007533b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007533b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CharGenMenu` destructor: stores the class's vtable, clears the
/// singleton `pMe`, destroys the item list at +0x58 (`00721150`) and runs
/// the `Menu` destructor (`00a1c520`).
// Translation incomplete: 007533b0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007533b0(e: &mut Engine, this: Ptr<CharGenMenu>) {
    e.mem.set_u32(this.addr(), CHAR_GEN_MENU_VTABLE);
    e.set_global(CHAR_GEN_MENU, 0u32);
    e.call(LIST_BOX_DESTRUCTOR, &args![this.byte_add(ITEM_LIST)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 00753420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::Create` (Xbox PDB, cdecl): opens the menu for the item
/// type (0 attributes, 1 skills) with `max_points` points to spend.
/// Returns the menu file's root tile, or 0.
///
/// Fails (0) for a non-positive `max_points` or a type other than 0 and 1
/// (the open menu of the class is looked up first, but only deleted after
/// these checks). Reads the menu file (`01071c5c`), takes its menu as
/// `pMe` and, when it is a menu of this class, attaches the tile, checks
/// that all seven tiles are there (else prints a message and fails), sets
/// the depth for the two stacking types, stores the type, points and flags,
/// fills the title tiles, builds the list (`InitAttributeList` or
/// `InitSkillList`), sets the point traits, shows the highlighted item's
/// icon and description and prepares the menu for showing. A menu of
/// another class is deleted (when it has a tile) and the result is 0.
pub fn char_gen_menu_create(
    e: &mut Engine,
    item_type: i32,
    max_points: i32,
    only_add_values: u8,
    show_tagged_skills: u8,
) -> u32 {
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![CHAR_GEN_MENU_CLASS])
        .u32();
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    if max_points <= 0 || !(0..=1).contains(&item_type) {
        return 0;
    }
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(CHAR_GEN_MENU, menu);

    if me(e) != 0 {
        let class = e.vcall(me(e), 0x34, &args![]).u32();
        if class == fn_00753760(e) {
            return finish_create(
                e,
                root,
                item_type,
                max_points,
                only_add_values,
                show_tagged_skills,
            );
        }
    }
    if me(e) != 0 && e.call(WORD_AT_FOUR, &args![me(e)]).u32() != 0 {
        let menu = me(e);
        e.vcall(menu, 0, &args![1u32]);
    }
    0
}

/// The part of [`char_gen_menu_create`] after the menu of the class was
/// read.
fn finish_create(
    e: &mut Engine,
    root: u32,
    item_type: i32,
    max_points: i32,
    only_add_values: u8,
    show_tagged_skills: u8,
) -> u32 {
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    if !fn_00753770(e, Ptr::new(me(e))) {
        e.call(PRINT_MESSAGE, &args![MISSING_TILE_MESSAGE]);
        return 0;
    }
    let kind = float_trait(e, root, TRAIT_STACKING_TYPE);
    let matches = kind == e.global::<f64>(STACKING_TYPE_ONE) || {
        let again = float_trait(e, root, TRAIT_STACKING_TYPE);
        again == e.global::<f64>(STACKING_TYPE_TWO)
    };
    if matches {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        set_float_bits(e, root, TRAIT_DEPTH, x87_store32(depth));
    }
    e.set(
        Ptr::<CharGenMenu>::new(me(e)),
        CharGenMenu::iMaxPoints,
        max_points,
    );
    e.set(
        Ptr::<CharGenMenu>::new(me(e)),
        CharGenMenu::eItemType,
        item_type,
    );
    e.set(
        Ptr::<CharGenMenu>::new(me(e)),
        CharGenMenu::bShowTaggedSkills,
        show_tagged_skills,
    );
    e.set(
        Ptr::<CharGenMenu>::new(me(e)),
        CharGenMenu::bOnlyAddValues,
        only_add_values,
    );
    let only_trait = e
        .call(TILE_TEXT_TO_TRAIT, &args![ONLY_ADD_TRAIT_NAME])
        .i32();
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    set_trait_from_int(e, tile, only_trait, only_add_values as i32);

    let text = e.call(SETTING_STRING, &args![TITLE_SETTING_ONE]).u32();
    let tile = tile_at(e, me(e), 0x40);
    set_string(e, tile, TRAIT_STRING, text);
    let text = e.call(SETTING_STRING, &args![TITLE_SETTING_TWO]).u32();
    let tile = tile_at(e, me(e), 0x44);
    set_string(e, tile, TRAIT_STRING, text);

    if item_type == 0 {
        e.call(INIT_ATTRIBUTE_LIST, &args![me(e)]);
    } else {
        char_gen_menu_init_skill_list(e, Ptr::new(me(e)));
    }
    let points = e.get(Ptr::<CharGenMenu>::new(me(e)), CharGenMenu::iCurrPoints);
    let trait_id = e.global::<u32>(TRAIT_CURRENT_POINTS);
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, points]);
    let points = e.get(Ptr::<CharGenMenu>::new(me(e)), CharGenMenu::iMaxPoints);
    let trait_id = e.global::<u32>(TRAIT_MAXIMUM_POINTS);
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, points]);
    char_gen_menu_update_point_counter(e, Ptr::new(me(e)));

    let list = me(e).wrapping_add(ITEM_LIST);
    let highlighted = e.call(LIST_BOX_HIGHLIGHTED, &args![list]).u32();
    let list = me(e).wrapping_add(ITEM_LIST);
    let value = e.call(LIST_BOX_VALUE_OF, &args![list, highlighted]).u32();
    let icon = e.call(ACTOR_VALUE_ICON, &args![value]).u32();
    let tile = tile_at(e, me(e), 0x34);
    set_string(e, tile, TRAIT_IMAGE, icon);
    let description = e.call(ACTOR_VALUE_DESCRIPTION, &args![value]).u32();
    let tile = tile_at(e, me(e), 0x38);
    set_string(e, tile, TRAIT_STRING, description);
    let tile = tile_at(e, me(e), 0x34);
    set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
    let tile = tile_at(e, me(e), 0x38);
    set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
    if me(e) != 0 {
        let menu = me(e);
        e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
        root
    } else {
        0
    }
}

// Translated from 00753760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class number `0x418`, without an object (no `this`).
pub fn fn_00753760(_e: &mut Engine) -> u32 {
    CHAR_GEN_MENU_CLASS
}

// Translated from 00753770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all seven tiles of `pTiles` are set.
pub fn fn_00753770(e: &mut Engine, this: Ptr<CharGenMenu>) -> bool {
    (0..7).all(|i| tile_at(e, this.addr(), TILES + 4 * i) != 0)
}

// Translated from 007537b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::Close` (Xbox PDB name from the call graph): with a menu
/// open, writes the values to the player (attributes: the list walk with
/// `00754460`; skills: clears the player class's tag skills, index 0 up to
/// `005d9660()`, then walks the list with `TagSkillsFunc`), sets the menu
/// tile's trait `0x1772` to 1 and starts the fade out.
pub fn char_gen_menu_close(e: &mut Engine) {
    if me(e) == 0 {
        return;
    }
    if e.get(Ptr::<CharGenMenu>::new(me(e)), CharGenMenu::eItemType) == 0 {
        let list = me(e).wrapping_add(ITEM_LIST);
        for_each(e, list, APPLY_VALUES_CALLBACK);
    } else {
        let mut index = 0i32;
        loop {
            player_class(e);
            let count = e.call(TAG_SKILL_COUNT, &args![]).i32();
            if index >= count {
                break;
            }
            let class = player_class(e);
            e.call(CLASS_SET_TAG_SKILL, &args![class, index, -1i32]);
            index += 1;
        }
        let list = me(e).wrapping_add(ITEM_LIST);
        for_each(e, list, TAG_SKILLS_CALLBACK);
    }
    let tile = e.call(WORD_AT_FOUR, &args![me(e)]).u32();
    set_trait_from_int(e, tile, TRAIT_0X1772, 1);
    let menu = me(e);
    e.call(MENU_START_FADE_OUT, &args![menu]);
}

// Translated from 00753870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::DoEnter` (Xbox PDB): ids 8 and 9 (a list item entered):
/// when the tile's list index (trait `0xfac`) differs from
/// `fPrevHighlightIndex` (or either is NaN), shows the item's icon and
/// description, plays menu sound 4 and stores the index. Ids 10 and 11: hands
/// the tile's parent to the item list's slot 0 and enters it as id 8
/// (`this`'s slot `+0x10`). Other ids do nothing.
pub fn char_gen_menu_do_enter(e: &mut Engine, this: Ptr<CharGenMenu>, id: i32, tile: u32) {
    match id {
        8 | 9 => {
            let index = float_trait(e, tile, TRAIT_LIST_INDEX as u32);
            let previous = x87_m32(e.get(this, CharGenMenu::fPrevHighlightIndex));
            if previous == index {
                return;
            }
            let list = this.addr() + ITEM_LIST;
            let value = e.call(LIST_BOX_VALUE_OF, &args![list, tile]).u32();
            let icon = e.call(ACTOR_VALUE_ICON, &args![value]).u32();
            let target = tile_at(e, this.addr(), 0x34);
            set_string(e, target, TRAIT_IMAGE, icon);
            let value = e.call(LIST_BOX_VALUE_OF, &args![list, tile]).u32();
            let description = e.call(ACTOR_VALUE_DESCRIPTION, &args![value]).u32();
            let target = tile_at(e, this.addr(), 0x38);
            set_string(e, target, TRAIT_STRING, description);
            let target = tile_at(e, this.addr(), 0x34);
            set_trait_from_int(e, target, TRAIT_VISIBLE, 1);
            let target = tile_at(e, this.addr(), 0x38);
            set_trait_from_int(e, target, TRAIT_VISIBLE, 1);
            e.call(PLAY_MENU_SOUND, &args![4u32]);
            let index = float_trait(e, tile, TRAIT_LIST_INDEX as u32);
            e.set(this, CharGenMenu::fPrevHighlightIndex, x87_store32(index));
        }
        10 | 11 => {
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            e.vcall(this.addr() + ITEM_LIST, 0, &args![parent]);
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            e.vcall(this.addr(), 0x10, &args![8u32, parent]);
        }
        _ => {}
    }
}

// Translated from 00753b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::InitSkillList` (Xbox PDB): sets up the item list on the
/// tile at +0x30 (template `CGM_SelectItemTemplate`), shows tutorial
/// message `0xe` for the menu's class (time 500) and adds every skill (the
/// actor values of section 2 without flag `0x2000`) with its name, the
/// player's value as the base-value trait and, for a skill the player's
/// class tags, the base value less the tag bonus (setting `011ccf5c`); with
/// `bShowTaggedSkills` the tag trait and the bonus are set too and, unless
/// only values may be added, the tagged skill counts as a spent point.
/// Each item's display string is then built; the list is finally sorted
/// with `007da270` (item list slot `+0x20`).
pub fn char_gen_menu_init_skill_list(e: &mut Engine, this: Ptr<CharGenMenu>) {
    let menu = this.addr();
    let container = tile_at(e, menu, 0x30);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![menu + ITEM_LIST, container, ITEM_TEMPLATE_NAME],
    );
    let value = x87_quiet_bits(e.global::<u32>(LIST_TRAIT_VALUE));
    let container = tile_at(e, menu, 0x30);
    set_float_bits(e, container, TRAIT_0XFA2, value);
    let class = e.vcall(menu, 0x34, &args![]).u32();
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0xeu32, class, 0x1f4u32],
    );
    let mut index = 0u32;
    while index < e.call(ACTOR_VALUE_SECTION_COUNT, &args![2u32]).u32() {
        let value = e
            .call(ACTOR_VALUE_FROM_SECTION, &args![2u32, index & 0xff])
            .u32();
        if !e
            .call(ACTOR_VALUE_HAS_FLAGS, &args![value, 0x2000u32])
            .bool()
        {
            add_skill(e, this, value);
        }
        index = index.wrapping_add(1);
    }
    e.vcall(menu + ITEM_LIST, 0x20, &args![COMPARE_CALLBACK]);
}

/// One skill of [`char_gen_menu_init_skill_list`].
fn add_skill(e: &mut Engine, this: Ptr<CharGenMenu>, value: u32) {
    let menu = this.addr();
    let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
    let tile = list_box_actor_value_add_item(e, Ptr::new(menu + ITEM_LIST), value, name, 0, 0);
    let player = e.global::<u32>(PLAYER);
    // The player's object at +0xa4, slot +0x1c, asked for `value` (an
    // integer value of the actor value; the slot's name is not confirmed).
    let points = e
        .vcall(player.wrapping_add(0xa4), 0x1c, &args![value])
        .i32();
    let base = x87_store32(f64::from(points));
    let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
    e.call(TILE_SET_TEXT, &args![tile, name]);
    set_trait_from_int(e, tile, TRAIT_ID, 9);
    let base_trait = e.global::<u32>(TRAIT_BASE_VALUE);
    set_float_bits(e, tile, base_trait, x87_quiet_bits(base));
    let class = player_class(e);
    if e.call(CLASS_TAGS_SKILL, &args![class, value]).bool() {
        let bonus = e
            .call(SETTING_VALUE_ADDRESS, &args![TAG_BONUS_SETTING])
            .u32();
        let lowered = x87_store32(x87_sub(x87_m32(base), x87_m32(e.mem.u32(bonus))));
        let base_trait = e.global::<u32>(TRAIT_BASE_VALUE);
        set_float_bits(e, tile, base_trait, x87_quiet_bits(lowered));
        if e.get(this, CharGenMenu::bShowTaggedSkills) != 0 {
            let tagged = e.global::<i32>(TRAIT_TAGGED);
            set_trait_from_int(e, tile, tagged, 1);
            let bonus = e
                .call(SETTING_VALUE_ADDRESS, &args![TAG_BONUS_SETTING])
                .u32();
            let bonus = x87_quiet_bits(e.mem.u32(bonus));
            let added = e.global::<u32>(TRAIT_ADDED_VALUE);
            set_float_bits(e, tile, added, bonus);
            if e.get(this, CharGenMenu::bOnlyAddValues) == 0 {
                let points = e.get(this, CharGenMenu::iCurrPoints);
                e.set(this, CharGenMenu::iCurrPoints, points.wrapping_add(1));
            }
        }
    }
    char_gen_menu_set_display_string(e, this, tile);
}

// Translated from 00753d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::UpdatePointCounter` (Xbox PDB name from the call graph):
/// formats the points (current, maximum) with the item type's format
/// setting into the tile at +0x2c; with no points left hides the tile at
/// +0x3c, otherwise formats the remaining points into it (appending the
/// text at `01012018` unless exactly one is left) and shows it.
///
/// The four format strings are kept on the stack in two pairs and indexed
/// by `eItemType`, so the locals are modelled at their frame offsets and
/// any type reads the frame word the exe reads.
// Translation incomplete: 00753d20 - the /GS stack cookie check (00ec408c) is not translated
pub fn char_gen_menu_update_point_counter(e: &mut Engine, this: Ptr<CharGenMenu>) {
    e.with_stack(0xa0, |e, frame| {
        // EBP is the end of the frame; the locals sit below it.
        let ebp = frame.addr() + 0xa0;
        let text = e
            .call(SETTING_STRING, &args![POINTS_FORMAT_ATTRIBUTES])
            .u32();
        e.mem.set_u32(ebp - 0x90, text);
        let text = e.call(SETTING_STRING, &args![POINTS_FORMAT_SKILLS]).u32();
        e.mem.set_u32(ebp - 0x8c, text);
        let text = e
            .call(SETTING_STRING, &args![REMAINING_FORMAT_ATTRIBUTES])
            .u32();
        e.mem.set_u32(ebp - 0x9c, text);
        let text = e
            .call(SETTING_STRING, &args![REMAINING_FORMAT_SKILLS])
            .u32();
        e.mem.set_u32(ebp - 0x98, text);
        let current = e.get(this, CharGenMenu::iCurrPoints);
        let maximum = e.get(this, CharGenMenu::iMaxPoints);
        let remaining = maximum.wrapping_sub(current);
        let buffer = ebp - 0x88;
        let kind = e.get(this, CharGenMenu::eItemType) as u32;
        let format = e.mem.u32((ebp - 0x90).wrapping_add(kind.wrapping_mul(4)));
        e.call(
            PRINT_BOUNDED,
            &args![buffer, 0x80u32, format, current, maximum],
        );
        let tile = tile_at(e, this.addr(), 0x2c);
        set_string(e, tile, TRAIT_STRING, buffer);
        if remaining == 0 {
            let tile = tile_at(e, this.addr(), 0x3c);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        } else {
            let kind = e.get(this, CharGenMenu::eItemType) as u32;
            let format = e.mem.u32((ebp - 0x9c).wrapping_add(kind.wrapping_mul(4)));
            e.call(PRINT_BOUNDED, &args![buffer, 0x80u32, format, remaining]);
            if remaining != 1 {
                e.call(APPEND_BOUNDED, &args![buffer, 0x80u32, PLURAL_SUFFIX]);
            }
            let tile = tile_at(e, this.addr(), 0x3c);
            set_string(e, tile, TRAIT_STRING, buffer);
            let tile = tile_at(e, this.addr(), 0x3c);
            set_trait_from_int(e, tile, TRAIT_VISIBLE, 1);
        }
    });
}

// Translated from 00753ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::DoClick` (Xbox PDB). By clicked id:
/// - 5: resets the points to 0 and walks the list with the reset callback of
///   the item type (`00754330` attributes, `007543c0` skills), updates the
///   point traits and plays menu sound 1 when the points changed;
/// - 6: closes the menu;
/// - 9: toggles the item's selection when it is selected, or when there are
///   points left (a selected item stays when only values may be added and
///   the player's class tags it), sets its added value (the tag bonus when
///   now selected, else 0), plays sound 3 and counts the point;
/// - 10 and 11: one point less (10) or more (11) for the item of the tile's
///   parent, added to its added value;
/// - 7, 8 and others: nothing.
pub fn char_gen_menu_do_click(e: &mut Engine, this: Ptr<CharGenMenu>, id: i32, tile: u32) {
    let list = this.addr() + ITEM_LIST;
    match id {
        9 => {
            let value = e.call(LIST_BOX_VALUE_OF, &args![list, tile]).u32();
            let selected = list_box_actor_value_is_selected(e, Ptr::new(list), tile);
            let toggles = if !selected {
                e.get(this, CharGenMenu::iCurrPoints) < e.get(this, CharGenMenu::iMaxPoints)
            } else if e.get(this, CharGenMenu::bOnlyAddValues) == 0 {
                true
            } else {
                let class = player_class(e);
                !e.call(CLASS_TAGS_SKILL, &args![class, value]).bool()
            };
            if !toggles {
                return;
            }
            fn_00754930(e, Ptr::new(list), tile);
            let bits = if list_box_actor_value_is_selected(e, Ptr::new(list), tile) {
                let bonus = e
                    .call(SETTING_VALUE_ADDRESS, &args![TAG_BONUS_SETTING])
                    .u32();
                x87_quiet_bits(e.mem.u32(bonus))
            } else {
                0
            };
            let added = e.global::<u32>(TRAIT_ADDED_VALUE);
            set_float_bits(e, tile, added, bits);
            e.call(PLAY_MENU_SOUND, &args![3u32]);
            let step = if selected { -1i32 } else { 1 };
            let points = e.get(this, CharGenMenu::iCurrPoints).wrapping_add(step);
            e.set(this, CharGenMenu::iCurrPoints, points);
            set_points_trait(e, this);
            char_gen_menu_set_display_string(e, this, tile);
            char_gen_menu_update_point_counter(e, this);
        }
        10 | 11 => {
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            e.call(LIST_BOX_VALUE_OF, &args![list, parent]);
            let step = if id != 10 { 1i32 } else { -1 };
            let points = e.get(this, CharGenMenu::iCurrPoints).wrapping_add(step);
            e.set(this, CharGenMenu::iCurrPoints, points);
            set_points_trait(e, this);
            let added = e.global::<u32>(TRAIT_ADDED_VALUE);
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            let value = float_trait(e, parent, added);
            let bits = x87_store32(x87_add(value, f64::from(step)));
            let added = e.global::<u32>(TRAIT_ADDED_VALUE);
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            set_float_bits(e, parent, added, x87_quiet_bits(bits));
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            char_gen_menu_set_display_string(e, this, parent);
            char_gen_menu_update_point_counter(e, this);
        }
        6 => char_gen_menu_close(e),
        5 => {
            let before = e.get(this, CharGenMenu::iCurrPoints);
            e.set(this, CharGenMenu::iCurrPoints, 0);
            if e.get(this, CharGenMenu::eItemType) == 0 {
                for_each(e, list, RESET_ATTRIBUTES_CALLBACK);
            }
            if e.get(this, CharGenMenu::eItemType) == 1 {
                for_each(e, list, RESET_SKILLS_CALLBACK);
            }
            set_points_trait(e, this);
            char_gen_menu_update_point_counter(e, this);
            if e.get(this, CharGenMenu::iCurrPoints) != before {
                e.call(PLAY_MENU_SOUND, &args![1u32]);
            }
        }
        _ => {}
    }
}

/// Sets the current-points trait of the menu tile to `iCurrPoints`.
fn set_points_trait(e: &mut Engine, this: Ptr<CharGenMenu>) {
    let points = e.get(this, CharGenMenu::iCurrPoints);
    let trait_id = e.global::<u32>(TRAIT_CURRENT_POINTS);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, points]);
}

// Translated from 00754530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::TagSkillsFunc` (Xbox PDB name from the call graph, cdecl):
/// the list walk's callback that writes the tagged skills to the player's
/// class. The first item (list index trait `0xfac` equal to 0.0) restarts
/// the tag-skill index; an item whose tag trait is true becomes tag skill
/// `index` (the value word) and the index moves on.
pub fn char_gen_menu_tag_skills_func(e: &mut Engine, tile: u32, value: u32) {
    let index = float_trait(e, tile, TRAIT_LIST_INDEX as u32);
    if index == e.global::<f64>(ZERO_DOUBLE) {
        e.set_global(TAG_SKILL_INDEX, 0u32);
    }
    let tagged = e.global::<u32>(TRAIT_TAGGED);
    if e.call(TILE_IS_TRUE, &args![tile, tagged]).bool() {
        let slot = e.global::<u32>(TAG_SKILL_INDEX);
        let class = player_class(e);
        e.call(CLASS_SET_TAG_SKILL, &args![class, slot, value]);
        let slot = e.global::<u32>(TAG_SKILL_INDEX);
        e.set_global(TAG_SKILL_INDEX, slot.wrapping_add(1));
    }
}

// Translated from 007545a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::SetDisplayString` (Xbox PDB): sums the tile's base,
/// second and added value traits (x87, through `double` temporaries),
/// truncates the sum (`_ftol2`), clamps it to 1..10 for attributes or 0..100
/// for skills (`00647b70` then `004a8f20`) and writes it with the format at
/// `01020764` into the tile's display user trait.
// Translation incomplete: 007545a0 - the /GS stack cookie check (00ec408c) is not translated
pub fn char_gen_menu_set_display_string(e: &mut Engine, this: Ptr<CharGenMenu>, tile: u32) {
    let base_trait = e.global::<u32>(TRAIT_BASE_VALUE);
    let base = float_trait(e, tile, base_trait);
    let second_trait = e.global::<u32>(TRAIT_SECOND_VALUE);
    let second = float_trait(e, tile, second_trait);
    let sum = x87_add(second, base);
    let added_trait = e.global::<u32>(TRAIT_ADDED_VALUE);
    let added = float_trait(e, tile, added_trait);
    let total = x87_add(added, sum);
    let mut value = e.call(FTOL, &args![total]).i32();
    match e.get(this, CharGenMenu::eItemType) {
        0 => {
            let low = e.call(MAX_OF, &args![value, 1i32]).i32();
            value = e.call(MIN_OF, &args![low, 0xai32]).i32();
        }
        1 => {
            let low = e.call(MAX_OF, &args![value, 0i32]).i32();
            value = e.call(MIN_OF, &args![low, 0x64i32]).i32();
        }
        _ => {}
    }
    e.with_stack(0x10, |e, buffer| {
        e.call(
            PRINT_BOUNDED,
            &args![buffer, 0x10u32, DISPLAY_FORMAT, value],
        );
        let display = e
            .call(TILE_ADD_USER_TRAIT, &args![DISPLAY_TRAIT_NAME, -1i32])
            .i32();
        set_string(e, tile, display, buffer.addr());
    });
}

// Translated from 00754690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ActorValue::Index>::AddItem` (Xbox PDB name from the call
/// graph): renders the item template (`template`, else the list's
/// `pTemplateName`) into the container tile, gives it id `-1` when it has
/// none and the text (when given), and adds a new list item (0xc bytes,
/// `007a62e0` with the tile and the value's address): sorted with `compare`
/// (then synchronized when the list's flag allows), or at the head (then
/// positioned and the list height updated), the tile taking the next list
/// index. When the list has one item and its container's scroll trait is
/// above 0, the container's `0xfb0` value is rebuilt from the new tile.
/// Returns the tile; 0 without a container or without a template.
// Translation incomplete: 00754690 - the C++ exception-unwinding frame (FS:[0]) that frees the new item is not translated
pub fn list_box_actor_value_add_item(
    e: &mut Engine,
    this: Ptr,
    value: u32,
    text: u32,
    compare: u32,
    template: u32,
) -> u32 {
    // ListBox::pContainerRect (Xbox PDB) +0x0C, pTemplateName +0x18.
    let container = e.mem.u32(this.addr() + 0xc);
    if container == 0 {
        return 0;
    }
    let own_template = e.mem.u32(this.addr() + 0x18);
    if own_template == 0 && template == 0 {
        return 0;
    }
    let name = if template != 0 {
        template
    } else {
        own_template
    };
    let menu = e.call(TILE_GET_MENU, &args![container]).u32();
    let tile = e
        .call(MENU_RENDER_TEMPLATE, &args![menu, container, name, 0u32])
        .u32();
    if e.call(TILE_GET_VALUE, &args![tile, TRAIT_ID]).u32() == 0 {
        set_trait_from_int(e, tile, TRAIT_ID, -1);
    }
    if text != 0 {
        set_string(e, tile, TRAIT_STRING, text);
    }
    if compare != 0 {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let item = new_item(e, block, tile, value);
        e.call(LIST_INSERT_SORTED, &args![this.addr() + 4, item, compare]);
        if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
            e.call(LIST_BOX_SYNCHRONIZE, &args![this]);
        }
    } else {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let item = new_item(e, block, tile, value);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_ADD_HEAD, &args![this.addr() + 4, slot]);
        });
        if e.call(LIST_BOX_FLAG, &args![this, 0u32]).bool() {
            e.call(LIST_BOX_INIT_TILE_POSITION, &args![this, tile]);
            e.call(LIST_BOX_SET_TOTAL_LIST_HEIGHT, &args![this]);
        }
        // ListBox::usNextIndex (Xbox PDB) +0x1C.
        let next = e.mem.u16(this.addr() + 0x1c);
        e.mem.set_u16(this.addr() + 0x1c, next.wrapping_add(1));
        set_trait_from_int(e, tile, TRAIT_LIST_INDEX, i32::from(next));
    }
    if e.call(LIST_BOX_COUNT, &args![this]).u32() == 1 {
        let scroll = e
            .call(TILE_ADD_USER_TRAIT, &args![LIST_SCROLL_TRAIT_NAME, -1i32])
            .u32();
        let container = e.mem.u32(this.addr() + 0xc);
        let amount = float_trait(e, container, scroll);
        // FCOMP; TEST AH,0x41; JNZ: skipped unless strictly above (NaN skips).
        if amount > e.global::<f64>(ZERO_DOUBLE) {
            let container = e.mem.u32(this.addr() + 0xc);
            let action_list = e
                .call(TILE_GET_VALUE_Q, &args![container, TRAIT_0XFB0])
                .u32();
            e.call(TILE_VALUE_CLEAR_ACTIONS, &args![action_list]);
            e.call(
                TILE_VALUE_ADD_ACTION_TILE,
                &args![action_list, 0x7d0u32, tile, TRAIT_0XFB0],
            );
            let scroll = e
                .call(TILE_ADD_USER_TRAIT, &args![LIST_SCROLL_TRAIT_NAME, -1i32])
                .u32();
            let container = e.mem.u32(this.addr() + 0xc);
            let amount = float_trait(e, container, scroll);
            e.call(
                TILE_VALUE_ADD_ACTION_FLOAT,
                &args![action_list, 0x7d3u32, x87_store32(amount)],
            );
            e.call(TILE_VALUE_CALCULATE, &args![action_list, 0u32]);
        }
    }
    tile
}

/// Constructs a list item in `block` (when the allocation succeeded) for
/// the tile and the value, the value passed by address as the exe passes
/// its argument slot. Returns the item (0 when `block` is 0).
fn new_item(e: &mut Engine, block: u32, tile: u32, value: u32) -> u32 {
    if block == 0 {
        return 0;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(LIST_ITEM_CONSTRUCT, &args![block, tile, slot]).u32()
    })
}

// Translated from 00754930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Toggles the selection of a list item: `00754a00(this, tile, 2)`.
pub fn fn_00754930(e: &mut Engine, this: Ptr, tile: u32) {
    fn_00754a00(e, this, tile, 2);
}

// Translated from 00754950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<ActorValue::Index>::IsSelected` (Xbox PDB name from the call
/// graph): whether the tile's selection user trait (a function-local
/// static, added on the first call) is true. False for a null tile or when
/// the trait id is 0.
// Translation incomplete: 00754950 - the C++ exception-unwinding frame (FS:[0]) of the static's initialization is not translated
pub fn list_box_actor_value_is_selected(e: &mut Engine, _this: Ptr, tile: u32) -> bool {
    let selected = static_trait(e, IS_SELECTED_GUARD, IS_SELECTED_TRAIT);
    if selected == 0 || tile == 0 {
        return false;
    }
    e.call(TILE_IS_TRUE, &args![tile, selected]).bool()
}

// Translated from 00754a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the selection user trait (a function-local static) of a list
/// item's tile: mode 0 selects, 1 deselects, 2 toggles. Answers whether the
/// trait was written; nothing is done when the list is disabled, the tile
/// is null or not in the list, the mode is another, or the item already is
/// in the wanted state.
// Translation incomplete: 00754a00 - the C++ exception-unwinding frame (FS:[0]) of the static's initialization is not translated
pub fn fn_00754a00(e: &mut Engine, this: Ptr, tile: u32, mode: i32) -> bool {
    if !e.call(LIST_BOX_IS_ENABLED, &args![this]).bool() || tile == 0 {
        return false;
    }
    if !e.call(LIST_BOX_CONTAINS, &args![this, tile]).bool() {
        return false;
    }
    let selected_trait = static_trait(e, TOGGLE_GUARD, TOGGLE_TRAIT);
    let selected = e.call(TILE_IS_TRUE, &args![tile, selected_trait]).bool();
    let wanted = match mode {
        0 if !selected => 1,
        1 if selected => 0,
        2 => i32::from(!selected),
        _ => return false,
    };
    let selected_trait = e.global::<i32>(TOGGLE_TRAIT);
    set_trait_from_int(e, tile, selected_trait, wanted);
    true
}

/// `double` 1.0, subtracted from an attribute's value when its points are
/// reset.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The attribute list's item template name.
const ATTRIBUTE_TEMPLATE_NAME: u32 = 0x0107_1c88;
/// The float written to the attribute list container's trait `0xfa2`.
const ATTRIBUTE_LIST_TRAIT_VALUE: u32 = 0x0107_1c84;
/// `Tile::GetChildByID` (`this`, id).
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// `Tile::PlayTileSound` (`this`, trait).
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
/// The player's method `0093a850` (`this`, actor value, float), called with
/// an attribute's new value unless values are only added.
const PLAYER_SET_VALUE_0093A850: u32 = 0x0093_a850;

// Translated from 007539a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::InitAttributeList` (Xbox PDB): clears the spent points
/// (and, unless only values may be added, takes 7 off `iMaxPoints`), sets
/// up the item list on the tile at +0x30 (template at `01071c88`, the
/// container's trait `0xfa2` set to the float at `01071c84`), shows
/// tutorial message `0xc` for the menu's class (time 500) and adds every
/// actor value of section 0 with its name: id trait `0xfaa` 8, the player's
/// value (slot +0x1c of the player's object at +0xa4) as the base-value
/// trait, added value 0.0, its display string; unless only values may be
/// added the value less one counts as spent points.
pub fn char_gen_menu_init_attribute_list(e: &mut Engine, this: Ptr<CharGenMenu>) {
    let menu = this.addr();
    e.set(this, CharGenMenu::iCurrPoints, 0);
    if e.get(this, CharGenMenu::bOnlyAddValues) == 0 {
        let maximum = e.get(this, CharGenMenu::iMaxPoints);
        e.set(this, CharGenMenu::iMaxPoints, maximum.wrapping_sub(7));
    }
    let container = tile_at(e, menu, 0x30);
    e.call(
        LIST_BOX_INITIALIZE,
        &args![menu + ITEM_LIST, container, ATTRIBUTE_TEMPLATE_NAME],
    );
    let value = x87_quiet_bits(e.global::<u32>(ATTRIBUTE_LIST_TRAIT_VALUE));
    let container = tile_at(e, menu, 0x30);
    set_float_bits(e, container, TRAIT_0XFA2, value);
    let class = e.vcall(menu, 0x34, &args![]).u32();
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(
        SHOW_TUTORIAL,
        &args![interface.wrapping_add(0x4d4), 0xcu32, class, 0x1f4u32],
    );
    let mut index = 0u32;
    while index < e.call(ACTOR_VALUE_SECTION_COUNT, &args![0u32]).u32() {
        let value = e
            .call(ACTOR_VALUE_FROM_SECTION, &args![0u32, index & 0xff])
            .u32();
        let name = e.call(ACTOR_VALUE_NAME, &args![value]).u32();
        let tile = list_box_actor_value_add_item(e, Ptr::new(menu + ITEM_LIST), value, name, 0, 0);
        let player = e.global::<u32>(PLAYER);
        // The player's object at +0xa4, slot +0x1c, asked for `value` (an
        // integer value of the actor value; the slot's name is not confirmed).
        let points = e
            .vcall(player.wrapping_add(0xa4), 0x1c, &args![value])
            .i32();
        set_trait_from_int(e, tile, TRAIT_ID, 8);
        let base_trait = e.global::<i32>(TRAIT_BASE_VALUE);
        set_trait_from_int(e, tile, base_trait, points);
        let added = e.global::<u32>(TRAIT_ADDED_VALUE);
        set_float_bits(e, tile, added, 0);
        char_gen_menu_set_display_string(e, this, tile);
        if e.get(this, CharGenMenu::bOnlyAddValues) == 0 {
            let spent = e.get(this, CharGenMenu::iCurrPoints);
            e.set(
                this,
                CharGenMenu::iCurrPoints,
                points.wrapping_add(spent).wrapping_sub(1),
            );
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00754150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::DoGamepad` (Xbox PDB): by button (the switch covers 3 to
/// 10; the float word is not read):
/// - 3 and 4, attributes with a highlighted tile (item list slot +4): the
///   child of id 10 (button 4) or 11 (button 3) of that tile, when it is
///   there and visible (`0xfa3`), plays its sound (`0xfcb`) and is clicked
///   (`this` slot +0xc); handled either way;
/// - 9: skills with a highlighted tile: clicks it as id 9 (handled);
///   attributes: the result of button 3 (`this` slot +0x38, value 1.0);
/// - 10: attributes: the result of button 4 the same way;
/// - others (and the cases whose test fails): not handled.
pub fn char_gen_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<CharGenMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    let menu = this.addr();
    let list = menu + ITEM_LIST;
    let one = 1.0f32.to_bits();
    match button {
        3 | 4 => {
            if e.get(this, CharGenMenu::eItemType) != 0 || e.vcall(list, 4, &args![]).u32() == 0 {
                return false;
            }
            let id = 0xa + u32::from(button != 4);
            let highlighted = e.vcall(list, 4, &args![]).u32();
            let child = e.call(TILE_GET_CHILD_BY_ID, &args![highlighted, id]).u32();
            if child != 0 && e.call(TILE_IS_TRUE, &args![child, TRAIT_VISIBLE]).bool() {
                e.call(TILE_PLAY_TILE_SOUND, &args![child, 0xfcbu32]);
                e.vcall(menu, 0xc, &args![id, child]);
            }
            true
        }
        9 => {
            if e.get(this, CharGenMenu::eItemType) == 1 && e.vcall(list, 4, &args![]).u32() != 0 {
                let highlighted = e.vcall(list, 4, &args![]).u32();
                e.vcall(menu, 0xc, &args![9u32, highlighted]);
                true
            } else if e.get(this, CharGenMenu::eItemType) == 0 {
                e.vcall(menu, 0x38, &args![3u32, one]).bool()
            } else {
                false
            }
        }
        10 => {
            if e.get(this, CharGenMenu::eItemType) == 0 {
                e.vcall(menu, 0x38, &args![4u32, one]).bool()
            } else {
                false
            }
        }
        _ => false,
    }
}

// Translated from 007542d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::DoGamepadUpEvent` (Xbox PDB): button 11 with the tile at
/// +0x44's trait `0xfaf` true plays menu sound 1 and clicks id 6 (tile 0;
/// `this` slot +0xc). Answers whether it was handled; the second word is
/// not read.
pub fn char_gen_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<CharGenMenu>,
    button: i32,
    _unused_2: u32,
) -> bool {
    if button != 0xb {
        return false;
    }
    let tile = tile_at(e, this.addr(), 0x44);
    if !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
        return false;
    }
    e.call(PLAY_MENU_SOUND, &args![1u32]);
    e.vcall(this.addr(), 0xc, &args![6u32, 0u32]);
    true
}

// Translated from 00754330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attribute reset callback of `DoClick` (no Xbox PDB name; cdecl
/// list-walk callback, the value word not read): with a menu open and
/// values not only added, adds the item's base plus second value less 1.0
/// (truncated, `_ftol2`) to the spent points; then clears the added value
/// and rebuilds the display string. Nothing without a menu.
pub fn fn_00754330(e: &mut Engine, tile: u32, _unused_1: u32) {
    if me(e) == 0 {
        return;
    }
    let menu = Ptr::<CharGenMenu>::new(me(e));
    if e.get(menu, CharGenMenu::bOnlyAddValues) == 0 {
        let base_trait = e.global::<u32>(TRAIT_BASE_VALUE);
        let base = float_trait(e, tile, base_trait);
        let second_trait = e.global::<u32>(TRAIT_SECOND_VALUE);
        let second = float_trait(e, tile, second_trait);
        let sum = x87_add(second, base);
        let value = x87_sub(sum, e.global::<f64>(ONE_DOUBLE));
        let points = e.call(FTOL, &args![value]).i32();
        let menu = Ptr::<CharGenMenu>::new(me(e));
        let spent = e.get(menu, CharGenMenu::iCurrPoints);
        e.set(menu, CharGenMenu::iCurrPoints, points.wrapping_add(spent));
    }
    let added = e.global::<u32>(TRAIT_ADDED_VALUE);
    set_float_bits(e, tile, added, 0);
    let menu = Ptr::<CharGenMenu>::new(me(e));
    char_gen_menu_set_display_string(e, menu, tile);
}

// Translated from 007543c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CharGenMenu::ResetSkillsFunc` (Xbox PDB, cdecl list-walk callback): a
/// tagged item (the tag trait true) loses its tag and added value, unless
/// only values may be added and the player's class tags the skill (then,
/// when values are not only added, it counts as a spent point: that
/// increment cannot run, as it needs the flag both set and clear). The
/// display string is rebuilt for every item.
pub fn char_gen_menu_reset_skills_func(e: &mut Engine, tile: u32, value: u32) {
    let tagged = e.global::<u32>(TRAIT_TAGGED);
    if e.call(TILE_IS_TRUE, &args![tile, tagged]).bool() {
        let menu = Ptr::<CharGenMenu>::new(me(e));
        let keeps = e.get(menu, CharGenMenu::bOnlyAddValues) != 0 && {
            let class = player_class(e);
            e.call(CLASS_TAGS_SKILL, &args![class, value]).bool()
        };
        if !keeps {
            let tagged = e.global::<i32>(TRAIT_TAGGED);
            set_trait_from_int(e, tile, tagged, 0);
            let added = e.global::<u32>(TRAIT_ADDED_VALUE);
            set_float_bits(e, tile, added, 0);
        } else {
            let menu = Ptr::<CharGenMenu>::new(me(e));
            if e.get(menu, CharGenMenu::bOnlyAddValues) == 0 {
                let spent = e.get(menu, CharGenMenu::iCurrPoints);
                e.set(menu, CharGenMenu::iCurrPoints, spent.wrapping_add(1));
            }
        }
    }
    let menu = Ptr::<CharGenMenu>::new(me(e));
    char_gen_menu_set_display_string(e, menu, tile);
}

// Translated from 00754460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `Close` writes the attributes with (no Xbox PDB name; cdecl
/// list-walk callback): with a menu open, values only added and attributes
/// listed, the player's current value (slot 0 of its object at +0xa4) plus
/// the item's added value goes through the player's slot +0x394; otherwise
/// the base plus added value goes through `0093a850`.
pub fn fn_00754460(e: &mut Engine, tile: u32, value: u32) {
    let adds_only = me(e) != 0 && {
        let menu = Ptr::<CharGenMenu>::new(me(e));
        e.get(menu, CharGenMenu::bOnlyAddValues) != 0 && e.get(menu, CharGenMenu::eItemType) == 0
    };
    if adds_only {
        let player = e.global::<u32>(PLAYER);
        // The player's object at +0xa4, slot 0, asked for `value` (an
        // integer; the slot's name is not confirmed).
        let current = e.vcall(player.wrapping_add(0xa4), 0, &args![value]).i32();
        let added_trait = e.global::<u32>(TRAIT_ADDED_VALUE);
        let added = float_trait(e, tile, added_trait);
        let sum = x87_quiet_bits(x87_store32(x87_add(added, f64::from(current))));
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x394, &args![value, sum]);
    } else {
        let base_trait = e.global::<u32>(TRAIT_BASE_VALUE);
        let base = float_trait(e, tile, base_trait);
        let added_trait = e.global::<u32>(TRAIT_ADDED_VALUE);
        let added = float_trait(e, tile, added_trait);
        let sum = x87_quiet_bits(x87_store32(x87_add(added, base)));
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_SET_VALUE_0093A850, &args![player, value, sum]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0075_3340,
            char_gen_menu_get_class(Ptr<CharGenMenu>) -> u32
        ),
        entry!(
            0x0075_3350,
            char_gen_menu_attach_tile_by_id(Ptr<CharGenMenu>, i32, u32)
        ),
        entry!(
            0x0075_3380,
            char_gen_menu_scalar_deleting_destructor(Ptr<CharGenMenu>, u32) -> Ptr<CharGenMenu>
        ),
        entry!(0x0075_33b0, fn_007533b0(Ptr<CharGenMenu>)),
        entry!(0x0075_3420, char_gen_menu_create(i32, i32, u8, u8) -> u32),
        entry!(0x0075_3760, fn_00753760() -> u32),
        entry!(0x0075_3770, fn_00753770(Ptr<CharGenMenu>) -> bool),
        entry!(0x0075_37b0, char_gen_menu_close()),
        entry!(
            0x0075_3870,
            char_gen_menu_do_enter(Ptr<CharGenMenu>, i32, u32)
        ),
        entry!(0x0075_3b10, char_gen_menu_init_skill_list(Ptr<CharGenMenu>)),
        entry!(
            0x0075_3d20,
            char_gen_menu_update_point_counter(Ptr<CharGenMenu>)
        ),
        entry!(
            0x0075_3ea0,
            char_gen_menu_do_click(Ptr<CharGenMenu>, i32, u32)
        ),
        entry!(0x0075_4530, char_gen_menu_tag_skills_func(u32, u32)),
        entry!(
            0x0075_45a0,
            char_gen_menu_set_display_string(Ptr<CharGenMenu>, u32)
        ),
        entry!(0x0075_4690, list_box_actor_value_add_item(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x0075_4930, fn_00754930(Ptr, u32)),
        entry!(0x0075_4950, list_box_actor_value_is_selected(Ptr, u32) -> bool),
        entry!(0x0075_4a00, fn_00754a00(Ptr, u32, i32) -> bool),
        entry!(
            0x0075_39a0,
            char_gen_menu_init_attribute_list(Ptr<CharGenMenu>)
        ),
        entry!(
            0x0075_4150,
            char_gen_menu_do_gamepad(Ptr<CharGenMenu>, i32, u32) -> bool
        ),
        entry!(
            0x0075_42d0,
            char_gen_menu_do_gamepad_up_event(Ptr<CharGenMenu>, i32, u32) -> bool
        ),
        entry!(0x0075_4330, fn_00754330(u32, u32)),
        entry!(0x0075_43c0, char_gen_menu_reset_skills_func(u32, u32)),
        entry!(0x0075_4460, fn_00754460(u32, u32)),
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

    /// Registers a double that answers 0 for each address.
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

    fn menu(e: &mut Engine) -> Ptr<CharGenMenu> {
        let this = e.new_object::<CharGenMenu>();
        e.map(CHAR_GEN_MENU & !0xfff, 0x1000);
        e.set_global(CHAR_GEN_MENU, this.addr());
        this
    }

    #[test]
    fn get_class_and_static_class_answer_0x418() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0075_3340, &args![0x1000u32]).u32(), 0x418);
        assert_eq!(e.call(0x0075_3760, &args![]).u32(), 0x418);
    }

    #[test]
    fn attach_tile_by_id_stores_ids_0_to_6_only() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.call(0x0075_3350, &args![this, 6i32, 0x77u32]);
        e.call(0x0075_3350, &args![this, 7i32, 0x88u32]);
        e.call(0x0075_3350, &args![this, -1i32, 0x99u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x2c + 24), 0x77);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0);
    }

    #[test]
    fn deleting_destructor_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[LIST_BOX_DESTRUCTOR, MENU_DESTRUCTOR, OPERATOR_DELETE],
        );
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        let result = e.call(0x0075_3380, &args![this, 1u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.mem.u32(this.addr()), CHAR_GEN_MENU_VTABLE);
        assert_eq!(e.global::<u32>(CHAR_GEN_MENU), 0);
        assert_eq!(
            calls_to(&e, LIST_BOX_DESTRUCTOR),
            vec![vec![this.addr() + 0x58]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0075_3380, &args![this, 2u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn destructor_body_clears_the_singleton() {
        let mut e = Engine::new();
        stub(&mut e, &[LIST_BOX_DESTRUCTOR, MENU_DESTRUCTOR]);
        let this = menu(&mut e);
        e.call(0x0075_33b0, &args![this]);
        assert_eq!(e.global::<u32>(CHAR_GEN_MENU), 0);
    }

    #[test]
    fn all_tiles_set_needs_seven_tiles() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        for i in 0..7 {
            e.mem.set_u32(this.addr() + 0x2c + 4 * i, 0x10 + i);
        }
        assert!(e.call(0x0075_3770, &args![this]).bool());
        e.mem.set_u32(this.addr() + 0x2c + 4 * 6, 0);
        assert!(!e.call(0x0075_3770, &args![this]).bool());
    }

    #[test]
    fn create_refuses_bad_arguments_before_deleting() {
        let mut e = Engine::new();
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        stub(&mut e, &[INTERFACE_SINGLETON]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_3420, &args![2i32, 5i32, 0u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0075_3420, &args![0i32, 0i32, 0u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, TILE_READ_FILE).is_empty());
    }

    #[test]
    fn create_with_a_menu_of_another_class_deletes_it() {
        let mut e = Engine::new();
        menu(&mut e);
        let other = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(other, vtable);
        e.mem.set_u32(vtable, 0x00c0_0000);
        e.mem.set_u32(vtable + 0x34, 0x00c0_0004);
        answer(&mut e, 0x00c0_0004, 0x3f5);
        stub(&mut e, &[0x00c0_0000, INTERFACE_SINGLETON, TILE_READER]);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, other);
        answer(&mut e, WORD_AT_FOUR, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_3420, &args![1i32, 5i32, 0u32, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![other, 1]]);
        assert_eq!(e.global::<u32>(CHAR_GEN_MENU), other);
    }

    #[test]
    fn create_fills_a_skill_menu() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0075_3340);
        for i in 0..7 {
            e.mem.set_u32(this.addr() + 0x2c + 4 * i, 0x100 + i);
        }
        e.map(0x0107_0000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        e.set_global(STACKING_TYPE_ONE, 102.0f64);
        e.set_global(STACKING_TYPE_TWO, 103.0f64);
        e.map(TRAIT_CURRENT_POINTS & !0xfff, 0x1000);
        e.set_global(TRAIT_CURRENT_POINTS, 0x55u32);
        e.set_global(TRAIT_MAXIMUM_POINTS, 0x66u32);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        answer(&mut e, WORD_AT_FOUR, 0x900);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 103.0);
        answer_st0(&mut e, TILE_GET_MAXIMUM_DEPTH, 7.0);
        answer(&mut e, TILE_TEXT_TO_TRAIT, 0x1234);
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_SET_MENU_TILE,
                SET_FLOAT_TRAIT,
                SET_TRAIT_FROM_INT,
                SET_STRING_TRAIT,
                SETTING_STRING,
                LIST_BOX_HIGHLIGHTED,
                LIST_BOX_VALUE_OF,
                ACTOR_VALUE_ICON,
                ACTOR_VALUE_DESCRIPTION,
                MENU_PREP_FOR_VISIBILITY,
                LIST_BOX_INITIALIZE,
                SHOW_TUTORIAL,
                ACTOR_VALUE_SECTION_COUNT,
                PRINT_BOUNDED,
                APPEND_BOUNDED,
                0x0071_0000,
            ],
        );
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x58, list_vtable);
        e.mem.set_u32(list_vtable + 0x20, 0x0071_0000);
        e.call_log = Some(vec![]);
        let root = e.call(0x0075_3420, &args![1i32, 5i32, 1u32, 1u32]).u32();
        assert_eq!(root, 0x4444);
        assert_eq!(e.get(this, CharGenMenu::iMaxPoints), 5);
        assert_eq!(e.get(this, CharGenMenu::eItemType), 1);
        assert_eq!(e.get(this, CharGenMenu::bOnlyAddValues), 1);
        assert_eq!(e.get(this, CharGenMenu::bShowTaggedSkills), 1);
        let floats = calls_to(&e, SET_FLOAT_TRAIT);
        assert_eq!(floats[0], vec![0x4444, 0xfad, 7.0f32.to_bits(), 1]);
        let ints = calls_to(&e, SET_TRAIT_FROM_INT);
        assert!(ints.contains(&vec![0x900, 0x1234, 1]));
        assert!(ints.contains(&vec![0x900, 0x55, 0]));
        assert!(ints.contains(&vec![0x900, 0x66, 5]));
        assert_eq!(
            calls_to(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![this.addr(), 0]]
        );
        assert!(calls_to(&e, INIT_ATTRIBUTE_LIST).is_empty());
    }

    #[test]
    fn create_reports_a_missing_tile() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0075_3340);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        answer(&mut e, TILE_READ_FILE, 0x4444);
        answer(&mut e, TILE_GET_MENU, this.addr());
        stub(
            &mut e,
            &[
                INTERFACE_SINGLETON,
                TILE_READER,
                MENU_SET_MENU_TILE,
                PRINT_MESSAGE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_3420, &args![0i32, 5i32, 0u32, 0u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MISSING_TILE_MESSAGE]]
        );
    }

    #[test]
    fn close_writes_tag_skills_for_the_skill_menu() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        e.set(this, CharGenMenu::eItemType, 1);
        e.map(PLAYER & !0xfff, 0x1000);
        e.set_global(PLAYER, 0x2000u32);
        answer(&mut e, ACTOR_GET_CLASS, 0x3000);
        answer(&mut e, TAG_SKILL_COUNT, 2);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        stub(
            &mut e,
            &[
                CLASS_SET_TAG_SKILL,
                LIST_BOX_FOR_EACH,
                SET_TRAIT_FROM_INT,
                MENU_START_FADE_OUT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_37b0, &args![]);
        assert_eq!(
            calls_to(&e, CLASS_SET_TAG_SKILL),
            vec![vec![0x3000, 0, u32::MAX], vec![0x3000, 1, u32::MAX]]
        );
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this.addr() + 0x58,
                TAG_SKILLS_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x900, 0x1772, 1]]
        );
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
    }

    #[test]
    fn close_applies_attribute_values() {
        let mut e = Engine::new();
        let this = menu(&mut e);
        stub(
            &mut e,
            &[
                LIST_BOX_FOR_EACH,
                WORD_AT_FOUR,
                SET_TRAIT_FROM_INT,
                MENU_START_FADE_OUT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_37b0, &args![]);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this.addr() + 0x58,
                APPLY_VALUES_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
    }

    #[test]
    fn do_enter_shows_a_new_highlight_once() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.mem.set_u32(this.addr() + 0x34, 0x701);
        e.mem.set_u32(this.addr() + 0x38, 0x702);
        e.set(this, CharGenMenu::fPrevHighlightIndex, 2.0f32.to_bits());
        answer_st0(&mut e, GET_FLOAT_TRAIT, 3.0);
        answer(&mut e, LIST_BOX_VALUE_OF, 0x2a);
        answer(&mut e, ACTOR_VALUE_ICON, 0x51);
        answer(&mut e, ACTOR_VALUE_DESCRIPTION, 0x52);
        stub(
            &mut e,
            &[SET_STRING_TRAIT, SET_TRAIT_FROM_INT, PLAY_MENU_SOUND],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3870, &args![this, 9i32, 0x500u32]);
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![0x701, 0xfcc, 0x51, 1], vec![0x702, 0xfc4, 0x52, 1]]
        );
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![4]]);
        assert_eq!(
            e.get(this, CharGenMenu::fPrevHighlightIndex),
            3.0f32.to_bits()
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3870, &args![this, 8i32, 0x500u32]);
        assert!(calls_to(&e, PLAY_MENU_SOUND).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0075_3870, &args![this, 12i32, 0x500u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_enter_on_an_arrow_enters_the_parent_item() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x10, 0x00c0_0010);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x58, list_vtable);
        e.mem.set_u32(list_vtable, 0x00c0_0020);
        answer(&mut e, WORD_AT_0X28, 0x600);
        stub(&mut e, &[0x00c0_0010, 0x00c0_0020]);
        e.call_log = Some(vec![]);
        e.call(0x0075_3870, &args![this, 10i32, 0x500u32]);
        assert_eq!(
            calls_to(&e, 0x00c0_0020),
            vec![vec![this.addr() + 0x58, 0x600]]
        );
        assert_eq!(calls_to(&e, 0x00c0_0010), vec![vec![this.addr(), 8, 0x600]]);
    }

    #[test]
    fn init_skill_list_adds_skills_without_flag_0x2000() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0075_3340);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x58, list_vtable);
        e.mem.set_u32(list_vtable + 0x20, 0x00c0_0020);
        e.mem.set_u32(this.addr() + 0x30, 0x330);
        e.set(this, CharGenMenu::bShowTaggedSkills, 1);
        e.map(0x0102_2000, 0x1000);
        e.set_global(LIST_TRAIT_VALUE, 65.0f32.to_bits());
        e.map(TRAIT_BASE_VALUE & !0xfff, 0x1000);
        e.set_global(TRAIT_BASE_VALUE, 0x61u32);
        e.set_global(TRAIT_TAGGED, 0x62u32);
        e.set_global(TRAIT_ADDED_VALUE, 0x63u32);
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0x1c, 0x00c0_001c);
        answer(&mut e, 0x00c0_001c, 40);
        let bonus = e.mem.alloc(4);
        e.mem.set_f32(bonus, 15.0);
        answer(&mut e, SETTING_VALUE_ADDRESS, bonus);
        answer(&mut e, ACTOR_VALUE_SECTION_COUNT, 2);
        e.register(ACTOR_VALUE_FROM_SECTION, |_, a| Ret {
            eax: 0x20 + a[1],
            ..Ret::default()
        });
        e.register(ACTOR_VALUE_HAS_FLAGS, |_, a| Ret {
            eax: u32::from(a[0] == 0x21),
            ..Ret::default()
        });
        answer(&mut e, CLASS_TAGS_SKILL, 1);
        answer(&mut e, FTOL, 25);
        answer(&mut e, MAX_OF, 25);
        answer(&mut e, MIN_OF, 25);
        stub(
            &mut e,
            &[
                LIST_BOX_INITIALIZE,
                SET_FLOAT_TRAIT,
                INTERFACE_SINGLETON,
                SHOW_TUTORIAL,
                ACTOR_VALUE_NAME,
                ACTOR_GET_CLASS,
                TILE_SET_TEXT,
                SET_TRAIT_FROM_INT,
                GET_FLOAT_TRAIT,
                PRINT_BOUNDED,
                TILE_ADD_USER_TRAIT,
                SET_STRING_TRAIT,
                0x00c0_0020,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3b10, &args![this]);
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![this.addr() + 0x58, 0x330, ITEM_TEMPLATE_NAME]]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x4d4, 0xe, 0x418, 500]]
        );
        let floats = calls_to(&e, SET_FLOAT_TRAIT);
        assert_eq!(floats[0], vec![0x330, 0xfa2, 65.0f32.to_bits(), 1]);
        // The list has no container, so AddItem answers tile 0.
        assert_eq!(floats[1], vec![0, 0x61, 40.0f32.to_bits(), 1]);
        assert_eq!(floats[2], vec![0, 0x61, 25.0f32.to_bits(), 1]);
        assert_eq!(floats[3], vec![0, 0x63, 15.0f32.to_bits(), 1]);
        assert_eq!(floats.len(), 4, "one skill skipped by its flag");
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 1);
        assert_eq!(
            calls_to(&e, 0x00c0_0020),
            vec![vec![this.addr() + 0x58, COMPARE_CALLBACK]]
        );
    }

    #[test]
    fn update_point_counter_formats_points_and_remaining() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.mem.set_u32(this.addr() + 0x2c, 0x72c);
        e.mem.set_u32(this.addr() + 0x3c, 0x73c);
        e.set(this, CharGenMenu::eItemType, 1);
        e.set(this, CharGenMenu::iCurrPoints, 1);
        e.set(this, CharGenMenu::iMaxPoints, 4);
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0] & 0xffff,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                PRINT_BOUNDED,
                APPEND_BOUNDED,
                SET_STRING_TRAIT,
                SET_TRAIT_FROM_INT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3d20, &args![this]);
        let prints = calls_to(&e, PRINT_BOUNDED);
        assert_eq!(&prints[0][1..], &[0x80, 0x4138, 1, 4]);
        assert_eq!(&prints[1][1..], &[0x80, 0x3888, 3]);
        assert_eq!(calls_to(&e, APPEND_BOUNDED).len(), 1);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x73c, 0xfa3, 1]]
        );
        e.set(this, CharGenMenu::iCurrPoints, 4);
        e.call_log = Some(vec![]);
        e.call(0x0075_3d20, &args![this]);
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x73c, 0xfa3, 0]]
        );
        assert_eq!(calls_to(&e, PRINT_BOUNDED).len(), 1);
    }

    #[test]
    fn do_click_selects_an_item_while_points_are_left() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.set(this, CharGenMenu::iMaxPoints, 3);
        e.map(TRAIT_ADDED_VALUE & !0xfff, 0x1000);
        e.set_global(TRAIT_ADDED_VALUE, 0x63u32);
        e.set_global(TRAIT_CURRENT_POINTS, 0x64u32);
        e.set_global(IS_SELECTED_TRAIT, 0x65u32);
        e.set_global(IS_SELECTED_GUARD, 1u32);
        e.set_global(TOGGLE_TRAIT, 0x65u32);
        e.set_global(TOGGLE_GUARD, 1u32);
        // Not selected on the first two questions, selected afterwards.
        let mut asked = 0;
        e.register_double(TILE_IS_TRUE, move |_, _| {
            asked += 1;
            Ret {
                eax: u32::from(asked > 2),
                ..Ret::default()
            }
        });
        let bonus = e.mem.alloc(4);
        e.mem.set_f32(bonus, 15.0);
        answer(&mut e, SETTING_VALUE_ADDRESS, bonus);
        answer(&mut e, LIST_BOX_IS_ENABLED, 1);
        answer(&mut e, LIST_BOX_CONTAINS, 1);
        answer(&mut e, WORD_AT_FOUR, 0x900);
        stub(
            &mut e,
            &[
                LIST_BOX_VALUE_OF,
                SET_TRAIT_FROM_INT,
                SET_FLOAT_TRAIT,
                PLAY_MENU_SOUND,
                GET_FLOAT_TRAIT,
                FTOL,
                MAX_OF,
                MIN_OF,
                PRINT_BOUNDED,
                TILE_ADD_USER_TRAIT,
                SET_STRING_TRAIT,
                SETTING_STRING,
                APPEND_BOUNDED,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3ea0, &args![this, 9i32, 0x500u32]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 1);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x500, 0x63, 15.0f32.to_bits(), 1]]
        );
        let ints = calls_to(&e, SET_TRAIT_FROM_INT);
        assert_eq!(ints[0], vec![0x500, 0x65, 1]);
        assert_eq!(ints[1], vec![0x900, 0x64, 1]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![3]]);
    }

    #[test]
    fn do_click_keeps_an_unselected_item_without_points() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.map(IS_SELECTED_TRAIT & !0xfff, 0x1000);
        e.set_global(IS_SELECTED_TRAIT, 0x65u32);
        e.set_global(IS_SELECTED_GUARD, 1u32);
        stub(&mut e, &[LIST_BOX_VALUE_OF, TILE_IS_TRUE]);
        e.call_log = Some(vec![]);
        e.call(0x0075_3ea0, &args![this, 9i32, 0x500u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 0);
    }

    #[test]
    fn do_click_arrows_step_the_points_and_the_added_value() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.set(this, CharGenMenu::iCurrPoints, 2);
        e.map(TRAIT_ADDED_VALUE & !0xfff, 0x1000);
        e.set_global(TRAIT_ADDED_VALUE, 0x63u32);
        answer(&mut e, WORD_AT_0X28, 0x600);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 2.0);
        stub(
            &mut e,
            &[
                LIST_BOX_VALUE_OF,
                WORD_AT_FOUR,
                SET_TRAIT_FROM_INT,
                SET_FLOAT_TRAIT,
                FTOL,
                MAX_OF,
                MIN_OF,
                PRINT_BOUNDED,
                TILE_ADD_USER_TRAIT,
                SET_STRING_TRAIT,
                SETTING_STRING,
                APPEND_BOUNDED,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3ea0, &args![this, 10i32, 0x500u32]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 1);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x600, 0x63, 1.0f32.to_bits(), 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3ea0, &args![this, 11i32, 0x500u32]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 2);
        assert_eq!(
            calls_to(&e, SET_FLOAT_TRAIT),
            vec![vec![0x600, 0x63, 3.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn do_click_reset_walks_the_reset_callback_and_plays_a_sound() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.set(this, CharGenMenu::iCurrPoints, 2);
        e.set(this, CharGenMenu::eItemType, 1);
        e.map(TRAIT_CURRENT_POINTS & !0xfff, 0x1000);
        stub(
            &mut e,
            &[
                LIST_BOX_FOR_EACH,
                WORD_AT_FOUR,
                SET_TRAIT_FROM_INT,
                PLAY_MENU_SOUND,
                SETTING_STRING,
                PRINT_BOUNDED,
                APPEND_BOUNDED,
                SET_STRING_TRAIT,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_3ea0, &args![this, 5i32, 0x500u32]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 0);
        assert_eq!(
            calls_to(&e, LIST_BOX_FOR_EACH),
            vec![vec![
                this.addr() + 0x58,
                RESET_SKILLS_CALLBACK,
                0,
                0x7fff_ffff
            ]]
        );
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![1]]);
    }

    #[test]
    fn tag_skills_func_restarts_on_the_first_item_and_counts_tags() {
        let mut e = Engine::new();
        e.map(TAG_SKILL_INDEX & !0xfff, 0x1000);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(PLAYER & !0xfff, 0x1000);
        e.set_global(TAG_SKILL_INDEX, 3u32);
        e.set_global(TRAIT_TAGGED, 0x62u32);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 0.0);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, ACTOR_GET_CLASS, 0x3000);
        stub(&mut e, &[CLASS_SET_TAG_SKILL]);
        e.call_log = Some(vec![]);
        e.call(0x0075_4530, &args![0x500u32, 0x2au32]);
        assert_eq!(
            calls_to(&e, CLASS_SET_TAG_SKILL),
            vec![vec![0x3000, 0, 0x2a]]
        );
        assert_eq!(e.global::<u32>(TAG_SKILL_INDEX), 1);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 4.0);
        answer(&mut e, TILE_IS_TRUE, 0);
        e.call(0x0075_4530, &args![0x500u32, 0x2bu32]);
        assert_eq!(e.global::<u32>(TAG_SKILL_INDEX), 1);
    }

    #[test]
    fn set_display_string_clamps_attributes_and_skills() {
        let mut e = Engine::new();
        let this = e.new_object::<CharGenMenu>();
        e.map(TRAIT_BASE_VALUE & !0xfff, 0x1000);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 4.0);
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        answer(&mut e, MAX_OF, 12);
        answer(&mut e, MIN_OF, 10);
        answer(&mut e, TILE_ADD_USER_TRAIT, 0x77);
        stub(&mut e, &[PRINT_BOUNDED, SET_STRING_TRAIT]);
        e.call_log = Some(vec![]);
        e.call(0x0075_45a0, &args![this, 0x500u32]);
        assert_eq!(calls_to(&e, MAX_OF), vec![vec![12, 1]]);
        assert_eq!(calls_to(&e, MIN_OF), vec![vec![12, 10]]);
        let print = calls_to(&e, PRINT_BOUNDED)[0].clone();
        assert_eq!(&print[1..], &[0x10, DISPLAY_FORMAT, 10]);
        let set = calls_to(&e, SET_STRING_TRAIT)[0].clone();
        assert_eq!(set[..2], [0x500, 0x77]);
        assert_eq!(set[2], print[0]);
        e.set(this, CharGenMenu::eItemType, 1);
        e.call_log = Some(vec![]);
        e.call(0x0075_45a0, &args![this, 0x500u32]);
        assert_eq!(calls_to(&e, MAX_OF), vec![vec![12, 0]]);
        assert_eq!(calls_to(&e, MIN_OF), vec![vec![12, 100]]);
    }

    #[test]
    fn add_item_puts_an_unsorted_item_at_the_head() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x30);
        e.mem.set_u32(list + 0xc, 0x300);
        e.mem.set_u32(list + 0x18, 0x1800);
        e.mem.set_u16(list + 0x1c, 4);
        answer(&mut e, TILE_GET_MENU, 0x200);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x800);
        answer(&mut e, TILE_GET_VALUE, 0);
        answer(&mut e, OPERATOR_NEW, 0x0090_0000);
        e.register(LIST_ITEM_CONSTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x2a, "the value passed by address");
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x0090_0000);
            Ret::default()
        });
        answer(&mut e, LIST_BOX_FLAG, 1);
        answer(&mut e, LIST_BOX_COUNT, 2);
        stub(
            &mut e,
            &[
                SET_TRAIT_FROM_INT,
                SET_STRING_TRAIT,
                LIST_BOX_INIT_TILE_POSITION,
                LIST_BOX_SET_TOTAL_LIST_HEIGHT,
            ],
        );
        e.call_log = Some(vec![]);
        let tile = e
            .call(0x0075_4690, &args![list, 0x2au32, 0x99u32, 0u32, 0u32])
            .u32();
        assert_eq!(tile, 0x800);
        assert_eq!(
            calls_to(&e, MENU_RENDER_TEMPLATE),
            vec![vec![0x200, 0x300, 0x1800, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x800, 0xfaa, u32::MAX], vec![0x800, 0xfac, 4]]
        );
        assert_eq!(
            calls_to(&e, SET_STRING_TRAIT),
            vec![vec![0x800, 0xfc4, 0x99, 1]]
        );
        assert_eq!(e.mem.u16(list + 0x1c), 5);
        assert_eq!(
            calls_to(&e, LIST_BOX_INIT_TILE_POSITION),
            vec![vec![list, 0x800]]
        );
    }

    #[test]
    fn add_item_sorts_and_rebuilds_the_scroll_value_of_a_single_item() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x30);
        e.mem.set_u32(list + 0xc, 0x300);
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        answer(&mut e, TILE_GET_MENU, 0x200);
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x800);
        answer(&mut e, TILE_GET_VALUE, 3);
        answer(&mut e, OPERATOR_NEW, 0);
        answer(&mut e, LIST_BOX_FLAG, 0);
        answer(&mut e, LIST_BOX_COUNT, 1);
        answer(&mut e, TILE_ADD_USER_TRAIT, 0x44);
        answer_st0(&mut e, GET_FLOAT_TRAIT, 2.5);
        answer(&mut e, TILE_GET_VALUE_Q, 0x4400);
        stub(
            &mut e,
            &[
                LIST_INSERT_SORTED,
                TILE_VALUE_CLEAR_ACTIONS,
                TILE_VALUE_ADD_ACTION_TILE,
                TILE_VALUE_ADD_ACTION_FLOAT,
                TILE_VALUE_CALCULATE,
            ],
        );
        e.call_log = Some(vec![]);
        let tile = e
            .call(0x0075_4690, &args![list, 1u32, 0u32, 0x5555u32, 0x1900u32])
            .u32();
        assert_eq!(tile, 0x800);
        assert!(calls_to(&e, LIST_ITEM_CONSTRUCT).is_empty(), "no block");
        assert_eq!(
            calls_to(&e, LIST_INSERT_SORTED),
            vec![vec![list + 4, 0, 0x5555]]
        );
        assert_eq!(
            calls_to(&e, TILE_VALUE_ADD_ACTION_TILE),
            vec![vec![0x4400, 0x7d0, 0x800, 0xfb0]]
        );
        assert_eq!(
            calls_to(&e, TILE_VALUE_ADD_ACTION_FLOAT),
            vec![vec![0x4400, 0x7d3, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, TILE_VALUE_CALCULATE), vec![vec![0x4400, 0]]);
    }

    #[test]
    fn add_item_needs_a_container_and_a_template() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0075_4690, &args![list, 1u32, 0u32, 0u32, 0x1900u32])
                .u32(),
            0
        );
        e.mem.set_u32(list + 0xc, 0x300);
        assert_eq!(
            e.call(0x0075_4690, &args![list, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn is_selected_initializes_its_trait_once() {
        let mut e = Engine::new();
        e.map(IS_SELECTED_TRAIT & !0xfff, 0x1000);
        answer(&mut e, TILE_ADD_USER_TRAIT, 0x70);
        answer(&mut e, TILE_IS_TRUE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4950, &args![0x100u32, 0x500u32]).bool());
        assert!(!e.call(0x0075_4950, &args![0x100u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, TILE_ADD_USER_TRAIT),
            vec![vec![SELECTED_TRAIT_NAME, u32::MAX]]
        );
        assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x500, 0x70]]);
    }

    #[test]
    fn toggle_and_set_selection_modes() {
        let mut e = Engine::new();
        e.map(TOGGLE_TRAIT & !0xfff, 0x1000);
        answer(&mut e, TILE_ADD_USER_TRAIT, 0x71);
        answer(&mut e, LIST_BOX_IS_ENABLED, 1);
        answer(&mut e, LIST_BOX_CONTAINS, 1);
        answer(&mut e, TILE_IS_TRUE, 1);
        stub(&mut e, &[SET_TRAIT_FROM_INT]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0075_4a00, &args![0x100u32, 0x500u32, 0i32]).bool());
        assert!(e.call(0x0075_4a00, &args![0x100u32, 0x500u32, 1i32]).bool());
        e.call(0x0075_4930, &args![0x100u32, 0x500u32]);
        assert!(!e.call(0x0075_4a00, &args![0x100u32, 0x500u32, 3i32]).bool());
        assert_eq!(
            calls_to(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x500, 0x71, 0], vec![0x500, 0x71, 0]]
        );
        assert_eq!(calls_to(&e, TILE_ADD_USER_TRAIT).len(), 1);
        assert!(!e.call(0x0075_4a00, &args![0x100u32, 0u32, 2i32]).bool());
    }
}

#[cfg(test)]
mod n052_tests {
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

    /// A menu as the singleton, the trait-id statics mapped and numbered,
    /// and a player whose object at +0xa4 has a vtable.
    fn setup(e: &mut Engine) -> (Ptr<CharGenMenu>, u32, u32) {
        let this = e.new_object::<CharGenMenu>();
        e.map(CHAR_GEN_MENU & !0xfff, 0x1000);
        e.set_global(CHAR_GEN_MENU, this.addr());
        e.set_global(TRAIT_BASE_VALUE, 0x61u32);
        e.set_global(TRAIT_SECOND_VALUE, 0x62u32);
        e.set_global(TRAIT_TAGGED, 0x63u32);
        e.set_global(TRAIT_ADDED_VALUE, 0x64u32);
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let player_vtable = e.mem.alloc(0x400);
        e.mem.set_u32(player, player_vtable);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        (this, player, owner_vtable)
    }

    /// Float traits by trait id (0x61 base 5.0, 0x62 second 2.0, 0x64
    /// added 3.0); the display string's callees answer zero.
    fn traits(e: &mut Engine) {
        e.register(GET_FLOAT_TRAIT, |_, a| Ret {
            st0: match a[1] {
                0x61 => 5.0,
                0x62 => 2.0,
                0x64 => 3.0,
                _ => 0.0,
            },
            ..Ret::default()
        });
        stub(
            e,
            &[
                SET_FLOAT_TRAIT,
                SET_TRAIT_FROM_INT,
                FTOL,
                MAX_OF,
                MIN_OF,
                PRINT_BOUNDED,
                TILE_ADD_USER_TRAIT,
                SET_STRING_TRAIT,
            ],
        );
    }

    #[test]
    fn init_attribute_list_adds_section_0_and_counts_points() {
        let mut e = Engine::new();
        let (this, _, owner_vtable) = setup(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0x34, 0x0075_3340);
        e.mem.set_u32(this.addr() + 0x30, 0x330);
        // The list box: a container and a template so AddItem renders.
        e.mem.set_u32(this.addr() + ITEM_LIST + 0xc, 0x440);
        e.mem.set_u32(this.addr() + ITEM_LIST + 0x18, 0x441);
        e.set(this, CharGenMenu::iMaxPoints, 40);
        e.map(0x0107_1000, 0x1000);
        e.set_global(ATTRIBUTE_LIST_TRAIT_VALUE, 105.0f32.to_bits());
        e.mem.set_u32(owner_vtable + 0x1c, 0x00c0_001c);
        e.register(0x00c0_001c, |_, a| Ret {
            eax: a[1] - 0x10,
            ..Ret::default()
        });
        answer(&mut e, ACTOR_VALUE_SECTION_COUNT, 2);
        e.register(ACTOR_VALUE_FROM_SECTION, |_, a| Ret {
            eax: 0x15 + a[1],
            ..Ret::default()
        });
        answer(&mut e, MENU_RENDER_TEMPLATE, 0x777);
        answer(&mut e, LIST_BOX_COUNT, 2);
        answer(&mut e, OPERATOR_NEW, 0);
        traits(&mut e);
        stub(
            &mut e,
            &[
                LIST_BOX_INITIALIZE,
                INTERFACE_SINGLETON,
                SHOW_TUTORIAL,
                ACTOR_VALUE_NAME,
                TILE_GET_MENU,
                TILE_GET_VALUE,
                LIST_BOX_FLAG,
                LIST_ADD_HEAD,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0075_39a0, &args![this]);
        assert_eq!(e.get(this, CharGenMenu::iMaxPoints), 33);
        // Values 5 and 6: (5 - 1) + (6 - 1).
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 9);
        assert_eq!(
            calls_to(&e, LIST_BOX_INITIALIZE),
            vec![vec![this.addr() + 0x58, 0x330, ATTRIBUTE_TEMPLATE_NAME]]
        );
        assert_eq!(
            calls_to(&e, SHOW_TUTORIAL),
            vec![vec![0x4d4, 0xc, 0x418, 0x1f4]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_VALUE_FROM_SECTION),
            vec![vec![0, 0], vec![0, 1]]
        );
        let ints = calls_to(&e, SET_TRAIT_FROM_INT);
        assert!(ints.contains(&vec![0x777, 0xfaa, 8]));
        assert!(ints.contains(&vec![0x777, 0x61, 5]));
        assert!(ints.contains(&vec![0x777, 0x61, 6]));
        assert!(calls_to(&e, SET_FLOAT_TRAIT).contains(&vec![0x330, 0xfa2, 105.0f32.to_bits(), 1]));

        // Only adding values: the maximum and the points stay.
        let (this, _, owner_vtable) = setup(&mut e);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(this.addr() + ITEM_LIST + 0xc, 0x440);
        e.mem.set_u32(this.addr() + ITEM_LIST + 0x18, 0x441);
        e.mem.set_u32(owner_vtable + 0x1c, 0x00c0_001c);
        e.set(this, CharGenMenu::iMaxPoints, 40);
        e.set(this, CharGenMenu::iCurrPoints, 12);
        e.set(this, CharGenMenu::bOnlyAddValues, 1);
        e.call(0x0075_39a0, &args![this]);
        assert_eq!(e.get(this, CharGenMenu::iMaxPoints), 40);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 0);
    }

    #[test]
    fn do_gamepad_clicks_by_button_and_item_type() {
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0xc, 0x00c0_000c);
        e.mem.set_u32(vtable + 0x38, 0x0075_4150);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x58, list_vtable);
        e.mem.set_u32(list_vtable + 4, 0x00c0_0004);
        answer(&mut e, 0x00c0_0004, 0x900);
        e.register(TILE_GET_CHILD_BY_ID, |_, a| Ret {
            eax: 0x1000 + a[1],
            ..Ret::default()
        });
        answer(&mut e, TILE_IS_TRUE, 1);
        stub(&mut e, &[TILE_PLAY_TILE_SOUND, 0x00c0_000c]);
        let one = 1.0f32.to_bits();

        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4150, &args![this, 3i32, one]).bool());
        assert_eq!(calls_to(&e, TILE_GET_CHILD_BY_ID), vec![vec![0x900, 11]]);
        assert_eq!(
            calls_to(&e, TILE_PLAY_TILE_SOUND),
            vec![vec![0x100b, 0xfcb]]
        );
        assert_eq!(
            calls_to(&e, 0x00c0_000c),
            vec![vec![this.addr(), 11, 0x100b]]
        );

        // Button 10 on attributes goes through button 4 (id 10).
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4150, &args![this, 10i32, one]).bool());
        assert_eq!(
            calls_to(&e, 0x00c0_000c),
            vec![vec![this.addr(), 10, 0x100a]]
        );

        // An invisible child: handled, nothing clicked.
        answer(&mut e, TILE_IS_TRUE, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4150, &args![this, 4i32, one]).bool());
        assert!(calls_to(&e, 0x00c0_000c).is_empty());

        // Skills: button 9 clicks the highlighted tile; 3 and 10 do nothing.
        e.set(this, CharGenMenu::eItemType, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_4150, &args![this, 9i32, one]).bool());
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![this.addr(), 9, 0x900]]);
        assert!(!e.call(0x0075_4150, &args![this, 3i32, one]).bool());
        assert!(!e.call(0x0075_4150, &args![this, 10i32, one]).bool());
        // No highlighted tile: 9 is not handled.
        answer(&mut e, 0x00c0_0004, 0);
        assert!(!e.call(0x0075_4150, &args![this, 9i32, one]).bool());
        // Buttons outside the switch.
        assert!(!e.call(0x0075_4150, &args![this, 5i32, one]).bool());
        assert!(!e.call(0x0075_4150, &args![this, 2i32, one]).bool());
        assert!(!e.call(0x0075_4150, &args![this, 11i32, one]).bool());
    }

    #[test]
    fn do_gamepad_up_event_closes_on_button_11() {
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(vtable + 0xc, 0x00c0_000c);
        e.mem.set_u32(this.addr() + 0x44, 0x444);
        answer(&mut e, TILE_IS_TRUE, 1);
        stub(&mut e, &[PLAY_MENU_SOUND, 0x00c0_000c]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0075_42d0, &args![this, 11i32, 0u32]).bool());
        assert_eq!(calls_to(&e, TILE_IS_TRUE), vec![vec![0x444, 0xfaf]]);
        assert_eq!(calls_to(&e, PLAY_MENU_SOUND), vec![vec![1]]);
        assert_eq!(calls_to(&e, 0x00c0_000c), vec![vec![this.addr(), 6, 0]]);
        assert!(!e.call(0x0075_42d0, &args![this, 10i32, 0u32]).bool());
        answer(&mut e, TILE_IS_TRUE, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0075_42d0, &args![this, 11i32, 0u32]).bool());
        assert!(calls_to(&e, PLAY_MENU_SOUND).is_empty());
    }

    #[test]
    fn reset_attributes_counts_the_value_less_one() {
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE_DOUBLE, 1.0f64);
        traits(&mut e);
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.set(this, CharGenMenu::iCurrPoints, 10);
        e.call_log = Some(vec![]);
        e.call(0x0075_4330, &args![0x500u32, 0x20u32]);
        // 5.0 + 2.0 - 1.0 = 6.
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 16);
        assert!(calls_to(&e, SET_FLOAT_TRAIT).contains(&vec![0x500, 0x64, 0, 1]));
        assert_eq!(calls_to(&e, TILE_ADD_USER_TRAIT).len(), 1);
        // Only adding values: the points stay.
        e.set(this, CharGenMenu::bOnlyAddValues, 1);
        e.call(0x0075_4330, &args![0x500u32, 0x20u32]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 16);
        // No menu: nothing.
        e.set_global(CHAR_GEN_MENU, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0075_4330, &args![0x500u32, 0x20u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn reset_skills_untags_unless_the_class_tags_it() {
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e);
        traits(&mut e);
        answer(&mut e, TILE_IS_TRUE, 1);
        answer(&mut e, CLASS_TAGS_SKILL, 1);
        stub(&mut e, &[ACTOR_GET_CLASS]);
        e.call_log = Some(vec![]);
        e.call(0x0075_43c0, &args![0x500u32, 0x21u32]);
        assert_eq!(calls_to(&e, SET_TRAIT_FROM_INT), vec![vec![0x500, 0x63, 0]]);
        assert!(calls_to(&e, CLASS_TAGS_SKILL).is_empty());
        // Only adding values and a class skill: kept.
        e.set(this, CharGenMenu::bOnlyAddValues, 1);
        e.call_log = Some(vec![]);
        e.call(0x0075_43c0, &args![0x500u32, 0x21u32]);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).is_empty());
        assert_eq!(calls_to(&e, CLASS_TAGS_SKILL), vec![vec![0, 0x21]]);
        assert_eq!(e.get(this, CharGenMenu::iCurrPoints), 0);
        // Not a class skill: untagged.
        answer(&mut e, CLASS_TAGS_SKILL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0075_43c0, &args![0x500u32, 0x21u32]);
        assert_eq!(calls_to(&e, SET_TRAIT_FROM_INT), vec![vec![0x500, 0x63, 0]]);
        // Not tagged: only the display string.
        answer(&mut e, TILE_IS_TRUE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0075_43c0, &args![0x500u32, 0x21u32]);
        assert!(calls_to(&e, SET_TRAIT_FROM_INT).is_empty());
        assert_eq!(calls_to(&e, TILE_ADD_USER_TRAIT).len(), 1);
    }

    #[test]
    fn apply_values_writes_the_sum_to_the_player() {
        let mut e = Engine::new();
        let (this, player, owner_vtable) = setup(&mut e);
        traits(&mut e);
        stub(&mut e, &[PLAYER_SET_VALUE_0093A850, 0x00c0_0394]);
        e.call_log = Some(vec![]);
        e.call(0x0075_4460, &args![0x500u32, 0x05u32]);
        assert_eq!(
            calls_to(&e, PLAYER_SET_VALUE_0093A850),
            vec![vec![player, 5, 8.0f32.to_bits()]]
        );
        // Only adding attribute values: current value 7 + added 3.0.
        e.set(this, CharGenMenu::bOnlyAddValues, 1);
        e.mem.set_u32(owner_vtable, 0x00c0_0000);
        answer(&mut e, 0x00c0_0000, 7);
        let player_vtable = e.mem.u32(player);
        e.mem.set_u32(player_vtable + 0x394, 0x00c0_0394);
        e.call_log = Some(vec![]);
        e.call(0x0075_4460, &args![0x500u32, 0x05u32]);
        assert_eq!(calls_to(&e, 0x00c0_0000), vec![vec![player + 0xa4, 5]]);
        assert_eq!(
            calls_to(&e, 0x00c0_0394),
            vec![vec![player, 5, 10.0f32.to_bits()]]
        );
        assert!(calls_to(&e, PLAYER_SET_VALUE_0093A850).is_empty());
        // Skills: the base path again.
        e.set(this, CharGenMenu::eItemType, 1);
        e.call_log = Some(vec![]);
        e.call(0x0075_4460, &args![0x500u32, 0x05u32]);
        assert_eq!(calls_to(&e, PLAYER_SET_VALUE_0093A850).len(), 1);
    }
}
