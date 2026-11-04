//! `fallout/misc/saveload/bgssaveloadinitialdata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A changed form's "initial data" is the small record a save keeps ahead of
//! the form's changes: for a cell its world space, grid and detach time, for
//! a reference its location, angle and (for created and moved references)
//! its base object or original location. `BGSSaveLoadInitialData` picks the
//! kind of record (1 to 6), writes it, reads it back into a
//! `CellInitialData` or `ReferenceInitialData` and applies it.
//!
//! The record layouts are the Xbox PDB's (`ExteriorCellInitialDataChar`,
//! `ExteriorCellInitialDataShort`, `InteriorCellInitialData`,
//! `ReferenceLocationInitialData`, `CreatedReferenceInitialData`,
//! `MovedReferenceInitialData`, all byte packed); every offset used here was
//! checked against the PC code. The table `InitialDataTypeData[21]` (Xbox
//! PDB: `cSize`, `bRefType`, `bCellType`, three bytes per kind) is at
//! `011a2230`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSSaveLoadInitialData::InitialDataTypeData` (Xbox PDB): three bytes per
/// kind, `cSize` at +0, `bRefType` at +1, `bCellType` at +2.
const INITIAL_DATA_TYPE_DATA: u32 = 0x011a_2230;
/// Pointer to the save/load game object (`BGSSaveLoadGame`, made by
/// `BGSSaveLoadGame::Create`).
const SAVE_LOAD_GAME_POINTER: u32 = 0x011d_df38;
/// Pointer to the object the `tesdatahandler.cpp` functions run on (the
/// data handler).
const DATA_HANDLER_POINTER: u32 = 0x011c_3f2c;
/// Pointer to the player character.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// Pointer to `TES` (`TES::IsCellLoaded` runs on it).
const TES_POINTER: u32 = 0x011d_ea10;

/// Returns the word at +0xc of its object (a form's id, `TESForm::iFormID`
/// +0x0C; on the save/load game, the object `00846d20`/`00846c90` run on;
/// on a save or load buffer, its position).
const WORD_AT_0C: u32 = 0x0084_e3a0;
/// `(this = object at +0xc of the save/load game, index)`: the form id of a
/// world-space index (or the index itself when the data handler accepts it).
const FORM_ID_OF_INDEX: u32 = 0x0084_6d20;
/// `(this = same object, form id)`: the world-space index of a form id (`u16`).
const INDEX_OF_FORM_ID: u32 = 0x0084_6c90;
/// `BGSNumericIDIndex::GetNumericID` and `SetNumericID` (Xbox PDB), on a
/// three-byte compact id.
const GET_NUMERIC_ID: u32 = 0x0085_3500;
const SET_NUMERIC_ID: u32 = 0x0085_3570;
/// Form lookup by id (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `GetLocationCellOrWorld(form, &cell, &world)` (Xbox PDB, cdecl).
const GET_LOCATION_CELL_OR_WORLD: u32 = 0x0084_6190;
/// `TESWorldSpace::GetCellFromCellCoord(this, x, y)` (Xbox PDB).
const WORLD_GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
/// Truncates the `float` argument to an integer (`FLD`/`FISTP`, cdecl).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// `(this = form)`: the form's type byte (+4), zero-extended.
const FORM_TYPE: u32 = 0x0040_1170;
/// `(form type)`, cdecl: true for the reference form types.
const IS_REFERENCE_TYPE: u32 = 0x0056_4900;
/// `(this = data handler, form id)`: a bool the data handler reports for
/// the id.
const DATA_HANDLER_ID_TEST: u32 = 0x0046_9860;
/// `(this = &flags, mask)`: true when any bit of `mask` is set.
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `BGSSaveLoadFile::Read(this, destination, size)` (Xbox PDB).
const FILE_READ: u32 = 0x0084_6300;
/// `(this = load game buffer, destination, size)`: reads bytes.
const BUFFER_READ: u32 = 0x0086_4980;
/// `memcpy(destination, source, size)` (cdecl).
const MEMCPY: u32 = 0x0040_1460;
/// The `NiPoint3` default constructor (returns `this`, writes nothing).
const POINT_CONSTRUCT: u32 = 0x0068_15c0;

/// `TESObjectCELL::GetWorldSpace`, `GetDataX`, `GetDataY`, `GetDetachTime`
/// (Xbox PDB), and `00546b10(this = cell, detach time, 1)`, which the load
/// passes the detach time to.
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
const CELL_GET_DETACH_TIME: u32 = 0x0054_6af0;
const CELL_SET_DETACH_TIME: u32 = 0x0054_6b10;
/// `(this = buffer, source, size, 0)`: writes bytes to the save buffer.
const BUFFER_WRITE: u32 = 0x0086_5e50;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB) and `008d6f30` (the
/// reference's parent cell), each on a reference.
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
/// `(this = reference)`: pointers to its location and angle (`NiPoint3`).
const REFERENCE_LOCATION: u32 = 0x0043_6aa0;
const REFERENCE_ANGLE: u32 = 0x0043_0830;
/// `BGSSaveFormBuffer::GetForm` (map name; on a reference the body returns
/// the reference's base object).
const REFERENCE_BASE_OBJECT: u32 = 0x007a_f430;
/// `(this = reference)`: its extra data list.
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetLevCreaOriginalBase` and `GetSavedHavokData` (Xbox PDB).
const EXTRA_GET_LEV_CREA_ORIGINAL_BASE: u32 = 0x0042_16f0;
const EXTRA_GET_SAVED_HAVOK_DATA: u32 = 0x0042_2b90;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB).
const REFERENCE_GET_PERSISTS: u32 = 0x0056_53d0;
/// `(this = reference)`: a bool asked when slot 0xfc answers true.
const REFERENCE_FLAG_TEST: u32 = 0x0057_4900;
/// `SaveGameWarning(format, ...)` (Xbox PDB, cdecl).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// Builds a change-flags word from a value (`BGSChangeFlags`, 4 bytes).
const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// `(this = &change flags)`: a word tested for zero.
const CHANGE_FLAGS_VALUE: u32 = 0x0055_9450;
/// `BGSLoadGameSubBuffer::SaveGame(this, buffer)` (Xbox PDB).
const SUB_BUFFER_SAVE_GAME: u32 = 0x0086_5510;
/// `BGSSaveGameBuffer::StartVariableSizedValue` and
/// `SaveVariableSizedValue_ov2(this, size, start)` (Xbox PDB).
const BUFFER_START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const BUFFER_SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// `TESObjectREFR::SaveHavokData(this, buffer)` (Xbox PDB).
const REFERENCE_SAVE_HAVOK_DATA: u32 = 0x0056_2de0;

/// `__RTDynamicCast(object, 0, source type, target type, 0)` (cdecl) and
/// the two type descriptors this unit passes it (from the bound object's
/// form to `TESBoundObject`, as the load's warning about a "bound object"
/// says; the descriptors' names are not read here).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_3108;
/// The constructor the load runs on its `ReferenceInitialData` (`00849280`:
/// the two `NiPoint3` members at +0x14 and +0x20).
const REFERENCE_DATA_CONSTRUCT: u32 = 0x0084_9280;
/// `TESObjectREFR::SetLocationOnReference(this, &location)` (Xbox PDB) and
/// `00575700(this, angle x, y, z)` (the angle by value).
const REFERENCE_SET_LOCATION: u32 = 0x0057_5830;
const REFERENCE_SET_ANGLE: u32 = 0x0057_5700;
/// `005f36f0` (named `ActorMover::GetPreferredMoveMode` by the map: a
/// folded one-word getter) on a world space, then
/// `ExtraDataList::SetPersistentCell(this, cell)` (Xbox PDB).
const FOLDED_GETTER: u32 = 0x005f_36f0;
const EXTRA_SET_PERSISTENT_CELL: u32 = 0x0041_d390;
/// `(this = data handler, bound object, &location, &angle, cell, world,
/// reference, 0, 0)`: places a created reference.
const DATA_HANDLER_PLACE_REFERENCE: u32 = 0x0046_98a0;
/// `(this = bound object)`: the name the warning prints.
const BOUND_OBJECT_NAME: u32 = 0x0044_0e30;
/// `"UNKNOWN"`.
const UNKNOWN_TEXT: u32 = 0x0101_5890;
/// `(this = form, flag)`.
const FORM_SET_DELETED: u32 = 0x0046_a010;
/// `BGSSaveLoadGame::DeleteForm(this, form)` (Xbox PDB).
const GAME_DELETE_FORM: u32 = 0x0084_a9f0;
/// `(this = load buffer, 0)` (map name `BGSMenuPacker::RecomputePacking`,
/// folded).
const BUFFER_FOLDED_STEP: u32 = 0x0070_37c0;
/// `TESObjectREFR::Update3DPosition` (Xbox PDB).
const REFERENCE_UPDATE_3D_POSITION: u32 = 0x0056_2020;
/// `BGSSaveLoadGame::CancelInitPackageLocations(this, reference)` (Xbox PDB).
const GAME_CANCEL_INIT_PACKAGE_LOCATIONS: u32 = 0x0084_a580;
/// `(this = save/load game)`: true skips placing the player.
const GAME_PLAYER_TEST: u32 = 0x0063_ef00;
/// `(this = cell)`: bit 0 of the cell's flags byte.
const CELL_FLAG_ZERO_TEST: u32 = 0x0042_5fd0;
/// `TESObjectREFR::MoveRefToNewSpace(reference, cell, world)` (Xbox PDB,
/// cdecl).
const REFERENCE_MOVE_TO_NEW_SPACE: u32 = 0x0057_3800;
/// `TES::IsCellLoaded(this, cell, 0)` (Xbox PDB).
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `BGSSaveLoadGame::QueueSubBuffer(this, 2, buffer, 0)` (Xbox PDB).
const GAME_QUEUE_SUB_BUFFER: u32 = 0x0084_a810;
/// `BGSSaveLoadGame::ClearForm(this, form)` (Xbox PDB).
const GAME_CLEAR_FORM: u32 = 0x0084_a880;
/// `TESObjectREFR::CreateReference(type, 1)` (Xbox PDB, cdecl),
/// `SetObjectReference(this, object)` and `SetRefPersists(this, 1)`.
const REFERENCE_CREATE: u32 = 0x0056_4480;
const REFERENCE_SET_OBJECT: u32 = 0x0057_5690;
const REFERENCE_SET_PERSISTS: u32 = 0x0056_5480;
/// `(this = bound object of form type 0x33)`: the reference type to create.
const BOUND_OBJECT_REFERENCE_TYPE: u32 = 0x004f_e1d0;
/// The allocator (`(size)`, cdecl) and `Explosion::Explosion` (Xbox PDB).
const ALLOCATE: u32 = 0x0040_1000;
const EXPLOSION_CONSTRUCT: u32 = 0x009a_bf30;

/// Warning formats.
const WARNING_NO_EDITOR_LOCATION: u32 = 0x0107_f818;
const WARNING_UNKNOWN_TYPE: u32 = 0x0107_f86c;
const WARNING_NO_WORLD_OR_CELL: u32 = 0x0107_f894;
const WARNING_UNLOADED_CELL: u32 = 0x0107_f8d0;
const WARNING_NON_EXISTENT_CELL: u32 = 0x0107_f938;
const WARNING_INVALID_CREATED_TYPE: u32 = 0x0107_f9b8;
const WARNING_NO_BOUND_OBJECT: u32 = 0x0107_fa20;

/// Byte offsets of the two `NiPoint3` members of `ReferenceInitialData`
/// (`Location`, `Angle`, Xbox PDB) and of `ReferenceLocationInitialData`.
const REFERENCE_DATA_LOCATION: u32 = 0x14;
const REFERENCE_DATA_ANGLE: u32 = 0x20;
const LOCATION_DATA_LOCATION: u32 = 0x03;
const LOCATION_DATA_ANGLE: u32 = 0x0f;

layout! {
    /// `CellInitialData` (Xbox PDB), 0x10 bytes.
    pub struct CellInitialData: 0x10 {
        /// `iWorldSpaceID` (Xbox PDB).
        0x00 iWorldSpaceID: u32,
        /// `iCellX` (Xbox PDB).
        0x04 iCellX: i32,
        /// `iCellY` (Xbox PDB).
        0x08 iCellY: i32,
        /// `iDetachTime` (Xbox PDB).
        0x0C iDetachTime: u32,
    }

    /// `ReferenceInitialData` (Xbox PDB), 0x2d bytes; `Location` (+0x14) and
    /// `Angle` (+0x20) are `NiPoint3`s.
    pub struct ReferenceInitialData: 0x2d {
        /// `iLocationID` (Xbox PDB).
        0x00 iLocationID: u32,
        /// `iBoundID` (Xbox PDB).
        0x04 iBoundID: u32,
        /// `iOriginalLocationID` (Xbox PDB).
        0x08 iOriginalLocationID: u32,
        /// `iOriginalCellX` (Xbox PDB).
        0x0C iOriginalCellX: i32,
        /// `iOriginalCellY` (Xbox PDB).
        0x10 iOriginalCellY: i32,
        /// `cCreatedReferenceFlags` (Xbox PDB).
        0x2C cCreatedReferenceFlags: u8,
    }
}

/// Copies three words (an `NiPoint3` copied with `MOV`s, no x87).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

/// The float at `address` passed through `FLD`/`FSTP` to `FLOAT_TO_INT`,
/// shifted right by 12 (arithmetic): the cell grid coordinate.
fn cell_coordinate(e: &mut Engine, address: u32) -> i32 {
    let bits = x87_quiet_bits(e.mem.u32(address));
    e.call(FLOAT_TO_INT, &args![bits]).i32() >> 12
}

fn word_at_0c(e: &mut Engine, object: u32) -> u32 {
    e.call(WORD_AT_0C, &args![object]).u32()
}

fn flags_test(e: &mut Engine, flags: u32, mask: u32) -> bool {
    e.call(FLAGS_TEST, &args![flags, mask]).bool()
}

/// The world-space id of the save/load game's index `index`.
fn form_id_of_index(e: &mut Engine, index: u32) -> u32 {
    let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
    let ids = word_at_0c(e, game);
    e.call(FORM_ID_OF_INDEX, &args![ids, index]).u32()
}

// Translated from 0084e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `CellInitialData` from an
/// `ExteriorCellInitialDataChar` (Xbox PDB layouts): the signed byte cell
/// grid (+2, +3), the detach time (+4) and the world-space id of the index
/// at +0 (`00846d20` on the save/load game's +0xc object).
pub fn fn_0084e340(e: &mut Engine, this: Ptr<CellInitialData>, source: Ptr) {
    let source = source.addr();
    let cell_x = e.mem.i8(source + 2) as i32;
    e.set(this, CellInitialData::iCellX, cell_x);
    let cell_y = e.mem.i8(source + 3) as i32;
    e.set(this, CellInitialData::iCellY, cell_y);
    let detach_time = e.mem.u32(source + 4);
    e.set(this, CellInitialData::iDetachTime, detach_time);
    let index = e.mem.u16(source) as u32;
    let world_space = form_id_of_index(e, index);
    e.set(this, CellInitialData::iWorldSpaceID, world_space);
}

// Translated from 0084e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `CellInitialData` from an
/// `ExteriorCellInitialDataShort` (Xbox PDB layouts): the `short` cell grid
/// (+2, +4), the detach time (+6) and the world-space id of the index at +0.
pub fn fn_0084e3c0(e: &mut Engine, this: Ptr<CellInitialData>, source: Ptr) {
    let source = source.addr();
    let cell_x = e.mem.i16(source + 2) as i32;
    e.set(this, CellInitialData::iCellX, cell_x);
    let cell_y = e.mem.i16(source + 4) as i32;
    e.set(this, CellInitialData::iCellY, cell_y);
    let detach_time = e.mem.u32(source + 6);
    e.set(this, CellInitialData::iDetachTime, detach_time);
    let index = e.mem.u16(source) as u32;
    let world_space = form_id_of_index(e, index);
    e.set(this, CellInitialData::iWorldSpaceID, world_space);
}

// Translated from 0084e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `CellInitialData` from an
/// `InteriorCellInitialData` (Xbox PDB): grid 0, the detach time, world 0.
pub fn fn_0084e420(e: &mut Engine, this: Ptr<CellInitialData>, source: Ptr) {
    e.set(this, CellInitialData::iCellX, 0);
    e.set(this, CellInitialData::iCellY, 0);
    let detach_time = e.mem.u32(source.addr());
    e.set(this, CellInitialData::iDetachTime, detach_time);
    e.set(this, CellInitialData::iWorldSpaceID, 0);
}

/// Copies `Location` and `Angle` of a `ReferenceLocationInitialData` (+3,
/// +0xf) into a `ReferenceInitialData` (+0x14, +0x20).
fn copy_location_and_angle(e: &mut Engine, this: u32, source: u32) {
    copy_point(
        e,
        source + LOCATION_DATA_LOCATION,
        this + REFERENCE_DATA_LOCATION,
    );
    copy_point(e, source + LOCATION_DATA_ANGLE, this + REFERENCE_DATA_ANGLE);
}

// Translated from 0084e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `ReferenceInitialData` from a
/// `ReferenceLocationInitialData` (Xbox PDB layouts): the location id
/// (`GetNumericID` of the compact id at +0), bound and original location 0,
/// location and angle, flags 0.
pub fn fn_0084e460(e: &mut Engine, this: Ptr<ReferenceInitialData>, source: Ptr) {
    let location = e.call(GET_NUMERIC_ID, &args![source]).u32();
    e.set(this, ReferenceInitialData::iLocationID, location);
    e.set(this, ReferenceInitialData::iBoundID, 0);
    e.set(this, ReferenceInitialData::iOriginalLocationID, 0);
    e.set(this, ReferenceInitialData::iOriginalCellX, 0);
    e.set(this, ReferenceInitialData::iOriginalCellY, 0);
    copy_location_and_angle(e, this.addr(), source.addr());
    e.set(this, ReferenceInitialData::cCreatedReferenceFlags, 0);
}

// Translated from 0084e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `ReferenceInitialData` from a
/// `CreatedReferenceInitialData` (Xbox PDB layouts): location id, bound id
/// (compact id at +0x1c), location and angle, and the created-reference
/// flags byte (+0x1b).
pub fn fn_0084e4f0(e: &mut Engine, this: Ptr<ReferenceInitialData>, source: Ptr) {
    let location = e.call(GET_NUMERIC_ID, &args![source]).u32();
    e.set(this, ReferenceInitialData::iLocationID, location);
    let bound = e.call(GET_NUMERIC_ID, &args![source.byte_add(0x1c)]).u32();
    e.set(this, ReferenceInitialData::iBoundID, bound);
    e.set(this, ReferenceInitialData::iOriginalLocationID, 0);
    e.set(this, ReferenceInitialData::iOriginalCellX, 0);
    e.set(this, ReferenceInitialData::iOriginalCellY, 0);
    copy_location_and_angle(e, this.addr(), source.addr());
    let flags = e.mem.u8(source.addr() + 0x1b);
    e.set(this, ReferenceInitialData::cCreatedReferenceFlags, flags);
}

// Translated from 0084e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fills a `ReferenceInitialData` from a
/// `MovedReferenceInitialData` (Xbox PDB layouts): location id, bound 0,
/// original location id (compact id at +0x1b) and its `short` cell grid
/// (+0x1e, +0x20), location and angle, flags 0.
pub fn fn_0084e580(e: &mut Engine, this: Ptr<ReferenceInitialData>, source: Ptr) {
    let location = e.call(GET_NUMERIC_ID, &args![source]).u32();
    e.set(this, ReferenceInitialData::iLocationID, location);
    e.set(this, ReferenceInitialData::iBoundID, 0);
    let original = e.call(GET_NUMERIC_ID, &args![source.byte_add(0x1b)]).u32();
    e.set(this, ReferenceInitialData::iOriginalLocationID, original);
    let cell_x = e.mem.i16(source.addr() + 0x1e) as i32;
    e.set(this, ReferenceInitialData::iOriginalCellX, cell_x);
    let cell_y = e.mem.i16(source.addr() + 0x20) as i32;
    e.set(this, ReferenceInitialData::iOriginalCellY, cell_y);
    copy_location_and_angle(e, this.addr(), source.addr());
    e.set(this, ReferenceInitialData::cCreatedReferenceFlags, 0);
}

// Translated from 0084e610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReferenceInitialData::GetLocationCellAndWorld` (Xbox PDB): looks up the
/// location form and lets `GetLocationCellOrWorld` fill `cell` and `world`;
/// when a world was found, `cell` becomes the world's cell at the grid of
/// `Location`'s x and y (truncated, shifted right by 12). Returns what
/// `GetLocationCellOrWorld` returned.
pub fn reference_initial_data_get_location_cell_and_world(
    e: &mut Engine,
    this: Ptr<ReferenceInitialData>,
    cell: Ptr,
    world: Ptr,
) -> bool {
    let location_id = e.get(this, ReferenceInitialData::iLocationID);
    let location = e.call(LOOKUP_FORM, &args![location_id]).u32();
    let found = e
        .call(GET_LOCATION_CELL_OR_WORLD, &args![location, cell, world])
        .bool();
    if e.mem.u32(world.addr()) != 0 {
        let x = cell_coordinate(e, this.addr() + REFERENCE_DATA_LOCATION);
        let y = cell_coordinate(e, this.addr() + REFERENCE_DATA_LOCATION + 4);
        let world_space = e.mem.u32(world.addr());
        let grid_cell = e
            .call(WORLD_GET_CELL_FROM_CELL_COORD, &args![world_space, x, y])
            .u32();
        e.mem.set_u32(cell.addr(), grid_cell);
    }
    found
}

// Translated from 0084e6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReferenceInitialData::GetOriginalLocationCellAndWorld` (Xbox PDB): like
/// `GetLocationCellAndWorld` for the original location; the cell of a world
/// is the one at `iOriginalCellX`/`iOriginalCellY`.
pub fn reference_initial_data_get_original_location_cell_and_world(
    e: &mut Engine,
    this: Ptr<ReferenceInitialData>,
    cell: Ptr,
    world: Ptr,
) -> bool {
    let location_id = e.get(this, ReferenceInitialData::iOriginalLocationID);
    let location = e.call(LOOKUP_FORM, &args![location_id]).u32();
    let found = e
        .call(GET_LOCATION_CELL_OR_WORLD, &args![location, cell, world])
        .bool();
    if e.mem.u32(world.addr()) != 0 {
        let y = e.get(this, ReferenceInitialData::iOriginalCellY);
        let x = e.get(this, ReferenceInitialData::iOriginalCellX);
        let world_space = e.mem.u32(world.addr());
        let grid_cell = e
            .call(WORLD_GET_CELL_FROM_CELL_COORD, &args![world_space, x, y])
            .u32();
        e.mem.set_u32(cell.addr(), grid_cell);
    }
    found
}

// Translated from 0084e700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::GetInitialDataType` (Xbox PDB, cdecl): the kind of
/// initial data of `form` (`0084e730` on its id, its type byte and the
/// change flags). The flags reach `0084e730` as the word the call of the
/// type getter (`00401170`, which takes none) leaves on the stack.
pub fn bgs_save_load_initial_data_get_initial_data_type(
    e: &mut Engine,
    form: Ptr,
    change_flags: u32,
) -> u32 {
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    let form_id = word_at_0c(e, form.addr());
    fn_0084e730(e, form_id, form_type, change_flags)
}

// Translated from 0084e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): the kind of initial data. For a
/// reference form type: 5 when the data handler reports the id
/// (`00469860`), else 6 for change flag 8, 4 for flags 2 or 4, else 0. For
/// form type 0x39 with flag 0x40000000: 1 with flag 0x20000000, else 2 with
/// flag 0x10000000, else 3. Anything else 0. The flags are tested through
/// their address (the argument's own stack slot).
pub fn fn_0084e730(e: &mut Engine, form_id: u32, form_type: u32, change_flags: u32) -> u32 {
    e.with_stack(4, |e, flags| {
        let flags = flags.addr();
        e.mem.set_u32(flags, change_flags);
        if e.call(IS_REFERENCE_TYPE, &args![form_type]).bool() {
            let data_handler: u32 = e.global(DATA_HANDLER_POINTER);
            if e.call(DATA_HANDLER_ID_TEST, &args![data_handler, form_id])
                .bool()
            {
                return 5;
            }
            if flags_test(e, flags, 8) {
                return 6;
            }
            if flags_test(e, flags, 6) {
                return 4;
            }
            0
        } else if form_type == 0x39 && flags_test(e, flags, 0x4000_0000) {
            if flags_test(e, flags, 0x2000_0000) {
                1
            } else if flags_test(e, flags, 0x1000_0000) {
                2
            } else {
                3
            }
        } else {
            0
        }
    })
}

// Translated from 0084e800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::GetInitialDataTypeDataSize` (Xbox PDB, cdecl):
/// the kind's record size plus one (the kind byte).
pub fn bgs_save_load_initial_data_get_initial_data_type_data_size(
    e: &mut Engine,
    data_type: u32,
) -> u32 {
    fn_0084e830(e, data_type) + 1
}

// Translated from 0084e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `cSize` of the kind's
/// `InitialDataTypeData` entry, zero-extended.
pub fn fn_0084e830(e: &mut Engine, data_type: u32) -> u32 {
    let entry = INITIAL_DATA_TYPE_DATA.wrapping_add(data_type.wrapping_mul(3));
    e.mem.u8(entry) as u32
}

// Translated from 0084e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::LoadInitialDataStruct` (Xbox PDB, cdecl): for a
/// kind other than 0, reads the kind's size plus one bytes from the save
/// file (`BGSSaveLoadFile::Read`) into a 128-byte local and converts them
/// into `out` (`LoadInitialDataStruct_ov3`), returning what that returns; 0
/// for kind 0.
// Translation incomplete: 0084e850 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bgs_save_load_initial_data_load_initial_data_struct(
    e: &mut Engine,
    file: Ptr,
    data_type: u32,
    out: Ptr,
) -> u32 {
    if data_type == 0 {
        return 0;
    }
    let size = bgs_save_load_initial_data_get_initial_data_type_data_size(e, data_type);
    e.with_stack(0x80, |e, record| {
        e.call(FILE_READ, &args![file, record, size]);
        bgs_save_load_initial_data_load_initial_data_struct_ov3(e, record, data_type, out)
    })
}

// Translated from 0084e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::LoadInitialDataStruct_ov2` (Xbox PDB, cdecl):
/// like `LoadInitialDataStruct` from a load game buffer (`00864980`),
/// reading the kind's size (no kind byte).
// Translation incomplete: 0084e8c0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn bgs_save_load_initial_data_load_initial_data_struct_ov2(
    e: &mut Engine,
    buffer: Ptr,
    data_type: u32,
    out: Ptr,
) -> u32 {
    if data_type == 0 {
        return 0;
    }
    let size = fn_0084e830(e, data_type);
    e.with_stack(0x80, |e, record| {
        e.call(BUFFER_READ, &args![buffer, record, size]);
        bgs_save_load_initial_data_load_initial_data_struct_ov3(e, record, data_type, out)
    })
}

// Translated from 0084e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::LoadInitialDataStruct_ov3` (Xbox PDB, cdecl):
/// copies the kind's size of bytes from `source` into a local record of the
/// kind (constructed first for kinds 4 to 6) and converts it into `out`: a
/// `CellInitialData` for kinds 1 to 3, a `ReferenceInitialData` for 4 to 6;
/// other kinds leave `out` alone. Returns the kind's size plus one.
pub fn bgs_save_load_initial_data_load_initial_data_struct_ov3(
    e: &mut Engine,
    source: Ptr,
    data_type: u32,
    out: Ptr,
) -> u32 {
    let size = fn_0084e830(e, data_type);
    // The size of the kind's local record in the game's frame.
    let frame_size = match data_type {
        1 => 8,
        2 => 0xc,
        3 => 4,
        4 => 0x1c,
        5 => 0x20,
        6 => 0x24,
        _ => 0,
    };
    if frame_size != 0 {
        e.with_stack(frame_size, |e, record| {
            match data_type {
                4 => {
                    fn_0084eac0(e, record);
                }
                5 => {
                    fn_0084eb20(e, record);
                }
                6 => {
                    fn_0084eb50(e, record);
                }
                _ => {}
            }
            e.call(MEMCPY, &args![record, source, size]);
            match data_type {
                1 => fn_0084e340(e, out.cast(), record),
                2 => fn_0084e3c0(e, out.cast(), record),
                3 => fn_0084e420(e, out.cast(), record),
                4 => fn_0084e460(e, out.cast(), record),
                5 => fn_0084e4f0(e, out.cast(), record),
                _ => fn_0084e580(e, out.cast(), record),
            }
        });
    }
    bgs_save_load_initial_data_get_initial_data_type_data_size(e, data_type)
}

// Translated from 0084eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs a `ReferenceLocationInitialData`
/// (Xbox PDB): its compact id (`0084eaf0`) and its two `NiPoint3`s (+3,
/// +0xf). Returns `this`.
pub fn fn_0084eac0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_compact_id(e, this);
    e.call(
        POINT_CONSTRUCT,
        &args![this.byte_add(LOCATION_DATA_LOCATION)],
    );
    e.call(POINT_CONSTRUCT, &args![this.byte_add(LOCATION_DATA_ANGLE)]);
    this
}

// Translated from 0084eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears exactly the three bytes of the compact form identifier and
/// returns the receiver. PC stores bytes individually; padding is untouched.
pub fn construct_compact_id(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in 0..3 {
        e.mem.set_u8(this.addr().wrapping_add(offset), 0);
    }
    this
}

// Translated from 0084eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs a `CreatedReferenceInitialData`
/// (Xbox PDB): its location record and the bound object's compact id
/// (+0x1c). Returns `this`.
pub fn fn_0084eb20(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0084eac0(e, this);
    construct_compact_id(e, this.byte_add(0x1c));
    this
}

// Translated from 0084eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs a `MovedReferenceInitialData`
/// (Xbox PDB): its location record and the original location's compact id
/// (+0x1b). Returns `this`.
pub fn fn_0084eb50(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0084eac0(e, this);
    construct_compact_id(e, this.byte_add(0x1b));
    this
}

/// The world space (or parent cell) of `reference`, as the save takes it.
fn reference_world_or_cell(e: &mut Engine, reference: u32) -> u32 {
    let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
    if world != 0 {
        world
    } else {
        e.call(REFERENCE_PARENT_CELL, &args![reference]).u32()
    }
}

/// The save's location of a reference for kinds 4 and 6: its world (or
/// cell), location and angle into the three locals; when it has neither,
/// the location slot 0x138 reports (with the locals' addresses and 0), and
/// when that fails, the player's world (or cell) with the reference's own
/// location and angle again.
fn save_reference_location(e: &mut Engine, reference: u32, world: u32, location: u32, angle: u32) {
    let world_space = reference_world_or_cell(e, reference);
    e.mem.set_u32(world, world_space);
    let source = e.call(REFERENCE_LOCATION, &args![reference]).u32();
    copy_point(e, source, location);
    let source = e.call(REFERENCE_ANGLE, &args![reference]).u32();
    copy_point(e, source, angle);
    if e.mem.u32(world) == 0
        && !e
            .vcall(reference, 0x138, &args![location, angle, world, 0u32])
            .bool()
    {
        let player: u32 = e.global(PLAYER_CHARACTER);
        let world_space = reference_world_or_cell(e, player);
        e.mem.set_u32(world, world_space);
        let source = e.call(REFERENCE_LOCATION, &args![reference]).u32();
        copy_point(e, source, location);
        let source = e.call(REFERENCE_ANGLE, &args![reference]).u32();
        copy_point(e, source, angle);
    }
}

/// Fills the `ReferenceLocationInitialData` at `record`: the world's id as
/// its compact id, then the location and angle.
fn save_location_record(e: &mut Engine, record: u32, world: u32, location: u32, angle: u32) {
    let world_space = e.mem.u32(world);
    let world_id = word_at_0c(e, world_space);
    e.call(SET_NUMERIC_ID, &args![record, world_id]);
    copy_point(e, location, record + LOCATION_DATA_LOCATION);
    copy_point(e, angle, record + LOCATION_DATA_ANGLE);
}

// Translated from 0084eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::SaveInitialData` (Xbox PDB, cdecl): writes the
/// initial data of `form` for its kind (`GetInitialDataType`) to `buffer`
/// and returns the kind.
///
/// - 1, 2: an exterior cell: the world space's index (`00846c90`), the
///   cell's grid as bytes (8 bytes) or shorts (10), and its detach time.
/// - 3: an interior cell: the detach time (4 bytes).
/// - 4: a reference's world (or cell), location and angle (0x1b bytes; see
///   `save_reference_location`).
/// - 5: a created reference: world (or cell), the location of slot 0x1f4,
///   the angle, flags (1, or 2 when it persists, or 4 when slot 0xfc and
///   `00574900` answer true) and its base object; with slot 0x100 true and
///   change flag 0x40000, a levelled creature's original base replaces the
///   base (0x1f bytes).
/// - 6: a moved reference: like 4, plus its editor location (slot 0x138
///   with fresh locals): the editor world's id and the location's grid,
///   or grid 0 and a warning naming the reference and its base when there
///   is none (0x22 bytes).
///
/// Then, for any kind, when slot 0xf0 answers true and the change flags
/// have bit 4: the Havok data. With slot 0x1d0 returning 0 and the
/// extra-list's saved Havok data's change-flags word (`00559450`) non-zero,
/// that sub buffer is saved as it is; otherwise the reference saves it
/// (`SaveHavokData`) as a variable-sized value.
pub fn bgs_save_load_initial_data_save_initial_data(
    e: &mut Engine,
    buffer: Ptr,
    form: Ptr,
    change_flags: u32,
) -> u32 {
    let buffer = buffer.addr();
    let form = form.addr();
    let data_type =
        bgs_save_load_initial_data_get_initial_data_type(e, Ptr::new(form), change_flags);
    // The change-flags argument's stack slot, tested through its address.
    e.with_stack(4, |e, flags| {
        let flags = flags.addr();
        e.mem.set_u32(flags, change_flags);
        match data_type {
            1 | 2 => e.with_stack(0xc, |e, record| {
                let record = record.addr();
                let world_space = e.call(CELL_GET_WORLD_SPACE, &args![form]).u32();
                let world_id = word_at_0c(e, world_space);
                let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
                let ids = word_at_0c(e, game);
                let index = e.call(INDEX_OF_FORM_ID, &args![ids, world_id]).u16();
                e.mem.set_u16(record, index);
                let x = e.call(CELL_GET_DATA_X, &args![form]).u32();
                let y = e.call(CELL_GET_DATA_Y, &args![form]).u32();
                let detach_time = e.call(CELL_GET_DETACH_TIME, &args![form]).u32();
                let size = if data_type == 1 {
                    e.mem.set_u8(record + 2, x as u8);
                    e.mem.set_u8(record + 3, y as u8);
                    e.mem.set_u32(record + 4, detach_time);
                    8u32
                } else {
                    e.mem.set_u16(record + 2, x as u16);
                    e.mem.set_u16(record + 4, y as u16);
                    e.mem.set_u32(record + 6, detach_time);
                    10
                };
                e.call(BUFFER_WRITE, &args![buffer, record, size, 0u32]);
            }),
            3 => e.with_stack(4, |e, record| {
                let detach_time = e.call(CELL_GET_DETACH_TIME, &args![form]).u32();
                e.mem.set_u32(record.addr(), detach_time);
                e.call(BUFFER_WRITE, &args![buffer, record, 4u32, 0u32]);
            }),
            4 => e.with_stack(0x40, |e, frame| {
                let frame = frame.addr();
                let (world, angle, location, record) =
                    (frame, frame + 4, frame + 0x10, frame + 0x1c);
                save_reference_location(e, form, world, location, angle);
                fn_0084eac0(e, Ptr::new(record));
                save_location_record(e, record, world, location, angle);
                e.call(BUFFER_WRITE, &args![buffer, record, 0x1bu32, 0u32]);
            }),
            5 => e.with_stack(0x20, |e, record| {
                save_created_reference(e, buffer, form, flags, record.addr());
            }),
            6 => e.with_stack(0x5c, |e, frame| {
                save_moved_reference(e, buffer, form, frame.addr());
            }),
            _ => {}
        }

        if e.vcall(form, 0xf0, &args![]).bool() && flags_test(e, flags, 4) {
            let list = e.call(REFERENCE_EXTRA_LIST, &args![form]).u32();
            let havok_data = e.call(EXTRA_GET_SAVED_HAVOK_DATA, &args![list]).u32();
            e.with_stack(4, |e, havok_flags| {
                e.call(CHANGE_FLAGS_CONSTRUCT, &args![havok_flags, havok_data]);
                if e.vcall(form, 0x1d0, &args![]).u32() == 0
                    && e.call(CHANGE_FLAGS_VALUE, &args![havok_flags]).u32() != 0
                {
                    e.call(SUB_BUFFER_SAVE_GAME, &args![havok_flags, buffer]);
                } else {
                    let start = e
                        .call(BUFFER_START_VARIABLE_SIZED_VALUE, &args![buffer])
                        .u32();
                    let before = word_at_0c(e, buffer);
                    e.call(REFERENCE_SAVE_HAVOK_DATA, &args![form, buffer]);
                    let after = word_at_0c(e, buffer);
                    e.call(
                        BUFFER_SAVE_VARIABLE_SIZED_VALUE,
                        &args![buffer, after.wrapping_sub(before), start],
                    );
                }
            });
        }
    });
    data_type
}

/// Kind 5 of `SaveInitialData`: the `CreatedReferenceInitialData` at
/// `record` (0x20 bytes of frame).
fn save_created_reference(e: &mut Engine, buffer: u32, reference: u32, flags: u32, record: u32) {
    let world_space = reference_world_or_cell(e, reference);
    fn_0084eb20(e, Ptr::new(record));
    let world_id = word_at_0c(e, world_space);
    e.call(SET_NUMERIC_ID, &args![record, world_id]);
    let location = e.vcall(reference, 0x1f4, &args![]).u32();
    copy_point(e, location, record + LOCATION_DATA_LOCATION);
    let angle = e.call(REFERENCE_ANGLE, &args![reference]).u32();
    copy_point(e, angle, record + LOCATION_DATA_ANGLE);
    let mut base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
    if e.vcall(reference, 0x100, &args![]).bool() && flags_test(e, flags, 0x40000) {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        let original = e.call(EXTRA_GET_LEV_CREA_ORIGINAL_BASE, &args![list]).u32();
        if original != 0 {
            base = original;
        }
    }
    let base_id = word_at_0c(e, base);
    e.call(SET_NUMERIC_ID, &args![record + 0x1c, base_id]);
    let flags_byte = record + 0x1b;
    e.mem.set_u8(flags_byte, 1);
    if e.call(REFERENCE_GET_PERSISTS, &args![reference]).bool() {
        let value = e.mem.u8(flags_byte);
        e.mem.set_u8(flags_byte, value | 2);
    }
    if e.vcall(reference, 0xfc, &args![]).bool()
        && e.call(REFERENCE_FLAG_TEST, &args![reference]).bool()
    {
        let value = e.mem.u8(flags_byte);
        e.mem.set_u8(flags_byte, value | 4);
    }
    e.call(BUFFER_WRITE, &args![buffer, record, 0x1fu32, 0u32]);
}

/// Kind 6 of `SaveInitialData`, in a 0x5c-byte frame: world (+0), angle
/// (+4), location (+0x10), editor world (+0x1c), the
/// `MovedReferenceInitialData` (+0x20, 0x24 bytes), editor location
/// (+0x44) and editor angle (+0x50).
fn save_moved_reference(e: &mut Engine, buffer: u32, reference: u32, frame: u32) {
    let (world, angle, location) = (frame, frame + 4, frame + 0x10);
    let editor_world = frame + 0x1c;
    let record = frame + 0x20;
    let editor_location = frame + 0x44;
    let editor_angle = frame + 0x50;
    save_reference_location(e, reference, world, location, angle);
    fn_0084eb50(e, Ptr::new(record));
    save_location_record(e, record, world, location, angle);
    e.call(POINT_CONSTRUCT, &args![editor_location]);
    e.call(POINT_CONSTRUCT, &args![editor_angle]);
    e.mem.set_u32(editor_world, 0);
    if e.vcall(
        reference,
        0x138,
        &args![editor_location, editor_angle, editor_world, 0u32],
    )
    .bool()
    {
        let world_space = e.mem.u32(editor_world);
        let world_id = word_at_0c(e, world_space);
        e.call(SET_NUMERIC_ID, &args![record + 0x1b, world_id]);
        let x = cell_coordinate(e, editor_location);
        e.mem.set_u16(record + 0x1e, x as u16);
        let y = cell_coordinate(e, editor_location + 4);
        e.mem.set_u16(record + 0x20, y as u16);
    } else {
        e.mem.set_u16(record + 0x1e, 0);
        e.mem.set_u16(record + 0x20, 0);
        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        let base_again = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        let base_id = word_at_0c(e, base_again);
        let base_name = e.vcall(base, 0x130, &args![]).u32();
        let reference_id = word_at_0c(e, reference);
        let reference_name = e.vcall(reference, 0x130, &args![]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![
                WARNING_NO_EDITOR_LOCATION,
                reference_name,
                reference_id,
                base_name,
                base_id
            ],
        );
    }
    e.call(BUFFER_WRITE, &args![buffer, record, 0x22u32, 0u32]);
}

// Translated from 0084f330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::LoadInitialData` (Xbox PDB, cdecl): reads the
/// initial data of `form`'s kind from `buffer` and applies it; returns the
/// kind.
///
/// - A cell kind (`bCellType`): the record's detach time goes to the cell
///   (`00546b10(cell, time, 1)`).
/// - Not a reference kind (`bRefType`): kind 0 is ignored, any other is
///   warned about.
/// - A reference kind: the `ReferenceInitialData` is read and its cell and
///   world found (`GetLocationCellAndWorld`). Kind 5 (created): the bound
///   object (dynamic cast of the looked-up `iBoundID`); with flag 4 the
///   reference gets its location, angle and cell (slot 0x228) or persistent
///   world cell, else the data handler places it (`004698a0`); with neither
///   cell nor world it is warned about, deleted and dropped
///   (`BGSSaveLoadGame::DeleteForm`) and the buffer told (`007037c0`);
///   otherwise its 3D position is updated and, for slot 0x100, its package
///   locations cancelled. Kinds 4 and 6 (moved): angle then location (the
///   player only when `0063ef00` says no), 3D position; for anyone but the
///   player the reference is moved to its cell or world, or warned about
///   (an unloaded cell for a non-persistent reference marks the buffer,
///   `0084f880`), its 3D state is unloaded (slot 0x1cc) or refreshed (slot
///   0x1c4); then package locations, and for kind 6 without slot 0x290 the
///   original location (`0084f8c0`).
/// - With change flag 4 (for every reference kind but a dropped created
///   one), the buffer is queued as sub buffer 2.
pub fn bgs_save_load_initial_data_load_initial_data(
    e: &mut Engine,
    buffer: Ptr,
    form: Ptr,
    change_flags: u32,
) -> u32 {
    let data_type = bgs_save_load_initial_data_get_initial_data_type(e, form, change_flags);
    if fn_0084f860(e, data_type) as u8 != 0 {
        e.with_stack(0x10, |e, cell_data| {
            bgs_save_load_initial_data_load_initial_data_struct_ov2(
                e, buffer, data_type, cell_data,
            );
            let detach_time = e.mem.u32(cell_data.addr() + 0xc);
            e.call(CELL_SET_DETACH_TIME, &args![form, detach_time, 1u32]);
        });
        return data_type;
    }
    if fn_0084f840(e, data_type) as u8 == 0 {
        if data_type != 0 {
            e.call(SAVE_GAME_WARNING, &args![WARNING_UNKNOWN_TYPE, data_type]);
        }
        return data_type;
    }
    // Frame: the reference data (+0, 0x2d bytes), cell (+0x30), world
    // (+0x34), change flags (+0x38).
    e.with_stack(0x3c, |e, frame| {
        let data: Ptr<ReferenceInitialData> = frame.cast();
        let (cell, world, flags) = (
            frame.addr() + 0x30,
            frame.addr() + 0x34,
            frame.addr() + 0x38,
        );
        e.mem.set_u32(flags, change_flags);
        e.call(REFERENCE_DATA_CONSTRUCT, &args![data]);
        bgs_save_load_initial_data_load_initial_data_struct_ov2(e, buffer, data_type, data.cast());
        e.mem.set_u32(world, 0);
        e.mem.set_u32(cell, 0);
        reference_initial_data_get_location_cell_and_world(
            e,
            data,
            Ptr::new(cell),
            Ptr::new(world),
        );
        let reference = form.addr();
        let applied = match data_type {
            4 | 6 => {
                load_moved_reference(e, buffer.addr(), reference, data, data_type, cell, world);
                true
            }
            5 => load_created_reference(e, buffer.addr(), reference, data, cell, world),
            _ => true,
        };
        if applied && flags_test(e, flags, 4) {
            let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
            e.call(GAME_QUEUE_SUB_BUFFER, &args![game, 2u32, buffer, 0u32]);
        }
    });
    data_type
}

/// The location (`&location`) and angle (by value) of the reference data.
fn set_angle_then_location(e: &mut Engine, reference: u32, data: u32) {
    let angle = data + REFERENCE_DATA_ANGLE;
    let (x, y, z) = (e.mem.u32(angle), e.mem.u32(angle + 4), e.mem.u32(angle + 8));
    e.call(REFERENCE_SET_ANGLE, &args![reference, x, y, z]);
    e.call(
        REFERENCE_SET_LOCATION,
        &args![reference, data + REFERENCE_DATA_LOCATION],
    );
}

/// Kind 5 of `LoadInitialData`. False when the reference was dropped (the
/// load returns without queueing the buffer).
fn load_created_reference(
    e: &mut Engine,
    buffer: u32,
    reference: u32,
    data: Ptr<ReferenceInitialData>,
    cell: u32,
    world: u32,
) -> bool {
    let bound_id = e.get(data, ReferenceInitialData::iBoundID);
    let bound_form = e.call(LOOKUP_FORM, &args![bound_id]).u32();
    let bound = e
        .call(
            DYNAMIC_CAST,
            &args![bound_form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    if e.get(data, ReferenceInitialData::cCreatedReferenceFlags) & 4 != 0 {
        e.call(
            REFERENCE_SET_LOCATION,
            &args![reference, data.addr() + REFERENCE_DATA_LOCATION],
        );
        let angle = data.addr() + REFERENCE_DATA_ANGLE;
        let (x, y, z) = (e.mem.u32(angle), e.mem.u32(angle + 4), e.mem.u32(angle + 8));
        e.call(REFERENCE_SET_ANGLE, &args![reference, x, y, z]);
        let cell_form = e.mem.u32(cell);
        if cell_form != 0 {
            e.vcall(reference, 0x228, &args![cell_form]);
        } else {
            let world_space = e.mem.u32(world);
            if world_space != 0 {
                let persistent_cell = e.call(FOLDED_GETTER, &args![world_space]).u32();
                let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                e.call(EXTRA_SET_PERSISTENT_CELL, &args![list, persistent_cell]);
            }
        }
    } else {
        let data_handler: u32 = e.global(DATA_HANDLER_POINTER);
        let (cell_form, world_space) = (e.mem.u32(cell), e.mem.u32(world));
        e.call(
            DATA_HANDLER_PLACE_REFERENCE,
            &args![
                data_handler,
                bound,
                data.addr() + REFERENCE_DATA_LOCATION,
                data.addr() + REFERENCE_DATA_ANGLE,
                cell_form,
                world_space,
                reference,
                0u32,
                0u32
            ],
        );
    }
    if e.mem.u32(cell) == 0 && e.mem.u32(world) == 0 {
        let name = if bound != 0 {
            e.call(BOUND_OBJECT_NAME, &args![bound]).u32()
        } else {
            UNKNOWN_TEXT
        };
        let reference_id = word_at_0c(e, reference);
        e.call(
            SAVE_GAME_WARNING,
            &args![WARNING_NON_EXISTENT_CELL, reference_id, bound_id, name],
        );
        e.call(FORM_SET_DELETED, &args![reference, 0u32]);
        let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
        e.call(GAME_DELETE_FORM, &args![game, reference]);
        e.call(BUFFER_FOLDED_STEP, &args![buffer, 0u32]);
        return false;
    }
    e.call(REFERENCE_UPDATE_3D_POSITION, &args![reference]);
    if e.vcall(reference, 0x100, &args![]).bool() {
        let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
        e.call(GAME_CANCEL_INIT_PACKAGE_LOCATIONS, &args![game, reference]);
    }
    true
}

/// True when `TES` reports `cell` loaded.
fn is_cell_loaded(e: &mut Engine, cell: u32) -> bool {
    let tes: u32 = e.global(TES_POINTER);
    e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool()
}

/// Kinds 4 and 6 of `LoadInitialData`.
fn load_moved_reference(
    e: &mut Engine,
    buffer: u32,
    reference: u32,
    data: Ptr<ReferenceInitialData>,
    data_type: u32,
    cell: u32,
    world: u32,
) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    if reference == player {
        let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
        if !e.call(GAME_PLAYER_TEST, &args![game]).bool() {
            set_angle_then_location(e, reference, data.addr());
        }
    } else {
        set_angle_then_location(e, reference, data.addr());
    }
    e.call(REFERENCE_UPDATE_3D_POSITION, &args![reference]);

    let player: u32 = e.global(PLAYER_CHARACTER);
    if reference != player {
        let cell_form = e.mem.u32(cell);
        if cell_form != 0 && e.call(CELL_FLAG_ZERO_TEST, &args![cell_form]).bool() {
            let parent = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
            if cell_form != parent {
                e.call(
                    REFERENCE_MOVE_TO_NEW_SPACE,
                    &args![reference, cell_form, 0u32],
                );
            }
        } else if e.mem.u32(world) != 0 {
            let keep = if cell_form == 0 {
                false
            } else {
                let parent = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                cell_form == parent || is_cell_loaded(e, cell_form)
            };
            if keep || e.call(REFERENCE_GET_PERSISTS, &args![reference]).bool() {
                let move_it = if cell_form == 0 {
                    true
                } else {
                    let parent = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                    cell_form != parent
                };
                if move_it {
                    let world_space = e.mem.u32(world);
                    e.call(
                        REFERENCE_MOVE_TO_NEW_SPACE,
                        &args![reference, 0u32, world_space],
                    );
                }
            } else {
                let world_space = e.mem.u32(world);
                let world_id = word_at_0c(e, world_space);
                let world_name = e.vcall(world_space, 0x130, &args![]).u32();
                let reference_id = word_at_0c(e, reference);
                let reference_name = e.vcall(reference, 0x130, &args![]).u32();
                e.call(
                    SAVE_GAME_WARNING,
                    &args![
                        WARNING_UNLOADED_CELL,
                        reference_name,
                        reference_id,
                        world_name,
                        world_id
                    ],
                );
                fn_0084f880(e, Ptr::new(buffer), true);
            }
        } else {
            let reference_id = word_at_0c(e, reference);
            let reference_name = e.vcall(reference, 0x130, &args![]).u32();
            let location_id = e.get(data, ReferenceInitialData::iLocationID);
            e.call(
                SAVE_GAME_WARNING,
                &args![
                    WARNING_NO_WORLD_OR_CELL,
                    location_id,
                    reference_name,
                    reference_id
                ],
            );
        }

        let cell_form = e.mem.u32(cell);
        if e.vcall(reference, 0x1d0, &args![]).u32() != 0
            && (cell_form == 0 || !is_cell_loaded(e, cell_form))
        {
            e.vcall(reference, 0x1cc, &args![0u32, 0u32]);
        } else if e.vcall(reference, 0x1d0, &args![]).u32() != 0
            && e.vcall(reference, 0x100, &args![]).bool()
        {
            e.vcall(reference, 0x1c4, &args![]);
        }
    }

    if e.vcall(reference, 0x100, &args![]).bool() {
        let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
        e.call(GAME_CANCEL_INIT_PACKAGE_LOCATIONS, &args![game, reference]);
        if data_type == 6 && !e.vcall(reference, 0x290, &args![]).bool() {
            let original_id = e.get(data, ReferenceInitialData::iOriginalLocationID);
            let original = e.call(LOOKUP_FORM, &args![original_id]).u32();
            fn_0084f8c0(e, Ptr::new(reference), original);
        }
    }
}

// Translated from 0084f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `bRefType` of the kind's
/// `InitialDataTypeData` entry in AL; the upper bytes of EAX keep those of
/// `kind * 3`, as the code leaves them.
pub fn fn_0084f840(e: &mut Engine, data_type: u32) -> u32 {
    let offset = data_type.wrapping_mul(3);
    let value = e
        .mem
        .u8(INITIAL_DATA_TYPE_DATA.wrapping_add(1).wrapping_add(offset));
    (offset & 0xffff_ff00) | value as u32
}

// Translated from 0084f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `bCellType` of the kind's
/// `InitialDataTypeData` entry in AL; the upper bytes of EAX keep those of
/// `kind * 3`.
pub fn fn_0084f860(e: &mut Engine, data_type: u32) -> u32 {
    let offset = data_type.wrapping_mul(3);
    let value = e
        .mem
        .u8(INITIAL_DATA_TYPE_DATA.wrapping_add(2).wrapping_add(offset));
    (offset & 0xffff_ff00) | value as u32
}

// Translated from 0084f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets (`flag` true) or clears bit 0x10 of the
/// word at +0x28 (on the load buffer, as `LoadInitialData` calls it).
pub fn fn_0084f880(e: &mut Engine, this: Ptr, flag: bool) {
    let address = this.addr() + 0x28;
    let value = e.mem.u32(address);
    let value = if flag { value | 0x10 } else { value & !0x10 };
    e.mem.set_u32(address, value);
}

// Translated from 0084f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `value` in the word at +0x170 (on a
/// reference, the original location `LoadInitialData` looks up).
pub fn fn_0084f8c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x170, value);
}

// Translated from 0084f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadInitialData::CreateReference` (Xbox PDB, cdecl): the
/// reference `form_id` of a created reference, made from its initial data.
///
/// `existing` (or the form looked up by `form_id`) is kept when slot 0xf0
/// answers true and its base object is the bound object (or, for slot 0x100,
/// its levelled creature's original base is); otherwise it is cleared
/// (`BGSSaveLoadGame::ClearForm`). A bound id the data handler reports is
/// dropped first; without a bound object nothing is created (warning, 0).
/// Without a kept reference, one is created by type: 0x3a, 0x3b for form
/// type 0x2a, 0x3c for 0x2b, the bound object's own (`004fe1d0`) for 0x33,
/// 0x16 with created-reference flag 4; for form type 0x51 an `Explosion`
/// (0x104 bytes) is constructed instead. With neither flag 1 nor 2 nothing
/// is created (warning). The new reference gets the bound object and the
/// form id (slot 0x128 with 1). Then flag 2 makes it persist, and any result
/// gets `0046a010(1)`.
// Translation incomplete: 0084f8e0 - C++ exception unwinding (the FS:[0] frame around the Explosion constructor) is not translated.
pub fn bgs_save_load_initial_data_create_reference(
    e: &mut Engine,
    form_id: u32,
    data: Ptr<ReferenceInitialData>,
    existing: Ptr,
) -> Ptr {
    let mut reference = existing.addr();
    if reference == 0 {
        reference = e.call(LOOKUP_FORM, &args![form_id]).u32();
    }
    let data_handler: u32 = e.global(DATA_HANDLER_POINTER);
    let bound_id = e.get(data, ReferenceInitialData::iBoundID);
    if e.call(DATA_HANDLER_ID_TEST, &args![data_handler, bound_id])
        .bool()
    {
        e.set(data, ReferenceInitialData::iBoundID, 0);
    }
    let bound_id = e.get(data, ReferenceInitialData::iBoundID);
    let bound_form = e.call(LOOKUP_FORM, &args![bound_id]).u32();
    let bound = e
        .call(
            DYNAMIC_CAST,
            &args![bound_form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .u32();
    if bound == 0 {
        let bound_id = e.get(data, ReferenceInitialData::iBoundID);
        e.call(
            SAVE_GAME_WARNING,
            &args![WARNING_NO_BOUND_OBJECT, bound_id, form_id],
        );
        return Ptr::NULL;
    }

    if reference != 0 {
        let clear = if !e.vcall(reference, 0xf0, &args![]).bool() {
            true
        } else {
            let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
            if base == bound {
                false
            } else if e.vcall(reference, 0x100, &args![]).bool() {
                let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                let original = e.call(EXTRA_GET_LEV_CREA_ORIGINAL_BASE, &args![list]).u32();
                original != bound
            } else {
                true
            }
        };
        if clear {
            let game: u32 = e.global(SAVE_LOAD_GAME_POINTER);
            e.call(GAME_CLEAR_FORM, &args![game, reference]);
            reference = 0;
        }
    }

    let flags = e.get(data, ReferenceInitialData::cCreatedReferenceFlags);
    if reference == 0 {
        let mut created = 0u32;
        let mut reference_type = 0x3au32;
        match e.call(FORM_TYPE, &args![bound]).u32() {
            0x2a => reference_type = 0x3b,
            0x2b => reference_type = 0x3c,
            0x33 => {
                reference_type = e.call(BOUND_OBJECT_REFERENCE_TYPE, &args![bound]).u32();
            }
            0x51 => {
                let memory = e.call(ALLOCATE, &args![0x104u32]).u32();
                created = if memory != 0 {
                    e.call(EXPLOSION_CONSTRUCT, &args![memory]).u32()
                } else {
                    0
                };
            }
            _ => {}
        }
        if flags & 4 != 0 {
            reference_type = 0x16;
        }
        if flags & 3 != 0 {
            if created == 0 {
                created = e
                    .call(REFERENCE_CREATE, &args![reference_type as u8 as u32, 1u32])
                    .u32();
            }
            e.call(REFERENCE_SET_OBJECT, &args![created, bound]);
            e.vcall(created, 0x128, &args![form_id, 1u32]);
            reference = created;
        } else {
            let bound_id = e.get(data, ReferenceInitialData::iBoundID);
            let location_id = e.get(data, ReferenceInitialData::iLocationID);
            e.call(
                SAVE_GAME_WARNING,
                &args![
                    WARNING_INVALID_CREATED_TYPE,
                    flags as u32,
                    form_id,
                    bound_id,
                    location_id
                ],
            );
        }
    }
    if reference != 0 && flags & 2 != 0 {
        e.call(REFERENCE_SET_PERSISTS, &args![reference, 1u32]);
    }
    if reference != 0 {
        e.call(FORM_SET_DELETED, &args![reference, 1u32]);
    }
    Ptr::new(reference)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084e340, fn_0084e340(Ptr<CellInitialData>, Ptr)),
        entry!(0x0084e3c0, fn_0084e3c0(Ptr<CellInitialData>, Ptr)),
        entry!(0x0084e420, fn_0084e420(Ptr<CellInitialData>, Ptr)),
        entry!(0x0084e460, fn_0084e460(Ptr<ReferenceInitialData>, Ptr)),
        entry!(0x0084e4f0, fn_0084e4f0(Ptr<ReferenceInitialData>, Ptr)),
        entry!(0x0084e580, fn_0084e580(Ptr<ReferenceInitialData>, Ptr)),
        entry!(
            0x0084e610,
            reference_initial_data_get_location_cell_and_world(
                Ptr<ReferenceInitialData>,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x0084e6a0,
            reference_initial_data_get_original_location_cell_and_world(
                Ptr<ReferenceInitialData>,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(0x0084e700, bgs_save_load_initial_data_get_initial_data_type(Ptr, u32) -> u32),
        entry!(0x0084e730, fn_0084e730(u32, u32, u32) -> u32),
        entry!(0x0084e800, bgs_save_load_initial_data_get_initial_data_type_data_size(u32) -> u32),
        entry!(0x0084e830, fn_0084e830(u32) -> u32),
        entry!(0x0084e850, bgs_save_load_initial_data_load_initial_data_struct(Ptr, u32, Ptr) -> u32),
        entry!(0x0084e8c0, bgs_save_load_initial_data_load_initial_data_struct_ov2(Ptr, u32, Ptr) -> u32),
        entry!(0x0084e930, bgs_save_load_initial_data_load_initial_data_struct_ov3(Ptr, u32, Ptr) -> u32),
        entry!(0x0084eac0, fn_0084eac0(Ptr) -> Ptr),
        entry!(0x0084eaf0, construct_compact_id(Ptr) -> Ptr),
        entry!(0x0084eb20, fn_0084eb20(Ptr) -> Ptr),
        entry!(0x0084eb50, fn_0084eb50(Ptr) -> Ptr),
        entry!(0x0084eb80, bgs_save_load_initial_data_save_initial_data(Ptr, Ptr, u32) -> u32),
        entry!(0x0084f330, bgs_save_load_initial_data_load_initial_data(Ptr, Ptr, u32) -> u32),
        entry!(0x0084f840, fn_0084f840(u32) -> u32),
        entry!(0x0084f860, fn_0084f860(u32) -> u32),
        entry!(0x0084f880, fn_0084f880(Ptr, bool)),
        entry!(0x0084f8c0, fn_0084f8c0(Ptr, u32)),
        entry!(
            0x0084f8e0,
            bgs_save_load_initial_data_create_reference(u32, Ptr<ReferenceInitialData>, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Captured = Rc<RefCell<Vec<Vec<u32>>>>;
    type CapturedBytes = Rc<RefCell<Vec<Vec<u8>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the globals' pages, the exe's `InitialDataTypeData`
    /// for kinds 0 to 6, a call log and doubles with the callees' plain
    /// semantics: the +0xc getter, the flags test, `memcpy`, the compact
    /// id getter and setter (three big-endian bytes), the `NiPoint3`
    /// constructor, the type byte, a reference-type test (0x3b only) and a
    /// data-handler test (ids `0xff......`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_2000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_d000, 0x2000);
        let table: [u8; 21] = [
            0, 0, 0, 8, 0, 1, 10, 0, 1, 4, 0, 1, 27, 1, 0, 31, 1, 0, 34, 1, 0,
        ];
        e.mem.write(INITIAL_DATA_TYPE_DATA, &table);
        e.register(WORD_AT_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FLAGS_TEST, |e, a| ret((e.mem.u32(a[0]) & a[1] != 0) as u32));
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(GET_NUMERIC_ID, |e, a| {
            let b = e.mem.bytes(a[0], 3);
            ret((b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32)
        });
        e.register(SET_NUMERIC_ID, |e, a| {
            e.mem
                .write(a[0], &[(a[1] >> 16) as u8, (a[1] >> 8) as u8, a[1] as u8]);
            ret(0)
        });
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(IS_REFERENCE_TYPE, |_, a| ret((a[0] == 0x3b) as u32));
        e.register(DATA_HANDLER_ID_TEST, |_, a| {
            ret((a[1] & 0xff00_0000 == 0xff00_0000) as u32)
        });
        e.call_log = Some(vec![]);
        e
    }

    fn stub(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| ret(eax));
    }

    /// Records every call's words and answers `eax`.
    fn capture(e: &mut Engine, address: u32, eax: u32) -> Captured {
        let seen: Captured = Rc::default();
        let log = seen.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(eax)
        });
        seen
    }

    /// A form of `form_type` with id `id` whose vtable slots answer the
    /// given values (slot targets `0x7000_0000 + tag * 0x1000 + slot`).
    fn form(e: &mut Engine, tag: u32, form_type: u8, id: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x200);
        let vtable = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        e.mem.set_u8(object + 4, form_type);
        e.mem.set_u32(object + 0xc, id);
        for &(offset, eax) in slots {
            let target = slot(tag, offset);
            e.mem.set_u32(vtable + offset, target);
            stub(e, target, eax);
        }
        object
    }

    fn slot(tag: u32, offset: u32) -> u32 {
        0x7000_0000 + tag * 0x1000 + offset
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The order in which the calls to `addresses` were made.
    fn order(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    /// The save/load game global with `ids` at +0xc.
    fn game(e: &mut Engine, ids: u32) -> u32 {
        let game = e.mem.alloc(0x40);
        e.mem.set_u32(game + 0xc, ids);
        e.set_global(SAVE_LOAD_GAME_POINTER, game);
        game
    }

    #[test]
    fn exterior_char_record_fills_cell_initial_data() {
        let mut e = engine();
        game(&mut e, 0x9000);
        stub(&mut e, FORM_ID_OF_INDEX, 0x0001_2345);
        let source = e.mem.alloc(8);
        e.mem
            .write(source, &[5, 0, 0xfd, 4, 0x44, 0x33, 0x22, 0x11]);
        let out: Ptr<CellInitialData> = e.new_object();
        e.call(0x0084_e340, &args![out, source]);
        assert_eq!(e.get(out, CellInitialData::iCellX), -3);
        assert_eq!(e.get(out, CellInitialData::iCellY), 4);
        assert_eq!(e.get(out, CellInitialData::iDetachTime), 0x1122_3344);
        assert_eq!(e.get(out, CellInitialData::iWorldSpaceID), 0x0001_2345);
        assert_eq!(calls(&e, FORM_ID_OF_INDEX), [vec![0x9000, 5]]);
    }

    #[test]
    fn exterior_short_record_fills_cell_initial_data() {
        let mut e = engine();
        game(&mut e, 0x9000);
        stub(&mut e, FORM_ID_OF_INDEX, 0x0001_2345);
        let source = e.mem.alloc(10);
        e.mem
            .write(source, &[7, 1, 0x00, 0x80, 0x10, 0, 0x44, 0x33, 0x22, 0x11]);
        let out: Ptr<CellInitialData> = e.new_object();
        e.call(0x0084_e3c0, &args![out, source]);
        assert_eq!(e.get(out, CellInitialData::iCellX), -32768);
        assert_eq!(e.get(out, CellInitialData::iCellY), 16);
        assert_eq!(e.get(out, CellInitialData::iDetachTime), 0x1122_3344);
        assert_eq!(e.get(out, CellInitialData::iWorldSpaceID), 0x0001_2345);
        assert_eq!(calls(&e, FORM_ID_OF_INDEX), [vec![0x9000, 0x107]]);
    }

    #[test]
    fn interior_record_keeps_only_the_detach_time() {
        let mut e = engine();
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 77);
        let out: Ptr<CellInitialData> = e.new_object();
        e.mem.write(out.addr(), &[0xff; 16]);
        e.call(0x0084_e420, &args![out, source]);
        assert_eq!(
            e.mem.bytes(out.addr(), 16),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0]
        );
    }

    /// A packed location record: compact id, location, angle, then `rest`.
    fn location_record(e: &mut Engine, id: [u8; 3], rest: &[u8]) -> u32 {
        let record = e.mem.alloc(0x24);
        e.mem.write(record, &id);
        for i in 0..6u32 {
            e.mem.set_f32(record + 3 + 4 * i, i as f32 + 1.0);
        }
        e.mem.write(record + 0x1b, rest);
        record
    }

    fn assert_location_and_angle(e: &Engine, out: u32) {
        for i in 0..6u32 {
            assert_eq!(e.mem.f32(out + 0x14 + 4 * i), i as f32 + 1.0);
        }
    }

    #[test]
    fn location_record_fills_reference_initial_data() {
        let mut e = engine();
        let source = location_record(&mut e, [0, 0x12, 0x34], &[]);
        let out: Ptr<ReferenceInitialData> = e.new_object();
        e.mem.write(out.addr(), &[0xff; 0x2d]);
        e.call(0x0084_e460, &args![out, source]);
        assert_eq!(e.get(out, ReferenceInitialData::iLocationID), 0x1234);
        assert_eq!(e.get(out, ReferenceInitialData::iBoundID), 0);
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalLocationID), 0);
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalCellX), 0);
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalCellY), 0);
        assert_location_and_angle(&e, out.addr());
        assert_eq!(e.get(out, ReferenceInitialData::cCreatedReferenceFlags), 0);
    }

    #[test]
    fn created_record_fills_bound_id_and_flags() {
        let mut e = engine();
        let source = location_record(&mut e, [0, 0, 1], &[6, 0, 0x22, 0x33]);
        let out: Ptr<ReferenceInitialData> = e.new_object();
        e.call(0x0084_e4f0, &args![out, source]);
        assert_eq!(e.get(out, ReferenceInitialData::iLocationID), 1);
        assert_eq!(e.get(out, ReferenceInitialData::iBoundID), 0x2233);
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalLocationID), 0);
        assert_location_and_angle(&e, out.addr());
        assert_eq!(e.get(out, ReferenceInitialData::cCreatedReferenceFlags), 6);
    }

    #[test]
    fn moved_record_fills_original_location_and_grid() {
        let mut e = engine();
        let source = location_record(&mut e, [0, 0, 1], &[0, 0x44, 0x55, 0xfe, 0xff, 3, 0]);
        let out: Ptr<ReferenceInitialData> = e.new_object();
        e.mem.write(out.addr(), &[0xff; 0x2d]);
        e.call(0x0084_e580, &args![out, source]);
        assert_eq!(e.get(out, ReferenceInitialData::iBoundID), 0);
        assert_eq!(
            e.get(out, ReferenceInitialData::iOriginalLocationID),
            0x4455
        );
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalCellX), -2);
        assert_eq!(e.get(out, ReferenceInitialData::iOriginalCellY), 3);
        assert_location_and_angle(&e, out.addr());
        assert_eq!(e.get(out, ReferenceInitialData::cCreatedReferenceFlags), 0);
    }

    /// Doubles for the location lookup: `GetLocationCellOrWorld` writes
    /// `world` and answers true; the cell of a grid is `0x7000 + x * 16 + y`.
    fn location_doubles(e: &mut Engine, world: u32) {
        stub(e, LOOKUP_FORM, 0x500);
        e.register_double(GET_LOCATION_CELL_OR_WORLD, move |e, a| {
            e.mem.set_u32(a[2], world);
            ret(1)
        });
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        e.register(WORLD_GET_CELL_FROM_CELL_COORD, |_, a| {
            ret(0x7000u32.wrapping_add((a[1] as i32 * 16 + a[2] as i32) as u32))
        });
    }

    #[test]
    fn location_cell_and_world_uses_the_grid_of_the_location() {
        let mut e = engine();
        location_doubles(&mut e, 0x600);
        let data: Ptr<ReferenceInitialData> = e.new_object();
        e.mem.set_f32(data.addr() + 0x14, 8192.0);
        e.mem.set_f32(data.addr() + 0x18, -4097.0);
        let (cell, world) = (e.mem.alloc(4), e.mem.alloc(4));
        assert!(e.call(0x0084_e610, &args![data, cell, world]).bool());
        assert_eq!(e.mem.u32(cell), 0x7000 + 2 * 16 - 2);
        assert_eq!(
            calls(&e, WORLD_GET_CELL_FROM_CELL_COORD),
            [vec![0x600, 2, (-2i32) as u32]]
        );

        // A signalling NaN reaches the conversion quiet.
        e.mem.set_u32(data.addr() + 0x14, 0x7f80_0001);
        e.call(0x0084_e610, &args![data, cell, world]);
        assert_eq!(calls(&e, FLOAT_TO_INT)[2], [0x7fc0_0001]);

        // Without a world the cell stays what the lookup left.
        let mut e = engine();
        location_doubles(&mut e, 0);
        let data: Ptr<ReferenceInitialData> = e.new_object();
        let (cell, world) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(cell, 0x55);
        assert!(e.call(0x0084_e610, &args![data, cell, world]).bool());
        assert_eq!(e.mem.u32(cell), 0x55);
        assert!(calls(&e, WORLD_GET_CELL_FROM_CELL_COORD).is_empty());
    }

    #[test]
    fn original_location_uses_the_original_grid() {
        let mut e = engine();
        location_doubles(&mut e, 0x600);
        let data: Ptr<ReferenceInitialData> = e.new_object();
        e.set(data, ReferenceInitialData::iOriginalLocationID, 0x42);
        e.set(data, ReferenceInitialData::iOriginalCellX, 3);
        e.set(data, ReferenceInitialData::iOriginalCellY, -1);
        let (cell, world) = (e.mem.alloc(4), e.mem.alloc(4));
        assert!(e.call(0x0084_e6a0, &args![data, cell, world]).bool());
        assert_eq!(calls(&e, LOOKUP_FORM), [vec![0x42]]);
        assert_eq!(e.mem.u32(cell), 0x7000 + 3 * 16 - 1);
    }

    #[test]
    fn initial_data_type_passes_id_type_and_flags() {
        let mut e = engine();
        let cell = form(&mut e, 1, 0x39, 0x1234, &[]);
        assert_eq!(e.call(0x0084_e700, &args![cell, 0x6000_0000u32]).u32(), 1);
        assert_eq!(calls(&e, IS_REFERENCE_TYPE), [vec![0x39]]);
        assert_eq!(calls(&e, WORD_AT_0C), [vec![cell]]);
        let reference = form(&mut e, 2, 0x3b, 0xff00_0001, &[]);
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        assert_eq!(e.call(0x0084_e700, &args![reference, 0u32]).u32(), 5);
    }

    #[test]
    fn initial_data_kind_by_type_and_flags() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        let kind = |e: &mut Engine, id: u32, form_type: u32, flags: u32| {
            e.call(0x0084_e730, &args![id, form_type, flags]).u32()
        };
        assert_eq!(kind(&mut e, 0xff00_0001, 0x3b, 0), 5);
        assert_eq!(
            calls(&e, DATA_HANDLER_ID_TEST),
            [vec![0x0abc_0000, 0xff00_0001]]
        );
        assert_eq!(kind(&mut e, 1, 0x3b, 8 | 2), 6);
        assert_eq!(kind(&mut e, 1, 0x3b, 4), 4);
        assert_eq!(kind(&mut e, 1, 0x3b, 2), 4);
        assert_eq!(kind(&mut e, 1, 0x3b, 1), 0);
        assert_eq!(kind(&mut e, 1, 0x39, 0x7000_0000), 1);
        assert_eq!(kind(&mut e, 1, 0x39, 0x5000_0000), 2);
        assert_eq!(kind(&mut e, 1, 0x39, 0x4000_0000), 3);
        assert_eq!(kind(&mut e, 1, 0x39, 0x3000_0000), 0);
        assert_eq!(kind(&mut e, 1, 0x38, 0x7000_0000), 0);
    }

    #[test]
    fn data_size_is_the_table_size_plus_one() {
        let mut e = engine();
        assert_eq!(e.call(0x0084_e800, &args![5u32]).u32(), 32);
        assert_eq!(e.call(0x0084_e800, &args![0u32]).u32(), 1);
    }

    #[test]
    fn table_size_of_a_kind() {
        let mut e = engine();
        assert_eq!(e.call(0x0084_e830, &args![6u32]).u32(), 34);
        assert_eq!(e.call(0x0084_e830, &args![2u32]).u32(), 10);
        assert_eq!(e.call(0x0084_e830, &args![0u32]).u32(), 0);
    }

    #[test]
    fn load_struct_from_the_file_reads_size_plus_one() {
        let mut e = engine();
        let out: Ptr<CellInitialData> = e.new_object();
        assert_eq!(e.call(0x0084_e850, &args![0x1234u32, 0u32, out]).u32(), 0);
        assert!(calls(&e, FILE_READ).is_empty());

        e.register(FILE_READ, |e, a| {
            e.mem.write(a[1], &[0x44, 0x55, 0x66, 0x77, 0x03]);
            ret(1)
        });
        assert_eq!(e.call(0x0084_e850, &args![0x1234u32, 3u32, out]).u32(), 5);
        assert_eq!(calls(&e, FILE_READ)[0][0], 0x1234);
        assert_eq!(calls(&e, FILE_READ)[0][2], 5);
        assert_eq!(calls(&e, MEMCPY)[0][2], 4);
        assert_eq!(e.get(out, CellInitialData::iDetachTime), 0x7766_5544);
    }

    #[test]
    fn load_struct_from_a_buffer_reads_the_size() {
        let mut e = engine();
        let out: Ptr<CellInitialData> = e.new_object();
        assert_eq!(e.call(0x0084_e8c0, &args![0x1234u32, 0u32, out]).u32(), 0);
        e.register(BUFFER_READ, |e, a| {
            e.mem.write(a[1], &[0x44, 0x55, 0x66, 0x77]);
            ret(1)
        });
        assert_eq!(e.call(0x0084_e8c0, &args![0x1234u32, 3u32, out]).u32(), 5);
        assert_eq!(calls(&e, BUFFER_READ)[0][2], 4);
        assert_eq!(e.get(out, CellInitialData::iDetachTime), 0x7766_5544);
    }

    #[test]
    fn load_struct_converts_each_kind() {
        let mut e = engine();
        let source = location_record(&mut e, [0, 0, 9], &[5, 0, 0, 0x21]);
        let out: Ptr<ReferenceInitialData> = e.new_object();
        assert_eq!(e.call(0x0084_e930, &args![source, 5u32, out]).u32(), 32);
        assert_eq!(calls(&e, MEMCPY)[0][1..], [source, 31]);
        assert_eq!(e.get(out, ReferenceInitialData::iLocationID), 9);
        assert_eq!(e.get(out, ReferenceInitialData::iBoundID), 0x21);
        assert_eq!(e.get(out, ReferenceInitialData::cCreatedReferenceFlags), 5);
        assert_location_and_angle(&e, out.addr());
        // The record is constructed first (two NiPoint3s).
        assert_eq!(calls(&e, POINT_CONSTRUCT).len(), 2);

        // An unknown kind copies nothing and leaves `out` alone.
        let before = e.mem.bytes(out.addr(), 0x2d);
        assert_eq!(e.call(0x0084_e930, &args![source, 9u32, out]).u32(), 1);
        assert_eq!(calls(&e, MEMCPY).len(), 1);
        assert_eq!(e.mem.bytes(out.addr(), 0x2d), before);
    }

    #[test]
    fn location_record_constructor_clears_the_id() {
        let mut e = engine();
        let record = e.mem.alloc(0x24);
        e.mem.write(record, &[0xff; 0x24]);
        assert_eq!(e.call(0x0084_eac0, &args![record]).u32(), record);
        assert_eq!(e.mem.bytes(record, 4), [0, 0, 0, 0xff]);
        assert_eq!(
            calls(&e, POINT_CONSTRUCT),
            [vec![record + 3], vec![record + 0xf]]
        );
    }

    #[test]
    fn created_record_constructor_clears_the_bound_id() {
        let mut e = engine();
        let record = e.mem.alloc(0x24);
        e.mem.write(record, &[0xff; 0x24]);
        assert_eq!(e.call(0x0084_eb20, &args![record]).u32(), record);
        assert_eq!(e.mem.bytes(record + 0x1b, 5), [0xff, 0, 0, 0, 0xff]);
        assert_eq!(e.mem.bytes(record, 3), [0, 0, 0]);
    }

    #[test]
    fn moved_record_constructor_clears_the_original_id() {
        let mut e = engine();
        let record = e.mem.alloc(0x24);
        e.mem.write(record, &[0xff; 0x24]);
        assert_eq!(e.call(0x0084_eb50, &args![record]).u32(), record);
        assert_eq!(e.mem.bytes(record + 0x1a, 5), [0xff, 0, 0, 0, 0xff]);
        assert_eq!(e.mem.bytes(record, 3), [0, 0, 0]);
    }

    /// Records the bytes each buffer write passes.
    fn capture_writes(e: &mut Engine) -> CapturedBytes {
        let seen: CapturedBytes = Rc::default();
        let log = seen.clone();
        e.register_double(BUFFER_WRITE, move |e, a| {
            assert_eq!(a[3], 0);
            log.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            ret(0)
        });
        seen
    }

    #[test]
    fn save_cells_write_index_grid_and_detach_time() {
        let mut e = engine();
        game(&mut e, 0x9000);
        let world = form(&mut e, 2, 0x41, 0x3c00, &[]);
        let cell = form(&mut e, 1, 0x39, 0x1234, &[(0xf0, 0)]);
        stub(&mut e, CELL_GET_WORLD_SPACE, world);
        stub(&mut e, INDEX_OF_FORM_ID, 7);
        stub(&mut e, CELL_GET_DATA_X, 0xffff_fffe);
        stub(&mut e, CELL_GET_DATA_Y, 3);
        stub(&mut e, CELL_GET_DETACH_TIME, 0x0102_0304);
        let writes = capture_writes(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, cell, 0x6000_0000u32])
                .u32(),
            1
        );
        assert_eq!(calls(&e, INDEX_OF_FORM_ID), [vec![0x9000, 0x3c00]]);
        assert_eq!(writes.borrow()[0], [7, 0, 0xfe, 3, 4, 3, 2, 1]);

        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, cell, 0x5000_0000u32])
                .u32(),
            2
        );
        assert_eq!(writes.borrow()[1], [7, 0, 0xfe, 0xff, 3, 0, 4, 3, 2, 1]);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, cell, 0x4000_0000u32])
                .u32(),
            3
        );
        assert_eq!(writes.borrow()[2], [4, 3, 2, 1]);
    }

    /// Location and angle doubles for a reference.
    fn place(e: &mut Engine) {
        let location = point(e, [1.0, 2.0, 3.0]);
        let angle = point(e, [4.0, 5.0, 6.0]);
        stub(e, REFERENCE_LOCATION, location);
        stub(e, REFERENCE_ANGLE, angle);
    }

    fn written_points(bytes: &[u8]) -> Vec<f32> {
        (0..6)
            .map(|i| f32::from_le_bytes(bytes[3 + 4 * i..7 + 4 * i].try_into().unwrap()))
            .collect()
    }

    #[test]
    fn save_reference_without_a_world_falls_back_to_the_player() {
        let mut e = engine();
        let world = form(&mut e, 2, 0x41, 0x00_3c01, &[]);
        let player = form(&mut e, 3, 0x3b, 0x14, &[]);
        e.set_global(PLAYER_CHARACTER, player);
        let reference = form(&mut e, 1, 0x3b, 0x5000, &[(0x138, 0), (0xf0, 0)]);
        e.register_double(REFERENCE_GET_WORLD_SPACE, move |_, a| {
            ret(if a[0] == player { world } else { 0 })
        });
        stub(&mut e, REFERENCE_PARENT_CELL, 0);
        place(&mut e);
        let writes = capture_writes(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, reference, 4u32]).u32(),
            4
        );
        assert_eq!(
            calls(&e, REFERENCE_GET_WORLD_SPACE),
            [vec![reference], vec![player]]
        );
        assert_eq!(calls(&e, REFERENCE_PARENT_CELL), [vec![reference]]);
        assert_eq!(calls(&e, slot(1, 0x138))[0][4], 0);
        assert_eq!(calls(&e, REFERENCE_LOCATION).len(), 2);
        let bytes = writes.borrow()[0].clone();
        assert_eq!(bytes.len(), 0x1b);
        assert_eq!(bytes[..3], [0, 0x3c, 0x01]);
        assert_eq!(written_points(&bytes), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn save_created_reference_writes_base_and_flags() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        let world = form(&mut e, 2, 0x41, 0x00_0102, &[]);
        let base = form(&mut e, 4, 0x2a, 0x00_0a0b, &[]);
        let original = form(&mut e, 5, 0x2b, 0x00_0c0d, &[]);
        let location = point(&mut e, [7.0, 8.0, 9.0]);
        let reference = form(
            &mut e,
            1,
            0x3b,
            0xff00_0001,
            &[(0x1f4, location), (0x100, 1), (0xfc, 1), (0xf0, 0)],
        );
        stub(&mut e, REFERENCE_GET_WORLD_SPACE, world);
        place(&mut e);
        stub(&mut e, REFERENCE_BASE_OBJECT, base);
        stub(&mut e, REFERENCE_EXTRA_LIST, 0x0bad_0000);
        stub(&mut e, EXTRA_GET_LEV_CREA_ORIGINAL_BASE, original);
        stub(&mut e, REFERENCE_GET_PERSISTS, 1);
        stub(&mut e, REFERENCE_FLAG_TEST, 1);
        let writes = capture_writes(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, reference, 0x40000u32])
                .u32(),
            5
        );
        let bytes = writes.borrow()[0].clone();
        assert_eq!(bytes.len(), 0x1f);
        assert_eq!(bytes[..3], [0, 1, 2]);
        assert_eq!(written_points(&bytes), [7.0, 8.0, 9.0, 4.0, 5.0, 6.0]);
        assert_eq!(bytes[0x1b..], [7, 0, 0x0c, 0x0d]);

        // Without change flag 0x40000 the base stays; nothing persists.
        stub(&mut e, REFERENCE_GET_PERSISTS, 0);
        stub(&mut e, REFERENCE_FLAG_TEST, 0);
        e.call(0x0084_eb80, &args![buffer, reference, 0u32]);
        assert_eq!(writes.borrow()[1][0x1b..], [1, 0, 0x0a, 0x0b]);
    }

    #[test]
    fn save_moved_reference_writes_the_editor_location() {
        let mut e = engine();
        let world = form(&mut e, 2, 0x41, 0x00_0102, &[]);
        let editor_world = form(&mut e, 3, 0x41, 0x00_0304, &[]);
        let reference = form(&mut e, 1, 0x3b, 0x5000, &[(0xf0, 0)]);
        let vtable = e.mem.u32(reference);
        e.mem.set_u32(vtable + 0x138, slot(1, 0x138));
        e.register_double(slot(1, 0x138), move |e, a| {
            e.mem.set_f32(a[1], 8192.0);
            e.mem.set_f32(a[1] + 4, -1.0);
            e.mem.set_u32(a[3], editor_world);
            ret(1)
        });
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        stub(&mut e, REFERENCE_GET_WORLD_SPACE, world);
        place(&mut e);
        let writes = capture_writes(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, reference, 8u32]).u32(),
            6
        );
        let bytes = writes.borrow()[0].clone();
        assert_eq!(bytes.len(), 0x22);
        assert_eq!(bytes[..3], [0, 1, 2]);
        assert_eq!(bytes[0x1b..], [0, 3, 4, 2, 0, 0xff, 0xff]);
        assert_eq!(calls(&e, slot(1, 0x138)).len(), 1);
    }

    #[test]
    fn save_moved_reference_without_editor_location_warns() {
        let mut e = engine();
        let world = form(&mut e, 2, 0x41, 0x00_0102, &[]);
        let base = form(&mut e, 4, 0x2a, 0x0a0b, &[(0x130, 0x5555)]);
        let reference = form(
            &mut e,
            1,
            0x3b,
            0x5000,
            &[(0x138, 0), (0x130, 0x4444), (0xf0, 0)],
        );
        stub(&mut e, REFERENCE_GET_WORLD_SPACE, world);
        stub(&mut e, REFERENCE_BASE_OBJECT, base);
        place(&mut e);
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        let writes = capture_writes(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, reference, 8u32]).u32(),
            6
        );
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_NO_EDITOR_LOCATION, 0x4444, 0x5000, 0x5555, 0x0a0b]
        );
        assert_eq!(writes.borrow()[0][0x1b..], [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn save_havok_data_as_sub_buffer_or_variable_sized_value() {
        let mut e = engine();
        let reference = form(&mut e, 1, 0x10, 0x5000, &[(0xf0, 1), (0x1d0, 1)]);
        stub(&mut e, REFERENCE_EXTRA_LIST, 0x0bad_0000);
        stub(&mut e, EXTRA_GET_SAVED_HAVOK_DATA, 0x77);
        stub(&mut e, CHANGE_FLAGS_CONSTRUCT, 0);
        stub(&mut e, BUFFER_START_VARIABLE_SIZED_VALUE, 0x99);
        e.register(REFERENCE_SAVE_HAVOK_DATA, |e, a| {
            e.mem.set_u32(a[1] + 0xc, 25);
            ret(0)
        });
        let saved = capture(&mut e, BUFFER_SAVE_VARIABLE_SIZED_VALUE, 0);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(buffer + 0xc, 10);
        assert_eq!(
            e.call(0x0084_eb80, &args![buffer, reference, 4u32]).u32(),
            0
        );
        assert_eq!(saved.borrow()[0], [buffer, 15, 0x99]);
        assert_eq!(calls(&e, CHANGE_FLAGS_CONSTRUCT)[0][1], 0x77);

        // Slot 0x1d0 answering 0 and a non-zero flags word: the sub buffer.
        let reference = form(&mut e, 2, 0x10, 0x5001, &[(0xf0, 1), (0x1d0, 0)]);
        stub(&mut e, CHANGE_FLAGS_VALUE, 1);
        let sub = capture(&mut e, SUB_BUFFER_SAVE_GAME, 0);
        e.call(0x0084_eb80, &args![buffer, reference, 4u32]);
        assert_eq!(sub.borrow()[0][1], buffer);
        assert_eq!(saved.borrow().len(), 1);

        // Without change flag 4 nothing is saved.
        e.call(0x0084_eb80, &args![buffer, reference, 0u32]);
        assert_eq!(sub.borrow().len(), 1);
    }

    /// Doubles for `LoadInitialData`: the buffer read hands out `record`,
    /// a lookup answers `0x0100_0000 | id`, the location lookup answers
    /// `cell` and `world`.
    fn load_doubles(e: &mut Engine, record: Vec<u8>, cell: u32, world: u32) {
        e.register_double(BUFFER_READ, move |e, a| {
            e.mem.write(a[1], &record[..a[2] as usize]);
            ret(1)
        });
        e.register(REFERENCE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LOOKUP_FORM, |_, a| ret(0x0100_0000 | a[0]));
        e.register_double(GET_LOCATION_CELL_OR_WORLD, move |e, a| {
            e.mem.set_u32(a[1], cell);
            e.mem.set_u32(a[2], world);
            ret(1)
        });
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        stub(e, WORLD_GET_CELL_FROM_CELL_COORD, cell);
        let game = e.mem.alloc(0x40);
        e.set_global(SAVE_LOAD_GAME_POINTER, game);
    }

    fn reference_record(rest: &[u8]) -> Vec<u8> {
        let mut record = vec![0, 0, 0x31];
        for i in 0..6 {
            record.extend_from_slice(&(i as f32 + 1.0).to_le_bytes());
        }
        record.extend_from_slice(rest);
        record.resize(0x24, 0);
        record
    }

    #[test]
    fn load_cell_sets_the_detach_time() {
        let mut e = engine();
        game(&mut e, 0x9000);
        stub(&mut e, FORM_ID_OF_INDEX, 0x3c);
        let set = capture(&mut e, CELL_SET_DETACH_TIME, 0);
        e.register(BUFFER_READ, |e, a| {
            e.mem
                .write(a[1], &[1, 0, 2, 0, 3, 0, 0x44, 0x33, 0x22, 0x11]);
            ret(1)
        });
        let cell = form(&mut e, 1, 0x39, 0x1234, &[]);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, cell, 0x5000_0000u32])
                .u32(),
            2
        );
        assert_eq!(set.borrow()[0], [cell, 0x1122_3344, 1]);
    }

    #[test]
    fn load_unknown_kind_warns() {
        let mut e = engine();
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        let cell = form(&mut e, 1, 0x39, 0x1234, &[]);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0084_f330, &args![buffer, cell, 0u32]).u32(), 0);
        assert!(warnings.borrow().is_empty());
        // Kind 3 with `bCellType` cleared is neither a cell nor a reference kind.
        e.mem.set_u8(INITIAL_DATA_TYPE_DATA + 3 * 3 + 2, 0);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, cell, 0x4000_0000u32])
                .u32(),
            3
        );
        assert_eq!(warnings.borrow()[0], [WARNING_UNKNOWN_TYPE, 3]);
    }

    #[test]
    fn load_created_reference_without_cell_or_world_is_dropped() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        load_doubles(&mut e, reference_record(&[0, 0, 0, 0x41]), 0, 0);
        stub(&mut e, DYNAMIC_CAST, 0x0b0b_0000);
        stub(&mut e, BOUND_OBJECT_NAME, 0x0123_4567);
        let placed = capture(&mut e, DATA_HANDLER_PLACE_REFERENCE, 0);
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        stub(&mut e, FORM_SET_DELETED, 0);
        stub(&mut e, GAME_DELETE_FORM, 0);
        stub(&mut e, BUFFER_FOLDED_STEP, 0);
        let reference = form(&mut e, 1, 0x3b, 0xff00_0001, &[]);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 4u32]).u32(),
            5
        );
        let place = placed.borrow()[0].clone();
        assert_eq!(place[1], 0x0b0b_0000);
        assert_eq!(e.mem.f32(place[2]), 1.0);
        assert_eq!(e.mem.f32(place[3]), 4.0);
        assert_eq!(place[4..], [0, 0, reference, 0, 0]);
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_NON_EXISTENT_CELL, 0xff00_0001, 0x41, 0x0123_4567]
        );
        assert_eq!(calls(&e, FORM_SET_DELETED), [vec![reference, 0]]);
        assert_eq!(calls(&e, GAME_DELETE_FORM)[0][1], reference);
        assert_eq!(calls(&e, BUFFER_FOLDED_STEP), [vec![buffer, 0]]);
        assert!(calls(&e, GAME_QUEUE_SUB_BUFFER).is_empty());
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            [0x0100_0041, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]
        );
    }

    #[test]
    fn load_created_reference_with_flag_4_sets_location_and_cell() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        load_doubles(&mut e, reference_record(&[4, 0, 0, 0x41]), 0x0ce1_0000, 0);
        stub(&mut e, DYNAMIC_CAST, 0x0b0b_0000);
        stub(&mut e, REFERENCE_SET_LOCATION, 0);
        stub(&mut e, REFERENCE_SET_ANGLE, 0);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        stub(&mut e, GAME_CANCEL_INIT_PACKAGE_LOCATIONS, 0);
        stub(&mut e, GAME_QUEUE_SUB_BUFFER, 0);
        let reference = form(&mut e, 1, 0x3b, 0xff00_0001, &[(0x228, 0), (0x100, 1)]);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 4u32]).u32(),
            5
        );
        assert_eq!(calls(&e, slot(1, 0x228)), [vec![reference, 0x0ce1_0000]]);
        assert_eq!(
            calls(&e, REFERENCE_SET_ANGLE)[0][1..],
            [4.0f32.to_bits(), 5.0f32.to_bits(), 6.0f32.to_bits()]
        );
        assert_eq!(calls(&e, GAME_CANCEL_INIT_PACKAGE_LOCATIONS).len(), 1);
        assert_eq!(calls(&e, GAME_QUEUE_SUB_BUFFER)[0][1..], [2, buffer, 0]);
        assert_eq!(
            order(&e, &[REFERENCE_SET_LOCATION, REFERENCE_SET_ANGLE]),
            [REFERENCE_SET_LOCATION, REFERENCE_SET_ANGLE]
        );
        assert!(calls(&e, DATA_HANDLER_PLACE_REFERENCE).is_empty());
    }

    #[test]
    fn load_moved_reference_into_a_loaded_cell() {
        let mut e = engine();
        load_doubles(&mut e, reference_record(&[]), 0x0ce1_0000, 0);
        e.set_global(PLAYER_CHARACTER, 0x0f0f_0000u32);
        stub(&mut e, REFERENCE_SET_LOCATION, 0);
        stub(&mut e, REFERENCE_SET_ANGLE, 0);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        stub(&mut e, CELL_FLAG_ZERO_TEST, 1);
        stub(&mut e, REFERENCE_PARENT_CELL, 0x0ce2_0000);
        let moved = capture(&mut e, REFERENCE_MOVE_TO_NEW_SPACE, 0);
        stub(&mut e, GAME_CANCEL_INIT_PACKAGE_LOCATIONS, 0);
        stub(&mut e, GAME_QUEUE_SUB_BUFFER, 0);
        let reference = form(&mut e, 1, 0x3b, 0x5000, &[(0x1d0, 0), (0x100, 1)]);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 6u32]).u32(),
            4
        );
        assert_eq!(moved.borrow()[0], [reference, 0x0ce1_0000, 0]);
        assert_eq!(calls(&e, slot(1, 0x1d0)).len(), 2);
        assert_eq!(calls(&e, GAME_QUEUE_SUB_BUFFER).len(), 1);
        assert_eq!(
            order(&e, &[REFERENCE_SET_LOCATION, REFERENCE_SET_ANGLE]),
            [REFERENCE_SET_ANGLE, REFERENCE_SET_LOCATION]
        );
    }

    #[test]
    fn load_moved_player_skips_placement_when_the_game_says_so() {
        let mut e = engine();
        load_doubles(&mut e, reference_record(&[]), 0x0ce1_0000, 0);
        let reference = form(&mut e, 1, 0x3b, 0x14, &[(0x100, 0)]);
        e.set_global(PLAYER_CHARACTER, reference);
        stub(&mut e, GAME_PLAYER_TEST, 1);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 2u32]).u32(),
            4
        );
        assert!(calls(&e, REFERENCE_SET_ANGLE).is_empty());
        assert_eq!(calls(&e, REFERENCE_UPDATE_3D_POSITION).len(), 1);
    }

    #[test]
    fn load_moved_reference_into_unloaded_cell_warns_and_unloads() {
        let mut e = engine();
        let world = form(&mut e, 2, 0x41, 0x3c, &[(0x130, 0x6666)]);
        load_doubles(&mut e, reference_record(&[0, 0, 0x77]), 0x0ce1_0000, world);
        e.set_global(PLAYER_CHARACTER, 0x0f0f_0000u32);
        e.set_global(TES_POINTER, 0x0e5e_0000u32);
        stub(&mut e, REFERENCE_SET_LOCATION, 0);
        stub(&mut e, REFERENCE_SET_ANGLE, 0);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        stub(&mut e, CELL_FLAG_ZERO_TEST, 0);
        stub(&mut e, REFERENCE_PARENT_CELL, 0x0ce2_0000);
        stub(&mut e, TES_IS_CELL_LOADED, 0);
        stub(&mut e, REFERENCE_GET_PERSISTS, 0);
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        stub(&mut e, GAME_CANCEL_INIT_PACKAGE_LOCATIONS, 0);
        let reference = form(
            &mut e,
            1,
            0x3b,
            0x5000,
            &[
                (0x130, 0x4444),
                (0x1d0, 1),
                (0x1cc, 0),
                (0x100, 1),
                (0x290, 0),
            ],
        );
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 8u32]).u32(),
            6
        );
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_UNLOADED_CELL, 0x4444, 0x5000, 0x6666, 0x3c]
        );
        assert_eq!(e.mem.u32(buffer + 0x28), 0x10);
        assert_eq!(calls(&e, slot(1, 0x1cc)), [vec![reference, 0, 0]]);
        assert_eq!(
            calls(&e, TES_IS_CELL_LOADED)[0],
            [0x0e5e_0000, 0x0ce1_0000, 0]
        );
        assert_eq!(e.mem.u32(reference + 0x170), 0x0100_0077);
        assert!(calls(&e, REFERENCE_MOVE_TO_NEW_SPACE).is_empty());
    }

    #[test]
    fn load_moved_persistent_reference_moves_to_the_world() {
        let mut e = engine();
        let world = form(&mut e, 2, 0x41, 0x3c, &[]);
        load_doubles(&mut e, reference_record(&[]), 0, world);
        e.set_global(PLAYER_CHARACTER, 0x0f0f_0000u32);
        stub(&mut e, REFERENCE_SET_LOCATION, 0);
        stub(&mut e, REFERENCE_SET_ANGLE, 0);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        stub(&mut e, REFERENCE_GET_PERSISTS, 1);
        let moved = capture(&mut e, REFERENCE_MOVE_TO_NEW_SPACE, 0);
        let reference = form(
            &mut e,
            1,
            0x3b,
            0x5000,
            &[(0x1d0, 1), (0x1cc, 0), (0x100, 0)],
        );
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 2u32]).u32(),
            4
        );
        assert_eq!(moved.borrow()[0], [reference, 0, world]);
        assert_eq!(calls(&e, slot(1, 0x1cc)), [vec![reference, 0, 0]]);
    }

    #[test]
    fn load_moved_reference_without_world_or_cell_warns() {
        let mut e = engine();
        load_doubles(&mut e, reference_record(&[]), 0, 0);
        e.set_global(PLAYER_CHARACTER, 0x0f0f_0000u32);
        stub(&mut e, REFERENCE_SET_LOCATION, 0);
        stub(&mut e, REFERENCE_SET_ANGLE, 0);
        stub(&mut e, REFERENCE_UPDATE_3D_POSITION, 0);
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        let reference = form(
            &mut e,
            1,
            0x3b,
            0x5000,
            &[(0x130, 0x4444), (0x1d0, 0), (0x100, 0)],
        );
        let buffer = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0084_f330, &args![buffer, reference, 2u32]).u32(),
            4
        );
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_NO_WORLD_OR_CELL, 0x31, 0x4444, 0x5000]
        );
    }

    #[test]
    fn ref_and_cell_type_bytes_keep_the_upper_bytes_of_the_offset() {
        let mut e = engine();
        assert_eq!(e.call(0x0084_f840, &args![5u32]).u32(), 1);
        assert_eq!(e.call(0x0084_f840, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x0084_f860, &args![2u32]).u32(), 1);
        assert_eq!(e.call(0x0084_f860, &args![5u32]).u32(), 0);
        // Kind 0x100: offset 0x300, whose upper bytes stay in EAX.
        assert_eq!(e.call(0x0084_f840, &args![0x100u32]).u32(), 0x300);
    }

    #[test]
    fn flag_0x10_at_0x28_set_and_cleared() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(buffer + 0x28, 0x101);
        e.call(0x0084_f880, &args![buffer, 0x100u32]);
        assert_eq!(e.mem.u32(buffer + 0x28), 0x101, "only the low byte counts");
        e.call(0x0084_f880, &args![buffer, 1u32]);
        assert_eq!(e.mem.u32(buffer + 0x28), 0x111);
        e.call(0x0084_f880, &args![buffer, 0u32]);
        assert_eq!(e.mem.u32(buffer + 0x28), 0x101);
    }

    #[test]
    fn word_at_0x170_is_stored() {
        let mut e = engine();
        let reference = e.mem.alloc(0x200);
        e.call(0x0084_f8c0, &args![reference, 0xabcdu32]);
        assert_eq!(e.mem.u32(reference + 0x170), 0xabcd);
    }

    fn created_data(e: &mut Engine, bound_id: u32, flags: u8) -> Ptr<ReferenceInitialData> {
        let data: Ptr<ReferenceInitialData> = e.new_object();
        e.set(data, ReferenceInitialData::iLocationID, 0x31);
        e.set(data, ReferenceInitialData::iBoundID, bound_id);
        e.set(data, ReferenceInitialData::cCreatedReferenceFlags, flags);
        data
    }

    #[test]
    fn create_reference_without_bound_object_warns() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        stub(&mut e, LOOKUP_FORM, 0);
        stub(&mut e, DYNAMIC_CAST, 0);
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        let data = created_data(&mut e, 0xff00_0002, 1);
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(e.get(data, ReferenceInitialData::iBoundID), 0);
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_NO_BOUND_OBJECT, 0, 0xff00_0001]
        );
    }

    #[test]
    fn create_reference_creates_by_form_type() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        let bound = form(&mut e, 2, 0x2b, 0x41, &[]);
        let created = form(&mut e, 3, 0x3c, 0, &[(0x128, 0)]);
        stub(&mut e, LOOKUP_FORM, 0);
        stub(&mut e, DYNAMIC_CAST, bound);
        let create = capture(&mut e, REFERENCE_CREATE, created);
        stub(&mut e, REFERENCE_SET_OBJECT, 0);
        stub(&mut e, REFERENCE_SET_PERSISTS, 0);
        stub(&mut e, FORM_SET_DELETED, 0);
        let data = created_data(&mut e, 0x41, 3);
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32])
            .u32();
        assert_eq!(result, created);
        assert_eq!(create.borrow()[0], [0x3c, 1]);
        assert_eq!(calls(&e, REFERENCE_SET_OBJECT), [vec![created, bound]]);
        assert_eq!(calls(&e, slot(3, 0x128)), [vec![created, 0xff00_0001, 1]]);
        assert_eq!(calls(&e, REFERENCE_SET_PERSISTS), [vec![created, 1]]);
        assert_eq!(calls(&e, FORM_SET_DELETED), [vec![created, 1]]);

        // Flag 4 makes type 0x16; flag 1 alone does not persist.
        let data = created_data(&mut e, 0x41, 5);
        e.call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32]);
        assert_eq!(create.borrow()[1], [0x16, 1]);
        assert_eq!(calls(&e, REFERENCE_SET_PERSISTS).len(), 1);

        // Neither flag 1 nor 2: warning, nothing created.
        let warnings = capture(&mut e, SAVE_GAME_WARNING, 0);
        let data = created_data(&mut e, 0x41, 4);
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(
            warnings.borrow()[0],
            [WARNING_INVALID_CREATED_TYPE, 4, 0xff00_0001, 0x41, 0x31]
        );
        assert_eq!(create.borrow().len(), 2);
    }

    #[test]
    fn create_reference_makes_an_explosion_for_type_0x51() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        let bound = form(&mut e, 2, 0x51, 0x41, &[]);
        let explosion = form(&mut e, 3, 0x3c, 0, &[(0x128, 0)]);
        stub(&mut e, LOOKUP_FORM, 0);
        stub(&mut e, DYNAMIC_CAST, bound);
        stub(&mut e, ALLOCATE, explosion);
        stub(&mut e, EXPLOSION_CONSTRUCT, explosion);
        stub(&mut e, REFERENCE_SET_OBJECT, 0);
        stub(&mut e, FORM_SET_DELETED, 0);
        let data = created_data(&mut e, 0x41, 1);
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32])
            .u32();
        assert_eq!(result, explosion);
        assert_eq!(calls(&e, ALLOCATE), [vec![0x104]]);
        assert!(calls(&e, REFERENCE_CREATE).is_empty());
    }

    #[test]
    fn create_reference_keeps_or_clears_the_existing_one() {
        let mut e = engine();
        e.set_global(DATA_HANDLER_POINTER, 0x0abc_0000u32);
        game(&mut e, 0);
        let bound = form(&mut e, 2, 0x2a, 0x41, &[]);
        let other = form(&mut e, 4, 0x2a, 0x42, &[]);
        stub(&mut e, DYNAMIC_CAST, bound);
        stub(&mut e, FORM_SET_DELETED, 0);
        let existing = form(&mut e, 1, 0x3b, 0xff00_0001, &[(0xf0, 1), (0x100, 1)]);
        stub(&mut e, REFERENCE_BASE_OBJECT, other);
        stub(&mut e, REFERENCE_EXTRA_LIST, 0x0bad_0000);
        stub(&mut e, EXTRA_GET_LEV_CREA_ORIGINAL_BASE, bound);
        stub(&mut e, LOOKUP_FORM, existing);
        let data = created_data(&mut e, 0x41, 1);
        // The levelled creature's original base is the bound object: kept.
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, 0u32])
            .u32();
        assert_eq!(result, existing);
        assert!(calls(&e, GAME_CLEAR_FORM).is_empty());

        // Slot 0xf0 false: cleared, then a new one is created (0x3b for 0x2a).
        let existing = form(&mut e, 5, 0x3b, 0xff00_0001, &[(0xf0, 0)]);
        let created = form(&mut e, 3, 0x3b, 0, &[(0x128, 0)]);
        stub(&mut e, GAME_CLEAR_FORM, 0);
        let create = capture(&mut e, REFERENCE_CREATE, created);
        stub(&mut e, REFERENCE_SET_OBJECT, 0);
        let result = e
            .call(0x0084_f8e0, &args![0xff00_0001u32, data, existing])
            .u32();
        assert_eq!(result, created);
        assert_eq!(calls(&e, GAME_CLEAR_FORM)[0][1], existing);
        assert_eq!(create.borrow()[0], [0x3b, 1]);
    }
}
