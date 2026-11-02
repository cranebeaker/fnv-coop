//! `fallout/interface/menumanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `MenuManager` (an empty class whose single instance sits
//! in the global `011d8d54`), its menu factory `CreateMenuByClass`, and the
//! constructors, destructors, `GetClass` and `AttachTileByID` methods of
//! the menus the factory makes that the compiler emitted here (tutorial,
//! stats, text edit, surgery, VATS, S.P.E.C.I.A.L. book, love tester and
//! quantity menus), followed by the `ListBox` template instances.
//!
//! Notes that apply to every function:
//! - The menu classes belong to other units (`statsmenu.cpp`, ...); their
//!   fields are written at the PC offsets the code uses, with the Xbox PDB
//!   field name in a comment where the offset matches the PDB. A class's
//!   identity comes from the RTTI type descriptor of the vtable the code
//!   stores (`.?AVStatsMenu@@`, ...).
//! - The menu constructors and destructors call the `ListBox` and
//!   `BSSimpleArray` constructors and destructors of this file through
//!   their exe address (they run the translations registered here); the
//!   factory calls every menu constructor through its address, as one
//!   table. The shared `ListBox` bodies call the instance's own methods
//!   (`Synchronize`, `IsEnabled`, `UpdateHighlightBoxPos`,
//!   `SetTotalListHeight`, `NumUnfiltered`) and the flag and tile checks
//!   (`fn_007242c0`, `fn_007242f0`) through their exe address too (they run
//!   the translations registered here), so tests can stand in for them.
//! - The six `ListBox<T>` instances (`ActorValue::Index`, `PerkRankData *`,
//!   `MiscStatManager::MiscStatID`, `StatsMenu::StatusEffect *`,
//!   `TESReputation *`, `unsigned int`) have the same code apart from the
//!   vtable, the instance's own methods they call and the function-local
//!   statics holding trait ids; one body each (`list_box_*`) serves all
//!   six, with a `ListBoxInstance` naming those addresses.
//! - The compiler's exception-unwinding frames (`FS:[0]`) and the security
//!   cookie the prologues mix into them are not translated; the functions
//!   that have them carry a `Translation incomplete` marker.
//! - `float` stores of `FLDZ` are `0.0`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::run_007e6320::TextEditMenu;
use crate::units::fallout_interface::startmenu::ListBox;
use crate::units::fallout_interface::tutorialmenu::TutorialMenu;
use crate::units::fallout_shared::bgsdestructibleobjectform::x87_truncate_i64;

/// The allocator, cdecl (size): the block or null.
const ALLOCATE: u32 = 0x0040_1000;
/// The allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// `memset`, cdecl (destination, byte, count).
const MEMSET: u32 = 0x0040_3d30;
/// The compiler's array constructor helper, stdcall (array, element size,
/// count, element constructor, element destructor).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// The compiler's array destructor helper, stdcall (array, element size,
/// count, element destructor); engine map: `eh vector destructor iterator`.
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// Debug message, cdecl (format, ...).
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `"MENUS: Unknown menu class!"`.
const UNKNOWN_MENU_CLASS_TEXT: u32 = 0x0106_ff64;
/// The `Menu` base constructor, thiscall.
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
/// The `Menu` base destructor, thiscall.
const MENU_DESTRUCT: u32 = 0x00a1_c520;

/// `MenuManager`'s single instance (`MenuManager::pInstance`, Xbox PDB).
const MENU_MANAGER: u32 = 0x011d_8d54;
/// The object the `MenuManager` constructor body grows to `0x54` entries.
const MENU_TABLE: u32 = 0x011f_3508;
/// Thiscall on `MENU_TABLE` (count).
const MENU_TABLE_GROW: u32 = 0x0096_ad30;
/// The number the constructor body passes: `0x54`, as many as the Xbox
/// PDB's `Menu::pMenusVisible` (`bool[84]`) has entries.
const MENU_TABLE_COUNT: u32 = 0x54;

/// Vtables (class from the RTTI type descriptor before each).
const TUTORIAL_MENU_VTABLE: u32 = 0x0106_ff84;
const STATS_MENU_VTABLE: u32 = 0x0106_ffd4;
const NAMED_EFFECTS_LIST_VTABLE: u32 = 0x0107_0020;
const STATUS_EFFECT_LIST_VTABLE: u32 = 0x0107_0028;
const TEXT_EDIT_MENU_VTABLE: u32 = 0x0107_0034;
const SURGERY_MENU_VTABLE: u32 = 0x0107_0084;
const VATS_MENU_VTABLE: u32 = 0x0107_00d4;
const SPECIAL_BOOK_MENU_VTABLE: u32 = 0x0107_0124;
const LOVE_TESTER_MENU_VTABLE: u32 = 0x0107_0174;
const QUANTITY_MENU_VTABLE: u32 = 0x0107_01c4;

/// The tutorial menu global the tutorial menu destructor clears
/// (`TutorialMenu::pMe`, see `tutorialmenu.rs`).
const TUTORIAL_MENU_GLOBAL: u32 = 0x011d_b088;
/// The global the stats menu destructor clears.
const STATS_MENU_GLOBAL: u32 = 0x011d_ace0;
/// The object `fn_0071f8c0` deletes (six 8-byte lists from +4, as the Xbox
/// PDB's `StatsMenu::BodyPartController::listFlashingTiles`).
const BODY_PART_CONTROLLER_GLOBAL: u32 = 0x011d_ace4;
/// The string the text edit menu constructor empties (the text edit
/// menu's entered text, see `run_007e6320.rs`).
const ENTERED_TEXT: u32 = 0x011d_aed8;
/// The three smart pointer globals the surgery menu constructor and
/// destructor set to null, in the order the code does.
const SURGERY_STATIC_POINTERS: [u32; 3] = [0x011d_adb4, 0x011d_ade8, 0x011d_ae5c];

/// Thiscall (flags) on the item a healing option holds (+8).
const ITEM_DELETE: u32 = 0x0044_59e0;
/// List constructor, thiscall (the list at +4 of the effect lists).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// List destructor, thiscall.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// List clear, thiscall.
const LIST_CLEAR: u32 = 0x0047_0470;
/// Thiscall on a `StatsMenu::NamedEffectsList` (statsmenu.cpp): empties it.
const NAMED_EFFECTS_CLEAR: u32 = 0x007e_09b0;
/// Thiscall on a `StatsMenu::StatusEffectList` (statsmenu.cpp): empties it.
const STATUS_EFFECTS_CLEAR: u32 = 0x007e_07f0;
/// `StatsMenu::StatusDataList::Flush` (engine map), thiscall.
const STATUS_DATA_FLUSH: u32 = 0x0040_6de0;
/// Thiscall: constructs the text edit menu's editor (+0x34).
const TEXT_EDITOR_CONSTRUCT: u32 = 0x0071_6980;
/// Thiscall: destroys the text edit menu's editor.
const TEXT_EDITOR_DESTRUCT: u32 = 0x0071_6a10;
/// String set, thiscall (text, length).
const STRING_SET: u32 = 0x0040_37f0;
/// Thiscall: returns `this` and writes nothing (an empty constructor; the
/// list box code calls it on a list node to reach the node's item word).
const RETURN_THIS: u32 = 0x0068_15c0;
/// Sound handle constructor, thiscall.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// An empty function (`RET`) the surgery menu destructor calls on its two
/// sound handles.
const EMPTY_FUNCTION: u32 = 0x0048_3710;
/// Smart pointer default constructor, thiscall.
const POINTER_CONSTRUCT_DEFAULT: u32 = 0x0066_94e0;
/// Smart pointer constructor from a pointer, thiscall (pointer).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// Smart pointer assignment, thiscall (pointer).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Smart pointer destructor, thiscall.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Thiscall: constructs the VATS menu's action list box (+0xB0; the
/// `ListBox<unsigned int>` constructor of this unit, `fn_00723750`).
const VATS_ACTION_LIST_CONSTRUCT: u32 = 0x0072_3750;
/// Thiscall: constructs the VATS menu's command at +0x11C.
const VATS_COMMAND_CONSTRUCT: u32 = 0x009c_a4a0;
/// The VATS menu destructor (fallout/interface), thiscall.
const VATS_MENU_DESTRUCT: u32 = 0x007e_9130;

/// The stats menu's list box constructors and destructors (this unit,
/// `fn_00721060` and the like), by the offset of the list box.
const STATS_LIST_BOX_CONSTRUCTORS: [(u32, u32); 6] = [
    (0x180, 0x0072_1060),
    (0x1b0, 0x0072_1060),
    (0x1e0, 0x0072_1850),
    (0x210, 0x0072_2000),
    (0x240, 0x0072_27b0),
    (0x270, 0x0072_2f60),
];
const STATS_LIST_BOX_DESTRUCTORS: [(u32, u32); 6] = [
    (0x270, 0x0072_3070),
    (0x240, 0x0072_2880),
    (0x210, 0x0072_20d0),
    (0x1e0, 0x0072_1920),
    (0x1b0, 0x0072_1150),
    (0x180, 0x0072_1150),
];

/// Constructors and destructors of the page arrays' `BSSimpleArray`
/// members (this unit, `fn_00723f00` and the like), thiscall.
const BOOK_BUTTONS_CONSTRUCT: u32 = 0x0072_3f50;
const BOOK_TEXT_ITEMS_CONSTRUCT: u32 = 0x0072_3f00;
const BOOK_TEXT_ITEMS_DESTRUCT: u32 = 0x0072_3f30;
const BOOK_BUTTONS_DESTRUCT: u32 = 0x0072_3f80;
const LOVE_TESTER_BUTTONS_CONSTRUCT: u32 = 0x0072_4040;
const LOVE_TESTER_TEXT_ITEMS_CONSTRUCT: u32 = 0x0072_3fa0;
const LOVE_TESTER_TEXT_ITEMS_DESTRUCT: u32 = 0x0072_3fd0;
const LOVE_TESTER_BUTTONS_DESTRUCT: u32 = 0x0072_4070;
const LOVE_TESTER_GLOWS_CONSTRUCT: u32 = 0x0072_3ff0;
const LOVE_TESTER_GLOWS_DESTRUCT: u32 = 0x0072_4020;

/// The global the `QuantityMenu` destructor clears.
const QUANTITY_MENU_GLOBAL: u32 = 0x011d_a618;

/// `Tile::GetMenuByClass` (engine map), cdecl (menu class): the open menu
/// of that class, or null.
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// Thiscall on a tile (trait id): a float in `ST0`.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::GetMenu` (engine map), thiscall on a tile.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::IsTrue` (engine map), thiscall on a tile (trait id): `AL`.
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// Thiscall on a tile (trait id, float, word); the list box code passes
/// the `"_num_filtered"` trait, `0.0` and 1.
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::TextToTrait` (engine map), cdecl (name): the trait id.
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::AddUserTrait` (engine map), cdecl (name, word).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `"_num_filtered"`.
const NUM_FILTERED_TEXT: u32 = 0x0106_f8d0;
/// `"_scroll_delta"`.
const SCROLL_DELTA_TEXT: u32 = 0x0106_f90c;
/// `"_enabled"`.
const ENABLED_TEXT: u32 = 0x0107_0244;
/// The compiler's float to integer conversion (`_ftol2`): the value in
/// `ST0` (a leading `f64`), the integer in `EAX`.
const FTOL2: u32 = 0x00ec_62c0;
/// `__RTDynamicCast`, cdecl (object, vftable offset, source type, target
/// type, is reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `ReloadMenu` casts with (`.?AVTile@@`,
/// `.?AVContainerMenu@@`, `.?AVBookMenu@@`, `.?AVTESBoundObject@@`,
/// `.?AVTESObjectBOOK@@`).
const TILE_TYPE: u32 = 0x0119_f4cc;
const CONTAINER_MENU_TYPE: u32 = 0x0119_f758;
const BOOK_MENU_TYPE: u32 = 0x0119_f400;
const BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
const BOOK_TYPE: u32 = 0x0118_a650;

/// `ReloadMenu`'s callees, all cdecl: a flag it reads first (unnamed;
/// its body tests the global `011daac0` and `004a4080`) and the start menu
/// class (`0070ede0` returns `0x3f5`).
const RELOAD_FLAG: u32 = 0x004a_4040;
const START_MENU_CLASS: u32 = 0x0070_ede0;
/// Unnamed; its body creates the map menu through `MapMenu::Create`.
const CREATE_MAP_MENU: u32 = 0x0070_4140;
/// `Interface::CreateRepairMenu` (engine map).
const CREATE_REPAIR_MENU: u32 = 0x0070_4490;
/// `Interface::CreateItemModMenu` (engine map).
const CREATE_ITEM_MOD_MENU: u32 = 0x0070_46c0;
/// Unnamed creators `ReloadMenu` calls for classes `0x3ea` and `0x3eb`.
const CREATE_MENU_3EA: u32 = 0x0070_48c0;
const CREATE_MENU_3EB: u32 = 0x0070_4be0;
/// `HUDMainMenu::Create` (engine map).
const HUD_MAIN_MENU_CREATE: u32 = 0x0076_bfe0;
/// `StartMenu::Create` (engine map), cdecl (flag, word): `AL`.
const START_MENU_CREATE: u32 = 0x007c_b7d0;
/// `LoadingMenu::Create` (engine map), cdecl (three words).
const LOADING_MENU_CREATE: u32 = 0x0078_8cc0;
/// `LevelUpMenu::Create` (engine map).
const LEVEL_UP_MENU_CREATE: u32 = 0x0078_4c80;
/// `ContainerMenu::Create` (engine map), cdecl (two words).
const CONTAINER_MENU_CREATE: u32 = 0x0075_b310;
/// `BookMenu::Create` (engine map), cdecl (two words).
const BOOK_MENU_CREATE: u32 = 0x0073_ad50;
/// Thiscall getters (unnamed): the word at +0x74, +0x30 and +0x20.
const WORD_AT_74: u32 = 0x004f_d3a0;
const WORD_AT_30: u32 = 0x0067_1d10;
const WORD_AT_20: u32 = 0x007a_f430;
/// `InterfaceManager::NewTimer` (engine map), cdecl (menu, float).
const NEW_TIMER: u32 = 0x0071_64c0;
/// `BGSMenuPacker::RecomputePacking` (engine map), thiscall (word).
const RECOMPUTE_PACKING: u32 = 0x0070_37c0;
/// Thiscall getter (unnamed; the word at +0x24) the open menu checks call
/// on a menu.
const WORD_AT_24: u32 = 0x0059_bb30;

/// The tile list `GetFrontmostMenu` and `IsMenuOpen` walk: `004b7210`
/// returns the global `011d8a80`, `00586150` (thiscall) the word at +0x9C
/// of that, and the list sits at +4 of the result.
const LIST_OWNER: u32 = 0x004b_7210;
const LIST_HOLDER: u32 = 0x0058_6150;
/// Thiscall on the list: its first node (the word at +0).
const LIST_HEAD: u32 = 0x0055_9450;
/// Thiscall on the list (node): the node's data slot (`node + 8`).
const LIST_NODE_DATA: u32 = 0x0063_17a0;
/// Thiscall on the list (node): the next node (the node's word at +0), or
/// null for a null node.
const LIST_NEXT: u32 = 0x007b_52d0;
/// Thiscall on the list (address of a node variable): advances the
/// variable to the next node and returns the old node's data slot.
const LIST_ADVANCE: u32 = 0x0057_cbe0;
/// The float `GetFrontmostMenu` starts its search with (`-1e8`).
const FRONTMOST_START_VALUE: u32 = 0x0107_0218;
/// The double a menu's trait `0xfa7` must not equal for
/// `GetFrontmostMenu` (`6008.0`).
const FRONTMOST_EXCLUDED_VALUE: u32 = 0x0107_0210;

/// `ListBoxBase`'s vtable (RTTI `.?AVListBoxBase@@`).
const LIST_BOX_BASE_VTABLE: u32 = 0x0107_0418;
/// The object at `011d8b54` every list box constructor passes itself to
/// (`005ae3d0`, by the address of its argument) and every destructor
/// again (`00905330`), thiscall.
const LIST_BOX_COLLECTION: u32 = 0x011d_8b54;
const LIST_BOX_COLLECTION_ADD: u32 = 0x005a_e3d0;
const LIST_BOX_COLLECTION_REMOVE: u32 = 0x0090_5330;
/// Thiscall (two words) the list box constructor calls with 0 and 1.
const LIST_BOX_INIT_FLAGS: u32 = 0x007a_1950;
/// Thiscall (bit number): whether that bit of `usFlags` (+0x2C) is set
/// (this unit, `fn_007242c0`).
const LIST_BOX_HAS_FLAG: u32 = 0x0072_42c0;
/// Thiscall (tile): `AL`, whether an item of the list has that tile (this
/// unit, `fn_007242f0`).
const LIST_BOX_BELONGS_TO: u32 = 0x0072_42f0;
/// Thiscall (word) on the embedded list (+4).
const LIST_SORT: u32 = 0x0083_fd60;
/// Thiscall on a list node (word): stores the word at +4 (the node's next
/// pointer; the engine map's name for the folded body is
/// `CombatProcedureAttackMelee::Initialize`).
const NODE_SET_NEXT: u32 = 0x006e_cd40;
/// A list node's deleting destructor, thiscall (flags).
const NODE_DELETE: u32 = 0x0047_02f0;
/// A byte flag: while it is set, `RemoveAll` leaves the items' tiles alone.
const KEEP_ITEM_TILES: u32 = 0x011d_8909;
/// Cdecl (float, float): a float in `ST0`; `GetHighlightTarget` passes the
/// scroll step and `1.0`.
const SCROLL_ROUND: u32 = 0x004b_d510;

/// Thiscall on a tile (trait id, integer): stores the integer as a float
/// (`FILD`, then `00a012d0` with the value and 1).
const TILE_SET_INT: u32 = 0x0070_0320;
/// `ListBox<StartMenu::Option *>::InitTilePosition` (engine map; Xbox PDB
/// name, `fallout/interface/menus/startmenu.cpp`), thiscall on a list box
/// (tile).
const INIT_TILE_POSITION: u32 = 0x0072_69d0;
/// Thiscall on a list box: the count of its embedded list (`005ae380` on
/// +4).
const LIST_COUNT: u32 = 0x0072_6c40;
/// `Tile::GetValue` (engine map), thiscall on a tile (trait id): the
/// trait's value object.
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::Value::AddAction_ov2` (engine map), thiscall on a value object
/// (action, tile, trait id).
const VALUE_ADD_ACTION: u32 = 0x00a0_9130;
/// `Tile::Value::CalculateValue` (engine map), thiscall on a value object
/// (word).
const VALUE_CALCULATE: u32 = 0x00a0_9410;
/// Thiscall on a tile: its byte at +0x35, in `AL`.
const TILE_GET_BYTE_35: u32 = 0x0071_aa80;
/// Thiscall on a tile (byte): stores the byte at +0x35.
const TILE_SET_BYTE_35: u32 = 0x0071_aa60;
/// Unnamed (`BSMenu/tile.cpp`), cdecl (tile, name): the tile the code
/// takes as the highlight box, or null.
const TILE_FIND_BY_NAME: u32 = 0x00a0_8b20;
/// `Tile::GetChildByName` (engine map), thiscall on a tile (name).
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
/// `Tile::AddNeedsUpdate` (engine map), thiscall on a tile (flags).
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// Cdecl (float): a float in `ST0` (its body passes the float on to
/// `004e4450`).
const ROUND_FLOAT: u32 = 0x004e_4430;
/// Cdecl (integer, integer): the larger one (signed).
const MAX_INT: u32 = 0x0064_7b70;
/// The `0.0` double `SetTotalListHeight` compares with.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `"_highlight_y"`, `"_selected_height"`, `"_number_of_items"`.
const HIGHLIGHT_Y_TEXT: u32 = 0x0106_f8c0;
const SELECTED_HEIGHT_TEXT: u32 = 0x0106_f8ac;
const NUMBER_OF_ITEMS_TEXT: u32 = 0x0106_f8f8;
/// `"lb_highlight_box"` and the names of its edges `"top"`, `"bottom"`,
/// `"left"`, `"right"`.
const HIGHLIGHT_BOX_TEXT: u32 = 0x0106_f898;
const TOP_TEXT: u32 = 0x0106_f894;
const BOTTOM_TEXT: u32 = 0x0106_f88c;
const LEFT_TEXT: u32 = 0x0106_f884;
const RIGHT_TEXT: u32 = 0x0106_f87c;

/// `BSSimpleArray` set-up and release, thiscall: `006b3eb0` (two words, 0
/// and 0) and `008454f0` (one word, 1).
const SIMPLE_ARRAY_INIT: u32 = 0x006b_3eb0;
const SIMPLE_ARRAY_RELEASE: u32 = 0x0084_54f0;
/// The `BSSimpleArray` vtables (RTTI): `SPECIALBookMenu::SpecialBookTextItem`,
/// `SpecialBookButton`, `LoveTesterMenu::LoveTesterTextItem`,
/// `LoveTesterLightBulbGlows` and `LoveTesterButton` pointers.
const BOOK_TEXT_ITEMS_VTABLE: u32 = 0x0107_03b4;
const BOOK_BUTTONS_VTABLE: u32 = 0x0107_03c8;
const LOVE_TESTER_TEXT_ITEMS_VTABLE: u32 = 0x0107_03dc;
const LOVE_TESTER_GLOWS_VTABLE: u32 = 0x0107_03f0;
const LOVE_TESTER_BUTTONS_VTABLE: u32 = 0x0107_0404;

/// A function-local static trait id: bit `bit` of the word at `guard`
/// says whether the id at `cache` was looked up (`Tile::AddUserTrait(name,
/// -1)`) yet.
#[derive(Clone, Copy)]
struct StaticTrait {
    guard: u32,
    bit: u32,
    cache: u32,
    name: u32,
}

const fn static_trait_of(guard: u32, bit: u32, cache: u32, name: u32) -> StaticTrait {
    StaticTrait {
        guard,
        bit,
        cache,
        name,
    }
}

/// One `ListBox<T>` instance: its vtable, the instance's own methods (this
/// unit; the shared bodies call them through these addresses, all
/// thiscall without arguments) and the function-local statics its
/// methods keep trait ids in.
#[derive(Clone, Copy)]
struct ListBoxInstance {
    vtable: u32,
    /// `Synchronize`.
    synchronize: u32,
    /// `IsEnabled`: `AL`.
    is_enabled: u32,
    /// `UpdateHighlightBoxPos`.
    update_highlight_box_pos: u32,
    /// `SetTotalListHeight`.
    set_total_list_height: u32,
    /// `NumUnfiltered`: `EAX`.
    num_unfiltered: u32,
    /// `IsEnabled`'s `"_enabled"` trait id.
    enabled_trait: StaticTrait,
    /// `UpdateHighlightBoxPos`'s `"_highlight_y"` trait id.
    highlight_y_trait: StaticTrait,
    /// `UpdateHighlightBoxPos`'s `"_selected_height"` trait id.
    selected_height_trait: StaticTrait,
    /// `NumUnfiltered`'s `"_num_filtered"` trait id.
    num_filtered_trait: StaticTrait,
}

/// An instance's table from its addresses: the vtable, the five methods in
/// the order of the fields, the `IsEnabled` guard (its id just below), the
/// `UpdateHighlightBoxPos` guard (bit 1 the `"_highlight_y"` id 4 bytes
/// below, bit 2 the `"_selected_height"` id 8 below) and the
/// `NumUnfiltered` guard (its id just below), as each method's disassembly
/// uses them.
const fn list_box_instance(
    vtable: u32,
    [synchronize, is_enabled, update_highlight_box_pos, set_total_list_height, num_unfiltered]: [u32; 5],
    [enabled_guard, highlight_guard, num_filtered_guard]: [u32; 3],
) -> ListBoxInstance {
    ListBoxInstance {
        vtable,
        synchronize,
        is_enabled,
        update_highlight_box_pos,
        set_total_list_height,
        num_unfiltered,
        enabled_trait: static_trait_of(enabled_guard, 1, enabled_guard - 4, ENABLED_TEXT),
        highlight_y_trait: static_trait_of(
            highlight_guard,
            1,
            highlight_guard - 4,
            HIGHLIGHT_Y_TEXT,
        ),
        selected_height_trait: static_trait_of(
            highlight_guard,
            2,
            highlight_guard - 8,
            SELECTED_HEIGHT_TEXT,
        ),
        num_filtered_trait: static_trait_of(
            num_filtered_guard,
            1,
            num_filtered_guard - 4,
            NUM_FILTERED_TEXT,
        ),
    }
}

/// `ListBox<ActorValue::Index>` (RTTI `.?AV?$ListBox@W4Index@ActorValue@@@@`).
const LIST_BOX_ACTOR_VALUE: ListBoxInstance = list_box_instance(
    0x0107_0220,
    [
        0x0072_4360,
        0x0072_4430,
        0x0072_44e0,
        0x0072_4800,
        0x0072_6660,
    ],
    [0x011d_8dd0, 0x011d_8ddc, 0x011d_8e48],
);
/// `ListBox<PerkRankData *>` (RTTI `.?AV?$ListBox@PAUPerkRankData@@@@`).
const LIST_BOX_PERK_RANK_DATA: ListBoxInstance = list_box_instance(
    0x0107_02ec,
    [
        0x0072_4930,
        0x0072_4a00,
        0x0072_4ab0,
        0x0072_4dd0,
        0x0072_6710,
    ],
    [0x011d_8de4, 0x011d_8df0, 0x011d_8e50],
);
/// `ListBox<MiscStatManager::MiscStatID>` (RTTI
/// `.?AV?$ListBox@W4MiscStatID@MiscStatManager@@@@`).
const LIST_BOX_MISC_STAT_ID: ListBoxInstance = list_box_instance(
    0x0107_0314,
    [
        0x0072_4f00,
        0x0072_4fd0,
        0x0072_5080,
        0x0072_53a0,
        0x0072_67c0,
    ],
    [0x011d_8df8, 0x011d_8e04, 0x011d_8e58],
);
/// `ListBox<StatsMenu::StatusEffect *>` (RTTI
/// `.?AV?$ListBox@PAUStatusEffect@StatsMenu@@@@`).
const LIST_BOX_STATUS_EFFECT: ListBoxInstance = list_box_instance(
    0x0107_033c,
    [
        0x0072_54d0,
        0x0072_55a0,
        0x0072_5650,
        0x0072_5970,
        0x0072_6870,
    ],
    [0x011d_8e0c, 0x011d_8e18, 0x011d_8e60],
);
/// `ListBox<TESReputation *>` (RTTI `.?AV?$ListBox@PAVTESReputation@@@@`).
const LIST_BOX_TES_REPUTATION: ListBoxInstance = list_box_instance(
    0x0107_0364,
    [
        0x0072_5aa0,
        0x0072_5b70,
        0x0072_5c20,
        0x0072_5f40,
        0x0072_6920,
    ],
    [0x011d_8e20, 0x011d_8e2c, 0x011d_8e68],
);
/// `ListBox<unsigned int>` (RTTI `.?AV?$ListBox@I@@`).
const LIST_BOX_UNSIGNED_INT: ListBoxInstance = list_box_instance(
    0x0107_038c,
    [
        0x0072_6090,
        0x0072_6160,
        0x0072_6210,
        0x0072_6530,
        0x0072_6b40,
    ],
    [0x011d_8e34, 0x011d_8e40, 0x011d_8e70],
);

/// `CreateMenuByClass`'s switch on the menu class (`0x3e9..=0x43c`): the allocation size and
/// the constructor of the classes that have one; a class outside it (or a gap) has none.
fn menu_class(class: u32) -> Option<(u32, u32)> {
    Some(match class {
        0x3e9 => (0x94, 0x007a_8a10),
        0x3ea => (0x124, 0x0077_fa00),
        0x3eb => (0x2a4, 0x0071_f340),
        0x3ec => (0x278, 0x0076_bb40),
        0x3ef => (0x5c0, 0x0078_8440),
        0x3f0 => (0x10c, 0x0075_b0d0),
        0x3f1 => (0x13c, 0x0076_17a0),
        0x3f4 => (0x4c, 0x007b_fb30),
        0x3f5 => (0x1d4, 0x007c_b330),
        0x3f6 => (0xe4, 0x0078_d930),
        0x3f8 => (0x44, 0x0072_0a10),
        0x3ff => (0x230, 0x0079_6740),
        0x402 => (0x3c, 0x0073_acb0),
        0x403 => (0xcc, 0x0078_4a40),
        0x40b => (0x8c, 0x007b_5400),
        0x40c => (0x1a4, 0x007a_c1f0),
        0x417 => (0x80, 0x0075_f070),
        0x418 => (0x88, 0x0075_3230),
        0x41b => (0x5c, 0x0071_fa80),
        0x41d => (0x120, 0x0072_ceb0),
        0x41e => (0x160, 0x0071_fbf0),
        0x41f => (0x1dc, 0x0076_54f0),
        0x420 => (0x144, 0x0072_0040),
        0x421 => (0xfc, 0x0075_77f0),
        0x422 => (0xa0, 0x007b_7400),
        0x423 => (0x54, 0x0071_f240),
        0x424 => (0x1e4, 0x0072_0170),
        0x425 => (0x90, 0x0078_3310),
        0x432 => (0x230, 0x0072_0550),
        0x433 => (0x88, 0x0075_4b40),
        0x434 => (0x88, 0x007e_7a80),
        0x435 => (0x104, 0x0072_6df0),
        0x438 => (0x158, 0x007c_06f0),
        0x439 => (0xe70, 0x0073_15e0),
        0x43a => (0x4014, 0x007b_8d30),
        0x43b => (0xe88, 0x0073_b7e0),
        0x43c => (0x8c, 0x007e_6780),
        _ => return None,
    })
}

/// Writes the vtable pointer at `object + 0`.
fn set_vtable(e: &mut Engine, object: Ptr, vtable: u32) {
    e.mem.set_u32(object.addr(), vtable);
}

fn memset(e: &mut Engine, destination: u32, size: u32) {
    e.call(MEMSET, &args![destination, 0u32, size]);
}

/// `count` smart pointers of 4 bytes at `object + offset`, built with the
/// default constructor and destroyed with the smart pointer destructor.
fn construct_pointer_array(e: &mut Engine, object: Ptr, offset: u32, count: u32) {
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            object.addr() + offset,
            4u32,
            count,
            POINTER_CONSTRUCT_DEFAULT,
            POINTER_DESTRUCT
        ],
    );
}

fn destruct_pointer_array(e: &mut Engine, object: Ptr, offset: u32, count: u32) {
    e.call(
        VECTOR_DESTRUCT,
        &args![object.addr() + offset, 4u32, count, POINTER_DESTRUCT],
    );
}

/// Frees `this` when bit 0 of `flags` is set (the deleting destructors'
/// tail).
fn free_if_flagged(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
}

/// The tile list `GetFrontmostMenu` and `IsMenuOpen` walk, fetched the
/// way the code does each time (`004b7210`, `00586150`, +4).
fn tile_list(e: &mut Engine) -> u32 {
    let owner = e.call(LIST_OWNER, &args![]).u32();
    e.call(LIST_HOLDER, &args![owner]).u32() + 4
}

/// `Tile::IsTrue(tile, Tile::AddUserTrait("_enabled", -1))`.
fn tile_enabled(e: &mut Engine, tile: u32) -> bool {
    let enabled = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TEXT, 0xffff_ffffu32])
        .u32();
    e.call(TILE_IS_TRUE, &args![tile, enabled]).bool()
}

/// The item word of a list node, reached through `006815c0` as the code
/// does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(RETURN_THIS, &args![node]).u32();
    e.mem.u32(node)
}

/// Deletes one `ListBoxItem` (Xbox PDB: `pTile` +0, `data` +4): the tile
/// through its deleting destructor (vtable slot 0, flag 1) unless
/// `KEEP_ITEM_TILES` is set, the data (`004459e0` with 1) when the list's
/// flag bit 1 is set, then frees the item. `item` re-reads the item word
/// each time, as the code does.
fn list_box_delete_item(e: &mut Engine, this: Ptr<ListBox>, item: impl Fn(&mut Engine) -> u32) {
    if e.global::<u8>(KEEP_ITEM_TILES) == 0 {
        let entry = item(e);
        let tile = e.mem.u32(entry);
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
    }
    if e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).bool() {
        let entry = item(e);
        let data = e.mem.u32(entry + 4);
        if data != 0 {
            e.call(ITEM_DELETE, &args![data, 1u32]);
        }
    }
    let entry = item(e);
    e.call(FREE, &args![entry]);
}

/// The `ListBox<T>` constructor body: the `ListBoxBase` base, the embedded
/// list (+4), the vtable, the fields cleared, `007a1950` with 0 and 1, and
/// the list box passed to the collection (`fn_00721130`).
fn list_box_construct(
    e: &mut Engine,
    this: Ptr<ListBox>,
    instance: ListBoxInstance,
) -> Ptr<ListBox> {
    fn_007240c0(e, this.cast());
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    set_vtable(e, this.cast(), instance.vtable);
    e.set(this, ListBox::pScrollBar, Ptr::NULL);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.set(this, ListBox::pContainerRect, Ptr::NULL);
    e.set(this, ListBox::pTemplateName, Ptr::NULL);
    e.set(this, ListBox::usFlags, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fSavedScrollbarValue, 0.0);
    e.set(this, ListBox::fSavedHighlightIndex, 0.0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    e.call(LIST_BOX_INIT_FLAGS, &args![this, 0u32, 1u32]);
    fn_00721130(e, this.addr());
    this
}

/// The `ListBox<T>` destructor body: restores the vtable, empties the list
/// (`RemoveAll`), passes the list box to the collection's second method
/// (`fn_007211c0`) and destroys the embedded list (+4).
fn list_box_destruct(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) {
    set_vtable(e, this.cast(), instance.vtable);
    list_box_remove_all(e, this, instance);
    fn_007211c0(e, this.addr());
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

/// `ListBox<T>::RemoveAll` (Xbox PDB): deletes the items of the nodes
/// after the first (while the node and its item are non-null: the item,
/// then the node's next pointer cleared and the node deleted, the list
/// then pointing at the following node), then the first node's item;
/// clears the first item, `usNextIndex` and `fTotalListHeight`, sets the
/// container's `"_num_filtered"` trait to 0 when there is a container,
/// recomputes the list height, clears `pHighlightedItem` and moves the
/// highlight box. Every field is re-read where the code re-reads it.
fn list_box_remove_all(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) {
    let base = this.addr();
    let second = |e: &mut Engine| e.mem.u32(base + 8);
    while second(e) != 0 {
        let node = second(e);
        if node_item(e, node) == 0 {
            break;
        }
        let node = second(e);
        let next = word_at_four(e, Ptr::new(node));
        list_box_delete_item(e, this, |e| {
            let node = e.mem.u32(base + 8);
            node_item(e, node)
        });
        let node = second(e);
        e.call(NODE_SET_NEXT, &args![node, 0u32]);
        let node = second(e);
        if node != 0 {
            e.call(NODE_DELETE, &args![node, 1u32]);
        }
        e.mem.set_u32(base + 8, next);
    }
    if e.mem.u32(base + 4) != 0 {
        list_box_delete_item(e, this, |e| e.mem.u32(base + 4));
    }
    e.mem.set_u32(base + 4, 0);
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    if !e.get(this, ListBox::pContainerRect).is_null() {
        let num_filtered = e.call(TEXT_TO_TRAIT, &args![NUM_FILTERED_TEXT]).u32();
        let container = e.get(this, ListBox::pContainerRect);
        e.call(
            TILE_SET_FLOAT,
            &args![container, num_filtered, 0.0f32.to_bits(), 1u32],
        );
    }
    e.call(instance.set_total_list_height, &args![this]);
    e.set(this, ListBox::pHighlightedItem, Ptr::NULL);
    e.call(instance.update_highlight_box_pos, &args![this]);
}

/// `ListBox<T>::Sort` (Xbox PDB): sorts the embedded list (`0083fd60` on
/// +4 with `compare`), then `Synchronize`.
fn list_box_sort(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance, compare: u32) {
    e.call(LIST_SORT, &args![this.addr() + 4, compare]);
    e.call(instance.synchronize, &args![this]);
}

/// `ListBox<T>::Highlight` (Xbox PDB): when the list is enabled and `tile`
/// is null, or belongs to the list (`007242f0`) and is `"_enabled"`, makes
/// it `pHighlightedItem`, moves the highlight box and returns true;
/// otherwise false.
fn list_box_highlight(
    e: &mut Engine,
    this: Ptr<ListBox>,
    instance: ListBoxInstance,
    tile: u32,
) -> bool {
    if !e.call(instance.is_enabled, &args![this]).bool() {
        return false;
    }
    if tile != 0
        && !(e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool() && tile_enabled(e, tile))
    {
        return false;
    }
    e.set(this, ListBox::pHighlightedItem, Ptr::new(tile));
    e.call(instance.update_highlight_box_pos, &args![this]);
    true
}

/// The step of a page scroll in `GetHighlightTarget`: 0 without a
/// container, otherwise the container's trait `0xfb0` (negated for a page
/// up) divided by its `"_scroll_delta"` trait, stored as a `float`, passed
/// with `1.0` to `004bd510` and converted with `_ftol2`.
fn list_box_page_step(e: &mut Engine, this: Ptr<ListBox>, negate: bool) -> i32 {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return 0;
    }
    let extent = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64();
    let numerator = if negate { -extent } else { extent };
    let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
    let container = e.get(this, ListBox::pContainerRect);
    let delta = e
        .call(TILE_GET_FLOAT, &args![container, scroll_delta])
        .f64();
    let step = x87_quiet_bits(((numerator / delta) as f32).to_bits());
    let rounded = e.call(SCROLL_ROUND, &args![step, 1.0f32.to_bits()]).f64();
    e.call(FTOL2, &args![rounded]).i32()
}

/// `ListBox<T>::GetHighlightTarget` (Xbox PDB): the tile a move in
/// `direction` highlights (1 up, 2 down, 0xF page up, 0x10 page down;
/// anything else does not move), or 0 when there is none or it is the one
/// already highlighted. See the body for the search; `GetTileAt` is
/// vtable slot +0x14.
fn list_box_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    instance: ListBoxInstance,
    direction: u32,
) -> u32 {
    let step = match direction {
        1 => -1,
        2 => 1,
        0xf => list_box_page_step(e, this, true),
        0x10 => list_box_page_step(e, this, false),
        _ => 0,
    };
    let mut target = 0u32;
    let count = if e.call(instance.is_enabled, &args![this]).bool() && step != 0 {
        e.get(this, ListBox::usNextIndex)
    } else {
        0
    };
    if count != 0 {
        let last = i32::from(count) - 1;
        let highlighted = e.get(this, ListBox::pHighlightedItem);
        let mut index = if !highlighted.is_null() {
            let value = e.call(TILE_GET_FLOAT, &args![highlighted, 0xfacu32]).f64();
            let moved = e.call(FTOL2, &args![value]).i32().wrapping_add(step);
            if moved < 0 {
                0
            } else if moved > last {
                last
            } else {
                moved
            }
        } else if step > 0 {
            0
        } else {
            last
        };
        'search: loop {
            // The item tile whose trait 0xfac equals the index.
            let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
            while target == 0 && node != 0 && node_item(e, node) != 0 {
                let item = node_item(e, node);
                let tile = e.mem.u32(item);
                let value = e.call(TILE_GET_FLOAT, &args![tile, 0xfacu32]).f64();
                if f64::from(index) == value {
                    let item = node_item(e, node);
                    target = e.mem.u32(item);
                }
                node = word_at_four(e, Ptr::new(node));
            }
            if target == 0 || tile_enabled(e, target) {
                break;
            }
            target = 0;
            if step > 0 {
                index = index.wrapping_add(1);
                if index > last {
                    // Past the end: back from the one before the last.
                    index = last - 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index < 0 {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index -= 1;
                    }
                }
            } else if step < 0 {
                index = index.wrapping_sub(1);
                if index < 0 {
                    // Before the start: on from index 1.
                    index = 1;
                    loop {
                        if target != 0 && tile_enabled(e, target) {
                            break 'search;
                        }
                        if index > last {
                            break 'search;
                        }
                        target = e.vcall(this.addr(), 0x14, &args![index, 0u32]).u32();
                        index += 1;
                    }
                }
            }
        }
    }
    if target != e.get(this, ListBox::pHighlightedItem).addr() {
        target
    } else {
        0
    }
}

/// A `BSSimpleArray` constructor: the vtable, then `006b3eb0` with 0 and
/// 0. Returns `this`.
fn simple_array_construct(e: &mut Engine, this: Ptr, vtable: u32) -> Ptr {
    set_vtable(e, this, vtable);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

/// A `BSSimpleArray` destructor: the vtable, then `008454f0` with 1.
fn simple_array_destruct(e: &mut Engine, this: Ptr, vtable: u32) {
    set_vtable(e, this, vtable);
    e.call(SIMPLE_ARRAY_RELEASE, &args![this, 1u32]);
}

/// Looks the static trait id up the first time (the guard bit is set
/// before `Tile::AddUserTrait(name, -1)` runs, as the code does); the code
/// then reads the id from the static where it uses it.
fn init_static_trait(e: &mut Engine, which: StaticTrait) {
    if e.mem.u32(which.guard) & which.bit == 0 {
        let guard = e.mem.u32(which.guard);
        e.mem.set_u32(which.guard, guard | which.bit);
        let id = e
            .call(ADD_USER_TRAIT, &args![which.name, 0xffff_ffffu32])
            .u32();
        e.mem.set_u32(which.cache, id);
    }
}

/// The first node of the embedded list (`this + 4`; the code guards a null
/// `this`).
fn first_node(this: Ptr<ListBox>) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr() + 4
    }
}

/// `ListBox<T>::Synchronize` (Xbox PDB): clears `usNextIndex` and
/// `fTotalListHeight`, then for every node with an item whose filtered
/// byte (+8) is clear gives the item's tile the next index (trait `0xfac`
/// through `00700320`) and its position (`InitTilePosition`); then
/// `SetTotalListHeight` and `UpdateHighlightBoxPos`.
fn list_box_synchronize(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) {
    e.set(this, ListBox::usNextIndex, 0);
    e.set(this, ListBox::fTotalListHeight, 0.0);
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u8(item + 8) == 0 {
            let index = e.get(this, ListBox::usNextIndex);
            e.set(this, ListBox::usNextIndex, index.wrapping_add(1));
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(TILE_SET_INT, &args![tile, 0xfacu32, u32::from(index)]);
            let item = node_item(e, node);
            let tile = e.mem.u32(item);
            e.call(INIT_TILE_POSITION, &args![this, tile]);
        }
        node = word_at_four(e, Ptr::new(node));
    }
    e.call(instance.set_total_list_height, &args![this]);
    e.call(instance.update_highlight_box_pos, &args![this]);
}

/// `ListBox<T>::IsEnabled` (Xbox PDB): the container exists and its
/// `"_enabled"` trait (a function-local static) is true.
fn list_box_is_enabled(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) -> bool {
    init_static_trait(e, instance.enabled_trait);
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return false;
    }
    let enabled = e.mem.u32(instance.enabled_trait.cache);
    e.call(TILE_IS_TRUE, &args![container, enabled]).bool()
}

/// The tile's byte at +0x35 set to `flag` (`0071aa60`) and the tile queued
/// for an update with flags 8.
fn mark_tile(e: &mut Engine, tile: u32, flag: u32) {
    e.call(TILE_SET_BYTE_35, &args![tile, flag]);
    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, 8u32]);
}

/// `ListBox<T>::UpdateHighlightBoxPos` (Xbox PDB): looks the
/// `"_highlight_y"` and `"_selected_height"` trait ids up (statics); with
/// a container, and no highlighted item, sets them to -1 and 0 on the
/// container; with one, makes both values follow the highlighted tile's
/// traits `0xfa2` and `0xfb0` (action `0x7d0`, then calculated) and gives
/// the container's `"lb_highlight_box"` tile, and its `"top"`, `"bottom"`,
/// `"left"` and `"right"` children when `"top"` exists (the other three
/// are not checked for null), the highlighted tile's byte at +0x35 (as 1
/// or 0) and an update.
fn list_box_update_highlight_box_pos(
    e: &mut Engine,
    this: Ptr<ListBox>,
    instance: ListBoxInstance,
) {
    init_static_trait(e, instance.highlight_y_trait);
    init_static_trait(e, instance.selected_height_trait);
    let y_cache = instance.highlight_y_trait.cache;
    let height_cache = instance.selected_height_trait.cache;
    if e.get(this, ListBox::pContainerRect).is_null() {
        return;
    }
    if e.get(this, ListBox::pHighlightedItem).is_null() {
        let y_trait = e.mem.u32(y_cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, y_trait, 0xffff_ffffu32]);
        let height_trait = e.mem.u32(height_cache);
        let container = e.get(this, ListBox::pContainerRect);
        e.call(TILE_SET_INT, &args![container, height_trait, 0u32]);
        return;
    }
    let y_trait = e.mem.u32(y_cache);
    let container = e.get(this, ListBox::pContainerRect);
    let y_value = e.call(TILE_GET_VALUE, &args![container, y_trait]).u32();
    let height_trait = e.mem.u32(height_cache);
    let container = e.get(this, ListBox::pContainerRect);
    let height_value = e
        .call(TILE_GET_VALUE, &args![container, height_trait])
        .u32();
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION,
        &args![y_value, 0x7d0u32, highlighted, 0xfa2u32],
    );
    e.call(VALUE_CALCULATE, &args![y_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    e.call(
        VALUE_ADD_ACTION,
        &args![height_value, 0x7d0u32, highlighted, 0xfb0u32],
    );
    e.call(VALUE_CALCULATE, &args![height_value, 0u32]);
    let highlighted = e.get(this, ListBox::pHighlightedItem);
    let flag = u32::from(e.call(TILE_GET_BYTE_35, &args![highlighted]).bool());
    let container = e.get(this, ListBox::pContainerRect);
    let box_tile = e
        .call(TILE_FIND_BY_NAME, &args![container, HIGHLIGHT_BOX_TEXT])
        .u32();
    if box_tile == 0 {
        return;
    }
    let top = e
        .call(TILE_GET_CHILD_BY_NAME, &args![box_tile, TOP_TEXT])
        .u32();
    if top != 0 {
        mark_tile(e, top, flag);
        for name in [BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT] {
            let edge = e.call(TILE_GET_CHILD_BY_NAME, &args![box_tile, name]).u32();
            mark_tile(e, edge, flag);
        }
    }
    mark_tile(e, box_tile, flag);
}

/// `ListBox<T>::SetTotalListHeight` (Xbox PDB): with a container, its
/// visible height (trait `0xfb0`, stored as a `float`) and the total list
/// height decide the number of scroll steps: 1, unless both are above
/// `0.0` and the visible height is below the total, then
/// `max(_ftol2(004e4430(unfiltered * (total - visible) / total)) + 1, 2)`
/// (the ratio stored as a `float`). With both above `0.0` and more than
/// one step, the container's `"_scroll_delta"` becomes
/// `(total - visible) / (steps - 1)`; with a scroll bar its
/// `"_number_of_items"` becomes the number of steps.
fn list_box_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) {
    let container = e.get(this, ListBox::pContainerRect);
    if container.is_null() {
        return;
    }
    let visible = e.call(TILE_GET_FLOAT, &args![container, 0xfb0u32]).f64() as f32;
    let mut steps = 1i32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP; TEST AH,0x41; JNZ`: on only when strictly above (not NaN).
    if f64::from(e.get(this, ListBox::fTotalListHeight)) > zero && f64::from(visible) > zero {
        // `FCOMPP; TEST AH,5; JP`: on only when the visible height is
        // below the total (ordered).
        if f64::from(visible) < f64::from(e.get(this, ListBox::fTotalListHeight)) {
            let count = e.call(instance.num_unfiltered, &args![this]).u32();
            let total = f64::from(e.get(this, ListBox::fTotalListHeight));
            let ratio = (f64::from(count) * (total - f64::from(visible)) / total) as f32;
            let rounded = e
                .call(ROUND_FLOAT, &args![x87_quiet_bits(ratio.to_bits())])
                .f64();
            let whole = e.call(FTOL2, &args![rounded]).i32();
            steps = e.call(MAX_INT, &args![whole.wrapping_add(1), 2i32]).i32();
        }
        if steps > 1 {
            let total = f64::from(e.get(this, ListBox::fTotalListHeight));
            let delta = ((total - f64::from(visible)) / f64::from(steps - 1)) as f32;
            let scroll_delta = e.call(TEXT_TO_TRAIT, &args![SCROLL_DELTA_TEXT]).u32();
            let container = e.get(this, ListBox::pContainerRect);
            e.call(
                TILE_SET_FLOAT,
                &args![
                    container,
                    scroll_delta,
                    x87_quiet_bits(delta.to_bits()),
                    1u32
                ],
            );
        }
    }
    if !e.get(this, ListBox::pScrollBar).is_null() {
        let items = e.call(TEXT_TO_TRAIT, &args![NUMBER_OF_ITEMS_TEXT]).u32();
        let scroll_bar = e.get(this, ListBox::pScrollBar);
        e.call(TILE_SET_INT, &args![scroll_bar, items, steps]);
    }
}

/// `ListBox<T>::NumUnfiltered` (Xbox PDB): the list's count (`00726c40`)
/// minus the container's `"_num_filtered"` trait (a function-local
/// static), truncated by a 64-bit `FISTP` and taken to 32 bits.
fn list_box_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>, instance: ListBoxInstance) -> i32 {
    init_static_trait(e, instance.num_filtered_trait);
    let count = e.call(LIST_COUNT, &args![this]).i32();
    let num_filtered = e.mem.u32(instance.num_filtered_trait.cache);
    let container = e.get(this, ListBox::pContainerRect);
    let filtered = e
        .call(TILE_GET_FLOAT, &args![container, num_filtered])
        .f64();
    count.wrapping_sub(x87_truncate_i64(filtered) as i32)
}

// Translated from 0071e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::Instance` (Xbox PDB), cdecl: with `create` set and no
/// instance yet, allocates one byte, constructs the manager in it
/// (`fn_0071e340`, skipped when the allocation fails), stores it in the
/// global and runs the constructor body (`fn_0071e400`) again on it.
/// Returns the global.
// Translation incomplete: 0071e290 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
pub fn menu_manager_instance(e: &mut Engine, create: bool) -> u32 {
    if create && e.global::<u32>(MENU_MANAGER) == 0 {
        let block = e.call(ALLOCATE, &args![1u32]).u32();
        let manager = if block == 0 {
            0
        } else {
            fn_0071e340(e, Ptr::new(block)).addr()
        };
        e.set_global(MENU_MANAGER, manager);
        let manager = e.global::<u32>(MENU_MANAGER);
        fn_0071e400(e, Ptr::new(manager));
    }
    e.global(MENU_MANAGER)
}

// Translated from 0071e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuManager` constructor: runs the constructor body
/// (`fn_0071e400`) and returns `this`.
pub fn fn_0071e340(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0071e400(e, this);
    this
}

// Translated from 0071e360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the `MenuManager` instance, if any (deleting destructor with
/// flag 1), and clears the global.
pub fn fn_0071e360(e: &mut Engine) {
    let manager: u32 = e.global(MENU_MANAGER);
    if manager != 0 {
        fn_0071e3b0(e, Ptr::new(manager), 1);
        e.set_global(MENU_MANAGER, 0u32);
    }
}

// Translated from 0071e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuManager` deleting destructor: runs the destructor
/// (`fn_0071e3e0`) and frees `this` when bit 0 of `flags` is set.
pub fn fn_0071e3b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071e3e0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071e3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuManager` destructor: clears the instance global
/// (`fn_0071e3f0`).
pub fn fn_0071e3e0(e: &mut Engine, _this: Ptr) {
    fn_0071e3f0(e);
}

// Translated from 0071e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the `MenuManager` instance global. Takes no argument (it does
/// not read `ECX`).
pub fn fn_0071e3f0(e: &mut Engine) {
    e.set_global(MENU_MANAGER, 0u32);
}

// Translated from 0071e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuManager` constructor body: calls `0096ad30` on the object at
/// `011f3508` with `0x54`. `this` is not read.
pub fn fn_0071e400(e: &mut Engine, _this: Ptr) {
    e.call(MENU_TABLE_GROW, &args![MENU_TABLE, MENU_TABLE_COUNT]);
}

// Translated from 0071e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::CreateMenuByClass` (Xbox PDB): allocates and constructs
/// the menu of class `class` (`menu_class`: allocation size and
/// constructor) and returns it; null when the allocation fails. An
/// unknown class (outside `0x3e9..=0x43c`, or a gap in the table) prints
/// `"MENUS: Unknown menu class!"` and returns null. `this` is not read.
// Translation incomplete: 0071e420 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when a menu constructor throws is not translated
pub fn menu_manager_create_menu_by_class(e: &mut Engine, _this: Ptr, class: u32) -> u32 {
    match menu_class(class) {
        Some((size, constructor)) => {
            let block = e.call(ALLOCATE, &args![size]).u32();
            if block == 0 {
                0
            } else {
                e.call(constructor, &args![block]).u32()
            }
        }
        None => {
            e.call(DEBUG_MESSAGE, &args![UNKNOWN_MENU_CLASS_TEXT]);
            0
        }
    }
}

// Translated from 0071f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TutorialMenu` constructor: the `Menu` base, the vtable, the eight
/// tile pointers (+0x28) zeroed, and the page count, current page and help
/// manual cleared. Returns `this`.
// Translation incomplete: 0071f240 - the C++ exception-unwinding frame (FS:[0]) that destroys the Menu base when the constructor throws is not translated
pub fn fn_0071f240(e: &mut Engine, this: Ptr<TutorialMenu>) -> Ptr<TutorialMenu> {
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this.cast(), TUTORIAL_MENU_VTABLE);
    memset(e, this.addr() + 0x28, 0x20);
    e.set(this, TutorialMenu::iNumPages, 0);
    e.set(this, TutorialMenu::iCurrentPage, 0);
    e.set(this, TutorialMenu::pHelpManual, 0);
    this
}

// Translated from 0071f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::GetClass` (Xbox PDB): `0x423`.
pub fn tutorial_menu_get_class(_e: &mut Engine, _this: Ptr<TutorialMenu>) -> u32 {
    0x423
}

// Translated from 0071f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn tutorial_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TutorialMenu>,
    flags: u32,
) -> Ptr<TutorialMenu> {
    fn_0071f310(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 0071f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TutorialMenu` destructor: restores the vtable, clears the tutorial
/// menu global and destroys the `Menu` base.
pub fn fn_0071f310(e: &mut Engine, this: Ptr<TutorialMenu>) {
    set_vtable(e, this.cast(), TUTORIAL_MENU_VTABLE);
    e.set_global(TUTORIAL_MENU_GLOBAL, 0u32);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 0071f340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu` constructor: the `Menu` base and vtable, the four
/// healing options (`pHealingOptionsA`, +0x28, 0x10 bytes each, built by
/// `fn_0071f4a0`), the two effect lists (+0x68 named, +0x84 status), the
/// six list boxes (+0x180 to +0x270), the 60 tile pointers (`pTiles`,
/// +0x90) zeroed, the four status effect pointers (+0x74 to +0x80) and the
/// two flags (+0x2A1, +0x2A0) cleared. Returns `this`.
// Translation incomplete: 0071f340 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_0071f340(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, STATS_MENU_VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x28, 0x10u32, 4u32, 0x0071_f4a0u32, 0x0071_f4e0u32],
    );
    fn_0071f540(e, Ptr::new(base + 0x68));
    fn_0071f5d0(e, Ptr::new(base + 0x84));
    for (offset, constructor) in STATS_LIST_BOX_CONSTRUCTORS {
        e.call(constructor, &args![base + offset]);
    }
    memset(e, base + 0x90, 0xf0);
    // pRadiationEffect, pDehydrationEffect, pSleepEffect, pHungerEffect
    // (Xbox PDB), in the code's order.
    for offset in [0x74, 0x78, 0x80, 0x7c] {
        e.mem.set_u32(base + offset, 0);
    }
    // bActiveEffectsUpdatedThisFrame, bLimbMode (Xbox PDB).
    e.mem.set_u8(base + 0x2a1, 0);
    e.mem.set_u8(base + 0x2a0, 0);
    this
}

// Translated from 0071f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::HealingOption` constructor: clears `pAlchemyItem`,
/// `pTile` and `pInventoryItem` and sets `pfTargetConditionCallback` to
/// `fn_0071f520` (Xbox PDB names). Returns `this`.
pub fn fn_0071f4a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0x0071_f520);
    this
}

// Translated from 0071f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::HealingOption` destructor: calls `004459e0` with flag 1
/// on the inventory item (+8), if any.
pub fn fn_0071f4e0(e: &mut Engine, this: Ptr) {
    let item = e.mem.u32(this.addr() + 8);
    if item != 0 {
        e.call(ITEM_DELETE, &args![item, 1u32]);
    }
}

// Translated from 0071f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// A healing option's target condition (cdecl, the option): whether it has
/// an inventory item (+8). The value is in `AL`; the rest of `EAX` is the
/// option's address, which callers of a `bool` do not read.
pub fn fn_0071f520(e: &mut Engine, option: Ptr) -> bool {
    e.mem.u32(option.addr() + 8) != 0
}

// Translated from 0071f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::NamedEffectsList` constructor: the list at +4, then the
/// vtable. Returns `this`.
pub fn fn_0071f540(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    set_vtable(e, this, NAMED_EFFECTS_LIST_VTABLE);
    this
}

// Translated from 0071f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::NamedEffectsList` destructor: restores the vtable,
/// empties the list (`007e09b0`) and destroys the list at +4.
// Translation incomplete: 0071f570 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when emptying it throws is not translated
pub fn fn_0071f570(e: &mut Engine, this: Ptr) {
    set_vtable(e, this, NAMED_EFFECTS_LIST_VTABLE);
    e.call(NAMED_EFFECTS_CLEAR, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 0071f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::StatusEffectList` constructor: the list at +4, then the
/// vtable. Returns `this`.
pub fn fn_0071f5d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    set_vtable(e, this, STATUS_EFFECT_LIST_VTABLE);
    this
}

// Translated from 0071f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::StatusEffectList` destructor: restores the vtable,
/// empties the list (`007e07f0`) and destroys the list at +4.
// Translation incomplete: 0071f600 - the C++ exception-unwinding frame (FS:[0]) that destroys the list when emptying it throws is not translated
pub fn fn_0071f600(e: &mut Engine, this: Ptr) {
    set_vtable(e, this, STATUS_EFFECT_LIST_VTABLE);
    e.call(STATUS_EFFECTS_CLEAR, &args![this]);
    e.call(LIST_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 0071f660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::GetClass` (Xbox PDB): `0x3eb`.
pub fn stats_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x3eb
}

// Translated from 0071f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::NamedEffectsList::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor and frees `this` when bit 0 of `flags` is set.
pub fn stats_menu_named_effects_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0071f570(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::StatusEffectList::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor and frees `this` when bit 0 of `flags` is set.
pub fn stats_menu_status_effect_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0071f600(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn stats_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071f700(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu` destructor: restores the vtable, clears the stats menu
/// global, deletes the four status effects (+0x74, +0x80, +0x7C, +0x78 in
/// that order, `fn_0071f9c0` with flag 1), deletes the body part
/// controller (`fn_0071f8c0`), destroys the six list boxes, the two effect
/// lists and the four healing options, then the `Menu` base.
// Translation incomplete: 0071f700 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated
pub fn fn_0071f700(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    set_vtable(e, this, STATS_MENU_VTABLE);
    e.set_global(STATS_MENU_GLOBAL, 0u32);
    for offset in [0x74, 0x80, 0x7c, 0x78] {
        let effect = e.mem.u32(base + offset);
        if effect != 0 {
            fn_0071f9c0(e, Ptr::new(effect), 1);
        }
    }
    fn_0071f8c0(e);
    for (offset, destructor) in STATS_LIST_BOX_DESTRUCTORS {
        e.call(destructor, &args![base + offset]);
    }
    fn_0071f600(e, Ptr::new(base + 0x84));
    fn_0071f570(e, Ptr::new(base + 0x68));
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x28, 0x10u32, 4u32, 0x0071_f4e0u32],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 0071f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the object in the global `011dace4`, if any (`fn_0071f900` with
/// flag 1), and clears the global.
pub fn fn_0071f8c0(e: &mut Engine) {
    let controller: u32 = e.global(BODY_PART_CONTROLLER_GLOBAL);
    if controller != 0 {
        fn_0071f900(e, Ptr::new(controller), 1);
    }
    e.set_global(BODY_PART_CONTROLLER_GLOBAL, 0u32);
}

// Translated from 0071f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the object `fn_0071f8c0` deletes: runs
/// `fn_0071f930` and frees `this` when bit 0 of `flags` is set.
pub fn fn_0071f900(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071f930(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the object `fn_0071f8c0` deletes: empties the six 8-byte
/// lists from +4 (`00470470`), then destroys them (`0046ffb0` through the
/// array destructor helper).
// Translation incomplete: 0071f930 - the C++ exception-unwinding frame (FS:[0]) that destroys the lists when emptying one throws is not translated
pub fn fn_0071f930(e: &mut Engine, this: Ptr) {
    for index in 0..6u32 {
        e.call(LIST_CLEAR, &args![this.addr() + index * 8 + 4]);
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 4, 8u32, 6u32, LIST_DESTRUCT],
    );
}

// Translated from 0071f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::StatusEffect` deleting destructor: runs the destructor
/// (`fn_0071f9f0`) and frees `this` when bit 0 of `flags` is set.
pub fn fn_0071f9c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071f9f0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `StatsMenu::StatusEffect` destructor: destroys `xNamedEffects`
/// (+0x10) and flushes `xStatsEffected` (+4) (Xbox PDB names).
// Translation incomplete: 0071f9f0 - the C++ exception-unwinding frame (FS:[0]) that flushes the data list when the effect list destructor throws is not translated
pub fn fn_0071f9f0(e: &mut Engine, this: Ptr) {
    fn_0071f570(e, Ptr::new(this.addr() + 0x10));
    e.call(STATUS_DATA_FLUSH, &args![this.addr() + 4]);
}

// Translated from 0071fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::AttachTileByID` (Xbox PDB): stores `tile` in `pTiles`
/// (+0x90) for ids 0 to 59 (signed compare); other ids do nothing.
pub fn stats_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0x3c).contains(&id) {
        e.mem.set_u32(this.addr() + 0x90 + 4 * id as u32, tile);
    }
}

// Translated from 0071fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TextEditMenu` constructor: the `Menu` base and vtable, the editor
/// (+0x34), the three tile pointers (+0x28) zeroed, the entered text
/// global emptied (`004037f0` with null and 0) and the text check (+0x58)
/// cleared. Returns `this`.
// Translation incomplete: 0071fa80 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_0071fa80(e: &mut Engine, this: Ptr<TextEditMenu>) -> Ptr<TextEditMenu> {
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this.cast(), TEXT_EDIT_MENU_VTABLE);
    e.call(TEXT_EDITOR_CONSTRUCT, &args![this.addr() + 0x34]);
    memset(e, this.addr() + 0x28, 0xc);
    e.call(STRING_SET, &args![ENTERED_TEXT, 0u32, 0u32]);
    e.set(this, TextEditMenu::pTextCheck, 0);
    this
}

// Translated from 0071fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TextEditMenu`'s `GetClass` (vtable slot +0x34): `0x41b`.
pub fn fn_0071fb20(_e: &mut Engine, _this: Ptr<TextEditMenu>) -> u32 {
    0x41b
}

// Translated from 0071fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TextEditMenu`'s deleting destructor (vtable slot 0): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn fn_0071fb30(e: &mut Engine, this: Ptr<TextEditMenu>, flags: u32) -> Ptr<TextEditMenu> {
    fn_0071fb60(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 0071fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TextEditMenu` destructor: restores the vtable, destroys the editor
/// (+0x34) and the `Menu` base.
// Translation incomplete: 0071fb60 - the C++ exception-unwinding frame (FS:[0]) that destroys the Menu base when the editor destructor throws is not translated
pub fn fn_0071fb60(e: &mut Engine, this: Ptr<TextEditMenu>) {
    set_vtable(e, this.cast(), TEXT_EDIT_MENU_VTABLE);
    e.call(TEXT_EDITOR_DESTRUCT, &args![this.addr() + 0x34]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 0071fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TextEditMenu`'s `AttachTileByID` (vtable slot +4): stores `tile` at
/// +0x28 for ids 0 to 2 (signed compare); other ids do nothing.
pub fn fn_0071fbc0(e: &mut Engine, this: Ptr<TextEditMenu>, id: i32, tile: u32) {
    if (0..3).contains(&id) {
        e.mem.set_u32(this.addr() + 0x28 + 4 * id as u32, tile);
    }
}

// Translated from 0071fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SurgeryMenu` constructor: the `Menu` base and vtable, the members
/// (empty constructors at +0x74, +0x7C, +0xF8, +0x104, +0x128, +0x150; sound handles
/// at +0x94 and +0xA0; six smart pointers at +0xC8; smart pointers at
/// +0xF0 and +0xF4), the 18 tile pointers (+0x2C) zeroed, the three static
/// smart pointers, +0xF0, +0xF4 and the six at +0xC8 set to null, then the
/// scalar fields (Xbox PDB names in the body). Returns `this`.
// Translation incomplete: 0071fbf0 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_0071fbf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, SURGERY_MENU_VTABLE);
    e.call(RETURN_THIS, &args![base + 0x74]);
    e.call(RETURN_THIS, &args![base + 0x7c]);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![base + 0x94]);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![base + 0xa0]);
    construct_pointer_array(e, this, 0xc8, 6);
    e.call(POINTER_CONSTRUCT, &args![base + 0xf0, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base + 0xf4, 0u32]);
    for offset in [0xf8, 0x104, 0x128, 0x150] {
        e.call(RETURN_THIS, &args![base + offset]);
    }
    memset(e, base + 0x2c, 0x48);
    for global in SURGERY_STATIC_POINTERS {
        e.call(POINTER_ASSIGN, &args![global, 0u32]);
    }
    e.call(POINTER_ASSIGN, &args![base + 0xf0, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + 0xf4, 0u32]);
    for index in 0..6u32 {
        e.call(POINTER_ASSIGN, &args![base + 0xc8 + 4 * index, 0u32]);
    }
    // pFrameGeometry, pSkinControllerManager, pStimPaks, pScar.
    for offset in [0xe0, 0xe4, 0xbc, 0xe8] {
        e.mem.set_u32(base + offset, 0);
    }
    // fFrameAnimTimer, fToolMovement, fPainPercent; +0x14C (a float).
    for offset in [0xec, 0x84, 0xac, 0x14c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    // uiDamageHealthTime is set to 1000 and cleared again below.
    e.mem.set_u32(base + 0xb8, 0x3e8);
    // uiWoundBurnTime, uiWoundCloseTime, uiPainTimer, uiHitPointTimer,
    // uiDamageHealthTime.
    for offset in [0x88, 0x8c, 0xb0, 0xb4, 0xb8] {
        e.mem.set_u32(base + offset, 0);
    }
    // bBurning, bToolBuriedInFlesh; +0x15C, +0x15D.
    for offset in [0x90, 0x91, 0x15c, 0x15d] {
        e.mem.set_u8(base + offset, 0);
    }
    // Resistance, ToolLocation (NiPoint2, Xbox PDB).
    for offset in [0x7c, 0x80, 0x74, 0x78] {
        e.mem.set_f32(base + offset, 0.0);
    }
    // eCurrentDamagedPart, eCurrentDamageLevel, eAnimation.
    e.mem.set_u32(base + 0xc0, 0x1b);
    e.mem.set_u32(base + 0xc4, 1);
    e.mem.set_u32(base + 0x28, 0);
    this
}

// Translated from 0071feb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::GetClass` (Xbox PDB): `0x41e`.
pub fn surgery_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x41e
}

// Translated from 0071fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn surgery_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0071fef0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0071fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SurgeryMenu` destructor: restores the vtable, sets the three
/// static smart pointers and +0xF0 to null, destroys the smart pointers at
/// +0xF4, +0xF0 and the six at +0xC8, calls the empty function `00483710`
/// on the sound handles (+0xA0, +0x94) and destroys the `Menu` base.
// Translation incomplete: 0071fef0 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated
pub fn fn_0071fef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    set_vtable(e, this, SURGERY_MENU_VTABLE);
    for global in SURGERY_STATIC_POINTERS {
        e.call(POINTER_ASSIGN, &args![global, 0u32]);
    }
    e.call(POINTER_ASSIGN, &args![base + 0xf0, 0u32]);
    e.call(POINTER_DESTRUCT, &args![base + 0xf4]);
    e.call(POINTER_DESTRUCT, &args![base + 0xf0]);
    destruct_pointer_array(e, this, 0xc8, 6);
    e.call(EMPTY_FUNCTION, &args![base + 0xa0]);
    e.call(EMPTY_FUNCTION, &args![base + 0x94]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 0071ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::AttachTileByID` (Xbox PDB): for ids 1 to 18 (signed
/// compare) stores `tile` in `pTiles` (+0x2C) at the index `fn_00720010`
/// gives (`id - 1`); other ids do nothing.
pub fn surgery_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (1..=0x12).contains(&id) {
        let index = fn_00720010(e, this, id);
        e.mem.set_u32(this.addr() + 0x2c + 4 * index as u32, tile);
    }
}

// Translated from 00720010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The surgery menu's tile index of a tile id: `id - 1` for ids 1 to 18
/// (signed compare), 18 otherwise.
pub fn fn_00720010(_e: &mut Engine, _this: Ptr, id: i32) -> i32 {
    if (1..=0x12).contains(&id) {
        id - 1
    } else {
        0x12
    }
}

// Translated from 00720040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `VATSMenu` constructor: the `Menu` base and vtable, the action list
/// box (`lbActions`, +0xB0), the command at +0x11C, the 32 tile pointers
/// (`pTiles`, +0x28) zeroed, `bHeadingReached` (+0xF8) and
/// `bConcentratedFirePerk` (+0x104) cleared and `pLastLimb` (+0x108) null
/// (Xbox PDB names). Returns `this`.
// Translation incomplete: 00720040 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_00720040(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, VATS_MENU_VTABLE);
    e.call(VATS_ACTION_LIST_CONSTRUCT, &args![base + 0xb0]);
    e.call(VATS_COMMAND_CONSTRUCT, &args![base + 0x11c]);
    memset(e, base + 0x28, 0x80);
    e.mem.set_u8(base + 0xf8, 0);
    e.mem.set_u8(base + 0x104, 0);
    e.mem.set_u32(base + 0x108, 0);
    this
}

// Translated from 00720100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::GetClass` (Xbox PDB): `0x420`.
pub fn vats_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x420
}

// Translated from 00720110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`007e9130`) and frees `this` when bit 0 of `flags` is set.
pub fn vats_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(VATS_MENU_DESTRUCT, &args![this]);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00720140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATSMenu::AttachTileByID` (Xbox PDB): stores `tile` at
/// `+0x28 + 4 * id` for ids 1 to 31 (signed compare); other ids, 0
/// included, do nothing.
pub fn vats_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (1..=0x1f).contains(&id) {
        e.mem.set_u32(this.addr() + 0x28 + 4 * id as u32, tile);
    }
}

// Translated from 00720170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SPECIALBookMenu` constructor: the `Menu` base and vtable, the nine
/// pages (`Pages`, +0x60, 0x20 bytes each: `fn_00720360` /
/// `fn_007203c0`), the smart pointer arrays (`spAVValues` 11 at +0x180,
/// `spPointsText` 4 at +0x1AC, five pairs from +0x1BC), the eight tile
/// pointers (+0x28) zeroed and the state fields set (Xbox PDB names in the
/// body). Returns `this`.
// Translation incomplete: 00720170 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_00720170(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, SPECIAL_BOOK_MENU_VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x60, 0x20u32, 9u32, 0x0072_0360u32, 0x0072_03c0u32],
    );
    construct_pointer_array(e, this, 0x180, 0xb);
    construct_pointer_array(e, this, 0x1ac, 4);
    for offset in [0x1bc, 0x1c4, 0x1cc, 0x1d4, 0x1dc] {
        construct_pointer_array(e, this, offset, 2);
    }
    memset(e, base + 0x28, 0x20);
    // pBookGeometry, eState, eXBoxIndexState, fFrameAnimTimer,
    // bAnimFinished, iAvailablePoints.
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 1);
    e.mem.set_f32(base + 0x50, 0.0);
    e.mem.set_u8(base + 0x54, 1);
    e.mem.set_u32(base + 0x58, 0);
    this
}

// Translated from 00720320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::GetClass` (Xbox PDB): `0x424`.
pub fn special_book_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x424
}

// Translated from 00720330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn special_book_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00720420(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00720360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SPECIALBookMenu::SpecialBookPage` constructor: `Buttons` (+0) and
/// `TextItems` (+0x10) (Xbox PDB). Returns `this`.
// Translation incomplete: 00720360 - the C++ exception-unwinding frame (FS:[0]) that destroys the buttons when the text item constructor throws is not translated
pub fn fn_00720360(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BOOK_BUTTONS_CONSTRUCT, &args![this]);
    e.call(BOOK_TEXT_ITEMS_CONSTRUCT, &args![this.addr() + 0x10]);
    this
}

// Translated from 007203c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SPECIALBookMenu::SpecialBookPage` destructor: `TextItems` (+0x10),
/// then `Buttons` (+0).
// Translation incomplete: 007203c0 - the C++ exception-unwinding frame (FS:[0]) that destroys the buttons when the text item destructor throws is not translated
pub fn fn_007203c0(e: &mut Engine, this: Ptr) {
    e.call(BOOK_TEXT_ITEMS_DESTRUCT, &args![this.addr() + 0x10]);
    e.call(BOOK_BUTTONS_DESTRUCT, &args![this]);
}

// Translated from 00720420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SPECIALBookMenu` destructor: restores the vtable, destroys the
/// smart pointer arrays (in reverse order) and the nine pages, then the
/// `Menu` base.
// Translation incomplete: 00720420 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated
pub fn fn_00720420(e: &mut Engine, this: Ptr) {
    set_vtable(e, this, SPECIAL_BOOK_MENU_VTABLE);
    for offset in [0x1dc, 0x1d4, 0x1cc, 0x1c4, 0x1bc] {
        destruct_pointer_array(e, this, offset, 2);
    }
    destruct_pointer_array(e, this, 0x1ac, 4);
    destruct_pointer_array(e, this, 0x180, 0xb);
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 0x60, 0x20u32, 9u32, 0x0072_03c0u32],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 00720550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterMenu` constructor: the `Menu` base and vtable,
/// `LightBulbs` (+0x64) and `LightBulbGlows` (+0x74), the nine pages
/// (`Pages`, +0x84, `fn_007207a0` / `fn_00720800`), `GlobalPage` (+0x1A4),
/// the smart pointer arrays (11 at +0x1C4, 4 at +0x1F0, six pairs from
/// +0x200), the eight tile pointers (+0x28) zeroed and the state fields
/// set (Xbox PDB names in the body). Returns `this`.
// Translation incomplete: 00720550 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated
pub fn fn_00720550(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, LOVE_TESTER_MENU_VTABLE);
    e.call(LOVE_TESTER_TEXT_ITEMS_CONSTRUCT, &args![base + 0x64]);
    e.call(LOVE_TESTER_GLOWS_CONSTRUCT, &args![base + 0x74]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x84, 0x20u32, 9u32, 0x0072_07a0u32, 0x0072_0800u32],
    );
    fn_007207a0(e, Ptr::new(base + 0x1a4));
    construct_pointer_array(e, this, 0x1c4, 0xb);
    construct_pointer_array(e, this, 0x1f0, 4);
    for offset in [0x200, 0x208, 0x210, 0x218, 0x220, 0x228] {
        construct_pointer_array(e, this, offset, 2);
    }
    memset(e, base + 0x28, 0x20);
    // pDynamicGeometry, pCabinetGeometry, eState, eXBoxIndexState,
    // fFrameAnimTimer, bAnimFinished, iAvailablePoints.
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 1);
    e.mem.set_f32(base + 0x50, 0.0);
    e.mem.set_u8(base + 0x54, 1);
    e.mem.set_u32(base + 0x58, 0);
    this
}

// Translated from 00720760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::GetClass` (Xbox PDB): `0x432`.
pub fn love_tester_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x432
}

// Translated from 00720770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn love_tester_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00720860(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 007207a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterMenu::LoveTesterPage` constructor: `Buttons` (+0) and
/// `TextItems` (+0x10) (Xbox PDB). Returns `this`.
// Translation incomplete: 007207a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the buttons when the text item constructor throws is not translated
pub fn fn_007207a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LOVE_TESTER_BUTTONS_CONSTRUCT, &args![this]);
    e.call(LOVE_TESTER_TEXT_ITEMS_CONSTRUCT, &args![this.addr() + 0x10]);
    this
}

// Translated from 00720800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterMenu::LoveTesterPage` destructor: `TextItems` (+0x10),
/// then `Buttons` (+0).
// Translation incomplete: 00720800 - the C++ exception-unwinding frame (FS:[0]) that destroys the buttons when the text item destructor throws is not translated
pub fn fn_00720800(e: &mut Engine, this: Ptr) {
    e.call(LOVE_TESTER_TEXT_ITEMS_DESTRUCT, &args![this.addr() + 0x10]);
    e.call(LOVE_TESTER_BUTTONS_DESTRUCT, &args![this]);
}

// Translated from 00720860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterMenu` destructor: restores the vtable, destroys the
/// smart pointer arrays (in reverse order), `GlobalPage`, the nine pages,
/// `LightBulbGlows` and `LightBulbs`, then the `Menu` base.
// Translation incomplete: 00720860 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated
pub fn fn_00720860(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    set_vtable(e, this, LOVE_TESTER_MENU_VTABLE);
    for offset in [0x228, 0x220, 0x218, 0x210, 0x208, 0x200] {
        destruct_pointer_array(e, this, offset, 2);
    }
    destruct_pointer_array(e, this, 0x1f0, 4);
    destruct_pointer_array(e, this, 0x1c4, 0xb);
    fn_00720800(e, Ptr::new(base + 0x1a4));
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x84, 0x20u32, 9u32, 0x0072_0800u32],
    );
    e.call(LOVE_TESTER_GLOWS_DESTRUCT, &args![base + 0x74]);
    e.call(LOVE_TESTER_TEXT_ITEMS_DESTRUCT, &args![base + 0x64]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 007209e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::AttachTileByID` (Xbox PDB): stores `tile` at
/// `+0x28 + 4 * id` for ids 0 to 7 (signed compare); other ids do nothing.
/// The linker shares the body: the `SPECIALBookMenu` and `LoveTesterMenu`
/// vtables have it in the same slot.
pub fn tutorial_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..8).contains(&id) {
        e.mem.set_u32(this.addr() + 0x28 + 4 * id as u32, tile);
    }
}

// Translated from 00720a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `QuantityMenu` constructor: the `Menu` base and the vtable. Returns
/// `this`.
pub fn fn_00720a10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCT, &args![this]);
    set_vtable(e, this, QUANTITY_MENU_VTABLE);
    this
}

// Translated from 00720a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::GetClass` (Xbox PDB): `0x3f8`.
pub fn quantity_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    0x3f8
}

// Translated from 00720a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QuantityMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and frees `this` when bit 0 of `flags` is set.
pub fn quantity_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00720a70(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00720a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `QuantityMenu` destructor: restores the vtable, clears the global
/// `011da618` and destroys the `Menu` base.
pub fn fn_00720a70(e: &mut Engine, this: Ptr) {
    set_vtable(e, this, QUANTITY_MENU_VTABLE);
    e.set_global(QUANTITY_MENU_GLOBAL, 0u32);
    e.call(MENU_DESTRUCT, &args![this]);
}

/// The steps after `ReloadMenu` creates a menu with a timer: the created
/// tile's trait `0xfc1` (a float) goes with the tile's menu to
/// `InterfaceManager::NewTimer`, and `RecomputePacking(8)` runs on the
/// tile's menu (fetched again).
fn reload_with_timer(e: &mut Engine, create: u32) -> u32 {
    let tile = e.call(create, &args![]).u32();
    let delay = e.call(TILE_GET_FLOAT, &args![tile, 0xfc1u32]).f32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(NEW_TIMER, &args![menu, delay.to_bits()]);
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(RECOMPUTE_PACKING, &args![menu, 8u32]);
    tile
}

// Translated from 00720aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::ReloadMenu` (Xbox PDB): reads a flag (`004a4040`),
/// deletes the open menu of `class` (vtable slot 0, flag 1), then creates
/// it again and returns what the creator returns (0 for classes it does
/// not handle):
/// - `0x3ff` (`fn_00720df0`), `0x40b`, `0x425`, `0x3ea`, `0x3eb`: the
///   creator, then the timer and packing steps (`reload_with_timer`);
/// - `0x3ec`: `HUDMainMenu::Create`;
/// - the start menu class (`0070ede0`) when `StartMenu::Create(flag, 0)`
///   succeeds: the menu of that class;
/// - `0x3ef`: `LoadingMenu::Create(0, 1, 0)`; `0x403`: `LevelUpMenu::Create`;
/// - `0x3f0`: the open menu of class `0x3f0` cast to `ContainerMenu`; when
///   it is one, `ContainerMenu::Create(<its word at +0x74>, 1)`;
/// - `0x402`: the open menu of class `0x3f0` cast to `BookMenu`; when it is
///   one, its word at +0x30 (`x`), `x`'s word at +0x20 cast from
///   `TESBoundObject` to `TESObjectBOOK`, the cast menu deleted, then
///   `BookMenu::Create(book, x)`.
///
/// `this` is not read.
pub fn menu_manager_reload_menu(e: &mut Engine, _this: Ptr, class: u32) -> u32 {
    let open = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    let flag = e.call(RELOAD_FLAG, &args![]).u8();
    if open != 0 {
        e.vcall(open, 0, &args![1u32]);
    }
    let timed_creator = if fn_00720df0(e) == class {
        Some(CREATE_MAP_MENU)
    } else {
        match class {
            0x40b => Some(CREATE_REPAIR_MENU),
            0x425 => Some(CREATE_ITEM_MOD_MENU),
            0x3ea => Some(CREATE_MENU_3EA),
            0x3eb => Some(CREATE_MENU_3EB),
            _ => None,
        }
    };
    if let Some(create) = timed_creator {
        return reload_with_timer(e, create);
    }
    if class == 0x3ec {
        return e.call(HUD_MAIN_MENU_CREATE, &args![]).u32();
    }
    if e.call(START_MENU_CLASS, &args![]).u32() == class
        && e.call(START_MENU_CREATE, &args![u32::from(flag), 0u32])
            .bool()
    {
        return e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    }
    match class {
        0x3ef => e.call(LOADING_MENU_CREATE, &args![0u32, 1u32, 0u32]).u32(),
        0x403 => e.call(LEVEL_UP_MENU_CREATE, &args![]).u32(),
        0x3f0 => {
            let open = e.call(GET_MENU_BY_CLASS, &args![0x3f0u32]).u32();
            let container = e
                .call(
                    DYNAMIC_CAST,
                    &args![open, 0u32, TILE_TYPE, CONTAINER_MENU_TYPE, 0u32],
                )
                .u32();
            if container == 0 {
                return 0;
            }
            let word = e.call(WORD_AT_74, &args![container]).u32();
            e.call(CONTAINER_MENU_CREATE, &args![word, 1u32]).u32()
        }
        0x402 => {
            let open = e.call(GET_MENU_BY_CLASS, &args![0x3f0u32]).u32();
            let book_menu = e
                .call(
                    DYNAMIC_CAST,
                    &args![open, 0u32, TILE_TYPE, BOOK_MENU_TYPE, 0u32],
                )
                .u32();
            if book_menu == 0 {
                return 0;
            }
            let reference = e.call(WORD_AT_30, &args![book_menu]).u32();
            let object = e.call(WORD_AT_20, &args![reference]).u32();
            let book = e
                .call(
                    DYNAMIC_CAST,
                    &args![object, 0u32, BOUND_OBJECT_TYPE, BOOK_TYPE, 0u32],
                )
                .u32();
            e.vcall(book_menu, 0, &args![1u32]);
            e.call(BOOK_MENU_CREATE, &args![book, reference]).u32()
        }
        _ => 0,
    }
}

// Translated from 00720df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `0x3ff`, the class `ReloadMenu` compares first. Takes no
/// argument (it does not read `ECX`).
pub fn fn_00720df0(_e: &mut Engine) -> u32 {
    0x3ff
}

// Translated from 00720e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::DeleteMenu` (Xbox PDB): deletes the open menu of `class`
/// (vtable slot 0, flag 1) and returns true; false when none is open.
/// `this` is not read.
pub fn menu_manager_delete_menu(e: &mut Engine, _this: Ptr, class: u32) -> bool {
    let open = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if open == 0 {
        return false;
    }
    e.vcall(open, 0, &args![1u32]);
    true
}

// Translated from 00720e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::GetFrontmostMenu` (Xbox PDB): walks the tile list
/// (`tile_list`) and returns the menu of the tile whose menu has a tile
/// (+4) with the largest trait `0xfad` above `-1e8`, among those whose
/// trait `0xfa7` is not `6008.0` (a NaN passes) and whose word at +0x24 is
/// neither 2 nor 4; null when none qualifies. The comparisons are the
/// x87 flag tests (`depth < value`, ordered; `!=` with NaN passing). `this`
/// is not read.
pub fn menu_manager_get_frontmost_menu(e: &mut Engine, _this: Ptr) -> u32 {
    let mut frontmost = 0;
    let mut depth = f32::from_bits(x87_quiet_bits(e.global::<u32>(FRONTMOST_START_VALUE)));
    let list = tile_list(e);
    let mut node = e.call(LIST_HEAD, &args![list]).u32();
    while node != 0 {
        let list = tile_list(e);
        let slot = e.call(LIST_NODE_DATA, &args![list, node]).u32();
        let tile = e.mem.u32(slot);
        let menu = if tile != 0 {
            e.call(TILE_GET_MENU, &args![tile]).u32()
        } else {
            0
        };
        if menu != 0 && word_at_four(e, Ptr::new(menu)) != 0 {
            let menu_tile = word_at_four(e, Ptr::new(menu));
            let value = e.call(TILE_GET_FLOAT, &args![menu_tile, 0xfadu32]).f64();
            if f64::from(depth) < value {
                let menu_tile = word_at_four(e, Ptr::new(menu));
                let kind = e.call(TILE_GET_FLOAT, &args![menu_tile, 0xfa7u32]).f64();
                if kind != e.global::<f64>(FRONTMOST_EXCLUDED_VALUE)
                    && e.call(WORD_AT_24, &args![menu]).u32() != 2
                    && e.call(WORD_AT_24, &args![menu]).u32() != 4
                {
                    let menu_tile = word_at_four(e, Ptr::new(menu));
                    depth = e.call(TILE_GET_FLOAT, &args![menu_tile, 0xfadu32]).f32();
                    frontmost = menu;
                }
            }
        }
        let list = tile_list(e);
        node = e.call(LIST_NEXT, &args![list, node]).u32();
    }
    frontmost
}

// Translated from 00720f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuManager::IsMenuOpen` (Xbox PDB): walks the tile list (advancing a
/// node variable on the stack through `0057cbe0`) to the first tile whose
/// menu has a tile (+4) and whose class (vtable slot +0x34) is `class`;
/// true when that menu's tile has trait `0xfa3` true (`Tile::IsTrue`) and
/// the menu's word at +0x24 is not 2. False when no menu matches. `this`
/// is not read.
pub fn menu_manager_is_menu_open(e: &mut Engine, _this: Ptr, class: u32) -> bool {
    let list = tile_list(e);
    let first = e.call(LIST_HEAD, &args![list]).u32();
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 {
            let list = tile_list(e);
            let slot = e.call(LIST_ADVANCE, &args![list, cursor]).u32();
            let tile = e.mem.u32(slot);
            let menu = if tile != 0 {
                e.call(TILE_GET_MENU, &args![tile]).u32()
            } else {
                0
            };
            if menu != 0
                && word_at_four(e, Ptr::new(menu)) != 0
                && e.vcall(menu, 0x34, &args![]).u32() == class
            {
                let menu_tile = word_at_four(e, Ptr::new(menu));
                return e.call(TILE_IS_TRUE, &args![menu_tile, 0xfa3u32]).bool()
                    && e.call(WORD_AT_24, &args![menu]).u32() != 2;
            }
        }
        false
    })
}

// Translated from 00721060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<ActorValue::Index>` constructor (`list_box_construct`).
/// Returns `this`.
// Translation incomplete: 00721060 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_00721060(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_ACTOR_VALUE)
}

// Translated from 00721130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl (list box): passes the address of its argument (a stack word
/// holding the list box) to `005ae3d0` on the object at `011d8b54`. Every
/// list box constructor of this unit calls it.
pub fn fn_00721130(e: &mut Engine, list_box: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), list_box);
        e.call(LIST_BOX_COLLECTION_ADD, &args![LIST_BOX_COLLECTION, slot]);
    });
}

// Translated from 00721150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<ActorValue::Index>` destructor (`list_box_destruct`).
// Translation incomplete: 00721150 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_00721150(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_ACTOR_VALUE);
}

// Translated from 007211c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl (list box): passes the address of its argument (a stack word
/// holding the list box) to `00905330` on the object at `011d8b54`. Every
/// list box destructor of this unit calls it.
pub fn fn_007211c0(e: &mut Engine, list_box: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), list_box);
        e.call(
            LIST_BOX_COLLECTION_REMOVE,
            &args![LIST_BOX_COLLECTION, slot],
        );
    });
}

// Translated from 007211e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::RemoveAll` (Xbox PDB):
/// `list_box_remove_all`.
pub fn list_box_actor_value_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_ACTOR_VALUE);
}

// Translated from 00721410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::Sort` (Xbox PDB): `list_box_sort`.
pub fn list_box_actor_value_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_ACTOR_VALUE, compare);
}

// Translated from 00721440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::Highlight` (Xbox PDB):
/// `list_box_highlight`.
pub fn list_box_actor_value_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_ACTOR_VALUE, tile)
}

// Translated from 007214b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::GetHighlightTarget` (Xbox PDB):
/// `list_box_get_highlight_target`.
pub fn list_box_actor_value_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_ACTOR_VALUE, direction)
}

// Translated from 00721850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<PerkRankData *>` constructor (`list_box_construct`).
/// Returns `this`.
// Translation incomplete: 00721850 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_00721850(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_PERK_RANK_DATA)
}

// Translated from 00721920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<PerkRankData *>` destructor (`list_box_destruct`).
// Translation incomplete: 00721920 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_00721920(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_PERK_RANK_DATA);
}

// Translated from 00721990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::RemoveAll` (Xbox PDB): `list_box_remove_all`.
pub fn list_box_perk_rank_data_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_PERK_RANK_DATA);
}

// Translated from 00721bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::Sort` (Xbox PDB): `list_box_sort`.
pub fn list_box_perk_rank_data_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_PERK_RANK_DATA, compare);
}

// Translated from 00721bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::Highlight` (Xbox PDB): `list_box_highlight`.
pub fn list_box_perk_rank_data_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_PERK_RANK_DATA, tile)
}

// Translated from 00721c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::GetHighlightTarget` (Xbox PDB):
/// `list_box_get_highlight_target`.
pub fn list_box_perk_rank_data_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_PERK_RANK_DATA, direction)
}

// Translated from 00722000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<MiscStatManager::MiscStatID>` constructor
/// (`list_box_construct`). Returns `this`.
// Translation incomplete: 00722000 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_00722000(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_MISC_STAT_ID)
}

// Translated from 007220d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<MiscStatManager::MiscStatID>` destructor
/// (`list_box_destruct`).
// Translation incomplete: 007220d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_007220d0(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_MISC_STAT_ID);
}

// Translated from 00722140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::RemoveAll` (Xbox PDB):
/// `list_box_remove_all`.
pub fn list_box_misc_stat_id_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_MISC_STAT_ID);
}

// Translated from 00722370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::Sort` (Xbox PDB):
/// `list_box_sort`.
pub fn list_box_misc_stat_id_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_MISC_STAT_ID, compare);
}

// Translated from 007223a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::Highlight` (Xbox PDB):
/// `list_box_highlight`.
pub fn list_box_misc_stat_id_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_MISC_STAT_ID, tile)
}

// Translated from 00722410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::GetHighlightTarget` (Xbox
/// PDB): `list_box_get_highlight_target`.
pub fn list_box_misc_stat_id_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_MISC_STAT_ID, direction)
}

// Translated from 007227b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<StatsMenu::StatusEffect *>` constructor
/// (`list_box_construct`). Returns `this`.
// Translation incomplete: 007227b0 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_007227b0(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_STATUS_EFFECT)
}

// Translated from 00722880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<StatsMenu::StatusEffect *>` destructor
/// (`list_box_destruct`).
// Translation incomplete: 00722880 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_00722880(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_STATUS_EFFECT);
}

// Translated from 007228f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::RemoveAll` (Xbox PDB):
/// `list_box_remove_all`.
pub fn list_box_status_effect_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_STATUS_EFFECT);
}

// Translated from 00722b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::Sort` (Xbox PDB): `list_box_sort`.
pub fn list_box_status_effect_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_STATUS_EFFECT, compare);
}

// Translated from 00722b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::Highlight` (Xbox PDB):
/// `list_box_highlight`.
pub fn list_box_status_effect_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_STATUS_EFFECT, tile)
}

// Translated from 00722bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::GetHighlightTarget` (Xbox PDB):
/// `list_box_get_highlight_target`.
pub fn list_box_status_effect_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_STATUS_EFFECT, direction)
}

// Translated from 00722f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<TESReputation *>` constructor (`list_box_construct`).
/// Returns `this`.
// Translation incomplete: 00722f60 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_00722f60(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_TES_REPUTATION)
}

// Translated from 00723070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<TESReputation *>` destructor (`list_box_destruct`).
// Translation incomplete: 00723070 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_00723070(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_TES_REPUTATION);
}

// Translated from 007230e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::RemoveAll` (Xbox PDB): `list_box_remove_all`.
pub fn list_box_tes_reputation_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_TES_REPUTATION);
}

// Translated from 00723310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::Sort` (Xbox PDB): `list_box_sort`.
pub fn list_box_tes_reputation_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_TES_REPUTATION, compare);
}

// Translated from 00723340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::Highlight` (Xbox PDB): `list_box_highlight`.
pub fn list_box_tes_reputation_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_TES_REPUTATION, tile)
}

// Translated from 007233b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::GetHighlightTarget` (Xbox PDB):
/// `list_box_get_highlight_target`.
pub fn list_box_tes_reputation_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_TES_REPUTATION, direction)
}

// Translated from 00723750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<unsigned int>` constructor (`list_box_construct`).
/// Returns `this`.
// Translation incomplete: 00723750 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a later step throws is not translated
pub fn fn_00723750(e: &mut Engine, this: Ptr<ListBox>) -> Ptr<ListBox> {
    list_box_construct(e, this, LIST_BOX_UNSIGNED_INT)
}

// Translated from 00723820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBox<unsigned int>` destructor (`list_box_destruct`).
// Translation incomplete: 00723820 - the C++ exception-unwinding frame (FS:[0]) that destroys the embedded list when a step throws is not translated
pub fn fn_00723820(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_destruct(e, this, LIST_BOX_UNSIGNED_INT);
}

// Translated from 00723890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::RemoveAll` (Xbox PDB): `list_box_remove_all`.
pub fn list_box_unsigned_int_remove_all(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_remove_all(e, this, LIST_BOX_UNSIGNED_INT);
}

// Translated from 00723ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::Sort` (Xbox PDB): `list_box_sort`.
pub fn list_box_unsigned_int_sort(e: &mut Engine, this: Ptr<ListBox>, compare: u32) {
    list_box_sort(e, this, LIST_BOX_UNSIGNED_INT, compare);
}

// Translated from 00723af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::Highlight` (Xbox PDB): `list_box_highlight`.
pub fn list_box_unsigned_int_highlight(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    list_box_highlight(e, this, LIST_BOX_UNSIGNED_INT, tile)
}

// Translated from 00723b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::GetHighlightTarget` (Xbox PDB):
/// `list_box_get_highlight_target`.
pub fn list_box_unsigned_int_get_highlight_target(
    e: &mut Engine,
    this: Ptr<ListBox>,
    direction: u32,
) -> u32 {
    list_box_get_highlight_target(e, this, LIST_BOX_UNSIGNED_INT, direction)
}

// Translated from 00723f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SPECIALBookMenu::SpecialBookTextItem *>` constructor
/// (`simple_array_construct`). Returns `this`.
pub fn fn_00723f00(e: &mut Engine, this: Ptr) -> Ptr {
    simple_array_construct(e, this, BOOK_TEXT_ITEMS_VTABLE)
}

// Translated from 00723f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SPECIALBookMenu::SpecialBookTextItem *>` destructor
/// (`simple_array_destruct`).
pub fn fn_00723f30(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this, BOOK_TEXT_ITEMS_VTABLE);
}

// Translated from 00723f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SPECIALBookMenu::SpecialBookButton *>` constructor
/// (`simple_array_construct`). Returns `this`.
pub fn fn_00723f50(e: &mut Engine, this: Ptr) -> Ptr {
    simple_array_construct(e, this, BOOK_BUTTONS_VTABLE)
}

// Translated from 00723f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SPECIALBookMenu::SpecialBookButton *>` destructor
/// (`simple_array_destruct`).
pub fn fn_00723f80(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this, BOOK_BUTTONS_VTABLE);
}

// Translated from 00723fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterTextItem *>` constructor
/// (`simple_array_construct`). Returns `this`.
pub fn fn_00723fa0(e: &mut Engine, this: Ptr) -> Ptr {
    simple_array_construct(e, this, LOVE_TESTER_TEXT_ITEMS_VTABLE)
}

// Translated from 00723fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterTextItem *>` destructor
/// (`simple_array_destruct`).
pub fn fn_00723fd0(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this, LOVE_TESTER_TEXT_ITEMS_VTABLE);
}

// Translated from 00723ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterLightBulbGlows *>`
/// constructor (`simple_array_construct`). Returns `this`.
pub fn fn_00723ff0(e: &mut Engine, this: Ptr) -> Ptr {
    simple_array_construct(e, this, LOVE_TESTER_GLOWS_VTABLE)
}

// Translated from 00724020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterLightBulbGlows *>`
/// destructor (`simple_array_destruct`).
pub fn fn_00724020(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this, LOVE_TESTER_GLOWS_VTABLE);
}

// Translated from 00724040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterButton *>` constructor
/// (`simple_array_construct`). Returns `this`.
pub fn fn_00724040(e: &mut Engine, this: Ptr) -> Ptr {
    simple_array_construct(e, this, LOVE_TESTER_BUTTONS_VTABLE)
}

// Translated from 00724070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<LoveTesterMenu::LoveTesterButton *>` destructor
/// (`simple_array_destruct`).
pub fn fn_00724070(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this, LOVE_TESTER_BUTTONS_VTABLE);
}

// Translated from 00724090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor (`fn_00721150`) and frees `this` when bit 0
/// of `flags` is set.
pub fn list_box_actor_value_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00721150(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 007240c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ListBoxBase` constructor: the vtable (RTTI `.?AVListBoxBase@@`).
/// Returns `this`.
pub fn fn_007240c0(e: &mut Engine, this: Ptr) -> Ptr {
    set_vtable(e, this, LIST_BOX_BASE_VTABLE);
    this
}

// Translated from 007240e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor (`fn_00721920`) and frees `this` when bit 0 of
/// `flags` is set.
pub fn list_box_perk_rank_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00721920(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 00724110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_007220d0`) and frees `this` when
/// bit 0 of `flags` is set.
pub fn list_box_misc_stat_id_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_007220d0(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 00724140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00722880`) and frees `this` when
/// bit 0 of `flags` is set.
pub fn list_box_status_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00722880(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 00724170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor (`fn_00723070`) and frees `this` when bit 0 of
/// `flags` is set.
pub fn list_box_tes_reputation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00723070(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 007241a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor (`fn_00723820`) and frees `this` when bit 0 of `flags`
/// is set.
pub fn list_box_unsigned_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ListBox>,
    flags: u32,
) -> Ptr<ListBox> {
    fn_00723820(e, this);
    free_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 007241d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SPECIALBookMenu::SpecialBookTextItem *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00723f30`) and frees `this` when bit 0
/// of `flags` is set.
pub fn simple_array_special_book_text_item_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00723f30(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00724200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SPECIALBookMenu::SpecialBookButton *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00723f80`) and frees `this` when bit 0
/// of `flags` is set.
pub fn simple_array_special_book_button_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00723f80(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00724230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LoveTesterMenu::LoveTesterTextItem *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00723fd0`) and frees `this` when bit 0
/// of `flags` is set.
pub fn simple_array_love_tester_text_item_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00723fd0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00724260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LoveTesterMenu::LoveTesterLightBulbGlows *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00724020`) and frees `this` when bit 0
/// of `flags` is set.
pub fn simple_array_love_tester_light_bulb_glows_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00724020(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00724290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<LoveTesterMenu::LoveTesterButton *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`fn_00724070`) and frees `this` when bit 0
/// of `flags` is set.
pub fn simple_array_love_tester_button_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00724070(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 007242c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `bit` (the shift count masked to 5 bits, as `SHL` does) of
/// the list box's `usFlags` is set.
pub fn fn_007242c0(e: &mut Engine, this: Ptr<ListBox>, bit: u32) -> bool {
    let flags = u32::from(e.get(this, ListBox::usFlags));
    flags & (1u32 << (bit & 0x1f)) != 0
}

// Translated from 007242f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an item of the list box has `tile` as its tile: walks the
/// nodes from the first while the node and its item are non-null.
pub fn fn_007242f0(e: &mut Engine, this: Ptr<ListBox>, tile: u32) -> bool {
    let mut node = first_node(this);
    while node != 0 && node_item(e, node) != 0 {
        let item = node_item(e, node);
        if e.mem.u32(item) == tile {
            return true;
        }
        node = word_at_four(e, Ptr::new(node));
    }
    false
}

// Translated from 00724360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_actor_value_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_ACTOR_VALUE);
}

// Translated from 00724430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 00724430 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_actor_value_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_ACTOR_VALUE)
}

// Translated from 007244e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 007244e0 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_actor_value_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_ACTOR_VALUE);
}

// Translated from 00724800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_actor_value_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_ACTOR_VALUE);
}

// Translated from 00724930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_perk_rank_data_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_PERK_RANK_DATA);
}

// Translated from 00724a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 00724a00 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_perk_rank_data_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_PERK_RANK_DATA)
}

// Translated from 00724ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 00724ab0 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_perk_rank_data_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_PERK_RANK_DATA);
}

// Translated from 00724dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_perk_rank_data_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_PERK_RANK_DATA);
}

// Translated from 00724f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_misc_stat_id_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_MISC_STAT_ID);
}

// Translated from 00724fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 00724fd0 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_misc_stat_id_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_MISC_STAT_ID)
}

// Translated from 00725080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 00725080 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_misc_stat_id_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_MISC_STAT_ID);
}

// Translated from 007253a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_misc_stat_id_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_MISC_STAT_ID);
}

// Translated from 007254d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_status_effect_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_STATUS_EFFECT);
}

// Translated from 007255a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 007255a0 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_status_effect_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_STATUS_EFFECT)
}

// Translated from 00725650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 00725650 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_status_effect_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_STATUS_EFFECT);
}

// Translated from 00725970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_status_effect_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_STATUS_EFFECT);
}

// Translated from 00725aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_tes_reputation_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_TES_REPUTATION);
}

// Translated from 00725b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 00725b70 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_tes_reputation_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_TES_REPUTATION)
}

// Translated from 00725c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 00725c20 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_tes_reputation_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_TES_REPUTATION);
}

// Translated from 00725f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_tes_reputation_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_TES_REPUTATION);
}

// Translated from 00726070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the word at +4. Folded callers include a list's next-node pointer
/// and an inventory entry's signed count; the accessor preserves all bits.
pub fn word_at_four(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(4))
}

// Translated from 00726090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::Synchronize` (Xbox PDB): `list_box_synchronize`.
pub fn list_box_unsigned_int_synchronize(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_synchronize(e, this, LIST_BOX_UNSIGNED_INT);
}

// Translated from 00726160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::IsEnabled` (Xbox PDB): `list_box_is_enabled`.
// Translation incomplete: 00726160 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_unsigned_int_is_enabled(e: &mut Engine, this: Ptr<ListBox>) -> bool {
    list_box_is_enabled(e, this, LIST_BOX_UNSIGNED_INT)
}

// Translated from 00726210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::UpdateHighlightBoxPos` (Xbox PDB):
/// `list_box_update_highlight_box_pos`.
// Translation incomplete: 00726210 - the C++ exception-unwinding frame (FS:[0]) around the static trait ids' initialization is not translated
pub fn list_box_unsigned_int_update_highlight_box_pos(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_update_highlight_box_pos(e, this, LIST_BOX_UNSIGNED_INT);
}

// Translated from 00726530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::SetTotalListHeight` (Xbox PDB):
/// `list_box_set_total_list_height`.
pub fn list_box_unsigned_int_set_total_list_height(e: &mut Engine, this: Ptr<ListBox>) {
    list_box_set_total_list_height(e, this, LIST_BOX_UNSIGNED_INT);
}

// Translated from 00726660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum ActorValue::Index>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 00726660 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_actor_value_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_ACTOR_VALUE)
}

// Translated from 00726710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<PerkRankData *>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 00726710 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_perk_rank_data_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_PERK_RANK_DATA)
}

// Translated from 007267c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<enum MiscStatManager::MiscStatID>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 007267c0 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_misc_stat_id_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_MISC_STAT_ID)
}

// Translated from 00726870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<StatsMenu::StatusEffect *>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 00726870 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_status_effect_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_STATUS_EFFECT)
}

// Translated from 00726920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<TESReputation *>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 00726920 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_tes_reputation_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_TES_REPUTATION)
}

// Translated from 00726b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ListBox<unsigned int>::NumUnfiltered` (Xbox PDB): `list_box_num_unfiltered`.
// Translation incomplete: 00726b40 - the C++ exception-unwinding frame (FS:[0]) around the static trait id's initialization is not translated
pub fn list_box_unsigned_int_num_unfiltered(e: &mut Engine, this: Ptr<ListBox>) -> i32 {
    list_box_num_unfiltered(e, this, LIST_BOX_UNSIGNED_INT)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0071e290, menu_manager_instance(bool) -> u32),
        entry!(0x0071e340, fn_0071e340(Ptr) -> Ptr),
        entry!(0x0071e360, fn_0071e360()),
        entry!(0x0071e3b0, fn_0071e3b0(Ptr, u32) -> Ptr),
        entry!(0x0071e3e0, fn_0071e3e0(Ptr)),
        entry!(0x0071e3f0, fn_0071e3f0()),
        entry!(0x0071e400, fn_0071e400(Ptr)),
        entry!(
            0x0071e420,
            menu_manager_create_menu_by_class(Ptr, u32) -> u32
        ),
        entry!(
            0x0071f240,
            fn_0071f240(Ptr<TutorialMenu>) -> Ptr<TutorialMenu>
        ),
        entry!(
            0x0071f2d0,
            tutorial_menu_get_class(Ptr<TutorialMenu>) -> u32
        ),
        entry!(
            0x0071f2e0,
            tutorial_menu_scalar_deleting_destructor(Ptr<TutorialMenu>, u32) -> Ptr<TutorialMenu>
        ),
        entry!(0x0071f310, fn_0071f310(Ptr<TutorialMenu>)),
        entry!(0x0071f340, fn_0071f340(Ptr) -> Ptr),
        entry!(0x0071f4a0, fn_0071f4a0(Ptr) -> Ptr),
        entry!(0x0071f4e0, fn_0071f4e0(Ptr)),
        entry!(0x0071f520, fn_0071f520(Ptr) -> bool),
        entry!(0x0071f540, fn_0071f540(Ptr) -> Ptr),
        entry!(0x0071f570, fn_0071f570(Ptr)),
        entry!(0x0071f5d0, fn_0071f5d0(Ptr) -> Ptr),
        entry!(0x0071f600, fn_0071f600(Ptr)),
        entry!(0x0071f660, stats_menu_get_class(Ptr) -> u32),
        entry!(
            0x0071f670,
            stats_menu_named_effects_list_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0071f6a0,
            stats_menu_status_effect_list_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0071f6d0,
            stats_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0071f700, fn_0071f700(Ptr)),
        entry!(0x0071f8c0, fn_0071f8c0()),
        entry!(0x0071f900, fn_0071f900(Ptr, u32) -> Ptr),
        entry!(0x0071f930, fn_0071f930(Ptr)),
        entry!(0x0071f9c0, fn_0071f9c0(Ptr, u32) -> Ptr),
        entry!(0x0071f9f0, fn_0071f9f0(Ptr)),
        entry!(0x0071fa50, stats_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(
            0x0071fa80,
            fn_0071fa80(Ptr<TextEditMenu>) -> Ptr<TextEditMenu>
        ),
        entry!(0x0071fb20, fn_0071fb20(Ptr<TextEditMenu>) -> u32),
        entry!(
            0x0071fb30,
            fn_0071fb30(Ptr<TextEditMenu>, u32) -> Ptr<TextEditMenu>
        ),
        entry!(0x0071fb60, fn_0071fb60(Ptr<TextEditMenu>)),
        entry!(0x0071fbc0, fn_0071fbc0(Ptr<TextEditMenu>, i32, u32)),
        entry!(0x0071fbf0, fn_0071fbf0(Ptr) -> Ptr),
        entry!(0x0071feb0, surgery_menu_get_class(Ptr) -> u32),
        entry!(
            0x0071fec0,
            surgery_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0071fef0, fn_0071fef0(Ptr)),
        entry!(0x0071ffe0, surgery_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x00720010, fn_00720010(Ptr, i32) -> i32),
        entry!(0x00720040, fn_00720040(Ptr) -> Ptr),
        entry!(0x00720100, vats_menu_get_class(Ptr) -> u32),
        entry!(
            0x00720110,
            vats_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00720140, vats_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x00720170, fn_00720170(Ptr) -> Ptr),
        entry!(0x00720320, special_book_menu_get_class(Ptr) -> u32),
        entry!(
            0x00720330,
            special_book_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00720360, fn_00720360(Ptr) -> Ptr),
        entry!(0x007203c0, fn_007203c0(Ptr)),
        entry!(0x00720420, fn_00720420(Ptr)),
        entry!(0x00720550, fn_00720550(Ptr) -> Ptr),
        entry!(0x00720760, love_tester_menu_get_class(Ptr) -> u32),
        entry!(
            0x00720770,
            love_tester_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x007207a0, fn_007207a0(Ptr) -> Ptr),
        entry!(0x00720800, fn_00720800(Ptr)),
        entry!(0x00720860, fn_00720860(Ptr)),
        entry!(0x007209e0, tutorial_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(0x00720a10, fn_00720a10(Ptr) -> Ptr),
        entry!(0x00720a30, quantity_menu_get_class(Ptr) -> u32),
        entry!(0x00720a40, quantity_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00720a70, fn_00720a70(Ptr)),
        entry!(0x00720aa0, menu_manager_reload_menu(Ptr, u32) -> u32),
        entry!(0x00720df0, fn_00720df0() -> u32),
        entry!(0x00720e00, menu_manager_delete_menu(Ptr, u32) -> bool),
        entry!(0x00720e60, menu_manager_get_frontmost_menu(Ptr) -> u32),
        entry!(0x00720f90, menu_manager_is_menu_open(Ptr, u32) -> bool),
        entry!(0x00721060, fn_00721060(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x00721130, fn_00721130(u32)),
        entry!(0x00721150, fn_00721150(Ptr<ListBox>)),
        entry!(0x007211c0, fn_007211c0(u32)),
        entry!(0x007211e0, list_box_actor_value_remove_all(Ptr<ListBox>)),
        entry!(0x00721410, list_box_actor_value_sort(Ptr<ListBox>, u32)),
        entry!(
            0x00721440,
            list_box_actor_value_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007214b0,
            list_box_actor_value_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x00721850, fn_00721850(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x00721920, fn_00721920(Ptr<ListBox>)),
        entry!(0x00721990, list_box_perk_rank_data_remove_all(Ptr<ListBox>)),
        entry!(0x00721bc0, list_box_perk_rank_data_sort(Ptr<ListBox>, u32)),
        entry!(
            0x00721bf0,
            list_box_perk_rank_data_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x00721c60,
            list_box_perk_rank_data_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x00722000, fn_00722000(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x007220d0, fn_007220d0(Ptr<ListBox>)),
        entry!(0x00722140, list_box_misc_stat_id_remove_all(Ptr<ListBox>)),
        entry!(0x00722370, list_box_misc_stat_id_sort(Ptr<ListBox>, u32)),
        entry!(
            0x007223a0,
            list_box_misc_stat_id_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x00722410,
            list_box_misc_stat_id_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x007227b0, fn_007227b0(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x00722880, fn_00722880(Ptr<ListBox>)),
        entry!(0x007228f0, list_box_status_effect_remove_all(Ptr<ListBox>)),
        entry!(0x00722b20, list_box_status_effect_sort(Ptr<ListBox>, u32)),
        entry!(
            0x00722b50,
            list_box_status_effect_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x00722bc0,
            list_box_status_effect_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x00722f60, fn_00722f60(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x00723070, fn_00723070(Ptr<ListBox>)),
        entry!(0x007230e0, list_box_tes_reputation_remove_all(Ptr<ListBox>)),
        entry!(0x00723310, list_box_tes_reputation_sort(Ptr<ListBox>, u32)),
        entry!(
            0x00723340,
            list_box_tes_reputation_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x007233b0,
            list_box_tes_reputation_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x00723750, fn_00723750(Ptr<ListBox>) -> Ptr<ListBox>),
        entry!(0x00723820, fn_00723820(Ptr<ListBox>)),
        entry!(0x00723890, list_box_unsigned_int_remove_all(Ptr<ListBox>)),
        entry!(0x00723ac0, list_box_unsigned_int_sort(Ptr<ListBox>, u32)),
        entry!(
            0x00723af0,
            list_box_unsigned_int_highlight(Ptr<ListBox>, u32) -> bool
        ),
        entry!(
            0x00723b60,
            list_box_unsigned_int_get_highlight_target(Ptr<ListBox>, u32) -> u32
        ),
        entry!(0x00723f00, fn_00723f00(Ptr) -> Ptr),
        entry!(0x00723f30, fn_00723f30(Ptr)),
        entry!(0x00723f50, fn_00723f50(Ptr) -> Ptr),
        entry!(0x00723f80, fn_00723f80(Ptr)),
        entry!(0x00723fa0, fn_00723fa0(Ptr) -> Ptr),
        entry!(0x00723fd0, fn_00723fd0(Ptr)),
        entry!(0x00723ff0, fn_00723ff0(Ptr) -> Ptr),
        entry!(0x00724020, fn_00724020(Ptr)),
        entry!(0x00724040, fn_00724040(Ptr) -> Ptr),
        entry!(0x00724070, fn_00724070(Ptr)),
        entry!(
            0x00724090,
            list_box_actor_value_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(0x007240c0, fn_007240c0(Ptr) -> Ptr),
        entry!(
            0x007240e0,
            list_box_perk_rank_data_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(
            0x00724110,
            list_box_misc_stat_id_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(
            0x00724140,
            list_box_status_effect_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(
            0x00724170,
            list_box_tes_reputation_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(
            0x007241a0,
            list_box_unsigned_int_scalar_deleting_destructor(Ptr<ListBox>, u32) -> Ptr<ListBox>
        ),
        entry!(0x007241d0, simple_array_special_book_text_item_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00724200, simple_array_special_book_button_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00724230, simple_array_love_tester_text_item_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00724260, simple_array_love_tester_light_bulb_glows_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00724290, simple_array_love_tester_button_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x007242c0, fn_007242c0(Ptr<ListBox>, u32) -> bool),
        entry!(0x007242f0, fn_007242f0(Ptr<ListBox>, u32) -> bool),
        entry!(0x00724360, list_box_actor_value_synchronize(Ptr<ListBox>)),
        entry!(
            0x00724430,
            list_box_actor_value_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x007244e0,
            list_box_actor_value_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x00724800,
            list_box_actor_value_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x00724930,
            list_box_perk_rank_data_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x00724a00,
            list_box_perk_rank_data_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x00724ab0,
            list_box_perk_rank_data_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x00724dd0,
            list_box_perk_rank_data_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x00724f00, list_box_misc_stat_id_synchronize(Ptr<ListBox>)),
        entry!(
            0x00724fd0,
            list_box_misc_stat_id_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x00725080,
            list_box_misc_stat_id_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x007253a0,
            list_box_misc_stat_id_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x007254d0, list_box_status_effect_synchronize(Ptr<ListBox>)),
        entry!(
            0x007255a0,
            list_box_status_effect_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x00725650,
            list_box_status_effect_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x00725970,
            list_box_status_effect_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x00725aa0,
            list_box_tes_reputation_synchronize(Ptr<ListBox>)
        ),
        entry!(
            0x00725b70,
            list_box_tes_reputation_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x00725c20,
            list_box_tes_reputation_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x00725f40,
            list_box_tes_reputation_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(0x00726070, word_at_four(Ptr) -> u32),
        entry!(0x00726090, list_box_unsigned_int_synchronize(Ptr<ListBox>)),
        entry!(
            0x00726160,
            list_box_unsigned_int_is_enabled(Ptr<ListBox>) -> bool
        ),
        entry!(
            0x00726210,
            list_box_unsigned_int_update_highlight_box_pos(Ptr<ListBox>)
        ),
        entry!(
            0x00726530,
            list_box_unsigned_int_set_total_list_height(Ptr<ListBox>)
        ),
        entry!(
            0x00726660,
            list_box_actor_value_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x00726710,
            list_box_perk_rank_data_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x007267c0,
            list_box_misc_stat_id_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x00726870,
            list_box_status_effect_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x00726920,
            list_box_tes_reputation_num_unfiltered(Ptr<ListBox>) -> i32
        ),
        entry!(
            0x00726b40,
            list_box_unsigned_int_num_unfiltered(Ptr<ListBox>) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;
    type Sequence = Vec<(u32, Vec<u32>)>;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Every callee outside this file answers 0; the allocator hands out
    /// heap blocks; the globals the file uses are mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut externals = vec![
            FREE,
            MEMSET,
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            DEBUG_MESSAGE,
            MENU_CONSTRUCT,
            MENU_DESTRUCT,
            MENU_TABLE_GROW,
            ITEM_DELETE,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            LIST_CLEAR,
            NAMED_EFFECTS_CLEAR,
            STATUS_EFFECTS_CLEAR,
            STATUS_DATA_FLUSH,
            TEXT_EDITOR_CONSTRUCT,
            TEXT_EDITOR_DESTRUCT,
            STRING_SET,
            RETURN_THIS,
            SOUND_HANDLE_CONSTRUCT,
            EMPTY_FUNCTION,
            POINTER_CONSTRUCT_DEFAULT,
            POINTER_CONSTRUCT,
            POINTER_ASSIGN,
            POINTER_DESTRUCT,
            VATS_ACTION_LIST_CONSTRUCT,
            VATS_COMMAND_CONSTRUCT,
            VATS_MENU_DESTRUCT,
            BOOK_BUTTONS_CONSTRUCT,
            BOOK_TEXT_ITEMS_CONSTRUCT,
            BOOK_TEXT_ITEMS_DESTRUCT,
            BOOK_BUTTONS_DESTRUCT,
            LOVE_TESTER_BUTTONS_CONSTRUCT,
            LOVE_TESTER_TEXT_ITEMS_CONSTRUCT,
            LOVE_TESTER_TEXT_ITEMS_DESTRUCT,
            LOVE_TESTER_BUTTONS_DESTRUCT,
            LOVE_TESTER_GLOWS_CONSTRUCT,
            LOVE_TESTER_GLOWS_DESTRUCT,
        ];
        externals.extend(STATS_LIST_BOX_CONSTRUCTORS.iter().map(|(_, a)| *a));
        externals.extend(STATS_LIST_BOX_DESTRUCTORS.iter().map(|(_, a)| *a));
        for addr in externals {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        for global in [
            MENU_MANAGER,
            TUTORIAL_MENU_GLOBAL,
            STATS_MENU_GLOBAL,
            BODY_PART_CONTROLLER_GLOBAL,
        ] {
            e.map(global, 4);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The logged calls to any of `addrs`, in order.
    fn sequence(e: &Engine, addrs: &[u32]) -> Sequence {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| addrs.contains(called))
            .cloned()
            .collect()
    }

    /// A heap block of `size` bytes filled with `0xAA`.
    fn filled(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.write(block, &vec![0xaa; size as usize]);
        block
    }

    fn grow_call() -> Vec<u32> {
        vec![MENU_TABLE, MENU_TABLE_COUNT]
    }

    /// Calls the deleting destructor at `addr` with flags 0, then 1: frees
    /// only the second time and returns `this` both times.
    fn check_deleting_destructor(e: &mut Engine, addr: u32, this: u32) {
        assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
        assert!(calls(e, FREE).is_empty());
        assert_eq!(e.call(addr, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(e, FREE), vec![vec![this]]);
    }

    #[test]
    fn instance_creates_once_and_only_when_asked() {
        let mut e = engine();
        assert_eq!(e.call(0x0071_e290, &args![false]).u32(), 0);
        assert!(calls(&e, ALLOCATE).is_empty());
        let manager = e.call(0x0071_e290, &args![true]).u32();
        assert_ne!(manager, 0);
        assert_eq!(e.global::<u32>(MENU_MANAGER), manager);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![1]]);
        // The constructor and the instance function both run the body.
        assert_eq!(calls(&e, MENU_TABLE_GROW), vec![grow_call(), grow_call()]);
        assert_eq!(e.call(0x0071_e290, &args![true]).u32(), manager);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn instance_with_failed_allocation_stays_null() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x0071_e290, &args![true]).u32(), 0);
        assert_eq!(e.global::<u32>(MENU_MANAGER), 0);
        assert_eq!(calls(&e, MENU_TABLE_GROW), vec![grow_call()]);
    }

    #[test]
    fn manager_constructor_runs_the_body() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0071_e340, &args![this]).u32(), this);
        assert_eq!(calls(&e, MENU_TABLE_GROW), vec![grow_call()]);
    }

    #[test]
    fn manager_shutdown_deletes_the_instance() {
        let mut e = engine();
        e.call(0x0071_e360, &args![]);
        assert!(calls(&e, FREE).is_empty());
        let manager = e.mem.alloc(4);
        e.set_global(MENU_MANAGER, manager);
        e.call(0x0071_e360, &args![]);
        assert_eq!(calls(&e, FREE), vec![vec![manager]]);
        assert_eq!(e.global::<u32>(MENU_MANAGER), 0);
    }

    #[test]
    fn manager_deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.set_global(MENU_MANAGER, 0x1234u32);
        assert_eq!(e.call(0x0071_e3b0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.global::<u32>(MENU_MANAGER), 0);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0071_e3b0, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn manager_destructor_clears_the_global() {
        let mut e = engine();
        e.set_global(MENU_MANAGER, 0x1234u32);
        e.call(0x0071_e3e0, &args![0x5678u32]);
        assert_eq!(e.global::<u32>(MENU_MANAGER), 0);
    }

    #[test]
    fn clear_instance_global() {
        let mut e = engine();
        e.set_global(MENU_MANAGER, 0x1234u32);
        e.call(0x0071_e3f0, &args![]);
        assert_eq!(e.global::<u32>(MENU_MANAGER), 0);
    }

    #[test]
    fn manager_constructor_body_grows_the_table() {
        let mut e = engine();
        e.call(0x0071_e400, &args![0u32]);
        assert_eq!(calls(&e, MENU_TABLE_GROW), vec![grow_call()]);
    }

    #[test]
    fn create_menu_by_class_allocates_and_constructs() {
        let mut e = engine();
        for (_, constructor) in (0x3e9..=0x43c).filter_map(menu_class) {
            e.register(constructor, |_, a| word(a[0] + 1));
        }
        let menu = e.call(0x0071_e420, &args![0u32, 0x3e9u32]).u32();
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls(&e, 0x007a_8a10), vec![vec![menu - 1]]);
        let menu = e.call(0x0071_e420, &args![0u32, 0x43au32]).u32();
        assert_eq!(calls(&e, ALLOCATE)[1], vec![0x4014]);
        assert_eq!(calls(&e, 0x007b_8d30), vec![vec![menu - 1]]);
        let menu = e.call(0x0071_e420, &args![0u32, 0x423u32]).u32();
        assert_eq!(calls(&e, ALLOCATE)[2], vec![0x54]);
        assert_eq!(calls(&e, 0x0071_f240), vec![vec![menu - 1]]);
        assert!(calls(&e, DEBUG_MESSAGE).is_empty());
    }

    #[test]
    fn create_menu_by_class_rejects_unknown_classes() {
        let mut e = engine();
        for class in [0x3e8u32, 0x3ed, 0x43d, 0, 0xffff_ffff] {
            assert_eq!(e.call(0x0071_e420, &args![0u32, class]).u32(), 0);
        }
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(
            calls(&e, DEBUG_MESSAGE),
            vec![vec![UNKNOWN_MENU_CLASS_TEXT]; 5]
        );
    }

    #[test]
    fn create_menu_by_class_with_failed_allocation_returns_null() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        e.register(0x0075_f070, |_, _| word(0xdead));
        assert_eq!(e.call(0x0071_e420, &args![0u32, 0x417u32]).u32(), 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x80]]);
        assert!(calls(&e, 0x0075_f070).is_empty());
    }

    #[test]
    fn tutorial_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x54);
        assert_eq!(e.call(0x0071_f240, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TUTORIAL_MENU_VTABLE);
        assert_eq!(calls(&e, MENU_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x28, 0, 0x20]]);
        for offset in [0x48, 0x4c, 0x50] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn tutorial_menu_get_class_is_0x423() {
        let mut e = engine();
        assert_eq!(e.call(0x0071_f2d0, &args![0u32]).u32(), 0x423);
    }

    #[test]
    fn tutorial_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        check_deleting_destructor(&mut e, 0x0071_f2e0, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn tutorial_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.set_global(TUTORIAL_MENU_GLOBAL, this);
        e.call(0x0071_f310, &args![this]);
        assert_eq!(e.mem.u32(this), TUTORIAL_MENU_VTABLE);
        assert_eq!(e.global::<u32>(TUTORIAL_MENU_GLOBAL), 0);
        assert_eq!(calls(&e, MENU_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn stats_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x2a4);
        assert_eq!(e.call(0x0071_f340, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), STATS_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x68), NAMED_EFFECTS_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 0x84), STATUS_EFFECT_LIST_VTABLE);
        let mut expected: Sequence = vec![
            (MENU_CONSTRUCT, vec![this]),
            (
                VECTOR_CONSTRUCT,
                vec![this + 0x28, 0x10, 4, 0x0071_f4a0, 0x0071_f4e0],
            ),
            (LIST_CONSTRUCT, vec![this + 0x6c]),
            (LIST_CONSTRUCT, vec![this + 0x88]),
        ];
        for (offset, constructor) in STATS_LIST_BOX_CONSTRUCTORS {
            expected.push((constructor, vec![this + offset]));
        }
        expected.push((MEMSET, vec![this + 0x90, 0, 0xf0]));
        let mut watched = vec![MENU_CONSTRUCT, VECTOR_CONSTRUCT, LIST_CONSTRUCT, MEMSET];
        watched.extend(STATS_LIST_BOX_CONSTRUCTORS.iter().map(|(_, a)| *a));
        assert_eq!(sequence(&e, &watched), expected);
        for offset in [0x74, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x2a0), 0);
        assert_eq!(e.mem.u8(this + 0x2a1), 0);
        assert_eq!(e.mem.u8(this + 0x2a2), 0xaa);
    }

    #[test]
    fn healing_option_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x10);
        assert_eq!(e.call(0x0071_f4a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x0071_f520);
    }

    #[test]
    fn healing_option_destructor_deletes_the_item() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call(0x0071_f4e0, &args![this]);
        assert!(calls(&e, ITEM_DELETE).is_empty());
        e.mem.set_u32(this + 8, 0x4444);
        e.call(0x0071_f4e0, &args![this]);
        assert_eq!(calls(&e, ITEM_DELETE), vec![vec![0x4444, 1]]);
    }

    #[test]
    fn healing_option_condition_needs_an_item() {
        let mut e = engine();
        let option = e.mem.alloc(0x10);
        assert!(!e.call(0x0071_f520, &args![option]).bool());
        e.mem.set_u32(option + 8, 0x100);
        assert!(e.call(0x0071_f520, &args![option]).bool());
    }

    #[test]
    fn named_effects_list_constructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0071_f540, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NAMED_EFFECTS_LIST_VTABLE);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this + 4]]);
    }

    #[test]
    fn named_effects_list_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.call(0x0071_f570, &args![this]);
        assert_eq!(e.mem.u32(this), NAMED_EFFECTS_LIST_VTABLE);
        assert_eq!(
            sequence(&e, &[NAMED_EFFECTS_CLEAR, LIST_DESTRUCT]),
            vec![
                (NAMED_EFFECTS_CLEAR, vec![this]),
                (LIST_DESTRUCT, vec![this + 4])
            ]
        );
    }

    #[test]
    fn status_effect_list_constructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0071_f5d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), STATUS_EFFECT_LIST_VTABLE);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this + 4]]);
    }

    #[test]
    fn status_effect_list_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.call(0x0071_f600, &args![this]);
        assert_eq!(e.mem.u32(this), STATUS_EFFECT_LIST_VTABLE);
        assert_eq!(
            sequence(&e, &[STATUS_EFFECTS_CLEAR, LIST_DESTRUCT]),
            vec![
                (STATUS_EFFECTS_CLEAR, vec![this]),
                (LIST_DESTRUCT, vec![this + 4])
            ]
        );
    }

    #[test]
    fn stats_menu_get_class_is_0x3eb() {
        let mut e = engine();
        assert_eq!(e.call(0x0071_f660, &args![0u32]).u32(), 0x3eb);
    }

    #[test]
    fn named_effects_list_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        check_deleting_destructor(&mut e, 0x0071_f670, this);
        assert_eq!(calls(&e, NAMED_EFFECTS_CLEAR).len(), 2);
    }

    #[test]
    fn status_effect_list_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        check_deleting_destructor(&mut e, 0x0071_f6a0, this);
        assert_eq!(calls(&e, STATUS_EFFECTS_CLEAR).len(), 2);
    }

    #[test]
    fn stats_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x2a4);
        check_deleting_destructor(&mut e, 0x0071_f6d0, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn stats_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x2a4);
        let first = e.mem.alloc(0x1c);
        let last = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x74, first);
        e.mem.set_u32(this + 0x78, last);
        e.set_global(STATS_MENU_GLOBAL, this);
        e.call(0x0071_f700, &args![this]);
        assert_eq!(e.mem.u32(this), STATS_MENU_VTABLE);
        assert_eq!(e.global::<u32>(STATS_MENU_GLOBAL), 0);
        // The two effects, +0x74 first and +0x78 last, then the members.
        let mut expected: Sequence = vec![
            (NAMED_EFFECTS_CLEAR, vec![first + 0x10]),
            (STATUS_DATA_FLUSH, vec![first + 4]),
            (FREE, vec![first]),
            (NAMED_EFFECTS_CLEAR, vec![last + 0x10]),
            (STATUS_DATA_FLUSH, vec![last + 4]),
            (FREE, vec![last]),
        ];
        for (offset, destructor) in STATS_LIST_BOX_DESTRUCTORS {
            expected.push((destructor, vec![this + offset]));
        }
        expected.push((STATUS_EFFECTS_CLEAR, vec![this + 0x84]));
        expected.push((NAMED_EFFECTS_CLEAR, vec![this + 0x68]));
        expected.push((VECTOR_DESTRUCT, vec![this + 0x28, 0x10, 4, 0x0071_f4e0]));
        expected.push((MENU_DESTRUCT, vec![this]));
        let mut watched = vec![
            STATUS_DATA_FLUSH,
            FREE,
            STATUS_EFFECTS_CLEAR,
            NAMED_EFFECTS_CLEAR,
            VECTOR_DESTRUCT,
            MENU_DESTRUCT,
        ];
        watched.extend(STATS_LIST_BOX_DESTRUCTORS.iter().map(|(_, a)| *a));
        assert_eq!(sequence(&e, &watched), expected);
    }

    #[test]
    fn body_part_controller_global_is_deleted() {
        let mut e = engine();
        e.call(0x0071_f8c0, &args![]);
        assert!(calls(&e, FREE).is_empty());
        let controller = e.mem.alloc(0x4c);
        e.set_global(BODY_PART_CONTROLLER_GLOBAL, controller);
        e.call(0x0071_f8c0, &args![]);
        assert_eq!(calls(&e, FREE), vec![vec![controller]]);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 6);
        assert_eq!(e.global::<u32>(BODY_PART_CONTROLLER_GLOBAL), 0);
    }

    #[test]
    fn body_part_controller_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        check_deleting_destructor(&mut e, 0x0071_f900, this);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 12);
    }

    #[test]
    fn body_part_controller_destructor_empties_six_lists() {
        let mut e = engine();
        let this = e.mem.alloc(0x4c);
        e.call(0x0071_f930, &args![this]);
        let cleared: Vec<u32> = calls(&e, LIST_CLEAR).iter().map(|c| c[0]).collect();
        let expected: Vec<u32> = (0..6).map(|i| this + 4 + 8 * i).collect();
        assert_eq!(cleared, expected);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![this + 4, 8, 6, LIST_DESTRUCT]]
        );
    }

    #[test]
    fn status_effect_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        check_deleting_destructor(&mut e, 0x0071_f9c0, this);
        assert_eq!(calls(&e, STATUS_DATA_FLUSH).len(), 2);
    }

    #[test]
    fn status_effect_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.call(0x0071_f9f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), NAMED_EFFECTS_LIST_VTABLE);
        assert_eq!(
            sequence(&e, &[NAMED_EFFECTS_CLEAR, LIST_DESTRUCT, STATUS_DATA_FLUSH]),
            vec![
                (NAMED_EFFECTS_CLEAR, vec![this + 0x10]),
                (LIST_DESTRUCT, vec![this + 0x14]),
                (STATUS_DATA_FLUSH, vec![this + 4]),
            ]
        );
    }

    #[test]
    fn stats_menu_attach_tile_takes_ids_0_to_59() {
        let mut e = engine();
        let this = e.mem.alloc(0x2a4);
        e.call(0x0071_fa50, &args![this, 0i32, 0x11u32]);
        e.call(0x0071_fa50, &args![this, 59i32, 0x22u32]);
        e.call(0x0071_fa50, &args![this, 60i32, 0x33u32]);
        e.call(0x0071_fa50, &args![this, -1i32, 0x44u32]);
        assert_eq!(e.mem.u32(this + 0x90), 0x11);
        assert_eq!(e.mem.u32(this + 0x90 + 59 * 4), 0x22);
        assert_eq!(e.mem.u32(this + 0x180), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
    }

    #[test]
    fn text_edit_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x5c);
        assert_eq!(e.call(0x0071_fa80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TEXT_EDIT_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(
            sequence(
                &e,
                &[MENU_CONSTRUCT, TEXT_EDITOR_CONSTRUCT, MEMSET, STRING_SET]
            ),
            vec![
                (MENU_CONSTRUCT, vec![this]),
                (TEXT_EDITOR_CONSTRUCT, vec![this + 0x34]),
                (MEMSET, vec![this + 0x28, 0, 0xc]),
                (STRING_SET, vec![ENTERED_TEXT, 0, 0]),
            ]
        );
    }

    #[test]
    fn text_edit_menu_get_class_is_0x41b() {
        let mut e = engine();
        assert_eq!(e.call(0x0071_fb20, &args![0u32]).u32(), 0x41b);
    }

    #[test]
    fn text_edit_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        check_deleting_destructor(&mut e, 0x0071_fb30, this);
        assert_eq!(calls(&e, TEXT_EDITOR_DESTRUCT).len(), 2);
    }

    #[test]
    fn text_edit_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.call(0x0071_fb60, &args![this]);
        assert_eq!(e.mem.u32(this), TEXT_EDIT_MENU_VTABLE);
        assert_eq!(
            sequence(&e, &[TEXT_EDITOR_DESTRUCT, MENU_DESTRUCT]),
            vec![
                (TEXT_EDITOR_DESTRUCT, vec![this + 0x34]),
                (MENU_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn text_edit_menu_attach_tile_takes_ids_0_to_2() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.call(0x0071_fbc0, &args![this, 2i32, 0x11u32]);
        e.call(0x0071_fbc0, &args![this, 3i32, 0x22u32]);
        e.call(0x0071_fbc0, &args![this, -1i32, 0x33u32]);
        assert_eq!(e.mem.u32(this + 0x30), 0x11);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn surgery_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x160);
        assert_eq!(e.call(0x0071_fbf0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SURGERY_MENU_VTABLE);
        let empty: Vec<u32> = calls(&e, RETURN_THIS).iter().map(|c| c[0]).collect();
        let expected: Vec<u32> = [0x74, 0x7c, 0xf8, 0x104, 0x128, 0x150]
            .iter()
            .map(|o| this + o)
            .collect();
        assert_eq!(empty, expected);
        assert_eq!(
            calls(&e, SOUND_HANDLE_CONSTRUCT),
            vec![vec![this + 0x94], vec![this + 0xa0]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![
                this + 0xc8,
                4,
                6,
                POINTER_CONSTRUCT_DEFAULT,
                POINTER_DESTRUCT
            ]]
        );
        assert_eq!(
            calls(&e, POINTER_CONSTRUCT),
            vec![vec![this + 0xf0, 0], vec![this + 0xf4, 0]]
        );
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x2c, 0, 0x48]]);
        let assigned: Vec<u32> = calls(&e, POINTER_ASSIGN).iter().map(|c| c[0]).collect();
        let mut expected = SURGERY_STATIC_POINTERS.to_vec();
        expected.extend([this + 0xf0, this + 0xf4]);
        expected.extend((0..6).map(|i| this + 0xc8 + 4 * i));
        assert_eq!(assigned, expected);
        for offset in [
            0xe0, 0xe4, 0xbc, 0xe8, 0xec, 0x84, 0xac, 0x14c, 0x88, 0x8c, 0xb0, 0xb4, 0xb8, 0x7c,
            0x80, 0x74, 0x78, 0x28,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:x}");
        }
        for offset in [0x90, 0x91, 0x15c, 0x15d] {
            assert_eq!(e.mem.u8(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x92), 0xaa);
        assert_eq!(e.mem.u32(this + 0xc0), 0x1b);
        assert_eq!(e.mem.u32(this + 0xc4), 1);
    }

    #[test]
    fn surgery_menu_get_class_is_0x41e() {
        let mut e = engine();
        assert_eq!(e.call(0x0071_feb0, &args![0u32]).u32(), 0x41e);
    }

    #[test]
    fn surgery_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x160);
        check_deleting_destructor(&mut e, 0x0071_fec0, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn surgery_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x160);
        e.call(0x0071_fef0, &args![this]);
        assert_eq!(e.mem.u32(this), SURGERY_MENU_VTABLE);
        let mut expected: Sequence = SURGERY_STATIC_POINTERS
            .iter()
            .map(|g| (POINTER_ASSIGN, vec![*g, 0]))
            .collect();
        expected.extend([
            (POINTER_ASSIGN, vec![this + 0xf0, 0]),
            (POINTER_DESTRUCT, vec![this + 0xf4]),
            (POINTER_DESTRUCT, vec![this + 0xf0]),
            (VECTOR_DESTRUCT, vec![this + 0xc8, 4, 6, POINTER_DESTRUCT]),
            (EMPTY_FUNCTION, vec![this + 0xa0]),
            (EMPTY_FUNCTION, vec![this + 0x94]),
            (MENU_DESTRUCT, vec![this]),
        ]);
        let watched = [
            POINTER_ASSIGN,
            POINTER_DESTRUCT,
            VECTOR_DESTRUCT,
            EMPTY_FUNCTION,
            MENU_DESTRUCT,
        ];
        assert_eq!(sequence(&e, &watched), expected);
    }

    #[test]
    fn surgery_menu_attach_tile_takes_ids_1_to_18() {
        let mut e = engine();
        let this = e.mem.alloc(0x160);
        e.call(0x0071_ffe0, &args![this, 1i32, 0x11u32]);
        e.call(0x0071_ffe0, &args![this, 18i32, 0x22u32]);
        e.call(0x0071_ffe0, &args![this, 0i32, 0x33u32]);
        e.call(0x0071_ffe0, &args![this, 19i32, 0x44u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x11);
        assert_eq!(e.mem.u32(this + 0x2c + 17 * 4), 0x22);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c + 18 * 4), 0);
    }

    #[test]
    fn surgery_tile_index_maps_ids_to_slots() {
        let mut e = engine();
        assert_eq!(e.call(0x0072_0010, &args![0u32, 1i32]).i32(), 0);
        assert_eq!(e.call(0x0072_0010, &args![0u32, 18i32]).i32(), 17);
        assert_eq!(e.call(0x0072_0010, &args![0u32, 0i32]).i32(), 18);
        assert_eq!(e.call(0x0072_0010, &args![0u32, 19i32]).i32(), 18);
        assert_eq!(e.call(0x0072_0010, &args![0u32, -5i32]).i32(), 18);
    }

    #[test]
    fn vats_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x144);
        assert_eq!(e.call(0x0072_0040, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VATS_MENU_VTABLE);
        let watched = [
            MENU_CONSTRUCT,
            VATS_ACTION_LIST_CONSTRUCT,
            VATS_COMMAND_CONSTRUCT,
            MEMSET,
        ];
        assert_eq!(
            sequence(&e, &watched),
            vec![
                (MENU_CONSTRUCT, vec![this]),
                (VATS_ACTION_LIST_CONSTRUCT, vec![this + 0xb0]),
                (VATS_COMMAND_CONSTRUCT, vec![this + 0x11c]),
                (MEMSET, vec![this + 0x28, 0, 0x80]),
            ]
        );
        assert_eq!(e.mem.u8(this + 0xf8), 0);
        assert_eq!(e.mem.u8(this + 0xf9), 0xaa);
        assert_eq!(e.mem.u8(this + 0x104), 0);
        assert_eq!(e.mem.u32(this + 0x108), 0);
    }

    #[test]
    fn vats_menu_get_class_is_0x420() {
        let mut e = engine();
        assert_eq!(e.call(0x0072_0100, &args![0u32]).u32(), 0x420);
    }

    #[test]
    fn vats_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x144);
        check_deleting_destructor(&mut e, 0x0072_0110, this);
        assert_eq!(calls(&e, VATS_MENU_DESTRUCT), vec![vec![this], vec![this]]);
    }

    #[test]
    fn vats_menu_attach_tile_takes_ids_1_to_31() {
        let mut e = engine();
        let this = e.mem.alloc(0x144);
        e.call(0x0072_0140, &args![this, 1i32, 0x11u32]);
        e.call(0x0072_0140, &args![this, 31i32, 0x22u32]);
        e.call(0x0072_0140, &args![this, 0i32, 0x33u32]);
        e.call(0x0072_0140, &args![this, 32i32, 0x44u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x11);
        assert_eq!(e.mem.u32(this + 0xa4), 0x22);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
    }

    #[test]
    fn special_book_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x1e4);
        assert_eq!(e.call(0x0072_0170, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SPECIAL_BOOK_MENU_VTABLE);
        let built = calls(&e, VECTOR_CONSTRUCT);
        let arrays: Vec<(u32, u32)> = built.iter().map(|c| (c[0] - this, c[2])).collect();
        assert_eq!(
            arrays,
            vec![
                (0x60, 9),
                (0x180, 0xb),
                (0x1ac, 4),
                (0x1bc, 2),
                (0x1c4, 2),
                (0x1cc, 2),
                (0x1d4, 2),
                (0x1dc, 2)
            ]
        );
        assert_eq!(
            built[0],
            vec![this + 0x60, 0x20, 9, 0x0072_0360, 0x0072_03c0]
        );
        assert_eq!(
            built[1],
            vec![
                this + 0x180,
                4,
                0xb,
                POINTER_CONSTRUCT_DEFAULT,
                POINTER_DESTRUCT
            ]
        );
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x28, 0, 0x20]]);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 1);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u8(this + 0x54), 1);
        assert_eq!(e.mem.u8(this + 0x55), 0xaa);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    #[test]
    fn special_book_menu_get_class_is_0x424() {
        let mut e = engine();
        assert_eq!(e.call(0x0072_0320, &args![0u32]).u32(), 0x424);
    }

    #[test]
    fn special_book_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x1e4);
        check_deleting_destructor(&mut e, 0x0072_0330, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn special_book_page_constructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0072_0360, &args![this]).u32(), this);
        assert_eq!(
            sequence(&e, &[BOOK_BUTTONS_CONSTRUCT, BOOK_TEXT_ITEMS_CONSTRUCT]),
            vec![
                (BOOK_BUTTONS_CONSTRUCT, vec![this]),
                (BOOK_TEXT_ITEMS_CONSTRUCT, vec![this + 0x10])
            ]
        );
    }

    #[test]
    fn special_book_page_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.call(0x0072_03c0, &args![this]);
        assert_eq!(
            sequence(&e, &[BOOK_TEXT_ITEMS_DESTRUCT, BOOK_BUTTONS_DESTRUCT]),
            vec![
                (BOOK_TEXT_ITEMS_DESTRUCT, vec![this + 0x10]),
                (BOOK_BUTTONS_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn special_book_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x1e4);
        e.call(0x0072_0420, &args![this]);
        assert_eq!(e.mem.u32(this), SPECIAL_BOOK_MENU_VTABLE);
        let destroyed = calls(&e, VECTOR_DESTRUCT);
        let arrays: Vec<(u32, u32)> = destroyed.iter().map(|c| (c[0] - this, c[2])).collect();
        assert_eq!(
            arrays,
            vec![
                (0x1dc, 2),
                (0x1d4, 2),
                (0x1cc, 2),
                (0x1c4, 2),
                (0x1bc, 2),
                (0x1ac, 4),
                (0x180, 0xb),
                (0x60, 9)
            ]
        );
        assert_eq!(destroyed[0], vec![this + 0x1dc, 4, 2, POINTER_DESTRUCT]);
        assert_eq!(destroyed[7], vec![this + 0x60, 0x20, 9, 0x0072_03c0]);
        assert_eq!(calls(&e, MENU_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn love_tester_menu_constructor() {
        let mut e = engine();
        let this = filled(&mut e, 0x230);
        assert_eq!(e.call(0x0072_0550, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LOVE_TESTER_MENU_VTABLE);
        assert_eq!(
            calls(&e, LOVE_TESTER_GLOWS_CONSTRUCT),
            vec![vec![this + 0x74]]
        );
        // LightBulbs (+0x64), then the global page's text items.
        assert_eq!(
            calls(&e, LOVE_TESTER_TEXT_ITEMS_CONSTRUCT),
            vec![vec![this + 0x64], vec![this + 0x1b4]]
        );
        assert_eq!(
            calls(&e, LOVE_TESTER_BUTTONS_CONSTRUCT),
            vec![vec![this + 0x1a4]]
        );
        let built = calls(&e, VECTOR_CONSTRUCT);
        let arrays: Vec<(u32, u32)> = built.iter().map(|c| (c[0] - this, c[2])).collect();
        assert_eq!(
            arrays,
            vec![
                (0x84, 9),
                (0x1c4, 0xb),
                (0x1f0, 4),
                (0x200, 2),
                (0x208, 2),
                (0x210, 2),
                (0x218, 2),
                (0x220, 2),
                (0x228, 2)
            ]
        );
        assert_eq!(
            built[0],
            vec![this + 0x84, 0x20, 9, 0x0072_07a0, 0x0072_0800]
        );
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x28, 0, 0x20]]);
        for offset in [0x5c, 0x60, 0x48, 0x50, 0x58] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x4c), 1);
        assert_eq!(e.mem.u8(this + 0x54), 1);
    }

    #[test]
    fn love_tester_menu_get_class_is_0x432() {
        let mut e = engine();
        assert_eq!(e.call(0x0072_0760, &args![0u32]).u32(), 0x432);
    }

    #[test]
    fn love_tester_menu_deleting_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x230);
        check_deleting_destructor(&mut e, 0x0072_0770, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn love_tester_page_constructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0072_07a0, &args![this]).u32(), this);
        let watched = [
            LOVE_TESTER_BUTTONS_CONSTRUCT,
            LOVE_TESTER_TEXT_ITEMS_CONSTRUCT,
        ];
        assert_eq!(
            sequence(&e, &watched),
            vec![
                (LOVE_TESTER_BUTTONS_CONSTRUCT, vec![this]),
                (LOVE_TESTER_TEXT_ITEMS_CONSTRUCT, vec![this + 0x10])
            ]
        );
    }

    #[test]
    fn love_tester_page_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.call(0x0072_0800, &args![this]);
        let watched = [
            LOVE_TESTER_TEXT_ITEMS_DESTRUCT,
            LOVE_TESTER_BUTTONS_DESTRUCT,
        ];
        assert_eq!(
            sequence(&e, &watched),
            vec![
                (LOVE_TESTER_TEXT_ITEMS_DESTRUCT, vec![this + 0x10]),
                (LOVE_TESTER_BUTTONS_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn love_tester_menu_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x230);
        e.call(0x0072_0860, &args![this]);
        assert_eq!(e.mem.u32(this), LOVE_TESTER_MENU_VTABLE);
        let watched = [
            VECTOR_DESTRUCT,
            LOVE_TESTER_TEXT_ITEMS_DESTRUCT,
            LOVE_TESTER_BUTTONS_DESTRUCT,
            LOVE_TESTER_GLOWS_DESTRUCT,
            MENU_DESTRUCT,
        ];
        let seen: Vec<(u32, u32)> = sequence(&e, &watched)
            .iter()
            .map(|(addr, words)| (*addr, words[0] - this))
            .collect();
        assert_eq!(
            seen,
            vec![
                (VECTOR_DESTRUCT, 0x228),
                (VECTOR_DESTRUCT, 0x220),
                (VECTOR_DESTRUCT, 0x218),
                (VECTOR_DESTRUCT, 0x210),
                (VECTOR_DESTRUCT, 0x208),
                (VECTOR_DESTRUCT, 0x200),
                (VECTOR_DESTRUCT, 0x1f0),
                (VECTOR_DESTRUCT, 0x1c4),
                (LOVE_TESTER_TEXT_ITEMS_DESTRUCT, 0x1b4),
                (LOVE_TESTER_BUTTONS_DESTRUCT, 0x1a4),
                (VECTOR_DESTRUCT, 0x84),
                (LOVE_TESTER_GLOWS_DESTRUCT, 0x74),
                (LOVE_TESTER_TEXT_ITEMS_DESTRUCT, 0x64),
                (MENU_DESTRUCT, 0),
            ]
        );
        let destroyed = calls(&e, VECTOR_DESTRUCT);
        assert_eq!(destroyed[6], vec![this + 0x1f0, 4, 4, POINTER_DESTRUCT]);
        assert_eq!(destroyed[8], vec![this + 0x84, 0x20, 9, 0x0072_0800]);
    }

    #[test]
    fn eight_tile_attach_takes_ids_0_to_7() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.call(0x0072_09e0, &args![this, 0i32, 0x11u32]);
        e.call(0x0072_09e0, &args![this, 7i32, 0x22u32]);
        e.call(0x0072_09e0, &args![this, 8i32, 0x33u32]);
        e.call(0x0072_09e0, &args![this, -1i32, 0x44u32]);
        assert_eq!(e.mem.u32(this + 0x28), 0x11);
        assert_eq!(e.mem.u32(this + 0x44), 0x22);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn quantity_menu_constructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x0072_0a10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), QUANTITY_MENU_VTABLE);
        assert_eq!(calls(&e, MENU_CONSTRUCT), vec![vec![this]]);
    }

    #[test]
    // Three generated vectors match native 00726070 on the executable/CPU
    // configuration recorded in docs/ENGINE_STATE.md, including untouched guards.
    fn folded_word_getter_preserves_next_pointer_or_signed_count_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(12);
        for word in [0u32, 0x1234_5678, (-2i32) as u32] {
            e.mem.set_u32(object, 0xaaaa_aaaa);
            e.mem.set_u32(object + 4, word);
            e.mem.set_u32(object + 8, 0xbbbb_bbbb);
            let before = e.mem.bytes(object, 12);
            assert_eq!(e.call(0x00726070, &args![object]).u32(), word);
            assert_eq!(e.mem.bytes(object, 12), before);
        }
    }

    /// Every `ListBox` instance with the addresses of its constructor,
    /// destructor, `RemoveAll`, `Sort`, `Highlight` and
    /// `GetHighlightTarget`.
    const LIST_BOXES: [(ListBoxInstance, [u32; 6]); 6] = [
        (
            LIST_BOX_ACTOR_VALUE,
            [
                0x0072_1060,
                0x0072_1150,
                0x0072_11e0,
                0x0072_1410,
                0x0072_1440,
                0x0072_14b0,
            ],
        ),
        (
            LIST_BOX_PERK_RANK_DATA,
            [
                0x0072_1850,
                0x0072_1920,
                0x0072_1990,
                0x0072_1bc0,
                0x0072_1bf0,
                0x0072_1c60,
            ],
        ),
        (
            LIST_BOX_MISC_STAT_ID,
            [
                0x0072_2000,
                0x0072_20d0,
                0x0072_2140,
                0x0072_2370,
                0x0072_23a0,
                0x0072_2410,
            ],
        ),
        (
            LIST_BOX_STATUS_EFFECT,
            [
                0x0072_27b0,
                0x0072_2880,
                0x0072_28f0,
                0x0072_2b20,
                0x0072_2b50,
                0x0072_2bc0,
            ],
        ),
        (
            LIST_BOX_TES_REPUTATION,
            [
                0x0072_2f60,
                0x0072_3070,
                0x0072_30e0,
                0x0072_3310,
                0x0072_3340,
                0x0072_33b0,
            ],
        ),
        (
            LIST_BOX_UNSIGNED_INT,
            [
                0x0072_3750,
                0x0072_3820,
                0x0072_3890,
                0x0072_3ac0,
                0x0072_3af0,
                0x0072_3b60,
            ],
        ),
    ];

    /// A scratch word the collection doubles copy the list box into.
    const SEEN: u32 = 0x0320_0000;
    /// Test vtables: menus and tiles (slot 0 `TEST_DELETE`, slot +0x14
    /// `TEST_TILE_AT`, slot +0x34 `TEST_GET_CLASS`).
    const TEST_VTABLE: u32 = 0x0330_0000;
    const TEST_DELETE: u32 = 0x0330_1000;
    const TEST_TILE_AT: u32 = 0x0330_1010;
    const TEST_GET_CLASS: u32 = 0x0330_1020;
    /// The tiles `TEST_TILE_AT` returns, by index.
    const TILE_TABLE: u32 = 0x0340_0000;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// `engine()` with this file's functions registered again (it doubles
    /// some of them), the callees of the functions from `00720a30` on
    /// answering 0, the node accessor returning its node, `_ftol2`
    /// truncating, and the test vtable.
    fn full_engine() -> Engine {
        let mut e = engine();
        for (addr, f) in funcs() {
            e.register(addr, f);
        }
        let mut externals = vec![
            GET_MENU_BY_CLASS,
            TILE_GET_FLOAT,
            TILE_GET_MENU,
            TILE_IS_TRUE,
            TILE_SET_FLOAT,
            TEXT_TO_TRAIT,
            ADD_USER_TRAIT,
            DYNAMIC_CAST,
            RELOAD_FLAG,
            START_MENU_CLASS,
            CREATE_MAP_MENU,
            CREATE_REPAIR_MENU,
            CREATE_ITEM_MOD_MENU,
            CREATE_MENU_3EA,
            CREATE_MENU_3EB,
            HUD_MAIN_MENU_CREATE,
            START_MENU_CREATE,
            LOADING_MENU_CREATE,
            LEVEL_UP_MENU_CREATE,
            CONTAINER_MENU_CREATE,
            BOOK_MENU_CREATE,
            WORD_AT_74,
            WORD_AT_30,
            WORD_AT_20,
            NEW_TIMER,
            RECOMPUTE_PACKING,
            WORD_AT_24,
            LIST_OWNER,
            LIST_HOLDER,
            LIST_HEAD,
            LIST_NODE_DATA,
            LIST_NEXT,
            LIST_ADVANCE,
            LIST_BOX_INIT_FLAGS,
            LIST_BOX_HAS_FLAG,
            LIST_BOX_BELONGS_TO,
            LIST_SORT,
            NODE_SET_NEXT,
            NODE_DELETE,
            SCROLL_ROUND,
            SIMPLE_ARRAY_INIT,
            SIMPLE_ARRAY_RELEASE,
            TEST_DELETE,
            TEST_TILE_AT,
            TEST_GET_CLASS,
            TILE_SET_INT,
            INIT_TILE_POSITION,
            LIST_COUNT,
            TILE_GET_VALUE,
            VALUE_ADD_ACTION,
            VALUE_CALCULATE,
            TILE_GET_BYTE_35,
            TILE_SET_BYTE_35,
            TILE_FIND_BY_NAME,
            TILE_GET_CHILD_BY_NAME,
            TILE_ADD_NEEDS_UPDATE,
            ROUND_FLOAT,
            MAX_INT,
        ];
        for (instance, _) in LIST_BOXES {
            externals.extend([
                instance.synchronize,
                instance.is_enabled,
                instance.update_highlight_box_pos,
                instance.set_total_list_height,
                instance.num_unfiltered,
            ]);
        }
        for addr in externals {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(RETURN_THIS, |_, a| word(a[0]));
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        e.map(SEEN, 4);
        for addr in [LIST_BOX_COLLECTION_ADD, LIST_BOX_COLLECTION_REMOVE] {
            e.register(addr, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(SEEN, value);
                Ret::default()
            });
        }
        let mut slots = vec![TEST_DELETE; 14];
        slots[5] = TEST_TILE_AT;
        slots[13] = TEST_GET_CLASS;
        e.put_vtable(TEST_VTABLE, &slots);
        e.map(TILE_TABLE, 0x40);
        for global in [
            QUANTITY_MENU_GLOBAL,
            KEEP_ITEM_TILES,
            FRONTMOST_EXCLUDED_VALUE,
        ] {
            e.map(global, 12);
        }
        // The list boxes' function-local statics and the 0.0 double.
        e.map(STATICS, 0xb0);
        e.map(ZERO_DOUBLE, 8);
        e
    }

    /// An object of `size` bytes with the test vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, TEST_VTABLE);
        block
    }

    #[test]
    fn class_getters() {
        let mut e = full_engine();
        assert_eq!(e.call(0x0072_0a30, &args![0u32]).u32(), 0x3f8);
        assert_eq!(e.call(0x0072_0df0, &args![]).u32(), 0x3ff);
    }

    #[test]
    fn quantity_menu_deleting_destructor() {
        let mut e = full_engine();
        let this = e.mem.alloc(0x44);
        check_deleting_destructor(&mut e, 0x0072_0a40, this);
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 2);
    }

    #[test]
    fn quantity_menu_destructor() {
        let mut e = full_engine();
        let this = e.mem.alloc(0x44);
        e.set_global(QUANTITY_MENU_GLOBAL, this);
        e.call(0x0072_0a70, &args![this]);
        assert_eq!(e.mem.u32(this), QUANTITY_MENU_VTABLE);
        assert_eq!(e.global::<u32>(QUANTITY_MENU_GLOBAL), 0);
        assert_eq!(calls(&e, MENU_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn reload_menu_with_timer() {
        for (class, creator) in [
            (0x3ffu32, CREATE_MAP_MENU),
            (0x40b, CREATE_REPAIR_MENU),
            (0x425, CREATE_ITEM_MOD_MENU),
            (0x3ea, CREATE_MENU_3EA),
            (0x3eb, CREATE_MENU_3EB),
        ] {
            let mut e = full_engine();
            let open = object(&mut e, 8);
            e.register_double(GET_MENU_BY_CLASS, move |_, _| word(open));
            e.register(creator, |_, _| word(0x5000));
            e.register(TILE_GET_FLOAT, |_, _| st0(2.5));
            e.register(TILE_GET_MENU, |_, _| word(0x6000));
            assert_eq!(e.call(0x0072_0aa0, &args![0u32, class]).u32(), 0x5000);
            assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![class]]);
            assert_eq!(calls(&e, TEST_DELETE), vec![vec![open, 1]]);
            assert_eq!(calls(&e, creator), vec![Vec::<u32>::new()]);
            assert_eq!(calls(&e, TILE_GET_FLOAT), vec![vec![0x5000, 0xfc1]]);
            assert_eq!(calls(&e, NEW_TIMER), vec![vec![0x6000, 2.5f32.to_bits()]]);
            assert_eq!(calls(&e, TILE_GET_MENU), vec![vec![0x5000]; 2]);
            assert_eq!(calls(&e, RECOMPUTE_PACKING), vec![vec![0x6000, 8]]);
        }
    }

    #[test]
    fn reload_menu_other_creators() {
        let mut e = full_engine();
        e.register(HUD_MAIN_MENU_CREATE, |_, _| word(0x77));
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x3ecu32]).u32(), 0x77);
        assert!(calls(&e, TEST_DELETE).is_empty());
        assert!(calls(&e, NEW_TIMER).is_empty());
        assert!(calls(&e, START_MENU_CLASS).is_empty());

        e.register(LOADING_MENU_CREATE, |_, _| word(0x88));
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x3efu32]).u32(), 0x88);
        assert_eq!(calls(&e, LOADING_MENU_CREATE), vec![vec![0, 1, 0]]);

        e.register(LEVEL_UP_MENU_CREATE, |_, _| word(0x99));
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x403u32]).u32(), 0x99);

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x400u32]).u32(), 0);
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x400]]);
        assert_eq!(calls(&e, START_MENU_CLASS).len(), 1);
        assert!(calls(&e, START_MENU_CREATE).is_empty());
    }

    #[test]
    fn reload_menu_start_menu() {
        let mut e = full_engine();
        e.register(START_MENU_CLASS, |_, _| word(0x3f5));
        e.register(RELOAD_FLAG, |_, _| word(0xff01));
        e.register(GET_MENU_BY_CLASS, |_, a| {
            word(if a[0] == 0x3f5 { 0 } else { 1 })
        });
        e.call(0x0072_0aa0, &args![0u32, 0x3f5u32]);
        // The flag is the low byte; creation failed: one lookup only.
        assert_eq!(calls(&e, START_MENU_CREATE), vec![vec![1, 0]]);
        assert_eq!(calls(&e, GET_MENU_BY_CLASS).len(), 1);
        e.register(START_MENU_CREATE, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x0072_0aa0, &args![0u32, 0x3f5u32]);
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x3f5]; 2]);
    }

    #[test]
    fn reload_menu_container_and_book() {
        let mut e = full_engine();
        let open = object(&mut e, 8);
        e.register_double(GET_MENU_BY_CLASS, move |_, _| word(open));
        e.register(DYNAMIC_CAST, |_, a| word(a[0]));
        e.register(WORD_AT_74, |_, _| word(0x74));
        e.register(CONTAINER_MENU_CREATE, |_, _| word(0xc0));
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x3f0u32]).u32(), 0xc0);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![open, 0, TILE_TYPE, CONTAINER_MENU_TYPE, 0]]
        );
        assert_eq!(calls(&e, WORD_AT_74), vec![vec![open]]);
        assert_eq!(calls(&e, CONTAINER_MENU_CREATE), vec![vec![0x74, 1]]);

        e.call_log = Some(vec![]);
        e.register(WORD_AT_30, |_, _| word(0x30));
        e.register(WORD_AT_20, |_, _| word(0x20));
        e.register(BOOK_MENU_CREATE, |_, _| word(0xb0));
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x402u32]).u32(), 0xb0);
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x402], vec![0x3f0]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![
                vec![open, 0, TILE_TYPE, BOOK_MENU_TYPE, 0],
                vec![0x20, 0, BOUND_OBJECT_TYPE, BOOK_TYPE, 0]
            ]
        );
        assert_eq!(calls(&e, WORD_AT_20), vec![vec![0x30]]);
        assert_eq!(calls(&e, TEST_DELETE), vec![vec![open, 1]; 2]);
        assert_eq!(calls(&e, BOOK_MENU_CREATE), vec![vec![0x20, 0x30]]);

        // Failed casts create nothing.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x3f0u32]).u32(), 0);
        assert_eq!(e.call(0x0072_0aa0, &args![0u32, 0x402u32]).u32(), 0);
        assert!(calls(&e, CONTAINER_MENU_CREATE).is_empty());
        assert!(calls(&e, WORD_AT_30).is_empty());
        assert!(calls(&e, BOOK_MENU_CREATE).is_empty());
    }

    #[test]
    fn delete_menu_deletes_the_open_menu() {
        let mut e = full_engine();
        assert!(!e.call(0x0072_0e00, &args![0u32, 0x3f0u32]).bool());
        let open = object(&mut e, 8);
        e.register_double(GET_MENU_BY_CLASS, move |_, _| word(open));
        assert!(e.call(0x0072_0e00, &args![0u32, 0x3f0u32]).bool());
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x3f0]; 2]);
        assert_eq!(calls(&e, TEST_DELETE), vec![vec![open, 1]]);
    }

    /// The tile list doubles: owner `0x100`, holder `0x200` (list
    /// `0x204`), nodes `[next, -, tile]`, a tile's word 0 its menu, a menu
    /// tile's trait `t` the float at `(t - 0xfa0) * 4`, `Tile::IsTrue` the
    /// word there, the menu's word at +0x24 its mode and +0x20 its class.
    fn menu_list_engine() -> Engine {
        let mut e = full_engine();
        e.register(LIST_OWNER, |_, _| word(0x100));
        e.register(LIST_HOLDER, |_, _| word(0x200));
        e.register(LIST_NODE_DATA, |_, a| word(a[1] + 8));
        e.register(LIST_NEXT, |e, a| word(e.mem.u32(a[1])));
        e.register(LIST_ADVANCE, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            word(node + 8)
        });
        e.register(TILE_GET_MENU, |e, a| word(e.mem.u32(a[0])));
        e.register(TILE_GET_FLOAT, |e, a| {
            st0(f64::from(e.mem.f32(a[0] + (a[1] - 0xfa0) * 4)))
        });
        e.register(TILE_IS_TRUE, |e, a| {
            word(e.mem.u32(a[0] + (a[1] - 0xfa0) * 4))
        });
        e.register(WORD_AT_24, |e, a| word(e.mem.u32(a[0] + 0x24)));
        e.register(TEST_GET_CLASS, |e, a| word(e.mem.u32(a[0] + 0x20)));
        e
    }

    /// One menu for [`add_menus`]: the five words it writes.
    type MenuSpec = (u32, u32, u32, u32, u32);

    /// Links `menus` (menu tile traits `0xfad`, `0xfa7`, `0xfa3` as words,
    /// mode, class) into the list and returns the menus; a `None` adds a
    /// node whose tile has no menu.
    fn add_menus(e: &mut Engine, menus: &[Option<MenuSpec>]) -> Vec<u32> {
        let mut nodes = Vec::new();
        let mut result = Vec::new();
        for entry in menus {
            let node = e.mem.alloc(12);
            let tile = e.mem.alloc(4);
            e.mem.set_u32(node + 8, tile);
            if let Some((depth, kind, open, mode, class)) = *entry {
                let menu = object(e, 0x28);
                let menu_tile = e.mem.alloc(0x40);
                e.mem.set_u32(tile, menu);
                e.mem.set_u32(menu + 4, menu_tile);
                e.mem.set_u32(menu + 0x20, class);
                e.mem.set_u32(menu + 0x24, mode);
                e.mem.set_u32(menu_tile + 0x34, depth);
                e.mem.set_u32(menu_tile + 0x1c, kind);
                e.mem.set_u32(menu_tile + 0xc, open);
                result.push(menu);
            } else {
                result.push(0);
            }
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        let first = nodes[0];
        e.register_double(LIST_HEAD, move |_, _| word(first));
        result
    }

    #[test]
    fn get_frontmost_menu_takes_the_deepest_eligible_menu() {
        let mut e = menu_list_engine();
        e.set_global(FRONTMOST_START_VALUE, (-1e8f32).to_bits());
        e.set_global(FRONTMOST_EXCLUDED_VALUE, 6008.0f64);
        let f = |v: f32| v.to_bits();
        let menus = add_menus(
            &mut e,
            &[
                Some((f(5.0), 0, 0, 0, 0)),
                None,
                Some((f(10.0), f(6008.0), 0, 0, 0)),
                Some((f(7.0), 0, 0, 2, 0)),
                Some((f(8.0), 0, 0, 4, 0)),
                Some((0x7fc0_0000, 0, 0, 0, 0)),
                Some((f(6.0), 0x7fc0_0000, 0, 0, 0)),
                Some((f(3.0), 0, 0, 0, 0)),
            ],
        );
        assert_eq!(e.call(0x0072_0e60, &args![0u32]).u32(), menus[6]);
        assert_eq!(calls(&e, LIST_HEAD), vec![vec![0x204]]);
        assert_eq!(calls(&e, LIST_NEXT).len(), 8);
    }

    #[test]
    fn get_frontmost_menu_skips_menus_without_a_tile() {
        let mut e = menu_list_engine();
        e.set_global(FRONTMOST_START_VALUE, (-1e8f32).to_bits());
        let menus = add_menus(&mut e, &[Some((0, 0, 0, 0, 0))]);
        e.mem.set_u32(menus[0] + 4, 0);
        assert_eq!(e.call(0x0072_0e60, &args![0u32]).u32(), 0);
        assert!(calls(&e, TILE_GET_FLOAT).is_empty());
    }

    #[test]
    fn is_menu_open_checks_the_first_menu_of_the_class() {
        let mut e = menu_list_engine();
        let menus = add_menus(
            &mut e,
            &[
                None,
                Some((0, 0, 0, 0, 0x3e9)),
                Some((0, 0, 1, 0, 0x3f0)),
                Some((0, 0, 1, 2, 0x3f1)),
            ],
        );
        assert!(e.call(0x0072_0f90, &args![0u32, 0x3f0u32]).bool());
        assert_eq!(
            calls(&e, TILE_IS_TRUE),
            vec![vec![e.mem.u32(menus[2] + 4), 0xfa3]]
        );
        assert!(!e.call(0x0072_0f90, &args![0u32, 0x3e9u32]).bool());
        assert!(!e.call(0x0072_0f90, &args![0u32, 0x3f1u32]).bool());
        assert!(!e.call(0x0072_0f90, &args![0u32, 0x500u32]).bool());
        assert_eq!(calls(&e, LIST_ADVANCE).len(), 3 + 2 + 4 + 4);
    }

    #[test]
    fn list_box_constructors() {
        for (instance, [constructor, ..]) in LIST_BOXES {
            let mut e = full_engine();
            let this = filled(&mut e, 0x30);
            assert_eq!(e.call(constructor, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this), instance.vtable);
            for offset in [0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
            assert_eq!(e.mem.u16(this + 0x1c), 0);
            assert_eq!(e.mem.u16(this + 0x2c), 0);
            assert_eq!(e.mem.u16(this + 0x1e), 0xaaaa);
            assert_eq!(e.mem.u16(this + 0x2e), 0xaaaa);
            assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this + 4]]);
            assert_eq!(calls(&e, LIST_BOX_INIT_FLAGS), vec![vec![this, 0, 1]]);
            assert_eq!(
                calls(&e, LIST_BOX_COLLECTION_ADD)[0][0],
                LIST_BOX_COLLECTION
            );
            assert_eq!(e.mem.u32(SEEN), this);
        }
    }

    #[test]
    fn list_box_base_constructor() {
        let mut e = full_engine();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0072_40c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LIST_BOX_BASE_VTABLE);
    }

    #[test]
    fn list_box_collection_calls_pass_the_argument_by_address() {
        let mut e = full_engine();
        e.call(0x0072_1130, &args![0x1234u32]);
        assert_eq!(e.mem.u32(SEEN), 0x1234);
        assert_eq!(
            calls(&e, LIST_BOX_COLLECTION_ADD)[0][0],
            LIST_BOX_COLLECTION
        );
        e.call(0x0072_11c0, &args![0x5678u32]);
        assert_eq!(e.mem.u32(SEEN), 0x5678);
        assert_eq!(
            calls(&e, LIST_BOX_COLLECTION_REMOVE)[0][0],
            LIST_BOX_COLLECTION
        );
    }

    #[test]
    fn list_box_destructors() {
        for (instance, [_, destructor, ..]) in LIST_BOXES {
            let mut e = full_engine();
            let this = e.mem.alloc(0x30);
            e.call(destructor, &args![this]);
            assert_eq!(e.mem.u32(this), instance.vtable);
            assert_eq!(e.mem.u32(SEEN), this);
            assert_eq!(
                sequence(
                    &e,
                    &[
                        instance.set_total_list_height,
                        instance.update_highlight_box_pos,
                        LIST_BOX_COLLECTION_REMOVE,
                        LIST_DESTRUCT,
                    ]
                )
                .iter()
                .map(|(addr, words)| (*addr, words[0]))
                .collect::<Vec<_>>(),
                vec![
                    (instance.set_total_list_height, this),
                    (instance.update_highlight_box_pos, this),
                    (LIST_BOX_COLLECTION_REMOVE, LIST_BOX_COLLECTION),
                    (LIST_DESTRUCT, this + 4),
                ]
            );
        }
    }

    #[test]
    fn list_box_deleting_destructors() {
        for (addr, instance) in [
            (0x0072_4090, LIST_BOX_ACTOR_VALUE),
            (0x0072_40e0, LIST_BOX_PERK_RANK_DATA),
            (0x0072_4110, LIST_BOX_MISC_STAT_ID),
            (0x0072_4140, LIST_BOX_STATUS_EFFECT),
            (0x0072_4170, LIST_BOX_TES_REPUTATION),
            (0x0072_41a0, LIST_BOX_UNSIGNED_INT),
        ] {
            let mut e = full_engine();
            let this = e.mem.alloc(0x30);
            check_deleting_destructor(&mut e, addr, this);
            assert_eq!(e.mem.u32(this), instance.vtable);
            assert_eq!(calls(&e, LIST_DESTRUCT), vec![vec![this + 4]; 2]);
        }
    }

    /// An item `[tile, data, -]` with a tile of the test vtable.
    fn list_item(e: &mut Engine, data: u32) -> (u32, u32) {
        let item = e.mem.alloc(12);
        let tile = object(e, 0x10);
        e.mem.set_u32(item, tile);
        e.mem.set_u32(item + 4, data);
        (item, tile)
    }

    #[test]
    fn list_box_remove_all_deletes_every_item() {
        for (instance, [_, _, remove_all, ..]) in LIST_BOXES {
            let mut e = full_engine();
            e.register(LIST_BOX_HAS_FLAG, |_, _| word(1));
            e.register(TEXT_TO_TRAIT, |_, _| word(0x55));
            let this = e.mem.alloc(0x30);
            let (item0, tile0) = list_item(&mut e, 0xd0);
            let (item1, tile1) = list_item(&mut e, 0xd1);
            let (item2, tile2) = list_item(&mut e, 0xd2);
            let node1 = e.mem.alloc(8);
            let node2 = e.mem.alloc(8);
            e.mem.set_u32(this + 4, item0);
            e.mem.set_u32(this + 8, node1);
            e.mem.set_u32(node1, item1);
            e.mem.set_u32(node1 + 4, node2);
            e.mem.set_u32(node2, item2);
            e.mem.set_u32(this + 0xc, 0x7000);
            e.mem.set_u32(this + 0x10, 0x7100);
            e.mem.set_u16(this + 0x1c, 3);
            e.mem.set_f32(this + 0x20, 9.0);
            e.call(remove_all, &args![this]);
            assert_eq!(
                calls(&e, TEST_DELETE),
                vec![vec![tile1, 1], vec![tile2, 1], vec![tile0, 1]]
            );
            assert_eq!(calls(&e, LIST_BOX_HAS_FLAG), vec![vec![this, 1]; 3]);
            assert_eq!(
                calls(&e, ITEM_DELETE),
                vec![vec![0xd1, 1], vec![0xd2, 1], vec![0xd0, 1]]
            );
            assert_eq!(calls(&e, FREE), vec![vec![item1], vec![item2], vec![item0]]);
            assert_eq!(
                calls(&e, NODE_SET_NEXT),
                vec![vec![node1, 0], vec![node2, 0]]
            );
            assert_eq!(calls(&e, NODE_DELETE), vec![vec![node1, 1], vec![node2, 1]]);
            for offset in [4, 8, 0x10, 0x20] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
            assert_eq!(e.mem.u16(this + 0x1c), 0);
            assert_eq!(calls(&e, TEXT_TO_TRAIT), vec![vec![NUM_FILTERED_TEXT]]);
            assert_eq!(calls(&e, TILE_SET_FLOAT), vec![vec![0x7000, 0x55, 0, 1]]);
            assert_eq!(
                sequence(
                    &e,
                    &[
                        instance.set_total_list_height,
                        instance.update_highlight_box_pos
                    ]
                ),
                vec![
                    (instance.set_total_list_height, vec![this]),
                    (instance.update_highlight_box_pos, vec![this]),
                ]
            );
        }
    }

    #[test]
    fn list_box_remove_all_keeps_tiles_and_data_when_told() {
        let mut e = full_engine();
        e.set_global(KEEP_ITEM_TILES, 1u8);
        let this = e.mem.alloc(0x30);
        let (item0, _) = list_item(&mut e, 0xd0);
        // A second node without an item stops the walk.
        let node = e.mem.alloc(8);
        e.mem.set_u32(this + 4, item0);
        e.mem.set_u32(this + 8, node);
        e.call(0x0072_11e0, &args![this]);
        assert!(calls(&e, TEST_DELETE).is_empty());
        assert!(calls(&e, ITEM_DELETE).is_empty());
        assert_eq!(calls(&e, FREE), vec![vec![item0]]);
        assert!(calls(&e, NODE_DELETE).is_empty());
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), node);
    }

    #[test]
    fn list_box_sort_sorts_then_synchronizes() {
        for (instance, [_, _, _, sort, ..]) in LIST_BOXES {
            let mut e = full_engine();
            let this = e.mem.alloc(0x30);
            e.call(sort, &args![this, 0x42u32]);
            assert_eq!(
                sequence(&e, &[LIST_SORT, instance.synchronize]),
                vec![
                    (LIST_SORT, vec![this + 4, 0x42]),
                    (instance.synchronize, vec![this])
                ]
            );
        }
    }

    #[test]
    fn list_box_highlight() {
        for (instance, [_, _, _, _, highlight, _]) in LIST_BOXES {
            let mut e = full_engine();
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x10, 0x99);
            assert!(!e.call(highlight, &args![this, 0u32]).bool());
            e.register(instance.is_enabled, |_, _| word(1));
            assert!(!e.call(highlight, &args![this, 0x500u32]).bool());
            assert!(calls(&e, ADD_USER_TRAIT).is_empty());
            e.register(LIST_BOX_BELONGS_TO, |_, _| word(1));
            e.register(ADD_USER_TRAIT, |_, _| word(0x66));
            assert!(!e.call(highlight, &args![this, 0x500u32]).bool());
            assert_eq!(e.mem.u32(this + 0x10), 0x99);
            assert!(calls(&e, instance.update_highlight_box_pos).is_empty());
            e.register(TILE_IS_TRUE, |_, _| word(1));
            assert!(e.call(highlight, &args![this, 0x500u32]).bool());
            assert_eq!(e.mem.u32(this + 0x10), 0x500);
            assert_eq!(calls(&e, LIST_BOX_BELONGS_TO)[0], vec![this, 0x500]);
            assert_eq!(
                calls(&e, ADD_USER_TRAIT)[0],
                vec![ENABLED_TEXT, 0xffff_ffff]
            );
            assert_eq!(calls(&e, TILE_IS_TRUE)[0], vec![0x500, 0x66]);
            assert!(e.call(highlight, &args![this, 0u32]).bool());
            assert_eq!(e.mem.u32(this + 0x10), 0);
            assert_eq!(
                calls(&e, instance.update_highlight_box_pos),
                vec![vec![this]; 2]
            );
        }
    }

    /// A list box of three items whose tiles have the indices 0 to 2
    /// (trait `0xfac`, the float at +8) and are enabled (the word at +4);
    /// a container whose trait `0xfb0` is 30 and whose scroll delta is 10.
    fn target_fixture(instance: ListBoxInstance) -> (Engine, u32, [u32; 3], u32) {
        let mut e = full_engine();
        e.register(instance.is_enabled, |_, _| word(1));
        e.register(TILE_GET_FLOAT, |e, a| {
            let offset = if a[1] == 0xfac || a[1] == 0xfb0 {
                8
            } else {
                0xc
            };
            st0(f64::from(e.mem.f32(a[0] + offset)))
        });
        e.register(TILE_IS_TRUE, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(TEXT_TO_TRAIT, |_, _| word(0x66));
        e.register(SCROLL_ROUND, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(TEST_TILE_AT, |e, a| word(e.mem.u32(TILE_TABLE + 4 * a[1])));
        let this = object(&mut e, 0x30);
        let mut tiles = [0; 3];
        let mut nodes = vec![this + 4];
        for (index, tile) in tiles.iter_mut().enumerate() {
            let (item, item_tile) = list_item(&mut e, 0);
            e.mem.set_f32(item_tile + 8, index as f32);
            e.mem.set_u32(item_tile + 4, 1);
            e.mem.set_u32(TILE_TABLE + 4 * index as u32, item_tile);
            *tile = item_tile;
            if index == 0 {
                e.mem.set_u32(this + 4, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                let previous = *nodes.last().unwrap();
                e.mem.set_u32(previous + 4, node);
                nodes.push(node);
            }
        }
        e.mem.set_u16(this + 0x1c, 3);
        let container = e.mem.alloc(0x10);
        e.mem.set_f32(container + 8, 30.0);
        e.mem.set_f32(container + 0xc, 10.0);
        (e, this, tiles, container)
    }

    #[test]
    fn list_box_get_highlight_target_steps() {
        for (instance, [.., target_of]) in LIST_BOXES {
            let target = |highlighted: Option<usize>, disabled: &[usize], direction: u32| {
                let (mut e, this, tiles, _) = target_fixture(instance);
                if let Some(index) = highlighted {
                    e.mem.set_u32(this + 0x10, tiles[index]);
                }
                for &index in disabled {
                    e.mem.set_u32(tiles[index] + 4, 0);
                }
                let found = e.call(target_of, &args![this, direction]).u32();
                let at: Calls = calls(&e, TEST_TILE_AT)
                    .iter()
                    .map(|words| words[1..].to_vec())
                    .collect();
                (tiles.iter().position(|&t| t == found), found, at)
            };
            // Down from nothing: the first; up from 1: 0.
            assert_eq!(target(None, &[], 2).0, Some(0));
            assert_eq!(target(Some(1), &[], 1).0, Some(0));
            // Down from the last stays on it: no new target.
            assert_eq!(target(Some(2), &[], 2).1, 0);
            // A disabled item is skipped.
            assert_eq!(target(Some(0), &[1], 2).0, Some(2));
            // Past the end: back from index 1 through GetTileAt.
            let (index, _, at) = target(Some(1), &[1, 2], 2);
            assert_eq!(index, Some(0));
            assert_eq!(at, vec![vec![1, 0], vec![0, 0]]);
            // Before the start: on from index 1 through GetTileAt.
            let (index, _, at) = target(Some(1), &[0, 1], 1);
            assert_eq!(index, Some(2));
            assert_eq!(at, vec![vec![1, 0], vec![2, 0]]);
            // No move for other directions.
            assert_eq!(target(Some(1), &[], 5).1, 0);
        }
    }

    #[test]
    fn list_box_get_highlight_target_pages_and_disabled_lists() {
        for (instance, [.., target_of]) in LIST_BOXES {
            let (mut e, this, tiles, container) = target_fixture(instance);
            // Page down without a container does not move.
            assert_eq!(e.call(target_of, &args![this, 0x10u32]).u32(), 0);
            assert!(calls(&e, TILE_GET_FLOAT).is_empty());
            assert_eq!(calls(&e, instance.is_enabled).len(), 1);
            e.mem.set_u32(this + 0xc, container);
            e.mem.set_u32(this + 0x10, tiles[0]);
            assert_eq!(e.call(target_of, &args![this, 0x10u32]).u32(), tiles[2]);
            assert_eq!(calls(&e, TEXT_TO_TRAIT), vec![vec![SCROLL_DELTA_TEXT]]);
            assert_eq!(
                calls(&e, TILE_GET_FLOAT)[..2],
                [vec![container, 0xfb0], vec![container, 0x66]]
            );
            assert_eq!(
                calls(&e, SCROLL_ROUND),
                vec![vec![3.0f32.to_bits(), 1.0f32.to_bits()]]
            );
            e.call_log = Some(vec![]);
            e.mem.set_u32(this + 0x10, tiles[2]);
            assert_eq!(e.call(target_of, &args![this, 0xfu32]).u32(), tiles[0]);
            assert_eq!(
                calls(&e, SCROLL_ROUND),
                vec![vec![(-3.0f32).to_bits(), 1.0f32.to_bits()]]
            );
            // A disabled list does not move.
            e.register(instance.is_enabled, |_, _| Ret::default());
            assert_eq!(e.call(target_of, &args![this, 1u32]).u32(), 0);
        }
    }

    #[test]
    fn simple_array_constructors_and_destructors() {
        for (constructor, destructor, vtable) in [
            (0x0072_3f00, 0x0072_3f30, BOOK_TEXT_ITEMS_VTABLE),
            (0x0072_3f50, 0x0072_3f80, BOOK_BUTTONS_VTABLE),
            (0x0072_3fa0, 0x0072_3fd0, LOVE_TESTER_TEXT_ITEMS_VTABLE),
            (0x0072_3ff0, 0x0072_4020, LOVE_TESTER_GLOWS_VTABLE),
            (0x0072_4040, 0x0072_4070, LOVE_TESTER_BUTTONS_VTABLE),
        ] {
            let mut e = full_engine();
            let this = e.mem.alloc(0x10);
            assert_eq!(e.call(constructor, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, SIMPLE_ARRAY_INIT), vec![vec![this, 0, 0]]);
            e.mem.set_u32(this, 0);
            e.call(destructor, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, SIMPLE_ARRAY_RELEASE), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn vats_menu_constructor_runs_the_list_box_constructor() {
        let mut e = full_engine();
        let this = e.mem.alloc(0x144);
        e.call(0x0072_0040, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb0), LIST_BOX_UNSIGNED_INT.vtable);
        assert_eq!(
            calls(&e, LIST_BOX_INIT_FLAGS),
            vec![vec![this + 0xb0, 0, 1]]
        );
    }

    /// The first word of the list boxes' function-local statics
    /// (`011d8dcc` to `011d8e70`).
    const STATICS: u32 = 0x011d_8dc0;

    /// Registers this file's translation at `addr` again (`full_engine`
    /// doubles the list box methods).
    fn real(e: &mut Engine, addr: u32) {
        let (_, f) = funcs()
            .into_iter()
            .find(|(registered, _)| *registered == addr)
            .unwrap();
        e.register(addr, f);
    }

    /// The logged calls to `addr` as (`this`, first argument).
    fn pairs(e: &Engine, addr: u32) -> Vec<(u32, u32)> {
        calls(e, addr)
            .iter()
            .map(|words| (words[0], words[1]))
            .collect()
    }

    /// A list box whose embedded list holds `items` in order (the first in
    /// the list box, the others in nodes `[item, next]`; `Some(filtered)`
    /// an item with that filtered byte, `None` a null item); returns the
    /// list box and the item tiles (0 for a null item).
    fn list_with_items(e: &mut Engine, items: &[Option<bool>]) -> (u32, Vec<u32>) {
        let this = e.mem.alloc(0x30);
        let mut tiles = Vec::new();
        let mut previous = this + 4;
        for (index, entry) in items.iter().enumerate() {
            let (item, tile) = match entry {
                Some(filtered) => {
                    let (item, tile) = list_item(e, 0);
                    e.mem.set_u8(item + 8, u8::from(*filtered));
                    (item, tile)
                }
                None => (0, 0),
            };
            tiles.push(tile);
            if index == 0 {
                e.mem.set_u32(this + 4, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(previous + 4, node);
                previous = node;
            }
        }
        (this, tiles)
    }

    #[test]
    fn simple_array_deleting_destructors() {
        for (addr, vtable) in [
            (0x0072_41d0, BOOK_TEXT_ITEMS_VTABLE),
            (0x0072_4200, BOOK_BUTTONS_VTABLE),
            (0x0072_4230, LOVE_TESTER_TEXT_ITEMS_VTABLE),
            (0x0072_4260, LOVE_TESTER_GLOWS_VTABLE),
            (0x0072_4290, LOVE_TESTER_BUTTONS_VTABLE),
        ] {
            let mut e = full_engine();
            let this = e.mem.alloc(0x10);
            check_deleting_destructor(&mut e, addr, this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, SIMPLE_ARRAY_RELEASE), vec![vec![this, 1]; 2]);
        }
    }

    #[test]
    fn list_box_flag_test_masks_the_shift_count() {
        let mut e = full_engine();
        real(&mut e, LIST_BOX_HAS_FLAG);
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 0x2c, 0x8002);
        e.mem.set_u16(this + 0x2e, 0xffff);
        let has = |e: &mut Engine, bit: u32| e.call(LIST_BOX_HAS_FLAG, &args![this, bit]).bool();
        assert!(has(&mut e, 1));
        assert!(!has(&mut e, 0));
        assert!(has(&mut e, 15));
        // Bits above the 16-bit field are clear; the count wraps at 32.
        assert!(!has(&mut e, 16));
        assert!(has(&mut e, 33));
        assert_eq!(e.call(LIST_BOX_HAS_FLAG, &args![this, 1u32]).u32(), 1);
    }

    #[test]
    fn list_box_tile_check_walks_until_a_null_item() {
        let mut e = full_engine();
        real(&mut e, LIST_BOX_BELONGS_TO);
        let (this, tiles) = list_with_items(&mut e, &[Some(false), Some(true), None, Some(false)]);
        let has =
            |e: &mut Engine, tile: u32| e.call(LIST_BOX_BELONGS_TO, &args![this, tile]).bool();
        assert!(has(&mut e, tiles[0]));
        assert!(has(&mut e, tiles[1]));
        // Behind the null item.
        assert!(!has(&mut e, tiles[3]));
        assert!(!has(&mut e, 0x1234));
        assert!(!e.call(LIST_BOX_BELONGS_TO, &args![0u32, tiles[0]]).bool());
    }

    #[test]
    fn list_box_synchronize_numbers_the_unfiltered_items() {
        for (instance, _) in LIST_BOXES {
            let mut e = full_engine();
            real(&mut e, instance.synchronize);
            let (this, tiles) = list_with_items(
                &mut e,
                &[Some(false), Some(true), Some(false), None, Some(false)],
            );
            e.mem.set_u16(this + 0x1c, 7);
            e.mem.set_f32(this + 0x20, 5.0);
            e.call(instance.synchronize, &args![this]);
            assert_eq!(e.mem.u16(this + 0x1c), 2);
            assert_eq!(e.mem.f32(this + 0x20), 0.0);
            assert_eq!(
                sequence(
                    &e,
                    &[
                        TILE_SET_INT,
                        INIT_TILE_POSITION,
                        instance.set_total_list_height,
                        instance.update_highlight_box_pos
                    ]
                ),
                vec![
                    (TILE_SET_INT, vec![tiles[0], 0xfac, 0]),
                    (INIT_TILE_POSITION, vec![this, tiles[0]]),
                    (TILE_SET_INT, vec![tiles[2], 0xfac, 1]),
                    (INIT_TILE_POSITION, vec![this, tiles[2]]),
                    (instance.set_total_list_height, vec![this]),
                    (instance.update_highlight_box_pos, vec![this]),
                ]
            );
        }
    }

    #[test]
    fn list_box_is_enabled_looks_the_trait_up_once() {
        for (instance, _) in LIST_BOXES {
            let mut e = full_engine();
            real(&mut e, instance.is_enabled);
            e.register(ADD_USER_TRAIT, |_, _| word(0x77));
            e.register(TILE_IS_TRUE, |e, a| word(e.mem.u32(a[0])));
            let this = e.mem.alloc(0x30);
            assert!(!e.call(instance.is_enabled, &args![this]).bool());
            let which = instance.enabled_trait;
            assert_eq!(e.mem.u32(which.guard), 1);
            assert_eq!(e.mem.u32(which.cache), 0x77);
            assert!(calls(&e, TILE_IS_TRUE).is_empty());
            let container = e.mem.alloc(4);
            e.mem.set_u32(this + 0xc, container);
            e.mem.set_u32(container, 1);
            assert!(e.call(instance.is_enabled, &args![this]).bool());
            // Only `AL` counts.
            e.mem.set_u32(container, 0x100);
            assert!(!e.call(instance.is_enabled, &args![this]).bool());
            assert_eq!(
                calls(&e, ADD_USER_TRAIT),
                vec![vec![ENABLED_TEXT, 0xffff_ffff]]
            );
            assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![container, 0x77]; 2]);
        }
    }

    /// `full_engine` with the instance's `UpdateHighlightBoxPos`, trait ids
    /// `0x71` (`"_highlight_y"`) and `0x72` (`"_selected_height"`), value
    /// objects `0x900 + trait`, a tile's byte at +0x35 answered with the
    /// tile's first word, the highlight box `0xb00` and its edges `0xb01`
    /// to `0xb04` (top, bottom, left, right).
    fn highlight_engine(instance: ListBoxInstance) -> Engine {
        let mut e = full_engine();
        real(&mut e, instance.update_highlight_box_pos);
        e.register(ADD_USER_TRAIT, |_, a| {
            word(if a[0] == HIGHLIGHT_Y_TEXT { 0x71 } else { 0x72 })
        });
        e.register(TILE_GET_VALUE, |_, a| word(0x900 + a[1]));
        e.register(TILE_GET_BYTE_35, |e, a| word(e.mem.u32(a[0])));
        e.register(TILE_FIND_BY_NAME, |_, _| word(0xb00));
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| {
            let edge = [TOP_TEXT, BOTTOM_TEXT, LEFT_TEXT, RIGHT_TEXT]
                .iter()
                .position(|&name| name == a[1])
                .unwrap() as u32;
            word(0xb01 + edge)
        });
        e
    }

    #[test]
    fn list_box_update_highlight_box_pos_without_a_highlight() {
        for (instance, _) in LIST_BOXES {
            let mut e = highlight_engine(instance);
            let this = e.mem.alloc(0x30);
            e.call(instance.update_highlight_box_pos, &args![this]);
            assert_eq!(
                calls(&e, ADD_USER_TRAIT),
                vec![
                    vec![HIGHLIGHT_Y_TEXT, 0xffff_ffff],
                    vec![SELECTED_HEIGHT_TEXT, 0xffff_ffff]
                ]
            );
            let guard = instance.highlight_y_trait.guard;
            assert_eq!(instance.selected_height_trait.guard, guard);
            assert_eq!(e.mem.u32(guard), 3);
            assert_eq!(e.mem.u32(instance.highlight_y_trait.cache), 0x71);
            assert_eq!(e.mem.u32(instance.selected_height_trait.cache), 0x72);
            assert!(calls(&e, TILE_SET_INT).is_empty());
            e.mem.set_u32(this + 0xc, 0x700);
            e.call(instance.update_highlight_box_pos, &args![this]);
            assert_eq!(calls(&e, ADD_USER_TRAIT).len(), 2);
            assert_eq!(
                calls(&e, TILE_SET_INT),
                vec![vec![0x700, 0x71, 0xffff_ffff], vec![0x700, 0x72, 0]]
            );
            assert!(calls(&e, TILE_GET_VALUE).is_empty());
        }
    }

    #[test]
    fn list_box_update_highlight_box_pos_follows_the_highlight() {
        for (instance, _) in LIST_BOXES {
            let mut e = highlight_engine(instance);
            let this = e.mem.alloc(0x30);
            let highlighted = e.mem.alloc(4);
            e.mem.set_u32(highlighted, 0x101);
            e.mem.set_u32(this + 0xc, 0x700);
            e.mem.set_u32(this + 0x10, highlighted);
            e.call(instance.update_highlight_box_pos, &args![this]);
            assert_eq!(
                calls(&e, TILE_GET_VALUE),
                vec![vec![0x700, 0x71], vec![0x700, 0x72]]
            );
            assert_eq!(
                sequence(&e, &[VALUE_ADD_ACTION, VALUE_CALCULATE]),
                vec![
                    (VALUE_ADD_ACTION, vec![0x971, 0x7d0, highlighted, 0xfa2]),
                    (VALUE_CALCULATE, vec![0x971, 0]),
                    (VALUE_ADD_ACTION, vec![0x972, 0x7d0, highlighted, 0xfb0]),
                    (VALUE_CALCULATE, vec![0x972, 0]),
                ]
            );
            assert_eq!(
                calls(&e, TILE_FIND_BY_NAME),
                vec![vec![0x700, HIGHLIGHT_BOX_TEXT]]
            );
            let marked = [0xb01, 0xb02, 0xb03, 0xb04, 0xb00];
            assert_eq!(
                pairs(&e, TILE_SET_BYTE_35),
                marked.iter().map(|&tile| (tile, 1)).collect::<Vec<_>>()
            );
            assert_eq!(
                pairs(&e, TILE_ADD_NEEDS_UPDATE),
                marked.iter().map(|&tile| (tile, 8)).collect::<Vec<_>>()
            );
            // Without a top edge only the box; a clear byte (`AL` 0) gives 0.
            e.call_log = Some(vec![]);
            e.mem.set_u32(highlighted, 0x100);
            e.register(TILE_GET_CHILD_BY_NAME, |_, _| Ret::default());
            e.call(instance.update_highlight_box_pos, &args![this]);
            assert_eq!(calls(&e, TILE_GET_CHILD_BY_NAME).len(), 1);
            assert_eq!(pairs(&e, TILE_SET_BYTE_35), vec![(0xb00, 0)]);
            // Without a highlight box nothing is marked.
            e.call_log = Some(vec![]);
            e.register(TILE_FIND_BY_NAME, |_, _| Ret::default());
            e.call(instance.update_highlight_box_pos, &args![this]);
            assert!(calls(&e, TILE_GET_CHILD_BY_NAME).is_empty());
            assert!(calls(&e, TILE_SET_BYTE_35).is_empty());
        }
    }

    /// Runs `SetTotalListHeight` on a list box with the container `0x700`
    /// whose trait `0xfb0` is `visible`, the given total height,
    /// unfiltered count and scroll bar; `004e4430` answers its argument,
    /// `00647b70` the larger argument, `"_scroll_delta"` is trait `0x66`
    /// and `"_number_of_items"` `0x67`. Returns the calls to
    /// `ROUND_FLOAT`, `MAX_INT`, `TILE_SET_FLOAT`, `TILE_SET_INT` and
    /// `NumUnfiltered`.
    fn total_height(
        instance: ListBoxInstance,
        visible: f32,
        total: f32,
        unfiltered: u32,
        scroll_bar: u32,
    ) -> [Calls; 5] {
        let mut e = full_engine();
        real(&mut e, instance.set_total_list_height);
        e.register_double(TILE_GET_FLOAT, move |_, _| st0(f64::from(visible)));
        e.register_double(instance.num_unfiltered, move |_, _| word(unfiltered));
        e.register(ROUND_FLOAT, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(MAX_INT, |_, a| word((a[0] as i32).max(a[1] as i32) as u32));
        e.register(TEXT_TO_TRAIT, |_, a| {
            word(if a[0] == SCROLL_DELTA_TEXT {
                0x66
            } else {
                0x67
            })
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, 0x700);
        e.mem.set_f32(this + 0x20, total);
        e.mem.set_u32(this + 0x14, scroll_bar);
        e.call(instance.set_total_list_height, &args![this]);
        assert_eq!(calls(&e, TILE_GET_FLOAT), vec![vec![0x700, 0xfb0]]);
        [
            calls(&e, ROUND_FLOAT),
            calls(&e, MAX_INT),
            calls(&e, TILE_SET_FLOAT),
            calls(&e, TILE_SET_INT),
            calls(&e, instance.num_unfiltered),
        ]
    }

    #[test]
    fn list_box_set_total_list_height_scroll_steps() {
        for (instance, _) in LIST_BOXES {
            // 4 * 60 / 100 = 2.4: 2 + 1 = 3 steps, a delta of 60 / 2.
            let [round, max, set_float, set_int, unfiltered] =
                total_height(instance, 40.0, 100.0, 4, 0x800);
            assert_eq!(round, vec![vec![2.4f32.to_bits()]]);
            assert_eq!(max, vec![vec![3, 2]]);
            assert_eq!(set_float, vec![vec![0x700, 0x66, 30.0f32.to_bits(), 1]]);
            assert_eq!(set_int, vec![vec![0x800, 0x67, 3]]);
            assert_eq!(unfiltered.len(), 1);
            // No unfiltered items still gives two steps.
            let [_, max, set_float, set_int, _] = total_height(instance, 40.0, 100.0, 0, 0x800);
            assert_eq!(max, vec![vec![1, 2]]);
            assert_eq!(set_float, vec![vec![0x700, 0x66, 60.0f32.to_bits(), 1]]);
            assert_eq!(set_int, vec![vec![0x800, 0x67, 2]]);
            // No scroll bar: the delta only.
            let [_, _, set_float, set_int, _] = total_height(instance, 40.0, 100.0, 4, 0);
            assert_eq!(set_float.len(), 1);
            assert!(set_int.is_empty());
        }
    }

    #[test]
    fn list_box_set_total_list_height_single_step() {
        for (instance, _) in LIST_BOXES {
            // Everything visible, an empty list, a NaN height: one step.
            for (visible, total) in [
                (100.0, 50.0),
                (50.0, 50.0),
                (40.0, 0.0),
                (0.0, 100.0),
                (f32::NAN, 100.0),
                (40.0, -1.0),
            ] {
                let [round, max, set_float, set_int, unfiltered] =
                    total_height(instance, visible, total, 4, 0x800);
                assert!(round.is_empty() && max.is_empty() && unfiltered.is_empty());
                assert!(set_float.is_empty());
                assert_eq!(set_int, vec![vec![0x800, 0x67, 1]]);
            }
        }
    }

    #[test]
    fn list_box_set_total_list_height_without_a_container() {
        let mut e = full_engine();
        real(&mut e, 0x0072_4800);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, 0x800);
        e.call(0x0072_4800, &args![this]);
        assert!(calls(&e, TILE_GET_FLOAT).is_empty());
        assert!(calls(&e, TILE_SET_INT).is_empty());
    }

    #[test]
    fn list_box_num_unfiltered_subtracts_the_truncated_filter_count() {
        for (instance, _) in LIST_BOXES {
            for (filtered, expected) in [
                (2.9, 5),
                (-2.9, 9),
                (f64::NAN, 7),
                (5e9, 7i32.wrapping_sub(5_000_000_000i64 as i32)),
            ] {
                let mut e = full_engine();
                real(&mut e, instance.num_unfiltered);
                e.register(ADD_USER_TRAIT, |_, _| word(0x78));
                e.register(LIST_COUNT, |_, _| word(7));
                e.register_double(TILE_GET_FLOAT, move |_, _| st0(filtered));
                let this = e.mem.alloc(0x30);
                e.mem.set_u32(this + 0xc, 0x700);
                for _ in 0..2 {
                    assert_eq!(
                        e.call(instance.num_unfiltered, &args![this]).i32(),
                        expected
                    );
                }
                assert_eq!(
                    calls(&e, ADD_USER_TRAIT),
                    vec![vec![NUM_FILTERED_TEXT, 0xffff_ffff]]
                );
                assert_eq!(e.mem.u32(instance.num_filtered_trait.guard), 1);
                assert_eq!(calls(&e, LIST_COUNT), vec![vec![this]; 2]);
                assert_eq!(calls(&e, TILE_GET_FLOAT), vec![vec![0x700, 0x78]; 2]);
            }
        }
    }
}
