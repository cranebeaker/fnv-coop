//! `fallout/misc/saveload/bgssaveloadgame.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSSaveLoadGame` is the object (`[0x011ddf38]`) that writes and reads the
//! body of a save file: the plugin list, the header, every changed form, the
//! form id maps and the history. This file holds its constructor and
//! destructor, `SaveGame`, `LoadGame`, and the small flag helpers around them.
//!
//! Translation notes
//!
//! - The build is not optimized: every function keeps `this` in a stack slot,
//!   so the decompiler's argument lists are unreliable. Arguments were read
//!   from the disassembly and checked against each callee's `RET n`. A
//!   callee that ends in a plain `RET` although words were pushed for it
//!   leaves them on the stack for the next call (`00866390`, `006815c0`);
//!   the words belong to that next call and are translated so.
//! - The C++ exception frames and the stack-protector cookie checks
//!   (`00ec408c`) are not translated.
//! - Session 1 covers the first 40 functions in address order, `00846f30` to
//!   `00849a70`. Session 2 covers the next 40, `00849a90` to `0084b650`.
//!   Session 3 covers the last 16, `0084b6c0` to `0084bc20`: the unit is
//!   complete. `0084b720` (the destructor body of the hash map base, vtable
//!   `0107f4c8`) is outside the unit's queue and is called by address.
//! - A `BGSChangeFlags` is one `u32`; `008c71b0` stores a value into one
//!   (`this`, value) and returns `this`. The game builds temporaries of it on
//!   its stack to pass them by value; [`change_flags`] does the same.
//! - Shared constants, layouts and helpers at the top are `pub(crate)` for
//!   the later sessions of this unit.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;

// ---- Globals -------------------------------------------------------------

/// The `BGSSaveLoadGame` singleton pointer.
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// The `BGSSaveLoadManager` singleton pointer.
pub(crate) const MANAGER_INSTANCE: u32 = 0x011d_e134;
/// The `TESDataHandler` pointer.
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The player character pointer.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// The `TES` world object pointer.
pub(crate) const WORLD: u32 = 0x011d_ea10;
/// Pointer to the I/O manager.
pub(crate) const IO_MANAGER: u32 = 0x0120_2d98;
/// Pointer to the object `00848e40` forwards to (null when there is none).
pub(crate) const MENU_OBJECT: u32 = 0x011d_aac0;
/// The `Calendar` object (its address, not a pointer to it).
pub(crate) const CALENDAR: u32 = 0x011d_e7b8;

// ---- Texts -----------------------------------------------------------------

/// `"SAVELOAD: Cannot find file %s referenced in the save game.  Errors may
/// result."`.
pub(crate) const MISSING_PLUGIN_WARNING: u32 = 0x0107_eec8;
/// `"---Finished saving game: %s"`.
pub(crate) const FINISHED_SAVING_NOTE: u32 = 0x0107_ef18;
/// `"SAVELOAD: Could not find form %08X with change flags %08X during
/// BGSSaveLoadGame::SaveGame()"`.
pub(crate) const SAVE_FORM_MISSING_WARNING: u32 = 0x0107_ef38;
/// `"***Saving game: %s (%s)"`.
pub(crate) const SAVING_NOTE: u32 = 0x0107_ef98;
/// `"---Finished loading game: %s"`.
pub(crate) const FINISHED_LOADING_NOTE: u32 = 0x0107_efb0;
/// `"SAVELOAD: Form '%s' (%08X) was saved with form type %s, but currently
/// has form type %s.  Its loading will be skipped."`.
pub(crate) const FORM_TYPE_MISMATCH_WARNING: u32 = 0x0107_efd0;
/// `"***Loading game: %s (%s)"`.
pub(crate) const LOADING_NOTE: u32 = 0x0107_f048;
/// `"SAVELOAD: (Load Error) Form '%s' (%08X) was saved with form type %s, but
/// currently has form type %s.  Its loading will be skipped."`.
pub(crate) const LOAD_ERROR_TYPE_MISMATCH_WARNING: u32 = 0x0107_f068;
/// `"FORMS: No Worldspace or Cell could not be found while unloading
/// reference '%s' (%08X)"`.
pub(crate) const UNLOAD_NO_PLACE_WARNING: u32 = 0x0107_f0f0;
/// `"FORMS: Form '%s' (%08X) is unloading, but it already has a buffer.  This
/// unload will be skipped."`.
pub(crate) const UNLOAD_HAS_BUFFER_WARNING: u32 = 0x0107_f148;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\Misc\SaveLoad\BGSSaveLoadGame.cpp"`.
pub(crate) const SOURCE_PATH: u32 = 0x0107_f1b0;

// ---- Callees outside this file ---------------------------------------------

/// `operator new(size)` and `operator delete(block)`.
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset(destination, value, size)` (cdecl).
pub(crate) const MEMORY_SET: u32 = 0x0040_3d30;
/// Returns its `this`; the game uses it as a cast.
pub(crate) const THIS_IDENTITY: u32 = 0x0068_15c0;
/// String comparison ignoring case `(a, b)`, 0 when equal.
pub(crate) const STRING_COMPARE_IGNORE_CASE: u32 = 0x0040_4dc0;
/// `BSString::Set(this, text, length)`.
pub(crate) const BSSTRING_SET: u32 = 0x0040_37f0;
/// The form type byte (`this + 4`, zero extended).
pub(crate) const FORM_GET_TYPE: u32 = 0x0040_1170;
/// Looks a form up by id (cdecl `(id)`), 0 when there is none.
pub(crate) const LOOKUP_FORM: u32 = 0x0048_39c0;
/// Reads the `u32` at its `this` (a `BGSChangeFlags` or pointer getter).
pub(crate) const READ_WORD: u32 = 0x0055_9450;
/// The number of compiled files the data handler holds (the byte at +0x218;
/// the engine map names the body `MiddleHighProcess::GetHeadNode` because the
/// linker folded identical code).
pub(crate) const DATA_HANDLER_COMPILED_FILE_COUNT: u32 = 0x0051_f550;
/// `TESDataHandler::GetCompiledFile(this, index)` (Xbox PDB) and the file name
/// that sits at +0x20 of a compiled file.
pub(crate) const DATA_HANDLER_GET_COMPILED_FILE: u32 = 0x0046_5010;
pub(crate) const COMPILED_FILE_NAME: u32 = 0x0089_1170;
/// Scope guard: construct `(this, 0x11, 1, file, line)` and destruct `(this)`.
pub(crate) const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
pub(crate) const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `(this = data handler, id)`: true when the form id is `0xff000000` or
/// more.
pub(crate) const IS_DYNAMIC_FORM_ID: u32 = 0x0046_9860;
/// `(this, on)` on the per-thread flag word (bit 0 of the TLS block's +0x294);
/// returns what `00462480(this)` returned, which the callers pass back to
/// restore the flag.
pub(crate) const SET_THREAD_FLAG: u32 = 0x0046_23f0;
/// `(this = form, on)` called on a form after its load data was applied.
pub(crate) const FORM_MARK_LOADED: u32 = 0x0046_a010;
/// The form's own type name (`this = form`) and the name of a type byte
/// (cdecl `(type)`), both for the warnings.
pub(crate) const FORM_TYPE_NAME: u32 = 0x0044_0e30;
pub(crate) const TYPE_NAME_FOR: u32 = 0x0046_12b0;
/// `SaveGameWarning(format, ...)` (cdecl).
pub(crate) const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `SaveGameDataToFileFunction(file, data, size)` and
/// `LoadGameDataFromFileFunction(file, data, size)` (cdecl).
pub(crate) const SAVE_DATA_TO_FILE: u32 = 0x0084_5d00;
pub(crate) const LOAD_DATA_FROM_FILE: u32 = 0x0084_5d20;
/// `BGSSaveLoadFile`: read `(file, buffer, size)`, write `(file, buffer,
/// size)`, position, seek to a position `(file, position)`, seek by an offset
/// `(file, offset)`.
pub(crate) const FILE_READ: u32 = 0x0084_6300;
pub(crate) const FILE_WRITE: u32 = 0x0084_6330;
pub(crate) const FILE_GET_POSITION: u32 = 0x0084_6400;
pub(crate) const FILE_SEEK_SET: u32 = 0x0084_6440;
pub(crate) const FILE_SEEK_CURRENT: u32 = 0x0084_6490;
/// `BGSSaveLoadFormIDMap::Save(this, file)` and `Load(this, file)`.
pub(crate) const FORM_ID_MAP_SAVE: u32 = 0x0084_6e00;
pub(crate) const FORM_ID_MAP_LOAD: u32 = 0x0084_6e70;
/// Constructors of the objects the game object owns.
pub(crate) const CHANGES_MAP_CONSTRUCT: u32 = 0x0084_5550;
pub(crate) const FORM_ID_MAP_CONSTRUCT: u32 = 0x0084_6b60;
pub(crate) const REFERENCES_MAP_CONSTRUCT: u32 = 0x0085_2a70;
pub(crate) const QUEUED_SUB_BUFFERS_MAP_CONSTRUCT: u32 = 0x0086_51d0;
pub(crate) const CHANGED_FORM_ID_MAP_CONSTRUCT: u32 = 0x006e_2080;
pub(crate) const HISTORY_CONSTRUCT: u32 = 0x0084_ded0;
pub(crate) const RECONSTRUCT_FORMS_CONSTRUCT: u32 = 0x0084_33d0;
/// Destructor bodies (without the delete) of three owned objects.
pub(crate) const FORM_ID_MAP_DESTRUCT: u32 = 0x0084_6be0;
pub(crate) const REFERENCES_MAP_DESTRUCT: u32 = 0x0085_2ae0;
pub(crate) const QUEUED_SUB_BUFFERS_MAP_DESTRUCT: u32 = 0x0086_5220;
pub(crate) const HISTORY_DESTRUCT: u32 = 0x0084_df30;
/// Constructor `(this)` of the form buffer array, constructor `(this, 0x25)`
/// of the package location map, and their destructors.
pub(crate) const FORM_BUFFER_ARRAY_CONSTRUCT: u32 = 0x0084_b570;
pub(crate) const PACKAGE_LOCATION_MAP_CONSTRUCT: u32 = 0x0084_b4b0;
pub(crate) const FORM_BUFFER_ARRAY_DESTRUCT: u32 = 0x0084_b5a0;
pub(crate) const PACKAGE_LOCATION_MAP_DESTRUCT: u32 = 0x0084_b6c0;
/// `(this, value)`: calls `00570f40(this)`, then sets bit 0 of `[+0x244]`
/// when `value` is 0 and clears it otherwise; returns what `00570f40`
/// returned (the engine map names the body `startmenu.cpp`: folded code).
pub(crate) const GAME_SET_FLAG_ONE: u32 = 0x007d_6bd0;
/// `(this, on)`: sets or clears mask 4 of `[+0x244]`; `SaveGame` brackets
/// itself with it.
pub(crate) const GAME_SET_SAVING_FLAG: u32 = 0x0084_7d00;
/// `BGSSaveGameBuffer`: constructor `(this)`, destructor, `Save(this, file)`,
/// `SaveBytes(this, data, size, 0)`, `SaveString(this, text, 0)`.
pub(crate) const SAVE_GAME_BUFFER_CONSTRUCT: u32 = 0x0086_5bd0;
pub(crate) const SAVE_GAME_BUFFER_DESTRUCT: u32 = 0x0086_5c10;
pub(crate) const SAVE_GAME_BUFFER_SAVE: u32 = 0x0086_5c40;
pub(crate) const SAVE_GAME_BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
pub(crate) const SAVE_GAME_BUFFER_SAVE_STRING: u32 = 0x0086_5e70;
/// `BGSLoadGameBuffer`: constructor `(this)`, `Load(this, file)`,
/// `LoadBytes(this, out, size)`, `LoadString(this, out)`, destructor body.
pub(crate) const LOAD_GAME_BUFFER_CONSTRUCT: u32 = 0x0086_46b0;
pub(crate) const LOAD_GAME_BUFFER_LOAD: u32 = 0x0086_4740;
pub(crate) const LOAD_GAME_BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
pub(crate) const LOAD_GAME_BUFFER_LOAD_STRING: u32 = 0x0086_49a0;
pub(crate) const LOAD_GAME_BUFFER_DESTRUCT: u32 = 0x0086_46f0;
/// `BGSSaveFormBuffer`: constructor `(this)`, `Save(this, file)`, header
/// setter `(this, id, flags, type, minor_version)`, form setter `(this,
/// form)`, reset `(this)`, destructor `(this)`.
pub(crate) const SAVE_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_59c0;
pub(crate) const SAVE_FORM_BUFFER_SAVE: u32 = 0x0086_5a30;
pub(crate) const SAVE_FORM_BUFFER_SET_HEADER: u32 = 0x0086_5ad0;
pub(crate) const SAVE_FORM_BUFFER_SET_FORM: u32 = 0x0050_f9c0;
pub(crate) const SAVE_FORM_BUFFER_RESET: u32 = 0x0084_7db0;
pub(crate) const SAVE_FORM_BUFFER_DESTRUCT: u32 = 0x0084_7dd0;
/// `(this = form buffer, out)`: copies the header's `iChangeFlags` (Xbox PDB
/// `BGSSaveLoadFormHeader`, +3 of the header at +0x14 of a form buffer, so
/// +0x17) into `*out` and returns `out`. `BGSSaveFormBuffer::GetForm` (Xbox
/// PDB) is `007af430`.
pub(crate) const FORM_BUFFER_HEADER_FLAGS: u32 = 0x0042_8110;
pub(crate) const BUFFER_GET_FORM: u32 = 0x007a_f430;
/// `BGSLoadFormBuffer`: constructor `(this)`, `LoadHeader(this, file)`, header
/// form type `(this)` (`00853640` on the header's `FormInfo`),
/// `LoadDataFromFile(this, file)` (negative on failure),
/// `SkipData(this, file, size)`, set the header's change flags `(this, flags)`
/// (`00428130`), set the old change flags `(this, flags)` (`0086cf00`), set
/// the form `(this, form)` (`007037c0`), read the old change flags `(this,
/// out)` (`0042ce30`), set the "loaded" bit `(this, on)` (`0042ce50`),
/// delete `(this, 1)` (`0081db60`) and destruct (`0081db90`).
pub(crate) const LOAD_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_43b0;
pub(crate) const LOAD_FORM_BUFFER_LOAD_HEADER: u32 = 0x0086_44b0;
pub(crate) const LOAD_FORM_BUFFER_HEADER_TYPE: u32 = 0x0086_65b0;
pub(crate) const LOAD_FORM_BUFFER_LOAD_DATA: u32 = 0x0086_4540;
pub(crate) const LOAD_FORM_BUFFER_SKIP_DATA: u32 = 0x0086_4580;
pub(crate) const LOAD_FORM_BUFFER_SET_HEADER_FLAGS: u32 = 0x0042_8130;
pub(crate) const LOAD_FORM_BUFFER_SET_OLD_FLAGS: u32 = 0x0086_cf00;
pub(crate) const LOAD_FORM_BUFFER_SET_FORM: u32 = 0x0070_37c0;
pub(crate) const LOAD_FORM_BUFFER_OLD_FLAGS: u32 = 0x0042_ce30;
pub(crate) const LOAD_FORM_BUFFER_SET_LOADED: u32 = 0x0042_ce50;
pub(crate) const LOAD_FORM_BUFFER_DELETE: u32 = 0x0081_db60;
pub(crate) const LOAD_FORM_BUFFER_DESTRUCT: u32 = 0x0081_db90;
/// The form id word of a form buffer (`this + 0x10`) and the form id word of
/// a form (`this + 0x0c`).
pub(crate) const BUFFER_FORM_ID: u32 = 0x0044_edb0;
pub(crate) const FORM_ID_WORD: u32 = 0x0084_e3a0;
/// `BGSUnloadedFormBuffer`: construct `(this)`, `Save(this, file, id,
/// flags)`, `Load(this, file)`, `CreateLoadFormBuffer(this)`, `(this, buffer)`
/// release, and `GetAdvancedBuffer(this)` (`this` only; see the notes above).
pub(crate) const UNLOADED_FORM_BUFFER_CONSTRUCT: u32 = 0x0044_dee0;
pub(crate) const UNLOADED_FORM_BUFFER_SAVE: u32 = 0x0086_6200;
pub(crate) const UNLOADED_FORM_BUFFER_LOAD: u32 = 0x0086_62c0;
pub(crate) const UNLOADED_FORM_BUFFER_CREATE_LOAD_BUFFER: u32 = 0x0086_6480;
pub(crate) const UNLOADED_FORM_BUFFER_RELEASE: u32 = 0x0086_6570;
pub(crate) const UNLOADED_FORM_BUFFER_ADVANCED: u32 = 0x0086_6390;
/// `(this)`: sets the word at `*this` to 0 (the stored buffer pointer of an
/// entry, once its load buffer has been made).
pub(crate) const UNLOADED_FORM_BUFFER_CLEAR: u32 = 0x0066_65a0;
/// `BGSSaveLoadChangesMap`: first position `(map)`, next `(map, &position,
/// &key, &value)`, get flags `(map, &out, id)`, set flags `(map, id,
/// flags)`, add flags `(map, id, flags)`, remove `(map, id)`, clear flags
/// `(map, id, flags)` (clears them from the entry; false when flags remain or
/// the id is dynamic, otherwise the entry is removed), get the entry `(map, &out, id)`, store a
/// buffer for an id `(map, id, header_flags, unloaded_buffer)`, and
/// `(map, id)` returning the entry of a form id (or 0).
pub(crate) const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
pub(crate) const MAP_NEXT: u32 = 0x006b_7f20;
pub(crate) const CHANGES_MAP_GET_FLAGS: u32 = 0x0084_56e0;
pub(crate) const CHANGES_MAP_SET_FLAGS: u32 = 0x0084_58b0;
pub(crate) const CHANGES_MAP_ADD_FLAGS: u32 = 0x0084_57b0;
pub(crate) const CHANGES_MAP_REMOVE: u32 = 0x0084_5a20;
pub(crate) const CHANGES_MAP_CLEAR_FLAGS: u32 = 0x0084_5a80;
pub(crate) const CHANGES_MAP_GET_ENTRY: u32 = 0x0084_5760;
pub(crate) const CHANGES_MAP_STORE_BUFFER: u32 = 0x0084_5960;
pub(crate) const CHANGES_MAP_ENTRY_FOR: u32 = 0x009a_4250;
/// `BGSSaveLoadInitialData` (all cdecl): `SaveInitialData(buffer, form,
/// flags)` returns the kind, `LoadInitialData(buffer, form, flags)`,
/// `(id, a, flags)` returns the kind (`0084e730`), `(advanced_buffer, kind,
/// struct)` (`0084e930`), and `ReferenceInitialData::
/// GetOriginalLocationCellAndWorld(this, &cell, &world)` `0084e6a0`.
pub(crate) const SAVE_INITIAL_DATA: u32 = 0x0084_eb80;
pub(crate) const LOAD_INITIAL_DATA: u32 = 0x0084_f330;
pub(crate) const INITIAL_DATA_KIND: u32 = 0x0084_e730;
pub(crate) const LOAD_INITIAL_DATA_STRUCT: u32 = 0x0084_e930;
pub(crate) const GET_ORIGINAL_LOCATION: u32 = 0x0084_e6a0;
/// `BGSSaveLoadHistory`: `AddNote(history, format, ...)` (cdecl), `Save(this,
/// file)`, `Load(this, file)`, `AddHistory(this, other)`.
pub(crate) const HISTORY_ADD_NOTE: u32 = 0x0084_dff0;
pub(crate) const HISTORY_SAVE: u32 = 0x0084_e0d0;
pub(crate) const HISTORY_LOAD: u32 = 0x0084_e190;
pub(crate) const HISTORY_ADD_HISTORY: u32 = 0x0084_e2c0;
/// Global data of the save (cdecl): `Save(file, first, last)`, the two
/// loaders `(first, last)`, `(file, size)` and `(flag)`.
pub(crate) const SAVE_GLOBAL_DATA: u32 = 0x0084_bb50;
pub(crate) const LOAD_GLOBAL_DATA_A: u32 = 0x0084_bfb0;
pub(crate) const LOAD_GLOBAL_DATA_B: u32 = 0x0084_c110;
pub(crate) const LOAD_GLOBAL_BLOCK: u32 = 0x0084_bc20;
pub(crate) const GLOBAL_DATA_RESET: u32 = 0x0084_c270;
/// `BGSReconstructFormsInAllFilesMap`: `AddForm(this, form, flags)`,
/// `AddReference(this, cell, id, flags)`, and the two closing steps.
pub(crate) const RECONSTRUCT_ADD_FORM: u32 = 0x0084_3630;
pub(crate) const RECONSTRUCT_ADD_REFERENCE: u32 = 0x0084_3750;
pub(crate) const RECONSTRUCT_FINISH_A: u32 = 0x0084_39e0;
pub(crate) const RECONSTRUCT_FINISH_B: u32 = 0x0084_35a0;
/// `BGSSaveLoadManager`: `GetVersionInfo(this, out, size)`, minor version
/// `(this)`, copy the file's name `(this, source, destination)`, the save
/// name builder `(out)` (cdecl) and the load preparation `(this)`.
pub(crate) const MANAGER_GET_VERSION_INFO: u32 = 0x0085_1120;
pub(crate) const MANAGER_GET_MINOR_VERSION: u32 = 0x0085_1110;
pub(crate) const MANAGER_COPY_FILE_NAME: u32 = 0x0084_fec0;
pub(crate) const MANAGER_BUILD_SAVE_NAME: u32 = 0x0085_0b40;
pub(crate) const MANAGER_START_LOAD: u32 = 0x0085_0c20;
/// Steps of the load and unload passes, each on `this`: the references map's
/// reset, the I/O manager's pause, resume and queued priority load, the
/// world's two steps, the physics world of the player's cell and its
/// `DeactivateAllIslands`, the queued sub buffers' flush and the changed form
/// id map's flush.
pub(crate) const REFERENCES_MAP_RESET: u32 = 0x0085_2b60;
pub(crate) const IO_MANAGER_PAUSE: u32 = 0x00c3_e310;
pub(crate) const IO_MANAGER_RESUME: u32 = 0x00c3_e340;
pub(crate) const IO_MANAGER_LOAD_QUEUED_PRIORITY: u32 = 0x0045_6520;
pub(crate) const AFTER_LOAD_FINISH: u32 = 0x00b6_0040;
pub(crate) const WORLD_PREPARE: u32 = 0x0045_9920;
pub(crate) const WORLD_FINISH: u32 = 0x0045_cda0;
pub(crate) const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
pub(crate) const CELL_PHYSICS_WORLD: u32 = 0x0045_43c0;
pub(crate) const PHYSICS_WORLD_DEACTIVATE_ALL_ISLANDS: u32 = 0x00c6_a870;
pub(crate) const QUEUED_SUB_BUFFERS_FLUSH: u32 = 0x0086_5360;
pub(crate) const CHANGED_FORM_ID_MAP_FLUSH: u32 = 0x0043_8af0;
/// `TES::IsCellLoaded(this, cell, 0)` (Xbox PDB).
pub(crate) const WORLD_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB).
pub(crate) const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `TESObjectCELL::RemoveReference(cell, reference)` (Xbox PDB) and
/// `GarbageCollector::Add(form)` (Xbox PDB, cdecl).
pub(crate) const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
pub(crate) const GARBAGE_COLLECTOR_ADD: u32 = 0x0086_7f90;
/// Other `BGSSaveLoadGame` functions: `AddChange(this, form, flags, flag)`,
/// `CheckInitialData(this, file, buffer, id_map)`,
/// `HandleUnrevertibleChanges(this, &form, flags, flag)`, `ClearForm(this,
/// form)`, `00849a90(this, form)` (true when the form may be unloaded) and
/// `0084a5b0(this)`.
pub(crate) const GAME_ADD_CHANGE: u32 = 0x0084_a690;
pub(crate) const GAME_CHECK_INITIAL_DATA: u32 = 0x0084_9d00;
pub(crate) const GAME_HANDLE_UNREVERTIBLE_CHANGES: u32 = 0x0084_a3a0;
pub(crate) const GAME_CLEAR_FORM: u32 = 0x0084_a880;
pub(crate) const GAME_CAN_UNLOAD_FORM: u32 = 0x0084_9a90;
pub(crate) const GAME_AFTER_LOAD_PASS: u32 = 0x0084_a5b0;
/// `(this = data handler)`: the byte at +0x61d, `bClearingData` (Xbox PDB).
pub(crate) const DATA_HANDLER_GET_CLEARING_DATA: u32 = 0x0042_26e0;
/// `(this = cell)`: bit 0 of the byte at +0x24, `cCellFlags` (Xbox PDB, +0x34
/// there), and the reference's worldspace getter `(this = reference)`
/// (`TESObjectREFR::GetWorldSpace`, Xbox PDB).
pub(crate) const CELL_FLAG_ZERO_TEST: u32 = 0x0042_5fd0;
pub(crate) const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
/// `(references_map, world_word, id, place)` and `NiTMap::SetAt(map, key,
/// value)` (Xbox PDB name `CombatThreatMap`, same body) and the pathing-LOS
/// style setup `(map, a, b)`.
pub(crate) const REFERENCES_MAP_ADD_UNLOADED: u32 = 0x0085_2c00;
pub(crate) const MAP_SET_AT: u32 = 0x0084_4700;
pub(crate) const REFERENCES_MAP_SETUP: u32 = 0x0085_2950;
/// The form buffer array: size `(array)`, address of element `(array,
/// index)`, `SetAt(array, index, &value)`, `SetReservedSize(array, n)`,
/// `Clear(array, flag)` and `AddUninitialized(array, &value)`.
pub(crate) const ARRAY_SIZE: u32 = 0x0044_ddc0;
pub(crate) const ARRAY_ELEMENT_ADDRESS: u32 = 0x006a_7ad0;
pub(crate) const ARRAY_SET_AT: u32 = 0x0047_a110;
pub(crate) const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
pub(crate) const ARRAY_CLEAR: u32 = 0x0084_54f0;
pub(crate) const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `Calendar` `fn_00867a20` and the menu forwarder `(object, 0x20000000,
/// value)`.
pub(crate) const CALENDAR_MARK_STALE: u32 = 0x0086_7a20;
pub(crate) const MENU_OBJECT_FORWARD: u32 = 0x0075_f6f0;
/// Clears the list at the player's +0x5fc.
pub(crate) const LIST_CLEAR: u32 = 0x0047_0470;
/// `BGSChangeFlags` store `(this, value)` and `|=` `(this, value)`; the test
/// `00840f70(entry, form)`; and the combination `00841120(out, flags, temp,
/// type, flag)` (cdecl).
pub(crate) const CHANGE_FLAGS_STORE: u32 = 0x008c_71b0;
pub(crate) const CHANGE_FLAGS_OR: u32 = 0x0084_1100;
pub(crate) const ENTRY_TEST: u32 = 0x0084_0f70;
pub(crate) const COMBINE_CHANGE_FLAGS: u32 = 0x0084_1120;

/// `BGSSaveLoadGame` constructor (`00846f30`, in this file), the initializer
/// `Create` runs after it, the test of mask 2 of `[+0x244]` (`0042ce10`,
/// folded into `extradatalist.cpp` in the engine map), and `00853640` (the
/// type of a stored unloaded buffer, read from the pointer `00849220` loads).
pub(crate) const GAME_CONSTRUCT: u32 = 0x0084_6f30;
pub(crate) const GAME_CREATE_FOLLOW_UP: u32 = 0x0085_35e0;
pub(crate) const GAME_LOADING_FLAG_TEST: u32 = 0x0042_ce10;
pub(crate) const STORED_BUFFER_TYPE: u32 = 0x0085_3640;

// ---- Session 2: texts, globals and callees (functions from 00849a90 on) ----

/// A `double` 0.0 in `.rdata` that `QueueInitPackageLocations` compares with.
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;
/// Offset of the per-thread flag word in the TLS block: bit 0 is the one
/// `004623f0` sets and clears, bit 1 (mask 2) the one `00849c00` sets and clears.
pub(crate) const THREAD_FLAGS_OFFSET: u32 = 0x294;
/// `"CELLS: Worldspace or Cell %08X could not be found while loading a %s
/// reference.  Its loading will be skipped."`.
pub(crate) const MISSING_PLACE_WARNING: u32 = 0x0107_f200;
/// `"moved"` and `"created"`: the `%s` of the warning above.
pub(crate) const TEXT_MOVED: u32 = 0x0107_f270;
pub(crate) const TEXT_CREATED: u32 = 0x0107_f278;
/// `"Reference %s %08X with type %s and with changes %08X could not revert its
/// changed location"`.
pub(crate) const REVERT_LOCATION_WARNING: u32 = 0x0107_f280;
/// `"FORMS: Form %08X with form type %i %s was re-numbered by
/// BGSSaveLoadGame::ClearForm()"`.
pub(crate) const RENUMBERED_WARNING: u32 = 0x0107_f2e0;
/// `"FORMS: Trying to set new form ID %08X for old form ID %08X, but already in
/// map with value %08X"`.
pub(crate) const CHANGED_ID_CONFLICT_WARNING: u32 = 0x0107_f338;
/// `"CleanupExpiredData removed havok from %i refs: %i removed all changes and
/// %i unloaded refs removed partial changes"`.
pub(crate) const CLEANUP_NOTE: u32 = 0x0107_f398;
/// `"During cleanup, form %08X with changeflags %08X and no unloaded form
/// buffer could not be found"`.
pub(crate) const CLEANUP_MISSING_BUFFER_WARNING: u32 = 0x0107_f410;
/// Vtables of the classes this file constructs: the package location map
/// (`NiTMap<unsigned int,Actor *>`), the expiry queue (`NiTMap<__int64,int>`),
/// the form buffer array (`BSSimpleArray<BGSLoadFormBuffer *,1024>`) and the
/// base class of the package location map (`0084b650`).
pub(crate) const PACKAGE_LOCATION_MAP_VTABLE: u32 = 0x0107_f474;
pub(crate) const EXPIRY_QUEUE_VTABLE: u32 = 0x0107_f494;
pub(crate) const FORM_BUFFER_ARRAY_VTABLE: u32 = 0x0107_f4b4;
pub(crate) const HASH_MAP_BASE_VTABLE: u32 = 0x0107_f4c8;
/// `(this = form)`: bit 0x4000 of the word at +8 (`iFormFlags`, Xbox PDB).
pub(crate) const FORM_FLAG_4000_TEST: u32 = 0x0040_77c0;
/// `(this = form)`: bit 0x20 of the word at +8 (`iFormFlags`, Xbox PDB).
pub(crate) const FORM_FLAG_20_TEST: u32 = 0x0044_0d80;
/// `(this = game)`: the test `00570f40` ends in; the callers use it as "changes
/// are being recorded".
pub(crate) const GAME_RECORDING_TEST: u32 = 0x0057_0f00;
/// `BGSSaveLoadChangesMap` test `(map, id, mask)`: the entry of `id` exists
/// and its flags have a bit of `mask`.
pub(crate) const CHANGES_MAP_HAS_FLAGS: u32 = 0x0084_5720;
/// A `BGSChangeFlags` word: test `(this = &flags, mask)` (any bit of `mask`
/// set) and clear `(this = &flags, mask)`.
pub(crate) const FLAGS_TEST: u32 = 0x0042_80f0;
pub(crate) const FLAGS_CLEAR: u32 = 0x0056_2210;
/// `(this = reference, 0)`: asks the reference for a location (slot `0x138`)
/// and moves it there when that place is loaded or the reference persists;
/// true when it moved.
pub(crate) const REFERENCE_MOVE_TO_LOCATION: u32 = 0x0056_1ef0;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB) `(this = reference)`.
pub(crate) const REFERENCE_IS_PERSISTENT: u32 = 0x0056_53d0;
/// `(this = form)`: the length of the text `FORM_SLOT_NAME` returns (0 when
/// there is none).
pub(crate) const FORM_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `(this = reference)`: the reference's full name.
pub(crate) const REFERENCE_FULL_NAME: u32 = 0x0055_d520;
/// `(this = actor)`: the word at +0x20, which the code treats as an object with
/// a sub-object at +0x100.
pub(crate) const ACTOR_PROCESS: u32 = 0x0041_81e0;
/// `Actor::InitPackageLocations(this, flag)` (Xbox PDB).
pub(crate) const ACTOR_INIT_PACKAGE_LOCATIONS: u32 = 0x0089_3340;
/// `TESDataHandler::GetNextID(this = data handler, 1)` (Xbox PDB).
pub(crate) const DATA_HANDLER_GET_NEXT_ID: u32 = 0x0046_9800;
/// `TESObjectCELL::GetDetachTime`, `GetWorldSpace`, `GetDataX` and `GetDataY`
/// (Xbox PDB), each on `this = cell`.
pub(crate) const CELL_DETACH_TIME: u32 = 0x0054_6af0;
pub(crate) const CELL_WORLD_SPACE: u32 = 0x0054_ddd0;
pub(crate) const CELL_DATA_X: u32 = 0x0054_4c30;
pub(crate) const CELL_DATA_Y: u32 = 0x0054_4c60;
/// `TESWorldSpace::GetCellFromWorldCoord(this = worldspace, &position)` (Xbox
/// PDB).
pub(crate) const WORLD_SPACE_CELL_FROM_COORD: u32 = 0x0058_7550;
/// cdecl `(x, y)`: the two 16-bit grid coordinates packed as `x << 16 | y`.
pub(crate) const PACK_GRID_COORDS: u32 = 0x0058_7410;
/// cdecl `(f32)`: rounds to an integer.
pub(crate) const ROUND_TO_INT: u32 = 0x0040_6d90;
/// `(this = calendar)`: the game time stamp the expiry test compares with.
pub(crate) const CALENDAR_TIME_STAMP: u32 = 0x0086_7e30;
/// `()`: how long a detached cell's data is kept.
pub(crate) const EXPIRY_LIMIT: u32 = 0x0052_6100;
/// cdecl `(file, kind, struct)`: 0 when `kind` is 0, otherwise reads the
/// initial data of `kind` from `file` and runs `0084e930` on it with `struct`
/// (returning its result).
pub(crate) const LOAD_INITIAL_DATA_FROM_FILE: u32 = 0x0084_e850;
/// `(this = initial data, &cell, &world)`: the place the initial data puts the
/// reference at.
pub(crate) const LOCATION_CELL_AND_WORLD: u32 = 0x0084_e610;
/// cdecl `(id, &initial_data, form)`: makes the reference of an initial data
/// structure (reusing `form` when given).
pub(crate) const CREATE_REFERENCE: u32 = 0x0084_f8e0;
/// `BGSReconstructFormsInAllFilesMap::AddReference` (Xbox PDB, second
/// overload) `(this, world, x, y, id, flags)`.
pub(crate) const RECONSTRUCT_ADD_REFERENCE_AT: u32 = 0x0084_38b0;
/// The destructor body of the reconstruct map, and the two accessors of the
/// reference list inside it: count `(this = list)` and element `(this = list,
/// index)`.
pub(crate) const RECONSTRUCT_DESTRUCT: u32 = 0x0084_34d0;
pub(crate) const FORM_LIST_COUNT: u32 = 0x0084_3bb0;
pub(crate) const FORM_LIST_ELEMENT: u32 = 0x0084_3330;
/// `BGSSaveLoadReferencesMap`: `LoadReferencesForCell(this, cell, &reconstruct)`,
/// `RemoveReference(this, cell_key, id, &place)` (Xbox PDB) and
/// `BGSCellNumericIDArrayMap::RemoveReference(this, a, b)` (Xbox PDB).
pub(crate) const REFERENCES_MAP_LOAD_FOR_CELL: u32 = 0x0085_2d20;
pub(crate) const REFERENCES_MAP_REMOVE_EXTERIOR: u32 = 0x0085_2cd0;
pub(crate) const REFERENCES_MAP_REMOVE: u32 = 0x0085_2a10;
/// `(this = map, key, &out)`: true when the map has `key`, with its value in
/// `*out`.
pub(crate) const MAP_GET_VALUE: u32 = 0x0085_3130;
/// `NiTMapBase::RemoveAt(this = map, key)` (Xbox PDB name of the body).
pub(crate) const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// `BGSSaveLoadQueuedSubBufferMap::QueueSubBuffer(this, a, b, c)` (Xbox PDB).
pub(crate) const QUEUED_SUB_BUFFERS_QUEUE: u32 = 0x0086_52f0;
/// `BGSSaveLoadHistory::AddNoteVarArgs(this, format, va_list)` (Xbox PDB).
pub(crate) const HISTORY_ADD_NOTE_VA: u32 = 0x0084_e020;
/// The expiry queue (`NiTMap<__int64,int>`): insert `(this, key_low,
/// key_high, time)`, find `(this, key_low, key_high, &out)` (true when found),
/// the destructor body `(this)` and the base constructor `(this, size)`.
pub(crate) const EXPIRY_QUEUE_INSERT: u32 = 0x0084_b7c0;
pub(crate) const EXPIRY_QUEUE_FIND: u32 = 0x0084_b8b0;
pub(crate) const EXPIRY_QUEUE_DESTRUCT: u32 = 0x0084_b9c0;
pub(crate) const EXPIRY_QUEUE_BASE_CONSTRUCT: u32 = 0x0084_b750;
/// `BGSUnloadedFormBuffer`: `GetSize(this)`, `Initialize(this, data, size,
/// type, flag)` (Xbox PDB), the initial data type's data size (cdecl `(kind)`,
/// `BGSSaveLoadInitialData::GetInitialDataTypeDataSize`, Xbox PDB) and
/// `LoadVariableSizedValueFromBuffer(buffer, &out)` (cdecl, Xbox PDB).
pub(crate) const UNLOADED_FORM_BUFFER_SIZE: u32 = 0x0086_63e0;
pub(crate) const UNLOADED_FORM_BUFFER_INITIALIZE: u32 = 0x0086_65d0;
pub(crate) const INITIAL_DATA_TYPE_SIZE: u32 = 0x0084_e800;
pub(crate) const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0084_6080;
/// The hash map base vtable `0107f4e8` of the expiry queue, and the base
/// destructor body `(this)` of the vtable `0107f4c8` base.
pub(crate) const EXPIRY_QUEUE_BASE_VTABLE: u32 = 0x0107_f4e8;
pub(crate) const HASH_MAP_BASE_DESTRUCT: u32 = 0x0084_b720;
/// Called on the map itself by both map destructors before the base
/// destructor (also `CHANGED_FORM_ID_MAP_FLUSH`).
pub(crate) const MAP_CLEAR_ITEMS: u32 = 0x0043_8af0;
/// Frees a block from [`ALLOCATE_BYTES`] `(block)` (cdecl).
pub(crate) const FREE_BYTES: u32 = 0x00aa_10f0;
/// `(this = allocator at +0xc, item)`: frees a map item.
pub(crate) const MAP_ITEM_FREE: u32 = 0x006b_8310;
/// `BGSSaveLoadGlobalData::LoadGlobalData(type, buffer)` (Xbox PDB, cdecl).
pub(crate) const LOAD_GLOBAL_DATA: u32 = 0x0084_be40;
/// Allocates `size` bytes (the allocator the maps' bucket arrays use).
pub(crate) const ALLOCATE_BYTES: u32 = 0x00aa_1070;
/// `(this = array, reserved, size)`: the form buffer array's base setup.
pub(crate) const ARRAY_BASE_SETUP: u32 = 0x006b_3eb0;
/// Three words in `.data` (0x011f426c to 0x011f4274) the game copies into the
/// position and rotation it passes to reference slot `0x138`.
pub(crate) const DEFAULT_POSITION: u32 = 0x011f_426c;
/// `(this = &word, value)`: whether the word equals `value`.
pub(crate) const FLAGS_EQUAL: u32 = 0x0082_2510;

// Virtual slots (byte offsets) the translations use. The names are the Xbox
// PDB's; the three marked "not confirmed" sit at offsets where the PDB's
// slot has another name or signature, so only the use is described.
/// `BGSLoadGameBuffer` slot 4, `GetForm` (Xbox PDB): the form the buffer loads.
const BUFFER_SLOT_GET_FORM: u32 = 0x04;
/// `BGSLoadGameBuffer` slot 8, `GetReference` (Xbox PDB).
const BUFFER_SLOT_GET_REFERENCE: u32 = 0x08;
/// `TESForm::SaveGame` (Xbox PDB), called with a save buffer.
const FORM_SLOT_SAVE: u32 = 0x54;
/// `TESForm::LoadGame` (Xbox PDB), called with a load buffer.
const FORM_SLOT_LOAD: u32 = 0x5c;
/// `TESForm::InitLoadGame` (Xbox PDB) and `FinishLoadGame` (Xbox PDB): the two
/// steps of the unload pass, each called with a load buffer.
const FORM_SLOT_INIT_LOAD_GAME: u32 = 0x64;
const FORM_SLOT_FINISH_LOAD_GAME: u32 = 0x84;
/// `TESForm::Revert` (Xbox PDB), called with a load buffer.
const FORM_SLOT_REVERT: u32 = 0x70;
/// `TESForm::CheckSaveGame` (Xbox PDB), called with a save buffer.
const FORM_SLOT_CHECK_SAVE_GAME: u32 = 0x80;
/// `TESForm::IsReference` (Xbox PDB).
const FORM_SLOT_IS_REFERENCE: u32 = 0xf0;
/// `TESForm::IsActor` (Xbox PDB).
const FORM_SLOT_IS_ACTOR: u32 = 0x100;
/// `TESForm::GetObjectTypeName` (Xbox PDB): the name the warnings print.
const FORM_SLOT_NAME: u32 = 0x130;
/// Reference slot `0x1f4` (not confirmed: the Xbox PDB lists `SetRunsInLow`
/// here): called without arguments, its result is the last argument of
/// `00852c00`.
const REFERENCE_SLOT_PLACE: u32 = 0x1f4;
/// Reference slot `0x244` (not confirmed: the Xbox PDB lists
/// `MovetoMiddleLow` on the player here), called without arguments.
const REFERENCE_SLOT_RELEASE: u32 = 0x244;
/// Player slot `0x228` (not confirmed: the Xbox PDB lists `IsDead` here),
/// called with `0`.
const PLAYER_SLOT_RESET: u32 = 0x228;
/// `~TESForm` (Xbox PDB), the deleting destructor, called with `1`.
const FORM_SLOT_DELETE: u32 = 0x10;
/// Form slot `0x128` (not confirmed: the Xbox PDB lists `Activate` here and
/// `SetFormID` at `0x12c`): called with a new form id from the data handler.
const FORM_SLOT_SET_NEW_ID: u32 = 0x128;
/// Reference slot `0x138` (not confirmed: the Xbox PDB lists
/// `VoiceSoundFunction` here): `(&position, &rotation, &location, 0)`, true
/// when it filled them in.
const REFERENCE_SLOT_LOCATION: u32 = 0x138;
/// Reference slots `0x224` and `0x324` (not confirmed: the Xbox PDB lists
/// `SetParentCell` at `0x224`), called without arguments by `00849a90`.
const REFERENCE_SLOT_UNLOAD_TEST_A: u32 = 0x224;
const REFERENCE_SLOT_UNLOAD_TEST_B: u32 = 0x324;
/// Slot `0xc` of the object at +0x100 of an actor's `004181e0` result, called
/// with `0x10`; it returns its value in `ST0`.
const PROCESS_SLOT_VALUE: u32 = 0x0c;

layout! {
    /// `BGSSaveLoadGame` (Xbox PDB), 0x24C bytes: the object that writes and
    /// reads the save body.
    pub struct BGSSaveLoadGame: 0x24C {
        /// `pChangesMap` (Xbox PDB): `BGSSaveLoadChangesMap*`.
        0x000 pChangesMap: Ptr,
        /// `pOldChangesMap` (Xbox PDB): `BGSSaveLoadChangesMap*`.
        0x004 pOldChangesMap: Ptr,
        /// `pFormIDMap` (Xbox PDB): `BGSSaveLoadFormIDMap*`.
        0x008 pFormIDMap: Ptr,
        /// `pWorldspaceFormIDMap` (Xbox PDB): `BGSSaveLoadFormIDMap*`.
        0x00C pWorldspaceFormIDMap: Ptr,
        /// `pReferencesMap` (Xbox PDB): `BGSSaveLoadReferencesMap*`.
        0x010 pReferencesMap: Ptr,
        /// `pQueuedSubBuffersMap` (Xbox PDB).
        0x014 pQueuedSubBuffersMap: Ptr,
        /// `pChangedFormIDMap` (Xbox PDB): `NiTMap<unsigned int,unsigned int>*`.
        0x018 pChangedFormIDMap: Ptr,
        /// `pHistory` (Xbox PDB): `BGSSaveLoadHistory*`.
        0x01C pHistory: Ptr,
        /// `pReconstructForms` (Xbox PDB):
        /// `BGSReconstructFormsInAllFilesMap*`.
        0x020 pReconstructForms: Ptr,
        /// `FormBufferArray` (Xbox PDB): `BSSimpleArray<BGSLoadFormBuffer *,1024>`
        /// (0x10 bytes, embedded).
        0x024 FormBufferArray: Inline<crate::types::BSSimpleArray>,
        /// `QueuedInitPackageLocationsActorMap` (Xbox PDB):
        /// `NiTMap<unsigned int,Actor *>` (0x10 bytes, embedded).
        0x034 QueuedInitPackageLocationsActorMap: Inline<()>,
        /// `cFileIndexArray` (Xbox PDB): `u8[255]`, the save's plugin index to
        /// the loaded plugin index (0xff when the plugin is missing).
        0x044 cFileIndexArray: u8,
        /// `cReverseFileIndexArray` (Xbox PDB): `u8[255]`, loaded index to
        /// save index.
        0x143 cReverseFileIndexArray: u8,
        /// `iGlobalFlags` (Xbox PDB). Bit 0 is set and cleared by
        /// `007d6bd0`, bit 1 (2) is set while a load runs (`0042ce10` tests
        /// it), bit 2 (4) by `00847d00` while a save runs, bit 3 (8) while
        /// the unload pass runs, bit 7 (0x80) is tested by `00848cf0`.
        0x244 iGlobalFlags: u32,
        /// `cCurrentMinorVersion` (Xbox PDB): the minor version of the save
        /// being loaded (0xff when none).
        0x248 cCurrentMinorVersion: u8,
    }
}

layout! {
    /// `BGSLoadGameBuffer` (Xbox PDB), 0x10 bytes (the base of
    /// `BGSLoadFormBuffer`).
    pub struct BGSLoadGameBuffer: 0x10 {
        /// `pBuffer` (Xbox PDB).
        0x04 pBuffer: Ptr,
        /// `iBufferSize` (Xbox PDB).
        0x08 iBufferSize: u32,
        /// `iBufferPosition` (Xbox PDB).
        0x0C iBufferPosition: u32,
    }
}

layout! {
    /// `BGSLoadFormBuffer` (Xbox PDB), 0x30 bytes: the base is a
    /// `BGSLoadGameBuffer`. Only the fields this unit touches.
    pub struct BGSLoadFormBuffer: 0x30 {
        /// `iFlags` (Xbox PDB). Bit 0 (1): the form is to be skipped
        /// (`00848d90`, `00848dd0`); bit 2 (4): set by `00849240`; bit 3 (8):
        /// set by `0042ce50`; bit 4 (0x10): tested by `00849590`.
        0x28 iFlags: u32,
    }
}

layout! {
    /// `TESDataHandler` field this unit writes.
    pub struct TESDataHandler: 0x630 {
        /// A byte `00848df0` sets: nonzero while a load rebuilds forms.
        0x621 bDontRemoveIDs: u8,
    }
}

layout! {
    /// The save body header the game writes after the plugin list: 0x6e
    /// bytes, mostly `u32` file positions. `SaveGame` clears it, writes it,
    /// fills it in while it saves the forms and writes it again at the same
    /// place; `LoadGame` reads it first. The Xbox PDB has no type for it; the
    /// names say what the two functions do with each word.
    pub struct SaveBodyHeader: 0x6E {
        /// File position of the form id maps (stored after the changed
        /// forms; `LoadGame` seeks to it before loading both maps).
        0x00 iFormIDMapsPosition: u32,
        /// File position after the form id maps (`SaveGame`).
        0x04 iAfterFormIDMapsPosition: u32,
        /// File position right after the header (`SaveGame`).
        0x08 iFormsPosition: u32,
        /// File position after the first global block (`SaveGame`).
        0x0C iAfterFirstBlockPosition: u32,
        /// File position before the second global block (`SaveGame`).
        0x10 iBeforeSecondBlockPosition: u32,
        /// What the first global block save (`00847d70`) returned;
        /// `LoadGame` passes it to `0084bc20` after the global reset.
        0x14 iFirstBlockSize: u32,
        /// What the second global block save (`00847d90`) returned;
        /// `LoadGame` passes it to `0084bc20` at the end.
        0x18 iSecondBlockSize: u32,
        /// Number of forms in the save (`SaveGame` counts them; `LoadGame`
        /// loops that many times).
        0x1C iFormCount: u32,
        /// Offset `LoadGame` seeks over after the form passes.
        0x20 iTrailerOffset: u32,
    }
}

// ---- Helpers ---------------------------------------------------------------

/// `new` of `size` bytes, then the constructor at `constructor` with `extra`
/// words after `this`, when the allocation worked; null otherwise (the
/// `if (p) new T(...)` the compiler wrote for each object).
pub(crate) fn new_and_construct(e: &mut Engine, size: u32, constructor: u32, extra: &[u32]) -> Ptr {
    let block: Ptr = e.call(OPERATOR_NEW, &args![size]).ptr();
    if block.is_null() {
        return Ptr::NULL;
    }
    let mut words = vec![block.addr()];
    words.extend_from_slice(extra);
    e.call(constructor, &words).ptr()
}

/// A temporary `BGSChangeFlags` holding `value`, as the game builds one on
/// its stack to pass it by value: `008c71b0(temp, value)`. Returns the word.
pub(crate) fn change_flags(e: &mut Engine, value: u32) -> u32 {
    e.with_stack(4, |e, temp| {
        e.call(CHANGE_FLAGS_STORE, &args![temp, value]);
        e.mem.u32(temp.addr())
    })
}

/// Deletes an owned object through the scalar deleting destructor at
/// `destructor` (`this`, 1), when it is not null.
fn delete_with(e: &mut Engine, object: Ptr, destructor: u32) {
    if !object.is_null() {
        e.call(destructor, &args![object, 1u32]);
    }
}

/// Deletes an owned object through its virtual destructor (slot 0, flag 1).
fn delete_virtual(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.vcall(object.addr(), 0, &args![1u32]);
    }
}

/// Reads the `u32` at `address` through the game's getter `00559450`.
fn read_word(e: &mut Engine, address: u32) -> u32 {
    e.call(READ_WORD, &args![address]).u32()
}

/// Sets or clears `mask` in the word at `address` (`on` set, otherwise clear).
fn update_flag(e: &mut Engine, address: u32, mask: u32, on: bool) {
    let flags = e.mem.u32(address);
    e.mem
        .set_u32(address, if on { flags | mask } else { flags & !mask });
}

// Translated from 00846f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::BGSSaveLoadGame` (Xbox PDB): sets up the form buffer
/// array and the package location map, then allocates and constructs the
/// eight owned objects (changes map, two form id maps, references map,
/// queued sub buffers map, changed form id map, history, reconstruct map)
/// and fills both plugin index arrays with `0xff`. Returns `this`.
pub fn bgssaveloadgame_bgssaveloadgame(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
) -> Ptr<BGSSaveLoadGame> {
    e.set(this, BGSSaveLoadGame::pOldChangesMap, Ptr::NULL);
    e.call(FORM_BUFFER_ARRAY_CONSTRUCT, &args![this.byte_add(0x24)]);
    e.call(
        PACKAGE_LOCATION_MAP_CONSTRUCT,
        &args![this.byte_add(0x34), 0x25u32],
    );
    e.set(this, BGSSaveLoadGame::iGlobalFlags, 0);
    e.set(this, BGSSaveLoadGame::cCurrentMinorVersion, 0xff);
    e.call(GAME_SET_FLAG_ONE, &args![this, 0u32]);

    let changes_map = new_and_construct(e, 0x10, CHANGES_MAP_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pChangesMap, changes_map);
    let form_id_map = new_and_construct(e, 0x24, FORM_ID_MAP_CONSTRUCT, &[0x13af]);
    e.set(this, BGSSaveLoadGame::pFormIDMap, form_id_map);
    let worldspace_map = new_and_construct(e, 0x24, FORM_ID_MAP_CONSTRUCT, &[0x25]);
    e.set(this, BGSSaveLoadGame::pWorldspaceFormIDMap, worldspace_map);
    let references_map = new_and_construct(e, 0x30, REFERENCES_MAP_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pReferencesMap, references_map);
    let sub_buffers_map = new_and_construct(e, 0x30, QUEUED_SUB_BUFFERS_MAP_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pQueuedSubBuffersMap, sub_buffers_map);
    let changed_id_map = new_and_construct(e, 0x10, CHANGED_FORM_ID_MAP_CONSTRUCT, &[0x25]);
    e.set(this, BGSSaveLoadGame::pChangedFormIDMap, changed_id_map);
    let history = new_and_construct(e, 0x10, HISTORY_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pHistory, history);
    let reconstruct = new_and_construct(e, 0x34, RECONSTRUCT_FORMS_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pReconstructForms, reconstruct);

    e.call(MEMORY_SET, &args![this.byte_add(0x44), 0xffu32, 0xffu32]);
    e.call(MEMORY_SET, &args![this.byte_add(0x143), 0xffu32, 0xffu32]);
    this
}

// Translated from 008471f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::~BGSSaveLoadGame` body (the Xbox PDB class; the linker
/// emitted this body without a name): deletes the changes map (through its
/// virtual destructor), the two form id maps, the references map, the queued
/// sub buffers map, the changed form id map (virtual), the history and the
/// reconstruct map (virtual), then destroys the package location map and the
/// form buffer array. The old changes map at +4 is not deleted.
pub fn fn_008471f0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) {
    let changes_map = e.get(this, BGSSaveLoadGame::pChangesMap);
    delete_virtual(e, changes_map);
    let form_id_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
    delete_with(e, form_id_map, 0x0084_73c0);
    let worldspace_map = e.get(this, BGSSaveLoadGame::pWorldspaceFormIDMap);
    delete_with(e, worldspace_map, 0x0084_73c0);
    let references_map = e.get(this, BGSSaveLoadGame::pReferencesMap);
    delete_with(e, references_map, 0x0084_73f0);
    let sub_buffers_map = e.get(this, BGSSaveLoadGame::pQueuedSubBuffersMap);
    delete_with(e, sub_buffers_map, 0x0084_7420);
    let changed_id_map = e.get(this, BGSSaveLoadGame::pChangedFormIDMap);
    delete_virtual(e, changed_id_map);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    delete_with(e, history, 0x0084_7450);
    let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
    delete_virtual(e, reconstruct);
    e.call(PACKAGE_LOCATION_MAP_DESTRUCT, &args![this.byte_add(0x34)]);
    e.call(FORM_BUFFER_ARRAY_DESTRUCT, &args![this.byte_add(0x24)]);
}

/// The shape of the four scalar deleting destructors: run the destructor body
/// at `body` on `this`, then free the block when bit 0 of `flags` is set.
fn scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32, body: u32) -> Ptr {
    e.call(body, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008473c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BGSSaveLoadFormIDMap`: runs the body
/// `00846be0`, then deletes the block when `flags & 1`. Returns `this`.
pub fn fn_008473c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, FORM_ID_MAP_DESTRUCT)
}

// Translated from 008473f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BGSSaveLoadReferencesMap`: body
/// `00852ae0`, then delete when `flags & 1`. Returns `this`.
pub fn fn_008473f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, REFERENCES_MAP_DESTRUCT)
}

// Translated from 00847420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the queued sub buffers map: body `00865220`,
/// then delete when `flags & 1`. Returns `this`.
pub fn fn_00847420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, QUEUED_SUB_BUFFERS_MAP_DESTRUCT)
}

// Translated from 00847450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BGSSaveLoadHistory`: body `0084df30`, then
/// delete when `flags & 1`. Returns `this`.
pub fn fn_00847450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, HISTORY_DESTRUCT)
}

// Translated from 00847480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::Create` (Xbox PDB): allocates and constructs the game
/// object (null when the allocation fails), stores it in `[0x011ddf38]`, and
/// runs the follow-up initializer `008535e0`.
pub fn bgssaveloadgame_create(e: &mut Engine) {
    let game = new_and_construct(e, 0x24c, GAME_CONSTRUCT, &[]);
    e.set_global(SAVE_LOAD_GAME, game.addr());
    e.call(GAME_CREATE_FOLLOW_UP, &args![]);
}

// Translated from 00847500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the game object in `[0x011ddf38]` through its scalar deleting
/// destructor (`00847540`) and clears the global.
pub fn fn_00847500(e: &mut Engine) {
    let game: Ptr = Ptr::new(e.global::<u32>(SAVE_LOAD_GAME));
    if !game.is_null() {
        fn_00847540(e, game, 1);
    }
    e.set_global(SAVE_LOAD_GAME, 0u32);
}

// Translated from 00847540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BGSSaveLoadGame`: runs `008471f0`, then
/// deletes the block when `flags & 1`. Returns `this`.
pub fn fn_00847540(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008471f0(e, this.cast());
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00847570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::GetCurrentMinorVersion` (Xbox PDB): the minor version
/// of the save being loaded.
pub fn bgssaveloadgame_get_current_minor_version(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) -> u8 {
    e.get(this, BGSSaveLoadGame::cCurrentMinorVersion)
}

// Translated from 00847590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::SavePluginList` (Xbox PDB): writes the number of
/// compiled files (one byte) and each file's name to `file` through a
/// `BGSSaveGameBuffer`.
pub fn bgssaveloadgame_save_plugin_list(e: &mut Engine, _this: Ptr<BGSSaveLoadGame>, file: Ptr) {
    let buffer = e.mem.alloc(0x14);
    e.call(SAVE_GAME_BUFFER_CONSTRUCT, &args![buffer]);
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let count = e
        .call(DATA_HANDLER_COMPILED_FILE_COUNT, &args![data_handler])
        .u8();
    let count_slot = e.mem.alloc(4);
    e.mem.set_u8(count_slot, count);
    e.call(
        SAVE_GAME_BUFFER_SAVE_BYTES,
        &args![buffer, count_slot, 1u32, 0u32],
    );
    for index in 0..count as u32 {
        let compiled = e
            .call(DATA_HANDLER_GET_COMPILED_FILE, &args![data_handler, index])
            .u32();
        let name = e.call(COMPILED_FILE_NAME, &args![compiled]).u32();
        e.call(SAVE_GAME_BUFFER_SAVE_STRING, &args![buffer, name, 0u32]);
    }
    e.call(SAVE_GAME_BUFFER_SAVE, &args![buffer, file]);
    e.call(SAVE_GAME_BUFFER_DESTRUCT, &args![buffer]);
    e.mem.free(count_slot);
    e.mem.free(buffer);
}

// Translated from 00847660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::LoadPluginList` (Xbox PDB): reads the plugin names of
/// the save and matches each, ignoring case, against the compiled files. It
/// fills `cFileIndexArray` (save index to loaded index, `0xff` for a missing
/// plugin, which also gets a warning) and `cReverseFileIndexArray`; both
/// arrays are reset to `0xff` first. Returns false when any plugin is
/// missing.
pub fn bgssaveloadgame_load_plugin_list(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    file: Ptr,
) -> bool {
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let loaded_count = e
        .call(DATA_HANDLER_COMPILED_FILE_COUNT, &args![data_handler])
        .u8() as u32;
    let buffer = e.mem.alloc(0x10);
    e.call(LOAD_GAME_BUFFER_CONSTRUCT, &args![buffer]);
    e.call(LOAD_GAME_BUFFER_LOAD, &args![buffer, file]);
    let mut all_found = true;
    e.call(MEMORY_SET, &args![this.byte_add(0x44), 0xffu32, 0xffu32]);
    e.call(MEMORY_SET, &args![this.byte_add(0x143), 0xffu32, 0xffu32]);
    let count_slot = e.mem.alloc(4);
    e.mem.set_u8(count_slot, 0);
    e.call(
        LOAD_GAME_BUFFER_LOAD_BYTES,
        &args![buffer, count_slot, 1u32],
    );
    let saved_count = e.mem.u8(count_slot) as u32;
    let name = e.mem.alloc(0x108);
    for saved_index in 0..saved_count {
        e.call(LOAD_GAME_BUFFER_LOAD_STRING, &args![buffer, name]);
        let mut found = false;
        for loaded_index in 0..loaded_count {
            let compiled = e
                .call(
                    DATA_HANDLER_GET_COMPILED_FILE,
                    &args![data_handler, loaded_index],
                )
                .u32();
            let compiled_name = e.call(COMPILED_FILE_NAME, &args![compiled]).u32();
            let different = e
                .call(STRING_COMPARE_IGNORE_CASE, &args![name, compiled_name])
                .u32();
            if different == 0 {
                e.mem
                    .set_u8(this.addr() + 0x44 + saved_index, loaded_index as u8);
                e.mem
                    .set_u8(this.addr() + 0x143 + loaded_index, saved_index as u8);
                found = true;
                break;
            }
        }
        if !found {
            all_found = false;
            e.mem.set_u8(this.addr() + 0x44 + saved_index, 0xff);
            e.call(SAVE_GAME_WARNING, &args![MISSING_PLUGIN_WARNING, name]);
        }
    }
    e.call(LOAD_GAME_BUFFER_DESTRUCT, &args![buffer]);
    e.mem.free(name);
    e.mem.free(count_slot);
    e.mem.free(buffer);
    all_found
}

// Translated from 00847850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::SaveGame` (Xbox PDB): writes the body of the save to
/// `file`.
///
/// Order of the file: the minor version byte, the plugin list, the 0x6e byte
/// header (cleared, written here, rewritten at the end), the first global
/// block, every changed form (an unloaded form through its
/// `BGSUnloadedFormBuffer`, a loaded one through a `BGSSaveFormBuffer`; a
/// form whose changes are all already known is skipped), the second global
/// block, the two form id maps and the history. The header then records the
/// positions and the number of forms. The saving flag (`00847d00`) is held
/// for the duration.
pub fn bgssaveloadgame_save_game(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, file: Ptr) {
    e.call(GAME_SET_SAVING_FLAG, &args![this, 1u32]);
    let name = e.mem.alloc(0x104);
    let version = e.mem.alloc(0x104);
    let manager = e.global::<u32>(MANAGER_INSTANCE);
    // `006815c0` hands back the file as its name (the destination buffer the
    // game pushed before it stays on the stack for the next call).
    let file_name = e.call(THIS_IDENTITY, &args![file]).u32();
    e.call(MANAGER_COPY_FILE_NAME, &args![manager, file_name, name]);
    e.call(MANAGER_GET_VERSION_INFO, &args![manager, version, 0x104u32]);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(
        HISTORY_ADD_NOTE,
        &args![history, SAVING_NOTE, name, version],
    );
    let minor_version = e.call(MANAGER_GET_MINOR_VERSION, &args![manager]).u8();
    let minor_slot = e.mem.alloc(4);
    e.mem.set_u8(minor_slot, minor_version);
    e.call(SAVE_DATA_TO_FILE, &args![file, minor_slot, 1u32]);
    bgssaveloadgame_save_plugin_list(e, this, file);
    let header_position = e.call(FILE_GET_POSITION, &args![file]).u32();
    let header: Ptr<SaveBodyHeader> = Ptr::new(e.mem.alloc(0x6e));
    fn_00847d50(e, header.cast());
    e.call(FILE_WRITE, &args![file, header, 0x6eu32]);
    let forms_position = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.set(header, SaveBodyHeader::iFormsPosition, forms_position);
    let first_block = fn_00847d70(e, file);
    e.set(header, SaveBodyHeader::iFirstBlockSize, first_block);
    let after_first_block = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.set(
        header,
        SaveBodyHeader::iAfterFirstBlockPosition,
        after_first_block,
    );

    let buffer = e.mem.alloc(0x24);
    e.call(SAVE_FORM_BUFFER_CONSTRUCT, &args![buffer]);
    let position_slot = e.mem.alloc(4);
    let key_slot = e.mem.alloc(4);
    let value_slot = e.mem.alloc(4);
    let word_slot = e.mem.alloc(4);
    let old_flags_slot = e.mem.alloc(4);
    let known_flags_slot = e.mem.alloc(4);
    let map = e.get(this, BGSSaveLoadGame::pChangesMap);
    let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    e.mem.set_u32(position_slot, first);
    while e.mem.u32(position_slot) != 0 {
        e.mem.set_u32(key_slot, 0);
        e.mem.set_u32(value_slot, 0);
        let map = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(MAP_NEXT, &args![map, position_slot, key_slot, value_slot]);
        let key = e.mem.u32(key_slot);
        let value = e.mem.u32(value_slot);

        // An entry with a stored buffer is an unloaded form.
        if read_word(e, value + 4) != 0 {
            let flags = e.mem.u32(value);
            e.call(
                UNLOADED_FORM_BUFFER_SAVE,
                &args![value + 4, file, key, flags],
            );
            let count = e.get(header, SaveBodyHeader::iFormCount);
            e.set(header, SaveBodyHeader::iFormCount, count + 1);
            continue;
        }

        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        if form == 0 {
            let flags = read_word(e, value);
            e.call(
                SAVE_GAME_WARNING,
                &args![SAVE_FORM_MISSING_WARNING, key, flags],
            );
            continue;
        }

        e.call(SAVE_FORM_BUFFER_RESET, &args![buffer]);
        let minor = e.call(MANAGER_GET_MINOR_VERSION, &args![manager]).u32();
        let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
        let flags = e.mem.u32(value);
        e.call(
            SAVE_FORM_BUFFER_SET_HEADER,
            &args![buffer, key, flags, form_type, minor],
        );
        e.call(SAVE_FORM_BUFFER_SET_FORM, &args![buffer, form]);
        e.vcall(form, FORM_SLOT_CHECK_SAVE_GAME, &args![buffer]);

        let value_flags = read_word(e, value);
        let header_flags = e
            .call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, word_slot])
            .u32();
        fn_00847cd0(
            e,
            Ptr::new(header_flags),
            Ptr::new(old_flags_slot),
            value_flags,
        );
        if read_word(e, old_flags_slot) != 0 {
            let map = e.get(this, BGSSaveLoadGame::pChangesMap);
            let flags = e.mem.u32(old_flags_slot);
            e.call(CHANGES_MAP_ADD_FLAGS, &args![map, key, flags]);
        }

        let header_flags = e
            .call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, word_slot])
            .u32();
        let header_flags = read_word(e, header_flags);
        fn_00847cd0(e, Ptr::new(value), Ptr::new(known_flags_slot), header_flags);
        if read_word(e, known_flags_slot) != 0 {
            let map = e.get(this, BGSSaveLoadGame::pChangesMap);
            let flags = e.mem.u32(known_flags_slot);
            let known = e
                .call(CHANGES_MAP_CLEAR_FLAGS, &args![map, key, flags])
                .bool();
            if known {
                continue;
            }
        }

        let flags = e.mem.u32(value);
        e.call(SAVE_INITIAL_DATA, &args![buffer, form, flags]);
        e.vcall(form, FORM_SLOT_SAVE, &args![buffer]);
        e.call(SAVE_FORM_BUFFER_SAVE, &args![buffer, file]);
        let count = e.get(header, SaveBodyHeader::iFormCount);
        e.set(header, SaveBodyHeader::iFormCount, count + 1);
    }

    let before_second_block = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.set(
        header,
        SaveBodyHeader::iBeforeSecondBlockPosition,
        before_second_block,
    );
    let second_block = fn_00847d90(e, file);
    e.set(header, SaveBodyHeader::iSecondBlockSize, second_block);
    let maps_position = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.set(header, SaveBodyHeader::iFormIDMapsPosition, maps_position);
    let form_id_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
    e.call(FORM_ID_MAP_SAVE, &args![form_id_map, file]);
    let worldspace_map = e.get(this, BGSSaveLoadGame::pWorldspaceFormIDMap);
    e.call(FORM_ID_MAP_SAVE, &args![worldspace_map, file]);
    let after_maps = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.set(header, SaveBodyHeader::iAfterFormIDMapsPosition, after_maps);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(
        HISTORY_ADD_NOTE,
        &args![history, FINISHED_SAVING_NOTE, name],
    );
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(HISTORY_SAVE, &args![history, file]);
    e.call(FILE_SEEK_SET, &args![file, header_position]);
    e.call(FILE_WRITE, &args![file, header, 0x6eu32]);
    e.call(GAME_SET_SAVING_FLAG, &args![this, 0u32]);
    e.call(SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);

    for block in [
        known_flags_slot,
        old_flags_slot,
        word_slot,
        value_slot,
        key_slot,
        position_slot,
        buffer,
        header.addr(),
        minor_slot,
        version,
        name,
    ] {
        e.mem.free(block);
    }
}

// Translated from 00847cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds in `out` the flags of `*this` without the bits of `mask`:
/// `008c71b0(out, ~mask & *this)`. Returns `out`.
pub fn fn_00847cd0(e: &mut Engine, this: Ptr, out: Ptr, mask: u32) -> Ptr {
    let flags = !mask & e.mem.u32(this.addr());
    e.call(CHANGE_FLAGS_STORE, &args![out, flags]);
    out
}

// Translated from 00847d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 0x6e byte save body header at `this` (`memset(this, 0, 0x6e)`)
/// and returns it.
pub fn fn_00847d50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMORY_SET, &args![this, 0u32, 0x6eu32]);
    this
}

// Translated from 00847d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the first block of global data: `0084bb50(file, 0, 0xc)` (cdecl).
/// Returns what it returns.
pub fn fn_00847d70(e: &mut Engine, file: Ptr) -> u32 {
    e.call(SAVE_GLOBAL_DATA, &args![file, 0u32, 0xcu32]).u32()
}

// Translated from 00847d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the second block of global data: `0084bb50(file, 1000, 1001)`
/// (cdecl). Returns what it returns.
pub fn fn_00847d90(e: &mut Engine, file: Ptr) -> u32 {
    e.call(SAVE_GLOBAL_DATA, &args![file, 1000u32, 1001u32])
        .u32()
}

// Translated from 00847df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::LoadGame` (Xbox PDB): reads the body of a save from
/// `file`. Returns false when it fails: a plugin of the save is missing and
/// `quiet` is not set (nothing has been touched but the history), or a form
/// fails to load or the load was cancelled during the form pass (then the
/// load flag is cleared and the form buffers are released).
///
/// Order: a new history for the load; the file name and version note; the
/// plugin list; the old history is dropped and the references map reset; the
/// header is read; the form id maps and the history are loaded from the
/// positions in the header; the global data are loaded with a fresh changes
/// map; every form header is read, matched with its live form and given its
/// old change flags (first pass, buffers kept in the form buffer array); the
/// changes are applied (`00848e70`); then the second pass loads each form's
/// data (`0084f330`, then `TESForm::LoadGame`); the trailing
/// global block, the unload pass (`008492b0`) and the closing notes follow.
///
/// The C++ exception frames and the cookie check are not translated.
pub fn bgssaveloadgame_load_game(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    file: Ptr,
    quiet: bool,
) -> bool {
    let name = e.mem.alloc(0x104);
    let version = e.mem.alloc(0x104);
    let save_name = e.mem.alloc(0x104);
    let header: Ptr<SaveBodyHeader> = Ptr::new(e.mem.alloc(0x6e));
    let slots = e.mem.alloc(0x40);
    let result = load_game_body(
        e, this, file, quiet, name, version, save_name, header, slots,
    );
    for block in [slots, header.addr(), save_name, version, name] {
        e.mem.free(block);
    }
    result
}

/// The body of [`bgssaveloadgame_load_game`], with its stack locals passed in
/// (`slots` is a scratch block of 16 words).
#[allow(clippy::too_many_arguments)]
fn load_game_body(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    file: Ptr,
    quiet: bool,
    name: u32,
    version: u32,
    save_name: u32,
    header: Ptr<SaveBodyHeader>,
    slots: u32,
) -> bool {
    // Scratch words of the frame.
    let header_flags_slot = slots; // the word `00428110` copies out of a buffer
    let old_flags_slot = slots + 4;
    let new_flags_slot = slots + 8;
    let combined_flags_slot = slots + 12;
    let buffer_flags_slot = slots + 16;
    let form_slot = slots + 20;
    let unloaded_buffer_slot = slots + 24;
    let zero_slot = slots + 28;
    let buffer_slot = slots + 32;
    let array = this.byte_add(0x24);
    let manager = e.global::<u32>(MANAGER_INSTANCE);
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let player = e.global::<u32>(PLAYER);

    let previous_history = e.get(this, BGSSaveLoadGame::pHistory);
    let history = new_and_construct(e, 0x10, HISTORY_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pHistory, history);
    // `006815c0` hands back the file as its name (the destination buffer the
    // game pushed before it stays on the stack for the next call).
    let file_name = e.call(THIS_IDENTITY, &args![file]).u32();
    e.call(MANAGER_COPY_FILE_NAME, &args![manager, file_name, name]);
    e.call(MANAGER_GET_VERSION_INFO, &args![manager, version, 0x104u32]);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(
        HISTORY_ADD_NOTE,
        &args![history, LOADING_NOTE, name, version],
    );
    e.call(
        LOAD_DATA_FROM_FILE,
        &args![file, this.byte_add(0x248), 1u32],
    );
    let plugins_found = bgssaveloadgame_load_plugin_list(e, this, file);
    if !plugins_found && !quiet {
        let history = e.get(this, BGSSaveLoadGame::pHistory);
        if !history.is_null() {
            fn_00847450(e, history, 1);
        }
        e.set(this, BGSSaveLoadGame::pHistory, previous_history);
        e.set(this, BGSSaveLoadGame::cCurrentMinorVersion, 0xff);
        fn_00848e40(e, 1);
        return false;
    }

    if !previous_history.is_null() {
        fn_00847450(e, previous_history, 1);
    }
    let references_map = e.get(this, BGSSaveLoadGame::pReferencesMap);
    e.call(REFERENCES_MAP_RESET, &args![references_map]);
    e.call(MANAGER_START_LOAD, &args![manager]);
    let io_manager = e.global::<u32>(IO_MANAGER);
    e.call(IO_MANAGER_PAUSE, &args![io_manager]);
    fn_00848ca0(e, this, true);
    fn_00848e10(e, Ptr::new(player));
    fn_00847d50(e, header.cast());
    e.call(FILE_READ, &args![file, header, 0x6eu32]);
    let forms_position = e.call(FILE_GET_POSITION, &args![file]).u32();
    let maps_position = e.get(header, SaveBodyHeader::iFormIDMapsPosition);
    e.call(FILE_SEEK_SET, &args![file, maps_position]);

    let previous_form_id_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
    let form_id_map = new_and_construct(e, 0x24, FORM_ID_MAP_CONSTRUCT, &[0x13af]);
    e.set(this, BGSSaveLoadGame::pFormIDMap, form_id_map);
    let form_id_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
    e.call(FORM_ID_MAP_LOAD, &args![form_id_map, file]);
    let worldspace_map = e.get(this, BGSSaveLoadGame::pWorldspaceFormIDMap);
    e.call(FORM_ID_MAP_LOAD, &args![worldspace_map, file]);
    let previous_history = e.get(this, BGSSaveLoadGame::pHistory);
    let history = new_and_construct(e, 0x10, HISTORY_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pHistory, history);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(HISTORY_LOAD, &args![history, file]);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(HISTORY_ADD_HISTORY, &args![history, previous_history]);
    if !previous_history.is_null() {
        fn_00847450(e, previous_history, 1);
    }

    e.call(FILE_SEEK_SET, &args![file, forms_position]);
    // The old changes map is parked at +4 while a new one is built; the new
    // one stays current only for the two global loads below.
    let current_map = e.get(this, BGSSaveLoadGame::pChangesMap);
    e.set(this, BGSSaveLoadGame::pOldChangesMap, current_map);
    let new_map = new_and_construct(e, 0x10, CHANGES_MAP_CONSTRUCT, &[]);
    e.set(this, BGSSaveLoadGame::pChangesMap, new_map);
    e.call(GLOBAL_DATA_RESET, &args![0u32]);
    fn_00848df0(e, Ptr::new(data_handler), 1);
    let first_block = e.get(header, SaveBodyHeader::iFirstBlockSize);
    e.call(LOAD_GLOBAL_BLOCK, &args![file, first_block]);
    let new_map = e.get(this, BGSSaveLoadGame::pChangesMap);
    let old_map = e.get(this, BGSSaveLoadGame::pOldChangesMap);
    e.set(this, BGSSaveLoadGame::pChangesMap, old_map);
    e.set(this, BGSSaveLoadGame::pOldChangesMap, Ptr::NULL);

    let before_forms_position = e.call(FILE_GET_POSITION, &args![file]).u32();
    e.call(ARRAY_CLEAR, &args![array, 0u32]);
    let form_count = e.get(header, SaveBodyHeader::iFormCount);
    e.call(ARRAY_SET_RESERVED_SIZE, &args![array, form_count]);
    e.call(GAME_SET_FLAG_ONE, &args![this, 0u32]);

    // First pass: read every form header and match it with the live form.
    let mut index = 0u32;
    while index < e.get(header, SaveBodyHeader::iFormCount) {
        let buffer = new_and_construct(e, 0x30, LOAD_FORM_BUFFER_CONSTRUCT, &[]);
        e.mem.set_u32(buffer_slot, buffer.addr());
        e.call(LOAD_FORM_BUFFER_LOAD_HEADER, &args![buffer, file]);
        let saved_type = e.call(LOAD_FORM_BUFFER_HEADER_TYPE, &args![buffer]).u32();
        let form_id = e.call(BUFFER_FORM_ID, &args![buffer]).u32();
        e.call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, header_flags_slot]);
        let map = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_GET_FLAGS, &args![map, old_flags_slot, form_id]);
        e.call(
            CHANGES_MAP_GET_FLAGS,
            &args![new_map, new_flags_slot, form_id],
        );
        if read_word(e, new_flags_slot) != 0 {
            let flags = read_word(e, new_flags_slot);
            e.call(CHANGE_FLAGS_OR, &args![old_flags_slot, flags]);
            e.call(CHANGES_MAP_REMOVE, &args![new_map, form_id]);
        }
        let header_flags = e.mem.u32(header_flags_slot);
        e.call(
            CHANGES_MAP_SET_FLAGS,
            &args![new_map, form_id, header_flags],
        );
        let old_flags = e.mem.u32(old_flags_slot);
        e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, old_flags]);
        let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
        e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, form]);
        let initial_size = e
            .call(
                GAME_CHECK_INITIAL_DATA,
                &args![this, file, buffer, previous_form_id_map],
            )
            .u32();
        e.call(
            LOAD_FORM_BUFFER_SKIP_DATA,
            &args![buffer, file, initial_size],
        );
        let form = e.vcall(buffer.addr(), BUFFER_SLOT_GET_FORM, &args![]).u32();
        e.mem.set_u32(form_slot, form);
        let flags_ptr = e
            .call(
                LOAD_FORM_BUFFER_OLD_FLAGS,
                &args![buffer, buffer_flags_slot],
            )
            .u32();
        let old_flags = e.mem.u32(flags_ptr);
        e.mem.set_u32(old_flags_slot, old_flags);

        let mut remove_changes = false;
        if form == 0 {
            remove_changes = true;
        } else {
            let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
            let mismatch = form_type != saved_type
                || (e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool()
                    && e.call(BUFFER_GET_FORM, &args![form]).u32() == 0);
            if mismatch {
                if e.call(IS_DYNAMIC_FORM_ID, &args![data_handler, form_id])
                    .bool()
                {
                    e.call(GAME_CLEAR_FORM, &args![this, form]);
                    e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, 0u32]);
                    let none = change_flags(e, 0);
                    e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, none]);
                    remove_changes = true;
                } else {
                    if e.call(FORM_GET_TYPE, &args![form]).u32() != saved_type {
                        warn_type_mismatch(
                            e,
                            FORM_TYPE_MISMATCH_WARNING,
                            form,
                            form_id,
                            saved_type,
                        );
                    }
                    fn_00848d90(e, buffer.cast(), true);
                }
            } else {
                e.vcall(form, FORM_SLOT_REVERT, &args![buffer]);
                let old_flags = e.mem.u32(old_flags_slot);
                let header_flags = e.mem.u32(header_flags_slot);
                e.call(
                    COMBINE_CHANGE_FLAGS,
                    &args![
                        combined_flags_slot,
                        old_flags,
                        header_flags,
                        saved_type,
                        1u32
                    ],
                );
                if read_word(e, combined_flags_slot) != 0 {
                    e.mem.set_u32(form_slot, form);
                    let combined = e.mem.u32(combined_flags_slot);
                    let handled = e
                        .call(
                            GAME_HANDLE_UNREVERTIBLE_CHANGES,
                            &args![this, form_slot, combined, 0u32],
                        )
                        .bool();
                    if !handled {
                        let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
                        let form = e.mem.u32(form_slot);
                        e.call(RECONSTRUCT_ADD_FORM, &args![reconstruct, form, 0u32]);
                        e.call(LOAD_FORM_BUFFER_SET_LOADED, &args![buffer, 1u32]);
                    }
                }
                remove_changes = true;
            }
        }
        if remove_changes {
            let map = e.get(this, BGSSaveLoadGame::pChangesMap);
            e.call(CHANGES_MAP_REMOVE, &args![map, form_id]);
        }
        e.call(ARRAY_ADD, &args![array, buffer_slot]);
        index += 1;
    }

    fn_00848e70(e, this, new_map, previous_form_id_map);
    e.call(AFTER_LOAD_FINISH, &args![]);
    if !previous_form_id_map.is_null() {
        fn_008473c0(e, previous_form_id_map, 1);
    }
    fn_00848df0(e, Ptr::new(data_handler), 0);
    e.call(GAME_AFTER_LOAD_PASS, &args![this]);
    e.call(FILE_SEEK_SET, &args![file, before_forms_position]);

    // Second pass: load the data of every form.
    let mut index = 0u32;
    while index < e.get(header, SaveBodyHeader::iFormCount) {
        let element = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
        let buffer = Ptr::new(e.mem.u32(element));
        if fn_00848dd0(e, buffer.cast()) {
            let form_id = e.call(BUFFER_FORM_ID, &args![buffer]).u32();
            let map = e.get(this, BGSSaveLoadGame::pChangesMap);
            e.call(CHANGES_MAP_REMOVE, &args![map, form_id]);
            e.call(LOAD_FORM_BUFFER_LOAD_HEADER, &args![buffer, file]);
            e.call(LOAD_FORM_BUFFER_SKIP_DATA, &args![buffer, file, 0u32]);
            drop_form_buffer(e, array, index, buffer, zero_slot);
            index += 1;
            continue;
        }
        let form_id = e.call(BUFFER_FORM_ID, &args![buffer]).u32();
        let form = e.vcall(buffer.addr(), BUFFER_SLOT_GET_FORM, &args![]).u32();
        if form == 0 {
            e.call(UNLOADED_FORM_BUFFER_CONSTRUCT, &args![unloaded_buffer_slot]);
            e.call(
                UNLOADED_FORM_BUFFER_LOAD,
                &args![unloaded_buffer_slot, file],
            );
            let word_ptr = e
                .call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, header_flags_slot])
                .u32();
            let header_flags = e.mem.u32(word_ptr);
            let unloaded = e.mem.u32(unloaded_buffer_slot);
            let map = e.get(this, BGSSaveLoadGame::pChangesMap);
            e.call(
                CHANGES_MAP_STORE_BUFFER,
                &args![map, form_id, header_flags, unloaded],
            );
        } else {
            e.call(LOAD_FORM_BUFFER_LOAD_HEADER, &args![buffer, file]);
            let saved_type = e.call(LOAD_FORM_BUFFER_HEADER_TYPE, &args![buffer]).u32();
            let mut skip = false;
            if e.call(FORM_GET_TYPE, &args![form]).u32() != saved_type {
                warn_type_mismatch(e, FORM_TYPE_MISMATCH_WARNING, form, form_id, saved_type);
                skip = true;
            } else if e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool()
                && e.call(BUFFER_GET_FORM, &args![form]).u32() == 0
            {
                skip = true;
            }
            if skip {
                let map = e.get(this, BGSSaveLoadGame::pChangesMap);
                e.call(CHANGES_MAP_REMOVE, &args![map, form_id]);
                e.call(LOAD_FORM_BUFFER_SKIP_DATA, &args![buffer, file, 0u32]);
                drop_form_buffer(e, array, index, buffer, zero_slot);
                index += 1;
                continue;
            }
            let loaded = e
                .call(LOAD_FORM_BUFFER_LOAD_DATA, &args![buffer, file])
                .i32();
            if loaded < 0 || fn_00848cf0(e, this) {
                e.call(LOAD_FORM_BUFFER_SKIP_DATA, &args![buffer, file, 0u32]);
                drop_form_buffer(e, array, index, buffer, zero_slot);
                e.call(ARRAY_CLEAR, &args![array, 1u32]);
                e.vcall(player, PLAYER_SLOT_RESET, &args![0u32]);
                e.call(GAME_SET_FLAG_ONE, &args![this, 1u32]);
                fn_00848ca0(e, this, false);
                return false;
            }
            let word_ptr = e
                .call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, header_flags_slot])
                .u32();
            let header_flags = e.mem.u32(word_ptr);
            e.call(LOAD_INITIAL_DATA, &args![buffer, form, header_flags]);
            let loaded_form = e.vcall(buffer.addr(), BUFFER_SLOT_GET_FORM, &args![]).u32();
            if loaded_form != 0 && !fn_00848cf0(e, this) {
                e.call(FORM_MARK_LOADED, &args![form, 1u32]);
                e.vcall(form, FORM_SLOT_LOAD, &args![buffer]);
            }
        }
        bgsloadgamebuffer_delete_buffer(e, buffer.cast());
        index += 1;
    }

    let trailer_offset = e.get(header, SaveBodyHeader::iTrailerOffset);
    e.call(FILE_SEEK_CURRENT, &args![file, trailer_offset]);
    fn_00848d10(e);
    fn_008492b0(e, this, true);
    fn_00848d30(e);
    let second_block = e.get(header, SaveBodyHeader::iSecondBlockSize);
    e.call(LOAD_GLOBAL_BLOCK, &args![file, second_block]);
    e.set(this, BGSSaveLoadGame::cCurrentMinorVersion, 0xff);
    e.call(MANAGER_BUILD_SAVE_NAME, &args![save_name]);
    e.call(BSSTRING_SET, &args![manager + 0x30, save_name, 0u32]);
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(
        HISTORY_ADD_NOTE,
        &args![history, FINISHED_LOADING_NOTE, name],
    );
    e.call(GAME_SET_FLAG_ONE, &args![this, 1u32]);
    fn_00867a20_calendar(e);
    fn_00848ca0(e, this, false);
    true
}

/// `fn_00867a20` of `calendar.cpp` on the calendar object (the game passes
/// its address as `this`).
fn fn_00867a20_calendar(e: &mut Engine) {
    e.call(CALENDAR_MARK_STALE, &args![CALENDAR]);
}

/// The warning for a form whose saved type differs from its current type:
/// `SaveGameWarning(format, name, id, name of the saved type, name of the
/// current type)`.
fn warn_type_mismatch(e: &mut Engine, format: u32, form: u32, form_id: u32, saved_type: u32) {
    let current_type_name = e.call(FORM_TYPE_NAME, &args![form]).u32();
    let saved_type_name = e.call(TYPE_NAME_FOR, &args![saved_type & 0xff]).u32();
    let form_name = e.vcall(form, FORM_SLOT_NAME, &args![]).u32();
    e.call(
        SAVE_GAME_WARNING,
        &args![
            format,
            form_name,
            form_id,
            saved_type_name,
            current_type_name
        ],
    );
}

/// Releases a form buffer of the second pass: deletes it (`0081db60`, 1) when
/// it is not null and clears its slot in the form buffer array.
fn drop_form_buffer(e: &mut Engine, array: Ptr, index: u32, buffer: Ptr, zero_slot: u32) {
    if !buffer.is_null() {
        e.call(LOAD_FORM_BUFFER_DELETE, &args![buffer, 1u32]);
    }
    e.mem.set_u32(zero_slot, 0);
    e.call(ARRAY_SET_AT, &args![array, index, zero_slot]);
}

// Translated from 00848ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 1 (mask 2) of `[+0x244]`: the load is running.
pub fn fn_00848ca0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, on: bool) {
    update_flag(e, this.addr() + 0x244, 2, on);
}

// Translated from 00848cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 7 (mask 0x80) of `[+0x244]`.
pub fn fn_00848cf0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) -> bool {
    e.get(this, BGSSaveLoadGame::iGlobalFlags) & 0x80 != 0
}

// Translated from 00848d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the global data loader `0084bfb0(0, 0xc)` (cdecl).
pub fn fn_00848d10(e: &mut Engine) {
    e.call(LOAD_GLOBAL_DATA_A, &args![0u32, 0xcu32]);
}

// Translated from 00848d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the global data loader `0084c110(0, 0xc)` (cdecl).
pub fn fn_00848d30(e: &mut Engine) {
    e.call(LOAD_GLOBAL_DATA_B, &args![0u32, 0xcu32]);
}

// Translated from 00848d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameBuffer::DeleteBuffer` (Xbox PDB): frees the data block and
/// clears the pointer, the size and the position.
pub fn bgsloadgamebuffer_delete_buffer(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>) {
    let data = e.get(this, BGSLoadGameBuffer::pBuffer);
    e.call(OPERATOR_DELETE, &args![data]);
    e.set(this, BGSLoadGameBuffer::pBuffer, Ptr::NULL);
    e.set(this, BGSLoadGameBuffer::iBufferSize, 0);
    e.set(this, BGSLoadGameBuffer::iBufferPosition, 0);
}

// Translated from 00848d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 0 (mask 1) of a `BGSLoadFormBuffer`'s `iFlags`:
/// the form is to be skipped in the second pass.
pub fn fn_00848d90(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>, on: bool) {
    update_flag(e, this.addr() + 0x28, 1, on);
}

// Translated from 00848dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0 (mask 1) of a `BGSLoadFormBuffer`'s `iFlags`.
pub fn fn_00848dd0(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> bool {
    e.get(this, BGSLoadFormBuffer::iFlags) & 1 != 0
}

// Translated from 00848df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0x621 of the data handler (`bDontRemoveIDs`).
pub fn fn_00848df0(e: &mut Engine, this: Ptr<TESDataHandler>, value: u8) {
    e.set(this, TESDataHandler::bDontRemoveIDs, value);
}

// Translated from 00848e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +0xd68 of the player and the list at its +0x5fc
/// (`00470470`).
pub fn fn_00848e10(e: &mut Engine, player: Ptr) {
    // PlayerCharacter +0xd68 (a counter or pointer cleared before a load).
    e.mem.set_u32(player.addr() + 0xd68, 0);
    e.call(LIST_CLEAR, &args![player.byte_add(0x5fc)]);
}

// Translated from 00848e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `[0x011daac0]` exists, forwards `(0x20000000, value)`
/// to `0075f6f0` on it.
pub fn fn_00848e40(e: &mut Engine, value: u8) {
    let object = e.global::<u32>(MENU_OBJECT);
    if object != 0 {
        e.call(MENU_OBJECT_FORWARD, &args![object, 0x2000_0000u32, value]);
    }
}

// Translated from 00848e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the changes recorded by a load to the live forms and installs
/// `changes_map` as the current changes map (a new one is made when it is
/// null). For every entry of the current map: a live form with a dynamic id
/// (`>= 0xff000000`) is cleared and its entry removed from `changes_map`;
/// another live form is captured into a temporary `BGSLoadFormBuffer`
/// (`HandleUnrevertibleChanges` and the reconstruct map get a chance), and
/// the entry is added to `changes_map` when `00840f70` says so. An entry
/// without a live form and with a stored buffer of kind 6 (when
/// `form_id_map` is given, it is swapped in as `pFormIDMap` meanwhile) adds
/// the reference to the reconstruct map when its original cell is loaded.
/// Afterwards both reconstruct steps run, the old current map is deleted
/// (virtual destructor) and `changes_map` takes its place.
pub fn fn_00848e70(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, changes_map: Ptr, form_id_map: Ptr) {
    let mut map = changes_map;
    if map.is_null() {
        map = new_and_construct(e, 0x10, CHANGES_MAP_CONSTRUCT, &[]);
    }
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let world = e.global::<u32>(WORLD);
    let slots = e.mem.alloc(0x40);
    let position_slot = slots;
    let key_slot = slots + 4;
    let value_slot = slots + 8;
    let form_slot = slots + 12;
    let flags_slot = slots + 16;
    let entry_word_slot = slots + 20;
    let entry_out_slot = slots + 24;
    let cell_slot = slots + 28;
    let place_slot = slots + 32;
    let initial_data = e.mem.alloc(0x30);
    let buffer = e.mem.alloc(0x30);

    let current = e.get(this, BGSSaveLoadGame::pChangesMap);
    let first = e.call(MAP_FIRST_POSITION, &args![current]).u32();
    e.mem.set_u32(position_slot, first);
    while e.mem.u32(position_slot) != 0 {
        e.mem.set_u32(key_slot, 0);
        e.mem.set_u32(value_slot, 0);
        let current = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(
            MAP_NEXT,
            &args![current, position_slot, key_slot, value_slot],
        );
        let key = e.mem.u32(key_slot);
        let value = e.mem.u32(value_slot);
        let form = e.call(LOOKUP_FORM, &args![key]).u32();
        e.mem.set_u32(form_slot, form);
        if form != 0 {
            if e.call(IS_DYNAMIC_FORM_ID, &args![data_handler, key]).bool() {
                e.call(GAME_CLEAR_FORM, &args![this, form]);
                e.call(CHANGES_MAP_REMOVE, &args![map, key]);
                continue;
            }
            e.call(LOAD_FORM_BUFFER_CONSTRUCT, &args![buffer]);
            e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, form]);
            let stored_flags = e.mem.u32(value);
            e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, stored_flags]);
            fn_00849240(e, Ptr::new(buffer), true);
            e.vcall(form, FORM_SLOT_REVERT, &args![buffer]);
            let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
            let none = change_flags(e, 0);
            let stored_flags = e.mem.u32(value);
            e.call(
                COMBINE_CHANGE_FLAGS,
                &args![flags_slot, stored_flags, none, form_type, 0u32],
            );
            if read_word(e, flags_slot) != 0 {
                let flags = e.mem.u32(flags_slot);
                let handled = e
                    .call(
                        GAME_HANDLE_UNREVERTIBLE_CHANGES,
                        &args![this, form_slot, flags, 1u32],
                    )
                    .bool();
                let form_now = e.mem.u32(form_slot);
                if !handled && form_now != 0 {
                    let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
                    e.call(RECONSTRUCT_ADD_FORM, &args![reconstruct, form_now, 8u32]);
                }
            }
            let form_now = e.mem.u32(form_slot);
            if form_now != 0 && e.call(ENTRY_TEST, &args![value, form_now]).bool() {
                let stored_flags = e.mem.u32(value);
                e.call(CHANGES_MAP_ADD_FLAGS, &args![map, key, stored_flags]);
            }
            e.call(LOAD_FORM_BUFFER_DESTRUCT, &args![buffer]);
        } else if !form_id_map.is_null() {
            let stored_buffer = e.mem.u32(value + 4);
            e.mem.set_u32(entry_word_slot, stored_buffer);
            if read_word(e, entry_word_slot) != 0 {
                let stored_flags = e.mem.u32(value);
                let inner = fn_00849220(e, Ptr::new(entry_word_slot));
                let kind = e
                    .call(INITIAL_DATA_KIND, &args![key, inner, stored_flags])
                    .u32();
                if kind == 6 {
                    let previous_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
                    e.set(this, BGSSaveLoadGame::pFormIDMap, form_id_map);
                    let current = e.get(this, BGSSaveLoadGame::pChangesMap);
                    e.call(CHANGES_MAP_GET_ENTRY, &args![current, entry_out_slot, key]);
                    fn_00849280(e, Ptr::new(initial_data));
                    let advanced = e
                        .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry_out_slot])
                        .u32();
                    e.call(
                        LOAD_INITIAL_DATA_STRUCT,
                        &args![advanced, kind, initial_data],
                    );
                    e.mem.set_u32(cell_slot, 0);
                    e.mem.set_u32(place_slot, 0);
                    e.call(
                        GET_ORIGINAL_LOCATION,
                        &args![initial_data, cell_slot, place_slot],
                    );
                    let cell = e.mem.u32(cell_slot);
                    if cell != 0
                        && e.call(WORLD_IS_CELL_LOADED, &args![world, cell, 0u32])
                            .bool()
                    {
                        let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
                        e.call(
                            RECONSTRUCT_ADD_REFERENCE,
                            &args![reconstruct, cell, key, 8u32],
                        );
                    }
                    e.set(this, BGSSaveLoadGame::pFormIDMap, previous_map);
                }
            }
        }
    }
    let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
    e.call(RECONSTRUCT_FINISH_A, &args![reconstruct]);
    let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
    e.call(RECONSTRUCT_FINISH_B, &args![reconstruct]);
    let current = e.get(this, BGSSaveLoadGame::pChangesMap);
    delete_virtual(e, current);
    e.set(this, BGSSaveLoadGame::pChangesMap, map);
    for block in [buffer, initial_data, slots] {
        e.mem.free(block);
    }
}

// Translated from 00849220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the pointer at `*this` and calls `00853640` on it (`this` is the
/// address of a stored buffer word); returns what it returns.
pub fn fn_00849220(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.mem.u32(this.addr());
    e.call(STORED_BUFFER_TYPE, &args![inner]).u32()
}

// Translated from 00849240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 2 (mask 4) of a `BGSLoadFormBuffer`'s `iFlags`.
pub fn fn_00849240(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>, on: bool) {
    update_flag(e, this.addr() + 0x28, 4, on);
}

// Translated from 00849280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the identity cast `006815c0` on the members at +0x14 and +0x20 of the
/// 0x30 byte initial data structure and returns `this` (the constructor of
/// that structure, which has nothing else to set up).
pub fn fn_00849280(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(THIS_IDENTITY, &args![this.byte_add(0x14)]);
    e.call(THIS_IDENTITY, &args![this.byte_add(0x20)]);
    this
}

// Translated from 008492b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The unload pass that ends a load. Sets mask 8 of `[+0x244]` for the
/// duration. With `full` set, the I/O manager's word at +0x68 is set to 5
/// first. Each buffer in the form buffer array (non-null, with a form) gets
/// `TESForm::InitLoadGame`. With `full` set the world, the I/O manager and the
/// physics world of the player's cell are prepared (all islands of that
/// physics world deactivated). Then the queued sub buffers are flushed, and
/// every buffer's form gets `FinishLoadGame`; a buffer with bit 4 (0x10) set
/// (`00849590`) has its reference recorded as changed (`AddChange` with flags
/// 8), unloaded (`UnloadForm`), removed from its cell and handed to the
/// garbage collector. Every form is marked with `0046a010(form, 0)` and every
/// buffer deleted; finally the array is cleared and the changed form id map
/// flushed.
pub fn fn_008492b0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, full: bool) {
    let array = this.byte_add(0x24);
    fn_00849540(e, this, true);
    if full {
        // 008492cb: the I/O manager word is loaded here, after `00849540`.
        let io_manager = e.global::<u32>(IO_MANAGER);
        fn_008495b0(e, Ptr::new(io_manager));
    }
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
        let buffer = e.mem.u32(element);
        if buffer != 0 {
            let form = e.vcall(buffer, BUFFER_SLOT_GET_FORM, &args![]).u32();
            if form != 0 {
                e.vcall(form, FORM_SLOT_INIT_LOAD_GAME, &args![buffer]);
            }
        }
        index += 1;
    }
    if full {
        // 00849343..00849364 / 008493a0: each global word (`TES`, the I/O
        // manager, the player) is loaded at its call, after the forms'
        // `InitLoadGame` calls above.
        let world = e.global::<u32>(WORLD);
        e.call(WORLD_PREPARE, &args![world]);
        let io_manager = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_RESUME, &args![io_manager]);
        let io_manager = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_LOAD_QUEUED_PRIORITY, &args![io_manager]);
        let player = e.global::<u32>(PLAYER);
        let cell = e.call(REFERENCE_PARENT_CELL, &args![player]).u32();
        let physics = if cell != 0 {
            e.call(CELL_PHYSICS_WORLD, &args![cell]).u32()
        } else {
            0
        };
        if physics != 0 {
            e.call(PHYSICS_WORLD_DEACTIVATE_ALL_ISLANDS, &args![physics]);
        }
        let world = e.global::<u32>(WORLD);
        e.call(WORLD_FINISH, &args![world]);
    }
    let sub_buffers = e.get(this, BGSSaveLoadGame::pQueuedSubBuffersMap);
    e.call(QUEUED_SUB_BUFFERS_FLUSH, &args![sub_buffers]);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
        let buffer = e.mem.u32(element);
        if buffer != 0 {
            let form = e.vcall(buffer, BUFFER_SLOT_GET_FORM, &args![]).u32();
            if form != 0 {
                e.vcall(form, FORM_SLOT_FINISH_LOAD_GAME, &args![buffer]);
                if fn_00849590(e, Ptr::new(buffer)) {
                    let reference = e.vcall(buffer, BUFFER_SLOT_GET_REFERENCE, &args![]).u32();
                    let flags = change_flags(e, 8);
                    e.call(GAME_ADD_CHANGE, &args![this, reference, flags, 1u32]);
                    if e.vcall(reference, FORM_SLOT_IS_ACTOR, &args![]).bool()
                        && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![reference]).u32() != 0
                    {
                        e.vcall(reference, REFERENCE_SLOT_RELEASE, &args![]);
                    }
                    bgssaveloadgame_unload_form(e, this, Ptr::new(reference), true);
                    let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                    if cell != 0 {
                        e.call(CELL_REMOVE_REFERENCE, &args![cell, reference]);
                    }
                    e.call(GARBAGE_COLLECTOR_ADD, &args![reference]);
                }
                e.call(FORM_MARK_LOADED, &args![form, 0u32]);
            }
            e.call(LOAD_FORM_BUFFER_DELETE, &args![buffer, 1u32]);
        }
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![array, 1u32]);
    let changed_id_map = e.get(this, BGSSaveLoadGame::pChangedFormIDMap);
    e.call(CHANGED_FORM_ID_MAP_FLUSH, &args![changed_id_map]);
    fn_00849540(e, this, false);
}

// Translated from 00849540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 3 (mask 8) of `[+0x244]`: the unload pass is
/// running.
pub fn fn_00849540(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, on: bool) {
    update_flag(e, this.addr() + 0x244, 8, on);
}

// Translated from 00849590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 4 (mask 0x10) of a `BGSLoadFormBuffer`'s `iFlags`.
pub fn fn_00849590(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>) -> bool {
    e.get(this, BGSLoadFormBuffer::iFlags) & 0x10 != 0
}

// Translated from 008495b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at +0x68 of the I/O manager to 5.
pub fn fn_008495b0(e: &mut Engine, this: Ptr) {
    // IOManager +0x68
    e.mem.set_u32(this.addr() + 0x68, 5);
}

// Translated from 008495d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::LoadForm` (Xbox PDB): loads one form's data from the
/// buffer stored for its id in the changes map, outside a load (it does
/// nothing while the load flag, mask 2, is set). When the form's type
/// differs from the stored type a warning is logged and the entry removed.
/// Otherwise a load buffer is made from the stored buffer (the entry's stored
/// buffer pointer is cleared), given the entry's change flags as its header
/// change flags, passed to `LoadInitialData` and `TESForm::LoadGame`, then
/// its data freed and added to the form buffer array. The thread flag is
/// lowered while this runs (`004623f0`).
pub fn bgssaveloadgame_load_form(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr) {
    if e.call(GAME_LOADING_FLAG_TEST, &args![this]).bool() {
        return;
    }
    let form_id = e.call(FORM_ID_WORD, &args![form]).u32();
    let map = e.get(this, BGSSaveLoadGame::pChangesMap);
    let entry = e.call(CHANGES_MAP_ENTRY_FOR, &args![map, form_id]).u32();
    if entry == 0 || read_word(e, entry + 4) == 0 {
        return;
    }
    let stored_type = fn_00849220(e, Ptr::new(entry + 4));
    if e.call(FORM_GET_TYPE, &args![form]).u32() != stored_type {
        warn_type_mismatch(
            e,
            LOAD_ERROR_TYPE_MISMATCH_WARNING,
            form.addr(),
            form_id,
            stored_type,
        );
        let map = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_REMOVE, &args![map, form_id]);
        return;
    }
    let previous_thread_flag = e.call(SET_THREAD_FLAG, &args![this, 0u32]).u8();
    let load_buffer = e
        .call(UNLOADED_FORM_BUFFER_CREATE_LOAD_BUFFER, &args![entry + 4])
        .u32();
    e.call(UNLOADED_FORM_BUFFER_CLEAR, &args![entry + 4]);
    e.call(LOAD_FORM_BUFFER_SET_FORM, &args![load_buffer, form]);
    let header_flags = e.mem.u32(entry);
    e.call(
        LOAD_FORM_BUFFER_SET_HEADER_FLAGS,
        &args![load_buffer, header_flags],
    );
    e.call(FORM_MARK_LOADED, &args![form, 1u32]);
    let header_flags = e.mem.u32(entry);
    e.call(LOAD_INITIAL_DATA, &args![load_buffer, form, header_flags]);
    e.vcall(form.addr(), FORM_SLOT_LOAD, &args![load_buffer]);
    bgsloadgamebuffer_delete_buffer(e, Ptr::new(load_buffer));
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, load_buffer);
    e.call(ARRAY_ADD, &args![this.byte_add(0x24), slot]);
    e.mem.free(slot);
    e.call(SET_THREAD_FLAG, &args![this, previous_thread_flag]);
}

// Translated from 00849730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::UnloadForm` (Xbox PDB): records a form that is being
/// unloaded in the changes map. Skipped while a load runs (mask 2 of
/// `[+0x244]`) unless `force` is set, while the data handler is clearing data
/// (`004226e0`), or when `00849a90` says the form may not be unloaded.
/// An entry that already has a stored buffer logs a warning and stops.
/// Otherwise a `BGSSaveFormBuffer` captures the form (`CheckSaveGame`); when the
/// remaining change flags are all already known (`00845a80`) nothing more is
/// done. Else the initial data is saved (`SaveInitialData`), the form saves
/// itself (`SaveGame`) and the buffer is stored in the entry. For the
/// kinds 5 and 6 the reference is also recorded with its cell (when the cell
/// passes `00425fd0`) in the references map, or with its worldspace, or a
/// warning when it has neither; kind 6 also adds the id to the references
/// map.
///
/// The scope guard `(0x11, 1, file, 0x37e)` brackets the body; the
/// compiler's exception frame is not translated.
pub fn bgssaveloadgame_unload_form(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    form: Ptr,
    force: bool,
) {
    let guard = e.mem.alloc(8);
    e.call(
        GUARD_CONSTRUCT,
        &args![guard, 0x11u32, 1u32, SOURCE_PATH, 0x37eu32],
    );
    unload_form_body(e, this, form, force);
    e.call(GUARD_DESTRUCT, &args![guard]);
    e.mem.free(guard);
}

/// The body of [`bgssaveloadgame_unload_form`] between the scope guard's
/// construction and destruction.
fn unload_form_body(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr, force: bool) {
    if e.call(GAME_LOADING_FLAG_TEST, &args![this]).bool() && !force {
        return;
    }
    let data_handler = e.global::<u32>(DATA_HANDLER);
    if e.call(DATA_HANDLER_GET_CLEARING_DATA, &args![data_handler])
        .bool()
    {
        return;
    }
    if !e.call(GAME_CAN_UNLOAD_FORM, &args![this, form]).bool() {
        return;
    }
    let form_id = e.call(FORM_ID_WORD, &args![form]).u32();
    let map = e.get(this, BGSSaveLoadGame::pChangesMap);
    let entry = e.call(CHANGES_MAP_ENTRY_FOR, &args![map, form_id]).u32();
    if entry == 0 {
        return;
    }
    if read_word(e, entry + 4) != 0 {
        let name = e.vcall(form.addr(), FORM_SLOT_NAME, &args![]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![UNLOAD_HAS_BUFFER_WARNING, name, form_id],
        );
        return;
    }
    let manager = e.global::<u32>(MANAGER_INSTANCE);
    let buffer = e.mem.alloc(0x24);
    let word_slot = e.mem.alloc(4);
    let flags_slot = e.mem.alloc(4);
    e.call(SAVE_FORM_BUFFER_CONSTRUCT, &args![buffer]);
    let minor = e.call(MANAGER_GET_MINOR_VERSION, &args![manager]).u32();
    let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
    let stored_flags = e.mem.u32(entry);
    e.call(
        SAVE_FORM_BUFFER_SET_HEADER,
        &args![buffer, form_id, stored_flags, form_type, minor],
    );
    e.call(SAVE_FORM_BUFFER_SET_FORM, &args![buffer, form]);
    e.vcall(form.addr(), FORM_SLOT_CHECK_SAVE_GAME, &args![buffer]);
    let header_flags = e
        .call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, word_slot])
        .u32();
    let header_flags = read_word(e, header_flags);
    fn_00847cd0(e, Ptr::new(entry), Ptr::new(flags_slot), header_flags);
    if read_word(e, flags_slot) != 0 {
        let map = e.get(this, BGSSaveLoadGame::pChangesMap);
        let flags = e.mem.u32(flags_slot);
        if e.call(CHANGES_MAP_CLEAR_FLAGS, &args![map, form_id, flags])
            .bool()
        {
            e.call(SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);
            e.mem.free(flags_slot);
            e.mem.free(word_slot);
            e.mem.free(buffer);
            return;
        }
    }
    let stored_flags = e.mem.u32(entry);
    let kind = e
        .call(SAVE_INITIAL_DATA, &args![buffer, form, stored_flags])
        .i32();
    e.vcall(form.addr(), FORM_SLOT_SAVE, &args![buffer]);
    e.call(UNLOADED_FORM_BUFFER_RELEASE, &args![entry + 4, buffer]);
    if kind == 5 || kind == 6 {
        let references_map = e.get(this, BGSSaveLoadGame::pReferencesMap);
        let cell = e.call(REFERENCE_PARENT_CELL, &args![form]).u32();
        let world_space = e.call(REFERENCE_WORLD_SPACE, &args![form]).u32();
        if cell != 0 && e.call(CELL_FLAG_ZERO_TEST, &args![cell]).bool() {
            let cell_word = e.call(FORM_ID_WORD, &args![cell]).u32();
            fn_00849a70(e, references_map, cell_word, form_id);
            if kind == 6 {
                fn_00849a50(e, references_map, form_id);
            }
        } else if world_space != 0 {
            let place = e.vcall(form.addr(), REFERENCE_SLOT_PLACE, &args![]).u32();
            let world_word = e.call(FORM_ID_WORD, &args![world_space]).u32();
            e.call(
                REFERENCES_MAP_ADD_UNLOADED,
                &args![references_map, world_word, form_id, place],
            );
            if kind == 6 {
                fn_00849a50(e, references_map, form_id);
            }
        } else {
            let name = e.vcall(form.addr(), FORM_SLOT_NAME, &args![]).u32();
            e.call(
                SAVE_GAME_WARNING,
                &args![UNLOAD_NO_PLACE_WARNING, name, form_id],
            );
        }
    }
    e.call(SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);
    e.mem.free(flags_slot);
    e.mem.free(word_slot);
    e.mem.free(buffer);
}

// Translated from 00849a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetAt(this, id, id)` on the references map (`00844700`, the body of
/// `NiTMapBase::SetAt`): maps `id` to itself.
pub fn fn_00849a50(e: &mut Engine, this: Ptr, id: u32) {
    e.call(MAP_SET_AT, &args![this, id, id]);
}

// Translated from 00849a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00852950` on the member at +0x10 of the references map with `(a,
/// b)` (the engine map names it `PathingLOSMap::Setup`: folded code).
pub fn fn_00849a70(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(REFERENCES_MAP_SETUP, &args![this.byte_add(0x10), a, b]);
}

// ---- Session 2: 00849a90 to 0084b650 ---------------------------------------

// Translated from 00849a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `form` may be unloaded: not when bit 0x4000 of its form flags is
/// set; and a reference for which virtual slot `0x224` is true and slot `0x324`
/// false is kept as well. `this` is not read.
pub fn fn_00849a90(e: &mut Engine, _this: Ptr<BGSSaveLoadGame>, form: Ptr) -> bool {
    if e.call(FORM_FLAG_4000_TEST, &args![form]).bool() {
        return false;
    }
    if e.vcall(form.addr(), FORM_SLOT_IS_REFERENCE, &args![])
        .bool()
        && e.vcall(form.addr(), REFERENCE_SLOT_UNLOAD_TEST_A, &args![])
            .bool()
        && !e
            .vcall(form.addr(), REFERENCE_SLOT_UNLOAD_TEST_B, &args![])
            .bool()
    {
        return false;
    }
    true
}

// Translated from 00849b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::LoadCell` (Xbox PDB): unless a load is running, loads the
/// cell's form, then the references the references map holds for that cell:
/// they are collected into a temporary reconstruct map (0x34 bytes, built on
/// the stack), its closing step runs with the thread flag of mask 2 set, and
/// each reference found is loaded with `LoadForm`. The C++ exception frame
/// around the temporary is not translated.
pub fn bgssaveloadgame_load_cell(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, cell: Ptr) {
    if e.call(GAME_LOADING_FLAG_TEST, &args![this]).bool() {
        return;
    }
    bgssaveloadgame_load_form(e, this, cell);
    e.with_stack(0x34, |e, pending| {
        e.call(RECONSTRUCT_FORMS_CONSTRUCT, &args![pending]);
        let references = e.get(this, BGSSaveLoadGame::pReferencesMap);
        e.call(
            REFERENCES_MAP_LOAD_FOR_CELL,
            &args![references, cell, pending],
        );
        fn_00849c00(e, this, true); // 00849b7f: ECX = this
        e.call(RECONSTRUCT_FINISH_A, &args![pending]);
        fn_00849c00(e, this, false); // 00849b91: ECX = this
        let count = fn_00849c80(e, pending);
        for index in 0..count {
            let reference = fn_00849ca0(e, pending, index);
            bgssaveloadgame_load_form(e, this, Ptr::new(reference));
        }
        e.call(RECONSTRUCT_DESTRUCT, &args![pending]);
    });
}

// Translated from 00849c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 1 (mask 2) of the per-thread flag word at +0x294
/// of the TLS block. `this` is not read.
/// 00849c04: a `thiscall` (`ECX` saved, `RET 4`): `this` is the first word
/// and the byte `on` (`MOVZX` of `[EBP+8]`) the second.
pub fn fn_00849c00(e: &mut Engine, _this: Ptr<BGSSaveLoadGame>, on: bool) {
    let word = e.tls() + THREAD_FLAGS_OFFSET;
    update_flag(e, word, 2, on);
}

// Translated from 00849c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of references in a reconstruct map: `00843bb0` on its list at
/// +0x10.
pub fn fn_00849c80(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FORM_LIST_COUNT, &args![this.byte_add(0x10)]).u32()
}

// Translated from 00849ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference at `index` of a reconstruct map: `00843330` on its list at
/// +0x10.
pub fn fn_00849ca0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(FORM_LIST_ELEMENT, &args![this.byte_add(0x10), index])
        .u32()
}

// Translated from 00849cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00849ce0` on the references map (`[+0x10]`).
pub fn fn_00849cc0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, id: u32) -> bool {
    let references = e.get(this, BGSSaveLoadGame::pReferencesMap);
    fn_00849ce0(e, references, id)
}

// Translated from 00849ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `id` up in a map (`00853130(this, id, &out)`; the out slot is the
/// parameter's own stack slot) and returns whether it is there.
pub fn fn_00849ce0(e: &mut Engine, this: Ptr, id: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(MAP_GET_VALUE, &args![this, id, slot]).bool()
    })
}

// Translated from 00849d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::CheckInitialData` (Xbox PDB): checks the initial data a
/// saved reference carries against what is loaded. `buffer` is the form's
/// `BGSLoadFormBuffer`, `file` the save file and `id_map` the form id map
/// swapped in while a kind 6 entry is read. Returns what `0084e850` returned
/// for kind 5 and 6 (0 otherwise).
///
/// With the buffer's header kind (`0084e730`) 5 (created) or 6 (moved), the
/// initial data is read and the place it names is found. A form that cannot
/// be the one saved (not a reference for kind 5, another base form) is
/// cleared and detached from the buffer. When the place is loaded or flag 2
/// of the data is set, a kind 5 reference is created (`0084f8e0`) and handed
/// to the buffer (or the buffer is told to skip it), and a kind 6 reference
/// without a live form is added to the reconstruct map. Otherwise the
/// reference is recorded in the references map, or a warning is issued and
/// the buffer skipped; a form that was loaded for it is removed from its cell
/// and deleted. With any other kind, a kind 6 entry (by the old change flags)
/// with no live form has its stored initial data read and its original place
/// added to the reconstruct map when that cell is loaded; a live reference
/// that is not persistent and whose location is not loaded is deleted.
pub fn bgssaveloadgame_check_initial_data(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    file: Ptr,
    buffer: Ptr,
    id_map: Ptr,
) -> u32 {
    e.with_stack(0xe0, |e, frame| {
        check_initial_data_body(e, this, file, buffer, id_map, frame.addr())
    })
}

/// The slots of `CheckInitialData`'s stack frame, as offsets into the block
/// the entry point allocates.
const CHECK_HEADER_FLAGS: u32 = 0x00;
const CHECK_OLD_FLAGS: u32 = 0x04;
const CHECK_CELL: u32 = 0x08;
const CHECK_WORLD: u32 = 0x0c;
const CHECK_COMBINED_FLAGS: u32 = 0x10;
const CHECK_ENTRY: u32 = 0x14;
const CHECK_LOCATION_FORM: u32 = 0x18;
const CHECK_POSITION: u32 = 0x20;
const CHECK_ROTATION: u32 = 0x30;
const CHECK_INITIAL_DATA: u32 = 0x40;
const CHECK_SECOND_INITIAL_DATA: u32 = 0x70;

fn check_initial_data_body(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    file: Ptr,
    buffer: Ptr,
    id_map: Ptr,
    frame: u32,
) -> u32 {
    let world = e.global::<u32>(WORLD);
    let player = e.global::<u32>(PLAYER);
    let header_flags_slot = frame + CHECK_HEADER_FLAGS;
    let old_flags_slot = frame + CHECK_OLD_FLAGS;
    let cell_slot = frame + CHECK_CELL;
    let world_slot = frame + CHECK_WORLD;
    let init = frame + CHECK_INITIAL_DATA;
    let mut result = 0u32;
    let form_type = e.call(LOAD_FORM_BUFFER_HEADER_TYPE, &args![buffer]).u32();
    e.call(FORM_BUFFER_HEADER_FLAGS, &args![buffer, header_flags_slot]);
    e.call(LOAD_FORM_BUFFER_OLD_FLAGS, &args![buffer, old_flags_slot]);
    let header_flags = e.mem.u32(header_flags_slot);
    let old_flags = e.mem.u32(old_flags_slot);
    let id = e.call(BUFFER_FORM_ID, &args![buffer]).u32();
    let mut form = e.vcall(buffer.addr(), BUFFER_SLOT_GET_FORM, &args![]).u32();
    let kind = e
        .call(INITIAL_DATA_KIND, &args![id, form_type, header_flags])
        .u32();
    if kind == 5 || kind == 6 {
        fn_00849280(e, Ptr::new(init));
        result = e
            .call(LOAD_INITIAL_DATA_FROM_FILE, &args![file, kind, init])
            .u32();
        let mut owner = 0u32;
        if form != 0 {
            let mut mismatch = false;
            if !e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool() {
                mismatch = kind == 5;
            } else {
                owner = form;
                if kind == 5 {
                    let base = e.call(BUFFER_GET_FORM, &args![owner]).u32();
                    let base_id = e.call(FORM_ID_WORD, &args![base]).u32();
                    if base_id != e.mem.u32(init + 4) {
                        mismatch = true;
                    }
                }
            }
            if mismatch {
                bgssaveloadgame_clear_form(e, this, Ptr::new(form));
                e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, 0u32]);
                let none = change_flags(e, 0);
                e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, none]);
                form = 0;
                owner = 0;
            }
        }
        e.mem.set_u32(cell_slot, 0);
        e.mem.set_u32(world_slot, 0);
        e.call(LOCATION_CELL_AND_WORLD, &args![init, cell_slot, world_slot]);
        let cell = e.mem.u32(cell_slot);
        let place_loaded = cell != 0
            && e.call(WORLD_IS_CELL_LOADED, &args![world, cell, 0u32])
                .bool();
        let flag_two = e.mem.u8(init + 0x2c) & 2 != 0;
        let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
        if place_loaded || flag_two {
            if kind == 5 {
                if form != 0 {
                    let combined = frame + CHECK_COMBINED_FLAGS;
                    e.call(
                        COMBINE_CHANGE_FLAGS,
                        &args![combined, old_flags, header_flags, form_type, 1u32],
                    );
                    if read_word(e, combined) != 0 {
                        bgssaveloadgame_delete_form(e, this, Ptr::new(form));
                        form = 0;
                        let none = change_flags(e, 0);
                        e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, none]);
                    }
                }
                form = e.call(CREATE_REFERENCE, &args![id, init, form]).u32();
                if form != 0 {
                    e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, form]);
                } else {
                    fn_00848d90(e, Ptr::new(buffer.addr()), true);
                }
            } else if kind == 6 && form == 0 {
                // 00849f21: the slots are not cleared again, so `0084e6a0`
                // starts from the cell and world `0084e610` left in them (it
                // writes nothing for a form that is not a cell or worldspace).
                e.call(GET_ORIGINAL_LOCATION, &args![init, cell_slot, world_slot]);
                let original_cell = e.mem.u32(cell_slot);
                if original_cell != 0 {
                    e.call(
                        RECONSTRUCT_ADD_REFERENCE,
                        &args![reconstruct, original_cell, id, 0u32],
                    );
                } else {
                    let original_world = e.mem.u32(world_slot);
                    let x = e.mem.u32(init + 0xc);
                    let y = e.mem.u32(init + 0x10);
                    e.call(
                        RECONSTRUCT_ADD_REFERENCE_AT,
                        &args![reconstruct, original_world, x, y, id, 0u32],
                    );
                }
                e.call(LOAD_FORM_BUFFER_SET_LOADED, &args![buffer, 1u32]);
                fn_0084a360(e, Ptr::new(buffer.addr()), true);
            }
            return result;
        }
        // The place is not loaded and the data does not insist on it.
        if owner != 0 && (e.call(REFERENCE_IS_PERSISTENT, &args![owner]).bool() || owner == player)
        {
            return result;
        }
        let references = e.get(this, BGSSaveLoadGame::pReferencesMap);
        let first_word = e.mem.u32(init);
        if cell != 0 && e.call(CELL_FLAG_ZERO_TEST, &args![cell]).bool() {
            fn_00849a70(e, references, first_word, id);
            if kind == 6 {
                fn_00849a50(e, references, id);
            }
        } else if e.mem.u32(world_slot) != 0 {
            e.call(
                REFERENCES_MAP_ADD_UNLOADED,
                &args![references, first_word, id, init + 0x14],
            );
            if kind == 6 {
                fn_00849a50(e, references, id);
            }
        } else {
            let name = if kind == 5 { TEXT_CREATED } else { TEXT_MOVED };
            e.call(
                SAVE_GAME_WARNING,
                &args![MISSING_PLACE_WARNING, first_word, name],
            );
            fn_00848d90(e, Ptr::new(buffer.addr()), true);
        }
        if owner != 0 {
            let parent = e.call(REFERENCE_PARENT_CELL, &args![owner]).u32();
            if parent != 0 {
                let parent = e.call(REFERENCE_PARENT_CELL, &args![owner]).u32();
                e.call(CELL_REMOVE_REFERENCE, &args![parent, owner]);
            }
            bgssaveloadgame_delete_form(e, this, Ptr::new(form));
            e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, 0u32]);
        }
        return result;
    }
    let second_kind = e
        .call(INITIAL_DATA_KIND, &args![id, form_type, old_flags])
        .u32();
    if second_kind != 6 {
        return result;
    }
    if form == 0 {
        let entry_slot = frame + CHECK_ENTRY;
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_GET_ENTRY, &args![changes, entry_slot, id]);
        if read_word(e, entry_slot) != 0 {
            let second_init = frame + CHECK_SECOND_INITIAL_DATA;
            let saved_map = e.get(this, BGSSaveLoadGame::pFormIDMap);
            e.set(this, BGSSaveLoadGame::pFormIDMap, id_map);
            fn_00849280(e, Ptr::new(second_init));
            let advanced = e
                .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry_slot])
                .u32();
            e.call(
                LOAD_INITIAL_DATA_STRUCT,
                &args![advanced, second_kind, second_init],
            );
            e.mem.set_u32(cell_slot, 0);
            e.mem.set_u32(world_slot, 0);
            e.call(
                GET_ORIGINAL_LOCATION,
                &args![second_init, cell_slot, world_slot],
            );
            let cell = e.mem.u32(cell_slot);
            if cell != 0
                && e.call(WORLD_IS_CELL_LOADED, &args![world, cell, 0u32])
                    .bool()
            {
                let reconstruct = e.get(this, BGSSaveLoadGame::pReconstructForms);
                e.call(
                    RECONSTRUCT_ADD_REFERENCE,
                    &args![reconstruct, cell, id, 0u32],
                );
                e.call(LOAD_FORM_BUFFER_SET_LOADED, &args![buffer, 1u32]);
                fn_0084a360(e, Ptr::new(buffer.addr()), true);
            }
            e.set(this, BGSSaveLoadGame::pFormIDMap, saved_map);
        }
    } else if e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool()
        && !e.call(REFERENCE_IS_PERSISTENT, &args![form]).bool()
    {
        // The reference's own location (the three words at `DEFAULT_POSITION`
        // are what the call starts from) must be in a loaded cell.
        let location_slot = frame + CHECK_LOCATION_FORM;
        let position = frame + CHECK_POSITION;
        let rotation = frame + CHECK_ROTATION;
        e.mem.set_u32(location_slot, 0);
        for offset in [0u32, 4, 8] {
            let value = e.global::<u32>(DEFAULT_POSITION + offset);
            e.mem.set_u32(position + offset, value);
            e.mem.set_u32(rotation + offset, value);
        }
        let mut remove = false;
        if !e
            .vcall(
                form,
                REFERENCE_SLOT_LOCATION,
                &args![position, rotation, location_slot, 0u32],
            )
            .bool()
        {
            remove = true;
        } else {
            let location = e.mem.u32(location_slot);
            let place = if e.call(FORM_GET_TYPE, &args![location]).u32() == 0x41 {
                e.call(WORLD_SPACE_CELL_FROM_COORD, &args![location, position])
                    .u32()
            } else {
                location
            };
            if place == 0
                || !e
                    .call(WORLD_IS_CELL_LOADED, &args![world, place, 0u32])
                    .bool()
            {
                remove = true;
            }
        }
        if remove {
            bgssaveloadgame_delete_form(e, this, Ptr::new(form));
            e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, 0u32]);
            let none = change_flags(e, 0);
            e.call(LOAD_FORM_BUFFER_SET_OLD_FLAGS, &args![buffer, none]);
        }
    }
    result
}

// Translated from 0084a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit 1 (mask 2) of a `BGSLoadFormBuffer`'s `iFlags`.
pub fn fn_0084a360(e: &mut Engine, this: Ptr<BGSLoadFormBuffer>, on: bool) {
    update_flag(e, this.addr() + 0x28, 2, on);
}

// Translated from 0084a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::HandleUnrevertibleChanges` (Xbox PDB): `form_slot` is the
/// address of the pointer to the form (cleared when the form is deleted),
/// `flags` a `BGSChangeFlags` passed by value, and `delete_unpersistent`
/// decides what happens to a reference whose location cannot be restored.
/// Returns true for the player and when no flags are left at the end.
///
/// For a reference with changes in mask 0xe: a dynamic form id or a successful
/// move to its location (`00561ef0`) clears those bits and queues an actor's
/// package locations; otherwise a non-persistent reference is deleted when
/// `delete_unpersistent` is set, and the rest raise a warning.
pub fn bgssaveloadgame_handle_unrevertible_changes(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    form_slot: Ptr,
    flags: u32,
    delete_unpersistent: bool,
) -> bool {
    let player = e.global::<u32>(PLAYER);
    let form = e.mem.u32(form_slot.addr());
    if form == player {
        return true;
    }
    e.with_stack(4, |e, flag_slot| {
        e.mem.set_u32(flag_slot.addr(), flags);
        if e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool()
            && e.call(FLAGS_TEST, &args![flag_slot, 0xeu32]).bool()
        {
            let id = e.call(FORM_ID_WORD, &args![form]).u32();
            let data_handler = e.global::<u32>(DATA_HANDLER);
            if e.call(IS_DYNAMIC_FORM_ID, &args![data_handler, id]).bool()
                || e.call(REFERENCE_MOVE_TO_LOCATION, &args![form, 0u32])
                    .bool()
            {
                e.call(FLAGS_CLEAR, &args![flag_slot, 0xeu32]);
                if read_word(e, flag_slot.addr()) == 0
                    && e.vcall(form, FORM_SLOT_IS_ACTOR, &args![]).bool()
                {
                    bgssaveloadgame_queue_init_package_locations(e, this, Ptr::new(form));
                }
            } else if !delete_unpersistent || e.call(REFERENCE_IS_PERSISTENT, &args![form]).bool() {
                let name = if e.call(FORM_NAME_LENGTH, &args![form]).u32() != 0 {
                    e.vcall(form, FORM_SLOT_NAME, &args![]).u32()
                } else {
                    e.call(REFERENCE_FULL_NAME, &args![form]).u32()
                };
                let flags_now = read_word(e, flag_slot.addr());
                let base = e.call(BUFFER_GET_FORM, &args![form]).u32();
                let type_name = e.call(FORM_TYPE_NAME, &args![base]).u32();
                let id = e.call(FORM_ID_WORD, &args![form]).u32();
                e.call(
                    SAVE_GAME_WARNING,
                    &args![REVERT_LOCATION_WARNING, name, id, type_name, flags_now],
                );
            } else {
                bgssaveloadgame_delete_form(e, this, Ptr::new(form));
                e.mem.set_u32(form_slot.addr(), 0);
            }
        }
        read_word(e, flag_slot.addr()) == 0
    })
}

// Translated from 0084a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::QueueInitPackageLocations` (Xbox PDB): when the value
/// slot `0xc` of the object at +0x100 of the actor's `004181e0` result gives
/// (called with `0x10`, result in `ST0`) is above 0.0, the actor is added to
/// the package location map (`this + 0x34`) under its form id.
pub fn bgssaveloadgame_queue_init_package_locations(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    actor: Ptr,
) {
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    let value = e
        .vcall(process + 0x100, PROCESS_SLOT_VALUE, &args![0x10u32])
        .f64();
    if value > e.global::<f64>(ZERO_DOUBLE) {
        let id = e.call(FORM_ID_WORD, &args![actor]).u32();
        e.call(MAP_SET_AT, &args![this.byte_add(0x34), id, actor]);
    }
}

// Translated from 0084a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::CancelInitPackageLocations` (Xbox PDB): removes the
/// actor's form id from the package location map.
pub fn bgssaveloadgame_cancel_init_package_locations(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    actor: Ptr,
) {
    let id = e.call(FORM_ID_WORD, &args![actor]).u32();
    e.call(MAP_REMOVE_AT, &args![this.byte_add(0x34), id]);
}

// Translated from 0084a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pass after a load: every actor in the package location map that has a
/// saved acquire object (`008d8520`) gets `InitPackageLocations(1)`; then the
/// map is flushed (`00438af0`).
pub fn fn_0084a5b0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) {
    let map = this.byte_add(0x34);
    e.with_stack(0xc, |e, slots| {
        let position = slots.addr();
        let key = position + 4;
        let value = position + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT, &args![map, position, key, value]);
            let actor = e.mem.u32(value);
            if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32() != 0 {
                e.call(ACTOR_INIT_PACKAGE_LOCATIONS, &args![actor, 1u32]);
            }
        }
    });
    e.call(CHANGED_FORM_ID_MAP_FLUSH, &args![map]);
}

// Translated from 0084a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a change of `form` is recorded: the game must be recording
/// (`00570f00`) or `force` set, the form must not have bit 0x4000 of its form
/// flags, and a form with bit 0x20 must not have a dynamic form id.
pub fn fn_0084a620(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr, force: bool) -> bool {
    if !(e.call(GAME_RECORDING_TEST, &args![this]).bool() || force) {
        return false;
    }
    if e.call(FORM_FLAG_4000_TEST, &args![form]).bool() {
        return false;
    }
    let id = e.call(FORM_ID_WORD, &args![form]).u32();
    if e.call(FORM_FLAG_20_TEST, &args![form]).bool() {
        let data_handler = e.global::<u32>(DATA_HANDLER);
        if e.call(IS_DYNAMIC_FORM_ID, &args![data_handler, id]).bool() {
            return false;
        }
    }
    true
}

// Translated from 0084a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::AddChange` (Xbox PDB): when `0084a620` allows it, adds
/// `flags` to the form's entry in the changes map.
pub fn bgssaveloadgame_add_change(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    form: Ptr,
    flags: u32,
    force: bool,
) {
    if fn_0084a620(e, this, form, force) {
        let id = e.call(FORM_ID_WORD, &args![form]).u32();
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_ADD_FLAGS, &args![changes, id, flags]);
    }
}

// Translated from 0084a6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::GetChange` (Xbox PDB): whether the form's entry in the
/// changes map has a bit of `mask` (`00845720`).
pub fn bgssaveloadgame_get_change(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    form: Ptr,
    mask: u32,
) -> bool {
    let id = e.call(FORM_ID_WORD, &args![form]).u32();
    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
    e.call(CHANGES_MAP_HAS_FLAGS, &args![changes, id, mask])
        .bool()
}

// Translated from 0084a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same test on the old changes map (`[+4]`); false when there is none.
pub fn fn_0084a700(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr, mask: u32) -> bool {
    let old = e.get(this, BGSSaveLoadGame::pOldChangesMap);
    if old.is_null() {
        return false;
    }
    let id = e.call(FORM_ID_WORD, &args![form]).u32();
    e.call(CHANGES_MAP_HAS_FLAGS, &args![old, id, mask]).bool()
}

// Translated from 0084a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether changes to `form` are recorded: the game is recording (`00570f00`)
/// and the form does not have bit 0x4000 of its form flags.
pub fn fn_0084a740(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr) -> bool {
    e.call(GAME_RECORDING_TEST, &args![this]).bool()
        && !e.call(FORM_FLAG_4000_TEST, &args![form]).bool()
}

// Translated from 0084a780 (decompiled, FalloutNV.exe 1.4.0.525)
/// When changes to `form` are recorded (`0084a740`), clears `flags` from its
/// entry in the changes map (`00845a80`).
pub fn fn_0084a780(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr, flags: u32) {
    if fn_0084a740(e, this, form) {
        let id = e.call(FORM_ID_WORD, &args![form]).u32();
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_CLEAR_FLAGS, &args![changes, id, flags]);
    }
}

// Translated from 0084a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::RemoveChanges` (Xbox PDB): removes the form's entry from
/// the changes map unless it has a stored unloaded buffer (the word at +4 of
/// the entry).
pub fn bgssaveloadgame_remove_changes(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr) {
    let id = e.call(FORM_ID_WORD, &args![form]).u32();
    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
    let entry = e.call(CHANGES_MAP_ENTRY_FOR, &args![changes, id]).u32();
    if entry != 0 && read_word(e, entry + 4) == 0 {
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(CHANGES_MAP_REMOVE, &args![changes, id]);
    }
}

// Translated from 0084a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::QueueSubBuffer` (Xbox PDB): forwards its three words to
/// `QueueSubBuffer` of the queued sub buffers map (`[+0x14]`).
pub fn bgssaveloadgame_queue_sub_buffer(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    a: u32,
    b: u32,
    c: u32,
) {
    let queue = e.get(this, BGSSaveLoadGame::pQueuedSubBuffersMap);
    e.call(QUEUED_SUB_BUFFERS_QUEUE, &args![queue, a, b, c]);
}

// Translated from 0084a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the changes of a load to the live forms: with the load flag set,
/// the global data is reset (`0084c270(1)`), `00848e70(0, 0)` runs, the package
/// locations are initialised (`0084a5b0`) and the load flag is cleared.
pub fn fn_0084a840(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) {
    fn_00848ca0(e, this, true);
    e.call(GLOBAL_DATA_RESET, &args![1u32]);
    fn_00848e70(e, this, Ptr::NULL, Ptr::NULL);
    fn_0084a5b0(e, this);
    fn_00848ca0(e, this, false);
}

// Translated from 0084a880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::ClearForm` (Xbox PDB): gives a form a new id from the
/// data handler (form slot `0x128`) or, for form types 0x3a to 0x40, 0x49 and
/// 0x69, deletes it. Types 4, 0x11, 0x12, 0x2a, 0x2b, 0x39, 0x42 and 0x43 are
/// renumbered silently; every other type is renumbered with a warning.
pub fn bgssaveloadgame_clear_form(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, form: Ptr) {
    if form.is_null() {
        return;
    }
    let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
    match form_type {
        0x3a..=0x40 | 0x49 | 0x69 => bgssaveloadgame_delete_form(e, this, form),
        0x04 | 0x11 | 0x12 | 0x2a | 0x2b | 0x39 | 0x42 | 0x43 => renumber_form(e, form),
        _ => {
            let type_name = e.call(TYPE_NAME_FOR, &args![form_type & 0xff]).u32();
            let id = e.call(FORM_ID_WORD, &args![form]).u32();
            e.call(
                SAVE_GAME_WARNING,
                &args![RENUMBERED_WARNING, id, form_type, type_name],
            );
            renumber_form(e, form);
        }
    }
}

/// Gives `form` the next form id of the data handler (form slot `0x128`).
fn renumber_form(e: &mut Engine, form: Ptr) {
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let new_id = e.call(DATA_HANDLER_GET_NEXT_ID, &args![data_handler]).u32();
    // 0084a8db / 0084a923: `00469800` takes no stack word (plain RET); the pushed 1
    // is the second word of the form's +0x128 (`00485c10`, RET 8).
    e.vcall(form.addr(), FORM_SLOT_SET_NEW_ID, &args![new_id, 1u32]);
}

// Translated from 0084a9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::ClearFormID` (Xbox PDB): `ClearForm` of the form with this
/// id (null when there is none).
pub fn bgssaveloadgame_clear_form_id(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, id: u32) {
    let form: Ptr = e.call(LOOKUP_FORM, &args![id]).ptr();
    bgssaveloadgame_clear_form(e, this, form);
}

// Translated from 0084a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::DeleteForm` (Xbox PDB): calls form slot `0x10` (the
/// deleting destructor) with `1` on a form that is not null.
pub fn bgssaveloadgame_delete_form(e: &mut Engine, _this: Ptr<BGSSaveLoadGame>, form: Ptr) {
    if !form.is_null() {
        e.vcall(form.addr(), FORM_SLOT_DELETE, &args![1u32]);
    }
}

// Translated from 0084aa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::AddChangedFormID` (Xbox PDB): maps `old_id` to `new_id` in
/// the changed form id map (`[+0x18]`), with a warning when `old_id` was
/// already mapped (the new value replaces the old one).
pub fn bgssaveloadgame_add_changed_form_id(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    old_id: u32,
    new_id: u32,
) {
    let map = e.get(this, BGSSaveLoadGame::pChangedFormIDMap);
    e.with_stack(4, |e, slot| {
        if e.call(MAP_GET_VALUE, &args![map, old_id, slot]).bool() {
            let existing = e.mem.u32(slot.addr());
            e.call(
                SAVE_GAME_WARNING,
                &args![CHANGED_ID_CONFLICT_WARNING, new_id, old_id, existing],
            );
        }
    });
    e.call(MAP_SET_AT, &args![map, old_id, new_id]);
}

// Translated from 0084aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The id `old_id` was changed to in the changed form id map, 0 when it was
/// not.
pub fn fn_0084aa90(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, old_id: u32) -> u32 {
    let map = e.get(this, BGSSaveLoadGame::pChangedFormIDMap);
    e.with_stack(4, |e, slot| {
        if e.call(MAP_GET_VALUE, &args![map, old_id, slot]).bool() {
            e.mem.u32(slot.addr())
        } else {
            0
        }
    })
}

// Translated from 0084aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the changes map has an entry for the id.
pub fn fn_0084aad0(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, id: u32) -> bool {
    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
    e.call(CHANGES_MAP_ENTRY_FOR, &args![changes, id]).u32() != 0
}

// Translated from 0084ab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::SaveGameWarningFunction` (Xbox PDB): forwards the format
/// and the `va_list` to `AddNoteVarArgs` of the history (`[+0x1c]`).
pub fn bgssaveloadgame_save_game_warning_function(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadGame>,
    format: u32,
    arguments: u32,
) {
    let history = e.get(this, BGSSaveLoadGame::pHistory);
    e.call(HISTORY_ADD_NOTE_VA, &args![history, format, arguments]);
}

// Translated from 0084ab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGame::CleanupExpiredData` (Xbox PDB): drops the havok state of
/// references in cells that have been detached for too long.
///
/// With the per-thread flag bit 0 cleared (restored at the end), the first
/// pass walks the changes map. For every entry with change bit 0x40000000 the
/// cell is found (from the stored unloaded buffer when it is a cell buffer,
/// type 0x39, or from the live cell) together with the time it was detached,
/// and a cell detached for longer than `00526100` (against the calendar's
/// stamp `00867e30`) is put into a temporary expiry queue under the 64-bit key
/// `area | grid << 32`. When the queue is not empty the second pass walks the
/// map again: for every entry that is not a dynamic form id, the place of a
/// reference (type 0x3a) is worked out the same way; an entry whose place is
/// in the queue, that has change bit 4 and not bit 2, loses its stored
/// reference map entry and either its whole entry (flags exactly 4) or the
/// change bits 0xc together with its initial data (the unloaded buffer is
/// rebuilt without them). The history gets a note with the three counts. The
/// C++ exception frame around the queue is not translated.
pub fn bgssaveloadgame_cleanup_expired_data(e: &mut Engine, this: Ptr<BGSSaveLoadGame>) {
    let saved_flag = e.call(SET_THREAD_FLAG, &args![this, 0u32]).u8();
    e.with_stack(0x10, |e, queue| {
        fn_0084b4e0(e, Ptr::new(queue.addr()), 0x25);
        e.with_stack(0xa0, |e, frame| {
            cleanup_expired_data_body(e, this, queue.addr(), frame.addr());
        });
        e.call(CHANGED_FORM_ID_MAP_FLUSH, &args![queue]);
        e.call(SET_THREAD_FLAG, &args![this, saved_flag as u32]);
        e.call(EXPIRY_QUEUE_DESTRUCT, &args![queue]);
    });
}

/// Slots of the stack frame of the cleanup passes (offsets into the block).
const CLEANUP_POSITION: u32 = 0x00;
const CLEANUP_KEY: u32 = 0x04;
const CLEANUP_ENTRY: u32 = 0x08;
const CLEANUP_FOUND: u32 = 0x0c;
const CLEANUP_VALUE_SIZE: u32 = 0x10;
const CLEANUP_INITIAL_DATA: u32 = 0x20;
const CLEANUP_SECOND_INITIAL_DATA: u32 = 0x50;

/// The 64-bit key of the expiry queue as two words: the area id, then (when
/// the cell has grid coordinates, `x != 0x7fffffff`) the packed coordinates
/// (`00587410(x & 0xffff, y & 0xffff)`) in the high word.
fn expiry_key(e: &mut Engine, area: u32, x: u32, y: u32) -> (u32, u32) {
    if x != 0x7fff_ffff {
        let packed = e
            .call(PACK_GRID_COORDS, &args![x & 0xffff, y & 0xffff])
            .u32();
        (area, packed)
    } else {
        (area, 0)
    }
}

/// A place coordinate as the game rounds it: the float with these bits through
/// `00406d90`, shifted right by 12 (arithmetic).
fn grid_of(e: &mut Engine, value: u32) -> u32 {
    let rounded = e.call(ROUND_TO_INT, &args![f32::from_bits(value)]).i32();
    (rounded >> 12) as u32
}

fn cleanup_expired_data_body(e: &mut Engine, this: Ptr<BGSSaveLoadGame>, queue: u32, frame: u32) {
    let data_handler = e.global::<u32>(DATA_HANDLER);
    let position = frame + CLEANUP_POSITION;
    let key_slot = frame + CLEANUP_KEY;
    let entry_slot = frame + CLEANUP_ENTRY;
    let init = frame + CLEANUP_INITIAL_DATA;
    let now = e.call(CALENDAR_TIME_STAMP, &args![CALENDAR]).u32();
    let mut expired = 0u32;

    // ---- first pass: which cells have expired --------------------------------
    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
    let first = e.call(MAP_FIRST_POSITION, &args![changes]).u32();
    e.mem.set_u32(position, first);
    while e.mem.u32(position) != 0 {
        e.mem.set_u32(key_slot, 0);
        e.mem.set_u32(entry_slot, 0);
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(MAP_NEXT, &args![changes, position, key_slot, entry_slot]);
        let key = e.mem.u32(key_slot);
        let entry = e.mem.u32(entry_slot);
        let mut area = key;
        if !e.call(FLAGS_TEST, &args![entry, 0x4000_0000u32]).bool() {
            continue;
        }
        let mut detach_time = 0u32;
        let mut x = 0x7fff_ffffu32;
        let mut y = 0x7fff_ffffu32;
        if read_word(e, entry + 4) != 0 {
            if fn_00849220(e, Ptr::new(entry + 4)) == 0x39 {
                let flags = e.mem.u32(entry);
                let kind = e.call(INITIAL_DATA_KIND, &args![key, 0x39u32, flags]).u32();
                if kind != 0 {
                    let advanced = e
                        .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry + 4])
                        .u32();
                    e.call(LOAD_INITIAL_DATA_STRUCT, &args![advanced, kind, init]);
                    detach_time = e.mem.u32(init + 0xc);
                    if kind != 3 {
                        area = e.mem.u32(init);
                        x = e.mem.u32(init + 4);
                        y = e.mem.u32(init + 8);
                    }
                }
            }
        } else {
            let cell = e.call(LOOKUP_FORM, &args![key]).u32();
            if cell != 0 && e.call(FORM_GET_TYPE, &args![cell]).u32() == 0x39 {
                detach_time = e.call(CELL_DETACH_TIME, &args![cell]).u32();
                if !e.call(CELL_FLAG_ZERO_TEST, &args![cell]).bool() {
                    let world_space = e.call(CELL_WORLD_SPACE, &args![cell]).u32();
                    area = e.call(FORM_ID_WORD, &args![world_space]).u32();
                    x = e.call(CELL_DATA_X, &args![cell]).u32();
                    y = e.call(CELL_DATA_Y, &args![cell]).u32();
                }
            }
        }
        if detach_time != 0 && now.wrapping_sub(detach_time) > e.call(EXPIRY_LIMIT, &args![]).u32()
        {
            let (low, high) = expiry_key(e, area, x, y);
            e.call(EXPIRY_QUEUE_INSERT, &args![queue, low, high, detach_time]);
            expired += 1;
        }
    }
    if expired == 0 {
        return;
    }

    // ---- second pass: strip the havok state of references in those cells -----
    let mut references_removed = 0u32;
    let mut all_removed = 0u32;
    let mut partial_removed = 0u32;
    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
    let first = e.call(MAP_FIRST_POSITION, &args![changes]).u32();
    e.mem.set_u32(position, first);
    while e.mem.u32(position) != 0 {
        e.mem.set_u32(key_slot, 0);
        e.mem.set_u32(entry_slot, 0);
        let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
        e.call(MAP_NEXT, &args![changes, position, key_slot, entry_slot]);
        let key = e.mem.u32(key_slot);
        let entry = e.mem.u32(entry_slot);
        if e.call(IS_DYNAMIC_FORM_ID, &args![data_handler, key]).bool() {
            continue;
        }
        let mut area = 0u32;
        let mut x = 0u32;
        let mut y = 0u32;
        let mut reference = 0u32;
        let form_type;
        if read_word(e, entry + 4) != 0 {
            form_type = fn_00849220(e, Ptr::new(entry + 4));
        } else {
            let form = e.call(LOOKUP_FORM, &args![key]).u32();
            if form != 0 {
                form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
                if e.vcall(form, FORM_SLOT_IS_REFERENCE, &args![]).bool() {
                    reference = form;
                }
            } else {
                form_type = 0;
                let flags = read_word(e, entry);
                e.call(
                    SAVE_GAME_WARNING,
                    &args![CLEANUP_MISSING_BUFFER_WARNING, key, flags],
                );
            }
        }
        if form_type == 0x3a {
            if read_word(e, entry + 4) != 0 {
                let flags = e.mem.u32(entry);
                let kind = e
                    .call(INITIAL_DATA_KIND, &args![key, form_type, flags])
                    .u32();
                if kind != 0 {
                    fn_00849280(e, Ptr::new(init));
                    let advanced = e
                        .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry + 4])
                        .u32();
                    e.call(LOAD_INITIAL_DATA_STRUCT, &args![advanced, kind, init]);
                    area = e.mem.u32(init);
                    let float_x = e.mem.u32(init + 0x14);
                    let float_y = e.mem.u32(init + 0x18);
                    x = grid_of(e, float_x);
                    y = grid_of(e, float_y);
                }
            } else if reference != 0 {
                let parent = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                let world_space = e.call(REFERENCE_WORLD_SPACE, &args![reference]).u32();
                if parent != 0 && e.call(CELL_FLAG_ZERO_TEST, &args![parent]).bool() {
                    area = e.call(FORM_ID_WORD, &args![parent]).u32();
                } else if world_space != 0 {
                    area = e.call(FORM_ID_WORD, &args![world_space]).u32();
                    let place = e.vcall(reference, REFERENCE_SLOT_PLACE, &args![]).u32();
                    // The third word of the place is copied by the game but not
                    // used.
                    let float_x = e.mem.u32(place);
                    let float_y = e.mem.u32(place + 4);
                    x = grid_of(e, float_x);
                    y = grid_of(e, float_y);
                }
            }
        }
        if area != 0 {
            let (low, high) = expiry_key(e, area, x, y);
            let found = frame + CLEANUP_FOUND;
            e.mem.set_u32(found, 0);
            if e.call(EXPIRY_QUEUE_FIND, &args![queue, low, high, found])
                .bool()
                && e.call(FLAGS_TEST, &args![entry, 4u32]).bool()
                && !e.call(FLAGS_TEST, &args![entry, 2u32]).bool()
            {
                references_removed += 1;
                let flags = e.mem.u32(entry);
                let kind = e
                    .call(INITIAL_DATA_KIND, &args![key, form_type, flags])
                    .u32();
                let second_init = frame + CLEANUP_SECOND_INITIAL_DATA;
                fn_00849280(e, Ptr::new(second_init));
                let references_map = e.get(this, BGSSaveLoadGame::pReferencesMap);
                if read_word(e, entry + 4) != 0 && kind == 6 {
                    let advanced = e
                        .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry + 4])
                        .u32();
                    e.call(
                        LOAD_INITIAL_DATA_STRUCT,
                        &args![advanced, kind, second_init],
                    );
                    fn_0084b470(e, references_map, key);
                    let cell_id = e.mem.u32(second_init);
                    let cell = e.call(LOOKUP_FORM, &args![cell_id]).u32();
                    if cell != 0 && e.call(FORM_GET_TYPE, &args![cell]).u32() == 0x41 {
                        e.call(
                            REFERENCES_MAP_REMOVE_EXTERIOR,
                            &args![references_map, cell_id, key, second_init + 0x14],
                        );
                    } else {
                        fn_0084b490(e, references_map, cell_id, key);
                    }
                }
                if e.call(FLAGS_EQUAL, &args![entry, 4u32]).bool() {
                    let changes = e.get(this, BGSSaveLoadGame::pChangesMap);
                    e.call(CHANGES_MAP_REMOVE, &args![changes, key]);
                    all_removed += 1;
                } else {
                    if read_word(e, entry + 4) != 0 && kind != 0 {
                        let advanced = e
                            .call(UNLOADED_FORM_BUFFER_ADVANCED, &args![entry + 4])
                            .u32();
                        let header_size = e.call(INITIAL_DATA_TYPE_SIZE, &args![kind]).u32();
                        let after_header = advanced.wrapping_add(header_size);
                        let size_slot = frame + CLEANUP_VALUE_SIZE;
                        e.mem.set_u32(size_slot, 0);
                        let length_size = e
                            .call(LOAD_VARIABLE_SIZED_VALUE, &args![after_header, size_slot])
                            .u32();
                        let value_size = e.mem.u32(size_slot);
                        let data = length_size
                            .wrapping_add(value_size)
                            .wrapping_add(after_header);
                        let old_buffer = read_word(e, entry + 4);
                        let flag = fn_0084b450(e, Ptr::new(entry + 4));
                        let total = e.call(UNLOADED_FORM_BUFFER_SIZE, &args![entry + 4]).u32();
                        let remaining = total
                            .wrapping_sub(header_size)
                            .wrapping_sub(length_size)
                            .wrapping_sub(value_size);
                        e.call(UNLOADED_FORM_BUFFER_CLEAR, &args![entry + 4]);
                        e.call(
                            UNLOADED_FORM_BUFFER_INITIALIZE,
                            &args![entry + 4, data, remaining, form_type, flag],
                        );
                        e.call(OPERATOR_DELETE, &args![old_buffer]);
                        partial_removed += 1;
                    }
                    e.call(FLAGS_CLEAR, &args![entry, 0xcu32]);
                }
                if reference != 0 {
                    e.call(REFERENCE_MOVE_TO_LOCATION, &args![reference, 0u32]);
                }
            }
        }
    }
    if references_removed != 0 {
        let history = e.get(this, BGSSaveLoadGame::pHistory);
        e.call(
            HISTORY_ADD_NOTE,
            &args![
                history,
                CLEANUP_NOTE,
                references_removed,
                all_removed,
                partial_removed
            ],
        );
    }
}

// Translated from 0084b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +1 of the block `*this` points to (the unloaded form buffer's
/// flag byte).
pub fn fn_0084b450(e: &mut Engine, this: Ptr) -> u8 {
    let block = e.mem.u32(this.addr());
    e.mem.u8(block + 1)
}

// Translated from 0084b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::RemoveAt(this, key)` (Xbox PDB name of the body, `00405430`).
pub fn fn_0084b470(e: &mut Engine, this: Ptr, key: u32) {
    e.call(MAP_REMOVE_AT, &args![this, key]);
}

// Translated from 0084b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCellNumericIDArrayMap::RemoveReference(a, b)` (Xbox PDB) on the member
/// at +0x10 of the references map (`00852a10`).
pub fn fn_0084b490(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(REFERENCES_MAP_REMOVE, &args![this.byte_add(0x10), a, b]);
}

// Translated from 0084b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the package location map `NiTMap<unsigned int,Actor *>`
/// (`this`, bucket count): the base constructor `0084b650`, then its own
/// vtable.
pub fn fn_0084b4b0(e: &mut Engine, this: Ptr<NiTPointerMap>, size: u32) -> Ptr<NiTPointerMap> {
    fn_0084b650(e, this, size);
    e.mem.set_u32(this.addr(), PACKAGE_LOCATION_MAP_VTABLE);
    this
}

// Translated from 0084b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the expiry queue `NiTMap<__int64,int>` (`this`, bucket
/// count): its base constructor `0084b750`, then its own vtable.
pub fn fn_0084b4e0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(EXPIRY_QUEUE_BASE_CONSTRUCT, &args![this, size]);
    e.mem.set_u32(this.addr(), EXPIRY_QUEUE_VTABLE);
    this
}

// Translated from 0084b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,Actor *>::scalar deleting destructor` (Xbox PDB): the
/// destructor body `0084b6c0`, then the block is freed when bit 0 of `flags`
/// is set.
pub fn fn_0084b510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, PACKAGE_LOCATION_MAP_DESTRUCT)
}

// Translated from 0084b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<__int64,int>::scalar deleting destructor` (Xbox PDB): the
/// destructor body `0084b9c0`, then the block is freed when bit 0 of `flags`
/// is set.
pub fn fn_0084b540(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, EXPIRY_QUEUE_DESTRUCT)
}

// Translated from 0084b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form buffer array `BSSimpleArray<BGSLoadFormBuffer *,
/// 1024>`: its vtable, then the base setup `006b3eb0(0, 0)`.
pub fn fn_0084b570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FORM_BUFFER_ARRAY_VTABLE);
    e.call(ARRAY_BASE_SETUP, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0084b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the form buffer array: its vtable, then `Clear(1)`
/// (`008454f0`).
pub fn fn_0084b5a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FORM_BUFFER_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0084b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of the package location map (`this`, bucket count):
/// the base vtable, the bucket count at +4, a zero item count at +0xc and a
/// zeroed bucket array of `size * 4` bytes at +8 (`00aa1070`, `memset`).
pub fn fn_0084b650(e: &mut Engine, this: Ptr<NiTPointerMap>, size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), HASH_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(ALLOCATE_BYTES, &args![size << 2]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, buckets);
    e.call(MEMORY_SET, &args![buckets, 0u32, size << 2]);
    this
}

// ---- Session 3: 0084b6c0 to 0084bc20 ---------------------------------------

// Translated from 0084b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the package location map `NiTMap<unsigned int,Actor *>`:
/// its vtable, `00438af0(this)`, then the base destructor body `0084b720`.
/// The exception frame is not translated.
pub fn fn_0084b6c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PACKAGE_LOCATION_MAP_VTABLE);
    e.call(MAP_CLEAR_ITEMS, &args![this]);
    e.call(HASH_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 0084b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the expiry queue (`this`, bucket count): the base
/// vtable `0107f4e8`, the bucket count at +4, a zero item count at +0xc and
/// a zeroed bucket array of `size * 4` bytes at +8.
pub fn fn_0084b750(e: &mut Engine, this: Ptr<NiTPointerMap>, size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), EXPIRY_QUEUE_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(ALLOCATE_BYTES, &args![size << 2]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, buckets);
    e.call(MEMORY_SET, &args![buckets, 0u32, size << 2]);
    this
}

// An item of the expiry queue (`NiTMapItem<__int64,int>`, 0x18 bytes): the
// next item at +0, the 64 bit key as two words at +8 and +0xc, the value at
// +0x10.
const EXPIRY_ITEM_NEXT: u32 = 0x00;
const EXPIRY_ITEM_KEY_LOW: u32 = 0x08;
const EXPIRY_ITEM_KEY_HIGH: u32 = 0x0c;
const EXPIRY_ITEM_VALUE: u32 = 0x10;

/// Walks the bucket of the key (hash from virtual slot +4, comparison from
/// slot +8) and returns the bucket array address, the bucket index and the
/// item with an equal key (0 when there is none).
fn expiry_find_item(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32) -> (u32, u32, u32) {
    let index = e.vcall(this.addr(), 0x4, &args![key_low, key_high]).u32();
    let buckets = e.get(this.cast::<NiTPointerMap>(), NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(buckets + index * 4);
    while item != 0 {
        let item_low = e.mem.u32(item + EXPIRY_ITEM_KEY_LOW);
        let item_high = e.mem.u32(item + EXPIRY_ITEM_KEY_HIGH);
        let equal = e.vcall(
            this.addr(),
            0x8,
            &args![key_low, key_high, item_low, item_high],
        );
        if equal.bool() {
            break;
        }
        item = e.mem.u32(item + EXPIRY_ITEM_NEXT);
    }
    (buckets, index, item)
}

// Translated from 0084b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...,__int64,int>::SetAt` shape (the map names none): a bucket
/// item with an equal key gets the new value; otherwise a new item (slot
/// +0x14, `NewItem`) is filled (slot +0xc, `SetValue`), put at the head of
/// the bucket and counted.
pub fn fn_0084b7c0(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32, value: u32) {
    let (buckets, index, item) = expiry_find_item(e, this, key_low, key_high);
    if item != 0 {
        e.mem.set_u32(item + EXPIRY_ITEM_VALUE, value);
        return;
    }
    let item = e.vcall(this.addr(), 0x14, &args![]).u32();
    e.vcall(this.addr(), 0xc, &args![item, key_low, key_high, value]);
    let head = e.mem.u32(buckets + index * 4);
    e.mem.set_u32(item + EXPIRY_ITEM_NEXT, head);
    e.mem.set_u32(buckets + index * 4, item);
    let map = this.cast::<NiTPointerMap>();
    let count = e.get(map, NiTPointerMap::m_uiCount);
    e.set(map, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 0084b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...,__int64,int>::GetAt` shape: finds the item of the key in
/// its bucket and stores its value through `out`; false when there is none.
pub fn fn_0084b8b0(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32, out: u32) -> bool {
    let (_, _, item) = expiry_find_item(e, this, key_low, key_high);
    if item == 0 {
        return false;
    }
    let value = e.mem.u32(item + EXPIRY_ITEM_VALUE);
    e.mem.set_u32(out, value);
    true
}

// Translated from 0084b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<__int64,int>>,__int64,int>::KeyToHashIndex`
/// (Xbox PDB): the low word of the key modulo the bucket count; the high word
/// is not read. A bucket count of zero would fault in the game.
pub fn fn_0084b930(e: &mut Engine, this: Ptr<NiTPointerMap>, key_low: u32, _key_high: u32) -> u32 {
    let size = e.get(this, NiTPointerMap::m_uiHashSize);
    key_low % size
}

// Translated from 0084b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<__int64,int>>,__int64,int>::IsKeysEqual`
/// (Xbox PDB): both words of the two keys are equal.
pub fn fn_0084b950(
    _e: &mut Engine,
    _this: Ptr,
    low_a: u32,
    high_a: u32,
    low_b: u32,
    high_b: u32,
) -> bool {
    low_a == low_b && high_a == high_b
}

// Translated from 0084b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<__int64,int>>,__int64,int>::SetValue`
/// (Xbox PDB): stores the key (two words) and the value in the item.
pub fn fn_0084b990(e: &mut Engine, _this: Ptr, item: u32, key_low: u32, key_high: u32, value: u32) {
    e.mem.set_u32(item + EXPIRY_ITEM_KEY_LOW, key_low);
    e.mem.set_u32(item + EXPIRY_ITEM_KEY_HIGH, key_high);
    e.mem.set_u32(item + EXPIRY_ITEM_VALUE, value);
}

// Translated from 0084b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the expiry queue `NiTMap<__int64,int>`: its vtable,
/// `00438af0(this)`, then the base destructor body `0084ba20`. The exception
/// frame is not translated.
pub fn fn_0084b9c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EXPIRY_QUEUE_VTABLE);
    e.call(MAP_CLEAR_ITEMS, &args![this]);
    fn_0084ba20(e, this);
}

// Translated from 0084ba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the expiry queue's base: the base vtable,
/// `00438af0(this)`, then the bucket array at +8 is freed (`00aa10f0`).
pub fn fn_0084ba20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EXPIRY_QUEUE_BASE_VTABLE);
    e.call(MAP_CLEAR_ITEMS, &args![this]);
    let buckets = e.get(this.cast::<NiTPointerMap>(), NiTPointerMap::m_ppkHashTable);
    e.call(FREE_BYTES, &args![buckets]);
}

// Translated from 0084ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<__int64,int>::NewItem` (Xbox PDB): a new item from the allocator
/// member at +0xc (`0084bb30`).
pub fn fn_0084ba50(e: &mut Engine, this: Ptr) -> u32 {
    fn_0084bb30(e, this.byte_add(0xc))
}

// Translated from 0084ba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<__int64,int>::DeleteItem` (Xbox PDB): clears the value at +0x10,
/// then frees the item through the allocator member at +0xc (`006b8310`).
pub fn fn_0084ba70(e: &mut Engine, this: Ptr, item: u32) {
    e.mem.set_u32(item + EXPIRY_ITEM_VALUE, 0);
    e.call(MAP_ITEM_FREE, &args![this.byte_add(0xc), item]);
}

// Translated from 0084baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSLoadFormBuffer *,1024>::scalar deleting destructor`
/// (Xbox PDB): the destructor `0084b5a0`, then the block is freed when bit 0
/// of `flags` is set.
pub fn fn_0084baa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, FORM_BUFFER_ARRAY_DESTRUCT)
}

// Translated from 0084bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int,Actor *>>,unsigned int,Actor *>::
/// scalar deleting destructor` (Xbox PDB): the hash map base destructor body
/// `0084b720`, then the block is freed when bit 0 of `flags` is set.
pub fn fn_0084bad0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting_destructor(e, this, flags, HASH_MAP_BASE_DESTRUCT)
}

// Translated from 0084bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<__int64,int>>,__int64,int>::scalar deleting
/// destructor` (Xbox PDB): the base destructor body `0084ba20`, then the
/// block is freed when bit 0 of `flags` is set.
pub fn fn_0084bb00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0084ba20(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0084bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The item allocator of the expiry queue (`this`, the allocator member at
/// +0xc, is not read): a new 0x18 byte block.
pub fn fn_0084bb30(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(OPERATOR_NEW, &args![0x18u32]).u32()
}

// Translated from 0084bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `count` global data records from `file` (cdecl `(file, count)`): for
/// each, a `BGSLoadGameBuffer` is constructed, the record type (4 bytes) is
/// read from the file, the buffer is loaded from the file, `LoadGlobalData(type,
/// buffer)` (`0084be40`) consumes it and the buffer is destroyed. The exception
/// frame is not translated.
pub fn fn_0084bc20(e: &mut Engine, file: u32, count: u32) {
    for _ in 0..count {
        e.with_stack(0x14, |e, stack| {
            let buffer = stack.addr();
            let record_type = buffer + 0x10;
            e.call(LOAD_GAME_BUFFER_CONSTRUCT, &args![buffer]);
            e.mem.set_u32(record_type, 0);
            e.call(LOAD_DATA_FROM_FILE, &args![file, record_type, 4u32]);
            e.call(LOAD_GAME_BUFFER_LOAD, &args![buffer, file]);
            let kind = e.mem.u32(record_type);
            e.call(LOAD_GLOBAL_DATA, &args![kind, buffer]);
            e.call(LOAD_GAME_BUFFER_DESTRUCT, &args![buffer]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00846f30,
            bgssaveloadgame_bgssaveloadgame(Ptr<BGSSaveLoadGame>) -> Ptr<BGSSaveLoadGame>
        ),
        entry!(0x008471f0, fn_008471f0(Ptr<BGSSaveLoadGame>)),
        entry!(0x008473c0, fn_008473c0(Ptr, u32) -> Ptr),
        entry!(0x008473f0, fn_008473f0(Ptr, u32) -> Ptr),
        entry!(0x00847420, fn_00847420(Ptr, u32) -> Ptr),
        entry!(0x00847450, fn_00847450(Ptr, u32) -> Ptr),
        entry!(0x00847480, bgssaveloadgame_create()),
        entry!(0x00847500, fn_00847500()),
        entry!(0x00847540, fn_00847540(Ptr, u32) -> Ptr),
        entry!(
            0x00847570,
            bgssaveloadgame_get_current_minor_version(Ptr<BGSSaveLoadGame>) -> u8
        ),
        entry!(
            0x00847590,
            bgssaveloadgame_save_plugin_list(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00847660,
            bgssaveloadgame_load_plugin_list(Ptr<BGSSaveLoadGame>, Ptr) -> bool
        ),
        entry!(
            0x00847850,
            bgssaveloadgame_save_game(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(0x00847cd0, fn_00847cd0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00847d50, fn_00847d50(Ptr) -> Ptr),
        entry!(0x00847d70, fn_00847d70(Ptr) -> u32),
        entry!(0x00847d90, fn_00847d90(Ptr) -> u32),
        entry!(
            0x00847df0,
            bgssaveloadgame_load_game(Ptr<BGSSaveLoadGame>, Ptr, bool) -> bool
        ),
        entry!(0x00848ca0, fn_00848ca0(Ptr<BGSSaveLoadGame>, bool)),
        entry!(0x00848cf0, fn_00848cf0(Ptr<BGSSaveLoadGame>) -> bool),
        entry!(0x00848d10, fn_00848d10()),
        entry!(0x00848d30, fn_00848d30()),
        entry!(
            0x00848d50,
            bgsloadgamebuffer_delete_buffer(Ptr<BGSLoadGameBuffer>)
        ),
        entry!(0x00848d90, fn_00848d90(Ptr<BGSLoadFormBuffer>, bool)),
        entry!(0x00848dd0, fn_00848dd0(Ptr<BGSLoadFormBuffer>) -> bool),
        entry!(0x00848df0, fn_00848df0(Ptr<TESDataHandler>, u8)),
        entry!(0x00848e10, fn_00848e10(Ptr)),
        entry!(0x00848e40, fn_00848e40(u8)),
        entry!(0x00848e70, fn_00848e70(Ptr<BGSSaveLoadGame>, Ptr, Ptr)),
        entry!(0x00849220, fn_00849220(Ptr) -> u32),
        entry!(0x00849240, fn_00849240(Ptr<BGSLoadFormBuffer>, bool)),
        entry!(0x00849280, fn_00849280(Ptr) -> Ptr),
        entry!(0x008492b0, fn_008492b0(Ptr<BGSSaveLoadGame>, bool)),
        entry!(0x00849540, fn_00849540(Ptr<BGSSaveLoadGame>, bool)),
        entry!(0x00849590, fn_00849590(Ptr<BGSLoadFormBuffer>) -> bool),
        entry!(0x008495b0, fn_008495b0(Ptr)),
        entry!(
            0x008495d0,
            bgssaveloadgame_load_form(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x00849730,
            bgssaveloadgame_unload_form(Ptr<BGSSaveLoadGame>, Ptr, bool)
        ),
        entry!(0x00849a50, fn_00849a50(Ptr, u32)),
        entry!(0x00849a70, fn_00849a70(Ptr, u32, u32)),
        entry!(0x00849a90, fn_00849a90(Ptr<BGSSaveLoadGame>, Ptr) -> bool),
        entry!(
            0x00849b10,
            bgssaveloadgame_load_cell(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(0x00849c00, fn_00849c00(Ptr<BGSSaveLoadGame>, bool)), // 00849c00: this, on
        entry!(0x00849c80, fn_00849c80(Ptr) -> u32),
        entry!(0x00849ca0, fn_00849ca0(Ptr, u32) -> u32),
        entry!(0x00849cc0, fn_00849cc0(Ptr<BGSSaveLoadGame>, u32) -> bool),
        entry!(0x00849ce0, fn_00849ce0(Ptr, u32) -> bool),
        entry!(
            0x00849d00,
            bgssaveloadgame_check_initial_data(Ptr<BGSSaveLoadGame>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x0084a360, fn_0084a360(Ptr<BGSLoadFormBuffer>, bool)),
        entry!(
            0x0084a3a0,
            bgssaveloadgame_handle_unrevertible_changes(
                Ptr<BGSSaveLoadGame>,
                Ptr,
                u32,
                bool,
            ) -> bool
        ),
        entry!(
            0x0084a520,
            bgssaveloadgame_queue_init_package_locations(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0084a580,
            bgssaveloadgame_cancel_init_package_locations(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(0x0084a5b0, fn_0084a5b0(Ptr<BGSSaveLoadGame>)),
        entry!(
            0x0084a620,
            fn_0084a620(Ptr<BGSSaveLoadGame>, Ptr, bool) -> bool
        ),
        entry!(
            0x0084a690,
            bgssaveloadgame_add_change(Ptr<BGSSaveLoadGame>, Ptr, u32, bool)
        ),
        entry!(
            0x0084a6d0,
            bgssaveloadgame_get_change(Ptr<BGSSaveLoadGame>, Ptr, u32) -> bool
        ),
        entry!(
            0x0084a700,
            fn_0084a700(Ptr<BGSSaveLoadGame>, Ptr, u32) -> bool
        ),
        entry!(0x0084a740, fn_0084a740(Ptr<BGSSaveLoadGame>, Ptr) -> bool),
        entry!(0x0084a780, fn_0084a780(Ptr<BGSSaveLoadGame>, Ptr, u32)),
        entry!(
            0x0084a7c0,
            bgssaveloadgame_remove_changes(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0084a810,
            bgssaveloadgame_queue_sub_buffer(Ptr<BGSSaveLoadGame>, u32, u32, u32)
        ),
        entry!(0x0084a840, fn_0084a840(Ptr<BGSSaveLoadGame>)),
        entry!(
            0x0084a880,
            bgssaveloadgame_clear_form(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0084a9c0,
            bgssaveloadgame_clear_form_id(Ptr<BGSSaveLoadGame>, u32)
        ),
        entry!(
            0x0084a9f0,
            bgssaveloadgame_delete_form(Ptr<BGSSaveLoadGame>, Ptr)
        ),
        entry!(
            0x0084aa30,
            bgssaveloadgame_add_changed_form_id(Ptr<BGSSaveLoadGame>, u32, u32)
        ),
        entry!(0x0084aa90, fn_0084aa90(Ptr<BGSSaveLoadGame>, u32) -> u32),
        entry!(0x0084aad0, fn_0084aad0(Ptr<BGSSaveLoadGame>, u32) -> bool),
        entry!(
            0x0084ab00,
            bgssaveloadgame_save_game_warning_function(Ptr<BGSSaveLoadGame>, u32, u32)
        ),
        entry!(
            0x0084ab20,
            bgssaveloadgame_cleanup_expired_data(Ptr<BGSSaveLoadGame>)
        ),
        entry!(0x0084b450, fn_0084b450(Ptr) -> u8),
        entry!(0x0084b470, fn_0084b470(Ptr, u32)),
        entry!(0x0084b490, fn_0084b490(Ptr, u32, u32)),
        entry!(
            0x0084b4b0,
            fn_0084b4b0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0084b4e0, fn_0084b4e0(Ptr, u32) -> Ptr),
        entry!(0x0084b510, fn_0084b510(Ptr, u32) -> Ptr),
        entry!(0x0084b540, fn_0084b540(Ptr, u32) -> Ptr),
        entry!(0x0084b570, fn_0084b570(Ptr) -> Ptr),
        entry!(0x0084b5a0, fn_0084b5a0(Ptr)),
        entry!(
            0x0084b650,
            fn_0084b650(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0084b6c0, fn_0084b6c0(Ptr)),
        entry!(
            0x0084b750,
            fn_0084b750(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0084b7c0, fn_0084b7c0(Ptr, u32, u32, u32)),
        entry!(0x0084b8b0, fn_0084b8b0(Ptr, u32, u32, u32) -> bool),
        entry!(0x0084b930, fn_0084b930(Ptr<NiTPointerMap>, u32, u32) -> u32),
        entry!(0x0084b950, fn_0084b950(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x0084b990, fn_0084b990(Ptr, u32, u32, u32, u32)),
        entry!(0x0084b9c0, fn_0084b9c0(Ptr)),
        entry!(0x0084ba20, fn_0084ba20(Ptr)),
        entry!(0x0084ba50, fn_0084ba50(Ptr) -> u32),
        entry!(0x0084ba70, fn_0084ba70(Ptr, u32)),
        entry!(0x0084baa0, fn_0084baa0(Ptr, u32) -> Ptr),
        entry!(0x0084bad0, fn_0084bad0(Ptr, u32) -> Ptr),
        entry!(0x0084bb00, fn_0084bb00(Ptr, u32) -> Ptr),
        entry!(0x0084bb30, fn_0084bb30(Ptr) -> u32),
        entry!(0x0084bc20, fn_0084bc20(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every function outside this file that the translations call: each gets a
    /// do-nothing double first, so no other unit's translation runs for real.
    const EXTERNAL: [u32; 162] = [
        0x0040_1000,
        0x0040_1030,
        0x0040_1170,
        0x0040_37f0,
        0x0040_3d30,
        0x0040_4dc0,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0042_26e0,
        0x0042_5fd0,
        0x0042_8110,
        0x0042_8130,
        0x0042_ce10,
        0x0042_ce30,
        0x0042_ce50,
        0x0043_8af0,
        0x0044_0e30,
        0x0044_ddc0,
        0x0044_dee0,
        0x0044_edb0,
        0x0045_11e0,
        0x0045_43c0,
        0x0045_6520,
        0x0045_9920,
        0x0045_cda0,
        0x0046_12b0,
        0x0046_23f0,
        0x0046_5010,
        0x0046_9860,
        0x0046_a010,
        0x0047_0470,
        0x0047_a110,
        0x0048_39c0,
        0x004b_9ba0,
        0x0050_f9c0,
        0x0051_f550,
        0x0054_ca90,
        0x0055_9450,
        0x0057_5d70,
        0x0066_65a0,
        0x0068_15c0,
        0x006a_7ad0,
        0x006b_7f20,
        0x006e_2080,
        0x0070_37c0,
        0x0075_f6f0,
        0x007a_f430,
        0x007c_b2e0,
        0x007d_6bd0,
        0x0081_db60,
        0x0081_db90,
        0x0084_0f70,
        0x0084_1100,
        0x0084_1120,
        0x0084_33d0,
        0x0084_35a0,
        0x0084_3630,
        0x0084_3750,
        0x0084_39e0,
        0x0084_4700,
        0x0084_54f0,
        0x0084_5550,
        0x0084_56e0,
        0x0084_5760,
        0x0084_57b0,
        0x0084_58b0,
        0x0084_5960,
        0x0084_5a20,
        0x0084_5a80,
        0x0084_5cc0,
        0x0084_5d00,
        0x0084_5d20,
        0x0084_6300,
        0x0084_6330,
        0x0084_6400,
        0x0084_6440,
        0x0084_6490,
        0x0084_6b60,
        0x0084_6be0,
        0x0084_6e00,
        0x0084_6e70,
        0x0084_7d00,
        0x0084_7db0,
        0x0084_7dd0,
        0x0084_9a90,
        0x0084_9d00,
        0x0084_a3a0,
        0x0084_a5b0,
        0x0084_a690,
        0x0084_a880,
        0x0084_b4b0,
        0x0084_b570,
        0x0084_b5a0,
        0x0084_b5c0,
        0x0084_b6c0,
        0x0084_bb50,
        0x0084_bc20,
        0x0084_bfb0,
        0x0084_c110,
        0x0084_c270,
        0x0084_ded0,
        0x0084_df30,
        0x0084_dff0,
        0x0084_e0d0,
        0x0084_e190,
        0x0084_e2c0,
        0x0084_e3a0,
        0x0084_e6a0,
        0x0084_e730,
        0x0084_e930,
        0x0084_eb80,
        0x0084_f330,
        0x0084_fec0,
        0x0085_0b40,
        0x0085_0c20,
        0x0085_1110,
        0x0085_1120,
        0x0085_2950,
        0x0085_2a70,
        0x0085_2ae0,
        0x0085_2b60,
        0x0085_2c00,
        0x0085_35e0,
        0x0085_3640,
        0x0086_43b0,
        0x0086_44b0,
        0x0086_4540,
        0x0086_4580,
        0x0086_46b0,
        0x0086_46f0,
        0x0086_4740,
        0x0086_4980,
        0x0086_49a0,
        0x0086_51d0,
        0x0086_5220,
        0x0086_5360,
        0x0086_59c0,
        0x0086_5a30,
        0x0086_5ad0,
        0x0086_5bd0,
        0x0086_5c10,
        0x0086_5c40,
        0x0086_5e50,
        0x0086_5e70,
        0x0086_6200,
        0x0086_62c0,
        0x0086_6390,
        0x0086_6480,
        0x0086_6570,
        0x0086_65b0,
        0x0086_7a20,
        0x0086_7f90,
        0x0086_cf00,
        0x0089_1170,
        0x008c_71b0,
        0x008d_6f30,
        0x008d_8520,
        0x009a_4250,
        0x00b6_0040,
        0x00c3_e310,
        0x00c3_e340,
        0x00c6_a870,
    ];

    /// The base of the fake targets the test vtable points at: slot `s` of an
    /// object's vtable is `FAKE_TARGET + s`.
    const FAKE_TARGET: u32 = 0x7000_0000;
    /// Address of the test vtable.
    const TEST_VTABLE: u32 = 0x6000_0000;

    fn rv(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine where every external function is a do-nothing double, the
    /// allocator, `memset` and the small getters behave, the globals' pages
    /// are mapped, and the test vtable is in place; the call log is on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in EXTERNAL {
            e.register(address, |_, _| Ret::default());
        }
        e.map(0x011c_0000, 0x0004_5000);
        e.map(TEST_VTABLE, 0x1000);
        for slot in (0..0x260).step_by(4) {
            e.mem.set_u32(TEST_VTABLE + slot, FAKE_TARGET + slot);
            e.register(FAKE_TARGET + slot, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| rv(e.mem.alloc(a[0])));
        e.register(MEMORY_SET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(READ_WORD, |e, a| rv(e.mem.u32(a[0])));
        e.register(THIS_IDENTITY, |_, a| rv(a[0]));
        e.register(FORM_BUFFER_HEADER_FLAGS, |e, a| {
            let word = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], word);
            rv(a[1])
        });
        e.register(CHANGE_FLAGS_STORE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            rv(a[0])
        });
        for constructor in [
            CHANGES_MAP_CONSTRUCT,
            FORM_ID_MAP_CONSTRUCT,
            REFERENCES_MAP_CONSTRUCT,
            QUEUED_SUB_BUFFERS_MAP_CONSTRUCT,
            CHANGED_FORM_ID_MAP_CONSTRUCT,
            HISTORY_CONSTRUCT,
            RECONSTRUCT_FORMS_CONSTRUCT,
            LOAD_FORM_BUFFER_CONSTRUCT,
        ] {
            e.register(constructor, |_, a| rv(a[0]));
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of the logged calls, in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// An object whose vtable is the test vtable.
    fn object(e: &mut Engine) -> u32 {
        let block = e.mem.alloc(0x300);
        e.mem.set_u32(block, TEST_VTABLE);
        block
    }

    fn game(e: &mut Engine) -> Ptr<BGSSaveLoadGame> {
        e.new_object::<BGSSaveLoadGame>()
    }

    // ---- 00846f30 and the destructors ---------------------------------------

    #[test]
    fn constructor_builds_the_owned_objects() {
        let mut e = engine();
        let this = game(&mut e);
        let result = bgssaveloadgame_bgssaveloadgame(&mut e, this);
        assert_eq!(result, this);

        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 0);
        assert_eq!(e.get(this, BGSSaveLoadGame::cCurrentMinorVersion), 0xff);
        assert_eq!(e.get(this, BGSSaveLoadGame::pOldChangesMap), Ptr::NULL);
        let members = [
            e.get(this, BGSSaveLoadGame::pChangesMap),
            e.get(this, BGSSaveLoadGame::pFormIDMap),
            e.get(this, BGSSaveLoadGame::pWorldspaceFormIDMap),
            e.get(this, BGSSaveLoadGame::pReferencesMap),
            e.get(this, BGSSaveLoadGame::pQueuedSubBuffersMap),
            e.get(this, BGSSaveLoadGame::pChangedFormIDMap),
            e.get(this, BGSSaveLoadGame::pHistory),
            e.get(this, BGSSaveLoadGame::pReconstructForms),
        ];
        assert!(members.iter().all(|m| !m.is_null()));
        let sizes: Vec<u32> = calls(&e, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x10, 0x24, 0x24, 0x30, 0x30, 0x10, 0x10, 0x34]);
        assert_eq!(
            calls(&e, FORM_ID_MAP_CONSTRUCT),
            vec![
                vec![members[1].addr(), 0x13af],
                vec![members[2].addr(), 0x25]
            ]
        );
        assert_eq!(
            calls(&e, CHANGED_FORM_ID_MAP_CONSTRUCT),
            vec![vec![members[5].addr(), 0x25]]
        );
        assert_eq!(
            calls(&e, PACKAGE_LOCATION_MAP_CONSTRUCT),
            vec![vec![this.addr() + 0x34, 0x25]]
        );
        assert_eq!(calls(&e, GAME_SET_FLAG_ONE), vec![vec![this.addr(), 0]]);
        assert_eq!(
            calls(&e, MEMORY_SET),
            vec![
                vec![this.addr() + 0x44, 0xff, 0xff],
                vec![this.addr() + 0x143, 0xff, 0xff]
            ]
        );
        assert_eq!(e.mem.u8(this.addr() + 0x44 + 0xfe), 0xff);
        assert_eq!(e.mem.u8(this.addr() + 0x143 + 0xfe), 0xff);
    }

    #[test]
    fn constructor_leaves_a_member_null_when_the_allocation_fails() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = game(&mut e);
        bgssaveloadgame_bgssaveloadgame(&mut e, this);
        assert_eq!(e.get(this, BGSSaveLoadGame::pChangesMap), Ptr::NULL);
        assert_eq!(e.get(this, BGSSaveLoadGame::pReconstructForms), Ptr::NULL);
        assert!(calls(&e, HISTORY_CONSTRUCT).is_empty());
        assert!(calls(&e, CHANGES_MAP_CONSTRUCT).is_empty());
    }

    #[test]
    fn destructor_deletes_every_member_and_the_two_containers() {
        let mut e = engine();
        let this = game(&mut e);
        let mut blocks = std::collections::BTreeMap::new();
        for offset in [0x00u32, 0x18, 0x20] {
            let o = object(&mut e);
            e.mem.set_u32(this.addr() + offset, o);
            blocks.insert(offset, o);
        }
        for offset in [0x08u32, 0x0c, 0x10, 0x14, 0x1c] {
            let o = e.mem.alloc(0x40);
            e.mem.set_u32(this.addr() + offset, o);
            blocks.insert(offset, o);
        }
        fn_008471f0(&mut e, this);
        assert_eq!(
            order(&e),
            vec![
                FAKE_TARGET,
                0x0084_73c0,
                FORM_ID_MAP_DESTRUCT,
                OPERATOR_DELETE,
                0x0084_73c0,
                FORM_ID_MAP_DESTRUCT,
                OPERATOR_DELETE,
                0x0084_73f0,
                REFERENCES_MAP_DESTRUCT,
                OPERATOR_DELETE,
                0x0084_7420,
                QUEUED_SUB_BUFFERS_MAP_DESTRUCT,
                OPERATOR_DELETE,
                FAKE_TARGET,
                0x0084_7450,
                HISTORY_DESTRUCT,
                OPERATOR_DELETE,
                FAKE_TARGET,
                PACKAGE_LOCATION_MAP_DESTRUCT,
                FORM_BUFFER_ARRAY_DESTRUCT,
            ]
        );
        let virtual_calls = calls(&e, FAKE_TARGET);
        assert_eq!(virtual_calls[0], vec![blocks[&0x00], 1]);
        assert_eq!(virtual_calls[1], vec![blocks[&0x18], 1]);
        assert_eq!(virtual_calls[2], vec![blocks[&0x20], 1]);
        assert_eq!(
            calls(&e, PACKAGE_LOCATION_MAP_DESTRUCT),
            vec![vec![this.addr() + 0x34]]
        );
        assert_eq!(
            calls(&e, FORM_BUFFER_ARRAY_DESTRUCT),
            vec![vec![this.addr() + 0x24]]
        );
    }

    #[test]
    fn destructor_skips_null_members() {
        let mut e = engine();
        let this = game(&mut e);
        fn_008471f0(&mut e, this);
        assert_eq!(
            order(&e),
            vec![PACKAGE_LOCATION_MAP_DESTRUCT, FORM_BUFFER_ARRAY_DESTRUCT]
        );
    }

    /// Runs one scalar deleting destructor with flags 0, 1 and 3 and checks
    /// the body runs every time and the delete only when bit 0 is set.
    fn check_scalar_destructor(function: fn(&mut Engine, Ptr, u32) -> Ptr, body: u32) {
        let mut e = engine();
        let this = Ptr::new(0x1111_0000);
        assert_eq!(function(&mut e, this, 0), this);
        assert_eq!(order(&e), vec![body]);
        assert_eq!(calls(&e, body), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        assert_eq!(function(&mut e, this, 1), this);
        assert_eq!(order(&e), vec![body, OPERATOR_DELETE]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        function(&mut e, this, 3);
        assert_eq!(order(&e), vec![body, OPERATOR_DELETE]);
    }

    #[test]
    fn form_id_map_scalar_deleting_destructor() {
        check_scalar_destructor(fn_008473c0, FORM_ID_MAP_DESTRUCT);
    }

    #[test]
    fn references_map_scalar_deleting_destructor() {
        check_scalar_destructor(fn_008473f0, REFERENCES_MAP_DESTRUCT);
    }

    #[test]
    fn queued_sub_buffers_map_scalar_deleting_destructor() {
        check_scalar_destructor(fn_00847420, QUEUED_SUB_BUFFERS_MAP_DESTRUCT);
    }

    #[test]
    fn history_scalar_deleting_destructor() {
        check_scalar_destructor(fn_00847450, HISTORY_DESTRUCT);
    }

    #[test]
    fn create_stores_the_new_object_and_runs_the_follow_up() {
        let mut e = engine();
        e.register(GAME_CONSTRUCT, |_, a| rv(a[0]));
        bgssaveloadgame_create(&mut e);
        let created = e.global::<u32>(SAVE_LOAD_GAME);
        assert_ne!(created, 0);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x24c]]);
        assert_eq!(calls(&e, GAME_CONSTRUCT), vec![vec![created]]);
        assert_eq!(
            order(&e),
            vec![OPERATOR_NEW, GAME_CONSTRUCT, GAME_CREATE_FOLLOW_UP]
        );
    }

    #[test]
    fn create_stores_null_when_the_allocation_fails() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.set_global(SAVE_LOAD_GAME, 0x55u32);
        bgssaveloadgame_create(&mut e);
        assert_eq!(e.global::<u32>(SAVE_LOAD_GAME), 0);
        assert!(calls(&e, GAME_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, GAME_CREATE_FOLLOW_UP).len(), 1);
    }

    #[test]
    fn destroy_deletes_the_global_object_and_clears_it() {
        let mut e = engine();
        let this = game(&mut e);
        e.set_global(SAVE_LOAD_GAME, this.addr());
        fn_00847500(&mut e);
        assert_eq!(e.global::<u32>(SAVE_LOAD_GAME), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, PACKAGE_LOCATION_MAP_DESTRUCT).len(), 1);
    }

    #[test]
    fn destroy_does_nothing_without_an_object() {
        let mut e = engine();
        fn_00847500(&mut e);
        assert_eq!(e.global::<u32>(SAVE_LOAD_GAME), 0);
        assert!(order(&e).is_empty());
    }

    #[test]
    fn game_scalar_deleting_destructor() {
        let mut e = engine();
        let this = game(&mut e);
        assert_eq!(fn_00847540(&mut e, this.cast(), 0), this.cast());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&e, FORM_BUFFER_ARRAY_DESTRUCT).len(), 1);
        fn_00847540(&mut e, this.cast(), 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
    }

    #[test]
    fn current_minor_version_is_the_byte_at_0x248() {
        let mut e = engine();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::cCurrentMinorVersion, 0x1b);
        assert_eq!(
            bgssaveloadgame_get_current_minor_version(&mut e, this),
            0x1b
        );
    }

    // ---- plugin list ----------------------------------------------------------

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8_lossy(&e.mem.cstr(address)).into_owned()
    }

    fn string(e: &mut Engine, s: &str) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s.as_bytes());
        block
    }

    /// Two compiled files named `Fallout.esm` and `Dlc.esm` in the data handler.
    fn two_compiled_files(e: &mut Engine) {
        let data_handler = e.mem.alloc(0x640);
        e.set_global(DATA_HANDLER, data_handler);
        e.register(DATA_HANDLER_COMPILED_FILE_COUNT, |_, _| rv(2));
        e.register(DATA_HANDLER_GET_COMPILED_FILE, |_, a| rv(0x2000 + a[1]));
        let first = string(e, "Fallout.esm");
        let second = string(e, "Dlc.esm");
        e.set_global(0x011c_4000u32, first);
        e.set_global(0x011c_4004u32, second);
        e.register(COMPILED_FILE_NAME, |e, a| {
            rv(e.mem.u32(0x011c_4000 + 4 * (a[0] - 0x2000)))
        });
    }

    #[test]
    fn save_plugin_list_writes_the_count_and_every_name() {
        let mut e = engine();
        two_compiled_files(&mut e);
        let this = game(&mut e);
        let file = Ptr::new(0x3000_0000);
        let count_seen = std::rc::Rc::new(std::cell::RefCell::new(0u8));
        let seen = count_seen.clone();
        e.register_double(SAVE_GAME_BUFFER_SAVE_BYTES, move |e, a| {
            *seen.borrow_mut() = e.mem.u8(a[1]);
            Ret::default()
        });
        bgssaveloadgame_save_plugin_list(&mut e, this, file);
        assert_eq!(*count_seen.borrow(), 2);
        let buffer = calls(&e, SAVE_GAME_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, SAVE_GAME_BUFFER_SAVE_BYTES)[0][0], buffer);
        assert_eq!(&calls(&e, SAVE_GAME_BUFFER_SAVE_BYTES)[0][2..], &[1, 0]);
        let names: Vec<String> = calls(&e, SAVE_GAME_BUFFER_SAVE_STRING)
            .iter()
            .map(|w| text(&e, w[1]))
            .collect();
        assert_eq!(names, vec!["Fallout.esm", "Dlc.esm"]);
        assert_eq!(
            calls(&e, SAVE_GAME_BUFFER_SAVE),
            vec![vec![buffer, file.addr()]]
        );
        assert_eq!(calls(&e, SAVE_GAME_BUFFER_DESTRUCT), vec![vec![buffer]]);
        assert_eq!(
            order(&e).last(),
            Some(&SAVE_GAME_BUFFER_DESTRUCT),
            "the buffer is destroyed last"
        );
    }

    /// Makes the load buffer return `count` and then the given names.
    fn saved_plugins(e: &mut Engine, names: &[&str]) {
        e.register_double(LOAD_GAME_BUFFER_LOAD_BYTES, {
            let count = names.len() as u8;
            move |e, a| {
                e.mem.set_u8(a[1], count);
                Ret::default()
            }
        });
        let blocks: Vec<u32> = names.iter().map(|n| string(e, n)).collect();
        let next = std::rc::Rc::new(std::cell::RefCell::new(0usize));
        e.register_double(LOAD_GAME_BUFFER_LOAD_STRING, move |e, a| {
            let index = *next.borrow();
            *next.borrow_mut() += 1;
            let name = e.mem.cstr(blocks[index]);
            e.mem.set_cstr(a[1], &name);
            Ret::default()
        });
        e.register(STRING_COMPARE_IGNORE_CASE, |e, a| {
            let (x, y) = (text(e, a[0]), text(e, a[1]));
            rv(!x.eq_ignore_ascii_case(&y) as u32)
        });
    }

    #[test]
    fn load_plugin_list_maps_the_saved_plugins_and_warns_about_a_missing_one() {
        let mut e = engine();
        two_compiled_files(&mut e);
        saved_plugins(&mut e, &["DLC.ESM", "Missing.esp", "fallout.ESM"]);
        let warned = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let names = warned.clone();
        e.register_double(SAVE_GAME_WARNING, move |e, a| {
            assert_eq!(a[0], MISSING_PLUGIN_WARNING);
            names.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        let this = game(&mut e);
        let file = Ptr::new(0x3000_0000);
        assert!(!bgssaveloadgame_load_plugin_list(&mut e, this, file));
        // save index -> loaded index; loaded index -> save index
        assert_eq!(e.mem.u8(this.addr() + 0x44), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x45), 0xff);
        assert_eq!(e.mem.u8(this.addr() + 0x46), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x143), 2);
        assert_eq!(e.mem.u8(this.addr() + 0x144), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x145), 0xff);
        let warnings = warned.borrow();
        assert_eq!(*warnings, vec!["Missing.esp".to_string()]);
        assert_eq!(calls(&e, LOAD_GAME_BUFFER_LOAD).len(), 1);
        assert_eq!(calls(&e, LOAD_GAME_BUFFER_DESTRUCT).len(), 1);
    }

    #[test]
    fn load_plugin_list_succeeds_when_every_plugin_is_found() {
        let mut e = engine();
        two_compiled_files(&mut e);
        saved_plugins(&mut e, &["Fallout.esm", "Dlc.esm"]);
        let this = game(&mut e);
        assert!(bgssaveloadgame_load_plugin_list(
            &mut e,
            this,
            Ptr::new(0x3000_0000)
        ));
        assert_eq!(e.mem.u8(this.addr() + 0x44), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x45), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x144), 1);
        assert!(calls(&e, SAVE_GAME_WARNING).is_empty());
    }

    // ---- SaveGame and its helpers ---------------------------------------------

    #[test]
    fn flag_removal_builds_the_flags_without_the_mask() {
        let mut e = engine();
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0b1111_0110);
        let out = e.mem.alloc(4);
        let result = fn_00847cd0(&mut e, Ptr::new(source), Ptr::new(out), 0b0101_0100);
        assert_eq!(result, Ptr::new(out));
        assert_eq!(e.mem.u32(out), 0b1010_0010);
        assert_eq!(calls(&e, CHANGE_FLAGS_STORE), vec![vec![out, 0b1010_0010]]);
    }

    #[test]
    fn header_clear_zeroes_0x6e_bytes() {
        let mut e = engine();
        let header = e.mem.alloc(0x80);
        for offset in 0..0x80 {
            e.mem.set_u8(header + offset, 0xaa);
        }
        assert_eq!(fn_00847d50(&mut e, Ptr::new(header)), Ptr::new(header));
        assert_eq!(calls(&e, MEMORY_SET), vec![vec![header, 0, 0x6e]]);
        assert_eq!(e.mem.u8(header + 0x6d), 0);
        assert_eq!(e.mem.u8(header + 0x6e), 0xaa);
    }

    #[test]
    fn first_global_block_is_saved_as_0_to_0xc() {
        let mut e = engine();
        e.register(SAVE_GLOBAL_DATA, |_, _| rv(0x99));
        assert_eq!(fn_00847d70(&mut e, Ptr::new(0x3000_0000)), 0x99);
        assert_eq!(calls(&e, SAVE_GLOBAL_DATA), vec![vec![0x3000_0000, 0, 0xc]]);
    }

    #[test]
    fn second_global_block_is_saved_as_1000_to_1001() {
        let mut e = engine();
        e.register(SAVE_GLOBAL_DATA, |_, _| rv(0x98));
        assert_eq!(fn_00847d90(&mut e, Ptr::new(0x3000_0000)), 0x98);
        assert_eq!(
            calls(&e, SAVE_GLOBAL_DATA),
            vec![vec![0x3000_0000, 1000, 1001]]
        );
    }

    /// Everything `SaveGame` needs, with four entries in the changes map:
    /// `0x100` has a stored buffer (an unloaded form), `0x200` has no form,
    /// `0x300` is a form that gets saved, `0x400` is a form whose changes are
    /// all known already.
    struct SaveSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        file: Ptr,
        unloaded_value: u32,
        saved_value: u32,
        saved_form: u32,
        headers: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    }

    fn save_setup() -> SaveSetup {
        let mut e = engine();
        let this = game(&mut e);
        let history = e.mem.alloc(0x10);
        e.set(this, BGSSaveLoadGame::pHistory, Ptr::new(history));
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.set(this, BGSSaveLoadGame::pFormIDMap, Ptr::new(0x4000_0100));
        e.set(
            this,
            BGSSaveLoadGame::pWorldspaceFormIDMap,
            Ptr::new(0x4000_0200),
        );
        e.set_global(MANAGER_INSTANCE, 0x5000_0000u32);
        e.register(MANAGER_GET_MINOR_VERSION, |_, _| rv(0x1b));
        e.register(MANAGER_COPY_FILE_NAME, |e, a| {
            e.mem.set_cstr(a[2], b"Save 1.fos");
            Ret::default()
        });
        e.register(MANAGER_GET_VERSION_INFO, |e, a| {
            e.mem.set_cstr(a[1], b"1.4.0.525");
            Ret::default()
        });
        let position = std::rc::Rc::new(std::cell::RefCell::new(0u32));
        e.register_double(FILE_GET_POSITION, move |_, _| {
            *position.borrow_mut() += 100;
            rv(*position.borrow())
        });
        let block = std::rc::Rc::new(std::cell::RefCell::new(0u32));
        e.register_double(SAVE_GLOBAL_DATA, move |_, _| {
            *block.borrow_mut() += 1;
            rv(0xa0 + *block.borrow())
        });
        let headers = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let written = headers.clone();
        e.register_double(FILE_WRITE, move |e, a| {
            written.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });

        // The changes map walk.
        let unloaded_value = e.mem.alloc(8);
        e.mem.set_u32(unloaded_value, 5);
        e.mem.set_u32(unloaded_value + 4, 0x77);
        let missing_value = e.mem.alloc(8);
        e.mem.set_u32(missing_value, 6);
        let saved_value = e.mem.alloc(8);
        e.mem.set_u32(saved_value, 6);
        let known_value = e.mem.alloc(8);
        e.mem.set_u32(known_value, 8);
        let entries = [
            (0x100u32, unloaded_value),
            (0x200, missing_value),
            (0x300, saved_value),
            (0x400, known_value),
        ];
        e.register(MAP_FIRST_POSITION, |_, _| rv(1));
        let index = std::rc::Rc::new(std::cell::RefCell::new(0usize));
        e.register_double(MAP_NEXT, move |e, a| {
            let i = *index.borrow();
            e.mem.set_u32(a[2], entries[i].0);
            e.mem.set_u32(a[3], entries[i].1);
            *index.borrow_mut() += 1;
            e.mem.set_u32(
                a[1],
                if i + 1 < entries.len() {
                    i as u32 + 2
                } else {
                    0
                },
            );
            Ret::default()
        });
        e.register(READ_WORD, |e, a| rv(e.mem.u32(a[0])));
        let saved_form = object(&mut e);
        let known_form = object(&mut e);
        e.register_double(LOOKUP_FORM, move |_, a| {
            rv(match a[0] {
                0x300 => saved_form,
                0x400 => known_form,
                _ => 0,
            })
        });
        e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        e.register(CHANGES_MAP_CLEAR_FLAGS, |_, a| rv((a[1] == 0x400) as u32));
        SaveSetup {
            e,
            this,
            file: Ptr::new(0x3000_0000),
            unloaded_value,
            saved_value,
            saved_form,
            headers,
        }
    }

    #[test]
    fn save_game_writes_the_forms_and_the_header() {
        let mut s = save_setup();
        bgssaveloadgame_save_game(&mut s.e, s.this, s.file);
        let e = &s.e;
        let file = s.file.addr();

        // The saving flag brackets the whole save.
        assert_eq!(
            calls(e, GAME_SET_SAVING_FLAG),
            vec![vec![s.this.addr(), 1], vec![s.this.addr(), 0]]
        );
        assert_eq!(order(e)[0], GAME_SET_SAVING_FLAG);
        assert_eq!(*order(e).last().unwrap(), SAVE_FORM_BUFFER_DESTRUCT);

        // The start note names the file and the version.
        let note = &calls(e, HISTORY_ADD_NOTE)[0];
        assert_eq!(note[1], SAVING_NOTE);
        assert_eq!(text(e, note[2]), "Save 1.fos");
        assert_eq!(text(e, note[3]), "1.4.0.525");
        assert_eq!(calls(e, HISTORY_ADD_NOTE)[1][1], FINISHED_SAVING_NOTE);
        assert_eq!(calls(e, HISTORY_ADD_NOTE)[1][2], note[2]);

        // The minor version is written as one byte.
        let version = &calls(e, SAVE_DATA_TO_FILE)[0];
        assert_eq!((version[0], version[2]), (file, 1));
        assert_eq!(e.mem.u8(version[1]), 0x1b);
        assert_eq!(calls(e, SAVE_GAME_BUFFER_SAVE).len(), 1, "plugin list");

        // The unloaded form goes through its own buffer.
        assert_eq!(
            calls(e, UNLOADED_FORM_BUFFER_SAVE),
            vec![vec![s.unloaded_value + 4, file, 0x100, 5]]
        );
        // The form without a live object is reported.
        assert_eq!(
            calls(e, SAVE_GAME_WARNING),
            vec![vec![SAVE_FORM_MISSING_WARNING, 0x200, 6]]
        );
        // Both forms are captured into the buffer; only the first is saved.
        let buffer = calls(e, SAVE_FORM_BUFFER_CONSTRUCT)[0][0];
        let second_form = calls(e, SAVE_FORM_BUFFER_SET_FORM)[1][1];
        assert_eq!(
            calls(e, SAVE_FORM_BUFFER_SET_HEADER),
            vec![
                vec![buffer, 0x300, 6, 0x2a, 0x1b],
                vec![buffer, 0x400, 8, 0x2a, 0x1b]
            ]
        );
        assert_eq!(
            calls(e, SAVE_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, s.saved_form], vec![buffer, second_form]]
        );
        assert_eq!(
            calls(e, SAVE_INITIAL_DATA),
            vec![vec![buffer, s.saved_form, 6]]
        );
        assert_eq!(
            calls(e, FAKE_TARGET + 0x80),
            vec![vec![s.saved_form, buffer], vec![second_form, buffer]]
        );
        assert_eq!(
            calls(e, FAKE_TARGET + 0x54),
            vec![vec![s.saved_form, buffer]]
        );
        assert_eq!(calls(e, SAVE_FORM_BUFFER_SAVE), vec![vec![buffer, file]]);
        assert_eq!(
            calls(e, CHANGES_MAP_CLEAR_FLAGS),
            vec![vec![0x4000_0000, 0x300, 6], vec![0x4000_0000, 0x400, 8]]
        );
        let _ = s.saved_value;

        // The two maps and the history follow, then the header is rewritten.
        assert_eq!(
            calls(e, FORM_ID_MAP_SAVE),
            vec![vec![0x4000_0100, file], vec![0x4000_0200, file]]
        );
        assert_eq!(
            calls(e, HISTORY_SAVE),
            vec![vec![e.get(s.this, BGSSaveLoadGame::pHistory).addr(), file]]
        );
        assert_eq!(calls(e, FILE_SEEK_SET), vec![vec![file, 100]]);
        let headers = s.headers.borrow();
        assert_eq!(headers.len(), 2);
        assert_eq!(
            headers[0],
            vec![0u8; 0x6e],
            "first write is the cleared header"
        );
        let word = |at: usize| u32::from_le_bytes(headers[1][at..at + 4].try_into().unwrap());
        assert_eq!(word(0x00), 500, "form id maps position");
        assert_eq!(word(0x04), 600, "after the maps");
        assert_eq!(word(0x08), 200, "right after the header");
        assert_eq!(word(0x0c), 300, "after the first global block");
        assert_eq!(word(0x10), 400, "before the second global block");
        assert_eq!(word(0x14), 0xa1, "first global block result");
        assert_eq!(word(0x18), 0xa2, "second global block result");
        assert_eq!(word(0x1c), 2, "the unloaded form and the saved one");
        assert_eq!(
            calls(e, SAVE_GLOBAL_DATA),
            vec![vec![file, 0, 0xc], vec![file, 1000, 1001]]
        );
    }

    // ---- small helpers ----------------------------------------------------------

    #[test]
    fn load_flag_is_set_and_cleared_without_touching_other_bits() {
        let mut e = engine();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::iGlobalFlags, 0x81);
        fn_00848ca0(&mut e, this, true);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 0x83);
        fn_00848ca0(&mut e, this, false);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 0x81);
        fn_00848ca0(&mut e, this, false);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 0x81);
    }

    #[test]
    fn quiet_flag_is_bit_7() {
        let mut e = engine();
        let this = game(&mut e);
        assert!(!fn_00848cf0(&mut e, this));
        e.set(this, BGSSaveLoadGame::iGlobalFlags, 0x7f);
        assert!(!fn_00848cf0(&mut e, this));
        e.set(this, BGSSaveLoadGame::iGlobalFlags, 0x80);
        assert!(fn_00848cf0(&mut e, this));
    }

    #[test]
    fn global_loader_a_runs_zero_to_0xc() {
        let mut e = engine();
        fn_00848d10(&mut e);
        assert_eq!(calls(&e, LOAD_GLOBAL_DATA_A), vec![vec![0, 0xc]]);
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn global_loader_b_runs_zero_to_0xc() {
        let mut e = engine();
        fn_00848d30(&mut e);
        assert_eq!(calls(&e, LOAD_GLOBAL_DATA_B), vec![vec![0, 0xc]]);
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn delete_buffer_frees_the_data_and_clears_the_fields() {
        let mut e = engine();
        let this = e.new_object::<BGSLoadGameBuffer>();
        e.set(this, BGSLoadGameBuffer::pBuffer, Ptr::new(0x2222_0000));
        e.set(this, BGSLoadGameBuffer::iBufferSize, 77);
        e.set(this, BGSLoadGameBuffer::iBufferPosition, 5);
        bgsloadgamebuffer_delete_buffer(&mut e, this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2222_0000]]);
        assert_eq!(e.get(this, BGSLoadGameBuffer::pBuffer), Ptr::NULL);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferSize), 0);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 0);
    }

    #[test]
    fn skip_flag_of_a_form_buffer_is_bit_0() {
        let mut e = engine();
        let this = e.new_object::<BGSLoadFormBuffer>();
        e.set(this, BGSLoadFormBuffer::iFlags, 0x10);
        assert!(!fn_00848dd0(&mut e, this));
        fn_00848d90(&mut e, this, true);
        assert_eq!(e.get(this, BGSLoadFormBuffer::iFlags), 0x11);
        assert!(fn_00848dd0(&mut e, this));
        fn_00848d90(&mut e, this, false);
        assert_eq!(e.get(this, BGSLoadFormBuffer::iFlags), 0x10);
    }

    #[test]
    fn loading_forms_byte_is_at_0x621() {
        let mut e = engine();
        let this = e.new_object::<TESDataHandler>();
        fn_00848df0(&mut e, this, 1);
        assert_eq!(e.mem.u8(this.addr() + 0x621), 1);
        fn_00848df0(&mut e, this, 0);
        assert_eq!(e.mem.u8(this.addr() + 0x621), 0);
    }

    #[test]
    fn player_reset_clears_the_word_and_the_list() {
        let mut e = engine();
        let player = e.mem.alloc(0xe00);
        e.mem.set_u32(player + 0xd68, 0x1234);
        fn_00848e10(&mut e, Ptr::new(player));
        assert_eq!(e.mem.u32(player + 0xd68), 0);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![player + 0x5fc]]);
    }

    #[test]
    fn menu_forwarder_needs_the_object() {
        let mut e = engine();
        fn_00848e40(&mut e, 1);
        assert!(order(&e).is_empty());
        e.set_global(MENU_OBJECT, 0x6666_0000u32);
        fn_00848e40(&mut e, 1);
        assert_eq!(
            calls(&e, MENU_OBJECT_FORWARD),
            vec![vec![0x6666_0000, 0x2000_0000, 1]]
        );
    }

    #[test]
    fn stored_buffer_type_is_read_through_the_pointer() {
        let mut e = engine();
        e.register(STORED_BUFFER_TYPE, |_, a| rv(a[0] + 1));
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x4040);
        assert_eq!(fn_00849220(&mut e, Ptr::new(cell)), 0x4041);
        assert_eq!(calls(&e, STORED_BUFFER_TYPE), vec![vec![0x4040]]);
    }

    #[test]
    fn form_buffer_bit_2_and_bit_4() {
        let mut e = engine();
        let this = e.new_object::<BGSLoadFormBuffer>();
        e.set(this, BGSLoadFormBuffer::iFlags, 1);
        fn_00849240(&mut e, this, true);
        assert_eq!(e.get(this, BGSLoadFormBuffer::iFlags), 5);
        fn_00849240(&mut e, this, false);
        assert_eq!(e.get(this, BGSLoadFormBuffer::iFlags), 1);
        assert!(!fn_00849590(&mut e, this));
        e.set(this, BGSLoadFormBuffer::iFlags, 0x10);
        assert!(fn_00849590(&mut e, this));
    }

    #[test]
    fn initial_data_struct_constructor_casts_its_two_members() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        assert_eq!(fn_00849280(&mut e, Ptr::new(this)), Ptr::new(this));
        assert_eq!(
            calls(&e, THIS_IDENTITY),
            vec![vec![this + 0x14], vec![this + 0x20]]
        );
    }

    #[test]
    fn unload_pass_flag_is_bit_3() {
        let mut e = engine();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::iGlobalFlags, 2);
        fn_00849540(&mut e, this, true);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 0xa);
        fn_00849540(&mut e, this, false);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags), 2);
    }

    #[test]
    fn io_manager_word_is_set_to_5() {
        let mut e = engine();
        let io = e.mem.alloc(0x70);
        fn_008495b0(&mut e, Ptr::new(io));
        assert_eq!(e.mem.u32(io + 0x68), 5);
    }

    #[test]
    fn set_at_maps_the_id_to_itself() {
        let mut e = engine();
        fn_00849a50(&mut e, Ptr::new(0x7777_0000), 0x1234);
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![0x7777_0000, 0x1234, 0x1234]]
        );
    }

    #[test]
    fn references_setup_runs_on_the_member_at_0x10() {
        let mut e = engine();
        fn_00849a70(&mut e, Ptr::new(0x7777_0000), 5, 6);
        assert_eq!(
            calls(&e, REFERENCES_MAP_SETUP),
            vec![vec![0x7777_0010, 5, 6]]
        );
    }

    // ---- LoadGame ---------------------------------------------------------------

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// True when `wanted` appears in `order` in that order (not necessarily
    /// adjacent).
    fn is_subsequence(order: &[u32], wanted: &[u32]) -> bool {
        let mut rest = order.iter();
        wanted.iter().all(|w| rest.any(|o| o == w))
    }

    /// A save with one form (`0x300`): header, live form and buffer doubles.
    struct LoadSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        file: Ptr,
        form: u32,
        player: u32,
        old_map: u32,
        old_form_id_map: u32,
        old_history: u32,
        new_map: Rc<Cell<u32>>,
        added: Rc<RefCell<Vec<u32>>>,
    }

    const LOAD_FILE: u32 = 0x3000_0000;

    fn load_setup() -> LoadSetup {
        let mut e = engine();
        let this = game(&mut e);
        let old_map = object(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(old_map));
        let old_form_id_map = e.mem.alloc(0x40);
        e.set(this, BGSSaveLoadGame::pFormIDMap, Ptr::new(old_form_id_map));
        e.set(
            this,
            BGSSaveLoadGame::pWorldspaceFormIDMap,
            Ptr::new(0x4000_0200),
        );
        let old_history = e.mem.alloc(0x10);
        e.set(this, BGSSaveLoadGame::pHistory, Ptr::new(old_history));
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0300));
        e.set(
            this,
            BGSSaveLoadGame::pReconstructForms,
            Ptr::new(0x4000_0400),
        );
        e.set(
            this,
            BGSSaveLoadGame::pQueuedSubBuffersMap,
            Ptr::new(0x4000_0500),
        );
        e.set(
            this,
            BGSSaveLoadGame::pChangedFormIDMap,
            Ptr::new(0x4000_0600),
        );
        let player = object(&mut e);
        e.set_global(PLAYER, player);
        e.set_global(MANAGER_INSTANCE, 0x5000_0000u32);
        let data_handler = e.mem.alloc(0x640);
        e.set_global(DATA_HANDLER, data_handler);
        e.set_global(WORLD, 0x1100_0000u32);
        let io_manager = e.mem.alloc(0x70);
        e.set_global(IO_MANAGER, io_manager);
        let form = object(&mut e);

        // The header the file holds.
        e.register(FILE_READ, |e, a| {
            for (offset, value) in [
                (0u32, 500u32),
                (0x14, 0x11),
                (0x18, 0x22),
                (0x1c, 1),
                (0x20, 0x33),
            ] {
                e.mem.set_u32(a[1] + offset, value);
            }
            Ret::default()
        });
        let position = Rc::new(Cell::new(0u32));
        e.register_double(FILE_GET_POSITION, move |_, _| {
            position.set(position.get() + 100);
            rv(position.get())
        });

        // The changes maps.
        let new_map = Rc::new(Cell::new(0u32));
        let remember = new_map.clone();
        e.register_double(CHANGES_MAP_CONSTRUCT, move |_, a| {
            remember.set(a[0]);
            rv(a[0])
        });
        let remember = new_map.clone();
        e.register_double(CHANGES_MAP_GET_FLAGS, move |e, a| {
            e.mem
                .set_u32(a[1], if a[0] == remember.get() { 0 } else { 5 });
            Ret::default()
        });

        // The form buffers: a vtable, the form they were set to, old flags.
        e.register(LOAD_FORM_BUFFER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_VTABLE);
            rv(a[0])
        });
        e.register(LOAD_FORM_BUFFER_SET_FORM, |e, a| {
            e.mem.set_u32(a[0] + 0x24, a[1]);
            Ret::default()
        });
        e.register(FAKE_TARGET + 4, |e, a| rv(e.mem.u32(a[0] + 0x24)));
        e.register(LOAD_FORM_BUFFER_SET_OLD_FLAGS, |e, a| {
            e.mem.set_u32(a[0] + 0x2c, a[1]);
            Ret::default()
        });
        e.register(LOAD_FORM_BUFFER_OLD_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], flags);
            rv(a[1])
        });
        e.register(LOAD_FORM_BUFFER_HEADER_TYPE, |_, _| rv(0x2a));
        e.register(BUFFER_FORM_ID, |_, _| rv(0x300));
        e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        e.register(GAME_CHECK_INITIAL_DATA, |_, _| rv(0x10));
        e.register(COMBINE_CHANGE_FLAGS, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(IS_DYNAMIC_FORM_ID, |_, a| rv((a[1] >= 0xff00_0000) as u32));
        e.register_double(LOOKUP_FORM, move |_, _| rv(form));
        e.register(FORM_TYPE_NAME, |_, _| rv(0xaa));
        e.register(TYPE_NAME_FOR, |_, _| rv(0xbb));
        e.register(FAKE_TARGET + 0x130, |_, _| rv(0xcc));

        // The form buffer array.
        let cells = e.mem.alloc(0x40);
        let added = Rc::new(RefCell::new(Vec::new()));
        let record = added.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            let buffer = e.mem.u32(a[1]);
            let index = record.borrow().len() as u32;
            e.mem.set_u32(cells + 4 * index, buffer);
            record.borrow_mut().push(buffer);
            Ret::default()
        });
        e.register_double(ARRAY_ELEMENT_ADDRESS, move |_, a| rv(cells + 4 * a[1]));

        LoadSetup {
            e,
            this,
            file: Ptr::new(LOAD_FILE),
            form,
            player,
            old_map,
            old_form_id_map,
            old_history,
            new_map,
            added,
        }
    }

    #[test]
    fn load_game_loads_one_form_from_start_to_end() {
        let mut s = load_setup();
        let (this, file, form) = (s.this, s.file.addr(), s.form);
        assert!(bgssaveloadgame_load_game(&mut s.e, this, s.file, false));
        let e = &s.e;
        let buffer = s.added.borrow()[0];
        let new_map = s.new_map.get();
        let new_history = e.get(this, BGSSaveLoadGame::pHistory).addr();

        assert!(is_subsequence(
            &order(e),
            &[
                HISTORY_ADD_NOTE,
                LOAD_DATA_FROM_FILE,
                LOAD_GAME_BUFFER_LOAD,
                REFERENCES_MAP_RESET,
                MANAGER_START_LOAD,
                IO_MANAGER_PAUSE,
                FILE_READ,
                FORM_ID_MAP_LOAD,
                FORM_ID_MAP_LOAD,
                HISTORY_LOAD,
                HISTORY_ADD_HISTORY,
                GLOBAL_DATA_RESET,
                LOAD_GLOBAL_BLOCK,
                ARRAY_SET_RESERVED_SIZE,
                LOAD_FORM_BUFFER_LOAD_HEADER,
                RECONSTRUCT_ADD_FORM,
                ARRAY_ADD,
                RECONSTRUCT_FINISH_A,
                AFTER_LOAD_FINISH,
                GAME_AFTER_LOAD_PASS,
                LOAD_INITIAL_DATA,
                FILE_SEEK_CURRENT,
                LOAD_GLOBAL_DATA_A,
                QUEUED_SUB_BUFFERS_FLUSH,
                LOAD_GLOBAL_DATA_B,
                LOAD_GLOBAL_BLOCK,
                MANAGER_BUILD_SAVE_NAME,
                BSSTRING_SET,
                HISTORY_ADD_NOTE,
                CALENDAR_MARK_STALE,
            ]
        ));

        // Start: the old history is replaced, the plugin list and the header
        // are read.
        assert_eq!(
            calls(e, LOAD_DATA_FROM_FILE),
            vec![vec![file, this.addr() + 0x248, 1]]
        );
        assert_eq!(calls(e, FILE_READ)[0][0], file);
        assert_eq!(calls(e, FILE_READ)[0][2], 0x6e);
        assert_eq!(
            calls(e, FILE_SEEK_SET),
            vec![vec![file, 500], vec![file, 100], vec![file, 200]]
        );
        assert_eq!(
            calls(e, FORM_ID_MAP_LOAD)[1],
            vec![0x4000_0200, file],
            "the worldspace map is loaded in place"
        );
        assert_eq!(calls(e, GLOBAL_DATA_RESET), vec![vec![0]]);
        assert_eq!(
            calls(e, LOAD_GLOBAL_BLOCK),
            vec![vec![file, 0x11], vec![file, 0x22]]
        );
        assert_eq!(
            calls(e, ARRAY_SET_RESERVED_SIZE),
            vec![vec![this.addr() + 0x24, 1]]
        );
        assert_eq!(calls(e, ARRAY_CLEAR)[0], vec![this.addr() + 0x24, 0]);

        // First pass: the buffer is matched with the live form.
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_LOAD_HEADER)[0],
            vec![buffer, file]
        );
        assert_eq!(
            calls(e, CHANGES_MAP_GET_FLAGS)
                .iter()
                .map(|w| (w[0], w[2]))
                .collect::<Vec<_>>(),
            vec![(s.old_map, 0x300), (new_map, 0x300)]
        );
        assert_eq!(
            calls(e, CHANGES_MAP_SET_FLAGS),
            vec![vec![new_map, 0x300, 0]]
        );
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![buffer, 5]]
        );
        assert_eq!(
            calls(e, GAME_CHECK_INITIAL_DATA),
            vec![vec![this.addr(), file, buffer, s.old_form_id_map]]
        );
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SKIP_DATA)[0],
            vec![buffer, file, 0x10]
        );
        assert_eq!(calls(e, FAKE_TARGET + 0x70), vec![vec![form, buffer]]);
        assert_eq!(calls(e, GAME_HANDLE_UNREVERTIBLE_CHANGES)[0][2..], [5, 0]);
        assert_eq!(
            calls(e, RECONSTRUCT_ADD_FORM),
            vec![vec![0x4000_0400, form, 0]]
        );
        assert_eq!(calls(e, LOAD_FORM_BUFFER_SET_LOADED), vec![vec![buffer, 1]]);
        assert_eq!(calls(e, CHANGES_MAP_REMOVE)[0], vec![s.old_map, 0x300]);

        // The changes are applied with the new map and the old id map goes.
        assert_eq!(e.get(this, BGSSaveLoadGame::pChangesMap).addr(), new_map);
        assert_eq!(e.get(this, BGSSaveLoadGame::pOldChangesMap), Ptr::NULL);
        assert!(calls(e, FORM_ID_MAP_DESTRUCT).contains(&vec![s.old_form_id_map]));
        assert_ne!(
            e.get(this, BGSSaveLoadGame::pFormIDMap).addr(),
            s.old_form_id_map
        );

        // Second pass: the data is loaded and given to the form.
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_LOAD_DATA),
            vec![vec![buffer, file]]
        );
        assert_eq!(calls(e, LOAD_INITIAL_DATA), vec![vec![buffer, form, 0]]);
        assert_eq!(calls(e, FORM_MARK_LOADED), vec![vec![form, 1]]);
        assert_eq!(calls(e, FAKE_TARGET + 0x5c), vec![vec![form, buffer]]);

        // End: the trailing block, the unload pass and the closing notes.
        assert_eq!(calls(e, FILE_SEEK_CURRENT), vec![vec![file, 0x33]]);
        assert_eq!(calls(e, LOAD_GLOBAL_DATA_A), vec![vec![0, 0xc]]);
        assert_eq!(calls(e, LOAD_GLOBAL_DATA_B), vec![vec![0, 0xc]]);
        assert_eq!(calls(e, QUEUED_SUB_BUFFERS_FLUSH), vec![vec![0x4000_0500]]);
        assert_eq!(calls(e, CHANGED_FORM_ID_MAP_FLUSH), vec![vec![0x4000_0600]]);
        let save_name = calls(e, MANAGER_BUILD_SAVE_NAME)[0][0];
        assert_eq!(
            calls(e, BSSTRING_SET),
            vec![vec![0x5000_0030, save_name, 0]]
        );
        let notes = calls(e, HISTORY_ADD_NOTE);
        assert_eq!(notes[0][1], LOADING_NOTE);
        assert_eq!(notes[0][0], new_history_of_first_note(&notes));
        assert_eq!(notes[1][1], FINISHED_LOADING_NOTE);
        assert_eq!(notes[1][0], new_history);
        assert_eq!(calls(e, CALENDAR_MARK_STALE), vec![vec![CALENDAR]]);
        assert_eq!(e.get(this, BGSSaveLoadGame::cCurrentMinorVersion), 0xff);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags) & 0xb, 0);
        assert_eq!(
            calls(e, GAME_SET_FLAG_ONE),
            vec![vec![this.addr(), 0], vec![this.addr(), 1]]
        );
        let _ = (s.old_history, s.player);
    }

    /// The history the first note went to (the load's own new history).
    fn new_history_of_first_note(notes: &[Vec<u32>]) -> u32 {
        notes[0][0]
    }

    #[test]
    fn load_game_stops_when_a_plugin_is_missing() {
        let mut s = load_setup();
        two_compiled_files(&mut s.e);
        saved_plugins(&mut s.e, &["Nope.esp"]);
        s.e.set_global(MENU_OBJECT, 0x6666_0000u32);
        s.e.set(s.this, BGSSaveLoadGame::cCurrentMinorVersion, 0x1b);
        assert!(!bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        let e = &s.e;
        assert_eq!(
            e.get(s.this, BGSSaveLoadGame::pHistory).addr(),
            s.old_history
        );
        assert_eq!(e.get(s.this, BGSSaveLoadGame::cCurrentMinorVersion), 0xff);
        assert!(calls(e, FILE_READ).is_empty());
        assert!(calls(e, REFERENCES_MAP_RESET).is_empty());
        // The history made for the failed load is deleted.
        assert_eq!(calls(e, HISTORY_DESTRUCT).len(), 1);
        assert_ne!(calls(e, HISTORY_DESTRUCT)[0][0], s.old_history);
        assert_eq!(
            calls(e, MENU_OBJECT_FORWARD),
            vec![vec![0x6666_0000, 0x2000_0000, 1]]
        );
        assert_eq!(e.get(s.this, BGSSaveLoadGame::iGlobalFlags) & 2, 0);
    }

    #[test]
    fn load_game_goes_on_when_a_plugin_is_missing_but_quiet() {
        let mut s = load_setup();
        two_compiled_files(&mut s.e);
        saved_plugins(&mut s.e, &["Nope.esp"]);
        assert!(bgssaveloadgame_load_game(&mut s.e, s.this, s.file, true));
        assert_eq!(calls(&s.e, FILE_READ).len(), 1);
        assert_eq!(calls(&s.e, SAVE_GAME_WARNING).len(), 1);
        // The history the load started with is deleted.
        assert!(calls(&s.e, HISTORY_DESTRUCT).contains(&vec![s.old_history]));
    }

    #[test]
    fn load_game_fails_when_the_form_data_cannot_be_loaded() {
        let mut s = load_setup();
        s.e.register(LOAD_FORM_BUFFER_LOAD_DATA, |_, _| rv(-1i32 as u32));
        assert!(!bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        let e = &s.e;
        let buffer = s.added.borrow()[0];
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SKIP_DATA)[1],
            vec![buffer, LOAD_FILE, 0]
        );
        assert_eq!(calls(e, LOAD_FORM_BUFFER_DELETE), vec![vec![buffer, 1]]);
        let slot = calls(e, ARRAY_SET_AT)[0].clone();
        assert_eq!(slot[..2], [s.this.addr() + 0x24, 0]);
        assert_eq!(e.mem.u32(slot[2]), 0);
        assert_eq!(calls(e, ARRAY_CLEAR)[1], vec![s.this.addr() + 0x24, 1]);
        assert_eq!(calls(e, FAKE_TARGET + 0x228), vec![vec![s.player, 0]]);
        assert_eq!(
            calls(e, GAME_SET_FLAG_ONE).last().unwrap(),
            &vec![s.this.addr(), 1]
        );
        assert_eq!(e.get(s.this, BGSSaveLoadGame::iGlobalFlags) & 2, 0);
        assert!(calls(e, LOAD_INITIAL_DATA).is_empty());
        assert!(calls(e, CALENDAR_MARK_STALE).is_empty());
    }

    #[test]
    fn load_game_fails_when_the_quiet_flag_is_set_during_the_second_pass() {
        let mut s = load_setup();
        s.e.register(LOAD_FORM_BUFFER_LOAD_DATA, |_, _| rv(0));
        s.e.set(s.this, BGSSaveLoadGame::iGlobalFlags, 0x80);
        assert!(!bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        assert!(calls(&s.e, LOAD_INITIAL_DATA).is_empty());
    }

    #[test]
    fn load_game_keeps_a_form_without_a_live_object_as_an_unloaded_buffer() {
        let mut s = load_setup();
        s.e.register(LOOKUP_FORM, |_, _| Ret::default());
        s.e.register(FORM_BUFFER_HEADER_FLAGS, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            rv(a[1])
        });
        let unloaded = Rc::new(Cell::new(0u32));
        let remember = unloaded.clone();
        s.e.register_double(UNLOADED_FORM_BUFFER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], 0x8888_0000);
            remember.set(a[0]);
            rv(a[0])
        });
        assert!(bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        let e = &s.e;
        let buffer = s.added.borrow()[0];
        // First pass: no form, so the entry is removed.
        assert_eq!(calls(e, CHANGES_MAP_REMOVE)[0], vec![s.old_map, 0x300]);
        assert!(calls(e, GAME_HANDLE_UNREVERTIBLE_CHANGES).is_empty());
        // Second pass: the stored buffer is loaded and kept in the map.
        assert_eq!(
            calls(e, UNLOADED_FORM_BUFFER_LOAD),
            vec![vec![unloaded.get(), LOAD_FILE]]
        );
        let new_map = e.get(s.this, BGSSaveLoadGame::pChangesMap).addr();
        assert_eq!(
            calls(e, CHANGES_MAP_STORE_BUFFER),
            vec![vec![new_map, 0x300, 0x77, 0x8888_0000]]
        );
        assert!(calls(e, LOAD_FORM_BUFFER_LOAD_DATA).is_empty());
        // The buffer itself is emptied, not deleted.
        assert_eq!(
            calls(e, OPERATOR_DELETE)
                .iter()
                .filter(|w| w[0] == 0)
                .count(),
            1
        );
        let _ = buffer;
    }

    #[test]
    fn load_game_clears_a_dynamic_form_whose_type_changed() {
        let mut s = load_setup();
        s.e.register(BUFFER_FORM_ID, |_, _| rv(0xff00_0001));
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x2b));
        assert!(bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        let e = &s.e;
        let buffer = s.added.borrow()[0];
        assert_eq!(calls(e, GAME_CLEAR_FORM), vec![vec![s.this.addr(), s.form]]);
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, s.form], vec![buffer, 0]]
        );
        // The old flags are reset to an empty set of flags.
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![buffer, 5], vec![buffer, 0]]
        );
        assert_eq!(
            calls(e, CHANGES_MAP_REMOVE)[0],
            vec![s.old_map, 0xff00_0001]
        );
        assert!(calls(e, SAVE_GAME_WARNING).is_empty());
        assert!(calls(e, GAME_HANDLE_UNREVERTIBLE_CHANGES).is_empty());
    }

    #[test]
    fn load_game_skips_a_form_whose_type_changed_and_drops_its_buffer() {
        let mut s = load_setup();
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x2b));
        assert!(bgssaveloadgame_load_game(&mut s.e, s.this, s.file, false));
        let e = &s.e;
        let buffer = s.added.borrow()[0];
        // The warning names the form, its id and both types.
        let warning = calls(e, SAVE_GAME_WARNING);
        assert_eq!(
            warning[0],
            vec![FORM_TYPE_MISMATCH_WARNING, 0xcc, 0x300, 0xbb, 0xaa]
        );
        assert_eq!(calls(e, TYPE_NAME_FOR), vec![vec![0x2a]]);
        // Second pass: header, skip, delete, clear the slot.
        assert_eq!(calls(e, LOAD_FORM_BUFFER_LOAD_HEADER).len(), 2);
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_SKIP_DATA)[1],
            vec![buffer, LOAD_FILE, 0]
        );
        assert_eq!(calls(e, LOAD_FORM_BUFFER_DELETE), vec![vec![buffer, 1]]);
        assert_eq!(calls(e, ARRAY_SET_AT).len(), 1);
        assert!(calls(e, LOAD_FORM_BUFFER_LOAD_DATA).is_empty());
        assert!(calls(e, LOAD_INITIAL_DATA).is_empty());
        assert_eq!(calls(e, CHANGES_MAP_REMOVE).len(), 1);
    }

    // ---- 00848e70 -------------------------------------------------------------

    /// Registers a walk over a changes map that yields `entries` (key, value
    /// pointer) in order.
    fn script_map_walk(e: &mut Engine, entries: Vec<(u32, u32)>) {
        e.register(MAP_FIRST_POSITION, |_, _| rv(1));
        let index = Rc::new(Cell::new(0usize));
        e.register_double(MAP_NEXT, move |e, a| {
            let i = index.get();
            e.mem.set_u32(a[2], entries[i].0);
            e.mem.set_u32(a[3], entries[i].1);
            index.set(i + 1);
            e.mem.set_u32(
                a[1],
                if i + 1 < entries.len() {
                    i as u32 + 2
                } else {
                    0
                },
            );
            Ret::default()
        });
    }

    /// A game object ready for `00848e70`: a current changes map with a
    /// vtable, a reconstruct map, the data handler, the world.
    fn apply_setup() -> (Engine, Ptr<BGSSaveLoadGame>, u32) {
        let mut e = engine();
        let this = game(&mut e);
        let current = object(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(current));
        e.set(
            this,
            BGSSaveLoadGame::pReconstructForms,
            Ptr::new(0x4000_0400),
        );
        e.set(this, BGSSaveLoadGame::pFormIDMap, Ptr::new(0x4000_0100));
        e.set_global(DATA_HANDLER, 0x1000_0000u32);
        e.set_global(WORLD, 0x1100_0000u32);
        (e, this, current)
    }

    /// A map entry: change flags then a stored buffer word.
    fn entry_value(e: &mut Engine, flags: u32, stored_buffer: u32) -> u32 {
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, flags);
        e.mem.set_u32(value + 4, stored_buffer);
        value
    }

    #[test]
    fn apply_changes_clears_a_dynamic_form_and_installs_the_map() {
        let (mut e, this, current) = apply_setup();
        let form = object(&mut e);
        let value = entry_value(&mut e, 4, 0);
        script_map_walk(&mut e, vec![(0xff00_0005, value)]);
        e.register_double(LOOKUP_FORM, move |_, _| rv(form));
        e.register(IS_DYNAMIC_FORM_ID, |_, a| rv((a[1] >= 0xff00_0000) as u32));
        let map = 0x4000_0900;
        fn_00848e70(&mut e, this, Ptr::new(map), Ptr::new(0));
        assert_eq!(calls(&e, GAME_CLEAR_FORM), vec![vec![this.addr(), form]]);
        assert_eq!(calls(&e, CHANGES_MAP_REMOVE), vec![vec![map, 0xff00_0005]]);
        assert!(calls(&e, LOAD_FORM_BUFFER_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, RECONSTRUCT_FINISH_A), vec![vec![0x4000_0400]]);
        assert_eq!(calls(&e, RECONSTRUCT_FINISH_B), vec![vec![0x4000_0400]]);
        // The previous current map is deleted through its destructor.
        assert_eq!(calls(&e, FAKE_TARGET), vec![vec![current, 1]]);
        assert_eq!(e.get(this, BGSSaveLoadGame::pChangesMap).addr(), map);
    }

    #[test]
    fn apply_changes_captures_a_live_form_and_reconstructs_it() {
        let (mut e, this, _) = apply_setup();
        let form = object(&mut e);
        let value = entry_value(&mut e, 4, 0);
        script_map_walk(&mut e, vec![(0x300, value)]);
        e.register_double(LOOKUP_FORM, move |_, _| rv(form));
        e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        e.register(COMBINE_CHANGE_FLAGS, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(ENTRY_TEST, |_, _| rv(1));
        let map = 0x4000_0900;
        fn_00848e70(&mut e, this, Ptr::new(map), Ptr::new(0));
        let buffer = calls(&e, LOAD_FORM_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, form]]
        );
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![buffer, 4]]
        );
        assert_eq!(e.mem.u32(buffer + 0x28) & 4, 4, "bit 2 of the buffer flags");
        assert_eq!(calls(&e, FAKE_TARGET + 0x70), vec![vec![form, buffer]]);
        let combine = &calls(&e, COMBINE_CHANGE_FLAGS)[0];
        assert_eq!(combine[1..], [4, 0, 0x2a, 0]);
        let handled = &calls(&e, GAME_HANDLE_UNREVERTIBLE_CHANGES)[0];
        assert_eq!((handled[0], handled[2], handled[3]), (this.addr(), 4, 1));
        assert_eq!(
            calls(&e, RECONSTRUCT_ADD_FORM),
            vec![vec![0x4000_0400, form, 8]]
        );
        assert_eq!(calls(&e, ENTRY_TEST), vec![vec![value, form]]);
        assert_eq!(calls(&e, CHANGES_MAP_ADD_FLAGS), vec![vec![map, 0x300, 4]]);
        assert_eq!(calls(&e, LOAD_FORM_BUFFER_DESTRUCT), vec![vec![buffer]]);
    }

    #[test]
    fn apply_changes_does_not_reconstruct_a_form_that_was_handled() {
        let (mut e, this, _) = apply_setup();
        let form = object(&mut e);
        let value = entry_value(&mut e, 4, 0);
        script_map_walk(&mut e, vec![(0x300, value)]);
        e.register_double(LOOKUP_FORM, move |_, _| rv(form));
        e.register(COMBINE_CHANGE_FLAGS, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0));
        // No flags remain: nothing to hand to the unrevertible-changes step.
        assert!(calls(&e, GAME_HANDLE_UNREVERTIBLE_CHANGES).is_empty());
        assert!(calls(&e, RECONSTRUCT_ADD_FORM).is_empty());
        // The entry test says no: the flags are not added.
        assert!(calls(&e, CHANGES_MAP_ADD_FLAGS).is_empty());
    }

    #[test]
    fn apply_changes_adds_the_reference_of_a_missing_form_to_the_reconstruct_map() {
        let (mut e, this, _) = apply_setup();
        let value = entry_value(&mut e, 9, 0x5151);
        script_map_walk(&mut e, vec![(0x300, value)]);
        e.register(STORED_BUFFER_TYPE, |_, a| rv(a[0] + 1));
        e.register(INITIAL_DATA_KIND, |_, _| rv(6));
        e.register(UNLOADED_FORM_BUFFER_ADVANCED, |_, _| rv(0xad));
        e.register(GET_ORIGINAL_LOCATION, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        let seen_id_map = Rc::new(Cell::new(0u32));
        let seen = seen_id_map.clone();
        let game_address = this.addr();
        e.register_double(CHANGES_MAP_GET_ENTRY, move |e, _| {
            seen.set(e.mem.u32(game_address + 8));
            Ret::default()
        });
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0x4000_0777));
        assert_eq!(calls(&e, STORED_BUFFER_TYPE), vec![vec![0x5151]]);
        assert_eq!(calls(&e, INITIAL_DATA_KIND), vec![vec![0x300, 0x5152, 9]]);
        // The form id map is swapped for the one given, and put back.
        assert_eq!(seen_id_map.get(), 0x4000_0777);
        assert_eq!(e.get(this, BGSSaveLoadGame::pFormIDMap).addr(), 0x4000_0100);
        let initial_data = calls(&e, LOAD_INITIAL_DATA_STRUCT)[0].clone();
        assert_eq!((initial_data[0], initial_data[1]), (0xad, 6));
        assert_eq!(
            calls(&e, WORLD_IS_CELL_LOADED),
            vec![vec![0x1100_0000, 0xce11, 0]]
        );
        assert_eq!(
            calls(&e, RECONSTRUCT_ADD_REFERENCE),
            vec![vec![0x4000_0400, 0xce11, 0x300, 8]]
        );
        // The two members of the initial data structure were cast.
        assert_eq!(calls(&e, THIS_IDENTITY).len(), 2);
    }

    #[test]
    fn apply_changes_ignores_a_missing_form_without_an_id_map_or_of_another_kind() {
        let (mut e, this, _) = apply_setup();
        let value = entry_value(&mut e, 9, 0x5151);
        script_map_walk(&mut e, vec![(0x300, value)]);
        e.register(INITIAL_DATA_KIND, |_, _| rv(6));
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0));
        assert!(calls(&e, INITIAL_DATA_KIND).is_empty());

        let (mut e, this, _) = apply_setup();
        let value = entry_value(&mut e, 9, 0x5151);
        script_map_walk(&mut e, vec![(0x300, value), (0x301, value)]);
        e.register(INITIAL_DATA_KIND, |_, _| rv(5));
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0x4000_0777));
        assert_eq!(calls(&e, INITIAL_DATA_KIND).len(), 2);
        assert!(calls(&e, CHANGES_MAP_GET_ENTRY).is_empty());
        assert!(calls(&e, RECONSTRUCT_ADD_REFERENCE).is_empty());

        // A stored buffer of 0 is not looked at at all.
        let (mut e, this, _) = apply_setup();
        let value = entry_value(&mut e, 9, 0);
        script_map_walk(&mut e, vec![(0x300, value)]);
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0x4000_0777));
        assert!(calls(&e, INITIAL_DATA_KIND).is_empty());
    }

    #[test]
    fn apply_changes_does_not_add_a_reference_when_the_cell_is_not_loaded() {
        let (mut e, this, _) = apply_setup();
        let value = entry_value(&mut e, 9, 0x5151);
        script_map_walk(&mut e, vec![(0x300, value)]);
        e.register(INITIAL_DATA_KIND, |_, _| rv(6));
        e.register(GET_ORIGINAL_LOCATION, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        fn_00848e70(&mut e, this, Ptr::new(0x4000_0900), Ptr::new(0x4000_0777));
        assert_eq!(calls(&e, WORLD_IS_CELL_LOADED).len(), 1);
        assert!(calls(&e, RECONSTRUCT_ADD_REFERENCE).is_empty());
    }

    #[test]
    fn apply_changes_makes_a_map_when_none_is_given() {
        let (mut e, this, _) = apply_setup();
        fn_00848e70(&mut e, this, Ptr::NULL, Ptr::NULL);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x10]]);
        let made = calls(&e, CHANGES_MAP_CONSTRUCT)[0][0];
        assert_eq!(e.get(this, BGSSaveLoadGame::pChangesMap).addr(), made);
    }

    // ---- 008492b0 -------------------------------------------------------------

    struct UnloadPass {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        io: u32,
        player: u32,
        buffers: [u32; 3],
        forms: [u32; 2],
        reference: u32,
        flags_seen: Rc<Cell<u32>>,
    }

    fn unload_pass_setup() -> UnloadPass {
        let mut e = engine();
        let this = game(&mut e);
        e.set(
            this,
            BGSSaveLoadGame::pQueuedSubBuffersMap,
            Ptr::new(0x4000_0500),
        );
        e.set(
            this,
            BGSSaveLoadGame::pChangedFormIDMap,
            Ptr::new(0x4000_0600),
        );
        let io = e.mem.alloc(0x70);
        e.set_global(IO_MANAGER, io);
        e.set_global(WORLD, 0x1100_0000u32);
        e.set_global(PLAYER, 0x1300_0000u32);
        let forms = [object(&mut e), object(&mut e)];
        let reference = object(&mut e);
        // Buffer 0 has a form and bit 4 (0x10); buffer 1 has a form; slot 2
        // of the array is empty.
        let buffer0 = object(&mut e);
        let buffer1 = object(&mut e);
        e.mem.set_u32(buffer0 + 0x24, forms[0]);
        e.mem.set_u32(buffer0 + 0x28, 0x10);
        e.mem.set_u32(buffer1 + 0x24, forms[1]);
        let cells = e.mem.alloc(12);
        for (i, buffer) in [buffer0, buffer1, 0].into_iter().enumerate() {
            e.mem.set_u32(cells + 4 * i as u32, buffer);
        }
        e.register(ARRAY_SIZE, |_, _| rv(3));
        e.register_double(ARRAY_ELEMENT_ADDRESS, move |_, a| rv(cells + 4 * a[1]));
        e.register(FAKE_TARGET + 4, |e, a| rv(e.mem.u32(a[0] + 0x24)));
        e.register_double(FAKE_TARGET + 8, move |_, _| rv(reference));
        e.register(FAKE_TARGET + 0x100, |_, _| rv(1));
        e.register(GET_SAVED_ACQUIRE_OBJECT, |_, _| rv(0x99));
        e.register(REFERENCE_PARENT_CELL, |_, _| rv(0xce11));
        e.register(CELL_PHYSICS_WORLD, |_, _| rv(0xaa));
        let flags_seen = Rc::new(Cell::new(0u32));
        let seen = flags_seen.clone();
        let game_address = this.addr();
        e.register_double(WORLD_PREPARE, move |e, _| {
            seen.set(e.mem.u32(game_address + 0x244));
            Ret::default()
        });
        UnloadPass {
            e,
            this,
            io,
            player: 0x1300_0000,
            buffers: [buffer0, buffer1, 0],
            forms,
            reference,
            flags_seen,
        }
    }

    #[test]
    fn unload_pass_prepares_unloads_and_releases_every_buffer() {
        let mut p = unload_pass_setup();
        fn_008492b0(&mut p.e, p.this, true);
        let e = &p.e;
        assert_eq!(e.mem.u32(p.io + 0x68), 5);
        assert_eq!(p.flags_seen.get() & 8, 8, "the unload flag is held");
        assert_eq!(e.get(p.this, BGSSaveLoadGame::iGlobalFlags) & 8, 0);
        assert_eq!(
            calls(e, FAKE_TARGET + 0x64),
            vec![
                vec![p.forms[0], p.buffers[0]],
                vec![p.forms[1], p.buffers[1]]
            ]
        );
        assert_eq!(calls(e, WORLD_PREPARE), vec![vec![0x1100_0000]]);
        assert_eq!(calls(e, IO_MANAGER_RESUME), vec![vec![p.io]]);
        assert_eq!(calls(e, IO_MANAGER_LOAD_QUEUED_PRIORITY), vec![vec![p.io]]);
        assert_eq!(calls(e, CELL_PHYSICS_WORLD), vec![vec![0xce11]]);
        assert_eq!(
            calls(e, PHYSICS_WORLD_DEACTIVATE_ALL_ISLANDS),
            vec![vec![0xaa]]
        );
        assert_eq!(calls(e, WORLD_FINISH), vec![vec![0x1100_0000]]);
        assert_eq!(calls(e, QUEUED_SUB_BUFFERS_FLUSH), vec![vec![0x4000_0500]]);

        // Second loop: forms get slot 0x84; buffer 0's reference is unloaded.
        assert_eq!(
            calls(e, FAKE_TARGET + 0x84),
            vec![
                vec![p.forms[0], p.buffers[0]],
                vec![p.forms[1], p.buffers[1]]
            ]
        );
        assert_eq!(
            calls(e, GAME_ADD_CHANGE),
            vec![vec![p.this.addr(), p.reference, 8, 1]]
        );
        assert_eq!(calls(e, FAKE_TARGET + 0x244), vec![vec![p.reference]]);
        assert_eq!(calls(e, GUARD_CONSTRUCT).len(), 1, "UnloadForm ran");
        assert_eq!(
            calls(e, CELL_REMOVE_REFERENCE),
            vec![vec![0xce11, p.reference]]
        );
        assert_eq!(calls(e, GARBAGE_COLLECTOR_ADD), vec![vec![p.reference]]);
        assert_eq!(
            calls(e, FORM_MARK_LOADED),
            vec![vec![p.forms[0], 0], vec![p.forms[1], 0]]
        );
        assert_eq!(
            calls(e, LOAD_FORM_BUFFER_DELETE),
            vec![vec![p.buffers[0], 1], vec![p.buffers[1], 1]]
        );
        assert_eq!(calls(e, ARRAY_CLEAR), vec![vec![p.this.addr() + 0x24, 1]]);
        assert_eq!(calls(e, CHANGED_FORM_ID_MAP_FLUSH), vec![vec![0x4000_0600]]);
        let _ = p.player;
    }

    #[test]
    fn unload_pass_without_the_full_flag_skips_the_world_steps() {
        let mut p = unload_pass_setup();
        fn_008492b0(&mut p.e, p.this, false);
        let e = &p.e;
        assert_eq!(e.mem.u32(p.io + 0x68), 0);
        for step in [
            WORLD_PREPARE,
            IO_MANAGER_RESUME,
            IO_MANAGER_LOAD_QUEUED_PRIORITY,
            WORLD_FINISH,
            PHYSICS_WORLD_DEACTIVATE_ALL_ISLANDS,
        ] {
            assert!(calls(e, step).is_empty(), "{step:08x}");
        }
        assert_eq!(calls(e, QUEUED_SUB_BUFFERS_FLUSH).len(), 1);
        assert_eq!(calls(e, LOAD_FORM_BUFFER_DELETE).len(), 2);
    }

    #[test]
    fn unload_pass_leaves_a_reference_alone_when_it_has_no_acquired_object() {
        let mut p = unload_pass_setup();
        p.e.register(GET_SAVED_ACQUIRE_OBJECT, |_, _| Ret::default());
        p.e.register(CELL_PHYSICS_WORLD, |_, _| Ret::default());
        fn_008492b0(&mut p.e, p.this, true);
        assert!(calls(&p.e, FAKE_TARGET + 0x244).is_empty());
        assert!(calls(&p.e, PHYSICS_WORLD_DEACTIVATE_ALL_ISLANDS).is_empty());
        assert_eq!(calls(&p.e, GARBAGE_COLLECTOR_ADD).len(), 1);
    }

    #[test]
    fn unload_pass_loads_the_global_words_at_their_calls() {
        // 008492b0: `TES` (011dea10), the I/O manager (01202d98) and the
        // player (011dea3c) are loaded at each call, so what the forms'
        // `InitLoadGame` (slot 0x64) leaves there is what the world steps get.
        let mut p = unload_pass_setup();
        let io2 = p.e.mem.alloc(0x70);
        p.e.register_double(FAKE_TARGET + 0x64, move |e, _| {
            e.set_global(IO_MANAGER, io2);
            e.set_global(WORLD, 0x1200_0000u32);
            e.set_global(PLAYER, 0x1400_0000u32);
            Ret::default()
        });
        fn_008492b0(&mut p.e, p.this, true);
        let e = &p.e;
        assert_eq!(e.mem.u32(p.io + 0x68), 5, "008492cb: before the loop");
        assert_eq!(e.mem.u32(io2 + 0x68), 0);
        assert_eq!(calls(e, WORLD_PREPARE), vec![vec![0x1200_0000]]);
        assert_eq!(calls(e, IO_MANAGER_RESUME), vec![vec![io2]]);
        assert_eq!(calls(e, IO_MANAGER_LOAD_QUEUED_PRIORITY), vec![vec![io2]]);
        assert_eq!(calls(e, REFERENCE_PARENT_CELL)[0], vec![0x1400_0000]);
        assert_eq!(calls(e, WORLD_FINISH), vec![vec![0x1200_0000]]);
    }

    // ---- LoadForm -----------------------------------------------------------------

    /// A game object with a changes map, one entry (`flags` word 0x1234, a
    /// stored buffer) and a form of id `0x300`.
    fn form_entry_setup() -> (Engine, Ptr<BGSSaveLoadGame>, u32, u32) {
        let mut e = engine();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        let form = object(&mut e);
        let entry = entry_value(&mut e, 0x1234, 0x5151);
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        e.register_double(CHANGES_MAP_ENTRY_FOR, move |_, _| rv(entry));
        e.register(STORED_BUFFER_TYPE, |_, _| rv(0x2a));
        e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        (e, this, form, entry)
    }

    #[test]
    fn load_form_does_nothing_during_a_load() {
        let (mut e, this, form, _) = form_entry_setup();
        e.register(GAME_LOADING_FLAG_TEST, |_, _| rv(1));
        bgssaveloadgame_load_form(&mut e, this, Ptr::new(form));
        assert_eq!(order(&e), vec![GAME_LOADING_FLAG_TEST]);
    }

    #[test]
    fn load_form_needs_an_entry_with_a_stored_buffer() {
        let (mut e, this, form, _) = form_entry_setup();
        e.register(CHANGES_MAP_ENTRY_FOR, |_, _| Ret::default());
        bgssaveloadgame_load_form(&mut e, this, Ptr::new(form));
        assert_eq!(
            calls(&e, CHANGES_MAP_ENTRY_FOR),
            vec![vec![0x4000_0000, 0x300]]
        );
        assert!(calls(&e, SET_THREAD_FLAG).is_empty());

        let (mut e, this, form, entry) = form_entry_setup();
        e.mem.set_u32(entry + 4, 0);
        bgssaveloadgame_load_form(&mut e, this, Ptr::new(form));
        assert!(calls(&e, SET_THREAD_FLAG).is_empty());
        assert!(calls(&e, STORED_BUFFER_TYPE).is_empty());
    }

    #[test]
    fn load_form_warns_and_forgets_the_entry_when_the_type_changed() {
        let (mut e, this, form, _) = form_entry_setup();
        e.register(STORED_BUFFER_TYPE, |_, _| rv(0x2b));
        e.register(FORM_TYPE_NAME, |_, _| rv(0xaa));
        e.register(TYPE_NAME_FOR, |_, _| rv(0xbb));
        e.register(FAKE_TARGET + 0x130, |_, _| rv(0xcc));
        bgssaveloadgame_load_form(&mut e, this, Ptr::new(form));
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![
                LOAD_ERROR_TYPE_MISMATCH_WARNING,
                0xcc,
                0x300,
                0xbb,
                0xaa
            ]]
        );
        assert_eq!(calls(&e, TYPE_NAME_FOR), vec![vec![0x2b]]);
        assert_eq!(
            calls(&e, CHANGES_MAP_REMOVE),
            vec![vec![0x4000_0000, 0x300]]
        );
        assert!(calls(&e, SET_THREAD_FLAG).is_empty());
    }

    #[test]
    fn load_form_loads_the_stored_buffer_into_the_form() {
        let (mut e, this, form, entry) = form_entry_setup();
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u32(buffer + 4, 0x2222_0000);
        e.register(SET_THREAD_FLAG, |_, a| rv(if a[1] == 0 { 7 } else { 0 }));
        e.register_double(UNLOADED_FORM_BUFFER_CREATE_LOAD_BUFFER, move |_, _| {
            rv(buffer)
        });
        let slot_seen = Rc::new(Cell::new(0u32));
        let seen = slot_seen.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            seen.set(e.mem.u32(a[1]));
            Ret::default()
        });
        bgssaveloadgame_load_form(&mut e, this, Ptr::new(form));
        let stored = entry + 4;
        assert_eq!(
            order(&e),
            vec![
                GAME_LOADING_FLAG_TEST,
                FORM_ID_WORD,
                CHANGES_MAP_ENTRY_FOR,
                READ_WORD,
                STORED_BUFFER_TYPE,
                FORM_GET_TYPE,
                SET_THREAD_FLAG,
                UNLOADED_FORM_BUFFER_CREATE_LOAD_BUFFER,
                UNLOADED_FORM_BUFFER_CLEAR,
                LOAD_FORM_BUFFER_SET_FORM,
                LOAD_FORM_BUFFER_SET_HEADER_FLAGS,
                FORM_MARK_LOADED,
                LOAD_INITIAL_DATA,
                FAKE_TARGET + 0x5c,
                OPERATOR_DELETE,
                ARRAY_ADD,
                SET_THREAD_FLAG,
            ]
        );
        assert_eq!(
            calls(&e, SET_THREAD_FLAG),
            vec![vec![this.addr(), 0], vec![this.addr(), 7]]
        );
        assert_eq!(
            calls(&e, UNLOADED_FORM_BUFFER_CREATE_LOAD_BUFFER),
            vec![vec![stored]]
        );
        assert_eq!(calls(&e, UNLOADED_FORM_BUFFER_CLEAR), vec![vec![stored]]);
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, form]]
        );
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_HEADER_FLAGS),
            vec![vec![buffer, 0x1234]]
        );
        assert_eq!(calls(&e, FORM_MARK_LOADED), vec![vec![form, 1]]);
        assert_eq!(
            calls(&e, LOAD_INITIAL_DATA),
            vec![vec![buffer, form, 0x1234]]
        );
        assert_eq!(calls(&e, FAKE_TARGET + 0x5c), vec![vec![form, buffer]]);
        // The buffer's data was freed and its fields cleared; it is queued.
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2222_0000]]);
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(slot_seen.get(), buffer);
        assert_eq!(calls(&e, ARRAY_ADD)[0][0], this.addr() + 0x24);
    }

    // ---- UnloadForm ---------------------------------------------------------------

    struct UnloadSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        form: u32,
        entry: u32,
    }

    /// A form with an entry (flags 7, no stored buffer) that can be unloaded.
    fn unload_setup() -> UnloadSetup {
        let (mut e, this, form, entry) = form_entry_setup();
        e.mem.set_u32(entry, 7);
        e.mem.set_u32(entry + 4, 0);
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0300));
        e.set_global(MANAGER_INSTANCE, 0x5000_0000u32);
        e.set_global(DATA_HANDLER, 0x1000_0000u32);
        e.register(GAME_CAN_UNLOAD_FORM, |_, _| rv(1));
        e.register(MANAGER_GET_MINOR_VERSION, |_, _| rv(0x1b));
        e.register(FAKE_TARGET + 0x130, |_, _| rv(0xcc));
        UnloadSetup {
            e,
            this,
            form,
            entry,
        }
    }

    /// The scope guard is made and unmade around every outcome.
    fn assert_guarded(e: &Engine) {
        let log = order(e);
        assert_eq!(log.iter().filter(|a| **a == GUARD_CONSTRUCT).count(), 1);
        assert_eq!(*log.first().unwrap(), GUARD_CONSTRUCT);
        assert_eq!(*log.last().unwrap(), GUARD_DESTRUCT);
        let guard = &calls(e, GUARD_CONSTRUCT)[0];
        assert_eq!(guard[1..], [0x11, 1, SOURCE_PATH, 0x37e]);
        assert_eq!(calls(e, GUARD_DESTRUCT), vec![vec![guard[0]]]);
    }

    #[test]
    fn unload_form_is_refused_during_a_load_unless_forced() {
        let mut s = unload_setup();
        s.e.register(GAME_LOADING_FLAG_TEST, |_, _| rv(1));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(order(&s.e).len(), 3, "guard, test, guard");

        let mut s = unload_setup();
        s.e.register(GAME_LOADING_FLAG_TEST, |_, _| rv(1));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), true);
        assert_eq!(calls(&s.e, GAME_CAN_UNLOAD_FORM).len(), 1);
    }

    #[test]
    fn unload_form_stops_when_the_data_handler_is_clearing_data_or_the_form_cannot_unload() {
        let mut s = unload_setup();
        s.e.register(DATA_HANDLER_GET_CLEARING_DATA, |_, _| rv(1));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert!(calls(&s.e, GAME_CAN_UNLOAD_FORM).is_empty());

        let mut s = unload_setup();
        s.e.register(GAME_CAN_UNLOAD_FORM, |_, _| Ret::default());
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(
            calls(&s.e, GAME_CAN_UNLOAD_FORM),
            vec![vec![s.this.addr(), s.form]]
        );
        assert!(calls(&s.e, FORM_ID_WORD).is_empty());
    }

    #[test]
    fn unload_form_needs_an_entry_without_a_buffer() {
        let mut s = unload_setup();
        s.e.register(CHANGES_MAP_ENTRY_FOR, |_, _| Ret::default());
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert!(calls(&s.e, SAVE_FORM_BUFFER_CONSTRUCT).is_empty());

        let mut s = unload_setup();
        s.e.mem.set_u32(s.entry + 4, 0x5151);
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(
            calls(&s.e, SAVE_GAME_WARNING),
            vec![vec![UNLOAD_HAS_BUFFER_WARNING, 0xcc, 0x300]]
        );
        assert!(calls(&s.e, SAVE_FORM_BUFFER_CONSTRUCT).is_empty());
    }

    #[test]
    fn unload_form_stops_when_the_changes_are_already_known() {
        let mut s = unload_setup();
        s.e.register(CHANGES_MAP_CLEAR_FLAGS, |_, _| rv(1));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        let buffer = calls(&s.e, SAVE_FORM_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&s.e, SAVE_FORM_BUFFER_SET_HEADER),
            vec![vec![buffer, 0x300, 7, 0x2a, 0x1b]]
        );
        assert_eq!(
            calls(&s.e, CHANGES_MAP_CLEAR_FLAGS),
            vec![vec![0x4000_0000, 0x300, 7]]
        );
        assert_eq!(calls(&s.e, SAVE_FORM_BUFFER_DESTRUCT), vec![vec![buffer]]);
        assert!(calls(&s.e, SAVE_INITIAL_DATA).is_empty());
        assert!(calls(&s.e, UNLOADED_FORM_BUFFER_RELEASE).is_empty());
    }

    #[test]
    fn unload_form_stores_the_buffer_in_the_entry() {
        let mut s = unload_setup();
        s.e.register(SAVE_INITIAL_DATA, |_, _| rv(0));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        let buffer = calls(&s.e, SAVE_FORM_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&s.e, SAVE_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, s.form]]
        );
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x80), vec![vec![s.form, buffer]]);
        assert_eq!(
            calls(&s.e, SAVE_INITIAL_DATA),
            vec![vec![buffer, s.form, 7]]
        );
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x54), vec![vec![s.form, buffer]]);
        assert_eq!(
            calls(&s.e, UNLOADED_FORM_BUFFER_RELEASE),
            vec![vec![s.entry + 4, buffer]]
        );
        assert_eq!(calls(&s.e, SAVE_FORM_BUFFER_DESTRUCT), vec![vec![buffer]]);
        // Kind 0: the references map is left alone.
        assert!(calls(&s.e, REFERENCES_MAP_SETUP).is_empty());
        assert!(calls(&s.e, REFERENCES_MAP_ADD_UNLOADED).is_empty());
        assert!(calls(&s.e, SAVE_GAME_WARNING).is_empty());
    }

    #[test]
    fn unload_form_records_a_reference_in_a_cell_with_flag_bit_zero() {
        let mut s = unload_setup();
        s.e.register(SAVE_INITIAL_DATA, |_, _| rv(6));
        s.e.register(REFERENCE_PARENT_CELL, |_, _| rv(0xce11));
        s.e.register(CELL_FLAG_ZERO_TEST, |_, _| rv(1));
        s.e.register(FORM_ID_WORD, |_, a| {
            rv(if a[0] == 0xce11 { 0xce00 } else { 0x300 })
        });
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(
            calls(&s.e, REFERENCES_MAP_SETUP),
            vec![vec![0x4000_0310, 0xce00, 0x300]]
        );
        // Kind 6 also maps the id to itself.
        assert_eq!(
            calls(&s.e, MAP_SET_AT),
            vec![vec![0x4000_0300, 0x300, 0x300]]
        );
        assert!(calls(&s.e, REFERENCES_MAP_ADD_UNLOADED).is_empty());
    }

    #[test]
    fn unload_form_records_a_reference_by_its_worldspace() {
        let mut s = unload_setup();
        s.e.register(SAVE_INITIAL_DATA, |_, _| rv(5));
        s.e.register(REFERENCE_WORLD_SPACE, |_, _| rv(0xa0a0));
        s.e.register(FAKE_TARGET + 0x1f4, |_, _| rv(0x777));
        s.e.register(FORM_ID_WORD, |_, a| {
            rv(if a[0] == 0xa0a0 { 0xa000 } else { 0x300 })
        });
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(
            calls(&s.e, REFERENCES_MAP_ADD_UNLOADED),
            vec![vec![0x4000_0300, 0xa000, 0x300, 0x777]]
        );
        // Kind 5 does not map the id to itself.
        assert!(calls(&s.e, MAP_SET_AT).is_empty());
        assert!(calls(&s.e, REFERENCES_MAP_SETUP).is_empty());
    }

    #[test]
    fn unload_form_uses_the_worldspace_of_an_exterior_cell_reference_too() {
        // A cell whose flag bit 0 is clear falls through to the
        // worldspace branch, and kind 6 maps the id.
        let mut s = unload_setup();
        s.e.register(SAVE_INITIAL_DATA, |_, _| rv(6));
        s.e.register(REFERENCE_PARENT_CELL, |_, _| rv(0xce11));
        s.e.register(REFERENCE_WORLD_SPACE, |_, _| rv(0xa0a0));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_eq!(calls(&s.e, CELL_FLAG_ZERO_TEST), vec![vec![0xce11]]);
        assert_eq!(calls(&s.e, REFERENCES_MAP_ADD_UNLOADED).len(), 1);
        assert_eq!(calls(&s.e, MAP_SET_AT).len(), 1);
    }

    #[test]
    fn unload_form_warns_when_a_reference_has_no_place() {
        let mut s = unload_setup();
        s.e.register(SAVE_INITIAL_DATA, |_, _| rv(6));
        bgssaveloadgame_unload_form(&mut s.e, s.this, Ptr::new(s.form), false);
        assert_guarded(&s.e);
        assert_eq!(
            calls(&s.e, SAVE_GAME_WARNING),
            vec![vec![UNLOAD_NO_PLACE_WARNING, 0xcc, 0x300]]
        );
        assert!(calls(&s.e, MAP_SET_AT).is_empty());
    }

    // ---- session 2: 00849a90 to 0084b650 ------------------------------------------

    /// Every external function the session 2 translations call.
    const EXTERNAL_SESSION_2: [u32; 47] = [
        FORM_FLAG_4000_TEST,
        FORM_FLAG_20_TEST,
        GAME_RECORDING_TEST,
        CHANGES_MAP_HAS_FLAGS,
        FLAGS_TEST,
        FLAGS_CLEAR,
        REFERENCE_MOVE_TO_LOCATION,
        REFERENCE_IS_PERSISTENT,
        FORM_NAME_LENGTH,
        REFERENCE_FULL_NAME,
        ACTOR_PROCESS,
        ACTOR_INIT_PACKAGE_LOCATIONS,
        DATA_HANDLER_GET_NEXT_ID,
        CELL_DETACH_TIME,
        CELL_WORLD_SPACE,
        CELL_DATA_X,
        CELL_DATA_Y,
        WORLD_SPACE_CELL_FROM_COORD,
        PACK_GRID_COORDS,
        ROUND_TO_INT,
        CALENDAR_TIME_STAMP,
        EXPIRY_LIMIT,
        LOAD_INITIAL_DATA_FROM_FILE,
        LOCATION_CELL_AND_WORLD,
        CREATE_REFERENCE,
        RECONSTRUCT_ADD_REFERENCE_AT,
        RECONSTRUCT_DESTRUCT,
        FORM_LIST_COUNT,
        FORM_LIST_ELEMENT,
        REFERENCES_MAP_LOAD_FOR_CELL,
        REFERENCES_MAP_REMOVE_EXTERIOR,
        REFERENCES_MAP_REMOVE,
        MAP_GET_VALUE,
        MAP_REMOVE_AT,
        QUEUED_SUB_BUFFERS_QUEUE,
        HISTORY_ADD_NOTE_VA,
        EXPIRY_QUEUE_INSERT,
        EXPIRY_QUEUE_FIND,
        EXPIRY_QUEUE_DESTRUCT,
        EXPIRY_QUEUE_BASE_CONSTRUCT,
        UNLOADED_FORM_BUFFER_SIZE,
        UNLOADED_FORM_BUFFER_INITIALIZE,
        INITIAL_DATA_TYPE_SIZE,
        LOAD_VARIABLE_SIZED_VALUE,
        ALLOCATE_BYTES,
        ARRAY_BASE_SETUP,
        FLAGS_EQUAL,
    ];

    /// `engine()` with the session 2 externals doubled, the bit tests and the
    /// map getters behaving, and the page of `ZERO_DOUBLE` mapped.
    fn engine2() -> Engine {
        let mut e = engine();
        for address in EXTERNAL_SESSION_2 {
            e.register(address, |_, _| Ret::default());
        }
        e.map(0x0101_2000, 0x1000);
        e.register(FLAGS_TEST, |e, a| rv((e.mem.u32(a[0]) & a[1] != 0) as u32));
        e.register(FLAGS_CLEAR, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], word & !a[1]);
            Ret::default()
        });
        e.register(FLAGS_EQUAL, |e, a| rv((e.mem.u32(a[0]) == a[1]) as u32));
        e.register(ALLOCATE_BYTES, |e, a| rv(e.mem.alloc(a[0])));
        e
    }

    // ---- 00849a90 ------------------------------------------------------------

    #[test]
    fn can_unload_form_follows_the_flag_and_the_two_reference_slots() {
        let mut e = engine2();
        let this = game(&mut e);
        e.mem.set_u32(TEST_VTABLE + 0x224, FAKE_TARGET + 0x224);
        e.mem.set_u32(TEST_VTABLE + 0x324, FAKE_TARGET + 0x324);
        e.register(FAKE_TARGET + 0x224, |_, _| rv(1));
        e.register(FAKE_TARGET + 0x324, |_, _| rv(0));
        let form = object(&mut e);
        // Flag 0x4000 set: refused without asking the form.
        e.register(FORM_FLAG_4000_TEST, |_, _| rv(1));
        assert!(!fn_00849a90(&mut e, this, Ptr::new(form)));
        assert!(calls(&e, FAKE_TARGET + 0xf0).is_empty());
        e.register(FORM_FLAG_4000_TEST, |_, _| rv(0));
        // Not a reference: may unload.
        assert!(fn_00849a90(&mut e, this, Ptr::new(form)));
        // A reference with slot 0x224 true and slot 0x324 false: kept.
        e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        assert!(!fn_00849a90(&mut e, this, Ptr::new(form)));
        // Slot 0x324 true: may unload.
        e.register(FAKE_TARGET + 0x324, |_, _| rv(1));
        assert!(fn_00849a90(&mut e, this, Ptr::new(form)));
        // Slot 0x224 false: may unload.
        e.register(FAKE_TARGET + 0x324, |_, _| rv(0));
        e.register(FAKE_TARGET + 0x224, |_, _| rv(0));
        assert!(fn_00849a90(&mut e, this, Ptr::new(form)));
    }

    // ---- 00849b10 and its helpers --------------------------------------------

    #[test]
    fn load_cell_does_nothing_during_a_load() {
        let mut e = engine2();
        let this = game(&mut e);
        e.register(GAME_LOADING_FLAG_TEST, |_, _| rv(1));
        bgssaveloadgame_load_cell(&mut e, this, Ptr::new(0xce11));
        assert_eq!(order(&e), vec![GAME_LOADING_FLAG_TEST]);
    }

    #[test]
    fn load_cell_loads_the_cell_then_each_reference_of_its_reconstruct_map() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0010));
        e.register(FORM_LIST_COUNT, |_, _| rv(2));
        e.register(FORM_LIST_ELEMENT, |_, a| rv(0x1100 + a[1]));
        let seen = Rc::new(Cell::new(0u32));
        let flag_seen = seen.clone();
        e.register_double(RECONSTRUCT_FINISH_A, move |e, _| {
            let word = e.tls() + THREAD_FLAGS_OFFSET;
            flag_seen.set(e.mem.u32(word));
            Ret::default()
        });
        bgssaveloadgame_load_cell(&mut e, this, Ptr::new(0xce11));
        // LoadForm of the cell and of the two references, each asking the
        // form id of its argument.
        let ids: Vec<u32> = calls(&e, FORM_ID_WORD).iter().map(|w| w[0]).collect();
        assert_eq!(ids, vec![0xce11, 0x1100, 0x1101]);
        let pending = calls(&e, RECONSTRUCT_FORMS_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, REFERENCES_MAP_LOAD_FOR_CELL),
            vec![vec![0x4000_0010, 0xce11, pending]]
        );
        assert_eq!(calls(&e, RECONSTRUCT_FINISH_A), vec![vec![pending]]);
        assert_eq!(calls(&e, FORM_LIST_COUNT), vec![vec![pending + 0x10]]);
        assert_eq!(
            calls(&e, FORM_LIST_ELEMENT),
            vec![vec![pending + 0x10, 0], vec![pending + 0x10, 1]]
        );
        assert_eq!(calls(&e, RECONSTRUCT_DESTRUCT), vec![vec![pending]]);
        // The thread flag was set around the closing step only.
        assert_eq!(seen.get() & 2, 2);
        let word = e.tls() + THREAD_FLAGS_OFFSET;
        assert_eq!(e.mem.u32(word) & 2, 0);
    }

    #[test]
    fn thread_flag_helper_sets_and_clears_mask_2_only() {
        let mut e = engine2();
        let word = e.tls() + THREAD_FLAGS_OFFSET;
        e.mem.set_u32(word, 0x11);
        fn_00849c00(&mut e, Ptr::new(0x5000_0000), true);
        assert_eq!(e.mem.u32(word), 0x13);
        fn_00849c00(&mut e, Ptr::new(0x5000_0000), false);
        assert_eq!(e.mem.u32(word), 0x11);
    }

    #[test]
    fn thread_flag_helper_takes_this_first_through_its_address() {
        // 00849c00 is a thiscall (RET 4): the uniform words are (this, on), so
        // `on` is the second word, not this's low byte.
        let mut e = engine2();
        let word = e.tls() + THREAD_FLAGS_OFFSET;
        e.mem.set_u32(word, 0x11);
        e.call(0x0084_9c00, &args![0x5000_0000u32, 1u32]);
        assert_eq!(e.mem.u32(word), 0x13);
        e.call(0x0084_9c00, &args![0x5000_0001u32, 0u32]);
        assert_eq!(e.mem.u32(word), 0x11);
    }

    #[test]
    fn reconstruct_accessors_use_the_list_at_0x10() {
        let mut e = engine2();
        e.register(FORM_LIST_COUNT, |_, _| rv(7));
        e.register(FORM_LIST_ELEMENT, |_, a| rv(a[1] * 3));
        assert_eq!(fn_00849c80(&mut e, Ptr::new(0x5000_0000)), 7);
        assert_eq!(fn_00849ca0(&mut e, Ptr::new(0x5000_0000), 4), 12);
        assert_eq!(calls(&e, FORM_LIST_COUNT), vec![vec![0x5000_0010]]);
        assert_eq!(calls(&e, FORM_LIST_ELEMENT), vec![vec![0x5000_0010, 4]]);
    }

    #[test]
    fn id_lookup_passes_the_key_and_its_own_slot() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0010));
        let slot_value = Rc::new(Cell::new(0u32));
        let seen = slot_value.clone();
        e.register_double(MAP_GET_VALUE, move |e, a| {
            seen.set(e.mem.u32(a[2]));
            rv(1)
        });
        assert!(fn_00849cc0(&mut e, this, 0x77));
        assert_eq!(calls(&e, MAP_GET_VALUE)[0][..2], [0x4000_0010, 0x77]);
        assert_eq!(slot_value.get(), 0x77);
        e.register(MAP_GET_VALUE, |_, _| rv(0));
        assert!(!fn_00849ce0(&mut e, Ptr::new(0x4000_0020), 5));
    }

    // ---- 00849d00 ------------------------------------------------------------

    struct CheckSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        buffer: u32,
        form: Rc<Cell<u32>>,
    }

    /// A check with the buffer header kind `kind`, form id 0x300, type 0x41,
    /// header flags 0x33 and old flags 0x44. The buffer's slot 4 returns the
    /// form cell, `INITIAL_DATA_KIND` the kind by the flags it is asked with
    /// (`0x33` header flags: `kind`; `0x44` old flags: `second_kind`).
    fn check_setup(kind: u32, second_kind: u32, form: u32) -> CheckSetup {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0010));
        e.set(
            this,
            BGSSaveLoadGame::pReconstructForms,
            Ptr::new(0x4000_0020),
        );
        e.set(this, BGSSaveLoadGame::pFormIDMap, Ptr::new(0x4000_0030));
        e.set_global(WORLD, 0x4000_0100u32);
        e.set_global(PLAYER, 0x4000_0200u32);
        e.set_global(DATA_HANDLER, 0x4000_0300u32);
        let buffer = object(&mut e);
        let form_cell = Rc::new(Cell::new(form));
        let slot_form = form_cell.clone();
        e.register_double(FAKE_TARGET + 4, move |_, _| rv(slot_form.get()));
        e.register(LOAD_FORM_BUFFER_HEADER_TYPE, |_, _| rv(0x41));
        e.register(BUFFER_FORM_ID, |_, _| rv(0x300));
        e.register(FORM_BUFFER_HEADER_FLAGS, |e, a| {
            e.mem.set_u32(a[1], 0x33);
            rv(a[1])
        });
        e.register(LOAD_FORM_BUFFER_OLD_FLAGS, |e, a| {
            e.mem.set_u32(a[1], 0x44);
            rv(a[1])
        });
        let kinds = (kind, second_kind);
        e.register_double(INITIAL_DATA_KIND, move |_, a| {
            rv(if a[2] == 0x33 { kinds.0 } else { kinds.1 })
        });
        e.register(LOAD_INITIAL_DATA_FROM_FILE, |_, _| rv(0xabc));
        e.register(FORM_ID_WORD, |_, a| rv(a[0] + 1));
        e.register(CELL_FLAG_ZERO_TEST, |_, _| rv(0));
        CheckSetup {
            e,
            this,
            buffer,
            form: form_cell,
        }
    }

    fn run_check(s: &mut CheckSetup) -> u32 {
        bgssaveloadgame_check_initial_data(
            &mut s.e,
            s.this,
            Ptr::new(0x4000_0400),
            Ptr::new(s.buffer),
            Ptr::new(0x4000_0500),
        )
    }

    #[test]
    fn check_initial_data_creates_a_reference_in_a_loaded_place() {
        let mut s = check_setup(5, 0, 0);
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        s.e.register(CREATE_REFERENCE, |_, _| rv(0x6000_0001));
        assert_eq!(run_check(&mut s), 0xabc);
        let init = calls(&s.e, LOAD_INITIAL_DATA_FROM_FILE)[0][2];
        assert_eq!(
            calls(&s.e, LOAD_INITIAL_DATA_FROM_FILE),
            vec![vec![0x4000_0400, 5, init]]
        );
        assert_eq!(calls(&s.e, CREATE_REFERENCE), vec![vec![0x300, init, 0]]);
        assert_eq!(
            calls(&s.e, WORLD_IS_CELL_LOADED),
            vec![vec![0x4000_0100, 0xce11, 0]]
        );
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![s.buffer, 0x6000_0001]]
        );
    }

    #[test]
    fn check_initial_data_skips_the_buffer_when_the_reference_cannot_be_made() {
        let mut s = check_setup(5, 0, 0);
        // The data's flag byte 2 makes the place count as loaded.
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u8(a[2] + 0x2c, 2);
            rv(1)
        });
        s.e.register(CREATE_REFERENCE, |_, _| rv(0));
        run_check(&mut s);
        assert_eq!(s.e.mem.u32(s.buffer + 0x28) & 1, 1);
        assert!(calls(&s.e, LOAD_FORM_BUFFER_SET_FORM).is_empty());
    }

    #[test]
    fn check_initial_data_replaces_a_created_form_that_changed() {
        // Kind 5 with a live reference that has stored changes: the old form is
        // deleted, the old flags cleared and a new reference made.
        let mut s = check_setup(5, 0, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(BUFFER_GET_FORM, |_, _| rv(0x6000_0800));
        // The base form's id is not the data's (init + 4 is 0): mismatch.
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        s.e.register(CREATE_REFERENCE, |_, a| rv(a[2] + 0x100));
        run_check(&mut s);
        // Mismatch: ClearForm of the form (type 0x2a: renumbered), buffer form
        // and old flags reset, then a fresh reference is made without a form.
        assert_eq!(calls(&s.e, FORM_GET_TYPE), vec![vec![form]]);
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![s.buffer, 0]]
        );
        assert_eq!(calls(&s.e, CREATE_REFERENCE)[0][2], 0);
        assert_eq!(calls(&s.e, LOAD_FORM_BUFFER_SET_FORM)[0], vec![s.buffer, 0]);
    }

    #[test]
    fn check_initial_data_deletes_a_changed_form_and_recreates_it() {
        // Same base form: not a mismatch; the combined flags are not zero, so
        // the form is deleted (slot 0x10 with 1) before the reference is made.
        let mut s = check_setup(5, 0, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(BUFFER_GET_FORM, |_, _| rv(0x6000_0800));
        // FORM_ID_WORD(base) = 0x6000_0801 = the data's word at init + 4.
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[2] + 4, 0x6000_0801);
            rv(1)
        });
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        s.e.register(COMBINE_CHANGE_FLAGS, |e, a| {
            e.mem.set_u32(a[0], 0x8);
            Ret::default()
        });
        s.e.register(CREATE_REFERENCE, |_, _| rv(0x6000_0020));
        run_check(&mut s);
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x10).len(), 1);
        assert_eq!(
            calls(&s.e, COMBINE_CHANGE_FLAGS)[0][1..],
            [0x44, 0x33, 0x41, 1]
        );
        assert_eq!(calls(&s.e, CREATE_REFERENCE)[0][2], 0);
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![s.buffer, 0]]
        );
    }

    #[test]
    fn check_initial_data_adds_a_moved_reference_to_the_reconstruct_map() {
        let mut s = check_setup(6, 0, 0);
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        s.e.register(GET_ORIGINAL_LOCATION, |e, a| {
            e.mem.set_u32(a[1], 0xce22);
            Ret::default()
        });
        run_check(&mut s);
        assert_eq!(
            calls(&s.e, RECONSTRUCT_ADD_REFERENCE),
            vec![vec![0x4000_0020, 0xce22, 0x300, 0]]
        );
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_LOADED),
            vec![vec![s.buffer, 1]]
        );
        assert_eq!(s.e.mem.u32(s.buffer + 0x28) & 2, 2);
        assert!(calls(&s.e, CREATE_REFERENCE).is_empty());
    }

    #[test]
    fn check_initial_data_uses_the_world_coordinates_without_an_original_cell() {
        let mut s = check_setup(6, 0, 0);
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[2] + 0xc, 0x11);
            e.mem.set_u32(a[2] + 0x10, 0x22);
            e.mem.set_u8(a[2] + 0x2c, 2);
            rv(1)
        });
        s.e.register(GET_ORIGINAL_LOCATION, |e, a| {
            e.mem.set_u32(a[2], 0x5151);
            Ret::default()
        });
        run_check(&mut s);
        assert_eq!(
            calls(&s.e, RECONSTRUCT_ADD_REFERENCE_AT),
            vec![vec![0x4000_0020, 0x5151, 0x11, 0x22, 0x300, 0]]
        );
    }

    #[test]
    fn check_initial_data_keeps_the_current_place_when_the_original_writes_nothing() {
        // 00849f21: the slots `0084e610` filled are not cleared before
        // `0084e6a0`; when it writes nothing the current cell is used.
        let mut s = check_setup(6, 0, 0);
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        run_check(&mut s);
        assert_eq!(calls(&s.e, GET_ORIGINAL_LOCATION).len(), 1);
        assert_eq!(
            calls(&s.e, RECONSTRUCT_ADD_REFERENCE),
            vec![vec![0x4000_0020, 0xce11, 0x300, 0]]
        );
        assert!(calls(&s.e, RECONSTRUCT_ADD_REFERENCE_AT).is_empty());
        // No current cell, only a world (flag 2 forces the branch): the world
        // `0084e610` wrote is the one the coordinates are placed in.
        let mut s = check_setup(6, 0, 0);
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[2] + 0xc, 0x11);
            e.mem.set_u32(a[2] + 0x10, 0x22);
            e.mem.set_u8(a[2] + 0x2c, 2);
            rv(1)
        });
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[2], 0x5252);
            Ret::default()
        });
        run_check(&mut s);
        assert!(calls(&s.e, RECONSTRUCT_ADD_REFERENCE).is_empty());
        assert_eq!(
            calls(&s.e, RECONSTRUCT_ADD_REFERENCE_AT),
            vec![vec![0x4000_0020, 0x5252, 0x11, 0x22, 0x300, 0]]
        );
    }

    #[test]
    fn check_initial_data_records_an_unloaded_reference() {
        // Not loaded, an interior cell: the references map is set up with the
        // data's first word; kind 6 marks the id too.
        let mut s = check_setup(6, 0, 0);
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            rv(1)
        });
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(0));
        s.e.register(CELL_FLAG_ZERO_TEST, |_, _| rv(1));
        run_check(&mut s);
        assert_eq!(
            calls(&s.e, REFERENCES_MAP_SETUP),
            vec![vec![0x4000_0010 + 0x10, 0x7777, 0x300]]
        );
        assert_eq!(calls(&s.e, CELL_FLAG_ZERO_TEST), vec![vec![0xce11]]);
    }

    #[test]
    fn check_initial_data_records_an_unloaded_reference_by_world() {
        let mut s = check_setup(5, 0, 0);
        s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            rv(1)
        });
        s.e.register(LOCATION_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[2], 0xaa);
            Ret::default()
        });
        run_check(&mut s);
        let init = calls(&s.e, LOAD_INITIAL_DATA_FROM_FILE)[0][2];
        assert_eq!(
            calls(&s.e, REFERENCES_MAP_ADD_UNLOADED),
            vec![vec![0x4000_0010, 0x7777, 0x300, init + 0x14]]
        );
    }

    #[test]
    fn check_initial_data_warns_when_no_place_is_known() {
        for (kind, text) in [(5, TEXT_CREATED), (6, TEXT_MOVED)] {
            let mut s = check_setup(kind, 0, 0);
            s.e.register_double(LOAD_INITIAL_DATA_FROM_FILE, |e, a| {
                e.mem.set_u32(a[2], 0x7777);
                rv(1)
            });
            run_check(&mut s);
            assert_eq!(
                calls(&s.e, SAVE_GAME_WARNING),
                vec![vec![MISSING_PLACE_WARNING, 0x7777, text]]
            );
            assert_eq!(s.e.mem.u32(s.buffer + 0x28) & 1, 1);
        }
    }

    #[test]
    fn check_initial_data_removes_the_loaded_reference_of_an_unplaceable_moved_reference() {
        // A reference form that is neither persistent nor the player, with no
        // place: warned about, then removed from its cell and deleted.
        let mut s = check_setup(6, 0, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(REFERENCE_PARENT_CELL, |_, _| rv(0xce11));
        run_check(&mut s);
        assert_eq!(calls(&s.e, CELL_REMOVE_REFERENCE), vec![vec![0xce11, form]]);
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x10).len(), 1);
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![s.buffer, 0]]
        );
    }

    #[test]
    fn check_initial_data_leaves_a_persistent_or_player_reference_alone() {
        let mut s = check_setup(6, 0, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(REFERENCE_IS_PERSISTENT, |_, _| rv(1));
        run_check(&mut s);
        assert!(calls(&s.e, FAKE_TARGET + 0x10).is_empty());
        assert!(calls(&s.e, SAVE_GAME_WARNING).is_empty());

        let mut s = check_setup(6, 0, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.set_global(PLAYER, form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        run_check(&mut s);
        assert!(calls(&s.e, SAVE_GAME_WARNING).is_empty());
    }

    #[test]
    fn check_initial_data_without_a_form_reconstructs_a_stored_kind_6_entry() {
        let mut s = check_setup(0, 6, 0);
        let entry = entry_value(&mut s.e, 0x44, 0x5151);
        s.e.register_double(CHANGES_MAP_GET_ENTRY, move |e, a| {
            e.mem.set_u32(a[1], entry);
            Ret::default()
        });
        s.e.register(UNLOADED_FORM_BUFFER_ADVANCED, |_, _| rv(0x9000));
        s.e.register(GET_ORIGINAL_LOCATION, |e, a| {
            e.mem.set_u32(a[1], 0xce22);
            Ret::default()
        });
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        let this = s.this;
        run_check(&mut s);
        assert_eq!(
            calls(&s.e, RECONSTRUCT_ADD_REFERENCE),
            vec![vec![0x4000_0020, 0xce22, 0x300, 0]]
        );
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_LOADED),
            vec![vec![s.buffer, 1]]
        );
        assert_eq!(s.e.mem.u32(s.buffer + 0x28) & 2, 2);
        // The form id map is back to the original afterwards.
        assert_eq!(
            s.e.get(this, BGSSaveLoadGame::pFormIDMap),
            Ptr::new(0x4000_0030)
        );
    }

    #[test]
    fn check_initial_data_deletes_a_live_reference_whose_location_is_not_loaded() {
        let mut s = check_setup(0, 6, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(FAKE_TARGET + 0x138, |e, a| {
            e.mem.set_u32(a[3], 0xce44);
            rv(1)
        });
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x41));
        s.e.register(WORLD_SPACE_CELL_FROM_COORD, |_, _| rv(0xce33));
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(0));
        run_check(&mut s);
        assert_eq!(
            calls(&s.e, WORLD_IS_CELL_LOADED),
            vec![vec![0x4000_0100, 0xce33, 0]]
        );
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x10).len(), 1);
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![s.buffer, 0]]
        );
        assert_eq!(
            calls(&s.e, LOAD_FORM_BUFFER_SET_OLD_FLAGS),
            vec![vec![s.buffer, 0]]
        );
    }

    #[test]
    fn check_initial_data_keeps_a_live_reference_whose_location_is_loaded() {
        let mut s = check_setup(0, 6, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(FAKE_TARGET + 0x138, |e, a| {
            e.mem.set_u32(a[3], 0xce44);
            rv(1)
        });
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x2a));
        s.e.register(WORLD_IS_CELL_LOADED, |_, _| rv(1));
        assert_eq!(run_check(&mut s), 0);
        assert!(calls(&s.e, FAKE_TARGET + 0x10).is_empty());
        // A reference without a location is removed.
        let mut s = check_setup(0, 6, 0);
        let form = object(&mut s.e);
        s.form.set(form);
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        run_check(&mut s);
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x10).len(), 1);
    }

    #[test]
    fn check_initial_data_ignores_other_kinds() {
        let mut s = check_setup(1, 2, 0);
        assert_eq!(run_check(&mut s), 0);
        assert!(calls(&s.e, LOAD_INITIAL_DATA_FROM_FILE).is_empty());
        assert!(calls(&s.e, CREATE_REFERENCE).is_empty());
    }

    // ---- 0084a360 to 0084a5b0 ------------------------------------------------

    #[test]
    fn buffer_flag_helper_sets_and_clears_mask_2() {
        let mut e = engine2();
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u32(buffer + 0x28, 0x5);
        fn_0084a360(&mut e, Ptr::new(buffer), true);
        assert_eq!(e.mem.u32(buffer + 0x28), 7);
        fn_0084a360(&mut e, Ptr::new(buffer), false);
        assert_eq!(e.mem.u32(buffer + 0x28), 5);
    }

    struct RevertSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
        form: u32,
        form_slot: u32,
    }

    fn revert_setup() -> RevertSetup {
        let mut e = engine2();
        let this = game(&mut e);
        let form = object(&mut e);
        let form_slot = e.mem.alloc(4);
        e.mem.set_u32(form_slot, form);
        e.set_global(PLAYER, 0x4000_0200u32);
        e.set_global(DATA_HANDLER, 0x4000_0300u32);
        e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        RevertSetup {
            e,
            this,
            form,
            form_slot,
        }
    }

    #[test]
    fn unrevertible_changes_always_accept_the_player() {
        let mut s = revert_setup();
        s.e.set_global(PLAYER, s.form);
        assert!(bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0xe,
            true
        ));
        assert!(calls(&s.e, FLAGS_TEST).is_empty());
    }

    #[test]
    fn unrevertible_changes_clear_the_bits_of_a_dynamic_reference_and_queue_its_actor() {
        let mut s = revert_setup();
        s.e.register(IS_DYNAMIC_FORM_ID, |_, _| rv(1));
        s.e.register(FAKE_TARGET + 0x100, |_, _| rv(1));
        let process = s.e.mem.alloc(0x200);
        s.e.mem.set_u32(process + 0x100, TEST_VTABLE);
        s.e.mem.set_u32(TEST_VTABLE + 0xc, FAKE_TARGET + 0xc);
        s.e.register_double(ACTOR_PROCESS, move |_, _| rv(process));
        s.e.register(FAKE_TARGET + 0xc, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        s.e.mem.set_f64(ZERO_DOUBLE, 0.0);
        // Only bit 0x2 (outside mask 0xe's bit... 0x6 has 0x2 and 0x4): after the
        // clear, nothing is left of 0x6.
        let done = bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x6,
            true,
        );
        assert!(done);
        assert!(calls(&s.e, REFERENCE_MOVE_TO_LOCATION).is_empty());
        assert_eq!(
            calls(&s.e, MAP_SET_AT),
            vec![vec![s.this.addr() + 0x34, 0x300, s.form]]
        );
        // Bits outside the mask stay: the result is false.
        let kept = bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x1006,
            true,
        );
        assert!(!kept);
    }

    #[test]
    fn unrevertible_changes_accept_a_reference_that_moves_back() {
        let mut s = revert_setup();
        s.e.register(REFERENCE_MOVE_TO_LOCATION, |_, _| rv(1));
        assert!(bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x2,
            false
        ));
        assert_eq!(
            calls(&s.e, REFERENCE_MOVE_TO_LOCATION),
            vec![vec![s.form, 0]]
        );
        // The form is not an actor here: nothing is queued.
        assert!(calls(&s.e, MAP_SET_AT).is_empty());
    }

    #[test]
    fn unrevertible_changes_delete_a_non_persistent_reference_on_request() {
        let mut s = revert_setup();
        let done = bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x2,
            true,
        );
        assert!(!done, "the flags are still there");
        assert_eq!(calls(&s.e, FAKE_TARGET + 0x10).len(), 1);
        assert_eq!(s.e.mem.u32(s.form_slot), 0);
        assert!(calls(&s.e, SAVE_GAME_WARNING).is_empty());
    }

    #[test]
    fn unrevertible_changes_warn_otherwise() {
        let mut s = revert_setup();
        s.e.register(FORM_NAME_LENGTH, |_, _| rv(3));
        s.e.register(FAKE_TARGET + 0x130, |_, _| rv(0xaaaa));
        s.e.register(BUFFER_GET_FORM, |_, _| rv(0xbbbb));
        s.e.register(FORM_TYPE_NAME, |_, a| rv(a[0] + 1));
        bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x2,
            false,
        );
        assert_eq!(
            calls(&s.e, SAVE_GAME_WARNING),
            vec![vec![REVERT_LOCATION_WARNING, 0xaaaa, 0x300, 0xbbbc, 0x2]]
        );
        assert_eq!(s.e.mem.u32(s.form_slot), s.form);
        // A persistent reference warns even when deletion was requested, with
        // the full name when the type name text is empty.
        let mut s = revert_setup();
        s.e.register(REFERENCE_IS_PERSISTENT, |_, _| rv(1));
        s.e.register(REFERENCE_FULL_NAME, |_, _| rv(0xcccc));
        bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x2,
            true,
        );
        assert_eq!(calls(&s.e, SAVE_GAME_WARNING)[0][1], 0xcccc);
        assert!(calls(&s.e, FAKE_TARGET + 0x10).is_empty());
    }

    #[test]
    fn unrevertible_changes_ignore_other_forms_and_masks() {
        let mut s = revert_setup();
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(0));
        assert!(!bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x5,
            true
        ));
        assert!(calls(&s.e, FLAGS_TEST).is_empty());
        let mut s = revert_setup();
        assert!(!bgssaveloadgame_handle_unrevertible_changes(
            &mut s.e,
            s.this,
            Ptr::new(s.form_slot),
            0x1,
            true
        ));
        assert!(calls(&s.e, FORM_ID_WORD).is_empty());
    }

    #[test]
    fn queue_init_package_locations_needs_a_positive_value() {
        let mut e = engine2();
        let this = game(&mut e);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let process = e.mem.alloc(0x200);
        e.mem.set_u32(process + 0x100, TEST_VTABLE);
        e.mem.set_u32(TEST_VTABLE + 0xc, FAKE_TARGET + 0xc);
        e.register_double(ACTOR_PROCESS, move |_, _| rv(process));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        e.register(FAKE_TARGET + 0xc, |_, _| Ret {
            st0: 0.0,
            ..Ret::default()
        });
        bgssaveloadgame_queue_init_package_locations(&mut e, this, Ptr::new(0x77));
        assert!(calls(&e, MAP_SET_AT).is_empty());
        assert_eq!(
            calls(&e, FAKE_TARGET + 0xc),
            vec![vec![process + 0x100, 0x10]]
        );
        e.register(FAKE_TARGET + 0xc, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        bgssaveloadgame_queue_init_package_locations(&mut e, this, Ptr::new(0x77));
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![this.addr() + 0x34, 0x300, 0x77]]
        );
    }

    #[test]
    fn cancel_init_package_locations_removes_the_form_id() {
        let mut e = engine2();
        let this = game(&mut e);
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        bgssaveloadgame_cancel_init_package_locations(&mut e, this, Ptr::new(0x77));
        assert_eq!(
            calls(&e, MAP_REMOVE_AT),
            vec![vec![this.addr() + 0x34, 0x300]]
        );
    }

    #[test]
    fn after_load_pass_initialises_the_actors_that_have_an_acquire_object() {
        let mut e = engine2();
        let this = game(&mut e);
        let remaining = Rc::new(Cell::new(2u32));
        e.register(MAP_FIRST_POSITION, |_, _| rv(1));
        let counter = remaining.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let left = counter.get();
            counter.set(left - 1);
            e.mem.set_u32(a[1], if left > 1 { 1 } else { 0 });
            e.mem.set_u32(a[2], 0x300 + left);
            e.mem.set_u32(a[3], 0x7000 + left);
            Ret::default()
        });
        e.register(GET_SAVED_ACQUIRE_OBJECT, |_, a| rv((a[0] == 0x7002) as u32));
        fn_0084a5b0(&mut e, this);
        assert_eq!(calls(&e, GET_SAVED_ACQUIRE_OBJECT).len(), 2);
        assert_eq!(
            calls(&e, ACTOR_INIT_PACKAGE_LOCATIONS),
            vec![vec![0x7002, 1]]
        );
        assert_eq!(
            calls(&e, CHANGED_FORM_ID_MAP_FLUSH),
            vec![vec![this.addr() + 0x34]]
        );
        assert_eq!(calls(&e, MAP_NEXT)[0][0], this.addr() + 0x34);
    }

    // ---- 0084a620 to 0084a840 ------------------------------------------------

    #[test]
    fn change_recording_test_checks_the_state_the_flags_and_dynamic_ids() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set_global(DATA_HANDLER, 0x4000_0300u32);
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        let form = Ptr::new(0x77);
        // Neither recording nor forced.
        assert!(!fn_0084a620(&mut e, this, form, false));
        // Forced, nothing else in the way.
        assert!(fn_0084a620(&mut e, this, form, true));
        // Recording, but the form has bit 0x4000.
        e.register(GAME_RECORDING_TEST, |_, _| rv(1));
        e.register(FORM_FLAG_4000_TEST, |_, _| rv(1));
        assert!(!fn_0084a620(&mut e, this, form, false));
        e.register(FORM_FLAG_4000_TEST, |_, _| rv(0));
        assert!(fn_0084a620(&mut e, this, form, false));
        // With bit 0x20 a dynamic id is refused.
        e.register(FORM_FLAG_20_TEST, |_, _| rv(1));
        e.register(IS_DYNAMIC_FORM_ID, |_, _| rv(1));
        assert!(!fn_0084a620(&mut e, this, form, false));
        e.register(IS_DYNAMIC_FORM_ID, |_, _| rv(0));
        assert!(fn_0084a620(&mut e, this, form, false));
        assert_eq!(calls(&e, IS_DYNAMIC_FORM_ID)[0], vec![0x4000_0300, 0x300]);
    }

    #[test]
    fn add_change_records_the_flags_only_when_allowed() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        bgssaveloadgame_add_change(&mut e, this, Ptr::new(0x77), 0x40, false);
        assert!(calls(&e, CHANGES_MAP_ADD_FLAGS).is_empty());
        bgssaveloadgame_add_change(&mut e, this, Ptr::new(0x77), 0x40, true);
        assert_eq!(
            calls(&e, CHANGES_MAP_ADD_FLAGS),
            vec![vec![0x4000_0000, 0x300, 0x40]]
        );
    }

    #[test]
    fn change_queries_use_the_current_and_the_old_map() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        e.register(CHANGES_MAP_HAS_FLAGS, |_, a| {
            rv((a[0] == 0x4000_0000) as u32)
        });
        assert!(bgssaveloadgame_get_change(&mut e, this, Ptr::new(0x77), 8));
        assert_eq!(
            calls(&e, CHANGES_MAP_HAS_FLAGS),
            vec![vec![0x4000_0000, 0x300, 8]]
        );
        // No old map: false without asking.
        assert!(!fn_0084a700(&mut e, this, Ptr::new(0x77), 8));
        assert_eq!(calls(&e, CHANGES_MAP_HAS_FLAGS).len(), 1);
        e.set(this, BGSSaveLoadGame::pOldChangesMap, Ptr::new(0x4000_0040));
        assert!(!fn_0084a700(&mut e, this, Ptr::new(0x77), 8));
        assert_eq!(calls(&e, CHANGES_MAP_HAS_FLAGS)[1][0], 0x4000_0040);
    }

    #[test]
    fn recorded_form_test_and_flag_clear() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        let form = Ptr::new(0x77);
        assert!(!fn_0084a740(&mut e, this, form));
        fn_0084a780(&mut e, this, form, 0x10);
        assert!(calls(&e, CHANGES_MAP_CLEAR_FLAGS).is_empty());
        e.register(GAME_RECORDING_TEST, |_, _| rv(1));
        assert!(fn_0084a740(&mut e, this, form));
        fn_0084a780(&mut e, this, form, 0x10);
        assert_eq!(
            calls(&e, CHANGES_MAP_CLEAR_FLAGS),
            vec![vec![0x4000_0000, 0x300, 0x10]]
        );
        e.register(FORM_FLAG_4000_TEST, |_, _| rv(1));
        assert!(!fn_0084a740(&mut e, this, form));
    }

    #[test]
    fn remove_changes_keeps_an_entry_with_a_stored_buffer() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        e.register(READ_WORD, |e, a| rv(e.mem.u32(a[0])));
        // No entry.
        bgssaveloadgame_remove_changes(&mut e, this, Ptr::new(0x77));
        assert!(calls(&e, CHANGES_MAP_REMOVE).is_empty());
        // An entry with a stored buffer stays.
        let entry = entry_value(&mut e, 4, 0x5151);
        e.register_double(CHANGES_MAP_ENTRY_FOR, move |_, _| rv(entry));
        bgssaveloadgame_remove_changes(&mut e, this, Ptr::new(0x77));
        assert!(calls(&e, CHANGES_MAP_REMOVE).is_empty());
        // Without a stored buffer it goes.
        e.mem.set_u32(entry + 4, 0);
        bgssaveloadgame_remove_changes(&mut e, this, Ptr::new(0x77));
        assert_eq!(
            calls(&e, CHANGES_MAP_REMOVE),
            vec![vec![0x4000_0000, 0x300]]
        );
    }

    #[test]
    fn queue_sub_buffer_forwards_to_the_queued_map() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(
            this,
            BGSSaveLoadGame::pQueuedSubBuffersMap,
            Ptr::new(0x4000_0014),
        );
        bgssaveloadgame_queue_sub_buffer(&mut e, this, 1, 2, 3);
        assert_eq!(
            calls(&e, QUEUED_SUB_BUFFERS_QUEUE),
            vec![vec![0x4000_0014, 1, 2, 3]]
        );
    }

    #[test]
    fn load_finish_sequence_runs_in_order_with_the_load_flag_set() {
        let mut e = engine2();
        let this = game(&mut e);
        let current = object(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(current));
        e.set(
            this,
            BGSSaveLoadGame::pReconstructForms,
            Ptr::new(0x4000_0020),
        );
        e.register(MAP_FIRST_POSITION, |_, _| rv(0));
        let flag_during = Rc::new(Cell::new(0u32));
        let seen = flag_during.clone();
        let this_addr = this.addr();
        e.register_double(GLOBAL_DATA_RESET, move |e, _| {
            seen.set(e.mem.u32(this_addr + 0x244));
            Ret::default()
        });
        fn_0084a840(&mut e, this);
        let log = order(&e);
        let reset = log.iter().position(|a| *a == GLOBAL_DATA_RESET).unwrap();
        let flush = log
            .iter()
            .position(|a| *a == CHANGED_FORM_ID_MAP_FLUSH)
            .unwrap();
        let finish = log.iter().position(|a| *a == RECONSTRUCT_FINISH_A).unwrap();
        assert!(reset < finish && finish < flush);
        assert_eq!(calls(&e, GLOBAL_DATA_RESET), vec![vec![1]]);
        // The load flag was set during the reset and is clear afterwards.
        assert_eq!(flag_during.get() & 2, 2);
        assert_eq!(e.get(this, BGSSaveLoadGame::iGlobalFlags) & 2, 0);
    }

    // ---- ClearForm and the id helpers -----------------------------------------

    #[test]
    fn clear_form_renumbers_deletes_or_warns_by_type() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set_global(DATA_HANDLER, 0x4000_0300u32);
        e.register(FORM_ID_WORD, |_, _| rv(0x300));
        e.register(DATA_HANDLER_GET_NEXT_ID, |_, _| rv(0x01_000099));
        e.register(TYPE_NAME_FOR, |_, a| rv(0x5000 + a[0]));
        let form = object(&mut e);
        // Null form: nothing happens.
        bgssaveloadgame_clear_form(&mut e, this, Ptr::NULL);
        assert!(order(&e).is_empty());
        // Silent renumbering.
        for form_type in [0x04, 0x11, 0x12, 0x2a, 0x2b, 0x39, 0x42, 0x43] {
            e.register_double(FORM_GET_TYPE, move |_, _| rv(form_type));
            bgssaveloadgame_clear_form(&mut e, this, Ptr::new(form));
        }
        assert_eq!(calls(&e, FAKE_TARGET + 0x128).len(), 8);
        assert_eq!(
            calls(&e, FAKE_TARGET + 0x128)[0],
            vec![form, 0x01_000099, 1]
        );
        assert_eq!(calls(&e, DATA_HANDLER_GET_NEXT_ID)[0], vec![0x4000_0300]);
        assert!(calls(&e, SAVE_GAME_WARNING).is_empty());
        // Deletion.
        for form_type in [0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x49, 0x69] {
            e.register_double(FORM_GET_TYPE, move |_, _| rv(form_type));
            bgssaveloadgame_clear_form(&mut e, this, Ptr::new(form));
        }
        assert_eq!(calls(&e, FAKE_TARGET + 0x10).len(), 9);
        assert_eq!(calls(&e, FAKE_TARGET + 0x128).len(), 8);
        // Anything else: warning, then renumbering. Types 0x41, 0x6a and 0x02.
        for form_type in [0x41u32, 0x6a, 0x02, 0xff] {
            e.register_double(FORM_GET_TYPE, move |_, _| rv(form_type));
            bgssaveloadgame_clear_form(&mut e, this, Ptr::new(form));
        }
        assert_eq!(calls(&e, FAKE_TARGET + 0x128).len(), 12);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING)[0],
            vec![RENUMBERED_WARNING, 0x300, 0x41, 0x5041]
        );
        assert_eq!(calls(&e, SAVE_GAME_WARNING).len(), 4);
    }

    #[test]
    fn clear_form_id_looks_the_form_up() {
        let mut e = engine2();
        let this = game(&mut e);
        e.register(LOOKUP_FORM, |_, _| rv(0));
        bgssaveloadgame_clear_form_id(&mut e, this, 0x300);
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![0x300]]);
        assert!(calls(&e, FORM_GET_TYPE).is_empty());
        let form = object(&mut e);
        e.register_double(LOOKUP_FORM, move |_, _| rv(form));
        e.register(FORM_GET_TYPE, |_, _| rv(0x3a));
        bgssaveloadgame_clear_form_id(&mut e, this, 0x300);
        assert_eq!(calls(&e, FAKE_TARGET + 0x10).len(), 1);
    }

    #[test]
    fn delete_form_calls_slot_0x10_with_one() {
        let mut e = engine2();
        let this = game(&mut e);
        let form = object(&mut e);
        bgssaveloadgame_delete_form(&mut e, this, Ptr::NULL);
        assert!(order(&e).is_empty());
        bgssaveloadgame_delete_form(&mut e, this, Ptr::new(form));
        assert_eq!(calls(&e, FAKE_TARGET + 0x10), vec![vec![form, 1]]);
    }

    #[test]
    fn changed_form_id_map_warns_on_a_second_mapping() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(
            this,
            BGSSaveLoadGame::pChangedFormIDMap,
            Ptr::new(0x4000_0018),
        );
        bgssaveloadgame_add_changed_form_id(&mut e, this, 0x100, 0x200);
        assert!(calls(&e, SAVE_GAME_WARNING).is_empty());
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![0x4000_0018, 0x100, 0x200]]);
        e.register(MAP_GET_VALUE, |e, a| {
            e.mem.set_u32(a[2], 0x300);
            rv(1)
        });
        bgssaveloadgame_add_changed_form_id(&mut e, this, 0x100, 0x400);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![CHANGED_ID_CONFLICT_WARNING, 0x400, 0x100, 0x300]]
        );
        assert_eq!(calls(&e, MAP_SET_AT)[1], vec![0x4000_0018, 0x100, 0x400]);
    }

    #[test]
    fn changed_id_and_entry_lookups() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(
            this,
            BGSSaveLoadGame::pChangedFormIDMap,
            Ptr::new(0x4000_0018),
        );
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        assert_eq!(fn_0084aa90(&mut e, this, 0x100), 0);
        e.register(MAP_GET_VALUE, |e, a| {
            e.mem.set_u32(a[2], 0x300);
            rv(1)
        });
        assert_eq!(fn_0084aa90(&mut e, this, 0x100), 0x300);
        assert_eq!(calls(&e, MAP_GET_VALUE)[1][..2], [0x4000_0018, 0x100]);
        assert!(!fn_0084aad0(&mut e, this, 0x100));
        e.register(CHANGES_MAP_ENTRY_FOR, |_, _| rv(0x9000));
        assert!(fn_0084aad0(&mut e, this, 0x100));
        assert_eq!(
            calls(&e, CHANGES_MAP_ENTRY_FOR)[0],
            vec![0x4000_0000, 0x100]
        );
    }

    #[test]
    fn save_game_warning_function_forwards_to_the_history() {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pHistory, Ptr::new(0x4000_001c));
        bgssaveloadgame_save_game_warning_function(&mut e, this, 0x1111, 0x2222);
        assert_eq!(
            calls(&e, HISTORY_ADD_NOTE_VA),
            vec![vec![0x4000_001c, 0x1111, 0x2222]]
        );
    }

    // ---- CleanupExpiredData ---------------------------------------------------

    /// What a scripted `MAP_NEXT` hands out: the key and the entry value.
    fn script_map(e: &mut Engine, entries: Vec<(u32, u32)>) {
        let queue = Rc::new(RefCell::new(entries));
        e.register(MAP_FIRST_POSITION, |_, _| rv(1));
        let walk = Rc::new(Cell::new(0usize));
        let cursor = walk.clone();
        let pending = queue.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let all = pending.borrow();
            let index = cursor.get() % all.len();
            cursor.set(cursor.get() + 1);
            e.mem.set_u32(a[2], all[index].0);
            e.mem.set_u32(a[3], all[index].1);
            e.mem
                .set_u32(a[1], if index + 1 < all.len() { 1 } else { 0 });
            Ret::default()
        });
    }

    struct CleanupSetup {
        e: Engine,
        this: Ptr<BGSSaveLoadGame>,
    }

    fn cleanup_setup() -> CleanupSetup {
        let mut e = engine2();
        let this = game(&mut e);
        e.set(this, BGSSaveLoadGame::pChangesMap, Ptr::new(0x4000_0000));
        e.set(this, BGSSaveLoadGame::pReferencesMap, Ptr::new(0x4000_0010));
        e.set(this, BGSSaveLoadGame::pHistory, Ptr::new(0x4000_001c));
        e.set_global(DATA_HANDLER, 0x4000_0300u32);
        e.register(CALENDAR_TIME_STAMP, |_, _| rv(1000));
        e.register(EXPIRY_LIMIT, |_, _| rv(50));
        e.register(SET_THREAD_FLAG, |_, _| rv(1));
        e.register(PACK_GRID_COORDS, |_, a| rv((a[0] << 16) | a[1]));
        e.register(ROUND_TO_INT, |_, a| rv(f32::from_bits(a[0]) as i32 as u32));
        e.register(READ_WORD, |e, a| rv(e.mem.u32(a[0])));
        e.register(IS_DYNAMIC_FORM_ID, |_, _| rv(0));
        CleanupSetup { e, this }
    }

    #[test]
    fn cleanup_does_nothing_when_no_cell_has_expired() {
        let mut s = cleanup_setup();
        let entry = entry_value(&mut s.e, 0x4000_0000, 0);
        script_map(&mut s.e, vec![(0x500, entry)]);
        s.e.register(LOOKUP_FORM, |_, _| rv(0xce11));
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x39));
        s.e.register(CELL_DETACH_TIME, |_, _| rv(990));
        bgssaveloadgame_cleanup_expired_data(&mut s.e, s.this);
        assert!(calls(&s.e, EXPIRY_QUEUE_INSERT).is_empty());
        assert!(calls(&s.e, EXPIRY_QUEUE_FIND).is_empty());
        assert!(calls(&s.e, HISTORY_ADD_NOTE).is_empty());
        // The thread flag is cleared first and then given back; the queue is
        // built, flushed and destroyed.
        let thread = calls(&s.e, SET_THREAD_FLAG);
        assert_eq!(thread[0][1], 0);
        assert_eq!(thread[1][1], 1);
        let queue = calls(&s.e, EXPIRY_QUEUE_BASE_CONSTRUCT)[0].clone();
        assert_eq!(queue[1], 0x25);
        assert_eq!(calls(&s.e, EXPIRY_QUEUE_DESTRUCT), vec![vec![queue[0]]]);
        assert_eq!(calls(&s.e, CHANGED_FORM_ID_MAP_FLUSH), vec![vec![queue[0]]]);
        assert_eq!(s.e.mem.u32(queue[0]), EXPIRY_QUEUE_VTABLE);
    }

    #[test]
    fn cleanup_removes_a_whole_entry_of_a_live_reference_in_an_expired_cell() {
        let mut s = cleanup_setup();
        // Entry 1: a loaded exterior cell detached at time 100 (grid 5, 6 of
        // worldspace id 0x3a1). Entry 2: a live reference in that cell with
        // exactly change bit 4.
        let cell_entry = entry_value(&mut s.e, 0x4000_0000, 0);
        let ref_entry = entry_value(&mut s.e, 4, 0);
        script_map(&mut s.e, vec![(0x500, cell_entry), (0x600, ref_entry)]);
        let reference = object(&mut s.e);
        let cell = object(&mut s.e);
        s.e.register_double(LOOKUP_FORM, move |_, a| {
            rv(if a[0] == 0x500 { cell } else { reference })
        });
        s.e.register_double(FORM_GET_TYPE, move |_, a| {
            rv(if a[0] == cell { 0x39 } else { 0x3a })
        });
        s.e.register(CELL_DETACH_TIME, |_, _| rv(100));
        s.e.register(CELL_WORLD_SPACE, |_, _| rv(0xaa));
        s.e.register(FORM_ID_WORD, |_, a| rv(a[0] + 0x3a0 - 0xaa));
        s.e.register(CELL_DATA_X, |_, _| rv(5));
        s.e.register(CELL_DATA_Y, |_, _| rv(6));
        s.e.register(FAKE_TARGET + 0xf0, |_, _| rv(1));
        s.e.register(REFERENCE_WORLD_SPACE, |_, _| rv(0xaa));
        let place = s.e.mem.alloc(12);
        s.e.mem.set_u32(place, (5.0f32 * 4096.0).to_bits());
        s.e.mem.set_u32(place + 4, (6.0f32 * 4096.0).to_bits());
        s.e.register_double(FAKE_TARGET + 0x1f4, move |_, _| rv(place));
        s.e.register(EXPIRY_QUEUE_FIND, |_, _| rv(1));
        bgssaveloadgame_cleanup_expired_data(&mut s.e, s.this);
        // First pass: the cell went into the queue under (worldspace, grid).
        assert_eq!(
            calls(&s.e, EXPIRY_QUEUE_INSERT)
                .iter()
                .map(|w| w[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0x3a0, (5 << 16) | 6, 100]]
        );
        // Second pass: the reference's key is the same; its entry is removed.
        let found = &calls(&s.e, EXPIRY_QUEUE_FIND)[0];
        assert_eq!(found[1..3], [0x3a0, (5 << 16) | 6]);
        assert_eq!(
            calls(&s.e, CHANGES_MAP_REMOVE),
            vec![vec![0x4000_0000, 0x600]]
        );
        assert_eq!(
            calls(&s.e, REFERENCE_MOVE_TO_LOCATION),
            vec![vec![reference, 0]]
        );
        assert_eq!(
            calls(&s.e, HISTORY_ADD_NOTE),
            vec![vec![0x4000_001c, CLEANUP_NOTE, 1, 1, 0]]
        );
    }

    #[test]
    fn cleanup_rebuilds_the_stored_buffer_of_a_partially_cleaned_entry() {
        let mut s = cleanup_setup();
        let cell_block = s.e.mem.alloc(0x10);
        let ref_block = s.e.mem.alloc(0x10);
        s.e.mem.set_u8(ref_block + 1, 0x5a);
        let cell_entry = entry_value(&mut s.e, 0x4000_0000, cell_block);
        let ref_entry = entry_value(&mut s.e, 0x4000_000c, ref_block);
        script_map(&mut s.e, vec![(0x3a0, cell_entry), (0x600, ref_entry)]);
        s.e.register_double(STORED_BUFFER_TYPE, move |_, a| {
            rv(if a[0] == cell_block { 0x39 } else { 0x3a })
        });
        s.e.register(INITIAL_DATA_KIND, |_, a| {
            rv(if a[1] == 0x39 { 3 } else { 6 })
        });
        s.e.register(LOAD_INITIAL_DATA_STRUCT, |e, a| {
            if a[1] == 3 {
                e.mem.set_u32(a[2] + 0xc, 100);
            } else {
                e.mem.set_u32(a[2], 0x3a0);
            }
            Ret::default()
        });
        s.e.register(UNLOADED_FORM_BUFFER_ADVANCED, |_, _| rv(0x9000));
        s.e.register(EXPIRY_QUEUE_FIND, |_, _| rv(1));
        s.e.register(INITIAL_DATA_TYPE_SIZE, |_, _| rv(3));
        s.e.register(LOAD_VARIABLE_SIZED_VALUE, |e, a| {
            e.mem.set_u32(a[1], 10);
            rv(2)
        });
        s.e.register(UNLOADED_FORM_BUFFER_SIZE, |_, _| rv(40));
        bgssaveloadgame_cleanup_expired_data(&mut s.e, s.this);
        // The cell is queued under its own id (a kind 3 buffer has no grid).
        assert_eq!(
            calls(&s.e, EXPIRY_QUEUE_INSERT)
                .iter()
                .map(|w| w[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0x3a0, 0, 100]]
        );
        // The reference map forgets the reference (no cell form: by key pair).
        assert_eq!(calls(&s.e, MAP_REMOVE_AT), vec![vec![0x4000_0010, 0x600]]);
        assert_eq!(
            calls(&s.e, REFERENCES_MAP_REMOVE),
            vec![vec![0x4000_0020, 0x3a0, 0x600]]
        );
        // The buffer is rebuilt past the initial data and the old block freed.
        assert_eq!(
            calls(&s.e, UNLOADED_FORM_BUFFER_INITIALIZE),
            vec![vec![
                ref_entry + 4,
                0x9000 + 3 + 2 + 10,
                40 - 3 - 2 - 10,
                0x3a,
                0x5a
            ]]
        );
        assert_eq!(
            calls(&s.e, UNLOADED_FORM_BUFFER_CLEAR),
            vec![vec![ref_entry + 4]]
        );
        assert_eq!(calls(&s.e, OPERATOR_DELETE), vec![vec![ref_block]]);
        // Change bits 0xc are cleared; the entry stays.
        assert_eq!(s.e.mem.u32(ref_entry), 0x4000_0000);
        assert!(calls(&s.e, CHANGES_MAP_REMOVE).is_empty());
        assert_eq!(
            calls(&s.e, HISTORY_ADD_NOTE),
            vec![vec![0x4000_001c, CLEANUP_NOTE, 1, 0, 1]]
        );
    }

    #[test]
    fn cleanup_warns_about_an_entry_without_a_form_or_buffer_and_skips_entries_not_in_the_queue() {
        let mut s = cleanup_setup();
        let cell_entry = entry_value(&mut s.e, 0x4000_0000, 0);
        let lost_entry = entry_value(&mut s.e, 0x8, 0);
        let other_entry = entry_value(&mut s.e, 4, 0);
        let dynamic_entry = entry_value(&mut s.e, 4, 0);
        script_map(
            &mut s.e,
            vec![
                (0x500, cell_entry),
                (0x601, lost_entry),
                (0x602, other_entry),
                (0xff00_0001, dynamic_entry),
            ],
        );
        let cell = object(&mut s.e);
        s.e.register_double(LOOKUP_FORM, move |_, a| {
            rv(if a[0] == 0x500 { cell } else { 0 })
        });
        s.e.register(FORM_GET_TYPE, |_, _| rv(0x39));
        s.e.register(CELL_DETACH_TIME, |_, _| rv(100));
        s.e.register(CELL_FLAG_ZERO_TEST, |_, _| rv(1));
        s.e.register(IS_DYNAMIC_FORM_ID, |_, a| rv((a[1] >= 0xff00_0000) as u32));
        bgssaveloadgame_cleanup_expired_data(&mut s.e, s.this);
        // An interior cell: no worldspace, so the key stays its own id.
        assert_eq!(calls(&s.e, EXPIRY_QUEUE_INSERT).len(), 1);
        assert_eq!(
            calls(&s.e, SAVE_GAME_WARNING)
                .iter()
                .map(|w| (w[0], w[1], w[2]))
                .collect::<Vec<_>>(),
            vec![
                (CLEANUP_MISSING_BUFFER_WARNING, 0x601, 0x8),
                (CLEANUP_MISSING_BUFFER_WARNING, 0x602, 0x4)
            ]
        );
        // Nothing was found in the queue (no reference was resolved).
        assert!(calls(&s.e, EXPIRY_QUEUE_FIND).is_empty());
        assert!(calls(&s.e, HISTORY_ADD_NOTE).is_empty());
    }

    // ---- small constructors and destructors ----------------------------------

    #[test]
    fn small_helpers_read_and_forward() {
        let mut e = engine2();
        let block = e.mem.alloc(8);
        let inner = e.mem.alloc(8);
        e.mem.set_u32(block, inner);
        e.mem.set_u8(inner + 1, 0x42);
        assert_eq!(fn_0084b450(&mut e, Ptr::new(block)), 0x42);
        fn_0084b470(&mut e, Ptr::new(0x4000_0050), 0x300);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![0x4000_0050, 0x300]]);
        fn_0084b490(&mut e, Ptr::new(0x4000_0050), 1, 2);
        assert_eq!(
            calls(&e, REFERENCES_MAP_REMOVE),
            vec![vec![0x4000_0060, 1, 2]]
        );
    }

    #[test]
    fn hash_map_constructors_set_up_buckets_and_vtables() {
        let mut e = engine2();
        let map = e.mem.alloc(0x10);
        let this = fn_0084b4b0(&mut e, Ptr::new(map), 0x25);
        assert_eq!(this, Ptr::new(map));
        assert_eq!(e.mem.u32(map), PACKAGE_LOCATION_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let buckets = e.mem.u32(map + 8);
        assert_ne!(buckets, 0);
        assert_eq!(e.mem.u32(buckets + 0x24 * 4), 0);
        assert_eq!(calls(&e, ALLOCATE_BYTES), vec![vec![0x94]]);
        assert_eq!(calls(&e, MEMORY_SET), vec![vec![buckets, 0, 0x94]]);
        // The base constructor alone leaves its own vtable.
        let base = e.mem.alloc(0x10);
        fn_0084b650(&mut e, Ptr::new(base), 3);
        assert_eq!(e.mem.u32(base), HASH_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(base + 4), 3);
    }

    #[test]
    fn expiry_queue_constructor_uses_its_base_and_vtable() {
        let mut e = engine2();
        let queue = e.mem.alloc(0x10);
        let result = fn_0084b4e0(&mut e, Ptr::new(queue), 0x25);
        assert_eq!(result, Ptr::new(queue));
        assert_eq!(
            calls(&e, EXPIRY_QUEUE_BASE_CONSTRUCT),
            vec![vec![queue, 0x25]]
        );
        assert_eq!(e.mem.u32(queue), EXPIRY_QUEUE_VTABLE);
    }

    #[test]
    fn scalar_deleting_destructors_free_only_when_asked() {
        let mut e = engine2();
        let this = Ptr::new(0x5000_0000);
        assert_eq!(fn_0084b510(&mut e, this, 0), this);
        assert_eq!(
            calls(&e, PACKAGE_LOCATION_MAP_DESTRUCT),
            vec![vec![0x5000_0000]]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(fn_0084b510(&mut e, this, 1), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5000_0000]]);
        assert_eq!(fn_0084b540(&mut e, this, 3), this);
        assert_eq!(calls(&e, EXPIRY_QUEUE_DESTRUCT), vec![vec![0x5000_0000]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn form_buffer_array_constructor_and_destructor() {
        let mut e = engine2();
        let array = e.mem.alloc(0x10);
        assert_eq!(fn_0084b570(&mut e, Ptr::new(array)), Ptr::new(array));
        assert_eq!(e.mem.u32(array), FORM_BUFFER_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_BASE_SETUP), vec![vec![array, 0, 0]]);
        e.mem.set_u32(array, 0);
        fn_0084b5a0(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), FORM_BUFFER_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array, 1]]);
    }

    // ---- session 3: the expiry queue and the global data loop -------------------

    /// An engine where only the callees outside this file are doubles; the
    /// functions of this file run for real, the queue virtual slots included.
    fn engine3() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_0000, 0x0004_5000);
        for address in [
            OPERATOR_DELETE,
            MAP_CLEAR_ITEMS,
            HASH_MAP_BASE_DESTRUCT,
            FREE_BYTES,
            MAP_ITEM_FREE,
            LOAD_GLOBAL_DATA,
            LOAD_GAME_BUFFER_CONSTRUCT,
            LOAD_GAME_BUFFER_LOAD,
            LOAD_GAME_BUFFER_DESTRUCT,
            LOAD_DATA_FROM_FILE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| rv(e.mem.alloc(a[0])));
        e.register(ALLOCATE_BYTES, |e, a| rv(e.mem.alloc(a[0])));
        e.register(MEMORY_SET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// An expiry queue with `size` buckets, built by the base constructor and
    /// given the derived vtable (slots +4, +8, +0xc, +0x14, +0x18 as in the exe).
    fn expiry_queue(e: &mut Engine, size: u32) -> Ptr {
        let queue = Ptr::new(e.mem.alloc(0x10));
        fn_0084b750(e, queue.cast(), size);
        e.put_vtable(
            0x7000_0000,
            &[
                0x0084_bb00,
                0x0084_b930,
                0x0084_b950,
                0x0084_b990,
                0,
                0x0084_ba50,
                0x0084_ba70,
            ],
        );
        e.mem.set_u32(queue.addr(), 0x7000_0000);
        queue
    }

    #[test]
    fn package_location_map_destructor_body() {
        let mut e = engine3();
        let this = Ptr::new(e.mem.alloc(0x10));
        fn_0084b6c0(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), PACKAGE_LOCATION_MAP_VTABLE);
        assert_eq!(order(&e), vec![MAP_CLEAR_ITEMS, HASH_MAP_BASE_DESTRUCT]);
        assert_eq!(calls(&e, MAP_CLEAR_ITEMS), vec![vec![this.addr()]]);
    }

    #[test]
    fn expiry_queue_base_constructor_allocates_zeroed_buckets() {
        let mut e = engine3();
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 0xc, 9);
        let result = fn_0084b750(&mut e, this.cast(), 5);
        assert_eq!(result, this.cast());
        assert_eq!(e.mem.u32(this.addr()), EXPIRY_QUEUE_BASE_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 5);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
        let buckets = e.mem.u32(this.addr() + 8);
        assert_eq!(calls(&e, ALLOCATE_BYTES), vec![vec![20]]);
        assert_eq!(calls(&e, MEMORY_SET), vec![vec![buckets, 0, 20]]);
    }

    #[test]
    fn expiry_queue_hash_and_key_comparison() {
        let mut e = engine3();
        let queue = expiry_queue(&mut e, 7);
        assert_eq!(e.call(0x0084_b930, &args![queue, 100u32, 5u32]).u32(), 2);
        assert_eq!(e.call(0x0084_b930, &args![queue, 6u32, 0xffu32]).u32(), 6);
        let same = e.call(0x0084_b950, &args![queue, 1u32, 2u32, 1u32, 2u32]);
        assert!(same.bool());
        let other_high = e.call(0x0084_b950, &args![queue, 1u32, 2u32, 1u32, 3u32]);
        assert!(!other_high.bool());
        let other_low = e.call(0x0084_b950, &args![queue, 1u32, 2u32, 4u32, 2u32]);
        assert!(!other_low.bool());
    }

    #[test]
    fn expiry_queue_set_value_fills_the_item() {
        let mut e = engine3();
        let queue = expiry_queue(&mut e, 7);
        let item = e.mem.alloc(0x18);
        e.call(0x0084_b990, &args![queue, item, 11u32, 22u32, 33u32]);
        assert_eq!(e.mem.u32(item + 8), 11);
        assert_eq!(e.mem.u32(item + 0xc), 22);
        assert_eq!(e.mem.u32(item + 0x10), 33);
    }

    #[test]
    fn expiry_queue_item_allocation_and_release() {
        let mut e = engine3();
        let queue = expiry_queue(&mut e, 7);
        let item = e.call(0x0084_ba50, &args![queue]).u32();
        assert_ne!(item, 0);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x18]]);
        e.mem.set_u32(item + 0x10, 77);
        e.call(0x0084_ba70, &args![queue, item]);
        assert_eq!(e.mem.u32(item + 0x10), 0);
        assert_eq!(
            calls(&e, MAP_ITEM_FREE),
            vec![vec![queue.addr() + 0xc, item]]
        );
        assert_ne!(e.call(0x0084_bb30, &args![queue]).u32(), 0);
    }

    #[test]
    fn expiry_queue_insert_find_and_replace() {
        let mut e = engine3();
        let queue = expiry_queue(&mut e, 7);
        let out = e.mem.alloc(4);
        // Not found in an empty queue.
        assert!(!fn_0084b8b0(&mut e, queue, 8, 1, out));
        // New items: keys 8 and 15 share bucket 1 (low word modulo 7).
        fn_0084b7c0(&mut e, queue, 8, 1, 100);
        fn_0084b7c0(&mut e, queue, 15, 1, 200);
        assert_eq!(e.mem.u32(queue.addr() + 0xc), 2);
        assert!(fn_0084b8b0(&mut e, queue, 8, 1, out));
        assert_eq!(e.mem.u32(out), 100);
        assert!(fn_0084b8b0(&mut e, queue, 15, 1, out));
        assert_eq!(e.mem.u32(out), 200);
        // The newest item is the head of its bucket.
        let buckets = e.mem.u32(queue.addr() + 8);
        let head = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(head + 8), 15);
        let second = e.mem.u32(head);
        assert_eq!(e.mem.u32(second + 8), 8);
        assert_eq!(e.mem.u32(second), 0);
        // Same low word, other high word: not equal.
        assert!(!fn_0084b8b0(&mut e, queue, 8, 2, out));
        // An equal key replaces the value without a new item.
        fn_0084b7c0(&mut e, queue, 8, 1, 300);
        assert_eq!(e.mem.u32(queue.addr() + 0xc), 2);
        assert!(fn_0084b8b0(&mut e, queue, 8, 1, out));
        assert_eq!(e.mem.u32(out), 300);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 2);
    }

    #[test]
    fn expiry_queue_destructors() {
        let mut e = engine3();
        let this = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 8, 0x1234);
        fn_0084ba20(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), EXPIRY_QUEUE_BASE_VTABLE);
        assert_eq!(order(&e), vec![MAP_CLEAR_ITEMS, FREE_BYTES]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x1234]]);
        e.call_log = Some(vec![]);
        fn_0084b9c0(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), EXPIRY_QUEUE_BASE_VTABLE);
        assert_eq!(
            order(&e),
            vec![MAP_CLEAR_ITEMS, MAP_CLEAR_ITEMS, FREE_BYTES]
        );
    }

    #[test]
    fn scalar_deleting_destructors_of_session_3() {
        let mut e = engine3();
        let this = Ptr::new(e.mem.alloc(0x40));
        e.register(FORM_BUFFER_ARRAY_DESTRUCT, |_, _| Ret::default());
        assert_eq!(fn_0084baa0(&mut e, this, 0), this);
        assert_eq!(
            calls(&e, FORM_BUFFER_ARRAY_DESTRUCT),
            vec![vec![this.addr()]]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(fn_0084baa0(&mut e, this, 1), this);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(fn_0084bad0(&mut e, this, 2), this);
        assert_eq!(calls(&e, HASH_MAP_BASE_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(fn_0084bad0(&mut e, this, 3), this);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        e.mem.set_u32(this.addr() + 8, 0x55);
        assert_eq!(fn_0084bb00(&mut e, this, 0), this);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x55]]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        assert_eq!(fn_0084bb00(&mut e, this, 1), this);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 3);
    }

    #[test]
    fn global_data_loop_reads_each_record() {
        let mut e = engine3();
        e.register(LOAD_DATA_FROM_FILE, |e, a| {
            e.mem.set_u32(a[1], 0x40 + e.mem.u32(a[1]));
            Ret::default()
        });
        fn_0084bc20(&mut e, 0x777, 0);
        assert!(order(&e).is_empty());
        fn_0084bc20(&mut e, 0x777, 2);
        let seen = order(&e);
        assert_eq!(seen.len(), 10);
        assert_eq!(
            &seen[..5],
            &[
                LOAD_GAME_BUFFER_CONSTRUCT,
                LOAD_DATA_FROM_FILE,
                LOAD_GAME_BUFFER_LOAD,
                LOAD_GLOBAL_DATA,
                LOAD_GAME_BUFFER_DESTRUCT
            ]
        );
        let buffer = calls(&e, LOAD_GAME_BUFFER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, LOAD_DATA_FROM_FILE)[0],
            vec![0x777, buffer + 0x10, 4]
        );
        assert_eq!(calls(&e, LOAD_GAME_BUFFER_LOAD)[0], vec![buffer, 0x777]);
        assert_eq!(calls(&e, LOAD_GLOBAL_DATA)[0], vec![0x40, buffer]);
        assert_eq!(calls(&e, LOAD_GAME_BUFFER_DESTRUCT)[0], vec![buffer]);
    }

    // ---- registration --------------------------------------------------------------

    #[test]
    fn every_function_is_registered_under_its_address() {
        let table = funcs();
        assert_eq!(table.len(), 96);
        let mut e = Engine::new();
        for (address, _) in table {
            assert!(e.is_translated(address), "{address:08x}");
        }
        e.map(0x011c_0000, 0x0004_5000);
        let this = e.new_object::<BGSSaveLoadGame>();
        e.set(this, BGSSaveLoadGame::cCurrentMinorVersion, 9);
        assert_eq!(e.call(0x0084_7570, &args![this]).u8(), 9);
    }
}
