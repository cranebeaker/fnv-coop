//! `fallout shared/tessaveloadgame.cpp` (Xbox PDB source unit), part 2: its functions from `008616f0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tessaveloadgame`]; anything public there may be used here.
//!
//! State of this file (second session, complete): the 32 functions from
//! `008637d0` to `008641d0`, after the 40 of the first session (`008616f0` to
//! `008637a0`). The range is finished.
//!
//! - `008616f0` to `00861d10`: whether saving is allowed, the created base
//!   objects list (adding to it, saving it), the numeric id and world space
//!   id arrays (look-up, adding, saving);
//! - `00861ea0` to `00862110`: the periodic load-screen poke, the version
//!   setter, the "test all cells" save loop, the save format version reset
//!   and `UseSaveGameBlocks`;
//! - `00862150` to `008623a0`: the name of the newest save, the next save
//!   number and the removal of a form from the queue of initial data;
//! - `00862430`, `008624e0`: the corrupt location / angle check and repair of
//!   a loaded reference;
//! - `008627b0`: the pass over the changes map that drops the changes of
//!   references in cells that have been detached for long;
//! - `008632a0`: adding a form to the init item array;
//! - `00863360` to `008637a0`: constructors and destructors of the
//!   `NiTPointerMap` instances the unit uses;
//! - `008637d0` to `008641d0`: the rest of the map instances (root
//!   constructors and destructors, the `unsigned char` keyed map's `SetAt`,
//!   `GetAt`, `GetNext`, scalar deleting destructors) and the
//!   `NiTLargePrimitiveArray` constructors, destructor, `Add` and cleanup.
//!
//! Not translated: the compiler's exception-unwinding frames (`FS:[0]`
//! chains, state variables) and the stack-cookie checks. The locals the game
//! keeps on its stack and passes by address are heap blocks here, freed where
//! the game's frame ends.
//!
//! In `008627b0` several tests mask a flag word with the constant 0 (`flags &
//! 0`; the mask is 0 in this build, as the main file notes for its own
//! functions). The code they guard is unreachable and is not translated:
//! the reads of the `CreatedReferenceData` / `MovedReferenceData` layouts, a
//! second clean-up for the other reference types, and the two other record
//! formats of a cell's detach time.

#[allow(unused_imports)]
use super::tessaveloadgame::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;

layout! {
    /// `NiTLargePrimitiveArray<T>` (Xbox PDB), 0x18 bytes: vtable, element
    /// block and its sizes.
    pub struct NiTLargePrimitiveArray: 0x18 {
        /// `m_pBase` (Xbox PDB): the element block.
        0x04 m_pBase: u32,
        /// `m_uiMaxSize` (Xbox PDB).
        0x08 m_uiMaxSize: u32,
        /// `m_uiSize` (Xbox PDB): the number of used slots.
        0x0C m_uiSize: u32,
        /// `m_uiESize` (Xbox PDB): the number of elements.
        0x10 m_uiESize: u32,
        /// `m_uiGrowBy` (Xbox PDB).
        0x14 m_uiGrowBy: u32,
    }
}

// ---------------------------------------------------------------------------
// Fields of `TESSaveLoadGame` the main file's layout lacks (offsets as in the
// Xbox PDB, checked against the PC code).

/// `m_CreatedBaseObjectsList` (Xbox PDB): a `BSSimpleList<unsigned int>`
/// embedded in the object; its head node is at this offset.
const CREATED_BASE_OBJECTS_LIST: u32 = 0x2C;
/// `m_pNumericIDArray` (Xbox PDB): `NiTLargePrimitiveArray<unsigned int> *`.
const NUMERIC_ID_ARRAY: u32 = 0x78;
/// `m_pWorldSpaceIDArray` (Xbox PDB): `NiTLargePrimitiveArray<unsigned int> *`.
const WORLDSPACE_ID_ARRAY: u32 = 0x7C;
/// `m_pInitItemArray` (Xbox PDB): `NiTLargePrimitiveArray<TESForm *> *`.
const INIT_ITEM_ARRAY: u32 = 0xB0;
/// `m_pMostRecentSaveGame` (Xbox PDB `_FILETIME *`): on the PC a heap copy of
/// the newest save's file name without its extension.
const MOST_RECENT_SAVE: u32 = 0x1C4;

// ---------------------------------------------------------------------------
// Callees and data outside this file

/// The dword at `+0x108` of the player object (`004f8960`); 2 and 1 forbid
/// saving.
const PLAYER_STATE_004F8960: u32 = 0x004f_8960;
/// True when any of the four flag bytes `+0x798..+0x79B` of the player is set
/// (`0093a740`); forbids saving.
const PLAYER_FLAGS_SET_0093A740: u32 = 0x0093_a740;
/// A test without arguments (`004a4040`): true when a global object exists and
/// its method `004a4080(1)` is true; makes saving allowed at once.
const SAVING_FORCED_004A4040: u32 = 0x004a_4040;
/// `Interface::GetTopMenuID` and `Interface::GetMenuModeType` (Xbox PDB).
const TOP_MENU_ID: u32 = 0x0070_23c0;
const MENU_MODE_TYPE: u32 = 0x0070_2640;
/// `Interface::GetLoadingMenuVisible` (Xbox PDB).
const LOADING_MENU_VISIBLE: u32 = 0x0070_5e80;
/// `KERNEL32.DLL GetTickCount`, the import slot the code calls through.
const GET_TICK_COUNT_IMPORT: u32 = 0x00fd_f060;
/// `00457d70(tes, 1, 0, 0)` (`fallout shared/tes.cpp`), called on the `TES`
/// object.
const TES_LOAD_SCREEN_POKE: u32 = 0x0045_7d70;
/// The global dword the poke compares the tick count with, and the
/// milliseconds that must have passed since it.
const LAST_POKE_TICK: u32 = 0x011d_e464;
const POKE_INTERVAL: u32 = 3000;

/// The `TES` object's grid cell count (`00453980`), its interior cell
/// (`005f36f0`, which the engine map names `ActorMover::GetPreferredMoveMode`
/// because the linker folded the identical code) and the grid cell of an
/// index (`00459470(tes, index)`).
const TES_CELL_COUNT: u32 = 0x0045_3980;
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
const TES_GRID_CELL: u32 = 0x0045_9470;
/// `TESObjectCELL::SaveGameTest` (Xbox PDB).
const CELL_SAVE_GAME_TEST: u32 = 0x0054_b5b0;
/// `004698a0` (`fallout shared/tesdatahandler.cpp`) with the nine words
/// `fn_00861f20` passes it, and the global it is given first.
const DATA_HANDLER_PLACE: u32 = 0x0046_98a0;
const PLACE_OBJECT_GLOBAL: u32 = 0x011c_a268;
/// The double `1000.0` added to the height of the placed reference.
const HEIGHT_OFFSET: u32 = 0x0101_7b70;
/// The counter of "Test All Cells %i.ess" saves, and how often one is made.
const TEST_ALL_CELLS_COUNTER: u32 = 0x011d_e500;
const TEST_ALL_CELLS_SAVE_EVERY: i32 = 0x32;
/// `"Test All Cells %i.ess"`.
const FORMAT_TEST_ALL_CELLS: u32 = 0x0108_1d7c;

/// Strings logged or formatted by this part.
const MSG_NULL_CREATED_BASE_OBJECT: u32 = 0x0108_1c18;
const MSG_NON_CREATED_BASE_OBJECT: u32 = 0x0108_1bc8;
const MSG_ENCHANTABLE_WITHOUT_ENCHANTMENT: u32 = 0x0108_1c60;
const LABEL_NUMERIC_ID_ARRAY: u32 = 0x0108_1cd8;
const LABEL_WORLDSPACE_ID_ARRAY: u32 = 0x0108_1cc0;
const MSG_OLD_LOAD_VERSION: u32 = 0x0108_1cf0;
const MSG_CORRUPT_ANGLE: u32 = 0x0108_1d98;
const MSG_CORRUPT_LOCATION: u32 = 0x0108_1de8;
/// `"\\"`, the path separator `004812f0` looks for.
const PATH_SEPARATOR: u32 = 0x0101_3444;

/// The oldest save format version the game loads without a warning.
const OLDEST_COMPATIBLE_VERSION: u8 = 0x13;

/// Run-time type descriptors cast to by `fn_00861820`: `.?AVSpellItem@@` and
/// `.?AVTESEnchantableForm@@`.
const RTTI_SPELL_ITEM: u32 = 0x0118_3060;
const RTTI_ENCHANTABLE_FORM: u32 = 0x0118_39b4;

/// Virtual slot `0x2C` of a form: fills the global form buffer that
/// `00473080` (its size) and `00473070` (its address) read; `00485b30`
/// frees it (`TESForm::FreeFormBuffer`, Xbox PDB).
const FORM_FILL_BUFFER_SLOT: u32 = 0x2C;
const FORM_BUFFER_SIZE: u32 = 0x0047_3080;
const FORM_BUFFER_ADDRESS: u32 = 0x0047_3070;
const FORM_FREE_BUFFER: u32 = 0x0048_5b30;
/// The word at `+4` of the enchantable part of a form is the enchantment
/// (`00726070`, the same code as `LIST_NODE_NEXT`).
const ENCHANTMENT_OF_FORM: u32 = LIST_NODE_NEXT;

/// `NiTLargePrimitiveArray<TESForm *>`: its constructor `(this, grow, size)`
/// (`004701d0`), the object size and the two parameters.
const FORM_ARRAY_CONSTRUCT: u32 = 0x0047_01d0;
const FORM_ARRAY_SIZE: u32 = 0x18;
const FORM_ARRAY_GROW: u32 = 20000;
const FORM_ARRAY_INITIAL: u32 = 2000;
/// `NiTLargePrimitiveArray::RemoveAt(array, index)` (`00486b80`).
const ARRAY_REMOVE_AT: u32 = 0x0048_6b80;

/// `_finite` and `_isnan` (CRT), each with a `double` argument.
const CRT_FINITE: u32 = 0x00ec_7595;
const CRT_IS_NAN: u32 = 0x00ec_75b1;

/// `008905f0(file)`: the byte at `+0x2C` of a file object. `004fd380(game)`:
/// `m_pSaveGameList`, the dword at `+0x70`.
const FILE_FLAG_2C: u32 = 0x0089_05f0;
const SAVE_GAME_LIST_GETTER: u32 = 0x004f_d380;
/// The extra data type whose presence `fn_008624e0` tests.
const EXTRA_DATA_START_LOCATION: u32 = 0xF;
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer(this, flags)` (Xbox PDB,
/// `00537e90`): stores `flags` in the `ChangeData`'s first word.
const CHANGE_DATA_SET_FLAGS: u32 = 0x0053_7e90;
/// The calendar object (`011de7b8`) and its method `00867e30`, which gives
/// the current time in the unit `TESObjectCELL::GetDetachTime` (`00546af0`)
/// uses, and the age after which a detached cell's changes are dropped
/// (`00526100`, read from a game setting).
const CALENDAR_OBJECT: u32 = 0x011d_e7b8;
const CALENDAR_TIME: u32 = 0x0086_7e30;
const CELL_GET_DETACH_TIME: u32 = 0x0054_6af0;
const DETACH_AGE_LIMIT: u32 = 0x0052_6100;
/// The change flag that says the record holds a cell's detach time.
const CHANGE_FLAG_DETACH_TIME: u32 = 0x4000_0000;
/// The form type of a cell.
const FORM_TYPE_CELL: u32 = TYPE_CELL as u32;
/// A saved cell record holds a detach time when its type byte is the cell
/// type and its version byte is at least this.
const CELL_RECORD_MIN_VERSION: u8 = 0x5B;
/// The marker for "no cell coordinate".
const NO_COORDINATE: u32 = 0x7fff_ffff;
/// Bits of a change flags word handled by `fn_008627b0`.
const CHANGE_FLAG_MOVED: u32 = 0x04;
const CHANGE_FLAG_CREATED: u32 = 0x02;
const CHANGE_FLAGS_LOCATION: u32 = 0x06;
/// Virtual slot `0x10` of a form: the scalar deleting destructor.
const FORM_SCALAR_DELETE_SLOT: u32 = 0x10;
/// Bytes of a `ReferenceData` record in a saved reference change block.
const REFERENCE_DATA_BYTES: u32 = 0x1C;

/// Vtables the constructors and destructors of the map instances set.
const CHANGES_MAP_BASE_VTABLE: u32 = 0x0108_1e30;
const INTERIOR_MAP_BASE_VTABLE: u32 = 0x0108_1e50;
const EXTERIOR_MAP_BASE_VTABLE: u32 = 0x0108_1e70;
const NUMERIC_ID_MAP_BASE_VTABLE: u32 = 0x0108_1e90;
const STATS_MAP_BASE_VTABLE: u32 = 0x0108_1eb0;
const LOCAL_MAP_VTABLE: u32 = 0x0108_1ed0;
const STATS_LIST_MAP_VTABLE: u32 = 0x0108_1ef0;
/// The vtables of the two root layers (`008635d0`, `008636d0`).
const CHANGES_MAP_ROOT_VTABLE: u32 = 0x0108_1ef8;
const INTERIOR_MAP_ROOT_VTABLE: u32 = 0x0108_1f18;
/// The vtables of the other root layers and of the arrays (`008637d0`,
/// `008638f0`, `008639f0`, `00863e30`, `00863d60`, `00863e00`, `00864160`).
const EXTERIOR_MAP_ROOT_VTABLE: u32 = 0x0108_1f38;
const NUMERIC_ID_MAP_ROOT_VTABLE: u32 = 0x0108_1f58;
const STATS_MAP_ROOT_VTABLE: u32 = 0x0108_1f78;
const FORM_ARRAY_VTABLE: u32 = 0x0108_1f98;
const FORM_ARRAY_DERIVED_VTABLE: u32 = 0x0108_1fa0;
const LOCAL_MAP_ROOT_VTABLE: u32 = 0x0108_1fa8;
const ARRAY_BASE_VTABLE: u32 = 0x0108_1fc8;
/// Slots of the `NiTMapBase` vtable: `KeyToHashIndex(key)`,
/// `IsKeysEqual(key, key)`, `SetValue(item, key, value)` and `NewItem()`.
const MAP_SLOT_KEY_TO_HASH_INDEX: u32 = 0x04;
const MAP_SLOT_IS_KEYS_EQUAL: u32 = 0x08;
const MAP_SLOT_SET_VALUE: u32 = 0x0C;
const MAP_SLOT_NEW_ITEM: u32 = 0x14;
/// `004ede70(block)` (cdecl): releases the element block of an array.
/// `0096afc0(bytes)` (cdecl): allocates it.
const ARRAY_FREE_BLOCK: u32 = 0x004e_de70;
const ARRAY_ALLOC_BLOCK: u32 = 0x0096_afc0;
/// `00864100(this)`, which `fn_008640e0` runs (the array's destructor body),
/// and `00864240(this, size, item)`, the array's insertion.
const ARRAY_DESTRUCT_BODY: u32 = 0x0086_4100;
const ARRAY_ADD_AT: u32 = 0x0086_4240;
/// The constructor and destructor of the map `fn_008627b0` keeps on its
/// stack (`00863e30`, `00863ea0`), reached by address from `fn_00863450`,
/// `fn_00863570` and `fn_008627b0`.
const LOCAL_MAP_ROOT_CONSTRUCT: u32 = 0x0086_3e30;
const LOCAL_MAP_DESTRUCT: u32 = 0x0086_3ea0;
/// `NiAlloc(size)` / `NiFree(block)` (`00aa1070`, `00aa10f0`, `cdecl`).
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;

// ---------------------------------------------------------------------------
// Helpers

/// The dword at `offset` in the game object.
fn member(e: &Engine, this: Ptr<TESSaveLoadGame>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

/// Count of a `NiTLargePrimitiveArray` (`0084e3a0`).
fn array_count(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// The address of the slot of `index` (`00877a30`).
fn array_slot(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32()
}

/// Appends the word `value` to the array (`00863d90(array, &value)`); the
/// game passes the address of a local. Returns what the call returns.
fn array_add(e: &mut Engine, array: u32, value: u32) -> u32 {
    let cell = e.mem.alloc(4);
    e.mem.set_u32(cell, value);
    let result = e.call(INIT_ARRAY_ADD, &args![array, cell]).u32();
    e.mem.free(cell);
    result
}

/// Writes the four bytes of `value` to the save file (`fn_00857b50`).
fn write_word(e: &mut Engine, game: Ptr<TESSaveLoadGame>, file: Ptr, value: u32) {
    let cell = e.mem.alloc(4);
    e.mem.set_u32(cell, value);
    fn_00857b50(e, game, file, Ptr::new(cell), 4);
    e.mem.free(cell);
}

// ---------------------------------------------------------------------------
// Whether saving is allowed, the created base objects

// Translated from 008616f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::GetSavingAllowed` (Xbox PDB): false while the player's
/// state word (`+0x108`) is 2 or 1 or any of its four flag bytes is set; true
/// at once when `004a4040` says so; otherwise only with no menu open or the
/// menu ids 0 / 3 (`Interface::GetTopMenuID`, `Interface::GetMenuModeType`).
pub fn tes_save_load_game_get_saving_allowed(e: &mut Engine, _this: Ptr<TESSaveLoadGame>) -> bool {
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_STATE_004F8960, &args![player]).i32() == 2 {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_STATE_004F8960, &args![player]).i32() == 1 {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_FLAGS_SET_0093A740, &args![player]).bool() {
        return false;
    }
    if e.call(SAVING_FORCED_004A4040, &args![]).bool() {
        return true;
    }
    let top_menu = e.call(TOP_MENU_ID, &args![]).i32();
    if top_menu != 0 && top_menu != 3 {
        return false;
    }
    let menu_mode = e.call(MENU_MODE_TYPE, &args![]).i32();
    menu_mode == 0 || menu_mode == 3
}

// Translated from 00861780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::AddCreatedBaseObject` (Xbox PDB): adds the form's id to
/// the created base objects list unless it is already there. A null form, or
/// a form whose id the data handler does not know as created (`00469860`),
/// logs an error and adds nothing.
pub fn tes_save_load_game_add_created_base_object(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form: Ptr,
) {
    if form.is_null() {
        e.call(LOG_ERROR, &args![MSG_NULL_CREATED_BASE_OBJECT]);
        return;
    }
    let form_id = form_key(e, form);
    let handler: u32 = e.global(DATA_HANDLER);
    if !e
        .call(DATA_HANDLER_HAS_FORM, &args![handler, form_id])
        .bool()
    {
        let form_id = form_key(e, form);
        e.call(LOG_ERROR, &args![MSG_NON_CREATED_BASE_OBJECT, form_id]);
        return;
    }
    let list = this.addr() + CREATED_BASE_OBJECTS_LIST;
    let cell = e.mem.alloc(4);
    let form_id = form_key(e, form);
    e.mem.set_u32(cell, form_id);
    if !e.call(LIST_CONTAINS, &args![list, cell]).bool() {
        let form_id = form_key(e, form);
        e.mem.set_u32(cell, form_id);
        e.call(LIST_ADD_HEAD, &args![list, cell]);
    }
    e.mem.free(cell);
}

// Translated from 00861820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveCreatedBaseObjects` (Xbox PDB): writes the created
/// base objects (the forms whose ids are in the list at `+0x2C`) to the save
/// file. First the count: every form that is a bound object or a spell item
/// counts one, and a form with an enchantment (the enchantable part of the
/// form, which the type descriptor `.?AVTESEnchantableForm@@` names) counts
/// one more when the data handler knows the enchantment's id as created; a
/// form that is enchantable but has no enchantment is not counted. Then, for
/// each such form, the enchantment first (when created), then the form
/// itself (`save_form_record`). An enchantable form with no enchantment logs
/// an error and is not written.
pub fn tes_save_load_game_save_created_base_objects(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
) {
    let handler: u32 = e.global(DATA_HANDLER);
    let list = this.addr() + CREATED_BASE_OBJECTS_LIST;

    let mut count = 0u32;
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let form_id = e.mem.u32(slot);
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        if form != 0 {
            let bound = dynamic_cast(e, form, RTTI_FORM, RTTI_BOUND_OBJECT);
            let spell = dynamic_cast(e, form, RTTI_FORM, RTTI_SPELL_ITEM);
            let enchantable = dynamic_cast(e, form, RTTI_FORM, RTTI_ENCHANTABLE_FORM);
            if enchantable != 0 && e.call(ENCHANTMENT_OF_FORM, &args![enchantable]).u32() == 0 {
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                continue;
            }
            if enchantable != 0 && e.call(ENCHANTMENT_OF_FORM, &args![enchantable]).u32() != 0 {
                let enchantment = e.call(ENCHANTMENT_OF_FORM, &args![enchantable]).u32();
                let id = e.call(FORM_ID, &args![enchantment]).u32();
                if e.call(DATA_HANDLER_HAS_FORM, &args![handler, id]).bool() {
                    count += 1;
                }
            }
            if bound != 0 || spell != 0 {
                count += 1;
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    write_word(e, this, file, count);
    if count == 0 {
        return;
    }

    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let form_id = e.mem.u32(slot);
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        if form != 0 {
            let bound = dynamic_cast(e, form, RTTI_FORM, RTTI_BOUND_OBJECT);
            let spell = dynamic_cast(e, form, RTTI_FORM, RTTI_SPELL_ITEM);
            if bound != 0 || spell != 0 {
                let enchantable = dynamic_cast(e, form, RTTI_FORM, RTTI_ENCHANTABLE_FORM);
                if enchantable != 0 {
                    let enchantment = e.call(ENCHANTMENT_OF_FORM, &args![enchantable]).u32();
                    if enchantment == 0 {
                        e.call(
                            LOG_ERROR,
                            &args![MSG_ENCHANTABLE_WITHOUT_ENCHANTMENT, form_id],
                        );
                        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                        continue;
                    }
                    let id = e.call(FORM_ID, &args![enchantment]).u32();
                    if e.call(DATA_HANDLER_HAS_FORM, &args![handler, id]).bool() {
                        save_form_record(e, this, file, enchantment, None);
                    }
                }
                save_form_record(e, this, file, form, Some(form_id));
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// Writes one form's record the way `fn_00861820` does: virtual slot `0x2C`
/// fills the global form buffer, its size and contents go to the file, the
/// statistics note a `LoadFormHeader` (type, id, size; flags zero, version
/// not set) when they are collected, and the buffer is freed. `list_id` is
/// the id the list holds for the form (the statistics use it for the form
/// itself); the enchantment (`None`) uses its own form id.
fn save_form_record(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    form: u32,
    list_id: Option<u32>,
) {
    e.vcall(form, FORM_FILL_BUFFER_SLOT, &args![]);
    let size = e.call(FORM_BUFFER_SIZE, &args![form]).u32();
    let buffer = e.call(FORM_BUFFER_ADDRESS, &args![form]).u32();
    fn_00857b50(e, this, file, Ptr::new(buffer), size);
    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        let header: Ptr<LoadFormHeader> = Ptr::new(e.mem.alloc(LoadFormHeader::SIZE));
        e.call(LIST_NODE_ITEM, &args![header]);
        let form_type = e.call(FORM_TYPE, &args![form]).u8();
        e.set(header, LoadFormHeader::cFormType, form_type);
        let id = match list_id {
            Some(id) => id,
            None => e.call(FORM_ID, &args![form]).u32(),
        };
        e.set(header, LoadFormHeader::iFormID, id);
        e.set(header, LoadFormHeader::iSize, size as u16);
        e.set(header, LoadFormHeader::iFlags, 0);
        fn_00855a20(e, stats, header);
        e.mem.free(header.addr());
    }
    e.call(FORM_FREE_BUFFER, &args![form]);
}

// ---------------------------------------------------------------------------
// Numeric id arrays

// Translated from 00861b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::AddNumericIDToArray` (Xbox PDB): the number a form id
/// is saved under. A created id (the data handler knows it, `00469860`) is
/// saved as it is; any other id is looked up in the numeric id array
/// (`+0x78`) and its index returned, or appended (`00863d90`) when absent.
pub fn tes_save_load_game_add_numeric_id_to_array(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    id: u32,
) -> u32 {
    let handler: u32 = e.global(DATA_HANDLER);
    if e.call(DATA_HANDLER_HAS_FORM, &args![handler, id]).bool() {
        return id;
    }
    let array = member(e, this, NUMERIC_ID_ARRAY);
    let count = array_count(e, array);
    for index in 0..count {
        let array = member(e, this, NUMERIC_ID_ARRAY);
        let slot = array_slot(e, array, index);
        if e.mem.u32(slot) == id {
            return index;
        }
    }
    let array = member(e, this, NUMERIC_ID_ARRAY);
    array_add(e, array, id)
}

// Translated from 00861c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `AddNumericIDToArray`: a created id stays; any other
/// number is an index into the numeric id array (`+0x78`) and gives the id
/// stored there, or 0 for a number above the array's count.
pub fn fn_00861c00(e: &mut Engine, this: Ptr<TESSaveLoadGame>, number: u32) -> u32 {
    let handler: u32 = e.global(DATA_HANDLER);
    if e.call(DATA_HANDLER_HAS_FORM, &args![handler, number])
        .bool()
    {
        return number;
    }
    let array = member(e, this, NUMERIC_ID_ARRAY);
    let count = array_count(e, array);
    if number > count {
        return 0;
    }
    let array = member(e, this, NUMERIC_ID_ARRAY);
    let slot = array_slot(e, array, number);
    e.mem.u32(slot)
}

// Translated from 00861c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of a world space id in the world space id array (`+0x7C`),
/// appended (`00863d90`) when absent; the callers use the low 16 bits (the
/// game returns `AX`).
pub fn fn_00861c60(e: &mut Engine, this: Ptr<TESSaveLoadGame>, id: u32) -> u16 {
    let array = member(e, this, WORLDSPACE_ID_ARRAY);
    let count = array_count(e, array);
    for index in 0..count {
        let array = member(e, this, WORLDSPACE_ID_ARRAY);
        let slot = array_slot(e, array, index);
        if e.mem.u32(slot) == id {
            return index as u16;
        }
    }
    let array = member(e, this, WORLDSPACE_ID_ARRAY);
    array_add(e, array, id) as u16
}

// Translated from 00861cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world space id stored at `index` in the world space id array
/// (`+0x7C`), or 0 for an index above the array's count.
pub fn fn_00861cd0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, index: u16) -> u32 {
    let array = member(e, this, WORLDSPACE_ID_ARRAY);
    let count = array_count(e, array);
    if index as u32 > count {
        return 0;
    }
    let array = member(e, this, WORLDSPACE_ID_ARRAY);
    let slot = array_slot(e, array, index as u32);
    e.mem.u32(slot)
}

// Translated from 00861d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveNumericIDArrays` (Xbox PDB): writes the numeric id
/// array (`+0x78`) and the world space id array (`+0x7C`) to the file, each
/// as its count followed by its ids. With statistics collected, notes
/// "Numeric ID Array(%i)" and "WorldSpace ID Array(%i)" with their sizes
/// (`count * 4 + 4`). Not translated: the stack-cookie check.
pub fn tes_save_load_game_save_numeric_id_arrays(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
) {
    let array = member(e, this, NUMERIC_ID_ARRAY);
    let numeric_count = array_count(e, array);
    write_word(e, this, file, numeric_count);
    for index in 0..numeric_count {
        let array = member(e, this, NUMERIC_ID_ARRAY);
        let slot = array_slot(e, array, index);
        let id = e.mem.u32(slot);
        write_word(e, this, file, id);
    }
    let array = member(e, this, WORLDSPACE_ID_ARRAY);
    let worldspace_count = array_count(e, array);
    write_word(e, this, file, worldspace_count);
    for index in 0..worldspace_count {
        let array = member(e, this, WORLDSPACE_ID_ARRAY);
        let slot = array_slot(e, array, index);
        let id = e.mem.u32(slot);
        write_word(e, this, file, id);
    }
    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        let text = e.mem.alloc(0x104);
        e.call(
            FORMAT,
            &args![text, 0x104u32, LABEL_NUMERIC_ID_ARRAY, numeric_count],
        );
        save_stats_add_extra_stat(
            e,
            stats,
            numeric_count.wrapping_mul(4).wrapping_add(4),
            Ptr::new(text),
        );
        e.call(
            FORMAT,
            &args![text, 0x104u32, LABEL_WORLDSPACE_ID_ARRAY, worldspace_count],
        );
        save_stats_add_extra_stat(
            e,
            stats,
            worldspace_count.wrapping_mul(4).wrapping_add(4),
            Ptr::new(text),
        );
        e.mem.free(text);
    }
}

// ---------------------------------------------------------------------------
// Small members

// Translated from 00861ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pokes the `TES` object (`00457d70(tes, 1, 0, 0)`) when more than 3000 ms
/// have passed since the tick count kept in `011de464` and the loading menu
/// is not visible (`Interface::GetLoadingMenuVisible`).
pub fn fn_00861ea0(e: &mut Engine, _this: Ptr<TESSaveLoadGame>) {
    let now = e.call(GET_TICK_COUNT_IMPORT, &args![]).u32();
    let last: u32 = e.global(LAST_POKE_TICK);
    if now > last.wrapping_add(POKE_INTERVAL) && !e.call(LOADING_MENU_VISIBLE, &args![]).bool() {
        let tes: u32 = e.global(TES_OBJECT);
        e.call(TES_LOAD_SCREEN_POKE, &args![tes, 1u32, 0u32, 0u32]);
    }
}

// Translated from 00861ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the version of the form being loaded in `m_cCurrentVersion`; a
/// version below 0x13 (the oldest compatible one) logs a warning first.
pub fn fn_00861ee0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, version: u8) {
    if version < OLDEST_COMPATIBLE_VERSION {
        e.call(
            LOG_ERROR,
            &args![
                MSG_OLD_LOAD_VERSION,
                version as u32,
                OLDEST_COMPATIBLE_VERSION as u32
            ],
        );
    }
    e.set(this, TESSaveLoadGame::m_cCurrentVersion, version);
}

// Translated from 00861f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::TestAllCells` (Xbox PDB): for the modes 4 and 5 only.
/// Mode 5 first runs `TESObjectCELL::SaveGameTest` on the interior cell, or on
/// every grid cell of the `TES` object, then five times makes the data handler
/// place a reference (`004698a0`) from the player's world space, parent cell,
/// rotation and position, moves it 1000 units up and sets that as its
/// location (`SetLocationOnReference`). Both modes then save "Test All Cells
/// %i.ess" through `fn_00856ca0` every 50th call and count the call. Not
/// translated: the stack-cookie check.
pub fn tes_save_load_game_test_all_cells(e: &mut Engine, this: Ptr<TESSaveLoadGame>, mode: u32) {
    if mode != 4 && mode != 5 {
        return;
    }
    if mode == 5 {
        let tes: u32 = e.global(TES_OBJECT);
        let mut count = e.call(TES_CELL_COUNT, &args![tes]).u32();
        let tes: u32 = e.global(TES_OBJECT);
        if e.call(TES_INTERIOR_CELL, &args![tes]).u32() != 0 {
            count = 1;
        }
        for index in 0..count {
            let tes: u32 = e.global(TES_OBJECT);
            let mut cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
            if cell == 0 {
                let tes: u32 = e.global(TES_OBJECT);
                cell = e.call(TES_GRID_CELL, &args![tes, index]).u32();
            }
            if cell != 0 {
                e.call(CELL_SAVE_GAME_TEST, &args![cell]);
            }
        }
        let placed_global: u32 = e.global(PLACE_OBJECT_GLOBAL);
        for _ in 0..5 {
            let player: u32 = e.global(PLAYER);
            let world_space = e.call(REF_GET_WORLDSPACE, &args![player]).u32();
            let player: u32 = e.global(PLAYER);
            let parent_cell = e.call(REF_GET_PARENT_CELL, &args![player]).u32();
            let player: u32 = e.global(PLAYER);
            let rotation = e.call(REF_GET_ROTATION, &args![player]).u32();
            let player: u32 = e.global(PLAYER);
            let position = e.vcall(player, REFERENCE_GET_POSITION_SLOT, &args![]).u32();
            let handler: u32 = e.global(DATA_HANDLER);
            let placed = e
                .call(
                    DATA_HANDLER_PLACE,
                    &args![
                        handler,
                        placed_global,
                        position,
                        rotation,
                        parent_cell,
                        world_space,
                        0u32,
                        0u32,
                        0u32
                    ],
                )
                .u32();
            let placed_position = e.vcall(placed, REFERENCE_GET_POSITION_SLOT, &args![]).u32();
            let location = e.mem.alloc(12);
            let words = e.mem.bytes(placed_position, 12);
            e.mem.write(location, &words);
            let height_offset: f64 = e.global(HEIGHT_OFFSET);
            let height = (e.mem.f32(location + 8) as f64 + height_offset) as f32;
            e.mem.set_f32(location + 8, height);
            e.call(REF_SET_POSITION, &args![placed, location]);
            e.mem.free(location);
        }
    }
    let counter: i32 = e.global(TEST_ALL_CELLS_COUNTER);
    if counter % TEST_ALL_CELLS_SAVE_EVERY == 0 {
        let name = e.mem.alloc(0x104);
        let counter: i32 = e.global(TEST_ALL_CELLS_COUNTER);
        e.call(
            FORMAT,
            &args![name, 0x104u32, FORMAT_TEST_ALL_CELLS, counter],
        );
        fn_00856ca0(e, this, Ptr::NULL, Ptr::new(name), true);
        e.mem.free(name);
    }
    let counter: i32 = e.global(TEST_ALL_CELLS_COUNTER);
    e.set_global(TEST_ALL_CELLS_COUNTER, counter.wrapping_add(1));
}

// Translated from 008620f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the save format version: `m_cMajorVersion` 0 and `m_cMinorVersion`
/// 0x7D.
pub fn fn_008620f0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    e.set(this, TESSaveLoadGame::m_cMajorVersion, 0);
    e.set(this, TESSaveLoadGame::m_cMinorVersion, 0x7D);
}

// Translated from 00862110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB): whether the current
/// version of the game's `TESSaveLoadGame` (`011de45c`) is from 0x1F to 0x59.
pub fn tes_save_load_game_use_save_game_blocks(
    e: &mut Engine,
    _this: Ptr<TESSaveLoadGame>,
) -> bool {
    let game: u32 = e.global(SAVE_LOAD_GAME);
    let version = e.call(CURRENT_VERSION, &args![game]).u8();
    if version < 0x1F {
        return false;
    }
    let game: u32 = e.global(SAVE_LOAD_GAME);
    e.call(CURRENT_VERSION, &args![game]).u8() < 0x5A
}

// Translated from 00862150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remembers the name of a save file. Frees the name kept at `+0x1C4`; when
/// `file` is given and `008905f0(file)` says so (the byte at `+0x2C`), takes
/// the file's name (virtual slot `0x18`), cuts it after its last `\`, copies
/// it, drops its last four characters (the extension) and keeps a heap copy
/// at `+0x1C4`. Not translated: the stack-cookie check. A name of fewer than
/// four characters would make the game write below its buffer; here the
/// buffer has four spare bytes in front.
pub fn fn_00862150(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr) {
    let old = member(e, this, MOST_RECENT_SAVE);
    if old != 0 {
        delete(e, old);
        e.mem.set_u32(this.addr() + MOST_RECENT_SAVE, 0);
    }
    if file.is_null() {
        return;
    }
    if !e.call(FILE_FLAG_2C, &args![file]).bool() {
        return;
    }
    let mut name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
    let mut found = e.call(STRING_FIND, &args![name, PATH_SEPARATOR]).u32();
    while found != 0 {
        name = found + 1;
        found = e.call(STRING_FIND, &args![name, PATH_SEPARATOR]).u32();
    }
    let block = e.mem.alloc(0x108);
    let buffer = block + 4;
    e.call(STRING_COPY, &args![buffer, 0x104u32, name]);
    let length = e.call(STRLEN, &args![buffer]).u32();
    e.mem.set_u8(buffer.wrapping_add(length).wrapping_sub(4), 0);
    let size = e.call(STRLEN, &args![buffer]).u32().wrapping_add(1);
    let copy = e.call(OPERATOR_NEW, &args![size]).u32();
    e.mem.set_u32(this.addr() + MOST_RECENT_SAVE, copy);
    e.call(STRING_COPY, &args![copy, size, buffer]);
    e.mem.free(block);
}

// Translated from 008622a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number the next save gets: one more than the largest number of the
/// saves in the list. Builds the list (`fn_0085f900`) when there is none and
/// destroys it again afterwards, asks `fn_0085fed0` for each file's number,
/// resets the version (`fn_008620f0`), runs `00856850`, stores the result in
/// `m_iNextSaveNumber` and returns it.
pub fn fn_008622a0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) -> u32 {
    let mut list = e.call(SAVE_GAME_LIST_GETTER, &args![this]).u32();
    let mut built = false;
    if list == 0 {
        fn_0085f900(e, this);
        list = e.call(SAVE_GAME_LIST_GETTER, &args![this]).u32();
        built = true;
    }
    let mut highest = 0u32;
    let number = e.mem.alloc(4);
    let mut node = list;
    while node != 0 {
        if e.call(LIST_NODE_IS_END, &args![node]).bool() {
            break;
        }
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let file = e.mem.u32(slot);
        // The game's local is uninitialised until the call sets it.
        e.mem.set_u32(number, 0);
        fn_0085fed0(
            e,
            this,
            Ptr::new(file),
            Ptr::new(number),
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
        );
        let found = e.mem.u32(number);
        if found > highest {
            highest = found;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.mem.free(number);
    if built {
        fn_0085fbd0(e, this);
    }
    fn_008620f0(e, this);
    e.call(END_FORM_PROCESSING, &args![this]);
    e.set(
        this,
        TESSaveLoadGame::m_iNextSaveNumber,
        highest.wrapping_add(1),
    );
    e.get(this, TESSaveLoadGame::m_iNextSaveNumber)
}

// Translated from 00862370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `m_iNextSaveNumber`, worked out first (`fn_008622a0`) when it is 0.
pub fn fn_00862370(e: &mut Engine, this: Ptr<TESSaveLoadGame>) -> u32 {
    if e.get(this, TESSaveLoadGame::m_iNextSaveNumber) == 0 {
        fn_008622a0(e, this);
    }
    e.get(this, TESSaveLoadGame::m_iNextSaveNumber)
}

// Translated from 008623a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `form`'s entry from the init array (`+0x20`, `FormAndFlags *`):
/// the first entry whose form is `form` is taken out of the array
/// (`00486b80`) and deleted. Nothing happens without an array or an entry.
pub fn fn_008623a0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: u32) {
    let array = e.get(this, TESSaveLoadGame::m_pInitArray).addr();
    if array == 0 {
        return;
    }
    let count = array_count(e, array);
    for index in 0..count {
        let array = e.get(this, TESSaveLoadGame::m_pInitArray).addr();
        let slot = array_slot(e, array, index);
        let entry = e.mem.u32(slot);
        if entry != 0 && e.mem.u32(entry) == form {
            let array = e.get(this, TESSaveLoadGame::m_pInitArray).addr();
            e.call(ARRAY_REMOVE_AT, &args![array, index]);
            delete(e, entry);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Loaded references with corrupt coordinates

// Translated from 00862430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a vector of three floats is corrupt: a component that is not
/// finite (`_finite`, x then y then z) or a NaN (`_isnan`, in the same
/// order). The checks stop at the first that fails.
pub fn fn_00862430(e: &mut Engine, _this: Ptr<TESSaveLoadGame>, vector: Ptr) -> bool {
    for index in 0..3 {
        let value = e.mem.f32(vector.addr() + 4 * index) as f64;
        if e.call(CRT_FINITE, &args![value]).i32() == 0 {
            return true;
        }
    }
    for index in 0..3 {
        let value = e.mem.f32(vector.addr() + 4 * index) as f64;
        if e.call(CRT_IS_NAN, &args![value]).i32() != 0 {
            return true;
        }
    }
    false
}

// Translated from 008624e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repairs a loaded reference whose position or rotation is corrupt
/// (`fn_00862430`). A corrupt position logs an error and puts the reference
/// back: an actor-like one with a saved location (virtual slots `0x100`,
/// `0x290`) and a cell or world space (`0x298`, `0x294`) goes to that
/// location (`0x170`, `0x16C`) and is moved into that space; any other takes
/// the location of its extra data of type 0xF (the same two slots, then the
/// starting world space or cell) or, without it, the default position
/// (`011f426c`). A corrupt rotation logs an error and takes the default
/// position's three words as its rotation.
pub fn fn_008624e0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, reference: Ptr) {
    if reference.is_null() {
        return;
    }
    let reference_addr = reference.addr();
    let position = e.mem.alloc(12);
    let rotation = e.mem.alloc(12);
    e.call(LIST_NODE_ITEM, &args![position]);
    e.call(LIST_NODE_ITEM, &args![rotation]);
    let source = e
        .vcall(reference_addr, REFERENCE_GET_POSITION_SLOT, &args![])
        .u32();
    let words = e.mem.bytes(source, 12);
    e.mem.write(position, &words);
    let source = e.call(REF_GET_ROTATION, &args![reference]).u32();
    let words = e.mem.bytes(source, 12);
    e.mem.write(rotation, &words);

    if fn_00862430(e, this, Ptr::new(position)) {
        let id = form_key(e, reference);
        e.call(LOG_ERROR, &args![MSG_CORRUPT_LOCATION, id]);
        let mut actor = 0u32;
        if e.vcall(reference_addr, REFERENCE_IS_ACTOR, &args![]).bool() {
            actor = reference_addr;
        }
        if actor != 0 && e.vcall(actor, ACTOR_HAS_LOCATION_SLOT, &args![]).bool() {
            let cell = e.vcall(actor, ACTOR_CELL_SLOT, &args![]).u32();
            let worldspace = e.vcall(actor, ACTOR_WORLDSPACE_SLOT, &args![]).u32();
            if cell != 0 || worldspace != 0 {
                let out = e.mem.alloc(0x18);
                let saved_position = e
                    .vcall(actor, REFERENCE_GET_LOCATION_SLOT, &args![out])
                    .u32();
                e.call(REF_SET_POSITION, &args![reference, saved_position]);
                let saved_rotation = e
                    .vcall(actor, REFERENCE_GET_ROTATION_SLOT, &args![out + 0xC])
                    .u32();
                let angle_z = e.mem.f32(saved_rotation + 8);
                e.call(FN_005757D0, &args![reference, angle_z]);
                e.call(REF_MOVE_TO_SPACE, &args![reference, cell, worldspace]);
                e.mem.free(out);
            }
        } else {
            let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
            let start = e
                .call(EXTRA_GET_DATA, &args![list, EXTRA_DATA_START_LOCATION])
                .u32();
            if start != 0 {
                let out = e.mem.alloc(0x18);
                e.vcall(reference_addr, REFERENCE_GET_LOCATION_SLOT, &args![out]);
                e.vcall(
                    reference_addr,
                    REFERENCE_GET_ROTATION_SLOT,
                    &args![out + 0xC],
                );
                e.call(REF_SET_POSITION, &args![reference, out]);
                let (x, y, z) = (
                    e.mem.u32(out + 0xC),
                    e.mem.u32(out + 0x10),
                    e.mem.u32(out + 0x14),
                );
                e.call(FN_00575700, &args![reference, x, y, z]);
                let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
                let space = e.call(EXTRA_GET_STARTING_SPACE, &args![list]).u32();
                if space != 0 {
                    let cell = dynamic_cast(e, space, RTTI_FORM, RTTI_CELL);
                    let worldspace = dynamic_cast(e, space, RTTI_FORM, RTTI_WORLDSPACE);
                    if cell != 0 || worldspace != 0 {
                        e.call(REF_MOVE_TO_SPACE, &args![reference, cell, worldspace]);
                    }
                }
                e.mem.free(out);
            } else {
                e.call(REF_SET_POSITION, &args![reference, DEFAULT_POSITION]);
            }
        }
    }
    if fn_00862430(e, this, Ptr::new(rotation)) {
        let id = form_key(e, reference);
        e.call(LOG_ERROR, &args![MSG_CORRUPT_ANGLE, id]);
        let (x, y, z) = (
            e.mem.u32(DEFAULT_POSITION),
            e.mem.u32(DEFAULT_POSITION + 4),
            e.mem.u32(DEFAULT_POSITION + 8),
        );
        e.call(FN_00575700, &args![reference, x, y, z]);
    }
    e.mem.free(rotation);
    e.mem.free(position);
}

// ---------------------------------------------------------------------------
// The pass over the changes map

/// Reads `size` bytes from the current buffer of the game's own
/// `TESSaveLoadGame` (`011de45c`) into a fresh block and returns it.
fn read_from_singleton(e: &mut Engine, size: u32) -> u32 {
    let block = e.mem.alloc(size);
    let game = game_singleton(e);
    fn_008579e0(e, game, Ptr::new(block), size);
    block
}

// Translated from 008627b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass that drops the changes the save does not need (`fn_00856ca0`
/// calls it before it writes). It clears the byte at `+0x1C`
/// (`m_bAllowChanges`) and restores it at the end, and builds a map from a
/// cell's location id to the list of cell coordinates (`x`, `y`) of the
/// cells that have been detached for longer than `00526100` says.
///
/// Pass 1 (`detached_cell_entry`) walks the changes map: for each entry with
/// the detach-time bit (`0x40000000`) in its flags it takes the cell's detach
/// time from the saved record (a 4-byte header whose type byte is 0x39 and
/// version byte at least 0x5B, then 4 bytes) or, without a record, from the
/// loaded cell (`TESObjectCELL::GetDetachTime`); an exterior cell is keyed by
/// its world space id, with its grid x and y. A cell detached for longer than
/// the limit goes into the map, with its coordinates (or a null list when it
/// has none).
///
/// Pass 2 (`pass_two_entry`) walks the changes map again and handles the
/// entries of reference types whose cell is in the map. Finally the map and
/// its lists are freed.
///
/// Not translated: the exception frame, and the blocks guarded by a mask of
/// 0 (see the module notes).
pub fn fn_008627b0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let saved_allow = e.call(BUFFER_GET_VERSION, &args![this]).u8();
    e.call(BUFFER_SET_VERSION, &args![this, 0u32]);
    let cells: Ptr = Ptr::new(e.mem.alloc(NiTPointerMap::SIZE));
    fn_00863450(e, cells, HASH_SIZE);
    let now = e.call(CALENDAR_TIME, &args![CALENDAR_OBJECT]).u32();
    let changes = e.get(this, TESSaveLoadGame::m_pChanges);

    for_each_entry(e, changes.cast(), |e, key, change| {
        detached_cell_entry(e, this, cells, now, key, change)
    });
    for_each_entry(e, changes.cast(), |e, key, change| {
        pass_two_entry(e, this, cells, key, change)
    });

    for_each_entry(e, cells, |e, _key, list| {
        if list == 0 {
            return;
        }
        // The head node stays where it is while its items are removed.
        loop {
            let slot = e.call(LIST_NODE_ITEM, &args![list]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_NODE_ITEM, &args![list]).u32();
            let coordinates = e.mem.u32(slot);
            delete(e, coordinates);
            e.call(LIST_REMOVE_HEAD, &args![list]);
        }
        e.call(LIST_SCALAR_DELETE, &args![list, 1u32]);
    });
    e.call(MAP_REMOVE_ALL, &args![cells]);
    e.call(BUFFER_SET_VERSION, &args![this, saved_allow as u32]);
    e.call(LOCAL_MAP_DESTRUCT, &args![cells]);
    e.mem.free(cells.addr());
}

/// Pass 1 of `fn_008627b0` for one changes-map entry: notes the cell under
/// its location id when it has been detached for long enough.
fn detached_cell_entry(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    cells: Ptr,
    now: u32,
    key: u32,
    change: u32,
) {
    let flags = e.call(READ_WORD, &args![change]).u32();
    let mut location = key;
    let mut x = NO_COORDINATE;
    let mut y = NO_COORDINATE;
    if flags & CHANGE_FLAG_DETACH_TIME == 0 {
        return;
    }
    let mut detach_time = 0u32;
    if e.call(LIST_NODE_NEXT, &args![change]).u32() != 0 {
        let buffer = e.call(LIST_NODE_NEXT, &args![change]).u32();
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        let header = read_from_singleton(e, 4);
        if e.mem.u8(header + 2) as u32 == FORM_TYPE_CELL
            && e.mem.u8(header + 3) >= CELL_RECORD_MIN_VERSION
        {
            let cell = e.mem.alloc(4);
            fn_008579e0(e, this, Ptr::new(cell), 4);
            detach_time = e.mem.u32(cell);
            e.mem.free(cell);
        }
        e.mem.free(header);
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
    } else {
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_CELL {
            detach_time = e.call(CELL_GET_DETACH_TIME, &args![form]).u32();
            if !e.call(CELL_IS_INTERIOR, &args![form]).bool() {
                let worldspace = e.call(CELL_GET_WORLDSPACE, &args![form]).u32();
                location = e.call(ARRAY_SIZE, &args![worldspace]).u32();
                x = e.call(CELL_GET_X, &args![form]).u32();
                y = e.call(CELL_GET_Y, &args![form]).u32();
            }
        }
    }
    if detach_time == 0 {
        return;
    }
    let age = now.wrapping_sub(detach_time);
    if age <= e.call(DETACH_AGE_LIMIT, &args![]).u32() {
        return;
    }
    if x == NO_COORDINATE {
        e.call(CHANGES_MAP_SET_AT, &args![cells, location, 0u32]);
        return;
    }
    let coordinates = e.call(OPERATOR_NEW, &args![8u32]).u32();
    e.mem.set_u32(coordinates, x);
    e.mem.set_u32(coordinates + 4, y);
    let list_cell = e.mem.alloc(4);
    let found = e
        .call(MAP_GET_AT, &args![cells, location, list_cell])
        .bool();
    if !found {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(SIMPLE_LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(list_cell, list);
        e.call(CHANGES_MAP_SET_AT, &args![cells, location, list]);
    }
    let list = e.mem.u32(list_cell);
    let item = e.mem.alloc(4);
    e.mem.set_u32(item, coordinates);
    e.call(LIST_ADD_HEAD, &args![list, item]);
    e.mem.free(item);
    e.mem.free(list_cell);
}

/// Pass 2 of `fn_008627b0` for one changes-map entry. The entry's location id
/// and grid coordinates come from the saved record (a reference type, with
/// the `ReferenceData` when the flags say it has one) or from the loaded
/// reference's parent cell; the entry is handled when its cell is in the map
/// of detached cells (a null list stands for an interior cell, otherwise the
/// list must hold the coordinates):
///
/// - with bit 4 set and bit 2 clear: flags exactly 4 drop the entry
///   (`fn_00855220`); a loaded reference without a record has the bit cleared
///   (`fn_00855150`); a record has its `ReferenceData` (0x1C bytes) stripped
///   and its buffer rebuilt with the smaller size, the bit cleared. Then a
///   loaded reference is put back (`fn_00857d10`);
/// - for the reference types 0x3B and 0x3C that the data handler knows as
///   created: the entry is dropped and a loaded reference is destroyed
///   (virtual slot `0x10`, flag 1).
fn pass_two_entry(e: &mut Engine, this: Ptr<TESSaveLoadGame>, cells: Ptr, key: u32, change: u32) {
    let flags = e.call(READ_WORD, &args![change]).u32();
    let mut location = 0u32;
    let mut x = NO_COORDINATE;
    let mut y = NO_COORDINATE;
    let mut reference = 0u32;
    let mut form_type = 0u32;
    // The 4-byte header of the saved record: size (2), form type, version.
    let header = e.mem.alloc(4);
    if e.call(LIST_NODE_NEXT, &args![change]).u32() != 0 {
        let buffer = e.call(LIST_NODE_NEXT, &args![change]).u32();
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        let game = game_singleton(e);
        fn_008579e0(e, game, Ptr::new(header), 4);
        form_type = e.mem.u8(header + 2) as u32;
        if REFERENCE_TYPES.contains(&(form_type as u8)) {
            if flags & CHANGE_FLAGS_LOCATION != 0 {
                let data: Ptr<ReferenceData> = Ptr::new(e.mem.alloc(ReferenceData::SIZE));
                fn_008572f0(e, data);
                let game = game_singleton(e);
                fn_008579e0(e, game, data.cast(), REFERENCE_DATA_BYTES);
                location = e.get(data, ReferenceData::iLocationID);
                let loc_x = e.get(data, ReferenceData::LocX);
                x = (e.call(FLOAT_TO_INTEGER, &args![loc_x as f64]).i32() >> 12) as u32;
                let loc_y = e.get(data, ReferenceData::LocY);
                y = (e.call(FLOAT_TO_INTEGER, &args![loc_y as f64]).i32() >> 12) as u32;
                e.mem.free(data.addr());
            }
            location = fn_00861c00(e, this, location);
        }
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
    } else {
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        if form != 0 {
            form_type = e.call(FORM_TYPE, &args![form]).u32();
            if REFERENCE_TYPES.contains(&(form_type as u8)) {
                reference = form;
                let cell = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
                if cell != 0 {
                    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                        location = e.call(ARRAY_SIZE, &args![cell]).u32();
                        x = 0;
                        y = 0;
                    } else {
                        let worldspace = e.call(CELL_GET_WORLDSPACE, &args![cell]).u32();
                        location = e.call(ARRAY_SIZE, &args![worldspace]).u32();
                        x = e.call(CELL_GET_X, &args![cell]).u32();
                        y = e.call(CELL_GET_Y, &args![cell]).u32();
                    }
                }
            }
        }
    }
    if location == 0 || x == NO_COORDINATE || y == NO_COORDINATE {
        e.mem.free(header);
        return;
    }
    let list_cell = e.mem.alloc(4);
    if !e
        .call(MAP_GET_AT, &args![cells, location, list_cell])
        .bool()
    {
        e.mem.free(list_cell);
        e.mem.free(header);
        return;
    }
    let mut node = e.mem.u32(list_cell);
    e.mem.free(list_cell);
    let mut listed = node == 0;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 && e.mem.u32(item) == x && e.mem.u32(item + 4) == y {
            listed = true;
            break;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    if !listed {
        e.mem.free(header);
        return;
    }

    let changes = e.get(this, TESSaveLoadGame::m_pChanges);
    if flags & CHANGE_FLAG_MOVED != 0 && flags & CHANGE_FLAG_CREATED == 0 {
        if flags == CHANGE_FLAG_MOVED {
            fn_00855220(e, changes, key, 1);
        } else if e.call(LIST_NODE_NEXT, &args![change]).u32() == 0 {
            if reference != 0 {
                fn_00855150(e, changes, Ptr::new(reference), CHANGE_FLAG_MOVED);
            }
        } else {
            let old_buffer = e.call(LIST_NODE_NEXT, &args![change]).u32();
            let remaining_flags = flags & !CHANGE_FLAG_MOVED;
            let size = e.mem.u16(header).wrapping_sub(REFERENCE_DATA_BYTES as u16);
            e.mem.set_u16(header, size);
            let new_buffer = tes_save_load_game_create_buffer(e, this, size as u32 + 4);
            let game = game_singleton(e);
            fn_008579b0(e, game, Ptr::new(header), 4);
            fn_008579b0(
                e,
                this,
                Ptr::new(old_buffer + REFERENCE_DATA_BYTES + 4),
                size as u32,
            );
            delete(e, old_buffer);
            e.call(CHANGE_DATA_SET_BUFFER, &args![change, new_buffer]);
            e.call(CHANGE_DATA_SET_FLAGS, &args![change, remaining_flags]);
            e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
        }
        if reference != 0 {
            fn_00857d10(e, this.cast(), Ptr::new(reference), false);
        }
    }
    if form_type == 0x3B || form_type == 0x3C {
        let handler: u32 = e.global(DATA_HANDLER);
        if e.call(DATA_HANDLER_HAS_FORM, &args![handler, key]).bool() {
            let changes = e.get(this, TESSaveLoadGame::m_pChanges);
            fn_00855220(e, changes, key, 1);
            if reference != 0 {
                e.vcall(reference, FORM_SCALAR_DELETE_SLOT, &args![1u32]);
            }
        }
    }
    e.mem.free(header);
}

// ---------------------------------------------------------------------------
// The init item array

// Translated from 008632a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a form to the init item array (`+0xB0`): only when `0047c850` is true
/// (it is false in this build, so nothing happens); the array is made on
/// first use (0x18 bytes, grow 20000, size 2000). Not translated: the
/// exception frame.
pub fn fn_008632a0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) {
    if !game_unavailable(e, this) {
        return;
    }
    if member(e, this, INIT_ITEM_ARRAY) == 0 {
        let block = e.call(OPERATOR_NEW, &args![FORM_ARRAY_SIZE]).u32();
        let array = if block != 0 {
            e.call(
                FORM_ARRAY_CONSTRUCT,
                &args![block, FORM_ARRAY_GROW, FORM_ARRAY_INITIAL],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(this.addr() + INIT_ITEM_ARRAY, array);
    }
    let array = member(e, this, INIT_ITEM_ARRAY);
    array_add(e, array, form.addr());
}

// ---------------------------------------------------------------------------
// The `NiTPointerMap` instances

// Translated from 00863360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<unsigned int, ChangeData *>` constructor (the vtable
/// `01081e30` is the one its destructor `00863480` names): the root
/// constructor `008635d0`, then the vtable.
pub fn fn_00863360(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_008635d0(e, this, hash_size);
    e.mem.set_u32(this.addr(), CHANGES_MAP_BASE_VTABLE);
    this
}

// Translated from 00863390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map of `InteriorCellNewReferencesMap` (vtable
/// `01081e50`): the root constructor `008636d0`, then the vtable.
pub fn fn_00863390(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_008636d0(e, this, hash_size);
    e.mem.set_u32(this.addr(), INTERIOR_MAP_BASE_VTABLE);
    this
}

// Translated from 008633c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map of `ExteriorCellNewReferencesMap` (vtable
/// `01081e70`): the root constructor `008637d0`, then the vtable.
pub fn fn_008633c0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_008637d0(e, this, hash_size);
    e.mem.set_u32(this.addr(), EXTERIOR_MAP_BASE_VTABLE);
    this
}

// Translated from 008633f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map of `NumericIDBufferMap` (vtable `01081e90`):
/// the root constructor `008638f0`, then the vtable.
pub fn fn_008633f0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_008638f0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NUMERIC_ID_MAP_BASE_VTABLE);
    this
}

// Translated from 00863420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `SaveStats`'s map (vtable `01081eb0`): the root
/// constructor `008639f0`, then the vtable.
pub fn fn_00863420(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_008639f0(e, this, hash_size);
    e.mem.set_u32(this.addr(), STATS_MAP_BASE_VTABLE);
    this
}

// Translated from 00863450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map `fn_008627b0` keeps on its stack (vtable
/// `01081ed0`): the root constructor `00863e30`, then the vtable.
pub fn fn_00863450(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.call(LOCAL_MAP_ROOT_CONSTRUCT, &args![this, hash_size]);
    e.mem.set_u32(this.addr(), LOCAL_MAP_VTABLE);
    this
}

// Translated from 00863480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int,ChangeData *>::scalar deleting destructor`
/// (Xbox PDB): the destructor `00863640`, then the block is freed when bit 0
/// of `flags` is set.
pub fn ni_t_pointer_map_unsigned_int_change_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863640(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 008634b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int,BSSimpleList<unsigned int> *>::scalar
/// deleting destructor` (Xbox PDB): the destructor `00863740`, then the block
/// is freed when bit 0 of `flags` is set.
pub fn ni_t_pointer_map_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863740(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 008634e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int,BSSimpleList<ExteriorCellReferenceData *>
/// *>::scalar deleting destructor` (Xbox PDB): the destructor `00863860`,
/// then the block is freed when bit 0 of `flags` is set.
pub fn ni_t_pointer_map_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863860(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int,void *>::scalar deleting destructor`
/// (Xbox PDB): the destructor `00863960`, then the block is freed when bit 0
/// of `flags` is set.
pub fn ni_t_pointer_map_unsigned_int_void_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863960(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned char,BSSimpleList<LoadFormHeader *> *>::scalar
/// deleting destructor` (Xbox PDB): the destructor `00863cd0`, then the block
/// is freed when bit 0 of `flags` is set.
pub fn ni_t_pointer_map_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863cd0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the map `fn_008627b0` keeps on its stack
/// (the destructor `00863ea0`), then the block is freed when bit 0 of `flags`
/// is set.
pub fn fn_00863570(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LOCAL_MAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 008635a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map constructor with two arguments (vtable `01081ef0`): the root
/// constructor `00864160(this, first, second)`, then the vtable.
pub fn fn_008635a0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    fn_00864160(e, this, first, second);
    e.mem.set_u32(this.addr(), STATS_LIST_MAP_VTABLE);
    this
}

// Translated from 008635d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the `ChangesMap` map: sets the vtable `01081ef8`,
/// the bucket count, a zero item count and a zero-filled bucket array of
/// `hash_size * 4` bytes (`NiAlloc`, `memset`).
pub fn fn_008635d0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, CHANGES_MAP_ROOT_VTABLE)
}

/// Shared by the two root constructors `008635d0` and `008636d0` (identical
/// code with another vtable).
fn construct_map_root(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    hash_size: u32,
    vtable: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let bytes = hash_size << 2;
    let table = e.call(NI_ALLOC, &args![bytes]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00863640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `ChangesMap` map: vtable `01081e30`, `RemoveAll`
/// (`00438af0`), then the root destructor `008636a0`. Not translated: the
/// exception frame.
pub fn fn_00863640(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), CHANGES_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_008636a0(e, this);
}

// Translated from 008636a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the `ChangesMap` map: vtable `01081ef8`,
/// `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_008636a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), CHANGES_MAP_ROOT_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
}

// Translated from 008636d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the interior cells' map: `fn_008635d0` with the
/// vtable `01081f18`.
pub fn fn_008636d0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, INTERIOR_MAP_ROOT_VTABLE)
}

// Translated from 00863740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the interior cells' map: vtable `01081e50`, `RemoveAll`,
/// then the root destructor `008637a0`. Not translated: the exception frame.
pub fn fn_00863740(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), INTERIOR_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_008637a0(e, this);
}

// Translated from 008637a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the interior cells' map: vtable `01081f18`,
/// `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_008637a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), INTERIOR_MAP_ROOT_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
}

// Translated from 008637d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the exterior cells' map: `fn_008635d0` with the
/// vtable `01081f38`.
pub fn fn_008637d0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, EXTERIOR_MAP_ROOT_VTABLE)
}

// Translated from 00863840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetValue` of a map with `unsigned int` keys and pointer values (the
/// engine map has no name for it): stores `key` at `+4` and `value` at `+8`
/// of the entry `item`.
pub fn fn_00863840(e: &mut Engine, _this: Ptr<NiTPointerMap>, item: Ptr, key: u32, value: u32) {
    e.mem.set_u32(item.addr() + 4, key);
    e.mem.set_u32(item.addr() + 8, value);
}

// Translated from 00863860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the exterior cells' map: vtable `01081e70`, `RemoveAll`
/// (`00438af0`), then the root destructor `008638c0`. Not translated: the
/// exception frame.
pub fn fn_00863860(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), EXTERIOR_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_008638c0(e, this);
}

// Translated from 008638c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the exterior cells' map: vtable `01081f38`,
/// `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_008638c0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    destruct_map_root(e, this, EXTERIOR_MAP_ROOT_VTABLE);
}

/// Shared by the root destructors: sets the root vtable, empties the map and
/// frees the bucket array.
fn destruct_map_root(e: &mut Engine, this: Ptr<NiTPointerMap>, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
}

// Translated from 008638f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the numeric id buffer map: `fn_008635d0` with the
/// vtable `01081f58`.
pub fn fn_008638f0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, NUMERIC_ID_MAP_ROOT_VTABLE)
}

// Translated from 00863960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the numeric id buffer map: vtable `01081e90`,
/// `RemoveAll`, then the root destructor `008639c0`. Not translated: the
/// exception frame.
pub fn fn_00863960(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), NUMERIC_ID_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_008639c0(e, this);
}

// Translated from 008639c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the numeric id buffer map: vtable `01081f58`,
/// `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_008639c0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    destruct_map_root(e, this, NUMERIC_ID_MAP_ROOT_VTABLE);
}

// Translated from 008639f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the map with `unsigned char` keys: `fn_008635d0`
/// with the vtable `01081f78`.
pub fn fn_008639f0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, STATS_MAP_ROOT_VTABLE)
}

// Translated from 00863a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetAt` of the map with `unsigned char` keys (`NiTMapBase<..., unsigned
/// char, BSSimpleList<LoadFormHeader *> *>`; the engine map has no name for
/// it): sets the value of `key`, adding an entry at the head of its bucket
/// when the key is new. Virtual slots used: `KeyToHashIndex` (4),
/// `IsKeysEqual` (8), `SetValue` (0xC), `NewItem` (0x14).
pub fn fn_00863a60(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u8, value: u32) {
    let index = e
        .vcall(this.addr(), MAP_SLOT_KEY_TO_HASH_INDEX, &args![key as u32])
        .u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        // The entry's next is at +0, its key byte at +4, its value at +8.
        let item_key = e.mem.u8(item + 4) as u32;
        let equal = e
            .vcall(
                this.addr(),
                MAP_SLOT_IS_KEYS_EQUAL,
                &args![key as u32, item_key],
            )
            .bool();
        if equal {
            e.mem.set_u32(item + 8, value);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(this.addr(), MAP_SLOT_NEW_ITEM, &args![]).u32();
    e.vcall(
        this.addr(),
        MAP_SLOT_SET_VALUE,
        &args![item, key as u32, value],
    );
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let slot = table.wrapping_add(index.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(item, head);
    e.mem.set_u32(slot, item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 00863b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetAt` of the map with `unsigned char` keys (the engine map has no name
/// for it): looks `key` up in its bucket; when found stores the value
/// through `value_out` and returns true.
pub fn fn_00863b40(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u8, value_out: Ptr) -> bool {
    let index = e
        .vcall(this.addr(), MAP_SLOT_KEY_TO_HASH_INDEX, &args![key as u32])
        .u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u8(item + 4) as u32;
        let equal = e
            .vcall(
                this.addr(),
                MAP_SLOT_IS_KEYS_EQUAL,
                &args![key as u32, item_key],
            )
            .bool();
        if equal {
            let value = e.mem.u32(item + 8);
            e.mem.set_u32(value_out.addr(), value);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 00863bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetNext` of the map with `unsigned char` keys (by the body): gives the
/// key and value of the entry `*position` and moves `*position` to the next
/// entry (the next of its bucket, else the first of the following non-empty
/// bucket, else null).
pub fn fn_00863bc0(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    position: Ptr,
    key_out: Ptr,
    value_out: Ptr,
) {
    let item = e.mem.u32(position.addr());
    let key = e.mem.u8(item + 4);
    e.mem.set_u8(key_out.addr(), key);
    let value = e.mem.u32(item + 8);
    e.mem.set_u32(value_out.addr(), value);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let mut index = e
        .vcall(this.addr(), MAP_SLOT_KEY_TO_HASH_INDEX, &args![key as u32])
        .u32()
        .wrapping_add(1);
    while index < e.get(this, NiTPointerMap::m_uiHashSize) {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let entry = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if entry != 0 {
            e.mem.set_u32(position.addr(), entry);
            return;
        }
        index += 1;
    }
    e.mem.set_u32(position.addr(), 0);
}

// Translated from 00863c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned char,
/// BSSimpleList<LoadFormHeader *> *>::KeyToHashIndex` (Xbox PDB): the key
/// modulo the bucket count (a zero bucket count would fault the CPU).
pub fn ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_key_to_hash_index(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    key: u8,
) -> u32 {
    let hash_size = e.get(this, NiTPointerMap::m_uiHashSize);
    key as u32 % hash_size
}

// Translated from 00863c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned char,
/// BSSimpleList<LoadFormHeader *> *>::IsKeysEqual` (Xbox PDB): the two key
/// bytes are equal.
pub fn ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_is_keys_equal(
    _e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    first: u8,
    second: u8,
) -> bool {
    first == second
}

// Translated from 00863cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned char,
/// BSSimpleList<LoadFormHeader *> *>::SetValue` (Xbox PDB): stores the key
/// byte at `+4` and the value at `+8` of the entry `item`.
pub fn ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_set_value(
    e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    item: Ptr,
    key: u8,
    value: u32,
) {
    e.mem.set_u8(item.addr() + 4, key);
    e.mem.set_u32(item.addr() + 8, value);
}

// Translated from 00863cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the map with `unsigned char` keys: vtable `01081eb0`,
/// `RemoveAll`, then the root destructor `00863d30`. Not translated: the
/// exception frame.
pub fn fn_00863cd0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), STATS_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00863d30(e, this);
}

// Translated from 00863d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the map with `unsigned char` keys: vtable
/// `01081f78`, `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_00863d30(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    destruct_map_root(e, this, STATS_MAP_ROOT_VTABLE);
}

// Translated from 00863d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTLargePrimitiveArray<FormAndFlags *>`: vtable
/// `01081f98`, then the element block (`m_pBase`) is released (`004ede70`).
pub fn fn_00863d60(e: &mut Engine, this: Ptr<NiTLargePrimitiveArray>) {
    e.mem.set_u32(this.addr(), FORM_ARRAY_VTABLE);
    let base = e.get(this, NiTLargePrimitiveArray::m_pBase);
    e.call(ARRAY_FREE_BLOCK, &args![base]);
}

// Translated from 00863d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Add` of the `NiTLargePrimitiveArray<FormAndFlags *>` (the engine map has
/// no name for it): `00864240(this, m_uiSize, item)` appends `item` at the
/// current size, answering what that gives (the index: the exe ends on that
/// call and leaves its `EAX`, which the callers keep).
pub fn fn_00863d90(e: &mut Engine, this: Ptr<NiTLargePrimitiveArray>, item: u32) -> u32 {
    let size = e.get(this, NiTLargePrimitiveArray::m_uiSize);
    e.call(ARRAY_ADD_AT, &args![this, size, item]).u32()
}

// Translated from 00863db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cleanup of the `NiTLargePrimitiveArray<FormAndFlags *>` before it is
/// deleted: the first `m_uiSize` element slots are set to null, then the
/// size and the effective size are zero.
pub fn fn_00863db0(e: &mut Engine, this: Ptr<NiTLargePrimitiveArray>) {
    let base = e.get(this, NiTLargePrimitiveArray::m_pBase);
    let mut index = 0u32;
    while index < e.get(this, NiTLargePrimitiveArray::m_uiSize) {
        e.mem.set_u32(base.wrapping_add(index.wrapping_mul(4)), 0);
        index += 1;
    }
    e.set(this, NiTLargePrimitiveArray::m_uiSize, 0);
    e.set(this, NiTLargePrimitiveArray::m_uiESize, 0);
}

// Translated from 00863e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `NiTLargePrimitiveArray<FormAndFlags *>`: the
/// base constructor `008641d0(max size, grow by)`, then the vtable
/// `01081fa0`.
pub fn fn_00863e00(
    e: &mut Engine,
    this: Ptr<NiTLargePrimitiveArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTLargePrimitiveArray> {
    fn_008641d0(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), FORM_ARRAY_DERIVED_VTABLE);
    this
}

// Translated from 00863e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root constructor of the map `fn_008627b0` keeps on its stack:
/// `fn_008635d0` with the vtable `01081fa8`.
pub fn fn_00863e30(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_root(e, this, hash_size, LOCAL_MAP_ROOT_VTABLE)
}

// Translated from 00863ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the map `fn_008627b0` keeps on its stack: vtable
/// `01081ed0`, `RemoveAll`, then the root destructor `00863f00`. Not
/// translated: the exception frame.
pub fn fn_00863ea0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), LOCAL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00863f00(e, this);
}

// Translated from 00863f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root destructor of the map `fn_008627b0` keeps on its stack: vtable
/// `01081fa8`, `RemoveAll`, then `NiFree` of the bucket array.
pub fn fn_00863f00(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    destruct_map_root(e, this, LOCAL_MAP_ROOT_VTABLE);
}

// Translated from 00863f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `NiTLargePrimitiveArray`: its
/// destructor body `008640e0`, then the block is freed when bit 0 of `flags`
/// is set.
pub fn fn_00863f30(
    e: &mut Engine,
    this: Ptr<NiTLargePrimitiveArray>,
    flags: u32,
) -> Ptr<NiTLargePrimitiveArray> {
    fn_008640e0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,ChangeData *>::
/// scalar deleting destructor` (Xbox PDB): the root destructor `008636a0`,
/// then the block is freed when bit 0 of `flags` is set.
pub fn ni_t_map_base_unsigned_int_change_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_008636a0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,
/// BSSimpleList<unsigned int> *>::scalar deleting destructor` (Xbox PDB): the
/// root destructor `008637a0`, then the block is freed when bit 0 of `flags`
/// is set.
pub fn ni_t_map_base_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_008637a0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,
/// BSSimpleList<ExteriorCellReferenceData *> *>::scalar deleting destructor`
/// (Xbox PDB): the root destructor `008638c0`, then the block is freed when
/// bit 0 of `flags` is set.
pub fn ni_t_map_base_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_008638c0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00863ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,void *>::
/// scalar deleting destructor` (Xbox PDB): the root destructor `008639c0`,
/// then the block is freed when bit 0 of `flags` is set.
pub fn ni_t_map_base_unsigned_int_void_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_008639c0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00864020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned char,
/// BSSimpleList<LoadFormHeader *> *>::scalar deleting destructor` (Xbox PDB):
/// the root destructor `00863d30`, then the block is freed when bit 0 of
/// `flags` is set.
pub fn ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00863d30(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 008640e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the `NiTLargePrimitiveArray`: runs `00864100` on
/// the object.
pub fn fn_008640e0(e: &mut Engine, this: Ptr<NiTLargePrimitiveArray>) {
    e.call(ARRAY_DESTRUCT_BODY, &args![this]);
}

// Translated from 00864160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of a `NiTLargePrimitiveArray` (vtable `01081fc8`):
/// `m_uiMaxSize` and `m_uiGrowBy` are the arguments, the size and effective
/// size are zero and the element block is allocated (`0096afc0`, with
/// `max_size` as the code passes it) unless `max_size` is 0.
pub fn fn_00864160(e: &mut Engine, this: Ptr, max_size: u32, grow_by: u32) -> Ptr {
    construct_large_array(e, this.cast(), max_size, grow_by, ARRAY_BASE_VTABLE).cast()
}

// Translated from 008641d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of the `NiTLargePrimitiveArray<FormAndFlags *>`:
/// as `fn_00864160` with the vtable `01081f98`.
pub fn fn_008641d0(
    e: &mut Engine,
    this: Ptr<NiTLargePrimitiveArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTLargePrimitiveArray> {
    construct_large_array(e, this, max_size, grow_by, FORM_ARRAY_VTABLE)
}

/// Shared by `fn_00864160` and `fn_008641d0` (identical code, another
/// vtable).
fn construct_large_array(
    e: &mut Engine,
    this: Ptr<NiTLargePrimitiveArray>,
    max_size: u32,
    grow_by: u32,
    vtable: u32,
) -> Ptr<NiTLargePrimitiveArray> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTLargePrimitiveArray::m_uiMaxSize, max_size);
    e.set(this, NiTLargePrimitiveArray::m_uiGrowBy, grow_by);
    e.set(this, NiTLargePrimitiveArray::m_uiSize, 0);
    e.set(this, NiTLargePrimitiveArray::m_uiESize, 0);
    let base = if max_size > 0 {
        e.call(ARRAY_ALLOC_BLOCK, &args![max_size]).u32()
    } else {
        0
    };
    e.set(this, NiTLargePrimitiveArray::m_pBase, base);
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008616f0,
            tes_save_load_game_get_saving_allowed(Ptr<TESSaveLoadGame>) -> bool
        ),
        entry!(
            0x00861780,
            tes_save_load_game_add_created_base_object(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00861820,
            tes_save_load_game_save_created_base_objects(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00861b70,
            tes_save_load_game_add_numeric_id_to_array(Ptr<TESSaveLoadGame>, u32) -> u32
        ),
        entry!(0x00861c00, fn_00861c00(Ptr<TESSaveLoadGame>, u32) -> u32),
        entry!(0x00861c60, fn_00861c60(Ptr<TESSaveLoadGame>, u32) -> u16),
        entry!(0x00861cd0, fn_00861cd0(Ptr<TESSaveLoadGame>, u16) -> u32),
        entry!(
            0x00861d10,
            tes_save_load_game_save_numeric_id_arrays(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(0x00861ea0, fn_00861ea0(Ptr<TESSaveLoadGame>)),
        entry!(0x00861ee0, fn_00861ee0(Ptr<TESSaveLoadGame>, u8)),
        entry!(
            0x00861f20,
            tes_save_load_game_test_all_cells(Ptr<TESSaveLoadGame>, u32)
        ),
        entry!(0x008620f0, fn_008620f0(Ptr<TESSaveLoadGame>)),
        entry!(
            0x00862110,
            tes_save_load_game_use_save_game_blocks(Ptr<TESSaveLoadGame>) -> bool
        ),
        entry!(0x00862150, fn_00862150(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(0x008622a0, fn_008622a0(Ptr<TESSaveLoadGame>) -> u32),
        entry!(0x00862370, fn_00862370(Ptr<TESSaveLoadGame>) -> u32),
        entry!(0x008623a0, fn_008623a0(Ptr<TESSaveLoadGame>, u32)),
        entry!(0x00862430, fn_00862430(Ptr<TESSaveLoadGame>, Ptr) -> bool),
        entry!(0x008624e0, fn_008624e0(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(0x008627b0, fn_008627b0(Ptr<TESSaveLoadGame>)),
        entry!(0x008632a0, fn_008632a0(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(
            0x00863360,
            fn_00863360(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863390,
            fn_00863390(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x008633c0,
            fn_008633c0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x008633f0,
            fn_008633f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863420,
            fn_00863420(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863450, fn_00863450(Ptr, u32) -> Ptr),
        entry!(
            0x00863480,
            ni_t_pointer_map_unsigned_int_change_data_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x008634b0,
            ni_t_pointer_map_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x008634e0,
            ni_t_pointer_map_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863510,
            ni_t_pointer_map_unsigned_int_void_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863540,
            ni_t_pointer_map_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863570, fn_00863570(Ptr, u32) -> Ptr),
        entry!(0x008635a0, fn_008635a0(Ptr, u32, u32) -> Ptr),
        entry!(
            0x008635d0,
            fn_008635d0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863640, fn_00863640(Ptr<NiTPointerMap>)),
        entry!(0x008636a0, fn_008636a0(Ptr<NiTPointerMap>)),
        entry!(
            0x008636d0,
            fn_008636d0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863740, fn_00863740(Ptr<NiTPointerMap>)),
        entry!(0x008637a0, fn_008637a0(Ptr<NiTPointerMap>)),
        entry!(
            0x008637d0,
            fn_008637d0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863840, fn_00863840(Ptr<NiTPointerMap>, Ptr, u32, u32)),
        entry!(0x00863860, fn_00863860(Ptr<NiTPointerMap>)),
        entry!(0x008638c0, fn_008638c0(Ptr<NiTPointerMap>)),
        entry!(
            0x008638f0,
            fn_008638f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863960, fn_00863960(Ptr<NiTPointerMap>)),
        entry!(0x008639c0, fn_008639c0(Ptr<NiTPointerMap>)),
        entry!(
            0x008639f0,
            fn_008639f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863a60, fn_00863a60(Ptr<NiTPointerMap>, u8, u32)),
        entry!(0x00863b40, fn_00863b40(Ptr<NiTPointerMap>, u8, Ptr) -> bool),
        entry!(0x00863bc0, fn_00863bc0(Ptr<NiTPointerMap>, Ptr, Ptr, Ptr)),
        entry!(
            0x00863c70,
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_key_to_hash_index(
                Ptr<NiTPointerMap>,
                u8
            ) -> u32
        ),
        entry!(
            0x00863c90,
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_is_keys_equal(
                Ptr<NiTPointerMap>,
                u8,
                u8
            ) -> bool
        ),
        entry!(
            0x00863cb0,
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_set_value(
                Ptr<NiTPointerMap>,
                Ptr,
                u8,
                u32
            )
        ),
        entry!(0x00863cd0, fn_00863cd0(Ptr<NiTPointerMap>)),
        entry!(0x00863d30, fn_00863d30(Ptr<NiTPointerMap>)),
        entry!(0x00863d60, fn_00863d60(Ptr<NiTLargePrimitiveArray>)),
        entry!(0x00863d90, fn_00863d90(Ptr<NiTLargePrimitiveArray>, u32) -> u32),
        entry!(0x00863db0, fn_00863db0(Ptr<NiTLargePrimitiveArray>)),
        entry!(
            0x00863e00,
            fn_00863e00(Ptr<NiTLargePrimitiveArray>, u32, u32) -> Ptr<NiTLargePrimitiveArray>
        ),
        entry!(
            0x00863e30,
            fn_00863e30(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00863ea0, fn_00863ea0(Ptr<NiTPointerMap>)),
        entry!(0x00863f00, fn_00863f00(Ptr<NiTPointerMap>)),
        entry!(
            0x00863f30,
            fn_00863f30(Ptr<NiTLargePrimitiveArray>, u32) -> Ptr<NiTLargePrimitiveArray>
        ),
        entry!(
            0x00863f60,
            ni_t_map_base_unsigned_int_change_data_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863f90,
            ni_t_map_base_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863fc0,
            ni_t_map_base_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00863ff0,
            ni_t_map_base_unsigned_int_void_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00864020,
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(0x008640e0, fn_008640e0(Ptr<NiTLargePrimitiveArray>)),
        entry!(0x00864160, fn_00864160(Ptr, u32, u32) -> Ptr),
        entry!(
            0x008641d0,
            fn_008641d0(Ptr<NiTLargePrimitiveArray>, u32, u32) -> Ptr<NiTLargePrimitiveArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// A double that returns `value` whatever it is called with.
    fn constant(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| returns(value));
    }

    /// An engine with the globals this part reads mapped, the game's own
    /// `TESSaveLoadGame` in its global, and working doubles for the small
    /// callees nearly every function uses. Calls are logged.
    fn game() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_e000,
            0x011c_3000,
            0x011c_a000,
            0x011f_4000,
            0x0101_7000,
            0x0101_6000,
            0x0101_3000,
            0x0108_1000,
            0x0107_f000,
            0x0108_0000,
            0x0103_4000,
        ] {
            e.map(page, 0x1000);
        }
        let singleton = e.new_object::<TESSaveLoadGame>();
        e.set_global(SAVE_LOAD_GAME, singleton.addr());
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0xC)));
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(LIST_NODE_ITEM, |_, a| returns(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(0));
        e.register(SIMPLE_LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            returns(a[0])
        });
        e.register(STRLEN, |e, a| returns(e.mem.cstr(a[0]).len() as u32));
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_FIND, |e, a| {
            let (text, pattern) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let found = text
                .windows(pattern.len().max(1))
                .position(|window| window == pattern.as_slice());
            returns(found.map_or(0, |at| a[0] + at as u32))
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        stub(&mut e, SCOPE_ENTER);
        stub(&mut e, SCOPE_LEAVE);
        stub(&mut e, LOG_ERROR);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn game_object(e: &mut Engine) -> Ptr<TESSaveLoadGame> {
        e.new_object::<TESSaveLoadGame>()
    }

    /// A form-like block: its id at `+0x0C`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0xC, id);
        form
    }

    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(offset, target) in slots {
            e.mem.set_u32(vtable + offset, target);
        }
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// A `NiTLargePrimitiveArray` stand-in: the count at `+0xC`, the items
    /// behind the pointer at `+0x10`; its element address (`00877a30`) and
    /// append (`00863d90`) are doubles shared by every array.
    fn install_arrays(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| returns(e.mem.u32(a[0] + 0xC)));
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            returns(e.mem.u32(a[0] + 0x10) + 4 * a[1])
        });
        e.register(INIT_ARRAY_ADD, |e, a| {
            let count = e.mem.u32(a[0] + 0xC);
            let data = e.mem.u32(a[0] + 0x10);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(data + 4 * count, value);
            e.mem.set_u32(a[0] + 0xC, count + 1);
            returns(count)
        });
    }

    fn make_array(e: &mut Engine, items: &[u32]) -> u32 {
        let array = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x100);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(array + 0xC, items.len() as u32);
        e.mem.set_u32(array + 0x10, data);
        array
    }

    fn array_items(e: &Engine, array: u32) -> Vec<u32> {
        let count = e.mem.u32(array + 0xC);
        let data = e.mem.u32(array + 0x10);
        (0..count).map(|i| e.mem.u32(data + 4 * i)).collect()
    }

    /// The data handler "knows as created" every id from `0xFF000000` up.
    fn install_created_ids(e: &mut Engine) {
        e.register(DATA_HANDLER_HAS_FORM, |_, a| {
            returns((a[1] >= 0xFF00_0000) as u32)
        });
    }

    // ---- 008616f0 ----

    #[test]
    fn saving_is_allowed_unless_the_player_or_a_menu_forbids_it() {
        let allowed = |state: u32, flags: u32, forced: u32, top: u32, mode: u32| {
            let mut e = game();
            e.set_global(PLAYER, 0x0123_4560u32);
            constant(&mut e, PLAYER_STATE_004F8960, state);
            constant(&mut e, PLAYER_FLAGS_SET_0093A740, flags);
            constant(&mut e, SAVING_FORCED_004A4040, forced);
            constant(&mut e, TOP_MENU_ID, top);
            constant(&mut e, MENU_MODE_TYPE, mode);
            let this = game_object(&mut e);
            let result = tes_save_load_game_get_saving_allowed(&mut e, this);
            (result, calls_to(&e, PLAYER_FLAGS_SET_0093A740).len())
        };
        assert_eq!(allowed(0, 0, 0, 0, 0), (true, 1));
        // States 2 and 1 stop at once, without asking about the flags.
        assert_eq!(allowed(2, 0, 0, 0, 0), (false, 0));
        assert_eq!(allowed(1, 0, 0, 0, 0), (false, 0));
        assert!(!allowed(0, 1, 1, 0, 0).0);
        // The forced answer wins over the menus.
        assert!(allowed(0, 0, 1, 7, 7).0);
        // Only the menu ids 0 and 3 allow it.
        assert!(allowed(0, 0, 0, 3, 3).0);
        assert!(!allowed(0, 0, 0, 5, 0).0);
        assert!(!allowed(0, 0, 0, 0, 4).0);
    }

    // ---- 00861780 ----

    #[test]
    fn a_created_base_object_is_added_once() {
        let mut e = game();
        install_created_ids(&mut e);
        let added: Rc<RefCell<Vec<u32>>> = Rc::default();
        let list = added.clone();
        e.register_double(LIST_CONTAINS, move |e, a| {
            returns(list.borrow().contains(&e.mem.u32(a[1])) as u32)
        });
        let list = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            list.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let this = game_object(&mut e);

        tes_save_load_game_add_created_base_object(&mut e, this, Ptr::NULL);
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_NULL_CREATED_BASE_OBJECT]]
        );

        let plain = form_with_id(&mut e, 0x100);
        tes_save_load_game_add_created_base_object(&mut e, this, Ptr::new(plain));
        assert_eq!(
            calls_to(&e, LOG_ERROR)[1],
            vec![MSG_NON_CREATED_BASE_OBJECT, 0x100]
        );
        assert!(added.borrow().is_empty());

        let created = form_with_id(&mut e, 0xFF00_0001);
        tes_save_load_game_add_created_base_object(&mut e, this, Ptr::new(created));
        tes_save_load_game_add_created_base_object(&mut e, this, Ptr::new(created));
        assert_eq!(*added.borrow(), vec![0xFF00_0001]);
        let adds = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], this.addr() + CREATED_BASE_OBJECTS_LIST);
        assert_eq!(calls_to(&e, LOG_ERROR).len(), 2);
    }

    // ---- 00861820 ----

    type Writes = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// Forms for `SaveCreatedBaseObjects`: a bound object (id 0x1001), a
    /// spell item that is enchantable with the created enchantment 0xFF000005
    /// (0x1002) and a bound object that is enchantable without an enchantment
    /// (0x1003); the file writes made are collected in `writes`.
    struct CreatedForms {
        writes: Writes,
        plain: u32,
        spell: u32,
        enchantment: u32,
    }

    fn created_forms(e: &mut Engine) -> CreatedForms {
        install_created_ids(e);
        let fill = 0x0390_0010;
        stub(e, fill);
        let make = |e: &mut Engine, id: u32, fill_byte: u8, size: u32| {
            let form = object_with_slots(e, &[(FORM_FILL_BUFFER_SLOT, fill)]);
            e.mem.set_u32(form + 0xC, id);
            let buffer = e.mem.alloc(size);
            e.mem.write(buffer, &vec![fill_byte; size as usize]);
            e.mem.set_u32(form + 0x10, size);
            e.mem.set_u32(form + 0x14, buffer);
            e.mem.set_u8(form + 4, 0x2A);
            form
        };
        let plain = make(e, 0x1001, 0xA1, 3);
        let spell = make(e, 0x1002, 0xB2, 5);
        let third = make(e, 0x1003, 0xC3, 2);
        let enchantment = make(e, 0xFF00_0005, 0xE5, 4);
        let spell_part = e.mem.alloc(8);
        e.mem.set_u32(spell_part + 4, enchantment);
        let third_part = e.mem.alloc(8);
        let table: HashMap<u32, u32> = [(0x1001, plain), (0x1002, spell), (0x1003, third)]
            .into_iter()
            .collect();
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(table.get(&a[0]).copied().unwrap_or(0))
        });
        let casts = [
            (plain, RTTI_BOUND_OBJECT, plain),
            (spell, RTTI_SPELL_ITEM, spell),
            (spell, RTTI_ENCHANTABLE_FORM, spell_part),
            (third, RTTI_BOUND_OBJECT, third),
            (third, RTTI_ENCHANTABLE_FORM, third_part),
        ];
        e.register_double(DYNAMIC_CAST, move |_, a| {
            returns(
                casts
                    .iter()
                    .find(|(object, target, _)| *object == a[0] && *target == a[3])
                    .map_or(0, |(_, _, result)| *result),
            )
        });
        e.register(FORM_BUFFER_SIZE, |e, a| returns(e.mem.u32(a[0] + 0x10)));
        e.register(FORM_BUFFER_ADDRESS, |e, a| returns(e.mem.u32(a[0] + 0x14)));
        e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        stub(e, FORM_FREE_BUFFER);
        let writes: Writes = Rc::default();
        let log = writes.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
            returns(a[2])
        });
        CreatedForms {
            writes,
            plain,
            spell,
            enchantment,
        }
    }

    #[test]
    fn the_created_base_objects_are_counted_and_written_with_their_enchantments() {
        let mut e = game();
        let forms = created_forms(&mut e);
        let this = game_object(&mut e);
        let list = this.addr() + CREATED_BASE_OBJECTS_LIST;
        // The embedded head node holds the first id, a second node the next.
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1001);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, 0x1002);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 0x1003);
        let file = 0x0BAD_F000;

        tes_save_load_game_save_created_base_objects(&mut e, this, Ptr::new(file));

        let writes = forms.writes.borrow();
        // The count: the plain form 1, the spell 2 (itself and its created
        // enchantment); the enchantable form without an enchantment none.
        assert_eq!(writes[0], (file, 3u32.to_le_bytes().to_vec()));
        // Then the plain form, the spell's enchantment and the spell.
        assert_eq!(writes[1], (file, vec![0xA1; 3]));
        assert_eq!(writes[2], (file, vec![0xE5; 4]));
        assert_eq!(writes[3], (file, vec![0xB2; 5]));
        assert_eq!(writes.len(), 4);
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_ENCHANTABLE_WITHOUT_ENCHANTMENT, 0x1003]]
        );
        let freed: Vec<u32> = calls_to(&e, FORM_FREE_BUFFER)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(freed, vec![forms.plain, forms.enchantment, forms.spell]);
    }

    #[test]
    fn nothing_but_the_count_is_written_when_no_created_object_counts() {
        let mut e = game();
        let forms = created_forms(&mut e);
        let this = game_object(&mut e);
        let list = this.addr() + CREATED_BASE_OBJECTS_LIST;
        // An id no form has, then the enchantable form without enchantment.
        let second = e.mem.alloc(8);
        e.mem.set_u32(list, 0x9999);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, 0x1003);

        tes_save_load_game_save_created_base_objects(&mut e, this, Ptr::new(0x0BAD_F000));

        assert_eq!(*forms.writes.borrow(), vec![(0x0BAD_F000, vec![0; 4])]);
        assert!(calls_to(&e, LOG_ERROR).is_empty());
    }

    #[test]
    fn the_statistics_note_each_written_form() {
        let mut e = game();
        let forms = created_forms(&mut e);
        // The statistics add: type 0x2A headers with their ids and sizes.
        e.register(BYTE_MAP_GET_AT, |_, _| returns(0));
        stub(&mut e, BYTE_MAP_SET_AT);
        stub(&mut e, LIST_INSERT);
        let this = game_object(&mut e);
        let stats = e.new_object::<SaveStats>();
        e.set(this, TESSaveLoadGame::m_pSaveLoadStats, stats);
        e.set(stats, SaveStats::pStatsMap, Ptr::new(0x0777_7000));
        let list = this.addr() + CREATED_BASE_OBJECTS_LIST;
        let second = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1001);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, 0x1002);

        tes_save_load_game_save_created_base_objects(&mut e, this, Ptr::new(0x0BAD_F000));

        // One header per record: the plain form, the enchantment, the spell.
        let inserted: Vec<(u32, u16)> = calls_to(&e, LIST_INSERT)
            .iter()
            .map(|c| (e.mem.u32(c[1]), e.mem.u16(c[1] + 0xA)))
            .collect();
        assert_eq!(inserted, vec![(0x1001, 3), (0xFF00_0005, 4), (0x1002, 5)]);
        assert_eq!(forms.writes.borrow().len(), 4);
    }

    // ---- 00861b70, 00861c00, 00861c60, 00861cd0 ----

    #[test]
    fn a_numeric_id_is_an_index_into_the_array_unless_it_is_created() {
        let mut e = game();
        install_arrays(&mut e);
        install_created_ids(&mut e);
        let this = game_object(&mut e);
        let array = make_array(&mut e, &[0x10, 0x20]);
        e.mem.set_u32(this.addr() + NUMERIC_ID_ARRAY, array);

        assert_eq!(
            tes_save_load_game_add_numeric_id_to_array(&mut e, this, 0xFF00_0007),
            0xFF00_0007
        );
        assert_eq!(
            tes_save_load_game_add_numeric_id_to_array(&mut e, this, 0x20),
            1
        );
        assert_eq!(array_items(&e, array), vec![0x10, 0x20]);
        assert_eq!(
            tes_save_load_game_add_numeric_id_to_array(&mut e, this, 0x30),
            2
        );
        assert_eq!(array_items(&e, array), vec![0x10, 0x20, 0x30]);
        // The id is passed to the append by address, in a cell of its own.
        assert_eq!(calls_to(&e, INIT_ARRAY_ADD).len(), 1);
    }

    #[test]
    fn a_numeric_id_resolves_back_to_the_id() {
        let mut e = game();
        install_arrays(&mut e);
        install_created_ids(&mut e);
        let this = game_object(&mut e);
        let array = make_array(&mut e, &[0x10, 0x20]);
        // The slot just past the end is readable and is read for number ==
        // count (the test is `>`).
        let data = e.mem.u32(array + 0x10);
        e.mem.set_u32(data + 8, 0xDEAD);
        e.mem.set_u32(this.addr() + NUMERIC_ID_ARRAY, array);

        assert_eq!(fn_00861c00(&mut e, this, 0xFF00_0001), 0xFF00_0001);
        assert_eq!(fn_00861c00(&mut e, this, 0), 0x10);
        assert_eq!(fn_00861c00(&mut e, this, 1), 0x20);
        assert_eq!(fn_00861c00(&mut e, this, 2), 0xDEAD);
        assert_eq!(fn_00861c00(&mut e, this, 3), 0);
    }

    #[test]
    fn a_world_space_id_is_found_or_appended() {
        let mut e = game();
        install_arrays(&mut e);
        let this = game_object(&mut e);
        let array = make_array(&mut e, &[0x3000, 0x3001]);
        e.mem.set_u32(this.addr() + WORLDSPACE_ID_ARRAY, array);

        assert_eq!(fn_00861c60(&mut e, this, 0x3001), 1);
        assert_eq!(fn_00861c60(&mut e, this, 0x3000), 0);
        assert_eq!(array_items(&e, array).len(), 2);
        assert_eq!(fn_00861c60(&mut e, this, 0x3002), 2);
        assert_eq!(array_items(&e, array), vec![0x3000, 0x3001, 0x3002]);
    }

    #[test]
    fn a_world_space_index_gives_the_id_or_zero_past_the_count() {
        let mut e = game();
        install_arrays(&mut e);
        let this = game_object(&mut e);
        let array = make_array(&mut e, &[0x3000, 0x3001]);
        let data = e.mem.u32(array + 0x10);
        e.mem.set_u32(data + 8, 0xBEEF);
        e.mem.set_u32(this.addr() + WORLDSPACE_ID_ARRAY, array);

        assert_eq!(fn_00861cd0(&mut e, this, 1), 0x3001);
        assert_eq!(fn_00861cd0(&mut e, this, 2), 0xBEEF);
        assert_eq!(fn_00861cd0(&mut e, this, 3), 0);
    }

    // ---- 00861d10 ----

    fn install_file_writes(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let words: Rc<RefCell<Vec<u32>>> = Rc::default();
        let log = words.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            returns(a[2])
        });
        words
    }

    #[test]
    fn the_numeric_id_arrays_are_written_with_their_counts() {
        let mut e = game();
        install_arrays(&mut e);
        let words = install_file_writes(&mut e);
        let this = game_object(&mut e);
        let numeric = make_array(&mut e, &[0x11, 0x22]);
        let worldspaces = make_array(&mut e, &[0x3000]);
        e.mem.set_u32(this.addr() + NUMERIC_ID_ARRAY, numeric);
        e.mem
            .set_u32(this.addr() + WORLDSPACE_ID_ARRAY, worldspaces);

        tes_save_load_game_save_numeric_id_arrays(&mut e, this, Ptr::new(0x0BAD_F000));

        assert_eq!(*words.borrow(), vec![2, 0x11, 0x22, 1, 0x3000]);
        assert!(calls_to(&e, FORMAT).is_empty());
    }

    #[test]
    fn the_numeric_id_arrays_are_noted_in_the_statistics() {
        let mut e = game();
        install_arrays(&mut e);
        install_file_writes(&mut e);
        e.mem
            .set_cstr(LABEL_NUMERIC_ID_ARRAY, b"Numeric ID Array(%i)");
        e.mem
            .set_cstr(LABEL_WORLDSPACE_ID_ARRAY, b"WorldSpace ID Array(%i)");
        e.register(FORMAT, |e, a| {
            let formatted = String::from_utf8(e.mem.cstr(a[2]))
                .unwrap()
                .replace("%i", &a[3].to_string());
            e.mem.set_cstr(a[0], formatted.as_bytes());
            Ret::default()
        });
        let stats_seen: Rc<RefCell<Vec<(u32, String)>>> = Rc::default();
        let seen = stats_seen.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            let stat = e.mem.u32(a[1]);
            let description = e.mem.u32(stat + 4);
            seen.borrow_mut().push((
                e.mem.u32(stat),
                String::from_utf8(e.mem.cstr(description)).unwrap(),
            ));
            Ret::default()
        });
        let this = game_object(&mut e);
        let stats = e.new_object::<SaveStats>();
        e.set(stats, SaveStats::pExtraStats, Ptr::new(0x0666_6000));
        e.set(this, TESSaveLoadGame::m_pSaveLoadStats, stats);
        let numeric = make_array(&mut e, &[0x11, 0x22]);
        let worldspaces = make_array(&mut e, &[0x3000]);
        e.mem.set_u32(this.addr() + NUMERIC_ID_ARRAY, numeric);
        e.mem
            .set_u32(this.addr() + WORLDSPACE_ID_ARRAY, worldspaces);

        tes_save_load_game_save_numeric_id_arrays(&mut e, this, Ptr::new(0x0BAD_F000));

        assert_eq!(
            *stats_seen.borrow(),
            vec![
                (12, "Numeric ID Array(2)".to_string()),
                (8, "WorldSpace ID Array(1)".to_string())
            ]
        );
    }

    // ---- 00861ea0 ----

    #[test]
    fn the_load_screen_is_poked_after_three_seconds_without_a_loading_menu() {
        let run = |now: u32, visible: u32| {
            let mut e = game();
            e.set_global(LAST_POKE_TICK, 1000u32);
            e.set_global(TES_OBJECT, 0x0555_5000u32);
            constant(&mut e, GET_TICK_COUNT_IMPORT, now);
            constant(&mut e, LOADING_MENU_VISIBLE, visible);
            stub(&mut e, TES_LOAD_SCREEN_POKE);
            fn_00861ea0(&mut e, Ptr::new(0));
            calls_to(&e, TES_LOAD_SCREEN_POKE)
        };
        assert_eq!(run(4001, 0), vec![vec![0x0555_5000, 1, 0, 0]]);
        // Exactly 3000 ms later is not enough, and a visible menu stops it.
        assert!(run(4000, 0).is_empty());
        assert!(run(9000, 1).is_empty());
    }

    // ---- 00861ee0 ----

    #[test]
    fn the_current_version_is_stored_and_an_old_one_warned_about() {
        let mut e = game();
        let this = game_object(&mut e);
        fn_00861ee0(&mut e, this, 0x13);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cCurrentVersion), 0x13);
        assert!(calls_to(&e, LOG_ERROR).is_empty());
        fn_00861ee0(&mut e, this, 0x12);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cCurrentVersion), 0x12);
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_OLD_LOAD_VERSION, 0x12, 0x13]]
        );
    }

    // ---- 00861f20 ----

    #[test]
    fn test_all_cells_only_acts_in_modes_four_and_five() {
        let mut e = game();
        e.set_global(TEST_ALL_CELLS_COUNTER, 7i32);
        let this = game_object(&mut e);
        tes_save_load_game_test_all_cells(&mut e, this, 3);
        assert_eq!(e.global::<i32>(TEST_ALL_CELLS_COUNTER), 7);
        tes_save_load_game_test_all_cells(&mut e, this, 4);
        assert_eq!(e.global::<i32>(TEST_ALL_CELLS_COUNTER), 8);
        assert!(calls_to(&e, FORMAT).is_empty());
    }

    #[test]
    fn test_all_cells_saves_a_numbered_file_every_fiftieth_call() {
        let mut e = game();
        e.set_global(TEST_ALL_CELLS_COUNTER, 100i32);
        e.mem
            .set_cstr(FORMAT_TEST_ALL_CELLS, b"Test All Cells %i.ess");
        e.mem.set_cstr(AUTOSAVE_NAME, b"autosave");
        e.register(FORMAT, |e, a| {
            let formatted = String::from_utf8(e.mem.cstr(a[2]))
                .unwrap()
                .replace("%i", &(a[3] as i32).to_string());
            e.mem.set_cstr(a[0], formatted.as_bytes());
            Ret::default()
        });
        // Saving is not allowed, so `fn_00856ca0` only shows its message.
        constant(&mut e, GET_SAVING_ALLOWED, 0);
        e.register(STRING_COMPARE, |e, a| {
            returns(e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) as i32 as u32)
        });
        stub(&mut e, GET_MESSAGE_QUEUE);
        stub(&mut e, SHOW_MESSAGE);
        e.set_global(MESSAGE_TIME, 2.0f32);
        let this = game_object(&mut e);

        tes_save_load_game_test_all_cells(&mut e, this, 4);

        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(&formats[0][1..], &[0x104, FORMAT_TEST_ALL_CELLS, 100]);
        assert_eq!(calls_to(&e, SHOW_MESSAGE).len(), 1);
        assert_eq!(e.global::<i32>(TEST_ALL_CELLS_COUNTER), 101);
    }

    /// The pieces of mode 5: the `TES` object, the player and the placed
    /// reference.
    fn place_setup(e: &mut Engine, interior: u32) -> Rc<RefCell<Vec<f32>>> {
        e.set_global(TEST_ALL_CELLS_COUNTER, 1i32);
        e.set_global(TES_OBJECT, 0x0555_5000u32);
        e.set_global(PLACE_OBJECT_GLOBAL, 0x0577_7000u32);
        e.set_global(HEIGHT_OFFSET, 1000.0f64);
        constant(e, TES_CELL_COUNT, 2);
        constant(e, TES_INTERIOR_CELL, interior);
        e.register(TES_GRID_CELL, |_, a| returns(0xC0 + a[1]));
        stub(e, CELL_SAVE_GAME_TEST);
        constant(e, REF_GET_WORLDSPACE, 11);
        constant(e, REF_GET_PARENT_CELL, 22);
        constant(e, REF_GET_ROTATION, 33);
        let position_slot = 0x0390_0020;
        constant(e, position_slot, 0x0888_0000);
        let player = object_with_slots(e, &[(REFERENCE_GET_POSITION_SLOT, position_slot)]);
        e.set_global(PLAYER, player);
        let placed_position = e.mem.alloc(12);
        e.mem.set_f32(placed_position + 8, 5.0);
        let placed_slot = 0x0390_0024;
        constant(e, placed_slot, placed_position);
        let placed = object_with_slots(e, &[(REFERENCE_GET_POSITION_SLOT, placed_slot)]);
        constant(e, DATA_HANDLER_PLACE, placed);
        let heights: Rc<RefCell<Vec<f32>>> = Rc::default();
        let log = heights.clone();
        e.register_double(REF_SET_POSITION, move |e, a| {
            log.borrow_mut().push(e.mem.f32(a[1] + 8));
            Ret::default()
        });
        heights
    }

    #[test]
    fn mode_five_tests_every_grid_cell_and_places_five_references() {
        let mut e = game();
        let heights = place_setup(&mut e, 0);
        let this = game_object(&mut e);

        tes_save_load_game_test_all_cells(&mut e, this, 5);

        let tested: Vec<u32> = calls_to(&e, CELL_SAVE_GAME_TEST)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(tested, vec![0xC0, 0xC1]);
        let places = calls_to(&e, DATA_HANDLER_PLACE);
        assert_eq!(places.len(), 5);
        // (handler, global, position, rotation, parent cell, world space, 0, 0, 0)
        assert_eq!(
            places[0][1..],
            [0x0577_7000, 0x0888_0000, 33, 22, 11, 0, 0, 0]
        );
        assert_eq!(*heights.borrow(), vec![1005.0; 5]);
        assert_eq!(e.global::<i32>(TEST_ALL_CELLS_COUNTER), 2);
    }

    #[test]
    fn mode_five_with_an_interior_cell_tests_only_that_cell() {
        let mut e = game();
        place_setup(&mut e, 0xCE11);
        let this = game_object(&mut e);

        tes_save_load_game_test_all_cells(&mut e, this, 5);

        let tested: Vec<u32> = calls_to(&e, CELL_SAVE_GAME_TEST)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(tested, vec![0xCE11]);
        assert!(calls_to(&e, TES_GRID_CELL).is_empty());
    }

    // ---- 008620f0, 00862110 ----

    #[test]
    fn the_save_version_is_reset() {
        let mut e = game();
        let this = game_object(&mut e);
        e.set(this, TESSaveLoadGame::m_cMajorVersion, 9);
        fn_008620f0(&mut e, this);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMajorVersion), 0);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMinorVersion), 0x7D);
    }

    #[test]
    fn save_game_blocks_are_used_for_versions_1f_to_59() {
        let with_version = |version: u32| {
            let mut e = game();
            constant(&mut e, CURRENT_VERSION, version);
            tes_save_load_game_use_save_game_blocks(&mut e, Ptr::new(0))
        };
        assert!(!with_version(0x1E));
        assert!(with_version(0x1F));
        assert!(with_version(0x59));
        assert!(!with_version(0x5A));
    }

    // ---- 00862150 ----

    #[test]
    fn the_name_of_the_newest_save_is_kept_without_its_path_and_extension() {
        let mut e = game();
        e.mem.set_cstr(PATH_SEPARATOR, b"\\");
        let name = e.mem.alloc(0x40);
        e.mem.set_cstr(name, b"base\\Saves\\Save 3 - Alex.ess");
        let slot = 0x0390_0030;
        constant(&mut e, slot, name);
        let file = object_with_slots(&mut e, &[(FILE_GET_NAME, slot)]);
        let flag = Rc::new(Cell::new(1u32));
        let flag_read = flag.clone();
        e.register_double(FILE_FLAG_2C, move |_, _| returns(flag_read.get()));
        let this = game_object(&mut e);
        let old = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + MOST_RECENT_SAVE, old);

        fn_00862150(&mut e, this, Ptr::new(file));
        let kept = member(&e, this, MOST_RECENT_SAVE);
        assert_eq!(text(&e, kept), "Save 3 - Alex");
        assert!(e.mem.block_size(old).is_none() || old == kept);

        // Without the file's flag the old name is dropped and none is kept.
        flag.set(0);
        fn_00862150(&mut e, this, Ptr::new(file));
        assert_eq!(member(&e, this, MOST_RECENT_SAVE), 0);
        assert!(e.mem.block_size(kept).is_none());

        // A null file only drops the name.
        flag.set(1);
        fn_00862150(&mut e, this, Ptr::new(file));
        assert_ne!(member(&e, this, MOST_RECENT_SAVE), 0);
        fn_00862150(&mut e, this, Ptr::NULL);
        assert_eq!(member(&e, this, MOST_RECENT_SAVE), 0);
    }

    // ---- 008622a0, 00862370 ----

    fn text_environment(e: &mut Engine) {
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            returns(
                bytes
                    .iter()
                    .rposition(|&b| b == a[1] as u8)
                    .map_or(0, |i| a[0] + i as u32),
            )
        });
        e.register(STRNCMP, |e, a| {
            let n = a[2] as usize;
            let cut = |v: Vec<u8>| v.iter().take(n).copied().collect::<Vec<u8>>();
            let (left, right) = (cut(e.mem.cstr(a[0])), cut(e.mem.cstr(a[1])));
            returns(left.cmp(&right) as i32 as u32)
        });
        e.register(COPY_COUNTED, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(ATOL, |e, a| {
            let digits: String = text(e, a[0])
                .trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            returns(digits.parse::<u32>().unwrap_or(0))
        });
        e.mem.set_cstr(SAVE_NAME_PREFIX, b"Save ");
        e.mem.set_cstr(TEXT_PLAYING_TIME, b"Playing Time");
        e.mem.set_cstr(TEXT_DASH, b"-");
    }

    fn named_file(e: &mut Engine, name: &str) -> u32 {
        let string = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(string, name.as_bytes());
        let slot = 0x0392_0000 + string;
        constant(e, slot, string);
        object_with_slots(e, &[(FILE_GET_NAME, slot)])
    }

    /// A save list holding saves numbered 4 and 9, as `m_pSaveGameList`.
    fn save_list_environment(e: &mut Engine) -> Ptr<TESSaveLoadGame> {
        text_environment(e);
        e.register(SAVE_GAME_LIST_GETTER, |e, a| {
            returns(e.mem.u32(a[0] + 0x70))
        });
        e.register(LIST_NODE_IS_END, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        stub(e, END_FORM_PROCESSING);
        let this = game_object(e);
        let first = named_file(e, "base\\Saves\\Save 4 - Alex Playing Time 01.02.03.ess");
        let second = named_file(e, "base\\Saves\\Save 9 - Alex Playing Time 01.02.03.ess");
        let list = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(node, second);
        e.set(this, TESSaveLoadGame::m_pSaveGameList, Ptr::new(list));
        this
    }

    #[test]
    fn the_next_save_number_is_one_more_than_the_highest() {
        let mut e = game();
        let this = save_list_environment(&mut e);
        e.set(this, TESSaveLoadGame::m_cMajorVersion, 3);

        assert_eq!(fn_008622a0(&mut e, this), 10);
        assert_eq!(e.get(this, TESSaveLoadGame::m_iNextSaveNumber), 10);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMajorVersion), 0);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMinorVersion), 0x7D);
        assert_eq!(calls_to(&e, END_FORM_PROCESSING), vec![vec![this.addr()]]);
        // The list existed, so it is kept.
        assert!(!e.get(this, TESSaveLoadGame::m_pSaveGameList).is_null());
    }

    #[test]
    fn without_a_save_list_one_is_built_and_destroyed_again() {
        let mut e = game();
        let this = save_list_environment(&mut e);
        e.set(this, TESSaveLoadGame::m_pSaveGameList, Ptr::NULL);
        // The list is built from the files found in the save folder: none.
        constant(&mut e, PATH_PREFIX, 0);
        constant(&mut e, PATH_OBJECT_GET, 0);
        constant(&mut e, IDENTITY, 0);
        stub(&mut e, LSTRCPY_IMPORT);
        stub(&mut e, LSTRCAT_IMPORT);
        constant(&mut e, FIND_FIRST_FILE_IMPORT, 0xFFFF_FFFF);
        stub(&mut e, LIST_REMOVE_ALL);
        stub(&mut e, LIST_SCALAR_DELETE);

        assert_eq!(fn_008622a0(&mut e, this), 1);
        assert!(e.get(this, TESSaveLoadGame::m_pSaveGameList).is_null());
        assert_eq!(calls_to(&e, LIST_SCALAR_DELETE).len(), 1);
    }

    #[test]
    fn the_next_save_number_is_only_worked_out_when_it_is_zero() {
        let mut e = game();
        let this = save_list_environment(&mut e);
        e.set(this, TESSaveLoadGame::m_iNextSaveNumber, 5);
        assert_eq!(fn_00862370(&mut e, this), 5);
        assert!(calls_to(&e, END_FORM_PROCESSING).is_empty());
        e.set(this, TESSaveLoadGame::m_iNextSaveNumber, 0);
        assert_eq!(fn_00862370(&mut e, this), 10);
        assert_eq!(calls_to(&e, END_FORM_PROCESSING).len(), 1);
    }

    // ---- 008623a0 ----

    #[test]
    fn a_form_is_taken_out_of_the_init_array() {
        let mut e = game();
        install_arrays(&mut e);
        e.register(ARRAY_REMOVE_AT, |e, a| {
            let count = e.mem.u32(a[0] + 0xC);
            let data = e.mem.u32(a[0] + 0x10);
            for i in a[1]..count - 1 {
                let next = e.mem.u32(data + 4 * (i + 1));
                e.mem.set_u32(data + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 0xC, count - 1);
            Ret::default()
        });
        let this = game_object(&mut e);
        // No array: nothing happens.
        fn_008623a0(&mut e, this, 0x1111);
        assert!(calls_to(&e, ARRAY_SIZE).is_empty());

        let entry = |e: &mut Engine, form: u32| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, form);
            block
        };
        let (a, b) = (entry(&mut e, 0x1111), entry(&mut e, 0x2222));
        let array = make_array(&mut e, &[0, a, b]);
        e.set(this, TESSaveLoadGame::m_pInitArray, Ptr::new(array));

        fn_008623a0(&mut e, this, 0x3333);
        assert_eq!(array_items(&e, array), vec![0, a, b]);
        assert!(calls_to(&e, ARRAY_REMOVE_AT).is_empty());

        fn_008623a0(&mut e, this, 0x2222);
        assert_eq!(array_items(&e, array), vec![0, a]);
        assert_eq!(calls_to(&e, ARRAY_REMOVE_AT), vec![vec![array, 2]]);
        assert!(e.mem.block_size(b).is_none());
        assert!(e.mem.block_size(a).is_some());
    }

    // ---- 00862430 ----

    fn install_float_checks(e: &mut Engine) {
        e.register(CRT_FINITE, |_, a| {
            returns(f64::take(a, &mut 0).is_finite() as u32)
        });
        e.register(CRT_IS_NAN, |_, a| {
            returns(f64::take(a, &mut 0).is_nan() as u32)
        });
    }

    fn vector(e: &mut Engine, x: f32, y: f32, z: f32) -> Ptr {
        let block = e.mem.alloc(12);
        e.mem.set_f32(block, x);
        e.mem.set_f32(block + 4, y);
        e.mem.set_f32(block + 8, z);
        Ptr::new(block)
    }

    #[test]
    fn a_vector_is_corrupt_with_an_infinite_or_nan_component() {
        let mut e = game();
        install_float_checks(&mut e);
        let this = game_object(&mut e);
        let good = vector(&mut e, 1.0, -2.0, 3.5);
        assert!(!fn_00862430(&mut e, this, good));
        // Three finite checks, then three NaN checks.
        assert_eq!(calls_to(&e, CRT_FINITE).len(), 3);
        assert_eq!(calls_to(&e, CRT_IS_NAN).len(), 3);

        let infinite = vector(&mut e, 1.0, f32::INFINITY, 0.0);
        assert!(fn_00862430(&mut e, this, infinite));
        // The checks stop at the first failure: no NaN check ran again.
        assert_eq!(calls_to(&e, CRT_FINITE).len(), 5);
        assert_eq!(calls_to(&e, CRT_IS_NAN).len(), 3);

        // A NaN the `_finite` stand-in lets through is caught by `_isnan`.
        e.register(CRT_FINITE, |_, _| returns(1));
        let not_a_number = vector(&mut e, 0.0, 0.0, f32::NAN);
        assert!(fn_00862430(&mut e, this, not_a_number));
    }

    // ---- 008624e0 ----

    struct Repair {
        reference: Ptr,
        position: u32,
    }

    /// A reference whose position pointer (slot `0x1F4`) leads to `position`
    /// and whose rotation (`00430830`) is `rotation`; the repair functions
    /// are stand-ins that only record their calls.
    fn repair_setup(
        e: &mut Engine,
        position: (f32, f32, f32),
        rotation: (f32, f32, f32),
        extra_slots: &[(u32, u32)],
    ) -> Repair {
        install_float_checks(e);
        e.mem.set_u32(DEFAULT_POSITION, 0x1111);
        e.mem.set_u32(DEFAULT_POSITION + 4, 0x2222);
        e.mem.set_u32(DEFAULT_POSITION + 8, 0x3333);
        let position_block = vector(e, position.0, position.1, position.2);
        let rotation_block = vector(e, rotation.0, rotation.1, rotation.2);
        let position_slot = 0x0390_0040;
        constant(e, position_slot, position_block.addr());
        let not_actor = 0x0390_0041;
        constant(e, not_actor, 0);
        let mut slots = vec![
            (REFERENCE_GET_POSITION_SLOT, position_slot),
            (REFERENCE_IS_ACTOR, not_actor),
        ];
        slots.extend_from_slice(extra_slots);
        let reference = object_with_slots(e, &slots);
        e.mem.set_u32(reference + 0xC, 0xAB);
        constant(e, REF_GET_ROTATION, rotation_block.addr());
        stub(e, REF_SET_POSITION);
        stub(e, FN_005757D0);
        stub(e, FN_00575700);
        stub(e, REF_MOVE_TO_SPACE);
        Repair {
            reference: Ptr::new(reference),
            position: position_block.addr(),
        }
    }

    #[test]
    fn a_good_reference_is_left_alone() {
        let mut e = game();
        let this = game_object(&mut e);
        let r = repair_setup(&mut e, (1.0, 2.0, 3.0), (0.0, 0.0, 1.0), &[]);
        fn_008624e0(&mut e, this, r.reference);
        assert!(calls_to(&e, LOG_ERROR).is_empty());
        assert!(calls_to(&e, REF_SET_POSITION).is_empty());
        assert!(calls_to(&e, FN_00575700).is_empty());
        // A null reference is ignored altogether.
        let before = e.call_log.as_ref().unwrap().len();
        fn_008624e0(&mut e, this, Ptr::NULL);
        assert_eq!(e.call_log.as_ref().unwrap().len(), before);
    }

    #[test]
    fn a_corrupt_position_without_extra_data_becomes_the_default_position() {
        let mut e = game();
        let this = game_object(&mut e);
        let r = repair_setup(&mut e, (f32::NAN, 0.0, 0.0), (0.0, 0.0, 1.0), &[]);
        constant(&mut e, ACTOR_HAS_LOCATION_SLOT, 0);
        constant(&mut e, REF_GET_EXTRA_LIST, 0x0999);
        constant(&mut e, EXTRA_GET_DATA, 0);

        fn_008624e0(&mut e, this, r.reference);

        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_CORRUPT_LOCATION, 0xAB]]
        );
        assert_eq!(
            calls_to(&e, REF_SET_POSITION),
            vec![vec![r.reference.addr(), DEFAULT_POSITION]]
        );
        assert_eq!(
            calls_to(&e, EXTRA_GET_DATA),
            vec![vec![0x0999, EXTRA_DATA_START_LOCATION]]
        );
        assert!(calls_to(&e, REF_MOVE_TO_SPACE).is_empty());
    }

    #[test]
    fn a_corrupt_actor_goes_to_its_saved_location() {
        let mut e = game();
        let this = game_object(&mut e);
        let saved_position = vector(&mut e, 1.0, 2.0, 3.0);
        let saved_rotation = vector(&mut e, 0.0, 0.0, 0.5);
        let (is_actor, has_location, cell, worldspace) =
            (0x0390_0050, 0x0390_0051, 0x0390_0052, 0x0390_0053);
        let (get_location, get_rotation) = (0x0390_0054, 0x0390_0055);
        constant(&mut e, is_actor, 1);
        constant(&mut e, has_location, 1);
        constant(&mut e, cell, 0xCE11);
        constant(&mut e, worldspace, 0);
        constant(&mut e, get_location, saved_position.addr());
        constant(&mut e, get_rotation, saved_rotation.addr());
        let r = repair_setup(
            &mut e,
            (f32::INFINITY, 0.0, 0.0),
            (0.0, 0.0, 1.0),
            &[
                (REFERENCE_IS_ACTOR, is_actor),
                (ACTOR_HAS_LOCATION_SLOT, has_location),
                (ACTOR_CELL_SLOT, cell),
                (ACTOR_WORLDSPACE_SLOT, worldspace),
                (REFERENCE_GET_LOCATION_SLOT, get_location),
                (REFERENCE_GET_ROTATION_SLOT, get_rotation),
            ],
        );

        fn_008624e0(&mut e, this, r.reference);

        let reference = r.reference.addr();
        assert_eq!(
            calls_to(&e, REF_SET_POSITION),
            vec![vec![reference, saved_position.addr()]]
        );
        assert_eq!(
            calls_to(&e, FN_005757D0),
            vec![vec![reference, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, REF_MOVE_TO_SPACE),
            vec![vec![reference, 0xCE11, 0]]
        );
    }

    #[test]
    fn a_corrupt_reference_with_start_location_data_takes_it() {
        let mut e = game();
        let this = game_object(&mut e);
        let (get_location, get_rotation) = (0x0390_0060, 0x0390_0061);
        let r = {
            let slots = [
                (REFERENCE_GET_LOCATION_SLOT, get_location),
                (REFERENCE_GET_ROTATION_SLOT, get_rotation),
            ];
            repair_setup(&mut e, (f32::NAN, 0.0, 0.0), (0.0, 0.0, 1.0), &slots)
        };
        // The slots fill the out buffers they are given.
        e.register(get_location, |e, a| {
            e.mem.set_f32(a[1], 10.0);
            Ret::default()
        });
        e.register(get_rotation, |e, a| {
            e.mem.set_u32(a[1], 7);
            e.mem.set_u32(a[1] + 4, 8);
            e.mem.set_u32(a[1] + 8, 9);
            Ret::default()
        });
        constant(&mut e, ACTOR_HAS_LOCATION_SLOT, 0);
        constant(&mut e, REF_GET_EXTRA_LIST, 0x0999);
        constant(&mut e, EXTRA_GET_DATA, 0x0AAA);
        constant(&mut e, EXTRA_GET_STARTING_SPACE, 0x0777);
        // The starting space is a cell.
        e.register(DYNAMIC_CAST, |_, a| {
            returns(if a[3] == RTTI_CELL { a[0] } else { 0 })
        });

        fn_008624e0(&mut e, this, r.reference);

        let reference = r.reference.addr();
        let set = calls_to(&e, REF_SET_POSITION);
        assert_eq!(set.len(), 1);
        assert_eq!(e.mem.f32(set[0][1]), 10.0);
        assert_eq!(calls_to(&e, FN_00575700), vec![vec![reference, 7, 8, 9]]);
        assert_eq!(
            calls_to(&e, REF_MOVE_TO_SPACE),
            vec![vec![reference, 0x0777, 0]]
        );
    }

    #[test]
    fn a_corrupt_rotation_becomes_the_default_position_words() {
        let mut e = game();
        let this = game_object(&mut e);
        let r = repair_setup(&mut e, (1.0, 2.0, 3.0), (f32::INFINITY, 0.0, 0.0), &[]);

        fn_008624e0(&mut e, this, r.reference);

        assert_eq!(calls_to(&e, LOG_ERROR), vec![vec![MSG_CORRUPT_ANGLE, 0xAB]]);
        assert_eq!(
            calls_to(&e, FN_00575700),
            vec![vec![r.reference.addr(), 0x1111, 0x2222, 0x3333]]
        );
        assert!(calls_to(&e, REF_SET_POSITION).is_empty());
        let _ = r.position;
    }

    // ---- 008627b0 ----

    /// What the stand-ins for the hash maps hold: the entries of each map by
    /// address and the keys removed from a map.
    #[derive(Default)]
    struct Maps {
        entries: HashMap<u32, Vec<(u32, u32)>>,
        removed: Vec<(u32, u32)>,
    }

    type SharedMaps = Rc<RefCell<Maps>>;

    /// Doubles for the map iteration (`GetFirstPos` / `GetNext`: the
    /// position cell is the 1-based index of the next entry, 0 at the end),
    /// `GetAt`, `SetAt` and `RemoveAt` over the entries of each map, and for
    /// the lists of coordinates.
    fn install_maps(e: &mut Engine) -> SharedMaps {
        let maps: SharedMaps = Rc::default();
        let m = maps.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, a| {
            let maps = m.borrow();
            returns(maps.entries.get(&a[0]).is_some_and(|v| !v.is_empty()) as u32)
        });
        let m = maps.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let maps = m.borrow();
            let entries = maps.entries.get(&a[0]).unwrap();
            let index = e.mem.u32(a[1]) as usize - 1;
            let (key, value) = entries[index];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let next = if index + 1 < entries.len() {
                index as u32 + 2
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let m = maps.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            let maps = m.borrow();
            let found = maps
                .entries
                .get(&a[0])
                .and_then(|v| v.iter().find(|(k, _)| *k == a[1]))
                .filter(|(k, _)| !maps.removed.contains(&(a[0], *k)));
            if let Some((_, value)) = found {
                e.mem.set_u32(a[2], *value);
            }
            returns(found.is_some() as u32)
        });
        let m = maps.clone();
        e.register_double(CHANGES_MAP_SET_AT, move |_, a| {
            m.borrow_mut()
                .entries
                .entry(a[0])
                .or_default()
                .push((a[1], a[2]));
            Ret::default()
        });
        let m = maps.clone();
        e.register_double(MAP_REMOVE_AT, move |_, a| {
            m.borrow_mut().removed.push((a[0], a[1]));
            returns(1)
        });
        stub(e, MAP_REMOVE_ALL);
        // A list of coordinates: the head node holds the newest item.
        e.register(LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            let (head_item, head_next) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
            if head_item != 0 {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, head_item);
                e.mem.set_u32(node + 4, head_next);
                e.mem.set_u32(a[0] + 4, node);
            }
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        stub(e, LIST_SCALAR_DELETE);
        maps
    }

    /// The pieces of `fn_008627b0` shared by its tests: the game object as
    /// the singleton, the calendar (time 1000) and age limit (100), the local
    /// map constructor and destructor, the maps and the cell and reference
    /// accessors (fields of the test blocks).
    struct Pass {
        this: Ptr<TESSaveLoadGame>,
        maps: SharedMaps,
        changes: u32,
        local_map: Rc<Cell<u32>>,
    }

    fn pass_setup(e: &mut Engine) -> Pass {
        install_created_ids(e);
        let maps = install_maps(e);
        let this = game_object(e);
        e.set_global(SAVE_LOAD_GAME, this.addr());
        let changes = e.mem.alloc(0x10);
        e.set(this, TESSaveLoadGame::m_pChanges, Ptr::new(changes));
        constant(e, BUFFER_GET_VERSION, 1);
        stub(e, BUFFER_SET_VERSION);
        constant(e, CALENDAR_TIME, 1000);
        constant(e, DETACH_AGE_LIMIT, 100);
        let local_map = Rc::new(Cell::new(0u32));
        let cell = local_map.clone();
        e.register_double(LOCAL_MAP_ROOT_CONSTRUCT, move |_, a| {
            cell.set(a[0]);
            returns(a[0])
        });
        stub(e, LOCAL_MAP_DESTRUCT);
        e.register(CHANGE_DATA_SET_BUFFER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(CHANGE_DATA_SET_FLAGS, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(CELL_IS_INTERIOR, |e, a| returns(e.mem.u32(a[0] + 0x14)));
        e.register(CELL_GET_WORLDSPACE, |e, a| returns(e.mem.u32(a[0] + 0x18)));
        e.register(CELL_GET_X, |e, a| returns(e.mem.u32(a[0] + 0x1C)));
        e.register(CELL_GET_Y, |e, a| returns(e.mem.u32(a[0] + 0x20)));
        e.register(CELL_GET_DETACH_TIME, |e, a| returns(e.mem.u32(a[0] + 0x24)));
        e.register(REF_GET_PARENT_CELL, |e, a| returns(e.mem.u32(a[0] + 0x28)));
        e.register(FLOAT_TO_INTEGER, |_, a| {
            returns(f64::take(a, &mut 0) as i32 as u32)
        });
        constant(e, REF_GET_EXTRA_LIST, 0);
        Pass {
            this,
            maps,
            changes,
            local_map,
        }
    }

    /// An exterior cell form (type 0x39) in world space `worldspace_id` at
    /// grid (`x`, `y`), detached at `detached`.
    fn exterior_cell(
        e: &mut Engine,
        id: u32,
        worldspace_id: u32,
        grid: (u32, u32),
        detached: u32,
    ) -> u32 {
        let worldspace = form_with_id(e, worldspace_id);
        let cell = form_with_id(e, id);
        e.mem.set_u8(cell + 4, 0x39);
        e.mem.set_u32(cell + 0x14, 0);
        e.mem.set_u32(cell + 0x18, worldspace);
        e.mem.set_u32(cell + 0x1C, grid.0);
        e.mem.set_u32(cell + 0x20, grid.1);
        e.mem.set_u32(cell + 0x24, detached);
        cell
    }

    fn change_data(e: &mut Engine, flags: u32, buffer: u32) -> u32 {
        let change = e.mem.alloc(8);
        e.mem.set_u32(change, flags);
        e.mem.set_u32(change + 4, buffer);
        change
    }

    fn lookup_forms(e: &mut Engine, forms: &[(u32, u32)]) {
        let table: HashMap<u32, u32> = forms.iter().copied().collect();
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(table.get(&a[0]).copied().unwrap_or(0))
        });
    }

    /// The map of detached cells as the function creates it, already holding
    /// the cell at grid (`x`, `y`) of the world space `location`.
    fn preset_detached_cell(e: &mut Engine, pass: &Pass, location: u32, x: u32, y: u32) {
        let list = e.mem.alloc(8);
        let coordinates = e.mem.alloc(8);
        e.mem.set_u32(coordinates, x);
        e.mem.set_u32(coordinates + 4, y);
        e.mem.set_u32(list, coordinates);
        let local = pass.local_map.clone();
        let maps = pass.maps.clone();
        e.register_double(LOCAL_MAP_ROOT_CONSTRUCT, move |_, a| {
            local.set(a[0]);
            maps.borrow_mut()
                .entries
                .insert(a[0], vec![(location, list)]);
            returns(a[0])
        });
    }

    /// The call `vcall(reference, 0x10, 1)` of a reference's destructor goes
    /// to this address.
    const DESTROY_REFERENCE: u32 = 0x0390_0070;

    /// A reference form of `form_type` in `cell`; its vtable answers "not an
    /// actor" and has a destructor stand-in.
    fn loaded_reference(e: &mut Engine, id: u32, form_type: u8, cell: u32) -> u32 {
        stub(e, DESTROY_REFERENCE);
        let not_actor = 0x0390_0071;
        constant(e, not_actor, 0);
        let reference = object_with_slots(
            e,
            &[
                (FORM_SCALAR_DELETE_SLOT, DESTROY_REFERENCE),
                (REFERENCE_IS_ACTOR, not_actor),
            ],
        );
        e.mem.set_u32(reference + 0xC, id);
        e.mem.set_u8(reference + 4, form_type);
        e.mem.set_u32(reference + 0x28, cell);
        reference
    }

    #[test]
    fn a_long_detached_loaded_cell_is_noted_with_its_coordinates() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        let cell = exterior_cell(&mut e, 0x100, 0x3000, (5, 6), 50);
        let fresh = exterior_cell(&mut e, 0x101, 0x3000, (7, 8), 990);
        lookup_forms(&mut e, &[(0x100, cell), (0x101, fresh)]);
        let old = change_data(&mut e, CHANGE_FLAG_DETACH_TIME, 0);
        let recent = change_data(&mut e, CHANGE_FLAG_DETACH_TIME, 0);
        let other = change_data(&mut e, 0x1, 0);
        pass.maps.borrow_mut().entries.insert(
            pass.changes,
            vec![(0x100, old), (0x101, recent), (0x102, other)],
        );

        fn_008627b0(&mut e, pass.this);

        // Only the cell detached 950 > 100 ticks ago is noted: a list under
        // its world space id; the coordinate pair was freed again with the
        // list at the end, and the byte at +0x1C restored.
        let maps = pass.maps.borrow();
        let local = pass.local_map.get();
        assert_eq!(maps.entries[&local].len(), 1);
        assert_eq!(maps.entries[&local][0].0, 0x3000);
        let list = maps.entries[&local][0].1;
        assert_eq!(calls_to(&e, LIST_SCALAR_DELETE), vec![vec![list, 1]]);
        let freed = calls_to(&e, OPERATOR_DELETE);
        assert_eq!(freed.len(), 1);
        assert!(e.mem.block_size(freed[0][0]).is_none());
        assert_eq!(
            calls_to(&e, BUFFER_SET_VERSION),
            vec![vec![pass.this.addr(), 0], vec![pass.this.addr(), 1]]
        );
        assert_eq!(calls_to(&e, LOCAL_MAP_DESTRUCT), vec![vec![local]]);
        assert_eq!(
            calls_to(&e, LOCAL_MAP_ROOT_CONSTRUCT),
            vec![vec![local, HASH_SIZE]]
        );
    }

    #[test]
    fn a_second_cell_in_the_same_world_space_joins_its_list() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        let first = exterior_cell(&mut e, 0x100, 0x3000, (5, 6), 50);
        let second = exterior_cell(&mut e, 0x101, 0x3000, (7, 8), 60);
        lookup_forms(&mut e, &[(0x100, first), (0x101, second)]);
        let (a, b) = (
            change_data(&mut e, CHANGE_FLAG_DETACH_TIME, 0),
            change_data(&mut e, CHANGE_FLAG_DETACH_TIME, 0),
        );
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0x100, a), (0x101, b)]);
        fn_008627b0(&mut e, pass.this);

        // One list for the world space (made once), two coordinate pairs
        // freed with it.
        let maps = pass.maps.borrow();
        assert_eq!(maps.entries[&pass.local_map.get()].len(), 1);
        assert_eq!(calls_to(&e, SIMPLE_LIST_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, LIST_ADD_HEAD).len(), 2);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn a_cell_with_a_saved_detach_time_is_read_from_its_record() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        // Record: 4 header bytes (cell type 0x39 and a version byte), then
        // the detach time 50.
        let record = |e: &mut Engine, version: u8| {
            let buffer = e.mem.alloc(8);
            e.mem.write(buffer, &[0, 0, 0x39, version]);
            e.mem.set_u32(buffer + 4, 50);
            buffer
        };
        let (buffer_new, buffer_old) = (record(&mut e, 0x5B), record(&mut e, 0x5A));
        let new = change_data(&mut e, CHANGE_FLAG_DETACH_TIME, buffer_new);
        let old = change_data(&mut e, CHANGE_FLAG_DETACH_TIME, buffer_old);
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0x500, new), (0x501, old)]);

        fn_008627b0(&mut e, pass.this);

        // Version 0x5B has a detach time (long ago); the saved record gives
        // no coordinates, so the cell is noted with a null list. Version 0x5A
        // has none.
        let maps = pass.maps.borrow();
        assert_eq!(maps.entries[&pass.local_map.get()], vec![(0x500, 0)]);
        assert!(e.get(pass.this, TESSaveLoadGame::m_pBuffer).is_null());
    }

    #[test]
    fn a_moved_reference_in_a_detached_cell_loses_its_change() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        preset_detached_cell(&mut e, &pass, 0x3000, 5, 6);
        let cell = exterior_cell(&mut e, 0x100, 0x3000, (5, 6), 0);
        let reference = loaded_reference(&mut e, 0x200, 0x3A, cell);
        lookup_forms(&mut e, &[(0x200, reference)]);
        let change = change_data(&mut e, CHANGE_FLAG_MOVED, 0);
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0x200, change)]);

        fn_008627b0(&mut e, pass.this);

        // Flags exactly 4 drop the entry; the reference (type 0x3A) is put
        // back (`fn_00857d10` asks for its extra data) and not destroyed.
        let maps = pass.maps.borrow();
        assert_eq!(maps.removed, vec![(pass.changes, 0x200)]);
        assert_eq!(calls_to(&e, REF_GET_EXTRA_LIST), vec![vec![reference]]);
        assert!(calls_to(&e, DESTROY_REFERENCE).is_empty());
    }

    #[test]
    fn a_loaded_reference_with_more_flags_only_loses_the_moved_bit() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        preset_detached_cell(&mut e, &pass, 0x3000, 5, 6);
        let cell = exterior_cell(&mut e, 0x100, 0x3000, (5, 6), 0);
        let reference = loaded_reference(&mut e, 0x200, 0x3A, cell);
        lookup_forms(&mut e, &[(0x200, reference)]);
        let change = change_data(&mut e, CHANGE_FLAG_MOVED | 0x8, 0);
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0x200, change)]);

        fn_008627b0(&mut e, pass.this);

        assert_eq!(e.mem.u32(change), 0x8);
        assert!(pass.maps.borrow().removed.is_empty());
        assert_eq!(calls_to(&e, REF_GET_EXTRA_LIST), vec![vec![reference]]);
    }

    #[test]
    fn a_created_reference_in_a_detached_cell_is_destroyed() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        preset_detached_cell(&mut e, &pass, 0x3000, 5, 6);
        let cell = exterior_cell(&mut e, 0x100, 0x3000, (5, 6), 0);
        let reference = loaded_reference(&mut e, 0xFF00_0200, 0x3B, cell);
        lookup_forms(&mut e, &[(0xFF00_0200, reference)]);
        let change = change_data(&mut e, CHANGE_FLAG_MOVED, 0);
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0xFF00_0200, change)]);

        fn_008627b0(&mut e, pass.this);

        // The entry was removed once; the second `fn_00855220` finds none.
        assert_eq!(
            pass.maps.borrow().removed,
            vec![(pass.changes, 0xFF00_0200)]
        );
        assert_eq!(calls_to(&e, DESTROY_REFERENCE), vec![vec![reference, 1]]);
    }

    #[test]
    fn a_reference_outside_the_detached_cells_is_left_alone() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        preset_detached_cell(&mut e, &pass, 0x3000, 5, 6);
        // Another grid cell, another world space, and a reference with no
        // cell at all.
        let elsewhere = exterior_cell(&mut e, 0x100, 0x3000, (9, 9), 0);
        let other_world = exterior_cell(&mut e, 0x101, 0x3999, (5, 6), 0);
        let first = loaded_reference(&mut e, 0x200, 0x3A, elsewhere);
        let second = loaded_reference(&mut e, 0x201, 0x3A, other_world);
        let third = loaded_reference(&mut e, 0x202, 0x3A, 0);
        lookup_forms(&mut e, &[(0x200, first), (0x201, second), (0x202, third)]);
        let changes: Vec<(u32, u32)> = (0x200..0x203)
            .map(|key| (key, change_data(&mut e, CHANGE_FLAG_MOVED, 0)))
            .collect();
        pass.maps.borrow_mut().entries.insert(pass.changes, changes);

        fn_008627b0(&mut e, pass.this);

        assert!(pass.maps.borrow().removed.is_empty());
        assert!(calls_to(&e, REF_GET_EXTRA_LIST).is_empty());
    }

    #[test]
    fn the_reference_data_is_stripped_from_a_saved_record() {
        let mut e = game();
        let pass = pass_setup(&mut e);
        preset_detached_cell(&mut e, &pass, 0xFF00_0010, 5, 6);
        // Record: size 0x1C + 6, type 0x3A, version 0; the ReferenceData
        // (location 0xFF000010, x 5 and y 6 in cells of 4096 units); then
        // six payload bytes.
        let old_buffer = e.mem.alloc(0x30);
        e.mem.write(old_buffer, &[0x22, 0x00, 0x3A, 0x00]);
        e.mem.set_u32(old_buffer + 4, 0xFF00_0010);
        e.mem.set_f32(old_buffer + 8, 5.0 * 4096.0);
        e.mem.set_f32(old_buffer + 12, 6.0 * 4096.0);
        e.mem.write(old_buffer + 4 + 0x1C, &[1, 2, 3, 4, 5, 6]);
        let change = change_data(&mut e, CHANGE_FLAG_MOVED | 0x8, old_buffer);
        pass.maps
            .borrow_mut()
            .entries
            .insert(pass.changes, vec![(0x400, change)]);

        fn_008627b0(&mut e, pass.this);

        // A new buffer of 4 + 6 bytes with the shorter size, flags without
        // the moved bit; the old buffer is gone and the current buffer
        // cleared.
        let new_buffer = e.mem.u32(change + 4);
        assert_ne!(new_buffer, old_buffer);
        assert_eq!(
            e.mem.bytes(new_buffer, 10),
            vec![6, 0, 0x3A, 0, 1, 2, 3, 4, 5, 6]
        );
        assert_eq!(e.mem.u32(change), 0x8);
        assert!(e.mem.block_size(old_buffer).is_none());
        assert!(e.get(pass.this, TESSaveLoadGame::m_pBuffer).is_null());
        assert!(pass.maps.borrow().removed.is_empty());
    }

    // ---- 008632a0 ----

    #[test]
    fn a_form_is_added_to_the_init_item_array_only_when_the_game_asks() {
        let mut e = game();
        install_arrays(&mut e);
        e.register(FORM_ARRAY_CONSTRUCT, |e, a| {
            let data = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 0x10, data);
            e.mem.set_u32(a[0] + 0xC, 0);
            returns(a[0])
        });
        let this = game_object(&mut e);

        // `0047c850` false (this build): nothing happens.
        fn_008632a0(&mut e, this, Ptr::new(0x1234));
        assert_eq!(member(&e, this, INIT_ITEM_ARRAY), 0);

        constant(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        fn_008632a0(&mut e, this, Ptr::new(0x1234));
        let array = member(&e, this, INIT_ITEM_ARRAY);
        assert_ne!(array, 0);
        assert_eq!(
            calls_to(&e, FORM_ARRAY_CONSTRUCT),
            vec![vec![array, FORM_ARRAY_GROW, FORM_ARRAY_INITIAL]]
        );
        assert_eq!(e.mem.block_size(array), Some(FORM_ARRAY_SIZE));
        fn_008632a0(&mut e, this, Ptr::new(0x5678));
        assert_eq!(member(&e, this, INIT_ITEM_ARRAY), array);
        assert_eq!(array_items(&e, array), vec![0x1234, 0x5678]);
        assert_eq!(calls_to(&e, FORM_ARRAY_CONSTRUCT).len(), 1);
    }

    // ---- the NiTPointerMap instances ----

    type Constructor = fn(&mut Engine, Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>;

    fn map_block(e: &mut Engine) -> Ptr<NiTPointerMap> {
        e.new_object::<NiTPointerMap>()
    }

    /// Allocation and clearing of a bucket array, as the root constructors
    /// call them.
    fn install_bucket_allocation(e: &mut Engine) {
        e.register(NI_ALLOC, |e, a| returns(e.mem.alloc(a[0])));
        stub(e, MEMSET);
    }

    #[test]
    fn the_root_constructors_make_a_zeroed_bucket_array() {
        let roots: [(Constructor, u32); 2] = [
            (fn_008635d0, CHANGES_MAP_ROOT_VTABLE),
            (fn_008636d0, INTERIOR_MAP_ROOT_VTABLE),
        ];
        for (root, vtable) in roots {
            let mut e = game();
            install_bucket_allocation(&mut e);
            let map = map_block(&mut e);
            e.set(map, NiTPointerMap::m_uiCount, 9);
            assert_eq!(root(&mut e, map, 0x25), map);
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x25);
            assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
            let table = e.get(map, NiTPointerMap::m_ppkHashTable);
            assert_eq!(e.mem.block_size(table), Some(0x98));
            assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![0x94]]);
            assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 0x94]]);
        }
    }

    #[test]
    fn the_map_constructors_set_their_vtable_after_the_root_constructor() {
        // The two with a root constructor in this part.
        let mut e = game();
        install_bucket_allocation(&mut e);
        let map = map_block(&mut e);
        assert_eq!(fn_00863360(&mut e, map, 0x25), map);
        assert_eq!(e.mem.u32(map.addr()), CHANGES_MAP_BASE_VTABLE);
        assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x25);
        let other = map_block(&mut e);
        assert_eq!(fn_00863390(&mut e, other, 0x11), other);
        assert_eq!(e.mem.u32(other.addr()), INTERIOR_MAP_BASE_VTABLE);
        assert_eq!(e.get(other, NiTPointerMap::m_uiHashSize), 0x11);

        let constructors: [(Constructor, u32); 3] = [
            (fn_008633c0, EXTERIOR_MAP_BASE_VTABLE),
            (fn_008633f0, NUMERIC_ID_MAP_BASE_VTABLE),
            (fn_00863420, STATS_MAP_BASE_VTABLE),
        ];
        for (constructor, vtable) in constructors {
            let mut e = game();
            install_bucket_allocation(&mut e);
            let map = map_block(&mut e);
            assert_eq!(constructor(&mut e, map, 0x25), map);
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x25);
            assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![0x94]]);
        }
    }

    #[test]
    fn the_other_root_constructors_set_their_own_vtable() {
        let roots: [(Constructor, u32); 4] = [
            (fn_008637d0, EXTERIOR_MAP_ROOT_VTABLE),
            (fn_008638f0, NUMERIC_ID_MAP_ROOT_VTABLE),
            (fn_008639f0, STATS_MAP_ROOT_VTABLE),
            (fn_00863e30, LOCAL_MAP_ROOT_VTABLE),
        ];
        for (root, vtable) in roots {
            let mut e = game();
            install_bucket_allocation(&mut e);
            let map = map_block(&mut e);
            e.set(map, NiTPointerMap::m_uiCount, 9);
            assert_eq!(root(&mut e, map, 0x10), map);
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x10);
            assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
            let table = e.get(map, NiTPointerMap::m_ppkHashTable);
            assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 0x40]]);
        }
    }

    #[test]
    fn the_stack_map_constructors_pass_their_arguments_on() {
        let mut e = game();
        stub(&mut e, LOCAL_MAP_ROOT_CONSTRUCT);
        e.register(ARRAY_ALLOC_BLOCK, |e, a| returns(e.mem.alloc(a[0])));
        let map = Ptr::<()>::new(e.mem.alloc(0x18));
        assert_eq!(fn_00863450(&mut e, map, 0x25), map);
        assert_eq!(
            calls_to(&e, LOCAL_MAP_ROOT_CONSTRUCT),
            vec![vec![map.addr(), 0x25]]
        );
        assert_eq!(e.mem.u32(map.addr()), LOCAL_MAP_VTABLE);
        assert_eq!(fn_008635a0(&mut e, map, 7, 8), map);
        assert_eq!(calls_to(&e, ARRAY_ALLOC_BLOCK), vec![vec![7]]);
        assert_eq!(e.mem.u32(map.addr()), STATS_LIST_MAP_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 8), 7);
        assert_eq!(e.mem.u32(map.addr() + 0x14), 8);
    }

    #[test]
    fn the_destructors_empty_the_map_and_free_the_buckets() {
        type Destructor = fn(&mut Engine, Ptr<NiTPointerMap>);
        let destructors: [(Destructor, u32, bool); 4] = [
            (fn_00863640, CHANGES_MAP_ROOT_VTABLE, true),
            (fn_00863740, INTERIOR_MAP_ROOT_VTABLE, true),
            (fn_008636a0, CHANGES_MAP_ROOT_VTABLE, false),
            (fn_008637a0, INTERIOR_MAP_ROOT_VTABLE, false),
        ];
        for (destructor, root, both) in destructors {
            let mut e = game();
            stub(&mut e, MAP_REMOVE_ALL);
            stub(&mut e, NI_FREE);
            let map = map_block(&mut e);
            e.set(map, NiTPointerMap::m_ppkHashTable, 0x0999_0000);

            destructor(&mut e, map);

            // The derived destructor empties the map, then the root one
            // does again and frees the bucket array.
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), if both { 2 } else { 1 });
            assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x0999_0000]]);
            assert_eq!(e.mem.u32(map.addr()), root);
        }
    }

    #[test]
    fn the_scalar_deleting_destructors_free_the_block_on_request() {
        type Scalar = fn(&mut Engine, Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>;
        // The two whose base destructor is in this part.
        let own: [(Scalar, u32); 2] = [
            (ni_t_pointer_map_unsigned_int_change_data_p_scalar_deleting_destructor, CHANGES_MAP_ROOT_VTABLE),
            (ni_t_pointer_map_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor, INTERIOR_MAP_ROOT_VTABLE),
        ];
        for (destroy, root) in own {
            for flags in [0u32, 1] {
                let mut e = game();
                stub(&mut e, MAP_REMOVE_ALL);
                stub(&mut e, NI_FREE);
                let map = map_block(&mut e);
                assert_eq!(destroy(&mut e, map, flags), map);
                assert_eq!(e.mem.u32(map.addr()), root);
                assert_eq!(e.mem.block_size(map.addr()).is_none(), flags == 1);
            }
        }
        // The ones of the other root layers.
        let others: [(Scalar, u32); 3] = [
            (ni_t_pointer_map_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor, EXTERIOR_MAP_ROOT_VTABLE),
            (ni_t_pointer_map_unsigned_int_void_p_scalar_deleting_destructor, NUMERIC_ID_MAP_ROOT_VTABLE),
            (ni_t_pointer_map_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor, STATS_MAP_ROOT_VTABLE),
        ];
        for (destroy, root) in others {
            for flags in [0u32, 1] {
                let mut e = game();
                stub(&mut e, MAP_REMOVE_ALL);
                stub(&mut e, NI_FREE);
                let map = map_block(&mut e);
                assert_eq!(destroy(&mut e, map, flags), map);
                assert_eq!(e.mem.u32(map.addr()), root);
                assert_eq!(e.mem.block_size(map.addr()).is_none(), flags == 1);
            }
        }
    }

    #[test]
    fn the_stack_map_scalar_deleting_destructor_frees_the_block_on_request() {
        for flags in [0u32, 1] {
            let mut e = game();
            stub(&mut e, LOCAL_MAP_DESTRUCT);
            let map = map_block(&mut e).cast::<()>();
            assert_eq!(fn_00863570(&mut e, map, flags), map);
            assert_eq!(calls_to(&e, LOCAL_MAP_DESTRUCT), vec![vec![map.addr()]]);
            assert_eq!(e.mem.block_size(map.addr()).is_none(), flags == 1);
        }
    }

    const BYTE_MAP_VTABLE: u32 = 0x0391_0000;
    const BYTE_MAP_FUNCTIONS: u32 = 0x0391_1000;

    /// A map with `unsigned char` keys and `buckets` buckets whose vtable
    /// holds the real `KeyToHashIndex`, `IsKeysEqual`, `SetValue` and a
    /// `NewItem` that allocates a 12-byte entry.
    fn byte_map(e: &mut Engine, buckets: u32) -> Ptr<NiTPointerMap> {
        e.put_vtable(
            BYTE_MAP_VTABLE,
            &[
                0,
                BYTE_MAP_FUNCTIONS,
                BYTE_MAP_FUNCTIONS + 4,
                BYTE_MAP_FUNCTIONS + 8,
                0,
                BYTE_MAP_FUNCTIONS + 12,
            ],
        );
        e.register(BYTE_MAP_FUNCTIONS, |e, a| {
            returns(
                ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_key_to_hash_index(
                    e,
                    Ptr::new(a[0]),
                    a[1] as u8,
                ),
            )
        });
        e.register(BYTE_MAP_FUNCTIONS + 4, |e, a| {
            returns(
                ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_is_keys_equal(
                    e,
                    Ptr::new(a[0]),
                    a[1] as u8,
                    a[2] as u8,
                ) as u32,
            )
        });
        e.register(BYTE_MAP_FUNCTIONS + 8, |e, a| {
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_set_value(
                e,
                Ptr::new(a[0]),
                Ptr::new(a[1]),
                a[2] as u8,
                a[3],
            );
            Ret::default()
        });
        e.register(BYTE_MAP_FUNCTIONS + 12, |e, _| returns(e.mem.alloc(12)));
        let map = map_block(e);
        e.mem.set_u32(map.addr(), BYTE_MAP_VTABLE);
        e.set(map, NiTPointerMap::m_uiHashSize, buckets);
        let table = e.mem.alloc(buckets * 4);
        e.set(map, NiTPointerMap::m_ppkHashTable, table);
        map
    }

    #[test]
    fn the_byte_key_methods_hash_compare_and_store() {
        let mut e = game();
        let map = byte_map(&mut e, 7);
        assert_eq!(
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_key_to_hash_index(
                &mut e, map, 0xFF
            ),
            0xFF % 7
        );
        assert!(
            ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_is_keys_equal(
                &mut e, map, 3, 3
            )
        );
        assert!(
            !ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_is_keys_equal(
                &mut e, map, 3, 4
            )
        );
        let item = Ptr::new(e.mem.alloc(12));
        ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_set_value(
            &mut e, map, item, 0x41, 0x1234,
        );
        assert_eq!(e.mem.u8(item.addr() + 4), 0x41);
        assert_eq!(e.mem.u32(item.addr() + 8), 0x1234);
    }

    #[test]
    fn set_at_adds_new_keys_to_the_bucket_head_and_replaces_known_ones() {
        let mut e = game();
        let map = byte_map(&mut e, 4);
        // Keys 1 and 5 share bucket 1.
        fn_00863a60(&mut e, map, 1, 0x100);
        fn_00863a60(&mut e, map, 5, 0x500);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let head = e.mem.u32(table + 4);
        assert_eq!(e.mem.u8(head + 4), 5);
        let second = e.mem.u32(head);
        assert_eq!(e.mem.u8(second + 4), 1);
        assert_eq!(e.mem.u32(second), 0);

        // A known key only changes the value.
        fn_00863a60(&mut e, map, 1, 0x111);
        assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 2);
        assert_eq!(e.mem.u32(second + 8), 0x111);
        assert_eq!(e.mem.u32(head + 8), 0x500);
    }

    #[test]
    fn get_at_finds_a_key_in_its_bucket_chain() {
        let mut e = game();
        let map = byte_map(&mut e, 4);
        fn_00863a60(&mut e, map, 1, 0x100);
        fn_00863a60(&mut e, map, 5, 0x500);
        let out = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(out.addr(), 0xdead);
        assert!(fn_00863b40(&mut e, map, 1, out));
        assert_eq!(e.mem.u32(out.addr()), 0x100);
        assert!(fn_00863b40(&mut e, map, 5, out));
        assert_eq!(e.mem.u32(out.addr()), 0x500);
        e.mem.set_u32(out.addr(), 0xdead);
        // Empty bucket, and a bucket whose chain lacks the key.
        assert!(!fn_00863b40(&mut e, map, 2, out));
        assert!(!fn_00863b40(&mut e, map, 9, out));
        assert_eq!(e.mem.u32(out.addr()), 0xdead);
    }

    #[test]
    fn get_next_walks_chains_then_buckets_then_ends() {
        let mut e = game();
        let map = byte_map(&mut e, 4);
        fn_00863a60(&mut e, map, 1, 0x100);
        fn_00863a60(&mut e, map, 5, 0x500);
        fn_00863a60(&mut e, map, 3, 0x300);
        let position = Ptr::new(e.mem.alloc(4));
        let key = Ptr::new(e.mem.alloc(4));
        let value = Ptr::new(e.mem.alloc(4));
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let first = e.mem.u32(table + 4);
        e.mem.set_u32(position.addr(), first);
        let mut seen = vec![];
        while e.mem.u32(position.addr()) != 0 {
            fn_00863bc0(&mut e, map, position, key, value);
            seen.push((e.mem.u8(key.addr()), e.mem.u32(value.addr())));
        }
        assert_eq!(seen, vec![(5, 0x500), (1, 0x100), (3, 0x300)]);
    }

    #[test]
    fn the_extra_destructors_set_their_vtable_and_free_the_buckets() {
        type Destructor = fn(&mut Engine, Ptr<NiTPointerMap>);
        let destructors: [(Destructor, u32, usize); 8] = [
            (fn_008638c0, EXTERIOR_MAP_ROOT_VTABLE, 1),
            (fn_008639c0, NUMERIC_ID_MAP_ROOT_VTABLE, 1),
            (fn_00863d30, STATS_MAP_ROOT_VTABLE, 1),
            (fn_00863f00, LOCAL_MAP_ROOT_VTABLE, 1),
            (fn_00863860, EXTERIOR_MAP_ROOT_VTABLE, 2),
            (fn_00863960, NUMERIC_ID_MAP_ROOT_VTABLE, 2),
            (fn_00863cd0, STATS_MAP_ROOT_VTABLE, 2),
            (fn_00863ea0, LOCAL_MAP_ROOT_VTABLE, 2),
        ];
        for (destructor, root, removals) in destructors {
            let mut e = game();
            stub(&mut e, MAP_REMOVE_ALL);
            stub(&mut e, NI_FREE);
            let map = map_block(&mut e);
            e.set(map, NiTPointerMap::m_ppkHashTable, 0x0999_0000);
            destructor(&mut e, map);
            assert_eq!(e.mem.u32(map.addr()), root);
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), removals);
            assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x0999_0000]]);
        }
    }

    #[test]
    fn the_set_value_of_the_int_key_map_stores_key_and_value() {
        let mut e = game();
        let map = map_block(&mut e);
        let item = Ptr::new(e.mem.alloc(12));
        fn_00863840(&mut e, map, item, 0x1122_3344, 0x5566);
        assert_eq!(e.mem.u32(item.addr() + 4), 0x1122_3344);
        assert_eq!(e.mem.u32(item.addr() + 8), 0x5566);
    }

    fn large_array(e: &mut Engine) -> Ptr<NiTLargePrimitiveArray> {
        e.new_object::<NiTLargePrimitiveArray>()
    }

    #[test]
    fn the_array_constructors_allocate_unless_the_size_is_zero() {
        for (size, expect_alloc) in [(5u32, true), (0, false)] {
            let mut e = game();
            e.register(ARRAY_ALLOC_BLOCK, |e, a| returns(e.mem.alloc(a[0])));
            let array = large_array(&mut e);
            e.set(array, NiTLargePrimitiveArray::m_uiSize, 9);
            e.set(array, NiTLargePrimitiveArray::m_uiESize, 9);
            assert_eq!(fn_008641d0(&mut e, array, size, 3), array);
            assert_eq!(e.mem.u32(array.addr()), FORM_ARRAY_VTABLE);
            assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiMaxSize), size);
            assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiGrowBy), 3);
            assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiSize), 0);
            assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiESize), 0);
            let base = e.get(array, NiTLargePrimitiveArray::m_pBase);
            assert_eq!(base != 0, expect_alloc);
            assert_eq!(calls_to(&e, ARRAY_ALLOC_BLOCK).len(), expect_alloc as usize);

            let plain = Ptr::new(e.mem.alloc(0x18));
            assert_eq!(fn_00864160(&mut e, plain, size, 3), plain);
            assert_eq!(e.mem.u32(plain.addr()), ARRAY_BASE_VTABLE);
            assert_eq!(e.mem.u32(plain.addr() + 0x14), 3);
        }
    }

    #[test]
    fn the_derived_array_constructor_sets_its_own_vtable_last() {
        let mut e = game();
        e.register(ARRAY_ALLOC_BLOCK, |e, a| returns(e.mem.alloc(a[0])));
        let array = large_array(&mut e);
        assert_eq!(fn_00863e00(&mut e, array, 4, 2), array);
        assert_eq!(e.mem.u32(array.addr()), FORM_ARRAY_DERIVED_VTABLE);
        assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiMaxSize), 4);
        assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiGrowBy), 2);
    }

    #[test]
    fn the_array_destructor_add_and_cleanup_work_on_the_element_block() {
        let mut e = game();
        stub(&mut e, ARRAY_FREE_BLOCK);
        stub(&mut e, ARRAY_ADD_AT);
        let array = large_array(&mut e);
        let block = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(block + 4 * i, 0x70 + i);
        }
        e.set(array, NiTLargePrimitiveArray::m_pBase, block);
        e.set(array, NiTLargePrimitiveArray::m_uiSize, 3);
        e.set(array, NiTLargePrimitiveArray::m_uiESize, 3);

        fn_00863d90(&mut e, array, 0x99);
        assert_eq!(
            calls_to(&e, ARRAY_ADD_AT),
            vec![vec![array.addr(), 3, 0x99]]
        );

        // Only the used slots are cleared.
        fn_00863db0(&mut e, array);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 12), 0x73);
        assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiSize), 0);
        assert_eq!(e.get(array, NiTLargePrimitiveArray::m_uiESize), 0);

        fn_00863d60(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), FORM_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_FREE_BLOCK), vec![vec![block]]);
    }

    #[test]
    fn the_array_scalar_deleting_destructor_frees_the_block_on_request() {
        for flags in [0u32, 1] {
            let mut e = game();
            stub(&mut e, ARRAY_DESTRUCT_BODY);
            let array = large_array(&mut e);
            assert_eq!(fn_00863f30(&mut e, array, flags), array);
            assert_eq!(calls_to(&e, ARRAY_DESTRUCT_BODY), vec![vec![array.addr()]]);
            assert_eq!(e.mem.block_size(array.addr()).is_none(), flags == 1);
        }
    }

    #[test]
    fn the_map_base_scalar_deleting_destructors_run_the_root_destructor() {
        type Scalar = fn(&mut Engine, Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>;
        let all: [(Scalar, u32); 5] = [
            (ni_t_map_base_unsigned_int_change_data_p_scalar_deleting_destructor, CHANGES_MAP_ROOT_VTABLE),
            (ni_t_map_base_unsigned_int_bs_simple_list_unsigned_int_p_scalar_deleting_destructor, INTERIOR_MAP_ROOT_VTABLE),
            (ni_t_map_base_unsigned_int_bs_simple_list_exterior_cell_reference_data_p_scalar_deleting_destructor, EXTERIOR_MAP_ROOT_VTABLE),
            (ni_t_map_base_unsigned_int_void_p_scalar_deleting_destructor, NUMERIC_ID_MAP_ROOT_VTABLE),
            (ni_t_map_base_unsigned_char_bs_simple_list_load_form_header_p_scalar_deleting_destructor, STATS_MAP_ROOT_VTABLE),
        ];
        for (destroy, root) in all {
            for flags in [0u32, 1] {
                let mut e = game();
                stub(&mut e, MAP_REMOVE_ALL);
                stub(&mut e, NI_FREE);
                let map = map_block(&mut e);
                assert_eq!(destroy(&mut e, map, flags), map);
                assert_eq!(e.mem.u32(map.addr()), root);
                assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 1);
                assert_eq!(e.mem.block_size(map.addr()).is_none(), flags == 1);
            }
        }
    }

    #[test]
    fn the_array_destructor_body_is_called_through_its_wrapper() {
        let mut e = game();
        stub(&mut e, ARRAY_DESTRUCT_BODY);
        let array = large_array(&mut e);
        fn_008640e0(&mut e, array);
        assert_eq!(calls_to(&e, ARRAY_DESTRUCT_BODY), vec![vec![array.addr()]]);
    }

    #[test]
    fn the_array_add_answers_the_index() {
        let mut e = game();
        e.register(ARRAY_ADD_AT, |_, a| a[1].into_ret());
        let array = large_array(&mut e);
        e.set(array, NiTLargePrimitiveArray::m_uiSize, 3);
        assert_eq!(fn_00863d90(&mut e, array, 0x99), 3);
    }
}
