//! `fallout shared/tesworldspace.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is `TESWorldSpace` (a `TESForm` that owns a map of exterior
//! cells, the persistent cell, the terrain manager and the world-map data)
//! and the small `NiTMap`/`NiTPointerMap` instances it needs. It has 100
//! queue entries, `004fd380` and `00583360` to `00589420`.
//!
//! First session (40 functions, `004fd380` and `00583360` to `00586340`,
//! address order): the form record functions (`LoadPartial`, `Load`, `Save`,
//! `InitItem`, `Copy`, `Compare`, `CreateDuplicateForm`, the two
//! `SavesBefore`), the per-file cell offset tables (`FindCellInFile`,
//! `GetExtCellDataFromFileByEditorID`, `LoadCell`, `UnLoadCell`,
//! `FindLandDataInFile`) and the inherited accessors (climate, image space,
//! water, terrain manager, the `cFlags` bits). The next session continues at
//! `00586390` (`fn_00586390`: the parent world space if the child uses the
//! parent's value `n`).
//!
//! Notes for the next session:
//! - Layout. `TESWorldSpace` is 0xEC bytes on PC (`TESForm` is 0x18, so
//!   every Xbox PDB offset after it is 0x10 lower), and the PC class has an
//!   editor ID string (`BSStringT`) at `+0xC0` that the Xbox class does not
//!   have, so the fields from `fDefaultLandHeight` on are only 8 lower. The
//!   engine map names `005f36f0` `ActorMover::GetPreferredMoveMode`; its body
//!   is `return *(this + 0x34)` and the world space code uses it for
//!   `pPersistentCell`. `0084e3a0` is `return *(this + 0xC)`: the form ID of a
//!   form, and the item count of an `NiTPointerMap`.
//! - Functions of this file that come after the first 40 (so they are not
//!   translated yet) are called by address: `fn_00586390` is `00586390`
//!   (see above), `00587670` `AddCell`, `005875a0` `GetCellFromCellCoord`,
//!   `00588150` `AssignPersistentRefsToCell`, `00588120` (called last by
//!   `CreateDuplicateForm`), `005863d0` (sets `pParentWorld`), `00588a90`
//!   (the `OFFSET_DATA` of a file, below), `00588b00` `CreateOffsetData`,
//!   `00587760` `ReleaseCell`, `00589000` (the `NiTPointerMap<int,
//!   TESObjectCELL *>` constructor, takes the bucket count).
//! - `OFFSET_DATA` (Xbox PDB `TESWorldSpace::OFFSET_DATA`, 0x18 bytes) is the
//!   per-plugin table of file offsets of the exterior cell records:
//!   `pCellFileOffsets` (+0), `OffsetMinCoords` (+4, two floats) and
//!   `OffsetMaxCoords` (+0xC) bound the grid (the code converts the floats
//!   with `_ftol2` and shifts right by 12 to get cell coordinates) and
//!   `iFileOffset` (+0x14) is the offset to add to the table entries.
//! - The plugin reader's record header (`TESFile + 0x240`, `00462270`
//!   returns its address) starts with a tag word, the size at `+4`, flags at
//!   `+8` and the form ID at `+0xC`; for a `GRUP` header (tag at
//!   `0x01187020`) `+8` is the label and `+0xC` the group type. The tags the
//!   unit compares against are initialized in the exe's data.
//! - x87: the game computes in extended precision and stores `float`
//!   results; here the one place a float is computed (the coordinates
//!   `min`/`max` calls) is done by the callee `0040ebd0`/`00404010`.
//!
//! Second session (40 functions, `00586390` to `00588b00`): the parent-value
//! helpers (`fn_00586390`, `fn_005863d0`, the world map data and texture
//! text), the location name of a point (`fn_00586500`, with its cache in the
//! exe's statics), `GetGrassForLocation`, the cell key and the cell map
//! accessors (`GetKeyForWorldCoord`, `GetCellFromWorldCoord`,
//! `GetCellFromCellCoord`, `GetCellFromKey`, `AddCell`, `ReleaseCell`), the
//! overlapped multibound map (`AddMultiBoundRef`), the persistent reference
//! data (the lock, the fixed reference map at `+0x50`, the mobile list at
//! `+0x60`, the persistent cell), the map marker lists, the ring walk over
//! the cells around a point (`fn_005885f0`) and the per-file offset data
//! (`fn_00588a90`, `CreateOffsetData`). The next session continues at
//! `00588c50` (the `OFFSET_DATA` constructor).
//!
//! Notes for the third session:
//! - `BSSimpleList` calls: `006815c0` returns its `this` (the address of the
//!   node's item), `00726070` the next node, `008256d0` tests for an empty
//!   head, `005ae3d0` is `AddHead` (takes the address of the item),
//!   `00905330` removes an item, `00470470` clears and `004702f0(list, 1)` is
//!   the destructor with delete. The list heads that are embedded in a
//!   structure (the data handler's world space list at `+0x10`, the lists
//!   `vcall` slots return at `+4`) start at that address.
//! - The persistent reference lock is the object at `0x011ca60c`:
//!   `004538a0(0)` enters and `004538c0` leaves.
//! - A cell key is `(x << 16) | (y & 0xffff)` (`fn_00587410`); the cell map at
//!   `+0x30` maps it to the cell, the fixed persistent reference map at `+0x50`
//!   and the overlapped multibound map at `+0x68` to a `BSSimpleList` of
//!   references. `OffsetDataMap` at `+0xB0` maps a `TESFile` (the first of a
//!   file's master chain, `00473c70`) to its `OFFSET_DATA`.
//! - Calls whose pushed arguments the decompiler gets wrong: in the location
//!   name function and the grass function the pushes before
//!   `TESObjectCELL::GetRegionList(1)` belong to
//!   `TESRegionList::GetDerivedData(kind, point, world)`; `PUSH 0` before
//!   `BSStringT` text getters belongs to the `Set(text, 0)` call.
//!
//! Third session (22 functions, `00588c50` to `00589420`, the end of the unit):
//! the `OFFSET_DATA` constructor and map teardown, the border region point
//! test (`fn_00588d10`), `AdjustMapMarkerCoord`, and the three `NiTMapBase`
//! instances (constructors, destructors and scalar deleting destructors, base
//! and derived). The unit is complete.
//!
//! Not translated: the compiler's exception-unwinding frames (`FS:[0]`
//! chains and state variables) of `Load`, `InitItem`, `Copy`,
//! `LoadCell`, `CreateDuplicateForm`, the location name, the grass function,
//! `AddMultiBoundRef`, `AddToPersistentRefData`, the map marker lists,
//! `CreatePersistentCell` and `CreateOffsetData`, the stack-cookie check of
//! `Load`, and the `_alloca_probe_16` of `Load` (a stack block of the engine
//! here).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;

/// A four-character chunk tag as the record reader compares it (the bytes
/// in file order, read as a little-endian word).
const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

// ---- Layouts ---------------------------------------------------------------

layout! {
    /// `TESWorldSpace` (Xbox PDB), 0xEC bytes on PC (the Xbox class is 0xF4;
    /// see the module notes for the offsets). Only the fields translated
    /// code uses are listed.
    pub struct TESWorldSpace: 0xEC {
        /// `pCellMap` (Xbox PDB): `NiTPointerMap<int, TESObjectCELL *> *`.
        0x30 pCellMap: Ptr,
        /// `pPersistentCell` (Xbox PDB): `TESObjectCELL *`.
        0x34 pPersistentCell: Ptr,
        /// `pTerrainManager` (Xbox PDB): `BGSTerrainManager *`.
        0x3C pTerrainManager: Ptr,
        /// `pClimate` (Xbox PDB): `TESClimate *`. Holds the form ID until
        /// `InitItem` resolves it.
        0x40 pClimate: Ptr,
        /// `pImageSpace` (Xbox PDB): `TESImageSpace *` (form ID until
        /// `InitItem`).
        0x44 pImageSpace: Ptr,
        /// `pImpactSwap` (Xbox PDB): `ImpactSwap *`.
        0x48 pImpactSwap: Ptr,
        /// `cFlags` (Xbox PDB): bit 0 (no cells of its own), bit 1, bit 3
        /// (`GetHasBorderRegion`), bits 4, 5, 6 (inverted by `00586320`) and 7
        /// are read by the accessors translated here.
        0x4C cFlags: u8,
        /// `sParentUseFlags` (Xbox PDB): bit `n` says the world space uses
        /// its parent's value `n`.
        0x4E sParentUseFlags: u16,
        /// `FixedPersistentRefMap` (Xbox PDB): an
        /// `NiTPointerMap<unsigned int, BSSimpleList<TESObjectREFR *> *>`
        /// from a cell key to the fixed persistent references there.
        0x50 FixedPersistentRefMap: Inline<crate::types::NiTPointerMap>,
        /// `MobilePersistentRefList` (Xbox PDB): a
        /// `BSSimpleList<TESObjectREFR *>` head.
        0x60 MobilePersistentRefList: Inline<crate::types::BSSimpleList>,
        /// `pOverlappedMultiboundMap` (Xbox PDB): an `NiTPointerMap` from a
        /// cell key to the multibound references overlapping that cell.
        0x68 pOverlappedMultiboundMap: Ptr,
        /// `pParentWorld` (Xbox PDB): `TESWorldSpace *` (form ID until
        /// `InitItem`).
        0x70 pParentWorld: Ptr,
        /// `pWorldWater` (Xbox PDB): `TESWaterForm *`.
        0x74 pWorldWater: Ptr,
        /// `pLODWater` (Xbox PDB): `TESWaterForm *`.
        0x78 pLODWater: Ptr,
        /// `fLODWaterHeight` (Xbox PDB).
        0x7C fLODWaterHeight: f32,
        /// `WorldMapData` (Xbox PDB): a [`WorldMapData`].
        0x80 WorldMapData: Inline<WorldMapData>,
        /// `WorldMapOffsetData` (Xbox PDB): a [`WorldMapOffsetData`].
        0x90 WorldMapOffsetData: Inline<WorldMapOffsetData>,
        /// `pMusicType` (Xbox PDB): `BGSMusicType *` (form ID until
        /// `InitItem`).
        0x9C pMusicType: Ptr,
        /// `MinimumCoords` (Xbox PDB, `NiPoint2`): x.
        0xA0 MinimumCoords_x: f32,
        /// `MinimumCoords`: y.
        0xA4 MinimumCoords_y: f32,
        /// `MaximumCoords` (Xbox PDB, `NiPoint2`): x.
        0xA8 MaximumCoords_x: f32,
        /// `MaximumCoords`: y.
        /// `OffsetDataMap` (Xbox PDB): an `NiTMap<TESFile *,
        /// TESWorldSpace::OFFSET_DATA *>`.
        0xB0 OffsetDataMap: Inline<crate::types::NiTPointerMap>,
        0xAC MaximumCoords_y: f32,
        /// PC only: the editor ID, a `BSStringT` (the form's name getter,
        /// vtable slot `+0x130`, returns its text).
        0xC0 cEditorID: Inline<crate::types::BSStringT>,
        /// `fDefaultLandHeight` (Xbox PDB).
        0xC8 fDefaultLandHeight: f32,
        /// `fDefaultWaterHeight` (Xbox PDB).
        0xCC fDefaultWaterHeight: f32,
        /// `pEncounterZone` (Xbox PDB): `BGSEncounterZone *` (form ID until
        /// `InitItem`).
        0xD0 pEncounterZone: Ptr,
        /// `CanopyShadowTexture` (Xbox PDB): a `TESTexture`, 0xC bytes.
        0xD4 CanopyShadowTexture: u32,
        /// `WaterNoiseTexture` (Xbox PDB): a `TESTexture`, 0xC bytes.
        0xE0 WaterNoiseTexture: u32,
    }

    /// `WORLD_MAP_DATA` (Xbox PDB), 0x10 bytes.
    pub struct WorldMapData: 0x10 {
        /// `iUsableWidth` (Xbox PDB).
        0x00 iUsableWidth: u32,
        /// `iUsableHeight` (Xbox PDB).
        0x04 iUsableHeight: u32,
        /// `sNWCellX` (Xbox PDB).
        0x08 sNWCellX: i16,
        /// `sNWCellY` (Xbox PDB).
        0x0A sNWCellY: i16,
        /// `sSECellX` (Xbox PDB).
        0x0C sSECellX: i16,
        /// `sSECellY` (Xbox PDB).
        0x0E sSECellY: i16,
    }

    /// `WORLD_MAP_OFFSET_DATA` (Xbox PDB), 0xC bytes.
    pub struct WorldMapOffsetData: 0x0C {
        /// `fMapScale` (Xbox PDB).
        0x00 fMapScale: f32,
        /// `fMapOffsetX` (Xbox PDB).
        0x04 fMapOffsetX: f32,
        /// `fMapOffsetY` (Xbox PDB).
        0x08 fMapOffsetY: f32,
    }

    /// `TESWorldSpace::OFFSET_DATA` (Xbox PDB), 0x18 bytes: the table of
    /// exterior cell record offsets one plugin file has for a world space.
    pub struct OffsetData: 0x18 {
        /// `pCellFileOffsets` (Xbox PDB): `u32 *`, one offset per cell of
        /// the grid (0 = none), row by row.
        0x00 pCellFileOffsets: Ptr,
        /// `OffsetMinCoords.x` (Xbox PDB, `NiPoint2` at +4).
        0x04 OffsetMinCoords_x: f32,
        /// `OffsetMinCoords.y`.
        0x08 OffsetMinCoords_y: f32,
        /// `OffsetMaxCoords.x` (Xbox PDB, `NiPoint2` at +0xC).
        0x0C OffsetMaxCoords_x: f32,
        /// `OffsetMaxCoords.y`.
        0x10 OffsetMaxCoords_y: f32,
        /// `iFileOffset` (Xbox PDB): added to the table entries.
        0x14 iFileOffset: u32,
    }
}

// ---- Callees ---------------------------------------------------------------

/// `MemoryManager::Allocate` (`00401000`) and `Deallocate` (`00401030`),
/// `__cdecl`.
const MEMORY_ALLOC: u32 = 0x0040_1000;
const MEMORY_FREE: u32 = 0x0040_1030;
/// `memset(ptr, value, size)` (`00403d30`) and `memcpy(dest, source, size)`
/// (`00401460`), `__cdecl`.
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
/// `memcmp(a, b, size)` (`00ec4835`, LIBCMT).
const MEMCMP: u32 = 0x00ec_4835;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX; the uniform form passes the value as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `__RTDynamicCast(object, 0, TESForm type, target type, 0)` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `float min(a, b)` (`0040ebd0`) and `float max(a, b)` (`00404010`),
/// results in ST0.
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;
/// Endian swaps of a 4-byte (`00401080`) and a 2-byte (`00407a90`) value in
/// place, `__cdecl(pointer, byte)`; the byte is always 0 here.
const SWAP_DWORD: u32 = 0x0040_1080;
const SWAP_WORD: u32 = 0x0040_7a90;
/// `__thiscall(this)` endian swaps of the two floats of the cell offset
/// block (`00462230`, used on the `DNAM` heights and on the cell grid
/// block) and of the 12-byte map offset block (`005a6b10`).
const SWAP_PAIR: u32 = 0x0046_2230;
const SWAP_MAP_OFFSET_BLOCK: u32 = 0x005a_6b10;
/// Whether the game is saving big-endian data (`00401500`, a global byte).
const SAVING_SWAPPED: u32 = 0x0040_1500;
/// `BSSimpleList` head item / `this` returned unchanged by the folded
/// constructors of the small local structures (`006815c0`, `00540720`).
const LOCAL_STRUCT_CONSTRUCT: u32 = 0x0068_15c0;
const CELL_COORDS_CONSTRUCT: u32 = 0x0054_0720;

// `TESFile` (tesfile.cpp), `__thiscall` on the file.
/// `GetTESForm`: the record type of the current form.
const FILE_RECORD_TYPE: u32 = 0x0047_2660;
/// `GetMaster`: whether the file is a master file.
const FILE_IS_MASTER: u32 = 0x0047_1c20;
/// `GetTESChunk`: the tag of the current chunk, 0 at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26b0;
/// Moves to the next chunk; `false` when there is none (`004726f0`).
const FILE_ADVANCE_CHUNK: u32 = 0x0047_26f0;
/// `GetChunkData(buffer, size)` (`00472890`).
const FILE_READ_CHUNK: u32 = 0x0047_2890;
/// `GetChunkData(u32 *)` (`004727f0`) and `GetChunkData(u16 *)`
/// (`00472840`): read one value of the chunk.
const FILE_READ_CHUNK_U32: u32 = 0x0047_27f0;
const FILE_READ_CHUNK_U16: u32 = 0x0047_2840;
/// The size of the current chunk (`00401660`, a word at `+0x25C`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Whether the file's data is byte-swapped (`00401680`, the byte at
/// `+0x299`).
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `SetOffset(offset)`: positions the reader on a record.
const FILE_SET_OFFSET: u32 = 0x0047_23a0;
/// `FindForm(form)` (`004734d0`).
const FILE_FIND_FORM: u32 = 0x0047_34d0;
/// `NextForm(1)` (`00472150`).
const FILE_NEXT_FORM: u32 = 0x0047_2150;
/// The address of the current record header (`00462270`, `file + 0x240`).
const FILE_CURRENT_RECORD: u32 = 0x0046_2270;
/// The word at `file + 0x264` (`00467bb0`): the offset of the current form
/// data, stored in `OFFSET_DATA::iFileOffset` and returned as the land
/// data's position.
const FILE_CURRENT_OFFSET: u32 = 0x0046_7bb0;
/// The form ID of the current record (`008d8ac0`, the word at
/// `file + 0x24C`).
const FILE_CURRENT_FORM_ID: u32 = 0x008d_8ac0;
/// Leaves the current `GRUP` (`00473660`, resets the group state and
/// calls `004721d0`) and `00472000` (rewinds the reader to the start of the
/// current record).
const FILE_LEAVE_GROUP: u32 = 0x0047_3660;
const FILE_REWIND_RECORD: u32 = 0x0047_2000;
/// `TESFile::GetThreadSafeFile` (`004739b0`).
const FILE_THREAD_SAFE: u32 = 0x0047_39b0;

// `TESForm`.
/// `GetFile(index)` (`00484e60`; -1 for the last file).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `AddCompileIndex(&form id, file)` (`00485d50`).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form with a given ID (`004839c0`).
const FORM_BY_ID: u32 = 0x0048_39c0;
/// The word at `+0xC` (`0084e3a0`): a form's ID, a map's item count.
const WORD_AT_0C: u32 = 0x0084_e3a0;
/// The record type byte of a form (`00401170`, `+4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// The flags word of a form (`0044ddc0`, `+8`).
const FORM_FLAGS: u32 = 0x0044_ddc0;
/// The form flag 8 test (`004013e0`).
const FORM_FLAG_8: u32 = 0x0040_13e0;
/// `TESForm::LoadForm(file)` (`00485110`).
const FORM_LOAD: u32 = 0x0048_5110;
/// `00484ab0(flag)`: a form state setter called with 0 by `Load` and 1 by
/// `InitItem`.
const FORM_SET_STATE: u32 = 0x0048_4ab0;
/// `00484e40`: a form query that picks the cell map size in `Load`.
const FORM_QUERY_LARGE_MAP: u32 = 0x0048_4e40;
/// `TESForm::CompareAllComponents(other)` (`00485270`) and
/// `TESForm::CopyAllComponents(source)` (`004851b0`).
const FORM_COMPARE_COMPONENTS: u32 = 0x0048_5270;
const FORM_COPY_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CreateDuplicateForm(flag, other)` (`004867a0`).
const FORM_CREATE_DUPLICATE: u32 = 0x0048_67a0;
/// `TESForm::SavesBefore(group header)` base version (`00484150`) and the
/// default `SavesBefore(form)` (`00484020`).
const FORM_SAVES_BEFORE_HEADER: u32 = 0x0048_4150;
const FORM_SAVES_BEFORE_FORM: u32 = 0x0048_4020;
/// `StartForm` (`004855a0`), `CloseForm` (`00485680`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
/// `AddChunk` helpers of the save record writer, `__cdecl`: a 4-byte value
/// (`00485910`), a 2-byte value (`00485950`), a byte (`004858f0`), a block
/// (`00485990`, `004856f0`) and an array of items (`00485710`).
const ADD_CHUNK_U32: u32 = 0x0048_5910;
const ADD_CHUNK_U16: u32 = 0x0048_5950;
const ADD_CHUNK_U8: u32 = 0x0048_58f0;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const ADD_CHUNK_BLOCK: u32 = 0x0048_56f0;
const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;

/// `TESFullName::Save` (`00487010`) and `TESFullName::Load` (`00487050`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESTexture::Save(tag)` (`0048e370`), `LoadTextureChunk(texture, file)`
/// (`0048e3d0`) and `SetSoundFile(path)` (`00489100`, the folded texture
/// name setter), `GetNameLength` (`0048cee0`) and the text getter
/// (`00408da0`, which returns a default empty string when there is none).
const TEXTURE_SAVE: u32 = 0x0048_e370;
const TEXTURE_LOAD_CHUNK: u32 = 0x0048_e3d0;
const TEXTURE_SET_NAME: u32 = 0x0048_9100;
const TEXTURE_NAME_LENGTH: u32 = 0x0048_cee0;
const TEXTURE_NAME_TEXT: u32 = 0x0040_8da0;

/// Setters of fields this class owns, translated in other units (folded
/// identical code): climate (`0087ce80`, `+0x40`), image space (`008d8040`,
/// `+0x44`), water (`004febb0`, `+0x74`), lod water (`00442a80`, `+0x78`)
/// and music type (`00810570`, `+0x9C`); and their getter for `+0x44`
/// (`008041a0`).
const SET_CLIMATE: u32 = 0x0087_ce80;
const SET_IMAGE_SPACE: u32 = 0x008d_8040;
const SET_WATER: u32 = 0x004f_ebb0;
const SET_LOD_WATER: u32 = 0x0044_2a80;
const SET_MUSIC_TYPE: u32 = 0x0081_0570;
const GET_IMAGE_SPACE_RAW: u32 = 0x0080_41a0;
/// Setter of `pParentWorld` (`005863d0`, a later function of this file).
const SET_PARENT_WORLD: u32 = 0x0058_63d0;
/// `fn_00586390(this, n)`: the parent world space if this world space uses
/// the parent's value `n`, else 0 (a later function of this file).
const PARENT_FOR: u32 = 0x0058_6390;
/// `fLODWaterHeight` getter (`0045cd80`, a float in ST0).
const GET_LOD_WATER_HEIGHT: u32 = 0x0045_cd80;
/// The `OFFSET_DATA` of a file (`00588a90`, a later function of this
/// file), `CreateOffsetData(file)` (`00588b00`).
const OFFSET_DATA_OF_FILE: u32 = 0x0058_8a90;
const CREATE_OFFSET_DATA: u32 = 0x0058_8b00;
/// `AddCell(cell)` (`00587670`), `GetCellFromCellCoord(x, y)` (`005875a0`),
/// `ReleaseCell(cell)` (`00587760`), `AssignPersistentRefsToCell(cell)`
/// (`00588150`) and `00588120` (called last by `CreateDuplicateForm`).
const ADD_CELL: u32 = 0x0058_7670;
const GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
const RELEASE_CELL: u32 = 0x0058_7760;
const ASSIGN_PERSISTENT_REFS: u32 = 0x0058_8150;
const FINISH_DUPLICATE: u32 = 0x0058_8120;
/// `ImpactSwap` (impactswap.cpp, 0x15C bytes): constructor (`0058efd0`),
/// destructor with delete flag (`0040f2a0`), `InitItem(world space)`
/// (`0058f210`), `Load` (`0058f180`), `Copy` (`0058f4c0`), `Compare`
/// (`0058f5c0`), `Save` (`0058f080`).
const IMPACT_SWAP_SIZE: u32 = 0x15C;
const IMPACT_SWAP_CONSTRUCT: u32 = 0x0058_efd0;
const IMPACT_SWAP_DESTRUCT: u32 = 0x0040_f2a0;
const IMPACT_SWAP_INIT_ITEM: u32 = 0x0058_f210;
const IMPACT_SWAP_LOAD: u32 = 0x0058_f180;
const IMPACT_SWAP_COPY: u32 = 0x0058_f4c0;
const IMPACT_SWAP_COMPARE: u32 = 0x0058_f5c0;
const IMPACT_SWAP_SAVE: u32 = 0x0058_f080;
/// `BGSTerrainManager` constructor (`006fc490`, takes the world space), 0x3C
/// bytes.
const TERRAIN_MANAGER_SIZE: u32 = 0x3C;
const TERRAIN_MANAGER_CONSTRUCT: u32 = 0x006f_c490;
/// `NiTPointerMap<int, TESObjectCELL *>` (0x10 bytes) constructor with its
/// bucket count (`00589000`), and the map operations `GetFirstPos`
/// (`004b9ba0`), `GetNext(&pos, &key, &value)` (`006b7f20`), `SetAt(key,
/// value)` (`00844700`) and `RemoveAll` (`00438af0`).
const CELL_MAP_SIZE: u32 = 0x10;
const CELL_MAP_CONSTRUCT: u32 = 0x0058_9000;
const MAP_FIRST_POS: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
const MAP_SET_AT: u32 = 0x0084_4700;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// Hash sizes `Load` gives the cell map: 0x1B59 when `00484e40` is true,
/// else 0x2BD.
const LARGE_CELL_MAP_BUCKETS: u32 = 0x1b59;
const SMALL_CELL_MAP_BUCKETS: u32 = 0x02bd;
/// The persistent cell getter (`005f36f0`, the folded `return *(this +
/// 0x34)`).
const GET_PERSISTENT_CELL: u32 = 0x005f_36f0;
/// `TESObjectCELL` (tesobjectcell.cpp), 0xE0 bytes: constructor
/// (`005415b0`), `GetWorldSpace` (`0054ddd0`), `SetInterior(flag)`
/// (`00544300`), `CreateCellData` (`00544630`), `SetDataCoord(x, y)`
/// (`00544c90`), `LoadTempDataFromFile(file)` (`00550500`), the cell
/// cleanup (`005508b0`), the temp-data query (`00551420`),
/// `SetHasTempData(flag)` (`00551440`), `FindLandDataInFile(file)`
/// (`00550f60`, `__cdecl`), the file count of a form (`005504e0`) and the
/// block keys `CalcExtGroupBlockKey(x, y)` (`00544280`) and
/// `CalcExtGroupSubBlockKey(x, y)` (`005442c0`), `__cdecl`.
const CELL_SIZE: u32 = 0xE0;
const CELL_CONSTRUCT: u32 = 0x0054_15b0;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_SET_INTERIOR: u32 = 0x0054_4300;
const CELL_CREATE_DATA: u32 = 0x0054_4630;
const CELL_SET_DATA_COORD: u32 = 0x0054_4c90;
const CELL_LOAD_TEMP_DATA: u32 = 0x0055_0500;
const CELL_CLEANUP: u32 = 0x0055_08b0;
const CELL_HAS_TEMP_DATA: u32 = 0x0055_1420;
const CELL_SET_HAS_TEMP_DATA: u32 = 0x0055_1440;
const CELL_FIND_LAND_DATA_IN_FILE: u32 = 0x0055_0f60;
const FORM_FILE_COUNT: u32 = 0x0055_04e0;
const CALC_BLOCK_KEY: u32 = 0x0054_4280;
const CALC_SUB_BLOCK_KEY: u32 = 0x0054_42c0;
/// `TESDataHandler::LoadForm(form, file)` (`004601d0`, `__cdecl`).
const DATA_HANDLER_LOAD_FORM: u32 = 0x0046_01d0;
/// The `TESSaveLoadGame` singleton (`0x011de45c`), the `this` of
/// `GetCreatedExteriorCellFormID(world form ID, x, y)` (`00861640`, the form
/// ID the save game made for an exterior cell, 0 for none) and of three
/// functions whose bodies do nothing: `008d0370(cell)` returns 0,
/// `0047c850()` returns 0 and `004534f0(state)` ignores its argument.
/// `LoadCell` still calls them (the last two around the cell's `InitItem`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_CREATED_CELL_ID: u32 = 0x0086_1640;
const SAVE_LOAD_NOTE_CELL: u32 = 0x008d_0370;
const SAVE_LOAD_GET_STATE: u32 = 0x0047_c850;
const SAVE_LOAD_SET_STATE: u32 = 0x0045_34f0;
/// The data loader state (`0x011c3f2c`), the `this` of `00469860(form ID)`
/// (true for a form ID of 0xFF000000 or above, a runtime-created form), and
/// the game loader (`0x011ddf38`) with its `UnloadForm(cell, 0)`
/// (`00849730`).
const LOADER_STATE: u32 = 0x011c_3f2c;
const LOADER_STATE_QUERY: u32 = 0x0046_9860;
const GAME_LOADER: u32 = 0x011d_df38;
const GAME_LOADER_UNLOAD_FORM: u32 = 0x0084_9730;
/// The world space being loaded, a global the cell code reads
/// (`0x011c3f34`).
const CURRENT_WORLD_SPACE: u32 = 0x011c_3f34;
/// The default water form, a global pointer (`0x011ca53c`).
const DEFAULT_WATER: u32 = 0x011c_a53c;
/// The case-insensitive string compare used on editor IDs (`00404dc0`).
const EDITOR_ID_COMPARE: u32 = 0x0040_4dc0;
/// The memory-context guard of the game's stack frame: constructor
/// `__thiscall(guard, context, 1, source file name, source line)`
/// (`00404eb0`) and destructor (`00404ee0`), and the source file name.
const MEMORY_CONTEXT_ENTER: u32 = 0x0040_4eb0;
const MEMORY_CONTEXT_LEAVE: u32 = 0x0040_4ee0;
const WORLD_SPACE_SOURCE: u32 = 0x0103_1f18;
/// The game log function (`005b5e40`, `__cdecl`, variadic).
const LOG: u32 = 0x005b_5e40;
/// The form's editor ID: `00474cb0` returns its length (0 inside a
/// recursive call), and slot `+0x130` returns the text.
const EDITOR_ID_LENGTH: u32 = 0x0047_4cb0;
const SLOT_EDITOR_ID: u32 = 0x130;
const SLOT_SET_EDITOR_ID: u32 = 0x134;

// Globals: record tags initialized in the exe's data.
/// The `GRUP` tag word of group headers.
const GROUP_TAG: u32 = 0x0118_7020;
/// The tag word of cell records.
const CELL_TAG: u32 = 0x0118_72b4;
/// The tag word of a record the cell scan skips whole (`01187338`).
const SKIPPED_RECORD_TAG_01187338: u32 = 0x0118_7338;
/// The record type byte of a world space record (`0118730c`).
const WORLD_SPACE_RECORD_TYPE: u32 = 0x0118_730c;

// RTTI type descriptors (`.?AV<name>@@`), read in the exe.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
const TYPE_TES_WORLD_SPACE: u32 = 0x0118_3fd0;
const TYPE_TES_CHILD_CELL: u32 = 0x0118_ac2c;
const TYPE_TES_CLIMATE: u32 = 0x0118_4170;
const TYPE_TES_IMAGE_SPACE: u32 = 0x0118_4154;
const TYPE_BGS_ENCOUNTER_ZONE: u32 = 0x0118_40dc;
const TYPE_TES_WATER_FORM: u32 = 0x0118_4118;
const TYPE_BGS_MUSIC_TYPE: u32 = 0x0118_40fc;

// Form type and form flag values.
/// Record type of a world space (`'A'`).
const RECORD_TYPE_WORLD_SPACE: u32 = 0x41;
/// Record type of a cell (`0x39`).
const RECORD_TYPE_CELL: u32 = 0x39;

// Chunk tags of the world space record.
const CHUNK_CLIMATE: u32 = tag(b"CNAM");
const CHUNK_DATA: u32 = tag(b"DATA");
const CHUNK_LOD_WATER: u32 = tag(b"NAM3");
const CHUNK_MIN_COORDS: u32 = tag(b"NAM0");
const CHUNK_WATER: u32 = tag(b"NAM2");
const CHUNK_LOD_WATER_HEIGHT: u32 = tag(b"NAM4");
const CHUNK_MAX_COORDS: u32 = tag(b"NAM9");
const CHUNK_IMPACT_FIRST: u32 = tag(b"IMPF");
const CHUNK_IMPACT_SECOND: u32 = tag(b"IMPS");
const CHUNK_EDITOR_ID: u32 = tag(b"EDID");
const CHUNK_OBJECT_BOUNDS: u32 = tag(b"OBND");
const CHUNK_FULL_NAME: u32 = tag(b"FULL");
const CHUNK_ICON: u32 = tag(b"ICON");
const CHUNK_ENCOUNTER_ZONE: u32 = tag(b"XEZN");
const CHUNK_OFFSETS: u32 = tag(b"OFST");
const CHUNK_DEFAULT_HEIGHTS: u32 = tag(b"DNAM");
const CHUNK_IMAGE_SPACE: u32 = tag(b"INAM");
const CHUNK_MAP_DATA: u32 = tag(b"MNAM");
const CHUNK_CANOPY_TEXTURE: u32 = tag(b"NNAM");
const CHUNK_MAP_OFFSET_DATA: u32 = tag(b"ONAM");
const CHUNK_PARENT_USE_FLAGS: u32 = tag(b"PNAM");
const CHUNK_SKIPPED: u32 = tag(b"SNAM");
const CHUNK_PARENT_WORLD: u32 = tag(b"WNAM");
const CHUNK_WATER_NOISE_TEXTURE: u32 = tag(b"XNAM");
const CHUNK_MUSIC_TYPE: u32 = tag(b"ZNAM");
/// The cell record's grid position chunk (`XCLC`).
const CHUNK_CELL_GRID: u32 = tag(b"XCLC");

// Log message formats (in the exe's data).
const MSG_CLIMATE_NAMED: u32 = 0x0103_1e78;
const MSG_CLIMATE_ID: u32 = 0x0103_1e30;
const MSG_IMAGE_SPACE_NAMED: u32 = 0x0103_1de8;
const MSG_IMAGE_SPACE_ID: u32 = 0x0103_1d98;
const MSG_ENCOUNTER_ZONE_NAMED: u32 = 0x0103_1d54;
const MSG_ENCOUNTER_ZONE_ID: u32 = 0x0103_1d10;
const MSG_WATER_NAMED: u32 = 0x0103_1cc8;
const MSG_WATER_ID: u32 = 0x0103_1c78;
const MSG_LOD_WATER_NAMED: u32 = 0x0103_1c28;
const MSG_LOD_WATER_ID: u32 = 0x0103_1bd8;
const MSG_MUSIC_NAMED: u32 = 0x0103_1b90;
const MSG_MUSIC_ID: u32 = 0x0103_1b40;
const MSG_PARENT_NAMED: u32 = 0x0103_1af0;
const MSG_PARENT_ID: u32 = 0x0103_1aa0;
const MSG_CELL_LOAD_FAILED: u32 = 0x0103_1ec0;

// ---- Callees and data of the second session (00586390 to 00588b00) ---------

/// `BSSimpleList` node functions, all `__thiscall` on a node: the node
/// itself (`006815c0` returns its `this`, the address of the node's item
/// word, which is what the folded constructors of small structures return
/// as well), the next node (`00726070`, the word at `+4`), whether the list
/// is empty (`008256d0`: item and next are both 0), `AddHead`-style add of
/// the item whose address is passed (`005ae3d0`), remove of the item whose
/// address is passed (`00905330`), clear (`00470470`: deletes every node
/// after the head and empties the head), the destructor with delete flag
/// (`004702f0`) and the constructor (`0096a2d0`: both words 0).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DESTROY: u32 = 0x0047_02f0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Size of a `BSSimpleList` head allocated on the heap.
const LIST_SIZE: u32 = 8;
/// `NiTMapBase::GetAt(key, &value)` (`00853130`, `false` when absent) and
/// `RemoveAt(key)` (`00405430`), `__thiscall` on the map.
const MAP_GET: u32 = 0x0085_3130;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// The constructor of the `NiTPointerMap<unsigned int, BSSimpleList *>`
/// (`00588fa0`, takes the bucket count; a later function of this file), its
/// size and bucket count in `AddMultiBoundRef`.
const LIST_MAP_SIZE: u32 = 0x10;
const LIST_MAP_CONSTRUCT: u32 = 0x0058_8fa0;
const MULTIBOUND_MAP_BUCKETS: u32 = 0x25;
/// The sentinel the persistent reference lock functions work on
/// (`0x011ca60c`): `Enter(0)` (`004538a0`) and `Leave` (`004538c0`).
const PERSISTENT_REF_LOCK: u32 = 0x011c_a60c;
const LOCK_ENTER: u32 = 0x0045_38a0;
const LOCK_LEAVE: u32 = 0x0045_38c0;
/// `_finite(double)` (`00ec7595`) and `_isnan(double)` (`00ec75b1`),
/// `__cdecl`: non-zero for true.
const IS_FINITE: u32 = 0x00ec_7595;
const IS_NAN: u32 = 0x00ec_75b1;
/// `int round(float)` (`00406d90`, `__cdecl`: `FLD`/`FISTP`, round to
/// nearest) and `_vector_constructor_iterator_(array, size, count,
/// constructor)` (`00401050`).
const FLOAT_ROUND: u32 = 0x0040_6d90;
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `sprintf_s(buffer, size, format, ...)` (`00406d00`, `__cdecl`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// Tests of a form: flag `0x4000` of `+8` (`004077c0`), flag `0x400`
/// (`005516c0`, via the flags getter `0044ddc0`), the base form of a
/// reference (`007af430`, the word at `+0x20`), a reference's
/// `+0x40` word (`008d6f30`) and the half-extent of a multibound
/// reference (`00569880`), its radius (`00457990`) and whether it is a
/// multibound marker (`00439f90`).
const FORM_FLAG_4000: u32 = 0x0040_77c0;
const FORM_FLAG_400: u32 = 0x0055_16c0;
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
const REFERENCE_WORD_40: u32 = 0x008d_6f30;
const REFERENCE_MULTIBOUND_HALF_EXTENT: u32 = 0x0056_9880;
const MULTIBOUND_RADIUS: u32 = 0x0045_7990;
const IS_MULTIBOUND_REF: u32 = 0x0043_9f90;
/// `MultiBoundMarkerData::MultiBoundIntersectsCell(reference, x, y)`
/// (`00439940`, `__cdecl`).
const MULTIBOUND_INTERSECTS_CELL: u32 = 0x0043_9940;
/// Vtable slot of a reference that returns the address of its position
/// (three floats).
const SLOT_POSITION: u32 = 0x1f4;
/// `TESObjectCELL`: bit 0 of the byte at `+0x24` (`00425fd0`),
/// `GetDataX` (`00544c30`), `GetDataY` (`00544c60`), `SetWorldSpace(world)`
/// (`0054de10`), `GetRegionList(flag)` (`00547110`),
/// `AddReference(reference, 0)` (`00548230`), `RemoveReference(reference)`
/// (`0054ca90`), `AssignPersistentRefsToCellsInWorld(world)` (`0054c8c0`),
/// `SetPersistentCell(flag)` (`005516f0`), the distance from the cell's
/// square to a point (`0054fb70`, a float in ST0), whether the cell's
/// square contains a point (`00550200`) and the visit of the references in
/// range of two points (`0054da20`).
const CELL_FLAG_24_BIT_0: u32 = 0x0042_5fd0;
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
const CELL_SET_WORLD_SPACE: u32 = 0x0054_de10;
const CELL_GET_REGION_LIST: u32 = 0x0054_7110;
const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
const CELL_ASSIGN_PERSISTENT_REFS_IN_WORLD: u32 = 0x0054_c8c0;
const CELL_SET_PERSISTENT: u32 = 0x0055_16f0;
const CELL_DISTANCE_TO_POINT: u32 = 0x0054_fb70;
const CELL_CONTAINS_POINT: u32 = 0x0055_0200;
const CELL_FOR_REFERENCES_IN_RANGE: u32 = 0x0054_da20;
/// `0054db50(cell, argument)` and the two list fillers `0054b830(cell,
/// list)` and `0054b8c0(cell, list)` the persistent cell offers the
/// world space's list builders.
const CELL_PERSISTENT_ACTION: u32 = 0x0054_db50;
const CELL_FILL_LIST_FIRST: u32 = 0x0054_b830;
const CELL_FILL_LIST_SECOND: u32 = 0x0054_b8c0;
/// `TESDataHandler` (the object `0x011c3f2c` points to):
/// `GetCellFromWorldCoord(x, y, world, 0)` (`00461bc0`), the region manager
/// (`00740940`, the word at `+0x624`), the list of world spaces
/// (`00460140`, the address `+0x10`) and the list at `+0x1d8`
/// (`004169d0`).
const DATA_HANDLER_POINTER: u32 = 0x011c_3f2c;
const DATA_HANDLER_CELL_FROM_COORD: u32 = 0x0046_1bc0;
const DATA_HANDLER_REGION_MANAGER: u32 = 0x0074_0940;
const DATA_HANDLER_WORLD_LIST: u32 = 0x0046_0140;
const DATA_HANDLER_LIST_1D8: u32 = 0x0041_69d0;
/// `TESRegionList::GetDerivedData(kind, point, world)` (`004f6800`).
const REGION_LIST_GET_DERIVED_DATA: u32 = 0x004f_6800;
/// Region data accessors: the word at `+0x18` of a region data list entry
/// (`009611e0`), `TESRegionDataList::Find(kind)` (`004f35b0`), the byte at
/// `+4` (`004f1540`) and at `+6` (`005bb4d0`) of a region data, the region
/// data list of an entry (`00441110`, the word at `+0x1c`), the flag `0x20`
/// of `+8` (`00440d80`), the object comparing a point to a region entry
/// (`004f7030` builds it from a point, `004f8360` tests an entry).
const REGION_ENTRY_WORD_18: u32 = 0x0096_11e0;
const REGION_DATA_LIST_FIND: u32 = 0x004f_35b0;
const REGION_DATA_BYTE_4: u32 = 0x004f_1540;
const REGION_DATA_BYTE_6: u32 = 0x005b_b4d0;
const REGION_ENTRY_LIST: u32 = 0x0044_1110;
const REGION_ENTRY_FLAG_20: u32 = 0x0044_0d80;
const REGION_POINT_BUILD: u32 = 0x004f_7030;
const REGION_POINT_IN_ENTRY: u32 = 0x004f_8360;
/// `BSStringT<char>`: constructor (`004037b0`), `Set(text, 0)` (`004037f0`),
/// the text pointer (`00559450`), `GetLength` (`004048e0`) and `StrCmp(text,
/// ignore case)` (`00408a80`); the empty text (`0x01011584`).
const BSSTRING_CONSTRUCT: u32 = 0x0040_37b0;
const BSSTRING_SET: u32 = 0x0040_37f0;
const BSSTRING_TEXT: u32 = 0x0055_9450;
const BSSTRING_LENGTH: u32 = 0x0040_48e0;
const BSSTRING_COMPARE: u32 = 0x0040_8a80;
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `NiPoint3::operator==` (`004390c0`, `__thiscall`, takes the address of
/// the other point).
const POINT_EQUAL: u32 = 0x0043_90c0;
/// `_atexit` (`00ec658f`) and the destructor `00fcb0c0` of the cached text
/// the location name function registers.
const ATEXIT: u32 = 0x00ec_658f;
const LOCATION_CACHE_ATEXIT: u32 = 0x00fc_b0c0;
/// The statics of the location name function (`00586500`): the world
/// space, the point (`NiPoint3`) and the text (`BSStringT`) of its last
/// answer, and the guard bits of their initialization.
const LOCATION_CACHE_WORLD: u32 = 0x011c_a648;
const LOCATION_CACHE_POINT: u32 = 0x011c_a64c;
const LOCATION_CACHE_TEXT: u32 = 0x011c_a658;
const LOCATION_CACHE_GUARD: u32 = 0x011c_a660;
/// The text `00586980` returns (`403df0(0x011ca130)`: the word at `+4` of
/// the object, or 0).
const DEFAULT_LOCATION_OBJECT: u32 = 0x011c_a130;
const DEFAULT_LOCATION_TEXT: u32 = 0x0040_3df0;
/// The default land height `00586480` writes (a float in the exe's data).
const RESET_LAND_HEIGHT: u32 = 0x0103_1920;
/// Constants of the grass and ring functions, read from the exe's data: the
/// double 2.0, the double 0.0, the double 100.0 and the double
/// `FLT_MAX` (`3.4028234663852886e38`).
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_HUNDRED: u32 = 0x0101_7a40;
const DOUBLE_FLOAT_MAX: u32 = 0x0102_31b0;
/// The floats `CreateOffsetData` starts the bounds with (`FLT_MAX` and
/// `-FLT_MAX`).
const FLOAT_MAX_VALUE: u32 = 0x0101_6970;
const FLOAT_MIN_VALUE: u32 = 0x0101_5f5c;
/// `NiPoint2` constructor `(x, y)` (`00452dc0`, returns the point), the
/// `OFFSET_DATA` constructor (`00588c50`) and its size, the file's master
/// (`00473c70`: the word the file chain follows, 0 at the end).
const NI_POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const OFFSET_DATA_CONSTRUCT: u32 = 0x0058_8c50;
const OFFSET_DATA_SIZE: u32 = 0x18;
const FILE_MASTER: u32 = 0x0047_3c70;
/// The static check of a record type (`005548a0`, `__cdecl`) that
/// `00588a60` falls back to.
const RECORD_TYPE_CHECK: u32 = 0x0055_48a0;
/// Grass entries: the mesh path format (`data/meshes/%s`, `0x01031f5c`), the
/// float `0053ca40` returns in ST0 (it reads a setting, no `this`) and the name
/// text of a form (`0050a550`, the empty text when there is none).
const MESH_PATH_FORMAT: u32 = 0x0103_1f5c;
const GRASS_SETTING_FLOAT: u32 = 0x0053_ca40;
const FORM_NAME_TEXT: u32 = 0x0050_a550;
/// Messages: invalid cell coordinate (two integers), cell already exists.
const MSG_INVALID_CELL_COORD: u32 = 0x0101_87a0;
const MSG_CELL_EXISTS: u32 = 0x0103_1f6c;
// ---- Callees and data of the third session (00588c50 to 00589420) ----------

/// The `NiTMapBase` family of this unit (0x10 bytes: vtable, bucket count,
/// bucket array, item count). The three instances the world space uses have
/// each a base class vtable and a derived one (slot 0 is the scalar
/// deleting destructor, read from the exe's data):
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int,
/// BSSimpleList<TESObjectREFR *> *>` (`0103200c`, derived `01031fac`),
/// `NiTMapBase<DFALL<NiTMapItem<TESFile *, OFFSET_DATA *> >, TESFile *,
/// OFFSET_DATA *>` (`0103202c`, derived `01031fcc`) and
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int, TESObjectCELL *>`
/// (`0103204c`, derived `01031fec`).
const LIST_MAP_BASE_VTABLE: u32 = 0x0103_200c;
const LIST_MAP_VTABLE: u32 = 0x0103_1fac;
const OFFSET_MAP_BASE_VTABLE: u32 = 0x0103_202c;
const OFFSET_MAP_VTABLE: u32 = 0x0103_1fcc;
const CELL_MAP_BASE_VTABLE: u32 = 0x0103_204c;
const CELL_MAP_VTABLE: u32 = 0x0103_1fec;
/// `NiAlloc(size)` (`00aa1070`, `__cdecl`: the bucket array) and
/// `NiFree(pointer)` (`00aa10f0`, `__cdecl`, null allowed).
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// The probe of a point the region entries are tested against (`004f7070`,
/// `__thiscall(this, x, y)`, 8 bytes; `004f7030` is the same from a
/// pointer), and the test of a region entry for flag `0x40` of `+8`
/// (`00549580`).
const REGION_POINT_BUILD_XY: u32 = 0x004f_7070;
const REGION_ENTRY_FLAG_40: u32 = 0x0054_9580;
/// The world map offset getters of a world space (`__thiscall`, a float in
/// ST0): `fMapScale` (`006ca4e0`, `+0x90`), `fMapOffsetX` (`00644930`,
/// `+0x94`) and `fMapOffsetY` (`008d01e0`, `+0x98`).
const MAP_SCALE: u32 = 0x006c_a4e0;
const MAP_OFFSET_X: u32 = 0x0064_4930;
const MAP_OFFSET_Y: u32 = 0x008d_01e0;
/// `NiPoint3` operations (`__thiscall`, in `multiboundmarkerdata.cpp`'s
/// range): `*= scalar` (`00439180`, returns this), `a - b` into the first
/// stack argument (`00439ef0`) and `a + b` (`00439e90`), both returning that
/// argument.
const POINT3_SCALE: u32 = 0x0043_9180;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_ADD: u32 = 0x0043_9e90;
/// The float `0.5` (`01016248`) and the double `1.0` (`01012070`) of
/// `AdjustMapMarkerCoord`; the double `0.0` is `DOUBLE_ZERO`.
const FLOAT_HALF: u32 = 0x0101_6248;
const DOUBLE_ONE: u32 = 0x0101_2070;

// ---- Helpers ---------------------------------------------------------------

/// `TESForm::GetFile(index)` of a form.
fn form_get_file(e: &mut Engine, form: u32, index: u32) -> Ptr {
    e.call(FORM_GET_FILE, &args![form, index]).ptr()
}

/// `TESFile::GetThreadSafeFile` of the file `GetFile` returned.
fn thread_safe_file(e: &mut Engine, file: Ptr) -> Ptr {
    e.call(FILE_THREAD_SAFE, &args![file]).ptr()
}

/// `_ftol2_sse` on a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`.
fn dynamic_cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// The `OFFSET_DATA` of a file for this world space (`00588a90`).
fn offset_data_of_file(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) -> Ptr<OffsetData> {
    e.call(OFFSET_DATA_OF_FILE, &args![this, file]).ptr()
}

/// The four cell coordinate bounds of an `OFFSET_DATA` (minimum x, minimum
/// y, maximum x, maximum y), each `_ftol2` of the float shifted right by 12,
/// converted in this order.
fn offset_data_bounds(e: &mut Engine, data: Ptr<OffsetData>) -> (i32, i32, i32, i32) {
    let min_x = e.get(data, OffsetData::OffsetMinCoords_x);
    let min_x = float_to_int(e, min_x) >> 12;
    let min_y = e.get(data, OffsetData::OffsetMinCoords_y);
    let min_y = float_to_int(e, min_y) >> 12;
    let max_x = e.get(data, OffsetData::OffsetMaxCoords_x);
    let max_x = float_to_int(e, max_x) >> 12;
    let max_y = e.get(data, OffsetData::OffsetMaxCoords_y);
    let max_y = float_to_int(e, max_y) >> 12;
    (min_x, min_y, max_x, max_y)
}

/// Swaps one 4-byte value of the loaded data in place.
fn swap_dword(e: &mut Engine, at: u32) {
    e.call(SWAP_DWORD, &args![at, 0u32]);
}

/// Swaps one 2-byte value of the loaded data in place.
fn swap_word(e: &mut Engine, at: u32) {
    e.call(SWAP_WORD, &args![at, 0u32]);
}

/// The log call every unresolved form reference makes.
fn log_unresolved(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    id: u32,
    message_named: u32,
    message_id: u32,
) {
    if e.call(EDITOR_ID_LENGTH, &args![this]).u32() != 0 {
        let name = e.vcall(this.addr(), SLOT_EDITOR_ID, &args![]).u32();
        e.call(LOG, &args![message_named, id, name]);
    } else {
        let own_id = e.call(WORD_AT_0C, &args![this]).u32();
        e.call(LOG, &args![message_id, id, own_id]);
    }
}

/// Resolves the form ID held in a field of the world space (`InitItem`):
/// makes it a load order ID (`AddCompileIndex` with the form's last file),
/// finds the form, casts it to `target`, and logs when the cast is null.
fn resolve_form_reference(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    stored_id: u32,
    target: u32,
    message_named: u32,
    message_id: u32,
) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), stored_id);
        let file = form_get_file(e, this.addr(), 0xffff_ffff);
        e.call(FORM_ADD_COMPILE_INDEX, &args![local, file]);
        let id = e.mem.u32(local.addr());
        let form = e.call(FORM_BY_ID, &args![id]).u32();
        let cast = dynamic_cast(e, form, target);
        if cast == 0 {
            log_unresolved(e, this, id, message_named, message_id);
        }
        cast
    })
}

// ---- Functions ---------------------------------------------------------------

// Translated from 004fd380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetParentWorldSimple` (Xbox PDB): the parent world space
/// pointer, whatever the parent-use flags say.
pub fn tes_world_space_get_parent_world_simple(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    e.get(this, TESWorldSpace::pParentWorld)
}

// Translated from 00583360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::LoadPartial` (Xbox PDB, vtable slot `+0x24`): reads the
/// `NAM0` / `NAM9` chunks, which hold the south-west and north-east corners
/// of the world. The corner of this world space is widened to take in the
/// file's (minimum / maximum per axis), and for a master file the file's own
/// corners are kept in its `OFFSET_DATA`. Always true.
pub fn tes_world_space_load_partial(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) -> bool {
    loop {
        let chunk = e.call(FILE_NEXT_CHUNK, &args![file]).u32();
        if chunk == 0 {
            return true;
        }
        if chunk == CHUNK_MIN_COORDS || chunk == CHUNK_MAX_COORDS {
            load_corner_chunk(e, this, file, chunk);
        }
        if !e.call(FILE_ADVANCE_CHUNK, &args![file]).bool() {
            return true;
        }
    }
}

// Translated from 00583560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::Load` (Xbox PDB, vtable slot `+0x20`): reads a world
/// space record (type `'A'`) from the plugin `file`, chunk by chunk.
///
/// - For a master file the `OFFSET_DATA` of the file gets the record's
///   position (`iFileOffset`), and `NAM0`, `NAM9` (see `LoadPartial`) and
///   `OFST` (the table of cell record offsets, byte-swapped when the file
///   is) fill the rest.
/// - Form references (`CNAM`, `INAM`, `NAM2`, `NAM3`, `WNAM`, `ZNAM`, `XEZN`)
///   are stored through the setters as form IDs; `InitItem` resolves them
///   later.
/// - When the record leaves the cell map empty and `cFlags` bit 0 is clear,
///   the map is replaced by a new one (7001 buckets when `00484e40` says so,
///   else 701).
///
/// Returns false for a record of another type, true otherwise.
pub fn fn_00583560(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) -> bool {
    let record_type = e.call(FILE_RECORD_TYPE, &args![file]).u8() as u32;
    if record_type != RECORD_TYPE_WORLD_SPACE {
        return false;
    }
    if e.call(FILE_IS_MASTER, &args![file]).bool() {
        let data = e
            .call(CREATE_OFFSET_DATA, &args![this, file])
            .ptr::<OffsetData>();
        let offset = e.call(FILE_CURRENT_OFFSET, &args![file]).u32();
        e.set(data, OffsetData::iFileOffset, offset);
    }
    e.call(FORM_LOAD, &args![this, file]);
    e.call(FORM_SET_STATE, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_NEXT_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        load_chunk(e, this, file, chunk);
        if !e.call(FILE_ADVANCE_CHUNK, &args![file]).bool() {
            break;
        }
    }
    // An empty cell map (and a world space that keeps cells) is replaced.
    let map = e.get(this, TESWorldSpace::pCellMap);
    let count = e.call(WORD_AT_0C, &args![map]).u32();
    if count == 0 && !fn_005861b0(e, this) {
        let map = e.get(this, TESWorldSpace::pCellMap);
        if !map.is_null() {
            e.vcall(map.addr(), 0, &args![1u32]);
        }
        let buckets = if e.call(FORM_QUERY_LARGE_MAP, &args![this]).bool() {
            LARGE_CELL_MAP_BUCKETS
        } else {
            SMALL_CELL_MAP_BUCKETS
        };
        let block = e.call(MEMORY_ALLOC, &args![CELL_MAP_SIZE]).u32();
        let new_map = if block == 0 {
            0
        } else {
            e.call(CELL_MAP_CONSTRUCT, &args![block, buckets]).u32()
        };
        e.set(this, TESWorldSpace::pCellMap, Ptr::new(new_map));
    }
    true
}

/// One chunk of the world space record (`Load`'s switch).
fn load_chunk(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr, chunk: u32) {
    match chunk {
        CHUNK_CLIMATE => {
            let value = read_chunk_word(e, file);
            e.call(SET_CLIMATE, &args![this, value]);
        }
        CHUNK_DATA => {
            if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 4 {
                let value = read_chunk_word(e, file);
                e.set(this, TESWorldSpace::cFlags, value as u8);
            } else {
                let at = this.addr() + 0x4C;
                e.call(FILE_READ_CHUNK, &args![file, at, 1u32]);
            }
        }
        CHUNK_LOD_WATER => {
            let value = read_chunk_word(e, file);
            e.call(SET_LOD_WATER, &args![this, value]);
        }
        CHUNK_MIN_COORDS | CHUNK_MAX_COORDS => load_corner_chunk(e, this, file, chunk),
        CHUNK_WATER => {
            let value = read_chunk_word(e, file);
            e.call(SET_WATER, &args![this, value]);
        }
        CHUNK_LOD_WATER_HEIGHT => {
            let bits = read_chunk_word(e, file);
            fn_00584150(e, this, f32::from_bits(bits));
        }
        CHUNK_IMPACT_FIRST | CHUNK_IMPACT_SECOND => {
            if e.get(this, TESWorldSpace::pImpactSwap).is_null() {
                let block = e.call(MEMORY_ALLOC, &args![IMPACT_SWAP_SIZE]).u32();
                let made = if block == 0 {
                    0
                } else {
                    e.call(IMPACT_SWAP_CONSTRUCT, &args![block]).u32()
                };
                e.set(this, TESWorldSpace::pImpactSwap, Ptr::new(made));
            }
            let swap = e.get(this, TESWorldSpace::pImpactSwap);
            e.call(IMPACT_SWAP_LOAD, &args![swap, file]);
        }
        CHUNK_EDITOR_ID => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            // The game allocates the chunk size on its stack (rounded up to
            // 16 bytes) and reads with a limit of 0x200.
            e.with_stack((size + 15) & !15, |e, buffer| {
                e.call(FILE_READ_CHUNK, &args![file, buffer, 0x200u32]);
                e.vcall(this.addr(), SLOT_SET_EDITOR_ID, &args![buffer]);
            });
        }
        CHUNK_OBJECT_BOUNDS => {
            e.vcall(this.addr(), 0xE0, &args![file]);
        }
        CHUNK_ICON => {
            let texture = this.addr() + 0x24;
            e.call(TEXTURE_LOAD_CHUNK, &args![texture, file]);
        }
        CHUNK_FULL_NAME => {
            let name = this.addr() + 0x18;
            e.call(FULL_NAME_LOAD, &args![name, file]);
        }
        CHUNK_ENCOUNTER_ZONE => {
            let value = read_chunk_word(e, file);
            e.set(this, TESWorldSpace::pEncounterZone, Ptr::new(value));
        }
        CHUNK_OFFSETS => load_offsets_chunk(e, this, file),
        CHUNK_DEFAULT_HEIGHTS => {
            e.with_stack(8, |e, block| {
                e.call(FILE_READ_CHUNK, &args![file, block, 8u32]);
                if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                    e.call(SWAP_PAIR, &args![block]);
                }
                let land = e.mem.f32(block.addr());
                e.set(this, TESWorldSpace::fDefaultLandHeight, land);
                let water = e.mem.f32(block.addr() + 4);
                e.set(this, TESWorldSpace::fDefaultWaterHeight, water);
            });
        }
        CHUNK_IMAGE_SPACE => {
            let value = read_chunk_word(e, file);
            e.call(SET_IMAGE_SPACE, &args![this, value]);
        }
        CHUNK_MAP_DATA => {
            let at = this.addr() + 0x80;
            e.call(FILE_READ_CHUNK, &args![file, at, 0x10u32]);
            if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                fn_005840b0(e, Ptr::new(at));
            }
        }
        CHUNK_CANOPY_TEXTURE => load_texture_name(e, this.addr() + 0xD4, file),
        CHUNK_MAP_OFFSET_DATA => {
            let at = this.addr() + 0x90;
            e.call(FILE_READ_CHUNK, &args![file, at, 0xCu32]);
            if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_MAP_OFFSET_BLOCK, &args![at]);
            }
        }
        CHUNK_PARENT_USE_FLAGS => {
            let at = this.addr() + 0x4E;
            e.call(FILE_READ_CHUNK_U16, &args![file, at]);
        }
        CHUNK_SKIPPED => {
            read_chunk_word(e, file);
        }
        CHUNK_PARENT_WORLD => {
            let value = read_chunk_word(e, file);
            e.call(SET_PARENT_WORLD, &args![this, value]);
            e.set(this, TESWorldSpace::sParentUseFlags, 0xffff);
        }
        CHUNK_WATER_NOISE_TEXTURE => load_texture_name(e, this.addr() + 0xE0, file),
        CHUNK_MUSIC_TYPE => {
            let value = read_chunk_word(e, file);
            e.call(SET_MUSIC_TYPE, &args![this, value]);
        }
        _ => {}
    }
}

/// `GetChunkData(&word)` into a zeroed local.
fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, local| {
        e.call(FILE_READ_CHUNK_U32, &args![file, local]);
        e.mem.u32(local.addr())
    })
}

/// `NAM0` / `NAM9` inside `Load` (the same work as `LoadPartial`).
fn load_corner_chunk(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr, chunk: u32) {
    let is_min = chunk == CHUNK_MIN_COORDS;
    e.with_stack(8, |e, corner| {
        e.call(LOCAL_STRUCT_CONSTRUCT, &args![corner]);
        e.call(FILE_READ_CHUNK, &args![file, corner, 8u32]);
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            swap_dword(e, corner.addr());
            swap_dword(e, corner.addr() + 4);
        }
        let (x_field, y_field, combine) = if is_min {
            (
                TESWorldSpace::MinimumCoords_x,
                TESWorldSpace::MinimumCoords_y,
                FLOAT_MIN,
            )
        } else {
            (
                TESWorldSpace::MaximumCoords_x,
                TESWorldSpace::MaximumCoords_y,
                FLOAT_MAX,
            )
        };
        let own = e.get(this, x_field);
        let read = e.mem.f32(corner.addr());
        let widened = e.call(combine, &args![own, read]).f32();
        e.set(this, x_field, widened);
        let own = e.get(this, y_field);
        let read = e.mem.f32(corner.addr() + 4);
        let widened = e.call(combine, &args![own, read]).f32();
        e.set(this, y_field, widened);
        if e.call(FILE_IS_MASTER, &args![file]).bool() {
            let data = e.call(CREATE_OFFSET_DATA, &args![this, file]).u32();
            let (x_at, y_at) = if is_min { (4, 8) } else { (0xC, 0x10) };
            let x_bits = e.mem.u32(corner.addr());
            let y_bits = e.mem.u32(corner.addr() + 4);
            e.mem.set_u32(data + x_at, x_bits);
            e.mem.set_u32(data + y_at, y_bits);
        }
    });
}

/// `OFST` inside `Load`: the table of cell record offsets of this file.
fn load_offsets_chunk(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    if size == 0 {
        return;
    }
    let count = size >> 2;
    let data = e
        .call(CREATE_OFFSET_DATA, &args![this, file])
        .ptr::<OffsetData>();
    let old = e.get(data, OffsetData::pCellFileOffsets);
    if !old.is_null() {
        e.call(MEMORY_FREE, &args![old]);
    }
    // `new u32[count]`: the byte count saturates when it overflows 32 bits.
    let bytes = (count as u64 * 4).min(u32::MAX as u64) as u32;
    let table = e.call(MEMORY_ALLOC, &args![bytes]).ptr();
    e.set(data, OffsetData::pCellFileOffsets, table);
    e.call(FILE_READ_CHUNK, &args![file, table, 0u32]);
    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
        for i in 0..count {
            swap_dword(e, table.addr() + i * 4);
        }
    }
}

/// `NNAM` / `XNAM` inside `Load`: a texture path read into a stack buffer
/// and set on the `TESTexture` at `texture`.
fn load_texture_name(e: &mut Engine, texture: u32, file: Ptr) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    if size != 0 {
        e.with_stack((size + 15) & !15, |e, buffer| {
            e.call(FILE_READ_CHUNK, &args![file, buffer, size]);
            e.call(TEXTURE_SET_NAME, &args![texture, buffer]);
        });
    }
}

// Translated from 005840b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps a `WORLD_MAP_DATA` (Xbox PDB) in place: its two 4-byte sizes
/// and four 2-byte cell coordinates.
pub fn fn_005840b0(e: &mut Engine, this: Ptr) {
    swap_dword(e, this.addr());
    swap_dword(e, this.addr() + 4);
    swap_word(e, this.addr() + 8);
    swap_word(e, this.addr() + 0xA);
    swap_word(e, this.addr() + 0xC);
    swap_word(e, this.addr() + 0xE);
}

// Translated from 00584150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fLODWaterHeight` (Xbox PDB).
pub fn fn_00584150(e: &mut Engine, this: Ptr<TESWorldSpace>, height: f32) {
    e.set(this, TESWorldSpace::fLODWaterHeight, x87_quiet(height));
}

// Translated from 00584170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::Save` (Xbox PDB, vtable slot `+0x2c`): writes the world
/// space record's chunks. The parent-world values (climate, image space,
/// lod water and its height, the default heights, the world map, the music
/// type) are only written when the world space does not take them from its
/// parent (`00586340`).
pub fn fn_00584170(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    e.call(FORM_START, &args![this]);
    e.call(FULL_NAME_SAVE, &args![this.addr() + 0x18]);
    if !e.get(this, TESWorldSpace::pEncounterZone).is_null() {
        let zone = e.get(this, TESWorldSpace::pEncounterZone);
        let id = e.call(WORD_AT_0C, &args![zone]).u32();
        e.call(ADD_CHUNK_U32, &args![CHUNK_ENCOUNTER_ZONE, id]);
    }
    if !e.get(this, TESWorldSpace::pParentWorld).is_null() {
        let parent = e.get(this, TESWorldSpace::pParentWorld);
        let id = e.call(WORD_AT_0C, &args![parent]).u32();
        e.call(ADD_CHUNK_U32, &args![CHUNK_PARENT_WORLD, id]);
        let flags = e.get(this, TESWorldSpace::sParentUseFlags) as u32;
        e.call(ADD_CHUNK_U16, &args![CHUNK_PARENT_USE_FLAGS, flags]);
    }
    if !fn_00586340(e, this, 4) {
        let climate = fn_00585fe0(e, this);
        if !climate.is_null() {
            let climate = fn_00585fe0(e, this);
            let id = e.call(WORD_AT_0C, &args![climate]).u32();
            e.call(ADD_CHUNK_U32, &args![CHUNK_CLIMATE, id]);
        }
    }
    if !fn_00586340(e, this, 3) {
        let water = fn_00586070(e, this);
        if !water.is_null() {
            let water = fn_00586070(e, this);
            let id = e.call(WORD_AT_0C, &args![water]).u32();
            e.call(ADD_CHUNK_U32, &args![CHUNK_WATER, id]);
        }
    }
    if !fn_00586340(e, this, 1) {
        let lod_water = fn_005860c0(e, this);
        if !lod_water.is_null() {
            let lod_water = fn_005860c0(e, this);
            let id = e.call(WORD_AT_0C, &args![lod_water]).u32();
            e.call(ADD_CHUNK_U32, &args![CHUNK_LOD_WATER, id]);
            let height = e.call(GET_LOD_WATER_HEIGHT, &args![this]).f32();
            e.call(ADD_CHUNK_U32, &args![CHUNK_LOD_WATER_HEIGHT, height]);
        }
    }
    if !fn_00586340(e, this, 0) {
        e.with_stack(8, |e, block| {
            let land = e.get(this, TESWorldSpace::fDefaultLandHeight);
            e.mem.set_f32(block.addr(), land);
            let water = e.get(this, TESWorldSpace::fDefaultWaterHeight);
            e.mem.set_f32(block.addr() + 4, water);
            if e.call(SAVING_SWAPPED, &args![]).bool() {
                e.call(SWAP_PAIR, &args![block]);
            }
            e.call(ADD_CHUNK_DATA, &args![CHUNK_DEFAULT_HEIGHTS, block, 8u32]);
            if e.call(SAVING_SWAPPED, &args![]).bool() {
                e.call(SWAP_PAIR, &args![block]);
            }
        });
    }
    if !fn_00586340(e, this, 2) {
        e.call(TEXTURE_SAVE, &args![this.addr() + 0x24, CHUNK_ICON]);
        if e.call(SAVING_SWAPPED, &args![]).bool() {
            fn_005840b0(e, Ptr::new(this.addr() + 0x80));
        }
        e.call(
            ADD_CHUNK_DATA,
            &args![CHUNK_MAP_DATA, this.addr() + 0x80, 0x10u32],
        );
        if e.call(SAVING_SWAPPED, &args![]).bool() {
            fn_005840b0(e, Ptr::new(this.addr() + 0x80));
        }
    }
    // The map offset block is written whatever the parent flags say.
    if e.call(SAVING_SWAPPED, &args![]).bool() {
        e.call(SWAP_MAP_OFFSET_BLOCK, &args![this.addr() + 0x90]);
    }
    e.call(
        ADD_CHUNK_DATA,
        &args![CHUNK_MAP_OFFSET_DATA, this.addr() + 0x90, 0xCu32],
    );
    if e.call(SAVING_SWAPPED, &args![]).bool() {
        e.call(SWAP_MAP_OFFSET_BLOCK, &args![this.addr() + 0x90]);
    }
    if !fn_00586340(e, this, 5) {
        let image_space = e.call(GET_IMAGE_SPACE_RAW, &args![this]).u32();
        if image_space != 0 {
            let image_space = e.call(GET_IMAGE_SPACE_RAW, &args![this]).u32();
            let id = e.call(WORD_AT_0C, &args![image_space]).u32();
            e.call(ADD_CHUNK_U32, &args![CHUNK_IMAGE_SPACE, id]);
        }
    }
    let flags = e.get(this, TESWorldSpace::cFlags) as u32;
    e.call(ADD_CHUNK_U8, &args![CHUNK_DATA, flags]);
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_MIN_COORDS, this.addr() + 0xA0, 2u32],
    );
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_MAX_COORDS, this.addr() + 0xA8, 2u32],
    );
    if !fn_00586150(e, this).is_null() {
        let music = fn_00586150(e, this);
        let id = e.call(WORD_AT_0C, &args![music]).u32();
        e.call(ADD_CHUNK_U32, &args![CHUNK_MUSIC_TYPE, id]);
    }
    save_texture_name(e, this.addr() + 0xD4, CHUNK_CANOPY_TEXTURE);
    save_texture_name(e, this.addr() + 0xE0, CHUNK_WATER_NOISE_TEXTURE);
    let swap = e.get(this, TESWorldSpace::pImpactSwap);
    if !swap.is_null() {
        e.call(IMPACT_SWAP_SAVE, &args![swap]);
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// A texture path chunk: the text and its length plus the terminating zero.
/// (The decompiler hangs the length on the wrong call: `004856f0` takes the
/// tag, the text and the size.)
fn save_texture_name(e: &mut Engine, texture: u32, chunk: u32) {
    let length = e.call(TEXTURE_NAME_LENGTH, &args![texture]).u32();
    let text = e.call(TEXTURE_NAME_TEXT, &args![texture]).u32();
    e.call(ADD_CHUNK_BLOCK, &args![chunk, text, length + 1]);
}

// Translated from 00584530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index into a file's cell offset table of the exterior cell
/// (`x`, `y`), or -1 when the file has no table for this world space or the
/// cell is outside its grid.
pub fn fn_00584530(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr, x: i32, y: i32) -> i32 {
    let data = offset_data_of_file(e, this, file);
    if data.is_null() {
        return -1;
    }
    let (min_x, min_y, max_x, max_y) = offset_data_bounds(e, data);
    if x > max_x || x < min_x || y > max_y || y < min_y {
        return -1;
    }
    max_x
        .wrapping_sub(min_x)
        .wrapping_add(1)
        .wrapping_mul(y.wrapping_sub(min_y))
        .wrapping_add(x)
        .wrapping_sub(min_x)
}

// Translated from 005845e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::InitItem` (Xbox PDB, vtable slot `+0x88`): once per form
/// (form flag 8 clear), turns the form IDs the record loaded into the
/// climate, image space, encounter zone, water, lod water, music type and
/// parent world space they name (each through its setter, logging when the
/// form is missing), runs the impact swap's `InitItem` and sets the form
/// state. Then every cell of the cell map and the persistent cell get their
/// own `InitItem`, and the terrain manager is created.
pub fn tes_world_space_init_item(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    if !e.call(FORM_FLAG_8, &args![this]).bool() {
        let climate = e.get(this, TESWorldSpace::pClimate).addr();
        if climate != 0 {
            let form = resolve_form_reference(
                e,
                this,
                climate,
                TYPE_TES_CLIMATE,
                MSG_CLIMATE_NAMED,
                MSG_CLIMATE_ID,
            );
            e.call(SET_CLIMATE, &args![this, form]);
        }
        let image_space = e.get(this, TESWorldSpace::pImageSpace).addr();
        if image_space != 0 {
            let form = resolve_form_reference(
                e,
                this,
                image_space,
                TYPE_TES_IMAGE_SPACE,
                MSG_IMAGE_SPACE_NAMED,
                MSG_IMAGE_SPACE_ID,
            );
            e.call(SET_IMAGE_SPACE, &args![this, form]);
        }
        let zone = e.get(this, TESWorldSpace::pEncounterZone).addr();
        if zone != 0 {
            let form = resolve_form_reference(
                e,
                this,
                zone,
                TYPE_BGS_ENCOUNTER_ZONE,
                MSG_ENCOUNTER_ZONE_NAMED,
                MSG_ENCOUNTER_ZONE_ID,
            );
            fn_00584bc0(e, this, Ptr::new(form));
        }
        let water = e.get(this, TESWorldSpace::pWorldWater).addr();
        if water != 0 {
            let form = resolve_form_reference(
                e,
                this,
                water,
                TYPE_TES_WATER_FORM,
                MSG_WATER_NAMED,
                MSG_WATER_ID,
            );
            e.call(SET_WATER, &args![this, form]);
        }
        let lod_water = e.get(this, TESWorldSpace::pLODWater).addr();
        if lod_water != 0 {
            let form = resolve_form_reference(
                e,
                this,
                lod_water,
                TYPE_TES_WATER_FORM,
                MSG_LOD_WATER_NAMED,
                MSG_LOD_WATER_ID,
            );
            e.call(SET_LOD_WATER, &args![this, form]);
        }
        let music = e.get(this, TESWorldSpace::pMusicType).addr();
        if music != 0 {
            let form = resolve_form_reference(
                e,
                this,
                music,
                TYPE_BGS_MUSIC_TYPE,
                MSG_MUSIC_NAMED,
                MSG_MUSIC_ID,
            );
            e.call(SET_MUSIC_TYPE, &args![this, form]);
        }
        let parent = e.get(this, TESWorldSpace::pParentWorld).addr();
        if parent != 0 {
            let form = resolve_form_reference(
                e,
                this,
                parent,
                TYPE_TES_WORLD_SPACE,
                MSG_PARENT_NAMED,
                MSG_PARENT_ID,
            );
            e.call(SET_PARENT_WORLD, &args![this, form]);
        }
        let swap = e.get(this, TESWorldSpace::pImpactSwap);
        if !swap.is_null() {
            e.call(IMPACT_SWAP_INIT_ITEM, &args![swap, this]);
        }
        e.call(FORM_SET_STATE, &args![this, 1u32]);
    }
    // Every cell of the map is initialized too (virtual slot +0x88).
    let map = e.get(this, TESWorldSpace::pCellMap);
    let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
    while position != 0 {
        let (next, _key, cell) = map_get_next(e, map.addr(), position);
        position = next;
        if cell != 0 {
            e.vcall(cell, 0x88, &args![]);
        }
    }
    let persistent = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if persistent != 0 {
        e.vcall(persistent, 0x88, &args![]);
    }
    let block = e.call(MEMORY_ALLOC, &args![TERRAIN_MANAGER_SIZE]).u32();
    let manager = if block == 0 {
        0
    } else {
        e.call(TERRAIN_MANAGER_CONSTRUCT, &args![block, this]).u32()
    };
    e.set(this, TESWorldSpace::pTerrainManager, Ptr::new(manager));
}

/// `NiTMapBase::GetNext(&position, &key, &value)` (`006b7f20`): the next
/// position, the key and the value, all three kept in game memory by the
/// caller.
fn map_get_next(e: &mut Engine, map: u32, position: u32) -> (u32, u32, u32) {
    e.with_stack(12, |e, locals| {
        let at = locals.addr();
        e.mem.set_u32(at, position);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0);
        e.call(MAP_GET_NEXT, &args![map, at, at + 4, at + 8]);
        (e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8))
    })
}

// Translated from 00584bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pEncounterZone` (Xbox PDB).
pub fn fn_00584bc0(e: &mut Engine, this: Ptr<TESWorldSpace>, zone: Ptr) {
    e.set(this, TESWorldSpace::pEncounterZone, zone);
}

// Translated from 00584be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::SavesBefore` (Xbox PDB, vtable slot `+0x3c`) for a form:
/// whether this world space is saved before `form`. A cell is ordered by
/// its world space, a placed reference (the reference types
/// `0x3a` to `0x40`, `0x42`, `0x43`, `0x69`) by what its `TESChildCell`
/// part's first virtual function returns, another world space (`0x41`) by
/// form ID; everything else by the base class's order (`00484020`).
pub fn fn_00584be0(e: &mut Engine, this: Ptr<TESWorldSpace>, form: Ptr) -> bool {
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    match form_type {
        RECORD_TYPE_CELL => {
            let cell = dynamic_cast(e, form.addr(), TYPE_TES_OBJECT_CELL);
            let world = if cell == 0 {
                0
            } else {
                e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32()
            };
            if world == this.addr() {
                true
            } else if world != 0 {
                e.vcall(this.addr(), 0x3C, &args![world]).bool()
            } else {
                false
            }
        }
        0x3a..=0x40 | 0x42 | 0x43 | 0x69 => {
            let child = dynamic_cast(e, form.addr(), TYPE_TES_CHILD_CELL);
            let owner = e.vcall(child, 0, &args![]).u32();
            e.vcall(this.addr(), 0x3C, &args![owner]).bool()
        }
        0x41 => {
            let own_id = e.call(WORD_AT_0C, &args![this]).u32();
            let other_id = e.call(WORD_AT_0C, &args![form]).u32();
            own_id < other_id
        }
        _ => e.call(FORM_SAVES_BEFORE_FORM, &args![this, form]).bool(),
    }
}

// Translated from 00584d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::SavesBefore` (Xbox PDB, vtable slot `+0x38`) for a
/// `GRUP` header: group type 0 is the base class's decision (`00484150`),
/// types 1 and 6 (the groups labelled with a world space or a cell) order by
/// the labelled form (through slot `+0x3c`), 7 sorts before, 2 and 3 after,
/// every other type and any record that is not a group header: no.
pub fn fn_00584d40(e: &mut Engine, this: Ptr<TESWorldSpace>, header: Ptr) -> bool {
    if header.is_null() || e.mem.u32(header.addr()) != e.global::<u32>(GROUP_TAG) {
        return false;
    }
    let group_type = e.mem.u32(header.addr() + 0xC);
    let label = e.mem.u32(header.addr() + 8);
    match group_type {
        0 => e
            .call(FORM_SAVES_BEFORE_HEADER, &args![this, header])
            .bool(),
        1 | 6 => {
            let target = if group_type == 1 {
                TYPE_TES_WORLD_SPACE
            } else {
                TYPE_TES_OBJECT_CELL
            };
            let form = e.call(FORM_BY_ID, &args![label]).u32();
            let cast = dynamic_cast(e, form, target);
            if cast == 0 {
                false
            } else {
                e.vcall(this.addr(), 0x3C, &args![cast]).bool()
            }
        }
        7 => true,
        _ => false,
    }
}

// Translated from 00584e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::CreateDuplicateForm` (Xbox PDB, vtable slot `+0x40`):
/// duplicates the world space through the base class, clears the copy's cell
/// map and fills it with duplicates of this world space's cells (each marked
/// altered, slot `+0xc8`), takes a duplicate of the persistent cell, and
/// finishes the copy (`00588120`). Returns the copy.
pub fn fn_00584e60(e: &mut Engine, this: Ptr<TESWorldSpace>, flag: u8, other: Ptr) -> Ptr {
    let duplicate = e
        .call(FORM_CREATE_DUPLICATE, &args![this, flag, other])
        .u32();
    let copy = Ptr::<TESWorldSpace>::new(dynamic_cast(e, duplicate, TYPE_TES_WORLD_SPACE));
    let copy_map = e.mem.u32(copy.addr() + 0x30);
    e.call(MAP_REMOVE_ALL, &args![copy_map]);
    let map = e.get(this, TESWorldSpace::pCellMap);
    let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
    while position != 0 {
        let (next, _key, cell) = map_get_next(e, map.addr(), position);
        position = next;
        if cell != 0 {
            let cell_copy = e.vcall(cell, 0x40, &args![0u32, other]).u32();
            let cell_copy = dynamic_cast(e, cell_copy, TYPE_TES_OBJECT_CELL);
            if cell_copy != 0 {
                e.vcall(cell_copy, 0xC8, &args![1u32]);
                e.call(ADD_CELL, &args![copy, cell_copy]);
            }
        }
    }
    let persistent = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if persistent != 0 {
        let cell_copy = e.vcall(persistent, 0x40, &args![0u32, other]).u32();
        let cell_copy = dynamic_cast(e, cell_copy, TYPE_TES_OBJECT_CELL);
        if cell_copy != 0 {
            e.set(copy, TESWorldSpace::pPersistentCell, Ptr::new(cell_copy));
            e.vcall(cell_copy, 0xC8, &args![1u32]);
        }
    }
    e.call(FINISH_DUPLICATE, &args![copy]);
    copy.cast()
}

// Translated from 00584fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::Copy` (Xbox PDB, vtable slot `+0x108`): copies the
/// world space `source` (a form cast to a world space; nothing happens when
/// it is not one) over this one: the components, flags, parent, encounter
/// zone, climate, image space, impact swap (created, copied or deleted to
/// match), water, lod water and its height, default heights, map data, map
/// offset data, music type, and the cell map (cleared, then every entry of
/// the source's is set), then the two textures' own copies.
pub fn fn_00584fa0(e: &mut Engine, this: Ptr<TESWorldSpace>, source: Ptr) {
    let source = Ptr::<TESWorldSpace>::new(dynamic_cast(e, source.addr(), TYPE_TES_WORLD_SPACE));
    if source.is_null() {
        return;
    }
    e.call(FORM_COPY_COMPONENTS, &args![this, source]);
    let flags = e.get(source, TESWorldSpace::cFlags);
    e.set(this, TESWorldSpace::cFlags, flags);
    let parent_flags = e.get(source, TESWorldSpace::sParentUseFlags);
    e.set(this, TESWorldSpace::sParentUseFlags, parent_flags);
    let parent = e.get(source, TESWorldSpace::pParentWorld);
    e.set(this, TESWorldSpace::pParentWorld, parent);
    let zone = e.get(source, TESWorldSpace::pEncounterZone);
    e.set(this, TESWorldSpace::pEncounterZone, zone);
    let climate = fn_00585fe0(e, source);
    e.set(this, TESWorldSpace::pClimate, climate);
    let image_space = e.call(GET_IMAGE_SPACE_RAW, &args![source]).ptr();
    e.set(this, TESWorldSpace::pImageSpace, image_space);
    let source_swap = e.get(source, TESWorldSpace::pImpactSwap);
    if !source_swap.is_null() {
        if e.get(this, TESWorldSpace::pImpactSwap).is_null() {
            let block = e.call(MEMORY_ALLOC, &args![IMPACT_SWAP_SIZE]).u32();
            let made = if block == 0 {
                0
            } else {
                e.call(IMPACT_SWAP_CONSTRUCT, &args![block]).u32()
            };
            e.set(this, TESWorldSpace::pImpactSwap, Ptr::new(made));
        }
        let own = e.get(this, TESWorldSpace::pImpactSwap);
        e.call(IMPACT_SWAP_COPY, &args![own, source_swap]);
    } else if !e.get(this, TESWorldSpace::pImpactSwap).is_null() {
        let own = e.get(this, TESWorldSpace::pImpactSwap);
        e.call(IMPACT_SWAP_DESTRUCT, &args![own, 1u32]);
        e.set(this, TESWorldSpace::pImpactSwap, Ptr::NULL);
    }
    let water = fn_00586070(e, source);
    e.call(SET_WATER, &args![this, water]);
    let lod_water = fn_005860c0(e, source);
    e.call(SET_LOD_WATER, &args![this, lod_water]);
    let height = e.call(GET_LOD_WATER_HEIGHT, &args![source]).f32();
    fn_00584150(e, this, height);
    let land = e.get(source, TESWorldSpace::fDefaultLandHeight);
    e.set(this, TESWorldSpace::fDefaultLandHeight, land);
    let water_height = e.get(source, TESWorldSpace::fDefaultWaterHeight);
    e.set(this, TESWorldSpace::fDefaultWaterHeight, water_height);
    e.call(
        MEMCPY,
        &args![this.addr() + 0x80, source.addr() + 0x80, 0x10u32],
    );
    e.call(
        MEMCPY,
        &args![this.addr() + 0x90, source.addr() + 0x90, 0xCu32],
    );
    let music = fn_00586150(e, source);
    e.call(SET_MUSIC_TYPE, &args![this, music]);
    let own_map = e.get(this, TESWorldSpace::pCellMap);
    e.call(MAP_REMOVE_ALL, &args![own_map]);
    let source_map = e.get(source, TESWorldSpace::pCellMap);
    let mut position = e.call(MAP_FIRST_POS, &args![source_map]).u32();
    while position != 0 {
        let (next, key, value) = map_get_next(e, source_map.addr(), position);
        position = next;
        let own_map = e.get(this, TESWorldSpace::pCellMap);
        e.call(MAP_SET_AT, &args![own_map, key, value]);
    }
    // The two textures copy themselves (`CopyComponent`, slot +8).
    e.vcall(this.addr() + 0xD4, 8, &args![source.addr() + 0xD4]);
    e.vcall(this.addr() + 0xE0, 8, &args![source.addr() + 0xE0]);
}

// Translated from 00585250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::Compare` (Xbox PDB, vtable slot `+0x10c`): true when
/// `other` differs from this world space (or is not a world space): the
/// components, flags, parent flags, parent, encounter zone, climate, image
/// space, impact swap, water, lod water and its height, the default
/// heights, the map data and map offset data bytes, the music type, and the
/// two textures' own compares.
pub fn fn_00585250(e: &mut Engine, this: Ptr<TESWorldSpace>, other: Ptr) -> bool {
    let other = Ptr::<TESWorldSpace>::new(dynamic_cast(e, other.addr(), TYPE_TES_WORLD_SPACE));
    if other.is_null() {
        return true;
    }
    if e.call(FORM_COMPARE_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    if e.get(this, TESWorldSpace::cFlags) != e.get(other, TESWorldSpace::cFlags) {
        return true;
    }
    if e.get(this, TESWorldSpace::sParentUseFlags) != e.get(other, TESWorldSpace::sParentUseFlags) {
        return true;
    }
    if e.get(this, TESWorldSpace::pParentWorld) != e.get(other, TESWorldSpace::pParentWorld) {
        return true;
    }
    if e.get(this, TESWorldSpace::pEncounterZone) != e.get(other, TESWorldSpace::pEncounterZone) {
        return true;
    }
    if fn_00585fe0(e, this) != fn_00585fe0(e, other) {
        return true;
    }
    let own_image = e.call(GET_IMAGE_SPACE_RAW, &args![this]).u32();
    let other_image = e.call(GET_IMAGE_SPACE_RAW, &args![other]).u32();
    if own_image != other_image {
        return true;
    }
    let own_swap = e.get(this, TESWorldSpace::pImpactSwap);
    let other_swap = e.get(other, TESWorldSpace::pImpactSwap);
    if !own_swap.is_null() && !other_swap.is_null() {
        if e.call(IMPACT_SWAP_COMPARE, &args![own_swap, other_swap])
            .bool()
        {
            return true;
        }
    } else if !own_swap.is_null() || !other_swap.is_null() {
        return true;
    }
    if fn_00586070(e, this) != fn_00586070(e, other) {
        return true;
    }
    if fn_005860c0(e, this) != fn_005860c0(e, other) {
        return true;
    }
    let own_height = e.call(GET_LOD_WATER_HEIGHT, &args![this]).f64();
    let other_height = e.call(GET_LOD_WATER_HEIGHT, &args![other]).f64();
    if own_height != other_height {
        return true;
    }
    if e.get(this, TESWorldSpace::fDefaultLandHeight)
        != e.get(other, TESWorldSpace::fDefaultLandHeight)
        || e.get(this, TESWorldSpace::fDefaultWaterHeight)
            != e.get(other, TESWorldSpace::fDefaultWaterHeight)
    {
        return true;
    }
    if e.call(
        MEMCMP,
        &args![this.addr() + 0x80, other.addr() + 0x80, 0x10u32],
    )
    .i32()
        != 0
    {
        return true;
    }
    if e.call(
        MEMCMP,
        &args![this.addr() + 0x90, other.addr() + 0x90, 0xCu32],
    )
    .i32()
        != 0
    {
        return true;
    }
    if fn_00586150(e, this) != fn_00586150(e, other) {
        return true;
    }
    if e.vcall(this.addr() + 0xD4, 0xC, &args![other.addr() + 0xD4])
        .bool()
    {
        return true;
    }
    e.vcall(this.addr() + 0xE0, 0xC, &args![other.addr() + 0xE0])
        .bool()
}

// Translated from 005854f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::FindCellInFile` (Xbox PDB): positions `file` on the
/// exterior cell record (`x`, `y`) of this world space. A master file with
/// an `OFFSET_DATA` table finds it by index (`fn_00584530`); any other file
/// is scanned record by record from this world space's record: the
/// `GRUP`s the scan passes are entered or skipped by group type and label
/// (cell block and sub-block keys), cell records are matched on their
/// `XCLC` grid position (an interior cell, flag 0x400, never matches). The
/// reader is left on the cell record when found.
pub fn tes_world_space_find_cell_in_file(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    file: Ptr,
    x: i32,
    y: i32,
) -> bool {
    if !file.is_null() && e.call(FILE_IS_MASTER, &args![file]).bool() {
        let data = offset_data_of_file(e, this, file);
        if !data.is_null() && !e.get(data, OffsetData::pCellFileOffsets).is_null() {
            let index = fn_00584530(e, this, file, x, y);
            if index == -1 {
                return false;
            }
            let table = e.get(data, OffsetData::pCellFileOffsets).addr();
            let offset = e
                .mem
                .u32(table.wrapping_add((index as u32).wrapping_mul(4)));
            if offset == 0 {
                return false;
            }
            let base = e.get(data, OffsetData::iFileOffset);
            e.call(FILE_SET_OFFSET, &args![file, offset.wrapping_add(base)]);
            return true;
        }
    }
    let mut result = false;
    let block_key = e.call(CALC_BLOCK_KEY, &args![x, y]).u32();
    let sub_block_key = e.call(CALC_SUB_BLOCK_KEY, &args![x, y]).u32();
    if !file.is_null() && e.call(FILE_FIND_FORM, &args![file, this]).bool() {
        e.call(FILE_NEXT_FORM, &args![file, 1u32]);
        let mut done = false;
        let record = e.call(FILE_CURRENT_RECORD, &args![file]).u32();
        while record != 0 && !done {
            let record_tag = e.mem.u32(record);
            if record_tag == e.global::<u32>(GROUP_TAG) {
                let mut skip = true;
                done = true;
                let group_type = e.mem.u32(record + 0xC);
                let label = e.mem.u32(record + 8);
                match group_type {
                    1 => {
                        skip = false;
                        done = false;
                    }
                    4 => {
                        if block_key == label || sub_block_key == label {
                            skip = false;
                        }
                        done = false;
                    }
                    5 => {
                        if sub_block_key == label {
                            skip = false;
                        }
                        done = false;
                    }
                    6 | 8 | 9 => done = false,
                    _ => {}
                }
                if !done {
                    if !skip {
                        e.call(FILE_NEXT_FORM, &args![file, 1u32]);
                    } else {
                        e.call(FILE_LEAVE_GROUP, &args![file]);
                    }
                }
            } else if record_tag == e.global::<u32>(CELL_TAG) {
                let matched = scan_cell_record(e, file, record, x, y);
                if matched {
                    result = true;
                    done = true;
                    e.call(FILE_REWIND_RECORD, &args![file]);
                } else {
                    e.call(FILE_NEXT_FORM, &args![file, 1u32]);
                }
            } else if record_tag == e.global::<u32>(SKIPPED_RECORD_TAG_01187338) {
                e.call(FILE_NEXT_FORM, &args![file, 1u32]);
            } else {
                done = true;
            }
        }
    }
    result
}

/// Whether the cell record under the reader is the exterior cell (`x`,
/// `y`): reads the `XCLC` chunk (three words, the grid x and y and flags)
/// into a zeroed local; an interior cell (record flag 0x400) has the grid x
/// 0x7fffffff.
fn scan_cell_record(e: &mut Engine, file: Ptr, record: u32, x: i32, y: i32) -> bool {
    e.with_stack(12, |e, grid| {
        e.call(CELL_COORDS_CONSTRUCT, &args![grid]);
        e.call(MEMSET, &args![grid, 0u32, 0xCu32]);
        if e.mem.u32(record + 8) & 0x400 != 0 {
            e.mem.set_i32(grid.addr(), 0x7fff_ffff);
        } else {
            let mut found = false;
            let mut chunk = e.call(FILE_NEXT_CHUNK, &args![file]).u32();
            while chunk != 0 && !found {
                if chunk == CHUNK_CELL_GRID {
                    e.call(FILE_READ_CHUNK, &args![file, grid, 0xCu32]);
                    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_PAIR, &args![grid]);
                    }
                    found = true;
                }
                chunk = if e.call(FILE_ADVANCE_CHUNK, &args![file]).bool() {
                    e.call(FILE_NEXT_CHUNK, &args![file]).u32()
                } else {
                    0
                };
            }
        }
        e.mem.i32(grid.addr()) == x && e.mem.i32(grid.addr() + 4) == y
    })
}

// Translated from 005857b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to every non-zero entry of the cell offset table of
/// `file` (when the file has one and `delta` is not 0): the entries'
/// base moved after the file's records were shifted.
pub fn fn_005857b0(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr, delta: u32) {
    let data = offset_data_of_file(e, this, file);
    if data.is_null() || e.get(data, OffsetData::pCellFileOffsets).is_null() || delta == 0 {
        return;
    }
    let (min_x, min_y, max_x, max_y) = offset_data_bounds(e, data);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let index = fn_00584530(e, this, file, x, y);
            if index >= 0 {
                let table = e.get(data, OffsetData::pCellFileOffsets).addr();
                let at = table.wrapping_add((index as u32).wrapping_mul(4));
                let entry = e.mem.u32(at);
                if entry != 0 {
                    e.mem.set_u32(at, entry.wrapping_add(delta));
                }
            }
        }
    }
}

// Translated from 005858b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::FindInFileFast` (Xbox PDB, vtable slot `+0x7c`): whether
/// `file` is a master file whose `OFFSET_DATA` has a record position for this
/// world space that holds a world space record with this form ID.
pub fn fn_005858b0(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) -> bool {
    let data = offset_data_of_file(e, this, file);
    if !file.is_null() && e.call(FILE_IS_MASTER, &args![file]).bool() && !data.is_null() {
        let position = e.get(data, OffsetData::iFileOffset);
        if position != 0 && e.call(FILE_SET_OFFSET, &args![file, position]).bool() {
            let record_type = e.call(FILE_RECORD_TYPE, &args![file]).u32();
            if record_type == e.global::<u8>(WORLD_SPACE_RECORD_TYPE) as u32 {
                let record_id = e.call(FILE_CURRENT_FORM_ID, &args![file]).u32();
                let own_id = e.call(WORD_AT_0C, &args![this]).u32();
                return record_id == own_id;
            }
        }
    }
    false
}

// Translated from 00585940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetExtCellDataFromFileByEditorID` (Xbox PDB): looks for
/// the exterior cell whose editor ID is `editor_id` in the cell offset
/// tables of this form's files, storing its grid position through `out_x`
/// and `out_y`.
pub fn tes_world_space_get_ext_cell_data_from_file_by_editor_id(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    editor_id: Ptr,
    out_x: Ptr,
    out_y: Ptr,
) -> bool {
    let count = e.call(FORM_FILE_COUNT, &args![this]).u32();
    for index in 0..count {
        let file = form_get_file(e, this.addr(), index);
        let file = thread_safe_file(e, file);
        let data = offset_data_of_file(e, this, file);
        if data.is_null() {
            continue;
        }
        let (min_x, min_y, max_x, max_y) = offset_data_bounds(e, data);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let cell_index = fn_00584530(e, this, file, x, y);
                if cell_index == -1 {
                    continue;
                }
                let table = e.get(data, OffsetData::pCellFileOffsets).addr();
                let offset = e
                    .mem
                    .u32(table.wrapping_add((cell_index as u32).wrapping_mul(4)));
                if offset == 0 {
                    continue;
                }
                let base = e.get(data, OffsetData::iFileOffset);
                e.call(FILE_SET_OFFSET, &args![file, offset.wrapping_add(base)]);
                if e.call(FILE_RECORD_TYPE, &args![file]).u32() != RECORD_TYPE_CELL {
                    continue;
                }
                if e.call(FILE_NEXT_CHUNK, &args![file]).u32() != CHUNK_EDITOR_ID {
                    continue;
                }
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32() + 1;
                let buffer = e.call(MEMORY_ALLOC, &args![size]).u32();
                e.call(MEMSET, &args![buffer, 0u32, size]);
                e.call(FILE_READ_CHUNK, &args![file, buffer, 0u32]);
                let differs = e.call(EDITOR_ID_COMPARE, &args![buffer, editor_id]).i32();
                if differs == 0 {
                    e.mem.set_i32(out_x.addr(), x);
                    e.mem.set_i32(out_y.addr(), y);
                    e.call(MEMORY_FREE, &args![buffer]);
                    return true;
                }
                e.call(MEMORY_FREE, &args![buffer]);
            }
        }
    }
    false
}

// Translated from 00585b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::LoadCell` (Xbox PDB): the exterior cell (`x`, `y`),
/// loaded or completed from every file of this world space that has it.
/// The existing cell (from the cell map) is used when it has no temporary
/// data yet; a missing one is created and registered first. Each file adds
/// its record (`TESDataHandler::LoadForm`) and its temporary data
/// (`LoadTempDataFromFile`); the finished cell gets `InitItem` and its
/// persistent references assigned. Logs when a file's temporary data does not
/// load. Returns the cell (null when no file has it).
pub fn tes_world_space_load_cell(e: &mut Engine, this: Ptr<TESWorldSpace>, x: i32, y: i32) -> Ptr {
    e.with_stack(8, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, 0x1au32, 1u32, WORLD_SPACE_SOURCE, 0x796u32],
        );
        let mut loaded = true;
        let mut cell = e.call(GET_CELL_FROM_CELL_COORD, &args![this, x, y]).u32();
        e.set_global(CURRENT_WORLD_SPACE, this.addr());
        let already_loaded = cell != 0 && e.call(CELL_HAS_TEMP_DATA, &args![cell]).bool();
        if !already_loaded {
            let count = e.call(FORM_FILE_COUNT, &args![this]).u32();
            for index in 0..count {
                let file = form_get_file(e, this.addr(), index);
                let file = thread_safe_file(e, file);
                let found =
                    !file.is_null() && tes_world_space_find_cell_in_file(e, this, file, x, y);
                if !found {
                    continue;
                }
                if cell == 0 {
                    let block = e.call(MEMORY_ALLOC, &args![CELL_SIZE]).u32();
                    cell = if block == 0 {
                        0
                    } else {
                        e.call(CELL_CONSTRUCT, &args![block]).u32()
                    };
                    e.call(CELL_SET_INTERIOR, &args![cell, 0u32]);
                    e.call(CELL_CREATE_DATA, &args![cell]);
                    e.call(CELL_SET_DATA_COORD, &args![cell, x, y]);
                    e.call(ADD_CELL, &args![this, cell]);
                    let world_id = e.call(WORD_AT_0C, &args![this]).u32();
                    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                    let created_id = e
                        .call(SAVE_LOAD_CREATED_CELL_ID, &args![save_load, world_id, x, y])
                        .u32();
                    if created_id != 0 {
                        e.vcall(cell, 0x128, &args![created_id, 1u32]);
                    }
                    loaded = e.call(DATA_HANDLER_LOAD_FORM, &args![cell, file]).bool();
                } else {
                    let record = e.call(FILE_CURRENT_RECORD, &args![file]).u32();
                    if e.mem.u32(record + 8) & 0x4000 == 0 {
                        e.call(DATA_HANDLER_LOAD_FORM, &args![cell, file]);
                    }
                }
                if !e.call(CELL_LOAD_TEMP_DATA, &args![cell, file]).bool() {
                    loaded = false;
                }
            }
            if cell != 0 {
                e.call(CELL_SET_HAS_TEMP_DATA, &args![cell, 1u32]);
                // 00585d4b/00585d66/00585d86: the global is read for each call.
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                let state = e.call(SAVE_LOAD_GET_STATE, &args![save_load]).u8();
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                e.call(SAVE_LOAD_SET_STATE, &args![save_load, (state == 0) as u32]);
                e.vcall(cell, 0x88, &args![]);
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                e.call(SAVE_LOAD_SET_STATE, &args![save_load, state as u32]);
                e.call(ASSIGN_PERSISTENT_REFS, &args![this, cell]);
            }
        }
        if !loaded {
            let world_id = e.call(WORD_AT_0C, &args![this]).u32();
            let name = e.vcall(this.addr(), SLOT_EDITOR_ID, &args![]).u32();
            e.call(LOG, &args![MSG_CELL_LOAD_FAILED, x, y, name, world_id]);
        }
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
        Ptr::new(cell)
    })
}

// Translated from 00585e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::UnLoadCell` (Xbox PDB): unloads `cell` if it belongs to
/// this world space. A cell from a master file, or from no file, is cleaned
/// up, noted to the save/load game, unloaded from the game loader unless its
/// form ID is 0xFF000000 or above (`00469860`), released from the cell map and
/// deleted; a cell from another file is only cleaned up.
pub fn tes_world_space_un_load_cell(e: &mut Engine, this: Ptr<TESWorldSpace>, cell: Ptr) {
    if cell.is_null() || e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr::<()>() != this.cast() {
        return;
    }
    let file = form_get_file(e, cell.addr(), 0xffff_ffff);
    if file.is_null() || e.call(FILE_IS_MASTER, &args![file]).bool() {
        e.call(CELL_CLEANUP, &args![cell]);
        let save_load = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_NOTE_CELL, &args![save_load, cell]);
        let cell_id = e.call(WORD_AT_0C, &args![cell]).u32();
        let loader_state = e.global::<u32>(LOADER_STATE);
        if !e
            .call(LOADER_STATE_QUERY, &args![loader_state, cell_id])
            .bool()
        {
            let loader = e.global::<u32>(GAME_LOADER);
            e.call(GAME_LOADER_UNLOAD_FORM, &args![loader, cell, 0u32]);
        }
        e.call(RELEASE_CELL, &args![this, cell]);
        e.vcall(cell.addr(), 0x10, &args![1u32]);
    } else {
        e.call(CELL_CLEANUP, &args![cell]);
    }
}

// Translated from 00585ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::FindLandDataInFile` (Xbox PDB): finds the file that has
/// land data for the exterior cell (`x`, `y`), searching this world space's
/// files from the last to the first, or the parent world space (the one
/// `fn_00586390(.., 0)` names) when it uses the parent's cells. On success
/// `*out_file` is the file (reader positioned on the land data) and
/// `*out_offset` its current offset; both are 0 otherwise.
pub fn tes_world_space_find_land_data_in_file(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    x: i32,
    y: i32,
    out_file: Ptr,
    out_offset: Ptr,
) -> bool {
    e.mem.set_u32(out_file.addr(), 0);
    e.mem.set_u32(out_offset.addr(), 0);
    // 00585f04 tests `00586390(0)`; 00585fbe calls it again for the parent
    // it recurses into.
    if !e
        .call(PARENT_FOR, &args![this, 0u32])
        .ptr::<TESWorldSpace>()
        .is_null()
    {
        let parent = e
            .call(PARENT_FOR, &args![this, 0u32])
            .ptr::<TESWorldSpace>();
        return tes_world_space_find_land_data_in_file(e, parent, x, y, out_file, out_offset);
    }
    let count = e.call(FORM_FILE_COUNT, &args![this]).u32();
    for index in (0..count).rev() {
        let file = form_get_file(e, this.addr(), index);
        let file = thread_safe_file(e, file);
        if !file.is_null()
            && tes_world_space_find_cell_in_file(e, this, file, x, y)
            && e.call(CELL_FIND_LAND_DATA_IN_FILE, &args![file]).bool()
        {
            e.mem.set_u32(out_file.addr(), file.addr());
            let offset = e.call(FILE_CURRENT_OFFSET, &args![file]).u32();
            e.mem.set_u32(out_offset.addr(), offset);
            return true;
        }
    }
    false
}

// Translated from 00585fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The climate: the parent world space's when this one uses the parent's
/// value 4, else `pClimate`.
pub fn fn_00585fe0(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = e
        .call(PARENT_FOR, &args![this, 4u32])
        .ptr::<TESWorldSpace>();
    if !parent.is_null() {
        let parent = e
            .call(PARENT_FOR, &args![this, 4u32])
            .ptr::<TESWorldSpace>();
        fn_00585fe0(e, parent)
    } else {
        e.get(this, TESWorldSpace::pClimate)
    }
}

// Translated from 00586020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The image space: `pImageSpace`, or when that is null the parent world
/// space's (value 5) if this one uses it.
pub fn fn_00586020(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let mut image_space = e.get(this, TESWorldSpace::pImageSpace);
    if image_space.is_null() {
        let parent = e
            .call(PARENT_FOR, &args![this, 5u32])
            .ptr::<TESWorldSpace>();
        if !parent.is_null() {
            let parent = e
                .call(PARENT_FOR, &args![this, 5u32])
                .ptr::<TESWorldSpace>();
            image_space = fn_00586020(e, parent);
        }
    }
    image_space
}

// Translated from 00586070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The water form: the parent world space's when this one uses the
/// parent's value 3, else `pWorldWater`, else the game's default water.
pub fn fn_00586070(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = e
        .call(PARENT_FOR, &args![this, 3u32])
        .ptr::<TESWorldSpace>();
    if !parent.is_null() {
        let parent = e
            .call(PARENT_FOR, &args![this, 3u32])
            .ptr::<TESWorldSpace>();
        fn_00586070(e, parent)
    } else if !e.get(this, TESWorldSpace::pWorldWater).is_null() {
        e.get(this, TESWorldSpace::pWorldWater)
    } else {
        e.global::<Ptr>(DEFAULT_WATER)
    }
}

// Translated from 005860c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lod water form: the parent world space's when this one uses the
/// parent's value 1, else `pLODWater`, else the game's default water.
pub fn fn_005860c0(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = e
        .call(PARENT_FOR, &args![this, 1u32])
        .ptr::<TESWorldSpace>();
    if !parent.is_null() {
        let parent = e
            .call(PARENT_FOR, &args![this, 1u32])
            .ptr::<TESWorldSpace>();
        fn_005860c0(e, parent)
    } else if !e.get(this, TESWorldSpace::pLODWater).is_null() {
        e.get(this, TESWorldSpace::pLODWater)
    } else {
        e.global::<Ptr>(DEFAULT_WATER)
    }
}

// Translated from 00586110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fDefaultLandHeight` (Xbox PDB).
pub fn fn_00586110(e: &mut Engine, this: Ptr<TESWorldSpace>, height: f32) {
    e.set(this, TESWorldSpace::fDefaultLandHeight, x87_quiet(height));
}

// Translated from 00586130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fDefaultWaterHeight` (Xbox PDB).
pub fn fn_00586130(e: &mut Engine, this: Ptr<TESWorldSpace>, height: f32) {
    e.set(this, TESWorldSpace::fDefaultWaterHeight, x87_quiet(height));
}

// Translated from 00586150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The music type (`pMusicType`, Xbox PDB).
pub fn fn_00586150(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    e.get(this, TESWorldSpace::pMusicType)
}

// Translated from 00586170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetTerrainManager` (Xbox PDB): the parent world
/// space's terrain manager when this one uses the parent's value 1, else
/// `pTerrainManager`.
pub fn tes_world_space_get_terrain_manager(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = e
        .call(PARENT_FOR, &args![this, 1u32])
        .ptr::<TESWorldSpace>();
    if !parent.is_null() {
        let parent = e
            .call(PARENT_FOR, &args![this, 1u32])
            .ptr::<TESWorldSpace>();
        tes_world_space_get_terrain_manager(e, parent)
    } else {
        e.get(this, TESWorldSpace::pTerrainManager)
    }
}

// Translated from 005861b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cFlags` bit 0.
pub fn fn_005861b0(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x01 != 0
}

// Translated from 005861d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears `cFlags` bit 0.
pub fn fn_005861d0(e: &mut Engine, this: Ptr<TESWorldSpace>, set: bool) {
    let flags = e.get(this, TESWorldSpace::cFlags);
    let flags = if set { flags | 0x01 } else { flags & !0x01 };
    e.set(this, TESWorldSpace::cFlags, flags);
}

// Translated from 00586210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cFlags` bit 1.
pub fn fn_00586210(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x02 != 0
}

// Translated from 00586230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Form flag `0x80000` (the form flags word is read through `0044ddc0`).
pub fn fn_00586230(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.call(FORM_FLAGS, &args![this]).u32() & 0x8_0000 != 0
}

// Translated from 00586260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetHasBorderRegion` (Xbox PDB): `cFlags` bit 3.
pub fn tes_world_space_get_has_border_region(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x08 != 0
}

// Translated from 00586280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears `cFlags` bit 3 (`GetHasBorderRegion`).
pub fn fn_00586280(e: &mut Engine, this: Ptr<TESWorldSpace>, set: bool) {
    let flags = e.get(this, TESWorldSpace::cFlags);
    let flags = if set { flags | 0x08 } else { flags & !0x08 };
    e.set(this, TESWorldSpace::cFlags, flags);
}

// Translated from 005862c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cFlags` bit 4.
pub fn fn_005862c0(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x10 != 0
}

// Translated from 005862e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cFlags` bit 5.
pub fn fn_005862e0(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x20 != 0
}

// Translated from 00586300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cFlags` bit 7.
pub fn fn_00586300(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x80 != 0
}

// Translated from 00586320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `cFlags` bit 6.
pub fn fn_00586320(e: &mut Engine, this: Ptr<TESWorldSpace>) -> bool {
    e.get(this, TESWorldSpace::cFlags) & 0x40 == 0
}

// Translated from 00586340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether this world space takes value number `which` (0 to 5) from its parent: bit
/// `which` of `sParentUseFlags`. Any other number: true.
pub fn fn_00586340(e: &mut Engine, this: Ptr<TESWorldSpace>, which: i32) -> bool {
    if !(0..6).contains(&which) {
        return true;
    }
    let bit = (1u32 << which) as u8 as u16;
    e.get(this, TESWorldSpace::sParentUseFlags) & bit != 0
}

// ---- Second session: 00586390 to 00588b00 --------------------------------------

/// The item of a `BSSimpleList` node (`*006815c0(node)`).
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

/// The next node of a `BSSimpleList` (`00726070`).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

/// Whether a `BSSimpleList` node is empty (`008256d0`).
fn list_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

/// Adds `item` to a list (`005ae3d0` takes the address of the item).
fn list_add(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD, &args![list, slot]);
    });
}

/// Removes `item` from a list (`00905330` takes the address of the item).
fn list_remove(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_REMOVE, &args![list, slot]);
    });
}

/// Allocates and constructs an empty `BSSimpleList` head: `Allocate(8)`,
/// then the constructor when the allocation succeeded.
fn new_list(e: &mut Engine) -> u32 {
    let block = e.call(MEMORY_ALLOC, &args![LIST_SIZE]).u32();
    if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    }
}

/// `NiTMapBase::GetAt(key, &value)` with the value starting at 0, as every
/// caller in this file does: `(found, value)`.
fn map_get(e: &mut Engine, map: u32, key: u32) -> (bool, u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let found = e.call(MAP_GET, &args![map, key, slot]).bool();
        (found, e.mem.u32(slot.addr()))
    })
}

/// `round(value) >> 12`: the cell coordinate of a world coordinate.
fn cell_coordinate(e: &mut Engine, value: f32) -> i32 {
    e.call(FLOAT_ROUND, &args![value]).i32() >> 12
}

/// The key of the cell a `TESObjectCELL` is at: `fn_00587410(GetDataX,
/// GetDataY)`.
fn cell_key(e: &mut Engine, cell: u32) -> u32 {
    let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
    let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
    fn_00587410(e, x as i16, y as i16)
}

// Translated from 00586390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The parent world space if this world space uses the parent's value
/// `which` (`00586340`), else 0. Without a parent: 0 as well.
pub fn fn_00586390(e: &mut Engine, this: Ptr<TESWorldSpace>, which: i32) -> Ptr {
    let parent = e.get(this, TESWorldSpace::pParentWorld);
    if !parent.is_null() && !fn_00586340(e, this, which) {
        return Ptr::new(0);
    }
    e.get(this, TESWorldSpace::pParentWorld)
}

// Translated from 005863d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pParentWorld` (Xbox PDB); without a parent no value is taken from
/// it, so `sParentUseFlags` is cleared.
pub fn fn_005863d0(e: &mut Engine, this: Ptr<TESWorldSpace>, parent: Ptr) {
    e.set(this, TESWorldSpace::pParentWorld, parent);
    if parent.is_null() {
        e.set(this, TESWorldSpace::sParentUseFlags, 0);
    }
}

// Translated from 00586400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetWorldMapData` (Xbox PDB): the parent world space's
/// `WORLD_MAP_DATA` when this one uses the parent's value 2, else its own.
pub fn tes_world_space_get_world_map_data(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = fn_00586390(e, this, 2);
    if !parent.is_null() {
        let parent = fn_00586390(e, this, 2);
        tes_world_space_get_world_map_data(e, parent.cast())
    } else {
        this.at(TESWorldSpace::WorldMapData).cast()
    }
}

// Translated from 00586440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the `TESTexture` part (`+0x24`, the world map texture path)
/// of the parent world space when this one uses the parent's value 2, else
/// of this one (`00408da0`, an empty text when there is none).
pub fn fn_00586440(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    let parent = fn_00586390(e, this, 2);
    let holder = if !parent.is_null() {
        fn_00586390(e, this, 2).addr() + 0x24
    } else {
        this.addr() + 0x24
    };
    e.call(TEXTURE_NAME_TEXT, &args![holder]).ptr()
}

// Translated from 00586480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the world space's own values: no climate, water or lod water, lod
/// water height 0, the default land height of the exe's data
/// (`0x01031920`), water height 0, an empty world map texture name and a
/// zeroed `WORLD_MAP_DATA`.
pub fn fn_00586480(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    e.call(SET_CLIMATE, &args![this, 0u32]);
    e.call(SET_WATER, &args![this, 0u32]);
    e.call(SET_LOD_WATER, &args![this, 0u32]);
    fn_00584150(e, this, 0.0);
    let land_height = e.global::<f32>(RESET_LAND_HEIGHT);
    fn_00586110(e, this, land_height);
    fn_00586130(e, this, 0.0);
    e.call(TEXTURE_SET_NAME, &args![this.addr() + 0x24, 0u32]);
    e.call(MEMSET, &args![this.addr() + 0x80, 0u32, 0x10u32]);
}

// Translated from 00586500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes into `out` (a `BSStringT`) the name of the place at the point
/// (`x`, `y`, `z`) of this world space: the name of the region data of the
/// cell there (through slot `+0x28` of the region data), else the cell's own
/// name; with no cell at the point, the best of the region data entries of
/// the data handler's list whose world space is this one and whose area holds
/// the point (preferring entries with the flag at `+4`, then the higher byte
/// at `+6`); at last the world space's own name and `00586980`'s text. The
/// last answer is cached per world space and point; the function returns
/// whether the text differs from the cached one, which it then replaces.
pub fn fn_00586500(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    out: Ptr,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    let guard = e.global::<u32>(LOCATION_CACHE_GUARD);
    if guard & 1 == 0 {
        e.set_global(LOCATION_CACHE_GUARD, guard | 1);
        e.call(BSSTRING_CONSTRUCT, &args![LOCATION_CACHE_TEXT]);
        e.call(ATEXIT, &args![LOCATION_CACHE_ATEXIT]);
    }
    let guard = e.global::<u32>(LOCATION_CACHE_GUARD);
    if guard & 2 == 0 {
        e.set_global(LOCATION_CACHE_GUARD, guard | 2);
        e.call(LOCAL_STRUCT_CONSTRUCT, &args![LOCATION_CACHE_POINT]);
    }
    let handler = e.global::<u32>(DATA_HANDLER_POINTER);
    if handler == 0 {
        return false;
    }
    e.with_stack(12, |e, point| {
        e.mem.set_f32(point.addr(), x);
        e.mem.set_f32(point.addr() + 4, y);
        e.mem.set_f32(point.addr() + 8, z);
        location_name(e, this, out, handler, point.addr())
    })
}

/// The body of `00586500` once the point is on the stack at `point`.
fn location_name(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    out: Ptr,
    handler: u32,
    point: u32,
) -> bool {
    let cached_world = e.global::<u32>(LOCATION_CACHE_WORLD);
    if cached_world == this.addr()
        && e.call(POINT_EQUAL, &args![LOCATION_CACHE_POINT, point])
            .bool()
    {
        let text = e.call(BSSTRING_TEXT, &args![LOCATION_CACHE_TEXT]).u32();
        e.call(BSSTRING_SET, &args![out, text, 0u32]);
        return false;
    }
    if cached_world != this.addr() {
        e.call(BSSTRING_SET, &args![LOCATION_CACHE_TEXT, EMPTY_TEXT, 0u32]);
    }
    e.set_global(LOCATION_CACHE_WORLD, this.addr());
    for word in 0..3 {
        let value = e.mem.u32(point + word * 4);
        e.set_global(LOCATION_CACHE_POINT + word * 4, value);
    }
    e.call(BSSTRING_SET, &args![out, EMPTY_TEXT, 0u32]);
    let (x, y) = (e.mem.f32(point), e.mem.f32(point + 4));
    let cell = e
        .call(
            DATA_HANDLER_CELL_FROM_COORD,
            &args![handler, x, y, this, 0u32],
        )
        .u32();
    if cell != 0 {
        let mut region_data = 0;
        if e.call(CELL_GET_REGION_LIST, &args![cell, 1u32]).u32() != 0 {
            let manager = e.call(DATA_HANDLER_REGION_MANAGER, &args![handler]).u32();
            let list = e.call(CELL_GET_REGION_LIST, &args![cell, 1u32]).u32();
            let derived = e
                .call(
                    REGION_LIST_GET_DERIVED_DATA,
                    &args![list, 4u32, point, this],
                )
                .u32();
            region_data = e.vcall(manager, 0x10, &args![derived]).u32();
        }
        if region_data != 0 {
            e.vcall(region_data, 0x28, &args![out]);
        } else {
            let name = e.call(TEXTURE_NAME_TEXT, &args![cell + 0x18]).u32();
            e.call(BSSTRING_SET, &args![out, name, 0u32]);
        }
    } else {
        best_region_name(e, this, out, handler, point);
    }
    if e.call(BSSTRING_LENGTH, &args![out]).u32() == 0 {
        let name = e.call(TEXTURE_NAME_TEXT, &args![this.addr() + 0x18]).u32();
        e.call(BSSTRING_SET, &args![out, name, 0u32]);
    }
    if e.call(BSSTRING_LENGTH, &args![out]).u32() == 0 {
        let text = fn_00586980(e).addr();
        e.call(BSSTRING_SET, &args![out, text, 0u32]);
    }
    let cached = e.call(BSSTRING_TEXT, &args![LOCATION_CACHE_TEXT]).u32();
    if e.call(BSSTRING_COMPARE, &args![out, cached, 1u32]).i32() != 0 {
        let text = e.call(BSSTRING_TEXT, &args![out]).u32();
        e.call(BSSTRING_SET, &args![LOCATION_CACHE_TEXT, text, 0u32]);
        true
    } else {
        false
    }
}

/// The loop of `00586500` for a point with no cell: goes over the list of
/// the data handler (`+0x1d8`), keeps the entries of this world space whose
/// region entry list holds the point, and lets the best region data write
/// its name into `out`.
fn best_region_name(e: &mut Engine, this: Ptr<TESWorldSpace>, out: Ptr, handler: u32, point: u32) {
    let list = e.call(DATA_HANDLER_LIST_1D8, &args![handler]).u32();
    let mut node = if list != 0 { list + 4 } else { 0 };
    let mut best_flag = false;
    let mut best_priority: i32 = -1;
    e.with_stack(8, |e, probe| {
        e.call(REGION_POINT_BUILD, &args![probe, point]);
        while node != 0 {
            let item_at = e.call(LIST_NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item_at) == 0 {
                break;
            }
            let entry = list_item(e, node);
            let mut found = false;
            let skip = e.call(REGION_ENTRY_FLAG_20, &args![entry]).bool()
                || e.call(REFERENCE_BASE_FORM, &args![entry]).u32() != this.addr()
                || e.call(REGION_ENTRY_LIST, &args![entry]).u32() == 0
                || {
                    let entries = e.call(REGION_ENTRY_LIST, &args![entry]).u32();
                    list_is_empty(e, entries)
                };
            if skip {
                node = list_next(e, node);
                continue;
            }
            let manager = e.call(DATA_HANDLER_REGION_MANAGER, &args![handler]).u32();
            let data_list = e.call(REGION_ENTRY_WORD_18, &args![entry]).u32();
            let find = e.call(REGION_DATA_LIST_FIND, &args![data_list, 4u32]).u32();
            let region_data = e.vcall(manager, 0x10, &args![find]).u32();
            if region_data == 0 {
                node = list_next(e, node);
                continue;
            }
            let mut entries = e.call(REGION_ENTRY_LIST, &args![entry]).u32();
            while entries != 0 && list_item(e, entries) != 0 {
                let candidate = list_item(e, entries);
                if e.call(REGION_POINT_IN_ENTRY, &args![candidate, probe])
                    .bool()
                {
                    found = true;
                    break;
                }
                entries = list_next(e, entries);
            }
            if !found {
                node = list_next(e, node);
                continue;
            }
            let flag = e.call(REGION_DATA_BYTE_4, &args![region_data]).bool();
            let accept = if flag && !best_flag {
                true
            } else if !flag && best_flag {
                false
            } else {
                e.call(REGION_DATA_BYTE_6, &args![region_data]).u8() as i32 > best_priority
            };
            if accept {
                best_flag = e.call(REGION_DATA_BYTE_4, &args![region_data]).bool();
                best_priority = e.call(REGION_DATA_BYTE_6, &args![region_data]).u8() as i32;
                e.vcall(region_data, 0x28, &args![out]);
            }
            node = list_next(e, node);
        }
    });
}

// Translated from 00586980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the exe's default location object (`0x011ca130`): the word at
/// `+4` of it, or 0.
pub fn fn_00586980(e: &mut Engine) -> Ptr {
    e.call(DEFAULT_LOCATION_TEXT, &args![DEFAULT_LOCATION_OBJECT])
        .ptr()
}

// Translated from 00586990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetGrassForLocation` (Xbox PDB): fills `count` entries of
/// 0x44 bytes at `out` with the grass of the area (`x1`, `y1`) to (`x2`,
/// `y2`) of this world space. The region data of the cell in the middle
/// gives the grass list (slot `+0x28` of the region data's list); an entry
/// is kept when its weight at the area's centre is not 0, and every kept
/// entry gets its mesh path (`data/meshes/%s`), form ID, parameters and, for
/// each of nine sample points over the area, the weight there. Entries
/// with the slot `+0xc` result non-zero come first.
#[allow(clippy::too_many_arguments)]
pub fn tes_world_space_get_grass_for_location(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    out: Ptr,
    count: u32,
) {
    // Nine points (12 bytes each), then the centre point (12 bytes).
    e.with_stack(0x6c + 12, |e, block| {
        let points = block.addr();
        let centre = block.addr() + 0x6c;
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![points, 0xCu32, 9u32, LOCAL_STRUCT_CONSTRUCT],
        );
        e.call(LOCAL_STRUCT_CONSTRUCT, &args![centre]);
        let handler = e.global::<u32>(DATA_HANDLER_POINTER);
        if handler == 0 || count == 0 {
            return;
        }
        let two = e.global::<f64>(DOUBLE_TWO);
        let centre_x = ((x1 as f64 + x2 as f64) / two) as f32;
        let centre_y = ((y1 as f64 + y2 as f64) / two) as f32;
        e.mem.set_f32(centre, centre_x);
        e.mem.set_f32(centre + 4, centre_y);
        e.mem.set_f32(centre + 8, 0.0);
        let step_x = ((x2 as f64 - x1 as f64) / two) as f32;
        let step_y = ((y2 as f64 - y1 as f64) / two) as f32;
        let cell = e
            .call(
                DATA_HANDLER_CELL_FROM_COORD,
                &args![handler, centre_x, centre_y, this, 0u32],
            )
            .u32();
        if cell == 0 || e.call(CELL_GET_REGION_LIST, &args![cell, 1u32]).u32() == 0 {
            return;
        }
        for i in 0..9u32 {
            let at = points + i * 12;
            e.mem
                .set_f32(at, ((i % 3) as f64 * step_x as f64 + x1 as f64) as f32);
            e.mem
                .set_f32(at + 4, ((i / 3) as f64 * step_y as f64 + y1 as f64) as f32);
            e.mem.set_f32(at + 8, 0.0);
        }
        let manager = e.call(DATA_HANDLER_REGION_MANAGER, &args![handler]).u32();
        let region_list = e.call(CELL_GET_REGION_LIST, &args![cell, 1u32]).u32();
        let derived = e
            .call(
                REGION_LIST_GET_DERIVED_DATA,
                &args![region_list, 6u32, centre, this],
            )
            .u32();
        let region_data = e.vcall(manager, 0x18, &args![derived]).u32();
        if region_data == 0 {
            return;
        }
        let list = new_list(e);
        for first_pass in [true, false] {
            let head = e.vcall(region_data, 0x28, &args![]).u32();
            let mut node = if head != 0 { head + 4 } else { 0 };
            while node != 0 {
                let item = list_item(e, node);
                let form = if item != 0 {
                    e.vcall(item, 4, &args![]).u32()
                } else {
                    0
                };
                let wanted = item != 0
                    && {
                        let slot_c = e.vcall(item, 0xc, &args![]).u32();
                        (slot_c != 0) == first_pass
                    }
                    && form != 0
                    && e.vcall(form, 0x180, &args![]).u8() != 0
                    && e.vcall(item, 0x18, &args![centre, this, 0u32]).f64()
                        != e.global::<f64>(DOUBLE_ZERO);
                if wanted {
                    let entry = e.call(MEMORY_ALLOC, &args![8u32]).u32();
                    e.mem.set_u32(entry, item);
                    let weight = e.vcall(form, 0x180, &args![]).u8();
                    let hundred = e.global::<f64>(DOUBLE_HUNDRED);
                    e.mem.set_f32(entry + 4, (weight as f64 / hundred) as f32);
                    list_add(e, list, entry);
                }
                node = list_next(e, node);
            }
        }
        if list == 0 || list_is_empty(e, list) {
            if list != 0 {
                e.call(LIST_DESTROY, &args![list, 1u32]);
            }
            return;
        }
        grass_entries(e, this, out, count, points, list);
    });
}

/// The second half of `GetGrassForLocation`: fills the output entries from
/// the collected list, evaluates the weights at the nine sample points,
/// then frees the list.
fn grass_entries(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    out: Ptr,
    count: u32,
    points: u32,
    list: u32,
) {
    let bytes = (count as u64 * 4).min(u32::MAX as u64) as u32;
    let sources = e.call(MEMORY_ALLOC, &args![bytes]).u32();
    e.call(MEMSET, &args![sources, 0u32, count.wrapping_shl(2)]);
    let mut cursor = list;
    for i in 0..count {
        let slot = out.addr() + i * 0x44;
        e.call(MEMSET, &args![slot + 0x20, 0u32, 0x24u32]);
        let old = e.mem.u32(slot);
        if old != 0 {
            e.call(MEMORY_FREE, &args![old]);
        }
        e.mem.set_u32(slot, 0);
        if cursor == 0 {
            continue;
        }
        let mut entry = list_item(e, cursor);
        while entry == 0 {
            cursor = list_next(e, cursor);
            if cursor == 0 {
                break;
            }
            entry = list_item(e, cursor);
        }
        if entry == 0 {
            continue;
        }
        let region = e.mem.u32(entry);
        let form = e.vcall(region, 4, &args![]).u32();
        e.mem.set_u32(sources + i * 4, region);
        let old = e.mem.u32(slot);
        if old != 0 {
            e.call(MEMORY_FREE, &args![old]);
        }
        let path = e.call(MEMORY_ALLOC, &args![0x104u32]).u32();
        let name = e.call(FORM_NAME_TEXT, &args![form]).u32();
        e.call(SPRINTF_S, &args![path, 0x104u32, MESH_PATH_FORMAT, name]);
        e.mem.set_u32(slot, path);
        let form_id = e.call(WORD_AT_0C, &args![form]).u32();
        e.mem.set_u32(slot + 4, form_id);
        for (offset, vtable_slot) in [(8u32, 0x1b0u32), (0xc, 0x1b8), (0x10, 0x1c0), (0x18, 0x1c8)]
        {
            let value = e.vcall(form, vtable_slot, &args![]).f32();
            e.mem.set_f32(slot + offset, value);
        }
        for (offset, vtable_slot) in [(0x1cu32, 0x1d0u32), (0x1d, 0x1d8), (0x1e, 0x1e0)] {
            let value = e.vcall(form, vtable_slot, &args![]).u8();
            e.mem.set_u8(slot + offset, value);
        }
        let setting = e.call(GRASS_SETTING_FLOAT, &args![]).f32();
        e.mem.set_f32(slot + 0x14, setting);
        cursor = list_next(e, cursor);
    }
    for sample in 0..9u32 {
        for i in 0..count {
            let region = e.mem.u32(sources + i * 4);
            let slot = out.addr() + i * 0x44;
            if region != 0 && e.mem.u32(slot + 4) != 0 {
                let weight = e
                    .vcall(region, 0x18, &args![points + sample * 12, this, 1u32])
                    .f32();
                e.mem.set_f32(slot + 0x20 + sample * 4, weight);
            }
        }
    }
    e.call(MEMORY_FREE, &args![sources]);
    let mut cursor = list;
    while cursor != 0 && list_item(e, cursor) != 0 {
        let entry = list_item(e, cursor);
        cursor = list_next(e, cursor);
        e.call(MEMORY_FREE, &args![entry]);
    }
    e.call(LIST_CLEAR, &args![list]);
    e.call(LIST_DESTROY, &args![list, 1u32]);
}

// Translated from 00587410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of the cell at (`x`, `y`): `x` in the high 16 bits, `y` in the low
/// 16 (`__cdecl`, both signed 16-bit).
pub fn fn_00587410(_e: &mut Engine, x: i16, y: i16) -> u32 {
    ((x as i32 as u32) << 16) | (y as u16 as u32)
}

// Translated from 00587440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetKeyForWorldCoord` (Xbox PDB), `__cdecl`: the key of the
/// cell holding the point (`x`, `y`, `z` read at `coords`); `0x7fff7fff` when
/// a coordinate is not finite or is a NaN.
pub fn tes_world_space_get_key_for_world_coord(e: &mut Engine, coords: Ptr) -> u32 {
    let (x, y, z) = (
        e.mem.f32(coords.addr()),
        e.mem.f32(coords.addr() + 4),
        e.mem.f32(coords.addr() + 8),
    );
    let valid = e.call(IS_FINITE, &args![x as f64]).i32() != 0
        && e.call(IS_FINITE, &args![y as f64]).i32() != 0
        && e.call(IS_FINITE, &args![z as f64]).i32() != 0
        && e.call(IS_NAN, &args![x as f64]).i32() == 0
        && e.call(IS_NAN, &args![y as f64]).i32() == 0
        && e.call(IS_NAN, &args![z as f64]).i32() == 0;
    if !valid {
        return 0x7fff_7fff;
    }
    let cell_x = cell_coordinate(e, x);
    let cell_y = cell_coordinate(e, y);
    fn_00587410(e, cell_x as i16, cell_y as i16)
}

// Translated from 00587520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits a cell key into its x (high 16 bits, stored at `out_x`) and y (low
/// 16 bits, stored at `out_y`), `__cdecl`.
pub fn fn_00587520(e: &mut Engine, key: u32, out_x: Ptr, out_y: Ptr) {
    e.mem.set_u16(out_x.addr(), (key >> 16) as u16);
    e.mem.set_u16(out_y.addr(), key as u16);
}

// Translated from 00587550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetCellFromWorldCoord` (Xbox PDB): the exterior cell
/// holding the point (`x`, `y` read at `coords`), or 0.
pub fn tes_world_space_get_cell_from_world_coord(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    coords: Ptr,
) -> Ptr {
    let x = e.mem.f32(coords.addr());
    let cell_x = cell_coordinate(e, x);
    let y = e.mem.f32(coords.addr() + 4);
    let cell_y = cell_coordinate(e, y);
    tes_world_space_get_cell_from_cell_coord(e, this, cell_x, cell_y)
}

// Translated from 005875a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB): the exterior cell at
/// (`x`, `y`) in the cell map, or 0. A coordinate outside -0x8000 to 0x7fff
/// is logged and gives 0.
pub fn tes_world_space_get_cell_from_cell_coord(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    x: i32,
    y: i32,
) -> Ptr {
    if x > 0x7fff || y > 0x7fff || x < -0x8000 || y < -0x8000 {
        e.call(LOG, &args![MSG_INVALID_CELL_COORD, -0x8000i32, 0x7fffu32]);
        return Ptr::new(0);
    }
    let key = fn_00587410(e, x as i16, y as i16);
    let map = e.get(this, TESWorldSpace::pCellMap).addr();
    let (found, cell) = map_get(e, map, key);
    if found {
        Ptr::new(cell)
    } else {
        Ptr::new(0)
    }
}

// Translated from 00587630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::GetCellFromKey` (Xbox PDB): the cell stored under `key`
/// in the cell map, or 0.
pub fn tes_world_space_get_cell_from_key(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    key: u32,
) -> Ptr {
    let map = e.get(this, TESWorldSpace::pCellMap).addr();
    let (found, cell) = map_get(e, map, key);
    if found {
        Ptr::new(cell)
    } else {
        Ptr::new(0)
    }
}

// Translated from 00587670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AddCell` (Xbox PDB): adds a cell to the world space and
/// tells the cell its world space. A persistent cell (form flag `0x400`)
/// becomes `pPersistentCell` unless there is one; any other goes into the cell
/// map under its coordinates' key, unless that key is taken (logged). A cell
/// with bit 0 of the byte at `+0x24` set is refused. Returns whether the
/// cell was added.
pub fn tes_world_space_add_cell(e: &mut Engine, this: Ptr<TESWorldSpace>, cell: Ptr) -> bool {
    if cell.is_null() || e.call(CELL_FLAG_24_BIT_0, &args![cell]).bool() {
        return false;
    }
    if e.call(FORM_FLAG_400, &args![cell]).bool() {
        if !e.get(this, TESWorldSpace::pPersistentCell).is_null() {
            return false;
        }
        e.set(this, TESWorldSpace::pPersistentCell, cell);
    } else {
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        let key = fn_00587410(e, x as i16, y as i16);
        let map = e.get(this, TESWorldSpace::pCellMap).addr();
        let (found, existing) = map_get(e, map, key);
        if found {
            let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
            let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
            let name = e.vcall(existing, SLOT_EDITOR_ID, &args![]).u32();
            let existing_id = e.call(WORD_AT_0C, &args![existing]).u32();
            e.call(LOG, &args![MSG_CELL_EXISTS, existing_id, name, x, y]);
            return false;
        }
        e.call(MAP_SET_AT, &args![map, key, cell]);
    }
    e.call(CELL_SET_WORLD_SPACE, &args![cell, this]);
    true
}

// Translated from 00587760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::ReleaseCell` (Xbox PDB): removes the cell from the cell
/// map if it is the one stored under its key, and clears its world space.
pub fn tes_world_space_release_cell(e: &mut Engine, this: Ptr<TESWorldSpace>, cell: Ptr) {
    if cell.is_null() || e.call(CELL_FLAG_24_BIT_0, &args![cell]).bool() {
        return;
    }
    let key = cell_key(e, cell.addr());
    let map = e.get(this, TESWorldSpace::pCellMap).addr();
    let (found, stored) = map_get(e, map, key);
    if found && stored == cell.addr() {
        e.call(MAP_REMOVE_AT, &args![map, key]);
        e.call(CELL_SET_WORLD_SPACE, &args![cell, 0u32]);
    }
}

// Translated from 005877e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every cell of the cell map (virtual destructor with the delete
/// flag, slot `+0x10`), unless the world space has form flag `0x4000`, and
/// empties the map.
pub fn fn_005877e0(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    let map = e.get(this, TESWorldSpace::pCellMap).addr();
    if !e.call(FORM_FLAG_4000, &args![this]).bool() {
        let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
        while position != 0 {
            let (next, _key, cell) = map_get_next(e, map, position);
            position = next;
            if cell != 0 {
                e.vcall(cell, 0x10, &args![1u32]);
            }
        }
    }
    e.call(MAP_REMOVE_ALL, &args![map]);
}

// Translated from 00587870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The list stored for the cell's key in the overlapped multibound map
/// (`+0x68`), or 0 when there is no map or no entry.
pub fn fn_00587870(e: &mut Engine, this: Ptr<TESWorldSpace>, cell: Ptr) -> Ptr {
    let mut list = 0;
    let map = e.get(this, TESWorldSpace::pOverlappedMultiboundMap).addr();
    if map != 0 {
        let key = cell_key(e, cell.addr());
        list = map_get(e, map, key).1;
    }
    Ptr::new(list)
}

// Translated from 005878d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AddMultiBoundRef` (Xbox PDB): for a multibound reference
/// (`00439f90`), adds it to the list of every cell its bounds overlap
/// except the cell it stands in (`MultiBoundIntersectsCell`), in the
/// overlapped multibound map (`+0x68`, created on first use). The range of
/// cells is the reference's position plus or minus the radius of its half
/// extent.
pub fn tes_world_space_add_multi_bound_ref(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    reference: Ptr,
) {
    if reference.is_null() || !e.call(IS_MULTIBOUND_REF, &args![reference]).bool() {
        return;
    }
    let extent = e
        .call(REFERENCE_MULTIBOUND_HALF_EXTENT, &args![reference])
        .u32();
    let radius = e.with_stack(12, |e, copy| {
        for word in 0..3 {
            let value = e.mem.u32(extent + word * 4);
            e.mem.set_u32(copy.addr() + word * 4, value);
        }
        e.call(MULTIBOUND_RADIUS, &args![copy]).f32()
    });
    let position = |e: &mut Engine, axis: u32| -> f32 {
        let at = e.vcall(reference.addr(), SLOT_POSITION, &args![]).u32();
        e.mem.f32(at + axis * 4)
    };
    let low_x = (position(e, 0) as f64 - radius as f64) as f32;
    let low_x = cell_coordinate(e, low_x);
    let low_y = (position(e, 1) as f64 - radius as f64) as f32;
    let low_y = cell_coordinate(e, low_y);
    let high_x = (position(e, 0) as f64 + radius as f64) as f32;
    let high_x = cell_coordinate(e, high_x);
    let high_y = (position(e, 1) as f64 + radius as f64) as f32;
    let high_y = cell_coordinate(e, high_y);
    let own_x = position(e, 0);
    let own_x = cell_coordinate(e, own_x);
    let own_y = position(e, 1);
    let own_y = cell_coordinate(e, own_y);
    let mut x = low_x;
    while x <= high_x {
        let mut y = low_y;
        while y <= high_y {
            if (x != own_x || y != own_y)
                && e.call(MULTIBOUND_INTERSECTS_CELL, &args![reference, x, y])
                    .bool()
            {
                if e.get(this, TESWorldSpace::pOverlappedMultiboundMap)
                    .is_null()
                {
                    let block = e.call(MEMORY_ALLOC, &args![LIST_MAP_SIZE]).u32();
                    let map = if block == 0 {
                        0
                    } else {
                        e.call(LIST_MAP_CONSTRUCT, &args![block, MULTIBOUND_MAP_BUCKETS])
                            .u32()
                    };
                    e.set(this, TESWorldSpace::pOverlappedMultiboundMap, Ptr::new(map));
                }
                let map = e.get(this, TESWorldSpace::pOverlappedMultiboundMap).addr();
                let key = fn_00587410(e, x as i16, y as i16);
                let (_, mut list) = map_get(e, map, key);
                if list == 0 {
                    list = new_list(e);
                    e.call(MAP_SET_AT, &args![map, key, list]);
                }
                list_add(e, list, reference.addr());
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 00587bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the overlapped multibound map (`+0x68`) with the lists it holds,
/// and clears the pointer.
pub fn fn_00587bb0(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    let map = e.get(this, TESWorldSpace::pOverlappedMultiboundMap).addr();
    if map == 0 {
        return;
    }
    let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
    while position != 0 {
        let (next, _key, list) = map_get_next(e, map, position);
        position = next;
        if list != 0 {
            e.call(LIST_CLEAR, &args![list]);
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
    }
    e.call(MAP_REMOVE_ALL, &args![map]);
    let map = e.get(this, TESWorldSpace::pOverlappedMultiboundMap).addr();
    if map != 0 {
        // The scalar deleting destructor (slot 0) with the delete flag.
        e.vcall(map, 0, &args![1u32]);
    }
    e.set(this, TESWorldSpace::pOverlappedMultiboundMap, Ptr::new(0));
}

// Translated from 00587c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::IsFixedRef` (Xbox PDB), `__cdecl`: whether the reference's
/// base form is one of the types that never move (form types 0xd, 0x15,
/// 0x1b, 0x1c, 0x20, 0x21, 0x25 to 0x27, 0x2c and 0x2d).
pub fn tes_world_space_is_fixed_ref(e: &mut Engine, reference: Ptr) -> bool {
    if reference.is_null() {
        return false;
    }
    let base = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if base == 0 {
        return false;
    }
    let base = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    matches!(
        e.call(FORM_TYPE, &args![base]).u32(),
        0xd | 0x15 | 0x1b | 0x1c | 0x20 | 0x21 | 0x25 | 0x26 | 0x27 | 0x2c | 0x2d
    )
}

// Translated from 00587d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AddToPersistentRefData` (Xbox PDB): files a persistent
/// reference (unless it has form flag `0x4000`) while holding the persistent
/// reference lock: a fixed one in the list under its cell's key in the fixed
/// persistent reference map (`+0x50`), any other in the mobile list (`+0x60`).
pub fn tes_world_space_add_to_persistent_ref_data(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    reference: Ptr,
) {
    if reference.is_null() || e.call(FORM_FLAG_4000, &args![reference]).bool() {
        return;
    }
    e.call(LOCK_ENTER, &args![PERSISTENT_REF_LOCK, 0u32]);
    if !tes_world_space_is_fixed_ref(e, reference) {
        let list = this.at(TESWorldSpace::MobilePersistentRefList).addr();
        list_add(e, list, reference.addr());
    } else {
        let position = e.vcall(reference.addr(), SLOT_POSITION, &args![]).u32();
        let key = tes_world_space_get_key_for_world_coord(e, Ptr::new(position));
        let map = this.at(TESWorldSpace::FixedPersistentRefMap).addr();
        let (_, mut list) = map_get(e, map, key);
        if list == 0 {
            list = new_list(e);
            e.call(MAP_SET_AT, &args![map, key, list]);
        }
        list_add(e, list, reference.addr());
    }
    e.call(LOCK_LEAVE, &args![PERSISTENT_REF_LOCK]);
}

// Translated from 00587e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::RemoveFromPersistentRefData` (Xbox PDB): the inverse of
/// `AddToPersistentRefData`; a fixed reference's list is deleted with its
/// map entry when it becomes empty.
pub fn tes_world_space_remove_from_persistent_ref_data(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    reference: Ptr,
) {
    if reference.is_null() || e.call(FORM_FLAG_4000, &args![reference]).bool() {
        return;
    }
    e.call(LOCK_ENTER, &args![PERSISTENT_REF_LOCK, 0u32]);
    if !tes_world_space_is_fixed_ref(e, reference) {
        let list = this.at(TESWorldSpace::MobilePersistentRefList).addr();
        list_remove(e, list, reference.addr());
    } else {
        let position = e.vcall(reference.addr(), SLOT_POSITION, &args![]).u32();
        let key = tes_world_space_get_key_for_world_coord(e, Ptr::new(position));
        let map = this.at(TESWorldSpace::FixedPersistentRefMap).addr();
        let (_, list) = map_get(e, map, key);
        if list != 0 {
            list_remove(e, list, reference.addr());
            if list_is_empty(e, list) {
                e.call(LIST_DESTROY, &args![list, 1u32]);
                e.call(MAP_REMOVE_AT, &args![map, key]);
            }
        }
    }
    e.call(LOCK_LEAVE, &args![PERSISTENT_REF_LOCK]);
}

// Translated from 00587f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the persistent reference data under the lock: the mobile list,
/// every list of the fixed persistent reference map (cleared and deleted)
/// and the map itself.
pub fn fn_00587f40(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    e.call(LOCK_ENTER, &args![PERSISTENT_REF_LOCK, 0u32]);
    let mobile = this.at(TESWorldSpace::MobilePersistentRefList).addr();
    e.call(LIST_CLEAR, &args![mobile]);
    let map = this.at(TESWorldSpace::FixedPersistentRefMap).addr();
    let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
    while position != 0 {
        let (next, _key, list) = map_get_next(e, map, position);
        position = next;
        if list != 0 {
            e.call(LIST_CLEAR, &args![list]);
            e.call(LIST_DESTROY, &args![list, 1u32]);
        }
    }
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.call(LOCK_LEAVE, &args![PERSISTENT_REF_LOCK]);
}

// Translated from 00587ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AddPersistentRef` (Xbox PDB): adds the reference (unless it
/// has form flag `0x4000`) to the persistent cell, making the cell first if
/// needed.
pub fn tes_world_space_add_persistent_ref(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    reference: Ptr,
) {
    if reference.is_null() || e.call(FORM_FLAG_4000, &args![reference]).bool() {
        return;
    }
    let cell = tes_world_space_create_persistent_cell(e, this);
    e.call(CELL_ADD_REFERENCE, &args![cell, reference, 0u32]);
}

// Translated from 00588030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::RemovePersistentRef` (Xbox PDB): removes the reference from
/// the persistent cell, if there is one.
pub fn tes_world_space_remove_persistent_ref(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    reference: Ptr,
) {
    if reference.is_null() {
        return;
    }
    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(CELL_REMOVE_REFERENCE, &args![cell, reference]);
    }
}

// Translated from 00588070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::CreatePersistentCell` (Xbox PDB): the persistent cell,
/// made (a `TESObjectCELL`, marked persistent, with its cell data) when the
/// world space has none yet.
pub fn tes_world_space_create_persistent_cell(e: &mut Engine, this: Ptr<TESWorldSpace>) -> Ptr {
    if e.get(this, TESWorldSpace::pPersistentCell).is_null() {
        let block = e.call(MEMORY_ALLOC, &args![CELL_SIZE]).u32();
        let cell = if block == 0 {
            0
        } else {
            e.call(CELL_CONSTRUCT, &args![block]).u32()
        };
        e.set(this, TESWorldSpace::pPersistentCell, Ptr::new(cell));
        let cell = e.get(this, TESWorldSpace::pPersistentCell);
        e.call(CELL_SET_PERSISTENT, &args![cell, 1u32]);
        let cell = e.get(this, TESWorldSpace::pPersistentCell);
        e.call(CELL_CREATE_DATA, &args![cell]);
    }
    e.get(this, TESWorldSpace::pPersistentCell)
}

// Translated from 00588120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the persistent cell, if there is one, to assign its persistent
/// references to the cells of this world space
/// (`AssignPersistentRefsToCellsInWorld`).
pub fn fn_00588120(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(CELL_ASSIGN_PERSISTENT_REFS_IN_WORLD, &args![cell, this]);
    }
}

// Translated from 00588150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AssignPersistentRefsToCell` (Xbox PDB): under the
/// persistent reference lock, adds to `cell` every reference filed under its
/// key in the fixed persistent reference map, and every mobile persistent
/// reference whose `+0x40` word is not `cell` and whose position is in the
/// cell.
pub fn tes_world_space_assign_persistent_refs_to_cell(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    cell: Ptr,
) {
    if cell.is_null() {
        return;
    }
    e.call(LOCK_ENTER, &args![PERSISTENT_REF_LOCK, 0u32]);
    let key = cell_key(e, cell.addr());
    let map = this.at(TESWorldSpace::FixedPersistentRefMap).addr();
    let (_, mut node) = map_get(e, map, key);
    while node != 0 && !list_is_empty(e, node) {
        let reference = list_item(e, node);
        e.call(CELL_ADD_REFERENCE, &args![cell, reference, 0u32]);
        node = list_next(e, node);
    }
    let mut node = this.at(TESWorldSpace::MobilePersistentRefList).addr();
    while node != 0 && !list_is_empty(e, node) {
        let reference = list_item(e, node);
        if e.call(REFERENCE_WORD_40, &args![reference]).u32() != cell.addr() {
            let position = e.vcall(reference, SLOT_POSITION, &args![]).u32();
            if e.call(CELL_CONTAINS_POINT, &args![cell, position]).bool() {
                e.call(CELL_ADD_REFERENCE, &args![cell, reference, 0u32]);
            }
        }
        node = list_next(e, node);
    }
    e.call(LOCK_LEAVE, &args![PERSISTENT_REF_LOCK]);
}

// Translated from 00588270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `argument` to `0054db50` on the persistent cell, if there is one.
pub fn fn_00588270(e: &mut Engine, this: Ptr<TESWorldSpace>, argument: u32) {
    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(CELL_PERSISTENT_ACTION, &args![cell, argument]);
    }
}

/// `BuildMapMarkerList` and `005883c0`: the list of world spaces below
/// `this` that fill it with `fill` (the parent's list when `all` is 0 and
/// this world space uses the parent's value 2).
fn build_world_list(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    all: u8,
    recurse: fn(&mut Engine, Ptr<TESWorldSpace>, u8) -> Ptr,
    fill: fn(&mut Engine, Ptr<TESWorldSpace>, Ptr),
) -> Ptr {
    if !fn_00586390(e, this, 2).is_null() && all == 0 {
        let parent = fn_00586390(e, this, 2);
        return recurse(e, parent.cast(), 0);
    }
    let list = Ptr::new(new_list(e));
    fill(e, this, list);
    if all == 0 {
        let handler = e.global::<u32>(DATA_HANDLER_POINTER);
        let mut node = e.call(DATA_HANDLER_WORLD_LIST, &args![handler]).u32();
        while node != 0 {
            let world = list_item(e, node);
            if world != 0 && fn_00586390(e, Ptr::new(world), 2).addr() == this.addr() {
                fill(e, Ptr::new(world), list);
            }
            node = list_next(e, node);
        }
    }
    list
}

// Translated from 005882a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::BuildMapMarkerList` (Xbox PDB): a new list of the map
/// markers of the world space (`fn_005884e0`, which asks the persistent
/// cell) and, unless `all` is non-zero, of the world spaces that use this
/// one's value 2. When this world space uses its parent's value 2 and `all`
/// is 0, the parent builds the list instead.
pub fn tes_world_space_build_map_marker_list(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    all: u8,
) -> Ptr {
    build_world_list(
        e,
        this,
        all,
        tes_world_space_build_map_marker_list,
        fn_005884e0,
    )
}

// Translated from 005883c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as `BuildMapMarkerList` with the other list filler
/// (`fn_00588520`).
pub fn fn_005883c0(e: &mut Engine, this: Ptr<TESWorldSpace>, all: u8) -> Ptr {
    build_world_list(e, this, all, fn_005883c0, fn_00588520)
}

// Translated from 005884e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets the persistent cell fill `list` (`0054b830`), if `list` and the
/// cell exist.
pub fn fn_005884e0(e: &mut Engine, this: Ptr<TESWorldSpace>, list: Ptr) {
    if list.is_null() {
        return;
    }
    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(CELL_FILL_LIST_FIRST, &args![cell, list]);
    }
}

// Translated from 00588520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets the persistent cell fill `list` (`0054b8c0`), if `list` and the
/// cell exist.
pub fn fn_00588520(e: &mut Engine, this: Ptr<TESWorldSpace>, list: Ptr) {
    if list.is_null() {
        return;
    }
    let cell = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(CELL_FILL_LIST_SECOND, &args![cell, list]);
    }
}

// Translated from 00588560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first cell of the cell map whose editor ID (slot `+0x130`) equals
/// `name` (`00404dc0`), or 0.
pub fn fn_00588560(e: &mut Engine, this: Ptr<TESWorldSpace>, name: Ptr) -> Ptr {
    let mut found = 0;
    if !name.is_null() {
        let map = e.get(this, TESWorldSpace::pCellMap).addr();
        let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
        while position != 0 && found == 0 {
            let (next, _key, cell) = map_get_next(e, map, position);
            position = next;
            if cell != 0 {
                let editor_id = e.vcall(cell, SLOT_EDITOR_ID, &args![]).u32();
                if e.call(EDITOR_ID_COMPARE, &args![editor_id, name]).i32() == 0 {
                    found = cell;
                }
            }
        }
    }
    Ptr::new(found)
}

// Translated from 005885f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits the references around a point with `0054da20` on the cell holding
/// `centre` and then, ring after ring, on every cell around it that exists
/// and is within `radius` of `centre` and within `second_radius` of
/// `second_centre` (a radius of `FLT_MAX` is no limit), until a ring has no
/// such cell; at last the persistent cell. Stops as soon as a visit
/// returns false. Does nothing without `visitor`.
#[allow(clippy::too_many_arguments)]
pub fn fn_005885f0(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    centre: Ptr,
    radius: f32,
    second_centre: Ptr,
    second_radius: f32,
    visitor: u32,
    context: u32,
) {
    if visitor == 0 {
        return;
    }
    let mut cells_in_ring = 1;
    let mut side = 0;
    let x = e.mem.f32(centre.addr());
    let mut cell_x = float_to_int(e, x) >> 12;
    let y = e.mem.f32(centre.addr() + 4);
    let mut cell_y = float_to_int(e, y) >> 12;
    let cell = tes_world_space_get_cell_from_cell_coord(e, this, cell_x, cell_y);
    let visit = |e: &mut Engine, cell: Ptr| -> bool {
        e.call(
            CELL_FOR_REFERENCES_IN_RANGE,
            &args![
                cell,
                centre,
                radius,
                second_centre,
                second_radius,
                visitor,
                context
            ],
        )
        .bool()
    };
    if !cell.is_null() && !visit(e, cell) {
        return;
    }
    // One side of the ring: `side` cells starting at (`cell_x`, `cell_y`),
    // stepping by (`step_x`, `step_y`).
    let ring_cell = |e: &mut Engine,
                     cell_x: &mut i32,
                     cell_y: &mut i32,
                     step_x: i32,
                     step_y: i32,
                     side: i32,
                     count: &mut i32|
     -> bool {
        for _ in 0..side {
            let cell = tes_world_space_get_cell_from_cell_coord(e, this, *cell_x, *cell_y);
            if !cell.is_null()
                && within_limit(e, cell, centre, radius, second_centre, second_radius)
            {
                *count += 1;
                if !visit(e, cell) {
                    return false;
                }
            }
            *cell_x += step_x;
            *cell_y += step_y;
        }
        true
    };
    while cells_in_ring != 0 {
        cell_x -= 1;
        cell_y -= 1;
        side += 2;
        cells_in_ring = 0;
        if !ring_cell(e, &mut cell_x, &mut cell_y, 1, 0, side, &mut cells_in_ring) {
            return;
        }
        if !ring_cell(e, &mut cell_x, &mut cell_y, 0, 1, side, &mut cells_in_ring) {
            return;
        }
        if !ring_cell(e, &mut cell_x, &mut cell_y, -1, 0, side, &mut cells_in_ring) {
            return;
        }
        if !ring_cell(e, &mut cell_x, &mut cell_y, 0, -1, side, &mut cells_in_ring) {
            return;
        }
    }
    let persistent = e.call(GET_PERSISTENT_CELL, &args![this]).u32();
    if persistent != 0 {
        visit(e, Ptr::new(persistent));
    }
}

/// `distance < limit` (false when either is a NaN).
fn is_below(distance: f64, limit: f32) -> bool {
    distance < limit as f64
}

/// The distance test of `005885f0` for one cell: the distance from the cell's
/// square to `centre` must be below `radius` (unless that is `FLT_MAX`), and
/// likewise for the second point.
fn within_limit(
    e: &mut Engine,
    cell: Ptr,
    centre: Ptr,
    radius: f32,
    second_centre: Ptr,
    second_radius: f32,
) -> bool {
    let no_limit = e.global::<f64>(DOUBLE_FLOAT_MAX);
    if radius as f64 != no_limit {
        let distance = e.call(CELL_DISTANCE_TO_POINT, &args![cell, centre]).f64();
        if !is_below(distance, radius) {
            return false;
        }
    }
    if second_radius as f64 != no_limit {
        let distance = e
            .call(CELL_DISTANCE_TO_POINT, &args![cell, second_centre])
            .f64();
        if !is_below(distance, second_radius) {
            return false;
        }
    }
    true
}

// Translated from 00588a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00588a60` for a record type byte (the `this` is not used).
pub fn fn_00588a40(e: &mut Engine, _this: Ptr<TESWorldSpace>, record_type: u8) -> bool {
    fn_00588a60(e, record_type)
}

// Translated from 00588a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// True for the record types `'9'` (cell) and `'D'`, else what `005548a0`
/// answers for the type; `__cdecl`.
pub fn fn_00588a60(e: &mut Engine, record_type: u8) -> bool {
    if record_type == b'9' || record_type == b'D' {
        true
    } else {
        e.call(RECORD_TYPE_CHECK, &args![record_type]).bool()
    }
}

/// Follows a file's master chain (`00473c70`) to its end.
fn first_master_file(e: &mut Engine, file: u32) -> u32 {
    let mut node = file;
    while node != 0 && e.call(FILE_MASTER, &args![node]).u32() != 0 {
        node = e.call(FILE_MASTER, &args![node]).u32();
    }
    node
}

// Translated from 00588a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `OFFSET_DATA` (Xbox PDB) the world space keeps for the first file of
/// `file`'s master chain (`OffsetDataMap`, `+0xb0`), or 0.
pub fn fn_00588a90(e: &mut Engine, this: Ptr<TESWorldSpace>, file: Ptr) -> Ptr {
    let node = first_master_file(e, file.addr());
    let map = this.at(TESWorldSpace::OffsetDataMap).addr();
    let (found, data) = map_get(e, map, node);
    Ptr::new(if found { data } else { 0 })
}

// Translated from 00588b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::CreateOffsetData` (Xbox PDB): the `OFFSET_DATA` for the
/// first file of `file`'s master chain, made (no table, offset 0, bounds
/// from `FLT_MAX` down to `-FLT_MAX`) and stored in the offset data map when
/// there is none.
pub fn tes_world_space_create_offset_data(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    file: Ptr,
) -> Ptr<OffsetData> {
    let node = first_master_file(e, file.addr());
    let map = this.at(TESWorldSpace::OffsetDataMap).addr();
    let (found, mut data) = map_get(e, map, node);
    if !found || data == 0 {
        let block = e.call(MEMORY_ALLOC, &args![OFFSET_DATA_SIZE]).u32();
        data = if block == 0 {
            0
        } else {
            e.call(OFFSET_DATA_CONSTRUCT, &args![block]).u32()
        };
        let data_ptr = Ptr::<OffsetData>::new(data);
        e.set(data_ptr, OffsetData::pCellFileOffsets, Ptr::new(0));
        e.set(data_ptr, OffsetData::iFileOffset, 0);
        let maximum = e.global::<f32>(FLOAT_MAX_VALUE);
        let corner = e.with_stack(8, |e, point| {
            let at = e
                .call(NI_POINT2_CONSTRUCT, &args![point, maximum, maximum])
                .u32();
            (e.mem.u32(at), e.mem.u32(at + 4))
        });
        e.mem.set_u32(data + 4, corner.0);
        e.mem.set_u32(data + 8, corner.1);
        let minimum = e.global::<f32>(FLOAT_MIN_VALUE);
        let corner = e.with_stack(8, |e, point| {
            let at = e
                .call(NI_POINT2_CONSTRUCT, &args![point, minimum, minimum])
                .u32();
            (e.mem.u32(at), e.mem.u32(at + 4))
        });
        e.mem.set_u32(data + 0xc, corner.0);
        e.mem.set_u32(data + 0x10, corner.1);
        e.call(MAP_SET_AT, &args![map, node, data]);
    }
    Ptr::new(data)
}
// ---- Third session: 00588c50 to 00589420 ----------------------------------------

// Translated from 00588c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::OFFSET_DATA` constructor (unnamed in the Xbox PDB): runs
/// the folded constructors of the two `NiPoint2` members (`+4` and `+0xC`,
/// which do nothing) and returns `this`. The caller sets the fields.
pub fn fn_00588c50(e: &mut Engine, this: Ptr<OffsetData>) -> Ptr<OffsetData> {
    e.call(LOCAL_STRUCT_CONSTRUCT, &args![this.addr() + 4]);
    e.call(LOCAL_STRUCT_CONSTRUCT, &args![this.addr() + 0xC]);
    this
}

// Translated from 00588c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the world space's offset data map (`+0xB0`): frees every
/// `OFFSET_DATA`'s cell offset table and the `OFFSET_DATA` itself, then
/// `RemoveAll`s the map.
pub fn fn_00588c80(e: &mut Engine, this: Ptr<TESWorldSpace>) {
    let map = this.at(TESWorldSpace::OffsetDataMap).addr();
    let mut position = e.call(MAP_FIRST_POS, &args![map]).u32();
    while position != 0 {
        let (next, _key, data) = map_get_next(e, map, position);
        position = next;
        let table = e.mem.u32(data);
        e.call(MEMORY_FREE, &args![table]);
        e.call(MEMORY_FREE, &args![data]);
    }
    e.call(MAP_REMOVE_ALL, &args![map]);
}

// Translated from 00588d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the point (two floats at `point`) is covered by the world
/// space's border region: true when the world space has no border region
/// flag at all, else true when some region entry of this world space has a
/// region in its list that contains the point.
pub fn fn_00588d10(e: &mut Engine, this: Ptr<TESWorldSpace>, point: Ptr) -> bool {
    if !tes_world_space_get_has_border_region(e, this) {
        return true;
    }
    let x = e.mem.u32(point.addr());
    let y = e.mem.u32(point.addr() + 4);
    let handler = e.global::<u32>(DATA_HANDLER_POINTER);
    let mut covered = false;
    e.with_stack(8, |e, probe| {
        e.call(REGION_POINT_BUILD_XY, &args![probe, x, y]);
        let list = e.call(DATA_HANDLER_LIST_1D8, &args![handler]).u32();
        let mut node = if list != 0 { list + 4 } else { 0 };
        while node != 0 {
            let item_at = e.call(LIST_NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item_at) == 0 {
                break;
            }
            let entry = list_item(e, node);
            if e.call(REGION_ENTRY_FLAG_40, &args![entry]).bool()
                && !e.call(REGION_ENTRY_FLAG_20, &args![entry]).bool()
                && e.call(REFERENCE_BASE_FORM, &args![entry]).u32() == this.addr()
            {
                let mut entries = e.call(REGION_ENTRY_LIST, &args![entry]).u32();
                while entries != 0 && list_item(e, entries) != 0 {
                    let candidate = list_item(e, entries);
                    if e.call(REGION_POINT_IN_ENTRY, &args![candidate, probe])
                        .bool()
                    {
                        covered = true;
                    }
                    entries = list_next(e, entries);
                }
            }
            node = list_next(e, node);
        }
    });
    covered
}

// Translated from 00588e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWorldSpace::AdjustMapMarkerCoord` (Xbox PDB): converts the position
/// (three floats at `coords`) between world and map space. With `to_map`
/// set it first subtracts the map offsets (x, y); then, when the map scale
/// is neither 1 nor 0, it recentres on the middle of the minimum and maximum
/// coordinates, scales by the scale (or its inverse when `to_map` is set)
/// and moves back; without `to_map` it finally adds the map offsets.
pub fn tes_world_space_adjust_map_marker_coord(
    e: &mut Engine,
    this: Ptr<TESWorldSpace>,
    coords: Ptr,
    to_map: u8,
) {
    let at = coords.addr();
    if to_map != 0 {
        let offset_x = e.call(MAP_OFFSET_X, &args![this]).f64();
        let x = f32::from_bits(e.mem.u32(at)) as f64;
        e.mem.set_u32(at, ((x - offset_x) as f32).to_bits());
        let offset_y = e.call(MAP_OFFSET_Y, &args![this]).f64();
        let y = f32::from_bits(e.mem.u32(at + 4)) as f64;
        e.mem.set_u32(at + 4, ((y - offset_y) as f32).to_bits());
    }
    let scale = e.call(MAP_SCALE, &args![this]).f64();
    if scale != e.global::<f64>(DOUBLE_ONE) && scale != e.global::<f64>(DOUBLE_ZERO) {
        // The middle point is three stack words: the sum of the minimum and
        // maximum coordinates (z is 0), halved.
        let minimum_x = e.get(this, TESWorldSpace::MinimumCoords_x) as f64;
        let maximum_x = e.get(this, TESWorldSpace::MaximumCoords_x) as f64;
        let minimum_y = e.get(this, TESWorldSpace::MinimumCoords_y) as f64;
        let maximum_y = e.get(this, TESWorldSpace::MaximumCoords_y) as f64;
        let half = e.global::<f32>(FLOAT_HALF);
        let factor = if to_map == 0 { scale } else { 1.0 / scale } as f32;
        e.with_stack(12, |e, middle| {
            e.call(LOCAL_STRUCT_CONSTRUCT, &args![middle]);
            let m = middle.addr();
            e.mem.set_u32(m, ((minimum_x + maximum_x) as f32).to_bits());
            e.mem
                .set_u32(m + 4, ((minimum_y + maximum_y) as f32).to_bits());
            e.mem.set_u32(m + 8, 0);
            e.call(POINT3_SCALE, &args![middle, half]);
            e.with_stack(12, |e, relative| {
                e.call(POINT3_SUBTRACT, &args![coords, relative, middle]);
                e.call(POINT3_SCALE, &args![relative, factor]);
                e.with_stack(12, |e, moved| {
                    let result = e.call(POINT3_ADD, &args![middle, moved, relative]).u32();
                    for word in 0..3 {
                        let value = e.mem.u32(result + word * 4);
                        e.mem.set_u32(at + word * 4, value);
                    }
                });
            });
        });
    }
    if to_map == 0 {
        let offset_x = e.call(MAP_OFFSET_X, &args![this]).f64();
        let x = f32::from_bits(e.mem.u32(at)) as f64;
        e.mem.set_u32(at, ((offset_x + x) as f32).to_bits());
        let offset_y = e.call(MAP_OFFSET_Y, &args![this]).f64();
        let y = f32::from_bits(e.mem.u32(at + 4)) as f64;
        e.mem.set_u32(at + 4, ((offset_y + y) as f32).to_bits());
    }
}

/// The body shared by the three `NiTMapBase` constructors: the vtable, the
/// bucket count, an empty item count and a zeroed bucket array.
fn map_base_construct(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTPointerMap::m_uiHashSize, buckets);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let table = e.call(NI_ALLOC, &args![buckets << 2]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(MEMSET, &args![table, 0u32, buckets << 2]);
    this.cast()
}

// Translated from 005890c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor of the `unsigned int` to list map
/// (`0103200c`), with its bucket count.
pub fn fn_005890c0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, LIST_MAP_BASE_VTABLE)
}

// Translated from 005891c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor of the `TESFile *` to `OFFSET_DATA *` map
/// (`0103202c`); see [`fn_005890c0`].
pub fn fn_005891c0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, OFFSET_MAP_BASE_VTABLE)
}

// Translated from 005892c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor of the `int` to cell map (`0103204c`); see
/// [`fn_005890c0`].
pub fn fn_005892c0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, CELL_MAP_BASE_VTABLE)
}

// Translated from 00588fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `NiTPointerMap<unsigned int,
/// BSSimpleList<TESObjectREFR *> *>` with its bucket count: the base
/// constructor, then the derived vtable.
pub fn fn_00588fa0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    fn_005890c0(e, this, buckets);
    e.mem.set_u32(this.addr(), LIST_MAP_VTABLE);
    this.cast()
}

// Translated from 00588fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `NiTMap<TESFile *, TESWorldSpace::OFFSET_DATA *>`
/// with its bucket count.
pub fn fn_00588fd0(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    fn_005891c0(e, this, buckets);
    e.mem.set_u32(this.addr(), OFFSET_MAP_VTABLE);
    this.cast()
}

// Translated from 00589000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `NiTPointerMap<int, TESObjectCELL *>` with its
/// bucket count.
pub fn fn_00589000(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr {
    fn_005892c0(e, this, buckets);
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    this.cast()
}

/// The base class destructor body: sets the base vtable, removes every item
/// and frees the bucket array.
fn map_base_destroy(e: &mut Engine, this: Ptr<NiTPointerMap>, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
}

/// The scalar deleting wrapper: frees the object when bit 0 of `flags` is
/// set; returns `this`.
fn delete_if_asked(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this.cast()
}

// Translated from 00589190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTMapBase` of the `unsigned int` to list map
/// (the C++ exception frame is left out).
pub fn fn_00589190(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    map_base_destroy(e, this, LIST_MAP_BASE_VTABLE);
}

// Translated from 00589290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTMapBase` of the `TESFile *` to `OFFSET_DATA *`
/// map.
pub fn fn_00589290(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    map_base_destroy(e, this, OFFSET_MAP_BASE_VTABLE);
}

// Translated from 00589390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTMapBase` of the `int` to cell map.
pub fn fn_00589390(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    map_base_destroy(e, this, CELL_MAP_BASE_VTABLE);
}

// Translated from 00589130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTPointerMap<unsigned int,
/// BSSimpleList<TESObjectREFR *> *>`: sets the derived vtable, removes every
/// item, then runs the base destructor (the C++ exception frame and its
/// stack cookie are left out).
pub fn fn_00589130(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), LIST_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00589190(e, this);
}

// Translated from 00589230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTMap<TESFile *, TESWorldSpace::OFFSET_DATA *>`;
/// see [`fn_00589130`].
pub fn fn_00589230(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), OFFSET_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00589290(e, this);
}

// Translated from 00589330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTPointerMap<int, TESObjectCELL *>`; see
/// [`fn_00589130`].
pub fn fn_00589330(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00589390(e, this);
}

// Translated from 00589030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, BSSimpleList<TESObjectREFR *> *>::
/// scalar deleting destructor` (Xbox PDB): the destructor, then the free
/// when bit 0 of `flags` is set.
pub fn fn_00589030(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589130(e, this);
    delete_if_asked(e, this, flags)
}

// Translated from 00589060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESFile *, TESWorldSpace::OFFSET_DATA *>::scalar deleting
/// destructor` (Xbox PDB); see [`fn_00589030`].
pub fn fn_00589060(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589230(e, this);
    delete_if_asked(e, this, flags)
}

// Translated from 00589090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<int, TESObjectCELL *>::scalar deleting destructor` (Xbox
/// PDB); see [`fn_00589030`].
pub fn fn_00589090(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589330(e, this);
    delete_if_asked(e, this, flags)
}

// Translated from 005893c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int,
/// BSSimpleList<TESObjectREFR *> *>::scalar deleting destructor` (Xbox
/// PDB): the base destructor, then the free when bit 0 of `flags` is set.
pub fn fn_005893c0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589190(e, this);
    delete_if_asked(e, this, flags)
}

// Translated from 005893f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESFile *, OFFSET_DATA *> >, TESFile *,
/// OFFSET_DATA *>::scalar deleting destructor` (Xbox PDB); see
/// [`fn_005893c0`].
pub fn fn_005893f0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589290(e, this);
    delete_if_asked(e, this, flags)
}

// Translated from 00589420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, int, TESObjectCELL *>::
/// scalar deleting destructor` (Xbox PDB); see [`fn_005893c0`].
pub fn fn_00589420(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr {
    fn_00589390(e, this);
    delete_if_asked(e, this, flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004fd380,
            tes_world_space_get_parent_world_simple(Ptr<TESWorldSpace>) -> Ptr
        ),
        entry!(
            0x00583360,
            tes_world_space_load_partial(Ptr<TESWorldSpace>, Ptr) -> bool
        ),
        entry!(0x00583560, fn_00583560(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(0x005840b0, fn_005840b0(Ptr)),
        entry!(0x00584150, fn_00584150(Ptr<TESWorldSpace>, f32)),
        entry!(0x00584170, fn_00584170(Ptr<TESWorldSpace>)),
        entry!(
            0x00584530,
            fn_00584530(Ptr<TESWorldSpace>, Ptr, i32, i32) -> i32
        ),
        entry!(0x005845e0, tes_world_space_init_item(Ptr<TESWorldSpace>)),
        entry!(0x00584bc0, fn_00584bc0(Ptr<TESWorldSpace>, Ptr)),
        entry!(0x00584be0, fn_00584be0(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(0x00584d40, fn_00584d40(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(0x00584e60, fn_00584e60(Ptr<TESWorldSpace>, u8, Ptr) -> Ptr),
        entry!(0x00584fa0, fn_00584fa0(Ptr<TESWorldSpace>, Ptr)),
        entry!(0x00585250, fn_00585250(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(
            0x005854f0,
            tes_world_space_find_cell_in_file(Ptr<TESWorldSpace>, Ptr, i32, i32) -> bool
        ),
        entry!(0x005857b0, fn_005857b0(Ptr<TESWorldSpace>, Ptr, u32)),
        entry!(0x005858b0, fn_005858b0(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(
            0x00585940,
            tes_world_space_get_ext_cell_data_from_file_by_editor_id(
                Ptr<TESWorldSpace>,
                Ptr,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00585b30,
            tes_world_space_load_cell(Ptr<TESWorldSpace>, i32, i32) -> Ptr
        ),
        entry!(
            0x00585e00,
            tes_world_space_un_load_cell(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(
            0x00585ee0,
            tes_world_space_find_land_data_in_file(Ptr<TESWorldSpace>, i32, i32, Ptr, Ptr) -> bool
        ),
        entry!(0x00585fe0, fn_00585fe0(Ptr<TESWorldSpace>) -> Ptr),
        entry!(0x00586020, fn_00586020(Ptr<TESWorldSpace>) -> Ptr),
        entry!(0x00586070, fn_00586070(Ptr<TESWorldSpace>) -> Ptr),
        entry!(0x005860c0, fn_005860c0(Ptr<TESWorldSpace>) -> Ptr),
        entry!(0x00586110, fn_00586110(Ptr<TESWorldSpace>, f32)),
        entry!(0x00586130, fn_00586130(Ptr<TESWorldSpace>, f32)),
        entry!(0x00586150, fn_00586150(Ptr<TESWorldSpace>) -> Ptr),
        entry!(
            0x00586170,
            tes_world_space_get_terrain_manager(Ptr<TESWorldSpace>) -> Ptr
        ),
        entry!(0x005861b0, fn_005861b0(Ptr<TESWorldSpace>) -> bool),
        entry!(0x005861d0, fn_005861d0(Ptr<TESWorldSpace>, bool)),
        entry!(0x00586210, fn_00586210(Ptr<TESWorldSpace>) -> bool),
        entry!(0x00586230, fn_00586230(Ptr<TESWorldSpace>) -> bool),
        entry!(
            0x00586260,
            tes_world_space_get_has_border_region(Ptr<TESWorldSpace>) -> bool
        ),
        entry!(0x00586280, fn_00586280(Ptr<TESWorldSpace>, bool)),
        entry!(0x005862c0, fn_005862c0(Ptr<TESWorldSpace>) -> bool),
        entry!(0x005862e0, fn_005862e0(Ptr<TESWorldSpace>) -> bool),
        entry!(0x00586300, fn_00586300(Ptr<TESWorldSpace>) -> bool),
        entry!(0x00586320, fn_00586320(Ptr<TESWorldSpace>) -> bool),
        entry!(0x00586340, fn_00586340(Ptr<TESWorldSpace>, i32) -> bool),
        entry!(0x00586390, fn_00586390(Ptr<TESWorldSpace>, i32) -> Ptr),
        entry!(0x005863d0, fn_005863d0(Ptr<TESWorldSpace>, Ptr)),
        entry!(
            0x00586400,
            tes_world_space_get_world_map_data(Ptr<TESWorldSpace>) -> Ptr
        ),
        entry!(0x00586440, fn_00586440(Ptr<TESWorldSpace>) -> Ptr),
        entry!(0x00586480, fn_00586480(Ptr<TESWorldSpace>)),
        entry!(
            0x00586500,
            fn_00586500(Ptr<TESWorldSpace>, Ptr, f32, f32, f32) -> bool
        ),
        entry!(0x00586980, fn_00586980() -> Ptr),
        entry!(
            0x00586990,
            tes_world_space_get_grass_for_location(
                Ptr<TESWorldSpace>,
                f32,
                f32,
                f32,
                f32,
                Ptr,
                u32,
            )
        ),
        entry!(0x00587410, fn_00587410(i16, i16) -> u32),
        entry!(
            0x00587440,
            tes_world_space_get_key_for_world_coord(Ptr) -> u32
        ),
        entry!(0x00587520, fn_00587520(u32, Ptr, Ptr)),
        entry!(
            0x00587550,
            tes_world_space_get_cell_from_world_coord(Ptr<TESWorldSpace>, Ptr) -> Ptr
        ),
        entry!(
            0x005875a0,
            tes_world_space_get_cell_from_cell_coord(Ptr<TESWorldSpace>, i32, i32) -> Ptr
        ),
        entry!(
            0x00587630,
            tes_world_space_get_cell_from_key(Ptr<TESWorldSpace>, u32) -> Ptr
        ),
        entry!(
            0x00587670,
            tes_world_space_add_cell(Ptr<TESWorldSpace>, Ptr) -> bool
        ),
        entry!(
            0x00587760,
            tes_world_space_release_cell(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(0x005877e0, fn_005877e0(Ptr<TESWorldSpace>)),
        entry!(0x00587870, fn_00587870(Ptr<TESWorldSpace>, Ptr) -> Ptr),
        entry!(
            0x005878d0,
            tes_world_space_add_multi_bound_ref(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(0x00587bb0, fn_00587bb0(Ptr<TESWorldSpace>)),
        entry!(0x00587c80, tes_world_space_is_fixed_ref(Ptr) -> bool),
        entry!(
            0x00587d10,
            tes_world_space_add_to_persistent_ref_data(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(
            0x00587e40,
            tes_world_space_remove_from_persistent_ref_data(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(0x00587f40, fn_00587f40(Ptr<TESWorldSpace>)),
        entry!(
            0x00587ff0,
            tes_world_space_add_persistent_ref(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(
            0x00588030,
            tes_world_space_remove_persistent_ref(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(
            0x00588070,
            tes_world_space_create_persistent_cell(Ptr<TESWorldSpace>) -> Ptr
        ),
        entry!(0x00588120, fn_00588120(Ptr<TESWorldSpace>)),
        entry!(
            0x00588150,
            tes_world_space_assign_persistent_refs_to_cell(Ptr<TESWorldSpace>, Ptr)
        ),
        entry!(0x00588270, fn_00588270(Ptr<TESWorldSpace>, u32)),
        entry!(
            0x005882a0,
            tes_world_space_build_map_marker_list(Ptr<TESWorldSpace>, u8) -> Ptr
        ),
        entry!(0x005883c0, fn_005883c0(Ptr<TESWorldSpace>, u8) -> Ptr),
        entry!(0x005884e0, fn_005884e0(Ptr<TESWorldSpace>, Ptr)),
        entry!(0x00588520, fn_00588520(Ptr<TESWorldSpace>, Ptr)),
        entry!(0x00588560, fn_00588560(Ptr<TESWorldSpace>, Ptr) -> Ptr),
        entry!(
            0x005885f0,
            fn_005885f0(Ptr<TESWorldSpace>, Ptr, f32, Ptr, f32, u32, u32)
        ),
        entry!(0x00588a40, fn_00588a40(Ptr<TESWorldSpace>, u8) -> bool),
        entry!(0x00588a60, fn_00588a60(u8) -> bool),
        entry!(0x00588a90, fn_00588a90(Ptr<TESWorldSpace>, Ptr) -> Ptr),
        entry!(
            0x00588b00,
            tes_world_space_create_offset_data(Ptr<TESWorldSpace>, Ptr) -> Ptr<OffsetData>
        ),
        entry!(0x00588c50, fn_00588c50(Ptr<OffsetData>) -> Ptr<OffsetData>),
        entry!(0x00588c80, fn_00588c80(Ptr<TESWorldSpace>)),
        entry!(0x00588d10, fn_00588d10(Ptr<TESWorldSpace>, Ptr) -> bool),
        entry!(
            0x00588e40,
            tes_world_space_adjust_map_marker_coord(Ptr<TESWorldSpace>, Ptr, u8)
        ),
        entry!(0x00588fa0, fn_00588fa0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00588fd0, fn_00588fd0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589000, fn_00589000(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589030, fn_00589030(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589060, fn_00589060(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589090, fn_00589090(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x005890c0, fn_005890c0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589130, fn_00589130(Ptr<NiTPointerMap>)),
        entry!(0x00589190, fn_00589190(Ptr<NiTPointerMap>)),
        entry!(0x005891c0, fn_005891c0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589230, fn_00589230(Ptr<NiTPointerMap>)),
        entry!(0x00589290, fn_00589290(Ptr<NiTPointerMap>)),
        entry!(0x005892c0, fn_005892c0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589330, fn_00589330(Ptr<NiTPointerMap>)),
        entry!(0x00589390, fn_00589390(Ptr<NiTPointerMap>)),
        entry!(0x005893c0, fn_005893c0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x005893f0, fn_005893f0(Ptr<NiTPointerMap>, u32) -> Ptr),
        entry!(0x00589420, fn_00589420(Ptr<NiTPointerMap>, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn ret_float(v: f32) -> Ret {
        Ret {
            st0: v as f64,
            ..Ret::default()
        }
    }

    /// An engine with the callees every function reaches: `__RTDynamicCast`
    /// (the cast always succeeds), the `+0xC` word getter and the global
    /// record tags, which the exe would have initialized.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(WORD_AT_0C, |e, a| ret(e.mem.u32(a[0] + 0xC)));
        for (addr, value) in [
            (GROUP_TAG, 0x5055_5247u32),
            (CELL_TAG, 0x4c4c_4543),
            (SKIPPED_RECORD_TAG_01187338, 0x4552_4652),
            (WORLD_SPACE_RECORD_TYPE, 0x41),
        ] {
            set_word(&mut e, addr, value);
        }
        e
    }

    fn set_word(e: &mut Engine, addr: u32, value: u32) {
        if !e.mem.is_mapped(addr) {
            e.map(addr & !0xfff, 0x1000);
        }
        e.set_global(addr, value);
    }

    /// A double that always returns `value` in EAX.
    fn stub(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    /// A double that does nothing.
    fn noop(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    fn world(e: &mut Engine) -> Ptr<TESWorldSpace> {
        e.new_object()
    }

    /// Gives `object` a vtable whose slots (byte offsets) go to `targets`.
    fn give_vtable(e: &mut Engine, object: u32, targets: &[(u32, u32)]) {
        let size = targets.iter().map(|(s, _)| s + 4).max().unwrap_or(4);
        let table = e.mem.alloc(size);
        for (slot, target) in targets {
            e.mem.set_u32(table + slot, *target);
        }
        e.mem.set_u32(object, table);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn logged(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The plugin reader: a script of chunks (tag, data), served the way the
    /// chunk functions of `TESFile` serve them.
    #[derive(Default)]
    struct Reader {
        chunks: Vec<(u32, Vec<u8>)>,
        position: usize,
        swapped: bool,
    }

    fn install_reader(e: &mut Engine, chunks: Vec<(u32, Vec<u8>)>) -> Rc<RefCell<Reader>> {
        let reader = Rc::new(RefCell::new(Reader {
            chunks,
            ..Reader::default()
        }));
        let r = reader.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let r = r.borrow();
            ret(r.chunks.get(r.position).map_or(0, |c| c.0))
        });
        let r = reader.clone();
        e.register_double(FILE_ADVANCE_CHUNK, move |_, _| {
            let mut r = r.borrow_mut();
            r.position += 1;
            ret((r.position < r.chunks.len()) as u32)
        });
        let r = reader.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let r = r.borrow();
            ret(r.chunks[r.position].1.len() as u32)
        });
        let r = reader.clone();
        e.register_double(FILE_READ_CHUNK, move |e, a| {
            let r = r.borrow();
            let data = &r.chunks[r.position].1;
            // A size of 0 means the whole chunk.
            let size = if a[2] == 0 {
                data.len()
            } else {
                (a[2] as usize).min(data.len())
            };
            e.mem.write(a[1], &data[..size]);
            ret(size as u32)
        });
        let r = reader.clone();
        e.register_double(FILE_READ_CHUNK_U32, move |e, a| {
            let r = r.borrow();
            let data = &r.chunks[r.position].1;
            e.mem.write(a[1], &data[..4]);
            Ret::default()
        });
        let r = reader.clone();
        e.register_double(FILE_READ_CHUNK_U16, move |e, a| {
            let r = r.borrow();
            let data = &r.chunks[r.position].1;
            e.mem.write(a[1], &data[..2]);
            Ret::default()
        });
        let r = reader.clone();
        e.register_double(FILE_NEEDS_SWAP, move |_, _| ret(r.borrow().swapped as u32));
        reader
    }

    fn floats(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// `min` and `max` as `0040ebd0` and `00404010` compute them.
    fn install_min_max(e: &mut Engine) {
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            ret_float(if y <= x { y } else { x })
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            ret_float(if x <= y { y } else { x })
        });
    }

    // ---- accessors ------------------------------------------------------

    #[test]
    fn parent_world_simple_returns_the_field() {
        let mut e = engine();
        let w = world(&mut e);
        e.set(w, TESWorldSpace::pParentWorld, Ptr::new(0x1234));
        assert_eq!(e.call(0x004f_d380, &args![w]).u32(), 0x1234);
    }

    #[test]
    fn plain_setters_and_getters() {
        let mut e = engine();
        let w = world(&mut e);
        e.call(0x0058_4150, &args![w, 12.5f32]);
        assert_eq!(e.get(w, TESWorldSpace::fLODWaterHeight), 12.5);
        e.call(0x0058_4bc0, &args![w, 0x7777u32]);
        assert_eq!(e.get(w, TESWorldSpace::pEncounterZone).addr(), 0x7777);
        e.call(0x0058_6110, &args![w, -3.0f32]);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultLandHeight), -3.0);
        e.call(0x0058_6130, &args![w, 4.5f32]);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultWaterHeight), 4.5);
        e.set(w, TESWorldSpace::pMusicType, Ptr::new(0x4242));
        assert_eq!(e.call(0x0058_6150, &args![w]).u32(), 0x4242);
    }

    #[test]
    fn flag_accessors_read_and_write_their_bit() {
        let mut e = engine();
        let w = world(&mut e);
        // (getter, bit)
        let getters = [
            (0x0058_61b0u32, 0x01u8),
            (0x0058_6210, 0x02),
            (0x0058_6260, 0x08),
            (0x0058_62c0, 0x10),
            (0x0058_62e0, 0x20),
            (0x0058_6300, 0x80),
        ];
        for (getter, bit) in getters {
            e.set(w, TESWorldSpace::cFlags, 0);
            assert!(!e.call(getter, &args![w]).bool(), "{getter:08x} clear");
            e.set(w, TESWorldSpace::cFlags, bit);
            assert!(e.call(getter, &args![w]).bool(), "{getter:08x} set");
            // Every other bit alone does not set it.
            e.set(w, TESWorldSpace::cFlags, !bit);
            assert!(!e.call(getter, &args![w]).bool(), "{getter:08x} others");
        }
        // The bit 6 accessor is the inverse.
        e.set(w, TESWorldSpace::cFlags, 0);
        assert!(e.call(0x0058_6320, &args![w]).bool());
        e.set(w, TESWorldSpace::cFlags, 0x40);
        assert!(!e.call(0x0058_6320, &args![w]).bool());
        // The setters keep the other bits.
        e.set(w, TESWorldSpace::cFlags, 0xA5);
        e.call(0x0058_61d0, &args![w, false]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0xA4);
        e.call(0x0058_61d0, &args![w, true]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0xA5);
        e.call(0x0058_6280, &args![w, true]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0xAD);
        e.call(0x0058_6280, &args![w, false]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0xA5);
    }

    #[test]
    fn form_flag_0x80000_is_read_through_the_flags_getter() {
        let mut e = engine();
        let w = world(&mut e);
        stub(&mut e, FORM_FLAGS, 0x0008_0000);
        assert!(e.call(0x0058_6230, &args![w]).bool());
        stub(&mut e, FORM_FLAGS, 0xfff7_ffff);
        assert!(!e.call(0x0058_6230, &args![w]).bool());
    }

    #[test]
    fn uses_parent_value_checks_one_bit_of_the_parent_flags() {
        let mut e = engine();
        let w = world(&mut e);
        e.set(w, TESWorldSpace::sParentUseFlags, 0b0001_0100);
        for (which, expected) in [
            (0, false),
            (1, false),
            (2, true),
            (3, false),
            (4, true),
            (5, false),
        ] {
            assert_eq!(
                e.call(0x0058_6340, &args![w, which as u32]).bool(),
                expected,
                "{which}"
            );
        }
        // Outside 0..6 the answer is true whatever the flags are.
        e.set(w, TESWorldSpace::sParentUseFlags, 0);
        for which in [-1i32, 6, 100] {
            assert!(e.call(0x0058_6340, &args![w, which]).bool(), "{which}");
        }
    }

    /// `00586390` stand-in: a table of (world, value number) -> parent.
    fn install_parents(e: &mut Engine, table: Vec<(u32, u32, u32)>) {
        e.register_double(PARENT_FOR, move |_, a| {
            ret(table
                .iter()
                .find(|(w, n, _)| *w == a[0] && *n == a[1])
                .map_or(0, |(_, _, p)| *p))
        });
    }

    #[test]
    fn inherited_getters_follow_the_parent_that_owns_the_value() {
        let mut e = engine();
        let child = world(&mut e);
        let parent = world(&mut e);
        let default_water = 0x00dd_0001;
        set_word(&mut e, DEFAULT_WATER, default_water);
        install_parents(
            &mut e,
            vec![
                (child.addr(), 4, parent.addr()),
                (child.addr(), 5, parent.addr()),
                (child.addr(), 3, parent.addr()),
                (child.addr(), 1, parent.addr()),
            ],
        );
        e.set(child, TESWorldSpace::pClimate, Ptr::new(0xc1));
        e.set(parent, TESWorldSpace::pClimate, Ptr::new(0xc2));
        assert_eq!(e.call(0x0058_5fe0, &args![child]).u32(), 0xc2);
        assert_eq!(e.call(0x0058_5fe0, &args![parent]).u32(), 0xc2);

        // Image space: the own one first, the parent's only when it is null.
        e.set(child, TESWorldSpace::pImageSpace, Ptr::new(0x15));
        e.set(parent, TESWorldSpace::pImageSpace, Ptr::new(0x25));
        assert_eq!(e.call(0x0058_6020, &args![child]).u32(), 0x15);
        e.set(child, TESWorldSpace::pImageSpace, Ptr::NULL);
        assert_eq!(e.call(0x0058_6020, &args![child]).u32(), 0x25);
        e.set(parent, TESWorldSpace::pImageSpace, Ptr::NULL);
        assert_eq!(e.call(0x0058_6020, &args![child]).u32(), 0);

        // Water and lod water: parent, else own, else the default water.
        e.set(parent, TESWorldSpace::pWorldWater, Ptr::new(0x31));
        assert_eq!(e.call(0x0058_6070, &args![child]).u32(), 0x31);
        e.set(parent, TESWorldSpace::pWorldWater, Ptr::NULL);
        assert_eq!(e.call(0x0058_6070, &args![child]).u32(), default_water);
        e.set(parent, TESWorldSpace::pWorldWater, Ptr::new(0x32));
        assert_eq!(e.call(0x0058_6070, &args![parent]).u32(), 0x32);
        e.set(parent, TESWorldSpace::pLODWater, Ptr::new(0x41));
        assert_eq!(e.call(0x0058_60c0, &args![child]).u32(), 0x41);
        e.set(parent, TESWorldSpace::pLODWater, Ptr::NULL);
        assert_eq!(e.call(0x0058_60c0, &args![child]).u32(), default_water);
        e.set(parent, TESWorldSpace::pLODWater, Ptr::new(0x42));
        assert_eq!(e.call(0x0058_60c0, &args![parent]).u32(), 0x42);

        // Terrain manager.
        e.set(child, TESWorldSpace::pTerrainManager, Ptr::new(0x51));
        e.set(parent, TESWorldSpace::pTerrainManager, Ptr::new(0x52));
        assert_eq!(e.call(0x0058_6170, &args![child]).u32(), 0x52);
        assert_eq!(e.call(0x0058_6170, &args![parent]).u32(), 0x52);
    }

    #[test]
    fn a_world_without_a_parent_reads_its_own_values() {
        let mut e = engine();
        let w = world(&mut e);
        install_parents(&mut e, vec![]);
        e.set(w, TESWorldSpace::pClimate, Ptr::new(0xc1));
        e.set(w, TESWorldSpace::pTerrainManager, Ptr::new(0x51));
        assert_eq!(e.call(0x0058_5fe0, &args![w]).u32(), 0xc1);
        assert_eq!(e.call(0x0058_6170, &args![w]).u32(), 0x51);
    }

    // ---- record loading and saving -----------------------------------------

    /// A texture and the path it was given.
    type NamedTexture = (u32, Vec<u8>);

    /// Swaps a value in place, as the endian swap helpers do.
    fn install_swaps(e: &mut Engine) {
        e.register(SWAP_DWORD, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v.swap_bytes());
            Ret::default()
        });
        e.register(SWAP_WORD, |e, a| {
            let v = e.mem.u16(a[0]);
            e.mem.set_u16(a[0], v.swap_bytes());
            Ret::default()
        });
    }

    fn corner_world(e: &mut Engine) -> Ptr<TESWorldSpace> {
        let w = world(e);
        e.set(w, TESWorldSpace::MinimumCoords_x, f32::MAX);
        e.set(w, TESWorldSpace::MinimumCoords_y, f32::MAX);
        e.set(w, TESWorldSpace::MaximumCoords_x, -f32::MAX);
        e.set(w, TESWorldSpace::MaximumCoords_y, -f32::MAX);
        w
    }

    #[test]
    fn load_partial_widens_the_corners_and_records_them_for_a_master() {
        let mut e = engine();
        install_min_max(&mut e);
        noop(&mut e, LOCAL_STRUCT_CONSTRUCT);
        stub(&mut e, FILE_IS_MASTER, 1);
        let data: Ptr<OffsetData> = e.new_object();
        stub(&mut e, CREATE_OFFSET_DATA, data.addr());
        let w = corner_world(&mut e);
        install_reader(
            &mut e,
            vec![
                (CHUNK_MIN_COORDS, floats(&[-10.0, -20.0])),
                (CHUNK_DATA, words(&[1])),
                (CHUNK_MAX_COORDS, floats(&[30.0, 40.0])),
            ],
        );
        logged(&mut e);
        assert!(e.call(0x0058_3360, &args![w, 0xf11eu32]).bool());
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_x), -10.0);
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_y), -20.0);
        assert_eq!(e.get(w, TESWorldSpace::MaximumCoords_x), 30.0);
        assert_eq!(e.get(w, TESWorldSpace::MaximumCoords_y), 40.0);
        assert_eq!(e.get(data, OffsetData::OffsetMinCoords_x), -10.0);
        assert_eq!(e.get(data, OffsetData::OffsetMinCoords_y), -20.0);
        assert_eq!(e.get(data, OffsetData::OffsetMaxCoords_x), 30.0);
        assert_eq!(e.get(data, OffsetData::OffsetMaxCoords_y), 40.0);
        // The file's own corners are recorded for each of the two chunks.
        assert_eq!(calls_to(&e, CREATE_OFFSET_DATA).len(), 2);
        assert_eq!(calls_to(&e, CREATE_OFFSET_DATA)[0], vec![w.addr(), 0xf11e]);
    }

    #[test]
    fn load_partial_keeps_a_wider_world_and_ignores_the_offset_data_of_a_plugin() {
        let mut e = engine();
        install_min_max(&mut e);
        noop(&mut e, LOCAL_STRUCT_CONSTRUCT);
        stub(&mut e, FILE_IS_MASTER, 0);
        let w = corner_world(&mut e);
        e.set(w, TESWorldSpace::MinimumCoords_x, -50.0);
        e.set(w, TESWorldSpace::MinimumCoords_y, 5.0);
        e.set(w, TESWorldSpace::MaximumCoords_x, 100.0);
        e.set(w, TESWorldSpace::MaximumCoords_y, 6.0);
        install_reader(
            &mut e,
            vec![
                (CHUNK_MIN_COORDS, floats(&[-10.0, -20.0])),
                (CHUNK_MAX_COORDS, floats(&[30.0, 40.0])),
            ],
        );
        logged(&mut e);
        assert!(e.call(0x0058_3360, &args![w, 0xf11eu32]).bool());
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_x), -50.0);
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_y), -20.0);
        assert_eq!(e.get(w, TESWorldSpace::MaximumCoords_x), 100.0);
        assert_eq!(e.get(w, TESWorldSpace::MaximumCoords_y), 40.0);
        assert!(calls_to(&e, CREATE_OFFSET_DATA).is_empty());
    }

    #[test]
    fn load_partial_swaps_the_corners_of_a_big_endian_file() {
        let mut e = engine();
        install_min_max(&mut e);
        install_swaps(&mut e);
        noop(&mut e, LOCAL_STRUCT_CONSTRUCT);
        stub(&mut e, FILE_IS_MASTER, 0);
        let w = corner_world(&mut e);
        let reader = install_reader(
            &mut e,
            vec![(
                CHUNK_MIN_COORDS,
                [(-10.0f32).to_be_bytes(), 7.0f32.to_be_bytes()].concat(),
            )],
        );
        reader.borrow_mut().swapped = true;
        assert!(e.call(0x0058_3360, &args![w, 0xf11eu32]).bool());
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_x), -10.0);
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_y), 7.0);
    }

    #[test]
    fn load_partial_of_an_empty_record_does_nothing() {
        let mut e = engine();
        let w = corner_world(&mut e);
        install_reader(&mut e, vec![]);
        assert!(e.call(0x0058_3360, &args![w, 0xf11eu32]).bool());
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_x), f32::MAX);
    }

    /// An engine for `Load`: a reader over `chunks`, benign doubles for
    /// everything the chunks call, and a world space whose cell map is not
    /// empty.
    fn load_engine(
        chunks: Vec<(u32, Vec<u8>)>,
    ) -> (Engine, Ptr<TESWorldSpace>, Rc<RefCell<Reader>>) {
        let mut e = engine();
        install_min_max(&mut e);
        install_swaps(&mut e);
        let reader = install_reader(&mut e, chunks);
        stub(&mut e, FILE_RECORD_TYPE, RECORD_TYPE_WORLD_SPACE);
        stub(&mut e, FILE_IS_MASTER, 0);
        for addr in [
            FORM_LOAD,
            FORM_SET_STATE,
            SET_CLIMATE,
            SET_IMAGE_SPACE,
            SET_WATER,
            SET_LOD_WATER,
            SET_MUSIC_TYPE,
            SET_PARENT_WORLD,
            LOCAL_STRUCT_CONSTRUCT,
            TEXTURE_LOAD_CHUNK,
            FULL_NAME_LOAD,
            TEXTURE_SET_NAME,
            SWAP_PAIR,
            SWAP_MAP_OFFSET_BLOCK,
            IMPACT_SWAP_LOAD,
        ] {
            noop(&mut e, addr);
        }
        let w = corner_world(&mut e);
        let map: Ptr<crate::types::NiTPointerMap> = e.new_object();
        e.set(map, crate::types::NiTPointerMap::m_uiCount, 3);
        e.set(w, TESWorldSpace::pCellMap, map.cast());
        (e, w, reader)
    }

    #[test]
    fn load_skips_records_of_other_types() {
        let (mut e, w, _) = load_engine(vec![(CHUNK_CLIMATE, words(&[1]))]);
        stub(&mut e, FILE_RECORD_TYPE, 0x42);
        logged(&mut e);
        assert!(!e.call(0x0058_3560, &args![w, 0xf11eu32]).bool());
        assert!(calls_to(&e, FORM_LOAD).is_empty());
        assert!(calls_to(&e, SET_CLIMATE).is_empty());
    }

    #[test]
    fn load_reads_the_form_references_and_scalars() {
        let (mut e, w, _) = load_engine(vec![
            (CHUNK_DATA, words(&[0x1234_5678])),
            (CHUNK_CLIMATE, words(&[0xa1])),
            (CHUNK_WATER, words(&[0xa2])),
            (CHUNK_LOD_WATER, words(&[0xa3])),
            (CHUNK_LOD_WATER_HEIGHT, floats(&[99.5])),
            (CHUNK_IMAGE_SPACE, words(&[0xa4])),
            (CHUNK_PARENT_USE_FLAGS, vec![5, 0]),
            (CHUNK_PARENT_WORLD, words(&[0xa5])),
            (CHUNK_MUSIC_TYPE, words(&[0xa6])),
            (CHUNK_ENCOUNTER_ZONE, words(&[0xa7])),
            (CHUNK_SKIPPED, words(&[0xa8])),
            (CHUNK_DEFAULT_HEIGHTS, floats(&[1.5, 2.5])),
            (tag(b"JUNK"), words(&[0])),
        ]);
        logged(&mut e);
        let file = 0xf11eu32;
        assert!(e.call(0x0058_3560, &args![w, file]).bool());
        assert_eq!(calls_to(&e, FORM_LOAD), vec![vec![w.addr(), file]]);
        assert_eq!(calls_to(&e, FORM_SET_STATE), vec![vec![w.addr(), 0]]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0x78);
        assert_eq!(calls_to(&e, SET_CLIMATE), vec![vec![w.addr(), 0xa1]]);
        assert_eq!(calls_to(&e, SET_WATER), vec![vec![w.addr(), 0xa2]]);
        assert_eq!(calls_to(&e, SET_LOD_WATER), vec![vec![w.addr(), 0xa3]]);
        assert_eq!(e.get(w, TESWorldSpace::fLODWaterHeight), 99.5);
        assert_eq!(calls_to(&e, SET_IMAGE_SPACE), vec![vec![w.addr(), 0xa4]]);
        assert_eq!(calls_to(&e, SET_PARENT_WORLD), vec![vec![w.addr(), 0xa5]]);
        // The parent chunk resets the parent-use flags the earlier chunk set.
        assert_eq!(e.get(w, TESWorldSpace::sParentUseFlags), 0xffff);
        assert_eq!(calls_to(&e, SET_MUSIC_TYPE), vec![vec![w.addr(), 0xa6]]);
        assert_eq!(e.get(w, TESWorldSpace::pEncounterZone).addr(), 0xa7);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultLandHeight), 1.5);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultWaterHeight), 2.5);
    }

    #[test]
    fn load_data_chunk_of_one_byte_is_read_into_the_flags() {
        let (mut e, w, _) = load_engine(vec![(CHUNK_DATA, vec![0x21])]);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0x21);
        assert_eq!(
            calls_to(&e, FILE_READ_CHUNK),
            vec![vec![0xf11e, w.addr() + 0x4C, 1]]
        );
    }

    #[test]
    fn load_parent_use_flags_are_read_as_a_short() {
        let (mut e, w, _) = load_engine(vec![(CHUNK_PARENT_USE_FLAGS, vec![0x2b, 0x01])]);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.get(w, TESWorldSpace::sParentUseFlags), 0x012b);
    }

    #[test]
    fn load_of_a_master_keeps_the_offset_table_and_the_record_position() {
        let (mut e, w, reader) = load_engine(vec![(CHUNK_OFFSETS, words(&[0x100, 0, 0x300]))]);
        stub(&mut e, FILE_IS_MASTER, 1);
        let data: Ptr<OffsetData> = e.new_object();
        let old_table = e.mem.alloc(8);
        e.set(data, OffsetData::pCellFileOffsets, Ptr::new(old_table));
        stub(&mut e, CREATE_OFFSET_DATA, data.addr());
        stub(&mut e, FILE_CURRENT_OFFSET, 0x4_0000);
        logged(&mut e);
        reader.borrow_mut().swapped = false;
        assert!(e.call(0x0058_3560, &args![w, 0xf11eu32]).bool());
        assert_eq!(e.get(data, OffsetData::iFileOffset), 0x4_0000);
        let table = e.get(data, OffsetData::pCellFileOffsets);
        assert_ne!(table.addr(), old_table);
        assert_eq!(e.mem.block_size(old_table), None, "old table freed");
        assert!(e.mem.block_size(table.addr()).unwrap() >= 12);
        assert_eq!(e.mem.u32(table.addr()), 0x100);
        assert_eq!(e.mem.u32(table.addr() + 4), 0);
        assert_eq!(e.mem.u32(table.addr() + 8), 0x300);
        assert!(calls_to(&e, SWAP_DWORD).is_empty());
    }

    #[test]
    fn load_swaps_each_entry_of_a_big_endian_offset_table() {
        let (mut e, w, reader) = load_engine(vec![(
            CHUNK_OFFSETS,
            [0x100u32.to_be_bytes(), 0x2000u32.to_be_bytes()].concat(),
        )]);
        stub(&mut e, FILE_IS_MASTER, 1);
        let data: Ptr<OffsetData> = e.new_object();
        stub(&mut e, CREATE_OFFSET_DATA, data.addr());
        stub(&mut e, FILE_CURRENT_OFFSET, 0);
        reader.borrow_mut().swapped = true;
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        let table = e.get(data, OffsetData::pCellFileOffsets).addr();
        assert_eq!(e.mem.u32(table), 0x100);
        assert_eq!(e.mem.u32(table + 4), 0x2000);
    }

    #[test]
    fn load_with_an_empty_offset_chunk_leaves_the_table_alone() {
        let (mut e, w, _) = load_engine(vec![(CHUNK_OFFSETS, vec![])]);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert!(calls_to(&e, CREATE_OFFSET_DATA).is_empty());
    }

    #[test]
    fn load_text_chunks_go_to_their_setters() {
        let (mut e, w, _) = load_engine(vec![
            (CHUNK_EDITOR_ID, b"Wasteland\0".to_vec()),
            (CHUNK_CANOPY_TEXTURE, b"canopy.dds\0".to_vec()),
            (CHUNK_WATER_NOISE_TEXTURE, b"noise.dds\0".to_vec()),
            (CHUNK_ICON, b"icon\0".to_vec()),
            (CHUNK_FULL_NAME, b"Mojave\0".to_vec()),
            (CHUNK_OBJECT_BOUNDS, vec![0; 12]),
        ]);
        // The editor ID setter (slot 0x134) and the bounds loader (slot 0xe0).
        let seen: Rc<RefCell<Vec<Vec<u8>>>> = Rc::default();
        let s = seen.clone();
        e.register_double(0x0058_3330, move |e, a| {
            s.borrow_mut().push(e.mem.cstr(a[1]));
            ret(1)
        });
        noop(&mut e, 0x0058_0e00);
        give_vtable(
            &mut e,
            w.addr(),
            &[(0x134, 0x0058_3330), (0xE0, 0x0058_0e00)],
        );
        let names: Rc<RefCell<Vec<NamedTexture>>> = Rc::default();
        let n = names.clone();
        e.register_double(TEXTURE_SET_NAME, move |e, a| {
            n.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(*seen.borrow(), vec![b"Wasteland".to_vec()]);
        assert_eq!(
            *names.borrow(),
            vec![
                (w.addr() + 0xD4, b"canopy.dds".to_vec()),
                (w.addr() + 0xE0, b"noise.dds".to_vec()),
            ]
        );
        assert_eq!(
            calls_to(&e, TEXTURE_LOAD_CHUNK),
            vec![vec![w.addr() + 0x24, 0xf11e]]
        );
        assert_eq!(
            calls_to(&e, FULL_NAME_LOAD),
            vec![vec![w.addr() + 0x18, 0xf11e]]
        );
        assert_eq!(calls_to(&e, 0x0058_0e00), vec![vec![w.addr(), 0xf11e]]);
    }

    #[test]
    fn load_empty_texture_name_chunks_set_nothing() {
        let (mut e, w, _) = load_engine(vec![
            (CHUNK_CANOPY_TEXTURE, vec![]),
            (CHUNK_WATER_NOISE_TEXTURE, vec![]),
        ]);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert!(calls_to(&e, TEXTURE_SET_NAME).is_empty());
    }

    #[test]
    fn load_map_blocks_are_copied_and_swapped_for_big_endian_files() {
        let map_data: Vec<u8> = (1..=16).collect();
        let offset_data: Vec<u8> = (101..=112).collect();
        let (mut e, w, reader) = load_engine(vec![
            (CHUNK_MAP_DATA, map_data.clone()),
            (CHUNK_MAP_OFFSET_DATA, offset_data.clone()),
        ]);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.mem.bytes(w.addr() + 0x80, 16), map_data);
        assert_eq!(e.mem.bytes(w.addr() + 0x90, 12), offset_data);
        assert!(calls_to(&e, SWAP_DWORD).is_empty());
        assert!(calls_to(&e, SWAP_MAP_OFFSET_BLOCK).is_empty());

        reader.borrow_mut().swapped = true;
        reader.borrow_mut().position = 0;
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        // The map data swap: two words and four shorts.
        assert_eq!(calls_to(&e, SWAP_DWORD).len(), 2);
        assert_eq!(calls_to(&e, SWAP_WORD).len(), 4);
        assert_eq!(
            calls_to(&e, SWAP_MAP_OFFSET_BLOCK),
            vec![vec![w.addr() + 0x90]]
        );
    }

    #[test]
    fn load_default_heights_are_swapped_when_the_file_is() {
        let (mut e, w, reader) = load_engine(vec![(CHUNK_DEFAULT_HEIGHTS, floats(&[1.0, 2.0]))]);
        reader.borrow_mut().swapped = true;
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(calls_to(&e, SWAP_PAIR).len(), 1);
    }

    #[test]
    fn load_corner_chunks_are_those_of_load_partial() {
        let (mut e, w, _) = load_engine(vec![
            (CHUNK_MIN_COORDS, floats(&[-1.0, -2.0])),
            (CHUNK_MAX_COORDS, floats(&[3.0, 4.0])),
        ]);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.get(w, TESWorldSpace::MinimumCoords_x), -1.0);
        assert_eq!(e.get(w, TESWorldSpace::MaximumCoords_y), 4.0);
    }

    #[test]
    fn load_creates_the_impact_swap_once() {
        let (mut e, w, _) = load_engine(vec![
            (CHUNK_IMPACT_FIRST, vec![]),
            (CHUNK_IMPACT_SECOND, vec![]),
        ]);
        e.register(IMPACT_SWAP_CONSTRUCT, |_, a| ret(a[0]));
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        let swap = e.get(w, TESWorldSpace::pImpactSwap);
        assert!(!swap.is_null());
        assert!(e.mem.block_size(swap.addr()).unwrap() >= IMPACT_SWAP_SIZE);
        assert_eq!(calls_to(&e, IMPACT_SWAP_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&e, IMPACT_SWAP_LOAD),
            vec![vec![swap.addr(), 0xf11e]; 2]
        );
    }

    #[test]
    fn load_replaces_an_empty_cell_map() {
        for (large, buckets) in [(true, 0x1b59u32), (false, 0x2bd)] {
            let (mut e, w, _) = load_engine(vec![]);
            let old: Ptr<crate::types::NiTPointerMap> = e.new_object();
            e.set(w, TESWorldSpace::pCellMap, old.cast());
            give_vtable(&mut e, old.addr(), &[(0, 0x0058_9060)]);
            noop(&mut e, 0x0058_9060);
            stub(&mut e, FORM_QUERY_LARGE_MAP, large as u32);
            e.register(CELL_MAP_CONSTRUCT, |_, a| ret(a[0]));
            logged(&mut e);
            assert!(e.call(0x0058_3560, &args![w, 0xf11eu32]).bool());
            // The old map is deleted through its destructor (delete flag 1).
            assert_eq!(calls_to(&e, 0x0058_9060), vec![vec![old.addr(), 1]]);
            let new_map = e.get(w, TESWorldSpace::pCellMap);
            assert_ne!(new_map, old.cast());
            assert!(e.mem.block_size(new_map.addr()).unwrap() >= CELL_MAP_SIZE);
            assert_eq!(
                calls_to(&e, CELL_MAP_CONSTRUCT),
                vec![vec![new_map.addr(), buckets]]
            );
        }
    }

    #[test]
    fn load_keeps_a_cell_map_that_has_cells_or_a_world_that_has_no_cells() {
        // Cells in the map.
        let (mut e, w, _) = load_engine(vec![]);
        let map = e.get(w, TESWorldSpace::pCellMap);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.get(w, TESWorldSpace::pCellMap), map);
        assert!(calls_to(&e, CELL_MAP_CONSTRUCT).is_empty());
        // An empty map, but cFlags bit 0 is set.
        let (mut e, w, _) = load_engine(vec![]);
        let empty: Ptr<crate::types::NiTPointerMap> = e.new_object();
        e.set(w, TESWorldSpace::pCellMap, empty.cast());
        e.set(w, TESWorldSpace::cFlags, 1);
        logged(&mut e);
        e.call(0x0058_3560, &args![w, 0xf11eu32]);
        assert_eq!(e.get(w, TESWorldSpace::pCellMap), empty.cast());
        assert!(calls_to(&e, CELL_MAP_CONSTRUCT).is_empty());
    }

    #[test]
    fn map_data_swap_covers_two_words_and_four_shorts() {
        let mut e = engine();
        logged(&mut e);
        noop(&mut e, SWAP_DWORD);
        noop(&mut e, SWAP_WORD);
        e.call(0x0058_40b0, &args![0x9000u32]);
        assert_eq!(
            calls_to(&e, SWAP_DWORD),
            vec![vec![0x9000, 0], vec![0x9004, 0]]
        );
        assert_eq!(
            calls_to(&e, SWAP_WORD),
            vec![
                vec![0x9008, 0],
                vec![0x900a, 0],
                vec![0x900c, 0],
                vec![0x900e, 0]
            ]
        );
    }

    // ---- Save ---------------------------------------------------------------

    /// A form with `id` at `+0xC`.
    fn form_with_id(e: &mut Engine, id: u32) -> Ptr {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xC, id);
        Ptr::new(form)
    }

    /// The chunk writes `Save` made, as (function, tag, first value).
    fn chunk_writes(e: &Engine) -> Vec<(u32, u32, u32)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| {
                [
                    ADD_CHUNK_U32,
                    ADD_CHUNK_U16,
                    ADD_CHUNK_U8,
                    ADD_CHUNK_DATA,
                    ADD_CHUNK_BLOCK,
                    ADD_CHUNK_ARRAY,
                ]
                .contains(a)
            })
            .map(|(a, args)| (*a, args[0], args[1]))
            .collect()
    }

    fn save_engine() -> Engine {
        let mut e = engine();
        install_parents(&mut e, vec![]);
        for addr in [
            FORM_START,
            FORM_CLOSE,
            FULL_NAME_SAVE,
            ADD_CHUNK_U32,
            ADD_CHUNK_U16,
            ADD_CHUNK_U8,
            ADD_CHUNK_DATA,
            ADD_CHUNK_BLOCK,
            ADD_CHUNK_ARRAY,
            TEXTURE_SAVE,
            SWAP_PAIR,
            SWAP_MAP_OFFSET_BLOCK,
            IMPACT_SWAP_SAVE,
        ] {
            noop(&mut e, addr);
        }
        stub(&mut e, SAVING_SWAPPED, 0);
        stub(&mut e, TEXTURE_NAME_LENGTH, 4);
        stub(&mut e, TEXTURE_NAME_TEXT, 0xbeef);
        e.register(GET_LOD_WATER_HEIGHT, |_, _| ret_float(7.25));
        install_swaps(&mut e);
        set_word(&mut e, DEFAULT_WATER, 0);
        e
    }

    #[test]
    fn save_writes_every_chunk_of_a_world_space_with_its_own_values() {
        let mut e = save_engine();
        let w = world(&mut e);
        let zone = form_with_id(&mut e, 0x11);
        let parent = form_with_id(&mut e, 0x12);
        let climate = form_with_id(&mut e, 0x13);
        let water = form_with_id(&mut e, 0x14);
        let lod_water = form_with_id(&mut e, 0x15);
        let image_space = form_with_id(&mut e, 0x16);
        let music = form_with_id(&mut e, 0x17);
        e.set(w, TESWorldSpace::pEncounterZone, zone);
        e.set(w, TESWorldSpace::pParentWorld, parent);
        e.set(w, TESWorldSpace::sParentUseFlags, 0);
        e.set(w, TESWorldSpace::pClimate, climate);
        e.set(w, TESWorldSpace::pWorldWater, water);
        e.set(w, TESWorldSpace::pLODWater, lod_water);
        e.set(w, TESWorldSpace::pMusicType, music);
        e.set(w, TESWorldSpace::cFlags, 0x21);
        e.set(w, TESWorldSpace::fDefaultLandHeight, -2000.0);
        e.set(w, TESWorldSpace::fDefaultWaterHeight, 3.5);
        stub(&mut e, GET_IMAGE_SPACE_RAW, image_space.addr());
        let swap = 0x5151;
        e.set(w, TESWorldSpace::pImpactSwap, Ptr::new(swap));
        logged(&mut e);
        e.call(0x0058_4170, &args![w]);

        let writes = chunk_writes(&e);
        let expected = vec![
            (ADD_CHUNK_U32, CHUNK_ENCOUNTER_ZONE, 0x11),
            (ADD_CHUNK_U32, CHUNK_PARENT_WORLD, 0x12),
            (ADD_CHUNK_U16, CHUNK_PARENT_USE_FLAGS, 0),
            (ADD_CHUNK_U32, CHUNK_CLIMATE, 0x13),
            (ADD_CHUNK_U32, CHUNK_WATER, 0x14),
            (ADD_CHUNK_U32, CHUNK_LOD_WATER, 0x15),
            (ADD_CHUNK_U32, CHUNK_LOD_WATER_HEIGHT, 7.25f32.to_bits()),
            (ADD_CHUNK_DATA, CHUNK_DEFAULT_HEIGHTS, 0),
            (ADD_CHUNK_DATA, CHUNK_MAP_DATA, w.addr() + 0x80),
            (ADD_CHUNK_DATA, CHUNK_MAP_OFFSET_DATA, w.addr() + 0x90),
            (ADD_CHUNK_U32, CHUNK_IMAGE_SPACE, 0x16),
            (ADD_CHUNK_U8, CHUNK_DATA, 0x21),
            (ADD_CHUNK_ARRAY, CHUNK_MIN_COORDS, w.addr() + 0xA0),
            (ADD_CHUNK_ARRAY, CHUNK_MAX_COORDS, w.addr() + 0xA8),
            (ADD_CHUNK_U32, CHUNK_MUSIC_TYPE, 0x17),
            (ADD_CHUNK_BLOCK, CHUNK_CANOPY_TEXTURE, 0xbeef),
            (ADD_CHUNK_BLOCK, CHUNK_WATER_NOISE_TEXTURE, 0xbeef),
        ];
        // The height block is a stack local: compare everything else.
        assert_eq!(writes.len(), expected.len());
        for (got, want) in writes.iter().zip(&expected) {
            assert_eq!((got.0, got.1), (want.0, want.1));
            if want.1 != CHUNK_DEFAULT_HEIGHTS {
                assert_eq!(got.2, want.2, "tag {:08x}", want.1);
            }
        }
        assert_eq!(calls_to(&e, FORM_START), vec![vec![w.addr()]]);
        assert_eq!(calls_to(&e, FULL_NAME_SAVE), vec![vec![w.addr() + 0x18]]);
        assert_eq!(
            calls_to(&e, TEXTURE_SAVE),
            vec![vec![w.addr() + 0x24, CHUNK_ICON]]
        );
        assert_eq!(calls_to(&e, IMPACT_SWAP_SAVE), vec![vec![swap]]);
        assert_eq!(calls_to(&e, FORM_CLOSE), vec![vec![w.addr()]]);
        // The height block holds the two heights and is 8 bytes.
        let heights = calls_to(&e, ADD_CHUNK_DATA)
            .into_iter()
            .find(|a| a[0] == CHUNK_DEFAULT_HEIGHTS)
            .unwrap();
        assert_eq!(heights[2], 8);
        // The text chunks are the text and its length plus the terminator.
        assert_eq!(
            calls_to(&e, ADD_CHUNK_BLOCK)[0],
            vec![CHUNK_CANOPY_TEXTURE, 0xbeef, 5]
        );
    }

    #[test]
    fn save_leaves_out_the_values_a_child_takes_from_its_parent() {
        let mut e = save_engine();
        let w = world(&mut e);
        let climate = form_with_id(&mut e, 0x13);
        e.set(w, TESWorldSpace::pClimate, climate);
        e.set(w, TESWorldSpace::sParentUseFlags, 0x3f);
        stub(&mut e, GET_IMAGE_SPACE_RAW, 0);
        logged(&mut e);
        e.call(0x0058_4170, &args![w]);
        let tags: Vec<u32> = chunk_writes(&e).iter().map(|c| c.1).collect();
        // Without own values: the map offset block, the flags, the corners
        // and the texture paths are still written.
        assert_eq!(
            tags,
            vec![
                CHUNK_MAP_OFFSET_DATA,
                CHUNK_DATA,
                CHUNK_MIN_COORDS,
                CHUNK_MAX_COORDS,
                CHUNK_CANOPY_TEXTURE,
                CHUNK_WATER_NOISE_TEXTURE,
            ]
        );
        assert!(calls_to(&e, TEXTURE_SAVE).is_empty());
    }

    #[test]
    fn save_skips_unset_references_even_when_the_world_has_its_own_values() {
        let mut e = save_engine();
        let w = world(&mut e);
        e.set(w, TESWorldSpace::sParentUseFlags, 0);
        // No climate, no water at all: the water getters return the
        // default water form, which is null here.
        set_word(&mut e, DEFAULT_WATER, 0);
        stub(&mut e, GET_IMAGE_SPACE_RAW, 0);
        logged(&mut e);
        e.call(0x0058_4170, &args![w]);
        let tags: Vec<u32> = chunk_writes(&e).iter().map(|c| c.1).collect();
        assert_eq!(
            tags,
            vec![
                CHUNK_DEFAULT_HEIGHTS,
                CHUNK_MAP_DATA,
                CHUNK_MAP_OFFSET_DATA,
                CHUNK_DATA,
                CHUNK_MIN_COORDS,
                CHUNK_MAX_COORDS,
                CHUNK_CANOPY_TEXTURE,
                CHUNK_WATER_NOISE_TEXTURE,
            ]
        );
        assert!(calls_to(&e, IMPACT_SWAP_SAVE).is_empty());
    }

    #[test]
    fn save_swaps_the_blocks_around_the_write_when_saving_big_endian() {
        let mut e = save_engine();
        let w = world(&mut e);
        e.set(w, TESWorldSpace::sParentUseFlags, 0);
        set_word(&mut e, DEFAULT_WATER, 0);
        stub(&mut e, GET_IMAGE_SPACE_RAW, 0);
        stub(&mut e, SAVING_SWAPPED, 1);
        logged(&mut e);
        e.call(0x0058_4170, &args![w]);
        // Heights: swapped before and after the write.
        assert_eq!(calls_to(&e, SWAP_PAIR).len(), 2);
        // Map data: six values (two words, four shorts) before and after.
        assert_eq!(calls_to(&e, SWAP_DWORD).len(), 4);
        assert_eq!(calls_to(&e, SWAP_WORD).len(), 8);
        // Map offset data: before and after.
        assert_eq!(
            calls_to(&e, SWAP_MAP_OFFSET_BLOCK),
            vec![vec![w.addr() + 0x90]; 2]
        );
    }

    // ---- the per-file cell offset tables -------------------------------------

    const FILE: u32 = 0xf11e;
    const BASE_OFFSET: u32 = 0x1_0000;

    /// A world space and an `OFFSET_DATA` for `FILE`: a grid of cells x -2..=1,
    /// y -1..=2 (the float bounds are cell numbers times 4096), with the cells
    /// (0, 0) and (1, 2) at offsets 0x500 and 0x900.
    fn grid_engine() -> (Engine, Ptr<TESWorldSpace>, Ptr<OffsetData>) {
        let mut e = engine();
        let w = world(&mut e);
        let data: Ptr<OffsetData> = e.new_object();
        e.set(data, OffsetData::OffsetMinCoords_x, -8192.0);
        e.set(data, OffsetData::OffsetMinCoords_y, -4096.0);
        e.set(data, OffsetData::OffsetMaxCoords_x, 4096.0 + 100.0);
        e.set(data, OffsetData::OffsetMaxCoords_y, 8192.0);
        e.set(data, OffsetData::iFileOffset, BASE_OFFSET);
        let table = e.mem.alloc(16 * 4);
        e.mem.set_u32(table + 6 * 4, 0x500);
        e.mem.set_u32(table + 15 * 4, 0x900);
        e.set(data, OffsetData::pCellFileOffsets, Ptr::new(table));
        let at = data.addr();
        e.register_double(OFFSET_DATA_OF_FILE, move |_, a| {
            ret(if a[1] == FILE { at } else { 0 })
        });
        (e, w, data)
    }

    #[test]
    fn cell_index_is_row_major_inside_the_grid() {
        let (mut e, w, _) = grid_engine();
        let index =
            |e: &mut Engine, x: i32, y: i32| e.call(0x0058_4530, &args![w, FILE, x, y]).i32();
        assert_eq!(index(&mut e, -2, -1), 0);
        assert_eq!(index(&mut e, 1, -1), 3);
        assert_eq!(index(&mut e, -2, 0), 4);
        assert_eq!(index(&mut e, 0, 0), 6);
        assert_eq!(index(&mut e, 1, 2), 15);
        // Outside the grid on each side.
        assert_eq!(index(&mut e, 2, 0), -1);
        assert_eq!(index(&mut e, -3, 0), -1);
        assert_eq!(index(&mut e, 0, 3), -1);
        assert_eq!(index(&mut e, 0, -2), -1);
        // A file without an OFFSET_DATA for the world space.
        assert_eq!(e.call(0x0058_4530, &args![w, 0x9999u32, 0, 0]).i32(), -1);
    }

    #[test]
    fn adjusting_the_offsets_moves_the_cells_that_have_one() {
        let (mut e, w, data) = grid_engine();
        e.call(0x0058_57b0, &args![w, FILE, 0x40u32]);
        let table = e.get(data, OffsetData::pCellFileOffsets).addr();
        assert_eq!(e.mem.u32(table + 6 * 4), 0x540);
        assert_eq!(e.mem.u32(table + 15 * 4), 0x940);
        // Empty entries stay empty.
        assert_eq!(e.mem.u32(table), 0);
        assert_eq!(e.mem.u32(table + 4 * 4), 0);
        // A delta of 0 and a file without data change nothing.
        e.call(0x0058_57b0, &args![w, FILE, 0u32]);
        e.call(0x0058_57b0, &args![w, 0x9999u32, 0x40u32]);
        assert_eq!(e.mem.u32(table + 6 * 4), 0x540);
        // So does a data block without a table.
        e.set(data, OffsetData::pCellFileOffsets, Ptr::NULL);
        e.call(0x0058_57b0, &args![w, FILE, 0x40u32]);
        assert_eq!(e.mem.u32(table + 6 * 4), 0x540);
    }

    /// The reader positioned by `SetOffset`, with a record at each offset.
    #[derive(Default)]
    struct Positioned {
        offset: u32,
        /// (offset, record type, form ID, editor ID)
        records: Vec<(u32, u32, u32, &'static str)>,
        set_offsets: Vec<u32>,
    }

    fn install_positioned(
        e: &mut Engine,
        records: Vec<(u32, u32, u32, &'static str)>,
    ) -> Rc<RefCell<Positioned>> {
        let state = Rc::new(RefCell::new(Positioned {
            records,
            ..Positioned::default()
        }));
        let s = state.clone();
        e.register_double(FILE_SET_OFFSET, move |_, a| {
            let mut s = s.borrow_mut();
            s.offset = a[1];
            s.set_offsets.push(a[1]);
            ret(1)
        });
        let s = state.clone();
        e.register_double(FILE_RECORD_TYPE, move |_, _| {
            let s = s.borrow();
            ret(s
                .records
                .iter()
                .find(|r| r.0 == s.offset)
                .map_or(0, |r| r.1))
        });
        let s = state.clone();
        e.register_double(FILE_CURRENT_FORM_ID, move |_, _| {
            let s = s.borrow();
            ret(s
                .records
                .iter()
                .find(|r| r.0 == s.offset)
                .map_or(0, |r| r.2))
        });
        let s = state.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let s = s.borrow();
            ret(
                if s.records.iter().any(|r| r.0 == s.offset && !r.3.is_empty()) {
                    CHUNK_EDITOR_ID
                } else {
                    CHUNK_DATA
                },
            )
        });
        let s = state.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            ret(s
                .records
                .iter()
                .find(|r| r.0 == s.offset)
                .map_or(0, |r| r.3.len() as u32 + 1))
        });
        let s = state.clone();
        e.register_double(FILE_READ_CHUNK, move |e, a| {
            let s = s.borrow();
            if let Some(r) = s.records.iter().find(|r| r.0 == s.offset) {
                e.mem.write(a[1], r.3.as_bytes());
            }
            Ret::default()
        });
        state
    }

    #[test]
    fn find_in_file_fast_checks_the_world_record_at_the_stored_position() {
        let (mut e, w, data) = grid_engine();
        e.mem.set_u32(w.addr() + 0xC, 0x3c);
        stub(&mut e, FILE_IS_MASTER, 1);
        stub(&mut e, FILE_CURRENT_FORM_ID, 0x3c);
        let state = install_positioned(&mut e, vec![(BASE_OFFSET, 0x41, 0x3c, "")]);
        // The data block's position is the file offset field.
        assert!(e.call(0x0058_58b0, &args![w, FILE]).bool());
        assert_eq!(state.borrow().set_offsets, vec![BASE_OFFSET]);
        // A different form ID in the record.
        let (mut e2, w2, _) = grid_engine();
        e2.mem.set_u32(w2.addr() + 0xC, 0x3d);
        stub(&mut e2, FILE_IS_MASTER, 1);
        install_positioned(&mut e2, vec![(BASE_OFFSET, 0x41, 0x3c, "")]);
        assert!(!e2.call(0x0058_58b0, &args![w2, FILE]).bool());
        // A record of another type.
        let (mut e3, w3, _) = grid_engine();
        e3.mem.set_u32(w3.addr() + 0xC, 0x3c);
        stub(&mut e3, FILE_IS_MASTER, 1);
        install_positioned(&mut e3, vec![(BASE_OFFSET, 0x39, 0x3c, "")]);
        assert!(!e3.call(0x0058_58b0, &args![w3, FILE]).bool());
        // Not a master file, no file, no stored position.
        stub(&mut e, FILE_IS_MASTER, 0);
        assert!(!e.call(0x0058_58b0, &args![w, FILE]).bool());
        stub(&mut e, FILE_IS_MASTER, 1);
        assert!(!e.call(0x0058_58b0, &args![w, 0u32]).bool());
        e.set(data, OffsetData::iFileOffset, 0);
        assert!(!e.call(0x0058_58b0, &args![w, FILE]).bool());
    }

    fn lower(text: &[u8]) -> Vec<u8> {
        text.to_ascii_lowercase()
    }

    #[test]
    fn cell_by_editor_id_searches_each_file_of_the_world_space() {
        let (mut e, w, _) = grid_engine();
        // Two files: the first has no table, the second is `FILE`.
        const NO_TABLE: u32 = 0x5555;
        stub(&mut e, FORM_FILE_COUNT, 2);
        e.register(FORM_GET_FILE, |_, a| {
            ret(if a[1] == 0 { NO_TABLE } else { FILE })
        });
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        let state = install_positioned(
            &mut e,
            vec![
                (BASE_OFFSET + 0x500, RECORD_TYPE_CELL, 1, "OtherCell"),
                (BASE_OFFSET + 0x900, RECORD_TYPE_CELL, 2, "GoodspringsCell"),
            ],
        );
        e.register(EDITOR_ID_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            ret((lower(&x) != lower(&y)) as u32)
        });
        let out_x = e.mem.alloc(4);
        let out_y = e.mem.alloc(4);
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, b"GOODSPRINGSCELL");
        let found = e.call(0x0058_5940, &args![w, name, out_x, out_y]).bool();
        assert!(found);
        assert_eq!(e.mem.i32(out_x), 1);
        assert_eq!(e.mem.i32(out_y), 2);
        // Cells are tried in x-major order: (0, 0) first, then (1, 2).
        assert_eq!(
            state.borrow().set_offsets,
            vec![BASE_OFFSET + 0x500, BASE_OFFSET + 0x900]
        );

        // Not found: the outputs stay as they were.
        e.mem.set_i32(out_x, -77);
        e.mem.set_i32(out_y, -78);
        e.mem.set_cstr(name, b"NoSuchCell");
        let found = e.call(0x0058_5940, &args![w, name, out_x, out_y]).bool();
        assert!(!found);
        assert_eq!(e.mem.i32(out_x), -77);
        assert_eq!(e.mem.i32(out_y), -78);
    }

    #[test]
    fn cell_by_editor_id_ignores_records_that_are_not_cells_or_have_no_editor_id() {
        let (mut e, w, _) = grid_engine();
        stub(&mut e, FORM_FILE_COUNT, 1);
        e.register(FORM_GET_FILE, |_, _| ret(FILE));
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        install_positioned(
            &mut e,
            vec![
                // Not a cell record.
                (BASE_OFFSET + 0x500, 0x41, 1, "Match"),
                // A cell whose first chunk is not the editor ID.
                (BASE_OFFSET + 0x900, RECORD_TYPE_CELL, 2, ""),
            ],
        );
        e.register(EDITOR_ID_COMPARE, |_, _| ret(0));
        let out = e.mem.alloc(8);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Match");
        assert!(!e.call(0x0058_5940, &args![w, name, out, out + 4]).bool());
    }

    // ---- scanning a plugin for an exterior cell --------------------------------

    #[derive(Clone, Copy, Debug)]
    enum Rec {
        Group { kind: u32, label: u32 },
        Cell { x: i32, y: i32, interior: bool },
        Skipped,
        Other,
    }

    #[derive(Default)]
    struct Stream {
        records: Vec<Option<Rec>>,
        position: usize,
        header: u32,
        chunk: usize,
        operations: Vec<&'static str>,
        formed: bool,
    }

    const BLOCK_KEY: u32 = 0xb10c;
    const SUB_BLOCK_KEY: u32 = 0x5b;

    fn write_header(e: &mut Engine, s: &Stream) {
        let h = s.header;
        let (tag_word, at8, at_c) = match s.records.get(s.position).copied().flatten() {
            Some(Rec::Group { kind, label }) => (e.global::<u32>(GROUP_TAG), label, kind),
            Some(Rec::Cell { interior, .. }) => (
                e.global::<u32>(CELL_TAG),
                if interior { 0x400 } else { 0 },
                9,
            ),
            Some(Rec::Skipped) => (e.global::<u32>(SKIPPED_RECORD_TAG_01187338), 0, 0),
            Some(Rec::Other) | None => (0x1234_5678, 0, 0),
        };
        e.mem.set_u32(h, tag_word);
        e.mem.set_u32(h + 8, at8);
        e.mem.set_u32(h + 0xC, at_c);
    }

    /// Installs a reader over `records` (index 0 stands for the world space's
    /// own record, which `FindForm` positions on).
    fn install_stream(e: &mut Engine, records: Vec<Rec>) -> Rc<RefCell<Stream>> {
        let header = e.mem.alloc(0x20);
        let stream = Rc::new(RefCell::new(Stream {
            records: std::iter::once(Some(Rec::Other))
                .chain(records.into_iter().map(Some))
                .collect(),
            header,
            ..Stream::default()
        }));
        noop(e, CELL_COORDS_CONSTRUCT);
        stub(e, FILE_NEEDS_SWAP, 0);
        stub(e, CALC_BLOCK_KEY, BLOCK_KEY);
        stub(e, CALC_SUB_BLOCK_KEY, SUB_BLOCK_KEY);
        stub(e, FILE_CURRENT_RECORD, header);
        let s = stream.clone();
        e.register_double(FILE_FIND_FORM, move |e, _| {
            let mut s = s.borrow_mut();
            s.position = 0;
            s.formed = true;
            write_header(e, &s);
            ret(1)
        });
        for (addr, name) in [(FILE_NEXT_FORM, "next"), (FILE_LEAVE_GROUP, "leave")] {
            let s = stream.clone();
            e.register_double(addr, move |e, _| {
                let mut s = s.borrow_mut();
                s.position += 1;
                s.chunk = 0;
                s.operations.push(name);
                write_header(e, &s);
                ret(1)
            });
        }
        let s = stream.clone();
        e.register_double(FILE_REWIND_RECORD, move |_, _| {
            s.borrow_mut().operations.push("rewind");
            Ret::default()
        });
        let s = stream.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let s = s.borrow();
            ret(match s.records[s.position] {
                Some(Rec::Cell { .. }) if s.chunk == 0 => CHUNK_CELL_GRID,
                Some(Rec::Cell { .. }) if s.chunk == 1 => CHUNK_DATA,
                _ => 0,
            })
        });
        let s = stream.clone();
        e.register_double(FILE_ADVANCE_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.chunk += 1;
            ret((s.chunk < 2) as u32)
        });
        let s = stream.clone();
        e.register_double(FILE_READ_CHUNK, move |e, a| {
            let s = s.borrow();
            if let Some(Rec::Cell { x, y, .. }) = s.records[s.position] {
                e.mem.set_i32(a[1], x);
                e.mem.set_i32(a[1] + 4, y);
                e.mem.set_u32(a[1] + 8, 0);
            }
            Ret::default()
        });
        stream
    }

    /// `FindCellInFile` for a file that has no offset table (the slow path).
    fn scan(records: Vec<Rec>, x: i32, y: i32) -> (bool, Vec<&'static str>) {
        let mut e = engine();
        let w = world(&mut e);
        // A plugin: not a master file.
        stub(&mut e, FILE_IS_MASTER, 0);
        let stream = install_stream(&mut e, records);
        let found = e.call(0x005854f0, &args![w, FILE, x, y]).bool();
        let operations = stream.borrow().operations.clone();
        (found, operations)
    }

    #[test]
    fn scan_finds_the_cell_through_the_groups_that_hold_it() {
        let (found, operations) = scan(
            vec![
                Rec::Group { kind: 1, label: 1 },
                Rec::Group {
                    kind: 4,
                    label: BLOCK_KEY,
                },
                Rec::Group {
                    kind: 5,
                    label: SUB_BLOCK_KEY,
                },
                Rec::Cell {
                    x: 3,
                    y: 4,
                    interior: false,
                },
            ],
            3,
            4,
        );
        assert!(found);
        // The first NextForm steps past the world record; then each group is
        // entered (NextForm) and the cell record ends the scan with a rewind.
        assert_eq!(operations, vec!["next", "next", "next", "next", "rewind"]);
    }

    #[test]
    fn scan_leaves_the_groups_whose_label_is_not_the_cells_block() {
        let (found, operations) = scan(
            vec![
                Rec::Group { kind: 1, label: 1 },
                // An exterior block of another cell.
                Rec::Group {
                    kind: 4,
                    label: 0xdead,
                },
                // A sub-block of another cell.
                Rec::Group {
                    kind: 5,
                    label: 0xbeef,
                },
                Rec::Group {
                    kind: 4,
                    label: SUB_BLOCK_KEY,
                },
                Rec::Cell {
                    x: 3,
                    y: 4,
                    interior: false,
                },
            ],
            3,
            4,
        );
        assert!(found);
        // Block key or sub-block key both match a type 4 group's label.
        assert_eq!(
            operations,
            vec!["next", "next", "leave", "leave", "next", "rewind"]
        );
    }

    #[test]
    fn scan_passes_other_cells_and_the_skipped_records() {
        let (found, operations) = scan(
            vec![
                Rec::Cell {
                    x: 0,
                    y: 0,
                    interior: false,
                },
                Rec::Skipped,
                Rec::Cell {
                    x: 3,
                    y: 4,
                    interior: true,
                },
                Rec::Cell {
                    x: 3,
                    y: 4,
                    interior: false,
                },
            ],
            3,
            4,
        );
        assert!(found);
        // An interior cell never matches, whatever its grid chunk says.
        assert_eq!(operations, vec!["next", "next", "next", "next", "rewind"]);
    }

    #[test]
    fn scan_stops_at_groups_that_end_the_world_and_at_unknown_records() {
        // Group types 2, 3 and 7 and any record that is not a cell or group.
        for stop in [
            Rec::Group { kind: 2, label: 0 },
            Rec::Group { kind: 3, label: 0 },
            Rec::Group { kind: 7, label: 0 },
            Rec::Other,
        ] {
            let (found, operations) = scan(
                vec![
                    stop,
                    Rec::Cell {
                        x: 3,
                        y: 4,
                        interior: false,
                    },
                ],
                3,
                4,
            );
            assert!(!found, "{stop:?}");
            assert_eq!(operations, vec!["next"], "{stop:?}");
        }
        // A group type 0 (and anything above 9) also stops.
        let (found, _) = scan(
            vec![
                Rec::Group { kind: 0, label: 0 },
                Rec::Cell {
                    x: 3,
                    y: 4,
                    interior: false,
                },
            ],
            3,
            4,
        );
        assert!(!found);
    }

    #[test]
    fn scan_leaves_cell_children_groups_by_the_group_exit() {
        // Cell children groups (6, 8, 9) are not entered: the scan leaves them.
        let (found, operations) = scan(
            vec![
                Rec::Group { kind: 6, label: 0 },
                Rec::Group { kind: 8, label: 0 },
                Rec::Group { kind: 9, label: 0 },
                Rec::Cell {
                    x: 1,
                    y: 1,
                    interior: false,
                },
            ],
            1,
            1,
        );
        assert!(found);
        assert_eq!(
            operations,
            vec!["next", "leave", "leave", "leave", "rewind"]
        );
    }

    #[test]
    fn scan_does_nothing_when_the_file_has_no_world_record() {
        let mut e = engine();
        let w = world(&mut e);
        stub(&mut e, FILE_IS_MASTER, 0);
        let stream = install_stream(
            &mut e,
            vec![Rec::Cell {
                x: 1,
                y: 1,
                interior: false,
            }],
        );
        stub(&mut e, FILE_FIND_FORM, 0);
        assert!(!e.call(0x005854f0, &args![w, FILE, 1i32, 1i32]).bool());
        assert!(stream.borrow().operations.is_empty());
        // A null file gives the same.
        assert!(!e.call(0x005854f0, &args![w, 0u32, 1i32, 1i32]).bool());
    }

    #[test]
    fn a_master_file_with_a_table_is_found_by_index() {
        let (mut e, w, _) = grid_engine();
        stub(&mut e, FILE_IS_MASTER, 1);
        let state = install_positioned(&mut e, vec![]);
        logged(&mut e);
        assert!(e.call(0x005854f0, &args![w, FILE, 0i32, 0i32]).bool());
        assert_eq!(state.borrow().set_offsets, vec![BASE_OFFSET + 0x500]);
        // A cell with no entry, and one outside the grid.
        assert!(!e.call(0x005854f0, &args![w, FILE, 1i32, 1i32]).bool());
        assert!(!e.call(0x005854f0, &args![w, FILE, 9i32, 9i32]).bool());
        assert_eq!(state.borrow().set_offsets.len(), 1);
        // No table scan was started.
        assert!(calls_to(&e, FILE_FIND_FORM).is_empty());
    }

    #[test]
    fn a_master_file_whose_table_is_missing_is_scanned() {
        let (mut e, w, data) = grid_engine();
        e.set(data, OffsetData::pCellFileOffsets, Ptr::NULL);
        stub(&mut e, FILE_IS_MASTER, 1);
        let stream = install_stream(
            &mut e,
            vec![Rec::Cell {
                x: 5,
                y: 6,
                interior: false,
            }],
        );
        assert!(e.call(0x005854f0, &args![w, FILE, 5i32, 6i32]).bool());
        assert_eq!(stream.borrow().operations, vec!["next", "rewind"]);
    }

    // ---- loading and unloading exterior cells ------------------------------------

    const SAVE_LOAD: u32 = 0x5a5a;
    const CELL_INIT_ITEM: u32 = 0x0054_f000;
    const CELL_SET_FORM_ID: u32 = 0x0054_f100;
    const WORLD_NAME: u32 = 0x0058_5000;

    /// An engine for `LoadCell`: the world space `0x3c` owns one file, `FILE`,
    /// whose offset table has the cell (0, 0) (a 1 x 1 grid); every cell
    /// function is a double that returns success.
    fn load_cell_engine() -> (Engine, Ptr<TESWorldSpace>) {
        let (mut e, w, data) = grid_engine();
        e.set(data, OffsetData::OffsetMinCoords_x, 0.0);
        e.set(data, OffsetData::OffsetMinCoords_y, 0.0);
        e.set(data, OffsetData::OffsetMaxCoords_x, 0.0);
        e.set(data, OffsetData::OffsetMaxCoords_y, 0.0);
        let table = e.get(data, OffsetData::pCellFileOffsets).addr();
        e.mem.set_u32(table, 0x500);
        e.mem.set_u32(w.addr() + 0xC, 0x3c);
        give_vtable(&mut e, w.addr(), &[(SLOT_EDITOR_ID, WORLD_NAME)]);
        stub(&mut e, WORLD_NAME, 0x7777);
        set_word(&mut e, SAVE_LOAD_GAME, SAVE_LOAD);
        set_word(&mut e, CURRENT_WORLD_SPACE, 0);
        stub(&mut e, FORM_FILE_COUNT, 1);
        e.register(FORM_GET_FILE, |_, _| ret(FILE));
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        stub(&mut e, FILE_IS_MASTER, 1);
        stub(&mut e, FILE_SET_OFFSET, 1);
        // The record header the reader exposes: flags at +8.
        let header = e.mem.alloc(0x20);
        stub(&mut e, FILE_CURRENT_RECORD, header);
        for addr in [
            MEMORY_CONTEXT_ENTER,
            MEMORY_CONTEXT_LEAVE,
            CELL_SET_INTERIOR,
            CELL_CREATE_DATA,
            CELL_SET_DATA_COORD,
            ADD_CELL,
            CELL_SET_HAS_TEMP_DATA,
            SAVE_LOAD_SET_STATE,
            ASSIGN_PERSISTENT_REFS,
            LOG,
            CELL_INIT_ITEM,
            CELL_SET_FORM_ID,
        ] {
            noop(&mut e, addr);
        }
        stub(&mut e, GET_CELL_FROM_CELL_COORD, 0);
        stub(&mut e, CELL_HAS_TEMP_DATA, 0);
        stub(&mut e, SAVE_LOAD_CREATED_CELL_ID, 0);
        stub(&mut e, DATA_HANDLER_LOAD_FORM, 1);
        stub(&mut e, CELL_LOAD_TEMP_DATA, 1);
        stub(&mut e, SAVE_LOAD_GET_STATE, 0);
        e.register(CELL_CONSTRUCT, |e, a| {
            give_vtable(
                e,
                a[0],
                &[(0x88, CELL_INIT_ITEM), (0x128, CELL_SET_FORM_ID)],
            );
            ret(a[0])
        });
        (e, w)
    }

    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    #[test]
    fn load_cell_creates_and_loads_a_missing_cell() {
        let (mut e, w) = load_cell_engine();
        logged(&mut e);
        let cell = e.call(0x0058_5b30, &args![w, 0i32, 0i32]).ptr::<()>();
        assert!(!cell.is_null());
        assert_eq!(
            e.mem.block_size(cell.addr()).map(|s| s >= CELL_SIZE),
            Some(true)
        );
        assert_eq!(e.global::<u32>(CURRENT_WORLD_SPACE), w.addr());
        let c = cell.addr();
        let calls = e.call_log.clone().unwrap();
        let find = |addr: u32| calls.iter().find(|x| x.0 == addr).cloned();
        let enter = find(MEMORY_CONTEXT_ENTER).unwrap().1;
        assert_eq!(&enter[1..], &[0x1a, 1, WORLD_SPACE_SOURCE, 0x796]);
        assert_eq!(find(CELL_SET_INTERIOR).unwrap().1, vec![c, 0]);
        assert_eq!(find(CELL_SET_DATA_COORD).unwrap().1, vec![c, 0, 0]);
        assert_eq!(find(ADD_CELL).unwrap().1, vec![w.addr(), c]);
        assert_eq!(
            find(SAVE_LOAD_CREATED_CELL_ID).unwrap().1,
            vec![SAVE_LOAD, 0x3c, 0, 0]
        );
        assert_eq!(find(DATA_HANDLER_LOAD_FORM).unwrap().1, vec![c, FILE]);
        assert_eq!(find(CELL_LOAD_TEMP_DATA).unwrap().1, vec![c, FILE]);
        assert_eq!(find(ASSIGN_PERSISTENT_REFS).unwrap().1, vec![w.addr(), c]);
        // Order of the work, and the state switch around the cell's InitItem.
        let order: Vec<u32> = addresses(&e)
            .into_iter()
            .filter(|a| {
                [
                    CELL_CONSTRUCT,
                    ADD_CELL,
                    DATA_HANDLER_LOAD_FORM,
                    CELL_LOAD_TEMP_DATA,
                    CELL_SET_HAS_TEMP_DATA,
                    SAVE_LOAD_SET_STATE,
                    CELL_INIT_ITEM,
                    ASSIGN_PERSISTENT_REFS,
                    MEMORY_CONTEXT_LEAVE,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                CELL_CONSTRUCT,
                ADD_CELL,
                DATA_HANDLER_LOAD_FORM,
                CELL_LOAD_TEMP_DATA,
                CELL_SET_HAS_TEMP_DATA,
                SAVE_LOAD_SET_STATE,
                CELL_INIT_ITEM,
                SAVE_LOAD_SET_STATE,
                ASSIGN_PERSISTENT_REFS,
                MEMORY_CONTEXT_LEAVE,
            ]
        );
        // The switch is turned on for the InitItem (the state was off), then
        // put back.
        let states = calls_to(&e, SAVE_LOAD_SET_STATE);
        assert_eq!(states, vec![vec![SAVE_LOAD, 1], vec![SAVE_LOAD, 0]]);
        assert!(calls_to(&e, LOG).is_empty());
    }

    #[test]
    fn load_cell_gives_a_created_cell_the_form_id_the_save_game_made_for_it() {
        let (mut e, w) = load_cell_engine();
        stub(&mut e, SAVE_LOAD_CREATED_CELL_ID, 0x00ff_1234);
        logged(&mut e);
        let cell = e.call(0x0058_5b30, &args![w, 0i32, 0i32]).u32();
        assert_eq!(
            calls_to(&e, CELL_SET_FORM_ID),
            vec![vec![cell, 0x00ff_1234, 1]]
        );
    }

    #[test]
    fn load_cell_restores_the_state_switch_it_found_on() {
        let (mut e, w) = load_cell_engine();
        stub(&mut e, SAVE_LOAD_GET_STATE, 1);
        logged(&mut e);
        e.call(0x0058_5b30, &args![w, 0i32, 0i32]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_SET_STATE),
            vec![vec![SAVE_LOAD, 0], vec![SAVE_LOAD, 1]]
        );
    }

    #[test]
    fn load_cell_reads_the_save_load_game_global_for_each_call() {
        // 00585d86: the state goes back to the object the global holds after InitItem.
        let (mut e, w) = load_cell_engine();
        e.register(CELL_INIT_ITEM, |e, _| {
            e.set_global(SAVE_LOAD_GAME, SAVE_LOAD + 1);
            Ret::default()
        });
        logged(&mut e);
        e.call(0x0058_5b30, &args![w, 0i32, 0i32]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_SET_STATE),
            vec![vec![SAVE_LOAD, 1], vec![SAVE_LOAD + 1, 0]]
        );
    }

    /// An existing cell with the vtable `LoadCell` needs.
    fn existing_cell(e: &mut Engine) -> u32 {
        let cell = e.mem.alloc(CELL_SIZE);
        give_vtable(
            e,
            cell,
            &[(0x88, CELL_INIT_ITEM), (0x128, CELL_SET_FORM_ID)],
        );
        stub(e, GET_CELL_FROM_CELL_COORD, cell);
        cell
    }

    #[test]
    fn load_cell_adds_the_files_record_to_an_existing_cell_unless_flagged() {
        let (mut e, w) = load_cell_engine();
        let cell = existing_cell(&mut e);
        logged(&mut e);
        // Record flag 0x4000 clear: the form is loaded again.
        assert_eq!(e.call(0x0058_5b30, &args![w, 0i32, 0i32]).u32(), cell);
        assert_eq!(calls_to(&e, DATA_HANDLER_LOAD_FORM), vec![vec![cell, FILE]]);
        assert!(calls_to(&e, ADD_CELL).is_empty());
        assert!(calls_to(&e, CELL_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, CELL_LOAD_TEMP_DATA), vec![vec![cell, FILE]]);
        // Flag 0x4000 set in the record header: not loaded again.
        let record = e.call(FILE_CURRENT_RECORD, &args![FILE]).u32();
        e.mem.set_u32(record + 8, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0058_5b30, &args![w, 0i32, 0i32]);
        assert!(calls_to(&e, DATA_HANDLER_LOAD_FORM).is_empty());
        assert_eq!(calls_to(&e, CELL_LOAD_TEMP_DATA).len(), 1);
    }

    #[test]
    fn load_cell_with_temporary_data_already_loaded_does_nothing_more() {
        let (mut e, w) = load_cell_engine();
        let cell = existing_cell(&mut e);
        stub(&mut e, CELL_HAS_TEMP_DATA, 1);
        logged(&mut e);
        assert_eq!(e.call(0x0058_5b30, &args![w, 0i32, 0i32]).u32(), cell);
        assert_eq!(
            addresses(&e),
            vec![
                0x0058_5b30,
                MEMORY_CONTEXT_ENTER,
                GET_CELL_FROM_CELL_COORD,
                CELL_HAS_TEMP_DATA,
                MEMORY_CONTEXT_LEAVE
            ]
        );
    }

    #[test]
    fn load_cell_logs_a_failure_to_load_the_temporary_data() {
        let (mut e, w) = load_cell_engine();
        let cell = existing_cell(&mut e);
        stub(&mut e, CELL_LOAD_TEMP_DATA, 0);
        logged(&mut e);
        // The cell is still returned and initialized.
        assert_eq!(e.call(0x0058_5b30, &args![w, 0i32, 0i32]).u32(), cell);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MSG_CELL_LOAD_FAILED, 0, 0, 0x7777, 0x3c]]
        );
        assert_eq!(calls_to(&e, ASSIGN_PERSISTENT_REFS).len(), 1);
    }

    #[test]
    fn load_cell_logs_when_the_new_cells_form_does_not_load() {
        let (mut e, w) = load_cell_engine();
        stub(&mut e, DATA_HANDLER_LOAD_FORM, 0);
        logged(&mut e);
        e.call(0x0058_5b30, &args![w, 0i32, 0i32]);
        assert_eq!(calls_to(&e, LOG).len(), 1);
    }

    #[test]
    fn load_cell_returns_null_when_no_file_has_the_cell() {
        let (mut e, w) = load_cell_engine();
        // Two files, neither with a thread-safe version.
        stub(&mut e, FORM_FILE_COUNT, 2);
        stub(&mut e, FILE_THREAD_SAFE, 0);
        logged(&mut e);
        assert_eq!(e.call(0x0058_5b30, &args![w, 1i32, 1i32]).u32(), 0);
        assert!(calls_to(&e, CELL_CONSTRUCT).is_empty());
        assert!(calls_to(&e, LOG).is_empty());
        // The file is there but the cell (1, 1) is outside its 1 x 1 grid.
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        stub(&mut e, FORM_FILE_COUNT, 1);
        assert_eq!(e.call(0x0058_5b30, &args![w, 1i32, 1i32]).u32(), 0);
        assert!(calls_to(&e, CELL_CONSTRUCT).is_empty());
    }

    fn unload_engine() -> (Engine, Ptr<TESWorldSpace>, u32) {
        let mut e = engine();
        let w = world(&mut e);
        let cell = e.mem.alloc(CELL_SIZE);
        e.mem.set_u32(cell + 0xC, 0xc311);
        const CELL_DESTRUCT: u32 = 0x0054_f200;
        give_vtable(&mut e, cell, &[(0x10, CELL_DESTRUCT)]);
        noop(&mut e, CELL_DESTRUCT);
        let owner = w.addr();
        e.register_double(CELL_GET_WORLD_SPACE, move |_, _| ret(owner));
        e.register(FORM_GET_FILE, |_, _| ret(0));
        for addr in [
            CELL_CLEANUP,
            SAVE_LOAD_NOTE_CELL,
            GAME_LOADER_UNLOAD_FORM,
            RELEASE_CELL,
        ] {
            noop(&mut e, addr);
        }
        stub(&mut e, LOADER_STATE_QUERY, 0);
        set_word(&mut e, SAVE_LOAD_GAME, SAVE_LOAD);
        set_word(&mut e, LOADER_STATE, 0x1111);
        set_word(&mut e, GAME_LOADER, 0x2222);
        (e, w, cell)
    }

    #[test]
    fn unload_cell_of_a_cell_with_no_file_runs_the_whole_teardown() {
        let (mut e, w, cell) = unload_engine();
        logged(&mut e);
        e.call(0x0058_5e00, &args![w, cell]);
        let order: Vec<u32> = addresses(&e).into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                CELL_GET_WORLD_SPACE,
                FORM_GET_FILE,
                CELL_CLEANUP,
                SAVE_LOAD_NOTE_CELL,
                WORD_AT_0C,
                LOADER_STATE_QUERY,
                GAME_LOADER_UNLOAD_FORM,
                RELEASE_CELL,
                0x0054_f200,
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_LOAD_NOTE_CELL),
            vec![vec![SAVE_LOAD, cell]]
        );
        assert_eq!(calls_to(&e, LOADER_STATE_QUERY), vec![vec![0x1111, 0xc311]]);
        assert_eq!(
            calls_to(&e, GAME_LOADER_UNLOAD_FORM),
            vec![vec![0x2222, cell, 0]]
        );
        assert_eq!(calls_to(&e, RELEASE_CELL), vec![vec![w.addr(), cell]]);
        // The cell is deleted through its destructor with the delete flag.
        assert_eq!(calls_to(&e, 0x0054_f200), vec![vec![cell, 1]]);
    }

    #[test]
    fn unload_cell_keeps_the_form_loaded_when_the_loader_still_has_it() {
        let (mut e, w, cell) = unload_engine();
        stub(&mut e, LOADER_STATE_QUERY, 1);
        logged(&mut e);
        e.call(0x0058_5e00, &args![w, cell]);
        assert!(calls_to(&e, GAME_LOADER_UNLOAD_FORM).is_empty());
        assert_eq!(calls_to(&e, RELEASE_CELL).len(), 1);
    }

    #[test]
    fn unload_cell_from_a_master_file_is_torn_down_but_from_a_plugin_only_cleaned() {
        let (mut e, w, cell) = unload_engine();
        e.register(FORM_GET_FILE, |_, _| ret(FILE));
        stub(&mut e, FILE_IS_MASTER, 1);
        logged(&mut e);
        e.call(0x0058_5e00, &args![w, cell]);
        assert_eq!(calls_to(&e, RELEASE_CELL).len(), 1);
        stub(&mut e, FILE_IS_MASTER, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_5e00, &args![w, cell]);
        assert_eq!(calls_to(&e, CELL_CLEANUP), vec![vec![cell]]);
        assert!(calls_to(&e, RELEASE_CELL).is_empty());
        assert!(calls_to(&e, 0x0054_f200).is_empty());
    }

    #[test]
    fn unload_cell_ignores_null_cells_and_cells_of_other_world_spaces() {
        let (mut e, w, cell) = unload_engine();
        let other = world(&mut e);
        logged(&mut e);
        e.call(0x0058_5e00, &args![w, 0u32]);
        e.call(0x0058_5e00, &args![other, cell]);
        assert!(calls_to(&e, CELL_CLEANUP).is_empty());
        assert!(calls_to(&e, RELEASE_CELL).is_empty());
    }

    // ---- land data ---------------------------------------------------------------

    #[test]
    fn find_land_data_searches_the_files_from_the_last_and_reports_the_first_hit() {
        let (mut e, w, data) = grid_engine();
        e.set(data, OffsetData::OffsetMinCoords_x, 0.0);
        e.set(data, OffsetData::OffsetMinCoords_y, 0.0);
        e.set(data, OffsetData::OffsetMaxCoords_x, 0.0);
        e.set(data, OffsetData::OffsetMaxCoords_y, 0.0);
        let table = e.get(data, OffsetData::pCellFileOffsets).addr();
        e.mem.set_u32(table, 0x500);
        let at = data.addr();
        // Three files that all share the data block.
        e.register_double(OFFSET_DATA_OF_FILE, move |_, _| ret(at));
        install_parents(&mut e, vec![]);
        stub(&mut e, FORM_FILE_COUNT, 3);
        e.register(FORM_GET_FILE, |_, a| ret(0xf000 + a[1]));
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        stub(&mut e, FILE_IS_MASTER, 1);
        stub(&mut e, FILE_SET_OFFSET, 1);
        // Land data in files 0 and 1 only.
        e.register(CELL_FIND_LAND_DATA_IN_FILE, |_, a| {
            ret((a[0] != 0xf002) as u32)
        });
        stub(&mut e, FILE_CURRENT_OFFSET, 0x777);
        let out_file = e.mem.alloc(4);
        let out_offset = e.mem.alloc(4);
        e.mem.set_u32(out_file, 0xdead);
        logged(&mut e);
        let found = e
            .call(0x0058_5ee0, &args![w, 0i32, 0i32, out_file, out_offset])
            .bool();
        assert!(found);
        // File 2 is tried first, has no land data; file 1 is the hit.
        assert_eq!(e.mem.u32(out_file), 0xf001);
        assert_eq!(e.mem.u32(out_offset), 0x777);
        assert_eq!(
            calls_to(&e, CELL_FIND_LAND_DATA_IN_FILE),
            vec![vec![0xf002], vec![0xf001]]
        );
    }

    #[test]
    fn find_land_data_that_nothing_has_leaves_the_outputs_zeroed() {
        let (mut e, w, _) = grid_engine();
        install_parents(&mut e, vec![]);
        stub(&mut e, FORM_FILE_COUNT, 1);
        e.register(FORM_GET_FILE, |_, _| ret(FILE));
        e.register(FILE_THREAD_SAFE, |_, a| ret(a[0]));
        stub(&mut e, FILE_IS_MASTER, 1);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xdead);
        e.mem.set_u32(out + 4, 0xbeef);
        // The cell (7, 7) is outside the grid.
        assert!(!e
            .call(0x0058_5ee0, &args![w, 7i32, 7i32, out, out + 4])
            .bool());
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(out + 4), 0);
    }

    #[test]
    fn find_land_data_of_a_world_that_uses_its_parents_cells_asks_the_parent() {
        let mut e = engine();
        let child = world(&mut e);
        let parent = world(&mut e);
        install_parents(&mut e, vec![(child.addr(), 0, parent.addr())]);
        // The parent has no files at all.
        e.register_double(FORM_FILE_COUNT, |_, a| ret((a[0] == 0) as u32));
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xdead);
        logged(&mut e);
        assert!(!e
            .call(0x0058_5ee0, &args![child, 1i32, 2i32, out, out + 4])
            .bool());
        assert_eq!(e.mem.u32(out), 0);
        // Only the parent's files were asked for.
        assert_eq!(calls_to(&e, FORM_FILE_COUNT), vec![vec![parent.addr()]]);
    }

    #[test]
    fn find_land_data_asks_for_the_parent_twice_before_recursing() {
        let mut e = engine();
        let child = world(&mut e);
        let parent = world(&mut e);
        install_parents(&mut e, vec![(child.addr(), 0, parent.addr())]);
        e.register_double(FORM_FILE_COUNT, |_, _| ret(0));
        let out = e.mem.alloc(8);
        logged(&mut e);
        assert!(!e
            .call(0x0058_5ee0, &args![child, 1i32, 2i32, out, out + 4])
            .bool());
        // 00585f04 and 00585fbe on the child, then 00585f04 on the parent.
        assert_eq!(
            calls_to(&e, PARENT_FOR),
            vec![
                vec![child.addr(), 0],
                vec![child.addr(), 0],
                vec![parent.addr(), 0]
            ]
        );
    }

    // ---- InitItem ------------------------------------------------------------------

    /// The doubles for walking a cell map: `GetFirstPos` gives 1 when there are
    /// cells and `GetNext` serves them in order (positions are 1-based
    /// indexes, keys are the indexes).
    fn install_cell_map(e: &mut Engine, cells: Vec<u32>) {
        let count = cells.len() as u32;
        e.register_double(MAP_FIRST_POS, move |_, _| ret((count > 0) as u32));
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let position = e.mem.u32(a[1]);
            let index = position - 1;
            let next = if position < count { position + 1 } else { 0 };
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[2], index);
            e.mem.set_u32(a[3], cells[index as usize]);
            Ret::default()
        });
    }

    /// An engine for `InitItem`: ids become load order ids (+0x01000000),
    /// forms are found from a table, and nothing is logged until asked.
    fn init_engine(known: Vec<(u32, u32)>) -> (Engine, Ptr<TESWorldSpace>) {
        let mut e = engine();
        let w = world(&mut e);
        stub(&mut e, FORM_FLAG_8, 0);
        stub(&mut e, FORM_GET_FILE, 0xf11e);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register_double(FORM_BY_ID, move |_, a| {
            ret(known
                .iter()
                .find(|(id, _)| *id == a[0])
                .map_or(0, |(_, form)| *form))
        });
        for addr in [
            SET_CLIMATE,
            SET_IMAGE_SPACE,
            SET_WATER,
            SET_LOD_WATER,
            SET_MUSIC_TYPE,
            SET_PARENT_WORLD,
            IMPACT_SWAP_INIT_ITEM,
            FORM_SET_STATE,
            LOG,
        ] {
            noop(&mut e, addr);
        }
        stub(&mut e, EDITOR_ID_LENGTH, 0);
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        e.register(TERRAIN_MANAGER_CONSTRUCT, |_, a| ret(a[0]));
        install_cell_map(&mut e, vec![]);
        let map: Ptr<crate::types::NiTPointerMap> = e.new_object();
        e.set(w, TESWorldSpace::pCellMap, map.cast());
        e.mem.set_u32(w.addr() + 0xC, 0x3c);
        give_vtable(&mut e, w.addr(), &[(SLOT_EDITOR_ID, 0x0058_5000)]);
        stub(&mut e, 0x0058_5000, 0x7777);
        (e, w)
    }

    #[test]
    fn init_item_resolves_each_form_reference_and_hands_it_to_its_setter() {
        let forms: Vec<(u32, u32)> = (1..=7)
            .map(|i| (0x0100_0000 | i, 0xf0f0_0000 + i))
            .collect();
        let (mut e, w) = init_engine(forms);
        // Climate, image space, water, lod water, music, parent, zone ids.
        e.set(w, TESWorldSpace::pClimate, Ptr::new(1));
        e.set(w, TESWorldSpace::pImageSpace, Ptr::new(2));
        e.set(w, TESWorldSpace::pEncounterZone, Ptr::new(3));
        e.set(w, TESWorldSpace::pWorldWater, Ptr::new(4));
        e.set(w, TESWorldSpace::pLODWater, Ptr::new(5));
        e.set(w, TESWorldSpace::pMusicType, Ptr::new(6));
        e.set(w, TESWorldSpace::pParentWorld, Ptr::new(7));
        e.set(w, TESWorldSpace::pImpactSwap, Ptr::new(0x5151));
        logged(&mut e);
        e.call(0x0058_45e0, &args![w]);
        let this = w.addr();
        assert_eq!(calls_to(&e, SET_CLIMATE), vec![vec![this, 0xf0f0_0001]]);
        assert_eq!(calls_to(&e, SET_IMAGE_SPACE), vec![vec![this, 0xf0f0_0002]]);
        assert_eq!(e.get(w, TESWorldSpace::pEncounterZone).addr(), 0xf0f0_0003);
        assert_eq!(calls_to(&e, SET_WATER), vec![vec![this, 0xf0f0_0004]]);
        assert_eq!(calls_to(&e, SET_LOD_WATER), vec![vec![this, 0xf0f0_0005]]);
        assert_eq!(calls_to(&e, SET_MUSIC_TYPE), vec![vec![this, 0xf0f0_0006]]);
        assert_eq!(
            calls_to(&e, SET_PARENT_WORLD),
            vec![vec![this, 0xf0f0_0007]]
        );
        assert_eq!(
            calls_to(&e, IMPACT_SWAP_INIT_ITEM),
            vec![vec![0x5151, this]]
        );
        assert_eq!(calls_to(&e, FORM_SET_STATE), vec![vec![this, 1]]);
        assert!(calls_to(&e, LOG).is_empty());
        // The terrain manager is created at the end.
        let manager = e.get(w, TESWorldSpace::pTerrainManager);
        assert!(!manager.is_null());
        assert_eq!(
            calls_to(&e, TERRAIN_MANAGER_CONSTRUCT),
            vec![vec![manager.addr(), this]]
        );
        // The form types: climate, image space, encounter zone, water, lod
        // water, music, world space (the cast targets, in this order).
        let targets: Vec<u32> = calls_to(&e, RT_DYNAMIC_CAST).iter().map(|c| c[3]).collect();
        assert_eq!(
            targets,
            vec![
                TYPE_TES_CLIMATE,
                TYPE_TES_IMAGE_SPACE,
                TYPE_BGS_ENCOUNTER_ZONE,
                TYPE_TES_WATER_FORM,
                TYPE_TES_WATER_FORM,
                TYPE_BGS_MUSIC_TYPE,
                TYPE_TES_WORLD_SPACE,
            ]
        );
        // `AddCompileIndex` got the form's last file.
        assert_eq!(calls_to(&e, FORM_ADD_COMPILE_INDEX).len(), 7);
        assert_eq!(calls_to(&e, FORM_ADD_COMPILE_INDEX)[0][1], 0xf11e);
    }

    #[test]
    fn init_item_logs_each_reference_it_cannot_find() {
        // (field offset, message with the editor ID, message with the form ID)
        let fields: [(u32, u32, u32); 7] = [
            (0x40, MSG_CLIMATE_NAMED, MSG_CLIMATE_ID),
            (0x44, MSG_IMAGE_SPACE_NAMED, MSG_IMAGE_SPACE_ID),
            (0xD0, MSG_ENCOUNTER_ZONE_NAMED, MSG_ENCOUNTER_ZONE_ID),
            (0x74, MSG_WATER_NAMED, MSG_WATER_ID),
            (0x78, MSG_LOD_WATER_NAMED, MSG_LOD_WATER_ID),
            (0x9C, MSG_MUSIC_NAMED, MSG_MUSIC_ID),
            (0x70, MSG_PARENT_NAMED, MSG_PARENT_ID),
        ];
        for (offset, named, plain) in fields {
            for has_editor_id in [true, false] {
                let (mut e, w) = init_engine(vec![]);
                e.mem.set_u32(w.addr() + offset, 0x42);
                stub(&mut e, EDITOR_ID_LENGTH, has_editor_id as u32);
                logged(&mut e);
                e.call(0x0058_45e0, &args![w]);
                let expected = if has_editor_id {
                    // The name comes from the form's slot +0x130.
                    vec![named, 0x0100_0042, 0x7777]
                } else {
                    vec![plain, 0x0100_0042, 0x3c]
                };
                assert_eq!(
                    calls_to(&e, LOG),
                    vec![expected],
                    "field {offset:#x} named {has_editor_id}"
                );
            }
        }
    }

    #[test]
    fn init_item_with_form_flag_8_set_only_initializes_the_cells() {
        let (mut e, w) = init_engine(vec![]);
        stub(&mut e, FORM_FLAG_8, 1);
        e.set(w, TESWorldSpace::pClimate, Ptr::new(1));
        e.set(w, TESWorldSpace::pImpactSwap, Ptr::new(0x5151));
        // Two cells in the map and a persistent cell.
        const INIT: u32 = 0x0054_f000;
        let cells: Vec<u32> = (0..3)
            .map(|_| {
                let cell = e.mem.alloc(0x20);
                give_vtable(&mut e, cell, &[(0x88, INIT)]);
                cell
            })
            .collect();
        noop(&mut e, INIT);
        install_cell_map(&mut e, vec![cells[0], 0, cells[1]]);
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(cells[2]));
        logged(&mut e);
        e.call(0x0058_45e0, &args![w]);
        assert!(calls_to(&e, SET_CLIMATE).is_empty());
        assert!(calls_to(&e, FORM_SET_STATE).is_empty());
        assert!(calls_to(&e, IMPACT_SWAP_INIT_ITEM).is_empty());
        // The null entry of the map is skipped; the persistent cell is last.
        assert_eq!(
            calls_to(&e, INIT),
            vec![vec![cells[0]], vec![cells[1]], vec![cells[2]]]
        );
        assert!(!e.get(w, TESWorldSpace::pTerrainManager).is_null());
    }

    // ---- ordering and duplicates ---------------------------------------------------

    const WORLD_SAVES_BEFORE: u32 = 0x0058_5100;
    const CHILD_OWNER: u32 = 0x0058_5200;

    #[test]
    fn saves_before_a_form_orders_cells_references_and_worlds() {
        let mut e = engine();
        let w = world(&mut e);
        give_vtable(&mut e, w.addr(), &[(0x3C, WORLD_SAVES_BEFORE)]);
        // The world space's own answer for another world space.
        e.register_double(WORLD_SAVES_BEFORE, |_, a| ret((a[1] == 0x7001) as u32));
        e.mem.set_u32(w.addr() + 0xC, 0x20);
        let form = e.mem.alloc(0x20);
        // A cell: ordered by the world space it is in.
        stub(&mut e, FORM_TYPE, 0x39);
        stub(&mut e, CELL_GET_WORLD_SPACE, w.addr());
        logged(&mut e);
        assert!(e.call(0x0058_4be0, &args![w, form]).bool());
        // Its own world space: true without asking the world space.
        assert!(calls_to(&e, WORLD_SAVES_BEFORE).is_empty());
        assert_eq!(calls_to(&e, RT_DYNAMIC_CAST)[0][3], TYPE_TES_OBJECT_CELL);
        stub(&mut e, CELL_GET_WORLD_SPACE, 0x7001);
        assert!(e.call(0x0058_4be0, &args![w, form]).bool());
        stub(&mut e, CELL_GET_WORLD_SPACE, 0x7002);
        assert!(!e.call(0x0058_4be0, &args![w, form]).bool());
        // A cell with no world space (an interior cell).
        stub(&mut e, CELL_GET_WORLD_SPACE, 0);
        assert!(!e.call(0x0058_4be0, &args![w, form]).bool());
        // A cell that does not cast.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(!e.call(0x0058_4be0, &args![w, form]).bool());
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));

        // A placed reference: ordered by what its child-cell part gives.
        let child = e.mem.alloc(0x20);
        give_vtable(&mut e, child, &[(0, CHILD_OWNER)]);
        e.register_double(CHILD_OWNER, |_, _| ret(0x7001));
        e.register_double(RT_DYNAMIC_CAST, move |_, _| ret(child));
        for form_type in [0x3a, 0x40, 0x42, 0x43, 0x69] {
            stub(&mut e, FORM_TYPE, form_type);
            assert!(
                e.call(0x0058_4be0, &args![w, form]).bool(),
                "{form_type:#x}"
            );
        }
        // Types next to the range take the default.
        stub(&mut e, FORM_SAVES_BEFORE_FORM, 0);
        for form_type in [0x38, 0x44, 0x68, 0x6a] {
            stub(&mut e, FORM_TYPE, form_type);
            assert!(
                !e.call(0x0058_4be0, &args![w, form]).bool(),
                "{form_type:#x}"
            );
        }
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));

        // Another world space: by form ID.
        stub(&mut e, FORM_TYPE, 0x41);
        e.mem.set_u32(form + 0xC, 0x21);
        assert!(e.call(0x0058_4be0, &args![w, form]).bool());
        e.mem.set_u32(form + 0xC, 0x20);
        assert!(!e.call(0x0058_4be0, &args![w, form]).bool());
        e.mem.set_u32(form + 0xC, 0x1f);
        assert!(!e.call(0x0058_4be0, &args![w, form]).bool());

        // Anything else: the base class's order.
        stub(&mut e, FORM_TYPE, 0x2a);
        stub(&mut e, FORM_SAVES_BEFORE_FORM, 1);
        assert!(e.call(0x0058_4be0, &args![w, form]).bool());
    }

    fn group_header(e: &mut Engine, group_type: u32, label: u32) -> u32 {
        let header = e.mem.alloc(0x20);
        e.mem.set_u32(header, e.global::<u32>(GROUP_TAG));
        e.mem.set_u32(header + 8, label);
        e.mem.set_u32(header + 0xC, group_type);
        header
    }

    #[test]
    fn saves_before_a_group_header_depends_on_the_group_type() {
        let mut e = engine();
        let w = world(&mut e);
        give_vtable(&mut e, w.addr(), &[(0x3C, WORLD_SAVES_BEFORE)]);
        e.register_double(WORLD_SAVES_BEFORE, |_, a| ret((a[1] == 0x7001) as u32));
        // The forms by label: a world space and a cell.
        e.register_double(FORM_BY_ID, |_, a| ret(a[0] + 0x6000));
        stub(&mut e, FORM_SAVES_BEFORE_HEADER, 1);
        let ask = |e: &mut Engine, kind: u32, label: u32| {
            let header = group_header(e, kind, label);
            e.call(0x0058_4d40, &args![w, header]).bool()
        };
        // Type 0: the base class decides.
        logged(&mut e);
        assert!(ask(&mut e, 0, 0));
        assert_eq!(calls_to(&e, FORM_SAVES_BEFORE_HEADER).len(), 1);
        // Types 1 and 6: the labelled form is cast to a world space / a cell
        // and ordered through slot +0x3c.
        assert!(ask(&mut e, 1, 0x1001));
        assert!(!ask(&mut e, 1, 0x1002));
        assert!(ask(&mut e, 6, 0x1001));
        let casts: Vec<u32> = calls_to(&e, RT_DYNAMIC_CAST).iter().map(|c| c[3]).collect();
        assert_eq!(
            casts,
            vec![
                TYPE_TES_WORLD_SPACE,
                TYPE_TES_WORLD_SPACE,
                TYPE_TES_OBJECT_CELL
            ]
        );
        // The form is not there.
        e.register_double(FORM_BY_ID, |_, _| ret(0));
        assert!(!ask(&mut e, 1, 0x1001));
        // Types 7: yes; 2, 3, 4, 5, 8, 9 and beyond: no.
        assert!(ask(&mut e, 7, 0));
        for kind in [2, 3, 4, 5, 8, 9, 10] {
            assert!(!ask(&mut e, kind, 0), "{kind}");
        }
        // Not a group header; no header.
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record, 0x1234);
        assert!(!e.call(0x0058_4d40, &args![w, record]).bool());
        assert!(!e.call(0x0058_4d40, &args![w, 0u32]).bool());
    }

    const CELL_DUPLICATE: u32 = 0x0054_f300;
    const CELL_SET_ALTERED: u32 = 0x0054_f400;

    #[test]
    fn create_duplicate_form_copies_the_cells_into_the_new_world_space() {
        let mut e = engine();
        let w = world(&mut e);
        let copy = world(&mut e);
        e.register_double(FORM_CREATE_DUPLICATE, move |_, _| ret(copy.addr()));
        let copy_map = e.mem.alloc(0x10);
        e.mem.set_u32(copy.addr() + 0x30, copy_map);
        noop(&mut e, MAP_REMOVE_ALL);
        noop(&mut e, ADD_CELL);
        noop(&mut e, FINISH_DUPLICATE);
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        // Each source cell duplicates into a new cell object.
        e.register_double(CELL_DUPLICATE, |e, _| {
            let duplicate = e.mem.alloc(0x20);
            give_vtable(e, duplicate, &[(0xC8, CELL_SET_ALTERED)]);
            ret(duplicate)
        });
        noop(&mut e, CELL_SET_ALTERED);
        let sources: Vec<u32> = (0..3)
            .map(|_| {
                let cell = e.mem.alloc(0x20);
                give_vtable(&mut e, cell, &[(0x40, CELL_DUPLICATE)]);
                cell
            })
            .collect();
        install_cell_map(&mut e, vec![sources[0], 0, sources[1]]);
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(sources[2]));
        logged(&mut e);
        let other = 0xcafeu32;
        let result = e.call(0x0058_4e60, &args![w, 1u32, other]).ptr::<()>();
        assert_eq!(result, copy.cast());
        assert_eq!(
            calls_to(&e, FORM_CREATE_DUPLICATE),
            vec![vec![w.addr(), 1, other]]
        );
        // The copy's cell map is emptied first.
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![copy_map]]);
        // Each non-null cell is duplicated with (0, other), marked altered
        // (slot +0xc8, argument 1) and added to the copy.
        let duplicates = calls_to(&e, CELL_DUPLICATE);
        assert_eq!(
            duplicates,
            vec![
                vec![sources[0], 0, other],
                vec![sources[1], 0, other],
                vec![sources[2], 0, other]
            ]
        );
        let added = calls_to(&e, ADD_CELL);
        assert_eq!(added.len(), 2);
        assert!(added.iter().all(|a| a[0] == copy.addr()));
        let altered = calls_to(&e, CELL_SET_ALTERED);
        assert_eq!(altered.len(), 3);
        assert!(altered.iter().all(|a| a[1] == 1));
        // The persistent cell's duplicate becomes the copy's.
        let persistent = e.get(copy, TESWorldSpace::pPersistentCell);
        assert_eq!(persistent.addr(), altered[2][0]);
        assert_eq!(calls_to(&e, FINISH_DUPLICATE), vec![vec![copy.addr()]]);
    }

    #[test]
    fn create_duplicate_form_skips_duplicates_that_are_not_cells_and_a_missing_persistent_cell() {
        let mut e = engine();
        let w = world(&mut e);
        let copy = world(&mut e);
        e.register_double(FORM_CREATE_DUPLICATE, move |_, _| ret(copy.addr()));
        let copy_map = e.mem.alloc(0x10);
        e.mem.set_u32(copy.addr() + 0x30, copy_map);
        noop(&mut e, MAP_REMOVE_ALL);
        noop(&mut e, ADD_CELL);
        noop(&mut e, FINISH_DUPLICATE);
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        // The duplicate of the cell is not a cell.
        e.register_double(CELL_DUPLICATE, |_, _| ret(0x9999));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[3] == TYPE_TES_OBJECT_CELL {
                0
            } else {
                a[0]
            })
        });
        let cell = e.mem.alloc(0x20);
        give_vtable(&mut e, cell, &[(0x40, CELL_DUPLICATE)]);
        install_cell_map(&mut e, vec![cell]);
        logged(&mut e);
        e.call(0x0058_4e60, &args![w, 0u32, 0u32]);
        assert!(calls_to(&e, ADD_CELL).is_empty());
        assert!(e.get(copy, TESWorldSpace::pPersistentCell).is_null());
        assert_eq!(calls_to(&e, FINISH_DUPLICATE).len(), 1);
    }

    // ---- Copy and Compare ----------------------------------------------------------

    const TEXTURE_COPY: u32 = 0x0054_f500;
    const TEXTURE_COMPARE: u32 = 0x0054_f600;

    /// Doubles shared by `Copy` and `Compare`: the getters read the fields.
    fn copy_engine() -> Engine {
        let mut e = engine();
        install_parents(&mut e, vec![]);
        set_word(&mut e, DEFAULT_WATER, 0x00dd_0001);
        e.register(GET_IMAGE_SPACE_RAW, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(GET_LOD_WATER_HEIGHT, |e, a| {
            ret_float(e.mem.f32(a[0] + 0x7C))
        });
        e
    }

    /// A world space with distinctive values; its textures have vtables.
    fn filled_world(e: &mut Engine, salt: u32) -> Ptr<TESWorldSpace> {
        let w = world(e);
        e.set(w, TESWorldSpace::cFlags, 0x11);
        e.set(w, TESWorldSpace::sParentUseFlags, 0x22);
        e.set(w, TESWorldSpace::pParentWorld, Ptr::new(0x70 + salt));
        e.set(w, TESWorldSpace::pEncounterZone, Ptr::new(0xd0 + salt));
        e.set(w, TESWorldSpace::pClimate, Ptr::new(0x40 + salt));
        e.set(w, TESWorldSpace::pImageSpace, Ptr::new(0x44 + salt));
        e.set(w, TESWorldSpace::pWorldWater, Ptr::new(0x74 + salt));
        e.set(w, TESWorldSpace::pLODWater, Ptr::new(0x78 + salt));
        e.set(w, TESWorldSpace::fLODWaterHeight, 5.0 + salt as f32);
        e.set(w, TESWorldSpace::fDefaultLandHeight, -1.0 + salt as f32);
        e.set(w, TESWorldSpace::fDefaultWaterHeight, 2.0 + salt as f32);
        e.set(w, TESWorldSpace::pMusicType, Ptr::new(0x9c + salt));
        e.mem.write(w.addr() + 0x80, &[salt as u8 + 1; 0x10]);
        e.mem.write(w.addr() + 0x90, &[salt as u8 + 2; 0xC]);
        give_vtable(
            e,
            w.addr() + 0xD4,
            &[(8, TEXTURE_COPY), (0xC, TEXTURE_COMPARE)],
        );
        give_vtable(
            e,
            w.addr() + 0xE0,
            &[(8, TEXTURE_COPY), (0xC, TEXTURE_COMPARE)],
        );
        w
    }

    #[test]
    fn copy_takes_every_value_of_the_source_world_space() {
        let mut e = copy_engine();
        let this = filled_world(&mut e, 0);
        let source = filled_world(&mut e, 0x10);
        for addr in [
            FORM_COPY_COMPONENTS,
            SET_WATER,
            SET_LOD_WATER,
            SET_MUSIC_TYPE,
            TEXTURE_COPY,
            MAP_REMOVE_ALL,
            MAP_SET_AT,
        ] {
            noop(&mut e, addr);
        }
        // The source map has two entries (key = index).
        install_cell_map(&mut e, vec![0xc1, 0xc2]);
        let source_map = e.mem.alloc(0x10);
        let own_map = e.mem.alloc(0x10);
        e.set(source, TESWorldSpace::pCellMap, Ptr::new(source_map));
        e.set(this, TESWorldSpace::pCellMap, Ptr::new(own_map));
        logged(&mut e);
        e.call(0x0058_4fa0, &args![this, source]);
        assert_eq!(
            calls_to(&e, FORM_COPY_COMPONENTS),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(e.get(this, TESWorldSpace::cFlags), 0x11);
        assert_eq!(e.get(this, TESWorldSpace::sParentUseFlags), 0x22);
        assert_eq!(e.get(this, TESWorldSpace::pParentWorld).addr(), 0x80);
        assert_eq!(e.get(this, TESWorldSpace::pEncounterZone).addr(), 0xe0);
        assert_eq!(e.get(this, TESWorldSpace::pClimate).addr(), 0x50);
        assert_eq!(e.get(this, TESWorldSpace::pImageSpace).addr(), 0x54);
        assert_eq!(calls_to(&e, SET_WATER), vec![vec![this.addr(), 0x84]]);
        assert_eq!(calls_to(&e, SET_LOD_WATER), vec![vec![this.addr(), 0x88]]);
        assert_eq!(calls_to(&e, SET_MUSIC_TYPE), vec![vec![this.addr(), 0xac]]);
        assert_eq!(e.get(this, TESWorldSpace::fLODWaterHeight), 21.0);
        assert_eq!(e.get(this, TESWorldSpace::fDefaultLandHeight), 15.0);
        assert_eq!(e.get(this, TESWorldSpace::fDefaultWaterHeight), 18.0);
        assert_eq!(e.mem.bytes(this.addr() + 0x80, 0x10), vec![0x11; 0x10]);
        assert_eq!(e.mem.bytes(this.addr() + 0x90, 0xC), vec![0x12; 0xC]);
        // The cell map is emptied and refilled with the source's entries.
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![own_map]]);
        assert_eq!(
            calls_to(&e, MAP_SET_AT),
            vec![vec![own_map, 0, 0xc1], vec![own_map, 1, 0xc2]]
        );
        // Both textures copy the source's texture.
        assert_eq!(
            calls_to(&e, TEXTURE_COPY),
            vec![
                vec![this.addr() + 0xD4, source.addr() + 0xD4],
                vec![this.addr() + 0xE0, source.addr() + 0xE0]
            ]
        );
    }

    #[test]
    fn copy_makes_copies_or_drops_the_impact_swap_to_match_the_source() {
        const SWAP: u32 = 0x5151;
        const OTHER_SWAP: u32 = 0x5252;
        for (source_has, own_has) in [(true, false), (true, true), (false, true), (false, false)] {
            let mut e = copy_engine();
            let this = filled_world(&mut e, 0);
            let source = filled_world(&mut e, 0x10);
            for addr in [
                FORM_COPY_COMPONENTS,
                SET_WATER,
                SET_LOD_WATER,
                SET_MUSIC_TYPE,
                TEXTURE_COPY,
                MAP_REMOVE_ALL,
                IMPACT_SWAP_COPY,
                IMPACT_SWAP_DESTRUCT,
            ] {
                noop(&mut e, addr);
            }
            e.register(IMPACT_SWAP_CONSTRUCT, |_, a| ret(a[0]));
            install_cell_map(&mut e, vec![]);
            if source_has {
                e.set(source, TESWorldSpace::pImpactSwap, Ptr::new(SWAP));
            }
            if own_has {
                e.set(this, TESWorldSpace::pImpactSwap, Ptr::new(OTHER_SWAP));
            }
            logged(&mut e);
            e.call(0x0058_4fa0, &args![this, source]);
            let own = e.get(this, TESWorldSpace::pImpactSwap).addr();
            match (source_has, own_has) {
                (true, false) => {
                    // A new swap (0x15C bytes) is constructed, then copied to.
                    assert_ne!(own, 0);
                    assert_eq!(calls_to(&e, IMPACT_SWAP_CONSTRUCT), vec![vec![own]]);
                    assert_eq!(calls_to(&e, IMPACT_SWAP_COPY), vec![vec![own, SWAP]]);
                }
                (true, true) => {
                    assert_eq!(own, OTHER_SWAP);
                    assert!(calls_to(&e, IMPACT_SWAP_CONSTRUCT).is_empty());
                    assert_eq!(calls_to(&e, IMPACT_SWAP_COPY), vec![vec![OTHER_SWAP, SWAP]]);
                }
                (false, true) => {
                    // Deleted through the destructor with the delete flag.
                    assert_eq!(own, 0);
                    assert_eq!(
                        calls_to(&e, IMPACT_SWAP_DESTRUCT),
                        vec![vec![OTHER_SWAP, 1]]
                    );
                    assert!(calls_to(&e, IMPACT_SWAP_COPY).is_empty());
                }
                (false, false) => {
                    assert_eq!(own, 0);
                    assert!(calls_to(&e, IMPACT_SWAP_COPY).is_empty());
                    assert!(calls_to(&e, IMPACT_SWAP_DESTRUCT).is_empty());
                }
            }
        }
    }

    #[test]
    fn copy_of_something_that_is_not_a_world_space_does_nothing() {
        let mut e = copy_engine();
        let this = filled_world(&mut e, 0);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        logged(&mut e);
        e.call(0x0058_4fa0, &args![this, 0x1234u32]);
        assert_eq!(e.get(this, TESWorldSpace::pClimate).addr(), 0x40);
        assert_eq!(addresses(&e), vec![0x0058_4fa0, RT_DYNAMIC_CAST]);
    }

    fn compare_engine() -> Engine {
        let mut e = copy_engine();
        stub(&mut e, FORM_COMPARE_COMPONENTS, 0);
        stub(&mut e, IMPACT_SWAP_COMPARE, 0);
        stub(&mut e, TEXTURE_COMPARE, 0);
        e
    }

    #[test]
    fn compare_of_identical_world_spaces_says_they_are_the_same() {
        let mut e = compare_engine();
        let a = filled_world(&mut e, 0);
        let b = filled_world(&mut e, 0);
        assert!(!e.call(0x0058_5250, &args![a, b]).bool());
        // Both with an impact swap that compares equal.
        e.set(a, TESWorldSpace::pImpactSwap, Ptr::new(0x5151));
        e.set(b, TESWorldSpace::pImpactSwap, Ptr::new(0x5252));
        logged(&mut e);
        assert!(!e.call(0x0058_5250, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, IMPACT_SWAP_COMPARE),
            vec![vec![0x5151, 0x5252]]
        );
    }

    #[test]
    fn compare_notices_each_difference() {
        type Change = (&'static str, fn(&mut Engine, Ptr<TESWorldSpace>));
        let changes: Vec<Change> = vec![
            ("flags", |e, w| e.set(w, TESWorldSpace::cFlags, 0x12)),
            ("parent flags", |e, w| {
                e.set(w, TESWorldSpace::sParentUseFlags, 0x23)
            }),
            ("parent", |e, w| {
                e.set(w, TESWorldSpace::pParentWorld, Ptr::new(1))
            }),
            ("zone", |e, w| {
                e.set(w, TESWorldSpace::pEncounterZone, Ptr::new(1))
            }),
            ("climate", |e, w| {
                e.set(w, TESWorldSpace::pClimate, Ptr::new(1))
            }),
            ("image space", |e, w| {
                e.set(w, TESWorldSpace::pImageSpace, Ptr::new(1))
            }),
            ("impact swap", |e, w| {
                e.set(w, TESWorldSpace::pImpactSwap, Ptr::new(1))
            }),
            ("water", |e, w| {
                e.set(w, TESWorldSpace::pWorldWater, Ptr::new(1))
            }),
            ("lod water", |e, w| {
                e.set(w, TESWorldSpace::pLODWater, Ptr::new(1))
            }),
            ("lod height", |e, w| {
                e.set(w, TESWorldSpace::fLODWaterHeight, 1.0)
            }),
            ("land height", |e, w| {
                e.set(w, TESWorldSpace::fDefaultLandHeight, 1.0)
            }),
            ("water height", |e, w| {
                e.set(w, TESWorldSpace::fDefaultWaterHeight, 1.0)
            }),
            ("map data", |e, w| e.mem.set_u8(w.addr() + 0x8F, 0x77)),
            ("map offset data", |e, w| {
                e.mem.set_u8(w.addr() + 0x9B, 0x77)
            }),
            ("music", |e, w| {
                e.set(w, TESWorldSpace::pMusicType, Ptr::new(1))
            }),
        ];
        for (name, change) in changes {
            let mut e = compare_engine();
            let a = filled_world(&mut e, 0);
            let b = filled_world(&mut e, 0);
            change(&mut e, b);
            assert!(e.call(0x0058_5250, &args![a, b]).bool(), "{name}");
        }
    }

    #[test]
    fn compare_asks_the_components_the_impact_swaps_and_the_textures() {
        // Components differ.
        let mut e = compare_engine();
        let (a, b) = (filled_world(&mut e, 0), filled_world(&mut e, 0));
        stub(&mut e, FORM_COMPARE_COMPONENTS, 1);
        assert!(e.call(0x0058_5250, &args![a, b]).bool());
        // Impact swaps that differ.
        let mut e = compare_engine();
        let (a, b) = (filled_world(&mut e, 0), filled_world(&mut e, 0));
        e.set(a, TESWorldSpace::pImpactSwap, Ptr::new(1));
        e.set(b, TESWorldSpace::pImpactSwap, Ptr::new(2));
        stub(&mut e, IMPACT_SWAP_COMPARE, 1);
        assert!(e.call(0x0058_5250, &args![a, b]).bool());
        // Only one of them has one.
        let mut e = compare_engine();
        let (a, b) = (filled_world(&mut e, 0), filled_world(&mut e, 0));
        e.set(a, TESWorldSpace::pImpactSwap, Ptr::new(1));
        assert!(e.call(0x0058_5250, &args![a, b]).bool());
        assert!(e.call(0x0058_5250, &args![b, a]).bool());
        // Textures: the first one, then the second.
        for (which, expected_calls) in [(0xD4u32, 1), (0xE0, 2)] {
            let mut e = compare_engine();
            let (a, b) = (filled_world(&mut e, 0), filled_world(&mut e, 0));
            let hit = a.addr() + which;
            e.register_double(TEXTURE_COMPARE, move |_, args| ret((args[0] == hit) as u32));
            logged(&mut e);
            assert!(e.call(0x0058_5250, &args![a, b]).bool(), "{which:#x}");
            assert_eq!(calls_to(&e, TEXTURE_COMPARE).len(), expected_calls);
        }
        // Not a world space: different.
        let mut e = compare_engine();
        let a = filled_world(&mut e, 0);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0058_5250, &args![a, 0x1234u32]).bool());
    }

    #[test]
    fn compare_does_not_equate_nan_heights() {
        let mut e = compare_engine();
        let (a, b) = (filled_world(&mut e, 0), filled_world(&mut e, 0));
        e.set(a, TESWorldSpace::fDefaultLandHeight, f32::NAN);
        e.set(b, TESWorldSpace::fDefaultLandHeight, f32::NAN);
        assert!(e.call(0x0058_5250, &args![a, b]).bool());
    }

    // ---- second session: 00586390 to 00588b00 ---------------------------------

    /// What the `NiTPointerMap` stand-in holds: (map, key, value).
    type MapTable = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// Calls recorded as (cell, list) pairs.
    type FillLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// Cells a visit double was given.
    type VisitLog = Rc<RefCell<Vec<u32>>>;

    /// Doubles for the map functions the unit calls (`GetAt`, `SetAt`,
    /// `RemoveAt`, `RemoveAll`, `GetFirstPos`, `GetNext`); the positions are
    /// 1-based indices into the entries of one map.
    fn install_maps(e: &mut Engine) -> MapTable {
        let table: MapTable = Rc::default();
        let t = table.clone();
        e.register_double(MAP_GET, move |e, a| {
            let found = t
                .borrow()
                .iter()
                .find(|m| m.0 == a[0] && m.1 == a[1])
                .map(|m| m.2);
            if let Some(value) = found {
                e.mem.set_u32(a[2], value);
            }
            ret(found.is_some() as u32)
        });
        let t = table.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            let mut t = t.borrow_mut();
            if let Some(m) = t.iter_mut().find(|m| m.0 == a[0] && m.1 == a[1]) {
                m.2 = a[2];
            } else {
                t.push((a[0], a[1], a[2]));
            }
            Ret::default()
        });
        let t = table.clone();
        e.register_double(MAP_REMOVE_AT, move |_, a| {
            t.borrow_mut().retain(|m| !(m.0 == a[0] && m.1 == a[1]));
            Ret::default()
        });
        let t = table.clone();
        e.register_double(MAP_REMOVE_ALL, move |_, a| {
            t.borrow_mut().retain(|m| m.0 != a[0]);
            Ret::default()
        });
        let t = table.clone();
        e.register_double(MAP_FIRST_POS, move |_, a| {
            ret(t.borrow().iter().any(|m| m.0 == a[0]) as u32)
        });
        let t = table.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let t = t.borrow();
            let entries: Vec<_> = t.iter().filter(|m| m.0 == a[0]).collect();
            let index = e.mem.u32(a[1]) as usize - 1;
            e.mem.set_u32(a[2], entries[index].1);
            e.mem.set_u32(a[3], entries[index].2);
            let next = if index + 1 < entries.len() {
                index as u32 + 2
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        table
    }

    /// What the `BSSimpleList` stand-ins were asked to do: (what, list,
    /// item or flag).
    type ListLog = Rc<RefCell<Vec<(&'static str, u32, u32)>>>;

    /// Doubles for the list functions: node access by the real layout (item
    /// at +0, next at +4); the changing ones only record what they were
    /// asked.
    fn install_lists(e: &mut Engine) -> ListLog {
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_CONSTRUCT, |_, a| ret(a[0]));
        let log: ListLog = Rc::default();
        let l = log.clone();
        // `AddHead`: the new item becomes the head's item, the old head
        // content moves into a new second node.
        e.register_double(LIST_ADD, move |e, a| {
            let item = e.mem.u32(a[1]);
            l.borrow_mut().push(("add", a[0], item));
            if item != 0 {
                if e.mem.u32(a[0]) != 0 {
                    let node = e.mem.alloc(8);
                    e.mem.set_u32(node, e.mem.u32(a[0]));
                    e.mem.set_u32(node + 4, e.mem.u32(a[0] + 4));
                    e.mem.set_u32(a[0] + 4, node);
                }
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        let l = log.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            l.borrow_mut().push(("remove", a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let l = log.clone();
        e.register_double(LIST_CLEAR, move |_, a| {
            l.borrow_mut().push(("clear", a[0], 0));
            Ret::default()
        });
        let l = log.clone();
        e.register_double(LIST_DESTROY, move |_, a| {
            l.borrow_mut().push(("destroy", a[0], a[1]));
            ret(a[0])
        });
        log
    }

    /// Copies the first node of the list at `head` into the node at `at`, so
    /// that a list whose head is embedded in an object starts there.
    fn embed_list(e: &mut Engine, at: u32, head: u32) {
        e.mem.set_u32(at, e.mem.u32(head));
        e.mem.set_u32(at + 4, e.mem.u32(head + 4));
    }

    /// A list of `items` in game memory (nodes of item then next); returns
    /// the head. No items: a head with item 0 and next 0.
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        let nodes: Vec<u32> = (0..items.len().max(1)).map(|_| e.mem.alloc(8)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, items.get(i).copied().unwrap_or(0));
            e.mem
                .set_u32(*node + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        nodes[0]
    }

    /// `_finite`, `_isnan` and the rounding `float` to `int` conversion
    /// (round to nearest even), the way the CRT computes them.
    fn install_float_helpers(e: &mut Engine) {
        fn double(a: &[u32]) -> f64 {
            f64::from_bits(a[0] as u64 | (a[1] as u64) << 32)
        }
        e.register(IS_FINITE, |_, a| ret(double(a).is_finite() as u32));
        e.register(IS_NAN, |_, a| ret(double(a).is_nan() as u32));
        e.register(FLOAT_ROUND, |_, a| {
            ret(f32::from_bits(a[0]).round_ties_even() as i32 as u32)
        });
    }

    fn set_double(e: &mut Engine, addr: u32, value: f64) {
        if !e.mem.is_mapped(addr) {
            e.map(addr & !0xfff, 0x1000);
        }
        e.set_global(addr, value);
    }

    /// A fake game object of `size` bytes with a vtable whose `slots`
    /// (byte offsets) go to `targets`.
    fn fake_object(e: &mut Engine, size: u32, targets: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        give_vtable(e, object, targets);
        object
    }

    /// A key as `fn_00587410` makes it.
    fn key(x: i16, y: i16) -> u32 {
        ((x as i32 as u32) << 16) | (y as u16 as u32)
    }

    // ---- parent values ----------------------------------------------------

    #[test]
    fn parent_for_gives_the_parent_only_when_the_value_is_inherited() {
        let mut e = engine();
        let w = world(&mut e);
        // No parent: 0 whatever the flags say.
        e.set(w, TESWorldSpace::sParentUseFlags, 0xffff);
        assert_eq!(e.call(0x0058_6390, &args![w, 2u32]).u32(), 0);
        // A parent, the value not inherited.
        e.set(w, TESWorldSpace::pParentWorld, Ptr::new(0x4444));
        e.set(w, TESWorldSpace::sParentUseFlags, 0b0000_0100);
        assert_eq!(e.call(0x0058_6390, &args![w, 1u32]).u32(), 0);
        // Inherited.
        assert_eq!(e.call(0x0058_6390, &args![w, 2u32]).u32(), 0x4444);
        // A number outside 0 to 5 is always inherited.
        assert_eq!(e.call(0x0058_6390, &args![w, 9u32]).u32(), 0x4444);
    }

    #[test]
    fn setting_the_parent_to_none_clears_the_inherited_flags() {
        let mut e = engine();
        let w = world(&mut e);
        e.set(w, TESWorldSpace::sParentUseFlags, 0x3f);
        e.call(0x0058_63d0, &args![w, 0x5000u32]);
        assert_eq!(e.get(w, TESWorldSpace::pParentWorld).addr(), 0x5000);
        assert_eq!(e.get(w, TESWorldSpace::sParentUseFlags), 0x3f);
        e.call(0x0058_63d0, &args![w, 0u32]);
        assert_eq!(e.get(w, TESWorldSpace::pParentWorld).addr(), 0);
        assert_eq!(e.get(w, TESWorldSpace::sParentUseFlags), 0);
    }

    #[test]
    fn world_map_data_comes_from_the_ancestor_that_owns_it() {
        let mut e = engine();
        let root = world(&mut e);
        let child = world(&mut e);
        let grandchild = world(&mut e);
        e.set(child, TESWorldSpace::pParentWorld, root.cast());
        e.set(child, TESWorldSpace::sParentUseFlags, 0b100);
        e.set(grandchild, TESWorldSpace::pParentWorld, child.cast());
        e.set(grandchild, TESWorldSpace::sParentUseFlags, 0b100);
        assert_eq!(e.call(0x0058_6400, &args![root]).u32(), root.addr() + 0x80);
        assert_eq!(e.call(0x0058_6400, &args![child]).u32(), root.addr() + 0x80);
        assert_eq!(
            e.call(0x0058_6400, &args![grandchild]).u32(),
            root.addr() + 0x80
        );
        // The child stops the chain by not inheriting.
        e.set(grandchild, TESWorldSpace::sParentUseFlags, 0);
        assert_eq!(
            e.call(0x0058_6400, &args![grandchild]).u32(),
            grandchild.addr() + 0x80
        );
    }

    #[test]
    fn world_map_texture_text_is_asked_of_the_owner_of_the_value() {
        let mut e = engine();
        // The text getter answers with the address of the object it is given.
        e.register(TEXTURE_NAME_TEXT, |_, a| ret(a[0]));
        let parent = world(&mut e);
        let child = world(&mut e);
        assert_eq!(
            e.call(0x0058_6440, &args![child]).u32(),
            child.addr() + 0x24
        );
        e.set(child, TESWorldSpace::pParentWorld, parent.cast());
        assert_eq!(
            e.call(0x0058_6440, &args![child]).u32(),
            child.addr() + 0x24
        );
        e.set(child, TESWorldSpace::sParentUseFlags, 0b100);
        assert_eq!(
            e.call(0x0058_6440, &args![child]).u32(),
            parent.addr() + 0x24
        );
    }

    #[test]
    fn reset_clears_the_world_space_values() {
        let mut e = engine();
        set_word(&mut e, RESET_LAND_HEIGHT, (-2048.0f32).to_bits());
        let w = world(&mut e);
        e.set(w, TESWorldSpace::fLODWaterHeight, 9.0);
        e.set(w, TESWorldSpace::fDefaultLandHeight, 9.0);
        e.set(w, TESWorldSpace::fDefaultWaterHeight, 9.0);
        e.mem.write(w.addr() + 0x80, &[0xff; 0x10]);
        e.set(w, TESWorldSpace::cFlags, 0x55);
        for setter in [SET_CLIMATE, SET_WATER, SET_LOD_WATER, TEXTURE_SET_NAME] {
            noop(&mut e, setter);
        }
        logged(&mut e);
        e.call(0x0058_6480, &args![w]);
        assert_eq!(calls_to(&e, SET_CLIMATE), vec![vec![w.addr(), 0]]);
        assert_eq!(calls_to(&e, SET_WATER), vec![vec![w.addr(), 0]]);
        assert_eq!(calls_to(&e, SET_LOD_WATER), vec![vec![w.addr(), 0]]);
        assert_eq!(
            calls_to(&e, TEXTURE_SET_NAME),
            vec![vec![w.addr() + 0x24, 0]]
        );
        assert_eq!(e.get(w, TESWorldSpace::fLODWaterHeight), 0.0);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultLandHeight), -2048.0);
        assert_eq!(e.get(w, TESWorldSpace::fDefaultWaterHeight), 0.0);
        assert_eq!(e.mem.bytes(w.addr() + 0x80, 0x10), vec![0; 0x10]);
        // Nothing else is touched.
        assert_eq!(e.get(w, TESWorldSpace::cFlags), 0x55);
    }

    #[test]
    fn default_location_text_comes_from_the_default_object() {
        let mut e = engine();
        e.register(DEFAULT_LOCATION_TEXT, |_, a| ret(a[0] + 1));
        assert_eq!(
            e.call(0x0058_6980, &args![]).u32(),
            DEFAULT_LOCATION_OBJECT + 1
        );
    }

    // ---- the location name (00586500) -----------------------------------------

    /// A C string in game memory.
    fn cstring(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    /// The strings of the location tests: the text of each `BSStringT` object
    /// by address.
    type Strings = Rc<RefCell<std::collections::HashMap<u32, String>>>;

    /// Doubles for `BSStringT` (set, text, length, compare) and the empty
    /// text; `names` is what the text getter of a form part answers.
    fn install_strings(e: &mut Engine, names: Vec<(u32, &'static str)>) -> Strings {
        set_word(e, EMPTY_TEXT, 0);
        let strings: Strings = Rc::default();
        let read = |e: &Engine, address: u32| -> String {
            String::from_utf8(e.mem.cstr(address)).unwrap()
        };
        let s = strings.clone();
        e.register_double(BSSTRING_SET, move |e, a| {
            let text = if a[1] == 0 {
                String::new()
            } else {
                read(e, a[1])
            };
            s.borrow_mut().insert(a[0], text);
            Ret::default()
        });
        let s = strings.clone();
        e.register_double(BSSTRING_TEXT, move |e, a| {
            let text = s.borrow().get(&a[0]).cloned().unwrap_or_default();
            ret(cstring(e, &text))
        });
        let s = strings.clone();
        e.register_double(BSSTRING_LENGTH, move |_, a| {
            ret(s.borrow().get(&a[0]).map_or(0, |t| t.len() as u32))
        });
        let s = strings.clone();
        e.register_double(BSSTRING_COMPARE, move |e, a| {
            let mine = s.borrow().get(&a[0]).cloned().unwrap_or_default();
            ret((!mine.eq_ignore_ascii_case(&read(e, a[1]))) as u32)
        });
        e.register_double(TEXTURE_NAME_TEXT, move |e, a| {
            match names.iter().find(|n| n.0 == a[0]) {
                Some((_, name)) => ret(cstring(e, name)),
                None => ret(EMPTY_TEXT),
            }
        });
        strings
    }

    /// The engine of the location tests: the data handler pointer, the
    /// statics and the callees every case reaches. Returns the world space,
    /// the data handler and an output string.
    fn location_engine() -> (Engine, Ptr<TESWorldSpace>, u32, u32) {
        let mut e = engine();
        // The location cache registers its destructor with `_atexit`: the start-up's exit table.
        engine_core::units::crt::start_up_test::exit_table(&mut e);
        let handler = e.mem.alloc(0x700);
        set_word(&mut e, DATA_HANDLER_POINTER, handler);
        set_word(&mut e, LOCATION_CACHE_WORLD, 0);
        set_word(&mut e, LOCATION_CACHE_GUARD, 0);
        for addr in [
            BSSTRING_CONSTRUCT,
            LOCAL_STRUCT_CONSTRUCT,
            REGION_POINT_BUILD,
        ] {
            noop(&mut e, addr);
        }
        e.register(POINT_EQUAL, |_, _| ret(0));
        install_lists(&mut e);
        let w = world(&mut e);
        let out = e.mem.alloc(8);
        (e, w, handler, out)
    }

    /// Calls the location name function for the point (`x`, 20, 30).
    fn location_name(e: &mut Engine, w: Ptr<TESWorldSpace>, out: u32, x: f32) -> bool {
        e.call(0x0058_6500, &args![w, out, x, 20.0f32, 30.0f32])
            .bool()
    }

    /// Makes the data handler find `cell` for every point and the region
    /// manager give `region_data` (0 for none) for the cell's region list.
    fn location_cell(e: &mut Engine, cell: u32, region_data: u32) {
        e.register_double(DATA_HANDLER_CELL_FROM_COORD, move |_, _| ret(cell));
        e.register(CELL_GET_REGION_LIST, |_, _| ret(0x1000));
        e.register(REGION_LIST_GET_DERIVED_DATA, |_, _| ret(0x2000));
        let manager = fake_object(e, 0x40, &[(0x10, 0x7000_0010)]);
        e.register_double(0x7000_0010, move |_, a| {
            ret(if a[1] == 0x2000 { region_data } else { 0 })
        });
        e.register_double(DATA_HANDLER_REGION_MANAGER, move |_, _| ret(manager));
    }

    #[test]
    fn location_name_does_nothing_without_a_data_handler() {
        let (mut e, w, _, out) = location_engine();
        set_word(&mut e, DATA_HANDLER_POINTER, 0);
        let strings = install_strings(&mut e, vec![]);
        assert!(!location_name(&mut e, w, out, 10.0));
        assert!(strings.borrow().is_empty());
        // The statics were still initialized.
        assert_eq!(e.global::<u32>(LOCATION_CACHE_GUARD), 3);
    }

    #[test]
    fn location_name_from_the_region_data_of_the_cell() {
        let (mut e, w, _, out) = location_engine();
        let strings = install_strings(&mut e, vec![]);
        let cell = e.mem.alloc(0x100);
        // The region data's slot +0x28 writes the name.
        let region_data = fake_object(&mut e, 0x10, &[(0x28, 0x7000_0028)]);
        let s = strings.clone();
        e.register_double(0x7000_0028, move |_, a| {
            s.borrow_mut().insert(a[1], "Mojave".to_string());
            Ret::default()
        });
        location_cell(&mut e, cell, region_data);
        logged(&mut e);
        assert!(location_name(&mut e, w, out, 10.0));
        assert_eq!(strings.borrow()[&out], "Mojave");
        // The answer is cached with the world space and the point.
        assert_eq!(e.global::<u32>(LOCATION_CACHE_WORLD), w.addr());
        assert_eq!(e.global::<f32>(LOCATION_CACHE_POINT), 10.0);
        assert_eq!(e.global::<f32>(LOCATION_CACHE_POINT + 4), 20.0);
        assert_eq!(e.global::<f32>(LOCATION_CACHE_POINT + 8), 30.0);
        assert_eq!(strings.borrow()[&LOCATION_CACHE_TEXT], "Mojave");
        // The region data was asked with kind 4 for the point and the world.
        let derived = calls_to(&e, REGION_LIST_GET_DERIVED_DATA);
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0][1], 4);
        assert_eq!(derived[0][3], w.addr());
        // The same place again: the cached text, and the answer is "no
        // change".
        e.register(POINT_EQUAL, |_, _| ret(1));
        strings.borrow_mut().remove(&out);
        assert!(!location_name(&mut e, w, out, 10.0));
        assert_eq!(strings.borrow()[&out], "Mojave");
        // Another point of the same world space is computed again; the text
        // is the same, so no change.
        e.register(POINT_EQUAL, |_, _| ret(0));
        assert!(!location_name(&mut e, w, out, 11.0));
        assert_eq!(e.global::<f32>(LOCATION_CACHE_POINT), 11.0);
        // Another world space: the cached text is emptied first, so the same
        // name counts as a change.
        let other = world(&mut e);
        assert!(location_name(&mut e, other, out, 11.0));
        assert_eq!(e.global::<u32>(LOCATION_CACHE_WORLD), other.addr());
    }

    #[test]
    fn location_name_uses_the_name_of_the_cell_without_region_data() {
        let (mut e, w, _, out) = location_engine();
        let cell = e.mem.alloc(0x100);
        let strings = install_strings(&mut e, vec![(cell + 0x18, "Goodsprings")]);
        location_cell(&mut e, cell, 0);
        assert!(location_name(&mut e, w, out, 1.0));
        assert_eq!(strings.borrow()[&out], "Goodsprings");
    }

    #[test]
    fn location_name_falls_back_to_the_world_space_name() {
        let (mut e, w, _, out) = location_engine();
        let cell = e.mem.alloc(0x100);
        let names = vec![(w.addr() + 0x18, "Mojave Wasteland")];
        let strings = install_strings(&mut e, names);
        location_cell(&mut e, cell, 0);
        assert!(location_name(&mut e, w, out, 2.0));
        assert_eq!(strings.borrow()[&out], "Mojave Wasteland");
    }

    #[test]
    fn location_name_uses_the_default_text_when_nothing_has_a_name() {
        let (mut e, w, _, out) = location_engine();
        let strings = install_strings(&mut e, vec![]);
        e.register(DATA_HANDLER_CELL_FROM_COORD, |_, _| ret(0));
        e.register(DATA_HANDLER_LIST_1D8, |_, _| ret(0));
        let default = cstring(&mut e, "Wasteland");
        e.register_double(DEFAULT_LOCATION_TEXT, move |_, _| ret(default));
        assert!(location_name(&mut e, w, out, 1.0));
        assert_eq!(strings.borrow()[&out], "Wasteland");
        assert_eq!(strings.borrow()[&LOCATION_CACHE_TEXT], "Wasteland");
    }

    /// A region entry for the no-cell scan: +8 flags, +0x18 the region
    /// data, +0x1c the list of areas, +0x20 the world space.
    fn region_entry(e: &mut Engine, world: u32, area: u32, region_data: u32, flags: u32) -> u32 {
        let entry = e.mem.alloc(0x40);
        e.mem.set_u32(entry + 8, flags);
        e.mem.set_u32(entry + 0x18, region_data);
        let areas = make_list(e, &[area]);
        e.mem.set_u32(entry + 0x1c, areas);
        e.mem.set_u32(entry + 0x20, world);
        entry
    }

    /// A region data: +4 the "preferred" byte, +6 the priority; its slot
    /// +0x28 is `0x7000_0028`.
    fn region_data(e: &mut Engine, preferred: u8, priority: u8) -> u32 {
        let data = fake_object(e, 0x10, &[(0x28, 0x7000_0028)]);
        e.mem.set_u8(data + 4, preferred);
        e.mem.set_u8(data + 6, priority);
        data
    }

    /// Sets up the scan for a point with no cell over `entries` (region
    /// entries), the names being what each region data writes; returns the
    /// order in which the region data wrote their names.
    fn scan_engine(
        e: &mut Engine,
        strings: &Strings,
        handler: u32,
        entries: &[u32],
        names: Vec<(u32, &'static str)>,
    ) -> Rc<RefCell<Vec<u32>>> {
        e.register(DATA_HANDLER_CELL_FROM_COORD, |_, _| ret(0));
        // The handler's +0x1d8 word points at the list holder; its nodes
        // start at +4.
        let holder = e.mem.alloc(0x10);
        let head = make_list(e, entries);
        embed_list(e, holder + 4, head);
        e.mem.set_u32(handler + 0x1d8, holder);
        e.register(DATA_HANDLER_LIST_1D8, |e, a| ret(e.mem.u32(a[0] + 0x1d8)));
        let written = Rc::new(RefCell::new(Vec::new()));
        let (s, w) = (strings.clone(), written.clone());
        e.register_double(0x7000_0028, move |_, a| {
            let name = names.iter().find(|n| n.0 == a[0]).unwrap().1;
            s.borrow_mut().insert(a[1], name.to_string());
            w.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register(REGION_ENTRY_FLAG_20, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x20 != 0) as u32)
        });
        e.register(REFERENCE_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(REGION_ENTRY_LIST, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(REGION_ENTRY_WORD_18, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(REGION_DATA_LIST_FIND, |_, a| ret(a[0]));
        // An area item with the low bit set contains the point.
        e.register(REGION_POINT_IN_ENTRY, |_, a| ret(a[0] & 1));
        e.register(REGION_DATA_BYTE_4, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(REGION_DATA_BYTE_6, |e, a| ret(e.mem.u8(a[0] + 6) as u32));
        let manager = fake_object(e, 0x40, &[(0x10, 0x7000_0010)]);
        e.register(0x7000_0010, |_, a| ret(a[1]));
        e.register_double(DATA_HANDLER_REGION_MANAGER, move |_, _| ret(manager));
        e.register(DEFAULT_LOCATION_TEXT, |_, _| ret(0));
        written
    }

    #[test]
    fn location_name_without_a_cell_takes_the_best_region_data() {
        let (mut e, w, handler, out) = location_engine();
        let strings = install_strings(&mut e, vec![]);
        let low = region_data(&mut e, 0, 5);
        let high = region_data(&mut e, 0, 9);
        let preferred = region_data(&mut e, 1, 2);
        let other_world = region_data(&mut e, 1, 200);
        let outside = region_data(&mut e, 1, 100);
        let hidden = region_data(&mut e, 1, 100);
        let me = w.addr();
        let entries = [
            region_entry(&mut e, me, 1, low, 0),
            region_entry(&mut e, me, 1, high, 0),
            region_entry(&mut e, me, 1, preferred, 0),
            region_entry(&mut e, 0x9999, 1, other_world, 0),
            region_entry(&mut e, me, 2, outside, 0),
            region_entry(&mut e, me, 1, hidden, 0x20),
        ];
        let names = vec![
            (low, "low"),
            (high, "high"),
            (preferred, "preferred"),
            (other_world, "other world"),
            (outside, "outside"),
            (hidden, "hidden"),
        ];
        let written = scan_engine(&mut e, &strings, handler, &entries, names);
        // `low` is taken first, `high` beats it by priority, `preferred`
        // (the flag byte) beats both. `other_world` is another world space's,
        // `outside` does not hold the point, `hidden` has flag 0x20.
        assert!(location_name(&mut e, w, out, 1.0));
        assert_eq!(*written.borrow(), vec![low, high, preferred]);
        assert_eq!(strings.borrow()[&out], "preferred");
    }

    #[test]
    fn location_name_keeps_a_preferred_region_against_later_ones() {
        let (mut e, w, handler, out) = location_engine();
        let strings = install_strings(&mut e, vec![]);
        let first = region_data(&mut e, 1, 50);
        let weaker = region_data(&mut e, 1, 10);
        let plain = region_data(&mut e, 0, 255);
        let stronger = region_data(&mut e, 1, 60);
        let me = w.addr();
        let entries = [
            region_entry(&mut e, me, 1, first, 0),
            region_entry(&mut e, me, 1, weaker, 0),
            region_entry(&mut e, me, 1, plain, 0),
            region_entry(&mut e, me, 1, stronger, 0),
        ];
        let names = vec![
            (first, "first"),
            (weaker, "weaker"),
            (plain, "plain"),
            (stronger, "stronger"),
        ];
        let written = scan_engine(&mut e, &strings, handler, &entries, names);
        // `weaker` and `plain` lose to `first`; `stronger` wins.
        assert!(location_name(&mut e, w, out, 1.0));
        assert_eq!(*written.borrow(), vec![first, stronger]);
        assert_eq!(strings.borrow()[&out], "stronger");
    }

    // ---- the grass of an area (00586990) --------------------------------------

    /// A fake object of the grass test (a region data entry or a grass form)
    /// with the slots of both. Fields: +0x10 the object slot 4 returns, +0x14
    /// slot 0xc, +0x18 the weight (slot 0x18), +0x1c the byte of slot 0x180,
    /// +0x20 to +0x2c the floats of slots 0x1b0, 0x1b8, 0x1c0, 0x1c8, +0x30
    /// to +0x32 the bytes of slots 0x1d0, 0x1d8, 0x1e0, +0x0c the form ID.
    fn grass_object(e: &mut Engine) -> u32 {
        fake_object(
            e,
            0x60,
            &[
                (4, 0x7100_0004),
                (0xc, 0x7100_000c),
                (0x18, 0x7100_0018),
                (0x180, 0x7100_0180),
                (0x1b0, 0x7100_01b0),
                (0x1b8, 0x7100_01b8),
                (0x1c0, 0x7100_01c0),
                (0x1c8, 0x7100_01c8),
                (0x1d0, 0x7100_01d0),
                (0x1d8, 0x7100_01d8),
                (0x1e0, 0x7100_01e0),
            ],
        )
    }

    fn install_grass_slots(e: &mut Engine) {
        e.register(0x7100_0004, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x7100_000c, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        // The weight: the stored value at the centre (flag 0); the x
        // coordinate of the sample point plus the stored value at a sample
        // point (flag 1).
        e.register(0x7100_0018, |e, a| {
            let stored = e.mem.f32(a[0] + 0x18);
            if a[3] == 1 {
                ret_float(e.mem.f32(a[1]) + stored)
            } else {
                ret_float(stored)
            }
        });
        e.register(0x7100_0180, |e, a| ret(e.mem.u8(a[0] + 0x1c) as u32));
        e.register(0x7100_01b0, |e, a| ret_float(e.mem.f32(a[0] + 0x20)));
        e.register(0x7100_01b8, |e, a| ret_float(e.mem.f32(a[0] + 0x24)));
        e.register(0x7100_01c0, |e, a| ret_float(e.mem.f32(a[0] + 0x28)));
        e.register(0x7100_01c8, |e, a| ret_float(e.mem.f32(a[0] + 0x2c)));
        e.register(0x7100_01d0, |e, a| ret(e.mem.u8(a[0] + 0x30) as u32));
        e.register(0x7100_01d8, |e, a| ret(e.mem.u8(a[0] + 0x31) as u32));
        e.register(0x7100_01e0, |e, a| ret(e.mem.u8(a[0] + 0x32) as u32));
    }

    /// A grass form: its form ID, the byte of slot 0x180 (the weight in
    /// percent), the four floats and three bytes of the other slots.
    fn grass_form(
        e: &mut Engine,
        form_id: u32,
        percent: u8,
        floats: [f32; 4],
        bytes: [u8; 3],
    ) -> u32 {
        let form = grass_object(e);
        e.mem.set_u32(form + 0xc, form_id);
        e.mem.set_u8(form + 0x1c, percent);
        for (i, value) in floats.iter().enumerate() {
            e.mem.set_f32(form + 0x20 + i as u32 * 4, *value);
        }
        for (i, value) in bytes.iter().enumerate() {
            e.mem.set_u8(form + 0x30 + i as u32, *value);
        }
        form
    }

    /// A region data entry pointing at `form`, with the result of slot 0xc
    /// and the weight at the centre.
    fn grass_entry(e: &mut Engine, form: u32, slot_c: u32, weight: f32) -> u32 {
        let entry = grass_object(e);
        e.mem.set_u32(entry + 0x10, form);
        e.mem.set_u32(entry + 0x14, slot_c);
        e.mem.set_f32(entry + 0x18, weight);
        entry
    }

    /// The engine of the grass tests: the data handler, a cell and region
    /// data whose slot +0x28 gives the list at `holder + 4` (set it with
    /// `set_grass_entries`). Returns the world space, the handler and the
    /// holder.
    fn grass_engine() -> (Engine, Ptr<TESWorldSpace>, u32, u32) {
        let mut e = engine();
        let handler = e.mem.alloc(0x700);
        set_word(&mut e, DATA_HANDLER_POINTER, handler);
        set_double(&mut e, DOUBLE_TWO, 2.0);
        set_double(&mut e, DOUBLE_ZERO, 0.0);
        set_double(&mut e, DOUBLE_HUNDRED, 100.0);
        install_grass_slots(&mut e);
        install_lists(&mut e);
        e.register(LOCAL_STRUCT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(DATA_HANDLER_CELL_FROM_COORD, |_, _| ret(0x5000));
        e.register(CELL_GET_REGION_LIST, |_, _| ret(0x1000));
        e.register(REGION_LIST_GET_DERIVED_DATA, |_, _| ret(0x2000));
        let holder = e.mem.alloc(0x10);
        let region_data = fake_object(&mut e, 0x10, &[(0x28, 0x7100_0028)]);
        e.register_double(0x7100_0028, move |_, _| ret(holder));
        let manager = fake_object(&mut e, 0x40, &[(0x18, 0x7100_1018)]);
        e.register_double(0x7100_1018, move |_, _| ret(region_data));
        e.register_double(DATA_HANDLER_REGION_MANAGER, move |_, _| ret(manager));
        // The mesh path is "data/meshes/" and the form's address in hex.
        e.register(FORM_NAME_TEXT, |_, a| ret(a[0]));
        e.register(SPRINTF_S, |e, a| {
            let text = format!("data/meshes/{:x}", a[3]);
            e.mem.set_cstr(a[0], text.as_bytes());
            ret(text.len() as u32)
        });
        e.register(GRASS_SETTING_FLOAT, |_, _| ret_float(0.5));
        let w = world(&mut e);
        (e, w, handler, holder)
    }

    fn set_grass_entries(e: &mut Engine, holder: u32, entries: &[u32]) {
        let head = make_list(e, entries);
        embed_list(e, holder + 4, head);
    }

    #[test]
    fn grass_does_nothing_without_a_data_handler_or_a_count() {
        for (handler_set, count) in [(false, 1u32), (true, 0u32)] {
            let (mut e, w, _, _) = grass_engine();
            let out = e.mem.alloc(0x100);
            e.mem.set_u32(out, 0x1234);
            if !handler_set {
                set_word(&mut e, DATA_HANDLER_POINTER, 0);
            }
            logged(&mut e);
            e.call(
                0x0058_6990,
                &args![w, 0.0f32, 0.0f32, 8192.0f32, 8192.0f32, out, count],
            );
            assert_eq!(e.mem.u32(out), 0x1234);
            assert!(calls_to(&e, DATA_HANDLER_CELL_FROM_COORD).is_empty());
        }
    }

    #[test]
    fn grass_stops_without_a_cell_or_a_region_list() {
        let (mut e, w, _, _) = grass_engine();
        let out = e.mem.alloc(0x100);
        e.mem.set_u32(out, 0x1234);
        e.register(DATA_HANDLER_CELL_FROM_COORD, |_, _| ret(0));
        logged(&mut e);
        e.call(
            0x0058_6990,
            &args![w, 0.0f32, 0.0f32, 8192.0f32, 8192.0f32, out, 1u32],
        );
        assert_eq!(e.mem.u32(out), 0x1234);
        assert!(calls_to(&e, REGION_LIST_GET_DERIVED_DATA).is_empty());
        // A cell without a region list.
        let (mut e, w, _, _) = grass_engine();
        let out = e.mem.alloc(0x100);
        e.mem.set_u32(out, 0x1234);
        e.register(CELL_GET_REGION_LIST, |_, _| ret(0));
        logged(&mut e);
        e.call(
            0x0058_6990,
            &args![w, 0.0f32, 0.0f32, 8192.0f32, 8192.0f32, out, 1u32],
        );
        assert_eq!(e.mem.u32(out), 0x1234);
        assert!(calls_to(&e, REGION_LIST_GET_DERIVED_DATA).is_empty());
    }

    #[test]
    fn grass_asks_the_cell_in_the_middle_of_the_area() {
        let (mut e, w, handler, _) = grass_engine();
        let out = e.mem.alloc(0x100);
        e.register(DATA_HANDLER_CELL_FROM_COORD, |_, _| ret(0));
        logged(&mut e);
        e.call(
            0x0058_6990,
            &args![w, 100.0f32, 200.0f32, 300.0f32, 600.0f32, out, 1u32],
        );
        assert_eq!(
            calls_to(&e, DATA_HANDLER_CELL_FROM_COORD),
            vec![vec![
                handler,
                200.0f32.to_bits(),
                400.0f32.to_bits(),
                w.addr(),
                0
            ]]
        );
        // The nine sample points were constructed first.
        let made = calls_to(&e, VECTOR_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1..], [0xC, 9, LOCAL_STRUCT_CONSTRUCT]);
    }

    #[test]
    fn grass_with_no_wanted_entry_leaves_the_output_alone() {
        let (mut e, w, _, holder) = grass_engine();
        let out = e.mem.alloc(0x100);
        e.mem.set_u32(out, 0x1234);
        // One entry with no weight at the centre.
        let form = grass_form(&mut e, 1, 10, [0.0; 4], [0; 3]);
        let entry = grass_entry(&mut e, form, 1, 0.0);
        set_grass_entries(&mut e, holder, &[entry]);
        e.call(
            0x0058_6990,
            &args![w, 0.0f32, 0.0f32, 8192.0f32, 8192.0f32, out, 1u32],
        );
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    #[test]
    fn grass_fills_the_entries_and_the_weights_of_the_sample_points() {
        let (mut e, w, _, holder) = grass_engine();
        let form_a = grass_form(&mut e, 0xAAAA, 50, [1.0, 2.0, 3.0, 4.0], [7, 8, 9]);
        let form_b = grass_form(&mut e, 0xBBBB, 25, [5.0, 6.0, 7.0, 8.0], [1, 2, 3]);
        let form_off = grass_form(&mut e, 0xCCCC, 0, [0.0; 4], [0; 3]);
        let form_c = grass_form(&mut e, 0xDDDD, 10, [0.0; 4], [0; 3]);
        let entries = [
            grass_entry(&mut e, form_b, 0, 0.25),
            grass_entry(&mut e, form_a, 1, 0.5),
            grass_entry(&mut e, form_off, 1, 0.5), // slot 0x180 is 0
            grass_entry(&mut e, form_c, 1, 0.0),   // no weight at the centre
            grass_entry(&mut e, 0, 1, 0.5),        // no form
        ];
        set_grass_entries(&mut e, holder, &entries);
        let out = e.mem.alloc(0x44 * 3);
        // The third output entry is stale: its path is freed and cleared.
        let stale = e.mem.alloc(8);
        e.mem.set_u32(out + 0x44 * 2, stale);
        e.mem.write(out + 0x44 * 2 + 0x20, &[0xff; 0x24]);
        e.call(
            0x0058_6990,
            &args![w, 0.0f32, 0.0f32, 8192.0f32, 8192.0f32, out, 3u32],
        );
        let path = |e: &Engine, at: u32| String::from_utf8(e.mem.cstr(e.mem.u32(at))).unwrap();
        // The list is made by adding at the head: the entry of the first
        // pass (slot 0xc non-zero: form A) is second, the entry of the
        // second pass (form B) first.
        let first = out;
        assert_eq!(path(&e, first), format!("data/meshes/{:x}", form_b));
        assert_eq!(e.mem.u32(first + 4), 0xBBBB);
        assert_eq!(e.mem.f32(first + 8), 5.0);
        assert_eq!(e.mem.f32(first + 0xc), 6.0);
        assert_eq!(e.mem.f32(first + 0x10), 7.0);
        assert_eq!(e.mem.f32(first + 0x18), 8.0);
        assert_eq!(e.mem.u8(first + 0x1c), 1);
        assert_eq!(e.mem.u8(first + 0x1d), 2);
        assert_eq!(e.mem.u8(first + 0x1e), 3);
        assert_eq!(e.mem.f32(first + 0x14), 0.5);
        let second = out + 0x44;
        assert_eq!(path(&e, second), format!("data/meshes/{:x}", form_a));
        assert_eq!(e.mem.u32(second + 4), 0xAAAA);
        assert_eq!(e.mem.f32(second + 8), 1.0);
        assert_eq!(e.mem.u8(second + 0x1e), 9);
        // The weights at the nine points: x of the point plus the stored
        // weight; the points are 0, 4096, 8192 along x.
        for sample in 0..9u32 {
            let x = (sample % 3) as f32 * 4096.0;
            assert_eq!(e.mem.f32(first + 0x20 + sample * 4), x + 0.25, "{sample}");
            assert_eq!(e.mem.f32(second + 0x20 + sample * 4), x + 0.5, "{sample}");
        }
        // The third entry has nothing: no path, the weights cleared.
        let third = out + 0x44 * 2;
        assert_eq!(e.mem.u32(third), 0);
        assert_eq!(e.mem.bytes(third + 0x20, 0x24), vec![0; 0x24]);
    }

    #[test]
    fn grass_weights_use_the_nine_points_of_the_area() {
        let (mut e, w, _, holder) = grass_engine();
        let form = grass_form(&mut e, 1, 100, [0.0; 4], [0; 3]);
        let entry = grass_entry(&mut e, form, 1, 0.0625);
        set_grass_entries(&mut e, holder, &[entry]);
        let out = e.mem.alloc(0x44);
        // The y coordinates differ: the area is 100..300 by 10..90.
        e.call(
            0x0058_6990,
            &args![w, 100.0f32, 10.0f32, 300.0f32, 90.0f32, out, 1u32],
        );
        for sample in 0..9u32 {
            let x = 100.0 + (sample % 3) as f32 * 100.0;
            assert_eq!(e.mem.f32(out + 0x20 + sample * 4), x + 0.0625, "{sample}");
        }
    }

    // ---- cell keys and lookups ------------------------------------------------

    #[test]
    fn cell_key_packs_the_coordinates() {
        let mut e = engine();
        assert_eq!(e.call(0x0058_7410, &args![3i16, -2i16]).u32(), 0x0003_fffe);
        assert_eq!(e.call(0x0058_7410, &args![-1i16, 0i16]).u32(), 0xffff_0000);
        assert_eq!(
            e.call(0x0058_7410, &args![0x7fffi16, -0x8000i16]).u32(),
            0x7fff_8000
        );
    }

    #[test]
    fn cell_key_splits_into_the_coordinates() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        e.call(0x0058_7520, &args![0xfffe_0007u32, out, out + 4]);
        assert_eq!(e.mem.u16(out), 0xfffe);
        assert_eq!(e.mem.u16(out + 4), 7);
    }

    #[test]
    fn key_for_a_world_coordinate_rounds_and_shifts() {
        let mut e = engine();
        install_float_helpers(&mut e);
        let point = e.mem.alloc(12);
        for (x, y, expected) in [
            (4096.0f32 * 3.0 + 1.0, -8192.0f32, key(3, -2)),
            (0.0, 0.0, key(0, 0)),
            (-1.0, 4095.0, key(-1, 0)),
            (4095.6, 4096.4, key(1, 1)),
        ] {
            e.mem.set_f32(point, x);
            e.mem.set_f32(point + 4, y);
            e.mem.set_f32(point + 8, 5.0);
            assert_eq!(
                e.call(0x0058_7440, &args![point]).u32(),
                expected,
                "{x} {y}"
            );
        }
    }

    #[test]
    fn key_for_a_world_coordinate_refuses_non_numbers() {
        let mut e = engine();
        install_float_helpers(&mut e);
        let point = e.mem.alloc(12);
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for slot in 0..3 {
                for i in 0..3 {
                    e.mem.set_f32(point + i * 4, 1.0);
                }
                e.mem.set_f32(point + slot * 4, bad);
                assert_eq!(e.call(0x0058_7440, &args![point]).u32(), 0x7fff_7fff);
            }
        }
    }

    /// A world space whose cell map holds `cells` (x, y, cell).
    fn world_with_cells(
        e: &mut Engine,
        cells: &[(i16, i16, u32)],
    ) -> (Ptr<TESWorldSpace>, MapTable) {
        let table = install_maps(e);
        let w = world(e);
        let map = e.mem.alloc(0x10);
        e.set(w, TESWorldSpace::pCellMap, Ptr::new(map));
        for (x, y, cell) in cells {
            table.borrow_mut().push((map, key(*x, *y), *cell));
        }
        (w, table)
    }

    #[test]
    fn cell_from_cell_coord_finds_cells_in_range() {
        let mut e = engine();
        let (w, _) = world_with_cells(&mut e, &[(3, -2, 0xCE11), (-0x8000, 0x7fff, 0xED6E)]);
        noop(&mut e, LOG);
        assert_eq!(e.call(0x0058_75a0, &args![w, 3u32, -2i32]).u32(), 0xCE11);
        assert_eq!(e.call(0x0058_75a0, &args![w, 4u32, -2i32]).u32(), 0);
        assert_eq!(
            e.call(0x0058_75a0, &args![w, -0x8000i32, 0x7fffi32]).u32(),
            0xED6E
        );
    }

    #[test]
    fn cell_from_cell_coord_logs_and_refuses_coordinates_out_of_range() {
        let mut e = engine();
        let (w, _) = world_with_cells(&mut e, &[]);
        noop(&mut e, LOG);
        logged(&mut e);
        for (x, y) in [(0x8000, 0), (0, 0x8000), (-0x8001, 0), (0, -0x8001)] {
            assert_eq!(e.call(0x0058_75a0, &args![w, x, y]).u32(), 0);
        }
        let logs = calls_to(&e, LOG);
        assert_eq!(logs.len(), 4);
        assert_eq!(logs[0], vec![MSG_INVALID_CELL_COORD, 0xffff_8000, 0x7fff]);
        // The map was not asked.
        assert!(calls_to(&e, MAP_GET).is_empty());
    }

    #[test]
    fn cell_from_world_coord_converts_the_point_to_cell_coordinates() {
        let mut e = engine();
        install_float_helpers(&mut e);
        let (w, _) = world_with_cells(&mut e, &[(1, -1, 0xCE11)]);
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 5000.0);
        e.mem.set_f32(point + 4, -100.0);
        assert_eq!(e.call(0x0058_7550, &args![w, point]).u32(), 0xCE11);
        e.mem.set_f32(point, 9000.0);
        assert_eq!(e.call(0x0058_7550, &args![w, point]).u32(), 0);
    }

    #[test]
    fn cell_from_key_reads_the_cell_map() {
        let mut e = engine();
        let (w, _) = world_with_cells(&mut e, &[(5, 6, 0xCE11)]);
        assert_eq!(e.call(0x0058_7630, &args![w, key(5, 6)]).u32(), 0xCE11);
        assert_eq!(e.call(0x0058_7630, &args![w, key(6, 5)]).u32(), 0);
    }

    // ---- adding and releasing cells ---------------------------------------------

    /// A cell for the add/release tests: +0x24 flag bits, +0x8 the form
    /// flags, +0x80 x and +0x84 y; its slot +0x130 answers `0xED17`.
    fn fake_cell(e: &mut Engine, x: i32, y: i32, flags24: u8, form_flags: u32) -> u32 {
        let cell = fake_object(
            e,
            0x100,
            &[(SLOT_EDITOR_ID, 0x7300_0130), (0x10, 0x7300_0010)],
        );
        e.mem.set_u8(cell + 0x24, flags24);
        e.mem.set_u32(cell + 8, form_flags);
        e.mem.set_i32(cell + 0x80, x);
        e.mem.set_i32(cell + 0x84, y);
        cell
    }

    fn install_cell_functions(e: &mut Engine) {
        e.register(CELL_FLAG_24_BIT_0, |e, a| {
            ret((e.mem.u8(a[0] + 0x24) & 1) as u32)
        });
        e.register(FORM_FLAG_400, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x400 != 0) as u32)
        });
        e.register(FORM_FLAG_4000, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x4000 != 0) as u32)
        });
        e.register(CELL_GET_DATA_X, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(CELL_GET_DATA_Y, |e, a| ret(e.mem.u32(a[0] + 0x84)));
        e.register(0x7300_0130, |_, a| ret(a[0] + 0x90));
    }

    #[test]
    fn add_cell_puts_an_exterior_cell_in_the_map() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        let map = e.get(w, TESWorldSpace::pCellMap).addr();
        noop(&mut e, CELL_SET_WORLD_SPACE);
        logged(&mut e);
        let cell = fake_cell(&mut e, 4, -5, 0, 0);
        assert!(e.call(0x0058_7670, &args![w, cell]).bool());
        assert_eq!(*table.borrow(), vec![(map, key(4, -5), cell)]);
        assert_eq!(
            calls_to(&e, CELL_SET_WORLD_SPACE),
            vec![vec![cell, w.addr()]]
        );
    }

    #[test]
    fn add_cell_refuses_nothing_flagged_or_taken() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        noop(&mut e, CELL_SET_WORLD_SPACE);
        noop(&mut e, LOG);
        // No cell.
        assert!(!e.call(0x0058_7670, &args![w, 0u32]).bool());
        // A cell with bit 0 of the byte at +0x24.
        let flagged = fake_cell(&mut e, 1, 1, 1, 0);
        assert!(!e.call(0x0058_7670, &args![w, flagged]).bool());
        assert!(table.borrow().is_empty());
        // The key is taken: logged with the form ID, name and coordinates.
        let first = fake_cell(&mut e, 7, 8, 0, 0);
        e.mem.set_u32(first + 0xC, 0x00AB_CDEF);
        assert!(e.call(0x0058_7670, &args![w, first]).bool());
        let second = fake_cell(&mut e, 7, 8, 0, 0);
        logged(&mut e);
        assert!(!e.call(0x0058_7670, &args![w, second]).bool());
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![MSG_CELL_EXISTS, 0x00AB_CDEF, first + 0x90, 7, 8]]
        );
        assert_eq!(table.borrow().len(), 1);
        assert!(calls_to(&e, CELL_SET_WORLD_SPACE).is_empty());
    }

    #[test]
    fn add_cell_makes_a_persistent_cell_the_persistent_cell_once() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        noop(&mut e, CELL_SET_WORLD_SPACE);
        let persistent = fake_cell(&mut e, 0, 0, 0, 0x400);
        assert!(e.call(0x0058_7670, &args![w, persistent]).bool());
        assert_eq!(e.get(w, TESWorldSpace::pPersistentCell).addr(), persistent);
        // It is not in the map.
        assert!(table.borrow().is_empty());
        let another = fake_cell(&mut e, 0, 0, 0, 0x400);
        logged(&mut e);
        assert!(!e.call(0x0058_7670, &args![w, another]).bool());
        assert_eq!(e.get(w, TESWorldSpace::pPersistentCell).addr(), persistent);
        assert!(calls_to(&e, CELL_SET_WORLD_SPACE).is_empty());
    }

    #[test]
    fn release_cell_removes_only_the_stored_cell() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        noop(&mut e, CELL_SET_WORLD_SPACE);
        let cell = fake_cell(&mut e, 2, 3, 0, 0);
        let impostor = fake_cell(&mut e, 2, 3, 0, 0);
        let map = e.get(w, TESWorldSpace::pCellMap).addr();
        table.borrow_mut().push((map, key(2, 3), cell));
        logged(&mut e);
        // Not the cell stored under the key: nothing happens.
        e.call(0x0058_7760, &args![w, impostor]);
        assert_eq!(table.borrow().len(), 1);
        assert!(calls_to(&e, CELL_SET_WORLD_SPACE).is_empty());
        // Null and flagged cells: nothing.
        e.call(0x0058_7760, &args![w, 0u32]);
        let flagged = fake_cell(&mut e, 2, 3, 1, 0);
        e.call(0x0058_7760, &args![w, flagged]);
        assert_eq!(table.borrow().len(), 1);
        // The stored one.
        e.call(0x0058_7760, &args![w, cell]);
        assert!(table.borrow().is_empty());
        assert_eq!(calls_to(&e, CELL_SET_WORLD_SPACE), vec![vec![cell, 0]]);
    }

    #[test]
    fn deleting_all_cells_calls_every_destructor_and_empties_the_map() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        let map = e.get(w, TESWorldSpace::pCellMap).addr();
        e.register(0x7300_0010, |_, _| Ret::default());
        let a = fake_cell(&mut e, 0, 0, 0, 0);
        let b = fake_cell(&mut e, 1, 0, 0, 0);
        table
            .borrow_mut()
            .extend([(map, 1, a), (map, 2, b), (map, 3, 0)]);
        logged(&mut e);
        e.call(0x0058_77e0, &args![w]);
        let destroyed: Vec<_> = calls_to(&e, 0x7300_0010);
        assert_eq!(destroyed, vec![vec![a, 1], vec![b, 1]]);
        assert!(table.borrow().is_empty());
        // With form flag 0x4000 the cells are left alone, the map is emptied.
        table.borrow_mut().extend([(map, 1, a)]);
        e.mem.set_u32(w.addr() + 8, 0x4000);
        logged(&mut e);
        e.call(0x0058_77e0, &args![w]);
        assert!(calls_to(&e, 0x7300_0010).is_empty());
        assert!(table.borrow().is_empty());
    }

    #[test]
    fn overlapped_multibound_list_of_a_cell() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let table = install_maps(&mut e);
        let w = world(&mut e);
        let cell = fake_cell(&mut e, 4, 9, 0, 0);
        // No map yet.
        assert_eq!(e.call(0x0058_7870, &args![w, cell]).u32(), 0);
        let map = e.mem.alloc(0x10);
        e.set(w, TESWorldSpace::pOverlappedMultiboundMap, Ptr::new(map));
        assert_eq!(e.call(0x0058_7870, &args![w, cell]).u32(), 0);
        table.borrow_mut().push((map, key(4, 9), 0xA157));
        assert_eq!(e.call(0x0058_7870, &args![w, cell]).u32(), 0xA157);
    }

    // ---- multibound references ----------------------------------------------------

    /// A fake reference with a position (+0x40) its slot +0x1f4 returns the
    /// address of.
    fn fake_reference(e: &mut Engine, position: [f32; 3], form_flags: u32) -> u32 {
        let reference = fake_object(e, 0x80, &[(SLOT_POSITION, 0x7400_01f4)]);
        for (i, v) in position.iter().enumerate() {
            e.mem.set_f32(reference + 0x40 + i as u32 * 4, *v);
        }
        e.mem.set_u32(reference + 8, form_flags);
        reference
    }

    #[test]
    fn multibound_ref_is_added_to_every_overlapped_cell_but_its_own() {
        let mut e = engine();
        install_cell_functions(&mut e);
        install_float_helpers(&mut e);
        let table = install_maps(&mut e);
        let lists = install_lists(&mut e);
        e.register(0x7400_01f4, |_, a| ret(a[0] + 0x40));
        let w = world(&mut e);
        let reference = fake_reference(&mut e, [5000.0, 5000.0, 0.0], 0);
        e.register(IS_MULTIBOUND_REF, |_, _| ret(1));
        let extent = e.mem.alloc(12);
        e.register_double(REFERENCE_MULTIBOUND_HALF_EXTENT, move |_, _| ret(extent));
        e.register(MULTIBOUND_RADIUS, |_, _| ret_float(4096.0));
        // The reference overlaps the cells with x != 0.
        e.register(MULTIBOUND_INTERSECTS_CELL, |_, a| ret((a[1] != 0) as u32));
        e.register(LIST_MAP_CONSTRUCT, |_, a| ret(a[0]));
        logged(&mut e);
        e.call(0x0058_78d0, &args![w, reference]);
        // Own cell (1, 1) is never asked; the others are (x 0..2, y 0..2).
        let asked: Vec<(i32, i32)> = calls_to(&e, MULTIBOUND_INTERSECTS_CELL)
            .iter()
            .map(|a| (a[1] as i32, a[2] as i32))
            .collect();
        assert_eq!(asked.len(), 8);
        assert!(!asked.contains(&(1, 1)));
        assert_eq!(asked[0], (0, 0));
        assert_eq!(asked[7], (2, 2));
        // The map was made once with 0x25 buckets.
        let made = calls_to(&e, LIST_MAP_CONSTRUCT);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1], MULTIBOUND_MAP_BUCKETS);
        let map = e.get(w, TESWorldSpace::pOverlappedMultiboundMap).addr();
        assert_eq!(map, made[0][0]);
        // One list per overlapped cell: (1,0), (2,0), (2,1), (2,2), (1,2)
        // and (0,..) not: x != 0 only.
        let keys: Vec<u32> = table.borrow().iter().map(|m| m.1).collect();
        assert_eq!(
            keys,
            vec![key(1, 0), key(1, 2), key(2, 0), key(2, 1), key(2, 2)]
        );
        // Each reference was added to its cell's list.
        let adds: Vec<_> = lists
            .borrow()
            .iter()
            .filter(|l| l.0 == "add")
            .cloned()
            .collect();
        assert_eq!(adds.len(), 5);
        assert!(adds.iter().all(|l| l.2 == reference));
    }

    #[test]
    fn multibound_ref_ignores_references_that_are_not_multibound() {
        let mut e = engine();
        install_lists(&mut e);
        let w = world(&mut e);
        e.register(IS_MULTIBOUND_REF, |_, _| ret(0));
        logged(&mut e);
        e.call(0x0058_78d0, &args![w, 0x1000u32]);
        e.call(0x0058_78d0, &args![w, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2 + 1);
        assert!(e.get(w, TESWorldSpace::pOverlappedMultiboundMap).is_null());
    }

    #[test]
    fn multibound_data_is_deleted_with_its_lists() {
        let mut e = engine();
        let table = install_maps(&mut e);
        let lists = install_lists(&mut e);
        let w = world(&mut e);
        // Nothing to delete without a map.
        e.call(0x0058_7bb0, &args![w]);
        assert!(lists.borrow().is_empty());
        let map = fake_object(&mut e, 0x10, &[(0, 0x7500_0000)]);
        e.register(0x7500_0000, |_, _| Ret::default());
        e.set(w, TESWorldSpace::pOverlappedMultiboundMap, Ptr::new(map));
        table
            .borrow_mut()
            .extend([(map, 1, 0xAA), (map, 2, 0), (map, 3, 0xBB)]);
        logged(&mut e);
        e.call(0x0058_7bb0, &args![w]);
        assert_eq!(
            *lists.borrow(),
            vec![
                ("clear", 0xAA, 0),
                ("destroy", 0xAA, 1),
                ("clear", 0xBB, 0),
                ("destroy", 0xBB, 1)
            ]
        );
        assert!(table.borrow().is_empty());
        // The map itself is deleted through its destructor (slot 0, flag 1).
        assert_eq!(calls_to(&e, 0x7500_0000), vec![vec![map, 1]]);
        assert!(e.get(w, TESWorldSpace::pOverlappedMultiboundMap).is_null());
    }

    // ---- fixed references and persistent data -------------------------------------

    #[test]
    fn fixed_ref_depends_on_the_type_of_the_base_form() {
        let mut e = engine();
        e.register(REFERENCE_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        let reference = e.mem.alloc(0x40);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(reference + 0x20, base);
        let fixed = [
            0xd, 0x15, 0x1b, 0x1c, 0x20, 0x21, 0x25, 0x26, 0x27, 0x2c, 0x2d,
        ];
        for form_type in 0..0x80u8 {
            e.mem.set_u8(base + 4, form_type);
            assert_eq!(
                e.call(0x0058_7c80, &args![reference]).bool(),
                fixed.contains(&form_type),
                "{form_type:#x}"
            );
        }
        // No reference, or no base form.
        assert!(!e.call(0x0058_7c80, &args![0u32]).bool());
        e.mem.set_u32(reference + 0x20, 0);
        assert!(!e.call(0x0058_7c80, &args![reference]).bool());
    }

    /// The engine of the persistent reference tests: the lock, the form
    /// flags, base form types and the lists and maps.
    fn persistent_engine() -> (Engine, Ptr<TESWorldSpace>, MapTable, ListLog) {
        let mut e = engine();
        install_cell_functions(&mut e);
        install_float_helpers(&mut e);
        let table = install_maps(&mut e);
        let lists = install_lists(&mut e);
        e.register(LOCK_ENTER, |_, _| Ret::default());
        e.register(LOCK_LEAVE, |_, _| Ret::default());
        e.register(REFERENCE_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(0x7400_01f4, |_, a| ret(a[0] + 0x40));
        let w = world(&mut e);
        (e, w, table, lists)
    }

    /// A persistent reference at `position`; fixed when `fixed`.
    fn persistent_reference(e: &mut Engine, position: [f32; 3], fixed: bool, flags: u32) -> u32 {
        let reference = fake_reference(e, position, flags);
        let base = e.mem.alloc(0x10);
        e.mem.set_u8(base + 4, if fixed { 0x20 } else { 0x30 });
        e.mem.set_u32(reference + 0x20, base);
        reference
    }

    #[test]
    fn a_mobile_persistent_ref_goes_in_the_mobile_list() {
        let (mut e, w, table, lists) = persistent_engine();
        let reference = persistent_reference(&mut e, [0.0; 3], false, 0);
        logged(&mut e);
        e.call(0x0058_7d10, &args![w, reference]);
        assert_eq!(*lists.borrow(), vec![("add", w.addr() + 0x60, reference)]);
        assert!(table.borrow().is_empty());
        // The lock was taken and released around the work.
        let log = e.call_log.as_ref().unwrap();
        let enter = log.iter().position(|c| c.0 == LOCK_ENTER).unwrap();
        let leave = log.iter().position(|c| c.0 == LOCK_LEAVE).unwrap();
        assert!(enter < leave);
        assert_eq!(log[enter].1, vec![PERSISTENT_REF_LOCK, 0]);
        assert_eq!(log[leave].1, vec![PERSISTENT_REF_LOCK]);
    }

    #[test]
    fn a_fixed_persistent_ref_goes_in_the_list_of_its_cell() {
        let (mut e, w, table, lists) = persistent_engine();
        let reference = persistent_reference(&mut e, [4097.0, -1.0, 0.0], true, 0);
        e.call(0x0058_7d10, &args![w, reference]);
        let map = w.addr() + 0x50;
        let cell_key = key(1, -1);
        // A list was made and stored under the key, then the reference
        // added to it.
        let stored = table.borrow().clone();
        assert_eq!(stored.len(), 1);
        assert_eq!((stored[0].0, stored[0].1), (map, cell_key));
        assert_eq!(*lists.borrow(), vec![("add", stored[0].2, reference)]);
        // A second reference in the same cell reuses the list.
        let other = persistent_reference(&mut e, [4100.0, -2.0, 0.0], true, 0);
        e.call(0x0058_7d10, &args![w, other]);
        assert_eq!(table.borrow().len(), 1);
        assert_eq!(lists.borrow()[1], ("add", stored[0].2, other));
    }

    #[test]
    fn persistent_refs_with_form_flag_0x4000_are_left_out() {
        let (mut e, w, table, lists) = persistent_engine();
        let reference = persistent_reference(&mut e, [0.0; 3], true, 0x4000);
        logged(&mut e);
        e.call(0x0058_7d10, &args![w, reference]);
        e.call(0x0058_7e40, &args![w, reference]);
        e.call(0x0058_7ff0, &args![w, reference]);
        e.call(0x0058_7d10, &args![w, 0u32]);
        assert!(table.borrow().is_empty());
        assert!(lists.borrow().is_empty());
        assert!(calls_to(&e, LOCK_ENTER).is_empty());
    }

    #[test]
    fn removing_a_persistent_ref_empties_and_deletes_the_cells_list() {
        let (mut e, w, table, lists) = persistent_engine();
        let reference = persistent_reference(&mut e, [4097.0, -1.0, 0.0], true, 0);
        let map = w.addr() + 0x50;
        // A list holding two references: the map keeps it.
        let busy = make_list(&mut e, &[0x1111, 0x2222]);
        table.borrow_mut().push((map, key(1, -1), busy));
        e.call(0x0058_7e40, &args![w, reference]);
        assert_eq!(*lists.borrow(), vec![("remove", busy, reference)]);
        assert_eq!(table.borrow().len(), 1);
        // An empty list: deleted and the key removed.
        lists.borrow_mut().clear();
        let empty = make_list(&mut e, &[]);
        table.borrow_mut()[0].2 = empty;
        e.call(0x0058_7e40, &args![w, reference]);
        assert_eq!(
            *lists.borrow(),
            vec![("remove", empty, reference), ("destroy", empty, 1)]
        );
        assert!(table.borrow().is_empty());
        // No list under the key: nothing to remove.
        lists.borrow_mut().clear();
        e.call(0x0058_7e40, &args![w, reference]);
        assert!(lists.borrow().is_empty());
        // A mobile one is removed from the mobile list.
        let mobile = persistent_reference(&mut e, [0.0; 3], false, 0);
        e.call(0x0058_7e40, &args![w, mobile]);
        assert_eq!(*lists.borrow(), vec![("remove", w.addr() + 0x60, mobile)]);
    }

    #[test]
    fn clearing_the_persistent_data_empties_the_lists_and_the_map() {
        let (mut e, w, table, lists) = persistent_engine();
        let map = w.addr() + 0x50;
        table
            .borrow_mut()
            .extend([(map, 1, 0xA1), (map, 2, 0), (map, 3, 0xA3)]);
        logged(&mut e);
        e.call(0x0058_7f40, &args![w]);
        assert_eq!(
            *lists.borrow(),
            vec![
                ("clear", w.addr() + 0x60, 0),
                ("clear", 0xA1, 0),
                ("destroy", 0xA1, 1),
                ("clear", 0xA3, 0),
                ("destroy", 0xA3, 1)
            ]
        );
        assert!(table.borrow().is_empty());
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    // ---- the persistent cell --------------------------------------------------------

    #[test]
    fn creating_the_persistent_cell_makes_it_once() {
        let mut e = engine();
        e.register(CELL_CONSTRUCT, |_, a| ret(a[0]));
        noop(&mut e, CELL_SET_PERSISTENT);
        noop(&mut e, CELL_CREATE_DATA);
        let w = world(&mut e);
        logged(&mut e);
        let cell = e.call(0x0058_8070, &args![w]).u32();
        assert_ne!(cell, 0);
        assert_eq!(e.get(w, TESWorldSpace::pPersistentCell).addr(), cell);
        assert_eq!(calls_to(&e, CELL_CONSTRUCT), vec![vec![cell]]);
        assert_eq!(calls_to(&e, CELL_SET_PERSISTENT), vec![vec![cell, 1]]);
        assert_eq!(calls_to(&e, CELL_CREATE_DATA), vec![vec![cell]]);
        // It is a block of the cell's size.
        assert_eq!(e.mem.block_size(cell), Some(CELL_SIZE));
        // A second call returns the same cell without making another.
        assert_eq!(e.call(0x0058_8070, &args![w]).u32(), cell);
        assert_eq!(calls_to(&e, CELL_CONSTRUCT).len(), 1);
    }

    #[test]
    fn adding_a_persistent_ref_creates_the_cell_and_adds_to_it() {
        let mut e = engine();
        install_cell_functions(&mut e);
        e.register(CELL_CONSTRUCT, |_, a| ret(a[0]));
        noop(&mut e, CELL_SET_PERSISTENT);
        noop(&mut e, CELL_CREATE_DATA);
        noop(&mut e, CELL_ADD_REFERENCE);
        let w = world(&mut e);
        let reference = fake_reference(&mut e, [0.0; 3], 0);
        logged(&mut e);
        e.call(0x0058_7ff0, &args![w, reference]);
        let cell = e.get(w, TESWorldSpace::pPersistentCell).addr();
        assert_ne!(cell, 0);
        assert_eq!(
            calls_to(&e, CELL_ADD_REFERENCE),
            vec![vec![cell, reference, 0]]
        );
        // No reference: nothing.
        logged(&mut e);
        e.call(0x0058_7ff0, &args![w, 0u32]);
        assert!(calls_to(&e, CELL_ADD_REFERENCE).is_empty());
    }

    #[test]
    fn removing_a_persistent_ref_asks_the_persistent_cell() {
        let mut e = engine();
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        noop(&mut e, CELL_REMOVE_REFERENCE);
        let w = world(&mut e);
        logged(&mut e);
        // No cell: nothing.
        e.call(0x0058_8030, &args![w, 0x7777u32]);
        assert!(calls_to(&e, CELL_REMOVE_REFERENCE).is_empty());
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(0xCE11));
        e.call(0x0058_8030, &args![w, 0x7777u32]);
        assert_eq!(
            calls_to(&e, CELL_REMOVE_REFERENCE),
            vec![vec![0xCE11, 0x7777]]
        );
        // No reference: nothing more.
        e.call(0x0058_8030, &args![w, 0u32]);
        assert_eq!(calls_to(&e, CELL_REMOVE_REFERENCE).len(), 1);
    }

    #[test]
    fn the_persistent_cell_is_asked_for_the_world_space_work() {
        let mut e = engine();
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        noop(&mut e, CELL_ASSIGN_PERSISTENT_REFS_IN_WORLD);
        noop(&mut e, CELL_PERSISTENT_ACTION);
        let w = world(&mut e);
        logged(&mut e);
        e.call(0x0058_8120, &args![w]);
        e.call(0x0058_8270, &args![w, 5u32]);
        assert!(calls_to(&e, CELL_ASSIGN_PERSISTENT_REFS_IN_WORLD).is_empty());
        assert!(calls_to(&e, CELL_PERSISTENT_ACTION).is_empty());
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(0xCE11));
        e.call(0x0058_8120, &args![w]);
        e.call(0x0058_8270, &args![w, 5u32]);
        assert_eq!(
            calls_to(&e, CELL_ASSIGN_PERSISTENT_REFS_IN_WORLD),
            vec![vec![0xCE11, w.addr()]]
        );
        assert_eq!(calls_to(&e, CELL_PERSISTENT_ACTION), vec![vec![0xCE11, 5]]);
    }

    #[test]
    fn assigning_persistent_refs_to_a_cell_uses_the_fixed_list_and_the_mobile_ones_inside() {
        let (mut e, w, table, _) = persistent_engine();
        noop(&mut e, CELL_ADD_REFERENCE);
        let cell = fake_cell(&mut e, 3, 4, 0, 0);
        // The fixed list of the cell: two references.
        let fixed = make_list(&mut e, &[0xF1, 0xF2]);
        table.borrow_mut().push((w.addr() + 0x50, key(3, 4), fixed));
        // Mobile ones: in the cell, outside, and already in the cell.
        let inside = fake_reference(&mut e, [1.0, 2.0, 0.0], 0);
        let outside = fake_reference(&mut e, [9.0, 9.0, 0.0], 0);
        let owned = fake_reference(&mut e, [1.0, 2.0, 0.0], 0);
        for (r, parent) in [(inside, 0), (outside, 0), (owned, cell)] {
            e.mem.set_u32(r + 0x40 + 0x20, parent);
        }
        e.register(REFERENCE_WORD_40, |e, a| ret(e.mem.u32(a[0] + 0x60)));
        e.register(CELL_CONTAINS_POINT, |e, a| {
            ret((e.mem.f32(a[1]) < 5.0) as u32)
        });
        let mobile = make_list(&mut e, &[inside, outside, owned]);
        // The mobile list head lives in the world space.
        let head = w.addr() + 0x60;
        e.mem.set_u32(head, e.mem.u32(mobile));
        e.mem.set_u32(head + 4, e.mem.u32(mobile + 4));
        logged(&mut e);
        e.call(0x0058_8150, &args![w, cell]);
        let added: Vec<u32> = calls_to(&e, CELL_ADD_REFERENCE)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(added, vec![0xF1, 0xF2, inside]);
        assert!(calls_to(&e, CELL_ADD_REFERENCE)
            .iter()
            .all(|a| a[0] == cell && a[2] == 0));
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
        // No cell: nothing, not even the lock.
        logged(&mut e);
        e.call(0x0058_8150, &args![w, 0u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 1);
    }

    // ---- lists of world spaces --------------------------------------------------------

    #[test]
    fn the_persistent_cell_fills_the_lists() {
        let mut e = engine();
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        noop(&mut e, CELL_FILL_LIST_FIRST);
        noop(&mut e, CELL_FILL_LIST_SECOND);
        let w = world(&mut e);
        logged(&mut e);
        // No persistent cell: nothing is asked of it.
        e.call(0x0058_84e0, &args![w, 0x1111u32]);
        e.call(0x0058_8520, &args![w, 0x1111u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(0xCE11));
        // No list: nothing either.
        e.call(0x0058_84e0, &args![w, 0u32]);
        e.call(0x0058_8520, &args![w, 0u32]);
        assert!(calls_to(&e, CELL_FILL_LIST_FIRST).is_empty());
        assert!(calls_to(&e, CELL_FILL_LIST_SECOND).is_empty());
        e.call(0x0058_84e0, &args![w, 0x1111u32]);
        e.call(0x0058_8520, &args![w, 0x2222u32]);
        assert_eq!(
            calls_to(&e, CELL_FILL_LIST_FIRST),
            vec![vec![0xCE11, 0x1111]]
        );
        assert_eq!(
            calls_to(&e, CELL_FILL_LIST_SECOND),
            vec![vec![0xCE11, 0x2222]]
        );
    }

    /// The engine of the map marker list tests: the data handler's list of
    /// world spaces starts at `+0x10`, every world space is its own
    /// persistent cell and the fillers record (cell, list).
    fn marker_engine() -> (Engine, FillLog, FillLog) {
        let mut e = engine();
        install_lists(&mut e);
        let handler = e.mem.alloc(0x700);
        set_word(&mut e, DATA_HANDLER_POINTER, handler);
        e.register(DATA_HANDLER_WORLD_LIST, |_, a| ret(a[0] + 0x10));
        e.register(GET_PERSISTENT_CELL, |_, a| ret(a[0]));
        let first = Rc::new(RefCell::new(Vec::new()));
        let f = first.clone();
        e.register_double(CELL_FILL_LIST_FIRST, move |_, a| {
            f.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let second = Rc::new(RefCell::new(Vec::new()));
        let s = second.clone();
        e.register_double(CELL_FILL_LIST_SECOND, move |_, a| {
            s.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        (e, first, second)
    }

    /// Makes the data handler's list of world spaces `items`.
    fn set_world_list(e: &mut Engine, items: &[u32]) {
        let handler = e.global::<u32>(DATA_HANDLER_POINTER);
        let head = make_list(e, items);
        e.mem.set_u32(handler + 0x10, e.mem.u32(head));
        e.mem.set_u32(handler + 0x14, e.mem.u32(head + 4));
    }

    /// Four world spaces: `root`, `child` (uses the root's value 2),
    /// `unrelated` and `grandchild` (has the root as parent but uses none of
    /// its values).
    fn world_family(e: &mut Engine) -> [Ptr<TESWorldSpace>; 4] {
        let root = world(e);
        let child = world(e);
        let unrelated = world(e);
        let grandchild = world(e);
        e.set(child, TESWorldSpace::pParentWorld, root.cast());
        e.set(child, TESWorldSpace::sParentUseFlags, 0b100);
        e.set(grandchild, TESWorldSpace::pParentWorld, root.cast());
        [root, child, unrelated, grandchild]
    }

    #[test]
    fn the_map_marker_list_gathers_the_world_space_and_those_that_use_its_values() {
        let (mut e, filled, _) = marker_engine();
        let [root, child, unrelated, grandchild] = world_family(&mut e);
        set_world_list(
            &mut e,
            &[
                root.addr(),
                child.addr(),
                unrelated.addr(),
                0,
                grandchild.addr(),
            ],
        );
        let list = e.call(0x0058_82a0, &args![root, 0u32]).u32();
        assert_ne!(list, 0);
        // The root and the world space that uses its value, in list order.
        assert_eq!(
            *filled.borrow(),
            vec![(root.addr(), list), (child.addr(), list)]
        );
        // With `all` set, only the world space itself.
        filled.borrow_mut().clear();
        let list = e.call(0x0058_82a0, &args![root, 1u32]).u32();
        assert_eq!(*filled.borrow(), vec![(root.addr(), list)]);
        // A world space that uses its parent's value builds nothing itself
        // (`all` 0): the parent builds the list.
        filled.borrow_mut().clear();
        let list = e.call(0x0058_82a0, &args![child, 0u32]).u32();
        assert_eq!(
            *filled.borrow(),
            vec![(root.addr(), list), (child.addr(), list)]
        );
        // With `all` set it builds its own.
        filled.borrow_mut().clear();
        let list = e.call(0x0058_82a0, &args![child, 1u32]).u32();
        assert_eq!(*filled.borrow(), vec![(child.addr(), list)]);
        // A world space that does not inherit value 2 builds its own list
        // and the ones of those that inherit from it.
        filled.borrow_mut().clear();
        let list = e.call(0x0058_82a0, &args![grandchild, 0u32]).u32();
        assert_eq!(*filled.borrow(), vec![(grandchild.addr(), list)]);
    }

    #[test]
    fn the_second_marker_list_builder_uses_the_other_filler() {
        let (mut e, first, second) = marker_engine();
        let [root, child, _, _] = world_family(&mut e);
        set_world_list(&mut e, &[root.addr(), child.addr()]);
        let list = e.call(0x0058_83c0, &args![root, 0u32]).u32();
        assert_eq!(
            *second.borrow(),
            vec![(root.addr(), list), (child.addr(), list)]
        );
        assert!(first.borrow().is_empty());
        // `all` set: a single fill.
        second.borrow_mut().clear();
        e.call(0x0058_83c0, &args![root, 7u32]);
        assert_eq!(second.borrow().len(), 1);
        // Through the parent when the value is inherited.
        second.borrow_mut().clear();
        let list = e.call(0x0058_83c0, &args![child, 0u32]).u32();
        assert_eq!(
            *second.borrow(),
            vec![(root.addr(), list), (child.addr(), list)]
        );
    }

    #[test]
    fn a_cell_is_found_by_editor_id() {
        let mut e = engine();
        install_cell_functions(&mut e);
        let (w, table) = world_with_cells(&mut e, &[]);
        let map = e.get(w, TESWorldSpace::pCellMap).addr();
        let a = fake_cell(&mut e, 0, 0, 0, 0);
        let b = fake_cell(&mut e, 1, 0, 0, 0);
        let c = fake_cell(&mut e, 2, 0, 0, 0);
        table
            .borrow_mut()
            .extend([(map, 1, a), (map, 2, 0), (map, 3, b), (map, 4, c)]);
        // The comparison answers 0 (equal) for the cells b and c.
        e.register_double(EDITOR_ID_COMPARE, move |_, args| {
            ret(if args[0] == b + 0x90 || args[0] == c + 0x90 {
                0
            } else {
                1
            })
        });
        // The first match wins.
        assert_eq!(e.call(0x0058_8560, &args![w, 0x6000u32]).u32(), b);
        // No name: nothing.
        assert_eq!(e.call(0x0058_8560, &args![w, 0u32]).u32(), 0);
        // No match.
        e.register(EDITOR_ID_COMPARE, |_, _| ret(1));
        assert_eq!(e.call(0x0058_8560, &args![w, 0x6000u32]).u32(), 0);
    }

    // ---- references in range (005885f0) ----------------------------------------------

    /// The engine of the ring walk tests: the world space has cells at the
    /// coordinates `cells` (returned in the same order), a cell's distance to
    /// a point is 100 times its x coordinate (stored in the cell), and the
    /// visits are recorded.
    fn ring_engine(cells: &[(i16, i16)]) -> (Engine, Ptr<TESWorldSpace>, VisitLog, Vec<u32>) {
        let mut e = engine();
        install_float_helpers(&mut e);
        noop(&mut e, LOG);
        let entries: Vec<(i16, i16, u32)> = cells
            .iter()
            .map(|(x, y)| (*x, *y, e.mem.alloc(0x20)))
            .collect();
        for (x, _, cell) in &entries {
            e.mem.set_i32(*cell, *x as i32);
        }
        let addresses = entries.iter().map(|c| c.2).collect();
        let (w, _) = world_with_cells(&mut e, &entries);
        set_double(&mut e, DOUBLE_FLOAT_MAX, f32::MAX as f64);
        e.register(CELL_DISTANCE_TO_POINT, |e, a| {
            ret_float(e.mem.i32(a[0]) as f32 * 100.0)
        });
        let visits = Rc::new(RefCell::new(Vec::new()));
        let v = visits.clone();
        e.register_double(CELL_FOR_REFERENCES_IN_RANGE, move |_, a| {
            v.borrow_mut().push(a[0]);
            ret(1)
        });
        e.register(GET_PERSISTENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        (e, w, visits, addresses)
    }

    fn ring_point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, x);
        e.mem.set_f32(point + 4, y);
        point
    }

    #[test]
    fn the_ring_walk_does_nothing_without_a_visitor() {
        let (mut e, w, visits, _) = ring_engine(&[(0, 0)]);
        let point = ring_point(&mut e, 10.0, 10.0);
        let f = f32::MAX;
        logged(&mut e);
        e.call(0x0058_85f0, &args![w, point, f, point, f, 0u32, 0u32]);
        assert!(visits.borrow().is_empty());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn the_ring_walk_visits_the_cell_then_the_rings_then_the_persistent_cell() {
        let cells = [(0, 0), (1, 0), (-1, 1), (0, -1), (3, 3)];
        let (mut e, w, visits, c) = ring_engine(&cells);
        let persistent = e.mem.alloc(0x20);
        e.set(w, TESWorldSpace::pPersistentCell, Ptr::new(persistent));
        let point = ring_point(&mut e, 100.0, 100.0);
        let f = f32::MAX;
        logged(&mut e);
        e.call(
            0x0058_85f0,
            &args![w, point, f, point, f, 0x4000u32, 0x77u32],
        );
        // The centre, then the cells of the first ring in the order the ring
        // is walked (bottom row left to right, right column up, top row
        // right to left, left column down): (0, -1), (1, 0), (-1, 1). The
        // second ring has no cell in it, so the walk ends with the
        // persistent cell. The cell at (3, 3) is never reached.
        assert_eq!(*visits.borrow(), vec![c[0], c[3], c[1], c[2], persistent]);
        // The visits get the arguments through unchanged.
        let first = calls_to(&e, CELL_FOR_REFERENCES_IN_RANGE)[0].clone();
        assert_eq!(
            first[1..],
            [point, f.to_bits(), point, f.to_bits(), 0x4000, 0x77]
        );
    }

    #[test]
    fn the_ring_walk_without_a_centre_cell_starts_with_the_ring() {
        let (mut e, w, visits, c) = ring_engine(&[(1, 1)]);
        let point = ring_point(&mut e, 100.0, 100.0);
        let f = f32::MAX;
        e.call(0x0058_85f0, &args![w, point, f, point, f, 1u32, 0u32]);
        assert_eq!(*visits.borrow(), vec![c[0]]);
    }

    #[test]
    fn the_ring_walk_stops_when_a_visit_returns_false() {
        let (mut e, w, _, _) = ring_engine(&[(0, 0), (1, 0)]);
        let count = Rc::new(RefCell::new(0u32));
        let c = count.clone();
        e.register_double(CELL_FOR_REFERENCES_IN_RANGE, move |_, _| {
            *c.borrow_mut() += 1;
            ret(0)
        });
        let point = ring_point(&mut e, 10.0, 10.0);
        let f = f32::MAX;
        e.call(0x0058_85f0, &args![w, point, f, point, f, 1u32, 0u32]);
        // The centre cell refused: nothing else.
        assert_eq!(*count.borrow(), 1);
        // A refusal in a ring ends the walk there.
        let (mut e, w, _, _) = ring_engine(&[(0, 0), (1, 0), (-1, 1)]);
        let count = Rc::new(RefCell::new(0u32));
        let c = count.clone();
        e.register_double(CELL_FOR_REFERENCES_IN_RANGE, move |_, _| {
            *c.borrow_mut() += 1;
            ret((*c.borrow() < 2) as u32)
        });
        let point = ring_point(&mut e, 10.0, 10.0);
        e.call(0x0058_85f0, &args![w, point, f, point, f, 1u32, 0u32]);
        assert_eq!(*count.borrow(), 2);
    }

    #[test]
    fn the_ring_walk_skips_cells_outside_the_radii() {
        let (mut e, w, visits, c) = ring_engine(&[(0, 0), (1, 0), (-1, 0), (0, 1)]);
        let point = ring_point(&mut e, 10.0, 10.0);
        let f = f32::MAX;
        // The distance of a cell is 100 times its column: a radius of 50
        // keeps the columns 0 and -1 (distances 0 and -100), not column 1.
        e.call(0x0058_85f0, &args![w, point, 50.0f32, point, f, 1u32, 0u32]);
        assert!(visits.borrow().contains(&c[2]));
        assert!(visits.borrow().contains(&c[3]));
        assert!(!visits.borrow().contains(&c[1]));
        // The second radius limits the same way.
        visits.borrow_mut().clear();
        e.call(0x0058_85f0, &args![w, point, f, point, 50.0f32, 1u32, 0u32]);
        assert!(visits.borrow().contains(&c[2]));
        assert!(!visits.borrow().contains(&c[1]));
        // A NaN radius keeps nothing of the rings (the centre is visited).
        visits.borrow_mut().clear();
        e.call(
            0x0058_85f0,
            &args![w, point, f32::NAN, point, f, 1u32, 0u32],
        );
        assert_eq!(*visits.borrow(), vec![c[0]]);
    }

    // ---- record types and offset data ---------------------------------------------------

    #[test]
    fn record_types_that_can_be_in_a_world_space() {
        let mut e = engine();
        e.register(RECORD_TYPE_CHECK, |_, a| ret((a[0] == b'X' as u32) as u32));
        let w = world(&mut e);
        for (record_type, expected) in [
            (b'9', true),
            (b'D', true),
            (b'X', true),
            (b'A', false),
            (0, false),
        ] {
            assert_eq!(
                e.call(0x0058_8a60, &args![record_type as u32]).bool(),
                expected
            );
            assert_eq!(
                e.call(0x0058_8a40, &args![w, record_type as u32]).bool(),
                expected
            );
        }
        // The first two do not ask the check.
        logged(&mut e);
        e.call(0x0058_8a60, &args![b'9' as u32]);
        e.call(0x0058_8a60, &args![b'D' as u32]);
        assert_eq!(calls_to(&e, RECORD_TYPE_CHECK).len(), 0);
    }

    /// Files chained through their master (+0x10); the master getter reads it.
    fn file_chain(e: &mut Engine, length: usize) -> Vec<u32> {
        e.register(FILE_MASTER, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let files: Vec<u32> = (0..length).map(|_| e.mem.alloc(0x20)).collect();
        for pair in files.windows(2) {
            e.mem.set_u32(pair[0] + 0x10, pair[1]);
        }
        files
    }

    #[test]
    fn offset_data_is_looked_up_under_the_first_file_of_the_chain() {
        let mut e = engine();
        let table = install_maps(&mut e);
        let files = file_chain(&mut e, 3);
        let w = world(&mut e);
        // None yet.
        assert_eq!(e.call(0x0058_8a90, &args![w, files[0]]).u32(), 0);
        // The offset data is stored under the last file of the chain.
        let map = w.addr() + 0xB0;
        table.borrow_mut().push((map, files[2], 0xDA7A));
        assert_eq!(e.call(0x0058_8a90, &args![w, files[0]]).u32(), 0xDA7A);
        assert_eq!(e.call(0x0058_8a90, &args![w, files[2]]).u32(), 0xDA7A);
        assert_eq!(e.call(0x0058_8a90, &args![w, files[1]]).u32(), 0xDA7A);
        // No file: nothing found.
        assert_eq!(e.call(0x0058_8a90, &args![w, 0u32]).u32(), 0);
    }

    #[test]
    fn creating_offset_data_makes_it_once_with_the_widest_bounds() {
        let mut e = engine();
        let table = install_maps(&mut e);
        let files = file_chain(&mut e, 2);
        set_word(&mut e, FLOAT_MAX_VALUE, f32::MAX.to_bits());
        set_word(&mut e, FLOAT_MIN_VALUE, (-f32::MAX).to_bits());
        e.register(OFFSET_DATA_CONSTRUCT, |_, a| ret(a[0]));
        // NiPoint2(x, y): stores both and returns itself.
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        let w = world(&mut e);
        let data = e.call(0x0058_8b00, &args![w, files[0]]).ptr::<OffsetData>();
        assert_ne!(data.addr(), 0);
        assert_eq!(e.mem.block_size(data.addr()), Some(OFFSET_DATA_SIZE));
        assert!(e.get(data, OffsetData::pCellFileOffsets).is_null());
        assert_eq!(e.get(data, OffsetData::iFileOffset), 0);
        assert_eq!(e.get(data, OffsetData::OffsetMinCoords_x), f32::MAX);
        assert_eq!(e.get(data, OffsetData::OffsetMinCoords_y), f32::MAX);
        assert_eq!(e.get(data, OffsetData::OffsetMaxCoords_x), -f32::MAX);
        assert_eq!(e.get(data, OffsetData::OffsetMaxCoords_y), -f32::MAX);
        // Stored under the last file of the chain.
        assert_eq!(
            *table.borrow(),
            vec![(w.addr() + 0xB0, files[1], data.addr())]
        );
        // Asking again, from either file, finds it.
        assert_eq!(e.call(0x0058_8b00, &args![w, files[1]]).u32(), data.addr());
        assert_eq!(e.call(0x0058_8b00, &args![w, files[0]]).u32(), data.addr());
        assert_eq!(table.borrow().len(), 1);
    }

    // ---- Third session: 00588c50 to 00589420 --------------------------------

    #[test]
    fn offset_data_constructor_runs_the_point_constructors_and_returns_this() {
        let mut e = engine();
        noop(&mut e, LOCAL_STRUCT_CONSTRUCT);
        logged(&mut e);
        let data = e.mem.alloc(OFFSET_DATA_SIZE);
        assert_eq!(e.call(0x0058_8c50, &args![data]).u32(), data);
        assert_eq!(
            calls_to(&e, LOCAL_STRUCT_CONSTRUCT),
            [[data + 4], [data + 0xC]]
        );
    }

    #[test]
    fn clearing_the_offset_data_map_frees_every_table_and_record() {
        let mut e = engine();
        let table = install_maps(&mut e);
        noop(&mut e, MEMORY_FREE);
        let w = world(&mut e);
        let map = w.addr() + 0xB0;
        let mut blocks = vec![];
        for key in [0x10u32, 0x20] {
            let data = e.mem.alloc(OFFSET_DATA_SIZE);
            let cells = e.mem.alloc(16);
            e.mem.set_u32(data, cells);
            table.borrow_mut().push((map, key, data));
            blocks.push((cells, data));
        }
        logged(&mut e);
        e.call(0x0058_8c80, &args![w]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_FREE).iter().map(|a| a[0]).collect();
        assert_eq!(freed, [blocks[0].0, blocks[0].1, blocks[1].0, blocks[1].1]);
        // RemoveAll ran last and emptied the map.
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), [[map]]);
        assert!(table.borrow().is_empty());
    }

    #[test]
    fn clearing_an_empty_offset_data_map_only_removes_all() {
        let mut e = engine();
        install_maps(&mut e);
        noop(&mut e, MEMORY_FREE);
        let w = world(&mut e);
        logged(&mut e);
        e.call(0x0058_8c80, &args![w]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 1);
    }

    /// The border region test: a data handler list of region entries
    /// (flags at `+8`, the world space at `+0x20`, a list of regions at
    /// `+0x1c`; a region "contains" the point when its first word is 1).
    struct BorderCase {
        e: Engine,
        w: Ptr<TESWorldSpace>,
        point: u32,
        tail: u32,
    }

    fn border_case() -> BorderCase {
        let mut e = engine();
        install_lists(&mut e);
        let handler = e.mem.alloc(0x700);
        set_word(&mut e, DATA_HANDLER_POINTER, handler);
        e.register(DATA_HANDLER_LIST_1D8, |e, a| ret(e.mem.u32(a[0] + 0x1d8)));
        e.register(REGION_ENTRY_FLAG_40, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x40 != 0) as u32)
        });
        e.register(REGION_ENTRY_FLAG_20, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x20 != 0) as u32)
        });
        e.register(REFERENCE_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(REGION_ENTRY_LIST, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(REGION_POINT_IN_ENTRY, |e, a| {
            ret((e.mem.u32(a[0]) == 1) as u32)
        });
        e.register(REGION_POINT_BUILD_XY, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        let w = world(&mut e);
        e.set(w, TESWorldSpace::cFlags, 8);
        let point = e.mem.alloc(8);
        e.mem.set_f32(point, 3.0);
        e.mem.set_f32(point + 4, 4.0);
        // The list object: the node is at +4 (item) and +8 (next).
        let tail = e.mem.alloc(0x10);
        e.mem.set_u32(handler + 0x1d8, tail);
        BorderCase { e, w, point, tail }
    }

    /// A region entry for `world` with one region whose first word is
    /// `region_word`.
    fn border_entry(e: &mut Engine, flags: u32, world: u32, region_word: u32) -> u32 {
        let entry = e.mem.alloc(0x40);
        e.mem.set_u32(entry + 8, flags);
        e.mem.set_u32(entry + 0x20, world);
        let region = e.mem.alloc(8);
        e.mem.set_u32(region, region_word);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, region);
        e.mem.set_u32(entry + 0x1c, list);
        entry
    }

    #[test]
    fn a_world_space_without_a_border_region_covers_every_point() {
        let mut c = border_case();
        c.e.set(c.w, TESWorldSpace::cFlags, 0);
        c.e.call_log = Some(vec![]);
        assert!(c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
        assert!(calls_to(&c.e, REGION_POINT_BUILD_XY).is_empty());
    }

    #[test]
    fn a_border_region_covers_the_point_when_an_entry_region_contains_it() {
        let mut c = border_case();
        let entry = border_entry(&mut c.e, 0x40, c.w.addr(), 1);
        c.e.mem.set_u32(c.tail + 4, entry);
        c.e.call_log = Some(vec![]);
        assert!(c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
        // The probe holds the point's x and y.
        let probe = calls_to(&c.e, REGION_POINT_BUILD_XY)[0][0];
        assert_eq!(c.e.mem.f32(probe), 3.0);
        assert_eq!(c.e.mem.f32(probe + 4), 4.0);
    }

    #[test]
    fn a_border_region_misses_the_point_when_no_entry_qualifies() {
        // The region does not contain the point.
        let mut c = border_case();
        let entry = border_entry(&mut c.e, 0x40, c.w.addr(), 0);
        c.e.mem.set_u32(c.tail + 4, entry);
        assert!(!c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
        // The entry belongs to another world space.
        let mut c = border_case();
        let entry = border_entry(&mut c.e, 0x40, 0x1234, 1);
        c.e.mem.set_u32(c.tail + 4, entry);
        assert!(!c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
        // The entry lacks flag 0x40, or has flag 0x20.
        for flags in [0u32, 0x60] {
            let mut c = border_case();
            let entry = border_entry(&mut c.e, flags, c.w.addr(), 1);
            c.e.mem.set_u32(c.tail + 4, entry);
            assert!(!c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
        }
        // No entries at all.
        let mut c = border_case();
        assert!(!c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
    }

    #[test]
    fn a_border_region_looks_past_entries_that_do_not_qualify() {
        let mut c = border_case();
        let other = border_entry(&mut c.e, 0, c.w.addr(), 1);
        let good = border_entry(&mut c.e, 0x40, c.w.addr(), 1);
        let second = c.e.mem.alloc(8);
        c.e.mem.set_u32(second, good);
        c.e.mem.set_u32(c.tail + 4, other);
        c.e.mem.set_u32(c.tail + 8, second);
        assert!(c.e.call(0x0058_8d10, &args![c.w, c.point]).bool());
    }

    /// Doubles for the map marker conversion: the world map data getters
    /// (`+0x90`, `+0x94`, `+0x98`), the float 0.5 and the `NiPoint3`
    /// operations on three floats.
    fn coord_engine(scale: f32, offset_x: f32, offset_y: f32) -> (Engine, Ptr<TESWorldSpace>) {
        let mut e = engine();
        set_double(&mut e, DOUBLE_ONE, 1.0);
        set_double(&mut e, DOUBLE_ZERO, 0.0);
        set_word(&mut e, FLOAT_HALF, 0.5f32.to_bits());
        noop(&mut e, LOCAL_STRUCT_CONSTRUCT);
        e.register(MAP_SCALE, |e, a| ret_float(e.mem.f32(a[0] + 0x90)));
        e.register(MAP_OFFSET_X, |e, a| ret_float(e.mem.f32(a[0] + 0x94)));
        e.register(MAP_OFFSET_Y, |e, a| ret_float(e.mem.f32(a[0] + 0x98)));
        e.register(POINT3_SCALE, |e, a| {
            let factor = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + i * 4);
                e.mem.set_f32(a[0] + i * 4, v * factor);
            }
            ret(a[0])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + i * 4) - e.mem.f32(a[2] + i * 4);
                e.mem.set_f32(a[1] + i * 4, v);
            }
            ret(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + i * 4) + e.mem.f32(a[2] + i * 4);
                e.mem.set_f32(a[1] + i * 4, v);
            }
            ret(a[1])
        });
        let w = world(&mut e);
        e.mem.set_f32(w.addr() + 0x90, scale);
        e.mem.set_f32(w.addr() + 0x94, offset_x);
        e.mem.set_f32(w.addr() + 0x98, offset_y);
        e.set(w, TESWorldSpace::MinimumCoords_x, -100.0);
        e.set(w, TESWorldSpace::MinimumCoords_y, -200.0);
        e.set(w, TESWorldSpace::MaximumCoords_x, 300.0);
        e.set(w, TESWorldSpace::MaximumCoords_y, 400.0);
        (e, w)
    }

    fn adjust_coords(e: &mut Engine, w: Ptr<TESWorldSpace>, to_map: u8) -> [f32; 3] {
        let coords = e.mem.alloc(12);
        for (i, v) in [10.0f32, 20.0, 5.0].iter().enumerate() {
            e.mem.set_f32(coords + i as u32 * 4, *v);
        }
        e.call(0x0058_8e40, &args![w, coords, to_map as u32]);
        [
            e.mem.f32(coords),
            e.mem.f32(coords + 4),
            e.mem.f32(coords + 8),
        ]
    }

    #[test]
    fn map_scale_one_or_zero_only_shifts_by_the_offsets() {
        for scale in [1.0f32, 0.0] {
            let (mut e, w) = coord_engine(scale, 1.0, 2.0);
            assert_eq!(adjust_coords(&mut e, w, 0), [11.0, 22.0, 5.0]);
            let (mut e, w) = coord_engine(scale, 1.0, 2.0);
            assert_eq!(adjust_coords(&mut e, w, 1), [9.0, 18.0, 5.0]);
        }
    }

    #[test]
    fn map_marker_to_world_scales_about_the_middle_then_adds_the_offsets() {
        // The middle is (100, 100); the point is 90 left and 80 below it,
        // doubled, and moved back: (-80, -60), then the offsets.
        let (mut e, w) = coord_engine(2.0, 1.0, 2.0);
        assert_eq!(adjust_coords(&mut e, w, 0), [-79.0, -58.0, 10.0]);
    }

    #[test]
    fn map_marker_to_map_subtracts_the_offsets_then_scales_by_the_inverse() {
        let (mut e, w) = coord_engine(2.0, 1.0, 2.0);
        assert_eq!(adjust_coords(&mut e, w, 1), [54.5, 59.0, 2.5]);
    }

    /// What the doubles of the map classes saw, in order: `RemoveAll`
    /// records the vtable the map has at that moment.
    type MapEvents = Rc<RefCell<Vec<(&'static str, u32, u32)>>>;

    fn map_class_engine() -> (Engine, MapEvents) {
        let mut e = engine();
        let events: MapEvents = Rc::default();
        let log = events.clone();
        e.register_double(NI_ALLOC, move |e, a| {
            log.borrow_mut().push(("alloc", a[0], 0));
            ret(e.mem.alloc(a[0]))
        });
        let log = events.clone();
        e.register_double(MEMSET, move |_, a| {
            log.borrow_mut().push(("memset", a[1], a[2]));
            ret(a[0])
        });
        let log = events.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            log.borrow_mut().push(("remove_all", e.mem.u32(a[0]), 0));
            Ret::default()
        });
        let log = events.clone();
        e.register_double(NI_FREE, move |_, a| {
            log.borrow_mut().push(("ni_free", a[0], 0));
            Ret::default()
        });
        let log = events.clone();
        e.register_double(MEMORY_FREE, move |_, a| {
            log.borrow_mut().push(("free", a[0], 0));
            Ret::default()
        });
        (e, events)
    }

    /// Per map class: the derived constructor and vtable, the base
    /// constructor and vtable, the derived and base destructors.
    const MAP_CLASSES: [(u32, u32, u32, u32, u32, u32); 3] = [
        (
            0x0058_8fa0,
            LIST_MAP_VTABLE,
            0x0058_90c0,
            LIST_MAP_BASE_VTABLE,
            0x0058_9130,
            0x0058_9190,
        ),
        (
            0x0058_8fd0,
            OFFSET_MAP_VTABLE,
            0x0058_91c0,
            OFFSET_MAP_BASE_VTABLE,
            0x0058_9230,
            0x0058_9290,
        ),
        (
            0x0058_9000,
            CELL_MAP_VTABLE,
            0x0058_92c0,
            CELL_MAP_BASE_VTABLE,
            0x0058_9330,
            0x0058_9390,
        ),
    ];

    #[test]
    fn map_constructors_zero_the_buckets_and_set_the_vtable() {
        for (derived, derived_vtable, base, base_vtable, _, _) in MAP_CLASSES {
            for (constructor, vtable) in [(base, base_vtable), (derived, derived_vtable)] {
                let (mut e, events) = map_class_engine();
                let map = e.new_object::<NiTPointerMap>();
                let got = e
                    .call(constructor, &args![map, 0x25u32])
                    .ptr::<NiTPointerMap>();
                assert_eq!(got, map);
                assert_eq!(e.mem.u32(map.addr()), vtable);
                assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 0x25);
                assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
                let table = e.get(map, NiTPointerMap::m_ppkHashTable);
                assert!(e.mem.block_size(table).unwrap() >= 0x94);
                assert_eq!(*events.borrow(), [("alloc", 0x94, 0), ("memset", 0, 0x94)]);
            }
        }
    }

    #[test]
    fn map_destructors_empty_the_map_and_free_the_buckets() {
        for (derived, derived_vtable, base, base_vtable, derived_destructor, base_destructor) in
            MAP_CLASSES
        {
            // The derived destructor: RemoveAll under the derived vtable,
            // then the base destructor's own.
            let (mut e, events) = map_class_engine();
            let map = e.new_object::<NiTPointerMap>();
            e.call(derived, &args![map, 4u32]);
            let table = e.get(map, NiTPointerMap::m_ppkHashTable);
            events.borrow_mut().clear();
            e.call(derived_destructor, &args![map]);
            assert_eq!(
                *events.borrow(),
                [
                    ("remove_all", derived_vtable, 0),
                    ("remove_all", base_vtable, 0),
                    ("ni_free", table, 0)
                ]
            );
            assert_eq!(e.mem.u32(map.addr()), base_vtable);
            // The base destructor alone.
            let (mut e, events) = map_class_engine();
            let map = e.new_object::<NiTPointerMap>();
            e.call(base, &args![map, 4u32]);
            let table = e.get(map, NiTPointerMap::m_ppkHashTable);
            events.borrow_mut().clear();
            e.call(base_destructor, &args![map]);
            assert_eq!(
                *events.borrow(),
                [("remove_all", base_vtable, 0), ("ni_free", table, 0)]
            );
        }
    }

    #[test]
    fn scalar_deleting_destructors_free_the_object_only_when_asked() {
        // (derived scalar deleting destructor, base scalar deleting
        // destructor) per class.
        let pairs = [
            (0x0058_9030u32, 0x0058_93c0u32),
            (0x0058_9060, 0x0058_93f0),
            (0x0058_9090, 0x0058_9420),
        ];
        for (derived, base) in pairs {
            for destructor in [derived, base] {
                let (mut e, events) = map_class_engine();
                let map = e.new_object::<NiTPointerMap>();
                e.set(map, NiTPointerMap::m_ppkHashTable, 0x4444);
                // Flag 0: destroyed, kept.
                assert_eq!(e.call(destructor, &args![map, 0u32]).u32(), map.addr());
                assert!(events.borrow().iter().all(|ev| ev.0 != "free"));
                assert_eq!(events.borrow().last().unwrap().0, "ni_free");
                // Flag 1: destroyed, then freed.
                events.borrow_mut().clear();
                assert_eq!(e.call(destructor, &args![map, 1u32]).u32(), map.addr());
                assert_eq!(*events.borrow().last().unwrap(), ("free", map.addr(), 0));
                // Only bit 0 counts.
                events.borrow_mut().clear();
                e.call(destructor, &args![map, 2u32]);
                assert!(events.borrow().iter().all(|ev| ev.0 != "free"));
            }
        }
    }
}
