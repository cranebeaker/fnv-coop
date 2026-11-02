//! `BSMenu/tile.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the menu system's `Tile` (a node of the interface tree), its
//! trait values (`Tile::Value`), the text table that maps XML words to
//! numbers (`Tile::MenuStringMap`) and the XML reader that builds a tile tree
//! from a menu file. It is a large unit, 164 functions in address order from
//! `009ff340`. Translated so far, in order:
//!
//! * session 1: `009ff340` to `00a03da0` (40 functions, up to and including
//!   `Tile::GetChildByName`);
//! * session 2: `00a03eb0` to `00a09410` (40 functions, from
//!   `Tile::GetChildByID` up to and including `Tile::Value::CalculateValue`):
//!   the dirty and hibernating tile lists, `UpdateAll` / `UpdateTile`,
//!   `GetTextureAtlasInfo`, the fade controls, `SetParent`, the link
//!   lookup, and the trait value's actions and calculation;
//! * session 3: `00a0a0b0` to `00a0c080` (40 functions, from
//!   `Tile::GetUnderscoreValue` up to and including the `GetRTTI` slot at
//!   `00a0c080`): the reactions, `Value::SetFloat` / `SetString`,
//!   `TileTemplate::AddPair` (the XML reader's template items, with the pool
//!   of unused items), `BuildStorage`, the colour walks, the rotation update
//!   and the small `TileText` / `Tile3D` / `XMLStorage` members;
//! * session 4: `00a0c090` to `00a0d7c0` (40 functions, from the
//!   `Tile::Extra` deleting destructor up to the hash-map node release at
//!   `00a0d7c0`): the template constructor and destructor, the template item
//!   constructor, the `std::string` `rfind` / `append` steps, and the
//!   container code the unit emitted (`NiTPointerMap` / `NiTMapBase` set,
//!   iterate, hash and destructor chain, `BSSimpleArray` insert, grow and
//!   sorted find, `BSSimpleList<FadeControl *>` add and remove). The
//!   last functions, from `00a0d7f0`, are the `NiTPointerMap<int, int>`
//!   code and then the map of `Tile::Value *` to `Tile::Reaction *`, up to
//!   `00a0d990`, the last function of the unit before the library tail);
//! * session 5: `00a0d7f0` to `00a0d990` (5 functions): the unit is
//!   complete (only the `library` initializer `00fb33f0` remains).
//!
//! Conventions of the whole unit:
//!
//! * Every tile lock (`Tile::Lock` `00a044f0`, `Tile::Unlock` `00a04500`)
//!   is a call by address; the Win32 critical section the trait code wraps
//!   is `011f3330`, entered and left through the import slots.
//! * Trait ids, tile types, update flags and template commands are the
//!   Xbox PDB enumerations (`Tile::enumTrait`, `enumType`, `enumbfUpdate`,
//!   `TEMPLATE_ID`); the PC build uses the same numbers.
//! * The compiler's exception-unwinding frames and stack cookies are not
//!   translated. Blocks the compiler folded away (`if (0)` compiled as
//!   `XOR ECX,ECX; JZ`) are left out and said so where they occur.
//! * Locals the game keeps on its stack and passes by address are
//!   [`Engine::with_stack`] blocks. The fixed-size character arrays of the
//!   XML reader are separate blocks here, not neighbours of the other
//!   locals as on the game's stack, so an overlong tag name overflows into
//!   its neighbour block instead of into the game's other locals.
//!
//! Layouts, constants and helpers a later session reuses are at the top of
//! this file.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, BSStringT, NiTPointerList, NiTPointerMap};
use crate::units::platform::MEMORY_MANAGER;
use crate::Inline;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `Tile` (Xbox PDB), 0x38 bytes on PC as on the Xbox. The offsets of
    /// the fields match the PDB.
    pub struct Tile: 0x38 {
        /// `xChildren` (Xbox PDB): `NiTPointerList<Tile *>`, head pointer at
        /// +4, item count at +0xC.
        0x04 xChildren: Inline<NiTPointerList>,
        /// `xTraits` (Xbox PDB): `BSSimpleArray<Tile::Value *, 8>`, kept
        /// sorted by trait id; buffer at +0x14, size at +0x18.
        0x10 xTraits: Inline<BSSimpleArray>,
        /// `xName` (Xbox PDB): `BSStringT<char>`.
        0x20 xName: Inline<BSStringT>,
        /// `pParent` (Xbox PDB).
        0x28 pParent: Ptr<Tile>,
        /// `spModel` (Xbox PDB): `NiPointer<NiNode>`.
        0x2C spModel: Ptr,
        /// `uiFlags` (Xbox PDB): the `enumbfUpdate` bits.
        0x30 uiFlags: u32,
        /// `bNeedsNiUpdate` (Xbox PDB).
        0x34 bNeedsNiUpdate: bool,
        /// `bSpeechChallengeFailure` (Xbox PDB).
        0x35 bSpeechChallengeFailure: bool,
    }

    /// `Tile::Value` (Xbox PDB), 0x14 bytes: one trait of a tile.
    pub struct TileValue: 0x14 {
        /// `eIndex` (Xbox PDB): the trait id.
        0x00 eIndex: i32,
        /// `pParent` (Xbox PDB): the owning tile.
        0x04 pParent: Ptr<Tile>,
        /// `fValue` (Xbox PDB).
        0x08 fValue: f32,
        /// `strValue` (Xbox PDB): `char *`.
        0x0C strValue: Ptr,
        /// `pActionListA` (Xbox PDB).
        0x10 pActionListA: Ptr,
    }

    /// `Tile::BuildStorage` (Xbox PDB), 0x14 bytes: what the XML reader
    /// fills.
    pub struct BuildStorage: 0x14 {
        /// `pTemplate` (Xbox PDB): the template being built.
        0x00 pTemplate: Ptr<TileTemplate>,
        /// `xSubTemplates` (Xbox PDB): `BSSimpleList<Tile::TileTemplate *>`,
        /// first node embedded (item at +4, next at +8).
        0x04 xSubTemplates: Inline<BSSimpleList>,
        /// `pCurrentTemplate` (Xbox PDB).
        0x0C pCurrentTemplate: Ptr<TileTemplate>,
        /// `bDeleteTemplates` (Xbox PDB).
        0x10 bDeleteTemplates: bool,
    }

    /// `Tile::TileTemplate` (Xbox PDB), 0x14 bytes.
    pub struct TileTemplate: 0x14 {
        /// `xName` (Xbox PDB): `NiFixedString`.
        0x00 xName: u32,
        /// `pParent` (Xbox PDB): the owning [`BuildStorage`].
        0x04 pParent: Ptr<BuildStorage>,
        /// `xList` (Xbox PDB): `NiTPointerList<Tile::TileTemplateItem *>`,
        /// head pointer at +8.
        0x08 xList: Inline<NiTPointerList>,
    }

    /// `Tile::TileTemplateItem` (Xbox PDB), 0x18 bytes: one parsed element.
    pub struct TileTemplateItem: 0x18 {
        /// `iCmd` (Xbox PDB): a `TEMPLATE_ID`.
        0x00 iCmd: i32,
        /// `fVal` (Xbox PDB).
        0x04 fVal: f32,
        /// `xStr` (Xbox PDB): `BSStringT<char>`, string at +8, length at +0xC.
        0x08 xStr: Inline<BSStringT>,
        /// `u` (Xbox PDB): a trait id, or (for tile items) the tile built
        /// from the item.
        0x10 u: u32,
        /// `iLine` (Xbox PDB).
        0x14 iLine: i32,
    }

    /// `XMLStorage` (Xbox PDB): a menu file read into memory.
    pub struct XmlStorage: 0x08 {
        /// `iFileSize` (Xbox PDB).
        0x00 iFileSize: u32,
        /// `pXMLData` (Xbox PDB).
        0x04 pXMLData: Ptr,
    }

    /// `Tile::FadeControl` (Xbox PDB), 0x1c bytes: one running fade of a
    /// tile's trait. The nodes of the global fade list ([`FADE_CONTROLS_LIST`])
    /// point to these.
    pub struct FadeControl: 0x1c {
        /// `fStartValue` (Xbox PDB).
        0x00 fStartValue: f32,
        /// `fEndValue` (Xbox PDB).
        0x04 fEndValue: f32,
        /// `uiStartTime` (Xbox PDB): `GetTickCount` at the start.
        0x08 uiStartTime: u32,
        /// `fDurationMillis` (Xbox PDB).
        0x0c fDurationMillis: f32,
        /// `iTrait` (Xbox PDB): the trait id that is faded.
        0x10 iTrait: i32,
        /// `pParent` (Xbox PDB): the tile.
        0x14 pParent: Ptr<Tile>,
        /// `eFadeType` (Xbox PDB): a `TILE_FADE_CONTROL_TYPE`.
        0x18 eFadeType: i32,
    }

    /// `Tile::Action` (Xbox PDB) with its two derived classes
    /// `Tile::FloatAction` and `Tile::RefValueAction` (0x10 bytes, a vtable
    /// at +0): one step of a trait's calculation.
    pub struct ValueAction: 0x10 {
        /// `eActionType` (Xbox PDB): a `VALUE_ACTION`.
        0x04 eActionType: i32,
        /// `pnext` (Xbox PDB).
        0x08 pnext: Ptr,
        /// `fValue` (Xbox PDB, `FloatAction`).
        0x0c fValue: f32,
        /// `pRefValue` (Xbox PDB, `RefValueAction`): the trait the action
        /// reads.
        0x0c pRefValue: Ptr<TileValue>,
    }

    /// `Tile::Reaction` (Xbox PDB), 0x8 bytes: a node of the list of the
    /// values that depend on a value.
    pub struct Reaction: 0x08 {
        /// `preactionValue` (Xbox PDB).
        0x00 preactionValue: Ptr<TileValue>,
        /// `pnext` (Xbox PDB).
        0x04 pnext: Ptr,
    }

    /// `Tile::Extra` (Xbox PDB, an `NiExtraData`), 0x14 bytes: the extra
    /// data that ties a model's scene-graph node to its tile.
    pub struct Extra: 0x14 {
        /// `pTile` (Xbox PDB).
        0x0c pTile: Ptr<Tile>,
        /// `pNode` (Xbox PDB): the `NiNode`.
        0x10 pNode: Ptr,
    }
}

// ---------------------------------------------------------------------------
// Constants

/// The win32 critical section the tile code enters around trait lists.
pub(crate) const TILE_CRITICAL_SECTION: u32 = 0x011f_3330;
/// `EnterCriticalSection` / `LeaveCriticalSection` import slots.
pub(crate) const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
pub(crate) const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `MemoryManager::Allocate(size)` and `Deallocate(block)` (methods of the
/// singleton at [`MEMORY_MANAGER`]).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

/// `Tile::Lock()` and `Tile::Unlock()` (Xbox PDB): recursive lock around
/// the shared tile data (`cdecl`, no arguments).
pub(crate) const TILE_LOCK: u32 = 0x00a0_44f0;
pub(crate) const TILE_UNLOCK: u32 = 0x00a0_4500;
/// `Tile::SetParent(parent, flag)` (Xbox PDB).
const TILE_SET_PARENT: u32 = 0x00a0_87d0;
/// `Tile::RemoveFadeControl` (Xbox PDB): given `0x80000000` by `Release`.
const TILE_REMOVE_FADE_CONTROL: u32 = 0x00a0_7dc0;
/// `Tile::AddNeedsUpdate(flags)` (Xbox PDB), `thiscall`.
pub(crate) const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::UpdateChildren(bool)` (Xbox PDB), `thiscall`.
const TILE_UPDATE_CHILDREN: u32 = 0x00a0_4620;
/// `Tile::Value::~Value` (Xbox PDB): destroys a trait value in place.
const VALUE_DESTROY: u32 = 0x00a0_9330;
/// `Tile::Value::SetFloat(float, bool)` and `SetString(string, bool)`
/// (Xbox PDB).
const VALUE_SET_FLOAT: u32 = 0x00a0_a270;
const VALUE_SET_STRING: u32 = 0x00a0_a300;
/// `Tile::Value::AddAction(action, float)` (Xbox PDB), `thiscall`.
const VALUE_ADD_ACTION: u32 = 0x00a0_9080;
/// `Tile::Value::AddAction_ov2(action, tile, int)` (Xbox PDB).
const VALUE_ADD_ACTION_OV2: u32 = 0x00a0_9130;
/// `Tile::Value::CalculateValue(bool)` (Xbox PDB).
const VALUE_CALCULATE_VALUE: u32 = 0x00a0_9410;
/// `BSSimpleArray<Tile::Value *, 8>::SortedFind(&key, compare)` (Xbox PDB)
/// and its insert counterpart `(&value, compare)`; both are given
/// [`fn_00a01160`] as the comparison.
const VALUES_SORTED_FIND: u32 = 0x00a0_cb90;
const VALUES_SORTED_INSERT: u32 = 0x00a0_caf0;
/// `BSSimpleArray` clear `(this, free)` (no PDB name): empties the array
/// and, with the flag, frees its buffer; `Release` calls it with 1.
const VALUES_CLEAR: u32 = 0x0084_54f0;
/// Destructor of the `Tile::Value *` array member.
const VALUES_DESTROY: u32 = 0x0070_9eb0;
/// `NiTPointerList` pop from the head, returning the element.
const LIST_REMOVE_HEAD: u32 = 0x007b_5390;
/// `NiTPointerListBase<...>::RemoveAll` (Xbox PDB).
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `NiTPointerList::Remove(&item)` (no PDB name): unlinks the item whose
/// element equals the one passed by address and returns that element (the
/// word itself when it is not in the list). Used with the dirty-tile list
/// (`011f3318`) and the hibernating-tile list (`011f3324`).
const LIST_REMOVE_ITEM: u32 = 0x0057_c730;
const DIRTY_TILES_LIST: u32 = 0x011f_3318;
const HIBERNATING_TILES_LIST: u32 = 0x011f_3324;
/// `Tile::iHibernatingTileCount` (Xbox PDB).
const HIBERNATING_TILE_COUNT: u32 = 0x011f_32d8;
/// `Interface::TileIsBeingDeleted(tile)` (Xbox PDB, `cdecl`).
const INTERFACE_TILE_IS_BEING_DELETED: u32 = 0x0070_6c20;
/// `Interface::GetMenusRoot()` (Xbox PDB).
const INTERFACE_GET_MENUS_ROOT: u32 = 0x0070_6cd0;
/// `Interface::TestConstantForGameSettings(name, out)` (Xbox PDB, `cdecl`).
const INTERFACE_TEST_CONSTANT_FOR_GAME_SETTINGS: u32 = 0x0070_73d0;
/// The interface's "is widescreen" query used by the text table.
const INTERFACE_IS_WIDESCREEN: u32 = 0x0070_7b40;
/// `bHas360Controller` (Xbox PDB, static of the interface manager) and the
/// byte the text table tests next to it.
const HAS_360_CONTROLLER: u32 = 0x011d_8a84;
const CONSOLE_UI_BYTE: u32 = 0x011d_8c50;
/// `NiPointer<T>::operator=(ptr)`, called on the embedded pointer.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `BSStringT<char>::Set(string, length)`: assigns (or, with `0, 0`, frees).
const STRING_SET: u32 = 0x0040_37f0;
/// `BSStringT<char>::operator_` (Xbox PDB): appends the string to the text.
const STRING_APPEND: u32 = 0x0040_4820;
/// `BSStringT<char>` assignment from another `BSStringT` (`(this, &other)`).
const STRING_ASSIGN: u32 = 0x0043_8470;
/// `sprintf`-like formatter into a `BSStringT` (`cdecl`, `(string, format,
/// ...)`).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// Prints an error to the console and log (`cdecl`, format and arguments).
const PRINT_ERROR: u32 = 0x005b_5e40;
/// `NiObjectNET::GetExtraData(key)` (Xbox PDB).
const NI_OBJECT_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The extra-data key (an `NiFixedString` handle) the tile pointer is
/// stored under on a model.
const TILE_EXTRA_DATA_KEY: u32 = 0x011f_34cc;
/// `Menu::GetXML(path, flag)` (Xbox PDB, `cdecl`).
const MENU_GET_XML: u32 = 0x00a1_c9b0;
/// `Menu::GetMaxDepth()` (Xbox PDB).
const MENU_GET_MAX_DEPTH: u32 = 0x00a1_dfb0;
/// `(menu, template)` (no PDB name, `BSMenu/menu.cpp`): adds the template
/// to a list (`BSSimpleList::AddHead`); `ReadFile` calls it with each
/// sub-template of the parse storage.
const MENU_ADD_TEMPLATE: u32 = 0x00a1_dd80;
/// `XMLStorage::~XMLStorage` and its scalar deleting destructor
/// (`(this, 1)`, Xbox PDB).
const XML_STORAGE_DESTROY: u32 = 0x00a0_be70;
const XML_STORAGE_SCALAR_DELETING_DESTRUCTOR: u32 = 0x00a0_be10;
/// `Tile::BuildStorage::BuildStorage` / `~BuildStorage` (Xbox PDB).
const BUILD_STORAGE_CONSTRUCT: u32 = 0x00a0_ac80;
const BUILD_STORAGE_DESTROY: u32 = 0x00a0_ad40;
/// `Tile::TileTemplate::AddPair(command, text, line, flag)` (Xbox PDB).
const TEMPLATE_ADD_PAIR: u32 = 0x00a0_a410;
/// The tile path lookup `(tile, name)` (`cdecl`) used by action links.
const TILE_FIND_BY_PATH: u32 = 0x00a0_8b20;

/// The `Tile::MenuStringMap` text table (`xTextTable`, Xbox PDB static):
/// a `char *` to `int` hash map (bucket count at +4, buckets at +8).
const TEXT_TABLE: u32 = 0x011f_32f4;
const TEXT_TABLE_BUCKET_COUNT: u32 = 0x011f_32f8;
const TEXT_TABLE_BUCKETS: u32 = 0x011f_32fc;
/// The `int` to `int` pointer map `xTraitExtraData` (Xbox PDB static, matched
/// by its type and use): registration enters a user trait's id under the
/// number at the end of its name.
const TRAIT_EXTRA_DATA_MAP: u32 = 0x011f_3308;
/// `Tile::iTextTableUserNextID` (Xbox PDB).
const TEXT_TABLE_USER_NEXT_ID: u32 = 0x011a_6d74;
/// First id of a user-defined name.
const FIRST_USER_TEXT_ID: i32 = 10000;
/// `MenuStringMap` lookup `(this, name, &out)`: true and `*out` when found.
const TEXT_TABLE_FIND: u32 = 0x00a1_e380;
/// `MenuStringMap` insert `(this, name, id)`, the insert into
/// [`TRAIT_EXTRA_DATA_MAP`] `(this, id, number)` and the text table's
/// iterator step `(this, &node, &key, &value)`.
const TEXT_TABLE_ADD: u32 = 0x00a0_c900;
const TRAIT_EXTRA_DATA_ADD: u32 = 0x00a0_c6c0;
const TEXT_TABLE_NEXT: u32 = 0x00a0_c850;
/// `(name)` (`cdecl`, no PDB name): true for a name that starts with `_`
/// or with `&_` (the two bytes at `01093f80`), the spelling of the names
/// that users add.
const TEXT_IS_USER_NAME: u32 = 0x00a0_bb00;
/// Destroys a `Tile::TextureAtlasEntry`'s three strings (no PDB name).
const TEXTURE_ATLAS_ENTRY_DESTROY: u32 = 0x00a0_bb50;
/// Empties a hash map and a `BSSimpleList` (`this`; code shared with the audio
/// library by identical-code folding, no PDB names).
const MAP_REMOVE_ALL: u32 = 0x00ae_91f0;
const SIMPLE_LIST_REMOVE_ALL: u32 = 0x00ae_83e0;
/// Heads of the two static `BSSimpleList`s that [`tile_free_trait_list`]
/// empties: `TextureEntryList` (`BSSimpleList<Tile::TextureAtlasEntry *>`,
/// Xbox PDB static; its entries are destroyed with
/// [`TEXTURE_ATLAS_ENTRY_DESTROY`]) and a list whose items are only freed,
/// presumably `xFadeControls` (Xbox PDB static; matched by role: `Release`
/// removes fade controls).
const TEXTURE_ENTRY_LIST: u32 = 0x011f_3350;
const FADE_CONTROLS_LIST: u32 = 0x011f_3348;
/// The flag the XML reader sets when it hit an error.
const XML_ERROR_FLAG: u32 = 0x011f_32c8;
/// The pointer to the interface object whose lock (+0x80) the reader holds
/// while it updates the new tree.
const INTERFACE_LOCK_OWNER: u32 = 0x011f_4748;

/// Function-local statics of [`tile_get_value`] and
/// [`tile_get_or_create_value`]: a search key (a 0x14-byte `Tile::Value`),
/// its "initialized" guard word and the `atexit` destructor registered when
/// it is initialized.
const GET_VALUE_KEY: u32 = 0x011f_3374;
const GET_VALUE_KEY_GUARD: u32 = 0x011f_3388;
const GET_VALUE_KEY_DESTROY: u32 = 0x00fd_a290;
const GET_OR_CREATE_KEY: u32 = 0x011f_338c;
const GET_OR_CREATE_KEY_GUARD: u32 = 0x011f_33a0;
const GET_OR_CREATE_KEY_DESTROY: u32 = 0x00fd_a2a0;
/// `atexit(function)` (CRT).
const ATEXIT: u32 = 0x00ec_658f;

/// Vtables written by the constructors and destructor of the tile classes
/// (the first word of the object).
const VTABLE_TILE: u32 = 0x0106_ed9c;
const VTABLE_TILE_RECT: u32 = 0x0106_ed70;
const VTABLE_TILE_TEXT: u32 = 0x0109_4878;
const VTABLE_TILE_MENU: u32 = 0x0106_ed44;
/// Constructors used by [`tile_build_tile`]: the base-class constructor
/// `(this)`, `TileImage::TileImage` (Xbox PDB), the `Tile3D` constructor and
/// `RadialTile::RadialTile` (Xbox PDB).
const TILE_BASE_CONSTRUCT: u32 = 0x0070_9860;
const TILE_IMAGE_CONSTRUCT: u32 = 0x0070_b5d0;
const TILE_3D_CONSTRUCT: u32 = 0x00a0_bc80;
const RADIAL_TILE_CONSTRUCT: u32 = 0x00a2_1660;

/// The empty string a tile's name is initialized with.
const EMPTY_NAME: u32 = 0x0101_1584;
/// `"solid.dds"`, the texture file name of the hot-rect tile.
const SOLID_TEXTURE_NAME: u32 = 0x0109_4a70;
/// The word at `011f336c`: the default texture atlas name the hot-rect
/// tile gets.
const DEFAULT_ATLAS_NAME: u32 = 0x011f_336c;

/// The `%s` format of the XML errors and their message strings (all in
/// the exe's `.rdata`).
const XML_ERROR_FORMAT: u32 = 0x0109_4a08;
const MSG_BASE_TILE_NOT_RELEASED: u32 = 0x0109_3f44;
const MSG_UNABLE_TO_CREATE_TILE: u32 = 0x0109_4a88;
const MSG_EMPTY_TAG_NAME: u32 = 0x0109_4a3c;
const MSG_ATTRIBUTE_NO_VALUE: u32 = 0x0109_49f0;
const MSG_MISSING_ATTRIBUTE_NAME: u32 = 0x0109_49d8;
const MSG_UNEXPECTED_WORD: u32 = 0x0109_49b0;
const MSG_MISSING_ATTRIBUTE_VALUE: u32 = 0x0109_4970;
const MSG_CLOSE_TAG_MARKER: u32 = 0x0109_4934;
const MSG_UNBALANCED_CLOSE_PAIR: u32 = 0x0109_4904;
const MSG_UNBALANCED_END_OF_TAG: u32 = 0x0109_48dc;
const MSG_BUFFER_TOO_SMALL: u32 = 0x0109_498c;
const MSG_TRAIT_OUTSIDE_TILE: u32 = 0x0109_4b78;
const MSG_ACTION_OUTSIDE_TRAIT: u32 = 0x0109_4b4c;
const MSG_ACTION_LINK_OUTSIDE_TRAIT: u32 = 0x0109_4b18;
const MSG_ACTION_BEGUN_OUTSIDE_TRAIT: u32 = 0x0109_4aec;
const MSG_ACTION_ENDED_OUTSIDE_TRAIT: u32 = 0x0109_4ac0;

/// Doubles the code compares floats with: `0.0` (`01012060`), the
/// `Tile::enumTrait::eNone` marker `-2147483648.0` (`01094ba8`) and `906.0`
/// (`01094a80`, `eHotRect` as a number).
const ZERO: u32 = 0x0101_2060;
const NO_TRAIT_VALUE: u32 = 0x0109_4ba8;
const HOT_RECT_TYPE_VALUE: u32 = 0x0109_4a80;
/// Floats the hot-rect setup writes: `-1.0` (`01012054`) and `255.0`
/// (`01023cd8`).
const MINUS_ONE: u32 = 0x0101_2054;
const TWO_FIFTY_FIVE: u32 = 0x0102_3cd8;

/// The `"<include"` tag searched for, the `"` delimiter and the include
/// path format `"Data\Menus\Prefabs\%s"`.
const INCLUDE_TAG: u32 = 0x0109_4a64;
const QUOTE_DELIMITER: u32 = 0x0102_1184;
const PREFAB_PATH_FORMAT: u32 = 0x0109_4a4c;
/// Longest text the reader's string buffers hold before it complains.
const READ_BUFFER_LIMIT: i32 = 0x1000;
/// `tile_parse_file`'s frame under `EBP` (the exception record and `_chkstk(0x2108)`).
const FRAME_BELOW_EBP: u32 = 0x2114;
/// Size of the buffer `fn_00a018d0` copies the part outside the brackets to.
const NAME_BUFFER_SIZE: u32 = 0x104;

/// CRT functions that have no translation yet (`strstr`, `strrchr`,
/// `strcmp`, `strtok_s`) and the translated ones the reader calls.
const STRSTR: u32 = 0x00ec_7750;
const STRRCHR: u32 = 0x00ec_6e30;
const STRCMP: u32 = 0x00ec_6da0;
const STRTOK_S: u32 = 0x00ec_c5f8;
const STRCHR: u32 = 0x00ec_7690;
const STRLEN: u32 = 0x00ec_6130;
const MEMSET: u32 = 0x00ec_61c0;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const ATOL: u32 = 0x00ec_a6d3;
/// `_ftol2_sse` (`00ec62c0`): truncates the float given as an `f64`.
const FTOL: u32 = 0x00ec_62c0;

/// Offset of the "current source line" word in the TLS block that
/// `Tile::Release` and `Tile::ReadFile` set to 13 for their duration.
const TLS_CURRENT_LINE: u32 = 0x2b4;

/// The names the text table registers whose value is computed at run time.
const XENON_NAME: u32 = 0x0106_f490;
const XBOX_NAME: u32 = 0x0106_f488;
const WIDESCREEN_NAME: u32 = 0x0109_47c8;

/// `Tile::enumType` (Xbox PDB): the tile types.
pub(crate) const TYPE_RECT: u32 = 0x385;
pub(crate) const TYPE_IMAGE: u32 = 0x386;
pub(crate) const TYPE_TEXT: u32 = 0x387;
pub(crate) const TYPE_3D: u32 = 0x388;
pub(crate) const TYPE_MENU: u32 = 0x389;
pub(crate) const TYPE_HOT_RECT: u32 = 0x38a;
pub(crate) const TYPE_RADIAL: u32 = 0x38c;

/// `Tile::enumbfUpdate` (Xbox PDB): the tile's update and state bits.
pub(crate) const UPDATE_POSITION: u32 = 0x1;
pub(crate) const UPDATE_CREATE: u32 = 0x2;
pub(crate) const UPDATE_VISIBILITY: u32 = 0x4;
pub(crate) const UPDATE_COLOR: u32 = 0x8;
pub(crate) const UPDATE_GEOMETRY: u32 = 0x10;
pub(crate) const UPDATE_TEXTURE: u32 = 0x20;
pub(crate) const UPDATE_NIF_FILE: u32 = 0x40;
pub(crate) const UPDATE_SCISSOR_WINDOW: u32 = 0x80;
pub(crate) const UPDATE_SCISSOR: u32 = 0x100;
pub(crate) const UPDATE_LOCUS: u32 = 0x200;
pub(crate) const FLAG_DIRTY: u32 = 0x400;
pub(crate) const FLAG_HIBERNATED: u32 = 0x800;
pub(crate) const FLAG_PROMOTED: u32 = 0x1000;
pub(crate) const FLAG_RELEASED: u32 = 0x2000;
pub(crate) const FLAG_MENU_DELETING: u32 = 0x4000;
pub(crate) const FLAG_TILE_LOADING: u32 = 0x10000;

/// `Tile::enumTrait` (Xbox PDB): the trait ids used so far.
pub(crate) const TRAIT_NONE: i32 = i32::MIN;
pub(crate) const TRAIT_X: i32 = 0xfa1;
pub(crate) const TRAIT_Y: i32 = 0xfa2;
pub(crate) const TRAIT_VISIBLE: i32 = 0xfa3;
pub(crate) const TRAIT_CLASS: i32 = 0xfa4;
pub(crate) const TRAIT_CLIP_WINDOW: i32 = 0xfa6;
pub(crate) const TRAIT_LOCUS: i32 = 0xfa8;
pub(crate) const TRAIT_ALPHA: i32 = 0xfa9;
pub(crate) const TRAIT_ID: i32 = 0xfaa;
pub(crate) const TRAIT_DEPTH: i32 = 0xfad;
pub(crate) const TRAIT_CLIPS: i32 = 0xfae;
pub(crate) const TRAIT_TARGET: i32 = 0xfaf;
pub(crate) const TRAIT_HEIGHT: i32 = 0xfb0;
pub(crate) const TRAIT_WIDTH: i32 = 0xfb1;
pub(crate) const TRAIT_RED: i32 = 0xfb2;
pub(crate) const TRAIT_GREEN: i32 = 0xfb3;
pub(crate) const TRAIT_BLUE: i32 = 0xfb4;
pub(crate) const TRAIT_JUSTIFY: i32 = 0xfb7;
pub(crate) const TRAIT_FONT: i32 = 0xfb9;
pub(crate) const TRAIT_WRAP_WIDTH: i32 = 0xfba;
pub(crate) const TRAIT_WRAP_LIMIT: i32 = 0xfbb;
pub(crate) const TRAIT_WRAP_LINES: i32 = 0xfbc;
pub(crate) const TRAIT_PAGE_NUM: i32 = 0xfbd;
pub(crate) const TRAIT_IS_HTML: i32 = 0xfbe;
pub(crate) const TRAIT_CROP_Y: i32 = 0xfbf;
pub(crate) const TRAIT_CROP_X: i32 = 0xfc0;
pub(crate) const TRAIT_STRING: i32 = 0xfc4;
pub(crate) const TRAIT_FILENAME: i32 = 0xfcc;
pub(crate) const TRAIT_SYSTEM_COLOR: i32 = 0xff4;
pub(crate) const TRAIT_BRIGHTNESS: i32 = 0xff5;
pub(crate) const TRAIT_LINE_GAP: i32 = 0xff7;
pub(crate) const TRAIT_TEX_ATLAS: i32 = 0xff9;
pub(crate) const TRAIT_ROTATE_ANGLE: i32 = 0xffa;
pub(crate) const TRAIT_ROTATE_AXIS_X: i32 = 0xffb;
pub(crate) const TRAIT_ROTATE_AXIS_Y: i32 = 0xffc;

/// `Tile::enumTag` (Xbox PDB): the tags for `<value>` text and for the
/// `name` attribute.
const TAG_VALUE: i32 = 0xbb9;
const TAG_NAME: i32 = 0xbba;

/// `Tile::VALUE_ACTION` (Xbox PDB): the left and right parenthesis actions.
const ACTION_LEFT_PAREN: i32 = 0x7e8;
const ACTION_RIGHT_PAREN: i32 = 0x7e9;

/// `Tile::TEMPLATE_ID` (Xbox PDB): the `iCmd` of a template item.
const TI_ACTION_START: i32 = 2;
const TI_ACTION_END: i32 = 3;
const TI_TRAIT_START: i32 = 4;
const TI_TRAIT_END: i32 = 5;
const TI_TILE_START: i32 = 6;
const TI_TILE_END: i32 = 7;
const TI_SIMPLE_TRAIT: i32 = 8;
const TI_SIMPLE_ACTION: i32 = 9;
const TI_TRAIT_LINK: i32 = 10;

// Session 2: constants of `GetChildByID` .. `Value::~Value`.

/// `Tile::enumbfUpdate` mask of the ten update bits (`ebfNeedsUpdateMask`).
pub(crate) const UPDATE_MASK: u32 = 0x3ff;
/// More `Tile::enumTrait` ids: `eStackingType`, `eChildCount`, `eZoom`,
/// `eFilewidth` and `eFileheight`.
pub(crate) const TRAIT_STACKING_TYPE: i32 = 0xfa7;
pub(crate) const TRAIT_CHILD_COUNT: i32 = 0xfb6;
pub(crate) const TRAIT_ZOOM: i32 = 0xfb8;
pub(crate) const TRAIT_FILE_WIDTH: i32 = 0xfcd;
pub(crate) const TRAIT_FILE_HEIGHT: i32 = 0xfce;
/// `Tile::VALUE_ACTION::VA_REF` (Xbox PDB): an action that reads another
/// trait.
const VA_REF: i32 = 2023;
/// `Tile::TILE_FADE_CONTROL_TYPE` (Xbox PDB).
const FADE_STANDARD: i32 = 0;
const FADE_NONLINEAR_LONG_DARK_REPEATING: i32 = 1;
const FADE_BLINK_THRICE: i32 = 2;
const FADE_BLINK_FAST_FADE: i32 = 3;
const FADE_IN_HOLD_FADE_OUT: i32 = 4;
/// The names of the user traits that count the flashes of the blinking
/// fades: `"_FlashCount"` and `"_TotalFlashCount"`.
const FLASH_COUNT_NAME: u32 = 0x0109_4c00;
const TOTAL_FLASH_COUNT_NAME: u32 = 0x0109_4bec;

/// `Tile::bNeedsCheckHibernate` (Xbox PDB static, a byte): set by
/// `UpdateTile` when a tile that was hidden became visible.
const NEEDS_CHECK_HIBERNATE: u32 = 0x011f_32c9;
/// `Tile::iTilesUpdatedThisFrame` (Xbox PDB static).
const TILES_UPDATED_THIS_FRAME: u32 = 0x011f_32d4;
/// `InterfaceManager::pInstance` (the word `011d8a80`), and its methods
/// that `UpdateAll` calls (`thiscall`): `AddTileToUpdateList(tile)`
/// (Xbox PDB) and the end-of-update step (no PDB name).
const INTERFACE_MANAGER: u32 = 0x011d_8a80;
const ADD_TILE_TO_UPDATE_LIST: u32 = 0x0071_3da0;
const INTERFACE_MANAGER_END_OF_UPDATE: u32 = 0x0071_3e20;
/// `Interface::GetFirstChanceTextureRelease()` (Xbox PDB), the step that
/// follows it (`cdecl(0)`, no PDB name) and
/// `BSTexturePalette::PurgeUnusedTextures()` (Xbox PDB).
const INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE: u32 = 0x0070_6d70;
const INTERFACE_PREPARE_TEXTURE_RELEASE: u32 = 0x0070_6db0;
const TEXTURE_PALETTE_PURGE_UNUSED: u32 = 0x00a6_1cd0;
/// `Interface::GetRealScreenWidth()` / `GetRealScreenHeight()` (Xbox PDB)
/// and the width and height of the rendered menu (no PDB names); each
/// answers a `float` in `ST0`.
const SCREEN_REAL_WIDTH: u32 = 0x0070_6e40;
const SCREEN_REAL_HEIGHT: u32 = 0x0070_6e10;
const RENDERED_MENU_WIDTH: u32 = 0x0070_6e80;
const RENDERED_MENU_HEIGHT: u32 = 0x0070_6e90;
/// The function `fn_00a08b20` calls for the link code `0x138f` (no PDB
/// name, `Interface`).
const INTERFACE_00706CF0: u32 = 0x0070_6cf0;
/// `NiAVObject::GetProperty(type)` (Xbox PDB, `thiscall`), the two
/// `NiTArray<NiPointer<NiAVObject>>` steps `Compact` and `UpdateSize` (Xbox
/// PDB, `thiscall` on the children array at +0x9C of an `NiNode`) and the
/// shader property's refresh step `(flag)` (no PDB name).
const NI_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
const NI_ARRAY_COMPACT: u32 = 0x004a_fc80;
const NI_ARRAY_UPDATE_SIZE: u32 = 0x004a_fe50;
const SHADER_PROPERTY_REFRESH: u32 = 0x00bb_79d0;
/// `NiTPointerList` steps (no PDB names; thiscall on the list): remove the
/// node `*iterator` and advance the iterator to the next node
/// `(list, &iterator)`; append `*item` `(list, &item)`; allocate a node
/// from the allocator `(allocator)`; link a node at the tail
/// `(list, node)`; add `*item` at the head `(list, &item)`; add `*item`
/// behind `node` `(list, node, &item)`.
const LIST_REMOVE_NODE: u32 = 0x0049_f590;
const LIST_ADD_TAIL: u32 = 0x004e_d8c0;
const LIST_NEW_NODE: u32 = 0x0043_a010;
const LIST_ADD_NODE_TAIL: u32 = 0x0055_9a70;
const LIST_ADD_HEAD: u32 = 0x00a0_c9f0;
const LIST_ADD_AFTER: u32 = 0x00a0_ca70;
/// `BSSimpleList<Tile::FadeControl *>::AddHead(&item)` and `Remove(&item)`
/// (Xbox PDB, `thiscall`).
const FADE_LIST_ADD_HEAD: u32 = 0x00a0_c7c0;
const FADE_LIST_REMOVE: u32 = 0x00a0_cbe0;
/// `Tile::ValueReactionList` (Xbox PDB static,
/// `NiTPointerMap<Tile::Value *, Tile::Reaction *>`) and its steps: find
/// `(map, key, &out)` (shared with the text table's find by identical-code
/// folding), erase `(map, key)` and set `(map, key, value)` (the latter
/// shared with [`TRAIT_EXTRA_DATA_ADD`]).
const REACTION_MAP: u32 = 0x011f_3358;
const REACTION_MAP_FIND: u32 = TEXT_TABLE_FIND;
const REACTION_MAP_REMOVE: u32 = 0x00a1_e240;
const REACTION_MAP_SET: u32 = TRAIT_EXTRA_DATA_ADD;
/// `Tile::AddReaction(value, owner)` (Xbox PDB, `cdecl`).
const ADD_REACTION: u32 = 0x00a0_a130;
/// The vtables of `Tile::FloatAction` and `Tile::RefValueAction`.
const VTABLE_FLOAT_ACTION: u32 = 0x0109_4c2c;
const VTABLE_REF_VALUE_ACTION: u32 = 0x0109_4c44;
/// `GetTickCount` (import slot), `_stricmp` and `_fabs` (CRT).
const TICK_COUNT_IMPORT: u32 = 0x00fd_f060;
const STRICMP: u32 = 0x00ec_68e4;
const FABS: u32 = 0x00ec_6cde;
/// Doubles in `.rdata` the code compares or computes with: `1000.0`,
/// `0.5`, `2.0`, `1.0`, `-1.0`, `3.0` and `255.0`.
const THOUSAND: u32 = 0x0101_7b70;
const HALF: u32 = 0x0101_1588;
const TWO: u32 = 0x0101_1590;
const ONE_DOUBLE: u32 = 0x0101_2070;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const THREE_DOUBLE: u32 = 0x0102_1928;
const TWO_FIFTY_FIVE_DOUBLE: u32 = 0x0101_e568;
/// The `id` values the update code recognises, as doubles: `110.0`,
/// and the stacking types `102.0` and `103.0` with the floats `6000.0`
/// and `6001.0` they are changed to.
const ID_VALUE_110: u32 = 0x0106_ebc8;
const STACKING_TYPE_102: u32 = 0x0107_04e8;
const STACKING_TYPE_103: u32 = 0x0109_4bb8;
const STACKING_VALUE_6000: u32 = 0x0109_4bc0;
const STACKING_VALUE_6001: u32 = 0x0109_4bb0;
/// The thresholds of the fade-in-hold-fade-out curve: `0.16666` and
/// `0.83334` as doubles (compared) and as floats (computed with).
const FADE_RISE_END_DOUBLE: u32 = 0x0109_4c20;
const FADE_FALL_START_DOUBLE: u32 = 0x0109_4c10;
const FADE_RISE_END_FLOAT: u32 = 0x0109_4c18;
const FADE_FALL_START_FLOAT: u32 = 0x0109_4c0c;
/// The link codes of [`fn_00a08b20`] (`Tile::enumKeyword`-style text table
/// values of the words that name a relative tile).
const LINK_PARENT: i32 = 0x1389;
const LINK_SELF: i32 = 0x138a;
const LINK_SIBLING: i32 = 0x138c;
const LINK_CHILD: i32 = 0x138d;
const LINK_MENUS_ROOT: i32 = 0x138e;
const LINK_00706CF0: i32 = 0x138f;
const LINK_MENU: i32 = 0x1390;
const LINK_GRANDPARENT: i32 = 0x1391;
/// The menu class table of `Tile::GetMenuByClass`: the class numbers it
/// accepts, the word at `011f350c` (table base) and the 16-bit entry count
/// at `011f3512`.
const MENU_CLASS_FIRST: i32 = 0x3e9;
const MENU_CLASS_LAST: i32 = 0x43c;
const MENU_CLASS_TABLE: u32 = 0x011f_350c;
const MENU_CLASS_COUNT: u32 = 0x011f_3512;

/// `Tile::enumTrait::eTile` (Xbox PDB).
pub(crate) const TRAIT_TILE: i32 = 0xfb5;
/// `Tile::enumbfUpdate::ebfManualUpdateTris` (Xbox PDB).
pub(crate) const FLAG_MANUAL_UPDATE_TRIS: u32 = 0x8000;
/// `Tile::VALUE_ACTION` (Xbox PDB), the operations of
/// [`value_calculate_value`] (`VA_REF` and the parentheses are above).
const VA_COPY: i32 = 2000;
const VA_ADD: i32 = 2001;
const VA_SUB: i32 = 2002;
const VA_MULT: i32 = 2003;
const VA_DIV: i32 = 2004;
const VA_MIN: i32 = 2005;
const VA_MAX: i32 = 2006;
const VA_MOD: i32 = 2007;
const VA_FLOOR: i32 = 2008;
const VA_CEIL: i32 = 2009;
const VA_ABS: i32 = 2010;
const VA_ROUND: i32 = 2011;
const VA_GT: i32 = 2012;
const VA_GTE: i32 = 2013;
const VA_EQ: i32 = 2014;
const VA_NEQ: i32 = 2015;
const VA_LT: i32 = 2016;
const VA_LTE: i32 = 2017;
const VA_AND: i32 = 2018;
const VA_OR: i32 = 2019;
const VA_NOT: i32 = 2020;
const VA_ONLYIF: i32 = 2021;
const VA_ONLYIFNOT: i32 = 2022;

/// Functions of other units (or of the CRT) that `Value::CalculateValue`
/// calls: the stack of floats it keeps (vtable `0106cc64`, constructor
/// `(0, 0)`, push `(&float)`, destructor; no PDB names),
/// `ValueChangeEvent(value)` (`cdecl`, next session), the lookup of a user
/// trait's value `(tile, trait, rounded value)` and `sprintf`; the CRT's
/// `floor`, `ceil` and `fabs`, which take and answer a `double`.
const VTABLE_FLOAT_STACK: u32 = 0x0106_cc64;
const FLOAT_STACK_CONSTRUCT: u32 = 0x006b_3eb0;
const FLOAT_STACK_PUSH: u32 = 0x006d_c320;
const FLOAT_STACK_DESTROY: u32 = 0x006d_b720;
const VALUE_CHANGE_EVENT: u32 = 0x00a0_a220;
const TILE_GET_UNDERSCORE_VALUE: u32 = 0x00a0_a0b0;
const SPRINTF_S: u32 = 0x0040_6d00;
const FORMAT_INTEGER: u32 = 0x0102_0764;
const FLOOR: u32 = 0x00ec_6940;
const CEIL: u32 = 0x00ec_9e10;

/// `UpdateTile`'s callees outside this file: the step after a text tile's
/// node was made (`cdecl(1)`, no PDB name), `TileShaderProperty::SetTileTexture
/// (texture)` (Xbox PDB), `TileImage::AddToTesTextures(name, &scale slot,
/// &out texture, zoom, counted copy of the old texture)` (Xbox PDB, `cdecl`),
/// the `BSStringT` constructor from a C string `(string, text)`, the
/// geometry data steps (`MarkAsChanged(flags)`, `SetConsistency(value)`) and
/// `NiBound::ComputeFromData(count, positions)`, and `Tile3D::UpdateNIF()`
/// (Xbox PDB).
const TEXT_TILE_CREATED: u32 = 0x0070_6dd0;
const SET_TILE_TEXTURE: u32 = 0x00bb_7a10;
const ADD_TO_TES_TEXTURES: u32 = 0x00a1_fa20;
const BSSTRING_CONSTRUCT: u32 = 0x0040_c0e0;
const GEOMETRY_DATA_MARK_AS_CHANGED: u32 = 0x00a6_7090;
const GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
const TILE_3D_UPDATE_NIF: u32 = 0x00a2_0980;
/// The placeholder texture `UpdateTile` gives an image tile without one:
/// `"Interface\\Shared\\empty.dds"`.
const EMPTY_TEXTURE_NAME: u32 = 0x0109_4bc8;
/// `100.0` as a `float` and as a `double`.
const HUNDRED_FLOAT: u32 = 0x0101_6410;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// The default translation of a model (three floats at `011f426c`) and
/// the double `-0.008` the depth is scaled by.
const DEFAULT_TRANSLATION: u32 = 0x011f_426c;
const DEPTH_SCALE: u32 = 0x0106_f290;
/// The `NiRTTI` record that `UpdateTile`'s rotation code looks for in an
/// object's class chain (virtual `GetRTTI`, slot +8; the chain is the word
/// at +4 of each record).
const SHAPE_RTTI: u32 = 0x011f_4a40;
/// `Interface::GetScreenWidth()` / `GetScreenHeight()` /
/// `GetScreenAspectRatio()` (Xbox PDB; floats in `ST0`) and
/// `Interface::IsRenderedMenu(menu)` (Xbox PDB, `cdecl`).
const SCREEN_WIDTH: u32 = 0x0070_6e00;
const SCREEN_HEIGHT: u32 = 0x0070_6df0;
const SCREEN_ASPECT_RATIO: u32 = 0x0070_6e70;
const INTERFACE_IS_RENDERED_MENU: u32 = 0x0070_7a30;
/// `NiMatrix3 * NiPoint3` `(matrix, out, point)`, `NiPoint3 + NiPoint3`
/// `(a, out, b)` and `NiPoint3 - NiPoint3` `(a, out, b)` (no PDB names,
/// `thiscall`; each answers `out`).
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `SystemColorManager::GetInstance()` and `GetColor(entry, &rgba)` (Xbox
/// PDB), and the colour scale step `(color, factor)` (no PDB name).
const SYSTEM_COLOR_MANAGER_GET_INSTANCE: u32 = 0x0071_8b60;
const SYSTEM_COLOR_MANAGER_GET_COLOR: u32 = 0x0071_9060;
const COLOR_SCALE: u32 = 0x0053_21d0;
/// The `id` value `111.0` (a double) and the name, brightness and alpha
/// of the dark red "speech challenge failure" highlight: `"lb_highlight_box"`,
/// `0.545f` and `0.2f`.
const ID_VALUE_111: u32 = 0x0106_ebc0;
const HIGHLIGHT_BOX_NAME: u32 = 0x0106_f898;
const HIGHLIGHT_RED: u32 = 0x0109_4bc4;
const HIGHLIGHT_ALPHA: u32 = 0x0101_df44;

/// For `GetTextureAtlasInfo`: the directory `"Data\\Textures\\"` the atlas
/// files are in; the token delimiters `" ,\t"` and `" ,"`;
/// `FileFinder::GetFile(path, 0, 0x4000)` (Xbox PDB); the CRT's `strcat_s`,
/// `strtok` and `atof`; the game's `std::string` steps (`_Tidy(built,
/// size)`, `assign(text, length)`, `rfind(&char, pos, length)`,
/// `substr(&out, pos, length)`, `assign(string, pos, length)` and
/// `append(text, length)`, thiscall, the last two and the first in
/// `BSMenu/tile.cpp` and next to it) with `std::string::npos`; and the
/// `Tile::TextureAtlasEntry` constructor (Xbox PDB `BSMenu/tile.cpp`,
/// next session).
const TEXTURE_DIRECTORY: u32 = 0x0103_3e9c;
const ATLAS_DELIMITERS: u32 = 0x0109_4be8;
const ATLAS_DELIMITERS_NO_TAB: u32 = 0x0109_4be4;
const FILE_FINDER_GET_FILE: u32 = 0x00af_df00;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRTOK: u32 = 0x00ec_c6e6;
const ATOF: u32 = 0x00ec_a573;
const STD_STRING_TIDY: u32 = 0x0044_a730;
const STD_STRING_ASSIGN: u32 = 0x0044_a7f0;
const STD_STRING_RFIND: u32 = 0x00a0_c300;
const STD_STRING_SUBSTR: u32 = 0x007a_b000;
const STD_STRING_ASSIGN_STRING: u32 = 0x0044_a580;
const STD_STRING_APPEND: u32 = 0x00a0_c430;
const STD_STRING_NPOS: u32 = 0x0101_73f0;
const TEXTURE_ATLAS_ENTRY_CONSTRUCT: u32 = 0x00a0_beb0;

// Session 3: constants of `GetUnderscoreValue` .. `NiExtraData::GetRTTI`.

/// The `"%s%d"` format of [`tile_get_underscore_value`].
const UNDERSCORE_NAME_FORMAT: u32 = 0x0109_4c50;
/// `sscanf` (CRT, no translation yet) and the `"%f"` format
/// [`template_add_pair`] reads a number with.
const SSCANF: u32 = 0x00ec_a4a6;
const FLOAT_SCAN_FORMAT: u32 = 0x0109_4cf0;
/// `-2147483648.0` as a `float` (`01094cf4`): the "no value yet" marker of
/// [`template_add_pair`] (the double at [`NO_TRAIT_VALUE`] is the same
/// number).
const NO_VALUE_FLOAT: u32 = 0x0109_4cf4;
/// The doubles [`template_add_pair`] compares with: `999.0` (the marker
/// value of a template definition), `901.0` and `908.0` (the range of the
/// tile-type numbers), `4001.0` and `4125.0` (the range of the trait
/// numbers), `2000.0` and `2025.0` (the range of the action numbers).
const TEMPLATE_MARKER_VALUE: u32 = 0x0109_4ce8;
const TILE_TYPE_FIRST_DOUBLE: u32 = 0x0109_4ca0;
const TILE_TYPE_LAST_DOUBLE: u32 = 0x0109_4c98;
const TRAIT_ID_FIRST_DOUBLE: u32 = 0x0109_4c90;
const TRAIT_ID_LAST_DOUBLE: u32 = 0x0109_4c88;
const ACTION_ID_FIRST_DOUBLE: u32 = 0x0107_3490;
const ACTION_ID_LAST_DOUBLE: u32 = 0x0109_4c80;
/// The error messages of [`template_add_pair`].
const MSG_NESTED_TEMPLATES: u32 = 0x0109_4ca8;
const MSG_BAD_TRAIT_OR_ACTION: u32 = 0x0109_4c58;
/// `Tile::enumTag` (Xbox PDB), more of the attribute words the XML reader
/// numbers (`"src"` at `01094394`, `"trait"` at `0109438c`).
const TAG_SRC: i32 = 0xbbb;
const TAG_TRAIT: i32 = 0xbbc;
/// The trait ids (`Tile::enumTrait`, `0xfa1` to `0x101d`) and the action
/// numbers (`Tile::VALUE_ACTION`, 2000 to `0x7e9`) as integers; ids from
/// [`FIRST_USER_TEXT_ID`] on are the user-defined names.
const TRAIT_ID_FIRST: i32 = 0xfa1;
const TRAIT_ID_LAST: i32 = 0x101d;
const ACTION_ID_FIRST: i32 = 2000;
const ACTION_ID_LAST: i32 = 0x7e9;
/// The `TEMPLATE_ID` command that [`template_add_pair`] gives an item whose
/// trait or action number is out of range (it also prints an error).
const TI_BAD: i32 = -1;
/// The doubly linked list of a template's items (`TileTemplate::xList`, an
/// `NiTPointerList`): link a node at the tail `(list, node)`, remove the
/// head and the tail and answer the element `(list)` (no PDB names;
/// `thiscall` on the list at +8 of the template).
const TEMPLATE_LIST_ADD_NODE_TAIL: u32 = 0x00a1_1db0;
const TEMPLATE_LIST_POP_HEAD: u32 = 0x00ae_7f90;
const TEMPLATE_LIST_POP_TAIL: u32 = 0x00ae_8010;
/// The pool of unused `Tile::TileTemplateItem`s (ten words at `011f329c`,
/// their count at `011f32dc`), and the constructor that builds a new item
/// `(this, command, float, text, trait, line)` (no PDB name, next session).
const ITEM_POOL: u32 = 0x011f_329c;
const ITEM_POOL_COUNT: u32 = 0x011f_32dc;
const ITEM_POOL_SIZE: u32 = 10;
const TEMPLATE_ITEM_CONSTRUCT: u32 = 0x00a0_c220;
/// The text a main template is made with (`01094d90`), the
/// `Tile::TileTemplate` constructor `(this, name, storage)` and its
/// destructor (Xbox PDB `Tile::TileTemplate::~TileTemplate`).
const MAIN_TEMPLATE_NAME: u32 = 0x0109_4d90;
const TEMPLATE_CONSTRUCT: u32 = 0x00a0_c0c0;
const TEMPLATE_DESTROY: u32 = 0x00a0_c190;
/// `BSSimpleList<Tile::TileTemplate *>::AddHead(&item)` (thiscall on the
/// list head embedded in a `BuildStorage`).
const SUB_TEMPLATE_LIST_ADD_HEAD: u32 = 0x00af_25b0;
/// `NiFixedString::NiFixedString(char *)` (answers the counted handle), the
/// handle of the empty string (the word at `0109b220`), and the import
/// slot of `InterlockedDecrement` that releases a handle.
const NI_FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The vtable of `Tile::Extra` and the destructor of its base class
/// `NiExtraData` (`thiscall`).
const VTABLE_EXTRA: u32 = 0x0109_4cfc;
const NI_EXTRA_DATA_DESTROY: u32 = 0x00a7_b300;
/// The sound code `PlayTileSound` uses: the audio manager pointer
/// (`011f6d98`), `BSAudio::GetSoundHandleByName(&out, name, flags)` and
/// `BSSoundHandle::Play()` (Xbox PDB, `thiscall`).
const AUDIO_MANAGER: u32 = 0x011f_6d98;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const TILE_SOUND_FLAGS: u32 = 0x121;
/// `NiMatrix3::MakeYRotation(angle)` (Xbox PDB, `thiscall` on the matrix).
const MATRIX_MAKE_Y_ROTATION: u32 = 0x0043_f850;
/// The vtable of `Tile3D`, its destructor `~Tile3D` (`thiscall`), and the
/// type names `TileText` and `Tile3D` answer.
const VTABLE_TILE_3D: u32 = 0x0109_48ac;
const TILE_3D_DESTROY: u32 = 0x00a2_0650;
const TILE_TEXT_TYPE_NAME: u32 = 0x0109_48a0;
const TILE_3D_TYPE_NAME: u32 = 0x0109_48d4;
/// `memcmp` (CRT) and the two bytes `"&_"` it compares a name's start with.
const MEMCMP: u32 = 0x00ec_4835;
const USER_NAME_PREFIX: u32 = 0x0109_3f80;
/// The `NiRTTI` record that `NiExtraData::GetRTTI` answers.
const EXTRA_DATA_RTTI: u32 = 0x011f_4a80;

// Session 4: constants of `Tile::Extra`'s deleting destructor to the hash
// map code.

/// `operator delete(block, size)` (`cdecl`, as `Tile::Extra` is deleted),
/// `alloc(bytes)` and `free(block)` (`cdecl`, the bucket arrays and the
/// copied keys of the hash maps; no PDB names).
const NI_OPERATOR_DELETE: u32 = 0x00aa_1460;
const ALLOC_BYTES: u32 = 0x00aa_1070;
const FREE_BYTES: u32 = 0x00aa_10f0;
/// Destructor of the `NiTPointerList` at +8 of a `Tile::TileTemplate` (no
/// PDB name; code shared with the audio library by identical-code folding).
const LIST_DESTROY: u32 = 0x00ae_8370;
/// `std::string` steps (thiscall on the string): `_Inside(ptr)` (true when
/// the pointer is into the string's own buffer) and `_Grow(size, trim)`
/// (true when the buffer holds `size` characters); the runtime's thrower of
/// `length_error` and of `out_of_range`; `memcpy_s`, `memmove`, `bsearch`
/// and `tolower`.
const STD_STRING_INSIDE: u32 = 0x0044_ab90;
const STD_STRING_GROW: u32 = 0x0044_a8d0;
const STD_LENGTH_ERROR: u32 = 0x00ec_30b0;
const STD_OUT_OF_RANGE: u32 = 0x00ec_30e8;
const MEMCPY_S: u32 = 0x00ec_7c66;
const MEMMOVE: u32 = 0x00ec_7230;
const BSEARCH: u32 = 0x00ec_716d;
const TOLOWER: u32 = 0x00ec_67aa;
/// Gives a node back to the free pool of a map's allocator `(allocator,
/// node)` (no PDB name; the allocator is at +0xC of the map).
const MAP_FREE_NODE: u32 = 0x0045_cee0;
/// The vtables the hash-map destructors write while they unwind the class
/// chain: `Tile::MenuStringMap` (`01094d9c`), `NiTStringTemplateMap<...>`
/// (`01094ddc`), `NiTPointerMap<char const *, int>` (`01094dfc`),
/// `NiTMapBase<... char const *, int>` (`01094e1c`) and the two levels of
/// `NiTPointerMap<int, int>` (`01094e3c`, `01094e5c`).
const VTABLE_MENU_STRING_MAP: u32 = 0x0109_4d9c;
const VTABLE_STRING_TEMPLATE_MAP: u32 = 0x0109_4ddc;
const VTABLE_POINTER_MAP: u32 = 0x0109_4dfc;
const VTABLE_MAP_BASE: u32 = 0x0109_4e1c;
const VTABLE_INT_POINTER_MAP: u32 = 0x0109_4e3c;
const VTABLE_INT_MAP_BASE: u32 = 0x0109_4e5c;
/// The two levels of the map of `Tile::Value *` to `Tile::Reaction *`
/// (`NiTPointerMap<Tile::Value *, Tile::Reaction *>`, `01094e7c`) and its
/// base class (`01094e9c`).
const VTABLE_REACTION_POINTER_MAP: u32 = 0x0109_4e7c;
const VTABLE_REACTION_MAP_BASE: u32 = 0x0109_4e9c;
/// Slots of the hash maps' vtable that the shared map code calls: the hash
/// of a key `(key)`, the key comparison `(a, b)`, the construction of a
/// node's key and value `(node, key, value)` and the allocation of a node.
const MAP_SLOT_HASH: u32 = 0x4;
const MAP_SLOT_KEYS_EQUAL: u32 = 0x8;
const MAP_SLOT_INIT_NODE: u32 = 0xc;
const MAP_SLOT_NEW_NODE: u32 = 0x14;
/// Slots of the vtable of a `BSSimpleArray`: allocate `(count)`, free
/// `(block)` and reallocate `(block, count)`.
const ARRAY_SLOT_ALLOCATE: u32 = 0x4;
const ARRAY_SLOT_FREE: u32 = 0x8;
const ARRAY_SLOT_REALLOCATE: u32 = 0xc;

/// The characters of a `std::string` (0x1c bytes, `_DebugHeapAllocator`):
/// the text is inline at +4 while the capacity (+0x18) is below 0x10, else
/// the pointer at +4 holds it; the length is at +0x14.
fn std_string_data(e: &Engine, string: u32) -> u32 {
    if e.mem.u32(string + 0x18) >= 0x10 {
        e.mem.u32(string + 4)
    } else {
        string + 4
    }
}

/// The remainder of `value` by the bucket count at +4 of a hash map (the
/// game divides by it unchecked).
fn bucket_index(e: &Engine, map: u32, value: u32) -> u32 {
    let buckets = e.mem.u32(map + 4);
    value
        .checked_rem(buckets)
        .expect("hash map without buckets (division by zero in the game)")
}

/// `!(a < b)`: true when `a >= b` and when the two are unordered, as the
/// FPU compare-and-branch sequences test it.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn not_less(a: f64, b: f64) -> bool {
    !(a < b)
}

/// `!(a > b)`: true when `a <= b` and when the two are unordered.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn not_greater(a: f64, b: f64) -> bool {
    !(a > b)
}

// ---------------------------------------------------------------------------
// Helpers

fn lock(e: &mut Engine) {
    e.call(TILE_LOCK, &args![]);
}

fn unlock(e: &mut Engine) {
    e.call(TILE_UNLOCK, &args![]);
}

fn enter_critical_section(e: &mut Engine, section: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
}

fn leave_critical_section(e: &mut Engine, section: u32) {
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

/// `new`: a block from the memory manager singleton.
fn allocate(e: &mut Engine, size: u32) -> Ptr {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).ptr()
}

/// `delete`: gives a block back to the memory manager singleton.
fn deallocate(e: &mut Engine, block: u32) {
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// True when a `float` differs from the `0.0` double in `.rdata` (or is not
/// a number), as the code's `FCOMP` / `TEST AH,44` idiom tests it.
fn differs_from_zero(e: &Engine, value: f32) -> bool {
    value as f64 != e.global::<f64>(ZERO)
}

/// `FISTP` of a `float` in the default rounding mode: round to nearest,
/// ties to even; out of range or not a number gives the "integer
/// indefinite" value `0x80000000`.
fn float_to_int_rounded(value: f32) -> i32 {
    let value = value as f64;
    let below = value.floor();
    let rounded = if value - below > 0.5 || (value - below == 0.5 && below % 2.0 != 0.0) {
        below + 1.0
    } else {
        below
    };
    if rounded.is_nan() || !(-2147483648.0..2147483648.0).contains(&rounded) {
        i32::MIN
    } else {
        rounded as i32
    }
}

/// `_ftol2_sse` on a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// The tile's virtual `GetType` (slot +0xC, Xbox PDB `Tile::GetType`).
fn tile_type(e: &mut Engine, tile: Ptr<Tile>) -> u32 {
    e.vcall(tile.addr(), 0xc, &[]).u32()
}

/// The template a `BuildStorage` adds items to: the current sub-template,
/// else the main one.
fn storage_template(e: &Engine, storage: Ptr<BuildStorage>) -> Ptr<TileTemplate> {
    let current = e.get(storage, BuildStorage::pCurrentTemplate);
    if current.is_null() {
        e.get(storage, BuildStorage::pTemplate)
    } else {
        current
    }
}

/// `NiPointer::~NiPointer` on a raw pointer: drops one reference through
/// `InterlockedDecrement` and, at zero, calls the object's virtual
/// destructor (slot +4).
fn release_reference(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// The names `Tile::LoadTextTable` registers, as `(name address, value)`,
/// in the order the game registers them. Three values are computed at run
/// time and are 0 here (`&xenon;`, `&xbox;`, `&widescreen;`).
const TEXT_TABLE_ENTRIES: &[(u32, i32)] = &[
    (0x0109_4868, -0x1),
    (0x0106_ec74, 0x0),
    (0x0106_ec7c, 0x1),
    (0x0109_4860, 0x1),
    (0x0109_4854, 0x2),
    (0x0109_484c, 0x4),
    (0x0109_4844, 0x5),
    (0x0109_483c, 0x6),
    (0x0109_4834, -0x1),
    (0x0109_4824, 0x65),
    (0x0109_4814, 0x66),
    (0x0109_4800, 0x1778),
    (0x0109_47f0, 0x67),
    (XENON_NAME, 0),
    (0x0109_47e4, 0x0),
    (XBOX_NAME, 0),
    (0x0109_47d8, 0x1),
    (WIDESCREEN_NAME, 0),
    (0x0109_47b8, 0x6e),
    (0x0109_47a8, 0x6f),
    (0x0109_4798, 0x70),
    (0x0109_4788, -0x1),
    (0x0109_4778, 0x0),
    (0x0109_476c, -0x1),
    (0x0109_4758, 0x9),
    (0x0109_474c, 0x9),
    (0x0109_4740, 0xa),
    (0x0109_4734, 0xb),
    (0x0109_4728, 0xc),
    (0x0109_471c, 0xd),
    (0x0109_4710, 0xe),
    (0x0109_4704, 0xf),
    (0x0109_46f8, 0x10),
    (0x0107_7c78, 0x11),
    (0x0107_7c6c, 0x12),
    (0x0109_46ec, 0x11),
    (0x0109_46e0, 0x12),
    (0x0109_46d8, 0x386),
    (0x0109_46d0, 0x389),
    (0x0109_46cc, 0x388),
    (0x0109_46c8, 0x388),
    (0x0109_46c0, 0x385),
    (0x0109_46b8, 0x38a),
    (0x0109_46ac, 0x3e7),
    (0x0103_c83c, 0x387),
    (0x0109_46a4, 0x38b),
    (0x0109_469c, 0x38c),
    (0x0109_4690, 0x3f9),
    (0x0109_4684, 0x402),
    (0x0109_4674, 0x418),
    (0x0109_4664, 0x3f0),
    (0x0109_4654, 0x3f1),
    (0x0109_4644, 0x3fc),
    (0x0109_4634, 0x3ec),
    (0x0109_4624, 0x3ea),
    (0x0109_4614, 0x3ef),
    (0x0109_4604, 0x3f6),
    (0x0109_45f8, 0x3ff),
    (0x0109_45e8, 0x3e9),
    (0x0109_45d8, 0x423),
    (0x0109_45c8, 0x41b),
    (0x0109_45bc, 0x3f5),
    (0x0109_45ac, 0x3f8),
    (0x0109_459c, 0x40c),
    (0x0109_458c, 0x3f4),
    (0x0109_4580, 0x3eb),
    (0x0109_4574, 0x3fa),
    (0x0109_4564, 0x403),
    (0x0109_4554, 0x40b),
    (0x0109_4544, 0x425),
    (0x0109_452c, 0x422),
    (0x0109_451c, 0x417),
    (0x0109_450c, 0x41d),
    (0x0109_44fc, 0x41e),
    (0x0109_44ec, 0x41f),
    (0x0109_44dc, 0x421),
    (0x0109_44d0, 0x420),
    (0x0109_44bc, 0x424),
    (0x0109_44a4, 0x433),
    (0x0109_4490, 0x432),
    (0x0109_447c, 0x434),
    (0x0109_4468, 0x438),
    (0x0109_4458, 0x439),
    (0x0109_4448, 0x43a),
    (0x0109_4438, 0x435),
    (0x0109_4428, 0x43b),
    (0x0109_441c, 0x43c),
    (0x0109_4418, 0x7da),
    (0x0109_4414, 0x7d1),
    (0x0109_4410, 0x7e2),
    (0x0109_4408, 0x7d9),
    (0x0109_4400, 0x7d0),
    (0x0109_43fc, 0x7d4),
    (0x0109_43f8, 0x7de),
    (0x0109_43f0, 0x7d8),
    (0x0109_43ec, 0x7dc),
    (0x0109_43e8, 0x7dd),
    (0x0107_6f38, 0x7e0),
    (0x0109_43e4, 0x7e1),
    (0x0109_43e0, 0x7d6),
    (0x0109_43dc, 0x7d5),
    (0x0109_43d8, 0x7d7),
    (0x0109_43d4, 0x7d3),
    (0x0109_43cc, 0x7d3),
    (0x0109_43c8, 0x7df),
    (0x0109_43c4, 0x7e4),
    (0x0109_43bc, 0x7e5),
    (0x0109_43b0, 0x7e6),
    (0x0109_43ac, 0x7e3),
    (0x0104_4c0c, 0x7e7),
    (0x0109_43a4, 0x7db),
    (0x0109_43a0, 0x7d2),
    (0x0109_4398, 0xbba),
    (0x0109_4394, 0xbbb),
    (0x0109_438c, 0xbbc),
    (0x0109_4384, 0xbb9),
    (0x0103_a454, 0xfa9),
    (0x0109_4378, 0xfd2),
    (0x0109_4370, 0xfb4),
    (0x0109_4364, 0xff5),
    (0x0109_4358, 0xfb6),
    (0x0109_434c, 0xfb6),
    (0x0109_4344, 0xfa4),
    (0x0109_433c, 0xfc7),
    (0x0109_4330, 0xfcb),
    (0x0109_4320, 0xfe8),
    (0x0109_4314, 0xfab),
    (0x0109_4308, 0xfc0),
    (0x0109_42fc, 0xfbf),
    (0x0109_42f4, 0xfc0),
    (0x0109_42ec, 0xfbf),
    (0x0109_42e4, 0xfae),
    (0x0109_42d8, 0xfa6),
    (0x0103_c034, 0xfad),
    (0x0109_42cc, 0xfe9),
    (0x0109_42c0, 0xfee),
    (0x0109_42b4, 0xfef),
    (0x0109_42a8, 0xfea),
    (0x0109_429c, 0xfeb),
    (0x0109_4290, 0xfec),
    (0x0109_4284, 0xfed),
    (0x0109_427c, 0xff0),
    (0x0109_4274, 0xff1),
    (0x0109_4268, 0xfc2),
    (0x0109_425c, 0xfce),
    (0x0109_4250, 0xfcc),
    (0x0109_4244, 0xfcd),
    (0x0109_423c, 0xfb9),
    (0x0109_4234, 0xfb3),
    (0x0109_422c, 0xfb0),
    (0x0109_4228, 0xfaa),
    (0x0109_4220, 0xfbe),
    (0x0109_4218, 0xfb7),
    (0x0109_420c, 0xfd4),
    (0x0109_4204, 0xff7),
    (0x0109_41f8, 0xfac),
    (0x0109_41f0, 0xfa8),
    (0x0109_41e4, 0xfc1),
    (0x0109_41d8, 0xfc3),
    (0x0109_41cc, 0xfd5),
    (0x0109_41c4, 0xfbd),
    (0x0109_41c0, 0xfb2),
    (0x0109_41b0, 0xfcf),
    (0x0109_419c, 0xfd0),
    (0x0109_4190, 0xffa),
    (0x0109_4184, 0xffb),
    (0x0109_4178, 0xffc),
    (0x0109_4168, 0xfc5),
    (0x0109_4158, 0xfa7),
    (0x0109_4150, 0xfc4),
    (0x0109_4144, 0xff4),
    (0x0108_dff8, 0xfaf),
    (0x0109_4138, 0xff9),
    (0x0109_4130, 0xfb5),
    (0x0109_4128, 0x1004),
    (0x0109_4120, 0x1005),
    (0x0109_4118, 0x1006),
    (0x0109_4110, 0x1007),
    (0x0109_4108, 0x1008),
    (0x0109_4100, 0x1009),
    (0x0109_40f8, 0x100a),
    (0x0109_40f0, 0x100b),
    (0x0109_40e8, 0x100c),
    (0x0109_40e0, 0x100d),
    (0x0109_40d8, 0x100e),
    (0x0109_40d0, 0x100f),
    (0x0109_40c8, 0x1010),
    (0x0109_40c0, 0x1011),
    (0x0109_40b8, 0x1012),
    (0x0109_40b0, 0x1013),
    (0x0109_40a8, 0x1014),
    (0x0106_fc9c, 0xfa3),
    (0x0109_409c, 0xff2),
    (0x0109_4090, 0xff3),
    (0x0109_4088, 0xfb1),
    (0x0109_407c, 0xfbb),
    (0x0109_4070, 0xfbc),
    (0x0109_4064, 0xfba),
    (0x0109_4058, 0xfdd),
    (0x0109_404c, 0xfde),
    (0x0109_4040, 0xfdf),
    (0x0109_4034, 0xfe0),
    (0x0109_4028, 0xfe1),
    (0x0109_401c, 0xfe2),
    (0x0109_4010, 0xfe3),
    (0x0109_4004, 0xfe4),
    (0x0109_3ff4, 0xfe7),
    (0x0109_3fe8, 0xfd6),
    (0x0109_3fe4, 0xfd7),
    (0x0109_3fdc, 0xfd8),
    (0x0109_3fd4, 0xfd9),
    (0x0109_3fcc, 0xfda),
    (0x0103_45e4, 0xfa1),
    (0x0107_6f20, 0xfa2),
    (0x0109_3fc4, 0xfb8),
    (0x0109_3fb0, 0xff8),
    (0x0108_6f50, 0x138d),
    (0x0109_3fac, 0x138a),
    (0x0109_3fa8, 0x1390),
    (0x0109_3fa0, 0x1389),
    (0x0107_8bc0, 0x138e),
    (0x0109_3f98, 0x138c),
    (0x0109_3f8c, 0x1391),
    (0x0109_3f84, 0x138f),
];

/// `Tile::SortedFind` comparison of two trait values; the address is also
/// what [`tile_get_value`] hands to the sorted-array functions.
const COMPARE_VALUES: u32 = 0x00a0_1160;
/// Called with the tile's model while the critical section is held, from
/// the destructor (no Xbox PDB name; the `BSMain` range).
const MODEL_HOOK_00C45830: u32 = 0x00c4_5830;

/// `LIST_REMOVE_ITEM` on `list` with the tile pointer passed by address;
/// true when the answer is not null (it answers the element, which is the
/// tile itself whether or not it was in the list).
fn remove_from_list(e: &mut Engine, list: u32, tile: Ptr<Tile>) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), tile.addr());
        e.call(LIST_REMOVE_ITEM, &args![list, slot]).u32() != 0
    })
}

// Translated from 009ff340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::~Tile` (Xbox PDB): under the tile critical section, tells the
/// interface the tile is going away, releases it first if nobody did
/// (printing a complaint), takes it out of the dirty and hibernating lists,
/// then drops the model and destroys the name, traits and children.
///
/// The code the compiler folded away (setting `FLAG_DIRTY`, `FLAG_HIBERNATED`
/// and `FLAG_PROMOTED` instead of clearing them) is not translated.
pub fn tile_destructor(e: &mut Engine, this: Ptr<Tile>) {
    e.mem.set_u32(this.addr(), VTABLE_TILE);
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    e.call(INTERFACE_TILE_IS_BEING_DELETED, &args![this]);
    if e.get(this, Tile::uiFlags) & FLAG_RELEASED == 0 {
        e.call(PRINT_ERROR, &args![MSG_BASE_TILE_NOT_RELEASED]);
        tile_release(e, this);
    }
    if e.get(this, Tile::uiFlags) & FLAG_DIRTY != 0 {
        remove_from_list(e, DIRTY_TILES_LIST, this);
    }
    let flags = e.get(this, Tile::uiFlags);
    e.set(this, Tile::uiFlags, flags & !FLAG_DIRTY);
    if remove_from_list(e, HIBERNATING_TILES_LIST, this)
        && e.get(this, Tile::uiFlags) & FLAG_HIBERNATED != 0
    {
        let count: u32 = e.global(HIBERNATING_TILE_COUNT);
        e.set_global(HIBERNATING_TILE_COUNT, count.wrapping_sub(1));
    }
    let flags = e.get(this, Tile::uiFlags);
    e.set(this, Tile::uiFlags, flags & !FLAG_HIBERNATED);
    let flags = e.get(this, Tile::uiFlags);
    e.set(this, Tile::uiFlags, flags & !FLAG_PROMOTED);
    let model = e.get(this, Tile::spModel);
    e.call(MODEL_HOOK_00C45830, &args![model]);
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    release_reference(e, e.get(this, Tile::spModel).addr());
    e.call(STRING_SET, &args![this.at(Tile::xName), 0u32, 0u32]);
    e.call(VALUES_DESTROY, &args![this.at(Tile::xTraits)]);
    e.call(LIST_REMOVE_ALL, &args![this.at(Tile::xChildren)]);
}

// Translated from 009ff600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Init` (Xbox PDB): clears the parent, model, flags and update
/// flag, sets the name to the empty string, attaches the tile to `parent`
/// (when given, passing `set_parent_arg` through) and gives it `name` (when
/// given).
pub fn tile_init(
    e: &mut Engine,
    this: Ptr<Tile>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.set(this, Tile::pParent, Ptr::NULL);
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), 0u32]);
    e.call(STRING_SET, &args![this.at(Tile::xName), EMPTY_NAME, 0u32]);
    e.set(this, Tile::bSpeechChallengeFailure, false);
    e.set(this, Tile::uiFlags, 0);
    if !parent.is_null() {
        e.call(TILE_SET_PARENT, &args![this, parent, set_parent_arg]);
    }
    if !name.is_null() {
        e.call(STRING_SET, &args![this.at(Tile::xName), name, 0u32]);
    }
}

// Translated from 009ff690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Release` (Xbox PDB): the tile's teardown, under `Tile::Lock` and
/// with the thread's current-line word set to 13: marks the tile and its
/// children as being deleted, tells the interface, sets `FLAG_RELEASED`,
/// severs the model's pointer back to the tile, detaches from the parent,
/// destroys and frees every trait value, empties the trait array, drops the
/// fade control, detaches the model from its parent node and deletes every
/// child.
pub fn tile_release(e: &mut Engine, this: Ptr<Tile>) {
    let tls = e.tls() + TLS_CURRENT_LINE;
    let saved_line = e.mem.u32(tls);
    e.mem.set_u32(tls, 0xd);
    lock(e);
    tile_set_menu_deleting(e, this);
    e.call(INTERFACE_TILE_IS_BEING_DELETED, &args![this]);
    let flags = e.get(this, Tile::uiFlags);
    e.set(this, Tile::uiFlags, flags | FLAG_RELEASED);
    tile_sever_extra_data(e, this);
    e.call(TILE_SET_PARENT, &args![this, 0u32, 0u32]);

    let traits = this.at(Tile::xTraits);
    let mut index = 0;
    while index < e.get(traits, BSSimpleArray::iSize) {
        let buffer = e.get(traits, BSSimpleArray::pBuffer);
        let value = e.mem.u32(buffer + index * 4);
        if value != 0 {
            e.call(VALUE_DESTROY, &args![value]);
            deallocate(e, value);
        }
        index += 1;
    }
    e.call(VALUES_CLEAR, &args![traits, 1u32]);
    e.call(TILE_REMOVE_FADE_CONTROL, &args![this, 0x8000_0000u32]);

    // The model's parent node removes the model (virtual slot +0xE8).
    let model = e.get(this, Tile::spModel);
    if !model.is_null() {
        let parent_node = e.mem.u32(model.addr() + 0x18);
        if parent_node != 0 {
            e.vcall(parent_node, 0xe8, &args![model]);
        }
    }

    let children = this.at(Tile::xChildren);
    while e.get(children, NiTPointerList::m_uiCount) != 0 {
        let child = e.call(LIST_REMOVE_HEAD, &args![children]).u32();
        if child != 0 {
            // Scalar deleting destructor, slot 0, with the "delete" flag.
            e.vcall(child, 0, &args![1u32]);
        }
    }
    unlock(e);
    e.mem.set_u32(tls, saved_line);
}

// Translated from 009ff8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers `name` in the text table with `id` (nothing for an empty or
/// missing name). A user trait name (one `TEXT_IS_USER_NAME` accepts) with an
/// id from 10000 up also raises the next free user id past `id` and enters
/// the id in the by-id map under the number at the end of the name (after
/// the last `_`), when it has one.
pub fn fn_009ff8a0(e: &mut Engine, name: Ptr, id: i32) {
    if name.is_null() || e.mem.i8(name.addr()) == 0 {
        return;
    }
    lock(e);
    e.call(TEXT_TABLE_ADD, &args![TEXT_TABLE, name, id]);
    if e.call(TEXT_IS_USER_NAME, &args![name]).bool() && id >= FIRST_USER_TEXT_ID {
        if e.global::<i32>(TEXT_TABLE_USER_NEXT_ID) <= id {
            e.set_global(TEXT_TABLE_USER_NEXT_ID, id.wrapping_add(1));
        }
        let suffix = fn_00a01a20(e, name);
        if suffix != 0 {
            e.call(
                TRAIT_EXTRA_DATA_ADD,
                &args![TRAIT_EXTRA_DATA_MAP, id, suffix],
            );
        }
    }
    unlock(e);
}

// Translated from 009ff970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::LoadTextTable` (Xbox PDB): fills the text table with the words
/// the menu XML uses (`&true;`, tile types, menu names, trait names,
/// keywords), 224 of them in a fixed order ([`TEXT_TABLE_ENTRIES`]). The
/// values of `&xenon;` and `&xbox;` (a 360 controller is present and the
/// console-UI byte is 0) and `&widescreen;` (the interface's query) are
/// computed when their turn comes.
pub fn tile_load_text_table(e: &mut Engine) {
    for &(name, value) in TEXT_TABLE_ENTRIES {
        let value = match name {
            XENON_NAME | XBOX_NAME => {
                (e.global::<u8>(HAS_360_CONTROLLER) != 0 && e.global::<u8>(CONSOLE_UI_BYTE) == 0)
                    as i32
            }
            WIDESCREEN_NAME => e.call(INTERFACE_IS_WIDESCREEN, &args![]).u8() as i32,
            _ => value,
        };
        fn_009ff8a0(e, Ptr::new(name), value);
    }
}

// Translated from 00a00940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::AddUserTrait` (Xbox PDB): the id of `name` in the text table; a
/// new name gets `id` (the next free user id when `id` is -1) if that is a
/// valid user id (from 10000 up, and `TEXT_IS_USER_NAME` accepts the name),
/// else the result is `0x80000000` (`eNone`).
pub fn tile_add_user_trait(e: &mut Engine, name: Ptr, id: i32) -> i32 {
    lock(e);
    let (found, existing) = e.with_stack(4, |e, out| {
        let found = e
            .call(TEXT_TABLE_FIND, &args![TEXT_TABLE, name, out])
            .bool();
        (found, e.mem.i32(out.addr()))
    });
    if found {
        unlock(e);
        return existing;
    }
    let mut id = id;
    if id == -1 {
        id = e.global(TEXT_TABLE_USER_NEXT_ID);
        e.set_global(TEXT_TABLE_USER_NEXT_ID, id.wrapping_add(1));
    }
    let result = if id >= FIRST_USER_TEXT_ID && e.call(TEXT_IS_USER_NAME, &args![name]).bool() {
        fn_009ff8a0(e, name, id);
        id
    } else {
        TRAIT_NONE
    };
    unlock(e);
    result
}

// Translated from 00a00a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::FreeTraitList` (Xbox PDB): empties the text table and the
/// trait-extra-data map, destroys and frees every entry of the texture atlas
/// entry list and frees (without a destructor) every item of the second
/// static list, then empties both lists.
pub fn tile_free_trait_list(e: &mut Engine) {
    lock(e);
    e.call(MAP_REMOVE_ALL, &args![TEXT_TABLE]);
    e.call(MAP_REMOVE_ALL, &args![TRAIT_EXTRA_DATA_MAP]);
    let mut node = TEXTURE_ENTRY_LIST;
    while node != 0 && e.mem.u32(node) != 0 {
        let item = e.mem.u32(node);
        e.call(TEXTURE_ATLAS_ENTRY_DESTROY, &args![item]);
        deallocate(e, item);
        node = e.mem.u32(node + 4);
    }
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![TEXTURE_ENTRY_LIST]);
    let mut node = FADE_CONTROLS_LIST;
    while node != 0 && e.mem.u32(node) != 0 {
        let item = e.mem.u32(node);
        deallocate(e, item);
        node = e.mem.u32(node + 4);
    }
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![FADE_CONTROLS_LIST]);
    unlock(e);
}

// Translated from 00a00b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SetMenuDeleting` (Xbox PDB): sets `FLAG_MENU_DELETING` on the
/// tile and, recursively, on its children, unless the tile has been
/// released or is already marked.
pub fn tile_set_menu_deleting(e: &mut Engine, this: Ptr<Tile>) {
    lock(e);
    let flags = e.get(this, Tile::uiFlags);
    if flags & FLAG_RELEASED == 0 && flags & FLAG_MENU_DELETING == 0 {
        e.set(this, Tile::uiFlags, flags | FLAG_MENU_DELETING);
        let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
        while node != 0 {
            let child = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            tile_set_menu_deleting(e, Ptr::new(child));
        }
    }
    unlock(e);
}

// Translated from 00a00c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::BuildTile` (Xbox PDB): allocates and constructs a tile of the
/// type `kind` (`TYPE_*`): the rect (0x3c bytes), the image and hot rect
/// (0x48), the text (0x4c, vtable set and a trailing byte cleared), the 3D
/// tile (0x50), the menu (0x40, last word cleared) and the radial tile
/// (0x48). Any other type (the window, template and unknown types) gives
/// null, as does a failed allocation.
///
/// The compiler folded away a call to `SetParent(0, 0)` in the text case.
pub fn tile_build_tile(e: &mut Engine, kind: u32) -> Ptr {
    let size = match kind {
        TYPE_RECT => 0x3c,
        TYPE_IMAGE | TYPE_HOT_RECT | TYPE_RADIAL => 0x48,
        TYPE_TEXT => 0x4c,
        TYPE_3D => 0x50,
        TYPE_MENU => 0x40,
        _ => return Ptr::NULL,
    };
    let block = allocate(e, size);
    if block.is_null() {
        return Ptr::NULL;
    }
    match kind {
        TYPE_RECT => {
            e.call(TILE_BASE_CONSTRUCT, &args![block]);
            e.mem.set_u32(block.addr(), VTABLE_TILE_RECT);
            block
        }
        TYPE_IMAGE | TYPE_HOT_RECT => e.call(TILE_IMAGE_CONSTRUCT, &args![block]).ptr(),
        TYPE_TEXT => {
            e.call(TILE_BASE_CONSTRUCT, &args![block]);
            e.mem.set_u32(block.addr(), VTABLE_TILE_TEXT);
            e.mem.set_u8(block.addr() + 0x48, 0);
            block
        }
        TYPE_3D => e.call(TILE_3D_CONSTRUCT, &args![block]).ptr(),
        TYPE_MENU => {
            e.call(TILE_BASE_CONSTRUCT, &args![block]);
            // The rect vtable is written first and replaced at once.
            e.mem.set_u32(block.addr(), VTABLE_TILE_RECT);
            e.mem.set_u32(block.addr(), VTABLE_TILE_MENU);
            e.mem.set_u32(block.addr() + 0x3c, 0);
            block
        }
        _ => e.call(RADIAL_TILE_CONSTRUCT, &args![block]).ptr(),
    }
}

// Translated from 00a00e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetValueQ` (Xbox PDB): the trait value with id `trait_id`, or
/// null; a linear search of the sorted trait array under the critical
/// section.
pub fn tile_get_value_q(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> Ptr {
    let mut result = Ptr::NULL;
    let mut index = 0;
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    let traits = this.at(Tile::xTraits);
    let id_at = |e: &Engine, index: u32| {
        let buffer = e.get(traits, BSSimpleArray::pBuffer);
        let value = e.mem.u32(buffer + index * 4);
        e.mem.i32(value)
    };
    while index < e.get(traits, BSSimpleArray::iSize) && id_at(e, index) < trait_id {
        index += 1;
    }
    if index < e.get(traits, BSSimpleArray::iSize) && id_at(e, index) == trait_id {
        let buffer = e.get(traits, BSSimpleArray::pBuffer);
        result = Ptr::new(e.mem.u32(buffer + index * 4));
    }
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    result
}

/// The search keys of `GetValue` and `GetOrCreateValue` are function-local
/// `static Tile::Value`s: on the first call the key is set to the unset
/// state (`eNone`, null tile, 0.0, no string, no actions) and its
/// destructor is registered with `atexit`.
fn init_search_key(e: &mut Engine, key: u32, guard: u32, destroy: u32) {
    let guard_word: u32 = e.global(guard);
    if guard_word & 1 == 0 {
        e.set_global(guard, guard_word | 1);
        e.set_global(key, 0x8000_0000u32);
        e.set_global(key + 4, 0u32);
        e.set_global(key + 8, 0.0f32);
        e.set_global(key + 0xc, 0u32);
        e.set_global(key + 0x10, 0u32);
        e.call(ATEXIT, &args![destroy]);
    }
}

/// `SortedFind` of the key at `key` in the tile's trait array: the index or
/// -1. The array function takes the key by address of a pointer to it.
fn find_value_index(e: &mut Engine, this: Ptr<Tile>, key: u32) -> i32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), key);
        e.call(
            VALUES_SORTED_FIND,
            &args![this.at(Tile::xTraits), slot, COMPARE_VALUES],
        )
        .i32()
    })
}

// Translated from 00a00f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetValue` (Xbox PDB): the trait value with id `trait_id`, or
/// null, by a sorted search of the trait array with the shared search key,
/// under the critical section.
pub fn tile_get_value(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> Ptr {
    let mut result = Ptr::NULL;
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    init_search_key(e, GET_VALUE_KEY, GET_VALUE_KEY_GUARD, GET_VALUE_KEY_DESTROY);
    e.set_global(GET_VALUE_KEY, trait_id);
    let index = find_value_index(e, this, GET_VALUE_KEY);
    if index != -1 {
        let buffer = e.get(this.at(Tile::xTraits), BSSimpleArray::pBuffer);
        result = Ptr::new(e.mem.u32(buffer + (index as u32) * 4));
    }
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    result
}

// Translated from 00a01000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetOrCreateValue` (Xbox PDB): the trait value with id
/// `trait_id`; a new one (0x14 bytes, unset, owned by this tile) is
/// allocated and inserted into the sorted array when there is none.
pub fn tile_get_or_create_value(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> Ptr {
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    init_search_key(
        e,
        GET_OR_CREATE_KEY,
        GET_OR_CREATE_KEY_GUARD,
        GET_OR_CREATE_KEY_DESTROY,
    );
    e.set_global(GET_OR_CREATE_KEY, trait_id);
    let index = find_value_index(e, this, GET_OR_CREATE_KEY);
    let result = if index != -1 {
        let buffer = e.get(this.at(Tile::xTraits), BSSimpleArray::pBuffer);
        Ptr::new(e.mem.u32(buffer + (index as u32) * 4))
    } else {
        let value: Ptr<TileValue> = allocate(e, 0x14).cast();
        e.set(value, TileValue::eIndex, TRAIT_NONE);
        e.set(value, TileValue::pParent, Ptr::NULL);
        e.set(value, TileValue::fValue, 0.0);
        e.set(value, TileValue::strValue, Ptr::NULL);
        e.set(value, TileValue::pActionListA, Ptr::NULL);
        e.set(value, TileValue::eIndex, trait_id);
        e.set(value, TileValue::pParent, this);
        // The insert takes the address of a variable holding the value.
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), value.addr());
            e.call(
                VALUES_SORTED_INSERT,
                &args![this.at(Tile::xTraits), slot, COMPARE_VALUES],
            );
            Ptr::new(e.mem.u32(slot.addr()))
        })
    };
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    result
}

// Translated from 00a01160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The comparison `GetValue`, `GetOrCreateValue` and the sorted array
/// functions use: both arguments point to a pointer to a `Tile::Value`;
/// -1, 0 or 1 by the signed trait ids.
pub fn fn_00a01160(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let first_id = e.mem.i32(e.mem.u32(first.addr()));
    let second_id = e.mem.i32(e.mem.u32(second.addr()));
    if first_id < second_id {
        -1
    } else if first_id > second_id {
        1
    } else {
        0
    }
}

// Translated from 00a011b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float of trait `trait_id` (`Tile::Value::fValue`), 0.0 when the tile
/// has no such trait.
pub fn fn_00a011b0(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> f32 {
    let value: Ptr<TileValue> = tile_get_value(e, this, trait_id).cast();
    if value.is_null() {
        0.0
    } else {
        e.get(value, TileValue::fValue)
    }
}

// Translated from 00a011f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetString` (Xbox PDB): the string of trait `trait_id`
/// (`Tile::Value::strValue`), null when the tile has no such trait.
pub fn tile_get_string(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> Ptr {
    let value: Ptr<TileValue> = tile_get_value(e, this, trait_id).cast();
    if value.is_null() {
        Ptr::NULL
    } else {
        e.get(value, TileValue::strValue)
    }
}

// Translated from 00a01230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::IsTrue` (Xbox PDB): the trait exists and its float is not 0.0.
pub fn tile_is_true(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> bool {
    let value: Ptr<TileValue> = tile_get_value(e, this, trait_id).cast();
    if value.is_null() {
        return false;
    }
    let float = e.get(value, TileValue::fValue);
    differs_from_zero(e, float)
}

// Translated from 00a01290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Poke` (Xbox PDB): sets trait `trait_id` to `value` and at once
/// back to 0.0, so that reactions to the change fire (both with the flag
/// set).
pub fn tile_poke(e: &mut Engine, this: Ptr<Tile>, trait_id: i32, value: f32) {
    fn_00a012d0(e, this, trait_id, value, true);
    fn_00a012d0(e, this, trait_id, 0.0, true);
}

// Translated from 00a012d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the float of trait `trait_id` (creating the trait when needed)
/// through `Tile::Value::SetFloat(value, flag)`, under `Tile::Lock`.
pub fn fn_00a012d0(e: &mut Engine, this: Ptr<Tile>, trait_id: i32, value: f32, flag: bool) {
    let trait_value = tile_get_or_create_value(e, this, trait_id);
    if !trait_value.is_null() {
        lock(e);
        e.call(VALUE_SET_FLOAT, &args![trait_value, value, flag as u32]);
        unlock(e);
    }
}

// Translated from 00a01350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the string of trait `trait_id` (creating the trait when needed)
/// through `Tile::Value::SetString(string, flag)`, under `Tile::Lock`.
pub fn fn_00a01350(e: &mut Engine, this: Ptr<Tile>, trait_id: i32, string: Ptr, flag: bool) {
    let trait_value = tile_get_or_create_value(e, this, trait_id);
    if !trait_value.is_null() {
        lock(e);
        e.call(VALUE_SET_STRING, &args![trait_value, string, flag as u32]);
        unlock(e);
    }
}

/// The sum of trait `trait_id` over the tile and those of its ancestors
/// whose locus trait is not 0.0 (`fn_00a013d0` for `x`, `fn_00a01440` for
/// `y`): each term is added in turn to a `float` running total.
fn sum_over_ancestors(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> f32 {
    let mut sum = fn_00a011b0(e, this, trait_id);
    let mut ancestor = e.get(this, Tile::pParent);
    while !ancestor.is_null() {
        let locus = fn_00a011b0(e, ancestor, TRAIT_LOCUS);
        if differs_from_zero(e, locus) {
            let term = fn_00a011b0(e, ancestor, trait_id);
            sum = (term as f64 + sum as f64) as f32;
        }
        ancestor = e.get(ancestor, Tile::pParent);
    }
    sum
}

// Translated from 00a013d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile's x position in the root's frame: its own `x` plus the `x` of
/// every ancestor whose locus trait is not 0.0 (see [`sum_over_ancestors`]).
pub fn fn_00a013d0(e: &mut Engine, this: Ptr<Tile>) -> f32 {
    sum_over_ancestors(e, this, TRAIT_X)
}

// Translated from 00a01440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile's y position in the root's frame (as [`fn_00a013d0`], with the
/// `y` trait).
pub fn fn_00a01440(e: &mut Engine, this: Ptr<Tile>) -> f32 {
    sum_over_ancestors(e, this, TRAIT_Y)
}

// Translated from 00a014b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tile's depth: its own depth trait plus the depth of every ancestor
/// whose locus trait is not 0.0 or whose parent is the menus root.
pub fn fn_00a014b0(e: &mut Engine, this: Ptr<Tile>) -> f32 {
    let mut sum = fn_00a011b0(e, this, TRAIT_DEPTH);
    let mut ancestor = e.get(this, Tile::pParent);
    while !ancestor.is_null() {
        let locus = fn_00a011b0(e, ancestor, TRAIT_LOCUS);
        let counts = if differs_from_zero(e, locus) {
            true
        } else {
            let parent = e.get(ancestor, Tile::pParent);
            let root: Ptr<Tile> = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr();
            parent == root
        };
        if counts {
            let term = fn_00a011b0(e, ancestor, TRAIT_DEPTH);
            sum = (term as f64 + sum as f64) as f32;
        }
        ancestor = e.get(ancestor, Tile::pParent);
    }
    sum
}

// Translated from 00a01530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetMaximumDepth` (Xbox PDB): `Menu::GetMaxDepth()`.
pub fn tile_get_maximum_depth(e: &mut Engine) -> f32 {
    e.call(MENU_GET_MAX_DEPTH, &args![]).f32()
}

// Translated from 00a01540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The greatest depth in the subtree of `tile` (`cdecl`): its own depth
/// trait, and for each child the child's subtree depth (plus this tile's
/// depth when its locus trait is not 0.0), whichever is larger.
pub fn fn_00a01540(e: &mut Engine, tile: Ptr<Tile>) -> f32 {
    let mut deepest = fn_00a011b0(e, tile, TRAIT_DEPTH);
    let mut node = e.get(tile.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let locus = fn_00a011b0(e, tile, TRAIT_LOCUS);
        let candidate = if differs_from_zero(e, locus) {
            let own_depth = fn_00a011b0(e, tile, TRAIT_DEPTH);
            let below = fn_00a01540(e, child);
            (below as f64 + own_depth as f64) as f32
        } else {
            fn_00a01540(e, child)
        };
        // `max`, but a candidate that is not a number replaces the maximum.
        deepest = if candidate < deepest {
            deepest
        } else {
            candidate
        };
    }
    deepest
}

// Translated from 00a01630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetTileFromNode` (Xbox PDB): the tile a scene-graph node belongs
/// to: the pointer stored in the node's extra data (key `011f34cc`, tile at
/// +0xC of the extra data), or else in its parent node's; null when there
/// is none. Under `Tile::Lock`; the extra data is held by a counted
/// reference while it is read.
pub fn tile_get_tile_from_node(e: &mut Engine, node: Ptr) -> Ptr<Tile> {
    lock(e);
    let mut result = 0;
    if !node.is_null() {
        let key = e.global::<u32>(TILE_EXTRA_DATA_KEY);
        let extra = e.call(NI_OBJECT_GET_EXTRA_DATA, &args![node, key]).u32();
        if extra != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![extra + 4]);
        }
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), extra);
            let parent_node = e.mem.u32(node.addr() + 0x18);
            if extra == 0 && parent_node != 0 {
                let key = e.global::<u32>(TILE_EXTRA_DATA_KEY);
                let found = e
                    .call(NI_OBJECT_GET_EXTRA_DATA, &args![parent_node, key])
                    .u32();
                e.call(NI_POINTER_ASSIGN, &args![held, found]);
            }
            let held_now = e.mem.u32(held.addr());
            if held_now != 0 {
                result = e.mem.u32(held_now + 0xc);
            }
            release_reference(e, held_now);
        });
    }
    unlock(e);
    Ptr::new(result)
}

// Translated from 00a01750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SeverExtraData` (Xbox PDB): clears the tile pointer in the
/// model's extra data (so the model no longer finds its tile), when the
/// model has any extra data (its 16-bit count at +0x14 is not 0). Under
/// `Tile::Lock`; the extra data is held by a counted reference meanwhile.
pub fn tile_sever_extra_data(e: &mut Engine, this: Ptr<Tile>) {
    lock(e);
    let model = e.get(this, Tile::spModel);
    if !model.is_null() && e.mem.u16(model.addr() + 0x14) != 0 {
        let key = e.global::<u32>(TILE_EXTRA_DATA_KEY);
        let extra = e.call(NI_OBJECT_GET_EXTRA_DATA, &args![model, key]).u32();
        if extra != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![extra + 4]);
            e.mem.set_u32(extra + 0xc, 0);
            release_reference(e, extra);
        }
    }
    unlock(e);
}

// Translated from 00a01860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TextToTrait` (Xbox PDB): the text table value of `name`, or
/// `0x80000000` (`eNone`) when it has none. Under `Tile::Lock`.
pub fn tile_text_to_trait(e: &mut Engine, name: Ptr) -> i32 {
    lock(e);
    let id = e.with_stack(4, |e, out| {
        e.mem.set_i32(out.addr(), TRAIT_NONE);
        e.call(TEXT_TABLE_FIND, &args![TEXT_TABLE, name, out]);
        e.mem.i32(out.addr())
    });
    unlock(e);
    id
}

// Translated from 00a018d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits `name` of the form `trait(argument)` into the part outside the
/// parentheses (looked up with [`tile_text_to_trait`], the result) and the
/// part inside, which is copied to `argument` (a buffer the caller
/// provides, 0x100 bytes at most). `0x80000000` when `name` or `argument`
/// is missing or `name` is empty. The scan stops at the first `)`, at a NUL
/// or after 254 characters.
pub fn fn_00a018d0(e: &mut Engine, name: Ptr, argument: Ptr) -> i32 {
    if name.is_null() || e.mem.i8(name.addr()) == 0 || argument.is_null() {
        return TRAIT_NONE;
    }
    e.with_stack(NAME_BUFFER_SIZE, |e, outside| {
        let mut inside_at: i32 = -1;
        e.mem.set_u8(argument.addr(), 0);
        e.mem.set_u8(outside.addr(), 0);
        let mut index: i32 = 0;
        while index < 0xff && e.mem.i8(name.addr() + index as u32) != 0 {
            e.mem.set_u8(argument.addr() + index as u32 + 1, 0);
            e.mem.set_u8(outside.addr() + index as u32 + 1, 0);
            let c = e.mem.u8(name.addr() + index as u32);
            if c == b'(' {
                inside_at = 0;
            } else if c == b')' {
                index = 0x100;
            } else if inside_at < 0 {
                e.mem.set_u8(outside.addr() + index as u32, c);
            } else {
                e.mem.set_u8(argument.addr() + inside_at as u32, c);
                inside_at += 1;
            }
            index += 1;
        }
        tile_text_to_trait(e, outside)
    })
}

// Translated from 00a01a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number at the end of a name: 0 when the name has no `_`, -1 when
/// the last `_` is the last character, else `atol` of what follows it.
pub fn fn_00a01a20(e: &mut Engine, name: Ptr) -> i32 {
    let underscore: Ptr = e.call(STRRCHR, &args![name, 0x5fu32]).ptr();
    if underscore.is_null() {
        0
    } else if e.mem.i8(underscore.addr() + 1) == 0 {
        -1
    } else {
        e.call(ATOL, &args![underscore.byte_add(1)]).i32()
    }
}

// Translated from 00a01a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name (key) of the text table entry whose value is `value`: scans the
/// table's buckets for the first non-empty one and steps through the
/// entries from there; null when no entry has the value.
pub fn fn_00a01a70(e: &mut Engine, value: i32) -> Ptr {
    let bucket_count: u32 = e.global(TEXT_TABLE_BUCKET_COUNT);
    let buckets: u32 = e.global(TEXT_TABLE_BUCKETS);
    let mut first = 0;
    for index in 0..bucket_count {
        let bucket = e.mem.u32(buckets + index * 4);
        if bucket != 0 {
            first = bucket;
            break;
        }
    }
    e.with_stack(12, |e, slots| {
        let (node, key, entry_value) = (slots.addr(), slots.addr() + 4, slots.addr() + 8);
        e.mem.set_u32(node, first);
        while e.mem.u32(node) != 0 {
            e.call(TEXT_TABLE_NEXT, &args![TEXT_TABLE, node, key, entry_value]);
            if e.mem.i32(entry_value) == value {
                return Ptr::new(e.mem.u32(key));
            }
        }
        Ptr::NULL
    })
}

/// What the XML reader does with one template element, `AddPair` on the
/// storage's current template (else the main one).
fn add_pair(
    e: &mut Engine,
    storage: Ptr<BuildStorage>,
    command: i32,
    text: u32,
    line: i32,
    flag: u32,
) {
    let template = storage_template(e, storage);
    e.call(
        TEMPLATE_ADD_PAIR,
        &args![template, command, text, line, flag],
    );
}

/// Prints an XML error (`"MENUS: (XML ERROR) %s -- in file '%s' on line
/// %i."`) and raises the reader's error flag.
fn xml_error(e: &mut Engine, path: Ptr, line: i32, message: u32) {
    e.call(PRINT_ERROR, &args![XML_ERROR_FORMAT, message, path, line]);
    e.set_global(XML_ERROR_FLAG, 1u8);
}

/// Appends `c` to the text buffer at `buffer` (index `length`), keeps it
/// terminated and complains once the length passes the buffer limit.
fn push_text_char(e: &mut Engine, buffer: u32, length: &mut u32, c: u8) {
    e.mem.set_u8(buffer + *length, c);
    *length += 1;
    e.mem.set_u8(buffer + *length, 0);
    if *length as i32 > READ_BUFFER_LIMIT {
        e.call(PRINT_ERROR, &args![MSG_BUFFER_TOO_SMALL]);
    }
}

/// The scan of [`tile_parse_file`]: a state machine over the characters of
/// the XML text at `data` (`length` bytes), which hands what it finds to
/// `add_pair`. `tag` is the tag-name buffer, `text` the attribute / text
/// buffer, `converted` the buffer a game-setting constant is expanded into.
/// False after an error.
///
/// States (the `Tile::enumXMLname` names): 0 outside a tag, 1 in a tag name,
/// 2 after the `/` of a close tag, 3 in tag data, 4 in a comment, 5 in an
/// attribute name, 6 in an attribute value.
fn parse_xml(
    e: &mut Engine,
    path: Ptr,
    (data, length): (u32, u32),
    storage: Ptr<BuildStorage>,
    (tag, text, converted): (u32, u32, u32),
) -> bool {
    let mut state = 0;
    let mut trait_id = 0;
    let mut index: u32 = 0;
    let mut used: u32 = 0;
    let mut line: i32 = 1;
    // A word ended at white space; the next word is a new one.
    let mut after_word = false;
    let mut in_quote = false;
    let mut closing = false;
    e.mem.set_u8(text, 0);
    while index < length {
        let c = e.mem.u8(data + index);
        let next = |e: &Engine, offset: u32| e.mem.u8(data + index.wrapping_add(offset));
        let ends_tag = |e: &Engine| {
            c == b'>' || (index.wrapping_add(1) < length && c == b'/' && next(e, 1) == b'>')
        };
        let starts_comment = |e: &Engine| {
            index.wrapping_add(3) < length
                && next(e, 1) == b'!'
                && next(e, 2) == b'-'
                && next(e, 3) == b'-'
        };
        match state {
            0 => {
                if c == b'<' {
                    if starts_comment(e) {
                        after_word = false;
                        state = 4;
                    } else {
                        state = 1;
                        e.mem.set_u8(tag, 0);
                        closing = false;
                        used = 0;
                    }
                }
            }
            1 => {
                let tag_empty = e.mem.i8(tag) == 0;
                if tag_empty && c == b'/' {
                    closing = true;
                } else if ends_tag(e) {
                    if !tag_empty {
                        add_pair(e, storage, closing as i32, tag, line, 0);
                    } else if !after_word {
                        xml_error(e, path, line, MSG_EMPTY_TAG_NAME);
                        return false;
                    }
                    used = 0;
                    state = if c == b'/' { 2 } else { 3 };
                    after_word = false;
                    line += 1;
                } else if c <= 0x20 {
                    if !tag_empty {
                        add_pair(e, storage, closing as i32, tag, line, 0);
                        used = 0;
                        after_word = true;
                        line += 1;
                    }
                } else if after_word {
                    index = index.wrapping_sub(1);
                    state = 5;
                    after_word = false;
                } else {
                    // No length check here, unlike the text buffers.
                    e.mem.set_u8(tag + used, c);
                    used += 1;
                    e.mem.set_u8(tag + used, 0);
                }
            }
            4 => {
                if index.wrapping_add(2) < length
                    && c == b'-'
                    && next(e, 1) == b'-'
                    && next(e, 2) == b'>'
                {
                    index += 2;
                    state = 3;
                    after_word = false;
                }
            }
            5 => {
                if ends_tag(e) {
                    xml_error(e, path, line, MSG_ATTRIBUTE_NO_VALUE);
                    return false;
                }
                if c == b'=' {
                    if e.mem.i8(text) == 0 {
                        xml_error(e, path, line, MSG_MISSING_ATTRIBUTE_NAME);
                        return false;
                    }
                    trait_id = tile_text_to_trait(e, Ptr::new(text));
                    e.mem.set_u8(text, 0);
                    used = 0;
                    state = 6;
                    after_word = false;
                } else if c <= 0x20 {
                    if e.mem.i8(text) != 0 {
                        trait_id = tile_text_to_trait(e, Ptr::new(text));
                        e.mem.set_u8(text, 0);
                        used = 0;
                        after_word = true;
                    }
                } else {
                    if after_word {
                        xml_error(e, path, line, MSG_UNEXPECTED_WORD);
                        return false;
                    }
                    push_text_char(e, text, &mut used, c);
                }
            }
            6 => {
                if c == b'"' {
                    in_quote = !in_quote;
                } else if in_quote {
                    push_text_char(e, text, &mut used, c);
                } else if ends_tag(e) {
                    if e.mem.i8(text) != 0 {
                        add_pair(e, storage, trait_id, text, line, 0);
                    } else if !after_word {
                        xml_error(e, path, line, MSG_MISSING_ATTRIBUTE_VALUE);
                        return false;
                    }
                    e.mem.set_u8(text, 0);
                    used = 0;
                    state = if c == b'/' { 2 } else { 3 };
                    after_word = false;
                } else if c <= 0x20 {
                    if e.mem.i8(text) != 0 {
                        add_pair(e, storage, trait_id, text, line, 0);
                        e.mem.set_u8(text, 0);
                        used = 0;
                        after_word = true;
                    }
                } else if after_word {
                    index = index.wrapping_sub(1);
                    state = 5;
                    after_word = false;
                } else {
                    push_text_char(e, text, &mut used, c);
                }
            }
            2 => {
                if c != b'>' {
                    xml_error(e, path, line, MSG_CLOSE_TAG_MARKER);
                    return false;
                }
                add_pair(e, storage, 1, tag, line, 0);
                state = 3;
                e.mem.set_u8(tag, 0);
            }
            _ => {
                // State 3: data between tags.
                if c == b'<' {
                    if starts_comment(e) {
                        state = 4;
                    } else {
                        if e.mem.i8(text) != 0 {
                            // Trim the white space at the end of the text.
                            let mut last = e.call(STRLEN, &args![text]).u32().wrapping_sub(1);
                            while e.mem.u8(text + last) <= 0x20 {
                                last = last.wrapping_sub(1);
                            }
                            e.mem.set_u8(text + last + 1, 0);
                            if e.mem.i8(text) == b'&' as i8 {
                                if e.mem.i8(text + 1) == b'-' as i8 {
                                    // "&-name;": a game-setting constant.
                                    let known = e
                                        .call(
                                            INTERFACE_TEST_CONSTANT_FOR_GAME_SETTINGS,
                                            &args![text, converted],
                                        )
                                        .bool();
                                    if known {
                                        add_pair(e, storage, TAG_VALUE, converted, line, 0);
                                    }
                                } else {
                                    add_pair(e, storage, TAG_VALUE, text, line, 0);
                                }
                            } else if e.mem.i8(text) != 0 {
                                add_pair(e, storage, TAG_VALUE, text, line, 1);
                            }
                        }
                        e.mem.set_u8(text, 0);
                        used = 0;
                        closing = false;
                        state = 1;
                        e.mem.set_u8(tag, 0);
                    }
                    after_word = false;
                } else {
                    if index.wrapping_add(1) < length && c == b'/' && next(e, 1) == b'>' {
                        xml_error(e, path, line, MSG_UNBALANCED_CLOSE_PAIR);
                        return false;
                    }
                    if c == b'>' {
                        xml_error(e, path, line, MSG_UNBALANCED_END_OF_TAG);
                        return false;
                    }
                    if c > 0x20 || after_word {
                        push_text_char(e, text, &mut used, c);
                        after_word = true;
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00a01b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ReadFile` (Xbox PDB): reads the menu file `path` into a tile tree
/// under `this`: parses it, builds and names the tiles, connects the
/// traits, registers the sub-templates with the tile's menu, frees the
/// parse storage and updates the new tree (under the interface lock).
/// Returns the first new tile, or null after an error (the error flag
/// `011f32c8` is set by the parser). The thread's current-line word is 13
/// for the duration.
pub fn tile_read_file(e: &mut Engine, this: Ptr<Tile>, path: Ptr) -> Ptr<Tile> {
    let line_word = e.tls() + TLS_CURRENT_LINE;
    let saved_line = e.mem.u32(line_word);
    e.mem.set_u32(line_word, 0xd);
    e.set_global(XML_ERROR_FLAG, 0u8);

    let storage = tile_parse_file(e, path, Ptr::NULL, 0);
    let failed = |e: &mut Engine| {
        let failed = e.global::<u8>(XML_ERROR_FLAG) != 0;
        if failed {
            e.mem.set_u32(line_word, saved_line);
        }
        failed
    };
    if failed(e) {
        return Ptr::NULL;
    }
    let template = storage_template(e, storage);
    let tile = tile_build_and_name_tree(e, this, template);
    if failed(e) {
        return Ptr::NULL;
    }
    let template = storage_template(e, storage);
    tile_connect_traits_to_tree(e, tile, template);
    if failed(e) {
        return Ptr::NULL;
    }
    let menu = tile_get_menu(e, tile);
    if !menu.is_null() {
        let mut node = storage.addr() + 4;
        while node != 0 && e.mem.u32(node) != 0 {
            let sub_template = e.mem.u32(node);
            e.call(MENU_ADD_TEMPLATE, &args![menu, sub_template]);
            // `bDeleteTemplates` of the menu is set; the storage's own flag
            // (a local that is never set in this function) is cleared.
            e.mem.set_u8(menu.addr() + 0x1d, 1);
            e.set(storage, BuildStorage::bDeleteTemplates, false);
            if failed(e) {
                return Ptr::NULL;
            }
            node = e.mem.u32(node + 4);
        }
    }
    // The parse storage is deleted (the "keep" flag is never set).
    if !storage.is_null() {
        e.call(BUILD_STORAGE_DESTROY, &args![storage]);
        deallocate(e, storage.addr());
    }
    let lock_owner: u32 = e.global(INTERFACE_LOCK_OWNER);
    enter_critical_section(e, lock_owner + 0x80);
    if !tile.is_null() {
        e.call(TILE_UPDATE_CHILDREN, &args![tile, 0u32]);
    }
    let lock_owner: u32 = e.global(INTERFACE_LOCK_OWNER);
    leave_critical_section(e, lock_owner + 0x80);
    e.set_global(XML_ERROR_FLAG, 0u8);
    e.mem.set_u32(line_word, saved_line);
    tile
}

// Translated from 00a01e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ParseFile` (Xbox PDB, `cdecl`): parses menu XML into a new
/// `BuildStorage` and returns it. With no `buffer` the text is the file
/// `path` read by `Menu::GetXML`, after its `<include>` tags are expanded
/// ([`tile_insert_included_files`]); with a `buffer` it is `length` bytes of
/// text given by the caller. The scanner is [`parse_xml`]. Returns null after
/// an XML error (the error flag `011f32c8` is set and the storage and the
/// file text are not freed); the file text is freed after a complete scan.
pub fn tile_parse_file(e: &mut Engine, path: Ptr, buffer: Ptr, length: u32) -> Ptr<BuildStorage> {
    let block = allocate(e, 0x14);
    let storage: Ptr<BuildStorage> = if block.is_null() {
        Ptr::NULL
    } else {
        e.call(BUILD_STORAGE_CONSTRUCT, &args![block]).ptr()
    };
    let mut xml: Ptr<XmlStorage> = Ptr::NULL;
    let (data, size) = if buffer.is_null() {
        xml = e.call(MENU_GET_XML, &args![path, 0u32]).ptr();
        if xml.is_null() {
            (0, 0)
        } else {
            tile_insert_included_files(e, xml);
            (
                e.get(xml, XmlStorage::pXMLData).addr(),
                e.get(xml, XmlStorage::iFileSize),
            )
        }
    } else {
        (buffer.addr(), length)
    };
    // The exe's frame: the exception record (three pushes) and `_chkstk(0x2108)` under `EBP`,
    // 0x2114 bytes; the tag buffer at `EBP - 0x94`, the file text at `EBP - 0x10bc` and the
    // converted text at `EBP - 0x20cc` (00a02142, 00a024c4, 00a02a62).
    let complete = e.with_stack(FRAME_BELOW_EBP, |e, frame| {
        let ebp = frame.addr() + FRAME_BELOW_EBP;
        let (tag, text, converted) = (ebp - 0x94, ebp - 0x10bc, ebp - 0x20cc);
        parse_xml(e, path, (data, size), storage, (tag, text, converted))
    });
    if !complete {
        return Ptr::NULL;
    }
    if !xml.is_null() {
        e.call(XML_STORAGE_SCALAR_DELETING_DESTRUCTOR, &args![xml, 1u32]);
    }
    storage
}

// Translated from 00a02d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::InsertIncludedFiles` (Xbox PDB, `cdecl`): replaces every
/// `<include src="name"/>` tag in the text of `xml` by the text of the
/// prefab file `Data\Menus\Prefabs\name` (read with `Menu::GetXML`, which
/// is freed afterwards), then repeats on the result in case the prefabs
/// have includes of their own. True when there was at least one include.
///
/// The pieces are collected in a `BSStringT`; the original text is freed
/// and replaced by a copy of the result. `strtok_s` writes into the old
/// text, as in the game.
pub fn tile_insert_included_files(e: &mut Engine, xml: Ptr<XmlStorage>) -> bool {
    let old_text = e.get(xml, XmlStorage::pXMLData).addr();
    let mut include = e.call(STRSTR, &args![old_text, INCLUDE_TAG]).u32();
    if include == 0 {
        return false;
    }
    e.with_stack(8, |e, result| {
        let size = e.get(xml, XmlStorage::iFileSize) + 1;
        let buffer = allocate(e, size).addr();
        e.call(MEMSET, &args![buffer, 0u32, size]);
        let head = include - old_text;
        e.call(STRNCPY_S, &args![buffer, head + 1, old_text, head]);
        e.call(STRING_SET, &args![result, buffer, 0x4000u32]);
        let mut last = false;
        while !last {
            e.with_stack(8, |e, path_string| {
                let tag_length = e.call(STRCHR, &args![include, 0x3eu32]).u32() - include + 1;
                e.call(MEMSET, &args![buffer, 0u32, size]);
                let name = e.with_stack(4, |e, context| {
                    e.call(STRTOK_S, &args![include, QUOTE_DELIMITER, context]);
                    e.call(STRTOK_S, &args![0u32, QUOTE_DELIMITER, context])
                        .u32()
                });
                e.call(STRING_FORMAT, &args![path_string, PREFAB_PATH_FORMAT, name]);
                let path = e.mem.u32(path_string.addr());
                let prefab: Ptr<XmlStorage> = e.call(MENU_GET_XML, &args![path, 0u32]).ptr();
                let prefab_text = e.get(prefab, XmlStorage::pXMLData);
                e.call(STRING_APPEND, &args![result, prefab_text]);
                if !prefab.is_null() {
                    e.call(XML_STORAGE_DESTROY, &args![prefab]);
                    deallocate(e, prefab.addr());
                }
                let after_tag = include + tag_length;
                let next = e.call(STRSTR, &args![after_tag, INCLUDE_TAG]).u32();
                e.call(MEMSET, &args![buffer, 0u32, size]);
                if next != 0 {
                    let between = next - after_tag;
                    e.call(STRNCPY_S, &args![buffer, between + 1, after_tag, between]);
                    include = next;
                } else {
                    e.call(STRCPY_S, &args![buffer, size, after_tag]);
                    last = true;
                }
                e.call(STRING_APPEND, &args![result, buffer]);
                e.call(STRING_SET, &args![path_string, 0u32, 0u32]);
            });
        }
        deallocate(e, old_text);
        // The result's length, from its length field or, when that is
        // 0xffff (unknown), by counting.
        let result_length = |e: &mut Engine| {
            if e.mem.u16(result.addr() + 4) == 0xffff {
                let text = e.mem.u32(result.addr());
                e.call(STRLEN, &args![text]).u32()
            } else {
                e.mem.u16(result.addr() + 4) as u32
            }
        };
        let length = result_length(e);
        let new_text = allocate(e, length + 1);
        e.set(xml, XmlStorage::pXMLData, new_text);
        let length = result_length(e);
        let text = e.mem.u32(result.addr());
        e.call(STRCPY_S, &args![new_text, length + 1, text]);
        let length = result_length(e);
        e.set(xml, XmlStorage::iFileSize, length);
        deallocate(e, buffer);
        tile_insert_included_files(e, xml);
        e.call(STRING_SET, &args![result, 0u32, 0u32]);
    });
    true
}

// Translated from 00a031b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::BuildAndNameTree` (Xbox PDB): walks the items of `template`,
/// creating a tile for each tile-start item (`BuildTile` with the item's
/// type number), initializing it as a child of the current parent with the
/// item's name (virtual slot +4, `Init`) and remembering it in the item;
/// a tile-end item moves back to the parent. A hot-rect tile (type 906) is
/// given its fixed look: id -1, target 1.0, brightness -1.0, red, green and
/// blue 255.0, alpha 0.0, texture `solid.dds` and the default atlas, and
/// queued for a texture update. Returns the first tile built, or null
/// (after printing a message) when a tile cannot be created.
pub fn tile_build_and_name_tree(
    e: &mut Engine,
    this: Ptr<Tile>,
    template: Ptr<TileTemplate>,
) -> Ptr<Tile> {
    let mut first: Ptr<Tile> = Ptr::NULL;
    let mut parent = this;
    let mut node = e.get(template.at(TileTemplate::xList), NiTPointerList::m_pkHead);
    while node != 0 {
        let item: Ptr<TileTemplateItem> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        match e.get(item, TileTemplateItem::iCmd) {
            TI_TILE_START => {
                let type_number = e.get(item, TileTemplateItem::fVal);
                let kind = float_to_int(e, type_number);
                let tile: Ptr<Tile> = tile_build_tile(e, kind as u32).cast();
                if tile.is_null() {
                    e.call(PRINT_ERROR, &args![MSG_UNABLE_TO_CREATE_TILE]);
                    return Ptr::NULL;
                }
                let name = e.mem.u32(item.addr() + 8);
                e.vcall(tile.addr(), 4, &args![parent, name, 0u32]);
                let is_hot_rect = type_number as f64 == e.global::<f64>(HOT_RECT_TYPE_VALUE);
                if is_hot_rect {
                    let minus_one: f32 = e.global(MINUS_ONE);
                    let full: f32 = e.global(TWO_FIFTY_FIVE);
                    fn_00a012d0(e, tile, TRAIT_ID, minus_one, true);
                    fn_00a012d0(e, tile, TRAIT_TARGET, 1.0, true);
                    fn_00a012d0(e, tile, TRAIT_BRIGHTNESS, minus_one, true);
                    fn_00a012d0(e, tile, TRAIT_RED, full, true);
                    fn_00a012d0(e, tile, TRAIT_GREEN, full, true);
                    fn_00a012d0(e, tile, TRAIT_BLUE, full, true);
                    fn_00a012d0(e, tile, TRAIT_ALPHA, 0.0, true);
                    fn_00a01350(e, tile, TRAIT_FILENAME, Ptr::new(SOLID_TEXTURE_NAME), true);
                    let atlas: u32 = e.global(DEFAULT_ATLAS_NAME);
                    fn_00a01350(e, tile, TRAIT_TEX_ATLAS, Ptr::new(atlas), true);
                    e.call(TILE_ADD_NEEDS_UPDATE, &args![tile, UPDATE_TEXTURE]);
                }
                if first.is_null() {
                    first = tile;
                }
                e.set(item, TileTemplateItem::u, tile.addr());
                parent = tile;
            }
            TI_TILE_END => {
                parent = e.get(parent, Tile::pParent);
            }
            _ => {}
        }
    }
    first
}

// Translated from 00a033b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ConnectTraitsToTree` (Xbox PDB): second pass over the items of
/// `template`, with the tiles `BuildAndNameTree` made:
///
/// * tile start: that tile becomes current and is queued for an update,
///   with `FLAG_TILE_LOADING` set;
/// * tile end: clears `FLAG_TILE_LOADING` and calculates every value of the
///   tile except its class, under the critical section, then goes back to
///   the parent;
/// * trait start / end: the current trait (the item's number / none);
/// * simple trait: a float, or a string for string traits or a value with
///   text (the `eName` tag becomes the tile's name; a file name of an image
///   tile also queues a texture update), set on the current tile;
/// * simple action, action link, action start / end (parentheses): an
///   action added to the current trait's value.
///
/// Items out of place print a message. The first parameter is `this` of
/// the call, which is not read.
pub fn tile_connect_traits_to_tree(
    e: &mut Engine,
    _unused_0: Ptr<Tile>,
    template: Ptr<TileTemplate>,
) {
    let mut first: Ptr<Tile> = Ptr::NULL;
    let mut current: Ptr<Tile> = Ptr::NULL;
    let mut trait_id: i32 = 0;
    let mut node = e.get(template.at(TileTemplate::xList), NiTPointerList::m_pkHead);
    while node != 0 {
        let item: Ptr<TileTemplateItem> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let value = e.get(item, TileTemplateItem::fVal);
        let argument = e.get(item, TileTemplateItem::u);
        let text = e.mem.u32(item.addr() + 8);
        match e.get(item, TileTemplateItem::iCmd) {
            TI_TILE_START => {
                current = Ptr::new(argument);
                if first.is_null() {
                    first = current;
                }
                e.call(TILE_ADD_NEEDS_UPDATE, &args![current, FLAG_TILE_LOADING]);
            }
            TI_TILE_END => {
                let flags = e.get(current, Tile::uiFlags);
                if flags & FLAG_TILE_LOADING != 0 {
                    e.set(current, Tile::uiFlags, flags ^ FLAG_TILE_LOADING);
                }
                enter_critical_section(e, TILE_CRITICAL_SECTION);
                let traits = current.at(Tile::xTraits);
                let mut index = 0;
                while index < e.get(traits, BSSimpleArray::iSize) {
                    let buffer = e.get(traits, BSSimpleArray::pBuffer);
                    let trait_value = e.mem.u32(buffer + index * 4);
                    if e.mem.i32(trait_value) != TRAIT_CLASS {
                        e.call(VALUE_CALCULATE_VALUE, &args![trait_value, 1u32]);
                    }
                    index += 1;
                }
                leave_critical_section(e, TILE_CRITICAL_SECTION);
                current = e.get(current, Tile::pParent);
            }
            TI_SIMPLE_TRAIT => {
                if current.is_null() {
                    e.call(PRINT_ERROR, &args![MSG_TRAIT_OUTSIDE_TILE]);
                    continue;
                }
                let item_trait = argument as i32;
                let mut is_string = false;
                if value as f64 == e.global::<f64>(NO_TRAIT_VALUE) {
                    // No number was given: a value with text is a string.
                    let length = if e.mem.u16(item.addr() + 0xc) == 0xffff {
                        e.call(STRLEN, &args![text]).u32()
                    } else {
                        e.mem.u16(item.addr() + 0xc) as u32
                    };
                    is_string = length != 0;
                }
                if !is_string && (item_trait == TRAIT_STRING || item_trait == TRAIT_FILENAME) {
                    is_string = true;
                }
                if !is_string {
                    fn_00a012d0(e, current, item_trait, value, true);
                } else if item_trait == TAG_NAME {
                    e.call(
                        STRING_ASSIGN,
                        &args![current.at(Tile::xName), item.at(TileTemplateItem::xStr)],
                    );
                } else if item_trait == TRAIT_FILENAME && tile_type(e, current) == TYPE_IMAGE {
                    fn_00a01350(e, current, item_trait, Ptr::new(text), true);
                    e.call(TILE_ADD_NEEDS_UPDATE, &args![current, UPDATE_TEXTURE]);
                } else {
                    fn_00a01350(e, current, item_trait, Ptr::new(text), true);
                }
            }
            TI_TRAIT_START => {
                trait_id = float_to_int(e, value);
            }
            TI_TRAIT_END => {
                trait_id = 0;
            }
            TI_SIMPLE_ACTION => {
                if trait_id == 0 {
                    e.call(PRINT_ERROR, &args![MSG_ACTION_OUTSIDE_TRAIT]);
                } else {
                    let trait_value = tile_get_or_create_value(e, current, trait_id);
                    e.call(VALUE_ADD_ACTION, &args![trait_value, argument, value]);
                }
            }
            TI_TRAIT_LINK => {
                if trait_id == 0 {
                    e.call(PRINT_ERROR, &args![MSG_ACTION_LINK_OUTSIDE_TRAIT]);
                } else {
                    let link = e.call(TILE_FIND_BY_PATH, &args![current, text]).u32();
                    if link != 0 {
                        let number = float_to_int_rounded(value);
                        let trait_value = tile_get_or_create_value(e, current, trait_id);
                        e.call(
                            VALUE_ADD_ACTION_OV2,
                            &args![trait_value, argument, link, number],
                        );
                    }
                }
            }
            TI_ACTION_START | TI_ACTION_END => {
                let (action, message) = if e.get(item, TileTemplateItem::iCmd) == TI_ACTION_START {
                    (ACTION_LEFT_PAREN, MSG_ACTION_BEGUN_OUTSIDE_TRAIT)
                } else {
                    (ACTION_RIGHT_PAREN, MSG_ACTION_ENDED_OUTSIDE_TRAIT)
                };
                if trait_id == 0 {
                    e.call(PRINT_ERROR, &args![message]);
                } else {
                    let number = float_to_int_rounded(value) as f32;
                    let trait_value = tile_get_or_create_value(e, current, trait_id);
                    e.call(VALUE_ADD_ACTION, &args![trait_value, action, number]);
                }
            }
            _ => {}
        }
    }
}

// Translated from 00a037e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::FinalPostParse` (Xbox PDB): after trait `trait_id` of the tile
/// changed to `value`, queues the updates it needs and returns `this`, or
/// null when the trait needs none:
///
/// * x, y or depth: position update (and, for depth, the menu's depth is
///   raised to the summed depth of the tile and its ancestors with a locus
///   when that is more and the tile is not the menu tile); a tile with a
///   clip window also gets a scissor-window update;
/// * a text tile's wrap, justify, font, page and similar traits: create;
/// * width, height, crop of an image: geometry (and scissor window with a
///   clip window);
/// * clip window of an image or rect: scissor window; clips of an image or
///   text: scissor, and the children's clips are set to the tile's;
/// * visible: visibility; colour traits of an image or text: colour;
/// * file name of an image with a file: texture; of a 3D tile with a file:
///   NIF file; locus: locus; id: the menu is told the new id (virtual slot
///   +4); rotation: position.
///
/// The third stack word of the call is not read.
pub fn tile_final_post_parse(
    e: &mut Engine,
    this: Ptr<Tile>,
    trait_id: i32,
    value: f32,
    _unused_3: u32,
) -> Ptr<Tile> {
    let queue = |e: &mut Engine, flags: u32| {
        e.call(TILE_ADD_NEEDS_UPDATE, &args![this, flags]);
    };
    if matches!(trait_id, TRAIT_X | TRAIT_Y | TRAIT_DEPTH) {
        if trait_id == TRAIT_DEPTH {
            let mut total = value;
            let mut ancestor = this;
            loop {
                let parent = e.get(ancestor, Tile::pParent);
                if parent.is_null() || e.get(parent, Tile::pParent).is_null() {
                    break;
                }
                let locus = fn_00a011b0(e, ancestor, TRAIT_LOCUS);
                if differs_from_zero(e, locus) {
                    let own = fn_00a011b0(e, ancestor, TRAIT_DEPTH);
                    total = (own as f64 + total as f64) as f32;
                }
                ancestor = parent;
            }
            let menu = tile_get_menu(e, this);
            let menu_tile = tile_get_menu_tile(e, this);
            if !menu.is_null() && !menu_tile.is_null() && menu_tile != this {
                let menu_depth = e.mem.i32(menu.addr() + 0x18);
                if total as f64 > menu_depth as f64 {
                    let depth = float_to_int(e, total);
                    e.mem.set_i32(menu.addr() + 0x18, depth);
                }
            }
        }
        let clip_window = fn_00a011b0(e, this, TRAIT_CLIP_WINDOW);
        if differs_from_zero(e, clip_window) {
            queue(e, UPDATE_SCISSOR_WINDOW);
        }
        queue(e, UPDATE_POSITION);
        return this;
    }

    let kind = tile_type(e, this);
    if kind == TYPE_TEXT
        && matches!(
            trait_id,
            TRAIT_WRAP_WIDTH
                | TRAIT_WRAP_LIMIT
                | TRAIT_WRAP_LINES
                | TRAIT_LINE_GAP
                | TRAIT_PAGE_NUM
                | TRAIT_IS_HTML
                | TRAIT_FONT
                | TRAIT_JUSTIFY
        )
    {
        queue(e, UPDATE_CREATE);
        return this;
    }
    if matches!(
        trait_id,
        TRAIT_WIDTH | TRAIT_HEIGHT | TRAIT_CROP_X | TRAIT_CROP_Y
    ) && tile_type(e, this) == TYPE_IMAGE
    {
        let clip_window = fn_00a011b0(e, this, TRAIT_CLIP_WINDOW);
        if differs_from_zero(e, clip_window) {
            queue(e, UPDATE_SCISSOR_WINDOW);
        }
        queue(e, UPDATE_GEOMETRY);
        return this;
    }
    if trait_id == TRAIT_CLIP_WINDOW
        && (tile_type(e, this) == TYPE_IMAGE || tile_type(e, this) == TYPE_RECT)
    {
        queue(e, UPDATE_SCISSOR_WINDOW);
        return this;
    }
    if trait_id == TRAIT_CLIPS
        && (tile_type(e, this) == TYPE_IMAGE || tile_type(e, this) == TYPE_TEXT)
    {
        queue(e, UPDATE_SCISSOR);
        let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
        while node != 0 {
            let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
            node = e.mem.u32(node);
            let clips = fn_00a011b0(e, this, TRAIT_CLIPS);
            fn_00a012d0(e, child, TRAIT_CLIPS, clips, true);
        }
        return this;
    }
    if trait_id == TRAIT_VISIBLE {
        queue(e, UPDATE_VISIBILITY);
        return this;
    }
    if matches!(
        trait_id,
        TRAIT_ALPHA | TRAIT_RED | TRAIT_GREEN | TRAIT_BLUE | TRAIT_SYSTEM_COLOR | TRAIT_BRIGHTNESS
    ) {
        if tile_type(e, this) == TYPE_IMAGE || tile_type(e, this) == TYPE_TEXT {
            queue(e, UPDATE_COLOR);
        }
        return this;
    }
    if trait_id == TRAIT_FILENAME
        && tile_type(e, this) == TYPE_IMAGE
        && !tile_get_string(e, this, TRAIT_FILENAME).is_null()
    {
        queue(e, UPDATE_TEXTURE);
        return this;
    }
    if trait_id == TRAIT_FILENAME
        && tile_type(e, this) == TYPE_3D
        && !tile_get_string(e, this, TRAIT_FILENAME).is_null()
    {
        queue(e, UPDATE_NIF_FILE);
        return this;
    }
    if trait_id == TRAIT_LOCUS {
        queue(e, UPDATE_LOCUS);
        return this;
    }
    if trait_id == TRAIT_ID {
        let menu = tile_get_menu(e, this);
        if !menu.is_null() {
            let id = float_to_int(e, value);
            e.vcall(menu.addr(), 4, &args![id, this]);
        }
    }
    if matches!(
        trait_id,
        TRAIT_ROTATE_ANGLE | TRAIT_ROTATE_AXIS_X | TRAIT_ROTATE_AXIS_Y
    ) {
        queue(e, UPDATE_POSITION);
        return this;
    }
    Ptr::NULL
}

// Translated from 00a03c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetRenderedMenu` (Xbox PDB): `bRenderedMenu` of the tile's menu.
pub fn tile_get_rendered_menu(e: &mut Engine, this: Ptr<Tile>) -> bool {
    let menu = tile_get_menu(e, this);
    e.mem.u8(menu.addr() + 0x1c) != 0
}

// Translated from 00a03c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetMenu` (Xbox PDB): the `Menu` object of the tile's menu tile
/// (the word at +0x3C of a `TileMenu`), or null when the menu tile is not
/// a menu-type tile. Under `Tile::Lock`.
pub fn tile_get_menu(e: &mut Engine, this: Ptr<Tile>) -> Ptr {
    lock(e);
    let menu_tile = tile_get_menu_tile(e, this);
    let menu = if !menu_tile.is_null() && tile_type(e, menu_tile) == TYPE_MENU {
        Ptr::new(e.mem.u32(menu_tile.addr() + 0x3c))
    } else {
        Ptr::NULL
    };
    unlock(e);
    menu
}

// Translated from 00a03d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetMenuTile` (Xbox PDB): the ancestor (or the tile itself) that
/// is a child of the root: walks up while the parent has a parent. Under
/// `Tile::Lock`.
pub fn tile_get_menu_tile(e: &mut Engine, this: Ptr<Tile>) -> Ptr<Tile> {
    lock(e);
    let mut tile = this;
    while !tile.is_null() {
        let parent = e.get(tile, Tile::pParent);
        if parent.is_null() || e.get(parent, Tile::pParent).is_null() {
            break;
        }
        tile = parent;
    }
    unlock(e);
    tile
}

// Translated from 00a03da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetChildByName` (Xbox PDB): the first tile named `name` among the
/// descendants of the tile, depth first (each child is compared before its
/// own children are searched); null when there is none or `name` is null.
/// Under `Tile::Lock`.
pub fn tile_get_child_by_name(e: &mut Engine, this: Ptr<Tile>, name: Ptr) -> Ptr<Tile> {
    lock(e);
    let mut result = Ptr::NULL;
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 && !name.is_null() {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let child_name = e.mem.u32(child.addr() + 0x20);
        if child_name != 0 && e.call(STRCMP, &args![child_name, name]).i32() == 0 {
            result = child;
            break;
        }
        let found = tile_get_child_by_name(e, child, name);
        if !found.is_null() {
            result = found;
            break;
        }
    }
    unlock(e);
    result
}

// Translated from 00a03eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetChildByID` (Xbox PDB): the first direct child whose `id` trait
/// (`eID`) equals `id`, or null. Under `Tile::Lock`.
pub fn tile_get_child_by_id(e: &mut Engine, this: Ptr<Tile>, id: i32) -> Ptr<Tile> {
    lock(e);
    let mut result = Ptr::NULL;
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let child_id = fn_00a011b0(e, child, TRAIT_ID);
        if id as f64 == child_id as f64 {
            result = child;
            break;
        }
    }
    unlock(e);
    result
}

// Translated from 00a03f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetFirstRefCopy` (Xbox PDB): follows the first reference action
/// (`VA_REF`, 2023) of trait `trait_id`: answers the tile of the referenced
/// trait and writes that trait's id to `*out`. When the tile has no such
/// trait the search continues in its parent. The answer is 0 when no
/// reference action is found. Under `Tile::Lock`.
pub fn tile_get_first_ref_copy(e: &mut Engine, this: Ptr<Tile>, trait_id: i32, out: Ptr) -> u32 {
    lock(e);
    let value: Ptr<TileValue> = tile_get_value(e, this, trait_id).cast();
    let mut answer = 0;
    if value.is_null() {
        let parent = e.get(this, Tile::pParent);
        if !parent.is_null() {
            answer = tile_get_first_ref_copy(e, parent, trait_id, out);
            unlock(e);
            return answer;
        }
    } else {
        let mut action = e.get(value, TileValue::pActionListA).addr();
        let mut found = false;
        while action != 0 && !found {
            // `Tile::Action::eActionType` is at +4; `QRefValue` is slot +4.
            if e.mem.i32(action + 4) == VA_REF && e.vcall(action, 4, &[]).u32() != 0 {
                let referenced = e.vcall(action, 4, &[]).u32();
                answer = e.mem.u32(referenced + 4);
                let referenced = e.vcall(action, 4, &[]).u32();
                let referenced_trait = e.mem.u32(referenced);
                e.mem.set_u32(out.addr(), referenced_trait);
                found = true;
            }
            action = e.mem.u32(action + 8);
        }
    }
    unlock(e);
    answer
}

// Translated from 00a040a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::IsVisible` (Xbox PDB): false when the tile's model or any of its
/// ancestor nodes has the `NiAVObject` culled flag (bit 0 of +0x30) set.
pub fn tile_is_visible(e: &mut Engine, this: Ptr<Tile>) -> bool {
    let mut hidden = false;
    let mut node = e.get(this, Tile::spModel).addr();
    while !hidden && node != 0 {
        hidden |= e.mem.u32(node + 0x30) & 1 != 0;
        node = e.mem.u32(node + 0x18);
    }
    !hidden
}

// Translated from 00a04100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Walks up from the tile's parent while the `id` trait of
/// the tile is the double at `0106ebc8` (110.0) and answers false when the
/// walk reaches the root. A tile with another id makes the game loop
/// forever (the loop only advances on a match); this translation stops
/// there with a panic instead of hanging.
pub fn fn_00a04100(e: &mut Engine, this: Ptr<Tile>) -> bool {
    let mut tile = e.get(this, Tile::pParent);
    while !tile.is_null() {
        let id = fn_00a011b0(e, tile, TRAIT_ID);
        if id as f64 != e.global::<f64>(ID_VALUE_110) {
            panic!("fn_00a04100: the game loops forever on a tile whose id is not 110");
        }
        tile = e.get(tile, Tile::pParent);
    }
    false
}

// Translated from 00a04150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::DeleteChildren` (Xbox PDB): under `Tile::Lock`, deletes every
/// child (virtual scalar deleting destructor, slot 0, with the delete flag)
/// and then empties the children list.
pub fn tile_delete_children(e: &mut Engine, this: Ptr<Tile>) {
    lock(e);
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if child != 0 {
            e.vcall(child, 0, &args![1u32]);
        }
    }
    e.call(LIST_REMOVE_ALL, &args![this.addr() + 4]);
    unlock(e);
}

// Translated from 00a04200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::UpdateAll` (Xbox PDB), `cdecl(flag)`: with the update critical
/// section held and the current-line word of the TLS block set to 13,
/// takes dirty tiles one at a time ([`tile_get_next_dirty_tile`]) and
/// updates those that have update bits and are neither released, deleting
/// nor hibernated; a tile that changed adds itself to the interface
/// manager's update list, and a menu's root-level tile with the stacking
/// type 102.0 or 103.0 gets the stacking value 6000.0 or 6001.0. When a
/// tile became visible again (`bNeedsCheckHibernate`) the hibernating tiles
/// are checked and the whole pass runs again. Otherwise, with `flag`, the
/// interface manager is asked to do its end-of-update work, and when the
/// interface wants textures released the unused ones are purged.
pub fn tile_update_all(e: &mut Engine, flag: bool) {
    let line_word = e.tls() + TLS_CURRENT_LINE;
    let saved_line = e.mem.u32(line_word);
    e.mem.set_u32(line_word, 0xd);
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    e.set_global(NEEDS_CHECK_HIBERNATE, 0u8);
    while e.mem.u32(DIRTY_TILES_LIST + 8) != 0 {
        let tile = tile_get_next_dirty_tile(e, true);
        if tile.is_null() {
            continue;
        }
        let flags = e.get(tile, Tile::uiFlags);
        if flags & (FLAG_RELEASED | FLAG_MENU_DELETING | FLAG_HIBERNATED) != 0
            || flags & UPDATE_MASK == 0
        {
            continue;
        }
        let changed = tile_update_tile(e, tile, true);
        let needs = e.mem.u8(tile.addr() + 0x34) | changed as u8;
        e.mem.set_u8(tile.addr() + 0x34, needs);
        if changed {
            let manager = e.global::<u32>(INTERFACE_MANAGER);
            e.call(ADD_TILE_TO_UPDATE_LIST, &args![manager, tile]);
        }
        let parent = e.get(tile, Tile::pParent);
        let root: Ptr<Tile> = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr();
        if parent == root {
            let stacking = fn_00a011b0(e, tile, TRAIT_STACKING_TYPE);
            if stacking as f64 == e.global::<f64>(STACKING_TYPE_102) {
                let value = e.global::<f32>(STACKING_VALUE_6000);
                fn_00a012d0(e, tile, TRAIT_STACKING_TYPE, value, true);
            } else {
                let stacking = fn_00a011b0(e, tile, TRAIT_STACKING_TYPE);
                if stacking as f64 == e.global::<f64>(STACKING_TYPE_103) {
                    let value = e.global::<f32>(STACKING_VALUE_6001);
                    fn_00a012d0(e, tile, TRAIT_STACKING_TYPE, value, true);
                }
            }
        }
    }
    if e.global::<u8>(NEEDS_CHECK_HIBERNATE) != 0 {
        tile_check_hibernating_tiles(e);
        tile_update_all(e, flag);
        leave_critical_section(e, TILE_CRITICAL_SECTION);
        e.mem.set_u32(line_word, saved_line);
        return;
    }
    if flag {
        let inner_line = e.mem.u32(line_word);
        e.mem.set_u32(line_word, 0xd);
        let manager = e.global::<u32>(INTERFACE_MANAGER);
        e.call(INTERFACE_MANAGER_END_OF_UPDATE, &args![manager]);
        e.mem.set_u32(line_word, inner_line);
    }
    if e.call(INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE, &args![])
        .u8()
        != 0
    {
        let inner_line = e.mem.u32(line_word);
        e.mem.set_u32(line_word, 0xd);
        e.call(INTERFACE_PREPARE_TEXTURE_RELEASE, &args![0u32]);
        e.call(TEXTURE_PALETTE_PURGE_UNUSED, &args![]);
        e.mem.set_u32(line_word, inner_line);
    }
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    e.mem.set_u32(line_word, saved_line);
}

// Translated from 00a044f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Lock` (Xbox PDB): `EnterCriticalSection` on the tile critical
/// section.
pub fn tile_lock(e: &mut Engine) {
    enter_critical_section(e, TILE_CRITICAL_SECTION);
}

// Translated from 00a04500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Unlock` (Xbox PDB): `LeaveCriticalSection` on the tile critical
/// section.
pub fn tile_unlock(e: &mut Engine) {
    leave_critical_section(e, TILE_CRITICAL_SECTION);
}

// Translated from 00a04510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Wakes hibernating tiles while the frame has update
/// budget: when the tiles updated this frame plus the dirty tiles number
/// 10 or fewer, up to `11 - that` tiles are taken from the front of the
/// hibernating list; each loses the hibernated bit, gets the promoted bit
/// and is added to the dirty list. (The compiler folded the `if (0)` /
/// `if (1)` bit setters of its flag helper away.)
pub fn fn_00a04510(e: &mut Engine) {
    let used = e
        .global::<i32>(TILES_UPDATED_THIS_FRAME)
        .wrapping_add(e.mem.u32(DIRTY_TILES_LIST + 8) as i32);
    if used > 10 {
        return;
    }
    let mut budget = 11 - used;
    let mut node = e.mem.u32(HIBERNATING_TILES_LIST);
    while node != 0 && budget != 0 {
        let tile: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        if !tile.is_null() {
            let flags = e.get(tile, Tile::uiFlags) & !FLAG_HIBERNATED;
            e.set(tile, Tile::uiFlags, flags | FLAG_PROMOTED);
            tile_add_dirty_tile(e, tile);
        }
        node = e.with_stack(4, |e, iterator| {
            e.mem.set_u32(iterator.addr(), node);
            e.call(LIST_REMOVE_NODE, &args![HIBERNATING_TILES_LIST, iterator]);
            e.mem.u32(iterator.addr())
        });
        budget -= 1;
    }
}

// Translated from 00a04620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::UpdateChildren` (Xbox PDB): `UpdateAll(false)`; the tile and the
/// stack word it is given are not used.
pub fn tile_update_children(e: &mut Engine, _this: Ptr<Tile>, _unused_1: u32) {
    tile_update_all(e, false);
}

/// The first child object of a model (`NiNode`): null when the children
/// array has no element (the 16-bit count at +0xA6 is zero).
fn first_child_object(e: &Engine, model: u32) -> u32 {
    if e.mem.u16(model + 0xa6) == 0 {
        0
    } else {
        e.mem.u32(e.mem.u32(model + 0xa0))
    }
}

/// A texture file name that means "none": null, empty or a single space.
fn is_blank_name(e: &Engine, name: u32) -> bool {
    name == 0 || e.mem.i8(name) == 0 || (e.mem.i8(name) == 0x20 && e.mem.i8(name + 1) == 0)
}

/// Clears the update bit `bit` of the tile once it was handled, when the
/// caller asked for that (`flag`).
fn clear_update_bit(e: &mut Engine, this: Ptr<Tile>, flag: bool, bit: u32) {
    let flags = e.get(this, Tile::uiFlags);
    if flag && flags & bit != 0 {
        e.set(this, Tile::uiFlags, flags ^ bit);
    }
}

// Translated from 00a04640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::UpdateTile` (Xbox PDB), `thiscall(flag)`: brings the tile's model
/// up to date with its traits, one update bit at a time, under the tile
/// critical section; answers whether anything was done. With `flag` the
/// handled bits are cleared, and the tile is put to sleep
/// ([`fn_00a077c0`]) when it still has update bits; without bits, or without
/// `flag`, `iTilesUpdatedThisFrame` is counted up instead. Nothing happens
/// for a tile without update bits. The current-line word of the TLS block
/// is 13 while the bits are handled. In the order of the game:
///
/// * `ebfCreate` (2): [`tile_delete_model`], then the virtual `MakeNode`
///   (slot +8) builds a new model (for a text tile `00706dd0(1)` follows);
/// * `ebfVisibility` (4): the model's culled flag follows the `visible`
///   trait; a model that was culled and is shown again sets
///   `bNeedsCheckHibernate`;
/// * when the interface wants textures released, an image tile's shader
///   property gets no texture and the tile the texture update bit;
/// * `ebfTexture` (32), image and hot-rect tiles: loads the texture named by
///   the `filename` trait (the empty placeholder `Interface\Shared\empty.dds`
///   when there is none and no action computes it), through the texture
///   atlas when the tile has an atlas trait, hands it to the shader
///   property, derives the file size traits from it and queues a geometry
///   update;
/// * `ebfGeometry` (16), image tiles: writes the model's four corners from
///   the `width` and `height` traits, the texture coordinates from the crop
///   traits, the atlas rectangle and the zoom, and updates the bound;
/// * `ebfNIFFile` (64): for a 3D tile `Tile3D::UpdateNIF`;
/// * `ebfPosition` (1): the model's translation (and, for a tile that is
///   neither a locus nor a child of the menus root, a rotation about the
///   axis traits) from the position, depth and rotation traits;
/// * `ebfLocus` (512): only cleared;
/// * `ebfScissorWindow` (128): [`tile_re_clip_children`] with the tile's
///   screen rectangle;
/// * `ebfScissor` (256): the scissor of the tile's own model children from
///   its nearest clip-window ancestor;
/// * `ebfColor` (8): alpha and colour, from the colour traits, the system
///   colour table and the brightness, handed to the virtual
///   `SetAlphaAndColor` (slot +0x24).
pub fn tile_update_tile(e: &mut Engine, this: Ptr<Tile>, flag: bool) -> bool {
    enter_critical_section(e, TILE_CRITICAL_SECTION);
    let mut result = false;
    if e.get(this, Tile::uiFlags) & UPDATE_MASK != 0 {
        let line_word = e.tls() + TLS_CURRENT_LINE;
        let saved_line = e.mem.u32(line_word);
        e.mem.set_u32(line_word, 0xd);
        update_tile_create(e, this, flag, &mut result);
        update_tile_visibility(e, this, flag, &mut result);
        update_tile_release_texture(e, this);
        let kind = tile_type(e, this);
        let is_image = kind == TYPE_IMAGE || kind == TYPE_HOT_RECT;
        // The atlas rectangle (left, right, top, bottom), -1.0 when unknown.
        e.with_stack(0x10, |e, rect| {
            let unknown = e.global::<f32>(MINUS_ONE);
            for index in 0..4 {
                e.mem.set_f32(rect.addr() + index * 4, unknown);
            }
            update_tile_texture(e, this, flag, is_image, rect, &mut result);
            update_tile_geometry(e, this, flag, is_image, rect, &mut result);
        });
        update_tile_nif(e, this, flag, &mut result);
        update_tile_position(e, this, flag, &mut result);
        clear_update_bit(e, this, flag, UPDATE_LOCUS);
        update_tile_scissor_window(e, this, flag, &mut result);
        update_tile_scissor(e, this, flag, &mut result);
        update_tile_color(e, this, flag);
        e.mem.set_u32(line_word, saved_line);
    }
    if flag && e.get(this, Tile::uiFlags) & UPDATE_MASK != 0 {
        fn_00a077c0(e, this);
    } else {
        let count = e.global::<u32>(TILES_UPDATED_THIS_FRAME);
        e.set_global(TILES_UPDATED_THIS_FRAME, count.wrapping_add(1));
    }
    leave_critical_section(e, TILE_CRITICAL_SECTION);
    result
}

/// `ebfCreate` of [`tile_update_tile`].
fn update_tile_create(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::uiFlags) & UPDATE_CREATE == 0 {
        return;
    }
    tile_delete_model(e, this);
    if e.vcall(this.addr(), 8, &[]).u32() != 0 {
        if tile_type(e, this) == TYPE_TEXT {
            e.call(TEXT_TILE_CREATED, &args![1u32]);
        }
        *result = true;
        clear_update_bit(e, this, flag, UPDATE_CREATE);
    }
}

/// `ebfVisibility` of [`tile_update_tile`].
fn update_tile_visibility(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::uiFlags) & UPDATE_VISIBILITY == 0 {
        return;
    }
    let model = e.get(this, Tile::spModel).addr();
    if model == 0 {
        return;
    }
    let was_culled = e.mem.u32(model + 0x30) & 1 != 0;
    let visible = fn_00a011b0(e, this, TRAIT_VISIBLE);
    let hidden = visible as f64 == e.global::<f64>(ZERO);
    let model = e.get(this, Tile::spModel).addr();
    let object_flags = e.mem.u32(model + 0x30);
    e.mem.set_u32(
        model + 0x30,
        if hidden {
            object_flags | 1
        } else {
            object_flags & !1
        },
    );
    *result = true;
    if was_culled {
        let model = e.get(this, Tile::spModel).addr();
        if e.mem.u32(model + 0x30) & 1 == 0 {
            e.set_global(NEEDS_CHECK_HIBERNATE, 1u8);
        }
    }
    clear_update_bit(e, this, flag, UPDATE_VISIBILITY);
}

/// When the interface asks for textures to be released, an image tile's
/// shader property loses its texture and the tile gets `ebfTexture`.
fn update_tile_release_texture(e: &mut Engine, this: Ptr<Tile>) {
    if e.call(INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE, &args![])
        .u8()
        == 0
    {
        return;
    }
    let model = e.get(this, Tile::spModel).addr();
    if model == 0 {
        return;
    }
    let child = first_child_object(e, model);
    if tile_type(e, this) == TYPE_IMAGE && child != 0 {
        let property = e.call(NI_OBJECT_GET_PROPERTY, &args![child, 3u32]).u32();
        if property != 0 {
            e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
            tile_add_needs_update(e, this, UPDATE_TEXTURE);
        }
    }
}

/// A reference-counted copy of the word at +0x3C of an image tile (its
/// texture pointer), made for the by-value last argument of
/// `TileImage::AddToTesTextures`.
fn counted_copy_of_texture(e: &mut Engine, this: Ptr<Tile>) -> u32 {
    let texture = e.mem.u32(this.addr() + 0x3c);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    texture
}

/// `ebfTexture` of [`tile_update_tile`], image and hot-rect tiles.
fn update_tile_texture(
    e: &mut Engine,
    this: Ptr<Tile>,
    flag: bool,
    is_image: bool,
    rect: Ptr,
    result: &mut bool,
) {
    if !is_image || e.get(this, Tile::uiFlags) & UPDATE_TEXTURE == 0 {
        return;
    }
    let model = e.get(this, Tile::spModel).addr();
    if model == 0 {
        return;
    }
    let child = first_child_object(e, model);
    if child == 0 {
        return;
    }
    if e.get(this, Tile::uiFlags) & FLAG_PROMOTED == 0 && !tile_is_visible(e, this) {
        return;
    }
    let property = e.call(NI_OBJECT_GET_PROPERTY, &args![child, 3u32]).u32();
    if property == 0 {
        return;
    }
    // The tile's texture, a counted reference that is released at the end.
    let mut texture = 0u32;
    let value: Ptr<TileValue> = tile_get_value(e, this, TRAIT_FILENAME).cast();
    let mut actions = 0u32;
    let mut text = 0u32;
    let mut used_placeholder = false;
    if !value.is_null() {
        actions = e.get(value, TileValue::pActionListA).addr();
        text = e.get(value, TileValue::strValue).addr();
    }
    if (text == 0 || e.mem.i8(text) == 0) && actions == 0 {
        fn_00a01350(e, this, TRAIT_FILENAME, Ptr::new(EMPTY_TEXTURE_NAME), true);
        text = tile_get_string(e, this, TRAIT_FILENAME).addr();
        used_placeholder = true;
    }
    if text != 0 && e.mem.i8(text) != 0 {
        if used_placeholder || !tile_get_value(e, this, TRAIT_TEX_ATLAS).is_null() {
            e.with_stack(8, |e, atlas_texture_name| {
                e.call(BSSTRING_CONSTRUCT, &args![atlas_texture_name, EMPTY_NAME]);
                let atlas = if used_placeholder {
                    e.global::<u32>(DEFAULT_ATLAS_NAME)
                } else {
                    tile_get_string(e, this, TRAIT_TEX_ATLAS).addr()
                };
                let subtexture = tile_get_string(e, this, TRAIT_FILENAME);
                tile_get_texture_atlas_info(
                    e,
                    Ptr::new(atlas),
                    subtexture,
                    atlas_texture_name,
                    rect,
                );
                let copy = counted_copy_of_texture(e, this);
                let name = e.mem.u32(atlas_texture_name.addr());
                let zoom = fn_00a011b0(e, this, TRAIT_ZOOM);
                e.with_stack(4, |e, slot| {
                    e.call(
                        ADD_TO_TES_TEXTURES,
                        &args![name, this.addr() + 0x38, slot, zoom, copy],
                    );
                    texture = e.mem.u32(slot.addr());
                });
                e.call(STRING_SET, &args![atlas_texture_name, 0u32, 0u32]);
            });
        }
        if texture == 0 {
            let copy = counted_copy_of_texture(e, this);
            let zoom = fn_00a011b0(e, this, TRAIT_ZOOM);
            e.with_stack(4, |e, slot| {
                e.call(
                    ADD_TO_TES_TEXTURES,
                    &args![text, this.addr() + 0x38, slot, zoom, copy],
                );
                texture = e.mem.u32(slot.addr());
            });
        }
    }
    if texture != 0 {
        e.call(SET_TILE_TEXTURE, &args![property, texture]);
        let tiled = tile_is_true(e, this, TRAIT_TILE);
        e.mem.set_u32(property + 0x8c, if tiled { 2 } else { 0 });
        let mut zoom = fn_00a011b0(e, this, TRAIT_ZOOM);
        let name = tile_get_string(e, this, TRAIT_FILENAME).addr();
        // `NiTexture` virtuals +0x94 and +0x98: the width and height.
        let width = e.vcall(texture, 0x94, &[]).u32() as f64 as f32;
        let height = e.vcall(texture, 0x98, &[]).u32() as f64 as f32;
        let divisor = e.mem.f32(this.addr() + 0x38) as f64;
        if fn_00a011b0(e, this, TRAIT_TILE) as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
            if is_blank_name(e, name) {
                fn_00a012d0(e, this, TRAIT_FILE_WIDTH, width, true);
                fn_00a012d0(e, this, TRAIT_FILE_HEIGHT, height, true);
            } else {
                let file_width = (width as f64 / divisor) as f32;
                fn_00a012d0(e, this, TRAIT_FILE_WIDTH, file_width, true);
                let file_height = (height as f64 / divisor) as f32;
                fn_00a012d0(e, this, TRAIT_FILE_HEIGHT, file_height, true);
            }
            tile_add_needs_update(e, this, UPDATE_GEOMETRY);
            zoom = e.global::<f32>(MINUS_ONE);
        }
        if zoom >= 0.0 {
            if zoom as f64 == e.global::<f64>(ZERO) {
                zoom = e.global::<f32>(HUNDRED_FLOAT);
            }
            let scale = zoom as f64 / e.global::<f64>(HUNDRED_DOUBLE);
            if is_blank_name(e, name) {
                fn_00a012d0(
                    e,
                    this,
                    TRAIT_FILE_WIDTH,
                    (width as f64 * scale) as f32,
                    true,
                );
                fn_00a012d0(
                    e,
                    this,
                    TRAIT_FILE_HEIGHT,
                    (height as f64 * scale) as f32,
                    true,
                );
            } else {
                let file_width = (width as f64 * scale / divisor) as f32;
                fn_00a012d0(e, this, TRAIT_FILE_WIDTH, file_width, true);
                let file_height = (height as f64 * scale / divisor) as f32;
                fn_00a012d0(e, this, TRAIT_FILE_HEIGHT, file_height, true);
            }
            tile_add_needs_update(e, this, UPDATE_GEOMETRY);
        }
        *result = true;
        clear_update_bit(e, this, flag, UPDATE_TEXTURE);
    } else if flag
        && !tile_is_true(e, this, TRAIT_VISIBLE)
        && e.get(this, Tile::uiFlags) & UPDATE_TEXTURE != 0
    {
        clear_update_bit(e, this, flag, UPDATE_TEXTURE);
    }
    if texture != 0 {
        release_reference(e, texture);
    }
}

/// `ebfGeometry` of [`tile_update_tile`], image tiles.
fn update_tile_geometry(
    e: &mut Engine,
    this: Ptr<Tile>,
    flag: bool,
    is_image: bool,
    rect: Ptr,
    result: &mut bool,
) {
    // Whether the model's geometry data is static (consistency bits
    // 0x7000 equal 0x4000): then its vertices and bound stay as they are.
    let mut static_geometry = false;
    if e.get(this, Tile::uiFlags) & UPDATE_GEOMETRY != 0 {
        let model = e.get(this, Tile::spModel).addr();
        if model != 0 {
            let child = first_child_object(e, model);
            // Virtual slot +0x1C (`IsTriBasedGeom`).
            let geometry = if child != 0 {
                e.vcall(child, 0x1c, &[]).u32()
            } else {
                0
            };
            let data = if geometry != 0 {
                e.mem.u32(geometry + 0xb8)
            } else {
                0
            };
            if data != 0 && e.mem.u16(data + 0xe) & 0x7000 == 0x4000 {
                static_geometry = true;
            }
        }
    }
    if e.get(this, Tile::uiFlags) & UPDATE_GEOMETRY != 0 {
        let model = e.get(this, Tile::spModel).addr();
        if model != 0 {
            let child = first_child_object(e, model);
            let width = fn_00a011b0(e, this, TRAIT_WIDTH);
            let height = fn_00a011b0(e, this, TRAIT_HEIGHT);
            let crop_x = fn_00a011b0(e, this, TRAIT_CROP_X);
            let crop_y = fn_00a011b0(e, this, TRAIT_CROP_Y);
            if child != 0 && tile_type(e, this) == TYPE_IMAGE {
                update_image_geometry(
                    e,
                    this,
                    is_image,
                    rect,
                    child,
                    static_geometry,
                    [width, height, crop_x, crop_y],
                );
                *result = true;
                let data = e.mem.u32(child + 0xb8);
                if data != 0 {
                    e.call(GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0u32]);
                }
            }
            clear_update_bit(e, this, flag, UPDATE_GEOMETRY);
        }
    }
}

/// The body of `ebfGeometry` for the image tile whose model has the
/// geometry `child`; `size` is (`width`, `height`, `cropx`, `cropy`).
fn update_image_geometry(
    e: &mut Engine,
    this: Ptr<Tile>,
    is_image: bool,
    rect: Ptr,
    child: u32,
    static_geometry: bool,
    size: [f32; 4],
) {
    let [width, height, crop_x, crop_y] = size;
    if !static_geometry {
        let data = e.mem.u32(child + 0xb8);
        if data != 0 && e.mem.u32(data + 0x20) != 0 {
            // The four corners of the quad: (0,0,0), (0,0,-height),
            // (width,0,0), (width,0,-height).
            let vertices = e.mem.u32(data + 0x20);
            let corners = [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -height],
                [width, 0.0, 0.0],
                [width, 0.0, -height],
            ];
            for (index, corner) in corners.iter().enumerate() {
                for (axis, value) in corner.iter().enumerate() {
                    e.mem
                        .set_f32(vertices + index as u32 * 12 + axis as u32 * 4, *value);
                }
            }
        }
    }
    // The size the texture coordinates are relative to.
    let mut texture_width = width;
    let mut texture_height = height;
    let property = e.call(NI_OBJECT_GET_PROPERTY, &args![child, 3u32]).u32();
    if property != 0 && is_image && e.mem.u32(property + 0x60) != 0 {
        let tiled = fn_00a011b0(e, this, TRAIT_TILE);
        e.mem.set_u32(
            property + 0x8c,
            if tiled as f64 == e.global::<f64>(ZERO) {
                0
            } else {
                2
            },
        );
        let mut zoom = fn_00a011b0(e, this, TRAIT_ZOOM);
        if fn_00a011b0(e, this, TRAIT_TILE) as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
            let texture = e.mem.u32(property + 0x60);
            let file_height = e.vcall(texture, 0x98, &[]).u32() as f64;
            let ratio = height as f64 / file_height;
            let file_width = e.vcall(texture, 0x94, &[]).u32() as f64;
            texture_width = (file_width * ratio) as f32;
        } else if zoom >= 0.0 {
            if zoom as f64 == e.global::<f64>(ZERO) {
                zoom = e.global::<f32>(HUNDRED_FLOAT);
            }
            let scale = zoom as f64 / e.global::<f64>(HUNDRED_DOUBLE);
            let name = tile_get_string(e, this, TRAIT_FILENAME).addr();
            let texture = e.mem.u32(property + 0x60);
            if is_blank_name(e, name) {
                let file_width = e.vcall(texture, 0x94, &[]).u32() as f64;
                texture_width = (file_width * scale) as f32;
                let file_height = e.vcall(texture, 0x98, &[]).u32() as f64;
                texture_height = (file_height * scale) as f32;
            } else {
                let divisor = e.mem.f32(this.addr() + 0x38) as f64;
                let file_width = e.vcall(texture, 0x94, &[]).u32() as f64;
                texture_width = (file_width * scale / divisor) as f32;
                let divisor = e.mem.f32(this.addr() + 0x38) as f64;
                let file_height = e.vcall(texture, 0x98, &[]).u32() as f64;
                texture_height = (file_height * scale / divisor) as f32;
            }
        }
    }
    let data = e.mem.u32(child + 0xb8);
    let coordinates = e.mem.u32(data + 0x2c);
    if coordinates != 0 {
        let mut left = (crop_x as f64 / texture_width as f64) as f32;
        let mut top = (crop_y as f64 / texture_height as f64) as f32;
        let mut span_x = (width as f64 / texture_width as f64) as f32;
        let mut span_y = (height as f64 / texture_height as f64) as f32;
        if !tile_get_value(e, this, TRAIT_TEX_ATLAS).is_null() {
            let minus_one = e.global::<f64>(MINUS_ONE_DOUBLE);
            e.with_stack(0x10, |e, atlas_rect| {
                let known = e.mem.f32(rect.addr() + 4) as f64 != minus_one
                    && e.mem.f32(rect.addr() + 12) as f64 != minus_one;
                if known {
                    for index in 0..4 {
                        let value = e.mem.f32(rect.addr() + index * 4);
                        e.mem.set_f32(atlas_rect.addr() + index * 4, value);
                    }
                } else {
                    let subtexture = tile_get_string(e, this, TRAIT_FILENAME);
                    let atlas = tile_get_string(e, this, TRAIT_TEX_ATLAS);
                    tile_get_texture_atlas_info(e, atlas, subtexture, Ptr::NULL, atlas_rect);
                }
                let rect_left = e.mem.f32(atlas_rect.addr());
                let rect_right = e.mem.f32(atlas_rect.addr() + 4);
                let rect_top = e.mem.f32(atlas_rect.addr() + 8);
                let rect_bottom = e.mem.f32(atlas_rect.addr() + 12);
                let atlas_width = (rect_right as f64 - rect_left as f64) as f32;
                let atlas_height = (rect_bottom as f64 - rect_top as f64) as f32;
                if atlas_width > 0.0 && atlas_height > 0.0 {
                    left = (left as f64 * atlas_width as f64 + rect_left as f64) as f32;
                    top = (top as f64 * atlas_height as f64 + rect_top as f64) as f32;
                    span_x = atlas_width;
                    span_y = atlas_height;
                }
                let mut zoom = fn_00a011b0(e, this, TRAIT_ZOOM);
                if zoom > 0.0 {
                    zoom = (zoom as f64 / e.global::<f64>(HUNDRED_DOUBLE)) as f32;
                    let tile_width = fn_00a011b0(e, this, TRAIT_WIDTH) as f64;
                    let file_width = fn_00a011b0(e, this, TRAIT_FILE_WIDTH) as f64;
                    span_x = (tile_width / file_width * zoom as f64) as f32;
                    let tile_height = fn_00a011b0(e, this, TRAIT_HEIGHT) as f64;
                    let file_height = fn_00a011b0(e, this, TRAIT_FILE_HEIGHT) as f64;
                    span_y = (tile_height / file_height * zoom as f64) as f32;
                }
            });
        }
        let right = (span_x as f64 + left as f64) as f32;
        let bottom = (span_y as f64 + top as f64) as f32;
        let coordinate_values = [left, top, left, bottom, right, top, right, bottom];
        for (index, value) in coordinate_values.iter().enumerate() {
            e.mem.set_f32(coordinates + index as u32 * 4, *value);
        }
    }
    if !static_geometry {
        let data = e.mem.u32(child + 0xb8);
        if data != 0 {
            e.call(GEOMETRY_DATA_MARK_AS_CHANGED, &args![data, 9u32]);
        }
        let data = e.mem.u32(child + 0xb8);
        let vertices = e.mem.u32(data + 0x20);
        let count = e.mem.u16(data + 8) as u32;
        e.call(
            BOUND_COMPUTE_FROM_DATA,
            &args![data + 0x10, count, vertices],
        );
    }
}

/// `ebfNIFFile` of [`tile_update_tile`].
fn update_tile_nif(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::uiFlags) & UPDATE_NIF_FILE == 0 || e.get(this, Tile::spModel).is_null() {
        return;
    }
    if tile_type(e, this) == TYPE_3D {
        e.call(TILE_3D_UPDATE_NIF, &args![this]);
    }
    *result = true;
    clear_update_bit(e, this, flag, UPDATE_NIF_FILE);
}

/// The three floats the position code starts from (the default
/// translation at `011f426c`).
fn default_translation(e: &Engine) -> [f32; 3] {
    [
        e.global::<f32>(DEFAULT_TRANSLATION),
        e.global::<f32>(DEFAULT_TRANSLATION + 4),
        e.global::<f32>(DEFAULT_TRANSLATION + 8),
    ]
}

fn set_translation(e: &mut Engine, object: u32, translation: [f32; 3]) {
    for (axis, value) in translation.iter().enumerate() {
        e.mem.set_f32(object + 0x58 + axis as u32 * 4, *value);
    }
}

/// `ebfPosition` of [`tile_update_tile`].
fn update_tile_position(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::uiFlags) & UPDATE_POSITION == 0 || e.get(this, Tile::spModel).is_null() {
        return;
    }
    let child_of_root = tile_is_true(e, this, TRAIT_LOCUS) || {
        let root: Ptr<Tile> = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr();
        e.get(this, Tile::pParent) == root
    };
    if child_of_root {
        let depth = fn_00a011b0(e, this, TRAIT_DEPTH);
        let mut translation = default_translation(e);
        translation[0] = fn_00a011b0(e, this, TRAIT_X);
        translation[2] = -fn_00a011b0(e, this, TRAIT_Y);
        translation[1] = (depth as f64 * e.global::<f64>(DEPTH_SCALE)) as f32;
        let root: Ptr<Tile> = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr();
        if e.get(this, Tile::pParent) == root {
            let screen_width = e.call(SCREEN_WIDTH, &args![]).f32();
            translation[0] =
                (translation[0] as f64 - screen_width as f64 / e.global::<f64>(TWO)) as f32;
            let screen_height = e.call(SCREEN_HEIGHT, &args![]).f32();
            let sum = screen_height as f64 + translation[2] as f64;
            let screen_height = e.call(SCREEN_HEIGHT, &args![]).f32();
            translation[2] = (sum - screen_height as f64 / e.global::<f64>(TWO)) as f32;
        }
        let model = e.get(this, Tile::spModel).addr();
        set_translation(e, model, translation);
        if e.get(this, Tile::uiFlags) & UPDATE_LOCUS != 0 {
            let model = e.get(this, Tile::spModel).addr();
            let mut index = 0u32;
            while model != 0 && index < e.mem.u16(model + 0xa8) as u32 {
                let object = if index < e.mem.u16(model + 0xa6) as u32 {
                    e.mem.u32(e.mem.u32(model + 0xa0) + index * 4)
                } else {
                    0
                };
                // Virtual slot +0x24 (`IsTriShape`).
                let shape = if object != 0 {
                    e.vcall(object, 0x24, &[]).u32()
                } else {
                    0
                };
                if shape != 0 && e.get(this, Tile::uiFlags) & FLAG_MANUAL_UPDATE_TRIS == 0 {
                    let translation = default_translation(e);
                    set_translation(e, shape, translation);
                }
                index += 1;
            }
        }
        tile_update_clipwindows(e, this);
    } else {
        let depth = fn_00a011b0(e, this, TRAIT_DEPTH);
        let mut translation = default_translation(e);
        let model = e.get(this, Tile::spModel).addr();
        set_translation(e, model, translation);
        translation[0] = (fn_00a011b0(e, this, TRAIT_X) as f64 + translation[0] as f64) as f32;
        translation[2] = (-fn_00a011b0(e, this, TRAIT_Y) as f64 + translation[2] as f64) as f32;
        translation[1] =
            (depth as f64 * e.global::<f64>(DEPTH_SCALE) + translation[1] as f64) as f32;
        let model = e.get(this, Tile::spModel).addr();
        let mut index = 0u32;
        while model != 0 && index < e.mem.u16(model + 0xa8) as u32 {
            let object = e.mem.u32(e.mem.u32(model + 0xa0) + index * 4);
            let shape = if object == 0 {
                0
            } else {
                // The object's RTTI chain contains the record at 011f4a40.
                let mut rtti = e.vcall(object, 8, &[]).u32();
                while rtti != 0 && rtti != SHAPE_RTTI {
                    rtti = e.mem.u32(rtti + 4);
                }
                if rtti != 0 {
                    object
                } else {
                    0
                }
            };
            let axis_x = fn_00a011b0(e, this, TRAIT_ROTATE_AXIS_X);
            let axis_z = -fn_00a011b0(e, this, TRAIT_ROTATE_AXIS_Y);
            let angle = fn_00a011b0(e, this, TRAIT_ROTATE_ANGLE);
            let (sine, cosine) = ((angle as f64).sin() as f32, (angle as f64).cos() as f32);
            let rotation = [cosine, 0.0, -sine, 0.0, 1.0, 0.0, sine, 0.0, cosine];
            if shape != 0 {
                for (slot, value) in rotation.iter().enumerate() {
                    e.mem.set_f32(shape + 0x34 + slot as u32 * 4, *value);
                }
                let moved = rotate_about_axis(e, rotation, [axis_x, 0.0, axis_z], translation);
                set_translation(e, shape, moved);
            }
            index += 1;
        }
    }
    *result = true;
    clear_update_bit(e, this, flag, UPDATE_POSITION);
}

/// `(translation + axis) - rotation * axis`, computed with the game's
/// point helpers: the position that makes the rotation about the point
/// `axis`.
fn rotate_about_axis(
    e: &mut Engine,
    rotation: [f32; 9],
    axis: [f32; 3],
    translation: [f32; 3],
) -> [f32; 3] {
    e.with_stack(0x24 + 5 * 12, |e, scratch| {
        let matrix = scratch.addr();
        let axis_at = matrix + 0x24;
        let translation_at = axis_at + 12;
        let rotated_at = translation_at + 12;
        let sum_at = rotated_at + 12;
        let result_at = sum_at + 12;
        for (slot, value) in rotation.iter().enumerate() {
            e.mem.set_f32(matrix + slot as u32 * 4, *value);
        }
        for (slot, value) in axis.iter().enumerate() {
            e.mem.set_f32(axis_at + slot as u32 * 4, *value);
        }
        for (slot, value) in translation.iter().enumerate() {
            e.mem.set_f32(translation_at + slot as u32 * 4, *value);
        }
        let rotated = e
            .call(MATRIX_TIMES_POINT, &args![matrix, rotated_at, axis_at])
            .u32();
        let sum = e
            .call(POINT_ADD, &args![translation_at, sum_at, axis_at])
            .u32();
        let moved = e
            .call(POINT_SUBTRACT, &args![sum, result_at, rotated])
            .u32();
        [e.mem.f32(moved), e.mem.f32(moved + 4), e.mem.f32(moved + 8)]
    })
}

/// The scale from layout units to the real screen that the scissor code
/// uses, and the real screen size: (`scale_x`, `scale_y`, `width`,
/// `height`). A rendered menu has its own size and aspect ratio.
fn scissor_scale(e: &mut Engine, this: Ptr<Tile>) -> (f32, f32, f32, f32) {
    let mut width = e.call(SCREEN_REAL_WIDTH, &args![]).f32();
    let mut height = e.call(SCREEN_REAL_HEIGHT, &args![]).f32();
    if tile_get_rendered_menu(e, this) {
        width = e.call(RENDERED_MENU_WIDTH, &args![]).f32();
        height = e.call(RENDERED_MENU_HEIGHT, &args![]).f32();
    }
    let screen_width = e.call(SCREEN_WIDTH, &args![]).f32();
    let mut scale_x = (width as f64 / screen_width as f64) as f32;
    let screen_height = e.call(SCREEN_HEIGHT, &args![]).f32();
    let scale_y = (height as f64 / screen_height as f64) as f32;
    let menu = tile_get_menu(e, this);
    if e.call(INTERFACE_IS_RENDERED_MENU, &args![menu]).u8() != 0 {
        let aspect = e.call(SCREEN_ASPECT_RATIO, &args![]).f32();
        scale_x = (aspect as f64 * scale_x as f64) as f32;
    }
    (scale_x, scale_y, width, height)
}

/// The screen rectangle (left, top, right, bottom) of `tile` in the real
/// screen's units.
fn scissor_rectangle(
    e: &mut Engine,
    tile: Ptr<Tile>,
    scale_x: f32,
    scale_y: f32,
) -> (f32, f32, f32, f32) {
    let left = fn_00a013d0(e, tile);
    let top = fn_00a01440(e, tile);
    let width = fn_00a011b0(e, tile, TRAIT_WIDTH);
    let right = ((width as f64 + left as f64) * scale_x as f64) as f32;
    let height = fn_00a011b0(e, tile, TRAIT_HEIGHT);
    let bottom = ((height as f64 + top as f64) * scale_y as f64) as f32;
    let left = (left as f64 * scale_x as f64) as f32;
    let top = (top as f64 * scale_y as f64) as f32;
    (left, top, right, bottom)
}

/// `ebfScissorWindow` of [`tile_update_tile`].
fn update_tile_scissor_window(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::spModel).is_null()
        || e.get(this, Tile::uiFlags) & UPDATE_SCISSOR_WINDOW == 0
    {
        return;
    }
    let (scale_x, scale_y, _, _) = scissor_scale(e, this);
    let (left, top, right, bottom) = scissor_rectangle(e, this, scale_x, scale_y);
    tile_re_clip_children(e, this, left, top, right, bottom);
    *result = true;
    clear_update_bit(e, this, flag, UPDATE_SCISSOR_WINDOW);
}

/// `ebfScissor` of [`tile_update_tile`].
fn update_tile_scissor(e: &mut Engine, this: Ptr<Tile>, flag: bool, result: &mut bool) {
    if e.get(this, Tile::spModel).is_null() || e.get(this, Tile::uiFlags) & UPDATE_SCISSOR == 0 {
        return;
    }
    // The nearest ancestor with the clip-window trait.
    let mut window = e.get(this, Tile::pParent);
    while !window.is_null() && !tile_is_true(e, window, TRAIT_CLIP_WINDOW) {
        window = e.get(window, Tile::pParent);
    }
    if !window.is_null() {
        let (scale_x, scale_y, width, height) = scissor_scale(e, this);
        let (left, top, right, bottom) = scissor_rectangle(e, window, scale_x, scale_y);
        let model = e.get(this, Tile::spModel).addr();
        e.call(NI_ARRAY_COMPACT, &args![model + 0x9c]);
        e.call(NI_ARRAY_UPDATE_SIZE, &args![model + 0x9c]);
        let mut index = 0u32;
        loop {
            let model = e.get(this, Tile::spModel).addr();
            if index >= e.mem.u16(model + 0xa8) as u32 {
                break;
            }
            let model = e.get(this, Tile::spModel).addr();
            let object = e.mem.u32(e.mem.u32(model + 0xa0) + index * 4);
            let property = e.call(NI_OBJECT_GET_PROPERTY, &args![object, 3u32]).u32();
            if property != 0 {
                let x0 = float_to_int(e, if 0.0 < left { left } else { 0.0 });
                let y0 = float_to_int(e, if 0.0 < top { top } else { 0.0 });
                let x1 = float_to_int(e, if right < width { right } else { width });
                let y1 = float_to_int(e, if bottom < height { bottom } else { height });
                e.mem.set_i32(property + 0x9c, x0);
                e.mem.set_i32(property + 0xa0, y0);
                e.mem.set_i32(property + 0xa4, x1);
                e.mem.set_i32(property + 0xa8, y1);
                e.call(SHADER_PROPERTY_REFRESH, &args![property, 1u32]);
            }
            index += 1;
        }
    }
    *result = true;
    clear_update_bit(e, this, flag, UPDATE_SCISSOR);
}

/// `ebfColor` of [`tile_update_tile`]: the colour (red, green, blue,
/// alpha 1.0 each at first) comes from the `systemcolor` table when the
/// tile or an ancestor names an entry, scaled by the brightness, else from
/// the colour traits divided by 255; the alpha is the `alpha` trait over
/// 255. A tile with the speech-challenge failure flag that is the
/// `lb_highlight_box`, or a child of it, turns dark red.
fn update_tile_color(e: &mut Engine, this: Ptr<Tile>, flag: bool) {
    if e.get(this, Tile::uiFlags) & UPDATE_COLOR == 0 {
        return;
    }
    let two_fifty_five = e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE);
    let mut color = [1.0f32; 4];
    let alpha = fn_00a011b0(e, this, TRAIT_ALPHA);
    let mut alpha = (alpha as f64 / two_fifty_five) as f32;
    let mut brightness = fn_00a011b0(e, this, TRAIT_BRIGHTNESS);
    if brightness >= 0.0 {
        brightness = (brightness as f64 / two_fifty_five) as f32;
    }
    if brightness >= 0.0 {
        let mut entry = 0i32;
        let mut tile = this;
        while !tile.is_null() {
            if !tile_get_value(e, tile, TRAIT_SYSTEM_COLOR).is_null()
                || fn_00a011b0(e, tile, TRAIT_ID) as f64 == e.global::<f64>(ID_VALUE_111)
            {
                break;
            }
            let parent = e.get(tile, Tile::pParent);
            if !parent.is_null()
                && !e.get(parent, Tile::pParent).is_null()
                && e.get(e.get(parent, Tile::pParent), Tile::pParent).is_null()
                && fn_00a011b0(e, tile, TRAIT_ID) as f64 == e.global::<f64>(ID_VALUE_110)
            {
                let system_color = fn_00a011b0(e, tile, TRAIT_SYSTEM_COLOR);
                entry = float_to_int(e, system_color);
                if entry == 0 {
                    let system_color = fn_00a011b0(e, parent, TRAIT_SYSTEM_COLOR);
                    entry = float_to_int(e, system_color);
                }
                if entry == 0 {
                    entry = 1;
                }
                tile = Ptr::NULL;
                break;
            }
            tile = parent;
        }
        if !tile.is_null() {
            let system_color = fn_00a011b0(e, tile, TRAIT_SYSTEM_COLOR);
            entry = float_to_int(e, system_color);
        }
        let found = e.with_stack(0x10, |e, rgba| {
            for (slot, value) in color.iter().enumerate() {
                e.mem.set_f32(rgba.addr() + slot as u32 * 4, *value);
            }
            let manager = e.call(SYSTEM_COLOR_MANAGER_GET_INSTANCE, &args![]).u32();
            let found = e
                .call(SYSTEM_COLOR_MANAGER_GET_COLOR, &args![manager, entry, rgba])
                .u8()
                != 0;
            if found {
                e.call(COLOR_SCALE, &args![rgba, brightness]);
            }
            for (slot, value) in color.iter_mut().enumerate() {
                *value = e.mem.f32(rgba.addr() + slot as u32 * 4);
            }
            found
        });
        if !found {
            brightness = e.global::<f32>(MINUS_ONE);
        }
    }
    if brightness as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
        for (slot, trait_id) in [TRAIT_RED, TRAIT_GREEN, TRAIT_BLUE].into_iter().enumerate() {
            let channel = fn_00a011b0(e, this, trait_id);
            color[slot] = (channel as f64 / two_fifty_five) as f32;
        }
    }
    if e.mem.u8(this.addr() + 0x35) != 0 {
        let name = e.mem.u32(this.addr() + 0x20);
        let is_box = e.call(STRCMP, &args![name, HIGHLIGHT_BOX_NAME]).i32() == 0;
        let parent = e.get(this, Tile::pParent);
        if is_box {
            color[0] = e.global::<f32>(HIGHLIGHT_RED);
            color[1] = 0.0;
            color[2] = 0.0;
            alpha = e.global::<f32>(HIGHLIGHT_ALPHA);
        } else if !parent.is_null() {
            let parent_name = e.mem.u32(parent.addr() + 0x20);
            if e.call(STRCMP, &args![parent_name, HIGHLIGHT_BOX_NAME])
                .i32()
                == 0
            {
                color[0] = e.global::<f32>(HIGHLIGHT_RED);
                color[1] = 0.0;
                color[2] = 0.0;
                alpha = 1.0;
            }
        }
    }
    let model = e.get(this, Tile::spModel).addr();
    e.with_stack(0x10, |e, rgba| {
        for (slot, value) in color.iter().enumerate() {
            e.mem.set_f32(rgba.addr() + slot as u32 * 4, *value);
        }
        // Virtual slot +0x24 (`SetAlphaAndColor(model, alpha, color)`).
        e.vcall(this.addr(), 0x24, &args![model, alpha, rgba]);
    });
    clear_update_bit(e, this, flag, UPDATE_COLOR);
}

/// Copies what a texture atlas entry answers: the atlas texture's file name
/// into the string `out_texture` (when given) and the rectangle
/// (left, right, top, bottom) into the four floats at `out_rect` (when
/// given).
fn copy_atlas_entry(e: &mut Engine, entry: u32, out_texture: Ptr, out_rect: Ptr) {
    if !out_texture.is_null() {
        let texture_name = e.mem.u32(entry + 0x10);
        e.call(STRING_SET, &args![out_texture, texture_name, 0u32]);
    }
    if !out_rect.is_null() {
        let rect = out_rect.addr();
        // `NiRect<float>` at +0x18 of the entry: left, right, top, bottom.
        let left = e.mem.f32(entry + 0x18);
        e.mem.set_f32(rect, left);
        let bottom = e.mem.f32(entry + 0x24);
        e.mem.set_f32(rect + 12, bottom);
        let top = e.mem.f32(entry + 0x20);
        e.mem.set_f32(rect + 8, top);
        let right = e.mem.f32(entry + 0x1c);
        e.mem.set_f32(rect + 4, right);
    }
}

// Translated from 00a06dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetTextureAtlasInfo` (Xbox PDB), `cdecl`: looks up the sub-texture
/// named `subtexture` (the part after the last backslash) in the texture
/// atlas description `atlas` and answers, through `out_texture` (a
/// `BSStringT`, may be null) the file name of the atlas texture and, through
/// `out_rect` (four floats: left, right, top, bottom; may be null) its
/// rectangle. Nothing is done unless `atlas`, `subtexture` and one of the
/// outputs are given.
///
/// The entries already read are kept on the static list `TextureEntryList`
/// (`011f3350`; every match writes the outputs). Only when neither the
/// sub-texture nor the atlas is known is the file `Data\Textures\<atlas>`
/// opened (`FileFinder::GetFile`; virtual slots +0x20 open, +0x34 read a
/// line up to a newline, 0 delete) and parsed: every line that does not
/// begin with `#`, a newline, tab, return or space is an entry
/// `<name> <texture> <x> <y> <width> <height>` (separated by spaces, commas
/// and tabs; the two tokens after the texture are skipped, and one more
/// after the second number); the texture file name is the atlas's
/// directory plus the second token. Each entry is a new
/// `Tile::TextureAtlasEntry` (0x28 bytes) put on the list; the right and
/// bottom are stored as left plus width and top plus height.
pub fn tile_get_texture_atlas_info(
    e: &mut Engine,
    atlas: Ptr,
    subtexture: Ptr,
    out_texture: Ptr,
    out_rect: Ptr,
) {
    e.with_stack(NAME_BUFFER_SIZE, |e, path| {
        e.call(STRCPY_S, &args![path, NAME_BUFFER_SIZE, TEXTURE_DIRECTORY]);
        if atlas.is_null() || subtexture.is_null() || (out_texture.is_null() && out_rect.is_null())
        {
            return;
        }
        let last_slash = e.call(STRRCHR, &args![subtexture, 0x5cu32]).u32();
        let mut found_subtexture = false;
        let mut found_atlas = false;
        e.call(STRCAT_S, &args![path, NAME_BUFFER_SIZE, atlas]);
        let subtexture = if last_slash != 0 {
            last_slash + 1
        } else {
            subtexture.addr()
        };
        let mut node = TEXTURE_ENTRY_LIST;
        while node != 0 && e.mem.u32(node) != 0 {
            let entry = e.mem.u32(node);
            let entry_atlas = e.mem.u32(entry);
            if e.call(STRCMP, &args![entry_atlas, atlas]).i32() == 0 {
                found_atlas = true;
            }
            let entry_subtexture = e.mem.u32(entry + 8);
            if e.call(STRCMP, &args![entry_subtexture, subtexture]).i32() == 0 {
                copy_atlas_entry(e, entry, out_texture, out_rect);
                found_subtexture = true;
            }
            node = e.mem.u32(node + 4);
        }
        if found_subtexture || found_atlas {
            return;
        }
        let file = e
            .call(FILE_FINDER_GET_FILE, &args![path, 0u32, 0x4000u32])
            .u32();
        if file != 0 && e.vcall(file, 0x20, &args![0u32, 0u32]).u8() != 0 {
            e.with_stack(0x404, |e, line| {
                while e.vcall(file, 0x34, &args![line, 0x400u32, 10u32]).u32() != 0 {
                    read_atlas_line(e, atlas, subtexture, out_texture, out_rect, line);
                }
            });
        }
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
    });
}

/// One line of a texture atlas description, see
/// [`tile_get_texture_atlas_info`].
fn read_atlas_line(
    e: &mut Engine,
    atlas: Ptr,
    subtexture: u32,
    out_texture: Ptr,
    out_rect: Ptr,
    line: Ptr,
) {
    // The game's `std::string` (0x1C bytes) that builds the texture's path.
    e.with_stack(0x1c, |e, text| {
        e.call(STD_STRING_TIDY, &args![text, 0u32, 0u32]);
        e.call(STRTOK, &args![line, ATLAS_DELIMITERS]);
        let first = e.mem.i8(line.addr());
        if matches!(first, 0x23 | 0x0a | 0x09 | 0x0d | 0x20) {
            e.call(STD_STRING_TIDY, &args![text, 1u32, 0u32]);
            return;
        }
        let block = allocate(e, 0x28);
        let entry = e.call(TEXTURE_ATLAS_ENTRY_CONSTRUCT, &args![block]).u32();
        e.call(STRING_SET, &args![entry, atlas, 0u32]);
        e.call(STRING_SET, &args![entry + 8, line, 0u32]);
        let atlas_length = e.call(STRLEN, &args![atlas]).u32();
        e.call(STD_STRING_ASSIGN, &args![text, atlas, atlas_length]);
        let npos = e.global::<u32>(STD_STRING_NPOS);
        // The atlas's directory: the part up to and including the last
        // backslash.
        let directory_length = e.with_stack(4, |e, backslash| {
            e.mem.set_u8(backslash.addr(), 0x5c);
            let position = e
                .call(STD_STRING_RFIND, &args![text, backslash, npos, 1u32])
                .u32();
            position.wrapping_add(1)
        });
        e.with_stack(0x1c, |e, directory| {
            let sub = e
                .call(
                    STD_STRING_SUBSTR,
                    &args![text, directory, 0u32, directory_length],
                )
                .u32();
            e.call(STD_STRING_ASSIGN_STRING, &args![text, sub, 0u32, npos]);
            e.call(STD_STRING_TIDY, &args![directory, 1u32, 0u32]);
        });
        let texture_token = e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]).u32();
        let token_length = e.call(STRLEN, &args![texture_token]).u32();
        e.call(STD_STRING_APPEND, &args![text, texture_token, token_length]);
        let data = if e.mem.u32(text.addr() + 0x18) >= 0x10 {
            e.mem.u32(text.addr() + 4)
        } else {
            text.addr() + 4
        };
        e.call(STRING_SET, &args![entry + 0x10, data, 0u32]);
        for _ in 0..2 {
            e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]);
        }
        // x and y, then (after one more token) width and height.
        let token = e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]).u32();
        let x = e.call(ATOF, &args![token]).f64() as f32;
        e.mem.set_f32(entry + 0x18, x);
        let token = e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]).u32();
        let y = e.call(ATOF, &args![token]).f64() as f32;
        e.mem.set_f32(entry + 0x20, y);
        e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS_NO_TAB]);
        let token = e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]).u32();
        let width = e.call(ATOF, &args![token]).f64() as f32;
        e.mem.set_f32(entry + 0x1c, width);
        let token = e.call(STRTOK, &args![0u32, ATLAS_DELIMITERS]).u32();
        let height = e.call(ATOF, &args![token]).f64() as f32;
        e.mem.set_f32(entry + 0x24, height);
        let right = (e.mem.f32(entry + 0x1c) as f64 + e.mem.f32(entry + 0x18) as f64) as f32;
        e.mem.set_f32(entry + 0x1c, right);
        let bottom = (e.mem.f32(entry + 0x24) as f64 + e.mem.f32(entry + 0x20) as f64) as f32;
        e.mem.set_f32(entry + 0x24, bottom);
        let entry_subtexture = e.mem.u32(entry + 8);
        if e.call(STRCMP, &args![entry_subtexture, subtexture]).i32() == 0 {
            copy_atlas_entry(e, entry, out_texture, out_rect);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), entry);
            e.call(FADE_LIST_ADD_HEAD, &args![TEXTURE_ENTRY_LIST, slot]);
        });
        e.call(STD_STRING_TIDY, &args![text, 1u32, 0u32]);
    });
}

// Translated from 00a074d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SetNeedsUpdate` (Xbox PDB): replaces the tile's update bits (the
/// low ten bits of its flags) with `bits` (ignored when `bits` has any
/// other bit) and, when the flags changed, puts the tile on the dirty list.
pub fn tile_set_needs_update(e: &mut Engine, this: Ptr<Tile>, bits: u32) {
    if bits > UPDATE_MASK {
        return;
    }
    let old = e.get(this, Tile::uiFlags);
    let new = (old & !UPDATE_MASK) | bits;
    e.set(this, Tile::uiFlags, new);
    if new != old {
        tile_add_dirty_tile(e, this);
    }
}

// Translated from 00a07530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::AddNeedsUpdate` (Xbox PDB): adds the update bits `bits` (ignored
/// when `bits` has any bit outside the low ten) and, when the flags
/// changed, puts the tile on the dirty list.
pub fn tile_add_needs_update(e: &mut Engine, this: Ptr<Tile>, bits: u32) {
    if bits > UPDATE_MASK {
        return;
    }
    let old = e.get(this, Tile::uiFlags);
    let new = old | bits;
    e.set(this, Tile::uiFlags, new);
    if new != old {
        tile_add_dirty_tile(e, this);
    }
}

// Translated from 00a07580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetNextDirtyTile` (Xbox PDB), `cdecl(remove)`: the tile at the
/// head of the dirty list, null when the list is empty. With `remove` the
/// tile is taken off the list and loses its dirty bit. Under `Tile::Lock`.
pub fn tile_get_next_dirty_tile(e: &mut Engine, remove: bool) -> Ptr<Tile> {
    lock(e);
    let tile: Ptr<Tile> = if e.mem.u32(DIRTY_TILES_LIST + 8) == 0 {
        Ptr::NULL
    } else if remove {
        let tile: Ptr<Tile> = e.call(LIST_REMOVE_HEAD, &args![DIRTY_TILES_LIST]).ptr();
        if !tile.is_null() {
            let flags = e.get(tile, Tile::uiFlags);
            e.set(tile, Tile::uiFlags, flags & !FLAG_DIRTY);
        }
        tile
    } else {
        let head = e.mem.u32(DIRTY_TILES_LIST);
        Ptr::new(e.mem.u32(head + 8))
    };
    unlock(e);
    tile
}

// Translated from 00a07690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::AddDirtyTile` (Xbox PDB), `cdecl(tile)`: nothing for a null tile.
/// Under `Tile::Lock`: appends the tile to the dirty list unless it is
/// there already (dirty bit) and sets the dirty bit; a hibernated tile is
/// taken off the hibernating list and loses the hibernated bit. (The
/// compiler folded the `if (1)` / `if (0)` bit setters of its flag helper.)
pub fn tile_add_dirty_tile(e: &mut Engine, tile: Ptr<Tile>) {
    if tile.is_null() {
        return;
    }
    lock(e);
    if e.get(tile, Tile::uiFlags) & FLAG_DIRTY == 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), tile.addr());
            e.call(LIST_ADD_TAIL, &args![DIRTY_TILES_LIST, slot]);
        });
        let flags = e.get(tile, Tile::uiFlags);
        e.set(tile, Tile::uiFlags, flags | FLAG_DIRTY);
    }
    if e.get(tile, Tile::uiFlags) & FLAG_HIBERNATED != 0 {
        remove_from_list(e, HIBERNATING_TILES_LIST, tile);
        let flags = e.get(tile, Tile::uiFlags);
        e.set(tile, Tile::uiFlags, flags & !FLAG_HIBERNATED);
    }
    unlock(e);
}

// Translated from 00a077c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Puts a tile to sleep: unless the tile is null, already
/// hibernated, or the last one on the hibernating list, counts it
/// (`iHibernatingTileCount`), appends it to the hibernating list, sets the
/// hibernated bit and clears the promoted bit. (The compiler folded the
/// `if (1)` / `if (0)` bit setters of its flag helper.)
pub fn fn_00a077c0(e: &mut Engine, tile: Ptr<Tile>) {
    if tile.is_null() || e.get(tile, Tile::uiFlags) & FLAG_HIBERNATED != 0 {
        return;
    }
    if e.mem.u32(HIBERNATING_TILES_LIST + 8) != 0 {
        let tail = e.mem.u32(HIBERNATING_TILES_LIST + 4);
        if tile.addr() == e.mem.u32(tail + 8) {
            return;
        }
    }
    let count = e.global::<u32>(HIBERNATING_TILE_COUNT);
    e.set_global(HIBERNATING_TILE_COUNT, count.wrapping_add(1));
    let node = e
        .call(LIST_NEW_NODE, &args![HIBERNATING_TILES_LIST + 8])
        .u32();
    e.mem.set_u32(node + 8, tile.addr());
    e.call(LIST_ADD_NODE_TAIL, &args![HIBERNATING_TILES_LIST, node]);
    let flags = e.get(tile, Tile::uiFlags) | FLAG_HIBERNATED;
    e.set(tile, Tile::uiFlags, flags & !FLAG_PROMOTED);
}

// Translated from 00a078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::CheckHibernatingTiles` (Xbox PDB): walks the hibernating list;
/// every tile that is visible again ([`tile_is_visible`]) loses the
/// hibernated and promoted bits, is added to the dirty list and is taken
/// off the hibernating list (the list's remove-and-advance step), the
/// others are skipped.
pub fn tile_check_hibernating_tiles(e: &mut Engine) {
    let mut node = e.mem.u32(HIBERNATING_TILES_LIST);
    while node != 0 {
        let tile: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        if tile_is_visible(e, tile) {
            let flags = e.get(tile, Tile::uiFlags) & !FLAG_HIBERNATED;
            e.set(tile, Tile::uiFlags, flags & !FLAG_PROMOTED);
            tile_add_dirty_tile(e, tile);
            node = e.with_stack(4, |e, iterator| {
                e.mem.set_u32(iterator.addr(), node);
                e.call(LIST_REMOVE_NODE, &args![HIBERNATING_TILES_LIST, iterator]);
                e.mem.u32(iterator.addr())
            });
        } else {
            node = e.mem.u32(node);
        }
    }
}

// Translated from 00a079d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::UpdateClipwindows` (Xbox PDB): every child with the clip-window
/// trait set gets the scissor-window update bit; for the other children the
/// search goes on among their own children.
pub fn tile_update_clipwindows(e: &mut Engine, this: Ptr<Tile>) {
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        if tile_is_true(e, child, TRAIT_CLIP_WINDOW) {
            tile_add_needs_update(e, child, UPDATE_SCISSOR_WINDOW);
        } else {
            tile_update_clipwindows(e, child);
        }
    }
}

// Translated from 00a07a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ReClipChildren` (Xbox PDB): for every descendant with the clips
/// trait set and a model, gives each of the model's child objects that has a
/// property of type 3 (the shader property) the scissor rectangle
/// `left, top, right, bottom` limited to the screen: the left and top are
/// raised to 0 when negative, the right and bottom lowered to the screen
/// width and height (those of the rendered menu when this tile's menu is
/// rendered) and each is truncated to an integer; then the property is
/// told to refresh (`00bb79d0(1)`).
pub fn tile_re_clip_children(
    e: &mut Engine,
    this: Ptr<Tile>,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
) {
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        if tile_is_true(e, child, TRAIT_CLIPS) && !e.get(child, Tile::spModel).is_null() {
            let model = e.get(child, Tile::spModel).addr();
            e.call(NI_ARRAY_COMPACT, &args![model + 0x9c]);
            e.call(NI_ARRAY_UPDATE_SIZE, &args![model + 0x9c]);
            let mut index = 0u32;
            loop {
                let model = e.get(child, Tile::spModel).addr();
                if index >= e.mem.u16(model + 0xa8) as u32 {
                    break;
                }
                let model = e.get(child, Tile::spModel).addr();
                let object = e.mem.u32(e.mem.u32(model + 0xa0) + index * 4);
                let property = e.call(NI_OBJECT_GET_PROPERTY, &args![object, 3u32]).u32();
                if property != 0 {
                    let mut width = e.call(SCREEN_REAL_WIDTH, &args![]).f32();
                    let mut height = e.call(SCREEN_REAL_HEIGHT, &args![]).f32();
                    if tile_get_rendered_menu(e, this) {
                        width = e.call(RENDERED_MENU_WIDTH, &args![]).f32();
                        height = e.call(RENDERED_MENU_HEIGHT, &args![]).f32();
                    }
                    let left_limited = if 0.0 < left { left } else { 0.0 };
                    let x0 = float_to_int(e, left_limited);
                    let top_limited = if 0.0 < top { top } else { 0.0 };
                    let y0 = float_to_int(e, top_limited);
                    let right_limited = if right < width { right } else { width };
                    let x1 = float_to_int(e, right_limited);
                    let bottom_limited = if bottom < height { bottom } else { height };
                    let y1 = float_to_int(e, bottom_limited);
                    e.mem.set_i32(property + 0x9c, x0);
                    e.mem.set_i32(property + 0xa0, y0);
                    e.mem.set_i32(property + 0xa4, x1);
                    e.mem.set_i32(property + 0xa8, y1);
                    e.call(SHADER_PROPERTY_REFRESH, &args![property, 1u32]);
                }
                index += 1;
            }
        }
        tile_re_clip_children(e, child, left, top, right, bottom);
    }
}

// Translated from 00a07c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::AddFadeControl` (Xbox PDB): starts fading trait `trait_id` of the
/// tile from `from` to `to` over `seconds` seconds with fade type
/// `fade_type`. Nothing when `from == to` or `seconds` is not above 0.
/// Replaces the tile's running fade of that trait ([`tile_remove_fade_control`]),
/// stamps the start time (`GetTickCount`) and puts the new control at the
/// head of the global fade list. Type 3 (`TFCT_BLINK_FAST_FADE`) first
/// zeroes the user trait `_FlashCount`, sets `_TotalFlashCount` to
/// `seconds / 0.5 - 2` and makes the duration `seconds / (seconds / 0.5)`
/// seconds long; type 2 (`TFCT_BLINK_THRICE`) zeroes `_FlashCount`.
pub fn tile_add_fade_control(
    e: &mut Engine,
    this: Ptr<Tile>,
    trait_id: i32,
    from: f32,
    to: f32,
    seconds: f32,
    fade_type: i32,
) {
    if from == to || not_greater(seconds as f64, e.global::<f64>(ZERO)) {
        return;
    }
    tile_remove_fade_control(e, this, trait_id);
    let control: Ptr<FadeControl> = allocate(e, 0x1c).cast();
    e.set(control, FadeControl::pParent, this);
    e.set(control, FadeControl::iTrait, trait_id);
    e.set(control, FadeControl::fStartValue, from);
    e.set(control, FadeControl::fEndValue, to);
    let millis = (seconds as f64 * e.global::<f64>(THOUSAND)) as f32;
    e.set(control, FadeControl::fDurationMillis, millis);
    let now = tick_count(e);
    e.set(control, FadeControl::uiStartTime, now);
    e.set(control, FadeControl::eFadeType, fade_type);
    if fade_type == FADE_BLINK_FAST_FADE {
        let flashes = (seconds as f64 / e.global::<f64>(HALF)) as f32;
        let flash_count = tile_add_user_trait(e, Ptr::new(FLASH_COUNT_NAME), -1);
        fn_00a012d0(e, this, flash_count, 0.0, true);
        let total = (flashes as f64 - e.global::<f64>(TWO)) as f32;
        let total_count = tile_add_user_trait(e, Ptr::new(TOTAL_FLASH_COUNT_NAME), -1);
        fn_00a012d0(e, this, total_count, total, true);
        let millis = (seconds as f64 / flashes as f64 * e.global::<f64>(THOUSAND)) as f32;
        e.set(control, FadeControl::fDurationMillis, millis);
    }
    if fade_type == FADE_BLINK_THRICE {
        let flash_count = tile_add_user_trait(e, Ptr::new(FLASH_COUNT_NAME), -1);
        fn_00a012d0(e, this, flash_count, 0.0, true);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), control.addr());
        e.call(FADE_LIST_ADD_HEAD, &args![FADE_CONTROLS_LIST, slot]);
    });
}

// Translated from 00a07dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::RemoveFadeControl` (Xbox PDB): removes from the global fade list
/// (and frees) every fade control of this tile for trait `trait_id`, or for
/// all traits when `trait_id` is `eNone` (`0x80000000`). The matches are
/// first collected in a temporary `BSSimpleList`, then removed one by one.
pub fn tile_remove_fade_control(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) {
    e.with_stack(8, |e, matches| {
        let mut node = FADE_CONTROLS_LIST;
        while node != 0 && e.mem.u32(node) != 0 {
            let control: Ptr<FadeControl> = Ptr::new(e.mem.u32(node));
            let control_trait = e.get(control, FadeControl::iTrait);
            if (control_trait == trait_id || trait_id == TRAIT_NONE)
                && e.get(control, FadeControl::pParent) == this
            {
                // The node itself serves as the pointer to the item (its
                // first word).
                e.call(FADE_LIST_ADD_HEAD, &args![matches, node]);
            }
            node = e.mem.u32(node + 4);
        }
        remove_listed_fade_controls(e, matches);
    });
}

/// Removes from the global fade list, and frees, every control on the
/// temporary list `matches`, then empties that list (twice: the game's
/// list clean-up runs once explicitly and once as the destructor).
fn remove_listed_fade_controls(e: &mut Engine, matches: Ptr) {
    let mut node = matches.addr();
    while node != 0 && e.mem.u32(node) != 0 {
        e.with_stack(4, |e, slot| {
            let control = e.mem.u32(node);
            e.mem.set_u32(slot.addr(), control);
            e.call(FADE_LIST_REMOVE, &args![FADE_CONTROLS_LIST, slot]);
            let control = e.mem.u32(slot.addr());
            deallocate(e, control);
        });
        node = e.mem.u32(node + 4);
    }
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![matches]);
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![matches]);
}

/// The controls of the global fade list in order (the nodes up to the first
/// one without an item).
fn fade_controls(e: &Engine) -> Vec<Ptr<FadeControl>> {
    let mut controls = vec![];
    let mut node = FADE_CONTROLS_LIST;
    while node != 0 && e.mem.u32(node) != 0 {
        controls.push(Ptr::new(e.mem.u32(node)));
        node = e.mem.u32(node + 4);
    }
    controls
}

/// The first fade control of `tile` for trait `trait_id`.
fn find_fade_control(e: &Engine, tile: Ptr<Tile>, trait_id: i32) -> Option<Ptr<FadeControl>> {
    fade_controls(e).into_iter().find(|&control| {
        e.get(control, FadeControl::iTrait) == trait_id
            && e.get(control, FadeControl::pParent) == tile
    })
}

/// `GetTickCount` through its import slot.
fn tick_count(e: &mut Engine) -> u32 {
    e.call(TICK_COUNT_IMPORT, &args![]).u32()
}

// Translated from 00a07ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::HasFadeControl` (Xbox PDB): whether the global fade list holds a
/// control of this tile for trait `trait_id`.
pub fn tile_has_fade_control(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> bool {
    find_fade_control(e, this, trait_id).is_some()
}

// Translated from 00a07f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetFadeEndFor` (Xbox PDB): the end value of the tile's fade of
/// trait `trait_id` truncated to an integer (the last matching control of
/// the list counts), else the trait's current value. A truncated end value
/// of exactly -1 also gives the current value.
pub fn tile_get_fade_end_for(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> f32 {
    let mut end: i32 = -1;
    for control in fade_controls(e) {
        if e.get(control, FadeControl::iTrait) == trait_id
            && e.get(control, FadeControl::pParent) == this
        {
            let to = e.get(control, FadeControl::fEndValue);
            end = float_to_int(e, to);
        }
    }
    if end as f64 == e.global::<f64>(MINUS_ONE_DOUBLE) {
        fn_00a011b0(e, this, trait_id)
    } else {
        end as f32
    }
}

// Translated from 00a07fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetFadeFinished` (Xbox PDB): true when the tile has no fade of
/// trait `trait_id`; false for a fade of type 1
/// (`TFCT_NONLINEAR_LONG_DARK_REPEATING`); else whether the elapsed time
/// has reached the duration.
pub fn tile_get_fade_finished(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> bool {
    let Some(control) = find_fade_control(e, this, trait_id) else {
        return true;
    };
    if e.get(control, FadeControl::eFadeType) == FADE_NONLINEAR_LONG_DARK_REPEATING {
        return false;
    }
    let now = tick_count(e);
    let elapsed = now.wrapping_sub(e.get(control, FadeControl::uiStartTime));
    let duration = e.get(control, FadeControl::fDurationMillis);
    let ratio = elapsed as f64 / duration as f64;
    ratio >= e.global::<f64>(ONE_DOUBLE)
}

// Translated from 00a08070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetFadeType` (Xbox PDB): the fade type of the tile's fade of
/// trait `trait_id`, 0 when there is none.
pub fn tile_get_fade_type(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) -> i32 {
    match find_fade_control(e, this, trait_id) {
        Some(control) => e.get(control, FadeControl::eFadeType),
        None => 0,
    }
}

// Translated from 00a080d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::UpdateFadeControls` (Xbox PDB): advances every fade control of the
/// global list by the time that has passed and sets the faded trait; the
/// controls that reached their end are collected on a temporary list and
/// then removed and freed. By fade type (`TILE_FADE_CONTROL_TYPE`):
///
/// * 0 standard: linear from start to end; a 3D tile's `alpha` also fades
///   the model ([`tile_fade_in_3d`], scaled by 1/255);
/// * 1, 2, 3 (blinks): the progress runs from 0 to 1 and back; at the end
///   of a cycle the start time is renewed (type 1 always; type 2 while the
///   flash counter is below 3; type 3 while it is below the total) and for
///   types 2 and 3 the flash counter (user trait `_FlashCount`) grows by
///   one; type 2 finishes by moving to the end value after the third
///   flash, type 3 by returning to the start value after the total;
/// * 4 (fade in, hold, fade out): rises over the first 0.16666 of the time,
///   holds the end value up to 0.83334 and falls back to the start value.
pub fn tile_update_fade_controls(e: &mut Engine) {
    if e.mem.u32(FADE_CONTROLS_LIST + 4) == 0 && e.mem.u32(FADE_CONTROLS_LIST) == 0 {
        return;
    }
    e.with_stack(8, |e, finished| {
        let mut node = FADE_CONTROLS_LIST;
        while node != 0 && e.mem.u32(node) != 0 {
            let control: Ptr<FadeControl> = Ptr::new(e.mem.u32(node));
            update_fade_control(e, node, control, finished);
            node = e.mem.u32(node + 4);
        }
        remove_listed_fade_controls(e, finished);
    });
}

/// One control of [`tile_update_fade_controls`]; `node` is its node in the
/// global list, `finished` the temporary list of the controls that ended.
fn update_fade_control(e: &mut Engine, node: u32, control: Ptr<FadeControl>, finished: Ptr) {
    let end = e.get(control, FadeControl::fEndValue);
    let start = e.get(control, FadeControl::fStartValue);
    let trait_id = e.get(control, FadeControl::iTrait);
    let tile = e.get(control, FadeControl::pParent);
    let fade_type = e.get(control, FadeControl::eFadeType);
    // `(end - start) * progress + start`, all in the FPU's precision.
    let mix = |progress: f32| (end as f64 - start as f64) * progress as f64 + start as f64;
    let progress_now = |e: &mut Engine| {
        let now = tick_count(e);
        let elapsed = now.wrapping_sub(e.get(control, FadeControl::uiStartTime));
        let duration = e.get(control, FadeControl::fDurationMillis);
        (elapsed as f64 / duration as f64) as f32
    };
    match fade_type {
        FADE_STANDARD => {
            let mut progress = progress_now(e);
            if not_less(progress as f64, 1.0) {
                progress = 1.0;
            }
            if tile_type(e, tile) == TYPE_3D && trait_id == TRAIT_ALPHA {
                let model = e.get(tile, Tile::spModel);
                let alpha = (mix(progress) / e.global::<f64>(TWO_FIFTY_FIVE_DOUBLE)) as f32;
                tile_fade_in_3d(e, tile, model, alpha);
            }
            fn_00a012d0(e, tile, trait_id, mix(progress) as f32, true);
            if progress >= 1.0 {
                e.call(FADE_LIST_ADD_HEAD, &args![finished, node]);
            }
        }
        FADE_NONLINEAR_LONG_DARK_REPEATING | FADE_BLINK_THRICE | FADE_BLINK_FAST_FADE => {
            let flash_id = tile_text_to_trait(e, Ptr::new(FLASH_COUNT_NAME));
            let flash = fn_00a011b0(e, tile, flash_id);
            let total_id = tile_text_to_trait(e, Ptr::new(TOTAL_FLASH_COUNT_NAME));
            let total = fn_00a011b0(e, tile, total_id);
            let mut progress = progress_now(e);
            let three = e.global::<f64>(THREE_DOUBLE);
            if progress >= 1.0 {
                if (fade_type == FADE_BLINK_THRICE && (flash as f64) < three)
                    || (fade_type == FADE_BLINK_FAST_FADE && flash < total)
                    || fade_type == FADE_NONLINEAR_LONG_DARK_REPEATING
                {
                    let now = tick_count(e);
                    e.set(control, FadeControl::uiStartTime, now);
                    progress = 0.0;
                }
                if fade_type == FADE_BLINK_THRICE || fade_type == FADE_BLINK_FAST_FADE {
                    let next = (flash as f64 + e.global::<f64>(ONE_DOUBLE)) as f32;
                    let flash_id = tile_text_to_trait(e, Ptr::new(FLASH_COUNT_NAME));
                    fn_00a012d0(e, tile, flash_id, next, true);
                    if fade_type == FADE_BLINK_THRICE && flash as f64 > three {
                        fn_00a012d0(e, tile, trait_id, end, true);
                    } else if fade_type == FADE_BLINK_FAST_FADE && total < flash {
                        fn_00a012d0(e, tile, trait_id, start, true);
                        e.call(FADE_LIST_ADD_HEAD, &args![finished, node]);
                    }
                }
            }
            if fade_type == FADE_BLINK_THRICE && flash as f64 >= three {
                let value = mix(progress) as f32;
                let limited = if end < value { end } else { value };
                fn_00a012d0(e, tile, trait_id, limited, true);
                if end <= value {
                    e.call(FADE_LIST_ADD_HEAD, &args![finished, node]);
                }
            } else {
                let swing =
                    (progress as f64 * e.global::<f64>(TWO) - e.global::<f64>(ONE_DOUBLE)) as f32;
                let magnitude = e.call(FABS, &args![swing as f64]).f64() as f32;
                let weight = (e.global::<f64>(ONE_DOUBLE) - magnitude as f64) as f32;
                fn_00a012d0(e, tile, trait_id, mix(weight) as f32, true);
            }
        }
        FADE_IN_HOLD_FADE_OUT => {
            let progress = progress_now(e);
            if (progress as f64) < e.global::<f64>(FADE_RISE_END_DOUBLE) {
                let rise = e.global::<f32>(FADE_RISE_END_FLOAT);
                let ratio = (progress as f64 - 0.0) / (rise as f64 - 0.0);
                let value = (start as f64 + (end as f64 - start as f64) * ratio) as f32;
                fn_00a012d0(e, tile, trait_id, value, true);
            } else if (progress as f64) < e.global::<f64>(FADE_FALL_START_DOUBLE) {
                fn_00a012d0(e, tile, trait_id, end, true);
            } else if progress >= 1.0 {
                fn_00a012d0(e, tile, trait_id, start, true);
                e.call(FADE_LIST_ADD_HEAD, &args![finished, node]);
            } else {
                let fall = e.global::<f32>(FADE_FALL_START_FLOAT) as f64;
                let ratio = (progress as f64 - fall) / (e.global::<f64>(ONE_DOUBLE) - fall);
                let value = (end as f64 + (start as f64 - end as f64) * ratio) as f32;
                fn_00a012d0(e, tile, trait_id, value, true);
            }
        }
        _ => {}
    }
}

// Translated from 00a08720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::FadeIn3D` (Xbox PDB): sets the alpha `alpha` of the shader
/// property (property type 3) of the node and of every geometry among its
/// descendants. The tile is not used. Virtual calls on the node: slot +0xC
/// (`IsNode`: answers the `NiNode` or null) and +0x18 (`IsGeometry`).
pub fn tile_fade_in_3d(e: &mut Engine, _this: Ptr<Tile>, node: Ptr, alpha: f32) {
    if node.is_null() {
        return;
    }
    let as_node = e.vcall(node.addr(), 0xc, &[]).u32();
    let mut index = 0;
    while as_node != 0 && index < e.mem.u16(as_node + 0xa6) as u32 {
        let child = e.mem.u32(e.mem.u32(as_node + 0xa0) + index * 4);
        tile_fade_in_3d(e, _this, Ptr::new(child), alpha);
        index += 1;
    }
    if e.vcall(node.addr(), 0x18, &[]).u32() != 0 {
        let property = e
            .call(NI_OBJECT_GET_PROPERTY, &args![node.addr(), 3u32])
            .u32();
        if property != 0 {
            e.mem.set_f32(property + 0x78, alpha);
        }
    }
}

// Translated from 00a087d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SetParent` (Xbox PDB): moves the tile under `parent` (null:
/// detaches it). The old parent, unless it is released, loses one from its
/// child count trait (`eChildCount`) and drops the tile from its children
/// list. The new parent gains one and gets the tile in its children list:
/// right behind the child `after` when that is among them (the tile is
/// first taken out of the list), else at the head of the list.
pub fn tile_set_parent(e: &mut Engine, this: Ptr<Tile>, parent: Ptr<Tile>, after: Ptr<Tile>) {
    let old_parent = e.get(this, Tile::pParent);
    if !old_parent.is_null() && e.get(old_parent, Tile::uiFlags) & FLAG_RELEASED == 0 {
        let count = fn_00a011b0(e, old_parent, TRAIT_CHILD_COUNT);
        let count = (count as f64 - e.global::<f64>(ONE_DOUBLE)) as f32;
        fn_00a012d0(e, old_parent, TRAIT_CHILD_COUNT, count, true);
        remove_from_list(e, old_parent.addr() + 4, this);
    }
    e.set(this, Tile::pParent, parent);
    if parent.is_null() {
        return;
    }
    let count = fn_00a011b0(e, parent, TRAIT_CHILD_COUNT);
    let count = (count as f64 + e.global::<f64>(ONE_DOUBLE)) as f32;
    fn_00a012d0(e, parent, TRAIT_CHILD_COUNT, count, true);
    let children = parent.addr() + 4;
    let mut placed = false;
    if !after.is_null() {
        let mut node = e.mem.u32(children);
        while node != 0 {
            let behind = node;
            let element = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            if element == after.addr() {
                remove_from_list(e, children, this);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), this.addr());
                    e.call(LIST_ADD_AFTER, &args![children, behind, slot]);
                });
                placed = true;
                break;
            }
        }
    }
    if !placed {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), this.addr());
            e.call(LIST_ADD_HEAD, &args![children, slot]);
        });
    }
}

// Translated from 00a089c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetParentModel` (Xbox PDB): the model of the tile, or of its
/// nearest ancestor that has one; null when none has.
pub fn tile_get_parent_model(e: &mut Engine, this: Ptr<Tile>) -> Ptr {
    let mut tile = this;
    while !tile.is_null() && e.get(tile, Tile::spModel).is_null() {
        tile = e.get(tile, Tile::pParent);
    }
    if tile.is_null() || e.get(tile, Tile::spModel).is_null() {
        Ptr::NULL
    } else {
        e.get(tile, Tile::spModel)
    }
}

// Translated from 00a08a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::DeleteModel` (Xbox PDB): nothing without a model. Otherwise
/// severs the tile's extra data from the model, has the model's parent
/// node remove it (slot +0xE8, `DetachChild`), and releases the tile's
/// reference to it. A local reference taken for the duration keeps the
/// model alive until the end, where it is released too (and the model is
/// destroyed through its virtual slot +4 if that was the last).
pub fn tile_delete_model(e: &mut Engine, this: Ptr<Tile>) {
    let model = e.get(this, Tile::spModel);
    if model.is_null() {
        return;
    }
    tile_sever_extra_data(e, this);
    let parent_node = e.mem.u32(model.addr() + 0x18);
    if parent_node != 0 {
        e.vcall(parent_node, 0xe8, &args![model]);
    }
    // The local `NiPointer` copy of the model.
    e.call(INTERLOCKED_INCREMENT, &args![model.addr() + 4]);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), model.addr());
        e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x2c, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![local, 0u32]);
        let held = e.mem.u32(local.addr());
        if held != 0 {
            release_reference(e, held);
        }
    });
}

// Translated from 00a08b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. Finds the tile an action link names: `path` is split
/// by [`fn_00a018d0`] into a word (looked up in the text table; the code
/// selects the rule) and an optional name in parentheses. The rules, by
/// code:
///
/// * `0x1389` the parent of `tile`; `0x138a` `tile` itself;
/// * `0x138c` the next sibling (the first child after the last) when the
///   name is empty, else the first sibling named like it (case-blind);
/// * `0x138d` the first child when the name is empty, else the child
///   found by name ([`tile_get_child_by_name`]);
/// * `0x138e` the menus root; `0x138f` the result of `00706cf0`;
/// * `0x1390` the word at +4 of the tile's menu ([`tile_get_menu`]);
/// * `0x1391` the grandparent;
/// * anything else (`0x138b` among it) a depth-first search by name
///   ([`fn_00a08f20`]) under the menu tile of `tile` (or the menus root
///   for a null `tile`).
///
/// The tile is dereferenced without a null check in most rules, as in the
/// game.
pub fn fn_00a08b20(e: &mut Engine, tile: Ptr<Tile>, path: Ptr) -> Ptr {
    e.with_stack(NAME_BUFFER_SIZE, |e, buffer| {
        e.mem.set_u8(buffer.addr(), 0);
        let code = fn_00a018d0(e, path, buffer);
        let named = e.mem.i8(buffer.addr()) != 0;
        match code {
            LINK_PARENT => e.get(tile, Tile::pParent).cast(),
            LINK_SELF => tile.cast(),
            LINK_SIBLING => {
                let parent = e.get(tile, Tile::pParent);
                let sibling_count = e.mem.u32(parent.addr() + 0xc);
                if !named && sibling_count != 0 {
                    let mut node = e.mem.u32(parent.addr() + 4);
                    while node != 0 {
                        let element = e.mem.u32(node + 8);
                        node = e.mem.u32(node);
                        if element == tile.addr() {
                            break;
                        }
                    }
                    if node != 0 {
                        Ptr::new(e.mem.u32(node + 8))
                    } else {
                        Ptr::new(e.mem.u32(e.mem.u32(parent.addr() + 4) + 8))
                    }
                } else if named && sibling_count != 0 {
                    let mut node = e.mem.u32(parent.addr() + 4);
                    let mut found = Ptr::NULL;
                    while node != 0 {
                        let sibling = e.mem.u32(node + 8);
                        node = e.mem.u32(node);
                        let sibling_name = e.mem.u32(sibling + 0x20);
                        if sibling_name != 0
                            && e.call(STRICMP, &args![sibling_name, buffer]).i32() == 0
                        {
                            found = Ptr::new(sibling);
                            break;
                        }
                    }
                    found
                } else {
                    Ptr::NULL
                }
            }
            LINK_CHILD => {
                if !named && e.mem.u32(tile.addr() + 0xc) != 0 {
                    return Ptr::new(e.mem.u32(e.mem.u32(tile.addr() + 4) + 8));
                }
                if !tile.is_null() && named && e.mem.u32(tile.addr() + 0xc) != 0 {
                    tile_get_child_by_name(e, tile, buffer).cast()
                } else {
                    Ptr::NULL
                }
            }
            LINK_MENUS_ROOT => e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr(),
            LINK_00706CF0 => e.call(INTERFACE_00706CF0, &args![]).ptr(),
            LINK_MENU => {
                let menu = tile_get_menu(e, tile);
                Ptr::new(e.mem.u32(menu.addr() + 4))
            }
            LINK_GRANDPARENT => {
                let parent = e.get(tile, Tile::pParent);
                if parent.is_null() {
                    Ptr::NULL
                } else {
                    e.get(parent, Tile::pParent).cast()
                }
            }
            _ => {
                let start = if tile.is_null() {
                    e.call(INTERFACE_GET_MENUS_ROOT, &args![]).ptr()
                } else {
                    tile_get_menu_tile(e, tile)
                };
                fn_00a08f20(e, start, path).cast()
            }
        }
    })
}

// Translated from 00a08f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name, `cdecl(tile, name)`: the first tile named `name`
/// (case-blind `_stricmp`) in the tree under and including `tile`, searched
/// depth first; null when there is none.
pub fn fn_00a08f20(e: &mut Engine, tile: Ptr<Tile>, name: Ptr) -> Ptr<Tile> {
    if tile.is_null() {
        return Ptr::NULL;
    }
    let tile_name = e.mem.u32(tile.addr() + 0x20);
    if tile_name != 0 && e.call(STRICMP, &args![tile_name, name]).i32() == 0 {
        return tile;
    }
    let mut node = e.get(tile.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let found = fn_00a08f20(e, child, name);
        if !found.is_null() {
            return found;
        }
    }
    Ptr::NULL
}

// Translated from 00a08fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name, `cdecl(tile, id)`: the first tile whose `id` trait
/// equals `id` in the tree under and including `tile`, depth first; null
/// when there is none. A null `tile` is not special-cased by the game: it
/// goes on to read the children list at address 4 (this translation reads
/// the same address).
pub fn fn_00a08fb0(e: &mut Engine, tile: Ptr<Tile>, id: i32) -> Ptr<Tile> {
    if !tile.is_null() {
        let tile_id = fn_00a011b0(e, tile, TRAIT_ID);
        if id as f64 == tile_id as f64 {
            return tile;
        }
    }
    let mut node = e.mem.u32(tile.addr() + 4);
    while node != 0 {
        let child: Ptr<Tile> = Ptr::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        let found = fn_00a08fb0(e, child, id);
        if !found.is_null() {
            return found;
        }
    }
    Ptr::NULL
}

// Translated from 00a09030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetMenuByClass` (Xbox PDB), `cdecl(class)`: the menu registered
/// for the menu class number `class` (1001 to 1084) in the table at
/// `[011f350c]` (indexed from 1001, `[011f3512]` entries long), else 0.
pub fn tile_get_menu_by_class(e: &mut Engine, class: i32) -> u32 {
    if !(MENU_CLASS_FIRST..=MENU_CLASS_LAST).contains(&class) {
        return 0;
    }
    let index = (class - MENU_CLASS_FIRST) as u32;
    if e.global::<u16>(MENU_CLASS_COUNT) as u32 <= index {
        return 0;
    }
    let table = e.global::<u32>(MENU_CLASS_TABLE);
    e.mem.u32(table + index * 4)
}

/// Appends the action `node` at the end of the action list of `value`.
fn append_action(e: &mut Engine, value: Ptr<TileValue>, node: u32) {
    let mut tail = e.get(value, TileValue::pActionListA).addr();
    if tail == 0 {
        e.set(value, TileValue::pActionListA, Ptr::new(node));
        return;
    }
    while e.mem.u32(tail + 8) != 0 {
        tail = e.mem.u32(tail + 8);
    }
    e.mem.set_u32(tail + 8, node);
}

// Translated from 00a09080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::AddAction` (Xbox PDB): appends a `Tile::FloatAction` (vtable
/// `01094c2c`, `QFloat`) of type `action` (a `VALUE_ACTION`) with the
/// operand `amount` to the trait's action list. The `Tile::Action`
/// constructor first writes its own vtable and the default type
/// `VA_COPY` (2000); both are overwritten at once.
pub fn value_add_action(e: &mut Engine, this: Ptr<TileValue>, action: i32, amount: f32) {
    let node: Ptr<ValueAction> = allocate(e, 0x10).cast();
    e.mem.set_u32(node.addr(), VTABLE_FLOAT_ACTION);
    e.set(node, ValueAction::eActionType, action);
    e.set(node, ValueAction::pnext, Ptr::NULL);
    e.set(node, ValueAction::fValue, amount);
    append_action(e, this, node.addr());
}

// Translated from 00a09130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::AddAction_ov2` (Xbox PDB): appends a `Tile::RefValueAction`
/// (vtable `01094c44`, `QRefValue`) of type `action` that reads trait
/// `trait_id` of `tile` (created when missing), and registers the trait
/// with `AddReaction(value, this)` so it is recalculated when that trait
/// changes.
pub fn value_add_action_ov2(
    e: &mut Engine,
    this: Ptr<TileValue>,
    action: i32,
    tile: Ptr<Tile>,
    trait_id: i32,
) {
    let node: Ptr<ValueAction> = allocate(e, 0x10).cast();
    e.mem.set_u32(node.addr(), VTABLE_REF_VALUE_ACTION);
    e.set(node, ValueAction::eActionType, action);
    e.set(node, ValueAction::pnext, Ptr::NULL);
    let referenced = tile_get_or_create_value(e, tile, trait_id);
    e.mem.set_u32(node.addr() + 0xc, referenced.addr());
    append_action(e, this, node.addr());
    e.call(ADD_REACTION, &args![referenced, this]);
}

// Translated from 00a09200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::ClearActions` (Xbox PDB): deletes the trait's actions one
/// by one. An action that reads another trait (`QRefValue`, slot +4,
/// answers it) is first removed from that trait's reaction list (the map
/// `ValueReactionList`, `011f3358`): the reactions of this trait come out
/// of the list, and the map entry is erased when none are left or updated
/// when the head changed.
pub fn value_clear_actions(e: &mut Engine, this: Ptr<TileValue>) {
    let list_head = this.addr() + 0x10;
    loop {
        let action = e.mem.u32(list_head);
        if action == 0 {
            break;
        }
        if e.vcall(action, 4, &[]).u32() != 0 {
            e.with_stack(4, |e, reactions| {
                e.mem.set_u32(reactions.addr(), 0);
                let referenced = e.vcall(action, 4, &[]).u32();
                let found = e
                    .call(
                        REACTION_MAP_FIND,
                        &args![REACTION_MAP, referenced, reactions],
                    )
                    .u8();
                if found != 0 {
                    let first_before = e.mem.u32(reactions.addr());
                    let mut link = reactions.addr();
                    loop {
                        let reaction = e.mem.u32(link);
                        if reaction == 0 {
                            break;
                        }
                        if e.mem.u32(reaction) == this.addr() {
                            let next = e.mem.u32(reaction + 4);
                            e.mem.set_u32(link, next);
                            deallocate(e, reaction);
                        } else {
                            link = reaction + 4;
                        }
                    }
                    let first_after = e.mem.u32(reactions.addr());
                    if first_after == 0 {
                        let referenced = e.vcall(action, 4, &[]).u32();
                        e.call(REACTION_MAP_REMOVE, &args![REACTION_MAP, referenced]);
                    } else if first_after != first_before {
                        let referenced = e.vcall(action, 4, &[]).u32();
                        e.call(
                            REACTION_MAP_SET,
                            &args![REACTION_MAP, referenced, first_after],
                        );
                    }
                }
            });
        }
        let next = e.mem.u32(action + 8);
        e.mem.set_u32(list_head, next);
        deallocate(e, action);
    }
}

// Translated from 00a09330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::~Value` (Xbox PDB): clears the trait's actions
/// ([`value_clear_actions`]); then, if the trait has a reaction list in the
/// map `ValueReactionList`, goes through its reactions, blanks the
/// referenced trait of every action of theirs that reads this trait, frees
/// the reaction nodes and erases the map entry. Frees the string and zeroes
/// the float and the owning tile. (The value itself is not freed.)
pub fn value_destructor(e: &mut Engine, this: Ptr<TileValue>) {
    value_clear_actions(e, this);
    e.with_stack(4, |e, reactions| {
        e.mem.set_u32(reactions.addr(), 0);
        let found = e
            .call(REACTION_MAP_FIND, &args![REACTION_MAP, this, reactions])
            .u8();
        if found != 0 {
            let mut reaction = e.mem.u32(reactions.addr());
            while reaction != 0 {
                let owner = e.mem.u32(reaction);
                let mut action = e.mem.u32(owner + 0x10);
                while action != 0 {
                    if e.vcall(action, 4, &[]).u32() == this.addr() {
                        e.mem.set_u32(action + 0xc, 0);
                    }
                    action = e.mem.u32(action + 8);
                }
                let node = reaction;
                reaction = e.mem.u32(reaction + 4);
                deallocate(e, node);
            }
            e.call(REACTION_MAP_REMOVE, &args![REACTION_MAP, this]);
        }
    });
    let text = e.get(this, TileValue::strValue);
    if !text.is_null() {
        deallocate(e, text.addr());
    }
    e.set(this, TileValue::fValue, 0.0);
    e.set(this, TileValue::pParent, Ptr::NULL);
}

// Translated from 00a09410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::CalculateValue` (Xbox PDB), `thiscall(flag)`: recomputes
/// the trait's float and string by running its action list. Nothing is done
/// when the owning tile is missing or deleting (`ebfMenuDeleting`), nor
/// (except for the `class` trait) while the tile is loading
/// (`ebfTileLoading`). The current-line word of the TLS block is 13 during
/// the calculation.
///
/// The value starts from the float it has; the actions run in order, each
/// with an operand (its `QFloat`; for a reference action `QRefValue` also
/// answers the other trait, whose string is taken too; a user trait whose
/// text-table number is -1 is looked up by [`GetUnderscoreValue`]
/// `00a0a0b0` for the current value rounded). Parentheses use a stack of
/// floats: a left parenthesis (`VA_LEFT_PAREN`) pushes the running value
/// and restarts from 0, a right parenthesis (`VA_RIGHT_PAREN`) pops it back
/// as the running value and applies the operation its own `QFloat`
/// encodes to the parenthesised result. Operations (`VALUE_ACTION`):
/// `VA_COPY` copies the float, or the string when there is one (which marks
/// the value changed unless the text is the same), `ADD`, `SUB`, `MULT`,
/// `DIV` (not by 0), `MIN`, `MAX`, `MOD` (on the rounded integers, not by
/// 0), `FLOOR` and `CEIL` of the sum, `ABS` of the sum, `ROUND` to a
/// multiple of the operand, the comparisons `GT`, `GTE`, `EQ`, `NEQ`,
/// `LT`, `LTE` (0 or 1), `AND`, `OR`, `NOT` (a string counts as true),
/// `ONLYIF` and `ONLYIFNOT` (set 0).
///
/// Afterwards the value counts as changed when it has no string and the
/// float differs from the starting one; the string of an `eString` trait is
/// rewritten as the decimal rounded integer when the float changed. When
/// the value changed (or `flag`), `ValueChangeEvent(value)` `00a0a220` is
/// called and, if the tile is not released, the tile's virtual `PostParse`
/// (slot +0x18) is asked about (trait, float, string); an answer of 0
/// leads to `Tile::FinalPostParse`.
pub fn value_calculate_value(e: &mut Engine, this: Ptr<TileValue>, flag: bool) {
    let tile = e.get(this, TileValue::pParent);
    let trait_id = e.get(this, TileValue::eIndex);
    // The first test reads the tile without a null check, as the game does.
    let tile_flags = e.mem.u32(tile.addr() + 0x30);
    if !(tile_flags & FLAG_TILE_LOADING == 0 || trait_id == TRAIT_CLASS) {
        return;
    }
    if tile.is_null() || e.get(tile, Tile::uiFlags) & FLAG_MENU_DELETING != 0 {
        return;
    }
    let line_word = e.tls() + TLS_CURRENT_LINE;
    let saved_line = e.mem.u32(line_word);
    e.mem.set_u32(line_word, 0xd);
    let mut action = e.get(this, TileValue::pActionListA).addr();
    let start = e.get(this, TileValue::fValue);
    let mut changed = false;
    e.with_stack(0x10, |e, stack| {
        // A stack of floats: vtable, buffer, size, capacity.
        e.mem.set_u32(stack.addr(), VTABLE_FLOAT_STACK);
        e.call(FLOAT_STACK_CONSTRUCT, &args![stack, 0u32, 0u32]);
        while action != 0 {
            run_action(e, this, action, stack, &mut changed);
            action = e.mem.u32(action + 8);
        }
        if e.get(this, TileValue::strValue).is_null() {
            changed = e.get(this, TileValue::fValue) != start;
        }
        let float = e.get(this, TileValue::fValue);
        if float != start && trait_id == TRAIT_STRING {
            let number = float_to_int_rounded(float);
            // The number of characters `%d` needs.
            let mut digits = (number == 0) as u32;
            let mut rest = number;
            while rest != 0 {
                digits += 1;
                rest /= 10;
            }
            if number < 0 {
                digits += 1;
            }
            let text = e.get(this, TileValue::strValue);
            if text.is_null() || e.call(STRLEN, &args![text]).u32() != digits {
                let old = e.get(this, TileValue::strValue).addr();
                deallocate(e, old);
                let new_text = allocate(e, digits + 1);
                e.set(this, TileValue::strValue, new_text);
            }
            let text = e.get(this, TileValue::strValue);
            e.call(SPRINTF_S, &args![text, digits + 1, FORMAT_INTEGER, number]);
            changed = true;
        }
        if changed || flag {
            e.call(VALUE_CHANGE_EVENT, &args![this]);
            let tile = e.get(this, TileValue::pParent);
            if !tile.is_null() && e.get(tile, Tile::uiFlags) & FLAG_RELEASED == 0 {
                let value = e.get(this, TileValue::fValue);
                let text = e.get(this, TileValue::strValue);
                let handled = e.vcall(tile.addr(), 0x18, &args![trait_id, value, text]);
                if handled.u32() == 0 {
                    tile_final_post_parse(e, tile, trait_id, value, text.addr());
                }
            }
        }
        e.call(FLOAT_STACK_DESTROY, &args![stack]);
    });
    e.mem.set_u32(line_word, saved_line);
}

/// One action of [`value_calculate_value`]; `stack` is its stack of floats
/// and `changed` its "the string changed" flag.
fn run_action(e: &mut Engine, this: Ptr<TileValue>, action: u32, stack: Ptr, changed: &mut bool) {
    let mut operand = e.vcall(action, 0, &[]).f32();
    let mut text = 0u32;
    if e.vcall(action, 4, &[]).u32() != 0 {
        let referenced = e.vcall(action, 4, &[]).u32();
        let mut found_number: i32 = 0;
        text = e.mem.u32(referenced + 0xc);
        let referenced_trait = e.mem.i32(referenced);
        if referenced_trait >= FIRST_USER_TEXT_ID {
            e.with_stack(4, |e, out| {
                e.call(
                    REACTION_MAP_FIND,
                    &args![TRAIT_EXTRA_DATA_MAP, referenced_trait, out],
                );
                found_number = e.mem.i32(out.addr());
            });
        }
        if found_number == -1 {
            let rounded = float_to_int_rounded(e.get(this, TileValue::fValue));
            let owner = e.mem.u32(referenced + 4);
            let found = e
                .call(
                    TILE_GET_UNDERSCORE_VALUE,
                    &args![owner, referenced_trait, rounded],
                )
                .u32();
            if found != 0 {
                operand = e.mem.f32(found + 8);
                text = e.mem.u32(found + 0xc);
            }
        }
    }
    let mut kind = e.mem.i32(action + 4);
    if kind == ACTION_LEFT_PAREN {
        let value_address = this.addr() + 8;
        e.call(FLOAT_STACK_PUSH, &args![stack, value_address]);
        e.set(this, TileValue::fValue, 0.0);
    } else if kind == ACTION_RIGHT_PAREN {
        let size = e.mem.u32(stack.addr() + 8);
        let top = e.mem.f32(
            e.mem
                .u32(stack.addr() + 4)
                .wrapping_add(size.wrapping_mul(4))
                .wrapping_sub(4),
        );
        let encoded = e.vcall(action, 0, &[]).f32();
        kind = float_to_int_rounded(encoded);
        operand = e.get(this, TileValue::fValue);
        e.set(this, TileValue::fValue, top);
        if size != 0 {
            e.mem.set_u32(stack.addr() + 8, size - 1);
        }
    }
    let current = e.get(this, TileValue::fValue);
    let (current64, operand64) = (current as f64, operand as f64);
    let zero = e.global::<f64>(ZERO);
    let flag = |condition: bool| if condition { 1.0f32 } else { 0.0f32 };
    let result: Option<f32> = match kind {
        VA_COPY => {
            if text == 0 {
                Some(operand)
            } else {
                let length = e.call(STRLEN, &args![text]).u32();
                let old = e.get(this, TileValue::strValue);
                *changed = old.is_null() || e.call(STRCMP, &args![old, text]).i32() != 0;
                if !old.is_null() {
                    deallocate(e, old.addr());
                    e.set(this, TileValue::strValue, Ptr::NULL);
                }
                let copy = allocate(e, length + 1);
                e.set(this, TileValue::strValue, copy);
                e.call(STRCPY_S, &args![copy, length + 1, text]);
                None
            }
        }
        VA_ADD => Some((current64 + operand64) as f32),
        VA_SUB => Some((current64 - operand64) as f32),
        VA_MULT => Some((current64 * operand64) as f32),
        VA_DIV => {
            if operand64 != zero {
                Some((current64 / operand64) as f32)
            } else {
                None
            }
        }
        VA_MIN => Some(if current < operand { current } else { operand }),
        VA_MAX => Some(if operand < current { current } else { operand }),
        VA_MOD => {
            if operand64 != zero {
                let dividend = float_to_int_rounded(current);
                let divisor = float_to_int_rounded(operand);
                Some((dividend % divisor) as f32)
            } else {
                None
            }
        }
        VA_FLOOR => {
            let sum = (operand64 + current64) as f32;
            Some(e.call(FLOOR, &args![sum as f64]).f64() as f32)
        }
        VA_CEIL => {
            let sum = (operand64 + current64) as f32;
            Some(e.call(CEIL, &args![sum as f64]).f64() as f32)
        }
        VA_ABS => {
            let sum = (operand64 + current64) as f32;
            Some(e.call(FABS, &args![sum as f64]).f64() as f32)
        }
        VA_ROUND => {
            let quotient = (current64 / operand64) as f32;
            let truncated = float_to_int(e, quotient);
            let round_up =
                not_less(quotient as f64 - truncated as f64, e.global::<f64>(HALF)) as i32;
            let multiple = float_to_int(e, quotient).wrapping_add(round_up);
            Some((operand64 * multiple as f64) as f32)
        }
        VA_GT => Some(flag(operand < current)),
        VA_GTE => Some(flag(operand <= current)),
        VA_EQ => Some(flag(operand == current)),
        VA_NEQ => Some(flag(operand != current)),
        VA_LT => Some(flag(current < operand)),
        VA_LTE => Some(flag(current <= operand)),
        VA_AND => Some(flag(if text == 0 {
            current64 != zero && operand64 != zero
        } else {
            current64 != zero
        })),
        VA_OR => Some(flag(if text == 0 {
            current64 != zero || operand64 != zero
        } else {
            current64 != zero || text != 0
        })),
        VA_NOT => Some(if text == 0 {
            flag(operand64 == zero)
        } else {
            0.0
        }),
        VA_ONLYIF => {
            if operand64 == zero && text == 0 {
                Some(0.0)
            } else {
                None
            }
        }
        VA_ONLYIFNOT => {
            if operand64 != zero || text != 0 {
                Some(0.0)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(value) = result {
        e.set(this, TileValue::fValue, value);
    }
}

// ---------------------------------------------------------------------------
// Session 3: `Tile::GetUnderscoreValue` .. `NiExtraData::GetRTTI`

// Translated from 00a0a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetUnderscoreValue` (Xbox PDB): the value of the user trait whose
/// name is the text table's name for `name_id` followed by `number`
/// (`sprintf("%s%d")`, at most 0x104 bytes), looked up with
/// [`tile_text_to_trait`] and [`tile_get_value`]; null when there is none.
pub fn tile_get_underscore_value(
    e: &mut Engine,
    this: Ptr<Tile>,
    name_id: i32,
    number: i32,
) -> Ptr {
    let name = fn_00a01a70(e, name_id);
    e.with_stack(NAME_BUFFER_SIZE, |e, buffer| {
        e.call(
            SPRINTF_S,
            &args![
                buffer,
                NAME_BUFFER_SIZE,
                UNDERSCORE_NAME_FORMAT,
                name,
                number
            ],
        );
        let trait_id = tile_text_to_trait(e, buffer);
        tile_get_value(e, this, trait_id)
    })
}

// Translated from 00a0a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::AddReaction` (Xbox PDB, `cdecl`): records in the map
/// `ValueReactionList` that `owner` depends on `value`. A reaction node
/// (owner, next) is made; when `value` has no list yet the node becomes
/// its list, otherwise the node is added behind the last node of the list,
/// unless that last node already names `owner` (then the node is freed).
pub fn add_reaction(e: &mut Engine, value: Ptr<TileValue>, owner: Ptr<TileValue>) {
    let node: Ptr<Reaction> = allocate(e, 8).cast();
    e.set(node, Reaction::preactionValue, Ptr::NULL);
    e.set(node, Reaction::pnext, Ptr::NULL);
    e.set(node, Reaction::preactionValue, owner);
    e.with_stack(4, |e, list| {
        e.mem.set_u32(list.addr(), 0);
        let found = e
            .call(REACTION_MAP_FIND, &args![REACTION_MAP, value, list])
            .u8();
        if found == 0 {
            e.call(REACTION_MAP_SET, &args![REACTION_MAP, value, node]);
            return;
        }
        let mut last = e.mem.u32(list.addr());
        let mut names_owner = e.mem.u32(last) == owner.addr();
        while e.mem.u32(last + 4) != 0 {
            last = e.mem.u32(last + 4);
            names_owner = e.mem.u32(last) == owner.addr();
        }
        if names_owner {
            deallocate(e, node.addr());
        } else {
            e.mem.set_u32(last + 4, node.addr());
        }
    });
}

// Translated from 00a0a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ValueChangeEvent` (Xbox PDB, `cdecl`): recalculates, with
/// `CalculateValue(false)`, every value that has a reaction on `value`.
pub fn tile_value_change_event(e: &mut Engine, value: Ptr<TileValue>) {
    e.with_stack(4, |e, list| {
        e.mem.set_u32(list.addr(), 0);
        let found = e
            .call(REACTION_MAP_FIND, &args![REACTION_MAP, value, list])
            .u8();
        if found != 0 {
            let mut reaction = e.mem.u32(list.addr());
            while reaction != 0 {
                let dependent = Ptr::new(e.mem.u32(reaction));
                value_calculate_value(e, dependent, false);
                reaction = e.mem.u32(reaction + 4);
            }
        }
    });
}

// Translated from 00a0a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::SetFloat` (Xbox PDB): stores `value` as the trait's float
/// and drops its string; with `flag` the actions are cleared too. Then
/// `CalculateValue` runs, told whether anything changed (the float differs
/// or there was a string).
pub fn value_set_float(e: &mut Engine, this: Ptr<TileValue>, value: f32, flag: bool) {
    let old = e.get(this, TileValue::fValue);
    let text = e.get(this, TileValue::strValue);
    let unchanged = value == old && text.is_null();
    let changed = !unchanged;
    e.set(this, TileValue::fValue, value);
    if !text.is_null() {
        deallocate(e, text.addr());
        e.set(this, TileValue::strValue, Ptr::NULL);
    }
    if flag {
        value_clear_actions(e, this);
    }
    value_calculate_value(e, this, changed);
}

// Translated from 00a0a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Value::SetString` (Xbox PDB): replaces the trait's string by a
/// copy of `text` (none when `text` is null), zeroes the float and, with
/// `flag`, clears the actions. Then `CalculateValue` runs, told whether
/// the string changed (one is missing, or they differ).
pub fn value_set_string(e: &mut Engine, this: Ptr<TileValue>, text: Ptr, flag: bool) {
    let old = e.get(this, TileValue::strValue);
    let changed = if old.is_null() != text.is_null() {
        true
    } else if !old.is_null() {
        e.call(STRCMP, &args![old, text]).i32() != 0
    } else {
        false
    };
    if !old.is_null() {
        deallocate(e, old.addr());
        e.set(this, TileValue::strValue, Ptr::NULL);
    }
    if !text.is_null() {
        let length = e.call(STRLEN, &args![text]).u32();
        let copy = allocate(e, length + 1);
        e.set(this, TileValue::strValue, copy);
        e.call(STRCPY_S, &args![copy, length + 1, text]);
    }
    e.set(this, TileValue::fValue, 0.0);
    if flag {
        value_clear_actions(e, this);
    }
    value_calculate_value(e, this, changed);
}

/// Links `item` behind the last item of the template's list (a node from
/// the list's allocator, then the tail link).
fn template_push_item(e: &mut Engine, this: Ptr<TileTemplate>, item: Ptr<TileTemplateItem>) {
    let list = this.at(TileTemplate::xList).addr();
    let node = e.call(LIST_NEW_NODE, &args![list + 8]).u32();
    e.mem.set_u32(node + 8, item.addr());
    e.call(TEMPLATE_LIST_ADD_NODE_TAIL, &args![list, node]);
}

/// Takes the last item off the template's list and gives it back to the
/// pool ([`fn_00a0b8d0`]).
fn template_drop_last_item(e: &mut Engine, this: Ptr<TileTemplate>) {
    let list = this.at(TileTemplate::xList).addr();
    let last = e.call(TEMPLATE_LIST_POP_TAIL, &args![list]).ptr();
    fn_00a0b8d0(e, last);
}

/// Reads a number from `text` with `sscanf("%f")` into `value` (left
/// alone when the text holds none).
fn scan_float(e: &mut Engine, text: Ptr, value: &mut f32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), *value);
        e.call(SSCANF, &args![text, FLOAT_SCAN_FORMAT, slot]);
        *value = e.mem.f32(slot.addr());
    });
}

// Translated from 00a0a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TileTemplate::AddPair` (Xbox PDB): one word of the XML reader
/// (`command` is a `Tile::enumTag`, or 0/1 for the start/end of an
/// element; `text`; `line`) becomes a template item, or is folded into
/// the items before it.
///
/// The item's value is the text table's number for `text` (unless `flag`),
/// a user trait id for a name that starts with `_` or `&_`, else the
/// number the text spells (items whose text is all digits, `-` and `.`).
/// Then, looking at the last three items (`previous`), the pair is
/// turned into: the definition of a template (a `name` after the marker
/// value 999: finds or creates the sub-template, and the end marker 999
/// closes it); a tile start (a `name` after an item that holds a tile-type
/// number 901 to 908); a simple trait or action (`value` after a trait or
/// action start of the same number); a trait link (`trait`, `src`, action
/// start); or a new item of kind trait start/end, action start/end or
/// tile end, which is added at the tail. Items that are folded are given
/// back to the pool.
pub fn template_add_pair(
    e: &mut Engine,
    this: Ptr<TileTemplate>,
    command: i32,
    text: Ptr,
    line: i32,
    flag: bool,
) {
    let mut value: f32 = e.global(NO_VALUE_FLOAT);
    if !flag {
        value = tile_text_to_trait(e, text) as f32;
    }
    let no_value = e.global::<f64>(NO_TRAIT_VALUE);
    let first = e.mem.i8(text.addr());
    if value as f64 == no_value
        && (first == b'_' as i8 || (first == b'&' as i8 && e.mem.i8(text.addr() + 1) == b'_' as i8))
    {
        value = tile_add_user_trait(e, text, -1) as f32;
    }
    let trait_id = float_to_int(e, value);
    let item = fn_00a0b950(e, command, value, text, trait_id, line);
    let length = e.call(STRLEN, &args![text]).u32();
    let mut numeric = true;
    for index in 0..length {
        let c = e.mem.i8(text.addr() + index);
        if !(b'0' as i8..=b'9' as i8).contains(&c) && c != b'-' as i8 && c != b'.' as i8 {
            numeric = false;
            break;
        }
    }
    if numeric {
        if value as f64 == no_value {
            scan_float(e, text, &mut value);
        }
        if value as f64 == e.global::<f64>(ZERO) {
            scan_float(e, text, &mut value);
        }
        e.call(STRING_SET, &args![item.addr() + 8, EMPTY_NAME, 0u32]);
        e.set(item, TileTemplateItem::fVal, value);
        let number = float_to_int(e, value);
        e.set(item, TileTemplateItem::u, number as u32);
    }
    // The last three items: `previous[0]` is the one before this pair.
    let mut node = e.get(this.at(TileTemplate::xList), NiTPointerList::m_pkTail);
    let mut previous = [0u32; 3];
    for slot in previous.iter_mut() {
        if node != 0 {
            *slot = e.mem.u32(node + 8);
            node = e.mem.u32(node + 4);
        }
    }
    let [last, second, third] = previous;
    let storage = e.get(this, TileTemplate::pParent);
    let marker = e.global::<f64>(TEMPLATE_MARKER_VALUE);
    let tile_type_first = e.global::<f64>(TILE_TYPE_FIRST_DOUBLE);
    let tile_type_last = e.global::<f64>(TILE_TYPE_LAST_DOUBLE);
    let command_of = |e: &Engine, item: u32| e.mem.i32(item);
    let number_of = |e: &Engine, item: u32| e.mem.f32(item + 4);
    let item_number = number_of(e, item.addr());
    let item_trait = e.mem.i32(item.addr() + 0x10);
    if command == TAG_NAME
        && last != 0
        && command_of(e, last) == 0
        && number_of(e, last) as f64 == marker
    {
        // The start of a template definition: `<name="...">` after the
        // marker.
        if e.get(storage, BuildStorage::pCurrentTemplate).is_null() {
            let found = fn_00a0af10(e, storage, text);
            e.set(storage, BuildStorage::pCurrentTemplate, found);
            if found.is_null() {
                let made = fn_00a0ae70(e, storage, text);
                e.set(storage, BuildStorage::pCurrentTemplate, made);
            }
        } else {
            e.call(PRINT_ERROR, &args![MSG_NESTED_TEMPLATES]);
        }
        template_drop_last_item(e, this);
        fn_00a0b8d0(e, item.cast());
    } else if command == 1
        && value as f64 == marker
        && !e.get(storage, BuildStorage::pCurrentTemplate).is_null()
    {
        // The end of the template definition.
        e.set(storage, BuildStorage::pCurrentTemplate, Ptr::NULL);
        fn_00a0b8d0(e, item.cast());
    } else if command == TAG_NAME
        && last != 0
        && command_of(e, last) == 0
        && number_of(e, last) as f64 >= tile_type_first
        && number_of(e, last) as f64 <= tile_type_last
    {
        // A tile type followed by its name: the tile start.
        e.mem.set_i32(last, TI_TILE_START);
        e.call(STRING_SET, &args![last + 8, text, 0u32]);
        fn_00a0b8d0(e, item.cast());
    } else if command == TAG_NAME {
        e.set(item, TileTemplateItem::iCmd, TI_SIMPLE_TRAIT);
        e.set(item, TileTemplateItem::u, TAG_NAME as u32);
        template_push_item(e, this, item);
    } else if command == 1
        && last != 0
        && command_of(e, last) == TAG_VALUE
        && second != 0
        && (command_of(e, second) == TI_TRAIT_START || command_of(e, second) == TI_ACTION_START)
        && number_of(e, second) == value
    {
        // `value` after a trait or action start of the same number: the
        // simple form.
        let number = item_number as f64;
        let kind = if (number >= e.global::<f64>(TRAIT_ID_FIRST_DOUBLE)
            && number <= e.global::<f64>(TRAIT_ID_LAST_DOUBLE))
            || item_trait >= FIRST_USER_TEXT_ID
        {
            TI_SIMPLE_TRAIT
        } else if number >= e.global::<f64>(ACTION_ID_FIRST_DOUBLE)
            && number <= e.global::<f64>(ACTION_ID_LAST_DOUBLE)
        {
            TI_SIMPLE_ACTION
        } else {
            TI_BAD
        };
        e.mem.set_i32(second, kind);
        if kind == TI_BAD {
            e.call(PRINT_ERROR, &args![MSG_BAD_TRAIT_OR_ACTION]);
        }
        let second_number = float_to_int(e, number_of(e, second));
        e.mem.set_i32(second + 0x10, second_number);
        let value_number = number_of(e, last);
        e.mem.set_f32(second + 4, value_number);
        e.call(STRING_ASSIGN, &args![second + 8, last + 8]);
        template_drop_last_item(e, this);
        fn_00a0b8d0(e, item.cast());
    } else if command == 1
        && last != 0
        && command_of(e, last) == TAG_TRAIT
        && second != 0
        && command_of(e, second) == TAG_SRC
        && third != 0
        && command_of(e, third) == TI_ACTION_START
        && number_of(e, third) == value
    {
        // `trait` and `src` after an action start: the trait link.
        e.mem.set_i32(third, TI_TRAIT_LINK);
        let third_number = float_to_int(e, number_of(e, third));
        e.mem.set_i32(third + 0x10, third_number);
        let link_number = number_of(e, last);
        e.mem.set_f32(third + 4, link_number);
        e.call(STRING_ASSIGN, &args![third + 8, second + 8]);
        template_drop_last_item(e, this);
        template_drop_last_item(e, this);
        fn_00a0b8d0(e, item.cast());
    } else {
        let kind = e.mem.i32(item.addr());
        if (TRAIT_ID_FIRST..=TRAIT_ID_LAST).contains(&item_trait)
            || item_trait >= FIRST_USER_TEXT_ID
        {
            if kind == 0 {
                e.mem.set_i32(item.addr(), TI_TRAIT_START);
            } else if kind == 1 {
                e.mem.set_i32(item.addr(), TI_TRAIT_END);
            }
        } else if (ACTION_ID_FIRST..=ACTION_ID_LAST).contains(&item_trait) {
            if kind == 0 {
                e.mem.set_i32(item.addr(), TI_ACTION_START);
            } else if kind == 1 && last != 0 {
                e.mem.set_i32(item.addr(), TI_ACTION_END);
            }
        } else if kind == 1
            && item_number as f64 >= tile_type_first
            && item_number as f64 <= tile_type_last
        {
            e.mem.set_i32(item.addr(), TI_TILE_END);
        }
        template_push_item(e, this, item);
    }
}

// Translated from 00a0ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TileTemplate::Clear` (Xbox PDB): removes the items from the
/// head of the list one by one, freeing the string of each and the item.
pub fn template_clear(e: &mut Engine, this: Ptr<TileTemplate>) {
    let list = this.at(TileTemplate::xList);
    while e.get(list, NiTPointerList::m_uiCount) != 0 {
        let item = e.call(TEMPLATE_LIST_POP_HEAD, &args![list]).u32();
        if item != 0 {
            e.call(STRING_SET, &args![item + 8, 0u32, 0u32]);
            deallocate(e, item);
        }
    }
}

// Translated from 00a0abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Extra::~Extra` (Xbox PDB): restores the vtable and, when the
/// extra data still has a tile, points that tile's parent at the tile of
/// the node's parent node and drops the tile's model reference (the C++
/// exception frame is not translated). Clears the tile pointer and runs
/// the destructor of the base class `NiExtraData`.
pub fn extra_destructor(e: &mut Engine, this: Ptr<Extra>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA);
    let tile = e.get(this, Extra::pTile);
    if !tile.is_null() {
        let node = e.get(this, Extra::pNode);
        let parent_node = Ptr::new(e.mem.u32(node.addr() + 0x18));
        let parent_tile = tile_get_tile_from_node(e, parent_node);
        e.set(tile, Tile::pParent, parent_tile);
        e.call(NI_POINTER_ASSIGN, &args![tile.addr() + 0x2c, 0u32]);
    }
    e.set(this, Extra::pTile, Ptr::NULL);
    e.call(NI_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 00a0ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::BuildStorage::BuildStorage` (Xbox PDB): empties the sub-template
/// list, makes the main template (0x14 bytes, [`TEMPLATE_CONSTRUCT`], named
/// `01094d90`, owned by the storage), no current template, and sets
/// `bDeleteTemplates`.
pub fn build_storage_construct(e: &mut Engine, this: Ptr<BuildStorage>) -> Ptr<BuildStorage> {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    let block = allocate(e, 0x14);
    let template = if block.is_null() {
        Ptr::NULL
    } else {
        e.call(TEMPLATE_CONSTRUCT, &args![block, MAIN_TEMPLATE_NAME, this])
            .ptr()
    };
    e.set(this, BuildStorage::pTemplate, template);
    e.set(this, BuildStorage::pCurrentTemplate, Ptr::NULL);
    e.set(this, BuildStorage::bDeleteTemplates, true);
    this
}

// Translated from 00a0ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::BuildStorage::~BuildStorage` (Xbox PDB): destroys and frees the
/// main template, clears the current template and, when `bDeleteTemplates`
/// is set, destroys and frees every sub-template of the list; then empties
/// the list (twice, as the compiler's cleanup runs the list destructor
/// again; the exception frame is not translated).
pub fn build_storage_destructor(e: &mut Engine, this: Ptr<BuildStorage>) {
    let main = e.get(this, BuildStorage::pTemplate);
    if !main.is_null() {
        e.call(TEMPLATE_DESTROY, &args![main]);
        deallocate(e, main.addr());
    }
    e.set(this, BuildStorage::pCurrentTemplate, Ptr::NULL);
    let list = this.at(BuildStorage::xSubTemplates).addr();
    if e.get(this, BuildStorage::bDeleteTemplates) {
        let mut node = list;
        while node != 0 {
            let template = e.mem.u32(node);
            if template != 0 {
                e.call(TEMPLATE_DESTROY, &args![template]);
                deallocate(e, template);
            }
            node = e.mem.u32(node + 4);
        }
    }
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![list]);
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![list]);
}

// Translated from 00a0ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a new sub-template named `name` (0x14 bytes, `thiscall` on
/// [`TEMPLATE_CONSTRUCT`], owned by the storage), adds it to the storage's
/// sub-template list and answers it.
pub fn fn_00a0ae70(e: &mut Engine, this: Ptr<BuildStorage>, name: Ptr) -> Ptr<TileTemplate> {
    let block = allocate(e, 0x14);
    let template = if block.is_null() {
        0
    } else {
        e.call(TEMPLATE_CONSTRUCT, &args![block, name, this]).u32()
    };
    let list = this.at(BuildStorage::xSubTemplates).addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), template);
        e.call(SUB_TEMPLATE_LIST_ADD_HEAD, &args![list, slot]);
        Ptr::new(e.mem.u32(slot.addr()))
    })
}

/// Drops the reference an `NiFixedString` handle holds (not for the empty
/// string's handle).
fn fixed_string_release(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle - 8]);
    }
}

// Translated from 00a0af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sub-template of the storage whose name equals `name`, or null. The
/// name is turned into a fixed string (the empty string's handle when
/// `name` is null) and compared with each template's name by handle, else
/// as text. The scan stops at the first empty list entry.
pub fn fn_00a0af10(e: &mut Engine, this: Ptr<BuildStorage>, name: Ptr) -> Ptr<TileTemplate> {
    let target = if name.is_null() {
        e.global::<u32>(EMPTY_FIXED_STRING)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![name]).u32()
    };
    let mut node = this.at(BuildStorage::xSubTemplates).addr();
    let mut found = 0;
    while node != 0 && e.mem.u32(node) != 0 {
        let template = e.mem.u32(node);
        let other = e.mem.u32(template);
        let same = if target == other {
            true
        } else if other == 0 || target == 0 {
            false
        } else {
            e.call(STRCMP, &args![target, other]).i32() == 0
        };
        if same {
            found = template;
            break;
        }
        node = e.mem.u32(node + 4);
    }
    fixed_string_release(e, target);
    Ptr::new(found)
}

/// The first child of a model node: element 0 of the child array (at
/// +0xa0, its count is the 16-bit word at +0xa6), null when there is none.
fn first_child(e: &Engine, model: Ptr) -> Ptr {
    if model.is_null() || e.mem.u16(model.addr() + 0xa6) == 0 {
        Ptr::NULL
    } else {
        Ptr::new(e.mem.u32(e.mem.u32(model.addr() + 0xa0)))
    }
}

// Translated from 00a0b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::ForceTextureRelease` (Xbox PDB): an image tile (type `0x386`)
/// whose model has a first child with a shader property (property type 3)
/// has the texture of that property cleared and a texture update queued;
/// then every child without a menu is asked (virtual slot +0x1c) to do
/// the same.
pub fn tile_force_texture_release(e: &mut Engine, this: Ptr<Tile>) {
    let model = e.get(this, Tile::spModel);
    if !model.is_null() {
        let shape = first_child(e, model);
        if tile_type(e, this) == TYPE_IMAGE && !shape.is_null() {
            let property = e.call(NI_OBJECT_GET_PROPERTY, &args![shape, 3u32]).u32();
            if property != 0 {
                e.call(SET_TILE_TEXTURE, &args![property, 0u32]);
                tile_add_needs_update(e, this, UPDATE_TEXTURE);
            }
        }
    }
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child = Ptr::<Tile>::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        if tile_get_menu(e, child).is_null() {
            e.vcall(child.addr(), 0x1c, &[]);
        }
    }
}

// Translated from 00a0b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::PlayTileSound` (Xbox PDB): when the tile's menu has `1` in its
/// word at +0x24 and trait `trait_id` holds a non-empty string, looks the
/// string up as a sound name (flags `0x121`) in the audio manager and
/// plays the handle it answers (a 12-byte `BSSoundHandle`; the C++
/// exception frame is not translated).
pub fn tile_play_tile_sound(e: &mut Engine, this: Ptr<Tile>, trait_id: i32) {
    let name = tile_get_string(e, this, trait_id);
    let menu = tile_get_menu(e, this);
    if !menu.is_null()
        && e.mem.u32(menu.addr() + 0x24) == 1
        && !name.is_null()
        && e.mem.i8(name.addr()) != 0
    {
        let audio = e.global::<u32>(AUDIO_MANAGER);
        e.with_stack(12, |e, found| {
            e.with_stack(12, |e, handle| {
                e.mem.set_u32(handle.addr(), u32::MAX);
                e.mem.set_u8(handle.addr() + 4, 0);
                e.mem.set_u32(handle.addr() + 8, 0);
                let source = e
                    .call(
                        GET_SOUND_HANDLE_BY_NAME,
                        &args![audio, found, name, TILE_SOUND_FLAGS],
                    )
                    .u32();
                let (id, flag, extra) = (
                    e.mem.u32(source),
                    e.mem.u8(source + 4),
                    e.mem.u32(source + 8),
                );
                e.mem.set_u32(handle.addr(), id);
                e.mem.set_u8(handle.addr() + 4, flag);
                e.mem.set_u32(handle.addr() + 8, extra);
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
            });
        });
    }
}

// Translated from 00a0b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::GetShaderProperty` (Xbox PDB): the shader property (property
/// type 3) of the first child of the tile's model, or null.
pub fn tile_get_shader_property(e: &mut Engine, this: Ptr<Tile>) -> Ptr {
    let model = e.get(this, Tile::spModel);
    let shape = first_child(e, model);
    if shape.is_null() {
        Ptr::NULL
    } else {
        e.call(NI_OBJECT_GET_PROPERTY, &args![shape, 3u32]).ptr()
    }
}

// Translated from 00a0b280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SetAlphaAndColor` (Xbox PDB, virtual; `this` is not used): for
/// each child of `model` that answers a geometry (virtual slot +0x18),
/// stores `alpha` at +0x78 and the four words at `color` at +0x68 of its
/// shader property (property type 3).
pub fn tile_set_alpha_and_color(
    e: &mut Engine,
    _this: Ptr<Tile>,
    model: Ptr,
    alpha: f32,
    color: Ptr,
) {
    if model.is_null() {
        return;
    }
    let mut index = 0u32;
    while index < e.mem.u16(model.addr() + 0xa6) as u32 {
        let child = e.mem.u32(e.mem.u32(model.addr() + 0xa0) + index * 4);
        let geometry = if child != 0 {
            e.vcall(child, 0x18, &[]).u32()
        } else {
            0
        };
        if geometry != 0 {
            let property = e.call(NI_OBJECT_GET_PROPERTY, &args![geometry, 3u32]).u32();
            e.mem.set_f32(property + 0x78, alpha);
            for word in 0..4 {
                let value = e.mem.u32(color.addr() + word * 4);
                e.mem.set_u32(property + 0x68 + word * 4, value);
            }
        }
        index += 1;
    }
}

/// The system colour scale the two colour walks use: trait `eSystemColor`
/// of the tile, or `fallback` when the tile has none (0.0).
fn system_color_or(e: &mut Engine, tile: Ptr<Tile>, fallback: f32) -> f32 {
    let value = fn_00a011b0(e, tile, TRAIT_SYSTEM_COLOR);
    if differs_from_zero(e, value) {
        value
    } else {
        fallback
    }
}

// Translated from 00a0b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under `Tile::Lock`: takes the tile's system colour trait (0.0 means
/// `inherited`, the value passed down), queues a colour update when it
/// equals the system colour number `color`, and does the same for every
/// child with the resulting colour. Called with `(color, inherited)`.
pub fn fn_00a0b350(e: &mut Engine, this: Ptr<Tile>, color: i32, inherited: f32) {
    lock(e);
    let own = system_color_or(e, this, inherited);
    if color as f64 == own as f64 {
        tile_add_needs_update(e, this, UPDATE_COLOR);
    }
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child = Ptr::<Tile>::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        fn_00a0b350(e, child, color, own);
    }
    unlock(e);
}

// Translated from 00a0b420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under `Tile::Lock`: as [`fn_00a0b350`], but queues the colour update
/// when the (truncated) system colour number is in the `list` of numbers
/// (a chain of nodes, number at +0 and next at +4); called with
/// `(list, inherited)`.
pub fn fn_00a0b420(e: &mut Engine, this: Ptr<Tile>, list: Ptr, inherited: f32) {
    lock(e);
    let own = system_color_or(e, this, inherited);
    let number = float_to_int(e, own);
    let mut entry = list.addr();
    while entry != 0 && e.mem.i32(entry) != number {
        entry = e.mem.u32(entry + 4);
    }
    if entry != 0 {
        tile_add_needs_update(e, this, UPDATE_COLOR);
    }
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child = Ptr::<Tile>::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        fn_00a0b420(e, child, list, own);
    }
    unlock(e);
}

// Translated from 00a0b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::RecursiveRotationUpdate` (Xbox PDB): for every child of the
/// tile, gives it the rotation axis point (`(x + axis x) - x of the child`
/// and the same for y; the child's x and y count the tile's when the tile
/// has a locus) and the tile's angle. A child that has a locus and is an
/// image tile (type `0x386`) also has its model's shapes turned: each
/// shape (an object whose RTTI chain holds [`SHAPE_RTTI`]) gets the
/// y-rotation matrix of the child's angle as its local rotation and as
/// translation `(translation + axis) - rotation * axis`, with the
/// translation built from the default one, the child's x, y and depth.
/// Then the child's own children are updated.
pub fn tile_recursive_rotation_update(e: &mut Engine, this: Ptr<Tile>) {
    let mut node = e.get(this.at(Tile::xChildren), NiTPointerList::m_pkHead);
    while node != 0 {
        let child = Ptr::<Tile>::new(e.mem.u32(node + 8));
        node = e.mem.u32(node);
        if child.is_null() {
            continue;
        }
        let x = fn_00a011b0(e, this, TRAIT_X);
        let y = fn_00a011b0(e, this, TRAIT_Y);
        let axis_x = fn_00a011b0(e, this, TRAIT_ROTATE_AXIS_X);
        let axis_y = fn_00a011b0(e, this, TRAIT_ROTATE_AXIS_Y);
        let mut child_x = fn_00a011b0(e, child, TRAIT_X);
        let mut child_y = fn_00a011b0(e, child, TRAIT_Y);
        if tile_is_true(e, this, TRAIT_LOCUS) {
            child_x = (child_x as f64 + x as f64) as f32;
            child_y = (child_y as f64 + y as f64) as f32;
        }
        let new_axis_x = ((x as f64 + axis_x as f64) - child_x as f64) as f32;
        fn_00a012d0(e, child, TRAIT_ROTATE_AXIS_X, new_axis_x, true);
        let new_axis_y = ((y as f64 + axis_y as f64) - child_y as f64) as f32;
        fn_00a012d0(e, child, TRAIT_ROTATE_AXIS_Y, new_axis_y, true);
        let angle = fn_00a011b0(e, this, TRAIT_ROTATE_ANGLE);
        fn_00a012d0(e, child, TRAIT_ROTATE_ANGLE, angle, true);
        if tile_is_true(e, child, TRAIT_LOCUS) && tile_type(e, child) == TYPE_IMAGE {
            turn_model_shapes(e, child);
        }
        tile_recursive_rotation_update(e, child);
    }
}

/// The shape part of [`tile_recursive_rotation_update`]: see there.
fn turn_model_shapes(e: &mut Engine, child: Ptr<Tile>) {
    let depth = fn_00a011b0(e, child, TRAIT_DEPTH);
    let mut translation = default_translation(e);
    let model = e.get(child, Tile::spModel);
    set_translation(e, model.addr(), translation);
    translation[0] = (fn_00a011b0(e, child, TRAIT_X) as f64 + translation[0] as f64) as f32;
    translation[2] = (-fn_00a011b0(e, child, TRAIT_Y) as f64 + translation[2] as f64) as f32;
    translation[1] = (depth as f64 * e.global::<f64>(DEPTH_SCALE) + translation[1] as f64) as f32;
    let model = e.get(child, Tile::spModel).addr();
    let mut index = 0u32;
    while model != 0 && index < e.mem.u16(model + 0xa8) as u32 {
        let object = e.mem.u32(e.mem.u32(model + 0xa0) + index * 4);
        let shape = if object == 0 {
            0
        } else {
            // The object's RTTI chain contains the record at 011f4a40.
            let mut rtti = e.vcall(object, 8, &[]).u32();
            while rtti != 0 && rtti != SHAPE_RTTI {
                rtti = e.mem.u32(rtti + 4);
            }
            if rtti != 0 {
                object
            } else {
                0
            }
        };
        let axis_x = fn_00a011b0(e, child, TRAIT_ROTATE_AXIS_X);
        let axis_z = -fn_00a011b0(e, child, TRAIT_ROTATE_AXIS_Y);
        let angle = fn_00a011b0(e, child, TRAIT_ROTATE_ANGLE);
        // Scratch: the matrix, then the axis, translation, rotated axis,
        // their sum and the result (three floats each).
        e.with_stack(0x24 + 5 * 12, |e, scratch| {
            let matrix = scratch.addr();
            let axis_at = matrix + 0x24;
            let translation_at = axis_at + 12;
            let rotated_at = translation_at + 12;
            let sum_at = rotated_at + 12;
            let result_at = sum_at + 12;
            for (slot, value) in [axis_x, 0.0, axis_z].iter().enumerate() {
                e.mem.set_f32(axis_at + slot as u32 * 4, *value);
            }
            for (slot, value) in translation.iter().enumerate() {
                e.mem.set_f32(translation_at + slot as u32 * 4, *value);
            }
            e.call(MATRIX_MAKE_Y_ROTATION, &args![matrix, angle]);
            if shape != 0 {
                for slot in 0..9 {
                    let value = e.mem.u32(matrix + slot * 4);
                    e.mem.set_u32(shape + 0x34 + slot * 4, value);
                }
                let rotated = e
                    .call(MATRIX_TIMES_POINT, &args![matrix, rotated_at, axis_at])
                    .u32();
                let sum = e
                    .call(POINT_ADD, &args![translation_at, sum_at, axis_at])
                    .u32();
                let moved = e
                    .call(POINT_SUBTRACT, &args![sum, result_at, rotated])
                    .u32();
                for slot in 0..3 {
                    let value = e.mem.u32(moved + slot * 4);
                    e.mem.set_u32(shape + 0x58 + slot * 4, value);
                }
            }
        });
        index += 1;
    }
}

// Translated from 00a0b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives a `Tile::TileTemplateItem` back: into the pool of unused items
/// (ten words at `011f329c`, count at `011f32dc`) while it has room, else
/// its string is freed and the item deleted.
pub fn fn_00a0b8d0(e: &mut Engine, item: Ptr) {
    let count: u32 = e.global(ITEM_POOL_COUNT);
    if count < ITEM_POOL_SIZE {
        e.set_global(ITEM_POOL + count * 4, item.addr());
        e.set_global(ITEM_POOL_COUNT, count + 1);
    } else if !item.is_null() {
        e.call(STRING_SET, &args![item.addr() + 8, 0u32, 0u32]);
        deallocate(e, item.addr());
    }
}

// Translated from 00a0b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `Tile::TileTemplateItem` (command, `value`, a copy of `text`, trait
/// number, line): the last one of the pool of unused items, filled in, or
/// a new one (0x18 bytes) built by [`TEMPLATE_ITEM_CONSTRUCT`] when the
/// pool is empty. `cdecl`.
pub fn fn_00a0b950(
    e: &mut Engine,
    command: i32,
    value: f32,
    text: Ptr,
    trait_id: i32,
    line: i32,
) -> Ptr<TileTemplateItem> {
    let count: u32 = e.global(ITEM_POOL_COUNT);
    if count == 0 {
        let block = allocate(e, 0x18);
        if block.is_null() {
            return Ptr::NULL;
        }
        return e
            .call(
                TEMPLATE_ITEM_CONSTRUCT,
                &args![block, command, value, text, trait_id, line],
            )
            .ptr();
    }
    e.set_global(ITEM_POOL_COUNT, count - 1);
    let item: Ptr<TileTemplateItem> = Ptr::new(e.global::<u32>(ITEM_POOL + (count - 1) * 4));
    e.set(item, TileTemplateItem::iCmd, command);
    e.set(item, TileTemplateItem::fVal, value);
    e.call(STRING_SET, &args![item.addr() + 8, text, 0u32]);
    e.set(item, TileTemplateItem::u, trait_id as u32);
    e.set(item, TileTemplateItem::iLine, line);
    item
}

// Translated from 00a0ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the pool of unused template items: each one's string is freed
/// and the item deleted.
pub fn fn_00a0ba50(e: &mut Engine) {
    loop {
        let count: u32 = e.global(ITEM_POOL_COUNT);
        e.set_global(ITEM_POOL_COUNT, count.wrapping_sub(1));
        if count == 0 {
            break;
        }
        let item: u32 = e.global(ITEM_POOL + (count - 1) * 4);
        if item != 0 {
            e.call(STRING_SET, &args![item + 8, 0u32, 0u32]);
            deallocate(e, item);
        }
    }
    e.set_global(ITEM_POOL_COUNT, 0u32);
}

// Translated from 00a0bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::SpecialBoundsCheck` (Xbox PDB, virtual; both words and `this`
/// are unused): always true.
pub fn tile_special_bounds_check(
    _e: &mut Engine,
    _this: Ptr<Tile>,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    true
}

// Translated from 00a0baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A tile method that only calls `Tile::Unlock` (`this` is not used).
pub fn fn_00a0baf0(e: &mut Engine, _this: Ptr<Tile>) {
    unlock(e);
}

// Translated from 00a0bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// True for the spelling of a user-defined name: not empty and starting
/// with `_`, or with the two bytes at `01093f80` (`"&_"`). `cdecl`.
pub fn fn_00a0bb00(e: &mut Engine, name: Ptr) -> bool {
    if name.is_null() || e.mem.i8(name.addr()) == 0 {
        return false;
    }
    e.mem.i8(name.addr()) == b'_' as i8
        || e.call(MEMCMP, &args![name, USER_NAME_PREFIX, 2u32]).i32() == 0
}

// Translated from 00a0bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the three strings of a `Tile::TextureAtlasEntry` (at +0x10, +8
/// and +0), in that order.
pub fn fn_00a0bb50(e: &mut Engine, this: Ptr) {
    for offset in [0x10u32, 8, 0] {
        e.call(STRING_SET, &args![this.addr() + offset, 0u32, 0u32]);
    }
}

// Translated from 00a0bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::GetType` (Xbox PDB): the type number of a text tile.
pub fn tile_text_get_type(_e: &mut Engine, _this: Ptr<Tile>) -> u32 {
    TYPE_TEXT
}

// Translated from 00a0bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::GetTypeName` (Xbox PDB): the address of the type name text.
pub fn tile_text_get_type_name(_e: &mut Engine, _this: Ptr<Tile>) -> u32 {
    TILE_TEXT_TYPE_NAME
}

// Translated from 00a0bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileText::_scalar_deleting_destructor_` (Xbox PDB): sets the text
/// tile's vtable, releases the tile ([`tile_release`]) unless it has the
/// released flag (0x2000), runs [`tile_destructor`] and, with bit 0 of
/// `flags`, deletes the object (the C++ exception frame is not
/// translated).
pub fn tile_text_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Tile>,
    flags: u32,
) -> Ptr<Tile> {
    e.mem.set_u32(this.addr(), VTABLE_TILE_TEXT);
    if e.get(this, Tile::uiFlags) & FLAG_RELEASED == 0 {
        tile_release(e, this);
    }
    tile_destructor(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::Tile3D` (Xbox PDB name of the body; the map has none): runs the
/// base tile constructor, sets the `Tile3D` vtable, empties the two arrays
/// at +0x40 and +0x48 (data pointer, 16-bit capacity and 16-bit count,
/// all 0), clears the model pointer at +0x2c and the words at +0x38 and
/// +0x3c. The C++ exception frame is not translated.
pub fn fn_00a0bc80(e: &mut Engine, this: Ptr<Tile>) -> Ptr<Tile> {
    e.call(TILE_BASE_CONSTRUCT, &args![this]);
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_TILE_3D);
    for array in [0x40u32, 0x48] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u16(base + array + 4, 0);
        e.mem.set_u16(base + array + 6, 0);
    }
    e.call(NI_POINTER_ASSIGN, &args![base + 0x2c, 0u32]);
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x3c, 0);
    this
}

// Translated from 00a0bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::GetType` (Xbox PDB): the type number of a 3D tile.
pub fn tile_3d_get_type(_e: &mut Engine, _this: Ptr<Tile>) -> u32 {
    TYPE_3D
}

// Translated from 00a0bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::GetTypeName` (Xbox PDB): the address of the type name text.
pub fn tile_3d_get_type_name(_e: &mut Engine, _this: Ptr<Tile>) -> u32 {
    TILE_3D_TYPE_NAME
}

// Translated from 00a0bde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::_scalar_deleting_destructor_` (Xbox PDB): runs `~Tile3D` and,
/// with bit 0 of `flags`, deletes the object.
pub fn tile_3d_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Tile>,
    flags: u32,
) -> Ptr<Tile> {
    e.call(TILE_3D_DESTROY, &args![this]);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `XMLStorage::_scalar_deleting_destructor_` (Xbox PDB): frees the file
/// data, clears the pointer and, with bit 0 of `flags`, deletes the object.
pub fn xml_storage_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<XmlStorage>,
    flags: u32,
) -> Ptr<XmlStorage> {
    xml_storage_destructor(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0be70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `XMLStorage::~XMLStorage` (Xbox PDB): frees the file data and clears
/// the pointer.
pub fn xml_storage_destructor(e: &mut Engine, this: Ptr<XmlStorage>) {
    let data = e.get(this, XmlStorage::pXMLData);
    if !data.is_null() {
        deallocate(e, data.addr());
    }
    e.set(this, XmlStorage::pXMLData, Ptr::NULL);
}

// Translated from 00a0beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TextureAtlasEntry::TextureAtlasEntry` (Xbox PDB name of the
/// body; the map has none): three empty strings (data pointer, 16-bit
/// capacity and count, all 0) at +0, +8 and +0x10 and four floats at
/// +0x18 set to 0.0.
pub fn fn_00a0beb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for string in [0u32, 8, 0x10] {
        e.mem.set_u32(base + string, 0);
        e.mem.set_u16(base + string + 4, 0);
        e.mem.set_u16(base + string + 6, 0);
    }
    for float in 0..4 {
        e.mem.set_f32(base + 0x18 + float * 4, 0.0);
    }
    this
}

// Translated from 00a0c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::FloatAction::QFloat` (Xbox PDB): the action's operand.
pub fn float_action_q_float(e: &mut Engine, this: Ptr<ValueAction>) -> f32 {
    e.get(this, ValueAction::fValue)
}

// Translated from 00a0c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::RefValueAction::QFloat` (Xbox PDB): the float of the trait the
/// action reads, 0.0 when it reads none.
pub fn ref_value_action_q_float(e: &mut Engine, this: Ptr<ValueAction>) -> f32 {
    let referenced = e.get(this, ValueAction::pRefValue);
    if referenced.is_null() {
        0.0
    } else {
        e.get(referenced, TileValue::fValue)
    }
}

// Translated from 00a0c060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trait an action reads (`pRefValue` at +0xc): slot +4 of
/// `Tile::RefValueAction` (the map has no name for it).
pub fn fn_00a0c060(e: &mut Engine, this: Ptr<ValueAction>) -> Ptr<TileValue> {
    e.get(this, ValueAction::pRefValue)
}

// Translated from 00a0c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::GetRTTI` (the map's name; the body answers the static
/// `NiRTTI` record at `011f4a80`).
pub fn ni_extra_data_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    EXTRA_DATA_RTTI
}

// Translated from 00a0c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Extra::_scalar_deleting_destructor_` (Xbox PDB): runs
/// [`extra_destructor`] and, with bit 0 of `flags`, deletes the 0x14-byte
/// object (`operator delete(block, size)`).
pub fn extra_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Extra>,
    flags: u32,
) -> Ptr<Extra> {
    extra_destructor(e, this);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00a0c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TileTemplate::TileTemplate` (the map has no name): the name starts
/// as the empty fixed string and the item list as empty (head, tail and count
/// 0); a `name` other than the empty handle is made into a fixed string
/// (`NiFixedString(char *)`) and the previous handle released; the owner is
/// `storage`. The C++ exception frame is not translated.
pub fn template_construct(
    e: &mut Engine,
    this: Ptr<TileTemplate>,
    name: u32,
    storage: Ptr<BuildStorage>,
) -> Ptr<TileTemplate> {
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.set(this, TileTemplate::xName, empty);
    let list = this.at(TileTemplate::xList);
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    if e.get(this, TileTemplate::xName) != name {
        let old = e.get(this, TileTemplate::xName);
        let handle = e.call(NI_FIXED_STRING_CREATE, &args![name]).u32();
        e.set(this, TileTemplate::xName, handle);
        fixed_string_release(e, old);
    }
    e.set(this, TileTemplate::pParent, storage);
    this
}

// Translated from 00a0c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TileTemplate::~TileTemplate` (Xbox PDB): clears the items
/// ([`template_clear`]), runs the destructor of the item list and drops the
/// reference of the name (not for the empty string's handle). The C++
/// exception frame is not translated.
pub fn template_destructor(e: &mut Engine, this: Ptr<TileTemplate>) {
    template_clear(e, this);
    e.call(LIST_DESTROY, &args![this.addr() + 8]);
    let name = e.get(this, TileTemplate::xName);
    fixed_string_release(e, name);
}

// Translated from 00a0c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::TileTemplateItem::TileTemplateItem` (the body; the map has no
/// name): an empty string at +8 (no buffer, capacity 0, length 0), then the
/// command, the number, the text (assigned with limit `0x1f`), the trait
/// number and the source line. The compiler folded the length clamps of
/// the empty string (`0 > 0xffff` never holds). Answers `this`.
pub fn fn_00a0c220(
    e: &mut Engine,
    this: Ptr<TileTemplateItem>,
    command: i32,
    number: f32,
    text: Ptr,
    trait_number: u32,
    line: i32,
) -> Ptr<TileTemplateItem> {
    let string = this.at(TileTemplateItem::xStr);
    e.set(string, BSStringT::pString, 0);
    e.set(string, BSStringT::sLen, 0);
    e.set(string, BSStringT::sMaxLen, 0);
    e.set(this, TileTemplateItem::iCmd, command);
    e.set(this, TileTemplateItem::fVal, number);
    e.call(STRING_SET, &args![string, text, 0x1fu32]);
    e.set(this, TileTemplateItem::u, trait_number);
    e.set(this, TileTemplateItem::iLine, line);
    this
}

// Translated from 00a0c300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::rfind(const char *text, size_t pos, size_t length)` (the
/// map has no name; `_DebugHeapAllocator` string, Visual C++ 2005): with an
/// empty needle the answer is `pos` limited to the string's length; else the
/// last position at or before `pos` (and at most `size - length`) where the
/// needle's `length` characters match, or `npos` (the word at `010173f0`).
pub fn fn_00a0c300(e: &mut Engine, this: Ptr, text: Ptr, pos: u32, length: u32) -> u32 {
    let string = this.addr();
    let size = e.mem.u32(string + 0x14);
    if length == 0 {
        return pos.min(size);
    }
    if length <= size {
        let data = std_string_data(e, string);
        let mut candidate = data + pos.min(size - length);
        loop {
            let first_equal = e.mem.u8(candidate) as i8 == e.mem.u8(text.addr()) as i8;
            if first_equal && e.call(MEMCMP, &args![candidate, text, length]).i32() == 0 {
                return candidate - data;
            }
            if candidate == data {
                break;
            }
            candidate -= 1;
        }
    }
    e.global::<u32>(STD_STRING_NPOS)
}

// Translated from 00a0c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::append(const char *text, size_t length)` (the map has no
/// name): when `text` points into the string itself, appends the
/// corresponding range ([`fn_00a0c570`] with the offset); else checks the
/// new length (the length-error thrower when it would overflow), grows the
/// buffer (`_Grow`), copies the characters with `memcpy_s`, sets the length
/// and the terminating NUL. Answers `this`.
pub fn fn_00a0c430(e: &mut Engine, this: Ptr, text: Ptr, length: u32) -> Ptr {
    let string = this.addr();
    if e.call(STD_STRING_INSIDE, &args![this, text]).bool() {
        let offset = text.addr().wrapping_sub(std_string_data(e, string));
        return fn_00a0c570(e, this, this, offset, length);
    }
    let size = e.mem.u32(string + 0x14);
    let npos = e.global::<u32>(STD_STRING_NPOS);
    if npos.wrapping_sub(size) <= length || size.wrapping_add(length) < size {
        e.call(STD_LENGTH_ERROR, &[]);
    }
    if length != 0 {
        let new_size = size.wrapping_add(length);
        if e.call(STD_STRING_GROW, &args![this, new_size, false])
            .bool()
        {
            let data = std_string_data(e, string);
            let capacity = e.mem.u32(string + 0x18);
            let size = e.mem.u32(string + 0x14);
            e.call(
                MEMCPY_S,
                &args![data + size, capacity.wrapping_sub(size), text, length],
            );
            e.mem.set_u32(string + 0x14, new_size);
            let data = std_string_data(e, string);
            e.mem.set_u8(data + new_size, 0);
        }
    }
    this
}

// Translated from 00a0c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::append(const string &other, size_t offset, size_t count)`
/// (the map has no name): throws out-of-range when `offset` is beyond the
/// other string, limits `count` to what remains, throws length-error when
/// the result would overflow, grows the buffer and copies the characters
/// with `memcpy_s`. Answers `this`.
pub fn fn_00a0c570(e: &mut Engine, this: Ptr, other: Ptr, offset: u32, count: u32) -> Ptr {
    let (string, other_string) = (this.addr(), other.addr());
    let other_size = e.mem.u32(other_string + 0x14);
    if other_size < offset {
        e.call(STD_OUT_OF_RANGE, &[]);
    }
    let remaining = e.mem.u32(other_string + 0x14).wrapping_sub(offset);
    let count = count.min(remaining);
    let size = e.mem.u32(string + 0x14);
    let npos = e.global::<u32>(STD_STRING_NPOS);
    if npos.wrapping_sub(size) <= count || size.wrapping_add(count) < size {
        e.call(STD_LENGTH_ERROR, &[]);
    }
    if count != 0 {
        let new_size = size.wrapping_add(count);
        if e.call(STD_STRING_GROW, &args![this, new_size, false])
            .bool()
        {
            let source = std_string_data(e, other_string);
            let data = std_string_data(e, string);
            let capacity = e.mem.u32(string + 0x18);
            let size = e.mem.u32(string + 0x14);
            e.call(
                MEMCPY_S,
                &args![
                    data + size,
                    capacity.wrapping_sub(size),
                    source + offset,
                    count
                ],
            );
            e.mem.set_u32(string + 0x14, new_size);
            let data = std_string_data(e, string);
            e.mem.set_u8(data + new_size, 0);
        }
    }
    this
}

// Translated from 00a0c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash map's `SetAt(key, value)` (the map has no name; shared by the
/// `int` to `int` map `xTraitExtraData` and the reaction map): the bucket
/// of the key (vtable slot +4), then the chain is searched with the key
/// comparison (slot +8); a match gets the new value, else a node is made
/// (slot +0x14), filled (slot +0xc) and linked at the head of its bucket,
/// and the count at +0xc grows.
pub fn fn_00a0c6c0(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let map = this.addr();
    let index = e.vcall(map, MAP_SLOT_HASH, &args![key]).u32();
    let mut node = e.mem.u32(e.mem.u32(map + 8) + index * 4);
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, MAP_SLOT_KEYS_EQUAL, &args![key, node_key])
            .bool()
        {
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(map, MAP_SLOT_NEW_NODE, &[]).u32();
    e.vcall(map, MAP_SLOT_INIT_NODE, &args![node, key, value]);
    let slot = e.mem.u32(map + 8) + index * 4;
    let old_head = e.mem.u32(slot);
    e.mem.set_u32(node, old_head);
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count + 1);
}

// Translated from 00a0c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a `BSSimpleList` (the map has no name): [`SIMPLE_LIST_REMOVE_ALL`].
pub fn fn_00a0c7a0(e: &mut Engine, this: Ptr) {
    e.call(SIMPLE_LIST_REMOVE_ALL, &args![this]);
}

// Translated from 00a0c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<Tile::FadeControl *>::AddHead` (Xbox PDB): does nothing when
/// the item (`*item`) is null; an empty list (first item null) takes it in
/// the embedded node; otherwise a new 8-byte node is made that takes over
/// the first item and the old link, becomes the second node, and the first
/// node takes the new item.
pub fn bs_simple_list_fade_control_add_head(e: &mut Engine, this: Ptr<BSSimpleList>, item: Ptr) {
    let new_item = e.mem.u32(item.addr());
    if new_item == 0 {
        return;
    }
    if e.get(this, BSSimpleList::m_item) == 0 {
        e.set(this, BSSimpleList::m_item, new_item);
        return;
    }
    let node = allocate(e, 8).addr();
    // The game would write through a null node if the allocator failed.
    let first = e.get(this, BSSimpleList::m_item);
    e.mem.set_u32(node, first);
    e.mem.set_u32(node + 4, 0);
    let next = e.get(this, BSSimpleList::m_pkNext);
    e.mem.set_u32(node + 4, next);
    e.set(this, BSSimpleList::m_pkNext, node);
    e.set(this, BSSimpleList::m_item, new_item);
}

// Translated from 00a0c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hash map iteration step `(map, &node, &key, &value)` (the map has no
/// name): answers the key (+4) and value (+8) of `*node` and moves `*node`
/// to the next node of its chain, else to the first node of the following
/// non-empty bucket (the bucket of the key comes from vtable slot +4), else
/// to null.
pub fn fn_00a0c850(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    node_slot: u32,
    key_out: u32,
    value_out: u32,
) {
    let map = this.addr();
    let node = e.mem.u32(node_slot);
    let key = e.mem.u32(node + 4);
    e.mem.set_u32(key_out, key);
    let value = e.mem.u32(node + 8);
    e.mem.set_u32(value_out, value);
    let next = e.mem.u32(node);
    if next != 0 {
        e.mem.set_u32(node_slot, next);
        return;
    }
    let mut index = e
        .vcall(map, MAP_SLOT_HASH, &args![key])
        .u32()
        .wrapping_add(1);
    loop {
        if index >= e.mem.u32(map + 4) {
            e.mem.set_u32(node_slot, 0);
            return;
        }
        let candidate = e.mem.u32(e.mem.u32(map + 8) + index * 4);
        if candidate != 0 {
            e.mem.set_u32(node_slot, candidate);
            return;
        }
        index += 1;
    }
}

// Translated from 00a0c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text table's `SetAt(key, value)` (the map has no name; the same
/// search as [`fn_00a0c6c0`]): a match keeps its key when the map copies its
/// keys (byte at +0x10) and else takes the new key pointer, and gets the new
/// value; a new node comes from the free pool ([`LIST_NEW_NODE`] on the
/// allocator at +0xC) instead of the vtable.
pub fn fn_00a0c900(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let map = this.addr();
    let index = e.vcall(map, MAP_SLOT_HASH, &args![key]).u32();
    let mut node = e.mem.u32(e.mem.u32(map + 8) + index * 4);
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, MAP_SLOT_KEYS_EQUAL, &args![key, node_key])
            .bool()
        {
            if e.mem.u8(map + 0x10) == 0 {
                e.mem.set_u32(node + 4, key);
            }
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.call(LIST_NEW_NODE, &args![map + 0xc]).u32();
    e.vcall(map, MAP_SLOT_INIT_NODE, &args![node, key, value]);
    let slot = e.mem.u32(map + 8) + index * 4;
    let old_head = e.mem.u32(slot);
    e.mem.set_u32(node, old_head);
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count + 1);
}

// Translated from 00a0c9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList::AddHead(&item)` (no PDB name; the list's allocator is
/// at +8): a node from the free pool ([`LIST_NEW_NODE`]) holding `*item`
/// becomes the head, the previous head (if any) points back to it, else it
/// is also the tail; the count at +8 grows.
pub fn fn_00a0c9f0(e: &mut Engine, this: Ptr<NiTPointerList>, item: Ptr) {
    let node = e.call(LIST_NEW_NODE, &args![this.addr() + 8]).u32();
    let element = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, element);
    e.mem.set_u32(node + 4, 0);
    let head = e.get(this, NiTPointerList::m_pkHead);
    e.mem.set_u32(node, head);
    if e.get(this, NiTPointerList::m_pkHead) == 0 {
        e.set(this, NiTPointerList::m_pkTail, node);
    } else {
        let head = e.get(this, NiTPointerList::m_pkHead);
        e.mem.set_u32(head + 4, node);
    }
    e.set(this, NiTPointerList::m_pkHead, node);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count + 1);
}

// Translated from 00a0ca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList::AddAfter(node, &item)` (no PDB name): links a new node
/// holding `*item` behind `node` (it becomes the tail when `node` was the
/// last) and counts it. Answers the new node.
pub fn fn_00a0ca70(e: &mut Engine, this: Ptr<NiTPointerList>, after: u32, item: Ptr) -> u32 {
    let node = e.call(LIST_NEW_NODE, &args![this.addr() + 8]).u32();
    let element = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, element);
    e.mem.set_u32(node + 4, after);
    let following = e.mem.u32(after);
    e.mem.set_u32(node, following);
    if e.mem.u32(after) != 0 {
        let following = e.mem.u32(after);
        e.mem.set_u32(following + 4, node);
    } else {
        e.set(this, NiTPointerList::m_pkTail, node);
    }
    e.mem.set_u32(after, node);
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count + 1);
    node
}

// Translated from 00a0caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Tile::Value *, 8>::SortedInsert(&value, compare)` (the map
/// has no name): a binary search for the place of `value` with the
/// comparison function `compare(&value, &element)` (`cdecl`, answering -1, 0
/// or 1), then the insert there ([`fn_00a0cd50`]). A match ends the search
/// at its index. Any other answer from the comparison would make the game
/// loop forever; here it stops with a panic.
pub fn fn_00a0caf0(e: &mut Engine, this: Ptr<BSSimpleArray>, value: u32, compare: u32) {
    let mut low: i32 = 0;
    let mut high: i32 = e.get(this, BSSimpleArray::iSize) as i32 - 1;
    while low <= high {
        let middle = ((high - low) >> 1) + low;
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let answer = e
            .call(
                compare,
                &args![value, buffer.wrapping_add_signed(middle * 4)],
            )
            .i32();
        match answer {
            -1 => high = middle - 1,
            0 => {
                low = middle;
                high = -1;
            }
            1 => low = middle + 1,
            other => panic!("comparison answered {other}: the game loops forever"),
        }
    }
    fn_00a0cd50(e, this, low as u32, value);
}

// Translated from 00a0cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Tile::Value *, 8>::SortedFind` (Xbox PDB): `bsearch` of
/// the key in the buffer (4-byte elements, `compare` as the comparison);
/// answers the element's index or -1 when there is none.
pub fn bs_simple_array_tile_value_sorted_find(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    key: u32,
    compare: u32,
) -> i32 {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let size = e.get(this, BSSimpleArray::iSize);
    let found = e
        .call(BSEARCH, &args![key, buffer, size, 4u32, compare])
        .u32();
    if found == 0 {
        -1
    } else {
        (found.wrapping_sub(buffer) >> 2) as i32
    }
}

// Translated from 00a0cbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<Tile::FadeControl *>::Remove` (Xbox PDB): does nothing for
/// a null item or an empty list; else finds the first node holding `*item`.
/// If it is the embedded first node, the second node's item and link move
/// into it (the second node is destroyed and freed), or the item is cleared
/// when there is no second node; any other node is unlinked, destroyed
/// ([`SIMPLE_LIST_REMOVE_ALL`] on it) and freed. The folded `AND 1` before
/// each free is always set.
pub fn bs_simple_list_fade_control_remove(e: &mut Engine, this: Ptr<BSSimpleList>, item: Ptr) {
    let wanted = e.mem.u32(item.addr());
    if wanted == 0 {
        return;
    }
    let list = this.addr();
    if e.get(this, BSSimpleList::m_pkNext) == 0 && e.get(this, BSSimpleList::m_item) == 0 {
        return;
    }
    let mut node = list;
    let mut previous = list;
    while node != 0 && e.mem.u32(node) != wanted {
        previous = node;
        node = e.mem.u32(node + 4);
    }
    if node == 0 {
        return;
    }
    if node == list {
        let second = e.get(this, BSSimpleList::m_pkNext);
        if second == 0 {
            e.set(this, BSSimpleList::m_item, 0);
        } else {
            let after_second = e.mem.u32(second + 4);
            e.set(this, BSSimpleList::m_pkNext, after_second);
            let second_item = e.mem.u32(second);
            e.set(this, BSSimpleList::m_item, second_item);
            e.mem.set_u32(second + 4, 0);
            e.call(SIMPLE_LIST_REMOVE_ALL, &args![second]);
            deallocate(e, second);
        }
    } else {
        let following = e.mem.u32(node + 4);
        e.mem.set_u32(previous + 4, following);
        e.mem.set_u32(node + 4, 0);
        e.call(SIMPLE_LIST_REMOVE_ALL, &args![node]);
        deallocate(e, node);
    }
}

// Translated from 00a0cd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T *>::Insert(index, &item)` (the map has no name): at the
/// end it appends ([`fn_00a0cf00`]); a full array gets a new buffer (the
/// capacity doubles up to 8 and then grows by 8; the vtable's allocate
/// slot), the elements before and after the index are copied around the gap
/// ([`fn_00a0cf60`]) and the old buffer is freed (slot +8); with room the
/// tail is moved up by one. Then the size grows and `*item` goes to the
/// index. The compiler's empty loops are left out.
pub fn fn_00a0cd50(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, item: u32) {
    let array = this.addr();
    let size = e.get(this, BSSimpleArray::iSize);
    if index == size {
        fn_00a0cf00(e, this, item);
        return;
    }
    let capacity = e.get(this, BSSimpleArray::iReservedSize);
    if size == capacity {
        let new_capacity = if capacity > 8 {
            capacity + 8
        } else {
            capacity << 1
        };
        let new_buffer = e
            .vcall(array, ARRAY_SLOT_ALLOCATE, &args![new_capacity])
            .u32();
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_00a0cf60(e, array, new_buffer, buffer, index);
        fn_00a0cf60(
            e,
            array,
            new_buffer + index * 4 + 4,
            buffer + index * 4,
            size - index,
        );
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.vcall(array, ARRAY_SLOT_FREE, &args![buffer]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, new_capacity);
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_00a0cf60(
            e,
            array,
            buffer + index * 4 + 4,
            buffer + index * 4,
            size - index,
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size + 1);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = e.mem.u32(item);
    e.mem.set_u32(buffer + index * 4, element);
}

// Translated from 00a0cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T *>::Add(&item)` (the map has no name): makes room for
/// one more element ([`fn_00a0d000`]) and stores `*item` there. Answers the
/// index. The compiler's empty loop is left out.
pub fn fn_00a0cf00(e: &mut Engine, this: Ptr<BSSimpleArray>, item: u32) -> u32 {
    let index = fn_00a0d000(e, this);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let element = e.mem.u32(item);
    e.mem.set_u32(buffer + index * 4, element);
    index
}

// Translated from 00a0cf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `count` four-byte elements from `source` to `destination` (the map
/// has no name; `thiscall` but `this` is not read, it is the array the
/// caller works on): forwards when the destination is below the source,
/// backwards when above, one `memmove` of 4 bytes per element; nothing when
/// the two are equal or `count` is 0.
pub fn fn_00a0cf60(e: &mut Engine, _this: u32, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        for index in 0..count {
            e.call(
                MEMMOVE,
                &args![destination + index * 4, source + index * 4, 4u32],
            );
        }
    } else if destination > source {
        for index in (0..count).rev() {
            e.call(
                MEMMOVE,
                &args![destination + index * 4, source + index * 4, 4u32],
            );
        }
    }
}

// Translated from 00a0d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes room for one more element of a `BSSimpleArray` (the map has no
/// name): a full array without a capacity gets 4 elements (allocate slot),
/// a full one with a capacity grows ([`fn_00a0d0c0`]: doubled up to 8, then
/// by 8). Counts the element and answers its index.
pub fn fn_00a0d000(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    let array = this.addr();
    let size = e.get(this, BSSimpleArray::iSize);
    let capacity = e.get(this, BSSimpleArray::iReservedSize);
    if size == capacity {
        if capacity == 0 {
            let buffer = e.vcall(array, ARRAY_SLOT_ALLOCATE, &args![4u32]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, 4);
        } else {
            let new_capacity = if capacity > 8 {
                capacity + 8
            } else {
                capacity << 1
            };
            fn_00a0d0c0(e, this, new_capacity, size);
            e.set(this, BSSimpleArray::iReservedSize, new_capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize) + 1;
    e.set(this, BSSimpleArray::iSize, size);
    size - 1
}

// Translated from 00a0d0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives a `BSSimpleArray` a buffer of `new_capacity` elements (the map has
/// no name): without a buffer, a new one (allocate slot) and the capacity is
/// set; when `count` equals the capacity, the buffer is reallocated (slot
/// +0xC); else a new buffer is allocated, the `count` elements are copied
/// ([`fn_00a0cf60`]) and the old buffer is freed (slot +8). The capacity
/// field of a grown array is set by the caller.
pub fn fn_00a0d0c0(e: &mut Engine, this: Ptr<BSSimpleArray>, new_capacity: u32, count: u32) {
    let array = this.addr();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let fresh = e
            .vcall(array, ARRAY_SLOT_ALLOCATE, &args![new_capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, fresh);
        e.set(this, BSSimpleArray::iReservedSize, new_capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let moved = e
            .vcall(array, ARRAY_SLOT_REALLOCATE, &args![buffer, new_capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, moved);
    } else {
        let fresh = e
            .vcall(array, ARRAY_SLOT_ALLOCATE, &args![new_capacity])
            .u32();
        fn_00a0cf60(e, array, fresh, buffer, count);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.vcall(array, ARRAY_SLOT_FREE, &args![old]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, fresh);
    }
}

// Translated from 00a0d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::MenuStringMap::IsKeysEqual` (Xbox PDB): the two keys are equal
/// ignoring case (`_stricmp` is 0).
pub fn menu_string_map_is_keys_equal(e: &mut Engine, _this: Ptr, a: Ptr, b: Ptr) -> bool {
    e.call(STRICMP, &args![a, b]).i32() == 0
}

// Translated from 00a0d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::MenuStringMap::KeyToHashIndex` (Xbox PDB): `hash = hash * 33 +
/// tolower(c)` over the characters (as signed bytes), reduced by the
/// bucket count at +4.
pub fn menu_string_map_key_to_hash_index(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let mut hash: u32 = 0;
    let mut at = key.addr();
    while e.mem.u8(at) as i8 != 0 {
        let character = e.mem.u8(at) as i8 as i32;
        let lower = e.call(TOLOWER, &args![character]).u32() as u8 as i8;
        hash = hash.wrapping_mul(33).wrapping_add(lower as i32 as u32);
        at += 1;
    }
    bucket_index(e, this.addr(), hash)
}

// Translated from 00a0d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::MenuStringMap::_scalar_deleting_destructor_` (Xbox PDB): sets the
/// class's vtable, runs the destructor of the string map
/// ([`ni_t_string_template_map_destructor`]) and, with bit 0 of `flags`,
/// frees the object.
pub fn menu_string_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_MENU_STRING_MAP);
    ni_t_string_template_map_destructor(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0d260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hash of an integer key (the map has no name): the key reduced by the
/// bucket count at +4.
pub fn fn_00a0d260(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    bucket_index(e, this.addr(), key)
}

// Translated from 00a0d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map's node allocation (vtable slot +0x14; the map has no name): a node
/// from the free pool of the allocator at +0xC ([`LIST_NEW_NODE`]).
pub fn fn_00a0d280(e: &mut Engine, this: Ptr) -> u32 {
    e.call(LIST_NEW_NODE, &args![this.addr() + 0xc]).u32()
}

// Translated from 00a0d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *, int>, int>::
/// ~NiTStringTemplateMap` (Xbox PDB): with the copy-keys byte (+0x10) set,
/// frees the key of every node of every bucket; then the destructors of the
/// two base levels run ([`MAP_REMOVE_ALL`] under the vtable of each level)
/// and the bucket array is freed. The C++ exception frame is not
/// translated.
pub fn ni_t_string_template_map_destructor(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_STRING_TEMPLATE_MAP);
    if e.mem.u8(map + 0x10) != 0 {
        let mut bucket = 0;
        while bucket < e.mem.u32(map + 4) {
            let mut node = e.mem.u32(e.mem.u32(map + 8) + bucket * 4);
            while node != 0 {
                let following = e.mem.u32(node);
                let key = e.mem.u32(node + 4);
                e.call(FREE_BYTES, &args![key]);
                node = following;
            }
            bucket += 1;
        }
    }
    fn_00a0d3c0(e, this);
}

// Translated from 00a0d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the map's base class `NiTMapBase<NiTPointerAllocator<
/// unsigned int>, char const *, int>` (the map has no name): sets the base
/// vtable, empties the map ([`MAP_REMOVE_ALL`]) and frees the bucket array.
pub fn fn_00a0d390(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BYTES, &args![buckets]);
}

// Translated from 00a0d3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTPointerMap<char const *, int>` (the map has no
/// name): empties the map under its own vtable, then runs the base
/// destructor ([`fn_00a0d390`]). The C++ exception frame is not translated.
pub fn fn_00a0d3c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_POINTER_MAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00a0d390(e, this);
}

// Translated from 00a0d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hash of a string key, case sensitive (the map has no name): `hash =
/// hash * 33 + c` over the characters (as signed bytes), reduced by the
/// bucket count at +4.
pub fn fn_00a0d440(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let mut hash: u32 = 0;
    let mut at = key.addr();
    while e.mem.u8(at) as i8 != 0 {
        hash = hash
            .wrapping_mul(33)
            .wrapping_add(e.mem.u8(at) as i8 as i32 as u32);
        at += 1;
    }
    bucket_index(e, this.addr(), hash)
}

// Translated from 00a0d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map node's key release (the map has no name): when the map copies its
/// keys (byte at +0x10), frees the key of `node` (+4).
pub fn fn_00a0d4a0(e: &mut Engine, this: Ptr, node: u32) {
    if e.mem.u8(this.addr() + 0x10) != 0 {
        let key = e.mem.u32(node + 4);
        e.call(FREE_BYTES, &args![key]);
    }
}

// Translated from 00a0d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const *, int>::
/// _scalar_deleting_destructor_` (Xbox PDB): the base destructor
/// ([`fn_00a0d390`]) and, with bit 0 of `flags`, frees the object.
pub fn ni_t_map_base_char_p_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a0d390(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *, int>::_scalar_deleting_destructor_` (Xbox
/// PDB): [`fn_00a0d3c0`] and, with bit 0 of `flags`, frees the object. The
/// C++ exception frame is not translated.
pub fn ni_t_pointer_map_char_p_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a0d3c0(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *, int>, int>::
/// _scalar_deleting_destructor_` (Xbox PDB): the destructor
/// ([`ni_t_string_template_map_destructor`]) and, with bit 0 of `flags`,
/// frees the object.
pub fn ni_t_string_template_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_t_string_template_map_destructor(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base class `NiTMapBase<..., char const *, int>`
/// (the map has no name): sets the base vtable and the bucket count, no
/// items, allocates `4 * buckets` bytes for the buckets and clears them.
/// Answers `this`.
pub fn fn_00a0d5f0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_MAP_BASE);
    e.mem.set_u32(map + 4, bucket_count);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOC_BYTES, &args![e.mem.u32(map + 4) << 2]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4) << 2;
    let buckets = e.mem.u32(map + 8);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    this
}

// Translated from 00a0d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int, int>::_scalar_deleting_destructor_` (Xbox PDB):
/// empties the map under the vtable of each of its two levels
/// ([`MAP_REMOVE_ALL`]), frees the bucket array and, with bit 0 of `flags`,
/// frees the object. The C++ exception frame is not translated.
pub fn ni_t_pointer_map_int_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_INT_POINTER_MAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(map, VTABLE_INT_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(map + 8);
    e.call(FREE_BYTES, &args![buckets]);
    if flags & 1 != 0 {
        deallocate(e, map);
    }
    this
}

// Translated from 00a0d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base class of `NiTPointerMap<int, int>` (the map
/// has no name): the same as [`fn_00a0d5f0`] with the vtable `01094e5c`.
pub fn fn_00a0d710(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_INT_MAP_BASE);
    e.mem.set_u32(map + 4, bucket_count);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOC_BYTES, &args![e.mem.u32(map + 4) << 2]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4) << 2;
    let buckets = e.mem.u32(map + 8);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    this
}

// Translated from 00a0d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the map base class of `NiTPointerMap<int, int>` (the
/// map has no name): sets the base vtable, empties the map
/// ([`MAP_REMOVE_ALL`]) and frees the bucket array.
pub fn fn_00a0d790(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_INT_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BYTES, &args![buckets]);
}

// Translated from 00a0d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map node's release (the map has no name): clears the node's value (+8)
/// and gives the node back to the free pool of the allocator at +0xC of the
/// map ([`MAP_FREE_NODE`]).
pub fn fn_00a0d7c0(e: &mut Engine, this: Ptr, node: u32) {
    e.mem.set_u32(node + 8, 0);
    e.call(MAP_FREE_NODE, &args![this.addr() + 0xc, node]);
}

// Translated from 00a0d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>, int, int>::
/// _scalar_deleting_destructor_` (Xbox PDB): the base destructor
/// ([`fn_00a0d790`]) and, with bit 0 of `flags`, frees the object.
pub fn ni_t_map_base_int_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a0d790(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

// Translated from 00a0d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<Tile::Value *, Tile::Reaction *>::
/// _scalar_deleting_destructor_` (Xbox PDB): empties the map under the
/// vtable of each of its two levels ([`MAP_REMOVE_ALL`]), frees the bucket
/// array and, with bit 0 of `flags`, frees the object. The C++ exception
/// frame is not translated.
pub fn ni_t_pointer_map_value_reaction_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_REACTION_POINTER_MAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(map, VTABLE_REACTION_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(map + 8);
    e.call(FREE_BYTES, &args![buckets]);
    if flags & 1 != 0 {
        deallocate(e, map);
    }
    this
}

// Translated from 00a0d8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base class of `NiTPointerMap<Tile::Value *,
/// Tile::Reaction *>` (the map has no name): the same as [`fn_00a0d5f0`]
/// with the vtable `01094e9c`.
pub fn fn_00a0d8e0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, VTABLE_REACTION_MAP_BASE);
    e.mem.set_u32(map + 4, bucket_count);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOC_BYTES, &args![e.mem.u32(map + 4) << 2]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4) << 2;
    let buckets = e.mem.u32(map + 8);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    this
}

// Translated from 00a0d960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the map base class of `NiTPointerMap<Tile::Value *,
/// Tile::Reaction *>` (the map has no name): sets the base vtable, empties
/// the map ([`MAP_REMOVE_ALL`]) and frees the bucket array.
pub fn fn_00a0d960(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_REACTION_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BYTES, &args![buckets]);
}

// Translated from 00a0d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>, Tile::Value *,
/// Tile::Reaction *>::_scalar_deleting_destructor_` (Xbox PDB): the base
/// destructor ([`fn_00a0d960`]) and, with bit 0 of `flags`, frees the
/// object.
pub fn ni_t_map_base_value_reaction_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a0d960(e, this);
    if flags & 1 != 0 {
        deallocate(e, this.addr());
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009ff340, tile_destructor(Ptr<Tile>)),
        entry!(0x009ff600, tile_init(Ptr<Tile>, Ptr<Tile>, Ptr, u32)),
        entry!(0x009ff690, tile_release(Ptr<Tile>)),
        entry!(0x009ff8a0, fn_009ff8a0(Ptr, i32)),
        entry!(0x009ff970, tile_load_text_table()),
        entry!(0x00a00940, tile_add_user_trait(Ptr, i32) -> i32),
        entry!(0x00a00a30, tile_free_trait_list()),
        entry!(0x00a00b50, tile_set_menu_deleting(Ptr<Tile>)),
        entry!(0x00a00c20, tile_build_tile(u32) -> Ptr),
        entry!(0x00a00e90, tile_get_value_q(Ptr<Tile>, i32) -> Ptr),
        entry!(0x00a00f30, tile_get_value(Ptr<Tile>, i32) -> Ptr),
        entry!(0x00a01000, tile_get_or_create_value(Ptr<Tile>, i32) -> Ptr),
        entry!(0x00a01160, fn_00a01160(Ptr, Ptr) -> i32),
        entry!(0x00a011b0, fn_00a011b0(Ptr<Tile>, i32) -> f32),
        entry!(0x00a011f0, tile_get_string(Ptr<Tile>, i32) -> Ptr),
        entry!(0x00a01230, tile_is_true(Ptr<Tile>, i32) -> bool),
        entry!(0x00a01290, tile_poke(Ptr<Tile>, i32, f32)),
        entry!(0x00a012d0, fn_00a012d0(Ptr<Tile>, i32, f32, bool)),
        entry!(0x00a01350, fn_00a01350(Ptr<Tile>, i32, Ptr, bool)),
        entry!(0x00a013d0, fn_00a013d0(Ptr<Tile>) -> f32),
        entry!(0x00a01440, fn_00a01440(Ptr<Tile>) -> f32),
        entry!(0x00a014b0, fn_00a014b0(Ptr<Tile>) -> f32),
        entry!(0x00a01530, tile_get_maximum_depth() -> f32),
        entry!(0x00a01540, fn_00a01540(Ptr<Tile>) -> f32),
        entry!(0x00a01630, tile_get_tile_from_node(Ptr) -> Ptr<Tile>),
        entry!(0x00a01750, tile_sever_extra_data(Ptr<Tile>)),
        entry!(0x00a01860, tile_text_to_trait(Ptr) -> i32),
        entry!(0x00a018d0, fn_00a018d0(Ptr, Ptr) -> i32),
        entry!(0x00a01a20, fn_00a01a20(Ptr) -> i32),
        entry!(0x00a01a70, fn_00a01a70(i32) -> Ptr),
        entry!(0x00a01b00, tile_read_file(Ptr<Tile>, Ptr) -> Ptr<Tile>),
        entry!(0x00a01e20, tile_parse_file(Ptr, Ptr, u32) -> Ptr<BuildStorage>),
        entry!(
            0x00a02d40,
            tile_insert_included_files(Ptr<XmlStorage>) -> bool
        ),
        entry!(
            0x00a031b0,
            tile_build_and_name_tree(Ptr<Tile>, Ptr<TileTemplate>) -> Ptr<Tile>
        ),
        entry!(
            0x00a033b0,
            tile_connect_traits_to_tree(Ptr<Tile>, Ptr<TileTemplate>)
        ),
        entry!(
            0x00a037e0,
            tile_final_post_parse(Ptr<Tile>, i32, f32, u32) -> Ptr<Tile>
        ),
        entry!(0x00a03c60, tile_get_rendered_menu(Ptr<Tile>) -> bool),
        entry!(0x00a03c90, tile_get_menu(Ptr<Tile>) -> Ptr),
        entry!(0x00a03d40, tile_get_menu_tile(Ptr<Tile>) -> Ptr<Tile>),
        entry!(
            0x00a03da0,
            tile_get_child_by_name(Ptr<Tile>, Ptr) -> Ptr<Tile>
        ),
        entry!(
            0x00a03eb0,
            tile_get_child_by_id(Ptr<Tile>, i32) -> Ptr<Tile>
        ),
        entry!(
            0x00a03f70,
            tile_get_first_ref_copy(Ptr<Tile>, i32, Ptr) -> u32
        ),
        entry!(0x00a040a0, tile_is_visible(Ptr<Tile>) -> bool),
        entry!(0x00a04100, fn_00a04100(Ptr<Tile>) -> bool),
        entry!(0x00a04150, tile_delete_children(Ptr<Tile>)),
        entry!(0x00a04200, tile_update_all(bool)),
        entry!(0x00a044f0, tile_lock()),
        entry!(0x00a04500, tile_unlock()),
        entry!(0x00a04510, fn_00a04510()),
        entry!(0x00a04620, tile_update_children(Ptr<Tile>, u32)),
        entry!(0x00a04640, tile_update_tile(Ptr<Tile>, bool) -> bool),
        entry!(0x00a06dd0, tile_get_texture_atlas_info(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a074d0, tile_set_needs_update(Ptr<Tile>, u32)),
        entry!(0x00a07530, tile_add_needs_update(Ptr<Tile>, u32)),
        entry!(0x00a07580, tile_get_next_dirty_tile(bool) -> Ptr<Tile>),
        entry!(0x00a07690, tile_add_dirty_tile(Ptr<Tile>)),
        entry!(0x00a077c0, fn_00a077c0(Ptr<Tile>)),
        entry!(0x00a078e0, tile_check_hibernating_tiles()),
        entry!(0x00a079d0, tile_update_clipwindows(Ptr<Tile>)),
        entry!(
            0x00a07a40,
            tile_re_clip_children(Ptr<Tile>, f32, f32, f32, f32)
        ),
        entry!(
            0x00a07c60,
            tile_add_fade_control(Ptr<Tile>, i32, f32, f32, f32, i32)
        ),
        entry!(0x00a07dc0, tile_remove_fade_control(Ptr<Tile>, i32)),
        entry!(0x00a07ed0, tile_has_fade_control(Ptr<Tile>, i32) -> bool),
        entry!(0x00a07f30, tile_get_fade_end_for(Ptr<Tile>, i32) -> f32),
        entry!(0x00a07fc0, tile_get_fade_finished(Ptr<Tile>, i32) -> bool),
        entry!(0x00a08070, tile_get_fade_type(Ptr<Tile>, i32) -> i32),
        entry!(0x00a080d0, tile_update_fade_controls()),
        entry!(0x00a08720, tile_fade_in_3d(Ptr<Tile>, Ptr, f32)),
        entry!(0x00a087d0, tile_set_parent(Ptr<Tile>, Ptr<Tile>, Ptr<Tile>)),
        entry!(0x00a089c0, tile_get_parent_model(Ptr<Tile>) -> Ptr),
        entry!(0x00a08a20, tile_delete_model(Ptr<Tile>)),
        entry!(0x00a08b20, fn_00a08b20(Ptr<Tile>, Ptr) -> Ptr),
        entry!(0x00a08f20, fn_00a08f20(Ptr<Tile>, Ptr) -> Ptr<Tile>),
        entry!(0x00a08fb0, fn_00a08fb0(Ptr<Tile>, i32) -> Ptr<Tile>),
        entry!(0x00a09030, tile_get_menu_by_class(i32) -> u32),
        entry!(0x00a09080, value_add_action(Ptr<TileValue>, i32, f32)),
        entry!(
            0x00a09130,
            value_add_action_ov2(Ptr<TileValue>, i32, Ptr<Tile>, i32)
        ),
        entry!(0x00a09200, value_clear_actions(Ptr<TileValue>)),
        entry!(0x00a09330, value_destructor(Ptr<TileValue>)),
        entry!(0x00a09410, value_calculate_value(Ptr<TileValue>, bool)),
        entry!(
            0x00a0a0b0,
            tile_get_underscore_value(Ptr<Tile>, i32, i32) -> Ptr
        ),
        entry!(0x00a0a130, add_reaction(Ptr<TileValue>, Ptr<TileValue>)),
        entry!(0x00a0a220, tile_value_change_event(Ptr<TileValue>)),
        entry!(0x00a0a270, value_set_float(Ptr<TileValue>, f32, bool)),
        entry!(0x00a0a300, value_set_string(Ptr<TileValue>, Ptr, bool)),
        entry!(
            0x00a0a410,
            template_add_pair(Ptr<TileTemplate>, i32, Ptr, i32, bool)
        ),
        entry!(0x00a0ab70, template_clear(Ptr<TileTemplate>)),
        entry!(0x00a0abe0, extra_destructor(Ptr<Extra>)),
        entry!(
            0x00a0ac80,
            build_storage_construct(Ptr<BuildStorage>) -> Ptr<BuildStorage>
        ),
        entry!(0x00a0ad40, build_storage_destructor(Ptr<BuildStorage>)),
        entry!(
            0x00a0ae70,
            fn_00a0ae70(Ptr<BuildStorage>, Ptr) -> Ptr<TileTemplate>
        ),
        entry!(
            0x00a0af10,
            fn_00a0af10(Ptr<BuildStorage>, Ptr) -> Ptr<TileTemplate>
        ),
        entry!(0x00a0b020, tile_force_texture_release(Ptr<Tile>)),
        entry!(0x00a0b110, tile_play_tile_sound(Ptr<Tile>, i32)),
        entry!(0x00a0b1f0, tile_get_shader_property(Ptr<Tile>) -> Ptr),
        entry!(
            0x00a0b280,
            tile_set_alpha_and_color(Ptr<Tile>, Ptr, f32, Ptr)
        ),
        entry!(0x00a0b350, fn_00a0b350(Ptr<Tile>, i32, f32)),
        entry!(0x00a0b420, fn_00a0b420(Ptr<Tile>, Ptr, f32)),
        entry!(0x00a0b520, tile_recursive_rotation_update(Ptr<Tile>)),
        entry!(0x00a0b8d0, fn_00a0b8d0(Ptr)),
        entry!(
            0x00a0b950,
            fn_00a0b950(i32, f32, Ptr, i32, i32) -> Ptr<TileTemplateItem>
        ),
        entry!(0x00a0ba50, fn_00a0ba50()),
        entry!(
            0x00a0bae0,
            tile_special_bounds_check(Ptr<Tile>, u32, u32) -> bool
        ),
        entry!(0x00a0baf0, fn_00a0baf0(Ptr<Tile>)),
        entry!(0x00a0bb00, fn_00a0bb00(Ptr) -> bool),
        entry!(0x00a0bb50, fn_00a0bb50(Ptr)),
        entry!(0x00a0bbd0, tile_text_get_type(Ptr<Tile>) -> u32),
        entry!(0x00a0bbe0, tile_text_get_type_name(Ptr<Tile>) -> u32),
        entry!(
            0x00a0bbf0,
            tile_text_scalar_deleting_destructor(Ptr<Tile>, u32) -> Ptr<Tile>
        ),
        entry!(0x00a0bc80, fn_00a0bc80(Ptr<Tile>) -> Ptr<Tile>),
        entry!(0x00a0bdc0, tile_3d_get_type(Ptr<Tile>) -> u32),
        entry!(0x00a0bdd0, tile_3d_get_type_name(Ptr<Tile>) -> u32),
        entry!(
            0x00a0bde0,
            tile_3d_scalar_deleting_destructor(Ptr<Tile>, u32) -> Ptr<Tile>
        ),
        entry!(
            0x00a0be10,
            xml_storage_scalar_deleting_destructor(Ptr<XmlStorage>, u32) -> Ptr<XmlStorage>
        ),
        entry!(0x00a0be70, xml_storage_destructor(Ptr<XmlStorage>)),
        entry!(0x00a0beb0, fn_00a0beb0(Ptr) -> Ptr),
        entry!(0x00a0c000, float_action_q_float(Ptr<ValueAction>) -> f32),
        entry!(
            0x00a0c020,
            ref_value_action_q_float(Ptr<ValueAction>) -> f32
        ),
        entry!(0x00a0c060, fn_00a0c060(Ptr<ValueAction>) -> Ptr<TileValue>),
        entry!(0x00a0c080, ni_extra_data_get_rtti(Ptr) -> u32),
        entry!(
            0x00a0c090,
            extra_scalar_deleting_destructor(Ptr<Extra>, u32) -> Ptr<Extra>
        ),
        entry!(
            0x00a0c0c0,
            template_construct(Ptr<TileTemplate>, u32, Ptr<BuildStorage>) -> Ptr<TileTemplate>
        ),
        entry!(0x00a0c190, template_destructor(Ptr<TileTemplate>)),
        entry!(
            0x00a0c220,
            fn_00a0c220(Ptr<TileTemplateItem>, i32, f32, Ptr, u32, i32) -> Ptr<TileTemplateItem>
        ),
        entry!(0x00a0c300, fn_00a0c300(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00a0c430, fn_00a0c430(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00a0c570, fn_00a0c570(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00a0c6c0, fn_00a0c6c0(Ptr<NiTPointerMap>, u32, u32)),
        entry!(0x00a0c7a0, fn_00a0c7a0(Ptr)),
        entry!(
            0x00a0c7c0,
            bs_simple_list_fade_control_add_head(Ptr<BSSimpleList>, Ptr)
        ),
        entry!(0x00a0c850, fn_00a0c850(Ptr<NiTPointerMap>, u32, u32, u32)),
        entry!(0x00a0c900, fn_00a0c900(Ptr<NiTPointerMap>, u32, u32)),
        entry!(0x00a0c9f0, fn_00a0c9f0(Ptr<NiTPointerList>, Ptr)),
        entry!(
            0x00a0ca70,
            fn_00a0ca70(Ptr<NiTPointerList>, u32, Ptr) -> u32
        ),
        entry!(0x00a0caf0, fn_00a0caf0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x00a0cb90,
            bs_simple_array_tile_value_sorted_find(Ptr<BSSimpleArray>, u32, u32) -> i32
        ),
        entry!(
            0x00a0cbe0,
            bs_simple_list_fade_control_remove(Ptr<BSSimpleList>, Ptr)
        ),
        entry!(0x00a0cd50, fn_00a0cd50(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00a0cf00, fn_00a0cf00(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x00a0cf60, fn_00a0cf60(u32, u32, u32, u32)),
        entry!(0x00a0d000, fn_00a0d000(Ptr<BSSimpleArray>) -> u32),
        entry!(0x00a0d0c0, fn_00a0d0c0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x00a0d180,
            menu_string_map_is_keys_equal(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a0d1b0,
            menu_string_map_key_to_hash_index(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a0d220,
            menu_string_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a0d260, fn_00a0d260(Ptr, u32) -> u32),
        entry!(0x00a0d280, fn_00a0d280(Ptr) -> u32),
        entry!(0x00a0d2a0, ni_t_string_template_map_destructor(Ptr)),
        entry!(0x00a0d390, fn_00a0d390(Ptr)),
        entry!(0x00a0d3c0, fn_00a0d3c0(Ptr)),
        entry!(0x00a0d440, fn_00a0d440(Ptr, Ptr) -> u32),
        entry!(0x00a0d4a0, fn_00a0d4a0(Ptr, u32)),
        entry!(
            0x00a0d4d0,
            ni_t_map_base_char_p_int_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a0d520,
            ni_t_pointer_map_char_p_int_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a0d5c0,
            ni_t_string_template_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a0d5f0, fn_00a0d5f0(Ptr, u32) -> Ptr),
        entry!(
            0x00a0d670,
            ni_t_pointer_map_int_int_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a0d710, fn_00a0d710(Ptr, u32) -> Ptr),
        entry!(0x00a0d790, fn_00a0d790(Ptr)),
        entry!(0x00a0d7c0, fn_00a0d7c0(Ptr, u32)),
        entry!(
            0x00a0d7f0,
            ni_t_map_base_int_int_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a0d840,
            ni_t_pointer_map_value_reaction_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a0d8e0, fn_00a0d8e0(Ptr, u32) -> Ptr),
        entry!(0x00a0d960, fn_00a0d960(Ptr)),
        entry!(
            0x00a0d990,
            ni_t_map_base_value_reaction_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// Every callee outside this file, given a do-nothing double by
    /// [`tile_engine`] (tests replace the ones they care about).
    const OUTSIDE_CALLEES: &[u32] = &[
        ATEXIT,
        ENTER_CRITICAL_SECTION,
        LEAVE_CRITICAL_SECTION,
        TILE_LOCK,
        TILE_UNLOCK,
        TILE_SET_PARENT,
        TILE_REMOVE_FADE_CONTROL,
        TILE_ADD_NEEDS_UPDATE,
        TILE_UPDATE_CHILDREN,
        VALUE_DESTROY,
        VALUE_SET_FLOAT,
        VALUE_SET_STRING,
        VALUE_ADD_ACTION,
        VALUE_ADD_ACTION_OV2,
        VALUE_CALCULATE_VALUE,
        VALUES_CLEAR,
        VALUES_DESTROY,
        LIST_REMOVE_HEAD,
        LIST_REMOVE_ALL,
        LIST_REMOVE_ITEM,
        INTERFACE_TILE_IS_BEING_DELETED,
        INTERFACE_GET_MENUS_ROOT,
        INTERFACE_TEST_CONSTANT_FOR_GAME_SETTINGS,
        INTERFACE_IS_WIDESCREEN,
        NI_POINTER_ASSIGN,
        STRING_SET,
        STRING_APPEND,
        STRING_ASSIGN,
        STRING_FORMAT,
        PRINT_ERROR,
        NI_OBJECT_GET_EXTRA_DATA,
        MENU_GET_XML,
        MENU_GET_MAX_DEPTH,
        MENU_ADD_TEMPLATE,
        XML_STORAGE_DESTROY,
        XML_STORAGE_SCALAR_DELETING_DESTRUCTOR,
        BUILD_STORAGE_DESTROY,
        TEMPLATE_ADD_PAIR,
        TILE_FIND_BY_PATH,
        TEXT_TABLE_FIND,
        TEXT_TABLE_ADD,
        TRAIT_EXTRA_DATA_ADD,
        TEXT_TABLE_NEXT,
        TEXT_IS_USER_NAME,
        TEXTURE_ATLAS_ENTRY_DESTROY,
        MAP_REMOVE_ALL,
        SIMPLE_LIST_REMOVE_ALL,
        MODEL_HOOK_00C45830,
        STRSTR,
        STRRCHR,
        STRCMP,
        STRTOK_S,
        ADD_TILE_TO_UPDATE_LIST,
        INTERFACE_MANAGER_END_OF_UPDATE,
        INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE,
        INTERFACE_PREPARE_TEXTURE_RELEASE,
        TEXTURE_PALETTE_PURGE_UNUSED,
        SCREEN_REAL_WIDTH,
        SCREEN_REAL_HEIGHT,
        RENDERED_MENU_WIDTH,
        RENDERED_MENU_HEIGHT,
        INTERFACE_00706CF0,
        NI_OBJECT_GET_PROPERTY,
        NI_ARRAY_COMPACT,
        NI_ARRAY_UPDATE_SIZE,
        SHADER_PROPERTY_REFRESH,
        LIST_REMOVE_NODE,
        LIST_ADD_TAIL,
        LIST_NEW_NODE,
        LIST_ADD_NODE_TAIL,
        LIST_ADD_HEAD,
        LIST_ADD_AFTER,
        FADE_LIST_ADD_HEAD,
        FADE_LIST_REMOVE,
        REACTION_MAP_REMOVE,
        ADD_REACTION,
        TICK_COUNT_IMPORT,
        FABS,
        FLOOR,
        CEIL,
        TEXT_TILE_CREATED,
        SET_TILE_TEXTURE,
        ADD_TO_TES_TEXTURES,
        BSSTRING_CONSTRUCT,
        GEOMETRY_DATA_MARK_AS_CHANGED,
        GEOMETRY_DATA_SET_CONSISTENCY,
        BOUND_COMPUTE_FROM_DATA,
        TILE_3D_UPDATE_NIF,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SCREEN_ASPECT_RATIO,
        INTERFACE_IS_RENDERED_MENU,
        MATRIX_TIMES_POINT,
        POINT_ADD,
        POINT_SUBTRACT,
        SYSTEM_COLOR_MANAGER_GET_INSTANCE,
        SYSTEM_COLOR_MANAGER_GET_COLOR,
        COLOR_SCALE,
        VALUE_CHANGE_EVENT,
        TILE_GET_UNDERSCORE_VALUE,
        FLOAT_STACK_CONSTRUCT,
        FLOAT_STACK_PUSH,
        FLOAT_STACK_DESTROY,
        SPRINTF_S,
        STD_STRING_TIDY,
        STD_STRING_ASSIGN,
        STD_STRING_RFIND,
        STD_STRING_SUBSTR,
        STD_STRING_ASSIGN_STRING,
        STD_STRING_APPEND,
        TEXTURE_ATLAS_ENTRY_CONSTRUCT,
        FILE_FINDER_GET_FILE,
        STRTOK,
        SSCANF,
        TEMPLATE_LIST_ADD_NODE_TAIL,
        TEMPLATE_LIST_POP_HEAD,
        TEMPLATE_LIST_POP_TAIL,
        TEMPLATE_ITEM_CONSTRUCT,
        TEMPLATE_CONSTRUCT,
        TEMPLATE_DESTROY,
        SUB_TEMPLATE_LIST_ADD_HEAD,
        NI_FIXED_STRING_CREATE,
        NI_EXTRA_DATA_DESTROY,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        MATRIX_MAKE_Y_ROTATION,
        TILE_3D_DESTROY,
        NI_OPERATOR_DELETE,
        LIST_DESTROY,
        STD_STRING_INSIDE,
        STD_STRING_GROW,
        STD_LENGTH_ERROR,
        STD_OUT_OF_RANGE,
        MEMCPY_S,
        MEMMOVE,
        BSEARCH,
        TOLOWER,
        MAP_FREE_NODE,
        ALLOC_BYTES,
        FREE_BYTES,
    ];

    /// An engine with do-nothing doubles for everything outside this file,
    /// constructors that return `this`, interlocked operations on memory,
    /// and the pages of the globals the code reads mapped (zeroed).
    fn tile_engine() -> Engine {
        let mut e = Engine::new();
        for &addr in OUTSIDE_CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in [
            TILE_BASE_CONSTRUCT,
            TILE_IMAGE_CONSTRUCT,
            TILE_3D_CONSTRUCT,
            RADIAL_TILE_CONSTRUCT,
            BUILD_STORAGE_CONSTRUCT,
        ] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        // `atol` and `atof` reach the CRT's `strtol` (00ec8d08) and `_atof_l` (00eca4c8),
        // which read the initial locale's tables from the image; the bare engine has no
        // image, so both are doubled with decimal parsers (leading blanks, a sign, digits).
        e.register(0x00ec_8d08, |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).to_string();
            let text = text.trim_start();
            let end = text
                .char_indices()
                .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && (c == '-' || c == '+'))))
                .map_or(text.len(), |(i, _)| i);
            text[..end].parse::<i32>().unwrap_or(0).into_ret()
        });
        e.register(0x00ec_a4c8, |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).to_string();
            let text = text.trim_start();
            let end = text
                .char_indices()
                .find(|&(i, c)| {
                    !(c.is_ascii_digit() || c == '.' || (i == 0 && (c == '-' || c == '+')))
                })
                .map_or(text.len(), |(i, _)| i);
            Ret {
                st0: text[..end].parse::<f64>().unwrap_or(0.0),
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        for page in [
            0x011a_6000,
            0x011d_8000,
            0x011f_3000,
            0x011f_4000,
            0x0101_2000,
            0x0102_3000,
            0x0109_4000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    /// The `Tile::Value *` array doubles: a sorted search by trait id and a
    /// sorted insert (the arrays the game keeps for a tile's traits).
    fn install_value_array_doubles(e: &mut Engine) {
        e.register(VALUES_SORTED_FIND, |e, a| {
            let key = e.mem.i32(e.mem.u32(a[1]));
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            for index in 0..size {
                if e.mem.i32(e.mem.u32(buffer + 4 * index)) == key {
                    return (index as i32).into_ret();
                }
            }
            (-1i32).into_ret()
        });
        e.register(VALUES_SORTED_INSERT, |e, a| {
            let value = e.mem.u32(a[1]);
            insert_value(e, a[0], value);
            Ret::default()
        });
    }

    /// Inserts `value` into the sorted `Tile::Value *` array at `array`.
    fn insert_value(e: &mut Engine, array: u32, value: u32) {
        let id = e.mem.i32(value);
        let mut buffer = e.mem.u32(array + 4);
        if buffer == 0 {
            buffer = e.mem.alloc(0x80);
            e.mem.set_u32(array + 4, buffer);
            e.mem.set_u32(array + 0xc, 32);
        }
        let size = e.mem.u32(array + 8);
        let mut at = 0;
        while at < size && e.mem.i32(e.mem.u32(buffer + 4 * at)) < id {
            at += 1;
        }
        for index in (at..size).rev() {
            let moved = e.mem.u32(buffer + 4 * index);
            e.mem.set_u32(buffer + 4 * (index + 1), moved);
        }
        e.mem.set_u32(buffer + 4 * at, value);
        e.mem.set_u32(array + 8, size + 1);
    }

    /// Gives the tile a float trait.
    fn give_trait(e: &mut Engine, tile: Ptr<Tile>, id: i32, value: f32) -> Ptr<TileValue> {
        let trait_value: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(trait_value, TileValue::eIndex, id);
        e.set(trait_value, TileValue::pParent, tile);
        e.set(trait_value, TileValue::fValue, value);
        insert_value(e, tile.addr() + 0x10, trait_value.addr());
        trait_value
    }

    /// A NUL-terminated string in game memory.
    fn cstring(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    fn string_at(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// The arguments of every logged call to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Appends `child` to the children list of `parent`.
    fn add_child(e: &mut Engine, parent: Ptr<Tile>, child: Ptr<Tile>) {
        let list = parent.at(Tile::xChildren);
        let node = e.mem.alloc(12);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        e.mem.set_u32(node + 4, tail);
        e.mem.set_u32(node + 8, child.addr());
        if tail == 0 {
            e.set(list, NiTPointerList::m_pkHead, node);
        } else {
            e.mem.set_u32(tail, node);
        }
        e.set(list, NiTPointerList::m_pkTail, node);
        let count = e.get(list, NiTPointerList::m_uiCount);
        e.set(list, NiTPointerList::m_uiCount, count + 1);
        e.set(child, Tile::pParent, parent);
    }

    /// A tile whose virtual `GetType` (slot +0xC) answers `kind`.
    fn typed_tile(e: &mut Engine, kind: u32) -> Ptr<Tile> {
        let vtable = 0x0400_0000 + kind * 0x20;
        let function = 0x0500_0000 + kind;
        e.put_vtable(vtable, &[0, 0, 0, function]);
        e.register_double(function, move |_, _| kind.into_ret());
        let tile: Ptr<Tile> = e.new_object();
        e.mem.set_u32(tile.addr(), vtable);
        tile
    }

    /// The `Menu` object of a menu-type tile; `depth` is its +0x18 word.
    fn make_menu_tile(e: &mut Engine, depth: i32) -> (Ptr<Tile>, Ptr) {
        let tile = typed_tile(e, TYPE_MENU);
        let menu: Ptr = Ptr::new(e.mem.alloc(0x28));
        e.mem.set_i32(menu.addr() + 0x18, depth);
        e.mem.set_u32(tile.addr() + 0x3c, menu.addr());
        (tile, menu)
    }

    /// `Lock` and `Unlock` calls balance.
    fn assert_lock_balanced(e: &Engine) {
        assert_eq!(calls_to(e, TILE_LOCK).len(), calls_to(e, TILE_UNLOCK).len());
    }

    #[test]
    fn destructor_clears_the_lists_flags_and_releases_the_model() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(
            tile,
            Tile::uiFlags,
            FLAG_RELEASED | FLAG_DIRTY | FLAG_HIBERNATED | FLAG_PROMOTED,
        );
        // A model with one reference and a destructor in vtable slot 1.
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model + 4, 1);
        e.put_vtable(0x0600_0000, &[0, 0x0600_1000]);
        e.mem.set_u32(model, 0x0600_0000);
        let destroyed = Rc::new(Cell::new(0));
        let seen = destroyed.clone();
        e.register_double(0x0600_1000, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.set_global(HIBERNATING_TILE_COUNT, 5u32);
        e.register(LIST_REMOVE_ITEM, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);

        e.call(0x009f_f340, &args![tile]);

        assert_eq!(e.mem.u32(tile.addr()), VTABLE_TILE);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_RELEASED);
        assert_eq!(e.global::<u32>(HIBERNATING_TILE_COUNT), 4);
        assert_eq!(destroyed.get(), model);
        assert_eq!(
            calls_to(&e, INTERFACE_TILE_IS_BEING_DELETED),
            vec![vec![tile.addr()]]
        );
        let removals: Vec<u32> = calls_to(&e, LIST_REMOVE_ITEM)
            .iter()
            .map(|args| args[0])
            .collect();
        assert_eq!(removals, vec![DIRTY_TILES_LIST, HIBERNATING_TILES_LIST]);
        assert_eq!(calls_to(&e, MODEL_HOOK_00C45830), vec![vec![model]]);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![tile.addr() + 0x20, 0, 0]]
        );
        assert_eq!(calls_to(&e, VALUES_DESTROY), vec![vec![tile.addr() + 0x10]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![tile.addr() + 4]]);
        // The critical section is left again.
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            calls_to(&e, LEAVE_CRITICAL_SECTION)
        );
    }

    #[test]
    fn destructor_keeps_the_hibernating_count_when_the_removal_answers_null() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, FLAG_RELEASED | FLAG_HIBERNATED);
        e.set_global(HIBERNATING_TILE_COUNT, 5u32);
        e.register(LIST_REMOVE_ITEM, |_, _| 0u32.into_ret());
        e.call(0x009f_f340, &args![tile]);
        assert_eq!(e.global::<u32>(HIBERNATING_TILE_COUNT), 5);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_RELEASED);
    }

    #[test]
    fn destructor_complains_about_and_releases_a_tile_nobody_released() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        e.call(0x009f_f340, &args![tile]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![MSG_BASE_TILE_NOT_RELEASED]]
        );
        assert_eq!(e.get(tile, Tile::uiFlags) & FLAG_RELEASED, FLAG_RELEASED);
        // `Release` detached the tile from its parent.
        assert_eq!(calls_to(&e, TILE_SET_PARENT), vec![vec![tile.addr(), 0, 0]]);
    }

    #[test]
    fn init_resets_the_tile_and_attaches_it_when_given_a_parent() {
        let mut e = tile_engine();
        let tile: Ptr<Tile> = e.new_object();
        e.set(tile, Tile::pParent, Ptr::new(0x1234));
        e.set(tile, Tile::uiFlags, 0xffff);
        e.set(tile, Tile::bSpeechChallengeFailure, true);
        let name = cstring(&mut e, "Name");
        e.call_log = Some(vec![]);
        e.call(
            0x009f_f600,
            &args![tile, Ptr::<Tile>::new(0x4000), Ptr::<()>::new(name), 7u32],
        );
        assert_eq!(e.get(tile, Tile::pParent), Ptr::NULL);
        assert_eq!(e.get(tile, Tile::uiFlags), 0);
        assert!(!e.get(tile, Tile::bSpeechChallengeFailure));
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![tile.addr() + 0x2c, 0]]
        );
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![
                vec![tile.addr() + 0x20, EMPTY_NAME, 0],
                vec![tile.addr() + 0x20, name, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_PARENT),
            vec![vec![tile.addr(), 0x4000, 7]]
        );
    }

    #[test]
    fn init_without_parent_and_name_only_resets() {
        let mut e = tile_engine();
        let tile: Ptr<Tile> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(
            0x009f_f600,
            &args![tile, Ptr::<Tile>::NULL, Ptr::<()>::NULL, 0u32],
        );
        assert!(calls_to(&e, TILE_SET_PARENT).is_empty());
        assert_eq!(calls_to(&e, STRING_SET).len(), 1);
    }

    #[test]
    fn release_tears_the_tile_down() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        // Two trait values, one child, a model with a parent node.
        let first = give_trait(&mut e, tile, TRAIT_X, 1.0);
        let second = give_trait(&mut e, tile, TRAIT_Y, 2.0);
        let child = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, tile, child);
        let model = e.mem.alloc(0x20);
        let parent_node = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0x18, parent_node);
        e.put_vtable(0x0600_0000, &[0; 0x3c]);
        e.mem.set_u32(0x0600_0000 + 0xe8, 0x0600_1000);
        e.mem.set_u32(parent_node, 0x0600_0000);
        let removed = Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(0x0600_1000, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.set(tile, Tile::spModel, Ptr::new(model));
        // The list pop hands out the child once.
        let pending = Rc::new(RefCell::new(vec![child.addr()]));
        let queue = pending.clone();
        e.register_double(LIST_REMOVE_HEAD, move |e, a| {
            let list = a[0];
            let child = queue.borrow_mut().pop().unwrap_or(0);
            let count = e.mem.u32(list + 8);
            e.mem.set_u32(list + 8, count - 1);
            child.into_ret()
        });
        // The child's scalar deleting destructor (slot 0) is recorded.
        let deleted = Rc::new(RefCell::new(vec![]));
        let seen = deleted.clone();
        e.register_double(0x0600_2000, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.put_vtable(
            0x0400_0000 + TYPE_RECT * 0x20,
            &[0x0600_2000, 0, 0, 0x0500_0385],
        );
        // The thread's current-line word is 13 while the tile is released.
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        let during = Rc::new(Cell::new(0));
        let seen = during.clone();
        e.register_double(VALUES_CLEAR, move |e, _| {
            let line_word = e.tls() + TLS_CURRENT_LINE;
            seen.set(e.mem.u32(line_word));
            Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x009f_f690, &args![tile]);

        assert_eq!(during.get(), 13);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert_eq!(e.get(tile, Tile::uiFlags) & FLAG_RELEASED, FLAG_RELEASED);
        assert_eq!(
            calls_to(&e, VALUE_DESTROY),
            vec![vec![first.addr()], vec![second.addr()]]
        );
        // Both values are given back to the allocator.
        assert_eq!(e.mem.block_size(first.addr()), None);
        assert_eq!(e.mem.block_size(second.addr()), None);
        assert_eq!(
            calls_to(&e, VALUES_CLEAR),
            vec![vec![tile.addr() + 0x10, 1]]
        );
        assert_eq!(
            calls_to(&e, TILE_REMOVE_FADE_CONTROL),
            vec![vec![tile.addr(), 0x8000_0000]]
        );
        assert_eq!(removed.borrow().clone(), vec![vec![parent_node, model]]);
        assert_eq!(deleted.borrow().clone(), vec![vec![child.addr(), 1]]);
        assert_eq!(
            calls_to(&e, INTERFACE_TILE_IS_BEING_DELETED),
            vec![vec![tile.addr()]]
        );
        assert_lock_balanced(&e);
    }

    #[test]
    fn registering_a_user_name_raises_the_next_id_and_records_its_number() {
        let mut e = tile_engine();
        e.register(TEXT_IS_USER_NAME, |_, _| 1u32.into_ret());
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            text.iter()
                .rposition(|&c| c == a[1] as u8)
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        e.set_global(TEXT_TABLE_USER_NEXT_ID, 10000i32);
        let name = cstring(&mut e, "user_12");
        e.call_log = Some(vec![]);
        e.call(0x009f_f8a0, &args![Ptr::<()>::new(name), 10005i32]);
        assert_eq!(e.global::<i32>(TEXT_TABLE_USER_NEXT_ID), 10006);
        assert_eq!(
            calls_to(&e, TEXT_TABLE_ADD),
            vec![vec![TEXT_TABLE, name, 10005]]
        );
        assert_eq!(
            calls_to(&e, TRAIT_EXTRA_DATA_ADD),
            vec![vec![TRAIT_EXTRA_DATA_MAP, 10005, 12]]
        );
        assert_lock_balanced(&e);
    }

    #[test]
    fn registering_a_small_id_only_enters_the_text_table() {
        let mut e = tile_engine();
        e.register(TEXT_IS_USER_NAME, |_, _| 1u32.into_ret());
        e.set_global(TEXT_TABLE_USER_NEXT_ID, 10000i32);
        let name = cstring(&mut e, "x_12");
        e.call_log = Some(vec![]);
        e.call(0x009f_f8a0, &args![Ptr::<()>::new(name), 9999i32]);
        assert_eq!(calls_to(&e, TEXT_TABLE_ADD).len(), 1);
        assert!(calls_to(&e, TRAIT_EXTRA_DATA_ADD).is_empty());
        assert_eq!(e.global::<i32>(TEXT_TABLE_USER_NEXT_ID), 10000);
    }

    #[test]
    fn registering_a_name_without_a_number_skips_the_id_map() {
        let mut e = tile_engine();
        e.register(TEXT_IS_USER_NAME, |_, _| 1u32.into_ret());
        e.register(STRRCHR, |_, _| 0u32.into_ret());
        e.set_global(TEXT_TABLE_USER_NEXT_ID, 20000i32);
        let name = cstring(&mut e, "plain");
        e.call_log = Some(vec![]);
        e.call(0x009f_f8a0, &args![Ptr::<()>::new(name), 10005i32]);
        // The next id was already past this one.
        assert_eq!(e.global::<i32>(TEXT_TABLE_USER_NEXT_ID), 20000);
        assert!(calls_to(&e, TRAIT_EXTRA_DATA_ADD).is_empty());
    }

    #[test]
    fn registering_an_empty_or_missing_name_does_nothing() {
        let mut e = tile_engine();
        let empty = cstring(&mut e, "");
        e.call_log = Some(vec![]);
        e.call(0x009f_f8a0, &args![Ptr::<()>::new(empty), 5i32]);
        e.call(0x009f_f8a0, &args![Ptr::<()>::NULL, 5i32]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
    }

    /// Makes every word of the text table non-empty in memory (the game's
    /// strings are in the exe's data, which tests do not map).
    fn provide_text_table_words(e: &mut Engine) {
        for &(name, _) in TEXT_TABLE_ENTRIES {
            e.map(name, 1);
            e.mem.set_u8(name, b'x');
        }
    }

    #[test]
    fn the_text_table_gets_all_its_words_in_order() {
        let mut e = tile_engine();
        provide_text_table_words(&mut e);
        e.set_global(HAS_360_CONTROLLER, 1u8);
        e.register(INTERFACE_IS_WIDESCREEN, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009f_f970, &args![]);
        let adds = calls_to(&e, TEXT_TABLE_ADD);
        assert_eq!(adds.len(), 224);
        assert_eq!(adds[0], vec![TEXT_TABLE, 0x0109_4868, 0xffff_ffff]);
        assert_eq!(adds[223][0], TEXT_TABLE);
        let value_of = |name: u32| adds.iter().find(|add| add[1] == name).unwrap()[2];
        // A 360 controller without the console-UI byte: xenon and xbox are 1.
        assert_eq!(value_of(XENON_NAME), 1);
        assert_eq!(value_of(XBOX_NAME), 1);
        assert_eq!(value_of(WIDESCREEN_NAME), 1);
        // The list is in the game's order: the third word is `&true;`.
        assert_eq!(adds[2][1..], [0x0106_ec7c, 1]);
        // Tile types and user traits further down.
        assert_eq!(adds[37][2], 0x386);
        assert_eq!(adds[223][2], 0x138f);
    }

    #[test]
    fn xenon_and_xbox_are_zero_without_a_controller_or_with_the_console_byte() {
        let mut e = tile_engine();
        provide_text_table_words(&mut e);
        e.register(INTERFACE_IS_WIDESCREEN, |_, _| 0u32.into_ret());
        for (controller, console) in [(0u8, 0u8), (1, 1)] {
            e.set_global(HAS_360_CONTROLLER, controller);
            e.set_global(CONSOLE_UI_BYTE, console);
            e.call_log = Some(vec![]);
            e.call(0x009f_f970, &args![]);
            let adds = calls_to(&e, TEXT_TABLE_ADD);
            let value_of = |name: u32| adds.iter().find(|add| add[1] == name).unwrap()[2];
            assert_eq!(value_of(XENON_NAME), 0);
            assert_eq!(value_of(XBOX_NAME), 0);
            assert_eq!(value_of(WIDESCREEN_NAME), 0);
        }
    }

    #[test]
    fn add_user_trait_returns_an_existing_names_id() {
        let mut e = tile_engine();
        e.register(TEXT_TABLE_FIND, |e, a| {
            e.mem.set_i32(a[2], 42);
            1u32.into_ret()
        });
        let name = cstring(&mut e, "known");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a0_0940, &args![Ptr::<()>::new(name), -1i32])
                .i32(),
            42
        );
        assert!(calls_to(&e, TEXT_TABLE_ADD).is_empty());
        assert_lock_balanced(&e);
    }

    #[test]
    fn add_user_trait_takes_the_next_free_id_for_a_new_name() {
        let mut e = tile_engine();
        e.register(TEXT_IS_USER_NAME, |_, _| 1u32.into_ret());
        e.set_global(TEXT_TABLE_USER_NEXT_ID, 10010i32);
        let name = cstring(&mut e, "fresh");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a0_0940, &args![Ptr::<()>::new(name), -1i32])
                .i32(),
            10010
        );
        // `AddUserTrait` took 10010; registering it raised the next id.
        assert_eq!(e.global::<i32>(TEXT_TABLE_USER_NEXT_ID), 10011);
        assert_eq!(
            calls_to(&e, TEXT_TABLE_ADD),
            vec![vec![TEXT_TABLE, name, 10010]]
        );
        assert_lock_balanced(&e);
    }

    #[test]
    fn add_user_trait_refuses_small_ids_and_unacceptable_names() {
        let mut e = tile_engine();
        e.register(TEXT_IS_USER_NAME, |_, _| 1u32.into_ret());
        let name = cstring(&mut e, "small");
        assert_eq!(
            e.call(0x00a0_0940, &args![Ptr::<()>::new(name), 500i32])
                .i32(),
            i32::MIN
        );
        e.register(TEXT_IS_USER_NAME, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00a0_0940, &args![Ptr::<()>::new(name), 20000i32])
                .i32(),
            i32::MIN
        );
    }

    #[test]
    fn free_trait_list_empties_the_maps_and_frees_both_lists() {
        let mut e = tile_engine();
        // Each static list: an embedded head node and one more node, both
        // with an item (a heap block).
        let mut items = vec![];
        for head in [TEXTURE_ENTRY_LIST, FADE_CONTROLS_LIST] {
            let second = e.mem.alloc(8);
            let (first_item, second_item) = (e.mem.alloc(8), e.mem.alloc(8));
            e.mem.set_u32(head, first_item);
            e.mem.set_u32(head + 4, second);
            e.mem.set_u32(second, second_item);
            items.push((first_item, second_item));
        }
        e.call_log = Some(vec![]);
        e.call(0x00a0_0a30, &args![]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .filter(|addr| {
                [
                    MAP_REMOVE_ALL,
                    SIMPLE_LIST_REMOVE_ALL,
                    TEXTURE_ATLAS_ENTRY_DESTROY,
                ]
                .contains(addr)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                MAP_REMOVE_ALL,
                MAP_REMOVE_ALL,
                TEXTURE_ATLAS_ENTRY_DESTROY,
                TEXTURE_ATLAS_ENTRY_DESTROY,
                SIMPLE_LIST_REMOVE_ALL,
                SIMPLE_LIST_REMOVE_ALL
            ]
        );
        // Only the texture entries are destroyed before being freed.
        assert_eq!(
            calls_to(&e, TEXTURE_ATLAS_ENTRY_DESTROY),
            vec![vec![items[0].0], vec![items[0].1]]
        );
        for (first, second) in items {
            assert_eq!(e.mem.block_size(first), None);
            assert_eq!(e.mem.block_size(second), None);
        }
        assert_eq!(
            calls_to(&e, SIMPLE_LIST_REMOVE_ALL),
            vec![vec![TEXTURE_ENTRY_LIST], vec![FADE_CONTROLS_LIST]]
        );
        assert_lock_balanced(&e);
    }

    #[test]
    fn set_menu_deleting_marks_the_tile_and_its_children() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let child = typed_tile(&mut e, TYPE_RECT);
        let grandchild = typed_tile(&mut e, TYPE_RECT);
        let released = typed_tile(&mut e, TYPE_RECT);
        let marked = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, tile, child);
        add_child(&mut e, child, grandchild);
        add_child(&mut e, tile, released);
        add_child(&mut e, released, marked);
        e.set(released, Tile::uiFlags, FLAG_RELEASED);
        e.set(marked, Tile::uiFlags, FLAG_MENU_DELETING);
        e.call_log = Some(vec![]);
        e.call(0x00a0_0b50, &args![tile]);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_MENU_DELETING);
        assert_eq!(e.get(child, Tile::uiFlags), FLAG_MENU_DELETING);
        assert_eq!(e.get(grandchild, Tile::uiFlags), FLAG_MENU_DELETING);
        // A released tile is left alone, with its children.
        assert_eq!(e.get(released, Tile::uiFlags), FLAG_RELEASED);
        assert_eq!(e.get(marked, Tile::uiFlags), FLAG_MENU_DELETING);
        assert_lock_balanced(&e);
    }

    #[test]
    fn build_tile_constructs_each_kind() {
        let mut e = tile_engine();
        // The constructors that return `this`; `TileImage`'s is told apart
        // by writing a word.
        e.register(TILE_IMAGE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x1111);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let rect = e.call(0x00a0_0c20, &args![TYPE_RECT]).ptr::<()>();
        assert_eq!(e.mem.u32(rect.addr()), VTABLE_TILE_RECT);
        assert_eq!(calls_to(&e, TILE_BASE_CONSTRUCT), vec![vec![rect.addr()]]);

        let image = e.call(0x00a0_0c20, &args![TYPE_IMAGE]).ptr::<()>();
        let hot_rect = e.call(0x00a0_0c20, &args![TYPE_HOT_RECT]).ptr::<()>();
        assert_eq!(e.mem.u32(image.addr() + 4), 0x1111);
        assert_eq!(e.mem.u32(hot_rect.addr() + 4), 0x1111);
        assert_eq!(e.mem.block_size(image.addr()), Some(0x48));

        let text = e.call(0x00a0_0c20, &args![TYPE_TEXT]).ptr::<()>();
        assert_eq!(e.mem.u32(text.addr()), VTABLE_TILE_TEXT);
        assert_eq!(e.mem.u8(text.addr() + 0x48), 0);
        assert_eq!(e.mem.block_size(text.addr()), Some(0x50));

        let three_d = e.call(0x00a0_0c20, &args![TYPE_3D]).ptr::<()>();
        assert_eq!(calls_to(&e, TILE_3D_CONSTRUCT), vec![vec![three_d.addr()]]);
        assert_eq!(e.mem.block_size(three_d.addr()), Some(0x50));

        let menu = e.call(0x00a0_0c20, &args![TYPE_MENU]).ptr::<()>();
        assert_eq!(e.mem.u32(menu.addr()), VTABLE_TILE_MENU);
        assert_eq!(e.mem.u32(menu.addr() + 0x3c), 0);

        let radial = e.call(0x00a0_0c20, &args![TYPE_RADIAL]).ptr::<()>();
        assert_eq!(
            calls_to(&e, RADIAL_TILE_CONSTRUCT),
            vec![vec![radial.addr()]]
        );
        assert_eq!(e.mem.block_size(radial.addr()), Some(0x48));
    }

    #[test]
    fn build_tile_makes_nothing_for_other_types() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        // The window type, the template type, and numbers outside the table.
        for kind in [0x38bu32, 999, 0, 0x384, 0x38d, 0xffff_ffff] {
            assert_eq!(e.call(0x00a0_0c20, &args![kind]).u32(), 0);
        }
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn get_value_q_scans_the_sorted_traits() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let _first = give_trait(&mut e, tile, -5, 0.5);
        let second = give_trait(&mut e, tile, 5, 1.5);
        let third = give_trait(&mut e, tile, 9, 2.5);
        e.call_log = Some(vec![]);
        let find = |e: &mut Engine, id: i32| e.call(0x00a0_0e90, &args![tile, id]).u32();
        assert_eq!(find(&mut e, 5), second.addr());
        assert_eq!(find(&mut e, 9), third.addr());
        // Between two ids, below all of them, above all of them.
        assert_eq!(find(&mut e, 6), 0);
        assert_eq!(find(&mut e, -6), 0);
        assert_eq!(find(&mut e, 10), 0);
        let bare = typed_tile(&mut e, TYPE_RECT);
        assert_eq!(e.call(0x00a0_0e90, &args![bare, 5i32]).u32(), 0);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION)[0],
            vec![TILE_CRITICAL_SECTION]
        );
    }

    #[test]
    fn get_value_searches_with_the_shared_key() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let _low = give_trait(&mut e, tile, 3, 1.0);
        let high = give_trait(&mut e, tile, 7, 2.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a0_0f30, &args![tile, 7i32]).u32(), high.addr());
        assert_eq!(e.call(0x00a0_0f30, &args![tile, 4i32]).u32(), 0);
        // The key was set up on the first call (unset state, guard bit,
        // `atexit` registration, once) and holds the last id searched.
        assert_eq!(e.global::<u32>(GET_VALUE_KEY_GUARD) & 1, 1);
        assert_eq!(e.global::<i32>(GET_VALUE_KEY), 4);
        assert_eq!(e.global::<u32>(GET_VALUE_KEY + 4), 0);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![GET_VALUE_KEY_DESTROY]]);
        let finds = calls_to(&e, VALUES_SORTED_FIND);
        assert_eq!(finds.len(), 2);
        assert_eq!(finds[0][0], tile.addr() + 0x10);
        assert_eq!(finds[0][2], COMPARE_VALUES);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn get_or_create_value_returns_or_inserts() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let existing = give_trait(&mut e, tile, 3, 1.0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a0_1000, &args![tile, 3i32]).u32(),
            existing.addr()
        );
        assert!(calls_to(&e, VALUES_SORTED_INSERT).is_empty());

        let created: Ptr<TileValue> = e.call(0x00a0_1000, &args![tile, 1i32]).ptr();
        assert!(!created.is_null());
        assert_eq!(e.get(created, TileValue::eIndex), 1);
        assert_eq!(e.get(created, TileValue::pParent), tile);
        assert_eq!(e.get(created, TileValue::fValue), 0.0);
        assert!(e.get(created, TileValue::strValue).is_null());
        assert!(e.get(created, TileValue::pActionListA).is_null());
        assert_eq!(e.mem.block_size(created.addr()), Some(0x18));
        // The new value went in before the existing one.
        let buffer = e.mem.u32(tile.addr() + 0x14);
        assert_eq!(e.mem.u32(buffer), created.addr());
        assert_eq!(e.mem.u32(buffer + 4), existing.addr());
        assert_eq!(e.mem.u32(tile.addr() + 0x18), 2);
        let inserts = calls_to(&e, VALUES_SORTED_INSERT);
        assert_eq!(inserts.len(), 1);
        assert_eq!(inserts[0][0], tile.addr() + 0x10);
        assert_eq!(inserts[0][2], COMPARE_VALUES);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![GET_OR_CREATE_KEY_DESTROY]]);
    }

    #[test]
    fn the_value_comparison_orders_by_signed_id() {
        let mut e = tile_engine();
        let slot_for = |e: &mut Engine, id: i32| {
            let value = e.mem.alloc(0x14);
            e.mem.set_i32(value, id);
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, value);
            slot
        };
        let (low, high, same) = (
            slot_for(&mut e, -3),
            slot_for(&mut e, 4),
            slot_for(&mut e, 4),
        );
        assert_eq!(
            e.call(
                0x00a0_1160,
                &args![Ptr::<()>::new(low), Ptr::<()>::new(high)]
            )
            .i32(),
            -1
        );
        assert_eq!(
            e.call(
                0x00a0_1160,
                &args![Ptr::<()>::new(high), Ptr::<()>::new(low)]
            )
            .i32(),
            1
        );
        assert_eq!(
            e.call(
                0x00a0_1160,
                &args![Ptr::<()>::new(high), Ptr::<()>::new(same)]
            )
            .i32(),
            0
        );
    }

    #[test]
    fn trait_getters_read_the_value_or_give_zero() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let value = give_trait(&mut e, tile, TRAIT_X, 2.5);
        e.set(value, TileValue::strValue, Ptr::new(0x7000));
        assert_eq!(e.call(0x00a0_11b0, &args![tile, TRAIT_X]).f32(), 2.5);
        assert_eq!(e.call(0x00a0_11b0, &args![tile, TRAIT_Y]).f32(), 0.0);
        assert_eq!(e.call(0x00a0_11f0, &args![tile, TRAIT_X]).u32(), 0x7000);
        assert_eq!(e.call(0x00a0_11f0, &args![tile, TRAIT_Y]).u32(), 0);
    }

    #[test]
    fn is_true_tests_the_float_against_zero() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, tile, 1, 0.0);
        give_trait(&mut e, tile, 2, 2.5);
        give_trait(&mut e, tile, 3, -0.5);
        give_trait(&mut e, tile, 4, f32::NAN);
        let is_true = |e: &mut Engine, id: i32| e.call(0x00a0_1230, &args![tile, id]).bool();
        assert!(!is_true(&mut e, 1));
        assert!(is_true(&mut e, 2));
        assert!(is_true(&mut e, 3));
        // Not a number differs from zero.
        assert!(is_true(&mut e, 4));
        // No such trait.
        assert!(!is_true(&mut e, 5));
    }

    #[test]
    fn poke_sets_the_value_and_then_zero() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        e.call(0x00a0_1290, &args![tile, TRAIT_VISIBLE, 1.5f32]);
        let value = e.mem.u32(e.mem.u32(tile.addr() + 0x14));
        assert_eq!(
            calls_to(&e, VALUE_SET_FLOAT),
            vec![
                vec![value, 1.5f32.to_bits(), 1],
                vec![value, 0.0f32.to_bits(), 1]
            ]
        );
        // The value was created once.
        assert_eq!(e.mem.u32(tile.addr() + 0x18), 1);
        assert_lock_balanced(&e);
    }

    #[test]
    fn setting_a_float_trait_creates_it_and_locks() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        e.call(0x00a0_12d0, &args![tile, TRAIT_ALPHA, 0.25f32, true]);
        let value = e.mem.u32(e.mem.u32(tile.addr() + 0x14));
        assert_eq!(e.mem.i32(value), TRAIT_ALPHA);
        let log = e.call_log.as_ref().unwrap();
        let positions: Vec<usize> = [TILE_LOCK, VALUE_SET_FLOAT, TILE_UNLOCK]
            .iter()
            .map(|addr| log.iter().position(|(called, _)| called == addr).unwrap())
            .collect();
        assert!(positions[0] < positions[1] && positions[1] < positions[2]);
        assert_eq!(
            calls_to(&e, VALUE_SET_FLOAT),
            vec![vec![value, 0.25f32.to_bits(), 1]]
        );
    }

    #[test]
    fn setting_a_string_trait_creates_it_and_locks() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        e.call(
            0x00a0_1350,
            &args![tile, TRAIT_FILENAME, Ptr::<()>::new(0x7100), false],
        );
        let value = e.mem.u32(e.mem.u32(tile.addr() + 0x14));
        assert_eq!(calls_to(&e, VALUE_SET_STRING), vec![vec![value, 0x7100, 0]]);
        assert_lock_balanced(&e);
        assert_eq!(calls_to(&e, TILE_LOCK).len(), 1);
    }

    /// root <- middle <- leaf, each with the given x, y, depth and locus.
    fn chain(e: &mut Engine, traits: [[f32; 4]; 3]) -> [Ptr<Tile>; 3] {
        install_value_array_doubles(e);
        let tiles = [
            typed_tile(e, TYPE_RECT),
            typed_tile(e, TYPE_RECT),
            typed_tile(e, TYPE_RECT),
        ];
        add_child(e, tiles[0], tiles[1]);
        add_child(e, tiles[1], tiles[2]);
        for (tile, values) in tiles.iter().zip(traits) {
            for (id, value) in [TRAIT_X, TRAIT_Y, TRAIT_DEPTH, TRAIT_LOCUS]
                .into_iter()
                .zip(values)
            {
                give_trait(e, *tile, id, value);
            }
        }
        tiles
    }

    #[test]
    fn position_sums_add_the_ancestors_with_a_locus() {
        let mut e = tile_engine();
        // [x, y, depth, locus]: the middle tile has a locus, the root none.
        let [_, _, leaf] = chain(
            &mut e,
            [
                [100.0, 200.0, 10.0, 0.0],
                [10.0, 20.0, 1.0, 1.0],
                [1.0, 2.0, 0.5, 0.0],
            ],
        );
        assert_eq!(e.call(0x00a0_13d0, &args![leaf]).f32(), 11.0);
        assert_eq!(e.call(0x00a0_1440, &args![leaf]).f32(), 22.0);
        // A tile without a parent is just its own value.
        let lone = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, lone, TRAIT_X, 3.5);
        assert_eq!(e.call(0x00a0_13d0, &args![lone]).f32(), 3.5);
    }

    #[test]
    fn depth_sum_adds_ancestors_with_a_locus_or_under_the_menus_root() {
        let mut e = tile_engine();
        let [root, middle, leaf] = chain(
            &mut e,
            [
                [0.0, 0.0, 10.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
            ],
        );
        e.register_double(INTERFACE_GET_MENUS_ROOT, move |_, _| root.addr().into_ret());
        // The middle tile has no locus, but its parent is the menus root.
        assert_eq!(e.call(0x00a0_14b0, &args![leaf]).f32(), 2.5);
        // With a locus on the middle tile the root is still not counted.
        give_trait_value(&mut e, middle, TRAIT_LOCUS, 1.0);
        assert_eq!(e.call(0x00a0_14b0, &args![leaf]).f32(), 2.5);
        // With a locus on the root it is counted as well.
        give_trait_value(&mut e, root, TRAIT_LOCUS, 1.0);
        assert_eq!(e.call(0x00a0_14b0, &args![leaf]).f32(), 12.5);
    }

    /// Changes the value of an existing trait.
    fn give_trait_value(e: &mut Engine, tile: Ptr<Tile>, id: i32, value: f32) {
        let found: Ptr<TileValue> = tile_get_value(e, tile, id).cast();
        e.set(found, TileValue::fValue, value);
    }

    #[test]
    fn maximum_depth_is_the_menus_answer() {
        let mut e = tile_engine();
        e.register(MENU_GET_MAX_DEPTH, |_, _| 3.5f32.into_ret());
        assert_eq!(e.call(0x00a0_1530, &args![]).f32(), 3.5);
    }

    #[test]
    fn subtree_depth_takes_the_deepest_child() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let top = typed_tile(&mut e, TYPE_RECT);
        let (first, second, grandchild) = (
            typed_tile(&mut e, TYPE_RECT),
            typed_tile(&mut e, TYPE_RECT),
            typed_tile(&mut e, TYPE_RECT),
        );
        add_child(&mut e, top, first);
        add_child(&mut e, top, second);
        add_child(&mut e, second, grandchild);
        for (tile, depth) in [(top, 1.0), (first, 5.0), (second, 3.0), (grandchild, 4.0)] {
            give_trait(&mut e, tile, TRAIT_DEPTH, depth);
        }
        // No locus anywhere: the deepest of 1, 5, 3 and 4.
        assert_eq!(e.call(0x00a0_1540, &args![top]).f32(), 5.0);
        // A locus on the top tile adds its depth to what is below it.
        give_trait(&mut e, top, TRAIT_LOCUS, 1.0);
        assert_eq!(e.call(0x00a0_1540, &args![top]).f32(), 6.0);
        // A tile without children gives its own depth.
        assert_eq!(e.call(0x00a0_1540, &args![grandchild]).f32(), 4.0);
    }

    /// A node with an extra data object (reference count `count`, tile
    /// pointer `tile`); `GetExtraData` answers for the nodes in `map`.
    fn extra_data(e: &mut Engine, count: i32, tile: u32) -> u32 {
        let extra = e.mem.alloc(0x20);
        e.mem.set_i32(extra + 4, count);
        e.mem.set_u32(extra + 0xc, tile);
        e.put_vtable(0x0600_3000, &[0, 0x0600_3100]);
        e.mem.set_u32(extra, 0x0600_3000);
        extra
    }

    fn install_extra_data(e: &mut Engine, map: Vec<(u32, u32)>) {
        e.register_double(NI_OBJECT_GET_EXTRA_DATA, move |e, a| {
            assert_eq!(a[1], e.global::<u32>(TILE_EXTRA_DATA_KEY));
            map.iter()
                .find(|(node, _)| *node == a[0])
                .map_or(0, |(_, extra)| *extra)
                .into_ret()
        });
        // `NiPointer::operator=`: takes a reference to the new object.
        e.register(NI_POINTER_ASSIGN, |e, a| {
            if a[1] != 0 {
                let count = e.mem.i32(a[1] + 4);
                e.mem.set_i32(a[1] + 4, count + 1);
            }
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn tile_from_node_reads_the_nodes_own_extra_data() {
        let mut e = tile_engine();
        e.set_global(TILE_EXTRA_DATA_KEY, 0x1234u32);
        let (node, parent_node) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(node + 0x18, parent_node);
        let extra = extra_data(&mut e, 1, 0xa000);
        let parent_extra = extra_data(&mut e, 1, 0xb000);
        install_extra_data(&mut e, vec![(node, extra), (parent_node, parent_extra)]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a0_1630, &args![Ptr::<()>::new(node)]).u32(),
            0xa000
        );
        // The reference taken while reading is dropped again.
        assert_eq!(e.mem.i32(extra + 4), 1);
        assert_eq!(calls_to(&e, NI_OBJECT_GET_EXTRA_DATA).len(), 1);
        assert_lock_balanced(&e);
    }

    #[test]
    fn tile_from_node_falls_back_to_the_parent_node() {
        let mut e = tile_engine();
        e.set_global(TILE_EXTRA_DATA_KEY, 0x1234u32);
        let (node, parent_node) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(node + 0x18, parent_node);
        let parent_extra = extra_data(&mut e, 1, 0xb000);
        install_extra_data(&mut e, vec![(parent_node, parent_extra)]);
        assert_eq!(
            e.call(0x00a0_1630, &args![Ptr::<()>::new(node)]).u32(),
            0xb000
        );
        assert_eq!(e.mem.i32(parent_extra + 4), 1);
        // Neither node has extra data: no tile.
        install_extra_data(&mut e, vec![]);
        assert_eq!(e.call(0x00a0_1630, &args![Ptr::<()>::new(node)]).u32(), 0);
        // No node at all: nothing is looked up.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a0_1630, &args![Ptr::<()>::NULL]).u32(), 0);
        assert!(calls_to(&e, NI_OBJECT_GET_EXTRA_DATA).is_empty());
        assert_lock_balanced(&e);
    }

    #[test]
    fn tile_from_node_destroys_extra_data_nobody_else_holds() {
        let mut e = tile_engine();
        let node = e.mem.alloc(0x20);
        // Reference count 0: taking and dropping the reference frees it.
        let extra = extra_data(&mut e, 0, 0xa000);
        install_extra_data(&mut e, vec![(node, extra)]);
        let destroyed = Rc::new(Cell::new(0));
        let seen = destroyed.clone();
        e.register_double(0x0600_3100, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        assert_eq!(
            e.call(0x00a0_1630, &args![Ptr::<()>::new(node)]).u32(),
            0xa000
        );
        assert_eq!(destroyed.get(), extra);
    }

    #[test]
    fn sever_extra_data_clears_the_tile_pointer_in_the_models_extra_data() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let model = e.mem.alloc(0x20);
        e.mem.set_u16(model + 0x14, 1);
        e.set(tile, Tile::spModel, Ptr::new(model));
        let extra = extra_data(&mut e, 1, 0xa000);
        install_extra_data(&mut e, vec![(model, extra)]);
        e.call_log = Some(vec![]);
        e.call(0x00a0_1750, &args![tile]);
        assert_eq!(e.mem.u32(extra + 0xc), 0);
        assert_eq!(e.mem.i32(extra + 4), 1);
        assert_lock_balanced(&e);

        // A model without extra data (count 0) is not asked.
        e.mem.set_u16(model + 0x14, 0);
        e.mem.set_u32(extra + 0xc, 0xa000);
        e.call_log = Some(vec![]);
        e.call(0x00a0_1750, &args![tile]);
        assert!(calls_to(&e, NI_OBJECT_GET_EXTRA_DATA).is_empty());
        assert_eq!(e.mem.u32(extra + 0xc), 0xa000);

        // No model at all.
        e.set(tile, Tile::spModel, Ptr::NULL);
        e.call(0x00a0_1750, &args![tile]);
        assert!(calls_to(&e, NI_OBJECT_GET_EXTRA_DATA).is_empty());
    }

    #[test]
    fn text_to_trait_looks_the_name_up_in_the_text_table() {
        let mut e = tile_engine();
        e.register(TEXT_TABLE_FIND, |e, a| {
            if e.mem.cstr(a[1]) == b"alpha" {
                e.mem.set_i32(a[2], TRAIT_ALPHA);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let known = cstring(&mut e, "alpha");
        let unknown = cstring(&mut e, "nonsense");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a0_1860, &args![Ptr::<()>::new(known)]).i32(),
            TRAIT_ALPHA
        );
        // A name the table does not know gives `eNone`.
        assert_eq!(
            e.call(0x00a0_1860, &args![Ptr::<()>::new(unknown)]).i32(),
            TRAIT_NONE
        );
        assert_eq!(calls_to(&e, TEXT_TABLE_FIND)[0][0], TEXT_TABLE);
        assert_lock_balanced(&e);
    }

    #[test]
    fn splitting_a_name_from_its_bracketed_argument() {
        let mut e = tile_engine();
        let seen = Rc::new(RefCell::new(vec![]));
        let names = seen.clone();
        e.register_double(TEXT_TABLE_FIND, move |e, a| {
            names.borrow_mut().push(string_at(e, a[1]));
            e.mem.set_i32(a[2], 77);
            1u32.into_ret()
        });
        let argument = e.mem.alloc(0x100);
        let split = |e: &mut Engine, text: &str| {
            let name = cstring(e, text);
            let result = e
                .call(
                    0x00a0_18d0,
                    &args![Ptr::<()>::new(name), Ptr::<()>::new(argument)],
                )
                .i32();
            (result, string_at(e, argument))
        };
        assert_eq!(split(&mut e, "trait(arg)"), (77, "arg".to_string()));
        assert_eq!(split(&mut e, "plain"), (77, String::new()));
        // Nothing after the closing bracket is looked at.
        assert_eq!(split(&mut e, "a(b)c"), (77, "b".to_string()));
        assert_eq!(split(&mut e, "(x)"), (77, "x".to_string()));
        assert_eq!(seen.borrow().clone(), vec!["trait", "plain", "a", ""]);
    }

    #[test]
    fn splitting_needs_a_name_and_a_buffer() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        let name = cstring(&mut e, "a(b)");
        let empty = cstring(&mut e, "");
        let argument = e.mem.alloc(0x100);
        for (name, argument) in [(0, argument), (empty, argument), (name, 0)] {
            assert_eq!(
                e.call(
                    0x00a0_18d0,
                    &args![Ptr::<()>::new(name), Ptr::<()>::new(argument)]
                )
                .i32(),
                i32::MIN
            );
        }
        assert!(calls_to(&e, TEXT_TABLE_FIND).is_empty());
    }

    #[test]
    fn the_number_at_the_end_of_a_name() {
        let mut e = tile_engine();
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            text.iter()
                .rposition(|&c| c == a[1] as u8)
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        let number = |e: &mut Engine, text: &str| {
            let name = cstring(e, text);
            e.call(0x00a0_1a20, &args![Ptr::<()>::new(name)]).i32()
        };
        assert_eq!(number(&mut e, "name_12"), 12);
        assert_eq!(number(&mut e, "a_b_7"), 7);
        assert_eq!(number(&mut e, "name"), 0);
        assert_eq!(number(&mut e, "name_"), -1);
        assert_eq!(number(&mut e, "x_abc"), 0);
    }

    /// A text-table node: next, key, value.
    fn table_node(e: &mut Engine, next: u32, key: u32, value: i32) -> u32 {
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, next);
        e.mem.set_u32(node + 4, key);
        e.mem.set_i32(node + 8, value);
        node
    }

    #[test]
    fn the_name_of_a_text_table_value() {
        let mut e = tile_engine();
        // The iterator step: writes the node's key and value to the two out
        // parameters and advances the node.
        e.register(TEXT_TABLE_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let (key, value, next) = (e.mem.u32(node + 4), e.mem.u32(node + 8), e.mem.u32(node));
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let third = table_node(&mut e, 0, 0x9003, 33);
        let second = table_node(&mut e, third, 0x9002, 22);
        let first = table_node(&mut e, second, 0x9001, 11);
        let buckets = e.mem.alloc(16);
        e.mem.set_u32(buckets + 4, first);
        e.set_global(TEXT_TABLE_BUCKET_COUNT, 4u32);
        e.set_global(TEXT_TABLE_BUCKETS, buckets);
        assert_eq!(e.call(0x00a0_1a70, &args![22i32]).u32(), 0x9002);
        assert_eq!(e.call(0x00a0_1a70, &args![11i32]).u32(), 0x9001);
        assert_eq!(e.call(0x00a0_1a70, &args![99i32]).u32(), 0);
        // An empty table never steps.
        e.mem.set_u32(buckets + 4, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a0_1a70, &args![11i32]).u32(), 0);
        assert!(calls_to(&e, TEXT_TABLE_NEXT).is_empty());
    }

    /// One `AddPair` call: command, text (read when it was made), line, flag.
    type Pair = (i32, String, i32, u32);

    /// Doubles for what the XML reader hands its results to: `AddPair`
    /// records its arguments, and the text table knows a few words.
    fn install_reader_doubles(e: &mut Engine) -> Rc<RefCell<Vec<Pair>>> {
        let pairs = Rc::new(RefCell::new(vec![]));
        let record = pairs.clone();
        e.register_double(TEMPLATE_ADD_PAIR, move |e, a| {
            record
                .borrow_mut()
                .push((a[1] as i32, string_at(e, a[2]), a[3] as i32, a[4]));
            Ret::default()
        });
        e.register(TEXT_TABLE_FIND, |e, a| {
            let id = match e.mem.cstr(a[1]).as_slice() {
                b"x" => TRAIT_X,
                b"y" => TRAIT_Y,
                b"width" => TRAIT_WIDTH,
                _ => return 0u32.into_ret(),
            };
            e.mem.set_i32(a[2], id);
            1u32.into_ret()
        });
        e.set_global(XML_ERROR_FLAG, 0u8);
        pairs
    }

    /// Runs the reader over `text`, returning what it produced.
    fn parse(e: &mut Engine, text: &str) -> (Vec<Pair>, Ptr<BuildStorage>) {
        let pairs = install_reader_doubles(e);
        let buffer = cstring(e, text);
        let path = cstring(e, "menu.xml");
        let storage: Ptr<BuildStorage> = e
            .call(
                0x00a0_1e20,
                &args![
                    Ptr::<()>::new(path),
                    Ptr::<()>::new(buffer),
                    text.len() as u32
                ],
            )
            .ptr();
        let pairs = pairs.borrow().clone();
        (pairs, storage)
    }

    #[test]
    fn the_reader_buffers_sit_at_the_exe_frame_offsets_under_ebp() {
        let mut e = tile_engine();
        install_reader_doubles(&mut e);
        let texts = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&texts);
        e.register_double(TEMPLATE_ADD_PAIR, move |_, a| {
            sink.borrow_mut().push(a[2]);
            Ret::default()
        });
        let text = r#"<image x="5">hi</image>"#;
        let buffer = cstring(&mut e, text);
        let path = cstring(&mut e, "menu.xml");
        let frame = e.next_stack_local(FRAME_BELOW_EBP);
        e.call(
            0x00a0_1e20,
            &args![
                Ptr::<()>::new(path),
                Ptr::<()>::new(buffer),
                text.len() as u32
            ],
        );
        let ebp = frame + FRAME_BELOW_EBP;
        let buffers = [ebp - 0x94, ebp - 0x10bc, ebp - 0x20cc];
        let seen = texts.borrow();
        assert!(!seen.is_empty());
        assert!(seen.iter().all(|at| buffers.contains(at)), "{seen:x?}");
        assert!(seen.contains(&(ebp - 0x94)));
    }

    fn pair(command: i32, text: &str, line: i32, flag: u32) -> Pair {
        (command, text.to_string(), line, flag)
    }

    #[test]
    fn parsing_a_tag_with_an_attribute_and_text() {
        let mut e = tile_engine();
        let (pairs, storage) = parse(&mut e, r#"<image x="5">hi</image>"#);
        assert!(!storage.is_null());
        assert_eq!(
            pairs,
            vec![
                // The tag name when it ends at white space.
                pair(0, "image", 1, 0),
                // The attribute: trait id of the name, the value.
                pair(TRAIT_X, "5", 2, 0),
                // The text between the tags, flagged as plain text.
                pair(TAG_VALUE, "hi", 2, 1),
                // The close tag.
                pair(1, "image", 2, 0),
            ]
        );
        assert_eq!(e.global::<u8>(XML_ERROR_FLAG), 0);
    }

    #[test]
    fn parsing_a_self_closing_tag_opens_and_closes_it() {
        let mut e = tile_engine();
        let (pairs, _) = parse(&mut e, "<a/>");
        assert_eq!(pairs, vec![pair(0, "a", 1, 0), pair(1, "a", 2, 0)]);
    }

    #[test]
    fn parsing_attribute_values_keeps_spaces_inside_quotes() {
        let mut e = tile_engine();
        let (pairs, _) = parse(&mut e, "<a x=\"1 2\" y=7 width=\"3\"/>");
        assert_eq!(
            pairs,
            vec![
                pair(0, "a", 1, 0),
                pair(TRAIT_X, "1 2", 2, 0),
                pair(TRAIT_Y, "7", 2, 0),
                pair(TRAIT_WIDTH, "3", 2, 0),
                pair(1, "a", 2, 0),
            ]
        );
    }

    #[test]
    fn parsing_skips_comments() {
        let mut e = tile_engine();
        let (pairs, _) = parse(&mut e, "<!-- <skip x=\"1\"> --><a/>");
        assert_eq!(pairs, vec![pair(0, "a", 1, 0), pair(1, "a", 2, 0)]);
        // A comment between a tag's end and its text too.
        let (pairs, _) = parse(&mut e, "<a><!-- c -->t</a>");
        assert_eq!(
            pairs,
            vec![
                pair(0, "a", 1, 0),
                pair(TAG_VALUE, "t", 2, 1),
                pair(1, "a", 2, 0)
            ]
        );
    }

    #[test]
    fn parsing_text_trims_the_end_but_not_the_middle() {
        let mut e = tile_engine();
        let (pairs, _) = parse(&mut e, "<a>  text  with space  </a>");
        assert_eq!(pairs[1], pair(TAG_VALUE, "text  with space", 2, 1));
    }

    #[test]
    fn parsing_a_constant_expands_game_settings() {
        let mut e = tile_engine();
        e.register(INTERFACE_TEST_CONSTANT_FOR_GAME_SETTINGS, |e, a| {
            if e.mem.cstr(a[0]) == b"&-known;" {
                e.mem.set_cstr(a[1], b"7");
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let (pairs, _) = parse(&mut e, "<a>&-known;</a>");
        assert_eq!(pairs[1], pair(TAG_VALUE, "7", 2, 0));
        // A game setting the interface does not know adds nothing.
        let (pairs, _) = parse(&mut e, "<a>&-other;</a>");
        assert_eq!(pairs.len(), 2);
        // A plain constant is entered as it stands, without the flag.
        let (pairs, _) = parse(&mut e, "<a>&true;</a>");
        assert_eq!(pairs[1], pair(TAG_VALUE, "&true;", 2, 0));
    }

    /// Parses `text`, expecting the reader to stop with `message`.
    fn assert_parse_error(text: &str, message: u32, line: u32) {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        let (_, storage) = parse(&mut e, text);
        assert!(storage.is_null(), "{text}");
        assert_eq!(e.global::<u8>(XML_ERROR_FLAG), 1, "{text}");
        let prints = calls_to(&e, PRINT_ERROR);
        assert_eq!(prints.len(), 1, "{text}");
        assert_eq!(prints[0][0], XML_ERROR_FORMAT);
        assert_eq!(prints[0][1], message, "{text}");
        assert_eq!(prints[0][3], line, "{text}");
        // The path given to the reader is the second argument.
        assert_eq!(string_at(&e, prints[0][2]), "menu.xml");
    }

    #[test]
    fn parsing_reports_malformed_xml() {
        assert_parse_error("<>", MSG_EMPTY_TAG_NAME, 1);
        assert_parse_error("<a x>", MSG_ATTRIBUTE_NO_VALUE, 2);
        assert_parse_error("<a =1>", MSG_MISSING_ATTRIBUTE_NAME, 2);
        assert_parse_error("<a x y=1>", MSG_UNEXPECTED_WORD, 2);
        assert_parse_error("<a x=>", MSG_MISSING_ATTRIBUTE_VALUE, 2);
        assert_parse_error("<a>/></a>", MSG_UNBALANCED_CLOSE_PAIR, 2);
        assert_parse_error("<a>></a>", MSG_UNBALANCED_END_OF_TAG, 2);
    }

    #[test]
    fn parsing_warns_when_the_text_buffer_overflows() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        let text = format!("<a>{}</a>", "b".repeat(0x1001));
        let (pairs, storage) = parse(&mut e, &text);
        assert!(!storage.is_null());
        assert_eq!(pairs[1].1.len(), 0x1001);
        assert_eq!(calls_to(&e, PRINT_ERROR), vec![vec![MSG_BUFFER_TOO_SMALL]]);
    }

    #[test]
    fn parsing_adds_to_the_current_template_of_the_storage() {
        let mut e = tile_engine();
        install_reader_doubles(&mut e);
        // The storage constructor leaves the template pointers to the test.
        let (main, current) = (e.mem.alloc(0x14), e.mem.alloc(0x14));
        let use_current = Rc::new(Cell::new(false));
        let flag = use_current.clone();
        e.register_double(BUILD_STORAGE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], main);
            if flag.get() {
                e.mem.set_u32(a[0] + 0xc, current);
            }
            a[0].into_ret()
        });
        let used = Rc::new(RefCell::new(vec![]));
        let seen = used.clone();
        e.register_double(TEMPLATE_ADD_PAIR, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        let buffer = cstring(&mut e, "<a/>");
        let path = cstring(&mut e, "m.xml");
        for (current_set, expected) in [(false, main), (true, current)] {
            use_current.set(current_set);
            used.borrow_mut().clear();
            e.call(
                0x00a0_1e20,
                &args![Ptr::<()>::new(path), Ptr::<()>::new(buffer), 4u32],
            );
            assert_eq!(used.borrow().clone(), vec![expected, expected]);
        }
    }

    /// Makes `Menu::GetXML` answer for `files` (path, text) and records the
    /// paths asked for.
    fn install_xml_files(
        e: &mut Engine,
        files: Vec<(&'static str, &'static str)>,
    ) -> Rc<RefCell<Vec<String>>> {
        let asked = Rc::new(RefCell::new(vec![]));
        let record = asked.clone();
        e.register_double(MENU_GET_XML, move |e, a| {
            let path = string_at(e, a[0]);
            record.borrow_mut().push(path.clone());
            let Some((_, text)) = files.iter().find(|(name, _)| *name == path) else {
                return 0u32.into_ret();
            };
            xml_storage(e, text).into_ret()
        });
        asked
    }

    /// An `XMLStorage` (size, text) holding `text`.
    fn xml_storage(e: &mut Engine, text: &str) -> u32 {
        let xml = e.mem.alloc(8);
        let data = cstring(e, text);
        e.mem.set_u32(xml, text.len() as u32);
        e.mem.set_u32(xml + 4, data);
        xml
    }

    #[test]
    fn parsing_a_file_reads_it_expands_includes_and_frees_the_text() {
        let mut e = tile_engine();
        let asked = install_xml_files(&mut e, vec![("menu.xml", "<a/>")]);
        let pairs = install_reader_doubles(&mut e);
        e.call_log = Some(vec![]);
        let path = cstring(&mut e, "menu.xml");
        let storage = e
            .call(
                0x00a0_1e20,
                &args![Ptr::<()>::new(path), Ptr::<()>::NULL, 0u32],
            )
            .ptr::<BuildStorage>();
        assert!(!storage.is_null());
        assert_eq!(pairs.borrow().len(), 2);
        assert_eq!(asked.borrow().clone(), vec!["menu.xml"]);
        let freed = calls_to(&e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1], 1);
        assert_eq!(calls_to(&e, MENU_GET_XML), vec![vec![path, 0]]);
    }

    #[test]
    fn parsing_a_missing_file_gives_an_empty_storage() {
        let mut e = tile_engine();
        install_xml_files(&mut e, vec![]);
        let pairs = install_reader_doubles(&mut e);
        e.call_log = Some(vec![]);
        let path = cstring(&mut e, "nothing.xml");
        let storage = e
            .call(
                0x00a0_1e20,
                &args![Ptr::<()>::new(path), Ptr::<()>::NULL, 0u32],
            )
            .ptr::<BuildStorage>();
        assert!(!storage.is_null());
        assert!(pairs.borrow().is_empty());
        assert!(calls_to(&e, XML_STORAGE_SCALAR_DELETING_DESTRUCTOR).is_empty());
    }

    /// Text and string doubles: `strstr` and `strtok_s` of the C library,
    /// and a small model of `BSStringT` (pointer and 16-bit length;
    /// `length_unknown` makes the length 0xffff, as a string whose length
    /// was never computed).
    fn install_string_doubles(e: &mut Engine, length_unknown: bool) {
        // The constants of the exe's data the include expansion uses.
        e.map(QUOTE_DELIMITER, 4);
        e.mem.set_cstr(QUOTE_DELIMITER, b"\"");
        e.mem.set_cstr(INCLUDE_TAG, b"<include");
        e.mem
            .set_cstr(PREFAB_PATH_FORMAT, br"Data\Menus\Prefabs\%s");
        e.register(STRSTR, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            haystack
                .windows(needle.len())
                .position(|window| window == needle.as_slice())
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        e.register(STRTOK_S, |e, a| {
            let delimiters = e.mem.cstr(a[1]);
            let mut at = if a[0] != 0 { a[0] } else { e.mem.u32(a[2]) };
            while e.mem.u8(at) != 0 && delimiters.contains(&e.mem.u8(at)) {
                at += 1;
            }
            if e.mem.u8(at) == 0 {
                e.mem.set_u32(a[2], at);
                return 0u32.into_ret();
            }
            let start = at;
            while e.mem.u8(at) != 0 && !delimiters.contains(&e.mem.u8(at)) {
                at += 1;
            }
            if e.mem.u8(at) != 0 {
                e.mem.set_u8(at, 0);
                at += 1;
            }
            e.mem.set_u32(a[2], at);
            start.into_ret()
        });
        e.register_double(STRING_SET, move |e, a| {
            set_string(
                e,
                a[0],
                &if a[1] == 0 { vec![] } else { e.mem.cstr(a[1]) },
                length_unknown,
            );
            Ret::default()
        });
        e.register_double(STRING_APPEND, move |e, a| {
            if a[1] != 0 {
                let mut text = if e.mem.u32(a[0]) == 0 {
                    vec![]
                } else {
                    e.mem.cstr(e.mem.u32(a[0]))
                };
                text.extend(e.mem.cstr(a[1]));
                set_string(e, a[0], &text, length_unknown);
            }
            Ret::default()
        });
        e.register(STRING_FORMAT, |e, a| {
            assert_eq!(a[1], PREFAB_PATH_FORMAT);
            let mut text = b"Data\\Menus\\Prefabs\\".to_vec();
            text.extend(e.mem.cstr(a[2]));
            set_string(e, a[0], &text, false);
            Ret::default()
        });
    }

    /// Replaces the text of the model `BSStringT` at `string`.
    fn set_string(e: &mut Engine, string: u32, text: &[u8], length_unknown: bool) {
        let old = e.mem.u32(string);
        if old != 0 {
            e.mem.free(old);
        }
        if text.is_empty() {
            e.mem.set_u32(string, 0);
            e.mem.set_u16(string + 4, 0);
            return;
        }
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text);
        e.mem.set_u32(string, block);
        let length = if length_unknown {
            0xffff
        } else {
            text.len() as u16
        };
        e.mem.set_u16(string + 4, length);
    }

    /// Runs `InsertIncludedFiles` over `text`; the text and size after, and
    /// the result.
    fn insert_includes(e: &mut Engine, text: &str) -> (bool, String, u32) {
        let xml = xml_storage(e, text);
        let result = e.call(0x00a0_2d40, &args![Ptr::<()>::new(xml)]).bool();
        (result, string_at(e, e.mem.u32(xml + 4)), e.mem.u32(xml))
    }

    #[test]
    fn includes_are_replaced_by_the_prefab_text() {
        for length_unknown in [false, true] {
            let mut e = tile_engine();
            install_string_doubles(&mut e, length_unknown);
            let asked = install_xml_files(&mut e, vec![("Data\\Menus\\Prefabs\\p.xml", "[X]")]);
            let (changed, text, size) = insert_includes(&mut e, "AA<include src=\"p.xml\"/>BB");
            assert!(changed);
            assert_eq!(text, "AA[X]BB");
            assert_eq!(size, 7);
            assert_eq!(asked.borrow().clone(), vec!["Data\\Menus\\Prefabs\\p.xml"]);
        }
    }

    #[test]
    fn several_includes_and_nested_includes_are_all_expanded() {
        let mut e = tile_engine();
        install_string_doubles(&mut e, false);
        install_xml_files(
            &mut e,
            vec![
                ("Data\\Menus\\Prefabs\\p", "1"),
                ("Data\\Menus\\Prefabs\\q", "2"),
                ("Data\\Menus\\Prefabs\\n", "[<include src=\"q\"/>]"),
            ],
        );
        let (changed, text, size) =
            insert_includes(&mut e, "A<include src=\"p\"/>B<include src=\"q\"/>C");
        assert!(changed);
        assert_eq!(text, "A1B2C");
        assert_eq!(size, 5);
        let (_, text, _) = insert_includes(&mut e, "A<include src=\"n\"/>B");
        assert_eq!(text, "A[2]B");
    }

    #[test]
    fn text_without_includes_is_left_alone() {
        let mut e = tile_engine();
        install_string_doubles(&mut e, false);
        let xml = xml_storage(&mut e, "<a/>");
        let before = e.mem.u32(xml + 4);
        assert!(!e.call(0x00a0_2d40, &args![Ptr::<()>::new(xml)]).bool());
        assert_eq!(e.mem.u32(xml + 4), before);
        assert_eq!(e.mem.u32(xml), 4);
    }

    #[test]
    fn included_files_have_their_prefab_storage_freed() {
        let mut e = tile_engine();
        install_string_doubles(&mut e, false);
        install_xml_files(&mut e, vec![("Data\\Menus\\Prefabs\\p", "1")]);
        e.call_log = Some(vec![]);
        insert_includes(&mut e, "<include src=\"p\"/>");
        // The prefab's storage object is destroyed in place and then given back to the memory
        // manager (a block allocated after that may take its place again).
        let destroyed = calls_to(&e, XML_STORAGE_DESTROY);
        assert_eq!(destroyed.len(), 1);
        let prefab = destroyed[0][0];
        let log = e.call_log.as_ref().unwrap();
        let destroyed_at = log
            .iter()
            .position(|(called, _)| *called == XML_STORAGE_DESTROY);
        let freed_at = log.iter().position(|(called, args)| {
            *called == MEMORY_DEALLOCATE && *args == vec![MEMORY_MANAGER, prefab]
        });
        assert!(
            matches!((destroyed_at, freed_at), (Some(d), Some(f)) if d < f),
            "{destroyed_at:?} {freed_at:?}"
        );
    }

    /// Appends `element` to the `NiTPointerList` at `list`.
    fn push_list(e: &mut Engine, list: Ptr<NiTPointerList>, element: u32) {
        let node = e.mem.alloc(12);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        e.mem.set_u32(node + 4, tail);
        e.mem.set_u32(node + 8, element);
        if tail == 0 {
            e.set(list, NiTPointerList::m_pkHead, node);
        } else {
            e.mem.set_u32(tail, node);
        }
        e.set(list, NiTPointerList::m_pkTail, node);
        let count = e.get(list, NiTPointerList::m_uiCount);
        e.set(list, NiTPointerList::m_uiCount, count + 1);
    }

    fn make_template(e: &mut Engine) -> Ptr<TileTemplate> {
        Ptr::new(e.mem.alloc(0x14))
    }

    /// Appends an item (`command`, number, text, argument) to a template.
    fn add_item(
        e: &mut Engine,
        template: Ptr<TileTemplate>,
        command: i32,
        value: f32,
        text: &str,
        argument: u32,
    ) -> Ptr<TileTemplateItem> {
        let item: Ptr<TileTemplateItem> = Ptr::new(e.mem.alloc(0x18));
        e.set(item, TileTemplateItem::iCmd, command);
        e.set(item, TileTemplateItem::fVal, value);
        if !text.is_empty() {
            let string = cstring(e, text);
            e.mem.set_u32(item.addr() + 8, string);
            e.mem.set_u16(item.addr() + 0xc, text.len() as u16);
        }
        e.set(item, TileTemplateItem::u, argument);
        push_list(e, template.at(TileTemplate::xList), item.addr());
        item
    }

    /// Records the calls to `addr` as (first word of the first argument's
    /// `eIndex`, second argument, third argument): the trait id of the
    /// value a setter was called on.
    fn record_value_calls(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<(i32, u32, u32)>>> {
        let calls = Rc::new(RefCell::new(vec![]));
        let record = calls.clone();
        e.register_double(addr, move |e, a| {
            record.borrow_mut().push((e.mem.i32(a[0]), a[1], a[2]));
            Ret::default()
        });
        calls
    }

    /// Records the (tile, flags) of every `AddNeedsUpdate`.
    fn record_updates(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let updates = Rc::new(RefCell::new(vec![]));
        let record = updates.clone();
        e.register_double(TILE_ADD_NEEDS_UPDATE, move |_, a| {
            record.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        updates
    }

    #[test]
    fn building_the_tree_creates_names_and_nests_the_tiles() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        e.set_global(HOT_RECT_TYPE_VALUE, 906.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(TWO_FIFTY_FIVE, 255.0f32);
        e.set_global(DEFAULT_ATLAS_NAME, 0x7777u32);
        // `Init` (slot +4) of the rect and of the hot-rect (image) class.
        let inits = Rc::new(RefCell::new(vec![]));
        let record = inits.clone();
        e.register_double(0x0600_4000, move |e, a| {
            record.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0] + 0x28, a[1]);
            Ret::default()
        });
        e.put_vtable(VTABLE_TILE_RECT, &[0, 0x0600_4000]);
        e.put_vtable(0x0600_4100, &[0, 0x0600_4000]);
        e.register(TILE_IMAGE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0600_4100);
            a[0].into_ret()
        });
        let floats = record_value_calls(&mut e, VALUE_SET_FLOAT);
        let strings = record_value_calls(&mut e, VALUE_SET_STRING);
        let updates = record_updates(&mut e);

        let top = typed_tile(&mut e, TYPE_RECT);
        let template = make_template(&mut e);
        let (root_name, hot_name, other_name) = (
            cstring(&mut e, "root"),
            cstring(&mut e, "hot"),
            cstring(&mut e, "other"),
        );
        let mut items = vec![];
        for (command, number, name) in [
            (TI_TILE_START, 901.0, root_name),
            (TI_TILE_START, 906.0, hot_name),
            (TI_TILE_END, 0.0, 0),
            (TI_TILE_START, 901.0, other_name),
            (TI_TILE_END, 0.0, 0),
            (TI_TILE_END, 0.0, 0),
        ] {
            let item = add_item(&mut e, template, command, number, "", 0);
            e.mem.set_u32(item.addr() + 8, name);
            items.push(item);
        }

        let first = e.call(0x00a0_31b0, &args![top, template]).ptr::<Tile>();

        let tile_of =
            |e: &Engine, index: usize| Ptr::<Tile>::new(e.get(items[index], TileTemplateItem::u));
        let (root, hot, other) = (tile_of(&e, 0), tile_of(&e, 1), tile_of(&e, 3));
        assert_eq!(first, root);
        assert!(!root.is_null() && !hot.is_null() && !other.is_null());
        // Each tile is initialized under its parent with its name.
        assert_eq!(
            inits.borrow().clone(),
            vec![
                vec![root.addr(), top.addr(), root_name, 0],
                vec![hot.addr(), root.addr(), hot_name, 0],
                vec![other.addr(), root.addr(), other_name, 0],
            ]
        );
        // The hot rect (type 906) gets its fixed look.
        let bits = |value: f32| value.to_bits();
        assert_eq!(
            floats.borrow().clone(),
            vec![
                (TRAIT_ID, bits(-1.0), 1),
                (TRAIT_TARGET, bits(1.0), 1),
                (TRAIT_BRIGHTNESS, bits(-1.0), 1),
                (TRAIT_RED, bits(255.0), 1),
                (TRAIT_GREEN, bits(255.0), 1),
                (TRAIT_BLUE, bits(255.0), 1),
                (TRAIT_ALPHA, bits(0.0), 1),
            ]
        );
        assert_eq!(
            strings.borrow().clone(),
            vec![
                (TRAIT_FILENAME, SOLID_TEXTURE_NAME, 1),
                (TRAIT_TEX_ATLAS, 0x7777, 1)
            ]
        );
        assert_eq!(updates.borrow().clone(), vec![(hot.addr(), UPDATE_TEXTURE)]);
    }

    #[test]
    fn building_the_tree_stops_when_a_tile_cannot_be_made() {
        let mut e = tile_engine();
        let top = typed_tile(&mut e, TYPE_RECT);
        let template = make_template(&mut e);
        // 907 is the window type, which `BuildTile` does not make.
        add_item(&mut e, template, TI_TILE_START, 907.0, "", 0);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a0_31b0, &args![top, template]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![MSG_UNABLE_TO_CREATE_TILE]]
        );
    }

    #[test]
    fn building_an_empty_template_gives_nothing() {
        let mut e = tile_engine();
        let top = typed_tile(&mut e, TYPE_RECT);
        let template = make_template(&mut e);
        assert_eq!(e.call(0x00a0_31b0, &args![top, template]).u32(), 0);
    }

    /// Items for `ConnectTraitsToTree` tests: a template whose first item
    /// starts `tile`.
    fn connect_template(e: &mut Engine, tile: Ptr<Tile>) -> Ptr<TileTemplate> {
        let template = make_template(e);
        add_item(e, template, TI_TILE_START, 0.0, "", tile.addr());
        template
    }

    fn connect(e: &mut Engine, template: Ptr<TileTemplate>) {
        let unused = typed_tile(e, TYPE_RECT);
        e.call(0x00a0_33b0, &args![unused, template]);
    }

    #[test]
    fn connecting_simple_traits_sets_floats_and_strings() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        e.set_global(NO_TRAIT_VALUE, -2147483648.0f64);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let template = connect_template(&mut e, tile);
        let none = -2147483648.0f32;
        add_item(&mut e, template, TI_SIMPLE_TRAIT, 5.0, "", TRAIT_X as u32);
        // No number but a text: a string, whose length may be unknown.
        let with_text = add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            none,
            "w",
            TRAIT_WIDTH as u32,
        );
        e.mem.set_u16(with_text.addr() + 0xc, 0xffff);
        // No number and no text: the number itself is set.
        add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            none,
            "",
            TRAIT_HEIGHT as u32,
        );
        // String traits are strings whatever the number.
        add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            1.0,
            "s",
            TRAIT_STRING as u32,
        );
        add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            1.0,
            "f.dds",
            TRAIT_FILENAME as u32,
        );
        let name = add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            none,
            "nm",
            TAG_NAME as u32,
        );
        let floats = record_value_calls(&mut e, VALUE_SET_FLOAT);
        let strings = record_value_calls(&mut e, VALUE_SET_STRING);
        let updates = record_updates(&mut e);
        e.call_log = Some(vec![]);

        connect(&mut e, template);

        assert_eq!(
            floats.borrow().clone(),
            vec![
                (TRAIT_X, 5.0f32.to_bits(), 1),
                (TRAIT_HEIGHT, none.to_bits(), 1)
            ]
        );
        let text_of = |e: &Engine, item: usize| {
            let items = e.get(template.at(TileTemplate::xList), NiTPointerList::m_pkHead);
            let mut node = items;
            for _ in 0..item {
                node = e.mem.u32(node);
            }
            e.mem.u32(e.mem.u32(node + 8) + 8)
        };
        assert_eq!(
            strings.borrow().clone(),
            vec![
                (TRAIT_WIDTH, text_of(&e, 2), 1),
                (TRAIT_STRING, text_of(&e, 4), 1),
                (TRAIT_FILENAME, text_of(&e, 5), 1),
            ]
        );
        // The name tag becomes the tile's name.
        assert_eq!(
            calls_to(&e, STRING_ASSIGN),
            vec![vec![tile.addr() + 0x20, name.addr() + 8]]
        );
        // A rect tile's file name does not need a texture update.
        assert_eq!(
            updates.borrow().clone(),
            vec![(tile.addr(), FLAG_TILE_LOADING)]
        );
    }

    #[test]
    fn connecting_an_image_file_name_queues_a_texture_update() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        e.set_global(NO_TRAIT_VALUE, -2147483648.0f64);
        let tile = typed_tile(&mut e, TYPE_IMAGE);
        let template = connect_template(&mut e, tile);
        add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            0.0,
            "a.dds",
            TRAIT_FILENAME as u32,
        );
        let strings = record_value_calls(&mut e, VALUE_SET_STRING);
        let updates = record_updates(&mut e);
        connect(&mut e, template);
        assert_eq!(strings.borrow().len(), 1);
        assert_eq!(
            updates.borrow().clone(),
            vec![
                (tile.addr(), FLAG_TILE_LOADING),
                (tile.addr(), UPDATE_TEXTURE)
            ]
        );
    }

    #[test]
    fn connecting_actions_adds_them_to_the_current_trait() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let template = connect_template(&mut e, tile);
        add_item(&mut e, template, TI_TRAIT_START, TRAIT_X as f32, "", 0);
        add_item(&mut e, template, TI_SIMPLE_ACTION, 2.0, "", 0x7d1);
        add_item(&mut e, template, TI_TRAIT_LINK, 2.5, "link", 0x7d2);
        add_item(&mut e, template, TI_ACTION_START, 2.6, "", 0);
        add_item(&mut e, template, TI_ACTION_END, 3.5, "", 0);
        add_item(&mut e, template, TI_TRAIT_END, 0.0, "", 0);
        // Out of any trait: a message.
        add_item(&mut e, template, TI_SIMPLE_ACTION, 2.0, "", 0x7d1);
        e.register(TILE_FIND_BY_PATH, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"link");
            0x5555u32.into_ret()
        });
        let actions = record_value_calls(&mut e, VALUE_ADD_ACTION);
        let links = Rc::new(RefCell::new(vec![]));
        let record = links.clone();
        e.register_double(VALUE_ADD_ACTION_OV2, move |e, a| {
            record
                .borrow_mut()
                .push((e.mem.i32(a[0]), a[1], a[2], a[3]));
            Ret::default()
        });
        e.call_log = Some(vec![]);

        connect(&mut e, template);

        let bits = |value: f32| value.to_bits();
        assert_eq!(
            actions.borrow().clone(),
            vec![
                (TRAIT_X, 0x7d1, bits(2.0)),
                // 2.6 rounds to 3, 3.5 to 4 (ties to even).
                (TRAIT_X, ACTION_LEFT_PAREN as u32, bits(3.0)),
                (TRAIT_X, ACTION_RIGHT_PAREN as u32, bits(4.0)),
            ]
        );
        // 2.5 rounds to 2 (ties to even).
        assert_eq!(links.borrow().clone(), vec![(TRAIT_X, 0x7d2, 0x5555, 2)]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![MSG_ACTION_OUTSIDE_TRAIT]]
        );
    }

    #[test]
    fn an_action_link_to_nothing_adds_nothing() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let template = connect_template(&mut e, tile);
        add_item(&mut e, template, TI_TRAIT_START, TRAIT_X as f32, "", 0);
        add_item(&mut e, template, TI_TRAIT_LINK, 1.0, "gone", 0x7d2);
        e.call_log = Some(vec![]);
        connect(&mut e, template);
        assert!(calls_to(&e, VALUE_ADD_ACTION_OV2).is_empty());
    }

    #[test]
    fn items_out_of_place_print_messages() {
        let mut e = tile_engine();
        let template = make_template(&mut e);
        add_item(&mut e, template, TI_SIMPLE_TRAIT, 1.0, "", TRAIT_X as u32);
        add_item(&mut e, template, TI_TRAIT_LINK, 1.0, "l", 0x7d2);
        add_item(&mut e, template, TI_ACTION_START, 1.0, "", 0);
        add_item(&mut e, template, TI_ACTION_END, 1.0, "", 0);
        e.call_log = Some(vec![]);
        connect(&mut e, template);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![
                vec![MSG_TRAIT_OUTSIDE_TILE],
                vec![MSG_ACTION_LINK_OUTSIDE_TRAIT],
                vec![MSG_ACTION_BEGUN_OUTSIDE_TRAIT],
                vec![MSG_ACTION_ENDED_OUTSIDE_TRAIT],
            ]
        );
    }

    #[test]
    fn ending_a_tile_calculates_its_values_and_returns_to_the_parent() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let parent = typed_tile(&mut e, TYPE_RECT);
        let child = typed_tile(&mut e, TYPE_RECT);
        e.set(child, Tile::pParent, parent);
        let x = give_trait(&mut e, child, TRAIT_X, 1.0);
        let class = give_trait(&mut e, child, TRAIT_CLASS, 2.0);
        let _ = class;
        let y = give_trait(&mut e, child, TRAIT_Y, 3.0);
        e.set(child, Tile::uiFlags, FLAG_TILE_LOADING | UPDATE_COLOR);
        let template = make_template(&mut e);
        add_item(&mut e, template, TI_TILE_START, 0.0, "", parent.addr());
        add_item(&mut e, template, TI_TILE_START, 0.0, "", child.addr());
        add_item(&mut e, template, TI_TILE_END, 0.0, "", 0);
        // Back on the parent tile.
        add_item(
            &mut e,
            template,
            TI_SIMPLE_TRAIT,
            4.0,
            "",
            TRAIT_WIDTH as u32,
        );
        let floats = record_value_calls(&mut e, VALUE_SET_FLOAT);
        e.call_log = Some(vec![]);

        connect(&mut e, template);

        // The loading flag is cleared, the other bits stay.
        assert_eq!(e.get(child, Tile::uiFlags), UPDATE_COLOR);
        // Every value but the class is calculated (with the flag 1).
        assert_eq!(
            calls_to(&e, VALUE_CALCULATE_VALUE),
            vec![vec![x.addr(), 1], vec![y.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
        // The width went to the parent tile.
        let width = e.mem.u32(e.mem.u32(parent.addr() + 0x14));
        assert_eq!(e.mem.i32(width), TRAIT_WIDTH);
        assert_eq!(floats.borrow().len(), 1);
        assert_eq!(e.get(child, Tile::pParent), parent);
    }

    /// A `Menu` object, and the log of what is queued for tiles.
    fn menu_with_virtual_slot(e: &mut Engine) -> (Ptr, Rc<RefCell<Vec<Vec<u32>>>>) {
        let menu = Ptr::new(e.mem.alloc(0x28));
        e.put_vtable(0x0600_5000, &[0, 0x0600_5100]);
        e.mem.set_u32(menu.addr(), 0x0600_5000);
        let calls = Rc::new(RefCell::new(vec![]));
        let record = calls.clone();
        e.register_double(0x0600_5100, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (menu, calls)
    }

    /// What `FinalPostParse` queues for `trait_id` on a tile of `kind`,
    /// and whether it answers the tile.
    fn post_parse(
        kind: u32,
        trait_id: i32,
        setup: impl FnOnce(&mut Engine, Ptr<Tile>),
    ) -> (Vec<u32>, bool) {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, kind);
        setup(&mut e, tile);
        let updates = record_updates(&mut e);
        let result = e
            .call(0x00a0_37e0, &args![tile, trait_id, 1.0f32, 0u32])
            .u32();
        let flags = updates.borrow().iter().map(|(_, flags)| *flags).collect();
        (flags, result == tile.addr())
    }

    fn no_setup(_: &mut Engine, _: Ptr<Tile>) {}

    fn with_clip_window(e: &mut Engine, tile: Ptr<Tile>) {
        give_trait(e, tile, TRAIT_CLIP_WINDOW, 1.0);
    }

    #[test]
    fn post_parse_of_a_position_trait_queues_a_position_update() {
        for trait_id in [TRAIT_X, TRAIT_Y] {
            assert_eq!(
                post_parse(TYPE_RECT, trait_id, no_setup),
                (vec![UPDATE_POSITION], true)
            );
            assert_eq!(
                post_parse(TYPE_RECT, trait_id, with_clip_window),
                (vec![UPDATE_SCISSOR_WINDOW, UPDATE_POSITION], true)
            );
        }
    }

    #[test]
    fn post_parse_of_the_depth_raises_the_menus_depth() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        // root <- menu tile <- tile; the tile has a locus and a depth of 2.
        let root = typed_tile(&mut e, TYPE_RECT);
        let (menu_tile, menu) = make_menu_tile(&mut e, 1);
        let tile = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, root, menu_tile);
        add_child(&mut e, menu_tile, tile);
        give_trait(&mut e, tile, TRAIT_LOCUS, 1.0);
        give_trait(&mut e, tile, TRAIT_DEPTH, 2.0);
        let updates = record_updates(&mut e);
        // 3.0 plus the tile's own 2.0 is more than the menu's 1.
        let result = e.call(0x00a0_37e0, &args![tile, TRAIT_DEPTH, 3.0f32, 0u32]);
        assert_eq!(result.u32(), tile.addr());
        assert_eq!(e.mem.i32(menu.addr() + 0x18), 5);
        assert_eq!(
            updates
                .borrow()
                .iter()
                .map(|(_, flags)| *flags)
                .collect::<Vec<_>>(),
            vec![UPDATE_POSITION]
        );
        // A smaller total leaves the menu's depth alone.
        e.mem.set_i32(menu.addr() + 0x18, 9);
        e.call(0x00a0_37e0, &args![tile, TRAIT_DEPTH, 3.0f32, 0u32]);
        assert_eq!(e.mem.i32(menu.addr() + 0x18), 9);
        // The menu tile itself never changes its own menu's depth.
        e.mem.set_i32(menu.addr() + 0x18, 0);
        give_trait(&mut e, menu_tile, TRAIT_DEPTH, 1.0);
        e.call(0x00a0_37e0, &args![menu_tile, TRAIT_DEPTH, 6.0f32, 0u32]);
        assert_eq!(e.mem.i32(menu.addr() + 0x18), 0);
    }

    #[test]
    fn post_parse_of_text_traits_queues_a_create_update() {
        for trait_id in [
            TRAIT_WRAP_WIDTH,
            TRAIT_WRAP_LIMIT,
            TRAIT_WRAP_LINES,
            TRAIT_LINE_GAP,
            TRAIT_PAGE_NUM,
            TRAIT_IS_HTML,
            TRAIT_FONT,
            TRAIT_JUSTIFY,
        ] {
            assert_eq!(
                post_parse(TYPE_TEXT, trait_id, no_setup),
                (vec![UPDATE_CREATE], true),
                "{trait_id:#x}"
            );
            // Not for other kinds of tile.
            assert_eq!(post_parse(TYPE_IMAGE, trait_id, no_setup), (vec![], false));
        }
    }

    #[test]
    fn post_parse_of_the_size_of_an_image_queues_a_geometry_update() {
        for trait_id in [TRAIT_WIDTH, TRAIT_HEIGHT, TRAIT_CROP_X, TRAIT_CROP_Y] {
            assert_eq!(
                post_parse(TYPE_IMAGE, trait_id, no_setup),
                (vec![UPDATE_GEOMETRY], true)
            );
            assert_eq!(
                post_parse(TYPE_IMAGE, trait_id, with_clip_window),
                (vec![UPDATE_SCISSOR_WINDOW, UPDATE_GEOMETRY], true)
            );
            assert_eq!(post_parse(TYPE_TEXT, trait_id, no_setup), (vec![], false));
        }
    }

    #[test]
    fn post_parse_of_the_clip_window() {
        assert_eq!(
            post_parse(TYPE_IMAGE, TRAIT_CLIP_WINDOW, no_setup),
            (vec![UPDATE_SCISSOR_WINDOW], true)
        );
        assert_eq!(
            post_parse(TYPE_RECT, TRAIT_CLIP_WINDOW, no_setup),
            (vec![UPDATE_SCISSOR_WINDOW], true)
        );
        assert_eq!(
            post_parse(TYPE_TEXT, TRAIT_CLIP_WINDOW, no_setup),
            (vec![], false)
        );
    }

    #[test]
    fn post_parse_of_clips_passes_the_value_to_the_children() {
        assert_eq!(
            post_parse(TYPE_RECT, TRAIT_CLIPS, no_setup),
            (vec![], false)
        );
        for kind in [TYPE_IMAGE, TYPE_TEXT] {
            let mut e = tile_engine();
            install_value_array_doubles(&mut e);
            let tile = typed_tile(&mut e, kind);
            let (first, second) = (typed_tile(&mut e, TYPE_RECT), typed_tile(&mut e, TYPE_RECT));
            add_child(&mut e, tile, first);
            add_child(&mut e, tile, second);
            give_trait(&mut e, tile, TRAIT_CLIPS, 1.0);
            let updates = record_updates(&mut e);
            let floats = record_value_calls(&mut e, VALUE_SET_FLOAT);
            let result = e.call(0x00a0_37e0, &args![tile, TRAIT_CLIPS, 1.0f32, 0u32]);
            assert_eq!(result.u32(), tile.addr());
            assert_eq!(
                updates.borrow().clone(),
                vec![(tile.addr(), UPDATE_SCISSOR)]
            );
            // Each child's clips trait is set to the tile's.
            assert_eq!(
                floats.borrow().clone(),
                vec![
                    (TRAIT_CLIPS, 1.0f32.to_bits(), 1),
                    (TRAIT_CLIPS, 1.0f32.to_bits(), 1)
                ]
            );
        }
    }

    #[test]
    fn post_parse_of_visibility_color_and_locus() {
        assert_eq!(
            post_parse(TYPE_RECT, TRAIT_VISIBLE, no_setup),
            (vec![UPDATE_VISIBILITY], true)
        );
        for trait_id in [
            TRAIT_ALPHA,
            TRAIT_RED,
            TRAIT_GREEN,
            TRAIT_BLUE,
            TRAIT_SYSTEM_COLOR,
            TRAIT_BRIGHTNESS,
        ] {
            assert_eq!(
                post_parse(TYPE_IMAGE, trait_id, no_setup),
                (vec![UPDATE_COLOR], true)
            );
            assert_eq!(
                post_parse(TYPE_TEXT, trait_id, no_setup),
                (vec![UPDATE_COLOR], true)
            );
            // A tile of another kind has no colour to update, but the trait
            // is still handled.
            assert_eq!(post_parse(TYPE_RECT, trait_id, no_setup), (vec![], true));
        }
        assert_eq!(
            post_parse(TYPE_RECT, TRAIT_LOCUS, no_setup),
            (vec![UPDATE_LOCUS], true)
        );
    }

    #[test]
    fn post_parse_of_a_file_name_needs_a_file() {
        let with_file = |e: &mut Engine, tile: Ptr<Tile>| {
            let value = give_trait(e, tile, TRAIT_FILENAME, 0.0);
            e.set(value, TileValue::strValue, Ptr::new(0x7000));
        };
        assert_eq!(
            post_parse(TYPE_IMAGE, TRAIT_FILENAME, with_file),
            (vec![UPDATE_TEXTURE], true)
        );
        assert_eq!(
            post_parse(TYPE_3D, TRAIT_FILENAME, with_file),
            (vec![UPDATE_NIF_FILE], true)
        );
        // Without a file, or on a tile that shows none: nothing.
        assert_eq!(
            post_parse(TYPE_IMAGE, TRAIT_FILENAME, no_setup),
            (vec![], false)
        );
        assert_eq!(
            post_parse(TYPE_3D, TRAIT_FILENAME, no_setup),
            (vec![], false)
        );
        assert_eq!(
            post_parse(TYPE_RECT, TRAIT_FILENAME, with_file),
            (vec![], false)
        );
    }

    #[test]
    fn post_parse_of_rotation_and_unknown_traits() {
        for trait_id in [TRAIT_ROTATE_ANGLE, TRAIT_ROTATE_AXIS_X, TRAIT_ROTATE_AXIS_Y] {
            assert_eq!(
                post_parse(TYPE_RECT, trait_id, no_setup),
                (vec![UPDATE_POSITION], true)
            );
        }
        assert_eq!(post_parse(TYPE_RECT, 0xfa5, no_setup), (vec![], false));
        assert_eq!(post_parse(TYPE_RECT, TRAIT_NONE, no_setup), (vec![], false));
    }

    #[test]
    fn post_parse_of_the_id_tells_the_menu() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let (menu_tile, menu) = make_menu_tile(&mut e, 0);
        let (hidden, calls) = menu_with_virtual_slot(&mut e);
        // The menu object of the menu tile answers a virtual call.
        e.mem.set_u32(menu_tile.addr() + 0x3c, hidden.addr());
        let _ = menu;
        let updates = record_updates(&mut e);
        let result = e.call(0x00a0_37e0, &args![menu_tile, TRAIT_ID, 42.9f32, 0u32]);
        // The id is truncated; the menu is called with it and the tile; the
        // trait itself queues nothing and is not answered.
        assert_eq!(
            calls.borrow().clone(),
            vec![vec![hidden.addr(), 42, menu_tile.addr()]]
        );
        assert_eq!(result.u32(), 0);
        assert!(updates.borrow().is_empty());
    }

    #[test]
    fn the_menu_tile_is_the_child_of_the_root() {
        let mut e = tile_engine();
        let root = typed_tile(&mut e, TYPE_RECT);
        let (a, b, c) = (
            typed_tile(&mut e, TYPE_MENU),
            typed_tile(&mut e, TYPE_RECT),
            typed_tile(&mut e, TYPE_RECT),
        );
        add_child(&mut e, root, a);
        add_child(&mut e, a, b);
        add_child(&mut e, b, c);
        e.call_log = Some(vec![]);
        for (from, expected) in [(c, a), (b, a), (a, a), (root, root)] {
            assert_eq!(e.call(0x00a0_3d40, &args![from]).u32(), expected.addr());
        }
        assert_eq!(e.call(0x00a0_3d40, &args![Ptr::<Tile>::NULL]).u32(), 0);
        assert_lock_balanced(&e);
    }

    #[test]
    fn the_menu_comes_from_a_menu_type_menu_tile() {
        let mut e = tile_engine();
        let root = typed_tile(&mut e, TYPE_RECT);
        let (menu_tile, menu) = make_menu_tile(&mut e, 0);
        let leaf = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, root, menu_tile);
        add_child(&mut e, menu_tile, leaf);
        e.mem.set_u8(menu.addr() + 0x1c, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a0_3c90, &args![leaf]).u32(), menu.addr());
        assert_eq!(e.call(0x00a0_3c90, &args![menu_tile]).u32(), menu.addr());
        assert_lock_balanced(&e);
        assert!(e.call(0x00a0_3c60, &args![leaf]).bool());
        e.mem.set_u8(menu.addr() + 0x1c, 0);
        assert!(!e.call(0x00a0_3c60, &args![leaf]).bool());

        // A menu tile of another type has no menu.
        let other_root = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_IMAGE);
        add_child(&mut e, other_root, other);
        assert_eq!(e.call(0x00a0_3c90, &args![other]).u32(), 0);
        assert_eq!(e.call(0x00a0_3c90, &args![Ptr::<Tile>::NULL]).u32(), 0);
    }

    #[test]
    fn child_names_are_searched_depth_first() {
        let mut e = tile_engine();
        e.register(STRCMP, |e, a| {
            let (first, second) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (first.cmp(&second) as i32).into_ret()
        });
        let root = typed_tile(&mut e, TYPE_RECT);
        let names = ["a", "target", "b", "target", "c"];
        let tiles: Vec<Ptr<Tile>> = names
            .iter()
            .map(|name| {
                let tile = typed_tile(&mut e, TYPE_RECT);
                let string = cstring(&mut e, name);
                e.mem.set_u32(tile.addr() + 0x20, string);
                tile
            })
            .collect();
        // root -> a -> [target (inner)], root -> b -> [target (second)],
        // and a nameless tile, and "c" last.
        add_child(&mut e, root, tiles[0]);
        add_child(&mut e, tiles[0], tiles[1]);
        let nameless = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, root, nameless);
        add_child(&mut e, root, tiles[2]);
        add_child(&mut e, tiles[2], tiles[3]);
        add_child(&mut e, root, tiles[4]);
        e.call_log = Some(vec![]);
        let target = cstring(&mut e, "target");
        let find = |e: &mut Engine, name: u32| {
            e.call(0x00a0_3da0, &args![root, Ptr::<()>::new(name)])
                .u32()
        };
        // Depth first: the inner tile under "a" comes before the one under "b".
        assert_eq!(find(&mut e, target), tiles[1].addr());
        let c = cstring(&mut e, "c");
        assert_eq!(find(&mut e, c), tiles[4].addr());
        let b = cstring(&mut e, "b");
        assert_eq!(find(&mut e, b), tiles[2].addr());
        let missing = cstring(&mut e, "missing");
        assert_eq!(find(&mut e, missing), 0);
        assert_eq!(find(&mut e, 0), 0);
        assert_lock_balanced(&e);
    }

    /// The tiles `ReadFile` works on: a parent tile and a template with one
    /// tile and a sub-template, where the tile's class is a menu.
    struct ReadSetup {
        parent: Ptr<Tile>,
        path: u32,
        menu: u32,
        storage: Rc<Cell<u32>>,
    }

    fn read_setup(e: &mut Engine, xml_text: &'static str) -> ReadSetup {
        install_value_array_doubles(e);
        install_xml_files(e, vec![("menu.xml", xml_text)]);
        install_reader_doubles(e);
        let parent = typed_tile(e, TYPE_RECT);
        let menu = e.mem.alloc(0x28);
        // The rect constructor's tile is a menu-type tile with that menu.
        e.put_vtable(VTABLE_TILE_RECT, &[0, 0x0600_6000, 0, 0x0600_6100]);
        e.register(0x0600_6000, |_, _| Ret::default());
        e.register(0x0600_6100, |_, _| TYPE_MENU.into_ret());
        e.register_double(TILE_BASE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x3c, menu);
            a[0].into_ret()
        });
        // The storage: its template holds one tile start and end; one
        // sub-template is registered.
        let storage = Rc::new(Cell::new(0));
        let made = storage.clone();
        e.register_double(BUILD_STORAGE_CONSTRUCT, move |e, a| {
            let template = make_template(e);
            add_item(e, template, TI_TILE_START, 901.0, "", 0);
            add_item(e, template, TI_TILE_END, 0.0, "", 0);
            e.mem.set_u32(a[0], template.addr());
            e.mem.set_u32(a[0] + 4, 0xcafe);
            made.set(a[0]);
            a[0].into_ret()
        });
        let path = cstring(e, "menu.xml");
        ReadSetup {
            parent,
            path,
            menu,
            storage,
        }
    }

    #[test]
    fn reading_a_file_builds_the_tree_and_updates_it() {
        let mut e = tile_engine();
        let setup = read_setup(&mut e, "");
        e.set_global(INTERFACE_LOCK_OWNER, 0x5000_0000u32);
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        e.set_global(XML_ERROR_FLAG, 1u8);
        e.call_log = Some(vec![]);

        let tile = e
            .call(
                0x00a0_1b00,
                &args![setup.parent, Ptr::<()>::new(setup.path)],
            )
            .ptr::<Tile>();

        assert!(!tile.is_null());
        assert_eq!(e.mem.u32(tile.addr() + 0x3c), setup.menu);
        // The sub-template is registered with the menu, which is marked as
        // deleting its templates.
        assert_eq!(
            calls_to(&e, MENU_ADD_TEMPLATE),
            vec![vec![setup.menu, 0xcafe]]
        );
        assert_eq!(e.mem.u8(setup.menu + 0x1d), 1);
        // The parse storage is destroyed and freed.
        assert_eq!(
            calls_to(&e, BUILD_STORAGE_DESTROY),
            vec![vec![setup.storage.get()]]
        );
        assert_eq!(e.mem.block_size(setup.storage.get()), None);
        // The new tree is updated under the interface lock.
        assert_eq!(
            calls_to(&e, TILE_UPDATE_CHILDREN),
            vec![vec![tile.addr(), 0]]
        );
        let log = e.call_log.as_ref().unwrap();
        let position = |addr: u32, args: Vec<u32>| {
            log.iter()
                .position(|(called, logged)| *called == addr && *logged == args)
                .unwrap()
        };
        let update = position(TILE_UPDATE_CHILDREN, vec![tile.addr(), 0]);
        assert!(position(ENTER_CRITICAL_SECTION, vec![0x5000_0080]) < update);
        assert!(update < position(LEAVE_CRITICAL_SECTION, vec![0x5000_0080]));
        // The error flag was cleared before and after; the line word is back.
        assert_eq!(e.global::<u8>(XML_ERROR_FLAG), 0);
        assert_eq!(e.mem.u32(line_word), 0x77);
    }

    #[test]
    fn reading_a_broken_file_gives_nothing() {
        let mut e = tile_engine();
        let setup = read_setup(&mut e, "<>");
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        e.call_log = Some(vec![]);
        let tile = e.call(
            0x00a0_1b00,
            &args![setup.parent, Ptr::<()>::new(setup.path)],
        );
        assert_eq!(tile.u32(), 0);
        assert_eq!(e.global::<u8>(XML_ERROR_FLAG), 1);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert!(calls_to(&e, TILE_UPDATE_CHILDREN).is_empty());
        assert!(calls_to(&e, BUILD_STORAGE_DESTROY).is_empty());
    }

    #[test]
    fn reading_stops_when_a_template_cannot_be_registered() {
        let mut e = tile_engine();
        let setup = read_setup(&mut e, "");
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        e.register(MENU_ADD_TEMPLATE, |e, _| {
            e.set_global(XML_ERROR_FLAG, 1u8);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let tile = e.call(
            0x00a0_1b00,
            &args![setup.parent, Ptr::<()>::new(setup.path)],
        );
        assert_eq!(tile.u32(), 0);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert!(calls_to(&e, TILE_UPDATE_CHILDREN).is_empty());
        // The storage is left as it was.
        assert!(calls_to(&e, BUILD_STORAGE_DESTROY).is_empty());
    }

    #[test]
    fn float_to_int_rounded_rounds_to_even_and_flags_overflow() {
        assert_eq!(float_to_int_rounded(2.5), 2);
        assert_eq!(float_to_int_rounded(3.5), 4);
        assert_eq!(float_to_int_rounded(-2.5), -2);
        assert_eq!(float_to_int_rounded(-3.5), -4);
        assert_eq!(float_to_int_rounded(2.6), 3);
        assert_eq!(float_to_int_rounded(-0.4), 0);
        assert_eq!(float_to_int_rounded(f32::NAN), i32::MIN);
        assert_eq!(float_to_int_rounded(3.0e9), i32::MIN);
        assert_eq!(float_to_int_rounded(-3.0e9), i32::MIN);
    }

    // -----------------------------------------------------------------------
    // Session 2: `GetChildByID` .. `Value::CalculateValue`

    /// The pages and values of the `.rdata` constants this batch reads.
    fn provide_constants(e: &mut Engine) {
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0101_a000,
            0x0101_d000,
            0x0101_e000,
            0x0102_0000,
            0x0102_1000,
            0x0103_3000,
            0x0106_e000,
            0x0106_f000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE_DOUBLE, 3.0f64);
        e.set_global(THOUSAND, 1000.0f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(HUNDRED_FLOAT, 100.0f32);
        e.set_global(TWO_FIFTY_FIVE_DOUBLE, 255.0f64);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ID_VALUE_110, 110.0f64);
        e.set_global(ID_VALUE_111, 111.0f64);
        e.set_global(STACKING_TYPE_102, 102.0f64);
        e.set_global(STACKING_TYPE_103, 103.0f64);
        e.set_global(STACKING_VALUE_6000, 6000.0f32);
        e.set_global(STACKING_VALUE_6001, 6001.0f32);
        e.set_global(DEPTH_SCALE, -0.008f64);
        e.set_global(FADE_RISE_END_DOUBLE, 0.16666f64);
        e.set_global(FADE_FALL_START_DOUBLE, 0.83334f64);
        e.set_global(FADE_RISE_END_FLOAT, 0.16666f32);
        e.set_global(FADE_FALL_START_FLOAT, 0.83334f32);
        e.set_global(HIGHLIGHT_RED, 0.545f32);
        e.set_global(HIGHLIGHT_ALPHA, 0.2f32);
        e.set_global(STD_STRING_NPOS, u32::MAX);
        e.mem.set_cstr(HIGHLIGHT_BOX_NAME, b"lb_highlight_box");
        e.mem.set_cstr(FORMAT_INTEGER, b"%d");
    }

    /// A `strcmp` over game memory.
    fn install_strcmp(e: &mut Engine) {
        e.register(STRCMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            (first.cmp(&second) as i32).into_ret()
        });
    }

    /// A clock for `GetTickCount`: the cell is the time it answers.
    fn install_clock(e: &mut Engine) -> Rc<Cell<u32>> {
        let now = Rc::new(Cell::new(0));
        let seen = now.clone();
        e.register_double(TICK_COUNT_IMPORT, move |_, _| seen.get().into_ret());
        now
    }

    /// Makes the function at `addr` answer the `float` `value`.
    fn answer_float(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// An `NiNode`-like block: children array at +0xA0 with counts at +0xA6
    /// and +0xA8, parent at +0x18, flags at +0x30.
    fn make_model(e: &mut Engine, children: &[u32]) -> u32 {
        let node = e.mem.alloc(0xc0);
        let buffer = e.mem.alloc(4 * children.len().max(1) as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *child);
        }
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        e.mem.set_u16(node + 0xa8, children.len() as u16);
        node
    }

    /// A vtable for a `Tile::Action`: `QFloat` (slot 0) answers `amount`,
    /// `QRefValue` (slot 4) answers `referenced`.
    fn action_vtable(e: &mut Engine, amount: f32, referenced: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, 0x7000_0000 + vtable);
        e.mem.set_u32(vtable + 4, 0x7000_0004 + vtable);
        e.register_double(0x7000_0000 + vtable, move |_, _| amount.into_ret());
        e.register_double(0x7000_0004 + vtable, move |_, _| referenced.into_ret());
        vtable
    }

    /// An action object of the given `VALUE_ACTION` type.
    fn make_action(e: &mut Engine, vtable: u32, kind: i32, next: u32) -> u32 {
        let action = e.mem.alloc(0x10);
        e.mem.set_u32(action, vtable);
        e.mem.set_i32(action + 4, kind);
        e.mem.set_u32(action + 8, next);
        action
    }

    /// A tile with scalar-deleting-destructor calls recorded as
    /// (tile, flag).
    fn deletable_tile(e: &mut Engine, log: &Rc<RefCell<Vec<(u32, u32)>>>) -> Ptr<Tile> {
        let vtable = e.mem.alloc(0x10);
        let function = 0x7100_0000 + vtable;
        e.mem.set_u32(vtable, function);
        let seen = log.clone();
        e.register_double(function, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let tile: Ptr<Tile> = e.new_object();
        e.mem.set_u32(tile.addr(), vtable);
        tile
    }

    /// One `SetAlphaAndColor` call: (model, alpha, color).
    type ColorCall = (u32, f32, [f32; 4]);

    /// What the virtual calls of an [`updating_tile`] saw.
    struct TileLog {
        /// What `MakeNode` (slot +8) answers.
        make_node: Rc<Cell<u32>>,
        /// How often `MakeNode` was called.
        made: Rc<Cell<u32>>,
        /// `SetAlphaAndColor(model, alpha, color)` (slot +0x24).
        colors: Rc<RefCell<Vec<ColorCall>>>,
        /// `PostParse(trait, value, text)` (slot +0x18) as it was called.
        post_parses: Rc<RefCell<Vec<(i32, f32, u32)>>>,
        /// What `PostParse` answers.
        post_parse_answer: Rc<Cell<u32>>,
    }

    /// A tile of the given type with `MakeNode` and `SetAlphaAndColor`.
    fn updating_tile(e: &mut Engine, kind: u32) -> (Ptr<Tile>, TileLog) {
        let vtable = e.mem.alloc(0x30);
        let base = 0x7200_0000 + vtable;
        for slot in [8u32, 0xc, 0x18, 0x24] {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        let log = TileLog {
            make_node: Rc::new(Cell::new(1)),
            made: Rc::new(Cell::new(0)),
            colors: Rc::new(RefCell::new(vec![])),
            post_parses: Rc::new(RefCell::new(vec![])),
            post_parse_answer: Rc::new(Cell::new(1)),
        };
        let (answer, made) = (log.make_node.clone(), log.made.clone());
        e.register_double(base + 8, move |_, _| {
            made.set(made.get() + 1);
            answer.get().into_ret()
        });
        e.register_double(base + 0xc, move |_, _| kind.into_ret());
        let (post_parses, answer) = (log.post_parses.clone(), log.post_parse_answer.clone());
        e.register_double(base + 0x18, move |_, a| {
            post_parses
                .borrow_mut()
                .push((a[1] as i32, f32::from_bits(a[2]), a[3]));
            answer.get().into_ret()
        });
        let colors = log.colors.clone();
        e.register_double(base + 0x24, move |e, a| {
            let rgba = [
                e.mem.f32(a[3]),
                e.mem.f32(a[3] + 4),
                e.mem.f32(a[3] + 8),
                e.mem.f32(a[3] + 12),
            ];
            colors.borrow_mut().push((a[1], f32::from_bits(a[2]), rgba));
            Ret::default()
        });
        let tile: Ptr<Tile> = e.new_object();
        e.mem.set_u32(tile.addr(), vtable);
        (tile, log)
    }

    #[test]
    fn all_functions_of_the_second_batch_are_registered() {
        let e = Engine::new();
        for address in [
            0x00a0_3eb0,
            0x00a0_3f70,
            0x00a0_40a0,
            0x00a0_4100,
            0x00a0_4150,
            0x00a0_4200,
            0x00a0_44f0,
            0x00a0_4500,
            0x00a0_4510,
            0x00a0_4620,
            0x00a0_4640,
            0x00a0_6dd0,
            0x00a0_74d0,
            0x00a0_7530,
            0x00a0_7580,
            0x00a0_7690,
            0x00a0_77c0,
            0x00a0_78e0,
            0x00a0_79d0,
            0x00a0_7a40,
            0x00a0_7c60,
            0x00a0_7dc0,
            0x00a0_7ed0,
            0x00a0_7f30,
            0x00a0_7fc0,
            0x00a0_8070,
            0x00a0_80d0,
            0x00a0_8720,
            0x00a0_87d0,
            0x00a0_89c0,
            0x00a0_8a20,
            0x00a0_8b20,
            0x00a0_8f20,
            0x00a0_8fb0,
            0x00a0_9030,
            0x00a0_9080,
            0x00a0_9130,
            0x00a0_9200,
            0x00a0_9330,
            0x00a0_9410,
        ] {
            assert!(e.is_translated(address), "{address:08x}");
        }
    }

    #[test]
    fn child_by_id_finds_the_first_child_with_that_id() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let parent = typed_tile(&mut e, TYPE_RECT);
        let mut children = vec![];
        for index in 0..3 {
            let child = typed_tile(&mut e, TYPE_RECT);
            give_trait(&mut e, child, TRAIT_ID, 10.0 + index as f32);
            add_child(&mut e, parent, child);
            children.push(child);
        }
        e.call_log = Some(vec![]);
        assert_eq!(tile_get_child_by_id(&mut e, parent, 11), children[1]);
        assert!(tile_get_child_by_id(&mut e, parent, 99).is_null());
        assert_lock_balanced(&e);
        assert_eq!(calls_to(&e, TILE_LOCK).len(), 2);
    }

    #[test]
    fn first_ref_copy_follows_the_first_reference_action() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        let referenced: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(referenced, TileValue::eIndex, TRAIT_Y);
        e.set(referenced, TileValue::pParent, other);
        let plain = action_vtable(&mut e, 1.0, 0);
        let reference = action_vtable(&mut e, 0.0, referenced.addr());
        let second = make_action(&mut e, reference, VA_REF, 0);
        let first = make_action(&mut e, plain, VA_COPY, second);
        let value = give_trait(&mut e, tile, TRAIT_X, 0.0);
        e.set(value, TileValue::pActionListA, Ptr::new(first));
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let answer = tile_get_first_ref_copy(&mut e, tile, TRAIT_X, Ptr::new(out));
        assert_eq!(answer, other.addr());
        assert_eq!(e.mem.i32(out), TRAIT_Y);
        assert_lock_balanced(&e);
        // A trait without a reference action answers 0 and leaves `out`.
        let only_plain = make_action(&mut e, plain, VA_COPY, 0);
        let value = give_trait(&mut e, tile, TRAIT_WIDTH, 0.0);
        e.set(value, TileValue::pActionListA, Ptr::new(only_plain));
        e.mem.set_i32(out, 77);
        assert_eq!(
            tile_get_first_ref_copy(&mut e, tile, TRAIT_WIDTH, Ptr::new(out)),
            0
        );
        assert_eq!(e.mem.i32(out), 77);
    }

    #[test]
    fn first_ref_copy_asks_the_parent_when_the_tile_lacks_the_trait() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let parent = typed_tile(&mut e, TYPE_RECT);
        let child = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, parent, child);
        let other = typed_tile(&mut e, TYPE_RECT);
        let referenced: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(referenced, TileValue::eIndex, TRAIT_HEIGHT);
        e.set(referenced, TileValue::pParent, other);
        let reference = action_vtable(&mut e, 0.0, referenced.addr());
        let action = make_action(&mut e, reference, VA_REF, 0);
        let value = give_trait(&mut e, parent, TRAIT_X, 0.0);
        e.set(value, TileValue::pActionListA, Ptr::new(action));
        let out = e.mem.alloc(4);
        assert_eq!(
            tile_get_first_ref_copy(&mut e, child, TRAIT_X, Ptr::new(out)),
            other.addr()
        );
        assert_eq!(e.mem.i32(out), TRAIT_HEIGHT);
        // Neither the tile nor a parent has it.
        assert_eq!(
            tile_get_first_ref_copy(&mut e, child, TRAIT_Y, Ptr::new(out)),
            0
        );
    }

    #[test]
    fn a_tile_is_visible_unless_its_model_or_an_ancestor_node_is_culled() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        assert!(tile_is_visible(&mut e, tile));
        let parent_node = make_model(&mut e, &[]);
        let model = make_model(&mut e, &[]);
        e.mem.set_u32(model + 0x18, parent_node);
        e.set(tile, Tile::spModel, Ptr::new(model));
        assert!(tile_is_visible(&mut e, tile));
        e.mem.set_u32(parent_node + 0x30, 1);
        assert!(!tile_is_visible(&mut e, tile));
        e.mem.set_u32(parent_node + 0x30, 0);
        e.mem.set_u32(model + 0x30, 3);
        assert!(!tile_is_visible(&mut e, tile));
        e.mem.set_u32(model + 0x30, 2);
        assert!(tile_is_visible(&mut e, tile));
    }

    #[test]
    fn walking_up_through_tiles_with_id_110_ends_at_the_root() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let [root, middle, leaf] = chain(&mut e, [[0.0; 4]; 3]);
        give_trait(&mut e, root, TRAIT_ID, 110.0);
        give_trait(&mut e, middle, TRAIT_ID, 110.0);
        assert!(!fn_00a04100(&mut e, leaf));
        assert!(!fn_00a04100(&mut e, root));
    }

    #[test]
    #[should_panic(expected = "loops forever")]
    fn walking_up_stops_with_a_panic_where_the_game_would_hang() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let [root, _, leaf] = chain(&mut e, [[0.0; 4]; 3]);
        give_trait(&mut e, root, TRAIT_ID, 5.0);
        fn_00a04100(&mut e, leaf);
    }

    #[test]
    fn delete_children_deletes_each_child_and_empties_the_list() {
        let mut e = tile_engine();
        let parent = typed_tile(&mut e, TYPE_RECT);
        let log = Rc::new(RefCell::new(vec![]));
        let first = deletable_tile(&mut e, &log);
        let second = deletable_tile(&mut e, &log);
        add_child(&mut e, parent, first);
        push_list(&mut e, parent.at(Tile::xChildren), 0);
        add_child(&mut e, parent, second);
        e.call_log = Some(vec![]);
        tile_delete_children(&mut e, parent);
        assert_eq!(*log.borrow(), vec![(first.addr(), 1), (second.addr(), 1)]);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![parent.addr() + 4]]);
        assert_lock_balanced(&e);
    }

    #[test]
    fn lock_and_unlock_enter_and_leave_the_tile_critical_section() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        tile_lock(&mut e);
        tile_unlock(&mut e);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![TILE_CRITICAL_SECTION]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![TILE_CRITICAL_SECTION]]
        );
    }

    /// The dirty-tile list doubles: `LIST_REMOVE_HEAD` hands out `tiles` in
    /// order and keeps the count at `DIRTY_TILES_LIST + 8` right.
    fn dirty_list_with(e: &mut Engine, tiles: Vec<Ptr<Tile>>) {
        e.mem.set_u32(DIRTY_TILES_LIST + 8, tiles.len() as u32);
        let mut queue = std::collections::VecDeque::from(tiles);
        e.register_double(LIST_REMOVE_HEAD, move |e, _| {
            let count = e.mem.u32(DIRTY_TILES_LIST + 8);
            e.mem.set_u32(DIRTY_TILES_LIST + 8, count - 1);
            queue.pop_front().map_or(0, |tile| tile.addr()).into_ret()
        });
    }

    #[test]
    fn update_all_updates_dirty_tiles_and_announces_the_changed_ones() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, UPDATE_CREATE | FLAG_DIRTY);
        dirty_list_with(&mut e, vec![tile]);
        e.set_global(INTERFACE_MANAGER, 0x5000_1000u32);
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        let during = Rc::new(Cell::new(0));
        let seen = during.clone();
        e.register_double(INTERFACE_MANAGER_END_OF_UPDATE, move |e, _| {
            let line_word = e.tls() + TLS_CURRENT_LINE;
            seen.set(e.mem.u32(line_word));
            Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x00a0_4200, &args![true]);

        assert_eq!(log.made.get(), 1);
        assert_eq!(e.get(tile, Tile::uiFlags), 0);
        assert!(e.get(tile, Tile::bNeedsNiUpdate));
        assert_eq!(
            calls_to(&e, ADD_TILE_TO_UPDATE_LIST),
            vec![vec![0x5000_1000, tile.addr()]]
        );
        assert_eq!(calls_to(&e, INTERFACE_MANAGER_END_OF_UPDATE).len(), 1);
        assert_eq!(during.get(), 13);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            calls_to(&e, LEAVE_CRITICAL_SECTION)
        );
        assert_lock_balanced(&e);
        // Without the flag the manager is left alone.
        e.call_log = Some(vec![]);
        e.call(0x00a0_4200, &args![false]);
        assert!(calls_to(&e, INTERFACE_MANAGER_END_OF_UPDATE).is_empty());
    }

    #[test]
    fn update_all_skips_released_deleting_hibernated_and_idle_tiles() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        let mut tiles = vec![];
        let mut logs = vec![];
        for flags in [
            UPDATE_CREATE | FLAG_RELEASED,
            UPDATE_CREATE | FLAG_MENU_DELETING,
            UPDATE_CREATE | FLAG_HIBERNATED,
            FLAG_DIRTY,
        ] {
            let (tile, log) = updating_tile(&mut e, TYPE_RECT);
            e.set(tile, Tile::uiFlags, flags);
            tiles.push(tile);
            logs.push(log);
        }
        dirty_list_with(&mut e, tiles);
        e.call(0x00a0_4200, &args![false]);
        for log in &logs {
            assert_eq!(log.made.get(), 0);
        }
    }

    #[test]
    fn update_all_gives_the_menus_root_children_their_stacking_values() {
        for (stacking, expected) in [(102.0f32, 6000.0f32), (103.0, 6001.0), (104.0, 104.0)] {
            let mut e = tile_engine();
            provide_constants(&mut e);
            install_value_array_doubles(&mut e);
            let (tile, _) = updating_tile(&mut e, TYPE_RECT);
            let root = typed_tile(&mut e, TYPE_RECT);
            add_child(&mut e, root, tile);
            e.register_double(INTERFACE_GET_MENUS_ROOT, move |_, _| root.addr().into_ret());
            let value = give_trait(&mut e, tile, TRAIT_STACKING_TYPE, stacking);
            e.set(tile, Tile::uiFlags, UPDATE_CREATE | FLAG_DIRTY);
            dirty_list_with(&mut e, vec![tile]);
            let set = Rc::new(RefCell::new(vec![]));
            let seen = set.clone();
            e.register_double(VALUE_SET_FLOAT, move |_, a| {
                seen.borrow_mut().push((a[0], f32::from_bits(a[1]), a[2]));
                Ret::default()
            });
            e.call(0x00a0_4200, &args![false]);
            let expected_sets = if expected == stacking {
                vec![]
            } else {
                vec![(value.addr(), expected, 1)]
            };
            assert_eq!(*set.borrow(), expected_sets);
        }
    }

    #[test]
    fn update_all_checks_the_hibernating_tiles_and_runs_again_after_a_tile_became_visible() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        // A culled model that the visibility trait shows again.
        let model = make_model(&mut e, &[]);
        e.mem.set_u32(model + 0x30, 1);
        e.set(tile, Tile::spModel, Ptr::new(model));
        give_trait(&mut e, tile, TRAIT_VISIBLE, 1.0);
        e.set(tile, Tile::uiFlags, UPDATE_VISIBILITY | FLAG_DIRTY);
        dirty_list_with(&mut e, vec![tile]);
        e.set_global(INTERFACE_MANAGER, 0x5000_1000u32);
        e.call_log = Some(vec![]);
        e.call(0x00a0_4200, &args![true]);
        assert_eq!(e.mem.u32(model + 0x30), 0);
        // The inner pass found nothing to do and finished the work once.
        assert_eq!(calls_to(&e, INTERFACE_MANAGER_END_OF_UPDATE).len(), 1);
        assert_eq!(e.global::<u8>(NEEDS_CHECK_HIBERNATE), 0);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn update_all_purges_textures_when_the_interface_asks() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        e.register(INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE, |_, _| {
            1u32.into_ret()
        });
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x55);
        let during = Rc::new(Cell::new(0));
        let seen = during.clone();
        e.register_double(TEXTURE_PALETTE_PURGE_UNUSED, move |e, _| {
            let line_word = e.tls() + TLS_CURRENT_LINE;
            seen.set(e.mem.u32(line_word));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        tile_update_all(&mut e, false);
        assert_eq!(
            calls_to(&e, INTERFACE_PREPARE_TEXTURE_RELEASE),
            vec![vec![0]]
        );
        assert_eq!(calls_to(&e, TEXTURE_PALETTE_PURGE_UNUSED).len(), 1);
        assert_eq!(during.get(), 13);
        assert_eq!(e.mem.u32(line_word), 0x55);
    }

    /// A hibernating list of the given tiles: nodes (next, previous,
    /// element) and a `LIST_REMOVE_NODE` that steps the iterator on.
    fn hibernating_list_with(e: &mut Engine, tiles: &[Ptr<Tile>]) {
        let nodes: Vec<u32> = tiles.iter().map(|_| e.mem.alloc(12)).collect();
        for (index, tile) in tiles.iter().enumerate() {
            e.mem.set_u32(nodes[index] + 8, tile.addr());
            if let Some(next) = nodes.get(index + 1) {
                e.mem.set_u32(nodes[index], *next);
            }
        }
        e.mem
            .set_u32(HIBERNATING_TILES_LIST, nodes.first().copied().unwrap_or(0));
        e.mem
            .set_u32(HIBERNATING_TILES_LIST + 8, tiles.len() as u32);
        e.register_double(LIST_REMOVE_NODE, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            e.mem.u32(node + 8).into_ret()
        });
    }

    #[test]
    fn hibernating_tiles_wake_while_the_frame_has_budget() {
        let mut e = tile_engine();
        let tiles: Vec<Ptr<Tile>> = (0..4)
            .map(|_| {
                let tile = typed_tile(&mut e, TYPE_RECT);
                e.set(tile, Tile::uiFlags, FLAG_HIBERNATED);
                tile
            })
            .collect();
        hibernating_list_with(&mut e, &tiles);
        // 9 tiles updated and 0 dirty: a budget of 11 - 9 = 2.
        e.set_global(TILES_UPDATED_THIS_FRAME, 9u32);
        let appended = Rc::new(RefCell::new(vec![]));
        let seen = appended.clone();
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        fn_00a04510(&mut e);
        for (index, tile) in tiles.iter().enumerate() {
            let flags = e.get(*tile, Tile::uiFlags);
            if index < 2 {
                assert_eq!(flags, FLAG_PROMOTED | FLAG_DIRTY);
            } else {
                assert_eq!(flags, FLAG_HIBERNATED);
            }
        }
        assert_eq!(*appended.borrow(), vec![tiles[0].addr(), tiles[1].addr()]);
    }

    #[test]
    fn hibernating_tiles_stay_asleep_when_the_frame_is_busy() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, FLAG_HIBERNATED);
        hibernating_list_with(&mut e, &[tile]);
        // 8 updated and 3 dirty: 11 is over the limit of 10.
        e.set_global(TILES_UPDATED_THIS_FRAME, 8u32);
        e.mem.set_u32(DIRTY_TILES_LIST + 8, 3);
        fn_00a04510(&mut e);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_HIBERNATED);
    }

    #[test]
    fn update_children_runs_update_all_without_the_flag() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        tile_update_children(&mut e, tile, 0);
        assert!(calls_to(&e, INTERFACE_MANAGER_END_OF_UPDATE).is_empty());
        assert_eq!(calls_to(&e, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    /// Sets the string of an existing or new trait.
    fn give_string(e: &mut Engine, tile: Ptr<Tile>, id: i32, text: &str) -> Ptr<TileValue> {
        let value = give_trait(e, tile, id, 0.0);
        let string = cstring(e, text);
        e.set(value, TileValue::strValue, Ptr::new(string));
        value
    }

    /// Doubles that record `Tile::Value::SetFloat(value, float, flag)` as
    /// (trait id, float, flag).
    fn record_float_sets(e: &mut Engine) -> Rc<RefCell<Vec<(i32, f32, u32)>>> {
        let sets = Rc::new(RefCell::new(vec![]));
        let seen = sets.clone();
        e.register_double(VALUE_SET_FLOAT, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.i32(a[0]), f32::from_bits(a[1]), a[2]));
            Ret::default()
        });
        sets
    }

    /// `NiMatrix3 * NiPoint3`, `+` and `-` over game memory.
    fn install_point_math(e: &mut Engine) {
        fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
            (0..count).map(|index| e.mem.f32(at + 4 * index)).collect()
        }
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let (m, v) = (read(e, a[0], 9), read(e, a[2], 3));
            for row in 0..3 {
                let value = m[row * 3] * v[0] + m[row * 3 + 1] * v[1] + m[row * 3 + 2] * v[2];
                e.mem.set_f32(a[1] + 4 * row as u32, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (x, y) = (read(e, a[0], 3), read(e, a[2], 3));
            for axis in 0..3 {
                e.mem.set_f32(a[1] + 4 * axis as u32, x[axis] + y[axis]);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            let (x, y) = (read(e, a[0], 3), read(e, a[2], 3));
            for axis in 0..3 {
                e.mem.set_f32(a[1] + 4 * axis as u32, x[axis] - y[axis]);
            }
            a[1].into_ret()
        });
    }

    /// A new hibernating-list node for `fn_00a077c0`.
    fn install_node_allocation(e: &mut Engine) {
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(12).into_ret());
    }

    fn translation_of(e: &Engine, object: u32) -> [f32; 3] {
        [
            e.mem.f32(object + 0x58),
            e.mem.f32(object + 0x5c),
            e.mem.f32(object + 0x60),
        ]
    }

    #[test]
    fn update_tile_without_update_bits_only_counts_the_tile() {
        let mut e = tile_engine();
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, FLAG_DIRTY);
        e.set_global(TILES_UPDATED_THIS_FRAME, 5u32);
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a0_4640, &args![tile, true]).bool());
        assert_eq!(e.global::<u32>(TILES_UPDATED_THIS_FRAME), 6);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert_eq!(log.made.get(), 0);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            calls_to(&e, LEAVE_CRITICAL_SECTION)
        );
    }

    #[test]
    fn update_tile_creates_the_model_and_clears_the_bit() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_node_allocation(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_TEXT);
        e.set(tile, Tile::uiFlags, UPDATE_CREATE | UPDATE_NIF_FILE);
        e.set_global(TILES_UPDATED_THIS_FRAME, 5u32);
        e.call_log = Some(vec![]);
        // `MakeNode` answers a node: created; the other bit keeps the tile
        // updating, so it is put to sleep instead of being counted.
        assert!(tile_update_tile(&mut e, tile, true));
        assert_eq!(log.made.get(), 1);
        assert_eq!(calls_to(&e, TEXT_TILE_CREATED), vec![vec![1]]);
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, UPDATE_NIF_FILE);
        assert_eq!(
            e.get(tile, Tile::uiFlags) & FLAG_HIBERNATED,
            FLAG_HIBERNATED
        );
        assert_eq!(e.global::<u32>(HIBERNATING_TILE_COUNT), 1);
        assert_eq!(e.global::<u32>(TILES_UPDATED_THIS_FRAME), 5);
    }

    #[test]
    fn update_tile_keeps_the_bits_without_the_flag_and_when_no_node_is_made() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_node_allocation(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, UPDATE_CREATE);
        assert!(tile_update_tile(&mut e, tile, false));
        assert_eq!(e.get(tile, Tile::uiFlags), UPDATE_CREATE);
        assert_eq!(e.global::<u32>(TILES_UPDATED_THIS_FRAME), 1);
        log.make_node.set(0);
        assert!(!tile_update_tile(&mut e, tile, true));
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_CREATE, UPDATE_CREATE);
        assert_eq!(log.made.get(), 2);
    }

    #[test]
    fn update_tile_follows_the_visibility_trait() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        let model = make_model(&mut e, &[]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        // Hidden tile, shown model: culled.
        e.set(tile, Tile::uiFlags, UPDATE_VISIBILITY);
        assert!(tile_update_tile(&mut e, tile, true));
        assert_eq!(e.mem.u32(model + 0x30), 1);
        assert_eq!(e.global::<u8>(NEEDS_CHECK_HIBERNATE), 0);
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, 0);
        // Visible trait: the culled model is shown again and the
        // hibernating tiles must be checked.
        give_trait(&mut e, tile, TRAIT_VISIBLE, 1.0);
        e.set(tile, Tile::uiFlags, UPDATE_VISIBILITY);
        assert!(tile_update_tile(&mut e, tile, true));
        assert_eq!(e.mem.u32(model + 0x30), 0);
        assert_eq!(e.global::<u8>(NEEDS_CHECK_HIBERNATE), 1);
        // Without a model nothing is done and the bit stays.
        let (bare, _) = updating_tile(&mut e, TYPE_RECT);
        e.set(bare, Tile::uiFlags, UPDATE_VISIBILITY);
        assert!(!tile_update_tile(&mut e, bare, false));
        assert_eq!(e.get(bare, Tile::uiFlags), UPDATE_VISIBILITY);
    }

    /// A tile of the given type with an alpha of 0.5, red 255, green 51 and
    /// blue 0 and the color bit set.
    fn colored_tile(e: &mut Engine) -> (Ptr<Tile>, TileLog) {
        let (tile, log) = updating_tile(e, TYPE_RECT);
        give_trait(e, tile, TRAIT_ALPHA, 127.5);
        give_trait(e, tile, TRAIT_RED, 255.0);
        give_trait(e, tile, TRAIT_GREEN, 51.0);
        give_trait(e, tile, TRAIT_BLUE, 0.0);
        e.set(tile, Tile::uiFlags, UPDATE_COLOR);
        (tile, log)
    }

    #[test]
    fn update_tile_colors_from_the_traits_when_there_is_no_system_color() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_strcmp(&mut e);
        let (tile, log) = colored_tile(&mut e);
        assert!(!tile_update_tile(&mut e, tile, true));
        let green = (51.0f64 / 255.0) as f32;
        assert_eq!(*log.colors.borrow(), vec![(0, 0.5, [1.0, green, 0.0, 1.0])]);
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, 0);
    }

    #[test]
    fn update_tile_scales_a_system_color_by_the_brightness() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_strcmp(&mut e);
        let (tile, log) = colored_tile(&mut e);
        give_trait(&mut e, tile, TRAIT_SYSTEM_COLOR, 3.0);
        give_trait(&mut e, tile, TRAIT_BRIGHTNESS, 127.5);
        e.register(SYSTEM_COLOR_MANAGER_GET_INSTANCE, |_, _| {
            0x5000_2000u32.into_ret()
        });
        let asked = Rc::new(RefCell::new(vec![]));
        let seen = asked.clone();
        e.register_double(SYSTEM_COLOR_MANAGER_GET_COLOR, move |e, a| {
            seen.borrow_mut().push((a[0], a[1]));
            for (slot, value) in [1.0f32, 0.5, 0.25, 1.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * slot as u32, *value);
            }
            1u32.into_ret()
        });
        e.register(COLOR_SCALE, |e, a| {
            let factor = f32::from_bits(a[1]);
            for slot in 0..4 {
                let value = e.mem.f32(a[0] + 4 * slot);
                e.mem.set_f32(a[0] + 4 * slot, value * factor);
            }
            a[0].into_ret()
        });
        tile_update_tile(&mut e, tile, true);
        assert_eq!(*asked.borrow(), vec![(0x5000_2000, 3)]);
        assert_eq!(
            *log.colors.borrow(),
            vec![(0, 0.5, [0.5, 0.25, 0.125, 0.5])]
        );
    }

    #[test]
    fn update_tile_finds_the_system_color_of_a_menu_tile_by_its_ancestors() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_strcmp(&mut e);
        // root > menu (id 110, system color 7) > leaf (id 110): the leaf is
        // a grandchild of the root with id 110 and no system color of its
        // own, so the menu's counts.
        let root = typed_tile(&mut e, TYPE_RECT);
        let menu = typed_tile(&mut e, TYPE_RECT);
        let (leaf, _) = colored_tile(&mut e);
        add_child(&mut e, root, menu);
        add_child(&mut e, menu, leaf);
        give_trait(&mut e, leaf, TRAIT_ID, 110.0);
        give_trait(&mut e, menu, TRAIT_SYSTEM_COLOR, 7.0);
        give_trait(&mut e, leaf, TRAIT_BRIGHTNESS, 255.0);
        let asked = Rc::new(RefCell::new(vec![]));
        let seen = asked.clone();
        e.register_double(SYSTEM_COLOR_MANAGER_GET_COLOR, move |_, a| {
            seen.borrow_mut().push(a[1]);
            0u32.into_ret()
        });
        tile_update_tile(&mut e, leaf, true);
        assert_eq!(*asked.borrow(), vec![7]);
    }

    #[test]
    fn update_tile_turns_a_failed_speech_challenge_highlight_red() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_strcmp(&mut e);
        let (tile, log) = colored_tile(&mut e);
        e.set(tile, Tile::bSpeechChallengeFailure, true);
        let name = cstring(&mut e, "lb_highlight_box");
        e.mem.set_u32(tile.addr() + 0x20, name);
        tile_update_tile(&mut e, tile, true);
        assert_eq!(*log.colors.borrow(), vec![(0, 0.2, [0.545, 0.0, 0.0, 1.0])]);
        // The child of the box: red and opaque.
        let (child, log) = colored_tile(&mut e);
        e.set(child, Tile::bSpeechChallengeFailure, true);
        let other = cstring(&mut e, "child");
        e.mem.set_u32(child.addr() + 0x20, other);
        add_child(&mut e, tile, child);
        tile_update_tile(&mut e, child, true);
        assert_eq!(*log.colors.borrow(), vec![(0, 1.0, [0.545, 0.0, 0.0, 1.0])]);
        // A failure flag on an unrelated tile changes nothing.
        let (plain, log) = colored_tile(&mut e);
        e.set(plain, Tile::bSpeechChallengeFailure, true);
        e.mem.set_u32(plain.addr() + 0x20, other);
        tile_update_tile(&mut e, plain, true);
        let green = (51.0f64 / 255.0) as f32;
        assert_eq!(*log.colors.borrow(), vec![(0, 0.5, [1.0, green, 0.0, 1.0])]);
    }

    #[test]
    fn update_tile_positions_a_menu_level_model_from_the_screen() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        answer_float(&mut e, SCREEN_WIDTH, 1280.0);
        answer_float(&mut e, SCREEN_HEIGHT, 720.0);
        let root = typed_tile(&mut e, TYPE_RECT);
        e.register_double(INTERFACE_GET_MENUS_ROOT, move |_, _| root.addr().into_ret());
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        add_child(&mut e, root, tile);
        give_trait(&mut e, tile, TRAIT_X, 100.0);
        give_trait(&mut e, tile, TRAIT_Y, 50.0);
        give_trait(&mut e, tile, TRAIT_DEPTH, 10.0);
        // A locus shape child gets the default translation back.
        e.set_global(DEFAULT_TRANSLATION, 1.0f32);
        e.set_global(DEFAULT_TRANSLATION + 4, 2.0f32);
        e.set_global(DEFAULT_TRANSLATION + 8, 3.0f32);
        let shape = e.mem.alloc(0xc0);
        let shape_vtable = e.mem.alloc(0x30);
        e.mem.set_u32(shape_vtable + 0x24, 0x7300_0000);
        e.register_double(0x7300_0000, |_, a| a[0].into_ret());
        e.mem.set_u32(shape, shape_vtable);
        let model = make_model(&mut e, &[shape]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.set(tile, Tile::uiFlags, UPDATE_POSITION | UPDATE_LOCUS);
        assert!(tile_update_tile(&mut e, tile, true));
        let depth = (10.0f32 as f64 * -0.008f64) as f32;
        assert_eq!(translation_of(&e, model), [-540.0, depth, 310.0]);
        assert_eq!(translation_of(&e, shape), [1.0, 2.0, 3.0]);
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, 0);
        // With the manual-update flag the shape is left alone.
        e.mem.set_f32(shape + 0x58, 9.0);
        e.set(
            tile,
            Tile::uiFlags,
            UPDATE_POSITION | UPDATE_LOCUS | FLAG_MANUAL_UPDATE_TRIS,
        );
        tile_update_tile(&mut e, tile, true);
        assert_eq!(e.mem.f32(shape + 0x58), 9.0);
    }

    #[test]
    fn update_tile_positions_a_nested_model_and_rotates_its_shapes_about_the_axis() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_point_math(&mut e);
        let parent = typed_tile(&mut e, TYPE_RECT);
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        add_child(&mut e, parent, tile);
        give_trait(&mut e, tile, TRAIT_X, 5.0);
        give_trait(&mut e, tile, TRAIT_Y, 7.0);
        give_trait(&mut e, tile, TRAIT_DEPTH, 2.0);
        give_trait(&mut e, tile, TRAIT_ROTATE_AXIS_X, 10.0);
        give_trait(&mut e, tile, TRAIT_ROTATE_AXIS_Y, 4.0);
        give_trait(
            &mut e,
            tile,
            TRAIT_ROTATE_ANGLE,
            std::f32::consts::FRAC_PI_2,
        );
        e.set_global(DEFAULT_TRANSLATION, 1.0f32);
        e.set_global(DEFAULT_TRANSLATION + 4, 2.0f32);
        e.set_global(DEFAULT_TRANSLATION + 8, 3.0f32);
        // The first child is a shape (its class chain holds the shape
        // record), the second is not.
        let make_object = |e: &mut Engine, rtti: u32| {
            let object = e.mem.alloc(0xc0);
            let vtable = e.mem.alloc(0x10);
            let function = 0x7400_0000 + vtable;
            e.mem.set_u32(vtable + 8, function);
            e.register_double(function, move |_, _| rtti.into_ret());
            e.mem.set_u32(object, vtable);
            object
        };
        let other_rtti = e.mem.alloc(8);
        e.mem.set_u32(other_rtti + 4, SHAPE_RTTI);
        let shape = make_object(&mut e, other_rtti);
        let stranger = make_object(&mut e, 0);
        let model = make_model(&mut e, &[shape, stranger]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.set(tile, Tile::uiFlags, UPDATE_POSITION);
        assert!(tile_update_tile(&mut e, tile, true));
        // The model gets the default translation plus x, depth and -y.
        let depth = (2.0f64 * -0.008 + 2.0) as f32;
        assert_eq!(translation_of(&e, model), [1.0, 2.0, 3.0]);
        let rotation: Vec<f32> = (0..9)
            .map(|slot| e.mem.f32(shape + 0x34 + 4 * slot))
            .collect();
        assert_eq!(
            rotation[1..9].to_vec(),
            vec![0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, rotation[0]]
        );
        assert!(rotation[0].abs() < 1e-6);
        // (translation + axis) - rotation * axis, with the axis (10, 0, -4):
        // (6, depth, -4) + (10, 0, -4) - (4, 0, 10).
        let moved = translation_of(&e, shape);
        assert!((moved[0] - 12.0).abs() < 1e-4, "{moved:?}");
        assert!((moved[1] - depth).abs() < 1e-6, "{moved:?}");
        assert!((moved[2] + 18.0).abs() < 1e-4, "{moved:?}");
        // The other child keeps its translation.
        assert_eq!(translation_of(&e, stranger), [0.0, 0.0, 0.0]);
    }

    /// An image tile whose model holds one geometry with the given data.
    struct ImageParts {
        tile: Ptr<Tile>,
        data: u32,
        vertices: u32,
        coordinates: u32,
        property: u32,
        geometry: u32,
    }

    fn image_tile(e: &mut Engine, flags: u32, data_flags: u16) -> ImageParts {
        let (tile, _) = updating_tile(e, TYPE_IMAGE);
        let vertices = e.mem.alloc(48);
        let coordinates = e.mem.alloc(32);
        let data = e.mem.alloc(0x40);
        e.mem.set_u16(data + 8, 4);
        e.mem.set_u16(data + 0xe, data_flags);
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(data + 0x2c, coordinates);
        // The geometry: `IsTriBasedGeom` (slot +0x1C) answers itself.
        let geometry = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x1c, 0x7500_0000);
        e.register_double(0x7500_0000, |_, a| a[0].into_ret());
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(geometry + 0xb8, data);
        // Its shader property holds a 128 x 64 texture.
        let texture = e.mem.alloc(0x20);
        let texture_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(texture_vtable + 0x94, 0x7500_0094);
        e.mem.set_u32(texture_vtable + 0x98, 0x7500_0098);
        e.register_double(0x7500_0094, |_, _| 128u32.into_ret());
        e.register_double(0x7500_0098, |_, _| 64u32.into_ret());
        e.mem.set_u32(texture, texture_vtable);
        let property = e.mem.alloc(0xb0);
        e.mem.set_u32(property + 0x60, texture);
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            if a[1] == 3 {
                property.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let model = make_model(e, &[geometry]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.set(tile, Tile::uiFlags, flags);
        ImageParts {
            tile,
            data,
            vertices,
            coordinates,
            property,
            geometry,
        }
    }

    #[test]
    fn update_tile_builds_the_geometry_of_an_image_tile() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let image = image_tile(&mut e, UPDATE_GEOMETRY, 0);
        give_trait(&mut e, image.tile, TRAIT_WIDTH, 64.0);
        give_trait(&mut e, image.tile, TRAIT_HEIGHT, 32.0);
        give_trait(&mut e, image.tile, TRAIT_TILE, -1.0);
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, image.tile, true));
        let vertices: Vec<f32> = (0..12)
            .map(|slot| e.mem.f32(image.vertices + 4 * slot))
            .collect();
        assert_eq!(
            vertices,
            vec![0.0, 0.0, 0.0, 0.0, 0.0, -32.0, 64.0, 0.0, 0.0, 64.0, 0.0, -32.0]
        );
        let coordinates: Vec<f32> = (0..8)
            .map(|slot| e.mem.f32(image.coordinates + 4 * slot))
            .collect();
        assert_eq!(coordinates, vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]);
        // The texture is tiled when the trait is not 0.
        assert_eq!(e.mem.u32(image.property + 0x8c), 2);
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_MARK_AS_CHANGED),
            vec![vec![image.data, 9]]
        );
        assert_eq!(
            calls_to(&e, BOUND_COMPUTE_FROM_DATA),
            vec![vec![image.data + 0x10, 4, image.vertices]]
        );
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SET_CONSISTENCY),
            vec![vec![image.data, 0]]
        );
        assert_eq!(e.get(image.tile, Tile::uiFlags) & UPDATE_MASK, 0);
    }

    #[test]
    fn update_tile_leaves_static_geometry_alone_and_zooms_the_coordinates() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let image = image_tile(&mut e, UPDATE_GEOMETRY, 0x4000);
        e.mem.set_f32(image.vertices, 9.0);
        give_trait(&mut e, image.tile, TRAIT_WIDTH, 64.0);
        give_trait(&mut e, image.tile, TRAIT_HEIGHT, 32.0);
        give_trait(&mut e, image.tile, TRAIT_CROP_X, 32.0);
        give_trait(&mut e, image.tile, TRAIT_CROP_Y, 16.0);
        // A zoom of 50% against the file size 128 x 64: the texture is
        // 64 x 32, the crop 0.5 / 0.5 and the span 1 x 1.
        give_trait(&mut e, image.tile, TRAIT_ZOOM, 50.0);
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, image.tile, true));
        assert_eq!(e.mem.f32(image.vertices), 9.0);
        let coordinates: Vec<f32> = (0..8)
            .map(|slot| e.mem.f32(image.coordinates + 4 * slot))
            .collect();
        assert_eq!(coordinates, vec![0.5, 0.5, 0.5, 1.5, 1.5, 0.5, 1.5, 1.5]);
        assert_eq!(e.mem.u32(image.property + 0x8c), 0);
        assert!(calls_to(&e, GEOMETRY_DATA_MARK_AS_CHANGED).is_empty());
        assert!(calls_to(&e, BOUND_COMPUTE_FROM_DATA).is_empty());
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SET_CONSISTENCY),
            vec![vec![image.data, 0]]
        );
        let _ = image.geometry;
    }

    /// A texture atlas entry on the static list `TextureEntryList`.
    fn cached_atlas_entry(
        e: &mut Engine,
        atlas: &str,
        subtexture: &str,
        texture: &str,
        rect: [f32; 4],
    ) -> u32 {
        let entry = e.mem.alloc(0x28);
        let atlas = cstring(e, atlas);
        let subtexture = cstring(e, subtexture);
        let texture = cstring(e, texture);
        e.mem.set_u32(entry, atlas);
        e.mem.set_u32(entry + 8, subtexture);
        e.mem.set_u32(entry + 0x10, texture);
        for (slot, value) in rect.iter().enumerate() {
            e.mem.set_f32(entry + 0x18 + 4 * slot as u32, *value);
        }
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, entry);
        // `TextureEntryList` is a `BSSimpleList`: its head is the first node.
        e.mem.set_u32(TEXTURE_ENTRY_LIST, entry);
        e.mem.set_u32(TEXTURE_ENTRY_LIST + 4, 0);
        entry
    }

    #[test]
    fn update_tile_maps_the_texture_coordinates_into_the_atlas_rectangle() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_strcmp(&mut e);
        e.register(STRRCHR, |_, _| 0u32.into_ret());
        let image = image_tile(&mut e, UPDATE_GEOMETRY, 0);
        give_trait(&mut e, image.tile, TRAIT_WIDTH, 64.0);
        give_trait(&mut e, image.tile, TRAIT_HEIGHT, 32.0);
        give_trait(&mut e, image.tile, TRAIT_TILE, -1.0);
        give_string(&mut e, image.tile, TRAIT_TEX_ATLAS, "atlas.txt");
        give_string(&mut e, image.tile, TRAIT_FILENAME, "icon");
        cached_atlas_entry(
            &mut e,
            "atlas.txt",
            "icon",
            "Interface\\icons.dds",
            [10.0, 74.0, 20.0, 52.0],
        );
        assert!(tile_update_tile(&mut e, image.tile, true));
        let coordinates: Vec<f32> = (0..8)
            .map(|slot| e.mem.f32(image.coordinates + 4 * slot))
            .collect();
        assert_eq!(
            coordinates,
            vec![10.0, 20.0, 10.0, 52.0, 74.0, 20.0, 74.0, 52.0]
        );
    }

    #[test]
    fn update_tile_loads_the_texture_of_an_image_tile() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_node_allocation(&mut e);
        let sets = record_float_sets(&mut e);
        let image = image_tile(&mut e, UPDATE_TEXTURE | FLAG_PROMOTED, 0);
        let tile = image.tile;
        let name = cstring(&mut e, "icon.dds");
        let value = give_trait(&mut e, tile, TRAIT_FILENAME, 0.0);
        e.set(value, TileValue::strValue, Ptr::new(name));
        give_trait(&mut e, tile, TRAIT_TILE, -1.0);
        e.mem.set_f32(tile.addr() + 0x38, 2.0);
        // `AddToTesTextures` hands out a texture with one reference.
        let texture = e.mem.alloc(0x20);
        e.mem.set_u32(texture + 4, 1);
        let texture_vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(texture_vtable + 4, 0x7600_0004);
        e.mem.set_u32(texture_vtable + 0x94, 0x7600_0094);
        e.mem.set_u32(texture_vtable + 0x98, 0x7600_0098);
        e.register_double(0x7600_0094, |_, _| 256u32.into_ret());
        e.register_double(0x7600_0098, |_, _| 128u32.into_ret());
        let destroyed = Rc::new(Cell::new(0));
        let seen = destroyed.clone();
        e.register_double(0x7600_0004, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        e.mem.set_u32(texture, texture_vtable);
        e.register_double(ADD_TO_TES_TEXTURES, move |e, a| {
            e.mem.set_u32(a[2], texture);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, tile, true));
        assert_eq!(
            calls_to(&e, ADD_TO_TES_TEXTURES).len(),
            1,
            "{:?}",
            calls_to(&e, ADD_TO_TES_TEXTURES)
        );
        let call = &calls_to(&e, ADD_TO_TES_TEXTURES)[0];
        assert_eq!(
            (call[0], call[1], call[3], call[4]),
            (name, tile.addr() + 0x38, 0, 0)
        );
        assert_eq!(
            calls_to(&e, SET_TILE_TEXTURE),
            vec![vec![image.property, texture]]
        );
        assert_eq!(e.mem.u32(image.property + 0x8c), 2);
        // The trait is -1: the file size follows the texture, divided by the
        // tile's scale (2.0), and a geometry update is queued (and, in the
        // same pass, handled).
        assert_eq!(
            *sets.borrow(),
            vec![(TRAIT_FILE_WIDTH, 128.0, 1), (TRAIT_FILE_HEIGHT, 64.0, 1)]
        );
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, 0);
        // The reference taken for the call is released: destroyed.
        assert_eq!(destroyed.get(), texture);
    }

    #[test]
    fn update_tile_without_a_texture_clears_the_bit_of_an_invisible_tile() {
        for (visible, cleared) in [(0.0, true), (1.0, false)] {
            let mut e = tile_engine();
            provide_constants(&mut e);
            install_value_array_doubles(&mut e);
            install_node_allocation(&mut e);
            let image = image_tile(&mut e, UPDATE_TEXTURE | FLAG_PROMOTED, 0);
            let value = give_trait(&mut e, image.tile, TRAIT_FILENAME, 0.0);
            let name = cstring(&mut e, "missing.dds");
            e.set(value, TileValue::strValue, Ptr::new(name));
            give_trait(&mut e, image.tile, TRAIT_VISIBLE, visible);
            assert!(!tile_update_tile(&mut e, image.tile, true));
            let texture_bit = e.get(image.tile, Tile::uiFlags) & UPDATE_TEXTURE;
            assert_eq!(texture_bit == 0, cleared);
        }
    }

    #[test]
    fn update_tile_gives_an_image_tile_without_a_file_the_placeholder_texture() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_node_allocation(&mut e);
        e.mem
            .set_cstr(EMPTY_TEXTURE_NAME, b"Interface\\Shared\\empty.dds");
        let image = image_tile(&mut e, UPDATE_TEXTURE | FLAG_PROMOTED, 0);
        give_trait(&mut e, image.tile, TRAIT_FILENAME, 0.0);
        let set_strings = Rc::new(RefCell::new(vec![]));
        let seen = set_strings.clone();
        e.register_double(VALUE_SET_STRING, move |e, a| {
            seen.borrow_mut().push((e.mem.i32(a[0]), a[1], a[2]));
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        tile_update_tile(&mut e, image.tile, true);
        assert_eq!(
            *set_strings.borrow(),
            vec![(TRAIT_FILENAME, EMPTY_TEXTURE_NAME, 1)]
        );
    }

    /// root > menu (a menu tile) > tile, with the screen sizes of a
    /// 1920 x 1080 window on a 1280 x 720 layout.
    fn scissor_scene(e: &mut Engine) -> (Ptr<Tile>, Ptr<Tile>) {
        provide_constants(e);
        install_value_array_doubles(e);
        answer_float(e, SCREEN_REAL_WIDTH, 1920.0);
        answer_float(e, SCREEN_REAL_HEIGHT, 1080.0);
        answer_float(e, SCREEN_WIDTH, 1280.0);
        answer_float(e, SCREEN_HEIGHT, 720.0);
        let root = typed_tile(e, TYPE_RECT);
        let (menu_tile, _) = make_menu_tile(e, 0);
        add_child(e, root, menu_tile);
        let (tile, _) = updating_tile(e, TYPE_RECT);
        add_child(e, menu_tile, tile);
        (menu_tile, tile)
    }

    /// A tile with a model of one object with a shader property (the
    /// property is `property`).
    fn clipped_child(e: &mut Engine, parent: Ptr<Tile>, property: u32) -> Ptr<Tile> {
        let child = typed_tile(e, TYPE_RECT);
        add_child(e, parent, child);
        give_trait(e, child, TRAIT_CLIPS, 1.0);
        let model = make_model(e, &[0x1234_0000 + property]);
        e.set(child, Tile::spModel, Ptr::new(model));
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            if a[1] == 3 {
                property.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        child
    }

    #[test]
    fn update_tile_clips_the_children_to_its_screen_rectangle() {
        let mut e = tile_engine();
        let (_, tile) = scissor_scene(&mut e);
        give_trait(&mut e, tile, TRAIT_X, 10.0);
        give_trait(&mut e, tile, TRAIT_Y, 20.0);
        give_trait(&mut e, tile, TRAIT_WIDTH, 100.0);
        give_trait(&mut e, tile, TRAIT_HEIGHT, 50.0);
        let model = make_model(&mut e, &[]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        let property = e.mem.alloc(0xb0);
        clipped_child(&mut e, tile, property);
        e.set(tile, Tile::uiFlags, UPDATE_SCISSOR_WINDOW);
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, tile, true));
        let rectangle: Vec<i32> = (0..4)
            .map(|slot| e.mem.i32(property + 0x9c + 4 * slot))
            .collect();
        assert_eq!(rectangle, vec![15, 30, 165, 105]);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_REFRESH),
            vec![vec![property, 1]]
        );
        assert_eq!(e.get(tile, Tile::uiFlags) & UPDATE_MASK, 0);
    }

    #[test]
    fn update_tile_clips_its_own_objects_to_the_clip_window_ancestor() {
        let mut e = tile_engine();
        let (menu_tile, tile) = scissor_scene(&mut e);
        let window = typed_tile(&mut e, TYPE_RECT);
        // window > tile instead of menu > tile.
        add_child(&mut e, menu_tile, window);
        give_trait(&mut e, window, TRAIT_CLIP_WINDOW, 1.0);
        give_trait(&mut e, window, TRAIT_X, 10.0);
        give_trait(&mut e, window, TRAIT_Y, 20.0);
        give_trait(&mut e, window, TRAIT_WIDTH, 100.0);
        give_trait(&mut e, window, TRAIT_HEIGHT, 50.0);
        let inner = updating_tile(&mut e, TYPE_RECT).0;
        add_child(&mut e, window, inner);
        let property = e.mem.alloc(0xb0);
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            if a[1] == 3 {
                property.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let model = make_model(&mut e, &[0x1234_0000]);
        e.set(inner, Tile::spModel, Ptr::new(model));
        e.set(inner, Tile::uiFlags, UPDATE_SCISSOR);
        let _ = tile;
        assert!(tile_update_tile(&mut e, inner, true));
        let rectangle: Vec<i32> = (0..4)
            .map(|slot| e.mem.i32(property + 0x9c + 4 * slot))
            .collect();
        assert_eq!(rectangle, vec![15, 30, 165, 105]);
        assert_eq!(e.get(inner, Tile::uiFlags) & UPDATE_MASK, 0);
    }

    #[test]
    fn update_tile_loads_the_nif_of_a_3d_tile_only() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_3D);
        let model = make_model(&mut e, &[]);
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.set(tile, Tile::uiFlags, UPDATE_NIF_FILE);
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, tile, true));
        assert_eq!(calls_to(&e, TILE_3D_UPDATE_NIF), vec![vec![tile.addr()]]);
        let (rect, _) = updating_tile(&mut e, TYPE_RECT);
        e.set(rect, Tile::spModel, Ptr::new(model));
        e.set(rect, Tile::uiFlags, UPDATE_NIF_FILE);
        e.call_log = Some(vec![]);
        assert!(tile_update_tile(&mut e, rect, true));
        assert!(calls_to(&e, TILE_3D_UPDATE_NIF).is_empty());
    }

    #[test]
    fn update_tile_lets_go_of_the_textures_when_the_interface_asks() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_node_allocation(&mut e);
        e.register(INTERFACE_GET_FIRST_CHANCE_TEXTURE_RELEASE, |_, _| {
            1u32.into_ret()
        });
        let image = image_tile(&mut e, UPDATE_NIF_FILE, 0);
        // A culled model: the texture update that this queues is not run.
        let model = e.get(image.tile, Tile::spModel).addr();
        e.mem.set_u32(model + 0x30, 1);
        e.call_log = Some(vec![]);
        tile_update_tile(&mut e, image.tile, false);
        assert_eq!(
            calls_to(&e, SET_TILE_TEXTURE),
            vec![vec![image.property, 0]]
        );
        assert_eq!(
            e.get(image.tile, Tile::uiFlags),
            UPDATE_NIF_FILE | UPDATE_TEXTURE | FLAG_DIRTY
        );
    }

    #[test]
    fn needs_update_bits_are_added_or_replaced_and_the_tile_is_queued() {
        let mut e = tile_engine();
        let queued = Rc::new(RefCell::new(vec![]));
        let seen = queued.clone();
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            assert_eq!(a[0], DIRTY_TILES_LIST);
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, UPDATE_POSITION);
        tile_add_needs_update(&mut e, tile, UPDATE_COLOR);
        assert_eq!(
            e.get(tile, Tile::uiFlags),
            UPDATE_POSITION | UPDATE_COLOR | FLAG_DIRTY
        );
        assert_eq!(*queued.borrow(), vec![tile.addr()]);
        // Nothing changes when the bits are there already, or are not update
        // bits at all.
        tile_add_needs_update(&mut e, tile, UPDATE_COLOR);
        tile_add_needs_update(&mut e, tile, FLAG_DIRTY);
        tile_set_needs_update(&mut e, tile, FLAG_PROMOTED);
        assert_eq!(
            e.get(tile, Tile::uiFlags),
            UPDATE_POSITION | UPDATE_COLOR | FLAG_DIRTY
        );
        // `SetNeedsUpdate` replaces the update bits and keeps the others.
        e.set(
            tile,
            Tile::uiFlags,
            UPDATE_POSITION | UPDATE_COLOR | FLAG_DIRTY | FLAG_PROMOTED,
        );
        tile_set_needs_update(&mut e, tile, UPDATE_GEOMETRY);
        assert_eq!(
            e.get(tile, Tile::uiFlags),
            UPDATE_GEOMETRY | FLAG_DIRTY | FLAG_PROMOTED
        );
        // The tile was dirty already: it is not queued again.
        assert_eq!(queued.borrow().len(), 1);
        // Set to nothing: changed, but queued only when not dirty yet.
        let other = typed_tile(&mut e, TYPE_RECT);
        e.set(other, Tile::uiFlags, UPDATE_TEXTURE);
        tile_set_needs_update(&mut e, other, 0);
        assert_eq!(e.get(other, Tile::uiFlags), FLAG_DIRTY);
        assert_eq!(queued.borrow().len(), 2);
    }

    #[test]
    fn the_next_dirty_tile_is_looked_at_or_taken() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, FLAG_DIRTY | UPDATE_COLOR);
        e.call_log = Some(vec![]);
        // Nothing in the list.
        assert!(tile_get_next_dirty_tile(&mut e, true).is_null());
        assert!(tile_get_next_dirty_tile(&mut e, false).is_null());
        // One node: look, then take.
        let node = e.mem.alloc(12);
        e.mem.set_u32(node + 8, tile.addr());
        e.mem.set_u32(DIRTY_TILES_LIST, node);
        e.mem.set_u32(DIRTY_TILES_LIST + 8, 1);
        assert_eq!(tile_get_next_dirty_tile(&mut e, false), tile);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_DIRTY | UPDATE_COLOR);
        e.register_double(LIST_REMOVE_HEAD, move |_, _| tile.addr().into_ret());
        assert_eq!(tile_get_next_dirty_tile(&mut e, true), tile);
        assert_eq!(e.get(tile, Tile::uiFlags), UPDATE_COLOR);
        assert_lock_balanced(&e);
    }

    #[test]
    fn a_dirty_tile_is_listed_once_and_leaves_the_hibernating_list() {
        let mut e = tile_engine();
        let appended = Rc::new(RefCell::new(vec![]));
        let seen = appended.clone();
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let removed = Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            1u32.into_ret()
        });
        // Null: nothing happens, not even the lock.
        e.call_log = Some(vec![]);
        tile_add_dirty_tile(&mut e, Ptr::NULL);
        assert!(calls_to(&e, TILE_LOCK).is_empty());
        let tile = typed_tile(&mut e, TYPE_RECT);
        tile_add_dirty_tile(&mut e, tile);
        tile_add_dirty_tile(&mut e, tile);
        assert_eq!(*appended.borrow(), vec![(DIRTY_TILES_LIST, tile.addr())]);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_DIRTY);
        assert!(removed.borrow().is_empty());
        // A hibernated tile is taken off the hibernating list.
        let sleeper = typed_tile(&mut e, TYPE_RECT);
        e.set(sleeper, Tile::uiFlags, FLAG_HIBERNATED);
        tile_add_dirty_tile(&mut e, sleeper);
        assert_eq!(e.get(sleeper, Tile::uiFlags), FLAG_DIRTY);
        assert_eq!(
            *removed.borrow(),
            vec![(HIBERNATING_TILES_LIST, sleeper.addr())]
        );
        assert_lock_balanced(&e);
    }

    #[test]
    fn putting_a_tile_to_sleep_lists_it_once() {
        let mut e = tile_engine();
        let node = e.mem.alloc(12);
        e.register_double(LIST_NEW_NODE, move |_, a| {
            assert_eq!(a[0], HIBERNATING_TILES_LIST + 8);
            node.into_ret()
        });
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.set(tile, Tile::uiFlags, FLAG_PROMOTED | UPDATE_COLOR);
        e.call_log = Some(vec![]);
        fn_00a077c0(&mut e, tile);
        assert_eq!(e.get(tile, Tile::uiFlags), FLAG_HIBERNATED | UPDATE_COLOR);
        assert_eq!(e.global::<u32>(HIBERNATING_TILE_COUNT), 1);
        assert_eq!(e.mem.u32(node + 8), tile.addr());
        assert_eq!(
            calls_to(&e, LIST_ADD_NODE_TAIL),
            vec![vec![HIBERNATING_TILES_LIST, node]]
        );
        // Asleep already, null, or the last tile of the list: nothing.
        fn_00a077c0(&mut e, tile);
        fn_00a077c0(&mut e, Ptr::NULL);
        let last = typed_tile(&mut e, TYPE_RECT);
        let tail = e.mem.alloc(12);
        e.mem.set_u32(tail + 8, last.addr());
        e.mem.set_u32(HIBERNATING_TILES_LIST + 4, tail);
        e.mem.set_u32(HIBERNATING_TILES_LIST + 8, 1);
        fn_00a077c0(&mut e, last);
        assert_eq!(e.get(last, Tile::uiFlags), 0);
        assert_eq!(calls_to(&e, LIST_ADD_NODE_TAIL).len(), 1);
        assert_eq!(e.global::<u32>(HIBERNATING_TILE_COUNT), 1);
        // Another tile is listed after the last one.
        let another = typed_tile(&mut e, TYPE_RECT);
        fn_00a077c0(&mut e, another);
        assert_eq!(calls_to(&e, LIST_ADD_NODE_TAIL).len(), 2);
    }

    #[test]
    fn hibernating_tiles_that_are_visible_again_wake_up() {
        let mut e = tile_engine();
        let awake = typed_tile(&mut e, TYPE_RECT);
        let asleep = typed_tile(&mut e, TYPE_RECT);
        let later = typed_tile(&mut e, TYPE_RECT);
        for tile in [awake, asleep, later] {
            e.set(tile, Tile::uiFlags, FLAG_HIBERNATED | FLAG_PROMOTED);
        }
        let culled = make_model(&mut e, &[]);
        e.mem.set_u32(culled + 0x30, 1);
        e.set(asleep, Tile::spModel, Ptr::new(culled));
        hibernating_list_with(&mut e, &[awake, asleep, later]);
        let queued = Rc::new(RefCell::new(vec![]));
        let seen = queued.clone();
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        tile_check_hibernating_tiles(&mut e);
        assert_eq!(e.get(awake, Tile::uiFlags), FLAG_DIRTY);
        assert_eq!(e.get(later, Tile::uiFlags), FLAG_DIRTY);
        assert_eq!(
            e.get(asleep, Tile::uiFlags),
            FLAG_HIBERNATED | FLAG_PROMOTED
        );
        assert_eq!(*queued.borrow(), vec![awake.addr(), later.addr()]);
    }

    #[test]
    fn clip_windows_are_marked_for_a_scissor_window_update() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let root = typed_tile(&mut e, TYPE_RECT);
        let [window, plain, nested_window, below_window] =
            [(); 4].map(|_| typed_tile(&mut e, TYPE_RECT));
        add_child(&mut e, root, window);
        add_child(&mut e, root, plain);
        add_child(&mut e, plain, nested_window);
        add_child(&mut e, window, below_window);
        give_trait(&mut e, window, TRAIT_CLIP_WINDOW, 1.0);
        give_trait(&mut e, nested_window, TRAIT_CLIP_WINDOW, 1.0);
        tile_update_clipwindows(&mut e, root);
        let marked =
            |e: &Engine, tile: Ptr<Tile>| e.get(tile, Tile::uiFlags) & UPDATE_SCISSOR_WINDOW != 0;
        assert!(marked(&e, window));
        assert!(marked(&e, nested_window));
        assert!(!marked(&e, plain));
        // The search does not go on below a clip window.
        assert!(!marked(&e, below_window));
    }

    #[test]
    fn re_clipping_limits_the_scissor_rectangles_to_the_screen() {
        for (rendered, expected) in [(false, [0, 10, 1920, 500]), (true, [0, 10, 800, 500])] {
            let mut e = tile_engine();
            let (menu_tile, parent) = scissor_scene(&mut e);
            let menu = Ptr::<()>::new(e.mem.u32(menu_tile.addr() + 0x3c));
            e.mem.set_u8(menu.addr() + 0x1c, rendered as u8);
            answer_float(&mut e, RENDERED_MENU_WIDTH, 800.0);
            answer_float(&mut e, RENDERED_MENU_HEIGHT, 600.0);
            let property = e.mem.alloc(0xb0);
            let child = clipped_child(&mut e, parent, property);
            // A clipped grandchild is reached by the recursion; a tile whose
            // clips trait is off is passed over.
            let grandchild_property = e.mem.alloc(0xb0);
            let skipped = typed_tile(&mut e, TYPE_RECT);
            add_child(&mut e, parent, skipped);
            e.call_log = Some(vec![]);
            tile_re_clip_children(&mut e, parent, -5.0, 10.5, 3000.0, 500.7);
            let rectangle: Vec<i32> = (0..4)
                .map(|slot| e.mem.i32(property + 0x9c + 4 * slot))
                .collect();
            assert_eq!(rectangle, expected.to_vec());
            assert_eq!(
                calls_to(&e, SHADER_PROPERTY_REFRESH),
                vec![vec![property, 1]]
            );
            let model = e.get(child, Tile::spModel).addr();
            assert_eq!(calls_to(&e, NI_ARRAY_COMPACT), vec![vec![model + 0x9c]]);
            assert_eq!(calls_to(&e, NI_ARRAY_UPDATE_SIZE), vec![vec![model + 0x9c]]);
            let _ = (grandchild_property, skipped);
        }
    }

    #[test]
    fn re_clipping_goes_through_the_whole_tree() {
        let mut e = tile_engine();
        let (_, parent) = scissor_scene(&mut e);
        let first = e.mem.alloc(0xb0);
        let child = clipped_child(&mut e, parent, first);
        let second = e.mem.alloc(0xb0);
        clipped_child(&mut e, child, second);
        // The properties are told apart by the object they are asked of.
        let objects = [(0x1234_0000 + first, first), (0x1234_0000 + second, second)];
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            objects
                .iter()
                .find(|(object, _)| *object == a[0])
                .map_or(0, |(_, property)| *property)
                .into_ret()
        });
        tile_re_clip_children(&mut e, parent, 1.0, 2.0, 3.0, 4.0);
        for property in [first, second] {
            let rectangle: Vec<i32> = (0..4)
                .map(|slot| e.mem.i32(property + 0x9c + 4 * slot))
                .collect();
            assert_eq!(rectangle, vec![1, 2, 3, 4]);
        }
    }

    /// The real `BSSimpleList` add-head, remove and remove-all over game
    /// memory (the global fade list and the texture entry list use them).
    fn install_simple_list_doubles(e: &mut Engine) {
        e.register(FADE_LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            if item == 0 {
                return Ret::default();
            }
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
            } else {
                let node = e.mem.alloc(8);
                let (head_item, head_next) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
                e.mem.set_u32(node, head_item);
                e.mem.set_u32(node + 4, head_next);
                e.mem.set_u32(a[0] + 4, node);
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        e.register(FADE_LIST_REMOVE, |e, a| {
            let item = e.mem.u32(a[1]);
            if item == 0 || (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) {
                return Ret::default();
            }
            let (mut previous, mut node) = (a[0], a[0]);
            while node != 0 && e.mem.u32(node) != item {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node == 0 {
                return Ret::default();
            }
            if node == a[0] {
                let next = e.mem.u32(node + 4);
                if next == 0 {
                    e.mem.set_u32(node, 0);
                } else {
                    let (next_item, next_next) = (e.mem.u32(next), e.mem.u32(next + 4));
                    e.mem.set_u32(node, next_item);
                    e.mem.set_u32(node + 4, next_next);
                    e.mem.free(next);
                }
            } else {
                let next = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, next);
                e.mem.free(node);
            }
            Ret::default()
        });
        e.register(SIMPLE_LIST_REMOVE_ALL, |e, a| {
            let mut node = e.mem.u32(a[0] + 4);
            while node != 0 {
                let next = e.mem.u32(node + 4);
                e.mem.free(node);
                node = next;
            }
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
    }

    /// The controls on the global fade list, newest first.
    fn fade_list(e: &Engine) -> Vec<u32> {
        let mut items = vec![];
        let mut node = FADE_CONTROLS_LIST;
        while node != 0 && e.mem.u32(node) != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// Puts a fade control on the global list.
    fn add_fade(
        e: &mut Engine,
        tile: Ptr<Tile>,
        trait_id: i32,
        range: (f32, f32),
        duration_millis: f32,
        kind: i32,
    ) -> u32 {
        let control = e.mem.alloc(0x1c);
        e.mem.set_f32(control, range.0);
        e.mem.set_f32(control + 4, range.1);
        e.mem.set_f32(control + 0xc, duration_millis);
        e.mem.set_i32(control + 0x10, trait_id);
        e.mem.set_u32(control + 0x14, tile.addr());
        e.mem.set_i32(control + 0x18, kind);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), control);
            e.call(FADE_LIST_ADD_HEAD, &args![FADE_CONTROLS_LIST, slot]);
        });
        control
    }

    /// The text table knows the two flash counters as 10001 and 10002.
    fn install_flash_traits(e: &mut Engine) {
        e.mem.set_cstr(FLASH_COUNT_NAME, b"_FlashCount");
        e.mem.set_cstr(TOTAL_FLASH_COUNT_NAME, b"_TotalFlashCount");
        e.register(TEXT_TABLE_FIND, |e, a| {
            let id = match e.mem.cstr(a[1]).as_slice() {
                b"_FlashCount" => 10001,
                b"_TotalFlashCount" => 10002,
                _ => return 0u32.into_ret(),
            };
            e.mem.set_i32(a[2], id);
            1u32.into_ret()
        });
    }

    #[test]
    fn adding_a_fade_control_stamps_it_and_replaces_the_tiles_old_fade() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        let clock = install_clock(&mut e);
        clock.set(1000);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        tile_add_fade_control(&mut e, tile, TRAIT_ALPHA, 0.0, 255.0, 2.0, FADE_STANDARD);
        let controls = fade_list(&e);
        assert_eq!(controls.len(), 1);
        let control = Ptr::<FadeControl>::new(controls[0]);
        assert_eq!(e.get(control, FadeControl::fStartValue), 0.0);
        assert_eq!(e.get(control, FadeControl::fEndValue), 255.0);
        assert_eq!(e.get(control, FadeControl::fDurationMillis), 2000.0);
        assert_eq!(e.get(control, FadeControl::uiStartTime), 1000);
        assert_eq!(e.get(control, FadeControl::iTrait), TRAIT_ALPHA);
        assert_eq!(e.get(control, FadeControl::pParent), tile);
        assert_eq!(e.get(control, FadeControl::eFadeType), FADE_STANDARD);
        // Another tile's fade of the same trait stays; the same tile's goes.
        tile_add_fade_control(&mut e, other, TRAIT_ALPHA, 1.0, 2.0, 1.0, 1);
        clock.set(1500);
        tile_add_fade_control(&mut e, tile, TRAIT_ALPHA, 10.0, 20.0, 1.0, FADE_STANDARD);
        let controls = fade_list(&e);
        assert_eq!(controls.len(), 2);
        let newest = Ptr::<FadeControl>::new(controls[0]);
        assert_eq!(e.get(newest, FadeControl::uiStartTime), 1500);
        assert_eq!(e.get(newest, FadeControl::fDurationMillis), 1000.0);
        assert_eq!(e.get(newest, FadeControl::pParent), tile);
        // No fade without a change or without a duration.
        tile_add_fade_control(&mut e, tile, TRAIT_RED, 5.0, 5.0, 1.0, FADE_STANDARD);
        tile_add_fade_control(&mut e, tile, TRAIT_RED, 5.0, 6.0, 0.0, FADE_STANDARD);
        tile_add_fade_control(&mut e, tile, TRAIT_RED, 5.0, 6.0, -1.0, FADE_STANDARD);
        assert_eq!(fade_list(&e).len(), 2);
    }

    #[test]
    fn blinking_fades_reset_their_flash_counters() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        install_flash_traits(&mut e);
        install_clock(&mut e);
        let sets = record_float_sets(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        // Blink thrice: the flash count restarts at 0.
        tile_add_fade_control(
            &mut e,
            tile,
            TRAIT_ALPHA,
            0.0,
            255.0,
            2.0,
            FADE_BLINK_THRICE,
        );
        assert_eq!(*sets.borrow(), vec![(10001, 0.0, 1)]);
        sets.borrow_mut().clear();
        // Blink and fade fast: 2 s are 4 flashes; the total is 2 and one
        // flash lasts 500 ms.
        tile_add_fade_control(
            &mut e,
            tile,
            TRAIT_ALPHA,
            0.0,
            255.0,
            2.0,
            FADE_BLINK_FAST_FADE,
        );
        assert_eq!(*sets.borrow(), vec![(10001, 0.0, 1), (10002, 2.0, 1)]);
        let control = Ptr::<FadeControl>::new(fade_list(&e)[0]);
        assert_eq!(e.get(control, FadeControl::fDurationMillis), 500.0);
        assert_eq!(fade_list(&e).len(), 1);
    }

    #[test]
    fn removing_fade_controls_takes_one_trait_or_all_of_a_tile() {
        let mut e = tile_engine();
        install_simple_list_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        let kept_other = add_fade(&mut e, other, TRAIT_ALPHA, (0.0, 1.0), 10.0, 0);
        let first = add_fade(&mut e, tile, TRAIT_ALPHA, (0.0, 1.0), 10.0, 0);
        let second = add_fade(&mut e, tile, TRAIT_RED, (0.0, 1.0), 10.0, 0);
        let third = add_fade(&mut e, tile, TRAIT_GREEN, (0.0, 1.0), 10.0, 0);
        assert_eq!(fade_list(&e), vec![third, second, first, kept_other]);
        tile_remove_fade_control(&mut e, tile, TRAIT_RED);
        assert_eq!(fade_list(&e), vec![third, first, kept_other]);
        assert_eq!(e.mem.block_size(second), None);
        tile_remove_fade_control(&mut e, tile, TRAIT_NONE);
        assert_eq!(fade_list(&e), vec![kept_other]);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(third), None);
        tile_remove_fade_control(&mut e, tile, TRAIT_ALPHA);
        assert_eq!(fade_list(&e), vec![kept_other]);
    }

    #[test]
    fn fade_queries_find_the_controls_of_the_tile() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        let clock = install_clock(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        assert!(!tile_has_fade_control(&mut e, tile, TRAIT_ALPHA));
        assert_eq!(tile_get_fade_type(&mut e, tile, TRAIT_ALPHA), 0);
        assert!(tile_get_fade_finished(&mut e, tile, TRAIT_ALPHA));
        give_trait(&mut e, tile, TRAIT_ALPHA, 3.5);
        assert_eq!(tile_get_fade_end_for(&mut e, tile, TRAIT_ALPHA), 3.5);
        // A fade of the other tile does not count.
        add_fade(&mut e, other, TRAIT_ALPHA, (0.0, 50.0), 1000.0, 2);
        assert!(!tile_has_fade_control(&mut e, tile, TRAIT_ALPHA));
        let first = add_fade(&mut e, tile, TRAIT_ALPHA, (0.0, 7.9), 1000.0, 3);
        e.mem.set_u32(first + 8, 100);
        assert!(tile_has_fade_control(&mut e, tile, TRAIT_ALPHA));
        assert!(!tile_has_fade_control(&mut e, tile, TRAIT_RED));
        assert_eq!(tile_get_fade_type(&mut e, tile, TRAIT_ALPHA), 3);
        // The end value is truncated; the last matching control counts.
        assert_eq!(tile_get_fade_end_for(&mut e, tile, TRAIT_ALPHA), 7.0);
        // Several controls: the last one in list order (the oldest) counts.
        add_fade(&mut e, tile, TRAIT_ALPHA, (0.0, 12.7), 1000.0, 3);
        assert_eq!(tile_get_fade_end_for(&mut e, tile, TRAIT_ALPHA), 7.0);
        // An end value of -1 means "use the trait's value".
        give_trait(&mut e, tile, TRAIT_BLUE, 4.5);
        add_fade(&mut e, tile, TRAIT_BLUE, (0.0, -1.0), 1000.0, 3);
        assert_eq!(tile_get_fade_end_for(&mut e, tile, TRAIT_BLUE), 4.5);
        // Finished once the duration is over, never for type 1.
        let control = add_fade(&mut e, tile, TRAIT_RED, (0.0, 1.0), 1000.0, 0);
        e.mem.set_u32(control + 8, 500);
        clock.set(1499);
        assert!(!tile_get_fade_finished(&mut e, tile, TRAIT_RED));
        clock.set(1500);
        assert!(tile_get_fade_finished(&mut e, tile, TRAIT_RED));
        let repeating = add_fade(
            &mut e,
            tile,
            TRAIT_GREEN,
            (0.0, 1.0),
            1000.0,
            FADE_NONLINEAR_LONG_DARK_REPEATING,
        );
        e.mem.set_u32(repeating + 8, 0);
        clock.set(1_000_000);
        assert!(!tile_get_fade_finished(&mut e, tile, TRAIT_GREEN));
    }

    /// Fades the given traits of a tile: `fades` are (trait, start, end,
    /// duration, type), the clock stands at `now`. Returns the float sets
    /// the update made as (trait, value).
    fn run_fades(
        e: &mut Engine,
        tile: Ptr<Tile>,
        fades: &[(i32, f32, f32, f32, i32)],
        now: u32,
    ) -> (Vec<(i32, f32)>, Vec<u32>) {
        e.mem.set_u32(FADE_CONTROLS_LIST, 0);
        e.mem.set_u32(FADE_CONTROLS_LIST + 4, 0);
        let clock = install_clock(e);
        clock.set(now);
        let sets = record_float_sets(e);
        let controls: Vec<u32> = fades
            .iter()
            .map(|&(trait_id, start, end, duration, kind)| {
                add_fade(e, tile, trait_id, (start, end), duration, kind)
            })
            .collect();
        tile_update_fade_controls(e);
        let made = sets
            .borrow()
            .iter()
            .map(|&(id, value, _)| (id, value))
            .collect();
        (made, controls)
    }

    /// The CRT's `fabs`, `floor` and `ceil` over a `double` argument.
    fn install_double_math(e: &mut Engine) {
        fn argument(a: &[u32]) -> f64 {
            f64::from_bits(a[0] as u64 | (a[1] as u64) << 32)
        }
        e.register(FABS, |_, a| argument(a).abs().into_ret());
        e.register(FLOOR, |_, a| argument(a).floor().into_ret());
        e.register(CEIL, |_, a| argument(a).ceil().into_ret());
    }

    #[test]
    fn update_fade_controls_with_no_controls_does_nothing() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        tile_update_fade_controls(&mut e);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn a_standard_fade_runs_linearly_and_ends_at_the_end_value() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let (sets, controls) = run_fades(
            &mut e,
            tile,
            &[(TRAIT_RED, 100.0, 200.0, 1000.0, FADE_STANDARD)],
            250,
        );
        assert_eq!(sets, vec![(TRAIT_RED, 125.0)]);
        assert_eq!(fade_list(&e), controls);
        // Past the end: clamped to the end value and finished (freed).
        let clock = install_clock(&mut e);
        clock.set(5000);
        let sets = record_float_sets(&mut e);
        tile_update_fade_controls(&mut e);
        assert_eq!(*sets.borrow(), vec![(TRAIT_RED, 200.0, 1)]);
        assert!(fade_list(&e).is_empty());
        assert_eq!(e.mem.block_size(controls[0]), None);
    }

    #[test]
    fn a_standard_alpha_fade_of_a_3d_tile_also_fades_the_model() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_3D);
        // The model is a geometry (virtual +0xC answers 0, +0x18 answers
        // true) with a shader property.
        let model = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0xc, 0x7700_000c);
        e.mem.set_u32(vtable + 0x18, 0x7700_0018);
        e.register_double(0x7700_000c, |_, _| 0u32.into_ret());
        e.register_double(0x7700_0018, |_, _| 1u32.into_ret());
        e.mem.set_u32(model, vtable);
        e.set(tile, Tile::spModel, Ptr::new(model));
        let property = e.mem.alloc(0x90);
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!(a[1], 3);
            property.into_ret()
        });
        let (sets, _) = run_fades(
            &mut e,
            tile,
            &[(TRAIT_ALPHA, 0.0, 255.0, 1000.0, FADE_STANDARD)],
            500,
        );
        assert_eq!(sets, vec![(TRAIT_ALPHA, 127.5)]);
        assert_eq!(e.mem.f32(property + 0x78), 0.5);
    }

    #[test]
    fn the_fade_in_hold_fade_out_curve_has_three_phases() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let fade = [(TRAIT_ALPHA, 10.0, 110.0, 1000.0, FADE_IN_HOLD_FADE_OUT)];
        // Rising: from the end... the curve goes from start to end.
        let (sets, _) = run_fades(&mut e, tile, &fade, 83);
        let expected = (10.0f64 + (110.0 - 10.0) * ((0.083f32 as f64) / 0.16666f32 as f64)) as f32;
        assert_eq!(sets, vec![(TRAIT_ALPHA, expected)]);
        // Holding at the end value.
        let (sets, _) = run_fades(&mut e, tile, &fade, 500);
        assert_eq!(sets, vec![(TRAIT_ALPHA, 110.0)]);
        // Falling back.
        let (sets, _) = run_fades(&mut e, tile, &fade, 916);
        let progress = 0.916f32 as f64;
        let low = 0.83334f32 as f64;
        let expected = (110.0 + (10.0 - 110.0) * ((progress - low) / (1.0 - low))) as f32;
        assert_eq!(sets, vec![(TRAIT_ALPHA, expected)]);
        // Over: the start value again, and finished.
        let (sets, controls) = run_fades(&mut e, tile, &fade, 1000);
        assert_eq!(sets, vec![(TRAIT_ALPHA, 10.0)]);
        assert!(!fade_list(&e).contains(&controls[0]));
    }

    #[test]
    fn a_blink_goes_to_the_end_and_back_each_cycle() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        install_flash_traits(&mut e);
        install_double_math(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, tile, 10001, 0.0);
        let fade = [(TRAIT_ALPHA, 0.0, 100.0, 1000.0, FADE_BLINK_THRICE)];
        // A quarter into the first flash: halfway up.
        let (sets, _) = run_fades(&mut e, tile, &fade, 250);
        assert_eq!(sets, vec![(TRAIT_ALPHA, 50.0)]);
        // The end of the cycle: the flash count rises to 1, the cycle
        // restarts (from the start value).
        let (sets, controls) = run_fades(&mut e, tile, &fade, 1000);
        assert_eq!(sets, vec![(10001, 1.0), (TRAIT_ALPHA, 0.0)]);
        assert_eq!(e.mem.u32(controls[0] + 8), 1000);
    }

    #[test]
    fn the_third_flash_of_a_blink_ends_at_the_end_value() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        install_flash_traits(&mut e);
        install_double_math(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, tile, 10001, 3.5);
        let fade = [(TRAIT_ALPHA, 0.0, 100.0, 1000.0, FADE_BLINK_THRICE)];
        // Climbing toward the end value without passing it.
        let (sets, _) = run_fades(&mut e, tile, &fade, 250);
        assert_eq!(sets, vec![(TRAIT_ALPHA, 25.0)]);
        // The end of the cycle: the count is past 3: set the end value and
        // finish.
        let (sets, controls) = run_fades(&mut e, tile, &fade, 1000);
        assert_eq!(
            sets,
            vec![(10001, 4.5), (TRAIT_ALPHA, 100.0), (TRAIT_ALPHA, 100.0)]
        );
        assert!(!fade_list(&e).contains(&controls[0]));
    }

    #[test]
    fn a_repeating_fade_restarts_and_a_fast_blink_finishes_after_its_total() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        install_flash_traits(&mut e);
        install_double_math(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        // Type 1 restarts forever and does not touch the flash counter.
        let (sets, controls) = run_fades(
            &mut e,
            tile,
            &[(
                TRAIT_ALPHA,
                0.0,
                100.0,
                1000.0,
                FADE_NONLINEAR_LONG_DARK_REPEATING,
            )],
            1000,
        );
        assert_eq!(sets, vec![(TRAIT_ALPHA, 0.0)]);
        assert!(fade_list(&e).contains(&controls[0]));
        // Type 3 with a total of 2: the count 3 is past it: back to the
        // start value, finished.
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        install_simple_list_doubles(&mut e);
        install_flash_traits(&mut e);
        install_double_math(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, tile, 10001, 3.0);
        give_trait(&mut e, tile, 10002, 2.0);
        let (sets, controls) = run_fades(
            &mut e,
            tile,
            &[(TRAIT_ALPHA, 0.0, 100.0, 1000.0, FADE_BLINK_FAST_FADE)],
            1000,
        );
        assert_eq!(
            sets,
            vec![(10001, 4.0), (TRAIT_ALPHA, 0.0), (TRAIT_ALPHA, 0.0)]
        );
        assert!(!fade_list(&e).contains(&controls[0]));
        // Fades of an unknown type are left alone.
        let (sets, controls) =
            run_fades(&mut e, tile, &[(TRAIT_ALPHA, 0.0, 100.0, 1000.0, 9)], 500);
        assert!(sets.is_empty());
        assert!(fade_list(&e).contains(&controls[0]));
    }

    #[test]
    fn fade_in_3d_sets_the_alpha_of_every_geometry_below_the_node() {
        let mut e = tile_engine();
        let properties = Rc::new(RefCell::new(std::collections::HashMap::new()));
        // A geometry: `IsNode` answers 0, `IsGeometry` answers true.
        let geometry_vtable = e.mem.alloc(0x30);
        e.mem.set_u32(geometry_vtable + 0xc, 0x7800_000c);
        e.mem.set_u32(geometry_vtable + 0x18, 0x7800_0018);
        e.register_double(0x7800_000c, |_, _| 0u32.into_ret());
        e.register_double(0x7800_0018, |_, _| 1u32.into_ret());
        // A node: `IsNode` answers itself, `IsGeometry` answers false.
        let node_vtable = e.mem.alloc(0x30);
        e.mem.set_u32(node_vtable + 0xc, 0x7800_010c);
        e.mem.set_u32(node_vtable + 0x18, 0x7800_0118);
        e.register_double(0x7800_010c, |_, a| a[0].into_ret());
        e.register_double(0x7800_0118, |_, _| 0u32.into_ret());
        let make = |e: &mut Engine, vtable: u32| {
            let object = e.mem.alloc(0xc0);
            e.mem.set_u32(object, vtable);
            object
        };
        let geometry_a = make(&mut e, geometry_vtable);
        let geometry_b = make(&mut e, geometry_vtable);
        let inner = make_model(&mut e, &[geometry_b]);
        e.mem.set_u32(inner, node_vtable);
        let outer = make_model(&mut e, &[geometry_a, inner]);
        e.mem.set_u32(outer, node_vtable);
        for geometry in [geometry_a, geometry_b] {
            let property = e.mem.alloc(0x90);
            properties.borrow_mut().insert(geometry, property);
        }
        let lookup = properties.clone();
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!(a[1], 3);
            lookup.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let tile = typed_tile(&mut e, TYPE_RECT);
        tile_fade_in_3d(&mut e, tile, Ptr::new(outer), 0.25);
        for property in properties.borrow().values() {
            assert_eq!(e.mem.f32(property + 0x78), 0.25);
        }
        // A null node does nothing.
        tile_fade_in_3d(&mut e, tile, Ptr::NULL, 0.5);
    }

    #[test]
    fn moving_a_tile_under_a_parent_keeps_the_child_counts_and_lists() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        let sets = record_float_sets(&mut e);
        let events = Rc::new(RefCell::new(vec![]));
        for (name, address) in [("remove", LIST_REMOVE_ITEM), ("add_head", LIST_ADD_HEAD)] {
            let seen = events.clone();
            e.register_double(address, move |e, a| {
                seen.borrow_mut().push((name, a[0], e.mem.u32(a[1])));
                1u32.into_ret()
            });
        }
        let seen = events.clone();
        e.register_double(LIST_ADD_AFTER, move |e, a| {
            seen.borrow_mut().push(("add_after", a[0], e.mem.u32(a[2])));
            seen.borrow_mut().push(("after_node", a[1], 0));
            Ret::default()
        });
        let old_parent = typed_tile(&mut e, TYPE_RECT);
        let new_parent = typed_tile(&mut e, TYPE_RECT);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let sibling = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, old_parent, tile);
        add_child(&mut e, new_parent, sibling);
        give_trait(&mut e, old_parent, TRAIT_CHILD_COUNT, 3.0);
        give_trait(&mut e, new_parent, TRAIT_CHILD_COUNT, 1.0);
        // No sibling asked for: the tile goes to the head.
        tile_set_parent(&mut e, tile, new_parent, Ptr::NULL);
        assert_eq!(e.get(tile, Tile::pParent), new_parent);
        assert_eq!(
            *sets.borrow(),
            vec![(TRAIT_CHILD_COUNT, 2.0, 1), (TRAIT_CHILD_COUNT, 2.0, 1)]
        );
        assert_eq!(
            *events.borrow(),
            vec![
                ("remove", old_parent.addr() + 4, tile.addr()),
                ("add_head", new_parent.addr() + 4, tile.addr())
            ]
        );
        // Behind a sibling that is on the list: removed, then added behind
        // that sibling's node.
        events.borrow_mut().clear();
        let sibling_node = e.mem.u32(new_parent.addr() + 4);
        tile_set_parent(&mut e, tile, new_parent, sibling);
        // (The tile is under the new parent already, so it is taken off its
        // list as the old parent first.)
        assert_eq!(
            *events.borrow(),
            vec![
                ("remove", new_parent.addr() + 4, tile.addr()),
                ("remove", new_parent.addr() + 4, tile.addr()),
                ("add_after", new_parent.addr() + 4, tile.addr()),
                ("after_node", sibling_node, 0)
            ]
        );
        // A sibling that is not on the list: the head again.
        events.borrow_mut().clear();
        let stranger = typed_tile(&mut e, TYPE_RECT);
        tile_set_parent(&mut e, tile, new_parent, stranger);
        assert_eq!(events.borrow().last().unwrap().0, "add_head");
        // Detaching: a null parent only clears the link; a released old
        // parent keeps its count and list.
        events.borrow_mut().clear();
        sets.borrow_mut().clear();
        e.set(new_parent, Tile::uiFlags, FLAG_RELEASED);
        tile_set_parent(&mut e, tile, Ptr::NULL, Ptr::NULL);
        assert!(e.get(tile, Tile::pParent).is_null());
        assert!(events.borrow().is_empty());
        assert!(sets.borrow().is_empty());
    }

    #[test]
    fn the_parent_model_is_the_nearest_one() {
        let mut e = tile_engine();
        let root = typed_tile(&mut e, TYPE_RECT);
        let middle = typed_tile(&mut e, TYPE_RECT);
        let leaf = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, root, middle);
        add_child(&mut e, middle, leaf);
        assert!(tile_get_parent_model(&mut e, leaf).is_null());
        let root_model = make_model(&mut e, &[]);
        e.set(root, Tile::spModel, Ptr::new(root_model));
        assert_eq!(tile_get_parent_model(&mut e, leaf).addr(), root_model);
        let middle_model = make_model(&mut e, &[]);
        e.set(middle, Tile::spModel, Ptr::new(middle_model));
        assert_eq!(tile_get_parent_model(&mut e, leaf).addr(), middle_model);
        assert_eq!(tile_get_parent_model(&mut e, root).addr(), root_model);
    }

    /// `NiPointer::operator=` over game memory.
    fn install_ni_pointer_assign(e: &mut Engine) {
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let old = e.mem.u32(a[0]);
            if a[1] != 0 {
                let count = e.mem.i32(a[1] + 4) + 1;
                e.mem.set_i32(a[1] + 4, count);
            }
            e.mem.set_u32(a[0], a[1]);
            if old != 0 {
                let count = e.mem.i32(old + 4) - 1;
                e.mem.set_i32(old + 4, count);
                if count == 0 {
                    e.vcall(old, 4, &[]);
                }
            }
            Ret::default()
        });
    }

    #[test]
    fn deleting_the_model_detaches_it_and_drops_the_references() {
        let mut e = tile_engine();
        install_ni_pointer_assign(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        // A model with one reference (the tile's): its destructor is slot 4.
        let model = e.mem.alloc(0xc0);
        e.mem.set_u32(model + 4, 1);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7900_0004);
        e.mem.set_u32(model, vtable);
        let destroyed = Rc::new(RefCell::new(vec![]));
        let seen = destroyed.clone();
        e.register_double(0x7900_0004, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        // Its parent node removes children through slot 0xE8.
        let parent_node = e.mem.alloc(0x10);
        let parent_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(parent_vtable + 0xe8, 0x7900_00e8);
        e.mem.set_u32(parent_node, parent_vtable);
        e.mem.set_u32(model + 0x18, parent_node);
        let detached = Rc::new(RefCell::new(vec![]));
        let seen = detached.clone();
        e.register_double(0x7900_00e8, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        e.set(tile, Tile::spModel, Ptr::new(model));
        e.call_log = Some(vec![]);
        tile_delete_model(&mut e, tile);
        assert!(e.get(tile, Tile::spModel).is_null());
        assert_eq!(*detached.borrow(), vec![(parent_node, model)]);
        assert_eq!(*destroyed.borrow(), vec![model]);
        assert_eq!(e.mem.i32(model + 4), 0);
        // Nothing happens for a tile without a model.
        e.call_log = Some(vec![]);
        tile_delete_model(&mut e, tile);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn deleting_a_model_without_a_parent_node_still_releases_it() {
        let mut e = tile_engine();
        install_ni_pointer_assign(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        // Another reference keeps the model alive.
        let model = e.mem.alloc(0xc0);
        e.mem.set_u32(model + 4, 2);
        e.set(tile, Tile::spModel, Ptr::new(model));
        tile_delete_model(&mut e, tile);
        assert!(e.get(tile, Tile::spModel).is_null());
        assert_eq!(e.mem.i32(model + 4), 1);
    }

    /// root > menu (a menu tile whose Menu answers `0xabcd` at +4) > "a" >
    /// "b", "c" (named "Somewhere"), with a text table that knows the link
    /// words.
    fn link_scene(e: &mut Engine) -> [Ptr<Tile>; 5] {
        provide_constants(e);
        install_value_array_doubles(e);
        install_strcmp(e);
        let root = typed_tile(e, TYPE_RECT);
        // A menu tile with room for its own fields behind the Tile part.
        let kind = typed_tile(e, TYPE_MENU);
        let vtable = e.mem.u32(kind.addr());
        let menu_tile: Ptr<Tile> = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(menu_tile.addr(), vtable);
        let menu = e.mem.alloc(0x28);
        e.mem.set_u32(menu_tile.addr() + 0x3c, menu);
        e.mem.set_u32(menu + 4, 0xabcd);
        add_child(e, root, menu_tile);
        let parent = typed_tile(e, TYPE_RECT);
        let first = typed_tile(e, TYPE_RECT);
        let second = typed_tile(e, TYPE_RECT);
        add_child(e, menu_tile, parent);
        add_child(e, parent, first);
        add_child(e, parent, second);
        for (tile, name) in [(parent, "a"), (first, "b"), (second, "Somewhere")] {
            let text = cstring(e, name);
            e.mem.set_u32(tile.addr() + 0x20, text);
        }
        e.register(TEXT_TABLE_FIND, |e, a| {
            let code = match e.mem.cstr(a[1]).as_slice() {
                b"parent" => LINK_PARENT,
                b"self" => LINK_SELF,
                b"sibling" => LINK_SIBLING,
                b"child" => LINK_CHILD,
                b"root" => LINK_MENUS_ROOT,
                b"special" => LINK_00706CF0,
                b"menu" => LINK_MENU,
                b"grandparent" => LINK_GRANDPARENT,
                _ => return 0u32.into_ret(),
            };
            e.mem.set_i32(a[2], code);
            1u32.into_ret()
        });
        e.register_double(INTERFACE_GET_MENUS_ROOT, move |_, _| root.addr().into_ret());
        e.register(INTERFACE_00706CF0, |_, _| 0x77u32.into_ret());
        [root, menu_tile, parent, first, second]
    }

    fn find(e: &mut Engine, tile: Ptr<Tile>, path: &str) -> u32 {
        let text = cstring(e, path);
        fn_00a08b20(e, tile, Ptr::new(text)).addr()
    }

    #[test]
    fn a_link_names_a_relative_tile() {
        let mut e = tile_engine();
        let [root, menu_tile, parent, first, second] = link_scene(&mut e);
        assert_eq!(find(&mut e, first, "parent"), parent.addr());
        assert_eq!(find(&mut e, first, "self"), first.addr());
        assert_eq!(find(&mut e, first, "grandparent"), menu_tile.addr());
        assert_eq!(find(&mut e, root, "grandparent"), 0);
        assert_eq!(find(&mut e, first, "root"), root.addr());
        assert_eq!(find(&mut e, first, "special"), 0x77);
        assert_eq!(find(&mut e, first, "menu"), 0xabcd);
        // The next sibling, wrapping around; or a sibling by name, case-blind.
        assert_eq!(find(&mut e, first, "sibling"), second.addr());
        assert_eq!(find(&mut e, second, "sibling"), first.addr());
        assert_eq!(find(&mut e, second, "sibling(B)"), first.addr());
        assert_eq!(find(&mut e, first, "sibling(nobody)"), 0);
        // The first child, or a descendant by name.
        assert_eq!(find(&mut e, parent, "child"), first.addr());
        assert_eq!(find(&mut e, parent, "child(Somewhere)"), second.addr());
        assert_eq!(find(&mut e, first, "child"), 0);
        assert_eq!(find(&mut e, first, "child(x)"), 0);
        // Any other word: a name searched under the menu tile, case-blind.
        assert_eq!(find(&mut e, first, "somewhere"), second.addr());
        assert_eq!(find(&mut e, first, "missing"), 0);
        // Without a tile the search starts at the menus root.
        assert_eq!(find(&mut e, Ptr::NULL, "A"), parent.addr());
    }

    #[test]
    fn a_tile_is_found_by_name_or_by_id_depth_first() {
        let mut e = tile_engine();
        let [root, menu_tile, parent, first, second] = link_scene(&mut e);
        let find_name = |e: &mut Engine, tile: Ptr<Tile>, name: &str| {
            let text = cstring(e, name);
            fn_00a08f20(e, tile, Ptr::new(text)).addr()
        };
        assert_eq!(find_name(&mut e, root, "SOMEWHERE"), second.addr());
        assert_eq!(find_name(&mut e, parent, "a"), parent.addr());
        assert_eq!(find_name(&mut e, first, "a"), 0);
        assert_eq!(find_name(&mut e, Ptr::NULL, "a"), 0);
        // By id: the first match in depth-first order.
        give_trait(&mut e, parent, TRAIT_ID, 5.0);
        give_trait(&mut e, first, TRAIT_ID, 6.0);
        give_trait(&mut e, second, TRAIT_ID, 6.0);
        assert_eq!(fn_00a08fb0(&mut e, root, 6), first);
        assert_eq!(fn_00a08fb0(&mut e, menu_tile, 5), parent);
        assert!(fn_00a08fb0(&mut e, root, 7).is_null());
        assert_eq!(fn_00a08fb0(&mut e, second, 6), second);
    }

    #[test]
    fn menus_are_looked_up_by_class_number() {
        let mut e = tile_engine();
        let table = e.mem.alloc(4 * 0x60);
        // The game indexes `table[class - 1001]`, 3 entries long.
        for index in 0..3u32 {
            e.mem.set_u32(table + 4 * index, 0x1000 + index);
        }
        e.set_global(MENU_CLASS_TABLE, table);
        e.set_global(MENU_CLASS_COUNT, 3u16);
        assert_eq!(tile_get_menu_by_class(&mut e, 1001), 0x1000);
        assert_eq!(tile_get_menu_by_class(&mut e, 1003), 0x1002);
        // Past the entries, and outside the class range.
        assert_eq!(tile_get_menu_by_class(&mut e, 1004), 0);
        assert_eq!(tile_get_menu_by_class(&mut e, 1000), 0);
        assert_eq!(tile_get_menu_by_class(&mut e, 0x43d), 0);
        e.set_global(MENU_CLASS_COUNT, 0x60u16);
        assert_eq!(tile_get_menu_by_class(&mut e, 0x43c), 0);
        assert_eq!(tile_get_menu_by_class(&mut e, 0x43d), 0);
    }

    // --- GetTextureAtlasInfo --------------------------------------------

    /// What the atlas-file doubles saw.
    struct AtlasDoubles {
        /// Paths handed to `FileFinder::GetFile`.
        opened: Rc<RefCell<Vec<String>>>,
        /// The lines still to be read.
        lines: Rc<RefCell<std::collections::VecDeque<String>>>,
        /// Strings assigned with `BSStringT::Set` as (string, text).
        strings: Rc<RefCell<Vec<(u32, String)>>>,
        /// Deleted file objects.
        deleted: Rc<RefCell<Vec<u32>>>,
        /// What opening the file answers.
        opens: Rc<Cell<bool>>,
    }

    fn text_of(e: &Engine, string: u32) -> String {
        let (buffer, length) = (e.mem.u32(string + 4), e.mem.u32(string + 0x14));
        String::from_utf8(e.mem.bytes(buffer, length)).unwrap()
    }

    fn set_text(e: &mut Engine, string: u32, text: &[u8]) {
        let buffer = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(buffer, text);
        e.mem.set_u32(string + 4, buffer);
        e.mem.set_u32(string + 0x14, text.len() as u32);
        e.mem.set_u32(string + 0x18, 0x1f);
    }

    /// Doubles for everything `GetTextureAtlasInfo` calls: the CRT string
    /// functions, the game's `std::string` (kept as a pointer, length and
    /// capacity of 0x1F), the file finder and the string setter.
    fn install_atlas_doubles(e: &mut Engine, lines: &[&str]) -> AtlasDoubles {
        provide_constants(e);
        install_strcmp(e);
        install_simple_list_doubles(e);
        e.mem.set_cstr(TEXTURE_DIRECTORY, b"Data\\Textures\\");
        e.mem.set_cstr(ATLAS_DELIMITERS, b" ,\t");
        e.mem.set_cstr(ATLAS_DELIMITERS_NO_TAB, b" ,");
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            text.iter()
                .rposition(|&byte| byte as u32 == a[1])
                .map_or(0, |at| a[0] + at as u32)
                .into_ret()
        });
        let saved = Rc::new(Cell::new(0u32));
        e.register_double(STRTOK, move |e, a| {
            let delimiters = e.mem.cstr(a[1]);
            let mut at = if a[0] != 0 { a[0] } else { saved.get() };
            if at == 0 {
                return 0u32.into_ret();
            }
            while e.mem.u8(at) != 0 && delimiters.contains(&e.mem.u8(at)) {
                at += 1;
            }
            if e.mem.u8(at) == 0 {
                saved.set(0);
                return 0u32.into_ret();
            }
            let start = at;
            while e.mem.u8(at) != 0 && !delimiters.contains(&e.mem.u8(at)) {
                at += 1;
            }
            if e.mem.u8(at) != 0 {
                e.mem.set_u8(at, 0);
                at += 1;
            }
            saved.set(at);
            start.into_ret()
        });
        e.register(STD_STRING_TIDY, |e, a| {
            set_text(e, a[0], b"");
            Ret::default()
        });
        e.register(STD_STRING_ASSIGN, |e, a| {
            let text = e.mem.bytes(a[1], a[2]);
            set_text(e, a[0], &text);
            Ret::default()
        });
        e.register(STD_STRING_RFIND, |e, a| {
            let text = text_of(e, a[0]).into_bytes();
            let wanted = e.mem.u8(a[1]);
            text.iter()
                .rposition(|&byte| byte == wanted)
                .map_or(u32::MAX, |at| at as u32)
                .into_ret()
        });
        e.register(STD_STRING_SUBSTR, |e, a| {
            let text = text_of(e, a[0]).into_bytes();
            let end = text.len().min(a[2] as usize + a[3] as usize);
            set_text(e, a[1], &text[a[2] as usize..end]);
            a[1].into_ret()
        });
        e.register(STD_STRING_ASSIGN_STRING, |e, a| {
            let text = text_of(e, a[1]).into_bytes();
            set_text(e, a[0], &text[a[2] as usize..]);
            Ret::default()
        });
        e.register(STD_STRING_APPEND, |e, a| {
            let mut text = text_of(e, a[0]).into_bytes();
            text.extend(e.mem.bytes(a[1], a[2]));
            set_text(e, a[0], &text);
            Ret::default()
        });
        e.register(TEXTURE_ATLAS_ENTRY_CONSTRUCT, |_, a| a[0].into_ret());
        let doubles = AtlasDoubles {
            opened: Rc::new(RefCell::new(vec![])),
            lines: Rc::new(RefCell::new(
                lines.iter().map(|line| line.to_string()).collect(),
            )),
            strings: Rc::new(RefCell::new(vec![])),
            deleted: Rc::new(RefCell::new(vec![])),
            opens: Rc::new(Cell::new(true)),
        };
        let seen = doubles.strings.clone();
        e.register_double(STRING_SET, move |e, a| {
            e.mem.set_u32(a[0], a[1]);
            if a[1] != 0 {
                seen.borrow_mut()
                    .push((a[0], String::from_utf8(e.mem.cstr(a[1])).unwrap()));
            }
            Ret::default()
        });
        // The file object: open (slot +0x20), read a line (+0x34), delete (0).
        let file_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(file_vtable, 0x7a00_0000);
        e.mem.set_u32(file_vtable + 0x20, 0x7a00_0020);
        e.mem.set_u32(file_vtable + 0x34, 0x7a00_0034);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file, file_vtable);
        let (opened, opens) = (doubles.opened.clone(), doubles.opens.clone());
        e.register_double(FILE_FINDER_GET_FILE, move |e, a| {
            opened
                .borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[0])).unwrap());
            file.into_ret()
        });
        let opens_now = opens.clone();
        e.register_double(0x7a00_0020, move |_, _| opens_now.get().into_ret());
        let remaining = doubles.lines.clone();
        e.register_double(0x7a00_0034, move |e, a| {
            let Some(line) = remaining.borrow_mut().pop_front() else {
                return 0u32.into_ret();
            };
            e.mem.set_cstr(a[1], line.as_bytes());
            1u32.into_ret()
        });
        let seen = doubles.deleted.clone();
        e.register_double(0x7a00_0000, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        doubles
    }

    fn read_rect(e: &Engine, rect: u32) -> [f32; 4] {
        [
            e.mem.f32(rect),
            e.mem.f32(rect + 4),
            e.mem.f32(rect + 8),
            e.mem.f32(rect + 12),
        ]
    }

    #[test]
    fn atlas_entries_that_are_cached_answer_without_reading_the_file() {
        let mut e = tile_engine();
        let doubles = install_atlas_doubles(&mut e, &[]);
        cached_atlas_entry(
            &mut e,
            "atlas.txt",
            "icon",
            "Interface\\icons.dds",
            [10.0, 74.0, 20.0, 52.0],
        );
        let atlas = Ptr::<()>::new(cstring(&mut e, "atlas.txt"));
        let texture = e.mem.alloc(8);
        let rect = e.mem.alloc(16);
        // The sub-texture is the part after the last backslash.
        let subtexture = Ptr::<()>::new(cstring(&mut e, "Interface\\Shared\\icon"));
        tile_get_texture_atlas_info(&mut e, atlas, subtexture, Ptr::new(texture), Ptr::new(rect));
        assert_eq!(read_rect(&e, rect), [10.0, 74.0, 20.0, 52.0]);
        assert_eq!(
            *doubles.strings.borrow(),
            vec![(texture, "Interface\\icons.dds".to_string())]
        );
        assert!(doubles.opened.borrow().is_empty());
        // Another sub-texture of a known atlas leaves the outputs alone and
        // reads nothing either.
        e.mem.set_f32(rect, -1.0);
        let other = Ptr::<()>::new(cstring(&mut e, "other"));
        tile_get_texture_atlas_info(&mut e, atlas, other, Ptr::NULL, Ptr::new(rect));
        assert_eq!(e.mem.f32(rect), -1.0);
        assert!(doubles.opened.borrow().is_empty());
    }

    #[test]
    fn atlas_info_needs_both_names_and_an_output() {
        let mut e = tile_engine();
        let doubles = install_atlas_doubles(&mut e, &[]);
        let name = Ptr::<()>::new(cstring(&mut e, "x"));
        let rect = Ptr::<()>::new(e.mem.alloc(16));
        tile_get_texture_atlas_info(&mut e, Ptr::NULL, name, Ptr::NULL, rect);
        tile_get_texture_atlas_info(&mut e, name, Ptr::NULL, Ptr::NULL, rect);
        tile_get_texture_atlas_info(&mut e, name, name, Ptr::NULL, Ptr::NULL);
        assert!(doubles.opened.borrow().is_empty());
    }

    #[test]
    fn an_unknown_atlas_is_read_from_its_description_file() {
        let mut e = tile_engine();
        let doubles = install_atlas_doubles(
            &mut e,
            &[
                "# a comment",
                " blank start",
                "other other.dds 0 0 1 2 0 3 4",
                "icon  icons.dds, 0 0 16 32 0 64 128",
            ],
        );
        let atlas = Ptr::<()>::new(cstring(&mut e, "Interface\\Shared\\atlas.txt"));
        let subtexture = Ptr::<()>::new(cstring(&mut e, "icon"));
        let texture = e.mem.alloc(8);
        let rect = e.mem.alloc(16);
        tile_get_texture_atlas_info(&mut e, atlas, subtexture, Ptr::new(texture), Ptr::new(rect));
        assert_eq!(
            *doubles.opened.borrow(),
            vec!["Data\\Textures\\Interface\\Shared\\atlas.txt".to_string()]
        );
        // Right and bottom are left plus width and top plus height.
        assert_eq!(read_rect(&e, rect), [16.0, 80.0, 32.0, 160.0]);
        // The texture file is in the atlas's directory.
        let texture_text = (texture, "Interface\\Shared\\icons.dds".to_string());
        assert!(doubles.strings.borrow().contains(&texture_text));
        // Both entries are kept: the sub-texture and atlas names are stored.
        let entries: Vec<u32> = {
            let mut items = vec![];
            let mut node = TEXTURE_ENTRY_LIST;
            while node != 0 && e.mem.u32(node) != 0 {
                items.push(e.mem.u32(node));
                node = e.mem.u32(node + 4);
            }
            items
        };
        assert_eq!(entries.len(), 2);
        let texts = doubles.strings.borrow();
        for (entry, name) in [(entries[1], "other"), (entries[0], "icon")] {
            assert!(texts.contains(&(entry, "Interface\\Shared\\atlas.txt".to_string())));
            assert!(texts.contains(&(entry + 8, name.to_string())));
        }
        assert_eq!(e.mem.f32(entries[1] + 0x1c), 4.0);
        assert_eq!(e.mem.f32(entries[1] + 0x24), 6.0);
        // The file is deleted at the end.
        assert_eq!(doubles.deleted.borrow().len(), 1);
    }

    #[test]
    fn an_atlas_file_that_cannot_be_opened_adds_nothing() {
        let mut e = tile_engine();
        let doubles = install_atlas_doubles(&mut e, &["icon icons.dds 0 0 1 2 0 3 4"]);
        doubles.opens.set(false);
        let atlas = Ptr::<()>::new(cstring(&mut e, "a.txt"));
        let subtexture = Ptr::<()>::new(cstring(&mut e, "icon"));
        let rect = Ptr::<()>::new(e.mem.alloc(16));
        tile_get_texture_atlas_info(&mut e, atlas, subtexture, Ptr::NULL, rect);
        assert_eq!(e.mem.u32(TEXTURE_ENTRY_LIST), 0);
        assert_eq!(doubles.deleted.borrow().len(), 1);
        assert!(doubles.strings.borrow().is_empty());
    }

    // --- Tile::Value --------------------------------------------------------

    /// A trait value of `tile` with the given actions (type, amount).
    fn value_with(
        e: &mut Engine,
        tile: Ptr<Tile>,
        trait_id: i32,
        start: f32,
        actions: &[(i32, f32)],
    ) -> Ptr<TileValue> {
        let value: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(value, TileValue::eIndex, trait_id);
        e.set(value, TileValue::pParent, tile);
        e.set(value, TileValue::fValue, start);
        let mut next = 0;
        for &(kind, amount) in actions.iter().rev() {
            let vtable = action_vtable(e, amount, 0);
            next = make_action(e, vtable, kind, next);
        }
        e.set(value, TileValue::pActionListA, Ptr::new(next));
        value
    }

    fn install_float_stack(e: &mut Engine) {
        e.register(FLOAT_STACK_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(64);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            a[0].into_ret()
        });
        e.register(FLOAT_STACK_PUSH, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let value = e.mem.f32(a[1]);
            e.mem.set_f32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
    }

    #[test]
    fn adding_actions_appends_them_to_the_trait() {
        let mut e = tile_engine();
        let value: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        value_add_action(&mut e, value, VA_ADD, 2.5);
        let first = e.get(value, TileValue::pActionListA).addr();
        assert_eq!(e.mem.u32(first), VTABLE_FLOAT_ACTION);
        assert_eq!(e.mem.i32(first + 4), VA_ADD);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.f32(first + 0xc), 2.5);
        assert_eq!(e.mem.block_size(first), Some(0x10));
        value_add_action(&mut e, value, VA_MULT, 3.0);
        value_add_action(&mut e, value, VA_SUB, 1.0);
        let second = e.mem.u32(first + 8);
        let third = e.mem.u32(second + 8);
        assert_eq!(e.mem.i32(second + 4), VA_MULT);
        assert_eq!(e.mem.i32(third + 4), VA_SUB);
        assert_eq!(e.mem.u32(third + 8), 0);
        assert_eq!(e.get(value, TileValue::pActionListA).addr(), first);
    }

    #[test]
    fn adding_a_reference_action_creates_the_referenced_trait_and_a_reaction() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let tile = typed_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        let value = give_trait(&mut e, tile, TRAIT_X, 0.0);
        e.call_log = Some(vec![]);
        value_add_action_ov2(&mut e, value, VA_REF, other, TRAIT_Y);
        let referenced = tile_get_value(&mut e, other, TRAIT_Y);
        assert!(!referenced.is_null());
        let action = e.get(value, TileValue::pActionListA).addr();
        assert_eq!(e.mem.u32(action), VTABLE_REF_VALUE_ACTION);
        assert_eq!(e.mem.i32(action + 4), VA_REF);
        assert_eq!(e.mem.u32(action + 0xc), referenced.addr());
        assert_eq!(
            calls_to(&e, ADD_REACTION),
            vec![vec![referenced.addr(), value.addr()]]
        );
        // A second action goes behind the first.
        value_add_action_ov2(&mut e, value, VA_COPY, other, TRAIT_Y);
        assert_eq!(e.mem.i32(e.mem.u32(action + 8) + 4), VA_COPY);
    }

    /// A reaction list `[owner values]` for the map doubles: nodes
    /// (value, next).
    fn reaction_list(e: &mut Engine, values: &[u32]) -> u32 {
        let mut next = 0;
        for &value in values.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, value);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn clearing_actions_takes_the_value_off_the_reaction_lists() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let this = value_with(&mut e, tile, TRAIT_X, 0.0, &[]);
        let referenced: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        let stranger = value_with(&mut e, tile, TRAIT_Y, 0.0, &[]);
        // Two reference actions on the same trait, one plain action.
        let reference = action_vtable(&mut e, 0.0, referenced.addr());
        let plain = action_vtable(&mut e, 1.0, 0);
        let third = make_action(&mut e, reference, VA_REF, 0);
        let second = make_action(&mut e, plain, VA_COPY, third);
        let first = make_action(&mut e, reference, VA_REF, second);
        e.set(this, TileValue::pActionListA, Ptr::new(first));
        // The referenced trait reacts for `this`, `stranger` and `this`.
        let list = reaction_list(&mut e, &[this.addr(), stranger.addr(), this.addr()]);
        let nodes: Vec<u32> = {
            let mut items = vec![];
            let mut node = list;
            while node != 0 {
                items.push(node);
                node = e.mem.u32(node + 4);
            }
            items
        };
        let head = Rc::new(Cell::new(list));
        let seen = head.clone();
        e.register_double(REACTION_MAP_FIND, move |e, a| {
            assert_eq!((a[0], a[1]), (REACTION_MAP, referenced.addr()));
            e.mem.set_u32(a[2], seen.get());
            1u32.into_ret()
        });
        let updated = Rc::new(RefCell::new(vec![]));
        let seen = updated.clone();
        let new_head = head.clone();
        e.register_double(REACTION_MAP_SET, move |_, a| {
            new_head.set(a[2]);
            seen.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        value_clear_actions(&mut e, this);
        // The first and last reactions are gone (freed); the stranger's
        // stays and becomes the head.
        assert_eq!(
            *updated.borrow(),
            vec![(REACTION_MAP, referenced.addr(), nodes[1])]
        );
        assert_eq!(e.mem.block_size(nodes[0]), None);
        assert_eq!(e.mem.block_size(nodes[2]), None);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        // All actions are freed and the list is empty.
        for action in [first, second, third] {
            assert_eq!(e.mem.block_size(action), None);
        }
        assert!(e.get(this, TileValue::pActionListA).is_null());
        // When nothing is left the map entry is erased instead.
        let only = value_with(&mut e, tile, TRAIT_Y, 0.0, &[]);
        let action = make_action(&mut e, reference, VA_REF, 0);
        e.set(only, TileValue::pActionListA, Ptr::new(action));
        let single = reaction_list(&mut e, &[only.addr()]);
        head.set(single);
        e.call_log = Some(vec![]);
        value_clear_actions(&mut e, only);
        assert_eq!(
            calls_to(&e, REACTION_MAP_REMOVE),
            vec![vec![REACTION_MAP, referenced.addr()]]
        );
        // A value without actions touches nothing.
        e.call_log = Some(vec![]);
        value_clear_actions(&mut e, only);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn destroying_a_value_blanks_the_references_other_values_hold_to_it() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let this = value_with(&mut e, tile, TRAIT_X, 4.5, &[]);
        let text = cstring(&mut e, "text");
        e.set(this, TileValue::strValue, Ptr::new(text));
        // Another value reads `this` through a reference action.
        let owner = value_with(&mut e, tile, TRAIT_Y, 0.0, &[]);
        let reference = action_vtable(&mut e, 0.0, this.addr());
        let elsewhere = action_vtable(&mut e, 0.0, owner.addr());
        let kept = make_action(&mut e, elsewhere, VA_REF, 0);
        let pointing = make_action(&mut e, reference, VA_REF, kept);
        e.mem.set_u32(pointing + 0xc, this.addr());
        e.mem.set_u32(kept + 0xc, owner.addr());
        e.set(owner, TileValue::pActionListA, Ptr::new(pointing));
        let reactions = reaction_list(&mut e, &[owner.addr()]);
        e.register_double(REACTION_MAP_FIND, move |e, a| {
            if a[1] == this.addr() {
                e.mem.set_u32(a[2], reactions);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        value_destructor(&mut e, this);
        assert_eq!(e.mem.u32(pointing + 0xc), 0);
        assert_eq!(e.mem.u32(kept + 0xc), owner.addr());
        assert_eq!(e.mem.block_size(reactions), None);
        assert_eq!(
            calls_to(&e, REACTION_MAP_REMOVE),
            vec![vec![REACTION_MAP, this.addr()]]
        );
        assert_eq!(e.mem.block_size(text), None);
        assert_eq!(e.get(this, TileValue::fValue), 0.0);
        assert!(e.get(this, TileValue::pParent).is_null());
    }

    #[test]
    fn calculating_runs_the_operations_in_order() {
        let calculate = |actions: &[(i32, f32)], start: f32| -> f32 {
            let mut e = tile_engine();
            provide_constants(&mut e);
            install_float_stack(&mut e);
            install_double_math(&mut e);
            let (tile, _) = updating_tile(&mut e, TYPE_RECT);
            let value = value_with(&mut e, tile, TRAIT_X, start, actions);
            value_calculate_value(&mut e, value, false);
            e.get(value, TileValue::fValue)
        };
        let table: &[(i32, f32, f32, f32)] = &[
            (VA_COPY, 1.0, 7.0, 7.0),
            (VA_ADD, 10.0, 5.0, 15.0),
            (VA_SUB, 10.0, 4.0, 6.0),
            (VA_MULT, 10.0, 3.0, 30.0),
            (VA_DIV, 10.0, 4.0, 2.5),
            (VA_DIV, 10.0, 0.0, 10.0),
            (VA_MIN, 10.0, 4.0, 4.0),
            (VA_MIN, 3.0, 4.0, 3.0),
            (VA_MAX, 10.0, 40.0, 40.0),
            (VA_MAX, 50.0, 40.0, 50.0),
            (VA_MOD, 13.0, 5.0, 3.0),
            (VA_MOD, 13.4, 5.6, 1.0),
            (VA_MOD, 13.0, 0.0, 13.0),
            (VA_FLOOR, -3.5, 1.0, -3.0),
            (VA_FLOOR, 1.25, 1.0, 2.0),
            (VA_CEIL, 1.25, 1.0, 3.0),
            (VA_ABS, -3.5, 1.0, 2.5),
            (VA_ROUND, 7.3, 2.0, 8.0),
            (VA_ROUND, 6.9, 2.0, 6.0),
            (VA_GT, 5.0, 3.0, 1.0),
            (VA_GT, 3.0, 3.0, 0.0),
            (VA_GTE, 3.0, 3.0, 1.0),
            (VA_GTE, 2.0, 3.0, 0.0),
            (VA_EQ, 3.0, 3.0, 1.0),
            (VA_EQ, 3.0, 4.0, 0.0),
            (VA_NEQ, 3.0, 4.0, 1.0),
            (VA_NEQ, 3.0, 3.0, 0.0),
            (VA_LT, 3.0, 5.0, 1.0),
            (VA_LT, 5.0, 5.0, 0.0),
            (VA_LTE, 5.0, 5.0, 1.0),
            (VA_LTE, 6.0, 5.0, 0.0),
            (VA_AND, 2.0, 3.0, 1.0),
            (VA_AND, 0.0, 3.0, 0.0),
            (VA_AND, 2.0, 0.0, 0.0),
            (VA_OR, 0.0, 0.0, 0.0),
            (VA_OR, 0.0, 2.0, 1.0),
            (VA_OR, 3.0, 0.0, 1.0),
            (VA_NOT, 9.0, 0.0, 1.0),
            (VA_NOT, 9.0, 5.0, 0.0),
            (VA_ONLYIF, 9.0, 0.0, 0.0),
            (VA_ONLYIF, 9.0, 1.0, 9.0),
            (VA_ONLYIFNOT, 9.0, 1.0, 0.0),
            (VA_ONLYIFNOT, 9.0, 0.0, 9.0),
            (9999, 9.0, 1.0, 9.0),
        ];
        for &(kind, start, amount, expected) in table {
            assert_eq!(
                calculate(&[(kind, amount)], start),
                expected,
                "{kind} {start} {amount}"
            );
        }
        // The actions run one after the other.
        assert_eq!(
            calculate(&[(VA_ADD, 5.0), (VA_MULT, 2.0), (VA_SUB, 6.0)], 10.0),
            24.0
        );
    }

    #[test]
    fn parentheses_calculate_the_inside_first_and_apply_the_operation_after() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        // 1 + 2 = 3; "(" saves 3 and restarts at 0; + 4 = 4; ")" restores
        // 3 and multiplies it by the 4 (the operation is the parenthesis's
        // own float, `VA_MULT`).
        let value = value_with(
            &mut e,
            tile,
            TRAIT_X,
            1.0,
            &[
                (VA_ADD, 2.0),
                (ACTION_LEFT_PAREN, 0.0),
                (VA_ADD, 4.0),
                (ACTION_RIGHT_PAREN, VA_MULT as f32),
            ],
        );
        value_calculate_value(&mut e, value, false);
        assert_eq!(e.get(value, TileValue::fValue), 12.0);
    }

    #[test]
    fn copying_a_text_replaces_the_string_and_counts_as_a_change() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        install_strcmp(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        let value = value_with(&mut e, tile, TRAIT_X, 0.0, &[]);
        let source: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        let text = cstring(&mut e, "hello");
        e.set(source, TileValue::strValue, Ptr::new(text));
        e.set(source, TileValue::eIndex, 5);
        let reference = action_vtable(&mut e, 0.0, source.addr());
        let action = make_action(&mut e, reference, VA_COPY, 0);
        e.set(value, TileValue::pActionListA, Ptr::new(action));
        e.call_log = Some(vec![]);
        value_calculate_value(&mut e, value, false);
        let copy = e.get(value, TileValue::strValue).addr();
        assert_ne!(copy, text);
        assert_eq!(string_at(&e, copy), "hello");
        assert_eq!(calls_to(&e, VALUE_CHANGE_EVENT), vec![vec![value.addr()]]);
        assert_eq!(*log.post_parses.borrow(), vec![(TRAIT_X, 0.0, copy)]);
        // The same text again: not a change, no event.
        e.call_log = Some(vec![]);
        value_calculate_value(&mut e, value, false);
        assert!(calls_to(&e, VALUE_CHANGE_EVENT).is_empty());
        assert_eq!(
            string_at(&e, e.get(value, TileValue::strValue).addr()),
            "hello"
        );
        // A text-less copy still announces when asked to.
        let plain = value_with(&mut e, tile, TRAIT_X, 2.0, &[]);
        e.set(plain, TileValue::strValue, Ptr::new(text));
        value_calculate_value(&mut e, plain, true);
        assert_eq!(calls_to(&e, VALUE_CHANGE_EVENT).len(), 1);
    }

    #[test]
    fn a_changed_value_is_announced_to_its_tile() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        let value = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_ADD, 2.0)]);
        let line_word = e.tls() + TLS_CURRENT_LINE;
        e.mem.set_u32(line_word, 0x77);
        e.call_log = Some(vec![]);
        value_calculate_value(&mut e, value, false);
        assert_eq!(calls_to(&e, VALUE_CHANGE_EVENT), vec![vec![value.addr()]]);
        assert_eq!(*log.post_parses.borrow(), vec![(TRAIT_X, 3.0, 0)]);
        assert_eq!(e.mem.u32(line_word), 0x77);
        assert_eq!(calls_to(&e, FLOAT_STACK_DESTROY).len(), 1);
        // No change and no flag: silence. With the flag: announced anyway.
        e.call_log = Some(vec![]);
        let steady = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_ADD, 0.0)]);
        value_calculate_value(&mut e, steady, false);
        assert!(calls_to(&e, VALUE_CHANGE_EVENT).is_empty());
        value_calculate_value(&mut e, steady, true);
        assert_eq!(calls_to(&e, VALUE_CHANGE_EVENT).len(), 1);
        // A released tile gets the event but is not asked.
        log.post_parses.borrow_mut().clear();
        e.set(tile, Tile::uiFlags, FLAG_RELEASED);
        value_calculate_value(&mut e, steady, true);
        assert!(log.post_parses.borrow().is_empty());
    }

    #[test]
    fn a_value_the_tile_does_not_post_parse_goes_to_final_post_parse() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        install_value_array_doubles(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        log.post_parse_answer.set(0);
        let updates = record_updates(&mut e);
        let value = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_ADD, 2.0)]);
        value_calculate_value(&mut e, value, false);
        // `FinalPostParse` of a position trait queues a position update.
        assert_eq!(*updates.borrow(), vec![(tile.addr(), UPDATE_POSITION)]);
    }

    #[test]
    fn a_tile_that_is_loading_or_deleting_is_not_calculated() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        let value = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_ADD, 2.0)]);
        e.set(tile, Tile::uiFlags, FLAG_TILE_LOADING);
        value_calculate_value(&mut e, value, false);
        assert_eq!(e.get(value, TileValue::fValue), 1.0);
        // Except for the class trait, which is always calculated.
        let class = value_with(&mut e, tile, TRAIT_CLASS, 1.0, &[(VA_ADD, 2.0)]);
        value_calculate_value(&mut e, class, false);
        assert_eq!(e.get(class, TileValue::fValue), 3.0);
        e.set(tile, Tile::uiFlags, FLAG_MENU_DELETING);
        value_calculate_value(&mut e, class, false);
        assert_eq!(e.get(class, TileValue::fValue), 3.0);
        // Without a tile the (tile-less) value is not calculated either.
        let orphan = value_with(&mut e, Ptr::NULL, TRAIT_CLASS, 1.0, &[(VA_ADD, 2.0)]);
        e.mem.map(0, 0x1000);
        value_calculate_value(&mut e, orphan, false);
        assert_eq!(e.get(orphan, TileValue::fValue), 1.0);
        let _ = log;
    }

    #[test]
    fn a_string_trait_shows_the_rounded_integer() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        e.register(SPRINTF_S, |e, a| {
            let text = format!("{}", a[3] as i32);
            assert!(text.len() < a[1] as usize);
            e.mem.set_cstr(a[0], text.as_bytes());
            0u32.into_ret()
        });
        let (tile, log) = updating_tile(&mut e, TYPE_RECT);
        let value = value_with(&mut e, tile, TRAIT_STRING, 0.0, &[(VA_ADD, 123.0)]);
        value_calculate_value(&mut e, value, false);
        let first = e.get(value, TileValue::strValue).addr();
        assert_eq!(string_at(&e, first), "123");
        assert!(e.mem.block_size(first).is_some());
        // The same number of characters keeps the buffer.
        let action = e.get(value, TileValue::pActionListA).addr();
        let vtable = action_vtable(&mut e, -168.0, 0);
        e.mem.set_u32(action, vtable);
        value_calculate_value(&mut e, value, false);
        assert_eq!(e.get(value, TileValue::fValue), -45.0);
        assert_eq!(e.get(value, TileValue::strValue).addr(), first);
        assert_eq!(string_at(&e, first), "-45");
        // A different length gets a new buffer; zero is one character.
        let vtable = action_vtable(&mut e, 45.0, 0);
        e.mem.set_u32(action, vtable);
        value_calculate_value(&mut e, value, false);
        let zero = e.get(value, TileValue::strValue).addr();
        assert_eq!(string_at(&e, zero), "0");
        assert_eq!(log.post_parses.borrow().len(), 3);
    }

    #[test]
    fn a_numbered_user_trait_reference_reads_the_underscore_value() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_float_stack(&mut e);
        let (tile, _) = updating_tile(&mut e, TYPE_RECT);
        let other = typed_tile(&mut e, TYPE_RECT);
        let value = value_with(&mut e, tile, TRAIT_X, 2.75, &[]);
        let referenced: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(referenced, TileValue::eIndex, 10005);
        e.set(referenced, TileValue::pParent, other);
        let reference = action_vtable(&mut e, 1.0, referenced.addr());
        let action = make_action(&mut e, reference, VA_ADD, 0);
        e.set(value, TileValue::pActionListA, Ptr::new(action));
        // The text table's number for the name is -1: look it up.
        e.register(REACTION_MAP_FIND, |e, a| {
            assert_eq!((a[0], a[1]), (TRAIT_EXTRA_DATA_MAP, 10005));
            e.mem.set_i32(a[2], -1);
            1u32.into_ret()
        });
        let found: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        e.set(found, TileValue::fValue, 9.0);
        e.register_double(TILE_GET_UNDERSCORE_VALUE, move |_, a| {
            assert_eq!((a[0], a[1] as i32, a[2] as i32), (other.addr(), 10005, 3));
            found.addr().into_ret()
        });
        value_calculate_value(&mut e, value, false);
        assert_eq!(e.get(value, TileValue::fValue), 11.75);
        // Nothing found: the action's own amount is used.
        e.register(TILE_GET_UNDERSCORE_VALUE, |_, _| 0u32.into_ret());
        e.set(value, TileValue::fValue, 2.0);
        value_calculate_value(&mut e, value, false);
        assert_eq!(e.get(value, TileValue::fValue), 3.0);
    }

    // --- Session 3 ----------------------------------------------------------

    /// A tile whose values can calculate (constants, float stack and math
    /// doubles) and whose `PostParse` calls are logged.
    fn calculating_tile(e: &mut Engine) -> (Ptr<Tile>, TileLog) {
        provide_constants(e);
        install_float_stack(e);
        install_double_math(e);
        updating_tile(e, TYPE_RECT)
    }

    #[test]
    fn the_underscore_value_is_found_by_the_numbered_name() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        install_value_array_doubles(&mut e);
        e.register(TEXT_TABLE_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            let (key, value, next) = (e.mem.u32(node + 4), e.mem.u32(node + 8), e.mem.u32(node));
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let key = cstring(&mut e, "_Slot");
        let node = table_node(&mut e, 0, key, 7);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(buckets, node);
        e.set_global(TEXT_TABLE_BUCKET_COUNT, 1u32);
        e.set_global(TEXT_TABLE_BUCKETS, buckets);
        e.register(SPRINTF_S, |e, a| {
            assert_eq!((a[1], a[2]), (NAME_BUFFER_SIZE, UNDERSCORE_NAME_FORMAT));
            let text = format!("{}{}", string_at(e, a[3]), a[4] as i32);
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
        e.register(TEXT_TABLE_FIND, |e, a| {
            if e.mem.cstr(a[1]) == b"_Slot3" {
                e.mem.set_i32(a[2], 10003);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let tile = typed_tile(&mut e, TYPE_RECT);
        let value = give_trait(&mut e, tile, 10003, 2.5);
        assert_eq!(tile_get_underscore_value(&mut e, tile, 7, 3), value.cast());
        // No such numbered name: no value.
        assert!(tile_get_underscore_value(&mut e, tile, 7, 4).is_null());
        assert_lock_balanced(&e);
    }

    /// What the reaction map double was asked to store: (map, key, list).
    type StoredReactions = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// Doubles for the reaction map: it reports the list `head` for every
    /// key and records what is stored.
    fn install_reaction_map(e: &mut Engine) -> (Rc<Cell<u32>>, StoredReactions) {
        let head = Rc::new(Cell::new(0u32));
        let seen = head.clone();
        e.register_double(REACTION_MAP_FIND, move |e, a| {
            if seen.get() == 0 {
                0u32.into_ret()
            } else {
                e.mem.set_u32(a[2], seen.get());
                1u32.into_ret()
            }
        });
        let stored = Rc::new(RefCell::new(vec![]));
        let (record, list) = (stored.clone(), head.clone());
        e.register_double(REACTION_MAP_SET, move |_, a| {
            record.borrow_mut().push((a[0], a[1], a[2]));
            list.set(a[2]);
            Ret::default()
        });
        (head, stored)
    }

    #[test]
    fn a_reaction_goes_behind_the_last_one_unless_that_names_the_owner() {
        let mut e = tile_engine();
        let (_, stored) = install_reaction_map(&mut e);
        let value: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        let owner: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        let other: Ptr<TileValue> = Ptr::new(e.mem.alloc(0x14));
        // A value without a list gets a one-node list in the map.
        add_reaction(&mut e, value, owner);
        let first = stored.borrow()[0].2;
        assert_eq!(stored.borrow()[0], (REACTION_MAP, value.addr(), first));
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (owner.addr(), 0));
        assert_eq!(e.mem.block_size(first), Some(8));
        // Another owner goes at the end; the same owner twice does not.
        add_reaction(&mut e, value, other);
        let second = e.mem.u32(first + 4);
        assert_eq!(e.mem.u32(second), other.addr());
        add_reaction(&mut e, value, other);
        assert_eq!(e.mem.u32(second + 4), 0);
        // Only the last node counts: `owner` is first, so it is added again.
        add_reaction(&mut e, value, owner);
        let third = e.mem.u32(second + 4);
        assert_eq!((e.mem.u32(third), e.mem.u32(third + 4)), (owner.addr(), 0));
        assert_eq!(stored.borrow().len(), 1);
    }

    #[test]
    fn a_value_change_recalculates_the_values_that_react() {
        let mut e = tile_engine();
        let (tile, log) = calculating_tile(&mut e);
        let (head, _) = install_reaction_map(&mut e);
        let changed = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_COPY, 5.0)]);
        let also = value_with(&mut e, tile, TRAIT_Y, 1.0, &[(VA_COPY, 7.0)]);
        let source = value_with(&mut e, tile, TRAIT_ALPHA, 0.0, &[]);
        // Nothing reacts: nothing is recalculated.
        tile_value_change_event(&mut e, source);
        assert_eq!(e.get(changed, TileValue::fValue), 1.0);
        head.set(reaction_list(&mut e, &[changed.addr(), also.addr()]));
        tile_value_change_event(&mut e, source);
        assert_eq!(e.get(changed, TileValue::fValue), 5.0);
        assert_eq!(e.get(also, TileValue::fValue), 7.0);
        assert_eq!(
            *log.post_parses.borrow(),
            vec![(TRAIT_X, 5.0, 0), (TRAIT_Y, 7.0, 0)]
        );
    }

    #[test]
    fn setting_a_float_replaces_the_string_and_recalculates() {
        let mut e = tile_engine();
        let (tile, log) = calculating_tile(&mut e);
        let value = value_with(&mut e, tile, TRAIT_X, 1.0, &[(VA_ADD, 1.0)]);
        let text = cstring(&mut e, "s");
        e.set(value, TileValue::strValue, Ptr::new(text));
        value_set_float(&mut e, value, 3.0, false);
        // The action still ran on the new float.
        assert_eq!(e.get(value, TileValue::fValue), 4.0);
        assert!(e.get(value, TileValue::strValue).is_null());
        assert_eq!(e.mem.block_size(text), None);
        assert_eq!(*log.post_parses.borrow(), vec![(TRAIT_X, 4.0, 0)]);
        // The same float again changes nothing.
        let plain = value_with(&mut e, tile, TRAIT_Y, 2.0, &[]);
        log.post_parses.borrow_mut().clear();
        value_set_float(&mut e, plain, 2.0, false);
        assert!(log.post_parses.borrow().is_empty());
        // A different float is reported.
        value_set_float(&mut e, plain, 2.5, false);
        assert_eq!(*log.post_parses.borrow(), vec![(TRAIT_Y, 2.5, 0)]);
        // With the flag the actions are cleared and not run.
        let action = e.get(value, TileValue::pActionListA).addr();
        value_set_float(&mut e, value, 9.0, true);
        assert_eq!(e.get(value, TileValue::fValue), 9.0);
        assert!(e.get(value, TileValue::pActionListA).is_null());
        assert_eq!(e.mem.block_size(action), None);
    }

    #[test]
    fn setting_a_string_copies_it_and_zeroes_the_float() {
        let mut e = tile_engine();
        install_strcmp(&mut e);
        let (tile, log) = calculating_tile(&mut e);
        let value = value_with(&mut e, tile, TRAIT_X, 6.0, &[(VA_ADD, 1.0)]);
        let first = cstring(&mut e, "abc");
        value_set_string(&mut e, value, Ptr::new(first), false);
        let copy = e.get(value, TileValue::strValue);
        assert_ne!(copy.addr(), first);
        assert_eq!(string_at(&e, copy.addr()), "abc");
        // The float was zeroed and the action added to it.
        assert_eq!(e.get(value, TileValue::fValue), 1.0);
        assert_eq!(log.post_parses.borrow().len(), 1);
        assert_eq!(log.post_parses.borrow()[0], (TRAIT_X, 1.0, copy.addr()));
        // The same text again is no change: the old copy is replaced, and
        // the string is not reported again (the action still adds to 0).
        let same = cstring(&mut e, "abc");
        log.post_parses.borrow_mut().clear();
        value_set_string(&mut e, value, Ptr::new(same), false);
        assert_eq!(
            string_at(&e, e.get(value, TileValue::strValue).addr()),
            "abc"
        );
        // A null text drops the string.
        let old = e.get(value, TileValue::strValue).addr();
        value_set_string(&mut e, value, Ptr::NULL, true);
        assert!(e.get(value, TileValue::strValue).is_null());
        assert_eq!(e.mem.block_size(old), None);
        assert!(e.get(value, TileValue::pActionListA).is_null());
        assert_eq!(e.get(value, TileValue::fValue), 0.0);
    }

    /// The list, string and item doubles `AddPair` needs, the constants it
    /// compares with, and a template owned by a storage.
    fn pair_engine() -> (Engine, Ptr<TileTemplate>, Ptr<BuildStorage>) {
        let mut e = tile_engine();
        install_strcmp(&mut e);
        for page in [0x0101_1000, 0x0107_3000, 0x0109_b000] {
            e.map(page, 0x1000);
        }
        for (address, value) in [
            (TEMPLATE_MARKER_VALUE, 999.0f64),
            (TILE_TYPE_FIRST_DOUBLE, 901.0),
            (TILE_TYPE_LAST_DOUBLE, 908.0),
            (TRAIT_ID_FIRST_DOUBLE, 4001.0),
            (TRAIT_ID_LAST_DOUBLE, 4125.0),
            (ACTION_ID_FIRST_DOUBLE, 2000.0),
            (ACTION_ID_LAST_DOUBLE, 2025.0),
            (NO_TRAIT_VALUE, -2147483648.0),
        ] {
            e.set_global(address, value);
        }
        e.set_global(NO_VALUE_FLOAT, -2147483648.0f32);
        e.register(STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_ASSIGN, |e, a| {
            let text = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], text);
            Ret::default()
        });
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(12).into_ret());
        e.register(TEMPLATE_LIST_ADD_NODE_TAIL, |e, a| {
            let (list, node) = (a[0], a[1]);
            let tail = e.mem.u32(list + 4);
            e.mem.set_u32(node, 0);
            e.mem.set_u32(node + 4, tail);
            if tail == 0 {
                e.mem.set_u32(list, node);
            } else {
                e.mem.set_u32(tail, node);
            }
            e.mem.set_u32(list + 4, node);
            let count = e.mem.u32(list + 8);
            e.mem.set_u32(list + 8, count + 1);
            Ret::default()
        });
        e.register(TEMPLATE_LIST_POP_TAIL, |e, a| {
            let list = a[0];
            let tail = e.mem.u32(list + 4);
            let previous = e.mem.u32(tail + 4);
            e.mem.set_u32(list + 4, previous);
            if previous == 0 {
                e.mem.set_u32(list, 0);
            } else {
                e.mem.set_u32(previous, 0);
            }
            let count = e.mem.u32(list + 8);
            e.mem.set_u32(list + 8, count - 1);
            e.mem.u32(tail + 8).into_ret()
        });
        e.register(TEMPLATE_LIST_POP_HEAD, |e, a| {
            let list = a[0];
            let head = e.mem.u32(list);
            let next = e.mem.u32(head);
            e.mem.set_u32(list, next);
            if next == 0 {
                e.mem.set_u32(list + 4, 0);
            } else {
                e.mem.set_u32(next + 4, 0);
            }
            let count = e.mem.u32(list + 8);
            e.mem.set_u32(list + 8, count - 1);
            e.mem.u32(head + 8).into_ret()
        });
        e.register(TEMPLATE_ITEM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            e.mem.set_u32(a[0] + 0x10, a[4]);
            e.mem.set_u32(a[0] + 0x14, a[5]);
            a[0].into_ret()
        });
        e.register(SSCANF, |e, a| match string_at(e, a[0]).parse::<f32>() {
            Ok(number) => {
                e.mem.set_f32(a[2], number);
                1u32.into_ret()
            }
            Err(_) => 0u32.into_ret(),
        });
        e.register(TEXT_TABLE_FIND, |e, a| {
            let id = match e.mem.cstr(a[1]).as_slice() {
                b"alpha" => TRAIT_ALPHA,
                b"act" => 2001,
                b"bad" => 3000,
                b"_user" => 10005,
                _ => return 0u32.into_ret(),
            };
            e.mem.set_i32(a[2], id);
            1u32.into_ret()
        });
        // A fixed string handle: the text behind a reference count.
        e.register(NI_FIXED_STRING_CREATE, |e, a| {
            let text = e.mem.cstr(a[0]);
            let block = e.mem.alloc(8 + text.len() as u32 + 1);
            e.mem.set_u32(block, 1);
            e.mem.set_cstr(block + 8, &text);
            (block + 8).into_ret()
        });
        e.register(TEMPLATE_CONSTRUCT, |e, a| {
            // The name is kept as text here.
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(SUB_TEMPLATE_LIST_ADD_HEAD, |e, a| {
            let (list, template) = (a[0], e.mem.u32(a[1]));
            let old_item = e.mem.u32(list);
            if old_item != 0 {
                let node = e.mem.alloc(8);
                let old_next = e.mem.u32(list + 4);
                e.mem.set_u32(node, old_item);
                e.mem.set_u32(node + 4, old_next);
                e.mem.set_u32(list + 4, node);
            }
            e.mem.set_u32(list, template);
            Ret::default()
        });
        let empty = e.mem.alloc(8);
        e.set_global(EMPTY_FIXED_STRING, empty);
        let storage: Ptr<BuildStorage> = e.new_object();
        let template: Ptr<TileTemplate> = e.new_object();
        e.set(template, TileTemplate::pParent, storage);
        e.set(storage, BuildStorage::pTemplate, template);
        (e, template, storage)
    }

    /// The items of a template, oldest first: (command, number, text,
    /// trait number).
    fn template_items(e: &Engine, template: Ptr<TileTemplate>) -> Vec<(i32, f32, String, i32)> {
        let mut items = vec![];
        let mut node = e.get(template.at(TileTemplate::xList), NiTPointerList::m_pkHead);
        while node != 0 {
            let item = e.mem.u32(node + 8);
            let text = e.mem.u32(item + 8);
            items.push((
                e.mem.i32(item),
                e.mem.f32(item + 4),
                if text == 0 {
                    String::new()
                } else {
                    string_at(e, text)
                },
                e.mem.i32(item + 0x10),
            ));
            node = e.mem.u32(node);
        }
        items
    }

    fn add_pair_text(
        e: &mut Engine,
        template: Ptr<TileTemplate>,
        command: i32,
        text: &str,
        flag: bool,
    ) {
        let text = cstring(e, text);
        template_add_pair(e, template, command, Ptr::new(text), 5, flag);
    }

    #[test]
    fn a_trait_element_becomes_a_start_item_a_simple_trait_and_an_end() {
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, 0, "alpha", false);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_TRAIT_START, 4009.0, "alpha".into(), 4009)]
        );
        // `<alpha>5</alpha>`: the text is a number, the end folds it in.
        add_pair_text(&mut e, template, TAG_VALUE, "5", true);
        let items = template_items(&e, template);
        assert_eq!(items[1], (TAG_VALUE, 5.0, "".into(), 5));
        e.call_log = Some(vec![]);
        add_pair_text(&mut e, template, 1, "alpha", false);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_SIMPLE_TRAIT, 5.0, "".into(), 4009)]
        );
        // Two items went back to the pool (the text item and the end).
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 2);
        // A trait end without a value item is an item of its own.
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, 0, "alpha", false);
        add_pair_text(&mut e, template, 1, "alpha", false);
        let kinds: Vec<i32> = template_items(&e, template).iter().map(|i| i.0).collect();
        assert_eq!(kinds, vec![TI_TRAIT_START, TI_TRAIT_END]);
    }

    #[test]
    fn an_action_element_becomes_a_simple_action_or_a_bad_item() {
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, 0, "act", false);
        add_pair_text(&mut e, template, TAG_VALUE, "3", true);
        add_pair_text(&mut e, template, 1, "act", false);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_SIMPLE_ACTION, 3.0, "".into(), 2001)]
        );
        // A number outside both ranges is a bad item, with an error.
        let (mut e, template, _) = pair_engine();
        let item = |e: &mut Engine, command, number, text: &str, argument| {
            add_item(e, template, command, number, text, argument);
        };
        item(&mut e, TI_TRAIT_START, 3000.0, "bad", 3000);
        item(&mut e, TAG_VALUE, 1.0, "", 0);
        e.call_log = Some(vec![]);
        add_pair_text(&mut e, template, 1, "bad", false);
        assert_eq!(template_items(&e, template)[0].0, TI_BAD);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![MSG_BAD_TRAIT_OR_ACTION]]
        );
        // A user trait number from 10000 on is a simple trait.
        let (mut e, template, _) = pair_engine();
        let item = |e: &mut Engine, command, number, text: &str, argument| {
            add_item(e, template, command, number, text, argument);
        };
        item(&mut e, TI_ACTION_START, 10005.0, "_user", 10005);
        item(&mut e, TAG_VALUE, 1.0, "", 0);
        add_pair_text(&mut e, template, 1, "_user", true);
        assert_eq!(template_items(&e, template)[0].0, TI_SIMPLE_TRAIT);
    }

    #[test]
    fn a_name_after_the_marker_finds_or_makes_a_template() {
        let (mut e, template, storage) = pair_engine();
        add_item(&mut e, template, 0, 999.0, "", 0);
        e.call_log = Some(vec![]);
        add_pair_text(&mut e, template, TAG_NAME, "MyTemplate", true);
        let made = e.get(storage, BuildStorage::pCurrentTemplate);
        assert!(!made.is_null());
        assert_eq!(string_at(&e, e.mem.u32(made.addr())), "MyTemplate");
        assert_eq!(e.mem.u32(made.addr() + 4), storage.addr());
        assert_eq!(calls_to(&e, TEMPLATE_CONSTRUCT).len(), 1);
        // The marker item is gone and both items are back in the pool.
        assert!(template_items(&e, template).is_empty());
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 2);
        // The closing marker ends the definition.
        add_pair_text(&mut e, template, 1, "999", true);
        assert!(e.get(storage, BuildStorage::pCurrentTemplate).is_null());
        assert!(template_items(&e, template).is_empty());
        // The same name again finds the template.
        add_item(&mut e, template, 0, 999.0, "", 0);
        add_pair_text(&mut e, template, TAG_NAME, "MyTemplate", true);
        assert_eq!(e.get(storage, BuildStorage::pCurrentTemplate), made);
        assert_eq!(calls_to(&e, TEMPLATE_CONSTRUCT).len(), 1);
        // A definition inside a definition is an error.
        add_item(&mut e, template, 0, 999.0, "", 0);
        add_pair_text(&mut e, template, TAG_NAME, "Inner", true);
        assert_eq!(calls_to(&e, PRINT_ERROR), vec![vec![MSG_NESTED_TEMPLATES]]);
        assert_eq!(e.get(storage, BuildStorage::pCurrentTemplate), made);
        assert!(template_items(&e, template).is_empty());
        // A 999 end marker without a definition is an ordinary number item.
        e.set(storage, BuildStorage::pCurrentTemplate, Ptr::NULL);
        add_pair_text(&mut e, template, 1, "999", true);
        assert_eq!(template_items(&e, template).len(), 1);
    }

    #[test]
    fn a_name_after_a_tile_type_starts_the_tile() {
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, 0, "903", true);
        assert_eq!(template_items(&e, template)[0], (0, 903.0, "".into(), 903));
        add_pair_text(&mut e, template, TAG_NAME, "tile1", true);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_TILE_START, 903.0, "tile1".into(), 903)]
        );
        // The end of the tile.
        add_pair_text(&mut e, template, 1, "903", true);
        assert_eq!(template_items(&e, template)[1].0, TI_TILE_END);
        // A name on its own is a simple trait item for the name word.
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, TAG_NAME, "x", true);
        let items = template_items(&e, template);
        assert_eq!((items[0].0, items[0].2.as_str()), (TI_SIMPLE_TRAIT, "x"));
        assert_eq!(items[0].3, TAG_NAME);
    }

    #[test]
    fn a_trait_and_a_source_after_an_action_make_a_trait_link() {
        let (mut e, template, _) = pair_engine();
        add_item(&mut e, template, TI_ACTION_START, 2001.0, "", 2001);
        add_item(&mut e, template, TAG_SRC, 5.0, "src", 0);
        add_item(&mut e, template, TAG_TRAIT, 7.0, "tr", 0);
        add_pair_text(&mut e, template, 1, "act", false);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_TRAIT_LINK, 7.0, "src".into(), 2001)]
        );
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 3);
    }

    #[test]
    fn user_names_and_action_ends_are_numbered() {
        let (mut e, template, _) = pair_engine();
        e.register(TEXT_TABLE_FIND, |e, a| {
            e.mem.set_i32(a[2], 10005);
            1u32.into_ret()
        });
        add_pair_text(&mut e, template, 0, "_user", true);
        assert_eq!(
            template_items(&e, template),
            vec![(TI_TRAIT_START, 10005.0, "_user".into(), 10005)]
        );
        let (mut e, template, _) = pair_engine();
        add_pair_text(&mut e, template, 0, "act", false);
        add_pair_text(&mut e, template, 1, "act", false);
        let kinds: Vec<i32> = template_items(&e, template).iter().map(|i| i.0).collect();
        assert_eq!(kinds, vec![TI_ACTION_START, TI_ACTION_END]);
    }

    #[test]
    fn clearing_a_template_frees_its_items_and_their_strings() {
        let (mut e, template, _) = pair_engine();
        let first = add_item(&mut e, template, 0, 1.0, "a", 0);
        let second = add_item(&mut e, template, 1, 2.0, "b", 0);
        e.call_log = Some(vec![]);
        template_clear(&mut e, template);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![first.addr() + 8, 0, 0], vec![second.addr() + 8, 0, 0]]
        );
        assert_eq!(e.mem.block_size(first.addr()), None);
        assert_eq!(e.mem.block_size(second.addr()), None);
        assert_eq!(
            e.get(template.at(TileTemplate::xList), NiTPointerList::m_uiCount),
            0
        );
        // An empty template does nothing.
        e.call_log = Some(vec![]);
        template_clear(&mut e, template);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    /// Doubles for the extra data of a node: `extra` answers for `node`.
    fn extra_for(e: &mut Engine, node: u32, extra: u32) {
        e.register_double(NI_OBJECT_GET_EXTRA_DATA, move |_, a| {
            if a[0] == node {
                extra.into_ret()
            } else {
                0u32.into_ret()
            }
        });
    }

    #[test]
    fn destroying_extra_data_hands_the_tile_to_the_parent_nodes_tile() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        let parent_tile = typed_tile(&mut e, TYPE_RECT);
        // The node's parent node has extra data that names `parent_tile`.
        let parent_node = e.mem.alloc(0x40);
        let parent_extra = e.mem.alloc(0x14);
        e.mem.set_u32(parent_extra + 4, 1);
        e.mem.set_u32(parent_extra + 0xc, parent_tile.addr());
        extra_for(&mut e, parent_node, parent_extra);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x18, parent_node);
        let this: Ptr<Extra> = e.new_object();
        e.set(this, Extra::pTile, tile);
        e.set(this, Extra::pNode, Ptr::new(node));
        e.call_log = Some(vec![]);
        extra_destructor(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_EXTRA);
        assert_eq!(e.get(tile, Tile::pParent), parent_tile);
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![tile.addr() + 0x2c, 0]]
        );
        assert!(e.get(this, Extra::pTile).is_null());
        assert_eq!(calls_to(&e, NI_EXTRA_DATA_DESTROY), vec![vec![this.addr()]]);
        // Without a tile only the base destructor runs.
        e.call_log = Some(vec![]);
        extra_destructor(&mut e, this);
        assert!(calls_to(&e, NI_POINTER_ASSIGN).is_empty());
        assert_eq!(calls_to(&e, NI_EXTRA_DATA_DESTROY).len(), 1);
    }

    #[test]
    fn a_build_storage_starts_with_a_main_template() {
        let mut e = tile_engine();
        e.register(TEMPLATE_CONSTRUCT, |_, a| a[0].into_ret());
        let storage: Ptr<BuildStorage> = e.new_object();
        e.mem.set_u32(storage.addr() + 4, 0x1234);
        e.mem.set_u32(storage.addr() + 8, 0x5678);
        e.call_log = Some(vec![]);
        assert_eq!(build_storage_construct(&mut e, storage), storage);
        let template = e.get(storage, BuildStorage::pTemplate);
        assert!(e.mem.block_size(template.addr()).is_some());
        assert_eq!(
            calls_to(&e, TEMPLATE_CONSTRUCT),
            vec![vec![template.addr(), MAIN_TEMPLATE_NAME, storage.addr()]]
        );
        assert_eq!(e.mem.u32(storage.addr() + 4), 0);
        assert_eq!(e.mem.u32(storage.addr() + 8), 0);
        assert!(e.get(storage, BuildStorage::pCurrentTemplate).is_null());
        assert!(e.get(storage, BuildStorage::bDeleteTemplates));
    }

    #[test]
    fn a_build_storage_destroys_its_templates() {
        let mut e = tile_engine();
        let storage: Ptr<BuildStorage> = e.new_object();
        let main = e.mem.alloc(0x14);
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        e.set(storage, BuildStorage::pTemplate, Ptr::new(main));
        e.set(storage, BuildStorage::pCurrentTemplate, Ptr::new(first));
        // The embedded node holds `first`, one more node holds `second`.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(storage.addr() + 4, first);
        e.mem.set_u32(storage.addr() + 8, node);
        e.set(storage, BuildStorage::bDeleteTemplates, true);
        e.call_log = Some(vec![]);
        build_storage_destructor(&mut e, storage);
        assert_eq!(
            calls_to(&e, TEMPLATE_DESTROY),
            vec![vec![main], vec![first], vec![second]]
        );
        assert_eq!(e.mem.block_size(main), None);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.mem.block_size(second), None);
        assert!(e.get(storage, BuildStorage::pCurrentTemplate).is_null());
        assert_eq!(
            calls_to(&e, SIMPLE_LIST_REMOVE_ALL),
            vec![vec![storage.addr() + 4]; 2]
        );
        // Without the flag the sub-templates are left alone, and a storage
        // without a main template destroys nothing.
        let other: Ptr<BuildStorage> = e.new_object();
        let kept = e.mem.alloc(0x14);
        e.mem.set_u32(other.addr() + 4, kept);
        e.call_log = Some(vec![]);
        build_storage_destructor(&mut e, other);
        assert!(calls_to(&e, TEMPLATE_DESTROY).is_empty());
        assert!(e.mem.block_size(kept).is_some());
    }

    #[test]
    fn a_sub_template_is_made_and_listed() {
        let (mut e, _, storage) = pair_engine();
        let name = cstring(&mut e, "Menu");
        e.call_log = Some(vec![]);
        let made = fn_00a0ae70(&mut e, storage, Ptr::new(name));
        assert!(e.mem.block_size(made.addr()).is_some());
        assert_eq!(
            calls_to(&e, TEMPLATE_CONSTRUCT),
            vec![vec![made.addr(), name, storage.addr()]]
        );
        // The first node of the list now holds it.
        assert_eq!(e.mem.u32(storage.addr() + 4), made.addr());
    }

    #[test]
    fn a_sub_template_is_found_by_its_name() {
        let (mut e, _, storage) = pair_engine();
        let (one, two) = (cstring(&mut e, "One"), cstring(&mut e, "Two"));
        let first = fn_00a0ae70(&mut e, storage, Ptr::new(one));
        let second = fn_00a0ae70(&mut e, storage, Ptr::new(two));
        e.call_log = Some(vec![]);
        let lookup = |e: &mut Engine, text: &str| {
            let name = cstring(e, text);
            fn_00a0af10(e, storage, Ptr::new(name))
        };
        assert_eq!(lookup(&mut e, "One"), first);
        assert_eq!(lookup(&mut e, "Two"), second);
        assert!(lookup(&mut e, "Three").is_null());
        // Each lookup released the handle it made.
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 3);
        // A null name is the empty string's handle, which is not released.
        e.call_log = Some(vec![]);
        assert!(fn_00a0af10(&mut e, storage, Ptr::NULL).is_null());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    /// A model with one child (shape) and a property answering for type 3.
    fn model_with_shape(e: &mut Engine) -> (u32, u32, u32) {
        let model = e.mem.alloc(0xb0);
        let shape = e.mem.alloc(0x40);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, shape);
        e.mem.set_u32(model + 0xa0, children);
        e.mem.set_u16(model + 0xa6, 1);
        let property = e.mem.alloc(0x80);
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!((a[0], a[1]), (shape, 3));
            property.into_ret()
        });
        (model, shape, property)
    }

    /// A child tile of `kind` whose virtual slot +0x1c records its calls.
    fn releasing_tile(e: &mut Engine, kind: u32, log: &Rc<RefCell<Vec<u32>>>) -> Ptr<Tile> {
        let vtable = e.mem.alloc(0x40);
        let base = 0x7300_0000 + vtable;
        e.mem.set_u32(vtable + 0xc, base + 0xc);
        e.mem.set_u32(vtable + 0x1c, base + 0x1c);
        e.register_double(base + 0xc, move |_, _| kind.into_ret());
        let seen = log.clone();
        e.register_double(base + 0x1c, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        let tile: Ptr<Tile> = e.new_object();
        e.mem.set_u32(tile.addr(), vtable);
        tile
    }

    #[test]
    fn forcing_a_texture_release_clears_the_texture_and_asks_the_children() {
        let mut e = tile_engine();
        let released = Rc::new(RefCell::new(vec![]));
        let tile = releasing_tile(&mut e, TYPE_IMAGE, &released);
        let (model, _, property) = model_with_shape(&mut e);
        e.set(tile, Tile::spModel, Ptr::new(model));
        // One ordinary child and one that belongs to a menu.
        let plain = releasing_tile(&mut e, TYPE_RECT, &released);
        let (menu_tile, _) = make_menu_tile(&mut e, 1);
        add_child(&mut e, tile, plain);
        add_child(&mut e, tile, menu_tile);
        e.call_log = Some(vec![]);
        tile_force_texture_release(&mut e, tile);
        assert_eq!(calls_to(&e, SET_TILE_TEXTURE), vec![vec![property, 0]]);
        assert_eq!(e.get(tile, Tile::uiFlags), UPDATE_TEXTURE | FLAG_DIRTY);
        assert_eq!(*released.borrow(), vec![plain.addr()]);
        // A tile that is not an image tile keeps its texture.
        let rect = releasing_tile(&mut e, TYPE_RECT, &released);
        e.set(rect, Tile::spModel, Ptr::new(model));
        e.call_log = Some(vec![]);
        tile_force_texture_release(&mut e, rect);
        assert!(calls_to(&e, SET_TILE_TEXTURE).is_empty());
        // No model: no texture work.
        let bare = releasing_tile(&mut e, TYPE_IMAGE, &released);
        tile_force_texture_release(&mut e, bare);
        assert!(calls_to(&e, SET_TILE_TEXTURE).is_empty());
    }

    #[test]
    fn a_tile_sound_plays_when_the_menu_wants_it() {
        let mut e = tile_engine();
        e.map(0x011f_6000, 0x1000);
        install_value_array_doubles(&mut e);
        let (menu_tile, menu) = make_menu_tile(&mut e, 1);
        let child = typed_tile(&mut e, TYPE_RECT);
        add_child(&mut e, menu_tile, child);
        // The menu tile's parent is the root, whose parent is none.
        let root = typed_tile(&mut e, TYPE_RECT);
        e.set(menu_tile, Tile::pParent, root);
        let name = cstring(&mut e, "UIMenuOK");
        let value = give_trait(&mut e, child, TRAIT_STRING, 0.0);
        e.set(value, TileValue::strValue, Ptr::new(name));
        e.set_global(AUDIO_MANAGER, 0x4444_0000u32);
        let source = e.mem.alloc(12);
        e.mem.set_u32(source, 77);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u32(source + 8, 99);
        e.register_double(GET_SOUND_HANDLE_BY_NAME, move |_, _| source.into_ret());
        let played = Rc::new(RefCell::new(vec![]));
        let seen = played.clone();
        e.register_double(SOUND_HANDLE_PLAY, move |e, a| {
            seen.borrow_mut().push((
                e.mem.u32(a[0]),
                e.mem.u8(a[0] + 4),
                e.mem.u32(a[0] + 8),
                a[1],
            ));
            Ret::default()
        });
        // The menu does not ask for sounds yet (its word at +0x24 is 0).
        tile_play_tile_sound(&mut e, child, TRAIT_STRING);
        assert!(played.borrow().is_empty());
        e.mem.set_u32(menu.addr() + 0x24, 1);
        e.call_log = Some(vec![]);
        tile_play_tile_sound(&mut e, child, TRAIT_STRING);
        assert_eq!(*played.borrow(), vec![(77, 1, 99, 0)]);
        let lookup = &calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0];
        assert_eq!(
            (lookup[0], lookup[2], lookup[3]),
            (0x4444_0000, name, 0x121)
        );
        // An empty or missing name plays nothing.
        e.mem.set_u8(name, 0);
        tile_play_tile_sound(&mut e, child, TRAIT_STRING);
        tile_play_tile_sound(&mut e, child, TRAIT_ID);
        assert_eq!(played.borrow().len(), 1);
    }

    #[test]
    fn the_shader_property_is_the_first_childs() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_IMAGE);
        assert!(tile_get_shader_property(&mut e, tile).is_null());
        let (model, _, property) = model_with_shape(&mut e);
        e.set(tile, Tile::spModel, Ptr::new(model));
        assert_eq!(tile_get_shader_property(&mut e, tile).addr(), property);
        // A model without children has none.
        e.mem.set_u16(model + 0xa6, 0);
        assert!(tile_get_shader_property(&mut e, tile).is_null());
    }

    #[test]
    fn alpha_and_color_go_to_the_shader_property_of_each_geometry() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_IMAGE);
        let model = e.mem.alloc(0xb0);
        let children = e.mem.alloc(16);
        // Child 0 is no geometry, child 1 is missing, child 2 is a geometry.
        let property = e.mem.alloc(0x80);
        let mut vtables = vec![];
        for answer in [0u32, 1] {
            let vtable = e.mem.alloc(0x40);
            let function = 0x7400_0000 + vtable;
            e.mem.set_u32(vtable + 0x18, function);
            e.register_double(function, move |_, a| {
                if answer == 0 {
                    0u32.into_ret()
                } else {
                    a[0].into_ret()
                }
            });
            let object = e.mem.alloc(0x40);
            e.mem.set_u32(object, vtable);
            vtables.push(object);
        }
        e.mem.set_u32(children, vtables[0]);
        e.mem.set_u32(children + 4, 0);
        e.mem.set_u32(children + 8, vtables[1]);
        e.mem.set_u32(model + 0xa0, children);
        e.mem.set_u16(model + 0xa6, 3);
        let geometry = vtables[1];
        e.register_double(NI_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!((a[0], a[1]), (geometry, 3));
            property.into_ret()
        });
        let color = e.mem.alloc(16);
        for (slot, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.mem.set_f32(color + 4 * slot as u32, *value);
        }
        tile_set_alpha_and_color(&mut e, tile, Ptr::new(model), 0.5, Ptr::new(color));
        assert_eq!(e.mem.f32(property + 0x78), 0.5);
        assert_eq!(
            [0, 4, 8, 12].map(|offset| e.mem.f32(property + 0x68 + offset)),
            [0.1, 0.2, 0.3, 0.4]
        );
        // No model: nothing happens.
        tile_set_alpha_and_color(&mut e, tile, Ptr::NULL, 0.9, Ptr::new(color));
        assert_eq!(e.mem.f32(property + 0x78), 0.5);
    }

    /// Gives a tile the system colour trait and the update spy.
    fn tile_with_color(e: &mut Engine, color: f32) -> Ptr<Tile> {
        let tile = typed_tile(e, TYPE_RECT);
        give_trait(e, tile, TRAIT_SYSTEM_COLOR, color);
        tile
    }

    #[test]
    fn the_color_walk_updates_the_tiles_that_use_the_color() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        install_value_array_doubles(&mut e);
        let parent = tile_with_color(&mut e, 3.0);
        // A child without the trait inherits 3.0; one with another colour
        // does not use it.
        let inheriting = typed_tile(&mut e, TYPE_RECT);
        let other = tile_with_color(&mut e, 4.0);
        add_child(&mut e, parent, inheriting);
        add_child(&mut e, parent, other);
        fn_00a0b350(&mut e, parent, 3, 0.0);
        assert_eq!(e.get(parent, Tile::uiFlags), UPDATE_COLOR | FLAG_DIRTY);
        assert_eq!(e.get(inheriting, Tile::uiFlags), UPDATE_COLOR | FLAG_DIRTY);
        assert_eq!(e.get(other, Tile::uiFlags), 0);
        assert_lock_balanced(&e);
    }

    #[test]
    fn the_color_list_walk_updates_the_tiles_whose_color_is_listed() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        install_value_array_doubles(&mut e);
        let parent = tile_with_color(&mut e, 3.9);
        let inheriting = typed_tile(&mut e, TYPE_RECT);
        let other = tile_with_color(&mut e, 8.0);
        add_child(&mut e, parent, inheriting);
        add_child(&mut e, parent, other);
        // The list holds 2 and 3: 3.9 is truncated to 3.
        let second = e.mem.alloc(8);
        e.mem.set_i32(second, 3);
        let first = e.mem.alloc(8);
        e.mem.set_i32(first, 2);
        e.mem.set_u32(first + 4, second);
        fn_00a0b420(&mut e, parent, Ptr::new(first), 0.0);
        assert_eq!(e.get(parent, Tile::uiFlags), UPDATE_COLOR | FLAG_DIRTY);
        assert_eq!(e.get(inheriting, Tile::uiFlags), UPDATE_COLOR | FLAG_DIRTY);
        assert_eq!(e.get(other, Tile::uiFlags), 0);
        assert_lock_balanced(&e);
    }

    #[test]
    fn rotation_gives_the_children_the_axis_and_the_angle() {
        let mut e = tile_engine();
        install_value_array_doubles(&mut e);
        let sets = Rc::new(RefCell::new(vec![]));
        let seen = sets.clone();
        e.register_double(VALUE_SET_FLOAT, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.i32(a[0]), f32::from_bits(a[1]), a[2]));
            Ret::default()
        });
        let parent = typed_tile(&mut e, TYPE_RECT);
        for (id, value) in [
            (TRAIT_X, 10.0),
            (TRAIT_Y, 20.0),
            (TRAIT_ROTATE_AXIS_X, 3.0),
            (TRAIT_ROTATE_AXIS_Y, 4.0),
            (TRAIT_ROTATE_ANGLE, 0.5),
            (TRAIT_LOCUS, 1.0),
        ] {
            give_trait(&mut e, parent, id, value);
        }
        let child = typed_tile(&mut e, TYPE_RECT);
        give_trait(&mut e, child, TRAIT_X, 1.0);
        give_trait(&mut e, child, TRAIT_Y, 2.0);
        add_child(&mut e, parent, child);
        // A child entry without a tile is skipped.
        push_list(&mut e, parent.at(Tile::xChildren), 0);
        tile_recursive_rotation_update(&mut e, parent);
        // The parent has a locus, so the child's position counts the
        // parent's: (10 + 3) - (1 + 10) and (20 + 4) - (2 + 20).
        assert_eq!(
            *sets.borrow(),
            vec![
                (TRAIT_ROTATE_AXIS_X, 2.0, 1),
                (TRAIT_ROTATE_AXIS_Y, 2.0, 1),
                (TRAIT_ROTATE_ANGLE, 0.5, 1),
            ]
        );
    }

    #[test]
    fn rotation_turns_the_shapes_of_a_located_image_tile() {
        let mut e = tile_engine();
        provide_constants(&mut e);
        install_value_array_doubles(&mut e);
        e.register(VALUE_SET_FLOAT, |_, _| Ret::default());
        for (address, value) in [
            (DEFAULT_TRANSLATION, 1.0f32),
            (DEFAULT_TRANSLATION + 4, 2.0),
            (DEFAULT_TRANSLATION + 8, 3.0),
        ] {
            e.set_global(address, value);
        }
        e.set_global(DEPTH_SCALE, -0.5f64);
        // The matrix routine writes the identity with the angle in slot 0.
        e.register(MATRIX_MAKE_Y_ROTATION, |e, a| {
            for slot in 0..9u32 {
                e.mem
                    .set_f32(a[0] + slot * 4, if slot % 4 == 0 { 1.0 } else { 0.0 });
            }
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for axis in 0..3u32 {
                let value = e.mem.f32(a[2] + axis * 4) * 2.0;
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            for axis in 0..3u32 {
                let value = e.mem.f32(a[0] + axis * 4) + e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for axis in 0..3u32 {
                let value = e.mem.f32(a[0] + axis * 4) - e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            a[1].into_ret()
        });
        let parent = typed_tile(&mut e, TYPE_RECT);
        let child = typed_tile(&mut e, TYPE_IMAGE);
        for (id, value) in [
            (TRAIT_X, 10.0),
            (TRAIT_Y, 20.0),
            (TRAIT_DEPTH, 4.0),
            (TRAIT_ROTATE_AXIS_X, 5.0),
            (TRAIT_ROTATE_AXIS_Y, 6.0),
            (TRAIT_ROTATE_ANGLE, 1.5),
            (TRAIT_LOCUS, 1.0),
        ] {
            give_trait(&mut e, child, id, value);
        }
        add_child(&mut e, parent, child);
        // The model has two children: a shape (its RTTI chain holds the
        // shape record) and another object.
        let model = e.mem.alloc(0xb0);
        let array = e.mem.alloc(8);
        e.set(child, Tile::spModel, Ptr::new(model));
        e.mem.set_u32(model + 0xa0, array);
        e.mem.set_u16(model + 0xa8, 2);
        let mut objects = vec![];
        for in_chain in [true, false] {
            let record = e.mem.alloc(8);
            let rtti = if in_chain { SHAPE_RTTI } else { 0 };
            e.mem.set_u32(record + 4, rtti);
            let vtable = e.mem.alloc(0x10);
            let function = 0x7500_0000 + vtable;
            e.mem.set_u32(vtable + 8, function);
            e.register_double(function, move |_, _| record.into_ret());
            let object = e.mem.alloc(0x80);
            e.mem.set_u32(object, vtable);
            objects.push(object);
        }
        e.mem.set_u32(array, objects[0]);
        e.mem.set_u32(array + 4, objects[1]);
        tile_recursive_rotation_update(&mut e, parent);
        let shape = objects[0];
        // The local rotation is the matrix the routine wrote (angle bits in
        // slot 0).
        assert_eq!(e.mem.u32(shape + 0x34), 1.5f32.to_bits());
        // Translation: default + (x, depth * -0.5, -y) = (11, 0, -17); axis
        // (axis x, 0, -axis y) = (5, 0, -6); the doubles rotate by doubling:
        // (translation + axis) - 2 * axis = (6, 0, -11).
        let moved = [0x58, 0x5c, 0x60].map(|offset| e.mem.f32(shape + offset));
        assert_eq!(moved, [6.0, 0.0, -11.0]);
        // The other object is untouched; the model keeps the default
        // translation.
        assert_eq!(e.mem.u32(objects[1] + 0x34), 0);
        assert_eq!(
            [0x58, 0x5c, 0x60].map(|offset| e.mem.f32(model + offset)),
            [1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn the_item_pool_takes_ten_items_and_deletes_the_rest() {
        let mut e = tile_engine();
        e.call_log = Some(vec![]);
        let mut items = vec![];
        for _ in 0..11 {
            let item = e.mem.alloc(0x18);
            items.push(item);
            fn_00a0b8d0(&mut e, Ptr::new(item));
        }
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 10);
        for (slot, item) in items.iter().take(10).enumerate() {
            assert_eq!(e.global::<u32>(ITEM_POOL + slot as u32 * 4), *item);
            assert_eq!(e.mem.block_size(*item), Some(0x18));
        }
        // The eleventh had its string freed and was deleted.
        assert_eq!(e.mem.block_size(items[10]), None);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![items[10] + 8, 0, 0]]);
        // A null item with a full pool is ignored.
        fn_00a0b8d0(&mut e, Ptr::NULL);
        assert_eq!(calls_to(&e, STRING_SET).len(), 1);
    }

    #[test]
    fn a_template_item_comes_from_the_pool_or_the_constructor() {
        let mut e = tile_engine();
        e.register(TEMPLATE_ITEM_CONSTRUCT, |_, a| a[0].into_ret());
        let text = cstring(&mut e, "t");
        e.call_log = Some(vec![]);
        // An empty pool: a new 0x18-byte item is built.
        let built = fn_00a0b950(&mut e, 4, 2.5, Ptr::new(text), 9, 33);
        assert_eq!(e.mem.block_size(built.addr()), Some(0x18));
        assert_eq!(
            calls_to(&e, TEMPLATE_ITEM_CONSTRUCT),
            vec![vec![built.addr(), 4, 2.5f32.to_bits(), text, 9, 33]]
        );
        // The pool's last item is refilled.
        let first = e.mem.alloc(0x18);
        let second = e.mem.alloc(0x18);
        fn_00a0b8d0(&mut e, Ptr::new(first));
        fn_00a0b8d0(&mut e, Ptr::new(second));
        e.call_log = Some(vec![]);
        let item = fn_00a0b950(&mut e, 5, 1.5, Ptr::new(text), 7, 8);
        assert_eq!(item.addr(), second);
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 1);
        assert_eq!(
            (
                e.get(item, TileTemplateItem::iCmd),
                e.get(item, TileTemplateItem::fVal),
                e.get(item, TileTemplateItem::u),
                e.get(item, TileTemplateItem::iLine)
            ),
            (5, 1.5, 7, 8)
        );
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![second + 8, text, 0]]);
        assert!(calls_to(&e, TEMPLATE_ITEM_CONSTRUCT).is_empty());
    }

    #[test]
    fn emptying_the_item_pool_deletes_every_item() {
        let mut e = tile_engine();
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x18)).collect();
        for item in &items {
            fn_00a0b8d0(&mut e, Ptr::new(*item));
        }
        // A null entry in the pool is skipped.
        fn_00a0b8d0(&mut e, Ptr::NULL);
        e.call_log = Some(vec![]);
        fn_00a0ba50(&mut e);
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 0);
        for item in &items {
            assert_eq!(e.mem.block_size(*item), None);
        }
        assert_eq!(calls_to(&e, STRING_SET).len(), 3);
        // Empty already: nothing to do.
        e.call_log = Some(vec![]);
        fn_00a0ba50(&mut e);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.global::<u32>(ITEM_POOL_COUNT), 0);
    }

    #[test]
    fn the_special_bounds_check_is_always_true() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        assert!(tile_special_bounds_check(&mut e, tile, 1, 2));
        assert!(e.call(0x00a0_bae0, &args![tile, 0u32, 0u32]).bool());
    }

    #[test]
    fn the_tile_method_at_00a0baf0_only_unlocks() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_RECT);
        e.call_log = Some(vec![]);
        fn_00a0baf0(&mut e, tile);
        assert_eq!(calls_to(&e, TILE_UNLOCK).len(), 1);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn a_user_name_starts_with_an_underscore_or_the_prefix() {
        let mut e = tile_engine();
        e.map(0x0109_3000, 0x1000);
        e.mem.write(USER_NAME_PREFIX, b"&_");
        let check = |e: &mut Engine, text: &str| {
            let name = cstring(e, text);
            fn_00a0bb00(e, Ptr::new(name))
        };
        assert!(check(&mut e, "_name"));
        assert!(check(&mut e, "&_name"));
        assert!(!check(&mut e, "name"));
        assert!(!check(&mut e, "&name"));
        assert!(!check(&mut e, ""));
        assert!(!fn_00a0bb00(&mut e, Ptr::NULL));
    }

    #[test]
    fn a_texture_atlas_entry_frees_its_three_strings() {
        let mut e = tile_engine();
        let entry = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        fn_00a0bb50(&mut e, Ptr::new(entry));
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![
                vec![entry + 0x10, 0, 0],
                vec![entry + 8, 0, 0],
                vec![entry, 0, 0]
            ]
        );
    }

    #[test]
    fn a_text_tile_answers_its_type_and_name() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_TEXT);
        assert_eq!(tile_text_get_type(&mut e, tile), TYPE_TEXT);
        assert_eq!(tile_text_get_type_name(&mut e, tile), 0x0109_48a0);
        assert_eq!(e.call(0x00a0_bbd0, &args![tile]).u32(), 0x387);
    }

    #[test]
    fn deleting_a_text_tile_releases_it_unless_it_was_released() {
        let mut e = tile_engine();
        let tile: Ptr<Tile> = e.new_object();
        e.set(tile, Tile::uiFlags, FLAG_RELEASED);
        e.call_log = Some(vec![]);
        // Released already: only the tile destructor runs, and the object
        // is deleted with bit 0 of the flags.
        assert_eq!(tile_text_scalar_deleting_destructor(&mut e, tile, 1), tile);
        assert_eq!(e.mem.block_size(tile.addr()), None);
        assert_eq!(calls_to(&e, INTERFACE_TILE_IS_BEING_DELETED).len(), 1);
        // Not released: the release step runs first (it takes the lock).
        let other: Ptr<Tile> = e.new_object();
        e.call_log = Some(vec![]);
        tile_text_scalar_deleting_destructor(&mut e, other, 0);
        assert_eq!(e.mem.block_size(other.addr()), Some(0x38));
        assert!(!calls_to(&e, TILE_LOCK).is_empty());
        assert_eq!(e.mem.u32(other.addr()), VTABLE_TILE);
    }

    #[test]
    fn a_3d_tile_starts_empty() {
        let mut e = tile_engine();
        let tile: Ptr<Tile> = Ptr::new(e.mem.alloc(0x50));
        for offset in (0x38..0x50).step_by(4) {
            e.mem.set_u32(tile.addr() + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(fn_00a0bc80(&mut e, tile), tile);
        assert_eq!(e.mem.u32(tile.addr()), VTABLE_TILE_3D);
        for offset in [0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c] {
            assert_eq!(e.mem.u32(tile.addr() + offset), 0, "{offset:#x}");
        }
        assert_eq!(calls_to(&e, TILE_BASE_CONSTRUCT), vec![vec![tile.addr()]]);
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN),
            vec![vec![tile.addr() + 0x2c, 0]]
        );
    }

    #[test]
    fn a_3d_tile_answers_its_type_and_name() {
        let mut e = tile_engine();
        let tile = typed_tile(&mut e, TYPE_3D);
        assert_eq!(tile_3d_get_type(&mut e, tile), TYPE_3D);
        assert_eq!(tile_3d_get_type_name(&mut e, tile), 0x0109_48d4);
    }

    #[test]
    fn deleting_a_3d_tile_runs_its_destructor_and_frees_it() {
        let mut e = tile_engine();
        let tile: Ptr<Tile> = e.new_object();
        e.call_log = Some(vec![]);
        tile_3d_scalar_deleting_destructor(&mut e, tile, 0);
        assert_eq!(e.mem.block_size(tile.addr()), Some(0x38));
        assert_eq!(calls_to(&e, TILE_3D_DESTROY), vec![vec![tile.addr()]]);
        assert_eq!(tile_3d_scalar_deleting_destructor(&mut e, tile, 1), tile);
        assert_eq!(e.mem.block_size(tile.addr()), None);
    }

    #[test]
    fn deleting_an_xml_storage_frees_its_data() {
        let mut e = tile_engine();
        let storage: Ptr<XmlStorage> = e.new_object();
        let data = e.mem.alloc(16);
        e.set(storage, XmlStorage::pXMLData, Ptr::new(data));
        xml_storage_scalar_deleting_destructor(&mut e, storage, 0);
        assert_eq!(e.mem.block_size(data), None);
        assert!(e.get(storage, XmlStorage::pXMLData).is_null());
        assert_eq!(e.mem.block_size(storage.addr()), Some(8));
        // With bit 0 the object goes too; no data is fine.
        xml_storage_scalar_deleting_destructor(&mut e, storage, 1);
        assert_eq!(e.mem.block_size(storage.addr()), None);
    }

    #[test]
    fn destroying_an_xml_storage_frees_the_data_only() {
        let mut e = tile_engine();
        let storage: Ptr<XmlStorage> = e.new_object();
        let data = e.mem.alloc(16);
        e.set(storage, XmlStorage::pXMLData, Ptr::new(data));
        xml_storage_destructor(&mut e, storage);
        assert_eq!(e.mem.block_size(data), None);
        assert!(e.get(storage, XmlStorage::pXMLData).is_null());
        assert_eq!(e.mem.block_size(storage.addr()), Some(8));
        xml_storage_destructor(&mut e, storage);
    }

    #[test]
    fn a_texture_atlas_entry_is_zeroed() {
        let mut e = tile_engine();
        let entry = e.mem.alloc(0x28);
        for offset in (0..0x28).step_by(4) {
            e.mem.set_u32(entry + offset, 0xdead_beef);
        }
        assert_eq!(fn_00a0beb0(&mut e, Ptr::new(entry)).addr(), entry);
        for offset in (0..0x28).step_by(4) {
            assert_eq!(e.mem.u32(entry + offset), 0, "{offset:#x}");
        }
    }

    #[test]
    fn the_actions_answer_their_operand() {
        let mut e = tile_engine();
        let action: Ptr<ValueAction> = e.new_object();
        e.set(action, ValueAction::fValue, 2.5);
        assert_eq!(float_action_q_float(&mut e, action), 2.5);
        // A reference action reads the other trait's float.
        let referenced: Ptr<TileValue> = e.new_object();
        e.set(referenced, TileValue::fValue, 7.0);
        let reference: Ptr<ValueAction> = e.new_object();
        assert_eq!(ref_value_action_q_float(&mut e, reference), 0.0);
        e.set(reference, ValueAction::pRefValue, referenced);
        assert_eq!(ref_value_action_q_float(&mut e, reference), 7.0);
        assert_eq!(fn_00a0c060(&mut e, reference), referenced);
    }

    #[test]
    fn the_extra_data_class_answers_its_rtti_record() {
        let mut e = tile_engine();
        assert_eq!(ni_extra_data_get_rtti(&mut e, Ptr::NULL), 0x011f_4a80);
    }

    // -----------------------------------------------------------------
    // Session 4: `Tile::Extra` deleting destructor to the map code.

    const MAP_VTABLE: u32 = 0x0600_5000;
    const ARRAY_VTABLE: u32 = 0x0600_6000;
    const ARRAY_ALLOCATE_DOUBLE: u32 = 0x0600_6104;
    const ARRAY_FREE_DOUBLE: u32 = 0x0600_6108;
    const ARRAY_REALLOCATE_DOUBLE: u32 = 0x0600_610c;
    const COMPARE_DOUBLE: u32 = 0x0600_6200;

    /// A tile engine for this batch: the pages of the globals, the string
    /// and memory helpers of the runtime as small Rust doubles.
    fn batch4_engine() -> Engine {
        let mut e = tile_engine();
        for page in [0x0109_b000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.set_global(STD_STRING_NPOS, u32::MAX);
        e.register(MEMMOVE, |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            a[0].into_ret()
        });
        e.register(MEMCMP, |e, a| {
            let (left, right) = (e.mem.bytes(a[0], a[2]), e.mem.bytes(a[1], a[2]));
            (left.cmp(&right) as i32).into_ret()
        });
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(STRICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            (left.cmp(&right) as i32).into_ret()
        });
        e.register(TOLOWER, |_, a| (a[0] as u8).to_ascii_lowercase().into_ret());
        e.register(ALLOC_BYTES, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_NEW_NODE, |e, _| zeroed_block(e, 12).into_ret());
        e
    }

    /// A heap block of `size` zero bytes.
    fn zeroed_block(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size.max(1));
        e.mem.write(block, &vec![0; size.max(1) as usize]);
        block
    }

    /// A `std::string` (0x1c bytes) holding `text`: inline when `capacity`
    /// is below 0x10, else in a heap buffer.
    fn make_std_string(e: &mut Engine, text: &str, capacity: u32) -> u32 {
        let string = zeroed_block(e, 0x1c);
        if capacity >= 0x10 {
            let buffer = zeroed_block(e, capacity + 1);
            e.mem.write(buffer, text.as_bytes());
            e.mem.set_u32(string + 4, buffer);
        } else {
            e.mem.write(string + 4, text.as_bytes());
        }
        e.mem.set_u32(string + 0x14, text.len() as u32);
        e.mem.set_u32(string + 0x18, capacity);
        string
    }

    fn std_string_text(e: &Engine, string: u32) -> String {
        let data = std_string_data(e, string);
        String::from_utf8(e.mem.cstr(data)).unwrap()
    }

    /// The runtime doubles the `std::string` appends call.
    fn install_append_doubles(e: &mut Engine) {
        e.register(STD_STRING_INSIDE, |e, a| {
            let data = std_string_data(e, a[0]);
            let size = e.mem.u32(a[0] + 0x14);
            (a[1] >= data && a[1] <= data + size).into_ret()
        });
        e.register(STD_STRING_GROW, |e, a| {
            (a[1] <= e.mem.u32(a[0] + 0x18)).into_ret()
        });
        e.register(MEMCPY_S, |e, a| {
            assert!(a[1] >= a[3], "destination too small");
            let data = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &data);
            0u32.into_ret()
        });
    }

    #[test]
    fn extra_scalar_deleting_destructor_deletes_with_bit_zero() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let extra: Ptr<Extra> = e.new_object();
        assert_eq!(extra_scalar_deleting_destructor(&mut e, extra, 0), extra);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA);
        assert!(calls_to(&e, NI_EXTRA_DATA_DESTROY).len() == 1);
        assert!(calls_to(&e, NI_OPERATOR_DELETE).is_empty());
        extra_scalar_deleting_destructor(&mut e, extra, 3);
        assert_eq!(
            calls_to(&e, NI_OPERATOR_DELETE),
            vec![vec![extra.addr(), 0x14]]
        );
    }

    /// A fixed string handle: the text behind a reference count of 1.
    fn install_fixed_strings(e: &mut Engine, empty: u32) {
        e.set_global(EMPTY_FIXED_STRING, empty);
        e.register(NI_FIXED_STRING_CREATE, |e, a| {
            let text = e.mem.cstr(a[0]);
            let block = e.mem.alloc(8 + text.len() as u32 + 1);
            e.mem.set_u32(block, 1);
            e.mem.set_cstr(block + 8, &text);
            (block + 8).into_ret()
        });
    }

    #[test]
    fn template_construct_names_the_template_and_empties_its_list() {
        let mut e = batch4_engine();
        install_fixed_strings(&mut e, 0x0600_7008);
        e.call_log = Some(vec![]);
        let template: Ptr<TileTemplate> = e.new_object();
        let storage: Ptr<BuildStorage> = e.new_object();
        let list = template.at(TileTemplate::xList);
        // Leftovers in the list fields are overwritten.
        e.set(list, NiTPointerList::m_pkHead, 0x1234);
        e.set(list, NiTPointerList::m_pkTail, 0x1234);
        e.set(list, NiTPointerList::m_uiCount, 7);
        let name = cstring(&mut e, "Prefab");
        assert_eq!(
            template_construct(&mut e, template, name, storage),
            template
        );
        let handle = e.get(template, TileTemplate::xName);
        assert_ne!(handle, 0x0600_7008);
        assert_eq!(string_at(&e, handle), "Prefab");
        assert_eq!(e.get(template, TileTemplate::pParent), storage);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), 0);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), 0);
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 0);
        assert_eq!(calls_to(&e, NI_FIXED_STRING_CREATE), vec![vec![name]]);
        // The previous handle was the empty string's: nothing is released.
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        // The name that is the empty handle itself is kept without a lookup.
        let again: Ptr<TileTemplate> = e.new_object();
        template_construct(&mut e, again, 0x0600_7008, storage);
        assert_eq!(e.get(again, TileTemplate::xName), 0x0600_7008);
        assert_eq!(calls_to(&e, NI_FIXED_STRING_CREATE).len(), 1);
    }

    #[test]
    fn template_destructor_clears_the_list_and_releases_the_name() {
        let mut e = batch4_engine();
        install_fixed_strings(&mut e, 0x0600_7008);
        e.call_log = Some(vec![]);
        let template: Ptr<TileTemplate> = e.new_object();
        let storage: Ptr<BuildStorage> = e.new_object();
        let name = cstring(&mut e, "Prefab");
        template_construct(&mut e, template, name, storage);
        let handle = e.get(template, TileTemplate::xName);
        // A second owner of the name keeps the text alive.
        e.mem.set_u32(handle - 8, 2);
        template_destructor(&mut e, template);
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(calls_to(&e, LIST_DESTROY), vec![vec![template.addr() + 8]]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![handle - 8]]);
        // The empty string's handle is not released.
        let plain: Ptr<TileTemplate> = e.new_object();
        e.set(plain, TileTemplate::xName, 0x0600_7008);
        template_destructor(&mut e, plain);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn template_item_construct_fills_the_item() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let item: Ptr<TileTemplateItem> = e.new_object();
        let string = item.at(TileTemplateItem::xStr);
        e.set(string, BSStringT::pString, 0x1111);
        e.set(string, BSStringT::sLen, 9);
        e.set(string, BSStringT::sMaxLen, 9);
        let text = cstring(&mut e, "hello");
        let built = fn_00a0c220(&mut e, item, -1, 2.5, Ptr::new(text), 4001, 42);
        assert_eq!(built, item);
        assert_eq!(e.get(item, TileTemplateItem::iCmd), -1);
        assert_eq!(e.get(item, TileTemplateItem::fVal), 2.5);
        assert_eq!(e.get(item, TileTemplateItem::u), 4001);
        assert_eq!(e.get(item, TileTemplateItem::iLine), 42);
        // The empty string was made before the text was assigned (the
        // assignment is a double here).
        assert_eq!(e.get(string, BSStringT::pString), 0);
        assert_eq!(e.get(string, BSStringT::sLen), 0);
        assert_eq!(e.get(string, BSStringT::sMaxLen), 0);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![item.addr() + 8, text, 0x1f]]
        );
    }

    #[test]
    fn string_rfind_finds_the_last_match_at_or_before_the_position() {
        let mut e = batch4_engine();
        let heap = make_std_string(&mut e, "abcabcabc-abc", 0x20);
        let needle = cstring(&mut e, "abc");
        let find = |e: &mut Engine, pos| fn_00a0c300(e, Ptr::new(heap), Ptr::new(needle), pos, 3);
        assert_eq!(find(&mut e, 100), 10);
        assert_eq!(find(&mut e, 10), 10);
        assert_eq!(find(&mut e, 9), 6);
        assert_eq!(find(&mut e, 5), 3);
        assert_eq!(find(&mut e, 0), 0);
        // No match: npos.
        let other = cstring(&mut e, "xyz");
        assert_eq!(
            fn_00a0c300(&mut e, Ptr::new(heap), Ptr::new(other), 100, 3),
            u32::MAX
        );
        // An empty needle answers the position limited to the length.
        assert_eq!(
            fn_00a0c300(&mut e, Ptr::new(heap), Ptr::new(needle), 4, 0),
            4
        );
        assert_eq!(
            fn_00a0c300(&mut e, Ptr::new(heap), Ptr::new(needle), 99, 0),
            13
        );
        // A needle longer than the string.
        let long = cstring(&mut e, "abcabcabc-abcd");
        assert_eq!(
            fn_00a0c300(&mut e, Ptr::new(heap), Ptr::new(long), 99, 14),
            u32::MAX
        );
        // A string held inline.
        let inline = make_std_string(&mut e, "hello", 15);
        let tail = cstring(&mut e, "lo");
        assert_eq!(
            fn_00a0c300(&mut e, Ptr::new(inline), Ptr::new(tail), 99, 2),
            3
        );
    }

    #[test]
    fn string_append_text_copies_grows_and_terminates() {
        let mut e = batch4_engine();
        install_append_doubles(&mut e);
        e.call_log = Some(vec![]);
        let string = make_std_string(&mut e, "abc", 0x20);
        let more = cstring(&mut e, "def!");
        assert_eq!(
            fn_00a0c430(&mut e, Ptr::new(string), Ptr::new(more), 3),
            Ptr::new(string)
        );
        assert_eq!(std_string_text(&e, string), "abcdef");
        assert_eq!(e.mem.u32(string + 0x14), 6);
        assert_eq!(calls_to(&e, STD_STRING_GROW), vec![vec![string, 6, 0]]);
        // Nothing to append: no growth.
        fn_00a0c430(&mut e, Ptr::new(string), Ptr::new(more), 0);
        assert_eq!(calls_to(&e, STD_STRING_GROW).len(), 1);
        // Text from the string itself is appended by range.
        let inner = std_string_data(&e, string) + 2;
        fn_00a0c430(&mut e, Ptr::new(string), Ptr::new(inner), 3);
        assert_eq!(std_string_text(&e, string), "abcdefcde");
        // A failed growth leaves the string alone.
        let small = make_std_string(&mut e, "ab", 2);
        fn_00a0c430(&mut e, Ptr::new(small), Ptr::new(more), 4);
        assert_eq!(std_string_text(&e, small), "ab");
        assert_eq!(e.mem.u32(small + 0x14), 2);
    }

    #[test]
    fn string_append_text_throws_length_error_when_too_long() {
        let mut e = batch4_engine();
        install_append_doubles(&mut e);
        e.register(STD_STRING_GROW, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        let string = make_std_string(&mut e, "abc", 0x20);
        let more = cstring(&mut e, "x");
        fn_00a0c430(&mut e, Ptr::new(string), Ptr::new(more), u32::MAX - 2);
        assert_eq!(calls_to(&e, STD_LENGTH_ERROR).len(), 1);
        fn_00a0c430(&mut e, Ptr::new(string), Ptr::new(more), 1);
        assert_eq!(calls_to(&e, STD_LENGTH_ERROR).len(), 1);
    }

    #[test]
    fn string_append_range_limits_the_count_and_checks_the_offset() {
        let mut e = batch4_engine();
        install_append_doubles(&mut e);
        e.call_log = Some(vec![]);
        let string = make_std_string(&mut e, "ab", 0x20);
        let other = make_std_string(&mut e, "xyz123", 0x20);
        fn_00a0c570(&mut e, Ptr::new(string), Ptr::new(other), 2, 3);
        assert_eq!(std_string_text(&e, string), "abz12");
        // The count is limited to what remains.
        fn_00a0c570(&mut e, Ptr::new(string), Ptr::new(other), 4, 10);
        assert_eq!(std_string_text(&e, string), "abz1223");
        assert_eq!(e.mem.u32(string + 0x14), 7);
        assert!(calls_to(&e, STD_OUT_OF_RANGE).is_empty());
        // An offset beyond the other string throws out-of-range.
        e.register(STD_STRING_GROW, |_, _| false.into_ret());
        fn_00a0c570(&mut e, Ptr::new(string), Ptr::new(other), 7, 1);
        assert_eq!(calls_to(&e, STD_OUT_OF_RANGE).len(), 1);
        // A result that cannot be counted throws length-error.
        e.mem.set_u32(string + 0x14, u32::MAX - 2);
        fn_00a0c570(&mut e, Ptr::new(string), Ptr::new(other), 0, 3);
        assert_eq!(calls_to(&e, STD_LENGTH_ERROR).len(), 1);
    }

    /// The vtable of a test hash map: the hash is the key modulo the bucket
    /// count, keys are equal when their low bytes are, the node's key and
    /// value are written, and a node is a zeroed 12-byte block.
    fn install_map_vtable(e: &mut Engine) {
        e.put_vtable(
            MAP_VTABLE,
            &[0, 0x0600_5104, 0x0600_5108, 0x0600_510c, 0, 0x0600_5114],
        );
        e.register(0x0600_5104, |e, a| {
            let buckets = e.mem.u32(a[0] + 4);
            (a[1] % buckets).into_ret()
        });
        e.register(0x0600_5108, |_, a| {
            ((a[1] & 0xff) == (a[2] & 0xff)).into_ret()
        });
        e.register(0x0600_510c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x0600_5114, |e, _| zeroed_block(e, 12).into_ret());
    }

    /// A hash map object: vtable, bucket count, empty buckets, no items.
    fn make_map(e: &mut Engine, buckets: u32, copy_keys: bool) -> Ptr<NiTPointerMap> {
        let map = zeroed_block(e, 0x14);
        let table = zeroed_block(e, buckets * 4);
        e.mem.set_u32(map, MAP_VTABLE);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u8(map + 0x10, copy_keys as u8);
        Ptr::new(map)
    }

    /// The `(key, value)` pairs of a bucket's chain, first node first.
    fn bucket_chain(e: &Engine, map: Ptr<NiTPointerMap>, bucket: u32) -> Vec<(u32, u32)> {
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let mut node = e.mem.u32(table + bucket * 4);
        let mut pairs = vec![];
        while node != 0 {
            pairs.push((e.mem.u32(node + 4), e.mem.u32(node + 8)));
            node = e.mem.u32(node);
        }
        pairs
    }

    #[test]
    fn map_set_inserts_at_the_bucket_head_or_updates() {
        let mut e = batch4_engine();
        install_map_vtable(&mut e);
        let map = make_map(&mut e, 4, false);
        fn_00a0c6c0(&mut e, map, 5, 50);
        fn_00a0c6c0(&mut e, map, 9, 90);
        fn_00a0c6c0(&mut e, map, 2, 20);
        assert_eq!(bucket_chain(&e, map, 1), vec![(9, 90), (5, 50)]);
        assert_eq!(bucket_chain(&e, map, 2), vec![(2, 20)]);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 3);
        // The same key (by the map's comparison) only gets the new value.
        fn_00a0c6c0(&mut e, map, 5, 55);
        assert_eq!(bucket_chain(&e, map, 1), vec![(9, 90), (5, 55)]);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 3);
    }

    #[test]
    fn map_set_with_key_copy_flag_keeps_or_replaces_the_key() {
        let mut e = batch4_engine();
        install_map_vtable(&mut e);
        e.call_log = Some(vec![]);
        let replacing = make_map(&mut e, 4, false);
        fn_00a0c900(&mut e, replacing, 5, 50);
        assert_eq!(calls_to(&e, LIST_NEW_NODE).len(), 1);
        // Key 0x105 equals key 5 in the test comparison and has its bucket.
        fn_00a0c900(&mut e, replacing, 0x105, 51);
        assert_eq!(bucket_chain(&e, replacing, 1), vec![(0x105, 51)]);
        assert_eq!(e.get(replacing, NiTPointerMap::m_uiCount), 1);
        let keeping = make_map(&mut e, 4, true);
        fn_00a0c900(&mut e, keeping, 5, 50);
        fn_00a0c900(&mut e, keeping, 0x105, 51);
        assert_eq!(bucket_chain(&e, keeping, 1), vec![(5, 51)]);
        // Two keys of one bucket chain up at the head.
        fn_00a0c900(&mut e, keeping, 9, 90);
        assert_eq!(bucket_chain(&e, keeping, 1), vec![(9, 90), (5, 51)]);
        assert_eq!(e.get(keeping, NiTPointerMap::m_uiCount), 2);
    }

    #[test]
    fn map_iteration_walks_chains_then_buckets() {
        let mut e = batch4_engine();
        install_map_vtable(&mut e);
        let map = make_map(&mut e, 4, false);
        for (key, value) in [(1, 10), (5, 50), (2, 20), (3, 30)] {
            fn_00a0c6c0(&mut e, map, key, value);
        }
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let slot = e.mem.alloc(12);
        let (key_out, value_out) = (slot + 4, slot + 8);
        e.mem.set_u32(slot, e.mem.u32(table + 4));
        let mut seen = vec![];
        while e.mem.u32(slot) != 0 {
            fn_00a0c850(&mut e, map, slot, key_out, value_out);
            seen.push((e.mem.u32(key_out), e.mem.u32(value_out)));
        }
        assert_eq!(seen, vec![(5, 50), (1, 10), (2, 20), (3, 30)]);
    }

    #[test]
    fn simple_list_empty_call_goes_to_the_shared_clear() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        fn_00a0c7a0(&mut e, Ptr::new(0x4321));
        assert_eq!(calls_to(&e, SIMPLE_LIST_REMOVE_ALL), vec![vec![0x4321]]);
    }

    /// A `BSSimpleList` head holding `items` (the first in the embedded
    /// node, the others in heap nodes).
    fn make_simple_list(e: &mut Engine, items: &[u32]) -> Ptr<BSSimpleList> {
        let head = zeroed_block(e, 8);
        let mut tail = head;
        for (index, item) in items.iter().enumerate() {
            let node = if index == 0 { head } else { zeroed_block(e, 8) };
            e.mem.set_u32(node, *item);
            if index > 0 {
                e.mem.set_u32(tail + 4, node);
            }
            tail = node;
        }
        Ptr::new(head)
    }

    fn simple_list_items(e: &Engine, list: Ptr<BSSimpleList>) -> Vec<u32> {
        let mut node = list.addr();
        let mut items = vec![];
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn fade_list_add_head_keeps_the_embedded_node_first() {
        let mut e = batch4_engine();
        let list = make_simple_list(&mut e, &[]);
        let slot = e.mem.alloc(4);
        // A null item is ignored.
        e.mem.set_u32(slot, 0);
        bs_simple_list_fade_control_add_head(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![0]);
        // An empty list takes the item in its embedded node.
        e.mem.set_u32(slot, 0xa1);
        bs_simple_list_fade_control_add_head(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![0xa1]);
        // Later items go first, the old first item moves to a new node.
        e.mem.set_u32(slot, 0xa2);
        bs_simple_list_fade_control_add_head(&mut e, list, Ptr::new(slot));
        e.mem.set_u32(slot, 0xa3);
        bs_simple_list_fade_control_add_head(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![0xa3, 0xa2, 0xa1]);
    }

    #[test]
    fn fade_list_remove_unlinks_destroys_and_frees() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let list = make_simple_list(&mut e, &[1, 2, 3]);
        let slot = e.mem.alloc(4);
        let second = e.get(list, BSSimpleList::m_pkNext);
        let third = e.mem.u32(second + 4);
        // A middle node is unlinked, destroyed and freed.
        e.mem.set_u32(slot, 2);
        bs_simple_list_fade_control_remove(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![1, 3]);
        assert_eq!(calls_to(&e, SIMPLE_LIST_REMOVE_ALL), vec![vec![second]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, second]]
        );
        // The embedded first node takes over the second node.
        e.mem.set_u32(slot, 1);
        bs_simple_list_fade_control_remove(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![3]);
        assert_eq!(calls_to(&e, SIMPLE_LIST_REMOVE_ALL)[1], vec![third]);
        // The last item is only cleared.
        e.mem.set_u32(slot, 3);
        bs_simple_list_fade_control_remove(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![0]);
        assert_eq!(calls_to(&e, SIMPLE_LIST_REMOVE_ALL).len(), 2);
        // A null item, an item that is not there and an empty list do nothing.
        e.mem.set_u32(slot, 0);
        bs_simple_list_fade_control_remove(&mut e, list, Ptr::new(slot));
        e.mem.set_u32(slot, 9);
        bs_simple_list_fade_control_remove(&mut e, list, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, list), vec![0]);
        let other = make_simple_list(&mut e, &[1, 2]);
        bs_simple_list_fade_control_remove(&mut e, other, Ptr::new(slot));
        assert_eq!(simple_list_items(&e, other), vec![1, 2]);
        assert_eq!(calls_to(&e, SIMPLE_LIST_REMOVE_ALL).len(), 2);
    }

    /// The elements of a doubly linked `NiTPointerList`, head first.
    fn pointer_list_elements(e: &Engine, list: Ptr<NiTPointerList>) -> Vec<u32> {
        let mut node = e.get(list, NiTPointerList::m_pkHead);
        let mut items = vec![];
        let mut previous = 0;
        while node != 0 {
            assert_eq!(e.mem.u32(node + 4), previous, "back link");
            items.push(e.mem.u32(node + 8));
            previous = node;
            node = e.mem.u32(node);
        }
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), previous, "tail");
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), items.len() as u32);
        items
    }

    #[test]
    fn pointer_list_add_head_links_a_new_first_node() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let list: Ptr<NiTPointerList> = e.new_object();
        let slot = e.mem.alloc(4);
        for item in [10, 20, 30] {
            e.mem.set_u32(slot, item);
            fn_00a0c9f0(&mut e, list, Ptr::new(slot));
        }
        assert_eq!(pointer_list_elements(&e, list), vec![30, 20, 10]);
        assert_eq!(calls_to(&e, LIST_NEW_NODE)[0], vec![list.addr() + 8]);
    }

    #[test]
    fn pointer_list_add_after_links_behind_the_node() {
        let mut e = batch4_engine();
        let list: Ptr<NiTPointerList> = e.new_object();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 1);
        fn_00a0c9f0(&mut e, list, Ptr::new(slot));
        let first = e.get(list, NiTPointerList::m_pkHead);
        // Behind the last node: it becomes the tail.
        e.mem.set_u32(slot, 3);
        let third = fn_00a0ca70(&mut e, list, first, Ptr::new(slot));
        assert_eq!(pointer_list_elements(&e, list), vec![1, 3]);
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), third);
        // In the middle: the later node points back to the new one.
        e.mem.set_u32(slot, 2);
        let second = fn_00a0ca70(&mut e, list, first, Ptr::new(slot));
        assert_eq!(pointer_list_elements(&e, list), vec![1, 2, 3]);
        assert_eq!(e.mem.u32(third + 4), second);
    }

    /// The vtable of a test `BSSimpleArray`: allocate `count` words, free
    /// (nothing, the call is logged) and reallocate keeping the old words.
    fn install_array_vtable(e: &mut Engine) {
        e.put_vtable(
            ARRAY_VTABLE,
            &[
                0,
                ARRAY_ALLOCATE_DOUBLE,
                ARRAY_FREE_DOUBLE,
                ARRAY_REALLOCATE_DOUBLE,
            ],
        );
        e.register(ARRAY_ALLOCATE_DOUBLE, |e, a| {
            zeroed_block(e, a[1] * 4).into_ret()
        });
        e.register(ARRAY_FREE_DOUBLE, |_, _| Ret::default());
        e.register(ARRAY_REALLOCATE_DOUBLE, |e, a| {
            let fresh = zeroed_block(e, a[2] * 4);
            let old = e.mem.block_size(a[1]).unwrap_or(0).min(a[2] * 4);
            let data = e.mem.bytes(a[1], old);
            e.mem.write(fresh, &data);
            fresh.into_ret()
        });
        // The comparison: the numbers the two pointers point to.
        e.register(COMPARE_DOUBLE, |e, a| {
            (e.mem.i32(a[0]).cmp(&e.mem.i32(a[1])) as i32).into_ret()
        });
    }

    /// A `BSSimpleArray` of `values` with room for `capacity` words.
    fn make_array(e: &mut Engine, values: &[u32], capacity: u32) -> Ptr<BSSimpleArray> {
        let array = zeroed_block(e, 0x10);
        e.mem.set_u32(array, ARRAY_VTABLE);
        if capacity > 0 {
            let buffer = zeroed_block(e, capacity * 4);
            for (index, value) in values.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * index as u32, *value);
            }
            e.mem.set_u32(array + 4, buffer);
        }
        e.mem.set_u32(array + 8, values.len() as u32);
        e.mem.set_u32(array + 0xc, capacity);
        Ptr::new(array)
    }

    fn array_values(e: &Engine, array: Ptr<BSSimpleArray>) -> Vec<u32> {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        (0..e.get(array, BSSimpleArray::iSize))
            .map(|index| e.mem.u32(buffer + 4 * index))
            .collect()
    }

    #[test]
    fn array_sorted_insert_finds_the_place_with_the_comparison() {
        let mut e = batch4_engine();
        install_array_vtable(&mut e);
        let array = make_array(&mut e, &[], 8);
        // The elements are the numbers themselves; the comparison reads
        // through the two pointers.
        let slot = e.mem.alloc(4);
        let insert = |e: &mut Engine, value: u32| {
            // The array holds numbers; the key slot holds the number.
            e.mem.set_u32(slot, value);
            fn_00a0caf0(e, array, slot, COMPARE_DOUBLE);
        };
        for value in [5, 1, 9, 3, 7] {
            insert(&mut e, value);
        }
        assert_eq!(array_values(&e, array), vec![1, 3, 5, 7, 9]);
        insert(&mut e, 0);
        insert(&mut e, 10);
        assert_eq!(array_values(&e, array), vec![0, 1, 3, 5, 7, 9, 10]);
        // A key that compares equal is inserted at the match.
        insert(&mut e, 5);
        assert_eq!(array_values(&e, array), vec![0, 1, 3, 5, 5, 7, 9, 10]);
    }

    #[test]
    fn array_sorted_find_answers_the_index_or_minus_one() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        e.register(BSEARCH, |e, a| {
            for index in 0..a[2] {
                if e.mem.u32(a[1] + 4 * index) == e.mem.u32(a[0]) {
                    return (a[1] + 4 * index).into_ret();
                }
            }
            0u32.into_ret()
        });
        let array = make_array(&mut e, &[10, 20, 30], 4);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 30);
        assert_eq!(
            bs_simple_array_tile_value_sorted_find(&mut e, array, key, 0x77),
            2
        );
        assert_eq!(calls_to(&e, BSEARCH), vec![vec![key, buffer, 3, 4, 0x77]]);
        e.mem.set_u32(key, 25);
        assert_eq!(
            bs_simple_array_tile_value_sorted_find(&mut e, array, key, 0x77),
            -1
        );
    }

    #[test]
    fn array_insert_shifts_grows_or_appends() {
        let mut e = batch4_engine();
        install_array_vtable(&mut e);
        e.call_log = Some(vec![]);
        let slot = e.mem.alloc(4);
        // With room: the tail moves up.
        let array = make_array(&mut e, &[1, 2, 3], 8);
        e.mem.set_u32(slot, 9);
        fn_00a0cd50(&mut e, array, 1, slot);
        assert_eq!(array_values(&e, array), vec![1, 9, 2, 3]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        // Full: a new buffer of doubled capacity, the old one is freed.
        let full = make_array(&mut e, &[1, 2, 3, 4], 4);
        let old = e.get(full, BSSimpleArray::pBuffer);
        fn_00a0cd50(&mut e, full, 2, slot);
        assert_eq!(array_values(&e, full), vec![1, 2, 9, 3, 4]);
        assert_eq!(e.get(full, BSSimpleArray::iReservedSize), 8);
        assert_ne!(e.get(full, BSSimpleArray::pBuffer), old);
        assert_eq!(
            calls_to(&e, ARRAY_FREE_DOUBLE),
            vec![vec![full.addr(), old]]
        );
        // Above 8 the capacity grows by 8.
        let wide = make_array(&mut e, &[0; 10], 10);
        fn_00a0cd50(&mut e, wide, 0, slot);
        assert_eq!(e.get(wide, BSSimpleArray::iReservedSize), 18);
        assert_eq!(array_values(&e, wide)[..3], [9, 0, 0]);
        // At the end it appends.
        let tail = make_array(&mut e, &[1, 2], 2);
        fn_00a0cd50(&mut e, tail, 2, slot);
        assert_eq!(array_values(&e, tail), vec![1, 2, 9]);
        assert_eq!(e.get(tail, BSSimpleArray::iReservedSize), 4);
    }

    #[test]
    fn array_add_stores_the_item_and_answers_the_index() {
        let mut e = batch4_engine();
        install_array_vtable(&mut e);
        let array = make_array(&mut e, &[], 0);
        let slot = e.mem.alloc(4);
        for (expected, value) in [(0, 7), (1, 8)] {
            e.mem.set_u32(slot, value);
            assert_eq!(fn_00a0cf00(&mut e, array, slot), expected);
        }
        assert_eq!(array_values(&e, array), vec![7, 8]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 4);
    }

    #[test]
    fn array_move_copies_forward_or_backward() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let block = e.mem.alloc(32);
        for index in 0..8 {
            e.mem.set_u32(block + 4 * index, index + 1);
        }
        let words = |e: &Engine| (0..8).map(|i| e.mem.u32(block + 4 * i)).collect::<Vec<_>>();
        let destinations = |e: &Engine| -> Vec<u32> {
            calls_to(e, MEMMOVE)
                .iter()
                .map(|call| call[0] - block)
                .collect()
        };
        // Down by one word: ascending.
        fn_00a0cf60(&mut e, 0, block, block + 4, 3);
        assert_eq!(words(&e), vec![2, 3, 4, 4, 5, 6, 7, 8]);
        assert_eq!(destinations(&e), vec![0, 4, 8]);
        // Up by one word: descending.
        e.call_log = Some(vec![]);
        fn_00a0cf60(&mut e, 0, block + 4, block, 3);
        assert_eq!(words(&e), vec![2, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(destinations(&e), vec![12, 8, 4]);
        // Equal places and an empty count copy nothing.
        e.call_log = Some(vec![]);
        fn_00a0cf60(&mut e, 0, block, block, 3);
        fn_00a0cf60(&mut e, 0, block, block + 8, 0);
        assert!(calls_to(&e, MEMMOVE).is_empty());
    }

    #[test]
    fn array_make_room_gives_the_first_buffer_then_grows() {
        let mut e = batch4_engine();
        install_array_vtable(&mut e);
        e.call_log = Some(vec![]);
        let array = make_array(&mut e, &[], 0);
        assert_eq!(fn_00a0d000(&mut e, array), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 1);
        assert_eq!(
            calls_to(&e, ARRAY_ALLOCATE_DOUBLE),
            vec![vec![array.addr(), 4]]
        );
        // Room left: only the count changes.
        assert_eq!(fn_00a0d000(&mut e, array), 1);
        assert_eq!(calls_to(&e, ARRAY_ALLOCATE_DOUBLE).len(), 1);
        // Full (4 of 4): doubled to 8 by reallocation.
        fn_00a0d000(&mut e, array);
        fn_00a0d000(&mut e, array);
        assert_eq!(fn_00a0d000(&mut e, array), 4);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE_DOUBLE).len(), 1);
        // 8 of 8: doubled to 16, then 16 grows by 8.
        for _ in 0..3 {
            fn_00a0d000(&mut e, array);
        }
        assert_eq!(fn_00a0d000(&mut e, array), 8);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 16);
        for _ in 0..8 {
            fn_00a0d000(&mut e, array);
        }
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 24);
    }

    #[test]
    fn array_resize_allocates_reallocates_or_copies() {
        let mut e = batch4_engine();
        install_array_vtable(&mut e);
        e.call_log = Some(vec![]);
        // Without a buffer: a new one and the capacity.
        let empty = make_array(&mut e, &[], 0);
        fn_00a0d0c0(&mut e, empty, 6, 0);
        assert_ne!(e.get(empty, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(empty, BSSimpleArray::iReservedSize), 6);
        // The count equals the capacity: reallocated.
        let full = make_array(&mut e, &[1, 2], 2);
        fn_00a0d0c0(&mut e, full, 4, 2);
        assert_eq!(calls_to(&e, ARRAY_REALLOCATE_DOUBLE).len(), 1);
        assert_eq!(e.mem.u32(e.get(full, BSSimpleArray::pBuffer)), 1);
        // Otherwise: a new buffer with the elements, the old one freed.
        let partial = make_array(&mut e, &[5, 6, 7], 4);
        let old = e.get(partial, BSSimpleArray::pBuffer);
        fn_00a0d0c0(&mut e, partial, 8, 3);
        assert_eq!(array_values(&e, partial), vec![5, 6, 7]);
        assert_ne!(e.get(partial, BSSimpleArray::pBuffer), old);
        assert_eq!(
            calls_to(&e, ARRAY_FREE_DOUBLE),
            vec![vec![partial.addr(), old]]
        );
    }

    #[test]
    fn menu_string_map_compares_keys_without_case() {
        let mut e = batch4_engine();
        let (a, b, c) = (
            cstring(&mut e, "Alpha"),
            cstring(&mut e, "aLPHA"),
            cstring(&mut e, "beta"),
        );
        let this = Ptr::new(0);
        assert!(menu_string_map_is_keys_equal(
            &mut e,
            this,
            Ptr::new(a),
            Ptr::new(b)
        ));
        assert!(!menu_string_map_is_keys_equal(
            &mut e,
            this,
            Ptr::new(a),
            Ptr::new(c)
        ));
    }

    #[test]
    fn menu_string_map_hash_lowers_the_characters() {
        let mut e = batch4_engine();
        let map = make_map(&mut e, 7, false);
        let key = cstring(&mut e, "AbC");
        let expected = ((97u32 * 33 + 98) * 33 + 99) % 7;
        assert_eq!(
            menu_string_map_key_to_hash_index(&mut e, map.cast(), Ptr::new(key)),
            expected
        );
        // The empty key hashes to 0.
        let empty = cstring(&mut e, "");
        assert_eq!(
            menu_string_map_key_to_hash_index(&mut e, map.cast(), Ptr::new(empty)),
            0
        );
    }

    #[test]
    fn string_hash_keeps_the_case_and_the_sign_of_the_bytes() {
        let mut e = batch4_engine();
        let map = make_map(&mut e, 7, false);
        let key = cstring(&mut e, "Ab");
        assert_eq!(
            fn_00a0d440(&mut e, map.cast(), Ptr::new(key)),
            (65u32 * 33 + 98) % 7
        );
        // A byte above 0x7f is added as a negative number.
        let high = e.mem.alloc(4);
        e.mem.set_cstr(high, &[0xe9]);
        assert_eq!(
            fn_00a0d440(&mut e, map.cast(), Ptr::new(high)),
            (-23i32 as u32) % 7
        );
    }

    #[test]
    fn integer_hash_and_node_allocation_use_the_map_fields() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let map = make_map(&mut e, 7, false);
        assert_eq!(fn_00a0d260(&mut e, map.cast(), 100), 100 % 7);
        let node = fn_00a0d280(&mut e, map.cast());
        assert_ne!(node, 0);
        assert_eq!(calls_to(&e, LIST_NEW_NODE), vec![vec![map.addr() + 0xc]]);
    }

    /// Records the vtable word of the object at each [`MAP_REMOVE_ALL`].
    fn record_remove_all(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        seen
    }

    #[test]
    fn base_map_destructors_empty_the_map_and_free_the_buckets() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let seen = record_remove_all(&mut e);
        let map = make_map(&mut e, 4, false);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        fn_00a0d390(&mut e, map.cast());
        assert_eq!(*seen.borrow(), vec![VTABLE_MAP_BASE]);
        assert_eq!(calls_to(&e, FREE_BYTES), vec![vec![table]]);
        // The pointer map level empties the map under its own vtable first.
        seen.borrow_mut().clear();
        fn_00a0d3c0(&mut e, map.cast());
        assert_eq!(*seen.borrow(), vec![VTABLE_POINTER_MAP, VTABLE_MAP_BASE]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_MAP_BASE);
        assert_eq!(calls_to(&e, FREE_BYTES).len(), 2);
        // The `int` to `int` map's base level.
        seen.borrow_mut().clear();
        fn_00a0d790(&mut e, map.cast());
        assert_eq!(*seen.borrow(), vec![VTABLE_INT_MAP_BASE]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_INT_MAP_BASE);
        assert_eq!(calls_to(&e, FREE_BYTES)[2], vec![table]);
    }

    #[test]
    fn string_map_destructor_frees_copied_keys_then_unwinds() {
        let mut e = batch4_engine();
        install_map_vtable(&mut e);
        e.call_log = Some(vec![]);
        let seen = record_remove_all(&mut e);
        let map = make_map(&mut e, 4, true);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        // Keys are copies (their own blocks), here the numbers 0x100 + n.
        for key in [0x101, 0x105, 0x102] {
            fn_00a0c6c0(&mut e, map, key, 0);
        }
        ni_t_string_template_map_destructor(&mut e, map.cast());
        // Bucket 1 (first node first: 0x105 then 0x101), then bucket 2.
        assert_eq!(
            calls_to(&e, FREE_BYTES),
            vec![vec![0x105], vec![0x101], vec![0x102], vec![table]]
        );
        assert_eq!(*seen.borrow(), vec![VTABLE_POINTER_MAP, VTABLE_MAP_BASE]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_MAP_BASE);
        // A map that does not copy its keys frees only the buckets.
        e.call_log = Some(vec![]);
        let plain = make_map(&mut e, 4, false);
        let plain_table = e.get(plain, NiTPointerMap::m_ppkHashTable);
        fn_00a0c6c0(&mut e, plain, 7, 0);
        ni_t_string_template_map_destructor(&mut e, plain.cast());
        assert_eq!(calls_to(&e, FREE_BYTES), vec![vec![plain_table]]);
    }

    #[test]
    fn map_node_key_release_only_when_keys_are_copied() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let node = zeroed_block(&mut e, 12);
        e.mem.set_u32(node + 4, 0x7777);
        let copying = make_map(&mut e, 2, true);
        let plain = make_map(&mut e, 2, false);
        fn_00a0d4a0(&mut e, plain.cast(), node);
        assert!(calls_to(&e, FREE_BYTES).is_empty());
        fn_00a0d4a0(&mut e, copying.cast(), node);
        assert_eq!(calls_to(&e, FREE_BYTES), vec![vec![0x7777]]);
    }

    #[test]
    fn scalar_deleting_destructors_free_the_object_with_bit_zero() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let seen = record_remove_all(&mut e);
        // The menu string map writes its own vtable, then unwinds the chain.
        let map = make_map(&mut e, 4, false);
        assert_eq!(
            menu_string_map_scalar_deleting_destructor(&mut e, map.cast(), 0),
            map.cast()
        );
        assert_eq!(*seen.borrow(), vec![VTABLE_POINTER_MAP, VTABLE_MAP_BASE]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        menu_string_map_scalar_deleting_destructor(&mut e, map.cast(), 1);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map.addr()]]
        );
        // The string template map and the two pointer map levels.
        seen.borrow_mut().clear();
        let second = make_map(&mut e, 4, false);
        ni_t_string_template_map_scalar_deleting_destructor(&mut e, second.cast(), 1);
        assert_eq!(*seen.borrow(), vec![VTABLE_POINTER_MAP, VTABLE_MAP_BASE]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
        seen.borrow_mut().clear();
        let third = make_map(&mut e, 4, false);
        ni_t_pointer_map_char_p_int_scalar_deleting_destructor(&mut e, third.cast(), 0);
        assert_eq!(*seen.borrow(), vec![VTABLE_POINTER_MAP, VTABLE_MAP_BASE]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
        seen.borrow_mut().clear();
        let fourth = make_map(&mut e, 4, false);
        ni_t_map_base_char_p_int_scalar_deleting_destructor(&mut e, fourth.cast(), 1);
        assert_eq!(*seen.borrow(), vec![VTABLE_MAP_BASE]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 3);
        seen.borrow_mut().clear();
        let ints = make_map(&mut e, 4, false);
        let table = e.get(ints, NiTPointerMap::m_ppkHashTable);
        ni_t_pointer_map_int_int_scalar_deleting_destructor(&mut e, ints.cast(), 1);
        assert_eq!(
            *seen.borrow(),
            vec![VTABLE_INT_POINTER_MAP, VTABLE_INT_MAP_BASE]
        );
        assert_eq!(calls_to(&e, FREE_BYTES).last().unwrap(), &vec![table]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE).last().unwrap(),
            &vec![MEMORY_MANAGER, ints.addr()]
        );
    }

    #[test]
    fn map_base_constructors_allocate_and_clear_the_buckets() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        for (build, vtable) in [
            (
                fn_00a0d5f0 as fn(&mut Engine, Ptr, u32) -> Ptr,
                VTABLE_MAP_BASE,
            ),
            (fn_00a0d710, VTABLE_INT_MAP_BASE),
        ] {
            let map = zeroed_block(&mut e, 0x14);
            e.mem.set_u32(map + 0xc, 99);
            assert_eq!(build(&mut e, Ptr::new(map), 5), Ptr::new(map));
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(e.mem.u32(map + 4), 5);
            assert_eq!(e.mem.u32(map + 0xc), 0);
            let table = e.mem.u32(map + 8);
            assert_ne!(table, 0);
            assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
        }
        assert_eq!(calls_to(&e, ALLOC_BYTES), vec![vec![20], vec![20]]);
        let memsets = calls_to(&e, MEMSET);
        assert_eq!(memsets.len(), 2);
        assert_eq!(memsets[0][1..], [0, 20]);
    }

    #[test]
    fn map_node_release_clears_the_value_and_frees_the_node() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let map = make_map(&mut e, 2, false);
        let node = zeroed_block(&mut e, 12);
        e.mem.set_u32(node + 8, 0x55);
        fn_00a0d7c0(&mut e, map.cast(), node);
        assert_eq!(e.mem.u32(node + 8), 0);
        assert_eq!(
            calls_to(&e, MAP_FREE_NODE),
            vec![vec![map.addr() + 0xc, node]]
        );
    }

    #[test]
    fn reaction_map_code_matches_the_int_map_code() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let seen = record_remove_all(&mut e);
        // The `int` map base's deleting destructor.
        let map = make_map(&mut e, 4, false);
        assert_eq!(
            ni_t_map_base_int_int_scalar_deleting_destructor(&mut e, map.cast(), 0),
            map.cast()
        );
        assert_eq!(*seen.borrow(), vec![VTABLE_INT_MAP_BASE]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        seen.borrow_mut().clear();
        let map = make_map(&mut e, 4, false);
        ni_t_map_base_int_int_scalar_deleting_destructor(&mut e, map.cast(), 1);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map.addr()]]
        );
        // The reaction map: both levels, buckets, then the object.
        seen.borrow_mut().clear();
        let map = make_map(&mut e, 4, false);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        ni_t_pointer_map_value_reaction_scalar_deleting_destructor(&mut e, map.cast(), 1);
        assert_eq!(
            *seen.borrow(),
            vec![VTABLE_REACTION_POINTER_MAP, VTABLE_REACTION_MAP_BASE]
        );
        assert_eq!(calls_to(&e, FREE_BYTES).last().unwrap(), &vec![table]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
        // Without bit 0 the object stays.
        let map = make_map(&mut e, 4, false);
        ni_t_pointer_map_value_reaction_scalar_deleting_destructor(&mut e, map.cast(), 0);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
        // The base destructor and its deleting form.
        seen.borrow_mut().clear();
        let map = make_map(&mut e, 4, false);
        fn_00a0d960(&mut e, map.cast());
        assert_eq!(*seen.borrow(), vec![VTABLE_REACTION_MAP_BASE]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_REACTION_MAP_BASE);
        ni_t_map_base_value_reaction_scalar_deleting_destructor(&mut e, map.cast(), 1);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 3);
        ni_t_map_base_value_reaction_scalar_deleting_destructor(&mut e, map.cast(), 2);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 3);
    }

    #[test]
    fn reaction_map_constructor_allocates_and_clears_the_buckets() {
        let mut e = batch4_engine();
        e.call_log = Some(vec![]);
        let map = zeroed_block(&mut e, 0x14);
        e.mem.set_u32(map + 0xc, 99);
        assert_eq!(fn_00a0d8e0(&mut e, Ptr::new(map), 5), Ptr::new(map));
        assert_eq!(e.mem.u32(map), VTABLE_REACTION_MAP_BASE);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
        assert_eq!(calls_to(&e, ALLOC_BYTES), vec![vec![20]]);
        assert_eq!(calls_to(&e, MEMSET)[0][1..], [0, 20]);
    }

    #[test]
    fn all_functions_of_the_fourth_batch_are_registered() {
        let e = Engine::new();
        for address in [
            0x00a0_c090,
            0x00a0_c0c0,
            0x00a0_c190,
            0x00a0_c220,
            0x00a0_c300,
            0x00a0_c430,
            0x00a0_c570,
            0x00a0_c6c0,
            0x00a0_c7a0,
            0x00a0_c7c0,
            0x00a0_c850,
            0x00a0_c900,
            0x00a0_c9f0,
            0x00a0_ca70,
            0x00a0_caf0,
            0x00a0_cb90,
            0x00a0_cbe0,
            0x00a0_cd50,
            0x00a0_cf00,
            0x00a0_cf60,
            0x00a0_d000,
            0x00a0_d0c0,
            0x00a0_d180,
            0x00a0_d1b0,
            0x00a0_d220,
            0x00a0_d260,
            0x00a0_d280,
            0x00a0_d2a0,
            0x00a0_d390,
            0x00a0_d3c0,
            0x00a0_d440,
            0x00a0_d4a0,
            0x00a0_d4d0,
            0x00a0_d520,
            0x00a0_d5c0,
            0x00a0_d5f0,
            0x00a0_d670,
            0x00a0_d710,
            0x00a0_d790,
            0x00a0_d7c0,
        ] {
            assert!(e.is_translated(address), "{address:08x}");
        }
    }

    #[test]
    fn all_functions_of_the_third_batch_are_registered() {
        let e = Engine::new();
        for address in [
            0x00a0_a0b0,
            0x00a0_a130,
            0x00a0_a220,
            0x00a0_a270,
            0x00a0_a300,
            0x00a0_a410,
            0x00a0_ab70,
            0x00a0_abe0,
            0x00a0_ac80,
            0x00a0_ad40,
            0x00a0_ae70,
            0x00a0_af10,
            0x00a0_b020,
            0x00a0_b110,
            0x00a0_b1f0,
            0x00a0_b280,
            0x00a0_b350,
            0x00a0_b420,
            0x00a0_b520,
            0x00a0_b8d0,
            0x00a0_b950,
            0x00a0_ba50,
            0x00a0_bae0,
            0x00a0_baf0,
            0x00a0_bb00,
            0x00a0_bb50,
            0x00a0_bbd0,
            0x00a0_bbe0,
            0x00a0_bbf0,
            0x00a0_bc80,
            0x00a0_bdc0,
            0x00a0_bdd0,
            0x00a0_bde0,
            0x00a0_be10,
            0x00a0_be70,
            0x00a0_beb0,
            0x00a0_c000,
            0x00a0_c020,
            0x00a0_c060,
            0x00a0_c080,
        ] {
            assert!(e.is_translated(address), "{address:08x}");
        }
    }
}
