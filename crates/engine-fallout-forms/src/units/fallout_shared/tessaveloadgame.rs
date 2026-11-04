//! `fallout shared/tessaveloadgame.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit has 192 functions (`ledger queue "fallout shared/tessaveloadgame.cpp"`);
//! it is translated in address order, a session at a time. State of this
//! file: all the functions of its range (up to `008616f0`): `00486a90` and
//! `00666050` (two `NiTPointerMap` instance methods the linker placed far from
//! the unit), `00854e10` to `0085a520`, and, in the third batch, `0085ac30` to
//! `00861640`: applying initial data to a loaded reference, `SavePluginList`,
//! `BuildChangesString`, the animation / character controller / Havok block
//! maps, the list of saves (`SaveGameFile`), the description of a save from its
//! name or header, `ReadSaveGameDataOLD`, `LoadHeader` and `SaveHeader`. The
//! rest of the unit is in `tessaveloadgame_p2.rs`.
//!
//! What is here:
//!
//! - `ChangeData` and `ChangesMap` (the per-form record of what changed, and
//!   the hash map of those records keyed by form id): the accessors
//!   `00854e10` to `00855220`, the map's destructors and
//!   `ChangesMap::RemoveAllChanges`;
//! - the three smaller maps `InteriorCellNewReferencesMap`,
//!   `ExteriorCellNewReferencesMap` and `NumericIDBufferMap` (constructor,
//!   destructor and scalar deleting destructor of each);
//! - `SaveStats` (the statistics of a save, which `PrintStats` writes out as
//!   a text file), `SaveStats::Stats` and the small file-writing helper;
//! - `TESSaveLoadGame::RemoveChanges` and the save routine `00856ca0`;
//! - the small value types of the save format: the constructors of
//!   `ReferenceData`, `MovedReferenceData` (`008572f0`, `00857320`) and the
//!   `LoadFormHeader` fill (`00858aa0`);
//! - opening and closing the save file (`00857370`, `008578b0`, `00857950`),
//!   the buffer cursor (`008579b0`, `008579e0`, `00857bd0`, `CreateBuffer`,
//!   `WriteFile`, `00858700`), writing and reading file bytes (`00857b50`,
//!   `00857ba0`), the numeric id helpers (`SaveNumericID`, `LoadNumericID`,
//!   the plugin index maps `00857bf0`, `00857c70`);
//! - the global data of a save (`SaveGlobalData`, `SaveGlobals`,
//!   `SaveFinalData`);
//! - loading: `00857d10` (put a loaded reference at its saved place),
//!   `00858730` (load one form), `00858af0` (apply the queued initial data),
//!   `00859690` and `008598d0` (a cell's new references), `00859a90`
//!   (make a created reference), `00859f20` (load a moved reference from the
//!   plugins), `DeleteForm` and its helpers;
//! - `CheckNewReference`, `CheckFlags`, `GetInitialDataSaveSize` and
//!   `SaveInitialData`.
//!
//! Layouts and constants are below. What the next session needs:
//!
//! - `TESSaveLoadGame` (0x1C8 bytes, the size `TES`'s constructor allocates),
//!   `ChangeData`, `SaveStats`, `Stats`, `ExtraStat`, `LoadFormHeader`,
//!   `SaveFormHeader`, `FormAndFlags`, `ReferenceData`, `MovedReferenceData`,
//!   `CreatedReferenceData` and `ExteriorCellReferenceData` are declared here
//!   with the fields used so far;
//! - the pointer `011de45c` is the game's `TESSaveLoadGame`; the save code
//!   reads it through memory and passes it as `this` to several helpers that
//!   are members of the same class (`00857b50`, `008579e0`, `0085b320`,
//!   `0047c850`);
//! - the shared map and list helpers (`NiTMapBase::GetFirstPos` `004b9ba0`,
//!   `GetNext` `006b7f20` for a map keyed by `unsigned int`, `00863bc0` for
//!   `SaveStats`'s map keyed by a byte, `RemoveAll` `00438af0`, `RemoveAt`
//!   `00405430`, `GetAt` `00853130`, `SetAt` `00844700`; the `BSSimpleList`
//!   node item `006815c0`, next `00726070`, `RemoveAll` `00470470`, scalar
//!   deleting destructor `004702f0`) are called by address.
//!
//! Not translated: the compiler's exception-unwinding frames (the `FS:[0]`
//! chains and state variables) and the stack-cookie check of `00855ba0`.
//! The locals the game keeps on its stack and passes by address (the 4-byte
//! allocation-scope object, out parameters, the buffers and the file object
//! of `PrintStats`, the form header of `00856ca0`) are heap blocks here,
//! freed where the game's scope ends.
//!
//! The decompiler dropped or mis-attached several arguments in this unit
//! (the `this` of `0047c850`, `0085b320`, `00857b50`; the five stack
//! arguments of the "can't save" message, which belong to `007052f0`; the
//! base-class destructor calls of the map destructors; the argument of
//! `00410220` in `00857d10`, the arguments of `00403df0` and `00464f30` in
//! `00857370`, the `1` that `00469800` leaves on the stack for
//! `DeleteForm`'s virtual call), so the translations follow the disassembly.
//!
//! `0047c850` returns false in this build (`XOR AL,AL`). The functions that
//! start with "do nothing unless it is true" (`00858730`, `00858af0`,
//! `00859690`) therefore return at once in this build; they are translated
//! all the same. Several flag tests in this unit compare against a constant
//! 0 (`flags & 0`, the compiler stored the mask in the code but the mask is
//! 0 in this build); the code they guard is unreachable and not translated
//! (`008598d0`, which would call `00859a90` and `00859f20`; `008591b0`,
//! `0085a450`, `0085a520`
//! say so).

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `ChangeData` (Xbox PDB), 8 bytes: what is saved for one form.
    pub struct ChangeData: 0x08 {
        /// `iFlags` (Xbox PDB): the changed-parts bits.
        0x00 iFlags: u32,
        /// `pBuffer` (Xbox PDB): the form's pre-built save buffer, or null.
        0x04 pBuffer: Ptr,
    }

    /// `ChangesMap` (Xbox PDB): `NiTPointerMap<unsigned int, ChangeData *>`,
    /// 0x10 bytes. Vtable at +0 (slot 0x14 `NewItem`, 0x18 `DeleteItem`).
    pub struct ChangesMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: Ptr,
        /// The allocator subobject (`AntiBloatAllocator`, Xbox PDB
        /// `m_kAllocator`), which holds the entry count (`NewItem` and
        /// `DeleteItem` pass its address).
        0x0C m_kAllocator: u32,
    }

    /// `InteriorCellNewReferencesMap` (Xbox PDB): a pointer map from a cell
    /// form id to a `BSSimpleList<unsigned int> *`, 0x10 bytes.
    pub struct InteriorCellNewReferencesMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
    }

    /// `ExteriorCellNewReferencesMap` (Xbox PDB): a pointer map to a
    /// `BSSimpleList<ExteriorCellReferenceData *> *`, 0x10 bytes.
    pub struct ExteriorCellNewReferencesMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
    }

    /// `NumericIDBufferMap` (Xbox PDB): a pointer map from an id to a buffer
    /// (`void *`), 0x10 bytes.
    pub struct NumericIDBufferMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
    }

    /// `SaveStats` (Xbox PDB), 8 bytes: the statistics of one save.
    pub struct SaveStats: 0x08 {
        /// `pStatsMap` (Xbox PDB): `NiTPointerMap<unsigned char,
        /// BSSimpleList<LoadFormHeader *> *> *`, from form type to the list
        /// of that type's headers, sorted by descending size.
        0x00 pStatsMap: Ptr,
        /// `pExtraStats` (Xbox PDB): `BSSimpleList<SaveStats::ExtraStat *> *`.
        0x04 pExtraStats: Ptr,
    }

    /// `SaveStats::Stats` (Xbox PDB), 0xC bytes: running count, total,
    /// minimum and maximum size.
    pub struct Stats: 0x0C {
        /// `iNum` (Xbox PDB).
        0x00 iNum: i32,
        /// `iTotalSize` (Xbox PDB).
        0x04 iTotalSize: i32,
        /// `iMinSize` (Xbox PDB).
        0x08 iMinSize: u16,
        /// `iMaxSize` (Xbox PDB).
        0x0A iMaxSize: u16,
    }

    /// `SaveStats::ExtraStat` (Xbox PDB), 8 bytes: a size and its description.
    pub struct ExtraStat: 0x08 {
        /// `iSize` (Xbox PDB).
        0x00 iSize: u32,
        /// `pDescription` (Xbox PDB): a heap copy of the text.
        0x04 pDescription: Ptr,
    }

    /// `LoadFormHeader` (Xbox PDB), 0xC bytes, packed (the flags word is
    /// unaligned).
    pub struct LoadFormHeader: 0x0C {
        /// `iFormID` (Xbox PDB).
        0x00 iFormID: u32,
        /// `cFormType` (Xbox PDB).
        0x04 cFormType: u8,
        /// `iFlags` (Xbox PDB), at the odd offset +5.
        0x05 iFlags: u32,
        /// `cVersion` (Xbox PDB).
        0x09 cVersion: u8,
        /// `iSize` (Xbox PDB).
        0x0A iSize: u16,
    }

    /// `SaveFormHeader` (Xbox PDB), 0xA bytes: the `LoadFormHeader` without
    /// the size.
    pub struct SaveFormHeader: 0x0A {
        /// `iFormID` (Xbox PDB).
        0x00 iFormID: u32,
        /// `cFormType` (Xbox PDB).
        0x04 cFormType: u8,
        /// `iFlags` (Xbox PDB), at the odd offset +5.
        0x05 iFlags: u32,
        /// `cVersion` (Xbox PDB).
        0x09 cVersion: u8,
    }

    /// `FormAndFlags` (Xbox PDB), 0x10 bytes.
    pub struct FormAndFlags: 0x10 {
        /// `pForm` (Xbox PDB).
        0x00 pForm: Ptr,
        /// `iFlags` (Xbox PDB).
        0x04 iFlags: u32,
        /// `iOldFlags` (Xbox PDB).
        0x08 iOldFlags: u32,
        /// `cVersion` (Xbox PDB).
        0x0C cVersion: u8,
    }

    /// `TESSaveLoadGame` (Xbox PDB), 0x1C8 bytes on the PC too (the size
    /// `TES` allocates); the fields this unit's first functions use, at the
    /// offsets the PC code uses (equal to the Xbox PDB's).
    pub struct TESSaveLoadGame: 0x1C8 {
        /// `m_pChanges` (Xbox PDB): the `ChangesMap`.
        0x00 m_pChanges: Ptr<ChangesMap>,
        /// `m_pNewChanges` (Xbox PDB): the `ChangesMap` of the changes made
        /// while a save is being loaded, or null.
        0x04 m_pNewChanges: Ptr<ChangesMap>,
        /// `m_pInteriorCellMap` (Xbox PDB): the
        /// `InteriorCellNewReferencesMap`.
        0x08 m_pInteriorCellMap: Ptr,
        /// `m_pExteriorCellMap` (Xbox PDB): the
        /// `ExteriorCellNewReferencesMap`.
        0x0C m_pExteriorCellMap: Ptr,
        /// `m_pCreatedExteriorCells` (Xbox PDB): an
        /// `ExteriorCellNewReferencesMap` of the exterior cells the save
        /// created, from world space id to a list of
        /// `ExteriorCellReferenceData *`.
        0x10 m_pCreatedExteriorCells: Ptr,
        /// `m_pBuffer` (Xbox PDB): the current form's pre-built buffer.
        0x14 m_pBuffer: Ptr,
        /// `m_pInitArray` (Xbox PDB):
        /// `NiTLargePrimitiveArray<FormAndFlags *> *`, the forms whose
        /// initial data still has to be applied after a load.
        0x20 m_pInitArray: Ptr,
        /// `m_pSaveLoadStats` (Xbox PDB): the `SaveStats`, or null.
        0x44 m_pSaveLoadStats: Ptr<SaveStats>,
        /// `m_iSavedPluginCount` (Xbox PDB).
        0x4C m_iSavedPluginCount: u8,
        /// `m_pFileIndexArray` (Xbox PDB): the table that maps the plugin
        /// index a save was written with to the current one (0xFF: none).
        0x50 m_pFileIndexArray: Ptr,
        /// `m_iQueuedRemoveChanges` (Xbox PDB): flags of a `RemoveChanges`
        /// that was asked for while a form was loading.
        0x54 m_iQueuedRemoveChanges: u32,
        /// `m_pAnimationMap` (Xbox PDB): a `NumericIDBufferMap`, from form id
        /// to a block (a `u16` size, then that many bytes) of the saved
        /// animation state.
        0x58 m_pAnimationMap: Ptr,
        /// `m_pAttachedAnimationMap` (Xbox PDB): the same, for attached
        /// animations.
        0x5C m_pAttachedAnimationMap: Ptr,
        /// `m_pCharControllerMap` (Xbox PDB): the same, for character
        /// controllers.
        0x60 m_pCharControllerMap: Ptr,
        /// `m_pHavokDataMap` (Xbox PDB): the same, for Havok data.
        0x64 m_pHavokDataMap: Ptr,
        /// `m_QueuedFormList` (Xbox PDB): a `BSSimpleList<TESForm *>` embedded
        /// in the object (its head node is at this address).
        0x68 m_QueuedFormList: u32,
        /// `m_pSaveGameList` (Xbox PDB): `BSSimpleList<SaveGameFile *> *`.
        0x70 m_pSaveGameList: Ptr,
        /// `m_cMajorVersion` (Xbox PDB): the major version of the save format
        /// of the save being read or written.
        0x74 m_cMajorVersion: u8,
        /// `m_cMinorVersion` (Xbox PDB).
        0x75 m_cMinorVersion: u8,
        /// `m_cCurrentVersion` (Xbox PDB): the version being processed
        /// (`008df040` reads it; `00856850` copies `m_cMinorVersion` to it).
        0x80 m_cCurrentVersion: u8,
        /// `m_bUseNumericIDArray` (Xbox PDB).
        0x81 m_bUseNumericIDArray: bool,
        /// `m_pCurrentlySavingFormHeader` (Xbox PDB).
        0x88 m_pCurrentlySavingFormHeader: Ptr,
        /// `m_iNextSaveNumber` (Xbox PDB).
        0x8C m_iNextSaveNumber: u32,
        /// `m_iFileStartPosition` (Xbox PDB): 0xD000 for a save with the
        /// "CON " prefix, else 0.
        0x90 m_iFileStartPosition: u32,
        /// `m_iSimulationFileSize` (Xbox PDB): the bytes counted instead of
        /// written when the save only measures.
        0x94 m_iSimulationFileSize: u32,
        /// `m_OriginalSaveTime` (Xbox PDB), a `_SYSTEMTIME` (16 bytes) at this
        /// offset.
        0x98 m_OriginalSaveTime: u32,
        /// `m_iOriginalSaveVersion` (Xbox PDB).
        0xA8 m_iOriginalSaveVersion: u32,
    }

    /// `ReferenceData` (Xbox PDB), 0x1C bytes: where a reference is, as the
    /// save format stores it (the location's numeric id, position, angle).
    pub struct ReferenceData: 0x1C {
        /// `iLocationID` (Xbox PDB).
        0x00 iLocationID: u32,
        /// `Loc` (Xbox PDB), a `NiPoint3`: x.
        0x04 LocX: f32,
        /// `Loc`: y.
        0x08 LocY: f32,
        /// `Loc`: z.
        0x0C LocZ: f32,
        /// `Angle` (Xbox PDB), a `NiPoint3`: x.
        0x10 AngleX: f32,
        /// `Angle`: y.
        0x14 AngleY: f32,
        /// `Angle`: z.
        0x18 AngleZ: f32,
    }

    /// `MovedReferenceData` (Xbox PDB), 0x2C bytes: the original location id
    /// (`iOriginalLocationID`, +0) and position (`OriginalLoc`, +4), then a
    /// `ReferenceData` (`RefData`) at +0x10.
    pub struct MovedReferenceData: 0x2C {
        /// `iOriginalLocationID` (Xbox PDB).
        0x00 iOriginalLocationID: u32,
    }

    /// `CreatedReferenceData` (Xbox PDB), 0x24 bytes: the type, the bound
    /// object's id, then a `ReferenceData` (`RefData`) at +8.
    pub struct CreatedReferenceData: 0x24 {
        /// `eType` (Xbox PDB), a `TESCreatedReferenceType`: 0 normal, 1 arrow
        /// projectile, 2 magic projectile, 3 persistent.
        0x00 eType: u32,
        /// `iBoundID` (Xbox PDB): the numeric id of the base object.
        0x04 iBoundID: u32,
    }

    /// `ExteriorCellReferenceData` (Xbox PDB), 0xC bytes.
    pub struct ExteriorCellReferenceData: 0x0C {
        /// `iFormID` (Xbox PDB).
        0x00 iFormID: u32,
        /// `iCellX` (Xbox PDB).
        0x04 iCellX: i32,
        /// `iCellY` (Xbox PDB).
        0x08 iCellY: i32,
    }

    /// `SaveGameFile` (Xbox PDB), 0x164 bytes on the PC: a `BSFile` (0x158
    /// bytes) with the time the file was last written, which the list of
    /// saves sorts by.
    pub struct SaveGameFile: 0x164 {
        /// `bHasFileTime` (Xbox PDB).
        0x158 bHasFileTime: bool,
        /// `FileTime` (Xbox PDB), a `_FILETIME`: the low dword.
        0x15C FileTimeLow: u32,
        /// `FileTime`: the high dword.
        0x160 FileTimeHigh: u32,
    }
}

// ---------------------------------------------------------------------------
// Constants: callees and data outside this file

/// `operator new(size)` (`00401000`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`00401030`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The dword at `this + 0x0C` (`MOV EAX,[ECX+0xC]`): for a form, its form id.
pub(crate) const FORM_ID: u32 = 0x0084_e3a0;
/// `MOV EAX,[ECX]`: the word at the address `this` (the flags of a
/// `ChangeData`; the first word of a file object).
pub(crate) const READ_WORD: u32 = 0x0055_9450;
/// The global that holds the pointer to the game's `TESSaveLoadGame`.
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `0047c850`: takes a `TESSaveLoadGame` and returns false in this build
/// (`XOR AL,AL`); the save code asks it before every write.
pub(crate) const SAVE_LOAD_UNAVAILABLE: u32 = 0x0047_c850;
/// `TESSaveLoadGame::GetSavingAllowed` (Xbox PDB), `this` the game.
pub(crate) const GET_SAVING_ALLOWED: u32 = 0x0086_16f0;
/// `(form flags at +8) & 0x4000` (`SETNZ`): the form is deleted.
pub(crate) const FORM_IS_DELETED: u32 = 0x0040_77c0;

/// The allocation scope: `this` is a 4-byte object, then (0x11, 1, file,
/// line); `00404eb0` constructs it, `00404ee0` destroys it.
pub(crate) const SCOPE_ENTER: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_LEAVE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESSaveLoadGame.cpp"`.
pub(crate) const SOURCE_FILE: u32 = 0x0108_04c8;

/// `NiTMapBase<unsigned int, X *>::GetAt(key, &value) -> bool`.
pub(crate) const MAP_GET_AT: u32 = 0x0085_3130;
/// `NiTMapBase::SetAt(key, value)` of `ChangesMap`.
pub(crate) const CHANGES_MAP_SET_AT: u32 = 0x0084_4700;
/// `NiTMapBase<unsigned int, X *>::RemoveAt(key) -> bool`.
pub(crate) const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// `NiTMapBase::GetFirstPos`: the first used slot's entry, or 0.
pub(crate) const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `NiTMapBase<unsigned int, X *>::GetNext(&pos, &key, &value)`.
pub(crate) const MAP_NEXT: u32 = 0x006b_7f20;
/// `SaveStats`'s map: `GetNext(&pos, &key (a byte), &value)`.
pub(crate) const BYTE_MAP_NEXT: u32 = 0x0086_3bc0;
/// `SaveStats`'s map: `GetAt(key (a byte), &value) -> bool`.
pub(crate) const BYTE_MAP_GET_AT: u32 = 0x0086_3b40;
/// `SaveStats`'s map: `SetAt(key (a byte), value)`.
pub(crate) const BYTE_MAP_SET_AT: u32 = 0x0086_3a60;
/// `NiTMapBase::RemoveAll`.
pub(crate) const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// The map allocator subobject's `Allocate` (called with `map + 0x0C`) and
/// `Deallocate(item)`.
pub(crate) const MAP_ALLOCATOR_NEW_ITEM: u32 = 0x0043_a010;
pub(crate) const MAP_ALLOCATOR_DELETE_ITEM: u32 = 0x0045_cee0;

/// `BSSimpleList` constructor (`0096a2d0`: item and next set to null; the
/// linker folded `ChangeData`'s constructor into it).
pub(crate) const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList` node's item address (`006815c0`: returns `this`; the same
/// code is the empty constructor of the form headers).
pub(crate) const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// A node's next node (`00726070`: `[this + 4]`).
pub(crate) const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList::RemoveAll`.
pub(crate) const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// `BSSimpleList` scalar deleting destructor: `this` and the delete flag.
pub(crate) const LIST_SCALAR_DELETE: u32 = 0x0047_02f0;
/// `BSSimpleList::AddHead(&item)`.
pub(crate) const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// `BSSimpleList::Insert(item, comparator)`, keeping the list sorted by the
/// comparator (a `cdecl` function of two items).
pub(crate) const LIST_INSERT: u32 = 0x007a_7eb0;
/// The comparator `fn_00855a20` passes to it.
pub(crate) const STATS_COMPARATOR: u32 = 0x0085_5b60;

/// `strlen` through the game's wrapper.
pub(crate) const STRLEN: u32 = 0x0044_a670;
/// `strcpy_s(destination, size, source)` through the game's wrapper.
pub(crate) const STRING_COPY: u32 = 0x0040_6d30;
/// `strcat_s(destination, size, source)`.
pub(crate) const STRING_CAT: u32 = 0x0040_6d50;
/// `sprintf_s(buffer, size, format, ...)`.
pub(crate) const FORMAT: u32 = 0x0040_6d00;
/// `strcmp(a, b)`.
pub(crate) const STRING_COMPARE: u32 = 0x0040_8b20;
/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`.
pub(crate) const DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// The `NiTPointerMap` base constructors (`this`, hash size) and
/// destructors of the maps this unit owns.
pub(crate) const CHANGES_MAP_BASE_DESTRUCT: u32 = 0x0086_3640;
pub(crate) const INTERIOR_MAP_BASE_CONSTRUCT: u32 = 0x0086_3390;
pub(crate) const INTERIOR_MAP_BASE_DESTRUCT: u32 = 0x0086_3740;
pub(crate) const EXTERIOR_MAP_BASE_CONSTRUCT: u32 = 0x0086_33c0;
pub(crate) const EXTERIOR_MAP_BASE_DESTRUCT: u32 = 0x0086_3860;
pub(crate) const NUMERIC_ID_MAP_BASE_CONSTRUCT: u32 = 0x0086_33f0;
pub(crate) const NUMERIC_ID_MAP_BASE_DESTRUCT: u32 = 0x0086_3960;
/// `SaveStats`'s `NiTPointerMap<unsigned char, ...>` constructor (`this`,
/// hash size).
pub(crate) const STATS_MAP_CONSTRUCT: u32 = 0x0086_3420;
/// Hash size of the three reference maps and of `SaveStats`'s map.
pub(crate) const HASH_SIZE: u32 = 0x25;

/// Vtables of the maps this unit owns.
pub(crate) const CHANGES_MAP_VTABLE: u32 = 0x0108_04a8;
pub(crate) const INTERIOR_MAP_VTABLE: u32 = 0x0108_0514;
pub(crate) const EXTERIOR_MAP_VTABLE: u32 = 0x0108_0534;
pub(crate) const NUMERIC_ID_MAP_VTABLE: u32 = 0x0108_0554;

/// `TESSaveLoadGame` members the save routine calls (`this` the game unless
/// noted).
pub(crate) const SAVE_HEADER: u32 = 0x0086_1130; // (file, name)
pub(crate) const SAVE_PLUGIN_LIST: u32 = 0x0085_b240; // (file)
pub(crate) const SAVE_GLOBAL_DATA: u32 = 0x0085_8030; // (file)
pub(crate) const SAVE_FINAL_DATA: u32 = 0x0085_8570; // (file)
pub(crate) const SAVE_NUMERIC_ID_ARRAYS: u32 = 0x0086_1d10; // (file)
pub(crate) const CHECK_FLAGS: u32 = 0x0085_91b0; // (form, flags) -> flags
pub(crate) const GET_INITIAL_DATA_SAVE_SIZE: u32 = 0x0085_a450; // (form, flags) -> u16
pub(crate) const SAVE_INITIAL_DATA: u32 = 0x0085_a520; // (form, flags)
pub(crate) const CREATE_BUFFER: u32 = 0x0085_8600; // (size) -> buffer
pub(crate) const WRITE_FILE: u32 = 0x0085_86a0; // (file, buffer, size)
pub(crate) const FREE_BUFFER: u32 = 0x0085_8700; // (buffer)
/// Writes `size` bytes at `data` to the file: (file, data, size).
pub(crate) const WRITE_BYTES: u32 = 0x0085_7b50;
/// Reads 4 bytes of the current buffer into `data`: (data, size).
pub(crate) const READ_BYTES: u32 = 0x0085_79e0;
/// The version number of the save format.
pub(crate) const CURRENT_VERSION: u32 = 0x008d_f040;
/// Opens the save file: (file or null, name, 0) -> the file.
pub(crate) const OPEN_SAVE_FILE: u32 = 0x0085_7370;
/// The save's preparation steps, each `this` only.
pub(crate) const SAVE_PREPARE_A: u32 = 0x0086_27b0;
pub(crate) const SAVE_PREPARE_B: u32 = 0x0086_20f0;
pub(crate) const SAVE_PREPARE_C: u32 = 0x0085_6850;
/// The save's closing steps: (file) and (file, 0).
pub(crate) const SAVE_CLOSE_A: u32 = 0x0086_2150;
pub(crate) const SAVE_CLOSE_B: u32 = 0x0085_78b0;
/// The current position of a file (`this` the file).
pub(crate) const FILE_POSITION: u32 = 0x0047_20a0;
/// The form type byte of a form (`MOVZX EAX,byte [ECX+4]`).
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// `LookupFormByID(id)`, `cdecl`; null for none.
pub(crate) const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The global that holds the object whose `Enter` and `Leave` (`00c3e310`,
/// `00c3e340`) bracket a save.
pub(crate) const SAVE_LOCK: u32 = 0x0120_2d98;
pub(crate) const SAVE_LOCK_ENTER: u32 = 0x00c3_e310;
pub(crate) const SAVE_LOCK_LEAVE: u32 = 0x00c3_e340;
/// File object virtual slots: `0x14` seek(position, mode), `0x18` name.
pub(crate) const FILE_SEEK: u32 = 0x14;
pub(crate) const FILE_GET_NAME: u32 = 0x18;
/// The seek mode the save routine uses (a global dword).
pub(crate) const SEEK_MODE: u32 = 0x010a_2480;
/// Form virtual slots: `0x50` the size of the changed data, `0x58` its save,
/// `0x130` its description.
pub(crate) const FORM_GET_CHANGES_SIZE: u32 = 0x50;
pub(crate) const FORM_SAVE_CHANGES: u32 = 0x58;
pub(crate) const FORM_GET_DESCRIPTION: u32 = 0x130;

/// The "can't save" message: the message queue getter (`this` the object at
/// `011d2364`) and `007052f0(queue, 0, icon, 0, time, 0)`.
pub(crate) const MESSAGE_QUEUE_OBJECT: u32 = 0x011d_2364;
pub(crate) const GET_MESSAGE_QUEUE: u32 = 0x004c_69f0;
pub(crate) const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds"`.
pub(crate) const SAD_ICON: u32 = 0x0102_08a0;
/// The message's display time (a `float` constant).
pub(crate) const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `"autosave"`.
pub(crate) const AUTOSAVE_NAME: u32 = 0x0107_fae0;

/// `FileFinder::Exist(path, 0, 0, -1)` and `BSSystemFile::DeleteFileA(path)`.
pub(crate) const FILE_EXISTS: u32 = 0x0045_6a20;
pub(crate) const FILE_DELETE: u32 = 0x00af_f0b0;
/// The text file object's constructor (`this`, path, 1, 2, 0) and
/// destructor.
pub(crate) const FILE_OBJECT_CONSTRUCT: u32 = 0x00b0_0900;
pub(crate) const FILE_OBJECT_DESTRUCT: u32 = 0x00b0_0950;
/// `BSSystemFile::DoWrite(this = file, text, size, 0, &scratch) -> error`.
pub(crate) const SYSTEM_FILE_DO_WRITE: u32 = 0x0085_6350;
/// `00aa15a0(file)`: the call that ends a save's use of the file.
pub(crate) const FILE_FLUSH: u32 = 0x00aa_15a0;
/// `TESSaveLoadGame::BuildChangesString(buffer, form, flags, type, 0)`,
/// `this` the singleton.
pub(crate) const BUILD_CHANGES_STRING: u32 = 0x0085_b320;
/// The name of a reference (`this` the reference) and the location name of
/// a map marker (`""` when it has none).
pub(crate) const REFERENCE_GET_NAME: u32 = 0x0055_d520;
pub(crate) const MAP_MARKER_GET_LOCATION_NAME: u32 = 0x0040_8da0;
/// The table of the form type names: 12 bytes per type, the first word the
/// name's address.
pub(crate) const FORM_TYPE_NAME_TABLE: u32 = 0x0118_7004;
/// The initializer of the record embedded at +8 of `fn_008572c0`'s object
/// (`fn_008572f0`, which this file also translates).
pub(crate) const EMBEDDED_RECORD_INIT: u32 = 0x0085_72f0;

// Globals the second batch of functions reads. Pointers (read with
// `e.global`) unless the doc says the address itself is the object.
/// The `TESDataHandler` (`TESDataHandler::GetNextID`, `00469800`, takes it as
/// `this`).
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `TES` object (`TES::GetWorldSpace`, `TES::SaveGame` take it).
pub(crate) const TES_OBJECT: u32 = 0x011d_ea10;
/// The player's reference.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// The `ProcessLists` object (the address itself is the object).
pub(crate) const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `TESSaveLoadGame::SaveGameCriticalSection` (Xbox PDB), a
/// `BSCriticalSection` (the address itself is the object): `004538a0`
/// enters it with a name, `004538c0` leaves it.
pub(crate) const LOAD_SECTION: u32 = 0x011d_e494;
/// The model loader (`ModelLoader::QueueReference` takes it as `this`).
pub(crate) const MODEL_LOADER: u32 = 0x011c_3b3c;
/// A global `NiPoint3` (three floats at this address) used as the position
/// of an actor that has none and as the starting vectors of the placement.
pub(crate) const DEFAULT_POSITION: u32 = 0x011f_426c;
/// A global `NiPoint3` passed to `FN_0057D0A0`.
pub(crate) const PLACEMENT_VECTOR: u32 = 0x011a_9478;
/// An object whose word at `+4` is a path string (`00403df0` reads it).
pub(crate) const PATH_OBJECT: u32 = 0x011c_3f74;

// Run-time type descriptors passed to `__RTDynamicCast`. The name says what
// the code does with the result; the first is the type every form is cast
// from.
pub(crate) const RTTI_FORM: u32 = 0x0118_3028;
pub(crate) const RTTI_REFERENCE: u32 = 0x0118_41cc;
pub(crate) const RTTI_CELL: u32 = 0x0118_3fb4;
pub(crate) const RTTI_WORLDSPACE: u32 = 0x0118_3fd0;
/// The base object of a reference (compared with `007af430`'s result).
pub(crate) const RTTI_BOUND_OBJECT: u32 = 0x0118_3108;
/// A reference that has a process (`Actor::InitPackageLocations` is called
/// on it).
pub(crate) const RTTI_ACTOR: u32 = 0x0118_46d4;
/// A reference with a character controller (`MobileObject` in the engine
/// map).
pub(crate) const RTTI_MOBILE_OBJECT: u32 = 0x0118_4920;

// Library calls and strings.
/// `memcpy(destination, source, size)` (`00401460`).
pub(crate) const MEMCPY: u32 = 0x0040_1460;
/// Returns the address of a constant string (the start of the save paths).
pub(crate) const PATH_PREFIX: u32 = 0x004d_c110;
/// `this` = `PATH_OBJECT`: returns the word at `this + 4`, or 0 for a null
/// `this`.
pub(crate) const PATH_OBJECT_GET: u32 = 0x0040_3df0;
/// Returns its argument (`00464f30`).
pub(crate) const IDENTITY: u32 = 0x0046_4f30;
/// Builds the default save name into a buffer: (game, buffer).
pub(crate) const DEFAULT_SAVE_NAME: u32 = 0x0086_0ae0;
/// `"%s%s%s.ess"`, `".ess"`, `"Save "`, `".bak"` and `""`.
pub(crate) const FORMAT_SAVE_PATH: u32 = 0x0108_06b8;
pub(crate) const ESS_EXTENSION: u32 = 0x0108_06b0;
pub(crate) const SAVE_NAME_PREFIX: u32 = 0x0108_06a8;
pub(crate) const BAK_EXTENSION: u32 = 0x0107_facc;
pub(crate) const EMPTY_STRING: u32 = 0x0101_1584;
/// Import slots of `CreateDirectoryA` (path, 0) and `DeleteFileA` (path), and
/// the CRT's `rename(old, new)`, `strrchr(text, char)` (`0040ab30`) and
/// `_strnicmp(a, b, count)`.
pub(crate) const CREATE_DIRECTORY_IMPORT: u32 = 0x00fd_f0b8;
pub(crate) const DELETE_FILE_IMPORT: u32 = 0x00fd_f0d4;
pub(crate) const RENAME: u32 = 0x00ec_862c;
pub(crate) const STRRCHR: u32 = 0x0040_ab30;
pub(crate) const STRNICMP: u32 = 0x00ec_7ec0;
/// `strstr(text, pattern)` through the game's wrapper (`004812f0`, the CRT
/// function at `00ec7750`): the match, or null.
pub(crate) const STRING_FIND: u32 = 0x0048_12f0;
/// `_sprintf(buffer, format, ...)`.
pub(crate) const SPRINTF: u32 = 0x00ec_623a;
/// `Error(format, ...)` (`0040fbe0`) and the logging call `005b5e40(format,
/// ...)`: `cdecl`, and empty in this build.
pub(crate) const ERROR: u32 = 0x0040_fbe0;
pub(crate) const LOG_ERROR: u32 = 0x005b_5e40;
/// The file's write (`00473180`) and read (`00462d80`): `this` the file,
/// (data, size).
pub(crate) const FILE_WRITE: u32 = 0x0047_3180;
pub(crate) const FILE_READ: u32 = 0x0046_2d80;

// Files and lists.
/// `BSFile::BSFile(this, path, writeMode, bufferSize, 0)`, the size of a
/// `BSFile` on the PC and `BSFile::Close(this)`.
pub(crate) const BSFILE_CONSTRUCT: u32 = 0x00b0_0260;
pub(crate) const BSFILE_SIZE: u32 = 0x158;
pub(crate) const BSFILE_CLOSE: u32 = 0x00af_fd10;
/// File virtual slots: `0` the scalar deleting destructor (flag), `0x20`
/// `BSFile::Open(a, b)` (the slot of the PC's vtable at `010a4764` is
/// `00aff300`).
pub(crate) const FILE_DESTRUCT: u32 = 0x00;
pub(crate) const FILE_OPEN: u32 = 0x20;
/// `BSSimpleList<..>` calls: remove the item `*item_slot` (`00905330`), test
/// that the list holds `*item_slot` (`005f65d0`), remove the first node
/// (`0063f7b0`), the end test of a node (`008256d0`: item and next are both
/// null) and the destructor of a local list (`0046ffb0`).
pub(crate) const LIST_REMOVE: u32 = 0x0090_5330;
pub(crate) const LIST_CONTAINS: u32 = 0x005f_65d0;
pub(crate) const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
pub(crate) const LIST_NODE_IS_END: u32 = 0x0082_56d0;
pub(crate) const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The list of global variables embedded in the data handler (`00461190`
/// returns `this + 0xE8`), its count (`005ae380`) and a global variable's
/// value (`00526ac0`, a float).
pub(crate) const GLOBALS_LIST: u32 = 0x0046_1190;
pub(crate) const LIST_COUNT: u32 = 0x005a_e380;
pub(crate) const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `NiTLargePrimitiveArray<FormAndFlags *>`: the destructor (`00863d60`), the
/// constructor (this, grow, size), `Add(&item)`, the cleanup before the
/// array is deleted and the address of the slot of an index (`00877a30`,
/// (array, index)). The size is the dword at `+0xC` (`0084e3a0`, the same
/// code as `FORM_ID`).
pub(crate) const INIT_ARRAY_DESTRUCT: u32 = 0x0086_3d60;
pub(crate) const INIT_ARRAY_CONSTRUCT: u32 = 0x0086_3e00;
pub(crate) const INIT_ARRAY_ADD: u32 = 0x0086_3d90;
pub(crate) const INIT_ARRAY_CLEANUP: u32 = 0x0086_3db0;
pub(crate) const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
pub(crate) const ARRAY_SIZE: u32 = 0x0084_e3a0;

// `TESSaveLoadGame` members and neighbours.
/// `AddNumericIDToArray(game, id)` and `(game, numeric id) -> form id`
/// (`00861c00`); `00574900(game)` reads `m_bUseNumericIDArray`.
pub(crate) const ADD_NUMERIC_ID: u32 = 0x0086_1b70;
pub(crate) const RESOLVE_NUMERIC_ID: u32 = 0x0086_1c00;
pub(crate) const USE_NUMERIC_IDS: u32 = 0x0057_4900;
/// `00861ee0(game, version)` stores the version of the form being loaded
/// (and logs when it is below 0x13); `00856850(game)` copies
/// `m_cMinorVersion` to it.
pub(crate) const SET_LOAD_VERSION: u32 = 0x0086_1ee0;
pub(crate) const END_FORM_PROCESSING: u32 = 0x0085_6850;
/// `(game, form, flags)`: applies the initial data of a loaded form
/// (`0085ac30`, a later session's range).
pub(crate) const LOAD_INITIAL_DATA: u32 = 0x0085_ac30;
/// `4fb090(game, header)` stores `m_pCurrentlyLoadingFormHeader`.
pub(crate) const SET_LOADING_HEADER: u32 = 0x004f_b090;
/// A function that does nothing and takes one argument (`004534f0`); the
/// code calls it as `this`, 0 or 1 around loads.
pub(crate) const SET_LOADING_STATE: u32 = 0x0045_34f0;
/// `004538a0(section, name)` enters a critical section and
/// `004538c0(section)` leaves it.
pub(crate) const SECTION_ENTER: u32 = 0x0045_38a0;
pub(crate) const SECTION_LEAVE: u32 = 0x0045_38c0;
/// Stores its argument at `this + 4` (`006ecd40`): clears a `ChangeData`'s
/// `pBuffer`.
pub(crate) const CHANGE_DATA_SET_BUFFER: u32 = 0x006e_cd40;
/// The data handler's `GetNextID` (`00469800`) and "knows this form id"
/// (`00469860(handler, id)`).
pub(crate) const DATA_HANDLER_GET_NEXT_ID: u32 = 0x0046_9800;
pub(crate) const DATA_HANDLER_HAS_FORM: u32 = 0x0046_9860;
/// `00484af0` `TESForm::SetDisabled(form, 1)`.
pub(crate) const FORM_SET_DISABLED: u32 = 0x0048_4af0;
/// `00483c70(form)`, `00483710(player)` (empty), the empty-handed
/// `TESPackage::CreatePackage` peer that makes a form of a type
/// (`00670b90`, one byte argument) and `0046a010(form, 1)`.
pub(crate) const FN_00483C70: u32 = 0x0048_3c70;
pub(crate) const EMPTY_FN_00483710: u32 = 0x0048_3710;
pub(crate) const CREATE_FORM_OF_TYPE: u32 = 0x0067_0b90;
pub(crate) const FORM_FINISH: u32 = 0x0046_a010;

/// `BGSLoadFormBuffer::GetVersion(this)` (the byte at `+0x1C`) and the setter
/// (`this`, version).
pub(crate) const BUFFER_GET_VERSION: u32 = 0x008a_81c0;
pub(crate) const BUFFER_SET_VERSION: u32 = 0x008a_8150;
// Form and reference virtual slots.
/// `0x60` loads a form from its buffer (flags, 0); `0x68` and `0x6C` are the
/// hooks before and after a load (flags, old flags); `0x88` runs after a
/// reference was made; `0x128` (id, 1) assigns the form id; `0x134` (name)
/// the editor id.
pub(crate) const FORM_LOAD: u32 = 0x60;
pub(crate) const FORM_BEGIN_INIT: u32 = 0x68;
pub(crate) const FORM_END_INIT: u32 = 0x6C;
pub(crate) const FORM_POST_CREATE: u32 = 0x88;
pub(crate) const FORM_SET_FORM_ID: u32 = 0x128;
pub(crate) const FORM_SET_EDITOR_ID: u32 = 0x134;
/// References: `0x100` true for an actor-like reference, `0x1F4` the
/// position, `0x170` and `0x16C` (out buffer) the saved location and
/// rotation. For an actor-like one `0x290` says it has a saved location,
/// `0x298` and `0x294` give its cell and world space, `0x22C` (flag) is a
/// test. A process: `0x20C`.
pub(crate) const REFERENCE_IS_ACTOR: u32 = 0x100;
pub(crate) const REFERENCE_GET_POSITION_SLOT: u32 = 0x1F4;
pub(crate) const REFERENCE_GET_LOCATION_SLOT: u32 = 0x170;
pub(crate) const REFERENCE_GET_ROTATION_SLOT: u32 = 0x16C;
pub(crate) const ACTOR_HAS_LOCATION_SLOT: u32 = 0x290;
pub(crate) const ACTOR_WORLDSPACE_SLOT: u32 = 0x294;
pub(crate) const ACTOR_CELL_SLOT: u32 = 0x298;
pub(crate) const ACTOR_SLOT_22C: u32 = 0x22C;
pub(crate) const PROCESS_SLOT_20C: u32 = 0x20C;

// References.
/// `TESObjectREFR` calls (`this` the reference unless noted):
/// the world space (`00575d70`) and parent cell (`008d6f30`), the extra data
/// list (`005d43c0`, `this + 0x44`), the position (`00436aa0`, `this +
/// 0x30`) and rotation (`00430830`, `this + 0x24`), whether it persists
/// (`005653d0`), the base object (`007af430`, the dword at `+0x20`),
/// `SetLocationOnReference(this, &position)` (`00575830`), the two setters
/// `005757d0(this, float)` and `00575700(this, x, y, z)`,
/// `MoveRefToNewSpace(reference, cell, worldspace)` (`00573800`, `cdecl`),
/// `GetOrientation(this, out)` (`0056fa00`), the 3D node (`0043fcd0`) and
/// `SetObjectReference(this, base)` (`00575690`).
pub(crate) const REF_GET_WORLDSPACE: u32 = 0x0057_5d70;
pub(crate) const REF_GET_PARENT_CELL: u32 = 0x008d_6f30;
pub(crate) const REF_GET_EXTRA_LIST: u32 = 0x005d_43c0;
pub(crate) const REF_GET_POSITION: u32 = 0x0043_6aa0;
pub(crate) const REF_GET_ROTATION: u32 = 0x0043_0830;
pub(crate) const REF_PERSISTS: u32 = 0x0056_53d0;
pub(crate) const REFERENCE_GET_BASE: u32 = 0x007a_f430;
pub(crate) const REF_SET_POSITION: u32 = 0x0057_5830;
pub(crate) const FN_005757D0: u32 = 0x0057_57d0;
pub(crate) const FN_00575700: u32 = 0x0057_5700;
pub(crate) const REF_MOVE_TO_SPACE: u32 = 0x0057_3800;
pub(crate) const REF_GET_ORIENTATION: u32 = 0x0056_fa00;
pub(crate) const REF_GET_NODE: u32 = 0x0043_fcd0;
pub(crate) const REF_SET_BASE: u32 = 0x0057_5690;
/// The 3D node and collision calls of `fn_00857d10`: `00440460(node,
/// &position)`, `0043fa80(node, orientation)`, `bhkNiCollisionObject::ResetSim`
/// (`00c6bd00(node, 1)`, `cdecl`), the constructor `0043d410(this, float, 0,
/// 0)` of a 12-byte object and `00a59c60(node, object)`.
pub(crate) const FN_00440460: u32 = 0x0044_0460;
pub(crate) const FN_0043FA80: u32 = 0x0043_fa80;
pub(crate) const COLLISION_RESET_SIM: u32 = 0x00c6_bd00;
pub(crate) const FN_0043D410: u32 = 0x0043_d410;
pub(crate) const FN_00A59C60: u32 = 0x00a5_9c60;
/// `MobileObject::GetCharController` (`009306d0`), a test on the controller
/// (`005c0860`) and `bhkCharacterController::SetPosition(controller,
/// &position)` (`005620e0`).
pub(crate) const MOBILE_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
pub(crate) const CHAR_CONTROLLER_TEST: u32 = 0x005c_0860;
pub(crate) const CHAR_CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
/// Extra data lists (`this` the list): `GetSeenData` (`00555bc0`),
/// `GetContainerChanges` (`00418520`), `BaseExtraList::GetExtraData(list,
/// type)` (`00410220`), `GetStartingWorldOrCell` (`0041b320`), the two
/// starting setters `(list, &scratch, reference, x, y, z)` (`0041b180`,
/// `0041b120`) and `0041d460`, which reads the dword at `+0xC` of the
/// extra data of type 0xC (0 when there is none).
pub(crate) const EXTRA_GET_SEEN: u32 = 0x0055_5bc0;
pub(crate) const EXTRA_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
pub(crate) const EXTRA_GET_DATA: u32 = 0x0041_0220;
pub(crate) const EXTRA_GET_STARTING_SPACE: u32 = 0x0041_b320;
pub(crate) const EXTRA_SET_STARTING_POSITION: u32 = 0x0041_b180;
pub(crate) const EXTRA_SET_STARTING_ROTATION: u32 = 0x0041_b120;
pub(crate) const EXTRA_GET_CELL_DATA: u32 = 0x0041_d460;
/// Cells (`this` the cell): `IsInterior` (`00425fd0`), the world space
/// (`TESObjectCELL::GetWorldSpace`, `0054ddd0`), the grid coordinates
/// (`00544c30`, `00544c60`), `GetCOCPlacementInfo(cell, &position,
/// &rotation)` (`0054cfd0`) and the number of plugin files (`005504e0`).
pub(crate) const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
pub(crate) const CELL_GET_WORLDSPACE: u32 = 0x0054_ddd0;
pub(crate) const CELL_GET_X: u32 = 0x0054_4c30;
pub(crate) const CELL_GET_Y: u32 = 0x0054_4c60;
pub(crate) const CELL_GET_PLACEMENT: u32 = 0x0054_cfd0;
pub(crate) const CELL_FILE_COUNT: u32 = 0x0055_04e0;
/// `float -> int` (`00406d90`) and `TESWorldSpace::GetCellFromCellCoord`
/// (`worldspace, x, y`).
pub(crate) const FLOAT_TO_INT: u32 = 0x0040_6d90;
pub(crate) const WORLDSPACE_GET_CELL: u32 = 0x0058_75a0;
/// Actors and their processes: the process of an actor (`008d8520`, the
/// engine map's `MiddleHighProcess::GetSavedAcquireObject` names the folded
/// body), `00933790(actor, package)`, `008aad40(actor, flags)`,
/// `87f890(actor, worldspace, cell, position, float)`,
/// `Actor::InitPackageLocations(actor, 0)` (`00893340`).
pub(crate) const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
pub(crate) const ACTOR_PACKAGE_FLAGS: u32 = 0x0093_3790;
pub(crate) const ACTOR_TEST_FLAGS: u32 = 0x008a_ad40;
pub(crate) const ACTOR_PLACE: u32 = 0x0087_f890;
pub(crate) const ACTOR_INIT_PACKAGE_LOCATIONS: u32 = 0x0089_3340;
/// Constructors of what `fn_00859a90` makes: `Character` (0x1C8 bytes),
/// `Creature` (0x1C0), `TESObjectREFR` (0x68), the arrow projectile (0xC8)
/// and the three magic projectiles (0xC4, 0xD0, 0xD8); each takes the
/// block and returns it.
pub(crate) const CHARACTER_CONSTRUCT: u32 = 0x008d_1d30;
pub(crate) const CREATURE_CONSTRUCT: u32 = 0x008d_43a0;
pub(crate) const REFERENCE_CONSTRUCT: u32 = 0x0055_a2f0;
pub(crate) const ARROW_PROJECTILE_CONSTRUCT: u32 = 0x008c_ab20;
pub(crate) const MAGIC_PROJECTILE_CONSTRUCT_A: u32 = 0x0080_f6e0;
pub(crate) const MAGIC_PROJECTILE_CONSTRUCT_B: u32 = 0x0081_92a0;
pub(crate) const MAGIC_PROJECTILE_CONSTRUCT_C: u32 = 0x0081_17d0;
/// Plugin files (`fn_00859f20`): `TESForm::GetFile(cell, index)`
/// (`00484e60`), `TESFile::GetThreadSafeFile` (`004739b0`),
/// `TESFile::FindForm`-style test (`004734d0(file, cell)`), "contains the
/// form" (`00550e10(file, id)`, `cdecl`), the file's current record type
/// (`00472660`), `TESObjectREFR::CreateReference(type, 1)` (`00564480`,
/// `cdecl`), `TESDataHandler::LoadForm(reference, file)` (`004601d0`,
/// `cdecl`) and `TESWorldSpace::FindCellInFile(worldspace, file, x, y)`
/// (`005854f0`).
pub(crate) const FILE_OF_CELL: u32 = 0x0048_4e60;
pub(crate) const THREAD_SAFE_FILE: u32 = 0x0047_39b0;
pub(crate) const FILE_HAS_CELL: u32 = 0x0047_34d0;
pub(crate) const FILE_HAS_FORM: u32 = 0x0055_0e10;
pub(crate) const FILE_RECORD_TYPE: u32 = 0x0047_2660;
pub(crate) const CREATE_REFERENCE: u32 = 0x0056_4480;
pub(crate) const LOAD_FORM_FROM_FILE: u32 = 0x0046_01d0;
pub(crate) const WORLDSPACE_FIND_CELL_IN_FILE: u32 = 0x0058_54f0;

// Other objects' save routines (`SaveGlobalData`).
/// The data handler's dword at `+0x208` (`0084c580`); `TES` calls:
/// `GetWorldSpace` (`004fd3e0`), the dwords at `+0x24` (`0059bb30`) and
/// `+0x28` (`0045cd60`, also read from a process), the size of its save
/// (`00459100`) and `TES::SaveGame` (`00459230`).
pub(crate) const GLOBAL_DATA_SIZE: u32 = 0x0084_c580;
pub(crate) const TES_GET_WORLDSPACE: u32 = 0x004f_d3e0;
pub(crate) const READ_FIELD_24: u32 = 0x0059_bb30;
pub(crate) const READ_FIELD_28: u32 = 0x0045_cd60;
pub(crate) const TES_SAVE_SIZE: u32 = 0x0045_9100;
pub(crate) const TES_SAVE: u32 = 0x0045_9230;
/// `ProcessLists`: save size (`00975450`), `SaveGame` (`009754f0`), the temp
/// effects list size (`009755b0`) and `SaveTempEffectsList` (`009756c0`).
pub(crate) const PROCESS_LISTS_SAVE_SIZE: u32 = 0x0097_5450;
pub(crate) const PROCESS_LISTS_SAVE: u32 = 0x0097_54f0;
pub(crate) const TEMP_EFFECTS_SIZE: u32 = 0x0097_55b0;
pub(crate) const TEMP_EFFECTS_SAVE: u32 = 0x0097_56c0;
/// `Sky::GetInstance` (`0046dd00`), the sky's save size (`0063e940`) and
/// `Sky::SaveGame` (`0063e9f0`); the interface's (`007065c0`, `00706610`)
/// and the regions' (`004f12b0`, `004f1300`); `SaveCreatedBaseObjects`
/// (`00861820`, (game, file)).
pub(crate) const SKY_INSTANCE: u32 = 0x0046_dd00;
pub(crate) const SKY_SAVE_SIZE: u32 = 0x0063_e940;
pub(crate) const SKY_SAVE: u32 = 0x0063_e9f0;
pub(crate) const INTERFACE_SAVE_SIZE: u32 = 0x0070_65c0;
pub(crate) const INTERFACE_SAVE: u32 = 0x0070_6610;
pub(crate) const REGIONS_SAVE_SIZE: u32 = 0x004f_12b0;
pub(crate) const REGIONS_SAVE: u32 = 0x004f_1300;
pub(crate) const SAVE_CREATED_BASE_OBJECTS: u32 = 0x0086_1820;

// The reload steps of `fn_00858af0`.
/// The dword at `+0x34` of `TES` (`005f36f0`); `00 4543c0(cell)`, which gives
/// the world object of a cell (an interior's, else the exterior one); the
/// exterior world object (`00451010`); a Havok world's `00c66300` (adds 1 to
/// its counter at `+0x18`) and `00c6b540(world, 0)` (takes 1 off).
pub(crate) const READ_FIELD_34: u32 = 0x005f_36f0;
pub(crate) const CELL_GET_PHYSICS_WORLD: u32 = 0x0045_43c0;
pub(crate) const GET_EXTERIOR_WORLD: u32 = 0x0045_1010;
pub(crate) const WORLD_ADD_LOCK: u32 = 0x00c6_6300;
pub(crate) const WORLD_REMOVE_LOCK: u32 = 0x00c6_b540;
/// `00459920(TES)`; `PlayerCharacter::Get3D`-like `00950bb0(player, 0)`
/// (the 3D node of the reference); `ModelLoader::QueueReference(loader,
/// reference, 0, 0)`; `IOManager::LoadQueuedPriority` and `008495b0` (sets
/// the dword at `+0x68` to 5), both on the object at `SAVE_LOCK`;
/// `00451590(byte)` (stores a byte in a global) and `0057d0a0` (seven
/// words: a position, a vector and 1.0).
pub(crate) const FN_00459920: u32 = 0x0045_9920;
pub(crate) const PLAYER_GET_3D: u32 = 0x0095_0bb0;
pub(crate) const MODEL_LOADER_QUEUE_REFERENCE: u32 = 0x0044_4850;
pub(crate) const IO_MANAGER_LOAD_QUEUED_PRIORITY: u32 = 0x0045_6520;
pub(crate) const IO_MANAGER_SET_STATE_5: u32 = 0x0084_95b0;
pub(crate) const SET_GLOBAL_FLAG: u32 = 0x0045_1590;
pub(crate) const FN_0057D0A0: u32 = 0x0057_d0a0;

// Strings in `.rdata` (addresses) the functions pass on.
pub(crate) const LABEL_TES_CLASS: u32 = 0x0108_06f8;
pub(crate) const LABEL_PROCESS_LISTS: u32 = 0x0108_06e4;
pub(crate) const LABEL_SKY: u32 = 0x0108_06d8;
pub(crate) const LABEL_HUD_RETICLE: u32 = 0x0108_06cc;
pub(crate) const LABEL_INTERFACE: u32 = 0x0107_ca80;
pub(crate) const LABEL_REGIONS: u32 = 0x0108_06c4;
pub(crate) const LABEL_GLOBAL_VARIABLES: u32 = 0x0107_f540;
pub(crate) const LABEL_TEMP_EFFECTS: u32 = 0x0108_073c;
pub(crate) const MSG_PLAYER_HAS_NO_SPACE: u32 = 0x0108_0704;
pub(crate) const MSG_NO_SAVE_BUFFER: u32 = 0x0108_0750;
pub(crate) const MSG_LOAD_FORM_SECTION: u32 = 0x0108_07fc;
pub(crate) const FORMAT_LOAD_ERROR: u32 = 0x0108_0780;
pub(crate) const MSG_ACTOR_NO_EDITOR_LOCATION: u32 = 0x0108_0818;
pub(crate) const MSG_CELL_REFERENCE_NO_FLAG: u32 = 0x0108_0864;
pub(crate) const MSG_BOUND_OBJECT_MISSING: u32 = 0x0107_fa20;
pub(crate) const MSG_INVALID_CREATED_TYPE: u32 = 0x0108_08a0;
pub(crate) const MSG_NO_CELL_OR_WORLDSPACE: u32 = 0x0108_0958;
pub(crate) const MSG_REFERENCE_NOT_LOADED: u32 = 0x0108_0900;
pub(crate) const MSG_DELETE_FORM_NOT_LOADING: u32 = 0x0108_09a0;
pub(crate) const MSG_NON_PERSISTENT_NO_CELL: u32 = 0x0108_0ba8;
pub(crate) const MSG_PERSISTENT_NO_CELL: u32 = 0x0108_0b38;
pub(crate) const MSG_HIGH_PROCESS_NO_CELL: u32 = 0x0108_0ae8;
pub(crate) const MSG_MIDDLE_HIGH_PROCESS_NO_CELL: u32 = 0x0108_0a90;
/// The name of the form's type, `table[type * 12]` of `FORM_TYPE_NAME_TABLE`
/// (`00440e30`, `this` the form).
pub(crate) const FORM_TYPE_NAME: u32 = 0x0044_0e30;

// ---------------------------------------------------------------------------
// Constants of the third batch of functions (`0085ac30` to `00861640`)

/// Run-time type descriptors (decorated names `.?AVBaseProcess@@`,
/// `.?AVHighProcess@@`, `.?AVArrowProjectile@@`, `.?AVTESQuest@@`).
pub(crate) const RTTI_BASE_PROCESS: u32 = 0x0118_a6b8;
pub(crate) const RTTI_HIGH_PROCESS: u32 = 0x0118_a6d4;
pub(crate) const RTTI_ARROW_PROJECTILE: u32 = 0x011a_28e0;
pub(crate) const RTTI_QUEST: u32 = 0x0118_6500;

/// `fn_0085ac30`: the check that moves a loaded reference with a corrupt
/// location to a valid one (`008624e0(game, reference)`, a function of
/// this unit outside this file's range), `TESObjectCELL::RemoveReference`
/// (`0054ca90`, `this` the cell, the reference), `ModelLoader::CancelReference`
/// (`00445570`, `this` the model loader, the reference).
pub(crate) const FIX_CORRUPT_LOCATION: u32 = 0x0086_24e0;
pub(crate) const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
pub(crate) const MODEL_LOADER_CANCEL_REFERENCE: u32 = 0x0044_5570;
/// Reference virtual slots called when a reference is taken out of its cell:
/// `0x228` (one argument, 0) and `0x1CC` (0, 1).
pub(crate) const REFERENCE_SLOT_228: u32 = 0x228;
pub(crate) const REFERENCE_SLOT_1CC: u32 = 0x1CC;
/// `"SAVELOAD: Trying to put non-persistent reference in non-existent cell."`
/// and `"SAVELOAD: Trying to load non-persistent ref into non-existent cell."`.
pub(crate) const MSG_PUT_NON_PERSISTENT: u32 = 0x0108_0c00;
pub(crate) const MSG_LOAD_NON_PERSISTENT: u32 = 0x0108_0c48;

/// `fn_0085b170`: the reference's virtual slot `0x1E8` (gives the object
/// that holds the five data slots) and `0043f220(object, index)`, which
/// gives the address of a slot's entry (null for none).
pub(crate) const REFERENCE_SLOT_1E8: u32 = 0x1E8;
pub(crate) const OBJECT_ENTRY_AT: u32 = 0x0043_f220;
/// The multiplier of the hash `fn_0085b170` builds (`hash * 0x1003F + id`).
pub(crate) const ENTRY_HASH_MULTIPLIER: u32 = 0x1003F;

/// `SavePluginList`: `TESDataHandler` calls on the object at `DATA_HANDLER`
/// (`0051f550(handler)` the number of compiled files, a byte;
/// `00465010(handler, index)` the compiled file), `00891170(file)` the name
/// of a compiled file, and the label `"Plugin List"` of its statistics line.
pub(crate) const DATA_HANDLER_FILE_COUNT: u32 = 0x0051_f550;
pub(crate) const DATA_HANDLER_GET_COMPILED_FILE: u32 = 0x0046_5010;
pub(crate) const COMPILED_FILE_NAME: u32 = 0x0089_1170;
pub(crate) const LABEL_PLUGIN_LIST: u32 = 0x0108_0c8c;
/// `NiTMapBase<unsigned int, X *>::SetAt(key, value)` of the pointer maps
/// of animation, character controller and Havok blocks (`00844700`, the
/// code `CHANGES_MAP_SET_AT` names for `ChangesMap`).
pub(crate) const MAP_SET_AT: u32 = CHANGES_MAP_SET_AT;

/// The animation and character controller blocks (`fn_0085f150`,
/// `fn_0085f450`, `fn_0085f6c0`): `PlayerCharacter::GetAnimation`
/// (`00950a60`, `this` the player, one argument), the reference's virtual
/// slot `0x1E4` (its animation object), the animation's `0049a1c0(anim,
/// reference)` and `00491040(anim, pointer)`, the process object's slots
/// `0x3E4`, `0x3E8` and `0x3EC` (the last takes what the first two gave),
/// the controller's `0x28C` (true when it has a character controller to
/// load), `00926690(controller, mobile)`, two tests on a mobile object
/// (`00440d80`, `00440da0`, bytes) and `00560530(reference, value)`.
pub(crate) const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
pub(crate) const REFERENCE_SLOT_1E4: u32 = 0x1E4;
pub(crate) const ANIMATION_LOAD: u32 = 0x0049_a1c0;
pub(crate) const ANIMATION_ADJUST: u32 = 0x0049_1040;
pub(crate) const PROCESS_SLOT_3E4: u32 = 0x3E4;
pub(crate) const PROCESS_SLOT_3E8: u32 = 0x3E8;
pub(crate) const PROCESS_SLOT_3EC: u32 = 0x3EC;
pub(crate) const PROCESS_SLOT_28C: u32 = 0x28C;
pub(crate) const LOAD_CHAR_CONTROLLER: u32 = 0x0092_6690;
pub(crate) const MOBILE_TEST_A: u32 = 0x0044_0d80;
pub(crate) const MOBILE_TEST_B: u32 = 0x0044_0da0;
pub(crate) const REFERENCE_APPLY_HAVOK_BLOCK: u32 = 0x0056_0530;
/// `"SAVELOAD: (LoadCharControllers()) Mob %s %08X does not have a character
/// controller to load."`.
pub(crate) const MSG_NO_CHAR_CONTROLLER: u32 = 0x0108_19f0;

/// `fn_0085f750`: `008cd210(arrow)`.
pub(crate) const ARROW_PROJECTILE_FINISH: u32 = 0x008c_d210;
/// `008905f0(file)`: a test on an opened save file (`ReadSaveGameData` reads
/// the header only when it is true) and the virtual slot 0 of an object (its
/// scalar deleting destructor, with the delete flag).
pub(crate) const OPENED_FILE_TEST: u32 = 0x0089_05f0;
pub(crate) const SLOT_DESTRUCTOR: u32 = 0;
/// `fn_0085f850`: `978420(process lists, 0.0, 0)`, `453550(TES, 0.0)` and
/// `bhkWorld::DeactivateAllIslands` (`00c6a870`, `this` the Havok world).
pub(crate) const PROCESS_LISTS_FN_00978420: u32 = 0x0097_8420;
pub(crate) const TES_FN_00453550: u32 = 0x0045_3550;
pub(crate) const WORLD_DEACTIVATE_ALL_ISLANDS: u32 = 0x00c6_a870;

/// The list of saves (`fn_0085f900`): the Win32 imports `lstrcpyA`,
/// `lstrcatA`, `FindFirstFileA`, `FindNextFileA` and `FindClose`; the
/// pattern `"*.ess"`, `"%s%s%s"`, the size of a `SaveGameFile`, its vtable
/// and the comparator the list is sorted with.
pub(crate) const LSTRCPY_IMPORT: u32 = 0x00fd_f078;
pub(crate) const LSTRCAT_IMPORT: u32 = 0x00fd_f074;
pub(crate) const FIND_FIRST_FILE_IMPORT: u32 = 0x00fd_f070;
pub(crate) const FIND_NEXT_FILE_IMPORT: u32 = 0x00fd_f068;
pub(crate) const FIND_CLOSE_IMPORT: u32 = 0x00fd_f064;
pub(crate) const SAVE_FILE_PATTERN: u32 = 0x0108_1a4c;
pub(crate) const FORMAT_THREE_STRINGS: u32 = 0x0104_f208;
pub(crate) const SAVE_GAME_FILE_SIZE: u32 = 0x164;
pub(crate) const SAVE_GAME_FILE_VTABLE: u32 = 0x0108_1a5c;
pub(crate) const SAVE_GAME_FILE_COMPARATOR: u32 = 0x0085_fc90;
/// `BSFile::~BSFile` (`00aff240`).
pub(crate) const BSFILE_DESTRUCT: u32 = 0x00af_f240;
/// `memset(destination, value, size)` through the game's wrapper.
pub(crate) const MEMSET: u32 = 0x0040_3d30;
/// The size of a `WIN32_FIND_DATAA` and the offsets of the fields used.
pub(crate) const FIND_DATA_SIZE: u32 = 0x140;
pub(crate) const FIND_DATA_LAST_WRITE_TIME: u32 = 0x14;
pub(crate) const FIND_DATA_SIZE_HIGH: u32 = 0x1C;
pub(crate) const FIND_DATA_SIZE_LOW: u32 = 0x20;
pub(crate) const FIND_DATA_FILE_NAME: u32 = 0x2C;

/// Text objects of the game (the string is the dword at `+4`, which
/// `00403df0` returns; the address of the object is the constant). The exe
/// holds no text there (it is loaded at run time), so they are named by
/// where they are used: before the save number and before the level in a
/// save's description, as the displayed names of `"quicksave"` and
/// `"autosave"`, as the location text of a save without one, and as the two
/// button texts of the version warning.
pub(crate) const SAVE_NUMBER_PREFIX_TEXT: u32 = 0x011d_2028;
pub(crate) const SAVE_LEVEL_PREFIX_TEXT: u32 = 0x011d_4624;
pub(crate) const QUICKSAVE_DISPLAY_TEXT: u32 = 0x011d_327c;
pub(crate) const AUTOSAVE_DISPLAY_TEXT: u32 = 0x011d_3ed4;
pub(crate) const DEFAULT_LOCATION_TEXT: u32 = 0x011d_4fcc;
pub(crate) const MESSAGE_BOX_TEXT_A: u32 = 0x011d_3684;
pub(crate) const MESSAGE_BOX_TEXT_B: u32 = 0x011d_34f8;
/// `"quicksave"`.
pub(crate) const QUICKSAVE_NAME: u32 = 0x0107_fad4;
/// `" - "`, `","`, `"-"`, `"Playing Time"`, `"%s%s"` and `".ess"`.
pub(crate) const TEXT_SEPARATOR: u32 = 0x0108_1aa8;
pub(crate) const TEXT_COMMA: u32 = 0x0106_3cd8;
pub(crate) const TEXT_DASH: u32 = 0x0103_45e0;
pub(crate) const TEXT_PLAYING_TIME: u32 = 0x0108_1aec;
pub(crate) const FORMAT_TWO_STRINGS: u32 = 0x0101_996c;
/// `strncmp(a, b, count)` and `atol(text)`, and `strncpy`-like
/// `004add50(destination, source, count)`.
pub(crate) const STRNCMP: u32 = 0x00ec_8a19;
pub(crate) const ATOL: u32 = 0x00ec_a6d3;
pub(crate) const COPY_COUNTED: u32 = 0x004a_dd50;
/// `strpbrk(text, set)` (`00ecbf20`).
pub(crate) const STRPBRK: u32 = 0x00ec_bf20;

/// The formats of `ReadSaveGameData`: `"%s %i"`, `"%02i:%02i:%02i"` and
/// `"%d/%d/%02d %02d:%02d"`.
pub(crate) const FORMAT_LABEL_NUMBER: u32 = 0x0108_1ad4;
pub(crate) const FORMAT_PLAY_TIME: u32 = 0x0108_1ac4;
pub(crate) const FORMAT_DATE_TIME: u32 = 0x0108_1aac;
/// `"Save %i - %s - %s, Level %i, Playing Time %02i.%02i.%02i"`, the set of
/// characters a file name cannot hold (`\/:*<>?|"`) and `" #%d"`.
pub(crate) const FORMAT_SAVE_NAME: u32 = 0x0108_1b08;
pub(crate) const INVALID_FILE_NAME_CHARACTERS: u32 = 0x0108_1afc;
pub(crate) const FORMAT_NUMBER_SUFFIX: u32 = 0x0108_1b44;
/// `"%s%s%s.ess"` is `FORMAT_SAVE_PATH`.
/// Save-name pieces of `fn_00860ae0`: the game's `Calendar` (the address
/// `0x11DE7B8` is the object): `00867de0` the day, `00867da0` the hour (a
/// float in `ST0`); the player's play time in milliseconds (`00851cb0`),
/// the level (`0087f9f0`), the next save number (`00862370`, `this` the
/// game) and the player's location text (`00578870(player, out)`), held in
/// a string object (`004037b0` constructs it, `004037d0` destroys it,
/// `00559450` gives its text).
pub(crate) const CALENDAR: u32 = 0x011d_e7b8;
pub(crate) const CALENDAR_GET_DAY: u32 = 0x0086_7de0;
pub(crate) const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
pub(crate) const PLAYER_PLAY_TIME: u32 = 0x0085_1cb0;
pub(crate) const PLAYER_LEVEL: u32 = 0x0087_f9f0;
pub(crate) const NEXT_SAVE_NUMBER: u32 = 0x0086_2370;
pub(crate) const PLAYER_GET_LOCATION_TEXT: u32 = 0x0057_8870;
pub(crate) const STRING_OBJECT_CONSTRUCT: u32 = 0x0040_37b0;
pub(crate) const STRING_OBJECT_DESTRUCT: u32 = 0x0040_37d0;
pub(crate) const STRING_OBJECT_TEXT: u32 = 0x0055_9450;
/// The double `24.0` the day fraction is divided by (a constant of `.rdata`).
pub(crate) const HOURS_PER_DAY: u32 = 0x0103_56d8;

/// `LoadHeader` and `SaveHeader`: the global pointer to the text
/// `"FO3SAVEGAME"` (the dword is the address of the text), the prefix
/// `"CON "`, the message of the version warning, the second seek mode
/// (a global dword), `Interface::ShowMessageBox`-like `00704010(text, 1, 5,
/// button, button, 0)` (`cdecl`), `GetSystemTime` and `GetLocalTime`
/// imports, the screenshot taker `00878f60(&width, &height, 0)`, the
/// `NiPixelData` calls `0064dac0(image, 0, 0)` (the pixel data) and
/// `00861610` (the size of a level, this unit), the screen size getters
/// (`004dc200`, `004dc1f0`), `_ftol2` (`00ec62c0`, the value in `ST0`), the
/// label `"Save Game Header"` and the code of the version warning.
pub(crate) const SAVE_SIGNATURE_POINTER: u32 = 0x011a_2224;
pub(crate) const CON_PREFIX: u32 = 0x0108_1bbc;
pub(crate) const MSG_SAVE_VERSION_WARNING: u32 = 0x0108_1b50;
pub(crate) const SEEK_MODE_CURRENT: u32 = 0x010a_2484;
pub(crate) const SHOW_MESSAGE_BOX: u32 = 0x0070_4010;
pub(crate) const GET_SYSTEM_TIME_IMPORT: u32 = 0x00fd_f0e0;
pub(crate) const GET_LOCAL_TIME_IMPORT: u32 = 0x00fd_f0d8;
pub(crate) const TAKE_SAVE_SCREENSHOT: u32 = 0x0087_8f60;
pub(crate) const IMAGE_PIXELS: u32 = 0x0064_dac0;
pub(crate) const SCREEN_WIDTH: u32 = 0x004d_c200;
pub(crate) const SCREEN_HEIGHT: u32 = 0x004d_c1f0;
pub(crate) const FLOAT_TO_INTEGER: u32 = 0x00ec_62c0;
pub(crate) const LABEL_SAVE_GAME_HEADER: u32 = 0x0108_0694;
/// The version of the game's save format (the current minor version is
/// `0x7D`, the warning is shown below `0x13`).
pub(crate) const CURRENT_MINOR_VERSION: u32 = 0x7D;

/// `ReadSaveGameDataOLD`: the format of the screenshot's pixels (17 dwords
/// copied from `0x011AA2E8`), `operator new`-like allocator for a
/// `NiPixelData` (`00aa13e0`, size 0x74), `NiPixelData::NiPixelData`
/// (`00a7c190(this, width, width, format, 1, 1)`), the format preferences
/// constructor (`004f31d0`), the `NiFixedString` constructor (`00438170`),
/// its text (`0043b1b0`) and destructor (`004381b0`), the texture creator
/// (`00a5ff10(image, name text, preferences)`, `cdecl`), the name
/// `"SaveGameTexture"`, a reference-count release (`0040f6e0`), and the
/// file time reader `00b003c0(file, out)`.
pub(crate) const PIXEL_FORMAT_TEMPLATE: u32 = 0x011a_a2e8;
pub(crate) const PIXEL_DATA_ALLOCATE: u32 = 0x00aa_13e0;
pub(crate) const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
pub(crate) const FORMAT_PREFS_CONSTRUCT: u32 = 0x004f_31d0;
pub(crate) const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
pub(crate) const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
pub(crate) const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
pub(crate) const CREATE_TEXTURE_FROM_PIXELS: u32 = 0x00a5_ff10;
pub(crate) const SAVE_GAME_TEXTURE_NAME: u32 = 0x0108_1adc;
pub(crate) const REFERENCE_RELEASE: u32 = 0x0040_f6e0;
pub(crate) const FILE_GET_TIME: u32 = 0x00b0_03c0;

/// `BuildChangesString`: the texts of the process levels, `(verbose text,
/// format)`, and of `"Base(%i)"`.
pub(crate) const PROCESS_LEVEL_HIGH: (u32, u32) = (0x0108_193c, 0x0108_1930);
pub(crate) const PROCESS_LEVEL_MID_HIGH: (u32, u32) = (0x0108_1964, 0x0108_1954);
pub(crate) const PROCESS_LEVEL_MID_LOW: (u32, u32) = (0x0108_1990, 0x0108_1984);
pub(crate) const PROCESS_LEVEL_LOW: (u32, u32) = (0x0108_19b4, 0x0108_19ac);
pub(crate) const PROCESS_LEVEL_NONE: (u32, u32) = (0x0108_19d8, 0x0108_19cc);
pub(crate) const FORMAT_BASE: u32 = 0x0108_1924;
/// The size of the buffer `BuildChangesString` fills, and of its line
/// scratch (`sprintf_s` is given 0x32).
pub(crate) const CHANGES_BUFFER_SIZE: u32 = 500;
pub(crate) const LINE_SIZE: u32 = 0x32;
/// A line is padded with spaces to this many characters.
pub(crate) const LINE_WIDTH: u32 = 0x19;
/// `" "`.
pub(crate) const SPACE: u32 = 0x0102_0770;
/// `TESQuest`'s test of `fn_0085b320` (`0059e300`, `this` the quest).
pub(crate) const QUEST_HAS_SCRIPT_LOCALS: u32 = 0x0059_e300;

/// A line of the changes string: the flag bit that selects it, its text for
/// the verbose form (`"CHANGE_...\r\n"`) and its format (`"Name(%i)"`), as
/// addresses in `.rdata`.
pub(crate) struct ChangeLine {
    mask: u32,
    verbose: u32,
    format: u32,
}

// The lines of each form type. Lines whose flag mask is 0 in this build
// (the compiler folded the test to `flags & 0`) are not here.
pub(crate) const LINE_FORM_FLAGS: ChangeLine = ChangeLine {
    mask: 0x1,
    verbose: 0x0108_1910, // "CHANGE_FORM_FLAGS"
    format: 0x0108_1900,  // "Form Flags(%i)"
};
pub(crate) const LINE_ACTOR_BASE_DATA: ChangeLine = ChangeLine {
    mask: 0x2,
    verbose: 0x0108_1884, // "CHANGE_ACTOR_BASE_DATA"
    format: 0x0108_1874,  // "Base Data(%i)"
};
pub(crate) const LINE_ACTOR_SPELL_LIST: ChangeLine = ChangeLine {
    mask: 0x10,
    verbose: 0x0108_1854, // "CHANGE_ACTOR_BASE_SPELLLIST"
    format: 0x0108_1844,  // "Spell List(%i)"
};
pub(crate) const LINE_ACTOR_FULL_NAME: ChangeLine = ChangeLine {
    mask: 0x20,
    verbose: 0x0108_1798, // "CHANGE_ACTOR_BASE_FULLNAME"
    format: 0x0108_1788,  // "Full Name(%i)"
};
pub(crate) const LINE_NPC_SKILLS: ChangeLine = ChangeLine {
    mask: 0x200,
    verbose: 0x0108_1774, // "CHANGE_NPC_SKILLS"
    format: 0x0108_1768,  // "Skills(%i)"
};
pub(crate) const LINE_CREATURE_SKILLS: ChangeLine = ChangeLine {
    mask: 0x200,
    verbose: 0x0108_16fc, // "CHANGE_CREATURE_SKILLS"
    format: 0x0108_1768,  // "Skills(%i)"
};
pub(crate) const LINE_QUEST_FLAGS: ChangeLine = ChangeLine {
    mask: 0x2,
    verbose: 0x0108_16c4, // "CHANGE_QUEST_FLAGS"
    format: 0x0108_16b4,  // "Quest Flags(%i)"
};
/// Quest stages and the "said once" flag of a topic have no size: only the
/// verbose text (the format is not used).
pub(crate) const LINE_QUEST_STAGES: ChangeLine = ChangeLine {
    mask: 0x8000_0000,
    verbose: 0x0108_169c, // "CHANGE_QUEST_STAGES"
    format: 0,
};
pub(crate) const LINE_QUEST_SCRIPT: ChangeLine = ChangeLine {
    mask: 0x4000_0000,
    verbose: 0x0108_1684, // "CHANGE_QUEST_SCRIPT"
    format: 0x0108_1670,  // "Quest Script(%i)"
};
/// `"Quest Script(No longer has script locals)"`.
pub(crate) const FORMAT_QUEST_SCRIPT_GONE: u32 = 0x0108_1644;
pub(crate) const LINE_TOPIC_SAID_ONCE: ChangeLine = ChangeLine {
    mask: 0x8000_0000,
    verbose: 0x0108_162c, // "CHANGE_TOPIC_SAIDONCE"
    format: 0,
};
pub(crate) const LINE_PACKAGE_NEVER_RUN: ChangeLine = ChangeLine {
    mask: 0x8000_0000,
    verbose: 0x0108_1610, // "CHANGE_PACKAGE_NEVER_RUN"
    format: 0x0108_15fc,  // "Never Run Flag(%i)"
};
pub(crate) const LINE_PACKAGE_WAITING: ChangeLine = ChangeLine {
    mask: 0x4000_0000,
    verbose: 0x0108_15e0, // "CHANGE_PACKAGE_WAITING"
    format: 0x0108_15cc,  // "Waiting Flag(%i)"
};
pub(crate) const LINE_CELL_FLAGS: ChangeLine = ChangeLine {
    mask: 0x2,
    verbose: 0x0108_15b8, // "CHANGE_CELL_FLAGS"
    format: 0x0108_15a8,  // "Cell Flags(%i)"
};
pub(crate) const LINE_CELL_SEEN_DATA: ChangeLine = ChangeLine {
    mask: 0x8000_0000,
    verbose: 0x0108_1564, // "CHANGE_CELL_SEENDATA"
    format: 0x0108_1554,  // "Seen Data(%i)"
};
pub(crate) const LINE_CELL_DETACH_TIME: ChangeLine = ChangeLine {
    mask: 0x4000_0000,
    verbose: 0x0108_1538, // "CHANGE_CELL_DETACHTIME"
    format: 0x0108_1528,  // "Detach Time(%i)"
};
pub(crate) const LINE_CELL_OWNERSHIP: ChangeLine = ChangeLine {
    mask: 0x8,
    verbose: 0x0108_1510, // "CHANGE_CELL_OWNERSHIP"
    format: 0x0108_1500,  // "Ownership(%i)"
};
pub(crate) const LINE_CELL_FULL_NAME: ChangeLine = ChangeLine {
    mask: 0x4,
    verbose: 0x0108_14e8, // "CHANGE_CELL_FULLNAME"
    format: 0x0108_1788,  // "Full Name(%i)"
};
pub(crate) const LINE_FACTION_FLAGS: ChangeLine = ChangeLine {
    mask: 0x2,
    verbose: 0x0108_14d0, // "CHANGE_FACTION_FLAGS"
    format: 0x0108_14bc,  // "Faction Flags(%i)"
};
pub(crate) const LINE_FACTION_REACTIONS: ChangeLine = ChangeLine {
    mask: 0x4,
    verbose: 0x0108_14a0, // "CHANGE_FACTION_REACTIONS"
    format: 0x0108_1488,  // "Faction Reactions(%i)"
};
pub(crate) const LINE_BOOK_SKILL: ChangeLine = ChangeLine {
    mask: 0x20,
    verbose: 0x0108_146c, // "CHANGE_BOOK_TEACHES_SKILL"
    format: 0x0108_145c,  // "Book Skill(%i)"
};
/// The verbose text and format of the moved and Havok-moved lines (their
/// sizes are worked out where they are used).
pub(crate) const TEXT_REFR_MOVE: u32 = 0x0108_13e8; // "CHANGE_REFR_MOVE"
pub(crate) const FORMAT_REFR_MOVE: u32 = 0x0108_13dc; // "Moved(%i)"
pub(crate) const TEXT_REFR_HAVOK_MOVE: u32 = 0x0108_13c0; // "CHANGE_REFR_HAVOK_MOVE"
pub(crate) const FORMAT_REFR_HAVOK_MOVE: u32 = 0x0108_13b0; // "Havok Moved(%i)"
pub(crate) const LINE_REFR_ANIMATION: ChangeLine = ChangeLine {
    mask: 0x1000_0000,
    verbose: 0x0108_1314, // "CHANGE_REFR_ANIMATION"
    format: 0x0108_1304,  // "Animation(%i)"
};
pub(crate) const LINE_REFR_SCALE: ChangeLine = ChangeLine {
    mask: 0x10,
    verbose: 0x0108_12f0, // "CHANGE_REFR_SCALE"
    format: 0x0108_12e4,  // "Scale(%i)"
};
pub(crate) const LINE_REFR_INVENTORY: ChangeLine = ChangeLine {
    mask: 0x20,
    verbose: 0x0108_1214, // "CHANGE_REFR_INVENTORY"
    format: 0x0108_1204,  // "Inventory(%i)"
};
pub(crate) const LINE_ACTOR_DAMAGE_MODIFIERS: ChangeLine = ChangeLine {
    mask: 0x20_0000,
    verbose: 0x0108_1154, // "CHANGE_ACTOR_DAMAGE_MODIFIERS"
    format: 0x0108_113c,  // "Damage Modifiers(%i)"
};
pub(crate) const LINE_ACTOR_SCRIPT_MODIFIERS: ChangeLine = ChangeLine {
    mask: 0x80_0000,
    verbose: 0x0108_1118, // "CHANGE_ACTOR_PERMANENT_MODIFIERS"
    format: 0x0108_1100,  // "Script Modifiers(%i)"
};
pub(crate) const LINE_ACTOR_TEMP_MODIFIERS: ChangeLine = ChangeLine {
    mask: 0x10_0000,
    verbose: 0x0108_10e0, // "CHANGE_ACTOR_TEMP_MODIFIERS"
    format: 0x0108_10cc,  // "Temp Modifiers(%i)"
};
pub(crate) const LINE_ACTOR_DISPOSITION: ChangeLine = ChangeLine {
    mask: 0x8_0000,
    verbose: 0x0108_0fb4, // "CHANGE_ACTOR_DISPOSITION_MODIFIERS"
    format: 0x0108_0fa0,  // "Disp Modifiers(%i)"
};
pub(crate) const TEXT_ACTOR_LIFE_STATE: u32 = 0x0108_0f14; // "CHANGE_ACTOR_LIFESTATE"
pub(crate) const FORMAT_ACTOR_LIFE_STATE: u32 = 0x0108_0f04; // "Life State(%i)"
pub(crate) const LINE_OBJECT_LOCK: ChangeLine = ChangeLine {
    mask: 0x1000,
    verbose: 0x0108_0e20, // "CHANGE_OBJECT_EXTRA_LOCK"
    format: 0x0108_0e14,  // "Lock(%i)"
};
pub(crate) const LINE_OBJECT_EMPTY: ChangeLine = ChangeLine {
    mask: 0x20_0000,
    verbose: 0x0108_0d30, // "CHANGE_OBJECT_EMPTY"
    format: 0x0108_0d20,  // "Empty Flag(%i)"
};
pub(crate) const LINE_DOOR_TELEPORT: ChangeLine = ChangeLine {
    mask: 0x2_0000,
    verbose: 0x0108_0d00, // "CHANGE_DOOR_EXTRA_TELEPORT"
    format: 0x0108_0cf0,  // "Teleport(%i)"
};
pub(crate) const LINE_OPEN_STATE: ChangeLine = ChangeLine {
    mask: 0x80_0000,
    verbose: 0x0108_0cdc, // "CHANGE_OPEN_STATE"
    format: 0x0108_0ccc,  // "Open State(%i)"
};
pub(crate) const LINE_OPEN_DEFAULT_STATE: ChangeLine = ChangeLine {
    mask: 0x40_0000,
    verbose: 0x0108_0cb0, // "CHANGE_OPEN_DEFAULT_STATE"
    format: 0x0108_0c98,  // "Default Open State(%i)"
};

/// The form types `BuildChangesString` has lines for. The numbers are the
/// ones the code compares; the exe's own text names what each type's lines
/// are about (`CHANGE_NPC_...`, `CHANGE_CREATURE_...`, `CHANGE_QUEST_...`,
/// `CHANGE_TOPIC_...`, `CHANGE_PACKAGE_...`, `CHANGE_CELL_...`,
/// `CHANGE_FACTION_...`, `CHANGE_BOOK_...`).
pub(crate) const TYPE_NPC: u8 = 0x2A;
pub(crate) const TYPE_CREATURE: u8 = 0x2B;
pub(crate) const TYPE_QUEST: u8 = 0x47;
pub(crate) const TYPE_TOPIC: u8 = 0x46;
pub(crate) const TYPE_PACKAGE: u8 = 0x49;
pub(crate) const TYPE_CELL: u8 = 0x39;
pub(crate) const TYPE_FACTION: u8 = 0x08;
pub(crate) const TYPE_BOOK: u8 = 0x19;
/// The types of references that get lines (all the same kind of change
/// data), and the two among them with the actor modifier lines.
pub(crate) const REFERENCE_TYPES: [u8; 8] = [0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x69, 0x40];
pub(crate) const ACTOR_REFERENCE_TYPES: [u8; 2] = [0x3B, 0x3C];
/// The type of the base object of a reference whose teleport line is
/// shown (compared with `00401170(base)`).
pub(crate) const BASE_TYPE_DOOR: u32 = 0x1C;

// ---------------------------------------------------------------------------
// Helpers

/// The allocation scope object the game keeps on its stack (`00404eb0`);
/// returns the object. `line` is the source line of the scope.
pub(crate) fn scope_enter(e: &mut Engine, line: u32) -> Ptr {
    let scope = Ptr::new(e.mem.alloc(4));
    e.call(SCOPE_ENTER, &args![scope, 0x11u32, 1u32, SOURCE_FILE, line]);
    scope
}

/// The allocation scope object with another first argument (`0x31` in
/// `fn_00859a90`); see `scope_enter`.
pub(crate) fn scope_enter_kind(e: &mut Engine, kind: u32, line: u32) -> Ptr {
    let scope = Ptr::new(e.mem.alloc(4));
    e.call(SCOPE_ENTER, &args![scope, kind, 1u32, SOURCE_FILE, line]);
    scope
}

/// `__RTDynamicCast(object, 0, source, target, 0)`: the object seen as the
/// target type, or null.
pub(crate) fn dynamic_cast(e: &mut Engine, object: u32, source: u32, target: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .u32()
}

/// The game's `TESSaveLoadGame` through the global pointer.
pub(crate) fn game_singleton(e: &mut Engine) -> Ptr<TESSaveLoadGame> {
    Ptr::new(e.global(SAVE_LOAD_GAME))
}

/// Destroys the scope object (`00404ee0`) and frees its block.
pub(crate) fn scope_leave(e: &mut Engine, scope: Ptr) {
    e.call(SCOPE_LEAVE, &args![scope]);
    e.mem.free(scope.addr());
}

/// `operator delete(block)`.
pub(crate) fn delete(e: &mut Engine, block: u32) {
    e.call(OPERATOR_DELETE, &args![block]);
}

/// Whether the save/load singleton reports the operation unavailable
/// (`0047c850` on `*011de45c`); false in this build.
pub(crate) fn singleton_unavailable(e: &mut Engine) -> bool {
    let singleton: u32 = e.global(SAVE_LOAD_GAME);
    e.call(SAVE_LOAD_UNAVAILABLE, &args![singleton]).bool()
}

/// `0047c850` on the game the save routine was called on.
pub(crate) fn game_unavailable(e: &mut Engine, game: Ptr<TESSaveLoadGame>) -> bool {
    e.call(SAVE_LOAD_UNAVAILABLE, &args![game]).bool()
}

/// The key of a form: the dword at +0x0C.
pub(crate) fn form_key(e: &mut Engine, form: Ptr) -> u32 {
    e.call(FORM_ID, &args![form]).u32()
}

/// Deletes a `BSSimpleList` the maps own: `RemoveAll`, then the scalar
/// deleting destructor (flag 1) when the list exists.
pub(crate) fn destroy_list(e: &mut Engine, list: u32) {
    e.call(LIST_REMOVE_ALL, &args![list]);
    if list != 0 {
        e.call(LIST_SCALAR_DELETE, &args![list, 1u32]);
    }
}

/// Walks a map keyed by `unsigned int` with the game's iterator
/// (`GetFirstPos`, then `GetNext(&pos, &key, &value)` until the position is
/// 0), calling `visit(e, key, value)` for each entry. The position, key and
/// value cells are a 12-byte block, as in the game's stack frame.
pub(crate) fn for_each_entry(
    e: &mut Engine,
    map: Ptr,
    mut visit: impl FnMut(&mut Engine, u32, u32),
) {
    let cells = e.mem.alloc(12);
    let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    e.mem.set_u32(cells, first);
    while e.mem.u32(cells) != 0 {
        e.mem.set_u32(cells + 4, 0);
        e.mem.set_u32(cells + 8, 0);
        e.call(MAP_NEXT, &args![map, cells, cells + 4, cells + 8]);
        let (key, value) = (e.mem.u32(cells + 4), e.mem.u32(cells + 8));
        visit(e, key, value);
    }
    e.mem.free(cells);
}

// Translated from 00486a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, ChangeData *>::NewItem` (Xbox PDB): gets a
/// map entry from the allocator subobject at `this + 0x0C`.
pub fn ni_t_pointer_map_unsigned_int_change_data_p_new_item(
    e: &mut Engine,
    this: Ptr<ChangesMap>,
) -> Ptr {
    e.call(MAP_ALLOCATOR_NEW_ITEM, &args![this.byte_add(0x0C)])
        .ptr()
}

// Translated from 00666050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, ChangeData *>::DeleteItem` (Xbox PDB):
/// clears the entry's value (`+8`) and gives the entry back to the allocator
/// subobject at `this + 0x0C`.
pub fn ni_t_pointer_map_unsigned_int_change_data_p_delete_item(
    e: &mut Engine,
    this: Ptr<ChangesMap>,
    item: Ptr,
) {
    e.mem.set_u32(item.addr() + 8, 0);
    e.call(MAP_ALLOCATOR_DELETE_ITEM, &args![this.byte_add(0x0C), item]);
}

// Translated from 00854e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangeData::~ChangeData`: frees the form's buffer when there is one.
pub fn fn_00854e10(e: &mut Engine, this: Ptr<ChangeData>) {
    let buffer = e.get(this, ChangeData::pBuffer);
    if !buffer.is_null() {
        delete(e, buffer.addr());
    }
}

// Translated from 00854e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangeData::AddFlags`: sets bits in `iFlags` unless the data already
/// carries a buffer.
pub fn fn_00854e40(e: &mut Engine, this: Ptr<ChangeData>, flags: u32) {
    if e.get(this, ChangeData::pBuffer).is_null() {
        let current = e.get(this, ChangeData::iFlags);
        e.set(this, ChangeData::iFlags, current | flags);
    }
}

// Translated from 00854e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangeData::RemoveFlags`: clears bits in `iFlags` unless the data
/// already carries a buffer.
pub fn fn_00854e70(e: &mut Engine, this: Ptr<ChangeData>, flags: u32) {
    if e.get(this, ChangeData::pBuffer).is_null() {
        let current = e.get(this, ChangeData::iFlags);
        e.set(this, ChangeData::iFlags, !flags & current);
    }
}

// Translated from 00854ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangesMap::scalar deleting destructor` (Xbox PDB): destroys the map and
/// frees it when bit 0 of `flags` is set.
pub fn changes_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ChangesMap>,
    flags: u32,
) -> Ptr<ChangesMap> {
    fn_00854f00(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00854f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangesMap::~ChangesMap`: sets the map's vtable, deletes every
/// `ChangeData`, then runs the `NiTPointerMap` base destructor (`00863640`).
pub fn fn_00854f00(e: &mut Engine, this: Ptr<ChangesMap>) {
    e.mem.set_u32(this.addr(), CHANGES_MAP_VTABLE);
    changes_map_remove_all_changes(e, this);
    e.call(CHANGES_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00854f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangesMap::RemoveAllChanges` (Xbox PDB): deletes every `ChangeData`
/// (the destructor with flag 1) and empties the map.
pub fn changes_map_remove_all_changes(e: &mut Engine, this: Ptr<ChangesMap>) {
    for_each_entry(e, this.cast(), |e, _key, value| {
        if value != 0 {
            fn_00854fe0(e, Ptr::new(value), 1);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
}

// Translated from 00854fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChangeData` scalar deleting destructor: frees the buffer, and the
/// `ChangeData` itself when bit 0 of `flags` is set.
pub fn fn_00854fe0(e: &mut Engine, this: Ptr<ChangeData>, flags: u32) -> Ptr<ChangeData> {
    fn_00854e10(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00855010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `flags` to the `ChangeData` of `form`, creating it (flags 0, no
/// buffer) and putting it in the map when the form has none. Returns the
/// `ChangeData`.
pub fn fn_00855010(
    e: &mut Engine,
    this: Ptr<ChangesMap>,
    form: Ptr,
    flags: u32,
) -> Ptr<ChangeData> {
    let key = form_key(e, form);
    let cell = e.mem.alloc(4);
    let found = e.call(MAP_GET_AT, &args![this, key, cell]).bool();
    if !found {
        let scope = scope_enter(e, 0x104);
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let change_data = if block != 0 {
            e.call(SIMPLE_LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(cell, change_data);
        e.call(CHANGES_MAP_SET_AT, &args![this, key, change_data]);
        scope_leave(e, scope);
    }
    let change_data: Ptr<ChangeData> = Ptr::new(e.mem.u32(cell));
    e.mem.free(cell);
    fn_00854e40(e, change_data, flags);
    change_data
}

// Translated from 00855100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ChangeData` stored under `key`, or null.
pub fn fn_00855100(e: &mut Engine, this: Ptr<ChangesMap>, key: u32) -> Ptr<ChangeData> {
    let cell = e.mem.alloc(4);
    e.call(MAP_GET_AT, &args![this, key, cell]);
    let found = e.mem.u32(cell);
    e.mem.free(cell);
    Ptr::new(found)
}

// Translated from 00855130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ChangeData` of `form` (looked up by the form's key), or null.
pub fn fn_00855130(e: &mut Engine, this: Ptr<ChangesMap>, form: Ptr) -> Ptr<ChangeData> {
    let key = form_key(e, form);
    fn_00855100(e, this, key)
}

// Translated from 00855150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `flags` from the `ChangeData` of `form`; when none are left it is
/// removed from the map and deleted. False when saving is unavailable or the
/// form has no `ChangeData`.
pub fn fn_00855150(e: &mut Engine, this: Ptr<ChangesMap>, form: Ptr, flags: u32) -> bool {
    if singleton_unavailable(e) {
        return false;
    }
    let change_data = fn_00855130(e, this, form);
    if change_data.is_null() {
        return false;
    }
    fn_00854e70(e, change_data, flags);
    if e.call(READ_WORD, &args![change_data]).u32() == 0 {
        let key = form_key(e, form);
        e.call(MAP_REMOVE_AT, &args![this, key]);
        if !change_data.is_null() {
            fn_00854fe0(e, change_data, 1);
        }
    }
    true
}

// Translated from 008551f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00855220` for the key of `form`.
pub fn fn_008551f0(e: &mut Engine, this: Ptr<ChangesMap>, form: Ptr, force: u8) -> bool {
    let key = form_key(e, form);
    fn_00855220(e, this, key, force)
}

// Translated from 00855220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops the `ChangeData` stored under `key` when it has no buffer or when
/// `force` is set. False when saving is unavailable or there is none.
pub fn fn_00855220(e: &mut Engine, this: Ptr<ChangesMap>, key: u32, force: u8) -> bool {
    if singleton_unavailable(e) {
        return false;
    }
    let change_data = fn_00855100(e, this, key);
    if change_data.is_null() {
        return false;
    }
    if e.get(change_data, ChangeData::pBuffer).is_null() || force != 0 {
        e.call(MAP_REMOVE_AT, &args![this, key]);
        if !change_data.is_null() {
            fn_00854fe0(e, change_data, 1);
        }
    }
    true
}

// Translated from 008552b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InteriorCellNewReferencesMap::InteriorCellNewReferencesMap`: the pointer
/// map base with hash size 0x25, then this class's vtable.
pub fn fn_008552b0(
    e: &mut Engine,
    this: Ptr<InteriorCellNewReferencesMap>,
) -> Ptr<InteriorCellNewReferencesMap> {
    e.call(INTERIOR_MAP_BASE_CONSTRUCT, &args![this, HASH_SIZE]);
    e.mem.set_u32(this.addr(), INTERIOR_MAP_VTABLE);
    this
}

// Translated from 008552e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InteriorCellNewReferencesMap::scalar deleting destructor` (Xbox PDB).
pub fn interior_cell_new_references_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<InteriorCellNewReferencesMap>,
    flags: u32,
) -> Ptr<InteriorCellNewReferencesMap> {
    fn_00855310(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00855310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InteriorCellNewReferencesMap::~InteriorCellNewReferencesMap`: deletes
/// each cell's list of new references, empties the map, runs the base
/// destructor (`00863740`).
pub fn fn_00855310(e: &mut Engine, this: Ptr<InteriorCellNewReferencesMap>) {
    e.mem.set_u32(this.addr(), INTERIOR_MAP_VTABLE);
    for_each_entry(e, this.cast(), |e, _key, list| {
        if list != 0 {
            destroy_list(e, list);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(INTERIOR_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 008553e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExteriorCellNewReferencesMap::ExteriorCellNewReferencesMap`.
pub fn fn_008553e0(
    e: &mut Engine,
    this: Ptr<ExteriorCellNewReferencesMap>,
) -> Ptr<ExteriorCellNewReferencesMap> {
    e.call(EXTERIOR_MAP_BASE_CONSTRUCT, &args![this, HASH_SIZE]);
    e.mem.set_u32(this.addr(), EXTERIOR_MAP_VTABLE);
    this
}

// Translated from 00855410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExteriorCellNewReferencesMap::scalar deleting destructor` (Xbox PDB).
pub fn exterior_cell_new_references_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExteriorCellNewReferencesMap>,
    flags: u32,
) -> Ptr<ExteriorCellNewReferencesMap> {
    fn_00855440(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00855440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExteriorCellNewReferencesMap::~ExteriorCellNewReferencesMap`: for each
/// cell, frees every `ExteriorCellReferenceData` in its list, then the list;
/// empties the map and runs the base destructor (`00863860`).
pub fn fn_00855440(e: &mut Engine, this: Ptr<ExteriorCellNewReferencesMap>) {
    e.mem.set_u32(this.addr(), EXTERIOR_MAP_VTABLE);
    for_each_entry(e, this.cast(), |e, _key, list| {
        if list != 0 {
            let mut node = list;
            while node != 0 {
                let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let item = e.mem.u32(item_slot);
                if item != 0 {
                    delete(e, item);
                }
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
            destroy_list(e, list);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(EXTERIOR_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00855550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NumericIDBufferMap::NumericIDBufferMap`.
pub fn fn_00855550(e: &mut Engine, this: Ptr<NumericIDBufferMap>) -> Ptr<NumericIDBufferMap> {
    e.call(NUMERIC_ID_MAP_BASE_CONSTRUCT, &args![this, HASH_SIZE]);
    e.mem.set_u32(this.addr(), NUMERIC_ID_MAP_VTABLE);
    this
}

// Translated from 00855580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NumericIDBufferMap::scalar deleting destructor` (Xbox PDB).
pub fn numeric_id_buffer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NumericIDBufferMap>,
    flags: u32,
) -> Ptr<NumericIDBufferMap> {
    fn_008555b0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 008555b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NumericIDBufferMap::~NumericIDBufferMap`: frees every buffer, empties
/// the map and runs the base destructor (`00863960`).
pub fn fn_008555b0(e: &mut Engine, this: Ptr<NumericIDBufferMap>) {
    e.mem.set_u32(this.addr(), NUMERIC_ID_MAP_VTABLE);
    for_each_entry(e, this.cast(), |e, _key, buffer| {
        if buffer != 0 {
            delete(e, buffer);
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(NUMERIC_ID_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00855660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats::SaveStats`: makes the per-type map (hash size 0x25) and the
/// empty list of extra stats.
pub fn fn_00855660(e: &mut Engine, this: Ptr<SaveStats>) -> Ptr<SaveStats> {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let stats_map = if block != 0 {
        e.call(STATS_MAP_CONSTRUCT, &args![block, HASH_SIZE]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, SaveStats::pStatsMap, stats_map);
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let extra_stats = if block != 0 {
        e.call(SIMPLE_LIST_CONSTRUCT, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, SaveStats::pExtraStats, extra_stats);
    this
}

// Translated from 00855730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats::~SaveStats`: frees the headers of every type's list and the
/// lists, deletes the map through its destructor (vtable slot 0, flag 1),
/// then frees every extra stat (description and record) and its list.
pub fn fn_00855730(e: &mut Engine, this: Ptr<SaveStats>) {
    let stats_map = e.get(this, SaveStats::pStatsMap);
    // `00863bc0(&pos, &type, &list)`: three cells, the type a byte.
    let cells = e.mem.alloc(12);
    let first = e.call(MAP_FIRST_POSITION, &args![stats_map]).u32();
    e.mem.set_u32(cells, first);
    while e.mem.u32(cells) != 0 {
        e.call(
            BYTE_MAP_NEXT,
            &args![stats_map, cells, cells + 4, cells + 8],
        );
        let list = e.mem.u32(cells + 8);
        if list != 0 {
            let mut node = list;
            while node != 0 {
                let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
                let header = e.mem.u32(item_slot);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                if header != 0 {
                    delete(e, header);
                }
            }
            destroy_list(e, list);
        }
    }
    e.mem.free(cells);
    let stats_map = e.get(this, SaveStats::pStatsMap);
    if !stats_map.is_null() {
        e.vcall(stats_map.addr(), 0, &args![1u32]);
    }
    let extra_stats = e.get(this, SaveStats::pExtraStats);
    if !extra_stats.is_null() {
        let mut node = extra_stats.addr();
        while node != 0 {
            let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let stat: Ptr<ExtraStat> = Ptr::new(e.mem.u32(item_slot));
            if !stat.is_null() {
                let description = e.get(stat, ExtraStat::pDescription);
                delete(e, description.addr());
                delete(e, stat.addr());
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        let extra_stats = e.get(this, SaveStats::pExtraStats);
        destroy_list(e, extra_stats.addr());
    }
}

// Translated from 008558a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats::AddExtraStat` (Xbox PDB): adds a record of `size` bytes with a
/// copy of `description` to the head of the extra stats list.
pub fn save_stats_add_extra_stat(
    e: &mut Engine,
    this: Ptr<SaveStats>,
    size: u32,
    description: Ptr,
) {
    let scope = scope_enter(e, 0x267);
    let stat: Ptr<ExtraStat> = e.call(OPERATOR_NEW, &args![8u32]).ptr();
    e.set(stat, ExtraStat::iSize, size);
    let length = e.call(STRLEN, &args![description]).u32().wrapping_add(1);
    let copy = e.call(OPERATOR_NEW, &args![length]).ptr();
    e.set(stat, ExtraStat::pDescription, copy);
    e.call(STRING_COPY, &args![copy, length, description]);
    let cell = e.mem.alloc(4);
    e.mem.set_u32(cell, stat.addr());
    let extra_stats = e.get(this, SaveStats::pExtraStats);
    e.call(LIST_ADD_HEAD, &args![extra_stats, cell]);
    e.mem.free(cell);
    scope_leave(e, scope);
}

// Translated from 00855970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records a saved form's header with its saved size: builds a
/// `LoadFormHeader` from the form id, type, flags and version of `header`
/// and `size`, and adds it to the stats (`fn_00855a20`).
pub fn fn_00855970(e: &mut Engine, this: Ptr<SaveStats>, header: Ptr<SaveFormHeader>, size: u16) {
    let scope = scope_enter(e, 0x276);
    let copy: Ptr<LoadFormHeader> = Ptr::new(e.mem.alloc(LoadFormHeader::SIZE));
    e.call(LIST_NODE_ITEM, &args![copy]);
    let flags = e.get(header, SaveFormHeader::iFlags);
    let form_id = e.get(header, SaveFormHeader::iFormID);
    let form_type = e.get(header, SaveFormHeader::cFormType);
    let version = e.get(header, SaveFormHeader::cVersion);
    e.set(copy, LoadFormHeader::iFlags, flags);
    e.set(copy, LoadFormHeader::iFormID, form_id);
    e.set(copy, LoadFormHeader::cFormType, form_type);
    e.set(copy, LoadFormHeader::cVersion, version);
    e.set(copy, LoadFormHeader::iSize, size);
    fn_00855a20(e, this, copy);
    e.mem.free(copy.addr());
    scope_leave(e, scope);
}

// Translated from 00855a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a heap copy of `header` to the list of its form type in the stats
/// map (creating the list when the type has none), kept sorted by
/// `fn_00855b60` (largest first).
pub fn fn_00855a20(e: &mut Engine, this: Ptr<SaveStats>, header: Ptr<LoadFormHeader>) {
    let scope = scope_enter(e, 0x286);
    let block = e.call(OPERATOR_NEW, &args![0xCu32]).u32();
    let item: Ptr<LoadFormHeader> = if block != 0 {
        e.call(LIST_NODE_ITEM, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    for word in 0..3 {
        let value = e.mem.u32(header.addr() + 4 * word);
        e.mem.set_u32(item.addr() + 4 * word, value);
    }
    let form_type = e.get(item, LoadFormHeader::cFormType);
    let stats_map = e.get(this, SaveStats::pStatsMap);
    let list_cell = e.mem.alloc(4);
    let found = e
        .call(BYTE_MAP_GET_AT, &args![stats_map, form_type, list_cell])
        .bool();
    if !found {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(SIMPLE_LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(list_cell, list);
        e.call(BYTE_MAP_SET_AT, &args![stats_map, form_type, list]);
    }
    let list = e.mem.u32(list_cell);
    e.mem.free(list_cell);
    e.call(LIST_INSERT, &args![list, item, STATS_COMPARATOR]);
    scope_leave(e, scope);
}

// Translated from 00855b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The comparison of two `LoadFormHeader`s the stats lists are sorted by:
/// -1 when `a` is larger than `b`, 1 when smaller, 0 when equal (so the list
/// runs from the largest to the smallest).
pub fn fn_00855b60(e: &mut Engine, a: Ptr<LoadFormHeader>, b: Ptr<LoadFormHeader>) -> i32 {
    let size_a = e.get(a, LoadFormHeader::iSize) as i32;
    let size_b = e.get(b, LoadFormHeader::iSize) as i32;
    if size_a > size_b {
        -1
    } else {
        (size_a < size_b) as i32
    }
}

// Translated from 00855ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats::PrintStats` (Xbox PDB): writes the statistics to the text
/// file `<path><extension>` (an existing file is deleted first): the table
/// header, then for each form type a section listing every saved form of
/// that type (form id, size, flags, version, name and the string of
/// changes) followed by that type's totals, then the extra stats and the
/// grand totals. A form that is not loaded prints "NOT LOADED"; otherwise
/// its name is the location name of a map marker or the name of a reference
/// when it has one, else its description (form vtable slot `0x130`).
/// Nothing is written when the file cannot be opened.
///
/// The `strlen` the game takes of the type heading is never used and is not
/// translated; the compiler's stack-cookie check and the exception frame are
/// not translated.
pub fn save_stats_print_stats(e: &mut Engine, this: Ptr<SaveStats>, path: Ptr) {
    const PATH: u32 = 0x000; // char[0x104]
    const FILE: u32 = 0x108; // the file object, 0x20 bytes
    const TOTAL: u32 = 0x128; // Stats
    const LINE: u32 = 0x138; // char[0x208]
    const CHANGES: u32 = 0x340; // char[0x1f8]
    const TYPE_NAME: u32 = 0x538; // char[10]
    const TYPE_STATS: u32 = 0x548; // Stats
    const CURSOR: u32 = 0x558; // position, type byte, list head
    const FRAME: u32 = 0x570;

    let frame = e.mem.alloc(FRAME);
    let (path_buffer, file, total) = (frame + PATH, frame + FILE, frame + TOTAL);
    let (line, changes, type_name) = (frame + LINE, frame + CHANGES, frame + TYPE_NAME);
    let (type_stats, cursor) = (frame + TYPE_STATS, frame + CURSOR);
    let singleton: u32 = e.global(SAVE_LOAD_GAME);

    e.call(STRING_COPY, &args![path_buffer, 0x104u32, path]);
    e.call(STRING_CAT, &args![path_buffer, 0x104u32, 0x0103_9788u32]);
    if e.call(FILE_EXISTS, &args![path_buffer, 0u32, 0u32, -1i32])
        .u32()
        != 0
    {
        e.call(FILE_DELETE, &args![path_buffer]);
    }
    e.call(
        FILE_OBJECT_CONSTRUCT,
        &args![file, path_buffer, 1u32, 2u32, 0u32],
    );
    if e.call(READ_WORD, &args![file]).u32() != 0 {
        e.call(FILE_OBJECT_DESTRUCT, &args![file]);
        e.mem.free(frame);
        return;
    }

    fn_008562c0(e, Ptr::new(total));
    e.call(
        FORMAT,
        &args![
            line,
            0x208u32,
            0x0108_0670u32,
            0x0108_0414u32,
            0x0108_041cu32,
            0x0104_4aecu32,
            0x0106_3d04u32,
            0x0105_ac4cu32,
            0x0108_0428u32
        ],
    );
    fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));

    let stats_map = e.get(this, SaveStats::pStatsMap);
    let first = e.call(MAP_FIRST_POSITION, &args![stats_map]).u32();
    e.mem.set_u32(cursor, first);
    while e.mem.u32(cursor) != 0 {
        e.call(
            BYTE_MAP_NEXT,
            &args![stats_map, cursor, cursor + 4, cursor + 8],
        );
        let form_type = e.mem.u8(cursor + 4);
        let list = e.mem.u32(cursor + 8);

        // The heading: "Form" for type 0, "Buffer" for 0x79, else the type's name.
        if form_type == 0 {
            e.call(FORMAT, &args![type_name, 10u32, 0x0104_469cu32]);
        } else if form_type == 0x79 {
            e.call(FORMAT, &args![type_name, 10u32, 0x0108_0668u32]);
        } else {
            let name = e.mem.u32(FORM_TYPE_NAME_TABLE + form_type as u32 * 12);
            e.call(FORMAT, &args![type_name, 10u32, 0x0101_9f08u32, name]);
        }
        e.call(FORMAT, &args![line, 0x208u32, 0x0108_03e4u32, type_name]);
        fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));
        fn_008562c0(e, Ptr::new(type_stats));

        let mut node = list;
        while node != 0 {
            let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let header: Ptr<LoadFormHeader> = Ptr::new(e.mem.u32(item_slot));
            if !header.is_null() {
                let form_id = e.get(header, LoadFormHeader::iFormID);
                let size = e.get(header, LoadFormHeader::iSize);
                let flags = e.get(header, LoadFormHeader::iFlags);
                let version = e.get(header, LoadFormHeader::cVersion);
                let header_type = e.get(header, LoadFormHeader::cFormType);
                let form = e.call(LOOKUP_FORM, &args![form_id]).u32();

                let stats: Ptr<Stats> = Ptr::new(type_stats);
                if size > e.get(stats, Stats::iMaxSize) {
                    e.set(stats, Stats::iMaxSize, size);
                }
                if size < e.get(stats, Stats::iMinSize) {
                    e.set(stats, Stats::iMinSize, size);
                }
                let total_size = e.get(stats, Stats::iTotalSize);
                e.set(
                    stats,
                    Stats::iTotalSize,
                    total_size.wrapping_add(size as i32),
                );
                let count = e.get(stats, Stats::iNum);
                e.set(stats, Stats::iNum, count.wrapping_add(1));

                e.call(
                    BUILD_CHANGES_STRING,
                    &[singleton, changes, form, flags, header_type as u32, 0],
                );
                let map_marker = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0i32, 0x0118_3028u32, 0x0118_3158u32, 0i32],
                    )
                    .u32();
                let reference = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0i32, 0x0118_3028u32, 0x0118_41ccu32, 0i32],
                    )
                    .u32();
                let text = if form != 0 {
                    let mut name = 0u32;
                    if reference != 0 {
                        name = e.call(REFERENCE_GET_NAME, &args![reference]).u32();
                    }
                    if map_marker != 0 && (name == 0 || is_empty_string(e, name)) {
                        name = e
                            .call(MAP_MARKER_GET_LOCATION_NAME, &args![map_marker])
                            .u32();
                    }
                    if name == 0 || is_empty_string(e, name) {
                        name = e.vcall(form, FORM_GET_DESCRIPTION, &args![]).u32();
                    }
                    name
                } else {
                    0x0108_063c // "NOT LOADED"
                };
                e.call(
                    FORMAT,
                    &args![
                        line,
                        0x208u32,
                        0x0108_0648u32,
                        form_id,
                        size as u32,
                        flags,
                        version as u32,
                        text,
                        changes
                    ],
                );
                fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }

        // This type's totals.
        let stats: Ptr<Stats> = Ptr::new(type_stats);
        let (count, total_size) = (e.get(stats, Stats::iNum), e.get(stats, Stats::iTotalSize));
        let (min_size, max_size) = (e.get(stats, Stats::iMinSize), e.get(stats, Stats::iMaxSize));
        let average = total_size as f64 / count as f64;
        e.call(
            FORMAT,
            &args![
                line,
                0x208u32,
                0x0108_05d8u32,
                type_name,
                count,
                type_name,
                total_size,
                type_name,
                min_size as u32,
                type_name,
                max_size as u32,
                type_name,
                average
            ],
        );
        fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));

        let grand: Ptr<Stats> = Ptr::new(total);
        if max_size > e.get(grand, Stats::iMaxSize) {
            e.set(grand, Stats::iMaxSize, max_size);
        }
        if min_size < e.get(grand, Stats::iMinSize) {
            e.set(grand, Stats::iMinSize, min_size);
        }
        let grand_size = e.get(grand, Stats::iTotalSize);
        e.set(
            grand,
            Stats::iTotalSize,
            grand_size.wrapping_add(total_size),
        );
        let grand_count = e.get(grand, Stats::iNum);
        e.set(grand, Stats::iNum, grand_count.wrapping_add(count));
    }

    fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(0x0108_05c4));
    let mut node = e.get(this, SaveStats::pExtraStats).addr();
    while node != 0 {
        let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let stat: Ptr<ExtraStat> = Ptr::new(e.mem.u32(item_slot));
        if !stat.is_null() {
            let size = e.get(stat, ExtraStat::iSize);
            let description = e.get(stat, ExtraStat::pDescription);
            e.call(
                FORMAT,
                &args![line, 0x208u32, 0x0108_0254u32, size, description],
            );
            fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));
            let grand: Ptr<Stats> = Ptr::new(total);
            let grand_size = e.get(grand, Stats::iTotalSize);
            e.set(
                grand,
                Stats::iTotalSize,
                grand_size.wrapping_add(size as i32),
            );
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }

    let grand: Ptr<Stats> = Ptr::new(total);
    let (count, total_size) = (e.get(grand, Stats::iNum), e.get(grand, Stats::iTotalSize));
    let (min_size, max_size) = (e.get(grand, Stats::iMinSize), e.get(grand, Stats::iMaxSize));
    let average = total_size as f64 / count as f64;
    e.call(
        FORMAT,
        &args![
            line,
            0x208u32,
            0x0108_0570u32,
            count,
            total_size,
            min_size as u32,
            max_size as u32,
            average
        ],
    );
    fn_00856300(e, this.cast(), Ptr::new(file), Ptr::new(line));
    e.call(FILE_OBJECT_DESTRUCT, &args![file]);
    e.mem.free(frame);
}

/// `strcmp(name, "") == 0` against the game's empty string (`01011584`).
pub(crate) fn is_empty_string(e: &mut Engine, name: u32) -> bool {
    e.call(STRING_COMPARE, &args![name, 0x0101_1584u32]).i32() == 0
}

// Translated from 008562c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats::Stats::Stats`: no entries, total 0, minimum 0xFFFF, maximum 0.
pub fn fn_008562c0(e: &mut Engine, this: Ptr<Stats>) -> Ptr<Stats> {
    e.set(this, Stats::iMaxSize, 0);
    e.set(this, Stats::iTotalSize, 0);
    e.set(this, Stats::iNum, 0);
    e.set(this, Stats::iMinSize, 0xFFFF);
    this
}

// Translated from 00856300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the text (without its terminator) to `file` through
/// `BSSystemFile::DoWrite`; true when the write reported no error. `this` is
/// not used.
pub fn fn_00856300(e: &mut Engine, _this: Ptr, file: Ptr, text: Ptr) -> bool {
    let length = e.call(STRLEN, &args![text]).u32();
    let scratch = e.mem.alloc(16);
    let error = e
        .call(
            SYSTEM_FILE_DO_WRITE,
            &args![file, text, length, 0u32, scratch],
        )
        .u32();
    e.mem.free(scratch);
    error == 0
}

// Translated from 00856c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::RemoveChanges` (Xbox PDB): unless the form is flagged
/// deleted (bit 0x4000 of its flags), drops its `ChangeData`
/// (`fn_008551f0`).
pub fn tes_save_load_game_remove_changes(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form: Ptr,
    force: u8,
) {
    if !e.call(FORM_IS_DELETED, &args![form]).bool() {
        let changes = e.get(this, TESSaveLoadGame::m_pChanges);
        fn_008551f0(e, changes, form, force);
    }
}

// Translated from 00856ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the game (the routine behind "sCantSaveNow"): refuses, with the
/// sad-Vault-Boy message, unless saving is allowed or the save is an
/// autosave; otherwise takes the save lock, opens the file (`file` is an
/// existing stream or null, `name` the save's name), writes the header, the
/// plugin list and the global data, then for every `ChangeData` of the
/// changes map the 10-byte form header, the size and the changes (the
/// pre-built buffer when the data has one, else the form's own initial data
/// and changes), the final data and the numeric id arrays, patches the
/// positions written at the start, prints the statistics when
/// `collect_stats` asked for them, and closes the file. True when it saved.
pub fn fn_00856ca0(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    name: Ptr,
    collect_stats: bool,
) -> bool {
    // The frame cells the game keeps on its stack: the form header, the size
    // words, the reserved word, the end position and the form count.
    const HEADER: u32 = 0x00;
    const BUFFER_SIZE: u32 = 0x10; // the 4 bytes `READ_BYTES` fills
    const FORM_SIZE: u32 = 0x14; // u16
    const RESERVED: u32 = 0x18;
    const END_POSITION: u32 = 0x1C;
    const COUNT: u32 = 0x20;
    const FRAME: u32 = 0x24;

    let scope = scope_enter(e, 0x466);
    let allowed = game_unavailable(e, this)
        || e.call(GET_SAVING_ALLOWED, &args![this]).bool()
        || (!name.is_null() && e.call(STRING_COMPARE, &args![name, AUTOSAVE_NAME]).i32() == 0);
    if !allowed {
        let queue = e
            .call(GET_MESSAGE_QUEUE, &args![MESSAGE_QUEUE_OBJECT])
            .u32();
        let time: f32 = e.global(MESSAGE_TIME);
        e.call(
            SHOW_MESSAGE,
            &args![queue, 0u32, SAD_ICON, 0u32, time, 0u32],
        );
        scope_leave(e, scope);
        return false;
    }

    let lock: u32 = e.global(SAVE_LOCK);
    e.call(SAVE_LOCK_ENTER, &args![lock]);
    e.call(SAVE_PREPARE_A, &args![this]);
    let mut stream = Ptr::NULL;
    if !game_unavailable(e, this) {
        stream = e.call(OPEN_SAVE_FILE, &args![this, file, name, 0u32]).ptr();
    }
    if collect_stats {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let stats = if block != 0 {
            fn_00855660(e, Ptr::new(block))
        } else {
            Ptr::NULL
        };
        e.set(this, TESSaveLoadGame::m_pSaveLoadStats, stats);
    }
    e.call(SAVE_PREPARE_B, &args![this]);
    e.call(SAVE_PREPARE_C, &args![this]);
    e.call(SAVE_HEADER, &args![this, stream, name]);
    e.call(SAVE_PLUGIN_LIST, &args![this, stream]);

    let frame = e.mem.alloc(FRAME);
    let header: Ptr<SaveFormHeader> = Ptr::new(frame + HEADER);
    let mut start_position = 0u32;
    if !game_unavailable(e, this) {
        start_position = e.call(FILE_POSITION, &args![stream]).u32();
    }
    e.mem.set_u32(frame + RESERVED, 0);
    e.call(WRITE_BYTES, &args![this, stream, frame + RESERVED, 4u32]);
    e.call(WRITE_BYTES, &args![this, stream, frame + RESERVED, 4u32]);
    e.call(SAVE_GLOBAL_DATA, &args![this, stream]);
    e.mem.set_u32(frame + COUNT, 0);

    let singleton: u32 = e.global(SAVE_LOAD_GAME);
    let changes = e.get(this, TESSaveLoadGame::m_pChanges);
    for_each_entry(e, changes.cast(), |e, form_id, change_data| {
        if form_id == 0 || change_data == 0 {
            return;
        }
        let buffer = e.call(LIST_NODE_NEXT, &args![change_data]).u32();
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        let flags = e.call(READ_WORD, &args![change_data]).u32();
        e.call(LIST_NODE_ITEM, &args![header]);
        e.set(header, SaveFormHeader::iFormID, form_id);
        e.set(header, SaveFormHeader::iFlags, flags);
        let version = e.call(CURRENT_VERSION, &args![this]).u8();
        e.set(header, SaveFormHeader::cVersion, version);

        if buffer != 0 {
            // The data has its own buffer: its first four bytes are the
            // size, the type and the version.
            e.call(READ_BYTES, &args![singleton, frame + BUFFER_SIZE, 4u32]);
            let form_type = e.mem.u8(frame + BUFFER_SIZE + 2);
            let version = e.mem.u8(frame + BUFFER_SIZE + 3);
            e.set(header, SaveFormHeader::cFormType, form_type);
            e.set(header, SaveFormHeader::cVersion, version);
            e.call(WRITE_BYTES, &args![singleton, stream, header, 10u32]);
            let count = e.mem.u32(frame + COUNT);
            e.mem.set_u32(frame + COUNT, count.wrapping_add(1));
            e.call(WRITE_BYTES, &args![this, stream, frame + BUFFER_SIZE, 2u32]);
            let size = e.mem.u16(frame + BUFFER_SIZE);
            if size != 0 {
                let saved = e.get(this, TESSaveLoadGame::m_pBuffer);
                e.call(WRITE_BYTES, &args![this, stream, saved, size as u32]);
            }
            let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
            if !stats.is_null() {
                fn_00855970(e, stats, header, size);
            }
            e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
        } else {
            let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
            if form == 0 {
                return;
            }
            let form_type = e.call(FORM_TYPE, &args![form]).u8();
            e.set(header, SaveFormHeader::cFormType, form_type);
            let flags = e.get(header, SaveFormHeader::iFlags);
            let flags = e.call(CHECK_FLAGS, &args![this, form, flags]).u32();
            e.set(header, SaveFormHeader::iFlags, flags);
            e.call(WRITE_BYTES, &args![singleton, stream, header, 10u32]);
            let count = e.mem.u32(frame + COUNT);
            e.mem.set_u32(frame + COUNT, count.wrapping_add(1));
            fn_00857230(e, this, header.cast());
            let mut size = e.vcall(form, FORM_GET_CHANGES_SIZE, &args![flags]).u16();
            let initial = e
                .call(GET_INITIAL_DATA_SAVE_SIZE, &args![this, form, flags])
                .u16();
            size = size.wrapping_add(initial);
            e.mem.set_u16(frame + FORM_SIZE, size);
            e.call(WRITE_BYTES, &args![this, stream, frame + FORM_SIZE, 2u32]);
            if size != 0 {
                let save_buffer = e.call(CREATE_BUFFER, &args![this, size as u32]).u32();
                e.call(SAVE_INITIAL_DATA, &args![this, form, flags]);
                e.vcall(form, FORM_SAVE_CHANGES, &args![flags]);
                e.call(WRITE_FILE, &args![this, stream, save_buffer, size as u32]);
                e.call(FREE_BUFFER, &args![this, save_buffer]);
            }
            fn_00857230(e, this, Ptr::NULL);
            let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
            if !stats.is_null() {
                fn_00855970(e, stats, header, size);
            }
        }
    });

    e.call(SAVE_FINAL_DATA, &args![this, stream]);
    e.mem.set_u32(frame + END_POSITION, 0);
    if !game_unavailable(e, this) {
        let position = e.call(FILE_POSITION, &args![stream]).u32();
        e.mem.set_u32(frame + END_POSITION, position);
    }
    e.call(SAVE_NUMERIC_ID_ARRAYS, &args![this, stream]);
    if !game_unavailable(e, this) {
        let mode: u32 = e.global(SEEK_MODE);
        e.vcall(stream.addr(), FILE_SEEK, &args![start_position, mode]);
        e.call(
            WRITE_BYTES,
            &args![this, stream, frame + END_POSITION, 4u32],
        );
        e.call(WRITE_BYTES, &args![this, stream, frame + COUNT, 4u32]);
    }
    e.mem.free(frame);

    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        let stream_name = e.vcall(stream.addr(), FILE_GET_NAME, &args![]).u32();
        save_stats_print_stats(e, stats, Ptr::new(stream_name));
        let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
        if !stats.is_null() {
            fn_00857250(e, stats, 1);
        }
        e.set(this, TESSaveLoadGame::m_pSaveLoadStats, Ptr::NULL);
    }
    if !game_unavailable(e, this) {
        fn_00857210(e, stream);
        e.call(SAVE_CLOSE_A, &args![this, stream]);
        e.call(SAVE_CLOSE_B, &args![this, stream, 0u32]);
    }
    e.call(SAVE_LOCK_LEAVE, &args![lock]);
    scope_leave(e, scope);
    true
}

// Translated from 00857210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends the save's use of the file through `00aa15a0`.
pub fn fn_00857210(e: &mut Engine, this: Ptr) {
    e.call(FILE_FLUSH, &args![this]);
}

// Translated from 00857230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `m_pCurrentlySavingFormHeader`.
pub fn fn_00857230(e: &mut Engine, this: Ptr<TESSaveLoadGame>, header: Ptr) {
    e.set(this, TESSaveLoadGame::m_pCurrentlySavingFormHeader, header);
}

// Translated from 00857250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveStats` scalar deleting destructor: runs `fn_00855730`, frees the
/// object when bit 0 of `flags` is set.
pub fn fn_00857250(e: &mut Engine, this: Ptr<SaveStats>, flags: u32) -> Ptr<SaveStats> {
    fn_00855730(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00857280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FormAndFlags::FormAndFlags(form, flags, oldFlags, version)`.
pub fn fn_00857280(
    e: &mut Engine,
    this: Ptr<FormAndFlags>,
    form: Ptr,
    flags: u32,
    old_flags: u32,
    version: u8,
) -> Ptr<FormAndFlags> {
    e.set(this, FormAndFlags::pForm, form);
    e.set(this, FormAndFlags::iFlags, flags);
    e.set(this, FormAndFlags::iOldFlags, old_flags);
    e.set(this, FormAndFlags::cVersion, version);
    this
}

// Translated from 008572c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreatedReferenceData` default constructor (Xbox PDB layout): the type
/// and the bound id are 0, then the `ReferenceData` embedded at +8 is
/// constructed (`fn_008572f0`).
pub fn fn_008572c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(EMBEDDED_RECORD_INIT, &args![this.byte_add(8)]);
    this
}

// Translated from 008572f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReferenceData` default constructor (Xbox PDB layout): the location id
/// is 0 and the two embedded `NiPoint3` (their constructor, `006815c0`, does
/// nothing) are left alone.
pub fn fn_008572f0(e: &mut Engine, this: Ptr<ReferenceData>) -> Ptr<ReferenceData> {
    e.set(this, ReferenceData::iLocationID, 0);
    e.call(LIST_NODE_ITEM, &args![this.byte_add(4)]);
    e.call(LIST_NODE_ITEM, &args![this.byte_add(0x10)]);
    this
}

// Translated from 00857320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovedReferenceData` default constructor (Xbox PDB layout): the original
/// location id is 0, the original position (+4) is left alone and the
/// `ReferenceData` at +0x10 is constructed.
pub fn fn_00857320(e: &mut Engine, this: Ptr<MovedReferenceData>) -> Ptr<MovedReferenceData> {
    e.set(this, MovedReferenceData::iOriginalLocationID, 0);
    e.call(LIST_NODE_ITEM, &args![this.byte_add(4)]);
    fn_008572f0(e, Ptr::new(this.addr() + 0x10));
    this
}

// Translated from 00857350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTLargePrimitiveArray<FormAndFlags *>` at
/// `TESSaveLoadGame + 0x20` (the engine map names the folded body
/// `~basic_streambuf<>`): runs `00863d60` on the object.
pub fn fn_00857350(e: &mut Engine, this: Ptr) {
    e.call(INIT_ARRAY_DESTRUCT, &args![this]);
}

// Translated from 00857370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens, or hands back, the file a save or load works on. `mode` 0 opens
/// the save for writing (rotating the previous save to `.bak` first; when
/// `file` is given it is closed and deleted and a fresh save is opened, in
/// a name that keeps only "autosave" names), 1 opens `name` (or the default
/// name) for reading, 2 re-opens `file`, 3 and anything else return `file`.
/// `name` null means the default save name (`00860ae0`). Not translated: the
/// exception frame and the stack cookie.
pub fn fn_00857370(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    name: Ptr,
    mode: u32,
) -> Ptr {
    // The character buffers the game keeps on its stack.
    const PATH: u32 = 0x000;
    const NAME: u32 = 0x104;
    const DIRECTORY: u32 = 0x208;
    const CURRENT_PATH: u32 = 0x30C;
    const BACKUP_PATH: u32 = 0x410;
    const TRIMMED: u32 = 0x514;
    const FRAME: u32 = 0x618;
    // The number of previous saves the rotation keeps: a constant stored in
    // a local, which the code clamps to 10.
    const KEPT_BACKUPS: i32 = 1;

    let frame = e.mem.alloc(FRAME);
    let (path, name_buf, directory, current_path, backup_path, trimmed) = (
        frame + PATH,
        frame + NAME,
        frame + DIRECTORY,
        frame + CURRENT_PATH,
        frame + BACKUP_PATH,
        frame + TRIMMED,
    );
    if !file.is_null() {
        let file_name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
        e.call(STRING_COPY, &args![path, 0x104u32, file_name]);
    } else {
        if name.is_null() {
            e.call(DEFAULT_SAVE_NAME, &args![this, name_buf]);
        } else {
            e.call(STRING_COPY, &args![name_buf, 0x104u32, name]);
        }
        // "<base><folder><name>.ess"
        let middle = save_folder(e);
        let prefix = e.call(PATH_PREFIX, &args![]).u32();
        e.call(
            FORMAT,
            &args![path, 0x104u32, FORMAT_SAVE_PATH, prefix, middle, name_buf],
        );
        if mode == 0 {
            // Make the folder and rotate the previous save to ".bak".
            let prefix = e.call(PATH_PREFIX, &args![]).u32();
            e.call(STRING_COPY, &args![directory, 0x104u32, prefix]);
            let middle = save_folder(e);
            e.call(STRING_CAT, &args![directory, 0x104u32, middle]);
            e.call(CREATE_DIRECTORY_IMPORT, &args![directory, 0u32]);
            let mut count = KEPT_BACKUPS;
            if count > 10 {
                count = 10;
            }
            let mut index = count - 1;
            while index >= 0 {
                let prefix = e.call(PATH_PREFIX, &args![]).u32();
                e.call(STRING_COPY, &args![current_path, 0x104u32, prefix]);
                let middle = save_folder(e);
                e.call(STRING_CAT, &args![current_path, 0x104u32, middle]);
                e.call(STRING_CAT, &args![current_path, 0x104u32, name_buf]);
                for _ in 0..index {
                    e.call(STRING_CAT, &args![current_path, 0x104u32, BAK_EXTENSION]);
                }
                e.call(STRING_COPY, &args![backup_path, 0x104u32, current_path]);
                e.call(STRING_CAT, &args![backup_path, 0x104u32, BAK_EXTENSION]);
                if index == 0 {
                    e.call(STRING_COPY, &args![current_path, 0x104u32, path]);
                }
                let exists = e
                    .call(
                        FILE_EXISTS,
                        &args![current_path, 0u32, 0u32, 0xFFFF_FFFFu32],
                    )
                    .u32();
                if exists != 0 {
                    let old_exists = e
                        .call(FILE_EXISTS, &args![backup_path, 0u32, 0u32, 0xFFFF_FFFFu32])
                        .u32();
                    if old_exists != 0 {
                        e.call(DELETE_FILE_IMPORT, &args![backup_path]);
                    }
                    e.call(RENAME, &args![current_path, backup_path]);
                }
                index -= 1;
            }
        }
    }

    let result = match mode {
        0 => {
            if !file.is_null() {
                // Re-save: close and delete the stream, then open the save
                // again, under its own name only when that name is neither a "Save "
                // name nor contains "autosave".
                let file_name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
                let last = e.call(STRRCHR, &args![file_name, 0x5Cu32]).u32();
                let leaf = last.wrapping_add(1);
                e.call(STRING_COPY, &args![trimmed, 0x104u32, leaf]);
                let length = e.call(STRLEN, &args![trimmed]).u32();
                if length > 4 {
                    let tail = trimmed + length - 4;
                    if e.call(STRNICMP, &args![tail, ESS_EXTENSION, 4u32]).i32() == 0 {
                        e.mem.set_u8(tail, 0);
                    }
                }
                fn_00857950(e, this, file, Ptr::NULL);
                let opens_default_name = e
                    .call(STRNICMP, &args![trimmed, SAVE_NAME_PREFIX, 5u32])
                    .i32()
                    == 0
                    || e.call(STRING_FIND, &args![trimmed, AUTOSAVE_NAME]).u32() != 0;
                if opens_default_name {
                    fn_00857370(e, this, Ptr::NULL, Ptr::NULL, 0)
                } else {
                    fn_00857370(e, this, Ptr::NULL, Ptr::new(trimmed), 0)
                }
            } else {
                open_bs_file(e, path, 1, false)
            }
        }
        1 => open_bs_file(e, path, 0, true),
        2 => {
            e.vcall(file.addr(), FILE_OPEN, &args![0u32, 0u32]);
            file
        }
        _ => file,
    };
    e.mem.free(frame);
    result
}

/// The folder part of a save's path: `00464f30(00403df0(PATH_OBJECT))`.
pub(crate) fn save_folder(e: &mut Engine) -> u32 {
    let text = e.call(PATH_OBJECT_GET, &args![PATH_OBJECT]).u32();
    e.call(IDENTITY, &args![text]).u32()
}

/// `new BSFile(path, write_mode, 0x20000, 0)`, then (for `open`) its
/// virtual slot `0x20` with two zeros. Null when the allocation fails.
pub(crate) fn open_bs_file(e: &mut Engine, path: u32, write_mode: u32, open: bool) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![BSFILE_SIZE]).u32();
    let file = if block != 0 {
        e.call(
            BSFILE_CONSTRUCT,
            &args![block, path, write_mode, 0x20000u32, 0u32],
        )
        .ptr()
    } else {
        Ptr::NULL
    };
    if open {
        e.vcall(file.addr(), FILE_OPEN, &args![0u32, 0u32]);
    }
    file
}

// Translated from 008578b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets go of a save file: for `mode` 0, 1 and 3 it is removed from the
/// game's list of save files (when there is one) and destroyed through its
/// virtual destructor; for `mode` 2 it is closed (`BSFile::Close`). Null
/// files and other modes do nothing.
pub fn fn_008578b0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr, mode: u32) {
    if file.is_null() {
        return;
    }
    match mode {
        0 | 1 | 3 => {
            let list = e.get(this, TESSaveLoadGame::m_pSaveGameList);
            if !list.is_null() {
                let cell = e.mem.alloc(4);
                e.mem.set_u32(cell, file.addr());
                e.call(LIST_REMOVE, &args![list, cell]);
                e.mem.free(cell);
            }
            e.vcall(file.addr(), FILE_DESTRUCT, &args![1u32]);
        }
        2 => {
            e.call(BSFILE_CLOSE, &args![file]);
        }
        _ => {}
    }
}

// Translated from 00857950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes a save file from disk: gets the stream for `file` (through
/// `fn_00857370` with mode 3, which returns it), deletes the file it names
/// and lets go of the stream.
pub fn fn_00857950(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr, name: Ptr) {
    if file.is_null() {
        return;
    }
    let mut stream = fn_00857370(e, this, file, name, 3);
    if stream.is_null() {
        stream = file;
    }
    let path = e.vcall(stream.addr(), FILE_GET_NAME, &args![]).u32();
    e.call(FILE_DELETE, &args![path]);
    fn_008578b0(e, this, stream, 3);
}

// Translated from 008579b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `size` bytes from `data` into the current buffer and moves the
/// buffer pointer on (`memcpy` then `fn_00857bd0`).
pub fn fn_008579b0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, data: Ptr, size: u32) {
    let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.call(MEMCPY, &args![buffer, data, size]);
    fn_00857bd0(e, this, size);
}

// Translated from 008579e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `size` bytes from the current buffer into `data` and moves the
/// buffer pointer on.
pub fn fn_008579e0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, data: Ptr, size: u32) {
    let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.call(MEMCPY, &args![data, buffer, size]);
    fn_00857bd0(e, this, size);
}

// Translated from 00857a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveNumericID` (Xbox PDB): writes `size / 4` form ids
/// into the buffer; when the game uses the numeric id array each id goes
/// through `AddNumericIDToArray` first.
pub fn tes_save_load_game_save_numeric_id(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    ids: Ptr,
    size: u32,
) {
    let count = size >> 2;
    let cell = e.mem.alloc(4);
    for index in 0..count {
        let id = e.mem.u32(ids.addr() + index * 4);
        let value = if e.call(USE_NUMERIC_IDS, &args![this]).bool() {
            e.call(ADD_NUMERIC_ID, &args![this, id]).u32()
        } else {
            id
        };
        e.mem.set_u32(cell, value);
        let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
        e.call(MEMCPY, &args![buffer, cell, 4u32]);
        fn_00857bd0(e, this, 4);
    }
    e.mem.free(cell);
}

// Translated from 00857aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::LoadNumericID` (Xbox PDB): reads `size` bytes of ids
/// from the buffer into `ids`; when the game uses the numeric id array each
/// id is turned back into a form id. True when a non-zero id could not be
/// resolved.
pub fn tes_save_load_game_load_numeric_id(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    ids: Ptr,
    size: u32,
) -> bool {
    let mut unresolved = false;
    let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.call(MEMCPY, &args![ids, buffer, size]);
    if e.call(USE_NUMERIC_IDS, &args![this]).bool() {
        for index in 0..(size >> 2) {
            let slot = ids.addr() + index * 4;
            let resolved = e
                .call(RESOLVE_NUMERIC_ID, &args![this, e.mem.u32(slot)])
                .u32();
            if e.mem.u32(slot) != 0 && resolved == 0 {
                unresolved = true;
            }
            e.mem.set_u32(slot, resolved);
        }
    }
    fn_00857bd0(e, this, size);
    unresolved
}

// Translated from 00857b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `size` bytes at `data` to the save file (`TESFile`'s write,
/// `00473180`, on `file`), or, when the game only measures a save
/// (`0047c850` is true), adds `size` to `m_iSimulationFileSize`. Returns the
/// size or what the file's write returns.
pub fn fn_00857b50(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    data: Ptr,
    size: u32,
) -> u32 {
    if game_unavailable(e, this) {
        let counted = e.get(this, TESSaveLoadGame::m_iSimulationFileSize);
        e.set(
            this,
            TESSaveLoadGame::m_iSimulationFileSize,
            counted.wrapping_add(size),
        );
        size
    } else {
        e.call(FILE_WRITE, &args![file, data, size]).u32()
    }
}

// Translated from 00857ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `size` bytes from the file into `buffer` (`00462d80` on `file`);
/// returns what the read returns.
pub fn fn_00857ba0(e: &mut Engine, _this: Ptr, file: Ptr, buffer: Ptr, size: u32) -> u32 {
    e.call(FILE_READ, &args![file, buffer, size]).u32()
}

// Translated from 00857bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the buffer pointer on by `count` bytes.
pub fn fn_00857bd0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, count: u32) {
    let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.set(
        this,
        TESSaveLoadGame::m_pBuffer,
        Ptr::new(buffer.addr().wrapping_add(count)),
    );
}

// Translated from 00857bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps a saved form id to the current one: the top byte (the plugin
/// index) is looked up in `m_pFileIndexArray` (`m_iSavedPluginCount`
/// entries). Without the table, or for plugin index 0xFF, the id is
/// unchanged; an index outside the table, or mapped to 0xFF, gives 0.
pub fn fn_00857bf0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, id: u32) -> u32 {
    let plugin = (id >> 24) as u8;
    let table = e.get(this, TESSaveLoadGame::m_pFileIndexArray);
    if table.is_null() || plugin == 0xFF {
        return id;
    }
    let count = e.get(this, TESSaveLoadGame::m_iSavedPluginCount);
    if plugin < count {
        let mapped = e.mem.u8(table.addr() + plugin as u32);
        if mapped != 0xFF {
            return (id & 0x00FF_FFFF).wrapping_add((mapped as u32) << 24);
        }
    }
    0
}

// Translated from 00857c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `fn_00857bf0`: finds the plugin index (the last one) whose
/// table entry is the id's top byte and puts that index in the top byte.
/// Without the table, or for plugin index 0xFF, the id is unchanged; a byte
/// that is not in the table gives 0.
pub fn fn_00857c70(e: &mut Engine, this: Ptr<TESSaveLoadGame>, id: u32) -> u32 {
    let plugin = (id >> 24) as u8;
    let table = e.get(this, TESSaveLoadGame::m_pFileIndexArray);
    if table.is_null() || plugin == 0xFF {
        return id;
    }
    let count = e.get(this, TESSaveLoadGame::m_iSavedPluginCount);
    let mut found = 0xFFu8;
    for index in 0..count as u32 {
        if e.mem.u8(table.addr() + index) == plugin {
            found = index as u8;
        }
    }
    if found != 0xFF {
        (id & 0x00FF_FFFF).wrapping_add((found as u32) << 24)
    } else {
        0
    }
}

// Translated from 00857d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts a loaded reference where its saved data says it is. `this` is the
/// load form buffer (`BGSLoadFormBuffer`, whose version byte at +0x1C is
/// cleared during the call and restored). For an actor-like reference with a
/// saved cell or world space (virtual slots `0x290`, `0x298`, `0x294`) it
/// sets the position and the angle from the actor's own (virtual slots
/// `0x170`, `0x16C`) and moves the reference into that space; otherwise it
/// uses the reference's extra data (type 0xF: the same position and rotation;
/// and the starting world space or cell). When it moved the reference and
/// the reference has a 3D node it also puts the node, its character
/// controller and its collision at the new position. `teleport` brackets
/// the move with `004534f0(this, 1)` and `004534f0(this, 0)`. Returns
/// whether the reference was moved.
pub fn fn_00857d10(e: &mut Engine, this: Ptr, reference: Ptr, teleport: bool) -> bool {
    let version = e.call(BUFFER_GET_VERSION, &args![this]).u8();
    e.call(BUFFER_SET_VERSION, &args![this, 0u32]);
    let mut moved = false;
    let mut actor = 0u32;
    if e.vcall(reference.addr(), REFERENCE_IS_ACTOR, &args![])
        .bool()
    {
        actor = reference.addr();
    }
    let located = actor != 0 && e.vcall(actor, ACTOR_HAS_LOCATION_SLOT, &args![]).bool();
    if located {
        let cell = e.vcall(actor, ACTOR_CELL_SLOT, &args![]).u32();
        let worldspace = e.vcall(actor, ACTOR_WORLDSPACE_SLOT, &args![]).u32();
        if cell != 0 || worldspace != 0 {
            let out = e.mem.alloc(0x18);
            let position = e
                .vcall(actor, REFERENCE_GET_LOCATION_SLOT, &args![out])
                .u32();
            e.call(REF_SET_POSITION, &args![reference, position]);
            let rotation = e
                .vcall(actor, REFERENCE_GET_ROTATION_SLOT, &args![out + 0xC])
                .u32();
            let angle_z = e.mem.f32(rotation + 8);
            e.call(FN_005757D0, &args![reference, angle_z]);
            e.mem.free(out);
            if teleport {
                e.call(SET_LOADING_STATE, &args![this, 1u32]);
            }
            e.call(REF_MOVE_TO_SPACE, &args![reference, cell, worldspace]);
            if teleport {
                e.call(SET_LOADING_STATE, &args![this, 0u32]);
            }
            moved = true;
        }
    } else if e.call(REF_GET_EXTRA_LIST, &args![reference]).u32() != 0 {
        let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
        let data = e.call(EXTRA_GET_DATA, &args![list, 0xFu32]).u32();
        if data != 0 {
            let out = e.mem.alloc(0x18);
            e.vcall(reference.addr(), REFERENCE_GET_LOCATION_SLOT, &args![out]);
            e.vcall(
                reference.addr(),
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
            e.mem.free(out);
            moved = true;
        }
        let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
        let space = e.call(EXTRA_GET_STARTING_SPACE, &args![list]).u32();
        if space != 0 {
            let cell = dynamic_cast(e, space, RTTI_FORM, RTTI_CELL);
            let worldspace = dynamic_cast(e, space, RTTI_FORM, RTTI_WORLDSPACE);
            if cell != 0 || worldspace != 0 {
                e.call(REF_MOVE_TO_SPACE, &args![reference, cell, worldspace]);
                moved = true;
            }
        }
    }

    if moved {
        let node = e.call(REF_GET_NODE, &args![reference]).u32();
        if node != 0 {
            let position_ptr = e
                .vcall(reference.addr(), REFERENCE_GET_POSITION_SLOT, &args![])
                .u32();
            let position = e.mem.alloc(0x0C);
            let words = e.mem.bytes(position_ptr, 0x0C);
            e.mem.write(position, &words);
            let mobile = dynamic_cast(e, reference.addr(), RTTI_REFERENCE, RTTI_MOBILE_OBJECT);
            if mobile != 0 {
                let controller = e.call(MOBILE_GET_CHAR_CONTROLLER, &args![mobile]).u32();
                if controller != 0 && !e.call(CHAR_CONTROLLER_TEST, &args![controller]).bool() {
                    e.call(CHAR_CONTROLLER_SET_POSITION, &args![controller, position]);
                }
            }
            e.call(FN_00440460, &args![node, position]);
            let matrix = e.mem.alloc(0x24);
            let orientation = e.call(REF_GET_ORIENTATION, &args![reference, matrix]).u32();
            e.call(FN_0043FA80, &args![node, orientation]);
            e.call(COLLISION_RESET_SIM, &args![node, 1u32]);
            let transform = e.mem.alloc(0x0C);
            e.call(FN_0043D410, &args![transform, 0.0f32, 0u32, 0u32]);
            e.call(FN_00A59C60, &args![node, transform]);
            e.mem.free(transform);
            e.mem.free(matrix);
            e.mem.free(position);
        }
    }
    e.call(BUFFER_SET_VERSION, &args![this, version as u32]);
    moved
}

/// Writes `bytes` (a local the game keeps on its stack) to the save file
/// through `fn_00857b50`.
pub(crate) fn put_bytes(e: &mut Engine, game: Ptr<TESSaveLoadGame>, file: Ptr, bytes: &[u8]) {
    let cell = e.mem.alloc(bytes.len() as u32);
    e.mem.write(cell, bytes);
    fn_00857b50(e, game, file, Ptr::new(cell), bytes.len() as u32);
    e.mem.free(cell);
}

/// One length-prefixed block of the global data: writes `size` (2 bytes),
/// and for a non-zero size notes it in the statistics under `label`, makes
/// a buffer of that size, lets `fill` write into it, writes the buffer to
/// the file and frees it.
pub(crate) fn save_sized_block(
    e: &mut Engine,
    game: Ptr<TESSaveLoadGame>,
    file: Ptr,
    size: u16,
    label: u32,
    fill: impl FnOnce(&mut Engine),
) {
    put_bytes(e, game, file, &size.to_le_bytes());
    if size != 0 {
        let stats = e.get(game, TESSaveLoadGame::m_pSaveLoadStats);
        if !stats.is_null() {
            save_stats_add_extra_stat(e, stats, size as u32, Ptr::new(label));
        }
        let buffer = tes_save_load_game_create_buffer(e, game, size as u32);
        fill(e);
        tes_save_load_game_write_file(e, game, file, buffer, size as u32);
        fn_00858700(e, game, buffer);
    }
}

// Translated from 00858030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveGlobalData` (Xbox PDB): writes the global data of
/// a save: the data handler's size word, the id of the `TES` world space,
/// two more `TES` words, the player's location (world space or parent cell
/// id, and position), the global variables (`SaveGlobals`), and then one
/// length-prefixed block each for the `TES` object, the process lists, the
/// sky and weather, the created base objects (a zero word, then
/// `SaveCreatedBaseObjects`), the (always empty) HUD reticle, the interface
/// and the regions. A player with neither a world space nor a parent cell
/// raises the "cannot save" error.
pub fn tes_save_load_game_save_global_data(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr) {
    let handler: u32 = e.global(DATA_HANDLER);
    let handler_size = e.call(GLOBAL_DATA_SIZE, &args![handler]).u32();
    put_bytes(e, this, file, &handler_size.to_le_bytes());

    let tes: u32 = e.global(TES_OBJECT);
    let tes_worldspace = e.call(TES_GET_WORLDSPACE, &args![tes]).u32();
    let tes_worldspace_id = e.call(FORM_ID, &args![tes_worldspace]).u32();
    put_bytes(e, this, file, &tes_worldspace_id.to_le_bytes());

    let first = e.call(READ_FIELD_24, &args![tes]).u32();
    let second = e.call(READ_FIELD_28, &args![tes]).u32();
    put_bytes(e, this, file, &first.to_le_bytes());
    put_bytes(e, this, file, &second.to_le_bytes());

    let player: u32 = e.global(PLAYER);
    let worldspace = e.call(REF_GET_WORLDSPACE, &args![player]).u32();
    let parent_cell = e.call(REF_GET_PARENT_CELL, &args![player]).u32();
    if worldspace == 0 && parent_cell == 0 {
        e.call(ERROR, &args![MSG_PLAYER_HAS_NO_SPACE]);
    }
    let mut location_id = 0u32;
    if worldspace != 0 {
        location_id = e.call(FORM_ID, &args![worldspace]).u32();
    } else if parent_cell != 0 {
        location_id = e.call(FORM_ID, &args![parent_cell]).u32();
    }
    let position_ptr = e.call(REF_GET_POSITION, &args![player]).u32();
    let position = e.mem.bytes(position_ptr, 0x0C);
    put_bytes(e, this, file, &location_id.to_le_bytes());
    put_bytes(e, this, file, &position);

    tes_save_load_game_save_globals(e, this, file);

    let size = e.call(TES_SAVE_SIZE, &args![tes]).u16();
    save_sized_block(e, this, file, size, LABEL_TES_CLASS, |e| {
        e.call(TES_SAVE, &args![tes]);
    });

    let size = e.call(PROCESS_LISTS_SAVE_SIZE, &args![PROCESS_LISTS]).u16();
    save_sized_block(e, this, file, size, LABEL_PROCESS_LISTS, |e| {
        e.call(PROCESS_LISTS_SAVE, &args![PROCESS_LISTS]);
    });

    let sky = e.call(SKY_INSTANCE, &args![]).u32();
    let size = e.call(SKY_SAVE_SIZE, &args![sky]).u16();
    save_sized_block(e, this, file, size, LABEL_SKY, |e| {
        let sky = e.call(SKY_INSTANCE, &args![]).u32();
        e.call(SKY_SAVE, &args![sky]);
    });

    put_bytes(e, this, file, &0u32.to_le_bytes());
    e.call(SAVE_CREATED_BASE_OBJECTS, &args![this, file]);

    // The reticle block is always empty: the size is a local set to zero.
    let size = 0u16;
    save_sized_block(e, this, file, size, LABEL_HUD_RETICLE, |_| {});

    let size = e.call(INTERFACE_SAVE_SIZE, &args![]).u16();
    save_sized_block(e, this, file, size, LABEL_INTERFACE, |e| {
        e.call(INTERFACE_SAVE, &args![]);
    });

    let size = e.call(REGIONS_SAVE_SIZE, &args![]).u16();
    save_sized_block(e, this, file, size, LABEL_REGIONS, |e| {
        e.call(REGIONS_SAVE, &args![]);
    });
}

// Translated from 00858480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveGlobals` (Xbox PDB): writes the global variables
/// of the data handler as one block: a 2-byte count, then for each variable
/// its numeric id (through `SaveNumericID`) and its value (a float). The
/// block is `count * 8 + 2` bytes.
pub fn tes_save_load_game_save_globals(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr) {
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(GLOBALS_LIST, &args![handler]).u32();
    let count = e.call(LIST_COUNT, &args![node]).u16();
    let size = (count as u32 * 8 + 2) as u16;
    let buffer = tes_save_load_game_create_buffer(e, this, size as u32);
    let cell = e.mem.alloc(8);
    e.mem.set_u16(cell, count);
    fn_008579b0(e, this, Ptr::new(cell), 2);
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let variable = e.mem.u32(slot);
        if variable != 0 {
            let id = e.call(FORM_ID, &args![variable]).u32();
            let value = e.call(GLOBAL_VALUE, &args![variable]).f32();
            e.mem.set_u32(cell, id);
            tes_save_load_game_save_numeric_id(e, this, Ptr::new(cell), 4);
            e.mem.set_f32(cell + 4, value);
            fn_008579b0(e, this, Ptr::new(cell + 4), 4);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.mem.free(cell);
    tes_save_load_game_write_file(e, this, file, buffer, size as u32);
    fn_00858700(e, this, buffer);
    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        save_stats_add_extra_stat(e, stats, size as u32, Ptr::new(LABEL_GLOBAL_VARIABLES));
    }
}

// Translated from 00858570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveFinalData` (Xbox PDB): writes the size (4 bytes)
/// of the process lists' temp effects list and, when it is not zero, the
/// list itself through a buffer of that size.
pub fn tes_save_load_game_save_final_data(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr) {
    let size = e.call(TEMP_EFFECTS_SIZE, &args![PROCESS_LISTS]).u32();
    put_bytes(e, this, file, &size.to_le_bytes());
    if size != 0 {
        let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
        if !stats.is_null() {
            save_stats_add_extra_stat(e, stats, size, Ptr::new(LABEL_TEMP_EFFECTS));
        }
        let buffer = tes_save_load_game_create_buffer(e, this, size);
        e.call(TEMP_EFFECTS_SAVE, &args![PROCESS_LISTS]);
        tes_save_load_game_write_file(e, this, file, buffer, size);
        fn_00858700(e, this, buffer);
    }
}

// Translated from 00858600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::CreateBuffer` (Xbox PDB): allocates `size` bytes (in an
/// allocation scope), makes them the current buffer and returns them; raises
/// an error when the allocation fails. Not translated: the exception frame.
pub fn tes_save_load_game_create_buffer(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    size: u32,
) -> Ptr {
    let scope = scope_enter(e, 0x103A);
    let block = e.call(OPERATOR_NEW, &args![size]).ptr::<()>();
    e.set(this, TESSaveLoadGame::m_pBuffer, block);
    if e.get(this, TESSaveLoadGame::m_pBuffer).is_null() {
        e.call(ERROR, &args![MSG_NO_SAVE_BUFFER]);
    }
    let buffer = e.get(this, TESSaveLoadGame::m_pBuffer);
    scope_leave(e, scope);
    buffer
}

// Translated from 008586a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::WriteFile` (Xbox PDB): `fn_00857b50`.
pub fn tes_save_load_game_write_file(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    buffer: Ptr,
    size: u32,
) {
    fn_00857b50(e, this, file, buffer, size);
}

// Translated from 008586d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The read counterpart of `WriteFile`: `fn_00857ba0`; returns its result.
pub fn fn_008586d0(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    buffer: Ptr,
    size: u32,
) -> u32 {
    fn_00857ba0(e, this.cast(), file, buffer, size)
}

// Translated from 00858700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a buffer made by `CreateBuffer` and clears the current buffer.
pub fn fn_00858700(e: &mut Engine, this: Ptr<TESSaveLoadGame>, buffer: Ptr) {
    delete(e, buffer.addr());
    e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
}

// Translated from 00858730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads one form from its pre-built buffer (the older load path; it does
/// nothing and returns false unless `0047c850` is true). Takes the form's
/// `ChangeData` and its buffer, enters the load section, reads the 4-byte
/// header (size, form type, version) and: when the saved form type is not
/// the form's own, formats the "Load Error" text (it is not shown), drops
/// the form's changes and returns false; otherwise builds the
/// `LoadFormHeader`, applies the initial data (`0085ac30`), lets the form
/// load its changes (virtual slot `0x60`), remembers the form and its
/// version in the init array (making the array on first use), frees the
/// buffer, applies a queued `RemoveChanges` and returns true. Not
/// translated: the exception frame and the stack cookie.
pub fn fn_00858730(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) -> bool {
    // The frame cells: the 4-byte header read from the buffer (size, type,
    // version), the 12-byte `LoadFormHeader` made from it, the text of the
    // load error and the slot passed to the init array's `Add`.
    const SOURCE: u32 = 0x00;
    const HEADER: u32 = 0x10;
    const TEXT: u32 = 0x20;
    const SLOT: u32 = 0x130;
    const FRAME: u32 = 0x140;

    if !game_unavailable(e, this) {
        return false;
    }
    let changes = e.get(this, TESSaveLoadGame::m_pChanges);
    let change_data = fn_00855130(e, changes, form);
    if change_data.is_null() {
        return false;
    }
    let buffer = e.call(LIST_NODE_NEXT, &args![change_data]).u32();
    if buffer == 0 {
        return false;
    }
    e.call(SECTION_ENTER, &args![LOAD_SECTION, MSG_LOAD_FORM_SECTION]);
    e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
    let frame = e.mem.alloc(FRAME);
    let game = game_singleton(e);
    fn_008579e0(e, game, Ptr::new(frame + SOURCE), 4);
    let saved_type = e.mem.u8(frame + SOURCE + 2);
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    if form_type != saved_type as u32 {
        let current_name = e.call(FORM_TYPE_NAME, &args![form]).u32();
        let saved_name = e.mem.u32(FORM_TYPE_NAME_TABLE + saved_type as u32 * 12);
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(
            SPRINTF,
            &args![
                frame + TEXT,
                FORMAT_LOAD_ERROR,
                id,
                saved_name,
                current_name
            ],
        );
        let changes = e.get(this, TESSaveLoadGame::m_pChanges);
        fn_008551f0(e, changes, form, 1);
        e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::NULL);
        e.call(SECTION_LEAVE, &args![LOAD_SECTION]);
        e.mem.free(frame);
        return false;
    }

    let size = e.mem.u16(frame + SOURCE) as u32;
    let flags = e.call(READ_WORD, &args![change_data]).u32();
    let flags = tes_save_load_game_check_new_reference(e, this.cast(), form, flags);
    let version = e.mem.u8(frame + SOURCE + 3);
    e.call(SET_LOAD_VERSION, &args![this, version as u32]);
    let id = e.call(FORM_ID, &args![form]).u32();
    let header: Ptr<LoadFormHeader> = Ptr::new(frame + HEADER);
    fn_00858aa0(e, header, Ptr::new(frame + SOURCE), id, flags);
    e.call(SET_LOADING_HEADER, &args![this, header]);
    let was_set = game_unavailable(e, this);
    e.call(SET_LOADING_STATE, &args![this, 1u32]);
    e.call(FORM_FINISH, &args![form, 1u32]);
    e.call(LOAD_INITIAL_DATA, &args![this, form, flags]);
    e.vcall(form.addr(), FORM_LOAD, &args![flags, 0u32]);
    e.call(SET_LOADING_STATE, &args![this, was_set as u32]);
    e.call(SET_LOADING_HEADER, &args![this, 0u32]);
    e.call(END_FORM_PROCESSING, &args![this]);

    if e.get(this, TESSaveLoadGame::m_pInitArray).is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        let array = if block != 0 {
            e.call(INIT_ARRAY_CONSTRUCT, &args![block, 0x32u32, 0x32u32])
                .ptr::<()>()
        } else {
            Ptr::NULL
        };
        e.set(this, TESSaveLoadGame::m_pInitArray, array);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let record = if block != 0 {
        fn_00857280(e, Ptr::new(block), form, flags, 0, version).addr()
    } else {
        0
    };
    e.mem.set_u32(frame + SLOT, record);
    let array = e.get(this, TESSaveLoadGame::m_pInitArray);
    e.call(INIT_ARRAY_ADD, &args![array, frame + SLOT]);
    // The game also computes how far the buffer pointer is past the form's
    // data (`m_pBuffer - (buffer + 4 + size)`) and does not use it.
    let _ = size;
    fn_00858700(e, this, Ptr::new(buffer));
    e.call(CHANGE_DATA_SET_BUFFER, &args![change_data, 0u32]);
    let queued = e.get(this, TESSaveLoadGame::m_iQueuedRemoveChanges);
    if queued != 0 {
        let changes = e.get(this, TESSaveLoadGame::m_pChanges);
        fn_00855150(e, changes, form, queued);
        e.set(this, TESSaveLoadGame::m_iQueuedRemoveChanges, 0);
    }
    e.call(SECTION_LEAVE, &args![LOAD_SECTION]);
    e.mem.free(frame);
    true
}

// Translated from 00858aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a `LoadFormHeader` from the 4 bytes read from a buffer (`source`:
/// size word, form type, version) and the form id and flags.
pub fn fn_00858aa0(
    e: &mut Engine,
    this: Ptr<LoadFormHeader>,
    source: Ptr,
    form_id: u32,
    flags: u32,
) -> Ptr<LoadFormHeader> {
    e.set(this, LoadFormHeader::iFormID, form_id);
    e.set(this, LoadFormHeader::iFlags, flags);
    let form_type = e.mem.u8(source.addr() + 2);
    e.set(this, LoadFormHeader::cFormType, form_type);
    let version = e.mem.u8(source.addr() + 3);
    e.set(this, LoadFormHeader::cVersion, version);
    let size = e.mem.u16(source.addr());
    e.set(this, LoadFormHeader::iSize, size);
    this
}

// Translated from 00858af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// After forms were loaded, applies what is queued: the forms of
/// `init_array` (when null, the game's own `m_pInitArray`), each with the
/// version it was loaded with, get their pre-initialization hook (virtual
/// slot `0x68`), the actors among them that are in no cell or world space
/// are collected (only when the array was given), and then those actors are
/// given their package locations, flagged in the changes map and moved to
/// the placement cell (or disabled when there is none). When `reload` is
/// set the world and the loader are locked and flushed around the work and
/// the player's camera is placed again. The forms then get their
/// post-initialization hook (slot `0x6C`) and are freed, and the game's
/// array is destroyed. `location` is an optional pair of words (+4, +8) given
/// to the player's two hooks. Does nothing unless `0047c850` is true. Not
/// translated: the exception frame.
pub fn fn_00858af0(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    init_array: Ptr,
    location: Ptr,
    reload: bool,
) {
    if !game_unavailable(e, this) {
        return;
    }
    e.call(SET_LOADING_STATE, &args![this, 1u32]);
    // 00858b41 and on: the global words (the I/O manager 01202d98, the player
    // 011dea3c, `TES` 011dea10) are loaded at each use, not once.
    if reload {
        let io_manager: u32 = e.global(SAVE_LOCK);
        e.call(IO_MANAGER_SET_STATE_5, &args![io_manager]);
    }
    let mut array = init_array;
    let mut supplied = true;
    if array.is_null() {
        array = e.get(this, TESSaveLoadGame::m_pInitArray);
        supplied = false;
    }
    if !location.is_null() {
        let (first, second) = (
            e.mem.u32(location.addr() + 4),
            e.mem.u32(location.addr() + 8),
        );
        let player: u32 = e.global(PLAYER);
        e.vcall(player, FORM_BEGIN_INIT, &args![first, second]);
    }
    let list = e.mem.alloc(8);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![list]);
    let cell = e.mem.alloc(4);

    if !array.is_null() {
        let count = e.call(ARRAY_SIZE, &args![array]).u32();
        for index in 0..count {
            let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
            let item = e.mem.u32(slot);
            // 00858bec: compared with the player word loaded each pass.
            if item != 0 && e.mem.u32(item) != e.global::<u32>(PLAYER) {
                let version = e.mem.u8(item + 0xC);
                e.call(SET_LOAD_VERSION, &args![this, version as u32]);
                let form = e.mem.u32(item);
                let (flags, old_flags) = (e.mem.u32(item + 4), e.mem.u32(item + 8));
                e.vcall(form, FORM_BEGIN_INIT, &args![flags, old_flags]);
                if supplied {
                    let actor = dynamic_cast(e, form, RTTI_FORM, RTTI_ACTOR);
                    e.mem.set_u32(cell, actor);
                    if actor != 0
                        && e.call(REF_GET_PARENT_CELL, &args![actor]).u32() == 0
                        && e.call(REF_GET_WORLDSPACE, &args![actor]).u32() == 0
                    {
                        e.call(LIST_ADD_HEAD, &args![list, cell]);
                    }
                }
                e.call(END_FORM_PROCESSING, &args![this]);
            }
        }
    }

    if supplied {
        let mut node = list;
        while node != 0 && !e.call(LIST_NODE_IS_END, &args![node]).bool() {
            let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let actor = e.mem.u32(slot);
            e.call(ACTOR_INIT_PACKAGE_LOCATIONS, &args![actor, 0u32]);
            let changes = e.get(this, TESSaveLoadGame::m_pChanges);
            fn_00855010(e, changes, Ptr::new(actor), 2);
            if e.call(REF_GET_PARENT_CELL, &args![actor]).u32() == 0
                && e.call(REF_GET_WORLDSPACE, &args![actor]).u32() == 0
            {
                // The placement cell comes from a lookup of form id 0.
                let default_form = e.call(LOOKUP_FORM, &args![0u32]).u32();
                let placement_cell = dynamic_cast(e, default_form, RTTI_FORM, RTTI_CELL);
                if placement_cell == 0 {
                    e.call(FORM_SET_DISABLED, &args![actor, 1u32]);
                } else {
                    let vectors = e.mem.alloc(0x18);
                    let words = e.mem.bytes(DEFAULT_POSITION, 0x0C);
                    e.mem.write(vectors, &words);
                    e.mem.write(vectors + 0xC, &words);
                    e.call(
                        CELL_GET_PLACEMENT,
                        &args![placement_cell, vectors, vectors + 0xC],
                    );
                    e.call(REF_SET_POSITION, &args![actor, vectors]);
                    let (x, y, z) = (
                        e.mem.u32(vectors + 0xC),
                        e.mem.u32(vectors + 0x10),
                        e.mem.u32(vectors + 0x14),
                    );
                    e.call(FN_00575700, &args![actor, x, y, z]);
                    e.call(REF_MOVE_TO_SPACE, &args![actor, placement_cell, 0u32]);
                    e.mem.free(vectors);
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        e.call(LIST_REMOVE_ALL, &args![list]);
    }

    if reload {
        // 00858de4..00858eb1: `TES`, the player and the I/O manager are each
        // loaded again at their calls.
        let tes: u32 = e.global(TES_OBJECT);
        let world = locked_world(e, tes);
        if world != 0 {
            e.call(WORLD_ADD_LOCK, &args![world]);
        }
        lock_other_world(e);
        let tes: u32 = e.global(TES_OBJECT);
        e.call(FN_00459920, &args![tes]);
        let player: u32 = e.global(PLAYER);
        if e.call(PLAYER_GET_3D, &args![player, 0u32]).u32() == 0 {
            let player: u32 = e.global(PLAYER);
            let loader: u32 = e.global(MODEL_LOADER);
            e.call(
                MODEL_LOADER_QUEUE_REFERENCE,
                &args![loader, player, 0u32, 0u32],
            );
        }
        let io_manager: u32 = e.global(SAVE_LOCK);
        e.call(IO_MANAGER_LOAD_QUEUED_PRIORITY, &args![io_manager]);
        let io_manager: u32 = e.global(SAVE_LOCK);
        e.call(IO_MANAGER_SET_STATE_5, &args![io_manager]);
        unlock_worlds(e, world);
        let player: u32 = e.global(PLAYER);
        let position_ptr = e.vcall(player, REFERENCE_GET_POSITION_SLOT, &args![]).u32();
        let position = e.mem.bytes(position_ptr, 0x0C);
        e.call(SET_GLOBAL_FLAG, &args![0u32]);
        let (x, y, z) = (
            u32::from_le_bytes(position[0..4].try_into().unwrap()),
            u32::from_le_bytes(position[4..8].try_into().unwrap()),
            u32::from_le_bytes(position[8..12].try_into().unwrap()),
        );
        let (a, b, c) = (
            e.mem.u32(PLACEMENT_VECTOR),
            e.mem.u32(PLACEMENT_VECTOR + 4),
            e.mem.u32(PLACEMENT_VECTOR + 8),
        );
        e.call(FN_0057D0A0, &args![x, y, z, a, b, c, 1.0f32]);
        e.call(SET_GLOBAL_FLAG, &args![1u32]);
        let _ = world;
    }
    if !location.is_null() {
        let player: u32 = e.global(PLAYER);
        if e.call(REF_GET_PARENT_CELL, &args![player]).u32() != 0 {
            let player: u32 = e.global(PLAYER);
            e.call(EMPTY_FN_00483710, &args![player]);
        }
    }
    if !location.is_null() {
        let (first, second) = (
            e.mem.u32(location.addr() + 4),
            e.mem.u32(location.addr() + 8),
        );
        let player: u32 = e.global(PLAYER);
        e.vcall(player, FORM_END_INIT, &args![first, second]);
    }

    if !array.is_null() {
        let count = e.call(ARRAY_SIZE, &args![array]).u32();
        for index in 0..count {
            let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
            let item = e.mem.u32(slot);
            if item != 0 {
                let version = e.mem.u8(item + 0xC);
                e.call(SET_LOAD_VERSION, &args![this, version as u32]);
                let form = e.mem.u32(item);
                let (flags, old_flags) = (e.mem.u32(item + 4), e.mem.u32(item + 8));
                e.vcall(form, FORM_END_INIT, &args![flags, old_flags]);
                e.call(END_FORM_PROCESSING, &args![this]);
                delete(e, item);
            }
        }
    }
    let game_array = e.get(this, TESSaveLoadGame::m_pInitArray);
    if !game_array.is_null() {
        e.call(INIT_ARRAY_CLEANUP, &args![game_array]);
        let game_array = e.get(this, TESSaveLoadGame::m_pInitArray);
        if !game_array.is_null() {
            e.vcall(game_array.addr(), FILE_DESTRUCT, &args![1u32]);
        }
        e.set(this, TESSaveLoadGame::m_pInitArray, Ptr::NULL);
    }

    if reload {
        // 00859060 / 008590bf: loaded again after the forms' second hooks.
        let tes: u32 = e.global(TES_OBJECT);
        let world = locked_world(e, tes);
        if world != 0 {
            e.call(WORLD_ADD_LOCK, &args![world]);
        }
        lock_other_world(e);
        let io_manager: u32 = e.global(SAVE_LOCK);
        e.call(IO_MANAGER_LOAD_QUEUED_PRIORITY, &args![io_manager]);
        if world != 0 {
            e.call(WORLD_REMOVE_LOCK, &args![world, 0u32]);
        }
        unlock_other_world(e);
    }
    e.call(SET_LOADING_STATE, &args![this, 0u32]);
    e.call(LIST_DESTRUCT, &args![list]);
    e.mem.free(cell);
    e.mem.free(list);
}

/// The object `00 5f36f0` (a field of `TES`) leads to through `004543c0`,
/// or 0: the code asks `TES` twice.
pub(crate) fn locked_world(e: &mut Engine, tes: u32) -> u32 {
    if e.call(READ_FIELD_34, &args![tes]).u32() != 0 {
        let mover = e.call(READ_FIELD_34, &args![tes]).u32();
        e.call(CELL_GET_PHYSICS_WORLD, &args![mover]).u32()
    } else {
        0
    }
}

/// Locks the second world object (`00451010`) when there is one.
pub(crate) fn lock_other_world(e: &mut Engine) {
    if e.call(GET_EXTERIOR_WORLD, &args![]).u32() != 0 {
        let other = e.call(GET_EXTERIOR_WORLD, &args![]).u32();
        e.call(WORLD_ADD_LOCK, &args![other]);
    }
}

/// Unlocks the second world object (`00451010`) when there is one.
pub(crate) fn unlock_other_world(e: &mut Engine) {
    if e.call(GET_EXTERIOR_WORLD, &args![]).u32() != 0 {
        let other = e.call(GET_EXTERIOR_WORLD, &args![]).u32();
        e.call(WORLD_REMOVE_LOCK, &args![other, 0u32]);
    }
}

/// Unlocks the world `world` (when not 0) and then the second one.
pub(crate) fn unlock_worlds(e: &mut Engine, world: u32) {
    if world != 0 {
        e.call(WORLD_REMOVE_LOCK, &args![world, 0u32]);
    }
    unlock_other_world(e);
}

// Translated from 00859120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::CheckNewReference` (Xbox PDB): for a form the data
/// handler knows (`00469860` on its id), clears bit 1 of the flags when the
/// form is a reference; returns the flags. (The code also casts the form to a
/// cell and then leaves the flags as they are.)
pub fn tes_save_load_game_check_new_reference(
    e: &mut Engine,
    _this: Ptr,
    form: Ptr,
    flags: u32,
) -> u32 {
    let mut flags = flags;
    let id = e.call(FORM_ID, &args![form]).u32();
    let handler: u32 = e.global(DATA_HANDLER);
    if e.call(DATA_HANDLER_HAS_FORM, &args![handler, id]).bool() {
        if dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_REFERENCE) != 0 {
            flags &= !2;
        }
        dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
    }
    flags
}

// Translated from 008591b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::CheckFlags` (Xbox PDB): adjusts the changed-parts flags
/// a form is saved with. Starts from `CheckNewReference`. A cell loses the
/// top bit when it has no seen data. A reference loses bit 5 unless its
/// extra data list has container changes. An actor (a reference with a
/// process) gets the flags of the package its process runs, and bit 2 is set
/// when `008aad40` says so, otherwise cleared when the actor's virtual slot
/// `0x22C` says no. A non-persistent reference other than the player that
/// has bits 1 or 2 is looked up in its cell (its own, or the one at its
/// position), which only matters for the calls made. Several tests the
/// compiler folded away (`flags & 0`) have no code here.
pub fn tes_save_load_game_check_flags(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form: Ptr,
    flags: u32,
) -> u32 {
    let mut flags = tes_save_load_game_check_new_reference(e, this.cast(), form, flags);
    let reference = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_REFERENCE);
    let cell = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
    if cell != 0 {
        if flags & 0x8000_0000 != 0 && e.call(EXTRA_GET_SEEN, &args![cell]).u32() == 0 {
            flags &= 0x7FFF_FFFF;
        }
        if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            e.call(CELL_GET_X, &args![cell]);
            e.call(CELL_GET_Y, &args![cell]);
        }
        return flags;
    }
    if reference == 0 {
        return flags;
    }
    if flags & 0x20 != 0 {
        let has_changes = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32() != 0 && {
            let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
            e.call(EXTRA_GET_CONTAINER_CHANGES, &args![list]).u32() != 0
        };
        if !has_changes {
            flags &= !0x20;
        }
    }
    let actor = dynamic_cast(e, reference, RTTI_REFERENCE, RTTI_ACTOR);
    if actor != 0 {
        if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_SLOT_20C, &args![]);
        }
        if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            if base_process_get_package_that_is_running(e, Ptr::new(process)) != 0 {
                let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
                let package = base_process_get_package_that_is_running(e, Ptr::new(process));
                flags |= e.call(ACTOR_PACKAGE_FLAGS, &args![actor, package]).u32();
            }
        }
        if e.call(ACTOR_TEST_FLAGS, &args![actor, flags]).bool() {
            flags |= 4;
        } else if !e.vcall(actor, ACTOR_SLOT_22C, &args![0u32]).bool() {
            flags &= !4;
        }
    }

    let player: u32 = e.global(PLAYER);
    if flags & 6 != 0 && !e.call(REF_PERSISTS, &args![reference]).bool() && reference != player {
        let mut located = 0u32;
        if e.vcall(reference, REFERENCE_IS_ACTOR, &args![]).bool() {
            located = reference;
        }
        let cell = if located != 0 {
            e.vcall(located, ACTOR_CELL_SLOT, &args![]).u32()
        } else {
            0
        };
        let worldspace = if located != 0 {
            e.vcall(located, ACTOR_WORLDSPACE_SLOT, &args![]).u32()
        } else {
            0
        };
        if worldspace != 0 || cell != 0 {
            let out = e.mem.alloc(0x14);
            let position_ptr = if located != 0 {
                e.vcall(located, REFERENCE_GET_LOCATION_SLOT, &args![out])
                    .u32()
            } else {
                DEFAULT_POSITION
            };
            let (x, y) = (e.mem.f32(position_ptr), e.mem.f32(position_ptr + 4));
            if cell != 0 {
                e.call(REF_GET_PARENT_CELL, &args![reference]);
            } else if worldspace != 0
                && worldspace == e.call(REF_GET_WORLDSPACE, &args![reference]).u32()
            {
                let cell_x = e.call(FLOAT_TO_INT, &args![x]).i32() >> 12;
                let cell_y = e.call(FLOAT_TO_INT, &args![y]).i32() >> 12;
                e.call(
                    WORLDSPACE_GET_CELL,
                    &args![worldspace, cell_x as u32, cell_y as u32],
                );
                e.call(REF_GET_PARENT_CELL, &args![reference]);
            }
            e.mem.free(out);
        } else {
            let parent = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
            if parent != 0 {
                let parent = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
                if !e.call(CELL_IS_INTERIOR, &args![parent]).bool() {
                    if located != 0 {
                        e.call(LOG_ERROR, &args![MSG_ACTOR_NO_EDITOR_LOCATION]);
                    }
                    let out = e.mem.alloc(0x0C);
                    e.vcall(reference, REFERENCE_GET_LOCATION_SLOT, &args![out]);
                    let (x, y) = (e.mem.f32(out), e.mem.f32(out + 4));
                    let cell_x = e.call(FLOAT_TO_INT, &args![x]).i32() >> 12;
                    let cell_y = e.call(FLOAT_TO_INT, &args![y]).i32() >> 12;
                    let worldspace = e.call(REF_GET_WORLDSPACE, &args![reference]).u32();
                    e.call(
                        WORLDSPACE_GET_CELL,
                        &args![worldspace, cell_x as u32, cell_y as u32],
                    );
                    e.call(REF_GET_PARENT_CELL, &args![reference]);
                    e.mem.free(out);
                }
            }
        }
    }
    flags
}

// Translated from 00859670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseProcess::GetPackageThatIsRunning` (Xbox PDB): the process's virtual
/// slot `0x22C`.
pub fn base_process_get_package_that_is_running(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), ACTOR_SLOT_22C, &args![]).u32()
}

// Translated from 00859690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Brings back what a save recorded for a cell: loads the cell itself
/// (`fn_00858730`) and then every reference the save lists for it (the
/// interior map is keyed by the cell's id and holds a list of form ids; the
/// exterior map is keyed by the cell's world space and holds the
/// references of each grid cell, of which the entries for this cell's
/// coordinates are used and removed), forgetting what it handled. Returns
/// true when anything was loaded. Does nothing unless `0047c850` is true.
pub fn fn_00859690(e: &mut Engine, this: Ptr<TESSaveLoadGame>, cell: Ptr) -> bool {
    if !game_unavailable(e, this) {
        return false;
    }
    let mut result = fn_00858730(e, this, cell);
    let out = e.mem.alloc(4);
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        let id = e.call(FORM_ID, &args![cell]).u32();
        let map = e.get(this, TESSaveLoadGame::m_pInteriorCellMap);
        if !e.call(MAP_GET_AT, &args![map, id, out]).bool() {
            e.mem.free(out);
            return result;
        }
        let list = e.mem.u32(out);
        let mut node = list;
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let id = e.mem.u32(slot);
            if id != 0 {
                fn_008598d0(e, this, id);
                result = true;
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        let id = e.call(FORM_ID, &args![cell]).u32();
        let map = e.get(this, TESSaveLoadGame::m_pInteriorCellMap);
        e.call(MAP_REMOVE_AT, &args![map, id]);
        destroy_list(e, list);
    } else {
        let worldspace = e.call(CELL_GET_WORLDSPACE, &args![cell]).u32();
        let id = e.call(FORM_ID, &args![worldspace]).u32();
        let map = e.get(this, TESSaveLoadGame::m_pExteriorCellMap);
        if !e.call(MAP_GET_AT, &args![map, id, out]).bool() {
            e.mem.free(out);
            return result;
        }
        let list = e.mem.u32(out);
        let mut node = list;
        let mut previous = 0u32;
        let cell_x = e.call(CELL_GET_X, &args![cell]).i32();
        let cell_y = e.call(CELL_GET_Y, &args![cell]).i32();
        let item_slot = e.mem.alloc(4);
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(slot);
            e.mem.set_u32(item_slot, item);
            if item != 0 && cell_x == e.mem.i32(item + 4) && cell_y == e.mem.i32(item + 8) {
                let form_id = e.mem.u32(item);
                fn_008598d0(e, this, form_id);
                result = true;
                if previous != 0 {
                    e.call(LIST_REMOVE, &args![previous, item_slot]);
                    node = e.call(LIST_NODE_NEXT, &args![previous]).u32();
                } else {
                    e.call(LIST_REMOVE_HEAD, &args![node]);
                }
                delete(e, item);
            } else {
                previous = node;
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
        }
        e.mem.free(item_slot);
        if e.call(LIST_NODE_IS_END, &args![list]).bool() {
            let worldspace = e.call(CELL_GET_WORLDSPACE, &args![cell]).u32();
            let id = e.call(FORM_ID, &args![worldspace]).u32();
            let map = e.get(this, TESSaveLoadGame::m_pExteriorCellMap);
            e.call(MAP_REMOVE_AT, &args![map, id]);
            if list != 0 {
                e.call(LIST_SCALAR_DELETE, &args![list, 1u32]);
            }
        }
    }
    e.mem.free(out);
    result
}

// Translated from 008598d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles one reference id of a cell's new-references list: when the
/// changes map has a `ChangeData` for it, reads its flags and logs "CELLS:
/// Reference in cell map has neither required flag." (the two flag tests
/// before it compare against 0 in this build, so the loading code that
/// follows them is not reachable and is not translated).
pub fn fn_008598d0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form_id: u32) {
    let changes = e.get(this, TESSaveLoadGame::m_pChanges);
    let change_data = fn_00855100(e, changes, form_id);
    if !change_data.is_null() {
        e.call(READ_WORD, &args![change_data]);
        e.call(LOG_ERROR, &args![MSG_CELL_REFERENCE_NO_FLAG]);
    }
}

// Translated from 00859a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes (or finds) the reference a `CreatedReferenceData` describes under
/// `form_id`. An existing form is kept only when it is a reference whose
/// base object is the record's bound object and the record is of type 0 or
/// 3; otherwise it is deleted (`DeleteForm`) and made again: type 0 and 3
/// build a `Character` (base form type 0x2A), a `Creature` (0x2B) or a plain
/// reference around the bound object, type 1 an arrow projectile, type 2 one
/// of three magic projectiles chosen by the record's `iBoundID`; each gets
/// its form id through virtual slot `0x128`. A record whose bound object no
/// longer exists logs an error and gives null, an unknown type logs one and
/// gives null too (the form is then not made). Returns the form, after
/// `0046a010(form, 1)`. Not translated: the exception frame.
pub fn fn_00859a90(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form_id: u32,
    record: Ptr<CreatedReferenceData>,
) -> Ptr {
    let scope = scope_enter_kind(e, 0x31, 0x133B);
    let mut form = e.call(LOOKUP_FORM, &args![form_id]).u32();
    let mut bound = 0u32;
    let kind = e.get(record, CreatedReferenceData::eType);
    let bound_id = e.get(record, CreatedReferenceData::iBoundID);
    if kind != 2 {
        let object = e.call(LOOKUP_FORM, &args![bound_id]).u32();
        bound = dynamic_cast(e, object, RTTI_FORM, RTTI_BOUND_OBJECT);
        if bound == 0 {
            e.call(
                LOG_ERROR,
                &args![MSG_BOUND_OBJECT_MISSING, bound_id, form_id],
            );
            scope_leave(e, scope);
            return Ptr::NULL;
        }
    }
    if form != 0 {
        let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
        let keep = reference != 0
            && (kind == 0 || kind == 3)
            && e.call(REFERENCE_GET_BASE, &args![reference]).u32() == bound;
        if !keep {
            tes_save_load_game_delete_form(e, this, Ptr::new(form));
            form = 0;
        }
    }
    if form == 0 {
        if kind == 0 || kind == 3 {
            let base_type = e.call(FORM_TYPE, &args![bound]).u32();
            form = match base_type {
                0x2A => construct_form(e, 0x1C8, CHARACTER_CONSTRUCT),
                0x2B => construct_form(e, 0x1C0, CREATURE_CONSTRUCT),
                _ => construct_form(e, 0x68, REFERENCE_CONSTRUCT),
            };
            let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
            e.call(REF_SET_BASE, &args![reference, bound]);
            e.vcall(form, FORM_SET_FORM_ID, &args![form_id, 1u32]);
        } else if kind == 1 {
            form = construct_form(e, 0xC8, ARROW_PROJECTILE_CONSTRUCT);
            let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
            e.call(REF_SET_BASE, &args![reference, bound]);
            e.vcall(form, FORM_SET_FORM_ID, &args![form_id, 1u32]);
        } else if kind == 2 {
            form = match bound_id {
                0 => construct_form(e, 0xC4, MAGIC_PROJECTILE_CONSTRUCT_A),
                3 => construct_form(e, 0xD0, MAGIC_PROJECTILE_CONSTRUCT_B),
                1 => construct_form(e, 0xD8, MAGIC_PROJECTILE_CONSTRUCT_C),
                _ => 0,
            };
            e.vcall(form, FORM_SET_FORM_ID, &args![form_id, 1u32]);
        } else {
            let location = e.mem.u32(record.addr() + 8);
            e.call(
                LOG_ERROR,
                &args![MSG_INVALID_CREATED_TYPE, kind, form_id, bound_id, location],
            );
        }
    }
    if form != 0 {
        e.call(FORM_FINISH, &args![form, 1u32]);
    }
    scope_leave(e, scope);
    Ptr::new(form)
}

/// `new` of `size` bytes and, when the allocation worked, the constructor
/// at `construct` on it; the constructed object (the constructor returns
/// `this`) or 0.
pub(crate) fn construct_form(e: &mut Engine, size: u32, construct: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block != 0 {
        e.call(construct, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 00859f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a moved reference from the plugins that have it. The location is
/// the record's original location id (or its `RefData` location id when
/// that is 0); looked up it is a cell, or a world space (then the record's
/// original position says which grid cell). For each plugin file of the
/// cell or world space that contains the cell and the reference, the
/// reference is made (`CreateReference`, with the record type of the plugin
/// file) and loaded from the file; the last one is the result. With no
/// result an error is logged and null returned. The reference's post-create
/// hook (virtual slot `0x88`) runs; an actor is placed in the cell with its
/// position and angle, other references get their extra data starting
/// position and rotation set.
pub fn fn_00859f20(
    e: &mut Engine,
    _this: Ptr,
    form_id: u32,
    moved: Ptr<MovedReferenceData>,
) -> Ptr {
    let mut result = 0u32;
    let mut location = e.get(moved, MovedReferenceData::iOriginalLocationID);
    if location == 0 {
        // MovedReferenceData::RefData.iLocationID
        location = e.mem.u32(moved.addr() + 0x10);
    }
    let location_form = e.call(LOOKUP_FORM, &args![location]).u32();
    let cell = dynamic_cast(e, location_form, RTTI_FORM, RTTI_CELL);
    let worldspace = dynamic_cast(e, location_form, RTTI_FORM, RTTI_WORLDSPACE);
    let (mut grid_x, mut grid_y) = (0i32, 0i32);
    if cell != 0 {
        let count = e.call(CELL_FILE_COUNT, &args![cell]).i32();
        for index in 0..count {
            let entry = e.call(FILE_OF_CELL, &args![cell, index as u32]).u32();
            let file = e.call(THREAD_SAFE_FILE, &args![entry]).u32();
            if e.call(FILE_HAS_CELL, &args![file, cell]).bool()
                && e.call(FILE_HAS_FORM, &args![file, form_id]).bool()
            {
                result = load_from_file(e, file);
            }
        }
    } else if worldspace != 0 {
        let count = e.call(CELL_FILE_COUNT, &args![worldspace]).i32();
        // MovedReferenceData::OriginalLoc.x and .y
        let x = e.mem.f32(moved.addr() + 4);
        grid_x = e.call(FLOAT_TO_INT, &args![x]).i32() >> 12;
        let y = e.mem.f32(moved.addr() + 8);
        grid_y = e.call(FLOAT_TO_INT, &args![y]).i32() >> 12;
        for index in 0..count {
            let entry = e.call(FILE_OF_CELL, &args![worldspace, index as u32]).u32();
            let file = e.call(THREAD_SAFE_FILE, &args![entry]).u32();
            if e.call(
                WORLDSPACE_FIND_CELL_IN_FILE,
                &args![worldspace, file, grid_x as u32, grid_y as u32],
            )
            .bool()
                && e.call(FILE_HAS_FORM, &args![file, form_id]).bool()
            {
                result = load_from_file(e, file);
            }
        }
    } else {
        e.call(LOG_ERROR, &args![MSG_NO_CELL_OR_WORLDSPACE]);
    }
    if result == 0 {
        e.call(
            LOG_ERROR,
            &args![
                MSG_REFERENCE_NOT_LOADED,
                form_id,
                location,
                grid_x as u32,
                grid_y as u32
            ],
        );
        return Ptr::NULL;
    }

    e.vcall(result, FORM_POST_CREATE, &args![]);
    let actor = dynamic_cast(e, result, RTTI_REFERENCE, RTTI_ACTOR);
    if actor != 0 {
        let rotation = e.call(REF_GET_ROTATION, &args![result]).u32();
        let angle_z = e.mem.f32(rotation + 8);
        let position = e.vcall(result, REFERENCE_GET_POSITION_SLOT, &args![]).u32();
        e.call(
            ACTOR_PLACE,
            &args![actor, worldspace, cell, position, angle_z],
        );
    } else {
        let position = e.vcall(result, REFERENCE_GET_POSITION_SLOT, &args![]).u32();
        let words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        let scratch = e.mem.alloc(0x10);
        let list = e.call(REF_GET_EXTRA_LIST, &args![result]).u32();
        e.call(
            EXTRA_SET_STARTING_POSITION,
            &args![list, scratch, result, words[0], words[1], words[2]],
        );
        let rotation = e.call(REF_GET_ROTATION, &args![result]).u32();
        let words = [
            e.mem.u32(rotation),
            e.mem.u32(rotation + 4),
            e.mem.u32(rotation + 8),
        ];
        let list = e.call(REF_GET_EXTRA_LIST, &args![result]).u32();
        e.call(
            EXTRA_SET_STARTING_ROTATION,
            &args![list, scratch, result, words[0], words[1], words[2]],
        );
        e.mem.free(scratch);
    }
    Ptr::new(result)
}

/// Makes the reference of the plugin file's current record (its record type
/// byte, `00472660`) and loads it from the file; returns the reference.
pub(crate) fn load_from_file(e: &mut Engine, file: u32) -> u32 {
    let record_type = e.call(FILE_RECORD_TYPE, &args![file]).u8();
    let reference = e
        .call(CREATE_REFERENCE, &args![record_type as u32, 1u32])
        .u32();
    e.call(LOAD_FORM_FROM_FILE, &args![reference, file]);
    reference
}

// Translated from 0085a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a form of `form_type` (`00670b90`) and gives it `form_id` (virtual
/// slot `0x128`) after `fn_0085a290` has dealt with an existing form of that
/// id. Returns the new form.
pub fn fn_0085a240(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form_id: u32, form_type: u8) -> Ptr {
    fn_0085a290(e, this, form_id);
    let form = e
        .call(CREATE_FORM_OF_TYPE, &args![form_type as u32])
        .ptr::<()>();
    e.vcall(form.addr(), FORM_SET_FORM_ID, &args![form_id, 1u32]);
    form
}

// Translated from 0085a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets rid of what is saved under `form_id`: an existing form is deleted
/// (`DeleteForm`), otherwise the id's changes are dropped from the changes
/// map (`fn_00855220`, force 1).
pub fn fn_0085a290(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form_id: u32) {
    let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
    if form != 0 {
        tes_save_load_game_delete_form(e, this, Ptr::new(form));
    } else {
        let changes = e.get(this, TESSaveLoadGame::m_pChanges);
        fn_00855220(e, changes, form_id, 1);
    }
}

// Translated from 0085a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::DeleteForm` (Xbox PDB): logs an error when it is not
/// called while loading (`0047c850` false), drops the form's changes
/// (`fn_008551f0`, force 1). A form already flagged deleted, or a cell, is
/// given a new form id from the data handler (virtual slot `0x128`).
/// Any other form is given id 0, cleared (`00483c70`), given an empty editor
/// id (slot `0x134`) and put on the deferred deletion list (once).
pub fn tes_save_load_game_delete_form(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) {
    if !game_unavailable(e, this) {
        e.call(LOG_ERROR, &args![MSG_DELETE_FORM_NOT_LOADING]);
    }
    let changes = e.get(this, TESSaveLoadGame::m_pChanges);
    fn_008551f0(e, changes, form, 1);
    let cell = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
    if e.call(FORM_IS_DELETED, &args![form]).bool() || cell != 0 {
        let handler: u32 = e.global(DATA_HANDLER);
        let id = e.call(DATA_HANDLER_GET_NEXT_ID, &args![handler]).u32();
        e.vcall(form.addr(), FORM_SET_FORM_ID, &args![id, 1u32]);
    } else {
        e.vcall(form.addr(), FORM_SET_FORM_ID, &args![0u32, 1u32]);
        e.call(FN_00483C70, &args![form]);
        e.vcall(form.addr(), FORM_SET_EDITOR_ID, &args![EMPTY_STRING]);
        tes_save_load_game_add_form_to_deferred_deletions_list(e, this, form);
    }
}

// Translated from 0085a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::AddFormToDeferredDeletionsList` (Xbox PDB): adds the
/// form to `m_DeferredDeleteList` unless it is already in it.
pub fn tes_save_load_game_add_form_to_deferred_deletions_list(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form: Ptr,
) {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, form.addr());
    let list = this.addr() + 0x34;
    if !e.call(LIST_CONTAINS, &args![list, slot]).bool() {
        e.call(LIST_ADD_HEAD, &args![list, slot]);
    }
    e.mem.free(slot);
}

// Translated from 0085a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the form from `m_DeferredDeleteList` when it is in it.
pub fn fn_0085a410(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, form.addr());
    let list = this.addr() + 0x34;
    if e.call(LIST_CONTAINS, &args![list, slot]).bool() {
        e.call(LIST_REMOVE, &args![list, slot]);
    }
    e.mem.free(slot);
}

// Translated from 0085a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::GetInitialDataSaveSize` (Xbox PDB): the bytes
/// `SaveInitialData` writes for the form: 0x1C (a `ReferenceData`) for a
/// reference whose flags have bit 1 or 2, otherwise 0. (The cell cases and
/// the other flag tests compare against 0 in this build and give 0.)
pub fn tes_save_load_game_get_initial_data_save_size(
    e: &mut Engine,
    _this: Ptr<TESSaveLoadGame>,
    form: Ptr,
    flags: u32,
) -> u16 {
    let reference = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_REFERENCE);
    if reference == 0 {
        dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
        return 0;
    }
    if flags & 6 != 0 {
        0x1C
    } else {
        0
    }
}

// Translated from 0085a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveInitialData` (Xbox PDB): for a reference writes
/// its `ReferenceData` (0x1C bytes: the numeric id of its world space or
/// parent cell, position, angle) into the current buffer when the flags have
/// bit 1 or 2. The location id comes from the parent cell's world space,
/// else the parent cell; a reference with neither logs why (non-persistent,
/// persistent without a cell, or an actor whose process level says it
/// should have one). A cell is cast and nothing is written. The code for the
/// flags the compiler folded to 0 is not translated.
pub fn tes_save_load_game_save_initial_data(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    form: Ptr,
    flags: u32,
) {
    let reference = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_REFERENCE);
    if reference == 0 {
        dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
        return;
    }
    let record: Ptr<ReferenceData> = Ptr::new(e.mem.alloc(0x1C));
    fn_008572f0(e, record);
    let parent_cell = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
    let mut worldspace = 0u32;
    if parent_cell != 0 {
        worldspace = e.call(CELL_GET_WORLDSPACE, &args![parent_cell]).u32();
    }
    if worldspace != 0 {
        let id = e.call(FORM_ID, &args![worldspace]).u32();
        let numeric = e.call(ADD_NUMERIC_ID, &args![this, id]).u32();
        e.set(record, ReferenceData::iLocationID, numeric);
    } else if parent_cell != 0 {
        let id = e.call(FORM_ID, &args![parent_cell]).u32();
        let numeric = e.call(ADD_NUMERIC_ID, &args![this, id]).u32();
        e.set(record, ReferenceData::iLocationID, numeric);
    } else {
        if !e.call(REF_PERSISTS, &args![reference]).bool() {
            let id = e.call(FORM_ID, &args![reference]).u32();
            e.call(LOG_ERROR, &args![MSG_NON_PERSISTENT_NO_CELL, id]);
        }
        if e.call(REF_PERSISTS, &args![reference]).bool() {
            let list = e.call(REF_GET_EXTRA_LIST, &args![reference]).u32();
            if e.call(EXTRA_GET_CELL_DATA, &args![list]).u32() == 0 {
                let id = e.call(FORM_ID, &args![reference]).u32();
                e.call(LOG_ERROR, &args![MSG_PERSISTENT_NO_CELL, id]);
            }
        }
        let mobile = dynamic_cast(e, reference, RTTI_REFERENCE, RTTI_MOBILE_OBJECT);
        if mobile != 0 && e.call(ACTOR_GET_PROCESS, &args![mobile]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
            if e.call(READ_FIELD_28, &args![process]).u32() == 0 {
                let id = e.call(FORM_ID, &args![reference]).u32();
                e.call(LOG_ERROR, &args![MSG_HIGH_PROCESS_NO_CELL, id]);
            } else {
                let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
                if e.call(READ_FIELD_28, &args![process]).u32() == 1 {
                    let id = e.call(FORM_ID, &args![reference]).u32();
                    e.call(LOG_ERROR, &args![MSG_MIDDLE_HIGH_PROCESS_NO_CELL, id]);
                }
            }
        }
    }
    let position = e.call(REF_GET_POSITION, &args![reference]).u32();
    let words = e.mem.bytes(position, 0x0C);
    e.mem.write(record.addr() + 4, &words);
    let rotation = e.call(REF_GET_ROTATION, &args![reference]).u32();
    let words = e.mem.bytes(rotation, 0x0C);
    e.mem.write(record.addr() + 0x10, &words);
    if flags & 6 != 0 {
        let game = game_singleton(e);
        fn_008579b0(e, game, record.cast(), 0x1C);
    }
    e.mem.free(record.addr());
}

// ---------------------------------------------------------------------------
// The third batch of functions: `0085ac30` to `00861640`

// Translated from 0085ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the initial data of a form that has just been loaded (the data
/// `SaveInitialData` wrote). A cell does nothing here but skip two bytes of
/// the buffer when the save is older than version 0x5B. A reference with
/// flag bit 1 or 2 reads its `ReferenceData` (0x1C bytes) from the current
/// buffer of the game (the singleton, not `this`), maps the location's
/// numeric id to a form id, gives the reference its angle and position,
/// and (unless it is the player) fixes a corrupt location (`008624e0`) and
/// moves it to the saved cell or world space: a cell moves it there when it
/// is not in it already; a world space picks the cell from the position
/// (4096 units per cell) and moves it unless it is already in that
/// exterior cell; with neither, a persistent reference is taken out of its
/// cell and its model request cancelled, a non-persistent one is only
/// logged. The code the compiler folded away (the flag tests against 0:
/// the `MovedReferenceData` read and the version 0x5B cell reads) is not
/// translated.
pub fn fn_0085ac30(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr, flags: u32) {
    let reference = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_REFERENCE);
    if reference == 0 {
        let cell = dynamic_cast(e, form.addr(), RTTI_FORM, RTTI_CELL);
        if cell != 0 {
            let singleton = game_singleton(e);
            e.call(CURRENT_VERSION, &args![singleton]);
            if e.call(CURRENT_VERSION, &args![singleton]).u8() < 0x5B {
                fn_00857bd0(e, this, 2);
            }
        }
        return;
    }
    if flags & 6 == 0 {
        return;
    }
    let data = e.mem.alloc(0x1C);
    fn_008572f0(e, Ptr::new(data));
    let singleton = game_singleton(e);
    fn_008579e0(e, singleton, Ptr::new(data), 0x1C);
    let location_id = e.mem.u32(data);
    let location_id = e.call(RESOLVE_NUMERIC_ID, &args![this, location_id]).u32();
    e.mem.set_u32(data, location_id);
    // The angle is passed by value (three words).
    let angle = [
        e.mem.u32(data + 0x10),
        e.mem.u32(data + 0x14),
        e.mem.u32(data + 0x18),
    ];
    e.call(FN_00575700, &args![reference, angle[0], angle[1], angle[2]]);
    let player: u32 = e.global(PLAYER);
    if reference == player {
        e.mem.free(data);
        return;
    }
    e.call(REF_SET_POSITION, &args![reference, data + 4]);
    e.call(FIX_CORRUPT_LOCATION, &args![this, reference]);
    let location = e.call(LOOKUP_FORM, &args![location_id]).u32();
    let cell = dynamic_cast(e, location, RTTI_FORM, RTTI_CELL);
    let worldspace = dynamic_cast(e, location, RTTI_FORM, RTTI_WORLDSPACE);
    if cell != 0 {
        if cell != e.call(REF_GET_PARENT_CELL, &args![reference]).u32() {
            e.call(REF_MOVE_TO_SPACE, &args![reference, cell, 0u32]);
        }
    } else if worldspace != 0 {
        let x = e.mem.f32(data + 4);
        let y = e.mem.f32(data + 8);
        let grid_x = e.call(FLOAT_TO_INT, &args![x]).i32() >> 12;
        let grid_y = e.call(FLOAT_TO_INT, &args![y]).i32() >> 12;
        let exterior_cell = e
            .call(WORLDSPACE_GET_CELL, &args![worldspace, grid_x, grid_y])
            .u32();
        let mut move_it = false;
        if e.call(REF_GET_PARENT_CELL, &args![reference]).u32() != 0 {
            let parent = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
            move_it = e.call(CELL_IS_INTERIOR, &args![parent]).bool();
        }
        if !move_it {
            if e.call(REF_GET_PARENT_CELL, &args![reference]).u32() == exterior_cell {
                if exterior_cell == 0 && !e.call(REF_PERSISTS, &args![reference]).bool() {
                    e.call(LOG_ERROR, &args![MSG_LOAD_NON_PERSISTENT]);
                }
            } else {
                move_it = true;
            }
        }
        if move_it {
            e.call(REF_MOVE_TO_SPACE, &args![reference, 0u32, worldspace]);
        }
    } else if !e.call(REF_PERSISTS, &args![reference]).bool() {
        e.call(LOG_ERROR, &args![MSG_PUT_NON_PERSISTENT]);
    } else {
        if e.call(REF_GET_PARENT_CELL, &args![reference]).u32() != 0 {
            let parent = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
            e.call(CELL_REMOVE_REFERENCE, &args![parent, reference]);
        }
        e.vcall(reference, REFERENCE_SLOT_228, &args![0u32]);
        let loader: u32 = e.global(MODEL_LOADER);
        e.call(MODEL_LOADER_CANCEL_REFERENCE, &args![loader, reference]);
        e.vcall(reference, REFERENCE_SLOT_1CC, &args![0u32, 1u32]);
    }
    e.mem.free(data);
}

// Translated from 0085b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash of what a reference holds in the five entries of the object its
/// virtual slot `0x1E8` gives: for each entry that exists and holds a form,
/// the form's id is mapped through the plugin table (`fn_00857c70`) and
/// folded in (`hash * 0x1003F + id`). 0 when the reference has no such
/// object.
pub fn fn_0085b170(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) -> u32 {
    let mut hash = 0u32;
    let object = e.vcall(form.addr(), REFERENCE_SLOT_1E8, &args![]).u32();
    if object != 0 {
        for index in 0..5u32 {
            let entry = e.call(OBJECT_ENTRY_AT, &args![object, index]).u32();
            if entry != 0 && e.mem.u32(entry) != 0 {
                let held = e.mem.u32(entry);
                let id = e.call(FORM_ID, &args![held]).u32();
                let mapped = fn_00857c70(e, this, id);
                // The game shifts and adds: (hash << 6) + id + (hash << 16) - hash.
                hash = hash
                    .wrapping_mul(ENTRY_HASH_MULTIPLIER)
                    .wrapping_add(mapped);
            }
        }
    }
    hash
}

// Translated from 0085b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SavePluginList` (Xbox PDB): writes the number of
/// compiled plugin files (a byte), then for each the length of its name
/// (a byte) and the name. When the save gathers statistics, the bytes
/// written are added as the extra stat "Plugin List".
pub fn tes_save_load_game_save_plugin_list(e: &mut Engine, this: Ptr<TESSaveLoadGame>, file: Ptr) {
    let handler: u32 = e.global(DATA_HANDLER);
    let count = e.call(DATA_HANDLER_FILE_COUNT, &args![handler]).u8();
    let mut start = 0u32;
    if !e.get(this, TESSaveLoadGame::m_pSaveLoadStats).is_null() {
        start = e.call(FILE_POSITION, &args![file]).u32();
    }
    // The count at +0, the current name's length at +1.
    let cells = e.mem.alloc(2);
    e.mem.set_u8(cells, count);
    fn_00857b50(e, this, file, Ptr::new(cells), 1);
    for index in 0..count {
        let compiled = e
            .call(
                DATA_HANDLER_GET_COMPILED_FILE,
                &args![handler, index as u32],
            )
            .u32();
        let name = e.call(COMPILED_FILE_NAME, &args![compiled]).u32();
        let length = e.call(STRLEN, &args![name]).u8();
        e.mem.set_u8(cells + 1, length);
        fn_00857b50(e, this, file, Ptr::new(cells + 1), 1);
        fn_00857b50(e, this, file, Ptr::new(name), length as u32);
    }
    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        let end = e.call(FILE_POSITION, &args![file]).u32();
        save_stats_add_extra_stat(
            e,
            stats,
            end.wrapping_sub(start),
            Ptr::new(LABEL_PLUGIN_LIST),
        );
    }
    e.mem.free(cells);
}

/// Pads the line in `scratch` with spaces up to `LINE_WIDTH` characters,
/// adds one more space, and appends it to `buffer` (the end of every line
/// `BuildChangesString` writes in its short form).
pub(crate) fn changes_pad_and_append(e: &mut Engine, buffer: u32, scratch: u32) {
    while e.call(STRLEN, &args![scratch]).u32() < LINE_WIDTH {
        e.call(STRING_CAT, &args![scratch, LINE_SIZE, SPACE]);
    }
    e.call(STRING_CAT, &args![scratch, LINE_SIZE, SPACE]);
    e.call(STRING_CAT, &args![buffer, CHANGES_BUFFER_SIZE, scratch]);
}

/// The size of the part of the form's changed data selected by `mask`
/// (virtual slot `0x50`) less `base`, as the lines of `BuildChangesString`
/// print it; 0 for no form.
pub(crate) fn changes_part_size(e: &mut Engine, form: u32, base: u32, mask: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    let size = e.vcall(form, FORM_GET_CHANGES_SIZE, &args![mask]).u16() as u32;
    size.wrapping_sub(base)
}

/// One line of the changes string: when `flags` has the line's bit, the
/// verbose text is appended to `buffer` or, in the short form, the line
/// `format` with the size of that part of the form's changes is built in
/// `scratch`, padded and appended.
#[allow(clippy::too_many_arguments)]
pub(crate) fn changes_line(
    e: &mut Engine,
    buffer: u32,
    scratch: u32,
    form: u32,
    base: u32,
    flags: u32,
    verbose: bool,
    line: &ChangeLine,
) {
    if flags & line.mask == 0 {
        return;
    }
    if verbose {
        e.call(
            STRING_CAT,
            &args![buffer, CHANGES_BUFFER_SIZE, line.verbose],
        );
        return;
    }
    let size = changes_part_size(e, form, base, line.mask);
    e.call(FORMAT, &args![scratch, LINE_SIZE, line.format, size]);
    changes_pad_and_append(e, buffer, scratch);
}

/// A line without a size: the short form pads and appends whatever the
/// scratch line held (the quest stages and the topic's "said once" flag do
/// not build a line of their own).
pub(crate) fn changes_line_without_size(
    e: &mut Engine,
    buffer: u32,
    scratch: u32,
    flags: u32,
    verbose: bool,
    line: &ChangeLine,
) {
    if flags & line.mask == 0 {
        return;
    }
    if verbose {
        e.call(
            STRING_CAT,
            &args![buffer, CHANGES_BUFFER_SIZE, line.verbose],
        );
    } else {
        changes_pad_and_append(e, buffer, scratch);
    }
}

// Translated from 0085b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::BuildChangesString` (Xbox PDB): writes into `buffer`
/// (500 characters) a description of what changed in the form, for the save
/// statistics. In the short form (`verbose` zero) every line is
/// `Name(size)` padded to 25 characters, with the size of that part of the
/// changed data (virtual slot `0x50` of the form, less the size of the
/// part the form always has); in the verbose form each line is the name of
/// the change flag. First comes the process level of a mobile object (or the
/// size of the base data of a form that is none), then the lines of the
/// form's type; the flag bits are first made consistent by `CheckFlags`.
/// The lines whose flag mask is 0 in this build (their tests are folded to
/// `flags & 0`: the actor base health, attributes, factions, modifiers and
/// AI data, the combat style and fame, the cell creation flag, the
/// created and cell-changed flags of a reference, its Havok, Oblivion and
/// disable state flags, its extra data lines and the actor package lines)
/// can never be reached and are not translated.
pub fn tes_save_load_game_build_changes_string(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    buffer: Ptr,
    form: Ptr,
    flags: u32,
    form_type: u8,
    verbose: u8,
) {
    let (buffer, form, verbose) = (buffer.addr(), form.addr(), verbose != 0);
    let scratch = e.mem.alloc(LINE_SIZE + 2);
    e.call(
        STRING_COPY,
        &args![buffer, CHANGES_BUFFER_SIZE, EMPTY_STRING],
    );
    let mut flags = flags;
    if form != 0 {
        flags = tes_save_load_game_check_flags(e, this, Ptr::new(form), flags);
    }
    let mut base = 0u32;
    if !verbose && form != 0 {
        base = e.vcall(form, FORM_GET_CHANGES_SIZE, &args![0u32]).u16() as u32;
    }
    let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
    let mobile = dynamic_cast(e, form, RTTI_FORM, RTTI_MOBILE_OBJECT);
    dynamic_cast(e, form, RTTI_FORM, RTTI_ACTOR);

    if mobile != 0 {
        let mut level = 0xFFFF_FFFFu32;
        if e.call(ACTOR_GET_PROCESS, &args![mobile]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
            level = e.call(READ_FIELD_28, &args![process]).u32();
        }
        // The text for the process level (-1 none, 0 high, 1 mid high, 2 mid
        // low, 3 low); any other level leaves the scratch line as it was.
        let text = match level.wrapping_add(1) {
            0 => Some(PROCESS_LEVEL_NONE),
            1 => Some(PROCESS_LEVEL_HIGH),
            2 => Some(PROCESS_LEVEL_MID_HIGH),
            3 => Some(PROCESS_LEVEL_MID_LOW),
            4 => Some(PROCESS_LEVEL_LOW),
            _ => None,
        };
        if let Some((verbose_text, format)) = text {
            if verbose {
                e.call(FORMAT, &args![scratch, LINE_SIZE, verbose_text]);
            } else {
                e.call(FORMAT, &args![scratch, LINE_SIZE, format, base]);
            }
        }
        if !verbose {
            while e.call(STRLEN, &args![scratch]).u32() < LINE_WIDTH {
                e.call(STRING_CAT, &args![scratch, LINE_SIZE, SPACE]);
            }
            e.call(STRING_CAT, &args![scratch, LINE_SIZE, SPACE]);
        }
        e.call(STRING_CAT, &args![buffer, CHANGES_BUFFER_SIZE, scratch]);
    } else if base != 0 && !verbose {
        e.call(FORMAT, &args![scratch, LINE_SIZE, FORMAT_BASE, base]);
        changes_pad_and_append(e, buffer, scratch);
    }

    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_FORM_FLAGS,
    );

    if form_type == TYPE_NPC || form_type == TYPE_CREATURE {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_BASE_DATA,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_SPELL_LIST,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_FULL_NAME,
        );
        let skills = if form_type == TYPE_NPC {
            &LINE_NPC_SKILLS
        } else {
            &LINE_CREATURE_SKILLS
        };
        changes_line(e, buffer, scratch, form, base, flags, verbose, skills);
    } else if form_type == TYPE_QUEST {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_QUEST_FLAGS,
        );
        changes_line_without_size(e, buffer, scratch, flags, verbose, &LINE_QUEST_STAGES);
        let line = &LINE_QUEST_SCRIPT;
        if flags & line.mask != 0 {
            if verbose {
                e.call(
                    STRING_CAT,
                    &args![buffer, CHANGES_BUFFER_SIZE, line.verbose],
                );
            } else {
                let quest = dynamic_cast(e, form, RTTI_FORM, RTTI_QUEST);
                if quest == 0 || e.call(QUEST_HAS_SCRIPT_LOCALS, &args![quest]).u32() != 0 {
                    let size = changes_part_size(e, form, base, line.mask);
                    e.call(FORMAT, &args![scratch, LINE_SIZE, line.format, size]);
                } else if form != 0 {
                    e.call(FORMAT, &args![scratch, LINE_SIZE, FORMAT_QUEST_SCRIPT_GONE]);
                }
                changes_pad_and_append(e, buffer, scratch);
            }
        }
    } else if form_type == TYPE_TOPIC {
        changes_line_without_size(e, buffer, scratch, flags, verbose, &LINE_TOPIC_SAID_ONCE);
    } else if form_type == TYPE_PACKAGE {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_PACKAGE_NEVER_RUN,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_PACKAGE_WAITING,
        );
    } else if form_type == TYPE_CELL {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_CELL_FLAGS,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_CELL_SEEN_DATA,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_CELL_DETACH_TIME,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_CELL_OWNERSHIP,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_CELL_FULL_NAME,
        );
    } else if form_type == TYPE_FACTION {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_FACTION_FLAGS,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_FACTION_REACTIONS,
        );
    } else if form_type == TYPE_BOOK {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_BOOK_SKILL,
        );
    } else if REFERENCE_TYPES.contains(&form_type) {
        changes_reference_lines(
            e, this, buffer, scratch, form, reference, base, flags, form_type, verbose,
        );
    }
    e.mem.free(scratch);
}

/// The lines of a reference of one of the `REFERENCE_TYPES` (the second
/// half of `BuildChangesString`): moved, Havok moved, animation, scale and
/// inventory; then, for the two actor types, the actor modifier lines and
/// the life state; for the other types the lock, the empty flag and (for a
/// reference that is a door, or has no base object test) the teleport, open
/// state and default open state.
#[allow(clippy::too_many_arguments)]
pub(crate) fn changes_reference_lines(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    buffer: u32,
    scratch: u32,
    form: u32,
    reference: u32,
    base: u32,
    flags: u32,
    form_type: u8,
    verbose: bool,
) {
    // Moved: the size is what `GetInitialDataSaveSize` gives for the move.
    if flags & 2 != 0 {
        if verbose {
            e.call(
                STRING_CAT,
                &args![buffer, CHANGES_BUFFER_SIZE, TEXT_REFR_MOVE],
            );
        } else {
            let size = if form != 0 {
                tes_save_load_game_get_initial_data_save_size(e, this, Ptr::new(form), 2) as u32
            } else {
                0
            };
            e.call(FORMAT, &args![scratch, LINE_SIZE, FORMAT_REFR_MOVE, size]);
            changes_pad_and_append(e, buffer, scratch);
        }
    }
    // Havok moved: with the move flag too, only the Havok part; otherwise
    // the initial data's size is added.
    if flags & 4 != 0 {
        if verbose {
            e.call(
                STRING_CAT,
                &args![buffer, CHANGES_BUFFER_SIZE, TEXT_REFR_HAVOK_MOVE],
            );
        } else {
            let size = if form == 0 {
                0
            } else if flags & 2 != 0 {
                changes_part_size(e, form, base, 4)
            } else {
                let initial =
                    tes_save_load_game_get_initial_data_save_size(e, this, Ptr::new(form), 4)
                        as u32;
                let part = e.vcall(form, FORM_GET_CHANGES_SIZE, &args![4u32]).u16() as u32;
                initial.wrapping_add(part).wrapping_sub(base)
            };
            e.call(
                FORMAT,
                &args![scratch, LINE_SIZE, FORMAT_REFR_HAVOK_MOVE, size],
            );
            changes_pad_and_append(e, buffer, scratch);
        }
    }
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_REFR_ANIMATION,
    );
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_REFR_SCALE,
    );
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_REFR_INVENTORY,
    );

    if ACTOR_REFERENCE_TYPES.contains(&form_type) {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_DAMAGE_MODIFIERS,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_SCRIPT_MODIFIERS,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_TEMP_MODIFIERS,
        );
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_ACTOR_DISPOSITION,
        );
        // Life state: the size is measured with the bits 0x400 and
        // `flags & 4` against the part selected by `flags & 4` alone.
        if flags & 0x400 != 0 {
            if verbose {
                e.call(
                    STRING_CAT,
                    &args![buffer, CHANGES_BUFFER_SIZE, TEXT_ACTOR_LIFE_STATE],
                );
            } else {
                let without = if form != 0 {
                    e.vcall(form, FORM_GET_CHANGES_SIZE, &args![flags & 4])
                        .u16() as u32
                } else {
                    0
                };
                let with = (flags & 4) + 0x400;
                let size = if form != 0 {
                    (e.vcall(form, FORM_GET_CHANGES_SIZE, &args![with]).u16() as u32)
                        .wrapping_sub(without)
                } else {
                    0
                };
                e.call(
                    FORMAT,
                    &args![scratch, LINE_SIZE, FORMAT_ACTOR_LIFE_STATE, size],
                );
                changes_pad_and_append(e, buffer, scratch);
            }
        }
        return;
    }

    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_OBJECT_LOCK,
    );
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_OBJECT_EMPTY,
    );
    // The teleport line is only for a reference with no reference cast (it
    // cannot be, as every type here is one) or whose base object has the
    // type of a door.
    let mut teleport = reference == 0;
    if reference != 0 && e.call(REFERENCE_GET_BASE, &args![reference]).u32() != 0 {
        let base_object = e.call(REFERENCE_GET_BASE, &args![reference]).u32();
        teleport = e.call(FORM_TYPE, &args![base_object]).u32() == BASE_TYPE_DOOR;
    }
    if teleport {
        changes_line(
            e,
            buffer,
            scratch,
            form,
            base,
            flags,
            verbose,
            &LINE_DOOR_TELEPORT,
        );
    }
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_OPEN_STATE,
    );
    changes_line(
        e,
        buffer,
        scratch,
        form,
        base,
        flags,
        verbose,
        &LINE_OPEN_DEFAULT_STATE,
    );
}

// Translated from 0085ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a block of `size + 2` bytes whose first two bytes are `size`
/// (the blocks of the animation, character controller and Havok maps);
/// `_this` is not used.
pub fn fn_0085ef80(e: &mut Engine, _this: Ptr, size: u16) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![size as u32 + 2]).u32();
    let header = e.mem.alloc(2);
    e.mem.set_u16(header, size);
    e.call(MEMCPY, &args![block, header, 2u32]);
    e.mem.free(header);
    Ptr::new(block)
}

// Translated from 0085efc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a block of `size` bytes from the current buffer into a new sized
/// block (`fn_0085ef80`) and stores it in the pointer map under `key`.
pub fn fn_0085efc0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, map: Ptr, key: u32, size: u16) {
    let block = fn_0085ef80(e, this.cast(), size);
    fn_008579e0(e, this, block.byte_add(2), size as u32);
    e.call(MAP_SET_AT, &args![map, key, block]);
}

// Translated from 0085f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps an animation block for the form: reads `size` bytes into the
/// animation map under the form's id, or under 0 when the map already has
/// that id.
pub fn fn_0085f010(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr, size: u16) {
    let mut key = e.call(FORM_ID, &args![form]).u32();
    let out = e.mem.alloc(4);
    let map = e.get(this, TESSaveLoadGame::m_pAnimationMap);
    if e.call(MAP_GET_AT, &args![map, key, out]).bool() {
        key = 0;
    }
    e.mem.free(out);
    let map = e.get(this, TESSaveLoadGame::m_pAnimationMap);
    fn_0085efc0(e, this, map, key, size);
}

// Translated from 0085f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives every reference of the animation map its saved animation block
/// (`fn_0085f150`) and empties the map: key 0 is the player. An entry whose
/// reference is gone, or has no 3D node yet, is only dropped. Each block is
/// freed.
pub fn fn_0085f070(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let map = e.get(this, TESSaveLoadGame::m_pAnimationMap);
    for_each_entry(e, map, |e, key, block| {
        let reference = if key == 0 {
            e.global::<u32>(PLAYER)
        } else {
            let form = e.call(LOOKUP_FORM, &args![key]).u32();
            dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE)
        };
        if reference != 0 && e.call(REF_GET_NODE, &args![reference]).u32() != 0 {
            fn_0085f150(e, this, Ptr::new(reference), key, Ptr::new(block));
        }
        delete(e, block);
        let map = e.get(this, TESSaveLoadGame::m_pAnimationMap);
        e.call(MAP_REMOVE_AT, &args![map, key]);
    });
}

// Translated from 0085f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads one animation block (`buffer`) for the reference: the player's
/// animation object (the first-person one when `key` is 0), else the one the
/// reference's virtual slot `0x1E4` gives. For a mobile object with a
/// non-zero key the high process object's two values (slots `0x3E4` and
/// `0x3E8`) are read first. With the game's buffer pointed at `buffer`, two
/// bytes are skipped, the animation object loads the reference
/// (`0049a1c0`), and the buffer pointer is put back; the process object is
/// then given its values back (the second adjusted by `00491040`) through
/// slot `0x3EC`.
pub fn fn_0085f150(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    reference: Ptr,
    key: u32,
    buffer: Ptr,
) {
    let player: u32 = e.global(PLAYER);
    let animation = if reference.addr() == player {
        let first_person = if key == 0 { 1u32 } else { 0u32 };
        e.call(PLAYER_GET_ANIMATION, &args![player, first_person])
            .u32()
    } else {
        e.vcall(reference.addr(), REFERENCE_SLOT_1E4, &args![])
            .u32()
    };
    let mobile = dynamic_cast(e, reference.addr(), RTTI_REFERENCE, RTTI_MOBILE_OBJECT);
    let mut process_object = 0u32;
    let mut first = 0xFFFF_FFFFu32;
    let mut second = 0u32;
    if mobile != 0 && key != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
        process_object = dynamic_cast(e, process, RTTI_BASE_PROCESS, RTTI_HIGH_PROCESS);
        if process_object != 0 {
            first = e.vcall(process_object, PROCESS_SLOT_3E4, &args![]).u32();
            second = e.vcall(process_object, PROCESS_SLOT_3E8, &args![]).u32();
        }
    }
    let saved = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.set(this, TESSaveLoadGame::m_pBuffer, buffer);
    let skipped = e.mem.alloc(4);
    let singleton = game_singleton(e);
    fn_008579e0(e, singleton, Ptr::new(skipped), 2);
    e.mem.free(skipped);
    if animation != 0 {
        e.call(ANIMATION_LOAD, &args![animation, reference]);
    }
    e.set(this, TESSaveLoadGame::m_pBuffer, saved);
    if process_object != 0 {
        if second != 0 {
            second = e
                .call(ANIMATION_ADJUST, &args![animation, second.wrapping_sub(8)])
                .u32();
        }
        e.vcall(process_object, PROCESS_SLOT_3EC, &args![first, second]);
    }
}

// Translated from 0085f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps an attached-animation block for the form: reads `size` bytes into
/// the attached animation map under the form's id.
pub fn fn_0085f2b0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr, size: u16) {
    let key = e.call(FORM_ID, &args![form]).u32();
    let map = e.get(this, TESSaveLoadGame::m_pAttachedAnimationMap);
    fn_0085efc0(e, this, map, key, size);
}

// Translated from 0085f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the attached-animation blocks (`fn_0085f3d0`) of the references
/// that have a 3D node, and empties the map. A reference without a node
/// yet has its changes flags set back (`fn_00855010` with 0, on the new
/// changes map when there is one) and its entry dropped without freeing
/// the block; an id that is no reference only frees the block.
pub fn fn_0085f2e0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let map = e.get(this, TESSaveLoadGame::m_pAttachedAnimationMap);
    for_each_entry(e, map, |e, key, block| {
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
        if reference != 0 {
            if e.call(REF_GET_NODE, &args![reference]).u32() == 0 {
                requeue_changes(e, this, reference);
                let map = e.get(this, TESSaveLoadGame::m_pAttachedAnimationMap);
                e.call(MAP_REMOVE_AT, &args![map, key]);
                return;
            }
            fn_0085f3d0(e, this, Ptr::new(reference), Ptr::new(block));
        }
        delete(e, block);
        let map = e.get(this, TESSaveLoadGame::m_pAttachedAnimationMap);
        e.call(MAP_REMOVE_AT, &args![map, key]);
    });
}

/// `fn_00855010(changes, reference, 0)` on the new changes map when the
/// game has one, else on the changes map (the animation passes that
/// `fn_0085f2e0` and `fn_0085f5d0` cannot finish).
pub(crate) fn requeue_changes(e: &mut Engine, this: Ptr<TESSaveLoadGame>, reference: u32) {
    let new_changes = e.get(this, TESSaveLoadGame::m_pNewChanges);
    let changes = if new_changes.is_null() {
        e.get(this, TESSaveLoadGame::m_pChanges)
    } else {
        new_changes
    };
    fn_00855010(e, changes, Ptr::new(reference), 0);
}

// Translated from 0085f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one attached-animation block (`buffer`) of the reference: with the
/// game's buffer pointed at it two bytes are skipped (the singleton's read)
/// and `004534f0(reference, 0)`, which does nothing, is called.
pub fn fn_0085f3d0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, reference: Ptr, buffer: Ptr) {
    let saved = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.set(this, TESSaveLoadGame::m_pBuffer, buffer);
    let skipped = e.mem.alloc(4);
    let singleton = game_singleton(e);
    fn_008579e0(e, singleton, Ptr::new(skipped), 2);
    e.mem.free(skipped);
    e.call(SET_LOADING_STATE, &args![reference, 0u32]);
    e.set(this, TESSaveLoadGame::m_pBuffer, saved);
}

// Translated from 0085f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps a character controller block for the form: reads `size` bytes into
/// the character controller map under the form's id.
pub fn fn_0085f420(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr, size: u16) {
    let key = e.call(FORM_ID, &args![form]).u32();
    let map = e.get(this, TESSaveLoadGame::m_pCharControllerMap);
    fn_0085efc0(e, this, map, key, size);
}

// Translated from 0085f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the character controller blocks and empties the map. For each
/// mobile object whose process is a high process, the game's buffer is
/// pointed at the block, two bytes are skipped, and the process object's
/// slot `0x28C` decides: true loads the controller (`00926690`); otherwise
/// nothing is done when the mobile object passes both tests (`00440d80`,
/// `00440da0`), else "does not have a character controller to load" is
/// logged with its name (virtual slot `0x130`) and id. Each block is freed.
pub fn fn_0085f450(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let map = e.get(this, TESSaveLoadGame::m_pCharControllerMap);
    for_each_entry(e, map, |e, key, block| {
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        let mobile = dynamic_cast(e, form, RTTI_FORM, RTTI_MOBILE_OBJECT);
        if mobile != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
            let process_object = dynamic_cast(e, process, RTTI_BASE_PROCESS, RTTI_HIGH_PROCESS);
            if process_object != 0 {
                let saved = e.get(this, TESSaveLoadGame::m_pBuffer);
                e.set(this, TESSaveLoadGame::m_pBuffer, Ptr::new(block));
                let skipped = e.mem.alloc(4);
                let singleton = game_singleton(e);
                fn_008579e0(e, singleton, Ptr::new(skipped), 2);
                e.mem.free(skipped);
                if e.vcall(process_object, PROCESS_SLOT_28C, &args![]).u32() != 0 {
                    e.call(LOAD_CHAR_CONTROLLER, &args![process_object, mobile]);
                } else {
                    let mut has_controller = false;
                    if e.call(MOBILE_TEST_A, &args![mobile]).bool() {
                        has_controller = e.call(MOBILE_TEST_B, &args![mobile]).bool();
                    }
                    if !has_controller {
                        let id = e.call(FORM_ID, &args![mobile]).u32();
                        let name = e.vcall(mobile, FORM_GET_DESCRIPTION, &args![]).u32();
                        e.call(LOG_ERROR, &args![MSG_NO_CHAR_CONTROLLER, name, id]);
                    }
                }
                e.set(this, TESSaveLoadGame::m_pBuffer, saved);
            }
        }
        delete(e, block);
        let map = e.get(this, TESSaveLoadGame::m_pCharControllerMap);
        e.call(MAP_REMOVE_AT, &args![map, key]);
    });
}

// Translated from 0085f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps a Havok data block for the form: reads `size` bytes into the
/// Havok data map under the form's id.
pub fn fn_0085f5a0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr, size: u16) {
    let key = e.call(FORM_ID, &args![form]).u32();
    let map = e.get(this, TESSaveLoadGame::m_pHavokDataMap);
    fn_0085efc0(e, this, map, key, size);
}

// Translated from 0085f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the Havok data blocks (`fn_0085f6c0`) of the references that have a
/// 3D node and empties the map; handled like `fn_0085f2e0` for a reference
/// without a node (changes flags set back, entry dropped, block kept) and
/// for an id that is no reference (block freed).
pub fn fn_0085f5d0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let map = e.get(this, TESSaveLoadGame::m_pHavokDataMap);
    for_each_entry(e, map, |e, key, block| {
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        let reference = dynamic_cast(e, form, RTTI_FORM, RTTI_REFERENCE);
        if reference != 0 {
            if e.call(REF_GET_NODE, &args![reference]).u32() == 0 {
                requeue_changes(e, this, reference);
                let map = e.get(this, TESSaveLoadGame::m_pHavokDataMap);
                e.call(MAP_REMOVE_AT, &args![map, key]);
                return;
            }
            fn_0085f6c0(e, this, Ptr::new(reference), Ptr::new(block));
        }
        delete(e, block);
        let map = e.get(this, TESSaveLoadGame::m_pHavokDataMap);
        e.call(MAP_REMOVE_AT, &args![map, key]);
    });
}

// Translated from 0085f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one Havok data block (`buffer`) of the reference: with the game's
/// buffer pointed at it a 16-bit value is read (the singleton's read) and
/// given to the reference (`00560530`).
pub fn fn_0085f6c0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, reference: Ptr, buffer: Ptr) {
    let saved = e.get(this, TESSaveLoadGame::m_pBuffer);
    e.set(this, TESSaveLoadGame::m_pBuffer, buffer);
    let value = e.mem.alloc(4);
    let singleton = game_singleton(e);
    fn_008579e0(e, singleton, Ptr::new(value), 2);
    let value_read = e.mem.u16(value) as u32;
    e.mem.free(value);
    e.call(REFERENCE_APPLY_HAVOK_BLOCK, &args![reference, value_read]);
    e.set(this, TESSaveLoadGame::m_pBuffer, saved);
}

// Translated from 0085f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the form to `m_QueuedFormList` unless it is in it.
pub fn fn_0085f710(e: &mut Engine, this: Ptr<TESSaveLoadGame>, form: Ptr) {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, form.addr());
    let list = this.addr() + 0x68;
    if !e.call(LIST_CONTAINS, &args![list, slot]).bool() {
        e.call(LIST_ADD_HEAD, &args![list, slot]);
    }
    e.mem.free(slot);
}

// Translated from 0085f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Goes through `m_QueuedFormList`: for each arrow projectile, the 3D node
/// of the form `fn_0085f810` gives is, when it has one, given a default
/// 12-byte object (`0043d410(object, 0.0, 0, 0)`, `00a59c60(node, object)`),
/// and the projectile is finished (`008cd210`). Then the list is emptied.
pub fn fn_0085f750(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let list = this.addr() + 0x68;
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            let arrow = dynamic_cast(e, item, RTTI_FORM, RTTI_ARROW_PROJECTILE);
            if arrow != 0 {
                let target = fn_0085f810(e, Ptr::new(arrow));
                if target != 0 {
                    let scene_node = e.call(REF_GET_NODE, &args![target]).u32();
                    if scene_node != 0 {
                        let object = e.mem.alloc(12);
                        e.call(FN_0043D410, &args![object, 0.0f32, 0u32, 0u32]);
                        e.call(FN_00A59C60, &args![scene_node, object]);
                        e.mem.free(object);
                    }
                }
                e.call(ARROW_PROJECTILE_FINISH, &args![arrow]);
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(LIST_REMOVE_ALL, &args![list]);
}

// Translated from 0085f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `+0x28` of the object the word at `+0x88` of `this` points
/// to (for an arrow projectile, what it is stuck in), or 0 without one.
pub fn fn_0085f810(e: &mut Engine, this: Ptr) -> u32 {
    let held = e.mem.u32(this.addr() + 0x88);
    if held == 0 {
        0
    } else {
        e.mem.u32(held + 0x28)
    }
}

// Translated from 0085f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes the loads that wait for the references' 3D nodes, when
/// `0047c850` is true (in this build it is never): the animations, attached
/// animations and character controllers, then (when `settle`) the process
/// lists and `TES` are given 0.0 and the Havok world of the player's cell
/// has all its islands deactivated, then the Havok data and the queued
/// arrow projectiles.
pub fn fn_0085f850(e: &mut Engine, this: Ptr<TESSaveLoadGame>, settle: bool) {
    if !game_unavailable(e, this) {
        return;
    }
    fn_0085f070(e, this);
    fn_0085f2e0(e, this);
    fn_0085f450(e, this);
    if settle {
        e.call(
            PROCESS_LISTS_FN_00978420,
            &args![PROCESS_LISTS, 0.0f32, 0u32],
        );
        let tes: u32 = e.global(TES_OBJECT);
        e.call(TES_FN_00453550, &args![tes, 0.0f32]);
        let player: u32 = e.global(PLAYER);
        let cell = e.call(REF_GET_PARENT_CELL, &args![player]).u32();
        if cell != 0 {
            let world = e.call(CELL_GET_PHYSICS_WORLD, &args![cell]).u32();
            if world != 0 {
                e.call(WORLD_DEACTIVATE_ALL_ISLANDS, &args![world]);
            }
        }
    }
    fn_0085f5d0(e, this);
    fn_0085f750(e, this);
}

// Translated from 0085f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the list of the saves in the save folder: drops the old list
/// (`fn_0085fbd0`), makes a new `BSSimpleList`, and for every non-empty
/// `*.ess` file in the folder (`FindFirstFileA` / `FindNextFileA`) makes a
/// `SaveGameFile` (`fn_0085fb40`, write mode 0, buffer size 0x20000) and
/// inserts it sorted by `fn_0085fc90` (newest first). The C++ exception
/// frame is not translated.
pub fn fn_0085f900(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    if !e.get(this, TESSaveLoadGame::m_pSaveGameList).is_null() {
        fn_0085fbd0(e, this);
    }
    let list = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list = if list != 0 {
        e.call(SIMPLE_LIST_CONSTRUCT, &args![list]).u32()
    } else {
        0
    };
    e.set(this, TESSaveLoadGame::m_pSaveGameList, Ptr::new(list));

    let pattern = e.mem.alloc(0x104);
    let find_data = e.mem.alloc(FIND_DATA_SIZE);
    let path = e.mem.alloc(0x104);
    let prefix = e.call(PATH_PREFIX, &args![]).u32();
    e.call(LSTRCPY_IMPORT, &args![pattern, prefix]);
    let folder = save_folder(e);
    e.call(LSTRCAT_IMPORT, &args![pattern, folder]);
    e.call(LSTRCAT_IMPORT, &args![pattern, SAVE_FILE_PATTERN]);
    let handle = e
        .call(FIND_FIRST_FILE_IMPORT, &args![pattern, find_data])
        .u32();
    if handle != 0xFFFF_FFFF {
        loop {
            if e.mem.u32(find_data + FIND_DATA_SIZE_HIGH) != 0
                || e.mem.u32(find_data + FIND_DATA_SIZE_LOW) != 0
            {
                let folder = save_folder(e);
                let prefix = e.call(PATH_PREFIX, &args![]).u32();
                e.call(
                    FORMAT,
                    &args![
                        path,
                        0x104u32,
                        FORMAT_THREE_STRINGS,
                        prefix,
                        folder,
                        find_data + FIND_DATA_FILE_NAME
                    ],
                );
                let block = e.call(OPERATOR_NEW, &args![SAVE_GAME_FILE_SIZE]).u32();
                let file = if block != 0 {
                    fn_0085fb40(e, Ptr::new(block), Ptr::new(path), 0, 0x20000).addr()
                } else {
                    0
                };
                let list = e.get(this, TESSaveLoadGame::m_pSaveGameList);
                e.call(LIST_INSERT, &args![list, file, SAVE_GAME_FILE_COMPARATOR]);
            }
            if e.call(FIND_NEXT_FILE_IMPORT, &args![handle, find_data])
                .u32()
                == 0
            {
                break;
            }
        }
        e.call(FIND_CLOSE_IMPORT, &args![handle]);
    }
    e.mem.free(pattern);
    e.mem.free(find_data);
    e.mem.free(path);
}

// Translated from 0085fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `SaveGameFile` (Xbox PDB): a `BSFile` for `path`
/// (`mode`, `buffer_size`), the `SaveGameFile` vtable and no file time.
pub fn fn_0085fb40(
    e: &mut Engine,
    this: Ptr<SaveGameFile>,
    path: Ptr,
    mode: u32,
    buffer_size: u32,
) -> Ptr<SaveGameFile> {
    e.call(
        BSFILE_CONSTRUCT,
        &args![this, path, mode, buffer_size, 0u32],
    );
    e.mem.set_u32(this.addr(), SAVE_GAME_FILE_VTABLE);
    e.set(this, SaveGameFile::bHasFileTime, false);
    this
}

// Translated from 0085fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `SaveGameFile` (Xbox PDB): the
/// `BSFile` destructor (`fn_0085fbb0`), then the memory when bit 0 of
/// `flags` is set.
pub fn fn_0085fb80(e: &mut Engine, this: Ptr<SaveGameFile>, flags: u32) -> Ptr<SaveGameFile> {
    fn_0085fbb0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 0085fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `SaveGameFile` (Xbox PDB): the `BSFile` destructor
/// (`00aff240`).
pub fn fn_0085fbb0(e: &mut Engine, this: Ptr<SaveGameFile>) {
    e.call(BSFILE_DESTRUCT, &args![this]);
}

// Translated from 0085fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the list of saves: every `SaveGameFile` is destroyed (virtual
/// slot 0, flag 1), the list emptied and deleted, and `m_pSaveGameList`
/// cleared. Nothing happens without a list.
pub fn fn_0085fbd0(e: &mut Engine, this: Ptr<TESSaveLoadGame>) {
    let list = e.get(this, TESSaveLoadGame::m_pSaveGameList).addr();
    if list == 0 {
        return;
    }
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let file = e.mem.u32(slot);
        if file != 0 {
            e.vcall(file, FILE_DESTRUCT, &args![1u32]);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let list = e.get(this, TESSaveLoadGame::m_pSaveGameList).addr();
    destroy_list(e, list);
    e.set(this, TESSaveLoadGame::m_pSaveGameList, Ptr::NULL);
}

// Translated from 0085fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The comparison that sorts the saves, newest first: the last write time
/// of each file, which a `SaveGameFile` remembers (`fn_0085fe80`) after
/// asking the file system for it (`FindFirstFileA` on its name) once.
/// Returns -1 when `a` is newer, 1 when `b` is newer, else 0.
pub fn fn_0085fc90(e: &mut Engine, a: Ptr<SaveGameFile>, b: Ptr<SaveGameFile>) -> i32 {
    let (a_low, a_high) = save_file_time(e, a);
    let (b_low, b_high) = save_file_time(e, b);
    if a_high > b_high {
        -1
    } else if a_high < b_high {
        1
    } else if a_low > b_low {
        -1
    } else {
        (a_low < b_low) as i32
    }
}

/// The last write time (low, high) of a save: the remembered one, or the
/// one the file system gives for the file's name (zero for a file it does
/// not find), which is then remembered.
pub(crate) fn save_file_time(e: &mut Engine, file: Ptr<SaveGameFile>) -> (u32, u32) {
    let time = e.mem.alloc(8);
    e.call(MEMSET, &args![time, 0u32, 8u32]);
    if fn_0085feb0(e, file) {
        let scratch = e.mem.alloc(8);
        let remembered = fn_0085fe50(e, file, Ptr::new(scratch));
        e.mem.set_u32(time, e.mem.u32(remembered.addr()));
        e.mem.set_u32(time + 4, e.mem.u32(remembered.addr() + 4));
        e.mem.free(scratch);
    } else {
        let find_data = e.mem.alloc(FIND_DATA_SIZE);
        let name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
        let handle = e
            .call(FIND_FIRST_FILE_IMPORT, &args![name, find_data])
            .u32();
        if handle != 0xFFFF_FFFF {
            e.mem
                .set_u32(time, e.mem.u32(find_data + FIND_DATA_LAST_WRITE_TIME));
            e.mem.set_u32(
                time + 4,
                e.mem.u32(find_data + FIND_DATA_LAST_WRITE_TIME + 4),
            );
        }
        e.call(FIND_CLOSE_IMPORT, &args![handle]);
        let (low, high) = (e.mem.u32(time), e.mem.u32(time + 4));
        fn_0085fe80(e, file, low, high);
        e.mem.free(find_data);
    }
    let result = (e.mem.u32(time), e.mem.u32(time + 4));
    e.mem.free(time);
    result
}

// Translated from 0085fe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameFile`'s file time getter: copies the 8 bytes at `+0x15C` to
/// `out` and returns `out`.
pub fn fn_0085fe50(e: &mut Engine, this: Ptr<SaveGameFile>, out: Ptr) -> Ptr {
    let low = e.get(this, SaveGameFile::FileTimeLow);
    let high = e.get(this, SaveGameFile::FileTimeHigh);
    e.mem.set_u32(out.addr(), low);
    e.mem.set_u32(out.addr() + 4, high);
    out
}

// Translated from 0085fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameFile`'s file time setter: stores the time and marks it known.
pub fn fn_0085fe80(e: &mut Engine, this: Ptr<SaveGameFile>, low: u32, high: u32) {
    e.set(this, SaveGameFile::FileTimeLow, low);
    e.set(this, SaveGameFile::FileTimeHigh, high);
    e.set(this, SaveGameFile::bHasFileTime, true);
}

// Translated from 0085feb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `SaveGameFile` knows its file time (`bHasFileTime`).
pub fn fn_0085feb0(e: &mut Engine, this: Ptr<SaveGameFile>) -> bool {
    e.get(this, SaveGameFile::bHasFileTime)
}

// Translated from 0085fed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Describes a save file: `name_out` (255 characters, may be null) is
/// cleared first. When the file's name has the form the game gives saves
/// (`fn_008607e0` reads the number, label and time out of it) that is all;
/// otherwise the header is read (`fn_008600b0`: the number into `number_out`,
/// the label into `label_out`, the time into `time_out`) and `name_out`
/// becomes the displayed name: for "quicksave" or "autosave" in the file's
/// name that kind's text, " - " and `label_out`; for any name that does not
/// start with "Save " the file's own name without its extension, cut to 18
/// characters.
pub fn fn_0085fed0(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    number_out: Ptr,
    name_out: Ptr,
    label_out: Ptr,
    time_out: Ptr,
) {
    if !name_out.is_null() {
        e.call(STRING_COPY, &args![name_out, 0xFFu32, EMPTY_STRING]);
    }
    if fn_008607e0(e, this, file, number_out, label_out, time_out) {
        return;
    }
    fn_008600b0(
        e,
        this,
        file,
        number_out,
        Ptr::NULL,
        label_out,
        Ptr::NULL,
        Ptr::NULL,
        Ptr::NULL,
        time_out,
        Ptr::NULL,
        0,
    );
    if name_out.is_null() {
        return;
    }
    let name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
    let kind_text = if e.call(STRING_FIND, &args![name, QUICKSAVE_NAME]).u32() != 0 {
        Some(QUICKSAVE_DISPLAY_TEXT)
    } else {
        let name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
        if e.call(STRING_FIND, &args![name, AUTOSAVE_NAME]).u32() != 0 {
            Some(AUTOSAVE_DISPLAY_TEXT)
        } else {
            None
        }
    };
    if let Some(object) = kind_text {
        let text = e.call(PATH_OBJECT_GET, &args![object]).u32();
        e.call(STRING_COPY, &args![name_out, 0xFFu32, text]);
        e.call(STRING_CAT, &args![name_out, 0xFFu32, TEXT_SEPARATOR]);
        e.call(STRING_CAT, &args![name_out, 0xFFu32, label_out]);
        return;
    }
    let name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
    let last = e.call(STRRCHR, &args![name, 0x5Cu32]).u32();
    let leaf = last.wrapping_add(1);
    if e.call(STRNICMP, &args![leaf, SAVE_NAME_PREFIX, 5u32]).i32() != 0 {
        e.call(STRING_COPY, &args![name_out, 0xFFu32, leaf]);
        let length = e.call(STRLEN, &args![name_out]).u32();
        if length > 4 {
            e.mem.set_u8(name_out.addr() + length - 4, 0);
        }
        if length > 0x12 {
            e.mem.set_u8(name_out.addr() + 0x12, 0);
        }
    }
}

// Translated from 008600b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the description of a save from its header into the buffers the
/// caller gives (each of 300 characters, and any of them may be null): when
/// `file_is_open` is zero the file is opened first (`fn_00857370`, mode 2)
/// and closed again (`fn_008578b0`). A file with no readable header
/// (`008905f0` false, or `LoadHeader` gives 0) clears every output and
/// returns 0. Otherwise `ReadSaveGameDataOLD` reads the fields (the value
/// into `value_out`, the player name into `name_out`, the location into
/// `location_out`, the screenshot's second dimension into `height_out`) and
/// `save_text_out` becomes "<word> <save number>", `level_text_out`
/// "<word> <level>", `time_text_out` "hh:mm:ss" from the play time and
/// `date_text_out` "month/day/year hour:minute" from the save time. The
/// file is put back at the start. Returns what `ReadSaveGameDataOLD` gave
/// (the screenshot texture, or 0).
#[allow(clippy::too_many_arguments)]
pub fn fn_008600b0(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    value_out: Ptr,
    name_out: Ptr,
    save_text_out: Ptr,
    location_out: Ptr,
    level_text_out: Ptr,
    date_text_out: Ptr,
    time_text_out: Ptr,
    height_out: Ptr,
    file_is_open: u8,
) -> u32 {
    let opened = if file_is_open == 0 {
        fn_00857370(e, this, file, Ptr::NULL, 2)
    } else {
        file
    };
    let mut header_size = 0u32;
    if !opened.is_null() && e.call(OPENED_FILE_TEST, &args![opened]).bool() {
        header_size = tes_save_load_game_load_header(e, this, opened, false);
    }
    if header_size == 0 {
        if !value_out.is_null() {
            e.mem.set_u32(value_out.addr(), 0);
        }
        for text in [
            name_out,
            save_text_out,
            location_out,
            level_text_out,
            date_text_out,
            time_text_out,
        ] {
            if !text.is_null() {
                e.call(STRING_COPY, &args![text, 300u32, EMPTY_STRING]);
            }
        }
        if !height_out.is_null() {
            e.mem.set_u32(height_out.addr(), 0);
        }
        if file_is_open == 0 {
            fn_008578b0(e, this, opened, 2);
        }
        return 0;
    }

    // The save number (u16), the level (f32), the save time (a
    // `_SYSTEMTIME`, 16 bytes) and the play time (milliseconds).
    let cells = e.mem.alloc(0x20);
    let (save_number, level, date, play_time) = (cells, cells + 4, cells + 8, cells + 0x18);
    let result = tes_save_load_game_read_save_game_data_old(
        e,
        this,
        file,
        header_size,
        value_out,
        name_out,
        Ptr::new(save_number),
        location_out,
        Ptr::new(level),
        Ptr::new(date),
        Ptr::new(play_time),
        height_out,
    );
    if !save_text_out.is_null() {
        let word = e
            .call(PATH_OBJECT_GET, &args![SAVE_NUMBER_PREFIX_TEXT])
            .u32();
        let number = e.mem.u16(save_number) as u32;
        e.call(
            FORMAT,
            &args![save_text_out, 300u32, FORMAT_LABEL_NUMBER, word, number],
        );
    }
    if !level_text_out.is_null() {
        // `FISTP` with the rounding forced to truncation.
        let truncated = (e.mem.f32(level) as f64).trunc() as i64 as u32;
        let word = e
            .call(PATH_OBJECT_GET, &args![SAVE_LEVEL_PREFIX_TEXT])
            .u32();
        e.call(
            FORMAT,
            &args![level_text_out, 300u32, FORMAT_LABEL_NUMBER, word, truncated],
        );
    }
    if !time_text_out.is_null() {
        let (hours, minutes, seconds) = split_play_time(e.mem.u32(play_time));
        e.call(
            FORMAT,
            &args![
                time_text_out,
                300u32,
                FORMAT_PLAY_TIME,
                hours,
                minutes,
                seconds
            ],
        );
    }
    if !date_text_out.is_null() {
        // A `_SYSTEMTIME`: year +0, month +2, day +6, hour +8, minute +10.
        let year = e.mem.u16(date) as u32;
        let month = e.mem.u16(date + 2) as u32;
        let day = e.mem.u16(date + 6) as u32;
        let hour = e.mem.u16(date + 8) as u32;
        let minute = e.mem.u16(date + 10) as u32;
        e.call(
            FORMAT,
            &args![
                date_text_out,
                300u32,
                FORMAT_DATE_TIME,
                month,
                day,
                year,
                hour,
                minute
            ],
        );
    }
    let mode: u32 = e.global(SEEK_MODE);
    e.vcall(opened.addr(), FILE_SEEK, &args![0u32, mode]);
    if file_is_open == 0 {
        fn_008578b0(e, this, opened, 2);
    }
    e.mem.free(cells);
    result
}

/// Splits a play time in milliseconds into hours, minutes and seconds.
pub(crate) fn split_play_time(milliseconds: u32) -> (u32, u32, u32) {
    let hours = milliseconds / 3_600_000;
    let rest = milliseconds - hours * 3_600_000;
    let minutes = rest / 60_000;
    let rest = rest - minutes * 60_000;
    (hours, minutes, rest / 1000)
}

// Translated from 00860390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::ReadSaveGameDataOLD` (Xbox PDB): reads the part of a
/// save's header after the signature, from `size` bytes of the file into a
/// buffer of the game: the value (when the format is at least 0x38; else
/// 0) into `value_out`; the player name (a length byte, then the text; skipped
/// without `name_out`); the save number (a `u16`, from format 0x34) into
/// `save_number_out`; the location text (a length byte; when 0 the default
/// location text) into `location_out`; the level (an `f32`) into
/// `level_out`; the play time (a `u32`, from format 0x1D) into
/// `play_time_out`; the save time (16 bytes, from format 0x38, else what the
/// file system gives for the file, `00b003c0`) into `date_out`; and, when
/// `height_out` is not null, the screenshot: two dimensions, 0x24 bytes
/// skipped before format 0x2E, the pixels read into a `NiPixelData`
/// (`00a7c190`), and a texture "SaveGameTexture" made from it. Without
/// `height_out` the screenshot's bytes are skipped. Returns the texture,
/// or 0. The buffer is freed.
#[allow(clippy::too_many_arguments)]
pub fn tes_save_load_game_read_save_game_data_old(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    size: u32,
    value_out: Ptr,
    name_out: Ptr,
    save_number_out: Ptr,
    location_out: Ptr,
    level_out: Ptr,
    date_out: Ptr,
    play_time_out: Ptr,
    height_out: Ptr,
) -> u32 {
    let scratch = e.mem.alloc(0x40);
    let buffer = tes_save_load_game_create_buffer(e, this, size);
    fn_008586d0(e, this, file, buffer, size);
    let singleton: Ptr<TESSaveLoadGame> = game_singleton(e);
    let version = |e: &mut Engine| e.call(CURRENT_VERSION, &args![singleton]).u8();

    if version(e) >= 0x38 {
        fn_008579e0(e, this, Ptr::new(scratch), 4);
        if !value_out.is_null() {
            e.mem.set_u32(value_out.addr(), e.mem.u32(scratch));
        }
    }
    if version(e) < 0x38 && !value_out.is_null() {
        e.mem.set_u32(value_out.addr(), 0);
    }
    fn_008579e0(e, this, Ptr::new(scratch + 4), 1);
    let name_length = e.mem.u8(scratch + 4) as u32;
    if !name_out.is_null() {
        fn_008579e0(e, this, name_out, name_length);
    } else {
        fn_00857bd0(e, this, name_length);
    }
    e.mem.set_u16(scratch + 8, 0);
    if version(e) >= 0x34 {
        fn_008579e0(e, this, Ptr::new(scratch + 8), 2);
    }
    if !save_number_out.is_null() {
        e.mem
            .set_u16(save_number_out.addr(), e.mem.u16(scratch + 8));
    }
    fn_008579e0(e, this, Ptr::new(scratch + 4), 1);
    let location_length = e.mem.u8(scratch + 4) as u32;
    if !location_out.is_null() {
        if location_length != 0 {
            fn_008579e0(e, this, location_out, location_length);
        } else {
            let text = e.call(PATH_OBJECT_GET, &args![DEFAULT_LOCATION_TEXT]).u32();
            e.call(STRING_COPY, &args![location_out, 300u32, text]);
        }
    } else if location_length != 0 {
        fn_00857bd0(e, this, location_length);
    }
    fn_008579e0(e, this, Ptr::new(scratch + 0xC), 4);
    if !level_out.is_null() {
        e.mem.set_f32(level_out.addr(), e.mem.f32(scratch + 0xC));
    }
    e.mem.set_u32(scratch + 0x10, 0);
    if version(e) >= 0x1D {
        fn_008579e0(e, this, Ptr::new(scratch + 0x10), 4);
    }
    if !play_time_out.is_null() {
        e.mem
            .set_u32(play_time_out.addr(), e.mem.u32(scratch + 0x10));
    }
    // The save time, 16 bytes at +0x14.
    let date = scratch + 0x14;
    e.call(MEMSET, &args![date, 0u32, 0x10u32]);
    if version(e) >= 0x38 {
        fn_008579e0(e, this, Ptr::new(date), 0x10);
    }
    if version(e) < 0x38 {
        let out = e.mem.alloc(0x10);
        let time = e.call(FILE_GET_TIME, &args![file, out]).u32();
        for word in 0..4 {
            e.mem.set_u32(date + 4 * word, e.mem.u32(time + 4 * word));
        }
        e.mem.free(out);
    }
    if !date_out.is_null() {
        for word in 0..4 {
            e.mem
                .set_u32(date_out.addr() + 4 * word, e.mem.u32(date + 4 * word));
        }
    }
    let mut texture = 0u32;
    fn_008579e0(e, this, Ptr::new(scratch + 4), 4);
    let screenshot_size = e.mem.u32(scratch + 4);
    if screenshot_size != 0 {
        if !height_out.is_null() {
            fn_008579e0(e, this, Ptr::new(scratch + 0x24), 4);
            fn_008579e0(e, this, Ptr::new(scratch + 0x28), 4);
            let first = e.mem.u32(scratch + 0x24);
            let second = e.mem.u32(scratch + 0x28);
            e.mem.set_u32(height_out.addr(), second);
            if version(e) < 0x2E {
                fn_00857bd0(e, this, 0x24);
            }
            let format = e.mem.alloc(0x44);
            for word in 0..0x11 {
                e.mem.set_u32(
                    format + 4 * word,
                    e.mem.u32(PIXEL_FORMAT_TEMPLATE + 4 * word),
                );
            }
            let block = e.call(PIXEL_DATA_ALLOCATE, &args![0x74u32]).u32();
            let image = if block != 0 {
                e.call(
                    PIXEL_DATA_CONSTRUCT,
                    &args![block, first, first, format, 1u32, 1u32],
                )
                .u32()
            } else {
                0
            };
            let bytes = first.wrapping_mul(second).wrapping_mul(3);
            let pixels = e.call(IMAGE_PIXELS, &args![image, 0u32, 0u32]).u32();
            fn_008579e0(e, this, Ptr::new(pixels), bytes);
            let preferences = e.mem.alloc(12);
            e.call(FORMAT_PREFS_CONSTRUCT, &args![preferences]);
            e.mem.set_u32(preferences, 2);
            e.mem.set_u32(preferences + 8, 0);
            e.mem.set_u32(preferences + 4, 0);
            let name = e.mem.alloc(4);
            e.call(FIXED_STRING_CONSTRUCT, &args![name, SAVE_GAME_TEXTURE_NAME]);
            let name_text = e.call(FIXED_STRING_TEXT, &args![name]).u32();
            texture = e
                .call(
                    CREATE_TEXTURE_FROM_PIXELS,
                    &args![image, name_text, preferences],
                )
                .u32();
            e.call(FIXED_STRING_DESTRUCT, &args![name]);
            e.call(REFERENCE_RELEASE, &args![texture]);
            e.mem.free(name);
            e.mem.free(preferences);
            e.mem.free(format);
        } else {
            fn_00857bd0(e, this, screenshot_size);
        }
    }
    fn_00858700(e, this, buffer);
    e.mem.free(scratch);
    texture
}

// Translated from 008607e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads what it can from the file's own name (the form the game gives
/// saves: "Save <number> - <name>, <location>, Level <n>, Playing Time
/// hh.mm.ss.ess"): false unless the name (after the last backslash) starts
/// with "Save " and holds "Playing Time" and "-". The number between "Save "
/// and the first "-" goes to `number_out`; the text after the second last
/// comma up to the last (starting at the first space) goes, after the game's
/// save word, to `label_out`; the play time after "Playing Time", with its
/// extension cut and "." turned into ":", goes to `time_out`. A name with
/// fewer than two commas gives false when the label or time is wanted.
pub fn fn_008607e0(
    e: &mut Engine,
    _this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    number_out: Ptr,
    label_out: Ptr,
    time_out: Ptr,
) -> bool {
    let name = e.vcall(file.addr(), FILE_GET_NAME, &args![]).u32();
    let leaf = e.call(STRRCHR, &args![name, 0x5Cu32]).u32().wrapping_add(1);
    let playing_time = e.call(STRING_FIND, &args![leaf, TEXT_PLAYING_TIME]).u32();
    let dash = e.call(STRING_FIND, &args![leaf, TEXT_DASH]).u32();
    if playing_time == 0
        || dash == 0
        || e.call(STRNCMP, &args![leaf, SAVE_NAME_PREFIX, 5u32]).i32() != 0
    {
        return false;
    }
    let work = e.mem.alloc(0x10C);
    if !number_out.is_null() {
        let count = dash.wrapping_sub(leaf + 5);
        e.call(COPY_COUNTED, &args![work, leaf + 5, count]);
        e.mem.set_u8(work + count, 0);
        let number = e.call(ATOL, &args![work]).u32();
        e.mem.set_u32(number_out.addr(), number);
    }
    if !label_out.is_null() || !time_out.is_null() {
        let mut last = 0u32;
        let mut second_last = 0u32;
        let mut cursor = leaf;
        loop {
            cursor = e.call(STRING_FIND, &args![cursor + 1, TEXT_COMMA]).u32();
            if cursor == 0 {
                break;
            }
            second_last = last;
            last = cursor;
        }
        if second_last == 0 || last == 0 {
            e.mem.free(work);
            return false;
        }
        if !label_out.is_null() {
            let mut start = second_last + 2;
            let length = e.call(STRLEN, &args![leaf]).u32();
            while e.mem.i8(start) != 0x20 && start < leaf + length {
                start += 1;
            }
            let count = last.wrapping_sub(start);
            e.call(COPY_COUNTED, &args![work, start, count]);
            e.mem.set_u8(work + count, 0);
            let word = e
                .call(PATH_OBJECT_GET, &args![SAVE_NUMBER_PREFIX_TEXT])
                .u32();
            e.call(
                FORMAT,
                &args![label_out, 0xFFu32, FORMAT_TWO_STRINGS, word, work],
            );
        }
        if !time_out.is_null() {
            e.call(STRING_COPY, &args![time_out, 0xFFu32, playing_time + 0xD]);
            let extension = e.call(STRING_FIND, &args![time_out, ESS_EXTENSION]).u32();
            if extension != 0 {
                e.mem.set_u8(extension, 0);
            }
            let mut index = 0u32;
            while index < e.call(STRLEN, &args![time_out]).u32() {
                if e.mem.u8(time_out.addr() + index) == b'.' {
                    e.mem.set_u8(time_out.addr() + index, b':');
                }
                index += 1;
            }
        }
    }
    e.mem.free(work);
    true
}

// Translated from 00860ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the file name of a new save into `out` (260 characters; nothing
/// happens for null): "Save <next number> - <player name> - <location>,
/// Level <n>, Playing Time hh.mm.ss" with the characters a file name cannot
/// hold (`\/:*<>?|"`) replaced (a quote by an apostrophe, the others by a
/// space), tried again until no file of that name exists in the save
/// folder. (The game builds a `" #n"` suffix for the retries but gives
/// `sprintf_s` no place for it, so the name does not change between tries.)
pub fn fn_00860ae0(e: &mut Engine, this: Ptr<TESSaveLoadGame>, out: Ptr) {
    if out.is_null() {
        return;
    }
    let player: u32 = e.global(PLAYER);
    let location_object = e.mem.alloc(8);
    e.call(STRING_OBJECT_CONSTRUCT, &args![location_object]);
    let mut count = 0u32;
    let name = e.call(REFERENCE_GET_NAME, &args![player]).u32();
    e.call(PLAYER_GET_LOCATION_TEXT, &args![player, location_object]);
    let mut location = e.call(STRING_OBJECT_TEXT, &args![location_object]).u32();
    if location == 0 {
        if e.call(REF_GET_PARENT_CELL, &args![player]).u32() != 0 {
            let cell = e.call(REF_GET_PARENT_CELL, &args![player]).u32();
            location = e.vcall(cell, FORM_GET_DESCRIPTION, &args![]).u32();
        } else {
            location = EMPTY_STRING;
        }
    }
    let suffix = e.mem.alloc(8);
    let save_name = e.mem.alloc(0x104);
    let path = e.mem.alloc(0x104);
    loop {
        if count == 0 {
            e.call(STRING_COPY, &args![suffix, 5u32, EMPTY_STRING]);
        } else {
            e.call(
                FORMAT,
                &args![suffix, 5u32, FORMAT_NUMBER_SUFFIX, count + 1],
            );
        }
        let play_time = e.call(PLAYER_PLAY_TIME, &args![player]).u32();
        let (hours, minutes, seconds) = split_play_time(play_time);
        let level = e.call(PLAYER_LEVEL, &args![player]).u16() as u32;
        let number = e.call(NEXT_SAVE_NUMBER, &args![this]).u32();
        e.call(
            FORMAT,
            &args![
                save_name,
                0x104u32,
                FORMAT_SAVE_NAME,
                number,
                name,
                location,
                level,
                hours,
                minutes,
                seconds,
                suffix
            ],
        );
        let mut cursor = save_name;
        loop {
            let found = fn_00860e00(e, Ptr::new(cursor), Ptr::new(INVALID_FILE_NAME_CHARACTERS));
            if found.is_null() {
                break;
            }
            let replacement = if e.mem.u8(found.addr()) == b'"' {
                b'\''
            } else {
                b' '
            };
            e.mem.set_u8(found.addr(), replacement);
            cursor = found.addr() + 1;
        }
        let folder = save_folder(e);
        let prefix = e.call(PATH_PREFIX, &args![]).u32();
        e.call(
            FORMAT,
            &args![path, 0x104u32, FORMAT_SAVE_PATH, prefix, folder, save_name],
        );
        let exists = e
            .call(FILE_EXISTS, &args![path, 0u32, 0u32, 0xFFFF_FFFFu32])
            .u32();
        count += 1;
        if exists == 0 {
            break;
        }
    }
    e.call(STRING_COPY, &args![out, 0x104u32, save_name]);
    e.call(STRING_OBJECT_DESTRUCT, &args![location_object]);
    e.mem.free(path);
    e.mem.free(save_name);
    e.mem.free(suffix);
    e.mem.free(location_object);
}

// Translated from 00860e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strpbrk(text, set)`: the first character of `text` that is in `set`
/// (the CRT function at `00ecbf20`).
pub fn fn_00860e00(e: &mut Engine, text: Ptr, set: Ptr) -> Ptr {
    e.call(STRPBRK, &args![text, set]).ptr()
}

// Translated from 00860e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::LoadHeader` (Xbox PDB): reads the start of a save from
/// `file`: the signature (a "CON " prefix moves the start to 0xD000 and
/// reads the signature again), then the two version bytes. A different
/// signature gives 0. From format 0x52 the original save time (16 bytes) and
/// version (4 bytes) are read when `read_save_time` is set, else skipped
/// (a seek); an older format clears them (0xFF) when `read_save_time` is
/// set. Returns the next dword (the size of the header's rest). A save
/// whose major version is not 0, or (with `read_save_time`) whose minor
/// version is neither 0x7D nor below 0x13, asks "Save game version is
/// ..., so errors may occur. Continue trying to load?" and gives
/// 0xFFFFFFFF when the answer is 2.
pub fn tes_save_load_game_load_header(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    read_save_time: bool,
) -> u32 {
    e.set(this, TESSaveLoadGame::m_iFileStartPosition, 0);
    let signature: u32 = e.global(SAVE_SIGNATURE_POINTER);
    let length = e.call(STRLEN, &args![signature]).u32();
    let text = e.mem.alloc(0x110);
    let result = e.mem.alloc(4);
    if fn_00857ba0(e, this.cast(), file, Ptr::new(text), length) == 0 {
        e.mem.free(text);
        e.mem.free(result);
        return 0;
    }
    let seek_mode: u32 = e.global(SEEK_MODE);
    if e.mem.i8(text) != 0 {
        if e.call(STRNCMP, &args![text, CON_PREFIX, 4u32]).u32() == 0 {
            e.vcall(file.addr(), FILE_SEEK, &args![0xD000u32, seek_mode]);
            e.set(this, TESSaveLoadGame::m_iFileStartPosition, 0xD000);
            let signature: u32 = e.global(SAVE_SIGNATURE_POINTER);
            let length = e.call(STRLEN, &args![signature]).u32();
            fn_00857ba0(e, this.cast(), file, Ptr::new(text), length);
        }
        let signature: u32 = e.global(SAVE_SIGNATURE_POINTER);
        let length = e.call(STRLEN, &args![signature]).u32();
        if e.call(STRNCMP, &args![text, signature, length]).u32() != 0 {
            e.mem.free(text);
            e.mem.free(result);
            return 0;
        }
    } else {
        e.vcall(file.addr(), FILE_SEEK, &args![0u32, seek_mode]);
    }
    fn_00857ba0(e, this.cast(), file, Ptr::new(this.addr() + 0x74), 1);
    fn_00857ba0(e, this.cast(), file, Ptr::new(this.addr() + 0x75), 1);
    e.call(END_FORM_PROCESSING, &args![this]);
    let singleton = game_singleton(e);
    if e.call(CURRENT_VERSION, &args![singleton]).u8() >= 0x52 {
        if read_save_time {
            fn_00857ba0(e, this.cast(), file, Ptr::new(this.addr() + 0x98), 0x10);
            fn_00857ba0(e, this.cast(), file, Ptr::new(this.addr() + 0xA8), 4);
        } else {
            let relative_mode: u32 = e.global(SEEK_MODE_CURRENT);
            e.vcall(file.addr(), FILE_SEEK, &args![0x14u32, relative_mode]);
        }
    }
    if e.call(CURRENT_VERSION, &args![singleton]).u8() < 0x52 && read_save_time {
        e.call(MEMSET, &args![this.addr() + 0x98, 0xFFu32, 0x10u32]);
        e.set(this, TESSaveLoadGame::m_iOriginalSaveVersion, 0xFFFF_FFFF);
    }
    fn_00857ba0(e, this.cast(), file, Ptr::new(result), 4);
    let major = e.get(this, TESSaveLoadGame::m_cMajorVersion);
    let minor = e.get(this, TESSaveLoadGame::m_cMinorVersion);
    if major != 0 || (read_save_time && minor as u32 != CURRENT_MINOR_VERSION && minor < 0x13) {
        let message = e.mem.alloc(0x220);
        e.call(
            SPRINTF,
            &args![
                message,
                MSG_SAVE_VERSION_WARNING,
                major as u32,
                minor as u32,
                0u32,
                CURRENT_MINOR_VERSION
            ],
        );
        e.call(SET_LOADING_STATE, &args![singleton, 1u32]);
        let button_a = e.call(PATH_OBJECT_GET, &args![MESSAGE_BOX_TEXT_A]).u32();
        let button_b = e.call(PATH_OBJECT_GET, &args![MESSAGE_BOX_TEXT_B]).u32();
        let answer = e
            .call(
                SHOW_MESSAGE_BOX,
                &args![message, 1u32, 5u32, button_b, button_a, 0u32],
            )
            .u8();
        e.call(SET_LOADING_STATE, &args![singleton, 0u32]);
        if answer as i8 == 2 {
            e.mem.set_u32(result, 0xFFFF_FFFF);
        }
        e.mem.free(message);
    }
    let size = e.mem.u32(result);
    e.mem.free(text);
    e.mem.free(result);
    size
}

// Translated from 00861130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::SaveHeader` (Xbox PDB): writes the header of a save to
/// `file`: the signature, the two version bytes, the original save time
/// (set from the system clock, with the minor version as the original
/// version, when the game has none yet), the player's name (a length byte
/// that counts the terminator, then the text), the level (`u16`), the
/// location text (the player's, else his cell's; a length byte then the
/// text), the in-game date as a day plus the fraction of the hour, the
/// play time, the local time, and a screenshot (taken with the save lock
/// released; 0x100 by 0x100 pixels, or the size worked out from the screen's
/// shape when the save only measures). The save number is written (and
/// counted up) unless the save only measures or `name` contains
/// "quicksave" or "autosave", which write 0. With statistics on, the whole
/// header's size is added as the extra stat "Save Game Header". The C++
/// exception frame is not translated.
pub fn tes_save_load_game_save_header(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    file: Ptr,
    name: Ptr,
) {
    // The locals the game keeps on its stack, and passes by address.
    const NAME_LENGTH: u32 = 0x00;
    const LOCATION_LENGTH: u32 = 0x01;
    const LEVEL: u32 = 0x02;
    const LOCATION_OBJECT: u32 = 0x04;
    const DATE: u32 = 0x0C;
    const PLAY_TIME: u32 = 0x10;
    const LOCAL_TIME: u32 = 0x14;
    const IMAGE_SIZE: u32 = 0x24;
    const WIDTH: u32 = 0x28;
    const HEIGHT: u32 = 0x2C;
    const TOTAL_SIZE: u32 = 0x30;
    const SAVE_NUMBER: u32 = 0x34;
    const FRAME_SIZE: u32 = 0x40;

    let frame = e.mem.alloc(FRAME_SIZE);
    let write = |e: &mut Engine, data: u32, size: u32| {
        fn_00857b50(e, this, file, Ptr::new(data), size);
    };
    e.call(SAVE_PREPARE_B, &args![this]);
    e.call(END_FORM_PROCESSING, &args![this]);
    let signature: u32 = e.global(SAVE_SIGNATURE_POINTER);
    let signature_length = e.call(STRLEN, &args![signature]).u32();
    write(e, signature, signature_length);
    write(e, this.addr() + 0x74, 1);
    write(e, this.addr() + 0x75, 1);
    if e.get(this, TESSaveLoadGame::m_iOriginalSaveVersion) == 0 {
        e.call(GET_SYSTEM_TIME_IMPORT, &args![this.addr() + 0x98]);
        let minor = e.get(this, TESSaveLoadGame::m_cMinorVersion) as u32;
        e.set(this, TESSaveLoadGame::m_iOriginalSaveVersion, minor);
    }
    write(e, this.addr() + 0x98, 0x10);
    write(e, this.addr() + 0xA8, 4);

    let player: u32 = e.global(PLAYER);
    let player_name = e.call(REFERENCE_GET_NAME, &args![player]).u32();
    let name_length = e.call(STRLEN, &args![player_name]).u32().wrapping_add(1) as u8;
    e.mem.set_u8(frame + NAME_LENGTH, name_length);
    let level = e.call(PLAYER_LEVEL, &args![player]).u16();
    e.mem.set_u16(frame + LEVEL, level);
    let location_object = frame + LOCATION_OBJECT;
    e.call(STRING_OBJECT_CONSTRUCT, &args![location_object]);
    e.call(PLAYER_GET_LOCATION_TEXT, &args![player, location_object]);
    let mut location = e.call(STRING_OBJECT_TEXT, &args![location_object]).u32();
    if location == 0 {
        let cell = e.call(REF_GET_PARENT_CELL, &args![player]).u32();
        location = e.vcall(cell, FORM_GET_DESCRIPTION, &args![]).u32();
    }
    e.mem.set_u8(frame + LOCATION_LENGTH, 0);
    if location != 0 {
        let length = e.call(STRLEN, &args![location]).u32().wrapping_add(1) as u8;
        e.mem.set_u8(frame + LOCATION_LENGTH, length);
    }
    // The date is the day plus the hour as a fraction of a day, as an f32.
    let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u32();
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let hours_per_day = e.mem.f64(HOURS_PER_DAY);
    e.mem
        .set_f32(frame + DATE, (hour / hours_per_day + day as f64) as f32);
    e.call(GET_LOCAL_TIME_IMPORT, &args![frame + LOCAL_TIME]);

    e.mem.set_u32(frame + IMAGE_SIZE, 0);
    e.mem.set_u32(frame + WIDTH, 0x100);
    e.mem.set_u32(frame + HEIGHT, 0x100);
    let mut screenshot = 0u32;
    let mut pixel_bytes = 0u32;
    if !game_unavailable(e, this) {
        let lock: u32 = e.global(SAVE_LOCK);
        e.call(SAVE_LOCK_LEAVE, &args![lock]);
        screenshot = e
            .call(
                TAKE_SAVE_SCREENSHOT,
                &args![frame + WIDTH, frame + HEIGHT, 0u32],
            )
            .u32();
        e.call(SAVE_LOCK_ENTER, &args![lock]);
        if screenshot != 0 {
            pixel_bytes = fn_00861610(e, Ptr::new(screenshot), 0, 0);
            e.mem
                .set_u32(frame + IMAGE_SIZE, pixel_bytes.wrapping_add(8));
        }
    } else {
        let width = e.call(SCREEN_WIDTH, &args![]).i32();
        let height = e.call(SCREEN_HEIGHT, &args![]).i32();
        let aspect = (width as f64 / height as f64) as f32;
        let rows = e.call(FLOAT_TO_INTEGER, &args![aspect as f64]).i32();
        let rows = rows.wrapping_mul(0x100);
        let size = 0x100i32.wrapping_mul(rows).wrapping_mul(3).wrapping_add(8);
        e.mem.set_u32(frame + IMAGE_SIZE, size as u32);
    }
    let play_time = e.call(PLAYER_PLAY_TIME, &args![player]).u32();
    e.mem.set_u32(frame + PLAY_TIME, play_time);
    let image_size = e.mem.u32(frame + IMAGE_SIZE);
    let total = (name_length as u32)
        .wrapping_add(image_size)
        .wrapping_add(e.mem.u8(frame + LOCATION_LENGTH) as u32)
        .wrapping_add(0x24);
    e.mem.set_u32(frame + TOTAL_SIZE, total);
    write(e, frame + TOTAL_SIZE, 4);

    let numbered = !game_unavailable(e, this)
        && (name.is_null()
            || (e.call(STRING_FIND, &args![name, QUICKSAVE_NAME]).u32() == 0
                && e.call(STRING_FIND, &args![name, AUTOSAVE_NAME]).u32() == 0));
    if numbered {
        let number = e.call(NEXT_SAVE_NUMBER, &args![this]).u32();
        e.mem.set_u32(frame + SAVE_NUMBER, number);
        write(e, frame + SAVE_NUMBER, 4);
        let next = e.get(this, TESSaveLoadGame::m_iNextSaveNumber);
        e.set(
            this,
            TESSaveLoadGame::m_iNextSaveNumber,
            next.wrapping_add(1),
        );
    } else {
        e.mem.set_u32(frame + SAVE_NUMBER, 0);
        write(e, frame + SAVE_NUMBER, 4);
    }
    write(e, frame + NAME_LENGTH, 1);
    write(e, player_name, name_length as u32);
    write(e, frame + LEVEL, 2);
    write(e, frame + LOCATION_LENGTH, 1);
    if location != 0 {
        let length = e.mem.u8(frame + LOCATION_LENGTH) as u32;
        write(e, location, length);
    }
    write(e, frame + DATE, 4);
    write(e, frame + PLAY_TIME, 4);
    write(e, frame + LOCAL_TIME, 0x10);
    write(e, frame + IMAGE_SIZE, 4);
    if !game_unavailable(e, this) {
        if screenshot != 0 {
            write(e, frame + WIDTH, 4);
            write(e, frame + HEIGHT, 4);
            let pixels = e.call(IMAGE_PIXELS, &args![screenshot, 0u32, 0u32]).u32();
            write(e, pixels, pixel_bytes);
            e.vcall(screenshot, SLOT_DESTRUCTOR, &args![1u32]);
        }
    } else {
        let measured = e.get(this, TESSaveLoadGame::m_iSimulationFileSize);
        e.set(
            this,
            TESSaveLoadGame::m_iSimulationFileSize,
            measured.wrapping_add(image_size),
        );
    }
    let stats = e.get(this, TESSaveLoadGame::m_pSaveLoadStats);
    if !stats.is_null() {
        save_stats_add_extra_stat(e, stats, total, Ptr::new(LABEL_SAVE_GAME_HEADER));
    }
    e.call(STRING_OBJECT_DESTRUCT, &args![location_object]);
    e.mem.free(frame);
}

// Translated from 00861610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size in bytes of one level of an image (a `NiPixelData`): the
/// difference of the two offsets at `index` and `index + 1` of the table
/// the image's pointer at `+0x5C` holds. The second word is not used.
pub fn fn_00861610(e: &mut Engine, this: Ptr, index: u32, _unused_2: u32) -> u32 {
    let table = e.mem.u32(this.addr() + 0x5C);
    let next = e.mem.u32(table + index.wrapping_mul(4).wrapping_add(4));
    let start = e.mem.u32(table + index.wrapping_mul(4));
    next.wrapping_sub(start)
}

// Translated from 00861640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame::GetCreatedExteriorCellFormID` (Xbox PDB): the form id
/// the save recorded for the exterior cell (`x`, `y`) of the world space
/// `world_space_id`, or 0. The record is taken out of the list and freed.
/// Without `m_pCreatedExteriorCells`, or a list for the world space, 0.
pub fn tes_save_load_game_get_created_exterior_cell_form_id(
    e: &mut Engine,
    this: Ptr<TESSaveLoadGame>,
    world_space_id: u32,
    x: i32,
    y: i32,
) -> u32 {
    let map = e.get(this, TESSaveLoadGame::m_pCreatedExteriorCells);
    if map.is_null() {
        return 0;
    }
    let out = e.mem.alloc(4);
    let found = e.call(MAP_GET_AT, &args![map, world_space_id, out]).bool();
    let list = e.mem.u32(out);
    e.mem.free(out);
    if !found {
        return 0;
    }
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let record: Ptr<ExteriorCellReferenceData> = Ptr::new(e.mem.u32(slot));
        if !record.is_null()
            && e.get(record, ExteriorCellReferenceData::iCellX) == x
            && e.get(record, ExteriorCellReferenceData::iCellY) == y
        {
            let form_id = e.get(record, ExteriorCellReferenceData::iFormID);
            let cell = e.mem.alloc(4);
            e.mem.set_u32(cell, record.addr());
            e.call(LIST_REMOVE, &args![list, cell]);
            e.mem.free(cell);
            delete(e, record.addr());
            return form_id;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00486a90,
            ni_t_pointer_map_unsigned_int_change_data_p_new_item(Ptr<ChangesMap>) -> Ptr
        ),
        entry!(
            0x00666050,
            ni_t_pointer_map_unsigned_int_change_data_p_delete_item(Ptr<ChangesMap>, Ptr)
        ),
        entry!(0x00854e10, fn_00854e10(Ptr<ChangeData>)),
        entry!(0x00854e40, fn_00854e40(Ptr<ChangeData>, u32)),
        entry!(0x00854e70, fn_00854e70(Ptr<ChangeData>, u32)),
        entry!(
            0x00854ed0,
            changes_map_scalar_deleting_destructor(Ptr<ChangesMap>, u32) -> Ptr<ChangesMap>
        ),
        entry!(0x00854f00, fn_00854f00(Ptr<ChangesMap>)),
        entry!(0x00854f60, changes_map_remove_all_changes(Ptr<ChangesMap>)),
        entry!(
            0x00854fe0,
            fn_00854fe0(Ptr<ChangeData>, u32) -> Ptr<ChangeData>
        ),
        entry!(
            0x00855010,
            fn_00855010(Ptr<ChangesMap>, Ptr, u32) -> Ptr<ChangeData>
        ),
        entry!(
            0x00855100,
            fn_00855100(Ptr<ChangesMap>, u32) -> Ptr<ChangeData>
        ),
        entry!(
            0x00855130,
            fn_00855130(Ptr<ChangesMap>, Ptr) -> Ptr<ChangeData>
        ),
        entry!(0x00855150, fn_00855150(Ptr<ChangesMap>, Ptr, u32) -> bool),
        entry!(0x008551f0, fn_008551f0(Ptr<ChangesMap>, Ptr, u8) -> bool),
        entry!(0x00855220, fn_00855220(Ptr<ChangesMap>, u32, u8) -> bool),
        entry!(
            0x008552b0,
            fn_008552b0(Ptr<InteriorCellNewReferencesMap>) -> Ptr<InteriorCellNewReferencesMap>
        ),
        entry!(
            0x008552e0,
            interior_cell_new_references_map_scalar_deleting_destructor(
                Ptr<InteriorCellNewReferencesMap>,
                u32,
            ) -> Ptr<
                InteriorCellNewReferencesMap,
            >
        ),
        entry!(0x00855310, fn_00855310(Ptr<InteriorCellNewReferencesMap>)),
        entry!(
            0x008553e0,
            fn_008553e0(Ptr<ExteriorCellNewReferencesMap>) -> Ptr<ExteriorCellNewReferencesMap>
        ),
        entry!(
            0x00855410,
            exterior_cell_new_references_map_scalar_deleting_destructor(
                Ptr<ExteriorCellNewReferencesMap>,
                u32,
            ) -> Ptr<
                ExteriorCellNewReferencesMap,
            >
        ),
        entry!(0x00855440, fn_00855440(Ptr<ExteriorCellNewReferencesMap>)),
        entry!(
            0x00855550,
            fn_00855550(Ptr<NumericIDBufferMap>) -> Ptr<NumericIDBufferMap>
        ),
        entry!(
            0x00855580,
            numeric_id_buffer_map_scalar_deleting_destructor(
                Ptr<NumericIDBufferMap>,
                u32,
            )
                -> Ptr<NumericIDBufferMap>
        ),
        entry!(0x008555b0, fn_008555b0(Ptr<NumericIDBufferMap>)),
        entry!(0x00855660, fn_00855660(Ptr<SaveStats>) -> Ptr<SaveStats>),
        entry!(0x00855730, fn_00855730(Ptr<SaveStats>)),
        entry!(
            0x008558a0,
            save_stats_add_extra_stat(Ptr<SaveStats>, u32, Ptr)
        ),
        entry!(
            0x00855970,
            fn_00855970(Ptr<SaveStats>, Ptr<SaveFormHeader>, u16)
        ),
        entry!(0x00855a20, fn_00855a20(Ptr<SaveStats>, Ptr<LoadFormHeader>)),
        entry!(
            0x00855b60,
            fn_00855b60(Ptr<LoadFormHeader>, Ptr<LoadFormHeader>) -> i32
        ),
        entry!(0x00855ba0, save_stats_print_stats(Ptr<SaveStats>, Ptr)),
        entry!(0x008562c0, fn_008562c0(Ptr<Stats>) -> Ptr<Stats>),
        entry!(0x00856300, fn_00856300(Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00856c70,
            tes_save_load_game_remove_changes(Ptr<TESSaveLoadGame>, Ptr, u8)
        ),
        entry!(
            0x00856ca0,
            fn_00856ca0(Ptr<TESSaveLoadGame>, Ptr, Ptr, bool) -> bool
        ),
        entry!(0x00857210, fn_00857210(Ptr)),
        entry!(0x00857230, fn_00857230(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(
            0x00857250,
            fn_00857250(Ptr<SaveStats>, u32) -> Ptr<SaveStats>
        ),
        entry!(
            0x00857280,
            fn_00857280(Ptr<FormAndFlags>, Ptr, u32, u32, u8) -> Ptr<FormAndFlags>
        ),
        entry!(0x008572c0, fn_008572c0(Ptr) -> Ptr),
        entry!(
            0x008572f0,
            fn_008572f0(Ptr<ReferenceData>) -> Ptr<ReferenceData>
        ),
        entry!(
            0x00857320,
            fn_00857320(Ptr<MovedReferenceData>) -> Ptr<MovedReferenceData>
        ),
        entry!(0x00857350, fn_00857350(Ptr)),
        entry!(
            0x00857370,
            fn_00857370(Ptr<TESSaveLoadGame>, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(0x008578b0, fn_008578b0(Ptr<TESSaveLoadGame>, Ptr, u32)),
        entry!(0x00857950, fn_00857950(Ptr<TESSaveLoadGame>, Ptr, Ptr)),
        entry!(0x008579b0, fn_008579b0(Ptr<TESSaveLoadGame>, Ptr, u32)),
        entry!(0x008579e0, fn_008579e0(Ptr<TESSaveLoadGame>, Ptr, u32)),
        entry!(
            0x00857a10,
            tes_save_load_game_save_numeric_id(Ptr<TESSaveLoadGame>, Ptr, u32)
        ),
        entry!(
            0x00857aa0,
            tes_save_load_game_load_numeric_id(Ptr<TESSaveLoadGame>, Ptr, u32) -> bool
        ),
        entry!(
            0x00857b50,
            fn_00857b50(Ptr<TESSaveLoadGame>, Ptr, Ptr, u32) -> u32
        ),
        entry!(0x00857ba0, fn_00857ba0(Ptr, Ptr, Ptr, u32) -> u32),
        entry!(0x00857bd0, fn_00857bd0(Ptr<TESSaveLoadGame>, u32)),
        entry!(0x00857bf0, fn_00857bf0(Ptr<TESSaveLoadGame>, u32) -> u32),
        entry!(0x00857c70, fn_00857c70(Ptr<TESSaveLoadGame>, u32) -> u32),
        entry!(0x00857d10, fn_00857d10(Ptr, Ptr, bool) -> bool),
        entry!(
            0x00858030,
            tes_save_load_game_save_global_data(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00858480,
            tes_save_load_game_save_globals(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00858570,
            tes_save_load_game_save_final_data(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00858600,
            tes_save_load_game_create_buffer(Ptr<TESSaveLoadGame>, u32) -> Ptr
        ),
        entry!(
            0x008586a0,
            tes_save_load_game_write_file(Ptr<TESSaveLoadGame>, Ptr, Ptr, u32)
        ),
        entry!(
            0x008586d0,
            fn_008586d0(Ptr<TESSaveLoadGame>, Ptr, Ptr, u32) -> u32
        ),
        entry!(0x00858700, fn_00858700(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(0x00858730, fn_00858730(Ptr<TESSaveLoadGame>, Ptr) -> bool),
        entry!(
            0x00858aa0,
            fn_00858aa0(Ptr<LoadFormHeader>, Ptr, u32, u32) -> Ptr<LoadFormHeader>
        ),
        entry!(
            0x00858af0,
            fn_00858af0(Ptr<TESSaveLoadGame>, Ptr, Ptr, bool)
        ),
        entry!(
            0x00859120,
            tes_save_load_game_check_new_reference(Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x008591b0,
            tes_save_load_game_check_flags(Ptr<TESSaveLoadGame>, Ptr, u32) -> u32
        ),
        entry!(
            0x00859670,
            base_process_get_package_that_is_running(Ptr) -> u32
        ),
        entry!(0x00859690, fn_00859690(Ptr<TESSaveLoadGame>, Ptr) -> bool),
        entry!(0x008598d0, fn_008598d0(Ptr<TESSaveLoadGame>, u32)),
        entry!(
            0x00859a90,
            fn_00859a90(Ptr<TESSaveLoadGame>, u32, Ptr<CreatedReferenceData>) -> Ptr
        ),
        entry!(
            0x00859f20,
            fn_00859f20(Ptr, u32, Ptr<MovedReferenceData>) -> Ptr
        ),
        entry!(
            0x0085a240,
            fn_0085a240(Ptr<TESSaveLoadGame>, u32, u8) -> Ptr
        ),
        entry!(0x0085a290, fn_0085a290(Ptr<TESSaveLoadGame>, u32)),
        entry!(
            0x0085a2e0,
            tes_save_load_game_delete_form(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0085a3d0,
            tes_save_load_game_add_form_to_deferred_deletions_list(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(0x0085a410, fn_0085a410(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(
            0x0085a450,
            tes_save_load_game_get_initial_data_save_size(Ptr<TESSaveLoadGame>, Ptr, u32) -> u16
        ),
        entry!(
            0x0085a520,
            tes_save_load_game_save_initial_data(Ptr<TESSaveLoadGame>, Ptr, u32)
        ),
        entry!(0x0085ac30, fn_0085ac30(Ptr<TESSaveLoadGame>, Ptr, u32)),
        entry!(0x0085b170, fn_0085b170(Ptr<TESSaveLoadGame>, Ptr) -> u32),
        entry!(
            0x0085b240,
            tes_save_load_game_save_plugin_list(Ptr<TESSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0085b320,
            tes_save_load_game_build_changes_string(Ptr<TESSaveLoadGame>, Ptr, Ptr, u32, u8, u8)
        ),
        entry!(0x0085ef80, fn_0085ef80(Ptr, u16) -> Ptr),
        entry!(0x0085efc0, fn_0085efc0(Ptr<TESSaveLoadGame>, Ptr, u32, u16)),
        entry!(0x0085f010, fn_0085f010(Ptr<TESSaveLoadGame>, Ptr, u16)),
        entry!(0x0085f070, fn_0085f070(Ptr<TESSaveLoadGame>)),
        entry!(0x0085f150, fn_0085f150(Ptr<TESSaveLoadGame>, Ptr, u32, Ptr)),
        entry!(0x0085f2b0, fn_0085f2b0(Ptr<TESSaveLoadGame>, Ptr, u16)),
        entry!(0x0085f2e0, fn_0085f2e0(Ptr<TESSaveLoadGame>)),
        entry!(0x0085f3d0, fn_0085f3d0(Ptr<TESSaveLoadGame>, Ptr, Ptr)),
        entry!(0x0085f420, fn_0085f420(Ptr<TESSaveLoadGame>, Ptr, u16)),
        entry!(0x0085f450, fn_0085f450(Ptr<TESSaveLoadGame>)),
        entry!(0x0085f5a0, fn_0085f5a0(Ptr<TESSaveLoadGame>, Ptr, u16)),
        entry!(0x0085f5d0, fn_0085f5d0(Ptr<TESSaveLoadGame>)),
        entry!(0x0085f6c0, fn_0085f6c0(Ptr<TESSaveLoadGame>, Ptr, Ptr)),
        entry!(0x0085f710, fn_0085f710(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(0x0085f750, fn_0085f750(Ptr<TESSaveLoadGame>)),
        entry!(0x0085f810, fn_0085f810(Ptr) -> u32),
        entry!(0x0085f850, fn_0085f850(Ptr<TESSaveLoadGame>, bool)),
        entry!(0x0085f900, fn_0085f900(Ptr<TESSaveLoadGame>)),
        entry!(
            0x0085fb40,
            fn_0085fb40(Ptr<SaveGameFile>, Ptr, u32, u32) -> Ptr<SaveGameFile>
        ),
        entry!(
            0x0085fb80,
            fn_0085fb80(Ptr<SaveGameFile>, u32) -> Ptr<SaveGameFile>
        ),
        entry!(0x0085fbb0, fn_0085fbb0(Ptr<SaveGameFile>)),
        entry!(0x0085fbd0, fn_0085fbd0(Ptr<TESSaveLoadGame>)),
        entry!(
            0x0085fc90,
            fn_0085fc90(Ptr<SaveGameFile>, Ptr<SaveGameFile>) -> i32
        ),
        entry!(0x0085fe50, fn_0085fe50(Ptr<SaveGameFile>, Ptr) -> Ptr),
        entry!(0x0085fe80, fn_0085fe80(Ptr<SaveGameFile>, u32, u32)),
        entry!(0x0085feb0, fn_0085feb0(Ptr<SaveGameFile>) -> bool),
        entry!(
            0x0085fed0,
            fn_0085fed0(Ptr<TESSaveLoadGame>, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x008600b0,
            fn_008600b0(
                Ptr<TESSaveLoadGame>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                u8,
            ) -> u32
        ),
        entry!(
            0x00860390,
            tes_save_load_game_read_save_game_data_old(
                Ptr<TESSaveLoadGame>,
                Ptr,
                u32,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> u32
        ),
        entry!(
            0x008607e0,
            fn_008607e0(Ptr<TESSaveLoadGame>, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00860ae0, fn_00860ae0(Ptr<TESSaveLoadGame>, Ptr)),
        entry!(0x00860e00, fn_00860e00(Ptr, Ptr) -> Ptr),
        entry!(
            0x00860e20,
            tes_save_load_game_load_header(Ptr<TESSaveLoadGame>, Ptr, bool) -> u32
        ),
        entry!(
            0x00861130,
            tes_save_load_game_save_header(Ptr<TESSaveLoadGame>, Ptr, Ptr)
        ),
        entry!(0x00861610, fn_00861610(Ptr, u32, u32) -> u32),
        entry!(
            0x00861640,
            tes_save_load_game_get_created_exterior_cell_form_id(
                Ptr<TESSaveLoadGame>,
                u32,
                i32,
                i32,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Lines = Rc<RefCell<Vec<String>>>;
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    type FileWrites = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
    type ByteWrites = Rc<RefCell<Vec<Vec<u8>>>>;
    type Added = Rc<RefCell<Vec<(u32, u32, u32, u32, u8)>>>;
    type Hooks = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

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

    /// An engine with the globals the unit reads mapped and working doubles
    /// for the small callees nearly every function uses: the dword getters,
    /// the list node accessors and constructor, `strlen`/`strcpy_s`, and the
    /// allocation scope (logged only). `operator new` and `delete` are the
    /// crate's own. Calls are logged.
    fn game() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000); // the TESSaveLoadGame pointer
        e.map(0x0120_2000, 0x1000); // the save lock
        e.map(0x010a_2000, 0x1000); // the seek mode
        e.map(0x0101_6000, 0x1000); // the message time
        let singleton = e.new_object::<TESSaveLoadGame>();
        e.set_global(SAVE_LOAD_GAME, singleton.addr());
        e.set_global(MESSAGE_TIME, 2.0f32);
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0xc)));
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
        e.register(STRING_COMPARE, |e, a| {
            returns(e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) as i32 as u32)
        });
        stub(&mut e, SCOPE_ENTER);
        stub(&mut e, SCOPE_LEAVE);
        e.call_log = Some(vec![]);
        e
    }

    /// The logged argument lists of the calls to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log: &Log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn freed(e: &Engine, block: u32) -> bool {
        e.mem.block_size(block).is_none()
    }

    /// A singly linked `BSSimpleList` of the items; returns the first node
    /// (0 for none).
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// The map iteration as the game's `GetFirstPos` / `GetNext` give it,
    /// over `entries` (key, value).
    fn install_entries(e: &mut Engine, entries: &[(u32, u32)]) {
        let queue = Rc::new(RefCell::new(VecDeque::from(entries.to_vec())));
        let first = queue.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, _| {
            returns(!first.borrow().is_empty() as u32)
        });
        e.register_double(MAP_NEXT, move |e, a| {
            let (key, value) = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], !queue.borrow().is_empty() as u32);
            Ret::default()
        });
    }

    /// `install_entries` for `SaveStats`'s map, keyed by a byte.
    fn install_byte_entries(e: &mut Engine, entries: &[(u8, u32)]) {
        let queue = Rc::new(RefCell::new(VecDeque::from(entries.to_vec())));
        let first = queue.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, _| {
            returns(!first.borrow().is_empty() as u32)
        });
        e.register_double(BYTE_MAP_NEXT, move |e, a| {
            let (key, value) = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u8(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], !queue.borrow().is_empty() as u32);
            Ret::default()
        });
    }

    type Table = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A key-value store behind `GetAt`, `SetAt` and `RemoveAt` of the
    /// given addresses.
    fn install_table(e: &mut Engine, get_at: u32, set_at: Option<u32>, remove_at: u32) -> Table {
        let table: Table = Rc::new(RefCell::new(vec![]));
        let t = table.clone();
        e.register_double(get_at, move |e, a| {
            let found = t.borrow().iter().find(|(k, _)| *k == a[1]).map(|kv| kv.1);
            if let Some(value) = found {
                e.mem.set_u32(a[2], value);
            }
            returns(found.is_some() as u32)
        });
        if let Some(set_at) = set_at {
            let t = table.clone();
            e.register_double(set_at, move |_, a| {
                t.borrow_mut().push((a[1], a[2]));
                Ret::default()
            });
        }
        let t = table.clone();
        e.register_double(remove_at, move |_, a| {
            let before = t.borrow().len();
            t.borrow_mut().retain(|(k, _)| *k != a[1]);
            returns((t.borrow().len() != before) as u32)
        });
        table
    }

    /// A form whose key is `key` (`+0x0C`).
    fn form_with_key(e: &mut Engine, key: u32) -> Ptr {
        let form = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(form.addr() + 0xc, key);
        form
    }

    fn change_data(e: &mut Engine, flags: u32, buffer: u32) -> Ptr<ChangeData> {
        let data: Ptr<ChangeData> = e.new_object();
        e.set(data, ChangeData::iFlags, flags);
        e.set(data, ChangeData::pBuffer, Ptr::new(buffer));
        data
    }

    #[test]
    fn new_item_asks_the_allocator_subobject() {
        let mut e = game();
        constant(&mut e, MAP_ALLOCATOR_NEW_ITEM, 0x1234);
        let map: Ptr<ChangesMap> = e.new_object();
        let item = ni_t_pointer_map_unsigned_int_change_data_p_new_item(&mut e, map);
        assert_eq!(item.addr(), 0x1234);
        assert_eq!(
            calls_to(&e, MAP_ALLOCATOR_NEW_ITEM),
            vec![vec![map.addr() + 0xc]]
        );
    }

    #[test]
    fn delete_item_clears_the_value_and_gives_the_entry_back() {
        let mut e = game();
        stub(&mut e, MAP_ALLOCATOR_DELETE_ITEM);
        let map: Ptr<ChangesMap> = e.new_object();
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 8, 0x77);
        ni_t_pointer_map_unsigned_int_change_data_p_delete_item(&mut e, map, Ptr::new(item));
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(
            calls_to(&e, MAP_ALLOCATOR_DELETE_ITEM),
            vec![vec![map.addr() + 0xc, item]]
        );
    }

    #[test]
    fn change_data_destructor_frees_only_a_buffer() {
        let mut e = game();
        let buffer = e.mem.alloc(16);
        let with_buffer = change_data(&mut e, 1, buffer);
        fn_00854e10(&mut e, with_buffer);
        assert!(freed(&e, buffer));
        let without = change_data(&mut e, 1, 0);
        fn_00854e10(&mut e, without);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![buffer]]);
    }

    #[test]
    fn add_flags_only_without_a_buffer() {
        let mut e = game();
        let plain = change_data(&mut e, 0b0101, 0);
        fn_00854e40(&mut e, plain, 0b0010);
        assert_eq!(e.get(plain, ChangeData::iFlags), 0b0111);
        let buffered = change_data(&mut e, 0b0101, 0x1000);
        fn_00854e40(&mut e, buffered, 0b0010);
        assert_eq!(e.get(buffered, ChangeData::iFlags), 0b0101);
    }

    #[test]
    fn remove_flags_only_without_a_buffer() {
        let mut e = game();
        let plain = change_data(&mut e, 0b0111, 0);
        fn_00854e70(&mut e, plain, 0b0010);
        assert_eq!(e.get(plain, ChangeData::iFlags), 0b0101);
        let buffered = change_data(&mut e, 0b0111, 0x1000);
        fn_00854e70(&mut e, buffered, 0b0010);
        assert_eq!(e.get(buffered, ChangeData::iFlags), 0b0111);
    }

    #[test]
    fn changes_map_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        install_entries(&mut e, &[]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, CHANGES_MAP_BASE_DESTRUCT);
        let kept: Ptr<ChangesMap> = e.new_object();
        let result = changes_map_scalar_deleting_destructor(&mut e, kept, 0);
        assert_eq!(result, kept);
        assert!(!freed(&e, kept.addr()));
        assert_eq!(e.mem.u32(kept.addr()), CHANGES_MAP_VTABLE);
        let deleted: Ptr<ChangesMap> = e.new_object();
        changes_map_scalar_deleting_destructor(&mut e, deleted, 1);
        assert!(freed(&e, deleted.addr()));
        assert_eq!(calls_to(&e, CHANGES_MAP_BASE_DESTRUCT).len(), 2);
    }

    #[test]
    fn changes_map_destructor_removes_the_changes_then_the_base() {
        let mut e = game();
        let data = change_data(&mut e, 1, 0);
        install_entries(&mut e, &[(0x100, data.addr())]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, CHANGES_MAP_BASE_DESTRUCT);
        let map: Ptr<ChangesMap> = e.new_object();
        fn_00854f00(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), CHANGES_MAP_VTABLE);
        assert!(freed(&e, data.addr()));
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a == MAP_REMOVE_ALL || *a == CHANGES_MAP_BASE_DESTRUCT)
            .collect();
        assert_eq!(order, vec![MAP_REMOVE_ALL, CHANGES_MAP_BASE_DESTRUCT]);
    }

    #[test]
    fn remove_all_changes_deletes_each_change_data_and_its_buffer() {
        let mut e = game();
        let buffer = e.mem.alloc(8);
        let with_buffer = change_data(&mut e, 1, buffer);
        let plain = change_data(&mut e, 2, 0);
        install_entries(
            &mut e,
            &[
                (0x100, with_buffer.addr()),
                (0x200, 0),
                (0x300, plain.addr()),
            ],
        );
        stub(&mut e, MAP_REMOVE_ALL);
        let map: Ptr<ChangesMap> = e.new_object();
        changes_map_remove_all_changes(&mut e, map);
        assert!(freed(&e, buffer));
        assert!(freed(&e, with_buffer.addr()));
        assert!(freed(&e, plain.addr()));
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map.addr()]]);
    }

    #[test]
    fn change_data_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        let buffer = e.mem.alloc(8);
        let data = change_data(&mut e, 1, buffer);
        assert_eq!(fn_00854fe0(&mut e, data, 0), data);
        assert!(freed(&e, buffer));
        assert!(!freed(&e, data.addr()));
        let other = change_data(&mut e, 1, 0);
        fn_00854fe0(&mut e, other, 1);
        assert!(freed(&e, other.addr()));
    }

    #[test]
    fn get_or_create_adds_flags_to_the_existing_change_data() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, Some(CHANGES_MAP_SET_AT), MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let form = form_with_key(&mut e, 0x77);
        let existing = change_data(&mut e, 0b01, 0);
        table.borrow_mut().push((0x77, existing.addr()));
        let result = fn_00855010(&mut e, map, form, 0b10);
        assert_eq!(result, existing);
        assert_eq!(e.get(existing, ChangeData::iFlags), 0b11);
        assert!(calls_to(&e, SCOPE_ENTER).is_empty());
    }

    #[test]
    fn get_or_create_makes_and_stores_a_new_change_data() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, Some(CHANGES_MAP_SET_AT), MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let form = form_with_key(&mut e, 0x78);
        let created = fn_00855010(&mut e, map, form, 0b100);
        assert!(!created.is_null());
        assert_eq!(e.get(created, ChangeData::iFlags), 0b100);
        assert_eq!(table.borrow().as_slice(), &[(0x78, created.addr())]);
        // The allocation scope: (0x11, 1, file, line 0x104).
        let scope = calls_to(&e, SCOPE_ENTER);
        assert_eq!(scope.len(), 1);
        assert_eq!(&scope[0][1..], &[0x11, 1, SOURCE_FILE, 0x104]);
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 1);
    }

    #[test]
    fn lookup_by_key_returns_the_stored_change_data_or_null() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((5, data.addr()));
        assert_eq!(fn_00855100(&mut e, map, 5), data);
        assert!(fn_00855100(&mut e, map, 6).is_null());
    }

    #[test]
    fn lookup_by_form_uses_the_forms_key() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((0x1234, data.addr()));
        let form = form_with_key(&mut e, 0x1234);
        assert_eq!(fn_00855130(&mut e, map, form), data);
        let other = form_with_key(&mut e, 0x4321);
        assert!(fn_00855130(&mut e, map, other).is_null());
    }

    #[test]
    fn clear_flags_keeps_the_change_data_while_flags_remain() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let form = form_with_key(&mut e, 9);
        let data = change_data(&mut e, 0b11, 0);
        table.borrow_mut().push((9, data.addr()));
        assert!(fn_00855150(&mut e, map, form, 0b01));
        assert_eq!(e.get(data, ChangeData::iFlags), 0b10);
        assert_eq!(table.borrow().len(), 1);
        assert!(!freed(&e, data.addr()));
    }

    #[test]
    fn clear_flags_removes_the_change_data_once_empty() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let form = form_with_key(&mut e, 9);
        let data = change_data(&mut e, 0b11, 0);
        table.borrow_mut().push((9, data.addr()));
        assert!(fn_00855150(&mut e, map, form, 0b11));
        assert!(table.borrow().is_empty());
        assert!(freed(&e, data.addr()));
        // No change data, or saving unavailable: false.
        assert!(!fn_00855150(&mut e, map, form, 1));
        constant(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        assert!(!fn_00855150(&mut e, map, form, 1));
    }

    #[test]
    fn drop_by_form_passes_the_forms_key_and_the_flag() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let form = form_with_key(&mut e, 0x55);
        let buffer = e.mem.alloc(8);
        let data = change_data(&mut e, 1, buffer);
        table.borrow_mut().push((0x55, data.addr()));
        // A buffered change data stays unless forced.
        assert!(fn_008551f0(&mut e, map, form, 0));
        assert_eq!(table.borrow().len(), 1);
        assert!(!freed(&e, data.addr()));
        assert!(fn_008551f0(&mut e, map, form, 1));
        assert!(table.borrow().is_empty());
        assert!(freed(&e, data.addr()));
        assert!(freed(&e, buffer));
        assert_eq!(calls_to(&e, MAP_REMOVE_AT), vec![vec![map.addr(), 0x55]]);
    }

    #[test]
    fn drop_by_key_removes_unbuffered_data_and_false_when_missing() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((3, data.addr()));
        assert!(fn_00855220(&mut e, map, 3, 0));
        assert!(table.borrow().is_empty());
        assert!(freed(&e, data.addr()));
        assert!(!fn_00855220(&mut e, map, 3, 0));
        constant(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        assert!(!fn_00855220(&mut e, map, 3, 1));
    }

    #[test]
    fn interior_map_constructor_sets_base_and_vtable() {
        let mut e = game();
        e.register(INTERIOR_MAP_BASE_CONSTRUCT, |_, a| returns(a[0]));
        let map: Ptr<InteriorCellNewReferencesMap> = e.new_object();
        assert_eq!(fn_008552b0(&mut e, map), map);
        assert_eq!(e.mem.u32(map.addr()), INTERIOR_MAP_VTABLE);
        assert_eq!(
            calls_to(&e, INTERIOR_MAP_BASE_CONSTRUCT),
            vec![vec![map.addr(), 0x25]]
        );
    }

    #[test]
    fn interior_map_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        install_entries(&mut e, &[]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, INTERIOR_MAP_BASE_DESTRUCT);
        let map: Ptr<InteriorCellNewReferencesMap> = e.new_object();
        interior_cell_new_references_map_scalar_deleting_destructor(&mut e, map, 0);
        assert!(!freed(&e, map.addr()));
        interior_cell_new_references_map_scalar_deleting_destructor(&mut e, map, 1);
        assert!(freed(&e, map.addr()));
    }

    #[test]
    fn interior_map_destructor_deletes_each_list() {
        let mut e = game();
        let list = e.mem.alloc(8);
        install_entries(&mut e, &[(1, list), (2, 0)]);
        stub(&mut e, LIST_REMOVE_ALL);
        e.register(LIST_SCALAR_DELETE, |e, a| {
            e.mem.free(a[0]);
            returns(a[0])
        });
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, INTERIOR_MAP_BASE_DESTRUCT);
        let map: Ptr<InteriorCellNewReferencesMap> = e.new_object();
        fn_00855310(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), INTERIOR_MAP_VTABLE);
        assert!(freed(&e, list));
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![list]]);
        assert_eq!(calls_to(&e, INTERIOR_MAP_BASE_DESTRUCT).len(), 1);
    }

    #[test]
    fn exterior_map_constructor_sets_base_and_vtable() {
        let mut e = game();
        e.register(EXTERIOR_MAP_BASE_CONSTRUCT, |_, a| returns(a[0]));
        let map: Ptr<ExteriorCellNewReferencesMap> = e.new_object();
        assert_eq!(fn_008553e0(&mut e, map), map);
        assert_eq!(e.mem.u32(map.addr()), EXTERIOR_MAP_VTABLE);
        assert_eq!(
            calls_to(&e, EXTERIOR_MAP_BASE_CONSTRUCT),
            vec![vec![map.addr(), 0x25]]
        );
    }

    #[test]
    fn exterior_map_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        install_entries(&mut e, &[]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, EXTERIOR_MAP_BASE_DESTRUCT);
        let map: Ptr<ExteriorCellNewReferencesMap> = e.new_object();
        exterior_cell_new_references_map_scalar_deleting_destructor(&mut e, map, 0);
        assert!(!freed(&e, map.addr()));
        exterior_cell_new_references_map_scalar_deleting_destructor(&mut e, map, 1);
        assert!(freed(&e, map.addr()));
    }

    #[test]
    fn exterior_map_destructor_frees_the_items_of_each_list() {
        let mut e = game();
        let (first, second) = (e.mem.alloc(12), e.mem.alloc(12));
        let list = list_of(&mut e, &[first, 0, second]);
        install_entries(&mut e, &[(1, list), (2, 0)]);
        stub(&mut e, LIST_REMOVE_ALL);
        e.register(LIST_SCALAR_DELETE, |_, a| returns(a[0]));
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, EXTERIOR_MAP_BASE_DESTRUCT);
        let map: Ptr<ExteriorCellNewReferencesMap> = e.new_object();
        fn_00855440(&mut e, map);
        assert!(freed(&e, first));
        assert!(freed(&e, second));
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![list]]);
        assert_eq!(calls_to(&e, LIST_SCALAR_DELETE), vec![vec![list, 1]]);
        assert_eq!(e.mem.u32(map.addr()), EXTERIOR_MAP_VTABLE);
    }

    #[test]
    fn numeric_id_map_constructor_sets_base_and_vtable() {
        let mut e = game();
        e.register(NUMERIC_ID_MAP_BASE_CONSTRUCT, |_, a| returns(a[0]));
        let map: Ptr<NumericIDBufferMap> = e.new_object();
        assert_eq!(fn_00855550(&mut e, map), map);
        assert_eq!(e.mem.u32(map.addr()), NUMERIC_ID_MAP_VTABLE);
        assert_eq!(
            calls_to(&e, NUMERIC_ID_MAP_BASE_CONSTRUCT),
            vec![vec![map.addr(), 0x25]]
        );
    }

    #[test]
    fn numeric_id_map_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        install_entries(&mut e, &[]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, NUMERIC_ID_MAP_BASE_DESTRUCT);
        let map: Ptr<NumericIDBufferMap> = e.new_object();
        numeric_id_buffer_map_scalar_deleting_destructor(&mut e, map, 0);
        assert!(!freed(&e, map.addr()));
        numeric_id_buffer_map_scalar_deleting_destructor(&mut e, map, 1);
        assert!(freed(&e, map.addr()));
    }

    #[test]
    fn numeric_id_map_destructor_frees_the_buffers() {
        let mut e = game();
        let buffer = e.mem.alloc(8);
        install_entries(&mut e, &[(1, buffer), (2, 0)]);
        stub(&mut e, MAP_REMOVE_ALL);
        stub(&mut e, NUMERIC_ID_MAP_BASE_DESTRUCT);
        let map: Ptr<NumericIDBufferMap> = e.new_object();
        fn_008555b0(&mut e, map);
        assert!(freed(&e, buffer));
        assert_eq!(e.mem.u32(map.addr()), NUMERIC_ID_MAP_VTABLE);
        assert_eq!(calls_to(&e, NUMERIC_ID_MAP_BASE_DESTRUCT).len(), 1);
    }

    #[test]
    fn save_stats_constructor_makes_the_map_and_the_list() {
        let mut e = game();
        e.register(STATS_MAP_CONSTRUCT, |_, a| returns(a[0]));
        let stats: Ptr<SaveStats> = e.new_object();
        assert_eq!(fn_00855660(&mut e, stats), stats);
        let map = e.get(stats, SaveStats::pStatsMap);
        let list = e.get(stats, SaveStats::pExtraStats);
        assert!(!map.is_null() && !list.is_null());
        assert_eq!(e.mem.block_size(map.addr()), Some(0x10));
        assert_eq!(e.mem.block_size(list.addr()), Some(8));
        assert_eq!(
            calls_to(&e, STATS_MAP_CONSTRUCT),
            vec![vec![map.addr(), 0x25]]
        );
    }

    #[test]
    fn save_stats_destructor_frees_headers_extra_stats_and_lists() {
        let mut e = game();
        let (header_a, header_b) = (e.mem.alloc(12), e.mem.alloc(12));
        let type_list = list_of(&mut e, &[header_a, header_b]);
        install_byte_entries(&mut e, &[(5, type_list)]);
        stub(&mut e, LIST_REMOVE_ALL);
        e.register(LIST_SCALAR_DELETE, |_, a| returns(a[0]));
        let map_destructor = 0x0200_1000;
        let vtable = 0x0200_0000;
        e.put_vtable(vtable, &[map_destructor]);
        stub(&mut e, map_destructor);
        let stats_map = e.mem.alloc(0x10);
        e.mem.set_u32(stats_map, vtable);
        let description = e.mem.alloc(8);
        let stat = e.mem.alloc(8);
        e.mem.set_u32(stat + 4, description);
        let extra_list = list_of(&mut e, &[stat, 0]);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(stats_map));
        e.set(stats, SaveStats::pExtraStats, Ptr::new(extra_list));
        fn_00855730(&mut e, stats);
        assert!(freed(&e, header_a));
        assert!(freed(&e, header_b));
        assert!(freed(&e, description));
        assert!(freed(&e, stat));
        assert_eq!(
            calls_to(&e, LIST_SCALAR_DELETE),
            vec![vec![type_list, 1], vec![extra_list, 1]]
        );
        // The map's own destructor, vtable slot 0, with flag 1.
        assert_eq!(calls_to(&e, map_destructor), vec![vec![stats_map, 1]]);
    }

    #[test]
    fn add_extra_stat_copies_the_description_to_the_list_head() {
        let mut e = game();
        // The record is read while the list head call runs: the cell that
        // holds its address is freed afterwards.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            let record = e.mem.u32(a[1]);
            sink.borrow_mut().push((a[0], record));
            Ret::default()
        });
        let list = e.mem.alloc(8);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pExtraStats, Ptr::new(list));
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"Animations");
        save_stats_add_extra_stat(&mut e, stats, 1234, Ptr::new(text));
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, list);
        let record: Ptr<ExtraStat> = Ptr::new(seen[0].1);
        assert_eq!(e.get(record, ExtraStat::iSize), 1234);
        let copy = e.get(record, ExtraStat::pDescription);
        assert_ne!(copy.addr(), text);
        assert_eq!(e.mem.cstr(copy.addr()), b"Animations");
        // The copy has room for the terminator.
        assert_eq!(e.mem.block_size(copy.addr()), Some(16));
        assert_eq!(
            &calls_to(&e, SCOPE_ENTER)[0][1..],
            &[0x11, 1, SOURCE_FILE, 0x267]
        );
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 1);
    }

    #[test]
    fn stats_record_copies_the_header_and_adds_the_size() {
        let mut e = game();
        install_table(
            &mut e,
            BYTE_MAP_GET_AT,
            Some(BYTE_MAP_SET_AT),
            MAP_REMOVE_AT,
        );
        stub(&mut e, LIST_INSERT);
        let map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(map));
        let header: Ptr<SaveFormHeader> = e.new_object();
        e.set(header, SaveFormHeader::iFormID, 0xAABBCCDD);
        e.set(header, SaveFormHeader::cFormType, 0x2A);
        e.set(header, SaveFormHeader::iFlags, 0x11223344);
        e.set(header, SaveFormHeader::cVersion, 0x0F);
        fn_00855970(&mut e, stats, header, 0x0123);
        // The list insert receives the heap copy of the 12-byte header.
        let insert = calls_to(&e, LIST_INSERT);
        assert_eq!(insert.len(), 1);
        assert_eq!(
            e.mem.bytes(insert[0][1], 12),
            vec![0xDD, 0xCC, 0xBB, 0xAA, 0x2A, 0x44, 0x33, 0x22, 0x11, 0x0F, 0x23, 0x01]
        );
        // Two scopes: the one of this function (line 0x276), then the one of
        // the insert (line 0x286).
        let scopes = calls_to(&e, SCOPE_ENTER);
        assert_eq!(scopes[0][4], 0x276);
        assert_eq!(scopes[1][4], 0x286);
    }

    #[test]
    fn stats_insert_creates_the_type_list_when_missing() {
        let mut e = game();
        let get = install_table(
            &mut e,
            BYTE_MAP_GET_AT,
            Some(BYTE_MAP_SET_AT),
            MAP_REMOVE_AT,
        );
        stub(&mut e, LIST_INSERT);
        let map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(map));
        let header: Ptr<LoadFormHeader> = e.new_object();
        e.set(header, LoadFormHeader::iFormID, 0x42);
        e.set(header, LoadFormHeader::cFormType, 9);
        e.set(header, LoadFormHeader::iSize, 77);
        fn_00855a20(&mut e, stats, header);
        // A list was created and stored under the type.
        let stored = get.borrow().clone();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].0, 9);
        let insert = calls_to(&e, LIST_INSERT);
        assert_eq!(insert.len(), 1);
        assert_eq!(insert[0][0], stored[0].1);
        assert_eq!(insert[0][2], STATS_COMPARATOR);
        // The inserted item is a copy of the header.
        let copy = insert[0][1];
        assert_ne!(copy, header.addr());
        assert_eq!(e.mem.u32(copy), 0x42);
        assert_eq!(e.mem.u16(copy + 0xa), 77);
        assert_eq!(
            &calls_to(&e, SCOPE_ENTER)[0][1..],
            &[0x11, 1, SOURCE_FILE, 0x286]
        );
    }

    #[test]
    fn stats_insert_reuses_the_existing_type_list() {
        let mut e = game();
        let table = install_table(
            &mut e,
            BYTE_MAP_GET_AT,
            Some(BYTE_MAP_SET_AT),
            MAP_REMOVE_AT,
        );
        stub(&mut e, LIST_INSERT);
        let list = e.mem.alloc(8);
        table.borrow_mut().push((9, list));
        let map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(map));
        let header: Ptr<LoadFormHeader> = e.new_object();
        e.set(header, LoadFormHeader::cFormType, 9);
        fn_00855a20(&mut e, stats, header);
        assert_eq!(table.borrow().len(), 1);
        assert_eq!(calls_to(&e, LIST_INSERT)[0][0], list);
    }

    #[test]
    fn stats_comparator_orders_largest_first() {
        let mut e = game();
        let small: Ptr<LoadFormHeader> = e.new_object();
        let large: Ptr<LoadFormHeader> = e.new_object();
        let twin: Ptr<LoadFormHeader> = e.new_object();
        e.set(small, LoadFormHeader::iSize, 10);
        e.set(large, LoadFormHeader::iSize, 0xFFF0);
        e.set(twin, LoadFormHeader::iSize, 10);
        assert_eq!(fn_00855b60(&mut e, large, small), -1);
        assert_eq!(fn_00855b60(&mut e, small, large), 1);
        assert_eq!(fn_00855b60(&mut e, small, twin), 0);
    }

    /// The doubles `PrintStats` needs; returns the lines written to the file
    /// and the `FORMAT` calls (all argument words).
    fn print_engine() -> (Engine, Lines, Calls) {
        let mut e = game();
        e.map(0x0118_7000, 0x1000); // the form type names
        e.map(0x0108_0000, 0x1000); // the format strings
        e.map(0x0101_1000, 0x1000); // the empty string
        e.mem.set_cstr(0x0108_05c4, b"Extra Stats:\r\n\r\n");
        stub(&mut e, STRING_CAT);
        e.register(FILE_EXISTS, |_, _| returns(0));
        stub(&mut e, FILE_DELETE);
        stub(&mut e, FILE_OBJECT_CONSTRUCT);
        stub(&mut e, FILE_OBJECT_DESTRUCT);
        let lines = Rc::new(RefCell::new(Vec::new()));
        let formats = Rc::new(RefCell::new(Vec::new()));
        let sink = formats.clone();
        e.register_double(FORMAT, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            // The "text" is the format's address, so the written lines can
            // be told apart.
            e.mem.set_cstr(a[0], format!("{:08x}", a[2]).as_bytes());
            Ret::default()
        });
        let written = lines.clone();
        e.register_double(SYSTEM_FILE_DO_WRITE, move |e, a| {
            let text = e.mem.cstr(a[1]);
            assert_eq!(text.len() as u32, a[2]);
            written.borrow_mut().push(String::from_utf8(text).unwrap());
            returns(0)
        });
        stub(&mut e, BUILD_CHANGES_STRING);
        e.register(DYNAMIC_CAST, |_, _| returns(0));
        (e, lines, formats)
    }

    fn load_header(e: &mut Engine, id: u32, form_type: u8, size: u16, flags: u32) -> u32 {
        let header: Ptr<LoadFormHeader> = e.new_object();
        e.set(header, LoadFormHeader::iFormID, id);
        e.set(header, LoadFormHeader::cFormType, form_type);
        e.set(header, LoadFormHeader::iFlags, flags);
        e.set(header, LoadFormHeader::cVersion, 15);
        e.set(header, LoadFormHeader::iSize, size);
        header.addr()
    }

    /// A form object whose description (vtable slot `0x130`) is `text`.
    fn form_describing(e: &mut Engine, vtable: u32, text: &[u8]) -> u32 {
        let describe = vtable + 0x200;
        e.mem.map(vtable, 0x300);
        e.mem.set_u32(vtable + 0x130, describe);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, text);
        constant(e, describe, name);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form, vtable);
        form
    }

    #[test]
    fn print_stats_writes_the_sections_and_the_totals() {
        let (mut e, lines, formats) = print_engine();
        // Type 0x79 with two forms: 0x100 not loaded (size 10), 0x200 loaded
        // (size 30); one extra stat of 5.
        let first = load_header(&mut e, 0x100, 0x79, 10, 0xA);
        let second = load_header(&mut e, 0x200, 0x79, 30, 0xB);
        let type_list = list_of(&mut e, &[second, first]);
        install_byte_entries(&mut e, &[(0x79, type_list)]);
        let form = form_describing(&mut e, 0x0200_0000, b"Boone");
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(if a[0] == 0x200 { form } else { 0 })
        });
        let description = e.mem.alloc(8);
        e.mem.set_cstr(description, b"Textures");
        let stat = e.mem.alloc(8);
        e.mem.set_u32(stat, 5);
        e.mem.set_u32(stat + 4, description);
        let extra_list = list_of(&mut e, &[stat]);
        let stats_map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(stats_map));
        e.set(stats, SaveStats::pExtraStats, Ptr::new(extra_list));
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"Saves\\x");

        save_stats_print_stats(&mut e, stats, Ptr::new(path));

        // The file object is built for the path with the extension.
        let construct = calls_to(&e, FILE_OBJECT_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(&construct[0][2..], &[1, 2, 0]);
        assert_eq!(calls_to(&e, FILE_OBJECT_DESTRUCT).len(), 1);
        assert!(calls_to(&e, FILE_DELETE).is_empty());

        let formats = formats.borrow();
        let used: Vec<u32> = formats.iter().map(|f| f[2]).collect();
        assert_eq!(
            used,
            vec![
                0x0108_0670, // table header
                0x0108_0668, // "Buffer"
                0x0108_03e4, // section heading
                0x0108_0648, // form 0x200 (largest first)
                0x0108_0648, // form 0x100
                0x0108_05d8, // section totals
                0x0108_0254, // extra stat
                0x0108_0570, // grand totals
            ]
        );
        // Rows: the loaded form prints its description, the other NOT LOADED.
        let row_loaded = &formats[3];
        assert_eq!(&row_loaded[3..7], &[0x200, 30, 0xB, 15]);
        assert_eq!(e.mem.cstr(row_loaded[7]), b"Boone");
        let row_missing = &formats[4];
        assert_eq!(&row_missing[3..8], &[0x100, 10, 0xA, 15, 0x0108_063c]);
        // Section totals: count 2, total 40, minimum 10, maximum 30, mean 20.
        let totals = &formats[5];
        let mean = f64::from_bits(totals[12] as u64 | (totals[13] as u64) << 32);
        assert_eq!(
            (totals[4], totals[6], totals[8], totals[10]),
            (2, 40, 10, 30)
        );
        assert_eq!(mean, 20.0);
        // Grand totals add the extra stat: 2 forms, 45 bytes, mean 22.5.
        let grand = &formats[7];
        let mean = f64::from_bits(grand[7] as u64 | (grand[8] as u64) << 32);
        assert_eq!(&grand[3..7], &[2, 45, 10, 30]);
        assert_eq!(mean, 22.5);
        // One line per format call except the type heading, plus the
        // "Extra Stats:" line.
        let lines = lines.borrow();
        assert_eq!(lines.len(), 8);
        assert_eq!(lines[0], format!("{:08x}", 0x0108_0670u32));
        assert_eq!(lines[5], "Extra Stats:\r\n\r\n");
    }

    #[test]
    fn print_stats_names_a_form_from_its_reference_marker_or_description() {
        let (mut e, _lines, formats) = print_engine();
        // Three loaded forms of type 0: A has a reference with a name, B a
        // map marker with a location, C a reference and a marker whose
        // names are empty, so its description is used.
        let ids = [0xA_u32, 0xB, 0xC];
        let headers: Vec<u32> = ids
            .iter()
            .map(|&id| load_header(&mut e, id, 0, 4, 1))
            .collect();
        let list = list_of(&mut e, &headers);
        install_byte_entries(&mut e, &[(0, list)]);
        let mut forms = Vec::new();
        for (i, text) in [b"descA", b"descB", b"descC"].iter().enumerate() {
            forms.push(form_describing(
                &mut e,
                0x0200_0000 + 0x1000 * i as u32,
                *text,
            ));
        }
        let table = forms.clone();
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(table[(a[0] - 0xA) as usize])
        });
        let strings: Vec<u32> = [b"RefName".as_slice(), b"Place", b""]
            .iter()
            .map(|text| {
                let block = e.mem.alloc(16);
                e.mem.set_cstr(block, text);
                block
            })
            .collect();
        e.mem.set_cstr(0x0101_1584, b"");
        // The casts: target 0x11841cc is the reference, 0x1183158 the marker.
        let (reference_cast, marker_cast) = (0x0200_5000, 0x0200_5004);
        let by_form = forms.clone();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            let index = by_form.iter().position(|&f| f == a[0]).unwrap();
            let target = a[3];
            returns(match (index, target) {
                (0, 0x0118_41cc) | (2, 0x0118_41cc) => reference_cast + index as u32,
                (1, 0x0118_3158) | (2, 0x0118_3158) => marker_cast + index as u32,
                _ => 0,
            })
        });
        let (name_a, name_empty) = (strings[0], strings[2]);
        e.register_double(REFERENCE_GET_NAME, move |_, a| {
            returns(if a[0] == reference_cast {
                name_a
            } else {
                name_empty
            })
        });
        let (place, marker_empty) = (strings[1], strings[2]);
        e.register_double(MAP_MARKER_GET_LOCATION_NAME, move |_, a| {
            returns(if a[0] == marker_cast + 1 {
                place
            } else {
                marker_empty
            })
        });
        let stats_map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(stats_map));
        let path = e.mem.alloc(8);
        e.mem.set_cstr(path, b"x");

        save_stats_print_stats(&mut e, stats, Ptr::new(path));

        let formats = formats.borrow();
        let rows: Vec<&Vec<u32>> = formats.iter().filter(|f| f[2] == 0x0108_0648).collect();
        assert_eq!(rows.len(), 3);
        let names: Vec<Vec<u8>> = rows.iter().map(|row| e.mem.cstr(row[7])).collect();
        assert_eq!(
            names,
            vec![b"RefName".to_vec(), b"Place".to_vec(), b"descC".to_vec()]
        );
        // The heading of type 0 is the text at `0104469c` ("Form").
        assert!(formats.iter().any(|f| f[2] == 0x0104_469c));
    }

    #[test]
    fn print_stats_titles_a_section_with_the_type_name() {
        let (mut e, _lines, formats) = print_engine();
        e.mem.set_u32(0x0118_7004 + 12 * 3, 0x0200_0100);
        let header = load_header(&mut e, 0x10, 3, 8, 0);
        let list = list_of(&mut e, &[header]);
        install_byte_entries(&mut e, &[(3, list)]);
        e.register(LOOKUP_FORM, |_, _| returns(0));
        let stats_map = e.mem.alloc(0x10);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(stats, SaveStats::pStatsMap, Ptr::new(stats_map));
        let path = e.mem.alloc(8);
        e.mem.set_cstr(path, b"x");
        save_stats_print_stats(&mut e, stats, Ptr::new(path));
        let heading = formats
            .borrow()
            .iter()
            .find(|f| f[2] == 0x0101_9f08)
            .cloned()
            .unwrap();
        assert_eq!(heading[3], 0x0200_0100);
    }

    #[test]
    fn print_stats_writes_nothing_when_the_file_will_not_open() {
        let (mut e, lines, formats) = print_engine();
        e.register(FILE_EXISTS, |_, _| returns(1));
        // The file object's first word is non-zero after construction.
        e.register(FILE_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 1);
            Ret::default()
        });
        let stats: Ptr<SaveStats> = e.new_object();
        let path = e.mem.alloc(8);
        e.mem.set_cstr(path, b"x");
        save_stats_print_stats(&mut e, stats, Ptr::new(path));
        assert!(formats.borrow().is_empty());
        assert!(lines.borrow().is_empty());
        // An existing file was deleted first, and the object was destroyed.
        assert_eq!(calls_to(&e, FILE_DELETE).len(), 1);
        assert_eq!(calls_to(&e, FILE_OBJECT_DESTRUCT).len(), 1);
    }

    #[test]
    fn stats_constructor_resets_the_counters() {
        let mut e = game();
        let stats: Ptr<Stats> = e.new_object();
        e.mem.write(stats.addr(), &[0x55; 12]);
        assert_eq!(fn_008562c0(&mut e, stats), stats);
        assert_eq!(e.get(stats, Stats::iNum), 0);
        assert_eq!(e.get(stats, Stats::iTotalSize), 0);
        assert_eq!(e.get(stats, Stats::iMinSize), 0xFFFF);
        assert_eq!(e.get(stats, Stats::iMaxSize), 0);
    }

    #[test]
    fn write_text_reports_success_of_the_file_write() {
        let mut e = game();
        e.register(SYSTEM_FILE_DO_WRITE, |_, a| returns(a[2] & 1));
        let file = e.mem.alloc(0x20);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"ab");
        assert!(fn_00856300(
            &mut e,
            Ptr::NULL,
            Ptr::new(file),
            Ptr::new(text)
        ));
        e.mem.set_cstr(text, b"abc");
        assert!(!fn_00856300(
            &mut e,
            Ptr::NULL,
            Ptr::new(file),
            Ptr::new(text)
        ));
        // (file, text, length, 0, scratch)
        let writes = calls_to(&e, SYSTEM_FILE_DO_WRITE);
        assert_eq!(&writes[0][..4], &[file, text, 2, 0]);
    }

    #[test]
    fn remove_changes_skips_deleted_forms() {
        let mut e = game();
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let map: Ptr<ChangesMap> = e.new_object();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, map);
        let form = form_with_key(&mut e, 0x31);
        e.register(FORM_IS_DELETED, |e, a| {
            returns((e.mem.u32(a[0] + 8) & 0x4000 != 0) as u32)
        });
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((0x31, data.addr()));
        e.mem.set_u32(form.addr() + 8, 0x4000);
        tes_save_load_game_remove_changes(&mut e, game_object, form, 1);
        assert_eq!(table.borrow().len(), 1);
        e.mem.set_u32(form.addr() + 8, 0);
        tes_save_load_game_remove_changes(&mut e, game_object, form, 1);
        assert!(table.borrow().is_empty());
    }

    // ---- the save routine ------------------------------------------------

    #[test]
    fn save_refuses_with_the_sad_message_unless_allowed() {
        let mut e = game();
        e.map(0x0107_f000, 0x1000);
        e.mem.set_cstr(AUTOSAVE_NAME, b"autosave");
        e.register(GET_SAVING_ALLOWED, |_, _| returns(0));
        constant(&mut e, GET_MESSAGE_QUEUE, 0x5000);
        stub(&mut e, SHOW_MESSAGE);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Quicksave");
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        // A named save that is not an autosave, and an unnamed one, refuse.
        for name in [Ptr::new(name), Ptr::NULL] {
            assert!(!fn_00856ca0(&mut e, game_object, Ptr::NULL, name, false));
        }
        let message = calls_to(&e, SHOW_MESSAGE);
        assert_eq!(message.len(), 2);
        assert_eq!(&message[0][..4], &[0x5000, 0, SAD_ICON, 0]);
        assert_eq!(f32::from_bits(message[0][4]), 2.0);
        assert_eq!(message[0][5], 0);
        assert!(calls_to(&e, SAVE_LOCK_ENTER).is_empty());
        assert_eq!(
            calls_to(&e, GET_MESSAGE_QUEUE)[0],
            vec![MESSAGE_QUEUE_OBJECT]
        );
        assert_eq!(
            &calls_to(&e, SCOPE_ENTER)[0][1..],
            &[0x11, 1, SOURCE_FILE, 0x466]
        );
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 2);
    }

    /// What the full save needs: everything it calls, with doubles that log
    /// what the file receives.
    struct SaveRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        stream: u32,
        writes: FileWrites,
        seeks: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn save_rig() -> SaveRig {
        let mut e = game();
        constant(&mut e, GET_SAVING_ALLOWED, 1);
        stub(&mut e, SAVE_LOCK_ENTER);
        stub(&mut e, SAVE_LOCK_LEAVE);
        for step in [
            SAVE_PREPARE_A,
            SAVE_PREPARE_B,
            SAVE_PREPARE_C,
            SAVE_HEADER,
            SAVE_PLUGIN_LIST,
            SAVE_GLOBAL_DATA,
            SAVE_FINAL_DATA,
            SAVE_NUMERIC_ID_ARRAYS,
            SAVE_CLOSE_A,
            SAVE_CLOSE_B,
            FILE_FLUSH,
        ] {
            stub(&mut e, step);
        }
        constant(&mut e, CURRENT_VERSION, 7);
        let positions = Rc::new(RefCell::new(VecDeque::from(vec![100u32, 500])));
        e.register_double(FILE_POSITION, move |_, _| {
            returns(positions.borrow_mut().pop_front().unwrap())
        });
        let writes = Rc::new(RefCell::new(Vec::new()));
        let sink = writes.clone();
        e.register_double(WRITE_BYTES, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.bytes(a[2], a[3])));
            Ret::default()
        });
        // The stream: seek is slot 0x14, name slot 0x18.
        let vtable = 0x0200_0000;
        e.mem.map(vtable, 0x100);
        e.mem.set_u32(vtable + 0x14, 0x0200_1000);
        e.mem.set_u32(vtable + 0x18, 0x0200_1004);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream, vtable);
        let seeks = Rc::new(RefCell::new(Vec::new()));
        let sink = seeks.clone();
        e.register_double(0x0200_1000, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Saves\\save");
        constant(&mut e, 0x0200_1004, name);
        constant(&mut e, OPEN_SAVE_FILE, stream);
        e.set_global(SEEK_MODE, 0u32);
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        SaveRig {
            e,
            game: game_object,
            stream,
            writes,
            seeks,
        }
    }

    #[test]
    fn save_writes_headers_sizes_and_patches_the_positions() {
        let mut rig = save_rig();
        let e = &mut rig.e;
        // Two changed forms: 0x100 (no buffer, a real form) and 0x200 (with
        // a pre-built buffer of 2 bytes "AB").
        let plain = change_data(e, 0x3, 0);
        let buffer = e.mem.alloc(8);
        e.mem.write(buffer, b"AB");
        let buffered = change_data(e, 0x9, buffer);
        let map: Ptr<ChangesMap> = e.new_object();
        e.set(rig.game, TESSaveLoadGame::m_pChanges, map);
        install_entries(e, &[(0x100, plain.addr()), (0x200, buffered.addr())]);
        // The form: vtable slot 0x50 changes size, 0x58 saves the changes.
        let form_vtable = 0x0200_2000;
        e.mem.map(form_vtable, 0x100);
        e.mem.set_u32(form_vtable + 0x50, 0x0200_3000);
        e.mem.set_u32(form_vtable + 0x58, 0x0200_3004);
        constant(e, 0x0200_3000, 4);
        stub(e, 0x0200_3004);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form, form_vtable);
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(if a[0] == 0x100 { form } else { 0 })
        });
        constant(e, FORM_TYPE, 0x2B);
        e.register(CHECK_FLAGS, |_, a| returns(a[2] | 0x100));
        constant(e, GET_INITIAL_DATA_SAVE_SIZE, 3);
        let save_buffer = e.mem.alloc(16);
        constant(e, CREATE_BUFFER, save_buffer);
        stub(e, SAVE_INITIAL_DATA);
        stub(e, WRITE_FILE);
        stub(e, FREE_BUFFER);
        // The buffer's first four bytes: size 2, type 0x33, version 5.
        e.register(READ_BYTES, |e, a| {
            e.mem.write(a[1], &[2, 0, 0x33, 5]);
            Ret::default()
        });

        let saved = fn_00856ca0(e, rig.game, Ptr::NULL, Ptr::NULL, false);
        assert!(saved);

        let writes = rig.writes.borrow();
        let stream = rig.stream;
        let sizes: Vec<usize> = writes.iter().map(|(_, bytes)| bytes.len()).collect();
        // Two zero words, then for form 0x100: header, size, and for 0x200:
        // header, size, data; then the patched end position and count.
        assert_eq!(sizes, vec![4, 4, 10, 2, 10, 2, 2, 4, 4]);
        assert!(writes.iter().all(|(file, _)| *file == stream));
        assert_eq!(writes[0].1, vec![0; 4]);
        // Header of 0x100: id, type 0x2B, flags 0x3 | 0x100 (CheckFlags),
        // version 7.
        assert_eq!(
            writes[2].1,
            vec![0x00, 0x01, 0x00, 0x00, 0x2B, 0x03, 0x01, 0x00, 0x00, 0x07]
        );
        assert_eq!(writes[3].1, vec![7, 0]); // 4 + 3
                                             // Header of the buffered form: type and version from the buffer.
        assert_eq!(
            writes[4].1,
            vec![0x00, 0x02, 0x00, 0x00, 0x33, 0x09, 0x00, 0x00, 0x00, 0x05]
        );
        assert_eq!(writes[5].1, vec![2, 0]);
        assert_eq!(writes[6].1, b"AB".to_vec());
        // The end position (500) and the form count (2), after seeking back
        // to the start position (100).
        assert_eq!(writes[7].1, 500u32.to_le_bytes().to_vec());
        assert_eq!(writes[8].1, 2u32.to_le_bytes().to_vec());
        assert_eq!(rig.seeks.borrow().as_slice(), &[(100, 0)]);
        // The first form's data went through the buffer helpers.
        assert_eq!(
            calls_to(e, WRITE_FILE),
            vec![vec![rig.game.addr(), stream, save_buffer, 7]]
        );
        assert_eq!(
            calls_to(e, FREE_BUFFER),
            vec![vec![rig.game.addr(), save_buffer]]
        );
        // The routine bracketed itself with the lock and closed the file.
        assert_eq!(calls_to(e, SAVE_LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(e, SAVE_LOCK_LEAVE).len(), 1);
        assert_eq!(calls_to(e, FILE_FLUSH), vec![vec![stream]]);
        assert_eq!(
            calls_to(e, SAVE_CLOSE_B),
            vec![vec![rig.game.addr(), stream, 0]]
        );
        // The form header pointer is cleared again, and the buffer too.
        assert!(e
            .get(rig.game, TESSaveLoadGame::m_pCurrentlySavingFormHeader)
            .is_null());
        assert!(e.get(rig.game, TESSaveLoadGame::m_pBuffer).is_null());
    }

    #[test]
    fn save_allows_an_autosave_when_saving_is_not_allowed() {
        let mut rig = save_rig();
        let e = &mut rig.e;
        constant(e, GET_SAVING_ALLOWED, 0);
        e.map(0x0107_f000, 0x1000);
        e.mem.set_cstr(AUTOSAVE_NAME, b"autosave");
        install_entries(e, &[]);
        let map: Ptr<ChangesMap> = e.new_object();
        e.set(rig.game, TESSaveLoadGame::m_pChanges, map);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"autosave");
        stub(e, SHOW_MESSAGE);
        assert!(fn_00856ca0(e, rig.game, Ptr::NULL, Ptr::new(name), false));
        assert!(calls_to(e, SHOW_MESSAGE).is_empty());
        // No forms: the two zero words and the patched end and count.
        let sizes: Vec<usize> = rig.writes.borrow().iter().map(|w| w.1.len()).collect();
        assert_eq!(sizes, vec![4, 4, 4, 4]);
    }

    #[test]
    fn save_prints_and_deletes_the_statistics_when_asked() {
        let mut rig = save_rig();
        let e = &mut rig.e;
        install_entries(e, &[]);
        let map: Ptr<ChangesMap> = e.new_object();
        e.set(rig.game, TESSaveLoadGame::m_pChanges, map);
        // `SaveStats`'s own map: constructed with a destructor in slot 0.
        let map_vtable = 0x0200_4000;
        e.put_vtable(map_vtable, &[0x0200_4100]);
        stub(e, 0x0200_4100);
        e.register_double(STATS_MAP_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], map_vtable);
            returns(a[0])
        });
        // `PrintStats` runs (it is this unit's own): it builds the text file
        // object for the stream's name, which fails to open here.
        let printed = Rc::new(RefCell::new(Vec::new()));
        let sink = printed.clone();
        e.register_double(FILE_OBJECT_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push(e.mem.cstr(a[1]));
            e.mem.set_u32(a[0], 1);
            Ret::default()
        });
        e.register(FILE_EXISTS, |_, _| returns(0));
        stub(e, STRING_CAT);
        stub(e, FILE_OBJECT_DESTRUCT);
        stub(e, LIST_REMOVE_ALL);
        stub(e, LIST_SCALAR_DELETE);
        let saved = fn_00856ca0(e, rig.game, Ptr::NULL, Ptr::NULL, true);
        assert!(saved);
        assert_eq!(printed.borrow().len(), 1);
        assert_eq!(printed.borrow()[0], b"Saves\\save");
        // The statistics were deleted (the map's destructor, flag 1) and
        // the pointer cleared.
        assert!(e.get(rig.game, TESSaveLoadGame::m_pSaveLoadStats).is_null());
        assert_eq!(calls_to(e, 0x0200_4100).len(), 1);
    }

    #[test]
    fn flush_calls_the_file_helper() {
        let mut e = game();
        stub(&mut e, FILE_FLUSH);
        fn_00857210(&mut e, Ptr::new(0x1234));
        assert_eq!(calls_to(&e, FILE_FLUSH), vec![vec![0x1234]]);
    }

    #[test]
    fn set_saving_form_header_stores_the_pointer() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        fn_00857230(&mut e, game_object, Ptr::new(0x4444));
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pCurrentlySavingFormHeader)
                .addr(),
            0x4444
        );
    }

    #[test]
    fn save_stats_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = game();
        install_byte_entries(&mut e, &[]);
        let stats: Ptr<SaveStats> = e.new_object();
        assert_eq!(fn_00857250(&mut e, stats, 0), stats);
        assert!(!freed(&e, stats.addr()));
        fn_00857250(&mut e, stats, 1);
        assert!(freed(&e, stats.addr()));
    }

    #[test]
    fn form_and_flags_constructor_stores_the_four_fields() {
        let mut e = game();
        let record: Ptr<FormAndFlags> = e.new_object();
        let result = fn_00857280(&mut e, record, Ptr::new(0x1111), 5, 6, 9);
        assert_eq!(result, record);
        assert_eq!(e.get(record, FormAndFlags::pForm).addr(), 0x1111);
        assert_eq!(e.get(record, FormAndFlags::iFlags), 5);
        assert_eq!(e.get(record, FormAndFlags::iOldFlags), 6);
        assert_eq!(e.get(record, FormAndFlags::cVersion), 9);
    }

    #[test]
    fn local_record_initializer_clears_the_words_and_runs_the_embedded_one() {
        let mut e = game();
        stub(&mut e, EMBEDDED_RECORD_INIT);
        let local = e.mem.alloc(0x40);
        e.mem.write(local, &[0xFF; 16]);
        assert_eq!(fn_008572c0(&mut e, Ptr::new(local)).addr(), local);
        assert_eq!(e.mem.u32(local), 0);
        assert_eq!(e.mem.u32(local + 4), 0);
        assert_eq!(calls_to(&e, EMBEDDED_RECORD_INIT), vec![vec![local + 8]]);
    }

    // ---- the second batch of functions: helpers ----

    /// Doubles that return 0 (calls are logged).
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            stub(e, address);
        }
    }

    /// `__RTDynamicCast` over a table of (object, target type, result); any
    /// other cast gives null.
    fn casts(e: &mut Engine, table: &[(u32, u32, u32)]) {
        let table = table.to_vec();
        e.register_double(DYNAMIC_CAST, move |_, a| {
            returns(
                table
                    .iter()
                    .find(|(object, target, _)| *object == a[0] && *target == a[3])
                    .map_or(0, |entry| entry.2),
            )
        });
    }

    /// Maps the pages of the globals the second batch reads.
    fn map_globals(e: &mut Engine) {
        for page in [
            0x011c_3000u32,
            0x011d_e000,
            0x011a_9000,
            0x011f_4000,
            0x0107_f000,
            0x0118_7000,
            0x0120_2000,
            0x0108_0000,
            0x0107_c000,
        ] {
            e.map(page, 0x1000);
        }
    }

    /// A game object whose current buffer is a fresh block of `size` bytes.
    fn game_with_buffer(e: &mut Engine, size: u32) -> (Ptr<TESSaveLoadGame>, u32) {
        let game: Ptr<TESSaveLoadGame> = e.new_object();
        let buffer = e.mem.alloc(size);
        e.set(game, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        (game, buffer)
    }

    /// An object with a vtable whose slots are `(offset, address)`.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(offset, target) in slots {
            e.mem.set_u32(vtable + offset, target);
        }
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The text at an address, as a `String`.
    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    // ---- the second batch of functions ----

    #[test]
    fn reference_data_constructor_clears_the_location_id() {
        let mut e = game();
        let record = e.mem.alloc(0x1C);
        e.mem.set_u32(record, 0xDEAD);
        let result = fn_008572f0(&mut e, Ptr::new(record));
        assert_eq!(result.addr(), record);
        assert_eq!(e.mem.u32(record), 0);
        // The two embedded points are built (their constructor does nothing).
        assert_eq!(
            calls_to(&e, LIST_NODE_ITEM),
            vec![vec![record + 4], vec![record + 0x10]]
        );
    }

    #[test]
    fn moved_reference_constructor_builds_the_inner_reference_data() {
        let mut e = game();
        let record = e.mem.alloc(0x2C);
        e.mem.set_u32(record, 0xDEAD);
        e.mem.set_u32(record + 0x10, 0xBEEF);
        let result = fn_00857320(&mut e, Ptr::new(record));
        assert_eq!(result.addr(), record);
        assert_eq!(e.mem.u32(record), 0);
        assert_eq!(e.mem.u32(record + 0x10), 0);
        assert_eq!(
            calls_to(&e, LIST_NODE_ITEM),
            vec![vec![record + 4], vec![record + 0x14], vec![record + 0x20]]
        );
    }

    #[test]
    fn init_array_destructor_runs_the_array_destructor() {
        let mut e = game();
        stub(&mut e, INIT_ARRAY_DESTRUCT);
        fn_00857350(&mut e, Ptr::new(0x4321));
        assert_eq!(calls_to(&e, INIT_ARRAY_DESTRUCT), vec![vec![0x4321]]);
    }

    /// What the file-opening doubles saw.
    struct OpenRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        events: Rc<RefCell<Vec<String>>>,
        /// The `BSFile` constructor's (path, write mode, buffer size).
        constructed: Rc<RefCell<Vec<(String, u32, u32)>>>,
    }

    /// The file-opening world: "base\" and "Saves\" are the path pieces,
    /// string functions work on memory, files named in `existing` exist.
    fn open_rig(existing: &[&str]) -> OpenRig {
        let mut e = game();
        map_globals(&mut e);
        let events: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        let prefix = e.mem.alloc(16);
        e.mem.set_cstr(prefix, b"base\\");
        let folder = e.mem.alloc(16);
        e.mem.set_cstr(folder, b"Saves\\");
        constant(&mut e, PATH_PREFIX, prefix);
        constant(&mut e, PATH_OBJECT_GET, folder);
        e.register(IDENTITY, |_, a| returns(a[0]));
        e.register(STRING_CAT, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        e.register(FORMAT, |e, a| {
            // "%s%s%s.ess"
            let mut joined = e.mem.cstr(a[3]);
            joined.extend(e.mem.cstr(a[4]));
            joined.extend(e.mem.cstr(a[5]));
            joined.extend(b".ess");
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        e.mem.set_cstr(BAK_EXTENSION, b".bak");
        e.mem.set_cstr(ESS_EXTENSION, b".ess");
        e.mem.set_cstr(SAVE_NAME_PREFIX, b"Save ");
        e.mem.set_cstr(AUTOSAVE_NAME, b"autosave");
        let existing: Vec<String> = existing.iter().map(|s| s.to_string()).collect();
        let sink = events.clone();
        e.register_double(FILE_EXISTS, move |e, a| {
            let path = text(e, a[0]);
            sink.borrow_mut().push(format!("exists {path}"));
            returns(existing.contains(&path) as u32)
        });
        let sink = events.clone();
        e.register_double(DELETE_FILE_IMPORT, move |e, a| {
            sink.borrow_mut().push(format!("delete {}", text(e, a[0])));
            returns(1)
        });
        let sink = events.clone();
        e.register_double(CREATE_DIRECTORY_IMPORT, move |e, a| {
            sink.borrow_mut().push(format!("mkdir {}", text(e, a[0])));
            returns(1)
        });
        let sink = events.clone();
        e.register_double(RENAME, move |e, a| {
            sink.borrow_mut()
                .push(format!("rename {} {}", text(e, a[0]), text(e, a[1])));
            returns(0)
        });
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            returns(
                bytes
                    .iter()
                    .rposition(|&b| b == a[1] as u8)
                    .map_or(0, |i| a[0] + i as u32),
            )
        });
        e.register(STRNICMP, |e, a| {
            let (left, right) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let n = a[2] as usize;
            let cut = |v: &[u8]| {
                v.iter()
                    .take(n)
                    .map(u8::to_ascii_lowercase)
                    .collect::<Vec<u8>>()
            };
            returns(cut(&left).cmp(&cut(&right)) as i32 as u32)
        });
        // `strstr`: the address of the match, or null.
        e.register(STRING_FIND, |e, a| {
            let (text, pattern) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let found = text
                .windows(pattern.len().max(1))
                .position(|window| window == pattern.as_slice());
            returns(found.map_or(0, |at| a[0] + at as u32))
        });
        e.register(DEFAULT_SAVE_NAME, |e, a| {
            e.mem.set_cstr(a[1], b"Save 9");
            Ret::default()
        });
        // A file object: name at +4, the vtable has the destructor (0), the
        // name (0x18) and `Open` (0x20).
        e.register_double(0x0300_0000, |_, _| Ret::default());
        e.register(0x0300_0018, |e, a| returns(e.mem.u32(a[0] + 4)));
        let sink = events.clone();
        e.register_double(0x0300_0020, move |_, a| {
            sink.borrow_mut()
                .push(format!("open {} {} {}", a[0], a[1], a[2]));
            returns(1)
        });
        let sink = events.clone();
        e.register_double(0x0300_0000, move |_, a| {
            sink.borrow_mut().push(format!("destroy {} {}", a[0], a[1]));
            Ret::default()
        });
        e.register(FILE_DELETE, |_, _| returns(1));
        let constructed: Rc<RefCell<Vec<(String, u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let sink = constructed.clone();
        e.register_double(BSFILE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((text(e, a[1]), a[2], a[3]));
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable, 0x0300_0000);
            e.mem.set_u32(vtable + 0x18, 0x0300_0018);
            e.mem.set_u32(vtable + 0x20, 0x0300_0020);
            e.mem.set_u32(a[0], vtable);
            returns(a[0])
        });
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        OpenRig {
            e,
            game: game_object,
            events,
            constructed,
        }
    }

    /// A file object like the ones `BSFile::BSFile` makes, named `name`.
    fn file_named(e: &mut Engine, name: &[u8]) -> Ptr {
        let file = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, 0x0300_0000);
        e.mem.set_u32(vtable + 0x18, 0x0300_0018);
        e.mem.set_u32(vtable + 0x20, 0x0300_0020);
        e.mem.set_u32(file, vtable);
        let name_block = e.mem.alloc(0x80);
        e.mem.set_cstr(name_block, name);
        e.mem.set_u32(file + 4, name_block);
        Ptr::new(file)
    }

    #[test]
    fn opening_a_save_for_writing_rotates_the_old_save_to_bak() {
        let mut rig = open_rig(&["base\\Saves\\quick.ess", "base\\Saves\\quick.bak"]);
        let name = rig.e.mem.alloc(16);
        rig.e.mem.set_cstr(name, b"quick");
        let file = fn_00857370(&mut rig.e, rig.game, Ptr::NULL, Ptr::new(name), 0);
        assert!(!file.is_null());
        assert_eq!(
            *rig.events.borrow(),
            vec![
                "mkdir base\\Saves\\",
                "exists base\\Saves\\quick.ess",
                "exists base\\Saves\\quick.bak",
                "delete base\\Saves\\quick.bak",
                "rename base\\Saves\\quick.ess base\\Saves\\quick.bak",
            ]
        );
        // The file is opened for writing with a 0x20000 byte buffer and is
        // not opened again.
        assert_eq!(
            *rig.constructed.borrow(),
            vec![("base\\Saves\\quick.ess".to_string(), 1, 0x20000)]
        );
    }

    #[test]
    fn opening_a_save_without_an_old_one_does_not_rename() {
        let mut rig = open_rig(&[]);
        let default_name = fn_00857370(&mut rig.e, rig.game, Ptr::NULL, Ptr::NULL, 0);
        assert!(!default_name.is_null());
        // The default name (`00860ae0`) is "Save 9".
        assert_eq!(
            *rig.constructed.borrow(),
            vec![("base\\Saves\\Save 9.ess".to_string(), 1, 0x20000)]
        );
        assert_eq!(
            *rig.events.borrow(),
            vec!["mkdir base\\Saves\\", "exists base\\Saves\\Save 9.ess"]
        );
    }

    #[test]
    fn opening_a_save_for_reading_opens_the_file() {
        let mut rig = open_rig(&[]);
        let name = rig.e.mem.alloc(16);
        rig.e.mem.set_cstr(name, b"quick");
        let file = fn_00857370(&mut rig.e, rig.game, Ptr::NULL, Ptr::new(name), 1);
        assert_eq!(
            *rig.constructed.borrow(),
            vec![("base\\Saves\\quick.ess".to_string(), 0, 0x20000)]
        );
        // Nothing is rotated, and the slot at 0x20 is called with two zeros.
        assert_eq!(
            *rig.events.borrow(),
            vec![format!("open {} 0 0", file.addr())]
        );
    }

    #[test]
    fn opening_with_a_file_reopens_it_or_hands_it_back() {
        let mut rig = open_rig(&[]);
        let file = file_named(&mut rig.e, b"base\\Saves\\quick.ess");
        // Mode 2 opens the file again.
        let again = fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 2);
        assert_eq!(again, file);
        assert_eq!(
            *rig.events.borrow(),
            vec![format!("open {} 0 0", file.addr())]
        );
        // Mode 3 and an unknown mode return the file untouched.
        let before = rig.events.borrow().len();
        assert_eq!(fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 3), file);
        assert_eq!(fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 9), file);
        assert_eq!(rig.events.borrow().len(), before);
        assert!(rig.constructed.borrow().is_empty());
    }

    #[test]
    fn saving_over_a_numbered_save_deletes_it_and_opens_the_default_name() {
        let mut rig = open_rig(&[]);
        let file = file_named(&mut rig.e, b"base\\Saves\\Save 3 Vault.ess");
        let result = fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 0);
        assert!(!result.is_null());
        // The old save is deleted and its stream destroyed (flag 1) ...
        let events = rig.events.borrow().clone();
        assert!(events.contains(&format!("destroy {} 1", file.addr())));
        assert_eq!(calls_to(&rig.e, FILE_DELETE).len(), 1);
        // ... and the new one is opened under the default name ("Save 9").
        assert_eq!(
            rig.constructed.borrow()[0].0,
            "base\\Saves\\Save 9.ess".to_string()
        );
    }

    #[test]
    fn saving_over_an_autosave_opens_the_default_name() {
        let mut rig = open_rig(&[]);
        let file = file_named(&mut rig.e, b"base\\Saves\\autosave.ess");
        fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 0);
        assert_eq!(
            rig.constructed.borrow()[0].0,
            "base\\Saves\\Save 9.ess".to_string()
        );
    }

    #[test]
    fn saving_over_a_quicksave_keeps_the_name() {
        let mut rig = open_rig(&[]);
        let file = file_named(&mut rig.e, b"base\\Saves\\quicksave.ess");
        fn_00857370(&mut rig.e, rig.game, file, Ptr::NULL, 0);
        assert_eq!(
            rig.constructed.borrow()[0].0,
            "base\\Saves\\quicksave.ess".to_string()
        );
    }

    #[test]
    fn letting_go_of_a_file_closes_or_destroys_it() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let file = object_with_slots(&mut e, &[(0, 0x0300_0000)]);
        e.register(0x0300_0000, |_, _| Ret::default());
        quiet(&mut e, &[BSFILE_CLOSE, LIST_REMOVE]);
        // Mode 2 closes.
        fn_008578b0(&mut e, game_object, Ptr::new(file), 2);
        assert_eq!(calls_to(&e, BSFILE_CLOSE), vec![vec![file]]);
        assert!(calls_to(&e, 0x0300_0000).is_empty());
        // Mode 3 without a list destroys it, with flag 1.
        fn_008578b0(&mut e, game_object, Ptr::new(file), 3);
        assert_eq!(calls_to(&e, 0x0300_0000), vec![vec![file, 1]]);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        // With a list the file is removed from it first.
        e.set(
            game_object,
            TESSaveLoadGame::m_pSaveGameList,
            Ptr::new(0x5000),
        );
        fn_008578b0(&mut e, game_object, Ptr::new(file), 0);
        let removed = calls_to(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], 0x5000);
        // Null files and other modes do nothing.
        fn_008578b0(&mut e, game_object, Ptr::NULL, 0);
        fn_008578b0(&mut e, game_object, Ptr::new(file), 7);
        assert_eq!(calls_to(&e, 0x0300_0000).len(), 2);
        assert_eq!(calls_to(&e, BSFILE_CLOSE).len(), 1);
    }

    #[test]
    fn deleting_a_save_file_removes_the_named_file_and_the_stream() {
        let mut rig = open_rig(&[]);
        let file = file_named(&mut rig.e, b"base\\Saves\\old.ess");
        fn_00857950(&mut rig.e, rig.game, file, Ptr::NULL);
        let deleted = calls_to(&rig.e, FILE_DELETE);
        assert_eq!(deleted.len(), 1);
        assert_eq!(text(&rig.e, deleted[0][0]), "base\\Saves\\old.ess");
        assert!(rig
            .events
            .borrow()
            .contains(&format!("destroy {} 1", file.addr())));
        // A null file does nothing.
        fn_00857950(&mut rig.e, rig.game, Ptr::NULL, Ptr::NULL);
        assert_eq!(calls_to(&rig.e, FILE_DELETE).len(), 1);
    }

    #[test]
    fn buffer_writes_and_reads_move_the_buffer_pointer() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 32);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3, 4, 5, 6]);
        fn_008579b0(&mut e, game_object, Ptr::new(source), 6);
        assert_eq!(e.mem.bytes(buffer, 6), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 6
        );
        // Reading takes the bytes from the buffer's current position.
        e.mem.write(buffer + 6, &[9, 8, 7, 6]);
        let target = e.mem.alloc(8);
        fn_008579e0(&mut e, game_object, Ptr::new(target), 4);
        assert_eq!(e.mem.bytes(target, 4), vec![9, 8, 7, 6]);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 10
        );
    }

    #[test]
    fn numeric_ids_are_saved_through_the_id_array_when_it_is_used() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 32);
        let ids = e.mem.alloc(16);
        e.mem
            .write(ids, &[0x11, 0, 0, 0, 0x22, 0, 0, 0, 0x33, 0, 0, 0]);
        // Without the array the ids are written as they are (size / 4 of
        // them: 9 bytes is two ids).
        constant(&mut e, USE_NUMERIC_IDS, 0);
        tes_save_load_game_save_numeric_id(&mut e, game_object, Ptr::new(ids), 9);
        assert_eq!(e.mem.u32(buffer), 0x11);
        assert_eq!(e.mem.u32(buffer + 4), 0x22);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 8
        );
        // With the array each id goes through `AddNumericIDToArray`.
        constant(&mut e, USE_NUMERIC_IDS, 1);
        e.register(ADD_NUMERIC_ID, |_, a| returns(a[1] + 0x1000));
        tes_save_load_game_save_numeric_id(&mut e, game_object, Ptr::new(ids), 12);
        assert_eq!(e.mem.u32(buffer + 8), 0x1011);
        assert_eq!(e.mem.u32(buffer + 12), 0x1022);
        assert_eq!(e.mem.u32(buffer + 16), 0x1033);
    }

    #[test]
    fn numeric_ids_are_loaded_and_resolved_when_the_array_is_used() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 32);
        e.mem.write(buffer, &[5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]);
        let ids = e.mem.alloc(16);
        // Without the array: copied as they are.
        constant(&mut e, USE_NUMERIC_IDS, 0);
        assert!(!tes_save_load_game_load_numeric_id(
            &mut e,
            game_object,
            Ptr::new(ids),
            12
        ));
        assert_eq!(e.mem.u32(ids), 5);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 12
        );
        // With the array: 6 does not resolve (gives 0), 0 stays 0 without
        // counting as a failure.
        constant(&mut e, USE_NUMERIC_IDS, 1);
        e.register(RESOLVE_NUMERIC_ID, |_, a| {
            returns(if a[1] == 6 { 0 } else { a[1] + 100 })
        });
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        let failed = tes_save_load_game_load_numeric_id(&mut e, game_object, Ptr::new(ids), 12);
        assert!(failed);
        assert_eq!(e.mem.u32(ids), 105);
        assert_eq!(e.mem.u32(ids + 4), 0);
        // Only 0 and resolvable ids: no failure.
        e.mem.write(buffer, &[5, 0, 0, 0, 0, 0, 0, 0]);
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        assert!(!tes_save_load_game_load_numeric_id(
            &mut e,
            game_object,
            Ptr::new(ids),
            8
        ));
    }

    #[test]
    fn writing_bytes_goes_to_the_file_or_is_only_counted() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        e.register(FILE_WRITE, |_, a| returns(a[2] + 100));
        assert_eq!(
            fn_00857b50(&mut e, game_object, Ptr::new(0x10), Ptr::new(0x20), 6),
            106
        );
        assert_eq!(calls_to(&e, FILE_WRITE), vec![vec![0x10, 0x20, 6]]);
        // When the game only measures (`0047c850` true) the size is added.
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(1));
        assert_eq!(
            fn_00857b50(&mut e, game_object, Ptr::new(0x10), Ptr::new(0x20), 4),
            4
        );
        fn_00857b50(&mut e, game_object, Ptr::new(0x10), Ptr::new(0x20), 5);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_iSimulationFileSize),
            9
        );
        assert_eq!(calls_to(&e, FILE_WRITE).len(), 1);
    }

    #[test]
    fn reading_bytes_forwards_to_the_file() {
        let mut e = game();
        e.register(FILE_READ, |_, a| returns(a[2] * 2));
        let result = fn_00857ba0(&mut e, Ptr::new(0x1), Ptr::new(0x30), Ptr::new(0x40), 5);
        assert_eq!(result, 10);
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![0x30, 0x40, 5]]);
    }

    #[test]
    fn advancing_the_buffer_adds_the_count() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 16);
        fn_00857bd0(&mut e, game_object, 3);
        fn_00857bd0(&mut e, game_object, 4);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 7
        );
    }

    fn plugin_table(e: &mut Engine, game_object: Ptr<TESSaveLoadGame>, table: &[u8]) {
        let block = e.mem.alloc(16);
        e.mem.write(block, table);
        e.set(
            game_object,
            TESSaveLoadGame::m_pFileIndexArray,
            Ptr::new(block),
        );
        e.set(
            game_object,
            TESSaveLoadGame::m_iSavedPluginCount,
            table.len() as u8,
        );
    }

    #[test]
    fn saved_ids_are_mapped_to_the_current_plugin_index() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        // No table: unchanged.
        assert_eq!(fn_00857bf0(&mut e, game_object, 0x0200_1234), 0x0200_1234);
        plugin_table(&mut e, game_object, &[0x07, 0xFF, 0x02]);
        assert_eq!(fn_00857bf0(&mut e, game_object, 0x0012_3456), 0x0712_3456);
        assert_eq!(fn_00857bf0(&mut e, game_object, 0x0212_3456), 0x0212_3456);
        // Mapped to 0xFF, or beyond the table: 0. Plugin 0xFF: unchanged.
        assert_eq!(fn_00857bf0(&mut e, game_object, 0x0112_3456), 0);
        assert_eq!(fn_00857bf0(&mut e, game_object, 0x0312_3456), 0);
        assert_eq!(fn_00857bf0(&mut e, game_object, 0xFF12_3456), 0xFF12_3456);
    }

    #[test]
    fn current_ids_are_mapped_back_to_the_saved_plugin_index() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        assert_eq!(fn_00857c70(&mut e, game_object, 0x0200_1234), 0x0200_1234);
        plugin_table(&mut e, game_object, &[0x05, 0x07, 0x05]);
        // 5 is at indexes 0 and 2: the last one wins.
        assert_eq!(fn_00857c70(&mut e, game_object, 0x0512_3456), 0x0212_3456);
        assert_eq!(fn_00857c70(&mut e, game_object, 0x0712_3456), 0x0112_3456);
        assert_eq!(fn_00857c70(&mut e, game_object, 0x0912_3456), 0);
        assert_eq!(fn_00857c70(&mut e, game_object, 0xFF12_3456), 0xFF12_3456);
    }

    /// The reference world of `fn_00857d10`: one object that is both the
    /// reference and the actor, with the virtual slots it calls.
    struct PlaceRig {
        e: Engine,
        buffer: Ptr,
        reference: Ptr,
    }

    fn place_rig(actor: bool, saved_location: bool) -> PlaceRig {
        let mut e = game();
        // Slots: 0x100 actor test, 0x290 has location, 0x294 world space,
        // 0x298 cell, 0x170 location, 0x16C rotation, 0x1F4 position.
        for (slot, value) in [
            (0x100u32, actor as u32),
            (0x290, saved_location as u32),
            (0x294, 0x5555),
            (0x298, 0xCE11),
        ] {
            constant(&mut e, 0x0310_0000 + slot, value);
        }
        e.register(0x0310_0170, |e, a| {
            for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *value);
            }
            returns(a[1])
        });
        e.register(0x0310_016C, |e, a| {
            for (i, value) in [4.0f32, 5.0, 6.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *value);
            }
            returns(a[1])
        });
        let position = e.mem.alloc(16);
        for (i, value) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *value);
        }
        constant(&mut e, 0x0310_01F4, position);
        let slots: Vec<(u32, u32)> = [0x100u32, 0x290, 0x294, 0x298, 0x170, 0x16C, 0x1F4]
            .iter()
            .map(|slot| (*slot, 0x0310_0000 + slot))
            .collect();
        let reference = Ptr::new(object_with_slots(&mut e, &slots));
        let buffer = Ptr::new(e.mem.alloc(0x40));
        constant(&mut e, BUFFER_GET_VERSION, 9);
        quiet(
            &mut e,
            &[
                BUFFER_SET_VERSION,
                SET_LOADING_STATE,
                REF_SET_POSITION,
                FN_005757D0,
                FN_00575700,
                REF_MOVE_TO_SPACE,
                FN_00440460,
                FN_0043FA80,
                COLLISION_RESET_SIM,
                FN_0043D410,
                FN_00A59C60,
                CHAR_CONTROLLER_SET_POSITION,
                EXTRA_GET_STARTING_SPACE,
            ],
        );
        casts(&mut e, &[]);
        PlaceRig {
            e,
            buffer,
            reference,
        }
    }

    #[test]
    fn a_saved_actor_location_moves_the_reference_and_its_node() {
        let mut rig = place_rig(true, true);
        let e = &mut rig.e;
        constant(e, REF_GET_NODE, 0x9900);
        constant(e, REF_GET_ORIENTATION, 0x9A00);
        let mobile = 0x9B00;
        casts(e, &[(rig.reference.addr(), RTTI_MOBILE_OBJECT, mobile)]);
        constant(e, MOBILE_GET_CHAR_CONTROLLER, 0x9C00);
        constant(e, CHAR_CONTROLLER_TEST, 0);
        // The positions passed by address live in blocks the game frees, so
        // read their first float when the call is made.
        let seen: Rc<RefCell<Vec<(u32, f32)>>> = Rc::new(RefCell::new(vec![]));
        for address in [REF_SET_POSITION, CHAR_CONTROLLER_SET_POSITION] {
            let sink = seen.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, e.mem.f32(a[1])));
                Ret::default()
            });
        }
        let moved = fn_00857d10(e, rig.buffer, rig.reference, true);
        assert!(moved);
        // The position is the location the actor gave (1, 2, 3) and the
        // character controller gets the reference's position (7, 8, 9); the
        // angle is the z of the rotation.
        assert_eq!(
            *seen.borrow(),
            vec![(REF_SET_POSITION, 1.0), (CHAR_CONTROLLER_SET_POSITION, 7.0)]
        );
        assert_eq!(calls_to(e, REF_SET_POSITION)[0][0], rig.reference.addr());
        assert_eq!(
            calls_to(e, FN_005757D0),
            vec![vec![rig.reference.addr(), 6.0f32.to_bits()]]
        );
        // Teleporting brackets the move; the cell and the world space follow.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|a| [SET_LOADING_STATE, REF_MOVE_TO_SPACE].contains(a))
            .collect();
        assert_eq!(
            order,
            vec![SET_LOADING_STATE, REF_MOVE_TO_SPACE, SET_LOADING_STATE]
        );
        assert_eq!(
            calls_to(e, REF_MOVE_TO_SPACE),
            vec![vec![rig.reference.addr(), 0xCE11, 0x5555]]
        );
        // The node: the character controller gets the position, the node the
        // position and the orientation, the collision is reset.
        assert_eq!(calls_to(e, CHAR_CONTROLLER_SET_POSITION).len(), 1);
        assert_eq!(calls_to(e, CHAR_CONTROLLER_SET_POSITION)[0][0], 0x9C00);
        assert_eq!(calls_to(e, FN_0043FA80), vec![vec![0x9900, 0x9A00]]);
        assert_eq!(calls_to(e, COLLISION_RESET_SIM), vec![vec![0x9900, 1]]);
        assert_eq!(calls_to(e, FN_00A59C60).len(), 1);
        // The version byte is cleared and put back.
        let versions = calls_to(e, BUFFER_SET_VERSION);
        assert_eq!(
            versions,
            vec![vec![rig.buffer.addr(), 0], vec![rig.buffer.addr(), 9]]
        );
    }

    #[test]
    fn a_reference_without_saved_data_is_not_moved() {
        let mut rig = place_rig(false, false);
        let e = &mut rig.e;
        constant(e, REF_GET_EXTRA_LIST, 0x7000);
        constant(e, EXTRA_GET_DATA, 0);
        let moved = fn_00857d10(e, rig.buffer, rig.reference, false);
        assert!(!moved);
        assert!(calls_to(e, REF_MOVE_TO_SPACE).is_empty());
        assert!(calls_to(e, REF_GET_NODE).is_empty());
        assert_eq!(calls_to(e, BUFFER_SET_VERSION).len(), 2);
    }

    #[test]
    fn extra_data_gives_the_position_and_the_starting_space() {
        let mut rig = place_rig(false, false);
        let e = &mut rig.e;
        constant(e, REF_GET_EXTRA_LIST, 0x7000);
        constant(e, EXTRA_GET_DATA, 1);
        constant(e, EXTRA_GET_STARTING_SPACE, 0x7100);
        constant(e, REF_GET_NODE, 0);
        casts(e, &[(0x7100, RTTI_CELL, 0x7100)]);
        let moved = fn_00857d10(e, rig.buffer, rig.reference, true);
        assert!(moved);
        // Extra data of type 0xF was asked of the list.
        assert_eq!(calls_to(e, EXTRA_GET_DATA), vec![vec![0x7000, 0xF]]);
        // The rotation words come from the out buffer (4, 5, 6).
        assert_eq!(
            calls_to(e, FN_00575700),
            vec![vec![
                rig.reference.addr(),
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                6.0f32.to_bits()
            ]]
        );
        // The starting cell moves the reference (no world space), and the
        // teleport bracket is not used on this path.
        assert_eq!(
            calls_to(e, REF_MOVE_TO_SPACE),
            vec![vec![rig.reference.addr(), 0x7100, 0]]
        );
        assert!(calls_to(e, SET_LOADING_STATE).is_empty());
    }

    /// The doubles and the log of `SaveGlobalData`.
    fn global_data_rig() -> (Engine, Ptr<TESSaveLoadGame>, ByteWrites) {
        let mut e = game();
        map_globals(&mut e);
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let handler = e.mem.alloc(0x20);
        let tes = e.mem.alloc(0x20);
        let player = e.mem.alloc(0x20);
        e.set_global(DATA_HANDLER, handler);
        e.set_global(TES_OBJECT, tes);
        e.set_global(PLAYER, player);
        constant(&mut e, GLOBAL_DATA_SIZE, 0x1122_3344);
        let tes_worldspace = e.mem.alloc(0x20);
        e.mem.set_u32(tes_worldspace + 0xC, 0x3C);
        constant(&mut e, TES_GET_WORLDSPACE, tes_worldspace);
        constant(&mut e, READ_FIELD_24, 5);
        constant(&mut e, READ_FIELD_28, 6);
        let worldspace = e.mem.alloc(0x20);
        e.mem.set_u32(worldspace + 0xC, 0x77);
        constant(&mut e, REF_GET_WORLDSPACE, worldspace);
        constant(&mut e, REF_GET_PARENT_CELL, 0);
        let position = e.mem.alloc(16);
        e.mem.set_f32(position, 1.5);
        constant(&mut e, REF_GET_POSITION, position);
        quiet(
            &mut e,
            &[
                GLOBALS_LIST,
                LIST_COUNT,
                SAVE_CREATED_BASE_OBJECTS,
                TES_SAVE,
                PROCESS_LISTS_SAVE,
                SKY_SAVE,
                INTERFACE_SAVE,
                REGIONS_SAVE,
                SKY_INSTANCE,
                ERROR,
                TES_SAVE_SIZE,
                PROCESS_LISTS_SAVE_SIZE,
                SKY_SAVE_SIZE,
                INTERFACE_SAVE_SIZE,
                REGIONS_SAVE_SIZE,
            ],
        );
        let writes: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));
        let sink = writes.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            returns(a[2])
        });
        (e, game_object, writes)
    }

    #[test]
    fn global_data_writes_the_header_words_and_the_blocks() {
        let (mut e, game_object, writes) = global_data_rig();
        constant(&mut e, TES_SAVE_SIZE, 3);
        tes_save_load_game_save_global_data(&mut e, game_object, Ptr::new(0x6000));
        let writes = writes.borrow();
        let lengths: Vec<usize> = writes.iter().map(Vec::len).collect();
        // handler size, TES world space id, two TES words, player location
        // id and position, the globals block, then the TES size word and
        // its block (3 bytes), the other blocks' size words (all 0), the
        // zero word before the created base objects, and the reticle,
        // interface and regions size words.
        assert_eq!(lengths, vec![4, 4, 4, 4, 4, 12, 2, 2, 3, 2, 2, 4, 2, 2, 2]);
        assert_eq!(writes[0], 0x1122_3344u32.to_le_bytes());
        assert_eq!(writes[1], 0x3Cu32.to_le_bytes());
        assert_eq!(writes[2], 5u32.to_le_bytes());
        assert_eq!(writes[3], 6u32.to_le_bytes());
        // The player is in a world space: its id is the location.
        assert_eq!(writes[4], 0x77u32.to_le_bytes());
        assert_eq!(writes[5][0..4], 1.5f32.to_le_bytes());
        assert_eq!(writes[7], 3u16.to_le_bytes());
        assert_eq!(calls_to(&e, TES_SAVE).len(), 1);
        assert!(calls_to(&e, PROCESS_LISTS_SAVE).is_empty());
        assert_eq!(
            calls_to(&e, SAVE_CREATED_BASE_OBJECTS),
            vec![vec![game_object.addr(), 0x6000]]
        );
        assert!(calls_to(&e, ERROR).is_empty());
        // The buffer made for the block was freed.
        assert!(e.get(game_object, TESSaveLoadGame::m_pBuffer).is_null());
    }

    #[test]
    fn global_data_complains_when_the_player_has_no_location() {
        let (mut e, game_object, writes) = global_data_rig();
        constant(&mut e, REF_GET_WORLDSPACE, 0);
        tes_save_load_game_save_global_data(&mut e, game_object, Ptr::new(0x6000));
        assert_eq!(calls_to(&e, ERROR), vec![vec![MSG_PLAYER_HAS_NO_SPACE]]);
        assert_eq!(writes.borrow()[4], 0u32.to_le_bytes());
    }

    #[test]
    fn global_data_uses_the_parent_cell_and_notes_blocks_in_the_statistics() {
        let (mut e, game_object, writes) = global_data_rig();
        let cell = e.mem.alloc(0x20);
        e.mem.set_u32(cell + 0xC, 0x99);
        constant(&mut e, REF_GET_WORLDSPACE, 0);
        constant(&mut e, REF_GET_PARENT_CELL, cell);
        constant(&mut e, PROCESS_LISTS_SAVE_SIZE, 5);
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pSaveLoadStats, stats);
        // `AddExtraStat` (this file's own) puts an `ExtraStat` on the list
        // at `pExtraStats`: note the sizes it was given.
        let sizes: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = sizes.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            let stat = e.mem.u32(a[1]);
            sink.borrow_mut().push(e.mem.u32(stat));
            Ret::default()
        });
        tes_save_load_game_save_global_data(&mut e, game_object, Ptr::new(0x6000));
        assert_eq!(writes.borrow()[4], 0x99u32.to_le_bytes());
        assert_eq!(calls_to(&e, PROCESS_LISTS_SAVE), vec![vec![PROCESS_LISTS]]);
        // The globals block (2 bytes) and the process lists block (5) are
        // noted.
        assert_eq!(*sizes.borrow(), vec![2, 5]);
    }

    #[test]
    fn globals_are_written_as_a_count_and_id_value_pairs() {
        let (mut e, game_object, writes) = global_data_rig();
        let variable = e.mem.alloc(0x20);
        e.mem.set_u32(variable + 0xC, 0x42);
        let list = list_of(&mut e, &[variable, 0]);
        constant(&mut e, GLOBALS_LIST, list);
        constant(&mut e, LIST_COUNT, 2);
        constant(&mut e, USE_NUMERIC_IDS, 0);
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        tes_save_load_game_save_globals(&mut e, game_object, Ptr::new(0x6000));
        let writes = writes.borrow();
        assert_eq!(writes.len(), 1);
        // 2 * 8 + 2 bytes: the count, then one pair (the second item is null).
        assert_eq!(writes[0].len(), 18);
        assert_eq!(writes[0][0..2], 2u16.to_le_bytes());
        assert_eq!(writes[0][2..6], 0x42u32.to_le_bytes());
        assert_eq!(writes[0][6..10], 2.5f32.to_le_bytes());
    }

    #[test]
    fn final_data_writes_the_size_and_the_temp_effects() {
        let (mut e, game_object, writes) = global_data_rig();
        stub(&mut e, TEMP_EFFECTS_SAVE);
        constant(&mut e, TEMP_EFFECTS_SIZE, 0);
        tes_save_load_game_save_final_data(&mut e, game_object, Ptr::new(0x6000));
        assert_eq!(writes.borrow().len(), 1);
        assert_eq!(writes.borrow()[0], 0u32.to_le_bytes());
        assert!(calls_to(&e, TEMP_EFFECTS_SAVE).is_empty());
        constant(&mut e, TEMP_EFFECTS_SIZE, 6);
        tes_save_load_game_save_final_data(&mut e, game_object, Ptr::new(0x6000));
        let writes = writes.borrow();
        assert_eq!(writes.len(), 3);
        assert_eq!(writes[1], 6u32.to_le_bytes());
        assert_eq!(writes[2].len(), 6);
        assert_eq!(calls_to(&e, TEMP_EFFECTS_SAVE), vec![vec![PROCESS_LISTS]]);
    }

    #[test]
    fn create_buffer_makes_the_current_buffer_and_complains_on_failure() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        quiet(&mut e, &[ERROR]);
        let buffer = tes_save_load_game_create_buffer(&mut e, game_object, 24);
        assert!(!buffer.is_null());
        assert_eq!(e.mem.block_size(buffer.addr()), Some(24));
        assert_eq!(e.get(game_object, TESSaveLoadGame::m_pBuffer), buffer);
        assert!(calls_to(&e, ERROR).is_empty());
        // A failed allocation raises the error and gives null.
        constant(&mut e, OPERATOR_NEW, 0);
        let none = tes_save_load_game_create_buffer(&mut e, game_object, 24);
        assert!(none.is_null());
        assert_eq!(calls_to(&e, ERROR), vec![vec![MSG_NO_SAVE_BUFFER]]);
        // The scope is entered with the source line of the call.
        let scopes = calls_to(&e, SCOPE_ENTER);
        assert_eq!(scopes[0][1..], [0x11, 1, SOURCE_FILE, 0x103A]);
    }

    #[test]
    fn write_file_and_read_file_forward_to_the_file_functions() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        e.register(FILE_WRITE, |_, a| returns(a[2]));
        e.register(FILE_READ, |_, a| returns(a[2] + 1));
        tes_save_load_game_write_file(&mut e, game_object, Ptr::new(0x10), Ptr::new(0x20), 7);
        assert_eq!(calls_to(&e, FILE_WRITE), vec![vec![0x10, 0x20, 7]]);
        let read = fn_008586d0(&mut e, game_object, Ptr::new(0x10), Ptr::new(0x20), 7);
        assert_eq!(read, 8);
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![0x10, 0x20, 7]]);
    }

    #[test]
    fn free_buffer_frees_it_and_clears_the_pointer() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 16);
        fn_00858700(&mut e, game_object, Ptr::new(buffer));
        assert!(freed(&e, buffer));
        assert!(e.get(game_object, TESSaveLoadGame::m_pBuffer).is_null());
    }

    /// `0047c850` is true for the first `n` calls and false after that.
    fn measuring_for(e: &mut Engine, n: u32) {
        let left = Rc::new(RefCell::new(n));
        e.register_double(SAVE_LOAD_UNAVAILABLE, move |_, _| {
            let mut left = left.borrow_mut();
            if *left > 0 {
                *left -= 1;
                returns(1)
            } else {
                returns(0)
            }
        });
    }

    #[test]
    fn load_header_is_built_from_the_buffer_bytes() {
        let mut e = game();
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0x10, 0x00, 0x28, 0x05]);
        let header: Ptr<LoadFormHeader> = e.new_object();
        let result = fn_00858aa0(&mut e, header, Ptr::new(source), 0x1234, 0x40);
        assert_eq!(result, header);
        assert_eq!(e.get(header, LoadFormHeader::iFormID), 0x1234);
        assert_eq!(e.get(header, LoadFormHeader::cFormType), 0x28);
        assert_eq!(e.get(header, LoadFormHeader::iFlags), 0x40);
        assert_eq!(e.get(header, LoadFormHeader::cVersion), 5);
        assert_eq!(e.get(header, LoadFormHeader::iSize), 0x10);
    }

    /// The load world of `fn_00858730`: the game is the singleton, a form
    /// `0x1234` of type `form_type` has a `ChangeData` whose buffer starts
    /// with a header of type `saved_type`.
    struct LoadRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        form: Ptr,
        change_data: Ptr<ChangeData>,
        buffer: u32,
    }

    fn load_rig(saved_type: u8, form_type: u8) -> LoadRig {
        let mut e = game();
        map_globals(&mut e);
        let game_object = game_singleton(&mut e);
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let form = Ptr::new(object_with_slots(&mut e, &[(0x60, 0x0320_0060)]));
        e.mem.set_u32(form.addr() + 0xC, 0x1234);
        let buffer = e.mem.alloc(16);
        // The header: size 0x10, form type, version 5.
        e.mem.write(buffer, &[0x10, 0x00, saved_type, 0x05]);
        let data = change_data(&mut e, 0x40, buffer);
        table.borrow_mut().push((0x1234, data.addr()));
        e.set_global(DATA_HANDLER, 0x0055_0000u32);
        constant(&mut e, DATA_HANDLER_HAS_FORM, 0);
        constant(&mut e, FORM_TYPE, form_type as u32);
        quiet(
            &mut e,
            &[
                SECTION_ENTER,
                SECTION_LEAVE,
                SET_LOAD_VERSION,
                SET_LOADING_HEADER,
                SET_LOADING_STATE,
                FORM_FINISH,
                LOAD_INITIAL_DATA,
                END_FORM_PROCESSING,
                CHANGE_DATA_SET_BUFFER,
                0x0320_0060,
            ],
        );
        LoadRig {
            e,
            game: game_object,
            form,
            change_data: data,
            buffer,
        }
    }

    #[test]
    fn load_form_does_nothing_unless_the_game_is_measuring_flag_is_set() {
        let mut rig = load_rig(0x28, 0x28);
        let e = &mut rig.e;
        // `0047c850` is false here.
        assert!(!fn_00858730(e, rig.game, rig.form));
        assert!(calls_to(e, SECTION_ENTER).is_empty());
    }

    #[test]
    fn load_form_needs_a_change_data_with_a_buffer() {
        let mut rig = load_rig(0x28, 0x28);
        let e = &mut rig.e;
        measuring_for(e, 100);
        let unknown = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(unknown.addr() + 0xC, 0x9999);
        assert!(!fn_00858730(e, rig.game, unknown));
        e.set(rig.change_data, ChangeData::pBuffer, Ptr::NULL);
        assert!(!fn_00858730(e, rig.game, rig.form));
        assert!(calls_to(e, SECTION_ENTER).is_empty());
    }

    #[test]
    fn load_form_loads_the_form_from_its_buffer() {
        let mut rig = load_rig(0x28, 0x28);
        let e = &mut rig.e;
        // True for the start and for the saved state; the queued remove is
        // not set, so nothing else asks.
        measuring_for(e, 2);
        let array = e.mem.alloc(0x18);
        constant(e, INIT_ARRAY_CONSTRUCT, array);
        let added: Added = Rc::new(RefCell::new(vec![]));
        let sink = added.clone();
        e.register_double(INIT_ARRAY_ADD, move |e, a| {
            let record = e.mem.u32(a[1]);
            sink.borrow_mut().push((
                a[0],
                e.mem.u32(record),
                e.mem.u32(record + 4),
                e.mem.u32(record + 8),
                e.mem.u8(record + 0xC),
            ));
            Ret::default()
        });
        assert!(fn_00858730(e, rig.game, rig.form));
        // The section is entered with the load name and left again.
        assert_eq!(
            calls_to(e, SECTION_ENTER),
            vec![vec![LOAD_SECTION, MSG_LOAD_FORM_SECTION]]
        );
        assert_eq!(calls_to(e, SECTION_LEAVE), vec![vec![LOAD_SECTION]]);
        // The initial data and the changes are applied with the flags.
        assert_eq!(
            calls_to(e, LOAD_INITIAL_DATA),
            vec![vec![rig.game.addr(), rig.form.addr(), 0x40]]
        );
        assert_eq!(
            calls_to(e, 0x0320_0060),
            vec![vec![rig.form.addr(), 0x40, 0]]
        );
        // The version read from the buffer is used, the form is noted in the
        // init array (made on first use) with the flags and the version.
        assert_eq!(
            calls_to(e, SET_LOAD_VERSION),
            vec![vec![rig.game.addr(), 5]]
        );
        assert_eq!(e.get(rig.game, TESSaveLoadGame::m_pInitArray).addr(), array);
        assert_eq!(*added.borrow(), vec![(array, rig.form.addr(), 0x40, 0, 5)]);
        // The buffer is freed and the change data lets go of it.
        assert!(freed(e, rig.buffer));
        assert_eq!(
            calls_to(e, CHANGE_DATA_SET_BUFFER),
            vec![vec![rig.change_data.addr(), 0]]
        );
        assert!(e.get(rig.game, TESSaveLoadGame::m_pBuffer).is_null());
    }

    #[test]
    fn load_form_applies_a_queued_remove_changes() {
        let mut rig = load_rig(0x28, 0x28);
        let e = &mut rig.e;
        // The start, the saved state and the flag check of `fn_00855150`.
        measuring_for(e, 2);
        constant(e, INIT_ARRAY_CONSTRUCT, 0);
        stub(e, INIT_ARRAY_ADD);
        let array = e.mem.alloc(0x18);
        e.set(rig.game, TESSaveLoadGame::m_pInitArray, Ptr::new(array));
        e.set(rig.game, TESSaveLoadGame::m_iQueuedRemoveChanges, 0x40);
        assert!(fn_00858730(e, rig.game, rig.form));
        // The queued flags were cleared from the change data (`fn_00854e70`)
        // and the request is gone.
        assert_eq!(e.get(rig.game, TESSaveLoadGame::m_iQueuedRemoveChanges), 0);
        assert_eq!(calls_to(e, CHANGE_DATA_SET_BUFFER).len(), 1);
        // The queued flags were looked up in the changes map a second time.
        assert_eq!(calls_to(e, MAP_GET_AT).len(), 2);
    }

    #[test]
    fn load_form_skips_a_form_whose_type_changed() {
        let mut rig = load_rig(0x28, 0x2A);
        let e = &mut rig.e;
        measuring_for(e, 1);
        // The type name table and the form's own type name.
        e.mem.set_u32(FORM_TYPE_NAME_TABLE + 0x28 * 12, 0x0108_0000);
        constant(e, FORM_TYPE_NAME, 0x0108_0010);
        stub(e, SPRINTF);
        assert!(!fn_00858730(e, rig.game, rig.form));
        // The text of the error is formatted with the id and both type names.
        assert_eq!(
            calls_to(e, SPRINTF)[0][1..],
            [FORMAT_LOAD_ERROR, 0x1234, 0x0108_0000, 0x0108_0010]
        );
        // The section is left, nothing was loaded and the buffer is not kept.
        assert_eq!(calls_to(e, SECTION_LEAVE), vec![vec![LOAD_SECTION]]);
        assert!(calls_to(e, LOAD_INITIAL_DATA).is_empty());
        assert!(e.get(rig.game, TESSaveLoadGame::m_pBuffer).is_null());
    }

    /// A form record of the init array: pForm, flags, old flags, version.
    fn form_and_flags(e: &mut Engine, form: u32, flags: u32, old: u32, version: u8) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, form);
        e.mem.set_u32(record + 4, flags);
        e.mem.set_u32(record + 8, old);
        e.mem.set_u8(record + 0xC, version);
        record
    }

    /// The after-load world: an init array of the given records.
    struct InitRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        array: u32,
        player: u32,
        hooks: Hooks,
    }

    fn init_rig(records: &[u32]) -> InitRig {
        let mut e = game();
        map_globals(&mut e);
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        measuring_for(&mut e, 1000);
        let array = e.mem.alloc(0x40);
        e.mem.set_u32(array + 0xC, records.len() as u32);
        for (i, record) in records.iter().enumerate() {
            e.mem.set_u32(array + 0x20 + 4 * i as u32, *record);
        }
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| {
            returns(a[0] + 0x20 + 4 * a[1])
        });
        let player = object_with_slots(&mut e, &[(0x68, 0x0330_0068), (0x6C, 0x0330_006C)]);
        e.set_global(PLAYER, player);
        let hooks: Hooks = Rc::new(RefCell::new(vec![]));
        for slot in [0x68u32, 0x6C] {
            let sink = hooks.clone();
            e.register_double(0x0330_0000 + slot, move |_, a| {
                sink.borrow_mut().push((slot, a[0], a[1], a[2]));
                Ret::default()
            });
        }
        quiet(
            &mut e,
            &[
                SET_LOADING_STATE,
                SET_LOAD_VERSION,
                END_FORM_PROCESSING,
                IO_MANAGER_SET_STATE_5,
                REF_GET_PARENT_CELL,
                REF_GET_WORLDSPACE,
                ACTOR_INIT_PACKAGE_LOCATIONS,
                LIST_REMOVE_ALL,
                LIST_DESTRUCT,
                FORM_SET_DISABLED,
                LOOKUP_FORM,
            ],
        );
        // The local list: its head node holds the first item.
        e.register(LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(LIST_NODE_IS_END, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        casts(&mut e, &[]);
        InitRig {
            e,
            game: game_object,
            array,
            player,
            hooks,
        }
    }

    #[test]
    fn after_load_runs_the_hooks_and_frees_the_records() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        let first: Ptr = Ptr::new(object_with_slots(
            e,
            &[(0x68, 0x0330_0068), (0x6C, 0x0330_006C)],
        ));
        let record_a = form_and_flags(e, first.addr(), 1, 2, 7);
        // The player's own record has no first hook and is still finished.
        let record_b = form_and_flags(e, rig.player, 3, 4, 8);
        e.mem.set_u32(rig.array + 0xC, 2);
        e.mem.set_u32(rig.array + 0x20, record_a);
        e.mem.set_u32(rig.array + 0x24, record_b);
        let array = Ptr::new(rig.array);
        fn_00858af0(e, rig.game, array, Ptr::NULL, false);
        let hooks = rig.hooks.borrow();
        assert_eq!(
            *hooks,
            vec![
                (0x68, first.addr(), 1, 2),
                (0x6C, first.addr(), 1, 2),
                (0x6C, rig.player, 3, 4),
            ]
        );
        assert_eq!(
            calls_to(e, SET_LOAD_VERSION),
            vec![
                vec![rig.game.addr(), 7],
                vec![rig.game.addr(), 7],
                vec![rig.game.addr(), 8]
            ]
        );
        assert!(freed(e, record_a));
        assert!(freed(e, record_b));
        // The loading state brackets the work.
        let states = calls_to(e, SET_LOADING_STATE);
        assert_eq!(states.first().unwrap()[1], 1);
        assert_eq!(states.last().unwrap()[1], 0);
    }

    #[test]
    fn after_load_places_or_disables_actors_that_are_in_no_space() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        let table = install_table(e, MAP_GET_AT, Some(CHANGES_MAP_SET_AT), MAP_REMOVE_AT);
        let actor: Ptr = Ptr::new(object_with_slots(
            e,
            &[(0x68, 0x0330_0068), (0x6C, 0x0330_006C)],
        ));
        e.mem.set_u32(actor.addr() + 0xC, 0xA11);
        let record = form_and_flags(e, actor.addr(), 1, 2, 7);
        e.mem.set_u32(rig.array + 0xC, 1);
        e.mem.set_u32(rig.array + 0x20, record);
        casts(e, &[(actor.addr(), RTTI_ACTOR, actor.addr())]);
        // No placement cell (the lookup of form id 0 is null): disabled.
        fn_00858af0(e, rig.game, Ptr::new(rig.array), Ptr::NULL, false);
        assert_eq!(
            calls_to(e, ACTOR_INIT_PACKAGE_LOCATIONS),
            vec![vec![actor.addr(), 0]]
        );
        assert_eq!(calls_to(e, FORM_SET_DISABLED), vec![vec![actor.addr(), 1]]);
        // The actor was flagged in the changes map (flags 2).
        let stored = table.borrow().clone();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].0, 0xA11);
        assert_eq!(e.mem.u32(stored[0].1), 2);
    }

    #[test]
    fn after_load_moves_an_actor_to_the_placement_cell() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        install_table(e, MAP_GET_AT, Some(CHANGES_MAP_SET_AT), MAP_REMOVE_AT);
        let actor: Ptr = Ptr::new(object_with_slots(
            e,
            &[(0x68, 0x0330_0068), (0x6C, 0x0330_006C)],
        ));
        let record = form_and_flags(e, actor.addr(), 1, 2, 7);
        e.mem.set_u32(rig.array + 0xC, 1);
        e.mem.set_u32(rig.array + 0x20, record);
        // The placement cell exists and fills the position and rotation.
        casts(
            e,
            &[
                (actor.addr(), RTTI_ACTOR, actor.addr()),
                (0x0777, RTTI_CELL, 0x0777),
            ],
        );
        constant(e, LOOKUP_FORM, 0x0777);
        e.mem.set_f32(DEFAULT_POSITION, 1.0);
        e.register(CELL_GET_PLACEMENT, |e, a| {
            e.mem.set_f32(a[1], 10.0);
            e.mem.set_f32(a[2], 20.0);
            Ret::default()
        });
        let seen: Rc<RefCell<Vec<f32>>> = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(REF_SET_POSITION, move |e, a| {
            sink.borrow_mut().push(e.mem.f32(a[1]));
            Ret::default()
        });
        quiet(e, &[FN_00575700, REF_MOVE_TO_SPACE]);
        fn_00858af0(e, rig.game, Ptr::new(rig.array), Ptr::NULL, false);
        assert!(calls_to(e, FORM_SET_DISABLED).is_empty());
        assert_eq!(*seen.borrow(), vec![10.0]);
        // The rotation vector's x is the second block's first float.
        assert_eq!(
            calls_to(e, FN_00575700),
            vec![vec![actor.addr(), 20.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(
            calls_to(e, REF_MOVE_TO_SPACE),
            vec![vec![actor.addr(), 0x0777, 0]]
        );
    }

    #[test]
    fn after_load_with_reload_refreshes_the_world_and_the_camera() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        let tes = e.mem.alloc(0x40);
        e.set_global(TES_OBJECT, tes);
        e.set_global(SAVE_LOCK, 0x0120_2d00u32);
        e.set_global(MODEL_LOADER, 0x0066_0000u32);
        // The player has no 3D: it is queued for loading.
        constant(e, PLAYER_GET_3D, 0);
        constant(e, READ_FIELD_34, 0x1111);
        constant(e, CELL_GET_PHYSICS_WORLD, 0x2222);
        constant(e, GET_EXTERIOR_WORLD, 0x3333);
        quiet(
            e,
            &[
                FN_00459920,
                MODEL_LOADER_QUEUE_REFERENCE,
                IO_MANAGER_LOAD_QUEUED_PRIORITY,
                WORLD_ADD_LOCK,
                WORLD_REMOVE_LOCK,
                FN_0057D0A0,
                SET_GLOBAL_FLAG,
            ],
        );
        // The player's position slot (0x1F4).
        let position = e.mem.alloc(16);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        constant(e, 0x0330_01F4, position);
        let player = rig.player;
        let vtable = e.mem.u32(player);
        e.mem.set_u32(vtable + 0x1F4, 0x0330_01F4);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(PLACEMENT_VECTOR + 4 * i as u32, *v);
        }
        fn_00858af0(e, rig.game, Ptr::NULL, Ptr::NULL, true);
        // Both worlds are locked and unlocked, twice (before and after the
        // queue is flushed).
        assert_eq!(
            calls_to(e, WORLD_ADD_LOCK),
            vec![vec![0x2222], vec![0x3333], vec![0x2222], vec![0x3333]]
        );
        assert_eq!(
            calls_to(e, WORLD_REMOVE_LOCK),
            vec![
                vec![0x2222, 0],
                vec![0x3333, 0],
                vec![0x2222, 0],
                vec![0x3333, 0]
            ]
        );
        assert_eq!(
            calls_to(e, MODEL_LOADER_QUEUE_REFERENCE),
            vec![vec![0x0066_0000, player, 0, 0]]
        );
        assert_eq!(calls_to(e, PLAYER_GET_3D), vec![vec![player, 0]]);
        // The camera: the player's position, the placement vector and 1.0,
        // between the flag being cleared and set.
        assert_eq!(
            calls_to(e, FN_0057D0A0),
            vec![vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(e, SET_GLOBAL_FLAG), vec![vec![0], vec![1]]);
        assert_eq!(calls_to(e, FN_00459920), vec![vec![tes]]);
    }

    #[test]
    fn after_load_loads_the_global_words_at_each_use() {
        // 00858af0 loads the I/O manager (01202d98), the player (011dea3c)
        // and `TES` (011dea10) at each use: what a form's first hook (slot
        // 0x68) leaves there is what the later steps are given.
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        e.set_global(TES_OBJECT, 0x0044_0000u32);
        e.set_global(SAVE_LOCK, 0x0120_2d00u32);
        e.set_global(MODEL_LOADER, 0x0066_0000u32);
        constant(e, PLAYER_GET_3D, 0);
        constant(e, READ_FIELD_34, 0);
        constant(e, GET_EXTERIOR_WORLD, 0);
        quiet(
            e,
            &[
                FN_00459920,
                MODEL_LOADER_QUEUE_REFERENCE,
                IO_MANAGER_LOAD_QUEUED_PRIORITY,
                WORLD_ADD_LOCK,
                WORLD_REMOVE_LOCK,
                FN_0057D0A0,
                SET_GLOBAL_FLAG,
            ],
        );
        let tes = e.mem.alloc(0x40);
        let player = object_with_slots(
            e,
            &[
                (0x68, 0x0330_0068),
                (0x6C, 0x0330_006C),
                (0x1F4, 0x0330_01F4),
            ],
        );
        let position = e.mem.alloc(16);
        constant(e, 0x0330_01F4, position);
        let form = object_with_slots(e, &[(0x68, 0x0350_0068), (0x6C, 0x0330_006C)]);
        e.register_double(0x0350_0068, move |e, _| {
            e.set_global(SAVE_LOCK, 0x0120_2e00u32);
            e.set_global(PLAYER, player);
            e.set_global(TES_OBJECT, tes);
            Ret::default()
        });
        let record = form_and_flags(e, form, 1, 2, 7);
        e.mem.set_u32(rig.array + 0xC, 1);
        e.mem.set_u32(rig.array + 0x20, record);
        fn_00858af0(e, rig.game, Ptr::new(rig.array), Ptr::NULL, true);
        assert_eq!(
            calls_to(e, IO_MANAGER_SET_STATE_5),
            vec![vec![0x0120_2d00], vec![0x0120_2e00]]
        );
        assert_eq!(
            calls_to(e, IO_MANAGER_LOAD_QUEUED_PRIORITY),
            vec![vec![0x0120_2e00], vec![0x0120_2e00]]
        );
        assert_eq!(calls_to(e, READ_FIELD_34), vec![vec![tes], vec![tes]]);
        assert_eq!(calls_to(e, FN_00459920), vec![vec![tes]]);
        assert_eq!(calls_to(e, PLAYER_GET_3D), vec![vec![player, 0]]);
        assert_eq!(
            calls_to(e, MODEL_LOADER_QUEUE_REFERENCE),
            vec![vec![0x0066_0000, player, 0, 0]]
        );
        assert_eq!(calls_to(e, 0x0330_01F4), vec![vec![player]]);
    }

    #[test]
    fn after_load_destroys_the_games_init_array() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        let array = object_with_slots(e, &[(0, 0x0340_0000)]);
        e.mem.set_u32(array + 0xC, 0);
        stub(e, 0x0340_0000);
        stub(e, INIT_ARRAY_CLEANUP);
        e.set(rig.game, TESSaveLoadGame::m_pInitArray, Ptr::new(array));
        fn_00858af0(e, rig.game, Ptr::NULL, Ptr::NULL, false);
        assert_eq!(calls_to(e, INIT_ARRAY_CLEANUP), vec![vec![array]]);
        assert_eq!(calls_to(e, 0x0340_0000), vec![vec![array, 1]]);
        assert!(e.get(rig.game, TESSaveLoadGame::m_pInitArray).is_null());
    }

    #[test]
    fn after_load_does_nothing_unless_the_measuring_flag_is_set() {
        let mut rig = init_rig(&[]);
        let e = &mut rig.e;
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(0));
        fn_00858af0(e, rig.game, Ptr::new(rig.array), Ptr::NULL, true);
        assert!(calls_to(e, SET_LOADING_STATE).is_empty());
    }

    #[test]
    fn a_known_reference_loses_bit_one() {
        let mut e = game();
        map_globals(&mut e);
        e.set_global(DATA_HANDLER, 0x0055_0000u32);
        let form = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(form.addr() + 0xC, 0x1234);
        casts(&mut e, &[(form.addr(), RTTI_REFERENCE, form.addr())]);
        let known: Rc<RefCell<bool>> = Rc::new(RefCell::new(true));
        let state = known.clone();
        e.register_double(DATA_HANDLER_HAS_FORM, move |_, a| {
            assert_eq!(a[1], 0x1234);
            returns(*state.borrow() as u32)
        });
        assert_eq!(
            tes_save_load_game_check_new_reference(&mut e, Ptr::NULL, form, 0xFF),
            0xFD
        );
        // A form the handler does not know keeps its flags, and so does one
        // that is not a reference.
        *known.borrow_mut() = false;
        assert_eq!(
            tes_save_load_game_check_new_reference(&mut e, Ptr::NULL, form, 0xFF),
            0xFF
        );
        *known.borrow_mut() = true;
        casts(&mut e, &[]);
        assert_eq!(
            tes_save_load_game_check_new_reference(&mut e, Ptr::NULL, form, 0xFF),
            0xFF
        );
    }

    /// The flag-checking world: a form that casts to a reference (and an
    /// actor), a game and the doubles every path needs.
    struct FlagsRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        form: Ptr,
    }

    fn flags_rig() -> FlagsRig {
        let mut e = game();
        map_globals(&mut e);
        e.set_global(DATA_HANDLER, 0x0055_0000u32);
        constant(&mut e, DATA_HANDLER_HAS_FORM, 0);
        let player = e.mem.alloc(0x40);
        e.set_global(PLAYER, player);
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let form = Ptr::new(e.mem.alloc(0x40));
        quiet(
            &mut e,
            &[
                EXTRA_GET_SEEN,
                CELL_IS_INTERIOR,
                CELL_GET_X,
                CELL_GET_Y,
                REF_GET_EXTRA_LIST,
                EXTRA_GET_CONTAINER_CHANGES,
                ACTOR_GET_PROCESS,
                ACTOR_PACKAGE_FLAGS,
                ACTOR_TEST_FLAGS,
                REF_PERSISTS,
                REF_GET_PARENT_CELL,
                REF_GET_WORLDSPACE,
                WORLDSPACE_GET_CELL,
                FLOAT_TO_INT,
                LOG_ERROR,
            ],
        );
        casts(&mut e, &[]);
        FlagsRig {
            e,
            game: game_object,
            form,
        }
    }

    #[test]
    fn check_flags_clears_the_top_bit_of_a_cell_without_seen_data() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        casts(e, &[(rig.form.addr(), RTTI_CELL, rig.form.addr())]);
        let flags = tes_save_load_game_check_flags(e, rig.game, rig.form, 0x8000_0003);
        assert_eq!(flags, 3);
        // An exterior cell also has its coordinates read.
        assert_eq!(calls_to(e, CELL_GET_X).len(), 1);
        assert_eq!(calls_to(e, CELL_GET_Y).len(), 1);
        // With seen data the bit stays.
        constant(e, EXTRA_GET_SEEN, 1);
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0x8000_0003),
            0x8000_0003
        );
    }

    #[test]
    fn check_flags_leaves_other_forms_alone() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0x1234),
            0x1234
        );
    }

    #[test]
    fn check_flags_drops_bit_five_without_container_changes() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        let reference = rig.form.addr();
        casts(e, &[(reference, RTTI_REFERENCE, reference)]);
        // No extra data list at all.
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0x21),
            0x01
        );
        // A list without container changes.
        constant(e, REF_GET_EXTRA_LIST, 0x7000);
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0x21),
            0x01
        );
        // With container changes bit 5 stays.
        constant(e, EXTRA_GET_CONTAINER_CHANGES, 1);
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0x21),
            0x21
        );
    }

    #[test]
    fn check_flags_takes_an_actors_package_flags() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        let reference = rig.form.addr();
        // The actor is a reference with a process; its slots 0x20C and 0x22C
        // are on the process object, 0x22C and 0x100 also on the actor.
        let process = object_with_slots(e, &[(0x20C, 0x0350_020C), (0x22C, 0x0350_022C)]);
        stub(e, 0x0350_020C);
        constant(e, 0x0350_022C, 0x77);
        constant(e, ACTOR_GET_PROCESS, process);
        constant(e, ACTOR_PACKAGE_FLAGS, 0x100);
        let actor = object_with_slots(e, &[(0x22C, 0x0350_0230)]);
        constant(e, 0x0350_0230, 0);
        casts(
            e,
            &[
                (reference, RTTI_REFERENCE, reference),
                (reference, RTTI_ACTOR, actor),
            ],
        );
        // The test says no and the actor's slot says no: bit 2 is cleared.
        let flags = tes_save_load_game_check_flags(e, rig.game, rig.form, 0x4);
        assert_eq!(flags, 0x100);
        // The package flags come from the package the process runs.
        assert_eq!(calls_to(e, ACTOR_PACKAGE_FLAGS), vec![vec![actor, 0x77]]);
        assert_eq!(calls_to(e, 0x0350_020C), vec![vec![process]]);
        // The test saying yes sets bit 2.
        constant(e, ACTOR_TEST_FLAGS, 1);
        // (A persistent reference is not looked up in a cell.)
        constant(e, REF_PERSISTS, 1);
        assert_eq!(
            tes_save_load_game_check_flags(e, rig.game, rig.form, 0),
            0x104
        );
    }

    #[test]
    fn check_flags_looks_up_the_cell_of_a_located_actor() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        let reference = rig.form.addr();
        // An actor-like reference (slot 0x100) with a saved world space
        // (0x294) and no cell (0x298); its location (slot 0x170) is x = 4096,
        // y = 8192.
        let object = object_with_slots(
            e,
            &[
                (0x100, 0x0360_0100),
                (0x294, 0x0360_0294),
                (0x298, 0x0360_0298),
                (0x170, 0x0360_0170),
            ],
        );
        constant(e, 0x0360_0100, 1);
        constant(e, 0x0360_0294, 0x5050);
        constant(e, 0x0360_0298, 0);
        e.register(0x0360_0170, |e, a| {
            e.mem.set_f32(a[1], 4096.0);
            e.mem.set_f32(a[1] + 4, 8192.0);
            returns(a[1])
        });
        casts(e, &[(reference, RTTI_REFERENCE, object)]);
        constant(e, REF_GET_WORLDSPACE, 0x5050);
        e.register(FLOAT_TO_INT, |_, a| {
            returns(f32::from_bits(a[0]) as i32 as u32)
        });
        // The flags have bit 1: the cell at (4096, 8192) >> 12 is looked up
        // in the reference's own world space.
        let flags = tes_save_load_game_check_flags(e, rig.game, rig.form, 0x2);
        assert_eq!(flags, 0x2);
        assert_eq!(calls_to(e, WORLDSPACE_GET_CELL), vec![vec![0x5050, 1, 2]]);
    }

    #[test]
    fn check_flags_complains_about_an_actor_without_an_editor_location() {
        let mut rig = flags_rig();
        let e = &mut rig.e;
        let reference = rig.form.addr();
        let object = object_with_slots(
            e,
            &[
                (0x100, 0x0360_0100),
                (0x294, 0x0360_0294),
                (0x298, 0x0360_0298),
                (0x170, 0x0360_0170),
            ],
        );
        constant(e, 0x0360_0100, 1);
        constant(e, 0x0360_0294, 0);
        constant(e, 0x0360_0298, 0);
        e.register(0x0360_0170, |e, a| {
            e.mem.set_f32(a[1], 4096.0);
            e.mem.set_f32(a[1] + 4, 4096.0);
            returns(a[1])
        });
        casts(e, &[(reference, RTTI_REFERENCE, object)]);
        // No cell, no world space; a parent cell that is not an interior.
        constant(e, REF_GET_PARENT_CELL, 0x6060);
        constant(e, CELL_IS_INTERIOR, 0);
        constant(e, REF_GET_WORLDSPACE, 0x5050);
        e.register(FLOAT_TO_INT, |_, a| {
            returns(f32::from_bits(a[0]) as i32 as u32)
        });
        tes_save_load_game_check_flags(e, rig.game, rig.form, 0x4);
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![vec![MSG_ACTOR_NO_EDITOR_LOCATION]]
        );
        assert_eq!(calls_to(e, WORLDSPACE_GET_CELL), vec![vec![0x5050, 1, 1]]);
    }

    #[test]
    fn the_running_package_is_the_process_slot_22c() {
        let mut e = game();
        let process = object_with_slots(&mut e, &[(0x22C, 0x0370_0000)]);
        constant(&mut e, 0x0370_0000, 0x4242);
        assert_eq!(
            base_process_get_package_that_is_running(&mut e, Ptr::new(process)),
            0x4242
        );
    }

    #[test]
    fn a_change_data_entry_without_a_required_flag_is_logged() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        quiet(&mut e, &[LOG_ERROR]);
        fn_008598d0(&mut e, game_object, 0x55);
        assert!(calls_to(&e, LOG_ERROR).is_empty());
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((0x55, data.addr()));
        fn_008598d0(&mut e, game_object, 0x55);
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_CELL_REFERENCE_NO_FLAG]]
        );
    }

    /// The new-references world: a game with both cell maps, a cell and the
    /// list helpers that act on memory.
    struct CellRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        cell: Ptr,
        interior_map: u32,
        exterior_map: u32,
        table: Table,
    }

    fn cell_rig() -> CellRig {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        let interior_map = e.mem.alloc(0x10);
        let exterior_map = e.mem.alloc(0x10);
        e.set(
            game_object,
            TESSaveLoadGame::m_pInteriorCellMap,
            Ptr::new(interior_map),
        );
        e.set(
            game_object,
            TESSaveLoadGame::m_pExteriorCellMap,
            Ptr::new(exterior_map),
        );
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let cell = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(cell.addr() + 0xC, 0xCE11);
        quiet(&mut e, &[LOG_ERROR, LIST_REMOVE_ALL, LIST_SCALAR_DELETE]);
        measuring_for(&mut e, 1);
        // The cell is not loaded itself here.
        CellRig {
            e,
            game: game_object,
            cell,
            interior_map,
            exterior_map,
            table,
        }
    }

    #[test]
    fn an_interior_cell_loads_its_listed_references_and_forgets_the_list() {
        let mut rig = cell_rig();
        let e = &mut rig.e;
        constant(e, CELL_IS_INTERIOR, 1);
        let list = list_of(e, &[0x111, 0, 0x222]);
        rig.table.borrow_mut().push((0xCE11, list));
        assert!(fn_00859690(e, rig.game, rig.cell));
        // Each non-zero id was handled (its change data looked up).
        let lookups: Vec<u32> = calls_to(e, MAP_GET_AT)
            .iter()
            .filter(|c| c[0] != rig.interior_map)
            .map(|c| c[1])
            .collect();
        assert_eq!(lookups, vec![0x111, 0x222]);
        assert_eq!(
            calls_to(e, MAP_REMOVE_AT),
            vec![vec![rig.interior_map, 0xCE11]]
        );
        assert_eq!(calls_to(e, LIST_REMOVE_ALL), vec![vec![list]]);
        assert_eq!(calls_to(e, LIST_SCALAR_DELETE), vec![vec![list, 1]]);
    }

    #[test]
    fn an_interior_cell_without_a_list_loads_nothing() {
        let mut rig = cell_rig();
        let e = &mut rig.e;
        constant(e, CELL_IS_INTERIOR, 1);
        assert!(!fn_00859690(e, rig.game, rig.cell));
        assert!(calls_to(e, MAP_REMOVE_AT).is_empty());
    }

    #[test]
    fn new_references_do_nothing_unless_the_measuring_flag_is_set() {
        let mut rig = cell_rig();
        let e = &mut rig.e;
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(0));
        assert!(!fn_00859690(e, rig.game, rig.cell));
        assert!(calls_to(e, MAP_GET_AT).is_empty());
    }

    /// An `ExteriorCellReferenceData`.
    fn exterior_item(e: &mut Engine, id: u32, x: i32, y: i32) -> u32 {
        let item = e.mem.alloc(12);
        e.mem.set_u32(item, id);
        e.mem.set_i32(item + 4, x);
        e.mem.set_i32(item + 8, y);
        item
    }

    #[test]
    fn an_exterior_cell_takes_only_the_references_at_its_coordinates() {
        let mut rig = cell_rig();
        let e = &mut rig.e;
        constant(e, CELL_IS_INTERIOR, 0);
        let worldspace = e.mem.alloc(0x40);
        e.mem.set_u32(worldspace + 0xC, 0x7777);
        constant(e, CELL_GET_WORLDSPACE, worldspace);
        constant(e, CELL_GET_X, 3);
        constant(e, CELL_GET_Y, 4);
        let (first, other, last) = (
            exterior_item(e, 0xA1, 3, 4),
            exterior_item(e, 0xB2, 9, 9),
            exterior_item(e, 0xC3, 3, 4),
        );
        let list = list_of(e, &[first, other, last]);
        rig.table.borrow_mut().push((0x7777, list));
        // `RemoveHead` copies the next node into the head; `Remove` unlinks
        // the node holding the item after the given one.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                let next = e.mem.u32(node + 4);
                if next != 0 && e.mem.u32(next) == item {
                    let after = e.mem.u32(next + 4);
                    e.mem.set_u32(node + 4, after);
                }
                node = next;
            }
            Ret::default()
        });
        e.register(LIST_NODE_IS_END, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        assert!(fn_00859690(e, rig.game, rig.cell));
        // The two matching references were handled; the other stays, so the
        // world space's entry stays too.
        let handled: Vec<u32> = calls_to(e, MAP_GET_AT)
            .iter()
            .filter(|c| c[0] != rig.exterior_map)
            .map(|c| c[1])
            .collect();
        assert_eq!(handled, vec![0xA1, 0xC3]);
        assert!(freed(e, first));
        assert!(freed(e, last));
        assert!(!freed(e, other));
        assert!(calls_to(e, MAP_REMOVE_AT).is_empty());
    }

    #[test]
    fn an_exterior_cell_with_nothing_left_removes_the_world_space_entry() {
        let mut rig = cell_rig();
        let e = &mut rig.e;
        constant(e, CELL_IS_INTERIOR, 0);
        let worldspace = e.mem.alloc(0x40);
        e.mem.set_u32(worldspace + 0xC, 0x7777);
        constant(e, CELL_GET_WORLDSPACE, worldspace);
        constant(e, CELL_GET_X, 3);
        constant(e, CELL_GET_Y, 4);
        let only = exterior_item(e, 0xA1, 3, 4);
        let list = list_of(e, &[only]);
        rig.table.borrow_mut().push((0x7777, list));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(LIST_NODE_IS_END, |e, a| {
            returns((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        assert!(fn_00859690(e, rig.game, rig.cell));
        assert_eq!(
            calls_to(e, MAP_REMOVE_AT),
            vec![vec![rig.exterior_map, 0x7777]]
        );
        assert_eq!(calls_to(e, LIST_SCALAR_DELETE), vec![vec![list, 1]]);
    }

    /// The world of `fn_00859a90`: forms are found in `forms`, the types
    /// of base objects in `types`, and each constructor makes an object
    /// whose slot `0x128` is recorded in `ids`.
    struct CreateRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        /// The objects the constructors made: (constructor address, block).
        made: Rc<RefCell<Vec<(u32, u32, u32)>>>,
        /// The slot `0x128` calls: (object, id, flag).
        ids: Rc<RefCell<Vec<(u32, u32, u32)>>>,
    }

    fn create_rig(forms: &[(u32, u32)]) -> CreateRig {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let forms = forms.to_vec();
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(forms.iter().find(|f| f.0 == a[0]).map_or(0, |f| f.1))
        });
        casts(&mut e, &[]);
        quiet(&mut e, &[LOG_ERROR, REF_SET_BASE, FORM_FINISH, FN_00483C70]);
        let ids: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let sink = ids.clone();
        e.register_double(0x0380_0128, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        let made: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + 0x128, 0x0380_0128);
        for constructor in [
            CHARACTER_CONSTRUCT,
            CREATURE_CONSTRUCT,
            REFERENCE_CONSTRUCT,
            ARROW_PROJECTILE_CONSTRUCT,
            MAGIC_PROJECTILE_CONSTRUCT_A,
            MAGIC_PROJECTILE_CONSTRUCT_B,
            MAGIC_PROJECTILE_CONSTRUCT_C,
        ] {
            let sink = made.clone();
            e.register_double(constructor, move |e, a| {
                let size = e.mem.block_size(a[0]).unwrap();
                sink.borrow_mut().push((constructor, a[0], size));
                e.mem.set_u32(a[0], vtable);
                returns(a[0])
            });
        }
        CreateRig {
            e,
            game: game_object,
            made,
            ids,
        }
    }

    /// A `CreatedReferenceData` of the given type and bound id with the
    /// location 0x77.
    fn created_reference(e: &mut Engine, kind: u32, bound_id: u32) -> Ptr<CreatedReferenceData> {
        let record: Ptr<CreatedReferenceData> = e.new_object();
        e.set(record, CreatedReferenceData::eType, kind);
        e.set(record, CreatedReferenceData::iBoundID, bound_id);
        e.mem.set_u32(record.addr() + 8, 0x77);
        record
    }

    #[test]
    fn a_created_reference_whose_bound_object_is_gone_is_not_made() {
        let mut rig = create_rig(&[]);
        let e = &mut rig.e;
        let record = created_reference(e, 0, 0x2222);
        let form = fn_00859a90(e, rig.game, 0x1111, record);
        assert!(form.is_null());
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![vec![MSG_BOUND_OBJECT_MISSING, 0x2222, 0x1111]]
        );
        assert_eq!(calls_to(e, SCOPE_LEAVE).len(), 1);
        assert!(rig.made.borrow().is_empty());
    }

    #[test]
    fn a_normal_created_reference_is_built_by_the_base_objects_type() {
        for (base_type, constructor, size) in [
            (0x2Au32, CHARACTER_CONSTRUCT, 0x1C8u32),
            (0x2B, CREATURE_CONSTRUCT, 0x1C0),
            (0x33, REFERENCE_CONSTRUCT, 0x68),
        ] {
            let mut rig = create_rig(&[(0x2222, 0xB0B0)]);
            let e = &mut rig.e;
            casts(e, &[(0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0)]);
            constant(e, FORM_TYPE, base_type);
            let record = created_reference(e, 0, 0x2222);
            let form = fn_00859a90(e, rig.game, 0x1111, record);
            let made = rig.made.borrow().clone();
            assert_eq!(made.len(), 1);
            assert_eq!(made[0].0, constructor);
            assert_eq!(calls_to(e, OPERATOR_NEW), vec![vec![size]]);
            assert_eq!(form.addr(), made[0].1);
            // It gets the form id (with 1) and is finished.
            assert_eq!(*rig.ids.borrow(), vec![(form.addr(), 0x1111, 1)]);
            assert_eq!(calls_to(e, FORM_FINISH), vec![vec![form.addr(), 1]]);
            // The reference (a cast of the form; null here) gets the base.
            assert_eq!(calls_to(e, REF_SET_BASE), vec![vec![0, 0xB0B0]]);
        }
    }

    #[test]
    fn an_existing_reference_with_the_same_base_object_is_kept() {
        let mut rig = create_rig(&[(0x1111, 0xF0F0), (0x2222, 0xB0B0)]);
        let e = &mut rig.e;
        casts(
            e,
            &[
                (0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0),
                (0xF0F0, RTTI_REFERENCE, 0xF0F0),
            ],
        );
        constant(e, REFERENCE_GET_BASE, 0xB0B0);
        let record = created_reference(e, 3, 0x2222);
        let form = fn_00859a90(e, rig.game, 0x1111, record);
        assert_eq!(form.addr(), 0xF0F0);
        assert!(rig.made.borrow().is_empty());
        assert_eq!(calls_to(e, FORM_FINISH), vec![vec![0xF0F0, 1]]);
        assert!(calls_to(e, LOG_ERROR).is_empty());
    }

    #[test]
    fn an_existing_reference_with_another_base_object_is_deleted_and_made_again() {
        let old = 0xF0F0;
        let mut rig = create_rig(&[(0x1111, old), (0x2222, 0xB0B0)]);
        let e = &mut rig.e;
        casts(
            e,
            &[
                (0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0),
                (old, RTTI_REFERENCE, old),
            ],
        );
        constant(e, REFERENCE_GET_BASE, 0xC1C1);
        constant(e, FORM_TYPE, 0x33);
        // `DeleteForm` sets the old form's id: it needs an object.
        let old_form = object_with_slots(e, &[(0x128, 0x0380_0128), (0x134, 0x0380_0134)]);
        let forms = [(0x1111u32, old_form), (0x2222, 0xB0B0)];
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(forms.iter().find(|f| f.0 == a[0]).map_or(0, |f| f.1))
        });
        casts(
            e,
            &[
                (0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0),
                (old_form, RTTI_REFERENCE, old_form),
            ],
        );
        stub(e, 0x0380_0134);
        stub(e, LIST_CONTAINS);
        stub(e, LIST_ADD_HEAD);
        constant(e, FORM_IS_DELETED, 0);
        let record = created_reference(e, 0, 0x2222);
        let form = fn_00859a90(e, rig.game, 0x1111, record);
        // The old form was deleted: it is given id 0 and a new form is made.
        assert_eq!(rig.ids.borrow()[0], (old_form, 0, 1));
        assert_eq!(rig.made.borrow().len(), 1);
        assert_eq!(form.addr(), rig.made.borrow()[0].1);
    }

    #[test]
    fn projectile_records_pick_their_constructors() {
        for (kind, bound_id, constructor, size) in [
            (1u32, 0x2222u32, ARROW_PROJECTILE_CONSTRUCT, 0xC8u32),
            (2, 0, MAGIC_PROJECTILE_CONSTRUCT_A, 0xC4),
            (2, 3, MAGIC_PROJECTILE_CONSTRUCT_B, 0xD0),
            (2, 1, MAGIC_PROJECTILE_CONSTRUCT_C, 0xD8),
        ] {
            let mut rig = create_rig(&[(0x2222, 0xB0B0)]);
            let e = &mut rig.e;
            casts(e, &[(0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0)]);
            let record = created_reference(e, kind, bound_id);
            let form = fn_00859a90(e, rig.game, 0x1111, record);
            let made = rig.made.borrow().clone();
            assert_eq!(made.len(), 1, "kind {kind} bound {bound_id}");
            assert_eq!(made[0].0, constructor);
            assert_eq!(calls_to(e, OPERATOR_NEW), vec![vec![size]]);
            assert_eq!(form.addr(), made[0].1);
        }
    }

    #[test]
    fn an_unknown_created_reference_type_is_logged() {
        let mut rig = create_rig(&[(0x2222, 0xB0B0)]);
        let e = &mut rig.e;
        casts(e, &[(0xB0B0, RTTI_BOUND_OBJECT, 0xB0B0)]);
        let record = created_reference(e, 7, 0x2222);
        let form = fn_00859a90(e, rig.game, 0x1111, record);
        assert!(form.is_null());
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![vec![MSG_INVALID_CREATED_TYPE, 7, 0x1111, 0x2222, 0x77]]
        );
        assert!(calls_to(e, FORM_FINISH).is_empty());
    }

    /// The world of `fn_00859f20`: a location (a cell or a world space)
    /// with two plugin files, of which the second has the reference.
    struct MovedRig {
        e: Engine,
        moved: Ptr<MovedReferenceData>,
        reference: u32,
        loaded: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn moved_rig() -> MovedRig {
        let mut e = game();
        // The location form 0x5000 is looked up by the record's location.
        let moved: Ptr<MovedReferenceData> = e.new_object();
        e.set(moved, MovedReferenceData::iOriginalLocationID, 0x5000);
        e.mem.set_f32(moved.addr() + 4, 8192.0);
        e.mem.set_f32(moved.addr() + 8, 4096.0);
        // The reference the file makes: slots 0x88 (post create), 0x1F4
        // position.
        let position = e.mem.alloc(16);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        let reference = object_with_slots(&mut e, &[(0x88, 0x0390_0088), (0x1F4, 0x0390_01F4)]);
        stub(&mut e, 0x0390_0088);
        constant(&mut e, 0x0390_01F4, position);
        let rotation = e.mem.alloc(16);
        for (i, v) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(rotation + 4 * i as u32, *v);
        }
        constant(&mut e, REF_GET_ROTATION, rotation);
        constant(&mut e, REF_GET_EXTRA_LIST, 0x7000);
        quiet(
            &mut e,
            &[
                LOG_ERROR,
                EXTRA_SET_STARTING_POSITION,
                EXTRA_SET_STARTING_ROTATION,
                ACTOR_PLACE,
            ],
        );
        // Entry i of the cell or world space is the file i + 1 times 10.
        e.register(FILE_OF_CELL, |_, a| returns(a[1] + 1));
        e.register(THREAD_SAFE_FILE, |_, a| returns(a[0] * 10));
        e.register(FILE_HAS_FORM, |_, a| returns((a[0] == 20) as u32));
        constant(&mut e, FILE_RECORD_TYPE, 0x33);
        constant(&mut e, CREATE_REFERENCE, reference);
        let loaded: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let sink = loaded.clone();
        e.register_double(LOAD_FORM_FROM_FILE, move |_, a| {
            sink.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        constant(&mut e, CELL_FILE_COUNT, 2);
        e.register(FLOAT_TO_INT, |_, a| {
            returns(f32::from_bits(a[0]) as i32 as u32)
        });
        MovedRig {
            e,
            moved,
            reference,
            loaded,
        }
    }

    #[test]
    fn a_moved_reference_is_loaded_from_the_cells_plugin_that_has_it() {
        let mut rig = moved_rig();
        let e = &mut rig.e;
        constant(e, LOOKUP_FORM, 0x5000);
        casts(e, &[(0x5000, RTTI_CELL, 0x5000)]);
        // Both files have the cell; only file 20 has the reference.
        constant(e, FILE_HAS_CELL, 1);
        let result = fn_00859f20(e, Ptr::NULL, 0x1234, rig.moved);
        assert_eq!(result.addr(), rig.reference);
        assert_eq!(*rig.loaded.borrow(), vec![(rig.reference, 20)]);
        assert_eq!(calls_to(e, CREATE_REFERENCE), vec![vec![0x33, 1]]);
        // Not an actor: the starting position and rotation are set on the
        // extra data from the reference's own.
        let position = calls_to(e, EXTRA_SET_STARTING_POSITION);
        assert_eq!(position.len(), 1);
        assert_eq!(position[0][0], 0x7000);
        assert_eq!(position[0][2], rig.reference);
        assert_eq!(
            position[0][3..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        let rotation = calls_to(e, EXTRA_SET_STARTING_ROTATION);
        assert_eq!(
            rotation[0][3..],
            [4.0f32.to_bits(), 5.0f32.to_bits(), 6.0f32.to_bits()]
        );
        assert!(calls_to(e, ACTOR_PLACE).is_empty());
    }

    #[test]
    fn a_moved_actor_is_placed_in_the_world_space_cell() {
        let mut rig = moved_rig();
        let e = &mut rig.e;
        constant(e, LOOKUP_FORM, 0x5000);
        let reference = rig.reference;
        casts(
            e,
            &[
                (0x5000, RTTI_WORLDSPACE, 0x5000),
                (reference, RTTI_ACTOR, 0xAC70),
            ],
        );
        // The cell (8192 / 4096 grid units: 2, 1) is in file 20 only.
        e.register(WORLDSPACE_FIND_CELL_IN_FILE, |_, a| {
            returns((a[1] == 20 && a[2] == 2 && a[3] == 1) as u32)
        });
        let result = fn_00859f20(e, Ptr::NULL, 0x1234, rig.moved);
        assert_eq!(result.addr(), reference);
        // The actor is placed with the world space, no cell, the position and
        // the z of the rotation.
        let placed = calls_to(e, ACTOR_PLACE);
        assert_eq!(placed.len(), 1);
        assert_eq!(placed[0][..3], [0xAC70, 0x5000, 0]);
        assert_eq!(placed[0][4], 6.0f32.to_bits());
        assert!(calls_to(e, EXTRA_SET_STARTING_POSITION).is_empty());
    }

    #[test]
    fn a_moved_reference_with_no_location_or_no_plugin_is_logged() {
        let mut rig = moved_rig();
        let e = &mut rig.e;
        // The location form is neither a cell nor a world space.
        constant(e, LOOKUP_FORM, 0);
        let result = fn_00859f20(e, Ptr::NULL, 0x1234, rig.moved);
        assert!(result.is_null());
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![
                vec![MSG_NO_CELL_OR_WORLDSPACE],
                vec![MSG_REFERENCE_NOT_LOADED, 0x1234, 0x5000, 0, 0]
            ]
        );
        // A cell no plugin has the reference for.
        constant(e, LOOKUP_FORM, 0x5000);
        casts(e, &[(0x5000, RTTI_CELL, 0x5000)]);
        constant(e, FILE_HAS_CELL, 0);
        assert!(fn_00859f20(e, Ptr::NULL, 0x1234, rig.moved).is_null());
        assert!(rig.loaded.borrow().is_empty());
    }

    #[test]
    fn a_moved_record_without_an_original_location_uses_the_reference_data() {
        let mut rig = moved_rig();
        let e = &mut rig.e;
        e.set(rig.moved, MovedReferenceData::iOriginalLocationID, 0);
        e.mem.set_u32(rig.moved.addr() + 0x10, 0x6000);
        let looked_up = Rc::new(RefCell::new(vec![]));
        let sink = looked_up.clone();
        e.register_double(LOOKUP_FORM, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        fn_00859f20(e, Ptr::NULL, 0x1234, rig.moved);
        assert_eq!(*looked_up.borrow(), vec![0x6000]);
    }

    #[test]
    fn making_a_form_removes_what_is_saved_under_its_id() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let data = change_data(&mut e, 1, 0);
        table.borrow_mut().push((0x1234, data.addr()));
        constant(&mut e, LOOKUP_FORM, 0);
        let form = object_with_slots(&mut e, &[(0x128, 0x03A0_0128)]);
        constant(&mut e, CREATE_FORM_OF_TYPE, form);
        stub(&mut e, 0x03A0_0128);
        let result = fn_0085a240(&mut e, game_object, 0x1234, 0x2A);
        assert_eq!(result.addr(), form);
        // The type byte is passed, the id (with 1) is assigned, and the
        // change data of the id was dropped (force 1).
        assert_eq!(calls_to(&e, CREATE_FORM_OF_TYPE), vec![vec![0x2A]]);
        assert_eq!(calls_to(&e, 0x03A0_0128), vec![vec![form, 0x1234, 1]]);
        assert!(table.borrow().is_empty());
    }

    #[test]
    fn removing_an_id_deletes_an_existing_form_instead() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        let table = install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let form = object_with_slots(&mut e, &[(0x128, 0x03A0_0128), (0x134, 0x03A0_0134)]);
        e.mem.set_u32(form + 0xC, 0x1234);
        constant(&mut e, LOOKUP_FORM, form);
        stub(&mut e, 0x03A0_0128);
        stub(&mut e, 0x03A0_0134);
        quiet(
            &mut e,
            &[LOG_ERROR, FN_00483C70, LIST_CONTAINS, LIST_ADD_HEAD],
        );
        casts(&mut e, &[]);
        constant(&mut e, FORM_IS_DELETED, 0);
        fn_0085a290(&mut e, game_object, 0x1234);
        // The form was reset: id 0 and an empty editor id.
        assert_eq!(calls_to(&e, 0x03A0_0128), vec![vec![form, 0, 1]]);
        assert_eq!(calls_to(&e, 0x03A0_0134), vec![vec![form, EMPTY_STRING]]);
        assert!(table.borrow().is_empty());
    }

    #[test]
    fn deleting_a_form_resets_it_and_defers_its_deletion() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let form = object_with_slots(&mut e, &[(0x128, 0x03A0_0128), (0x134, 0x03A0_0134)]);
        e.mem.set_u32(form + 0xC, 0x1234);
        stub(&mut e, 0x03A0_0128);
        stub(&mut e, 0x03A0_0134);
        quiet(
            &mut e,
            &[LOG_ERROR, FN_00483C70, LIST_CONTAINS, LIST_ADD_HEAD],
        );
        casts(&mut e, &[]);
        constant(&mut e, FORM_IS_DELETED, 0);
        tes_save_load_game_delete_form(&mut e, game_object, Ptr::new(form));
        // Not loading: the error is logged.
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_DELETE_FORM_NOT_LOADING]]
        );
        assert_eq!(calls_to(&e, 0x03A0_0128), vec![vec![form, 0, 1]]);
        assert_eq!(calls_to(&e, FN_00483C70), vec![vec![form]]);
        assert_eq!(calls_to(&e, 0x03A0_0134), vec![vec![form, EMPTY_STRING]]);
        // It was put on the deferred list (at +0x34), not found there before.
        assert_eq!(calls_to(&e, LIST_CONTAINS)[0][0], game_object.addr() + 0x34);
        let added = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], game_object.addr() + 0x34);
    }

    #[test]
    fn deleting_an_already_deleted_form_or_a_cell_gives_it_a_new_id() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let changes: Ptr<ChangesMap> = e.new_object();
        e.set(game_object, TESSaveLoadGame::m_pChanges, changes);
        install_table(&mut e, MAP_GET_AT, None, MAP_REMOVE_AT);
        let form = object_with_slots(&mut e, &[(0x128, 0x03A0_0128)]);
        e.mem.set_u32(form + 0xC, 0x1234);
        stub(&mut e, 0x03A0_0128);
        quiet(&mut e, &[LOG_ERROR]);
        map_globals(&mut e);
        e.set_global(DATA_HANDLER, 0x0055_0000u32);
        constant(&mut e, DATA_HANDLER_GET_NEXT_ID, 0xFF00_0001);
        casts(&mut e, &[]);
        // Measuring: no error. A deleted form:
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(1));
        constant(&mut e, FORM_IS_DELETED, 1);
        tes_save_load_game_delete_form(&mut e, game_object, Ptr::new(form));
        assert!(calls_to(&e, LOG_ERROR).is_empty());
        assert_eq!(calls_to(&e, 0x03A0_0128), vec![vec![form, 0xFF00_0001, 1]]);
        // A cell too.
        constant(&mut e, FORM_IS_DELETED, 0);
        casts(&mut e, &[(form, RTTI_CELL, form)]);
        tes_save_load_game_delete_form(&mut e, game_object, Ptr::new(form));
        assert_eq!(calls_to(&e, 0x03A0_0128).len(), 2);
        assert!(calls_to(&e, LIST_ADD_HEAD).is_empty());
    }

    #[test]
    fn the_deferred_deletion_list_gets_a_form_once() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let contained: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        let state = contained.clone();
        e.register_double(LIST_CONTAINS, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0xF0F0);
            returns(*state.borrow() as u32)
        });
        stub(&mut e, LIST_ADD_HEAD);
        stub(&mut e, LIST_REMOVE);
        tes_save_load_game_add_form_to_deferred_deletions_list(
            &mut e,
            game_object,
            Ptr::new(0xF0F0),
        );
        assert_eq!(calls_to(&e, LIST_ADD_HEAD).len(), 1);
        *contained.borrow_mut() = true;
        tes_save_load_game_add_form_to_deferred_deletions_list(
            &mut e,
            game_object,
            Ptr::new(0xF0F0),
        );
        assert_eq!(calls_to(&e, LIST_ADD_HEAD).len(), 1);
        // Removing takes it off only when it is there.
        fn_0085a410(&mut e, game_object, Ptr::new(0xF0F0));
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
        *contained.borrow_mut() = false;
        fn_0085a410(&mut e, game_object, Ptr::new(0xF0F0));
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
        assert_eq!(calls_to(&e, LIST_REMOVE)[0][0], game_object.addr() + 0x34);
    }

    #[test]
    fn the_initial_data_size_is_a_reference_data_for_references_with_location_flags() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let form = Ptr::new(e.mem.alloc(0x40));
        // Not a reference and not a cell: nothing.
        casts(&mut e, &[]);
        assert_eq!(
            tes_save_load_game_get_initial_data_save_size(&mut e, game_object, form, 6),
            0
        );
        // A cell: nothing either.
        casts(&mut e, &[(form.addr(), RTTI_CELL, form.addr())]);
        assert_eq!(
            tes_save_load_game_get_initial_data_save_size(&mut e, game_object, form, 6),
            0
        );
        // A reference: 0x1C with bit 1 or 2, else nothing.
        casts(&mut e, &[(form.addr(), RTTI_REFERENCE, form.addr())]);
        for (flags, size) in [(2u32, 0x1Cu16), (4, 0x1C), (6, 0x1C), (1, 0), (0x38, 0)] {
            assert_eq!(
                tes_save_load_game_get_initial_data_save_size(&mut e, game_object, form, flags),
                size,
                "flags {flags:#x}"
            );
        }
    }

    /// The world of `SaveInitialData`: a reference with a position and a
    /// rotation, the singleton with a buffer.
    struct InitialRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        form: Ptr,
        buffer: u32,
    }

    fn initial_rig() -> InitialRig {
        let mut e = game();
        let game_object = game_singleton(&mut e);
        let buffer = e.mem.alloc(0x40);
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        let form = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(form.addr() + 0xC, 0xF00D);
        casts(&mut e, &[(form.addr(), RTTI_REFERENCE, form.addr())]);
        let position = e.mem.alloc(16);
        let rotation = e.mem.alloc(16);
        for i in 0..3 {
            e.mem.set_f32(position + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(rotation + 4 * i, 10.0 + i as f32);
        }
        constant(&mut e, REF_GET_POSITION, position);
        constant(&mut e, REF_GET_ROTATION, rotation);
        e.register(ADD_NUMERIC_ID, |_, a| returns(a[1] + 0x100));
        quiet(
            &mut e,
            &[
                LOG_ERROR,
                REF_GET_PARENT_CELL,
                CELL_GET_WORLDSPACE,
                REF_PERSISTS,
                REF_GET_EXTRA_LIST,
                EXTRA_GET_CELL_DATA,
                ACTOR_GET_PROCESS,
                READ_FIELD_28,
            ],
        );
        InitialRig {
            e,
            game: game_object,
            form,
            buffer,
        }
    }

    #[test]
    fn initial_data_writes_the_location_position_and_angle() {
        let mut rig = initial_rig();
        let e = &mut rig.e;
        // The parent cell is in a world space: its id gives the numeric id.
        let cell = e.mem.alloc(0x40);
        let worldspace = e.mem.alloc(0x40);
        e.mem.set_u32(worldspace + 0xC, 0x3A);
        constant(e, REF_GET_PARENT_CELL, cell);
        constant(e, CELL_GET_WORLDSPACE, worldspace);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        assert_eq!(e.mem.u32(rig.buffer), 0x13A);
        assert_eq!(e.mem.f32(rig.buffer + 4), 1.0);
        assert_eq!(e.mem.f32(rig.buffer + 0xC), 3.0);
        assert_eq!(e.mem.f32(rig.buffer + 0x10), 10.0);
        assert_eq!(e.mem.f32(rig.buffer + 0x18), 12.0);
        // 0x1C bytes were written.
        assert_eq!(
            e.get(rig.game, TESSaveLoadGame::m_pBuffer).addr(),
            rig.buffer + 0x1C
        );
        // Without bits 1 and 2 nothing is written (the calls are still made).
        e.set(rig.game, TESSaveLoadGame::m_pBuffer, Ptr::new(rig.buffer));
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 0x38);
        assert_eq!(
            e.get(rig.game, TESSaveLoadGame::m_pBuffer).addr(),
            rig.buffer
        );
        assert!(calls_to(e, LOG_ERROR).is_empty());
    }

    #[test]
    fn initial_data_uses_the_parent_cell_when_it_has_no_world_space() {
        let mut rig = initial_rig();
        let e = &mut rig.e;
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell + 0xC, 0x4B);
        constant(e, REF_GET_PARENT_CELL, cell);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 4);
        assert_eq!(e.mem.u32(rig.buffer), 0x14B);
    }

    #[test]
    fn initial_data_logs_a_reference_that_is_in_no_cell() {
        let mut rig = initial_rig();
        let e = &mut rig.e;
        // Not persistent: one message; persistent without the extra data:
        // the other.
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![vec![MSG_NON_PERSISTENT_NO_CELL, 0xF00D]]
        );
        constant(e, REF_PERSISTS, 1);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        assert_eq!(
            calls_to(e, LOG_ERROR)[1..],
            [vec![MSG_PERSISTENT_NO_CELL, 0xF00D]]
        );
        // The location id stays 0.
        assert_eq!(e.mem.u32(rig.buffer), 0);
    }

    #[test]
    fn initial_data_logs_a_cell_less_actor_by_its_process_level() {
        let mut rig = initial_rig();
        let e = &mut rig.e;
        constant(e, REF_PERSISTS, 1);
        constant(e, EXTRA_GET_CELL_DATA, 1);
        let reference = rig.form.addr();
        casts(
            e,
            &[
                (reference, RTTI_REFERENCE, reference),
                (reference, RTTI_MOBILE_OBJECT, 0xAC70),
            ],
        );
        constant(e, ACTOR_GET_PROCESS, 0x9000);
        // Level 0 is a high process, 1 a middle high process; others are fine.
        constant(e, READ_FIELD_28, 0);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        constant(e, READ_FIELD_28, 1);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        constant(e, READ_FIELD_28, 2);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 2);
        assert_eq!(
            calls_to(e, LOG_ERROR),
            vec![
                vec![MSG_HIGH_PROCESS_NO_CELL, 0xF00D],
                vec![MSG_MIDDLE_HIGH_PROCESS_NO_CELL, 0xF00D]
            ]
        );
    }

    #[test]
    fn initial_data_for_a_cell_or_another_form_writes_nothing() {
        let mut rig = initial_rig();
        let e = &mut rig.e;
        let form = rig.form.addr();
        casts(e, &[(form, RTTI_CELL, form)]);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 6);
        casts(e, &[]);
        tes_save_load_game_save_initial_data(e, rig.game, rig.form, 6);
        assert_eq!(
            e.get(rig.game, TESSaveLoadGame::m_pBuffer).addr(),
            rig.buffer
        );
    }

    // ---- the third batch of functions: helpers ----

    /// The game object `game()` made (the singleton the global points to).
    fn singleton_of(e: &Engine) -> Ptr<TESSaveLoadGame> {
        Ptr::new(e.global::<u32>(SAVE_LOAD_GAME))
    }

    /// A mini `sprintf` for the formats of this unit: `%i`, `%d`, `%s` and
    /// `%X`, with an optional `0` and width.
    fn format_text(e: &Engine, format: &[u8], args: &[u32]) -> Vec<u8> {
        let mut out = vec![];
        let (mut next, mut i) = (0usize, 0usize);
        while i < format.len() {
            if format[i] != b'%' {
                out.push(format[i]);
                i += 1;
                continue;
            }
            i += 1;
            let zero = format[i] == b'0';
            if zero {
                i += 1;
            }
            let mut width = 0usize;
            while format[i].is_ascii_digit() {
                width = width * 10 + (format[i] - b'0') as usize;
                i += 1;
            }
            let conversion = format[i];
            i += 1;
            let arg = args[next];
            next += 1;
            let text = match conversion {
                b'i' | b'd' => (arg as i32).to_string().into_bytes(),
                b'X' => format!("{arg:X}").into_bytes(),
                b's' => e.mem.cstr(arg),
                other => panic!("format {}", other as char),
            };
            let fill = if zero { b'0' } else { b' ' };
            out.extend(std::iter::repeat_n(fill, width.saturating_sub(text.len())));
            out.extend(text);
        }
        out
    }

    /// Working `sprintf_s` (`FORMAT`), `strcat_s` and `strcpy_s` doubles,
    /// and the pages the formats are written on.
    fn text_functions(e: &mut Engine) {
        for page in [0x0101_1000u32, 0x0102_0000, 0x0108_0000, 0x0108_1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        e.mem.set_cstr(SPACE, b" ");
        e.register(FORMAT, |e, a| {
            let format = e.mem.cstr(a[2]);
            let text = format_text(e, &format, &a[3..]);
            e.mem.set_cstr(a[0], &text);
            returns(text.len() as u32)
        });
        e.register(STRING_CAT, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
    }

    /// Writes the text of the constants at their addresses.
    fn put_texts(e: &mut Engine, texts: &[(u32, &[u8])]) {
        for &(address, text) in texts {
            e.mem.set_cstr(address, text);
        }
    }

    // ---- 0085ac30 ----

    /// Points the game's buffer at a new `ReferenceData` (location id 0x100,
    /// position (9000, 5000, 3), angle (0.5, 0.25, 0.125)).
    fn refill_location(e: &mut Engine, game: Ptr<TESSaveLoadGame>) {
        let buffer = e.mem.alloc(0x40);
        e.set(game, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        e.mem.set_u32(buffer, 0x100);
        for (i, v) in [9000.0f32, 5000.0, 3.0, 0.5, 0.25, 0.125]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(buffer + 4 + 4 * i as u32, *v);
        }
    }

    struct LocationRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        reference: u32,
        moves: Rc<RefCell<Vec<Vec<u32>>>>,
        positions: Rc<RefCell<Vec<[f32; 3]>>>,
    }

    /// A reference (not the player) whose buffer holds a `ReferenceData`
    /// with the location id 0x100 and the position (9000, 5000, 3); every
    /// numeric id maps to the id plus one.
    fn location_rig() -> LocationRig {
        let mut e = game();
        map_globals(&mut e);
        let game = singleton_of(&e);
        refill_location(&mut e, game);
        e.register(RESOLVE_NUMERIC_ID, |_, a| returns(a[1] + 1));
        e.register(CURRENT_VERSION, |_, _| returns(0x5B));
        quiet(
            &mut e,
            &[
                FN_00575700,
                FIX_CORRUPT_LOCATION,
                LOG_ERROR,
                CELL_REMOVE_REFERENCE,
                MODEL_LOADER_CANCEL_REFERENCE,
            ],
        );
        e.register(FLOAT_TO_INT, |_, a| {
            returns(f32::from_bits(a[0]) as i32 as u32)
        });
        let positions: Rc<RefCell<Vec<[f32; 3]>>> = Rc::new(RefCell::new(vec![]));
        let sink = positions.clone();
        e.register_double(REF_SET_POSITION, move |e, a| {
            sink.borrow_mut()
                .push([e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)]);
            Ret::default()
        });
        let moves: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(vec![]));
        let sink = moves.clone();
        e.register_double(REF_MOVE_TO_SPACE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let reference = object_with_slots(
            &mut e,
            &[
                (REFERENCE_SLOT_228, 0x0390_0228),
                (REFERENCE_SLOT_1CC, 0x0390_01CC),
            ],
        );
        quiet(&mut e, &[0x0390_0228, 0x0390_01CC]);
        e.set_global(PLAYER, 0x0099_0000u32);
        e.set_global(MODEL_LOADER, 0x0066_0000u32);
        LocationRig {
            e,
            game,
            reference,
            moves,
            positions,
        }
    }

    #[test]
    fn initial_data_of_a_cell_skips_two_bytes_of_an_old_save() {
        let mut e = game();
        map_globals(&mut e);
        let game = singleton_of(&e);
        let buffer = e.mem.alloc(8);
        e.set(game, TESSaveLoadGame::m_pBuffer, Ptr::new(buffer));
        casts(&mut e, &[(0x6000, RTTI_CELL, 0x6000)]);
        // Version 0x5A: the buffer moves on; 0x5B: it stays.
        e.register(CURRENT_VERSION, |_, _| returns(0x5A));
        fn_0085ac30(&mut e, game, Ptr::new(0x6000), 6);
        assert_eq!(e.get(game, TESSaveLoadGame::m_pBuffer).addr(), buffer + 2);
        e.register(CURRENT_VERSION, |_, _| returns(0x5B));
        fn_0085ac30(&mut e, game, Ptr::new(0x6000), 6);
        assert_eq!(e.get(game, TESSaveLoadGame::m_pBuffer).addr(), buffer + 2);
        // The version is asked for twice each time.
        assert_eq!(calls_to(&e, CURRENT_VERSION).len(), 4);
        // A form that is neither a reference nor a cell does nothing.
        casts(&mut e, &[]);
        fn_0085ac30(&mut e, game, Ptr::new(0x6000), 6);
        assert_eq!(calls_to(&e, CURRENT_VERSION).len(), 4);
    }

    #[test]
    fn initial_data_of_a_reference_needs_flag_bit_one_or_two() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        casts(e, &[(reference, RTTI_REFERENCE, reference)]);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 1);
        assert!(calls_to(e, RESOLVE_NUMERIC_ID).is_empty());
        assert!(calls_to(e, FN_00575700).is_empty());
    }

    #[test]
    fn initial_data_gives_the_player_only_the_angle() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        e.set_global(PLAYER, reference);
        casts(e, &[(reference, RTTI_REFERENCE, reference)]);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 2);
        assert_eq!(
            calls_to(e, RESOLVE_NUMERIC_ID),
            vec![vec![rig.game.addr(), 0x100]]
        );
        assert_eq!(
            calls_to(e, FN_00575700),
            vec![vec![
                reference,
                0.5f32.to_bits(),
                0.25f32.to_bits(),
                0.125f32.to_bits()
            ]]
        );
        assert!(rig.positions.borrow().is_empty());
        assert!(calls_to(e, FIX_CORRUPT_LOCATION).is_empty());
    }

    #[test]
    fn initial_data_moves_a_reference_into_its_saved_cell() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        constant(e, LOOKUP_FORM, 0x5000);
        casts(
            e,
            &[
                (reference, RTTI_REFERENCE, reference),
                (0x5000, RTTI_CELL, 0x5000),
            ],
        );
        constant(e, REF_GET_PARENT_CELL, 0x7000);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 2);
        // The id is mapped (0x100 + 1) and looked up; the position set and
        // the location fixed; the cell is another one, so the reference moves.
        assert_eq!(calls_to(e, LOOKUP_FORM), vec![vec![0x101]]);
        assert_eq!(*rig.positions.borrow(), vec![[9000.0, 5000.0, 3.0]]);
        assert_eq!(
            calls_to(e, FIX_CORRUPT_LOCATION),
            vec![vec![rig.game.addr(), reference]]
        );
        assert_eq!(*rig.moves.borrow(), vec![vec![reference, 0x5000, 0]]);
        // Already in that cell: no move.
        rig.moves.borrow_mut().clear();
        let e = &mut rig.e;
        let game = rig.game;
        refill_location(e, game);
        constant(e, REF_GET_PARENT_CELL, 0x5000);
        fn_0085ac30(e, game, Ptr::new(reference), 2);
        assert!(rig.moves.borrow().is_empty());
    }

    #[test]
    fn initial_data_picks_the_exterior_cell_from_the_position() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        constant(e, LOOKUP_FORM, 0x5100);
        casts(
            e,
            &[
                (reference, RTTI_REFERENCE, reference),
                (0x5100, RTTI_WORLDSPACE, 0x5100),
            ],
        );
        e.register(WORLDSPACE_GET_CELL, |_, a| returns(a[1] * 100 + a[2]));
        // (9000, 5000) is the grid cell (2, 1): cell 201. In no cell: moved.
        constant(e, REF_GET_PARENT_CELL, 0);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 2);
        assert_eq!(calls_to(e, WORLDSPACE_GET_CELL), vec![vec![0x5100, 2, 1]]);
        assert_eq!(*rig.moves.borrow(), vec![vec![reference, 0, 0x5100]]);
        // Already in cell 201 (an exterior cell): left alone.
        rig.moves.borrow_mut().clear();
        let e = &mut rig.e;
        let game = rig.game;
        constant(e, REF_GET_PARENT_CELL, 201);
        constant(e, CELL_IS_INTERIOR, 0);
        refill_location(e, game);
        fn_0085ac30(e, game, Ptr::new(reference), 2);
        assert!(rig.moves.borrow().is_empty());
        // In an interior cell: moved out whatever the cell.
        constant(e, CELL_IS_INTERIOR, 1);
        refill_location(e, game);
        fn_0085ac30(e, game, Ptr::new(reference), 2);
        assert_eq!(*rig.moves.borrow(), vec![vec![reference, 0, 0x5100]]);
    }

    #[test]
    fn initial_data_logs_a_non_persistent_reference_that_has_no_cell() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        constant(e, LOOKUP_FORM, 0x5100);
        // A world space whose cell for the position does not exist.
        casts(
            e,
            &[
                (reference, RTTI_REFERENCE, reference),
                (0x5100, RTTI_WORLDSPACE, 0x5100),
            ],
        );
        constant(e, WORLDSPACE_GET_CELL, 0);
        constant(e, REF_GET_PARENT_CELL, 0);
        constant(e, REF_PERSISTS, 0);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 2);
        assert_eq!(calls_to(e, LOG_ERROR), vec![vec![MSG_LOAD_NON_PERSISTENT]]);
        assert!(rig.moves.borrow().is_empty());
    }

    #[test]
    fn initial_data_takes_a_reference_with_no_place_out_of_its_cell() {
        let mut rig = location_rig();
        let (e, reference) = (&mut rig.e, rig.reference);
        // The location is neither a cell nor a world space.
        constant(e, LOOKUP_FORM, 0);
        casts(e, &[(reference, RTTI_REFERENCE, reference)]);
        constant(e, REF_PERSISTS, 0);
        fn_0085ac30(e, rig.game, Ptr::new(reference), 2);
        assert_eq!(calls_to(e, LOG_ERROR), vec![vec![MSG_PUT_NON_PERSISTENT]]);
        assert!(calls_to(e, CELL_REMOVE_REFERENCE).is_empty());
        // A persistent one is removed from its cell, its model request
        // cancelled and the two virtual hooks called.
        constant(e, REF_PERSISTS, 1);
        constant(e, REF_GET_PARENT_CELL, 0x7000);
        let game = rig.game;
        refill_location(e, game);
        fn_0085ac30(e, game, Ptr::new(reference), 2);
        assert_eq!(
            calls_to(e, CELL_REMOVE_REFERENCE),
            vec![vec![0x7000, reference]]
        );
        assert_eq!(
            calls_to(e, MODEL_LOADER_CANCEL_REFERENCE),
            vec![vec![0x0066_0000, reference]]
        );
        assert_eq!(calls_to(e, 0x0390_0228), vec![vec![reference, 0]]);
        assert_eq!(calls_to(e, 0x0390_01CC), vec![vec![reference, 0, 1]]);
        // Without a parent cell nothing is removed.
        constant(e, REF_GET_PARENT_CELL, 0);
        refill_location(e, game);
        fn_0085ac30(e, game, Ptr::new(reference), 2);
        assert_eq!(calls_to(e, CELL_REMOVE_REFERENCE).len(), 1);
        assert_eq!(calls_to(e, 0x0390_0228).len(), 2);
    }

    // ---- 0085b170, 0085b240 ----

    #[test]
    fn entry_hash_folds_the_mapped_ids_of_the_held_forms() {
        let mut e = game();
        map_globals(&mut e);
        let game = singleton_of(&e);
        // The object holds forms 0x0100000A and 0x0200000B in entries 1 and
        // 3; entry 0 is missing and entry 2 is empty. With no plugin table
        // the ids map to themselves.
        let held = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        e.mem.set_u32(held[0] + 0xC, 0x0100_000A);
        e.mem.set_u32(held[1] + 0xC, 0x0200_000B);
        let entries = e.mem.alloc(16);
        e.mem.set_u32(entries, held[0]);
        e.mem.set_u32(entries + 4, 0);
        e.mem.set_u32(entries + 8, held[1]);
        let reference = object_with_slots(&mut e, &[(REFERENCE_SLOT_1E8, 0x0390_01E8)]);
        constant(&mut e, 0x0390_01E8, 0x1234);
        e.register_double(OBJECT_ENTRY_AT, move |_, a| {
            returns(match a[1] {
                1 => entries,
                2 => entries + 4,
                3 => entries + 8,
                _ => 0,
            })
        });
        e.set(game, TESSaveLoadGame::m_pFileIndexArray, Ptr::NULL);
        let hash = fn_0085b170(&mut e, game, Ptr::new(reference));
        // Without a table the id is used as it is: (0 * 0x1003F + 0x0100000A)
        // for entry 1, then (that * 0x1003F + 0x0200000B) for entry 3.
        let first = 0x0100_000Au32;
        let expected = first.wrapping_mul(0x1003F).wrapping_add(0x0200_000B);
        assert_eq!(hash, expected);
        // The object is asked for the five entries.
        assert_eq!(calls_to(&e, OBJECT_ENTRY_AT).len(), 5);
        assert_eq!(calls_to(&e, 0x0390_01E8), vec![vec![reference]]);
        // A reference with no object hashes to 0.
        constant(&mut e, 0x0390_01E8, 0);
        assert_eq!(fn_0085b170(&mut e, game, Ptr::new(reference)), 0);
    }

    #[test]
    fn plugin_list_writes_the_count_and_each_name() {
        let mut e = game();
        map_globals(&mut e);
        let game = singleton_of(&e);
        e.set_global(DATA_HANDLER, 0x0044_0000u32);
        constant(&mut e, DATA_HANDLER_FILE_COUNT, 2);
        e.register(DATA_HANDLER_GET_COMPILED_FILE, |_, a| {
            returns(0x7000 + a[1])
        });
        let names = [e.mem.alloc(16), e.mem.alloc(16)];
        e.mem.set_cstr(names[0], b"FalloutNV.esm");
        e.mem.set_cstr(names[1], b"Mod.esp");
        e.register_double(COMPILED_FILE_NAME, move |_, a| {
            returns(names[(a[0] - 0x7000) as usize])
        });
        let writes: FileWrites = Rc::new(RefCell::new(vec![]));
        let sink = writes.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
            returns(a[2])
        });
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(0));
        let file = Ptr::new(0x8000);
        tes_save_load_game_save_plugin_list(&mut e, game, file);
        let written: Vec<Vec<u8>> = writes.borrow().iter().map(|w| w.1.clone()).collect();
        assert_eq!(
            written,
            vec![
                vec![2],
                vec![13],
                b"FalloutNV.esm".to_vec(),
                vec![7],
                b"Mod.esp".to_vec()
            ]
        );
        // Without statistics the position of the file is not asked for.
        assert!(calls_to(&e, FILE_POSITION).is_empty());
    }

    #[test]
    fn plugin_list_adds_an_extra_stat_for_the_bytes_written() {
        let mut e = game();
        map_globals(&mut e);
        let game = singleton_of(&e);
        e.set_global(DATA_HANDLER, 0x0044_0000u32);
        constant(&mut e, DATA_HANDLER_FILE_COUNT, 0);
        e.register(SAVE_LOAD_UNAVAILABLE, |_, _| returns(1));
        let positions = Rc::new(RefCell::new(VecDeque::from(vec![100u32, 136])));
        e.register_double(FILE_POSITION, move |_, _| {
            returns(positions.borrow_mut().pop_front().unwrap())
        });
        let stats: Ptr<SaveStats> = e.new_object();
        e.set(game, TESSaveLoadGame::m_pSaveLoadStats, stats);
        e.mem.set_cstr(LABEL_PLUGIN_LIST, b"Plugin List");
        e.register(LIST_ADD_HEAD, |_, _| Ret::default());
        tes_save_load_game_save_plugin_list(&mut e, game, Ptr::new(0x8000));
        // The one byte for the count is counted, not written (measuring).
        assert_eq!(e.get(game, TESSaveLoadGame::m_iSimulationFileSize), 1);
        let extra = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(extra.len(), 1);
        let stat = e.mem.u32(extra[0][1]);
        assert_eq!(e.mem.u32(stat), 36);
        assert_eq!(text(&e, e.mem.u32(stat + 4)), "Plugin List");
    }

    // ---- 0085b320 ----

    /// The texts of the lines of `BuildChangesString`, by address.
    const CHANGE_TEXTS: &[(u32, &[u8])] = &[
        (0x01080c98, b"Default Open State(%i)"),
        (0x01080cb0, b"CHANGE_OPEN_DEFAULT_STATE\r\n"),
        (0x01080ccc, b"Open State(%i)"),
        (0x01080cdc, b"CHANGE_OPEN_STATE\r\n"),
        (0x01080cf0, b"Teleport(%i)"),
        (0x01080d00, b"CHANGE_DOOR_EXTRA_TELEPORT\r\n"),
        (0x01080d20, b"Empty Flag(%i)"),
        (0x01080d30, b"CHANGE_OBJECT_EMPTY\r\n"),
        (0x01080d48, b"Dropped Item Flag(%i)"),
        (0x01080d60, b"CHANGEFLAG_OBJECT_DROPPED_NON_QUEST_ITEM\r\n"),
        (0x01080d8c, b"Rank Owner(%i)"),
        (0x01080d9c, b"CHANGE_OBJECT_EXTRA_RANK\r\n"),
        (0x01080db8, b"Global Owner(%i)"),
        (0x01080dcc, b"CHANGE_OBJECT_EXTRA_GLOBAL\r\n"),
        (0x01080dec, b"Owner(%i)"),
        (0x01080df8, b"CHANGE_OBJECT_EXTRA_OWNER\r\n"),
        (0x01080e14, b"Lock(%i)"),
        (0x01080e20, b"CHANGE_OBJECT_EXTRA_LOCK\r\n"),
        (0x01080e3c, b"Extra Magic(%i)"),
        (0x01080e4c, b"CHANGE_OBJECT_EXTRA_MAGIC\r\n"),
        (0x01080e68, b"Map Marker Flags(%i)"),
        (0x01080e80, b"CHANGE_MAPMARKER_EXTRA_FLAGS\r\n"),
        (0x01080ea0, b"Crime Gold(%i)"),
        (0x01080eb0, b"CHANGE_CHARACTER_EXTRA_CRIMEGOLD\r\n"),
        (0x01080ed4, b"Persuasion(%i)"),
        (0x01080ee4, b"CHANGE_ACTOR_EXTRA_PERSUASION\r\n"),
        (0x01080f04, b"Life State(%i)"),
        (0x01080f14, b"CHANGE_ACTOR_LIFESTATE\r\n"),
        (0x01080f30, b"Investment Gold(%i)"),
        (0x01080f44, b"CHANGE_ACTOR_EXTRA_INVESTMENTGOLD\r\n"),
        (0x01080f68, b"Oblivion Entry(%i)"),
        (0x01080f7c, b"CHANGE_ACTOR_EXTRA_OBLIVION_ENTRY\r\n"),
        (0x01080fa0, b"Disp Modifiers(%i)"),
        (0x01080fb4, b"CHANGE_ACTOR_DISPOSITION_MODIFIERS\r\n"),
        (0x01080fdc, b"Non-saved Package(%i)"),
        (0x01080ff4, b"CHANGE_ACTOR_NONSAVED_PACKAGE\r\n"),
        (0x01081014, b"%s %08X(%i)"),
        (0x01081020, b"Interrupt"),
        (0x0108102c, b"CHANGE_ACTOR_INTERRUPT_PACKAGE %08X\r\n"),
        (0x01081054, b"Trespass %08X(%i)"),
        (0x01081068, b"CHANGE_ACTOR_EXTRA_TRESPASS_PACKAGE %08X\r\n"),
        (0x01081094, b"Run Once %08X(%i)"),
        (0x010810a8, b"CHANGE_ACTOR_RUNONCE_PACKAGE %08X\r\n"),
        (0x010810cc, b"Temp Modifiers(%i)"),
        (0x010810e0, b"CHANGE_ACTOR_TEMP_MODIFIERS\r\n"),
        (0x01081100, b"Script Modifiers(%i)"),
        (0x01081118, b"CHANGE_ACTOR_PERMANENT_MODIFIERS\r\n"),
        (0x0108113c, b"Damage Modifiers(%i)"),
        (0x01081154, b"CHANGE_ACTOR_DAMAGE_MODIFIERS\r\n"),
        (0x01081174, b"Equipment(%i)"),
        (0x01081184, b"CHANGE_ACTOR_EQUIPMENT\r\n"),
        (0x010811a0, b"Movement Extra(%i)"),
        (0x010811b4, b"CHANGE_REFR_EXTRA_SAVEDMOVEMENTDATA\r\n"),
        (0x010811dc, b"Script(%i)"),
        (0x010811e8, b"CHANGE_REFR_EXTRA_SCRIPT\r\n"),
        (0x01081204, b"Inventory(%i)"),
        (0x01081214, b"CHANGE_REFR_INVENTORY\r\n"),
        (0x0108122c, b"Leveled Creature(%i)"),
        (0x01081244, b"CHANGE_REFR_EXTRA_LEVELED_CREATURE\r\n"),
        (0x0108126c, b"Disabled/Enabled(%i)"),
        (0x01081284, b"Enabled(%i)"),
        (0x01081290, b"Disabled(%i)"),
        (0x010812a0, b"CHANGE_REFR_DISABLE_STATE\r\n"),
        (0x010812bc, b"All Extra(%i)"),
        (0x010812cc, b"CHANGE_REFR_ALL_EXTRA\r\n"),
        (0x010812e4, b"Scale(%i)"),
        (0x010812f0, b"CHANGE_REFR_SCALE\r\n"),
        (0x01081304, b"Animation(%i)"),
        (0x01081314, b"CHANGE_REFR_ANIMATION\r\n"),
        (0x0108132c, b"Oblivion Flag(%i)"),
        (0x01081340, b"Oblivion Flag(0)"),
        (0x01081354, b"CHANGE_REFR_OBLIVION_FLAG\r\n"),
        (0x01081370, b"Had Havok Move Flag(%i)"),
        (0x01081388, b"CHANGEFLAG_REFR_HAD_HAVOK_MOVE_FLAG\r\n"),
        (0x010813b0, b"Havok Moved(%i)"),
        (0x010813c0, b"CHANGE_REFR_HAVOK_MOVE\r\n"),
        (0x010813dc, b"Moved(%i)"),
        (0x010813e8, b"CHANGE_REFR_MOVE\r\n"),
        (0x010813fc, b"Cell Changed(%i)"),
        (0x01081410, b"CHANGEFLAG_REFR_CELL_CHANGED\r\n"),
        (0x01081430, b"Created(%i)"),
        (0x0108143c, b"CHANGE_CREATED_NEW_REFERENCE\r\n"),
        (0x0108145c, b"Book Skill(%i)"),
        (0x0108146c, b"CHANGE_BOOK_TEACHES_SKILL\r\n"),
        (0x01081488, b"Faction Reactions(%i)"),
        (0x010814a0, b"CHANGE_FACTION_REACTIONS\r\n"),
        (0x010814bc, b"Faction Flags(%i)"),
        (0x010814d0, b"CHANGE_FACTION_FLAGS\r\n"),
        (0x010814e8, b"CHANGE_CELL_FULLNAME\r\n"),
        (0x01081500, b"Ownership(%i)"),
        (0x01081510, b"CHANGE_CELL_OWNERSHIP\r\n"),
        (0x01081528, b"Detach Time(%i)"),
        (0x01081538, b"CHANGE_CELL_DETACHTIME\r\n"),
        (0x01081554, b"Seen Data(%i)"),
        (0x01081564, b"CHANGE_CELL_SEENDATA\r\n"),
        (0x0108157c, b"Cell Created(%i)"),
        (0x01081590, b"CHANGE_CELL_CREATED\r\n"),
        (0x010815a8, b"Cell Flags(%i)"),
        (0x010815b8, b"CHANGE_CELL_FLAGS\r\n"),
        (0x010815cc, b"Waiting Flag(%i)"),
        (0x010815e0, b"CHANGE_PACKAGE_WAITING\r\n"),
        (0x010815fc, b"Never Run Flag(%i)"),
        (0x01081610, b"CHANGE_PACKAGE_NEVER_RUN\r\n"),
        (0x0108162c, b"CHANGE_TOPIC_SAIDONCE\r\n"),
        (0x01081644, b"Quest Script(No longer has script locals)"),
        (0x01081670, b"Quest Script(%i)"),
        (0x01081684, b"CHANGE_QUEST_SCRIPT\r\n"),
        (0x0108169c, b"CHANGE_QUEST_STAGES\r\n"),
        (0x010816b4, b"Quest Flags(%i)"),
        (0x010816c4, b"CHANGE_QUEST_FLAGS\r\n"),
        (0x010816dc, b"CHANGE_CREATURE_COMBATSTYLE\r\n"),
        (0x010816fc, b"CHANGE_CREATURE_SKILLS\r\n"),
        (0x01081718, b"Fame(%i)"),
        (0x01081724, b"CHANGE_NPC_FAME\r\n"),
        (0x01081738, b"Combat Style(%i)"),
        (0x0108174c, b"CHANGE_NPC_COMBATSTYLE\r\n"),
        (0x01081768, b"Skills(%i)"),
        (0x01081774, b"CHANGE_NPC_SKILLS\r\n"),
        (0x01081788, b"Full Name(%i)"),
        (0x01081798, b"CHANGE_ACTOR_BASE_FULLNAME\r\n"),
        (0x010817b8, b"AI Data(%i)"),
        (0x010817c4, b"CHANGE_ACTOR_BASE_AIDATA\r\n"),
        (0x010817e0, b"Base Modifiers(%i)"),
        (0x010817f4, b"CHANGE_ACTOR_BASE_MODIFIERS\r\n"),
        (0x01081814, b"Factions(%i)"),
        (0x01081824, b"CHANGE_ACTOR_BASE_FACTIONS\r\n"),
        (0x01081844, b"Spell List(%i)"),
        (0x01081854, b"CHANGE_ACTOR_BASE_SPELLLIST\r\n"),
        (0x01081874, b"Base Data(%i)"),
        (0x01081884, b"CHANGE_ACTOR_BASE_DATA\r\n"),
        (0x010818a0, b"Base Attributes(%i)"),
        (0x010818b4, b"CHANGE_ACTOR_BASE_ATTRIBUTES\r\n"),
        (0x010818d4, b"Base Health(%i)"),
        (0x010818e4, b"CHANGE_ACTOR_BASE_HEALTH\r\n"),
        (0x01081900, b"Form Flags(%i)"),
        (0x01081910, b"CHANGE_FORM_FLAGS\r\n"),
        (0x01081924, b"Base(%i)"),
        (0x01081930, b"High(%i)"),
        (0x0108193c, b"Process Level: High\r\n"),
        (0x01081954, b"Mid High(%i)"),
        (0x01081964, b"Process Level: Middle High\r\n"),
        (0x01081984, b"Mid Low(%i)"),
        (0x01081990, b"Process Level: Middle Low\r\n"),
        (0x010819ac, b"Low(%i)"),
        (0x010819b4, b"Process Level: Low\r\n"),
        (0x010819cc, b"None(%i)"),
        (0x010819d8, b"Process Level: None\r\n"),
    ];

    /// Writes `CHANGE_TEXTS` and `" "`.
    fn put_change_texts(e: &mut Engine) {
        text_functions(e);
        put_texts(e, CHANGE_TEXTS);
    }

    /// A name padded with spaces to the 26 characters of a short line.
    fn short_line(name: &str) -> String {
        format!("{name:<26}")
    }

    struct ChangesRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        buffer: Ptr,
        form: Ptr,
        /// The masks the form was asked the size of.
        asked: Rc<RefCell<Vec<u32>>>,
    }

    /// A form whose virtual slot `0x50` gives `100 + mask % 1000` (so the
    /// size of a part is `mask % 1000` once the 100 of mask 0 is taken off),
    /// not a reference and not a mobile object; the data handler knows no
    /// form (so `CheckFlags` leaves the flags).
    fn changes_rig() -> ChangesRig {
        let mut e = game();
        map_globals(&mut e);
        put_change_texts(&mut e);
        e.set_global(DATA_HANDLER, 0x0044_0000u32);
        constant(&mut e, DATA_HANDLER_HAS_FORM, 0);
        casts(&mut e, &[]);
        let asked: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = asked.clone();
        e.register_double(0x0390_0050, move |_, a| {
            sink.borrow_mut().push(a[1]);
            returns(100 + a[1] % 1000)
        });
        let form = object_with_slots(&mut e, &[(FORM_GET_CHANGES_SIZE, 0x0390_0050)]);
        let buffer = e.mem.alloc(CHANGES_BUFFER_SIZE);
        let game = singleton_of(&e);
        ChangesRig {
            e,
            game,
            buffer: Ptr::new(buffer),
            form: Ptr::new(form),
            asked,
        }
    }

    fn build(rig: &mut ChangesRig, flags: u32, form_type: u8, verbose: u8) -> String {
        tes_save_load_game_build_changes_string(
            &mut rig.e, rig.game, rig.buffer, rig.form, flags, form_type, verbose,
        );
        text(&rig.e, rig.buffer.addr())
    }

    #[test]
    fn changes_string_has_a_line_per_flag_with_the_size_of_its_part() {
        let mut rig = changes_rig();
        // A faction: form flags (1), faction flags (2), reactions (4). The
        // size of each part is what the form says less the 100 of the base.
        let text = build(&mut rig, 1 | 2 | 4, TYPE_FACTION, 0);
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Form Flags(1)")
                + &short_line("Faction Flags(2)")
                + &short_line("Faction Reactions(4)")
        );
        assert_eq!(*rig.asked.borrow(), vec![0, 1, 2, 4]);
        // The buffer is cleared first and flags that are not set have no line.
        let text = build(&mut rig, 4, TYPE_FACTION, 0);
        assert_eq!(
            text,
            short_line("Base(100)") + &short_line("Faction Reactions(4)")
        );
    }

    #[test]
    fn changes_string_verbose_form_lists_the_flag_names() {
        let mut rig = changes_rig();
        let text = build(&mut rig, 1 | 2, TYPE_FACTION, 1);
        assert_eq!(text, "CHANGE_FORM_FLAGS\r\nCHANGE_FACTION_FLAGS\r\n");
        // The form is not asked for sizes.
        assert!(rig.asked.borrow().is_empty());
        // A form of no type with no flags gives an empty string.
        assert_eq!(build(&mut rig, 0, 0, 1), "");
    }

    #[test]
    fn changes_string_without_a_form_prints_zero_sizes() {
        let mut rig = changes_rig();
        rig.form = Ptr::NULL;
        let text = build(&mut rig, 1, TYPE_FACTION, 0);
        assert_eq!(text, short_line("Form Flags(0)"));
        assert!(rig.asked.borrow().is_empty());
    }

    #[test]
    fn changes_string_actor_base_lines_depend_on_the_type() {
        let mut rig = changes_rig();
        let flags = 0x2 | 0x10 | 0x20 | 0x200;
        let text = build(&mut rig, flags, TYPE_NPC, 0);
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Base Data(2)")
                + &short_line("Spell List(16)")
                + &short_line("Full Name(32)")
                + &short_line("Skills(512)")
        );
        // A creature has the same, with its own verbose name for the skills.
        let text = build(&mut rig, flags, TYPE_CREATURE, 1);
        assert_eq!(
            text,
            "CHANGE_ACTOR_BASE_DATA\r\nCHANGE_ACTOR_BASE_SPELLLIST\r\n\
             CHANGE_ACTOR_BASE_FULLNAME\r\nCHANGE_CREATURE_SKILLS\r\n"
        );
        let text = build(&mut rig, 0x200, TYPE_NPC, 1);
        assert_eq!(text, "CHANGE_NPC_SKILLS\r\n");
    }

    #[test]
    fn changes_string_names_the_process_level_of_a_mobile_object() {
        let mut rig = changes_rig();
        let form = rig.form.addr();
        casts(&mut rig.e, &[(form, RTTI_MOBILE_OBJECT, form)]);
        // No process: none.
        constant(&mut rig.e, ACTOR_GET_PROCESS, 0);
        assert_eq!(build(&mut rig, 0, 0, 0), short_line("None(100)"));
        assert_eq!(build(&mut rig, 0, 0, 1), "Process Level: None\r\n");
        // With a process the level is the dword at +0x28 of it.
        constant(&mut rig.e, ACTOR_GET_PROCESS, 0x9000);
        for (level, short, long) in [
            (0, "High(100)", "Process Level: High\r\n"),
            (1, "Mid High(100)", "Process Level: Middle High\r\n"),
            (2, "Mid Low(100)", "Process Level: Middle Low\r\n"),
            (3, "Low(100)", "Process Level: Low\r\n"),
        ] {
            constant(&mut rig.e, READ_FIELD_28, level);
            assert_eq!(build(&mut rig, 0, 0, 0), short_line(short));
            assert_eq!(build(&mut rig, 0, 0, 1), long);
        }
        // Any other level leaves the line as it was (empty), still padded.
        constant(&mut rig.e, READ_FIELD_28, 9);
        assert_eq!(build(&mut rig, 0, 0, 0), short_line(""));
    }

    #[test]
    fn changes_string_quest_lines() {
        let mut rig = changes_rig();
        let flags = 0x2 | 0x8000_0000 | 0x4000_0000;
        let text = build(&mut rig, flags, TYPE_QUEST, 0);
        // The stages have no line of their own: the previous line is padded
        // (it is long enough already) and appended again.
        let flags_line = short_line("Quest Flags(2)");
        assert_eq!(
            text,
            short_line("Base(100)")
                + &flags_line
                + &format!("{flags_line} ")
                + &short_line("Quest Script(824)")
        );
        assert_eq!(
            build(&mut rig, flags, TYPE_QUEST, 1),
            "CHANGE_QUEST_FLAGS\r\nCHANGE_QUEST_STAGES\r\nCHANGE_QUEST_SCRIPT\r\n"
        );
        // A quest that no longer has script locals says so.
        let form = rig.form.addr();
        casts(&mut rig.e, &[(form, RTTI_QUEST, 0x7777)]);
        constant(&mut rig.e, QUEST_HAS_SCRIPT_LOCALS, 0);
        let text = build(&mut rig, 0x4000_0000, TYPE_QUEST, 0);
        assert_eq!(
            text,
            short_line("Base(100)") + "Quest Script(No longer has script locals) "
        );
        // One that has them prints the size.
        constant(&mut rig.e, QUEST_HAS_SCRIPT_LOCALS, 1);
        let text = build(&mut rig, 0x4000_0000, TYPE_QUEST, 0);
        assert_eq!(
            text,
            short_line("Base(100)") + &short_line("Quest Script(824)")
        );
    }

    #[test]
    fn changes_string_topic_package_cell_book_lines() {
        let mut rig = changes_rig();
        // A topic's "said once" has no line of its own: the base line left in
        // the scratch is appended again (with one more space).
        let text = build(&mut rig, 0x8000_0000, TYPE_TOPIC, 0);
        let base_line = short_line("Base(100)");
        assert_eq!(text, base_line.clone() + &format!("{base_line} "));
        assert_eq!(
            build(&mut rig, 0x8000_0000, TYPE_TOPIC, 1),
            "CHANGE_TOPIC_SAIDONCE\r\n"
        );
        // A package.
        let text = build(&mut rig, 0x8000_0000 | 0x4000_0000, TYPE_PACKAGE, 0);
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Never Run Flag(648)")
                + &short_line("Waiting Flag(824)")
        );
        // A cell: flags, seen data, detach time, ownership, name.
        let flags = 0x2 | 0x8000_0000 | 0x4000_0000 | 0x8 | 0x4;
        assert_eq!(
            build(&mut rig, flags, TYPE_CELL, 1),
            "CHANGE_CELL_FLAGS\r\nCHANGE_CELL_SEENDATA\r\nCHANGE_CELL_DETACHTIME\r\n\
             CHANGE_CELL_OWNERSHIP\r\nCHANGE_CELL_FULLNAME\r\n"
        );
        // A book.
        assert_eq!(
            build(&mut rig, 0x20, TYPE_BOOK, 0),
            short_line("Base(100)") + &short_line("Book Skill(32)")
        );
        // A type with no lines gives nothing for these flags.
        assert_eq!(build(&mut rig, 0x2 | 0x20, 0x01, 1), "");
    }

    /// A reference form of the changes rig: `form` also casts to a
    /// reference, it has a base object `0x6000` of the given type, and its
    /// extra data has container changes.
    fn reference_changes_rig(base_type: u32) -> ChangesRig {
        let mut rig = changes_rig();
        let form = rig.form.addr();
        casts(&mut rig.e, &[(form, RTTI_REFERENCE, form)]);
        constant(&mut rig.e, REF_PERSISTS, 1);
        constant(&mut rig.e, REF_GET_EXTRA_LIST, 0x7000);
        constant(&mut rig.e, EXTRA_GET_CONTAINER_CHANGES, 1);
        constant(&mut rig.e, REFERENCE_GET_BASE, 0x6000);
        constant(&mut rig.e, FORM_TYPE, base_type);
        rig
    }

    #[test]
    fn changes_string_reference_lines_for_a_moved_reference() {
        let mut rig = reference_changes_rig(0);
        // Moved: the size is what `GetInitialDataSaveSize` gives (0x1C for a
        // reference with the move flag). Havok moved with the move flag too
        // is the form's part less the base. Then animation, scale, inventory.
        let flags = 0x2 | 0x4 | 0x1000_0000 | 0x10 | 0x20;
        let text = build(&mut rig, flags, REFERENCE_TYPES[3], 0);
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Moved(28)")
                + &short_line("Havok Moved(4)")
                + &short_line("Animation(456)")
                + &short_line("Scale(16)")
                + &short_line("Inventory(32)")
        );
        assert_eq!(
            build(&mut rig, flags, REFERENCE_TYPES[3], 1),
            "CHANGE_REFR_MOVE\r\nCHANGE_REFR_HAVOK_MOVE\r\nCHANGE_REFR_ANIMATION\r\n\
             CHANGE_REFR_SCALE\r\nCHANGE_REFR_INVENTORY\r\n"
        );
        // Havok moved alone adds the initial data's size (0x1C).
        let text = build(&mut rig, 0x4, REFERENCE_TYPES[3], 0);
        assert_eq!(
            text,
            short_line("Base(100)") + &short_line("Havok Moved(32)")
        );
    }

    #[test]
    fn changes_string_actor_reference_lines() {
        let mut rig = reference_changes_rig(0);
        let flags = 0x20_0000 | 0x80_0000 | 0x10_0000 | 0x8_0000;
        for form_type in ACTOR_REFERENCE_TYPES {
            let text = build(&mut rig, flags, form_type, 0);
            assert_eq!(
                text,
                short_line("Base(100)")
                    + &short_line("Damage Modifiers(152)")
                    + &short_line("Script Modifiers(608)")
                    + &short_line("Temp Modifiers(576)")
                    + &short_line("Disp Modifiers(288)")
            );
        }
        // The sizes are the masks' remainders: 2097152 % 1000 = 152 and so on.
        let text = build(&mut rig, 0x20_0000, ACTOR_REFERENCE_TYPES[0], 0);
        assert_eq!(
            text,
            short_line("Base(100)") + &short_line("Damage Modifiers(152)")
        );
        // The lock and open lines are not for these types.
        let text = build(&mut rig, 0x1000, ACTOR_REFERENCE_TYPES[0], 1);
        assert_eq!(text, "");
    }

    #[test]
    fn changes_string_life_state_is_measured_against_the_move_bit() {
        let mut rig = reference_changes_rig(0);
        // Flags 0x404: the part asked for is 0x404 + (flags & 4) - hence
        // 0x408 ...
        let text = build(&mut rig, 0x400 | 0x4, ACTOR_REFERENCE_TYPES[0], 0);
        // `flags & 4` is 4: the part for 4 is 104 and for 0x404 is 100 + 1028
        // % 1000 = 128; but the Havok line (flag 4) comes first: 28 + 104 - 100.
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Havok Moved(32)")
                + &short_line("Life State(24)")
        );
        assert_eq!(
            build(&mut rig, 0x400, ACTOR_REFERENCE_TYPES[0], 1),
            "CHANGE_ACTOR_LIFESTATE\r\n"
        );
        // Without a form, size 0.
        rig.form = Ptr::NULL;
        let text = build(&mut rig, 0x400, ACTOR_REFERENCE_TYPES[0], 0);
        assert_eq!(text, short_line("Life State(0)"));
    }

    #[test]
    fn changes_string_door_lines_only_for_a_door_base_object() {
        let flags = 0x1000 | 0x20_0000 | 0x2_0000 | 0x80_0000 | 0x40_0000;
        // A door (base object type 0x1C): the teleport line is there.
        let mut rig = reference_changes_rig(BASE_TYPE_DOOR);
        let text = build(&mut rig, flags, REFERENCE_TYPES[0], 0);
        assert_eq!(
            text,
            short_line("Base(100)")
                + &short_line("Lock(96)")
                + &short_line("Empty Flag(152)")
                + &short_line("Teleport(72)")
                + &short_line("Open State(608)")
                + &short_line("Default Open State(304)")
        );
        // Another base object type: no teleport line.
        let mut rig = reference_changes_rig(5);
        let text = build(&mut rig, flags, REFERENCE_TYPES[0], 1);
        assert_eq!(
            text,
            "CHANGE_OBJECT_EXTRA_LOCK\r\nCHANGE_OBJECT_EMPTY\r\n\
             CHANGE_OPEN_STATE\r\nCHANGE_OPEN_DEFAULT_STATE\r\n"
        );
        // No base object: no teleport line either.
        constant(&mut rig.e, REFERENCE_GET_BASE, 0);
        let text = build(&mut rig, 0x2_0000, REFERENCE_TYPES[0], 1);
        assert_eq!(text, "");
        // A form that is not a reference at all has the teleport line.
        let mut rig = changes_rig();
        let text = build(&mut rig, 0x2_0000, REFERENCE_TYPES[0], 1);
        assert_eq!(text, "CHANGE_DOOR_EXTRA_TELEPORT\r\n");
    }

    // ---- 0085ef80 to 0085f850 ----

    /// The pointer-map stores of the animation, attached animation,
    /// character controller and Havok blocks, as (map, key, block) of each
    /// `SetAt`.
    fn record_set_at(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32, u32)>>> {
        let stored: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let sink = stored.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        stored
    }

    #[test]
    fn a_sized_block_has_its_size_in_front() {
        let mut e = game();
        let block = fn_0085ef80(&mut e, Ptr::NULL, 5);
        assert_eq!(e.mem.u16(block.addr()), 5);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![7]]);
    }

    #[test]
    fn a_map_block_reads_its_bytes_and_is_stored_under_the_key() {
        let mut e = game();
        let (game_object, buffer) = game_with_buffer(&mut e, 16);
        e.mem.write(buffer, b"abcdef");
        let stored = record_set_at(&mut e);
        fn_0085efc0(&mut e, game_object, Ptr::new(0x5000), 0x77, 4);
        let (map, key, block) = stored.borrow()[0];
        assert_eq!((map, key), (0x5000, 0x77));
        assert_eq!(e.mem.u16(block), 4);
        assert_eq!(e.mem.bytes(block + 2, 4), b"abcd");
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            buffer + 4
        );
    }

    #[test]
    fn an_animation_block_goes_under_zero_when_the_id_is_already_in_the_map() {
        let mut e = game();
        let (game_object, _) = game_with_buffer(&mut e, 16);
        e.set(
            game_object,
            TESSaveLoadGame::m_pAnimationMap,
            Ptr::new(0x5000),
        );
        let stored = record_set_at(&mut e);
        let form = form_with_key(&mut e, 0x1234);
        constant(&mut e, MAP_GET_AT, 0);
        fn_0085f010(&mut e, game_object, form, 2);
        assert_eq!(stored.borrow()[0].0, 0x5000);
        assert_eq!(stored.borrow()[0].1, 0x1234);
        constant(&mut e, MAP_GET_AT, 1);
        fn_0085f010(&mut e, game_object, form, 2);
        assert_eq!(stored.borrow()[1].1, 0);
        // The lookup was for the form's id in the animation map.
        assert_eq!(calls_to(&e, MAP_GET_AT)[0][..2], [0x5000, 0x1234]);
    }

    #[test]
    fn the_other_block_kinds_use_their_own_maps() {
        let mut e = game();
        let (game_object, _) = game_with_buffer(&mut e, 16);
        e.set(
            game_object,
            TESSaveLoadGame::m_pAttachedAnimationMap,
            Ptr::new(0x5001),
        );
        e.set(
            game_object,
            TESSaveLoadGame::m_pCharControllerMap,
            Ptr::new(0x5002),
        );
        e.set(
            game_object,
            TESSaveLoadGame::m_pHavokDataMap,
            Ptr::new(0x5003),
        );
        let stored = record_set_at(&mut e);
        let form = form_with_key(&mut e, 0x4321);
        fn_0085f2b0(&mut e, game_object, form, 2);
        fn_0085f420(&mut e, game_object, form, 2);
        fn_0085f5a0(&mut e, game_object, form, 2);
        let maps: Vec<(u32, u32)> = stored.borrow().iter().map(|s| (s.0, s.1)).collect();
        assert_eq!(
            maps,
            vec![(0x5001, 0x4321), (0x5002, 0x4321), (0x5003, 0x4321)]
        );
    }

    /// A reference object with the animation slot `0x1E4` giving
    /// `animation`.
    fn reference_with_animation(e: &mut Engine, animation: u32) -> u32 {
        let slot = 0x0390_0000 + animation;
        constant(e, slot, animation);
        object_with_slots(e, &[(REFERENCE_SLOT_1E4, slot)])
    }

    #[test]
    fn animation_blocks_reach_the_references_that_have_a_node() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        e.set(
            game_object,
            TESSaveLoadGame::m_pAnimationMap,
            Ptr::new(0xAAA0),
        );
        let blocks: Vec<u32> = (0..4).map(|_| e.mem.alloc(8)).collect();
        install_entries(
            &mut e,
            &[
                (0, blocks[0]),
                (0x500, blocks[1]),
                (0x501, blocks[2]),
                (0x502, blocks[3]),
            ],
        );
        let player = 0x0099_0000u32;
        e.set_global(PLAYER, player);
        let reference = reference_with_animation(&mut e, 0x5100);
        // The id is the key plus 0x1000: 0x1500 is a reference with a node,
        // 0x1501 one without, 0x1502 is no reference.
        e.register(LOOKUP_FORM, |_, a| returns(a[0] + 0x1000));
        casts(
            &mut e,
            &[
                (0x1500, RTTI_REFERENCE, reference),
                (0x1501, RTTI_REFERENCE, 0x6501),
            ],
        );
        e.register(REF_GET_NODE, |_, a| returns((a[0] != 0x6501) as u32));
        constant(&mut e, PLAYER_GET_ANIMATION, 0x5000);
        quiet(&mut e, &[ANIMATION_LOAD, MAP_REMOVE_AT]);
        fn_0085f070(&mut e, game_object);
        // The player (key 0) gets the first-person animation.
        assert_eq!(calls_to(&e, PLAYER_GET_ANIMATION), vec![vec![player, 1]]);
        assert_eq!(
            calls_to(&e, ANIMATION_LOAD),
            vec![vec![0x5000, player], vec![0x5100, reference]]
        );
        let removed: Vec<u32> = calls_to(&e, MAP_REMOVE_AT).iter().map(|c| c[1]).collect();
        assert_eq!(removed, vec![0, 0x500, 0x501, 0x502]);
        for block in blocks {
            assert!(freed(&e, block));
        }
    }

    #[test]
    fn an_animation_block_for_the_third_person_player_uses_the_other_animation() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        let player = 0x0099_0000u32;
        e.set_global(PLAYER, player);
        casts(&mut e, &[]);
        constant(&mut e, PLAYER_GET_ANIMATION, 0x5000);
        quiet(&mut e, &[ANIMATION_LOAD]);
        let block = e.mem.alloc(8);
        fn_0085f150(&mut e, game_object, Ptr::new(player), 9, Ptr::new(block));
        assert_eq!(calls_to(&e, PLAYER_GET_ANIMATION), vec![vec![player, 0]]);
    }

    #[test]
    fn a_mobile_objects_process_values_are_read_and_given_back() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        let previous = e.mem.alloc(8);
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(previous));
        e.set_global(PLAYER, 0x0099_0000u32);
        let reference = reference_with_animation(&mut e, 0x5100);
        let process = object_with_slots(
            &mut e,
            &[
                (PROCESS_SLOT_3E4, 0x0390_03E4),
                (PROCESS_SLOT_3E8, 0x0390_03E8),
                (PROCESS_SLOT_3EC, 0x0390_03EC),
            ],
        );
        constant(&mut e, 0x0390_03E4, 7);
        constant(&mut e, 0x0390_03E8, 0x2008);
        quiet(&mut e, &[0x0390_03EC, ANIMATION_LOAD]);
        e.register(ANIMATION_ADJUST, |_, a| returns(a[1] + 1));
        constant(&mut e, ACTOR_GET_PROCESS, 0x9000);
        casts(
            &mut e,
            &[
                (reference, RTTI_MOBILE_OBJECT, 0xAC70),
                (0x9000, RTTI_HIGH_PROCESS, process),
            ],
        );
        let block = e.mem.alloc(8);
        fn_0085f150(&mut e, game_object, Ptr::new(reference), 5, Ptr::new(block));
        assert_eq!(calls_to(&e, ANIMATION_LOAD), vec![vec![0x5100, reference]]);
        assert_eq!(calls_to(&e, ANIMATION_ADJUST), vec![vec![0x5100, 0x2000]]);
        assert_eq!(calls_to(&e, 0x0390_03EC), vec![vec![process, 7, 0x2001]]);
        // The game's own buffer is back, and the block's 2 bytes were skipped.
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            previous
        );
        // With key 0 (no process values) the process is not touched.
        let before = calls_to(&e, 0x0390_03EC).len();
        fn_0085f150(&mut e, game_object, Ptr::new(reference), 0, Ptr::new(block));
        assert_eq!(calls_to(&e, 0x0390_03EC).len(), before);
        assert_eq!(calls_to(&e, 0x0390_03E4).len(), 1);
    }

    /// Reference objects (the form with id `0x1000 + key` of each key given
    /// casts to one); the ones in `without_node` have no 3D node. Returns
    /// the objects of each list.
    fn node_world(e: &mut Engine, with_node: &[u32], without_node: &[u32]) -> (Vec<u32>, Vec<u32>) {
        e.register(LOOKUP_FORM, |_, a| returns(a[0] + 0x1000));
        let (mut table, mut with, mut without) = (vec![], vec![], vec![]);
        for (keys, objects) in [(with_node, &mut with), (without_node, &mut without)] {
            for &key in keys {
                let object = e.mem.alloc(0x20);
                e.mem.set_u32(object + 0xC, 0x1000 + key);
                table.push((0x1000 + key, RTTI_REFERENCE, object));
                objects.push(object);
            }
        }
        casts(e, &table);
        let nodeless = without.clone();
        e.register_double(REF_GET_NODE, move |_, a| {
            returns(!nodeless.contains(&a[0]) as u32)
        });
        (with, without)
    }

    #[test]
    fn attached_animation_blocks_reach_the_references_that_have_a_node() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        e.set(
            game_object,
            TESSaveLoadGame::m_pAttachedAnimationMap,
            Ptr::new(0xAAA1),
        );
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        install_entries(
            &mut e,
            &[(0x500, blocks[0]), (0x501, blocks[1]), (0x502, blocks[2])],
        );
        let (with, without) = node_world(&mut e, &[0x500], &[0x501]);
        // The reference without a node has its changes flags put back into
        // the changes map (the new one when there is one).
        let change_data = e.mem.alloc(8);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], change_data);
            returns(1)
        });
        quiet(&mut e, &[MAP_REMOVE_AT, SET_LOADING_STATE]);
        let changes = e.mem.alloc(0x10);
        e.set(game_object, TESSaveLoadGame::m_pChanges, Ptr::new(changes));
        fn_0085f2e0(&mut e, game_object);
        assert_eq!(calls_to(&e, SET_LOADING_STATE), vec![vec![with[0], 0]]);
        assert_eq!(calls_to(&e, MAP_GET_AT)[0][..2], [changes, 0x1501]);
        let _ = &without;
        // The reference without a node keeps its block; the others free it.
        assert!(freed(&e, blocks[0]));
        assert!(!freed(&e, blocks[1]));
        assert!(freed(&e, blocks[2]));
        let removed: Vec<u32> = calls_to(&e, MAP_REMOVE_AT).iter().map(|c| c[1]).collect();
        assert_eq!(removed, vec![0x500, 0x501, 0x502]);
        // With a new changes map that one is used.
        let new_changes = e.mem.alloc(0x10);
        e.set(
            game_object,
            TESSaveLoadGame::m_pNewChanges,
            Ptr::new(new_changes),
        );
        install_entries(&mut e, &[(0x501, blocks[1])]);
        fn_0085f2e0(&mut e, game_object);
        assert_eq!(calls_to(&e, MAP_GET_AT)[1][0], new_changes);
    }

    #[test]
    fn havok_blocks_reach_the_references_that_have_a_node() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        e.set(
            game_object,
            TESSaveLoadGame::m_pHavokDataMap,
            Ptr::new(0xAAA3),
        );
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        e.mem.set_u16(blocks[0], 0x1234);
        install_entries(
            &mut e,
            &[(0x500, blocks[0]), (0x501, blocks[1]), (0x502, blocks[2])],
        );
        let (with, _) = node_world(&mut e, &[0x500], &[0x501]);
        let change_data = e.mem.alloc(8);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], change_data);
            returns(1)
        });
        quiet(&mut e, &[MAP_REMOVE_AT, REFERENCE_APPLY_HAVOK_BLOCK]);
        let changes = e.mem.alloc(0x10);
        e.set(game_object, TESSaveLoadGame::m_pChanges, Ptr::new(changes));
        fn_0085f5d0(&mut e, game_object);
        assert_eq!(
            calls_to(&e, REFERENCE_APPLY_HAVOK_BLOCK),
            vec![vec![with[0], 0x1234]]
        );
        assert_eq!(calls_to(&e, MAP_GET_AT)[0][0], changes);
        assert!(freed(&e, blocks[0]));
        assert!(!freed(&e, blocks[1]));
        assert!(freed(&e, blocks[2]));
    }

    #[test]
    fn single_blocks_are_read_with_the_buffer_pointed_at_them() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        let previous = e.mem.alloc(8);
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(previous));
        quiet(&mut e, &[SET_LOADING_STATE, REFERENCE_APPLY_HAVOK_BLOCK]);
        let block = e.mem.alloc(8);
        e.mem.set_u16(block, 0xBEEF);
        fn_0085f3d0(&mut e, game_object, Ptr::new(0x5500), Ptr::new(block));
        fn_0085f6c0(&mut e, game_object, Ptr::new(0x5500), Ptr::new(block));
        assert_eq!(calls_to(&e, SET_LOADING_STATE), vec![vec![0x5500, 0]]);
        assert_eq!(
            calls_to(&e, REFERENCE_APPLY_HAVOK_BLOCK),
            vec![vec![0x5500, 0xBEEF]]
        );
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            previous
        );
    }

    #[test]
    fn character_controller_blocks_load_or_log_for_each_mobile_object() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        e.set(
            game_object,
            TESSaveLoadGame::m_pCharControllerMap,
            Ptr::new(0xAAA2),
        );
        let blocks: Vec<u32> = (0..5).map(|_| e.mem.alloc(8)).collect();
        install_entries(
            &mut e,
            &[
                (0x600, blocks[0]),
                (0x601, blocks[1]),
                (0x602, blocks[2]),
                (0x603, blocks[3]),
                (0x604, blocks[4]),
            ],
        );
        // Key k is the form 0x1000 + k, a mobile object (except 0x603) with
        // the process 0x9000 + k; the process is a high process (except for
        // 0x604) whose slot 0x28C is true only for 0x600.
        e.register(LOOKUP_FORM, |_, a| returns(a[0] + 0x1000));
        let mut table = vec![];
        let mut mobiles = vec![];
        for key in [0x600u32, 0x601, 0x602, 0x604] {
            let mobile = object_with_slots(&mut e, &[(FORM_GET_DESCRIPTION, 0x0390_0130)]);
            e.mem.set_u32(mobile + 0xC, key + 0x10);
            table.push((0x1000 + key, RTTI_MOBILE_OBJECT, mobile));
            mobiles.push(mobile);
            if key != 0x604 {
                let controller = object_with_slots(&mut e, &[(PROCESS_SLOT_28C, 0x0390_028C)]);
                table.push((0x9000 + key, RTTI_HIGH_PROCESS, controller));
            }
        }
        casts(&mut e, &table);
        e.register(ACTOR_GET_PROCESS, move |e, a| {
            let id = e.mem.u32(a[0] + 0xC) - 0x10;
            returns(0x9000 + id)
        });
        let name = e.mem.alloc(16);
        constant(&mut e, 0x0390_0130, name);
        let flags: Rc<RefCell<VecDeque<u32>>> =
            Rc::new(RefCell::new(VecDeque::from(vec![1, 0, 0])));
        e.register_double(0x0390_028C, move |_, _| {
            returns(flags.borrow_mut().pop_front().unwrap())
        });
        quiet(&mut e, &[LOAD_CHAR_CONTROLLER, LOG_ERROR, MAP_REMOVE_AT]);
        // 0x601 passes both tests; 0x602 fails the first.
        e.register(MOBILE_TEST_A, |_, a| returns((a[0] != 0) as u32));
        let first_mobile = mobiles[1];
        e.register_double(MOBILE_TEST_B, move |_, a| {
            returns((a[0] == first_mobile) as u32)
        });
        let previous = e.mem.alloc(8);
        e.set(game_object, TESSaveLoadGame::m_pBuffer, Ptr::new(previous));
        fn_0085f450(&mut e, game_object);
        assert_eq!(
            calls_to(&e, LOAD_CHAR_CONTROLLER).len(),
            1,
            "only 0x600 has a controller to load"
        );
        assert_eq!(calls_to(&e, LOAD_CHAR_CONTROLLER)[0][1], mobiles[0]);
        // 0x601 passes both tests, 0x602 does not: one error, with the name
        // and the id.
        assert_eq!(
            calls_to(&e, LOG_ERROR),
            vec![vec![MSG_NO_CHAR_CONTROLLER, name, 0x602 + 0x10]]
        );
        for block in blocks {
            assert!(freed(&e, block));
        }
        assert_eq!(calls_to(&e, MAP_REMOVE_AT).len(), 5);
        assert_eq!(
            e.get(game_object, TESSaveLoadGame::m_pBuffer).addr(),
            previous
        );
    }

    #[test]
    fn a_form_is_queued_once() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        let contained = Rc::new(RefCell::new(false));
        let flag = contained.clone();
        e.register_double(LIST_CONTAINS, move |_, _| returns(*flag.borrow() as u32));
        quiet(&mut e, &[LIST_ADD_HEAD]);
        fn_0085f710(&mut e, game_object, Ptr::new(0x5500));
        let added = calls_to(&e, LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], game_object.addr() + 0x68);
        assert_eq!(e.mem.u32(added[0][1]), 0x5500);
        *contained.borrow_mut() = true;
        fn_0085f710(&mut e, game_object, Ptr::new(0x5500));
        assert_eq!(calls_to(&e, LIST_ADD_HEAD).len(), 1);
    }

    #[test]
    fn the_queued_arrow_projectiles_are_finished_and_the_list_emptied() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        // Two arrows, each holding an object whose +0x28 is the form it is
        // stuck in (only the first has a node), then an empty item and a
        // form that is no arrow.
        let arrows = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        let held = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for i in 0..2 {
            e.mem.set_u32(arrows[i] + 0x88, held[i]);
            e.mem.set_u32(held[i] + 0x28, 0x9001 + i as u32);
        }
        let third = list_of(&mut e, &[0, 0x5003]);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, arrows[1]);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(game_object.addr() + 0x68, arrows[0]);
        e.mem.set_u32(game_object.addr() + 0x6C, second);
        casts(
            &mut e,
            &[
                (arrows[0], RTTI_ARROW_PROJECTILE, arrows[0]),
                (arrows[1], RTTI_ARROW_PROJECTILE, arrows[1]),
            ],
        );
        e.register(REF_GET_NODE, |_, a| {
            returns(if a[0] == 0x9001 { 0xA001 } else { 0 })
        });
        quiet(
            &mut e,
            &[
                FN_0043D410,
                FN_00A59C60,
                ARROW_PROJECTILE_FINISH,
                LIST_REMOVE_ALL,
            ],
        );
        fn_0085f750(&mut e, game_object);
        assert_eq!(
            calls_to(&e, ARROW_PROJECTILE_FINISH),
            vec![vec![arrows[0]], vec![arrows[1]]]
        );
        let constructed = calls_to(&e, FN_0043D410);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1..], [0.0f32.to_bits(), 0, 0]);
        assert_eq!(
            calls_to(&e, FN_00A59C60),
            vec![vec![0xA001, constructed[0][0]]]
        );
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ALL),
            vec![vec![game_object.addr() + 0x68]]
        );
    }

    #[test]
    fn what_an_arrow_is_stuck_in_is_read_through_its_held_object() {
        let mut e = game();
        let arrow = e.mem.alloc(0x100);
        assert_eq!(fn_0085f810(&mut e, Ptr::new(arrow)), 0);
        let held = e.mem.alloc(0x40);
        e.mem.set_u32(held + 0x28, 0xCAFE);
        e.mem.set_u32(arrow + 0x88, held);
        assert_eq!(fn_0085f810(&mut e, Ptr::new(arrow)), 0xCAFE);
    }

    #[test]
    fn the_node_dependent_loads_finish_only_when_the_game_is_loading() {
        let mut e = game();
        map_globals(&mut e);
        let game_object = singleton_of(&e);
        constant(&mut e, MAP_FIRST_POSITION, 0);
        quiet(
            &mut e,
            &[
                PROCESS_LISTS_FN_00978420,
                TES_FN_00453550,
                WORLD_DEACTIVATE_ALL_ISLANDS,
                LIST_REMOVE_ALL,
            ],
        );
        e.set_global(PLAYER, 0x0099_0000u32);
        e.set_global(TES_OBJECT, 0x0098_0000u32);
        constant(&mut e, REF_GET_PARENT_CELL, 0x7000);
        constant(&mut e, CELL_GET_PHYSICS_WORLD, 0x7100);
        // `0047c850` false (this build): nothing happens.
        fn_0085f850(&mut e, game_object, true);
        assert!(calls_to(&e, MAP_FIRST_POSITION).is_empty());
        // True: the three loops run, the settle steps happen when asked.
        constant(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        fn_0085f850(&mut e, game_object, false);
        assert_eq!(calls_to(&e, MAP_FIRST_POSITION).len(), 4);
        assert!(calls_to(&e, PROCESS_LISTS_FN_00978420).is_empty());
        fn_0085f850(&mut e, game_object, true);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_FN_00978420),
            vec![vec![PROCESS_LISTS, 0.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, TES_FN_00453550),
            vec![vec![0x0098_0000, 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, CELL_GET_PHYSICS_WORLD), vec![vec![0x7000]]);
        assert_eq!(
            calls_to(&e, WORLD_DEACTIVATE_ALL_ISLANDS),
            vec![vec![0x7100]]
        );
        // No cell, or a cell with no world: no deactivation.
        constant(&mut e, CELL_GET_PHYSICS_WORLD, 0);
        fn_0085f850(&mut e, game_object, true);
        assert_eq!(calls_to(&e, WORLD_DEACTIVATE_ALL_ISLANDS).len(), 1);
        constant(&mut e, REF_GET_PARENT_CELL, 0);
        fn_0085f850(&mut e, game_object, true);
        assert_eq!(calls_to(&e, CELL_GET_PHYSICS_WORLD).len(), 2);
    }

    // ---- 0085f900 to 0085fed0: the list of saves ----

    /// A `SaveGameFile` object whose virtual slot `0x18` (the name) gives
    /// `name` and slot 0 (the destructor) is a logged double.
    fn save_file(e: &mut Engine, name: u32) -> Ptr<SaveGameFile> {
        let file: Ptr<SaveGameFile> = e.new_object();
        let vtable = e.mem.alloc(0x40);
        let name_slot = 0x0391_0000 + name;
        constant(e, name_slot, name);
        e.mem.set_u32(vtable + FILE_GET_NAME, name_slot);
        e.mem.set_u32(vtable + FILE_DESTRUCT, 0x0391_0000);
        e.mem.set_u32(file.addr(), vtable);
        stub(e, 0x0391_0000);
        file
    }

    #[test]
    fn a_save_game_file_is_a_bs_file_with_its_own_vtable_and_no_time() {
        let mut e = game();
        let file: Ptr<SaveGameFile> = e.new_object();
        e.mem.set_u32(file.addr() + 0x158, 0x0101_0101);
        e.mem.set_u32(file.addr(), 0xDEAD);
        quiet(&mut e, &[BSFILE_CONSTRUCT]);
        let result = fn_0085fb40(&mut e, file, Ptr::new(0x5000), 1, 0x20000);
        assert_eq!(result, file);
        assert_eq!(
            calls_to(&e, BSFILE_CONSTRUCT),
            vec![vec![file.addr(), 0x5000, 1, 0x20000, 0]]
        );
        assert_eq!(e.mem.u32(file.addr()), SAVE_GAME_FILE_VTABLE);
        assert!(!fn_0085feb0(&mut e, file));
    }

    #[test]
    fn a_save_game_file_remembers_its_file_time() {
        let mut e = game();
        let file: Ptr<SaveGameFile> = e.new_object();
        assert!(!fn_0085feb0(&mut e, file));
        fn_0085fe80(&mut e, file, 0x1111, 0x2222);
        assert!(fn_0085feb0(&mut e, file));
        let out = e.mem.alloc(8);
        let result = fn_0085fe50(&mut e, file, Ptr::new(out));
        assert_eq!(result.addr(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x1111, 0x2222));
    }

    #[test]
    fn a_save_game_file_destructs_its_bs_file_and_frees_on_bit_zero() {
        let mut e = game();
        quiet(&mut e, &[BSFILE_DESTRUCT]);
        let file: Ptr<SaveGameFile> = e.new_object();
        fn_0085fbb0(&mut e, file);
        assert_eq!(calls_to(&e, BSFILE_DESTRUCT), vec![vec![file.addr()]]);
        assert_eq!(fn_0085fb80(&mut e, file, 0), file);
        assert!(!freed(&e, file.addr()));
        assert_eq!(fn_0085fb80(&mut e, file, 1), file);
        assert!(freed(&e, file.addr()));
        assert_eq!(calls_to(&e, BSFILE_DESTRUCT).len(), 3);
    }

    #[test]
    fn the_list_of_saves_is_destroyed_with_its_files() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        // Without a list nothing happens.
        quiet(&mut e, &[LIST_REMOVE_ALL, LIST_SCALAR_DELETE]);
        fn_0085fbd0(&mut e, game_object);
        assert!(calls_to(&e, LIST_REMOVE_ALL).is_empty());
        let files = [save_file(&mut e, 1), save_file(&mut e, 2)];
        let list = list_of(&mut e, &[files[0].addr(), 0, files[1].addr()]);
        e.set(
            game_object,
            TESSaveLoadGame::m_pSaveGameList,
            Ptr::new(list),
        );
        fn_0085fbd0(&mut e, game_object);
        // Each file is destroyed (flag 1); the empty item is skipped.
        assert_eq!(
            calls_to(&e, 0x0391_0000),
            vec![vec![files[0].addr(), 1], vec![files[1].addr(), 1]]
        );
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![list]]);
        assert_eq!(calls_to(&e, LIST_SCALAR_DELETE), vec![vec![list, 1]]);
        assert!(e
            .get(game_object, TESSaveLoadGame::m_pSaveGameList)
            .is_null());
    }

    type FoundFile = (String, u32, u32, u32);

    /// Doubles for the file system: `FindFirstFileA` / `FindNextFileA`
    /// give `files` (name, last write time low, high, size) in turn.
    fn file_system(e: &mut Engine, files: &[(&str, u32, u32, u32)]) {
        let queue: Rc<RefCell<VecDeque<FoundFile>>> = Rc::new(RefCell::new(
            files
                .iter()
                .map(|(n, l, h, s)| (n.to_string(), *l, *h, *s))
                .collect(),
        ));
        fn give(e: &mut Engine, data: u32, entry: (String, u32, u32, u32)) {
            e.mem.set_u32(data + FIND_DATA_LAST_WRITE_TIME, entry.1);
            e.mem.set_u32(data + FIND_DATA_LAST_WRITE_TIME + 4, entry.2);
            e.mem.set_u32(data + FIND_DATA_SIZE_HIGH, 0);
            e.mem.set_u32(data + FIND_DATA_SIZE_LOW, entry.3);
            e.mem
                .set_cstr(data + FIND_DATA_FILE_NAME, entry.0.as_bytes());
        }
        let q = queue.clone();
        e.register_double(FIND_FIRST_FILE_IMPORT, move |e, a| {
            let wanted = text(e, a[0]);
            let mut q = q.borrow_mut();
            // A path of the folder search gives the first entry; the
            // path of a file gives the entry of that name.
            let found = if wanted.ends_with("*.ess") {
                q.pop_front()
            } else {
                q.iter().find(|f| wanted.ends_with(&f.0)).cloned()
            };
            match found {
                Some(entry) => {
                    give(e, a[1], entry);
                    returns(7)
                }
                None => returns(0xFFFF_FFFF),
            }
        });
        let q = queue.clone();
        e.register_double(FIND_NEXT_FILE_IMPORT, move |e, a| {
            match q.borrow_mut().pop_front() {
                Some(entry) => {
                    give(e, a[1], entry);
                    returns(1)
                }
                None => returns(0),
            }
        });
        stub(e, FIND_CLOSE_IMPORT);
    }

    #[test]
    fn the_list_of_saves_holds_each_non_empty_ess_file() {
        let mut e = game();
        map_globals(&mut e);
        text_functions(&mut e);
        e.map(0x0104_f000, 0x1000);
        e.mem.set_cstr(FORMAT_THREE_STRINGS, b"%s%s%s");
        e.mem.set_cstr(SAVE_FILE_PATTERN, b"*.ess");
        let prefix = e.mem.alloc(16);
        e.mem.set_cstr(prefix, b"base\\");
        let folder = e.mem.alloc(16);
        e.mem.set_cstr(folder, b"Saves\\");
        constant(&mut e, PATH_PREFIX, prefix);
        constant(&mut e, PATH_OBJECT_GET, folder);
        e.register(IDENTITY, |_, a| returns(a[0]));
        e.register(LSTRCPY_IMPORT, |e, a| {
            let source = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(LSTRCAT_IMPORT, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[1]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        // Three files: the second is empty.
        file_system(
            &mut e,
            &[
                ("one.ess", 1, 1, 10),
                ("empty.ess", 2, 2, 0),
                ("two.ess", 3, 3, 5),
            ],
        );
        let constructed: Rc<RefCell<Vec<(u32, String)>>> = Rc::new(RefCell::new(vec![]));
        let sink = constructed.clone();
        e.register_double(BSFILE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((a[0], text(e, a[1])));
            Ret::default()
        });
        quiet(&mut e, &[LIST_INSERT]);
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        fn_0085f900(&mut e, game_object);
        let list = e.get(game_object, TESSaveLoadGame::m_pSaveGameList);
        assert!(!list.is_null());
        let made = constructed.borrow().clone();
        assert_eq!(
            made.iter().map(|m| m.1.clone()).collect::<Vec<_>>(),
            vec!["base\\Saves\\one.ess", "base\\Saves\\two.ess"]
        );
        // Every file is a 0x164-byte `SaveGameFile` inserted with the
        // comparator.
        let inserted = calls_to(&e, LIST_INSERT);
        assert_eq!(
            inserted,
            vec![
                vec![list.addr(), made[0].0, SAVE_GAME_FILE_COMPARATOR],
                vec![list.addr(), made[1].0, SAVE_GAME_FILE_COMPARATOR]
            ]
        );
        assert_eq!(e.mem.u32(made[0].0), SAVE_GAME_FILE_VTABLE);
        assert_eq!(calls_to(&e, FIND_CLOSE_IMPORT).len(), 1);
        // The folder search was for "base\Saves\*.ess".
        let searched = calls_to(&e, FIND_FIRST_FILE_IMPORT);
        assert_eq!(text(&e, searched[0][0]), "base\\Saves\\*.ess");
        // An existing list is destroyed first; a search that finds nothing
        // leaves an empty list.
        file_system(&mut e, &[]);
        quiet(&mut e, &[LIST_REMOVE_ALL, LIST_SCALAR_DELETE]);
        fn_0085f900(&mut e, game_object);
        assert_eq!(calls_to(&e, LIST_REMOVE_ALL), vec![vec![list.addr()]]);
        assert!(!e
            .get(game_object, TESSaveLoadGame::m_pSaveGameList)
            .is_null());
        assert_eq!(calls_to(&e, FIND_CLOSE_IMPORT).len(), 1);
    }

    #[test]
    fn saves_are_ordered_newest_first_by_last_write_time() {
        let mut e = game();
        file_system(
            &mut e,
            &[
                ("a.ess", 100, 5, 1),
                ("b.ess", 200, 5, 1),
                ("c.ess", 50, 6, 1),
                ("d.ess", 50, 4, 1),
            ],
        );
        let mut names = vec![];
        for n in ["a.ess", "b.ess", "c.ess", "d.ess", "gone.ess"] {
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, n.as_bytes());
            names.push(name);
        }
        let files: Vec<Ptr<SaveGameFile>> = names.iter().map(|&n| save_file(&mut e, n)).collect();
        let compare = |e: &mut Engine, x: usize, y: usize| fn_0085fc90(e, files[x], files[y]);
        // The high dword decides first, then the low one.
        assert_eq!(compare(&mut e, 2, 0), -1);
        assert_eq!(compare(&mut e, 0, 2), 1);
        assert_eq!(compare(&mut e, 1, 0), -1);
        assert_eq!(compare(&mut e, 0, 1), 1);
        assert_eq!(compare(&mut e, 0, 3), -1);
        // Equal times (and a file the system does not know: time 0).
        assert_eq!(compare(&mut e, 0, 0), 0);
        assert_eq!(compare(&mut e, 4, 4), 0);
        assert_eq!(compare(&mut e, 4, 0), 1);
        // The time of each file is remembered after the first look.
        assert!(fn_0085feb0(&mut e, files[0]));
        let lookups = calls_to(&e, FIND_FIRST_FILE_IMPORT).len();
        compare(&mut e, 0, 1);
        assert_eq!(calls_to(&e, FIND_FIRST_FILE_IMPORT).len(), lookups);
        assert_eq!(e.get(files[2], SaveGameFile::FileTimeHigh), 6);
    }

    // ---- 0085fed0 to 00860e20: describing a save ----

    /// Text objects (`PATH_OBJECT_GET` reads the string at `+4`), the path
    /// helpers and the string functions the save-name code uses.
    fn name_functions(e: &mut Engine) {
        map_globals(e);
        text_functions(e);
        for page in [
            0x011d_2000u32,
            0x011d_3000,
            0x011d_4000,
            0x011c_3000,
            0x0104_f000,
            0x0103_4000,
            0x0106_3000,
            0x0101_9000,
        ] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        e.register(PATH_OBJECT_GET, |e, a| {
            returns(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 4) })
        });
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            returns(
                bytes
                    .iter()
                    .rposition(|&b| b == a[1] as u8)
                    .map_or(0, |i| a[0] + i as u32),
            )
        });
        e.register(STRING_FIND, |e, a| {
            let (text, pattern) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let found = text
                .windows(pattern.len().max(1))
                .position(|window| window == pattern.as_slice());
            returns(found.map_or(0, |at| a[0] + at as u32))
        });
        e.register(STRNICMP, |e, a| {
            let n = a[2] as usize;
            let cut = |v: Vec<u8>| {
                v.iter()
                    .take(n)
                    .map(u8::to_ascii_lowercase)
                    .collect::<Vec<u8>>()
            };
            let (left, right) = (cut(e.mem.cstr(a[0])), cut(e.mem.cstr(a[1])));
            returns(left.cmp(&right) as i32 as u32)
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
        e.register(STRING_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.mem.set_cstr(SAVE_NAME_PREFIX, b"Save ");
        e.mem.set_cstr(ESS_EXTENSION, b".ess");
        e.mem.set_cstr(TEXT_PLAYING_TIME, b"Playing Time");
        e.mem.set_cstr(TEXT_DASH, b"-");
        e.mem.set_cstr(TEXT_COMMA, b",");
        e.mem.set_cstr(TEXT_SEPARATOR, b" - ");
        e.mem.set_cstr(QUICKSAVE_NAME, b"quicksave");
        e.mem.set_cstr(AUTOSAVE_NAME, b"autosave");
        e.mem.set_cstr(FORMAT_TWO_STRINGS, b"%s%s");
        e.mem.set_cstr(FORMAT_LABEL_NUMBER, b"%s %i");
        e.mem.set_cstr(FORMAT_PLAY_TIME, b"%02i:%02i:%02i");
        e.mem.set_cstr(FORMAT_DATE_TIME, b"%d/%d/%02d %02d:%02d");
    }

    /// A text object at `address` whose string is `value`.
    fn text_object(e: &mut Engine, address: u32, value: &[u8]) {
        let string = e.mem.alloc(value.len() as u32 + 1);
        e.mem.set_cstr(string, value);
        e.mem.set_u32(address + 4, string);
    }

    /// A file object whose virtual slot `0x18` gives `name`.
    fn named_file(e: &mut Engine, name: &str) -> Ptr {
        let string = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(string, name.as_bytes());
        let slot = 0x0392_0000 + string;
        constant(e, slot, string);
        stub(e, 0x0392_0020);
        stub(e, BSFILE_CLOSE);
        let file = object_with_slots(e, &[(FILE_GET_NAME, slot), (FILE_OPEN, 0x0392_0020)]);
        Ptr::new(file)
    }

    const SAVE_FILE_NAME: &str =
        "base\\Saves\\Save 12 - Alex - Goodsprings, Level 3, Playing Time 01.02.03.ess";

    #[test]
    fn a_save_name_gives_its_number_label_and_time() {
        let mut e = game();
        name_functions(&mut e);
        text_object(&mut e, SAVE_NUMBER_PREFIX_TEXT, b"Lvl");
        let file = named_file(&mut e, SAVE_FILE_NAME);
        let (number, label, time) = (e.mem.alloc(4), e.mem.alloc(0x100), e.mem.alloc(0x100));
        let this = singleton_of(&e);
        assert!(fn_008607e0(
            &mut e,
            this,
            file,
            Ptr::new(number),
            Ptr::new(label),
            Ptr::new(time)
        ));
        assert_eq!(e.mem.u32(number), 12);
        // The text between the second last comma and the last one, from the
        // first space after the comma, after the game's word.
        assert_eq!(text(&e, label), "Lvl 3");
        assert_eq!(text(&e, time), "01:02:03");
        // Any of the outputs may be left out.
        assert!(fn_008607e0(
            &mut e,
            this,
            file,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL
        ));
    }

    #[test]
    fn a_name_that_is_not_the_game_form_gives_nothing() {
        let mut e = game();
        name_functions(&mut e);
        text_object(&mut e, SAVE_NUMBER_PREFIX_TEXT, b"Lvl");
        let this = singleton_of(&e);
        for name in [
            "base\\Saves\\quicksave.ess",
            "base\\Saves\\Load 12 - Alex - Goodsprings, Level 3, Playing Time 01.02.03.ess",
            "base\\Saves\\Save 12 Alex Playing Time 01.02.03.ess",
            "base\\Saves\\Save 12 - Alex Playing Time 01.02.03.ess",
        ] {
            let file = named_file(&mut e, name);
            let label = e.mem.alloc(0x100);
            assert!(
                !fn_008607e0(&mut e, this, file, Ptr::NULL, Ptr::new(label), Ptr::NULL),
                "{name}"
            );
        }
        // Without the two commas the label and time cannot be read, but the
        // number can.
        let file = named_file(
            &mut e,
            "base\\Saves\\Save 5 - Alex Playing Time 01.02.03.ess",
        );
        let number = e.mem.alloc(4);
        assert!(fn_008607e0(
            &mut e,
            this,
            file,
            Ptr::new(number),
            Ptr::NULL,
            Ptr::NULL
        ));
        assert_eq!(e.mem.u32(number), 5);
    }

    #[test]
    fn the_description_of_a_save_file_names_quick_and_auto_saves() {
        let mut e = game();
        name_functions(&mut e);
        text_object(&mut e, QUICKSAVE_DISPLAY_TEXT, b"Quick");
        text_object(&mut e, AUTOSAVE_DISPLAY_TEXT, b"Auto");
        let this = singleton_of(&e);
        // The header read finds nothing, which also clears the label (the
        // displayed name is the kind's text, " - " and that empty label).
        e.register(OPENED_FILE_TEST, |_, _| returns(0));
        let name = e.mem.alloc(0x100);
        let label = e.mem.alloc(0x100);
        e.mem.set_cstr(label, b"Save 4");
        for (file_name, expected) in [
            ("base\\Saves\\quicksave.ess", "Quick - "),
            ("base\\Saves\\autosave.ess", "Auto - "),
        ] {
            e.mem.set_cstr(name, b"old");
            let file = named_file(&mut e, file_name);
            fn_0085fed0(
                &mut e,
                this,
                file,
                Ptr::NULL,
                Ptr::new(name),
                Ptr::new(label),
                Ptr::NULL,
            );
            assert_eq!(text(&e, name), expected);
        }
        // Another name that is not a "Save " name: its file name without
        // the extension, at most 18 characters.
        let file = named_file(&mut e, "base\\Saves\\My Favourite Hideout Spot.ess");
        fn_0085fed0(
            &mut e,
            this,
            file,
            Ptr::NULL,
            Ptr::new(name),
            Ptr::new(label),
            Ptr::NULL,
        );
        assert_eq!(text(&e, name), "My Favourite Hideo");
        // A "Save " name leaves the (cleared) name empty.
        let file = named_file(&mut e, "base\\Saves\\Save 4 - x.ess");
        fn_0085fed0(
            &mut e,
            this,
            file,
            Ptr::NULL,
            Ptr::new(name),
            Ptr::new(label),
            Ptr::NULL,
        );
        assert_eq!(text(&e, name), "");
        // No name wanted: only the header is read.
        fn_0085fed0(
            &mut e,
            this,
            file,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::new(label),
            Ptr::NULL,
        );
    }

    #[test]
    fn the_description_of_a_save_in_the_game_form_stops_there() {
        let mut e = game();
        name_functions(&mut e);
        text_object(&mut e, SAVE_NUMBER_PREFIX_TEXT, b"Lvl");
        let this = singleton_of(&e);
        let file = named_file(&mut e, SAVE_FILE_NAME);
        let (name, number, label, time) = (
            e.mem.alloc(0x100),
            e.mem.alloc(4),
            e.mem.alloc(0x100),
            e.mem.alloc(0x100),
        );
        e.mem.set_cstr(name, b"old");
        // No header is read: `OPENED_FILE_TEST` is not even asked.
        fn_0085fed0(
            &mut e,
            this,
            file,
            Ptr::new(number),
            Ptr::new(name),
            Ptr::new(label),
            Ptr::new(time),
        );
        assert!(calls_to(&e, OPENED_FILE_TEST).is_empty());
        assert_eq!(text(&e, name), "");
        assert_eq!(text(&e, time), "01:02:03");
    }

    // ---- the header of a save ----

    /// A file object that reads from `data` (`FILE_READ`), seeks (slot
    /// `0x14`, relative when the mode is the second seek mode) and has a
    /// name (slot `0x18`). Returns the file and the read position.
    fn memory_file(e: &mut Engine, data: Vec<u8>) -> (Ptr, Rc<RefCell<usize>>) {
        let position = Rc::new(RefCell::new(0usize));
        let cursor = position.clone();
        e.register_double(FILE_READ, move |e, a| {
            let mut at = cursor.borrow_mut();
            let end = (*at + a[2] as usize).min(data.len());
            let chunk = &data[*at..end];
            e.mem.write(a[1], chunk);
            *at = end;
            returns(chunk.len() as u32)
        });
        let cursor = position.clone();
        e.register_double(0x0393_0014, move |_, a| {
            let mut at = cursor.borrow_mut();
            if a[2] == 0 {
                *at = a[1] as usize;
            } else {
                *at += a[1] as usize;
            }
            Ret::default()
        });
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"x.ess");
        constant(e, 0x0393_0018, name);
        let file = object_with_slots(e, &[(FILE_SEEK, 0x0393_0014), (FILE_GET_NAME, 0x0393_0018)]);
        (Ptr::new(file), position)
    }

    /// The globals and doubles `LoadHeader` uses: the signature, the seek
    /// modes, the version.
    fn header_functions(e: &mut Engine, version: u8) {
        name_functions(e);
        for page in [0x011a_2000u32, 0x010a_2000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let signature = e.mem.alloc(16);
        e.mem.set_cstr(signature, b"FO3SAVEGAME");
        e.set_global(SAVE_SIGNATURE_POINTER, signature);
        e.set_global(SEEK_MODE, 0u32);
        e.set_global(SEEK_MODE_CURRENT, 1u32);
        e.mem.set_cstr(CON_PREFIX, b"CON ");
        e.register_double(CURRENT_VERSION, move |_, _| returns(version as u32));
        quiet(e, &[END_FORM_PROCESSING, SET_LOADING_STATE]);
        e.mem
            .set_cstr(MSG_SAVE_VERSION_WARNING, b"version %i.%02i of %i.%02i");
    }

    #[test]
    fn load_header_reads_the_signature_the_versions_and_the_size() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let mut bytes = b"FO3SAVEGAME".to_vec();
        bytes.extend([0, 0x7D]); // major, minor
        bytes.extend([0xEE; 0x14]); // time and version, skipped
        bytes.extend(1234u32.to_le_bytes());
        let (file, position) = memory_file(&mut e, bytes);
        let this = singleton_of(&e);
        let size = tes_save_load_game_load_header(&mut e, this, file, false);
        assert_eq!(size, 1234);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMajorVersion), 0);
        assert_eq!(e.get(this, TESSaveLoadGame::m_cMinorVersion), 0x7D);
        assert_eq!(e.get(this, TESSaveLoadGame::m_iFileStartPosition), 0);
        // The skip of the time was a relative seek of 0x14; all was read.
        assert_eq!(*position.borrow(), 11 + 2 + 0x14 + 4);
        assert_eq!(calls_to(&e, END_FORM_PROCESSING).len(), 1);
        assert!(calls_to(&e, SHOW_MESSAGE_BOX).is_empty());
    }

    #[test]
    fn load_header_can_read_the_original_save_time() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let mut bytes = b"FO3SAVEGAME".to_vec();
        bytes.extend([0, 0x7D]);
        bytes.extend((1u8..=16).collect::<Vec<u8>>());
        bytes.extend(0xCAFEu32.to_le_bytes());
        bytes.extend(77u32.to_le_bytes());
        let (file, _) = memory_file(&mut e, bytes);
        let this = singleton_of(&e);
        assert_eq!(tes_save_load_game_load_header(&mut e, this, file, true), 77);
        assert_eq!(
            e.mem.bytes(this.addr() + 0x98, 16),
            (1u8..=16).collect::<Vec<u8>>()
        );
        assert_eq!(e.get(this, TESSaveLoadGame::m_iOriginalSaveVersion), 0xCAFE);
        // An old format has no time: 0xFF bytes and the version -1.
        let mut e = game();
        header_functions(&mut e, 0x40);
        let mut bytes = b"FO3SAVEGAME".to_vec();
        bytes.extend([0, 0x7D]);
        bytes.extend(78u32.to_le_bytes());
        let (file, _) = memory_file(&mut e, bytes);
        let this = singleton_of(&e);
        assert_eq!(tes_save_load_game_load_header(&mut e, this, file, true), 78);
        assert_eq!(e.mem.bytes(this.addr() + 0x98, 16), vec![0xFF; 16]);
        assert_eq!(
            e.get(this, TESSaveLoadGame::m_iOriginalSaveVersion),
            0xFFFF_FFFF
        );
    }

    #[test]
    fn load_header_refuses_another_signature_or_an_empty_file() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let (file, _) = memory_file(&mut e, b"NOTASAVEFILE".to_vec());
        let this = singleton_of(&e);
        assert_eq!(tes_save_load_game_load_header(&mut e, this, file, false), 0);
        let (file, _) = memory_file(&mut e, vec![]);
        assert_eq!(tes_save_load_game_load_header(&mut e, this, file, false), 0);
    }

    #[test]
    fn load_header_skips_the_console_prefix() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        // "CON " then padding up to 0xD000, then the signature again.
        let mut bytes = b"CON ".to_vec();
        bytes.resize(0xD000, 0);
        bytes.extend(b"FO3SAVEGAME");
        bytes.extend([0, 0x7D]);
        bytes.extend([0; 0x14]);
        bytes.extend(55u32.to_le_bytes());
        let (file, _) = memory_file(&mut e, bytes);
        let this = singleton_of(&e);
        assert_eq!(
            tes_save_load_game_load_header(&mut e, this, file, false),
            55
        );
        assert_eq!(e.get(this, TESSaveLoadGame::m_iFileStartPosition), 0xD000);
        // An empty first byte means the file starts at 0.
        let mut bytes = vec![0u8; 0x16];
        bytes.extend(56u32.to_le_bytes());
        let (file, _) = memory_file(&mut e, bytes);
        assert_eq!(
            tes_save_load_game_load_header(&mut e, this, file, false),
            56
        );
    }

    #[test]
    fn load_header_asks_before_loading_a_save_of_another_version() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let this = singleton_of(&e);
        let build = |major: u8, minor: u8| {
            let mut bytes = b"FO3SAVEGAME".to_vec();
            bytes.extend([major, minor]);
            bytes.extend([0; 0x14]);
            bytes.extend(9u32.to_le_bytes());
            bytes
        };
        text_object(&mut e, MESSAGE_BOX_TEXT_A, b"Yes");
        text_object(&mut e, MESSAGE_BOX_TEXT_B, b"No");
        let message: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        let sink = message.clone();
        e.register_double(SPRINTF, move |e, a| {
            let format = e.mem.cstr(a[1]);
            let text = format_text(e, &format, &a[2..]);
            e.mem.set_cstr(a[0], &text);
            sink.borrow_mut().push(String::from_utf8(text).unwrap());
            Ret::default()
        });
        constant(&mut e, SHOW_MESSAGE_BOX, 2);
        // A major version other than 0: asks; the answer 2 refuses.
        let (file, _) = memory_file(&mut e, build(1, 0x7D));
        assert_eq!(
            tes_save_load_game_load_header(&mut e, this, file, false),
            0xFFFF_FFFF
        );
        assert_eq!(
            *message.borrow(),
            vec!["version 1.125 of 0.125".to_string()]
        );
        let asked = calls_to(&e, SHOW_MESSAGE_BOX);
        assert_eq!(asked[0][1..3], [1, 5]);
        assert_eq!(calls_to(&e, SET_LOADING_STATE).len(), 2);
        // The answer 1 goes on.
        constant(&mut e, SHOW_MESSAGE_BOX, 1);
        let (file, _) = memory_file(&mut e, build(1, 0x7D));
        assert_eq!(tes_save_load_game_load_header(&mut e, this, file, false), 9);
        // An old minor version asks only when the time is wanted.
        let before = calls_to(&e, SHOW_MESSAGE_BOX).len();
        let (file, _) = memory_file(&mut e, build(0, 0x10));
        tes_save_load_game_load_header(&mut e, this, file, false);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_BOX).len(), before);
        let (file, _) = memory_file(&mut e, build(0, 0x10));
        tes_save_load_game_load_header(&mut e, this, file, true);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_BOX).len(), before + 1);
        // The current minor version and the ones from 0x13 on do not.
        let (file, _) = memory_file(&mut e, build(0, 0x13));
        tes_save_load_game_load_header(&mut e, this, file, true);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_BOX).len(), before + 1);
    }

    // ---- 008600b0, 00860390: the description in a save's header ----

    /// The bytes after the header's size field, for format 0x7D: the value,
    /// the player name, the save number, the location, the level, the play
    /// time, the save time and a 2 by 3 pixel screenshot.
    fn description_bytes(with_screenshot: bool) -> Vec<u8> {
        let mut body = vec![];
        body.extend(0xAABBu32.to_le_bytes());
        body.push(5);
        body.extend(b"Alex\0");
        body.extend(7u16.to_le_bytes());
        body.push(6);
        body.extend(b"Vault\0");
        body.extend(12.75f32.to_le_bytes());
        body.extend(3_723_000u32.to_le_bytes());
        // A `_SYSTEMTIME`: 2010-10-21 (Thursday) 14:05:00.
        for field in [2010u16, 10, 4, 21, 14, 5, 0, 0] {
            body.extend(field.to_le_bytes());
        }
        if with_screenshot {
            body.extend(26u32.to_le_bytes());
            body.extend(2u32.to_le_bytes());
            body.extend(3u32.to_le_bytes());
            body.extend((1u8..=18).collect::<Vec<u8>>());
        } else {
            body.extend(0u32.to_le_bytes());
        }
        body
    }

    /// Doubles for the texture the screenshot becomes; the log of what the
    /// texture is made of.
    fn texture_functions(e: &mut Engine) -> Rc<RefCell<Vec<String>>> {
        e.map(0x011a_a000, 0x1000);
        for word in 0..0x11u32 {
            e.mem
                .set_u32(PIXEL_FORMAT_TEMPLATE + 4 * word, 0x100 + word);
        }
        let events: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        e.register(PIXEL_DATA_ALLOCATE, |e, a| returns(e.mem.alloc(a[0])));
        let sink = events.clone();
        e.register_double(PIXEL_DATA_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push(format!(
                "pixel data {} {} format {:x}..{:x} {} {}",
                a[1],
                a[2],
                e.mem.u32(a[3]),
                e.mem.u32(a[3] + 0x40),
                a[4],
                a[5]
            ));
            returns(a[0])
        });
        let pixels = e.mem.alloc(32);
        e.register_double(IMAGE_PIXELS, move |_, _| returns(pixels));
        e.register(FORMAT_PREFS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 6);
            e.mem.set_u32(a[0] + 4, 3);
            e.mem.set_u32(a[0] + 8, 2);
            returns(a[0])
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        constant(e, FIXED_STRING_TEXT, 0x4444);
        let sink = events.clone();
        e.register_double(CREATE_TEXTURE_FROM_PIXELS, move |e, a| {
            sink.borrow_mut().push(format!(
                "texture {} prefs {} {} {}",
                a[1],
                e.mem.u32(a[2]),
                e.mem.u32(a[2] + 4),
                e.mem.u32(a[2] + 8)
            ));
            returns(0x7777)
        });
        let sink = events.clone();
        e.register_double(REFERENCE_RELEASE, move |_, a| {
            sink.borrow_mut().push(format!("release {:x}", a[0]));
            Ret::default()
        });
        quiet(e, &[FIXED_STRING_DESTRUCT]);
        events
    }

    #[test]
    fn the_old_header_description_reads_every_field_and_the_screenshot() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let events = texture_functions(&mut e);
        let this = singleton_of(&e);
        text_object(&mut e, DEFAULT_LOCATION_TEXT, b"Nowhere");
        let body = description_bytes(true);
        let size = body.len() as u32;
        let (file, position) = memory_file(&mut e, body.clone());
        let out = e.mem.alloc(0x100);
        let (value, name, number, location, level, date, play_time, height) = (
            out,
            out + 4,
            out + 0x14,
            out + 0x18,
            out + 0x78,
            out + 0x7C,
            out + 0x8C,
            out + 0x90,
        );
        let texture = tes_save_load_game_read_save_game_data_old(
            &mut e,
            this,
            file,
            size,
            Ptr::new(value),
            Ptr::new(name),
            Ptr::new(number),
            Ptr::new(location),
            Ptr::new(level),
            Ptr::new(date),
            Ptr::new(play_time),
            Ptr::new(height),
        );
        assert_eq!(e.mem.u32(value), 0xAABB);
        assert_eq!(text(&e, name), "Alex");
        assert_eq!(e.mem.u16(number), 7);
        assert_eq!(text(&e, location), "Vault");
        assert_eq!(e.mem.f32(level), 12.75);
        assert_eq!(e.mem.u32(play_time), 3_723_000);
        assert_eq!(
            e.mem.bytes(date, 16),
            body[4 + 6 + 2 + 7 + 8..][..16].to_vec()
        );
        assert_eq!(e.mem.u32(height), 3);
        // The texture is made from 2 by 3 pixels (the format copied from the
        // template), the preferences are (2, 0, 0).
        assert_eq!(texture, 0x7777);
        assert_eq!(
            *events.borrow(),
            vec![
                "pixel data 2 2 format 100..110 1 1".to_string(),
                "texture 17476 prefs 2 0 0".to_string(),
                "release 7777".to_string()
            ]
        );
        let pixels = calls_to(&e, IMAGE_PIXELS);
        assert_eq!(pixels.len(), 1);
        // All of the file was read; the buffer is freed and cleared.
        assert_eq!(*position.borrow(), body.len());
        assert!(e.get(this, TESSaveLoadGame::m_pBuffer).is_null());
    }

    #[test]
    fn the_old_header_description_skips_what_is_not_wanted() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        texture_functions(&mut e);
        let this = singleton_of(&e);
        let body = description_bytes(true);
        let size = body.len() as u32;
        let (file, position) = memory_file(&mut e, body.clone());
        let texture = tes_save_load_game_read_save_game_data_old(
            &mut e,
            this,
            file,
            size,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
        );
        // No picture wanted: no texture, nothing built.
        assert_eq!(texture, 0);
        assert!(calls_to(&e, PIXEL_DATA_ALLOCATE).is_empty());
        assert_eq!(*position.borrow(), body.len());
    }

    #[test]
    fn the_old_header_description_of_an_old_format_uses_defaults() {
        let mut e = game();
        header_functions(&mut e, 0x20);
        texture_functions(&mut e);
        let this = singleton_of(&e);
        text_object(&mut e, DEFAULT_LOCATION_TEXT, b"Nowhere");
        // Format 0x20: no value, no save number, no save time; the location
        // has no text and the file system gives the time.
        let mut body = vec![];
        body.push(5);
        body.extend(b"Alex\0");
        body.push(0);
        body.extend(1.5f32.to_le_bytes());
        body.extend(99u32.to_le_bytes());
        body.extend(0u32.to_le_bytes());
        let size = body.len() as u32;
        let (file, _) = memory_file(&mut e, body);
        let time = e.mem.alloc(16);
        for word in 0..4 {
            e.mem.set_u32(time + 4 * word, 0x10 + word);
        }
        constant(&mut e, FILE_GET_TIME, time);
        let out = e.mem.alloc(0x100);
        e.mem.set_u32(out, 0x1234);
        e.mem.set_u16(out + 0x18, 0x55);
        let (value, name, number, location, date) =
            (out, out + 8, out + 0x18, out + 0x20, out + 0x80);
        tes_save_load_game_read_save_game_data_old(
            &mut e,
            this,
            file,
            size,
            Ptr::new(value),
            Ptr::new(name),
            Ptr::new(number),
            Ptr::new(location),
            Ptr::NULL,
            Ptr::new(date),
            Ptr::NULL,
            Ptr::NULL,
        );
        assert_eq!(e.mem.u32(value), 0);
        assert_eq!(e.mem.u16(number), 0);
        assert_eq!(text(&e, location), "Nowhere");
        assert_eq!(e.mem.bytes(date, 16), e.mem.bytes(time, 16));
        assert_eq!(calls_to(&e, FILE_GET_TIME)[0][0], file.addr());
    }

    /// The whole header of a save for format 0x7D: the signature, the
    /// versions, 0x14 skipped bytes, the size and the description.
    fn whole_header(with_screenshot: bool) -> Vec<u8> {
        let body = description_bytes(with_screenshot);
        let mut bytes = b"FO3SAVEGAME".to_vec();
        bytes.extend([0, 0x7D]);
        bytes.extend([0xEE; 0x14]);
        bytes.extend((body.len() as u32).to_le_bytes());
        bytes.extend(body);
        bytes
    }

    #[test]
    fn a_save_is_described_by_its_header() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        texture_functions(&mut e);
        let this = singleton_of(&e);
        text_object(&mut e, SAVE_NUMBER_PREFIX_TEXT, b"Save");
        text_object(&mut e, SAVE_LEVEL_PREFIX_TEXT, b"Level");
        constant(&mut e, OPENED_FILE_TEST, 1);
        let (file, position) = memory_file(&mut e, whole_header(true));
        let out = e.mem.alloc(0x400);
        let mut texts = vec![];
        for i in 0..6 {
            let buffer = out + 0x10 + 0x40 * i;
            e.mem.set_cstr(buffer, b"garbage");
            texts.push(buffer);
        }
        let ptr = |i: usize| Ptr::new(texts[i]);
        // name, save text, location, level text, date text, time text
        let result = fn_008600b0(
            &mut e,
            this,
            file,
            Ptr::new(out),
            ptr(0),
            ptr(1),
            ptr(2),
            ptr(3),
            ptr(4),
            ptr(5),
            Ptr::new(out + 4),
            1,
        );
        assert_eq!(result, 0x7777);
        assert_eq!(e.mem.u32(out), 0xAABB);
        assert_eq!(text(&e, texts[0]), "Alex");
        assert_eq!(text(&e, texts[1]), "Save 7");
        assert_eq!(text(&e, texts[2]), "Vault");
        assert_eq!(text(&e, texts[3]), "Level 12");
        assert_eq!(text(&e, texts[4]), "10/21/2010 14:05");
        assert_eq!(text(&e, texts[5]), "01:02:03");
        assert_eq!(e.mem.u32(out + 4), 3);
        // The file is put back at its start and left open.
        assert_eq!(*position.borrow(), 0);
        assert!(calls_to(&e, BSFILE_CLOSE).is_empty());
    }

    #[test]
    fn a_file_without_a_readable_header_clears_every_description() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let this = singleton_of(&e);
        let out = e.mem.alloc(0x400);
        let mut texts = vec![];
        for i in 0..6 {
            let buffer = out + 0x10 + 0x40 * i;
            e.mem.set_cstr(buffer, b"garbage");
            texts.push(buffer);
        }
        e.mem.set_u32(out, 5);
        e.mem.set_u32(out + 4, 6);
        let all = |e: &mut Engine, file: Ptr| {
            fn_008600b0(
                e,
                this,
                file,
                Ptr::new(out),
                Ptr::new(texts[0]),
                Ptr::new(texts[1]),
                Ptr::new(texts[2]),
                Ptr::new(texts[3]),
                Ptr::new(texts[4]),
                Ptr::new(texts[5]),
                Ptr::new(out + 4),
                1,
            )
        };
        // The file is not a save at all.
        constant(&mut e, OPENED_FILE_TEST, 0);
        let (file, _) = memory_file(&mut e, whole_header(true));
        assert_eq!(all(&mut e, file), 0);
        for text_at in &texts {
            assert_eq!(text(&e, *text_at), "");
        }
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
        // The header has another signature: the same.
        constant(&mut e, OPENED_FILE_TEST, 1);
        e.mem.set_cstr(texts[0], b"garbage");
        let (file, _) = memory_file(&mut e, b"NOTASAVEFILE".to_vec());
        assert_eq!(all(&mut e, file), 0);
        assert_eq!(text(&e, texts[0]), "");
    }

    #[test]
    fn a_save_that_is_opened_and_closed_by_the_description() {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        let this = singleton_of(&e);
        // `file_is_open` zero: the file is opened (mode 2: its `Open` slot)
        // and closed (`BSFile::Close`) around the read, even when it fails.
        name_functions(&mut e);
        let file = named_file(&mut e, "x.ess").addr();
        constant(&mut e, OPENED_FILE_TEST, 0);
        let result = fn_008600b0(
            &mut e,
            this,
            Ptr::new(file),
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::NULL,
            0,
        );
        assert_eq!(result, 0);
        assert_eq!(calls_to(&e, 0x0392_0020), vec![vec![file, 0, 0]]);
        assert_eq!(calls_to(&e, BSFILE_CLOSE), vec![vec![file]]);
    }

    // ---- 00860ae0, 00860e00, 00861610, 00861640 ----

    #[test]
    fn a_new_save_is_named_from_the_player_and_made_unique() {
        let mut e = game();
        name_functions(&mut e);
        e.mem.set_cstr(
            FORMAT_SAVE_NAME,
            b"Save %i - %s - %s, Level %i, Playing Time %02i.%02i.%02i",
        );
        e.mem.set_cstr(FORMAT_NUMBER_SUFFIX, b" #%d");
        e.mem.set_cstr(INVALID_FILE_NAME_CHARACTERS, b"\\/:*<>?|\"");
        e.mem.set_cstr(FORMAT_SAVE_PATH, b"%s%s%s.ess");
        let prefix = e.mem.alloc(16);
        e.mem.set_cstr(prefix, b"base\\");
        let folder = e.mem.alloc(16);
        e.mem.set_cstr(folder, b"Saves\\");
        constant(&mut e, PATH_PREFIX, prefix);
        e.register(IDENTITY, |_, a| returns(a[0]));
        e.mem.set_u32(PATH_OBJECT + 4, folder);
        e.register(STRPBRK, |e, a| {
            let set = e.mem.cstr(a[1]);
            let text = e.mem.cstr(a[0]);
            returns(
                text.iter()
                    .position(|c| set.contains(c))
                    .map_or(0, |at| a[0] + at as u32),
            )
        });
        let player = 0x0099_0000u32;
        e.set_global(PLAYER, player);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Al/ex");
        constant(&mut e, REFERENCE_GET_NAME, name);
        quiet(
            &mut e,
            &[
                STRING_OBJECT_CONSTRUCT,
                PLAYER_GET_LOCATION_TEXT,
                STRING_OBJECT_DESTRUCT,
            ],
        );
        // The player's location has no text: his cell's description is used.
        constant(&mut e, STRING_OBJECT_TEXT, 0);
        let description = e.mem.alloc(16);
        e.mem.set_cstr(description, b"Va\"ult");
        constant(&mut e, 0x0394_0130, description);
        let cell = object_with_slots(&mut e, &[(FORM_GET_DESCRIPTION, 0x0394_0130)]);
        constant(&mut e, REF_GET_PARENT_CELL, cell);
        constant(&mut e, PLAYER_PLAY_TIME, 3_723_000);
        constant(&mut e, PLAYER_LEVEL, 5);
        constant(&mut e, NEXT_SAVE_NUMBER, 3);
        // The first name is taken, the second is free.
        let taken = Rc::new(RefCell::new(VecDeque::from(vec![1u32, 0])));
        e.register_double(FILE_EXISTS, move |_, _| {
            returns(taken.borrow_mut().pop_front().unwrap())
        });
        let out = e.mem.alloc(0x104);
        let this = singleton_of(&e);
        fn_00860ae0(&mut e, this, Ptr::new(out));
        assert_eq!(
            text(&e, out),
            "Save 3 - Al ex - Va'ult, Level 5, Playing Time 01.02.03"
        );
        // Asked twice; the path is the save folder, the name and ".ess".
        let asked = calls_to(&e, FILE_EXISTS);
        assert_eq!(asked.len(), 2);
        assert_eq!(
            text(&e, asked[1][0]),
            "base\\Saves\\Save 3 - Al ex - Va'ult, Level 5, Playing Time 01.02.03.ess"
        );
        assert_eq!(asked[1][1..], [0, 0, 0xFFFF_FFFF]);
        assert_eq!(calls_to(&e, NEXT_SAVE_NUMBER).len(), 2);
        // The suffix is built for the retry (" #2") but never used.
        assert_eq!(calls_to(&e, STRING_OBJECT_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_new_save_name_needs_somewhere_to_go() {
        let mut e = game();
        let this = singleton_of(&e);
        fn_00860ae0(&mut e, this, Ptr::NULL);
        assert!(calls_to(&e, STRING_OBJECT_CONSTRUCT).is_empty());
    }

    #[test]
    fn strpbrk_is_the_runtime_library_function() {
        let mut e = game();
        e.register(STRPBRK, |_, a| returns(a[0] + 3));
        assert_eq!(
            fn_00860e00(&mut e, Ptr::new(0x5000), Ptr::new(0x6000)).addr(),
            0x5003
        );
        assert_eq!(calls_to(&e, STRPBRK), vec![vec![0x5000, 0x6000]]);
    }

    #[test]
    fn the_size_of_an_image_level_is_the_difference_of_two_offsets() {
        let mut e = game();
        let image = e.mem.alloc(0x80);
        let table = e.mem.alloc(16);
        for (i, v) in [0u32, 100, 150, 175].iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *v);
        }
        e.mem.set_u32(image + 0x5C, table);
        assert_eq!(fn_00861610(&mut e, Ptr::new(image), 0, 0), 100);
        assert_eq!(fn_00861610(&mut e, Ptr::new(image), 2, 77), 25);
    }

    #[test]
    fn a_created_exterior_cell_is_found_by_its_coordinates_and_forgotten() {
        let mut e = game();
        let game_object: Ptr<TESSaveLoadGame> = e.new_object();
        // Without the map: nothing.
        assert_eq!(
            tes_save_load_game_get_created_exterior_cell_form_id(&mut e, game_object, 0x3C, 1, 2),
            0
        );
        e.set(
            game_object,
            TESSaveLoadGame::m_pCreatedExteriorCells,
            Ptr::new(0x5000),
        );
        let records: Vec<u32> = [(0xAAA, 1, 2), (0xBBB, 3, 4)]
            .iter()
            .map(|&(id, x, y)| {
                let record = e.mem.alloc(12);
                e.mem.set_u32(record, id);
                e.mem.set_i32(record + 4, x);
                e.mem.set_i32(record + 8, y);
                record
            })
            .collect();
        let list = list_of(&mut e, &records);
        e.register_double(MAP_GET_AT, move |e, a| {
            if a[1] == 0x3C {
                e.mem.set_u32(a[2], list);
                returns(1)
            } else {
                returns(0)
            }
        });
        quiet(&mut e, &[LIST_REMOVE]);
        // Another world space: not found.
        assert_eq!(
            tes_save_load_game_get_created_exterior_cell_form_id(&mut e, game_object, 0x3D, 3, 4),
            0
        );
        // No record at those coordinates.
        assert_eq!(
            tes_save_load_game_get_created_exterior_cell_form_id(&mut e, game_object, 0x3C, 9, 9),
            0
        );
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        // The second record: its id comes back, it is taken out of the list
        // (by its address) and freed.
        assert_eq!(
            tes_save_load_game_get_created_exterior_cell_form_id(&mut e, game_object, 0x3C, 3, 4),
            0xBBB
        );
        let removed = calls_to(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], list);
        assert_eq!(e.mem.u32(removed[0][1]), records[1]);
        assert!(freed(&e, records[1]));
        assert!(!freed(&e, records[0]));
    }

    // ---- 00861130: the header of a save ----

    struct HeaderRig {
        e: Engine,
        game: Ptr<TESSaveLoadGame>,
        writes: FileWrites,
        screenshot: u32,
    }

    /// The player is "Alex", level 7, in "Vault", 1000 ms played, day 5 at
    /// 6 o'clock; the screenshot is an image whose levels are 12 bytes; the
    /// next save number is 9; the original save time is not set yet.
    fn header_rig() -> HeaderRig {
        let mut e = game();
        header_functions(&mut e, 0x7D);
        for page in [0x0103_5000u32, 0x0120_2000, 0x0108_0000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let game_object = singleton_of(&e);
        e.set(game_object, TESSaveLoadGame::m_cMajorVersion, 0);
        e.set(game_object, TESSaveLoadGame::m_cMinorVersion, 0x7D);
        let writes: FileWrites = Rc::new(RefCell::new(vec![]));
        let sink = writes.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
            returns(a[2])
        });
        quiet(
            &mut e,
            &[
                SAVE_PREPARE_B,
                STRING_OBJECT_CONSTRUCT,
                PLAYER_GET_LOCATION_TEXT,
            ],
        );
        quiet(
            &mut e,
            &[STRING_OBJECT_DESTRUCT, SAVE_LOCK_ENTER, SAVE_LOCK_LEAVE],
        );
        e.register(GET_SYSTEM_TIME_IMPORT, |e, a| {
            e.mem.write(a[0], &[0x11; 16]);
            Ret::default()
        });
        e.register(GET_LOCAL_TIME_IMPORT, |e, a| {
            e.mem.write(a[0], &[0x22; 16]);
            Ret::default()
        });
        e.set_global(PLAYER, 0x0099_0000u32);
        e.set_global(SAVE_LOCK, 0x0200_0000u32);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Alex");
        constant(&mut e, REFERENCE_GET_NAME, name);
        constant(&mut e, PLAYER_LEVEL, 7);
        let location = e.mem.alloc(16);
        e.mem.set_cstr(location, b"Vault");
        constant(&mut e, STRING_OBJECT_TEXT, location);
        constant(&mut e, CALENDAR_GET_DAY, 5);
        e.register(CALENDAR_GET_HOUR, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        e.mem.set_f64(HOURS_PER_DAY, 24.0);
        constant(&mut e, PLAYER_PLAY_TIME, 1000);
        constant(&mut e, NEXT_SAVE_NUMBER, 9);
        // The screenshot: an object with a destructor and a table at +0x5C
        // that makes its first level 12 bytes.
        let screenshot = object_with_slots(&mut e, &[(SLOT_DESTRUCTOR, 0x0395_0000)]);
        stub(&mut e, 0x0395_0000);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 0);
        e.mem.set_u32(table + 4, 12);
        e.mem.set_u32(screenshot + 0x5C, table);
        constant(&mut e, TAKE_SAVE_SCREENSHOT, screenshot);
        let pixels = e.mem.alloc(16);
        e.mem.write(pixels, &[0x33; 12]);
        constant(&mut e, IMAGE_PIXELS, pixels);
        HeaderRig {
            e,
            game: game_object,
            writes,
            screenshot,
        }
    }

    fn written(rig: &HeaderRig) -> Vec<u8> {
        rig.writes
            .borrow()
            .iter()
            .flat_map(|w| w.1.clone())
            .collect()
    }

    #[test]
    fn save_header_writes_the_description_and_the_screenshot() {
        let mut rig = header_rig();
        let file = Ptr::new(0x8000);
        tes_save_load_game_save_header(&mut rig.e, rig.game, file, Ptr::NULL);
        // total = name (5) + image (12 + 8) + location (6) + 0x24
        let mut expected = b"FO3SAVEGAME".to_vec();
        expected.extend([0, 0x7D]);
        expected.extend([0x11; 16]); // the original save time, set now
        expected.extend(0x7Du32.to_le_bytes()); // and its version: the minor
        expected.extend(67u32.to_le_bytes());
        expected.extend(9u32.to_le_bytes()); // the save number
        expected.push(5);
        expected.extend(b"Alex\0");
        expected.extend(7u16.to_le_bytes());
        expected.push(6);
        expected.extend(b"Vault\0");
        expected.extend(5.25f32.to_le_bytes()); // day 5 + 6/24
        expected.extend(1000u32.to_le_bytes());
        expected.extend([0x22; 16]); // the local time
        expected.extend(20u32.to_le_bytes());
        expected.extend(0x100u32.to_le_bytes());
        expected.extend(0x100u32.to_le_bytes());
        expected.extend([0x33; 12]);
        assert_eq!(written(&rig), expected);
        assert!(rig.writes.borrow().iter().all(|w| w.0 == 0x8000));
        // The number is counted up, the screenshot destroyed (flag 1), and
        // the save lock released around the picture.
        assert_eq!(rig.e.get(rig.game, TESSaveLoadGame::m_iNextSaveNumber), 1);
        assert_eq!(calls_to(&rig.e, 0x0395_0000), vec![vec![rig.screenshot, 1]]);
        assert_eq!(calls_to(&rig.e, SAVE_LOCK_LEAVE), vec![vec![0x0200_0000]]);
        assert_eq!(calls_to(&rig.e, SAVE_LOCK_ENTER), vec![vec![0x0200_0000]]);
        assert_eq!(
            rig.e.get(rig.game, TESSaveLoadGame::m_iOriginalSaveVersion),
            0x7D
        );
    }

    #[test]
    fn save_header_keeps_the_original_save_time_it_has() {
        let mut rig = header_rig();
        let game_object = rig.game;
        rig.e
            .set(game_object, TESSaveLoadGame::m_iOriginalSaveVersion, 0x55);
        rig.e.mem.write(game_object.addr() + 0x98, &[0x44; 16]);
        tes_save_load_game_save_header(&mut rig.e, rig.game, Ptr::new(0x8000), Ptr::NULL);
        assert!(calls_to(&rig.e, GET_SYSTEM_TIME_IMPORT).is_empty());
        let bytes = written(&rig);
        assert_eq!(bytes[13..29], [0x44; 16]);
        assert_eq!(bytes[29..33], 0x55u32.to_le_bytes());
    }

    #[test]
    fn save_header_writes_no_save_number_for_a_quick_or_auto_save() {
        for name in ["quicksave", "my autosave"] {
            let mut rig = header_rig();
            let name_text = rig.e.mem.alloc(32);
            rig.e.mem.set_cstr(name_text, name.as_bytes());
            tes_save_load_game_save_header(
                &mut rig.e,
                rig.game,
                Ptr::new(0x8000),
                Ptr::new(name_text),
            );
            let bytes = written(&rig);
            assert_eq!(bytes[37..41], [0, 0, 0, 0], "{name}");
            assert_eq!(rig.e.get(rig.game, TESSaveLoadGame::m_iNextSaveNumber), 0);
            assert!(calls_to(&rig.e, NEXT_SAVE_NUMBER).is_empty());
        }
        // Another name gets a number.
        let mut rig = header_rig();
        let name_text = rig.e.mem.alloc(32);
        rig.e.mem.set_cstr(name_text, b"Save 1");
        tes_save_load_game_save_header(&mut rig.e, rig.game, Ptr::new(0x8000), Ptr::new(name_text));
        assert_eq!(written(&rig)[37..41], 9u32.to_le_bytes());
    }

    #[test]
    fn save_header_falls_back_to_the_cells_description_and_skips_a_failed_picture() {
        let mut rig = header_rig();
        // No location text: the cell's description (slot 0x130) is used.
        constant(&mut rig.e, STRING_OBJECT_TEXT, 0);
        let description = rig.e.mem.alloc(16);
        rig.e.mem.set_cstr(description, b"Cell");
        constant(&mut rig.e, 0x0395_0130, description);
        let cell = object_with_slots(&mut rig.e, &[(FORM_GET_DESCRIPTION, 0x0395_0130)]);
        constant(&mut rig.e, REF_GET_PARENT_CELL, cell);
        // The picture fails: no image part at all.
        constant(&mut rig.e, TAKE_SAVE_SCREENSHOT, 0);
        tes_save_load_game_save_header(&mut rig.e, rig.game, Ptr::new(0x8000), Ptr::NULL);
        let bytes = written(&rig);
        // total = 5 + 0 + 5 + 0x24 = 46, and the image size field is 0.
        assert_eq!(bytes[33..37], 46u32.to_le_bytes());
        assert!(bytes.windows(5).any(|w| w == b"Cell\0"));
        assert_eq!(bytes[bytes.len() - 4..], [0, 0, 0, 0]);
        assert!(calls_to(&rig.e, IMAGE_PIXELS).is_empty());
    }

    #[test]
    fn save_header_only_measures_when_the_game_does() {
        let mut rig = header_rig();
        constant(&mut rig.e, SAVE_LOAD_UNAVAILABLE, 1);
        constant(&mut rig.e, SCREEN_WIDTH, 800);
        constant(&mut rig.e, SCREEN_HEIGHT, 600);
        // `_ftol2` of the aspect ratio (800 / 600 = 1.33...) is 1.
        rig.e.register(FLOAT_TO_INTEGER, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            returns(value as i32 as u32)
        });
        let stats: Ptr<SaveStats> = rig.e.new_object();
        rig.e
            .set(rig.game, TESSaveLoadGame::m_pSaveLoadStats, stats);
        rig.e
            .mem
            .set_cstr(LABEL_SAVE_GAME_HEADER, b"Save Game Header");
        quiet(&mut rig.e, &[LIST_ADD_HEAD]);
        tes_save_load_game_save_header(&mut rig.e, rig.game, Ptr::new(0x8000), Ptr::NULL);
        // Nothing is written; every size is counted: the fixed fields (84
        // bytes) and the image (0x100 * 0x100 * 3 + 8).
        assert!(rig.writes.borrow().is_empty());
        assert_eq!(
            rig.e.get(rig.game, TESSaveLoadGame::m_iSimulationFileSize),
            84 + 0x100 * 0x100 * 3 + 8
        );
        assert!(calls_to(&rig.e, TAKE_SAVE_SCREENSHOT).is_empty());
        assert!(calls_to(&rig.e, SAVE_LOCK_LEAVE).is_empty());
        // The statistics get the header's size: 5 + 196616 + 6 + 0x24.
        let added = calls_to(&rig.e, LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        let stat = rig.e.mem.u32(added[0][1]);
        assert_eq!(rig.e.mem.u32(stat), 5 + 196_616 + 6 + 0x24);
        assert_eq!(text(&rig.e, rig.e.mem.u32(stat + 4)), "Save Game Header");
        // The save number is not counted up when measuring.
        assert_eq!(rig.e.get(rig.game, TESSaveLoadGame::m_iNextSaveNumber), 0);
    }

    #[test]
    fn every_function_is_registered_once() {
        let list = funcs();
        assert_eq!(list.len(), 120);
        let mut addresses: Vec<u32> = list.iter().map(|(a, _)| *a).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 120);
    }
}
